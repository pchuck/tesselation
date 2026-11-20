//! Pentagon type 1 tiling — regular-hexagon member, wallpaper group cmm,
//! edge-to-edge.
//!
//! Type 1 (Reinhardt 1918, family 1) is the family of pentagons whose
//! article-lettered angles satisfy `B + C = 180°` (equivalently
//! `A + D + E = 360°`), with no side conditions — see "Pentagonal
//! tiling" (https://en.wikipedia.org/wiki/Pentagonal_tiling).  The
//! instance shipped here is the **regular-hexagon member**: a regular
//! hexagon of side 2 cut by the segment joining the midpoints of two
//! opposite edges — perpendicular to both, through the centre, length
//! 2√3 — splitting each hexagon's six vertices 3/3 into two congruent
//! pentagons.  The cut is unique up to the hexagon's own symmetry: a
//! general opposite-midpoint cut at parameter c ∈ (1, 3) yields side
//! cycles (c−1, 2, 2, 3−c, 2√3) vs. (3−c, 2, 2, 3−c, 2√3), directly
//! congruent only at c = 2.
//!
//! * angles: A = B = C = 120°, D = E = 90°;
//! * sides: a = b = 2, c = 1, d = 2√3, e = 1 (side a runs A→B);
//! * tile area: 3√3 — all geometry lies in Q(√3).
//!
//! The family condition holds under the relabelling (A′, B′, C′, D′,
//! E′) = (C, D, E, A, B) (equivalently its reversal (A, E, D, C, B)):
//! B′ + C′ = D + E = 180°, i.e. in the member's own lettering the
//! complementary pair is D + E = 180° ≡ A + B + C = 360°.  The member
//! is type-1 exclusive: none of the other 14 Reinhardt conditions holds
//! under any of the 10 dihedral relabellings.  (The shipped
//! `prismatic-pentagonal` Laves tiling also realises a type-1 pentagon,
//! but a different one — squared side multiset {1/3 ×2, ((2+√3)/6)² ×2,
//! 1} vs. {1, 1, 4, 4, 12} here.)
//!
//! The primitive cell holds two tiles on the centered-rectangular
//! translation lattice Λ (canonical basis (6, 0), (0, 2√3), centering
//! (3, √3) — not primitive-rectangular) in its 60° rhombus basis
//! `w1 = (3, −√3)`, `w2 = (3, √3)` with `det = +6√3 = 2 × tile area`:
//! cell 0 is the prototile (corner A at the origin, its hexagon centre
//! at (−2, 0)); cell 1 is its half-turn image about that hexagon centre
//! (anchor (−4, 0), rotation 180°).  The rhombus basis is the special
//! centered-rectangular case a² = 3b² (a = 6, b = 2√3): the lattice's
//! maximal metric type is hexagonal, even though the tiling's point
//! group has no 3-/6-fold.  The hexagon centres (−2, 0) + Λ form the
//! triangular lattice of spacing 2√3, and the two pentagons tile each
//! hexagon exactly.
//!
//! The uncoloured wallpaper group is **cmm** (orbifold \*2222).  The
//! prototile is *achiral*: a −120° rotation about the hexagon centre
//! (−2, 0) maps it exactly onto its mirror image (A↦C′, B↦B′,
//! C↦A′, D↦E′, E↦D′), so orientation-reversing symmetries exist.
//! There are two perpendicular mirror families: 60° lines at
//! s = p·ê₁₅₀ ≡ 0 (mod √3) — through hexagon centres, corners A/C′,
//! and cut midpoints — and 150° lines at t = p·ê₆₀ ≡ −1 (mod 3).
//! Glide reflections run between the mirrors on the same directions:
//! 60° axes at s ≡ √3/2 (mod √3) with glide vector 3ê₆₀ (whose
//! square is the lattice translation −w₁ + 2w₂), and 150° axes at
//! t ≡ 1/2 (mod 3) with glide vector √3·ê₁₅₀ (whose square is
//! −w₁).  The half-turn centres are the four classes (−2, 0) + Λ/2
//! mod Λ: two on mirrors (the hexagon centre, which swaps the two
//! cells, and the cut midpoint (−1/2, −√3/2)) and two off every
//! mirror ((−1/2, √3/2) and (1, 0)); the bare half-lattice points
//! Λ/2 = {0, w1/2, w2/2, (w1+w2)/2} + Λ are not centres.  There are
//! no 60/120/240/300° rotations.  Because the two off-mirror 2-fold
//! classes exist and the translation lattice is centered-rectangular
//! (basis (6, 0), (0, 2√3), centering (3, √3) — not
//! primitive-rectangular), the group is cmm rather than pmm or p2.
//! A coset-style audit (translation, half-turn census over the
//! quarter lattice, 60/120/240/300° rotation, mirror and glide
//! probes over ±6-cell patches) pins the group; the Wikipedia article
//! lists cmm (2\*22) among the type-1 tiling topologies.
//!
//! The tiling is **edge-to-edge**: whole hexagon sides pair full to
//! full, each half side (c- and e-sides, length 1) pairs one-to-one
//! with a neighbouring half side, and each cut d-side (2√3) pairs with
//! its companion tile's d-side in the same hexagon — no T-junctions.
//! The vertex figures are {120°, 120°, 120°} at the hexagon corners
//! (three tiles) and {90°, 90°, 90°, 90°} at the cut midpoints (four
//! tiles).  Per primitive cell: V = 3, E = 5, F = 2 (Euler characteristic
//! 0).
//!
//! It is **1-isohedral**: the half-turn about (−2, 0) swaps the two
//! cells mod lattice, so both tiles share one seed (0).

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

/// Exact tile area, 3√3.
#[cfg(test)]
pub(crate) fn tile_area() -> f64 {
    3.0 * s3()
}

/// 2-fold rotation centre of the tiling: c0 = (−2, 0), the hexagon
/// centre — the half-turn that swaps the two cells of the primitive
/// cell.
#[cfg(test)]
pub(crate) fn two_fold_centre() -> Vec2 {
    Vec2::new(-2.0, 0.0)
}

/// The CCW prototile P with corner A at the origin: edge directions
/// 120°, 180°, 240°, 330° of lengths `a = 2`, `b = 2`, `c = 1`,
/// `d = 2√3` (the closing edge E→A has direction 60° and length
/// `e = 1` — the closure `E = (−1/2, −√3/2)` holds to ~1e-15).
pub(crate) fn prototile() -> [Vec2; 5] {
    let dir = |deg: f64| Vec2::new((deg * DEG).cos(), (deg * DEG).sin());
    let va = Vec2::ZERO;
    let vb = va + dir(120.0) * 2.0;
    let vc = vb + dir(180.0) * 2.0;
    let vd = vc + dir(240.0);
    let ve = vd + dir(330.0) * (2.0 * s3());
    [va, vb, vc, vd, ve]
}

/// The two cells of the primitive unit as `(mirror, anchor, rotation)`
/// triples: the cell polygon is the prototile rotated by `rotation`
/// degrees and translated so its corner A sits at `anchor`.  Cell 1 is
/// the half-turn image of cell 0 about the hexagon centre (−2, 0).
/// Both cells are direct images of the prototile, so no mirror flag is
/// set (the tiling nevertheless has mirror symmetries — see the module
/// docs: the prototile is achiral).
pub(crate) fn cell_table() -> [(bool, Vec2, f64); 2] {
    [(false, Vec2::ZERO, 0.0), (false, Vec2::new(-4.0, 0.0), 180.0)]
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

/// Pentagon type 1 (Reinhardt 1918 family 1), regular-hexagon member:
/// 1-isohedral, 2 tiles per primitive cell, wallpaper group cmm,
/// edge-to-edge (each hexagon is cut by the segment joining the
/// midpoints of two opposite edges).
pub fn spec() -> LatticeTiling {
    let cells = cell_polygons()
        .into_iter()
        .map(|pts| Cell { poly: Polygon::from_slice(&pts), seed: 0 })
        .collect();
    LatticeTiling {
        id: "pentagon-1",
        name: "Pentagon type 1 (Reinhardt)",
        description: "Pentagonal tiling of type 1 (Reinhardt, 1918): A = B = C = 120°, D = E = 90° (the family condition B′ + C′ = 180° holds under the relabelling (C, D, E, A, B)), sides a = b = 2, c = 1, d = 2√3, e = 1 — the regular-hexagon member, a regular hexagon of side 2 cut by the segment joining the midpoints of two opposite edges; edge-to-edge, 1-isohedral, 2 tiles per centered-rectangular cell (60° rhombus basis, hexagonal-metric special case), wallpaper group cmm (achiral prototile: mirrors in two perpendicular directions, glide reflections between them, four half-turn classes (−2, 0) + Λ/2 with two off every mirror).",
        lattice: [Vec2::new(3.0, -s3()), Vec2::new(3.0, s3())],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::Cmm,
        lattice_kind: LatticeKind::CenteredRectangular,
        category: TilingCategory::PentagonTypes,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
