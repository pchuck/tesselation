//! Running bond paver pattern (M41) — the classic stretcher
//! (running) bond, expressed as a [`LatticeTiling`] data table.
//!
//! # The pattern
//!
//! The running-bond paver bond: rows of 2:1 dominoes (stretcher
//! bricks) with each row offset by half a brick relative to its
//! neighbours — the plain sibling of the paver trio. In the
//! canonical frame the tiling is periodic on the rectangular lattice
//! `⟨ (2,0), (0,2) ⟩` (area 4); one unit cell holds exactly two
//! horizontal dominoes,
//!
//! * `H (0,0)`: `[0,2] × [0,1]` (row `y ∈ [0,1]`, even-x dominoes),
//! * `H (1,1)`: `[1,3] × [1,2]` (row `y ∈ [1,2]`, odd-x dominoes),
//!
//! area `2·2 = 4 = |det|`, so the lattice translates tile the plane
//! with no gaps and no overlaps. Even rows (`y ∈ [2l, 2l+1]` with
//! `l` even) carry even-x dominoes; odd rows carry odd-x dominoes —
//! the half-brick offset.
//!
//! # Non-edge-to-edge note
//!
//! At brick scale a domino's long side is a 2-unit edge, so the raw
//! brick tiling is not edge-to-edge (a neighbour's brick end lands in
//! the middle of a long side). As with the Pythagorean T-junction
//! lesson, basketweave (M39) and herringbone (M40), each long side is
//! split into two collinear unit edges (a mid-side vertex), making
//! the emitted polygons a genuine unit-edge, edge-to-edge partition;
//! the vertex figures then carry `180°` angles at the split
//! midpoints (`2.4.4`).
//!
//! # Wallpaper group
//!
//! **cmm** — mirrors at every integer axis `x = k` (all `k ∈ ℤ`) and
//! at the half-integer axes `y = l + 1/2`; 2-fold centers at
//! `(k, l+1/2)` (on the mirrors) and at `(k+1/2, l)` (between the
//! rows); two glide families in each direction — reps
//! `glide(x+1, −y)` / `glide(x+1, 4−y)` and `glide(1−x, y+1)` /
//! `glide(3−x, y+1)`; no diagonal mirrors and no 4-folds; the
//! off-axis glides `glide(x+1, 1−y)` and `glide(−x, y+1)` are
//! absent.
//!
//! # Seeds and protos
//!
//! `proto` = cell index (both dominoes horizontal); `seed` =
//! orientation class — a single class (0, horizontal), since every
//! domino of the running bond is a stretcher.
//!
//! # Verification
//!
//! Verified exactly before porting (Python, stdlib) over a bounded
//! lattice patch: cell area sum `4 = |det|`; every interior unit edge
//! shared by exactly 2 dominoes; every interior integer-vertex figure
//! sums to 360° (with the 180° split midpoints); a fine shifted grid
//! over the patch covered exactly once; and a rigorous coset check —
//! `g = (M, v)` is a symmetry iff `M` maps the lattice basis into
//! `L = 2ℤ²` and each cell's image-domino origin lands in the
//! domino-origin set `L ∪ (L + (1,1))` — confirming the cmm group
//! (mirrors at the integer x-axes and half-integer y-axes, centered
//! 2-folds, two glide families per direction) and the forbidden list
//! above. Rust: the standard trio, the brute-force culling
//! cross-check (`lattice_culling_matches_brute_force`), and the exact
//! in-module group probe (`running_bond_wallpaper_group_is_cmm`).

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

/// Running bond paver pattern — stretcher rows offset by half a brick
/// (cmm on the rectangular lattice ⟨(2,0),(0,2)⟩).
pub fn running_bond() -> LatticeTiling {
    // One row + its half-offset successor: the even-x domino of row 0
    // and the odd-x domino of row 1; the lattice translates complete
    // the bond.
    let cells = vec![
        Cell {
            poly: h_domino(0.0, 0.0),
            seed: 0,
        },
        Cell {
            poly: h_domino(1.0, 1.0),
            seed: 0,
        },
    ];

    LatticeTiling {
        id: "running-bond",
        name: "Running bond (2:1 dominoes)",
        description: "Running-bond paver pattern: stretcher rows of 2:1 dominoes, each row offset by half a brick. Rectangular lattice <(2,0),(0,2)>, 2 split dominoes per cell (long sides split into unit edges, 180° vertex figures at the split midpoints). Wallpaper group cmm: mirrors at the integer x-axes and half-integer y-axes, 2-folds at (k, l+1/2) and (k+1/2, l), glides in both directions.",
        lattice: [Vec2::new(2.0, 0.0), Vec2::new(0.0, 2.0)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::Cmm,
        lattice_kind: LatticeKind::Rectangular,
        category: TilingCategory::Wallpaper,
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
