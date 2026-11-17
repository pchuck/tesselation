//! Pentagon type 2 tiling — pinned exact-Q(√3) member (a, b, c, d, e) =
//! (1, 1 + √3/2, 1/2, 1 + √3/2, 1/2), wallpaper group pgg, edge-to-edge.
//!
//! Type 2 (Reinhardt 1918, family 2) is the one-parameter family with
//! `c = e`, `d = b` and `B + D = 180°`.  The instance shipped here is the
//! exact member with side `a = 1`:
//!
//! * angles: A = 90°, B = 90°, C = 120°, D = 90°, E = 150°;
//! * sides: a = 1, b = d = 1 + √3/2, c = e = 1/2 (side a runs E→A);
//! * tile area: 1 + √3/2 = b — every coordinate lies in Q(√3).
//!
//! The tiling is **edge-to-edge at the pentagon level**: every side is
//! matched by exactly one full side of one neighbour, with no T-junctions
//! at all (a refined edge census over 276 tiles finds 1182 full edges and
//! zero splits; the two vertex figures are {90°, 120°, 150°} and
//! {90°, 90°, 90°, 90°}, both genuine corner stars summing to 360°).
//!
//! The prototile is **chiral** — its (side, angle) cycle
//! ((1, 90°), (1 + √3/2, 90°), (1/2, 120°), (1 + √3/2, 90°), (1/2, 150°))
//! differs from every cyclic rotation of its reversal — yet the tiling
//! uses *both* chiralities: the primitive cell holds two direct and two
//! mirror tiles (two chiral pairs).  The uncoloured wallpaper group is
//! therefore **pgg**: two perpendicular glide families with axes along
//! the lattice directions, `g1² = w1` and `g2² = −w2`, and no mirrors
//! (each glide shifts by exactly half a primitive translation along its
//! axis, so no power of it is a pure reflection).  The two glides
//! compose to the half-turn about c0 = (1 + √3/2, 0) — the prototile's
//! B vertex — and the half-turn centres are the four classes
//! c0 + Λ/2.  If the tiling is coloured by chirality (direct vs mirror),
//! the glides become pure reflections of each colour class and the group
//! drops to p2.
//!
//! The tiling is **1-isohedral**: the half-turn and the two glide classes
//! together permute the four cells of the primitive cell as a single
//! orbit — the glides doing the identifying across chiralities — so all
//! tiles carry one seed (0).
//!
//! The primitive cell: **4 tiles** (two chiral pairs) on the rectangular
//! lattice `w1 = (1/2, 1 + √3/2)`, `w2 = (−2 − √3, 1)` with `w1 ⊥ w2`
//! and `|det(w1, w2)| = 4 + 2√3 = 4 × tile area`.  The cell table was
//! extracted from a verified exact-Q(√3) construction (a 400-tile
//! edge-to-edge closure walk with zero branches and zero overlaps, plus
//! an exact symmetry audit of the quotient table: the half-turn and both
//! glide families are automorphisms; the mirror classes are not realised).

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

/// Exact tile area, 1 + √3/2.
#[cfg(test)]
pub(crate) fn tile_area() -> f64 {
    1.0 + s3() / 2.0
}

/// 2-fold rotation centre of the tiling: c0 = (1 + √3/2, 0), the
/// prototile's B vertex (the half-turn swaps the two chiral pairs).
#[cfg(test)]
pub(crate) fn two_fold_centre() -> Vec2 {
    Vec2::new(1.0 + s3() / 2.0, 0.0)
}

/// The CCW prototile P with corner A at the origin: edge directions
/// 0°, 90°, 150°, 240° of lengths `b = 1 + √3/2`, 1/2, `b`, 1/2
/// (the closing edge E→A has direction 270° and length `a = 1` — the
/// closure `E = (0, 1)` holds to ~1e-15).
pub(crate) fn prototile() -> [Vec2; 5] {
    let dir = |deg: f64| Vec2::new((deg * DEG).cos(), (deg * DEG).sin());
    let s = 1.0 + s3() / 2.0;
    let va = Vec2::ZERO;
    let vb = va + dir(0.0) * s;
    let vc = vb + dir(90.0) * 0.5;
    let vd = vc + dir(150.0) * s;
    let ve = vd + dir(240.0) * 0.5;
    [va, vb, vc, vd, ve]
}

/// The four cells of the primitive unit as `(mirror, anchor, rotation)`
/// triples: the cell polygon is the prototile (y-flipped when `mirror`)
/// rotated by `rotation` degrees and translated to `anchor` at vertex A
/// (the first 90° corner, distinguished by its angle).  Cells 0 and 1
/// are the direct chiral pair (cell 1 is the half-turn image of cell 0
/// about c0); cells 2 and 3 are the mirror chiral pair.
pub(crate) fn cell_table() -> [(bool, Vec2, f64); 4] {
    let s = 1.0 + s3() / 2.0;
    [
        (false, Vec2::ZERO, 0.0),
        (false, Vec2::new(2.0 + s3(), 0.0), 180.0),
        (true, Vec2::new(s, 0.5), 150.0),
        (true, Vec2::new((3.0 + s3()) / 2.0, (1.0 + s3()) / 2.0), 330.0),
    ]
}

/// The cell polygons in CCW order (mirrored cells are stored reversed, so
/// every polygon is positively oriented).
fn cell_polygons() -> [Vec<Vec2>; 4] {
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

/// Pentagon type 2 (Reinhardt 1918 family 2), pinned exact member:
/// 1-isohedral, 4 tiles (two chiral pairs) per rectangular cell,
/// wallpaper group pgg, edge-to-edge.
pub fn spec() -> LatticeTiling {
    let cells = cell_polygons()
        .into_iter()
        .map(|pts| Cell { poly: Polygon::from_slice(&pts), seed: 0 })
        .collect();
    LatticeTiling {
        id: "pentagon-2",
        name: "Pentagon type 2 (Reinhardt)",
        description: "Pentagonal tiling of type 2 (Reinhardt, 1918): A = B = D = 90°, C = 120°, E = 150°, sides a = 1, b = d = 1 + √3/2, c = e = 1/2 (c = e, d = b, B + D = 180°); edge-to-edge (every side is matched by exactly one full partner side), 1-isohedral, 4 tiles (two chiral pairs) per rectangular cell, wallpaper group pgg (two perpendicular glide families with axes along the lattice directions, squaring to w1 and −w2, no mirrors; colouring the two chiralities drops the group to p2).",
        lattice: [
            Vec2::new(0.5, 1.0 + s3() / 2.0),
            Vec2::new(-2.0 - s3(), 1.0),
        ],
        cells,
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
