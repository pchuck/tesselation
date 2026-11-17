//! Pentagon type 12 (Rice, 1977) tiling — 2-isohedral,
//! non-edge-to-edge at the pentagon level, wallpaper group pgg.
//!
//! Type 12 is one of the four pentagon families discovered by Marjorie
//! Rice in 1976–77.  It is the two-parameter family with conditions
//! `2a = d = c + e`,
//! `A = 90°`, `2B + C = 360°` (so `C = 360° − 2B`, `E = 2B − 180°`,
//! `D = 270° − B`, convex for `90° < B < 180°`, with `b` the second free
//! parameter; side a runs E→A).  The instance shipped here is the family
//! member at `B = 150°` and scale `c = 1`; at that member the remaining
//! sides are exact surds in Q(√3):
//!
//! * angles: A = 90°, B = 150°, C = 60°, D = 120°, E = 120°;
//! * sides: `a = 1`, `b = √3`, `c = 1`, `d = 2`, `e = 1` — computed at
//!   runtime from the closed forms
//!   `a = (sin B + cos 2B)/(2 cos 2B − 2 sin B + 1)`,
//!   `b = cos B − 2a cos B − (2a − 1) sin 2B`, `d = 2a`, `e = 2a − 1`
//!   (the tests pin the surds a = 1, b = √3, d = 2 at 1e-12);
//! * tile area: 3√3/2 (8·area = 12√3 = the lattice determinant).
//!
//! The tiling is **2-isohedral**: the two full-group orbits are
//! {cell 0, cell 3, cell 4, cell 7} (seed 0) and {cell 1, cell 2, cell 5,
//! cell 6} (seed 1); each orbit mixes both chiralities, the glides doing
//! the identifying.  The uncoloured wallpaper group is **pgg**: two
//! perpendicular glide families with axes along the lattice directions,
//! squaring to the primitive translations — `g1² = w1` and `g2² = −w2` —
//! and no mirrors (each glide shifts by exactly half a primitive
//! translation along its axis, so no power of it is a pure reflection).
//! The half-turns are centred at c0 = (√3/4, 5/4) and its images;
//! composing the two glides gives the half-turn about c0 itself.
//!
//! The tiling is **not edge-to-edge at the pentagon level**: the long side
//! d = 2 of a tile is split at its exact midpoint, covered by two unit
//! partner sides (a- or c-halves on one side, a- or e-halves on the
//! other), and every other side is matched by exactly one full side of one
//! neighbour (b ↔ b, c ↔ c, e ↔ e).  A refined edge census over 215 tiles
//! (620 full edges, 153 d-sides split at t = 1/2, every sub-edge contained
//! in exactly one partner edge, no other T-junctions) pins this.  The four
//! vertex figures are {90°, 90°, 180°}, {60°, 120°, 180°},
//! {120°, 120°, 120°} and {60°, 150°, 150°} (each 360° with exactly three
//! rays; the two straight-angle figures are T-junctions where the d-side
//! is bisected).  The cell table was extracted from a verified
//! construction:
//! zero overlaps, exactly-once coverage, a complete refined edge gluing,
//! and an exact symmetry audit of the quotient table (the half-turn and
//! both glide families are automorphisms; the mirror classes are not
//! realised).
//!
//! Primitive cell: **8 tiles** on the rectangular lattice
//! `w1 = (−2.598076211353316, −4.5)`, `w2 = (3.4641016151377544, −2.0)`
//! with `w1 ⊥ w2` and `|det(w1, w2)| = 12√3 = 8 × tile area`.

use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// Degrees to radians.
const DEG: f64 = core::f64::consts::PI / 180.0;

/// Side ratios of the shipped `B = 150°` prototile at scale `c = 1`:
/// `(a, b, d)` with `c = 1`, `e = 2a − 1`, from the closure forms
/// `a = (sin B + cos 2B)/(2 cos 2B − 2 sin B + 1)`,
/// `b = cos B − 2a cos B − (2a − 1) sin 2B`, `d = 2a`.
pub(crate) fn ratios() -> (f64, f64, f64) {
    let b = 150.0 * DEG;
    let (sb, cb) = b.sin_cos();
    let s2 = (2.0 * b).sin();
    let c2 = (2.0 * b).cos();
    let a = (sb + c2) / (2.0 * c2 - 2.0 * sb + 1.0);
    let bb = cb - 2.0 * a * cb - (2.0 * a - 1.0) * s2;
    (a, bb, 2.0 * a)
}

/// The CCW prototile P with corner A at the origin: edge directions
/// 0°, 30°, 150°, 210° of lengths `b, 1, d, e` (the closing edge E→A has
/// direction 270° and length `a` — the closure `E = (0, a)` holds to
/// ~1e-16).
pub(crate) fn prototile() -> [Vec2; 5] {
    let (a, b, d) = ratios();
    let e = 2.0 * a - 1.0;
    let dir = |deg: f64| Vec2::new((deg * DEG).cos(), (deg * DEG).sin());
    let va = Vec2::ZERO;
    let vb = va + dir(0.0) * b;
    let vc = vb + dir(30.0);
    let vd = vc + dir(150.0) * d;
    let ve = vd + dir(210.0) * e;
    [va, vb, vc, vd, ve]
}

/// The eight cells of the primitive unit as `(mirror, anchor, rotation)`
/// triples: the cell polygon is the prototile (y-flipped when `mirror`)
/// rotated by `rotation` degrees and translated to `anchor` at vertex A
/// (the 90° corner, distinguished by its angle).  The two cells of each
/// mirror pair share one anchor.
pub(crate) fn cell_table() -> [(bool, Vec2, f64); 8] {
    [
        (false, Vec2::ZERO, 0.0),
        (true, Vec2::ZERO, 0.0),
        (false, Vec2::new(2.598076211353316, 1.5), 120.0),
        (true, Vec2::new(0.0, -3.0), 120.0),
        (false, Vec2::new(0.8660254037844386, 2.5), 180.0),
        (true, Vec2::new(0.8660254037844386, 2.5), 180.0),
        (false, Vec2::new(-1.7320508075688772, 1.0), 300.0),
        (true, Vec2::new(-1.7320508075688772, 1.0), 300.0),
    ]
}

/// 2-fold rotation centre of the tiling: c0 = (√3/4, 5/4), the centre of
/// the half-turn g2∘g1.
#[cfg(test)]
pub(crate) fn two_fold_centre() -> Vec2 {
    Vec2::new(0.4330127018922193, 1.25)
}

/// The cell polygons in CCW order (mirrored cells are stored reversed, so
/// every polygon is positively oriented).
fn cell_polygons() -> [Vec<Vec2>; 8] {
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

/// Pentagon type 12 (Rice 1977): 2-isohedral, 8 tiles per rectangular
/// cell, wallpaper group pgg, non-edge-to-edge (each d-side is split at
/// its midpoint between two unit sides).
pub fn spec() -> LatticeTiling {
    let cells = cell_polygons()
        .into_iter()
        .zip([0u32, 1, 1, 0, 0, 1, 1, 0])
        .map(|(pts, seed)| Cell { poly: Polygon::from_slice(&pts), seed })
        .collect();
    LatticeTiling {
        id: "pentagon-12",
        name: "Pentagon type 12 (Rice)",
        description: "Pentagonal tiling of type 12 (Rice, 1977): A = 90°, B = 150°, C = 60°, D = 120°, E = 120°, sides a = 1, b = √3, c = 1, d = 2, e = 1 (2a = d = c + e); non-edge-to-edge (each d-side is split at its midpoint between two unit sides), 2-isohedral (seeds 0/1), 8 tiles per rectangular cell, wallpaper group pgg (two perpendicular glide families with axes along the lattice directions, squaring to w1 and −w2, no mirrors).",
        lattice: [
            Vec2::new(-2.598076211353316, -4.5),
            Vec2::new(3.4641016151377544, -2.0),
        ],
        cells,
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
