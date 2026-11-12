//! Rasterizes tiles into a per-cell color buffer for the TUI canvas.
//!
//! Uses a uniform spatial grid (one bucket per screen cell) so that each
//! pixel only tests the tiles whose bounding box covers its cell.

use tess_core::geometry::{BBox, Vec2};
use tess_core::tiling::Tile;
use tess_core::view::ColorMode;

pub const BORDER: u8 = 255;
pub const GAP: u8 = 254;

pub struct Raster {
    pub w: u32,
    pub h: u32,
    /// Per-pixel fg color (256-cube index); special values: BORDER, GAP.
    /// Row 0 is the bottom row (math orientation); the renderer flips.
    pub pixels: Vec<u8>,
}

impl Raster {
    /// fg color for screen column `x`, screen row `y` (0 = top).
    pub fn at(&self, x: u32, y: u32) -> u8 {
        self.pixels[((self.h - 1 - y) * self.w + x) as usize]
    }
}

fn color_index(mode: ColorMode, tile: &Tile) -> usize {
    match mode {
        ColorMode::Single => 0,
        ColorMode::Pattern => (tile.seed as usize) % 8,
        ColorMode::Proto => (tile.proto as usize) % 8,
    }
}

pub fn raster(tiles: &[Tile], bbox: &BBox, w: u32, h: u32, mode: ColorMode, palette256: &[u8]) -> Raster {
    let n = (w * h) as usize;
    let sx = bbox.size().x / w as f64;
    let sy = bbox.size().y / h as f64;

    // Spatial grid: for each screen cell, which tiles' bboxes overlap it.
    let mut grid: Vec<Vec<u32>> = vec![Vec::new(); n];
    for (ti, tile) in tiles.iter().enumerate() {
        let bb = tile.poly.aabb();
        let x0 = ((bb.min.x - bbox.min.x) / sx).floor() as i64;
        let x1 = ((bb.max.x - bbox.min.x) / sx).ceil() as i64;
        let y0 = ((bb.min.y - bbox.min.y) / sy).floor() as i64;
        let y1 = ((bb.max.y - bbox.min.y) / sy).ceil() as i64;
        if x1 < 0 || y1 < 0 || x0 >= i64::from(w) || y0 >= i64::from(h) {
            continue;
        }
        let x0 = x0.clamp(0, i64::from(w) - 1);
        let x1 = x1.clamp(0, i64::from(w) - 1);
        let y0 = y0.clamp(0, i64::from(h) - 1);
        let y1 = y1.clamp(0, i64::from(h) - 1);
        for cy in y0..=y1 {
            for cx in x0..=x1 {
                grid[(cy * i64::from(w) + cx) as usize].push(ti as u32);
            }
        }
    }

    let mut pixels = vec![GAP; n];
    let mut owner = vec![u32::MAX; n];

    for cy in 0..h {
        for cx in 0..w {
            let p = Vec2::new(
                bbox.min.x + (cx as f64 + 0.5) * sx,
                bbox.min.y + (cy as f64 + 0.5) * sy,
            );
            let mut count = 0u32;
            let mut last = u32::MAX;
            for &ti in &grid[(cy * w + cx) as usize] {
                if tiles[ti as usize].poly.contains(p) {
                    count += 1;
                    last = ti;
                    if count > 1 {
                        break;
                    }
                }
            }
            let idx = (cy * w + cx) as usize;
            if count > 1 {
                pixels[idx] = BORDER;
            } else if count == 1 {
                owner[idx] = last;
                pixels[idx] = palette256[color_index(mode, &tiles[last as usize])];
            }
        }
    }

    // Boundary pass: a pixel becomes a border when its owner differs from
    // the left or upper (screen) neighbor.
    for cy in 0..h {
        for cx in 0..w {
            let idx = (cy * w + cx) as usize;
            if pixels[idx] == BORDER {
                continue;
            }
            let o = owner[idx];
            if o == u32::MAX {
                continue;
            }
            let mut border = false;
            if cx > 0 && owner[idx - 1] != o {
                border = true;
            }
            if cy + 1 < h && owner[idx + w as usize] != o {
                border = true;
            }
            if border {
                pixels[idx] = BORDER;
            }
        }
    }

    Raster { w, h, pixels }
}
