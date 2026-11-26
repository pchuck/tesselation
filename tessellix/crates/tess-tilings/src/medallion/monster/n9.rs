//! The nonagon monsters: the catalog's first ODD-n records, on the
//! exact unit-edge regular nonagon hull — `{9/3}` for (9,2) and `{9/1}`
//! for (9,3).
//!
//! Odd n has NO parallel monster record: the parallel displacement
//! census at n = 9 (scratch `monster-n9/census*.txt`) shows no
//! axis-parallel midpoint pin (parity theorem T1), and the off-axis
//! midpoint pins (Δ = i − j ≡ 2, 7 mod 9) carry TWO distinct contact
//! angles (80° and 40°) — no single `{n/d}` figure describes them.
//! The odd-n analog is the ROTATED pair: `B = R·A + P` with `R` the
//! rotation by `pi/n = 20°`. Because `-U₀ = R·U₀` as VERTEX SETS for
//! odd n, the half-turn about the centre midpoint swaps `A` and `B`
//! for every displacement — `κ(X) = c_A + c_B − X` maps `V_k^A ->
//! V_(k+4)^B`, `M_k^A -> M_(k+4)^B` and (bijectively) `V_m^B ->
//! V_(m+5)^A` (asserted exactly below) — so `eBA = {i + 4}` stays
//! automatic and the pinning rule becomes `P = M_i − R·M_j`. The
//! forced figure step shifts with the rotation (`B`-edge directions
//! carry an extra `pi/n`): `d ≡ 2·(i − j) − 1 (mod n)`, canonical
//! `min(d, n − d)` — verified on both certified crossings of both
//! records.
//!
//! **Outline structure** (exact battery `walk_out.txt`, re-certified
//! in `verify_pair9`): the catalog counts hold — the union is ONE
//! closed cycle of `2(n−c−1)+2` nodes, the lens ONE of `2c+4`, with
//! `n − c − 1` exposed vertices per polygon — but the split differs
//! from the parallel records: the rotation shifts the index frames,
//! so the union runs after `X0` along the `B`-arc `V_(eb0+1) ..
//! V_(eb1)` and after `X1` along the `A`-arc `V_(ea1+1) .. V_(ea0)`
//! (the lens takes the complementary arcs). The absorbed runs' END
//! vertices `V_(ea1+1)`, `V_(eb1+1)` are exposed TRANSITION vertices:
//! they walk the union while the inner half of their edge closes the
//! lens beyond a crossing. The hub's [`super::outlines`] rederives
//! the same cycles from the certified exposed/crossing data
//! (cross-checked assert), and every cycle is re-certified piece by
//! piece with exact midpoint half-plane tests.
//!
//! Both records live in `N = K₉(w)`, `w² = 2 − b` (`w = 2 sin 20°`):
//! the rotated frame needs `sin(2π/9) ∉ K₉`, so coordinates ship as
//! [`Nq`] = `(u + v·w)/den` over [`K9`], and the trait's `Field` gains
//! the `K9` variant (additive, zero other consumers). `contact_cos2`
//! returns the rational `1/4` for `{9/3}` and `None` for `{9/1}`
//! (`(2+b)/4` is cubic — the dec10/L10 precedent; the battery pins it
//! exactly). See the [module docs](super) for the catalog, pinning and
//! window certification.

use super::{
    assert_general_position, gadd, gcentre, gcrossings, gdot, gedges_inside, ginside, gmidpoints,
    gmul_i, gsub, hom_eq, hom_of, hull_sanity, star_exits, star_figure, FieldEl, Gp, PairData,
    WindowProbe,
};
use crate::fields::K9;
use crate::medallion::{Absorption, Field, Medallion};
use std::cmp::Ordering;

// =====================================================================
// N = K₉(w) coordinates with an exact denominator: `(u + v*w) / den`,
// `den > 0`, gcd-reduced over all seven integers (canonical form, so
// derived equality is exact equality). Every comparison
// cross-multiplies; there is no division on any path. The Python size
// battery (scratch `monster-n9/size_out.txt`) walks every operation of
// [`verify_pair9`] with this exact integer scheme: worst canonical
// coefficient 31 bits, denominator 27 bits, cross-products 26 bits,
// convolution intermediates 32 bits — far inside the i64 operating
// range the `K9` sign engine documents.
// =====================================================================

/// `s = w² = 2 − b` (positive: `b ≈ 1.5321`).
const S_NQ: K9 = K9([2, -1, 0]);

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn gcd7(vals: [i64; 7]) -> i64 {
    vals.into_iter().fold(0, gcd).max(1)
}

/// An element `u + v*w` of `N = K₉(w)`, `w² = 2 − b = 4 sin²20°`
/// (`w > 0`), over a positive integer denominator, gcd-reduced.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Nq {
    pub u: K9,
    pub v: K9,
    pub den: i64,
}

impl Nq {
    /// `(u + v*w) / den`, gcd-reduced (`den > 0` required).
    pub fn new(u: K9, v: K9, den: i64) -> Nq {
        assert!(den > 0, "Nq denominator must be positive");
        let g = gcd7([u.0[0], u.0[1], u.0[2], v.0[0], v.0[1], v.0[2], den]);
        Nq {
            u: K9([u.0[0] / g, u.0[1] / g, u.0[2] / g]),
            v: K9([v.0[0] / g, v.0[1] / g, v.0[2] / g]),
            den: den / g,
        }
    }
    /// The integer `x` at denominator 1.
    pub const fn whole(x: i64) -> Nq {
        Nq {
            u: K9([x, 0, 0]),
            v: K9([0, 0, 0]),
            den: 1,
        }
    }
    /// The zero element.
    pub const ZERO: Nq = Nq::whole(0);
    /// The unit element.
    pub const ONE: Nq = Nq::whole(1);
    /// `-1`.
    pub const NEG_ONE: Nq = Nq::whole(-1);
    /// Exact sum (cross-multiplication).
    pub fn add(self, o: Nq) -> Nq {
        Nq::new(
            self.u.scale(o.den).add(o.u.scale(self.den)),
            self.v.scale(o.den).add(o.v.scale(self.den)),
            self.den * o.den,
        )
    }
    /// Exact difference.
    pub fn sub(self, o: Nq) -> Nq {
        Nq::new(
            self.u.scale(o.den).sub(o.u.scale(self.den)),
            self.v.scale(o.den).sub(o.v.scale(self.den)),
            self.den * o.den,
        )
    }
    /// Exact negation.
    pub fn neg(self) -> Nq {
        Nq {
            u: self.u.neg(),
            v: self.v.neg(),
            den: self.den,
        }
    }
    /// Field product: `(u1 + v1 w)(u2 + v2 w) = u1 u2 + s v1 v2 +
    /// (u1 v2 + u2 v1) w`.
    pub fn mul(self, o: Nq) -> Nq {
        Nq::new(
            self.u.mul(o.u).add(S_NQ.mul(self.v.mul(o.v))),
            self.u.mul(o.v).add(o.u.mul(self.v)),
            self.den * o.den,
        )
    }
    /// Exact sign. Same-sign cases are immediate; for mixed signs
    /// `x = u + v w` satisfies `x · (u − v w) = u² − s v²`, so
    /// `sgn x = sgn(u) · sgn(u² − s v²)` (both factors are exact K₉
    /// signs). `u² = s v²` for `(u, v) != 0` is impossible: it would
    /// make `s = 2 − b` a square in K₉, and `N(s) = 3`.
    pub fn sgn(self) -> Ordering {
        let su = self.u.sign();
        let sv = self.v.sign();
        match (su, sv) {
            (Ordering::Equal, o) => o,
            (o, Ordering::Equal) => o,
            (Ordering::Greater, Ordering::Greater) => Ordering::Greater,
            (Ordering::Less, Ordering::Less) => Ordering::Less,
            // Mixed signs: the conjugate factor `u − v w` has sign
            // `sgn(u)`, so sgn(x) = sgn(u) · sgn(u² − s v²) — the
            // product rule. `u² = s v²` is impossible for (u, v) ≠ 0
            // (it would make s a square in K₉ with N(s) = 3), so the
            // second factor is never Equal.
            (su, _) => {
                let product = self
                    .u
                    .mul(self.u)
                    .sub(S_NQ.mul(self.v.mul(self.v)))
                    .sign();
                debug_assert!(
                    product != Ordering::Equal,
                    "mixed-sign sgn: u² = s v² is impossible for (u, v) != 0"
                );
                if su == product {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        }
    }
    /// Exact equality across denominators (cross-multiplication).
    pub fn eq_x(self, o: Nq) -> bool {
        self.u.scale(o.den) == o.u.scale(self.den) && self.v.scale(o.den) == o.v.scale(self.den)
    }
    /// Float value — landed cross-check views only (`w = 2 sin 20°`,
    /// derived from the shipped `K9` float view, no decimal literal).
    pub fn fl(self) -> f64 {
        let w = (2.0 - K9::B.float()).sqrt();
        (self.u.float() + self.v.float() * w) / self.den as f64
    }
    /// Canonical-form check (emitted-numbers battery): positive
    /// denominator and gcd-reduced across all seven integers.
    pub fn is_canonical(self) -> bool {
        self.den > 0
            && gcd7([
                self.u.0[0],
                self.u.0[1],
                self.u.0[2],
                self.v.0[0],
                self.v.0[1],
                self.v.0[2],
                self.den,
            ]) == 1
    }
}

impl FieldEl for Nq {
    fn add(self, o: Self) -> Self {
        Nq::add(self, o)
    }
    fn sub(self, o: Self) -> Self {
        Nq::sub(self, o)
    }
    fn neg(self) -> Self {
        Nq::neg(self)
    }
    fn mul(self, o: Self) -> Self {
        Nq::mul(self, o)
    }
    fn sgn(self) -> Ordering {
        Nq::sgn(self)
    }
    fn eq_x(self, o: Self) -> bool {
        Nq::eq_x(self, o)
    }
    fn fl(self) -> f64 {
        Nq::fl(self)
    }
    fn zero_like(&self) -> Self {
        Nq::ZERO
    }
    fn int_like(&self, v: i64) -> Self {
        Nq::whole(v)
    }
    fn scaled_div(self, num: i64, den: i64) -> Self {
        assert!(den > 0, "Nq scaled_div denominator");
        Nq::new(self.u.scale(num), self.v.scale(num), self.den * den)
    }
}

// =====================================================================
// The exact nonagon hull and the rotation constants.
// =====================================================================

/// `cos 40° = b/2` (exact; `b = 2cos(2pi/9)`).
fn c40() -> Nq {
    Nq::new(K9::B, K9::ZERO, 2)
}
/// `sin 40° = (b² + b − 2)/2 · w` (`= a w / 2`, `a = 2cos20°`,
/// `a² = b + 2`).
fn s40() -> Nq {
    Nq::new(K9::ZERO, K9([-2, 1, 1]), 2)
}
/// `cos 20° = (b² + b − 2)/2`.
fn c20() -> Nq {
    Nq::new(K9([-2, 1, 1]), K9::ZERO, 2)
}
/// `sin 20° = w/2`.
fn s20() -> Nq {
    Nq::new(K9::ZERO, K9::ONE, 2)
}

/// Rotation by 40° about the origin (the hull walk step).
fn rot40(p: Gp<Nq>) -> Gp<Nq> {
    (
        c40().mul(p.0).sub(s40().mul(p.1)),
        s40().mul(p.0).add(c40().mul(p.1)),
    )
}

/// Rotation by `pi/n = 20°` about the origin — the pair rotation `R`.
fn rot20(p: Gp<Nq>) -> Gp<Nq> {
    (
        c20().mul(p.0).sub(s20().mul(p.1)),
        s20().mul(p.0).add(c20().mul(p.1)),
    )
}

/// The record hull: the unit-edge regular nonagon, normalized frame
/// (`V0 = origin`, edge 0 along +x), built by walking unit edge
/// directions `u_(k+1) = rot40(u_k)`. The closure lemma `rot40^9 = id`
/// and `Sum u_k = 0` and the unit-length lemma `|u_k|² = 1` are
/// asserted exactly (the Python twin is scratch `monster-n9/exact.py`).
pub fn hull() -> Vec<Gp<Nq>> {
    let mut pts = vec![(Nq::ZERO, Nq::ZERO)];
    let mut u: Gp<Nq> = (Nq::ONE, Nq::ZERO);
    let mut sum: Gp<Nq> = (Nq::ZERO, Nq::ZERO);
    for k in 0..9 {
        assert!(gdot(u, u).eq_x(Nq::ONE), "n9 hull: |u_{k}| != 1");
        sum = gadd(sum, u);
        pts.push(sum);
        u = rot40(u);
    }
    assert!(
        u.0.eq_x(Nq::ONE) && u.1.eq_x(Nq::ZERO),
        "n9 hull: rot40^9 != id"
    );
    assert!(
        sum.0.eq_x(Nq::ZERO) && sum.1.eq_x(Nq::ZERO),
        "n9 hull: walk does not close"
    );
    pts.pop();
    pts
}

/// `cos²(3·20°) = 1/4` — the (9,2) crossing and contact angle (60°).
pub(super) fn cos292() -> Nq {
    Nq::new(K9::ONE, K9::ZERO, 4)
}
/// `cos²(1·20°) = (2 + b)/4` — the (9,3) crossing and contact angle
/// (20°, i.e. `pi/n` — the recorded boundary of the M163 EXTENSION
/// guard; the standalone star emission path is unguarded, like the
/// `{12/6}` recorded degeneracy).
pub(super) fn cos293() -> Nq {
    Nq::new(K9([2, 1, 0]), K9::ZERO, 4)
}

/// The (9,2) pin `P = M₂ − R·M₀ = ((4+b)/4, (−6+3b+3b²)/4 · w)`
/// (`|P|² = (13+2b−2b²)/4`).
pub(super) fn pin92() -> Gp<Nq> {
    (
        Nq::new(K9([4, 1, 0]), K9::ZERO, 4),
        Nq::new(K9::ZERO, K9([-6, 3, 3]), 4),
    )
}
/// The (9,3) pin `P = M₁ − R·M₀ = ((6−b²)/4, (−3+b+b²)/4 · w)`
/// (`|P|² = (12−b−3b²)/4`).
pub(super) fn pin93() -> Gp<Nq> {
    (
        Nq::new(K9([6, 0, -1]), K9::ZERO, 4),
        Nq::new(K9::ZERO, K9([-3, 1, 1]), 4),
    )
}

/// The certified (9,2) window `(s_lo, s_hi)` — exact
/// vertex-on-edge-line events (scratch `window_out.txt`): below
/// `s_lo = (26−6b−2b²)/19` the absorbed count jumps to 3
/// (`eAB = [3,4,5]`, `eBA = [0,7,8]`, crossings relabel to
/// `(A2,B1)+(A6,B6)`); above `s_hi = (12+6b+2b²)/19` it drops to 1
/// (crossings `(A2,B8)+(A4,B6)`).
pub(super) fn win92() -> (Nq, Nq) {
    (
        Nq::new(K9([26, -6, -2]), K9::ZERO, 19),
        Nq::new(K9([12, 6, 2]), K9::ZERO, 19),
    )
}

/// The certified (9,3) window `(s_lo, s_hi)`: below
/// `s_lo = (4+2b−2b²)/3` the absorbed count drops to 2 and FOUR extra
/// transversals appear at the contacts (six crossings, scratch
/// `window_out.txt`); above the shared `s_hi = (12+6b+2b²)/19` it
/// drops to 2 with crossings `(A1,B8)+(A4,B5)`. Line-level collinear
/// events exist inside the window (they change no count — the state
/// census at σ = 0.8565/0.90/1.10 is identical to the pin's).
pub(super) fn win93() -> (Nq, Nq) {
    (
        Nq::new(K9([4, 2, -2]), K9::ZERO, 3),
        Nq::new(K9([12, 6, 2]), K9::ZERO, 19),
    )
}

// =====================================================================
// The rotated placement record: spec + verifier.
// =====================================================================

/// Catalog entry input for a rotated pair placement.
struct PairSpec9 {
    /// Label for assertion messages.
    name: &'static str,
    /// Exact displacement: `B = R·A + P` (`R = rot20`).
    p: Gp<Nq>,
    /// Star figure step: the contact angle is `d * pi / n`.
    d: usize,
    /// Certified `A`-edges strictly inside `B` (a consecutive run).
    e_ab: Vec<usize>,
    /// Certified `B`-edges strictly inside `A` (`= {i + 4}`: the
    /// half-turn image under `kappa`, not the parallel `{i + n/2}`).
    e_ba: Vec<usize>,
    /// Certified crossing edge pairs `(A_i, B_j)`, sorted by `A`-edge:
    /// `(eab[0], eb)` and `(eab[c-1] + 1, eb')`.
    cross: [(usize, usize); 2],
    /// Exact `cos²(d pi / n)`.
    cos2: Nq,
    /// Certified window ends along the ray `sigma * P` (pin `sigma = 1`).
    s_lo: Nq,
    s_hi: Nq,
    /// Direction in which the primary crossing's `t(sigma)` moves.
    t_increasing: bool,
    /// Certified union outline (cyclic order, from module docs).
    union_cycle: Vec<super::OutNode>,
    /// Certified lens outline (cyclic order).
    lens_cycle: Vec<super::OutNode>,
}

/// Build the two outline cycles of a rotated placement (module docs,
/// "Outline structure"): `X0` on `A`-edge `ea0` and `B`-edge `eb0`,
/// `X1` on `A`-edge `ea1` and `B`-edge `eb1` (the absorbed `A`-run is
/// `ea0 .. ea1`, the absorbed `B`-run `(eb1+1) .. eb0`, `c` edges
/// each). Unlike the parallel records, the two polygons' index frames
/// are shifted by the rotation: the union carries, after `X0`, the
/// `B`-arc `V_(eb0+1) .. V_(eb1)` and after `X1` the `A`-arc
/// `V_(ea1+1) .. V_(ea0)` (the lens the complementary arcs) — see the
/// exact battery (scratch `walk_out.txt`). The sizes are exactly the
/// catalog formulas `2(n - c - 1) + 2` (union) and `2c + 4` (lens):
/// every cycle has `n - c - 1` vertices per side (the exposed ones)
/// and the lens `c + 1` hidden vertices per side. The hub's
/// [`super::outlines`] is rederived identically and cross-checked
/// against these lists inside [`verify_pair9`].
fn rotated_cycles(
    n: usize,
    ea0: usize,
    ea1: usize,
    eb0: usize,
    eb1: usize,
) -> (Vec<super::OutNode>, Vec<super::OutNode>) {
    use super::OutNode;
    let run = |s: usize, e: usize| -> Vec<usize> {
        let mut v = Vec::new();
        let mut k = s;
        loop {
            v.push(k);
            if k == e {
                return v;
            }
            k = (k + 1) % n;
        }
    };
    let union_cycle = {
        let mut v = vec![OutNode::X(0)];
        v.extend(run((eb0 + 1) % n, eb1).into_iter().map(OutNode::B));
        v.push(OutNode::X(1));
        v.extend(run((ea1 + 1) % n, ea0).into_iter().map(OutNode::A));
        v
    };
    let lens_cycle = {
        let mut v = vec![OutNode::X(0)];
        v.extend(run((ea0 + 1) % n, ea1).into_iter().map(OutNode::A));
        v.push(OutNode::X(1));
        v.extend(run((eb1 + 1) % n, eb0).into_iter().map(OutNode::B));
        v
    };
    (union_cycle, lens_cycle)
}

/// The full exact verification of one rotated monster placement — the
/// odd-n analog of the hub's [`super::verify_pair`]: hull schema and
/// closure lemma, the half-turn swap `kappa` (vertex maps both
/// directions, midpoint map, and the crossing swap
/// `(A_i, B_j) -> (A_(j+5), B_(i+4))`), general position, the
/// definition counts with the `+4` pairing, absorbed contacts, exactly
/// two transversal crossings at the edge-midpoint pin (`P = M_i −
/// R·M_j`), the angle identity, the rotated figure-step rule
/// `d ≡ 2(i−j) − 1`, the exact outline cycles with every piece
/// re-certified by a midpoint half-plane test, the window samples
/// `sigma = 1 -+ 1/64` (lists and crossings kept, `t != 1/2`, strict
/// monotonicity through `1/2` with the exact sample values `63/128`
/// and `65/128`) and the tabulated out-of-window probes.
fn verify_pair9(spec: &PairSpec9, probes: &[WindowProbe<Nq>]) -> PairData<Nq> {
    let a = hull();
    let n = a.len();
    hull_sanity(spec.name, &a);

    let p = spec.p;
    let b: Vec<Gp<Nq>> = a.iter().map(|&q| gadd(rot20(q), p)).collect();
    let m = gmidpoints(&a);

    // Half-turn swap lemma (module docs): with cB = R*c + P and
    // q = c + cB, kappa(X) = q - X maps V_k^A -> V_(k+4)^B,
    // M_k^A -> M_(k+4)^B and V_k^B -> V_(k+5)^A.
    let c = gcentre(&a);
    let cb = gadd(rot20(c), p);
    let q = gadd(c, cb);
    for k in 0..n {
        let ka = gsub(q, a[k]);
        assert!(
            ka.0.eq_x(b[(k + 4) % n].0) && ka.1.eq_x(b[(k + 4) % n].1),
            "{}: kappa(V_{k}^A) != V_(k+4)^B",
            spec.name
        );
        let km = gsub(q, m[k]);
        let mb = gadd(rot20(m[(k + 4) % n]), p);
        assert!(
            km.0.eq_x(mb.0) && km.1.eq_x(mb.1),
            "{}: kappa(M_{k}^A) != M_(k+4)^B",
            spec.name
        );
        let kb = gsub(q, b[k]);
        assert!(
            kb.0.eq_x(a[(k + 5) % n].0) && kb.1.eq_x(a[(k + 5) % n].1),
            "{}: kappa(V_{k}^B) != V_(k+5)^A",
            spec.name
        );
    }

    assert_general_position(&a, &b, spec.name);

    // Definition: c full edges strictly inside, both ways, certified
    // lists (consecutive runs), and eBA = {i + 4} (the kappa image).
    let e_ab = gedges_inside(&a, &b);
    let e_ba = gedges_inside(&b, &a);
    assert_eq!(e_ab, spec.e_ab, "{}: eAB", spec.name);
    assert_eq!(e_ba, spec.e_ba, "{}: eBA", spec.name);
    let cabs = e_ab.len();
    assert!(
        cabs >= 2 && cabs <= (n + 1) / 2 - 2,
        "{}: c outside legal band",
        spec.name
    );
    let mut want: Vec<usize> = e_ab.iter().map(|&i| (i + 4) % n).collect();
    want.sort_unstable();
    assert_eq!(e_ba, want, "{}: eBA != kappa image +4", spec.name);

    // Absorbed contacts (interior edge midpoints) = interior edges.
    let absorbed_a: Vec<usize> = (0..n).filter(|&i| ginside(m[i], &b)).collect();
    assert_eq!(absorbed_a, e_ab, "{}: absorbed contacts A-in-B", spec.name);
    let absorbed_b: Vec<usize> = (0..n)
        .filter(|&i| ginside(gadd(rot20(m[i]), p), &a))
        .collect();
    assert_eq!(absorbed_b, e_ba, "{}: absorbed contacts B-in-A", spec.name);

    // Exactly two transversal crossings, at edge midpoints, on the
    // certified edge pairs, with P = M_i − R·M_j (the pinning
    // identity) and the angle identity cos^2 = cos^2(d pi / n).
    let cr = gcrossings(&a, &b);
    assert_eq!(cr.len(), 2, "{}: not exactly 2 crossings", spec.name);
    let two = Nq::whole(2);
    for (k, ci) in cr.iter().enumerate() {
        assert_eq!(ci.ea, spec.cross[k].0, "{}: crossing {k} A-edge", spec.name);
        assert_eq!(ci.eb, spec.cross[k].1, "{}: crossing {k} B-edge", spec.name);
        assert!(
            two.mul(ci.t_num).eq_x(ci.den) && two.mul(ci.s_num).eq_x(ci.den),
            "{}: crossing {k} not at the edge midpoints",
            spec.name
        );
        let ma = m[ci.ea];
        let mb = gadd(rot20(m[ci.eb]), p);
        assert!(
            hom_eq(&ci.x, &hom_of(ma)) && hom_eq(&ci.x, &hom_of(mb)),
            "{}: crossing {k} off the midpoint",
            spec.name
        );
        let pin = gsub(ma, rot20(m[ci.eb]));
        assert!(
            p.0.sub(pin.0).sgn() == Ordering::Equal && p.1.sub(pin.1).sgn() == Ordering::Equal,
            "{}: P != M_{} - R*M_{}",
            spec.name,
            ci.ea,
            ci.eb
        );
        // Crossing angle: edges are unit, so cos^2 = dot^2. Rotated
        // pair: the A-edge direction is 2i pi/n and the B-edge
        // direction 2j pi/n + pi/n, so the figure step is 2(i-j) - 1.
        let u = gsub(a[(ci.ea + 1) % n], a[ci.ea]);
        let v = gsub(b[(ci.eb + 1) % n], b[ci.eb]);
        let lhs = gdot(u, v).mul(gdot(u, v));
        assert!(
            lhs.eq_x(spec.cos2),
            "{}: crossing cos^2 != cos^2(d pi/n)",
            spec.name
        );
        let dif = (2 * (ci.ea as i64 - ci.eb as i64) - 1).rem_euclid(n as i64);
        let dcan = dif.min(n as i64 - dif);
        let scan = (spec.d as i64).min(n as i64 - spec.d as i64);
        assert_eq!(dcan, scan, "{}: d != 2 (i - j) - 1 mod n", spec.name);
    }
    // The half-turn maps crossings (A_i, B_j) -> (A_(j+5), B_(i+4))
    // and swaps the two crossing POINTS.
    assert_eq!(cr[1].ea, (cr[0].eb + 5) % n, "{}: crossing swap image", spec.name);
    assert_eq!(cr[1].eb, (cr[0].ea + 4) % n, "{}: crossing swap image", spec.name);
    let x0 = m[cr[0].ea];
    let x1 = m[cr[1].ea];
    let kx0 = gsub(q, x0);
    assert!(
        kx0.0.eq_x(x1.0) && kx0.1.eq_x(x1.1),
        "{}: kappa does not swap the crossings",
        spec.name
    );

    // Vertex classification. Hidden = strictly inside the other
    // polygon: the A-hidden run is `ea0 + 1 .. ea1` (the absorbed
    // run's end vertices), the B-hidden run is `eb1 + 1 .. eb0`
    // (module docs, outline structure).
    let hidden_a: Vec<usize> = (0..n).filter(|&k| ginside(a[k], &b)).collect();
    let hidden_b: Vec<usize> = (0..n).filter(|&k| ginside(b[k], &a)).collect();
    assert_eq!(
        hidden_a,
        arc((cr[0].ea + 1) % n, cr[1].ea, n),
        "{}: hidden A run",
        spec.name
    );
    assert_eq!(
        hidden_b,
        arc((cr[1].eb + 1) % n, cr[0].eb, n),
        "{}: hidden B run",
        spec.name
    );

    // Outlines from the certified crossing structure, then
    // re-certify: every consecutive pair of one cycle is either two
    // adjacent vertices of one polygon (step +1) or a vertex and a
    // crossing whose shared edge carries the crossing at its midpoint
    // (the certified `t = s = 1 / 2`); the midpoint of every piece
    // (for half edges, of the vertex-to-crossing half segment) lies
    // strictly on the right side — outside the other polygon for the
    // union, strictly inside for the lens. At each crossing the two
    // cycle neighbours of a side are exactly the two endpoints of
    // that side's crossing edge. The vertex sets of the two cycles
    // partition all `n` vertices per side; sizes are the catalog
    // formulas `2(n - c - 1) + 2` (union) and `2c + 4` (lens).
    let (union_cycle, lens_cycle) = rotated_cycles(n, cr[0].ea, cr[1].ea, cr[0].eb, cr[1].eb);
    assert_eq!(union_cycle, spec.union_cycle, "{}: union cycle", spec.name);
    assert_eq!(lens_cycle, spec.lens_cycle, "{}: lens cycle", spec.name);
    // The hub's arc-uniformity model holds for the rotated records as
    // well and rederives the same cycles: the arcs through the
    // crossing edges pair exposed-with-exposed exactly as the walk
    // shows. Cross-check the certified lists against it.
    let ea = (cr[0].ea, cr[1].ea);
    let eb = (cr[0].eb, cr[1].eb);
    let exa: Vec<bool> = (0..n).map(|k| !ginside(a[k], &b)).collect();
    let exb: Vec<bool> = (0..n).map(|k| !ginside(b[k], &a)).collect();
    let (hu, hl) = super::outlines(n, &exa, &exb, ea, eb);
    assert_eq!(hu, union_cycle, "{}: hub union rederivation", spec.name);
    assert_eq!(hl, lens_cycle, "{}: hub lens rederivation", spec.name);
    let mid = |u: Gp<Nq>, v: Gp<Nq>| {
        let mm = gmidpoints(&[u, v]);
        mm[0]
    };
    let certify = |cyc: &[super::OutNode], want_inside: bool, tag: &str| {
        let len = cyc.len();
        assert_eq!(
            cyc[0],
            super::OutNode::X(0),
            "{}: {tag} cycle must start at X0",
            spec.name
        );
        for i in 0..len {
            let (here, next) = (cyc[i], cyc[(i + 1) % len]);
            match (here, next) {
                (super::OutNode::A(u), super::OutNode::A(v)) => {
                    assert_eq!((v + n - u) % n, 1, "{}: {tag} A step", spec.name);
                    assert_eq!(
                        ginside(mid(a[u], a[v]), &b),
                        want_inside,
                        "{}: {tag} A piece {u}-{v}",
                        spec.name
                    );
                }
                (super::OutNode::B(u), super::OutNode::B(v)) => {
                    assert_eq!((v + n - u) % n, 1, "{}: {tag} B step", spec.name);
                    assert_eq!(
                        ginside(mid(b[u], b[v]), &a),
                        want_inside,
                        "{}: {tag} B piece {u}-{v}",
                        spec.name
                    );
                }
                // Piece INTO a crossing: half of this side's crossing
                // edge, from its START vertex to the midpoint.
                (super::OutNode::A(u), super::OutNode::X(k)) => {
                    let e = cr[k].ea;
                    assert_eq!(u, e, "{}: {tag} into X{k} adjacency", spec.name);
                    assert_eq!(
                        ginside(mid(a[e], mid(a[e], a[(e + 1) % n])), &b),
                        want_inside,
                        "{}: {tag} half-edge into X{k}",
                        spec.name
                    );
                }
                (super::OutNode::B(u), super::OutNode::X(k)) => {
                    let e = cr[k].eb;
                    assert_eq!(u, e, "{}: {tag} into X{k} adjacency", spec.name);
                    assert_eq!(
                        ginside(mid(b[e], mid(b[e], b[(e + 1) % n])), &a),
                        want_inside,
                        "{}: {tag} half-edge into X{k}",
                        spec.name
                    );
                }
                // Piece OUT OF a crossing: the far half of the same
                // edge, from the midpoint to its END vertex.
                (super::OutNode::X(k), super::OutNode::A(v)) => {
                    let e = cr[k].ea;
                    assert_eq!(v, (e + 1) % n, "{}: {tag} out of X{k} adjacency", spec.name);
                    assert_eq!(
                        ginside(mid(mid(a[e], a[(e + 1) % n]), a[(e + 1) % n]), &b),
                        want_inside,
                        "{}: {tag} half-edge out of X{k}",
                        spec.name
                    );
                }
                (super::OutNode::X(k), super::OutNode::B(v)) => {
                    let e = cr[k].eb;
                    assert_eq!(v, (e + 1) % n, "{}: {tag} out of X{k} adjacency", spec.name);
                    assert_eq!(
                        ginside(mid(mid(b[e], b[(e + 1) % n]), b[(e + 1) % n]), &a),
                        want_inside,
                        "{}: {tag} half-edge out of X{k}",
                        spec.name
                    );
                }
                _ => panic!("{}: {tag} outline side mismatch at {i}", spec.name),
            }
        }
    };
    certify(&union_cycle, false, "union");
    certify(&lens_cycle, true, "lens");
    assert_eq!(
        union_cycle.len(),
        2 * (n - cabs - 1) + 2,
        "{}: union size",
        spec.name
    );
    assert_eq!(lens_cycle.len(), 2 * cabs + 4, "{}: lens size", spec.name);
    for cyc in [&union_cycle, &lens_cycle] {
        assert_eq!(
            cyc.iter().filter(|t| matches!(**t, super::OutNode::X(_))).count(),
            2,
            "{}: outline crossing nodes",
            spec.name
        );
    }
    let verts = |cyc: &[super::OutNode], side_a: bool| -> Vec<usize> {
        let mut v: Vec<usize> = cyc
            .iter()
            .filter_map(|t| match (t, side_a) {
                (super::OutNode::A(k), true) => Some(*k),
                (super::OutNode::B(k), false) => Some(*k),
                _ => None,
            })
            .collect();
        v.sort_unstable();
        v
    };
    for side_a in [true, false] {
        let mut all = verts(&union_cycle, side_a);
        all.extend(verts(&lens_cycle, side_a));
        all.sort_unstable();
        assert_eq!(all, (0..n).collect::<Vec<usize>>(), "{}: partition", spec.name);
    }
    // Union-side vertex sets: strictly outside the other polygon,
    // `n - c - 1` per side (the parallel-record counts). The rotated
    // twist is WHICH vertices: on `A` the union carries
    // `ea1 + 1 .. ea0` (the far run-end `V_(ea1+1)` is exposed and
    // walks the union, while its edge's inner half closes the lens
    // beyond the crossing) and symmetrically on `B`.
    let exposed_a: Vec<usize> = (0..n).filter(|&k| !ginside(a[k], &b)).collect();
    let exposed_b: Vec<usize> = (0..n).filter(|&k| !ginside(b[k], &a)).collect();
    assert_eq!(exposed_a.len(), n - cabs - 1, "{}: exposed A count", spec.name);
    assert_eq!(exposed_b.len(), n - cabs - 1, "{}: exposed B count", spec.name);

    // Window: the pin `sigma = 1` is strictly interior (`s_lo < 1 <
    // s_hi`); the samples `sigma = 1 -+ 1/64` stay general position,
    // keep both certified edge lists and both crossing pairs, leave
    // the pin, and the primary crossing's parameter is strictly
    // monotone through 1/2 — exactly `t = 63/128` / `65/128` (the
    // scratch census `verify2.py` pins these values).
    assert_eq!(
        spec.s_lo.sub(Nq::ONE).sgn(),
        Ordering::Less,
        "{}: s_lo >= 1",
        spec.name
    );
    assert_eq!(
        spec.s_hi.sub(Nq::ONE).sgn(),
        Ordering::Greater,
        "{}: s_hi <= 1",
        spec.name
    );
    for sgnk in [-1i64, 1] {
        let sig = Nq::ONE.scaled_div(64 + sgnk, 64);
        let p2 = (p.0.mul(sig), p.1.mul(sig));
        let b2: Vec<Gp<Nq>> = a.iter().map(|&q| gadd(rot20(q), p2)).collect();
        assert_general_position(&a, &b2, spec.name);
        let ca = gedges_inside(&a, &b2);
        let cb2 = gedges_inside(&b2, &a);
        assert_eq!(ca, spec.e_ab, "{}: window sample eAB", spec.name);
        assert_eq!(cb2, spec.e_ba, "{}: window sample eBA", spec.name);
        let cr2 = gcrossings(&a, &b2);
        assert_eq!(cr2.len(), 2, "{}: window sample crossings", spec.name);
        for (k, ci) in cr2.iter().enumerate() {
            assert_eq!(ci.ea, spec.cross[k].0, "{}: window sample crossing {k}", spec.name);
            assert_eq!(ci.eb, spec.cross[k].1, "{}: window sample crossing {k}", spec.name);
        }
        let ci = &cr2[0];
        let diff = gmul_i(ci.t_num, 2).sub(ci.den);
        let s = diff.sgn();
        assert_ne!(s, Ordering::Equal, "{}: window sample still at the pin", spec.name);
        // t > 1/2 iff 2 t_num - den has the sign of den (t = t_num/den).
        let above = if ci.den.sgn() == Ordering::Greater {
            s
        } else {
            s.reverse()
        };
        let expect = if (sgnk == -1) == spec.t_increasing {
            Ordering::Less
        } else {
            Ordering::Greater
        };
        assert_eq!(
            above, expect,
            "{}: t(sigma) not strictly monotone through the pin",
            spec.name
        );
        // The exact sample values: t = sigma/2 when increasing and
        // (2 - sigma)/2 when decreasing — 63/128 and 65/128.
        let want_num = if (sgnk == -1) == spec.t_increasing { 63 } else { 65 };
        let lhs = gmul_i(ci.t_num, 128);
        let rhs = gmul_i(ci.den, want_num);
        assert!(lhs.eq_x(rhs), "{}: window sample t != {want_num}/128", spec.name);
    }

    // Tabulated out-of-window probes (counts change as documented).
    for pr in probes {
        let w = (pr.rho.mul(p.0), pr.rho.mul(p.1));
        let b3: Vec<Gp<Nq>> = a.iter().map(|&q| gadd(rot20(q), w)).collect();
        let ca = gedges_inside(&a, &b3).len();
        let cb3 = gedges_inside(&b3, &a).len();
        assert_eq!(
            (ca, cb3),
            (pr.ca, pr.cb),
            "{}: window probe at sigma {:?}",
            spec.name,
            pr.rho.fl()
        );
        if let Some(k) = pr.crossings {
            assert_eq!(
                gcrossings(&a, &b3).len(),
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

/// Ascending-cyclic vertex list `s, s+1, ..., e` (mod n).
fn arc(s: usize, e: usize, n: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut k = s;
    loop {
        v.push(k);
        if k == e {
            v.sort_unstable();
            return v;
        }
        k = (k + 1) % n;
    }
}

// =====================================================================
// The two records and their certified placements.
// =====================================================================

use super::OutNode;

/// The `{9/3}` record: two tricigrams on the nonagon, contact angle
/// `3*pi/9 = 60` degrees (`cos^2 = 1/4`).
pub struct Monster93;

impl Medallion for Monster93 {
    const ID: &'static str = "nonagon {9/3} monster star (24c)";
    const FIELD: Field = Field::K9;
    const SYM: usize = 9;
    const PATH: Absorption = Absorption::BoundaryInference;

    fn hull_f64() -> Vec<[f64; 2]> {
        hull().iter().map(|q| [q.0.fl(), q.1.fl()]).collect()
    }
    fn exit_rays_f64() -> Vec<(usize, [f64; 2])> {
        exit_rays3()
            .into_iter()
            .map(|(i, d)| (i, [d.0.fl(), d.1.fl()]))
            .collect()
    }
    fn contact_cos2() -> Option<(i64, i64, i64, i64)> {
        Some((1, 0, 0, 4)) // cos^2 60 degrees is rational
    }
}

/// The `{9/1}` record: the midpoint nonagon on the nonagon, contact
/// angle `pi/9 = 20` degrees (`cos^2 = (2+b)/4` — cubic, pinned in the
/// battery; the trait tuple cannot encode it, the dec10/L10 precedent).
/// The step `d = 1` figure is a new catalog degeneracy (all landed
/// records have `d >= 2`), recorded like the `{12/6}` note; `theta =
/// pi/n` sits exactly on the M163 EXTENSION guard boundary, like
/// (12,2) at `pi/2` — the standalone star emission path is unguarded.
pub struct Monster91;

impl Medallion for Monster91 {
    const ID: &'static str = "nonagon {9/1} monster star (24c)";
    const FIELD: Field = Field::K9;
    const SYM: usize = 9;
    const PATH: Absorption = Absorption::BoundaryInference;

    fn hull_f64() -> Vec<[f64; 2]> {
        hull().iter().map(|q| [q.0.fl(), q.1.fl()]).collect()
    }
    fn exit_rays_f64() -> Vec<(usize, [f64; 2])> {
        exit_rays1()
            .into_iter()
            .map(|(i, d)| (i, [d.0.fl(), d.1.fl()]))
            .collect()
    }
    // cos^2 = (2+b)/4 is cubic: None (dec10/L10 precedent), pinned
    // exactly as cos293 in the batteries below.
}

/// The `{9/3}` figure as contact-index pairs.
pub fn figure3() -> Vec<(usize, usize)> {
    star_figure(9, 3)
}

/// The `{9/1}` figure as contact-index pairs.
pub fn figure1() -> Vec<(usize, usize)> {
    star_figure(9, 1)
}

/// The 18 directed exit rays of the `{9/3}` record.
pub fn exit_rays3() -> Vec<(usize, Gp<Nq>)> {
    star_exits(&gmidpoints(&hull()), 3)
}

/// The 18 directed exit rays of the `{9/1}` record.
pub fn exit_rays1() -> Vec<(usize, Gp<Nq>)> {
    star_exits(&gmidpoints(&hull()), 1)
}

/// The certified (9,2) placement: `eAB = [3,4]`, `eBA = [7,8]`,
/// crossings `(A2,B0)` and `(A5,B6)` at the edge midpoints, `P = M2 -
/// R*M0`, window `((26-6b-2b^2)/19, (12+6b+2b^2)/19)` with pin
/// `sigma = 1`. Union 14 nodes, lens 8.
pub fn pair92() -> PairData<Nq> {
    let (lo, hi) = win92();
    verify_pair9(
        &PairSpec9 {
            name: "(9,2)",
            p: pin92(),
            d: 3,
            e_ab: vec![3, 4],
            e_ba: vec![7, 8],
            cross: [(2, 0), (5, 6)],
            cos2: cos292(),
            s_lo: lo,
            s_hi: hi,
            t_increasing: true,
            union_cycle: vec![
                OutNode::X(0),
                OutNode::B(1),
                OutNode::B(2),
                OutNode::B(3),
                OutNode::B(4),
                OutNode::B(5),
                OutNode::B(6),
                OutNode::X(1),
                OutNode::A(6),
                OutNode::A(7),
                OutNode::A(8),
                OutNode::A(0),
                OutNode::A(1),
                OutNode::A(2),
            ],
            lens_cycle: vec![
                OutNode::X(0),
                OutNode::A(3),
                OutNode::A(4),
                OutNode::A(5),
                OutNode::X(1),
                OutNode::B(7),
                OutNode::B(8),
                OutNode::B(0),
            ],
        },
        &[
            // Above the window: only edge 3 (resp. 7) stays absorbed,
            // crossings relabel to (A2,B8) + (A4,B6).
            WindowProbe {
                rho: hi.add(Nq::ONE.scaled_div(1, 64)),
                ca: 1,
                cb: 1,
                crossings: Some(2),
            },
            // Below the window: the count jumps to 3
            // (eAB = [3,4,5], eBA = [0,7,8]), still two crossings,
            // relabeled (A2,B1) + (A6,B6).
            WindowProbe {
                rho: lo.sub(Nq::ONE.scaled_div(1, 64)),
                ca: 3,
                cb: 3,
                crossings: Some(2),
            },
        ],
    )
}

/// The certified (9,3) placement: `eAB = [2,3,4]`, `eBA = [6,7,8]`,
/// crossings `(A1,B0)` and `(A5,B5)` at the edge midpoints, `P = M1 -
/// R*M0`, window `((4+2b-2b^2)/3, (12+6b+2b^2)/19)` with pin
/// `sigma = 1`. Union 12 nodes, lens 10.
pub fn pair93() -> PairData<Nq> {
    let (lo, hi) = win93();
    verify_pair9(
        &PairSpec9 {
            name: "(9,3)",
            p: pin93(),
            d: 1,
            e_ab: vec![2, 3, 4],
            e_ba: vec![6, 7, 8],
            cross: [(1, 0), (5, 5)],
            cos2: cos293(),
            s_lo: lo,
            s_hi: hi,
            t_increasing: false,
            union_cycle: vec![
                OutNode::X(0),
                OutNode::B(1),
                OutNode::B(2),
                OutNode::B(3),
                OutNode::B(4),
                OutNode::B(5),
                OutNode::X(1),
                OutNode::A(6),
                OutNode::A(7),
                OutNode::A(8),
                OutNode::A(0),
                OutNode::A(1),
            ],
            lens_cycle: vec![
                OutNode::X(0),
                OutNode::A(2),
                OutNode::A(3),
                OutNode::A(4),
                OutNode::A(5),
                OutNode::X(1),
                OutNode::B(6),
                OutNode::B(7),
                OutNode::B(8),
                OutNode::B(0),
            ],
        },
        &[
            // Above the window: edge 4 (resp. 8) drops out, crossings
            // relabel to (A1,B8) + (A4,B5).
            WindowProbe {
                rho: hi.add(Nq::ONE.scaled_div(1, 64)),
                ca: 2,
                cb: 2,
                crossings: Some(2),
            },
            // Below the window: the absorbed run shifts (eAB = [3,4],
            // eBA = [7,8]) and FOUR extra transversals appear at the
            // contacts (six crossings). Exact battery
            // `winprobe_exact.txt`.
            WindowProbe {
                rho: lo.sub(Nq::ONE.scaled_div(1, 64)),
                ca: 2,
                cb: 2,
                crossings: Some(6),
            },
            // A rational probe deep below the window (sigma = 7/10)
            // shows the same state.
            WindowProbe {
                rho: Nq::ONE.scaled_div(7, 10),
                ca: 2,
                cb: 2,
                crossings: Some(6),
            },
        ],
    )
}

// =====================================================================
// Tests.
// =====================================================================

#[cfg(test)]
mod tests {
    use super::super::battery_record;
    use super::*;

    #[test]
    fn nonagon_hull_closes_and_is_normalized() {
        let h = hull();
        assert_eq!(h.len(), 9);
        hull_sanity("n9 hull", &h);
    }

    #[test]
    fn nq_field_axioms_on_certified_elements() {
        // Associativity/distributivity/commutativity spot checks via
        // exact equality, plus canonical form and sign consistency.
        let xs = [
            Nq::ONE,
            pin92().0,
            pin92().1,
            pin93().0,
            pin93().1,
            cos292(),
            cos293(),
            win92().0,
            win92().1,
            win93().0,
            c20().add(s20()),
            c40().mul(s40()),
        ];
        for &x in &xs {
            assert!(x.is_canonical());
            assert!(x.add(Nq::ZERO).eq_x(x));
            assert!(x.mul(Nq::ONE).eq_x(x));
            assert!(x.sub(x).sgn() == Ordering::Equal);
            assert!(x.mul(Nq::NEG_ONE).neg().eq_x(x));
            assert_eq!(x.sgn(), x.neg().sgn().reverse());
        }
        for &x in &xs {
            for &y in &xs {
                assert!(x.add(y).eq_x(y.add(x)));
                assert!(x.mul(y).eq_x(y.mul(x)));
                assert!(x.mul(y).is_canonical());
                // (x + y)^2 = x^2 + 2xy + y^2
                let l = x.add(y).mul(x.add(y));
                let r = x.mul(x).add(gmul_i(x, 2).mul(y)).add(y.mul(y));
                assert!(l.eq_x(r));
                // sign of a product
                let s = x.mul(y).sgn();
                let expect = match (x.sgn(), y.sgn()) {
                    (Ordering::Equal, _) | (_, Ordering::Equal) => Ordering::Equal,
                    (a, b) if a == b => Ordering::Greater,
                    _ => Ordering::Less,
                };
                assert_eq!(s, expect);
            }
        }
    }

    #[test]
    fn both_figures_pass_the_record_battery() {
        // The {9/3} record of (9,2) and the {9/1} record of (9,3).
        battery_record("<{9/3} record>", &hull(), 3, cos292());
        battery_record("<{9/1} record>", &hull(), 1, cos293());
        assert_eq!(<Monster93 as Medallion>::SYM, 9);
        assert_eq!(<Monster93 as Medallion>::FIELD, Field::K9);
        assert_eq!(<Monster93 as Medallion>::contact_cos2(), Some((1, 0, 0, 4)));
        assert_eq!(<Monster91 as Medallion>::SYM, 9);
        assert_eq!(<Monster91 as Medallion>::FIELD, Field::K9);
        assert_eq!(<Monster91 as Medallion>::contact_cos2(), None);
        assert_eq!(figure3().len(), 9);
        assert_eq!(figure1().len(), 9);
        assert_eq!(exit_rays3().len(), 18);
        assert_eq!(exit_rays1().len(), 18);
    }

    #[test]
    fn placement92_is_the_certified_pin() {
        let d = pair92();
        assert!(gp_eq(&d.p, &pin92()));
        assert_eq!(d.e_ab, vec![3, 4]);
        assert_eq!(d.e_ba, vec![7, 8]);
        // Strictly-outside vertices: `n - c - 1 = 6` per side; the
        // run-end transition vertices V2/V6 (A) and B0/B6 (B) are
        // exposed even though their edges' inner halves close the
        // lens (module docs, outline structure).
        assert_eq!(d.exposed_a, vec![0, 1, 2, 6, 7, 8]);
        assert_eq!(d.exposed_b, vec![1, 2, 3, 4, 5, 6]);
        let h = hull();
        let m = gmidpoints(&h);
        assert!(gp_eq(&d.cross_pts[0], &m[2]));
        assert!(gp_eq(&d.cross_pts[1], &m[5]));
        let p_of = |i: usize, j: usize| gsub(m[i], rot20(m[j]));
        assert!(gp_eq(&p_of(2, 0), &d.p));
        assert!(gp_eq(&p_of(5, 6), &d.p));
        // Union 2(9-2-1)+2 = 14 nodes; lens 2*2+4 = 8.
        assert_eq!(d.union_cycle.len(), 14);
        assert_eq!(d.lens_cycle.len(), 8);
    }

    #[test]
    fn placement93_is_the_certified_pin() {
        let d = pair93();
        assert!(gp_eq(&d.p, &pin93()));
        assert_eq!(d.e_ab, vec![2, 3, 4]);
        assert_eq!(d.e_ba, vec![6, 7, 8]);
        assert_eq!(d.exposed_a, vec![0, 1, 6, 7, 8]);
        assert_eq!(d.exposed_b, vec![1, 2, 3, 4, 5]);
        let h = hull();
        let m = gmidpoints(&h);
        assert!(gp_eq(&d.cross_pts[0], &m[1]));
        assert!(gp_eq(&d.cross_pts[1], &m[5]));
        let p_of = |i: usize, j: usize| gsub(m[i], rot20(m[j]));
        assert!(gp_eq(&p_of(1, 0), &d.p));
        assert!(gp_eq(&p_of(5, 5), &d.p));
        // Union 2(9-3-1)+2 = 12 nodes; lens 2*3+4 = 10.
        assert_eq!(d.union_cycle.len(), 12);
        assert_eq!(d.lens_cycle.len(), 10);
    }

    #[test]
    fn outline_cycles_close_in_float() {
        // The float reading of the exact cycles: consecutive nodes
        // distinct, nonzero enclosed area (the cycles themselves are
        // certified exactly in verify_pair9).
        for (d, xs) in [(pair92(), [2usize, 5]), (pair93(), [1, 5])] {
            let h = hull();
            let m = gmidpoints(&h);
            let pos = |t: &OutNode| -> (f64, f64) {
                match *t {
                    OutNode::A(k) => (h[k].0.fl(), h[k].1.fl()),
                    OutNode::B(k) => {
                        let q = gadd(rot20(h[k]), d.p);
                        (q.0.fl(), q.1.fl())
                    }
                    OutNode::X(k) => (m[xs[k]].0.fl(), m[xs[k]].1.fl()),
                }
            };
            for cyc in [&d.union_cycle, &d.lens_cycle] {
                let pts: Vec<(f64, f64)> = cyc.iter().map(pos).collect();
                for w in pts.windows(2) {
                    let dx = w[1].0 - w[0].0;
                    let dy = w[1].1 - w[0].1;
                    assert!(dx * dx + dy * dy > 1e-9, "outline has a degenerate edge");
                }
                let dd = pts.len();
                let mut area: f64 = 0.0;
                for i in 0..dd {
                    let (p1, p2) = (pts[i], pts[(i + 1) % dd]);
                    area += p1.0 * p2.1 - p1.1 * p2.0;
                }
                assert!(area.abs() > 1.0, "outline area degenerates");
            }
        }
    }

    #[test]
    fn windows_and_metric_identities() {
        let (lo2, hi) = win92();
        let (lo3, hi3) = win93();
        assert!(hi.eq_x(hi3)); // the shared upper event
        assert_eq!(lo2.sub(Nq::ONE).sgn(), Ordering::Less);
        assert_eq!(lo3.sub(Nq::ONE).sgn(), Ordering::Less);
        assert_eq!(hi.sub(Nq::ONE).sgn(), Ordering::Greater);
        // |P|^2 exact forms.
        assert!(gdot(pin92(), pin92()).eq_x(Nq::new(K9([13, 2, -2]), K9::ZERO, 4)));
        assert!(gdot(pin93(), pin93()).eq_x(Nq::new(K9([12, -1, -3]), K9::ZERO, 4)));
        // Centre-centre distances: |c_B - c|^2 = inradius^2 for (9,2)
        // (= (1+8b+4b^2)/12) and (2 r sin 10 deg)^2 for (9,3)
        // (= (16 - b - 5b^2)/12).
        let c = gcentre(&hull());
        for (p, want) in [
            (pin92(), Nq::new(K9([1, 8, 4]), K9::ZERO, 12)),
            (pin93(), Nq::new(K9([16, -1, -5]), K9::ZERO, 12)),
        ] {
            let dd = gsub(gadd(rot20(c), p), c);
            assert!(gdot(dd, dd).eq_x(want));
        }
        // cos^2 ordering sanity: (2+b)/4 > 1/4.
        assert_eq!(cos293().sub(cos292()).sgn(), Ordering::Greater);
    }

    /// Regression battery for the `sgn` mixed-sign arm (docs §4.3, §6.3):
    /// the shipped arm returned `sgn(u)` unconditionally (the
    /// `Ordering::then_with` returns `self` unless Equal), which
    /// disagrees with the product rule `sgn(u) · sgn(u² − s·v²)`
    /// exactly on mixed-sign elements with u² < s·v². The elements
    /// below pin both sides of the u² vs s·v² split, plus same-sign
    /// and zero controls. Expected signs are derived from the product
    /// rule (exact K₉ arithmetic) and cross-checked against the f64
    /// float view (`Nq::fl`, w₉ = 2 sin 20° from the shipped K9 float
    /// view).
    #[test]
    fn sgn_mixed_sign_product_rule_regression() {
        // Mandated element (docs §4.3, §6.3): 1 − 2·w₉ ≈ −0.368 < 0.
        // u = 1, v = −2: u² = 1 < s·4 (s = 2−b ≈ 0.468) → product
        // rule: Greater · Less = Less.
        let x = Nq::new(K9::ONE, K9([-2, 0, 0]), 1);
        assert_eq!(x.sgn(), Ordering::Less);
        assert!(x.fl() < 0.0, "1 − 2w₉ ≈ {} < 0", x.fl());

        // Mixed signs, both sides of the u² vs s·v² split:
        //   u = 1, v = −1:  u² = 1 > s·1 ≈ 0.468 → Greater · Greater = Greater
        //   u = −1, v = 1:  u² = 1 > s·1 ≈ 0.468 → Less · Greater = Less
        //   u = −1, v = 2:  u² = 1 < s·4 ≈ 1.87 → Less · Less = Greater
        let cases: [(K9, K9, Ordering); 3] = [
            (K9::ONE, K9([-1, 0, 0]), Ordering::Greater),
            (K9([-1, 0, 0]), K9::ONE, Ordering::Less),
            (K9([-1, 0, 0]), K9([2, 0, 0]), Ordering::Greater),
        ];
        for (u, v, expect) in cases {
            let x = Nq::new(u, v, 1);
            assert_eq!(x.sgn(), expect);
            // cross-check against the f64 view (w₉ from the shipped float path)
            let fl = x.fl();
            assert!(
                (fl > 0.0 && expect == Ordering::Greater) || (fl < 0.0 && expect == Ordering::Less),
                "f64 cross-check: {:?} fl = {} expect {:?}", x, fl, expect
            );
            // fl() must be consistent with the product rule's signs
            let su = u.sign();
            let term = u
                .mul(u)
                .sub(S_NQ.mul(v.mul(v)))
                .sign();
            let product_rule = if su == term { Ordering::Greater } else { Ordering::Less };
            assert_eq!(product_rule, expect, "product rule for {:?}", x);
        }
        // the u = −1, v = 2 element is the mirror image of 1 − 2w₉
        // (u = 1, v = −2); note sgn(−x) = sgn(x).reverse()
        let mirror = Nq::new(K9([-1, 0, 0]), K9([2, 0, 0]), 1);
        let original = Nq::new(K9::ONE, K9([-2, 0, 0]), 1);
        assert_eq!(mirror.sgn(), original.sgn().reverse());

        // Same-sign controls (the arm that was already correct):
        //   u = 1, v = 2:  sgn(u) = Greater; same-sign ⇒ Greater
        //   u = −1, v = −1: sgn(u) = Less; same-sign ⇒ Less
        let same_sign: [(K9, K9, Ordering); 2] = [
            (K9::ONE, K9([2, 0, 0]), Ordering::Greater),
            (K9([-1, 0, 0]), K9([-1, 0, 0]), Ordering::Less),
        ];
        for (u, v, expect) in same_sign {
            let x = Nq::new(u, v, 1);
            assert_eq!(x.sgn(), expect);
            let fl = x.fl();
            assert!(
                (fl > 0.0 && expect == Ordering::Greater) || (fl < 0.0 && expect == Ordering::Less),
                "f64 cross-check: {:?} fl = {} expect {:?}", x, fl, expect
            );
        }

        // Zero controls: u = 0 / v = 0 (zero factors ⇒ the `Equal` arms)
        let zero_v = Nq::new(K9::ONE, K9::ZERO, 1);
        let zero_u = Nq::new(K9::ZERO, K9::ONE, 1);
        assert_eq!(zero_v.sgn(), Ordering::Greater);
        assert_eq!(zero_u.sgn(), Ordering::Greater);
        let zero = Nq::ZERO;
        assert_eq!(zero.sgn(), Ordering::Equal);
    }

    fn gp_eq(p: &Gp<Nq>, q: &Gp<Nq>) -> bool {
        p.0.eq_x(q.0) && p.1.eq_x(q.1)
    }
}
