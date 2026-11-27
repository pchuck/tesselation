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
///
/// # i128 envelope (impl-time exact audit, pinned by
/// [`k11_k13_sign_engine_envelope`])
///
/// Near a simple root the Horner **intermediates** — step
/// `A₁·num = 2^{deg·s′}·(m(y) − m₀)` ride at `|m₀|·2^{deg·s′}` — bound
/// the evaluation, not the final value `2^{deg·s}·|m(y)|`: the true
/// wall is `deg·s′ + 1` bits. The steps are therefore **checked** — an
/// evaluation past the envelope fails loudly (release included) instead
/// of wrapping silently. Live non-firing peaks on the certified grids
/// (ceiling 127): K₇/K₉ @ s = 31 endpoints 94, K₁₅/L₁₀ @ s = 26
/// 105/107, [`ISO13`] @ s₀ = 20 endpoints 121 / fallback round 1 = 127.
/// The one constant past it — [`ISO11`] @ s = 26, endpoints 131, CAP = 4
/// mid-evals 136–151 — is certified through the exact 192-bit scan
/// [`poly_scan_192`] in [`check_iso_bracket`] instead, so no certified
/// workload ever reaches these checks in overflow.
fn poly_sign_at(m: &[i64], num: i64, s: i32) -> i32 {
    let deg = m.len() - 1;
    let mut acc = m[deg] as i128; // leading coefficient
    let mut sa = 0i32;
    for i in (0..deg).rev() {
        // Checked Horner steps (envelope note above): identical
        // arithmetic, but a past-envelope evaluation panics with the
        // operating-range message in every profile instead of wrapping.
        let prod = acc.checked_mul(num as i128).unwrap_or_else(|| {
            panic!(
                "poly_sign_at: Horner product overflow (deg={deg}, s={s}, \
                 sa={sa}) — interval scale past the i128 envelope; see the \
                 envelope note on this function"
            )
        });
        let add = (m[i] as i128).checked_shl((sa + s) as u32).unwrap_or_else(|| {
            panic!(
                "poly_sign_at: addend shift overflow (sa+s={})",
                sa + s
            )
        });
        acc = prod.checked_add(add).unwrap_or_else(|| {
            panic!(
                "poly_sign_at: Horner sum overflow (deg={deg}, s={s}, \
                 sa={sa}) — interval scale past the i128 envelope"
            )
        });
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
// K11 — Q(b), b = 2cos(2π/11); degree 5; denominator 1. The odd-prime
// cyclotomic field of the M178 design (docs/cyclotomic-field-degree5.md):
// additive-only infrastructure at landing, zero consumers — the K9/M171
// pattern (the future n = 11 monster geometry lives in its quadratic
// extension by w = 2sin(π/11), w² = 2 − b; design §5.2).
// =====================================================================

/// Float value of the generator b = 2cos(2π/11) (cross-check/emission
/// use only).
const B11_F: f64 = 1.6825070656623624;

/// An element of K₁₁ = Q(b), b = 2cos(2π/11): the tuple `K11([c0..c4])`
/// is the field element c0 + c1·b + … + c4·b⁴ at global denominator 1.
///
/// Minpoly (M178-certified — the Laurent identity x⁵·m(x + x⁻¹) = Φ₁₁
/// holds EXACTLY and m is irreducible (Rabin test mod 2), so m is THE
/// minimal polynomial of b; tests re-assert both): m(y) = y⁵ + y⁴ −
/// 4y³ − 3y² + 3y + 1, whence the reduction rows [`RED11`].
///
/// disc(m) = 11⁴ = 14641 (Sylvester/Bareiss, the Rust test) — the field
/// discriminant of the real subfield of Q(ζ₁₁): conductor 11 prime, so
/// Z[b] is the full ring of integers (M178 §2). N(2 − b) = m(2) = 11
/// prime ⇒ 2 − b is not a square ⇒ w ∉ K₁₁ (genuine quadratic
/// extension, design §4.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct K11(pub [i64; 5]);

/// Minpoly of b, low→high: y⁵ + y⁴ − 4y³ − 3y² + 3y + 1 (M178 §5.1).
pub const MK11: [i64; 6] = [1, 3, -3, -4, 1, 1];

/// Reduction rows (M178 §5.1, b⁵..b⁸ low→high, remainder-derived and
/// float-certified Python-side — re-derived independently by polynomial
/// remainder in the fresh verification pass, and re-asserted exactly by
/// the Rust tower test): basis coordinates of b⁵, …, b⁸.
pub const RED11: [[i64; 5]; 4] = [
    [-1, -3, 3, 4, -1],
    [1, 2, -6, -1, 5],
    [-5, -14, 17, 14, -6],
    [6, 13, -32, -7, 20],
];

/// Isolating interval of b (the rightmost conjugate): the consecutive
/// dyadics [112911137/2²⁶, 112911138/2²⁶] ≈ [1.682507053, 1.682507068],
/// strictly containing b ≈ 1.6825070656623624. Certified Python-side
/// (spike `spike_k11.py` B3–B5, re-run by the fresh pass): m(lo) < 0 <
/// m(hi) exactly, exactly one root of m inside (Sturm), and m′ is
/// root-free on the bracket with m′ > 0 there (Sturm count + exact
/// interval positivity — the fresh path) so m is strictly increasing:
/// the fact [`ksign`]'s bisection consumes. s = 26 is the M178 constant:
/// the 55-element battery is single-pass decisive over it with
/// accumulators ≤ 113 bits. **Envelope erratum (exact audit — see
/// [`k11_k13_sign_engine_envelope`]):** the binding i128 wall of the
/// *m-side* evaluator [`poly_sign_at`] is the near-root Horner
/// INTERMEDIATE riding at |m(b) − m₀| ≈ 1 on the scale 2^{5s} — 131
/// bits at these endpoints — not the final value (M178 §2.2's
/// 112/115/118/124 walls count final values and reproduce exactly; the
/// erratum landed with the doc). **Landing (option C): the constant
/// stands at s = 26** — endpoint certification runs through the exact
/// 192-bit scan [`poly_scan_192`] in [`check_iso_bracket`]; the
/// 0-bisection battery keeps the fallback dead, and [`poly_sign_at`]'s
/// steps are now checked (loud panic, never a silent release wrap).
/// Full-contract grids (endpoint + CAP = 4 intermediates ≤ 127):
/// s = 21 (peaks 106/111–126), or s = 25 endpoint-only (126) with a
/// dead fallback — documented, not shipped.
pub const ISO11: Iso = Iso {
    lo_num: 112911137,
    hi_num: 112911138,
    s: 26,
    m_increasing: true,
};

/// Float value of a K11 element (cross-check/emission use only).
#[inline]
pub fn kval11(x: [i64; 5]) -> f64 {
    x[0] as f64
        + x[1] as f64 * B11_F
        + x[2] as f64 * (B11_F * B11_F)
        + x[3] as f64 * (B11_F * B11_F * B11_F)
        + x[4] as f64 * (B11_F * B11_F * B11_F * B11_F)
}

impl K11 {
    /// The zero element.
    pub const ZERO: K11 = K11([0, 0, 0, 0, 0]);
    /// The unit element.
    pub const ONE: K11 = K11([1, 0, 0, 0, 0]);
    /// The generator b = 2cos(2π/11).
    pub const B: K11 = K11([0, 1, 0, 0, 0]);

    /// Exact zero test — equality is never numeric (doc §5.1).
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == [0, 0, 0, 0, 0]
    }

    /// Field addition.
    #[inline]
    pub fn add(self, y: K11) -> K11 {
        K11(kadd(self.0, y.0))
    }

    /// Field subtraction.
    #[inline]
    pub fn sub(self, y: K11) -> K11 {
        K11(ksub(self.0, y.0))
    }

    /// Field negation.
    #[inline]
    pub fn neg(self) -> K11 {
        K11(kneg(self.0))
    }

    /// Scaling by an integer (exact).
    #[inline]
    pub fn scale(self, c: i64) -> K11 {
        K11(kscale(self.0, c))
    }

    /// Field multiplication (convolution + [`RED11`] fold).
    #[inline]
    pub fn mul(self, y: K11) -> K11 {
        K11(kmul(self.0, y.0, &RED11))
    }

    /// Exact sign — the isolating-interval engine ([`ksign`] with the
    /// K₁₁ constants).
    pub fn sign(&self) -> Ordering {
        ksign(self.0, &MK11, &ISO11)
    }

    /// Exact norm N(x) = Res(m, x) (Sylvester/Bareiss, i128).
    pub fn norm(&self) -> i128 {
        knorm(&MK11, &self.0)
    }

    /// Float value — cross-check and emission use only.
    #[inline]
    pub fn float(&self) -> f64 {
        kval11(self.0)
    }
}

// =====================================================================
// K13 — Q(b), b = 2cos(2π/13); degree 6; denominator 1. The odd-prime
// cyclotomic field of the M179 design (docs/cyclotomic-field-odd-n.md):
// additive-only infrastructure at landing, zero consumers — the
// K9/M171 pattern (the future n = 13 monster geometry lives in its
// quadratic extension by w = 2sin(π/13), w² = 2 − b; design §5.2).
// =====================================================================

/// Float value of the generator b = 2cos(2π/13) (cross-check/emission
/// use only).
const B13_F: f64 = 1.7709120513064198;

/// An element of K₁₃ = Q(b), b = 2cos(2π/13): the tuple
/// `K13([c0..c5])` is the field element c0 + c1·b + … + c5·b⁵ at global
/// denominator 1.
///
/// Minpoly (M179-certified — Laurent identity x⁶·m(x + x⁻¹) = Φ₁₃
/// exactly + irreducible (Rabin test mod 2); tests re-assert both):
/// m(y) = y⁶ + y⁵ − 5y⁴ − 4y³ + 6y² + 3y − 1 (Newton sums
/// p₁..p₆ = −1, 11, −4, 31, −16, 98), whence the reduction rows
/// [`RED13`]. disc(m) = 13⁵ = 371293 (Sylvester/Bareiss; also the
/// conductor-discriminant chain disc(F₃)² · disc(Q(√13)) = 13⁴ · 13
/// through the cubic-period subfield — certified in the subfield test).
/// The quadratic subfield Q(√13) is generated by the Gauss period η =
/// Σ χ(t)·C_t = [5, −4, −8, 2, 2, 0] with η² = 13 exactly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct K13(pub [i64; 6]);

/// Minpoly of b, low→high: y⁶ + y⁵ − 5y⁴ − 4y³ + 6y² + 3y − 1 (M179
/// §5.1).
pub const MK13: [i64; 7] = [-1, 3, 6, -4, -5, 1, 1];

/// Reduction rows (M179 §5.1, b⁶..b¹⁰ low→high; fold amplification
/// L1-norms 20/25/111/152/491, each row float-certified and re-derived
/// by polynomial remainder in the fresh verification pass).
pub const RED13: [[i64; 6]; 5] = [
    [1, -3, -6, 4, 5, -1],
    [-1, 4, 3, -10, -1, 6],
    [6, -19, -32, 27, 20, -7],
    [-7, 27, 23, -60, -8, 27],
    [27, -88, -135, 131, 75, -35],
];

/// Isolating interval of b (the rightmost conjugate): the consecutive
/// dyadics [1856935/2²⁰, 1856936/2²⁰] ≈ [1.77091121673, 1.77091217041],
/// strictly containing b ≈ 1.7709120513064198.
///
/// **Why s = 20 and NOT the 26 the deg-3/4/5 fields use** (M179 §2.2 —
/// the design mandates this note): at s = 26 the interval-Horner
/// accumulator for the α⁵ monomial alone reaches 135 bits — over the
/// 127-bit i128 ceiling — so the s = 26 bracket [118843896/2²⁶,
/// 118843897/2²⁶] (re-certified exact, the M178 constant) is
/// structurally unusable for D = 6. The s₀ was chosen by the
/// pre-registered three-part rule over the s-ladder: **R1** every
/// battery element AND every comparator resolves single-pass at s
/// (holds through s = 26); **R2** the bisection mid-evals
/// [`poly_sign_at`] along the true CAP = 4 path stay ≤ 127 bits
/// (element-height-independent, m-side) — this is the BINDING gate: s =
/// 21 and 22 hit walls of 130/135 bits in round 4, s = 20 is the
/// largest passing value; **R3** pick the largest qualifying s. The
/// certified mid-eval walls at s₀ = 20 are **110/114/110/125** bits
/// through rounds 1–4 (final-value bits — reproduced exactly, and
/// asserted, by the Rust envelope test), and the whole 59-element
/// battery sits at 113-bit accumulators, single-pass. Note from the impl-time exact
/// audit: the R2 figures count final accumulator values; the Horner
/// INTERMEDIATES at s₀ = 20 peak at 121 (endpoints) / 127 (round 1) /
/// 133–145 (rounds 2–4, dead code under the certified 0-bisection
/// battery) — the erratum landed in M179 §2.2 with this commit; s₀ = 20
/// remains valid for every path the engine actually executes, and under
/// the option-C bracket check its endpoints (121 bits) still run through
/// the shipped i128 evaluator with parity evidence.
/// Certified Python-side (spike `spike_k13.py` F0–F6 with the Sturm
/// isolation + strict-monotonicity chain, re-run by the fresh
/// verification pass); m(lo) < 0 < m(hi) exactly and m′ > 0 on the
/// bracket — the facts [`ksign`] consumes.
pub const ISO13: Iso = Iso {
    lo_num: 1856935,
    hi_num: 1856936,
    s: 20,
    m_increasing: true,
};

/// Float value of a K13 element (cross-check/emission use only).
#[inline]
pub fn kval13(x: [i64; 6]) -> f64 {
    x[0] as f64
        + x[1] as f64 * B13_F
        + x[2] as f64 * (B13_F * B13_F)
        + x[3] as f64 * (B13_F * B13_F * B13_F)
        + x[4] as f64 * (B13_F * B13_F * B13_F * B13_F)
        + x[5] as f64 * (B13_F * B13_F * B13_F * B13_F * B13_F)
}

impl K13 {
    /// The zero element.
    pub const ZERO: K13 = K13([0, 0, 0, 0, 0, 0]);
    /// The unit element.
    pub const ONE: K13 = K13([1, 0, 0, 0, 0, 0]);
    /// The generator b = 2cos(2π/13).
    pub const B: K13 = K13([0, 1, 0, 0, 0, 0]);

    /// Exact zero test — equality is never numeric (doc §5.1).
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == [0, 0, 0, 0, 0, 0]
    }

    /// Field addition.
    #[inline]
    pub fn add(self, y: K13) -> K13 {
        K13(kadd(self.0, y.0))
    }

    /// Field subtraction.
    #[inline]
    pub fn sub(self, y: K13) -> K13 {
        K13(ksub(self.0, y.0))
    }

    /// Field negation.
    #[inline]
    pub fn neg(self) -> K13 {
        K13(kneg(self.0))
    }

    /// Scaling by an integer (exact).
    #[inline]
    pub fn scale(self, c: i64) -> K13 {
        K13(kscale(self.0, c))
    }

    /// Field multiplication (convolution + [`RED13`] fold).
    #[inline]
    pub fn mul(self, y: K13) -> K13 {
        K13(kmul(self.0, y.0, &RED13))
    }

    /// Exact sign — the isolating-interval engine ([`ksign`] with the
    /// K₁₃ constants).
    pub fn sign(&self) -> Ordering {
        ksign(self.0, &MK13, &ISO13)
    }

    /// Exact norm N(x) = Res(m, x) (Sylvester/Bareiss, i128).
    pub fn norm(&self) -> i128 {
        knorm(&MK13, &self.0)
    }

    /// Float value — cross-check and emission use only.
    #[inline]
    pub fn float(&self) -> f64 {
        kval13(self.0)
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
    /// numerics. The endpoint signs come from the exact 192-bit Horner
    /// scan [`poly_scan_192`] (same step semantics as the shipped
    /// [`poly_sign_at`], unbounded width — the only form that certifies
    /// [`ISO11`] @ s = 26, whose near-root intermediates need 131 bits);
    /// wherever the scan shows the i128 envelope fits (peak ≤ 127 —
    /// every field but [`ISO11`]) the shipped evaluator is additionally
    /// parity-checked, so each certification keeps engine evidence.
    /// Then the float sanity of the pasted dyadic endpoints
    /// (width ≤ `max_w`) and a smoke pass of the engine on the obvious
    /// elements.
    fn check_iso_bracket<const D: usize>(
        m: &[i64],
        iso: &Iso,
        bf: f64,
        max_w: f64,
        sign: impl Fn([i64; D]) -> Ordering,
    ) {
        let (sg_lo, _, pk_lo) = poly_scan_192(m, iso.lo_num, iso.s);
        let (sg_hi, _, pk_hi) = poly_scan_192(m, iso.hi_num, iso.s);
        assert_eq!(sg_lo, -1, "m(lo) < 0");
        assert_eq!(sg_hi, 1, "m(hi) > 0");
        if pk_lo.max(pk_hi) <= 127 {
            assert_eq!(
                poly_sign_at(m, iso.lo_num, iso.s),
                sg_lo,
                "i128 engine parity at lo"
            );
            assert_eq!(
                poly_sign_at(m, iso.hi_num, iso.s),
                sg_hi,
                "i128 engine parity at hi"
            );
        }
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

    /// (4) the 44-element L₁₀ sign/order battery — the degree-4 girih
    /// field's operating range, derived from its REAL consumers rather
    /// than seeded noise (the K₇/K₉/K₁₅ `check_sign_order` precedent,
    /// fields.rs:1627/:1753/:1912, re-pinned here for L₁₀). Every tuple
    /// is a distinct numerator the shipped `medallion/girih_m161.rs`
    /// consumer paths can produce:
    ///   raw /64 hull grid coordinates (`HULL_TBL`, raw `Kp` /64 tuples);
    ///   shipped ROTM /64 entries (the ANCHOR_ROTM validation target);
    ///   the exactness pins — cos² 54° = (5−a²)/4, doubled area = 5a³−10a,
    ///   circumradius² = a²−1, the unit element, a² and one(4);
    ///   and the strap/exit-census/convexity/outward products (the gcd-reduced
    ///   Kq numerators at the `sgn()` call sites of `medallion/girih_m161.rs`:
    ///   the strap closure walk, `exit_direction_census`, the strict
    ///   convexity and outward-exit checks).
    ///
    /// Coefficient-height certification (STOP-REPORT §3(a)): max |cᵢ| of
    /// any battery element = 160 (raw); max |cᵢ| of any pairwise difference
    /// (the exact-sort comparator inputs, ksign(ksub(a, b))) = 256;
    /// 946 ordered distinct pairwise differences over the 44 distinct
    /// numerators (raw /64 elements reach |cᵢ| = 128–160); 44/44
    /// distinct floats (min |value| 1.0, min pairwise gap 1.719e-2); 0
    /// exact-vs-float sign mismatches; and every element and every
    /// comparator difference resolves in a single pass over [`ISO_L10`] —
    /// no bisection needed, i.e. the whole battery sits comfortably inside
    /// the deg-4 s₀ = 26 envelope ([`BISECT_CAP`]). Certified
    /// independently (exact Fraction dyadic interval arithmetic as a second
    /// path) in scratch `.scratch/l10-battery/derive_l10.py` — the same
    /// discipline the K₇/K₉/K₁₅ batteries carry (`derive_k9k15l10.py`).
    const BATTERY_L10: [L10; 44] = [
        // raw /64 hull grid coordinates (Kp /64 tuples, max heights).
        L10([160, 0, -64, 0]),   // raw hull raw[8].0
        L10([0, -128, 0, 64]),   // raw hull raw[6].1
        L10([-96, 0, 64, 0]),    // raw hull raw[3].0
        L10([0, -32, 0, 32]),    // raw hull raw[7].1
        L10([0, 0, 32, 0]),      // raw hull raw[4].0
        L10([0, -64, 0, 32]),    // raw hull raw[8].1
        // shipped ROTM /64 entries (the ANCHOR_ROTM validation target).
        L10([64, 0, 0, 0]),      // ROTM entry [41]
        L10([-64, 0, 0, 0]),     // ROTM entry [42]
        L10([0, -32, 0, 0]),     // ROTM entry [45]
        L10([64, 0, -32, 0]),    // ROTM entry [48]
        L10([-64, 0, 32, 0]),    // ROTM entry [49]
        L10([0, -96, 0, 32]),    // ROTM entry [50]
        L10([0, 96, 0, -32]),    // ROTM entry [51]
        L10([0, 32, 0, 0]),      // ROTM entry [52]
        L10([-96, 0, 32, 0]),    // ROTM entry [53]
        L10([96, 0, -32, 0]),    // ROTM entry [54]
        // exactness pins at the girih_m161 consumer call sites.
        L10([5, 0, -1, 0]),      // cos² 54° = (5 − a²)/4
        L10([0, -10, 0, 5]),     // doubled area = 5a³ − 10a
        L10([-1, 0, 1, 0]),      // circumradius² = a² − 1
        L10([1, 0, 0, 0]),       // unit element (hull origins)
        L10([0, 0, 1, 0]),       // a² — cos/sin 18° identity operand
        L10([4, 0, 0, 0]),       // one(4) — 4·cos² cross-check
        // strap / exit / convexity / outward / hull-reduced numerators.
        L10([0, 17, 0, -12]),    // strap [9,2][6] ns
        L10([0, 13, 0, -10]),    // strap [9,2][7] ns
        L10([0, 7, 0, -6]),      // strap [9,2][8] ns
        L10([0, 7, 0, -4]),      // strap [9,2][0] (den.sub(ns))
        L10([0, 3, 0, -2]),      // strap [9,2][9] ns
        L10([0, 1, 0, -1]),      // strap [9,2][8] den
        L10([0, 2, 0, -1]),      // strap [9,2][6] den
        L10([0, -3, 0, 1]),      // strap [9,2][8] nt
        L10([0, -2, 0, 1]),      // strap [9,2][5] den
        L10([0, -5, 0, 2]),      // strap [9,2][7] nt
        L10([0, -1, 0, 1]),      // strap [9,2][3] den
        L10([0, -3, 0, 2]),      // strap [9,2][6] nt
        L10([0, -5, 0, 3]),      // strap [9,2][5] nt
        L10([0, -7, 0, 4]),      // strap [9,2][1] ns
        L10([5, 0, -4, 0]),      // exit[7].0 x-coordinate numerator
        L10([5, 0, -3, 0]),      // exit[8].0 x-coordinate numerator
        L10([-5, 0, 3, 0]),      // exit[3].0 x-coordinate numerator
        L10([-5, 0, 4, 0]),      // exit[2].0 x-coordinate numerator
        L10([-15, 0, 11, 0]),    // outward[9] (kdot(exit ray, contact − centre))
        L10([5, 0, -2, 0]),      // hull[8].0 (gcd-reduced Kq numerator)
        L10([2, 0, -1, 0]),      // hull[9].0 (gcd-reduced Kq numerator)
        L10([-3, 0, 2, 0]),      // hull[3].0 (gcd-reduced Kq numerator)
    ];

    /// (4) exact signs match float signs; floats distinct; the exact
    /// total order (comparator = sign(x − y), doc §5.4) matches the
    /// float sort — the L₁₀ certificate for the consumer-derived
    /// operating range above.
    #[test]
    fn l10_sign_order_battery_matches_float() {
        let cells: [[i64; 4]; 44] = BATTERY_L10.map(|k| k.0);
        check_sign_order(&cells, &ML10, &ISO_L10, kval10);
    }

    // =================================================================
    // K11 / K13 — odd-prime cyclotomic K-layers (M178/M179 design docs,
    // additive-only infrastructure at landing, zero consumers — the
    // K9/M171 pattern).
    // =================================================================

    /// m(x + x⁻¹) == x^{−D}·Φₙ == Σ_{k=−D..D} x^k EXACTLY in integer
    /// Laurent arithmetic (Horner over a windowed exponent vector): m is
    /// the minimal polynomial of 2cos(2π/n) once irreducibility is pinned
    /// ([`m11_m13_irreducible_rabin_mod2`], M178 §2 / M179 §2.1).
    fn laurent_is_all_ones(m: &[i64]) -> bool {
        let d = m.len() - 1;
        let win = 2 * d + 3;
        let off = d + 1;
        let mut p = vec![0i64; win];
        for &c in m.iter().rev() {
            let mut q = vec![0i64; win];
            for i in 0..win {
                if p[i] != 0 {
                    q[i + 1] += p[i];
                    q[i - 1] += p[i];
                }
            }
            q[off] += c;
            p = q;
        }
        (0..win).all(|i| p[i] == ((off - d..=off + d).contains(&i)) as i64)
    }

    /// Rabin irreducibility over F₂ (bitmask polynomial arithmetic):
    /// m̄ squarefree AND gcd(x^{2^k} − x, m̄) == 1 for k = 1..=d/2. Both
    /// m₁₁ and m₁₃ are irreducible mod 2 (the fresh verification path
    /// found p = 2 as the witness prime for both; M178 §2, M179 §7.2
    /// applies the same test shape to the extension polynomial).
    fn rabin_irreducible_mod2(m: &[i64]) -> bool {
        fn pmul2(a: u64, b: u64) -> u64 {
            let (mut r, mut a, mut b) = (0u64, a, b);
            while b != 0 {
                if b & 1 != 0 {
                    r ^= a;
                }
                a <<= 1;
                b >>= 1;
            }
            r
        }
        fn pmod2(mut a: u64, f: u64) -> u64 {
            let df = 63 - f.leading_zeros();
            while a != 0 && 63 - a.leading_zeros() >= df {
                a ^= f << (63 - a.leading_zeros() - df);
            }
            a
        }
        fn gcd2(mut a: u64, mut b: u64) -> u64 {
            while b != 0 {
                let t = pmod2(a, b);
                a = b;
                b = t;
            }
            a
        }
        let d = m.len() - 1;
        let f: u64 = m
            .iter()
            .enumerate()
            .map(|(i, &c)| ((c & 1) as u64) << i)
            .sum();
        if f >> d != 1 {
            return false; // degenerate mod 2 (leading coeff must stay 1)
        }
        // squarefree: gcd(m, m′) == 1 in F₂[x] (m′ keeps odd-degree terms).
        let mut deriv = 0u64;
        for i in (1..=d).step_by(2) {
            if m[i] & 1 != 0 {
                deriv |= 1u64 << (i - 1);
            }
        }
        if gcd2(f, deriv) != 1 {
            return false;
        }
        // gcd(x^{2^k} − x, m) == 1 for k = 1..=d/2, Frobenius-iterated.
        let mut h = pmod2(0b100, f); // x² mod m
        for k in 1..=d / 2 {
            if k > 1 {
                h = pmod2(pmul2(h, h), f);
            }
            if gcd2(h ^ 0b10, f) != 1 {
                return false;
            }
        }
        true
    }

    /// The Chebyshev tower over the RED table: C₀ = 2, C₁ = b,
    /// C_k = b·C_{k−1} − C_{k−2} closes exactly — C_k == C_{n−k} for
    /// k = 1..=(n−1)/2 and C_n == 2 (2cos(2πn/n) = 2cos(2π)). This is the
    /// bⁿ-fold reduction the RED rows encode; it holds only when every
    /// pasted row is the true remainder (M178 §3, M179 §2.3).
    fn check_red_tower_wrap<const D: usize>(red: &[[i64; D]], n: usize) {
        let (two, gen) = (kscale(kbasis::<D>(0), 2), kbasis::<D>(1));
        let mut c = vec![two, gen];
        for _ in 2..=n {
            let k = c.len();
            c.push(ksub(kmul(gen, c[k - 1], red), c[k - 2]));
        }
        for k in 1..=(n - 1) / 2 {
            assert_eq!(c[k], c[n - k], "tower: C_{k} != C_({n}−k)");
        }
        assert_eq!(c[n], two, "tower: C_{n} != 2");
    }

    /// One single-pass interval evaluation of every element and every
    /// pairwise difference of `cells` over `iso` (no bisection): returns
    /// (undecided count, peak accumulator bits) — the 0-bisection
    /// certification bar and the i128 envelope load at the certified s.
    fn single_pass_audit<const D: usize>(cells: &[[i64; D]], iso: &Iso) -> (usize, u32) {
        let mut undecided = 0usize;
        let mut peak = 0u32;
        for i in 0..cells.len() {
            for j in i..cells.len() {
                let x = if i == j {
                    cells[i]
                } else {
                    ksub(cells[i], cells[j])
                };
                let (vl, vh) = interval_horner(&x, iso.lo_num, iso.hi_num, iso.s);
                let b = (128 - vl.unsigned_abs().leading_zeros())
                    .max(128 - vh.unsigned_abs().leading_zeros());
                if b > peak {
                    peak = b;
                }
                // undecided ⇔ vl ≤ 0 ≤ vh (ksign decides iff vh < 0 or vl > 0)
                if vl <= 0 && vh >= 0 {
                    undecided += 1;
                }
            }
        }
        (undecided, peak)
    }

    /// Exact float statistics of a battery: (min |value|, min pairwise gap).
    fn float_stats<const D: usize>(cells: &[[i64; D]], fval: impl Fn([i64; D]) -> f64) -> (f64, f64) {
        let vals: Vec<f64> = cells.iter().map(|c| fval(*c)).collect();
        let mut minabs = f64::INFINITY;
        let mut gap = f64::INFINITY;
        for (i, &v) in vals.iter().enumerate() {
            minabs = minabs.min(v.abs());
            for &w in &vals[i + 1..] {
                gap = gap.min((v - w).abs());
            }
        }
        (minabs, gap)
    }

    /// Battery (55 elements, M178 §4.1/§6.1 construction families):
    /// 18 raw /64 hull-grid tuples + 14 exact field pins (Chebyshev tower
    /// C₂..C₅, w² = 2 − b and its companions, basis/rational shapes) + 23
    /// strap/exit/top-degree numerator shapes. Fresh verification pass
    /// (scratch `.scratch/oddn-fields-impl/verify_impl.py`, independent
    /// second path to the banked `spike_k11.py`) pins: 55/55 distinct,
    /// max coefficient height 160, min |value| 0.2846, min comparator gap
    /// 0.05592; every element and all 1485 comparator differences resolve
    /// SINGLE-PASS over [`ISO11`] — 0 bisections — with Horner
    /// accumulators peaking at 112 (element) / 113 (difference) bits of
    /// the 127-bit i128 ceiling.
    const BATTERY_K11: [K11; 55] = [
        // (i) raw /64 hull-grid tuples (max height 160).
        K11([160, 0, -64, 0, 32]),
        K11([0, -128, 0, 64, -32]),
        K11([-96, 0, 64, 0, -16]),
        K11([0, -32, 0, 32, 0]),
        K11([0, 0, 32, 0, -8]),
        K11([0, -64, 0, 32, 8]),
        K11([64, 0, 0, 0, 0]),
        K11([-64, 0, 0, 0, 0]),
        K11([0, -32, 0, 0, 0]),
        K11([64, 0, -32, 0, 0]),
        K11([-64, 0, 32, 0, 0]),
        K11([0, -96, 0, 32, 0]),
        K11([0, 96, 0, -32, 0]),
        K11([0, 32, 0, 0, 0]),
        K11([-96, 0, 32, 0, 0]),
        K11([96, 0, -32, 0, 0]),
        K11([0, 0, 0, 64, -32]),
        K11([0, 0, -64, 0, 16]),
        // (ii) exact field pins.
        K11([-2, 0, 1, 0, 0]), // C₂ = b² − 2
        K11([0, -3, 0, 1, 0]), // C₃ = b³ − 3b
        K11([2, 0, -4, 0, 1]), // C₄
        K11([-1, 2, 3, -1, -1]), // C₅
        K11([2, -1, 0, 0, 0]), // w² = 2 − b (N = 11, M178 §4.1)
        K11([4, 0, -1, 0, 0]), // 4 − b² = (2 − b)(2 + b)
        K11([5, 0, -1, 0, 0]), // 5 − b²
        K11([-1, 0, 1, 0, 0]), // b² − 1
        K11([4, 0, 0, 0, 0]),  // rational 4
        K11([1, 0, 0, 0, 0]),  // unit
        K11([0, 0, 1, 0, 0]),  // b²
        K11([0, 1, 0, 0, 0]),  // generator b
        K11([-3, 0, 0, 1, 0]), // b³ − 3
        K11([-15, 0, 11, 0, 0]), // 11b² − 15
        // (iii) strap / exit / top-degree numerator shapes.
        K11([0, 17, 0, -12, 0]), // 17b − 12b³
        K11([0, 13, 0, -10, 0]), // 13b − 10b³
        K11([0, 7, 0, -6, 0]),   // 7b − 6b³
        K11([0, 7, 0, -4, 0]),   // 7b − 4b³
        K11([0, 3, 0, -2, 0]),   // 3b − 2b³
        K11([0, 1, 0, -1, 0]),   // b − b³
        K11([0, 2, 0, -1, 0]),   // 2b − b³
        K11([0, -2, 0, 1, 0]),   // b³ − 2b
        K11([0, -5, 0, 2, 0]),   // 2b³ − 5b
        K11([0, -1, 0, 1, 0]),   // b³ − b
        K11([0, -3, 0, 2, 0]),   // 2b³ − 3b
        K11([0, -5, 0, 3, 0]),   // 3b³ − 5b
        K11([0, -7, 0, 4, 0]),   // 4b³ − 7b
        K11([5, 0, -4, 0, 0]),   // 5 − 4b² (exit x-numerator)
        K11([5, 0, -3, 0, 0]),   // 5 − 3b²
        K11([-5, 0, 3, 0, 0]),   // 3b² − 5
        K11([-5, 0, 4, 0, 0]),   // 4b² − 5
        K11([5, 0, -2, 0, 0]),   // 5 − 2b²
        K11([-3, 0, 2, 0, 0]),   // 2b² − 3
        K11([0, -9, 0, 5, 0]),   // 5b³ − 9b
        K11([0, 0, 0, 0, 3]),    // 3b⁴ (top-degree probe)
        K11([0, 0, 0, -2, 5]),   // 5b⁴ − 2b³
        K11([1, 1, 1, 1, 1]),    // all-ones shape
    ];

    /// (1) m(b) == 0 exactly on the tuple model; the pasted [`RED11`]
    /// rows are exactly b⁵..b⁸ under [`kmul`]; float cross-checks; the
    /// B11_F float constant; the norm identities N(1) = 1,
    /// N(b) = (−1)⁵·m₀ = −1 with multiplicativity on integral probes;
    /// N(2 − b) = m(2) = 11 prime (the genuine-quadratic certificate of
    /// M178 §4.1: w ∉ K₁₁); and the RED tower wrap C_k == C_{11−k},
    /// C₁₁ = 2.
    #[test]
    fn k11_minpoly_red_rows_and_norm() {
        check_minpoly_red(&MK11, &RED11, kval11, B11_F);
        assert!(
            (B11_F - 2.0 * (2.0 * PI / 11.0).cos()).abs() < 1e-15,
            "B11_F constant"
        );
        check_norm_identities(
            &MK11,
            &RED11,
            &[[2, -1, 0, 0, 0], [-3, 0, 1, 0, 0], [1, 1, 0, -1, 0]],
            -1,
        );
        assert_eq!(knorm(&MK11, &[2, -1, 0, 0, 0]), 11, "N(2 − b) != 11");
        check_red_tower_wrap(&RED11, 11);
    }

    /// Sign-engine certificate, exact (see [`check_iso_bracket`]); width
    /// bound is the deg-5 s = 26 envelope (2⁻²⁶ ≈ 1.49e-8). The endpoint
    /// re-assertion runs through the exact 192-bit scan [`poly_scan_192`]
    /// — ISO11's near-root Horner intermediates need 131 bits at s = 26
    /// ([`k11_k13_sign_engine_envelope`]); every field whose endpoints fit
    /// i128 additionally parity-checks the shipped [`poly_sign_at`] here.
    #[test]
    fn k11_iso_interval_brackets_b_exactly() {
        check_iso_bracket(&MK11, &ISO11, B11_F, 2e-8, |x| K11(x).sign());
    }

    /// (4) exact signs match float signs; floats distinct; the exact
    /// total order matches the float sort (shared checker).
    #[test]
    fn k11_sign_order_battery_matches_float() {
        let cells: [[i64; 5]; 55] = BATTERY_K11.map(|k| k.0);
        check_sign_order(&cells, &MK11, &ISO11, kval11);
    }

    /// Certification bar: every [`BATTERY_K11`] element and every one of
    /// the 1485 comparator differences is decided in a SINGLE pass over
    /// [`ISO11`] (0 bisections), i128 accumulators ≤ 113 bits — and the
    /// battery statistics the fresh path pinned (55 distinct, max height
    /// 160, min |value| 0.2846, min gap 0.05592) hold here.
    #[test]
    fn k11_battery_single_pass_zero_bisection() {
        let cells: [[i64; 5]; 55] = BATTERY_K11.map(|k| k.0);
        let mut seen = cells.to_vec();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen.len(), 55, "battery must be a set (55 distinct)");
        assert_eq!(
            cells
                .iter()
                .map(|c| c.iter().map(|v| v.abs()).max().unwrap())
                .max()
                .unwrap(),
            160,
            "max coefficient height"
        );
        let (undecided, peak) = single_pass_audit(&cells, &ISO11);
        assert_eq!(undecided, 0, "0-bisection bar broken at s = 26");
        assert!(peak <= 127, "i128 envelope breached: {peak} bits");
        assert_eq!(peak, 113, "pair-side accumulator peak");
        let (minabs, gap) = float_stats(&cells, kval11);
        assert!((minabs - 0.2846).abs() < 1e-3, "min |value| {minabs}");
        assert!((gap - 0.05592).abs() < 1e-4, "min comparator gap {gap}");
    }

    /// Battery (59 elements; 62 constructed, 3 exact duplicates dropped —
    /// the u² − (2−b)v² cores collide in the classes (1, ±3)/(−1, 3) and
    /// (±1, 2): the pinned drops are (−17,9,0,0,0,0) twice and
    /// (−7,4,0,0,0,0) once). Construction families (M179 §6.1, re-run
    /// independently by scratch `verify_impl.py` against banked
    /// `spike_k13.py`): 6 tower C₁..C₆ ×32, 6 qⱼ·cos-half y-numerators
    /// ×32, 6 synthetic /64 hull tuples, 17 exact pins (tower, 2 − b,
    /// η = √13, cubic periods A₀..A₂), 16 strap/exit shapes (incl. the
    /// b⁵ envelope probe), 4 u² − (2−b)v² cores, 4 hull-edge u² cores
    /// (height 4096). Fresh-path statistics (asserted below): min |value|
    /// 0.08365, min comparator gap 0.01199, every element and all 1711
    /// comparator differences SINGLE-PASS at s₀ = 20, accumulator peak
    /// 113 bits, L1 norms ≤ 13312 (element) / 20480 (pair) under the
    /// 2^{127−5s₀}/x̂⁵ ≈ 7.7e6 ceiling.
    const BATTERY_K13: [K13; 59] = [
        // (i) Chebyshev tower ×32 and qⱼ·cos-half ×32 numerators.
        K13([0, 32, 0, 0, 0, 0]),             // 32·C₁
        K13([-64, 0, 32, 0, 0, 0]),           // 32·C₂
        K13([0, -96, 0, 32, 0, 0]),           // 32·C₃
        K13([64, 0, -128, 0, 32, 0]),         // 32·C₄
        K13([0, 160, 0, -160, 0, 32]),        // 32·C₅
        K13([-32, -96, 96, 128, -32, -32]),   // 32·C₆
        K13([32, 96, -96, -128, 32, 32]),     // 32·q₁·(−C₆), j = 1
        K13([32, -64, -96, 32, 32, 0]),       // 32·q₂·cos-half, j = 2
        K13([-32, -64, 32, 32, 0, 0]),        // 32·q₃·cos-half, j = 3
        K13([-32, 32, 32, 0, 0, 0]),          // 32·q₄·cos-half, j = 4
        K13([32, 32, 0, 0, 0, 0]),            // 32·q₅·cos-half, j = 5
        K13([32, 0, 0, 0, 0, 0]),             // 32·q₆·cos-half, j = 6
        // (i) synthetic /64 hull-grid tuples.
        K13([160, 0, -64, 0, 32, 0]),
        K13([0, -128, 0, 64, -32, 8]),
        K13([-96, 0, 64, 0, -16, 0]),
        K13([0, -32, 0, 32, 0, -8]),
        K13([0, 0, 32, 0, -8, 0]),
        K13([64, 0, 0, 0, 0, -16]),
        // (ii) exact field pins.
        K13([-2, 0, 1, 0, 0, 0]),         // C₂
        K13([0, -3, 0, 1, 0, 0]),         // C₃
        K13([2, 0, -4, 0, 1, 0]),         // C₄
        K13([0, 5, 0, -5, 0, 1]),         // C₅
        K13([-1, -3, 3, 4, -1, -1]),      // C₆
        K13([2, -1, 0, 0, 0, 0]),         // w² = 2 − b (N = 13)
        K13([4, 0, -1, 0, 0, 0]),         // 4 − b²
        K13([5, 0, -1, 0, 0, 0]),         // 5 − b²
        K13([-1, 0, 1, 0, 0, 0]),         // b² − 1
        K13([4, 0, 0, 0, 0, 0]),          // rational 4
        K13([1, 0, 0, 0, 0, 0]),          // unit
        K13([0, 1, 0, 0, 0, 0]),          // generator b
        K13([5, -4, -8, 2, 2, 0]),        // η = √13 (Gauss periods)
        K13([1, -4, -8, 2, 2, 0]),        // η − 4
        K13([0, 6, 0, -5, 0, 1]),         // A₀ = C₁ + C₅
        K13([-2, -3, 1, 1, 0, 0]),        // A₁ = C₂ + C₃
        K13([1, -3, -1, 4, 0, -1]),       // A₂ = C₄ + C₆
        // (iii) strap / exit / top-degree numerator shapes.
        K13([0, 17, 0, -12, 0, 3]),
        K13([0, 13, 0, -10, 0, -2]),
        K13([0, 7, 0, -6, 0, 1]),
        K13([0, 3, 0, -2, 0, 0]),
        K13([0, 1, 0, -1, 0, 0]),
        K13([0, -2, 0, 1, 0, 0]),
        K13([0, -5, 0, 2, 0, 0]),
        K13([0, -7, 0, 4, 0, 0]),
        K13([5, 0, -4, 0, 0, 0]),         // exit x-numerator
        K13([-5, 0, 3, 0, 0, 0]),         // exit x-numerator
        K13([1, 1, 1, 1, 1, 1]),          // all-ones shape
        K13([0, 0, 0, 0, 0, 1]),          // b⁵ — the s = 26 envelope probe
        K13([0, 0, 0, 0, 3, -2]),
        K13([0, 0, 0, 0, -2, 5]),
        K13([0, 0, 0, 3, 0, -5]),
        K13([0, 0, -7, 0, 0, 9]),
        // (iv) real u² − (2−b)v² cores (from the mixed-sign list).
        K13([-17, 9, 0, 0, 0, 0]),        // (u, v) = (1, ±3)/(−1, 3) class
        K13([-46, 25, 0, 0, 0, 0]),       // (2, −5)
        K13([-89, 49, 0, 0, 0, 0]),       // (3, −7)
        K13([-7, 4, 0, 0, 0, 0]),         // (±1, 2) class
        // (v) hull-edge u² cores (X-row walk, heights 4096).
        K13([4096, 0, 0, 0, 0, 0]),              // (32·C₀)²
        K13([0, 0, 1024, 0, 0, 0]),              // (32·C₁)²
        K13([4096, 0, -4096, 0, 1024, 0]),       // (32·C₂)²
        K13([1024, -3072, 3072, 4096, -1024, -1024]), // (32·C₃)²
    ];

    /// (1) m(b) == 0 exactly; the pasted [`RED13`] rows are exactly
    /// b⁶..b¹⁰ under [`kmul`] (fold amplification L1 20/25/111/152/491);
    /// float cross-checks; B13_F; norm identities with
    /// N(b) = (−1)⁶·m₀ = −1 and multiplicativity; N(2 − b) = m(2) = 13
    /// prime; and the RED tower wrap C_k == C_{13−k}, C₁₃ = 2.
    #[test]
    fn k13_minpoly_red_rows_and_norm() {
        check_minpoly_red(&MK13, &RED13, kval13, B13_F);
        assert!(
            (B13_F - 2.0 * (2.0 * PI / 13.0).cos()).abs() < 1e-15,
            "B13_F constant"
        );
        check_norm_identities(
            &MK13,
            &RED13,
            &[[2, -1, 0, 0, 0, 0], [-3, 0, 1, 0, 0, 0], [1, 1, 0, -1, 0, 0]],
            -1,
        );
        assert_eq!(knorm(&MK13, &[2, -1, 0, 0, 0, 0]), 13, "N(2 − b) != 13");
        check_red_tower_wrap(&RED13, 13);
    }

    /// Sign-engine certificate, exact (see [`check_iso_bracket`]); width
    /// bound is the s₀ = 20 envelope (2⁻²⁰ ≈ 9.54e-7) — see [`ISO13`] for
    /// why 20 is the certified constant for degree 6.
    #[test]
    fn k13_iso_interval_brackets_b_exactly() {
        check_iso_bracket(&MK13, &ISO13, B13_F, 1e-6, |x| K13(x).sign());
    }

    /// (4) exact signs match float signs; floats distinct; the exact
    /// total order matches the float sort (shared checker).
    #[test]
    fn k13_sign_order_battery_matches_float() {
        let cells: [[i64; 6]; 59] = BATTERY_K13.map(|k| k.0);
        check_sign_order(&cells, &MK13, &ISO13, kval13);
    }

    /// Certification bar (see the K₁₁ twin): 59/59 distinct (the three
    /// duplicate cores are dropped by construction, pinned in the battery
    /// comment), max height 4096, all 59 elements + 1711 comparator
    /// differences single-pass at s₀ = 20 with 113-bit peak accumulators,
    /// min |value| 0.08365, min gap 0.01199 — and the L1 envelope:
    /// element 13312 / pair 20480 against the 2^{127−5s₀}/x̂⁵ ≈ 7.7e6
    /// ceiling (≈ 376× margin).
    #[test]
    fn k13_battery_single_pass_zero_bisection() {
        let cells: [[i64; 6]; 59] = BATTERY_K13.map(|k| k.0);
        let mut seen = cells.to_vec();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen.len(), 59, "battery must be a set (59 distinct)");
        assert_eq!(
            cells
                .iter()
                .map(|c| c.iter().map(|v| v.abs()).max().unwrap())
                .max()
                .unwrap(),
            4096,
            "max coefficient height"
        );
        let (undecided, peak) = single_pass_audit(&cells, &ISO13);
        assert_eq!(undecided, 0, "0-bisection bar broken at s₀ = 20");
        assert!(peak <= 127, "i128 envelope breached: {peak} bits");
        assert_eq!(peak, 113, "accumulator peak");
        let l1 = |x: &[i64; 6]| -> i64 { x.iter().map(|c| c.abs()).sum() };
        let l1e = cells.iter().map(l1).max().unwrap();
        let mut l1p = 0i64;
        for i in 0..cells.len() {
            for j in i + 1..cells.len() {
                l1p = l1p.max(l1(&ksub(cells[i], cells[j])));
            }
        }
        assert_eq!(l1e, 13312, "max element L1");
        assert_eq!(l1p, 20480, "max pair L1");
        let xhat = ISO13.hi_num as f64 / (1u64 << ISO13.s) as f64;
        let ceiling = (1i128 << (127 - 5 * ISO13.s)) as f64 / xhat.powi(5);
        assert!((l1p as f64) * 376.0 < ceiling, "L1 envelope margin");
        let (minabs, gap) = float_stats(&cells, kval13);
        assert!((minabs - 0.08365).abs() < 1e-3, "min |value| {minabs}");
        assert!((gap - 0.01199).abs() < 1e-4, "min comparator gap {gap}");
    }

    /// Algebraic certificate, part 1: the Laurent identity
    /// xⁿ·mₙ(x + x⁻¹) = Φₙ exactly (exact integer Laurent Horner).
    #[test]
    fn m11_m13_laurent_identity_exact() {
        assert!(laurent_is_all_ones(&MK11), "m₁₁ Laurent identity");
        assert!(laurent_is_all_ones(&MK13), "m₁₃ Laurent identity");
    }

    /// Algebraic certificate, part 2: irreducibility of m₁₁ and m₁₃
    /// (Rabin test, witness prime p = 2 for both — combined with the
    /// Laurent identity this pins them as THE minimal polynomials of
    /// 2cos(2π/11) resp. 2cos(2π/13)).
    #[test]
    fn m11_m13_irreducible_rabin_mod2() {
        assert!(rabin_irreducible_mod2(&MK11), "m₁₁ reducible mod 2");
        assert!(rabin_irreducible_mod2(&MK13), "m₁₃ reducible mod 2");
    }

    /// Algebraic certificate, part 3: discriminants through the engine's
    /// own Sylvester/Bareiss layout ([`knorm`]): knorm(m₁₁, m₁₁′) = +11⁴
    /// (d = 5: Res = disc) and knorm(m₁₃, m₁₃′) = −13⁵ (d = 6:
    /// Res = (−1)^{d(d−1)/2}·disc). disc = n^{(n−3)/2} is the conductor
    /// discriminant of the real subfield ⇒ Z[b] is the full ring of
    /// integers (M178 §2, M179 §2.1).
    #[test]
    fn k11_k13_discriminants_via_knorm() {
        assert_eq!(
            knorm(&MK11, &[3, -6, -12, 4, 5]),
            11u32.pow(4) as i128,
            "disc(m₁₁) via knorm"
        );
        assert_eq!(
            knorm(&MK13, &[3, 12, -12, -20, 5, 6]),
            -(13u32.pow(5) as i128),
            "Res(m₁₃, m₁₃′) via knorm"
        );
    }

    /// K₁₃ subfield certificates (M179 §8): η = [5, −4, −8, 2, 2, 0]
    /// with η² = 13 exactly (the quadratic subfield Q(√13)); cubic
    /// periods A₀ = C₁ + C₅, A₁ = C₂ + C₃, A₂ = C₄ + C₆ with
    /// A₀ + A₁ + A₂ = −1, e₂ = −4, e₃ = −1 exactly — i.e. the cubic
    /// period polynomial h₃ = y³ + y² − 4y + 1 (disc 13², certified
    /// Python-side).
    #[test]
    fn k13_subfield_certificates() {
        let eta = K13([5, -4, -8, 2, 2, 0]);
        assert_eq!(eta.mul(eta), K13([13, 0, 0, 0, 0, 0]), "η² != 13");
        let a0 = K13([0, 6, 0, -5, 0, 1]);
        let a1 = K13([-2, -3, 1, 1, 0, 0]);
        let a2 = K13([1, -3, -1, 4, 0, -1]);
        assert_eq!(
            a0.add(a1).add(a2),
            K13([-1, 0, 0, 0, 0, 0]),
            "A₀ + A₁ + A₂ != −1"
        );
        let e2 = a0.mul(a1).add(a0.mul(a2)).add(a1.mul(a2));
        assert_eq!(e2, K13([-4, 0, 0, 0, 0, 0]), "e₂ != −4");
        assert_eq!(a0.mul(a1).mul(a2), K13([-1, 0, 0, 0, 0, 0]), "e₃ != −1");
    }

    /// Exact 192-bit signed integer (two's complement, 3×u64 limbs) —
    /// test-only evidence machinery to re-run the shipped [`poly_sign_at`]
    /// Horner beyond the i128 ceiling and measure where it breaks; the
    /// engine itself never uses it.
    #[derive(Clone)]
    struct I192([u64; 3]);

    impl I192 {
        fn from_i128(v: i128) -> Self {
            I192([v as u64, (v >> 64) as u64, if v < 0 { !0 } else { 0 }])
        }
        /// Multiply by a small factor (|m| ≤ 6 suffices for minpoly
        /// coefficients; products stay below 2^{165} on every scanned
        /// path, so the limb chain is exact).
        fn mul_small(&self, m: i64) -> Self {
            let mut l = [0u64; 3];
            let mut carry: i128 = 0;
            for i in 0..3 {
                let cur = self.0[i] as u128 as i128 * (m as i128) + carry;
                l[i] = cur as u64;
                carry = cur >> 64;
            }
            let _ = carry; // top-out carry = sign-extension + m·[self < 0]; the
                           // magnitudes above guarantee no wraparound
            I192(l)
        }
        /// Add `v << k` (sign-extended; `k` may exceed 127).
        fn add_shl(&mut self, v: i64, k: u32) {
            let (base, sh) = ((k / 64) as usize, k % 64);
            let mut carry: i128 = (v as i128) << sh;
            for i in base..3 {
                let cur = self.0[i] as u128 as i128 + carry;
                self.0[i] = cur as u64;
                carry = cur >> 64;
            }
            let _ = carry; // as in mul_small: chain exact for bounded magnitudes
        }
        fn negative(&self) -> bool {
            (self.0[2] as i64) < 0
        }
        fn signum(&self) -> i32 {
            if self.0 == [0, 0, 0] {
                0
            } else if self.negative() {
                -1
            } else {
                1
            }
        }
        /// Bit length of |self|.
        fn bitlen(&self) -> u32 {
            let a = if self.negative() {
                // two's complement negate
                let mut l = [!self.0[0], !self.0[1], !self.0[2]];
                let mut c = 1u128;
                for x in l.iter_mut() {
                    let s = *x as u128 + c;
                    *x = s as u64;
                    c = s >> 64;
                }
                I192(l)
            } else {
                self.clone()
            };
            (0..3).rev().find(|&i| a.0[i] != 0).map_or(0, |i| {
                64 * i as u32 + 64 - a.0[i].leading_zeros()
            })
        }
    }

    /// Exact re-run of the shipped [`poly_sign_at`] Horner in [`I192`]:
    /// returns (signum, final |acc| bits, peak |intermediate| bits). The
    /// peak covers every `acc·num` product, every materialized addend
    /// `mᵢ << (sa+s)` and every accumulator, so the engine's eval is
    /// debug-panic-free on i128 exactly when `peak ≤ 127` (the criterion
    /// catches both the shift-count and the value overflow, as both
    /// minpolys have |m₀| = 1).
    fn poly_scan_192(m: &[i64], num: i64, s: i32) -> (i32, u32, u32) {
        let deg = m.len() - 1;
        let mut acc = I192::from_i128(m[deg] as i128);
        let mut peak = acc.bitlen();
        let mut sa = 0i32;
        for i in (0..deg).rev() {
            let mut prod = acc.mul_small(num);
            peak = peak.max(prod.bitlen());
            if m[i] != 0 {
                peak = peak.max(64 - m[i].unsigned_abs().leading_zeros() + (sa + s) as u32);
            }
            prod.add_shl(m[i], (sa + s) as u32);
            peak = peak.max(prod.bitlen());
            acc = prod;
            sa += s;
        }
        (acc.signum(), acc.bitlen(), peak)
    }

    /// Walk the [`BISECT_CAP`] path [`ksign`] would take on an undecided
    /// cell ([`mid_num`] + keeping the root half by `m_increasing`),
    /// scanning each mid-eval in [`I192`]; returns (s′, final, peak).
    fn bisect_path_192(m: &[i64], iso: &Iso) -> Vec<(i32, u32, u32)> {
        let (mut lo, mut hi, mut s) = (iso.lo_num, iso.hi_num, iso.s);
        let mut out = Vec::new();
        for _ in 0..BISECT_CAP {
            let (mid, s2) = mid_num(lo, hi, s);
            let (sg, fin, peak) = poly_scan_192(m, mid, s2);
            out.push((s2, fin, peak));
            if s2 > s {
                lo *= 2;
                hi *= 2;
            }
            if sg > 0 {
                hi = mid; // m_increasing on both ISO intervals
            } else {
                lo = mid;
            }
            s = s2;
        }
        out
    }

    /// Envelope truth of the shipped m-side evaluator [`poly_sign_at`]
    /// near a root: the Horner intermediates ride at |m(y) − m₀| ≈ |m₀|
    /// = 1 on the scale 2^{deg·s′}, so the real i128 wall is the
    /// INTERMEDIATE peak ≈ deg·s′ + 1 — not the final-value bits the
    /// design docs tabulated. Measured exactly here (fresh audit, scratch
    /// `envelope_audit.py`): **ISO13 @ s₀ = 20** endpoints peak 121 bits
    /// (fits — the shipped bracket check passes), and along the CAP = 4
    /// path the doc walls 110/114/110/125 reproduce as final values
    /// while the intermediates peak at 127/133/139/145 — only round 1 is
    /// i128-executable (rounds 2–4 are certified-dead code, the battery
    /// being 0-bisection; M179 §2.2's "all four rounds ≤ 127" reads
    /// the final bits — erratum landed with this commit). **ISO11 @
    /// s = 26** was the STOP-gate (d) finding: the two ENDPOINT
    /// re-assertions need 131 bits, past the shipped evaluator
    /// (bracket itself is exact: scan signs −1/+1, Sturm-isolated;
    /// the battery never calls `poly_sign_at`). The
    /// corrected full-contract answers (endpoint + CAP = 4 peak ≤ 127)
    /// are s = 21 for deg 5 (peak 126) and s = 16 for deg 6; s = 25 is
    /// endpoint-only-fit (126) for deg 5. Landed under option C
    /// (single-pass-pin discipline): the constants stand, [`ISO11`]
    /// certifies through this scan in [`check_iso_bracket`], and the
    /// dead fallback is now loud (checked steps in [`poly_sign_at`])
    /// — the shipped K₇/K₉/K₁₅/L₁₀ constants all pass
    /// the full contract.
    #[test]
    fn k11_k13_sign_engine_envelope() {
        // K13 @ s₀ = 20: endpoints exact, executable, and the scan agrees
        // with the shipped evaluator; round 1 fits and matches too.
        let (sgl, fin_l, pk_l) = poly_scan_192(&MK13, ISO13.lo_num, ISO13.s);
        let (sgh, fin_h, pk_h) = poly_scan_192(&MK13, ISO13.hi_num, ISO13.s);
        assert_eq!((sgl, sgh), (-1, 1), "ISO13 brackets b exactly");
        assert_eq!((fin_l, fin_h), (105, 102));
        assert_eq!(pk_l.max(pk_h), 121, "ISO13 endpoint peak bits (i128-fits)");
        assert_eq!(poly_sign_at(&MK13, ISO13.lo_num, ISO13.s), sgl);
        assert_eq!(poly_sign_at(&MK13, ISO13.hi_num, ISO13.s), sgh);
        let p13 = bisect_path_192(&MK13, &ISO13);
        assert_eq!(
            p13.iter().map(|r| r.1).collect::<Vec<_>>(),
            vec![110, 114, 110, 125],
            "M179 wall table = FINAL value bits, reproduced exactly"
        );
        assert_eq!(
            p13.iter().map(|r| (r.0, r.2)).collect::<Vec<_>>(),
            vec![(21, 127), (22, 133), (23, 139), (24, 145)],
            "intermediate peaks — only round 1 is i128-executable"
        );
        let (mid, s2) = mid_num(ISO13.lo_num, ISO13.hi_num, ISO13.s);
        assert_eq!(
            poly_sign_at(&MK13, mid, s2),
            poly_scan_192(&MK13, mid, s2).0,
            "round-1 parity with the shipped evaluator (it fits at s′ = 21)"
        );

        // K11 @ s = 26: exact bracket, but 131-bit endpoint intermediates
        // — the gate (d) finding; certified through this scan at landing
        // (option C), see the envelope test's doc comment.
        let (sgl, fin_l, pk_l) = poly_scan_192(&MK11, ISO11.lo_num, ISO11.s);
        let (sgh, fin_h, pk_h) = poly_scan_192(&MK11, ISO11.hi_num, ISO11.s);
        assert_eq!((sgl, sgh), (-1, 1), "ISO11 brackets b exactly (I192 scan)");
        assert_eq!((fin_l, fin_h), (108, 106));
        assert_eq!((pk_l, pk_h), (131, 131), "ISO11 endpoint peak — gate (d)");
        let p11 = bisect_path_192(&MK11, &ISO11);
        assert_eq!(
            p11.iter().map(|r| r.1).collect::<Vec<_>>(),
            vec![112, 115, 118, 124],
            "M178 wall table = FINAL value bits, reproduced exactly"
        );
        assert_eq!(
            p11.iter().map(|r| (r.0, r.2)).collect::<Vec<_>>(),
            vec![(27, 136), (28, 141), (29, 146), (30, 151)],
            "intermediate peaks — every round dead at s = 26"
        );
    }
}
