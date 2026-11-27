//! Herringbone paver pattern (M40) — the chiral single-direction
//! 2:1-domino herringbone bond, expressed as a [`LatticeTiling`] data
//! table.
//!
//! # The pattern
//!
//! The herringbone paver bond: `L`-pairs of one horizontal and one
//! vertical 2:1 domino, `H(a,b) + V(a+2,b)`, laid down at every point
//! of the rectangular lattice `L = ⟨ (2,2), (1,−1) ⟩` (area 4) — one
//! 2-brick cell, `H (0,0)` and `V (2,0)`, area `2·2 = 4 = |det|`, so
//! the lattice translates tile the plane with no gaps and no overlaps.
//! All L-pairs run in the same diagonal direction (this is one of the
//! two enantiomorphs of the herringbone; the pattern is chiral).
//!
//! # Non-edge-to-edge note
//!
//! At brick scale a domino's long side is a 2-unit edge, so the raw
//! brick tiling is not edge-to-edge (a neighbour's brick end lands in
//! the middle of a long side). As with the Pythagorean T-junction
//! lesson and basketweave (M39), each long side is split into two
//! collinear unit edges (a mid-side vertex), making the emitted
//! polygons a genuine unit-edge, edge-to-edge partition; the vertex
//! figures then carry `180°` angles at the split midpoints
//! (`2.4.4`).
//!
//! # Wallpaper group
//!
//! **pgg** — the expected group for the generic 2:1 herringbone:
//! chiral (no mirrors, no 4-folds), with 16 2-fold centers per
//! `4×4` period and two perpendicular glide-reflection families —
//! the diagonal axes `y = x + c` (reps `glide(y+1, x+1)`,
//! `glide(y, x+2)`, `glide(y−1, x+3)`, `glide(y−2, x+4)`) and the
//! anti-diagonal axes `y = −x + c` (reps `glide(1−y, −x)`,
//! `glide(−1−y, −x−2)`). The off-family glides `glide(1−y, 1−x)`,
//! `glide(−x, y+1)` and `glide(x+1, −y)` are absent, and no mirror of
//! any family is a symmetry (that is what keeps pgg, not cmm).
//!
//! # Seeds and protos
//!
//! `proto` = cell index (0 horizontal, 1 vertical domino);
//! `seed` = orientation class (0 horizontal, 1 vertical).
//!
//! # Verification
//!
//! Verified exactly before porting (Python, stdlib) over a bounded
//! lattice patch: cell area sum `4 = |det|`; every interior unit edge
//! shared by exactly 2 dominoes; every interior integer-vertex figure
//! sums to 360° (with the 180° split midpoints); a fine shifted grid
//! over the patch covered exactly once; and a rigorous coset check —
//! `g = (M, v)` is a symmetry iff `M` maps the lattice basis into `L`
//! and each cell's image-domino origin lands in its kind's
//! origin-coset set — confirming the pgg group (16 2-folds + two
//! perpendicular glide families) and the forbidden list above. Rust:
//! the standard trio, the brute-force culling cross-check
//! (`lattice_culling_matches_brute_force`), and the exact in-module
//! group probe (`herringbone_wallpaper_group_is_pgg`).

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
/// two collinear unit edges.
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

/// Herringbone paver bond — L-pairs of 2:1 dominoes on the rectangular
/// lattice ⟨(2,2),(1,−1)⟩ (pgg; chiral, single-direction).
pub fn herringbone() -> LatticeTiling {
    // One L-pair: a horizontal domino + the vertical one completing
    // the herringbone elbow; the lattice translates complete the
    // bond.
    let cells = vec![
        Cell {
            poly: h_domino(0.0, 0.0),
            seed: 0,
        },
        Cell {
            poly: v_domino(2.0, 0.0),
            seed: 1,
        },
    ];

    LatticeTiling {
        id: "herringbone",
        name: "Herringbone (2:1 dominoes)",
        description: "Herringbone paver bond: chiral single-direction L-pairs of 2:1 dominoes on the rectangular lattice <(2,2),(1,−1)>, 2 split dominoes per cell (long sides split into unit edges, 180° vertex figures at the split midpoints). Wallpaper group pgg: 16 2-fold centers per 4×4 period, two perpendicular glide families (diagonal and anti-diagonal axes), no mirrors. In the default Pattern fill the cell seed stores the tile's orientation class (0 = horizontal, 1 = vertical), so the bond colors by tile orientation — the axis-aligned 0° and 90° buckets {0,2} in the Orientation mode (M196), never the 45°/135° diagonal buckets.",
        lattice: [Vec2::new(2.0, 2.0), Vec2::new(1.0, -1.0)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::Pgg,
        lattice_kind: LatticeKind::Rectangular,
        category: TilingCategory::Arrangements,
        secondary: &[TilingCategory::Wallpaper],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
