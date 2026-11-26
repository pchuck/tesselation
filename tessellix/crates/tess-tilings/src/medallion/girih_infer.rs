//! 24b sprint-2 — the boundary-matched inference engine of [`super::infer`]
//! generalized onto the **shipped L10 girih lattice** (24b gap map: the
//! engine spoke only the WIS-12 ℤ[√3] arithmetic; the girih substrates
//! live in L10 = ℚ(α), α = 2cos(π/10), the /64 convention of the landed
//! Darb-i Imam patch tables).
//!
//! **Input contract (general path, same shape as [`super::infer`]).**
//! The engine consumes only
//!
//! * a hole polygon (CCW, exact L10//64 points) — for the M161 bar the
//!   fixture extracts it from the landed patch's unmatched directed
//!   edges (a single 530-edge unit cycle; extraction is fixture-side per
//!   the `holes.rs` precedent, the engine never sees the cell tables), and
//! * the prototile alphabet supplied by the substrate: the three girih
//!   prototiles `DEC` (regular unit decagon), `HEX` (equilateral bobbin
//!   hexagon — *not* regular) and `BOW` (bow-tie), all unit-edge over
//!   L10//64, with the 36° rotation table `ROTM` of the shipped patch.
//!
//! **Constraint levels.** This engine runs the structural core C1
//! (boundary covered exactly once), C3 (edge-to-edge, inside-closure,
//! wedge rule, pairwise interior-disjoint — exact) and C4 (exact L10
//! 2-area monotonicity + identity). It has **no C2 anatomy level**: the
//! record-derived RayLine / RayLineVset / Segment levels of
//! [`super::infer`] are features of the WIS-12 star record. Measured on
//! the landed M161 patch: **0 of 1810 interior edges** lie on any
//! carried exit-ray or {10/3} chord line of the M161 record's hull
//! readings, and 0 equal a record segment — the girih substitution
//! absorbs its strapwork (see the "ray absorption" note in
//! [`super::girih_m161`]), so the record's carried lines are
//! boundary annotations, not interior generators. The girih path
//! therefore requests the `Plain`-equivalent stack only; the WIS engine
//! keeps its anatomy levels untouched.
//!
//! **Underdetermination is a first-class output** (as in
//! [`super::infer`]): the search returns the documented freedom set up
//! to deterministic caps; no RNG, every iteration over total orders,
//! re-runs byte-identical.
//!
//! **Exact arithmetic.** Elements of ℤ[α] as `i128` 4-tuples at the
//! fixed denominator 64 (the shipped `Kp`/`Ke` convention; α⁴ = 5α² − 5).
//! Sign decisions use raw (unscaled) products — denominator-invariant —
//! through a self-contained exact sign
//! ([`sgn`]: descent to ℚ(√5) via ξ = U + αV, N = U² − α²V²).
//! `crate::fields::L10::sign` is deliberately **not** used at search
//! coefficient heights: its certified operating range covers only small
//! coefficients (documented residual (a) of the 24b gap map). The
//! descent here is verified against exact float evaluation over the
//! whole box [−9,9]⁴ plus 2·10⁵ random tuples up to ±500
//! (`sgnl10_matches_float` in the test battery). f64 appears only in
//! the conservative bbox prefilter (slack 0.02) and analysis floats —
//! every accept/reject is exact.

use std::collections::{BTreeMap, BTreeSet};

// ------------------------------------------------------------------
// exact L10 arithmetic at /64 (self-contained)
// ------------------------------------------------------------------

/// An element of ℤ[α], α = 2cos(π/10), stored at the fixed
/// denominator 64: value = (t[0] + t[1]α + t[2]α² + t[3]α³)/64.
pub type K4 = [i128; 4];

/// A plane point: two L10//64 coordinates.
pub type GirihP = (K4, K4);

/// The zero element.
pub const KZERO: K4 = [0, 0, 0, 0];

#[inline]
pub fn kadd(x: K4, y: K4) -> K4 {
    [x[0] + y[0], x[1] + y[1], x[2] + y[2], x[3] + y[3]]
}

#[inline]
pub fn ksub(x: K4, y: K4) -> K4 {
    [x[0] - y[0], x[1] - y[1], x[2] - y[2], x[3] - y[3]]
}

/// Product in ℤ[α] without the /64 rescale (raw: represents the
/// product of the two stored numerators times 64²). Sign-only use.
#[inline]
pub fn kmul_raw(x: K4, y: K4) -> K4 {
    let p0 = x[0] * y[0];
    let p1 = x[0] * y[1] + x[1] * y[0];
    let p2 = x[0] * y[2] + x[1] * y[1] + x[2] * y[0];
    let p3 = x[0] * y[3] + x[1] * y[2] + x[2] * y[1] + x[3] * y[0];
    let p4 = x[1] * y[3] + x[2] * y[2] + x[3] * y[1];
    let p5 = x[2] * y[3] + x[3] * y[2];
    let p6 = x[3] * y[3];
    // reductions: a^4 = 5a^2 - 5, a^5 = 5a^3 - 5a, a^6 = 20a^2 - 25
    [
        p0 - 5 * p4 - 25 * p6,
        p1 - 5 * p5,
        p2 + 5 * p4 + 20 * p6,
        p3 + 5 * p5,
    ]
}

/// Product at the closed denominator 64 (the shipped convention: every
/// product of table values in this lattice re-closes at /64; the
/// debug assertion enforces it exactly as the landed patch tables do).
#[inline]
pub fn kmul(x: K4, y: K4) -> K4 {
    let r = kmul_raw(x, y);
    debug_assert!(
        r[0] % 64 == 0 && r[1] % 64 == 0 && r[2] % 64 == 0 && r[3] % 64 == 0,
        "K product left the fixed denominator"
    );
    [r[0] / 64, r[1] / 64, r[2] / 64, r[3] / 64]
}

#[inline]
pub fn padd(p: GirihP, q: GirihP) -> GirihP {
    (kadd(p.0, q.0), kadd(p.1, q.1))
}

#[inline]
pub fn psub(p: GirihP, q: GirihP) -> GirihP {
    (ksub(p.0, q.0), ksub(p.1, q.1))
}

/// Raw dot product (sign-only comparisons; denominator-invariant).
#[inline]
pub fn dot2(u: GirihP, v: GirihP) -> K4 {
    kadd(kmul_raw(u.0, v.0), kmul_raw(u.1, v.1))
}

/// Raw cross product (sign-only comparisons).
#[inline]
pub fn cross2(u: GirihP, v: GirihP) -> K4 {
    ksub(kmul_raw(u.0, v.1), kmul_raw(u.1, v.0))
}

/// Dot product at the closed denominator (unit-edge value checks,
/// cos² classes: unit-edge inputs give the exact cosine at /64).
#[inline]
pub fn dot64(u: GirihP, v: GirihP) -> K4 {
    kadd(kmul(u.0, v.0), kmul(u.1, v.1))
}

/// Cross product at the closed denominator (exact 2-area shoelace).
#[inline]
pub fn cross64(u: GirihP, v: GirihP) -> K4 {
    ksub(kmul(u.0, v.1), kmul(u.1, v.0))
}

/// Exact sign of an element of ℚ(α): zero iff the tuple is zero (the
/// powers 1, α, α², α³ are a ℚ-basis). Descent: ξ = U + αV with
/// U, V ∈ ℚ(√5), U = (2c₀+5c₂ + c₂√5)/2, V = (2c₁+5c₃ + c₃√5)/2;
/// when U and V have opposite signs, sgn ξ is ±sgn N for
/// N = U² − α²V² ∈ ℚ(√5). Exact integer arithmetic throughout.
pub fn sgn(x: K4) -> i8 {
    // sign of p + q√5, exact.
    fn sgn5(p: i128, q: i128) -> i8 {
        if p >= 0 && q >= 0 {
            return if p != 0 || q != 0 { 1 } else { 0 };
        }
        if p <= 0 && q <= 0 {
            return if p != 0 || q != 0 { -1 } else { 0 };
        }
        let t = p * p - 5 * q * q;
        if p > 0 {
            // p > 0 > q: p + q√5 > 0  iff  p² > 5q²
            if t > 0 { 1 } else { -1 }
        } else {
            // p < 0 < q: p + q√5 > 0  iff  5q² > p²  (t < 0)
            if t < 0 { 1 } else { -1 }
        }
    }
    let u0 = 2 * x[0] + 5 * x[2];
    let u1 = x[2];
    let v0 = 2 * x[1] + 5 * x[3];
    let v1 = x[3];
    let su = sgn5(u0, u1);
    let sv = sgn5(v0, v1);
    if sv == 0 {
        return su;
    }
    if su == 0 {
        return sv;
    }
    if su > 0 && sv > 0 {
        return 1;
    }
    if su < 0 && sv < 0 {
        return -1;
    }
    // N·8 = (2Au − 5A′ − 5B′) + (2Bu − A′ − 5B′)√5, where
    // Au = u0² + 5u1², Bu = 2u0u1, A′ = v0² + 5v1², B′ = 2v0v1
    // (the cross term of V²·α², α² = (5+√5)/2, carries A′ + 5B′).
    let au = u0 * u0 + 5 * u1 * u1;
    let bu = 2 * u0 * u1;
    let ap = v0 * v0 + 5 * v1 * v1;
    let bp = 2 * v0 * v1;
    let n = sgn5(2 * au - 5 * ap - 5 * bp, bu * 2 - ap - 5 * bp);
    if su > 0 { n } else { -n }
}

// ------------------------------------------------------------------
// prototile alphabet + 36° rotation lattice (shipped patch tables)
// ------------------------------------------------------------------

pub const HEX_V: [GirihP; 6] = [
    ([-32,0,32,0],[0,0,0,0]),
    ([32,0,0,0],[0,-96,0,32]),
    ([-32,0,0,0],[0,-96,0,32]),
    ([32,0,-32,0],[0,0,0,0]),
    ([-32,0,0,0],[0,96,0,-32]),
    ([32,0,0,0],[0,96,0,-32]),
];
pub const BOW_V: [GirihP; 6] = [
    ([0,32,0,0],[32,0,0,0]),
    ([0,0,0,0],[128,0,-32,0]),
    ([0,-32,0,0],[32,0,0,0]),
    ([0,-32,0,0],[-32,0,0,0]),
    ([0,0,0,0],[-128,0,32,0]),
    ([0,32,0,0],[-32,0,0,0]),
];
pub const DEC_V: [GirihP; 10] = [
    ([-128,0,64,0],[0,0,0,0]),
    ([-32,0,32,0],[0,32,0,0]),
    ([32,0,0,0],[0,-64,0,32]),
    ([-32,0,0,0],[0,-64,0,32]),
    ([32,0,-32,0],[0,32,0,0]),
    ([128,0,-64,0],[0,0,0,0]),
    ([32,0,-32,0],[0,-32,0,0]),
    ([-32,0,0,0],[0,64,0,-32]),
    ([32,0,0,0],[0,64,0,-32]),
    ([-32,0,32,0],[0,-32,0,0]),
];
const ROTM: [((K4, K4), (K4, K4)); 20] = [
    (([64,0,0,0],[0,0,0,0]),([0,0,0,0],[64,0,0,0])),
    (([0,32,0,0],[96,0,-32,0]),([-96,0,32,0],[0,32,0,0])),
    (([-64,0,32,0],[0,96,0,-32]),([0,-96,0,32],[-64,0,32,0])),
    (([0,-96,0,32],[64,0,-32,0]),([-64,0,32,0],[0,-96,0,32])),
    (([-96,0,32,0],[0,-32,0,0]),([0,32,0,0],[-96,0,32,0])),
    (([0,0,0,0],[-64,0,0,0]),([64,0,0,0],[0,0,0,0])),
    (([96,0,-32,0],[0,-32,0,0]),([0,32,0,0],[96,0,-32,0])),
    (([0,96,0,-32],[64,0,-32,0]),([-64,0,32,0],[0,96,0,-32])),
    (([64,0,-32,0],[0,96,0,-32]),([0,-96,0,32],[64,0,-32,0])),
    (([0,-32,0,0],[96,0,-32,0]),([-96,0,32,0],[0,-32,0,0])),
    (([-64,0,0,0],[0,0,0,0]),([0,0,0,0],[-64,0,0,0])),
    (([0,-32,0,0],[-96,0,32,0]),([96,0,-32,0],[0,-32,0,0])),
    (([64,0,-32,0],[0,-96,0,32]),([0,96,0,-32],[64,0,-32,0])),
    (([0,96,0,-32],[-64,0,32,0]),([64,0,-32,0],[0,96,0,-32])),
    (([96,0,-32,0],[0,32,0,0]),([0,-32,0,0],[96,0,-32,0])),
    (([0,0,0,0],[64,0,0,0]),([-64,0,0,0],[0,0,0,0])),
    (([-96,0,32,0],[0,32,0,0]),([0,-32,0,0],[-96,0,32,0])),
    (([0,-96,0,32],[-64,0,32,0]),([64,0,-32,0],[0,-96,0,32])),
    (([-64,0,32,0],[0,-96,0,32]),([0,96,0,-32],[-64,0,32,0])),
    (([0,32,0,0],[-96,0,32,0]),([96,0,-32,0],[0,32,0,0])),
];

/// Bobbin hexagon (equilateral, **not** regular: edge directions are
/// not all 60° apart, so it has no rotational symmetry that fixes a
/// directed edge — a directed edge admits 3 distinct placements).
pub const HEX: u8 = 1;
/// Bow-tie.
pub const BOW: u8 = 2;
/// Regular decagon (36° rotational symmetry: a directed edge admits
/// exactly 1 placement).
pub const DEC: u8 = 3;

/// Rotation by r·36° (r mod 20), exact at /64.
pub fn rot(r: usize, p: GirihP) -> GirihP {
    let m = ROTM[r % 20];
    (
        kadd(kmul(m.0.0, p.0), kmul(m.0.1, p.1)),
        kadd(kmul(m.1.0, p.0), kmul(m.1.1, p.1)),
    )
}

/// Direction class of a unit vector: the r with v = rot(r)·(1,0), or
/// None if v is not one of the 20 canonical girih directions.
pub fn dir_class(v: GirihP) -> Option<usize> {
    for r in 0..20 {
        if v.0 == ROTM[r].0.0 && v.1 == ROTM[r].1.0 {
            return Some(r);
        }
    }
    None
}

/// Vertex table of a prototile kind (CCW, unit edges).
pub fn proto_verts(kind: u8) -> &'static [GirihP] {
    match kind {
        HEX => &HEX_V,
        BOW => &BOW_V,
        _ => &DEC_V,
    }
}

/// Exact shoelace 2-area at /64 of a CCW polygon of lattice points.
pub fn area2(poly: &[GirihP]) -> K4 {
    let n = poly.len();
    let mut s = KZERO;
    for i in 0..n {
        s = kadd(s, cross64(poly[i], poly[(i + 1) % n]));
    }
    s
}

/// Exact 2-area of each prototile (pinned; the test battery checks
/// these equal `area2(proto_verts(kind))`).
pub fn tile_area2(kind: u8) -> K4 {
    match kind {
        HEX => [0, -320, 0, 128],
        BOW => [0, 320, 0, -64],
        _ => [0, -640, 0, 320],
    }
}

// ------------------------------------------------------------------
// placed tiles
// ------------------------------------------------------------------

const ALPHA_F: f64 = 1.902113032590307;

/// Float value of an L10 element (analysis + prefilter only — never a
/// decision input).
pub fn kvar(t: K4) -> f64 {
    (t[0] as f64 + t[1] as f64 * ALPHA_F + t[2] as f64 * (ALPHA_F * ALPHA_F)
        + t[3] as f64 * (ALPHA_F * ALPHA_F * ALPHA_F))
        / 64.0
}

/// X/Y of a lattice point (analysis + prefilter only).
pub fn pxy(p: GirihP) -> (f64, f64) {
    (kvar(p.0), kvar(p.1))
}

/// A placed girih prototile (6 or 10 corners, CCW, unit edges).
#[derive(Clone, Copy, Debug)]
pub struct GirihTile {
    pub kind: u8,
    pub n: usize,
    pub cs: [GirihP; 10],
    /// world bbox `[minx, maxx, miny, maxy]` — prefilter only.
    pub bbox: [f64; 4],
}

impl GirihTile {
    pub fn new(kind: u8, cs: &[GirihP]) -> GirihTile {
        let n = cs.len();
        assert!(n == 6 || n == 10, "girih prototile needs 6 or 10 corners");
        let mut arr = [(KZERO, KZERO); 10];
        arr[..n].copy_from_slice(cs);
        // [minx, maxx, miny, maxy] — max-slots must start at NEG_INFINITY.
        let mut bbox = [f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY];
        for &p in &arr[..n] {
            let (x, y) = pxy(p);
            bbox[0] = bbox[0].min(x);
            bbox[1] = bbox[1].max(x);
            bbox[2] = bbox[2].min(y);
            bbox[3] = bbox[3].max(y);
        }
        GirihTile { kind, n, cs: arr, bbox }
    }
}

#[inline]
fn bbox_disjoint(a: &GirihTile, b: &GirihTile) -> bool {
    const SLACK: f64 = 0.02;
    b.bbox[1] < a.bbox[0] - SLACK
        || b.bbox[0] > a.bbox[1] + SLACK
        || b.bbox[3] < a.bbox[2] - SLACK
        || b.bbox[2] > a.bbox[3] + SLACK
}

/// Undirected edge key: the sorted corner pair (total order, no hash).
pub type EK = (GirihP, GirihP);

#[inline]
pub fn edge_key(a: GirihP, b: GirihP) -> EK {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

/// Cyclic-rotation canonical form of a polygon vertex tuple (the
/// prototile placements for one directed edge can realize the same
/// polygon through several (kind, edge-index, rotation) alignments —
/// the bobbin hexagon k ↦ k+3-style coincidences — so candidates are
/// deduplicated by canonical polygon, not by alignment index).
fn canon_poly(cs: &[GirihP]) -> Vec<GirihP> {
    let n = cs.len();
    let mut best: Option<Vec<GirihP>> = None;
    for i in 0..n {
        let r: Vec<GirihP> = (0..n).map(|j| cs[(i + j) % n]).collect();
        if best.as_ref().is_none_or(|b| r < *b) {
            best = Some(r);
        }
    }
    best.expect("nonempty")
}

/// Candidate placements with the directed unit edge `a→b` on the
/// boundary, tile on the left (CCW): every prototile edge alignment,
/// deduplicated by canonical polygon, fixed order (HEX, BOW, DEC;
/// edge index; first alignment wins). Exactly 7 placements survive for
/// every direction class of the M161 hole (measured histogram
/// `{7: 530}`): DEC 1 (36° symmetry collapses 10 alignments), HEX 3
/// (bobbin edge phases), BOW 3.
pub fn candidates(a: GirihP, b: GirihP) -> Vec<GirihTile> {
    let d = psub(b, a);
    let cls = dir_class(d).expect("candidate edge must be a canonical girih direction");
    let mut out: Vec<GirihTile> = Vec::new();
    let mut seen: BTreeSet<(u8, Vec<GirihP>)> = BTreeSet::new();
    for kind in [HEX, BOW, DEC] {
        let vs = proto_verts(kind);
        let n = vs.len();
        for k in 0..n {
            let ecls = dir_class(psub(vs[(k + 1) % n], vs[k]))
                .expect("prototile edge must be a canonical girih direction");
            let r = (cls + 20 - ecls) % 20;
            let vsr: Vec<GirihP> = vs.iter().map(|&v| rot(r, v)).collect();
            let cs: Vec<GirihP> = vsr.iter().map(|&v| padd(a, psub(v, vsr[k]))).collect();
            assert!(
                (0..n).any(|i| cs[i] == a && cs[(i + 1) % n] == b),
                "candidate lost its directed edge"
            );
            if !seen.insert((kind, canon_poly(&cs))) {
                continue;
            }
            out.push(GirihTile::new(kind, &cs));
        }
    }
    out
}

// ------------------------------------------------------------------
// geometry predicates (exact; mirror of the WIS engine)
// ------------------------------------------------------------------

fn seg_side(a: GirihP, b: GirihP, p: GirihP) -> i8 {
    sgn(cross2(psub(b, a), psub(p, a)))
}

/// `p` on the closed segment `ab` (exact).
pub fn on_seg(a: GirihP, b: GirihP, p: GirihP) -> bool {
    if seg_side(a, b, p) != 0 {
        return false;
    }
    let d = psub(b, a);
    let e = psub(p, a);
    let f = psub(d, e);
    sgn(dot2(d, e)) >= 0 && sgn(dot2(d, f)) >= 0
}

/// Proper (interior×interior) crossing of segments `ab` and `cd`.
pub fn segs_proper_cross(a: GirihP, b: GirihP, c: GirihP, d: GirihP) -> bool {
    let d1 = seg_side(a, b, c) as i32;
    let d2 = seg_side(a, b, d) as i32;
    let d3 = seg_side(c, d, a) as i32;
    let d4 = seg_side(c, d, b) as i32;
    d1 * d2 < 0 && d3 * d4 < 0
}

/// Exact world-y scanline crossing count (point assumed known not to
/// lie on any boundary edge here; a collinear straddling edge still
/// returns false as a safety net). Works for the hole polygon and for
/// placed tile polygons alike.
fn scanline(p: GirihP, poly: &[GirihP]) -> bool {
    let n = poly.len();
    let mut cnt = 0usize;
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        let say = sgn(ksub(a.1, p.1));
        let sby = sgn(ksub(b.1, p.1));
        if (say > 0) == (sby > 0) {
            continue;
        }
        let d = psub(b, a);
        let c = cross2(d, psub(a, p));
        let sdy = sgn(d.1);
        let cs = sgn(c);
        if cs == 0 {
            return false;
        }
        if (cs as i32) * (sdy as i32) < 0 {
            cnt += 1;
        }
    }
    cnt % 2 == 1
}

/// Exact world-y scanline point-in-polygon (boundary excluded); all
/// signs exact via [`sgn`]. Works for the hole polygon and for placed
/// tile polygons alike.
pub fn strictly_inside(p: GirihP, poly: &[GirihP]) -> bool {
    let n = poly.len();
    for i in 0..n {
        if on_seg(poly[i], poly[(i + 1) % n], p) {
            return false;
        }
    }
    scanline(p, poly)
}

// ------------------------------------------------------------------
/// Uniform-cell size of the conservative spatial grids (world units).
const CELL: f64 = 2.0;

/// Cell index of a coordinate.
#[inline]
fn cell_of(x: f64) -> i64 {
    (x / CELL).floor() as i64
}

/// Inclusive cell span `[lo, hi]` of the window `[e0 - slack, e1 + slack]`
/// (the slack also absorbs float evaluation error; at these coordinate
/// magnitudes it is ~1e7× larger). `lo <= hi` always holds.
#[inline]
fn cell_range(e0: f64, e1: f64, slack: f64) -> (i64, i64) {
    (cell_of(e0 - slack), cell_of(e1 + slack).max(cell_of(e0 - slack)))
}

/// Bucketing of items by the grid cells their f64 bbox intersects.
/// [`CellGrid::query`] returns a superset of the items whose bbox
/// intersects the query window (+slack): every bbox-crossing cell of an
/// item lies inside the queried span, and any cell an item touches lies
/// inside its own span. Callers therefore keep exact semantics —
/// duplicates are harmless (idempotent re-tests), the grid is a pure
/// prefilter, never a decision.
struct CellGrid {
    cells: BTreeMap<(i64, i64), Vec<u32>>,
}

impl CellGrid {
    fn new() -> CellGrid {
        CellGrid { cells: BTreeMap::new() }
    }

    fn insert_bb(&mut self, ix: u32, bb: &[f64; 4]) {
        let (x0, x1) = cell_range(bb[0], bb[1], 0.0);
        let (y0, y1) = cell_range(bb[2], bb[3], 0.0);
        for cx in x0..=x1 {
            for cy in y0..=y1 {
                self.cells.entry((cx, cy)).or_default().push(ix);
            }
        }
    }

    /// Indices of all items in the cells covering the window
    /// `[x0−slack, x1+slack] × [y0−slack, y1+slack]` (with slack, a
    /// conservative superset of the bbox-intersecting items).
    fn query(&self, x0: f64, x1: f64, y0: f64, y1: f64, slack: f64) -> impl Iterator<Item = u32> + '_ {
        let (a0, a1) = cell_range(x0, x1, slack);
        let (b0, b1) = cell_range(y0, y1, slack);
        (a0..=a1).flat_map(move |cx| {
            (b0..=b1).flat_map(move |cy| {
                self.cells.get(&(cx, cy)).into_iter().flat_map(|v| v.iter().copied())
            })
        })
    }
}

/// A conservative bucketing of the placed tiles for [`valid_placement`]:
/// a candidate's pairwise scans consult only tiles sharing a grid cell
/// with its bbox — a superset of the bbox-overlapping tiles, hence
/// byte-identical outcomes to the linear scan (the grid is a pure
/// conservative prefilter; `None` selects the linear scan).
#[derive(Default)]
pub struct TileBuckets {
    cells: BTreeMap<(i64, i64), Vec<u32>>,
}

impl TileBuckets {
    fn insert_bb(&mut self, ix: u32, bb: &[f64; 4]) {
        let (x0, x1) = cell_range(bb[0], bb[1], 0.0);
        let (y0, y1) = cell_range(bb[2], bb[3], 0.0);
        for cx in x0..=x1 {
            for cy in y0..=y1 {
                self.cells.entry((cx, cy)).or_default().push(ix);
            }
        }
    }

    /// Index of `ix` inside its cells (position-independent: removal
    /// swaps the last entry into the hole, indices of other entries
    /// move but bucket membership — all the query contract needs —
    /// is preserved).
    pub fn insert(&mut self, ix: u32, bbox: &[f64; 4]) {
        self.insert_bb(ix, bbox);
    }

    pub fn remove(&mut self, ix: u32, bbox: &[f64; 4]) {
        let (x0, x1) = cell_range(bbox[0], bbox[1], 0.0);
        let (y0, y1) = cell_range(bbox[2], bbox[3], 0.0);
        for cx in x0..=x1 {
            for cy in y0..=y1 {
                if let Some(v) = self.cells.get_mut(&(cx, cy)) {
                    if let Some(p) = v.iter().position(|&x| x == ix) {
                        v.swap_remove(p);
                    }
                }
            }
        }
    }

    /// Indices of all placed tiles in the cells covering the window
    /// `[x0−slack, x1+slack] × [y0−slack, y1+slack]` (with slack, a
    /// conservative superset of the bbox-overlapping tiles).
    pub fn query(&self, x0: f64, x1: f64, y0: f64, y1: f64, slack: f64) -> impl Iterator<Item = u32> + '_ {
        let (a0, a1) = cell_range(x0, x1, slack);
        let (b0, b1) = cell_range(y0, y1, slack);
        (a0..=a1).flat_map(move |cx| {
            (b0..=b1).flat_map(move |cy| {
                self.cells.get(&(cx, cy)).into_iter().flat_map(|v| v.iter().copied())
            })
        })
    }
}

/// The hole-bearing girih substrate: a single CCW unit-edge hole
/// polygon in exact L10//64 coordinates.
///
/// Alongside the exact data it caches *conservative* f64 uniform cell
/// grids (cell [`CELL`], query slack [`BB_SLACK`] ≫ any float
/// evaluation error at these coordinate magnitudes) over the hole
/// edges and hole vertices, plus a float bbox of the polygon. They
/// prefilter the exact scans in [`valid_placement`]: a skipped exact
/// test is provably false (grid queries return supersets of the
/// geometrically relevant items), so every accept/reject decision
/// stays byte-identical to the naive full scans — the same discipline
/// as the WIS engine's `bbox_disjoint` prefilter, at the granularity
/// the 530-edge M161 hole needs. (The hole bbox is ~70×70 world
/// units, so a pure bbox prefilter would never prune; cells do.)
pub struct GirihSubstrate {
    pub poly: Vec<GirihP>,
    pub hole_vset: BTreeSet<GirihP>,
    pub edges: Vec<(GirihP, GirihP)>,
    pub edge_set: BTreeSet<EK>,
    pub edge_dir: BTreeMap<EK, (GirihP, GirihP)>,
    nxt: BTreeMap<GirihP, GirihP>,
    prv: BTreeMap<GirihP, GirihP>,
    poly2: Vec<GirihP>,
    edges2: Vec<(GirihP, GirihP)>,
    pub hole_area: K4,
    /// hole-edge indices bucketed by cell (prefilter).
    egrid: CellGrid,
    /// hole vertices bucketed by cell (prefilter for the wedge rule).
    vgrid: CellGrid,
    vtx: Vec<GirihP>,
    /// float bbox of the hole polygon (prefilter).
    pbb: [f64; 4],
}

/// Conservative slack for the f64 grid/bbox prefilters (coordinate
/// magnitudes here are < 1e3, float evaluation error < 1e-9).
const BB_SLACK: f64 = 0.02;

#[inline]
fn bb_of(p: GirihP) -> (f64, f64) {
    pxy(p)
}

#[inline]
fn pt_outside_bb(x: f64, y: f64, bb: &[f64; 4], slack: f64) -> bool {
    x < bb[0] - slack || x > bb[1] + slack || y < bb[2] - slack || y > bb[3] + slack
}

impl GirihSubstrate {
    pub fn new(poly: Vec<GirihP>) -> GirihSubstrate {
        let nv = poly.len();
        assert!(nv >= 3, "degenerate hole polygon");
        let area = area2(&poly);
        assert!(sgn(area) > 0, "hole polygon must be CCW");
        let hole_vset: BTreeSet<GirihP> = poly.iter().copied().collect();
        assert_eq!(hole_vset.len(), nv, "duplicate hole vertex");
        let edges: Vec<(GirihP, GirihP)> =
            (0..nv).map(|i| (poly[i], poly[(i + 1) % nv])).collect();
        let mut edge_set = BTreeSet::new();
        let mut edge_dir = BTreeMap::new();
        let mut nxt = BTreeMap::new();
        let mut prv = BTreeMap::new();
        for &(a, b) in &edges {
            edge_set.insert(edge_key(a, b));
            edge_dir.insert(edge_key(a, b), (a, b));
            nxt.insert(a, b);
            prv.insert(b, a);
        }
        assert_eq!(edge_set.len(), nv, "hole boundary is not a simple cycle");
        let poly2: Vec<GirihP> = poly.iter().map(|&p| padd(p, p)).collect();
        let edges2: Vec<(GirihP, GirihP)> =
            edges.iter().map(|&(a, b)| (padd(a, a), padd(b, b))).collect();
        let mut egrid = CellGrid::new();
        for (k, &(a, b)) in edges.iter().enumerate() {
            let (ax, ay) = bb_of(a);
            let (bx, by) = bb_of(b);
            egrid.insert_bb(k as u32, &[ax.min(bx), ax.max(bx), ay.min(by), ay.max(by)]);
        }
        let vtx: Vec<GirihP> = hole_vset.iter().copied().collect();
        let mut vgrid = CellGrid::new();
        let mut pbb = [f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY];
        for (k, &w) in vtx.iter().enumerate() {
            let (wx, wy) = bb_of(w);
            vgrid.insert_bb(k as u32, &[wx, wx, wy, wy]);
            pbb[0] = pbb[0].min(wx);
            pbb[1] = pbb[1].max(wx);
            pbb[2] = pbb[2].min(wy);
            pbb[3] = pbb[3].max(wy);
        }
        GirihSubstrate {
            poly,
            hole_vset,
            edges,
            edge_set,
            edge_dir,
            nxt,
            prv,
            poly2,
            edges2,
            hole_area: area,
            egrid,
            vgrid,
            vtx,
            pbb,
        }
    }

    /// Hole-closure membership of a point: on a hole edge (exact,
    /// grid-prefiltered) or strictly inside the hole (exact scanline,
    /// bbox-shortcut when conservatively outside).
    pub fn inside_or_on_hole(&self, p: GirihP) -> bool {
        let (px, py) = bb_of(p);
        if pt_outside_bb(px, py, &self.pbb, BB_SLACK) {
            return false;
        }
        for k in self.egrid.query(px, px, py, py, BB_SLACK) {
            let (a, b) = self.edges[k as usize];
            if on_seg(a, b, p) {
                return true;
            }
        }
        scanline(p, &self.poly)
    }

    /// Conservative grid membership test for the DOUBLED midpoint `m`
    /// against the doubled polygon edges (equivalent to the doubled
    /// exact test up to the positive scale ½: the exact predicate is
    /// sign-homogeneous; float halves carry only evaluation error,
    /// dominated by `BB_SLACK` here and by the doubled slack below).
    fn mid_on_hole_edge(&self, m: GirihP) -> bool {
        let (hx, hy) = (bb_of(m).0 / 2.0, bb_of(m).1 / 2.0);
        if pt_outside_bb(hx, hy, &self.pbb, BB_SLACK) {
            return false;
        }
        for k in self.egrid.query(hx, hx, hy, hy, BB_SLACK) {
            let (a2, b2) = self.edges2[k as usize];
            if on_seg(a2, b2, m) {
                return true;
            }
        }
        false
    }

    /// Grid membership of a hole vertex in the window around the
    /// segment bbox `u..v` (conservative superset).
    fn hole_verts_near(&self, u: GirihP, v: GirihP) -> impl Iterator<Item = GirihP> + '_ {
        let (ux, uy) = bb_of(u);
        let (vx, vy) = bb_of(v);
        let (x0, x1) = (ux.min(vx), ux.max(vx));
        let (y0, y1) = (uy.min(vy), uy.max(vy));
        self.vgrid
            .query(x0, x1, y0, y1, BB_SLACK)
            .map(move |k| self.vtx[k as usize])
    }

    /// Hole edges whose bbox may meet the segment bbox `u..v`
    /// (conservative superset).
    fn hole_edges_near(&self, u: GirihP, v: GirihP) -> impl Iterator<Item = (GirihP, GirihP)> + '_ {
        let (ux, uy) = bb_of(u);
        let (vx, vy) = bb_of(v);
        let (x0, x1) = (ux.min(vx), ux.max(vx));
        let (y0, y1) = (uy.min(vy), uy.max(vy));
        self.egrid
            .query(x0, x1, y0, y1, BB_SLACK)
            .map(move |k| self.edges[k as usize])
    }
}

// ------------------------------------------------------------------
// placement validity (C3, exact; Plain level — no vset/line/segment)
// ------------------------------------------------------------------

fn mid_inside_or_on(sub: &GirihSubstrate, u: GirihP, v: GirihP) -> bool {
    let m = padd(u, v); // doubled midpoint stays integral
    if sub.mid_on_hole_edge(m) {
        return true;
    }
    scanline(m, &sub.poly2)
}

/// A tile edge may pass through a hole vertex only in the wedge the
/// hole opens there (both tile-edge endpoints left-of-or-on the two
/// adjacent CCW hole edges).
fn edge_thru_hole_vertex_ok(sub: &GirihSubstrate, u: GirihP, v: GirihP) -> bool {
    for w in sub.hole_verts_near(u, v) {
        if w == u || w == v || !on_seg(u, v, w) {
            continue;
        }
        let p = sub.prv[&w];
        let q = sub.nxt[&w];
        for x in [u, v] {
            if x == w {
                continue;
            }
            if sgn(cross2(psub(w, p), psub(x, p))) < 0 {
                return false;
            }
            if sgn(cross2(psub(q, w), psub(x, w))) < 0 {
                return false;
            }
        }
    }
    true
}

/// C3 single-placement predicate (identical predicate list to the WIS
/// engine, minus the level-specific vset membership which has no
/// girih counterpart): hole-closure vertex membership, no proper
/// crossing of hole edges, midpoint inside-or-on, hole-vertex wedge
/// rule, rule (3) edge-to-edge vs the placed tiles (both directions),
/// pairwise interior disjointness. Exact except the conservative f64
/// grid/bbox prefilters ([`TileBuckets`], [`GirihSubstrate::hole_edges_near`],
/// [`GirihSubstrate::hole_verts_near`]).
pub fn valid_placement(
    sub: &GirihSubstrate,
    t: &GirihTile,
    placed: &[GirihTile],
    buckets: Option<&TileBuckets>,
) -> bool {
    let n = t.n;
    for i in 0..n {
        let v = t.cs[i];
        if !sub.hole_vset.contains(&v) && !sub.inside_or_on_hole(v) {
            return false;
        }
    }
    for i in 0..n {
        let (u, v) = (t.cs[i], t.cs[(i + 1) % n]);
        for (a, b) in sub.hole_edges_near(u, v) {
            if segs_proper_cross(u, v, a, b) {
                return false;
            }
        }
        if !mid_inside_or_on(sub, u, v) {
            return false;
        }
        if !edge_thru_hole_vertex_ok(sub, u, v) {
            return false;
        }
    }
    // Pairwise phase. With buckets the candidate consults only tiles
    // sharing a cell with its bbox (a conservative superset of the
    // bbox-overlapping tiles; re-testing a tile twice is idempotent,
    // so outcomes are identical to the linear scan).
    let mut scratch: Vec<GirihTile> = Vec::new();
    let near: &[GirihTile] = match buckets {
        None => placed,
        Some(b) => {
            for ix in b.query(t.bbox[0], t.bbox[1], t.bbox[2], t.bbox[3], BB_SLACK) {
                scratch.push(placed[ix as usize]);
            }
            &scratch
        }
    };
    for t2 in near.iter() {
        if bbox_disjoint(t, t2) {
            continue;
        }
        // rule (3): no vertex in another tile edge's relative interior
        for i in 0..n {
            let p = t.cs[i];
            for j in 0..t2.n {
                let (u, w) = (t2.cs[j], t2.cs[(j + 1) % t2.n]);
                if p != u && p != w && on_seg(u, w, p) {
                    return false;
                }
            }
        }
        for j in 0..t2.n {
            let p = t2.cs[j];
            for i in 0..n {
                let (u, w) = (t.cs[i], t.cs[(i + 1) % n]);
                if p != u && p != w && on_seg(u, w, p) {
                    return false;
                }
            }
        }
        // pairwise interior disjointness (exact)
        for i in 0..n {
            if strictly_inside(t.cs[i], &t2.cs[..t2.n]) {
                return false;
            }
        }
        for j in 0..t2.n {
            if strictly_inside(t2.cs[j], &t.cs[..n]) {
                return false;
            }
        }
        for i in 0..n {
            let (u, v) = (t.cs[i], t.cs[(i + 1) % n]);
            for j in 0..t2.n {
                let (c, d) = (t2.cs[j], t2.cs[(j + 1) % t2.n]);
                if segs_proper_cross(u, v, c, d) {
                    return false;
                }
            }
        }
    }
    true
}

// ------------------------------------------------------------------
// search (Plain-equivalent stack: C1 + C3 + C4; deterministic caps)
// ------------------------------------------------------------------

use super::infer::Limits;

/// Exact canonical fingerprint of a girih fill: the set of its tiles,
/// each (kind, corners rotated to start at the minimal corner).
pub type GirihFillFp = BTreeSet<(u8, Vec<GirihP>)>;

/// The documented output of one girih search run (same semantics as
/// [`super::infer::Report`], girih kinds/arithmetic).
#[derive(Debug, Clone)]
pub struct GirihReport {
    pub nodes: u64,
    pub placements: u64,
    /// false ⇒ a cap was hit (documented partial freedom set).
    pub exhausted: bool,
    pub sols: usize,
    pub distinct: usize,
    /// completions failing independent global validation (must stay 0).
    pub invalid: usize,
    /// max |component| over all completion vertices (finiteness).
    pub max_coord: i128,
    /// (hex, bow, dec) census → completions in that class.
    pub by_census: BTreeMap<[u32; 3], usize>,
    /// FNV-1a over the solution sequence (kinds + corners, in order).
    pub seq_hash: u64,
    /// exact canonical fingerprints of every distinct completion.
    pub fps: BTreeSet<GirihFillFp>,
}

pub struct GCtx<'a> {
    pub sub: &'a GirihSubstrate,
    pub limits: Limits,
    nodes: u64,
    placements: u64,
    aborted: bool,
    sols: usize,
    invalid: usize,
    max_coord: i128,
    by_census: BTreeMap<[u32; 3], usize>,
    seq_hash: u64,
    fps: BTreeSet<GirihFillFp>,
    /// conservative cell buckets of `placed` (search-local; mirrors the
    /// Vec's prefix, see [`TileBuckets`]).
    buckets: TileBuckets,
}

/// Run the deterministic backtracking search over the girih alphabet.
///
/// Depth note: like [`super::infer::search`], the backtracker recurses
/// once per placed tile, so a run whose completion fills hundreds of
/// tiles needs a stack proportional to the fill (a 611-tile completion
/// runs comfortably on a 512 MB stack; small holes such as the unit
/// decagon run fine on default stacks). Results are identical in every
/// stack context: the search is deterministic.
pub fn search(sub: &GirihSubstrate, limits: Limits) -> GirihReport {
    let mut ctx = GCtx {
        sub,
        limits,
        nodes: 0,
        placements: 0,
        aborted: false,
        sols: 0,
        invalid: 0,
        max_coord: 0,
        by_census: BTreeMap::new(),
        seq_hash: 0xcbf29ce484222325,
        fps: BTreeSet::new(),
        buckets: TileBuckets::default(),
    };
    let mut placed: Vec<GirihTile> = Vec::new();
    grec(&mut ctx, BTreeSet::new(), BTreeMap::new(), &mut placed, KZERO);
    GirihReport {
        nodes: ctx.nodes,
        placements: ctx.placements,
        exhausted: !ctx.aborted,
        sols: ctx.sols,
        distinct: ctx.fps.len(),
        invalid: ctx.invalid,
        max_coord: ctx.max_coord,
        by_census: ctx.by_census,
        seq_hash: ctx.seq_hash,
        fps: ctx.fps,
    }
}

fn grec(
    ctx: &mut GCtx,
    cov: BTreeSet<EK>,
    opn: BTreeMap<EK, (GirihP, GirihP)>,
    placed: &mut Vec<GirihTile>,
    ar: K4,
) {
    ctx.nodes += 1;
    if ctx.nodes > ctx.limits.max_nodes {
        ctx.aborted = true;
    }
    if ctx.aborted {
        return;
    }
    // Frontier: first uncovered boundary edge (CCW direction), else the
    // minimal open edge with the candidate on its empty side.
    let mut frontier: Option<(GirihP, GirihP)> = None;
    for ek in ctx.sub.edge_set.iter() {
        if !cov.contains(ek) {
            let &(a, b) = ctx.sub.edge_dir.get(ek).expect("hole edge dir");
            frontier = Some((a, b));
            break;
        }
    }
    let (a, b) = match frontier {
        Some(ab) => ab,
        None => match opn.iter().next() {
            Some((_, &(u, v))) => (v, u),
            None => {
                if sgn(ksub(ar, ctx.sub.hole_area)) == 0 {
                    grecord(ctx, placed);
                }
                return;
            }
        },
    };
    let ix = placed.len() as u32;
    for cand in candidates(a, b) {
        let na = kadd(ar, tile_area2(cand.kind));
        if sgn(ksub(na, ctx.sub.hole_area)) > 0 {
            continue; // C4: exact area monotonicity
        }
        ctx.placements += 1;
        if !valid_placement(ctx.sub, &cand, placed, Some(&ctx.buckets)) {
            continue;
        }
        let n = cand.n;
        let mut ok = true;
        let mut nc: Vec<EK> = Vec::new();
        let mut matched: Vec<EK> = Vec::new();
        let mut nop: Vec<(EK, (GirihP, GirihP))> = Vec::new();
        for i in 0..n {
            let (u, v) = (cand.cs[i], cand.cs[(i + 1) % n]);
            let ek = edge_key(u, v);
            if ctx.sub.edge_set.contains(&ek) {
                if cov.contains(&ek) {
                    ok = false;
                    break; // C1: boundary edge twice
                }
                nc.push(ek);
            } else if opn.contains_key(&ek) {
                matched.push(ek);
            } else {
                nop.push((ek, (u, v)));
            }
        }
        if !ok {
            continue;
        }
        let mut cov2 = cov.clone();
        cov2.extend(nc);
        let mut opn2 = opn.clone();
        for ek in matched {
            opn2.remove(&ek);
        }
        for (ek, d) in nop {
            opn2.insert(ek, d);
        }
        placed.push(cand);
        ctx.buckets.insert(ix, &cand.bbox);
        grec(ctx, cov2, opn2, placed, na);
        ctx.buckets.remove(ix, &cand.bbox);
        placed.pop();
        if ctx.aborted {
            return;
        }
    }
}

fn grecord(ctx: &mut GCtx, placed: &[GirihTile]) {
    ctx.sols += 1;
    if !global_validate(ctx.sub, placed).is_empty() {
        ctx.invalid += 1;
    }
    let an = analyze(placed);
    *ctx.by_census.entry(an.census).or_insert(0) += 1;
    ctx.fps.insert(an.fp);
    for t in placed {
        let mut bytes = Vec::with_capacity(1 + 8 * 16);
        bytes.push(t.kind);
        for &p in &t.cs[..t.n] {
            for &x in p.0.iter().chain(p.1.iter()) {
                ctx.max_coord = ctx.max_coord.max(x.abs());
                bytes.extend_from_slice(&x.to_le_bytes());
            }
        }
        for &b in &bytes {
            ctx.seq_hash ^= b as u64;
            ctx.seq_hash = ctx.seq_hash.wrapping_mul(0x100000001b3);
        }
    }
    if ctx.sols >= ctx.limits.max_sols {
        ctx.aborted = true;
    }
}

// ------------------------------------------------------------------
// independent global validation + fill analysis
// ------------------------------------------------------------------

/// Independent post-validation of a completion: every hole edge covered
/// exactly once, every interior edge exactly twice, zero T-junctions,
/// and the exact area identity.
pub fn global_validate(sub: &GirihSubstrate, placed: &[GirihTile]) -> Vec<String> {
    let mut errs = Vec::new();
    let mut cnt: BTreeMap<EK, u32> = BTreeMap::new();
    for t in placed {
        for i in 0..t.n {
            *cnt.entry(edge_key(t.cs[i], t.cs[(i + 1) % t.n])).or_insert(0) += 1;
        }
    }
    for ek in sub.edge_set.iter() {
        if cnt.get(ek) != Some(&1) {
            errs.push("hole-edge multiplicity".to_string());
        }
    }
    for (ek, &c) in cnt.iter() {
        if !sub.edge_set.contains(ek) && c != 2 {
            errs.push("interior multiplicity".to_string());
        }
    }
    let mut tj = 0usize;
    for i in 0..placed.len() {
        for j in (i + 1)..placed.len() {
            let (t1, t2) = (&placed[i], &placed[j]);
            if bbox_disjoint(t1, t2) {
                continue;
            }
            for p in t1.cs[..t1.n].iter().copied() {
                for k in 0..t2.n {
                    let (u, w) = (t2.cs[k], t2.cs[(k + 1) % t2.n]);
                    if p != u && p != w && on_seg(u, w, p) {
                        tj += 1;
                    }
                }
            }
            for p in t2.cs[..t2.n].iter().copied() {
                for k in 0..t1.n {
                    let (u, w) = (t1.cs[k], t1.cs[(k + 1) % t1.n]);
                    if p != u && p != w && on_seg(u, w, p) {
                        tj += 1;
                    }
                }
            }
        }
    }
    if tj != 0 {
        errs.push(format!("t-junctions: {tj}"));
    }
    let mut area = KZERO;
    for t in placed {
        area = kadd(area, tile_area2(t.kind));
    }
    if sgn(ksub(area, sub.hole_area)) != 0 {
        errs.push("area identity".to_string());
    }
    errs
}

/// Exact corner cos² at corner `i` of a unit-edge girih tile: the
/// cosine is an L10//64 element (unit edges), its square likewise.
/// Girih corners are L10-valued — unlike WIS, cos² is *not* rational,
/// so the WIS rational-normalized [`super::infer::Cos2`] key is
/// replaced by the exact L10//64 tuple.
pub fn corner_cos2(t: &GirihTile, i: usize) -> K4 {
    let n = t.n;
    let v = t.cs[i];
    let e1 = psub(t.cs[(i + n - 1) % n], v);
    let e2 = psub(t.cs[(i + 1) % n], v);
    let dt = dot64(e1, e2);
    kmul(dt, dt)
}

fn canon_corners(t: &GirihTile) -> Vec<GirihP> {
    let mut min_i = 0;
    for i in 1..t.n {
        if t.cs[i] < t.cs[min_i] {
            min_i = i;
        }
    }
    (0..t.n).map(|j| t.cs[(min_i + j) % t.n]).collect()
}

/// Full analysis of a girih fill: census, exact area, vertex/edge
/// counts, per-vertex cos²-class census, canonical fingerprint.
pub struct GirihAnalysis {
    pub census: [u32; 3],
    pub area: K4,
    pub nverts: usize,
    pub nedges: usize,
    pub vcos: BTreeMap<Vec<K4>, usize>,
    pub fp: GirihFillFp,
}

/// Analyze a fill (census order: hex, bow, dec).
pub fn analyze(placed: &[GirihTile]) -> GirihAnalysis {
    let mut census = [0u32; 3];
    let mut area = KZERO;
    let mut verts: BTreeSet<GirihP> = BTreeSet::new();
    let mut edges: BTreeSet<EK> = BTreeSet::new();
    let mut per_v: BTreeMap<GirihP, Vec<K4>> = BTreeMap::new();
    let mut fp = GirihFillFp::new();
    for t in placed {
        census[t.kind as usize - 1] += 1;
        area = kadd(area, tile_area2(t.kind));
        let n = t.n;
        for i in 0..n {
            verts.insert(t.cs[i]);
            edges.insert(edge_key(t.cs[i], t.cs[(i + 1) % n]));
            per_v.entry(t.cs[i]).or_default().push(corner_cos2(t, i));
        }
        fp.insert((t.kind, canon_corners(t)));
    }
    let mut vcos: BTreeMap<Vec<K4>, usize> = BTreeMap::new();
    for (_, mut lst) in per_v {
        lst.sort();
        *vcos.entry(lst).or_insert(0) += 1;
    }
    GirihAnalysis { census, area, nverts: verts.len(), nedges: edges.len(), vcos, fp }
}
