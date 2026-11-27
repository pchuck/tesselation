//! Stack bond paver pattern (M114) — the plain stacked 2:1-domino
//! bond, expressed as a [`LatticeTiling`] data table.
//!
//! # The pattern
//!
//! The stack-bond paver bond: rows of 2:1 dominoes (stretcher bricks)
//! stacked directly on top of one another — every brick sits exactly
//! above the one below, with no offset between rows (the running
//! bond's half-brick offset is what distinguishes its sibling M41).
//! In the canonical frame the tiling is periodic on the rectangular
//! lattice `⟨ (2,0), (0,1) ⟩` (area 2); one unit cell holds exactly
//! one horizontal domino,
//!
//! * `H (0,0)`: `[0,2] × [0,1]` (row `y ∈ [0,1]`),
//!
//! area `2 = |det|`, so the lattice translates tile the plane with no
//! gaps and no overlaps. Row `y ∈ [j, j+1]` carries the dominoes
//! `[2i, 2i+2] × [j, j+1]` for all `i, j ∈ ℤ` — every row is a
//! translate of the one below it.
//!
//! # Non-edge-to-edge note
//!
//! At brick scale a domino's long side is a 2-unit edge, so the raw
//! brick tiling is not edge-to-edge (a neighbour's brick end lands in
//! the middle of a long side). As with the Pythagorean T-junction
//! lesson, basketweave (M39) and herringbone (M40), each long side is
//! split into two collinear unit edges (a mid-side vertex), making the
//! emitted polygons a genuine unit-edge, edge-to-edge partition; the
//! vertex figures then carry `180°` angles at the split midpoints
//! (the even-x integer lattice points figure as `4.4.4.4`, the odd-x
//! split midpoints as `2.2`).
//!
//! # Wallpaper group
//!
//! **pmm** — mirrors at every vertical axis `x = k` (`k ∈ ℤ`) and at
//! every horizontal axis `y = n/2` (`n ∈ ℤ`, the row boundaries *and*
//! the row midlines); 2-fold centers at every mirror intersection
//! `(k, n/2)`; no glides (the rigid sibling of the running bond's
//! cmm — with no half-brick offset there is nothing for a glide to
//! carry), no 4-folds, no diagonal mirrors.
//!
//! # Seeds and protos
//!
//! `proto` = cell index (0); `seed` = orientation class — a single
//! class (0, horizontal), since every domino of the stack bond is a
//! stretcher. The shipped seed alternates by row via
//! `SeedPattern::Rows`: the tile at lattice index `(i, j)` sits at
//! world `y = j` (lattice `⟨(2,0),(0,1)⟩`), so the row-index parity
//! is exactly the course parity — alternate courses get alternate
//! fills (the running-bond sibling, M41).
//!
//! # Verification
//!
//! Verified exactly before porting (Python, stdlib) over a bounded
//! lattice patch: cell area sum `2 = |det|`; every interior unit edge
//! shared by exactly 2 dominoes; every interior vertex figure sums to
//! 360° (`4.4.4.4` at the even-x integer points, `2.2` at the odd-x
//! split midpoints); a fine shifted grid over the patch covered
//! exactly once (2898 sample points); and a rigorous coset check —
//! `g = (M, v)` is a symmetry iff `M` maps the lattice basis into
//! `L` and each cell's image-domino origin lands in the domino-origin
//! set (a single coset, `L` itself) — confirming the pmm group
//! (vertical mirrors `x = k`, horizontal mirrors `y = n/2`, 2-folds
//! `(k, n/2)`, and no 4-folds, no diagonal mirrors, no glides). Rust:
//! the standard trio, the brute-force culling cross-check
//! (`lattice_culling_matches_brute_force`), and the exact in-module
//! group probe (`stack_bond_wallpaper_group_is_pmm`).

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

/// Stack bond paver pattern — 2:1 dominoes stacked directly on top of
/// one another (pmm on the rectangular lattice ⟨(2,0),(0,1)⟩).
pub fn stack_bond() -> LatticeTiling {
    // One domino per cell; the rectangular lattice ⟨(2,0),(0,1)⟩
    // translates it into the full bond.
    let cells = vec![Cell {
        poly: h_domino(0.0, 0.0),
        seed: 0,
    }];

    LatticeTiling {
        id: "stack-bond",
        name: "Stack bond (2:1 dominoes)",
        description: "Stack-bond paver pattern: rows of 2:1 dominoes stacked directly on top of one another with no offset. Rectangular lattice <(2,0),(0,1)>, one split domino per cell (long sides split into unit edges, 180° vertex figures at the split midpoints). Wallpaper group pmm: mirrors at x = k and y = n/2, 2-folds at (k, n/2), no glides. In the default Pattern fill the pattern seed alternates with the lattice row index j (SeedPattern::Rows, M200), so the bond colors by row parity — alternate courses get alternate fills — not by tile orientation; every domino is a horizontal stretcher (a single orientation class), so all tiles fall in the 0° bucket of the Orientation mode.",
        lattice: [Vec2::new(2.0, 0.0), Vec2::new(0.0, 1.0)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::Pmm,
        lattice_kind: LatticeKind::Rectangular,
        category: TilingCategory::Arrangements,
        secondary: &[],
        seed_pattern: SeedPattern::Rows,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
