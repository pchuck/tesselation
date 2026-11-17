//! Pentagon type 10 (James, 1975) tiling — 3-isohedral,
//! non-edge-to-edge at the pentagon level, wallpaper group cmm.
//!
//! Type 10 is the first pentagon type found after Kershner's three
//! (1968); it was discovered by James in 1975.  It is the two-parameter
//! family with conditions `a = b = c + e`,
//! `A = 90°`, `B + E = 180°` (so `E = 180° − B`, `C = 180° − B/2`,
//! `D = 90° + B/2`, convex for `90° < B < 180°`, with the side ratio
//! `e/c` the second free parameter; side a runs E→A).  The instance
//! shipped here is the family member at `B = 90°`, at scale `c = 1`:
//!
//! * angles: A = B = E = 90°, C = D = 135°;
//! * sides: `a = b = 2`, `c = e = 1`, `d = √2` (exact — the d-edge
//!   closes the right angle between the two unit edges, `d² = c² + e²`);
//! * tile area: 7/2, and the primitive-cell determinant 21 = 6 × area.
//!
//! At `B = 90°` the prototile is an isosceles pentagon, symmetric in the
//! line y = x (vertex A, where the two length-2 sides meet, is fixed and
//! the axis crosses the midpoint of side d).  The two chiralities
//! therefore coincide as vertex sets: an M-cell and the D-cell at the
//! same anchor with matching rotation span exactly the same region.  The
//! mirrored M-prototile is nonetheless built explicitly (reflected in
//! y = x, stored reversed to stay CCW) so the cell table records which
//! chirality each tile carries.
//!
//! The tiling is **3-isohedral**: the three full-group orbits are
//! {cell 0, cell 2} (seed 0), {cell 1, cell 3} (seed 1), {cell 4,
//! cell 5} (seed 2).  The wallpaper group is **cmm**: point group
//! D2 = {I, −I, R1(y = x), R2(y = −x)}, all four realised exactly, and a
//! genuine glide g = T_u ∘ R2 with u = (5, −2): g² = T_{(7,−7)} is a
//! lattice translation, and the axis x + y = 3/2 sits at perpendicular
//! offset (3/4, 3/4) from the origin, so no conjugate of g is a pure
//! reflection.  The translation lattice L = Z(5, −2) + Z(8, 1) is
//! **centered rectangular**: the index-2 sublattice
//! Z(3, 3) + Z(−7, 7) is a primitive rectangle (sides 3√2 × 7√2) and its
//! centre (−2, 5) = ((3,3) + (−7,7))/2 lies in L.  That centered lattice
//! is what separates cmm from pmm.  R90 is *not* a symmetry:
//! R90(M90@(1,2)) lands on the D180@(−2,1) region, which no placed tile
//! occupies.
//!
//! The tiling is **not edge-to-edge at the pentagon level**: the two
//! length-2 sides a and b are split at their exact midpoint (an integer
//! T-junction), each half matched by one unit partner side; the c, d and
//! e sides are matched by exactly one full side each.  A refined edge
//! census over 164 tiles (594 edges with midpoints in the count window:
//! 518 full, 76 split at t = 1/2, every sub-edge contained in exactly
//! one partner side, no other T-junctions) pins this.  Interior vertex
//! stars are (90°, 135°, 135°), (90°, 90°, 180°) (T-junctions) and
//! (90°, 90°, 90°, 90°); every star sums to 360°.
//!
//! Primitive cell: **6 tiles** on the lattice `w1 = (5, −2)`,
//! `w2 = (8, 1)` with `|det(w1, w2)| = 21 = 6 × 7/2`.  Registered as
//! `Oblique` (the basis is not orthogonal); the centered-rectangular
//! structure lives in the index-2 mirror sublattice above.

use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// Degrees to radians.
const DEG: f64 = core::f64::consts::PI / 180.0;

/// Side ratios of the shipped `B = 90°` prototile at scale `c = 1`:
/// `(a, b, d)` with `c = 1`, `e = a − c`.  The B = 90° member is exact:
/// a = b = 2, c = e = 1, d = √2 (d² = c² + e² closes the right angle
/// at corner C).
pub(crate) fn ratios() -> (f64, f64, f64) {
    (2.0, 2.0, f64::sqrt(2.0))
}

/// The CCW prototile P with corner A at the origin: edge directions
/// 0°, 90°, 135°, 180° of lengths b, c, d, e (the closing edge E→A has
/// direction 270° and length a = 2 — the closure E = (0, 2) holds to
/// ~1e-15).
pub(crate) fn prototile() -> [Vec2; 5] {
    let (a, b, d) = ratios();
    let c = 1.0;
    let e = a - c;
    let dir = |deg: f64| Vec2::new((deg * DEG).cos(), (deg * DEG).sin());
    let va = Vec2::ZERO;
    let vb = va + dir(0.0) * b;
    let vc = vb + dir(90.0) * c;
    let vd = vc + dir(135.0) * d;
    let ve = vd + dir(180.0) * e;
    [va, vb, vc, vd, ve]
}

/// The six cells of the primitive unit as `(mirror, anchor, rotation)`
/// triples: the cell polygon is the prototile (reflected in the line
/// y = x when `mirror`) rotated by `rotation` degrees and translated to
/// `anchor` at vertex A.  At `B = 90°` the reflection is the isosceles
/// symmetry of the prototile, so the flag records the chirality each
/// tile carries without changing the region.
pub(crate) fn cell_table() -> [(bool, Vec2, f64); 6] {
    [
        (false, Vec2::ZERO, 0.0),
        (false, Vec2::ZERO, 90.0),
        (false, Vec2::ZERO, 180.0),
        (false, Vec2::ZERO, 270.0),
        (true, Vec2::new(1.0, 2.0), 90.0),
        (true, Vec2::new(-1.0, -2.0), 270.0),
    ]
}

/// The cell polygons in CCW order (mirrored cells are stored reversed, so
/// every polygon is positively oriented).
fn cell_polygons() -> [Vec<Vec2>; 6] {
    let p = prototile();
    // Reflection in the line y = x: the isosceles axis of the prototile
    // at B = 90°, so m and p span the same region.
    let m: [Vec2; 5] = p.map(|q| Vec2::new(q.y, q.x));
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

/// Pentagon type 10 (James 1975): 3-isohedral, 6 tiles per cell,
/// wallpaper group cmm, non-edge-to-edge (each length-2 side is split
/// at its midpoint between two unit sides).
pub fn spec() -> LatticeTiling {
    let cells = cell_polygons()
        .into_iter()
        .zip([0u32, 1, 0, 1, 2, 2])
        .map(|(pts, seed)| Cell { poly: Polygon::from_slice(&pts), seed })
        .collect();
    LatticeTiling {
        id: "pentagon-10",
        name: "Pentagon type 10 (James)",
        description: "Pentagonal tiling of type 10 (James, 1975): A = B = E = 90°, C = D = 135°, sides a = b = 2, c = e = 1, d = √2 (a = b = c + e, d² = c² + e²); isosceles B = 90° instance, non-edge-to-edge (each length-2 side is split at its midpoint between two unit sides), 3-isohedral (seeds 0/1/2), 6 tiles per cell, wallpaper group cmm (mirrors y = x and y = −x, half-turn at the origin, genuine glide g = T(5,−2)∘R2 with axis x + y = 3/2; centered rectangular lattice Z(5,−2)+Z(8,1)).",
        lattice: [
            Vec2::new(5.0, -2.0),
            Vec2::new(8.0, 1.0),
        ],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::Cmm,
        lattice_kind: LatticeKind::Oblique,
        category: TilingCategory::Wallpaper,
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
