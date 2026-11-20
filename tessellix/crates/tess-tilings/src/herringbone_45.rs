//! Herringbone 45° paver pattern (M115) — the 45°-laid canonical
//! frame of the herringbone bond (M40), expressed as a
//! [`LatticeTiling`] data table.
//!
//! # The pattern
//!
//! The same chiral single-direction 2:1-domino herringbone bond as
//! `herringbone` (M40), re-framed the way the paver catalog lays it:
//! the bond running along the world axes at 45°. Concretely it is the
//! M40 bond rotated by 45° (the rotation `A/√2` with
//! `A = [[1,−1],[1,1]]`). In this canonical frame the bond is
//! periodic on the rectangular lattice
//! `⟨ (0, 2√2), (√2, 0) ⟩` (area 4); one unit cell holds the usual
//! L-pair, each domino at 45° with long sides split into two
//! collinear unit edges (mid-side vertices),
//!
//! * `H (0,0)`: `(0,0), (√2/2, √2/2), (√2, √2), (√2/2, 3√2/2),
//!   (0, √2), (−√2/2, √2/2)` — area 2,
//!
//! * `V (√2, √2)`: `(√2, √2), (3√2/2, 3√2/2), (√2, 2√2),
//!   (√2/2, 5√2/2), (0, 2√2), (√2/2, 3√2/2)` — area 2,
//!
//! area `4 = |det|`, so the lattice translates tile the plane with no
//! gaps and no overlaps. (The pattern stays chiral under the
//! re-framing: this table is the same M40 enantiomorph, only rotated.)
//!
//! # Non-edge-to-edge note
//!
//! At brick scale a domino's long side is a 2-unit edge, so the raw
//! brick tiling is not edge-to-edge (a neighbour's brick end lands in
//! the middle of a long side). As with basketweave (M39), herringbone
//! (M40) and the running bond (M41), each long side is split into two
//! collinear unit edges (a mid-side vertex), making the emitted
//! polygons a genuine unit-edge, edge-to-edge partition; the vertex
//! figures then carry `180°` angles at the split midpoints — every
//! interior vertex of the 45° frame figures as `2.4.4`.
//!
//! # Wallpaper group
//!
//! **pgg** — the expected group for the generic 2:1 herringbone,
//! invariant under the 45° re-framing: chiral (no mirrors of any
//! family, no 4-folds), with 2-fold centers at
//! `(√2/4 + k√2/2, 3√2/4 + ℓ√2)` (i.e. `v ≡ (1,3) mod (2,4)` in the
//! table's P-coords) and two perpendicular glide-reflection families
//! descended from M40's diagonal and anti-diagonal glides under the
//! 45° rotation: vertical axes `x = k√2/2` with glide `√2`, and
//! horizontal axes `y = ℓ√2 + √2/4` with glide `√2/2`.
//!
//! # Seeds and protos
//!
//! `proto` = cell index (0 horizontal, 1 vertical domino);
//! `seed` = orientation class (0 horizontal, 1 vertical).
//!
//! # Verification
//!
//! Verified exactly before porting (Python, stdlib) in P-coords —
//! integer, with world `= (√2/2)·P`: cell area sum `8 = |det|`
//! (world `4 = 4`); a fine shifted grid over the world patch covered
//! exactly once (3906 sample points); 192 interior unit edges each
//! shared by exactly 2 dominoes; vertex figures over all 113 interior
//! vertices sum to 360° and the four vertex classes mod (2,4) —
//! (0,0), (1,1), (0,2), (1,3) — all figure `2.4.4`; and a rigorous
//! coset check — `g = (M, v)` is a symmetry iff `M` maps the lattice
//! basis into `L'' = ⟨(2,0),(0,4)⟩` and every cell's image shape
//! equals some target shape translated by a lattice vector (anchor
//! test — the rotated hexagons break the axis-aligned-bbox probe) —
//! measuring I exactly on `v ∈ L''` (x even, y ≡ 0 mod 4), ROT2
//! exactly on `v ≡ (1,3) mod (2,4)`, MIRX exactly on (x even,
//! y ≡ 2 mod 4) — the vertical glide family — MIRY exactly on
//! (x odd, y ≡ 1 mod 4) — the horizontal glide family — and ROT4,
//! ROT4_270, DIAG, ANTI never (64 passing v in a 2-period window for
//! each of the four matrices, 0 for the other four). Cross-checked by
//! conjugating the independently measured M40 group by the 45°
//! rotation: all eight candidate-matrix cosets agree. Rust: the
//! standard trio, the brute-force culling cross-check
//! (`lattice_culling_matches_brute_force`), and the exact in-module
//! group probe (`herringbone_45_wallpaper_group_is_pgg`).

use std::f64::consts::SQRT_2;

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

fn poly(pts: &[Vec2]) -> Polygon {
    Polygon::from_slice(pts)
}

/// The 45° frame's scale: P-coord → world `= (√2/2)·P`, i.e. the M40
/// bond rotated by 45°.
const HALF_SQRT2: f64 = SQRT_2 / 2.0;

/// 2:1 domino at 45° (M40's `h_domino(0,0)` rotated 45°), long sides
/// split into two collinear unit edges.
fn h45_domino() -> Polygon {
    poly(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(HALF_SQRT2, HALF_SQRT2),
        Vec2::new(SQRT_2, SQRT_2),
        Vec2::new(HALF_SQRT2, 3.0 * HALF_SQRT2),
        Vec2::new(0.0, SQRT_2),
        Vec2::new(-HALF_SQRT2, HALF_SQRT2),
    ])
}

/// 2:1 domino at 45° (M40's `v_domino(2,0)` rotated 45°), long sides
/// split into two collinear unit edges.
fn v45_domino() -> Polygon {
    poly(&[
        Vec2::new(SQRT_2, SQRT_2),
        Vec2::new(3.0 * HALF_SQRT2, 3.0 * HALF_SQRT2),
        Vec2::new(SQRT_2, 2.0 * SQRT_2),
        Vec2::new(HALF_SQRT2, 5.0 * HALF_SQRT2),
        Vec2::new(0.0, 2.0 * SQRT_2),
        Vec2::new(HALF_SQRT2, 3.0 * HALF_SQRT2),
    ])
}

/// Herringbone 45° paver pattern — the M40 bond in its 45°-laid
/// canonical frame (pgg on the rectangular lattice
/// ⟨(0,2√2),(√2,0)⟩).
pub fn herringbone_45() -> LatticeTiling {
    // One L-pair: the horizontal 45° domino and its vertical
    // partner completing the herringbone elbow; the lattice
    // translates complete the bond.
    let cells = vec![
        Cell {
            poly: h45_domino(),
            seed: 0,
        },
        Cell {
            poly: v45_domino(),
            seed: 1,
        },
    ];

    LatticeTiling {
        id: "herringbone-45",
        name: "Herringbone 45° (2:1 dominoes)",
        description: "Herringbone paver bond in its 45°-laid canonical frame: chiral single-direction L-pairs of 2:1 dominoes on the rectangular lattice <(0,2√2),(√2,0)>, 2 split dominoes per cell (long sides split into unit edges, 180° vertex figures at the split midpoints). Wallpaper group pgg: 2-fold centers at (√2/4 + k√2/2, 3√2/4 + ℓ√2), vertical glide axes x = k√2/2 (glide √2), horizontal glide axes y = ℓ√2 + √2/4 (glide √2/2), no mirrors.",
        lattice: [Vec2::new(0.0, 2.0 * SQRT_2), Vec2::new(SQRT_2, 0.0)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::Pgg,
        lattice_kind: LatticeKind::Rectangular,
        category: TilingCategory::Arrangements,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
