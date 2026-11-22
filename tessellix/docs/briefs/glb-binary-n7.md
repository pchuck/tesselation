# Brief: glb-binary-n7 — the generalized Godrèche-Lançon-Billard binary tiling

**Family:** `quasiperiodic` · **Candidate:** PROPOSED.md §23e (GLB binary
re-scope) via `docs/cyclotomic-field-layer.md` §7 step 3 ("the first
consumer family … the cheapest XL") · **Effort:** XL ·
**Precedent to mirror:** `crates/tess-tilings/src/fields.rs` (the K₇
exact core, M137 — geometry layer) and
`crates/tess-tilings/src/families/quasiperiodic/golden_bee.rs`
(substitution-module structure, tests, header conventions).

> **Prepass status:** the rule has been derived, decoded from the
> sources, and certified by exact-arithmetic batteries (K₇/K₁₄ field
> arithmetic, stdlib Python). Everything in "The certified rule" and
> "The certified λ-scheme" below is measured, not speculative — paste
> it into the module header and reproduce it with in-module tests.
> The prepass scratch is `/tmp/opencode/glb-binary-n7/` (ephemeral —
> re-run scripts, do not re-derive).

### Goal

Land `glb-binary-n7`: the generalized Godrèche-Lançon-Billard binary
substitution tiling at n = 7 — three rhomb prototiles **P₁, P₂, P₃**
(unit edge, acute angles π/7, 2π/7, 3π/7), inflation
**λ = 2cos(π/14) ≈ 1.9498558244** (areal multiplier **λ² = 2 +
2cos(π/7) ≈ 3.8019377358**), and a **two-choice** fan/crown
substitution: each parent shape admits up to two same-census
arrangements (rule k odd and rule k = n−s even), and **both are
required** — odd-only does not tile past depth 2 (device meshing).
Substitution generator like the
golden-family siblings — not a lattice table. All interior angles are
integer multiples of π/7; all tile edges run in the 7 lattice
directions (even direction classes mod 28). **Property wording (cite,
don't overclaim):** Bielefeld tags the family FLC + Finite Rotations +
Rhomb Tiles; the inflation factor is explicitly **non-PV**
([GoLa92] via the Bielefeld binary entry — true for n = 7 too: the
conjugates of λ = 2cos(π/14) are ±1.9499, ±1.5637, ±0.8678, two of
them outside the unit circle), and the binary family does **not**
obey the alternation condition. No pure-point/quasiperiodic-spectrum
claim appears in the sources — mirror the `golden_triangle.rs` scope
note convention (spectra cited, not proven).

### Literature basis (pinned in prepass)

- **Bielefeld Tilings Encyclopedia**, entry "Generalized
  Godreche-Lancon-Billard Binary",
  `https://tilings.math.uni-bielefeld.de/substitution/generalized_binary/`
  (**underscores** — the hyphenated URL 404s). Fetched 2026-11-22,
  verbatim copy at prepass scratch
  `src/generalized_binary_entry.md`. Verbatim facts: "All interior
  angles are integer multiples of π/n. For n=5 it is identical to the
  Godreche-Lancon-Billard Binary tiling with 2 prototiles. For odd n
  it has (n−1)/2 prototiles. For even n it has n+1 prototiles. The
  inflation multiplier is √(2 + 2 cos(π/n))." Discovered by Pautze;
  first derived by Hibma. Tags include FLC. Example shown: n = 9.
- **S. Pautze, "Cyclotomic aperiodic substitution tilings",
  Symmetry 2017, 9(2), 19**, doi `10.3390/sym9020019`
  (arXiv:1606.06858) — binary section: eqs (75)–(77) (area vector =
  left eigenvector; µₙ,ₖ tabulation), **eq (78): "Mₙ = Mₙ,₂ + 2E (odd
  n)"**, Remark 9 ("… we choose the part which relays on prototiles
  Pₖ, n > k ≥ 1, odd k only"), and: "The generalized Lançon-Billard
  tiling does not contain any patches with individual dihedral
  symmetry Dₙ or D₂ₙ."
- **T. Hibma, "Generalization of Non-periodic Rhomb Substitution
  Tilings", arXiv:1509.02053 (2015)** — §2 grid mechanics (L145–190),
  §3 m₁ = 1, L = 2cos(π/n) (L193–330), §4 eq (11) Sₛ and the even-s
  set {2,4} for n = 5 = the binary (⟺ odd-k {1,3} under s = n−k).
- **Godrèche–Lançon–Billard** original: cite via the Bielefeld
  `binary` entry ([Lan88]); the GoLa92 PDF host is unreachable — do
  not retry.
- Re-verified in prepass: the rule decoded from Pautze Fig 4 and the
  Bielefeld rule/patch images; the n = 5 ground truth extracted from
  the Bielefeld binary patch image (2341 tiles,
  `src/n5_tiles.json`) and censused (below). Image pixel extraction
  is too noisy for evidence — ground truth enters only through the
  extracted tile census, everything else through the exact batteries.

**Known literature ambiguity, resolved in prepass:** Pautze's Remark 9
publishes the **odd-k-only** scheme. That scheme alone does **not**
tile at depth ≥ 2 (certified: σ³ assemblies come out exact-area but
with pinched, non-staircase boundaries). The even rules k = n−s are
the second arrangement of the same eq-78 core row; the substitution is
two-choice per shape. Do not "simplify" to odd-only.

### The certified rule (paste into the module header)

Units: directions are classes mod 2n·2 = 28; class c ↔ angle
c·π/14. **U(m) = (cos(mπ/14), sin(mπ/14))** for m ∈ ℤ/28 (opposite
classes are negatives). The seven tile-edge directions are the **even
classes** (multiples of π/7); all tiles in every certified patch have
even edge classes.

Fundamental identities (exact, in K₇ ⊂ Q(ζ₂₈)⁺):

- **λ·U(h) = U(h−1) + U(h+1)** — a λ-chord is a 2-step unit path.
- **φ = λ·rot(π/14)** (one half-class): **φ(U(m)) = U(m) + U(m+2)**.
- Unit tile (p, s) = [0, U(p), U(p)+U(p+2s), U(p+2s)], s ∈ {1,2,3}.
- Big tile (inflated parent) = φ(unit tile):
  big_tile(p, s) = [0, λU(p+1), λU(p+1)+λU(p+2s+1), λU(p+2s+1)] —
  by the chord identity all four corners are 2-step unit sums, i.e.
  unit-lattice points.
- **The big tile a child patch fills is an ε-region, not the straight
  rhombus:** each big-tile side of direction h expands to the two unit
  steps {h−1, h+1} in either order (16 staircase walks per big tile).
  Per side the two unit-edge classes are [step, side_class − step].
  All 16 walks share one class multiset; they differ in which corners
  are bumped. Corner "devices" (bumps/indents) of neighboring parents
  must mesh at shared vertices — this is the depth ≥ 2 constraint.

The rule (Pautze Fig 4 form): rule k places a fan of four sectors
around a fan center O with sector sizes
**(n−k−1, k, n−k+1, k)·(π/n)** (sums to 2π for every k; rays at
r₀ + 2·cumsum in class units). Sector a is filled by the rhombus of
shape min(a, n−a) presenting angle a·π/n at O; **a ∈ {0, n} sectors
are EMPTY** (a=0 degenerate full-vertex, a=n straight vertex).
n = 7 sector table and child shapes:

| rule k | shape s=min(k,7−k) | sectors (a) | children (shapes) | census (P₁,P₂,P₃) |
|---|---|---|---|---|
| 1 | 1 | [5,1,7,1] | [2,1,E,1] | (2,1,0) |
| 2 | 2 | [4,2,6,2] | [3,2,1,2] | (1,2,1) |
| 3 | 3 | [3,3,5,3] | [3,3,2,3] | (0,1,3) |
| 4 | 3 | [2,4,4,4] | [2,3,3,3] | (0,1,3) |
| 5 | 2 | [1,5,3,5] | [1,2,3,2] | (1,2,1) |
| 6 | 1 | [0,6,2,6] | [E,1,2,1] | (2,1,0) |

(k and n−k share the census row; the sector areas satisfy
Σ sin(aᵢπ/7) = λ²·sin(sπ/7) exactly for every k — verified to machine
precision for all six rules — so every rule line is area-exact.)

**TWO-CHOICE pools (certified):** per shape the candidate rules are
**s=1 → {k=1}** (k=6 structurally impossible: its a=0 sector is
degenerate, the two extreme rays coincide, and the patch outline has
6 unit edges vs the 8 edges of every ε-region — 0 level-1 survivors,
proved, not a bug); **s=2 → {k=5 ∪ k=2}**; **s=3 → {k=3 ∪ k=4}**.
Both rules of a pair have the same census — the substitution matrix is
well-defined regardless of the choices. This is exactly Pautze eq 78:
M − 2I has one 0/1 "core row" per shape, +2E adds the two same-shape
children; the odd and even rules are the two arrangements of that core.

**Substitution matrix and powers (rows = child census of P₁,P₂,P₃
parents):**

```
M  = [[2,1,0],[1,2,1],[0,1,3]]
M² = [[5,4,1],[4,6,5],[1,5,10]]
M³ = [[14,14,7],[14,21,21],[7,21,35]]
```

PF eigenvalue **= λ² = b²+b+1 = 3.8019377358048385** (verified by
power iteration and as the left-eigenvalue of the area vector
(sin(π/7), sin(2π/7), sin(3π/7)) — Pautze eq 75). Growth per
generation: single-parent seed totals 3/4/4 (depth 1), 10/15/16,
35/56/63, 126/210/245, 462/791/945, 1715/2989/3626, 6419/11319/13867,
24157/42924/52920, **91238/162925/201684 (depth 9)**, 345401/… at
depth 10.

**Level-1 survivor table (n = 7, exact battery; canonical fan rays at
r₀ = 4k, survivor = translation t whose patch boundary walk EQUALS an
ε-walk of big_tile(0, s); θ = r₀ − 4k mod 28; exactly 2 survivors per
valid rule, r₀-pairs differ by 2n = 14):**

```
k=1 s=1: r0= 4  t=(1.900969, 0.433884)  eps=(0,0,1,1)  children [2,1,1]   theta= 0
k=1      r0=18  t=(1.524459, 1.215715)  eps=(1,1,0,0)                    theta= 0
k=2 s=2: r0= 6  t=(1.623490, 0.781831)  eps=(0,0,1,1)  children [3,2,1,2] theta=26
k=2      r0=20  t=(1.123490, 1.408812)  eps=(1,1,0,0)                    theta=26
k=3 s=3: r0= 8  t=(1.222521, 0.974928)  eps=(0,0,1,1)  children [3,3,2,3] theta=24
k=3      r0=22  t=(0.678448, 1.408812)  eps=(1,1,0,0)                    theta=24
k=4 s=3: r0= 2  t=(0.777479, 0.974928)  eps=(0,1,1,0)  children [2,3,3,3] theta=14
k=4      r0=16  t=(1.123490, 1.408812)  eps=(1,0,0,1)                    theta=14
k=5 s=2: r0= 2  t=(1.222521, 0.974928)  eps=(0,1,1,0)  children [1,2,3,2] theta=10
k=5      r0=16  t=(1.524459, 1.215715)  eps=(1,0,0,1)                    theta=10
k=6 s=1: NO SURVIVORS (structural: 6-edge outline vs 8-edge ε-region)
```

(t printed to 6 decimals for reference only — the exact values are
K₁₄-field elements; e.g. k=1: t = λU(1) = U(0)+U(2).)

**Placement law (verified by walk-equality at every depth used):**
children of parent tile (p, s) under rule k = the canonical fan patch
`fan_patch(k, r₀=4k)` rotated by (p + θ_k) about the anchor side
vertex v_j and translated so O lands at φ(v_j) + t_k, where the anchor
v_j is the unique side-start vertex satisfying the turn test
(cls[j+1] − cls[j] ≡ 2s mod 28). Equivalently and concretely (the
prepass `place_child_gen`, which is the certified implementation):

```python
def place_child_gen(F, n, poly, rule_of_shape, frames, Ut, Uv2m):
    """All local placements of poly's rule patch whose boundary is a
    staircase walk of phi(poly): anchors x frames.  rule_of_shape maps
    shape -> rule k or LIST of rules (two-choice pools: odd union even)."""
    cls = poly_classes(F, poly, Uv2m)
    delta = (cls[1] - cls[0]) % (4*n)
    assert delta % 2 == 0 and 0 < delta < 2*n
    s = min(delta, 2*n - delta) // 2
    rul = rule_of_shape[s]
    ks = rul if isinstance(rul, (list, tuple)) else [rul]
    out = []
    for k in ks:
        Pcan, corners, sizes, rho = fan_patch(F, n, k, 4*k, Ut)
        for j in range(4):
            if (cls[(j+1) % 4] - cls[j]) % (4*n) != 2*s:
                continue                      # anchor turn test
            p = cls[j] % (4*n)
            w = phi_pt(F, poly[j])            # phi(v_j): U->U+U(+2)
            for fi, (theta, tk) in enumerate(frames[k]):
                T = padd(F, w, apply_place(F, p, ZERO(F), [tk])[0])
                pl = [apply_place(F, (p + theta) % (4*n), T, tl['poly'])
                      for tl in Pcan]
                wv = patch_boundary(F, pl)
                if wv is None: continue
                cc = canon_cyc(F, wv)
                wcanon = {}                   # 16 eps-walks of phi(poly)
                for bits, wc, _ in staircase_walks(F, n, p, s):
                    wT = canon_cyc(F, [padd(F, v, w) for v in wc])
                    wcanon.setdefault(wT, []).append(bits)
                if cc in wcanon:
                    out.append(dict(j=j, fi=fi, k=k, p=p,
                                    tiles=[dict(shape=tl['shape'], poly=q)
                                           for tl, q in zip(Pcan, pl)],
                                    eps=wcanon[cc]))
    return out
```

with `frames[k] = [(theta_k, t_k) per survivor]` from the table above.
Local candidates per parent tile: up to 2 anchors × 2 frames × #rules;
deduplicate by tile-set key before any search (the 2 frames are
r₀-mirror patches reachable twice).

### The certified λ-scheme (exact arithmetic; paste/adapt verbatim)

The prepass scheme runs everything over **F = Q(λ) ⊃ K₇**, λ =
2cos(π/14), minpoly **x⁶ − 7x⁴ + 14x² − 7**, via minpoly reduction of
polynomial tuples over Fractions, Chebyshev frames, and a 50-digit
Decimal shadow used for signs only (division-free predicates; exact
zero = canonical all-zero coefficients; |value| < 1e−25 raises):

```python
def mkfield(minpoly, x0, n):
    mp = [Fr(x) for x in minpoly]          # monic, low->high
    d = len(mp) - 1
    def red(p):                            # reduce mod mp, d-tuple low->high
        p = [Fr(x) for x in p]
        for m in range(len(p)-1, d-1, -1):
            t = p[m]
            if t:
                for i in range(d): p[m-d+i] -= t * mp[i]
                p[m] = Fr(0)
        del p[d:]
        while len(p) < d: p.append(Fr(0))
        return tuple(p)
    # add/sub/neg/mul/scal via red; iszero/eq coefficientwise (EXACT)
    # 80-Newton-step Decimal shadow lamD from x0; val() = Horner over lamD
    # sgn(): 0 if iszero else sign(val), AssertionError if |val| < 1e-25
    # cmp() = sgn(sub); t[k] = T_k(lambda/2) via t[k] = X*t[k-1] - t[k-2]
    def ccos(k):                           # cos(k*pi/(2n)) exact in F
        k %= 4*n
        return t[k] if k <= 2*n else t[4*n - k]
    def csin(k): return ccos(n - k)        # sin(k*pi/(2n)) = cos((n-k)*pi/(2n))
    return dict(d=d, n=n, ..., ccos=ccos, csin=csin)
```

- n = 7: minpoly `[-7, 0, 14, 0, -7, 0, 1]`, x0 ≈ 1.9498558243636194.
  n = 5 (battery cross-check): minpoly `[5, 0, -5, 0, 1]`,
  x0 ≈ 1.9021130325903071.
- **K₇ identities (b-basis, b = 2cos(2π/7) ≈ 1.2469796, RED b³ = 1 +
  2b − b²):** c = 2cos(π/7) = b²+b−1; c² = b+2; **λ² = b²+b+1**;
  d = 2cos(3π/7) = 2−b². K₇ = Q(ζ₁₄)⁺ (deg 3) is the even λ-basis of
  F (fixed field of λ ↦ −λ); λ² = 2 + c.
- Unit frames: `U(m) = (ccos(m), csin(m))`, m mod 28; chord table
  `lamU(m) = U(m−1) + U(m+1)`; φ(U(m)) = U(m) + U(m+2).
- **The landed module can run exact K₇ arithmetic end-to-end**
  (fields.rs alone). Certificate (`k7_twist.py`, ALL PASS): all 8816
  vertices of all 160 certified patches (level-1 both rules both
  survivors + all valid σ² sets, n = 5 and n = 7) have the form
  **v = (k₁, s₁·k₂)** with k₁, k₂ ∈ Kₙ (even λ-basis) and
  s₁ = sin(π/n) ∉ Kₙ, s₁² = (2−b)/4 ∈ K₇. So either (a) instantiate
  the fields.rs generic layer at the deg-6 minpoly (mirrors the
  scratch exactly — recommended, certified by execution), or (b) carry
  twisted K₇ pairs (k₁, k₂) per vertex with the s₁-twist applied only
  at emission. Same 6-integer payload per vertex either way. NB:
  multiplication by s₁ mixes λ parities — test k₂'s parity after the
  solve, never before.

### Prepass battery (reproduce these numbers; scripts in scratch)

All exact (F-field) unless marked float:

1. **Field identities** (`field_identities.py`): minpoly, Chebyshev
   closure, chord identity, K₇ b-basis identities, sector-area law for
   all 6 rules — "FAILS: none — ALL CERTIFIED".
2. **Level-1** (`subdiv.level1`): exactly 2 survivors per rule
   k ∈ {1,…,5}; k = 6 → 0 (structural); each survivor's patch boundary
   walk EQUALS an ε-walk of the big tile. Same for n = 5
   (k ∈ {1,2,3}; k = 4 → 0).
3. **σ² census** (`d3_two.py`): distinct globally-valid σ² tile sets
   per shape — **s=1: 8** (parent k=1 only, r₀ 4/18), **s=2: 32**
   (16 per parent rule k ∈ {2,5}, r₀ 6/20 and 2/16), **s=3: 64**
   (32 per parent rule k ∈ {3,4}) — each set passing
   `validate_level(2)`: counts == M² row, pairwise interior
   disjointness, edge-connected, single boundary cycle of 16 even unit
   segments, 2 consecutive pairings, rhombus closure, corner set =
   φ(big_tile), exact area 2λ⁴sin(sπ/7). Pattern counts per
   (rule, r₀): 4/8/16 for s=1/2/3 — the device-meshing pattern count
   doubles per shape index. (n=5: s=1: 8, s=2: 32.)
4. **σ³ census** (`d3_two.py`): for EVERY σ² set, per-parent
   subdivision candidates dedup → all-pairs interior-disjointness DFS
   → `validate_level(3)` (counts == M³ row, 32-edge single cycle, 3
   pairings, corner set φ²(big tile), exact area 2λ⁶sin(sπ/7)).
   **σ³(1): 8 sets × 8 valid assemblies = 64 TOTAL** (locally
   consistent per set: 555, 555, 625, 657, 657, 555, 625, 555);
   **σ³(3): 4 sampled sets — both parent rules and both r₀ mirrors
   (k3 r22, k4 r2, k4 r16, k3 r8) — 8 valid assemblies each (32 valid
   over 9042 locally-consistent)**; **σ³(2): 2 sampled sets spanning
   both parent rules (k2 r20: 9011 consistent; k5 r2: 5300
   consistent) — 8 valid assemblies each under a per-set early-stop
   cap of 8**; the k2-r20 set's FULL uncapped sweep count is **16
   valid over 19600 locally-consistent** (valid-per-set count grows
   with shape index, like the σ² pattern counts). n = 5 twin:
   σ³(1) full sweep 64 valid (multiset identical to n = 7);
   σ³(2) 4 full sets — 8 valid each, uncapped (8260/9350/9425/9590
   locally-consistent). Full 32/64-set n = 7 sweeps continue in
   scratch logs `d3_two_full.log`/`d5_two.log`; if a re-run differs,
   report the diff — do not fudge.
   DECISIVE: with odd-only pools σ³(1)/σ³(2) assemble exact-area but
   FAIL the staircase-boundary check (pinched, 36-edge cycles) — the
   even rules are REQUIRED.
5. **Orientation census** (`orient_census.py`, printed test output,
   ALL PASS): every unit edge in every certified patch has an EVEN
   direction class mod 2n. Measured direction sets: n=7 level-1 union
   {0,2,4,6,8} mod 14 (5 dirs), σ² union {0,2,4,6,8,10} (6 dirs);
   n=5 level-1 {0,2,4,6} mod 10, σ² {0,2,4,6,8}. (7 edge directions
   total exist for n=7.)
6. **Filling-space note** (`fill_enum.py`/`fill_all.py`, float
   enumerator, bugfixed): the full edge-connected unit-rhombus filling
   space of the 16 ε-regions with M-row census is much larger than the
   rule's selections: n5 s1: 38, n5 s2: 56; n7 s1: 65, n7 s2: 84, n7
   s3: 74 distinct fillings. The substitution RULE (fan/crown
   placements) selects a small subset. Never claim the fan search
   space is exhaustive.
7. **n=5 ground truth** (`census_n5.py` on the Bielefeld binary
   patch, 2341 tiles = 885 thin + 1456 thick): thin supertiles
   (1,1,3,5) = k=1 chains ×236 (161+75 over two orientation variants);
   thick supertiles BOTH (1,3,3,3) = k=3 fans ×424 (218+206) AND
   (2,2,2,4) = k=2 crowns ×399 (200+199); overlapping-window pairs
   367/370/187 (boundary-tolerant covering; M₅⁻¹·(885,1456) =
   (239.8, 405.4) non-integer — the finite patch is not an exact union
   of supertiles). Two orientation variants per type = the 2 mirror
   survivors. Empirical two-choice corroboration.
8. **PF/area checks** (float spot checks on exact M): PF(M₇) =
   3.801937735804838 = λ²; area vector left-eigenvalue; sector-area
   law diffs 0.00e+00 for all rules.

### Files you own (at landing)

- `crates/tess-tilings/src/families/quasiperiodic/glb_binary_n7.rs` —
  the new module (mirror `golden_bee.rs`: exact field arithmetic via
  `fields.rs`, substitution rule tables = the certified tables above,
  seed block, in-module tests; header documents rule source +
  two-choice + eq-78 decode).
- `crates/tess-tilings/src/families/quasiperiodic.rs` — module decl +
  `register` slice + `classification` arm + census `IDS`. (No
  `lattice_tilings()` change — not a lattice table.)
- `docs/families/quasiperiodic.md` — the catalog row (guard-checked).
- `crates/tess-tilings/src/fields.rs` — scoped shrink of the module
  level `#![allow(dead_code)]` (its comment says "no consumer family
  yet … §7 step 3 is a later brief" — that brief is this one): keep
  the blanket allow only for items still unconsumed (K₉/K₁₅/L₁₀
  tables; the girih_qc per-item precedent), or convert to per-item
  allowances. Minimal diff; `cargo check` stays warning-free.
- `/tmp/opencode/glb-binary-n7/` — prepass scratch (subdiv.py,
  d3_two.py, orient_census.py, k7_twist.py, fill_enum.py,
  census_n5.py, field_identities.py, NOTES.md, src/ pins). Re-run;
  do not re-derive. Copy any script you extend into the repo tests
  only as needed for in-module test data.

**Do not touch:** `lib.rs` (census pin/guards are orchestrator
territory — report the deltas instead), README, CHANGELOG, INDEX,
other families, UI crates.

### Constraints (from AGENTS.md, the ones that bite here)

1. Exact geometry end-to-end on the certified λ-scheme (or the
   documented K₇-twist equivalent) — floats only at the emission
   boundary. Never decimal literals on the exact path.
2. Python spike first (stdlib only, ≤240 s per experiment): the
   prepass scratch IS the spike; re-run to confirm, extend only for
   the Rust-mirror numbers.
3. **Two-choice is load-bearing.** Odd-only fails σ³ (certified). Do
   not reduce the pools; do not "fix" k=6 (structural impossibility).
4. Emission gotcha (AGENTS.md step 3): geometric tiles sharing a
   vertex emit exactly once.
5. Tile budget `MAX_TILES = 250_000`: single-parent seeds stay under
   budget through depth 9 (≤ 201684 tiles, worst shape P₃); depth 10
   exceeds (345401+). Region renders (parent + ring) should cap at
   depth 7–8. Pick the seed/window policy accordingly (see Open
   items).
6. Keep `cargo check --workspace` warning-free.

### Verification battery (what "done" means)

- In-module tests reproduce, in Rust on the exact field: the level-1
  survivor table (2 survivors/rule, k=6 empty, walk equality); the σ²
  census counts per shape (8/32/64 with the r₀/parent-rule split); one
  full σ³ assembly per shape passing `validate_level(3)`-equivalent
  checks (counts = M³ row, single 32-edge even-class boundary cycle,
  3 pairings, corner set, exact area); the orientation census (even
  classes only); area law per rule line; PF = λ² = b²+b+1 to float
  cross-check tolerance.
- Standard trio (below) plus the battery as in-module tests
  (`glb_binary_n7_tests`).
- Optional recon: export a depth-2/3 SVG and parse class counts
  against the count vector. Evidence is the numbers, not the look.

### Standard tests

Copy the trio shape from `golden_bee.rs`'s test module
(`glb_binary_n7_covers_region`, `glb_binary_n7_rotated_scaled`,
`glb_binary_n7_deterministic`).

### Scope note

The battery certifies substitution mechanics to depth 3 (all shapes),
the orientation law, and the exact-field representation. Cite exactly
what the sources claim: FLC + Finite Rotations + Rhomb Tiles
(Bielefeld tags), **non-PV inflation factor** ([GoLa92] via Bielefeld;
true for n=7: λ's conjugates ±1.9499, ±1.5637, ±0.8678 — two outside
the unit circle), alternation condition fails (binary entry). The GLB
family has no patches with individual dihedral symmetry Dₙ/D₂ₙ
(Pautze). Mirror `golden_triangle.rs`'s header note: diffraction/
pure-point spectra cited, not proven. Do not claim "exactly N
fillings" anywhere — the filling space is larger than the rule.

### Open items (decide at landing, report the choice)

- **MAX_TILES depth/window policy** (see growth table): seed depth
  cap 9; region window policy your call — report what you chose and
  the measured tile counts.
- **Denominator policy (design-doc OQ4):** b-basis integrality of the
  K₇-twist coordinates is NOT yet pinned (girih needed D=64; expect
  D=1 or 2 here). Census all frame/child coordinates and pin D from
  the data; do not assume.
- **Classification arm wording**: mirror the quasiperiodic siblings
  (substitution; `translational: Some("quasiperiodic")` pattern is for
  the family table — follow `golden_bee.rs`).
- n=5 sibling (the GLB binary proper): OUT OF SCOPE here.

### Design-doc notes (bonus, no erratum)

- **OQ2 (λ policy per family) CONFIRMED for this family:** λ =
  2cos(π/14) ∈ Q(ζ₂₈)⁺ (deg 6), λ ∉ K₇ (deg 3), λ² = b²+b+1 ∈ K₇;
  child-unit coordinates are mandatory and suffice (K-twist
  certificate). No change to the design doc needed.
- **OQ1 context (CAST DTO orientation policy):** Pautze's statement
  that the generalized Lançon-Billard tiling has no patches with
  individual dihedral symmetry Dₙ or D₂ₙ is consistent with the
  prepass geometry (mirror-pair survivors, no individual symmetry
  found at any certified depth). Informational only.
- **Errata against `docs/cyclotomic-field-layer.md`: none.** §7
  sequencing and OQ2 verified as written.

### Deliverable (worker report)

Per the fleet protocol in `tessellix/AGENTS.md`:

1. Diff-scope statement (exactly the owned files).
2. The pasted certified rule/scheme confirmation (any deviation from
   this brief must be reported with the battery diff that motivated
   it).
3. Battery numbers: level-1 table, σ²/σ³ censuses, orientation
   census, area law, PF; Rust pass counts; D-pin result.
4. Census deltas: quasiperiodic family 11 → 12; Quasiperiodic tab
   12 → 13; built-ins 129 → 130; memberships 143 → 144 (read the
   pinned totals in `lib.rs` at your start — if another tiling landed
   first, the orchestrator resolves cumulative state at integration;
   report what you saw). NOT a lattice table → remind the orchestrator
   no culling-test line is needed.
5. Proposed commit-message body (rule, field, verification). No M
   number — the orchestrator assigns it at integration.