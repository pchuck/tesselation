//! Exact cyclotomic field cores for the n-fold tiling families.
//!
//! Design doc: `docs/cyclotomic-field-layer.md` — §2 (element
//! representation + reduction tables), §4 (the oblique U(k) frame and
//! the companion rotation R = [[0,−1],[1,b]]), §5 (exact sign/order as
//! the generalization of the quadratic `zcmp`), §7 (this module's
//! implementation plan). All pasted constants (minpolies, reduction
//! rows, isolating intervals, battery element lists) were *derived and
//! Sturm-certified* by the Python spike
//! (`.scratch/cyclotomic-design/spike.py`, scratch `derive_k7.py`) per
//! the doc's anti-hardcoding rule: derivation and certification live
//! Python-side; the Rust layer carries only certified constants and
//! re-asserts them against floats in tests.
//!
//! # Layers
//!
//! * **Generic field arithmetic over Q(b)** — free functions const-generic
//!   in the degree `D` so K₉/K₁₅ later land as ~40-line const+test
//!   additions (doc §7): [`kadd`]/[`ksub`]/[`kneg`]/[`kscale`]/[`kmul`]
//!   (convolution + reduction-row fold), [`ksign`] (exact sign via
//!   interval arithmetic over the isolating interval of b), [`knorm`]
//!   (exact norm = Sylvester resultant).
//! * **[`K7`]** — the first field core: K₇ = Q(b), b = 2cos(2π/7),
//!   degree 3, basis (1, b, b²) at global denominator 1.
//! * **[`P7`]** — the oblique frame point type (doc §4.1):
//!   p = A·U₀ + B·U₁ with exact norm², dot, sine-free cross sign, and
//!   companion rotation.
//!
//! No family consumes this module yet (doc §7 sequencing: consumer
//! families are a later step); the `Tiling` contract stays f64 at the
//! boundary — exact layers live *inside* generators (girih precedent),
//! and f64 appears in this module only in float cross-check tests,
//! never on the exact path.
//!
//! # Why the sign engine looks like this
//!
//! The shipped quadratic `zcmp` trick (sign-switch on `p² − 3q²`) does
//! **not** generalize: with d ≥ 3 conjugates, `sign(N(x)) ⇏ sign(x)`
//! (K₇ has elements of mixed conjugate signs — doc §5). The general
//! engine below is: exact zero test (all-zero tuple — equality is never
//! numeric), then interval-Horner evaluation of the element's
//! polynomial over the *Sturm isolating interval* of b (the rightmost
//! conjugate), bisecting on undecided. Termination is continuity, not a
//! separation constant: x(b) ≠ 0 and width → 0 forces a strict sign.

#![allow(dead_code)] // no consumer family yet (doc §7 step 3 is a later
                     // brief); every item is part of the intended API for
                     // K₉/K₁₅ and the first Kₙ family (girih_qc carries the
                     // same per-item allowances for unconsumed tables).

use std::cmp::Ordering;

// =====================================================================
// Generic layer — free functions over (degree, minpoly, isolating
// interval), so a new field is consts + tests (doc §7)
// =====================================================================

/// A dyadic rational interval `[lo_num/2^s, hi_num/2^s]` isolating a
/// field's generator b (its rightmost conjugate), together with the
/// per-field facts the sign engine's bisection relies on — all
/// certified Python-side by exact Sturm counts (scratch `derive_k7.py`)
/// and re-asserted exactly by tests:
///
/// * exactly one (simple) root of the minpoly lies in the interval;
/// * the minpoly is strictly *monotone* on the interval
///   (`m_increasing` fixes the direction) — for K₇: m′ = 3x² + 2x − 2
///   is root-free on [1, 3/2] with m′(1) = 3 > 0;
/// * m(lo) < 0 < m(hi) exactly, which (with monotonicity and m(b) = 0)
///   pins b strictly inside without any numerics.
///
/// Dyadic endpoints keep every rational the engine touches in the
/// hand-rolled (i64 numerator, 2^s denominator) form — bisection
/// preserves dyadicity, so no general rational pairs are needed and no
/// crate dependencies enter (brief constraint).
pub struct Iso {
    /// interval lower endpoint, over the common denominator 2^`s`
    pub lo_num: i64,
    /// interval upper endpoint, over the common denominator 2^`s`
    pub hi_num: i64,
    /// common denominator exponent: endpoints are lo_num/2^s, hi_num/2^s
    pub s: i32,
    /// certificate: m is strictly increasing (true) / decreasing (false)
    /// on the whole interval — decides which half of a bisection
    /// contains the root from the exact sign of m(midpoint)
    pub m_increasing: bool,
}

/// Coefficient-wise addition in the power basis (low→high tuples).
#[inline]
pub fn kadd<const D: usize>(x: [i64; D], y: [i64; D]) -> [i64; D] {
    std::array::from_fn(|i| x[i] + y[i])
}

/// Coefficient-wise subtraction.
#[inline]
pub fn ksub<const D: usize>(x: [i64; D], y: [i64; D]) -> [i64; D] {
    std::array::from_fn(|i| x[i] - y[i])
}

/// Coefficient-wise negation.
#[inline]
pub fn kneg<const D: usize>(x: [i64; D]) -> [i64; D] {
    x.map(|c| -c)
}

/// Coefficient-wise scaling by an integer (exact).
#[inline]
pub fn kscale<const D: usize>(x: [i64; D], c: i64) -> [i64; D] {
    x.map(|k| k * c)
}

/// Exact coefficient-wise halving; panics when any coefficient is odd
/// (the half leaves Z[b] and the D = 1 core cannot represent it — use
/// the factor-2 form of the quantity instead, e.g. [`P7::dot2`]).
fn khalf<const D: usize>(x: [i64; D]) -> [i64; D] {
    assert!(
        x.iter().all(|&c| c % 2 == 0),
        "field half: result leaves Z[b] (odd coefficient) — use the factor-2 form"
    );
    x.map(|c| c / 2)
}

/// Multiplication in Q(b): the raw convolution (degrees 0..2D−2) is
/// folded back into the basis with the reduction rows `red`, where
/// `red[j − D]` holds the certified basis coordinates of b^j
/// (b^D = −(m_{D−1}b^{D−1} + … + m₀) from the minpoly, then multiply by
/// b and fold — the girih `gexact.py` fmul pattern generalized, doc
/// §2.1). Products accumulate in i128; an overflowing result
/// coefficient is a hard error (operating-range note on [`ksign`]).
pub fn kmul<const D: usize>(x: [i64; D], y: [i64; D], red: &[[i64; D]]) -> [i64; D] {
    let mut raw = vec![0i128; 2 * D - 1];
    for i in 0..D {
        if x[i] == 0 {
            continue;
        }
        for j in 0..D {
            raw[i + j] += x[i] as i128 * y[j] as i128;
        }
    }
    std::array::from_fn(|i| {
        let mut c = raw[i];
        for j in 0..D - 1 {
            c += raw[D + j] * red[j][i] as i128;
        }
        i64::try_from(c).expect("field mul: coefficient overflow (operating-range note on ksign)")
    })
}

/// Exact sign of x(b) as an [`Ordering`] (`Equal` ⟺ x = 0 exactly).
///
/// The engine (doc §5): zero is decided on the tuple alone — never
/// numerically (`deg x < d < deg m`, m irreducible ⇒ x(b) = 0 ⇔ x ≡ 0).
/// Otherwise the element's polynomial p(X) = ΣxᵢXⁱ is evaluated by
/// exact interval-Horner (spike `_ieval`, dyadic arithmetic) over the
/// isolating interval; if the value interval straddles 0 the interval
/// is bisected, always keeping the half containing b — decided by the
/// *exact* sign of m(mid) together with the certified strict
/// monotonicity of m on the interval (m(mid) = 0 is impossible: mid is
/// rational, m is irreducible of degree ≥ 2). Continuity terminates the
/// loop because p(b) ≠ 0 for a nonzero element.
///
/// # Operating range
///
/// The pasted K₇ interval (width 2⁻³¹) decides in one pass for element
/// coefficients |cᵢ| ≲ 176, and the `BISECT_CAP` bisections (each
/// buys a factor 2^{1/3} of coefficient headroom, and each i128 Horner
/// step stays exact) cover |cᵢ| ≲ 1400 — far above anything the frame
/// and walk geometry of doc §4 produces (battery maximum |cᵢ| = 6,
/// walk/shoelace heights ≤ 8). Beyond that this panics loudly rather
/// than wrapping; taller heights want a wider `Iso` (per-field const)
/// or the doc §2 Tier-R rational layer.
pub fn ksign<const D: usize>(x: [i64; D], m: &[i64], iso: &Iso) -> Ordering {
    if x.iter().all(|&c| c == 0) {
        return Ordering::Equal;
    }
    let (mut lo, mut hi, mut s) = (iso.lo_num, iso.hi_num, iso.s);
    for _ in 0..BISECT_CAP {
        let (vl, vh) = interval_horner(&x, lo, hi, s);
        if vh < 0 {
            return Ordering::Less;
        }
        if vl > 0 {
            return Ordering::Greater;
        }
        // Undecided: bisect, keeping the half that contains b.
        let (mnum, ms) = mid_num(lo, hi, s);
        let msign = poly_sign_at(m, mnum, ms);
        assert!(
            msign != 0,
            "ksign: m(mid) == 0 — isolating-interval certificate broken"
        );
        if (msign > 0) == iso.m_increasing {
            hi = mnum;
        } else {
            lo = mnum;
        }
        s = ms;
    }
    panic!(
        "ksign: sign unresolved after {BISECT_CAP} bisections — \
         coefficient height exceeds the documented operating range"
    );
}

/// Bisection cap of the sign engine (see the operating-range note).
const BISECT_CAP: usize = 10;

/// Interval-Horner evaluation of the polynomial with coefficients `c`
/// (low→high) over the dyadic interval [lo/2^s, hi/2^s]: returns an
/// outer approximation (vl, vh)/2^{(deg)·s} of the value range — the
/// exact dyadic form of the spike's `_ieval` (spike.py L237–243).
fn interval_horner(c: &[i64], lo: i64, hi: i64, s: i32) -> (i128, i128) {
    let (mut vl, mut vh) = (0i128, 0i128);
    let mut sa = 0i32; // accumulator's common denominator is 2^sa
    for i in (0..c.len()).rev() {
        if sa == 0 {
            (vl, vh) = (c[i] as i128, c[i] as i128);
        } else {
            let prods = [
                vl * lo as i128,
                vl * hi as i128,
                vh * lo as i128,
                vh * hi as i128,
            ];
            let mn = prods.into_iter().min().unwrap();
            let mx = prods.into_iter().max().unwrap();
            let add = (c[i] as i128) << sa;
            vl = mn + add;
            vh = mx + add;
        }
        sa += s;
    }
    (vl, vh)
}

/// Exact sign of m(num/2^s) by dyadic Horner — integer arithmetic only
/// (degree read from the slice length).
fn poly_sign_at(m: &[i64], num: i64, s: i32) -> i32 {
    let deg = m.len() - 1;
    let mut acc = m[deg] as i128; // leading coefficient
    let mut sa = 0i32;
    for i in (0..deg).rev() {
        acc = acc * num as i128 + ((m[i] as i128) << (sa + s));
        sa += s;
    }
    acc.signum() as i32
}

/// Midpoint of the dyadic interval [lo/2^s, hi/2^s], kept on the grid.
fn mid_num(lo: i64, hi: i64, s: i32) -> (i64, i32) {
    let sum = lo + hi;
    if sum & 1 == 0 {
        (sum / 2, s)
    } else {
        (sum, s + 1)
    }
}

/// Exact norm N(x) = Res(m, x): the Sylvester determinant of the monic
/// minpoly m (low→high) and the element x, built leading-coefficient-
/// leftmost and evaluated by fraction-free Bareiss elimination in i128
/// (spike `sylvester_det`, L246–267; porting note in doc §9). N(x) = 0
/// ⟺ x = 0, and N(x) is a nonzero integer otherwise — the certificate
/// behind the sign engine's termination argument (doc §5.3).
pub fn knorm(m: &[i64], x: &[i64]) -> i128 {
    let n1 = m.len() - 1;
    let n2 = (0..x.len()).rev().find(|&i| x[i] != 0); // deg f after trimming
    let Some(n2) = n2 else { return 0 };
    let size = n1 + n2;
    let mut rows: Vec<Vec<i128>> = Vec::with_capacity(size);
    for i in 0..n2 {
        let mut row = vec![0i128; size];
        for (j, &c) in m.iter().enumerate() {
            row[i + (n1 - j)] = c as i128;
        }
        rows.push(row);
    }
    for i in 0..n1 {
        let mut row = vec![0i128; size];
        for (j, &c) in x.iter().take(n2 + 1).enumerate() {
            row[i + (n2 - j)] = c as i128;
        }
        rows.push(row);
    }
    det_bareiss(&mut rows)
}

/// Fraction-free (Bareiss) exact determinant in i128; every division is
/// exact by the Bareiss identity.
fn det_bareiss(rows: &mut [Vec<i128>]) -> i128 {
    let n = rows.len();
    let mut sign = 1i128;
    let mut prev = 1i128;
    for k in 0..n.saturating_sub(1) {
        if rows[k][k] == 0 {
            let Some(p) = (k + 1..n).find(|&r| rows[r][k] != 0) else {
                return 0;
            };
            rows.swap(k, p);
            sign = -sign;
        }
        for r in k + 1..n {
            for c in k + 1..n {
                rows[r][c] = (rows[r][c] * rows[k][k] - rows[r][k] * rows[k][c]) / prev;
            }
            rows[r][k] = 0;
        }
        prev = rows[k][k];
    }
    sign * rows[n - 1][n - 1]
}

// =====================================================================
// K7 — Q(b), b = 2cos(2π/7); degree 3; basis (1, b, b²), denominator 1
// =====================================================================

/// Float value of the generator: b = 2cos(2π/7) (cross-check/emission
/// use only — the exact path never reads a float).
const B_F: f64 = 1.246979603717467;
/// Float value of sin(2π/7) — the global positive factor that the
/// cross-sign trick divides out (cross-check/emission use only).
const SIN_F: f64 = 0.7818314824680298;

/// An element of K₇ = Q(b), b = 2cos(2π/7): the tuple `K7([c0, c1, c2])`
/// is the field element c0 + c1·b + c2·b² at global denominator 1
/// (doc §2 Tier E — all frame/walk geometry of doc §4 is integral in
/// Z[b], spike-verified, so the D = 1 core carries it).
///
/// Minpoly (Sturm-certified, doc §2): m(x) = x³ + x² − 2x − 1, whence
/// the reduction rows [`RED7`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct K7(pub [i64; 3]);

/// Minpoly of b, low→high: x³ + x² − 2x − 1.
pub const M7: [i64; 4] = [-1, -2, 1, 1];

/// Reduction rows (doc §2.1 table, Sturm-certified Python-side):
/// basis coordinates of b³ = 1 + 2b − b² and b⁴ = −1 − b + 3b².
pub const RED7: [[i64; 3]; 2] = [[1, 2, -1], [-1, -1, 3]];

/// Isolating interval of b (the rightmost conjugate): the consecutive
/// dyadics [2677868308/2³¹, 2677868309/2³¹] ≈ [1.246979603543878,
/// 1.246979604009539], strictly containing b ≈ 1.246979603717467.
/// Certified Python-side (scratch `derive_k7.py`, spike machinery):
/// exactly one root of m in (1, 3/2] (exact Sturm count); m′ = 3x²+2x−2
/// root-free on [1, 3/2] with m′(1) = 3 > 0, so m is strictly
/// increasing on the whole interval — the fact [`ksign`]'s bisection
/// consumes; and m(lo) < 0 < m(hi) exactly. The bracket and the float
/// cross-checks are re-asserted by the Rust tests.
pub const ISO7: Iso = Iso {
    lo_num: 2677868308,
    hi_num: 2677868309,
    s: 31,
    m_increasing: true,
};

/// Float value of a K7 element (cross-check/emission use only; all
/// correctness comparisons are exact tuple tests — girih_qc `kval`
/// precedent).
#[inline]
pub fn kval(x: [i64; 3]) -> f64 {
    x[0] as f64 + x[1] as f64 * B_F + x[2] as f64 * (B_F * B_F)
}

impl K7 {
    /// The zero element.
    pub const ZERO: K7 = K7([0, 0, 0]);
    /// The unit element.
    pub const ONE: K7 = K7([1, 0, 0]);
    /// The generator b = 2cos(2π/7).
    pub const B: K7 = K7([0, 1, 0]);

    /// Exact zero test — equality is never numeric (doc §5.1).
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == [0, 0, 0]
    }

    /// Field addition.
    #[inline]
    pub fn add(self, y: K7) -> K7 {
        K7(kadd(self.0, y.0))
    }

    /// Field subtraction.
    #[inline]
    pub fn sub(self, y: K7) -> K7 {
        K7(ksub(self.0, y.0))
    }

    /// Field negation.
    #[inline]
    pub fn neg(self) -> K7 {
        K7(kneg(self.0))
    }

    /// Scaling by an integer (exact).
    #[inline]
    pub fn scale(self, c: i64) -> K7 {
        K7(kscale(self.0, c))
    }

    /// Field multiplication (convolution + [`RED7`] fold).
    #[inline]
    pub fn mul(self, y: K7) -> K7 {
        K7(kmul(self.0, y.0, &RED7))
    }

    /// Exact sign — the isolating-interval engine ([`ksign`] with the
    /// K₇ constants).
    pub fn sign(&self) -> Ordering {
        ksign(self.0, &M7, &ISO7)
    }

    /// Exact norm N(x) = Res(m, x) (Sylvester/Bareiss, i128).
    pub fn norm(&self) -> i128 {
        knorm(&M7, &self.0)
    }

    /// Float value — cross-check and emission use only.
    #[inline]
    pub fn float(&self) -> f64 {
        kval(self.0)
    }
}

// =====================================================================
// The oblique frame (doc §4): points of M₇ = K₇U₀ ⊕ K₇U₁
// =====================================================================

/// A point p = A·U₀ + B·U₁ in the oblique frame (doc §4.1): U₀ = (1,0),
/// U₁ = the unit vector at angle 2π/7, with U₀·U₁ = b/2. The pair
/// (A, B) ∈ K₇² is the entire runtime state: dot products, norms and
/// orientation tests never leave K₇, because crosses differ from K₇
/// numbers by the *global positive* factor sin(2π/7) > 0 — the
/// sine-free cross-sign trick that makes the missing sine harmless.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct P7(pub K7, pub K7);

impl P7 {
    /// The origin.
    pub const ORIGIN: P7 = P7(K7::ZERO, K7::ZERO);

    /// Vertex translation (running edge-walk sums).
    #[inline]
    pub fn add(self, q: P7) -> P7 {
        P7(self.0.add(q.0), self.1.add(q.1))
    }

    /// Difference vector.
    #[inline]
    pub fn sub(self, q: P7) -> P7 {
        P7(self.0.sub(q.0), self.1.sub(q.1))
    }

    /// |p|² = A² + B² + A·B·b — exact in K₇ (doc §4.1 [P]).
    #[inline]
    pub fn norm2(&self) -> K7 {
        let (a, b) = (self.0, self.1);
        a.mul(a).add(b.mul(b)).add(a.mul(b).mul(K7::B))
    }

    /// 2·p·q = 2A·A′ + 2B·B′ + (A·B′ + A′·B)·b — the in-core primitive
    /// for the oblique bilinear form: p·q itself carries a ½ on the
    /// mixed term (U₀·U₁ = b/2), so the always-exact form carries the
    /// factor 2 ([`P7::dot`] halves it when the result is integral).
    #[inline]
    pub fn dot2(&self, q: P7) -> K7 {
        let (a, b) = (self.0, self.1);
        let (ap, bp) = (q.0, q.1);
        a.mul(ap)
            .scale(2)
            .add(b.mul(bp).scale(2))
            .add(a.mul(bp).add(ap.mul(b)).mul(K7::B))
    }

    /// p·q = A·A′ + B·B′ + (A·B′ + A′·B)·b/2 — exact when the half stays
    /// in Z[b] (all U(k)·U(k) products do); panics otherwise, pointing
    /// at [`P7::dot2`], which is always exact.
    #[inline]
    pub fn dot(&self, q: P7) -> K7 {
        K7(khalf(self.dot2(q).0))
    }

    /// p × q up to the global positive factor sin(2π/7): the K₇ element
    /// A·B′ − A′·B (doc §4.1 [P] — the sine-free orientation trick).
    #[inline]
    pub fn cross(&self, q: P7) -> K7 {
        self.0.mul(q.1).sub(q.0.mul(self.1))
    }

    /// Exact orientation sign of q seen from p (sign of [`P7::cross`]):
    /// `Greater` = counter-clockwise, `Less` = clockwise, `Equal` =
    /// collinear (an exact zero test, doc §4.3).
    #[inline]
    pub fn cross_sign(&self, q: P7) -> Ordering {
        self.cross(q).sign()
    }

    /// One companion-rotation step R(A, B) = (−B, A + b·B) — rotation of
    /// the point by 2π/7 about the origin (doc §4.2 [P]; det R = 1,
    /// trace R = b, and R^k is an exact K₇^{2×2} rotation for every k).
    #[inline]
    pub fn rot(&self) -> P7 {
        P7(self.1.neg(), self.0.add(self.1.mul(K7::B)))
    }

    /// Rotation by 2πk/7: k companion steps (k taken mod 7).
    pub fn rot_by(&self, k: usize) -> P7 {
        let mut p = *self;
        for _ in 0..k % 7 {
            p = p.rot();
        }
        p
    }

    /// Float coordinates (A + B·cos(2π/7), B·sin(2π/7)) — emission into
    /// the f64 `Tiling` boundary and float cross-checks only (doc §7:
    /// exact tuples → f64 once at emission).
    #[inline]
    pub fn float(&self) -> (f64, f64) {
        let (af, bf) = (self.0.float(), self.1.float());
        (af + bf * (B_F / 2.0), bf * SIN_F)
    }
}

// =====================================================================
// Test battery — mirrors the spike battery per doc §7 / the brief:
// (1) minpoly zero + float cross-checks, (2) frame, (3) star walks,
// (4) sign/order, (5) norm identities; plus the isolating-interval
// bracket certificate. f64 appears only here.
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    const TWO_PI_OVER_7: f64 = 2.0 * PI / 7.0;

    /// U(k) via the Chebyshev recurrence U(k+1) = b·U(k) − U(k−1) in the
    /// oblique coordinates (doc §4.2 [P]; spike `Frame.U_table`) — the
    /// construction the companion-rotation test is checked *against*
    /// (two independent paths to the same frame).
    fn u_table() -> Vec<P7> {
        let b = K7::B;
        let mut u = vec![P7(K7::ONE, K7::ZERO), P7(K7::ZERO, K7::ONE)];
        for _ in 2..7 {
            let (p1, p0) = (u[u.len() - 1], u[u.len() - 2]);
            u.push(P7(p1.0.mul(b).sub(p0.0), p1.1.mul(b).sub(p0.1)));
        }
        u
    }

    /// (1) m(b) == 0 exactly on the tuple model; the pasted reduction
    /// rows are exactly b³ and b⁴ under [`kmul`]; float cross-checks of
    /// the pasted constants (spike: reductions ≤ 1e-12, b ≤ 1e-15).
    #[test]
    fn minpoly_zero_on_tuple_model_and_float() {
        let b = K7::B;
        let b2 = b.mul(b);
        assert_eq!(b2, K7([0, 0, 1]));
        let b3 = b2.mul(b);
        assert_eq!(b3, K7(RED7[0]), "b³ differs from the pasted reduction row");
        let b4 = b3.mul(b);
        assert_eq!(b4, K7(RED7[1]), "b⁴ differs from the pasted reduction row");
        // m(b) = b³ + b² − 2b − 1 == 0 exactly
        let mb = b3.add(b2).sub(b.scale(2)).sub(K7::ONE);
        assert_eq!(mb, K7::ZERO, "m(b) != 0 on the tuple model");
        // float cross-checks
        assert!(
            (B_F - 2.0 * TWO_PI_OVER_7.cos()).abs() < 1e-15,
            "B_F constant"
        );
        assert!(
            (SIN_F - TWO_PI_OVER_7.sin()).abs() < 1e-15,
            "SIN_F constant"
        );
        assert!((kval(RED7[0]) - B_F.powi(3)).abs() < 1e-12, "b³ row float");
        assert!((kval(RED7[1]) - B_F.powi(4)).abs() < 1e-12, "b⁴ row float");
        assert!(
            (B_F.powi(3) + B_F * B_F - 2.0 * B_F - 1.0).abs() < 1e-12,
            "minpoly float evaluation"
        );
    }

    /// Sign-engine certificate, exact: with m strictly increasing on
    /// ISO7 (certified Python-side) and m(b) = 0, opposite exact signs
    /// of m at the endpoints pin b strictly inside — no numerics. Then
    /// the float sanity of the pasted dyadic endpoints.
    #[test]
    fn iso_interval_brackets_b_exactly() {
        assert_eq!(poly_sign_at(&M7, ISO7.lo_num, ISO7.s), -1, "m(lo) < 0");
        assert_eq!(poly_sign_at(&M7, ISO7.hi_num, ISO7.s), 1, "m(hi) > 0");
        let den = (1u64 << ISO7.s) as f64;
        let (lof, hif) = (ISO7.lo_num as f64 / den, ISO7.hi_num as f64 / den);
        assert!(lof < B_F && B_F < hif, "float b not inside the interval");
        assert!(hif - lof <= 5e-10, "interval unexpectedly wide");
        // smoke: the engine signs the obvious elements
        assert_eq!(K7::ZERO.sign(), Ordering::Equal);
        assert_eq!(K7::ONE.sign(), Ordering::Greater);
        assert_eq!(K7::B.sign(), Ordering::Greater);
        assert_eq!(K7::B.neg().sign(), Ordering::Less);
    }

    /// (2) |U(k)|² == 1 exactly for all k, plus the dot machinery on
    /// frame vectors: dot(U(k), U(k)) == 1 and a concrete dot2 value
    /// (2·cos(4π/7) = b² − 2).
    #[test]
    fn frame_unit_norms() {
        let u = u_table();
        for (k, p) in u.iter().enumerate() {
            assert_eq!(p.norm2(), K7::ONE, "|U({k})|² != 1");
            assert_eq!(p.dot(*p), K7::ONE, "U({k})·U({k}) != 1");
        }
        assert_eq!(u[2].dot2(u[0]), K7([-2, 0, 1]), "2·U(2)·U(0) = b² − 2");
    }

    /// (2) Σ_k U(k) == 0 exactly in both coordinates (spike L519–521).
    #[test]
    fn frame_sums_to_zero() {
        let u = u_table();
        let sa = u.iter().fold(K7::ZERO, |acc, p| acc.add(p.0));
        let sb = u.iter().fold(K7::ZERO, |acc, p| acc.add(p.1));
        assert_eq!(sa, K7::ZERO, "Σ A-coordinates != 0");
        assert_eq!(sb, K7::ZERO, "Σ B-coordinates != 0");
    }

    /// (2) R^k U(0) == U(k) and R^k U(1) == U(k+1) for all k — the
    /// companion matrix reproduces the recurrence-built frame (spike
    /// L545–548); rot_by(7) is the identity (U(k+7) = U(k)).
    #[test]
    fn companion_rotation_consistency() {
        let u = u_table();
        for k in 0..7 {
            assert_eq!(u[0].rot_by(k), u[k], "R^{k} U(0) != U({k})");
            assert_eq!(
                u[1].rot_by(k),
                u[(k + 1) % 7],
                "R^{k} U(1) != U({})",
                (k + 1) % 7
            );
        }
        assert_eq!(u[2].rot_by(7), u[2], "U(k+7) != U(k)");
    }

    /// (2) the exact frame lands on the float directions (spike L523–527,
    /// tolerance 1e-12).
    #[test]
    fn frame_matches_float_directions() {
        for (k, p) in u_table().iter().enumerate() {
            let (x, y) = p.float();
            let (sy, cx) = (TWO_PI_OVER_7 * k as f64).sin_cos();
            assert!(
                (x - cx).abs() < 1e-12 && (y - sy).abs() < 1e-12,
                "U({k}) float direction off: ({x}, {y}) vs ({cx}, {sy})"
            );
        }
    }

    /// (3) unit-edge star walk {7/s}: vertices P_j = Σ_{i<j} U(s·i mod 7).
    fn star_walk(s: usize) -> Vec<P7> {
        let u = u_table();
        let mut p = P7::ORIGIN;
        let mut pts = vec![p];
        for i in 0..7 {
            p = p.add(u[(s * i) % 7]);
            pts.push(p);
        }
        pts
    }

    /// Shared {7/s} battery (spike L552–593): exact closure; the
    /// centered walk C_j = P_j − G is R^s-equivariant (G = exact
    /// centroid, R^s G = G − shifted-U(0) proven in the spike); carried
    /// in the D = 1 core as Ĉ_j = 7·P_j − S (S = ΣP_j), which scales the
    /// equivariance without leaving Z[b]; edge directions advance by
    /// exactly one R-step; the shoelace factor Σ cross(P_j, P_{j+1}) is
    /// an exact K₇ constant (2·area/sin(2π/7)); float area agreement.
    fn star_walk_case(s: usize, shoelace: K7) {
        let u = u_table();
        let pts = &star_walk(s);
        assert_eq!(pts[7], P7::ORIGIN, "walk {{7/{s}}} does not close exactly");
        let seven = K7([7, 0, 0]);
        let sum = pts.iter().fold(P7::ORIGIN, |acc, p| acc.add(*p));
        let centered = |p: P7| P7(p.0.mul(seven).sub(sum.0), p.1.mul(seven).sub(sum.1));
        for j in 0..7 {
            assert_eq!(
                centered(pts[j]).rot_by(s),
                centered(pts[(j + 1) % 7]),
                "centered walk not R^{s}-equivariant at j = {j}"
            );
            let e = pts[(j + 1) % 7].sub(pts[j]);
            assert_eq!(
                e.rot_by(s),
                u[(s * (j + 1)) % 7],
                "edge direction not R^{s}-regular at j = {j}"
            );
        }
        let sho = (0..7).fold(K7::ZERO, |acc, j| acc.add(pts[j].cross(pts[(j + 1) % 7])));
        assert_eq!(
            sho, shoelace,
            "shoelace factor differs from the spike value"
        );
        let area = sho.float() * SIN_F / 2.0;
        let fp: Vec<(f64, f64)> = pts.iter().map(P7::float).collect();
        let farea: f64 = (0..7)
            .map(|j| fp[j].0 * fp[(j + 1) % 7].1 - fp[(j + 1) % 7].0 * fp[j].1)
            .sum::<f64>()
            / 2.0;
        assert!(
            (area.abs() - farea.abs()).abs() < 1e-9,
            "exact area {area} vs float area {farea}"
        );
    }

    /// (3) {7/1} — the regular heptagon walk; shoelace 4 + 3b + b².
    #[test]
    fn star_walk_7_1_regular_and_exact_shoelace() {
        star_walk_case(1, K7([4, 3, 1]));
    }

    /// (3) {7/3} — the far-star walk; shoelace −8 + b + 5b².
    #[test]
    fn star_walk_7_3_regular_and_exact_shoelace() {
        star_walk_case(3, K7([-8, 1, 5]));
    }

    /// (4) the spike's 29-element sign/order battery (spike L605–624),
    /// ported to the D = 1 integer core: the 24 seeded random elements
    /// verbatim (random.seed(7), coefficients in [−6, 6], low→high),
    /// then the fixed probes b+1, b²−3b, b⁴, and the integral stand-ins
    /// b²+1 (for the spike's 1/(b²+1)) and 3b (for b/3) — the spike's
    /// two non-integral elements are not in Z[b] (doc §2 Tier E);
    /// distinct floats + exact sign/order agreement re-certified in
    /// scratch `derive_k7.py`.
    const BATTERY: [K7; 29] = [
        K7([-1, -4, 0]),
        K7([4, -6, -5]),
        K7([2, -5, -1]),
        K7([3, -6, 2]),
        K7([-3, -6, -5]),
        K7([0, 0, -5]),
        K7([-3, -5, 2]),
        K7([0, -6, 3]),
        K7([-5, -3, 4]),
        K7([4, 3, -6]),
        K7([3, 3, 0]),
        K7([-6, -3, -6]),
        K7([2, -4, -2]),
        K7([0, -4, 2]),
        K7([-5, 3, -2]),
        K7([2, 4, -4]),
        K7([-5, 3, 3]),
        K7([4, -3, -1]),
        K7([-5, 2, 5]),
        K7([-5, 3, -6]),
        K7([3, -3, 1]),
        K7([4, 2, 0]),
        K7([6, -1, 1]),
        K7([3, 1, -1]),
        K7([1, 1, 0]),   // b + 1
        K7([0, -3, 1]),  // b² − 3b
        K7([-1, -1, 3]), // b⁴
        K7([1, 0, 1]),   // b² + 1   (stands in for 1/(b²+1))
        K7([0, 3, 0]),   // 3b       (stands in for b/3)
    ];

    /// (4) exact signs match float signs; floats distinct; the exact
    /// total order (comparator = sign(x − y), doc §5.4) matches the
    /// float sort.
    #[test]
    fn sign_order_battery_matches_float() {
        for x in BATTERY {
            let fx = x.float();
            assert!(fx.abs() > 1e-8, "battery element too close to 0: {x:?}");
            let want = if fx > 0.0 {
                Ordering::Greater
            } else {
                Ordering::Less
            };
            assert_eq!(x.sign(), want, "exact sign mismatch at {x:?} (float {fx})");
        }
        let mut fl = BATTERY;
        fl.sort_by(|a, b| a.float().partial_cmp(&b.float()).unwrap());
        for w in fl.windows(2) {
            assert!(w[0].float() < w[1].float(), "float collision in battery");
        }
        let mut exact = BATTERY;
        exact.sort_by(|a, b| a.sub(*b).sign());
        assert_eq!(exact, fl, "exact total order differs from the float sort");
    }

    /// (5) norm identities via the exact Sylvester resultant (spike
    /// L596–603): N(1) = 1, N(b) = (−1)³·m₀ = 1, multiplicativity on the
    /// integral probes b+1, b²−3b, b⁴ with y = x+1 (the spike's third
    /// probe 1/(b²+1) is not in Z[b]; its denominator element is in the
    /// sign battery instead).
    #[test]
    fn norm_identities_sylvester() {
        assert_eq!(K7::ONE.norm(), 1, "N(1) != 1");
        assert_eq!(
            K7::B.norm(),
            (-1i128).pow(3) * M7[0] as i128,
            "N(b) != (−1)³·m₀"
        );
        assert_eq!(K7::B.norm(), 1, "N(b) != 1");
        for x in [K7::B.add(K7::ONE), K7([0, -3, 1]), K7([-1, -1, 3])] {
            let y = x.add(K7::ONE);
            assert_eq!(
                x.mul(y).norm(),
                x.norm() * y.norm(),
                "N not multiplicative at {x:?}"
            );
        }
    }
}
