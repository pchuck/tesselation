# Roadmap

Live planning surface: the vetted-candidates queue
(full dossiers in `PROPOSED.md`) and the cross-cutting research
areas.

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

- 🔭 **22a Design-element motif library** — parameterized
  d_n-symmetric motifs for regular polygons in any substrate:
  stars (n, r, s, θ) with {n/d}s ↔ θ = πd/n (s = 2 when n > 6 and
  θ > 2π/n), rosettes (n, r, s, θ, h, φ) with Lee's ideal at
  (π/n, 1, 0), and the extension operator
  (θ′ = θ/2, r′ = r[1/cos(π/n) − sinθ·tan(π/n)·cos(π/n − θ)]).
  Generalizes the M94 fixed configs to per-tile motif assignment.
- 🔭 **22b Najm template-tiling generator** — the
  ([p,q]; m_A o_A, m_B o_B, m_C o_C; inflation) notation: regular
  polygons on a [p,q] group's rotational axes, inflated by the four
  absolute-trig case equations (monotone binary search), bowtie
  hole-fillers. A parameterized substrate family replacing
  hand-coded templates (Euclidean [4,4]/[6,3] first).
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
- 🔭 **22d Islamic parquet deformations** — θ as a smooth function
  of position (θ(x) strips); spatial-animation star patterns on the
  M94 engine (an export/render feature, not a registry entry).
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
- 🔭 **22g Decoration suite completion** — §3.7 styles audited
  against the M53–M57 engine: bipartite 2-colour fill, equidistant-
  curve outlines with mitered joins (Θ(l) = Θ(w)/sin(θ/2)),
  interlacing fixed on the [p,q]⁺ double fundamental map,
  outline-interlace composition; Castéra's "correct" band widths as
  an open derivation.
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
- 🔭 **Ammann sets A1–A4** — the rest of Ammann's aperiodic families;
  moderate each once the AB machinery exists.
- 🔭 **§23d limit-periodic substitutions** — ✅ **half-hex landed (M135)**
  (Socolar's half-hexagon substitution; closes the #17
  substitution-hierarchy gap; measured period hierarchy
  p = (3/2)·2^g = 6, 12, 24, 48, 96); next: domino/table
  (+variants) in the same family.
- 🔭 **§23 Bielefeld salvage wave** — four briefs revived from the
  lost pre-rework §23 session (rules pinned in the briefs; prototype
  batteries archived in `.scratch/salvage-20261120/`):
  [golden-rhomboid](briefs/golden-rhomboid.md) (Y/O/R φ-substitution,
  Andritz), [fibonacci-product](briefs/fibonacci-product.md) (product
  substitution, τ²), [tangram](briefs/tangram.md) (§23d rep-tile
  dissection, Z[√2]); [pregnant-chairs](briefs/pregnant-chairs.md)
  (§23d, √3 / Z[√3]) ✅ landed M136.
- 🔭 **Andritz Bowtie–Hexagon family (§23f)** — four 5-fold FLC
  substitution entries (small/large bowtie, hexagon variants) built
  on the now-shipped M131–M134 girih primitive tables;
  [andritz-bowtie-hexagon](briefs/andritz-bowtie-hexagon.md);
  dispatched (strong tier, per-landing commits).

### Finite verified patches (patch-style entries)

- 🔭 **Long/short girih bands (10-fold)** — the other periodic girih
  band orientations complementing shesh-band.
- 🔭 **Full girih decagon patch** — the classic decagon-and-bowtie
  strapwork region with hexagon/bowtie/rhombus infill (Lu–Steinhardt
  figures), as a documentary overlay on the substitution entry.
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
  bit-for-bit by the generic recipe. Next gated step: `fields.rs` +
  K₇ per doc §7 — [fields-layer-k7](briefs/fields-layer-k7.md);
  dispatched (frontier).
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
- ❓ **Projection (cut-and-project) generator class** — window-based
  generation for rhombus tilings (Penrose, AB, 12-fold) as an
  alternative to substitution; would validate patches independently.
- ❓ **Random/statistical tilings** — lozenge (rhombille with random
  heights), square–triangle, Truchet mazes. Blocked on a determinism
  policy: would need a seeded-RNG parameter (`ParamSpec::Int`) and a
  stated rule for `check_deterministic` (seeded, stateless ⇒ still
  deterministic per parameter set).
- ❓ **Isohedral (IH) tile families** — parameterized generators for
  some of the 93 isohedral types (e.g. IH7-like interlocking
  variants); needs a shape-parameterization design.
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
