//! Pentagon type 7 (Richard Kershner, 1968) tiling — 2-isohedral,
//! edge-to-edge, wallpaper group pgg.
//!
//! Type 7 is the one-parameter Kershner family with `b = c = d = e`,
//! `B + 2E = 360°` and `2C + D = 360°` (so `B = 360° − 2E`,
//! `D = 360° − 2C`, `A = C − B/2`, with C the unique root in
//! (120°, 180°) of the closure equation
//! `F(t) = sin(E − 2t) − sin(t + E) − sin(E − t) − sin(E)`
//! in degrees).  The equation is transcendental — the family's
//! published members are not surd angles — so, like the pentagon-11
//! module, the prototile is computed at runtime (the applet's own bisection
//! to 1e-3 plus Newton refinement, replicated bit-for-bit from the
//! verified M58-R construction) rather than pinned to a closed form.
//! The instance shipped here is the **published E = 106° member**
//! (the angle measured from the applet's P5-type7 reference figure;
//! the applet's slider default is 110° — the family construction
//! closes for every E in the convex window, sweep-verified across
//! 15 parameters):
//!
//! * angles: A = 58.8626451020303°, B = 148°, C = 132.8626451020303°,
//!   D = 94.2747097959394°, E = 106°;
//! * sides: `b = c = d = e = 1/2`, `a = 1.035794206008535` (the
//!   closing side E→A);
//! * tile area: 0.529540774017982.
//!
//! The tiling is **2-isohedral**: the two full-group orbits are
//! {cell 0, cell 3, cell 4, cell 7} (seed 0) and {cell 1, cell 2,
//! cell 5, cell 6} (seed 1); each orbit mixes both chiralities, the
//! glides doing the identifying.  The primitive cell holds four
//! direct and four mirror tiles.
//!
//! The tiling is **edge-to-edge at the pentagon level**: every side
//! is matched by exactly one full side of one neighbour (a refined
//! edge census over a 907-tile patch finds 3770 full edges and zero
//! splits — the arc's first edge-to-edge pgg).  The three vertex
//! stars are {E, E, B} = {106°, 106°, 148°}, {D, C, C} =
//! {94.27°, 132.86°, 132.86°} and {A, A, D, B} = {58.86°, 58.86°,
//! 94.27°, 148°} (each 360°).
//!
//! The uncoloured wallpaper group is **pgg**: two perpendicular
//! glide families with axes along the lattice directions, squaring
//! to the primitive translations — `g1² = w1` and `g2² = −w2` (each
//! glide shifts by exactly half a primitive translation along its
//! axis, so no power of it is a pure reflection) — and no mirrors
//! (24 negative probes).  Composing the two glides gives the
//! half-turn about c0 = (1/4, 0) itself; the half-turn centres are
//! the four classes c0 + Λ/2.
//!
//! The cell is the **bit-exact port** of the M58-R study
//! construction, extracted from Jaap Scherphuis' Tiling Viewer
//! (`Tilings.jar`, class `TilingTypeN5_07`, symmetry pgg): tile 0 is
//! the base prototile and tiles 1..8 are glued on by the applet's
//! 7-row description table `(v1, v2, src, k1, k2, mirror)` with the
//! applet's exact transform semantics (translate; mirror branch =
//! rotate −getangle4(v1→v2), mirror y about v1, rotate
//! +getangle4(v1→src.k2); direct branch = rotate getangle6 about
//! v1).  Mirrored tiles are stored vertex-reversed so every stored
//! polygon stays CCW.  The lattice is **computed from the
//! construction**, not hardcoded: `w1 = tiles[2].p3 − tiles[0].p0`,
//! `w2raw = tiles[5].p3 − tiles[3].p1` (the applet's L1/L2); the
//! raw determinant is negative (−4.236326192143856), so the shipped
//! basis is `(w1, −w2raw)`, positively oriented.  For the published
//! instance the basis is rectangular:
//! `w1 = (1.4422756957517922, 1.02096015812545)`,
//! `w2 = (−1.3851382485043073, 1.9567376994827812)`,
//! `w1·w2 ≈ 0`, `det = 4.236326192143856 = 8 × tile area`.
//!
//! Provenance: the M58-R study closed the type-7 deferral — the M58
//! bounded negative was a grid-pinning artifact (the family's
//! published members are off-grid) compounded by two search-phase
//! accounting bugs; the applet-grounded published instance below
//! carries the full verification battery (window censuses, refined
//! edge census, vertex-star census, quotient-torus census,
//! symmetry audit) and 11 tests.

use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// Degrees to radians (== the applet's 0.017453292519943295).
const DEG: f64 = core::f64::consts::PI / 180.0;

/// The published instance parameter: angle E, in degrees (measured
/// from the applet's P5-type7 reference figure; the applet's slider
/// default is 110°).
const E_DEG: f64 = 106.0;

/// The closure equation extracted from the applet
/// (`TilingTypeN5_07$Func.calculate`), in degrees:
/// `F(t) = sin(E − 2t) − sin(t + E) − sin(E − t) − sin(E)`.
pub(crate) fn closure_f(t: f64) -> f64 {
    let e = E_DEG;
    ((e - 2.0 * t) * DEG).sin()
        - ((t + e) * DEG).sin()
        - ((e - t) * DEG).sin()
        - (e * DEG).sin()
}

/// Angle C of the shipped instance: the unique root of
/// [`closure_f`] in (120°, 180°).  Replicates the applet's
/// bisection (bracket [120, 180], tolerance 1e-3) followed by Newton
/// refinement (≤200 iterations, central difference h = 1e-6),
/// operation-for-operation with the verified Python study.
pub(crate) fn angle_c() -> f64 {
    let mut a = 120.0;
    let mut b = 180.0;
    let mut fa = closure_f(a);
    while b - a > 1e-3 {
        let m = (a + b) / 2.0;
        let fm = closure_f(m);
        if fm * fa <= 0.0 {
            b = m;
        } else {
            a = m;
            fa = fm;
        }
    }
    let mut t = (a + b) / 2.0;
    for _ in 0..200 {
        let f0 = closure_f(t);
        let h = 1e-6;
        let df = (closure_f(t + h) - closure_f(t - h)) / (2.0 * h);
        let step = f0 / df;
        t -= step;
        if step.abs() <= 1e-16 * t.abs().max(1.0) {
            break;
        }
    }
    t
}

/// The five interior angles of the shipped instance in degrees, in
/// vertex order (A, B, C, D, E): B = 360 − 2E, C = the closure root,
/// D = 360 − 2C, A = C − B/2, E = 106°.
pub(crate) fn angles() -> [f64; 5] {
    let e = E_DEG;
    let c = angle_c();
    let b = 360.0 - 2.0 * e;
    let d = 360.0 - 2.0 * c;
    let a = c - b / 2.0;
    [a, b, c, d, e]
}

/// The CCW base prototile with corner A at the origin and B at
/// (1/2, 0), edge directions 0°, 180°−B, −(B+C), 180°−(B+C+D) of
/// lengths 1/2 each (the closing side E→A has length
/// `a = 1.035794206008535` — the closure holds to ~1e-15).
/// Operation order replicates the applet's `recalcBase` exactly.
pub(crate) fn prototile() -> [Vec2; 5] {
    // x = B, c = C, y = D (A and E are not needed for the edge
    // directions; routing through `angles` keeps the single angle_c
    // evaluation the study verified).
    let [_a, x, c, y, _e] = angles();
    let s = 0.5;
    let dir = |deg: f64| Vec2::new((deg * DEG).cos(), (deg * DEG).sin());
    let p0 = Vec2::ZERO;
    let p1 = Vec2::new(s, 0.0);
    let p2 = p1 + dir(180.0 - x) * s;
    let p3 = p2 + dir(-x - c) * s;
    let p4 = p3 + dir(180.0 - x - c - y) * s;
    [p0, p1, p2, p3, p4]
}

/// The applet's gluing table, rows 1..8 as `(v1, v2, src, k1, k2,
/// mirror)`: glue a fresh copy of the base so that its vertex v1
/// lands on `tiles[src].k1`, then (mirror) rotate −getangle4(v1→v2)
/// about v1, mirror y about v1, rotate +getangle4(v1→src.k2) about
/// v1, or (direct) rotate getangle6(v1→src.k2 relative to v1→v2)
/// about v1.
const DESC: [(u8, u8, u8, u8, u8, bool); 7] = [
    (0, 4, 0, 0, 4, true),
    (2, 1, 0, 3, 4, false),
    (0, 4, 2, 0, 4, true),
    (3, 2, 0, 2, 3, false),
    (0, 4, 4, 0, 4, true),
    (2, 1, 4, 3, 4, false),
    (0, 4, 6, 0, 4, true),
];

/// Which of the eight cells is a mirror image (stored vertex-reversed
/// to stay CCW): cells 1, 3, 5, 7.
pub(crate) const MIRROR: [bool; 8] = [false, true, false, true, false, true, false, true];

/// Pattern seeds: the applet's colour column — the two full-group
/// isohedral orbits {0, 3, 4, 7} → 0 and {1, 2, 5, 6} → 1.
const SEEDS: [u32; 8] = [0, 1, 1, 0, 0, 1, 1, 0];

/// Applet transform: translation.
fn translate(t: [Vec2; 5], d: Vec2) -> [Vec2; 5] {
    let mut out = [Vec2::ZERO; 5];
    for i in 0..5 {
        out[i] = Vec2::new(t[i].x + d.x, t[i].y + d.y);
    }
    out
}

/// Applet transform: rotation about `c` by angle `a` (radians),
/// `co`/`si` computed once.
fn rotate(t: [Vec2; 5], c: Vec2, a: f64) -> [Vec2; 5] {
    let co = a.cos();
    let si = a.sin();
    let mut out = [Vec2::ZERO; 5];
    for i in 0..5 {
        let u = t[i].x - c.x;
        let v = t[i].y - c.y;
        out[i] = Vec2::new(u * co - v * si + c.x, v * co + u * si + c.y);
    }
    out
}

/// Applet transform: mirror y about the line y = cy.
fn mirror_y(t: [Vec2; 5], cy: f64) -> [Vec2; 5] {
    let mut out = [Vec2::ZERO; 5];
    for i in 0..5 {
        out[i] = Vec2::new(t[i].x, 2.0 * cy - t[i].y);
    }
    out
}

/// Applet `getangle4`: direction angle atan2(dy, dx) of p→q.
fn getangle4(p: Vec2, q: Vec2) -> f64 {
    (q.y - p.y).atan2(q.x - p.x)
}

/// Applet `getangle6`: angle from direction p→q to direction p→r.
fn getangle6(p: Vec2, q: Vec2, r: Vec2) -> f64 {
    getangle4(p, r) - getangle4(p, q)
}

/// The eight raw cell tiles exactly as the applet constructs them
/// (tile 0 = the base prototile; tiles 1..8 per [`DESC`]).  Bit-exact
/// with the verified study construction.
pub(crate) fn raw_cells() -> [[Vec2; 5]; 8] {
    let base = prototile();
    let mut tiles = [base; 8];
    for i in 1..8 {
        let (v1, v2, src, k1, k2, mir) = DESC[i - 1];
        let a = tiles[src as usize];
        let tile = base;
        // Translate so that tile.v1 lands on a.k1 (dx/dy from the
        // untranslated base, before the translate).
        let dx = a[k1 as usize].x - tile[v1 as usize].x;
        let dy = a[k1 as usize].y - tile[v1 as usize].y;
        let tile = translate(tile, Vec2::new(dx, dy));
        let tile = if mir {
            let ang = getangle4(tile[v1 as usize], tile[v2 as usize]);
            let tile = rotate(tile, tile[v1 as usize], -ang);
            let tile = mirror_y(tile, tile[v1 as usize].y);
            let ang = getangle4(tile[v1 as usize], a[k2 as usize]);
            rotate(tile, tile[v1 as usize], ang)
        } else {
            let ang = getangle6(tile[v1 as usize], tile[v2 as usize], a[k2 as usize]);
            rotate(tile, tile[v1 as usize], ang)
        };
        tiles[i] = tile;
    }
    tiles
}

/// The primitive unit: the eight stored cells (mirrored ones
/// vertex-reversed so every polygon is CCW) and the lattice basis
/// computed from the construction: `w1 = raw[2].p3 − raw[0].p0`,
/// `w2raw = raw[5].p3 − raw[3].p1`; the raw determinant is negative,
/// so the shipped second basis vector is `−w2raw` (positive det).
fn unit_cells() -> ([Vec<Vec2>; 8], Vec2, Vec2) {
    let raw = raw_cells();
    let w1 = raw[2][3] - raw[0][0];
    let w2_raw = raw[5][3] - raw[3][1];
    let w2 = if w1.cross(w2_raw) < 0.0 {
        -w2_raw
    } else {
        w2_raw
    };
    let cells: [Vec<Vec2>; 8] = std::array::from_fn(|k| {
        let c = raw[k];
        let pts: [Vec2; 5] = if MIRROR[k] {
            [c[0], c[4], c[3], c[2], c[1]]
        } else {
            c
        };
        pts.to_vec()
    });
    (cells, w1, w2)
}

/// 2-fold rotation centre of the tiling: c0 = (1/4, 0).  The two
/// glides compose to the half-turn about this point, and the
/// half-turn centres are the four classes c0 + Λ/2.
#[cfg(test)]
pub(crate) fn two_fold_centre() -> Vec2 {
    Vec2::new(0.25, 0.0)
}

/// Pentagon type 7 (Kershner 1968), published E = 106° member:
/// 2-isohedral, 8 tiles (four direct, four mirror) per rectangular
/// cell, wallpaper group pgg, edge-to-edge.
pub fn spec() -> LatticeTiling {
    let (cells, w1, w2) = unit_cells();
    LatticeTiling {
        id: "pentagon-7",
        name: "Pentagon type 7 (Kershner)",
        description: "Pentagonal tiling of type 7 (Richard Kershner, 1968): b = c = d = e, B + 2E = 2C + D = 360°, published E = 106° member (A ≈ 58.86°, B = 148°, C ≈ 132.86°, D ≈ 94.27°, a ≈ 1.0357942, b = c = d = e = 1/2; prototile computed at runtime — the family's published members are not surd angles); edge-to-edge (every side is matched by exactly one full partner side), 2-isohedral (seeds 0/1), 8 tiles (four direct, four mirror) per rectangular cell, wallpaper group pgg (two perpendicular glide families with axes along the lattice directions, squaring to w1 and −w2, no mirrors).",
        lattice: [w1, w2],
        cells: cells
            .into_iter()
            .zip(SEEDS)
            .map(|(pts, seed)| Cell {
                poly: Polygon::from_slice(&pts),
                seed,
            })
            .collect(),
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
