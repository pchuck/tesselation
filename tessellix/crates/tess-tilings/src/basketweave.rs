//! Basketweave paver pattern (M39) — the classical 2:1-domino
//! basketweave bond, expressed as a [`LatticeTiling`] data table.
//!
//! # The pattern
//!
//! The basketweave paver bond: `2×2` blocks of two parallel 2:1
//! dominoes, block orientation alternating like a checkerboard — the
//! woven look of a wicker basket. In the canonical frame, the block
//! `[2m, 2m+2] × [2n, 2n+2]` carries two horizontal dominoes when
//! `m + n` is even and two vertical ones when it is odd.
//!
//! The tiling is periodic on the 45°-rotated square lattice
//! `⟨ (2,2), (2,−2) ⟩` (area 8); one unit cell holds exactly four
//! dominoes,
//!
//! * `H (0,0)`: `[0,2] × [0,1]`,
//! * `H (0,1)`: `[0,2] × [1,2]`,
//! * `V (2,0)`: `[2,3] × [0,2]`,
//! * `V (3,0)`: `[3,4] × [0,2]`,
//!
//! area `4·2 = 8 = |det|` exactly, so the lattice translates tile the
//! plane with no gaps and no overlaps.
//!
//! # Non-edge-to-edge note
//!
//! At brick scale a domino's long side is a 2-unit edge, so the raw
//! brick tiling is not edge-to-edge (a neighbour's brick end lands in
//! the middle of a long side). As with the Pythagorean T-junction
//! lesson, each long side is split into two unit edges (a collinear
//! mid-side vertex), making the emitted polygons a genuine unit-edge,
//! edge-to-edge partition; the vertex figures then carry `180°`
//! angles at the split midpoints (`2.4.4` at the odd-integer points
//! like `(1, 0)`) and `4.4.4.4` at the even-even block corners.
//!
//! # Wallpaper group
//!
//! **p4g** — not the naive p4m guess: the 4-fold centers sit at every
//! even-even point `(2i, 2j)`, but the mirrors run only along the odd
//! axes `x = 2k+1` / `y = 2k+1` — offset from every 4-fold center,
//! with no diagonal mirrors. Six glide families: the two diagonal
//! families `glide(y, x+2)` / `glide(y+2, x)` and `glide(2−y, −x)` /
//! `glide(4−y, 2−x)` through the 4-fold centers, plus the vertical /
//! horizontal `glide(−x, y+2)` / `glide(x+2, −y)`. The odd-axis
//! vertical/horizontal glides `glide(2−x, y+2)` and `glide(x+2, 2−y)`
//! are *not* symmetries: `T(2,0)` and `T(0,2)` are not translations
//! of the tiling (the translation lattice is exactly
//! `⟨ (2,2), (2,−2) ⟩`).
//!
//! # Seeds and protos
//!
//! `proto` = cell index (0/1 horizontal, 2/3 vertical dominoes);
//! `seed` = orientation class (0 = horizontal, 1 = vertical).
//!
//! # Verification
//!
//! Verified exactly before porting (Python, stdlib) over a bounded
//! lattice patch: cell area sum `8 = |det|`; every interior unit edge
//! shared by exactly 2 dominoes; every interior integer-vertex figure
//! sums to 360° (with the 180° split midpoints); a fine shifted grid
//! over the patch covered exactly once; and a rigorous coset check —
//! `g = (M, v)` is a symmetry iff `M` maps the lattice basis into `L`
//! and each cell's image-domino origin lands in its kind's
//! origin-coset set — confirming the p4g group and the forbidden list
//! above. Rust: the standard trio, the brute-force culling
//! cross-check (`lattice_culling_matches_brute_force`), and the exact
//! in-module group probe (`basketweave_wallpaper_group_is_p4g`).

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

/// Basketweave paver bond — 2×2 blocks of two parallel 2:1 dominoes,
/// checkerboard orientation (p4g on the 45°-rotated square lattice).
pub fn basketweave() -> LatticeTiling {
    // Two horizontal dominoes stacked + two vertical side by side: one
    // woven 4-brick block (the cell is the 4×2 band [0,4]×[0,2]; the
    // lattice translates complete the checkerboard).
    let cells = vec![
        Cell {
            poly: h_domino(0.0, 0.0),
            seed: 0,
        },
        Cell {
            poly: h_domino(0.0, 1.0),
            seed: 0,
        },
        Cell {
            poly: v_domino(2.0, 0.0),
            seed: 1,
        },
        Cell {
            poly: v_domino(3.0, 0.0),
            seed: 1,
        },
    ];

    LatticeTiling {
        id: "basketweave",
        name: "Basketweave (2:1 dominoes)",
        description: "Basketweave paver bond: 2×2 blocks of two parallel 2:1 dominoes, checkerboard orientation. 45°-rotated square lattice <(2,2),(2,−2)>, 4 split dominoes per cell (long sides split into unit edges, 180° vertex figures at the split midpoints). Wallpaper group p4g: 4-folds at the even-even points, mirrors at the odd axes (offset from the 4-fold centers — not p4m), six glide families.",
        lattice: [Vec2::new(2.0, 2.0), Vec2::new(2.0, -2.0)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P4g,
        lattice_kind: LatticeKind::Square,
        category: TilingCategory::Wallpaper,
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
