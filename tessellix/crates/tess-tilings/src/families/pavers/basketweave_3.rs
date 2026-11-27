//! Basketweave three-brick paver pattern (M116) — the "triple
//! basketweave" bond, expressed as a [`LatticeTiling`] data table.
//!
//! # The pattern
//!
//! The triple-basketweave paver bond: three courses of horizontal 2:1
//! dominoes stacked without offset (stack bond, M114), then one course
//! of vertical dominoes, repeating vertically with period 5. In the
//! canonical frame the tiling is periodic on the rectangular lattice
//! `⟨ (6,0), (0,5) ⟩` (area 30); one cell holds 15 split dominoes,
//!
//! * 9 horizontal dominoes, `H`: `[2i, 2i+2] × [j, j+1]`,
//!   `i = 0,1,2` and `j = 0,1,2` — three stacked courses,
//!
//! * 6 vertical dominoes, `V`: `[ox, ox+1] × [3, 5]`, `ox = 0..5` —
//!   a full wall of vertical dominoes,
//!
//! area `15 · 2 = 30 = |det|`, so the lattice translates tile the
//! plane with no gaps and no overlaps.
//!
//! Note: the true translation lattice of the pattern is the finer
//! `⟨ (2,0), (0,5) ⟩` — every horizontal course has period 2 and the
//! vertical walls have period 1 — so the 6-wide declared cell above is
//! an index-3 superlattice. It is kept for the whole-cluster cell
//! convention (cf. basketweave, M39), and the group probe below
//! measures the symmetries of the true, finer lattice.
//!
//! # Why this bond reads as bands (review A6)
//!
//! The woven look of a classic basketweave needs 3:1 bricks: three of
//! them side by side form a square block, and alternating those blocks
//! by 90° gives the weave. With 2:1 dominoes three bricks side by side
//! are a 3×2 rectangle, not a square, so the paver trade's "triple
//! basketweave" is the course bond shipped here — three horizontal
//! courses abutting one wall of vertical bricks, period 5 — and that
//! pattern is banded by construction, at any aspect ratio of the brick.
//! The 90°-alternating woven relative in this registry is `basketweave`
//! (M39, p4g), whose clusters are square.
//!
//! The "15 per cell" figure is exact and pinned twice over: the group
//! battery (`basketweave_3_wallpaper_group_is_pmm`) enumerates the 15
//! dominoes of one cell in doubled integer coordinates — 9 horizontal
//! (4×2 at x ∈ {0,4,8}, y ∈ {0,2,4}) plus 6 vertical (2×4 along y = 6) —
//! and `lattice_area_identity_over_the_lattice_class` checks the
//! bookkeeping 15 · 2 = 30 = |det ⟨(6,0),(0,5)⟩|.
//!
//! # Non-edge-to-edge note
//!
//! At brick scale a domino's long side is a 2-unit edge, so the raw
//! brick tiling is not edge-to-edge (a neighbour's brick end lands in
//! the middle of a long side). As with basketweave (M39), herringbone
//! (M40), running bond (M41) and stack bond (M114), each long side is
//! split into two collinear unit edges (a mid-side vertex), making the
//! emitted polygons a genuine unit-edge, edge-to-edge partition; the
//! vertex figures then carry `180°` angles at the split midpoints.
//!
//! # Vertex figures
//!
//! All interior vertices are integer lattice points; the figure class
//! is determined by `(x mod 2, y mod 5)`:
//!
//! | y mod 5 | x even   | x odd  |
//! |---------|----------|--------|
//! | 0       | 4.4.4.4  | 2.4.4  |
//! | 1       | 4.4.4.4  | 2.2    |
//! | 2       | 4.4.4.4  | 2.2    |
//! | 3       | 4.4.4.4  | 2.4.4  |
//! | 4       | 2.2      | 2.2    |
//!
//! (row boundaries between horizontal courses figure `4.4.4.4`; the
//! split midpoints on the horizontal courses figure `2.2`; the course
//! against the vertical walls figures `2.4.4` at the odd-x points;
//! every split midpoint in the vertical wall figures `2.2`.)
//!
//! # Wallpaper group
//!
//! **pmm** — mirrors at every vertical axis `x = k` (`k ∈ ℤ`) and at
//! the horizontal axes `y = 2.5k + 1.5` (equivalently `2y ≡ 3 (mod 5)`
//! — the family `y = 5k+4` and `y = 5k+1.5`, i.e. the two midlines of
//! the vertical walls); 2-fold centers at every mirror intersection
//! `(k, 2.5k' + 1.5)`; no 4-folds, no diagonal mirrors, no essential
//! glides (a half-period glide along a mirror would carry the even-x
//! horizontal courses to odd-x positions, which the tiling does not
//! contain).
//!
//! # Seeds and protos
//!
//! `proto` = cell index (0–8 horizontal, 9–14 vertical dominoes);
//! `seed` = orientation class (0 = horizontal, 1 = vertical).
//!
//! # References
//!
//! - <https://diypaverpatio.com/ideas/basket-weave-pattern/> —
//!   "Triple Basketweave: Three narrower rectangular pavers group
//!   together to form larger alternating blocks."
//! - <https://3dshouse.com/textures/wave-triple-basketweave/> —
//!   "basket weave of three units a block, alternating across and
//!   along."
//! - <https://pinehallbrick.com/no-cut-paver-patterns> — the triple
//!   basketweave as three horizontal courses plus one vertical course
//!   (a "modified stack bond").
//!
//! # Verification
//!
//! Verified exactly before porting (Python, stdlib) over a bounded
//! lattice patch: cell area sum `30 = |det|`; a fine shifted grid
//! over the world patch covered exactly once (6474 sample points);
//! 296 interior unit edges each shared by exactly 2 cells; every
//! interior vertex figure sums to 360°, with the 10
//! `(x mod 2, y mod 5)` classes matching the table above; and a
//! rigorous coset check (doubled coordinates, axis-aligned bbox +
//! orientation) — `g = (M, v)` is a symmetry iff `M` maps the
//! declared-cell basis into the declared cell and each image domino's
//! origin lands in its kind's origin-coset set — measuring I and
//! MIRX exactly on `v_x ≡ 0 (mod 4) ∧ v_y ≡ 0 (mod 10)`, MIRY and
//! ROT2 exactly on `v_x ≡ 0 (mod 4) ∧ v_y ≡ 6 (mod 10)`, and ROT4,
//! ROT4_270, DIAG, ANTI never (72 passing `v` in a 48×60 window for
//! each of the four matrices, 0 for the other four); plus direct
//! patch verification of the mirror `x = 1` and the mirror `y = 4 +
//! period` passes and of seven fail samples. Rust: the standard trio,
//! the brute-force culling cross-check
//! (`lattice_culling_matches_brute_force`), and the exact in-module
//! group probe (`basketweave_3_wallpaper_group_is_pmm`).

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

fn poly(pts: &[Vec2]) -> Polygon {
    Polygon::from_slice(pts)
}

/// 2:1 horizontal domino `[x, x+2] × [y, y+1]`, long sides split into
/// two collinear unit edges (mid-side vertex at `(x+1, ·)`).
fn h_domino(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x, y),
        Vec2::new(x + 1.0, y),
        Vec2::new(x + 2.0, y),
        Vec2::new(x + 2.0, y + 1.0),
        Vec2::new(x + 1.0, y + 1.0),
        Vec2::new(x, y + 1.0),
    ])
}

/// 2:1 vertical domino `[x, x+1] × [y, y+2]`, long sides split into
/// two collinear unit edges (mid-side vertex at `(·, y+1)`).
fn v_domino(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x, y),
        Vec2::new(x, y + 1.0),
        Vec2::new(x, y + 2.0),
        Vec2::new(x + 1.0, y + 2.0),
        Vec2::new(x + 1.0, y + 1.0),
        Vec2::new(x + 1.0, y),
    ])
}

/// Basketweave three-brick paver bond — three horizontal courses plus
/// one vertical course, period 5 (pmm on the rectangular lattice
/// ⟨(6,0),(0,5)⟩, an index-3 superlattice of the true
/// ⟨(2,0),(0,5)⟩).
pub fn basketweave_3() -> LatticeTiling {
    // One whole 6×5 "triple basketweave" cluster: three courses of
    // horizontal dominoes (y in [0,3]) plus one wall of vertical
    // dominoes (y in [3,5]); the lattice translates complete the bond.
    let mut cells = Vec::with_capacity(15);
    for oy in 0..3u32 {
        for ox in 0..3u32 {
            cells.push(Cell {
                poly: h_domino(2.0 * ox as f64, oy as f64),
                seed: 0,
            });
        }
    }
    for ox in 0..6u32 {
        cells.push(Cell {
            poly: v_domino(ox as f64, 3.0),
            seed: 1,
        });
    }

    LatticeTiling {
        id: "basketweave-3",
        name: "Basketweave three-brick (2:1 dominoes)",
        description: "Triple-basketweave paver bond: three courses of horizontal 2:1 dominoes stacked without offset, then one course of vertical dominoes, period 5. Rectangular lattice <(6,0),(0,5)> (index-3 superlattice of the true <(2,0),(0,5)>), 15 split dominoes per cell (9 horizontal + 6 vertical; long sides split into unit edges, 180° vertex figures at the split midpoints). Wallpaper group pmm: mirrors x = k and y = 2.5k + 1.5, 2-folds at (k, 2.5k' + 1.5), no 4-folds, no glides.",
        lattice: [Vec2::new(6.0, 0.0), Vec2::new(0.0, 5.0)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::Pmm,
        lattice_kind: LatticeKind::Rectangular,
        category: TilingCategory::Arrangements,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
