//! 24b — boundary-matched inference engine for hole-bearing substrates
//! (WIS 12-fold ground-truth bar).
//!
//! **Input contract (general path).** The engine consumes only
//!
//! * a hole polygon (CCW, exact lattice points), and
//! * the landed M158 record tables ([`HULL_TBL`], [`NOTCH_TBL`],
//!   [`CENTRE_TBL`]).
//!
//! From the record it derives the star geometry (tips `H[k]`, notches
//! `N[j]`, the 12 unit directions, the two unit tip–notch chords per
//! tip) and reconstructs the *kissing instances*: every star placement
//! `c` for which some hole edge is a hull edge of the star at `c`
//! (`hull readings`). The principled substrate is the 20-instance set
//! (centres not strictly inside the hole); the 24-instance set keeps
//! the phantom annotations. The engine never sees the shipped cluster
//! data ([`SQ_SUP`] lives in the fixture side, `holes.rs`, and in the
//! tests only).
//!
//! **Inference (deterministic exact backtracking jigsaw).** Frontier =
//! first uncovered boundary edge (CCW direction), else the open edge
//! with the smallest canonical key (tile placed on the empty side).
//! Candidates per directed unit edge `a→b` (tile on the left): the
//! unit triangle, the unit square, and the two unit 30°/150° rhombi.
//! A placement is kept iff
//!
//! 1. **C1** boundary edges are covered exactly once,
//! 2. **C2** every *new* interior edge satisfies the level constraint
//!    (see [`Level`]),
//! 3. **C3** the tile is edge-to-edge with the placed tiles (no vertex
//!    in another edge's relative interior — this kills T-junctions and
//!    collinear partial overlaps), pairwise interior-disjoint, inside
//!    the hole closure, and
//! 4. **C4** the running exact 2-area never exceeds the hole 2-area
//!    and equals it at completion (coverage + area identity).
//!
//! **Underdetermination is a first-class output.** The search never
//! forces a choice among valid completions: it returns the documented
//! freedom set (census classes, distinct-fill fingerprints, a sequence
//! hash, node/placement counts) up to deterministic caps, or runs to
//! exhaustion. No RNG anywhere; every iteration is over total orders
//! (`BTreeMap`/`BTreeSet`, fixed candidate order), so re-runs are
//! byte-identical.
//!
//! **Exact arithmetic.** Raw `Z[√3]` 4-tuples `(P,Q,R,S)` with world
//! `((P+Q√3)/2, (R+S√3)/2)`; pair values `(a,b)` mean `a+b√3`;
//! [`sgn3`] is the exact sign of `a+b√3` (zero iff `a=b=0`). Raw
//! cross/dot are 4× their world counterparts. f64 appears only in the
//! bbox *prefilter* (slack 0.02, conservative) and in analysis floats
//! — every accept/reject decision is exact integer/pair arithmetic.

use std::collections::{BTreeMap, BTreeSet};

use super::wis_m158::{CENTRE_TBL, HULL_TBL, NOTCH_TBL};
use crate::families::quasiperiodic::wis_12_fold::{add4, rot4, P4};

// ------------------------------------------------------------------
// exact raw arithmetic (self-contained; pair (a,b) means a + b√3)
// ------------------------------------------------------------------

/// An element of ℤ[√3] (or a rational, when the √3 part vanishes).
pub type Pair = (i64, i64);

#[inline]
pub fn sub4(a: P4, b: P4) -> P4 {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2, a.3 - b.3)
}

#[inline]
fn neg4(a: P4) -> P4 {
    (-a.0, -a.1, -a.2, -a.3)
}

/// Raw cross product, 4× the world cross product.
#[inline]
pub fn cross2(u: P4, v: P4) -> Pair {
    (
        u.0 * v.2 + 3 * u.1 * v.3 - u.2 * v.0 - 3 * u.3 * v.1,
        u.0 * v.3 + u.1 * v.2 - u.2 * v.1 - u.3 * v.0,
    )
}

/// Raw dot product, 4× the world dot product.
#[inline]
pub fn dot4(u: P4, v: P4) -> Pair {
    (
        u.0 * v.0 + 3 * u.1 * v.1 + u.2 * v.2 + 3 * u.3 * v.3,
        u.0 * v.1 + u.1 * v.0 + u.2 * v.3 + u.3 * v.2,
    )
}

/// Exact sign of `x + y√3` (zero iff `x = y = 0`, since √3 irrational).
#[inline]
pub fn sgn3(x: i64, y: i64) -> i8 {
    if x == 0 && y == 0 {
        return 0;
    }
    if x >= 0 && y >= 0 {
        return 1;
    }
    if x <= 0 && y <= 0 {
        return -1;
    }
    let t = (x as i128) * (x as i128) - 3 * ((y as i128) * (y as i128));
    if x > 0 && y < 0 {
        if t > 0 {
            1
        } else {
            -1
        }
    } else if t < 0 {
        1
    } else {
        -1
    }
}

/// Sign of a pair value.
#[inline]
pub fn sp(p: Pair) -> i8 {
    sgn3(p.0, p.1)
}

#[inline]
pub fn padd(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}

#[inline]
pub fn psub(a: Pair, b: Pair) -> Pair {
    (a.0 - b.0, a.1 - b.1)
}

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// Shoelace 2-area of a raw polygon as a ℤ[√3] pair (4× world).
pub fn poly_area2(poly: &[P4]) -> Pair {
    let mut s = (0i64, 0i64);
    let n = poly.len();
    for i in 0..n {
        s = padd(s, cross2(poly[i], poly[(i + 1) % n]));
    }
    s
}

fn seg_side(a: P4, b: P4, p: P4) -> i8 {
    sp(cross2(sub4(b, a), sub4(p, a)))
}

/// `p` on the closed segment `ab` (exact).
pub fn on_seg(a: P4, b: P4, p: P4) -> bool {
    if seg_side(a, b, p) != 0 {
        return false;
    }
    let d = sub4(b, a);
    let e = sub4(p, a);
    let f = sub4(d, e);
    sp(dot4(d, e)) >= 0 && sp(dot4(d, f)) >= 0
}

/// Proper (interior×interior) crossing of segments `ab` and `cd`.
pub fn segs_proper_cross(a: P4, b: P4, c: P4, d: P4) -> bool {
    let d1 = seg_side(a, b, c) as i32;
    let d2 = seg_side(a, b, d) as i32;
    let d3 = seg_side(c, d, a) as i32;
    let d4 = seg_side(c, d, b) as i32;
    d1 * d2 < 0 && d3 * d4 < 0
}

/// Exact world-y scanline point-in-polygon (integer coordinates;
/// boundary excluded). Sign tests go through [`sgn3`], so the result
/// is exact over ℤ[√3] embeddings.
pub fn strictly_inside(p: P4, poly: &[P4]) -> bool {
    let n = poly.len();
    for i in 0..n {
        if on_seg(poly[i], poly[(i + 1) % n], p) {
            return false;
        }
    }
    let mut cnt = 0usize;
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        let say = sgn3(a.2 - p.2, a.3 - p.3);
        let sby = sgn3(b.2 - p.2, b.3 - p.3);
        if (say > 0) == (sby > 0) {
            continue;
        }
        let d = sub4(b, a);
        let c = cross2(d, sub4(a, p));
        let sdy = sgn3(d.2, d.3);
        let cs = sp(c);
        if cs == 0 {
            return false;
        }
        if (cs as i32) * (sdy as i32) < 0 {
            cnt += 1;
        }
    }
    cnt % 2 == 1
}

/// Canonical orientation of a direction: the world vector is made
/// lexicographically positive (x-sign first, then y-sign). Parallel
/// unit directions canonicalize to the same tuple, so
/// `(dir, cross2(dir, p))` is an exact geometric line key.
fn canon_dir4(d: P4) -> P4 {
    let sx = sgn3(d.0, d.1);
    if sx > 0 {
        return d;
    }
    if sx < 0 {
        return neg4(d);
    }
    if sgn3(d.2, d.3) >= 0 {
        d
    } else {
        neg4(d)
    }
}

/// A geometric line through `p` with direction `d` (exact key).
pub type LK = (P4, Pair);

#[inline]
pub fn line_key(p: P4, d: P4) -> LK {
    let dd = canon_dir4(d);
    (dd, cross2(dd, p))
}

/// Legal WIS lattice point: `P ≡ S (mod 2)`, `Q ≡ R (mod 2)`.
pub fn is_legal_lattice(p: P4) -> bool {
    p.0.rem_euclid(2) == p.3.rem_euclid(2) && p.1.rem_euclid(2) == p.2.rem_euclid(2)
}

// ------------------------------------------------------------------
// prototiles
// ------------------------------------------------------------------

pub const TRI: u8 = 0;
pub const SQ: u8 = 1;
pub const RHO: u8 = 2;

const SQRT3: f64 = 1.7320508075688772;

/// Exact raw shoelace 2-area of each prototile.
pub fn tile_area(kind: u8) -> Pair {
    match kind {
        TRI => (0, 2),
        SQ => (8, 0),
        _ => (4, 0),
    }
}

/// A placed prototile (3 or 4 corners, CCW, unit edges).
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub kind: u8,
    pub n: usize,
    pub cs: [P4; 4],
    /// world bbox `[minx, maxx, miny, maxy]` — prefilter only.
    pub bbox: [f64; 4],
}

impl Tile {
    pub fn new(kind: u8, cs: &[P4]) -> Tile {
        let n = cs.len();
        assert!(n == 3 || n == 4, "prototile needs 3 or 4 corners");
        let mut arr = [(0i64, 0i64, 0i64, 0i64); 4];
        arr[..n].copy_from_slice(cs);
        let mut bbox = [f64::INFINITY; 4];
        for &p in &arr[..n] {
            let x = 0.5 * (p.0 as f64 + p.1 as f64 * SQRT3);
            let y = 0.5 * (p.2 as f64 + p.3 as f64 * SQRT3);
            bbox[0] = bbox[0].min(x);
            bbox[1] = bbox[1].max(x);
            bbox[2] = bbox[2].min(y);
            bbox[3] = bbox[3].max(y);
        }
        debug_assert!(arr[..n].iter().all(|&p| is_legal_lattice(p)));
        Tile { kind, n, cs: arr, bbox }
    }
}

#[inline]
fn bbox_disjoint(a: &Tile, b: &Tile) -> bool {
    const SLACK: f64 = 0.02;
    b.bbox[1] < a.bbox[0] - SLACK
        || b.bbox[0] > a.bbox[1] + SLACK
        || b.bbox[3] < a.bbox[2] - SLACK
        || b.bbox[2] > a.bbox[3] + SLACK
}

/// Undirected edge key: the sorted corner pair (a total order — no
/// hash, no collisions, deterministic iteration).
pub type EK = (P4, P4);

#[inline]
pub fn edge_key(a: P4, b: P4) -> EK {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

/// The four candidate placements of a prototile with the directed unit
/// edge `a→b` on its boundary, tile on the left (CCW):
/// triangle, square, 30°-rhombus, 150°-rhombus. Fixed order.
pub fn candidates(a: P4, b: P4) -> [Tile; 4] {
    let e = sub4(b, a);
    let r30 = rot4(e, 1);
    let r60 = rot4(e, 2);
    let r90 = (-e.2, -e.3, e.0, e.1);
    let r150 = (-r60.2, -r60.3, r60.0, r60.1);
    [
        Tile::new(TRI, &[a, b, add4(a, r60)]),
        Tile::new(SQ, &[a, b, add4(b, r90), add4(a, r90)]),
        Tile::new(RHO, &[a, b, add4(b, r30), add4(a, r30)]),
        Tile::new(RHO, &[a, b, add4(b, r150), add4(a, r150)]),
    ]
}

// ------------------------------------------------------------------
// substrate: hole polygon + record → instances and carried data
// ------------------------------------------------------------------

/// Which reconstructed instance set the engine works over.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InstanceSet {
    /// Instances whose centre is not strictly inside the hole (the
    /// principled substrate; the phantom F sites are dropped).
    K20,
    /// All hull-reading instances, including the phantom annotations.
    K24,
}

/// The hole-bearing substrate, entirely engine-derived from the hole
/// polygon and the landed record.
pub struct Substrate {
    pub poly: Vec<P4>,
    pub hole_vset: BTreeSet<P4>,
    pub edges: Vec<(P4, P4)>,
    pub edge_set: BTreeSet<EK>,
    pub edge_dir: BTreeMap<EK, (P4, P4)>,
    nxt: BTreeMap<P4, P4>,
    prv: BTreeMap<P4, P4>,
    poly2: Vec<P4>,
    edges2: Vec<(P4, P4)>,
    pub hole_area: Pair,
    /// Record-derived star tips relative to the star centre.
    pub h: Vec<P4>,
    /// Record-derived star notches relative to the star centre.
    pub nch: Vec<P4>,
    /// Per tip: the two (notch index, unit chord direction) pairs.
    pub tn: Vec<Vec<(usize, P4)>>,
    pub instances20: Vec<P4>,
    pub instances24: Vec<P4>,
    ray_lines20: BTreeSet<LK>,
    ray_lines24: BTreeSet<LK>,
    segments20: BTreeSet<EK>,
    segments24: BTreeSet<EK>,
    vset20: BTreeSet<P4>,
    vset24: BTreeSet<P4>,
}

impl Substrate {
    pub fn new(poly: Vec<P4>) -> Substrate {
        let nv = poly.len();
        assert!(nv >= 3, "degenerate hole polygon");
        let area = poly_area2(&poly);
        assert!(sp(area) > 0, "hole polygon must be CCW");
        let hole_vset: BTreeSet<P4> = poly.iter().copied().collect();
        assert_eq!(hole_vset.len(), nv, "duplicate hole vertex");
        let edges: Vec<(P4, P4)> =
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
        let poly2: Vec<P4> = poly.iter().map(|&p| padd4(p, p)).collect();
        let edges2: Vec<(P4, P4)> =
            edges.iter().map(|&(a, b)| (padd4(a, a), padd4(b, b))).collect();

        // Record-derived star geometry (relative to the star centre).
        let h: Vec<P4> = HULL_TBL.iter().map(|&t| sub4(t, CENTRE_TBL)).collect();
        let nch: Vec<P4> = NOTCH_TBL.iter().map(|&t| sub4(t, CENTRE_TBL)).collect();
        let dirs: BTreeSet<P4> = nch.iter().copied().collect();
        assert_eq!(dirs.len(), 12, "record notches must give 12 unit dirs");
        let mut tn: Vec<Vec<(usize, P4)>> = vec![Vec::new(); 12];
        for k in 0..12 {
            for j in 0..12 {
                let d = sub4(h[k], nch[j]);
                if dirs.contains(&d) {
                    tn[k].push((j, d));
                }
            }
            assert_eq!(tn[k].len(), 2, "each tip has exactly 2 unit chords");
        }

        // Hull readings: per hole edge, every star placement whose hull
        // edge (in either orientation) equals the edge.
        let mut tags: Vec<Vec<P4>> = Vec::with_capacity(nv);
        for &(a, b) in &edges {
            let d = sub4(b, a);
            let mut s: BTreeSet<P4> = BTreeSet::new();
            for t in 0..12usize {
                let (u, v) = (h[t], h[(t + 1) % 12]);
                if sub4(v, u) == d {
                    s.insert(sub4(a, u));
                }
                if sub4(v, u) == neg4(d) {
                    s.insert(sub4(b, u));
                }
            }
            tags.push(s.into_iter().collect());
        }

        let mut all: BTreeSet<P4> = BTreeSet::new();
        for lst in &tags {
            for &c in lst {
                all.insert(c);
            }
        }
        let instances24: Vec<P4> = all.into_iter().collect();
        let instances20: Vec<P4> = instances24
            .iter()
            .copied()
            .filter(|&c| !strictly_inside(c, &poly))
            .collect();

        let ray_lines24 = ray_lines(&h, &tn, &instances24);
        let ray_lines20 = ray_lines(&h, &tn, &instances20);
        let segments24 = segments(&h, &tn, &nch, &instances24);
        let segments20 = segments(&h, &tn, &nch, &instances20);
        let vset24 = vset(&h, &nch, &instances24, &hole_vset, &poly);
        let vset20 = vset(&h, &nch, &instances20, &hole_vset, &poly);

        Substrate {
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
            h,
            nch,
            tn,
            instances20,
            instances24,
            ray_lines20,
            ray_lines24,
            segments20,
            segments24,
            vset20,
            vset24,
        }
    }

    pub fn instances(&self, s: InstanceSet) -> &[P4] {
        match s {
            InstanceSet::K20 => &self.instances20,
            InstanceSet::K24 => &self.instances24,
        }
    }
    pub fn ray_lines(&self, s: InstanceSet) -> &BTreeSet<LK> {
        match s {
            InstanceSet::K20 => &self.ray_lines20,
            InstanceSet::K24 => &self.ray_lines24,
        }
    }
    pub fn segments(&self, s: InstanceSet) -> &BTreeSet<EK> {
        match s {
            InstanceSet::K20 => &self.segments20,
            InstanceSet::K24 => &self.segments24,
        }
    }
    pub fn vset(&self, s: InstanceSet) -> &BTreeSet<P4> {
        match s {
            InstanceSet::K20 => &self.vset20,
            InstanceSet::K24 => &self.vset24,
        }
    }
}

#[inline]
fn padd4(a: P4, b: P4) -> P4 {
    add4(a, b)
}

fn ray_lines(h: &[P4], tn: &[Vec<(usize, P4)>], inst: &[P4]) -> BTreeSet<LK> {
    let mut out = BTreeSet::new();
    for &c in inst {
        for k in 0..12 {
            let tip = add4(c, h[k]);
            for &(_, d) in &tn[k] {
                out.insert(line_key(tip, d));
            }
        }
    }
    out
}

fn segments(
    h: &[P4],
    tn: &[Vec<(usize, P4)>],
    nch: &[P4],
    inst: &[P4],
) -> BTreeSet<EK> {
    let mut out = BTreeSet::new();
    for &c in inst {
        for k in 0..12 {
            let t_k = add4(c, h[k]);
            let t_n = add4(c, h[(k + 1) % 12]);
            out.insert(edge_key(t_k, t_n)); // hull edge
            for &(j, _) in &tn[k] {
                out.insert(edge_key(t_k, add4(c, nch[j]))); // unit chord
            }
        }
        for j in 0..12 {
            out.insert(edge_key(c, add4(c, nch[j]))); // radius
        }
    }
    out
}

fn vset(
    h: &[P4],
    nch: &[P4],
    inst: &[P4],
    hole_vset: &BTreeSet<P4>,
    poly: &[P4],
) -> BTreeSet<P4> {
    let mut out = BTreeSet::new();
    for &c in inst {
        let cand = [c]
            .into_iter()
            .chain((0..12).map(|k| add4(c, h[k])))
            .chain((0..12).map(|j| add4(c, nch[j])));
        for p in cand {
            if hole_vset.contains(&p) || strictly_inside(p, poly) {
                out.insert(p);
            }
        }
    }
    out
}

// ------------------------------------------------------------------
// levels + search
// ------------------------------------------------------------------

/// Constraint level on new interior edges / tile vertices.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Level {
    /// C1+C3+C4 only (pure structural jigsaw).
    Plain,
    /// Every new interior edge lies on a carried exit-ray *line*.
    RayLine,
    /// [`Level::RayLine`] plus every tile vertex in the anatomy point
    /// set of the declared instances intersected with the hole closure.
    RayLineVset,
    /// Every new interior edge *is* a substrate segment (hull edge,
    /// unit chord, or radius of a declared instance).
    Segment,
}

/// Deterministic search caps (no wall-clock — re-runs are identical).
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub max_nodes: u64,
    pub max_sols: usize,
}

/// The engine's documented output for one search run.
#[derive(Debug, Clone)]
pub struct Report {
    pub nodes: u64,
    pub placements: u64,
    /// false ⇒ a cap was hit (documented partial freedom set).
    pub exhausted: bool,
    pub sols: usize,
    pub distinct: usize,
    /// completions failing independent global validation (must stay 0).
    pub invalid: usize,
    /// max |component| over all completion vertices (finiteness).
    pub max_coord: i64,
    /// (tri, sq, rho) census → completions in that class.
    pub by_census: BTreeMap<[u32; 3], usize>,
    /// FNV-1a over the solution sequence (kinds + corners, in order).
    pub seq_hash: u64,
    /// exact canonical fingerprints of every distinct completion.
    pub fps: BTreeSet<FillFp>,
}

/// Exact canonical fingerprint of a fill: the set of its tiles, each
/// (kind, corners rotated to start at the minimal corner).
pub type FillFp = BTreeSet<(u8, Vec<[i32; 4]>)>;

/// Level constraint data resolved for one run.
pub struct Constraints<'a> {
    pub lines: Option<&'a BTreeSet<LK>>,
    pub vset: Option<&'a BTreeSet<P4>>,
    pub segs: Option<&'a BTreeSet<EK>>,
}

pub fn constraints(sub: &Substrate, set: InstanceSet, level: Level) -> Constraints<'_> {
    match level {
        Level::Plain => Constraints { lines: None, vset: None, segs: None },
        Level::RayLine => {
            Constraints { lines: Some(sub.ray_lines(set)), vset: None, segs: None }
        }
        Level::RayLineVset => Constraints {
            lines: Some(sub.ray_lines(set)),
            vset: Some(sub.vset(set)),
            segs: None,
        },
        Level::Segment => {
            Constraints { lines: None, vset: None, segs: Some(sub.segments(set)) }
        }
    }
}

struct Ctx<'a> {
    sub: &'a Substrate,
    cons: Constraints<'a>,
    limits: Limits,
    nodes: u64,
    placements: u64,
    aborted: bool,
    sols: usize,
    invalid: usize,
    max_coord: i64,
    by_census: BTreeMap<[u32; 3], usize>,
    seq_hash: u64,
    fps: BTreeSet<FillFp>,
}

/// Run the deterministic backtracking search.
pub fn search(sub: &Substrate, set: InstanceSet, level: Level, limits: Limits) -> Report {
    let cons = constraints(sub, set, level);
    let mut ctx = Ctx {
        sub,
        cons,
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
    };
    let mut placed: Vec<Tile> = Vec::new();
    rec(&mut ctx, BTreeSet::new(), BTreeMap::new(), &mut placed, (0, 0));
    Report {
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

fn rec(
    ctx: &mut Ctx,
    cov: BTreeSet<EK>,
    opn: BTreeMap<EK, (P4, P4)>,
    placed: &mut Vec<Tile>,
    ar: Pair,
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
    let mut frontier: Option<(P4, P4)> = None;
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
                if sp(psub(ar, ctx.sub.hole_area)) == 0 {
                    record(ctx, placed);
                }
                return;
            }
        },
    };
    for cand in candidates(a, b) {
        let na = padd(ar, tile_area(cand.kind));
        if sp(psub(na, ctx.sub.hole_area)) > 0 {
            continue;
        }
        ctx.placements += 1;
        if !valid_placement(ctx.sub, &ctx.cons, &cand, placed) {
            continue;
        }
        let n = cand.n;
        let mut ok = true;
        let mut nc: Vec<EK> = Vec::new();
        let mut matched: Vec<EK> = Vec::new();
        let mut nop: Vec<(EK, (P4, P4))> = Vec::new();
        for i in 0..n {
            let (u, v) = (cand.cs[i], cand.cs[(i + 1) % n]);
            let ek = edge_key(u, v);
            if ctx.sub.edge_set.contains(&ek) {
                if cov.contains(&ek) {
                    ok = false;
                    break;
                }
                nc.push(ek);
            } else if opn.contains_key(&ek) {
                matched.push(ek);
            } else {
                if let Some(segs) = ctx.cons.segs {
                    if !segs.contains(&ek) {
                        ok = false;
                        break;
                    }
                }
                if let Some(lines) = ctx.cons.lines {
                    if !lines.contains(&line_key(u, sub4(v, u))) {
                        ok = false;
                        break;
                    }
                }
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
        rec(ctx, cov2, opn2, placed, na);
        placed.pop();
        if ctx.aborted {
            return;
        }
    }
}

fn record(ctx: &mut Ctx, placed: &[Tile]) {
    ctx.sols += 1;
    if !global_validate(ctx.sub, placed).is_empty() {
        ctx.invalid += 1;
    }
    let an = analyze(placed);
    *ctx.by_census.entry(an.census).or_insert(0) += 1;
    ctx.fps.insert(an.fp);
    for t in placed {
        let mut bytes = Vec::with_capacity(1 + 4 * 32);
        bytes.push(t.kind);
        for &p in &t.cs[..t.n] {
            for x in [p.0, p.1, p.2, p.3] {
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
// placement validity (C3, exact)
// ------------------------------------------------------------------

fn mid_inside_or_on(sub: &Substrate, u: P4, v: P4) -> bool {
    let m = padd4(u, v); // doubled midpoint stays integral
    for &(a2, b2) in sub.edges2.iter() {
        if on_seg(a2, b2, m) {
            return true;
        }
    }
    strictly_inside(m, &sub.poly2)
}

/// A tile edge may pass through a hole vertex only in the wedge the
/// hole opens there (both tile-edge endpoints left-of-or-on the two
/// adjacent CCW hole edges).
fn edge_thru_hole_vertex_ok(sub: &Substrate, u: P4, v: P4) -> bool {
    for &w in sub.hole_vset.iter() {
        if w == u || w == v || !on_seg(u, v, w) {
            continue;
        }
        let p = sub.prv[&w];
        let q = sub.nxt[&w];
        for x in [u, v] {
            if x == w {
                continue;
            }
            if sp(cross2(sub4(w, p), sub4(x, p))) < 0 {
                return false;
            }
            if sp(cross2(sub4(q, w), sub4(x, w))) < 0 {
                return false;
            }
        }
    }
    true
}

/// C3 single-placement predicate: hole-closure (and level vset)
/// vertex membership, no proper crossing of hole edges, midpoint
/// inside-or-on, hole-vertex wedge rule, rule (3) edge-to-edge vs the
/// placed tiles (both directions), pairwise interior disjointness.
/// Exact except the conservative f64 bbox prefilter.
pub fn valid_placement(
    sub: &Substrate,
    cons: &Constraints,
    t: &Tile,
    placed: &[Tile],
) -> bool {
    let n = t.n;
    for i in 0..n {
        let v = t.cs[i];
        if let Some(vs) = cons.vset {
            if !vs.contains(&v) {
                return false;
            }
        }
        if !sub.hole_vset.contains(&v) && !strictly_inside(v, &sub.poly) {
            return false;
        }
    }
    for i in 0..n {
        let (u, v) = (t.cs[i], t.cs[(i + 1) % n]);
        for &(a, b) in sub.edges.iter() {
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
    for t2 in placed.iter() {
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
// independent global validation + fill analysis
// ------------------------------------------------------------------

/// Independent post-validation of a completion: every hole edge covered
/// exactly once, every interior edge exactly twice, zero T-junctions,
/// and the exact area identity.
pub fn global_validate(sub: &Substrate, placed: &[Tile]) -> Vec<String> {
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
    if tj > 0 {
        errs.push(format!("{tj} T-junctions"));
    }
    let mut ar = (0i64, 0i64);
    for t in placed {
        ar = padd(ar, tile_area(t.kind));
    }
    if ar != sub.hole_area {
        errs.push(format!("area {ar:?} != {:?}", sub.hole_area));
    }
    errs
}

/// cos² of a prototile corner's interior angle as a reduced rational
/// `(num, den)` (zero pinned to `(0, 0)`); the WIS unit-edge corners
/// give exactly `(0,0)`, `(1,4)`, `(3,4)`.
pub type Cos2 = (i64, i64);

pub fn corner_cos2(cs: &[P4; 4], n: usize, i: usize) -> Cos2 {
    let v = cs[i];
    let e1 = sub4(cs[(i + n - 1) % n], v);
    let e2 = sub4(cs[(i + 1) % n], v);
    let dt = dot4(e1, e2);
    let sq = (dt.0 * dt.0 + 3 * dt.1 * dt.1, 2 * dt.0 * dt.1);
    assert!(
        sq.1 == 0,
        "non-rational cos² — unit-edge invariant broken at corner {i}"
    );
    if sq.0 == 0 {
        return (0, 0);
    }
    let g = gcd(sq.0, 16);
    (sq.0 / g, 16 / g)
}

fn canon_corners(t: &Tile) -> Vec<[i32; 4]> {
    let mut min_i = 0;
    for i in 1..t.n {
        if t.cs[i] < t.cs[min_i] {
            min_i = i;
        }
    }
    (0..t.n)
        .map(|j| {
            let p = t.cs[(min_i + j) % t.n];
            debug_assert!(p.0.abs() < 1 << 20);
            [p.0 as i32, p.1 as i32, p.2 as i32, p.3 as i32]
        })
        .collect()
}

/// Full analysis of a fill: census, exact area, vertex/edge counts,
/// per-vertex cos²-class census, canonical fingerprint.
pub struct Analysis {
    pub census: [u32; 3],
    pub area: Pair,
    pub nverts: usize,
    pub nedges: usize,
    pub vcos: BTreeMap<Vec<Cos2>, usize>,
    pub fp: FillFp,
}

pub fn analyze(placed: &[Tile]) -> Analysis {
    let mut census = [0u32; 3];
    let mut area = (0i64, 0i64);
    let mut verts: BTreeSet<P4> = BTreeSet::new();
    let mut edges: BTreeSet<EK> = BTreeSet::new();
    let mut per_v: BTreeMap<P4, Vec<Cos2>> = BTreeMap::new();
    let mut fp = FillFp::new();
    for t in placed {
        census[t.kind as usize] += 1;
        area = padd(area, tile_area(t.kind));
        let n = t.n;
        for i in 0..n {
            verts.insert(t.cs[i]);
            edges.insert(edge_key(t.cs[i], t.cs[(i + 1) % n]));
            per_v.entry(t.cs[i]).or_default().push(corner_cos2(&t.cs, n, i));
        }
        fp.insert((t.kind, canon_corners(t)));
    }
    let mut vcos: BTreeMap<Vec<Cos2>, usize> = BTreeMap::new();
    for (_, mut lst) in per_v {
        lst.sort();
        *vcos.entry(lst).or_insert(0) += 1;
    }
    Analysis {
        census,
        area,
        nverts: verts.len(),
        nedges: edges.len(),
        vcos,
        fp,
    }
}