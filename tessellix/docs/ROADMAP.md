# Roadmap

Live planning surface: the vetted-candidates queue
(full dossiers in `PROPOSED.md`) and the cross-cutting research
areas.

## Medallion composition program (§24 — planned sprints, next when the queue runs dry)

Two-level star-and-field compositions: high-degree medallions on
periodic hosts, rays absorbed exactly, interstices filled by a
complementary strap pattern. Full dossier in
[PROPOSED §24](../PROPOSED.md); the sprint ladder:

- ✅ **24a Medallion interface & catalog** — schema + exact re-derivations
  of landed medallions — **landed as M167**
  ([design doc](medallion-interface-design.md)); five catalog records
  with ray-exit cos² censuses; the 24b gate is now open.
- **24b Hole-bearing substrates + boundary-matched inference** — the
  core algorithm; ground-truth bar: reproduce M158/M161 cell tables
  first. Subsumes the open 22b hole decomposition.
  ✅ M174 sprint 1: WIS substrate + inference engine landed
  (6480/6480 exhaustive fills, 0 invalid; freedom `Report` output
  feeds 24e); M158 reproduced from the general path, M161 staged
  (degree-4 field kernel gap documented). Un-gates 24d's inference
  addendum + 24e.
- **24c (n,c)-monster medallion generator** (22e partial; {8,12},
  {9,12}, {9,18} first) — ✅ M169: six parallel-pair records with
  exact placements, windows and M163-checked exits; ✅ M171: n=9
  shipped via the rotated odd-n family — (9,2) {9/3}, (9,3) {9/1}
  (K₉-exact, M163-checked eight records); n=11 blocked on degree-5
  fields; {8,12}/{9,12} deferred.
- **24d Composition families on periodic nets** — instance 1 ✅ M172
  (dual-net fill, reduced scope; inference-fill deferred to 24b).
  Companion instances after 24b.
- **24e Underdetermined-θ search** (22h partial) — after 24b maps the
  freedom.
- **24g Bonner polygonal systems** — the subdivision path to multi-level
  patterns (nested 4-fold/5-fold systems); compared against 24b's
  inference path on the Darb-i Imam ground truth. Source-gated (Bonner's
  book needs acquiring).
- **24h PIC explorer** — δ/θ sweep over host tilings with programmatic
  scoring (symmetry, sliver-free, strap density); ranked galleries via
  the 22g styles; winners become exact-pin registry candidates.
  ✅ M170: sweep + `explore` CLI shipped (1921 rows, 0 refusals,
  byte-identity tested); leaderboard banked, exact-pin candidates
  queued. After 22g + 24a. Exact-pin follow-up #1 ✅ STOP — the δ=90°
  winners are already-registered centroid duals (square→square,
  trihexagonal→rhombille, truncated-square→tetrakis-square; δ=45 top
  row = pic-4-8-8); sweep validated the registry, no new entries.
  Remaining candidate: the 3.12.12 canonical motif assignment.
- **24f Quasiperiodic medallion hosts** — gated behind CAP/A4 family
  ownership; the dual method (Ammann-bar / Robinson-triangle duals) is
  the exact path for Penrose-class hosts.
  ⛔ STOPPED 2026-11-25: Type A tables book-gated (figures raster-only);
  2φ³ shared-constant finding banked (three exact derivations; joint-rule
  contradiction [DEC|PENT|PENT|DEC] vs [HEX|DEC|HEX]) — resume needs the
  Bonner book (USD 17.99 proposed).

User-pinned: ground-truth-first · registry instances · periodic nets
first.

## Proposed — vetted candidates

The per-candidate research dossier — literature verification,
construction path, exact-arithmetic plan, effort, and suggested order —
lives in `PROPOSED.md`; the entries below are the short-form queue.

### Periodic, straightforward

- 🔭 **2-uniform tilings** (20 known types) — supercells mixing two
  uniform vertex figures (e.g. 3³.4² + 3².4.3.4); data tables on
  bigger cells; moderate work each, visually rich. Their duals give
the "mixed floret/cairo/prismatic" patterns.
- ✅ **2-uniform arc underway (M44, M48).** *`kuniform-p6m` done
  (M44): the "1/3-dissected" honeycomb (Wikipedia `2-uniform n10`) on
  the index-3 sublattice m ≡ n mod 3 of the honeycomb centers.
  `kuniform-pmm` done (M48): the "rows of triangles between rows of
  hexagons" tiling (Wikipedia `2-uniform n11`).*
- ✅ **Paver trio complete (M39–M41).** *Basketweave done
  (M39): p4g, not the naive p4m guess — mirrors sit at the odd axes,
  offset from every 4-fold center. Herringbone done (M40): pgg as
  expected for the generic 2:1 case — chiral, 16 2-folds per 4×4
  period, two perpendicular glide families. Running bond done
  (M41): cmm as expected — mirrors at the integer x-axes and
  half-integer y-axes, centered 2-folds, glides in both directions.*

### Islamic star-pattern extensions (Kaplan dissertation Ch. 3, PROPOSED.md §22)

- ✅ **22a Design-element motif library (M163).** *Landed:
  `hankin_motifs.rs` — `Motif::Star` / `Rosette` (Lee's ideal pinned
  at (π/n, 1, 0)) / `ExtendedStar` / `ExtendedRosette` with per-tile
  motif assignment via `build_cells_with_motifs`; additive hooks in
  `hankin.rs` (regression-proven byte-identical on the ten existing
  engine-built hankin tilings). Erratum: the dissertation prints the
  extension child angle as θ/2 — the battery-pinned reading is
  θ′ = θ − π/n with r′ = r·cos θ/cos(π/n − θ) (coincident only at
  θ = 2π/n). Three showcase instances: hankin-4-8-8-motifs (p4m),
  hankin-hex-motifs (pmm), najm-63-motifs (p6m); Islamic 26 → 29.*
  Brief: [kaplan-22a-motifs](briefs/kaplan-22a-motifs.md).
- ✅ **22b Najm template-tiling generator (M99).** *Landed:
  `najm.rs` with `najm-63-2e-0-3e-ac` (Fig. 3.21, p6m) and
  `najm-44-3e-0-0-a` (Fig. 3.22, p4m); radii cases 1–4 implemented;
  generic hole decomposition remains open (see PROPOSED §22b for the
  full status incl. the re-derived case-3 factor).*
- ✅ **22c Rosette transform (M99).** *Landed: `rosette_transform.rs`
  (hankin_ext) implements the diss. §3.8 bridge — inner n-gon radius
  r′ = r(cos(π/n) − sin(π/n)tan(π(n−2)/4n)) rotated π/n, spokes to edge
  midpoints; irregular tiles via perpendicular-bisector inference +
  cluster averaging — and the first registry instance
  `hankin-4-8-8-rosette-transform` (4.8.8, θK = θE = 45°, δ = 0):
  93-tile transformed table, edge-to-edge, p4m-exact to 6.5e-16;
  §55. The 11/13-pointed-star angle-mismatch applications
  ride on 22b/22e substrates; Bonner's off-center contact adjustments
  remain unformalized.*
- ✅ **22d Islamic parquet deformations (M165)** — θ as a smooth
  function of position (θ(x) strips); spatial-animation star
  patterns on the strapwork engine (an export/render feature, not a
  registry entry). Landed: `walls_with_delta(tiles, f)` per-contact-
  point override (walls() byte-identical as the const wrapper),
  Linear/Radial/Sine profiles, `--deform` CLI. **Phase-A finding:**
  the closure envelope is a DISCRETE point set per substrate
  (square/truncated-square {45°,90°}, trihexagonal {60°,90°},
  truncated-hexagonal {60°} — 90° does not close there), derived by
  1°+0.1° sweeps + probes; "clamp" = snap-to-nearest-closed-angle
  (`snap_to_envelope`, `envelope_for` in tess-tilings). Also
  pinned: the §3.4.1 continuity identity is RAY-PAIR-level (shared
  midpoint ⇒ reflected pairs); wall-level identities do not survive
  varying f. Feeds the 24h explorer (`envelope_for` +
  `validity_stats` + snap).
- 🔭 **22e Quasiperiodic substrates + (n,c)-monster motifs** —
  regular 2n-gons on lattice-projection rhomb patches (5-D ⇒ Penrose
  P3), overlapping polygons fused into (n,c)-monsters
  (n > 6, 2 ≤ c ≤ ⌈n/2⌉−2) with a motif menagerie for n ≤ 12 at
  contact angles kπ/n; plus Kepler's Aa (36°-rhomb construction,
  pentacle-ring modification).
- 🔭 **22f Absolute-geometry star patterns** — spherical
  ([3,3], [3,4], [3,5]) and hyperbolic (Poincaré disk) instances of
  the same construction; absolute trig Θ(x), E(x), T(x); absolute
  star θ = π/2 − φ, tan φ = 1/(E(r)tan(πd/n)); symmetrohedra by
  convex hull. Merges with and de-risks the hyperbolic arc (#16).
- ✅ **22g Decoration suite (M164)** — §3.7 styles over the M57
  engine: bipartite 2-colour fill, mitered outline bands
  (l = d/sin(θ/2), spike-pinned 1e-9), interlacing via strand-parity
  alternation proven lattice-consistent by ±2-cell torus folds on
  every classic-δ substrate, outline-interlace composition;
  region-gated preconditions with honest per-style errors; Phase-A
  δ-regime audit (12/192 even-degree rows, all classic δ clean).
  Castéra's "correct" band widths remain an open derivation.
- ❓ **22h Moroccan finite designs; optimization-based motif
  correction; strange stars** — Castéra star+saft skeletons (up to
  96-point stars), search over an aesthetic objective to replace
  ad-hoc contact corrections, and the star-order-set question
  (nearly-regular polygons; the Altair near-miss tiling).
- ✅ **Girih primitive tiles (M131–M134).** *All five classical
  primitives now ship as periodic data tables beside shesh-band:
  bowtie (72.72.216, M131), 72°/108° rhombus (M132), decagon
  (10.6.10, M133), pentagon (5.5.4.4, M134) — exact
  K = Q(2cos π/10) tables with patch batteries and cross-table
  assembly congruence. Unblocks the §23f follow-ups (Andritz
  bowtie–hexagon family, girih-inspired 14-fold).*

### Aperiodic substitution (each needs rule derivation first)

- ✅ **Socolar 12-fold shield tiling (M51).** *Shield hexagons +
  squares + black/white triangles on the dodecagonal frame; PF
  Perron 2+√3; the class's substitution representative. The original
  Socolar-12 decoration tables are blocked-on-source (PROPOSED.md #8);
  the wheel tiling (shield's MLD dual) is parked (M61) — see the
  Research entry below.*
- 🔭 **Ammann sets A1–A4** — A4 ✅ M173 (`ammann-a4`, Quasiperiodic
  family); A3 (φ-inflation) + A1 (parked: no substitution structure)
  remain.
- 🔭 **§23d limit-periodic substitutions** — ✅ **half-hex landed (M135)**
  (Socolar's half-hexagon substitution; closes the #17
  substitution-hierarchy gap; measured period hierarchy
  p = (3/2)·2^g = 6, 12, 24, 48, 96) and ✅ **domino (table) landed
  (M138)** — one entry ("Also known as 'table tiling.'"),
  limit-periodic via approximating periodic tilings, the depth-g
  patch itself the fundamental cell of the period-(2^g, 2^(g+1))
  approximant ("full-block" manifest); NOT cut-and-project (Solomyak
  1997 / Robinson 1999 continuous spectral component — dossier
  corrected); brief: [domino-table](briefs/domino-table.md).
- 🔭 **§23 Bielefeld salvage wave** — four briefs revived from the
  lost pre-rework §23 session (rules pinned in the briefs; prototype
  batteries archived in `.scratch/salvage-20261120/`):
  [golden-rhomboid](briefs/golden-rhomboid.md) (Y/O/R φ-substitution,
  Andritz) ✅ landed M144; [fibonacci-product](briefs/fibonacci-product.md) (product
  substitution, τ²), [tangram](briefs/tangram.md) (§23d rep-tile
  dissection, Z[√2]) ✅ landed M142; [pregnant-chairs](briefs/pregnant-chairs.md)
  (§23d, √3 / Z[√3]) ✅ landed M136.
- ✅ **Andritz Bowtie–Hexagon family (§23f).** *Arc complete — four
  5-fold FLC substitution entries landed on the M131–M134 girih
  primitive tables: `andritz-bowtie-hexagon` (M151),
  `andritz-bowtie-hexagon-10gon-1` (M152),
  `andritz-bowtie-hexagon-10gon-2` (M153),
  `andritz-bowtie-hexagon-10gon-3` (M160). Arc-closing finding
  (SESSION 19, programmatic): "bowtie-hexagon-12gon-3" — sometimes
  seen as a fifth name in the wild — is the image-directory alias of
  entry 10gon-3, not a separate Bielefeld entry (md5-identical
  patch/rule images; zero `12gon-*` substitution-index slugs; Pautze
  CAST Table 8 lists exactly three Bowtie-Hexagon-Decagon rows,
  matching landings 2/3/4). Brief:
  [andritz-bowtie-hexagon](briefs/andritz-bowtie-hexagon.md).*

### Finite verified patches (patch-style entries)

- ✅ **Long/short girih bands (10-fold).** *Covered by the
  girih-primitive cluster M131–M134: all five classical girih
  primitives (shesh-band hexagon, bowtie, rhombus, decagon, pentagon)
  ship as periodic translation tables — the "other periodic girih
  band orientations complementing shesh-band" this item queued.
  Girih-band-adjacent remainder: the full girih decagon patch (landed
  M161 below).*
- ✅ **Full girih decagon patch (M161).** *Landed:
  `girih_decagon_patch.rs` — the classic two-level girih figure as a
  finite patch data table: 1 large decagon + its 10 edge-adjacent
  large bow-ties (each large tile = its closed Darb-i Imam cluster
  from `girih_qc`'s certified tables + the edge-marker trios on every
  unique large edge) = 611 small tiles (270 bobbins + 220 bow-ties +
  121 decagons), 0 duplicates, 0 T-junctions, Euler χ = 2, exact
  10-fold symmetry, edge-to-edge unit edges throughout; straps NOT
  shipped (the weave crosses the DEC {10/3} star and breaks the
  exact-partition contract — reported per brief). islamic family
  30 → 31; Islamic tab 25 → 26.*
- 🔭 **Penrose sun/star/cartwheel finite patches** — easy given P3
  machinery; mostly of documentary value.

### Research (no construction path chosen yet)

- 🔭 **Cyclotomic field layer (§23e/§23i)** — exact degree-3/4
  arithmetic as the gated first step for the 7/9/14/15/21-fold
  families (Danzer, Harriss, Goodman-Strauss, CAST, Nischke–Danzer
  deltoids, Say-awen, GLB-binary re-scope). **Design doc landed:**
  [docs/cyclotomic-field-layer.md](cyclotomic-field-layer.md) —
  oblique U(k) frame (rotation = companion matrix, no sine stored),
  derived + Sturm-certified minpolies (K₇/K₉/K₁₅/K₂₁/K₁₇/L₁₀), Sturm
  sign engine, per-family applicability table; CAST 10-fold re-scoped
  to L₁₀ (the girih field, NOT Q(√5) — dense DTO orientations handled
  per orientation class); all 20 shipped girih ROTMs reproduced
  bit-for-bit by the generic recipe. ✅ **K₇ core landed (M137,
  fields.rs)** — generic degree-D exact layer (kmul/knorm/ksign) +
  oblique frame + star walks + sign battery, no census change.
  ✅ **K₉ + K₁₅ + the girih ROTM anchor landed (M139)** — both cores
  (minpolies x³−3x+1, x⁴−x³−4x²+4x+1) + L₁₀ (the girih quartic) with
  the permanent regression anchor: all 20 shipped girih ROTM matrices
  rebuilt through the generic kmul/RED_L10 path at /64, bit-for-bit;
  BISECT_CAP tightened 10→4 with the i128-envelope derivation.
  ✅ **First consumer landed (M149)** — GLB binary n = 7 (§23e
  re-scope): the K₇ slice (fields.rs) consumed directly by the
  certified DFS generator; depth cap 5 = deepest certified level.
- ❓ **Wheel tiling (Nissen)** — **parked (M61)** with a proven
  negative: the edge-orthogonal dual of the shield substitution is
  exact but admits no substitution at any inflation (algebraic killer
  at θ; 0/849 union tests at θ²), and no published substitution exists
  (the Tilings Encyclopedia documents the wheel as a windowed
  tiling). W_0 is fully pinned — chiral 11-gon (area (2+√3)/4) and
  star 24-gon (area 3/2+√3), all edges on the 15° frame in Q(√3);
  evidence pack in the task scratch (WHEEL_REPORT.md). Revival paths:
  the projection generator below (the wheel is canonically
  cut-and-project) or a static W_0 patch; the inert `wheel = []`
  feature stays registered for a future CPS milestone (PROPOSED.md #8).
- ✅ **Projection (cut-and-project) generator class — first instance
  landed (M166):** `cap` — the Baake–Gähler–Sadun cut-and-project
  tiling (arXiv:2305.05639), the D-map deformation of the hat metatile
  lineage with shape vector (3φ, 3φ, 1, φ+ξ, 2φ) = a φ² left
  eigenvector of σ̃₁ ⇒ LIDS; exact integer abcd arithmetic, floats
  only at emission ([brief](briefs/cap-cut-project.md),
  [design doc](../cap-cut-project-design.md)). The generic class
  (window-based generation for Penrose/AB/12-fold as an alternative
  to substitution) remains open.
- ❓ **Random/statistical tilings** — lozenge (rhombille with random
  heights), square–triangle, Truchet mazes. Blocked on a determinism
  policy: would need a seeded-RNG parameter (`ParamSpec::Int`) and a
  stated rule for `check_deterministic` (seeded, stateless ⇒ still
  deterministic per parameter set).
- ✅ **Isohedral (IH) tile families (M162).** *Landed:
  `isohedral-generator` — incidence-symbol engine over the 11 Laves
  topologies with exact per-orbit shaped edges, plus five showcase
  instances (IH1 p1 / IH6 pgg / IH7 p3 hexagons on the 6³ net,
  IH27 pgg / IH29 p4g pentagons on the Cairo net; design doc
  [../isohedral-generator-design.md](../isohedral-generator-design.md)).
  Exact Q(√3) field; adjacency symbols derived by exact geometric
  edge matching (BFS-closure verified), never hand-written. Category
  rule held: Laves primary, Pentagon Types secondary on the two
  pentagons — the pinned 17-card Wallpaper catalog untouched.
  isohedral family 0 → 5; Laves 28 → 33; Pentagon Types 15 → 17.*
  Brief: [isohedral-generator](briefs/isohedral-generator.md).
- ❓ **Polyomino/polyiamond rep-tile zoo** — L-tromino ✅ (chair,
  M30); P-pentomino ✅ (implemented on branch `reptiles`, pending
  integration, M47 — exactly two dissections of the inflated P,
  8 chiral orientation classes, 5×8 seed block); T-tetromino ❌
  proven **not** rep-4 (the inflated 16-cell T has zero tilings by
  four T's: the corner cell forces a bar + stem placement that
  isolates its neighbour — exhaustive search confirms 0); hexiamonds
  beyond sphinx (only the 3×1 rhombus and the sphinx are rep-4);
  remaining shapes easy, verification-heavy.
- ❓ **Hyperbolic tilings {p,q}** — would need a hyperbolic geometry
  core (Poincaré disk) behind the same trait; large, separate arc.
- ❓ **Wang-tiling UI** — generate Wang-satisfying patches from small
  aperiodic sets (e.g. Kari/Culik); research both math and UX.

## Research / exploration areas (cross-cutting)

1. **Substitution-rule toolkit** — the AB/Stampfli/hat work all needs
   the same machinery: given prototile shapes + inflation factor,
   search for valid subdivisions (boundary closure into child edges,
   inter-parent consistency, count-matrix match) and emit exact
   coordinates. Build it as a standalone Python module in scratch,
   then document the workflow in AGENTS.md.
2. **Automated dual construction** ✅ (M98) — the arc that motivated
   this entry dualized 7 tables by hand-transcribed Python; the
   `dual(primal: &LatticeTiling) -> Option<LatticeTiling>` helper
   (centroid dual, exact for regular primal faces) now exists in
   `tess-tilings/src/dual.rs`. It was first validated as an oracle
   against all 7 hand-built Laves tables (exact reproduction, max
   vertex residual 0.0) and then used to construct the first batch of
   k-uniform duals (p6m, n18, n9, n8) — transcription risk for these
   classes is gone, and the remaining p6m/pmm 2-uniform duals queue
   behind the same helper (see §48).
3. **Coloring engine** — seeds are currently hand-assigned per table
   (cairo's 4-coloring, rhombille's orientation 3-coloring). A small
   adjacency-graph proper-coloring helper (test-side, like the brute
   force census) could assert "pattern mode is a proper coloring" for
   every tiling that claims it. **Started**: `laves_pattern_mode_is_proper_coloring`
   (lib.rs) checks every Laves tiling's generated patch for edge-adjacent
   equal colors; extend to other families that claim proper coloring.
4. **Matching rules vs generation** — we generate valid tilings but do
   not enforce matching rules (aperiodicity is inherited from the
   substitution + seed, not from local rules). Document the
   distinction; optionally add Ammann-bar decoration as a rendering
   layer (segments per tile edge class).
5. **Invariant censuses as standard tests** — hirschhorn (Euler χ = 1,
   C6 symmetry) and girih-qc (full edge/vertex census) set the
   pattern; extend angle-multiset and edge-degree censuses to every
   family as optional per-tiling tests.
6. **LOD / view independence for substitution tilings** — pinwheel's
   octave-anchored seeding solved per-frame regeneration; generalize
   the idea (seed-depth quantization by view extent) to penrose and
   future substitution entries.
7. **Symmetry auto-verification** — wallpaper-group labels are
   declared per table; a probe test could reflect/rotate generated
   patches about declared centers and assert invariance, catching
   mislabeled groups.
8. **Performance envelope** — MAX_TILES = 250_000 guards the GUI;
   spatial indexing or incremental generation could push it; measure
   per-tiling generation cost and record budgets in README.
