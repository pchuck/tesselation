# Brief: Girih-inspired 14-fold — certified substitution rule, implementation dispatch

**Family:** `quasiperiodic` · **Candidate:** Bielefeld Tilings Encyclopedia
"Girih inspired 14-fold Tiling" (Pautze 2017) · **Effort:** L · **Tier:**
strong · **Precedent to mirror:** `crates/tess-tilings/src/families/quasiperiodic/glb_binary_n7.rs`
(dyadic K₇ twist-coordinate machinery — reuse its `Tw` representation and
normalization discipline), `stampfli.rs` / `ammann_beenker.rs`
(quasiperiodic presentation shape), `glb-binary-n7.md` (battery shape).
**Certified rule data (committed beside this brief, READ-ONLY input):**
`docs/briefs/girih-14-fold-rule.json`.

### Status and scope

Recon prepass **complete**; the substitution rule is **CERTIFIED exactly**
(no STOP clause on the rule — every number below is exact or a measured
pin). This document + the rule JSON are a **docs-only commit**: `crates/`
is untouched here. Implementation is **dispatch-ready**.

"Certified" means: all 11 substitution panels of the source (paper Figs
7.4–7.5, one panel per prototile class) were extracted from the vector
drawings as **exact K₇ twist-pair vertex sets** (849 kid rings: 86 corner +
86 edge + 677 interior) and verified by an exact-arithmetic battery
(§Battery below): corner rings equal the global-phase 14-gon model as
sets, edge decagons have exact center = exact edge midpoint and exact
mirror invariance across the parent edge line, interior kids are
inside-or-on their parent and pairwise disjoint under exact predicates,
the per-parent area law holds **exactly** (K₇-0) for all 11 rules, the
effective-count matrix N′ matches the certified values, and the Perron
eigenvalue of N′ equals λ² with power-iteration spread 0. Floats were
used only for discrete hypothesis matching during extraction (max
residual 1.0e-2 child units); every stored placement is exact.

### Goal

Land the girih-inspired 14-fold tiling in the quasiperiodic family:
**11 prototiles** (10 dihedral-D₂ tiles + 1 regular 14-gon), 14-fold,
inflation **λ = 2b²+3b ≈ 6.850855** (λ ∈ K₇, **not** a unit of K₇),
substitution acting on the module **½Z[ζ₁₄]** (twist pairs, dyadic),
Perron eigenvalue **λ² ≈ 46.934215**, single 14-direction edge family
(π/7 steps). Registry id `girih-14-fold`, module
`girih_14_fold.rs`, struct `Girih14FoldTiling`.

### Source and evidence chain

- **Primary:** Bielefeld Tilings Encyclopedia,
  `https://tilings.math.uni-bielefeld.de/substitution/girih-14fold/`
  (page slug hyphenated `girih-14fold`; image dir is NOT:
  `/img/substitution/girih14-fold/`). 11 prototiles; attribution
  Pautze, Stefan.
- **Paper:** [Pau2017] arXiv:1606.06858 = Symmetry 2017, 9(2), 19,
  doi:10.3390/sym9020019. **Extraction source = Figs 7.4–7.5** (all 11
  rule panels; `pdftocairo -svg` full-precision exports in scratch as
  `fig51.svg`/`fig52.svg`). Fig 7.6 is the patch figure.
- **Errata to carry in docs:**
  1. The Bielefeld page claims λ "is a unit" — **FALSE**:
     Norm(2b²+3b) = −7.
  2. [Pau2017] Table 7, n=7 row (5.801939) is contradicted by the
     paper's own Figs 7.4–7.6 (measured λ = 6.8507 ± 0.0006 across all
     11 panels). Pin λ from geometry, not the table.
  3. URL hyphen asymmetry (above) — recorded so image re-fetches don't 404.
- **Related-only (not candidates):** Nischke–Danzer deltoid 14-fold
  constructions; Danzer 7/14-fold. Same symmetry class, different rules.

### Pinned facts (exact; K₇ = Q(b), b = 2cos(2π/7), minpoly b³+b²−2b−1)

- λ = 2b²+3b — K₇ triple **(0,3,2)**, float 6.850855075327. Norm(λ) = −7
  (NOT a unit — erratum 1). λ² = 8+20b+9b² — triple **(8,20,9)**, float
  46.9342153 (= PF eigenvalue of N′; a scratch note saying "46.9342133"
  was a low-precision float rendering — the exact triple is (8,20,9)).
- Twist pairs: a point is **(P,Q) ∈ K₇²**, physical (x,y) =
  (val(P), sin(π/7)·val(Q)) — the **same representation as
  `glb_binary_n7.rs`'s `Tw`** (dyadic: integer Z[b] coefficients over 2^e,
  normalized per point). All coordinates of every certified placement
  are in ½Z[ζ₁₄], i.e. dyadic exponent ≤ 1.
- Twist multiplication: (P₁,Q₁)·(P₂,Q₂) = (P₁P₂ − s²Q₁Q₂, P₁Q₂ + Q₁P₂)
  with **s² = (2−b)/4** = (1/2, −1/4, 0). The quarter-denominator is why
  storage is dyadic; `glb_binary_n7.rs` already carries the same constant
  in its φ matrix.
- Rotation table: ζ₁₄^k as twist pairs — **`rot_table_t6` in the rule
  JSON** (14 rows, doubled coords; rot_j(z) = twist_mul(z, row j);
  rot₇ = (−1, 0) = point reflection). Mirror = (P, −Q). All rotations
  used by the rule are these 14 classes; the rule JSON's (j,m) indices
  refer to this table.
- Kid 14-gon circumradius **R = 1+b** (triple (1,1,0)), measured from the
  drawing as 19.32 fig units / 8.598 kid-edge = 2.2470 = 1+b ✓.
- Module closure: λ·(½Z[ζ₁₄]) ⊆ ½Z[ζ₁₄] (λ ∈ Z[b]; λ·ζ₁₄^k has integral
  cosine part and integral sin(π/7)-ratio part). Consequence: generating
  **downward** from a seed at scale λᴺ (built by repeated K₇
  multiplication by λ — never divide by λ; λ⁻¹ has denominator 7) keeps
  every coordinate dyadic exponent ≤ 1 at every level.
- **Prototile table** (edge = 1; S = certified 2·area/sin(π/7) ∈ K₇ —
  the form the area law uses; A = geometric area = S·sin(π/7)/2;
  angles are k·(π/7) in cyclic order):

| cls | nv | cyclic k | S (K₇ triple) | A (float) | convex | symmetry |
|-----|----|----------|----------------|-----------|--------|----------|
| R2  | 4  | 2525           | (−2,2,2)   | 0.7818  | yes | D₂ |
| R3  | 4  | 3434           | (2,2,0)    | 0.9749  | yes | D₂ |
| H26 | 6  | 266266         | (2,2,2)    | 1.6496  | yes | D₂ |
| H44 | 6  | 446446         | (6,4,0)    | 2.3837  | yes | D₂ |
| H45 | 6  | 455455         | (−2,6,4)   | 2.5386  | yes | D₂ |
| O29 | 8  | 28292829       | (0,6,2)    | 2.2978  | **no** | D₂ |
| O36 | 8  | 36663666       | (4,6,4)    | 3.8402  | yes | D₂ |
| D2  | 10 | 2828828288     | (0,10,2)   | 3.3799  | **no** | D₂ |
| D46 | 10 | 4666646666     | (8,12,6)   | 7.0058  | yes | D₂ |
| D56 | 10 | 5566655666     | (4,14,8)   | 7.3538  | yes | D₂ |
| G14 | 14 | 66666666666666 | (14,28,14) | 15.3345 | yes | D₁₄ |

  Tile vertex walk (canonical frame, certified): v₀ = (0,0), edge dirs
  d₀ = 0, v_{i+1} = v_i + ζ₁₄^{d_i}, d_{i+1} = d_i + (7 − k_{i+1}) mod 14.
  Vertex i's interior angle = k_i·π/7 (this holds also for k > 7 — the
  turn is right-handed and the angle reflex: **O29 and D2 are concave**;
  any containment/orientation code must handle reflex vertices).
  Wedge identity: Σ_i k_i = 7(nv − 2) for every class (verified).

### The certified rule (recipe + counts)

Canonical parent frame: parent vertices CP[i] = λ·CV[cls][i] (parent edge
λ, kid edge 1). Every parent decomposes into three kid groups:

1. **Corner kids** — one regular 14-gon (G14) per parent vertex i,
   vertices = CP[i] + R·ζ₁₄^m, m = 0..13, **global phase** (this exact
   set is forced by module membership; any other phase leaves ½Z[ζ₁₄]).
   The part inside the parent at vertex i is the wedge k_i/14 of the
   14-gon; the kid is emitted WHOLE (straddler). All 86 corner rings
   across the 11 panels match this model as exact sets.
2. **Edge kids** — one D2 decagon per parent edge i, geometrically
   **centered at the exact edge midpoint, exactly mirror-symmetric
   across the parent edge line** (exact set-invariance verified on all
   86), half inside / half outside (emitted whole). In the JSON each
   edge kid is an exact instance `(j, m, t6)` in parent edge-cyclic
   order (`rules[cls].edge[i]`); the geometric characterization is the
   human-readable form, the instances are the data.
3. **Interior kids** — exact instances `(kid_cls, j, m, t6)` in
   `rules[cls].interior` (677 total). `(j, m, t6)` semantics for 2+3:
   ring = t/2 + rot_j(mirror_m(CV[kid])) in rule coords (t6 = doubled
   twist pair, 6 ints). `(j,m)` is a **non-unique parametrization** for
   symmetric kids (D2 carries D₂ symmetry, G14 D₁₄); the certified
   object is the vertex SET; the stored (j,m) is the deterministic
   first match — rebuild rings from the JSON, don't re-derive (j,m).

**Interior kid counts per rule** (kid class → count):

| parent | interior kids |
|--------|---------------|
| D2 (nv10)  | D2:6 D46:1 H26:16 H44:4 R2:12 R3:8 = **47** |
| D46 (nv10) | D2:14 D46:4 G14:8 H26:20 H44:1 R2:12 R3:8 = **67** |
| G14 (nv14) | G14:15 H26:14 H44:28 H45:42 R2:70 R3:126 = **295** |
| H44 (nv6)  | D56:1 G14:2 H26:4 O29:2 R2:8 R3:16 = **33** |
| O29 (nv8)  | D2:2 H26:9 H44:4 R2:12 R3:8 = **35** |
| D56 (nv10) | D2:10 G14:10 H26:14 H45:3 O36:2 R2:18 R3:28 = **85** |
| O36 (nv8)  | D2:2 G14:4 H26:8 H45:4 R2:10 R3:22 = **50** |
| H26 (nv6)  | D46:4 R2:6 R3:4 = **14** |
| R2 (nv4)   | H26:4 H45:1 R2:2 R3:4 = **11** |
| R3 (nv4)   | D2:2 D46:1 H26:6 = **9** |
| H45 (nv6)  | D2:4 G14:2 H26:9 H44:4 R2:10 R3:2 = **31** |

Total kids per parent (whole-kid emissions, incl. nv corner + nv edge):
D2 67, D46 87, G14 323, H44 45, O29 51, D56 105, O36 66, H26 26, R2 19,
R3 17, H45 43.

**Effective-count matrix N′** (interior + corner wedges as (nv−2)/2
G14-equivalents + nv/2 edge-decagon halves; rows = parent, columns in
order R2,R3,H26,H44,H45,O29,O36,D2,D46,D56,G14) — certified, matches
the prepass spike exactly:

```
R2  : 2   4   4   0  1  0  0   2  0  0   1
R3  : 0   0   6   0  0  0  0   4  1  0   1
H26 : 6   4   0   0  0  0  0   3  4  0   2
H44 : 8   16  4   0  0  2  0   3  0  1   4
H45 : 10  2   9   4  0  0  0   7  0  0   4
O29 : 12  8   9   4  0  0  0   6  0  0   3
O36 : 10  22  8   0  4  0  0   6  0  0   7
D2  : 12  8   16  4  0  0  0   11 1  0   4
D46 : 12  8   20  1  0  0  0   19 4  0   12
D56 : 18  28  14  0  3  0  2   15 0  0   14
G14 : 70  126 14  28 42 0  0   7  0  0   21
```

**Area law (exact):** for every parent class r,
  Σ_c N′_rc·S_c − λ²·S_r = 0 **exactly in K₇** (all 11 rows; equivalently
  per panel: Σ interior S_c + (7(nv−2)/14)·S_G14 + (nv/2)·S_D2 − λ²·S_r
  = 0). This certifies the wedge/half/interior decomposition partitions
  the parent with no double-counting.

**Adjacency/dedup certification** (generation-critical):
- Corner rings are phase-global ⇒ all parents meeting at a vertex emit
  the **identical** ring; dedup by exact vertex set merges them.
- Edge decagons: one relative family per direction d across all 11
  panels, **and F_d = F_{d+7}** for all 14 d ⇒ the two parents sharing
  an edge emit the **identical** decagon; dedup merges them. Verified
  over all 110 edges.
- Adjacent corner rings cannot overlap: 2R = 2(1+b) ≈ 4.494 < λ ≈ 6.851;
  a corner ring and the adjacent edge decagon touch in exactly one
  point (both extreme along the edge at distance R from the vertex —
  observed exactly in the drawings).

### Generation semantics (correctness-critical)

- Tile instance: (cls, scale s, orientation (J,M), anchor t) — vertices
  t + s·g(CV[cls]) with g(z) = rot_J(mirror_M(z)), all in twist pairs.
  Level-0 seed at scale 1; a level-n tile has scale s = λⁿ.
- The rule is drawn with parent scale λ, kid scale 1. Substituting a
  parent of scale s yields kids at scale **s/λ = λⁿ⁻¹** (exact K₇ value —
  generate downward from a big seed; never multiply by λ⁻¹).
- **Uniform kid transport** for edge + interior kids (rule-frame point
  z_rule → world):  world = t_p + (s/λ)·g_p(z_rule).  For an interior
  kid (j,m,t_k): z_rule = t_k + g_k(CV[kid]); composition
  **g_p ∘ g_k = (Jp + (Mp ? −j : j) mod 14, Mp XOR m)**.
- Corner kids: world ring = world parent vertex i + (s/λ)·g_p(G14REL);
  since G14REL is dihedral-invariant, g_p(G14REL) = G14REL as a set —
  the global phase survives transport automatically (no phase choices
  at world scale).
- **Do NOT build kid placements from midpoint arithmetic at world
  scale**: edge midpoints live in ¼Z[ζ₁₄] cosets. Use the transport
  formula on the rule-frame rings instead (mid_world =
  t_p + (s/λ)·g_p(mid_rule) exactly, so transported rings stay in
  ½Z[ζ₁₄]).
- **Dedup by exact vertex set** (class + sorted 6-int doubled key):
  emit whole kids (corner rings and edge decagons straddle), then
  dedup — shared kids are bit-identical across incident parents
  (certified above). Finite patches therefore have a **fringe**:
  boundary kids appear whole, extending ≤ R·s ≈ 2.25·(kid scale) past
  the parent union.
- Single edge-direction family: all edges at every level are
  λⁿ·ζ₁₄^k (one π/7-family) — module closure; a direction census
  (mod 14) on any generated patch must find exactly 14 classes.
- Growth: tile count multiplies per level by Σ-kids/parent ≈ 17–323
  (≈ λ²·S_parent/mean-kid-area); area grows exactly λ² per level.
  From a single seed expect ~10²–10³ tiles at depth 2 and ~10⁴–10⁵ at
  depth 3 — **MAX_TILES = 250_000 bites at depth 3–4**: seed small or
  window-prune (presentation may also grow seed scale instead of depth;
  mirror how `stampfli.rs` budget-feeds its patch).

### Extraction provenance (how the JSON was certified — for re-derivation)

- `fig51.svg`/`fig52.svg`: full-precision `pdftocairo -svg` exports of
  [Pau2017] Figs 7.4/7.5; drawn kid edge s ≈ 8.5976 fig units.
- Classifier (reproducible): kids = filled polygons, fill ∉ {white,
  black}, median edge within 0.6 of 8.598; corner/edge kids: ring
  vertex-average within 0.35 fig of parent vertex / edge midpoint;
  interior = all vertices inside-or-on (ε 0.25 fig). Parent outlines =
  black-stroke closed polylines. White fills = backgrounds/ghosts.
- Frame solve per panel over 28 discrete (J,M) hypotheses + gauge vote
  from corner/edge kid centroids (no continuous fitting); interior kids
  by exact lattice propagation from parent/corner/edge anchors
  (TOL_MATCH 0.015, TOL_RING 0.02; distinct-vertex separation ≥ 0.868
  makes matching unambiguous).
- **Measured panel residuals** (drawn vs exact, child units — floats
  used for matching only; stored placements exact):

| panel | frame | corner | edge | interior |
|---------|---------|---------|---------|---------|
| D2  | 1.08e-03 | 8.36e-04 | 8.22e-04 | 8.50e-04 (47/47) |
| D46 | 9.46e-03 | 5.44e-03 | 5.55e-03 | 5.65e-03 (67/67) |
| G14 | 4.10e-03 | 3.60e-03 | 4.44e-03 | 5.13e-03 (295/295) |
| H44 | 1.50e-03 | 1.67e-03 | 1.49e-03 | 2.29e-03 (33/33) |
| O29 | 6.16e-04 | 8.33e-04 | 6.87e-04 | 6.32e-04 (35/35) |
| D56 | 5.71e-03 | 3.17e-03 | 3.13e-03 | 3.01e-03 (85/85) |
| O36 | 5.50e-03 | 3.30e-03 | 3.11e-03 | 3.96e-03 (50/50) |
| H26 | 1.17e-03 | 8.76e-04 | 8.27e-04 | 6.95e-04 (14/14) |
| R2  | 4.97e-04 | 5.72e-04 | 4.56e-04 | 4.56e-04 (11/11) |
| R3  | 3.95e-04 | 5.98e-04 | 4.31e-04 | 4.61e-04 (9/9) |
| H45 | 6.66e-04 | 7.12e-04 | 6.73e-04 | 7.18e-04 (31/31) |

- **Bielefeld patch specimen is NON-GENERIC — do not use as a census
  oracle**: 1564 tiles, O36 absent, PF frequencies deviate on rare
  classes (D46 5 observed vs 35.8 expected; O29 18 vs 4.3); its seed is
  not a rule-frame tile. Use self-generated patches for statistics.

### Verification battery (measured pins — what "done" means)

Per-check prepass numbers below are measured (scratch
`/tmp/opencode/girih14-recon/`, salvage `.scratch/girih14-recon/`);
the implementation must reproduce or supersede each. Programmatic
evidence only.

1. **Rule-table rebuild (exact):** rebuild all 849 kid rings from
   `girih-14-fold-rule.json` (reference consumer:
   `verify_rule_json.py` in salvage — 849/849 exact in Python). In
   Rust: in-module test recomputing every rule panel's kid set from
   the JSON + the vertex walk and comparing vertex sets exactly
   (dyadic keys) against the same JSON. Any mismatch = port bug, not
   data bug (STOP).
2. **Area law per rule (exact K₇):** Σ_c N′_rc·S_c − λ²·S_r = 0 for all
   11 rows, computed in Rust with K₇ arithmetic (values from the
   table above).
3. **PF eigenvalue:** dominant eigenvalue of N′ = λ² = 46.9342153
   (float tolerance 1e-6; power iteration spread 0 in prepass).
4. **Module + direction census:** every vertex of any generated patch
   has dyadic exponent ≤ 1; edge-direction census (mod π/7) finds
   exactly 14 classes, every edge a λⁿ·ζ₁₄^k.
5. **Two-level patch (R2 seed at scale λ²):** depth-2 substitution
   (19 level-1 parents → ~1.8k emissions pre-dedup); pin in-test: exact
   tile count after dedup, zero overlapping-kid pairs (exact predicate
   spot-check or count identity via the area law: Σ kid areas − λ²·(Σ
   parent areas) = 0 up to fringe handling — fringe tiles counted
   whole), corner/edge dedup idempotence (re-running substitution adds
   no duplicate placements).
6. **Rare-class coverage:** O36 has PF frequency ≈ 0.006% (≈1/17000) —
   it will be ABSENT from small patches (expected, not a bug). Cover
   O36 by (a) the O36 parent panel (66 kids, from the rule table) and
   (b) a D56-seeded patch (O36 appears as interior kid, 2 per D56).
7. **λ measurement:** from a rendered/generated level-1 panel, measured
   parent-edge / kid-edge = λ within 1e-3 (prepass measured 6.8507 ±
   0.0006 across all 11 panels vs exact 6.850855075327).
8. **Panel render match:** rendering the 11 canonical panels (one seed
   tile each, orientation identity) reproduces the paper figure
   structure: per-panel tile counts = nv corner + nv edge + interior N
   (table above), all vertex sets equal to the JSON rings.
9. **Standard trio** (deterministic generation, census consistency,
   render smoke) mirroring `golden_rhomboid.rs`'s test module, plus
   MAX_TILES discipline: generation terminates at the budget with a
   coherent patch (report achieved depth/counts).

### Census deltas (re-derive at dispatch — STALE-BASE WARNING)

At prepass base `0babc9c`: built-ins **143**, memberships **157**,
quasiperiodic **13**. These are almost certainly stale at dispatch —
**re-derive against your own base** before reporting. Expected deltas
for this landing: built-ins +1, memberships +1, quasiperiodic 13 → 14.
`LatticeTiling`? No — this is a substitution family entry; no culling
test line expected. Guards (`registry_groups_are_complete_and_canonically_ordered`,
`family_docs_match_registrations`, `readme_catalog_matches_registry`)
are orchestrator-owned: report verbatim, never fix.

### Files you own (implementation dispatch)

- `crates/tess-tilings/src/families/quasiperiodic/girih_14_fold.rs` —
  the new module.
- `crates/tess-tilings/src/families/quasiperiodic.rs` — module decl,
  `register` slice, `classification` arm, census `IDS`.
- `docs/families/quasiperiodic.md` — catalog row (guard-checked).
- `/tmp/opencode/girih14-recon/` — scratch (Python spikes, batteries).
- This brief + `docs/briefs/girih-14-fold-rule.json` are committed
  prepass artifacts — treat the JSON as **certified read-only input**;
  if you believe it is wrong, STOP (§STOP) instead of patching it.

**Do not touch anything else** — not `lib.rs` (census pin, guards,
culling test), not README, not `docs/INDEX.md`, not
`docs/history/CHANGELOG.md` (M number written by the orchestrator),
not other families, not the UI crates, and do not push.

### Constraints (from AGENTS.md)

1. Exact geometry: twist-pair vertices over K₇ (dyadic integer
   coefficients, per-point exponent — `glb_binary_n7.rs` `Tw` pattern);
   never decimal literals for placements.
2. Verify numerically BEFORE writing Rust: Python spike, **stdlib
   only**, in `/tmp/opencode/girih14-recon/`, foreground with per-step
   output, ≤ 240 s per experiment.
3. Emission: when several geometric tiles of one class share a vertex,
   emit each geometric tile exactly once (dedup by exact vertex set —
   here certified to suffice: §Adjacency).
4. Tile budget `MAX_TILES = 250_000`; keep generators cheap (growth
   ≈ 17–323 kids/parent/level — budget-feed like `stampfli.rs`).
5. Keep `cargo check --workspace` warning-free.
6. Census deltas re-derived against main at dispatch time (§Census).
7. No repo-wide `cargo fmt` (convention: `cargo fmt` defaults only, no
   rustfmt/clippy config, no pre-commit hooks).
8. Orchestrator census guards report verbatim, never fix.
9. No M number in commits, docs, or the report; the orchestrator
   assigns it at integration.

### Standard tests

The 3-test trio every tiling ships (deterministic generation, census
consistency, render smoke) — copy the shape from `golden_rhomboid.rs`'s
test module — plus the battery above as in-module tests.

### STOP conditions

- Any rule-JSON rebuild mismatch in Rust (after re-checking against the
  Python reference consumer) — the data is certified; a persistent
  mismatch means the port misreads the encoding.
- Any battery pin failing with no documentation-consistent explanation.
- Discovering the committed JSON was edited by anything but the prepass
  (checksum drift vs the salvage copy in `.scratch/girih14-recon/`).

### Deliverable (worker report)

Per the fleet protocol in `tessellix/AGENTS.md`:

1. Diff-scope statement (exactly the owned files).
2. Verification evidence with exact numbers and pass counts (battery
   §1–§9; programmatic only — a visual look is recon, never evidence).
3. Census deltas re-derived against current main (expected +1/+1/+1;
   quasiperiodic → 14).
4. Proposed commit-message body (basis, family contents, verification).

### Effort + tier

PROPOSED.md §23f guessed **L**. Prepass verdict: **L stands, now
de-risked** — the historically hard part (the exact 677-instance
interior layouts) is committed certified data; what remains is a
representation port onto the existing K₇/Tw machinery, a substitution
generator with certified dedup semantics, the battery, and family docs.
No novel derivation remains. **Tier: strong-worker** (exactness
discipline + concave-tile geometry + dyadic bookkeeping; not frontier —
the derivations are all pinned above).

---

## Appendix — recon scratch index

Live scratch: `/tmp/opencode/girih14-recon/` · Salvage (durable):
`.scratch/girih14-recon/` (repo root).

| file | role |
|---|---|
| `NOTES.md` | full evidence chain s1–s12 (source hunt, λ pinning, errata, classification, spike, pass-1/2 certification, encoding decisions) |
| `svgparse.py` | CTM-aware SVG polygon parser (`walk_svg`, `elen`, `inner_angles`, closed rings) |
| `rules4.json` | classified kid rings per panel (corner/edge/interior/context, 4-decimal) |
| `cyc.json` | the 11 certified cyclic k-sequences |
| `spike2.py` | exact area-law spike (K₇ Fractions; all 11 rows EXACT-0) |
| `placements.py` | pass 1: exact kid-ring extraction from fig51/52 SVGs → `placements_raw.json` |
| `placements2.py` | pass 2: exact verification battery + emits `girih-14-fold-rule.json` |
| `verify_rule_json.py` | JSON round-trip verifier + reference consumer (849/849 exact) |
| `placements_raw.json` | the 849 exact kid rings (twist pairs as Fraction strings) |
| `girih-14-fold-rule.json` | the committed rule table (copy of `docs/briefs/` artifact) |
| `fig51.svg`, `fig52.svg` | pdftocairo vector exports of [Pau2017] Figs 7.4/7.5 (extraction source) |
| `pau2017_arxiv.pdf`, `entry.html` | sources (paper; Bielefeld entry) |
| `patch.svg` | Bielefeld patch specimen (non-generic — census oracle FORBIDDEN) |
| `dbg_edge.py` | edge-decagon alignment debug (kept for the record) |

View-once discipline held: `rule.png` viewed once (recon); every
certification number above is programmatic (exact K₇ arithmetic or
measured residuals against parsed vector data).