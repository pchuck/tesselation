# 24c odd-n engine-side design — (n,c) monster rungs at n = 11, 13 over the designed field layers

Docs-only design record for the next 24c rungs: per-record (n,c) monster
placements at **n = 11** (degree 5) and **n = 13** (degree 6), expressed over
the field layers designed in
[docs/cyclotomic-field-degree5.md](cyclotomic-field-degree5.md) (M178, K₁₁)
and [docs/cyclotomic-field-odd-n.md](cyclotomic-field-odd-n.md) (M179, K₁₃).
Catalog mechanics are the shipped ones (M169 parallel pairs, M171 the n = 9
rotated family); nothing here edits code — every claim below is settled by an
exact-integer stdlib-Python spike
(`.scratch/24c-oddn-engine-design/spike_24c_oddn.py`, green log banked next to
it: 226 checks, 0 failures, ~110 s). Section numbers of the field docs are
quoted as M178 §x / M179 §x.

## 1. The record ladder (derivation, then machine-confirmed)

The odd-n records live in the **rotated π/n family**: `B = rot(π/n)·A + P`,
the half-turn family (M171: `−U₀ = R·U₀` as vertex sets at odd n, so the
half-turn `κ(X) = O_A + O_B − X` swaps the two polygons for *any* `P`). With
`h = (n−1)/2` and the crossing pair pinned at both polygons' edge midpoints,

```text
record (n,c)  <=>  pin P = M_Δ − R·M_0,  Δ = h − c,  c = 2..h−1
crossings     =  (A_Δ, B_0) and κ-image (A_{h+1}, B_{Δ+h})
eAB           =  [Δ+1 .. h]          (A-edges inside B)
eBA           =  eAB + h             (B-edges inside A)
d             =  2Δ − 1              (canonical fold min(d, n−d))
cos²(contact) =  (2 + C_d(α))/4,  C_k = 2cos(2πk/n) the Chebyshev tower on α
```

Both crossings always carry the *same* angle: the κ-image crossing has offset
`−(2Δ−1) mod n`, and `cos²` is fold-invariant. This is why the rotated family
works at odd n where the parallel family cannot (T1 below). All seven rungs
implied by the Kaplan band `2 ≤ c ≤ ⌈n/2⌉−2` (Kaplan, p. 123: "for every
n>6 and 2 ≤ c ≤ ⌈n/2⌉−2") were enumerated and are **clean** — including the
two Δ = 1 rungs the working brief excluded (see §4 and §8, deviation D1).

## 2. Per-record design table (exact, spike-bankable)

Notation: `a = α = 2cos(2π/n)`; K-tuples are low→high coefficient lists over
`(1, a, …, a^{D−1})`, written `u/den`. The pin ships as two N-layer slots
`(u_x, 0)/4 + (0, v_y)/4` — Cartesian `(x, y·w)` with `w² = 2−α` — exactly the
shipped n9 slot discipline. All seven pins have **den = 4**.

| record | Δ | crossings (A_i,B_j) | eAB | eBA | d (canon) | pin x = u_x/4 | pin y·w = v_y/4 | \|P\|² = u/4 |
|--------|---|---------------------|-----|-----|-----------|---------------|-----------------|--------------|
| (11,2) | 3 | (3,0), (6,8)        | [4,5]       | [9,10]       | 5 | (−1,1,5,0,−1)       | (−2,−7,−3,4,2)    | (2,9,10,−1,−2)     |
| (11,3) | 2 | (2,0), (6,7)        | [3,4,5]     | [8,9,10]     | 3 | (1,4,4,−1,−1)       | (0,−6,−5,3,2)     | (4,14,10,−4,−3)    |
| (11,4) | 1 | (1,0), (6,6)        | [2,3,4,5]   | [7,8,9,10]   | 1 | (3,3,3,−1,−1)       | (0,−2,−3,1,1)     | (3,8,9,−3,−3)      |
| (13,2) | 4 | (4,0), (7,10)       | [5,6]       | [11,12]      | 7→6 | (1,−7,1,6,0,−1)   | (0,−1,−9,−4,4,2)  | (2,−9,6,13,0,−2)   |
| (13,3) | 3 | (3,0), (7,9)        | [4,5,6]     | [10,11,12]   | 5 | (−1,−4,5,5,−1,−1)   | (2,0,−11,−5,4,2)  | (0,2,14,8,−3,−2)   |
| (13,4) | 2 | (2,0), (7,8)        | [3,4,5,6]   | [9,10,11,12] | 3 | (1,−1,4,4,−1,−1)    | (2,4,−9,−7,3,2)   | (2,−1,14,11,−4,−3) |
| (13,5) | 1 | (1,0), (7,7)        | [2,3,4,5,6] | [8,9,10,11,12] | 1 | (3,−2,3,4,−1,−1)  | (0,3,−3,−4,1,1)   | (3,−7,9,12,−3,−3)  |

Both crossings sit at `t = s = 1/2` **exactly** at every record pin
(`2·t = den` battery form, as shipped).

### cos² (contact_cos2 = None for all seven; exact tuple pin)

All are pure-K (odd prime n ⇒ ζ_{2n} ∈ Q(ζ_n) ⇒ cos(π/n) ∈ Kₙ), full degree
`h` — certified in the spike by the Galois-conjugate orbit count
(`(2 + C_d(C_k(α)))/4`, k = 1..h distinct; 5 at n = 11, 6 at n = 13), the
dec10/L10/n9-cos293 precedent for `None` + exact tuple pin:

| record | d | cos² tuple (u/4) | degree |
|--------|---|------------------|--------|
| (11,2) | 5 | (1,2,3,−1,−1)/4       | 5 (full) |
| (11,3) | 3 | (2,−3,0,1,0)/4        | 5 (full) |
| (11,4) | 1 | (2,1,0,0,0)/4         | 5 (full) |
| (13,2) | 6 | (1,−3,3,4,−1,−1)/4    | 6 (full) |
| (13,3) | 5 | (2,5,0,−5,0,1)/4      | 6 (full) |
| (13,4) | 3 | (2,−3,0,1,0,0)/4      | 6 (full) |
| (13,5) | 1 | (2,1,0,0,0,0)/4       | 6 (full) |

## 3. Windows (σ-ray event census) and the window-pin verdict

Parameter σ along the ray `σ·P` (pin at σ = 1), samples `1 ∓ 1/64` — the
shipped /64 convention, unchanged. Events are the exact vertex-on-edge-line
census (A-vertices on B-edge-lines and B-vertices on A-edge-lines; every event
quotient is an exact Kₙ element; the maximal window around σ = 1 is found by
walking outward while the state key is constant, and every event bounding the
window is re-verified real by the walk). Event counts (the σ-ray census at
each record): 123, 121, 123 (n = 11); 175, 175, 173, 173 (n = 13).

| record | σ_lo | σ_hi | pin == midpoint? |
|--------|------|------|------------------|
| (11,2) | `4+2a−8a²+2a⁴` | `−2−2a+8a²−2a⁴` | **midpoint** (sum = 2) |
| (11,3) | `(12+16a+14a²−10a³−2a⁴)/23` | `(34−16a−14a²+10a³+2a⁴)/23` | **midpoint** |
| (11,4) | `(22−8a+86a²−2a³−26a⁴)/43` | `(34−16a−14a²+10a³+2a⁴)/23` | off-midpoint |
| (13,2) | `(272+304a−72a²−426a³+22a⁴+96a⁵)/131` | `(−10−304a+72a²+426a³−22a⁴−96a⁵)/131` | **midpoint** |
| (13,3) | `(1442−516a−218a²+426a³+14a⁴−96a⁵)/911` | `(380+516a+218a²−426a³−14a⁴+96a⁵)/911` | **midpoint** |
| (13,4) | `(52−110a−22a²+160a³+2a⁴−38a⁵)/53` | `(54+110a+22a²−160a³−2a⁴+38a⁵)/53` | **midpoint** |
| (13,5) | `(78−98a+62a²+96a³−26a⁴−18a⁵)/79` | `(54+110a+22a²−160a³−2a⁴+38a⁵)/53` | off-midpoint |

The verdict splits cleanly along Δ: **Δ ≥ 2 rungs pin at the exact window
midpoint (`σ_lo + σ_hi = 2`); the two Δ = 1 rungs do not** — matching the
shipped n9 pattern ((9,2) Δ=2 midpoint, (9,3) Δ=1 not). The Δ = 1 rung shares
its upper window end with the Δ = 2 rung of the same n ((11,4) and (13,5)
reuse the (11,2)/(13,4) σ_hi): adjacent c-windows meet at that relabeling
event.

### t(σ), s(σ) linearity

Per crossing the parameters are exactly linear on the window:
`t(σ) = t₀ + k·σ`, `s(σ) = s₀ + ks·σ`, all coefficients Kₙ elements, and
`t₀+k = s₀+ks = 1/2` exactly at the pin. The n9 `t₀ ∈ {0,1}` pattern is
**not** n-specific but Δ-specific: for Δ ∈ {1,2} the intersection at σ = 0 is
the vertex V₂ (which lies on the π/n ray for *every* n — `V₂ = u₀+u₁` has
argument π/n), so `t₀ = 0` (Δ=2) or `t₀ = 1` (Δ=1), with `k = ∓1/2`. For
Δ ≥ 3 the rungs get genuine Kₙ t₀:

| record | t₀ | k |
|--------|----|---|
| (11,2) | (0,−3,−3,1,1)/1 | (1,6,6,−2,−2)/2 |
| (11,3) | 0 | 1/2 |
| (11,4) | 1 | −1/2 |
| (13,2) | (0,0,6,3,−2,−1)/1 | (1,0,−12,−6,4,2)/2 |
| (13,3) | (2,0,−4,0,1,0)/1 | (−3,0,8,0,−2,0)/2 |
| (13,4) | 0 | 1/2 |
| (13,5) | 1 | −1/2 |

(`k ≠ 0` at every record — the pin is the unique `t = 1/2` point of its
window; exact samples `t(1∓1/64) ≠ 1/2` battery-asserted. `k > 0` for
(11,2),(11,3),(13,2),(13,3),(13,4); `k < 0` for the two Δ=1 rungs.)

## 4. Census structure at n = 9, 11, 13 (full n² rotated enumeration)

The spike enumerates **all n² pin labels** `(i,j) ↦ P = M_i − R·M_j` (the
rotated family is not rotation-covariant as a pin map, so no (i−j)-reduction
is sound). Findings, all battery-asserted:

1. **The rotated pin map is 2-to-1, not injective**: `P(i,j) =
   P(j+h+1, i+h)` for *all* n² labels — the two labelings of one configuration
   (the κ A/B-swap relabeling). Distinct pins: `(n²+n)/2` (45, 66, 91). The
   fixed locus `i−j ≡ h+1 (mod n)` is exactly the **collinear edge-pair
   degeneracy**: `A_i ∥ B_j` with the shared midpoint forcing an 8-contact
   collinear overlap, no transversal crossing, c = 0. (This *corrects* the
   working brief, which assumed injectivity; the injectivity proof applies to
   the parallel pin map only — see §5.)
2. **Clean families**: for every `c ∈ {0, …, h−1}` the clean pins (exactly 2
   transversal crossings, both at `t = s = 1/2`, zero vertex-on-edge contacts,
   `c_A = c_B = c`) form a family of `2n` labels = n distinct configurations,
   with label offsets `j−i ∈ {h−c−1, c−h}`; within a family the state classes
   pair up as `{pin, partner(pin)}` (n classes of 2).
3. **Band-qualifying families = records**: the Kaplan band `2 ≤ c ≤
   ⌈n/2⌉−2 = h−1` selects *exactly* the families with `c ≥ 2` — at n = 9:
   {2,3} (18 labels each, one dihedral orbit — census5 re-confirmed: the
   O_A-rotation by `2πs/9` maps `config(2,0)` onto `config(2+s, s)` as vertex
   sets); at n = 11: {2,3,4} (22 labels each); at n = 13: {2,3,4,5} (26
   labels each). **Every band rung is cleanly realizable.**
4. The c < 2 families are excluded by the band (c = 0/1 pins are not
   monsters); the collinear labels are degenerate.

**Δ = 1 settlement (brief question):** (11,4) and (13,5) are clean record
rungs — 2 midpoint crossings, no dirty contacts, full windows (§3), correct
outline counts (§6). The brief's 5-record list is superseded by the Kaplan
band's 7 (deviation D1).

## 5. T1 extension to prime n (parallel family), proof status

The shipped T1 (mod.rs top doc, M171) extends verbatim to all odd n; the
spike verifies each part exhaustively at n = 9, 11, 13:

- **(a) Pinning.** At odd n the map `(p,q) ↦ M_p − M_q` (ordered pairs,
  `p ≠ q`) is **injective**: `M_p − M_q` is the difference of two equal-length
  radius vectors of the midpoint circle about O_A, which determines the
  unordered direction pair; since no two edge normals are antipodal at odd n,
  the ordered pair is recovered. Hence at most *one* both-midpoint crossing
  per parallel pin — and the pin `(i,j)` itself always realizes exactly one
  (itself). Machine: all `n(n−1)` pins, exactly one midpoint crossing
  `(i,j)`, at all three n. A parallel pair therefore can never have *both*
  transversal crossings at edge midpoints — the rotated family's defining
  property is out of parallel reach.
- **(b) Axis.** No midpoint pin `P = M_i − M_j` is parallel to any symmetry
  axis: pin direction `(i+j+1)π/n + π/2` vs axis directions `kπ/n` needs
  `n/2 ∈ Z`. Machine: zero axis-parallel pins over all `n(n−1)` (tested
  against both polygons' vertex- and edge-axis directions), at all three n.
- **(c) Two distinct angles.** Every clean parallel pin carries **two
  distinct** contact angles (`d ≡ 2(p−q) mod n`, no −1): 54/88/130 clean
  pins at n = 9/11/13, all with `|{cos²}| = 2` — so no parallel pin is ever a
  single-`{n/d}` record, which is the *reason* the odd-n catalog is rotated.
  Band-qualifying parallel deltas (c ∈ band, clean): n = 9: δ ∈ {2,7}
  (d-pairs {4,2} folded — 80°/40°, `X₂ = (1−cos20°, cos20°)` at δ=2 and the
  mirrored `(cos20°, 1−cos20°)` at δ=7 — VERDICT.md re-confirmed); n = 11:
  δ ∈ {2,3,8,9} (folded d-pairs {4,2} at c=3, {5,4} at c=2); n = 13:
  δ ∈ {2,3,4,9,10,11} (folded d-pairs {4,2} at c=4, {6,4} at c=3, {6,5} at
  c=2). Each δ-class holds n pins; δ and n−δ give mirrored angle pairs.
- **Cross-family disjointness**: no parallel pin equals any rotated pin
  (angle congruence would need `n/2 ∈ Z`); machine-checked at all three n.

Verdict: T1 extends to prime n with no changes; the *new* structural fact is
the rotated map's 2-to-1 partner involution (§4.1), which constrains the
implementation's pin canonicalization (ship the (Δ,0) labeling; the partner
labeling is the same configuration).

## 6. Outline certification (arc walk), per record

For the clean 2-crossing state, with crossings `X₀ = (A_Δ, B₀)`,
`X₁ = (A_{h+1}, B_{Δ+h})`: the four polygon arcs between the crossings are
each *uniformly* strictly inside/outside the other polygon (note: "outside a
convex polygon" = not-all-half-plane-signs-positive — a vertex past one edge
has mixed signs; the spike's first draft asserted all-negative and was
corrected). The lens takes the two inside arcs, the union the two outside
arcs; each cycle is simple; the interior full edges of the inside arcs equal
`eAB`/`eBA` exactly; and every cycle piece's midpoint is certified on the
correct side of the other polygon (including the two partial crossing-edge
half-pieces). Node counts:

```text
lens  = one cycle of 2c + 4 nodes          (8, 10, 12 at c = 2, 3, 4; 14 at c = 5)
union = one cycle of 2(n−c−1) + 2 nodes
```

Measured: (11,2) 8/18, (11,3) 10/16, (11,4) 12/14; (13,2) 8/22, (13,3)
10/20, (13,4) 12/18, (13,5) 14/16 — the M171 formulas hold verbatim. The
rotated split (union runs after X₀ along the B-arc `V_{eb0+1}..V_{eb1}` and
after X₁ along the A-arc `V_{ea1+1}..V_{ea0}`; the lens takes the
complementary arcs) is the reference for the hub `outlines()`
rederivation cross-check at implementation time.

## 7. Field-consumption map (engine-side contract)

Per record, the implementation consumes exactly:

- **`Field::K11` / `Field::K13`** — new additive variants in the `Field`
  enum (medallion/mod.rs, current variants `Qsqrt2, Qsqrt3, L10, K9`; the
  n = 11/13 sites are additive only, zero disturbance to existing variants).
- **Kq layer over Kₙ** (M178 §1/M179 §1): basis `(1, α, …, α^{D−1})`,
  `m₁₁ = y⁵+y⁴−4y³−3y²+3y+1` (D = 5), `m₁₃ = y⁶+y⁵−5y⁴−4y³+6y²+3y−1`
  (D = 6); arithmetic add/sub/mul with monic-reduction fold; inverse via the
  5×5/6×6 system (Gaussian on multiplication table columns — spike-verified
  roundtrip).
- **N-layer `(u + v·w)/den`** with `w² = 2−α`, `s = 2−α = [2,−1,0,…]` shape
  (spike: `m(2) = Φ_n(1) = n` prime ⇒ genuine quadratic extension): slots
  `u, v ∈ Z^D`, one integer denominator, canonical gcd over the `2D+1`
  integers. Pin encodings are the two pure slots `(u_x, 0)/4` and
  `(0, v_y)/4` plus `|P|²` (§2) — no mixed slot is ever produced by geometry
  (purity, below).
- **Sign engine**: `ksign` = interval-Horner over the field's ISO bracket +
  bisection guided by certified `m` increasing — ISO11 = {lo_num 112911137,
  hi_num 112911138, s = 26}, ISO13 = {[1856935, 1856936]/2²⁰} (M178 §4 /
  M179 §2.3; the spike uses exactly these brackets). N-layer sign = the
  two-level product rule `sgn z = sgn(u)·sgn(u² − s·v²)` (n9 §4.3 shape),
  `u² = s·v²` impossible (`N(s) = m(2) = n` prime).
- **Zero test** = tuple equality after canonical reduction (no numeric
  zero-test anywhere).
- **`contact_cos2` encoding**: `Option<(i64,i64,i64,i64)>` = `(a,b,d,den)`
  for `(a+b√d)/den`. **All seven records ship `None`** (cos² has full degree
  h — Galois-orbit battery) with the exact tuple pin (§2 cos² table) in the
  battery, the dec10/L10/n9-cos293 precedent.
- **/64 window convention**: unchanged (samples `σ = 1∓1/64`).
- **M163 exit-ray cross-check**: `Motif::Star { n, theta_deg =
  d_canon·180/n, s: 1 }` segments on the record exit-ray lines (f64,
  tol 1e-8, both directions of the shipped check: every segment on some
  exit line, every exit realized). Per record θ = 5·180/11, 3·180/11,
  180/11, 6·180/13, 5·180/13, 3·180/13, 180/13 — the canonical folded d
  (6, not 7, for (13,2)). The two Δ = 1 rungs sit at `θ = π/n`, exactly on
  the engine's EXTENSION-guard boundary — the recorded (9,3) boundary
  precedent (motif_segments has no guard; the standalone star emits the
  record exits).

### Engine gaps (what the implementation must add)

1. `Field::K11`, `Field::K13` variants (additive sites in the `Field` enum
   and its match arms).
2. An N-layer type over D = 5/6: `n9::Nq` is hard-wired to `[i64; 3]`; K₁₁
   needs width 5, K₁₃ width 6. The `(u + v·w)/den` shape, `s = 2−α`, the
   two-level sign rule, and the gcd canonicalization are *identical* —
   genericize over D (const generic or macro) rather than triplicate.
3. ISO-bracket plumbing: `ksign`'s dyadic Horner must take the field's own
   ISO (s = 26 / s₀ = 20; K₉ uses 31). The spike's engine replica ran
   **1,237,016 ksgn calls with 0 bisections** across the whole census+record
   workload — the batteries certify 0 bisections, census values never
   bisect.
4. Bit-height headroom: max Fraction bit height seen **22** (< 2⁵⁰, the
   M164 emitted-numbers bound); `kmul` raw-accumulator peak **25 bits**
   (i128-safe with enormous margin). All pin/cos²/window constants land at
   den ≤ 911 with coefficients ≤ 22 bits — the M164 battery verdicts
   (canonical, positive den, |coeffs| < 2⁵⁰) will pass with room.
5. Catalog table: seven new rotated rows (§2/§3 values are the battery
   constants, ready to paste).
6. `outlines()` rederivation cross-check per record (§6 counts).

## 8. Verification-gate spec (per record, implementation battery)

Each record gets the following battery mandates (mirror of the shipped n9
battery, generalized; every assert exact unless marked f64):

1. **Pin exactness**: `P == M_Δ − R·M_0` as `(u_x,0)/4 + (0,v_y)/4`; `|P|² ==
   u/4` (§2 tuples).
2. **Crossings**: exactly `(Δ,0)` and `(h+1, Δ+h)`, both at `t = s = 1/2`
   (`2·t = den` form); no other transversal crossing.
3. **eAB/eBA** lists (§2); `c_A = c_B = c`.
4. **d-rule** on both crossings: `d ≡ 2(i−j)−1 (mod n)`; `cos² == (2+C_d)/4`
   exact tuple; `0 < cos² < 1`.
5. **Degree certification**: Galois-conjugate orbit of cos² has exactly h
   distinct values ⇒ `contact_cos2 = None` + tuple pin.
6. **Window census**: event list over the σ-ray (exact K arithmetic,
   comparator `ksign(a−b)` sort, dedupe); no event at σ = 1; maximal window
   (§3 bounds); probes at window midpoint and `σ = 1∓1/64`: state key
   constant and `t ≠ 1/2`; midpoint verdict as measured (§3 table);
   bounding events re-verified real (adjacent states differ).
7. **t/s linearity**: `(t₀,k)`, `(s₀,ks)` exact (§3); `t₀+k = s₀+ks = 1/2`;
   `k ≠ 0`; monotonicity direction recorded.
8. **Outline walk** (§6): node counts 2c+4 / 2(n−c−1)+2; arc uniformity
   (strictly-in/out); `eAB`/`eBA` consistency; per-piece midpoint half-plane
   certification; simplicity (no repeated vertex, `X₀ ≠ X₁`).
9. **κ index maps** at the pin: `κ(V_k^A) = V^B_{k+h}`,
   `κ(V^B_m) = V^A_{m+h+1}` (all k, m).
10. **No vertex-on-edge contact** at the pin (dirty = ∅).
11. **Purity**: the record work produces **zero mixed-sign N-layer
    elements** (split representation: geometry arithmetic stays in the pure
    arms; spike-instrumented 0 hits across all census/record sections —
    15 mixed-arm hits total are *all* from the deliberate S1 field battery).
12. **Field-level regressions** (mandated by M179 §2.4/M178 §4, rerun as
    part of the 24c job): the N-layer wrong-shortcut mixed-sign battery —
    K₁₃ elements `(1,−3)`, `(2,−5)`, `(3,−7)`, `(−1,3)` plus `(1,±2)`
    boundary controls plus exact-zero elements (n9.rs
    `sgn_mixed_sign_product_rule_regression` is the K₉ mirror); K₁₁
    two-level battery per M178. These exercise the arm geometry never
    touches — mandated at field level, stated here so the 24c job cannot
    skip them.
13. **M163 cross-check** (§7): star segments vs exit rays, f64 tol 1e-8,
    with the (11,4)/(13,5) guard-boundary note.
14. **Census regression** (new, mandated by this design): the full n² pin
    enumeration of §4 at the implemented field — the band families, the
    2-to-1 partner identity, and the collinear fixed locus are regression
    pins (the spike is the reference oracle; ~35 s at n = 13 in stdlib
    Python, trivial in-release).

## 9. Spike evidence (what is machine-settled)

`spike_24c_oddn.py` (stdlib-only; `Fraction` exact arithmetic; ISO-bracket
sign engine replica with bisection instrumentation), sections S1–S8, run
`all`: **226 checks, 0 failures**, log `spike_24c_oddn_green.log`.

- **S1 fields** (n = 9, 11, 13): minpoly via half-Ramanujan power sums +
  Newton (matches shipped m₉/m₁₁/m₁₃); ISO bracket m-sign certified; sgn(α);
  `s = 2−α` shape; `m(2) = n` (genuine extension); cos bounds; kinv/kmul
  roundtrip; two-level N-sign vs f64 view (mixed arm deliberately
  exercised); `nsgn(0)`.
- **S2 geometry**: split-representation closure lemmas, exact: hull walk;
  `rotR² == rot2`; `rotRⁿ = −I`; `Σu_k = 0`; `|u_k| = 1`;
  `dot(u_p,u_q) = C_|p−q|/2`; circumradius/midpoint-radius identities.
- **S4 n9 oracle**: every shipped catalog value reproduced exactly — pins,
  norms, κ maps, crossings, eAB/eBA, d-rule, cos², windows
  (`σ_lo+σ_hi` midpoint verdicts), t-samples 63/128·65/128, outline counts
  8/14 and 10/12. (The trust anchor: identical machinery then applied to
  11/13.)
- **S5 T1**: parallel census `n(n−1)` pins × {9,11,13}: injectivity, T1(a)
  exactly-one-midpoint-crossing, T1(b) axis, T1(c) distinct angles, δ/angle
  findings, X₂ exact tuples.
- **S6 census**: rotated n² labels × {9,11,13}: 2-to-1 partner map, fixed
  collinear locus, families, band selection, dihedral orbit (n=9), ladder
  pins.
- **S7 records**: the 14-point battery of §8 on all seven records (exact
  values in §2/§3).
- **S8 engine metrics**: §7 item 3/4 numbers.

## 10. Proposed scope-line extension (verbatim, for PROPOSED.md §24c)

The §24c entry's scope line and closing note are proposed to become (edit
owned by the orchestrator; quoted verbatim from the current entry with the
extension applied):

> **24c. (n,c)-monster medallion generator** (the 22e medallion source)
> — overlapping regular n-gons fused at contact angles kπ/n (**n ≤ 13**,
> 2 ≤ c ≤ ⌈n/2⌉−2) as additional catalog entries in exact fields; feeds
> 24a's catalog. Strange-star sets {8,12}, {9,12}, {9,18} first.

and the closing note's last sentence replaced by:

> The odd-n ladder rungs at n = 11 (degree 5) and n = 13 (degree 6) are
> likewise designed engine-side:
> [docs/cyclotomic-field-degree5.md](docs/cyclotomic-field-degree5.md)
> (M178),
> [docs/cyclotomic-field-odd-n.md](cyclotomic-field-odd-n.md) (M179), and
> [docs/medallion-24c-odd-n-design.md](docs/medallion-24c-odd-n-design.md)
> (this doc) — seven rotated-family rungs ((11,2..4), (13,2..5)), census-,
> window- and outline-certified, all `contact_cos2 = None`. **Effort** M,
> **tier** strong.

(The existing "n = 11 remains blocked…implementation job pending" sentence
and the "note this entry's 'n ≤ 12' scope line must be extended for a
13-rung" remark are superseded by the above once this design lands; the
implementation job itself remains open.)

## 11. Proposed pointer lines for the two field docs (verbatim; not edited here)

For [docs/cyclotomic-field-degree5.md](cyclotomic-field-degree5.md) §5.3
(consumers), append:

> - **24c n = 11 rungs (this design's consumer)**: the three (11,2..4)
>   monster records consume K₁₁ exactly as contracted here (ISO11@s=26,
>   N₁₁ two-level sign rule) — engine-side design and battery mandates in
>   [docs/medallion-24c-odd-n-design.md](medallion-24c-odd-n-design.md)
>   (spike-validated: 0 bisections on the full census workload, max bit
>   height 22).

For [docs/cyclotomic-field-odd-n.md](cyclotomic-field-odd-n.md) §5.3
(consumers), append:

> - **24c n = 13 rungs (this design's consumer)**: the four (13,2..5)
>   monster records consume K₁₃ exactly as contracted here (ISO13@s₀=20,
>   N₁₃ two-level sign rule with the mandated wrong-shortcut regressions) —
>   engine-side design and battery mandates in
>   [docs/medallion-24c-odd-n-design.md](medallion-24c-odd-n-design.md)
>   (spike-validated: 0 bisections on the full census workload, max bit
>   height 22, all cos² full-degree ⇒ contact_cos2 = None).

## 12. Deviations and implementation-time decisions

- **D1 — seven records, not five.** The working brief listed
  (11,2),(11,3),(13,2),(13,3),(13,4); the Kaplan band (`2 ≤ c ≤ ⌈n/2⌉−2`,
  p. 123) adds the Δ = 1 rungs (11,4),(13,5), and the census settles them
  **clean** (windows, outlines, purity all green). The design covers seven.
- **D2 — rotated pin map is 2-to-1**, not injective (partner
  `(i,j) → (j+h+1, i+h)`; fixed locus = collinear overlap). The brief's
  injectivity assumption holds for the parallel map only. Implementation
  note: canonicalize records at the `(Δ,0)` labeling.
- **D3 — t₀ ∈ {0,1} is Δ-specific, not n-specific** (V₂ on the π/n ray at
  every n): the brief's warning that the n9 pattern does not generalize is
  refined — it generalizes exactly to the Δ ∈ {1,2} rungs; Δ ≥ 3 rungs have
  genuine Kₙ t₀ (values in §3).
- **D4 — "outside convex polygon" classification.** The arc-uniformity test
  is `all signs > 0` vs *not*-all (mixed signs are outside); an
  all-signs-negative reading is wrong (first-draft bug, caught by the n9
  oracle before any 11/13 claim).
- **Implementation-time decisions recorded** (to revisit at code time):
  N-layer genericization strategy (const-generic vs macro) — §7 item 2;
  whether the census regression of §8.14 ships as a release test or a
  `#[ignore]` oracle test (runtime budget); the catalog rows' `ρ`-column
  presentation for rotated rows (`σ = 1`, as the n9 rows already do).