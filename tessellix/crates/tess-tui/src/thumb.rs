//! Cached card previews for the tiling browser.
//!
//! Each preview is a small half-block raster: one terminal cell holds
//! two vertical pixels (`▀` with fg = upper pixel, bg = lower pixel).
//!
//! The preview window is derived per tiling by a two-pass scheme that
//! mirrors [`tess_core::App::preview_region`] / `preview_result`: a
//! bounded ±10-world-unit probe captures the lattice basis (or the mean
//! tile extent for aperiodic patches), [`tess_core::app::preview_window`]
//! turns it into a world window for the card's aspect ratio, and the
//! window is regenerated with the same shrink-and-retry on generation
//! refusal as the main view. The registry spans pitch scales from ~1
//! world unit (square lattice) to ~80 (measured girih panels), so a
//! fixed window would show either a single tile or a shapeless mass.
//!
//! Card previews are pure functions of (tiling, params, effective
//! preview mode, size): the preview mode follows the caller's browse-
//! grid override — `None` means the tiling's own default preview mode
//! (proto fallback for `Single`, which would otherwise render a
//! featureless slab), `Some(m)` means `m` explicitly, with the same
//! `Single`→`Proto` fallback applied at the call site. Callers can
//! cache via [`CardCache`]; the key embeds the resolved preview mode,
//! so a mode change regenerates the thumbnail while a no-op does not.

use std::collections::HashMap;

use tess_core::geometry::{BBox, Vec2};
use tess_core::params::{ParamSet, ParamSpec, ParamValue};
use tess_core::symmetry::SymmetryMeta;
use tess_core::tiling::{Query, Tiling};
use tess_core::view::ColorMode;

use crate::raster;

/// A packed half-block thumbnail: `rows[r][x]` = (upper, lower) pixel
/// color indices for character cell (x, r). Special indices from
/// [`raster`] (BORDER/GAP) are passed through for the caller to map.
pub struct Thumb {
    pub w: u32,
    pub rows: Vec<Vec<(u8, u8)>>,
}

impl Thumb {
    fn empty(w: u32, h: u32) -> Self {
        Self {
            w,
            rows: vec![vec![(raster::GAP, raster::GAP); w as usize]; h as usize],
        }
    }
}

/// `glb-binary-n7` id — the only tiling whose card preview changes.
const GLB_BINARY_N7_ID: &str = "glb-binary-n7";
/// Preview-only depth pin for `glb-binary-n7` (see `preview_params`).
const GLB_BINARY_N7_PREVIEW_DEPTH: i64 = 3;

/// The params the card preview actually renders with: the tiling's
/// passed params, except for `glb-binary-n7`, whose preview is pinned
/// to depth 3 via the tiling's own `depth` param (the same cap the
/// e01fc76 scale override enforced, closed here by the param). Window
/// derivation (the probe) always runs on the original params.
fn preview_params(params: &ParamSet, tiling_id: &str) -> ParamSet {
    if tiling_id != GLB_BINARY_N7_ID {
        return params.clone();
    }
    let mut p = params.clone();
    p.set(
        tess_tilings::families::quasiperiodic::glb_binary_n7::DEPTH_ID,
        ParamValue::Int(GLB_BINARY_N7_PREVIEW_DEPTH),
    );
    p
}

/// A card preview: the half-block raster plus the symmetry facts
/// (group, lattice, notes) and tile count captured from the generation.
///
/// The TUI card text itself comes from `tess_core::browse::CardMeta`
/// (declared metadata, no generation required), and the render view's
/// status bar reads the live `App` result — so `symmetry` and `tiles`
/// are not surfaced by the TUI yet; they are kept on the preview for
/// card enrichment (unit tests read them today).
pub struct CardPreview {
    pub thumb: Thumb,
    #[allow(dead_code)]
    pub symmetry: SymmetryMeta,
    #[allow(dead_code)]
    pub tiles: usize,
}

/// Cache key component: the effective parameter values of a tiling.
pub fn params_key(specs: &[ParamSpec], params: &ParamSet) -> String {
    let mut parts: Vec<String> = Vec::new();
    for spec in specs {
        let v = params.get(spec.id).unwrap_or_else(|| spec.default_value());
        parts.push(format!("{}={}", spec.id.as_str(), v.display()));
    }
    parts.join(",")
}

/// Derive the preview world-window for `tiling` at `params` for a card
/// of `aspect` (width / height) ratio: a bounded ±10-world-unit probe
/// captures the lattice basis (or the mean tile extent for aperiodic
/// patches), then [`tess_core::app::preview_window`] fits a window to
/// the aspect ratio.
fn derive_window(tiling: &dyn Tiling, params: &ParamSet, aspect: f64) -> BBox {
    let probe = tiling.generate(params, &Query {
        region: BBox::new(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0)),
    });
    tess_core::app::preview_window(
        probe.periodic,
        tess_core::app::mean_tile_side(&probe.tiles),
        aspect,
    )
}

/// Generate the card preview for `tiling` at `params` (typically
/// `ParamSet::default_for(tiling.params())`) at `w_chars` × `h_chars`
/// character cells (each cell = 2 vertical pixels), rasterized in the
/// resolved `mode` (the caller's effective preview mode: the browse-
/// grid override or the tiling's own default, post `Single`→`Proto`
/// fallback — the same contract the GUI's `card_texture` follows).
///
/// Window derivation is two-pass (see the module docs): [`derive_window`]
/// → generate → shrink-and-retry on generation refusal. Symmetry is read
/// from the final generation (declared by the generator,
/// region-independent).
pub fn generate_for(
    tiling: &dyn Tiling,
    params: &ParamSet,
    palette256: &[u8],
    w_chars: u32,
    h_chars: u32,
    mode: ColorMode,
) -> CardPreview {
    let wc = w_chars.max(1);
    let hc = h_chars.max(1);
    let aspect = wc as f64 / (2.0 * hc as f64);
    let effective = preview_params(params, tiling.id());

    // Window derivation: the probe runs on the original params.
    let mut window = derive_window(tiling, params, aspect);

    // Pass 2: the preview itself, with shrink-and-retry on refusal.
    let mut result = tiling.generate(&effective, &Query { region: window });
    let mut shrink = 0.4;
    while result.message.is_some() && shrink > 0.05 {
        let cx = (window.min.x + window.max.x) / 2.0;
        let cy = (window.min.y + window.max.y) / 2.0;
        let hx = (window.max.x - window.min.x) / 2.0 * shrink;
        let hy = (window.max.y - window.min.y) / 2.0 * shrink;
        window = BBox::new(Vec2::new(cx - hx, cy - hy), Vec2::new(cx + hx, cy + hy));
        result = tiling.generate(&effective, &Query { region: window });
        shrink *= 0.4;
    }

    if result.tiles.is_empty() {
        return CardPreview {
            thumb: Thumb::empty(wc, hc),
            symmetry: result.symmetry,
            tiles: 0,
        };
    }

    let r = raster::raster(&result.tiles, &window, wc, 2 * hc, mode, palette256);
    let mut rows = Vec::with_capacity(hc as usize);
    for cr in 0..hc {
        let mut row = Vec::with_capacity(wc as usize);
        for x in 0..wc {
            row.push((r.at(x, 2 * cr), r.at(x, 2 * cr + 1)));
        }
        rows.push(row);
    }
    CardPreview {
        thumb: Thumb { w: wc, rows },
        symmetry: result.symmetry,
        tiles: result.tiles.len(),
    }
}

/// Lazily-populated card-preview cache keyed by
/// `(tiling id, effective params, resolved preview mode, size)`. The
/// preview mode is part of the key — the M215 card_key precedent — so
/// a real mode change regenerates the thumbnail while a no-op leaves
/// the cached entry untouched.
pub struct CardCache {
    map: HashMap<String, CardPreview>,
}

impl CardCache {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get_or_generate(
        &mut self,
        tiling: &dyn Tiling,
        params: &ParamSet,
        palette256: &[u8],
        w_chars: u32,
        h_chars: u32,
        mode: ColorMode,
    ) -> &CardPreview {
        let effective = preview_params(params, tiling.id());
        let key = format!(
            "{}|{}|{}|{}x{}",
            tiling.id(),
            params_key(tiling.params(), &effective),
            mode.label(),
            w_chars,
            h_chars
        );
        let palette = palette256.to_vec();
        self.map.entry(key).or_insert_with(|| {
            generate_for(tiling, params, &palette, w_chars, h_chars, mode)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tess_core::ColorMode;
    use tess_tilings::registry;

    fn bbox_half(b: &BBox) -> (f64, f64) {
        ((b.max.x - b.min.x) / 2.0, (b.max.y - b.min.y) / 2.0)
    }

    fn palette() -> Vec<u8> {
        tess_core::paint::PALETTE
            .iter()
            .map(|&c| tess_core::paint::rgb_to_256(c))
            .collect()
    }

    /// The derived preview window for a registry tiling at its default
    /// params (the registry owns the tilings for the duration of the
    /// call, so the window is the only thing we can hand back out).
    fn window_for(id: &str, aspect: f64) -> BBox {
        let reg = registry();
        let t = reg
            .get(id)
            .unwrap_or_else(|| panic!("unknown tiling {id}"));
        let params = ParamSet::default_for(t.params());
        derive_window(t, &params, aspect)
    }

    fn preview_for(id: &str, w: u32, h: u32) -> CardPreview {
        let reg = registry();
        let t = reg
            .get(id)
            .unwrap_or_else(|| panic!("unknown tiling {id}"));
        let params = ParamSet::default_for(t.params());
        generate_for(t, &params, &palette(), w, h, t.default_color_mode().preview_mode())
    }

    #[test]
    fn card_preview_square() {
        let p = preview_for("square", 20, 8);
        assert_eq!(p.thumb.w, 20);
        assert_eq!(p.thumb.rows.len(), 8);
        // The window is derived from the lattice pitch (1.0): half 1.3,
        // stretched by the 20/16 aspect.
        let (hx, hy) = bbox_half(&window_for("square", 20.0 / 16.0));
        assert!((hy - 1.3).abs() < 1e-9, "hy = {hy}");
        assert!(hx > hy);
        // Content: most cells are filled tiles, not gaps.
        let filled = p
            .thumb
            .rows
            .iter()
            .flatten()
            .filter(|(a, b)| *a != raster::GAP || *b != raster::GAP)
            .count();
        assert!(filled > 100, "expected a dense patch, got {filled}");
        // Declared symmetry captured from the generation.
        assert_eq!(p.symmetry.lattice, tess_core::LatticeKind::Square);
        assert_eq!(p.symmetry.group, tess_core::WallpaperGroup::P4m);
        assert!(p.tiles > 0);
    }

    #[test]
    fn card_preview_large_pitch() {
        // The measured girih panel lives on a ~(77,45)/(0,90) lattice —
        // the derived window must scale far beyond the square's.
        let p = preview_for("girih-star-cross", 20, 8);
        let (hx, _) = bbox_half(&window_for("girih-star-cross", 20.0 / 16.0));
        assert!(
            hx > 40.0,
            "window should span multiple 77-unit cells, got {hx}"
        );
        let filled = p
            .thumb
            .rows
            .iter()
            .flatten()
            .filter(|(a, b)| *a != raster::GAP || *b != raster::GAP)
            .count();
        assert!(filled > 0, "expected visible tiles");
    }

    #[test]
    fn card_preview_proto_default_shows_structure() {
        // No registry tiling defaults to `Single` (tess-core already
        // unit-tests the Single→Proto `preview_mode` fallback), so the
        // structure check runs on a `Proto`-default tiling: the girih
        // star-and-cross patch must show more than one prototile color
        // class (stars vs crosses), i.e. not be a featureless slab.
        let reg = registry();
        let t = reg.get("girih-quasiperiodic").unwrap();
        assert_eq!(t.default_color_mode(), ColorMode::Proto);
        let params = ParamSet::default_for(t.params());
        let p = generate_for(t, &params, &palette(), 24, 10, t.default_color_mode().preview_mode());
        let colors: std::collections::BTreeSet<u8> = p
            .thumb
            .rows
            .iter()
            .flatten()
            .flat_map(|(a, b)| [*a, *b])
            .filter(|c| *c != raster::GAP && *c != raster::BORDER)
            .collect();
        assert!(
            colors.len() >= 2,
            "single-default preview should use proto fallback, got colors {colors:?}"
        );
    }

    #[test]
    fn card_preview_aperiodic_bounded() {
        // An aperiodic tiling (Penrose) must preview through the mean-
        // tile extent path without blowing the window limits.
        let p = preview_for("penrose", 20, 8);
        let (hx, hy) = bbox_half(&window_for("penrose", 20.0 / 16.0));
        assert!(hx > 0.0 && hx < 5000.0, "window {hx}");
        assert!(hy > 0.0 && hy < 5000.0, "window {hy}");
        assert!(p.tiles > 0);
        assert_eq!(p.symmetry.lattice, tess_core::LatticeKind::None);
    }

    #[test]
    fn card_cache_hits() {
        let reg = registry();
        let t = reg.get("square").unwrap();
        let params = ParamSet::default_for(t.params());
        let pal = palette();
        let mut cache = CardCache::new();
        let a = cache.get_or_generate(t, &params, &pal, 10, 4, t.default_color_mode().preview_mode());
        let pa = a as *const CardPreview;
        let b = cache.get_or_generate(t, &params, &pal, 10, 4, t.default_color_mode().preview_mode());
        assert!(
            std::ptr::eq(pa, b as *const CardPreview),
            "identical keys must hit the cache"
        );
        assert_eq!(b.thumb.w, 10);
        assert_eq!(b.thumb.rows.len(), 4);
    }
}

#[cfg(test)]
mod tests_card_cache_pins {
    //! CardCache key-semantics and render-identity pins (post-M184
    //! depth-param semantics). These pin the CONTRACT of the cache —
    //! key stability/sensitivity on effective params, render identity
    //! across fills and cache hits, and shrink-retry convergence — not
    //! any per-tiling counts (the registry grows over time).
    use super::*;
    use tess_core::tiling::TilingResult;
    use tess_core::{LatticeKind, TilingCategory, WallpaperGroup};
    use tess_tilings::registry;

    fn palette() -> Vec<u8> {
        tess_core::paint::PALETTE
            .iter()
            .map(|&c| tess_core::paint::rgb_to_256(c))
            .collect()
    }

    fn preview_for(id: &str, w: u32, h: u32) -> CardPreview {
        let reg = registry();
        let t = reg
            .get(id)
            .unwrap_or_else(|| panic!("unknown tiling {id}"));
        let params = ParamSet::default_for(t.params());
        generate_for(t, &params, &palette(), w, h, t.default_color_mode().preview_mode())
    }

    fn doc_text(path: &str) -> String {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        std::fs::read_to_string(root.join(path)).expect("doc readable")
    }

    /// Doc claims about card thumbnails: each phrase must appear exactly
    /// once in the named doc (unique anchor, zero-or-one match).
    fn assert_doc_anchor(path: &str, phrase: &str) {
        let doc = doc_text(path);
        let matches: Vec<&str> = doc.lines().filter(|l| l.contains(phrase)).collect();
        assert_eq!(
            matches.len(),
            1,
            "doc claim must be unique in {path}: {phrase} (found {}): {matches:?}",
            matches.len()
        );
    }

    /// Doc-side pin: the (id, params, size) cache contract claimed in the
    /// Browse-mode docs — the key IS id + effective params + size.
    #[test]
    fn doc_claim_browse_grid_half_block_raster() {
        assert_doc_anchor("docs/USAGE.md", "half-block raster thumbnail");
    }

    #[test]
    fn doc_claim_cache_keyed_by_id_params_size() {
        assert_doc_anchor("docs/USAGE.md", "cached per (id, params, size)");
    }

    #[test]
    fn doc_claim_browse_thumbnails_keep_color_previews() {
        assert_doc_anchor("docs/DESIGN.md", "browse thumbnails keep their");
    }

    /// Same (tiling, params, size) → identical cache key: a repeated
    /// `get_or_generate` must hit (pointer-identical), i.e. the key is
    /// stable across calls.
    #[test]
    fn card_cache_key_stable() {
        let reg = registry();
        let t = reg.get("star-cross").unwrap();
        let params = ParamSet::default_for(t.params());
        let pal = palette();
        let mut cache = CardCache::new();
        let a = cache.get_or_generate(t, &params, &pal, 10, 4, t.default_color_mode().preview_mode());
        let pa = a as *const CardPreview;
        let b = cache.get_or_generate(t, &params, &pal, 10, 4, t.default_color_mode().preview_mode());
        assert!(
            std::ptr::eq(pa, b as *const CardPreview),
            "identical (tiling, params, size) must hit the cache (stable key)"
        );
        assert_eq!(
            params_key(t.params(), &params),
            params_key(t.params(), &params),
            "params_key must be a pure function of (specs, params)"
        );
    }

    /// Key sensitivity: any param change moves the key. At the raw
    /// params_key level (the key's params component) and at the cache
    /// level (a miss, no pointer reuse).
    #[test]
    fn card_cache_key_sensitive_to_param_change() {
        let reg = registry();
        let t = reg.get("star-cross").unwrap();
        let specs = t.params();
        // star-cross declares (scale, rotation, star_radius); the first
        // spec is scale (declared in the GUI-side pin).
        let scale_id = specs[0].id;
        assert_eq!(specs[0].id.as_str(), "scale");
        let mut p_lo = ParamSet::default_for(specs);
        p_lo.set(scale_id, ParamValue::Float(1.0));
        let mut p_hi = ParamSet::default_for(specs);
        p_hi.set(scale_id, ParamValue::Float(2.0));
        assert_ne!(
            params_key(specs, &p_lo),
            params_key(specs, &p_hi),
            "params_key must reflect the param values it is handed"
        );
        let pal = palette();
        let mut cache = CardCache::new();
        let a = cache.get_or_generate(t, &p_lo, &pal, 10, 4, t.default_color_mode().preview_mode()) as *const CardPreview;
        let b = cache.get_or_generate(t, &p_hi, &pal, 10, 4, t.default_color_mode().preview_mode()) as *const CardPreview;
        assert!(
            !std::ptr::eq(a, b),
            "a param change must move the cache key (miss, no reuse)"
        );
    }

    /// Key sensitivity on the effective preview mode (the M215
    /// card_key precedent): the same (tiling, params, size) under
    /// different modes must yield different cache entries (a real
    /// change regenerates), and a repeat hit stays pointer-identical.
    #[test]
    fn card_cache_key_sensitive_to_mode() {
        let reg = registry();
        let t = reg.get("square").unwrap();
        let params = ParamSet::default_for(t.params());
        let pal = palette();
        let mut cache = CardCache::new();
        let proto = cache.get_or_generate(t, &params, &pal, 10, 4, ColorMode::Proto) as *const CardPreview;
        let gray = cache.get_or_generate(t, &params, &pal, 10, 4, ColorMode::Gray) as *const CardPreview;
        assert!(
            !std::ptr::eq(proto, gray),
            "different effective preview modes must yield different cache entries"
        );
        let proto_again = cache.get_or_generate(t, &params, &pal, 10, 4, ColorMode::Proto) as *const CardPreview;
        assert!(
            std::ptr::eq(proto, proto_again),
            "a repeat hit for the same mode must return the same entry"
        );
    }

    /// Key sensitivity, glb-binary-n7 depth specifically: the raw key
    /// construction must reflect the depth value handed in — depth 0 vs
    /// 3 yield different keys.
    #[test]
    fn card_cache_key_sensitive_to_glb_depth() {
        let reg = registry();
        let t = reg.get("glb-binary-n7").unwrap();
        let specs = t.params();
        let depth_id = tess_tilings::families::quasiperiodic::glb_binary_n7::DEPTH_ID;
        assert!(
            specs.iter().any(|s| s.id == depth_id),
            "glb-binary-n7 must declare a `depth` param"
        );
        let mut p0 = ParamSet::default_for(specs);
        p0.set(depth_id, ParamValue::Int(0));
        let mut p3 = ParamSet::default_for(specs);
        p3.set(depth_id, ParamValue::Int(3));
        assert_ne!(
            params_key(specs, &p0),
            params_key(specs, &p3),
            "depth 0 vs 3 must yield different keys (key reflects the param value)"
        );
    }

    /// The cache key is computed from the EFFECTIVE (post-depth-cap)
    /// params, not the raw request: raw depth 0 and raw depth 4 both
    /// preview at depth 3 (the M184 preview pin), so they share one
    /// cache entry — no re-generation, no extra raster.
    #[test]
    fn card_cache_key_reflects_effective_params_not_raw() {
        let reg = registry();
        let t = reg.get("glb-binary-n7").unwrap();
        let specs = t.params();
        let depth_id = tess_tilings::families::quasiperiodic::glb_binary_n7::DEPTH_ID;
        let mut p0 = ParamSet::default_for(specs);
        p0.set(depth_id, ParamValue::Int(0));
        let mut p4 = ParamSet::default_for(specs);
        p4.set(depth_id, ParamValue::Int(4));
        let pal = palette();
        let mut cache = CardCache::new();
        let a = cache.get_or_generate(t, &p0, &pal, 10, 4, t.default_color_mode().preview_mode()) as *const CardPreview;
        let b = cache.get_or_generate(t, &p4, &pal, 10, 4, t.default_color_mode().preview_mode()) as *const CardPreview;
        assert!(
            std::ptr::eq(a, b),
            "raw depth 0 vs raw depth 4 → same effective params (depth 3) → same cache entry"
        );
    }

    /// Render identity: two independent fills for the same key produce
    /// byte-identical rasters (deterministic generation).
    #[test]
    fn card_cache_fills_are_byte_identical() {
        let reg = registry();
        let t = reg.get("square").unwrap();
        let params = ParamSet::default_for(t.params());
        let pal = palette();
        let mut cache_a = CardCache::new();
        let mut cache_b = CardCache::new();
        let a = cache_a.get_or_generate(t, &params, &pal, 12, 5, t.default_color_mode().preview_mode());
        let b = cache_b.get_or_generate(t, &params, &pal, 12, 5, t.default_color_mode().preview_mode());
        assert_eq!(a.thumb.w, b.thumb.w);
        assert_eq!(a.thumb.rows, b.thumb.rows);
        assert_eq!(a.tiles, b.tiles);
    }

    /// A cache hit returns the SAME bytes as the initial fill — no
    /// regeneration drift (pointer AND byte identity).
    #[test]
    fn card_cache_hit_returns_same_bytes_as_fill() {
        let reg = registry();
        let t = reg.get("square").unwrap();
        let params = ParamSet::default_for(t.params());
        let pal = palette();
        let mut cache = CardCache::new();
        let a = cache.get_or_generate(t, &params, &pal, 12, 5, t.default_color_mode().preview_mode());
        let pa = a as *const CardPreview;
        let rows = a.thumb.rows.clone();
        let tiles = a.tiles;
        let b = cache.get_or_generate(t, &params, &pal, 12, 5, t.default_color_mode().preview_mode());
        assert!(
            std::ptr::eq(pa, b as *const CardPreview),
            "hit must be pointer-identical to the fill"
        );
        assert_eq!(b.thumb.rows, rows, "hit must be byte-identical to the fill");
        assert_eq!(b.tiles, tiles);
    }

    /// The depth-3 card converges: glb-binary-n7 at default params
    /// previews at depth 3 (the M184 preview pin) and the render comes
    /// back non-empty — the shrink-retry loop never dead-ends.
    #[test]
    fn glb_depth3_card_converges() {
        let p = preview_for("glb-binary-n7", 20, 8);
        assert!(
            p.tiles > 0,
            "depth-3 card must converge to a non-empty raster"
        );
        assert_eq!(p.thumb.w, 20);
        assert_eq!(p.thumb.rows.len(), 8);
        let filled = p
            .thumb
            .rows
            .iter()
            .flatten()
            .filter(|(a, b)| *a != raster::GAP || *b != raster::GAP)
            .count();
        assert!(filled > 0, "depth-3 card must render content");
    }

    /// Shrink-retry convergence on refusal: a tiling that always refuses
    /// must terminate the shrink-retry loop and produce an empty
    /// (gap) raster — never a panic, never a hang.
    #[test]
    fn refusing_tiling_converges_to_empty_preview() {
        struct RefusingTiling;
        impl Tiling for RefusingTiling {
            fn id(&self) -> &'static str {
                "stub-refusing"
            }
            fn name(&self) -> &'static str {
                "Refusing stub"
            }
            fn description(&self) -> &'static str {
                "always refuses"
            }
            fn category(&self) -> TilingCategory {
                TilingCategory::Uniform
            }
            fn params(&self) -> &'static [ParamSpec] {
                &[]
            }
            fn generate(&self, _params: &ParamSet, _query: &Query) -> TilingResult {
                TilingResult {
                    tiles: Vec::new(),
                    symmetry: SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None),
                    periodic: None,
                    message: Some("stub refuses"),
                }
            }
        }
        let stub = RefusingTiling;
        let p = generate_for(&stub, &ParamSet::EMPTY, &palette(), 8, 4, stub.default_color_mode().preview_mode());
        assert_eq!(p.tiles, 0, "refusing tiling must yield zero tiles");
        assert_eq!(p.thumb.w, 8);
        assert_eq!(p.thumb.rows.len(), 4);
        for row in &p.thumb.rows {
            assert!(
                row.iter()
                    .all(|(a, b)| *a == raster::GAP && *b == raster::GAP),
                "refusing tiling must yield a gap raster"
            );
        }
    }
}
