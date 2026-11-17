//! Pentagon type 13 (Rice, 1977) tiling — canonical A = C = D = 120° instance.
//!
//! Rice's type-13 pentagon satisfies `d = 2a = 2e`, `B = E = 90°` and
//! `2A + D = 360°` (which forces `C = A`); with b = 1 and A = 120° all
//! geometry lies in Q(√3):
//!
//! * sides: `a = e = (3√3+1)/13`, `b = 1`, `c = (4−√3)/13`,
//!   `d = (6√3+2)/13 = 2a`;
//! * angles: A = C = D = 120°, B = E = 90°;
//! * tile area: `9(14+3√3)/338`.
//!
//! The tiling is **2-isohedral**: the two isohedral classes are the two
//! chiralities of the scalene prototile, shipped as the two pattern seeds
//! (0 = the CCW class, 1 = its mirror). The uncoloured pattern has
//! wallpaper group pgg (two perpendicular glide families, no mirrors);
//! treating the chiral pair as distinct reduces it to **p2**, the group
//! shipped here.
//!
//! Not edge-to-edge at the pentagon level: the long side d is split into
//! two collinear a-edges (midpoint vertex `M`), so every joint in the
//! refined 6-vertex figure is a full fine-edge against a full
//! fine-edge (a = e = d/2).
//!
//! Primitive cell: **8 tiles** on the rectangular lattice
//! `T1 = (3s, 3s)`, `T2 = (s, −s)` with `s = (9+√3)/13`
//! (`T1 ⊥ T2`, `|T1| = 3|T2|`; `|det| = 6s² = 8 × tile area`). All 48
//! tile vertices lie on the exact `(p + q√3)/26` grid; the cell table
//! below was extracted from a verified exact-Q(√3) forced-growth unit
//! (zero overlaps, exactly-once coverage, 360° vertex figures).

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
/// Prototile side a = e = (3√3+1)/13 (b = 1).
pub(crate) fn side_a() -> f64 {
    (3.0 * s3() + 1.0) / 13.0
}

#[cfg(test)]
/// Prototile side c = (4−√3)/13.
pub(crate) fn side_c() -> f64 {
    (4.0 - s3()) / 13.0
}

#[cfg(test)]
/// Prototile side d = (6√3+2)/13 = 2a.
pub(crate) fn side_d() -> f64 {
    (6.0 * s3() + 2.0) / 13.0
}

/// Lattice parameter s = a + 2c = (9+√3)/13.
pub(crate) fn s_lat() -> f64 {
    (9.0 + s3()) / 13.0
}

#[cfg(test)]
/// Exact tile area, 9(14+3√3)/338.
pub(crate) fn tile_area() -> f64 {
    9.0 * (14.0 + 3.0 * s3()) / 338.0
}

/// Pentagon type 13 (Rice 1977), canonical A = 120° instance: 2-isohedral,
/// 8 tiles per primitive cell, wallpaper group p2.
pub fn spec() -> LatticeTiling {
    let s3 = s3();
    let s = s_lat();
    // (p + q·√3)/26 — every tile vertex lies on this exact grid.
    let v = |px: f64, qx: f64, py: f64, qy: f64| {
        Vec2::new((px + qx * s3) / 26.0, (py + qy * s3) / 26.0)
    };
    // Eight six-vertex cells in (A, B, C, M, D, E) order — M is the
    // midpoint of the long side d = CD, splitting it into two collinear
    // a-edges. Seeds 0/1 mark the two chiral (isohedral) classes; the
    // CCW class (L0, L1, Y1, Y0) first, so the prototile is L0.
    let l0 = Polygon::from_slice(&[
        v(0.0, 0.0, 0.0, 0.0),
        v(26.0, 0.0, 0.0, 0.0),
        v(26.0, 0.0, 8.0, -2.0),
        v(17.0, -1.0, 9.0, 1.0),
        v(8.0, -2.0, 10.0, 4.0),
        v(-1.0, -3.0, 9.0, 1.0),
    ]);
    let l1 = Polygon::from_slice(&[
        v(34.0, -2.0, 18.0, 2.0),
        v(8.0, -2.0, 18.0, 2.0),
        v(8.0, -2.0, 10.0, 4.0),
        v(17.0, -1.0, 9.0, 1.0),
        v(26.0, 0.0, 8.0, -2.0),
        v(35.0, 1.0, 9.0, 1.0),
    ]);
    let y1 = Polygon::from_slice(&[
        v(44.0, 2.0, 10.0, 4.0),
        v(44.0, 2.0, 36.0, 4.0),
        v(36.0, 4.0, 36.0, 4.0),
        v(35.0, 1.0, 27.0, 3.0),
        v(34.0, -2.0, 18.0, 2.0),
        v(35.0, 1.0, 9.0, 1.0),
    ]);
    let y0 = Polygon::from_slice(&[
        v(62.0, 4.0, 44.0, 2.0),
        v(62.0, 4.0, 18.0, 2.0),
        v(70.0, 2.0, 18.0, 2.0),
        v(71.0, 5.0, 27.0, 3.0),
        v(72.0, 8.0, 36.0, 4.0),
        v(71.0, 5.0, 45.0, 5.0),
    ]);
    let g0 = Polygon::from_slice(&[
        v(0.0, 0.0, 0.0, 0.0),
        v(26.0, 0.0, 0.0, 0.0),
        v(26.0, 0.0, -8.0, 2.0),
        v(17.0, -1.0, -9.0, -1.0),
        v(8.0, -2.0, -10.0, -4.0),
        v(-1.0, -3.0, -9.0, -1.0),
    ]);
    let g1 = Polygon::from_slice(&[
        v(34.0, -2.0, 18.0, 2.0),
        v(8.0, -2.0, 18.0, 2.0),
        v(8.0, -2.0, 26.0, 0.0),
        v(17.0, -1.0, 27.0, 3.0),
        v(26.0, 0.0, 28.0, 6.0),
        v(35.0, 1.0, 27.0, 3.0),
    ]);
    let d0 = Polygon::from_slice(&[
        v(44.0, 2.0, 10.0, 4.0),
        v(44.0, 2.0, 36.0, 4.0),
        v(52.0, 0.0, 36.0, 4.0),
        v(53.0, 3.0, 27.0, 3.0),
        v(54.0, 6.0, 18.0, 2.0),
        v(53.0, 3.0, 9.0, 1.0),
    ]);
    let d1 = Polygon::from_slice(&[
        v(62.0, 4.0, 44.0, 2.0),
        v(62.0, 4.0, 18.0, 2.0),
        v(54.0, 6.0, 18.0, 2.0),
        v(53.0, 3.0, 27.0, 3.0),
        v(52.0, 0.0, 36.0, 4.0),
        v(53.0, 3.0, 45.0, 5.0),
    ]);
    LatticeTiling {
        id: "pentagon-13",
        name: "Pentagon type 13 (Rice, 1977)",
        description: "Pentagonal tiling of type 13 (Rice, 1977), canonical A = 120°: d = 2a = 2e, B = E = 90°, 2A + D = 360°; 2-isohedral chiral pair (seeds 0/1), 8 tiles per cell, long side split into two collinear a-edges.",
        lattice: [Vec2::new(3.0 * s, 3.0 * s), Vec2::new(s, -s)],
        cells: vec![
            Cell { poly: l0, seed: 0 },
            Cell { poly: l1, seed: 0 },
            Cell { poly: y1, seed: 0 },
            Cell { poly: y0, seed: 0 },
            Cell { poly: g0, seed: 1 },
            Cell { poly: g1, seed: 1 },
            Cell { poly: d0, seed: 1 },
            Cell { poly: d1, seed: 1 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P2,
        lattice_kind: LatticeKind::Rectangular,
        category: TilingCategory::Wallpaper,
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
