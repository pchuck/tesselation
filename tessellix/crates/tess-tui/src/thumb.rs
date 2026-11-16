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
//! Card previews are pure functions of (tiling, params, size): the
//! color mode is the tiling's own default preview mode (proto fallback
//! for `Single`, which would otherwise render a featureless slab), so
//! grid previews stay stable while the render view cycles color modes.
//! Callers can cache via [`CardCache`].

use std::collections::HashMap;

use tess_core::geometry::{BBox, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::SymmetryMeta;
use tess_core::tiling::{Query, Tiling};

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
/// character cells (each cell = 2 vertical pixels).
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
) -> CardPreview {
    let wc = w_chars.max(1);
    let hc = h_chars.max(1);
    let aspect = wc as f64 / (2.0 * hc as f64);

    let mut window = derive_window(tiling, params, aspect);

    // Pass 2: the preview itself, with shrink-and-retry on refusal.
    let mut result = tiling.generate(params, &Query { region: window });
    let mut shrink = 0.4;
    while result.message.is_some() && shrink > 0.05 {
        let cx = (window.min.x + window.max.x) / 2.0;
        let cy = (window.min.y + window.max.y) / 2.0;
        let hx = (window.max.x - window.min.x) / 2.0 * shrink;
        let hy = (window.max.y - window.min.y) / 2.0 * shrink;
        window = BBox::new(Vec2::new(cx - hx, cy - hy), Vec2::new(cx + hx, cy + hy));
        result = tiling.generate(params, &Query { region: window });
        shrink *= 0.4;
    }

    if result.tiles.is_empty() {
        return CardPreview {
            thumb: Thumb::empty(wc, hc),
            symmetry: result.symmetry,
            tiles: 0,
        };
    }

    let mode = tiling.default_color_mode().preview_mode();
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
/// `(tiling id, effective params, size)`. The color mode is fixed to the
/// tiling's default preview mode (see [`generate_for`]), so param edits
/// and resizes are the only invalidations.
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
    ) -> &CardPreview {
        let key = format!(
            "{}|{}|{}x{}",
            tiling.id(),
            params_key(tiling.params(), params),
            w_chars,
            h_chars
        );
        let palette = palette256.to_vec();
        self.map
            .entry(key)
            .or_insert_with(|| generate_for(tiling, params, &palette, w_chars, h_chars))
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
        generate_for(t, &params, &palette(), w, h)
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
        let p = generate_for(t, &params, &palette(), 24, 10);
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
        let a = cache.get_or_generate(t, &params, &pal, 10, 4);
        let pa = a as *const CardPreview;
        let b = cache.get_or_generate(t, &params, &pal, 10, 4);
        assert!(
            std::ptr::eq(pa, b as *const CardPreview),
            "identical keys must hit the cache"
        );
        assert_eq!(b.thumb.w, 10);
        assert_eq!(b.thumb.rows.len(), 4);
    }
}
