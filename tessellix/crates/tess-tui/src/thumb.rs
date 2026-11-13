//! Cached mini-previews ("thumbnails") of tilings for the side panel.
//!
//! Each thumbnail is a small half-block raster: one terminal cell holds
//! two vertical pixels (`▀` with fg = upper pixel, bg = lower pixel).
//!
//! The preview window is derived per tiling by
//! [`tess_core::App::preview_region`] — the registry spans pitch scales
//! from ~1 world unit (square lattice) to ~80 (measured girih panels),
//! so a fixed window would show either a single tile or a shapeless
//! mass. Color mode falls back to `preview_mode()` (proto) when the
//! canvas is in `Single`, which would otherwise render a featureless
//! slab.
//!
//! Thumbnails are pure functions of (tiling id, params, color mode,
//! size): generation is deterministic per view, so callers can cache.

use std::collections::HashMap;

use tess_core::params::{ParamSet, ParamSpec};
use tess_core::App;

use crate::raster;

/// A packed half-block thumbnail: `rows[r][x]` = (upper, lower) pixel
/// color indices for character cell (x, r). Special indices from
/// [`raster`] (BORDER/GAP) are passed through for the caller to map.
pub struct Thumb {
    pub w: u32,
    pub rows: Vec<Vec<(u8, u8)>>,
    /// World-space window the thumbnail was rendered over. Read by the
    /// tests; kept public for debugging thumbnail geometry.
    #[allow(dead_code)]
    pub window: tess_core::geometry::BBox,
}

impl Thumb {
    fn empty(w: u32, h: u32, window: tess_core::geometry::BBox) -> Self {
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

/// Generate the thumbnail for the app's selected tiling at `w_chars` ×
/// `h_chars` character cells (each cell = 2 vertical pixels).
pub fn generate(app: &App, palette256: &[u8], w_chars: u32, h_chars: u32) -> Thumb {
    let wc = w_chars.max(1);
    let hc = h_chars.max(1);
    let aspect = wc as f64 / (2.0 * hc as f64);

    let (result, window) = app.preview_result(aspect);
    if result.tiles.is_empty() {
        return Thumb::empty(wc, hc, window);
    }

    let mode = app.color_mode.preview_mode();
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

    pub fn get_or_generate(&mut self, app: &App, palette256: &[u8], w_chars: u32, h_chars: u32) -> &Thumb {
        let key = format!(
            "{}|{}|{}|{}x{}",
            app.selected_id(),
            params_key(app.tiling().params(), app.params()),
            app.color_mode.label(),
            w_chars,
            h_chars
        );
        let palette = palette256.to_vec();
        self.map
            .entry(key)
            .or_insert_with(|| generate(app, &palette, w_chars, h_chars))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tess_core::ColorMode;

    fn bbox_half(b: &tess_core::geometry::BBox) -> (f64, f64) {
        ((b.max.x - b.min.x) / 2.0, (b.max.y - b.min.y) / 2.0)
    }

    fn palette() -> Vec<u8> {
        tess_core::paint::PALETTE
            .iter()
            .map(|&c| tess_core::paint::rgb_to_256(c))
            .collect()
    }

    #[test]
    fn thumb_generation_square() {
        let mut app = App::new(tess_tilings::registry(), 8.0);
        app.select_id("square");
        let thumb = generate(&app, &palette(), 20, 8);
        assert_eq!(thumb.w, 20);
        assert_eq!(thumb.rows.len(), 8);
        // The window is derived from the lattice pitch (1.0): half 1.3,
        // stretched by the 20/16 aspect.
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
        let mut app = App::new(tess_tilings::registry(), 8.0);
        app.select_id("girih-star-cross");
        let thumb = generate(&app, &palette(), 20, 8);
        let (hx, _) = bbox_half(&thumb.window);
        assert!(hx > 40.0, "window should span multiple 77-unit cells, got {hx}");
        let filled = thumb
            .rows
            .iter()
            .flatten()
            .filter(|(a, b)| *a != raster::GAP || *b != raster::GAP)
            .count();
        assert!(filled > 0, "expected visible tiles");
    }

    #[test]
    fn thumb_structure_visible_in_single_mode() {
        // Single mode falls back to proto coloring for previews: the
        // hexagram patch must show more than one tile color class
        // (stars vs hexagons), i.e. not be a featureless slab.
        let mut app = App::new(tess_tilings::registry(), 8.0);
        app.select_id("hexagram");
        app.color_mode = ColorMode::Single;
        let thumb = generate(&app, &palette(), 24, 10);
        let colors: std::collections::BTreeSet<u8> = thumb
            .rows
            .iter()
            .flatten()
            .flat_map(|(a, b)| [*a, *b])
            .filter(|c| *c != raster::GAP && *c != raster::BORDER)
            .collect();
        assert!(
            colors.len() >= 2,
            "single-mode preview should use proto fallback, got colors {colors:?}"
        );
    }
}