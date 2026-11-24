# Cyclotomic field layer for n-fold tiling families — design doc

Status: LANDED — design delivered and superseded by the implementation
(M137 step 1 = fields.rs + K₇; M139 step 2 = K₉/K₁₅/L₁₀ + ROTM anchor;
M149 first consumer). This doc's §7 sequencing is complete through
step 2; §8 items are deferred by design. Companion POC:
`spike.py` (stdlib-only, all checks green in 0.06 s; log
`spike_output.txt`; notes `NOTES.md`). Scope: the §23e/§23i families
gated behind "degree-3/4 arithmetic before any substitution table"
(GLB generalized binary; Danzer 7-fold, Harriss 9-fold,
Goodman-Strauss 7-fold, CAST 10-fold, Nischke–Danzer deltoids,
Say-awen 17/21-fold). **[P]** = proven by exact algebra below (the
spike's assertions are the executable proof); **[C]** = cited
(shipped code, dossier, or literature).

---

## 0. One-paragraph summary

Represent tiling points in the **oblique frame** `(U₀, U₁)` with
coordinates in `Kₙ = Q(b), b = 2cos(2π/n)` — degree `φ(n)/2`. Rotation
by `2π/n` is the *companion matrix* `R = [[0,−1],[1,b]]`, so the entire
2n-direction frame, all substitution walks, and all orientation signs
live in `Kₙ` with **no sine ever stored as a field element**. Minpolies
and reduction tables are *derived* (Newton power sums), never
hardcoded, and certified by exact Sturm counts. Exact sign/order uses
Sturm-isolated rational intervals (the generalization of `pc_rule.py`'s
quadratic `zcmp`; the quadratic shortcut itself does *not*
generalize). The spike reproduces all 20 shipped girih rotation
matrices **bit-for-bit** from the generic recipe, validating the scheme
against the one quartic layer already in production.

---

## 1. Field selection and generator convention **[P]**

- `Kₙ := Q(ζₙ)⁺ = Q(2cos(2π/n))`, `deg = φ(n)/2`. Standardize the
  generator as `b = 2cos(2π/n)` for **all** n (frame algebra needs the
  `2π`-angle element; see §4).
- For **odd** n: `Q(2cos(π/n)) = Kₙ` (same field: `ζ₂ₙ = −ζₙ^((n+1)/2)`
  when n is odd, so `Q(ζ₂ₙ) = Q(ζₙ)`). The brief's `2cos(π/n)` wording
  for odd n and this doc's `2cos(2π/n)` generate the identical field —
  pick `b` for uniformity.
- For `n ≡ 2 (mod 4)`: `Q(2cos(π/n)) = Q(ζ₂ₙ)⁺` is the strictly larger
  "half-step" field `Lₙ`, `deg Lₙ = φ(2n)/2 = φ(n)` — twice
  `deg Kₙ`. Concretely `L₁₀ = Q(2cos(π/10))` deg 4 ⊋ `K₁₀ = Q(φ)`
  deg 2: the shipped girih field, and the classical binary STOP in
  §23e names exactly this jump ("needs `Q(2cos π/10) ⊋ Q(√5)`")
  **[C: PROPOSED.md L1470–71]**.
- For `4 | n`: cos/sin rotation matrices are first-class `Kₙ`-objects
  (see sine rule below).

**Sine-location rule** (from `sin θ = cos(π/2 − θ)`):
`4 | n`: `π/2 = 2π(n/4)/n` is an exact `2π/n`-multiple, so
`sin(2πk/n) = cos(2π(n/4−k)/n) ∈ Kₙ`. **[P]**
`n` odd: `sin(2πk/n) = cos((n−4k)π/(2n))` with `n−4k` odd, so it lies
in `Q(ζ₄ₙ)⁺ = Q(2cos(π/2n))`, `deg φ(n) > deg Kₙ = φ(n)/2` (strict:
odd `π/(2n)`-multiples are never `2π/n`-multiples).
`n ≡ 2 mod 4`: `n−4k` is even and `(n−4k)/2 = n/2−2k` odd, so
`sin(2πk/n) = cos((n/2−2k)π/n) ∈ Q(ζ₂ₙ)⁺ = Lₙ ∖ Kₙ`, `deg φ(n)`.
girih (n = 10) is the shipped case: its first-class *half-step* pair
has `cos(π/10) ∈ L₁₀ ∖ K₁₀`, `sin(π/10) = cos(2π·2/10) ∈ K₁₀`
**[C: girih_qc.rs]**. The oblique frame (§4) is what lets the odd-n
runtime never materialize the sine.

Derived + Sturm-certified minpolies of `b` (spike; low→high):

| n | deg φ(n)/2 | minpoly of b = 2cos(2π/n) |
|---|---|---|
| 7 | 3 | x³ + x² − 2x − 1 |
| 9 | 3 | x³ − 3x + 1 |
| 15 | 4 | x⁴ − x³ − 4x² + 4x + 1 |
| 21 | 6 | x⁶ − x⁵ − 6x⁴ + 6x³ + 8x² − 8x + 1 |
| 17 | 8 | x⁸ + x⁷ − 7x⁶ − 6x⁵ + 15x⁴ + 10x³ − 10x² − 4x + 1 |
| 20 | 4 | x⁴ − 5x² + 5  (generator 2cos(π/10); == shipped kmul) **[C]** |

**Brief erratum to flag:** the brief lists K₁₆ as `Q(2cos(π/16))`,
which is degree **8**; the quartic 16-fold field is
`Q(2cos(π/8)) = Q(2cos(2π/16))`, minpoly `x⁴ − 4x² + 2`
(Eisenstein at 2), and `4|16` so sines are in-field there.

---

## 2. Specific 1 — element representation and reduction tables

Element of `Kₙ` = tuple of `d` rationals, `d = φ(n)/2`, in the power
basis `1, b, …, b^{d−1}`. Two storage tiers, mirroring shipped
precedents **[C: girih_qc.rs Ke = [i64;4] @ /64; fractalizing.rs /
stampfli.rs Z[√3] pairs; penrose.rs f64]**:

- **Tier E (exact core):** fixed-denominator integer tuples
  `Ke = [i64; d]` at a per-field denominator `D`. For the odd-n frames
  of §4, all U(k) coordinates are *integers in Z[b]* (spike-asserted),
  so a pure-integer `D = 1` core carries all frame/walk geometry;
  per-family tile data may raise `D` (girih: 64) for prototile
  vertices. Overflow: products of two D-scaled d-tuples have
  coefficients ≤ d·D²·H² for height H — i64 is comfortable for
  d ≤ 4, D ≤ 64 (girih precedent); d = 6/8 families should use i128
  or gcd-normalize per mul.
- **Tier R (derivation-time):** `Fraction`-like exact rationals
  (`num-rational`) for table derivation, certificates, and the sign
  engine; never on the tile-emission path.

### 2.1 Reduction tables (exact, per field)

Reduction = rewrite `b^j` for `j ∈ [d, 2d−2]` back into the basis,
built once from the minpoly (`b^d = −(m_{d−1}b^{d−1} + … + m₀)`, then
multiply by b and fold). Spike-derived tables:

```text
K₇ :  b³ = 1 + 2b − b²          b⁴ = −1 − b + 3b²
K₉ :  b³ = −1 + 3b              b⁴ = −b + 3b²
K₁₅:  b⁴ = −1 − 4b + 4b² + b³   b⁵ = −1 − 5b + 5b³
      b⁶ = −5 − 21b + 15b² + 5b³
K₂₁:  b⁶ = −1 + 8b − 8b² − 6b³ + 6b⁴ + b⁵          (d = 6)
K₁₇:  b⁸ = −1 + 4b + 10b² − 10b³ − 15b⁴ + 6b⁵ + 7b⁶ − b⁷   (d = 8)
L₁₀:  a⁴ = 5a² − 5   (== girih_qc kmul rule, bit-verified) **[C]**
```

`fmul` = convolution to degree `2d−2`, then fold with the table —
exactly the girih `gexact.py` pattern (`c₀ += −5·p₄ − 25·p₆; …`)
generalized **[C: salvage gexact.py fmul]**.

**Derivation discipline (anti-hardcoding rule):** minpolies are
*derived* from the Galois orbit `{2cos(2πk/n) : k ∈ (Z/n)* mod ±}` via
exact Newton power sums (`p_j = Σ_t C(j,t)·c_n(j−2t)/2`, Ramanujan
sums `c_n`), then **certified**: Sturm count of distinct real roots in
[−2.1, 2.1] equals d, and each orbit value is a simple root (one root
per 1e-6-wide rational bracket around its 1e-9 float rounding). The
spike executes both certificates for every field above, making the
tables self-verifying: a wrong coefficient mechanically breaks a Sturm
or bracket count.

---

## 3. Specific 2 — exact constants and their certificates

All constants below are proven by the cited exact identity (spike
asserts each at equality level, floats only cross-check ~1e−15).

| constant | exact form | certificate |
|---|---|---|
| cos(2π/n), `U₀·U₁` | `b/2` | definition / `cos θ = b/2` |
| `\|p\|²` for p = A·U₀+B·U₁ | `A² + B² + A·B·b` | expand bilinear form **[P]** |
| girih cos 18° | `a/2` | — |
| girih sin 18° | `(a²−3)/2` | `((a²−3)/2)² + (a/2)² = 1 ⇔ a⁴−5a²+5=0` **[P, spike]** |
| φ-subfield elt of K₁₅ | `b³ − 3b = 2cos(2π/5) = 1/φ` | `(b³−3b)² + (b³−3b) − 1 = 0` **[P, spike]** |
| K₇-subfield elt of K₂₁ | `b³ − 3b = 2cos(2π/7)` | `s³+s²−2s−1 = 0` at `s = b³−3b` **[P, spike]** |
| tower direction | `2cos(2πm/n) = b^m − …` Chebyshev | `2cos(mθ) = T_m(b)` (integer Chebyshev poly) **[P]** |

General subfield probe (used above, reusable): for `g | n`,
`2cos(2π/g)` lies in `Kₙ` and satisfies `K_g`'s minpoly — verify by
exact substitution. This is how a quartic family certifies its
quadratic subfield (K₁₅ ⊃ Q(√5), K₂₁ ⊃ K₇) without writing a radical.

Float cross-checks (spike): every reduction entry, frame direction,
walk area, and tower constant agrees with independent float math to
≤ 1e−12 (frame directions ≤ 6.7e−16); salvage anchors **[C:
gexact.py]** (DEC area 7.694208843275204, BOW 1.3143277802978344)
remain available for girih-family re-validations.

---

## 4. Specific 3 — the generalized U(k) rotation frame

### 4.1 Representation

Point `p = A·U₀ + B·U₁`, `A, B ∈ Kₙ`, with `U₀ = (1,0)`,
`U₁ = (cos 2π/n, sin 2π/n)` the *oblique* unit frame; `(A, B)` is the
entire runtime state — two d-tuples. Since `U₀·U₁ = b/2`, **all** dot
products, norms, and orientation tests stay inside `Kₙ`:

- `p·q = A A' + B B' + (A B' + A' B)·b/2` **[P]**
- `|p|² = A² + B² + A·B·b` **[P]**
- `p × q = (A B' − A' B)·sin(2π/n)`, and `sin(2π/n) > 0` for n ≥ 5 ⇒
  `sign(p × q) = sign(A B' − A' B)` — orientation reduces to the sign
  of a `Kₙ` element **[P]**. (This is the load-bearing trick that makes
  the missing sine harmless: crosses differ from `Kₙ`-numbers by a
  *global positive* factor.)

### 4.2 Rotation = companion matrix **[P]**

`U(k) := unit vector at angle 2πk/n`. The Chebyshev recurrence
`U(k+1) = b·U(k) − U(k−1)` holds in *any* linear coordinates (it is
`z^{k+1} + z^{k−1} = (z + z^{−1}) z^k` with `z + z^{−1} = b`), and in
oblique coordinates `U(0) = (1,0)`, `U(1) = (0,1)`. Hence:

```text
U(k) ∈ Kₙ² for all k        (induction: b·U(k) − U(k−1) stays in Kₙ²)
U(k+n) = U(k)               (spike: exact, via ΣU(k) = 0 and recurrence)
R := [[0, −1], [1, b]]      (companion; R(A,B) = (−B, A + bB))
R(U(k)) = U(k+1) for all k  (R linear; R(U₀)=U₁; R(U₁)=b·U₁−U₀=U(2))
⇒ R^k = rotation by 2πk/n, an EXACT Kₙ^{2×2} matrix for every k
```

No sine, no floating point; `det R = 1`, `trace R = b` (exact Rust
sanity invariants; `trace R^k = 2cos(2πk/n)` basis-independently, and
the spike certifies `sin²(2πk/n) = (1 − cos(4πk/n))/2 ∈ Kₙ` exactly —
the backbone of the cross-sign trick above). The spike verifies
`R^k U(0) = U(k)`, `R^k U(1) = U(k+1)`, `|U(k)|² = 1`, `Σ_k U(k) = 0`
for every k in every field.

Frame coverage **[P]**: for odd n the 2n-direction set `{±U(k)}` equals
`{U(m) : m ∈ Z/2n}` (negation is coordinate-wise, hence free and
in-module; angles of `−U(k)` are the odd multiples of `π/n`); for even
n it is the n-direction set with multiplicity 2. So the module
`Mₙ = KₙU₀ ⊕ KₙU₁` supports full 2n-fold star geometry for odd n with
no extra machinery.

### 4.3 What this buys at substitution time

A substitution edge-walk is a list of `±U(k)` steps; vertices are
running sums in `Kₙ²`. The inflation `λ` enters tuples only if
`λ ∈ Kₙ` (see §6 caveat); the girih precedent shows per-level
child-unit scaling avoids `λ` entirely **[C: girih_qc.rs]**.
Adjacency (`p × q` sign), collinearity (`cross == 0`, an exact zero
test), and rotation symmetry (`R^s`-equivariance of centered vertices)
are all exact `Kₙ`-tests. Spike demonstration: unit-edge star walks
`{n/1}`, `{n/(n−1)/2}` close exactly, their exact centroids
`G = (Σ P_j)/n ∈ Kₙ²` satisfy `R(G) = G − U(0)` (proved in-spike), and
the centered walks are `R^s`-equivariant with `R^s E_j = E_{j+1}`,
`E_j = U(s·j)`.

### 4.4 Module-completeness caveat (honest boundary) **[P]**

`Mₙ` contains exactly the points reachable by edge-walks with steps in
`±U(k)` — all **edge-to-edge rhomb/hexagon/star constructions on the
2n-direction grid** — and not e.g. the circumradius `1/(2 sin(π/7))`
of a unit-edge regular heptagon (sine field, deg φ(n) for odd n).
Consequence: the real §23e/§23i targets — Danzer/Harriss/
Goodman-Strauss rhombs, GLB rhombs, Nischke deltoids — are all in `Mₙ`
**[C: Bielefeld entries; all rhomb-based]**, but a family whose
prototiles are unit-edge *regular n-gons* (not `Kₙ`-scaled) or that
rotates by `π/2n` first-class escalates to the degree-φ(n) sine field
`Q(2cos(π/2n))` — the girih situation (n = 10 ≡ 2 mod 4, L₁₀ for 18°
ops) **[C]**. Design rule: *try Mₙ first; escalate to Lₙ only when a
half-step rotation or sine-circumradius is provably required.*

---

## 5. Specific 4 — exact comparison / ordering (zcmp generalization)

The shipped quadratic precedent **[C: salvage pc_rule.py zcmp]**
decides `sign(p + q√3)` by sign-switching on `p² − 3q²`, `p`, `q` — a
conjugation trick that **does not generalize**: with d ≥ 3 conjugates,
`sign(N(x)) ≠⇒ sign(x)` (K₇ has elements with mixed conjugate signs).
The general engine:

1. **Zero test is exact and free:** `x = 0` iff the tuple is all-zero
   (`deg x < d < deg m`, `m` irreducible over Q ⇒ `x(b) = 0 ⇔ x ≡ 0`).
   Never do numerics for equality.
2. **Sign of nonzero x:** exact interval-Horner evaluation over the
   Sturm *isolating interval* of `b` (rightmost real root; b is always
   the rightmost conjugate since `k = 1` minimizes the orbit angle
   **[P]**), bisecting when undecided. Termination: `x(b) ≠ 0` and
   width → 0 forces a strict sign (continuity — no separation constant
   needed for correctness).
3. **Norm fast path / certificates:** `N(x) = Res(m, x)` is a nonzero
   integer when x ≠ 0 (exact Sylvester determinant; spike asserts
   `N(1) = 1`, `N(b) = (−1)^d·m₀`, multiplicativity). It gives
   `|x(b)| ≥ |N(x)| / B^{d−1}`, `B := Σ|xᵢ|·2^i` (conjugates in [−2,2])
   — a provably finite bisection budget; optional in Rust v1.
4. **Ordering** = `sign(x − y)`; total order via comparator.
   Spike battery: 29 mixed-height elements per field — exact signs
   match float signs, exact total order matches float sort
   (distinct floats asserted) for K₇/K₉/K₁₅. Rust: `ksign() -> Ordering`
   on the element type; the isolating interval is a per-field
   `const (Rational, Rational)` pasted from the Python battery.

---

## 6. Per-family applicability table (Specific 5)

Vertices of every family below are `Kₙ`-expressible in the oblique
module unless noted. `λ` = inflation factor — a per-family **derivation
step, not a doc promise**: if `λ ∉ Kₙ`, scale per level in child units
(girih pattern) or accept a degree-×2 λ-tower.

| family (dossier ref) | field | deg | tower / notes |
|---|---|---|---|
| GLB generalized binary, n = 7 (§23e) | K₇ | 3 | λ = √(2+2cos(π/7)) = 2cos(π/14) ∉ K₇ ⇒ child-unit scaling (girih pattern) |
| GLB generalized binary, n = 9 (§23e) | K₉ | 3 | same λ pattern (2cos(π/18)) |
| GLB generalized binary, n = 15 (§23e) | K₁₅ | 4 | contains Q(√5) **[P]**; λ = 2cos(π/30) ∉ K₁₅ |
| Danzer 7-fold (+variants) (§23i) | K₇ | 3 | rhomb-based ⇒ in M₇ |
| Goodman-Strauss 7-fold rhomb (§23i) | K₇ | 3 | in M₇ |
| Nischke–Danzer deltoids (§23i) | K₇ | 3 | 14-fold variants reuse K₇ (14 = 2·7: K₁₄ = K₇) **[P]** |
| Harriss 9-fold rhomb (§23i) | K₉ | 3 | in M₉ |
| CAST 10-fold (§23i) | **L₁₀** = Q(2cos(π/10)) | 4 | **NOT Q(√5)** — see verdict below |
| Say-awen 17-fold (§23i) | K₁₇ | 8 | heaviest; consider only after K₇/K₉/K₁₅ land |
| Say-awen 21-fold (§23i) | K₂₁ | 6 | quadratic tower over K₇ **[P]** |
| classical binary 5-fold (§9 STOP) | L₁₀ | 4 | dossier's own wording **[C]** |
| (future) 16-fold | K₁₆ = Q(2cos(π/8)) | 4 | brief erratum: not Q(2cos(π/16)) (deg 8) |
| (shipped) girih | L₁₀ | 4 | bit-for-bit cross-check target **[C]** |

**CAST verdict (deliverable):** the hypothesis "CAST 10-fold needs
only Q(√5)" is **refuted** on two independent grounds.
(1) Vertex support: Pautze's CAST construction places vertices on the
real subfield of the 2n-th cyclotomic field **[C: Pautze, Symmetry
9(2):19, 2017; Bielefeld entry "CAST 10-fold", ref. Frettlöh–Say-awen–de
las Peñas, Indag. Math. 28(1) 2017, arXiv:1602.00518]** — for n = 10
that is `Q(ζ₂₀)⁺ = L₁₀` (deg 4, the girih field), a proper extension
of `Q(√5)`; the dossier's binary-STOP wording already treats this
field jump as the operative cost **[C: PROPOSED.md L1470–71]**.
(2) Dense tile orientations: DTO **[C: FSDLP 2017 — existence for
n ∈ {2,3,4,5,6,8}; Bielefeld CAST DTO extends to 10-fold via Pautze]**
introduces rotation angles generating a dense subgroup of SO(2) — not
containable in *any* number field. Exact arithmetic therefore applies
per orientation class (positions within a class, plus the torsion
skeleton), with DTO angles as float parameters or rational
Pythagorean-parameterized rotations — a CAST-specific-brief decision.

Field-nesting summary (spike-proven where marked): `Q(√5) ⊊ K₁₅`
(2 ⊂ 4) **[P]**; `K₇ ⊊ K₂₁` (3 ⊂ 6) **[P]**; `K₁₀ = Q(√5) ⊊ L₁₀`
(2 ⊂ 4) **[C: shipped kmul + §23e STOP]**; `K₇ = K₁₄`, `K₉ = K₁₈`,
`K₁₅ = K₃₀` (odd-n doubling is field-stable) **[P]**.

---

## 7. Specific 6 — implementation plan (Rust)

**Location:** new shared module `crates/tess-tilings/src/fields.rs`
(single file first; split to `fields/` when the third field lands);
no feature or family file depends on it until a family opts in.

**Concrete types, no trait** (precedent: girih_qc's local `Ke`,
fractalizing/stampfli Z[√3] pairs, pc_rule zcmp — every shipped exact
layer is a concrete per-module tuple type; the `Tiling` contract is
f64 at the boundary, so exact layers live *inside* generators and
never appear in `Tiling` signatures):

```rust
// fields.rs — one block per field, shared free functions
pub struct K7(pub [i64; 3]);   // value = (c0 + c1·b + c2·b²) / 1  (D=1 core)
pub struct K9(pub [i64; 3]);
pub struct K15(pub [i64; 4]);
// helpers: k3_mul/k4_mul (minpoly coeff arrays as consts),
// ksign::<d>(poly, iso), sylvester… (test-only)
// point layer per family (inside the family module, girih-style):
//   P7(K7, K7) with dot/cross/norm2/rot — "girih generalized".
```

Migration rule: **none** for shipped code — `girih_qc` stays as-is
(it is the bit-for-bit validation target); new families opt in.
Generator-agnostic parts (companion rotation, walk closure, sign
engine) are free functions over `(d, minpoly, iso)` so a new field is
~40 lines of consts + tests.

**Test pattern per field** (mirrors the spike battery, all exact):
`m(b) == 0` on the tuple model (Sturm certification stays Python-side)
+ float cross-check at 1e−12; `|U(k)|² == 1`, `ΣU(k) == 0`,
`R^k U(0) == U(k)` for all k; star-walk closure + centered
equivariance; sign/order battery vs float; and, for the first field
landed, the girih ROTM reproduction test (20 matrices × /64 against
pasted integers) as the permanent regression anchor.

**Sequencing:** (1) `fields.rs` + K₇ + tests; (2) K₉, K₁₅ (same shape);
(3) first consumer family (GLB binary n = 7 — the §23e re-scope
candidate — is the cheapest XL); (4) K₂₁/K₁₇ only on demand.
**Performance envelope:** exact layers serve *table derivation and
generator-time* geometry; per-frame GUI regeneration still emits f64
`Vec2`s (girih precedent: exact tuples → f64 once at emission); the
250k-tile budget is unaffected.

---

## 8. Open questions

1. **CAST DTO orientation policy** (§6): float angles vs Pythagorean
   rational rotations per orientation class — needs the Pautze PDF's
   substitution table read in full (the Bielefeld vector entry is
   suggestive, not conclusive).
2. **λ policy per family:** the generalized-GLB inflation
   `λ = √(2+2cos(π/n)) = 2cos(π/2n)` for odd n **[P: `1 + cos θ =
   2cos²(θ/2)`]** lies in `Q(ζ₄ₙ)⁺` (deg `φ(n)` = 2·deg Kₙ), so
   `λ ∉ Kₙ`; child-unit coordinates are mandatory — confirm the girih
   pattern suffices per family.
3. **i64 vs i128 for d ≥ 6** (K₂₁/K₁₇ coefficient growth under
   substitution depth) and **Say-awen 17-fold practicality** (deg 8) — defer until a
   consumer family exists.
4. **Denominator policy:** girih needed D = 64 for half-step cos/sin;
   odd-n frames are D = 1 (spike-verified integrality); whether any
   target family needs D > 1 in the frame is per-family work.

---

## 9. Verification appendix (spike evidence index)

`spike.py` (stdlib-only; 0.06 s): minpoly derivation via exact Newton
power sums over the Galois orbit (Ramanujan sums); certification by
Sturm counts (total == d; one root per bracketed orbit value); frame
battery `|U(k)|² == 1`, `ΣU(k) == 0`, `R^k` consistency, U(k) ∈ Z[b]²
integrality, `sin²(2πk/n) ∈ Kₙ` — exact asserts, all k, fields 7/9/15;
six star walks (closure, centered equivariance, exact shoelace factor
in K, float area agreement); 29-element sign/order battery per field;
Sylvester norm identities (`N(1)=1`, `N(b)=(−1)^d m₀`, multiplicativity);
towers K₁₅ ⊃ Q(√5), K₂₁ ⊃ K₇ by exact minpoly substitution; girih
cross-check with `cos²+sin² == 1` exact and **all 20 shipped ROTM
matrices reproduced exactly at /64**; float cross-checks ≤ 1e−12
(frame dirs ≤ 6.7e−16).

Porting notes from spike debugging: Sturm chain keeps `p'` unnegated;
Sylvester rows leading-coeff-leftmost; float `b` may sit 1 ulp outside
a 2⁻⁷² isolating interval (compare midpoints, not membership);
star-walk equivariance is on centered vertices.