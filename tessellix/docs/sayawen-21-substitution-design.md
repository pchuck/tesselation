# Say-awen 21-fold substitution design — prototiles, exact inflation in K₂₁, implementation-ready spec (M-number pending, docs-only, spike-validated)

Family: Say-awen 21-fold (§23i Tier-2 candidate; field layer shipped by
`cyclotomic-field-17-21-design.md` (M183) §1.2/§3/§4). This doc is the
substitution-layer design: prototiles, the exact inflation
`σ₂₁: T_{21,k} ↦ μ₂₁·T_{21,k}` dissection data, and the
implementation-ready spec for the engine landing. Docs-only +
stdlib-exact spike; no engine code, no registry edits, no other doc
edits. Companion arc: M179 (K₁₃ field layer), M181 (medallion 24c spec
bar), M183 (K₁₇/K₂₁ field layers), #18 (engine substitution path).

## 0. Verdict box

- **Canonical-source verdict: PARTIAL — gate observed, no invention.**
  The Say-awen paper (arXiv:2408.17082v2, "substitution rules … with
  infinite local complexity", n ∈ {13, 17, 21}) fixes **text-exactly**:
  the prototile definitions `T_{n,k}` (legs 1, apex angle
  `(n−2k)π/n`, base `s_{n,k} = 2cos(kπ/n)`, with the pseudo-prototile
  `s_{n,0} := 1`), the leg/base repetition equations
  (`eq:legrep`, `eq:baserep`), the substitution matrix `M_n` and its
  characteristic polynomial factorization, the inflation
  `μ_n = 1/s_{n,(n−1)/2}` with `g_n` its (non-Pisot) minimal
  polynomial, the coordinate-vector table (pure length algebra in
  `Z[μ_n]`), and the KSK criterion used to obtain dissections. It is
  **figure-only** about: the interior dissection layouts, the per-side
  tile *orders*, and the rhomb fill of each `σ_n(T_{n,k})` — these are
  not transcribable from text and are **not claimed as canonical data
  here** (§1). Everything interior in this doc is labeled *derived* and
  is spike-certified exact in `K₂₁ × w₂₁K₂₁`.
- **Substitution summary:** boundary multisets per side are
  text-forced by `eq:legrep`/`eq:baserep` (left leg = even index set
  incl. the pseudo-tile, right leg = odd set, base = `{T₀…T_{10−k}}`);
  the interior is filled by rhomb units (two isosceles halves glued
  base-to-base), for which this doc gives the complete exact
  **two-family attachment algebra** (40 attachment geometries per
  pending edge — §5.2; the one-family enumeration is a proven
  half-coverage bug, fixed and regression-verified this session).
- **Spike evidence:** Stage A field battery 64/64 green; Stage B
  cascade search + full in-leaf verification battery; Stage C
  independent rebuild + census battery. Certified rows of the count
  matrix `N` with exact area closure
  `N·a = μ₂₁²·a`: **k = 1, 2, 10** (Stage C ALL OK on all rows; k = 2
  verified on all fifteen of its exhaustive solutions). Both k = 2 and
  k = 9 exhaustions **complete**: k = 2 has exactly 15 unique
  dissections all with one `N` (forced within the vocabulary); k = 9
  has **no dissection at all** in the spike's construction vocabulary
  (158,107-node full tree, 0 solutions — §6.1; the vocabulary hedge
  has proven content). k = 3 and k = 8 cap-bounded (400k nodes each,
  0 solutions — §6.2, consistent with the paper's interrupted-chain
  remark); k = 4..7 open
  at a quantified achievable-set resource wall (§7.2).
- **D = 8:** not applicable to this family — see §9 (one section, as
  pre-agreed).

## 1. Canonical source and the pre-registered stop gate

### 1.1 Source

Say-awen (single-author surname per the arc's citation practice),
arXiv:2408.17082v2 (2024/2025), sections read as TeX text
(`sec3construction.tex`, `sec4algebraic.tex`, `sec5ILC.tex`): the
three substitution rules (n = 13, 17, 21), the algebraic section for
`μ_n`, and the ILC section via Danzer's algorithm. The rules generate
tilings with infinite local complexity; the substitution factor is
**non-Pisot**; each rule's tiling space contains a tiling with global
n-fold rotational symmetry, n ∈ {13, 17, 21} (abstract, l. 83,
text-exact).

### 1.2 What the text fixes (text-exact, quoted)

- **Prototiles** (sec3): isosceles triangles `T_{n,k}`, k = 0..(n−1)/2,
  legs of length 1, apex angle `(n−2k)π/n`, base
  `s_{n,k} = 2cos(kπ/n)`. The pseudo-prototile:
  > "We define `T_{17,0}` as the horizontal segment of length
  > `s_{17,0} = 1`, and refer to it as a *pseudo-prototile*."
  (sec3 l. 132–133, stated for n = 17; the definition is family-wide.)
  So **`s_{n,0} := 1` by definition** — the even index set always
  contains a length-1 segment, and `T_{n,0}` contributes area zero.
- **Boundary repetition** (sec3, `eq:legrep`/`eq:baserep`, verbatim):
  `μ_n = Σ_{j∈I_{n,O}} s_{n,j} = Σ_{j∈I_{n,E}} s_{n,j}` (odd set vs
  even set incl. `j = 0`), and
  `μ_n·s_{n,k} = Σ_{j=0}^{(n−1)/2−k} s_{n,j}` (base).
- **Boundary placement convention** (sec3 l. 138, verbatim):
  > "we place, for each small edge of length `s_{n,j}`, a tile
  > equivalent to `T_{n,j}` inside the inflated prototile so that its
  > **base coincides with the small edge**."
  This fixes the boundary tile types and their base-side placement;
  it does not fix their order along the side, nor anything interior.
- **The (17, 6) worked example** (sec3): the paper dissects
  `σ₁₇(T₁₇,₆)` explicitly in text — boundary-convention anchor. The
  spike re-verified its algebra exactly at n = 17 (Stage A W-block:
  legrep/baserep for n = 17, the `tab:labels` orientation rows as
  `θ_side ± jπ/17 (mod 34)` for all 12 rows, and — banked as a
  documented correction — the sec3 l. 136 four-term even-leg shorthand
  `s₀+s₂+s₄+s₈` is **false** for n = 17 too; the correct even
  decomposition `s₀+s₂+s₄+s₆+s₈ = μ₁₇` matches the 5-tile left-leg
  label list, all green), which pins the boundary-placement reading of
  l. 138 before any 21-fold search.
- **Provenance honesty about the rules themselves** (sec3 l. 2–4,
  verbatim — this is what makes the derived-interior route legitimate):
  > "substitution rules were not derived through a systematic,
  > step-by-step procedure; rather, they arose from brute-force
  > exploration guided by the KSK criterion and by techniques for
  > enforcing the n-fold rotational symmetry property."
- **KSK criterion** (sec3, citing Kannan–Soroker / Kenyon): a
  dissection exists when the **balance** condition and the
  **convex-crossing** condition hold. Sec3 remarks (l. 240–245):
  "all values of n and k satisfy the balance condition, whereas a few
  values of k do not satisfy the convex-crossing condition … even when
  a non-convex crossing occurs, it is still possible to associate
  chains of rhombs with the curves forming the non-convex crossing,
  although these chains are interrupted along a line (see, for
  example, `σ_{17}(T_{17,5})` and `σ_{21}(T_{21,3})`)."
  For n = 21 this names **k = 3** as an interrupted-chain case —
  consistent with this spike's k = 2 fill being a pure rhomb chain
  (§6). And the spike's k = 9 exhaustive negative (§6.1) shows the
  beyond-clean-chains structure is not confined to the paper's named
  example: within the spike's vocabulary, k = 9 admits no dissection
  at all, so the paper's `σ₂₁(T₂₁,₉)` interior must use structure this
  spike does not model.
- **Algebraic section** (sec4): the substitution matrix `M_n` (rows =
  boundary repetition vectors per `k`), `z^T M_n = μ_n z^T`, the
  characteristic polynomial factorization
  `p_{21}(x) = −(x²−1)(x³+2x²−x−1)·g_{21}(x)` with
  `g_{21}(x) = x⁶−8x⁵+8x⁴+6x³−6x²−x+1`, the inflation
  `μ_{21} = 1/s_{21,10} ≈ 6.69075` (`s_{21,10} = 2sin(π/42)`), and the
  coordinate-vector table `y_{n,j}` expressing each `s_{n,j}` in
  `Z[μ_n]`. **`tab:vectorrep` is length algebra only** — it contains
  no dissection geometry (verified line-by-line, sec4 l. 26–117), so
  it cannot substitute for the interior search (§5).
- **ILC section** (sec5): the `G_n`/w/u tables (r = 1..9 for n = 21,
  `U₂₁ = 1.8`), with the Danzer-algorithm hit at r = 9. The spike
  re-verified the whole chain exactly: `G₂₁` last column
  `(−1, 1, 6, −6, −8, 8)` with `y_j = coords(μ s_j)`, `w₁ = y₁ − y₈ − y₁₀`,
  the `w_r`/`u_r` recurrences (`u₄ = y₅`, `u₈ = y₄`), `μ₂₁,₂ = 1/s₈ ≈
  1.36858` with threshold `1/(|μ₂|−1) ≈ 2.7131`, `|w'|` table matches,
  Danzer ratio exceeding threshold **only at r = 9**, and the subset-max
  `U₂₁` check (`max |Σ c_j v₂ y_j| = 1.6786` over the 7-positive c-set
  `{0,1,2,7,8,9,10}`, covered by `U₂₁ := 1.8`) — Stage A V9–V12, all
  green.

### 1.3 The stop gate (pre-registered, observed)

The paper's dissection interiors, per-side tile orders, and rhomb
fills exist only as figures. Per the pre-registered gate: **this doc
does not claim canonical status for any interior data.** All interior
content (orders, rhomb multisets, `N` rows) is labeled **derived** and
carries its exact certificate. The honest label for the family is:

> **Say-awen 21-fold (boundary/text-exact; dissection derived &
> spike-certified).**

Two consequences, stated up front:

1. **Order non-uniqueness is expected and reported.** Multiple valid
   boundary orders exist per side (k = 2: fifteen certified
   dissections by exhaustion, several distinct order triples, all with
   the *same* `N` — §6). The
   doc pins the first solution under a fixed enumeration order and
   reports the rest.
2. **`N`-forcing is claimed only within the paper's own construction
   vocabulary** (boundary bases on the small edges per l. 138 +
   interior rhombs). At k = 2 the exhaustion **completes** (563k-node
   full tree), so `N[k=2]` is forced within that vocabulary; at k = 9
   the exhaustion **completes empty** (no dissection at all, §6.1) —
   the vocabulary's boundary is proven on both sides. Where a campaign
   does not complete (k = 4..8, §7.2 wall), the row is simply open.

## 2. Prototiles (exact)

### 2.1 Isosceles prototiles `T_{21,j}`, j = 0..10

Legs 1, apex angle `(21−2j)π/21`, base `s_j := s_{21,j} = 2cos(jπ/21)`;
`T_0` is the pseudo-prototile (a unit segment, `s_0 := 1`, apex angle
π, area 0; its "apex" coincides with a base endpoint and its edge
ledger contributes only the segment). Areas in `(w/4)`-units, where
`w = w₂₁ = 2sin(π/21) ≈ 0.29808453` and
`c₁ = cos(π/21)`:

- `area(T_j) = U_{2j−1}(c₁) = sin(2jπ/21)/sin(π/21)` for j ≥ 1;
  `area(T_0) = 0`.
- `U_m` = Chebyshev of the second kind (`U_m(cos θ) = sin((m+1)θ)/sin θ`).

Exact `s_j` in the power basis of `γ = 2cos(π/21)` (deg 6;
`x⁶`-free reduction, coordinates low→high), all spike-verified:

| j | `s_j` in `Z[γ]` | numeric |
|---|---|---|
| 1 | `γ` | 1.977662 |
| 2 | `−2 + γ²` | 1.911146 |
| 3 | `−3γ + γ³` | 1.801938 |
| 4 | `2 − 4γ² + γ⁴` | 1.652478 |
| 5 | `5γ − 5γ³ + γ⁵` | 1.466104 |
| 6 | `−3 − 8γ + γ² + 6γ³ − γ⁵` | 1.246980 |
| 7 | `1` | 1.000000 |
| 8 | `3 + 9γ − γ² − 6γ³ + γ⁵` | 0.730682 |
| 9 | `−2 − 5γ + γ² + 5γ³ − γ⁵` | 0.445042 |
| 10 | `−2 − 3γ + 4γ² + γ³ − γ⁴` | 0.149460 |

(`s_0 = s_7 = 1`: **two** tile types have unit base — a pending
length-1 edge can close as a `T_0` segment *or* as a `T_7` base; the
cascade disambiguates by the remaining multiset. Also note the exact
identities `s_3·s_6·s_9 = 1` and `μ = 1/s_{10}`.)

Tile areas `a_j = U_{2j−1}(c₁)` (in `(w/4)`-units; exact in `Z[γ]`
via the same basis; numeric):

| j | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
|---|---|---|---|---|---|---|---|---|---|---|
| `a_j` | 1.977662 | 3.779599 | 5.245703 | 6.245703 | 6.690745 | 6.541285 | 5.810603 | 4.563623 | 2.911146 | 1.000000 |

(`a_5 = μ₂₁` exactly: `sin(10π/21) = cos(π/42)`, so
`a_5 = cos(π/42)/sin(π/21) = 1/(2sin(π/42)) = 1/s_{10} = μ₂₁`;
`a_{10} = 1` exactly.)

### 2.2 Rhombs as glued tile pairs (bijection)

`rhomb_t` (t = 1..10): side-1 rhombus with acute angle `tπ/21`.
Exactly one decomposition into two prototile halves:

| t | halves | glued along | diagonal length |
|---|---|---|---|
| t even | `2 × T_{t/2}` | **long** diagonal | `s_{t/2} = 2cos(tπ/42)` |
| t odd | `2 × T_{(21−t)/2}` | **short** diagonal | `s_{(21−t)/2} = 2sin(tπ/42)` |

Area `area(rhomb_t) = 2·U_{t−1}(c₁) = 2sin(tπ/21)/sin(π/21)` in
`(w/4)`-units (numeric):

| t | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
|---|---|---|---|---|---|---|---|---|---|---|
| area | 2.000000 | 3.955323 | 5.822291 | 7.559199 | 9.127246 | 10.491406 | 11.621206 | 12.491406 | 13.082570 | 13.381490 |

The map j ↦ the unique rhomb type containing `T_j` as a half is a
bijection on {1..10}: j ≤ 5 → `rhomb_{2j}`; j ≥ 6 → `rhomb_{21−2j}`.

## 3. Parents and the text-fixed boundary multisets

### 3.1 Parents

The parent of `σ₂₁(T_{21,k})` is `μ₂₁·T_{21,k}`: legs `μ₂₁`, base
`μ₂₁·s_k`, apex angle `(21−2k)π/21`, base angles `kπ/21` at L and R.
Parent area `μ₂₁²·U_{2k−1}(c₁)` in `(w/4)`-units. Frame: L = (0,0),
R = `(μ₂₁·s_k, 0)`, P = L + `μ₂₁·dir((21−k)π/21)`; side direction
classes (units of `π/21`, mod 42): SIDE_M = [0, 21−k, k] for
(base L→R, right R→P, left L→P).

### 3.2 Boundary multisets (text-forced, order-free)

Per `eq:legrep`/`eq:baserep` with `s_{21,0} = 1`:

- **base side (L→R):** one each of `T_0 … T_{10−k}` (the baserep sum);
- **right leg (R→P):** one each of `T_1, T_3, …, T_9` (odd set);
- **left leg (L→P):** one each of `T_0, T_2, …, T_10` (even set).

Orders within a side are **derived** (§1.3). Boundary area deficit
(exact in `Z[γ]`, `(w/4)`-units; `d_k = d_{11−k}` — verified exactly;
numeric):

| k | deficit `d_k` in `Z[γ]` | numeric |
|---|---|---|
| 1, 10 | `0` | 0 |
| 2, 9 | `−2 −10γ −12γ² +6γ⁴ +2γ⁵` | 83.576814000530 |
| 3, 8 | `−2 −16γ −26γ² −2γ³ +12γ⁴ +4γ⁵` | 153.772138001404 |
| 4, 7 | `−4 −28γ −34γ² +4γ³ +16γ⁴ +4γ⁵` | 204.348809418041 |
| 5, 6 | `−4 −30γ −38γ² +6γ³ +18γ⁴ +4γ⁵` | 230.812869043005 |

The deficit is exactly the area the interior rhomb fill must supply
(`d_1 = d_{10} = 0` — the boundary already closes the parent for
k = 1 and k = 10; accordingly their certified dissections have **zero
rhombs**, §6).

## 4. Field kit K₂₁ (the engine contract, per M183 §5)

### 4.1 Constants

- `γ = 2cos(π/21)`, deg 6, `K₂₁ = Q(γ)`; `α = γ² − 2` with minimal
  polynomial `m₂₁(x) = x⁶ − 8x⁵ + 8x⁴ + 6x³ − 6x² − x + 1`
  (coefficient list `[1, −8, 8, 6, −6, −1, 1]`, reciprocal of
  `g₂₁`); `g₂₁(x) = x⁶ − 8x⁵ + 8x⁴ + 6x³ − 6x² − x + 1` is
  `minpoly(μ₂₁)`, irreducible, **non-Pisot** (constant term ±1 with a
  conjugate inside the unit circle — M183 §4's square-class
  certificate territory).
- `p₂₁(x) = −(x²−1)(x³+2x²−x−1)·g₂₁(x)`; its roots are exactly
  `{(−1)^j/s_j}` (spike-verified): `g₂₁` roots =
  `{1/s_2, 1/s_4, 1/s_8, 1/s_10, −1/s_1, −1/s_5}`; cubic roots =
  `{−1/s_3, 1/s_6, −1/s_9}`.
- `w² = 2 − α` (=`4 − γ²`) — the sine layer: `sin(mπ/21) = w·U_{m−1}(c₁)/2`.
- `μ₂₁ ≈ 6.690745`; `μ₂₁² ≈ 44.76595`.

### 4.2 Representation contract (verbatim transfer from M183 §5)

- **Positions:** `K₂₁` power-basis tuples in the oblique frame
  `(u, v ∈ Z[α]` over a common denominator`)`.
- **Edge directions:** `(a, b·w)` split pairs — x from cosines
  (`a ∈ K₂₁`), y from sines (`b ∈ K₂₁`, through `w`), never a flat
  degree-12 element.
- **Lengths / squared lengths:** split-pair cores `x² + s·y²` with
  `s = 2 − α`.
- **Sign/order:** the isolating-interval engine at `s₀ = 21`
  (`M₂₁` row 0 + `CAP = 4` bisects land at 25 ≤ 30 documented range —
  M183 §3), `m_increasing = true`, `BISECT_CAP = 4`.
- **Records:** `(u + v·w)/den` — the `n9::Nq` shape; the `N(s) = 3`
  assert in `n9.rs` is n = 9-specific and must NOT be transplanted
  (the K₂₁ genuine-extension certificate is the **p = 43 square-class
  witness**, M183 §4.2 — cited again by the battery in §8.3).

### 4.3 `M₂₁` — what it is and is not

`M₂₁` is the 11×11 **boundary-repetition** matrix: row k has 1 in
column j iff (j = 0 and k even) or (1 ≤ j ≤ 10 − k); `z^T M₂₁ =
μ₂₁ z^T` with `z = (s_0, …, s_{10})`. It is **not** the substitution
count matrix `N` (which counts rhomb fill too, §5.4). The spike
verified `M₂₁·s = μ₂₁·s` exactly, `charpoly(M₂₁)(μ₂₁) = 0` via
Bareiss + Lagrange interpolation, and primitivity via boolean powers.

## 5. The substitution: exact transition algebra

### 5.1 Boundary tile placement (derived, text-consistent)

A boundary tile `T_j` on side s with base interval `[t_0, t_0 + s_j]`
(side parameter in `K₂₁`): A = side point at `t_0`, B at `t_0 + s_j`,
apex `X = A + dir(ad)` with

- `ad = (SIDE_M[s] + j) mod 42` for s ∈ {0, 1},
- `ad = (SIDE_M[s] − j) mod 42` for s = 2 (left leg points inward).

`|X−A| = |X−B| = 1` is **asserted exactly** at placement. The CCW
walk classes of the placed tile: s ∈ {0,1}: `{SIDE_M[s],
SIDE_M[s]+21−j, SIDE_M[s]+21+j}`; s = 2: `{SIDE_M[2]−j, SIDE_M[2]+j,
SIDE_M[2]+21}` (mod 42). `T_0` contributes no walk (segment).

### 5.2 Rhomb-as-unit transition — the complete two-family algebra

A pending unit edge `e = [p, q]` (length 1) is resolved by a rhomb
unit of type t: the parallelogram `p, q, q+u, p+u` where
`u = dir(θ + σ·off)`, θ = the canonical direction class of `e`
(`edge_key` canonicalizes `p ≤ q`, so θ is fixed per **geometric**
edge), and `off/σ` range over **two families**:

- **Family 0 (acute rhomb corner at p):** `off = t`, σ ∈ {+1, −1}.
- **Family 1 (obtuse rhomb corner at p):** `off = 21 − t`,
  σ ∈ {+1, −1}.

giving 4 attachment geometries per type, 40 per edge — all with `e` as
a side (a leg of exactly one half). The glued diagonal is the shared
base `s_j` of the two halves; apexes sit at the rhombus vertices with
angle `(21−2j)π/21`. Exact assignment (spike-verified 200/200 across
directions; diagonal length `dist² = s_j²` checked exactly, apex
angles checked exactly):

| family | t parity | glued diagonal | apexes | half-type |
|---|---|---|---|---|
| 0 | t even | `[p, q+u]` (long) | `q`, `p+u` | `T_{t/2}` |
| 0 | t odd | `[q, p+u]` (short) | `p`, `q+u` | `T_{(21−t)/2}` |
| 1 | t even | `[q, p+u]` (long) | `p`, `q+u` | `T_{t/2}` |
| 1 | t odd | `[p, q+u]` (short) | `q`, `p+u` | `T_{(21−t)/2}` |

The three non-glued parallelogram sides become new pending edges; the
glued diagonal is interior forever. **History (implementation lesson,
banked for the engine battery):** the first spike enumerated family 0
only — `edge_key`'s endpoint canonicalization silently halves the
attachment space (acute-corner-at-p only), which is invisible at
k = 1/k = 10 (zero rhombs) and blocked k ≥ 2 convergence entirely
(100k+ nodes without a solution). With both families, k = 2 produces
certified solutions quickly (30k nodes under LIFO; **3k nodes** with
the corner-priority ordering, §7.2), and family-1 units occur
throughout the search (5,780 family-1 commits in the 30k-node probe).
The engine battery must include a **both-families attachment probe**
(§8.3, B7).

### 5.3 The count matrix `N` (derived, certified per dissection)

`N[k][j]` = number of type-j prototiles in `σ₂₁(T_{21,k})`:

- `N[k][j] = boundary_count(k, j) + 2·(#rhomb units whose half-type
  is j)` for j ≥ 1; `N[k][0] = boundary_count(k, 0)` (pseudo-tile,
  zero area).
- Equivalently: boundary multiset (§3.2) + 2× the rhomb multiset via
  the §2.2 bijection.
- **Area closure (exact):** `Σ_j N[k][j]·a_j = μ₂₁²·a_k` for every
  certified row (spike-verified in `K₂₁`; this is the row-level
  face of the substitution-matrix eigenvalue equation `N·a =
  μ₂₁²·a` with prototile area vector `a = (a_1..a_{10})`, checked
  fully as Stage C4 once all rows exist).

### 5.4 Deficit accounting

The rhomb fill sums exactly to `d_k` (§3.2): the committed rhomb area
running sum can never exceed `d_k` (running-sum prune), and the
residual after each commit must be an exact sub-sum of remaining
rhomb areas (achievable-set prune — bounded build; the middle-k wall
is quantified in §7.2). For k = 2 the
certified fill is `rhomb_3..rhomb_10` one each — and indeed
`Σ_{t=3}^{10} 2U_{t−1}(c₁) = d₂` exactly (numeric check: 83.576814).

## 6. Certified results per k (updated as campaigns land)

All rows: exact `K₂₁` arithmetic; dissection certified by the Stage B
in-leaf battery (§7.3) **and** the Stage C independent rebuild
(§7.4). `N` rows below are **derived data** under the §1.3 gate.

| k | tiles | rhombs | `N[k][0..10]` | status |
|---|---|---|---|---|
| 1 | 21 | — | `[2,2,2,2,2,2,2,2,2,2,1]` | certified; search exhausted (137 nodes, unique); Stage C ALL OK |
| 2 | 36 | `rhomb_3..rhomb_10` ×1 | `[2,2,4,4,4,4,4,4,4,3,1]` | certified **and exhausted**: 563,023-node full tree → exactly **15 unique dissections**, all with this `N` and the same rhomb multiset — `N[k=2]` is forced within the vocabulary; Stage C ALL OK on all 15 |
| 9 | — | — | — | **no dissection exists in the vocabulary — exhaustively proven** (158,107 nodes, full tree below the 400k cap, 0 solutions; §6.1) |
| 10 | 12 | — | `[2,1,1,1,1,1,1,1,1,1,1]` | certified; search exhausted (69 nodes, unique); Stage C ALL OK |
| 3 | — | — | — | cap-bounded: 400,001 nodes (cap hit), 321 dead ends, **0 solutions** — open (§6.2) |
| 8 | — | — | — | cap-bounded: 400,001 nodes (cap hit), 27 dead ends, **0 solutions** — open (§6.2) |
| 4..7 | — | — | — | open — achievable-set resource wall, quantified (§7.2) |

Derived boundary orders (first solution per k, fixed enumeration
order; k = 2 has five distinct order triples across its fifteen unique
dissections — reported, not hidden):

- k = 1: base L→R `[1,3,5,7,9,8,6,4,2,0]`; right R→P
  `[1,3,5,7,9]`; left L→P `[0,2,4,6,8,10]`.
- k = 2 (sol 0): base `[0,4,8,2,6,7,5,3,1]`; right `[1,3,5,7,9]`;
  left `[2,6,10,0,4,8]`.
- k = 10: base `[0]`; right `[9,7,5,3,1]`; left `[10,8,6,4,2,0]`.

### 6.1 The k = 9 exhaustive negative (and what it means for the vocabulary)

The k = 9 campaign **completed its full tree** (158,107 nodes < the
400k cap, no cap message, 931 recorded dead ends, 0 raw solutions):
within the construction vocabulary of this spike (boundary bases on
the parent sides per the paper's l. 138 convention + interior tiles
only as same-type glued rhomb pairs, §5.2), **σ₂₁(T₂₁,₉) has no
dissection**. Three readings, in increasing importance:

1. It is *not* a machine artifact: the same machine, same prunes,
   same candidate algebra certifies k = 1, 2, 10, and the k = 9 run's
   P-set (78,379 entries) is exactly the k = 2 set (equal deficits,
   `d₂ = d₉`) — the area economics that make k = 2 solvable are
   present at k = 9, and the tree still closes empty.
2. It gives the §1.3 vocabulary hedge a **proven non-empty boundary**:
   "N-forcing within the vocabulary" now has a certified instance of
   *no N at all*. Any engine implementation that hard-codes
   rhomb-pair interiors for all 11 rows would be wrong at k = 9.
3. It connects to the paper's own interruption remark (§1.2): the
   paper documents interrupted rhomb chains at `σ₂₁(T₂₁,₃)` — interior
   structure beyond clean rhomb chains. The k = 9 exhaustion says the
   phenomenon is not confined to the named case: whatever the paper's
   `σ₂₁(T₂₁,₉)` figure contains, it is not a boundary-bases +
   rhomb-pairs dissection. The spike cannot read the figure
   (text-only discipline), so the honest statement is precisely:
   *no rhomb-pair dissection exists at k = 9; the paper's k = 9
   interior must use additional structure (e.g. non-paired interior
   tiles along interruption lines) that this spike's machine does not
   model.*

Rows certified for the engine remain k = 1, 2, 10 (below); the other
rows are open in-spike (k = 4..7 at the §7.2 resource wall; k = 3
cap-bounded, §6.2; k = 4..7 at the §7.2 resource wall at this writing).

Cross-checks banked for these rows:

- `N·a = μ²·a` row-wise, exact (k = 1, 2, 10 all verified in `K₂₁`).
- k = 2 rhomb fill = deficit exactly (§5.4).
- Stage C vertex census (level-2 angle sums in `π/21` units: 42
  interior / 21 side / corners `k, k, 21−2k`), edge-direction census
  (k = 1: 20 classes, class 10 unused; k = 2: 20 classes / 19 classes
  across its solutions), max vertex-coordinate numerator/denominator
  magnitudes (k = 1: 7; k = 2: 23) — all clean (§10 log).

### 6.2 The k = 3 / k = 8 cap-bounded status

Both campaigns hit the 400,001-node cap with **0 solutions**:
k = 3 after 321 dead ends (46,632 s) and k = 8 after only 27
(61,543 s; `stageB_k3_corner.log`, `stageB_k8_corner.log`). Unlike
k = 9, exhaustion was **not** reached — the honest status is *open*,
not negative. The best rhomb-fill reached along the searched corridor
is ~115.2 (k = 3) / ~106.0 (k = 8) of the required 153.772 (maximum
heartbeat reading; the search never gets near closing the area).
Two observations shape the reading:

1. 321 (k = 3) / 27 (k = 8) dead ends across 400k nodes says the
   corner-priority + P-prune machine essentially never takes a wrong
   area/geometry branch; the tree is deep (19..32 placed tiles at the
   heartbeats) and the branching space at the deficit-153.77 pair
   (≈ 1.8× k = 2's) is simply enormous. Completion is out of reach at
   this node cost (≈ 117–154 ms/node → a 2M-node budget ≈ 2.7–3.6 days).
2. The cap-bounded emptiness is *consistent with the paper's own
   structure remark* (§1.2): the interrupted-chain case named in the
   text is exactly `σ₂₁(T₂₁,₃)`. The k = 9 exhaustive negative (§6.1)
   proves this spike's vocabulary cannot express at least one of the
   paper's interiors; if the k = 3 interior (and, by the exact deficit
   pairing `d₃ = d₈`, the k = 8 one) uses interruption structure, the
   rhomb-pair fill cannot close, and a 0-solution outcome —
   cap-bounded or exhaustive — is the *expected* result for this
   machine, not a search failure.

Engine rows therefore remain k = 1, 2, 10; k = 3 and k = 8 are reported
open with the cap-bounded evidence banked (§10).

## 7. Spike architecture (stdlib-only, all exact)

### 7.1 Layers

- **`sw21_lib.py`** — the exact kit: `K₂₁` field arithmetic
  (Fraction-coefficient vectors over `Z[γ]`'s basis, degree-6
  reduction), polynomial arithmetic, `build_n(21)`, decimal
  evaluation at precision 60 (`getcontext().prec = 60`).
- **`sw21_geom.py`** — pure exact geometry: direction classes mod 42,
  exact `place` (with leg-length asserts), edge ledger primitives,
  angle classes, exact orientation/sign tests (tiny-nonzero guard at
  1e−25 raises — no silent sign), memoized primitives with a 4M-entry
  clear guard.
- **`sw21_stageA.py`** — field battery: 64/64 green (minpolys,
  root structures, M₂₁ algebra, sine chains, basis tables).
- **`sw21_stageB.py`** — the cascade search (§7.2) + in-leaf battery
  (§7.3).
- **`sw21_stageC.py`** — independent rebuild + census battery (§7.4)
  + matrix checks (C4, all-rows gated).

### 7.2 Search machine (cascade DFS)

State: placed units (boundary tiles + rhomb halves), pending
length-1 edges (legs / `T_0` segments), per-side interval bookkeeping,
remaining boundary multisets (text-fixed), rhomb area running sum,
corner budgets. Pending selection: `corner` (default for the campaigns;
see below), LIFO / FIFO / MCV available. Candidates per pending edge:
boundary leg placement, boundary base
placement (incl. the `T_0`/`T_7` length-1 ambiguity, resolved by
multiset), and the 40 rhomb attachments (§5.2). Exact prunes:

1. deficit running-sum (rhomb area can never exceed `d_k`);
2. achievable-set membership (residual must be an exact sub-sum of
   rhomb areas; bounded build with truncation-disable — and the
   measured middle-k wall, below);
3. corner base-angle budgets (L, R ≤ k; P ≤ 21−2k), incremental +
   at-verify;
4. bbox prefilter before exact triangle-pair overlap;
5. candidate-time T-junction prune (sound: edges/vertices never
   disappear down a path; regression-verified tree-identical at
   k = 1/2/10 — §10);
6. node cap (`--cap`, default 400k) with heartbeat.

Candidate *selection* (pure reordering, sound — the candidate set is
never filtered by it): `--select lifo` (default), `fifo`, `mcv`
(fewest-candidates edge), and **`corner`** — scan the pending vertices
for the minimum hole sector (smallest gap `g ∈ 1..20` between two
pending unit edges at one vertex, direction classes in π/21 units),
then promote the corner rhomb filling that hole to the front of its
edge's candidate list (matched by exact 4-vertex set). Rationale
(KSK-shaped): a valid dissection must fill every hole with the rhomb
matching the hole angle, so forced corners are the correct first
commits. Measured at k = 2: the first four of the fifteen unique
solutions appear within a
**3,001-node** budget (90 dead ends; `--cap 3000`), vs 30k nodes /
3,510 dead ends under LIFO for the same four — ≈ 10× discovery speedup
(`stageB_k2_corner.log`). The ablation `--nopset` (no achievable-area
prune) collapses even combined with corner ordering (< 10k nodes in a
20-minute wall budget, vs 3k nodes with P): the P-prune and the corner
ordering are complementary and BOTH load-bearing — P cuts infeasible
area branches early; corner cuts the branching factor among feasible
ones (`stageB_k2_nopset.log`). Middle-k campaigns therefore run with
both, and the achievable-set build is the middle-k resource wall,
measured exactly (`measure_p.py`, banked): k = 3 needs 2,144,466 exact
entries (837 s, ≈ 2.9 GB); k = 4 **truncates a 4M-entry probe**
(2,397 s without reaching closure — `measure_p_k4.log`), i.e.
`|P₄| > 4·10⁶` and scaling past practical memory/time; by the deficit
pairing `d_k = d_{11−k}` the same sets serve (k, 11−k) pairs —
(2, 9) = 78,379 entries ✓ built, (3, 8) = 2,144,466 ✓ built,
(4, 7) and (5, 6) open at the wall. Rows k = 4..8 are therefore
**reported open with the wall quantified**, not guessed; a persistent
P-set cache or a smarter exact residual filter is the implementation
path (§11).

Orientation is **not** a search variable: for fixed (side, interval,
j) the triangle point set is unique (asserted); for fixed (edge,
family, type) the rhomb point set is unique. Dedup by canonical
frozenset of exact tile tuples; per-k uniqueness counts reported
honestly.

### 7.3 In-leaf verification battery (Stage B `verify()`)

V-a ledger audit (no ≥3-owner edges; lone segments must be boundary
bases); V-b side coverage (exact sums + disjointness); V-c parent
containment (closed); V-d pairwise interior-disjointness (exact SAT +
orientation separation); V-e **no T-junctions** (no vertex strictly
inside an edge); V-f glued pairs opposite-side; V-g area closure
(exact, `Σ area = parent area`); V-h corner budgets.

### 7.4 Stage C (independent battery)

C1/2 rebuild tiles from the serialized JSON **without any search
state** and re-run the full battery; C2b re-derive `N` from the
ledger and compare to the stored row; C3 level-2 vertex census; C5
edge-direction census; C6 coordinate magnitude audit; C4 (all-rows
gated): exact `charpoly(N)(μ₂₁²) = 0` via Bareiss + Lagrange, PF
eigenvalue check, primitivity — machinery unit-tested on known
matrices this session.

## 8. Implementation spec (M181 bar)

### 8.1 Engine data contract

Everything in M183 §5 transfers verbatim (§4.2): positions in
`K₂₁`/oblique frame, edge directions as `(a, b·w)` split pairs,
split-pair length cores with `s = 2 − α`, isolating-interval sign
engine at `s₀ = 21`, `Nq`-shape records with the p = 43
square-class certificate in place of the `n9` norm assert. The
substitution layer adds:

- **Prototile registry:** 11 entries `T_0..T_{10}` (T_0 pseudo —
  represent as a degenerate tile with the s₀ = 1 contract from M183
  §3, *not* as a separate length) with per-tile: apex angle index
  (21−2j in π/21 units), base length `s_j` (exact, §2.1 table),
  area `a_j` (exact), walk classes (§5.1).
- **Substitution table:** rows `N[k]` (§6) + the derived boundary
  orders (first-solution pin) + the rhomb fill per k as explicit
  unit pairs (glued-diagonal + apexes, §5.2 assignment table).
- **Inflation factor:** `μ₂₁` with `g₂₁`; the engine's inflation
  metadata carries the non-Pisot flag (this is what routes ILC
  behavior; it also forbids the norm-based certificates used for
  Pisot families — §8.3's assert covers the difference).

### 8.2 Per-prototile exact tables (engine inputs)

All values exact in `K₂₁` (`Z[γ]` basis, low→high), numeric in
parentheses:

- Base lengths `s_j`: §2.1 table (10 rows + `s_0 = 1`).
- Tile areas `a_j`: §2.1 table.
- Rhomb areas `2U_{t−1}(c₁)`: §2.2 table.
- Deficits `d_k`: §3.2 table.
- Rhomb half-mapping: §2.2 (t even → `T_{t/2}` long-diagonal glue;
  t odd → `T_{(21−t)/2}` short-diagonal glue).
- Attachment families: §5.2 (both families; the engine must carry
  family-1 attachments — B7 below fails otherwise).

### 8.3 Battery design (implementation gate, per M181 bar)

- **B1 field regression:** the Stage A 64-check battery re-run
  in-engine (minpolys, `M₂₁` eigen, root sets, sine chains).
- **B2 s₀ battery:** M183 §3's `s₀ = 21` battery verbatim
  (`m_increasing`, bisect depth, RED fold rows).
- **B3 boundary multiset check:** for each k, the side multisets
  match `eq:legrep`/`eq:baserep` exactly (sum identities in `K₂₁`).
- **B4 area closure:** `Σ_j N[k][j]·a_j = μ²·a_k` exact per row, and
  `charpoly(N)(μ²) = 0` for the full matrix (C4 machinery).
- **B5 dissection certificates:** the certified JSON per k re-verified
  in-engine (C1/2 battery, all of V-a..V-h).
- **B6 mixed-sign regressions:** vertex coordinates across all four
  K₂₁ conjugate arms — the spike's max |num|/|den| audit (C6) bounds
  the coordinate heights (k = 1: 7; k = 2: 23); the engine must
  reproduce the exact tuples under conjugate embedding, exercising
  mixed-sign numerator arms (the `Nq` mixed-arm template).
- **B7 both-families attachment probe:** assert that family-1
  attachments (obtuse-corner-at-p) are generated and geometrically
  valid (diagonal identity + apex angles, §5.2 table) — this is the
  regression that would have caught the half-coverage bug.
- **B8 tie-arm unreachability assert:** `s_0 = s_7 = 1` creates a
  length-1 tie (a pending unit edge can close as `T_0` segment or
  `T_7` base). The assert: at every certified leaf, the tie resolved
  per the remaining multiset, and no leaf contains a `T_7` base
  occupying a `T_0` slot or vice versa. The *certificate* for why the
  tie is safe to resolve greedily is the multiset accounting itself
  (text-forced boundary), **not** a norm argument; the family's
  field-certificate reference remains the **p = 43 square-class
  witness** (M183 §4.2), which certifies genuine-extension behavior
  and nothing about the tie.
- **B9 order-census:** derived orders per k reproduce the side
  interval covers exactly (no gaps/overlaps at unit scale /64).

### 8.4 Census deltas and registry-entry plan (derived from `lib.rs` structure)

Current census at the M183 tip: **1545 tests** (84 + 40 + 1353 + 68)
and, in `registry_groups_are_complete_and_canonically_ordered`,
**165 built-in tilings**, **186 tab memberships** (11 categories,
`TilingCategory::ALL`; per-tab card sizes
`[11, 20, 33, 17, 17, 5, 30, 8, 22, 12, 11]` — gates re-run green this
session, no movement: `cargo check --workspace --all-targets` 0
warnings, census test pass). Registration of the Say-awen 21-fold
tiling (one entry, single primary tab) moves:

- built-ins 165 → **166** (assert message "update when adding
  tilings");
- memberships 186 → **187** (one primary category card);
- README census sentence + per-tab prose via the existing
  `readme_matches_registry` guard; per-tiling doc via
  `tiling_docs_match_registry`.

Category placement must follow the paper's own claims (§1.2): non-Pisot
substitution, ILC, n-fold symmetric member — **no pure-point
diffraction claim and no prototile-set aperiodicity claim** in the
source. Under the 11-category taxonomy: `Quasiperiodic` ("pure-point
diffraction / cut-and-project") would over-claim; `Aperiodic` ("einstein
family … pinwheel") would import a forcing claim the paper does not
make; `Nonperiodic` ("nonperiodic by construction whose prototile is
not aperiodic") fits the source's claims most literally. **Decision
deferred to implementation time with this grounding recorded**; the
census arithmetic above is identical for any single-tab choice (a
secondary tab would make memberships 186 → 188 and must be called out
in the landing's census line). If the entry lands as a new family
feature (`families::sayawen`), the `family_cases()` doc-sync guard
list gains one row and the family doc gains the per-tiling table.

### 8.5 What the engine does NOT need (scope fence)

No new field layer (K₂₁ shipped), no new sign-engine depth class
(D = 6 verbatim), no Danzer-algorithm machinery (that is the paper's
ILC proof route, not an engine requirement), no 17-fold work (blocked
at the engine wall per M183 §2 — see §9).

## 9. D = 8 recommendation (single section, as pre-agreed)

Recommend the 21-first engine path: K₂₁'s substitution coordinates all
live in the shipped D = 6 depth class (`s₀ = 21` battery green, split
pairs `(a, b·w₂₁)`, `w₂₁² = 2 − α₂₁`), so the Say-awen 21-fold table
lands on the existing engine contract with no accumulator-width or
isolation-scheme work. K₁₇ (Say-awen 17-fold) stays blocked at the
measured D = 8 wall (M183 §2 NON-QUALIFICATION) until an engine
decision on accumulator width or an isolation scheme, and nothing in
this doc changes that verdict. Sequencing note: land 21-fold first to
bank the two-family attachment battery (B7) and the non-Pisot
inflation metadata path, both of which the 17-fold landing will reuse
verbatim once the wall falls.

## 10. Verification appendix (spike evidence index)

Scratch (persistent): `/home/pcharles/dev/pchuck/tesselation/.scratch/sayawen-21-substitution/`

| artifact | content | status |
|---|---|---|
| `stageA_run.log` | field battery 64/64 (Ψ₂₁, s_j tables, legrep/baserep, g₂₁/m₂₁, root sets, M₂₁ algebra, sine chains) | GREEN |
| `smoke_geom.log` | geometry kit smoke 16/16 | GREEN |
| `stageB_k1.log`, `stageB_solutions_k1.json` | k = 1 certificate (137 nodes, unique) | GREEN |
| `stageB_k10.log`, `stageB_solutions_k10.json` | k = 10 certificate (69 nodes, unique) | GREEN |
| `stageB_k2_famfix.log`, `stageB_solutions_k2.json` | k = 2 (30k cap; 4 unique, same `N`) | GREEN |
| `stageB_k2_tj.log` | k = 2 A/B with the T-junction prune (tree-identical: 30001 nodes, 3510 dead ends, same 4 solutions) | GREEN |
| `probe_fam_algebra.log` | 200/200 two-family attachment identities (A1 diagonal, A2 apex angles, A3 sides, A4 glue test, A5 no dups; θ ∈ {0, 7, 12, 19, 25}, t 1..10 × fam 0,1 × sgn ±1) | GREEN |
| `probe_k2_fam.py` (instrumented run, output in session log) | family census in search: 5,780 family-1 rhomb commits in a 30k-node k = 2 probe — family 1 is live in real solutions | GREEN |
| `test_tj.log`, `test_tj_k2.log` | `_tj_ok` soundness (no false positives on certified k = 1/k = 2 dissections; both defect directions caught; undo integrity) | GREEN |
| `stageC_k1_k2.log` | Stage C k = 1, 2 — independent rebuild + census | ALL OK |
| `stageB_k2_corner.log` | corner-priority A/B: all 4 unique within 3,001 nodes (90 dead ends; 30k-node run: 392 dead ends vs 3510 LIFO) | GREEN |
| `stageB_k2_nopset.log` | achievable-set ablation: 1 of 4 solutions, 11,933 dead ends (LIFO); with corner, < 10k nodes in a 20-min wall budget — P-prune is load-bearing | GREEN |
| `stageB_k2_exhaust.log`, `stageB_solutions_k2.json` | k = 2 **exhausted**: 563,023-node full tree, 30 raw → **15 unique**, one `N`, one rhomb multiset | GREEN |
| `stageB_k9_corner.log` | k = 9 full-tree exhaustion: **0 solutions** (158,107 nodes, 931 dead ends) — no rhomb-pair dissection exists at k = 9 | NEGATIVE (exhaustive) |
| `stageB_k3_corner.log` | k = 3 cap-bounded: 400,001 nodes (cap hit), 321 dead ends, 0 solutions (§6.2) | OPEN (cap-bounded) |
| `stageB_k8_corner.log` | k = 8 cap-bounded: 400,001 nodes (cap hit), 27 dead ends, 0 solutions (§6.2) | OPEN (cap-bounded) |
| `stageC_k10.log` | Stage C k = 10 — independent rebuild + census | ALL OK |
| `stageC_k2_15.log` | Stage C k = 2 re-run on all 15 exhaustive solutions (60 PASS lines) | ALL OK |
| `measure_p_k4.log` | k = 4 achievable-set probe: 4M-entry truncation at 2,397 s → `|P₄| > 4M`, P route infeasible at k = 4 | measured |

## 11. Deviations and open items

- **Deviation (documented):** the interior dissections and per-side
  orders are *derived*, not transcribed (§1.3 gate). The doc's title
  carries the resulting family label.
- **Deviation (implementation lesson, banked):** the two-family
  attachment requirement (§5.2) — the single-family enumeration is a
  silent half-coverage trap for any engine that canonicalizes edge
  endpoints.
- **Open:** rows k = 3 / k = 8 (cap-bounded at 400k nodes, 0 solutions
  each — §6.2; completion would need a ~2.7–3.6-day budget at the
  measured node cost, or a stronger prune) and k = 4..7 (blocked at
  the quantified achievable-set wall, §7.2); C4 full-matrix checks
  (gated on all rows — now permanently gated, given the k = 9
  vocabulary negative: the full 11-row `N` matrix does not exist
  in-spike; C4's machinery stays banked and unit-tested for whichever
  rows a future engine certifies); the paper's `σ₂₁(T₂₁,₃)`
  interrupted-chain structure as a validation hook for the k = 3 row
  (§1.2), and the k = 9 vocabulary negative (§6.1) as the anchor for
  the engine's interior-representation decision.
- **Implementation path for the wall (recorded, not started):** the
  achievable-set prune needs the full exact DP over sub-sums ≤ `d_k`;
  at k ≥ 4 that DP outgrows memory/time (§7.2 numbers). Candidates:
  a persistent on-disk P-cache keyed by deficit (the `d_k = d_{11−k}`
  pairing halves the builds), or a lazy memoized exact-matchability
  oracle restricted to residuals the corner-guided DFS actually
  queries (worst case = the full DP again; corridor hypothesis untested
  at k ≥ 4).
- **Text-only discipline held:** no figure/PNG content was read or
  used; all source data is TeX text (§1).