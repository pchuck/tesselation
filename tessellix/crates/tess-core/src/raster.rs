//! Rasterizes tiles into a per-pixel color-index buffer.
//!
//! Shared by the terminal UI (the TUI canvas and card thumbnails map
//! the indices to xterm-256 colors) and the desktop GUI (card preview
//! textures map them into [`crate::paint::PALETTE`]). Uses a uniform
//! spatial grid (one bucket per screen cell) so that each pixel only
//! tests the tiles whose bounding box covers its cell.
//!
//! `palette` is a color-index map: the output pixel value is
//! `palette[color_index(mode, tile)]`, clamped through
//! [`palette_slot`]'s last slot — so each front end passes the palette
//! of its choice (the TUI passes the 256-cube indices of the shared
//! 8-color palette, the GUI passes the identity map
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

/// A grout cell that belongs to a large-tile accent outline (Landing B's
/// `--large-tile-outline` / TUI `o` toggle): same geometry as [`BORDER`],
/// painted by the front end in the outline accent instead of the dark grout.
/// Written only by [`raster_aperiodic`] with an outline predicate; every
/// other entry leaves it out of the buffer, so front ends that map it are
/// never byte-affected while the flag is off.
pub const OUTLINE: u8 = 253;

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

/// Map a `color_index` class to a palette slot, clamping out-of-range
/// indices to the last slot.
///
/// The per-mode index space (`color_index`) is fixed at 8 entries for
/// `Pattern`/`PatternGray`/`Proto`/`Gray`/`Shape`/`Role` (`Orientation`
/// is a 4-entry space; [`BORDER`]/[`GAP`] never enter this map), and 9
/// for `Metatile`/`Level` — but front ends pass palettes of their own
/// size. The GUI card path passes a hardcoded 8-entry identity map;
/// without the clamp a `Metatile`/`Level` tile whose class reaches 8
/// (42 registry tilings, see `tess-tilings/tests/palette_index_reachability.rs`)
/// indexes one past the palette and panics. Clamping keeps the result
/// deterministic: valid indices are untouched and the out-of-range slot
/// degrades to the last palette slot rather than aborting the whole card.
fn palette_slot(palette: &[u8], class: usize) -> u8 {
    palette[class.min(palette.len() - 1)]
}

fn color_index(mode: ColorMode, tile: &Tile) -> usize {
    // Landing A family palette rule: the shared `paint::family_slot`
    // override (identity modes + tagged tiles only, slots < 8) keeps
    // this index space in lockstep with `paint::tile_rgb` (M192 sync).
    if let Some(slot) = crate::paint::family_slot(mode, tile) {
        return slot;
    }
    match mode {
        ColorMode::Single => 0,
        // `PatternGray` shares `Pattern`'s index space; the front end
        // picks the pattern-gray ramp (`paint::PATTERN_GRAY_PALETTE`)
        // instead of the color palette.
        ColorMode::Pattern | ColorMode::PatternGray => (tile.seed as usize) % 8,
        // `Gray` shares `Proto`'s index space; the front end picks the
        // gray ramp (`paint::GRAY_PALETTE`) instead of the color palette.
        ColorMode::Proto | ColorMode::Gray => (tile.proto as usize) % 8,
        // `Shape` is polygon-derived exactly like `tile_rgb`; the front end
        // maps the class through the shared color palette.
        ColorMode::Shape => crate::paint::shape_class(&tile.poly),
        // `Role` shares `Pattern`'s seed-index space; the front end picks
        // the warm/earth ramp (`paint::ROLE_PALETTE`) instead of the color palette.
        ColorMode::Role => (tile.seed as usize) % 8,
        // `Metatile` uses the aperiodic classification directly; the front
        // end picks the categorical ramp (`paint::METATILE_PALETTE`).
        ColorMode::Metatile => crate::paint::metatile_class(tile),
        // `Orientation` is polygon-derived exactly like `tile_rgb`; the
        // front end maps the bucket through the orientation ramp
        // (`paint::ORIENTATION_PALETTE`).
        ColorMode::Orientation => crate::paint::orientation_class(&tile.poly),
        // `Level` uses the substitution-level class directly; the front
        // end picks the level ramp (`paint::LEVEL_PALETTE`).
        ColorMode::Level => crate::paint::level_class(tile),
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

/// Rasterize `tiles` into a per-pixel color-index buffer.
///
/// Exactly [`raster_reflected`] with no reflected-palette: the pre-Landing-B
/// entry, byte-identical (the M205 delegator precedent).
pub fn raster(
    tiles: &[Tile],
    bbox: &BBox,
    w: u32,
    h: u32,
    mode: ColorMode,
    palette: &[u8],
) -> Raster {
    raster_reflected(tiles, bbox, w, h, mode, palette, None)
}

/// [`raster`] with the reflected-tile highlight (`--highlight-reflected`).
///
/// `reflected_palette` is the highlight twin of `palette` — same length,
/// each entry the lightened variant of the same class (the front end builds
/// it with [`crate::paint::reflected_palette_256`]). A tile whose `reflected`
/// flag is set writes its class index through that twin; every other tile
/// writes through `palette` as before. `None` (or an unreflected view) is
/// byte-identical to [`raster`].
///
/// Only fills are tagged: [`BORDER`] grout and [`GAP`] cells keep their
/// sentinel indices, so the highlight never recolors the grout pass, and
/// cells whose owner is a reflected tile keep their ordinary owner/border
/// bookkeeping (the boundary pass below is untouched).
pub fn raster_reflected(
    tiles: &[Tile],
    bbox: &BBox,
    w: u32,
    h: u32,
    mode: ColorMode,
    palette: &[u8],
    reflected_palette: Option<&[u8]>,
) -> Raster {
    raster_aperiodic(tiles, bbox, w, h, mode, palette, reflected_palette, None)
}

/// [`raster_reflected`] plus the large-tile outline pass
/// (`--large-tile-outline`).
///
/// `outline` is the marked-tile predicate (the canvas call sites pass
/// [`crate::svg::large_tile_outlined`], the family-keyed law). When set, a
/// grout cell whose ownership touches a marked tile — its own owner, or one
/// of the four owning neighbours — is written [`OUTLINE`] instead of plain
/// [`BORDER`]: the accent traces exactly the marked tiles' boundary, one
/// cell wide, over grout that is already there (the raster draws hairline
/// strokes as grout cells, so the outline recolors the boundary grout of
/// the marked family rather than inventing a second stroke geometry). `None`
/// is byte-identical to [`raster_reflected`].
///
/// Like the SVG pass, this is keyed on `tile.family`/`tile.proto`, so it is
/// inert for cards with no large-tile notion (no cell is ever marked).
pub fn raster_aperiodic(
    tiles: &[Tile],
    bbox: &BBox,
    w: u32,
    h: u32,
    mode: ColorMode,
    palette: &[u8],
    reflected_palette: Option<&[u8]>,
    outline: Option<&dyn Fn(&Tile) -> bool>,
) -> Raster {
    // Class → palette index for one tile, choosing the highlight twin for
    // reflected tiles when the caller supplied one.
    let slot = |tile: &Tile| -> u8 {
        let pal = match reflected_palette {
            Some(rp) if tile.reflected => rp,
            _ => palette,
        };
        palette_slot(pal, color_index(mode, tile))
    };
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
                    pixels[idx] = slot(&tiles[bucket[0] as usize]);
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
            let mut order: Vec<usize> = (0..bucket.len())
                .filter(|&bi| counts[bi] >= FG_MIN)
                .collect();
            order.sort_by(|&a, &b| thin[bucket[a] as usize].total_cmp(&thin[bucket[b] as usize]));
            if order.len() >= 2 {
                let t1 = thin[bucket[order[0]] as usize];
                let t2 = thin[bucket[order[1]] as usize];
                if t1 < FG_RATIO * t2 && t1 < FG_CELL * sx.max(sy) {
                    let ti = bucket[order[0]] as usize;
                    owner[idx] = bucket[order[0]];
                    fg[idx] = true;
                    pixels[idx] = slot(&tiles[ti]);
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
            pixels[idx] = slot(&tiles[bucket[best] as usize]);
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

    // Outline pass (Landing B): recolor the boundary grout of the marked
    // family. Runs after the boundary pass so it sees the finished grout
    // network; fills and GAP are never touched, and grout that bounds no
    // marked tile keeps its dark BORDER value.
    if let Some(mark) = outline {
        let hits: Vec<bool> = owner
            .iter()
            .map(|&o| o != u32::MAX && mark(&tiles[o as usize]))
            .collect();
        for cy in 0..h {
            for cx in 0..w {
                let idx = (cy * w + cx) as usize;
                if pixels[idx] != BORDER {
                    continue;
                }
                let mut hit = hits[idx];
                if !hit && cx > 0 {
                    hit = hits[idx - 1];
                }
                if !hit && cx + 1 < w {
                    hit = hits[idx + 1];
                }
                if !hit && cy > 0 {
                    hit = hits[idx - w as usize];
                }
                if !hit && cy + 1 < h {
                    hit = hits[idx + w as usize];
                }
                if hit {
                    pixels[idx] = OUTLINE;
                }
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
        Tile { family: crate::tiling::FamilyTag::None, reflected: false,
            proto,
            seed, level: 0,
            poly: Polygon::from_slice(
                &pts.iter()
                    .map(|p| Vec2::new(p[0], p[1]))
                    .collect::<Vec<_>>(),
            ),
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

    /// Designed explicit orientation pin: four 5x5 quadrant tiles covering a
    /// 10x10 view (seeds 1/2/3/4 = pattern-mode pixel values 1/2/3/4), asserting
    /// BOTH axes at the two screen corners plus interior cells. Unlike the
    /// incidental flip pin in `cells_outside_the_tiles_stay_gap` (which covers a
    /// single bottom-half cell and misses rows shifted by any amount), this fails
    /// under a row flip AND under a row shift. Seam cells (screen column 5 and
    /// screen row 5) render as grout — the two 5-unit splits land exactly on the
    /// pixel grid boundary, so every quadrant interior cell is a single-candidate
    /// fast-path fill, no supersampling.
    #[test]
    fn quadrant_layout_pins_both_axes_at_screen_corners() {
        let tl = tile(0, 1, &[[0.0, 5.0], [5.0, 5.0], [5.0, 10.0], [0.0, 10.0]]);
        let tr = tile(1, 2, &[[5.0, 5.0], [10.0, 5.0], [10.0, 10.0], [5.0, 10.0]]);
        let bl = tile(2, 3, &[[0.0, 0.0], [5.0, 0.0], [5.0, 5.0], [0.0, 5.0]]);
        let br = tile(3, 4, &[[5.0, 0.0], [10.0, 0.0], [10.0, 5.0], [5.0, 5.0]]);
        let bbox = BBox::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let r = raster(&[tl, tr, bl, br], &bbox, 10, 10, ColorMode::Pattern, &PAL);

        // The documented fact, at both screen corners. Row 0 is top,
        // column 0 is left: the tl quadrant must be at (0, 0).
        assert_eq!(r.at(0, 0), 1, "screen (0, 0) must be world top-left");
        assert_eq!(r.at(9, 9), 4, "screen (w-1, h-1) must be world bottom-right");

        // Both axes, explicit screen coordinates (not just the corners).
        assert_eq!(r.at(9, 0), 2, "screen (w-1, 0) must be world top-right");
        assert_eq!(r.at(0, 9), 3, "screen (0, h-1) must be world bottom-left");

        // Interior cells of each quadrant.
        assert_eq!(r.at(2, 2), 1, "world top-left interior");
        assert_eq!(r.at(7, 2), 2, "world top-right interior");
        assert_eq!(r.at(2, 7), 3, "world bottom-left interior");
        assert_eq!(r.at(7, 7), 4, "world bottom-right interior");

        // The seams: column 5 and row 5 are the split lines — grout, not tile
        // color, on both axes at the corner-adjacent cells.
        assert_eq!(r.at(5, 0), BORDER, "vertical seam (screen col 5) must be grout");
        assert_eq!(r.at(5, 9), BORDER, "vertical seam must be grout at the bottom");
        assert_eq!(r.at(0, 5), BORDER, "horizontal seam (screen row 5) must be grout");
        assert_eq!(r.at(9, 5), BORDER, "horizontal seam must be grout at the right");
    }

    /// Sentinel-value distinctness: BORDER (255) and GAP (254) must be distinct
    /// from each other and from every possible index of the 8-entry palette space
    /// (pattern/proto mode: pixel value = seed % 8, i.e. indices 0..=7). Front
    /// ends match on exactly these two values — if a sentinel ever fell inside
    /// the palette space, or duplicated the other sentinel, a real pixel would be
    /// indistinguishable from a sentinel and every front end would silently
    /// mismap it. Sentinels sit at the top of the u8 range (255/254), so no seed
    /// index space up to 255 can collide with them; the pin here is against the
    /// documented 8-entry space, so any future palette growth stays collision-free.
    #[test]
    fn border_gap_sentinels_distinct_from_seed_indices() {
        const SEED_SPACE: u8 = 8; // seed % 8 → indices 0..=7
        assert_eq!(BORDER, 255, "BORDER sentinel value");
        assert_eq!(GAP, 254, "GAP sentinel value");
        assert_ne!(BORDER, GAP, "BORDER and GAP must be distinct sentinels");
        assert!(BORDER >= SEED_SPACE, "BORDER must exceed every seed index (0..=7)");
        assert!(GAP >= SEED_SPACE, "GAP must exceed every seed index (0..=7)");
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

    #[test]
    fn pattern_gray_shares_pattern_index_space() {
        // PatternGray produces byte-identical rasters to Pattern through
        // one palette: same index space (seed % 8), only the ramp differs
        // (the front end's choice).
        let a = tile(0, 0, &[[0.0, 0.0], [4.0, 0.0], [4.0, 4.0], [0.0, 4.0]]);
        let b = tile(1, 5, &[[0.0, 1.9], [4.0, 1.9], [4.0, 2.1], [0.0, 2.1]]);
        let bbox = BBox::new(Vec2::new(0.0, 0.0), Vec2::new(4.0, 4.0));
        assert_eq!(
            raster(&[a.clone(), b.clone()], &bbox, 20, 20, ColorMode::PatternGray, &PAL).pixels,
            raster(&[a, b], &bbox, 20, 20, ColorMode::Pattern, &PAL).pixels
        );
        // color_index directly: seed % 8, the proto field never leaks.
        let t = tile(3, 9, &[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
        assert_eq!(
            color_index(ColorMode::PatternGray, &t),
            color_index(ColorMode::Pattern, &t)
        );
        assert_eq!(color_index(ColorMode::PatternGray, &t), 9 % 8);
    }

    #[test]
    fn gray_mode_shares_proto_index_space() {
        // Gray must produce byte-identical rasters to Proto through one
        // palette: same index space, only the ramp differs (the front
        // end's choice).
        let a = tile(0, 0, &[[0.0, 0.0], [4.0, 0.0], [4.0, 4.0], [0.0, 4.0]]);
        let b = tile(3, 9, &[[0.0, 1.9], [4.0, 1.9], [4.0, 2.1], [0.0, 2.1]]);
        let bbox = BBox::new(Vec2::new(0.0, 0.0), Vec2::new(4.0, 4.0));
        assert_eq!(
            raster(&[a.clone(), b.clone()], &bbox, 20, 20, ColorMode::Gray, &PAL).pixels,
            raster(&[a, b], &bbox, 20, 20, ColorMode::Proto, &PAL).pixels
        );
    }

    #[test]
    fn level_mode_uses_the_level_class_index_space() {
        // color_index(Level) is the paint-level class (0 neutral,
        // 1 + (level - 1) % 8) — the front end maps it through the
        // level ramp. The field is the only input: proto/seed must not
        // leak. Indices stay inside the 9-entry space (0..=8), far
        // below the 254/255 sentinels.
        let poly = |x: f64| {
            tile(3, 7, &[[x, 0.0], [x + 1.0, 0.0], [x + 1.0, 1.0], [x, 1.0]])
        };
        let mut t = poly(0.0);
        for l in 0u8..=8 {
            t.level = l;
            assert_eq!(color_index(ColorMode::Level, &t), l as usize, "level {l}");
        }
        t.level = 9;
        assert_eq!(color_index(ColorMode::Level, &t), 1, "level 9 cycles to 1");
        t.level = 255;
        assert_eq!(color_index(ColorMode::Level, &t), 7);
        assert!(color_index(ColorMode::Level, &t) < LEVEL_INDEX_SPACE);
    }

    /// HAZARD 1 pin (b2b4-recon: "raster index-space panic"): the modes
    /// whose index space runs one past the 8-entry palette the GUI card
    /// path passes (Metatile: proto % 9; Level: 1 + (level - 1) % 8)
    /// must render deterministically — the out-of-range class clamps to
    /// the last palette slot instead of panicking. A valid index is
    /// untouched by the clamp.
    #[test]
    fn b2b4_hazard_metatile_index_8_clamps_instead_of_panicking() {
        // Metatile: proto 8 -> class 8 (8 % 9), one past the 8-entry palette.
        let t = tile(8, 0, &[[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]]);
        assert_eq!(color_index(ColorMode::Metatile, &t), 8);
        let bbox = BBox::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let tiles = vec![t];
        let r = raster(&tiles, &bbox, 20, 20, ColorMode::Metatile, &PAL);
        assert!(r.pixels.iter().all(|&p| p == 7 || p == BORDER),
            "index 8 clamps to the last 8-entry palette slot (7), not a panic");
        assert!(r.pixels.iter().any(|&p| p == 7), "clamped tile rendered");
        // A valid index 7 stays 7 (no shift).
        let t7 = tile(7, 0, &[[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]]);
        assert_eq!(color_index(ColorMode::Metatile, &t7), 7);
        let r7 = raster(&[t7], &bbox, 20, 20, ColorMode::Metatile, &PAL);
        assert!(r7.pixels.iter().all(|&p| p == 7 || p == BORDER));
    }

    #[test]
    fn b2b4_hazard_level_index_8_clamps_instead_of_panicking() {
        // Level: level 8 -> class 1 + (8 - 1) % 8 = 8.
        let mut t = tile(0, 0, &[[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]]);
        t.level = 8;
        assert_eq!(color_index(ColorMode::Level, &t), 8);
        let bbox = BBox::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let tiles = vec![t];
        let r = raster(&tiles, &bbox, 20, 20, ColorMode::Level, &PAL);
        assert!(r.pixels.iter().all(|&p| p == 7 || p == BORDER),
            "index 8 clamps to the last 8-entry palette slot (7), not a panic");
        assert!(r.pixels.iter().any(|&p| p == 7), "clamped tile rendered");
        // A valid index 7 stays 7.
        let mut t7 = tile(0, 0, &[[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]]);
        t7.level = 7;
        assert_eq!(color_index(ColorMode::Level, &t7), 7);
        let r7 = raster(&[t7], &bbox, 20, 20, ColorMode::Level, &PAL);
        assert!(r7.pixels.iter().all(|&p| p == 7 || p == BORDER));
    }

    /// The Level ramp adds a 9-entry index space (0..=8); sentinels at
    /// 254/255 stay collision-free for it too (same argument as the
    /// 8-entry space pin above).
    const LEVEL_INDEX_SPACE: usize = 9;

    // ---- Landing B: reflected-tile highlight (two-palette raster) -------

    /// A highlight twin of [`PAL`] with values no class can produce on its
    /// own, so "this pixel went through the twin" is unambiguous.
    const TWIN: [u8; 8] = [100, 101, 102, 103, 104, 105, 106, 107];

    #[test]
    fn raster_reflected_off_is_byte_identical_to_raster() {
        // Off-state byte identity, both ways: no twin (`None`), and a twin
        // present but no reflected tile in the view.
        let left = tile(0, 1, &[[0.0, 0.0], [5.0, 0.0], [5.0, 10.0], [0.0, 10.0]]);
        let right = tile(1, 3, &[[5.0, 0.0], [10.0, 0.0], [10.0, 10.0], [5.0, 10.0]]);
        let tiles = [left, right];
        let bbox = BBox::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        for mode in [
            ColorMode::Pattern,
            ColorMode::Metatile,
            ColorMode::Level,
            ColorMode::Shape,
            ColorMode::Orientation,
        ] {
            let base = raster(&tiles, &bbox, 20, 20, mode, &PAL);
            let no_twin = raster_reflected(&tiles, &bbox, 20, 20, mode, &PAL, None);
            let twin_but_flat = raster_reflected(&tiles, &bbox, 20, 20, mode, &PAL, Some(&TWIN));
            assert_eq!(base.pixels, no_twin.pixels, "{mode:?}: None twin must be byte-identical");
            assert_eq!(
                base.pixels, twin_but_flat.pixels,
                "{mode:?}: an all-unreflected view must be byte-identical with a twin present"
            );
        }
    }

    #[test]
    fn raster_reflected_tags_reflected_fills_and_leaves_sentinels() {
        // Left half reflected, right half not, on a two-class view: the
        // left interior reads the twin value, the right interior its own,
        // the seam stays BORDER grout and the outside stays GAP (the
        // sentinels never enter the palette map, so they cannot be tagged).
        let mut left = tile(0, 0, &[[0.0, 0.0], [5.0, 0.0], [5.0, 10.0], [0.0, 10.0]]);
        left.reflected = true;
        let right = tile(0, 1, &[[5.0, 0.0], [10.0, 0.0], [10.0, 10.0], [5.0, 10.0]]);
        let bbox = BBox::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let r = raster_reflected(&[left, right], &bbox, 20, 20, ColorMode::Pattern, &PAL, Some(&TWIN));
        for cy in 0..20 {
            // Cell interiors (columns 1..9 left, 11..19 right).
            for cx in 1..9 {
                assert_eq!(r.at(cx, cy), 100, "reflected left cell ({cx},{cy})");
            }
            for cx in 11..19 {
                assert_eq!(r.at(cx, cy), 1, "unreflected right cell ({cx},{cy})");
            }
        }
        // The seam column is grout, not a tagged fill.
        assert!((0..20).all(|cy| r.at(10, cy) == BORDER || r.at(10, cy) == 1 || r.at(10, cy) == 100));
        assert!(
            (0..20).any(|cy| r.at(10, cy) == BORDER),
            "the owner change must still paint grout"
        );

        // GAP: a tile covering a quarter of the frame leaves the rest GAP,
        // tagged or not.
        let mut corner = tile(0, 0, &[[0.0, 0.0], [5.0, 0.0], [5.0, 5.0], [0.0, 5.0]]);
        corner.reflected = true;
        let big = BBox::new(Vec2::new(0.0, 0.0), Vec2::new(20.0, 20.0));
        let r2 = raster_reflected(&[corner], &big, 20, 20, ColorMode::Pattern, &PAL, Some(&TWIN));
        assert!(r2.pixels.iter().any(|&p| p == GAP), "uncovered cells stay GAP");
        assert!(
            !r2.pixels.iter().any(|&p| p == 101 || p == 102 || p == 103),
            "no palette slot other than the tile's own class may appear"
        );
    }

    // ---- Landing B: large-tile outline pass -----------------------------

    /// Two squares at `x`..`x+5` (same layout the reflected tests use).
    fn pair() -> [Tile; 2] {
        [
            tile(0, 0, &[[0.0, 0.0], [5.0, 0.0], [5.0, 10.0], [0.0, 10.0]]),
            tile(1, 1, &[[5.0, 0.0], [10.0, 0.0], [10.0, 10.0], [5.0, 10.0]]),
        ]
    }

    #[test]
    fn raster_aperiodic_outline_off_is_byte_identical() {
        let tiles = pair();
        let bbox = BBox::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        for mode in [
            ColorMode::Pattern,
            ColorMode::Metatile,
            ColorMode::Level,
            ColorMode::Orientation,
        ] {
            let base = raster(&tiles, &bbox, 20, 20, mode, &PAL);
            let off =
                raster_aperiodic(&tiles, &bbox, 20, 20, mode, &PAL, None, Some(&|_| false));
            let none = raster_aperiodic(&tiles, &bbox, 20, 20, mode, &PAL, None, None);
            assert_eq!(base.pixels, none.pixels, "{mode:?}: no outline predicate");
            assert_eq!(
                base.pixels, off.pixels,
                "{mode:?}: a predicate that marks nothing must be a byte no-op"
            );
            assert!(!off.pixels.iter().any(|&p| p == OUTLINE));
        }
    }

    #[test]
    fn raster_aperiodic_outline_recolors_only_marked_boundary_grout() {
        // Left tile marked, right not: the shared seam (a four-sample
        // straddle) and the left tile's outer edge (GAP side, owner =
        // marked tile) read OUTLINE; a border between two UNMARKED owners
        // stays BORDER; fills and GAP never change.
        let tiles = pair();
        let bbox = BBox::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let off = raster(&tiles, &bbox, 20, 20, ColorMode::Pattern, &PAL);
        let on = raster_aperiodic(&tiles, &bbox, 20, 20, ColorMode::Pattern, &PAL, None, Some(
            &|t: &Tile| t.seed == 0,
        ));
        // The seam column (cx = 10) is grout in the off state and accent
        // in the on state.
        let seam_off = (0..20).filter(|&cy| off.at(10, cy) == BORDER).count();
        let seam_on = (0..20).filter(|&cy| on.at(10, cy) == OUTLINE).count();
        assert!(seam_off > 0, "the seam must be grout in the off state");
        assert_eq!(seam_on, seam_off, "every seam grout cell becomes accent");
        for cy in 0..20 {
            assert_eq!(on.at(10, cy), match off.at(10, cy) {
                BORDER => OUTLINE,
                v => v,
            });
        }
        // Fills are untouched everywhere.
        for cy in 0..20 {
            for cx in 0..20 {
                if off.at(cx, cy) != BORDER {
                    assert_eq!(on.at(cx, cy), off.at(cx, cy), "fill at ({cx},{cy}) moved");
                }
            }
        }
        // Unmarked-only view: two tiles that both fail the predicate keep
        // plain BORDER.
        let none = raster_aperiodic(&tiles, &bbox, 20, 20, ColorMode::Pattern, &PAL, None, Some(
            &|t: &Tile| t.seed == 99,
        ));
        assert!(!none.pixels.iter().any(|&p| p == OUTLINE));
        // GAP side of a marked tile: a half-covering marked tile outlines
        // its grout rim, not the GAP.
        let mut half = tile(0, 0, &[[0.0, 0.0], [5.0, 0.0], [5.0, 5.0], [0.0, 5.0]]);
        half.proto = 4;
        let big = BBox::new(Vec2::new(0.0, 0.0), Vec2::new(20.0, 20.0));
        let r = raster_aperiodic(&[half], &big, 20, 20, ColorMode::Pattern, &PAL, None, Some(
            &|t: &Tile| t.proto == 4,
        ));
        assert!(r.pixels.iter().any(|&p| p == OUTLINE), "marked rim outlined");
        assert!(r.pixels.iter().any(|&p| p == GAP), "GAP stays GAP");
        assert!(
            !r.pixels.iter().any(|&p| p == BORDER),
            "the only tile is marked, so all its boundary grout is accent"
        );
    }
}
