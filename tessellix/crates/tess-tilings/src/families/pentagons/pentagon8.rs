//! Pentagon type 8 (Richard Kershner, 1968) tiling — 2-isohedral,
//! edge-to-edge, wallpaper group pgg.
//!
//! Type 8 is the oldest of the fifteen convex-pentagon families
//! (Kershner 1968).  It is a two-parameter family with conditions
//! `b = c = d = e` and `2B + C = D + 2E = 360°`; the instance shipped
//! here is the exact Q(√3) member with B = C = 120°:
//!
//! * angles: B = C = 120°, tan A = 3√3 (A ≈ 79.11°), cos D = 1/7
//!   (D ≈ 81.79°), tan E = −√3/2 (E ≈ 139.11°), with A = E − 60°;
//! * sides (shipped scale): a = 6√7, b = c = d = e = 14;
//! * tile area: 189·√3.
//!
//! The tiling is **2-isohedral**: the two full-group orbits are
//! {T1, T4, T5, T8} (seed 0) and {T2, T3, T6, T7} (seed 1); each orbit
//! mixes both chiralities, the glides doing the identifying — the cell
//! holds four direct and four mirror tiles.  The uncoloured wallpaper
//! group is **pgg**: two perpendicular glide families with axes along
//! the lattice directions, whose squares are the primitive
//! translations — `g1² = w1` and `g2² = −w2` — and no mirrors (each
//! glide shifts by exactly half a primitive translation along its
//! axis, so no conjugate of it is a pure reflection).  The half-turns
//! are centred at c0 = (−1/2, 25√3/2) and its images; c0, the glide
//! matrices and shifts below are all exact Q(√3).
//!
//! The tiling is edge-to-edge at the pentagon level: every side of
//! every tile is matched by exactly one full side of one neighbour,
//! and all 20 torus pairs are reverse-glued (side pairs a-a×4, b-c×4,
//! b-d×4, c-c×2, d-e×4, e-e×2).  The three vertex stars are {A,A,C,D}
//! (2·79.11° + 120° + 81.79°), {B,B,C} (3·120°) and {D,E,E}
//! (81.79° + 2·139.11°) — each 360° — in equal proportions (12 vertex
//! orbits: eight of size 3, four of size 4).  The cell table was
//! extracted from a verified exact-Q(√3) construction (M73): zero
//! overlaps, exactly-once coverage, complete full-edge gluing with the
//! three closed vertex stars, and an exact symmetry audit of the
//! quotient table (the half-turn and both glide families are
//! automorphisms with exact Q(√3) representatives — the mirrors are
//! not).
//!
//! Primitive cell: **8 tiles** on the rectangular lattice
//! `w1 = (36, 24√3)`, `w2 = (−36, 18√3)` with `w1 ⊥ w2` and
//! `|det(w1, w2)| = 1512·√3 = 8 × tile area`.

use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// √3 — every coordinate in this module is a Q(√3) surd built from it
/// (computed at runtime; no decimal approximations are shipped).
pub(crate) fn s3() -> f64 {
    3.0f64.sqrt()
}

#[cfg(test)]
/// Exact tile area, 189·√3.
pub(crate) fn tile_area() -> f64 {
    189.0 * s3()
}

/// Pentagon type 8 (Kershner 1968): 2-isohedral, 8 tiles per rectangular
/// cell, wallpaper group pgg.
pub fn spec() -> LatticeTiling {
    let s3 = s3();
    // (p + q·√3) — every tile vertex lies on the Z[√3] grid at the
    // shipped scale (integer x, integer·√3 y).
    let v = |px: f64, qx: f64, py: f64, qy: f64| Vec2::new(px + qx * s3, py + qy * s3);
    // Eight pentagon cells in CCW order, construction coordinates (the
    // shipped-scale prototile T1 has its corner A at the origin).  The
    // seeds follow the two full-group isohedral orbits
    // {T1,T4,T5,T8} / {T2,T3,T6,T7}; T2, T3, T5, T8 are mirror
    // placements (stored CCW with their vertex order reversed).
    let t1 = Polygon::from_slice(&[
        v(0.0, 0.0, 0.0, 0.0),
        v(14.0, 0.0, 0.0, 0.0),
        v(21.0, 0.0, 0.0, 7.0),
        v(14.0, 0.0, 0.0, 14.0),
        v(3.0, 0.0, 0.0, 9.0),
    ]);
    let t2 = Polygon::from_slice(&[
        v(3.0, 0.0, 0.0, 9.0),
        v(-4.0, 0.0, 0.0, 16.0),
        v(-15.0, 0.0, 0.0, 11.0),
        v(-13.0, 0.0, 0.0, 3.0),
        v(0.0, 0.0, 0.0, 0.0),
    ]);
    let t3 = Polygon::from_slice(&[
        v(-4.0, 0.0, 0.0, 16.0),
        v(3.0, 0.0, 0.0, 9.0),
        v(14.0, 0.0, 0.0, 14.0),
        v(12.0, 0.0, 0.0, 22.0),
        v(-1.0, 0.0, 0.0, 25.0),
    ]);
    let t4 = Polygon::from_slice(&[
        v(-1.0, 0.0, 0.0, 25.0),
        v(-15.0, 0.0, 0.0, 25.0),
        v(-22.0, 0.0, 0.0, 18.0),
        v(-15.0, 0.0, 0.0, 11.0),
        v(-4.0, 0.0, 0.0, 16.0),
    ]);
    let t5 = Polygon::from_slice(&[
        v(29.0, 0.0, 0.0, 17.0),
        v(36.0, 0.0, 0.0, 24.0),
        v(23.0, 0.0, 0.0, 27.0),
        v(12.0, 0.0, 0.0, 22.0),
        v(14.0, 0.0, 0.0, 14.0),
    ]);
    let t6 = Polygon::from_slice(&[
        v(14.0, 0.0, 0.0, 14.0),
        v(21.0, 0.0, 0.0, 7.0),
        v(35.0, 0.0, 0.0, 7.0),
        v(42.0, 0.0, 0.0, 14.0),
        v(29.0, 0.0, 0.0, 17.0),
    ]);
    let t7 = Polygon::from_slice(&[
        v(57.0, 0.0, 0.0, 17.0),
        v(50.0, 0.0, 0.0, 24.0),
        v(36.0, 0.0, 0.0, 24.0),
        v(29.0, 0.0, 0.0, 17.0),
        v(42.0, 0.0, 0.0, 14.0),
    ]);
    let t8 = Polygon::from_slice(&[
        v(42.0, 0.0, 0.0, 14.0),
        v(35.0, 0.0, 0.0, 7.0),
        v(48.0, 0.0, 0.0, 4.0),
        v(59.0, 0.0, 0.0, 9.0),
        v(57.0, 0.0, 0.0, 17.0),
    ]);
    LatticeTiling {
        id: "pentagon-8",
        name: "Pentagon type 8 (Kershner)",
        description: "Pentagonal tiling of type 8 (Richard Kershner, 1968): B = C = 120°, tan A = 3√3, cos D = 1/7, tan E = −√3/2 (A = E − 60°), a = 6√7, b = c = d = e = 14; edge-to-edge (every side is matched by exactly one full partner side, all reverse-glued), 2-isohedral (seeds 0/1), 8 tiles (four direct, four mirror) per rectangular cell, wallpaper group pgg (two perpendicular glide families with axes along the lattice directions, squaring to w1 and −w2, no mirrors).",
        lattice: [Vec2::new(36.0, 24.0 * s3), Vec2::new(-36.0, 18.0 * s3)],
        cells: vec![
            Cell { poly: t1, seed: 0 },
            Cell { poly: t2, seed: 1 },
            Cell { poly: t3, seed: 1 },
            Cell { poly: t4, seed: 0 },
            Cell { poly: t5, seed: 0 },
            Cell { poly: t6, seed: 1 },
            Cell { poly: t7, seed: 1 },
            Cell { poly: t8, seed: 0 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::Pgg,
        lattice_kind: LatticeKind::Rectangular,
        category: TilingCategory::PentagonTypes,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
