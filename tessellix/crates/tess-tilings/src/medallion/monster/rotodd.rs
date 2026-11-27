//! The shared ROTATED odd-n monster engine (M181 §7 engine gaps 1–3).
//!
//! The `{9/3}` + `{9/1}` records (module [`n9`](super::n9)) established
//! the placement shape for ODD `n`: `B = R·A + P` with `R` the rotation
//! by `pi/n`, the pinning rule `P = M_i − R·M_j`, the forced figure
//! step `d ≡ 2·(i − j) − 1 (mod n)`, and the N-layer coordinate type
//! `(u + v·w)/den` over the real cyclotomic field `K_n` with `w² = s =
//! 2 − b` (`b = 2cos(2pi/n)`; odd n puts `w` outside `K_n`, and `a =
//! 2cos(pi/n)` INSIDE it, `a = −C_((n+1)/2)(b)`).
//!
//! This module lifts that machinery from `K₉` to any odd-n core field:
//! [`Core`] is the surface the N-layer needs from `K_n` (`K9`, `K11`,
//! `K13` implement it, each forwarding to its certified inherent
//! operations — the ISO-bracket `ksign` engine included: at the shipped
//! brackets the record workload needs ZERO bisections, M181 §9/S8),
//! [`Nq<F>`] is the generic canonical coordinate, [`hull_odd`] walks
//! the unit-edge regular n-gon, and [`verify_pair_rot`] is the full
//! exact placement verifier (the former `n9::verify_pair9`, with the
//! kappa offsets generalized from the hardcoded `+4`/`+5` to `+h`/`+(h
//! + 1)`, `h = (n−1)/2`, and the exact `t = 63/128 / 65/128` sample pin
//! generalized to the certified affine line `t(sigma) = t0 + k·sigma`
//! (and `s`): at `n = 9` (`(t0, k)` = `(0, ½)` resp. `(1, −½)`) the two
//! forms pin the same values, so the shipped records are unchanged).
//!
//! The genericization decision recorded in M181 §12 (const-generic vs
//! macro) resolves to this TRAIT OVER THE CORE FIELD: the tuple degree
//! `D` is baked into each core type, so the N-layer is generic once and
//! every arithmetic path forwards to the per-field certified code.
//!
//! The window-census layer ([`RotGeom`]) evaluates the pair state at an
//! exact `sigma` held as a HOMOGENEOUS ratio `num / den` of N-layer
//! elements — every sign cross-multiplies through `den`, no field
//! division happens on any path (the M164 discipline) — and runs the
//! spike's maximal-window walk (event list, sort, dedupe, bracket of
//! `sigma = 1`, walk out while the state key holds) at the implemented
//! field. It is the executor of the M181 §8.6 window-census mandate in
//! the `n11`/`n13` batteries.

use super::{
    gadd, gcross, gcrossings, gdot, gedges_inside, ginside, gmidpoints, gmul_i, gsub, hom_eq,
    hom_of, hull_sanity, gcentre, assert_general_position, FieldEl, Gp, PairData, WindowProbe,
};
use crate::fields::{K9, K11, K13};
use std::cell::Cell;
use std::cmp::Ordering;

// =====================================================================
// The core-field surface and the generic N-layer
// =====================================================================

/// The core real-cyclotomic field `K_n = Q(b)` of a rotated odd-n
/// record, with exactly the surface the N-layer [`Nq`] consumes. Each
/// implementation forwards to the field's certified inherent
/// operations (tuple arithmetic over the reduction table, the
/// isolating-interval sign engine at the M178/M179-certified brackets).
pub trait Core: Copy + PartialEq + Eq + std::fmt::Debug {
    /// The zero element.
    const ZERO: Self;
    /// The unit element.
    const ONE: Self;
    /// `-1`.
    const NEG_ONE: Self;
    /// `b = 2cos(2pi/n)`, the field generator.
    fn b() -> Self;
    /// `a = 2cos(pi/n) = -C_((n+1)/2)(b)` — in `K_n` for odd `n`
    /// (`zeta_2n` in `Q(zeta_n)`). The `n11`/`n13` batteries re-assert
    /// the two defining identities exactly (`a^2 = 2 + b`, `a > 0`).
    fn a() -> Self;
    fn add(self, y: Self) -> Self;
    fn sub(self, y: Self) -> Self;
    fn neg(self) -> Self;
    /// Exact scaling by an integer.
    fn scale(self, c: i64) -> Self;
    fn mul(self, y: Self) -> Self;
    /// Exact sign — the isolating-interval engine of the field.
    fn sign(self) -> Ordering;
    /// Float value — cross-check and emission views only.
    fn float(self) -> f64;
    /// Power-basis coordinates, zero-padded to six slots (the padded
    /// gcd canonicalization never sees the pad).
    fn coords(self) -> [i64; 6];
    /// Rebuild from padded coordinates (the pad must be zero).
    fn from_coords(c: &[i64; 6]) -> Self;
}

/// `s = w^2 = 2 - b` (positive at every shipped odd `n`).
pub fn core_s<F: Core>() -> F {
    F::ONE.scale(2).sub(F::b())
}

impl Core for K9 {
    const ZERO: K9 = K9::ZERO;
    const ONE: K9 = K9::ONE;
    const NEG_ONE: K9 = K9([-1, 0, 0]);
    fn b() -> K9 {
        K9::B
    }
    fn a() -> K9 {
        // 2cos(20 deg): a^2 = b + 2 with a > 0 (the shipped `2 cos 20`
        // numerator, n9.rs `c20`).
        K9([-2, 1, 1])
    }
    fn add(self, y: K9) -> K9 {
        K9::add(self, y)
    }
    fn sub(self, y: K9) -> K9 {
        K9::sub(self, y)
    }
    fn neg(self) -> K9 {
        K9::neg(self)
    }
    fn scale(self, c: i64) -> K9 {
        K9::scale(self, c)
    }
    fn mul(self, y: K9) -> K9 {
        K9::mul(self, y)
    }
    fn sign(self) -> Ordering {
        K9::sign(&self)
    }
    fn float(self) -> f64 {
        K9::float(&self)
    }
    fn coords(self) -> [i64; 6] {
        [self.0[0], self.0[1], self.0[2], 0, 0, 0]
    }
    fn from_coords(c: &[i64; 6]) -> K9 {
        assert!(c[3] == 0 && c[4] == 0 && c[5] == 0, "K9 pad");
        K9([c[0], c[1], c[2]])
    }
}

impl Core for K11 {
    const ZERO: K11 = K11::ZERO;
    const ONE: K11 = K11::ONE;
    const NEG_ONE: K11 = K11([-1, 0, 0, 0, 0]);
    fn b() -> K11 {
        K11::B
    }
    fn a() -> K11 {
        // 2cos(pi/11): a^2 = b + 2, a > 0; equals -C_6(b). Verified
        // exactly in the n11 battery; identical tuple as the (11,3)
        // certified s0 constant (M181 §3).
        K11([1, -2, -3, 1, 1])
    }
    fn add(self, y: K11) -> K11 {
        K11::add(self, y)
    }
    fn sub(self, y: K11) -> K11 {
        K11::sub(self, y)
    }
    fn neg(self) -> K11 {
        K11::neg(self)
    }
    fn scale(self, c: i64) -> K11 {
        K11::scale(self, c)
    }
    fn mul(self, y: K11) -> K11 {
        K11::mul(self, y)
    }
    fn sign(self) -> Ordering {
        K11::sign(&self)
    }
    fn float(self) -> f64 {
        K11::float(&self)
    }
    fn coords(self) -> [i64; 6] {
        [self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], 0]
    }
    fn from_coords(c: &[i64; 6]) -> K11 {
        assert!(c[5] == 0, "K11 pad");
        K11([c[0], c[1], c[2], c[3], c[4]])
    }
}

impl Core for K13 {
    const ZERO: K13 = K13::ZERO;
    const ONE: K13 = K13::ONE;
    const NEG_ONE: K13 = K13([-1, 0, 0, 0, 0, 0]);
    fn b() -> K13 {
        K13::B
    }
    fn a() -> K13 {
        // 2cos(pi/13): a^2 = b + 2, a > 0; equals -C_7(b). Verified
        // exactly in the n13 battery; identical tuple as the (13,4)
        // certified s0 constant (M181 §3).
        K13([1, 3, -3, -4, 1, 1])
    }
    fn add(self, y: K13) -> K13 {
        K13::add(self, y)
    }
    fn sub(self, y: K13) -> K13 {
        K13::sub(self, y)
    }
    fn neg(self) -> K13 {
        K13::neg(self)
    }
    fn scale(self, c: i64) -> K13 {
        K13::scale(self, c)
    }
    fn mul(self, y: K13) -> K13 {
        K13::mul(self, y)
    }
    fn sign(self) -> Ordering {
        K13::sign(&self)
    }
    fn float(self) -> f64 {
        K13::float(&self)
    }
    fn coords(self) -> [i64; 6] {
        self.0
    }
    fn from_coords(c: &[i64; 6]) -> K13 {
        K13(*c)
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn gcd_arr(vals: &[i64]) -> i64 {
    vals.iter().fold(0, |g, x| gcd(g, *x)).max(1)
}

/// An element `u + v*w` of `N = K_n(w)`, `w^2 = s = 2 - b` (`w > 0`),
/// over a positive integer denominator, gcd-reduced over all integers
/// (canonical form, so derived equality is exact equality). Every
/// comparison cross-multiplies; there is no division on any path. The
/// genericization of the shipped `n9::Nq` (M181 §7 gap 1): identical
/// scheme, identical method set, the core field carried as `F`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Nq<F> {
    pub u: F,
    pub v: F,
    pub den: i64,
}

impl<F: Core> Nq<F> {
    /// `(u + v*w) / den`, gcd-reduced (`den > 0` required).
    pub fn new(u: F, v: F, den: i64) -> Nq<F> {
        assert!(den > 0, "Nq denominator must be positive");
        let mut vals = [0i64; 13];
        vals[..6].copy_from_slice(&u.coords());
        vals[6..12].copy_from_slice(&v.coords());
        vals[12] = den;
        let g = gcd_arr(&vals);
        let mut du = u.coords();
        let mut dv = v.coords();
        for x in du.iter_mut() {
            *x /= g;
        }
        for x in dv.iter_mut() {
            *x /= g;
        }
        Nq {
            u: F::from_coords(&du),
            v: F::from_coords(&dv),
            den: den / g,
        }
    }
    /// The integer `x` at denominator 1.
    pub fn whole(x: i64) -> Nq<F> {
        Nq {
            u: F::ONE.scale(x),
            v: F::ZERO,
            den: 1,
        }
    }
    /// Exact scaling by an integer.
    pub fn scale(self, c: i64) -> Nq<F> {
        Nq::new(self.u.scale(c), self.v.scale(c), self.den)
    }
    /// The zero element.
    pub const ZERO: Nq<F> = Nq {
        u: F::ZERO,
        v: F::ZERO,
        den: 1,
    };
    /// The unit element.
    pub const ONE: Nq<F> = Nq {
        u: F::ONE,
        v: F::ZERO,
        den: 1,
    };
    /// `-1`.
    pub const NEG_ONE: Nq<F> = Nq {
        u: F::NEG_ONE,
        v: F::ZERO,
        den: 1,
    };
    /// Exact sum (cross-multiplication).
    pub fn add(self, o: Nq<F>) -> Nq<F> {
        Nq::new(
            self.u.scale(o.den).add(o.u.scale(self.den)),
            self.v.scale(o.den).add(o.v.scale(self.den)),
            self.den * o.den,
        )
    }
    /// Exact difference.
    pub fn sub(self, o: Nq<F>) -> Nq<F> {
        Nq::new(
            self.u.scale(o.den).sub(o.u.scale(self.den)),
            self.v.scale(o.den).sub(o.v.scale(self.den)),
            self.den * o.den,
        )
    }
    /// Exact negation.
    pub fn neg(self) -> Nq<F> {
        Nq {
            u: self.u.neg(),
            v: self.v.neg(),
            den: self.den,
        }
    }
    /// Field product: `(u1 + v1 w)(u2 + v2 w) = u1 u2 + s v1 v2 +
    /// (u1 v2 + u2 v1) w`.
    pub fn mul(self, o: Nq<F>) -> Nq<F> {
        Nq::new(
            self.u.mul(o.u).add(core_s::<F>().mul(self.v.mul(o.v))),
            self.u.mul(o.v).add(o.u.mul(self.v)),
            self.den * o.den,
        )
    }
    /// Exact sign. Same-sign cases are immediate; for mixed signs
    /// `x = u + v w` satisfies `x · (u − v w) = u² − s v²`, so
    /// `sgn x = sgn(u) · sgn(u² − s v²)` (both factors are exact `K_n`
    /// signs). `u² = s v²` for `(u, v) != 0` is impossible: it would
    /// make `s = 2 − b` a square in `K_n`, and `N(s) = n` is prime.
    /// The M179 §4.3 lesson: this is the PRODUCT rule, not `sgn(u)` —
    /// the mixed-sign regression of the n11/n13 batteries pins it.
    pub fn sgn(self) -> Ordering {
        let su = self.u.sign();
        let sv = self.v.sign();
        match (su, sv) {
            (Ordering::Equal, o) => o,
            (o, Ordering::Equal) => o,
            (Ordering::Greater, Ordering::Greater) => Ordering::Greater,
            (Ordering::Less, Ordering::Less) => Ordering::Less,
            (su, _) => {
                let product = self
                    .u
                    .mul(self.u)
                    .sub(core_s::<F>().mul(self.v.mul(self.v)))
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
    pub fn eq_x(self, o: Nq<F>) -> bool {
        self.u.scale(o.den) == o.u.scale(self.den) && self.v.scale(o.den) == o.v.scale(self.den)
    }
    /// Float value — landed cross-check views only (`w = 2 sin(pi/n)`
    /// derived from the field float view of `b`, no decimal literal).
    pub fn fl(self) -> f64 {
        let w = (2.0 - F::b().float()).sqrt();
        (self.u.float() + self.v.float() * w) / self.den as f64
    }
    /// Canonical-form check (emitted-numbers battery): positive
    /// denominator and gcd-reduced across all eleven/tirteen integers.
    pub fn is_canonical(self) -> bool {
        self.den > 0 && {
            let mut vals = [0i64; 13];
            vals[..6].copy_from_slice(&self.u.coords());
            vals[6..12].copy_from_slice(&self.v.coords());
            vals[12] = self.den;
            gcd_arr(&vals) == 1
        }
    }
}

impl<F: Core> FieldEl for Nq<F> {
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
// Rotations and the unit-edge regular n-gon hull
// =====================================================================

/// Rotation by `2pi/n` about the origin (the hull walk step):
/// `cos = b/2`, `sin = w·a/2` (`sin(2pi/n) = 2 sin(pi/n)cos(pi/n)`).
pub fn rot_step<F: Core>(p: Gp<Nq<F>>) -> Gp<Nq<F>> {
    let c = Nq::new(F::b(), F::ZERO, 2);
    let s = Nq::new(F::ZERO, F::a(), 2);
    (
        c.mul(p.0).sub(s.mul(p.1)),
        s.mul(p.0).add(c.mul(p.1)),
    )
}

/// Rotation by `pi/n` about the origin — the pair rotation `R`:
/// `cos = a/2`, `sin = w/2`.
pub fn rot_half<F: Core>(p: Gp<Nq<F>>) -> Gp<Nq<F>> {
    let c = Nq::new(F::a(), F::ZERO, 2);
    let s = Nq::new(F::ZERO, F::ONE, 2);
    (
        c.mul(p.0).sub(s.mul(p.1)),
        s.mul(p.0).add(c.mul(p.1)),
    )
}

/// The record hull: the unit-edge regular odd-n gon, normalized frame
/// (`V0 = origin`, edge 0 along +x), built by walking unit edge
/// directions `u_(k+1) = rot_step(u_k)`. The closure lemma
/// `rot_step^n = id`, `Sum u_k = 0` and the unit-length lemma
/// `|u_k|^2 = 1` are asserted exactly (the scheme of `n9::hull`,
/// which delegates here — byte-identical values and messages).
pub fn hull_odd<F: Core>(n: usize, tag: &str) -> Vec<Gp<Nq<F>>> {
    assert!(n >= 5 && n % 2 == 1, "{tag}: hull needs odd n >= 5");
    let mut pts = vec![(Nq::ZERO, Nq::ZERO)];
    let mut u: Gp<Nq<F>> = (Nq::ONE, Nq::ZERO);
    let mut sum: Gp<Nq<F>> = (Nq::ZERO, Nq::ZERO);
    for k in 0..n {
        assert!(gdot(u, u).eq_x(Nq::ONE), "{tag}: |u_k| != 1");
        sum = gadd(sum, u);
        pts.push(sum);
        u = rot_step(u);
        let _ = k;
    }
    assert!(
        u.0.eq_x(Nq::ONE) && u.1.eq_x(Nq::ZERO),
        "{tag}: rot_step^{n} != id"
    );
    assert!(
        sum.0.eq_x(Nq::ZERO) && sum.1.eq_x(Nq::ZERO),
        "{tag}: walk does not close"
    );
    pts.pop();
    pts
}

/// The rotated pin label `P(i, j) = M_i - R·M_j` from the midpoint
/// list `m` (M181 §1: the rotated family's pin map; the record pin is
/// `P(Delta, 0)`).
pub fn pin_rot<F: Core>(m: &[Gp<Nq<F>>], i: usize, j: usize) -> Gp<Nq<F>> {
    gsub(m[i], rot_half(m[j]))
}

/// Chebyshev value `C_d(b)` over the core field: the tower `C_0 = 2`,
/// `C_1 = b`, `C_(k+1) = b·C_k - C_(k-1)`, so `C_d(2 cos t) = 2 cos
/// (d t)`. The contact-angle certificate is `cos^2(d pi / n) = (2 +
/// C_d(b)) / 4` (M181 §2 mandate 4).
pub fn cheb<F: Core>(d: usize) -> F {
    let b = F::b();
    let mut prev = F::ONE.scale(2);
    if d == 0 {
        return prev;
    }
    let mut cur = b;
    for _ in 1..d {
        let next = cur.mul(b).sub(prev);
        prev = cur;
        cur = next;
    }
    cur
}

/// The Galois conjugation `b -> C_k(b)` applied to a core element
/// (the automorphism `zeta + zeta^-1 -> zeta^k + zeta^-k`, valid for
/// `k` coprime to `n`; `k = 1` is the identity). Horner over the
/// power-basis coordinates with `b -> C_k(b)`.
pub fn conj<F: Core>(x: F, k: usize) -> F {
    let ck = cheb::<F>(k);
    let mut acc = F::ZERO;
    for &c in x.coords().iter().rev() {
        acc = acc.mul(ck).add(F::ONE.scale(c));
    }
    acc
}

// =====================================================================
// The rotated placement record: spec + verifier
// =====================================================================

/// Catalog entry input for a rotated pair placement — the generic
/// form of the former `n9::PairSpec9` (M181 §7 gap 3), with the
/// certified crossing-parameter line `(t0, k)` / `(s0, ks)` added and
/// the derived `t_increasing` flag dropped (the direction now follows
/// from `sgn k`, asserted against the samples).
pub struct RotSpec<F> {
    /// Label for assertion messages.
    pub name: &'static str,
    /// Hull order (odd): `h = (n - 1) / 2` is the kappa offset.
    pub n: usize,
    /// Exact displacement: `B = R·A + P` (`R` the `pi/n` rotation).
    pub p: Gp<F>,
    /// Star figure step: the contact angle is `d * pi / n`.
    pub d: usize,
    /// Certified `A`-edges strictly inside `B` (a consecutive run).
    pub e_ab: Vec<usize>,
    /// Certified `B`-edges strictly inside `A` (the half-turn image
    /// `{i + h}` under kappa — not the parallel `{i + n/2}`).
    pub e_ba: Vec<usize>,
    /// Certified crossing edge pairs `(A_i, B_j)`, sorted by `A`-edge:
    /// `(Delta, 0)` and `(h + 1, Delta + h)` (M181 §1).
    pub cross: [(usize, usize); 2],
    /// Exact `cos^2(d pi / n)`.
    pub cos2: F,
    /// Certified window ends along the ray `sigma * P` (pin `sigma = 1`).
    pub s_lo: F,
    pub s_hi: F,
    /// Certified crossing-parameter line of the PRIMARY crossing
    /// (`t(sigma) = t0 + k*sigma`, `s(sigma) = s0 + ks*sigma`); the
    /// midpoint pin is `t0 + k = s0 + ks = 1/2` and `k != 0`
    /// (M181 §3/§8.7).
    pub t0: F,
    pub k: F,
    pub s0: F,
    pub ks: F,
    /// Certified union outline (cyclic order, [`rotated_cycles`]).
    pub union_cycle: Vec<super::OutNode>,
    /// Certified lens outline (cyclic order).
    pub lens_cycle: Vec<super::OutNode>,
}

/// Build the two outline cycles of a rotated placement (module docs,
/// "Outline structure"): `X0` on `A`-edge `ea0` and `B`-edge `eb0`,
/// `X1` on `A`-edge `ea1` and `B`-edge `eb1` (the absorbed `A`-run is
/// `ea0 .. ea1`, the absorbed `B`-run `(eb1+1) .. eb0`, `c` edges
/// each). Unlike the parallel records, the two polygons' index frames
/// are shifted by the rotation: the union carries, after `X0`, the
/// `B`-arc `V_(eb0+1) .. V_(eb1)` and after `X1` the `A`-arc
/// `V_(ea1+1) .. V_(ea0)` (the lens the complementary arcs). The sizes
/// are exactly the catalog formulas `2(n - c - 1) + 2` (union) and
/// `2c + 4` (lens). (Moved verbatim from the former `n9::rotated_cycles`
/// — already n-generic.)
pub fn rotated_cycles(
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

/// The full exact verification of one rotated monster placement — the
/// shared odd-n analog of `verify_pair` (the former `n9::verify_pair9`,
/// generalized): hull schema and closure lemma, the half-turn swap
/// `kappa` (vertex maps both directions, midpoint map, and the
/// crossing swap `(A_i, B_j) -> (A_(j+h+1), B_(i+h))`), general
/// position, the definition counts with the `+h` pairing, absorbed
/// contacts, exactly two transversal crossings at the edge-midpoint
/// pin (`P = M_i − R·M_j`), the angle identity, the rotated
/// figure-step rule `d ≡ 2(i−j) − 1`, the exact outline cycles with
/// every piece re-certified by a midpoint half-plane test, the window
/// samples `sigma = 1 -+ 1/64` (lists and crossings kept, `t != 1/2`,
/// the crossing parameters EXACTLY on the certified line `t0 + k·sigma`
/// and `s0 + ks·sigma`, moving with `sgn k`), and the tabulated
/// out-of-window probes.
pub fn verify_pair_rot<C: Core>(
    spec: &RotSpec<Nq<C>>,
    probes: &[WindowProbe<Nq<C>>],
) -> PairData<Nq<C>> {
    let a = hull_odd::<C>(spec.n, spec.name);
    let n = a.len();
    let h = (n - 1) / 2; // kappa offset pair (h, h+1)
    hull_sanity(spec.name, &a);

    let p = spec.p;
    let b: Vec<Gp<Nq<C>>> = a.iter().map(|&q| gadd(rot_half(q), p)).collect();
    let m = gmidpoints(&a);

    // Half-turn swap lemma (n9 module docs): with cB = R*c + P and
    // q = c + cB, kappa(X) = q - X maps V_k^A -> V_(k+h)^B,
    // M_k^A -> M_(k+h)^B and V_k^B -> V_(k+h+1)^A.
    let c = gcentre(&a);
    let cb = gadd(rot_half(c), p);
    let q = gadd(c, cb);
    for k in 0..n {
        let ka = gsub(q, a[k]);
        assert!(
            ka.0.eq_x(b[(k + h) % n].0) && ka.1.eq_x(b[(k + h) % n].1),
            "{}: kappa(V_{k}^A) != V_(k+h)^B",
            spec.name
        );
        let km = gsub(q, m[k]);
        let mb = gadd(rot_half(m[(k + h) % n]), p);
        assert!(
            km.0.eq_x(mb.0) && km.1.eq_x(mb.1),
            "{}: kappa(M_{k}^A) != M_(k+h)^B",
            spec.name
        );
        let kb = gsub(q, b[k]);
        assert!(
            kb.0.eq_x(a[(k + h + 1) % n].0) && kb.1.eq_x(a[(k + h + 1) % n].1),
            "{}: kappa(V_{k}^B) != V_(k+h+1)^A",
            spec.name
        );
    }

    assert_general_position(&a, &b, spec.name);

    // Definition: c full edges strictly inside, both ways, certified
    // lists (consecutive runs), and eBA = {i + h} (the kappa image).
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
    assert_eq!(e_ba.len(), cabs, "{}: c_A != c_B", spec.name);
    let mut want: Vec<usize> = e_ab.iter().map(|&i| (i + h) % n).collect();
    want.sort_unstable();
    assert_eq!(e_ba, want, "{}: eBA != kappa image +h", spec.name);

    // Absorbed contacts (interior edge midpoints) = interior edges.
    let absorbed_a: Vec<usize> = (0..n).filter(|&i| ginside(m[i], &b)).collect();
    assert_eq!(absorbed_a, e_ab, "{}: absorbed contacts A-in-B", spec.name);
    let absorbed_b: Vec<usize> = (0..n)
        .filter(|&i| ginside(gadd(rot_half(m[i]), p), &a))
        .collect();
    assert_eq!(absorbed_b, e_ba, "{}: absorbed contacts B-in-A", spec.name);

    // Exactly two transversal crossings, at edge midpoints, on the
    // certified edge pairs, with P = M_i − R·M_j (the pinning
    // identity) and the angle identity cos^2 = cos^2(d pi / n).
    let cr = gcrossings(&a, &b);
    assert_eq!(cr.len(), 2, "{}: not exactly 2 crossings", spec.name);
    let two = Nq::<C>::whole(2);
    for (k, ci) in cr.iter().enumerate() {
        assert_eq!(ci.ea, spec.cross[k].0, "{}: crossing {k} A-edge", spec.name);
        assert_eq!(ci.eb, spec.cross[k].1, "{}: crossing {k} B-edge", spec.name);
        assert!(
            two.mul(ci.t_num).eq_x(ci.den) && two.mul(ci.s_num).eq_x(ci.den),
            "{}: crossing {k} not at the edge midpoints",
            spec.name
        );
        let ma = m[ci.ea];
        let mb = gadd(rot_half(m[ci.eb]), p);
        assert!(
            hom_eq(&ci.x, &hom_of(ma)) && hom_eq(&ci.x, &hom_of(mb)),
            "{}: crossing {k} off the midpoint",
            spec.name
        );
        let pin = gsub(ma, rot_half(m[ci.eb]));
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
    // The half-turn maps crossings (A_i, B_j) -> (A_(j+h+1), B_(i+h))
    // and swaps the two crossing POINTS.
    assert_eq!(
        cr[1].ea,
        (cr[0].eb + h + 1) % n,
        "{}: crossing swap image",
        spec.name
    );
    assert_eq!(
        cr[1].eb,
        (cr[0].ea + h) % n,
        "{}: crossing swap image",
        spec.name
    );
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
    // run's end vertices), the B-hidden run is `eb1 + 1 .. eb0`.
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
    // well and rederives the same cycles. Cross-check the certified
    // lists against it.
    let ea = (cr[0].ea, cr[1].ea);
    let eb = (cr[0].eb, cr[1].eb);
    let exa: Vec<bool> = (0..n).map(|k| !ginside(a[k], &b)).collect();
    let exb: Vec<bool> = (0..n).map(|k| !ginside(b[k], &a)).collect();
    let (hu, hl) = super::outlines(n, &exa, &exb, ea, eb);
    assert_eq!(hu, union_cycle, "{}: hub union rederivation", spec.name);
    assert_eq!(hl, lens_cycle, "{}: hub lens rederivation", spec.name);
    let mid = |u: Gp<Nq<C>>, v: Gp<Nq<C>>| {
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
    // `n - c - 1` per side. The rotated twist is WHICH vertices: on
    // `A` the union carries `ea1 + 1 .. ea0`, on `B` the arc `eb0 + 1
    // .. eb1` (module docs).
    let exposed_a: Vec<usize> = (0..n).filter(|&k| !ginside(a[k], &b)).collect();
    let exposed_b: Vec<usize> = (0..n).filter(|&k| !ginside(b[k], &a)).collect();
    assert_eq!(exposed_a.len(), n - cabs - 1, "{}: exposed A count", spec.name);
    assert_eq!(exposed_b.len(), n - cabs - 1, "{}: exposed B count", spec.name);

    // Window: the pin `sigma = 1` is strictly interior (`s_lo < 1 <
    // s_hi`). Mandate 7 (M181 §8): the crossing-parameter LINEARITY,
    // exact — `t(sigma) = t0 + k*sigma`, `s(sigma) = s0 + ks*sigma`,
    // `t0 + k = s0 + ks = 1/2`, `k != 0`; at the samples `sigma = 1 -+
    // 1/64` both parameters are off `1/2`, equal the line EXACTLY, and
    // move with `sgn k`. For the shipped n = 9 records `(t0, k)` is
    // `(0, 1/2)` ((9,2)) resp. `(1, -1/2)` ((9,3)) — the line then
    // reads `t = sigma/2` / `(2 - sigma)/2`, pinning exactly the old
    // `63/128` / `65/128` sample values (identical behavior).
    let half = Nq::<C>::ONE.scaled_div(1, 2);
    assert!(
        spec.t0.add(spec.k).eq_x(half),
        "{}: t0 + k != 1/2",
        spec.name
    );
    assert!(
        spec.s0.add(spec.ks).eq_x(half),
        "{}: s0 + ks != 1/2",
        spec.name
    );
    assert_ne!(spec.k.sgn(), Ordering::Equal, "{}: k = 0", spec.name);
    assert_ne!(spec.ks.sgn(), Ordering::Equal, "{}: ks = 0", spec.name);
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
        let b2: Vec<Gp<Nq<C>>> = a.iter().map(|&q| gadd(rot_half(q), p2)).collect();
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
        // The exact line values at the sample (t = t_num / den):
        let want_t = spec.t0.add(spec.k.mul(sig)).mul(ci.den);
        assert!(
            ci.t_num.eq_x(want_t),
            "{}: t(sigma) off the certified line",
            spec.name
        );
        let want_s = spec.s0.add(spec.ks.mul(sig)).mul(ci.den);
        assert!(
            ci.s_num.eq_x(want_s),
            "{}: s(sigma) off the certified line",
            spec.name
        );
        // Off the pin at the samples, and the direction through the
        // pin: sign(t(sigma) - 1/2) = sign(sigma - 1) * sign(k) (and
        // the same for s with sign(ks)). `t > 1/2` iff `2 t_num - den`
        // carries the sign of `den` (t = t_num / den).
        let orient = |diff: Nq<C>, den: Nq<C>| -> Ordering {
            let s = diff.sgn();
            if den.sgn() == Ordering::Greater {
                s
            } else {
                s.reverse()
            }
        };
        let tdiff = gmul_i(ci.t_num, 2).sub(ci.den);
        assert_ne!(tdiff.sgn(), Ordering::Equal, "{}: window sample still at the pin", spec.name);
        let sdiff = gmul_i(ci.s_num, 2).sub(ci.den);
        assert_ne!(sdiff.sgn(), Ordering::Equal, "{}: window sample s at the pin", spec.name);
        let above_t = orient(tdiff, ci.den);
        let expect_t = if sgnk == -1 {
            spec.k.sgn().reverse()
        } else {
            spec.k.sgn()
        };
        assert_eq!(
            above_t, expect_t,
            "{}: t(sigma) not monotone with sign(k) through the pin",
            spec.name
        );
        let above_s = orient(sdiff, ci.den);
        let expect_s = if sgnk == -1 {
            spec.ks.sgn().reverse()
        } else {
            spec.ks.sgn()
        };
        assert_eq!(above_s, expect_s, "{}: s(sigma) not monotone with sign(ks)", spec.name);
    }

    // Tabulated out-of-window probes (counts change as documented).
    for pr in probes {
        let w = (pr.rho.mul(p.0), pr.rho.mul(p.1));
        let b3: Vec<Gp<Nq<C>>> = a.iter().map(|&q| gadd(rot_half(q), w)).collect();
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

    // Independent window audit (M181 §8.6 + §8.11, at the hub): run
    // the sigma-ray event census at the pin — no event at `sigma = 1`,
    // no persistent incidence, and the MAXIMAL window around `sigma =
    // 1` equals the certified §3 bounds exactly. The census state keys
    // reproduce the pin state and every tabulated probe count, and the
    // mixed-sign counter stays zero throughout — the purity of the
    // construction is guarded by the comparator itself, not only in
    // the per-record battery. This is the same census the per-record
    // §8.6 batteries run, so the certified window constants are
    // re-derived, not just bracketed, inside every constructor.
    let g = RotGeom::new(n, p);
    let one: Sig<C> = (Nq::ONE, Nq::ONE);
    let pin_state = g.state(&one);
    assert_eq!(pin_state.0, cabs, "{}: census pin cA", spec.name);
    assert_eq!(pin_state.1, cabs, "{}: census pin cB", spec.name);
    assert_eq!(
        pin_state.2,
        vec![spec.cross[0], spec.cross[1]],
        "{}: census pin crossings",
        spec.name
    );
    assert_eq!(pin_state.3, 0, "{}: census pin dirty contacts", spec.name);
    let (lo, hi, _nev, persistent) = g.window(&pin_state);
    assert!(persistent.is_empty(), "{}: persistent incidences", spec.name);
    assert!(g.eq_sig_const(&lo, &spec.s_lo), "{}: s_lo != maximal window end", spec.name);
    assert!(g.eq_sig_const(&hi, &spec.s_hi), "{}: s_hi != maximal window end", spec.name);
    for pr in probes {
        let st = g.state(&(pr.rho, Nq::ONE));
        assert_eq!(st.0, pr.ca, "{}: census probe cA", spec.name);
        assert_eq!(st.1, pr.cb, "{}: census probe cB", spec.name);
        if let Some(k) = pr.crossings {
            assert_eq!(st.2.len(), k, "{}: census probe crossings", spec.name);
        }
        assert_eq!(st.3, 0, "{}: census probe dirty contacts", spec.name);
    }
    assert_eq!(g.mixed(), 0, "{}: mixed-sign N-layer elements signed", spec.name);

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

// =====================================================================
// The sigma-ray census layer (M181 §8.6 / §8.14): the pair state at an
// exact sigma, held as a HOMOGENEOUS ratio `num / den` — every sign
// cross-multiplies, no field division happens. The state convention is
// the spike's (`spike_24c_oddn.py`): absorbed counts, crossing edge
// pairs, vertex-on-edge-segment contact count.
// =====================================================================

/// Pair state at one sigma: `(c_A, c_B, crossing edge pairs sorted,
/// vertex-on-edge-segment contacts)` — the spike's `state_key` arms.
pub type StateKey = (usize, usize, Vec<(usize, usize)>, usize);

/// An exact sigma as a homogeneous pair `(num, den)`, `sigma = num /
/// den`.
pub type Sig<C> = (Nq<C>, Nq<C>);

/// The rotated geometry around one pin `P`: hull `A`, the rotated
/// frame `RV`/`RF` of `B`, and the sigma-ray machinery. Counts every
/// MIXED-SIGN N-layer element it signs (M181 §8.11 purity — the
/// shipped geometry must produce none).
pub(crate) struct RotGeom<C: Core> {
    pub a: Vec<Gp<Nq<C>>>,
    pub rv: Vec<Gp<Nq<C>>>,
    pub rf: Vec<Gp<Nq<C>>>,
    pub us: Vec<Gp<Nq<C>>>,
    pub p: Gp<Nq<C>>,
    mixed: Cell<u32>,
}

impl<C: Core> RotGeom<C> {
    pub(crate) fn new(n: usize, p: Gp<Nq<C>>) -> Self {
        let a = hull_odd::<C>(n, "rot census");
        let us: Vec<Gp<Nq<C>>> = (0..n).map(|q| gsub(a[(q + 1) % n], a[q])).collect();
        RotGeom {
            rv: (0..n).map(|q| rot_half(a[q])).collect(),
            rf: (0..n).map(|q| rot_half(us[q])).collect(),
            a,
            us,
            p,
            mixed: Cell::new(0),
        }
    }

    /// Exact sign with the purity counter: a MIXED-sign element (u and
    /// v signs opposite and nonzero) increments the counter.
    fn s(&self, x: &Nq<C>) -> Ordering {
        let su = x.u.sign();
        let sv = x.v.sign();
        let opp = matches!(
            (su, sv),
            (Ordering::Less, Ordering::Greater) | (Ordering::Greater, Ordering::Less)
        );
        if opp {
            self.mixed.set(self.mixed.get() + 1);
        }
        x.sgn()
    }

    /// `sgn(x / lam)` for the sigma denominator `lam`.
    fn sd(&self, x: &Nq<C>, lam: &Nq<C>) -> Ordering {
        let s = self.s(x);
        if self.s(lam) == Ordering::Greater {
            s
        } else {
            s.reverse()
        }
    }

    pub(crate) fn mixed(&self) -> u32 {
        self.mixed.get()
    }

    /// `B`-vertex `q` at sigma `num / lam`, homogeneous numerator
    /// (the true vertex is this divided by `lam`).
    fn wnum(&self, q: usize, num: &Nq<C>, lam: &Nq<C>) -> Gp<Nq<C>> {
        (
            self.rv[q].0.mul(*lam).add(self.p.0.mul(*num)),
            self.rv[q].1.mul(*lam).add(self.p.1.mul(*num)),
        )
    }

    /// The pair state at sigma: absorbed edge lists' counts, the
    /// crossing edge pairs, and the dirty-contact count — the spike's
    /// `pair_state` + `state_key`, division-free.
    pub(crate) fn state(&self, sig: &Sig<C>) -> StateKey {
        let (num, lam) = *sig;
        let n = self.a.len();
        // Transversal crossings: for (A_p, B_q) the edge directions
        // `us_p`, `rf_q` are sigma-free; with `D = W_q - V_p` the
        // numerators `cross(D, rf_q)` and `cross(D, us_p)` share the
        // denominator `cross(us_p, rf_q)` (spike `crossings_exact`).
        let mut cr: Vec<(usize, usize)> = Vec::new();
        for pp in 0..n {
            for q in 0..n {
                let den_c = gcross(self.us[pp], self.rf[q]);
                let ds = self.s(&den_c);
                if ds == Ordering::Equal {
                    continue;
                }
                let wn = self.wnum(q, &num, &lam);
                let dnum = gsub(wn, (self.a[pp].0.mul(lam), self.a[pp].1.mul(lam)));
                let numt = gcross(dnum, self.rf[q]);
                let st = self.sd(&numt, &lam);
                if st == Ordering::Equal || st != ds {
                    continue;
                }
                if self.sd(&numt.sub(den_c.mul(lam)), &lam) != ds.reverse() {
                    continue;
                }
                let nums = gcross(dnum, self.us[pp]);
                let ss = self.sd(&nums, &lam);
                if ss == Ordering::Equal || ss != ds {
                    continue;
                }
                if self.sd(&nums.sub(den_c.mul(lam)), &lam) != ds.reverse() {
                    continue;
                }
                cr.push((pp, q));
            }
        }
        let crossed_a: Vec<bool> = (0..n).map(|p| cr.iter().any(|&(x, _)| x == p)).collect();
        let crossed_b: Vec<bool> = (0..n).map(|q| cr.iter().any(|&(_, y)| y == q)).collect();
        // Absorbed A-edges: M_p^A strictly inside B (left of every
        // B-edge LINE — the spike's `strictly_inside_dir`).
        let mids = gmidpoints(&self.a);
        let eab: Vec<usize> = (0..n)
            .filter(|&p| {
                !crossed_a[p]
                    && (0..n).all(|q| {
                        let dnum = gsub(
                            (mids[p].0.mul(lam), mids[p].1.mul(lam)),
                            (self.rv[q].0.mul(lam), self.rv[q].1.mul(lam)),
                        );
                        let dnum = (
                            dnum.0.sub(self.p.0.mul(num)),
                            dnum.1.sub(self.p.1.mul(num)),
                        );
                        self.sd(&gcross(self.rf[q], dnum), &lam) == Ordering::Greater
                    })
            })
            .collect();
        // Absorbed B-edges: the B-edge midpoint strictly inside A.
        let eba: Vec<usize> = (0..n)
            .filter(|&q| {
                !crossed_b[q]
                    && {
                        let mm = gmidpoints(&[self.rv[q], self.rv[(q + 1) % n]]);
                        let mnum = (
                            mm[0].0.mul(lam).add(self.p.0.mul(num)),
                            mm[0].1.mul(lam).add(self.p.1.mul(num)),
                        );
                        (0..n).all(|p| {
                            let dnum = gsub(
                                mnum,
                                (self.a[p].0.mul(lam), self.a[p].1.mul(lam)),
                            );
                            self.sd(&gcross(self.us[p], dnum), &lam) == Ordering::Greater
                        })
                    }
            })
            .collect();
        // Dirty contacts: vertex ON the other side's edge SEGMENT
        // (the spike's `vertex_contacts`; `d` is the projection
        // parameter along the unit edge).
        let mut dirty = 0usize;
        for k in 0..n {
            for q in 0..n {
                let dnum = gsub(
                    (self.a[k].0.mul(lam), self.a[k].1.mul(lam)),
                    (self.rv[q].0.mul(lam), self.rv[q].1.mul(lam)),
                );
                let dnum = (dnum.0.sub(self.p.0.mul(num)), dnum.1.sub(self.p.1.mul(num)));
                if self.sd(&gcross(dnum, self.rf[q]), &lam) == Ordering::Equal {
                    let dn = gdot(dnum, self.rf[q]);
                    if self.sd(&dn, &lam) >= Ordering::Equal
                        && self.sd(&dn.sub(lam), &lam) <= Ordering::Equal
                    {
                        dirty += 1;
                    }
                }
            }
        }
        for mm in 0..n {
            for p in 0..n {
                let wn = self.wnum(mm, &num, &lam);
                let dnum = gsub(wn, (self.a[p].0.mul(lam), self.a[p].1.mul(lam)));
                if self.sd(&gcross(dnum, self.us[p]), &lam) == Ordering::Equal {
                    let dn = gdot(dnum, self.us[p]);
                    if self.sd(&dn, &lam) >= Ordering::Equal
                        && self.sd(&dn.sub(lam), &lam) <= Ordering::Equal
                    {
                        dirty += 1;
                    }
                }
            }
        }
        cr.sort_unstable();
        (eab.len(), eba.len(), cr, dirty)
    }

    /// Exact sigma comparison: `sgn((na db - nb da) * (da db))`.
    pub(crate) fn cmp_sig(&self, x: &Sig<C>, y: &Sig<C>) -> Ordering {
        let diff = x.0.mul(y.1).sub(y.0.mul(x.1));
        let prod = x.1.mul(y.1);
        let s = self.s(&diff);
        if self.s(&prod) == Ordering::Greater {
            s
        } else {
            s.reverse()
        }
    }

    /// `(x + y) / 2` as a homogeneous sigma.
    pub(crate) fn mid_sig(&self, x: &Sig<C>, y: &Sig<C>) -> Sig<C> {
        let num = x.0.mul(y.1).add(y.0.mul(x.1));
        let den = x.1.mul(y.1).scale(2);
        (num, den)
    }

    /// Does the homogeneous sigma equal the constant `c`?
    pub(crate) fn eq_sig_const(&self, x: &Sig<C>, c: &Nq<C>) -> bool {
        x.0.eq_x(c.mul(x.1))
    }

    /// All sigma events (vertex on the other side's edge LINE), as
    /// homogeneous sigmas grouped by equality, sorted exactly; plus
    /// the incidences that persist at EVERY sigma. (Spike
    /// `window_events`, division-free.)
    pub(crate) fn events(&self) -> (Vec<(Sig<C>, Vec<(bool, usize, usize)>)>, Vec<(bool, usize, usize)>) {
        let n = self.a.len();
        let mut raw: Vec<(Sig<C>, Vec<(bool, usize, usize)>)> = Vec::new();
        let mut persistent: Vec<(bool, usize, usize)> = Vec::new();
        for k in 0..n {
            for q in 0..n {
                let den = gcross(self.p, self.rf[q]);
                let num = gcross(gsub(self.a[k], self.rv[q]), self.rf[q]);
                if self.s(&den) == Ordering::Equal {
                    if self.s(&num) == Ordering::Equal {
                        persistent.push((true, k, q));
                    }
                    continue;
                }
                Self::push_event(&mut raw, (num, den), (true, k, q));
            }
        }
        for mm in 0..n {
            for p in 0..n {
                let den = gcross(self.p, self.us[p]);
                let num = gcross(gsub(self.rv[mm], self.a[p]), self.us[p]);
                if self.s(&den) == Ordering::Equal {
                    if self.s(&num) == Ordering::Equal {
                        persistent.push((false, mm, p));
                    }
                    continue;
                }
                Self::push_event(&mut raw, (num, den), (false, mm, p));
            }
        }
        // Exact sort (comparison by cross-multiplication) + equality
        // is already grouped by `push_event`.
        let mut idx: Vec<usize> = (0..raw.len()).collect();
        idx.sort_by(|&i, &j| self.cmp_sig(&raw[i].0, &raw[j].0));
        let sorted = idx.into_iter().map(|i| raw[i].clone()).collect();
        (sorted, persistent)
    }

    fn push_event(
        raw: &mut Vec<(Sig<C>, Vec<(bool, usize, usize)>)>,
        sig: Sig<C>,
        det: (bool, usize, usize),
    ) {
        // Group by exact sigma equality (cross-multiplication). The
        // event count is at most 2 n^2, so the linear scan is cheap
        // and avoids any canonicalization of the pair.
        for (s, dets) in raw.iter_mut() {
            if sig.0.mul(s.1).eq_x(s.0.mul(sig.1)) {
                dets.push(det);
                return;
            }
        }
        raw.push((sig, vec![det]));
    }

    /// The maximal open sigma-window around `sigma = 1` on which the
    /// pair state equals the pin state `pin_key` (the spike's
    /// `window_bracket`, division-free): the sorted event list, the
    /// bracket of `1` (no event AT `1`), the state re-check at the
    /// midpoint of the bracket, then the walk out in both directions —
    /// each stop is re-verified REAL by construction (the state at the
    /// midpoint just beyond differs from the pin state). Returns
    /// `(lo, hi, n_events, persistent)`.
    pub(crate) fn window(&self, pin_key: &StateKey) -> (Sig<C>, Sig<C>, usize, Vec<(bool, usize, usize)>) {
        let one = (Nq::<C>::ONE, Nq::<C>::ONE);
        let (evs, persistent) = self.events();
        // The bracket of sigma = 1: the last event below 1 and the
        // first event above it.
        let mut i0 = None;
        for (i, (s, _)) in evs.iter().enumerate() {
            match self.cmp_sig(s, &one) {
                Ordering::Less => i0 = Some(i),
                Ordering::Equal => panic!("rot census: event AT sigma = 1"),
                Ordering::Greater => break,
            }
        }
        let i0 = i0.expect("rot census: sigma = 1 not bracketed from below");
        assert!(i0 + 1 < evs.len(), "rot census: sigma = 1 not bracketed from above");
        assert_eq!(
            &self.state(&self.mid_sig(&evs[i0].0, &one)),
            pin_key,
            "rot census: state changes inside the bracket containing 1"
        );
        let mut lo_i = i0;
        while lo_i >= 1
            && &self.state(&self.mid_sig(&evs[lo_i - 1].0, &evs[lo_i].0)) == pin_key
        {
            lo_i -= 1;
        }
        let mut hi_i = i0 + 1;
        while hi_i + 1 < evs.len()
            && &self.state(&self.mid_sig(&evs[hi_i].0, &evs[hi_i + 1].0)) == pin_key
        {
            hi_i += 1;
        }
        // The boundary events are real: one step further out, the
        // state differs (asserted where a next event exists; the
        // bracket walk stopped exactly because the next midpoint
        // differs or the list ends).
        (evs[lo_i].0, evs[hi_i].0, evs.len(), persistent)
    }
}
