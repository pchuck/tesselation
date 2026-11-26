# Degree-5 cyclotomic field layer — K₁₁ = Q(2cos(2π/11)) — design doc

Status: **PROPOSED** — design delivered and spike-validated; **no code in
this landing** (docs-only). Companion spike:
`.scratch/degree5-fields-design/spike_k11.py` (stdlib-only Python, exact
integer/Fraction arithmetic, **48/48 checks green**, log
`spike_k11_output.txt`). Scope: unblocks the 24c n = 11 monster family
(M169 six parallel-pair records, c ∈ {2, 3, 4}) and the M171 odd-n stall
at degree 5 (PROPOSED §24c: "n = 11 remains blocked on degree-5
fields"). Design mirror: [`cyclotomic-field-layer.md`](cyclotomic-field-layer.md)
(degrees 3/4, LANDED) — same section layout, same citation markers:
**[P]** = proven by exact algebra below (the spike's assertions are the
executable proof); **[C]** = cited (shipped code, dossier, or
literature).

---

## 0. One-paragraph summary

Represent n = 11 tiling points in the **oblique frame** `(U₀, U₁)` with
coordinates in the degree-5 field `K₁₁ = Q(α), α = 2cos(2π/11)` — the
first odd-prime field past the cubic/quartic layer, with minpoly
`m(y) = y⁵ + y⁴ − 4y³ − 3y² + 3y + 1`, ring of integers `Z[α]`
(discriminant `11⁴`), and **no proper subfields** (degree 5 is prime —
the K₁₅ ⊃ Q(√5) complication does not arise). Sines never enter the
core: every sine of a multiple of π/11 factors as `w · (K₁₁ element)`
with `w = 2sin(π/11)`, `w² = 2 − α`, a **genuine quadratic extension**
(norm certificate `N(2 − α) = 11`, not a square). Records store
`(u + v·w)/den` with `u, v ∈ Z[α]` — the exact `n9::Nq` shape at one
degree higher. Sign/order reuses the shipped isolating-interval
machinery verbatim (`ksign` + `Iso` at `s = 26`), and the extension's
two-level sign engine must use the **full mixed-sign product rule**
`sgn(u + vw) = sgn(u)·sgn(u² − s·v²)` — the spike exhibits a
mixed-sign element where a `sign(u)`-only shortcut (which the shipped
`n9::Nq::sgn` mixed-sign arm effectively is; see §4.3) returns the
wrong answer. Everything below is certified by the spike; no Rust
changes are proposed in this document.

---

## 1. Field selection and the odd-n sine rule **[P]**

- `K₁₁ := Q(ζ₁₁)⁺ = Q(α)`, `α = 2cos(2π/11)`, `deg = φ(11)/2 = 5`.
  Generator standardized at the **2π angle** (frame algebra needs it;
  deg-3/4 doc §1).
- Minimal polynomial (two independent derivations, spike A1–A2):
  `m₁₁(y) = y⁵ + y⁴ − 4y³ − 3y² + 3y + 1`.
  - *Laurent path:* `Φ₁₁(x) = x⁵·m₁₁(x + x⁻¹)` — verified coefficient-
    exact as a Laurent identity (exponents −5..5 collapse to 0..10).
  - *Newton path:* orbit power sums `pⱼ = Σₖ (ζᵏ + ζ⁻ᵏ)ʲ` via the
    Ramanujan sum `c₁₁(t) = 10 if 11|t else −1`:
    `p₁..p₅ = −1, 9, −4, 25, −16`; Newton identities reproduce `m₁₁`
    integrally.
  - *Irreducibility (⇒ this is THE minpoly):* `Φ₁₁(x+1) = Σⱼ C(11,j)x^{j−1}`
    is Eisenstein at 11 ⇒ `Φ₁₁` irreducible ⇒ `[Q(ζ₁₁):Q] = 10`; ζ₁₁
    non-real ⇒ `[K₁₁:Q] = 5` = deg m₁₁ (spike A3a–A3c: also mod-2
    irreducible and `gcd(m, m′) = 1`).
- Conjugates = `2cos(2πk/11)`, k = 1..5 (Sturm census: exactly 5 real
  roots, one per orbit bracket; spike B1–B2):

  | k | value (float, cross-check only) |
  |---|---------------------------------|
  | 1 | +1.6825070656623624 |
  | 2 | +0.8308300260055631 |
  | 3 | −0.2846296765465706 |
  | 4 | −1.3097214674034323 |
  | 5 | −1.9189859472289947 |

  Sum = −1 = −(coefficient of y⁴) ✓.
- **No subfields:** 5 is prime ⇒ `K₁₁` has no intermediate field —
  unlike `K₁₅ ⊃ Q(√5)` (deg-3/4 doc §8.3) there is no subfield probe,
  no trace/norm splitting trick, no composite-degree reduction. The
  whole record lives at one degree.
- **Ring of integers = Z[α]:** `disc(m₁₁) = Res(m₁₁, m₁₁′) = 11⁴ =
  14641` exactly (spike B7, fraction-free Bareiss). For the prime-p
  real subfield the field discriminant is `p^{(p−3)/2} = 11⁴`; equality
  with the polynomial discriminant ⇒ index 1 ⇒ **`Z[α]` is the maximal
  order** — the D = 1 core needs no denominator, ever (same lucky
  position as K₇/K₉/K₁₅; L₁₀ is the exception, not the rule).
- **Odd-n quadratic sine extension (the n9 pattern at degree 5):**
  define `w = 2sin(π/11)`. Then:
  - `w² = 2 − α` **[P]** (`4sin²(θ/2) = 2 − 2cos θ` at θ = 2π/11);
  - `w ∈ Q(ζ₄₄)⁺` and `[Q(ζ₄₄)⁺ : K₁₁] = 10/5 = 2` **[P]** (φ(44) = 20);
  - the extension is **genuine**: `2 − α` is not a square in `K₁₁`,
    because a square has square norm and `N(2 − α) = m₁₁(2) = 11` is
    prime (spike G2) — the exact n9 certificate (`N(s) = 3` there)
    one field up;
  - `w > 0` exactly: Sturm shows m₁₁ root-free on [2, 21/10] with
    m₁₁(2) > 0 ⇒ α < 2 ⇒ s := 2 − α > 0 (spike G3);
  - **one w suffices:** `2sin(2πk/11) = w·q_k·cos(πk/11)` with
    `q_k = sin(πk/11)/sin(π/11) ∈ Z[α]` (Chebyshev-U at `b′/2`,
    `b′ = 2cos(π/11)`; integrality certified — spike G4, q-table in
    §3) and `cos(πk/11) ∈ K₁₁`. So every hull coordinate is
    `(K₁₁ element) + w·(K₁₁ element)` — the two-level shape, never a
    degree-10 flat element.

---

## 2. Element representation, reduction, isolation — and the RECOMMENDATION

### 2.1 Core tuples and reduction table

Element = `[i64; 5]`, low→high: `c₀ + c₁α + c₂α² + c₃α³ + c₄α⁴`,
denominator 1 (§1: Z[α] maximal). Minpoly low→high
`MK11: [i64; 6] = [1, 3, -3, -4, 1, 1]`. Reduction rows
`RED11: [[i64; 5]; 4]` (b⁵..b⁸, spike C1–C6, each row float-certified
≤ 1e-9 and the m(b) = 0 fold is exact):

| power | row (low→high) |
|-------|----------------|
| b⁵ | `[-1, -3, 3, 4, -1]` |
| b⁶ | `[1, 2, -6, -1, 5]` |
| b⁷ | `[-5, -14, 17, 14, -6]` |
| b⁸ | `[6, 13, -32, -7, 20]` |

Wrap certificate (spike C6): computing `C_k = y·C_{k−1} − C_{k−2}`
through k = 11 with these folds reproduces `C_k(α) = C_{11−k}(α)`
**exactly** (k = 1..5) and `C₁₁(α) = 2` — the reduction rows are
consistent across the whole orbit, a strong RED11-only check.

All generic ops (`kadd/ksub/kneg/kscale/kmul/koblique_*`) are already
const-generic over D in `fields.rs` — K₁₁ is a pure data + tests block
**[C]** (the L₁₀ block is the template: `L10`/`ML10`/`RED_L10`/
`ISO_L10`/`kval10`).

### 2.2 Isolating interval and the D = 5 envelope (op bounds at real magnitudes)

`ISO11: Iso { lo_num: 112911137, hi_num: 112911138, s: 26,
m_increasing: true }` **[P]** — the bracket is found by exact integer
sign of `m₁₁(n/2²⁶)·2^{5·26}` (no floats), Sturm-isolated (exactly one
root inside), and `m₁₁′ = 5y⁴ + 4y³ − 12y² − 6y + 3` is root-free on
the bracket with `m₁₁′(lo) > 0` ⇒ strictly increasing ⇒ the bisection
half-selection rule of the shipped `ksign` applies verbatim (spike
B3–B5). The i64 numerator width matches ISO9/ISO15/ISO_L10 (all s = 26)
**[C]**.

Envelope at real magnitudes (spike F + H, measured):
- Battery heights ≤ 160 (8 bits); `kmul` convolution peak
  ≤ 25·160² ≈ 2²⁰ before the fold, final tuples ≤ 40 bits — the i64
  result hard-error path is never approached at /64-grid shapes **[P]**.
- `interval_horner` accumulator at s = 26: **112 bits** worst element,
  **113 bits** worst pairwise comparator difference (all 1485 pairs
  measured) — i64-impossible, i128-safe **[P]**.
- Each bisection round lifts the accumulator scale by `deg−1 = 4`
  bits (sa = 4·s′) and the `poly_sign_at(m, ·)` peak by the same net
  amount (deg·s′ +5, near-root shrink −1): **3 rounds certified**
  (2¹¹³⁺¹² = 2¹²⁵ < 2¹²⁷); a 4th round at max battery heights would sit
  at 2¹²⁹ — outside i128 (see §8, open question; the battery pins
  single-pass so the cap is never exercised in tests).

**Why not a flat degree-10 representation:** the natural alternative —
store `u + v·w` as one degree-10 element of `Q(ζ₄₄)⁺` — needs
`interval_horner` at `sa = 9s = 234` bits at s = 26: i128 impossible,
and dropping to s ≈ 14 loses the shared 2⁻²⁶ grid. Two-level wins
decisively and keeps the sign engine's cost profile identical to n9.

### 2.3 The extension type

`N11 { u: K11, v: K11, den: i64 }` = `(u + v·w)/den`, `den > 0`,
gcd-reduced over all 11 integers (the `n9::Nq` convention **[C]**).
Multiplication: `(u₁ + v₁w)(u₂ + v₂w) = (u₁u₂ + s·v₁v₂) +
(u₁v₂ + u₂v₁)w` with `s = 2 − α` stored as the K₁₁ constant
`[2, -1, 0, 0, 0]`. Addition/subtraction cross-multiplies
denominators; no division anywhere.

### 2.4 RECOMMENDATION

Adopt the **two-level shape**: `K11` core tuples (D = 5, denominator 1)
in `fields.rs` as a data + tests block mirroring `L10`, plus an
`N11` quadratic-extension type in a future `monster/n11.rs` mirroring
`n9::Nq` — *not* a flat degree-10 field (§2.2), *not* a rational/Float
retreat, *not* a generic power-of-ζ table. Reasons: (a) the 24c/M169
and M171 records need exactly `(K₁₁, w·K₁₁)` coordinates (§1: every
hull coordinate factors through one w); (b) the sign engine stays at
the certified 113-bit envelope instead of an impossible 234-bit one;
(c) the review surface is two small blocks with landed precedents
(L₁₀ block, n9 module) rather than a new arithmetic kernel; (d) the
sine discipline ("sines live as w-multiples, never as stored field
elements") carries over verbatim from the deg-3/4 design.

---

## 3. Exact constants and their certificates **[P]**

All tuples low→high in Z[α]. Certified exact by the spike (floats in
parentheses, cross-check only).

- **Chebyshev tower** `C_k(α) = 2cos(2πk/11)` (recursion
  `C_k = y·C_{k−1} − C_{k−2}`, `C₀ = 2`, `C₁ = y`):

  | k | tuple | float |
  |---|-------|-------|
  | 0 | `[2, 0, 0, 0, 0]` | 2 |
  | 1 | `[0, 1, 0, 0, 0]` | α |
  | 2 | `[-2, 0, 1, 0, 0]` | 0.83083003 |
  | 3 | `[0, -3, 0, 1, 0]` | −0.28462968 |
  | 4 | `[2, 0, -4, 0, 1]` | −1.30972147 |
  | 5 | `[-1, 2, 3, -1, -1]` | −1.91898595 |

  plus the wrap identities `C₆..C₁₀ = C₅..C₁`, `C₁₁ = C₀` (spike C6).
- **`b′ = 2cos(π/11) = −C₅(α) = [1, -2, -3, 1, 1]`** (1.91898595) —
  the π/11-rotation cosine, in Z[α].
- **Sine ratios** `q_j = sin(πj/11)/sin(π/11) = U_{j−1}(b′/2)` — all
  in **Z[α] at denominator 1** (integrality asserted exactly; the
  `2^{j−1}·U_{j−1}(b′/2)` computation must use *raw* Chebyshev-U
  coefficients — field-folded ones are wrong from j = 6 on; spike G4
  asserts both the integrality and the resulting unit edges):

  | j | q_j tuple |
  |---|-----------|
  | 1 | `[1, 0, 0, 0, 0]` |
  | 2 | `[1, -2, -3, 1, 1]` (= b′) |
  | 3 | `[1, 1, 0, 0, 0]` (= 1 + α) |
  | 4 | `[-1, -2, 1, 1, 0]` |
  | 5 | `[-1, 1, 1, 0, 0]` |

  (q₆..q₁₀ = q₅..q₁ by the same reflection symmetry.)
- **Angle-pair pins (factor-2 forms):** `4cos²(2π/11) = α²` and
  `4sin²(2π/11) = 4 − α² = [4, 0, -1, 0, 0]`, summing to 4 exactly
  (spike G9). Watch the trap the spike fell into and documents:
  `2sin²(2π/11)` is a **half-integer** element — the integral pin is
  the factor-4 form.
- **Norm facts:** `N(α) = Παᵢ = −m₁₁(0) = −1` (α is a unit — the
  cyclotomic real-subfield unit; `m(0) = (−1)⁵·Παᵢ`), `N(2 − α) = m₁₁(2) = 11` (prime ⇒
  the extension certificate), `N(s) = 11` non-square ⇒ `u² = s·v²`
  has no nonzero solution in K₁₁ (§4.2).
- **Star-walk shoelace factors** `{11/d}`: `Σⱼ (AⱼBⱼ₊₁ − Aⱼ₊₁Bⱼ)` as
  exact K₁₁ tuples (area = |factor|·sin(2π/11)/2):

  | walk | factor tuple | area (float) |
  |------|--------------|--------------|
  | {11/1} | `[5, 1, 2, 3, 1]` | 9.365639907 |
  | {11/2} | `[-3, -5, 12, 7, -5]` | 4.279083526 |
  | {11/3} | `[-13, -7, 30, 1, -7]` | 2.382888564 |
  | {11/4} | `[15, -19, -16, 9, 3]` | 1.255882919 |
  | {11/5} | `[1, -13, -26, 5, 9]` | 0.395390308 |

  Walks close exactly, are R^d-equivariant about the exact centroid,
  and the exact shoelace × sin agrees with the float area ≤ 1e-9
  (spike E1) — the exact exit/outline machinery the M169 records need.

---

## 4. Exact sign and order — the isolating-interval machinery at D = 5

### 4.1 Core K₁₁ signs

Verbatim reuse of shipped `ksign(x, m, iso)` **[C]**: interval-Horner
of the degree-4 tuple-polynomial over `ISO11`; undecided ⇒ bisect
keeping the half containing α via exact `poly_sign_at(m₁₁, mid)` and
the certified `m_increasing = true`. The spike simulates this engine
exactly (same interval arithmetic, same bisection rule): **all 55
battery elements and all 1485 pairwise comparator differences resolve
with 0 bisections** at s = 26, exact order == float order, min
pairwise gap 0.0559 (spike F1–F5). Zero test: a degree-<5 tuple
evaluates to 0 iff it is the zero tuple (m₁₁ irreducible ⇒ the power
basis is faithful) — `eq` is tuple equality after gcd reduction.

### 4.2 Extension N₁₁ signs — the two-level product rule

`sgn((u + vw)/den)`, den > 0, reduces to K₁₁ signs:
1. `sgn(u) = Equal` ⇒ `sgn = sgn(v)` (and conversely);
2. `sgn(u) = sgn(v)` (both positive or both negative) ⇒ `sgn = sgn(u)`
   — exact because `s = 2 − α > 0` (§1), so `u + vw` has the common
   sign of its parts;
3. **mixed signs ⇒ the product rule:**
   `sgn(u + vw) = sgn(u) · sgn(u² − s·v²)`.
   Proof sketch **[P]**: `x·(u − vw) = u² − s·v²`, and in each mixed
   case the conjugate factor `u − vw` has sign `sgn(u)`
   (u > 0, v < 0: `u − vw = u + |v|w > 0`; u < 0, v > 0: `u − vw < 0`),
   so dividing the exact product sign by `sgn(u)` (an involution)
   gives the rule. The zero case `u² = s·v²` with `(u, v) ≠ 0` is
   **impossible**: it would make s a square in K₁₁, hence
   `N(s) = N(z)²` a rational square — but `N(s) = 11` (§3). Both
   factors are exact `ksign` calls on degree-4 tuples.

### 4.3 The n9 finding (consumer-contract caution) **[P]**

While inventorying the n9 precedent, the spike's G8 regression exposed
a discrepancy in the **shipped** `n9::Nq::sgn` mixed-sign arm
(n9.rs:156–171): the code computes
`su.then_with(|| sign(u² − s·v²))`, and Rust's
`Ordering::then_with` returns `self` unless the receiver is `Equal`
(verified by compiled snippet) — so the shipped arm returns
`sgn(u)` **unconditionally**, while the arm's own doc comment states
the product rule `sgn(u)·sgn(u² − sv²)`. The two disagree exactly on
mixed-sign elements with `u² < s·v²`; worked counterexample at n = 9:
`u = 1, v = −2` ⇒ `x = 1 − 2w₉ ≈ −0.368 < 0` (correct: Less) but the
shipped arm returns Greater. The n9 battery (3 sgn assertions) never
exercises that subset — the geometry apparently never produces a
mixed-sign `u² < sv²` element, so this is a **latent, untriggered**
hazard, not an active wrong-answer bug. Consequences for THIS design:
- the N₁₁ spec (§4.2) must carry the **full product rule**; the spike's
  G8 case (`1 − 2w < 0`, product rule ⇒ Less, shortcut ⇒ Greater) is
  the mandatory battery element (§6.3);
- the n9 code fix + battery gap-fill is a **separate code job** (this
  landing is docs-only; no fields.rs/n9.rs edits here).

---

## 5. Consumer contract at the fields.rs seam (signature-level; no code)

### 5.1 The fields.rs K₁₁ block (mirrors the L₁₀ block)

```rust
pub struct K11(pub [i64; 5]);            // c0 + c1*a + ... + c4*a^4, den 1
pub const MK11: [i64; 6] = [1, 3, -3, -4, 1, 1];   // low->high minpoly
pub const RED11: [[i64; 5]; 4] = [        // b^5..b^8 fold rows, low->high
    [-1, -3,   3,  4, -1],
    [ 1,  2,  -6, -1,  5],
    [-5, -14, 17, 14, -6],
    [ 6, 13, -32, -7, 20],
];
pub const ISO11: Iso = Iso {
    lo_num: 112911137, hi_num: 112911138, s: 26, m_increasing: true,
};
pub fn kval11(x: [i64; 5]) -> f64;        // float view, emission only
```

Everything else is the existing const-generic layer: `kadd/ksub/kneg/
kscale/kmul::<5>`, `ksign`, `knorm`, `koblique_norm2/dot2/cross/rot`
**[C]**. No new arithmetic kernels.

### 5.2 Field tag and medallion wiring

- `medallion::Field` gains `K11` — additive variant, documented exactly
  like `Field::K9` ("the record coordinates live in its quadratic
  extension by w = 2sin(π/11), w² = 2 − α; see `monster::n11`"), with
  zero other consumers at its landing (grep-verified — the K9
  disclosed-exception pattern **[C]**).
- Future `monster/n11.rs`: `N11` implements the generic `FieldEl`
  trait (`add/sub/neg/mul/sgn/eq_x/fl/zero_like/int_like/scaled_div`)
  with `sgn` = §4.2; the generic point layer
  (`gadd/gdot/gcrossings/outlines/hull_sanity/star_exits`) then works
  over N11 unchanged **[C]**.
- `Medallion` for the M169 records: `FIELD = Field::K11`;
  `contact_cos2` returns `None` — degree-5 cos² pins do not fit the
  `(a + b√d)/den` quadratic contract (same as L10 **[C]**); exact
  angle work goes through cross-multiplied K₁₁ identities instead.
- **/64 grid convention:** raw hull-grid numerators arrive at the
  fixed denominator 64 (the BATTERY_L10 real-numerator principle
  **[C]**); stored N₁₁ elements are gcd-reduced `(u, v, den)` (the n9
  convention). At n = 11 the hull battery's canonical coefficients
  stay ≤ 128 at den 64 (measured, §6.2).

### 5.3 The 24c/M169 + M171 consumers

PROPOSED §24c frames M169 as six parallel-pair records at n = 11,
c ∈ {2, 3, 4} ("n = 11 remains blocked on degree-5 fields"). This doc
is the unblocking design: hull = regular 11-gon (§6.2's exact /64
hull), pin rule `P = Mᵢ − R·Mⱼ` with `R` the π/11 half-step, d-rule
`d ≡ 2(i − j) − 1 (mod 11)` (M171 odd-n rules **[C]**). The spike
works one full example at the /64 grid (spike G6–G7):
`P = M₃ − R_{π/11}M₇ = ((α + 3α² + α³)/4,
−(2 + 9α + 6α² − 3α³ − 2α⁴)·w/4)` — float (3.734472166, −0.536936037),
exact component signs (+, −), d-rule value 2. Note the pin's
coordinates are u-pure in x and w-pure in y at this (i, j) — the
two-level storage pays off directly.

---

## 6. BATTERY_K11 design (mirrors BATTERY_L10's real-numerator principle)

### 6.1 Sign/order battery — 55 elements, three families

Mirroring `BATTERY_L10` (raw /64-scale shapes with realistic heights +
exact field pins + strap/exit-shaped numerators; floats never decide):
1. **Raw /64 hull-grid tuples** (18): heights ≤ 160 — the girih-style
   raw grid transplanted to D = 5;
2. **Exact field pins** (14): tower `C₂..C₅`, `s = 2 − α`, `4sin²`,
   `5 − α²`, `α² − 1`, constants, `α³ − 3`, `11α² − 15` — every
   deg-4-reachable pin shape;
3. **Strap/exit/outward shapes** (23): mixed-height `b·q − c·b³`
   families and degree-4/5 probes (`3α⁴`, `5α⁴ − 2α³`, the all-ones
   tuple).

Assertions (all exact, spike F): exact sign == float sign on all 55;
all 55 float values distinct (min gap 0.0559); no element within 1e−6
of 0; exact total order (comparator = sign of difference) matches the
float sort; **single-pass** — 0 bisections on every element and every
one of the 1485 pairwise comparator differences (the L10
single-pass discipline **[C]**).

### 6.2 Geometric battery (the N₁₁ layer, new vs. L10)

- **/64 hull walk:** `V_k = Σ_{j<k} (cos 2πj/11, sin 2πj/11)` at den 64
  — x-numerators `32·C_j(α)` (u-part), y-numerators `32·q_j·c_j(α)`
  (v-part, w-multiple). Asserts: exact closure `V₁₁ = V₀`; all 11 edge
  lengths `|V_{k+1} − V_k|² = 1` **exactly as N₁₁ elements** (worst
  canonical coefficient 1!); canonical coefficients ≤ 128 (spike G4).
- **Companion rotation about the exact centroid:** subtract c = ΣV/11
  first, then `R_{2π/11}(V_k − c) = V_{k+1} − c` for all k — exact N₁₁
  (spike G5; note the subtraction must precede rotation — rotating
  about the origin fails at k = 0, the spike's own worked trap).
- **Pin displacement worked example** (§5.3) with exact signs.
- **Mixed-sign regression:** `1 − 2w ≈ −0.1269 < 0` must sign Less via
  the product rule (a sign(u)-only shortcut returns Greater) — the
  §4.3 case, mandatory.

### 6.3 What the n9 battery must gain (flag for the n9 fix job)

The N₁₁ battery includes mixed-sign elements with `u² < s·v²` by
construction (§6.2 last bullet). The n9 battery has none (§4.3) — the
fix job should add the n9 analog (`1 − 2w₉`, `s = 2 − b₉`) alongside
the `Nq::sgn` correction.

---

## 7. n = 13 generalization shape (degree 6)

Same construction one prime up, for the record (no n = 13 consumer is
proposed yet):

- `m₁₃(y) = y⁶ + y⁵ − 5y⁴ − 4y³ + 6y² + 3y − 1` (Laurent identity
  `x⁶·m₁₃(x + x⁻¹) = Φ₁₃` exact; Sturm census 6 real roots; spike I1–I2);
- `disc(m₁₃) = 13⁵ = 371293` (spike I4) ⇒ `Z[2cos(2π/13)]` maximal,
  degree 6 prime ⇒ still no subfields;
- `ISO13 = [118843896, 118843897]/2²⁶`, same s = 26 convention, dyadic
  bracket exact (spike I3);
- `N(2 − b₁₃) = m₁₃(2) = 13`, prime — the same genuine-extension
  certificate (hand arithmetic; the future n = 13 spike re-certifies);
- **D = 6 envelope deltas** (design-ahead notes): `interval_horner`
  scale becomes `sa = 5s` (+25% per round vs D = 5 — measured D = 5
  peaks 113 bits ⇒ D = 6 ≈ 2^{135.6} at s = 26: **over i128**; an
  n = 13 landing must either drop s₀ (a 2⁻²⁴ grid still resolves the
  same gaps) or gcd-normalize/split the accumulator (the deg-3/4 doc
  §8.3 flag); `MK13` carries 7 coefficients, `RED13` has 5 rows; the
  Chebyshev wrap certificate extends unchanged (`C_k = C_{13−k}`,
  `C₁₃ = 2`).

---

## 8. Open questions

1. **n9 `Nq::sgn` mixed-sign correction** (§4.3): one-arm fix + battery
   gap-fill (`1 − 2w₉` case) — separate code job, docs-only here.
   Verify first that no shipped n9 geometry path can hit the subset
   (`u² < s·v²` on mixed signs) before treating it as latent-only.
2. **BISECT_CAP at D = 5** (§2.2): measured envelope certifies 3
   bisection rounds; the generic CAP = 4 would touch 2¹²⁹ at max
   battery heights in round 4. Options: (a) keep CAP = 4 and pin
   single-pass in the battery (current L10 discipline — the cap is
   then dead code in tests); (b) document a K₁₁-specific 3-round
   bound in the block comment; (c) drop s₀ to 24 for +8 bits of
   headroom. Recommendation: (a) now, revisit if a consumer produces
   near-collinear comparators.
3. **N₁₁ denominator-growth policy:** gcd-reduce-per-op (n9) vs the
   /64-closed convention (girih Kq, CHANGELOG-flagged residual).
   Measure both at the M169/M171 record shapes before the impl job.
4. **Exit rays at n = 11:** M169's per-record exits need the
   {11/2}/{11/3}/{11/4} star-exit direction sets — the §3 walk
   machinery produces them exactly; float emission for the consumer
   view only. Design these shapes when M169 lands.
5. **2sin(2π/11)-shaped elements:** they always factor as `w·(K₁₁
   element)` (§1) — keep them as v-parts, never as degree-5 core
   elements; worth an assert in the impl job's battery (the spike's
   hull construction is the existence proof).

---

## 9. Verification appendix (spike evidence index)

Spike: `.scratch/degree5-fields-design/spike_k11.py` — stdlib-only
Python, integer/Fraction exact arithmetic, f64 in cross-checks only.
**48/48 checks green** (log `spike_k11_output.txt`; rc = 0). Section
map:

| part | certifies | checks |
|------|-----------|--------|
| A | m₁₁ two paths (Laurent + Newton), Eisenstein/mod-2/separability | A1–A4 |
| B | Sturm census 5, orbit brackets, ISO11 bracket exact, monotonicity, disc = 11⁴ | B1–B8 |
| C | RED11 rows float-certified, m(b) = 0 exact, tower wrap identities | C1–C6 |
| D | U(k) frame: |U|² = 1, ΣU = 0, companion rotation, integrality, float dirs | D1–D6 |
| E | star walks {11/1..11/5}: closure, R^d-equivariance, exact shoelace | E1 |
| F | 55-element battery: signs, distinctness, total order, 0 bisections | F1–F5 |
| G | N₁₁: w² = 2−α, N(2−α) = 11, s > 0, /64 hull unit edges, centroid rotation, pin + d-rule, mixed-sign regression, 4cos²+4sin² = 4 | G1–G9 |
| H | bit sizes: 112/113-bit Horner peaks, 8-bit battery heights | H1–H2 |
| I | n = 13 probe: m₁₃, census 6, ISO13, disc = 13⁵ | I1–I4 |

Discipline (matching the 24e independent-mirror bar): derivation and
certification live Python-side; the future Rust job carries only the
certified constants of §2–§3 and re-asserts them against floats in
tests (`BATTERY_K11` + the geometric battery of §6). Spike notes worth
preserving: the raw-vs-field Chebyshev-U trap (§3), the rotate-after-
centering trap (§6.2), the 2sin² half-integer trap (§3), and the
`then_with` semantics trap (§4.3) are all documented where they bit.