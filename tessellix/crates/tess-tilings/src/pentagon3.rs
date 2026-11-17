//! Pentagon type 3 tiling — pinned integral member (a, b, c, d, e) =
//! (2, 2, 1, 4, 3), wallpaper group p2, non-edge-to-edge.
//!
//! Type 3 (Reinhardt 1918, family 3) is the one-parameter family with
//! `a = b`, `d = c + e` and `A = C = D = 120°`, `B + E = 180°`.  The
//! instance shipped here is the integral member `c = 1`:
//!
//! * angles: A = B = C = D = 120°, E = 60°;
//! * sides: a = b = 2, c = 1, d = 4, e = 3 (side a runs A→B);
//! * tile area: 5√3 — all geometry lies in Q(√3).
//!
//! The pinned member is simultaneously a **type-1** pentagon under the
//! relabelling (A′, B′, C′, D′, E′) = (D, E, A, B, C): B′ + C′ = E + A =
//! 180° and A′ + D′ + E′ = D + B + C = 360°.  The type-3 identity is the
//! one registered here (M64 brief).
//!
//! The **classical type-3 tiling does not exist for this member**.  The
//! textbook construction is the three-tile pinwheel — three copies
//! rotated by 120° about the shared 120° corner — but a half-turn- or
//! 120°-rotation-polygon argument pins it out: at a 120° corner the
//! boundary of any putative pinwheel must pair equal side *directions*,
//! yet the type-3 d-side (length 4 = c + e) is never direction-paired by
//! the two neighbours, and for the β = 60° members of the family the
//! per-direction boundary length totals are odd.  A complete closure
//! search (every lattice of determinant 15 in unit-side coordinates,
//! both chiralities, T-junctions admitted; ~1.7 × 10⁶ pairing trials)
//! finds no 3-tile periodic cell, and the hexagonal lattice routes are
//! dead by the norm-form test x² + 3y² = 120 (det 6 in tile units;
//! cell-6/cell-9 variants scale to 240/360, also insoluble).  The
//! shipped structure is therefore the discovered minimal periodic cell:
//! **2 tiles, p2**.
//!
//! The primitive cell is `T0` (the prototile) plus `T1`, the half-turn
//! image of `T0` about the midpoint of its long side d; the tiling is
//! the lattice orbit on the oblique lattice
//! `w1 = (0.5, 2.598076211353316)`, `w2 = (−6.5, 0.8660254037844386)`
//! with `det = 10√3 = 2 × tile area`.
//!
//! The uncoloured wallpaper group is **p2** (orbifold 2222):
//! the prototile is chiral — its side cycle is `(2, 2, 1, 4, 3)`, which
//! differs from every rotation of its reversal `(3, 4, 1, 2, 2)` — and
//! the tiling uses one chirality only, so no orientation-reversing
//! isometry (mirror or glide) can preserve it; realised tile
//! orientations are {0°, 180°}, excluding 3- and 4-folds.  The half-turn
//! centres are the four classes `d/2 + Λ/2` mod Λ (e.g. c0 = (−0.75,
//! 1.299038105676658), the midpoint of the prototile's d-side); the
//! lattice points and the half-lattice points off `d/2 + Λ/2` are not
//! centres.
//!
//! The tiling is **1-isohedral**: the half-turn swaps the two cells of
//! the primitive cell mod Λ, so both tiles share one seed (0).
//!
//! It is **not edge-to-edge at the pentagon level**: only the long d = 4
//! sides are split, each at its exact midpoint t = 1/2 by the ends of
//! two length-2 partner sides (a- or b-sides); every other side is
//! matched by exactly one full side of one neighbour.  A refined edge
//! census over 60 tiles (172 full edges, 46 d-sides split at t = 1/2,
//! every sub-edge contained in exactly one partner edge, no other
//! T-junctions; minimum boundary margin 0.0388) pins this.  The two
//! vertex figures are {A, E, 180°} = 120° + 60° + 180° at the A- and
//! E-type lattice points (the T-junction points) and
//! {B, C, D} = 120°·3 at the B-, C- and D-type points (per primitive
//! cell: two of the first, four of the second — the B- and D-classes
//! coincide mod Λ because D = B + w1).  The cell table was extracted
//! from a verified exact-Q construction: zero overlaps, exactly-once
//! coverage, a complete refined edge gluing, and an exact symmetry audit
//! of the quotient table (the half-turn is an automorphism; no mirror
//! class is realised).
//!
//! Primitive cell: **2 tiles** on the oblique lattice
//! `w1 = (0.5, 2.598076211353316)`, `w2 = (−6.5, 0.8660254037844386)`
//! with `|det(w1, w2)| = 10√3 = 2 × tile area`.

use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// Degrees to radians.
const DEG: f64 = core::f64::consts::PI / 180.0;

/// √3 — every coordinate in this module is a Q(√3) surd built from it
/// (computed at runtime; no decimal approximations are shipped).
pub(crate) fn s3() -> f64 {
    3.0f64.sqrt()
}

/// xy image of the uv point `(u2/2, v2/2)` under the convention
/// xy = (u + v/2, v·√3/2) — the pinned member's uv coordinates are all
/// half-integers, so this helper takes doubled numerators.
fn uv2(u2: f64, v2: f64) -> Vec2 {
    Vec2::new(u2 + v2 / 2.0, v2 * s3() / 2.0)
}

/// Exact tile area, 5√3.
#[cfg(test)]
pub(crate) fn tile_area() -> f64 {
    5.0 * s3()
}

/// 2-fold rotation centre of the tiling: c0 = (−3/4, 3√3/4), the
/// midpoint of the prototile's long side d (the half-turn taking the
/// prototile to the second cell).
#[cfg(test)]
pub(crate) fn two_fold_centre() -> Vec2 {
    uv2(-1.5, 1.5)
}

/// The CCW prototile P with corner A at the origin: edge directions
/// 0°, 60°, 120°, 180° of lengths `a = 2`, `b = 2`, `c = 1`, `d = 4`
/// (the closing edge E→A has direction 300° and length `e = 3` — the
/// closure `E = (−3/2, 3√3/2)` holds to ~1e-15).
pub(crate) fn prototile() -> [Vec2; 5] {
    let dir = |deg: f64| Vec2::new((deg * DEG).cos(), (deg * DEG).sin());
    let va = Vec2::ZERO;
    let vb = va + dir(0.0) * 2.0;
    let vc = vb + dir(60.0) * 2.0;
    let vd = vc + dir(120.0);
    let ve = vd + dir(180.0) * 4.0;
    [va, vb, vc, vd, ve]
}

/// The two cells of the primitive unit as `(mirror, anchor, rotation)`
/// triples: the cell polygon is the prototile rotated by `rotation`
/// degrees and translated so its corner A sits at `anchor`.  Cell 1 is
/// the half-turn image of cell 0 about the d-midpoint c0.  No mirror
/// occurs — the tiling uses one chirality only.
pub(crate) fn cell_table() -> [(bool, Vec2, f64); 2] {
    [(false, Vec2::ZERO, 0.0), (false, uv2(-3.0, 3.0), 180.0)]
}

/// The cell polygons in CCW order (mirrored cells would be stored
/// reversed, so every polygon is positively oriented).
fn cell_polygons() -> [Vec<Vec2>; 2] {
    let p = prototile();
    let m: [Vec2; 5] = p.map(|q| Vec2::new(q.x, -q.y));
    std::array::from_fn(|k| {
        let (mir, anchor, th) = cell_table()[k];
        let rad = th * DEG;
        let (co, si) = (rad.cos(), rad.sin());
        let src = if mir { &m } else { &p };
        (0..5)
            .map(|i| {
                // Mirrored cells are stored reversed to stay CCW.
                let q = if mir { src[(5 - i) % 5] } else { src[i] };
                Vec2::new(co * q.x - si * q.y + anchor.x, si * q.x + co * q.y + anchor.y)
            })
            .collect()
    })
}

/// Pentagon type 3 (Reinhardt 1918 family 3), pinned integral member:
/// 1-isohedral, 2 tiles per primitive cell, wallpaper group p2,
/// non-edge-to-edge (each d-side is split at its midpoint between two
/// length-2 sides).
pub fn spec() -> LatticeTiling {
    let cells = cell_polygons()
        .into_iter()
        .map(|pts| Cell { poly: Polygon::from_slice(&pts), seed: 0 })
        .collect();
    LatticeTiling {
        id: "pentagon-3",
        name: "Pentagon type 3 (Reinhardt)",
        description: "Pentagonal tiling of type 3 (Reinhardt, 1918): A = B = C = D = 120°, E = 60°, sides a = b = 2, c = 1, d = 4, e = 3 (a = b, d = c + e); non-edge-to-edge (each d-side is split at its midpoint between two length-2 sides), 1-isohedral, 2 tiles per oblique cell, wallpaper group p2 (chiral prototile, one chirality used; four half-turn classes d/2 + Λ/2, no mirrors or glides; the classical three-tile type-3 pinwheel admits no periodic cell for this member).",
        lattice: [Vec2::new(0.5, 1.5 * s3()), Vec2::new(-6.5, 0.5 * s3())],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P2,
        lattice_kind: LatticeKind::Oblique,
        category: TilingCategory::PentagonTypes,
        secondary: &[TilingCategory::Wallpaper],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
