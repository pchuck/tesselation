//! Pentagon type 11 (Rice, 1977) tiling — 2-isohedral,
//! non-edge-to-edge at the pentagon level, wallpaper group pgg.
//!
//! Type 11 is one of the four pentagon families discovered by Marjorie
//! Rice in 1976–77.  It is the two-parameter family with conditions
//! `2a + c = d = e`,
//! `A = 90°`, `2B + C = 360°`, `C + E = 180°` (so `C = 360° − 2B`,
//! `E = 2B − 180°`, `D = 270° − B`, convex for `90° < B < 180°`, with `b`
//! the second free parameter; side a runs E→A).  The instance shipped here
//! is the reference construction's `B = 156°` member at scale `c = 1`:
//!
//! * angles: A = 90°, B = 156°, C = 48°, D = 114°, E = 132°;
//! * sides: `a = r = 0.0946641…`, `b = s = 1.0568030…`, `c = 1`,
//!   `d = e = 1 + 2r = 1.1893282…` — computed at runtime from the closed
//!   forms `r = (2 sin B − cos 2B)/(1 + 2 cos 2B − 2 sin B)` and
//!   `s = −2r cos B − (1 + 2r) sin 2B` (B = 156° is not a surd angle, so
//!   the shipped numbers are bounded-precision floats; every closure
//!   identity below holds to ~1e-15 and the tests pin them);
//! * tile area: 0.9542866…
//!
//! The tiling is **2-isohedral**: the two full-group orbits are
//! {cell 0, cell 2, cell 4, cell 7} (seed 0) and {cell 1, cell 3, cell 5,
//! cell 6} (seed 1); each orbit mixes both chiralities, the glides doing
//! the identifying.  The uncoloured wallpaper group is **pgg**: two
//! perpendicular glide families with axes along the lattice directions,
//! squaring to the primitive translations — `g1² = w1` and `g2² = −w2` —
//! and no mirrors (each glide shifts by exactly half a primitive
//! translation along its axis, so no power of it is a pure reflection).
//! The half-turns are centred at c0 = (−0.37157…, 0.76010…) and its
//! images; composing the two glides gives the half-turn about c0 itself.
//!
//! The tiling is **not edge-to-edge at the pentagon level**: the long side
//! d = e of a tile is covered by one partner c-side plus two partner
//! a-sides — exactly the family condition `2a + c = d` — and every other
//! side is matched by exactly one full side of one neighbour.  A refined
//! edge census over 528 tiles (1882 full edges, 210 d-sides split at the
//! 2a + c positions, each sub-edge contained in exactly one partner edge,
//! no other T-junctions) pins this.  The four vertex stars are
//! {C,B,B} = 48° + 2·156°, {A,A,180°} = 2·90° + 180°, {D,D,E} =
//! 2·114° + 132° and {C,E,180°} = 48° + 132° + 180° (each 360°).  The
//! cell table was extracted from a verified construction: zero overlaps,
//! exactly-once coverage, a complete refined edge gluing, and an exact
//! symmetry audit of the quotient table (the half-turn and both glide
//! families are automorphisms; the mirror classes are not realised).
//!
//! Primitive cell: **8 tiles** on the rectangular lattice
//! `w1 = (−2.1433083997208975, −1.3918807493424277)`,
//! `w2 = (1.6269879260579216, −2.5053417038859127)` with `w1 ⊥ w2` and
//! `|det(w1, w2)| = 7.63429309180… = 8 × tile area`.

use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// Degrees to radians.
const DEG: f64 = core::f64::consts::PI / 180.0;

/// Side ratios of the shipped `B = 156°` prototile at scale `c = 1`:
/// `(r, s, d)` with `a = r`, `b = s`, `c = 1`, `d = e = 1 + 2r`, from the
/// closure forms `r = (2 sin B − cos 2B)/(1 + 2 cos 2B − 2 sin B)` and
/// `s = −2r cos B − (1 + 2r) sin 2B`.
pub(crate) fn ratios() -> (f64, f64, f64) {
    let b = 156.0 * DEG;
    let (sb, cb) = b.sin_cos();
    let s2 = (2.0 * b).sin();
    let c2 = (2.0 * b).cos();
    let r = (2.0 * sb - c2) / (1.0 + 2.0 * c2 - 2.0 * sb);
    let s = -2.0 * r * cb - (1.0 + 2.0 * r) * s2;
    (r, s, 1.0 + 2.0 * r)
}

/// The CCW prototile P with corner A at the origin: edge directions
/// 0°, 24°, 156°, 222° of lengths `s, 1, d, d` (the closing edge E→A has
/// direction 270° and length `a = r` — the closure `E = (0, r)` holds to
/// ~1e-15).
pub(crate) fn prototile() -> [Vec2; 5] {
    let (_r, s, d) = ratios();
    let dir = |deg: f64| Vec2::new((deg * DEG).cos(), (deg * DEG).sin());
    let va = Vec2::ZERO;
    let vb = va + dir(0.0) * s;
    let vc = vb + dir(24.0);
    let vd = vc + dir(156.0) * d;
    let ve = vd + dir(222.0) * d;
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
        (true, Vec2::new(-1.172985338765445, -0.5399041041282324), 66.0),
        (false, Vec2::new(-1.172985338765445, -0.5399041041282324), 66.0),
        (false, Vec2::new(-0.7431448254773945, 1.5201975976200481), 180.0),
        (true, Vec2::new(-0.7431448254773945, 1.5201975976200481), 180.0),
        (false, Vec2::new(-1.7134678864329238, 0.6682209524054715), 246.0),
        (true, Vec2::new(-1.7134678864329238, 0.6682209524054715), 246.0),
    ]
}

/// 2-fold rotation centre of the tiling (half of the 180°-pair anchor).
#[cfg(test)]
pub(crate) fn two_fold_centre() -> Vec2 {
    Vec2::new(-0.37157241273869723, 0.7600987988100241)
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

/// Pentagon type 11 (Rice 1977): 2-isohedral, 8 tiles per rectangular
/// cell, wallpaper group pgg, non-edge-to-edge (the d-side carries a
/// c + a + a joint).
pub fn spec() -> LatticeTiling {
    let cells = cell_polygons()
        .into_iter()
        .zip([0u32, 1, 0, 1, 0, 1, 1, 0])
        .map(|(pts, seed)| Cell { poly: Polygon::from_slice(&pts), seed })
        .collect();
    LatticeTiling {
        id: "pentagon-11",
        name: "Pentagon type 11 (Rice)",
        description: "Pentagonal tiling of type 11 (Rice, 1977): A = 90°, B = 156°, C = 48°, D = 114°, E = 132°, sides a = r, b = s, c = 1, d = e = 1 + 2r (2a + c = d = e); non-edge-to-edge (each d-side carries one c-side and two a-sides), 2-isohedral (seeds 0/1), 8 tiles per rectangular cell, wallpaper group pgg (two perpendicular glide families with axes along the lattice directions, squaring to w1 and −w2, no mirrors).",
        lattice: [
            Vec2::new(-2.1433083997208975, -1.3918807493424277),
            Vec2::new(1.6269879260579216, -2.5053417038859127),
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
