//! (n,c)-monster medallion catalog: exact pair placements, overlap
//! tables and ray-exit records (brief 24c).
//!
//! A **monster** (Kaplan diss. §3.10, definition cited at
//! `kaplan_full.txt` line 3036) is "the union of a pair of regular
//! n-gons arranged so that exactly c full edges of each polygon lie
//! inside the other" — the definition ignores c = 0/1 (shared
//! vertex/edge) and admits a one-parameter family of center
//! displacements for every legal (n, c). This module catalogues the
//! family by a representative pair — one convex n-gon Medallion record
//! placed twice, at `A = U` and `B = R·A + P` with `U` the normalized
//! unit-edge regular n-gon — over n ∈ {8, 9, 10, 12} and every legal
//! `c` (8 → {2}; 9 → {2, 3}; 10 → {2, 3}; 12 → {2, 3, 4}, i.e.
//! 2 ≤ c ≤ ⌈n/2⌉ − 2, the band the displacement censuses realize). For
//! even n the rotation `R` is the identity (a **parallel** pair); for
//! odd n = 9 it is `rot(π/9)` — the parallel family cannot pin an odd
//! polygon at all ("Why the parallel pair"). The
//! union is non-convex, so a monster is *not* one hull record (the 24a
//! schema requires a strictly convex hull, doc §1.2): each entry is the
//! record PAIR plus the exact placement record below, exactly as the
//! 24c brief pins the architecture.
//!
//! # Why the parallel pair
//!
//! * A half-turn about the midpoint of the centers (`H(X) = 2·O + P − X`
//!   with `O` the centre of `U`) swaps `A` and `B` for *any*
//!   displacement — `2·O − V_k = V_{k+n/2}` in the record frame, a
//!   battery-asserted lemma — so `eAB = eBA` (the "c of each" half of
//!   the definition) is automatic, and `eBA = {i + n/2 mod n}` exactly.
//! * Requiring the pair to carry the menagerie's mirror symmetry as
//!   well forces `P` along a symmetry axis of `U`: the vertex axis
//!   `V_{n/2} − V_0` or the mid-edge axis `M_{n/2} − M_0`. Both axis
//!   families occur below.
//! * For **odd** `n` the parallel family is uncataloguable: the pin
//!   needs `P = M_i − M_j`, and an exact parity count (scratch
//!   `.scratch/monster-n9/VERDICT.md`) shows no parallel midpoint pin
//!   sits on a symmetry axis of an odd-gon, while every off-axis
//!   parallel pin carries TWO distinct contact angles at its two
//!   crossings — no single `{n/d}` figure, hence no record. The odd-n
//!   analog is the pair **rotated by δ = π/n**: `B = rot(π/n)·A + P`.
//!   There the half-turn `κ(X) = q − X` about a certified center
//!   swaps the two polygons for *any* `P` — for odd `n`, `rot(π/n)`
//!   composed with the half-turn is a vertex-frame shift — with the
//!   exact index maps `κ(V_k^A) = V_{k+4}^B`, `κ(M_k^A) = M_{k+4}^B`
//!   and `κ(V_m^B) = V_{m+5}^A` at n = 9 (battery-asserted). The
//!   exact n = 9 enumeration (`.scratch/monster-n9/census5.py`)
//!   realizes exactly two clean uniform-angle classes — (9,2) and
//!   (9,3), 18 pins each, one dihedral orbit — and no `c ≥ 4`
//!   anywhere: the same band `2 ≤ c ≤ ⌈n/2⌉ − 2`. Rotated placements
//!   with other contact angles (e.g. 45°/75° for (12,2)) need `cos Δ
//!   ∉ Q(√3)` — a shipped-field gap, documented, not forced (brief
//!   STOP list). n = 11 stays blocked on its degree-5 field
//!   (unshipped).
//!
//! # Pinning: crossing-midpoint coincidence
//!
//! The c-count window (interval of `ρ = |P|/|D|`, `D` the axis
//! diagonal of the normalized hull) is a continuum, so the catalog
//! pins the unique displacement at which the two transversal boundary
//! crossings sit exactly at **edge midpoints of both polygons** —
//! equivalently, for the crossing edge pair `(i, j)`: `P = M_i − M_j`.
//! The pin is unique in each window: the crossing parameter `t(ρ)` is
//! strictly monotone and passes through 1/2 only at the pin (battery:
//! `2·t = den` exactly at the pin; the pin ± `D/64` samples keep
//! `t ≠ 1/2`). For five of the six parallel entries the pin is also the
//! midpoint of the certified c-count window; for (12,3) it is the
//! symmetric center of the relabeling-event pair `((√3−1)/2,
//! (3−√3)/2)` bracketing the catalogued combinatorics. The rotated
//! n = 9 entries pin identically at both polygons' edge midpoints,
//! with the pin rule `P = M_i − R·M_j` for `R = rot(π/9)`: (9,2) pins
//! `(A2,B0)` and `(A5,B6)`, (9,3) pins `(A1,B0)` and `(A5,B5)`, both
//! at `t = s = 1/2` exactly. Their parameter is `σ` along the ray
//! `σ·P` (pin at `σ = 1`), and `t(σ)` is strictly monotone in the
//! record's own direction — increasing for (9,2), decreasing for
//! (9,3) — through 1/2 only at the pin (exact samples `t(63/64) =
//! 63/128`, `t(65/64) = 65/128` resp. `t(63/64) = 65/128`, `t(65/64)
//! = 63/128`). The (9,2) pin is again the exact window midpoint
//! (`σ_lo + σ_hi = 2`); the (9,3) pin is not.
//!
//! # Catalog
//!
//! `a = 2cos(π/10)` (basis `(1, a, a², a³)`, [`L10`]); `Zq` tuples are
//! `(a + b√d)/den`. `P = ρ·D` holds exactly (`ρ` a field element, the
//! battery asserting `P = M_i − M_j` against it). The two nonagon rows
//! use `b = 2cos(2π/9)` and `w = 2sin20°` (`w² = 2 − b`), elements
//! `(u + v·w)/den` over the basis `(1, b, b²)` ([`n9::Nq`], [`K9`]);
//! there `P` is the pin itself (`σ = 1`, no axis diagonal).
//!
//! | pair   | axis     | ρ              | P (exact)                       | eAB       | eBA        | crossings (A_i, B_j) | d | cos²(cross) |
//! |--------|----------|----------------|---------------------------------|-----------|------------|----------------------|---|-------------|
//! | (8,2)  | vertex   | √2/4           | (√2/4, (2+√2)/4)                | [3,4]     | [0,7]      | (2,1), (5,6)         | 2 | 1/2         |
//! | (10,2) | vertex   | √5/4           | ((−5+2a²)/4, a³/4)              | [4,5]     | [0,9]      | (3,1), (6,8)         | 4 | (3−√5)/8    |
//! | (10,3) | mid-edge | (√5−1)/4       | (0, a/2)                        | [4,5,6]   | [0,1,9]    | (3,2), (7,8)         | 2 | (3+√5)/8    |
//! | (12,2) | vertex   | (1+√3)/4       | ((1+√3)/4, (5+3√3)/4)           | [5,6]     | [0,11]     | (4,1), (7,10)        | 6 | 0           |
//! | (12,3) | mid-edge | 1/2            | (0, (2+√3)/2)                   | [5,6,7]   | [0,1,11]   | (4,2), (8,10)        | 4 | 1/4         |
//! | (12,4) | vertex   | 1/4            | (1/4, (2+√3)/4)                 | [4,5,6,7] | [0,1,10,11]| (3,2), (8,9)         | 2 | 3/4         |
//! | (9,2)  | rot π/9  | 1 (σ)          | ((4+b)/4, w(−6+3b+3b²)/4)      | [3,4]     | [7,8]      | (2,0), (5,6)         | 3 | 1/4         |
//! | (9,3)  | rot π/9  | 1 (σ)          | ((6−b²)/4, w(−3+b+b²)/4)       | [2,3,4]   | [6,7,8]    | (1,0), (5,5)         | 1 | (2+b)/4      |
//!
//! `d` is *forced* per (n, c) inside each family: the 2-D
//! displacement census of the dodecagon (`.scratch/monster-24c/
//! explore3.py`) finds crossing class {(6,)} for every c = 2
//! placement, {(4,)} for c = 3, {(2,)} for c = 4, and the n = 8/10
//! census (`sweep2.py`) is unique per window; `d = 2·(i − j) mod n`
//! (folded to `min(d, n − d)`, the canonical figure step) for the
//! crossing pair (battery-asserted, since the polygons are parallel:
//! edge k has direction angle `k·2π/n`, and `cos²` has period `π`).
//! For the rotated n = 9 family the exact census
//! (`.scratch/monster-n9/census5.py`) finds one uniform-angle class
//! per (n, c), and the rotation shifts the second polygon's edge
//! directions by half a step, so the rule becomes `d ≡ 2·(i − j) − 1
//! (mod n)` (same folding; battery-asserted: `(2,0) → 3`, `(5,6) → 3`,
//! `(1,0) → 1`, `(5,5) → 1`). The parallel half-turn maps `(A_i, B_j)
//! → (A_{j+n/2}, B_{i+n/2})`
//! — it swaps the roles — and the mirror about the axis swaps the two
//! crossings (battery); the rotated half-turn swaps the roles too,
//! `(A_i, B_j) → (A_{j+5}, B_{i+4})` at n = 9. Exposed vertices:
//! `n − c − 1` per polygon; the union outline is one exact cycle of
//! `2(n − c − 1) + 2` nodes, the overlap (lens) outline one exact
//! cycle of `2c + 4` — the same counts for the rotated records, with
//! a different split: the union runs after `X0` along the `B`-arc
//! `V_(eb0+1) .. V_(eb1)` and after `X1` along the `A`-arc `V_(ea1+1)
//! .. V_(ea0)` (the lens takes the complementary arcs), and the
//! absorbed runs' END vertices are exposed TRANSITION vertices whose
//! edges' inner halves close the lens beyond a crossing.
//!
//! Windows (certified by the exact vertex-on-edge-line event census,
//! `.scratch/monster-24c/spike2_events.py`, plus battery probes):
//! (8,2) `(0, √2/2)`; (10,2) `(sin 18°, cos 36°)` — below the lower
//! end the boundaries slide into collinear edge overlap with no
//! transversal crossings (probe); (10,3) `(0, φ−1)`; (12,2) `(1/2,
//! √3/2)` — below it the count jumps to c = 4 (probe); (12,3)
//! `(0, √3−1)` — near its lower end the pair still counts (3,3) but
//! has no transversal crossing (probe); (12,4) `(0, 1/2)` — at the
//! upper end edges go collinear (probe). Interior relabeling events
//! exist (e.g. (8,2) at ρ = (2−√2)/2); the pin is always in the
//! generic, crossing, window. The rotated pairs are windows in `σ`
//! along the pin ray `σ·P` (pin `σ = 1`), certified by the same exact
//! vertex-on-edge-line census (`.scratch/monster-n9/window.py`):
//! (9,2) `((26−6b−2b²)/19, (12+6b+2b²)/19)`; (9,3) `((4+2b−2b²)/3,
//! (12+6b+2b²)/19)` — below each lower end FOUR extra transversals
//! appear at the contacts (six crossings; exact probe), above each
//! upper end the absorbed run drops one edge per side (exact probe).
//!
//! # The (12,2) entry and the {12/6} figure
//!
//! The forced angle at c = 2 is 90°, i.e. `d = 6`: the {12/6} figure
//! joins opposite edge midpoints — six diameters — and the two exit
//! rays at each contact **coincide** (`i + 6 ≡ i − 6`), pointing
//! radially outward (`cos² = 0`). This is recorded, not hidden; the
//! M163 extension operator's own guard (`π/n < θ < π/2`,
//! [`extended_segments`]) excludes `θ = 90°` from *extension*, but the
//! standalone star emits the perpendicular arms, and the battery
//! cross-checks (12,2) like the others.
//!
//! # The (9,3) entry and the {9/1} figure
//!
//! The forced angle at n = 9, c = 3 is 20° = π/n, i.e. `d = 1`: the
//! {9/1} figure joins ADJACENT edge midpoints — the midpoint nonagon
//! itself — the catalog's first `d = 1` entry (all landed records have
//! `d ≥ 2`), recorded like the {12/6} degeneracy above. `θ = π/n`
//! sits exactly on the M163 EXTENSION guard boundary `π/n < θ`
//! ([`extended_segments`]), like (12,2) at `π/2`; the standalone
//! [`Motif::Star`] emission path is unguarded, and the battery
//! cross-checks (9,3) like the others. Its `cos² 20° = (2+b)/4` is
//! cubic — outside the trait's integer quadruple, so
//! [`Medallion::contact_cos2`] returns `None` (dec10/L10 precedent)
//! while the exact battery pins `(2 + b)/4` (`n9::cos293`).
//!
//! # M163 consistency
//!
//! [`Motif::Star`] on a regular n-gon emits, per edge midpoint, the
//! two arms `rot(edge tangent, ±θ)` (both chiralities) — the same
//! lines as the record's exit rays with `θ = d·180/n`. The battery
//! matches segment lines to record exit lines for all eight records; a
//! mismatch would be a STOP, the check passes (the two nonagon checks
//! exercise `Motif::Star` at `θ = 60°` and the boundary `θ = π/n`).
//!
//! # Consumed by 24d / documented gaps
//!
//! 24d (the fused ornament) consumes first the placement records
//! ([`oct8::pair82`], [`dec10::pair102`], [`dec10::pair103`],
//! [`dod12::pair122`], [`dod12::pair123`], [`dod12::pair124`],
//! [`n9::pair92`], [`n9::pair93`]) —
//! displacement, inside-edge tables, crossings, union/lens outlines —
//! and the per-record exits. The Bonner/Castéra/menagerie
//! *overlap-motif* content (Figure 3.40's cell drawings are not
//! recoverable from the text source) is a 24b/24e target: this catalog
//! pins geometry and never guesses motifs from pictures. The
//! {8,12}/{9,12} "strange stars" (§3.11.4, rhomb-vertex inflation)
//! are a separate placement technique, out of the 24c base scope and
//! deferred.
//!
//! This module is **data + tests only** — no [`Tiling`], no registry
//! entry (built-ins stay 163, memberships 184). Every claim is exact:
//! signs via [`FieldEl::sgn`], angles as cross-multiplied `cos²`
//! identities, outlines as exact closed cycles. f64 appears only in
//! the consumer views and the landed M163 cross-check. The n = 9
//! records are the field's first consumer of the cubic nonagon field
//! `K₉ = Q(b)`, `b³ = 3b − 1` ([`K9`]) extended by `w = 2sin20°`,
//! `w² = 2 − b` (`n9::Nq`); supporting them added one variant
//! `Field::K9` to the shared field tag — a disclosed exception to
//! this module's file ownership, with zero other consumers
//! (grep-verified at landing).
//!
//! [`L10`]: crate::fields::L10
//! [`K9`]: crate::fields::K9
//! [`Medallion::contact_cos2`]: crate::medallion::Medallion::contact_cos2
//! [`Motif::Star`]: crate::families::islamic::hankin_motifs::Motif::Star
//! [`extended_segments`]: crate::families::islamic::hankin_motifs
//! [`Tiling`]: tess_core::Tiling

pub mod dec10;
pub mod dod12;
pub mod n9;
pub mod oct8;

use std::cmp::Ordering;

use super::Zq;
use crate::medallion::girih_m161::Kq;

/// Which symmetry axis of the hull carries the displacement `P`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Axis {
    /// The long vertex diagonal `V_{n/2} - V_0`.
    VertexAxis,
    /// The opposite-edge-midpoint axis `M_{n/2} - M_0`.
    MidEdgeAxis,
}

// =====================================================================
// Generic exact layer — one implementation over Q(sqrt2), Q(sqrt3)
// and L10. All sign/angle tests cross-multiply; nothing divides exact
// data.
// =====================================================================

/// Element arithmetic shared by the two exact coordinate types shipped
/// for medallion records ([`Zq`] for the quadratic fields, [`Kq`] for
/// L10). Methods resolve to the inherent implementations on concrete
/// types; the `_like` constructors take `&self` as a field witness
/// because `Zq` carries its discriminant.
pub trait FieldEl: Copy {
    fn add(self, o: Self) -> Self;
    fn sub(self, o: Self) -> Self;
    fn neg(self) -> Self;
    fn mul(self, o: Self) -> Self;
    fn sgn(self) -> Ordering;
    fn eq_x(self, o: Self) -> bool;
    fn fl(self) -> f64;
    /// Zero of this element's field.
    fn zero_like(&self) -> Self;
    /// The integer `v` in this element's field.
    fn int_like(&self, v: i64) -> Self;
    /// `self * num / den`, `den > 0` — an exact scaling kept as a
    /// denominator (like [`super::div_i`]); no division happens.
    fn scaled_div(self, num: i64, den: i64) -> Self;
}

impl FieldEl for Zq {
    fn add(self, o: Self) -> Self {
        Zq::add(self, o)
    }
    fn sub(self, o: Self) -> Self {
        Zq::sub(self, o)
    }
    fn neg(self) -> Self {
        Zq::neg(self)
    }
    fn mul(self, o: Self) -> Self {
        Zq::mul(self, o)
    }
    fn sgn(self) -> Ordering {
        Zq::sgn(self)
    }
    fn eq_x(self, o: Self) -> bool {
        Zq::eq_x(self, o)
    }
    fn fl(self) -> f64 {
        Zq::fl(self)
    }
    fn zero_like(&self) -> Self {
        Zq::zero(self.d)
    }
    fn int_like(&self, v: i64) -> Self {
        Zq::whole(self.d, v)
    }
    fn scaled_div(self, num: i64, den: i64) -> Self {
        Zq::new(self.a * num, self.b * num, self.d, self.den * den)
    }
}

impl FieldEl for Kq {
    fn add(self, o: Self) -> Self {
        Kq::add(self, o)
    }
    fn sub(self, o: Self) -> Self {
        Kq::sub(self, o)
    }
    fn neg(self) -> Self {
        Kq::neg(self)
    }
    fn mul(self, o: Self) -> Self {
        Kq::mul(self, o)
    }
    fn sgn(self) -> Ordering {
        Kq::sgn(self)
    }
    fn eq_x(self, o: Self) -> bool {
        Kq::eq_x(self, o)
    }
    fn fl(self) -> f64 {
        Kq::fl(self)
    }
    fn zero_like(&self) -> Self {
        Kq::zero()
    }
    fn int_like(&self, v: i64) -> Self {
        Kq::new([v, 0, 0, 0], 1)
    }
    fn scaled_div(self, num: i64, den: i64) -> Self {
        // `(n / d) * num / den = (n * num) / (d * den)` — the numerator
        // is scaled exactly (Kq::new gcd-reduces).
        Kq::new(self.n.scale(num).0, self.den * den)
    }
}

/// Exact planar point over a [`FieldEl`] coordinate type.
pub type Gp<F> = (F, F);

/// Homogeneous exact point `((x, y), w)` standing for `(x/w, y/w)` —
/// used only for intermediate crossing nodes; every catalogued crossing
/// equals an exact hull-edge midpoint (asserted), so nothing ships
/// homogeneous. Weights are never sign-normalized: only [`hom_eq`]
/// (cross-multiplication) compares them.
pub(crate) type Hom<F> = (Gp<F>, F);

fn gadd<F: FieldEl>(p: Gp<F>, q: Gp<F>) -> Gp<F> {
    (p.0.add(q.0), p.1.add(q.1))
}
fn gsub<F: FieldEl>(p: Gp<F>, q: Gp<F>) -> Gp<F> {
    (p.0.sub(q.0), p.1.sub(q.1))
}
/// Exact integer scaling (a field multiply by an integer).
fn gsmul<F: FieldEl>(p: Gp<F>, k: i64) -> Gp<F> {
    let f = p.0.int_like(k);
    (p.0.mul(f), p.1.mul(f))
}
fn gmul_i<F: FieldEl>(x: F, k: i64) -> F {
    x.mul(x.int_like(k))
}
fn gdot<F: FieldEl>(u: Gp<F>, v: Gp<F>) -> F {
    u.0.mul(v.0).add(u.1.mul(v.1))
}
fn gcross<F: FieldEl>(u: Gp<F>, v: Gp<F>) -> F {
    u.0.mul(v.1).sub(u.1.mul(v.0))
}

/// Orientation of the triple `(a, b, c)`: `cross(b - a, c - a)`.
fn gcross3<F: FieldEl>(a: Gp<F>, b: Gp<F>, c: Gp<F>) -> F {
    gcross(gsub(b, a), gsub(c, a))
}

/// Strictly inside a strictly CCW convex polygon: left of every edge.
fn ginside<F: FieldEl>(p: Gp<F>, poly: &[Gp<F>]) -> bool {
    let n = poly.len();
    (0..n).all(|i| gcross3(poly[i], poly[(i + 1) % n], p).sgn() == Ordering::Greater)
}

/// Edges of `a` with **both endpoints** strictly inside `b` (convexity
/// makes this exactly the edges lying strictly inside).
fn gedges_inside<F: FieldEl>(a: &[Gp<F>], b: &[Gp<F>]) -> Vec<usize> {
    let n = a.len();
    (0..n)
        .filter(|&i| ginside(a[i], b) && ginside(a[(i + 1) % n], b))
        .collect()
}

/// Exact midpoints of a polygon's edges.
pub(crate) fn gmidpoints<F: FieldEl>(poly: &[Gp<F>]) -> Vec<Gp<F>> {
    let n = poly.len();
    (0..n)
        .map(|i| {
            let (a, b) = (poly[i], poly[(i + 1) % n]);
            (
                a.0.scaled_div(1, 2).add(b.0.scaled_div(1, 2)),
                a.1.scaled_div(1, 2).add(b.1.scaled_div(1, 2)),
            )
        })
        .collect()
}

/// Centre `Sum(V) / n` — exact (a kept denominator).
fn gcentre<F: FieldEl>(poly: &[Gp<F>]) -> Gp<F> {
    let z = poly[0].0.zero_like();
    let sum = poly
        .iter()
        .fold((z, z), |s, p| (s.0.add(p.0), s.1.add(p.1)));
    let n = poly.len() as i64;
    (sum.0.scaled_div(1, n), sum.1.scaled_div(1, n))
}

fn hom_of<F: FieldEl>(p: Gp<F>) -> Hom<F> {
    (p, p.0.int_like(1))
}

fn hom_eq<F: FieldEl>(a: &Hom<F>, b: &Hom<F>) -> bool {
    a.0 .0.mul(b.1).eq_x(b.0 .0.mul(a.1)) && a.0 .1.mul(b.1).eq_x(b.0 .1.mul(a.1))
}

/// Hull sanity (the 24a schema, doc §1.2): strictly CCW convex, equal
/// unit edges, normalized frame (V0 = origin, edge 0 along +x).
fn hull_sanity<F: FieldEl>(name: &str, hull: &[Gp<F>]) {
    let n = hull.len();
    assert!(
        (0..n).all(|i| gcross3(hull[i], hull[(i + 1) % n], hull[(i + 2) % n]).sgn()
            == Ordering::Greater),
        "{name}: hull not strictly ccw"
    );
    let e0 = gdot(gsub(hull[1], hull[0]), gsub(hull[1], hull[0]));
    assert!(
        (1..n).all(|i| gdot(gsub(hull[(i + 1) % n], hull[i]), gsub(hull[(i + 1) % n], hull[i]))
            .eq_x(e0)),
        "{name}: hull edges not equal"
    );
    assert!(e0.eq_x(e0.int_like(1)), "{name}: hull not unit-edged");
    assert!(
        hull[0].0.eq_x(hull[0].0.zero_like())
            && hull[0].1.eq_x(hull[0].1.zero_like())
            && hull[1].1.eq_x(hull[1].1.zero_like())
            && hull[1].0.eq_x(e0.int_like(1)),
        "{name}: hull not normalized"
    );
}

// ---------------------------------------------------------------------
// Transversal crossings (division-free)
// ---------------------------------------------------------------------

/// One transversal crossing of the boundaries of `a` and `b`: `a`-edge
/// `ea` at `t = t_num / den`, `b`-edge `eb` at `s = s_num / den` (both
/// strictly in `(0, 1)`), with the homogeneous point `p + u * t`
/// represented as `(p * den + u * t_num) / den`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct CrossItem<F> {
    pub ea: usize,
    pub eb: usize,
    pub t_num: F,
    pub s_num: F,
    pub den: F,
    pub x: Hom<F>,
}

/// All transversal boundary crossings of two polygons (parameters
/// compared by sign; parallel pairs skipped, so collinear-overlap
/// regimes contribute zero).
fn gcrossings<F: FieldEl>(a: &[Gp<F>], b: &[Gp<F>]) -> Vec<CrossItem<F>> {
    let (na, nb) = (a.len(), b.len());
    let mut out = Vec::new();
    for i in 0..na {
        let (p, q) = (a[i], a[(i + 1) % na]);
        let u = gsub(q, p);
        for j in 0..nb {
            let (r, s2) = (b[j], b[(j + 1) % nb]);
            let v = gsub(s2, r);
            let den = gcross(u, v);
            let sd = den.sgn();
            if sd == Ordering::Equal {
                continue;
            }
            let w = gsub(r, p);
            let tn = gcross(w, v);
            // p + t u = r + s v  =>  s = -cross(u, w) / den; store the
            // numerator over the SAME denominator as t.
            let sn = gcross(u, w).neg();
            // `0 < tn / den < 1` strictly, without dividing:
            // sgn(num) == sgn(den) and sgn(num - den) == -sgn(den).
            let strict_unit = |num: F| {
                num.sgn() == sd && num.sub(den).sgn() == sd.reverse()
            };
            if strict_unit(tn) && strict_unit(sn) {
                out.push(CrossItem {
                    ea: i,
                    eb: j,
                    t_num: tn,
                    s_num: sn,
                    den,
                    x: (
                        (
                            p.0.mul(den).add(u.0.mul(tn)),
                            p.1.mul(den).add(u.1.mul(tn)),
                        ),
                        den,
                    ),
                });
            }
        }
    }
    out
}

// ---------------------------------------------------------------------
// Outlines (exact closed traversals)
// ---------------------------------------------------------------------

/// Boundary node label in an outline traversal.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OutNode {
    /// Vertex `k` of hull `A`.
    A(usize),
    /// Vertex `k` of hull `B = A + P`.
    B(usize),
    /// Crossing `k` (index into the entry's crossing table).
    X(usize),
}

/// The vertices `V_{start+1}, .., V_{end}` walked forward (cyclically)
/// along a polygon boundary from a crossing on edge `start` to a
/// crossing on edge `end` (both endpoints included; never empty).
fn arc_nodes(start: usize, end: usize, n: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut k = (start + 1) % n;
    loop {
        v.push(k);
        if k == end {
            return v;
        }
        k = (k + 1) % n;
    }
}

/// The two overlap outlines as single closed cycles (cyclic order; the
/// closing edge back to element 0 is implied):
/// * **union**: the exposed arcs — from crossing X0 along `A` through
///   its exposed vertices to X1, then back along `B` (reversed);
/// * **lens** (overlap boundary): the interior arcs through the hidden
///   vertices, same construction.
///
/// `ea`/`eb` are the crossing edge indices on `A`/`B` (`ea[k]` = the
/// `A`-edge of crossing `k`); `exposed_*` classify every vertex.
fn outlines(
    n: usize,
    exposed_a: &[bool],
    exposed_b: &[bool],
    ea: (usize, usize),
    eb: (usize, usize),
) -> (Vec<OutNode>, Vec<OutNode>) {
    let uniform = |tag: &str, vs: &[usize], ex: &[bool]| -> bool {
        assert!(!vs.is_empty(), "{tag}: empty arc");
        let e = ex[vs[0]];
        assert!(
            vs.iter().all(|&k| ex[k] == e),
            "{tag}: arc mixes exposed/hidden vertices"
        );
        e
    };
    let a0 = arc_nodes(ea.0, ea.1, n);
    let a1 = arc_nodes(ea.1, ea.0, n);
    let b0 = arc_nodes(eb.0, eb.1, n);
    let b1 = arc_nodes(eb.1, eb.0, n);
    let a0_out = uniform("A arc X0->X1", &a0, exposed_a);
    let a1_out = uniform("A arc X1->X0", &a1, exposed_a);
    let b0_out = uniform("B arc X0->X1", &b0, exposed_b);
    let b1_out = uniform("B arc X1->X0", &b1, exposed_b);
    assert_ne!(a0_out, a1_out, "A arcs are complementary");
    assert_ne!(b0_out, b1_out, "B arcs are complementary");
    // The two simple closed cycles through both crossings are
    // C1 = X0 -a0-> X1 -b1-> X0 and C2 = X0 -b0-> X1 -a1-> X0 (every
    // arc walked forward from the crossing on its start edge). Exactly
    // one of them is the union boundary, so their arcs agree in
    // exposedness and C1 is the union iff a0 (equivalently b1) is.
    assert_eq!(a0_out, b1_out, "outline arcs must pair across crossings");
    let cyc = |head: &[usize],
               tail: &[usize],
               head_tag: fn(usize) -> OutNode,
               tail_tag: fn(usize) -> OutNode| -> Vec<OutNode> {
        let mut v = vec![OutNode::X(0)];
        v.extend(head.iter().map(|&k| head_tag(k)));
        v.push(OutNode::X(1));
        v.extend(tail.iter().map(|&k| tail_tag(k)));
        v
    };
    let c1 = cyc(&a0, &b1, OutNode::A, OutNode::B);
    let c2 = cyc(&b0, &a1, OutNode::B, OutNode::A);
    if a0_out {
        (c1, c2)
    } else {
        (c2, c1)
    }
}

// ---------------------------------------------------------------------
// The shared {n/d} star-record battery (every catalog record follows
// the khatam M53 pattern: contacts = edge midpoints, figure = chords
// (i, i + d), exits = both chord continuations at every contact).
// ---------------------------------------------------------------------

/// The `{n/d}` figure as contact-index pairs.
pub(crate) fn star_figure(n: usize, d: usize) -> Vec<(usize, usize)> {
    (0..n).map(|i| (i, (i + d) % n)).collect()
}

/// The `2n` directed exit rays `(contact, direction)`: both chord
/// continuations `M_i - M_{i +/- d}` at every contact. For `2d = n` the
/// two rows per contact coincide (the {12/6} case — recorded as is).
pub(crate) fn star_exits<F: FieldEl>(m: &[Gp<F>], d: usize) -> Vec<(usize, Gp<F>)> {
    let n = m.len();
    let mut out = Vec::with_capacity(2 * n);
    for s in [d, n - d] {
        for i in 0..n {
            out.push((i, gsub(m[i], m[(i + s) % n])));
        }
    }
    out
}

/// The 24a module checks for a star-record, exactly: hull schema, the
/// figure spanning every contact with degree 2, `2n` exits that are the
/// chord continuations, outwardness against the centre, and the exact
/// contact-angle identity `cos2 * |chord|^2 * |edge|^2 = dot^2`
/// (`cos2 = cos^2(d pi/n)`, cross-multiplied into the field; `cos2 = 0`
/// degenerates to exact orthogonality).
pub fn battery_record<F: FieldEl>(name: &str, hull: &[Gp<F>], d: usize, cos2: F) {
    let n = hull.len();
    hull_sanity(name, hull);
    assert!(d >= 1 && 2 * d <= n, "{name}: figure step out of range");
    let m = gmidpoints(hull);
    let fig = star_figure(n, d);
    for i in 0..n {
        assert_eq!(
            fig.iter().filter(|(a, b)| *a == i || *b == i).count(),
            2,
            "{name}: contact {i} figure degree"
        );
    }
    let ex = star_exits(&m, d);
    assert_eq!(ex.len(), 2 * n, "{name}: exit count");
    for s in [d, n - d] {
        for i in 0..n {
            let want = gsub(m[i], m[(i + s) % n]);
            assert!(
                ex.iter()
                    .any(|(c, v)| *c == i && v.0.eq_x(want.0) && v.1.eq_x(want.1)),
                "{name}: exit continuation missing at contact {i} (+{s})"
            );
        }
    }
    let c = gcentre(hull);
    for (i, v) in &ex {
        let w = gsub(m[*i], c);
        assert_eq!(
            gdot(*v, w).sgn(),
            Ordering::Greater,
            "{name}: exit at contact {i} not outward"
        );
    }
    // Contact angle: cos^2 between figure chord and hull edge.
    for &(a, b) in &fig {
        let chord = gsub(m[b], m[a]);
        let edge = gsub(hull[(a + 1) % n], hull[a]);
        let lhs = cos2.mul(gdot(chord, chord)).mul(gdot(edge, edge));
        assert!(
            lhs.eq_x(gdot(chord, edge).mul(gdot(chord, edge))),
            "{name}: contact angle cos^2 identity at {a}"
        );
    }
}

// ---------------------------------------------------------------------
// The placement record: spec, verifier, certified data
// ---------------------------------------------------------------------

/// Verified overlap data for one monster pair (the catalog entry, as
/// produced by [`verify_pair`], which asserts every claimed identity).
pub struct PairData<F> {
    /// Exact displacement `B = A + P`.
    pub p: Gp<F>,
    /// `A`-edges strictly inside `B` (the definition count, `c` of them).
    pub e_ab: Vec<usize>,
    /// `B`-edges strictly inside `A` = `{i + n/2}` (the half-turn image).
    pub e_ba: Vec<usize>,
    /// `A` vertices not strictly inside `B`, ascending (`n - c - 1`).
    pub exposed_a: Vec<usize>,
    /// `B` vertices not strictly inside `A`, ascending.
    pub exposed_b: Vec<usize>,
    /// The two crossing points, exact (each is a midpoint of both an
    /// `A`-edge and a `B`-edge).
    pub cross_pts: Vec<Gp<F>>,
    /// Union outline: one exact closed traversal.
    pub union_cycle: Vec<OutNode>,
    /// Overlap (lens) outline: one exact closed traversal.
    pub lens_cycle: Vec<OutNode>,
}

/// A certified-window probe: place `B` at `rho * D` and require the
/// stated `(cAB, cBA)` counts (and transversal-crossing count).
pub struct WindowProbe<F> {
    pub rho: F,
    pub ca: usize,
    pub cb: usize,
    pub crossings: Option<usize>,
}

/// Catalog entry input to the shared exact verifier: the record hull,
/// the placement, the forced angle and the certified tables.
pub struct PairSpec<F: FieldEl> {
    /// Label for assertion messages.
    pub name: &'static str,
    /// Normalized unit-edge hull of the record (vertex 0 at origin).
    pub hull_a: Vec<Gp<F>>,
    /// Exact `rho = |P| / |D|`; the displacement is `P = rho * D`.
    pub rho: F,
    /// Which axis carries `P`.
    pub axis: Axis,
    /// Star figure step: the contact angle is `d * pi / n`.
    pub d: usize,
    /// Certified `A`-edges strictly inside `B`.
    pub e_ab: Vec<usize>,
    /// Certified crossing edge pairs `(A_i, B_j)`, sorted by `A`-edge.
    pub cross: Vec<(usize, usize)>,
    /// Exact `cos^2(d pi / n)` (crossing angle = figure contact angle).
    pub cos2: F,
}

/// No vertex of either polygon lies on an edge SEGMENT of the other.
/// Collinearity with the extended edge line is allowed (the
/// displacement runs along a symmetry axis, so vertices routinely sit
/// on extended opposite-edge lines); only on-segment contact would be
/// a degeneracy that breaks the transversal-crossing outline model.
/// A vertex `q` is on the segment `[o_j, o_{j+1}]` iff it is collinear
/// and `0 <= dot(q - o_j, e) <= |e|^2` with `e = o_{j+1} - o_j`.
fn assert_general_position<F: FieldEl>(a: &[Gp<F>], b: &[Gp<F>], name: &str) {
    let n = a.len();
    for (poly, other, tag) in [(a, b, "A"), (b, a, "B")] {
        for (k, &q) in poly.iter().enumerate() {
            for j in 0..n {
                let e = gsub(other[(j + 1) % n], other[j]);
                let w = gsub(q, other[j]);
                if gcross(e, w).sgn() != Ordering::Equal {
                    continue;
                }
                let t2 = gdot(w, e);
                let e2 = gdot(e, e);
                let on_seg =
                    t2.sgn() != Ordering::Less && t2.sub(e2).sgn() != Ordering::Greater;
                assert!(
                    !on_seg,
                    "{}: vertex {}{} on edge segment {}{} (degenerate)",
                    name,
                    tag,
                    k,
                    if tag == "A" { "B" } else { "A" },
                    j
                );
            }
        }
    }
}

/// The full exact verification of one monster placement (module docs):
/// hull schema, general position, the definition counts both ways with
/// the half-turn pairing, absorbed contacts, exactly two transversal
/// crossings at the edge-midpoint pin (`P = M_i - M_j`), the crossing
/// angle identity, axis/mirror symmetry, the exact outline cycles, and
/// the certified window (pin interior: the `pin +/- D/64` samples keep
/// the counts and leave the pin; the tabulated out-of-window probes
/// behave as documented). Returns the certified [`PairData`].
pub(crate) fn verify_pair<F: FieldEl>(spec: &PairSpec<F>, probes: &[WindowProbe<F>]) -> PairData<F> {
    let a: &[Gp<F>] = &spec.hull_a;
    let n = a.len();
    assert!(n % 2 == 0, "{}: n even", spec.name);
    let half = n / 2;
    hull_sanity(spec.name, a);

    // Placement: P = rho * D along the declared axis (rho != 0).
    let m = gmidpoints(a);
    assert_ne!(spec.rho.sgn(), Ordering::Equal, "{}: rho is zero", spec.name);
    let dvec = match spec.axis {
        Axis::VertexAxis => gsub(a[half], a[0]),
        Axis::MidEdgeAxis => gsub(m[half], m[0]),
    };
    let p = (spec.rho.mul(dvec.0), spec.rho.mul(dvec.1));
    let b: Vec<Gp<F>> = a.iter().map(|&q| gadd(q, p)).collect();

    // General position at the pin: no vertex of either polygon lies on
    // an edge SEGMENT of the other (this excludes collinear edge
    // overlap and vertex touching). Line-level collinearities with the
    // extended edge lines do occur (the displacement runs along a
    // symmetry axis), always strictly outside the segments.
    assert_general_position(a, &b, spec.name);

    // Definition: exactly c full edges strictly inside, both ways, the
    // certified index lists, and eBA = {i + n/2} (half-turn image).
    let e_ab = gedges_inside(a, &b);
    let e_ba = gedges_inside(&b, a);
    assert_eq!(e_ab, spec.e_ab, "{}: eAB", spec.name);
    let c = e_ab.len();
    assert!(c >= 2 && c <= half - 2, "{}: c outside legal band", spec.name);
    let want: Vec<usize> = {
        let mut v: Vec<usize> = spec.e_ab.iter().map(|&i| (i + half) % n).collect();
        v.sort_unstable();
        v
    };
    assert_eq!(e_ba, want, "{}: eBA != half-turn image", spec.name);
    // Absorbed contacts (interior edge midpoints) = interior edges.
    let absorbed_a: Vec<usize> = (0..n).filter(|&i| ginside(m[i], &b)).collect();
    assert_eq!(absorbed_a, e_ab, "{}: absorbed contacts A-in-B", spec.name);
    let absorbed_b: Vec<usize> = (0..n)
        .filter(|&i| ginside(gadd(m[i], p), a))
        .collect();
    assert_eq!(absorbed_b, e_ba, "{}: absorbed contacts B-in-A", spec.name);

    // Exactly two transversal crossings, at edge midpoints, on the
    // certified edge pairs, with P = M_i - M_j (the pinning identity)
    // and the angle identity cos^2 = cos^2(d pi / n).
    let cr = gcrossings(a, &b);
    assert_eq!(cr.len(), 2, "{}: not exactly 2 crossings", spec.name);
    let two = spec.rho.int_like(2);
    for (k, ci) in cr.iter().enumerate() {
        assert_eq!(ci.ea, spec.cross[k].0, "{}: crossing {k} A-edge", spec.name);
        assert_eq!(ci.eb, spec.cross[k].1, "{}: crossing {k} B-edge", spec.name);
        assert!(
            two.mul(ci.t_num).eq_x(ci.den) && two.mul(ci.s_num).eq_x(ci.den),
            "{}: crossing {k} not at the edge midpoints",
            spec.name
        );
        let ma = m[ci.ea];
        let mb = gadd(m[ci.eb], p);
        assert!(
            hom_eq(&ci.x, &hom_of(ma)) && hom_eq(&ci.x, &hom_of(mb)),
            "{}: crossing {k} off the midpoint",
            spec.name
        );
        let pin = gsub(ma, m[ci.eb]);
        assert!(
            p.0.sub(pin.0).sgn() == Ordering::Equal && p.1.sub(pin.1).sgn() == Ordering::Equal,
            "{}: P != M_{} - M_{}",
            spec.name,
            ci.ea,
            ci.eb
        );
        // Crossing angle: edges are unit, so cos^2 = dot^2. (Parallel
        // polygons: d = 2 (i - j) mod n is the forced figure step.)
        let u = gsub(a[(ci.ea + 1) % n], a[ci.ea]);
        let v = gsub(b[(ci.eb + 1) % n], b[ci.eb]);
        let lhs = gdot(u, v).mul(gdot(u, v));
        assert!(
            lhs.eq_x(spec.cos2),
            "{}: crossing cos^2 != cos^2(d pi/n)",
            spec.name
        );
        // Edge k has direction angle k*pi*2/n, so the line angle
        // between edges (i, j) is 2(i - j) pi/n and cos^2 of it only
        // depends on 2(i - j) mod n — the figure step d, canonicalized
        // to the figure {n/d} with 2d <= n.
        let dif = (2 * (ci.ea as i64 - ci.eb as i64)).rem_euclid(n as i64);
        let dcan = dif.min(n as i64 - dif);
        let scan = (spec.d as i64).min(n as i64 - spec.d as i64);
        assert_eq!(dcan, scan, "{}: d != 2 (i - j) mod n", spec.name);
    }
    // The half-turn maps (A_i, B_j) -> (A_{j+n/2}, B_{i+n/2}).
    assert_eq!(cr[1].ea, (cr[0].eb + half) % n, "{}: crossing swap image", spec.name);
    assert_eq!(cr[1].eb, (cr[0].ea + half) % n, "{}: crossing swap image", spec.name);

    // Exposed vertices: n - c - 1 per polygon.
    let exposed_a: Vec<usize> = (0..n).filter(|&k| !ginside(a[k], &b)).collect();
    let exposed_b: Vec<usize> = (0..n).filter(|&k| !ginside(b[k], a)).collect();
    assert_eq!(exposed_a.len(), n - c - 1, "{}: exposed A count", spec.name);
    assert_eq!(exposed_b.len(), n - c - 1, "{}: exposed B count", spec.name);

    // Half-turn lemma: 2 O = V_k + V_{k + n/2} for every k (so the
    // half-turn about the centre-midpoint maps A <-> B, V_k -> B_{k+n/2}).
    let o = gcentre(a);
    let rhs = (o.0.mul(two), o.1.mul(two));
    for k in 0..n {
        let lhs = gadd(a[k], a[(k + half) % n]);
        assert!(
            lhs.0.eq_x(rhs.0) && lhs.1.eq_x(rhs.1),
            "{}: half-turn lemma at k = {k}",
            spec.name
        );
    }

    // Axis: P = rho * D (parallel by construction, checked); D passes
    // through the centre; the mirror about the axis line acts as the
    // expected dihedral reflection on both hulls and swaps the
    // crossings.
    let sum = {
        let z = a[0].0.zero_like();
        a.iter().fold((z, z), |s, q| (s.0.add(q.0), s.1.add(q.1)))
    };
    assert_eq!(
        gcross(dvec, p).sgn(),
        Ordering::Equal,
        "{}: P not along the axis",
        spec.name
    );
    let n64 = n as i64;
    let anchor = match spec.axis {
        Axis::VertexAxis => a[0],
        Axis::MidEdgeAxis => m[0],
    };
    assert_eq!(
        gcross(dvec, gsub(gsmul(anchor, n64), sum)).sgn(),
        Ordering::Equal,
        "{}: axis diagonal misses the centre",
        spec.name
    );
    let dd = gdot(dvec, dvec);
    let vmap = |k: usize| match spec.axis {
        Axis::VertexAxis => (n - k) % n,
        Axis::MidEdgeAxis => (1 + n - k) % n,
    };
    // Reflection identity (n-scaled, exact): for Xn = n X - Sum(V),
    // sigma(X) = Y  <=>  2 (Xn . D) D = dd (Xn + Yn).
    let refl_ok = |x: Gp<F>, y: Gp<F>| {
        let xn = gsub(gsmul(x, n64), sum);
        let yn = gsub(gsmul(y, n64), sum);
        let k2 = gdot(xn, dvec).mul(two);
        let l = (k2.mul(dvec.0), k2.mul(dvec.1));
        let r = (dd.mul(xn.0.add(yn.0)), dd.mul(xn.1.add(yn.1)));
        l.0.sub(r.0).sgn() == Ordering::Equal && l.1.sub(r.1).sgn() == Ordering::Equal
    };
    for k in 0..n {
        assert!(
            refl_ok(a[k], a[vmap(k)]),
            "{}: mirror fails at A vertex {k}",
            spec.name
        );
        assert!(
            refl_ok(b[k], b[vmap(k)]),
            "{}: mirror fails at B vertex {k}",
            spec.name
        );
    }
    let xr = hom_reflect(&cr[0].x, n64, sum, dvec, dd);
    assert!(
        hom_eq(&xr, &cr[1].x),
        "{}: mirror does not swap the crossings",
        spec.name
    );

    // Outlines: exact closed cycles over exposed/hidden vertices and
    // the two crossings, with multiset checks.
    let flag_a: Vec<bool> = (0..n).map(|k| !ginside(a[k], &b)).collect();
    let flag_b: Vec<bool> = (0..n).map(|k| !ginside(b[k], &a)).collect();
    let (union_cycle, lens_cycle) = outlines(
        n,
        &flag_a,
        &flag_b,
        (cr[0].ea, cr[1].ea),
        (cr[0].eb, cr[1].eb),
    );
    assert_eq!(
        union_cycle.len(),
        exposed_a.len() + exposed_b.len() + 2,
        "{}: union cycle size",
        spec.name
    );
    assert_eq!(lens_cycle.len(), 2 * c + 4, "{}: lens cycle size", spec.name);
    let collect = |cyc: &[OutNode], want: &mut Vec<usize>, side: u8| {
        *want = cyc
            .iter()
            .filter_map(|t| match *t {
                OutNode::A(k) if side == 0 => Some(k),
                OutNode::B(k) if side == 1 => Some(k),
                _ => None,
            })
            .collect();
        want.sort_unstable();
    };
    let mut got = Vec::new();
    collect(&union_cycle, &mut got, 0);
    assert_eq!(got, exposed_a, "{}: union A-vertices", spec.name);
    collect(&union_cycle, &mut got, 1);
    assert_eq!(got, exposed_b, "{}: union B-vertices", spec.name);
    let hidden_a: Vec<usize> = (0..n).filter(|k| ginside(a[*k], &b)).collect();
    collect(&lens_cycle, &mut got, 0);
    assert_eq!(got, hidden_a, "{}: lens A-vertices", spec.name);
    for cyc in [&union_cycle, &lens_cycle] {
        assert_eq!(
            cyc.iter().filter(|t| matches!(**t, OutNode::X(_))).count(),
            2,
            "{}: outline crossing nodes",
            spec.name
        );
    }

    // Window: the pin is strictly interior — the pin +/- D/64 samples
    // stay general position, keep (c, c) with the same eAB list and 2
    // transversal crossings, and are NOT at the pin (t != 1/2).
    for sgnk in [1i64, -1] {
        let off = gsmul(
            (
                dvec.0.scaled_div(1, 64),
                dvec.1.scaled_div(1, 64),
            ),
            sgnk,
        );
        let p2 = gadd(p, off);
        let b2: Vec<Gp<F>> = a.iter().map(|&q| gadd(q, p2)).collect();
        assert_general_position(a, &b2, spec.name);
        let ca = gedges_inside(a, &b2);
        let cb = gedges_inside(&b2, a);
        let cr2 = gcrossings(a, &b2);
        assert_eq!(ca, e_ab, "{}: window sample eAB", spec.name);
        assert_eq!(cb.len(), c, "{}: window sample eBA count", spec.name);
        assert_eq!(cr2.len(), 2, "{}: window sample crossings", spec.name);
        assert!(
            !two.mul(cr2[0].t_num).eq_x(cr2[0].den),
            "{}: window sample still at the pin",
            spec.name
        );
    }
    // Tabulated out-of-window probes.
    for pr in probes {
        let w = (pr.rho.mul(dvec.0), pr.rho.mul(dvec.1));
        let b3: Vec<Gp<F>> = a.iter().map(|&q| gadd(q, w)).collect();
        let ca = gedges_inside(a, &b3).len();
        let cb = gedges_inside(&b3, a).len();
        assert_eq!(
            (ca, cb),
            (pr.ca, pr.cb),
            "{}: window probe at rho {:?}",
            spec.name,
            pr.rho.fl()
        );
        if let Some(k) = pr.crossings {
            assert_eq!(
                gcrossings(a, &b3).len(),
                k,
                "{}: window probe crossings",
                spec.name
            );
        }
    }

    PairData {
        p,
        e_ab,
        e_ba,
        exposed_a,
        exposed_b,
        cross_pts: cr.iter().map(|ci| m[ci.ea]).collect(),
        union_cycle,
        lens_cycle,
    }
}

/// Mirror of a homogeneous crossing about the axis line (`X = Xh / w`,
/// `U = n Xh - w Sum(V)` is `n w` times `X - O`; the reflection is
/// `O + (2 (U.D) D - U dd) / (nw dd)` = `(dd w Sum(V) + N, n w dd)`).
fn hom_reflect<F: FieldEl>(h: &Hom<F>, n: i64, sum: Gp<F>, dvec: Gp<F>, dd: F) -> Hom<F> {
    let u = (
        gmul_i(h.0 .0, n).sub(sum.0.mul(h.1)),
        gmul_i(h.0 .1, n).sub(sum.1.mul(h.1)),
    );
    let two = u.0.int_like(2);
    let k = gdot(u, dvec).mul(two);
    let num = (
        dvec.0.mul(k).sub(dd.mul(u.0)),
        dvec.1.mul(k).sub(dd.mul(u.1)),
    );
    (
        (
            sum.0.mul(dd).mul(h.1).add(num.0),
            sum.1.mul(dd).mul(h.1).add(num.1),
        ),
        gmul_i(h.1, n).mul(dd),
    )
}

/// The catalog's (name, n, c, d) table: every legal (n, c) for
/// n in {8, 9, 10, 12}, each once (the legality band `c <=
/// ceil(n / 2) - 2` is re-derived in the catalog test).
pub fn catalog_pairs() -> Vec<(&'static str, usize, usize, usize)> {
    vec![
        ("(8,2)", 8, 2, 2),
        ("(9,2)", 9, 2, 3),
        ("(9,3)", 9, 3, 1),
        ("(10,2)", 10, 2, 4),
        ("(10,3)", 10, 3, 2),
        ("(12,2)", 12, 2, 6),
        ("(12,3)", 12, 3, 4),
        ("(12,4)", 12, 4, 2),
    ]
}

// =====================================================================
// Tests: catalog legality, the landed M163 cross-check, and the
// finiteness / field-membership battery (the M164 lesson).
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::families::islamic::hankin_motifs::{motif_segments, Motif};
    use crate::medallion::gcd5;
    use crate::medallion::khatam_m53::KhatamM53;
    use crate::medallion::rosette_m56::RosetteM56;
    use crate::medallion::monster::dec10::{Monster102, Monster104};
    use crate::medallion::monster::dod12::{Monster122, Monster126};
    use crate::medallion::monster::n9::{Monster91, Monster93};
    use crate::medallion::Medallion;
    use tess_core::Vec2;

    #[test]
    fn catalog_is_the_legal_band_exactly_once() {
        // Legal band c in [2, ceil(n/2) - 2] (= n/2 - 2 for even n);
        // the catalog realizes each (n, c) in n in {8, 9, 10, 12}
        // exactly once, and no other n. n = 9 is the odd entry: the
        // rotated pi/9 family (parallel pairs cannot pin odd n), and
        // its exact census found exactly c in {2, 3}.
        let cat = catalog_pairs();
        assert_eq!(cat.len(), 8);
        let mut seen: Vec<(usize, usize)> = Vec::new();
        for (_, n, c, d) in &cat {
            assert!([8usize, 9, 10, 12].contains(n), "n = {n} out of scope");
            assert!(
                *c >= 2 && *c <= (n + 1) / 2 - 2,
                "(n, c) = ({n}, {c}) illegal"
            );
            assert!(*d * 2 == *n || 2 * *d < *n, "d = {d} at n = {n}");
            let probe = (*n, *c);
            assert!(!seen.contains(&probe), "(n, c) = {probe:?} duplicated");
            seen.push(probe);
        }
        for n in [8usize, 9, 10, 12] {
            for c in 2..=((n + 1) / 2 - 2) {
                assert!(seen.contains(&(n, c)), "({n}, {c}) missing");
            }
        }
        // Forced d per (n, c), from the 2-D crossing-angle census:
        let want = [
            ((8, 2), 2),
            ((9, 2), 3),
            ((9, 3), 1),
            ((10, 2), 4),
            ((10, 3), 2),
            ((12, 2), 6),
            ((12, 3), 4),
            ((12, 4), 2),
        ];
        for (nc, d) in want {
            let got = cat.iter().find(|(_, n, c, _)| (*n, *c) == nc).unwrap();
            assert_eq!(got.3, d, "d forced at {nc:?}");
        }
    }

    #[test]
    fn all_eight_placements_verify_their_certified_tables() {
        // Each constructor runs the full exact battery (asserts); the
        // per-entry data checks live in the entry modules.
        let e = oct8::pair82();
        assert_eq!(e.e_ab.len(), 2);
        let e = dec10::pair102();
        assert_eq!(e.e_ab.len(), 2);
        let e = dec10::pair103();
        assert_eq!(e.e_ab.len(), 3);
        let e = dod12::pair122();
        assert_eq!(e.e_ab.len(), 2);
        let e = dod12::pair123();
        assert_eq!(e.e_ab.len(), 3);
        let e = dod12::pair124();
        assert_eq!(e.e_ab.len(), 4);
        let e = n9::pair92();
        assert_eq!(e.e_ab.len(), 2);
        let e = n9::pair93();
        assert_eq!(e.e_ab.len(), 3);
    }

    /// M163 cross-check: the landed standalone [`Motif::Star`] at
    /// `theta = d * 180 / n` on the record's own regular hull emits
    /// segments exactly along the record exit-ray lines (and every
    /// exit line is realized). f64 only — the engine is landed float
    /// machinery; the record side is exact and its float view is what
    /// the engine would consume.
    fn m163_star_matches<R: Medallion>(name: &str, n: usize, d: usize) {
        let tile: Vec<Vec2> = R::hull_f64().iter().map(|p| Vec2::new(p[0], p[1])).collect();
        let theta = d as f64 * 180.0 / n as f64;
        let segs = motif_segments(&tile, &Motif::Star { n, theta_deg: theta, s: 1 });
        assert!(!segs.is_empty(), "{name}: M163 star emitted nothing");
        let mids: Vec<Vec2> = (0..n)
            .map(|i| {
                Vec2::new(
                    (tile[i].x + tile[(i + 1) % n].x) / 2.0,
                    (tile[i].y + tile[(i + 1) % n].y) / 2.0,
                )
            })
            .collect();
        let exits: Vec<(usize, Vec2)> = R::exit_rays_f64()
            .into_iter()
            .map(|(i, dd)| {
                let v = Vec2::new(dd[0], dd[1]);
                (i, v * (1.0 / v.length()))
            })
            .collect();
        let on_exit_line = |a: &Vec2, u: &Vec2| -> bool {
            exits.iter().any(|(i, q)| {
                u.cross(*q).abs() < 1e-8 && (mids[*i] - *a).cross(*u).abs() < 1e-8
            })
        };
        for (a, b) in &segs {
            let u = *b - *a;
            let u = u * (1.0 / u.length());
            assert!(
                on_exit_line(a, &u),
                "{name}: M163 segment off every record exit line"
            );
        }
        for (i, q) in &exits {
            let hit = segs.iter().any(|(a, b)| {
                let u = *b - *a;
                let u = u * (1.0 / u.length());
                u.cross(*q).abs() < 1e-8 && (mids[*i] - *a).cross(u).abs() < 1e-8
            });
            assert!(hit, "{name}: record exit at contact {i} not realized by M163");
        }
    }

    #[test]
    fn m163_star_matches_all_eight_catalog_records() {
        m163_star_matches::<KhatamM53>("(8,2) KhatamM53 {8/2}", 8, 2);
        m163_star_matches::<Monster104>("(10,2) {10/4}", 10, 4);
        m163_star_matches::<Monster102>("(10,3) {10/2}", 10, 2);
        m163_star_matches::<RosetteM56>("(12,3) RosetteM56 {12/4}", 12, 4);
        m163_star_matches::<Monster122>("(12,4) {12/2}", 12, 2);
        // (12,2): theta = 90 deg is outside the engine's EXTENSION
        // guard but legal for the standalone star (the two arm
        // chiralities coincide — the degenerate diameter case).
        m163_star_matches::<Monster126>("(12,2) {12/6}", 12, 6);
        m163_star_matches::<Monster93>("(9,2) {9/3}", 9, 3);
        // (9,3): theta = 20 deg = pi/n is exactly on the EXTENSION
        // guard boundary (same recorded-boundary precedent as (12,2)
        // at pi/2); motif_segments has no guard and emits the
        // midpoint-nonagon arms, matching the record exits.
        m163_star_matches::<Monster91>("(9,3) {9/1}", 9, 1);
    }

    fn zq_ok(name: &str, z: Zq, disc: i64) {
        assert_eq!(z.d, disc, "{name}: wrong field discriminant");
        assert!(z.den > 0, "{name}: non-positive denominator");
        assert!(z.a.abs() < 1 << 50 && z.b.abs() < 1 << 50, "{name}: magnitude");
        assert!(z.fl().is_finite(), "{name}: non-finite float view");
    }

    #[test]
    fn emitted_numbers_are_finite_in_field_and_canonical() {
        // The M164 lesson: every exact number the catalog emits is
        // checked for field membership (discriminant, positive
        // denominator, magnitude) and a finite f64 view; every
        // constructed table is in canonical (gcd-reduced) form.
        let oct = oct8::pair82();
        let dod = [dod12::pair122(), dod12::pair123(), dod12::pair124()];
        for pt in [oct.p].iter().chain(oct.cross_pts.iter()) {
            zq_ok("oct8 P/X", pt.0, 2);
            zq_ok("oct8 P/X", pt.1, 2);
        }
        for q in dod.iter() {
            for pt in [q.p].iter().chain(q.cross_pts.iter()) {
                zq_ok("dod12 P/X", pt.0, 3);
                zq_ok("dod12 P/X", pt.1, 3);
            }
        }
        for q in [dec10::pair102(), dec10::pair103()] {
            for pt in [q.p].iter().chain(q.cross_pts.iter()) {
                assert!(pt.0.den > 0 && pt.1.den > 0, "dec10 denominator");
                assert!(
                    pt.0.n.0.iter().all(|&v| v.abs() < (1i64 << 50))
                        && pt.1.n.0.iter().all(|&v| v.abs() < (1i64 << 50)),
                    "dec10 magnitude"
                );
                assert!(pt.0.fl().is_finite() && pt.1.fl().is_finite(), "dec10 finite");
            }
        }
        // Record views: hulls and exits, exact-membership and finite-float.
        for h in [KhatamM53::hull_f64(), RosetteM56::hull_f64(), Monster104::hull_f64(), Monster102::hull_f64(), Monster122::hull_f64(), Monster126::hull_f64()] {
            for p in &h {
                assert!(p[0].is_finite() && p[1].is_finite(), "hull_f64 finite");
            }
        }
        for ex in [KhatamM53::exit_rays_f64(), RosetteM56::exit_rays_f64(), Monster104::exit_rays_f64(), Monster102::exit_rays_f64(), Monster122::exit_rays_f64(), Monster126::exit_rays_f64()] {
            for (_, d) in &ex {
                assert!(d[0].is_finite() && d[1].is_finite(), "exit_rays_f64 finite");
            }
        }
        // Exact hulls: member-of-field with positive denominators.
        for p in oct8::hull() {
            zq_ok("oct8 hull", p.0, 2);
            zq_ok("oct8 hull", p.1, 2);
        }
        for p in dod12::hull() {
            zq_ok("dod12 hull", p.0, 3);
            zq_ok("dod12 hull", p.1, 3);
        }
        for p in dec10::hull() {
            assert!(p.0.den > 0 && p.1.den > 0, "dec10 hull denominator");
        }
        // Literal rho constants are gcd-reduced (canonical Zq/Kq):
        for (z, disc) in [
            (oct8::rho(), 2),
            (dod12::rho122(), 3),
            (dod12::rho123(), 3),
            (dod12::rho124(), 3),
        ] {
            assert!(z.den > 0 && gcd5([z.a, z.b, 0, 0, z.den]) == 1, "rho {disc} canonical");
            zq_ok("rho", z, disc);
        }
        for k in [dec10::rho102(), dec10::rho103()] {
            assert!(k.den > 0 && gcd5([k.n.0[0], k.n.0[1], k.n.0[2], k.n.0[3], k.den]) == 1, "rho L10 canonical");
            assert!(k.fl().is_finite(), "rho L10 finite");
        }
        // cos^2 constants: positive-denominator members of the entry
        // field (or zero, the {12/6} case).
        zq_ok("cos2 (8,2)", oct8::cos2(), 2);
        zq_ok("cos2 (12,2)", dod12::cos2122(), 3);
        zq_ok("cos2 (12,3)", dod12::cos2123(), 3);
        zq_ok("cos2 (12,4)", dod12::cos2124(), 3);
        for k in [dec10::cos2102(), dec10::cos2103()] {
            assert!(k.den > 0, "cos2 L10 denominator");
            assert!(k.fl().is_finite(), "cos2 L10 finite");
        }
        // n = 9: the K9(w) coordinate type Nq — positive denominator,
        // gcd-canonical across all seven integers, finite f64 view
        // (the M164 lesson repeated for the cubic extension).
        let nq_ok = |name: &str, q: n9::Nq| {
            assert!(q.den > 0, "{name}: non-positive denominator");
            assert!(q.is_canonical(), "{name}: not gcd-canonical");
            assert!(q.fl().is_finite(), "{name}: non-finite float view");
        };
        for pt in [n9::pin92(), n9::pin93()] {
            nq_ok("n9 pin", pt.0);
            nq_ok("n9 pin", pt.1);
        }
        for (lo, hi) in [n9::win92(), n9::win93()] {
            nq_ok("n9 window", lo);
            nq_ok("n9 window", hi);
        }
        nq_ok("cos2 (9,2)", n9::cos292());
        nq_ok("cos2 (9,3)", n9::cos293());
        for q in [n9::pair92(), n9::pair93()] {
            for pt in [q.p].iter().chain(q.cross_pts.iter()) {
                nq_ok("n9 P/X", pt.0);
                nq_ok("n9 P/X", pt.1);
            }
        }
        for p in n9::hull() {
            nq_ok("n9 hull", p.0);
            nq_ok("n9 hull", p.1);
        }
        for h in [Monster93::hull_f64(), Monster91::hull_f64()] {
            for p in &h {
                assert!(p[0].is_finite() && p[1].is_finite(), "n9 hull_f64 finite");
            }
        }
        for ex in [Monster93::exit_rays_f64(), Monster91::exit_rays_f64()] {
            for (_, d) in &ex {
                assert!(d[0].is_finite() && d[1].is_finite(), "n9 exit_rays_f64 finite");
            }
        }
    }
}
