# Say-awen 17/21-fold field layers — K₁₇ (degree 8, the D = 8 wall) and K₂₁ (degree 6, composite) — design doc

Status: **PROPOSED** — design delivered and spike-validated; **no code in
this landing** (docs-only). Companion spike:
`.scratch/sayawen-17-21-design/spike_k17_k21.py` (stdlib-only Python,
exact integer/Fraction arithmetic, **97/97 checks green**, log
`spike_k17_k21_output.txt`, rc = 0; banked in the main checkout's
`.scratch/sayawen-17-21-design/`). Scope: the complete field-layer
treatment of the **Say-awen 17-fold** (K₁₇, degree 8 — the first degree-8
candidate, and the answer to "how far does the shipped engine stretch":
**it does not stretch to D = 8**, measured) and the **Say-awen
21-fold** (K₂₁, degree 6 composite — the engine transfers verbatim), the
**norm-vacuity finding** at n = 21 (the M179 prime-n norm certificate is
vacuous there; a p = 43 square-class witness replaces it), the corrected
odd-n sine/cosine structure for substitution tables, and the corrected
§23i new-field tiers table. Predecessor docs:
[`cyclotomic-field-degree5.md`](cyclotomic-field-degree5.md)
(K₁₁, M178),
[`cyclotomic-field-odd-n.md`](cyclotomic-field-odd-n.md)
(K₁₃ + the general prime-n recipe, M179 — this doc **is the K₁₇/K₂₁
design-level rows** its §7.3 table deferred), and
[`medallion-24c-odd-n-design.md`](medallion-24c-odd-n-design.md)
(M181, the (n, c) rung consumer shape). Same citation markers: **[P]** =
proven by exact algebra below (the spike's assertions are the executable
proof); **[C]** = cited (shipped code, dossier, or literature).

---

## 0. One-paragraph summary

The Say-awen 17-fold needs the degree-8 field `K₁₇ = Q(α₁₇)`,
`α₁₇ = 2cos(2π/17) ≈ 1.864944`, minpoly
`m₁₇(y) = y⁸ + y⁷ − 7y⁶ − 6y⁵ + 15y⁴ + 10y³ − 10y² − 4y + 1`, disc
`17⁷ = 410,338,673` (prime-n lemma re-verified at the largest prime-n
degree yet), 8 real roots, mod-3 Rabin witness, norm
`m₁₇(2) = 17` **prime** (the M179 norm certificate transfers verbatim —
K₁₇'s extension certificate is *not* the problem). **The problem is the
i128 envelope**: the pre-registered R1/R2/R3 stress test over the whole
a-priori operating range `s ∈ [11, 15]` (`s_max(8) = ⌊(127−9)/7⌋ − 1`)
returns **NON-QUALIFICATION** — R2 (the `poly_sign_at` CAP=4 wall, i128)
kills s ∈ {14, 15} with measured walls 133/140 bits, R1 (single-pass
battery decisiveness) kills s ∈ {11, 12, 13} with 4/2/2 undecided pairs
and a *structural* worst-pair margin `mu = 0.2559 < 1` at the R2-alive
best s = 13 — so **no s qualifies: the D = 8 WALL**. The shipped engine
(consecutive-dyadic ISO + interval-Horner + `BISECT_CAP = 4` + i128,
`fields.rs`) does not transfer to degree 8 at any resolution in range;
unblocking Say-awen 17 is an *engine* decision (accumulator width or
isolation scheme), not a field decision. The Say-awen 21-fold needs
`K₂₁ = Q(α₂₁)`, `α₂₁ = 2cos(2π/21) ≈ 1.91115`, minpoly
`m₂₁(y) = y⁶ − y⁵ − 6y⁴ + 6y³ + 8y² − 8y + 1` (two independent
derivations), disc `453789 = 3³·7⁵` (conductors `[1, 7, 7, 21, 21, 21]`),
cyclic sextic with quadratic subfield `Q(√21)` and cubic subfield `K₇`
(`K₂₁ ⊃ K₇` exactly) — a **degree-6 composite**, i.e. the same D = 6
engine depth class as K₁₃: the battery (71 elements, 2485 comparator
pairs, faithful K₁₃ mirror) is single-pass green at **s₀ = 21** with
acc peaks 118/118 bits and the R2 wall exactly at the 127-bit boundary.
The N-layer `w₂₁ = 2sin(π/21)`, `w² = 2 − α` is a **genuine quadratic
extension** — but *not* by the M179 norm argument:
`N_{K₂₁/Q}(2 − α) = m₂₁(2) = Φ₂₁(1) = 1` is a **perfect square** (norm
certificate VACUOUS; `Φ₂₁(2) = 2359 = 7·337` is the full degree-12 norm,
not the K-norm), the mod-p Rabin route on `m₂₁(2 − y²)` is
**structurally impossible** (Gal(`Q(ζ₈₄)⁺`) has exponent 6 < 12 — no
mod-p irreducibility witness exists, now a theorem with a measured DDF
census), and the working certificate is a **p = 43 square-class
witness**: 43 splits `m₂₁` completely and `(2 − α)²¹ ≡ −1` in
`F₄₃[x]/(m₂₁)` for every root, so `s = 2 − α` is a non-square in K₂₁ and
`[K₂₁(w₂₁) : K₂₁] = 2`. Both fields' sine/cosine structure for
substitution tables is the corrected odd-n shape: cosines in K_n,
sines as `(K_n, w·K_n)` split pairs, never flat degree-2·deg elements.

---

## 1. The two fields and the corrected odd-n sine structure **[P]**

### 1.1 K₁₇ — exact constants

`α = α₁₇ = 2cos(2π/17)`, the largest root of the monic degree-8
minpoly (all conjugates real — `ζ₁₇ ↦ ζ₁₇⁻¹` pairs):

| constant | value | certificate |
|---|---|---|
| `m₁₇` (low→high) | `[1, −4, −10, 10, 15, −6, −7, 1, 1]` | Newton power sums `p₁..p₈ = [−1, 15, −4, 43, −16, 138, −64, 467]` AND the Laurent identity `x⁸·m₁₇(x + x⁻¹) == Φ₁₇` exactly |
| disc(`m₁₇`) | `410338673 = 17⁷` | fraction-free Bareiss `Res(m, m′)`, prime-n lemma `p^((p−3)/2)` at p = 17; conductor product of the even chars mod 17 (`1, 17, 17, 17, 17, 17, 17, 17`); index² = 1 ⟹ `Z[α]` maximal |
| irreducibility | `m₁₇ mod 3` irreducible | Rabin at p = 3; `ord₃(17) = 8` exactly (the zeta/ζ⁻¹ Frobenius orbits merge only at k = 8) — a *genuine* mod-p witness, unlike Φ₂₁ |
| real roots | 8 (Sturm census in `[−2.1, 2.1]`) | separable (`gcd(m, m′) = 1`); with deg 8 = `[Q(ζ+ζ⁻¹):Q]` this is THE minpoly |
| norm | `m₁₇(2) = 17` PRIME | `== Φ₁₇(1)` — the M179 §3 F3e prime-n norm certificate transfers **verbatim**; the K₁₇ extension certificate is NOT vacuous (contrast §4) |
| quadratic subfield | `Q(√17)`: `η₁₇ = [3, 10, −18, −20, 12, 12, −2, −2]`, `η₁₇² == 17` exact | unique quadratic subfield of the cyclic octic (`Gal(K₁₇) ≅ Z/8`, conductor 17) |
| ISO at the s = 15 ceiling | `[61110/2¹⁵, 61111/2¹⁵]` | Sturm-isolates exactly one root (the bracket is real even at the ceiling) |
| RED fold table | `b⁸..b¹⁴` rows, L1 amplification `[54, 67, 399, 532, 2138, 3010, 10004]` | the reduction table `fmul` indexes; `m₁₇(b) == 0` exact on the tuple model |

### 1.2 K₂₁ — exact constants (the composite contrast)

`α = α₂₁ = 2cos(2π/21) ≈ 1.91115`, the largest of the 6 real roots
`2cos(2πk/21)`, `k ∈ {1, 2, 4, 5, 8, 10}` (sum = +1):

| constant | value | certificate |
|---|---|---|
| `m₂₁` (low→high) | `[1, −8, 8, 6, −6, −1, 1]` = `y⁶ − y⁵ − 6y⁴ + 6y³ + 8y² − 8y + 1` | **derivation 1**: Newton power sums `p₁..p₆ = [1, 13, 1, 41, 1, 139]` (half-Ramanujan orbit sums); **derivation 2**: exact `Fraction` null-space of `1, β, …, β⁶` in `Q[x]/(Φ₂₁)` (`β = ζ + ζ⁻¹`) — independent paths, same polynomial |
| Laurent | `x⁶·m₂₁(x + x⁻¹) == Φ₂₁` exactly | the composite-n cyclotomic identity |
| disc(`m₂₁`) | `453789 = 3³·7⁵` | Bareiss, re-certifying M179 §7.4's measurement; conductor-discriminant: even chars mod 21 have conductors `[1, 7, 7, 21, 21, 21]`, product matches — cyclic sextic, `Gal ≅ (Z/21)*/±1 ≅ Z/6` |
| irreducibility of the SHIPPED constant | `m₂₁ mod 2 = y⁶ + y⁵ + 1` irreducible | Rabin at p = 2: `ord₂(21) = 6` (powers `[2, 4, 8, 16, 11, 1]`, −1 never hit) — a genuine witness for `m₂₁` itself |
| Φ₂₁ contrast | **NO** mod-p irreducibility witness for Φ₂₁ | `ord_p(21) < φ(21) = 12` for every p in `{2, 5, 11, 13, 17, 19}` (`Gal (Z/21)*` has exponent `lcm(2, 6) = 6`); Φ₂₁'s irreducibility rests on the classical theorem + the Laurent identity |
| real roots / separability | 6 (Sturm); `gcd(m₂₁, m₂₁′) = 1` | the minpoly is THE minpoly |
| norm | `m₂₁(2) = 1 == Φ₂₁(1)` | **a perfect square** — see §4 (the norm-vacuity finding) |
| quadratic subfield | `Q(√21)`: `η₂₁ = [3, 12, −8, −10, 2, 2]`, `η₂₁² == 21` exact | conductor 21 |
| cubic subfield | `K₇`: periods `A₀ = C₁+C₈`, `A₁ = C₂+C₅`, `A₂ = C₄+C₁₀` have rational symmetric functions `e₁ = 1, e₂ = −2, e₃ = −1` ⟹ minpoly `y³ − y² − 2y + 1`, disc `49 = 7²` | `K₂₁ ⊃ K₇` EXACTLY as M179 §7.4's structure row claimed (float cross-check: `A₀ ≈ 0.44504`, `A₁ ≈ 1.80194`, `A₂ ≈ −1.24698`) |

The composite-n discriminant lemma failure (M179 §7.4) is confirmed at
the new degree: `3³·7⁵` is nothing like any `p^((p−3)/2)` shape, and the
conductor product `[1, 7, 7, 21, 21, 21]` is the whole story.

### 1.3 The corrected odd-n sine/cosine structure (supersedes the M179 §1 shorthand)

For **odd n** — and this is the structure every substitution table in
the §6 table must use:

- **Cosines**: `cos(mπ/n) ∈ K_n` for **every** `m = 1..n−1` (the exact
  fold chain through `C_k(α)`; float-certified for all four spike fields
  n = 7, 9, 17, 21).
- **Sines are NOT in K_n**: `sin(mπ/n) = cos((n − 2m)π/(2n)) ∈
  Q(ζ₄ₙ)⁺`, a field of degree `φ(4n)/2 = φ(n) = 2·deg K_n` — the
  **N-layer**. The mistake to avoid (it appeared in working notes):
  `sin(mπ/n)` is *not* a K_n element and cannot be stored flat without
  doubling the degree.
- **But the representation never leaves the split-pair shape**:
  `sin(mπ/n) = w · q_m/2` with `w = 2sin(π/n)`, `w² = 2 − α ∈ K_n`
  (exact tuple `[2, −1, 0, …]`), and `q_m = sin(mπ/n)/sin(π/n) ∈
  Z[α]` (Chebyshev-U at `b′/2`, `b′ = 2cos(π/n) = −C_{(n−1)/2}`). So
  every edge-direction coordinate is `(a, b·w)` with `a, b ∈ K_n` —
  the exact `n9::Nq` record shape (`u + v·w)/den`), one degree class up.
- **The exact identity chain** (tuple-verified for all four fields):
  `q_k · cos_half(k) == q_{2k}` for `k = 1..(n−1)/2`, where
  `cos_half(k)` is the exact double-cosine element `2cos(πk/n)` and
  `q_{2k} = sin(2kπ/n)/sin(π/n)` — no folding is ever needed because
  `2k ≤ n − 1` keeps the U-polynomial evaluation exact for every
  argument.
- **The w₂₁ identity, stated once correctly** (a typo circulated):
  `w₂₁ = 2sin(π/21) = 2cos(19π/42) = 2cos(2π·19/84)`. The form
  `2cos(19π/84)` is WRONG (value ≈ 1.518). The Galois computations of
  §4 (`σ₄₃(w₂₁) = −w₂₁`) hold with the correct form: 43 ≡ 1 mod 21
  fixes ζ₂₁, 43 ≡ 3 mod 4 conjugates i, and the class [43] ≡ [41]
  mod 84 acts on the ζ₈₄-cosine by the sign flip.

The **even-n = 10 contrast** (CAST 10-fold): `sin(kπ/10) =
cos((5−k)π/10)` — both edge-direction coordinates live in
`Q(ζ₂₀)⁺ = L₁₀` (degree 4, disc 2000, the SHIPPED `ML10 =
[5, 0, −5, 0, 1]`, `fields.rs:1071`). **No N-layer for 10-fold
positions** — even n folds the sine into the cosine field, which is
precisely why CAST 10 is the cheapest new-field family. (The spike's
float check uses `|5 − k|` — cos is even; a `% 10` fold is a latent
sign bug for k > 5, fixed in this spike and never to be copied.)

---

## 2. K₁₇ at D = 8: the envelope stress test — NON-QUALIFICATION **[P]**

### 2.1 The pre-registered rule and the engine seam

The stress test imports M179's `qualifies()` rule unchanged and runs it
at the D = 8 operating range:

- **R1** — single-pass decisive: at the isolating bracket
  `ISO = [lo/2^s, hi/2^s]` (consecutive dyadic), interval-Horner decides
  the exact sign of **every** battery element **and every** pairwise
  difference with **zero** bisections.
- **R2** — the engine's own m-side gate: `poly_sign_at` (the shipped
  midpoint evaluator) stays ≤ 127 bits through all `CAP = 4` bisection
  rounds at the ISO (the `fields.rs` operating-range note:
  `s₀ + CAP ≤ 30`, `BISECT_CAP = 4`, i128 accumulators).
- **R3** — s₀ = the **largest** qualifying s in the a-priori range
  `[11, 15]`, where `s_max(8) = ⌊(127 − 9)/(D − 1)⌋ − 1 = 15`.

Engine facts cited: `Iso { lo_num, hi_num, s, m_increasing }`
(`fields.rs:82-96`), the ksign half-selection
(`(msign > 0) == iso.m_increasing` decides the half),
`BISECT_CAP = 4` (`fields.rs:196-232`). Either R3 outcome was
pre-registered as admissible — **the data decides, and the data says no**.

**Monotonicity certificate, corrected**: `m₁₇` is strictly **INCREASING**
at its largest root (`m₁₇′` root-free on the ISO and `m₁₇′(lo) > 0`;
generically, a monic even-degree minpoly has `m′(largest root) =
∏(α − αᵢ) > 0`). The working notes carried a WRONG "DECREASING"
premise; `Iso.m_increasing = true` for **both** K₁₇ and K₂₁ — the same
orientation as K₁₃. The half-selection *logic* is orientation-correct;
only the flags were wrong. All walls and s₀ selections in this doc are
measured under the corrected True flag.

### 2.2 The battery (value-robust by construction)

66 K₁₇ elements (73 pre-dedupe, 7 exact duplicates), **2145 comparator
pairs**: 8 raw x + 8 raw y hull-grid shapes, 6 synthetic K₁₃-verbatim
shapes, 17 pins (`C₂..C₈`, `s`, `4−α²`, `5−α²`, `α²−1`, `4`, `1`, `α`,
`η₁₇`, `η₁₇−4`, `η₁₇+1`), 17 strap/exit rows, 7 MIXED `(u, v)` w-layer
rows, 8 edge cores. Composition: **41 pure 8-coord engine-basis tuples +
25 envelope probes** carrying degree-8 terms beyond the basis (including
the `[0,…,0,1]` α⁸ monomial). The probes are deliberate: they stress the
top of the reduction table (fold amplification up to 10004 at b¹⁴). The
verdict below is **representation-robust**: R1 undecided-ness is
value-determined (interval containment of 0 depends only on the real
value and the bracket), and the decisive stubborn pair (§2.4) has
`c₈ = 0` in both tuples — both elements are engine-representable, so no
canonicalization of the probes could flip the verdict.

### 2.3 Measured walls

**The s = 15 ceiling is structurally dead, two independent ways**:

- **D1b (the battery-side monomial wall)**: the single tuple
  `α⁸ = [0, 0, 0, 0, 0, 0, 0, 1]` has one-pass interval-Horner
  accumulator `2⁸·2^(8·15)/2^… = 128 bits > 127`. Gcd-normalize cannot
  help (the tuple is primitive). NO D = 8 element with a nonzero top
  coefficient is one-pass evaluable at s = 15 — the K₁₃ F1b wall, one
  degree class up.
- **D1d (the engine-side psa wall)**: `poly_sign_at` at the s = 15 ISO
  through CAP=4 rounds measures `(round, s′, bits) =
  (1, 16, 104), (2, 17, 126), (3, 18, 133), (4, 19, 140)` — worst
  **140 > 127**.

**The s-ladder** (acc bits elem/pair | undecided elem/pair, 2145 pairs):

| s | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 |
|---|---|----|----|----|----|----|----|----|----|
| acc bits | 84/85 | 92/93 | 100/101 | 108/109 | 116/117 | 124/125 | 132/133 | 140/141 | 148/149 |
| undecided | 1/16 | 1/7 | 0/4 | 0/2 | 0/2 | 0/1 | 0/1 | 0/0 | 0/0 |

**The per-s gate table over the pre-registered range** (True-flag walls):

| s | R1 undecided (elem/pair) | R2 psa CAP=4 wall (bits) | verdict |
|---|---|---|---|
| 11 | 0 / **4** | 111 | R1 kill |
| 12 | 0 / **2** | 111 | R1 kill |
| 13 | 0 / **2** | 126 | R1 kill (R2 passes by 1 bit) |
| 14 | 0 / 1 | **133** | R2 kill |
| 15 | 0 / 1 | **140** | R2 kill |

### 2.4 The verdict

**NO s in [11, 15] qualifies.** R2 kills {14, 15}; R1 kills
{11, 12, 13} — the partition is complete, and both gates are needed
(neither alone covers the range). This is the **D = 8 WALL** finding:
the shipped engine design (consecutive-dyadic ISO + interval-Horner +
`BISECT_CAP = 4` + i128) does not transfer to K₁₇ at any resolution in
the a-priori range. Per the pre-registration, NON-QUALIFICATION is a
*finding*, not a failure of the test.

Near-miss diagnostics at the R2-best s = 13 (measured):

- 2 undecided pairs of 2145; **worst pair `mu = 0.2559 < 1`** — the
  one-pass failure is *structural*: resolution `2⁻¹³` cannot separate
  the pair, and `mu` scales as `(pv/dv)·2^s`, so lowering s only
  shrinks it — no smaller s rescues R1. The ladder undecided counts are
  monotone down in s (`{11: 4, 12: 2, 13: 2, 14: 1, 15: 1}`) — widening
  resolution only helps, and not enough anywhere.
- Median pair `mu = 1795`; min element `mu = 11.41` — the battery is
  overwhelmingly comfortable; exactly one pair sits below 1 and it
  blocks the gate.
- The five worst pairs (mu, elems): `(0.2559, 33, 41), (1.12, 31, 39),
  (5.06, 26, 39), (5.55, 24, 41), (9.25, 7, 19)`. The stubborn pair is
  `(5 − α², 19α − 14α³ + 6α⁵ − α⁷)` — both tuples have `c₈ = 0`
  (engine-representable), so the failure is value-determined, not an
  artifact of the probe representation (§2.2).
- R1 *would* clear at s = 16 (`0/0` in the ladder) — but R2's wall is
  already ≥ 133 there and rising with s: the binding constraint flips
  from the battery side to the engine side as s grows, and the two
  gate windows never overlap.

**Consequence for the Say-awen 17-fold**: the field layer is *ready*
(§1.1 — constants, certificates, subfield, reduction table all
certified; the norm certificate transfers verbatim), but an
implementation is **blocked on an engine decision**: (a) wider
accumulators (i128 → i256 or double-word), (b) a non-consecutive
isolation scheme (the consecutive-dyadic invariant is load-bearing in
ksign/psa), or (c) a battery envelope reduction (smaller coefficient
heights — not available for a real tiling). Effort XL, and the spike is
the reusable harness: re-run it unchanged after any engine change.

---

## 3. K₂₁ at D = 6: the engine transfers verbatim **[P]**

K₂₁ is degree **6** — the same depth class as K₁₃ (M179), and the
battery/envelope story re-runs with new constants, not new machinery.

### 3.1 The battery (faithful K₁₃ mirror, D = 6 verbatim)

71 elements (74 pre-dedupe, 3 MIXED duplicates), **2485 comparator
pairs**: 10 raw x (`32·C_k`) + 10 raw y (`32·q_j·cos_half(j)`) hull-grid
shapes + the 6 K₁₃ synthetic shapes **verbatim** + 21 pins (`C₂..C₁₀`,
`s`, `4−α²`, `5−α²`, `α²−1`, `4`, `1`, `α`, `η₂₁`, `η₂₁−4`, `A₀`, `A₁`,
`A₂`) + the 16 K₁₃ strap rows **verbatim** (including the `[0,0,0,0,0,1]`
α⁵ monomial — the top of the D = 6 basis) + 7 MIXED `(u, v)` rows + 4
edge cores. No padded or invented rows: the K₁₃ shapes are used as-is at
D = 6.

**Geometric closure**: the /64 hull walk at n = 21 closes exactly
(`V₂₁ == V₀`), and all 21 unit edges have `|V_{k+1} − V_k|² =
[4096, 0, …]` **exactly as split K₁₇-style pairs — `4096 = 64²` in
/64-numerator units** (the walk steps `32·C_k` numerators at den 64, so
a unit edge is 64 numerator units per axis; the cos_half convention
returns `2cos(πj/n)`, making the y-step `64·sin(2πk/21)` exactly). The
"expected 1" in the working notes was a units error; both fields' edge
checks now assert the corrected constant.

### 3.2 Measured operating point

| gate | measurement |
|---|---|
| R1 ladder 16..24 | acc bits 93/93 → 133/133; undecided **0/0 at every s** — single-pass decisive across the whole pre-registered range |
| R2 s₀ candidates | 22 → wall 129 (**kill**), 21 → 127 (pass), 20 → 121, 19 → 117, 18 → 113 |
| **s₀ (R3)** | **21** — largest qualifying in [18, 22] |
| G3 psa rounds at s₀ | s′ = 22..25 → 113, 117, 121, **127** bits — worst lands exactly ON the i128 boundary (zero headroom, inside) |
| G4 L1 ceiling | `2^(127−5·21)/x̂⁵ ≈ 1.645e5` ≥ battery L1 max 26624 (degree-5 product bound for D = 6: `acc ≤ 2^(5s)·x̂⁵·L1`) |
| G5 margins | min element `mu = 1.853e4 > 2` |
| G6/G7 signs | exact sign == float sign on all 71 elements; **0 bisections** everywhere |
| G8/G9 separation | min `|value| = 0.08885` (= `s = 2 − α`, the smallest pin); min pairwise gap 0.0233 |
| G10/G11 order | exact total order == float sort; all 2485 comparators single-pass |
| G12 i128 | acc peaks 118/118 ≤ 127 |
| ISO@s₀ | `[4007962/2²¹, 4007963/2²¹]` — the same real interval as the pre-registered `[2003981, 2003982]/2²⁰` pin, expressed at s = 21 |

`s₀ = 21`, one step above K₁₃'s `s₀ = 20` (same D = 6 depth class; the
shift is the corrected monotonicity flag plus the verbatim battery —
the rule is dynamic and the data decides). Everything else — battery
families, sign/order checks, L1 bounds, the mixed-sign regression
mandate — mirrors K₁₃ check-for-check. **The engine transfer is
verbatim**: `Iso` + ksign + `BISECT_CAP = 4` + the `m_increasing = true`
certificate, with `s₀ + CAP = 25 ≤ 30` inside the documented operating
range.

---

## 4. The K₂₁ genuine-extension certificate — norm vacuity and the p = 43 square-class witness **[P]**

### 4.1 The problem: the M179 certificate routes all fail at n = 21

The N-layer extension `w₂₁ = 2sin(π/21)`, `w² = 2 − α` (exact tuple
`[2, −1, 0, 0, 0, 0]`) needs a proof that `x² − (2 − α)` is irreducible
over K₂₁. Three routes, two dead:

- **Norm route — VACUOUS.** `N_{K₂₁/Q}(2 − α) = m₂₁(2) = Φ₂₁(1) = 1`,
  a perfect square (`1 = 1²`). The M179 prime-n argument ("N = p prime
  ⟹ `2 − α` a non-square ⟹ `w ∉ K_n`", used at n = 13 where
  `m₁₃(2) = 13`) has no content when N = 1. **Do not write
  "N = 2359"**: `Φ₂₁(2) = 2359 = 7·337` is the FULL degree-12 field
  norm `N_{Q(ζ₂₁)/Q}(2 − ζ₂₁)`, not the K-norm `m₂₁(2)` — the spike
  prints the contrast row and asserts both values exactly.
- **Mod-p Rabin route on `m₂₁(2 − y²)` — STRUCTURALLY IMPOSSIBLE (now
  a theorem).** `L = Q(w₂₁) = Q(ζ₈₄)⁺`, degree `φ(84)/2 = 12`;
  `Gal(L/Q) = (Z/84)*/±1` has **exponent 6** (max over the 24 units of
  the class order `min{k : a^k ≡ ±1 mod 84}`, witnessed by `a = 5`),
  and `6 < 12`. For every `p ∤ 84` the Frobenius class of p has order
  ≤ 6, so its orbits on the 12 roots have size ≤ 6 and `m₂₁(2 − y²)`
  has ≥ 2 irreducible factors mod p: **no mod-p irreducibility witness
  exists**. Programmatic confirmation — the distinct-degree
  factorization census of `m₂₁(2 − y²)` mod p (squarefree-guarded DDF):

  | p | 5 | 11 | 13 | 17 | 19 | 43 |
|---|---|----|----|----|----|----|
| classes {deg: total deg} | {6: 12} | {6: 12} | {2: 12} | {6: 12} | {6: 12} | {2: 12} |

  — deg-6 factor pairs where the class order is 6; six deg-2 factors at
  p = 13 and p = 43 where it is 2. Never a degree-12 class.
- **p = 2 witnesses a square, not irreducibility**: the formal
  derivative of `m₂₁(2 − y²)` vanishes mod 2 and its even-coefficient
  square root is `m₂₁ mod 2 = y⁶ + y⁵ + 1` (Frobenius:
  `g(y²) = g(y)²` in `F₂[y]`).
- **Inert-prime `F_{p⁶}` Legendre route — ALSO vacuous.** For p inert
  in K₂₁ (residue degree 6): `p⁶ ≡ 1 mod 168` (≡ 1 mod 21 by
  `ord₂₁(p) | 6`, ≡ 1 mod 8 for odd p), so the Frobenius at the sixth
  power is trivial on L — in particular trivial on L/K — so `2 − ᾱ` is
  a **square** in `F_{p⁶}` for EVERY inert p. The Legendre test can
  never produce a non-square witness along this route.

### 4.2 The working certificate: p = 43 square-class **[P]**

Take **p = 43**: prime; `43 ≡ 1 (mod 21)` so the residue degree is
`f = ord₂₁(43) = 1`; `43 ≡ 3 (mod 4)`; `43 ∤ 453789` (unramified).
Then `p^f = 43 ≡ CRT(1 mod 21, 3 mod 4) = 43 (mod 84)` — exactly the
**nontrivial element** of `Gal(L/K₂₁) = ker((Z/84)*/±1 → (Z/21)*/±1) =
{1, [43]}`: σ₄₃ fixes ζ₂₁ and sends `w₂₁ ↦ −w₂₁`. So every prime of
K₂₁ over 43 is **inert in L/K₂₁**, which forces `s = 2 − ᾱ` to be a
non-square in each residue field. Measured directly (the check needs no
theory):

1. `m₂₁` **splits completely** mod 43 (`21 | 42 = 43 − 1`) with the 6
   roots `[11, 12, 13, 23, 33, 38]` — the residue fields of K₂₁ at 43
   are bare `F₄₃`.
2. For **every** root r: `(2 − r)²¹ ≡ −1 (mod 43)` (Euler criterion,
   `21 = (43−1)/2`) — measured `[42]` uniformly.
3. `m₂₁` is squarefree mod 43 (`gcd(m₂₁, m₂₁′) = 1`), so
   `F₄₃[x]/(m₂₁) ≅ F₄₃⁶` and the per-root Legendres lift to the ring.
4. **The ring witness**: `(2 − α)²¹ ≡ −1` in `F₄₃[x]/(m₂₁)` exactly.

**Conclusion chain**: a square of K₂₁ reduces to a square in EVERY
residue field; at 43, `s` is a non-residue ⟹ `s` is NOT a square in
K₂₁ ⟹ `x² − (2 − α)` is irreducible over K₂₁ ⟹
`[K₂₁(w₂₁) : K₂₁] = 2` with `w₂₁` real positive — the **PRIMARY
genuine-extension certificate at n = 21**.

**Corroboration (tower)**: `w₂₁² = 2 − α₂₁ ∈ K₂₁` ⟹ `Q(w₂₁) ⊇ K₂₁`;
`deg w₂₁ = 12 > 6 = [K₂₁:Q]` ⟹ `[Q(w₂₁):K₂₁] = 2` ⟹ `w₂₁ ∉ K₂₁`. The
M179 §7.2 degree theorem (`w_n ∉ K_n` for every odd n) still holds —
what fails at n = 21 is only the *norm route* to it; the square-class
witness is the n = 21-specific executable proof.

**Sign-engine tie-case impossibility** transfers by the same
certificate: `u² = s·v²` with `(u, v) ≠ (0, 0)` would give
`(u/v)² = s` ⟹ s a square ⟹ contradiction with p = 43. Note the
n9.rs mixed-sign arm's `N(s) = 3` debug-assert does **NOT** transfer
(`N_{K₂₁}(s) = N(2 − α) = 1`); the n = 21 tie-arm's unreachability
rests on the square-class certificate instead.

### 4.3 The K₁₇ contrast

At n = 17 the norm certificate is **not** vacuous: `m₁₇(2) = 17` is
prime, so M179's F3e route transfers verbatim (`w₁₇ ∉ K₁₇` by the norm
argument alone). The K₁₇ blocker is purely the §2 envelope — never the
extension certificate. This asymmetry (prime n vs composite n = 3·7) is
the clean statement of when each certificate applies.

---

## 5. Substitution-table field requirements per family **[P]**

What a Say-awen (or any §6-family) substitution table needs from the
field layer, now stated with the corrected §1.3 structure:

- **Positions**: K_n power-basis tuples in the oblique frame
  (`u, v ∈ Z[α]` over a common denominator), reduced through the RED
  fold table (`b^D..b^{2D−2}` rows).
- **Edge directions**: `(a, b·w)` split pairs — the x-coordinate is a
  difference of cosines (`a ∈ K_n`), the y-coordinate a difference of
  sines (`b ∈ K_n`, factored through `w`): `sin(kπ/n) = w·q_k/2` with
  the exact `q_k·cos_half(k) == q_{2k}` chain (§1.3). Never a flat
  degree-2·deg element.
- **Lengths/squared lengths**: split-pair arithmetic
  (`x² + s·y²`-shaped cores with `s = 2 − α`); unit edges at the /64
  walk are `64² = 4096` in numerator units (§3.1).
- **Sign/order**: the isolating-interval engine at the measured s₀
  (§2/§3), `m_increasing = true` at both fields, `BISECT_CAP = 4`.
- **Records**: `(u + v·w)/den` — the `n9::Nq` shape (`n9.rs` Nq sgn
  ~156-176 is the mixed-arm template; its `N(s) = 3` assert is
  n = 9-specific, see §4.2).

Per-family census (measured in the spike's H block — degrees certified
against the float/cosine fold chains):

| family | n | positions | edge-direction coords | N-layer? |
|---|---|---|---|---|
| Danzer 7 (+variants), G-S 7 rhomb, N-D deltoids | 7 | K₇ (deg 3) | `(K₇, w₇·K₇)` | yes, `w₇² = 2 − α₇` |
| Harriss 9 rhomb | 9 | K₉ (deg 3) | `(K₉, w₉·K₉)` | yes |
| CAST 10 | 10 | L₁₀ (deg 4) | **L₁₀ both** (`sin = cos` fold) | **no** (even n) |
| Say-awen 17 | 17 | K₁₇ (deg 8) | `(K₁₇, w₁₇·K₁₇)` | yes — **engine-blocked (§2)** |
| Say-awen 21 | 21 | K₂₁ (deg 6) | `(K₂₁, w₂₁·K₂₁)` | yes, certified by §4 |

---

## 6. The corrected §23i applicability table + replacement text

### 6.1 Current §23i (PROPOSED.md, verbatim — NOT edited in this landing)

> ### 23i. New-field tiers (cubic/quartic cyclotomic layers required)
>
> - **Danzer 7-fold (+variants)**, **Harriss 9-fold rhomb**,
>   **Goodman-Strauss 7-fold rhomb**, **CAST 10-fold**, the
>   **Nischke–Danzer deltoids**, **Say-awen 17/21-fold** — each needs
>   degree-3/4 arithmetic before any substitution table; effort XL per
>   family (design docs first; overlaps #18).

What is superseded: the "each needs degree-3/4 arithmetic" clause
(true only for the first five families) and the flat "design docs
first" status (the design docs for every family's field layer now
exist; Say-awen 17 is engine-blocked, not design-blocked).

### 6.2 Proposed §23i replacement (verbatim proposal — orchestrator-owned)

> ### 23i. New-field tiers (cyclotomic field layers, tiered by degree)
>
> - **Tier 1 — field shipped (degree 3/4):** **Danzer 7-fold
>   (+variants)**, **Goodman-Strauss 7-fold rhomb**, the
>   **Nischke–Danzer deltoids** (degree 3, K₇), **Harriss 9-fold rhomb**
>   (degree 3, K₉ + the Nq engine), **CAST 10-fold** (degree 4, L₁₀) —
>   each needs only substitution-table work; effort L–XL per family
>   (overlaps #18).
> - **Tier 2 — field designed (degree 5/6):** K₁₁ (M178) and K₁₃ (M179)
>   certify the D = 6 engine depth class; **Say-awen 21-fold** joins it
>   (degree 6 composite, s₀ = 21 battery green, genuine extension by a
>   p = 43 square-class witness — the prime-n norm certificate is
>   vacuous at n = 21, see `cyclotomic-field-17-21-design.md` §4);
>   effort XL per family.
> - **Tier 3 — field blocked at the engine wall (degree 8):**
>   **Say-awen 17-fold** needs K₁₇; the D = 8 stress test returns
>   NON-QUALIFICATION — no s in the a-priori range [11, 15] satisfies
>   the single-pass (R1) and i128 (R2) gates together
>   (`cyclotomic-field-17-21-design.md` §2) — blocked on an engine
>   decision (accumulator width or isolation scheme); the field
>   constants and certificates are complete.

### 6.3 Corrected applicability table (supersedes M179 §7.3's K₁₇/K₂₁ rows)

| family | n | field | deg | field-layer status | engine transfer |
|---|---|---|---|---|---|
| Danzer 7 (+variants), Goodman-Strauss 7 rhomb, Nischke–Danzer deltoids (ND96, 7-fold) | 7 | K₇ | 3 | **shipped** (`fields.rs`, M137/M149) | verbatim |
| Harriss 9-fold rhomb | 9 | K₉ | 3 | **shipped** (M139) + Nq (`n9.rs`, M171) | verbatim |
| CAST 10-fold | 10 | L₁₀ = Q(ζ₂₀)⁺ (disc 2000) | 4 | **shipped** (`ML10`, `fields.rs:1071`) | verbatim — no N-layer (even n) |
| Say-awen 21-fold | 21 | K₂₁ (composite 3·7) | 6 | **designed** (this doc §1.2, §3, §4) | **transfers verbatim** (D = 6 class, s₀ = 21, battery green) |
| Say-awen 17-fold | 17 | K₁₇ | 8 | **designed, constants certified** (this doc §1.1) | **BLOCKED — D = 8 NON-QUALIFICATION** (this doc §2) |

M179 §7.3's "design-level row only" rows for n = 17/21 are now
delivered: n = 21 is implementation-ready at the field layer; n = 17
carries a measured engine-wall verdict instead of a deferral.

---

## 7. Open questions

1. **The D = 8 engine decision** (the only blocker for Say-awen 17):
   wider accumulators vs a non-consecutive isolation scheme vs
   accepting a reduced battery envelope. The spike is the reusable
   stress harness — re-run unchanged after any engine change; the
   pre-registered R1/R2/R3 rule and the battery (66 elements, value-
   robust) are stable.
2. **Say-awen 21 implementation order**: the field layer is ready
   (§3); the substitution-table work (rhomb/pro data, WIS-style
   inference) is the XL remainder and is not addressed here.
3. **The K₂₁ tie-arm implementation shape**: the mixed-sign regression
   mandate carries over from n9.rs, but the unreachability assert must
   cite the square-class certificate (§4.2), not a norm value.
4. **Scratch note (orchestrator)**: the `kuniform-n17` scratch job is a
   tiling *rendering* task — it contains no field-layer material to
   salvage into this arc.

---

## 8. Verification appendix (spike evidence index)

Spike: `.scratch/sayawen-17-21-design/spike_k17_k21.py` — stdlib-only
Python (`math`, `fractions`, `functools`), exact integer/Fraction
arithmetic throughout; floats appear only as cross-checks (≤ 1e-9 /
1e-12 tolerances), never as evidence. **97/97 checks green**, log
`spike_k17_k21_output.txt` (banked beside the spike), rc = 0.

| doc claim | spike checks (measured values) |
|---|---|
| §1.1 m₁₇ constants | A1 (Newton `p₁..p₈`), A2 (Laurent == Φ₁₇), A3a-c (Eisenstein at 17 + Rabin p=3), A4 (conjugate sum −1), A5 (Sturm 8), A6 (`17⁷`), A7 (conductors), A8 (`m₁₇(2) = 17`) |
| §1.1 subfield/reduction | C1 (7 RED rows b⁸..b¹⁴, amplification `[54, 67, 399, 532, 2138, 3010, 10004]`), C2 (`m₁₇(b) = 0`), C3 (tower wrap), C4/C5 (`b′ = −C₈`, `q₂ = b′`), C7 (`η₁₇² = 17`) |
| §2 envelope | D1a (ISO@s15), D1b (α⁸ monomial 128 bits), D1c (INCREASING), D1d (psa 140), ladder 9..17, per-s gate table 11..15 (111/111/126/133/140), D2 verdict, D2a (monotone `{11:4, 12:2, 13:2, 14:1, 15:1}`), D2b (`mu = 0.2559 < 1`, pair (33, 41)) |
| §1.2 m₂₁ constants | F1 (Newton `p₁..p₆ = [1,13,1,41,1,139]`), F2 (Laurent), F3 (null-space, derivation 2), F4a-b (`m₂₁ mod 2` irreducible, `ord₂(21) = 6`), F4c (Φ₂₁ no witness), F4d (separable), F5/F5b (Sturm 6, sum +1), F6 (`453789`), F7 (conductors) |
| §1.2 subfields | F8 (`η₂₁² = 21`), F9/F9b/F9c (`K₇` periods, `y³ − y² − 2y + 1`, disc 49, floats) |
| §4 norm contrast | F10 (`m₂₁(2) = 1 = Φ₂₁(1)`, square), F11 (`Φ₂₁(2) = 2359 = 7·337` full-field norm) |
| §4 certificate | F12a-i (exponent 6 < 12, witness a = 5; DDF census `{5:{6:12}, 11:{6:12}, 13:{2:12}, 17:{6:12}, 19:{6:12}, 43:{2:12}}`), F12a-ii (p = 2 Frobenius square), F12a-iii a-d (roots `[11,12,13,23,33,38]`; per-root `(2−r)²¹ = −1`; squarefree; ring witness `[42]`), F12b (tower), F12c (`s` tuple), F12d (tie-case impossibility) |
| §3 envelope | G0a/G0b (hull closure; edges `4096 = 64²`), G0c (battery `71 = 74 − 3` dups, 2485 pairs), ladder 16..24 (undecided 0/0), G1 (candidates 129-kill/127/121/117/113 ⟹ s₀ = 21), G2a-c (ISO, INCREASING, `α < 2`), G3 (rounds 113/117/121/127), G4 (L1 ceiling 1.645e5 ≥ 26624), G5 (min mu 1.853e4), G6-G11 (0 bisections, min |val| 0.08885, gap 0.0233, order, single-pass comparators), G12 (118/118 bits) |
| §1.3 / §5 sine structure | H n=7/9/17/21 (cos fold chain; `q_k·cos_half(k) == q_{2k}` exact for k = 1..half; `s = 2 − α` tuples; `m_n(2) > 0`), H n=10 (even-n no-N-layer contrast) |
| §6 census rows | H census (degrees 3/3/4/8/6 for the five families) |

### Corrections ledger (vs the working notes / run3 baselines)

1. **Monotonicity**: DECREASING → **INCREASING** for both fields —
   `Iso.m_increasing = true`, same orientation as K₁₃ (the working-notes
   premise was wrong; all walls/s₀ re-measured under the True flag).
2. **Edge-length units**: `|edge|² = 4096 = 64²` in /64-numerator units
   (not 1) — both fields' edge checks corrected.
3. **K₁₇ D5 L1 ceiling**: degree-7 product `2^(127−7s)/x̂⁷` (the
   degree-8 form was wrong; dead code at `S0_17 = None`, fixed for
   fidelity).
4. **w₂₁ identity**: `2cos(19π/42) = 2cos(2π·19/84)` — the
   `2cos(19π/84)` shorthand was a typo.
5. **K₂₁ extension certificate**: norm route VACUOUS (N = 1); Rabin
   route IMPOSSIBLE (exponent theorem + census); p = 43 square-class
   is THE certificate (the working-notes Rabin search was fruitless by
   necessity, and the `[41, 42]` poly-element note was wrong — the
   element is `[2, 42]`).
6. **α₂₁ ≈ 1.91115** (the working-notes 1.95629 conjugate list was
   garbage; roots re-listed in §1.2).
7. **K₂₁ battery**: verbatim K₁₃ 6-coord shapes (71 elements) — the
   padded 7-coord rows of run3 were inventions and are deleted; s₀ =
   **21** under the corrected flag (run3's 20 was flag- and
   battery-dependent).
8. **D = 8 wall values**: run3's False-flag walls (142/132/~125) were
   flag-dependent; corrected True-flag walls are 111/111/126/133/140.
   The verdict partition is unchanged (R2 kills {14, 15}; R1 kills
   {11, 12, 13}) — the finding is flag-robust, the per-s reasons are
   now measured.
9. **H n=10 fold**: `% 10` → `|5 − k|` (cos is even; a latent sign bug
   first exercised by run6 — run3 crashed in G before H ran).
10. **DDF census + exponent theorem**: run3's fruitless Rabin search is
    now a proof (§4.1), with the pre-registered census classes measured
    exactly.