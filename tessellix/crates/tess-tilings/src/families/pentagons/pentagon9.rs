//! Pentagon type 9 (Marjorie Rice, 1976) tiling — 2-isohedral, edge-to-edge,
//! wallpaper group pgg.
//!
//! Type 9 is one of the four families Rice discovered in 1976–77.  It is a
//! two-parameter family with conditions `b = c = d = e` and
//! `2A + C = D + 2E = 360°`; the instance shipped here is the exact
//! Q(√3) member with A = C = 120° (computed at runtime; no decimal
//! approximations are shipped):
//!
//! * angles: A = C = 120°, cos B = 1/7 (B ≈ 81.79°),
//!   cos D = 23/98 (D ≈ 76.43°), cos E = −11/14 (E ≈ 141.79°);
//! * sides (shipped scale): a = 6, b = c = d = e = 14;
//! * tile area: 141·√3.
//!
//! The tiling is **2-isohedral**: the two full-group orbits are
//! {T1, T4, T5, T8} (seed 0) and {T2, T3, T6, T7} (seed 1); the glides
//! identify the two chiralities inside each orbit.  The uncoloured
//! wallpaper group is **pgg**: two perpendicular glide families with axes
//! along the lattice directions, whose squares are the primitive
//! translations — `g1² = w2` and `g2² = −w1` — and no mirrors (each glide
//! shifts by exactly half a primitive translation along its axis, so no
//! conjugate of it is a pure reflection).  The half-turns are centred at
//! c0 = (13, 4√3) and its images; c0, the glide matrices and shifts below
//! are all exact Q(√3).
//!
//! The tiling is edge-to-edge at the pentagon level: every side of every
//! tile is matched by exactly one full side of one neighbour.  The three
//! vertex stars are {A,A,C}, {D,E,E} and {B,B,C,D} (120°+120°+120°,
//! 76.43°+2·141.79°, 2·81.79°+120°+76.43° — each 360°), in equal
//! proportions.  The cell table was extracted from a verified
//! exact-Q(√3) construction: zero overlaps, exactly-once coverage of a
//! 81×71-point sample, complete full-edge gluing with the three closed
//! vertex stars, and an exact symmetry audit of the quotient table (the
//! half-turn and both glide families are automorphisms with exact
//! Q(√3) representatives — the mirrors are not).
//!
//! Primitive cell: **8 tiles** on the rectangular lattice
//! `w1 = (−30, 6√3)`, `w2 = (−141/7, −235√3/7)` with `w1 ⊥ w2` and
//! `|det(w1, w2)| = 1128·√3 = 8 × tile area`.

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
/// Exact tile area, 141·√3.
pub(crate) fn tile_area() -> f64 {
    141.0 * s3()
}

/// Pentagon type 9 (Rice 1976): 2-isohedral, 8 tiles per rectangular
/// cell, wallpaper group pgg.
pub fn spec() -> LatticeTiling {
    let s3 = s3();
    // (p + q·√3) — every tile vertex lies on the (Z + Z√3)/7 grid.
    let v = |px: f64, qx: f64, py: f64, qy: f64| Vec2::new(px + qx * s3, py + qy * s3);
    // Eight pentagon cells in CCW order, construction coordinates (the
    // shipped-scale prototile T1 has its corner A at the origin).  The
    // seeds follow the two full-group isohedral orbits
    // {T1,T4,T5,T8} / {T2,T3,T6,T7}.
    let t1 = Polygon::from_slice(&[
        v(0.0, 0.0, 0.0, 0.0),
        v(14.0, 0.0, 0.0, 0.0),
        v(12.0, 0.0, 0.0, 8.0),
        v(-1.0, 0.0, 0.0, 11.0),
        v(-3.0, 0.0, 0.0, 3.0),
    ]);
    let t2 = Polygon::from_slice(&[
        v(29.0, 0.0, 0.0, 5.0),
        v(42.0, 0.0, 0.0, 2.0),
        v(44.0, 0.0, 0.0, 10.0),
        v(33.0, 0.0, 0.0, 15.0),
        v(26.0, 0.0, 0.0, 8.0),
    ]);
    let t3 = Polygon::from_slice(&[
        v(12.0, 0.0, 0.0, -8.0),
        v(14.0, 0.0, 0.0, 0.0),
        v(0.0, 0.0, 0.0, 0.0),
        v(-7.0, 0.0, 0.0, -7.0),
        v(45.0 / 7.0, 0.0, 0.0, -65.0 / 7.0),
    ]);
    let t4 = Polygon::from_slice(&[
        v(206.0 / 7.0, 0.0, 0.0, -156.0 / 7.0),
        v(255.0 / 7.0, 0.0, 0.0, -107.0 / 7.0),
        v(23.0, 0.0, 0.0, -13.0),
        v(90.0 / 7.0, 0.0, 0.0, -130.0 / 7.0),
        v(167.0 / 7.0, 0.0, 0.0, -165.0 / 7.0),
    ]);
    let t5 = Polygon::from_slice(&[
        v(26.0, 0.0, 0.0, 8.0),
        v(12.0, 0.0, 0.0, 8.0),
        v(14.0, 0.0, 0.0, 0.0),
        v(27.0, 0.0, 0.0, -3.0),
        v(29.0, 0.0, 0.0, 5.0),
    ]);
    let t6 = Polygon::from_slice(&[
        v(27.0, 0.0, 0.0, -3.0),
        v(14.0, 0.0, 0.0, 0.0),
        v(12.0, 0.0, 0.0, -8.0),
        v(23.0, 0.0, 0.0, -13.0),
        v(30.0, 0.0, 0.0, -6.0),
    ]);
    let t7 = Polygon::from_slice(&[
        v(44.0, 0.0, 0.0, 10.0),
        v(42.0, 0.0, 0.0, 2.0),
        v(56.0, 0.0, 0.0, 2.0),
        v(63.0, 0.0, 0.0, 9.0),
        v(347.0 / 7.0, 0.0, 0.0, 79.0 / 7.0),
    ]);
    let t8 = Polygon::from_slice(&[
        v(45.0 / 7.0, 0.0, 0.0, -65.0 / 7.0),
        v(-4.0 / 7.0, 0.0, 0.0, -114.0 / 7.0),
        v(90.0 / 7.0, 0.0, 0.0, -130.0 / 7.0),
        v(23.0, 0.0, 0.0, -13.0),
        v(12.0, 0.0, 0.0, -8.0),
    ]);
    LatticeTiling {
        id: "pentagon-9",
        name: "Pentagon type 9 (Rice)",
        description: "Pentagonal tiling of type 9 (Marjorie Rice, 1976): A = C = 120°, cos B = 1/7, cos D = 23/98, cos E = −11/14, a = 6, b = c = d = e = 14; edge-to-edge, 2-isohedral (seeds 0/1), 8 tiles per rectangular cell, wallpaper group pgg (two perpendicular glide families squaring to the primitive translations, no mirrors).",
        lattice: [Vec2::new(-30.0, 6.0 * s3), Vec2::new(-141.0 / 7.0, -235.0 / 7.0 * s3)],
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
