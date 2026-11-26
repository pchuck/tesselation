//! Exact cyclotomic field cores for the n-fold tiling families.
//!
//! Design doc: `docs/cyclotomic-field-layer.md` — §2 (element
//! representation + reduction tables), §4 (the oblique U(k) frame and
//! the companion rotation R = [[0,−1],[1,b]]), §5 (exact sign/order as
//! the generalization of the quadratic `zcmp`), §7 (this module's
//! implementation plan). All pasted constants (minpolies, reduction
//! rows, isolating intervals, battery element lists) were *derived and
//! Sturm-certified* by the Python spike
//! (`.scratch/cyclotomic-design/spike.py`, scratch `derive_k7.py` /
//! `derive_k9k15l10.py`) per
//! the doc's anti-hardcoding rule: derivation and certification live
//! Python-side; the Rust layer carries only certified constants and
//! re-asserts them against floats in tests.
//!
//! # Layers
//!
//! * **Generic field arithmetic over Q(b)** — free functions const-generic
//!   in the degree `D` so new fields land as const+test additions (doc
//!   §7): [`kadd`]/[`ksub`]/[`kneg`]/[`kscale`]/[`kmul`]
//!   (convolution + reduction-row fold), [`ksign`] (exact sign via
//!   interval arithmetic over the isolating interval of b), [`knorm`]
//!   (exact norm = Sylvester resultant), plus the generator-agnostic
//!   oblique-frame functions [`koblique_norm2`]/[`koblique_dot2`]/[
//!   `koblique_cross`]/[`koblique_rot`] (doc §4 over any field).
//! * **[`K7`]** — K₇ = Q(b), b = 2cos(2π/7), degree 3, basis (1, b, b²)
//!   at global denominator 1, with the oblique frame [`P7`] (doc §4.1).
//! * **[`K9`]/[`K15`]** — the degree-3 and degree-4 cores for the 9-fold
//!   and 15-fold families (same shape as [`K7`]), with frames [`P9`]/[
//!   `P15`]; K₁₅ carries the exact Q(√5) subfield probe b³ − 3b = 2cos(2π/5)
//!   (doc §3, tested below).
//! * **[`L10`]** — the girih field Q(a), a = 2cos(π/10): the shipped
//!   `families/islamic/girih_qc.rs` arithmetic (a⁴ = 5a² − 5 at /64)
//!   lifted onto the generic layer. Its anchor test rebuilds the 20
//!   shipped girih rotation matrices bit-for-bit at denominator /64 —
//!   the permanent regression anchor (doc §7).
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

// Unconsumed items below carry per-item `#[allow(dead_code)]` — the
// generic Kₙ layer and the K₉/K₁₅/L₁₀ cores (doc §7) are intended
// API for later briefs; glb_binary_n7 consumes the K₇ slice today.
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
#[allow(dead_code)]
pub fn kneg<const D: usize>(x: [i64; D]) -> [i64; D] {
    x.map(|c| -c)
}

/// Coefficient-wise scaling by an integer (exact).
#[inline]
#[allow(dead_code)]
pub fn kscale<const D: usize>(x: [i64; D], c: i64) -> [i64; D] {
    x.map(|k| k * c)
}

/// Exact coefficient-wise halving; panics when any coefficient is odd
/// (the half leaves Z[b] and the D = 1 core cannot represent it — use
/// the factor-2 form of the quantity instead, e.g. [`P7::dot2`]).
#[allow(dead_code)]
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
/// The engine never needs bisections in practice: the scratch
/// certification (`derive_k9k15l10.py`, exact simulation of this code)
/// resolves every sign/order battery element *and* every ordered
/// pairwise difference (the exact-sort comparator) of K₇/K₉/K₁₅ — 29
/// elements each, |cᵢ| ≤ 12 after subtraction — in a single pass over
/// the pasted intervals. The `BISECT_CAP` bisections are pure headroom
/// (each buys a factor 2^{1/D} of coefficient headroom, and each i128
/// Horner step stays exact) — far above anything the frame and walk
/// geometry of doc §4 produces (battery maximum |cᵢ| = 6, walk/shoelace
/// heights ≤ 90). Beyond that this panics loudly rather than wrapping;
/// taller heights want a wider `Iso` (per-field const) or the doc §2
/// Tier-R rational layer.
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
///
/// Why 4 and not more: the deg-4 fields (`K15`, `L10`) pin the i128
/// envelope. Every bisection of a consecutive-dyadic interval lifts the
/// accumulator's common denominator by one power of two, and the exact
/// `m(mid)` evaluation (`poly_sign_at`) accumulates
/// `m(num/2^s')·2^{deg·s'}` ≤ `2^{deg·s'}·B_m` with
/// `B_m = Σ|mᵢ|·2.1^i ≈ 49` for the K₁₅ minpoly — so
/// `s₀ + BISECT_CAP ≤ 30` keeps the worst case at
/// `2^{4·30}·49 ≈ 2^125.6 < 2^127` (certified in scratch; measured
/// peak `|acc| ≈ 2^85`). The deg-3 fields (K₇/K₉, s₀ = 31) stay far
/// inside the same cap (`2^{3·35}·17 ≈ 2^109`).
const BISECT_CAP: usize = 4;

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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
// The oblique frame over any field (doc §4) — generator-agnostic free
// functions. A field's point type is a thin wrapper (A, B) ∈ K²
// delegating to these with its own reduction rows (doc §7:
// "generator-agnostic parts are free functions over (d, minpoly, iso)").
// =====================================================================

/// The basis element b^`i` as a one-hot tuple (0 ≤ i < D).
#[inline]
#[allow(dead_code)]
pub fn kbasis<const D: usize>(i: usize) -> [i64; D] {
    std::array::from_fn(|j| if j == i { 1 } else { 0 })
}

/// |p|² = A² + B² + A·B·b for p = A·U₀ + B·U₁ — exact in Z[b] for every
/// field's oblique frame (doc §4.1 [P], the bilinear-form expansion:
/// U₀·U₁ = b/2, |U₀| = |U₁| = 1).
#[inline]
#[allow(dead_code)]
pub fn koblique_norm2<const D: usize>(a: [i64; D], b: [i64; D], red: &[[i64; D]]) -> [i64; D] {
    let ab = kmul(a, b, red);
    kadd(
        kadd(kmul(a, a, red), kmul(b, b, red)),
        kmul(ab, kbasis::<D>(1), red),
    )
}

/// 2·p·q = 2A·A′ + 2B·B′ + (A·B′ + A′·B)·b — the in-core primitive for
/// the oblique bilinear form: p·q itself carries a ½ on the mixed term
/// (U₀·U₁ = b/2), so the always-exact form carries the factor 2 (doc
/// §4.1; the halved [`P7::dot`]-style form is a thin `khalf` wrapper).
#[inline]
#[allow(dead_code)]
pub fn koblique_dot2<const D: usize>(
    a: [i64; D],
    b: [i64; D],
    ap: [i64; D],
    bp: [i64; D],
    red: &[[i64; D]],
) -> [i64; D] {
    let mixed = kadd(kmul(a, bp, red), kmul(ap, b, red));
    kadd(
        kadd(kscale(kmul(a, ap, red), 2), kscale(kmul(b, bp, red), 2)),
        kmul(mixed, kbasis::<D>(1), red),
    )
}

/// p × q up to the global positive factor sin(2π/n): the field element
/// A·B′ − A′·B (doc §4.1 — the sine-free orientation trick: sign of this
/// element is the orientation sign for every n, since sin(2π/n) > 0 for
/// n ≥ 3).
#[inline]
#[allow(dead_code)]
pub fn koblique_cross<const D: usize>(
    a: [i64; D],
    b: [i64; D],
    ap: [i64; D],
    bp: [i64; D],
    red: &[[i64; D]],
) -> [i64; D] {
    ksub(kmul(a, bp, red), kmul(ap, b, red))
}

/// One companion-rotation step R(A, B) = (−B, A + b·B) — rotation of the
/// point by 2π/n about the origin (doc §4.2 [P]: det R = 1, trace R = b,
/// and R^k is an exact K^{2×2} rotation for every k).
#[inline]
#[allow(dead_code)]
pub fn koblique_rot<const D: usize>(
    a: [i64; D],
    b: [i64; D],
    red: &[[i64; D]],
) -> ([i64; D], [i64; D]) {
    (kneg(b), kadd(a, kmul(kbasis::<D>(1), b, red)))
}

// =====================================================================
// K7 — Q(b), b = 2cos(2π/7); degree 3; basis (1, b, b²), denominator 1
// =====================================================================

/// Float value of the generator: b = 2cos(2π/7) (cross-check/emission
/// use only — the exact path never reads a float).
const B_F: f64 = 1.246979603717467;
/// Float value of sin(2π/7) — the global positive factor that the
/// cross-sign trick divides out (cross-check/emission use only).
#[allow(dead_code)]
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
pub fn kval7(x: [i64; 3]) -> f64 {
    x[0] as f64 + x[1] as f64 * B_F + x[2] as f64 * (B_F * B_F)
}

#[allow(dead_code)]
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
        kval7(self.0)
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
#[allow(dead_code)]
pub struct P7(pub K7, pub K7);

#[allow(dead_code)]
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
        K7(koblique_norm2(self.0 .0, self.1 .0, &RED7))
    }

    /// 2·p·q = 2A·A′ + 2B·B′ + (A·B′ + A′·B)·b — the in-core primitive
    /// for the oblique bilinear form: p·q itself carries a ½ on the
    /// mixed term (U₀·U₁ = b/2), so the always-exact form carries the
    /// factor 2 ([`P7::dot`] halves it when the result is integral).
    #[inline]
    pub fn dot2(&self, q: P7) -> K7 {
        K7(koblique_dot2(self.0 .0, self.1 .0, q.0 .0, q.1 .0, &RED7))
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
        K7(koblique_cross(self.0 .0, self.1 .0, q.0 .0, q.1 .0, &RED7))
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
        let (a, b) = koblique_rot(self.0 .0, self.1 .0, &RED7);
        P7(K7(a), K7(b))
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
// K9 — Q(b), b = 2cos(2π/9); degree 3; basis (1, b, b²), denominator 1
// =====================================================================

/// Float value of the generator: b = 2cos(2π/9) (cross-check/emission
/// use only — the exact path never reads a float).
#[allow(dead_code)]
const B9_F: f64 = 1.532088886237956;
/// Float value of sin(2π/9) — the global positive factor that the
/// cross-sign trick divides out (cross-check/emission use only).
#[allow(dead_code)]
const SIN9_F: f64 = 0.6427876096865393;

/// An element of K₉ = Q(b), b = 2cos(2π/9): the tuple `K9([c0, c1, c2])`
/// is the field element c0 + c1·b + c2·b² at global denominator 1
/// (doc §2 Tier E — the K₉ frame/walk geometry of doc §4 is integral in
/// Z[b], spike-verified, so the D = 1 core carries it).
///
/// Minpoly (Sturm-certified, doc §2.1): m(x) = x³ − 3x + 1, whence the
/// reduction rows [`RED9`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct K9(pub [i64; 3]);

/// Minpoly of b, low→high: x³ − 3x + 1.
#[allow(dead_code)]
pub const M9: [i64; 4] = [1, -3, 0, 1];

/// Reduction rows (doc §2.1 table, Sturm-certified Python-side):
/// basis coordinates of b³ = −1 + 3b and b⁴ = −b + 3b².
#[allow(dead_code)]
pub const RED9: [[i64; 3]; 2] = [[-1, 3, 0], [0, -1, 3]];

/// Isolating interval of b (the rightmost conjugate): the consecutive
/// dyadics [3290135830/2³¹, 3290135831/2³¹] ≈ [1.532088886015117,
/// 1.532088886480778], strictly containing b ≈ 1.532088886237956.
/// Certified Python-side (scratch `derive_k9k15l10.py`, spike
/// machinery): exactly one root of m in [3/2, 2] (exact Sturm count);
/// m′ = 3x² − 3 is root-free on [3/2, 2] (its roots ±1 lie below) with
/// m′(3/2) = 15/4 > 0, so m is strictly increasing on the whole
/// interval — the fact [`ksign`]'s bisection consumes; and
/// m(lo) < 0 < m(hi) exactly. The bracket and the float cross-checks
/// are re-asserted by the Rust tests.
#[allow(dead_code)]
pub const ISO9: Iso = Iso {
    lo_num: 3290135830,
    hi_num: 3290135831,
    s: 31,
    m_increasing: true,
};

/// Float value of a K9 element (cross-check/emission use only).
#[inline]
#[allow(dead_code)]
pub fn kval9(x: [i64; 3]) -> f64 {
    x[0] as f64 + x[1] as f64 * B9_F + x[2] as f64 * (B9_F * B9_F)
}

#[allow(dead_code)]
impl K9 {
    /// The zero element.
    pub const ZERO: K9 = K9([0, 0, 0]);
    /// The unit element.
    pub const ONE: K9 = K9([1, 0, 0]);
    /// The generator b = 2cos(2π/9).
    pub const B: K9 = K9([0, 1, 0]);

    /// Exact zero test — equality is never numeric (doc §5.1).
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == [0, 0, 0]
    }

    /// Field addition.
    #[inline]
    pub fn add(self, y: K9) -> K9 {
        K9(kadd(self.0, y.0))
    }

    /// Field subtraction.
    #[inline]
    pub fn sub(self, y: K9) -> K9 {
        K9(ksub(self.0, y.0))
    }

    /// Field negation.
    #[inline]
    pub fn neg(self) -> K9 {
        K9(kneg(self.0))
    }

    /// Scaling by an integer (exact).
    #[inline]
    pub fn scale(self, c: i64) -> K9 {
        K9(kscale(self.0, c))
    }

    /// Field multiplication (convolution + [`RED9`] fold).
    #[inline]
    pub fn mul(self, y: K9) -> K9 {
        K9(kmul(self.0, y.0, &RED9))
    }

    /// Exact sign — the isolating-interval engine ([`ksign`] with the
    /// K₉ constants).
    pub fn sign(&self) -> Ordering {
        ksign(self.0, &M9, &ISO9)
    }

    /// Exact norm N(x) = Res(m, x) (Sylvester/Bareiss, i128).
    pub fn norm(&self) -> i128 {
        knorm(&M9, &self.0)
    }

    /// Float value — cross-check and emission use only.
    #[inline]
    pub fn float(&self) -> f64 {
        kval9(self.0)
    }
}

/// A point p = A·U₀ + B·U₁ in the K₉ oblique frame (doc §4.1): U₀ = (1,0),
/// U₁ = the unit vector at angle 2π/9, with U₀·U₁ = b/2 — same exact
/// machinery as [`P7`], delegated to the generic frame functions over
/// [`RED9`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct P9(pub K9, pub K9);

#[allow(dead_code)]
impl P9 {
    /// The origin.
    pub const ORIGIN: P9 = P9(K9::ZERO, K9::ZERO);

    /// Vertex translation (running edge-walk sums).
    #[inline]
    pub fn add(self, q: P9) -> P9 {
        P9(self.0.add(q.0), self.1.add(q.1))
    }

    /// Difference vector.
    #[inline]
    pub fn sub(self, q: P9) -> P9 {
        P9(self.0.sub(q.0), self.1.sub(q.1))
    }

    /// |p|² = A² + B² + A·B·b — exact in K₉ (doc §4.1 [P]).
    #[inline]
    pub fn norm2(&self) -> K9 {
        K9(koblique_norm2(self.0 .0, self.1 .0, &RED9))
    }

    /// 2·p·q = 2A·A′ + 2B·B′ + (A·B′ + A′·B)·b — the always-exact form
    /// (see [`P7::dot2`]).
    #[inline]
    pub fn dot2(&self, q: P9) -> K9 {
        K9(koblique_dot2(self.0 .0, self.1 .0, q.0 .0, q.1 .0, &RED9))
    }

    /// p·q = A·A′ + B·B′ + (A·B′ + A′·B)·b/2 — exact when the half stays
    /// in Z[b] (all U(k)·U(k) products do); panics otherwise, pointing
    /// at [`P9::dot2`].
    #[inline]
    pub fn dot(&self, q: P9) -> K9 {
        K9(khalf(self.dot2(q).0))
    }

    /// p × q up to the global positive factor sin(2π/9) (doc §4.1 [P]).
    #[inline]
    pub fn cross(&self, q: P9) -> K9 {
        K9(koblique_cross(self.0 .0, self.1 .0, q.0 .0, q.1 .0, &RED9))
    }

    /// Exact orientation sign of q seen from p (doc §4.3).
    #[inline]
    pub fn cross_sign(&self, q: P9) -> Ordering {
        self.cross(q).sign()
    }

    /// One companion-rotation step — rotation by 2π/9 (doc §4.2 [P]).
    #[inline]
    pub fn rot(&self) -> P9 {
        let (a, b) = koblique_rot(self.0 .0, self.1 .0, &RED9);
        P9(K9(a), K9(b))
    }

    /// Rotation by 2πk/9: k companion steps (k taken mod 9).
    pub fn rot_by(&self, k: usize) -> P9 {
        let mut p = *self;
        for _ in 0..k % 9 {
            p = p.rot();
        }
        p
    }

    /// Float coordinates (A + B·cos(2π/9), B·sin(2π/9)) — emission and
    /// float cross-checks only.
    #[inline]
    pub fn float(&self) -> (f64, f64) {
        let (af, bf) = (self.0.float(), self.1.float());
        (af + bf * (B9_F / 2.0), bf * SIN9_F)
    }
}

// =====================================================================
// K15 — Q(b), b = 2cos(2π/15); degree 4; basis (1, b, b², b³), denom. 1
// =====================================================================

/// Float value of the generator: b = 2cos(2π/15) (cross-check/emission
/// use only — the exact path never reads a float).
#[allow(dead_code)]
const B15_F: f64 = 1.8270909152852017;
/// Float value of sin(2π/15) — the global positive factor that the
/// cross-sign trick divides out (cross-check/emission use only).
#[allow(dead_code)]
const SIN15_F: f64 = 0.40673664307580015;

/// An element of K₁₅ = Q(b), b = 2cos(2π/15): the tuple
/// `K15([c0, c1, c2, c3])` is the field element c0 + c1·b + c2·b² + c3·b³
/// at global denominator 1 (doc §2 Tier E — the K₁₅ frame/walk geometry
/// is integral in Z[b], spike-verified). K₁₅ ⊃ Q(√5): the subfield probe
/// b³ − 3b = 2cos(2π/5) is tested exactly below (doc §3).
///
/// Minpoly (Sturm-certified, doc §2.1): m(x) = x⁴ − x³ − 4x² + 4x + 1,
/// whence the reduction rows [`RED15`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct K15(pub [i64; 4]);

/// Minpoly of b, low→high: x⁴ − x³ − 4x² + 4x + 1.
#[allow(dead_code)]
pub const M15: [i64; 5] = [1, 4, -4, -1, 1];

/// Reduction rows (doc §2.1 table, Sturm-certified Python-side):
/// basis coordinates of b⁴ = −1 − 4b + 4b² + b³, b⁵ = −1 − 5b + 5b³ and
/// b⁶ = −5 − 21b + 15b² + 5b³.
#[allow(dead_code)]
pub const RED15: [[i64; 4]; 3] = [[-1, -4, 4, 1], [-1, -5, 0, 5], [-5, -21, 15, 5]];

/// Isolating interval of b (the rightmost conjugate): the consecutive
/// dyadics [122613995/2²⁶, 122613996/2²⁶] ≈ [1.827090904116631,
/// 1.827090919017792], strictly containing b ≈ 1.827090915285202.
/// Certified Python-side (scratch `derive_k9k15l10.py`, spike
/// machinery): exactly one root of m in [7/4, 2] (exact Sturm count);
/// m′ = 4x³ − 3x² − 8x + 4 is root-free there (its largest root ≈ 1.61
/// lies below) with m′(7/4) = 9/4 > 0, so m is strictly increasing on
/// the whole interval — the fact [`ksign`]'s bisection consumes; and
/// m(lo) < 0 < m(hi) exactly. The s₀ = 26 start is the i128 envelope of
/// the deg-4 fields (see [`BISECT_CAP`]). The bracket and the float
/// cross-checks are re-asserted by the Rust tests.
#[allow(dead_code)]
pub const ISO15: Iso = Iso {
    lo_num: 122613995,
    hi_num: 122613996,
    s: 26,
    m_increasing: true,
};

/// Float value of a K15 element (cross-check/emission use only).
#[inline]
#[allow(dead_code)]
pub fn kval15(x: [i64; 4]) -> f64 {
    x[0] as f64
        + x[1] as f64 * B15_F
        + x[2] as f64 * (B15_F * B15_F)
        + x[3] as f64 * (B15_F * B15_F * B15_F)
}

#[allow(dead_code)]
impl K15 {
    /// The zero element.
    pub const ZERO: K15 = K15([0, 0, 0, 0]);
    /// The unit element.
    pub const ONE: K15 = K15([1, 0, 0, 0]);
    /// The generator b = 2cos(2π/15).
    pub const B: K15 = K15([0, 1, 0, 0]);

    /// Exact zero test — equality is never numeric (doc §5.1).
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == [0, 0, 0, 0]
    }

    /// Field addition.
    #[inline]
    pub fn add(self, y: K15) -> K15 {
        K15(kadd(self.0, y.0))
    }

    /// Field subtraction.
    #[inline]
    pub fn sub(self, y: K15) -> K15 {
        K15(ksub(self.0, y.0))
    }

    /// Field negation.
    #[inline]
    pub fn neg(self) -> K15 {
        K15(kneg(self.0))
    }

    /// Scaling by an integer (exact).
    #[inline]
    pub fn scale(self, c: i64) -> K15 {
        K15(kscale(self.0, c))
    }

    /// Field multiplication (convolution + [`RED15`] fold).
    #[inline]
    pub fn mul(self, y: K15) -> K15 {
        K15(kmul(self.0, y.0, &RED15))
    }

    /// Exact sign — the isolating-interval engine ([`ksign`] with the
    /// K₁₅ constants).
    pub fn sign(&self) -> Ordering {
        ksign(self.0, &M15, &ISO15)
    }

    /// Exact norm N(x) = Res(m, x) (Sylvester/Bareiss, i128).
    pub fn norm(&self) -> i128 {
        knorm(&M15, &self.0)
    }

    /// Float value — cross-check and emission use only.
    #[inline]
    pub fn float(&self) -> f64 {
        kval15(self.0)
    }
}

/// A point p = A·U₀ + B·U₁ in the K₁₅ oblique frame (doc §4.1): U₀ = (1,0),
/// U₁ = the unit vector at angle 2π/15, with U₀·U₁ = b/2 — same exact
/// machinery as [`P7`], delegated to the generic frame functions over
/// [`RED15`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct P15(pub K15, pub K15);

#[allow(dead_code)]
impl P15 {
    /// The origin.
    pub const ORIGIN: P15 = P15(K15::ZERO, K15::ZERO);

    /// Vertex translation (running edge-walk sums).
    #[inline]
    pub fn add(self, q: P15) -> P15 {
        P15(self.0.add(q.0), self.1.add(q.1))
    }

    /// Difference vector.
    #[inline]
    pub fn sub(self, q: P15) -> P15 {
        P15(self.0.sub(q.0), self.1.sub(q.1))
    }

    /// |p|² = A² + B² + A·B·b — exact in K₁₅ (doc §4.1 [P]).
    #[inline]
    pub fn norm2(&self) -> K15 {
        K15(koblique_norm2(self.0 .0, self.1 .0, &RED15))
    }

    /// 2·p·q = 2A·A′ + 2B·B′ + (A·B′ + A′·B)·b — the always-exact form
    /// (see [`P7::dot2`]).
    #[inline]
    pub fn dot2(&self, q: P15) -> K15 {
        K15(koblique_dot2(self.0 .0, self.1 .0, q.0 .0, q.1 .0, &RED15))
    }

    /// p·q = A·A′ + B·B′ + (A·B′ + A′·B)·b/2 — exact when the half stays
    /// in Z[b] (all U(k)·U(k) products do); panics otherwise, pointing
    /// at [`P15::dot2`].
    #[inline]
    pub fn dot(&self, q: P15) -> K15 {
        K15(khalf(self.dot2(q).0))
    }

    /// p × q up to the global positive factor sin(2π/15) (doc §4.1 [P]).
    #[inline]
    pub fn cross(&self, q: P15) -> K15 {
        K15(koblique_cross(self.0 .0, self.1 .0, q.0 .0, q.1 .0, &RED15))
    }

    /// Exact orientation sign of q seen from p (doc §4.3).
    #[inline]
    pub fn cross_sign(&self, q: P15) -> Ordering {
        self.cross(q).sign()
    }

    /// One companion-rotation step — rotation by 2π/15 (doc §4.2 [P]).
    #[inline]
    pub fn rot(&self) -> P15 {
        let (a, b) = koblique_rot(self.0 .0, self.1 .0, &RED15);
        P15(K15(a), K15(b))
    }

    /// Rotation by 2πk/15: k companion steps (k taken mod 15).
    pub fn rot_by(&self, k: usize) -> P15 {
        let mut p = *self;
        for _ in 0..k % 15 {
            p = p.rot();
        }
        p
    }

    /// Float coordinates (A + B·cos(2π/15), B·sin(2π/15)) — emission and
    /// float cross-checks only.
    #[inline]
    pub fn float(&self) -> (f64, f64) {
        let (af, bf) = (self.0.float(), self.1.float());
        (af + bf * (B15_F / 2.0), bf * SIN15_F)
    }
}

// =====================================================================
// L10 — Q(a), a = 2cos(π/10) = 2cos(2π/20); degree 4; denominator 1.
// The girih field: this is exactly the shipped girih_qc arithmetic
// (a⁴ = 5a² − 5, elements at a global /64 denominator) lifted onto the
// generic layer — the anchor test below rebuilds girih_qc's 20 rotation
// matrices bit-for-bit through it (doc §7).
// =====================================================================

/// Float value of the generator: a = 2cos(π/10) (girih_qc's `A`;
/// cross-check/emission use only).
#[allow(dead_code)]
const A10_F: f64 = 1.902113032590307;

/// An element of L₁₀ = Q(a), a = 2cos(π/10): the tuple
/// `L10([c0, c1, c2, c3])` is the field element c0 + c1·a + c2·a² + c3·a³
/// at global denominator 1. (The girih family stores its elements at the
/// fixed denominator 64 — the anchor test reproduces that convention
/// locally; this core is the D = 1 carrier of the same field.)
///
/// Minpoly (Sturm-certified Python-side, spike `newton_minpoly(20)`):
/// m(x) = x⁴ − 5x² + 5, whence the reduction rows [`RED_L10`] — exactly
/// the shipped girih_qc `kmul` rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct L10(pub [i64; 4]);

/// Minpoly of a, low→high: x⁴ − 5x² + 5.
#[allow(dead_code)]
pub const ML10: [i64; 5] = [5, 0, -5, 0, 1];

/// Reduction rows — the shipped girih_qc kmul rule (doc §2.1 **[C]**,
/// bit-verified by the anchor test below): basis coordinates of
/// a⁴ = 5a² − 5, a⁵ = 5a³ − 5a and a⁶ = 20a² − 25.
#[allow(dead_code)]
pub const RED_L10: [[i64; 4]; 3] = [[-5, 0, 5, 0], [0, -5, 0, 5], [-25, 0, 20, 0]];

/// Isolating interval of a (the rightmost conjugate): the consecutive
/// dyadics [127648644/2²⁶, 127648645/2²⁶] ≈ [1.902113020420074,
/// 1.902113035321236], strictly containing a ≈ 1.902113032590307.
/// Certified Python-side (scratch `derive_k9k15l10.py`, spike
/// machinery): exactly one root of m in [7/4, 2] (exact Sturm count);
/// m′ = 4x³ − 10x is root-free there (its roots 0, ±√2.5 ≈ ±1.581 lie
/// below) with m′(7/4) = 63/16 > 0, so m is strictly increasing on the
/// whole interval; and m(lo) < 0 < m(hi) exactly. Same s₀ = 26 i128
/// envelope as [`ISO15`] (see [`BISECT_CAP`]); the anchor tests
/// re-assert the bracket.
#[allow(dead_code)]
pub const ISO_L10: Iso = Iso {
    lo_num: 127648644,
    hi_num: 127648645,
    s: 26,
    m_increasing: true,
};

/// Float value of an L10 element (cross-check/emission use only).
#[inline]
#[allow(dead_code)]
pub fn kval10(x: [i64; 4]) -> f64 {
    x[0] as f64
        + x[1] as f64 * A10_F
        + x[2] as f64 * (A10_F * A10_F)
        + x[3] as f64 * (A10_F * A10_F * A10_F)
}

#[allow(dead_code)]
impl L10 {
    /// The zero element.
    pub const ZERO: L10 = L10([0, 0, 0, 0]);
    /// The unit element.
    pub const ONE: L10 = L10([1, 0, 0, 0]);
    /// The generator a = 2cos(π/10) (girih's cos-18° generator: cos 18°
    /// = a/2, sin 18° = (a² − 3)/2 — doc §3).
    pub const A: L10 = L10([0, 1, 0, 0]);

    /// Exact zero test — equality is never numeric (doc §5.1).
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == [0, 0, 0, 0]
    }

    /// Field addition.
    #[inline]
    pub fn add(self, y: L10) -> L10 {
        L10(kadd(self.0, y.0))
    }

    /// Field subtraction.
    #[inline]
    pub fn sub(self, y: L10) -> L10 {
        L10(ksub(self.0, y.0))
    }

    /// Field negation.
    #[inline]
    pub fn neg(self) -> L10 {
        L10(kneg(self.0))
    }

    /// Scaling by an integer (exact).
    #[inline]
    pub fn scale(self, c: i64) -> L10 {
        L10(kscale(self.0, c))
    }

    /// Field multiplication (convolution + [`RED_L10`] fold — the
    /// girih_qc rule).
    #[inline]
    pub fn mul(self, y: L10) -> L10 {
        L10(kmul(self.0, y.0, &RED_L10))
    }

    /// Exact sign — the isolating-interval engine ([`ksign`] with the
    /// L₁₀ constants).
    pub fn sign(&self) -> Ordering {
        ksign(self.0, &ML10, &ISO_L10)
    }

    /// Exact norm N(x) = Res(m, x) (Sylvester/Bareiss, i128).
    pub fn norm(&self) -> i128 {
        knorm(&ML10, &self.0)
    }

    /// Float value — cross-check and emission use only.
    #[inline]
    pub fn float(&self) -> f64 {
        kval10(self.0)
    }
}
// =====================================================================
// Test battery — mirrors the spike battery per doc §7 / the brief:
// (1) minpoly zero + float cross-checks, (2) frame, (3) star walks,
// (4) sign/order, (5) norm identities; plus the isolating-interval
// bracket certificate — one shared generic machinery per degree, called
// once per field (K₇, K₉, K₁₅), plus the L₁₀ girih anchor. f64 appears
// only here. Every pasted constant was derived and certified Python-side
// (scratch `derive_k9k15l10.py`) and is re-asserted exactly in Rust.
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    const TWO_PI_OVER_7: f64 = 2.0 * PI / 7.0;
    const TWO_PI_OVER_9: f64 = 2.0 * PI / 9.0;
    const TWO_PI_OVER_15: f64 = 2.0 * PI / 15.0;

    // =================================================================
    // Shared battery machinery (generic over the degree D; the per-field
    // tests below re-point at these — one implementation, three fields).
    // =================================================================

    /// U(k) via the Chebyshev recurrence U(k+1) = b·U(k) − U(k−1) in the
    /// oblique coordinates (doc §4.2 [P]; spike `Frame.U_table`) — the
    /// construction the companion-rotation test is checked *against*
    /// (two independent paths to the same frame).
    fn u_table<const D: usize>(n: usize, red: &[[i64; D]]) -> Vec<([i64; D], [i64; D])> {
        let (gen, one) = (kbasis::<D>(1), kbasis::<D>(0));
        let zero = [0i64; D];
        let mut u = vec![(one, zero), (zero, one)];
        for _ in 2..n {
            let (p1, q1) = u[u.len() - 1];
            let (p0, q0) = u[u.len() - 2];
            u.push((ksub(kmul(gen, p1, red), p0), ksub(kmul(gen, q1, red), q0)));
        }
        u
    }

    /// Frame-point addition on raw (A, B) tuples.
    fn padd2<const D: usize>(
        p: ([i64; D], [i64; D]),
        q: ([i64; D], [i64; D]),
    ) -> ([i64; D], [i64; D]) {
        (kadd(p.0, q.0), kadd(p.1, q.1))
    }

    /// Frame-point difference on raw (A, B) tuples.
    fn psub2<const D: usize>(
        p: ([i64; D], [i64; D]),
        q: ([i64; D], [i64; D]),
    ) -> ([i64; D], [i64; D]) {
        (ksub(p.0, q.0), ksub(p.1, q.1))
    }

    /// Rotation by 2πk/n: k companion steps (k taken mod n).
    fn prot_by<const D: usize>(
        p: ([i64; D], [i64; D]),
        k: usize,
        n: usize,
        red: &[[i64; D]],
    ) -> ([i64; D], [i64; D]) {
        let mut p = p;
        for _ in 0..k % n {
            p = koblique_rot(p.0, p.1, red);
        }
        p
    }

    /// Float coordinates of a frame point: (A + B·cos(2π/n),
    /// B·sin(2π/n)) — emission into the f64 `Tiling` boundary and float
    /// cross-checks only (doc §7: exact tuples → f64 once at emission).
    fn frame_float<const D: usize>(
        a: [i64; D],
        b: [i64; D],
        cf: f64,
        sf: f64,
        fval: impl Fn([i64; D]) -> f64,
    ) -> (f64, f64) {
        let (af, bf) = (fval(a), fval(b));
        (af + bf * cf, bf * sf)
    }

    /// (1) m(b) == 0 exactly on the tuple model; the pasted reduction
    /// rows are exactly b^D … b^{2D−2} under [`kmul`] (and one-hot below
    /// the degree); float cross-checks of the rows and the minpoly
    /// evaluation (spike: reductions ≤ 1e-12, generator ≤ 1e-15).
    fn check_minpoly_red<const D: usize>(
        m: &[i64],
        red: &[[i64; D]],
        fval: impl Fn([i64; D]) -> f64,
        bf: f64,
    ) {
        let gen = kbasis::<D>(1);
        let mut p = gen;
        for j in 2..2 * D - 1 {
            p = kmul(p, gen, red);
            if j < D {
                assert_eq!(p, kbasis::<D>(j), "b^{j} not one-hot");
            } else {
                assert_eq!(p, red[j - D], "b^{j} differs from the pasted reduction row");
            }
        }
        // m(b) == 0 exactly: the low coefficients plus m_D·b^D (the
        // pasted reduction row).
        let mut mb = [0i64; D];
        for (i, &c) in m.iter().enumerate().take(D) {
            mb[i] = c;
        }
        let mb = kadd(mb, kscale(red[0], m[D]));
        assert_eq!(mb, [0i64; D], "m(b) != 0 on the tuple model");
        // float cross-checks
        for (j, row) in red.iter().enumerate() {
            let want = bf.powi((D + j) as i32);
            assert!(
                (fval(*row) - want).abs() < 1e-12,
                "b^{} row float: {} vs {want}",
                D + j,
                fval(*row)
            );
        }
        let mf: f64 = m
            .iter()
            .enumerate()
            .map(|(i, &c)| c as f64 * bf.powi(i as i32))
            .sum();
        assert!(mf.abs() < 1e-12, "minpoly float evaluation: {mf}");
    }

    /// Sign-engine certificate, exact: with m strictly increasing on the
    /// isolating interval (certified Python-side) and m(b) = 0, opposite
    /// exact signs of m at the endpoints pin b strictly inside — no
    /// numerics. Then the float sanity of the pasted dyadic endpoints
    /// (width ≤ `max_w`) and a smoke pass of the engine on the obvious
    /// elements.
    fn check_iso_bracket<const D: usize>(
        m: &[i64],
        iso: &Iso,
        bf: f64,
        max_w: f64,
        sign: impl Fn([i64; D]) -> Ordering,
    ) {
        assert_eq!(poly_sign_at(m, iso.lo_num, iso.s), -1, "m(lo) < 0");
        assert_eq!(poly_sign_at(m, iso.hi_num, iso.s), 1, "m(hi) > 0");
        let den = (1u64 << iso.s) as f64;
        let (lof, hif) = (iso.lo_num as f64 / den, iso.hi_num as f64 / den);
        assert!(lof < bf && bf < hif, "float b not inside the interval");
        assert!(hif - lof <= max_w, "interval unexpectedly wide");
        // smoke: the engine signs the obvious elements
        assert_eq!(sign([0i64; D]), Ordering::Equal);
        assert_eq!(sign(kbasis::<D>(0)), Ordering::Greater);
        assert_eq!(sign(kbasis::<D>(1)), Ordering::Greater);
        assert_eq!(sign(kscale(kbasis::<D>(1), -1)), Ordering::Less);
    }

    /// (2) |U(k)|² == 1 exactly for all k; the dot machinery on frame
    /// vectors (2·U(k)·U(k) == 2, so the halved dot is 1 — all U(k)·U(k)
    /// products stay integral); and the concrete value 2·U(2)·U(0) =
    /// b² − 2 (= 2cos(4π/n), the Chebyshev trace).
    fn check_frame_norms<const D: usize>(n: usize, red: &[[i64; D]]) {
        let u = u_table(n, red);
        let two = kscale(kbasis::<D>(0), 2);
        let mut b2m2 = [0i64; D];
        b2m2[0] = -2;
        b2m2[2] = 1;
        for (k, p) in u.iter().enumerate() {
            assert_eq!(
                koblique_norm2(p.0, p.1, red),
                kbasis::<D>(0),
                "|U({k})|² != 1"
            );
            assert_eq!(
                koblique_dot2(p.0, p.1, p.0, p.1, red),
                two,
                "2·U({k})·U({k}) != 2"
            );
        }
        assert_eq!(
            koblique_dot2(u[2].0, u[2].1, u[0].0, u[0].1, red),
            b2m2,
            "2·U(2)·U(0) = b² − 2"
        );
    }

    /// (2) Σ_k U(k) == 0 exactly in both coordinates (spike L519–521).
    fn check_frame_sums<const D: usize>(n: usize, red: &[[i64; D]]) {
        let u = u_table(n, red);
        let sa = u.iter().fold([0i64; D], |acc, p| kadd(acc, p.0));
        let sb = u.iter().fold([0i64; D], |acc, p| kadd(acc, p.1));
        assert_eq!(sa, [0i64; D], "Σ A-coordinates != 0");
        assert_eq!(sb, [0i64; D], "Σ B-coordinates != 0");
    }

    /// (2) R^k U(0) == U(k) and R^k U(1) == U(k+1) for all k — the
    /// companion matrix reproduces the recurrence-built frame (spike
    /// L545–548); rot_by(n) is the identity (U(k+n) = U(k)).
    fn check_companion_rotation<const D: usize>(n: usize, red: &[[i64; D]]) {
        let u = u_table(n, red);
        for k in 0..n {
            assert_eq!(prot_by(u[0], k, n, red), u[k], "R^{k} U(0) != U({k})");
            assert_eq!(
                prot_by(u[1], k, n, red),
                u[(k + 1) % n],
                "R^{k} U(1) != U({})",
                (k + 1) % n
            );
        }
        assert_eq!(prot_by(u[2], n, n, red), u[2], "U(k+n) != U(k)");
    }

    /// (2) the exact frame lands on the float directions (spike L523–527,
    /// tolerance 1e-12).
    fn check_frame_float_dirs<const D: usize>(
        n: usize,
        red: &[[i64; D]],
        step: f64,
        cf: f64,
        sf: f64,
        fval: impl Fn([i64; D]) -> f64,
    ) {
        for (k, p) in u_table(n, red).iter().enumerate() {
            let (x, y) = frame_float(p.0, p.1, cf, sf, &fval);
            let (sy, cx) = (step * k as f64).sin_cos();
            assert!(
                (x - cx).abs() < 1e-12 && (y - sy).abs() < 1e-12,
                "U({k}) float direction off: ({x}, {y}) vs ({cx}, {sy})"
            );
        }
    }

    /// (3) unit-edge star walk {n/s}: vertices P_j = Σ_{i<j} U(s·i mod n).
    fn star_walk<const D: usize>(
        n: usize,
        s: usize,
        red: &[[i64; D]],
    ) -> Vec<([i64; D], [i64; D])> {
        let u = u_table(n, red);
        let mut p = ([0i64; D], [0i64; D]);
        let mut pts = vec![p];
        for i in 0..n {
            p = padd2(p, u[(s * i) % n]);
            pts.push(p);
        }
        pts
    }

    /// Shared {n/s} battery (spike L552–593): exact closure; the scaled
    /// centered walk Ĉ_j = n·P_j − S (S = ΣP_j) is R^s-equivariant (the
    /// exact centroid satisfies R^s G = G − U(0), proven in the spike;
    /// carrying n·P_j − S keeps the D = 1 core in Z[b]); edge directions
    /// advance by exactly one R-step; the shoelace factor Σ
    /// cross(P_j, P_{j+1}) is an exact field constant (2·area/sin(2π/n));
    /// float area agreement.
    fn check_star_walk<const D: usize>(
        n: usize,
        s: usize,
        red: &[[i64; D]],
        shoelace: [i64; D],
        cf: f64,
        sf: f64,
        fval: impl Fn([i64; D]) -> f64,
    ) {
        let u = u_table(n, red);
        let pts = star_walk(n, s, red);
        let zero = [0i64; D];
        assert_eq!(
            pts[n],
            (zero, zero),
            "walk {{n/{s}}} does not close exactly"
        );
        let sum = pts.iter().take(n).fold((zero, zero), |p, q| padd2(p, *q));
        let centered = |p: ([i64; D], [i64; D])| -> ([i64; D], [i64; D]) {
            (
                ksub(kscale(p.0, n as i64), sum.0),
                ksub(kscale(p.1, n as i64), sum.1),
            )
        };
        for j in 0..n {
            assert_eq!(
                prot_by(centered(pts[j]), s, n, red),
                centered(pts[(j + 1) % n]),
                "centered walk not R^{s}-equivariant at j = {j}"
            );
            let e = psub2(pts[(j + 1) % n], pts[j]);
            assert_eq!(
                prot_by(e, s, n, red),
                u[(s * (j + 1)) % n],
                "edge direction not R^{s}-regular at j = {j}"
            );
        }
        let sho = (0..n).fold([0i64; D], |acc, j| {
            kadd(
                acc,
                koblique_cross(
                    pts[j].0,
                    pts[j].1,
                    pts[(j + 1) % n].0,
                    pts[(j + 1) % n].1,
                    red,
                ),
            )
        });
        assert_eq!(
            sho, shoelace,
            "shoelace factor differs from the spike value"
        );
        let area = fval(sho) * sf / 2.0;
        let fp: Vec<(f64, f64)> = pts
            .iter()
            .take(n)
            .map(|p| frame_float(p.0, p.1, cf, sf, &fval))
            .collect();
        let farea: f64 = (0..n)
            .map(|j| fp[j].0 * fp[(j + 1) % n].1 - fp[(j + 1) % n].0 * fp[j].1)
            .sum::<f64>()
            / 2.0;
        assert!(
            (area.abs() - farea.abs()).abs() < 1e-9,
            "exact area {area} vs float area {farea}"
        );
    }

    /// (4) exact signs match float signs; floats distinct; the exact
    /// total order (comparator = sign(x − y), doc §5.4) matches the
    /// float sort.
    fn check_sign_order<const D: usize>(
        battery: &[[i64; D]],
        m: &[i64],
        iso: &Iso,
        fval: impl Fn([i64; D]) -> f64,
    ) {
        for x in battery {
            let fx = fval(*x);
            assert!(fx.abs() > 1e-8, "battery element too close to 0: {x:?}");
            let want = if fx > 0.0 {
                Ordering::Greater
            } else {
                Ordering::Less
            };
            assert_eq!(
                ksign(*x, m, iso),
                want,
                "exact sign mismatch at {x:?} (float {fx})"
            );
        }
        let mut fl: Vec<[i64; D]> = battery.to_vec();
        fl.sort_by(|a, b| fval(*a).partial_cmp(&fval(*b)).unwrap());
        for w in fl.windows(2) {
            assert!(fval(w[0]) < fval(w[1]), "float collision in battery");
        }
        let mut exact: Vec<[i64; D]> = battery.to_vec();
        exact.sort_by(|a, b| ksign(ksub(*a, *b), m, iso));
        assert_eq!(exact, fl, "exact total order differs from the float sort");
    }

    /// (5) norm identities via the exact Sylvester resultant (spike
    /// L596–603): N(1) = 1, N(b) = (−1)^d·m₀ = `nb`, multiplicativity on
    /// the integral probes with y = x+1 (the spike's third probe
    /// 1/(b²+1) is not in Z[b]; its denominator element is in the sign
    /// battery instead).
    fn check_norm_identities<const D: usize>(
        m: &[i64],
        red: &[[i64; D]],
        probes: &[[i64; D]],
        nb: i128,
    ) {
        let one = kbasis::<D>(0);
        let gen = kbasis::<D>(1);
        assert_eq!(knorm(m, &one), 1, "N(1) != 1");
        assert_eq!(
            knorm(m, &gen),
            (-1i128).pow((m.len() - 1) as u32) * m[0] as i128,
            "N(b) != (−1)^d·m₀"
        );
        assert_eq!(knorm(m, &gen), nb, "N(b) unexpected");
        for x in probes {
            let y = kadd(*x, one);
            assert_eq!(
                knorm(m, &kmul(*x, y, red)),
                knorm(m, x) * knorm(m, &y),
                "N not multiplicative at {x:?}"
            );
        }
    }

    // =================================================================
    // K7 battery (landed M137, re-pointed at the shared machinery)
    // =================================================================

    /// (1) m(b) == 0 exactly on the tuple model; the pasted reduction
    /// rows are exactly b³ and b⁴ under [`kmul`]; float cross-checks of
    /// the pasted constants (spike: reductions ≤ 1e-12, b ≤ 1e-15).
    #[test]
    fn minpoly_zero_on_tuple_model_and_float() {
        check_minpoly_red(&M7, &RED7, kval7, B_F);
        assert!(
            (B_F - 2.0 * TWO_PI_OVER_7.cos()).abs() < 1e-15,
            "B_F constant"
        );
        assert!(
            (SIN_F - TWO_PI_OVER_7.sin()).abs() < 1e-15,
            "SIN_F constant"
        );
    }

    /// Sign-engine certificate, exact (see [`check_iso_bracket`]).
    #[test]
    fn iso_interval_brackets_b_exactly() {
        check_iso_bracket(&M7, &ISO7, B_F, 5e-10, |x| K7(x).sign());
    }

    /// (2) |U(k)|² == 1 exactly for all k, plus the halved-dot path on a
    /// concrete frame vector (dot(U(k), U(k)) == 1 — all U(k)·U(k)
    /// products stay integral, so the ½ never leaves Z[b]).
    #[test]
    fn frame_unit_norms() {
        check_frame_norms(7, &RED7);
        let u = u_table::<3>(7, &RED7);
        let p = P7(K7(u[2].0), K7(u[2].1));
        assert_eq!(p.dot(p), K7::ONE, "U(2)·U(2) != 1 (halved-dot path)");
    }

    /// (2) Σ_k U(k) == 0 exactly in both coordinates (spike L519–521).
    #[test]
    fn frame_sums_to_zero() {
        check_frame_sums(7, &RED7);
    }

    /// (2) R^k U(0) == U(k) and R^k U(1) == U(k+1) for all k — the
    /// companion matrix reproduces the recurrence-built frame (spike
    /// L545–548); rot_by(7) is the identity (U(k+7) = U(k)).
    #[test]
    fn companion_rotation_consistency() {
        check_companion_rotation(7, &RED7);
    }

    /// (2) the exact frame lands on the float directions (spike L523–527,
    /// tolerance 1e-12).
    #[test]
    fn frame_matches_float_directions() {
        check_frame_float_dirs(7, &RED7, TWO_PI_OVER_7, B_F / 2.0, SIN_F, kval7);
    }

    /// (3) {7/1} — the regular heptagon walk; shoelace 4 + 3b + b².
    #[test]
    fn star_walk_7_1_regular_and_exact_shoelace() {
        check_star_walk(7, 1, &RED7, [4, 3, 1], B_F / 2.0, SIN_F, kval7);
    }

    /// (3) {7/3} — the far-star walk; shoelace −8 + b + 5b².
    #[test]
    fn star_walk_7_3_regular_and_exact_shoelace() {
        check_star_walk(7, 3, &RED7, [-8, 1, 5], B_F / 2.0, SIN_F, kval7);
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
        let cells: [[i64; 3]; 29] = BATTERY.map(|k| k.0);
        check_sign_order(&cells, &M7, &ISO7, kval7);
    }

    /// (5) norm identities via the exact Sylvester resultant (spike
    /// L596–603): N(1) = 1, N(b) = (−1)³·m₀ = 1, multiplicativity on the
    /// integral probes b+1, b²−3b, b⁴ with y = x+1.
    #[test]
    fn norm_identities_sylvester() {
        check_norm_identities(&M7, &RED7, &[[1, 1, 0], [0, -3, 1], [-1, -1, 3]], 1);
    }

    // =================================================================
    // K9 battery
    // =================================================================

    /// (1) m(b) == 0 exactly on the tuple model; the pasted reduction
    /// rows are exactly b³ and b⁴ under [`kmul`]; float cross-checks of
    /// the pasted constants (scratch `derive_k9k15l10.py`: reductions
    /// ≤ 1e-12, b ≤ 1e-15).
    #[test]
    fn k9_minpoly_zero_on_tuple_model_and_float() {
        check_minpoly_red(&M9, &RED9, kval9, B9_F);
        assert!(
            (B9_F - 2.0 * TWO_PI_OVER_9.cos()).abs() < 1e-15,
            "B9_F constant"
        );
        assert!(
            (SIN9_F - TWO_PI_OVER_9.sin()).abs() < 1e-15,
            "SIN9_F constant"
        );
    }

    /// Sign-engine certificate, exact (see [`check_iso_bracket`]).
    #[test]
    fn k9_iso_interval_brackets_b_exactly() {
        check_iso_bracket(&M9, &ISO9, B9_F, 5e-10, |x| K9(x).sign());
    }

    /// (2) |U(k)|² == 1 for all k plus the halved-dot path (see the K₇
    /// test; all U(k)·U(k) products stay integral in Z[b] — the D = 1
    /// core claim, spike-verified for K₉).
    #[test]
    fn k9_frame_unit_norms() {
        check_frame_norms(9, &RED9);
        let u = u_table::<3>(9, &RED9);
        let p = P9(K9(u[4].0), K9(u[4].1));
        assert_eq!(p.dot(p), K9::ONE, "U(4)·U(4) != 1 (halved-dot path)");
    }

    /// (2) Σ_k U(k) == 0 exactly in both coordinates.
    #[test]
    fn k9_frame_sums_to_zero() {
        check_frame_sums(9, &RED9);
    }

    /// (2) R^k U(0) == U(k) and R^k U(1) == U(k+1) for all k;
    /// rot_by(9) is the identity (U(k+9) = U(k)).
    #[test]
    fn k9_companion_rotation_consistency() {
        check_companion_rotation(9, &RED9);
    }

    /// (2) the exact frame lands on the float directions (≤ 1e-12).
    #[test]
    fn k9_frame_matches_float_directions() {
        check_frame_float_dirs(9, &RED9, TWO_PI_OVER_9, B9_F / 2.0, SIN9_F, kval9);
    }

    /// (3) {9/1} — the regular 9-gon walk; shoelace 3 + 6b + 3b².
    #[test]
    fn k9_star_walk_9_1_regular_and_exact_shoelace() {
        check_star_walk(9, 1, &RED9, [3, 6, 3], B9_F / 2.0, SIN9_F, kval9);
    }

    /// (3) {9/4} — the far-star walk; shoelace −15 + 6b + 3b².
    #[test]
    fn k9_star_walk_9_4_regular_and_exact_shoelace() {
        check_star_walk(9, 4, &RED9, [-15, 6, 3], B9_F / 2.0, SIN9_F, kval9);
    }

    /// (4) the 29-element sign/order battery, spike recipe (see the K₇
    /// twin): the 24 seeded random elements verbatim (random.seed(9),
    /// coefficients in [−6, 6], low→high), then the fixed probes b+1,
    /// b²−3b, b⁴, and the integral stand-ins b²+1 (for 1/(b²+1)) and 3b
    /// (for b/3). Scratch re-certified: distinct floats (min |value|
    /// 1.84, min pairwise gap 0.030), exact signs and exact total order
    /// matching the float sort, and every element/order comparison
    /// resolved by the [`ksign`] engine in a single pass (cap-4
    /// simulation, `derive_k9k15l10.py`).
    const BATTERY9: [K9; 29] = [
        K9([1, 3, -1]),
        K9([-2, -4, -4]),
        K9([4, -6, -1]),
        K9([2, 1, 3]),
        K9([-5, -1, 2]),
        K9([3, 5, -6]),
        K9([5, 0, -4]),
        K9([5, 1, 5]),
        K9([0, -4, -4]),
        K9([-3, -6, -5]),
        K9([-4, 2, 3]),
        K9([-5, 6, 5]),
        K9([0, 6, 5]),
        K9([-5, -2, -3]),
        K9([4, -3, 5]),
        K9([6, 0, -5]),
        K9([6, -2, -3]),
        K9([0, -2, -1]),
        K9([6, -6, -3]),
        K9([5, -6, 0]),
        K9([-6, 0, 1]),
        K9([-4, -6, -3]),
        K9([0, 5, -5]),
        K9([3, -6, -5]),
        K9([1, 1, 0]),  // b + 1
        K9([0, -3, 1]), // b² − 3b
        K9([0, -1, 3]), // b⁴
        K9([1, 0, 1]),  // b² + 1   (stands in for 1/(b²+1))
        K9([0, 3, 0]),  // 3b       (stands in for b/3)
    ];

    /// (4) exact signs match float signs; floats distinct; the exact
    /// total order matches the float sort.
    #[test]
    fn k9_sign_order_battery_matches_float() {
        let cells: [[i64; 3]; 29] = BATTERY9.map(|k| k.0);
        check_sign_order(&cells, &M9, &ISO9, kval9);
    }

    /// (5) norm identities: N(1) = 1, N(b) = (−1)³·m₀ = −1 (K₉'s minpoly
    /// has m₀ = 1 — unlike K₇, the generator has negative norm),
    /// multiplicativity on the integral probes b+1, b²−3b, b⁴.
    #[test]
    fn k9_norm_identities_sylvester() {
        check_norm_identities(&M9, &RED9, &[[1, 1, 0], [0, -3, 1], [0, -1, 3]], -1);
    }

    // =================================================================
    // K15 battery
    // =================================================================

    /// (1) m(b) == 0 exactly on the tuple model; the pasted reduction
    /// rows are exactly b⁴, b⁵, b⁶ under [`kmul`]; float cross-checks of
    /// the pasted constants (scratch `derive_k9k15l10.py`).
    #[test]
    fn k15_minpoly_zero_on_tuple_model_and_float() {
        check_minpoly_red(&M15, &RED15, kval15, B15_F);
        assert!(
            (B15_F - 2.0 * TWO_PI_OVER_15.cos()).abs() < 1e-15,
            "B15_F constant"
        );
        assert!(
            (SIN15_F - TWO_PI_OVER_15.sin()).abs() < 1e-15,
            "SIN15_F constant"
        );
    }

    /// Sign-engine certificate, exact (see [`check_iso_bracket`]); the
    /// width bound is the deg-4 s₀ = 26 envelope ([`BISECT_CAP`]).
    #[test]
    fn k15_iso_interval_brackets_b_exactly() {
        check_iso_bracket(&M15, &ISO15, B15_F, 2e-8, |x| K15(x).sign());
    }

    /// K₁₅ ⊃ Q(√5) — the doc §3 subfield probe: s = b³ − 3b =
    /// 2cos(2π/5) = 1/φ satisfies s² + s − 1 = 0 exactly (the quadratic
    /// subfield certified by exact substitution, no radicals written).
    #[test]
    fn k15_sqrt5_subfield_probe_exact() {
        let s = K15([0, -3, 0, 1]); // b³ − 3b
        assert_eq!(s.mul(s).add(s).sub(K15::ONE), K15::ZERO, "s² + s − 1 != 0");
        assert!(
            (kval15(s.0) - 2.0 * (2.0 * PI / 5.0).cos()).abs() < 1e-12,
            "s != 2cos(2π/5)"
        );
        assert!(
            (kval15(s.0) - ((5.0f64).sqrt() - 1.0) / 2.0).abs() < 1e-12,
            "s != 1/φ"
        );
    }

    /// (2) |U(k)|² == 1 for all k plus the halved-dot path (all U(k)·U(k)
    /// products stay integral in Z[b] — spike-verified for K₁₅).
    #[test]
    fn k15_frame_unit_norms() {
        check_frame_norms(15, &RED15);
        let u = u_table::<4>(15, &RED15);
        let p = P15(K15(u[5].0), K15(u[5].1));
        assert_eq!(p.dot(p), K15::ONE, "U(5)·U(5) != 1 (halved-dot path)");
    }

    /// (2) Σ_k U(k) == 0 exactly in both coordinates.
    #[test]
    fn k15_frame_sums_to_zero() {
        check_frame_sums(15, &RED15);
    }

    /// (2) R^k U(0) == U(k) and R^k U(1) == U(k+1) for all k;
    /// rot_by(15) is the identity (U(k+15) = U(k)).
    #[test]
    fn k15_companion_rotation_consistency() {
        check_companion_rotation(15, &RED15);
    }

    /// (2) the exact frame lands on the float directions (≤ 1e-12).
    #[test]
    fn k15_frame_matches_float_directions() {
        check_frame_float_dirs(15, &RED15, TWO_PI_OVER_15, B15_F / 2.0, SIN15_F, kval15);
    }

    /// (3) {15/1} — the regular 15-gon walk; shoelace −30b + 15b² + 15b³.
    #[test]
    fn k15_star_walk_15_1_regular_and_exact_shoelace() {
        check_star_walk(
            15,
            1,
            &RED15,
            [0, -30, 15, 15],
            B15_F / 2.0,
            SIN15_F,
            kval15,
        );
    }

    /// (3) {15/7} — the far-star walk; shoelace
    /// −30 − 60b + 15b² + 15b³.
    #[test]
    fn k15_star_walk_15_7_regular_and_exact_shoelace() {
        check_star_walk(
            15,
            7,
            &RED15,
            [-30, -60, 15, 15],
            B15_F / 2.0,
            SIN15_F,
            kval15,
        );
    }

    /// (4) the 29-element sign/order battery, spike recipe (see the K₇
    /// twin): the 24 seeded random elements verbatim (random.seed(15),
    /// coefficients in [−6, 6], low→high), then the fixed probes b+1,
    /// b²−3b, b⁴, and the integral stand-ins b²+1 (for 1/(b²+1)) and 3b
    /// (for b/3). Scratch re-certified: distinct floats (min |value|
    /// 2.05, min pairwise gap 0.088), exact signs and exact total order
    /// matching the float sort, every comparison resolved in a single
    /// pass (cap-4 simulation, `derive_k9k15l10.py`).
    const BATTERY15: [K15; 29] = [
        K15([-3, -6, 2, 5]),
        K15([-6, -4, -3, -6]),
        K15([-6, 6, 4, -4]),
        K15([5, -1, -3, -5]),
        K15([-1, 1, 5, -1]),
        K15([-2, 0, -2, -1]),
        K15([-3, -3, -1, 6]),
        K15([-1, -3, -2, 5]),
        K15([2, 0, -3, 3]),
        K15([1, 0, 1, -5]),
        K15([1, 3, 6, -1]),
        K15([1, 3, -1, 5]),
        K15([4, 1, 0, 6]),
        K15([6, -5, 1, -6]),
        K15([-3, -4, 2, -4]),
        K15([3, 6, 5, 5]),
        K15([-6, -2, 2, 1]),
        K15([-4, -5, -5, 0]),
        K15([-6, -5, 1, 5]),
        K15([2, -2, 0, 2]),
        K15([-3, -4, 2, 2]),
        K15([-1, -5, -3, -2]),
        K15([-2, -4, 4, -1]),
        K15([0, 1, -1, 1]),
        K15([1, 1, 0, 0]),   // b + 1
        K15([0, -3, 1, 0]),  // b² − 3b
        K15([-1, -4, 4, 1]), // b⁴
        K15([1, 0, 1, 0]),   // b² + 1   (stands in for 1/(b²+1))
        K15([0, 3, 0, 0]),   // 3b       (stands in for b/3)
    ];

    /// (4) exact signs match float signs; floats distinct; the exact
    /// total order matches the float sort.
    #[test]
    fn k15_sign_order_battery_matches_float() {
        let cells: [[i64; 4]; 29] = BATTERY15.map(|k| k.0);
        check_sign_order(&cells, &M15, &ISO15, kval15);
    }

    /// (5) norm identities: N(1) = 1, N(b) = (−1)⁴·m₀ = 1,
    /// multiplicativity on the integral probes b+1, b²−3b, b⁴.
    #[test]
    fn k15_norm_identities_sylvester() {
        check_norm_identities(
            &M15,
            &RED15,
            &[[1, 1, 0, 0], [0, -3, 1, 0], [-1, -4, 4, 1]],
            1,
        );
    }

    // =================================================================
    // L10 — the girih field: anchor tests (no frame/walks — girih_qc
    // keeps its own tables; this core is their exact carrier, doc §7)
    // =================================================================

    /// (1) m(a) == 0 exactly on the tuple model; the pasted reduction
    /// rows are exactly a⁴, a⁵, a⁶ under [`kmul`] — i.e. the shipped
    /// girih_qc `kmul` rule; float cross-checks; and the norm identities
    /// N(1) = 1, N(a) = (−1)⁴·m₀ = 5, multiplicative on integral probes.
    #[test]
    fn l10_minpoly_red_rows_and_norm() {
        check_minpoly_red(&ML10, &RED_L10, kval10, A10_F);
        assert!(
            (A10_F - 2.0 * (PI / 10.0).cos()).abs() < 1e-15,
            "A10_F constant"
        );
        check_norm_identities(
            &ML10,
            &RED_L10,
            &[[1, 1, 0, 0], [-3, 0, 1, 0], [-5, 0, 5, 0]],
            5,
        );
    }

    /// Sign-engine certificate, exact (see [`check_iso_bracket`]); the
    /// width bound is the deg-4 s₀ = 26 envelope ([`BISECT_CAP`]).
    #[test]
    fn l10_iso_interval_brackets_a_exactly() {
        check_iso_bracket(&ML10, &ISO_L10, A10_F, 2e-8, |x| L10(x).sign());
    }

    /// cos 18° = a/2, sin 18° = (a²−3)/2, and sin²18 + cos²18 = 1
    /// exactly — the identity that exercises the shipped kmul rule
    /// a⁴ = 5a² − 5 (doc §3 [P, spike]; the shipped girih rotation
    /// entries are built from exactly these two elements).
    #[test]
    fn l10_girih_cos_sin_identity_exact() {
        let a2 = L10::A.mul(L10::A);
        assert_eq!(a2, L10([0, 0, 1, 0]), "a² not one-hot");
        let s = a2.sub(L10([3, 0, 0, 0])); // a² − 3 = 2·sin 18°
                                           // (a²−3)² + a² == 4  ⇔  ((a²−3)/2)² + (a/2)² == 1, exactly:
                                           // (a²−3)² = a⁴ − 6a² + 9 = (5a²−5) − 6a² + 9 = 4 − a².
        assert_eq!(s.mul(s).add(a2), L10([4, 0, 0, 0]), "sin²+cos² != 1");
        // float cross-checks
        assert!(
            (kval10(s.0) / 2.0 - (PI / 10.0).sin()).abs() < 1e-12,
            "sin 18° float"
        );
        assert!(
            (kval10(L10::A.0) / 2.0 - (PI / 10.0).cos()).abs() < 1e-15,
            "cos 18° float"
        );
    }

    /// The pasted 20 girih rotation matrices R_r = R₁^r (rotations by
    /// 18r°, entries `Ke = [i64; 4]` at the global denominator 64) —
    /// verbatim from the shipped
    /// `families/islamic/girih_qc.rs` `const ROTM` (80 tuples; the
    /// bit-for-bit validation target, doc §7).
    const ANCHOR_ROTM: [[[[i64; 4]; 2]; 2]; 20] = [
        [[[64, 0, 0, 0], [0, 0, 0, 0]], [[0, 0, 0, 0], [64, 0, 0, 0]]],
        [
            [[0, 32, 0, 0], [96, 0, -32, 0]],
            [[-96, 0, 32, 0], [0, 32, 0, 0]],
        ],
        [
            [[-64, 0, 32, 0], [0, 96, 0, -32]],
            [[0, -96, 0, 32], [-64, 0, 32, 0]],
        ],
        [
            [[0, -96, 0, 32], [64, 0, -32, 0]],
            [[-64, 0, 32, 0], [0, -96, 0, 32]],
        ],
        [
            [[-96, 0, 32, 0], [0, -32, 0, 0]],
            [[0, 32, 0, 0], [-96, 0, 32, 0]],
        ],
        [
            [[0, 0, 0, 0], [-64, 0, 0, 0]],
            [[64, 0, 0, 0], [0, 0, 0, 0]],
        ],
        [
            [[96, 0, -32, 0], [0, -32, 0, 0]],
            [[0, 32, 0, 0], [96, 0, -32, 0]],
        ],
        [
            [[0, 96, 0, -32], [64, 0, -32, 0]],
            [[-64, 0, 32, 0], [0, 96, 0, -32]],
        ],
        [
            [[64, 0, -32, 0], [0, 96, 0, -32]],
            [[0, -96, 0, 32], [64, 0, -32, 0]],
        ],
        [
            [[0, -32, 0, 0], [96, 0, -32, 0]],
            [[-96, 0, 32, 0], [0, -32, 0, 0]],
        ],
        [
            [[-64, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 0, 0, 0], [-64, 0, 0, 0]],
        ],
        [
            [[0, -32, 0, 0], [-96, 0, 32, 0]],
            [[96, 0, -32, 0], [0, -32, 0, 0]],
        ],
        [
            [[64, 0, -32, 0], [0, -96, 0, 32]],
            [[0, 96, 0, -32], [64, 0, -32, 0]],
        ],
        [
            [[0, 96, 0, -32], [-64, 0, 32, 0]],
            [[64, 0, -32, 0], [0, 96, 0, -32]],
        ],
        [
            [[96, 0, -32, 0], [0, 32, 0, 0]],
            [[0, -32, 0, 0], [96, 0, -32, 0]],
        ],
        [
            [[0, 0, 0, 0], [64, 0, 0, 0]],
            [[-64, 0, 0, 0], [0, 0, 0, 0]],
        ],
        [
            [[-96, 0, 32, 0], [0, 32, 0, 0]],
            [[0, -32, 0, 0], [-96, 0, 32, 0]],
        ],
        [
            [[0, -96, 0, 32], [-64, 0, 32, 0]],
            [[64, 0, -32, 0], [0, -96, 0, 32]],
        ],
        [
            [[-64, 0, 32, 0], [0, -96, 0, 32]],
            [[0, 96, 0, -32], [-64, 0, 32, 0]],
        ],
        [
            [[0, 32, 0, 0], [-96, 0, 32, 0]],
            [[96, 0, -32, 0], [0, 32, 0, 0]],
        ],
    ];

    /// THE permanent regression anchor (doc §7): the 20 shipped girih
    /// rotation matrices R_r = R₁^r rebuilt through this module's generic
    /// layer — exact /64 field products via [`kmul`] with [`RED_L10`]
    /// (the girih_qc rule: fold, assert the product stays in the /64
    /// lattice, divide), accumulated along the matrix chain — compared
    /// bit-for-bit against the 80 pasted tuples above. Any regression
    /// in the generic field layer, the L₁₀ reduction rows, or the girih
    /// table breaks this equality.
    #[test]
    fn girih_rotation_matrices_rebuild_bit_exact() {
        // One exact /64 field product: girih_qc's kmul rule.
        fn mul64(x: [i64; 4], y: [i64; 4]) -> [i64; 4] {
            let c = kmul(x, y, &RED_L10);
            assert!(
                c.iter().all(|&v| v % 64 == 0),
                "girih /64 product left the fixed denominator: {c:?}"
            );
            [c[0] / 64, c[1] / 64, c[2] / 64, c[3] / 64]
        }
        // 2×2 matrix product at /64: entries are field elements, so each
        // product entry is the sum of two exact /64 products.
        fn rot_mul(m: [[[i64; 4]; 2]; 2], r: [[[i64; 4]; 2]; 2]) -> [[[i64; 4]; 2]; 2] {
            let mut out = [[[0i64; 4]; 2]; 2];
            for i in 0..2 {
                for j in 0..2 {
                    let p0 = mul64(m[i][0], r[0][j]);
                    let p1 = mul64(m[i][1], r[1][j]);
                    out[i][j] = [p0[0] + p1[0], p0[1] + p1[1], p0[2] + p1[2], p0[3] + p1[3]];
                }
            }
            out
        }
        const I64: [[[i64; 4]; 2]; 2] =
            [[[64, 0, 0, 0], [0, 0, 0, 0]], [[0, 0, 0, 0], [64, 0, 0, 0]]];
        // R₁ = rotation by 18°: cos 18° = a/2, sin 18° = (a²−3)/2; at
        // /64: a/2 → 32a, −sin → 32(3−a²), sin → 32(a²−3).
        const R1: [[[i64; 4]; 2]; 2] = [
            [[0, 32, 0, 0], [96, 0, -32, 0]],
            [[-96, 0, 32, 0], [0, 32, 0, 0]],
        ];
        assert_eq!(I64, ANCHOR_ROTM[0], "identity != shipped ROTM[0]");
        assert_eq!(R1, ANCHOR_ROTM[1], "R1 != shipped ROTM[1]");
        let mut m = I64;
        for r in 1..20 {
            m = rot_mul(m, R1);
            assert_eq!(
                m, ANCHOR_ROTM[r],
                "R^{r} != shipped ROTM[{r}] — girih table or field-layer regression"
            );
        }
    }
}
