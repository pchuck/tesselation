//! Cached mini-previews ("thumbnails") of tilings for the side panel.
//!
//! Each thumbnail is a small half-block raster: one terminal cell holds
//! two vertical pixels (`▀` with fg = upper pixel, bg = lower pixel).
//!
//! The registry spans pitch scales from ~1 world unit (square lattice)
//! to ~80 (measured girih panels), so the preview window is derived per
//! tiling:
//! - periodic tilings (`TilingResult.periodic`) — a window spanning
//!   ~2.6 lattice pitches of the longer basis vector, centered at the
//!   origin;
//! - aperiodic tilings — a probe pass over a small fixed window gives
//!   the mean tile extent; the preview window spans ~9 such tiles;
//! - degenerate cases fall back to a fixed 16-unit window.
//!
//! Thumbnails are pure functions of (tiling id, params, color mode,
//! size): generation is deterministic per view, so callers can cache.

use std::collections::HashMap;

use tess_core::geometry::BBox;
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::tiling::{Query, Tile, Tiling};
use tess_core::view::ColorMode;

use crate::raster;

/// Probe window for the scale-measuring pass (±10 world units).
const PROBE_HALF: f64 = 10.0;
/// Fallback preview half-extent when the probe yields nothing usable.
const FALLBACK_HALF: f64 = 8.0;
/// Preview spans this multiple of the (longer) lattice basis vector.
const PERIODIC_SPAN: f64 = 2.6;
/// Preview spans this multiple of the mean aperiodic tile extent.
const APERIODIC_SPAN: f64 = 9.0;
/// Clamp for the preview half-extent, in world units.
const HALF_EXTENT_LIMITS: (f64, f64) = (0.2, 5000.0);
/// Hard cap on probe tiles before the scale estimate gives up.
const PROBE_TILE_CAP: usize = 60_000;

/// A packed half-block thumbnail: `rows[r][x]` = (upper, lower) pixel
/// color indices for character cell (x, r). Special indices from
/// [`raster`] (BORDER/GAP) are passed through for the caller to map.
pub struct Thumb {
    pub w: u32,
    pub rows: Vec<Vec<(u8, u8)>>,
    /// World-space window the thumbnail was rendered over. Read by the
    /// tests; kept public for debugging thumbnail geometry.
    #[allow(dead_code)]
    pub window: BBox,
}

impl Thumb {
    fn empty(w: u32, h: u32, window: BBox) -> Self {
        Self {
            w,
            rows: vec![vec![(raster::GAP, raster::GAP); w as usize]; h as usize],
            window,
        }
    }
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

/// The world window a preview of `tiling` should be rendered over.
///
/// `aspect` = preview pixel width / height (terminal cells are twice as
/// tall as wide, so the pixel aspect uses 2·char-rows). `mean_tile` is
/// the mean tile bounding-box side from the probe pass (0 if unknown).
pub fn preview_window(
    periodic: Option<[tess_core::geometry::Vec2; 2]>,
    mean_tile: f64,
    aspect: f64,
) -> BBox {
    let base = match periodic {
        Some([a1, a2]) => PERIODIC_SPAN * a1.length().max(a2.length()),
        None if mean_tile > 0.0 => APERIODIC_SPAN * mean_tile,
        _ => 2.0 * FALLBACK_HALF,
    };
    let half = (base / 2.0).clamp(HALF_EXTENT_LIMITS.0, HALF_EXTENT_LIMITS.1);
    // Aspect fit: the tight dimension keeps `half`; the other stretches
    // so the thumbnail is filled edge to edge.
    let (hx, hy) = if aspect >= 1.0 {
        (half * aspect, half)
    } else {
        (half, half / aspect)
    };
    BBox::new(
        tess_core::geometry::Vec2::new(-hx, -hy),
        tess_core::geometry::Vec2::new(hx, hy),
    )
}

/// Mean tile bounding-box side over the probe tiles (0 if none usable).
fn mean_tile_side(tiles: &[Tile]) -> f64 {
    let mut total = 0.0;
    let mut n = 0usize;
    for t in tiles.iter().take(PROBE_TILE_CAP) {
        let bb = t.poly.aabb();
        let s = (bb.size().x + bb.size().y) / 2.0;
        if s.is_finite() && s > 0.0 {
            total += s;
            n += 1;
        }
    }
    if n == 0 {
        0.0
    } else {
        total / n as f64
    }
}

/// Generate the thumbnail for one tiling at `w_chars` × `h_chars`
/// character cells (each cell = 2 vertical pixels).
pub fn generate(
    tiling: &dyn Tiling,
    params: &ParamSet,
    mode: ColorMode,
    palette256: &[u8],
    w_chars: u32,
    h_chars: u32,
) -> Thumb {
    let wc = w_chars.max(1);
    let hc = h_chars.max(1);
    let aspect = wc as f64 / (2.0 * hc as f64);

    // Probe pass: scale discovery over a small fixed window.
    let probe_region = BBox::new(
        tess_core::geometry::Vec2::new(-PROBE_HALF, -PROBE_HALF),
        tess_core::geometry::Vec2::new(PROBE_HALF, PROBE_HALF),
    );
    let probe = tiling.generate(
        params,
        &Query {
            region: probe_region,
        },
    );
    let window = preview_window(probe.periodic, mean_tile_side(&probe.tiles), aspect);

    // Final pass over the derived window; shrink and retry if the
    // generator refused (too many tiles for the view).
    let mut window = window;
    let mut result = tiling.generate(params, &Query { region: window });
    let mut shrink = 0.4;
    while result.message.is_some() && shrink > 0.05 {
        let cx = (window.min.x + window.max.x) / 2.0;
        let cy = (window.min.y + window.max.y) / 2.0;
        let hx = (window.max.x - window.min.x) / 2.0 * shrink;
        let hy = (window.max.y - window.min.y) / 2.0 * shrink;
        window = BBox::new(
            tess_core::geometry::Vec2::new(cx - hx, cy - hy),
            tess_core::geometry::Vec2::new(cx + hx, cy + hy),
        );
        result = tiling.generate(params, &Query { region: window });
        shrink *= 0.4;
    }
    if result.tiles.is_empty() {
        return Thumb::empty(wc, hc, window);
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
    Thumb {
        w: wc,
        rows,
        window,
    }
}

/// The per-tiling inputs a thumbnail depends on, bundled so cache
/// lookups stay a two-argument call.
pub struct ThumbCtx<'a> {
    pub tiling: &'a dyn Tiling,
    pub specs: &'a [ParamSpec],
    pub params: &'a ParamSet,
    pub mode: ColorMode,
    pub palette256: &'a [u8],
}

/// Lazily-populated thumbnail cache keyed by
/// `(tiling id, effective params, color mode, size)`. Param edits and
/// color-mode changes therefore invalidate their own entries.
pub struct Cache {
    map: HashMap<String, Thumb>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get_or_generate(&mut self, ctx: ThumbCtx, w_chars: u32, h_chars: u32) -> &Thumb {
        let key = format!(
            "{}|{}|{}|{}x{}",
            ctx.tiling.id(),
            params_key(ctx.specs, ctx.params),
            ctx.mode.label(),
            w_chars,
            h_chars
        );
        let palette = ctx.palette256.to_vec();
        self.map.entry(key).or_insert_with(|| {
            generate(ctx.tiling, ctx.params, ctx.mode, &palette, w_chars, h_chars)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tess_core::geometry::Vec2;

    fn bbox_half(b: &BBox) -> (f64, f64) {
        ((b.max.x - b.min.x) / 2.0, (b.max.y - b.min.y) / 2.0)
    }

    #[test]
    fn periodic_window_spans_lattice() {
        // Square-ish basis with pitches 3 and 4 → base = 2.6·4 = 10.4.
        let b = preview_window(Some([Vec2::new(3.0, 0.0), Vec2::new(0.0, 4.0)]), 0.0, 1.0);
        let (hx, hy) = bbox_half(&b);
        assert!((hx - 5.2).abs() < 1e-9 && (hy - 5.2).abs() < 1e-9);
        // Wide aspect stretches x only.
        let b = preview_window(Some([Vec2::new(3.0, 0.0), Vec2::new(0.0, 4.0)]), 0.0, 2.0);
        let (hx, hy) = bbox_half(&b);
        assert!((hx - 10.4).abs() < 1e-9 && (hy - 5.2).abs() < 1e-9);
    }

    #[test]
    fn aperiodic_window_spans_mean_tile() {
        let b = preview_window(None, 0.5, 1.0);
        let (hx, hy) = bbox_half(&b);
        assert!((hx - 2.25).abs() < 1e-9 && (hy - 2.25).abs() < 1e-9);
    }

    #[test]
    fn fallback_window_when_no_signal() {
        let b = preview_window(None, 0.0, 1.0);
        let (hx, _) = bbox_half(&b);
        assert!((hx - FALLBACK_HALF).abs() < 1e-9);
    }

    #[test]
    fn tall_aspect_stretches_y() {
        let b = preview_window(None, 1.0, 0.5);
        let (hx, hy) = bbox_half(&b);
        assert!((hx - 4.5).abs() < 1e-9 && (hy - 9.0).abs() < 1e-9);
    }

    #[test]
    fn thumb_generation_square() {
        let registry = tess_tilings::registry();
        let t = registry.get("square").expect("square registered");
        let specs = t.params();
        let params = ParamSet::default_for(specs);
        let palette: Vec<u8> = tess_core::paint::PALETTE
            .iter()
            .map(|&c| tess_core::paint::rgb_to_256(c))
            .collect();
        let thumb = generate(t, &params, ColorMode::Pattern, &palette, 20, 8);
        assert_eq!(thumb.w, 20);
        assert_eq!(thumb.rows.len(), 8);
        // The window is derived from the lattice pitch (1.0): half 1.3,
        // stretched x2 by the 20/16 aspect.
        let (hx, hy) = bbox_half(&thumb.window);
        assert!((hy - 1.3).abs() < 1e-9, "hy = {hy}");
        assert!(hx > hy);
        // Content: most cells are filled tiles, not gaps.
        let filled = thumb
            .rows
            .iter()
            .flatten()
            .filter(|(a, b)| *a != raster::GAP || *b != raster::GAP)
            .count();
        assert!(filled > 100, "expected a dense patch, got {filled}");
    }

    #[test]
    fn thumb_generation_large_pitch() {
        // The measured girih panel lives on a ~(77,45)/(0,90) lattice —
        // the derived window must scale far beyond the square's.
        let registry = tess_tilings::registry();
        let t = registry
            .get("girih-star-cross")
            .expect("girih-sc registered");
        let specs = t.params();
        let params = ParamSet::default_for(specs);
        let palette: Vec<u8> = tess_core::paint::PALETTE
            .iter()
            .map(|&c| tess_core::paint::rgb_to_256(c))
            .collect();
        let thumb = generate(t, &params, ColorMode::Proto, &palette, 20, 8);
        let (hx, _) = bbox_half(&thumb.window);
        assert!(
            hx > 40.0,
            "window should span multiple 77-unit cells, got {hx}"
        );
        let filled = thumb
            .rows
            .iter()
            .flatten()
            .filter(|(a, b)| *a != raster::GAP || *b != raster::GAP)
            .count();
        assert!(filled > 0, "expected visible tiles");
    }
}
