# Quasiperiodic family

Substitution-generated quasiperiodic tilings: the Penrose family, Ammann–Beenker, Stampfli 12-fold, the shield tiling, the fractalizing limit-periodic family, the golden A2 members and Pautze's generalized GLB binary n = 7. Each implements `Tiling` directly in its module under `families/quasiperiodic/`.

Code: `crates/tess-tilings/src/families/quasiperiodic/` — cargo feature `quasiperiodic` (default-on).
Adding a tiling here touches only this family: `families/quasiperiodic.rs`
(module decl, register slice, classification arms, census `IDS`,
`lattice_tilings` entries) plus the new module and its doc row below.
Work briefs: `docs/briefs/TEMPLATE.md`; fleet protocol: `tessellix/AGENTS.md`.

## Implemented (16 tilings)

### Quasiperiodic substitution families

| id | motif | notes |
|---|---|---|
| `penrose` | Penrose P3 (thick/thin rhombi) | Robinson-triangle substitution, φ-inflation, chirality-aware; sun-seeded deflation growth |
| `penrose-p1` | Penrose P1 (pentagon, star, boat, diamond) | exact φ² deflation: short-edge (1/φ) pieces merge across parent boundaries (G+G→D, CORE+2G→B, S2C+2G→S, S1C+G→S); single-seed pentagonal gasket with 5-fold notch channels; clean-disk radius R/φ² guarantees view coverage |
| `penrose_p2` | Penrose P2 (kites & darts) | glue-labelled half-tile substitution (A → 2A+O, O → A+O); halves pair along glue legs into kites/darts; sun/star seeds; unique labelling found by exhaustive search |
| `penrose-pentagon` | five pentagons (φ-substitution) | aperiodic set found by exhaustive pool search (strict-0 + 1-hole layers); M = [[0,1,0,1,0],[1,1,0,0,1],[2,0,0,0,3],[0,0,1,0,0],[1,1,0,0,1]] in order (50,105,451,468,484); single-seed deflation growth; non-edge-to-edge (φ-substitution children meet parent edges at t = 1/φ) |
| `ammann-beenker` | Ammann–Beenker A5 (octagonal aperiodic) | half-tile substitution (two rhombus halves + square) with per-level mirror flips; exact Q(√2) geometry; edge-to-edge; 8-fold finite patches |
| `ammann-beenker-tri` | Ammann–Beenker rhomb-triangle (A5 variant, 3 prototiles) | Bielefeld's "Ammann-Beenker rhomb triangle" entry: the A5 substitution's half-square triangles emitted as their own tile class instead of being paired back into squares — rhombus (proto 0) + R-triangle (proto 1) + L-triangle (proto 2), same silver-mean deflation as the classic entry (hypotenuse-pairing step removed); exact Q(√2), 8-rhombus star seed, edge-to-edge growth (no interior overlaps, hypotenuse groups ≤ 2 and R–L paired), PF-orbit counts (24,16,16) → (136,96,96) → (792,560,560) → (4616,3264,3264); 8-fold finite patches (M129) |
| `stampfli` | Stampfli 12-fold dodecagonal | squares + equilateral triangles + 30° rhombi; Stampfli substitution, inflation 2+√3 (Bielefeld encyclopedia rule); straddling unit children deduped by exact vertex set; 12-rhombus seed star, exact Z[√3]/4 anchors; edge-to-edge, T-junction-free, 12-fold symmetric at every generation |
| `fractalizing-3-12-12` | Fractalizing 3.12.12 (limit-periodic dodecagon-dense) | Chavey's fractalizing family over the uniform 3.12.12: same-plane levels, level d at child edge (2+√3)^{1−d} with level d+1 = level d shrunk by 1/(2+√3) about the origin flower; hexagonal flower orbit 44 T + 12 S + 1 H + 12 D per cell (69 tiles, cell 84 + 48.5√3 = the level-0 3.12.12 cell at edge 2+√3); all level-1 vertices on the exact (m + n√3)/6 grid; edge-to-edge closure verified in exact Q(√3) (0 overlaps, 0 T-junctions, exact single coverage) |
| `shield` | Shield tiling (Gähler, dodecagonal 12-fold) | shield hexagon + square + black/white triangles, each in two labelled orientations R/L — 4 prototiles / 8 substitution kinds carrying Gähler's matching rules; inflation θ = 2cos15° = (√6+√2)/2, PF Perron θ² = 2+√3; children tile kinked supertiles (±¼-unit-square notches); exact Q(√2,√3) geometry, all edges on the 12 dodecagonal directions; Bragg 12-fold (no local 30° center) |
| `golden-bee` | Golden bee / Ammann chair (A2, inflation √φ, irreptile) | the registry's first irreptile: concave hexagon (five right angles + one reflex), edge lengths {1, s, s⁻¹, s⁻², s⁻³, s⁻⁴} (s = √φ), area s − s⁻⁷; dissects into two similar copies of unequal size (scales φ^−1/2 and φ^−1; area law s⁻² + s⁻⁴ = 1); √φ-inflation substitution over 8 orientation classes (A0…A3 rotations, B0…B3 mirror family), every rule pinned exact in Q(√φ) (4-integer tuples, s⁴ = s² + 1; the girih-quasiperiodic fixed-point trick); non-edge-to-edge by intrinsic T-junctions at the exact s⁻⁴ edge fraction (13 at depth 3, 64 at depth 5, 1 − s⁻⁴ = s⁻²); growth 2^d tiles, Σ s^−2e = 1 exact at every depth; finite covered patch (seed chair = domain, Jeandel–Rao precedent) recentered on the patch bbox center, pinwheel-convention view-anchored depth (nested views bit-identical within one φ-octave); category Quasiperiodic (Vince, AMM 121 (2014), canonical sequence; Durand–Shen–Vereshchagin, DCG 63 (2020), structure of A2); proto = class, seed = size exponent (one hue per size class) (M104) |
| `golden-triangle` | Golden triangle (√−τ reptile, Danzer–van Ophuysen) | the right triangle (legs 1, √τ; hypotenuse τ) dissected by the altitude from its right angle into two similar children (scales τ^−1 and τ^−1/2, area law τ^−2 + τ^−1 = 1); complex √(−τ)-inflation viewpoint (λ = i√τ, λ² = −τ, λL = L ∪ S, λS = L) in exact K = Q(λ), λ⁴ = 1 − λ²; uniform rule foot F = P + τ^−2(Q − P) (τ^−2 = 2 − τ exact); canonical vertex order carries chirality (flips every cut), class = parity of −e; both mirror chiralities, dense rotations, T-junctions at the τ^−2 hypotenuse ratio; finite covered patch (seed triangle = domain, chair/girih precedent), depth = clamp(ceil(−ln(E/τ)/ln τ), 5, 17); category Quasiperiodic per the Danzer–van Ophuysen 2000 triangle-pair family (scope note in module docs) (M127) |
| `golden-rhomboid` | Golden rhomboid-triangle (Andritz, λ = φ) | the Bielefeld "golden-rhomboid-triangle": equilateral triangles side 1 (Y) and φ (O) plus the 60°/120° rhomboid 1×φ (R); Y→O, O→O+Y+R, R→R+2O (areas 1:φ²:2φ in √3/4 units, all identities exact); count matrix [[1,1,0],[2,1,1],[0,1,0]] in (R,O,Y) order, char poly λ³−2λ²−2λ+1 with Perron root φ² an EXACT Q(√5) root; exact 4-tuple Q(√5) arithmetic (point (a₀,a₁,b₀,b₁) ⇔ x=(a₀+a₁φ)/2, y=(b₀+b₁φ)√3/2, φ·(a₀,a₁)=(a₁,a₀+a₁)), floats only at emission; 18 anchored orientation classes (Y/O frames 120°-quotiented, 2+2+6 = 10 unanchored sets, the dissection's distinctness keys); single-seed unbounded growth — depth d covers φ^d·R₀ exactly, the union is the 60° wedge {y ≥ 0, y ≤ √3·x} (views outside are background); T-junctions intrinsic (first at depth 2, 2 points / 5 incidences), every T-point closes to exactly π on both sides in π/3 units; valid gap-free/overlap-free partition, NOT face-to-edge; Bielefeld patch.pdf cross-check 1525 Y : 4012 O : 2477 R (report only) |
| `glb-binary-n7` | Generalized GLB binary tiling, n = 7 (three 7-fold rhombi, acute angles π/7, 2π/7, 3π/7) | Pautze's generalized binary family at n = 7 (Hibma's GLB-binary companion): inflation λ = √(2+2cos(π/7)) = 2cos(π/14) ≈ 1.9499, non-Pisot; odd n gives (n−1)/2 = 3 prototiles; per-parent refinement is a TWO-CHOICE pool (shape 1 → {1} — rule 6 has no certified survivor frame, shape 2 → {5,2}, shape 3 → {3,4}) forced by compatible meshing at depth ≥ 2 — the pools are a derivation result beyond the published one-rule scheme, and the valid choice is context-dependent: eps-complementarity cascades through arbitrarily long adjacency chains, so no local (shape, class) map drives refinement (probed and refuted); certified generator = deterministic per-level DFS, all-pairs interior disjointness + eps-complementary bend bits on shared parent edges (compat ε_A + ε_B = 1), with the unculled parity battery pinning that spatial culling of the refinement frontier is UNSOUND; K₇-twist exact arithmetic — every vertex a twist pair (k₁, s₁k₂) ∈ K₇², K₇ = Q(b), b = 2cos(2π/7), dyadic denominators ≤ 2 (D = 2, depth-stable), float emission only, all vertices on the even π/7 class grid (6 488 vertices, 0 odd at the depth-4 certificate); fan-area law: every valid assembly carries total twist area 2λ^{2d} exact; unculled growth 3/10/35/126/462 (M^d row sums, M = [[2,1,0],[1,2,1],[0,1,3]], PF eigenpair λ²·v with v = (1, b²+b−1, b+1)); depth cap 5 = the deepest level constructed AND certified — σ²…σ⁵ pass the full staircase boundary certificate (single boundary cycle of 4·2^d even unit segments, consecutive pairings, closure on φ^{d−1}(B₀), exact area), while at depth 6 the candidate search exhausts: a legal patch need not admit a legal refinement; σ² census per fan pick: 16 two-choice combos, 9 disjoint-only, 4 meshed — disjointness alone NOT sufficient, eps-compatibility load-bearing; finite rotations + polytopal rhomb tiles (Bielefeld tags FLC / Finite Rotations / Polytopal Tiles / Rhomb Tiles), Pautze notes no Dₙ-symmetric patches, no pure-point claim; refs [Pau2017] Symmetry 9(2), doi.org/10.3390/sym9020019, [Hib2015] arxiv.org/abs/1509.02053; plane-filling presentation via nested fan hierarchy (M149 follow-up): every query window covered exactly once at any scale/rotation/pan — the seed-rhombus lattice of the certified seed parent over φ⁻¹(W+C₀), the same per-level DFS meshes cell seams by eps-complementarity, depth 3–5 from a per-view emitted-tile budget, culling at emission only (plane coverage census + cross-boundary mesh audit + clipped-area identity in-test); M149 shipped the certified single-domain core |
| `fibonacci-fibonacci` | Fibonacci × Fibonacci (Bielefeld product substitution; 3 shapes: φ×φ square, φ×1 / 1×φ rectangle, unit square) | the Bielefeld "fibonacci-fibonacci" entry: the 1-D Fibonacci substitution taken as a product with itself — 4 orientation classes (LL, LS, SL, SS) on 3 prototile shapes (LS/SL are one shape at two rotations); linear inflation λ = τ² = φ², module rule = two product steps (children tile the INFLATED parent, anchors scaled by λ); count matrix B = A₄² = [[4,2,2,1],[2,2,1,1],[2,1,2,1],[1,1,1,1]] — the square of the 1-D product matrix A₄ = [[1,1,1,1],[1,0,1,0],[1,1,0,0],[1,0,0,0]] pinned from the entry's rule figure; char poly (x−1)²(x²−7x+1), PF eigenvalue τ⁴ = λ², area vector (φ², φ, φ, 1) obeys the per-parent area law child = parent·τ⁴ exactly; strict PIP + proper-crossing + T-junction checks at every growth step; exact Q(√5) integer pairs end to end, floats only at emission; finite patch recentered −φ⁷/2 (depth-3 floor, patch frame centred on the origin), depth cap 6 (totals F_{2d+2}²: 1, 9, 64, 441, 3025, 20 736, 142 129 ≤ MAX_TILES = 250 000); LL rule locally 90°-symmetric (M154) — the corner-grown depth-3 patch itself is not 4-fold symmetric (SymmetryMeta P1); diffraction cited, not proven (golden-family convention) |
| `wis-12-fold` | WIS 12-fold (Watanabe–Ito–Soma, Bielefeld) | the Bielefeld "WIS 12-fold" entry (Watanabe, Ito & Soma, Acta Cryst. A 51:936, 1995): equilateral triangle + square + 30°/150° rhombus, dodecagonal symmetry, inflation λ = 2+√3 (λ² = 7+4√3); the ENTRY FIGURE's published rules (WSI95 Fig. 2(iii) — the entry's non-unique ruleset), certified level-1: every dissection a closed simple boundary with all interior edges paired, exact ℤ[√3] area identity and λ² area law, parent ⊆ union (exact Q⁴ containment). **Caveat: NOT the Bielefeld patch's undocumented ruleset** — the figure's clusters have zero occurrences in the 792 k-node patch (tri 0/150 certified leaves, rho topologically distinct, sq 0 in the crop) and the patch is non-generic (no census oracle); the "correct set" attributed to Musesti–Paolini (2023) has no machine-readable record. Non-conformal same-scale rules: children are prototiles at edge 2H (same size as the parent) in a jagged cluster whose union has area λ²·parent and CONTAINS the parent; the rho rule is a 3-piece disconnected cluster (9+2+2 tiles; fitted lattice-snap offsets island (−2,−1,−3,−2) / pendant (2,1,1,0), inter-piece gaps artist-chosen); pieces share no edges and touch at exactly two pinches, each a 4-tile vertex with corner sum 8 (a 120° gap). Exact ℤ[√3] 4-tuple points (x = H(P+Q√3), y = H(R+S√3), H = s/2), floats only at emission; the rules do NOT compose at depth ≥ 2 (pinned D2 drift-guard: tri seed 155 tiles / 770 overlap pairs, sq 358 / 3660, rho 179 / 1023, same-parent 0), so the emission is the depth-1 single-parent square supertile (26 tiles, simply connected) as a finite patch — views outside the patch are background (patch cross-check, report only: Bielefeld patch.pdf 18 302 tri : 11 644 sq : 7 689 rho) |
| `girih-14-fold` | Girih-inspired 14-fold (Pautze; 11 prototiles) | the Bielefeld "Girih 14-fold" entry ([Pau2017] S. Pautze, Symmetry 9(2):19, 2017): rhombi R2/R3, hexagons H26/H44/H45, octagons O29 (concave)/O36, decagons D2 (concave)/D46/D56 and the regular 14-gon G14; inflation λ = 2b²+3b (b = 2cos(2π/7), λ ≈ 6.850855, λ² ≈ 46.934215, **Norm(λ) = −7 — NOT a unit**; the entry's unit claim is false). Exact ½Z[ζ₁₄] geometry as twist pairs (P,Q) ∈ K₇², z = val(P) + i·sin(π/7)·val(Q); certified rule tables (763 entries) rebuilt and pinned entry-by-entry against the brief JSON in tests: per class nv corner 14-gons at the parent vertices + nv edge decagons centered on the edge midpoints (mirror-symmetric across the edge, bit-identical across incident parents) + interior kids; the effective kid matrix N′ (edge kids ×½, corner 14-gon wedges k_i/14) satisfies the exact K₇ area law Σ N′·S = λ²·S on all 11 rows with PF eigenvalue λ²; one full 11-panel round = 849 kids. Generation: centered G14 seed (convex — the view bbox fits iff its four corners fit), per-level substitute/cull/exact-vertex-set-dedup downward from level N; straddling corner/edge kids dedup by placement; the Bielefeld patch specimen is non-generic (O36 absent) and never used as a census oracle; [Pau2017] Table 7's n = 7 factor ≈ 5.801939 is contradicted by the paper's own Figs 7.4/7.5 (measured 6.8507 ± 0.0006 = λ) |

## Build arcs

Numbered arcs keep their PATTERNS.MD §numbers — code comments and
briefs cite them as `§N`.

### 3. Ammann–Beenker A5 (octagonal aperiodic) ✅ (M27)

- **Motif:** square + 45°-rhombus, unit edges; perfect 8-fold patches
  (the SVG export shows the canonical stars, double-square bars and
  arrow motifs).
- **Inflation:** λ = 1+√2 (silver mean); square → 3 squares + 4 rhombi,
  rhombus → 2 squares + 3 rhombi (PF matrix [[3,2],[4,3]], eigenvalue
  λ² = 3+2√2; counts area-forced, re-checked by a Rust growth test).
- **Rule (the decisive find):** work in the half-square view — the
  published recipe (geometricolor's AB implementation, 8-direction
  integer coordinates) uses THREE prototiles: the rhombus plus two
  half-square labels `RSquare`/`LSquare` that have the *same geometric
  shape* (45-45-90) but different substitution rules; every interior
  square diagonal pairs one R half with one L half, and that labelling
  is what makes the rule consistent across parent boundaries. An
  earlier armchair decoding that treated L as a *mirrored shape* produced
  per-parent-valid but globally inconsistent patches (T-junctions from
  depth 2) — the armchair ansatz was abandoned for the verbatim source
  tables, after which growth verified clean to depth 4.
- **Tables:** 7/5/5 children, anchors as `(m + n√2)/2` in the λ-inflated
  parent frame; the deflation view divides by λ, exact on such
  coordinates via `(m, n) → (2n−m, m−n)`. Boundary partitions: legs
  {1, √2} (unit piece at the acute/45° corner), half-square hypotenuse
  {1, √2, 1} (√2 centred).
- **Implementation:** `ammann_beenker.rs` (feature `ammann_beenker`),
  mirroring penrose.rs: seed = 8-rhombus star (inradius √2/2·scale,
  scale = λⁿ covering the view), repeated deflation with per-step region
  culling (children never leave their parent), pair halves into squares
  on the 1e-7 hypotenuse grid at the end (lone boundary halves emitted
  as triangles). STEP_CAP 20: λ²⁰ ≈ 4.5e7 keeps accumulated float error
  ~7e-9, comfortably inside half a key-grid step (5e-8).
- **Verification:** exact-Q(√2) Python battery (`ab_verify.py`): per-parent
  area/coverage/sub-edge parity, rhombus 180° symmetry, star growth to
  depth 4 edge-to-edge (zero T-junctions), R/L hyp pairing, PF counts
  (792/560, 4616/3264), sampled coverage. Plus a mechanical
  Python↔Rust table diff (`ab_table_diff.py`) that decoded the Rust
  `Child` consts back to Q(√2) points — **it caught a (M,N)-packing slip
  in six table entries** (rational/√2 parts swapped per component), the
  second time transcription diffing caught a real bug before the tests
  did. Nine Rust tests: 3 substitution-tiles-parent, growth counts /
  edge-to-edge / 8-fold symmetry, and the 3 standard battery tests.


### 4. Penrose P2 — kites & darts ✅ (M29)

- **Motif:** the Penrose kite (72,72,72,144°, sides φ,1,1,φ) and dart
  (36,36,72,216°, sides φ,1,1,φ); φ-deflation kite → 2 kites + 1 dart,
  dart → 1 kite + 1 dart (PF law K′ = 2K + D, D′ = K + D; eigenvalue φ²).
- **Rule (the decisive find, mirroring the AB half-square lesson):** work
  on *glue-labelled half-tiles* — `A` (kite half, 36-72-72, legs φ,
  base 1) and `O` (dart half, 36-108-36, sides 1/φ from the apex) — with
  substitution `A → 2A + O`, `O → A + O`. Each half carries a designated
  *glue leg* (an A leg of length φ, resp. an O leg of length 1); two
  halves sharing a glue edge merge into a kite (A+A) or dart (O+O), and
  half-darts pair **across** parent boundaries — which is why the rules
  are applied to whole patches seeded with the sun (5 kites) or star
  (5 darts), never to single tiles (the classical Wikipedia caveat).
  Two coupled binary choices per child — *frame order* (which leg sits
  in the canonical g slot → grandchild placement) and *glue edge*
  (which leg pairs now) — give 4 A-subdivisions × 4³·4² labellings × 2
  O-subdivisions = 8192 candidate systems.
- **Search:** exhaustive over all 8192 (script `p2_search4.py`), tiered
  pruning: sun g1 → sun g2 + star g1 → sun d3 + star d3 (area, edge
  census ≤ 2, zero T-junctions, interior-lone-glue, census law, sampled
  coverage) → sun/star d5. **Exactly two systems survive every tier**;
  they share the same A-rule and differ only in the O-parent
  subdivision (both exact-valid; either implements P2). Survivor 0 was
  transcribed to Rust.
- **Verification:** exact-arithmetic battery over K = Q(φ, sin 36°)
  (`p2_exact.py`, basis {1, φ, s, φ·s} with exact sign tests): every
  rule child exactly tiles the φ-inflated parent (exact area identity,
  pairwise interior-disjoint, children in the convex parent — a complete
  proof, no sampling), and exact growth to g4 on both seeds: exact glue
  keys, exact merged side-square patterns (φ²,1,1,φ²)·s², exact edge
  census ≤ 2, exact T-junction absence, lone glue edges exactly on the
  boundary, census law in integers, patch area exactly 5φ²·s (sun) /
  5φ·s (star) per generation. Float growth verified to g6 both seeds.
  A mechanical Python↔Rust table diff (`p2_table_diff.py`) confirmed the
  Rust consts float-for-float. Rust tests: 2 substitution-tiles-parent,
  sun/star census law to depth 3, refined-patch structural checks
  (glue pairing sides, area, edge census, T-junctions, lone-glue
  boundary), merged-tile kite/dart shapes, and the standard battery
  (covers_region, rotated_scaled, deterministic) plus a star-seed
  coverage test.
- **Implementation:** `penrose_p2.rs` (feature `penrose_p2`), mirroring
  penrose.rs: `Half::frame` is a general affine map (handles mirrored
  parents), rule consts in the φ-inflated parent frame divided by φ on
  application, per-step region culling (children never leave their
  parent), glue pairing on the 1e-7 grid at the end (lone boundary
  halves emitted as triangles, protos: kite 0, dart 1). Sun seed
  default; `star seed` bool param selects the star.


> Follow-up: penrose P1 + einstein family — ✅ complete (M31, M37, M38)
>
> (Committed between M34 and M35 — numbering follows the milestone
> labels, which are out of chronological order.)


### 12. Penrose P1 (pentagon, star, boat, diamond) ✅ (M31)

- **Motif:** the classic P1 set — four prototiles of edge 1, every
  angle a multiple of 36°: pentagon `P` (area 1.72048), 10-point star
  `S` (2.12663), boat `B` (1.35721), 36° diamond `D` (sin 36° =
  0.58779). Same φ-inflation family as the implemented P3 (`penrose`)
  and P2 (`penrose_p2`) — and distinct from `penrose-pentagon`, which
  is a five-pentagon pool-search set, not P1.
- **Construction:** exact φ² deflation with cross-boundary merges.
  Each parent disassembles into short-edge (1/φ) pieces (P → central
  P + 5 corner P + 5 flower gnomons; S → S0 + 5 star-anchored P + 5
  boat cores; B → 3 PA + 3 CORE + star-minus-tip; D → S2C + PA +
  CORE), and short edges glue *across* parent boundaries by four
  rules: `G + G → D`, `CORE + 2G → B`, `S2C + 2G → S`, `S1C + G → S`;
  unmatched short edges are discarded at the patch boundary. Children
  tile their parents exactly, so overlaps are structurally impossible.
- **Generation window:** a single pentagon seed of edge φ^(2m) (m ≤ 6,
  patch < 72k tiles) deflated m times so the guaranteed-clean disk of
  radius R·φ^(2m−2) covers the query — a pentagonal *gasket* with five
  5-fold notch channels (the discarded first-level gnomons) opening at
  the boundary; the interior disk is tiled exactly. Bulk ratios of
  this window (≈ 23 : 1 : 1.9 : 4.5, P:S:B:D) differ honestly from
  full-plane reference patches (≈ 17.4 : 1 : 2.2 : 3.5) even though
  the local substitution agrees (documented in the module header).
- **Verification:** exact φ-arithmetic child placements (area
  conservation per substitution), merge-rule exhaustiveness, no
  duplicate or overlapping children, clean-disk coverage; standard
  trio + SVG proto visual check (4 classes, 5-fold rosettes).
- **Status:** committed (M31). 34 tilings, 204 tests at the commit.


> Follow-up: Stampfli 12-fold — ✅ complete (M45)
>


### 15. Stampfli 12-fold dodecagonal ✅ (M45)

- **What it is:** the classic dodecagonal quasiperiodic tiling by
  squares, equilateral triangles and 30° rhombi with locally repeated
  12-fold rosettes (Stampfli & Schaad, Bridges 2021); completes the
  repo's rotation series Penrose 5 / Ammann–Beenker 8 / girih 10 /
  Stampfli 12.
- **Rule:** patch substitution with inflation λ = 2+√3, transcribed
  from the Bielefeld Tilings Encyclopedia rule figure
  (`staempflis-12-fold-1/rule.svg`, canonical R01 variants): square →
  1 square + 12 rhombi + 20 triangles, rhombus → 2 squares + 3 rhombi
  + 12 triangles, triangle → 6 rhombi + 10 triangles (unit children;
  every parent edge partitions 1 : √3/2 : √3/2 : 1 = λ). Triangles
  straddle parent edges with exactly one vertex outside; adjacent
  parents generate the same straddler, so children are deduped by
  exact sorted vertex set.
- **Construction** (`stampfli.rs`, feature `stampfli`): seeds the
  12-rhombus star (inradius 1/2·scale) at scale λ^n and deflates to
  unit pieces with per-step culling (descendant cone ≤ 0.32·s bound);
  anchors stay exact in Z[√3]/4 under all 12 rotations — no floating
  point in the tiling code.
- **Verification:** exact-Q(√3) Python battery first (per-parent cover
  exactly once + edge partitions; gen 1–4 growth: zero overlaps, zero
  T-junctions, no edge used >2×, exact 12-fold symmetry, PF area law
  per parent); Python↔Rust rule-table diff exact; Rust census rows
  S′ = S+2R, R′ = 12S+3R+6T for the star at all depths (gen 1–4
  counts (24,36,120)/(96,1116,1608)/(2328,14148,22104)/(30624,203004,
  306696)), Perron eigenvalue λ² = 7+4√3; standard trio.
- **Status:** implemented (M45).

### 19. Shield tiling — Gähler, dodecagonal ✅ (M51)

- **What it is:** the dodecagonal (12-fold) substitution tiling of
  Gähler (1988; popularized by Baake & Schmid's dodecagonal matching
  rules): a "shield" hexagon, a unit square, and two unit equilateral
  triangles ("black" and "white"), each carrying an R/L matching label —
  8 labelled kinds collapsing to 4 output prototiles. Together with
  Stampfli 12-fold it gives the repo a second, matching-rule-based
  dodecagonal quasiperiodic.
- **Rule:** inflation θ = 2cos 15° = (√6+√2)/2, θ² = 2+√3 (Perron root
  of the 8×8 count matrix; char poly divisible by x²−4x+1). Children
  (scale s/θ) do **not** tile the plain θ-parent: they tile a *kinked*
  supertile — the θ-parent with ±¼-unit-square triangular notches
  (shield 6 in, square 4 in, black triangle 3 out, white triangle
  2 out + 1 in). Kind counts per parent: shield → 13, square → 5,
  black tri → 1 shield, white tri → 3.
- **Construction** (`shield.rs`, feature `shield`): exact Q(√2,√3)
  anchors (dodecagonal 6-vectors via the recipe's `dod2orth`); seeds
  with the 13-tile kinked-shield supertile at θ^n (area centroid exact
  at √2/2 + √6/3, covering a disk of inradius (1+√3)/2 per scale) and
  deflates with per-step culling (bound sin15°/(θ−1) ≈ 0.2778).
- **Verification:** exact Python battery in Q(√2,√3) — per-kind
  children pairwise interior-disjoint, closed unit-edge boundary equal
  to the pinned kinked polygon, Σ areas = θ²·plain ± net-kink/4; PF
  Perron 2+√3 (exact power iteration); growth totals 13→31→151→493→1981
  and the published 10-tile seed sequence 10, 28, 112, 400, 1528, 5632,
  21160, 78688, 294232 reproduced from the count matrix; all edges on
  the 12 dodecagonal directions; honest negative: no local 30° rotation
  center (tile angles ≥ 60°; probe over 493-tile patch finds none) —
  the 12-fold character is Bragg/MLD, matching window a regular
  dodecagon. Rust↔Python float diff ≤ 4e-15 through depth 4; Rust
  battery: 8 per-kind tiling tests + coverage-once through depth 3 +
  standard trio + proto census (16 + 4 tests).
- **Status:** implemented (M51). Follow-up: the Socolar wheel tiling
  (MLD-dual of the shield) remains proposed.


### 64. Golden bee / Ammann chair (Ammann A2, inflation √φ) ✅ (M104)

> Numbering note: renumbered from the fork's provisional §62 to
> §64 by landing order (§63 = hankin-3-3-3-4-4, M107; this landing
> followed M107's).

- **Motif:** the registry's first **irreptile** — Ammann's A2 "chair": a
  concave hexagon with five right angles and one reflex angle, six edge
  lengths {1, s, s⁻¹, s⁻², s⁻³, s⁻⁴} (s = √φ), area s − s⁻⁷. It dissects
  into exactly **two similar copies of unequal size** — a large child at
  scale s⁻¹ = φ^−1/2 (area fraction 1/φ) and a small child at scale
  s⁻² = φ⁻¹ (area fraction 1/φ²), area law s⁻² + s⁻⁴ = 1. Ammann's
  aperiodic set A2; the only polygon besides non-isosceles right
  triangles that partitions into a non-congruent similar pair (Vince,
  "Self-Similar Polygonal Tilings", Amer. Math. Monthly 121 (2014));
  A2-tiling structure: Durand–Shen–Vereshchagin, Discrete Comput. Geom.
  63 (2020).
- **Construction:** √φ-inflation substitution over 8 orientation classes
  (A0…A3 = rotations of the unit chair by 90°·k, B0…B3 = the mirror
  family); each parent dissects into L (scale s⁻¹) + S (scale s⁻²) at
  pinned exact offsets (module rule table). 8/8 dissections verified
  exact by open-slab even-odd sweeps; growth census d0…d5 (2^d tiles;
  class parity even {A0,A2,B1,B3} / odd {A1,A3,B0,B2} at 2^(d−2); size
  histograms; Σ s^−2e = 1) all asserted as exact K equalities.
- **Exact arithmetic:** all geometry in K = Q(√φ) — integer 4-tuples
  (a0,a1,a2,a3) = a0 + a1·s + a2·s² + a3·s³ with s² = φ and reduction
  s⁴ = s² + 1 (the same quartic-field fixed-point trick as
  girih-quasiperiodic). The 8 unit shapes (edge multiset, 2·area =
  (0,−8,0,6), turn census 5 right + 1 reflex, bbox 1×s/s×1), all child
  anchors, all growth invariants, and the junction ratios are exact K
  assertions; f64 only for the real embedding that orders K (distinct
  elements separate by > 1e-9, asserted at each comparison) and at
  emission.
- **Anchoring:** finite-patch model (Jeandel–Rao
  precedent) — the seed is a single unit A0 chair and the patch *is* the
  covered domain; cell space recentered on the patch bbox center via a
  fixed float offset (−1/2, −s/2) (1/2 ∉ Z[√φ] — documented, not
  lattice-exact). Depth is view-anchored on the patch (pinwheel
  convention re-based): E = cell-space view diagonal, zoom = 0 for
  E ≥ φ (the patch diagonal √(1 + s²)) else ceil(−ln(E/φ) / ln φ), then
  d = max(D_PRESENT, zoom), capped at MAX_DEPTH = 17 (2^17 ≤ tile
  budget), D_PRESENT = 5 — whole-patch views floor at 5 (d = 0 would
  render the bounded patch as the lone seed chair; the UI preview
  window is derived from the generated tiles and would pin an absolute
  rule at d = 0), and zoomed-in views refine past the floor by
  φ-octaves. Nested views within one φ-octave emit bit-identical tiles
  on their intersection (pinned).
- **T-junctions:** not edge-to-edge, intrinsically — a child's reflex
  (notch) corner lands in the relative interior of its sibling's long
  edge at exactly the s⁻⁴ fraction of that edge (s⁻² from the other end,
  since 1 − s⁻⁴ = s⁻²). Per-line slab coverage depth ∈ {0,1,2} (1 iff on
  the patch boundary); junction census 13 (depth 3) / 64 (depth 5),
  every junction at the exact ratio, pinned.
- **Rust:** `GoldenBeeTiling` in `golden_bee.rs` (feature
  `golden_bee`, default-on), registered after `stampfli`; category
  Quasiperiodic — this module implements exactly Vince's canonical
  sequence, and the header states the scope honestly: the battery
  verifies substitution mechanics (two-piece dissections, area law,
  size hierarchy, junction ratios), not the diffraction limit; the
  quasiperiodicity claim rests on the cited theorem (chair precedent).
  `proto` = orientation class (0…7), `seed` = size exponent e (width
  s^−e), so the default Pattern coloring paints one hue per size class.
- **Tests:** 8 module tests — (1) 8-shape exact battery (SINV/SPOWS
  power-table cross-checks vs independent floats; edge multiset
  {s⁻⁴…s}; exact 2·area; turn census 5+1; bbox origin + 1×s/s×1);
  (2) all 8 two-piece dissections exact (area law, slab union == parent,
  child disjointness, on-boundary ⟺ depth 1, junction ratios); (3)
  growth census d0…d5 (counts, class censuses, size histograms, exact
  area law) + depth-6 count; (4) global junction census 13/64 at the
  exact s-ratio; (5) nested-view bit-identity (5 windows, one φ-octave,
  depth pinned by size classes e ∈ 3..=6); (6–8) standard trio
  (covers_region / rotated_scaled / deterministic) on windows inside the
  seed chair.
- **Symmetry:** P1, no lattice (finite patch; no symmetry claimed).
- **Census:** 108 → 109 tilings, 120 → 121 memberships (Quasiperiodic
  8 → 9), 721 → 729 tests (landed on main after the M107 landing; the
  fork's step numbers were on its own base 91dd843).
- **Integration fix (default view):** the shared `SIMILARITY_PARAMS`
  scale default 1.0 rendered the ~1.3-cell-unit patch as a tiny
  centered chair at depth 0 (pattern index 0, light gray) — reported
  as a broken-looking preview/render pane. First fix (scale default
  7.0) addressed only the CLI's fixed `20/scale` pane; the owner
  reported the lone chair persisting in the UI, and the root cause
  turned out to be the depth rule itself: the UI preview window is
  derived from the generated tiles (`tess_core::app::preview_region`:
  ±10 probe → 9× mean tile side), an anchor-self-canceling loop that
  pins an absolute "E ≥ 1 → d = 0" rule at depth 0 for EVERY scale —
  and for a bounded patch, d = 0 IS the whole tiling (one gray
  chair). Rule re-anchored on the patch: zoom octaves measured
  relative to the patch bbox diagonal (exactly φ = √(1 + s²)),
  floored at a presentation depth `D_PRESENT = 5` for whole-patch
  views (the fork's pinwheel-convention `E ≥ 1` transplant is the
  zoom term, re-based; zoomed-in views refine past the floor by
  φ-octaves unchanged). The scale default is now **3.5** (frames the
  whole patch in the CLI's `20/scale` pane at ≈ 78% height). Verified
  on both conventions at the default params: the App-derived preview
  frames all 2^5 = 32 chairs at ≈ 73% of the pane height, six size
  classes with the exact binomial counts (1, 5, 10, 10, 5, 1) =
  C(5,k); the CLI pane at 77.9%; zero non-finite, fill/outline
  geometry identical. Pinned by `default_view_fills_pane` (both
  conventions, incl. the probe fixed-point round). Mechanics tests
  stay pinned at explicit scale 1.0.
- **Status:** landed (M104). Full bar green: cargo check + clippy
  (both target sets) zero warnings, cargo test --workspace
  --all-features 729/729.


### 65. Fractalizing 3.12.12 — limit-periodic dodecagon-dense family ✅ (M110)

- **Numbering note:** renumbered from the fork's provisional §64 to
  §65 by landing order (§64 = golden bee, M104, which landed first).
- **Motif:** Chavey's *fractalizing* dodecagon-dense family (Tilings by
  regular polygons III: dodecagon-dense tilings, Symmetry 25(3), 2014;
  the Wikipedia "fractalizing k-uniform tilings" entries), reconstructed
  from the ground-truth figure `Planar Fractalizing Truncated Hexagonal
  Tiling II.svg`. The levels are NOT one substitution: each level is a
  tiling of the SAME plane, level d at child edge p^{1−d} for
  p = 2+√3, with the level-(d+1) plane exactly the level-d plane shrunk
  by 1/p about the origin flower. A new period per level makes the
  family limit-periodic rather than substitution-self-similar; every
  single level is 6-fold periodic. Classified Quasiperiodic on the
  Stampfli rationale (the limit carries the aperiodic flavor), with the
  limit-periodic nuance declared in the module header, the meta note
  and every doc. The handoff's σ-patch substitution model
  (tfract_svg ESCALATE) is disproven by this reconstruction — its
  per-ring patch rules cannot produce the figure's edge-to-edge flower.
- **Construction (level 1, child edge 1):** the hexagonal orbit of one
  57-tile flower unit — motif (1 H at the lattice point + 2 S at
  (1+√3)/2 and (5+3√3)/2 along the 0° ray + 7 T with centers at
  inradius √3/6 beyond their source edge) rotated by 60° steps,
  deduped to 55 tiles, plus 2 seam triangles — on the cell basis
  ⟨p²(√3/2, 1/2), p²(√3/2, −1/2)⟩ (|det| = 84 + 48.5√3), plus 12
  dodecagon classes per cell at exact reduced coordinates
  DM3 ∈ (Z[√3]/3)², one orientation class (vertices at 15° + 30k°,
  apothem p/2 = cot 15°/2). Per-cell census 44 T + 12 S + 1 H + 12 D
  = 69 tiles, forced by the area identity
  44(√3/4) + 12·1 + 3√3/2 + 12(6+3√3) = 84 + 48.5√3 = |det|.
- **Exact arithmetic:** every level-1 vertex lies on the
  (m + n√3)/6 grid — the 57-unit table, the DSHAPE and the 12 DM3
  classes are integer tables; cell anchors and dodecagon centers
  (m = 3i + u, n = 3j + v; x = (m+n)(36 + 21√3)/18,
  y = (m−n)(21 + 12√3)/18) stay in integer Z[√3]; depth d evaluates
  ((m + n√3)/6) · p^{1−d} with the single float multiply last, so no
  drift accumulates across levels. Depth selection d = 1 +
  floor(log_p(6/diag)) for view diagonal < 6, capped at 16 (child
  edge p^{-15} ≈ 2.6e-9 — deeper views fail with a message, the
  float horizon, mirroring Stampfli's STEP_CAP).
- **Level-0 seed:** the uniform 3.12.12 at edge p (the shipped
  archimedean cell table: dodecagons on the lattice
  p(1.5+√3, 1+√3/2), p(0, 2+√3) plus the E and NE triangle orbits);
  its cell area (1.5+√3)(2+√3)·p² = 84 + 48.5√3 equals the level-1
  flower cell exactly — the fixed point the family starts from.
- **Validation (exact Q(√3) battery, Python 40-dp, mirrored by an
  i128 Rust test):** over a 5×5-cell level-1 block (1725 = 25×69
  tiles): 0 pairwise interior overlaps, no edge used more than twice,
  0 T-junctions, 2500 exact rational sample points in the central
  2×2-cell window each covered exactly once (0 gaps / 0 overlaps),
  per-cell census 44/12/1/12 for all 9 central cells, all 300
  dodecagon edges with squared length exactly 1. The module's
  `exact_closure_level1` re-runs the battery on a 3×3-cell block in
  i128; `nested_view_identity` pins
  render(d+1, scale p, V) == render(d, scale 1, V) to 1e-9 (the
  nesting law as the user sees it), `growth_p2_and_nested_count_invariance`
  pins the cell-span p-law and exact nested-window count invariance.
- **Census:** 108 → 109 tilings, 120 → 121 memberships (Quasiperiodic
  8 → 9), 721 → 730 tests.
- **Status:** landed (M110, fork-local numbering — renumbered at
  integration). Full bar green: cargo check + clippy (both target
  sets) zero warnings, cargo test --workspace --all-features
  730/730.

### 66. Penrose pentagon (five-shape φ-substitution) ✅ (M28)

- **Motif:** the five-pentagon set of the Penrose/Robinson family — aperiodic set found by exhaustive pool search (strict-0 + 1-hole layers) over the pentagon pool (`penrose-pentagon`, family quasiperiodic; M28).
- **Key facts:** M = [[0,1,0,1,0],[1,1,0,0,1],[2,0,0,0,3],[0,0,1,0,0],[1,1,0,0,1]] in the order (50,105,451,468,484) — one φ-substitution rule for the five shapes, single-seed deflation growth; non-edge-to-edge (φ-substitution children meet parent edges at t = 1/φ); 29 tilings, 141 tests (see CHANGELOG M28).

### 67. Golden triangle (√−τ reptile) ✅ (M127)

- **Motif:** the golden triangle — Danzer–van Ophuysen's triangle-pair family: the right triangle (legs 1, √τ; hypotenuse τ) dissected by the altitude from its right angle into two similar children (scales τ^−1 and τ^−1/2, area law τ^−2 + τ^−1 = 1) (`golden-triangle`, family quasiperiodic; M127).
- **Key facts:** complex √(−τ)-inflation viewpoint (λ = i√τ, λ² = −τ, λL = L ∪ S, λS = L) in exact K = Q(λ), λ⁴ = 1 − λ², embedding (a + bλ + cλ² + dλ³) → (a − cτ) + i√τ(b − dτ); uniform rule foot F = P + τ^−2(Q − P) (τ^−2 = 2 − τ exact); chirality carried by the canonical vertex order (flips every cut), class = parity of −e; finite covered patch (seed L0), depth = clamp(ceil(−ln(E/τ)/ln τ), 5, 17), scale default 3.5; categorized Quasiperiodic on the Danzer–van Ophuysen 2000 triangle-pair family (see CHANGELOG M127).

### 68. Ammann–Beenker rhomb-triangle ✅ (M129)

- **Motif:** Bielefeld's "Ammann-Beenker rhomb triangle" entry (PROPOSED.md §23b) — the A5 substitution's half-square triangles emitted as their own tile class instead of being paired back into squares: 3 prototiles (rhombus 0, R-triangle 1, L-triangle 2), same silver-mean deflation as the classic entry (hypotenuse-pairing step removed) (`ammann-beenker-tri`, family quasiperiodic; M129).
- **Key facts:** module mirrors ammann_beenker.rs (rule tables, Q(√2) deflation, 8-rhombus star seed, STEP_CAP 20); exact Q(√2) Python battery: PF-orbit counts (24,16,16) → (136,96,96) → (792,560,560) → (4616,3264,3264), edge-to-edge growth, no interior overlaps, hypotenuse groups ≤ 2 with R–L paired, parity H = 2P + U, H/r → √2; 8-fold finite patches (see CHANGELOG M129).

### 69. Golden rhomboid (Andritz φ-substitution) ✅ (M144)

- **Motif:** the third Andritz-family φ-substitution — equilateral triangles side 1 (Y) and side φ (O) plus the 60/120 rhombus 1×φ (R) (`golden-rhomboid`, family quasiperiodic; M144; cf. §67 golden triangle, the Andritz-family sibling).
- **Key facts:** rules Y → O, O → O+Y+R, R → R+2O (areas 1 : φ² : 2φ in √3/4 units, all identities exact); count matrix [[1,1,0],[2,1,1],[0,1,0]] in (R,O,Y) order, char poly λ³−2λ²−2λ+1, PF eigenvalue φ² pinned as an exact Q(√5) root (φ² = (1,1)); exact 4-tuple Q(√5) arithmetic end to end, floats only at emission; 18 anchored orientation classes (Y_k 0–5, O_k 6–11, R_A/B_k 12–17), single-seed unbounded growth — depth d covers φ^d·R₀ exactly, the union is the 60° wedge {y ≥ 0, y ≤ √3·x}; T-junctions intrinsic (first at depth 2: 2 points / 5 incidences, every T-point closes to exactly π on both sides in π/3 units) (see CHANGELOG M144).

### 70. Generalized GLB binary n = 7 ✅ (M149)

- **Motif:** Pautze's generalized binary family at n = 7 (Hibma 2015 / Pautze 2017) — three 7-fold rhombi (acute angles π/7, 2π/7, 3π/7), inflation λ = √(2+2cos(π/7)) = 2cos(π/14) ≈ 1.9499, non-Pisot (odd n gives (n−1)/2 = 3 prototiles) (`glb-binary-n7`, family quasiperiodic; M149).
- **Key facts:** per-parent refinement is a TWO-CHOICE pool (shape 1 → {1} — rule 6 has no certified survivor frame, shape 2 → {5,2}, shape 3 → {3,4}) forced by compatible meshing at depth ≥ 2 — no local (shape, class) map drives refinement (eps-complementarity cascades through arbitrarily long adjacency chains, probed and refuted); certified generator = deterministic per-level DFS with all-pairs interior disjointness + eps-complementary bend bits on shared parent edges; exact K₇-twist pairs ((k₁, s₁k₂) ∈ K₇², K₇ = Q(b), b = 2cos(2π/7), dyadic denominators D = 2); fan-area law: every valid assembly carries total twist area 2λ^{2d} exact; depth cap 5 — the deepest level constructed AND certified (σ²…σ⁵ pass the full staircase boundary certificate; at depth 6 the candidate search exhausts: a legal patch need not admit a legal refinement) (see CHANGELOG M149).

### 71. Fibonacci × Fibonacci ✅ (M154)

- **Motif:** the 1-D Fibonacci substitution taken as a product with itself (`fibonacci-fibonacci`, family quasiperiodic; M154; the last §23 salvage item — siblings pregnant-chair M136, tangram M142 and golden-rhomboid §69/M144).
- **Key facts:** 4 orientation classes (LL, LS, SL, SS) on 3 prototile shapes (φ×φ square, φ×1 / 1×φ rectangle, unit square; LS/SL one shape at two rotations); linear inflation λ = τ² = φ², module rule = two product steps (children tile the INFLATED parent, anchors scaled by λ); count matrix B = A₄² = [[4,2,2,1],[2,2,1,1],[2,1,2,1],[1,1,1,1]], char poly (x−1)²(x²−7x+1), PF eigenvalue τ⁴ = λ², per-parent area laws (5,8)/(3,5)/(3,5)/(2,3) = area·τ⁴ exact; strict PIP + proper-crossing + T-junction checks at every growth step; exact Q(√5) integer pairs end to end, floats only at emission; depth cap 6 (growth totals F_{2d+2}²: 1, 9, 64, 441, 3025, 20 736, 142 129 ≤ MAX_TILES); the LL rule is locally 90°-symmetric; the patches themselves are P1 with no point symmetry (tests: 90° 273/441, 180° 169/441, mirror 273/441 invariants) (see CHANGELOG M154).

### 72. WIS 12-fold (Watanabe–Ito–Soma) ✅ (M158)

- **Motif:** the Bielefeld "WIS 12-fold" entry (Watanabe, Ito & Soma, Acta Cryst. A 51:936, 1995) — equilateral triangle + square + 30°/150° rhombus, dodecagonal symmetry, inflation λ = 2+√3 (λ² = 7+4√3), shipping the ENTRY FIGURE's published rules (WSI95 Fig. 2(iii)), NOT the Bielefeld patch's undocumented ruleset (`wis-12-fold`, family quasiperiodic; M158).
- **Key facts:** exact ℤ[√3] 4-tuple points (x = H(P+Q√3), y = H(R+S√3), H = s/2), floats only at emission; non-conformal same-scale rules — children are prototiles at edge 2H (same size as the parent) in a jagged cluster whose union has area λ²·parent and CONTAINS the parent; the rho rule is a 3-piece disconnected cluster (9+2+2 tiles) with fitted lattice-snap piece offsets; certified level-1 battery (closed simple boundary per piece, all interior edges paired, exact area identities (24,14)/(56,32)/(28,16) = λ² × parent, parent ⊆ union) — the rules do NOT compose at depth ≥ 2 (pinned drift-guard), so emission is the depth-1 single-parent square supertile (26 tiles) as a finite patch (see CHANGELOG M158).

### 73. Girih-inspired 14-fold (Pautze) ✅ (M159)

- **Motif:** the Bielefeld "Girih 14-fold" entry (Pautze, Symmetry 9(2):19, 2017) — 11 prototiles (R2/R3 rhombi, H26/H44/H45 hexagons, concave O29 + convex O36 octagons, concave D2 + D46/D56 decagons, regular 14-gon G14), inflation λ = 2b²+3b ≈ 6.850855 (b = 2cos(2π/7), exact K₇ triple (0,3,2), Norm(λ) = −7), λ² ≈ 46.934215 (`girih-14-fold`, family quasiperiodic; M159).
- **Key facts:** exact ½ℤ[ζ₁₄] twist-pair geometry (dyadic K₇ pairs, sin(π/7) folded into shared exponents); certified rule tables (763 entries, 11 classes) — corner-14-gon + edge-decagon + interior-kid panels (849 kids per full round), effective kid matrix N′ (edge kids ×½, corner 14-gon wedges k_i/14) satisfying the exact K₇ area law on all 11 rows with PF eigenvalue λ²; G14 convex-fit seed generation with pad-expanded culling and MAX_TILES guards; errata pinned in the per-tiling doc (the entry's unit claim is false — Norm(λ) = −7; Pau2017 Table 7's λ = 5.801939 is contradicted by its own Figs 7.4/7.5, measured 6.8507) (see CHANGELOG M159).

## Per-tiling documents

Each member's full specification — algorithm, exact geometry, verification, sources — lives in its own document under [docs/tilings/](../tilings/):

- [`penrose`](../tilings/penrose.md) — Penrose P3 (rhombs)
- [`penrose-p1`](../tilings/penrose-p1.md) — Penrose P1 (pentagon, star, boat, diamond)
- [`penrose_p2`](../tilings/penrose_p2.md) — Penrose P2 (kite and dart)
- [`penrose-pentagon`](../tilings/penrose-pentagon.md) — Penrose pentagons (5-tile φ-substitution)
- [`ammann-beenker`](../tilings/ammann-beenker.md) — Ammann-Beenker (A5)
- [`ammann-beenker-tri`](../tilings/ammann-beenker-tri.md) — Ammann-Beenker rhomb-triangle
- [`stampfli`](../tilings/stampfli.md) — Stampfli (12-fold)
- [`fractalizing-3-12-12`](../tilings/fractalizing-3-12-12.md) — Fractalizing 3.12.12
- [`shield`](../tilings/shield.md) — Gähler shield
- [`golden-bee`](../tilings/golden-bee.md) — Golden bee (Ammann A2 chair)
- [`golden-triangle`](../tilings/golden-triangle.md) — Golden triangle (√−τ reptile)
- [`golden-rhomboid`](../tilings/golden-rhomboid.md) — Golden rhomboid-triangle (Andritz φ-substitution)
- [`glb-binary-n7`](../tilings/glb-binary-n7.md) — GLB binary, generalized (n = 7)
- [`fibonacci-fibonacci`](../tilings/fibonacci-fibonacci.md) — Fibonacci × Fibonacci (product substitution)
- [`wis-12-fold`](../tilings/wis-12-fold.md) — Watanabe–Ito–Soma (12-fold)
- [`girih-14-fold`](../tilings/girih-14-fold.md) — Girih-inspired 14-fold (Pautze)

---

*Map: [docs/INDEX.md](../INDEX.md) · Changelog: [docs/history/CHANGELOG.md](../history/CHANGELOG.md) · Queue: [docs/ROADMAP.md](../ROADMAP.md)*
