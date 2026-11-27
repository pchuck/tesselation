//! Pentagon type 14 (Stein 1985) tiling — 3-isohedral, wallpaper group p2.
//!
//! Stein's type-14 prototile is the completely determined pentagon with
//! side constraints `2a = 2c = d = e` (i.e. `a = c = 1`, `d = e = 2`) and
//! `b = βa`, where β = √((11√57−25)/8); its angles are `A = 90°`,
//! `B ≈ 145.34°`, `C ≈ 69.32°`, `D ≈ 124.66°`, `E ≈ 110.68°`
//! (`2B + C = 360°`, `C + E = 180°`) — no free parameters.
//!
//! All coordinates in this module are exact surds in Q(√57, β, p, bq)
//! (computed at runtime; no decimal approximations are shipped):
//!
//! * `s57 = √57`, `β = √((11√57−25)/8)`, `q = (√57−3)/8 = sin B`,
//!   `p = √((6√57−2)/64) = sin(B − 90°)`;
//! * `βp = (43−√57)/16` lies in Q(√57) exactly;
//!   `βq = √((438√57−2706)/256)`;
//! * tile area: `β(3√57−1)/16 + p(√57−5)/4`.
//!
//! The tiling is **3-isohedral** (three isohedral classes: the two
//! D-class poses, the two M-class poses, and the two salmon poses) with
//! wallpaper group **p2** (orbifold 2222). The 2-fold centre
//! `O = (β + p/2, q/2)` maps each class onto itself, so the group stays
//! p2 with the classes distinguished (seeds 0, 1, 2). The p2 label is
//! exact, not a colouring convention: the translation lattice is a
//! generic oblique lattice (`w1·w2 ≠ 0`, `|w1| ≠ |w2|`), so its
//! normaliser in O(2) is {I, −I} and no mirror or glide can normalise
//! the translations at all — pinned by
//! `pentagon_14_lattice_rules_out_pg_glide_groups` (the "type 14 is
//! pgg" claim would require a rectangular lattice; the pgg-family
//! Rice types are 9–13, and Stein's tiling is unique).
//!
//! Not edge-to-edge at the pentagon level: every long side (length 2) is
//! split into two collinear unit fine edges, giving 7-vertex D-class
//! cells, 5-vertex M-class cells and 6-vertex salmon cells. The shipped
//! cell table was verified by a 21600-point exactly-once overlay
//! (40×40 grid + 20000 random points over a ±5×±6 lattice window).
//!
//! Primitive cell: **6 tiles** on the oblique lattice
//! `w1 = (2pq, −(13+9√57)/16)`, `w2 = (−(β+2p), −(√57−1)/2)` with
//! `|det| = 6 × tile area` and `w1 · w2 ≠ 0`.

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// √57 — every coordinate in this module is built from this, β, p and
/// bq (computed at runtime; no decimal approximations are shipped).
pub(crate) fn s57() -> f64 {
    57.0f64.sqrt()
}

/// Side ratio b/a = √((11√57−25)/8).
pub(crate) fn beta() -> f64 {
    ((11.0 * s57() - 25.0) / 8.0).sqrt()
}

/// sin(B − 90°) = √((6√57−2)/64) (the rotation that carries the
/// M-class mirror prototile onto the salmon poses).
pub(crate) fn p() -> f64 {
    ((6.0 * s57() - 2.0) / 64.0).sqrt()
}

/// sin B = (√57−3)/8.
pub(crate) fn q() -> f64 {
    (s57() - 3.0) / 8.0
}

/// p·q — the x-component of the first lattice vector w1.
pub(crate) fn pq() -> f64 {
    p() * q()
}

/// βp = (43−√57)/16 — holds in Q(√57) exactly (no nested surd).
pub(crate) fn bp() -> f64 {
    (43.0 - s57()) / 16.0
}

/// βq = √((438√57−2706)/256).
pub(crate) fn bq() -> f64 {
    ((438.0 * s57() - 2706.0) / 256.0).sqrt()
}

/// First oblique lattice vector w1 = (2pq, −(13+9√57)/16).
pub(crate) fn w1() -> Vec2 {
    Vec2::new(2.0 * pq(), -(13.0 + 9.0 * s57()) / 16.0)
}

/// Second oblique lattice vector w2 = (−(β+2p), −(√57−1)/2).
pub(crate) fn w2() -> Vec2 {
    Vec2::new(-(beta() + 2.0 * p()), -(s57() - 1.0) / 2.0)
}

#[cfg(test)]
/// Exact tile area: β(3√57−1)/16 + p(√57−5)/4.
pub(crate) fn tile_area() -> f64 {
    let s57 = s57();
    let p = p();
    beta() * (3.0 * s57 - 1.0) / 16.0 + p * (s57 - 5.0) / 4.0
}

/// Pentagon type 14 (Stein 1985): 3-isohedral, 6 tiles per primitive
/// oblique cell, wallpaper group p2.
pub fn spec() -> LatticeTiling {
    let b = beta();
    let p = p();
    let q = q();
    let pq = pq();
    let bp = bp();
    let bq = bq();
    // D0 (7 vertices, seed 0): corners A, B, C, D, E plus the midpoint
    // of the long side C–D (Md) and of D–E (Me), in CCW order
    // (A, B, C, Md, D, Me, E).
    let d0 = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(b, 0.0),
        Vec2::new(b + p, q),
        Vec2::new(b, 2.0 * q),
        Vec2::new(b - p, 3.0 * q),
        Vec2::new((b - p) / 2.0, (1.0 + 3.0 * q) / 2.0),
        Vec2::new(0.0, 1.0),
    ]);
    // 2-fold centre O: the d180/m180 cells are d0/m0 rotated 180° about
    // O; o4 = 2O = (2β + p, q) is the A-corner of the rotated pair.
    let o4 = Vec2::new(2.0 * b + p, q);
    let d180 = Polygon::from_slice(&[
        o4,
        o4 + Vec2::new(-b, 0.0),
        o4 + Vec2::new(-b - p, -q),
        o4 + Vec2::new(-b, -2.0 * q),
        o4 + Vec2::new(p - b, -3.0 * q),
        o4 + Vec2::new((p - b) / 2.0, -(1.0 + 3.0 * q) / 2.0),
        o4 + Vec2::new(0.0, -1.0),
    ]);
    // M0 (5 vertices, seed 1): corners in CCW order A, E, D, C, B.
    let m0 = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, -1.0),
        Vec2::new(b - p, -3.0 * q),
        Vec2::new(b + p, -q),
        Vec2::new(b, 0.0),
    ]);
    let m180 = Polygon::from_slice(&[
        o4,
        o4 + Vec2::new(0.0, 1.0),
        o4 + Vec2::new(p - b, 3.0 * q),
        o4 + Vec2::new(-b - p, q),
        o4 + Vec2::new(-b, 0.0),
    ]);
    // Salmon shared L_s (vertices E', D', Md', C', B', A'): the
    // M-class mirror prototile rotated by B − 90° (cos = q, sin = p);
    // every coordinate simplifies to a Q(√57, β, p, bq) surd.
    let ls: [Vec2; 6] = [
        Vec2::new(p, -q),
        Vec2::new(bq + 2.0 * pq, bp - p * p - 3.0 * q * q),
        Vec2::new(bq + 2.0 * pq, bp - 2.0 * q * q),
        Vec2::new(bq + 2.0 * pq, bp + p * p - q * q),
        Vec2::new(bq, bp),
        Vec2::ZERO,
    ];
    let o12 = Vec2::new(-3.0 * p, -(1.0 + q));
    let o17 = Vec2::new(b + 2.0 * p, -2.0 * q);
    let mths = Polygon::from_slice(&ls.map(|v| o12 + v));
    let mthsp180 = Polygon::from_slice(&ls.map(|v| o17 - v));
    LatticeTiling {
        id: "pentagon-14",
        name: "Pentagon type 14 (Stein)",
        description: "Pentagonal tiling of type 14 (Stein 1985): 2a = 2c = d = e (a = c = 1, d = e = 2, b = βa), A = 90°, 2B + C = 360°, C + E = 180°; 3-isohedral (seeds 0/1/2), 6 tiles per oblique cell, long sides split into two collinear unit fine edges. Wallpaper group p2 is exact: the generic oblique lattice admits no mirrors or glides.",
        lattice: [w1(), w2()],
        cells: vec![
            Cell { poly: d0, seed: 0 },
            Cell { poly: d180, seed: 0 },
            Cell { poly: m0, seed: 1 },
            Cell { poly: m180, seed: 1 },
            Cell { poly: mths, seed: 2 },
            Cell { poly: mthsp180, seed: 2 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P2,
        lattice_kind: LatticeKind::Oblique,
        category: TilingCategory::PentagonTypes,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
