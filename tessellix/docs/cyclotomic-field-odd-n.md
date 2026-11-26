# Odd-n cyclotomic field layer extension — K₁₃ = Q(2cos(2π/13)) (degree 6) and the general prime-n shape — design doc

Status: **PROPOSED** — design delivered and spike-validated; **no code in
this landing** (docs-only). Companion spike:
`.scratch/odd-n-fields-design/spike_k13.py` (stdlib-only Python, exact
integer arithmetic, **93/93 checks green**, log
`spike_k13_output.txt`, rc = 0; banked in the main checkout's
`.scratch/odd-n-fields-design/`). Scope: the full n = 13 treatment
(the first degree-6 field class), the resolution of the D = 6 i128
envelope problem that M178 flagged as an open risk, and the general
odd-n recipe that turns the per-n spike work into a uniform shape.
Predecessor docs: [`cyclotomic-field-degree5.md`](cyclotomic-field-degree5.md)
(K₁₁, M178 — this doc **replaces its §7 n = 13 stub** and corrects one
of its claims, see §7.5) and
[`cyclotomic-field-layer.md`](cyclotomic-field-layer.md)
(degrees 3/4, LANDED). Same citation markers: **[P]** = proven by exact
algebra below (the spike's assertions are the executable proof);
**[C]** = cited (shipped code, dossier, or literature).

---

## 0. One-paragraph summary

Represent n = 13 tiling points in the **oblique frame** `(U₀, U₁)` with
coordinates in the degree-6 field `K₁₃ = Q(α), α = 2cos(2π/13)`, minpoly
`m₁₃(y) = y⁶ + y⁵ − 5y⁴ − 4y³ + 6y² + 3y − 1`, ring of integers `Z[α]`
(discriminant `13⁵` — the prime-n lemma re-verified at n = 11 *and*
n = 13). **K₁₃ is not subfield-free**: `Gal(K₁₃) ≅ Z/6` gives a unique
quadratic subfield `Q(√13)` (exact tuple `η = [5, −4, −8, 2, 2, 0]`,
`η² = 13`) and a unique cubic subfield `F₃` (`h₃(y) = y³ + y² − 4y + 1`,
disc `13²`); this **corrects M178 §7** ("degree 6 prime ⇒ still no
subfields" — 6 is composite). Sines never enter the core: every sine of
a multiple of π/13 factors as `w · (K₁₃ element)` with `w = 2sin(π/13)`,
`w² = 2 − α`, a **genuine quadratic extension** (norm certificate
`N(2 − α) = m₁₃(2) = 13`, prime; the spike proves `w_n ∉ K_n` for
*every* odd n, §7.2). Records store `(u + v·w)/den` with `u, v ∈ Z[α]`
— the exact `n9::Nq` shape at two degrees up. **The headline result is
the D = 6 envelope resolution**: the K₁₁ convention `s = 26` is
*structurally impossible* at degree 6 (the monomial tuple `α⁵` alone
overflows i128 at 135 bits — no gcd trick can help), and the
spike-selected fix is the **s₀-drop to s = 20**, chosen by a
pre-registered three-part rule: at `ISO13 = [1856935, 1856936]/2²⁰` the
59-element battery is single-pass decisive with **0 bisections
everywhere** (all 1711 pairwise comparators included), accumulator
peaks **113 bits** against the 127-bit i128 ceiling, and the retained
bisection fallback (`poly_sign_at` mid-evals through `BISECT_CAP = 4`
rounds) peaks at **125 bits** — every round certified. The general
prime-n recipe (§7.1) plus the measured s_max(D) curve (§7.6) make
K₇/K₉/K₁₁/K₁₃/K₁₅/K₁₇/K₂₁ one design-level family; D = 7 never arises
(φ(n) = 14 is unsolvable), so after degree 6 the next real wall is
D = 8 (K₁₇). No Rust changes are proposed in this document.

---

## 1. Field selection and the odd-n sine rule **[P]**

- `K₁₃ := Q(ζ₁₃)⁺ = Q(α)`, `α = 2cos(2π/13)`, `deg = φ(13)/2 = 6`.
  Generator standardized at the **2π angle** (frame algebra needs it;
  deg-3/4 doc §1).
- Minimal polynomial (two independent derivations, spike A1–A2):
  `m₁₃(y) = y⁶ + y⁵ − 5y⁴ − 4y³ + 6y² + 3y − 1`.
  - *Laurent path:* `Φ₁₃(x) = x⁶·m₁₃(x + x⁻¹)` — verified
    coefficient-exact as a Laurent identity (exponents −6..6 collapse
    to 0..12; the identity-checker must drop zero coefficients after
    term cancellation — a subtle spike bug this exercise surfaced and
    fixed, §9).
  - *Newton path:* orbit power sums `pⱼ = Σₖ (ζᵏ + ζ⁻ᵏ)ʲ` via the
    Ramanujan sum `c₁₃(t) = 12 if 13|t else −1`:
    `p₁..p₆ = −1, 11, −4, 31, −16, 98`; Newton identities reproduce
    `m₁₃` integrally.
  - *Irreducibility (⇒ this is THE minpoly):* `Φ₁₃(x+1)` is Eisenstein
    at 13 ⇒ `Φ₁₃` irreducible ⇒ `[Q(ζ₁₃):Q] = 12`; ζ₁₃ non-real ⇒
    `[K₁₃:Q] = 6` = deg m₁₃ (spike A3a–A3c: also mod-2 irreducible and
    `gcd(m₁₃, m₁₃′) = 1`).
- Conjugates = `2cos(2πk/13)`, k = 1..6 (Sturm census: exactly 6 real
  roots in [−2.1, 2.1], spike F3c):

  | k | value (float, cross-check only) |
  |---|---------------------------------|
  | 1 | +1.7709120513 |
  | 2 | +1.1361294935 |
  | 3 | +0.2410733605 |
  | 4 | −0.7092097741 |
  | 5 | −1.4970214963 |
  | 6 | −1.9418836349 |

  Sum = −1 = −(coefficient of y⁵) ✓.
- **Subfields exist (Gal ≅ Z/6) — the M178 §7 correction:** deg 6 =
  φ(13)/2 is *composite*, so the prime-degree no-subfield argument
  (which holds at K₇/K₉ deg 3 and K₁₁ deg 5) **stops applying**.
  `Gal(K₁₃) = (Z/13)*/{±1} ≅ Z/6` is cyclic with exactly one subgroup
  per divisor: a unique quadratic subfield `Q(√13)` and a unique cubic
  subfield `F₃`; `K₁₃ = F₃·Q(√13)` (compositum), and the
  conductor-discriminant formula gives
  `disc(K₁₃) = disc(F₃)²·disc(Q(√13)) = 13⁴·13 = 13⁵` — consistency
  with §1's polynomial discriminant is itself a check (spike C8–C9).
  Exact witnesses in §3. Consequences: subfield probes (trace/norm
  into `F₃` or `Q(√13)`) *become available* at n = 13 for the first
  time in the odd-n layer; no current consumer needs them (§8.4).
- **Ring of integers = Z[α]:** `disc(m₁₃) = Res(m₁₃, m₁₃′) = 371293 =
  13⁵` exactly (spike B13, fraction-free Bareiss). The **prime-n
  discriminant lemma** `disc(m_p) = p^{(p−3)/2}` is verified by the
  spike for **p ∈ {3, 5, 7, 11, 13, 17, 19, 23}** — including the two
  fields this program actually lands (11⁴ at K₁₁, 13⁵ here). Equality
  of polynomial and field discriminant ⇒ index 1 ⇒ **`Z[α]` is the
  maximal order** — the D = 1 core needs no denominator, ever (same
  position as K₇/K₉/K₁₅/K₁₁; L₁₀ remains the exception).
- **Odd-n quadratic sine extension (the n9/n11 pattern at degree 6):**
  define `w = 2sin(π/13)`. Then:
  - `w² = 2 − α` **[P]** (`4sin²(θ/2) = 2 − 2cos θ` at θ = 2π/13);
  - `w ∈ Q(ζ₅₂)⁺` and `[Q(ζ₅₂)⁺ : K₁₃] = 12/6 = 2` **[P]** (φ(52) = 24);
  - the extension is **genuine**: `2 − α` is not a square in `K₁₃`,
    because a square has square norm and `N(2 − α) = m₁₃(2) = 13` is
    prime (spike F3e) — the exact n9 (`N = 3`) / n11 (`N = 11`)
    certificate one field up. §7.2 proves the stronger universal
    statement: `w_n ∉ K_n` for **every** odd n, including the
    multi-prime cases (15, 21) where the norm certificate degenerates;
  - `w > 0` exactly: Sturm shows m₁₃ root-free on [2, 2.1] with
    `m₁₃(2) = 13 > 0` ⇒ α < 2 ⇒ `s := 2 − α > 0` (spike F3d);
  - **one w suffices:** `2sin(2πk/13) = w·q_k·cos(πk/13)` with
    `q_k = sin(πk/13)/sin(π/13) ∈ Z[α]` (Chebyshev-U at `b′/2`,
    `b′ = 2cos(π/13)`; integrality certified — q-table in §3) and
    `cos(πk/13) ∈ K₁₃`. So every hull coordinate is
    `(K₁₃ element) + w·(K₁₃ element)` — the two-level shape, never a
    degree-12 flat element.

---

## 2. Element representation, reduction, isolation — and the RECOMMENDATION

### 2.1 Core tuples and reduction table

Element = `[i64; 6]`, low→high: `c₀ + c₁α + … + c₅α⁵`, denominator 1
(§1: Z[α] maximal). Minpoly low→high
`MK13: [i64; 7] = [-1, 3, 6, -4, -5, 1, 1]`. Reduction rows
`RED13: [[i64; 6]; 5]` (b⁶..b¹⁰, spike C1–C6, each row float-certified
≤ 1e-9 and the m(b) = 0 fold is exact):

| power | row (low→high) |
|-------|----------------|
| b⁶ | `[1, -3, -6, 4, 5, -1]` |
| b⁷ | `[-1, 4, 3, -10, -1, 6]` |
| b⁸ | `[6, -19, -32, 27, 20, -7]` |
| b⁹ | `[-7, 27, 23, -60, -8, 27]` |
| b¹⁰ | `[27, -88, -135, 131, 75, -35]` |

Fold amplification (`‖row‖₁`, the per-fold L1 growth the envelope
derivation consumes): `[20, 25, 111, 152, 491]` for b⁶..b¹⁰.

Wrap certificate (spike C7): computing `C_k = y·C_{k−1} − C_{k−2}`
through k = 13 with these folds reproduces `C_k(α) = C_{13−k}(α)`
**exactly** (k = 1..6) and `C₁₃(α) = 2` — the reduction rows are
consistent across the whole orbit, a strong RED13-only check (the
`C_k = C_{n−k}` symmetry is a theorem — `2cos(2πk/13) =
2cos(2π(13−k)/13)` — here certified on the tuples, as at K₁₁).

All generic ops (`kadd/ksub/kneg/kscale/kmul/koblique_*`) are already
const-generic over D in `fields.rs` — K₁₃ is a pure data + tests block
**[C]** (the L₁₀/K₁₁ block template; `MK13` carries 7 coefficients and
`RED13` has 5 rows, both one deeper than K₁₁).

### 2.2 Isolating interval and the D = 6 envelope — the s = 26 wall and its resolution

**The M178 constant is real but unusable.** `ISO13@s26 =
[118843896, 118843897]/2²⁶` re-certifies exactly (spike F1a: both
bracket endpoints by exact integer sign of `m₁₃(n/2²⁶)·2^{6·26}`, no
floats). But the **s = 26 wall is structural** (spike F1b): the single
monomial tuple `α⁵ = [0, 0, 0, 0, 0, 1]` has interval-Horner
accumulator numerator `n⁵ = 118843897⁵` — bit-length 135
(`n < 2²⁷ ⇒ n⁵ < 2^{135} = 2⁵·2^{5·26}`) — **135 bits > 127**.
The tuple is primitive (gcd 1), so **gcd-normalize cannot help**; every
degree-6 element with a nonzero top coefficient is un-evaluable at
s = 26. No accumulator widening of the *interval-Horner input* fixes a
wall that lives in the numerator scale itself.

**The s-ladder (measured, spike F2):** over the full 59-element
battery and all 1711 pairwise comparator differences:

| s | max acc bits (elem / pair) | undecided (elem / pair) |
|----|----|----|
| 17 | 98 / 98 | 0 / 0 |
| 18 | 103 / 103 | 0 / 0 |
| 19 | 108 / 108 | 0 / 0 |
| 20 | 113 / 113 | 0 / 0 |
| 21 | 118 / 118 | 0 / 0 |
| 22 | 123 / 123 | 0 / 0 |
| 23 | 128 / 128 | 0 / 0 |
| 24 | 133 / 133 | 0 / 0 |
| 25 | 138 / 138 | 0 / 0 |
| 26 | 143 / 143 | 0 / 0 |

Each step of s costs exactly 5 bits at D = 6 (`sa = (D−1)·s`): the
battery alone would tolerate s up to 23 (128 bits — already at the
edge) — but the battery is not the binding gate.

**The binding gate is the m₁₃-side mid-eval wall** (`poly_sign_at`
along the true bisection path): element-height-independent, it depends
only on m₁₃'s coefficients (~6) and the bracket location. The
**s₀ selection rule (pre-registered)**:
- **R1 (one-pass):** every battery element AND pair single-pass
  decisive at s (0 undecided);
- **R2 (bisection capability):** the `poly_sign_at` mid-evals
  (`m₁₃(mid)·2^{6s′}`) along the true CAP = 4 bisection path starting
  from ISO@s stay ≤ 127 bits;
- **R3:** take the **largest** qualifying s in 18..22 (finest
  one-pass resolution).

Result (spike F2): **s₀ = 20**. Candidates 22 (mid-eval wall 135 bits)
and 21 (130 bits) fail R2; 18, 19, 20 pass (walls 114/114/125), and R3
picks 20. An important spike finding shaped R2: the *naive*
all-elements round-gate (require every battery element to survive
CAP = 4 rounds) is **wrong** — it is element-height-dependent, and tall
decisive elements never bisect in practice; the true bisection path
only visits mid-points of the *bracket*, so the wall is m₁₃-side. At
s₀ = 20 the true-path walls are:

| bisection round | s′ | `poly_sign_at` acc bits |
|----|----|----|
| 1 | 21 | 110 |
| 2 | 22 | 114 |
| 3 | 23 | 110 |
| 4 | 24 | 125 |

— the walls tabulated here are **final-value** bits (reproduced exactly,
asserted by the Rust test `k11_k13_sign_engine_envelope`). **Erratum
(impl-time exact audit, landed with the K₁₃ code):** the binding i128
quantity is the near-root Horner *intermediate*, `|m₀|·2^{6s′}` — wall
`6s′ + 1` bits — measured 127/133/139/145 through rounds 1–4 (the
endpoint check itself: 121 bits, fits). At s₀ = 20 only **round 1** of
the fallback is therefore i128-executable: rounds 2–4 are dead code,
safe under the certified 0-bisection battery, and loud now (checked
Horner steps in `poly_sign_at`, no silent release wrap). Full-contract
grid for deg 6 (endpoint + all four rounds): s₀ = 16 (peaks 97 /
103–121) — documented, not shipped; s₀ = 20 stands under the
single-pass-pin discipline.

**`ISO13: Iso { lo_num: 1856935, hi_num: 1856936, s: 20,
m_increasing: true }`** **[P]** — Sturm-isolated (exactly one root of
m₁₃ inside), and `m₁₃′` is root-free on the bracket with `m₁₃′(lo) > 0`
⇒ strictly increasing ⇒ the bisection half-selection rule of the
shipped `ksign` applies verbatim (spike F3a–F3b). Note
`ISO@s0 = ISO@s26 ≫ 6` (118843896 ≫ 6 = 1856935): the s₀ bracket is the
coarsening of the M178 bracket, exactly as the engine's own
bisection would produce it.

**One-pass sufficiency at s₀ = 20 (the construction-level safety
criterion).** For any degree-6 tuple with L1 height `L1 = Σ|cᵢ|`,
`acc ≤ 2^{5s}·x⁵·L1` with `x = hi/2^s ≈ 1.771 < 2`, so **any** consumer
tuple is i128-safe by construction whenever
`L1 < 2^{127−5s₀}/x⁵ ≈ 7.706×10⁶`. The battery's L1 maxima are 13312
(elements) and 20480 (pair differences) — a **377× margin** under the
ceiling. The honest *precision* requirement is the μ-form
`μ = |p(α)|·2^{s₀}/|p′(α)| > 1` (the exact value dominating the
bracket-width term): measured over the 55 non-constant battery
elements, min **μ = 2332**, median **2.861×10⁵** — every interval
excludes 0 with 1000× slack. (The naive width-form
`log₂(|p′|_max/min-gap) ≈ 21.1 bits` is *not* the criterion — the
closest-gap pair carries a near-canceling derivative; the spike
measures per-pair μ = 99.9 minimum. Recorded so the impl job does not
"derive" a false precision wall from the width form.)

**Why not the alternatives (the M178 §7 options, now with evidence):**
- **(a) s₀-drop — RECOMMENDED (primary).** Drops the accumulator scale
  by 30 bits (5 bits × 6), clears both walls (battery 113 ≤ 127,
  mid-evals 125 ≤ 127), keeps the shipped i128 kernel and `BISECT_CAP
  = 4` untouched, and keeps single-pass decisiveness over the entire
  battery with μ ≥ 2332. Resolution cost: 2⁻²⁰ vs 2⁻²⁶ — still 2¹⁴
  finer than the /64 consumer grid (2⁻⁶), and the μ-margins show the
  grid gaps are resolved with 1000× slack. Battery impact: **none** —
  0 bisections everywhere (§6).
- **(b) gcd-normalize — secondary optimization only.** Cannot touch the
  monomial wall (primitive tuples); what it *does* buy is
  `2·log₂(gcd(u, v))` bits on the mixed-sign `u² − s·v²` arm of the
  extension engine — measured on the 67-element N battery: 48/67
  elements have g > 1, median saving 12 bits, max 14. Adopt it in the
  N₁₃ `sgn` arm (§4.2) as a cheap win, not as the envelope resolution.
- **(c) "better" (wider accumulators / i256): rejected.** Doubling the
  kernel accumulator width doubles the cost of every `ksign`/mul at
  every degree, to solve a wall that (a) removes entirely. The i256
  retreat also breaks the shared-engine property that makes the
  const-generic layer reviewable (one accumulator type across
  K₇/K₉/K₁₅/L₁₀/K₁₁/K₁₃).

### 2.3 The extension type

`N13 { u: K13, v: K13, den: i64 }` = `(u + v·w)/den`, `den > 0`,
gcd-reduced over all 13 integers (the `n9::Nq` convention **[C]**).
Multiplication: `(u₁ + v₁w)(u₂ + v₂w) = (u₁u₂ + s·v₁v₂) +
(u₁v₂ + u₂v₁)w` with `s = 2 − α` stored as the K₁₃ constant
`[2, -1, 0, 0, 0, 0]`. Addition/subtraction cross-multiplies
denominators; no division anywhere.

### 2.4 RECOMMENDATION

Adopt the **two-level shape at s₀ = 20**: `K13` core tuples (D = 6,
denominator 1) in `fields.rs` as a data + tests block mirroring
`L10`/`K11`, plus an `N13` quadratic-extension type in a future
`monster/n13.rs` mirroring `n9::Nq` — *not* a flat degree-12 field
(`interval_horner` at `sa = 11s`: i128 impossible at any useful s), *not*
a rational/Float retreat, *not* an i256 kernel, *not* a generic
power-of-ζ table. The **s₀-drop is the D = 6 envelope resolution**
(§2.2, spike-certified end to end); **gcd-normalize rides along in the
extension's mixed-sign arm** as an optimization (§4.2). Reasons: (a) the
§5.3 consumers need exactly `(K₁₃, w·K₁₃)` coordinates; (b) the sign
engine stays at the certified 113-bit envelope in the shipped i128
kernel; (c) the review surface is two small blocks with landed
precedents; (d) the sine discipline ("sines live as w-multiples, never
as stored field elements") carries over verbatim.

---

## 3. Exact constants and their certificates **[P]**

All tuples low→high in Z[α]. Certified exact by the spike (floats in
parentheses, cross-check only).

- **Chebyshev tower** `C_k(α) = 2cos(2πk/13)` (recursion
  `C_k = y·C_{k−1} − C_{k−2}`, `C₀ = 2`, `C₁ = y`):

  | k | tuple | float |
  |---|-------|-------|
  | 0 | `[2, 0, 0, 0, 0, 0]` | 2 |
  | 1 | `[0, 1, 0, 0, 0, 0]` | 1.77091205 |
  | 2 | `[-2, 0, 1, 0, 0, 0]` | 1.13612949 |
  | 3 | `[0, -3, 0, 1, 0, 0]` | 0.24107336 |
  | 4 | `[2, 0, -4, 0, 1, 0]` | −0.70920977 |
  | 5 | `[0, 5, 0, -5, 0, 1]` | −1.49702150 |
  | 6 | `[-1, -3, 3, 4, -1, -1]` | −1.94188363 |

  plus the wrap identities `C₇..C₁₂ = C₆..C₁`, `C₁₃ = C₀` (spike C7).
- **`b′ = 2cos(π/13) = −C₆(α) = [1, 3, -3, -4, 1, 1]`** (1.94188363) —
  the π/13-rotation cosine, in Z[α]. Half-step table:
  `cos(πj/13) = −C₆, C₁, −C₅, C₂, −C₄, C₃` for j = 1..6 (the odd
  half-steps pick up a sign from the wrap symmetry).
- **Sine ratios** `q_j = sin(πj/13)/sin(π/13) = U_{j−1}(b′/2)` — all
  in **Z[α] at denominator 1** (integrality asserted exactly; the
  `2^{j−1}·U_{j−1}(b′/2)` computation must use *raw* Chebyshev-U
  coefficients — field-folded ones are wrong from j = 6 on; same trap
  as K₁₁, spike-asserted):

  | j | q_j tuple |
  |---|-----------|
  | 1 | `[1, 0, 0, 0, 0, 0]` |
  | 2 | `[1, 3, -3, -4, 1, 1]` (= b′) |
  | 3 | `[1, 1, 0, 0, 0, 0]` (= 1 + α) |
  | 4 | `[1, -2, -3, 1, 1, 0]` |
  | 5 | `[-1, 1, 1, 0, 0, 0]` |
  | 6 | `[-1, -2, 1, 1, 0, 0]` |

  (q₇..q₁₂ = q₆..q₁ by the reflection symmetry.)
- **Angle-pair pins (factor-2 forms):** `4cos²(2π/13) = α²` and
  `4sin²(2π/13) = 4 − α² = [4, 0, -1, 0, 0, 0]`, summing to 4 exactly.
  Watch the K₁₁-documented trap: `2sin²(2π/13)` is a **half-integer**
  element — the integral pin is the factor-4 form.
- **Norm facts:** `N(α) = (−1)⁶·m₁₃(0) = −1` (α is a unit — the
  cyclotomic real-subfield unit), `N(2 − α) = m₁₃(2) = 13` (prime ⇒
  the extension certificate), `N(s) = 13` non-square ⇒ `u² = s·v²`
  has no nonzero solution in K₁₃ (§4.2).
- **The quadratic subfield:** `√13 = η = [5, -4, -8, 2, 2, 0]` — the
  Gauss period `Σ_t χ(t)·C_t` over the Legendre symbol mod 13 — with
  `η² = 13` **exact** on the tuple model (spike C8; float cross-check
  3.6055512…). This is the first exact square root in the odd-n layer
  (K₇/K₉/K₁₁ have no quadratic subfield) — a subfield *probe* the
  composite fields already use (K₁₅ ⊃ Q(√5)).
- **The cubic subfield F₃:** the index-3 Gauss periods
  `A₀ = C₁ + C₅`, `A₁ = C₂ + C₃`, `A₂ = C₄ + C₆` (floats 0.27389055,
  1.37720285, −2.65109341) satisfy `h₃(y) = y³ + y² − 4y + 1` exactly,
  `disc(h₃) = 169 = 13²` — the cyclic cubic field of conductor 13
  (spike C9a–C9d). `K₁₃ = F₃·Q(√13)`;
  `disc(K₁₃) = 13⁴·13 = 13⁵` ✓ (§1).
- **Star-walk shoelace factors** `{13/d}`: `Σⱼ (AⱼBⱼ₊₁ − Aⱼ₊₁Bⱼ)` as
  exact K₁₃ tuples (area = |factor|·sin(2π/13)/2):

  | walk | factor tuple | area (float) |
  |------|--------------|--------------|
  | {13/1} | `[7, 2, -2, 1, 3, 1]` | 13.185768328 |
  | {13/2} | `[-3, 14, 12, -19, -5, 7]` | 6.192357637 |
  | {13/3} | `[-15, 44, 8, -43, 1, 9]` | 3.668491645 |
  | {13/4} | `[-21, -32, 32, 23, -9, -3]` | 2.243313026 |
  | {13/5} | `[17, -10, -42, 21, 11, -5]` | 1.232562420 |
  | {13/6} | `[-1, 22, -22, -41, 7, 11]` | 0.394621445 |

  All six walks close exactly, are R^d-equivariant about the exact
  centroid, and the exact shoelace × sin agrees with the float area
  ≤ 1e-9 (spike E1) — the exact exit/outline machinery the n = 13
  records need.
- **The oblique frame U(k)** (spike D1–D6): `|U(k)|² = 1` exactly for
  all k, `Σ_k U(k) = 0` exactly, `R^k U(0) = U(k)`, all coordinates
  integral in `Z[b]²` — the D = 1 core carries the whole odd-n frame
  at n = 13 exactly as at n = 7/9/11 (the frame recipe of the deg-3/4
  doc §3 is uniform in n).

---

## 4. Exact sign and order — the isolating-interval machinery at D = 6

### 4.1 Core K₁₃ signs

Verbatim reuse of shipped `ksign(x, m, iso)` **[C]**: interval-Horner
of the degree-5 tuple-polynomial over `ISO13` (s₀ = 20); undecided ⇒
bisect keeping the half containing α via exact
`poly_sign_at(m₁₃, mid)` and the certified `m_increasing = true`. The
spike simulates this engine exactly (same interval arithmetic, same
bisection rule): **all 59 battery elements and all 1711 pairwise
comparator differences resolve with 0 bisections** at s₀ = 20, exact
order == float order, min pairwise gap 0.01199, accumulator peaks
**113 bits** (elements and comparators) against the 127-bit ceiling
(spike G1–G7). Zero test: a degree-<6 tuple evaluates to 0 iff it is
the zero tuple (m₁₃ irreducible ⇒ the power basis is faithful) — `eq`
is tuple equality after gcd reduction.

### 4.2 Extension N₁₃ signs — the two-level product rule (with the gcd-normalize arm)

`sgn((u + vw)/den)`, den > 0, reduces to K₁₃ signs:
1. `sgn(u) = Equal` ⇒ `sgn = sgn(v)` (and conversely);
2. `sgn(u) = sgn(v)` (both positive or both negative) ⇒ `sgn = sgn(u)`
   — exact because `s = 2 − α > 0` (§1), so `u + vw` has the common
   sign of its parts;
3. **mixed signs ⇒ the product rule:**
   `sgn(u + vw) = sgn(u) · sgn(u² − s·v²)`.
   Proof sketch **[P]**: `x·(u − vw) = u² − s·v²`, and in each mixed
   case the conjugate factor `u − vw` has sign `sgn(u)`, so dividing
   the exact product sign by `sgn(u)` (an involution) gives the rule.
   The zero case `u² = s·v²` with `(u, v) ≠ 0` is **impossible**: it
   would make s a square in K₁₃, hence `N(s) = N(z)²` a rational
   square — but `N(s) = 13` (§3). Both factors are exact `ksign`
   calls on degree-5 tuples.
   **The gcd-normalize arm (adopt §2.2-(b) here):** before evaluating
   `u² − s·v²`, divide both tuples by `g = gcd(all 13 integers)` —
   the sign is unchanged and the ussv accumulator shrinks by
   `2·log₂(g)` bits. Measured on the N battery: 48/67 elements have
   `g > 1` (max g = 64), median saving 12 bits, max 14 bits; worst
   case g = 1 saves nothing — the trick is an optimization, **not**
   the envelope resolution (§2.2).

### 4.3 The n9 lesson carried forward (consumer-contract caution) **[P]**

M178 §4.3 documented that the shipped `n9::Nq::sgn` mixed-sign arm
returns `sgn(u)` unconditionally (`then_with` semantics) while its own
doc states the product rule — a latent hazard on mixed-sign elements
with `u² < s·v²`. The N₁₃ battery therefore **mandates** the
regression subset (§6.3), and the spike exhibits it: **6 mixed-sign
`(u·v < 0)` elements, of which 4 have `u² < s·v²` — a sign(u)-only
shortcut returns the WRONG sign on every one of them** (spike H9). An
n = 13-specific observation worth recording: unlike n = 11 (where
`1 − 2w ≈ −0.127 < 0` is itself a wrong-shortcut element), at n = 13
`w ≈ 0.4786 < ½` so `1 − 2w ≈ +0.043 > 0` — the `(1, ±2)` pair
becomes a **boundary control** (u² > s·v², shortcut accidentally
right), and the wrong-shortcut burden shifts to `(1, −3)`, `(2, −5)`,
`(3, −7)`, `(−1, 3)`. The battery spec below bakes both roles in
explicitly so the impl job cannot "pass" with a shortcut engine.

---

## 5. Consumer contract at the fields.rs seam (signature-level; no code)

### 5.1 The fields.rs K₁₃ block (mirrors the L₁₀ block)

```rust
pub struct K13(pub [i64; 6]);            // c0 + c1*a + ... + c5*a^5, den 1
pub const MK13: [i64; 7] = [-1, 3, 6, -4, -5, 1, 1]; // low->high minpoly
pub const RED13: [[i64; 6]; 5] = [       // b^6..b^10 fold rows, low->high
    [  1,  -3,   -6,   4,   5,  -1],
    [ -1,   4,    3, -10,  -1,   6],
    [  6, -19,  -32,  27,  20,  -7],
    [ -7,  27,   23, -60,  -8,  27],
    [ 27, -88, -135, 131,  75, -35],
];
pub const ISO13: Iso = Iso {
    lo_num: 1856935, hi_num: 1856936, s: 20, m_increasing: true,
};
pub fn kval13(x: [i64; 6]) -> f64;       // float view, emission only
```

The block comment **must** state why `s = 20` and not the 26
convention: the s = 26 wall (α⁵ monomial, 135 bits), the pre-registered
R1/R2/R3 rule, and the certified CAP = 4 mid-eval walls
(110/114/110/125) — pointing at §2.2 of this doc. Everything else is
the existing const-generic layer: `kadd/ksub/kneg/kscale/kmul::<6>`,
`ksign`, `knorm`, `koblique_norm2/dot2/cross/rot` **[C]**. No new
arithmetic kernels.

### 5.2 Field tag and medallion wiring (the M158 tables / M161 catalog pattern)

- `medallion::Field` gains `K13` — additive variant, documented exactly
  like `Field::K9`/`Field::K11` ("the record coordinates live in its
  quadratic extension by w = 2sin(π/13), w² = 2 − α; see
  `monster::n13`"), with zero other consumers at its landing (the K9
  disclosed-exception pattern **[C]**).
- Future `monster/n13.rs`: `N13` implements the generic `FieldEl`
  trait (`add/sub/neg/mul/sgn/eq_x/fl/zero_like/int_like/scaled_div`)
  with `sgn` = §4.2 (including the gcd-normalize arm); the generic
  point layer (`gadd/gdot/gcrossings/outlines/hull_sanity/star_exits`)
  then works over N13 unchanged **[C]**.
- `Medallion` for the n = 13 records: `FIELD = Field::K13`;
  `contact_cos2` returns `None` — degree-6 cos² pins do not fit the
  `(a + b√d)/den` quadratic contract (same as L10/K11 **[C]**); exact
  angle work goes through cross-multiplied K₁₃ identities instead.
- **/64 grid convention (the M161 `Kq` pattern **[C]**):** raw
  hull-grid numerators arrive at the fixed denominator 64 (the
  BATTERY_L10/M158 real-numerator principle); stored N₁₃ elements are
  gcd-reduced `(u, v, den)` (the n9 convention). At n = 13 the hull
  battery's canonical coefficients stay ≤ 192 at den 64 (8 bits,
  measured — M178's K₁₁ analog was ≤ 128), and all 13 edge lengths are
  exactly 1 as N₁₃ elements with worst canonical coefficient 1 (§6.2).

### 5.3 The named n = 13 consumers

1. **24c monster rung at n = 13** (PROPOSED §24c): the odd-n ladder —
   n = 9 shipped (M171), n = 11 unblocked at design level (M178), n =
   13 is the next rung and the first degree-6 one. Note the current
   §24c scope line reads "n ≤ 12" — a 13-rung extends that cap; the
   pointer-line proposal in this landing's report flags it. The pin
   rule `P = Mᵢ − R·Mⱼ` with `R` the π/13 half-step and d-rule
   `d ≡ 2(i − j) − 1 (mod 13)` extend unchanged (M171 odd-n rules
   **[C]**; spike H6–H7 works the full example below).
2. **11/13-pointed-star angle-mismatch instances** (PROPOSED
   hankin-transform entry, M99 status line: "Remaining: 11/13-pointed-
   star angle-mismatch instances (need 22b/22e substrates)"): the 13
   half needs a K₁₃-exact substrate; the {13/d} star walks and exit
   machinery of §3 are its exact-geometry feed.
3. **Harriss n-fold rhomb family at n = 13** (PROPOSED #18, Harriss
   2004 + Pautze CAST framework): the any-n family's n = 13 instance
   lands on this field; the applicability table (§7.3) carries the
   row.
4. **Say-awen 21-fold shares the engine depth** (deg-3/4 doc §6: K₂₁,
   deg 6, quadratic tower over K₇): whatever the D = 6 envelope story
   certifies here constrains K₂₁ too — but K₂₁'s representation choice
   (flat deg-6 vs tower-over-K₇) is a separate impl-job decision (§8.3).

- **24c n = 13 rungs (M181 design's consumer)**: the four (13,2..5)
  monster records consume K₁₃ exactly as contracted here (ISO13@s₀=20,
  N₁₃ two-level sign rule with the mandated wrong-shortcut regressions)
  — engine-side design and battery mandates in
  [medallion-24c-odd-n-design.md](medallion-24c-odd-n-design.md)
  (M181; spike-validated: 0 bisections on the full census workload,
  max bit height 22, all cos² full-degree ⇒ contact_cos2 = None).

---

## 6. BATTERY_K13 design (mirrors BATTERY_L10's real-numerator principle)

### 6.1 Sign/order battery — 59 elements, three families (deduped from 62)

Mirroring `BATTERY_L10` (raw /64-scale shapes with realistic heights +
exact field pins + strap/exit-shaped numerators; floats never decide).
The constructed set is 62; **3 exact duplicates are dropped** (the
battery is a set: `u² − s·v²` cores are shared by mirrored mixed-sign
entries — drop diagnostic `(−17, 9, 0, 0, 0, 0)` ×2, `(−7, 4, 0, 0, 0,
0)` ×1):
1. **Raw /64 hull-grid tuples** (18): the 6 x-numerators `32·C_k`, the
   6 y-numerators `32·q_j·cos_half(j)` (the tall realistic shapes),
   and 6 synthetic mixed-height /64-scale tuples; heights ≤ 4096
   (13 bits) — the girih-style raw grid transplanted to D = 6;
2. **Exact field pins** (17): tower `C₂..C₆`, `s = 2 − α`, `4 − α²`,
   `5 − α²`, `α² − 1`, constants `4`, `1`, `α`, the `√13` tuple `η`
   and `η − 4`, and the three cubic periods `A₀, A₁, A₂` — every
   deg-5-reachable pin shape plus both subfield witnesses;
3. **Strap/exit/outward shapes + envelope probes** (16): mixed-height
   `b·q − c·b³` families, top-degree probes (`[0,0,0,0,3,−2]`,
   `[0,0,0,0,−2,5]`, `[0,0,0,3,0,−5]`, `[0,0,−7,0,0,9]`), the
   all-ones tuple, and **the α⁵ monomial** — the envelope probe that
   certifies the s = 26 wall (§2.2);
4. **Real mixed-sign cores** (11): the 7 mandated `u² − s·v²` shapes
   of §6.3 (4 wrong-shortcut + 2 boundary + 1 same-sign control) and
   4 hull-edge `u² − s·v²` cores (the tall real-geometry shapes).

Assertions (all exact, spike G): exact sign == float sign on all 59;
all 59 float values distinct (min gap 0.01199); no element within
1e−6 of 0 (min |value| 0.0837); exact total order (comparator = sign
of difference) matches the float sort; **single-pass** — 0 bisections
on every element and every one of the 1711 pairwise comparator
differences; accumulator peaks 113 bits ≤ 127 (the L10 single-pass
discipline **[C]**).

### 6.2 Geometric battery (the N₁₃ layer, 67 elements)

- **/64 hull walk:** `V_k = Σ_{j<k} (cos 2πj/13, sin 2πj/13)` at den
  64 — x-numerators `32·C_j(α)` (u-part), y-numerators
  `32·q_j·cos(πj/13)` (v-part, w-multiple). Asserts: exact closure
  `V₁₃ = V₀`; all 13 edge lengths `|V_{k+1} − V_k|² = 1` **exactly as
  N₁₃ elements** (worst canonical coefficient 1; tallest raw edge
  ussv tuple height 5120); hull canonical coefficients ≤ 192 (spike
  H4, F0a).
- **Companion rotation about the exact centroid:** subtract
  `c = ΣV/13` first, then `R_{2π/13}(V_k − c) = V_{k+1} − c` for all
  k — exact N₁₃ (spike H5; the subtraction must precede rotation —
  the K₁₁ spike's worked trap, carried).
- **Pin displacement worked example** (spike H6–H7):
  `P = M₃ − R_{π/13}·M₇` at the /64 grid — float
  (3.456517125, −2.101447416), exact component signs (+, −); d-rule
  `d ≡ 2(3 − 7) − 1 = −9 ≡ 4 (mod 13)` → canonical figure step
  `min(d, 13 − d) = 4` ({13/4}) — the M171 odd-n rule extends to
  n = 13 unchanged.
- **N battery composition (67):** the 7 mandated mixed-sign pairs
  `(u, v, 1)`, the 2 pin displacement components, 6 pure `w·q_j`
  shapes, all 13 hull vertices (x and y components, 26), all 13 edge
  components (dx and dy, 26). Assertions: exact N₁₃ sign == float
  sign on all 67 with **0 bisections**; the K₁₃-level accumulator
  peaks (u, v, and every normalized ussv arm) **110 bits** ≤ 127 —
  the tall hull/pin tuples included (spike H8/H8b).
- **Zero-awareness mandate:** exactly 4 battery elements are exactly
  zero — the origin vertex (both components), the first vertex's
  y-component, and the horizontal first edge's y-component (the walk
  starts along +x: `V₀ = (0, 0)`, `V₁ = (1, 0)`). No *nonzero*
  element sits within 1e−9 of 0 (min |value| 0.0427 — itself the
  `1 − 2w` boundary control). `N13::sgn` must return `Equal` on the
  zero tuple *before* the product rule (spike H10).

### 6.3 Mixed-sign regression mandate (the n9 lesson, battery-side)

The N battery **must** contain mixed-sign elements with `u² < s·v²`
— at n = 13: `(1, −3)`, `(2, −5)`, `(3, −7)`, `(−1, 3)` (each signs
opposite to `u`; a sign(u)-only shortcut is wrong on all four), the
boundary pair `(1, −2)`, `(−1, 2)` (`u² > s·v²`, shortcut accidentally
right — they pin the *rule*, not the shortcut), and the same-sign
control `(1, 3)`. This is the executable form of the §4.3 caution: a
future `N13::sgn` implementation that passes this battery cannot
carry the shipped n9 defect.

---

## 7. The general odd-n shape: uniform prime-n recipe, the degree theorem, applicability, and the composite contrast

### 7.1 The uniform prime-n recipe **[P]**

For any odd prime p, the entire §1–§6 construction is uniform in n
(every step certified at multiple n by the spike):

1. `K_p = Q(2cos(2π/p))`, `deg = (p−1)/2`; `m_p` by Newton
   (`c_p(t) = p−1 if p|t else −1`) and Laurent
   (`x^d·m_p(x+x⁻¹) = Φ_p`) — both certified for
   p ∈ {3, 5, 7, 11, 13, 17, 19, 23} (spike B);
2. irreducibility: `Φ_p(x+1)` Eisenstein at p; `disc(m_p) =
   p^{(p−3)/2}` ⇒ `Z[α]` maximal, denominator-1 core (spike B —
   including the n = 11 and n = 13 landings this program actually
   ships);
3. `w_p = 2sin(π/p)`, `w_p² = 2 − α`, `N(2 − α) = m_p(2) = Φ_p(1) = p`
   — prime ⇒ genuine quadratic extension (the identity
   `m_p(2) = Φ_p(1)` is the Laurent identity at x = 1; `Φ_p(1) = p`
   is elementary — certified for all eight primes, spike H2);
4. sine ratios `q_j ∈ Z[α]`, oblique frame `U(k)` integral with
   `|U|² = 1`, /64 hull with unit edges, star walks exact — the
   geometric layer is uniform (spike D/E at n = 13, G at n = 11);
5. pin rule `P = Mᵢ − R_{π/p}Mⱼ`, d-rule `d ≡ 2(i−j)−1 (mod p)`
   (M171 **[C]**; spike H6 at n = 13);
6. the envelope: pick `s₀` by the pre-registered R1/R2/R3 rule of
   §2.2 at the target battery; the m₁₃-side mid-eval wall is the
   binding gate, the s_max(D) curve (§7.6) is the a priori map.

What is *not* uniform: the **subfield structure** (§7.4–7.5) and the
composite-n discriminants (§7.5) — prime n gives a cyclic
`(Z/p)*/±1` Galois group with subfields iff `(p−1)/2` is composite.

### 7.2 The degree theorem: `w_n ∉ K_n` for every odd n **[P]**

The spike proves (and certifies per n ∈ {9, 11, 13, 15, 21}):

- `2 − α_n = −ζ_n⁻¹(1 − ζ_n)²` **exactly** (the Laurent identity
  `2 − x − x⁻¹ = −x⁻¹(1 − x)²`; spike H3a).
- **Order argument (universal):** if `2 − α_n` were a square `β²` in
  `K_n ⊆ Q(ζ_n)`, then `(β/(1 − ζ_n))² = −ζ_n⁻¹` — a root of unity of
  order exactly 2n would be a square in `Q(ζ_n)`, i.e. `Q(ζ_n)` would
  contain a root of unity of order 4n. But the roots of unity of
  `Q(ζ_n)` (n odd) are exactly `±ζ_n^j` — orders ≤ 2n. Contradiction.
  Hence `w_n ∉ K_n` and `deg w_n = φ(n)` for **every** odd n ≥ 3.
- **Prime and prime-power n (9, 11, 13) — the elementary a priori
  proof:** `N(2 − α) = m_n(2) = p` is not a rational square, and a
  square in `K_n` has square norm. No mod-p work needed.
- **Multi-prime n (15, 21):** `N = m_n(2) = Φ_n(1) = 1` — the norm
  certificate is vacuous, and the order argument carries the proof.
- **Machine certificates (spike H3):** for n = 9, 11, 13 the
  irreducibility of `m_n(2 − y²)` (deg φ(n), twice the core degree)
  over Q is certified by a **Rabin test** mod a witness prime (p = 5, 3, 7
  respectively — matching the spike's earlier divisor-enumeration
  results), after fixing two implementation subtleties worth
  recording: (i) the test's Frobenius condition is
  `x^{p^d} ≡ x (mod f)` — compare against `[0, 1]` (coefficient of x),
  not the constant term; (ii) when `f mod p` is not squarefree the
  Frobenius orbit can legitimately hit 0 — an empty iterate means
  *reducible*, not a crash.
- **A structural finding the impl jobs should know:** for n = 15 and
  n = 21 **no Rabin witness exists at any prime** (checked through
  47): `Gal(Q(w_n)) = (Z/4n)*/±1` has exponent 4 < φ(15) = 8 and
  6 < φ(21) = 12 — no full-cycle Frobenius — so `m_n(2 − y²)` is
  irreducible over Q yet **reducible mod every prime**. The Rabin
  route is sound only for prime/prime-power n; multi-prime n needs
  the order argument (or a factorization argument over Q — for the
  record, `m₁₅(2 − y²) = y⁸ − 7y⁶ + 14y⁴ − 8y² + 1` was verified
  irreducible by exhausting all monic factorization patterns by
  hand). The Rabin test also **replaces** the spike's earlier
  brute-force `p^{d/2}`-divisor enumeration, which hangs at degree 12
  (≈ 266M polydivmods across four primes) — the Frobenius iteration
  runs in d ≤ 12 steps.

### 7.3 Applicability table update (the deg-3/4 doc §6, extended)

Design-level coverage of the odd-n cyclotomic layer now spans
**n = 7, 9, 11, 13, 15, 17, 21** (shipped in `fields.rs`: K₇, K₉,
K₁₅, L₁₀; design docs: K₁₁ [M178], K₁₃ [this doc]; K₁₇/K₂₁
design-level rows only — no D ≥ 6 precedent ships):

| n | field | deg | status | consumers |
|---|-------|-----|--------|-----------|
| 7 | K₇ | 3 | **shipped** (M137/M149) | GLB binary, Danzer, G-S 7-fold, Nischke–Danzer |
| 9 | K₉ | 3 | **shipped** (M139) + Nq (M171) | Harriss 9-fold, GLB 9-fold, 24c n=9 |
| 11 | K₁₁ | 5 | design (M178) | 24c n=11 rung (queued) |
| 13 | K₁₃ | 6 | design (this doc) | 24c n=13 rung, 11/13-star instances, Harriss #18 @13 |
| 15 | K₁₅ | 4 | **shipped** (M139) | GLB 15-fold |
| 17 | K₁₇ | 8 | design-level row only | Say-awen 17-fold (deferred, §8.2) |
| 21 | K₂₁ | 6 | design-level row only | Say-awen 21-fold (shares D = 6 depth) |

The K₁₃ row is the first degree-6 entry: **degree 6 is a new engine
depth class**, and its sign/battery/envelope story was re-derived
here rather than extrapolated from K₁₁ (the s = 26 convention that
works at D ≤ 5 fails structurally at D = 6).

### 7.4 Composite-n contrast (measured discriminants — the spike corrects the guesses)

The prime-n discriminant lemma does **not** extend to composite n;
the conductor-discriminant formula (even Dirichlet characters mod n)
does, and the spike measured the exact values by fraction-free
Bareiss (the pre-spike guesses 225/1029 were wrong):

| n | m_n (low→high) | disc(m_n) | structure |
|----|----|----|----|
| 9 | `[1, -3, 0, 1]` | **81 = 9²** | cyclic cubic, conductor 9 (even chars: conductors 1, 9, 9) |
| 15 | `[1, 4, -4, -1, 1]` | **1125 = 5·15²** | cyclic **quartic** (not biquadratic); unique quadratic subfield Q(√5) (conductors 1, 5, 15, 15) |
| 21 | `[1, -8, 8, 6, -6, -1, 1]` | **453789 = 3³·7⁵** | cyclic sextic (conductors 1, 7, 7, 21, 21, 21) |

These match the deg-3/4 doc's subfield narrative (`K₁₅ ⊃ Q(√5)`,
`K₂₁ ⊃ K₇`) and the shipped K₉/K₁₅ minpolies exactly.

### 7.5 Subfield structure at n = 13 — and the M178 §7 correction

M178 §7 stated "disc(m₁₃) = 13⁵ ⇒ Z[2cos(2π/13)] maximal, degree 6
prime ⇒ still no subfields". The maximality conclusion is right, the
"degree 6 prime" premise is **wrong**: 6 = 2·3 is composite, and
`Gal(K₁₃) ≅ Z/6` has proper subgroups. The correct picture (spike
C8–C9, all exact):

- unique quadratic subfield `Q(√13)`: witness `η = [5, −4, −8, 2, 2,
  0]`, `η² = 13` exact (Gauss-period construction over the Legendre
  symbol mod 13);
- unique cubic subfield `F₃`: witness periods `A₀ = C₁ + C₅`,
  `A₁ = C₂ + C₃`, `A₂ = C₄ + C₆` with elementary symmetric functions
  collapsing to rationals (e₂ = −4, e₃ = −1 — alpha-free, the
  subfield membership test), minpoly `h₃(y) = y³ + y² − 4y + 1`,
  `disc = 13²` (conductor 13);
- `K₁₃ = F₃·Q(√13)`, `disc(K₁₃) = disc(F₃)²·disc(Q(√13)) = 13⁴·13 =
  13⁵` — consistent with the polynomial discriminant and the
  conductor-discriminant formula.

Practical consequence: subfield *probes* (trace/norm maps into `F₃`
or `Q(√13)`) become available at n = 13 for the first time in the
odd-n layer — potential coefficient-growth dampeners for deep
substitution chains. No current consumer needs them; §8.4 flags the
revisit condition.

### 7.6 The s_max(D) envelope curve — and where D = 7 falls

The a priori one-pass envelope (derivation form:
`bits(H) + (D−1)·(s + log₂ α) ≤ 127`, α < 2, conservative consumer
height H = 320):

| D | s_max | where it binds |
|----|----|----|
| 3 | 58 | K₇/K₉ (shipped, huge headroom) |
| 4 | 38 | K₁₅/L₁₀ (shipped) |
| 5 | 28 | K₁₁ (M178 chose 26 — inside the curve) |
| **6** | **22** | **K₁₃/K₂₁ — this doc's class; measured s₀ = 20 (battery-driven, below the conservative ceiling)** |
| 7 | 18 | **never arises**: φ(n) = 14 has no solutions (spike I1, brute-forced n ≤ 2000) — no cyclotomic real subfield has degree 7 |
| 8 | 15 | K₁₇ (Say-awen 17-fold; the next real wall, §8.2) |

Two readings worth fixing in the record: (i) the curve is the
*conservative* map (H = 320); the measured K₁₃ battery (heights to
4096) still fits at s₀ = 20 with 14 bits of headroom because tall
decisive tuples are L1-bounded, not height-bounded — the §2.2 L1
ceiling (7.706×10⁶ at s₀ = 20) is the honest consumer-side criterion;
(ii) D = 7 is a **gap in the degree sequence, not a wall** — after
degree 6 the next new depth class is D = 8.

---

## 8. Open questions

1. **N₁₃ denominator-growth policy** (same question M178 §8.3 flagged
   for N₁₁): gcd-reduce-per-op (n9) vs the /64-closed convention (girih
   Kq, CHANGELOG-flagged residual). Measure both at the n = 13 record
   shapes before the impl job. The §4.2 gcd-normalize arm is
   sign-engine-local and orthogonal to this choice.
2. **K₁₇ (D = 8) practicality:** the curve gives s_max = 15; the same
   s₀-drop machinery applies, but 2⁻¹⁵ one-pass resolution may be too
   coarse for girih-scale /64-grid gaps (2⁻⁶ grid vs 2⁻¹⁵ evaluation —
   μ-margins would need a real battery). Say-awen 17-fold stays
   deferred until a consumer family exists; this doc's R1/R2/R3 rule
   is the template.
3. **K₂₁ representation choice:** flat degree-6 (this doc's engine
   class, one `kmul::<6>`) vs the quadratic tower over K₇ (deg-3/4 doc
   §6) — the tower reuses shipped K₇ rows but pays a two-level sign
   engine; flat pays the D = 6 envelope (solved here). Decide at the
   K₂₁ impl job with the Say-awen 21-fold record shapes in hand.
4. **Subfield probes at n = 13:** trace/norm into `F₃`/`Q(√13)` as
   coefficient-growth dampeners (§7.5) — revisit only if a 13-fold
   substitution family (Harriss #18 at deep levels) shows fold
   amplification stress; the §2.1 amplification table (max 491 at
   b¹⁰) is the monitoring point.
5. **BISECT_CAP at D = 6:** all four rounds certified (§2.2, worst
   125 bits) — the shipped CAP = 4 stands unchanged; the single-pass
   battery discipline keeps it dead code in tests, exactly as at
   L10/K₁₁.

---

## 9. Verification appendix (spike evidence index)

Spike: `.scratch/odd-n-fields-design/spike_k13.py` — stdlib-only
Python, exact integer arithmetic (`fractions` only in float
cross-checks), f64 never decides. **93/93 checks green** (log
`spike_k13_output.txt`; rc = 0). Section map:

| part | certifies | checks |
|------|-----------|--------|
| A | m₁₃ two paths (Laurent + Newton: p₁..p₆ = −1, 11, −4, 31, −16, 98), Eisenstein at 13, mod-2, separability, conjugate sum | A1–A4 |
| B | prime-n lemma battery p ∈ {3,5,7,11,13,17,19,23}: Laurent, disc = p^{(p−3)/2} (11⁴ **and** 13⁵), m_p(2) = Φ_p(1) = p; Bareiss disc(m₁₃) = 13⁵ | B, B13 |
| C | RED13 rows float-certified (amplification 20/25/111/152/491), m(b) = 0 exact, tower wrap C_k = C_{13−k}, C₁₃ = 2; √13 = η exact; cubic periods A₀/A₁/A₂, h₃ disc 13², disc(K₁₃) chain | C1–C9 |
| D | U(k) frame: \|U\|² = 1, ΣU = 0, R^k-equivariance, integrality, float dirs ≤ 1e−11 | D1–D6 |
| E | star walks {13/1..13/6}: closure, R^d-equivariance, exact shoelace factors (areas 13.186 → 0.395) | E1 |
| F | /64 hull closure (max coeff 192); ISO13@s26 re-certified; **the s = 26 wall (α⁵ monomial, 135 bits)**; s-ladder 17..26; **s₀ = 20** by R1/R2/R3; ISO13@s0 Sturm-isolated + strictly increasing; census 6; α < 2; N(2−α) = 13; **psa walls 110/114/110/125 through CAP = 4**; L1 ceiling 7.706×10⁶ vs battery 20480; μ min 2332 / median 2.861×10⁵ | F0–F6 |
| G | 59-element battery (62 → 59 dedupe): signs, distinctness (gap 0.01199), total order, **0 bisections on 59 + 1711 comparators**, acc peaks 113 ≤ 127 | G1–G7 |
| H | w² = 2−α float; m_n(2) = Φ_n(1) for n ∈ {9,11,13,15,21} + Laurent; **the degree theorem** (order argument; Rabin witnesses 5/3/7; the n = 15/21 no-witness structural finding); /64 hull unit edges exact (worst coeff 1); centroid rotation; pin + d-rule ({13/4}); 67-element N battery 0-bisection, ussv peaks 110; **4 wrong-shortcut mixed-sign regressions**; zero-awareness (4 exact zeros); gcd-normalize savings (48/67, median 12, max 14 bits) | H1–H11 |
| I | **D = 7 impossible** (φ(n) = 14 unsolvable, n ≤ 2000); s_max(D) curve 58/38/28/22/18/15; the μ-form vs naive width-form precision criterion (measured per-pair μ ≥ 99.9); composite discs 81 / 1125 / 453789 (Bareiss) vs conductor-discriminant | I1–I4 |

Discipline (matching the M178 bar): derivation and certification live
Python-side; the future Rust job carries only the certified constants
of §2–§3 and re-asserts them against floats in tests (`BATTERY_K13` +
the geometric battery of §6). Spike findings worth preserving beyond
the constants:

- **the s = 26 wall is structural** — a primitive monomial defeats
  gcd-normalize; any future "just raise s" proposal must re-derive
  against the monomial probe;
- **the naive all-elements bisection-round gate is wrong** — the
  mid-eval wall is m-side (element-height-independent); gate on the
  true bisection path;
- **the μ-form, not the width-form**, is the one-pass precision
  criterion — the width form "derives" a false 21.1-bit wall;
- **Rabin `x^{p^d} ≡ x` compares the x-coefficient**, and an empty
  Frobenius iterate means reducible (non-squarefree mod p), not a
  crash;
- **multi-prime n has no Rabin witness at any prime** (Gal exponent <
  φ(n)) — the norm/order argument is the only uniform certificate;
- the Laurent-identity checker must strip zero coefficients after
  term cancellation (composite n leaves cancelled terms in the
  accumulator — the spike's own A2/H2 bug, fixed en route);
- the carried traps: rotate-after-centering (§6.2), the half-integer
  `2sin²` pin (§3), raw-vs-field Chebyshev-U (§3), `then_with`
  semantics (§4.3), and the Eisenstein shift-allocation bug the spike
  caught in its own A3a (fixed en route — `shifted` must have exactly
  `len(Φ)` entries).
