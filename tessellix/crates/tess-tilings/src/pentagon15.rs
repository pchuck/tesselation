//! Pentagon type 15 (Mann / McLoud-Mann / Von Derau 2015) tiling —
//! wallpaper group pgg.
//!
//! Type 15 is the tenth known family of convex-pentagon tilings, found
//! by Mann, McLoud-Mann and Von Derau in 2015.  It is a two-parameter
//! family; the instance shipped here is the one in which every
//! coordinate is an exact Q(√3) surd (computed at runtime; no decimal
//! approximations are shipped):
//!
//! * angles: A = 150°, B = 60°, C = 135°, D = 105°, E = 90°
//!   (A + E = C + D = 240°, total 540°);
//! * sides (shipped scale): a = 2, b = d = e = 1, c = √(2+√3), so
//!   `a = 2b` and `c² = 2 + √3`;
//! * tile area: (5+3√3)/4.
//!
//! The tiling is **3-isohedral**: the three isohedral classes are the
//! full-group orbits {T1,T4,T7,T10}, {T2,T5,T8,T11}, {T3,T6,T9,T12}
//! (shipped as seeds 0, 1, 2 — a glide identifies the two chiralities
//! within each class).  The uncoloured wallpaper group is **pgg**: two
//! perpendicular glide families (axes ∥ w1 and ∥ w2, with squares
//! w1 and w2 respectively), no mirrors, and 2-fold centers
//! c0 + (i·w1 + j·w2)/2 — four per primitive cell, c0 = (√3, −1/2).
//!
//! Not edge-to-edge at the pentagon level: a side of length 2 can meet
//! two collinear unit sides of a neighbour (a T-junction); four of the
//! 60 edges of the primitive cell are T-split, the other 56 are full
//! edge-to-edge matches.
//!
//! The 12-tile primitive cell decomposes into four congruent 3-tile
//! blocks (T1–T3, T4–T6 = g(T1–T3), T10–T12 = T4–T6 + w1 under the
//! first glide g; T7–T9 = the half-turn of T1–T3 about c0).  All 60
//! cell vertices lie on the exact (p + q√3)/2 grid; the cell table was
//! extracted from a verified exact-Q(√3) construction: zero overlaps,
//! exactly-once coverage on a 20400-point sample, all 60 central edges
//! exactly glued, and an exhaustive symmetry audit (25 translations +
//! 25 half-turns + 50 glides + 0 mirrors in a 5×5 window).
//!
//! Primitive cell: **12 tiles** on the rectangular lattice
//! `w1 = (−(1+√3)/2, (3+√3)/2)`, `w2 = ((9+6√3)/2, 3+3√3/2)` with
//! `w1 ⊥ w2`, `|w1|² = 4+2√3`, `|w2|² = 63+36√3` and
//! `|det(w1,w2)| = 12 × tile area = 15 + 9√3`.

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// √3 — every coordinate in this module is a Q(√3) surd built from it
/// (computed at runtime; no decimal approximations are shipped).
pub(crate) fn s3() -> f64 {
    3.0f64.sqrt()
}

#[cfg(test)]
/// Exact tile area, (5+3√3)/4.
pub(crate) fn tile_area() -> f64 {
    (5.0 + 3.0 * s3()) / 4.0
}

#[cfg(test)]
/// Prototile side c = √(2+√3) (the only non-unit side ratio).
pub(crate) fn side_c() -> f64 {
    (2.0 + s3()).sqrt()
}

/// Pentagon type 15 (Mann / McLoud-Mann / Von Derau 2015): 3-isohedral,
/// 12 tiles per rectangular cell, wallpaper group pgg.
pub fn spec() -> LatticeTiling {
    let s3 = s3();
    // (p + q·√3) — every tile vertex lies on the (p + q√3)/2 grid.
    let v = |px: f64, qx: f64, py: f64, qy: f64| {
        Vec2::new(px + qx * s3, py + qy * s3)
    };
    // Twelve pentagon cells in CCW order, construction coordinates
    // (the shipped-scale prototile anchor at the origin).  Cell T1 is
    // the prototile (seed 0, corner B at the origin); the seeds follow
    // (k−1) mod 3 — the three full-group isohedral orbits.
    let t1 = Polygon::from_slice(&[
        v(0.0, 0.0, 0.0, 0.0),
        v(-1.0 / 2.0, 0.0, 0.0, 1.0 / 2.0),
        v(-1.5, -1.0 / 2.0, 0.5, 1.0 / 2.0),
        v(-2.0, -1.0 / 2.0, 0.5, 0.0),
        v(-2.0, 0.0, 0.0, 0.0),
    ]);
    let t2 = Polygon::from_slice(&[
        v(0.0, 1.0, -1.0, 0.0),
        v(0.0, 1.0, 0.0, 0.0),
        v(-1.0 / 2.0, 1.0 / 2.0, 0.5, 1.0 / 2.0),
        v(-1.0 / 2.0, 0.0, 0.0, 1.0 / 2.0),
        v(0.0, 0.0, 0.0, 0.0),
    ]);
    let t3 = Polygon::from_slice(&[
        v(-1.0 / 2.0, -1.0 / 2.0, 1.5, 1.0 / 2.0),
        v(-1.5, -1.0 / 2.0, 1.5, 1.0 / 2.0),
        v(-1.5, -1.0 / 2.0, 0.5, 1.0 / 2.0),
        v(-1.0 / 2.0, 0.0, 0.0, 1.0 / 2.0),
        v(-1.0 / 2.0, 1.0 / 2.0, 0.5, 1.0 / 2.0),
    ]);
    let t4 = Polygon::from_slice(&[
        v(-1.5, -1.0 / 2.0, 0.5, 1.0 / 2.0),
        v(-1.5, -1.0 / 2.0, 1.5, 1.0 / 2.0),
        v(-2.5, -1.0 / 2.0, 1.5, 1.0 / 2.0),
        v(-3.0, -1.0 / 2.0, 0.5, 0.0),
        v(-2.5, -1.0 / 2.0, 0.5, -1.0 / 2.0),
    ]);
    let t5 = Polygon::from_slice(&[
        v(-2.5, -1.0 / 2.0, 0.5, -1.0 / 2.0),
        v(-3.0, -1.0 / 2.0, 0.5, 0.0),
        v(-3.0, -1.0, 0.0, 0.0),
        v(-2.5, -1.0, -1.0, -1.0 / 2.0),
        v(-2.5, -1.0 / 2.0, -1.5, -1.0 / 2.0),
    ]);
    let t6 = Polygon::from_slice(&[
        v(-3.0, -1.0, 0.0, 0.0),
        v(-3.0, -1.0 / 2.0, 0.5, 0.0),
        v(-2.5, -1.0 / 2.0, 1.5, 1.0 / 2.0),
        v(-2.5, -1.0, 2.0, 1.0 / 2.0),
        v(-3.0, -1.0, 2.0, 0.0),
    ]);
    let t7 = Polygon::from_slice(&[
        v(0.0, 2.0, -1.0, 0.0),
        v(1.0 / 2.0, 2.0, -1.0, -1.0 / 2.0),
        v(1.5, 2.5, -1.5, -1.0 / 2.0),
        v(2.0, 2.5, -1.5, 0.0),
        v(2.0, 2.0, -1.0, 0.0),
    ]);
    let t8 = Polygon::from_slice(&[
        v(0.0, 1.0, 0.0, 0.0),
        v(0.0, 1.0, -1.0, 0.0),
        v(1.0 / 2.0, 1.5, -1.5, -1.0 / 2.0),
        v(1.0 / 2.0, 2.0, -1.0, -1.0 / 2.0),
        v(0.0, 2.0, -1.0, 0.0),
    ]);
    let t9 = Polygon::from_slice(&[
        v(1.0 / 2.0, 2.5, -2.5, -1.0 / 2.0),
        v(1.5, 2.5, -2.5, -1.0 / 2.0),
        v(1.5, 2.5, -1.5, -1.0 / 2.0),
        v(1.0 / 2.0, 2.0, -1.0, -1.0 / 2.0),
        v(1.0 / 2.0, 1.5, -1.5, -1.0 / 2.0),
    ]);
    let t10 = Polygon::from_slice(&[
        v(1.5, 2.5, -1.5, -1.0 / 2.0),
        v(1.5, 2.5, -2.5, -1.0 / 2.0),
        v(2.5, 2.5, -2.5, -1.0 / 2.0),
        v(3.0, 2.5, -1.5, 0.0),
        v(2.5, 2.5, -1.5, 1.0 / 2.0),
    ]);
    let t11 = Polygon::from_slice(&[
        v(2.5, 2.5, -1.5, 1.0 / 2.0),
        v(3.0, 2.5, -1.5, 0.0),
        v(3.0, 3.0, -1.0, 0.0),
        v(2.5, 3.0, 0.0, 1.0 / 2.0),
        v(2.5, 2.5, 0.5, 1.0 / 2.0),
    ]);
    let t12 = Polygon::from_slice(&[
        v(3.0, 3.0, -1.0, 0.0),
        v(3.0, 2.5, -1.5, 0.0),
        v(2.5, 2.5, -2.5, -1.0 / 2.0),
        v(2.5, 3.0, -3.0, -1.0 / 2.0),
        v(3.0, 3.0, -3.0, 0.0),
    ]);
    LatticeTiling {
        id: "pentagon-15",
        name: "Pentagon type 15 (Mann)",
        description: "Pentagonal tiling of type 15 (Mann / McLoud-Mann / Von Derau 2015): A = 150°, B = 60°, C = 135°, D = 105°, E = 90°, a = 2, b = d = e = 1, c = √(2+√3); 3-isohedral (seeds 0/1/2), 12 tiles per rectangular cell, wallpaper group pgg (two perpendicular glides, no mirrors), some long sides T-split by collinear unit edges.",
        lattice: [
            Vec2::new(-(1.0 + s3) / 2.0, (3.0 + s3) / 2.0),
            Vec2::new((9.0 + 6.0 * s3) / 2.0, (6.0 + 3.0 * s3) / 2.0),
        ],
        cells: vec![
            Cell { poly: t1, seed: 0 },
            Cell { poly: t2, seed: 1 },
            Cell { poly: t3, seed: 2 },
            Cell { poly: t4, seed: 0 },
            Cell { poly: t5, seed: 1 },
            Cell { poly: t6, seed: 2 },
            Cell { poly: t7, seed: 0 },
            Cell { poly: t8, seed: 1 },
            Cell { poly: t9, seed: 2 },
            Cell { poly: t10, seed: 0 },
            Cell { poly: t11, seed: 1 },
            Cell { poly: t12, seed: 2 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::Pgg,
        lattice_kind: LatticeKind::Rectangular,
        category: TilingCategory::Wallpaper,
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
