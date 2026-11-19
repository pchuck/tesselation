//! Rasterizes tiles into a per-pixel color-index buffer.
//!
//! Shared by the terminal UI (the TUI canvas and card thumbnails map
//! the indices to xterm-256 colors) and the desktop GUI (card preview
//! textures map them into [`crate::paint::PALETTE`]). Uses a uniform
//! spatial grid (one bucket per screen cell) so that each pixel only
//! tests the tiles whose bounding box covers its cell.
//!
//! `palette` is a color-index map: the output pixel value is
//! `palette[color_index(mode, tile)]`, so each front end passes the
//! palette of its choice — the TUI passes the 256-cube indices of the
//! shared 8-color palette, the GUI passes the identity map
//! `[0, 1, ..., 7]` so that the indices are positions in
//! [`crate::paint::PALETTE`] (full 8-bit colors, no 256-cube loss).
//!
//! # Coverage model
//!
//! Each screen cell shows one color. Early versions sampled only the
//! cell centre and marked every cell whose neighbour differed as
//! [`BORDER`], which erased any feature thinner than about two cells:
//! sub-cell decoration (e.g. the taylor-socolar chord network)
//! collapsed into dark channels and its tips vanished under the
//! neighboring fills. The rasterizer now supersamples contested cells
//! on a fixed 4×4 grid and applies a thin-feature rule:
//!
//! - A piece that covers a visible part of the cell (≥ [`FG_MIN`] of
//!   the sub-samples) and is dramatically thinner than every other
//!   such piece (thinness ratio, see [`thinness`]) claims the cell
//!   outright — thin features render in the foreground even when a
//!   large fill dominates the cell — but only while the feature is
//!   sub-cell-scale (at fine zoom ordinary boundary rendering takes
//!   over again). Claimed cells are exempt from the neighbour boundary
//!   pass, so the feature stays visible.
//! - A cell genuinely straddling two comparable pieces (runner-up
//!   coverage ≥ 25%) still renders as [`BORDER`] grout.
//! - Cells no piece covers stay [`GAP`].

use crate::geometry::{BBox, Vec2};
use crate::tiling::Tile;
use crate::view::ColorMode;

pub const BORDER: u8 = 255;
pub const GAP: u8 = 254;

/// Supersample grid per contested cell (K×K sub-samples).
const K: u32 = 4;
const KS: u32 = K * K;

/// A piece claims a cell outright when its thinness is below this
/// factor times the next-thinnest piece covering the cell.
const FG_RATIO: f64 = 0.8;

/// ...and covers at least this many of the K² sub-samples (~19%), so
/// a piece must be a visible presence in the cell, not a corner clip.
const FG_MIN: u32 = 3;

/// ...and only while its thinness is below this factor times the
/// larger cell dimension (the foreground rule rescues sub-cell-scale
/// features; at fine zoom normal boundary rendering applies).
const FG_CELL: f64 = 1.5;

/// A cell straddling two comparable pieces renders as grout when the
/// runner-up covers at least this fraction of the sub-samples (below
/// it the cell shows the majority piece; 1-cell grout lines would
/// otherwise dominate views whose cells are comparable to the tile
/// features).
const BORDER_MIN: u32 = KS * 3 / 8;

pub struct Raster {
    pub w: u32,
    pub h: u32,
    /// Per-pixel color index; special values: BORDER, GAP.
    /// Row 0 is the bottom row (math orientation); `at` flips.
    pub pixels: Vec<u8>,
}

impl Raster {
    /// Color for screen column `x`, screen row `y` (0 = top).
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

/// Shoelace area (the point list need not be closed).
fn area(pts: &[Vec2]) -> f64 {
    let n = pts.len();
    let mut a = 0.0;
    for i in 0..n {
        let p = pts[i];
        let q = pts[(i + 1) % n];
        a += p.x * q.y - q.x * p.y;
    }
    (a / 2.0).abs()
}

/// Thinness measure: polygon area divided by its AABB diagonal. For a
/// strip this is (approximately) its width; for compact pieces it is
/// proportionally larger, so ranking pieces by this value orders them
/// from thin decoration to large field.
fn thinness(tile: &Tile) -> f64 {
    let bb = tile.poly.aabb();
    let dx = bb.max.x - bb.min.x;
    let dy = bb.max.y - bb.min.y;
    let diag = (dx * dx + dy * dy).sqrt().max(1e-12);
    area(&tile.poly.pts) / diag
}

pub fn raster(tiles: &[Tile], bbox: &BBox, w: u32, h: u32, mode: ColorMode, palette: &[u8]) -> Raster {
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

    // Thinness of every tile, precomputed once.
    let thin: Vec<f64> = tiles.iter().map(thinness).collect();

    let mut pixels = vec![GAP; n];
    let mut owner = vec![u32::MAX; n];
    let mut fg = vec![false; n];
    let mut counts: Vec<u32> = Vec::new();

    for cy in 0..h {
        for cx in 0..w {
            let idx = (cy * w + cx) as usize;
            let bucket = &grid[idx];
            if bucket.is_empty() {
                continue; // GAP
            }
            let x0 = bbox.min.x + cx as f64 * sx;
            let y0 = bbox.min.y + cy as f64 * sy;
            // Fast path: a single candidate whose AABB covers the cell
            // fills it completely.
            if bucket.len() == 1 {
                let bb = tiles[bucket[0] as usize].poly.aabb();
                if bb.min.x <= x0 && x0 + sx <= bb.max.x && bb.min.y <= y0 && y0 + sy <= bb.max.y {
                    owner[idx] = bucket[0];
                    pixels[idx] = palette[color_index(mode, &tiles[bucket[0] as usize])];
                    continue;
                }
            }
            // Contested cell: supersample on a K×K grid.
            counts.clear();
            counts.resize(bucket.len(), 0);
            for j in 0..K {
                for i in 0..K {
                    let p = Vec2::new(
                        bbox.min.x + ((cx * K + i) as f64 + 0.5) * (sx / K as f64),
                        bbox.min.y + ((cy * K + j) as f64 + 0.5) * (sy / K as f64),
                    );
                    for (bi, &ti) in bucket.iter().enumerate() {
                        if tiles[ti as usize].poly.contains(p) {
                            counts[bi] += 1;
                        }
                    }
                }
            }
            // Foreground claim: the thinnest covering piece wins the
            // cell when it is in a thinness class of its own.
            let mut order: Vec<usize> =
                (0..bucket.len()).filter(|&bi| counts[bi] >= FG_MIN).collect();
            order.sort_by(|&a, &b| thin[bucket[a] as usize].total_cmp(&thin[bucket[b] as usize]));
            if order.len() >= 2 {
                let t1 = thin[bucket[order[0]] as usize];
                let t2 = thin[bucket[order[1]] as usize];
                if t1 < FG_RATIO * t2 && t1 < FG_CELL * sx.max(sy) {
                    let ti = bucket[order[0]] as usize;
                    owner[idx] = bucket[order[0]];
                    fg[idx] = true;
                    pixels[idx] = palette[color_index(mode, &tiles[ti])];
                    continue;
                }
            }
            // Majority owner; a genuine straddle renders as grout.
            let mut best = 0usize;
            let mut bestc = 0u32;
            let mut second = 0u32;
            for (bi, &c) in counts.iter().enumerate() {
                if c > bestc {
                    second = bestc;
                    bestc = c;
                    best = bi;
                } else if c > second {
                    second = c;
                }
            }
            if bestc == 0 {
                continue; // every sub-sample missed: GAP
            }
            if second >= BORDER_MIN {
                // Grout; the majority owner is kept so that the
                // boundary pass below does not spread the marking to
                // the neighbours of the grout cell.
                owner[idx] = bucket[best];
                pixels[idx] = BORDER;
                continue;
            }
            owner[idx] = bucket[best];
            pixels[idx] = palette[color_index(mode, &tiles[bucket[best] as usize])];
        }
    }

    // Boundary pass: a cell becomes a border when its owner differs from
    // the left or upper (screen) neighbor. Foreground-claimed cells are
    // exempt (they would otherwise be erased again), and they do not
    // mark their neighbors either.
    for cy in 0..h {
        for cx in 0..w {
            let idx = (cy * w + cx) as usize;
            if pixels[idx] == BORDER {
                continue;
            }
            let o = owner[idx];
            if o == u32::MAX || fg[idx] {
                continue;
            }
            let mut border = false;
            if cx > 0 && !fg[idx - 1] && owner[idx - 1] != o {
                border = true;
            }
            if cy + 1 < h && !fg[idx + w as usize] && owner[idx + w as usize] != o {
                border = true;
            }
            if border {
                pixels[idx] = BORDER;
            }
        }
    }

    Raster { w, h, pixels }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Polygon;

    fn tile(proto: u8, seed: u32, pts: &[[f64; 2]]) -> Tile {
        Tile {
            proto,
            seed,
            poly: Polygon::from_slice(&pts.iter().map(|p| Vec2::new(p[0], p[1])).collect::<Vec<_>>()),
        }
    }

    const PAL: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

    #[test]
    fn thin_strip_claims_its_cells() {
        // A 0.2-wide strip across a large square: at 0.5-unit cells the
        // strip is sub-cell, yet every cell it enters renders in the
        // strip's color (foreground rule), not the square's, not grout.
        let square = tile(0, 0, &[[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]]);
        let strip = tile(1, 1, &[[0.0, 4.9], [10.0, 4.9], [10.0, 5.1], [0.0, 5.1]]);
        let bbox = BBox::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let r = raster(&[square, strip], &bbox, 20, 20, ColorMode::Pattern, &PAL);
        // Cell rows spanning y in [4.5, 5.5): all 20 columns strip-colored.
        for cy in [9u32, 10] {
            for cx in 0..20 {
                assert_eq!(r.at(cx, 19 - cy), 1, "cell ({cx},{cy}) not strip");
            }
        }
        // ...and no grout was painted over the strip.
        for cy in 8..12 {
            for cx in 0..20 {
                assert_ne!(r.at(cx, 19 - cy), BORDER, "grout over strip at ({cx},{cy})");
            }
        }
        // Rows beyond the strip are the square's color.
        assert_eq!(r.at(10, 19 - 4), 0);
        assert_eq!(r.at(10, 19 - 15), 0);
    }

    #[test]
    fn abutting_squares_keep_their_boundary() {
        // Two comparable squares sharing the edge x=5: cells the edge
        // cuts through render as grout; interiors keep their colors.
        let a = tile(0, 0, &[[0.0, 0.0], [5.0, 0.0], [5.0, 10.0], [0.0, 10.0]]);
        let b = tile(1, 1, &[[5.0, 0.0], [10.0, 0.0], [10.0, 10.0], [5.0, 10.0]]);
        let bbox = BBox::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        // 15 columns: cell width 2/3, so the seam cuts mid-cell.
        let r = raster(&[a, b], &bbox, 15, 15, ColorMode::Pattern, &PAL);
        let mut grout = 0;
        for cy in 0..15 {
            if r.at(7, 14 - cy) == BORDER {
                grout += 1;
            }
        }
        assert_eq!(grout, 15, "seam column must be grout");
        assert_eq!(r.at(2, 7), 0);
        assert_eq!(r.at(12, 7), 1);
    }

    #[test]
    fn cells_outside_the_tiles_stay_gap() {
        let a = tile(0, 0, &[[0.0, 0.0], [5.0, 0.0], [5.0, 5.0], [0.0, 5.0]]);
        let bbox = BBox::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let r = raster(&[a], &bbox, 10, 10, ColorMode::Pattern, &PAL);
        assert_eq!(r.at(2, 7), 0);
        assert_eq!(r.at(7, 2), GAP);
    }

    #[test]
    fn raster_is_deterministic() {
        let tiles = vec![
            tile(0, 0, &[[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]]),
            tile(1, 1, &[[0.0, 4.9], [10.0, 4.9], [10.0, 5.1], [0.0, 5.1]]),
        ];
        let bbox = BBox::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let r1 = raster(&tiles, &bbox, 23, 17, ColorMode::Pattern, &PAL);
        let r2 = raster(&tiles, &bbox, 23, 17, ColorMode::Pattern, &PAL);
        assert_eq!(r1.pixels, r2.pixels);
    }
}