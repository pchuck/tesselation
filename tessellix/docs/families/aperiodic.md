# Aperiodic family

Aperiodic monotiles and forcing sets: hat, turtle, spectre, Taylor–Socolar, Jeandel–Rao; plus the limit-periodic half-hex, domino (table) and period-doubling substitution tilings, and the aperiodic thue-morse substitution tiling.

Code: `crates/tess-tilings/src/families/aperiodic/` — cargo feature `aperiodic` (default-on).
Adding a tiling here touches only this family: `families/aperiodic.rs`
(module decl, register slice, classification arms, census `IDS`,
`lattice_tilings` entries) plus the new module and its doc row below.
Work briefs: `docs/briefs/TEMPLATE.md`; fleet protocol: `tessellix/AGENTS.md`.

## Implemented (9 tilings)

### Aperiodic monotiles and tile sets

| id | motif | notes |
|---|---|---|
| `hat` | Hat (aperiodic monotile, "einstein") | SMKG 2023; H/T/P/F metatile substitution (29-rule patch, 22 kept), exact Eisenstein placements; inflation φ² only in the limit (iterated outlines drift by design); tiles only with its mirror image; per-view seeded lineage blobs with exact clipped-area acceptance; non-edge-to-edge (565 midpoint T-junctions at depth 3) |
| `turtle` | Turtle ("hats in turtles", einstein family) | turtle (14 sides, area 10√3) + hat prototiles on Kaplan's exact hex lattice; 9-label 8+1 key-point substitution with per-level chirality flip; homochiral (all tiles unreflected — the plain turtle tiling mixes reflected copies), edge-to-edge; turtle:hat ratio → 4+√15 |
| `spectre` | Spectre (chiral einstein Tile(1,1)) | single equal-edge prototile, aperiodic *without* reflections; 9-label D12 substitution (spectre.js-identical), exact Q(√3) surd geometry; edge-to-edge |
| `taylor-socolar` | Taylor-Socolar striped hexagon (decorated-hexagon monotile) | the paper's forced 529-hexagon patch (Fig. 18 of arXiv:1008.3103) as a verbatim state table, each hexagon partitioned into 8 non-overlapping convex sub-polygons carrying one thick vertex-to-vertex chord plus a centre-split axis (2 chord-band halves + 2 axis half-bands + 2 body pentagons + 2 body triangles, 12 templates); the chords close into complete equilateral Taylor triangles on three scales (60 / 12 / 2 of side √3, 3√3, 7√3), and the state rules stay exact (R1 0/2992, R2 0/2816, R3 0/2904 re-derived in tests); finite covered patch (girih precedent), no period within hex-radius 30 |
| `jeandel-rao` | Jeandel-Rao 11-tile Wang tiles (smallest aperiodic Wang set) | the author's canonical 100x100 patch (arXiv:1506.06492, cited not re-proven) as an emitted table, each unit cell partitioned into 5 non-overlapping convex sub-polygons (4 edge bands of width 0.08 + interior, areas exact in Q) whose band colors make the Wang edge matching visible (H 0/9900, V 0/9900 on exact 9-color ids re-derived in tests); tile frequencies pinned; no grid translation period |t| ≤ 30 (2820 probes); finite covered patch (Taylor-Socolar precedent); no generator port — the study's DFS growth is a proven dead end (M93) |
| `half-hex` | Half-hexagon (Socolar limit-periodic substitution) | Bielefeld entry "half-hex": one **achiral** half-hexagon prototile, six rotation classes (no reflection class — its mirror is a translation of a rotation), λ = 2 substitution into 4 children (substitution matrix S[j][i] = 1 iff (j−i) mod 6 ∈ {0,2,3,4}; PF eigenvalue 4 = λ², char poly (x−4)(x−2)(x−1)²(x+1)², pinned by exact 720-permutation i128 expansion); finite covered patch = 2^d-scaled half-hexagon recentered on the origin, d = 5..8 picked by the view diagonal (nested views bit-identical on their intersection); exact √3-field integer-pair (u,v) arithmetic end to end, floats only at the view boundary; limit-periodic — the minimal x-period of the depth-g patch is (3/2)·2^g = 6, 12, 24, 48, 96 at g = 2..6 (strictly 2× growing), cited not re-proven (Fre02 cut-and-project; GS87 Ex. 10.1.3) |
| `domino` | Domino (table) substitution (Lançon limit-periodic) | Bielefeld entry "domino" (says verbatim "Also known as 'table tiling'" — no independent "table" entry in the substitution index; "table" is an alias of this one tiling): two domino prototiles on the square lattice — A = vertical 1×2, B = horizontal 2×1 — λ = 2 orientation-preserving substitution into 4 children (substitution matrix S = [[2,2],[2,2]], column sums = 4 = λ², PF eigenvalue 4 = λ², char poly x²−4x = x(x−4), pinned by exact 2×2 determinants); finite covered patch = the 2^d × 2^(d+1) block recentered on the origin, d = 5..8 picked by the view diagonal (nested views bit-identical on their intersection); exact integer cell arithmetic on Z² end to end, floats only at the view boundary; limit-periodic via **approximating periodic tilings** — the depth-g patch carries no proper within-patch period (measured g = 1..6; Rust battery g = 2..6): it is itself the fundamental cell of the period-(2^g, 2^(g+1)) approximant, (2,4), (4,8), (8,16), (16,32), (32,64), (64,128), strictly 2× growing per level — a different kind from half-hex, whose patch is itself (3/2)·2^g-periodic; **not** a cut-and-project / model-set structure (entry states verbatim that its dynamical spectrum has a continuous component, "Thus it cannot be a cut and project tiling" — Solomyak ETDS 17(3) 1997; Robinson Indag. Math. 10(4) 1999, "On the table and the chair"); entry tags: Finite Rotations {id}, Polyomio Tiling, Rep-Tiles, Self-Similar Substitution; discovered by Frédéric Lançon |
| `period-doubling` | 4 prototile label classes, one square prototile | Bielefeld entry is 1D-only (the "period-doubling" word substitution a→ab, b→aa over the 2-adics, Frettlöh & Sing); the tiling here is its 2D product rule σ⊗σ on the square lattice: one unit-square prototile carrying 4 label classes (aa/ab/ba/bb — cell (c, r) of the depth-g word w gets class 2·w[c] + w[r]), λ = 2 into 4 children per class (substitution matrix S[j][i] (child j, parent i) = [[1,2,2,4],[1,0,2,0],[1,2,0,0],[1,0,0,0]], column sums = 4 = λ², PF eigenvalue 4 = λ², char poly x⁴−x³−12x²−4x+16 with eigenvalues {4, 1, −2, −2}, pinned by exact i64 determinant expansion); finite covered patch = the 2^d × 2^d block recentered on the origin, d = 5..8 picked by the view diagonal (nested views bit-identical on their intersection); exact Z² integer-cell arithmetic end to end, floats only at emission; limit-periodic via **approximating periodic tilings** — the depth-g patch is itself the fundamental cell of the period-(2^g, 2^g) approximant, measured (2,2), (4,4), (8,8), (16,16), (32,32), (64,64) at g = 1..6, strictly 2× growing per level — ratio-1 square manifest, where domino's is ratio-2 (2^g, 2^(g+1)); the unit-square in-tile-offset degeneracy of the domino period probe (offsets 0 → no proper period for any p ≥ 1) is documented in-module; aperiodic — no Bravais lattice, no space group (lattice "—", group "—") |
| `thue-morse` | 4 prototile label classes (seeds 0–3) over one square prototile | Bielefeld entry "Thue Morse" is 1D-only (the classic Thue–Morse morphism 0→01, 1→10, reference JS99 = Allouche & Shallit, Springer 1999); the tiling here is its 2D product rule τ⊗τ on the square lattice: one unit-square prototile carrying 4 label classes (00/01/10/11 — the child at frame cell (c, r) of parent p = 2x+y gets class 2·(x⊕c)+(y⊕r)), λ = 2 into 4 children per class (substitution matrix S = the all-ones 4×4 — the Steinhaus property, every parent block contains each child class exactly once; column sums = 4 = λ², PF eigenvalue 4 = λ², char poly x⁴−4x³ with eigenvalues {4, 0, 0, 0}, pinned by exact i64 determinant expansion); finite covered patch = the 2^d × 2^d block recentered on the origin, d = 5..8 picked by the view diagonal (nested views bit-identical on their intersection); exact Z² integer-cell arithmetic end to end, floats only at emission; **aperiodic at every depth** — unlike the limit-periodic period-doubling cousin, the depth-g patch carries **no proper translation period** (measured g = 1..6: all (g+1)²−1 proper candidate shifts rejected — 3/8/15/24/35/48, 133 cumulative, zero survivors — each killed by the class-0 (0,0) cell, since w[2^k] = 1); singular continuous diffraction (JS99), not pure point — aperiodic, no Bravais lattice, no space group (lattice "—", group "—") |

## Build arcs

Numbered arcs keep their PATTERNS.MD §numbers — code comments and
briefs cite them as `§N`.

### 8. Hat tiling (aperiodic monotile) ✅ (M34)

- **Motif:** the hat — a 13-sided polykite assembled from eight kites of
  the [3.4.6.4] Laves tiling; the "einstein" of Smith, Myers, Kaplan and
  Goodman-Strauss (arXiv:2303.10798). It tiles the plane, but only
  aperiodically, and only together with its mirror image. World area
  exactly 8√3 (six sides 1, six sides √3, one side 2 = two collinear
  unit edges).
- **Construction (paper Fig 2.6 rules, cross-validated against Kaplan's
  hatviz and the einstein-tiling tables; exact Python battery before
  Rust):** hats cluster (Theorem 4.1) into four metatiles — H (4 hats),
  T (1), P (2), F (2); a 29-rule patch fit by edge alignment, 22 kept
  as the next level's supertiles; new outlines from a few key patch
  points and 120° hexagon corners. Every tile placement is provably an
  exact Eisenstein integer pair (snapped, debug-asserted residual
  < 0.25). The iterated supertile outlines are rational and **drift by
  design** — the paper builds each generation with slightly different
  outlines, so inflation φ² holds only in the limit (measured step
  ratios 1.78, 2.64, 2.66, 2.64).
- **Exact model:** Eisenstein integers `(a, b) = a + b·ω`, world
  `(a + b/2, (√3/2)b)`; 12 orientation classes; H1 is the only
  reflected hat; H-lineage counts 4, 25, 169, 1156, 7921, 54289;
  matching-rule labels A±, B±, X±, F±, L honored at every level (0
  violations); every level's hats a literal subset of the next.
- **Why per-view seeded generation:** one lineage blob's *solid*
  interior saturates — exact rim analysis shows permanent interstitial
  fjords near the seed (rim stabilizes at 40 edges from level 3 on).
  In the full plane tiling those fjords are filled by *neighboring*
  supertile hierarchies (the paper's Extension Theorem), which no
  single lineage materializes. So the generator seeds the blob per
  view: view center quantized to a 96-unit grid anchor, the blob
  translated up its open (1, −1) wedge at distance growing with the
  view's hexagonal radius (5 fallback offsets), depth escalated 3→6
  per candidate, acceptance = exact clipped-area equality against the
  view box (catches gaps *and* overlaps). Python sweep: 100 view boxes
  (half-widths to 200 world units, 9 centers) all covered.
- **Non-edge-to-edge:** 565 directed-side midpoint T-junction
  incidences at depth 3 (hat sides of lattice length 2 are two
  collinear unit edges whose midpoints carry neighboring hat
  vertices). Asserted, not fixed — same policy as chair/sphinx.
- **Implementation:** `hat.rs` (feature `hat`). `proto` always 0;
  `seed` = cluster role + 1 (H1..H4, T1, P1..P2, F1..F2 = 1..=9) for
  pattern mode. Views sharing an anchor cell + view-radius bucket see
  one fixed translated lineage — grown/zoomed views keep every visible
  tile bit-identical (nested patches); crossing an anchor cell re-seeds
  to another equally legal finite piece of the plane tiling.
- **Verification:** 14 Rust tests — outline geometry (all 12
  orientations; the raw shoelace sum is the doubled area 32H), cluster
  internal census, count tables (metatiles + hats, 5 levels × 4
  types), exact nesting, no-overlap + 565 T-junction census, matching
  rules on the integer grid, reflected-hat occurrence + all 9 roles,
  geometric inflation band, view stability (grow/pan/zoom), unit-hat
  emission, and the standard trio plus large/distant-view coverage.
- **Status:** committed (M34). 33 tilings, 190 tests.


### 13. Turtle — "hats in turtles" ✅ (M37)

- **Motif:** the two-prototile member of the einstein family (Smith,
  Myers, Kaplan, Goodman-Strauss, arXiv:2305.17743): the 14-sided
  **turtle** (8 sides of √3, 6 of 1; area exactly 10√3) as the
  dominant tile, plus the **hat** (area 8√3, with its length-2 side
  split into two collinear unit sides) as the minor tile. Aperiodic,
  **homochiral** (every emitted tile unreflected) and **edge-to-edge**;
  turtle:hat ratio → 4+√15 ≈ 7.873.
- **Geometry:** Kaplan's hexagonal lattice — lattice point `(x, y)` is
  world `(x + y/2, −(√3/2)y)` — so every tile placement is an exact
  integer D6 transform.
- **Substitution:** nine matching-rule labels (Γ…Ψ); a supertile is an
  8-ring of children glued by a key-point chain, then the whole ring
  is reflected (the per-level chirality flip of the paper). Label
  tables `T_RULES` / `SUPER_RULES` transcribed verbatim from Kaplan's
  reference implementation (spectre.js); Γ is the "Mystic" compound
  (one turtle at the identity + one hat rotated 60° and translated to
  lattice point (4, −5)).
- **Per-view generation:** a supertile never contains a copy of
  itself, so each view is one rigid depth-`d` patch of the Delta
  supertile, translated so its inscribed disk (radius from the
  exact-computed anchor table, re-verified by tests) contains the
  query; the translation snaps to the anchor grid (deterministic per
  view, re-seeds only on anchor-cell or depth crossings, like the
  hat's). Full-patch counts 9/71/559/4401/34649 at depths 1–5.
- **Verification:** Python exact-integer reference — all levels
  pose-identical to spectre.js at depths 0–4, census Euler χ = 1,
  zero bad edges, tile area == union area, chirality uniform per
  depth, growth ratio → 4+√15; Rust in-module outline/substitution/
  edge-census tests (incl. `hats_d = turtles_{d−1}` from the single
  Mystic child); standard coverage trio at depths 3–5.
- **Status:** committed (M37). 35 tilings, 214 tests at the commit.


### 14. Spectre — chiral einstein Tile(1,1) ✅ (M38)

- **Motif:** the chiral member of the einstein family (SMKG 2023): a
  single 14-gon prototile with **all 14 sides unit length** (the
  equal-edge Tile(1,1)), area exactly 3+3√3, tiling the plane
  aperiodically **without reflected copies** — the hat and turtle both
  need their mirrors; the spectre does not. Edge-to-edge.
- **Geometry:** unlike hat/turtle the vertices are not
  rational-lattice points — they live in Q(√3), stored as exact surds
  `((a + b√3)/2ⁿ, (c + d√3)/2ⁿ)`; every substitution frame is a D12
  transform (30° rotations, optional y-mirror, surd translation).
- **Substitution:** the same nine-label machinery as the turtle
  (spectre.js-identical `T_RULES` / `SUPER_RULES`; the build is
  shape-generic), from the spectre's quad points (vertices 3, 5, 7,
  11); Γ is the Mystic compound of **two spectres** (one rotated 30°
  about the origin and translated by spectre[8]), so ring children sit
  at 30°-multiple orientations → D12. Odd-depth patches are entirely
  reflected; the generator composes the global flip into the root
  placement so every emitted tile is unreflected.
- **Per-view generation:** identical scheme to the turtle (one rigid
  depth-`d` Delta patch, inscribed-disk coverage, 4-unit anchor grid);
  full-patch counts 1/9/71/559/4401/34649 (Delta) and 2/8/62/488/3842
  (Gamma); depth-5 inscribed radius ≈ 90.4 world units.
- **Verification:** Python exact-surd reference — vertex-set multisets
  match spectre.js at every depth 0–5 (34649/34649 tiles at depth 5),
  census χ = 1, zero bad edges, chirality uniform; Rust outline tests
  in all 24 D12 placements, Mystic leaf-count law
  `G(d+1)+G(d) = 2·total(d)`, edge census, anchor-table validation;
  standard coverage trio at depth 5. (Drive-by: dropped an unused
  `Query` import that warned under `cargo test`.)
- **Status:** committed (M38). 36 tilings, 224 tests at the commit.


### 36. Taylor–Socolar striped hexagons ✅ (M86; decoration reworked in M86-fix)

- **Motif:** the decorated-hexagon aperiodic monotile of Taylor &
  Socolar (arXiv:1008.3103): one hexagon carrying one decoration band,
  the tile state `(a, rbt, sdt)` — `a ∈ Z3` stripe-axis class, `rbt` which
  end of the polar vertex pair is red, `sdt` the stripe-offset bit —
  whose edge-midpoint handoff rule is
  `sigma(state, m) = SUM[sdt][(m − a) mod 6]`,
  `SUM[0] = (+,−,−,+,−,+)`, `SUM[1] = (+,−,+,+,−,−)`
  (rules R1–R3, re-derived in tests; the state table is unchanged by
  M86-fix, only the drawing of it). The shipped object
  is the paper's **forced 529-hexagon patch** (its Figure 18): the
  23×23 rhombus `{v ∈ [−21, 1], u ∈ [−(v+21), 22−(v+21)]}` in axial
  coordinates, covered domain = patch (girih precedent: `generate`
  returns exactly the tiles intersecting the query region, background
  beyond).
- **Decoration (canonical chord + split axis, as in the reference
  figures of arXiv:1504.06823 checked against `HLR_trap.jpg` /
  `Hex_tiling_3digit.jpg`):** with vertex `V_k` at bearing `30° + 60k`
  (`R = 1`) and `kappa = (a + 3·sdt) mod 6`, each hexagon carries the
  chord `V_{kappa+1} — V_{kappa−1}` as a straight band of width
  `W = 0.2·R`, plus the full diameter `V_kappa — V_{kappa+3}` split at
  the centre into two half-bands, the half toward vertex
  `t = a + 3·(1 − rbt)` red and the other blue (`t == V_kappa` exactly
  when `rbt ≠ sdt`). Chord ends sit on hexagon vertices (shared by three
  hexagons) and R1 pairs them there: 1058 chord ends → **505 interior
  degree-2 junctions (264 straight-through, 241 at 60° corners) + 48
  open boundary ends**, and the network **closes**: half-edge face
  enumeration gives 172 cycles whose 74 closed rings are **all**
  equilateral triangles — 60 of side `√3`, 12 of side `3√3`, 2 of side
  `7√3` (three nested scales, areas in the ratio 1 : 9 : 49; a larger
  triangle just carries chord vertices along its sides) — with 24 chains
  streaming out of the boundary (in tests).
- **Provenance (AP-STUDY handoff):** the state table, the neighbour
  table `DELTA`, the `SUM` vectors and the battery numbers are
  machine-transcribed from the study's `ap_ts_patch.json`; the
  decoration geometry comes from the study's exact-`Q(√3)` scratch
  generator (`m86fix_part2.py` → `m86fix_emit.py` emits every Rust
  coordinate plus the four reference placements from the exact model —
  zero hand-copied numbers, float-eval checked to 1e−9, worst delta
  0.0).
- **Partition mandate:** each hexagon is cut into **8 convex,
  non-overlapping pieces** — proto 1 the two chord-band halves (split by
  the axis line), protos 2/3 the red/blue axis half-bands, proto 0 the
  two body pentagons and two body triangles between them. All cuts are
  exact in `Q(√3)`; areas sum to the hexagon area `3√3/2` exactly, every
  piece is convex and CCW, and the coverage check's clipped-intersection
  accounting reports 0 overlaps. Geometry depends only on `kappa`, and
  `rbt ≠ sdt` flips which axis half is red → **12 templates**, indexed
  `kappa * 2 + flag`.
- **Colors:** chord band and red axis half → palette 2 (red), blue axis
  half → 1, body → 0, i.e. `seed_of(proto)`: every band colour is
  state-determined (this is what fixes the "mostly red but sometimes
  white, sometimes blue at the apexes" complaint). The palette has no
  black, so red stands in uniformly for the canonical black stripe.
  Geometry depends only on `(a, sdt)` through `kappa`; `rbt` is colour
  (and axis-half choice) only.
- **Verify (Rust, re-derived from the shipped table):** handoff
  battery reproduces the study's JSON numbers **R1 0/2992,
  R2 0/2816, R3 0/2904**; the chord network closes (junction/boundary
  census `(505, 48)` and bearing-delta census `(264, 241)`, no vertex
  carrying more than two chord ends); triangle census (172 cycles; the
  74 closed rings are all triangles — corner count 3 with 120° exterior
  angle, side-scale census 60/12/2 at 1·√3 / 3·√3 / 7·√3; 24 zero-area
  chains); whole-patch emission is exactly 529 × 8 pieces, each matched
  signature-by-signature against the recomputed placement, every
  hexagon's piece areas summing to `3√3/2`; no-period probe over the
  2790 nonzero translations of hex-radius ≤ 30 finds **no period**
  (overlap ≥ 3 sites); coverage trio inside upstream-verified
  exactly-once windows (default `(−14, −8.68, 14, 8.68)`,
  0.6-scaled/25°-rotated `(−9, −5.58, 9, 5.58)`); determinism; empty
  output beyond the patch; the runtime templates reproduce the emitter's
  four reference placements vertex-exact (1e−12).
- **Decoration corrected in M86-fix** to the canonical closed-triangle
  chord/split-axis structure after owner review against the
  arXiv:1504.06823 figures: the first attempt drew three straight stripe
  bands per hexagon (one long across edges `a+2`/`a+5` at `±sigma·t0`,
  two corner bands at `±(3/4 + t0/2)`, `t0 = 0.24`) from the Processing
  sketch's edge-crossing data, and that construction was **disproved**
  as unable to close triangles — under the handoff rules its chains only
  ever continue straight (the crossing bearing is preserved, and stubs
  are collinear dead ends), so its curve network has no closed polygon
  face; of all 1296 affine rules `kappa = (p·a + q·rbt + r·sdt + s) mod 6`,
  125 pair every interior chord end on this state table, and
  `kappa = a + 3·sdt` is the **only** one of those whose network closes
  anything (74 rings / 60 unit triangles; the other 124 close 0 rings —
  census artifact `m86fix_rulecensus.json`).
- **Scope honesty:** aperiodicity of the plane tiling, the two local-
  isomorphism classes and the limit-periodic structure are **cited,
  not proven** (beyond the radius-30 probe on the shipped patch); the
  inflation/deflation hierarchy is **not reproduced** (the study's
  algebraic inflation fit aligns only 57.6% of sites, so it is not
  even verified); "forced" is a local-rule statement — the study's
  solver extends the patch R9→R10 with zero backtracks under R1–R3;
  band width `W/R = 0.2` is a **compromise** of the reference figures
  (measured ≈ 0.19–0.21 in `HLR_trap.jpg`, ≈ 0.12 in
  `Hex_tiling_3digit.jpg`), and one decoration scale is drawn — no
  scale family claimed.
- **Registration:** unconditional module + registry entry (the
  `socolar` feature exists in Cargo.toml but is not a default feature
  and Cargo.toml is frozen — M74-style deviation, documented in
  `lib.rs`).
- **Status:** landed (M86), decoration reworked (M86-fix).
  75 tilings, 551 tests at this commit.


### 43. Jeandel–Rao 11-tile aperiodic Wang tiling ✅ (M93)

- **Motif:** the Jeandel–Rao Wang tile set (arXiv:1506.06492) — the
  smallest known aperiodic set of Wang tiles: 11 edge-coloured unit
  squares (A..K), no rotations or reflections. The 9 distinct edge
  colours are 5 on N/S edges (cyan, magenta, pink, white, yellow) and
  4 on E/W edges (blue, green, orange, red), all names distinct. The
  shipped object is the **author's canonical 100×100 patch**
  (`100.txt` from Jeandel's research page, cited by the handoff),
  transcribed with the handoff's symbol map (0–9 → A..J, a → K).
  Covered domain = the finite patch (Taylor–Socolar / girih precedent:
  `generate` returns exactly the pieces intersecting the query region,
  background beyond the patch boundary).
- **Provenance (AP-STUDY handoff):** the 100×100 tile table, the 11
  (N,E,S,W) quadruples, the frequency table and the battery numbers
  are machine-transcribed from the study's `ap_jr_patch.json` by
  `m93_emit.py` (zero hand-copied numbers; the emitter re-checks both
  batteries, the frequencies, the no-period probe and the partition
  geometry against the JSON before emitting). Two reference
  placements (cells (0,0) and (99,99)) are float-evaluated by the
  emitter and pinned by `jr_matches_generated_reference` to 1e−9.
- **Frame:** grid (x, y) with x = column right, y = row down (handoff
  convention); cell centre = `(x − 49.5, 49.5 − y)` (y flipped to the
  math y-up frame), so the 100×100 patch spans **[−50, 50]²**.
- **Edge-colour encoding (geometric edge bands):** each unit-square
  prototile is partitioned into **5 non-overlapping convex pieces** —
  protos 1–4 the N/E/S/W edge bands of width `W = 0.08` (the single
  documented scale), proto 0 the interior remainder. Each `W×W` corner
  square is split by its 45° miter diagonal between the two adjacent
  bands (each band owns the triangle of that corner). Band area
  `W − W² = 46/625`, interior area `(1 − 2W)² = 441/625`, so the five
  piece areas sum to `4·(46/625) + 441/625 = 1` **exactly in Q** for
  `W = 8/100` (re-derived per cell by `jr_partition_areas_sum_to_cell`
  on the emitted pieces). Each band is painted with its edge's
  palette colour, so the Wang matching is visible: touching bands
  carry the same palette colour exactly where the handoff colours
  agree.
- **Palette mapping:** the 9 distinct edge colours map onto the fixed
  8-colour tess-core palette by nearest colour — white→0 (light gray),
  blue→1, red→2, green→3, yellow→4, magenta→5, cyan→6, orange→7.
  **Magenta and pink both map to purple (5)** (one documented merge),
  and white maps to the light-gray base (0), which the interior also
  uses (border strokes keep the two visually distinct in Pattern
  mode). `TILE_COLORS` keeps the exact 9-colour ids, so the edge
  batteries are checked *before* the merge — a palette-id check alone
  would mask a magenta/pink swap.
- **Verification (all re-derived in Rust tests):** handoff battery on
  the shipped patch — **H 0/9900 and V 0/9900** edge mismatches on the
  exact 9-colour ids; tile frequencies match the handoff table (sum
  10000); no grid translation period |t| ≤ 30 — 2820 nonzero
  translations with dx²+dy² ≤ 900 probed, none invariant over an
  overlap of ≥ 3 cells (mirrors the study's
  `translation_periods_100x100_le30 = []`); partition exactness (5
  pieces/cell, areas sum to 1, full emission accounted for —
  clipped-area accounting catches overlap as well as gaps); the
  standard coverage trio (axis-aligned window, 0.6-scaled / 25°-
  rotated window — both verified exactly-once by the emitter on 4400
  samples — and determinism); empty output beyond the patch.
- **Rust:** `JeandelRaoTiling` in `jr_aperiodic.rs` (no feature gate
  exists in Cargo.toml, which the task leaves untouched — module and
  registration are unconditional, M86-style comment). id
  `jeandel-rao`, name "Jeandel-Rao 11-tile Wang tiles", category
  Aperiodic, similarity params (scale + rotation) only.
- **Scope honesty:** aperiodicity of the tile set and minimality of
  its subshift are **cited** (Jeandel–Rao, arXiv:1506.06492,
  machine-verified by the authors), not re-proved here — the radius-30
  no-period probe is evidence, not proof. **No generator is ported:**
  the study's independent DFS growth is a *proven dead end* — its
  grown 24×24 has 0 edge mismatches but no 25×25 extension (exhaustive
  DFS, 27 nodes), while the author's 24×24 does extend (226 nodes);
  locally-valid windows are therefore a strict superset of the JR
  tiling language, and the author's 100×100 patch is the only
  renderable artifact. The band width `W = 0.08` is a porting choice
  for the edge encoding, not a claim of the paper.
- **Status:** landed (M93).
  79 tilings, 91 memberships, 571 tests at this step (landed on the
  post-M90-M92 base).

---

*Map: [docs/INDEX.md](../INDEX.md) · Changelog: [docs/history/CHANGELOG.md](../history/CHANGELOG.md) · Queue: [docs/ROADMAP.md](../ROADMAP.md)*
