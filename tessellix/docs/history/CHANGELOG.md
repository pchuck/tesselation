# Milestone changelog

One entry per milestone, appended at integration time (orchestrator
only — see the fleet protocol in `tessellix/AGENTS.md`).

- 2026-11-07 — created; arc "complete the classics" queued
  (M19–M27: 7 Laves duals, Pythagorean, Ammann–Beenker).
- 2026-11-07 — arc complete: M19–M25 Laves duals, M26 Pythagorean,
  M27 Ammann–Beenker all ✅ (28 tilings, 136 tests).
- 2026-11-08 — M28 Penrose Pentagon (5-tile φ-substitution) ✅: five-
  shape set from the exhaustive pentagon pool, strict-0 + 1-hole layers
  (29 tilings, 141 tests).
- 2026-11-08 — M29 Penrose P2 (kites & darts) ✅: glue-labelled
  half-tile substitution, unique labelling found by exhaustive 8192-way
  search, exact Q(φ, s36) verification (30 tilings, 152 tests). Stale
  Hat/tess-core concave-polygon caveat removed.
- 2026-11-09 — M30 Chair tiling (L-tromino) ✅: λ = 2 rep-4
  substitution, rule forced by exhaustive dissection enumeration,
  non-edge-to-edge (inherent T-junctions, refined-edge census
  instead), octave-anchored integer-grid generation (31 tilings,
  162 tests).
- 2026-11-09 — gui shading fix ✅: the triangulation cache keyed on
  (proto, vertex count) mis-applied one orientation's ear-clip to all
  four chair classes (chair colors bled across tile walls through the
  concave notches). Cache now keys on a vertex-layout fingerprint;
  chair emission made rotation-aligned so all four classes share one
  valid triangulation (165 tests).
- 2026-11-09 — M32 Sphinx tiling (pentagonal hexiamond) ✅: λ = 2
  rep-4 substitution on the triangular lattice, fully machine-derived
  (hexiamond enumeration self-check → rep-4 census → unique dissection
  → 12-class conjugation rule → doubly stochastic substitution matrix
  → unique 2×3-rhombus seed of 2 sphinxes → g0–g4 growth battery with
  refined edge census), non-edge-to-edge with T-junction counts
  0/2/16/78/344, octave-anchored integer-grid generation (32 tilings,
  176 tests).
- 2026-11-09 — M33 quaquaversal ❌ removed from the 2D queue: it is a
  3-space tiling (triangular prisms, orientations dense in SO(3)); the
  2D dense-orientation analogue already exists (pinwheel).
- 2026-11-09 — M34 Hat tiling (aperiodic monotile) ✅: the SMKG
  einstein as H/T/P/F metatile substitution (paper Fig 2.6 rules,
  cross-validated against hatviz and einstein-tiling), exact Eisenstein
  placements, H-lineage count tables 4→25→169→1156→7921→54289, 0
  matching-rule violations, exact nesting; iterated outlines drift by
  design (φ² only in the limit); per-view seeded lineage blobs
  (96-grid anchor + (1,−1) wedge offsets, depth escalation 3→6, exact
  clipped-area acceptance) because one lineage leaves permanent
  interstitial fjords near its seed — filled in the plane tiling by
  neighboring supertile hierarchies (Extension Theorem); NOT
  edge-to-edge (565 midpoint T-junctions at depth 3) (33 tilings,
  190 tests).
- 2026-11-09 — M18 redo — girih star-and-cross ✅: the static 41-tile
  patch (valid tiles, wrong arrangement) replaced by the genuine
  Darb-i Imam 3-prototile substitution (Lu & Steinhardt 2007; DEC/HEX/
  BOW, IF = 2(2+√5) = 2φ³, edge-marker seam rule), all tables derived
  by exact BFS in K = Q(2cos18°) and cross-checked against Egan's
  reference doubles to ≤ 1.1e-14; marker↔interior-edge bijection
  proven (gen 2: 55 080 = 55 080, 0 duplicates); full structural
  battery (unit edges, matched edges, 0 T-junctions, legal fans,
  χ = 2, exact 10-fold symmetry) green in both Python and Rust;
  frozen core P₂ = P₃ to radius 79; S-box window cascade proven equal
  to the full lineage (gens 2–3); covered growth ι₂ = 111.3,
  ι₃ = 910 (8.18×/gen); TUI render now the correct 10-fold rosette
  (33 tilings, 197 tests).
- 2026-11-10 — M31 Penrose P1 (pentagon/star/boat/diamond) ✅: the
  classic P1 set by exact φ² deflation with cross-boundary short-edge
  merges (`penrose_p1.rs`, feature `penrose_p1`; G+G→D, CORE+2G→B,
  S2C+2G→S, S1C+G→S); single-seed pentagonal-gasket window with five
  5-fold notch channels, clean disk radius R/φ² guaranteed to cover
  the view; area-conserving substitutions verified exactly (34
  tilings, 204 tests).
- 2026-11-10 — M37 Turtle ("hats in turtles", einstein family) ✅:
  turtle (area 10√3) + hat (8√3) on Kaplan's exact hex lattice,
  9-label 8+1 key-point substitution transcribed verbatim from
  spectre.js with per-level chirality flip; homochiral, edge-to-edge,
  ratio → 4+√15; per-view rigid depth-d Delta patches behind an
  inscribed-disk anchor grid (`turtle.rs`, feature `turtle`);
  pose-identity to the reference verified at every depth, census χ = 1
  (35 tilings, 214 tests).
- 2026-11-10 — M38 Spectre (chiral einstein Tile(1,1)) ✅: the
  equal-edge single-prototile member of the family, aperiodic without
  reflections; exact Q(√3) surd geometry, D12 frames, same nine-label
  machinery (Mystic = two spectres) (`spectre.rs`, feature `spectre`);
  vertex-set multisets match spectre.js through depth 5
  (34649/34649), census χ = 1, all emitted tiles unreflected (36
  tilings, 224 tests).
- 2026-11-11 — M35 Girih star-and-cross, 10-fold periodic (measured)
  ✅: the classical periodic star-and-cross strapwork as a measured
  data-table `LatticeTiling` (`girih_sc.rs`, feature `girih_sc`) —
  reference panel decoded at 1 px/unit, cell [-38.5,38.5]×[-45,45] on
  the (77,45)/(0,90) oblique lattice, group pmm; 545-tile cell (99
  white ground faces + 446 ink strap pieces) measured by band fitting
  (ridge-snapped centrelines, duplicate merging, cap trimming) + raster
  clipping + medial-skeleton partition of the ink + conformalization;
  cell verified exactly on the shipped data (area sum 3.9e-16 rel.,
  edge parity 0 violations, all 1822 interior vertex angle sums 360°,
  3×3-patch grid covered exactly once); TUI export matches the
  reference cell-for-cell (37 tilings, 227 tests).
- 2026-11-11 — M36 Khatam girih decomposition (8-point) ✅: the classical
  kissing khatam ({8/3} star-and-cross, M15 at T = 1/2) decomposed into
  its prototile kitab (`khatam_girih.rs`, feature `khatam_girih`) —
  each star into 8 rhombi (45°/135°, side √(4−2√2)/4), each interstice
  cross into a central square (side (2−√2)/2) + 4 pentagons
  (135/90/90/112.5/112.5); 13 tiles per unit cell, areas summing to 1
  exactly, edge-to-edge across the plane with vertex figures 4⁸ /
  90.135.135 / 45.45.135.135 / 45.90.112.5.112.5 (p4m). Verified
  exactly before porting (637-tile patch battery: shapes, edge parity,
  vertex figures, exact-once coverage, p4m invariance); standard trio +
  culling list green. TUI export matches M15's undecomposed silhouette
  (38 tilings, 230 tests).
- 2026-11-12 — M45 Stampfli 12-fold dodecagonal ✅: the dodecagonal
  quasiperiodic tiling by squares + equilateral triangles + 30° rhombi
  (Stampfli & Schaad, Bridges 2021) as a patch substitution with
  inflation 2+√3, rule transcribed from the Bielefeld encyclopedia
  figure; straddling unit children (one vertex outside the parent,
  shared by two parents) deduped by exact sorted vertex set; 12-rhombus
  seed star at scale λ^n deflated to unit pieces with per-step culling,
  all anchors exact in Z[√3]/4 (`stampfli.rs`, feature `stampfli`).
  Verified exact before porting: per-parent cover-exactly-once + edge
  partition 1:√3/2:√3/2:1; gen 1–4 battery zero overlaps, zero
  T-junctions, exact 12-fold symmetry, PF area law; star census rows
  S′=S+2R, R′=12S+3R+6T with Perron λ²=7+4√3 (PF vector
  (1, 3+2√3, 4+10√3/3)); Python↔Rust table diff exact; standard trio
  green (39 tilings, 246 tests).
- 2026-11-12 — M47 P-pentomino tiling (rep-4 reptile) ✅: λ = 2 rep-4
  substitution (`p_pentomino.rs`, feature `reptiles`). Exhaustive
  enumeration of the inflated 20-cell P found exactly two dissections
  (class multisets {0,1,4,6} and {3,4,4,5}); the canonical {0,1,4,6}
  is used. The P is chiral, so the rule necessarily mixes chiralities —
  8 orientation classes with a doubly stochastic substitution matrix
  (rows and columns sum to 4); non-edge-to-edge (5 T-junctions per
  parent, verified on maximal segments); 5×8 seed block with one tile
  per class (62 such tilings of the rectangle exist); octave-anchored
  integer-grid generation like chair. The T-tetromino sibling was
  dropped, proven **not** rep-4: exhaustive search finds 0 tilings of
  the inflated 16-cell T by four T's (the corner cell forces a
  bar + stem placement that isolates its neighbour) (40 tilings,
  258 tests).
- 2026-11-12 — M43 Periodic pinwheel arrangement ("windmill", p4) ✅:
  Conway's periodic "spin" of the pinwheel motif as a 5-tile
  data-table `LatticeTiling` (`windmill.rs`, feature `windmill`) on
  the (3,0)/(0,3) square lattice — four 1–2–√5 right triangles
  (seeds 1–4, the four orientation classes) pinwheeling around one
  square (seed 0, the largest tile); all coordinates exact integers,
  no surds, areas 4·1 + 5 = 9 = |det|; non-edge-to-edge (mid-side
  T-junctions): vertex figures 90⁴ at the lattice points (3i, 3j) and
  180·90·arctan(1/2)·arctan(2) at the four T-junction classes
  {(0,1),(0,2),(1,0),(2,0)} mod 3, all pinned by test; homochiral
  (every triangle one chirality sign), so p4 rather than p4g —
  4-folds at the motif centres (3i+1.5, 3j+1.5) *and* the lattice
  points, 2-folds at all four centre classes mod 3, no mirrors or
  glides (both directions pinned by a restricted symmetry probe over
  a 180-tile window, 14 positive + 24 negative candidates); naming
  waiver per AGENTS.md step 4 (no canonical published name exists:
  Wikipedia has no "Windmill tiling", pinwheel literature is
  aperiodic-only, Grünbaum's 1977 "windmill blades" quilt is a
  different edge-to-edge design), approved by the repo owner
  2026-11-12 (41 tilings, 263 tests).
- 2026-11-12 — M39 Basketweave paver bond (2:1 dominoes) ✅: the
  classical basketweave as a `LatticeTiling` data table
  (`basketweave.rs`, feature `pavers`) — 45°-rotated square lattice
  ⟨(2,2),(2,−2)⟩ (area 8), 2 H + 2 V split dominoes per cell; each
  long side split into two unit edges (mid-side vertex), so vertex
  figures carry 180° at the split midpoints (`2.4.4`) and 4.4.4.4 at
  the even-even block corners. Verified exactly before porting
  (Python spike over a bounded patch: area sum = |det|, edge parity,
  vertex figures, exact-once coverage, rigorous coset symmetry check);
  group **p4g** (not the naive p4m guess): 4-folds at every even-even
  point, mirrors at the odd axes only — offset from the 4-fold
  centers, no diagonal mirrors — and six glide families, with the
  odd-axis vertical/horizontal glides absent (T(2,0)/T(0,2) are not
  tiling translations). Standard trio + brute-force culling cross-check
  + in-module exact group probe. TUI SVG export sanity-checked
(42 tilings, 267 tests).
- 2026-11-12 — M40 Herringbone paver bond (2:1 dominoes) ✅: the
  chiral single-direction herringbone as a `LatticeTiling` data table
  (`herringbone.rs`, feature `pavers`) — L-pairs `H(a,b) + V(a+2,b)`
  at every point of the oblique lattice ⟨(1,3),(2,2)⟩ (area 4), one
  2-domino cell; each long side split into two unit edges (mid-side
  vertex), so vertex figures carry 180° at the split midpoints
  (`2.4.4`). Verified exactly before porting (Python spike over a
  bounded patch: area sum = |det|, edge parity, vertex figures,
  exact-once coverage, rigorous coset symmetry check); group **pgg**
  as expected for the generic 2:1 case: chiral (no mirrors, no
  4-folds), 16 2-fold centers per 4×4 period, two perpendicular glide
  families — diagonal axes `y = x + c` (reps `(y+1,x+1)`,
  `(y,x+2)`, `(y−1,x+3)`, `(y−2,x+4)`) and anti-diagonal axes
  `y = −x + c` (reps `(1−y,−x)`, `(−1−y,−x−2)`) — with the off-family
  glides `(1−y,1−x)`, `(−x,y+1)`, `(x+1,−y)` absent. Standard trio +
  brute-force culling cross-check + in-module exact group probe.
TUI SVG export sanity-checked (43 tilings, 271 tests).
- 2026-11-13 — M41 Running bond paver bond (2:1 dominoes) ✅: the
  stretcher (running) bond as a `LatticeTiling` data table
  (`running_bond.rs`, feature `pavers`) — rows of 2:1 dominoes offset
  by half a brick, rectangular lattice ⟨(2,0),(0,2)⟩, 2 split
  dominoes per cell (long sides split into two unit edges, 180°
  vertex figures at the split midpoints, `2.4.4`). Verified exactly
  before porting (Python spike over a bounded patch: area sum = |det|,
  edge parity, vertex figures, exact-once coverage, rigorous coset
  symmetry check); group **cmm** as expected: mirrors at every
  integer `x = k` axis and at the half-integer `y = l + 1/2` axes,
  2-folds at `(k, l+1/2)` (on the mirrors) and `(k+1/2, l)` (between
  the rows), two glide families per direction (reps `(x+1,−y)`,
  `(x+1,4−y)`, `(1−x,y+1)`, `(3−x,y+1)`), no diagonal mirrors, no
  4-folds, off-axis glides `(x+1,1−y)`, `(−x,y+1)` absent. Standard
  trio + brute-force culling cross-check + in-module exact group
  probe. TUI SVG export sanity-checked. Paver trio complete
  (44 tilings, 275 tests).
- 2026-11-13 — M42 Hexagram-and-hexagon-and-hexagon ({6/2} star-and-cross) ✅: the
   PROPOSED.md #2 entry as a one-parameter family on the hexagonal
   lattice (`hexagram.rs`, feature `hexagram`) — {6/2} six-pointed
   stars at the lattice points, tip radius T ∈ [0.05, 1/2]
   (default 1/2 = the kissing hexagram-and-hexagon tiling); the
   interstice is one 16-vertex polygon for T < 1/2 and splits into two
   regular hexagons H(C1), H(C2) per cell (area √3/8 each, touching at
   the cell center) at the kissing radius; per-cell area identity exact
   (√3·T² + √3/2 − √3·T² = √3/2); T = 1/3 (tips on the Voronoi ring)
   is the only critical value and does not change the 16-gon
   combinatorics; p6m group probe on the star layer (the interstice is
   its complement); kissing vertex figures 60.120.60.120 /
   60.60.120.120; sweep covers gapped + critical + kissing radii
   (45 tilings, 280 tests).
- 2026-11-13 — M44 2-uniform tiling (3⁶; 3².6², p6m) ✅: first entry
  of the 2-uniform arc (`kuniform-p6m` in `kuniform.rs`, feature
  `kuniform`) — the "1/3-dissected" honeycomb (Wikipedia `2-uniform
  n10`, t = 2, e = 3): the honeycomb with every third hexagon (the
  index-3 sublattice m ≡ n mod 3 of the center lattice
  span{(3/2,√3/2),(0,√3)}) cut into a six-triangle fan; exact Q(√3)
  table on the index-3 translation lattice (3,0),(3/2,3√3/2), two
  hexagons + six triangles per cell (area 9√3/2 = |det|). Verified
  exactly in the Python spike before porting (±3 patch: edge parity
  681 shared × 2 + 108 boundary, 398 vertices / 290 interior with
  figures 3⁶ (49, 1 orbit) and 3².6² (241, 6 translation orbits),
  fine irrational-offset grid exact-once, exact cell-set probes for
  R60/R120 + both mirrors); standard trio + vertex-figure + cell-set
  symmetry tests + culling list green (46 tilings, 285 tests).
- 2026-11-13 — M48 2-uniform tiling (3.6.3.6; 3².6², pmm) ✅: second
  entry of the 2-uniform arc (`kuniform-pmm` in `kuniform.rs`, same
  `kuniform` feature) — the "rows of triangles between rows of
  hexagons" tiling (Wikipedia `2-uniform n11`, t = 2, e = 3):
  flat-top hexagons on the rectangular lattice span{(2,0),(0,√3)}
  sharing full horizontal edges, gaps filled by one down-pointing
  triangle (apex (1,0)) and one up-pointing triangle (apex (1,√3))
  per cell; exact Q(√3) table, one hexagon + two triangles per cell
  (area 2√3 = |det|). Verified exactly in the Python spike before
  porting (±3 patch: edge parity 260 shared × 2 + 68 boundary,
  182 vertices / 114 interior with figures 3.6.3.6 (36, 1 orbit) and
  3².6² (78, 2 translation orbits), fine irrational-offset grid
  exact-once, exact cell-set probes for the three mirrors + two
  half-turns, 9 negative rotation probes R60/R90/R120 about
  (0,0)/(1,0)/(1,√3/2)); standard trio + vertex-figure + cell-set
  symmetry tests + culling list green (47 tilings, 290 tests).
- 2026-11-13 — M49 Pentagon type 13 (Rice) ✅: the canonical
  A = 120° instance of Rice's type-13 pentagon (d = 2a = 2e,
  B = E = 90°) as an exact-Q(√3) 8-tile data table
  (`pentagon13.rs`, feature `pentagon13`) — rectangular lattice
  T1 = (3s, 3s), T2 = (s, −s), s = (9+√3)/13, all 48 vertices on the
  (p + q√3)/26 grid; long side split into two collinear a-edges;
  2-isohedral chiral pair as seeds 0/1; group p2 (pgg with the chiral
  pair distinguished); C2 center at the prototile's d-midpoint.
  Verified exactly before porting (648-tile block: zero overlaps,
  exactly-once coverage, edge parity, 360° vertex figures, pgg/p2
  glide probe); standard trio + unit structure + chirality + C2
  invariance tests, culling cross-check (48 tilings, 297 tests).
- 2026-11-13 — M52 12-pointed girih star interlace panel (W85), p6 ✅:
  the Wichmann data18/W85 panel (Alhambra, Mexuar patio; SPA 1914 / Wade,
  Lee's Kharraqan 1067–8, Bonner's Uljaytu, Topkapı scroll) as an
  **exact-Q(√3)** line-grid data table (`girih_w85.rs`, feature
  `girih_ext`) — hexagonal lattice a1 = (283.92, 0), a2 = rot60(a1),
  strap width 9; six band families at 30°·k with a regular-vertex
  12-pointed star at every lattice point; 42 tiles per cell (star 1,
  kite 12, pentagon 6, triangle 2, rectangle 3, sheared quad 6, reflex
  ink hexagon 12), all vertices on the (p + q√3)/50 grid. Wichmann
  classifies the motif as *632 (p6m), but the drawn over/under interlace
  is **chiral**: the exact Q(√3) symmetry probe confirms rot60 at the
  stars + rot180 at the three lattice-edge midpoints + rot120 at
  (a1+a2)/3 and hold while all six mirror axes and both 30° rotations
  fail — so the drawn face-to-face tiling is **p6** (the only proper
  overgroup of p6 is p6m, ruled out). Verified exactly in the Python
  spike before porting (Fraction-Q(√3) cell areas sum to |det| = 0 error,
  150×150 exactly-once coverage, 1775/1775 side-by-side edge test; ground
  faces alone are p6m 167/167 but ink pieces never mirror-match 0/177);
  standard trio + unit structure + p6 symmetry tests + culling list
  green (49 tilings, 314 tests).
- 2026-11-13 — M53 Hankin polygons-in-contact fourfold khatam (4.8.8
  substrate), p4m ✅: the generative strapwork engine's first deliverable
  (`pic_4_8_8.rs`, feature `hankin_ext`) — Hankin polygons-in-contact at
  contact angle δ = 45° (median opening) on the truncated-square 4.8.8
  substrate. **Tiling mode** (design decision, PROPOSED #14): the
  strapwork region partition is a face-to-face periodic tiling in its own
  right, so it registers directly (M35 precedent) rather than as a render
  layer; layer mode stays the documented follow-up. At δ = 45° every ray
  runs edge-midpoint to edge-midpoint, so the walls are exactly the
  opposite-midpoint chords — the {8/2} two-square octagram (Rub el Hizb)
  inside each octagon, the four midpoint chords inside each substrate
  square — head-to-head at shared midpoints (angle census {45°, 90°,
  135°}, no T-junctions). 14 faces per square `A·Z²` cell (A = 1+√2):
  1 regular octagon (area 1+√2), 1 small square [±√2/4]² (1/2), 4 house
  pentagons (1/8+√2/4), 8 star-tip triangles (1/8), Σ = 3+2√2 = |det|
  **exactly over Q(√2)**; every vertex on the (p+q√2)/4 grid. Exact Q(√2)
  spike before porting (Fraction arithmetic: areas sum to |det| with 0
  error, 90×90 exactly-once coverage, interior-edge pairing, full
  symmetry probe set); group **p4m** verified position-aware (positive:
  rot90 about the square centre and octagon centre, rot180 about both
  edge-contact classes, mirrors y=0/x=0/y=A/2/y=±x; negative: rot90@(A/4,
  A/4), rot180@(A/4,0) and the glide all fail). Standard trio + unit
  structure + p4m symmetry + symmetry-metadata tests, culling cross-check
  green (50 tilings, 320 tests).
- 2026-11-13 — M50 Pentagon type 14 (Stein 1985) ✅: the completely
  determined type-14 prototile (2a = 2c = d = e, b = βa, A = 90°,
  2B + C = 360°, C + E = 180°) as an exact-surds 6-tile data table
  (`pentagon14.rs`, feature `pentagon14`) — coordinates in
  Q(√57, β, p, bq) with β = √((11√57−25)/8), βp = (43−√57)/16
  exact in Q(√57); oblique lattice w1 = (2pq, −(13+9√57)/16),
  w2 = (−(β+2p), −(√57−1)/2), |det| = 6 × tile area; long sides
  split into two collinear unit fine edges (7/5/6-vertex cells);
  3-isohedral classes as seeds 0/1/2; group p2 (2222), C2 center
  O = (β + p/2, q/2). Verified exactly before porting (21600-point
  exactly-once overlay over a ±5×±6 lattice window; surd
  identities to ≤2.2e-16); standard trio + unit structure +
  isohedral-class + C2 invariance tests, culling cross-check
  (51 tilings, 332 tests).
- 2026-11-13 — M54 Hankin polygons-in-contact sixfold khatam (3.6.3.6
  substrate), p6m ✅: the sixfold engine of the same strapwork generator
  (`pic_3_6_3_6.rs`, feature `hankin_ext`) — Hankin polygons-in-contact at
  contact angle **δ = 60°** on the trihexagonal 3.6.3.6 substrate, the
  hexagonal twin of M53. δ is not a free choice here: the ray from a unit
  edge's other end meets this edge's midline at depth ½·tan(90°−δ), and the
  next midpoint's ray closes onto exactly that point only at depth √3/4,
  i.e. only at δ = 60°, where the chord spans two midpoint steps. At δ = 30°
  the chords span four steps (the same lines run backwards, the triangle
  walls never arrive) and at δ = 90° the rays run through the substrate
  vertices (T-junctions). So the walls are exactly the **{6/2} hexagram** —
  `M_i → M_{i+2}` chords in each hexagon — plus the **three midsegments** in
  each triangle, head-to-head on shared midpoints (angle census {60°, 120°}
  and nothing else). Closure, proved exactly: that network *is* a
  trihexagonal tiling of edge ½, so a cell of the substrate's own lattice
  Λ = ⟨(2,0),(1,√3)⟩ (one substrate hexagon per cell) holds **12 faces in
  three seeds** — 4 regular hexagons of side ½ (the hexagram core on the
  hexagon centre plus three closing on the hexagon–hexagon contacts, area
  3√3/8), 6 hexagram tip triangles riding the six midpoints M₀…M₅ (√3/16)
  and 2 midsegment triangles in the two substrate triangles (√3/16);
  Σ = 4·(3√3/8) + 8·(√3/16) = 3√3/2 + √3/2 = **2√3 = |det Λ| exactly over
  Q(√3)**, and all 12 orbit centroids are distinct modulo Λ. Every vertex
  lands on the (p+q√3)/4 grid — in fact on x ∈ Z/4, y ∈ (√3/4)·Z — so the
  table is 12 rows of integer quadruples with no decimal anywhere. Exact
  Fraction-Q(√3) spike before porting (VERDICT: ALL GOOD): exact segment
  splitting at exact intersections, 169+288 interior faces with every
  interior edge shared exactly twice, 32 300 fine-grid probes all covered
  exactly once, corner angles only 60°/120° with all 50 core vertices
  summing to 360°, Σ areas = 2√3 as a field identity with 0 error. A second
  exact script re-checked the shipped table against the spike's face set
  both ways (972 lattice faces, no duplicates, 0 missing, 0 extra). Group
  **p6m** verified position-aware (positive: rot60 at the hexagon centre and
  at a hexagon–hexagon contact, rot120 at a substrate triangle centre,
  rot180 at a midline point, mirrors y=0, x=0, y=√3, x=2 and the 60° mirror
  through O; negative: rot90@O, rot60 at the triangle centre, rot180 at
  (3/8, √3/8) and the mirror y=√3/4 all fail). Standard trio + unit
  structure + p6m symmetry + symmetry-metadata tests, culling cross-check
  green; SVG export parse (text only, never rendered) confirms 2251 tiles in
  exactly two area classes 3√3/8 and √3/16 with 6-gon ⇔ hexagon and
  3-gon ⇔ triangle and 0 of 2816 probes off exactly-once coverage
  (52 tilings, 332 tests).
- 2026-11-13 — M56 Hankin polygons-in-contact twelvefold khatam (3.12.12
  substrate), p6m ✅: the twelvefold engine of the same strapwork generator
  (`pic_3_12_12.rs`, feature `hankin_ext`) — Hankin polygons-in-contact at
  contact angle **δ = 60°** on the truncated-hexagonal 3.12.12 substrate, the
  third member of the PIC family after M53 (fourfold) and M54 (sixfold). δ is
  *derived*, not chosen: the midpoints of a regular n-gon lie on its incircle
  360°/n apart, and a ray that leaves a midpoint at angle δ to the edge leaves
  it at angle δ to the tangent, so by the tangent-chord theorem it cuts that
  circle again after a central angle of exactly 2δ — it arrives head-on at
  another midpoint iff δ = 180°·k/n. The dodecagon allows
  {15°, 30°, 45°, 60°, 75°, 90°}, the triangle only 60°: **δ = 60° is the one
  contact angle that closes both** (δ = 90°, which the dodecagon would accept,
  drives the triangle rays along their normals through the substrate vertices
  — the M54 T-junction lesson). The walls are therefore the 12 chords
  `M_i → M_{i+4}` per dodecagon — the {12/4} star, four overlaid equilateral
  triangles, the classic twelvefold rosette — plus the three midsegments per
  triangle: 18 walls per cell, collinear through the shared midpoints and
  face-to-face (exact splitting of the 882 walls of a 49-cell patch gives
  V = 2266, E = 4410, F = 2146, Euler characteristic 2, T-junctions 0). A cell
  of the substrate's own lattice Λ = ⟨(3/2+√3, 1+√3/2),(0, 2+√3)⟩ (both
  vectors of length 2+√3, 60° apart; one dodecagon + two triangles) holds
  **45 faces in six seeds** — 1 rosette-core regular dodecagon of side ½ at O
  (3/2 + 3√3/4), 6 vertex pentagons each wrapping one 3.12.12 vertex
  (3/8 + 3√3/16), 12 band quadrilaterals (1/8 + √3/12), the 2 midsegment medial
  triangles of the substrate triangles (√3/16), 12 inner quadrilaterals
  (1/16 + √3/48) and 12 star-notch triangles (√3/48); Σ = **6 + 7√3/2 =
  |det Λ| exactly over Q(√3)**. The decomposition is the classical one: the
  midsegments quarter every substrate triangle into four √3/16 pieces — the
  medial one is a face, the three corner ones are pentagon tips — and each
  pentagon is exactly one such quarter plus two dodecagon corner pieces of
  (3+√3)/16, so each dodecagon holds the core, twelve each of seeds 2/4/5 and
  twelve half-pentagon corners = 6 + 3√3, its own area. Every vertex lands on
  the (p+q√3)/24 grid (the exact common denominator of the spike's denominator
  survey) and the corner census is 30°/60°/90°/120°/150° only
  (1176/2058/2232/1878/1176), summing to 360° at every interior vertex. Exact
  Fraction-Q(√3) spike before porting (VERDICT: ALL GOOD): interior edges
  shared exactly twice (4110 interior, 300 patch border), 2500 fine-grid probes
  all covered exactly once, Σ areas = |det| with 0 error. Group **p6m**
  verified position-aware (positive: rot60/rot120/rot180 at the dodecagon
  centre, rot120 at a substrate triangle centre, rot180 at a
  dodecagon–dodecagon midpoint, mirrors y=0, x=0 and the 30° line through O;
  negative: rot30@O, rot90@O, rot60 at the triangle centre, rot180 at the
  dodecagon–triangle midpoint, rot180 at a substrate vertex, and the mirrors
  y=√3/2 and x=1 all fail). Standard trio + unit structure + p6m symmetry +
  symmetry-metadata tests, culling cross-check green; SVG export parse (text
  only, never rendered) confirms 2289 tiles in exactly the six table area
  classes (2.79904 = 3/2+3√3/4 core … 0.03608 = √3/48 notch), vertex histogram
  {12: 49, 5: 320, 4: 1246, 3: 674} in the 1 : 6 : 24 : 14 orbit ratio, and
  all 9194 exported vertices coincident with a table vertex plus an exact
  lattice translation (worst 7e-5 — the SVG's own rounding — and 0 off).
  Deviation from the milestone brief: the 3.12.12 substrate has **no squares**
  (dodecagons and triangles only), so M53's square fills have no counterpart;
  the vertex pentagons and the medial triangles take that role.
  (53 tilings, 338 tests).
- 2026-11-13 — M57 PIC layer-mode strapwork engine ✅: the general
  polygons-in-contact engine behind M53/M54/M56, decoupled from any one
  substrate — `tess_core::strapwork::walls(tiles, δ)` (tess-core, no
  feature gate; layer mode is infrastructure) lays the wall network over
  an **arbitrary** host tiling as a pure f64 function: at every edge
  midpoint two rays enter each adjacent tile at contact angle δ to the
  edge, symmetric about the inward normal; the adjacent tile fires the
  point-reflected pair at the shared midpoint, so walls are continuous
  across edges, straight through at M. Inside a tile each ray runs to
  the boundary exit and is **split at every crossing** with the tile's
  other rays — first-crossing termination would drop the rosette-core
  edges (the chord segments strictly between two crossings of
  `pic-4-8-8`'s {8/2} and `pic-3-6-3-6`/`pic-3-12-12`'s {6/2}, {12/4}
  stars), and per PROPOSED #14 the registered PIC modules are ground
  truth. Guards: degenerate tiles (< 3 vertices or area) and zero-length
  edges emit nothing; near-parallel rays (|d₁×d₂| < 1e-9 on unit dirs)
  give no crossing — this covers the collinear strand ends, whose
  duplicate pieces dedupe on the 1e-6 endpoint grid; events within
  1e-8·scale merge, sub-eps pieces drop. Output is ordered + deduped
  through a BTreeMap: deterministic, no RNG. **Verification anchors:**
  three cross-check tests (`strapwork_layer_mode` in tess-tilings) run
  the engine on the registry substrates and compare the segment set
  against the registered PIC tiling's internal edges (edges shared by
  exactly 2 faces, endpoint-canonical 1e-6 keys): 4.8.8@45° ↔ `pic-4-8-8`
  640 walls, 3.6.3.6@60° ↔ `pic-3-6-3-6` 668, 3.12.12@60° ↔
  `pic-3-12-12` 1030 — zero missing, zero extra, worst endpoint 2e-15
  (≤ 1e-9 bar), length sums equal to 1e-12, both sets invariant under
  window growth. **Rendering:** SVG only this round —
  `export --strapwork <δ>` strokes the walls in the border color at 3×
  the tile-border width (∝ view, round caps); the flag's absence keeps
  the export byte-identical (verified by string diff), overlay `<line>`
  counts 716/1224/1488 on the three substrates vs 0 without. Five
  tess-core engine tests (median-chord census, midpoint crossing,
  no-exit/no-dangle termination, determinism, degenerate skip) + the
  three anchors. **Deferred follow-up:** the TUI/GUI live overlay
  (`App`-level wall stroke) — layer mode is CLI/SVG for now.
  (53 tilings, 346 tests).
- 2026-11-13 — M55 Pentagon type 15 (Mann/McLoud-Mann/Von Derau 2015)
  ✅: the last-discovered convex pentagon type (fixed-shape,
  150°/60°/135°/105°/90°, a = c = e, b = 2a, d = a√2/(√3−1) — the
  dossier's (a+√2)/(√3−1) was a typo, corrected) as an exact-Q(√3)
  12-tile data table (`pentagon15.rs`, feature `pentagon15`) — all
  vertices on the (p+q√3)/2 grid; 12 tiles per rectangular cell
  (four congruent 3-tile blocks); non-edge-to-edge (4 T-split edges
  per cell). Group **pgg** confirmed by an exhaustive exact
  symmetry audit (25 translations + 25 half-turns + 50 glides + 0
  mirrors in a 5×5 window): two perpendicular glide families
  (g1² = w1, g2² = w2), no mirrors, 2-fold centers on the coset
  c0 + (1/2)L, c0 = (√3, −1/2), four per cell; 3-isohedral full-group
  classes (seeds 0/1/2), six p2-subgroup orbits. Verified exactly
  before porting (20400-point exactly-once coverage, 60/60 edge
  gluing, symmetry audit, shipped-template congruence); standard trio
  + unit structure + isohedral-class + pgg-symmetry + window-census
  tests, culling cross-check green (54 tilings, 353 tests).
- 2026-11-14 — M59 Pentagon type 9 (Rice 1977) ✅: a two-parameter
  Rice family (b = c = d = e, 2A + C = D + 2E = 360°) pinned to its
  exact-Q(√3) member A = C = 120°, cos B = 1/7, cos D = 23/98,
  cos E = −11/14, a = 6, b = c = d = e = 14 (area 141√3) as an
  8-tile data table (`pentagon9.rs`, feature `pentagon9`) — all
  vertices on the (Z+Z√3)/7 grid; rectangular cell
  w1 = (−30, 6√3) ⊥ w2 = (−141/7, −235√3/7), det = 1128√3 = 8·area.
  Edge-to-edge at the pentagon level (equal-length quotient census
  a↔a/b↔c/b↔d/c↔c/d↔e/e↔e), vertex stars {A,A,C}, {D,E,E},
  {B,B,C,D}. Group **pgg** confirmed by a gauge-invariant
  automorphism audit of the exact quotient table plus exact
  geometric verification: half-turn c0 = (13, 4√3), glides
  g1 = (M_v, (167/7, −165√3/7)) (axis ∥ w2, g1² = w2) and
  g2 = (M_u, (12, −8√3)) (axis ∥ w1, g2² = −w1), no mirrors (every
  glide-class element shifts half a primitive translation along its
  axis). 2-isohedral (seeds 0/1: {T1,T4,T5,T8} / {T2,T3,T6,T7}).
  Verified exactly before porting (81×71 exactly-once coverage
  sample, complete full-edge gluing + closed vertex stars); standard
  trio + unit structure + edge-gluing census + vertex stars +
  isohedral-class + pgg-symmetry + window-census tests (counts
  61/32/16/4 cross-checked against an independent Python emulation
  of the culling), culling cross-check green (55 tilings, 362 tests).
- 2026-11-15 — M51 Shield tiling (Gähler, dodecagonal 12-fold) ✅: the
  shield hexagon + square + black/white triangle substitution with
  Gähler's R/L matching labels (4 prototiles, 8 labelled kinds;
  inflation θ = (√6+√2)/2, PF Perron θ² = 2+√3) as exact-Q(√2,√3)
  substitution code (`shield.rs`, feature `shield`); children tile
  kinked supertiles (±¼ unit-square notches: shield 6 in, square 4 in,
  black tri 3 out, white tri 2 out + 1 in), 13-tile kinked-shield seed
  (centroid √2/2+√6/3, inradius (1+√3)/2) deflated with per-step
  culling. Verified exactly in Python first: per-kind interior-disjoint
  children + closed unit-edge boundary + area law Σ = θ²·plain ± kinks;
  Perron 2+√3 with x²−4x+1 | char poly; growth 13→31→151→493→1981 and
  the published 10-tile sequence (10, 28, 112, 400, …, 294232);
  dodecagonal frame (12 edge directions, matching window a regular
  dodecagon); honest negative re-confirmed: no local 30° center, so
  12-fold is Bragg/MLD. Python↔Rust rule-table diff ≤ 4e-15 through
  depth 4. 16 shield.rs tests + standard trio + proto census
  (56 tilings, 373 tests).
- 2026-11-15 — M60 Pentagon type 11 (Kershner 1968) ✅: Kershner's
  two-parameter family (2a + c = d = e, A = 90°, 2B + C = 360°,
  C + E = 180°) pinned to the reference construction's B = 156°
  member at c = 1 — the pentagon-census arc's first bounded-precision
  instance (156° is not a surd angle): the prototile is computed at
  runtime from the closed forms r = (2 sin B − cos 2B)/(1 + 2cos 2B − 2 sin B),
  s = −2r cos B − (1 + 2r) sin 2B and shipped as 8 (mirror, anchor,
  rotation) placements (`pentagon11.rs`, feature `pentagon11`);
  a = r = 0.0946641…, b = s = 1.0568030…, d = e = 1 + 2r, closure
  identities pinned to ~1e-12.  Rectangular cell
  w1 = (−2.14330839972…, −1.39188074934…) ⊥
  w2 = (1.62698792605…, −2.50534170388…), det = 7.63429309… =
  8 × area (0.9542866…).  NON-edge-to-edge: every d-side carries one
  c-side + two a-sides (2a + c = d); refined census 1882 full + 210
  split edges (parameters c/d, (c + a)/d up to orientation), vertex
  stars {C,B,B}, {A,A,180°}, {D,D,E}, {C,E,180°} (100 interior
  vertices 24/25/25/26).  Group **pgg** by anchor-pair symmetry
  enumeration on the exact model: half-turns at c0 = (−0.37157…,
  0.76010…) + half-lattice images; glides g1 (axis ∥ w1, g1² = w1)
  and g2 (axis ∥ w2, g2² = −w2), each shifting half a primitive
  period along its axis; no mirrors; g2∘g1 = half-turn about c0.
  2-isohedral (seeds 0/1: {0,2,4,7} / {1,3,5,6}, orbits mixing
  chiralities).  Model re-verified before porting (window counts,
  vertex/edge censuses, symmetry audit — all green); standard trio +
  unit structure + refined-edge census + vertex figures +
  isohedral-class + pgg-symmetry (interior-AABB-filtered exact
  half-turn equality, glide containment with inverses and lattice
  shifts, mirror-absence probes) + window-census tests (counts
  238/213/116 cross-checked against an independent Python emulation
  of the culling), culling cross-check green (57 tilings, 371 tests).

- 2026-11-15 — M62 Pentagon type 12 (Kershner 1968) ✅: Kershner's
  two-parameter family (2a = d = c + e, A = 90°, 2B + C = 360° ⇒
  C = 360 − 2B, D = 270 − B, E = 2B − 180; convex 90° < B < 180°,
  side b the second free parameter) pinned to the B = 150° member
  at c = 1 — a surd angle, so the shipped instance closes to exact
  Q(√3): A = 90°, B = 150°, C = 60°, D = E = 120°, a = 1, b = √3,
  c = 1, d = 2, e = 1 (area 3√3/2); the prototile is runtime-
  computed from the closed forms
  a = (sinB + cos2B)/(2cos2B − 2sinB + 1),
  b = cosB − 2a·cosB − (2a − 1)·sin2B, d = 2a, e = 2a − 1, with
  the surds pinned to 1e-12 (`pentagon12.rs`, feature `pentagon12`).
  8-tile rectangular unit w1 = (−3√3/2, −9/2) ⊥ w2 = (2√3, −2),
  det = 12√3 = 8 × area; 2-isohedral (seeds 0/1: {0,3,4,7} /
  {1,2,5,6}, orbits mixing chiralities). NON-edge-to-edge: every
  d-side splits at its exact midpoint (t = 1/2) into two unit
  halves, all other sides full-side matched (b↔b, c↔c, e↔e);
  refined census over 215 tiles: 620 full + 153 split edges, no
  other T-junctions; vertex figures {90°,90°,180°} 8×,
  {60°,120°,180°} 9×, {120°,120°,120°} 8×, {60°,150°,150°} 10×
  (35 interior vertices). Group **pgg** by exact symmetry audit of
  the quotient table: perpendicular glides g1 = (M1, (0,−3)),
  g1² = w1, g2 = (M2, (−√3,1)), g2² = −w2, g2∘g1 = half-turn about
  c0 = (√3/4, 5/4); no mirrors (22 negative candidates). One
  edge-census pitfall closed: the float64 prototile differs from
  the exact surds by ~1e-15 (sin/cos at runtime), so the census
  window was re-pinned to (−7.45, 13.55) × (−7.6, 11.4) — an exact
  count with minimum boundary gap 0.0888 from any counted midpoint
  (verified in Q(√3)), robust against that float noise. 9
  tess-tilings tests + standard trio (58 tilings, 400 tests).

- 2026-11-16 — M63 Pentagon type 10 (Kershner 1968) ✅: Kershner's
  two-parameter family (a = b = c + e, A = 90°, B + E = 180° ⇒
  C = 180 − B/2, D = 90 + B/2, E = 180 − B; convex 90° < B <
  180°, e/c the second free parameter) pinned to the B = 90° member
  at c = 1: A = B = E = 90°, C = D = 135°, a = b = 2, c = e = 1,
  d = √2 (d² = c² + e²; area 7/2) — isosceles, so the two
  chiralities coincide as vertex sets. 6-tile oblique unit
  w1 = (5,−2), w2 = (8,1), det = 21 = 6 × area; 3-isohedral (seeds
  0/1/2: {0,2}, {1,3}, {4,5}). NON-edge-to-edge: both length-2 sides
  split at their exact midpoints (integer T-junctions); the c, d, e
  sides full-side matched; refined census over 164 tiles: 518 full +
  76 split edges at t = 1/2, no other T-junctions; vertex figures
  {90°,135°,135°} 71×, {90°,90°,180°} 50×,
  {90°,90°,90°,90°} 13× (134 interior vertices). Group
  **cmm** by exact symmetry audit: point group {I, −I, R1(y=x),
  R2(y=−x)} all four realised (104/104), mirror lines x+y = 3k and
  x−y = 7k, genuine glide g = T(5,−2)∘R2 with g² = (7,−7)
  = 3w1−w2 (axis x+y = 3/2, perpendicular offset (3/4,3/4), no
  pure-reflection conjugate), R90 negative (M90@(1,2) unplaced / D0
  placed). Centered rectangular lattice: L = Z(5,−2)+Z(8,1) =
  {x−8y ≡ 0 mod 21} has no orthogonal Z-basis (min orthogonal
  pair det 42), while the index-2 sublattice L' = Z(3,3)+Z(−7,7)
  (3√2 × 7√2, det 42) has centre (−2,5) ∈ L\L' → cmm,
  not pmm; registered as Oblique since w1·w2 = 38 ≠ 0. The task
  premise "p2" is corrected: the tiling has mirrors and a half-turn.
  9 tess-tilings tests (59 tilings, 409 tests).

- 2026-11-16 — M64 Pentagon type 3 (Reinhardt family 3, 1918) ✅: the
  pinned integral member (a,b,c,d,e) = (2,2,1,4,3), A = B = C =
  D = 120°, E = 60° (a = b, d = c + e, B + E = 180°), exact Q(√3),
  area 5√3 — the arc's first 2-tile unit and first chiral one-orbit
  **p2** cell (prototile + half-turn about the d-midpoint; four
  half-turn classes d/2 + Λ/2; no mirrors or glides, 144 negative
  probes). NON-edge-to-edge: only the d = 4 sides split, at their
  exact midpoints between two length-2 (a/b) partner sides. A
  complete det-15 lattice closure search (0 solutions in 1,735,315
  trials) plus the norm-form x²+3y²=120 insolubility and a parity
  lemma proved the classical three-tile 120° type-3 pinwheel admits
  NO periodic cell for this member — the discovery that resolved the
  task (earlier attempts had also been refuting cells against a wrong
  scratch polygon, voiding those negatives). Refined census: 172
  full + 46 split edges (all d-midpoints exact tiling vertices),
  vertex figures {60°,120°,180°} 46× + {120°,120°,120°} 39×.
  9 tess-tilings tests (60 tilings, 409 tests).

- 2026-11-17 — M65 Pentagon type 2 (Reinhardt family 2, 1918) ✅: the
  c = e, d = b, B + D = 180° family pinned to the exact-Q(√3) member
  a = 1 (A = B = D = 90°, C = 120°, E = 150°, b = d = 1 + √3/2,
  c = e = 1/2; area 1 + √3/2, all vertices in Q(√3)) — the arc's
  edge-to-edge variation and its second pentagon-level edge-to-edge
  (after type 9). **4-tile rectangular unit = two chiral pairs**
  (the prototile is chiral; the tiling uses both chiralities — the
  arc's first two-chiral-pair unit); 1-isohedral (seeds 0,0,0,0; the
  two glide classes identify the chiralities: single orbit
  r = (0 1)(2 3), g1 = (0 2)(1 3), g2 = (0 3)(1 2)). Rectangular
  lattice w1 = (1/2, 1+√3/2) ⊥ w2 = (−2−√3, 1), |w1|² = 2+√3,
  |w2|² = 8+4√3, det = 4 + 2√3 = 4·area. Group **pgg** by exact
  quotient-table audit: perpendicular glides g1² = w1, g2² = −w2
  (along-axis half-period shifts +1/2·w1 and −1/2·w2, so no power
  is a pure reflection), g2∘g1 = half-turn about c0 = (1+√3/2, 0)
  — the prototile's B vertex, not a lattice point; all four
  c0 + Λ/2 half-turn classes realised (×9 lattice shifts); no
  mirrors (22 negative probes). Colouring the two chiralities drops
  the group to **p2** (the classical reference's note).
  EDGE-TO-EDGE: refined census over the 276-tile window: 1182 full
  edges, 0 splits (min midpoint boundary margin 0.00144). Vertex
  figures {90°,120°,150°} 237× + {90°,90°,90°,90°} 118× (355
  interior vertices, star-to-boundary margin 0.067). Window census
  115/165/272/281/17 over five fixed windows, cross-checked against
  the independent Python culling emulation. 9 tess-tilings tests
  (61 tilings, 465 tests).

- 2026-11-17 — M72 Pentagon type 7 (Kershner 1968) ✅: the
  b = c = d = e, B + 2E = 2C + D = 360° family pinned to the
  published E = 106° member (Jaap Scherphuis Tiling Viewer N5-07;
  the applet slider defaults to 110°): A ≈ 58.86°, B = 148°,
  C ≈ 132.86°, D ≈ 94.27°, a ≈ 1.0357942, b = c = d = e = 1/2,
  tile area 0.529540774017982. The instance is not surd-pinnable
  (transcendental closure in E), so the prototile is computed at
  runtime: C the unique root in (120°, 180°) of
  F(t) = sin(E−2t) − sin(t+E) − sin(E−t) − sin(E) (bisection to
  1e-3, then ≤ 200 Newton steps), A = C − B/2. **8-tile rectangular
  unit, four direct + four mirror, 2-isohedral** (seeds
  [0,1,1,0,0,1,1,0]; full-group orbits {0,3,4,7}/{1,2,5,6}; cell
  permutations ht = (0 4)(1 5)(2 6)(3 7), g1 = (0 3)(1 2)(4 7)(5
  6), g2 = (0 7)(1 6)(2 5)(3 4)). Lattice computed from the
  construction, not hardcoded: w1 = cell2.p3 − cell0.p0 =
  (1.4422756957517922, 1.02096015812545), w2 = −(cell5.p3 −
  cell3.p1) = (−1.3851382485043073, 1.9567376994827812);
  rectangular, |det| = 4.236326192143856 = 8 × area (the raw
  construction basis had negative det, so w2 ships negated). Group
  **pgg** by exact audit: perpendicular glides g1² = w1, g2² = −w2
  (half-shifts +|w1|/2 and −|w2|/2, so no power of a glide is a
  reflection), g2∘g1 = half-turn about c0 = (1/4, 0); all four
  c0 + Λ/2 half-turn classes verified at ×9 lattice shifts; 22
  negative mirror probes + 2 quarter-turn probes. **EDGE-TO-EDGE**:
  907-tile census 3770 full / 0 split edges; torus 20 edge orbits,
  side pairs a-a:4 b-b:2 b-e:4 c-d:4 c-e:4 d-d:2, 12 vertex orbits
  (eight 3s, four 4s). Vertex stars 816 = 271 × {106°,106°,148°} +
  266 × {94°,133°,133°} + 279 × {59°,59°,94°,148°}. Window census
  398/369/193/1149 at four fixed windows; 1922-tile big patch for
  tile-image membership. Closes the M58 deferral (the M58 bounded
  negative was a grid-pinning artifact plus two search-phase
  accounting bugs — the published instance is off-grid, so the grid
  search could never have found it). 11 tess-tilings tests (62
  tilings, 476 tests).
- 2026-11-17 — M75 taxonomy metadata fixes (errata) ✅: attributions in
  the earlier entries corrected. M60/M62 labelled types 11/12 "Kershner
  1968" and M63 labelled type 10 "Kershner 1968" — Kershner (1968) found
  types 6–8 only; type 10 was found by James (1975) and types 9, 11, 12,
  13 by Marjorie Rice (1976–77). The card names/descriptions
  (pentagon10/11/12/13.rs), the tables above, and the census narrative are
  corrected accordingly; the dated changelog entries above (M60/M62/M63)
  are retained as historical record. Also: the seven quasiperiodic card
  descriptions now lead "Quasiperiodic" (matching the tab label) instead
  of "Aperiodic"; chair, sphinx and P-pentomino moved from Aperiodic to
  Nonperiodic (each prototile also admits a periodic tiling — 2 sphinxes
  → 2×3 parallelogram, 2 L-trominoes → 2×3, 2 P-pentominoes → 2×5; the
  category.rs rule); the turtle card now states the shipped tiling is the
  chiral hats-in-turtles variant (homochiral, arXiv:2305.17743), as
  distinct from the plain turtle tiling (arXiv:2403.01911), which mixes
  reflected copies; the two girih star-cross cards now self-identify in
  title (periodic vs quasiperiodic). No registry or geometry changes:
  Aperiodic 7→4, Nonperiodic 1→4, 62 tilings, no new tests.
- 2026-11-17 — M76 gallery taxonomy redesign ✅: (1) new
  `LatticeKind::CenteredRectangular` (fifth lattice kind) + legality
  test pinning declared (group, lattice kind) against the 17-group
  table with basis-vector-only lattice measurement (hexagonal →
  orthogonal → centered-rectangular probes, tol 1e-6); re-declares
  pentagon-10 (3,3)/(−2,5), running-bond (2,0)/(1,1) + 1-domino cell
  (√2-degenerate-square exception), girih-shesh-band (p2, centered),
  girih-star-cross (pmm → cmm, centered, (77,45)/(0,90) basis kept —
  a primitive cell of the centered lattice), elongated-triangular,
  prismatic-pentagonal → centered, herringbone basis normalized to
  (2,2)/(1,−1); (2) `Tiling::secondary_categories()` multi-tab
  membership — triangle/square/hexagon also under Laves, nine tilings
  under the new Wallpaper group-catalog tab (9 of 17 groups; shared
  gap note names the 8 absent: p1, pm, pg, cm, pmg, p3, p3m1, p31m);
  the 17th group is pmg, not cmc (Cmc is a 3D space group — the
  misnamed enum variant renamed Cmc → Pmg); (3) `cairo-collinear`
  merged into `cairo-pentagonal` ("Cairo Pentagonal (V3.3.4.3.4)" +
  collinear-variant sentence) — 62 → 61 tilings; (4) browse gallery
  restructured to ten tabs (Uniform 11, Two-Uniform 2, Laves 11,
  Wallpaper 9, Pentagon Types 10, Islamic 9, Arrangements 5,
  Quasiperiodic 8, Aperiodic 4, Nonperiodic 4; 73 memberships),
  pinned in both UIs with word-boundary 2-line card titles, number
  keys 1–9/0, `section_key(tab_category, card)`, and the headless
  no-clipping geometry test at three window sizes (mutation-checked).
- 2026-11-17 — M77 Pentagon type 8 (Kershner 1968) ✅: the
  b = c = d = e, 2B + C = D + 2E = 360° family pinned to the exact
  Q(√3) member B = C = 120°: tan A = 3√3 (A ≈ 79.11°),
  cos D = 1/7 (D ≈ 81.79°), tan E = −√3/2 (E ≈ 139.11°),
  A = E − 60°, a = 6√7, b = c = d = e = 14, tile area 189√3.
  **8-tile rectangular unit, four direct + four mirror,
  2-isohedral** (seeds [0,1,1,0,0,1,1,0]; full-group orbits
  {0,3,4,7}/{1,2,5,6}, each orbit mixing both chiralities; cell
  permutations ht = [3,2,1,0,7,6,5,4], g1 = [4,5,6,7,0,1,2,3],
  g2 = [7,6,5,4,3,2,1,0]). Lattice w1 = (36, 24√3),
  w2 = (−36, 18√3), exactly perpendicular, |det| = 1512√3 =
  8 × area. Group **pgg** by exact audit: perpendicular glides
  g1² = w1, g2² = −w2 (axis shifts +|w1|/2 and −|w2|/2 exactly;
  perpendicular parts (−4, 2√3) and (39, 26√3) nonzero — no power
  of a glide is a reflection); g1 maps T1 exactly onto T5, g2 maps
  T1 exactly onto T8; half-turn centre c0 = (−1/2, 25√3/2)
  (lattice coords (7/24, 11/36)); 22 negative mirror probes +
  2 quarter-turn probes. **EDGE-TO-EDGE**: 79-tile census
  395 full / 0 split edges, 290 interior directed edges each
  paired with its reverse, 148 vertices, no T-junctions; torus
  20 edge orbits, side pairs a-a:4 b-c:4 b-d:4 c-c:2 d-e:4 e-e:2
  (all reverse-glued), 12 vertex orbits (eight 3s, four 4s).
  Vertex stars 91 = 30 × {A,A,C,D} + 36 × {B,B,C} + 25 × {D,E,E},
  every star 360°. Window census 79/42/20/5 at four fixed
  windows. The exact-Q(√3) member of the oldest Kershner family
  lands the second Kershner after type 7. 11 tess-tilings tests
  (62 tilings, 485 tests).
- 2026-11-17 — M74 Pentagon type 6 (Kershner 1968) ✅: the
  a = d = e, b = c, A = 2C, C + E = 180° family pinned to its
  unique 30°-grid member (A = B = D = E = 120°, C = 60°,
  a = d = e = 1, b = c = 2, tile area 7√3/4; exact Q(√3) pin
  (0,0), (1,0), (2,√3), (0,√3), (−1/2,√3/2)). **4-tile oblique
  unit, 2-isohedral** (seeds [0,0,1,1]; full-group orbits
  {T0,T1}/{T2,T3}; half-turns permute cells 0 ↔ 1, 2 ↔ 3).
  Lattice w1 = (1, 2√3), w2 = (−3, √3), |det| = 7√3 = 4 × area
  (|w1|² = 13, |w2|² = 12, w1·w2 = 3). Group **p2** by structure +
  complete mirror search: half-turns one coset c0 + Λ/2 with
  c0 = (1/2, 0); 22-ray enumeration 0 genuine, the six
  edge-direction mirror candidates and R60/R90/R120 all fail to
  normalize Λ (a block-image check alone admits four spurious
  subset maps, so the normalizer condition is load-bearing).
  **EDGE-TO-EDGE**: all 20 stored sides reverse-glued exactly once
  (involution with λ′ = −λ; 4 in-unit pairs + 6 pairs across
  ±w1, ±w2, ±(w1−w2)); side pairs a-a:1 a-d:2 b-b:1 b-c:2 c-c:1
  d-e:2 e-e:1; 20 directed sides → 18 torus classes, one
  double-covered arc (2,√3)→(0,√3) ≡ (3,0). 787-tile census:
  3935 directed full edges all unique, 3424 interior directed
  edges each paired, 1286 vertices, 1022 interior; vertex stars
  1022 = 340 × {60°,60°,120°,120°} + 682 × {120°,120°,120°} (torus
  6 orbits, sizes 3,3,3,3,4,4). Window census 3268/864/112/20 at
  four fixed windows. Achiral prototile, congruent to the
  floret-pentagonal tile — one pentagon, two tilings (p2 2-iso vs
  the floret's p6 1-iso). 12 tess-tilings tests (63 tilings,
  497 tests).
- 2026-11-17 — M78 Stepped hexominoes (p1) ✅: the asymmetric
  2-step staircase hexomino (unit squares (0,0),(0,1),(1,1),(1,2),
  (2,1),(2,2)) tiled by its own translation lattice Λ = ⟨(3,1),
  (0,2)⟩ — det 6 = area, 1 tile per cell, 1-isohedral,
  edge-to-edge under the collinear-vertex convention (180° at the
  four interior corners) — the catalog's first **p1** realizer
  (10 of the 17 groups now represented). Bravais kind
  **centered-rectangular**, declared on the *primitive* cell (same
  convention as girih-star-cross): the centered cell
  ⟨(0,2),(−6,0)⟩ is a 6×2 rectangle (area 12 = 2·det) whose center
  (−3,1) = −w1 + w2 is a lattice point. The early "oblique" guess
  was wrong — `measured_kind`'s stage-3 probe (|u×v| = 2|det|,
  u ⊥ v, (u+v)/2 ∈ L) returns CenteredRectangular and the legality
  test pins the declaration; the Python `bravais_kind` used during
  search had the centered test inverted (it asked for |u×v| =
  |det|/2 instead of 2|det| — impossible for u, v ∈ L) and so
  mislabeled the winners as oblique. Group **p1** by exact probe:
  the normalizer of Λ is {I, R180, Mx, My} and the asymmetric
  staircase shape-breaks all three non-identity members (R180/Mx/My
  map the hexomino onto a different shape — exact vertex-set
  test), R90/R270/Md/Ma do not normalize Λ, all seven coset
  candidates False, and an independent 12-tile patch carries no
  non-identity symmetry. Structural note: the columns x ≡ 2 (mod 3)
  are complete vertical tile-boundary lines (the tiling decomposes
  into width-3 bands) — (3,0) ∉ Λ, so no two bands are congruent
  and p1 is unaffected. The brief's det-3 L-triomino candidate is
  flawed at the partition level: the three candidate anchor values
  k, k−1, k+2 ≡ k, k−1, k−1 (mod 3) hit only two residue classes —
  k=1 cells double-covered, k=2 cells uncovered (verified:
  coverage census {0: 192, 1: 192, 2: 192} over [−12,12)²); the
  complete det-3/6/9/12/18 triomino sweeps (det-3: 4 lattices × 5
  orientations, none e2e-p1 — the only e2e det-3 triomino tiling is
  straight-I on (3,0),(0,1), which is p2mm) and the det-5
  pentomino sweep (0 p1) leave this area-6 staircase as the minimal
  single-orbit p1 — found in the full area 4–8 hexomino search
  (72 p1 winners = 2 essentially distinct tilings × 4 normalizer
  variants; the shipped one is winner 2, its T-B sibling sits on
  (1,3),(0,6)). 4 tess-tilings tests (standard trio + exact p1
  probe). Also fixed the stale Gallery-taxonomy block, which
  pre-dated M74/M77/M78 (9 Wallpaper / 10 Pentagon Types / 73
  memberships / 61 tilings) — updated to the at-commit census
  (10 / 12 / 76 / 64). 64 tilings, 501 tests at the commit.
- 2026-11-17 — M79 Trapezoid courses (pm) ✅: two courses of
  trapezoids per 2×1 cell on the rectangular lattice
  Λ = ⟨(2,0),(0,1)⟩ (det 2 = area) — four tiles, two incongruent
  trapezoid classes (areas 6/10 and 4/10, slant slope ±1/5,
  intercept 3/10) each with its mirror in x = 1 — the catalog's
  first **pm** realizer (11 of the 17 groups now represented).
  Group pm by exact probe: the mirror lines are exactly the
  vertical lines x = k (x = 1 pairs P ↔ mP, P' ↔ mP' within one
  cell); no horizontal mirror or half-turn — the y = 1/2 + n
  centers are area-impossible (6/10 → 4/10 course) and the
  y = 1 + n centers map a trapezoid to a slant-reversed copy that
  is not a tile — the incongruent prototile classes (c = 3/10 ≠
  (1 − m)/2 = 2/5 with slope m = 1/5) are the structural killer,
  which is what keeps this out of pmm. Edge-to-edge (every edge
  paired exactly twice, no T-junctions, W = 2/3/5); vertex stars
  {78.6901° × 2, 101.3099° × 2} and {90° × 4}, all summing to
  360°. 4 tess-tilings tests (standard trio + exact pm probe,
  15 positive / 53 negative candidates mirroring m79_verify.py).
  Census: 64 → 65 tilings, 76 → 77 memberships, Wallpaper tab
  10 → 11, gap table 7 → 6 (pm drops out of
  `WALLPAPER_GROUP_GAPS`; the shared gap note now names six
  groups). 65 tilings, 505 tests at the commit.
- 2026-11-17 — M80 Glide hexagon strips (pg) ✅: two horizontal
  hexagon strips per 1×2 cell on the rectangular lattice
  Λ = ⟨(1,0),(0,2)⟩ (det 2 = area) — four tiles, two shape classes
  (fat, convex, area 29/40 and waist, 2 reflex vertices, area 11/40;
  A ≅ T and B ≅ U chiral twins, all four sharing the squared-edge
  multiset {1/4, 1/4, 1/5, 2/5, 5/16, 5/16}) — strip 1 the glide
  image of strip 0 under g(x,y) = (x+1/2, −y), g² = (1,0) — the
  catalog's first **pg** realizer (12 of the 17 groups now
  represented). Group pg by exact probe: the only non-translation
  symmetries are horizontal glides of one family (glide 1/2) on two
  parallel orbits, y ∈ 2ℤ and y = 1+2ℤ (Φ = T_{(−1,2)}∘g = g∘(−1,−2)
  is the y = 1 glide, Φ² = (−1,0)); no horizontal mirror or other
  glide (the half-glide s ≡ 1/2, 2c ≡ 1 is the only survivor of the
  finger-tip case split and is killed by the elbow (−1/4,1/2) ↦
  (1/4,1/2) not being a vertex), no vertical mirrors (the
  half-unit edge pattern on y ∈ ℤ forces 2c ∈ (1/2)ℤ while the
  finger tip forces 2c ≡ 7/5 (mod 1)), no half-turns (both center
  classes b ≡ 0 and b ≡ 1/2 (mod 1) force 2a ≡ 9/10 (mod 1) from
  the finger tip landing on level 3/5, vertices only at x ≡ 1/5 + ℤ,
  against 2a ∈ (1/2)ℤ), diagonal and 90°/270°/120°/240° parts
  do not normalize Λ. Edge-to-edge (396-tile window census: 352
  interior edges each paired twice, 868 vertices, no T-junctions,
  clipped coverage 72 = 72); five edge directions mod 180° (0°,
  arctan 2, arctan 3, 180°−arctan 2, 180°−arctan 3), no verticals;
  265 interior vertex figures all sum to 360°. 4 tess-tilings tests
  (standard trio + exact pg probe, mirroring m80_verify.py: 74
  positive / 62 exact-coset negative + 6 non-normalizing parts).
  Census: 65 → 66 tilings, 77 → 78 memberships, Wallpaper tab
  11 → 12, gap table 6 → 5 (pg drops out of
  `WALLPAPER_GROUP_GAPS`; the shared gap note now names five
  groups). 66 tilings, 509 tests at the commit.
- 2026-11-17 — M81 Shifted trapezoid courses (pmg) ✅: the §29
  (M79) trapezoid-course row stacked in rows of height 1 — row 0
  (y ∈ [0,1]) is the M79 pm row, row 1 (y ∈ [1,2]) its mirror
  F(x,y) = (x, 2−y) shifted by (1,0) — eight tiles per 2×2 cell on
  the rectangular lattice Λ = ⟨(2,0),(0,2)⟩ (det 4 = area), two
  congruence classes (P-class {P,mP,R0,R2} doubled area 120,
  squared edges {25,49,100,104}; P'-class {P',mP',R1,R3} doubled
  area 80, squared edges {9,25,100,104}) — the catalog's first
  **pmg** realizer (13 of the 17 groups now represented). Group
  pmg by exact 128-coset probe: exactly 4 positive cosets — the
  identity lattice, vertical mirrors x = k (x = 1 pairs P ↔ mP,
  P' ↔ mP' with no translate), the horizontal glide family
  g(x,y) = (x+1, −y) on two Λ-orbits (y ∈ 2ℤ and y = 1+2ℤ; glide
  1, g² = (2,0) ∈ Λ; the y = 2 glide maps row 0 → row 1 exactly),
  and half-turns about every center (m+1/2, n) for all m,n ∈ ℤ
  (two Λ-orbits, even-y and odd-y, exchanged by mirror x = 1); no
  horizontal mirrors (F(P) = R0+(−1,0) is half a period off Λ —
  what keeps it pmg and not pmm), and the rot90/rot270/diag/anti
  diagonal parts all fail to normalize Λ (square-metric lattice),
  so no p4/p4m/p4g/cmm upgrade. Edge-to-edge (window [−4,4]²: 188
  interior edges each paired exactly twice, no T-junctions among
  448 vertices, clipped coverage 6400 = 6400, no overlaps among
  180 clipped tiles; slants at the 0.3/0.5 and 1.5/1.7 levels pair
  within rows, flat-flat seams at y ∈ ℤ pair row 1's bottoms with
  row 0's tops across the cell boundary). Vertex stars {78.6901°
  × 2, 101.3099° × 2} (56 slant-endpoint vertices, 4 orbits mod 2)
  and {90° × 4} (49 seam vertices), all 105 interior figures
  summing to 360°. 4 tess-tilings tests (standard trio + exact pmg
  probe mirroring m81_verify.py: 128 coset checks, 4 positive /
  124 negative). Census: 66 → 67 tilings, 78 → 79 memberships,
  Wallpaper tab 12 → 13, gap table 5 → 4 (pmg drops out of
  `WALLPAPER_GROUP_GAPS`; the shared gap note now names four
  groups). 67 tilings, 513 tests at the commit.
- 2026-11-17 — M82 Wavy courses (cm) ✅: zigzag courses of height 4/3
  between phase-shifted period-2 waves (slopes ±1/3, peaks 1/3 above
  the valleys; Z(t−1) + 1 = 4/3 − Z(t) stacks the courses exactly),
  each column-course quadrilateral cut by a diagonal seam alternating
  slope ±1 with column-course parity (the alternation is what makes
  the vertical mirrors symmetries — the M82 brief's original
  all-slope-+1 seam construction had no mirror at all and was
  re-derived before landing) — 4 tiles per primitive cell of
  Λ = ⟨(1,1),(2,0)⟩ (det 2 = the x+y-even lattice of ℤ², also
  square-metric — the declared centered-rectangular kind is admitted
  by the degenerate-realization clause), two incongruent congruence
  classes (C1 = {T_low, T_low'} ×3 doubled area 6, squared edges
  {4,10,18} ×9; C2 = {T_high, T_high'} doubled area 12 ×3,
  {10,16,18} ×9 — class mixing impossible by area). The catalog's
  first **cm** realizer (14 of the 17 groups now represented). Group
  cm by exact 144-coset probe: exactly two positive cosets — the
  identity lattice and the single mirror/glide coset (vertical mirrors
  x = integer, all of them, together with the vertical glides
  (2j+1−x, y+1) on the half-integer axes, glide (0,1), g² = (0,2) ∈ Λ
  — (1,1) ∈ Λ puts the glide and mirror representatives in one
  coset); no half-turns, no horizontal mirrors or glides, no
  rot90/270 or diagonal/anti-diagonal parts (edge directions ±1 and
  ±1/3 close under s ↦ −s but not s ↦ 1/s), and the translation
  subgroup is exactly Λ. Edge-to-edge (window [−4,4]²: 124 interior
  edges each paired exactly twice, no T-junctions among 121
  vertices, clipped coverage 576 = 576, no overlaps among 200 clipped
  tiles, 0 gaps on a 48×48 fine grid). Vertex stars {45° × 4,
  arctan(1/2) × 2, arctan(2) × 2} (25 eight-tile stars, integer-y
  vertices) and {90° ± arctan(1/3), each × 2} (28 four-tile stars,
  third-y vertices), all 53 interior figures summing to 360°. 4
  tess-tilings tests (standard trio + exact cm probe mirroring
  m82_verify.py: 144 coset checks, 2 positive / 142 negative).
  Census: 67 → 68 tilings, 79 → 80 memberships, Wallpaper tab
  13 → 14, gap table 4 → 3 (cm drops out of
  `WALLPAPER_GROUP_GAPS`; the shared gap note now names three
  groups). 68 tilings, 517 tests at the commit.
- 2026-11-18 — M83 Chiral-cut honeycomb (p3) ✅: a chiral 3-armed Y
  carved into a side-2 regular hexagon from the alternating
  vertices V0, V2, V4 — each arm V_i → W_i stops at
  W_i = mid(V_i, O) + delta (delta = (−1/8, −1/10), rotated 120°
  with each arm) and the inner seams W_i → O close the Y — 3
  congruent hexagonal tiles per hexagonal primitive cell
  Λ = ⟨(2√3,0),(√3,3)⟩ (det 6√3 = 3 × area 2√3), one chiral
  congruence class (tile1 = R120(tile0), tile2 = R240(tile0)
  exactly), construction transcribed from the HX-STUDY handoff
  `hx_p3_cell.json` (PROPOSED §21) via the scratch emitter (no
  hand-typed coords). The catalog's first **p3** realizer (15 of
  the 17 groups now represented). Group p3 by exact ×320-integer
  probe: the only positive cosets of the 11 D6 linear parts are
  the zero coset for R120 and R240 (candidate census 27/18/18/
  27/18 and 30 × 6), rot60/180/300 and all six reflection
  families negative, 25/25 lattice translations + 0/6
  off-lattice, and the rot120 centres form exactly three orbits
  c(i,j) = (√3·i, i+2j) (reps (0,0), (√3,1), (2√3,2);
  c − R120·c ∈ Λ). Edge-to-edge (450 interior edge pairs, 0
  unpaired, 0 T-junctions; clipped coverage 4096 = 4096, 0
  overlaps, 0 gaps). Vertex stars: 170 figures (25 hexagon
  centres + 35 odd corners {120° × 3} × 60, even corners
  {≈58.8079° × 3, ≈61.1921° × 3} × 35, arm tips {≈177.1677°,
  ≈182.8323°} × 75), all summing to 360°. 4 tess-tilings tests
  (standard trio + exact p3 probe mirroring m83_verify.py: the
  11-part candidate census, R120/R240 positive in the zero coset
  only, 25/25 translations, 0/6 off-lattice, the three centre
  orbits, the structural R120/R240 tile cycling, and the cell
  arithmetic). Census: 68 → 69 tilings, 80 → 81 memberships,
  Wallpaper tab 14 → 15, gap table 3 → 2 (p3 drops out of
  `WALLPAPER_GROUP_GAPS`; the shared gap note now names two
  groups). 69 tilings, 521 tests at the commit.
- 2026-11-18 — M84 Split-triangle grid (p31m) ✅: a side-2 triangular
  grid in which every small equilateral triangle is split by its
  orbit triangle (a generic Q(√3) point P0 near a vertex, rotated
  120° about the triangle's centroid to P1, P2) into an inner
  equilateral triangle + 3 corner quads; the cell's down-triangle
  takes the mirror image of the same construction — 8 tiles per
  hexagonal primitive cell Λ = ⟨(2,0),(1,√3)⟩ (det 2√3 = 2 × area
  √3), two congruence classes (inner triangles achiral seed 0,
  corner quads chiral seed 1), construction transcribed from the
  HX-STUDY handoff `hx_p31m_cell.json` (PROPOSED §21) via the
  scratch emitter (no hand-typed coords). The catalog's first
  **p31m** realizer (16 of the 17 groups now represented). Group
  p31m by exact ×240-integer probe: the only positive cosets of the
  11 D6 linear parts are the zero cosets of rot120, rot240, refl0,
  refl60, refl120 (candidate census 43 × 5 / 28 × 6), rot60/180/
  300 and the 30°/90°/150° reflection families negative, 25/25
  lattice translations + 0/5 off-lattice, the rot120 centres form
  exactly three orbits c(i,j) ×240 = (240i, 0, 0, 80(i+2j)) (reps
  (0,0), (1,√3/3), (2,2√3/3); c − R120·c ∈ Λ), and the on-mirror
  incidence separates p31m from p3m1 (grid-vertex orbit on mirrors,
  both centroid orbits off — [true,false,false]). Edge-to-edge (750
  interior edge pairs, 0 unpaired, 0 T-junctions; clipped coverage
  4096 = 4096, 0 overlaps, 0 gaps). Vertex stars: 186 figures (grid-
  vertex stars ≈18.9442°×6 + ≈41.0558°×6, inner-triangle-vertex
  stars 60°, ≈126.0665°, ≈173.9335°), all summing to 360°. 4
  tess-tilings tests (standard trio + exact p31m probe mirroring
  m84_verify.py: the 11-part candidate census, five positive parts
  positive in the zero coset only, 25/25 translations, 0/5
  off-lattice, the three centre orbits + incidence, and the cell
  arithmetic). Census: 69 → 70 tilings, 81 → 82 memberships,
  Wallpaper tab 15 → 16, gap table 2 → 1 (p31m drops out of
  `WALLPAPER_GROUP_GAPS`; the shared gap note now names one group,
  p3m1). 70 tilings, 525 tests at the commit.
 - 2026-11-18 — M85 Alcove-cut grid (p3m1) ✅: a side-2 triangular
  grid on the index-3 sublattice Λ = ⟨(3,−√3),(3,√3)⟩ (det 6√3) in
  which every grid triangle is cut from its alcove point — the orbit
  of the generic Q(√3) junction P0 = (8/7, 4√3/9) under the
  edge-line reflections of the up-triangle (six alcove points per
  cell) — to its three vertices: 18 tiles per hexagonal primitive
  cell, three congruence classes (six tiles each — the three piece
  shapes, each realized in up- and down-triangle orientations; each
  class mirror-closed, achiral as a set), construction transcribed
  from the HX-STUDY handoff `hx_p3m1_cell.json` (PROPOSED §21) via
  the scratch emitter (no hand-typed coords). The catalog's first
  **p3m1** realizer (all 17 of the 17 groups now represented). Group
  p3m1 by exact ×126-integer probe: the only positive cosets of the
  11 D6 linear parts are the zero cosets of rot120, rot240, refl0,
  refl60, refl120 (candidate census 69 × 5 / 42 × 6), rot60/180/
  300 and the 30°/90°/150° reflection families negative, 25/25
  lattice translations + 0/5 off-lattice, the rot120 centres form
  exactly three orbits c(i,j) ×126 = (126(2i+j), 0, 0, 126j) (reps
  (0,0), (2,0), (4,0); c − R120·c ∈ Λ), and the on-mirror incidence
  is the *333 fingerprint that separates p3m1 from p31m (all three
  centre orbits on mirrors — [true,true,true], versus M84's
  [true,false,false]). Edge-to-edge (1350 interior edge pairs, 0
  unpaired, 0 T-junctions; clipped coverage 4096 = 4096, 0
  overlaps, 0 gaps). Vertex stars: 245 figures (alcove junctions
  ≈104.110°, ≈115.519°, ≈140.372°; grid-vertex stars one type per
  centre orbit: ≈26.037°×6+≈33.963°×6, ≈18.073°×6+≈41.927°×6,
  ≈21.555°×6+≈38.444°×6), all summing to 360°. 4 tess-tilings tests
  (standard trio + exact p3m1 probe mirroring m85_emit.py: the
  11-part candidate census, five positive parts positive in the zero
  coset only, 25/25 translations, 0/5 off-lattice, the three centre
  orbits + incidence [T,T,T], and the cell arithmetic). Census:
  70 → 71 tilings, 82 → 83 memberships, Wallpaper tab 16 → 17, gap
  table 1 → 0 (`WALLPAPER_GROUP_GAPS` is now empty; the shared note
  reports the completed catalog). 71 tilings, 529 tests at the
  commit.
- 2026-11-18 — M86 Taylor–Socolar striped hexagons ✅: aperiodic
  decorated-hexagon monotile, the paper's forced 529-hexagon patch
  (arXiv:1008.3103 Fig. 18) as a verbatim state table, each hexagon
  partitioned into 13 convex pieces preserving the edge-midpoint
  handoff exactly (handoff battery R1 0/2992, R2 0/2816, R3 0/2904
  re-derived in tests); no-period probe over 2790 translations to
  hex-radius 30 finds no period; 4 tess-tilings tests (72 tilings,
  538 tests at the commit). (Changelog entry added belatedly with
  the M87 batch — the M86 commit updated §36 but not this list.
  **Decoration superseded** by the 2026-11-19 M86-fix entry below: the
  13-piece stripe/corner-band drawing is gone, the state table is not.)
- 2026-11-18 — M87 2-uniform (3⁶; 3².4.3.4, p6m) ✅: first of the
  three-entry p6m batch of the closed 2-uniform list (`kuniform-n18`
  in `kuniform.rs`, feature `kuniform`) — squares + two triangle
  orbits, the 3⁶ vertices as the lattice origin (a genuine 6-fold
  center); 60° hexagonal basis |b1| = |b2| = 1+√3, |det| = 3+2√3
  (left-handed basis, handled by the generator); 11 tiles per cell
  (3 squares seed 0, 6 triangles seed 1, 2 triangles seed 2), area
  3+2√3 = |det| exact in Q(√3). Python-first verification (±3×±3 =
  49-cell window): edge-to-edge 834 interior pairs / 0 unpaired / 0
  T-junctions, vertex stars 333333 (36 vertices, 1 orbit) + 33434
  (260, 6 orbits), 19600-point grid 9800 exactly-once / 336
  on-boundary / max count 2, R60..R300 invariance 5/5 (group
  determination p6m *632 — an earlier probe's false R60 failure was
  a 30°-vs-60° bug in the probe, not the tiling). 4 tess-tilings
  tests (standard trio + two-orbit vertex-figure test pinning the
  battery's counts and orbit representatives). Census: 72 → 73
  tilings, 84 → 85 memberships, 538 → 542 tests (commit pending the
  coordinator).
- 2026-11-18 — M88 2-uniform (3.4.6.4; 3².4.3.4, p6m) ✅: second
  entry of the p6m batch (`kuniform-n9`) — hexagons ringed by six
  squares with triangle fills; origin a hexagon face center
  (genuine 6-fold rotation center, not a vertex); 60° hexagonal
  basis |b1| = |b2| = 2+√3, |det| = 6+7√3/2 (left-handed); 15 tiles
  per cell (1 hexagon seed 0, 6 squares seed 1, 6 triangles seed 2,
  2 triangles seed 3); 3-fold centers at the golden-triangle
  centroids, 2-fold centers at the triangle–triangle edge
  midpoints. Battery: edge-to-edge 1255/0/0, stars 33434 (234, 6
  orbits) + 3464 (287, 6 orbits), 25600-point grid 14256
  exactly-once / 144 on-boundary / max 2, p6m group determination
  (R60..R300 + six mirrors, 0 color changes). 4 tess-tilings
  tests. Census: 73 → 74 tilings, 85 → 86 memberships, 542 → 546
  tests (landed on main).
- 2026-11-18 — M89 2-uniform (3.4.6.4; 3³.4², p6m) ✅: third entry
  of the p6m batch (`kuniform-n8`) — the second p6m form of
  3.4.6.4 (one square–square edge per cell; 2-folds at the
  square–square edge midpoints, cf. M88's triangle–triangle); same
  basis class as M88 (|b1| = |b2| = 2+√3, |det| = 6+7√3/2,
  left-handed), 15 tiles per cell, origin a hexagon center.
  Battery: edge-to-edge 1261/0/0, stars 33344 (260, 6 orbits) +
  3464 (267, 6 orbits), 25600-point grid 14256 exactly-once / 168
  on-boundary / max 2, p6m group determination. 4 tess-tilings
  tests. Census: 74 → 75 tilings, 86 → 87 memberships, 546 → 550
  tests (landed on main). All three batch entries
  leave the full bar green: cargo check + clippy (both target
  sets) with zero warnings, cargo test --workspace 550/550.
- 2026-11-19 — M90 2-uniform (3.4.6.4; 3.4².6, p6m) ✅: fourth
  entry of the p6m batch (`kuniform-n5`) — hexagons ringed by
  squares, two square orbits + one triangle orbit (t = 5, e = 5);
  18 tiles per cell (3 hex, 9 sq, 6 tri); origin a hexagon center,
  the 6-fold center c the cell's minority-hexagon center (not O);
  60° hexagonal basis |b1| = |b2| = 3+√3, |det| = 9+6√3 — the
  batch's first right-handed basis. Battery: edge-to-edge
  1681/0/0, stars 3446 (266) + 3464 (267) + 3644 (267 — the chiral
  split of the 3.4².6 orbit), 25600-point grid 18492 exactly-once
  / 0 on-boundary / max 1, p6m group determination. 4 tess-tilings
  tests. Census: 75 → 76 tilings, 87 → 88 memberships, 550 → 554
  tests (landed on main).
- 2026-11-19 — M91 2-uniform 3-4-6-12 (4.6.12; 3.4.6.4, p6m) ✅:
  fifth entry of the p6m batch (`kuniform-n1`) — the first tiling
  in the module with dodecagons; 12 tiles per cell (1 doz, 3 hex,
  6 sq, 2 tri); origin a hexagon center, 6-fold center c the
  cell's dodecagon center (not O); 60° hexagonal basis |b1| = |b2|
  = 2+2√3, |det| = 12+8√3 (right-handed). Battery: edge-to-edge
  1386/0/0, stars 4126 (266) + 4612 (266 — the chiral 4.6.12
  orbit, ring labels 1246/1264) + 3464 (267), 25600-point grid
  19044 exactly-once / 0 on-boundary / max 1, p6m group
  determination. 4 tess-tilings tests. Census: 76 → 77 tilings,
  88 → 89 memberships, 554 → 558 tests (commit pending the
  coordinator).
- 2026-11-19 — M92 2-uniform (3⁶; 3².4.12, p6m) ✅: sixth entry of
  the p6m batch (`kuniform-n13`) — dodecagons ringed by squares
  and two triangle orbits around 3⁶ vertices; 16 tiles per cell
  (1 doz, 3 sq, 12 tri, the two triangle orbits tied 6/6 →
  first-appearance seed order); origin a 3⁶ vertex, 6-fold center
  c the cell's dodecagon center (off-ring, float only); 60°
  hexagonal basis |b1| = |b2| = 3+√3, |det| = 9+6√3
  (right-handed). Battery: edge-to-edge 1346/0/0, stars 33412
  (241) + 33124 (240 — the chiral 3².4.12 orbit, ring labels
  12334/12433) + 333333 (84, the 6-fold vertices), 28800-point
  grid 16674 exactly-once / 326 on-boundary / max 2 (grid lines
  coincide with tile edges — expected), p6m group determination.
  4 tess-tilings tests. Census: 77 → 78 tilings, 89 → 90
  memberships, 558 → 562 tests (landed on main).
  All three batch entries leave the full bar green: cargo check +
  clippy (both target sets) with zero warnings, cargo test
  --workspace 562/562 — the six-entry p6m block of the closed list
  is complete (8 of the 20 entries landed, 12 remain).

- 2026-11-19 — M93 Jeandel–Rao 11-tile aperiodic Wang tiling ✅:
  the smallest known aperiodic set of Wang tiles (11 edge-coloured
  unit squares, no rotations/reflections; Jeandel & Rao, arXiv:
  1506.06492) as a **finite-patch data-table port** — the author's
  canonical 100×100 patch (`patch_author_100x100` in the AP-STUDY
  handoff `ap_jr_patch.json`), machine-transcribed by `m93_emit.py`
  (zero hand-copied numbers; the emitter re-checks both batteries,
  the frequency table, the no-period probe and the partition
  geometry against the JSON before emitting). Frame: grid (x, y)
  x-col-right / y-row-down, cell centre `(x − 49.5, 49.5 − y)`, so
  the patch spans [−50, 50]² (10 000 tiles, 50 000 pieces). Edge
  colours are encoded geometrically: each unit cell is partitioned
  into 5 non-overlapping convex pieces — 4 edge bands of width
  W = 0.08 (one documented scale) + the interior — with each W×W
  corner square split by its 45° miter between the adjacent bands;
  band area 46/625, interior 441/625, summing to 1 exactly in Q.
  The 9 distinct edge colours map onto the fixed 8-colour
  tess-core palette by nearest colour (white→0, blue→1, red→2,
  green→3, yellow→4, magenta→5, cyan→6, orange→7; magenta and pink
  merge to 5, and white shares 0 with the interior base — the
  exact 9-colour ids in `TILE_COLORS` keep the batteries merge-
  immune). Batteries re-derived in tests: H 0/9900 and V 0/9900
  edge-color mismatches on the exact ids; tile frequencies match
  the handoff table (sum 10000); no grid translation period
  |t| ≤ 30 (2820 probes, none invariant over an overlap of ≥ 3
  cells); partition areas sum to 1 per cell; coverage trio incl. a
  0.6-scale / 25°-rotated window (both exact-once, 4400 samples);
  empty output beyond the patch. 9 tess-tilings tests. Census:
  78 → 79 tilings, 90 → 91 memberships, Aperiodic 5 → 6, 562 → 571
  tests (landed on the post-M90-M92 base). Full bar green:
  cargo check + clippy (both target sets) with zero warnings,
  cargo test --workspace 559/559 on its fork base. **Scope honesty:** aperiodicity
  and minimality are **cited** (arXiv:1506.06492, machine-verified
  by the authors), not re-proved here; **no generator is ported** —
  the study's independent DFS-grown 24×24 has 0 edge mismatches but
  no 25×25 extension (exhaustive DFS, 27 nodes) while the author's
  24×24 does (226 nodes), so the study's `patch` is a proven dead
  end and the author's 100×100 is the only renderable artifact;
  the band width W = 0.08 is a porting choice for the edge
  encoding, not a claim of the paper. Details in §43.

- 2026-11-19 — M86-fix Taylor–Socolar decoration rework ✅: the owner
  rejected the M86 render against Resnikoff's figures
  (arXiv:1504.06823) — the bands must form **complete closed triangles**
  and the band colouring must be regular — so the invented 13-piece
  stripe/corner-band decoration is replaced by the canonical chord +
  split-axis decoration on the same, unchanged 529-site state table:
  with `kappa = (a + 3·sdt) mod 6` each hexagon carries the chord
  `V_{kappa+1}—V_{kappa−1}` as a straight band of width `W = 0.2·R`,
  plus the diameter `V_kappa—V_{kappa+3}` split at the centre into a red
  half (toward vertex `t = a + 3·(1 − rbt)`) and a blue half. 8 convex
  pieces per hexagon, 12 templates indexed `kappa * 2 + (rbt != sdt)`,
  area sum `3√3/2` exact in Q(√3), 0 overlaps, strict convexity. The
  chord rule is unique in the sense that matters: of the 1296 affine
  rules `kappa = (p·a + q·rbt + r·sdt + s) mod 6`, 125 pair every
  interior chord end (0 of 1058 unpaired), and only `a + 3·sdt` closes
  any ring at all (the other 124 close 0), and the network **closes**
  — 505 interior degree-2 junctions (264 straight-through, 241 at 60°
  corners) + 48 boundary ends, face census 172 cycles whose 74 closed
  rings are all equilateral triangles (60 of side √3, 12 of side 3√3,
  2 of side 7√3 — three nested scales) seen from both sides plus 24
  zero-area boundary chains. The
  Processing sketch's stripe + perpendicular-stub reading was
  **disproved** as closure-incapable (under R1–R3 its chains only ever
  continue straight and stubs are collinear dead ends), so the shipped
  decoration follows the reference chord figures instead — documented as
  a deviation in the module docs and §36, together with the `W/R = 0.2`
  compromise between the two reference images (≈0.19–0.21 vs ≈0.12).
  Tests: the stripe-continuity test is replaced by chord-closure +
  triangle-census tests and the area test by a partition-exactness test
  (convexity, hexagon containment, 0 overlaps, whole-patch 529 × 8
  signature match); the reference test now checks the four emitted
  template placements vertex-exact to 1e−12. Module 9 → 10 tests.
  Census: 79
  tilings and 91 memberships **unchanged**, 571 → 572 tests (landed on the
  post-M93 base). Bar green:
  cargo check + clippy (both target sets) 0 warnings, cargo test
  --workspace 551/551; R1 0/2992 / R2 0/2816 / R3 0/2904 and the
  radius-30 no-period probe untouched and still passing.
- 2026-11-19 — M94 Hankin motif-inference tilings ✅ (§44): the
  inference half of Hankin's PIC method (Kaplan §3.4) — contact-ray
  growth + greedy min-length pairing + planar arrangement to bounded
  faces — as first-class tilings, f64 port of a 40-dps mpmath
  prototype with measured margins and proven greedy tie
  order-invariance; three configurations on three template families:
  `hankin-4-8-8-rosette` (4.8.8, θK = 67.5°, p4m), `hankin-4-6-12`
  (4.6.12, θK = 60°, p6m), `hankin-hex-two-point` (6³, θK = 45°,
  δ = 0.5 two-point, p6m — Kaplan Fig 3.6 row 1 col c). Battery per
  config: pairing complete + tie-invariant, partition exact,
  400-point coverage exact, single-cell torus fold (valences +
  strand closure), group determination on the torus multiset.
  19 tess-tilings tests. Census: 75 → 78 tilings, 87 → 90
  memberships, 550 → 569 tests (landed on main).
- 2026-11-19 — M98 p6m 2-uniform duals (first batch) ✅: the
  face-centroid duals of the first four p6m 2-uniform primals —
  `kuniform-dual-p6m`, `kuniform-dual-n18`, `kuniform-dual-n9`,
  `kuniform-dual-n8` — constructed at runtime by the new `dual()`
  helper (`tess-tilings/src/dual.rs`, research area 2 landed): one
  dual vertex per primal face centroid, one dual edge per primal
  edge, one dual face per primal vertex (surrounding centroids,
  CCW), shared primal lattice, orbit reps taken from the stored
  table vertices (1e6 grid for grouping only). Oracle gate:
  `dual()` on all 7 Archimedean primals reproduces the 7 hand-built
  Laves tables exactly (same lattice, same cell count, bijection up
  to lattice translation + polygon vertex rotation/reversal, max
  vertex residual 0.0). The four duals: p6m → 1 regular hexagon +
  6 congruent quads (Σ = 9√3/2 = |det|); n18 → 1 regular hexagon +
  6 congruent pentagons (Σ = 3+2√3); n9 → 6 congruent quads + 6
  pentagons congruent to the n18-dual pentagons (Σ = 6+7√3/2);
  n8 → 6 quads (same shape as n9) + 6 congruent pentagons, equal
  area but different shape than the n18/n9-dual pentagons (Σ =
  6+7√3/2). All four p6m (R60–R300 + the six mirrors through the
  origin all hold, R30 negative; 6-fold center at the origin),
  edge-to-edge (0 unpaired, 0 T-junctions), convex + CCW per face,
  per-cell Σ face areas = |det| to 1e-9, fine-grid exactly-once
  coverage, proper coloring (seeds < 8). Laves primary,
  `secondary: &[]` (the brief's "Two-Uniform secondary" prose is
  inconsistent with its own census — no secondary). 19 new
  tess-tilings tests incl. the oracle. Census: 79 → 83 tilings,
  91 → 95 memberships, Laves 11 → 15, 571 → 590 tests (commit
  pending the coordinator). Full bar green: cargo check + clippy
  (both target sets) with zero warnings, cargo test --workspace
  590/590. Details in §48.
- 2026-11-19 — M96 2-uniform (3⁶; 3⁴.6, p6) ✅: the two-uniform
  family's first chiral (p6) entry (`kuniform-n20`) — flower form:
  the unit triangular lattice with the 6-face flower at each point
  of an index-13 sublattice L13 = ⟨(1,3),(−3,4)⟩ dissected into a
  regular unit hexagon; 21 tiles/cell (1 hex + 20 tri), 26 lattice
  faces minus the 6-flower; 60° hexagonal basis |b1| = |b2| = √13,
  |det| = 13√3/2 (right-handed); origin a hexagon center = 6-fold
  point. Exact modular arithmetic (kernel k = 4X+3Y mod 13) pins
  A:B = 6:6 = 1:1, two vertex orbits, t = 5. Battery: R60..R300
  about center PASS 5791/5791, all six mirror probes FAIL (73.8%,
  no mirror) + A/B-point mirror probes FAIL → p6; torus census
  V−E+F = 12−33+21 = 0; triangle orbits {6,6,6,2}, edge orbits
  {6,6,6,6,6,3} → **e = 6**, correcting the wiki parenthetical's
  e = 7 (unreliable: also lists e = 1 for the snub hexagonal);
  wiki figure's 4 triangle colors reproduce {6,6,6,2}, 31/38 full
  rings. Vertex-figure test 333333 (266, 6 reps) + 33336 (240, 6
  reps), Python replica m96b_rust_expect.py matches bit-exact. 4
  tess-tilings tests. Census: 78 → 79 tilings, 90 → 91
  memberships, 562 → 566 tests (landed on main).
  Full bar green: cargo check + clippy (both target sets) zero
  warnings, cargo test --workspace 566/566. 9 of the 20 closed-list
  entries landed, 11 remain.
- 2026-11-19 — M97 2-uniform (3².6²; 3⁴.6, cmm) ✅: the two-uniform
  family's first cmm entry (`kuniform-n12`) — rows of edge-sharing
  regular hexagons (consecutive centers √3 apart) with unit-triangle
  bands between the rows; 5 tiles/cell (1 hex + 4 tri), the M97
  spike's 2-hex + 8-tri centered cell recovered exactly as
  S ⊎ (S + c) mod ℤb1 + ℤb2; centered-rectangular lattice declared
  on the primitive basis ⟨(√3,0), (√3/2,5/2)⟩ (the centering vector
  is a basis vector; witness pair (√3,0), (0,5) cross 5√3 = 2|det|
  with center c ∈ Λ), |det| = 5√3/2 (right-handed); origin a hexagon
  center = 2-fold point. Probes on the emitted table: 8 translation
  probes PASS + six 2-fold probes PASS (two translation classes of
  2-fold centers) while the valley 2-folds FAIL (245/245 off) +
  both mirror families PASS (6) + R60/R90/R120 FAIL at every mirror
  intersection → cmm, no 3/4/6-fold; torus census V−E+F = 4−9+5 = 0;
  all triangles ONE orbit (origin 2-fold swaps the visual A/B pairs
  mod Λ) → **t = 2**, edge orbits by adjacency + corner classes
  {hh, ht, tt|346-346, tt|3366-346} → **e = 4** — both match the
  wiki parenthetical (2,4). Vertex-figure test 3366 (78, 2 reps) +
  33336 (84, 2 reps), Python replica m97_rust_expect.py matches
  bit-exact first try; battery 406 edge pairs / 0 interior unpaired
  / 0 T-junctions, fine grid 8000/8000 exactly-once. 4 tess-tilings
  tests. Census: 87 → 88 tilings, 99 → 100 memberships, 618 → 622
  tests (landed on main). Full bar green: cargo
  check + clippy (both target sets) zero warnings, cargo test
  --workspace 622/622. 10 of the 20 closed-list entries landed,
  10 remain.
- 2026-11-19 — M95 2-uniform (3⁶; 3⁴.6)₁ (n19, p6m) ✅: the t = 3
  member of the two-vertex-orbit 3⁴.6 pair (`kuniform-n19`, §50) —
  flower form on the index-9 sublattice L9 = ⟨(3,0),(0,3)⟩ of the
  unit triangular lattice (center spacing 3, one-unit gaps along
  center lines); 13 tiles/cell (1 hex + 6 petals + 6 gap triangles),
  60° hexagonal basis |b1| = |b2| = 3, |det| = 9√3/2 (right-handed),
  origin a hexagon center = 6-fold point; torus V−E+F = 8−21+13 = 0;
  2-uniformity pinned by v mod 3 (kernel L9): 3⁶ = petal-tip classes
  {(1,1),(2,2)} (R60 2-cycle), 3⁴.6 = the six hexagon-corner classes
  (R60 6-cycle), A:B = 1:3; triangle orbits {6,6} + hex → t = 3.
  **The wiki table's p6 label under-states the symmetry:** both
  mirror classes pass exactly (x-axis and 30° mirrors normalize L9
  and map all 13 table tiles to table positions, torus-exact; window
  probes at axes 0°/30°/60°/90° PASS 181/181, mid-axes FAIL 0/181);
  under the full D6 point group the edge orbits are {6,12,3} → e = 3
  (the mirror merges the two C6 petal-edge orbits 6+6 → 12; under p6
  alone e would be 4) — matching the wiki parenthetical; only n20
  (t = 5) is genuinely chiral p6. Battery: edge-to-edge 941 interior
  pairs / 0 unpaired re-pairing at ±4 / 0 T-junctions, fine grid
  1600/1600 exactly-once, vertex figures exact per class (A 3⁶, B
  3⁴.6), translation scan over all 64 Λ candidates norm² ≤ 20 finds
  exactly ±(3,0), ±(0,3), ±(3,−3) (min period norm² 9). Vertex-figure
  test 333333 (72, 2 reps) + 33336 (241, 6 reps), Python replica
  m95x_rust_expect.py matches bit-exact. 4 tess-tilings tests.
  Census (cumulative on main, after M97): 88 → 89 tilings,
  100 → 101 memberships, TwoUniform 10 → 11, 622 → 626 tests
  (landed on main). Full bar green:
  cargo check + clippy (both target sets) zero warnings, cargo test
  --workspace --all-features 626/626. 11 of the 20 closed-list
  entries landed, 9 remain. Details in §50.


- 2026-11-19 — new proposed-queue group "Islamic star-pattern
  extensions (Kaplan dissertation Ch. 3)": eight sub-items 22a–22h
  extracted from reference/kaplan_diss_full_print.pdf §3.4–3.11
  (design elements, Najm template-tiling generator, rosette
  transform, parquet deformations, quasiperiodic substrates +
  (n,c)-monsters + Kepler Aa, absolute-geometry patterns,
  decoration styles, Moroccan/optimization/strange stars), each with
  the chapter's formulas transcribed and the existing M53–M57/M94
  coverage audited. Dossier: PROPOSED.md §22.
- 2026-11-19 — M99 Hankin twelvefold khatam (3.12.12) ✅ (§53): the
  fourth configuration of the M94 inference engine — `hankin-3-12-12`
  (θK = 45°, δ = 0) on the truncated hexagonal 3.12.12 substrate:
  shining twelve-pointed star in each dodecagon (24-gon core + 12
  petals, 13 faces) plus a 4-pointed star with a hexagon tip in each
  triangle orbit (4 faces each); 21 cells, 4 seed orbits, per cell
  ⟨(3/2+√3, 1+√3/2),(0, 2+√3)⟩, det 6+7√3/2 exact in Q(√3). All
  three of θK ∈ {30°, 45°, 60°} pair completely on every tile
  class; 45° chosen as the mid-"shine" choice (90° tips / 150°
  notches; seed-1 core minimum interior angle 90.0° vs 120.0° /
  60.0° at 30° / 60°). No 40-dps Python prototype for this
  substrate: expected-face constants machine-emitted from the Rust
  engine itself (self-consistency pin, documented as a deviation in
  the module header). p6m verified on a ±4-lattice window face
  multiset (1701 faces, all 12 D6 elements, 0 key mismatches at
  1e-6 quantization). 6 new tests (1 engine self-consistency + 5
  registry). Census: 89 → 90 tilings, 101 → 102 memberships,
  Islamic 12 → 13, 626 → 632 tests (landed on main). Full bar
  green: cargo check + clippy (both target sets) zero warnings,
  cargo test --workspace --all-features 632/632. Details in §53.
- 2026-11-19 — M99 Hankin two-point square weave (4.4.4.4) ✅ (§54):
  a fifth configuration of the M94 inference engine —
  `hankin-square-two-point` (θK = 45°, δ = 0.5 two-point) on the
  square 4.4.4.4 substrate: a decagonal star per unit square — four
  corner triangles, two pentagonal petals, one 10-gon core — 7
  cells, 3 seed orbits, per cell ⟨(1,0),(0,1)⟩ det 1. All three of
  θK ∈ {30°, 45°, 60°} pair completely (8 rays, 4 pairs); 45°
  chosen as the mid-"shine" choice (10-gon core, 90° tips; 30°
  flat 8-gon limit, 60° spiky 60° tips), symmetric θK = θE. Two
  honest deviations: no 40-dps Python prototype (expected-face
  constants machine-emitted from the Rust engine, self-consistency
  pin — documented in the module header) and the substrate is
  `square::spec` (Uniform), not the Archimedean family. cmm (2*22)
  verified: the p4m 4-fold hypothesis is falsified (4-fold probes
  at corner / edge-midpoint / cell-center all fail, axis-parallel
  mirrors fail); the two diagonal mirror families x∓y=k pass on two
  instances each, 2-folds on-mirror at (0,0),(0.5,0.5) and
  off-mirror at (0.5,0),(0,0.5); 10-element battery on radius-4
  invariant disks, 0 unexpected outcomes, 1e-6 quantization. 6 new
  tests (1 engine self-consistency + 5 registry). Census: 90 → 91
  tilings, 102 → 103 memberships, Islamic 13 → 14, 632 → 638 tests
  (landed on main). Full bar green: cargo check + clippy (both
  target sets) zero warnings, cargo test --workspace --all-features
  638/638. Details in §54.
- 2026-11-19 — M99: rosette transform (Kaplan diss. §3.8) lands as the
  first registry entry of the §22 arc — `rosette_transform.rs`
  (hankin_ext) + `hankin-4-8-8-rosette-transform` (4.8.8 substrate,
  θK = θE = 45°, δ = 0): 93-tile transformed table (16 triangles, 36
  quads, 32 pentagons, 8 hexagons, one 16-gon per cell), edge-to-edge,
  area census |det|, p4m-exact to 6.5e-16; queue item 22c marked
  landed, §55; 91 → 92 tilings, 103 → 104 memberships,
  Islamic 14 → 15, 651 tests.
- 2026-11-19 — M99 Najm template-tiling generator (roadmap 22b) ✅
  (§56): new module `najm.rs` (feature `najm`, in defaults) —
  notation `([p,q]; m_A o_A, m_B o_B, m_C o_C; inflation)` →
  circumradii via the absolute-trig extent equations (case-3 diad
  closed form, case-4 ABC by monotone bisection, cases 1/2 as a
  sequential min-constraint list; m_B = 1 rejected, every nonzero
  polygon sized exactly once) → substrate cell tables, all
  coordinates derived from the solved radii. Two Islamic entries:
  `najm-63-2e-0-3e-ac` (p6m; dodecagons + enneagons sharing equal
  chords x = 2/(cot 15° + cot 20°) + equilateral bowties {70°×4,
  220°×2}; 6 tiles/cell, areas closing 3√3/2 = |det| to 1e-50; the
  two enneagon orbits carry phases 190°/250° — rot180(B0) witness)
  and `najm-44-3e-0-0-a` (p4m; dodecagons √6−√2 on 2Z² sharing
  chords across odd grid lines + equilateral 4-pointed stars, side
  4−2√3; exact Q(√2,√3), 24−12√3 + 12√3−20 = 4 = |det|). Documented
  deviation: the extracted diss. case-3 formula `T(l) = T(x)/sin(π/n)`
  is missing a factor ½ (l = x/sin(π/n) exceeds the circumradius;
  Fig. 3.21 cannot close) — re-derived l = x/(2 sin), x/(2 tan),
  verified by exact chord coincidence at 60 digits. Spike
  `t22b_spike.py` (mpmath-60 + sympy): both tilings edge-to-edge
  588/588 + 192/192, exactly-once 130²/140² grids, solver fidelity on
  two extra diss. examples. 11 Rust tests (covers/rotated/
  deterministic ×2, radii 1e-14 vs emitted constants, shape/angle
  multisets, area census, primitive-lattice witness). Census:
  92 → 94 tilings, 104 → 106 memberships, Islamic 15 → 17,
  651 → 662 tests (landed on main after the rosette-transform
  landing; the fork's step numbers were on its own base). Full bar
  green on main: cargo check + clippy (both target sets) zero
  warnings, cargo test --workspace --all-features 662/662.
- 2026-11-19 — M100 Hankin sixfold hexagram (3.6.3.6) ✅ (§57): the
  sixth configuration of the M94 inference engine — `hankin-3-6-3-6`
  (θK = 60°, δ = 0) on the trihexagonal 3.6.3.6 substrate: a
  regular {6/2} hexagram per hexagon (12-gon core with 60° tips
  and 240° notches, vertices alternating on circles of radius 1/2
  and √3/2, numerically verified; plus six 60°-lozenge rhombi —
  7 faces); the two equilateral triangle tiles pair with nothing
  (6 rays, 0 pairs) and stay single-face — 9 faces per lattice
  cell ⟨(2,0),(1,√3)⟩, det 2√3 exact in Q(√3). θK = 60° chosen
  over 30°/45° (no star: core tips 120°/90°), 67.5° (sharper star
  variant, probed), and 90° (degenerate: the star core collapses,
  hexagon splits into 6 quads + central triangle); largest minimum
  interior angle (60°) among the star candidates, no slivers. The
  §22a star heuristic does not formally fire at n = 6 (n > 6
  required); the hexagon star first appears at the critical angle
  60° = 2π/6, whose regular endpoint is this configuration. No
  40-dps Python prototype: expected-face constants machine-emitted
  from the Rust engine (self-consistency pin — documented in the
  module header). p6m verified on a ±5-lattice window (1089 faces;
  radius-8 interior disk = 517 faces; all 12 D6 elements; 0 key
  mismatches at 1e-6 quantization). Distinct from
  `pic-3-6-3-6` — same substrate, different construction (greedy
  inference vs. δ = 60° reflection-chord strapwork). 6 new tests
  (1 engine self-consistency + 5 registry). Census: 94 → 95
  tilings, 106 → 107 memberships, Islamic 17 → 18, 662 → 668
  tests (landed on main after the najm landing; the fork's step
  numbers were on its own base). Full bar green: cargo check +
  clippy (both target sets) zero warnings, cargo test
  --workspace --all-features 668/668. Details in §57.
- 2026-11-19 — M100 Hankin sixfold rhombitrihex motif (3.4.6.4) ✅
  (§58): a seventh configuration of the M94 inference engine —
  `hankin-3-4-6-4` (θK = 60°, δ = 0) on the rhombitrihexagonal
  3.4.6.4 substrate: a regular {6/2} hexagram per hexagon (12-gon
  core with 60° tips / 240° notches plus six 60°-lozenge rhombi —
  7 faces); the three unit squares pair with nothing (0 pairs, 1
  face each); each equilateral triangle tile sits in the side-1/2
  4-subdivision limit (its star tips reach the edge midpoints
  exactly — 4 faces each) — 18 faces per lattice cell
  ⟨(1.5+√3/2, √3/2+1/2),(0,1+√3)⟩, det 4√3/2+3 exact in Q(√3).
  θK = 60° chosen over 30° (30° slivers — the hexagon splits into
  six thin triangles), 45° (no hexagon star: a truncated 12-gon
  with 90° tips; the two triangle instances break congruence under
  the greedy tie-break), 67.5° (sharper star, 45° tips), and 90°
  (star collapse): largest minimum interior angle (60°) among the
  star candidates and the only probed angle whose all-motif
  configuration is orientation-invariant (D6-compatible). No 40-dps
  Python prototype: expected-face constants machine-emitted from
  the Rust engine (self-consistency pin — documented in the module
  header). p6m verified on a ±5-lattice window (2178 faces;
  radius-8 interior disk = 553 faces, 319 at R = 6; all 12 D6
  elements, 0 key mismatches at 1e-6 quantization). 6 new tests
  (1 engine self-consistency + 5 registry). Census: 95 → 96
  tilings, 107 → 108 memberships, Islamic 18 → 19, 668 → 674
  tests (landed on main after the hexagram landing; the fork's
  step numbers were on its own base). Full bar green: cargo
  check + clippy (both target sets) zero warnings, cargo test
  --workspace --all-features 674/674. Details in §58.
- 2026-11-19 — M102 k-uniform duals batch 2 (n5/n1/n13/n20) ✅
  (§59): the M98 `dual()` helper dualizes the remaining p6-family
  2-uniform primals — `kuniform-n5`, `kuniform-n1`,
  `kuniform-n13`, `kuniform-n20` — four new face-centroid duals,
  all constructed at runtime, no hand-transcribed geometry. Per cell:
  n5 18 faces (6 + 12 quadrilaterals — two congruence classes of
  the same arity, so the shared face-class census now matches by
  (vertex count, count in cell)); n1 18 (12 30°/60°/90° triangles +
  6 quadrilaterals); n13 14 (2 regular hexagons + 12 quadrilaterals);
  n20 12 (6 regular hexagons + 6 pentagons) — per-class per-face
  areas exact in Q(√3), per-cell Σ = |det| in every case.
  Symmetry measured, not assumed: n5/n1/n13 are p6m with their
  6-fold centers off the origin ((3+√3, 2+2√3) /
  (3(1+√3)/2, (3+√3)/2) / (5(1+√3)/2, 5(3+√3)/2) — R60 about
  the origin fails there), and n20 is chiral p6 (all six mirrors
  fail, matching the primal). The symmetry test was generalized to
  per-case centers — `duals_match_measured_groups`, with the
  lattice-preservation check applied to the map's linear part
  g(x) − g(0). 12 new tests (the per-dual trio); the shared battery
  now covers all 8 duals. Census: 96 → 100 tilings, 108 → 112
  memberships, Laves 15 → 19, 674 → 686 tests (landed on main
  after the hankin-3-4-6-4 landing; the fork's step numbers were on
  its own base). Full bar green: cargo check + clippy (both target
  sets) zero warnings, cargo test --workspace --all-features 686/686.
  Details in §59 (renumbered from the fork's provisional §60 by
  landing order).
- 2026-11-19 — M101 pentagon census coverage audit ✅: docs + 4
  condition tests, 0 new tilings, 686 → 690 tests
  (fork-local on 339e57f)
- 2026-11-19 — M103 2-uniform n4 + n14 (kuniform-n4 cmm,
  kuniform-n14 pmm) ✅ (§60): two more closed-list 2-uniform entries.
  `kuniform-n4` = `[4⁴; 3³.4²]₁ (t=2, e=4)`, cmm: staggered
  two-square-tall bands + alternating-triangle one-row bands, basis
  ⟨(1,0), (1/2, 2+√3/2)⟩, det 2+√3/2, 4 tiles/cell; centered
  rectangular lattice (orthogonal index-2 pair (1,0), (0, 4+√3)).
  `kuniform-n14` = `[3⁶; 3³.4²]₁ (t=3, e=4)`, pmm: square rows +
  double triangle strips, primitive basis ⟨(1,0), (0, 1+√3)⟩,
  det 1+√3, 5 tiles/cell in 3 orbits (wiki-figure colors confirm the
  orbit pairing). Both solved exactly in the tku3 Q(√3) spike
  (coverage 1.0, edge-to-edge, torus orbits reproduce the wiki
  parentheticals (2,4)/(3,4) exactly; full scans give cmm/pmm
  respectively — R180 + both mirror classes, nothing else). The
  brief's stale row labels were corrected against the current wiki
  table before porting (filename↔caption mapping quoted in §60).
  n16 (p4g) stretch: lattice extracted
  (⟨((3+√3)/2, ±(3+√3)/2)⟩, area 6+3√3, noise-tolerant probe) but
  construction not derived — stays open. 8 new tests (4 per entry,
  incl. vertex_figures pinned by the bit-exact rust_expect replica).
  Census: 100 → 102 tilings, 112 → 114 memberships, TwoUniform
  11 → 13, 690 → 698 tests (landed on main after the M101/M102
  landings; the fork's step numbers were on its own base). Full bar
  green: cargo check + clippy (both target sets) zero warnings,
  cargo test --workspace --all-features 698/698. Details in §60.
- 2026-11-19 — M106 k-uniform duals batch 3 (pmm/n12/n19) ✅
  (§61 — renumbered from the fork's provisional §64 by landing
  order): the M98
  `dual()` helper dualizes the remaining non-square 2-uniform
  primals — `kuniform-pmm` (pmm), `kuniform-n12` (cmm),
  `kuniform-n19` (p6m) — the first pmm- and cmm-duals of the family
  and the first to leave the hexagonal 60° basis. Per cell: pmm 3
  faces (1 rhombus + 2 congruent quadrilaterals, all area 2√3/3);
  n12 4 (2 quadrilaterals, area 2√3/3, + 2 pentagons, area 7√3/12);
  n19 8 (6 pentagons, area 7√3/12, + 2 regular hexagons, area √3/2)
  — per-class per-face areas exact in Q(√3), per-cell Σ = |det| in
  every case. The primals' vertex orbits split into more translation
  classes than full-group orbits (3/4/8 per cell), so each dual cell
  carries 3/4/8 faces. Symmetry measured, not assumed: pmm keeps pmm
  (four 2-fold centers, all mirror intersections, mirrors
  horizontal/vertical); cmm keeps cmm (two on-mirror 2-folds, two
  off-mirror glide-only 2-folds at (√3/4,5/4) and (3√3/4,5/4)); p6m
  keeps p6m with the 6-fold center at the origin (the primal's flower
  center). The symmetry test was generalized to non-hexagonal
  lattices: `Case` gained `lattice_kind` and `mirror_dirs`;
  `duals_match_measured_groups` now branches on group — P6m/P6 keep
  the C6 + six-mirror probes; Pmm/Cmm probe R180 about the measured
  2-fold center, negative R30/R60/R90/R120, the two axis-parallel
  mirrors positive, the 45° bisector negative, off-center R180
  negative. 9 new tests (the per-dual trio); the shared battery now
  covers all 11 duals. Census: 102 → 105 tilings, 114 → 117
  memberships, Laves 19 → 22 (19 primary + 3 self-dual secondaries;
  11 machine-dualized k-uniform duals across M98 + M102 + M106),
  698 → 707 tests (landed on main after the M103 landing; the fork's
  step numbers were on its own base). Full bar green: cargo check +
  clippy (both target sets) zero warnings, cargo test --workspace
  --all-features 707/707. Details in §61.
- 2026-11-20 — M108 2-uniform n3 + n15 (kuniform-n3 cmm,
  kuniform-n15 cmm) ✅ (§62): the two sibling forms of the M103 pair.
  `kuniform-n3` = `[4⁴; 3³.4²]₂ (t=3, e=5)`, cmm: staggered
  three-square-tall bands + alternating-triangle one-row bands, basis
  ⟨(1,0), (1/2, 3+√3/2)⟩, det 3+√3/2, 5 tiles/cell; centered
  rectangular lattice (orthogonal index-2 pair (1,0), (0, 6+√3)).
  `kuniform-n15` = `[3⁶; 3³.4²]₂ (t=4, e=5)`, cmm: square rows +
  triple triangle strips (every horizontal edge shared back-to-back,
  one flanker orbit on the square-row lines), basis ⟨(1,0), (1/2,
  1+3√3/2)⟩, det 1+3√3/2, 7 tiles/cell in orbits {square}, {flanker
  pair}, {yellow pair}, {cyan pair} — the wiki figure's four colors
  confirmed as exact orbit classes. Stale brief row indices corrected
  against the saved wiki page before porting (caption-cell mapping in
  §62; the wiki set has no n17). New pitfall solved: the 2×2-cell
  torus orbit quotient splits staggered cells by parity (spurious
  t = 8 on n15) — replaced with the exact affine action on
  lattice-orbit classes; both t/e then match the wiki
  parentheticals (3,5)/(4,5) exactly. One winding fix at port time
  (n3 t_dn, caught by the vertex-figure walk). p4g stretch dropped
  with blocker note: its brief row is the wiki n16 entry, excluded by
  the batch brief; §60's open construction stands. 8 new tests
  (4 per entry, incl. vertex_figures pinned by the bit-exact
  rust_expect replica). Census: 105 → 107 tilings, 117 → 119
  memberships, TwoUniform 13 → 15, 707 → 715 tests (landed on main
  after the M106 landing; the fork's step numbers were on its own
  base). Full bar green: cargo check + clippy (both target sets)
  zero warnings, cargo test --workspace --all-features 715/715.
  Details in §62.
- 2026-11-20 — M107 Hankin elongated-triangular motif (3.3.3.4.4) ✅
  (§63): the eighth configuration of the M94 inference engine —
  `hankin-3-3-3-4-4` (θK = 60°, δ = 0) on the elongated-triangular
  3.3.3.4.4 substrate (cmm, cell ⟨(1,0),(1/2,1+√3/2)⟩, det
  1+√3/2): both triangle tiles in the side-1/2 4-subdivision limit
  (six rays, 3 pairs, four equilateral faces each), the square as a
  four-corner-quadrilateral + central-octagon motif (8 rays, 4
  pairs, 5 faces) — 13 faces per lattice cell, exact Q(√3) (all
  vertices on the a/24 + b√3/96 grid, Σ = 1+√3/2 = det). θK = 60°
  chosen over 30° (30° slivers), 45°/67.5° (sharper 45° minimum
  interior angle), 90° (triangle pairing degenerates — the 3 spokes
  meet at the incenter, 2 pairs / 2 faces; the square star
  collapses to 4 faces); the CW down-triangle and CW square never
  pair (outward rays, 0 pairs at every probed angle) — the all-CCW
  in-cell table is the one that produces a paired motif. No 40-dps
  Python prototype: expected-face constants machine-emitted from the
  Rust engine (self-consistency pin — documented in the module
  header). cmm verified exactly over Q(√3) on a 551-face disk about
  the square center C = (1, H+1/2) (adaptive R² =
  24882743452934203698152347053/10²⁶ — two face centroids at
  exactly distance 5; strict midpoint radius; faces re-canonicalized
  to CCW / minimum-vertex before keying): all 20 cmm elements
  0-mismatch (translations ⟨a1, a2⟩; 2-fold orbits
  {C+L} ∪ {C+a1+L} ∪ {(1/2,H+1/2)+L} ∪ {(±1/4,H/2)+L} ∪
  {(1/4,H/2+1+H)}; mirrors x=k/2 and y=−1/2+k(1+H); glides
  y=H/2+k(1+H) shift a1/2); negative controls fail (rot90 about C,
  rot30/60 about the triangle centroids, off-orbit mirror y = H/2,
  off-orbit glides). 6 new tests (1 engine self-consistency + 5
  registry). Census: 107 → 108 tilings, 119 → 120 memberships,
  Islamic 19 → 20, 715 → 721 tests (landed on main after the M108
  landing; the fork's step numbers were on its own base). Full bar
  green: cargo check + clippy (both target sets) zero warnings,
  cargo test --workspace --all-features 721/721. Details in §63
  (renumbered from the fork's provisional §65 by landing order).
- 2026-11-20 — M104 Golden bee / Ammann chair (A2, inflation √φ) ✅
  (§64 — renumbered
  from the fork's provisional §62 by landing order): the registry's first irreptile — the
  concave chair hexagon (five right angles + one reflex; edges
  {1, s, s⁻¹, s⁻², s⁻³, s⁻⁴}, s = √φ; area s − s⁻⁷) dissecting into two
  similar copies of unequal size (scales φ^−1/2 and φ^−1; area law
  s⁻² + s⁻⁴ = 1). √φ-inflation substitution over 8 orientation classes
  (A0…A3 rotations, B0…B3 mirror family); all 8 rules pinned exact in
  Q(√φ) (integer 4-tuples, s⁴ = s² + 1 — the girih-quasiperiodic
  fixed-point trick): 8/8 two-piece dissections verified slab-exact
  (open-slab even-odd sweeps), growth census d0…d5 (2^d tiles, class
  parities, size histograms, Σ s^−2e = 1 exact) and the intrinsic
  T-junction census 13 (depth 3) / 64 (depth 5), every junction at
  exactly the s⁻⁴ edge fraction (1 − s⁻⁴ = s⁻²), all asserted as exact
  K equalities. Finite covered patch (seed chair = domain;
  Taylor–Socolar precedent) recentered on the patch bbox center (fixed
  float (−1/2, −s/2), 1/2 ∉ Z[√φ] documented), pinwheel-convention
  view-anchored depth (nested views bit-identical within one φ-octave,
  pinned). Category Quasiperiodic — Vince (AMM 121, 2014) proves the
  canonical sequence quasiperiodic (the module implements exactly that
  sequence); DSV (DCG 63, 2020) classifies A2 tilings; the battery
  verifies substitution mechanics, not the diffraction limit (module
  header says so; chair precedent for the declared-strongest
  category). 8 module tests (exact-field battery, two-piece
  dissections, growth census, junction census, nested-view
  bit-identity, standard trio). Census: 108 → 109 tilings, 120 → 121
  memberships, Quasiperiodic 8 → 9, 721 → 729 tests (landed on main
  after the M107 landing; the fork's step numbers were on its own
  base 91dd843). Full bar green: cargo check + clippy (both target
  sets) zero warnings, cargo test --workspace --all-features
  729/729. Details in §64.
- 2026-11-20 — M111 Hankin snub-square motif (3.3.4.3.4) ✅
  (§66, fork-local provisional numbering — renumbered at
  integration by landing order): the ninth configuration of the M94
  inference engine — `hankin-3-3-4-3-4` (θK = 60°, δ = 0) on the
  snub-square 3.3.4.3.4 substrate (p4g, cell ⟨(1+√3,0),(0,1+√3)⟩,
  det (1+√3)² = 4+2√3): each of the four square tiles in its
  five-face motif (four corner quadrilaterals + central octagon —
  8 rays, 4 pairs), each of the eight triangle tiles in the
  side-1/2 4-subdivision limit (6 rays, 3 pairs, four equilateral
  faces each) — 52 faces per lattice cell. Tiles and lattice live
  in Q(√3) with all face vertices in (ℤ+ℤ√3)/4: the exact
  Q(√2,√3) coefficient audit finds the √2 and √6 coefficients
  identically zero (denominator max 4) — √2 enters only as the
  direction of the 45° glide rays, no face coordinate carries it.
  θK = 60° chosen over 30° (30° slivers), 45°/67.5° (sharper 45°
  minimum interior angle), and 90° (triangle pairing degenerates —
  the 3 spokes meet at the incenter, 2 pairs / 2 faces,
  incomplete; the square star collapses to 4 faces); all twelve CW
  rows never pair (outward rays, 0 pairs at every probed angle) —
  the all-CCW in-cell table is the one that produces a paired
  motif. No 40-dps Python prototype: expected-face constants
  machine-emitted from a faithful f64 port of the Rust engine,
  cross-audited exactly in the Q(√2,√3) coefficient ring
  (self-consistency pin — documented in the module header). p4g
  verified exactly over the full field Q(√2,√3) on a 121-cell
  window of 6292 faces (all distinct; each candidate censused
  0-mismatch on its interior square S(c, 2L); faces
  re-canonicalized to CCW / minimum-vertex before keying): the
  face-set translation lattice is the CENTERED lattice
  Λ′ = ⟨(L/2,L/2),(L/2,−L/2)⟩ with L = 1+√3 — translations
  (L,0)/(0,L) and the centered (L/2,±L/2) pass 788/788 while the
  square-lattice half-steps (L/2,0)/(0,L/2) fail 788/788;
  4-fold centers {(L/4, L/4+1/2), (L/4, 3L/4+1/2), (3L/4,
  L/4+1/2), (3L/4, 3L/4+1/2)} (769 faces each), 2-folds = those ∪
  {(0,1/2), (0,(L+1)/2), (L/2,1/2), (L/2,(L+1)/2)} (800 faces
  each); glides measured off the L/8 grid — 45° lines y−x ≡
  L/4+1/2, 3L/4+1/2 (mod L) of shift ±(L/4, L/4) and 135° lines
  y+x ≡ L/4+1/2, 3L/4+1/2 of shift ±(L/4, −L/4) (785 faces each,
  0 mismatch); full-disk census about (5L/4, 5L/4) — 2615 faces,
  adaptive exact R² = 377/6 + 98√3/3 ≈ 119.413660. Negative
  controls fail: no mirror line in either family (all 18 t = 0
  controls, 772–800 faces tested — p4g has glides, no mirrors),
  the half-period shifts ±(L/2, ±L/2) on the off-grid lines,
  rot45 about a 4-fold center, and the off-orbit L/8 mirrors and
  glides. 6 new tests (1 engine self-consistency + 5 registry).
  Census: 109 → 110 tilings, 121 → 122 memberships, Islamic 20 →
  21, 729 → 735 tests (fork-local numbers on the hankin-snub fork;
  renumbered at integration). Full bar green: cargo check +
  clippy (both target sets) zero warnings, cargo test --workspace
  --all-features 735/735. Details in §66.

- 2026-11-20 — M110 Fractalizing 3.12.12 (fractalizing-3-12-12) ✅
  (§65 — renumbered from the fork's provisional §64 by
  landing order; landed after M104): Chavey's limit-periodic dodecagon-dense fractalizing family
  over the uniform 3.12.12 (Symmetry 25(3) 2014; Wikipedia
  fractalizing-k-uniform entries), fully reverse-engineered from
  `Planar Fractalizing Truncated Hexagonal Tiling II.svg` — the
  handoff's σ-patch substitution model was disproven against the
  figure (its per-ring patches cannot close edge-to-edge), so the
  entry lands as the same-plane nesting it actually is: level d at
  child edge (2+√3)^{1−d}, level d+1 = level d shrunk by 1/(2+√3)
  about the origin flower; a new period per level (limit-periodic),
  every level 6-fold periodic — Quasiperiodic on the Stampfli
  rationale, nuance declared in header/meta/docs. Level ≥ 1 =
  hexagonal orbit of a 57-tile flower (44 T + 12 S + 1 H after
  60°-rotation dedupe + 2 seam triangles) + 12 dodecagon classes per
  cell at exact reduced coordinates DM3 ∈ (Z[√3]/3)² — 69 tiles per
  cell, cell 84 + 48.5√3, the census forced by the exact area
  identity; level 0 = uniform 3.12.12 at edge 2+√3 whose cell area
  equals the level-1 flower cell exactly. All level-1 vertices on
  the exact (m + n√3)/6 grid: integer tables, Z[√3] anchors, dodecagon
  centers (m+n)(36+21√3)/18 × (m−n)(21+12√3)/18, one float multiply
  per vertex last. Depth d = 1 + floor(log_p(6/diag)), cap 16 with a
  fail message. Exact Q(√3) acceptance battery (Python 40-dp, mirrored
  in Rust i128): 5×5-cell block 1725 tiles — 0 interior overlaps,
  edges ≤ 2, 0 T-junctions, 2500 exact rational samples single-covered,
  per-cell census 44/12/1/12 × 9 cells, all D edges squared-length 1.
  Rust: `fractalizing.rs` mirrors `stampfli.rs` (Piece-free direct
  orbit emission, similarity-map params, STEP_CAP), 6 module tests
  (i128 exact battery, winding/area table census, cell-span p-law +
  nested count invariance, nested-view identity
  render(d+1, scale p, V) == render(d, scale 1, V) to 1e-9, level-0
  seed structure) + the 3 registry tests. Census: 109 → 110 tilings,
  121 → 122 memberships, Quasiperiodic 9 → 10, 729 → 738 tests
  (landed on main after the M104 golden-bee landing; the fork's step
  numbers were on its own base 419c63a). Full bar green: cargo check
  + clippy (both target sets) zero warnings, cargo test --workspace
  --all-features 738/738. Details in §65.
- 2026-11-20 — M112 seed-class coloring for uniform-seed cards (UI
  audit) ✅: fixed the "plain outlines on gray" finding — tilings whose
  every tile carried seed 0 painted one flat PALETTE[0] color in
  Pattern mode. New `SeedPattern::Rows` (seed parity alternates with
  the lattice row index j — alternate courses for paver bonds);
  per-tiling declarations: pentagon-2 seeds track chirality (the same
  coloring that drops the group to p2), pentagon-3 / stepped-
  hexominoes / chiral-cut-honeycomb / girih-shesh-band decorative
  checkers, running-bond Rows (course parity). No geometry, lattice,
  group, or description numbers changed; census untouched. Six new
  seed-variation tests; full bar green.
- 2026-11-20 — M113 naming and typography audit (UI audit) ✅: titles
  aligned to the family scheme with ids/geometry/censuses unchanged —
  the duplicate two-uniform pair resolved by subscripts (n20 →
  "(3⁶; 3⁴.6)₁" chiral p6 index-13 flowers; n19 → "(3⁶; 3⁴.6)₂"
  achiral p6m index-9 flowers, mirrors confirmed), najm titles
  simplified ("Najm template (6,3)" / "(4,4)"), Penrose titles
  disambiguated (P3 (rhombs) / P2 (kite and dart) / pentagons (5-tile
  φ-substitution)), all eleven dual names brought to the Laves style
  "Dual of 2-uniform (V<config>)" with V-markers on both members
  (also fixing the n19/n20 duals' duplicate title via ₁/₂
  subscripts). Registry-wide name uniqueness re-verified (temporary
  test, since removed); 745 tests green at the commit.
- 2026-11-20 — M114 Stack bond tiling (2:1 dominoes) ✅: new
  `LatticeTiling` (id `stack-bond`, Arrangements) — rows of 2:1
  dominoes stacked directly on top of one another, the running bond
  without its half-brick offset; lattice ⟨(2,0),(0,1)⟩ (area 2), one
  split horizontal domino per cell ([0,2]×[0,1], long sides split
  into two collinear unit edges — 180° figures at the split
  midpoints); measured pmm (mirrors x = k and y = n/2, 2-folds at
  (k, n/2), no glides; coset probe 128-per-coset exact,
  ROT4/ROT4_270/DIAG/ANTI never); figures 4.4.4.4 at even-x points,
  2.2 at odd-x split midpoints. Census: Arrangements 5 → 6,
  112 built-ins, 124 memberships.
- 2026-11-20 — M115 Herringbone 45° tiling (2:1 dominoes) ✅: the M40
  herringbone bond in its 45°-laid canonical frame (`herringbone-45`,
  Arrangements) — exact radical geometry via std::f64::consts::SQRT_2;
  lattice ⟨(0,2√2),(√2,0)⟩ (area 4), two split 45° dominoes per cell
  (the L-pair); measured pgg — 2-folds at (√2/4 + k√2/2, 3√2/4 + ℓ√2),
  vertical glides x = k√2/2 (glide √2), horizontal glides
  y = ℓ√2 + √2/4 (glide √2/2), descended from M40's diagonal/
  anti-diagonal glides under the 45° rotation; verified in integer
  P-coords (world = (√2/2)·P): cell area sum 8 = |det|, 3906-point
  fine grid covered once, 192 interior unit edges each shared by
  exactly 2, all 113 interior vertex figures 2.4.4, coset probe via
  the new shape-set helper `shape_set_is_sym` (rotated cells break
  the axis-aligned bbox probe) + the independently measured M40 group
  conjugated by the rotation reproducing all 8 candidate matrices.
  Census: Arrangements 7, 113 built-ins, 125 memberships.
- 2026-11-20 — M116 Basketweave three-brick tiling (2:1 dominoes) ✅
  (`basketweave-3`, Arrangements): the triple-basketweave paver bond —
  three courses of horizontal 2:1 dominoes stacked without offset,
  then one course of vertical dominoes, repeating with vertical
  period 5; lattice ⟨(6,0),(0,5)⟩ (area 30) an index-3 superlattice
  of the true ⟨(2,0),(0,5)⟩, kept for the whole-cluster cell
  convention (cf. M39); 15 split dominoes per cell (9 horizontal +
  6 vertical, long sides split into collinear unit edges); measured
  pmm (mirrors x = k and y = 2.5k + 1.5, 2-folds at (k, 2.5k′ + 1.5),
  no 4-folds, no essential glides); verified exactly in Python
  (stdlib) before porting: cell area sum 30 = |det|, fine shifted
  grid covered exactly once (6474 points), 296 interior unit edges
  each shared by exactly 2, vertex figures sum to 360.
  Census: Arrangements 8, 114 built-ins, 126 memberships.
- 2026-11-20 — M117 Pentagon type 1 tiling (Reinhardt type 1) ✅
  (`pentagon-1`): the regular-hexagon member — a side-2 hexagon cut
  by the segment joining the midpoints of two opposite edges (cut
  unique up to the hexagon's symmetry; (A,B,C,D,E) = (120°,120°,
  120°,90°,90°), sides (2,2,1,2√3,1), tile area 3√3, all in Q(√3));
  1-isohedral, 2 tiles per centered-rectangular cell (60°-rhombus
  basis, det 6√3 = 2 × area; the legality test now documents the
  a² = 3b² hexagonal-metric special realization); edge-to-edge,
  vertex stars {120°,120°,120°} / {90°,90°,90°,90°}; measured cmm
  overriding the brief's p2 pin (achiral prototile — a −120° rotation
  about the hexagon centre maps it onto its y-mirror; two
  perpendicular mirror families + glides between them; four
  half-turn classes, two off every mirror — ruling out pmm — plus
  orientation-reversing isometries ruling out p2); type-1 exclusive
  under all 10 dihedral relabellings; distinct from the shipped
  prismatic-pentagonal (also type 1, different pentagon). Python
  audit 31/31 before porting; 9-test in-module suite.
  Census: PentagonTypes 12 → 13, 112 built-ins, 124 memberships.
- 2026-11-20 — M118 2-uniform (3.12.12; 3.4.3.12) tiling ✅
  (`kuniform-n2`): the 3-4-3-12 tiling — dodecagons kissing
  flat-edge-to-flat-edge on a square lattice with 1 unit square (at
  the origin) + 4 unit edge triangles (apex = dodecagon corner)
  filling each four-dodecagon hole; square basis ⟨(2+√3,0),
  (0,2+√3)⟩, det 7+4√3 exact; 6 tiles/cell; p4m measured by coset
  audit (order 8 mod L, point group D4, no true glides); torus
  orbits t = 3 tiles / 2 vertex classes / 3 edges — matching the
  Wikipedia parenthetical exactly. Python exact battery (Q(√3)
  arithmetic) before porting. Census: TwoUniform 15 → 16,
  112 built-ins, 124 memberships.
- 2026-11-20 — M119 2-uniform (3³.4²; 3².4.3.4) tiling ✅
  (`kuniform-n16`): variant 1 of the wiki's 33344-33434 tiling
  (variant 2 = n17, pgg, remains open) — staggered square rows and
  triangle bands in the snub-square-related form; 45°-rotated square
  basis ⟨(h,−h),(h,h)⟩, h = (3+√3)/2, det 6+3√3; 18 tiles/cell
  (6 squares + 12 triangles; 4 vertex classes of 3³.4² and 8 of
  3².4.3.4); p4g measured by coset audit — axis-reflection cosets
  pure, both diagonal cosets true glides (axial shift 3/4 of a
  diagonal lattice period, no integral shift cancels); torus census
  4 tile orbits (8+4+4+2) + 5 edge orbits = the wiki's
  4-isohedral/5-isotoxal exactly. Census: TwoUniform 16 → 17,
  113 built-ins, 125 memberships.
- 2026-11-20 — M120 Dual of 2-uniform (3.12.12; 3.4.3.12) ✅
  (`kuniform-dual-n2`, Laves tab): face-centroid dual of
  `kuniform-n2` — same square lattice, det 7+4√3 inherited; 8
  faces/cell over the primal's 8 vertex orbits: 4 congruent kites
  (3.4.3.12 orbit; sides (3+√3)/6, (2+√3)/√3, (2+√3)/√3, (3+√3)/6;
  angles 90/120/30/120; area 3/4+5√3/12) + 4 congruent isosceles
  triangles (3.12.12 orbit; sides (2+√3)/√3 ×2 over base 2+√3,
  angles 120/30/30); Σ = 7+4√3 = |det| exact; p4m — every
  orientation-reversing coset a pure reflection (zero glide cosets);
  face/vertex/edge orbits 2/3/3 matching the primal's e = 3; tiled
  dual edge-to-edge. Census: Laves 22 → 23, 114 built-ins,
  126 memberships.
- 2026-11-20 — M121 Dual of 2-uniform (3³.4²; 3².4.3.4) ✅
  (`kuniform-dual-n16`, Laves tab) — the key-lock tiling:
  face-centroid dual of `kuniform-n16` — same 45°-rotated square
  lattice, det 6+3√3 inherited; 12 faces/cell over the primal's 12
  vertex orbits: 8 congruent pentagons (3².4.3.4 orbit; sides
  (3+√3)/6 ×4, 1/√3; angles 120/90/120/120/90) + 4 congruent
  pentagons (3³.4² orbit; sides (3+√3)/6, 1, (3+√3)/6, 1/√3, 1/√3 —
  the unit side crossing two squares' shared edge; angles
  120/90/90/120/120), both exact areas 1/2+√3/4; Σ = 6+3√3 = |det|
  exact; p4g — axis mirrors pure (meeting at the corner-2 point),
  both diagonal cosets true half-period glides, 4-fold centers at
  ((√3−1)/4, (5+√3)/4) + partners with no mirror through any of
  them; new Rust P4g probe arm (Case gained an axis_pt field); tile
  orbits 2 (8+4), vertex orbits 4, edge orbits 5. Census end state:
  Laves 23 → 24, 115 built-ins, 127 memberships.
- 2026-11-20 — M122 classification schema ✅: `TilingClass` +
  per-tiling classification table in tess-core (substitution family,
  inflation, point order, prototile count, reflections), cards
  composed from the schema, declared (measured-lattice) tab sorts —
  Pentagon types sequential 2–15 with cairo = type 4 and floret =
  type 5 joining as typed secondary members (PentagonTypes 12 → 14),
  Two-Uniform by vertex configuration with ₁/₂ pairs adjacent, Laves
  by rank regulars→proper→duals, Wallpaper by the group's
  conventional lattice (p1/p2 oblique … p3*/p6* hexagonal, fixing
  the M76 measured-lattice sections); Ctrl+wheel resizes browse
  cards (width 22–44, height at half, thumbnail cache keyed by size).
  Census: 111/123 → 111/125 at the commit (cairo/floret secondary
  memberships).
- 2026-11-20 — M123 preview-grid resizing everywhere ✅ (owner
  report: "the preview grid doesn't resize on mouse/trackpad or +/-"):
  TUI — the +/- keys (with =/_ variants) resize Browse-mode cards
  (previously only zooming the canvas in Render), shift+wheel joins
  ctrl+wheel as an accepted alias (several terminals swallow or strip
  Ctrl on wheel), Browse status bar advertises "+/- card size"; GUI —
  real card-grid zoom: ctrl+wheel / trackpad pinch (egui's
  zoom_delta, which already diverts ctrl-scroll away from scrolling,
  so the grid never scroll-jumps) and +/- keys scale the card target
  width across [150, 480] px with the preview image scaling along
  (4:3 kept; the fixed 168 px CARD_INNER_H retired for
  card_inner_h/card_total_h of the live image size; stale textures of
  other sizes pruned; the grid fills the window width exactly at
  every zoom level). Tests: TUI key/wheel/search-isolation; GUI
  scaling pins + real headless passes at default and zoomed sizes.
  800 tests at the commit.
- 2026-11-20 — M127 Golden triangle tiling (√−τ reptile
  substitution) ✅ (`golden-triangle`, Quasiperiodic): the right
  triangle (legs 1, √τ; hypotenuse τ) dissected by the altitude from
  its right angle into two similar children (scales τ^−1 and
  τ^−1/2, area law τ^−2 + τ^−1 = 1); complex √(−τ)-inflation
  viewpoint (λ = i√τ, λ² = −τ, λL = L ∪ S, λS = L) in exact
  K = Q(λ), λ⁴ = 1 − λ², embedding (a + bλ + cλ² + dλ³) →
  (a − cτ) + i√τ(b − dτ); uniform rule foot F = P + τ^−2(Q − P)
  (τ^−2 = 2 − τ exact); canonical vertex order carries chirality
  (flips every cut), class = parity of −e with hyp² = τ^(e+2);
  finite covered patch (seed L0), recentred, depth =
  clamp(ceil(−ln(E/τ)/ln τ), 5, 17), scale default 3.5 (chair
  framing); category Quasiperiodic on the Danzer–van Ophuysen 2000
  triangle-pair family (scope note in module docs); classify entry
  substitution / √(−τ) / 2000. Python battery: field axioms,
  prototile identities, exact altitude cuts, d ≤ 6 triangulation
  (2^d tiles, 2^(d−1) per class, exact area conservation, no
  crossings, strict PIP), junction census, chirality flip, exact
  child-rotation multipliers. Census: Quasiperiodic 10 → 11,
  120 built-ins, 134 memberships, 814 tests at the commit.
- 2026-11-20 — M128 Armchair tiling (L-tetromino rep-4) ✅
  (`armchair`, Nonperiodic): the chair-family rep-tile line's next
  member (PROPOSED.md §23d), mirroring the chair machinery — the
  L-tetromino "armchair" (2×3 bbox, area 4); the scale-2 inflated
  armchair has a unique L-tetromino dissection (exhaustive
  exact-integer enumeration: one child of each of the four
  orientations, rule forced); non-edge-to-edge (the canonical side-2
  parent carries a T-junction at (2,2)); limit-periodic (the depth-g
  block's only translation period (2^(g+1), 2^(g+1)), measured on
  the integer grid; Python period lattice (4,4)/(8,8)/(16,16)/(32,32)
  and T-junction counts 32/144/656/2736 per generation); 9 in-module
  tests + the standard trio. Census: Nonperiodic 4 → 5, 120
  built-ins, 134 memberships at the commit.
- 2026-11-20 — M129 Ammann-Beenker rhomb-triangle tiling ✅
  (`ammann-beenker-tri`, Quasiperiodic): Bielefeld's "Ammann-Beenker
  rhomb triangle" entry (PROPOSED.md §23b) — the A5 substitution's
  half-square triangles emitted as their own tile class instead of
  being paired back into squares: 3 prototiles (rhombus 0,
  R-triangle 1, L-triangle 2), same silver-mean deflation as the
  classic entry (hypotenuse-pairing step removed); module mirrors
  ammann_beenker.rs (rule tables, Q(√2) deflation, 8-rhombus star
  seed, STEP_CAP 20). Python battery (exact Q(√2), star depths 1–4):
  per-parent substitution covers, PF-orbit counts (24,16,16) →
  (136,96,96) → (792,560,560) → (4616,3264,3264), edge-to-edge
  growth, no interior overlaps, hypotenuse groups ≤ 2 with R–L
  paired, parity H = 2P + U, H/r → √2. Census: Quasiperiodic 11,
  120 built-ins, 134 memberships, 810 tests at the commit.
  Visual-by-proxy: TUI proto export 1016 tiles (436 rhombi + 580
  triangles, R/L 290/290).
- 2026-11-20 — doc-sync pass (this file): the Implemented census
  heading, tables and changelog brought level with the registry —
  122 tilings / 136 memberships pinned by
  `registry_groups_are_complete_and_canonically_ordered`; the
  paver-bonds table (which had absorbed the two-uniform, dual and
  pentagon rows without section headers) split into Paver bonds /
  Two-uniform / Two-uniform duals / Pentagon types / Paver tilings
  (M78–M85) sections; the 27 rows that had never landed here added
  (hankin-3-6-3-6, hankin-3-4-6-4; stack-bond, herringbone-45,
  basketweave-3; kuniform n2/n16/n3/n4/n14/n15; kuniform-dual
  n2/n16; pentagon-1/2/7; golden-triangle, armchair,
  ammann-beenker-tri). A tess-tilings guard test now fails when any
  registry id is missing from this section or from the README
  catalog table, or when the stated counts drift. Note: the
  M111–M129 tilings each landed with their own census lines on their
  own bases (the M122 schema landed on main before the M114–M121
  batch); the end state is the pinned 122/136 with per-tab
  [11,17,24,17,15,21,8,12,6,5].
- 2026-11-20 — M130: TUI params-strip repaint fix — the render-mode
  arrow branches for the params strip (Left/Right adjusting the
  pointed-at parameter, Up/Down moving the cursor) changed app state
  without setting the render-loop `dirty` flag, so the frame never
  repainted: parameter steps and cursor moves showed no feedback and
  the canvas kept the pre-adjust tiling until some other key
  dirtied the frame (found by visual testing: scale sat at "1" on
  screen while the param set read 1.2). Fix: set `dirty = true` in
  the four branches (the state-changing helpers in Browse —
  `move_cursor`, `walk`, `scroll_by`, `resize_cards` — already set
  it internally). Verified live on X11: scale 1 → 2 via 20 Right
  steps repaints each step (Penrose P3 tile count 130 → 53 with
  visibly larger rhombs), rotation 0° → 30° via Down + 6 Rights
  repaints rotated output (53 → 92 tiles); tess-tui suite 27/27,
  workspace 848/848.
- 2026-11-21 — M131: Girih bowtie tiling (72.72.216) ✅: the classical
  concave girih hexagon (papillon; unit edges on the 36° girih frame,
  interior angles 216°,72°,72° ×2) tessellates the plane alone —
  single-tile cmm CenteredRectangular lattice table, det(t1,t2) =
  area(tile) ≈ 1.314328 exact over K = Q(2cos π/10); interior vertex
  figure 72/72/216. 7×7-patch battery: shoelace area == det < 1e-15,
  interior edge occurrences shared exactly 2 (6-digit keys), vertex
  wedges closed-or-boundary, irrational-offset grid coverage.
  Registry 122 → 123 built-ins, memberships 136 → 137, per-tab
  [11,17,24,17,15,22,8,12,6,5].
- 2026-11-21 — M132: Girih rhombus tiling (72.72.108.108) ✅: the
  classical thin girih rhombus (72°/108°, unit edges on the 36°
  frame) tessellates the plane alone — single-tile cmm
  CenteredRectangular lattice table ⟨(1,0),(cos72°,sin72°)⟩,
  |det| = sin72° = tile area exact over K = Q(2cos π/10).
  7×7-patch battery: 84 interior / 28 boundary edge occurrences of
  196 (shared exactly 2), wedges closed-or-boundary, coverage once
  per sample. Registry 124 built-ins, memberships 138, per-tab
  [11,17,24,17,15,23,8,12,6,5].
- 2026-11-21 — M133: Girih decagon tiling (10.6.10) ✅: the classical
  regular girih decagon cannot close a figure alone (144° reflex
  pairing needs 144+72+144 = 360) — cell = 1 DEC + 1 BOW on a cmm
  CenteredRectangular lattice ⟨(0,−(1+√5)·S72), (5/4+3√5/4, √5·S72)⟩,
  det = 2a³−5a/2 ≈ 9.008536623 = DEC+BOW areas, exact over
  K = Q(2cos 18°); tile-level wallpaper probe (mirrors on both
  centered-rectangle axes, interleaved glides, rot180 off mirrors) →
  cmm. 7×7-patch battery: 313 interior / 158 boundary of 784 edge
  occurrences, wedges closed-or-boundary, 1600 irrational-offset
  samples covered exactly once. Assembly cross-check (exact-K):
  classic rosette DEC + 10 PENT — 0 overlaps, all 10 decagon
  vertices close at exactly 360° (144+108+108); DEC-cell DEC+BOW
  shares exactly 1 edge. Registry 125 built-ins, memberships 139,
  per-tab [11,17,24,17,15,24,8,12,6,5].
- 2026-11-21 — M134: Girih pentagon tiling (5.5.4.4) ✅: the classical
  regular girih pentagon leaves 36° gaps alone (108°×5) — cell = 2
  PENT (paired on a shared edge) + 2 HEX (shesh-band hexagons) + 3
  RHOMB on a cm CenteredRectangular lattice ⟨(−C36, −2·S72−S36),
  (2+√5, 0)⟩, det ≈ 10.547378392 = 2·PENT + 2·HEX + 3·RHOMB exact
  over K (exact-field tuple identity as the area certificate;
  float det within 1e-14 = 2 ulp); exactly one mirror family
  (72°, pentagon-pair axis) with interleaved glides, no rotations →
  cm. 7×7-patch battery: 765 interior / 136 boundary of 1666 edge
  occurrences (reconfirmed on the shipped tables with exact-K and
  6-digit keys), 1600 irrational-offset samples covered exactly
  once. Cross-table congruence: rhombus-table tile ≅ PENT-cell
  rhombi, DEC-cell bowtie ≅ bowtie-table tile. Registry 126
  built-ins, memberships 140, per-tab [11,17,24,17,15,25,8,12,6,5].
  Girih-primitive cluster complete (M131–M134): all five classical
  primitives now ship as periodic tables beside the quasiperiodic
  girih-qc entry.
- 2026-11-21 — M135: Half-hex tiling (limit-periodic substitution) ✅:
  Socolar's half-hexagon substitution (Bielefeld "half-hex"; cited
  Fre02 cut-and-project, GS87 Ex. 10.1.3) — one achiral prototile P1
  (trapezoid half of a unit hexagon, edges {2,1,1,1}), six rotation
  classes and no reflection class (mirror = translated rotation);
  λ = 2 into 4 children (orientation offsets {0,2,3,4}), substitution
  matrix S[j][i] = 1 iff (j−i) mod 6 ∈ {0,2,3,4}, PF eigenvalue
  4 = λ², char poly (x−4)(x−2)(x−1)²(x+1)² pinned by an exact
  720-permutation i128 expansion — which caught and fixed a real
  next_perm off-by-one (scan from i=4 visited 48/720;
  regression-guarded). Exact √3-field (u,v) integer-pair arithmetic
  end to end, f64 only at the emission boundary; depth d = 5..8
  picked by the view diagonal (4⁸ = 65 536 ≤ MAX_TILES), patch
  recentered by the presentation shift (2^d, 2^{d−2}). Battery: rule
  line green in all 6 orientations; depth counts 4^g with vectors
  [1,0,1,1,1,0]/[4,2,3,2,3,2]/[12,9,12,10,12,9]/[46,40,45,40,45,40];
  0 overlaps over 12/116/653/3017 tested nearby pairs; fine-grid
  coverage 52/200/784/3104 exact; nested d = 7 views bit-identical on
  intersections (containments B5⊂B1, B1⊂B2, B4⊂B2). Limit-periodicity
  measured: minimal x-period p = (3/2)·2^g = 6, 12, 24, 48, 96 at
  g = 2..6, strictly doubling per level (offset from the naive
  2^{g+1} by the depth-0 atomic tiling's pure-x period 3/2).
  Registry 126 → 127 built-ins, memberships 140 → 141, per-tab
  [11,17,24,17,15,25,8,12,7,5]; aperiodic family 5 → 6. Not a lattice
  table — no culling line.
- 2026-11-21 — M136: Pregnant chair tiling (rep-tile) ✅: the
  √3-inflation two-prototile rep-tile from the Bielefeld substitution
  index (E. Amezcua; rule salvage-pinned against the recovered
  pc_rule/pc_battery/pc_seed battery, C1 = x-mirror of the published
  figure) — large chair [0,3]×[0,2√3]∪[3,6]×[√3,2√3], small chair
  [0,2]×[0,√3]∪[0,1]×[√3,2√3]; the inflated large chair tiles as
  1 large + 6 small, the inflated small chair is exactly one large
  (area-conservative: 3√3·small = 9 = large). Exact-integer Z[√3]
  (i,j)-pair arithmetic throughout, floats only at render.
  Finite-depth approximant: two-chair seed blocks on the base period
  lattice, refined depth times; per-step culling by the proven
  orientation-agnostic superset box (√3^r·shape, half-extents
  6√3^r / 2√3^r); depth = octave quantization of the view extent, so
  panning/zooming keeps bit-identical overlapping tiles. Battery:
  seed-block exact cover, depth-4 block = 266 leaves (area law
  9nL+3nS = 18·3^d), depth-1 block = 14 tiles with 11 exact-zcmp
  T-junctions, single-seed d4 = 133 leaves = 55L+78S (area 729),
  chiral orientation census {0,2,6,7} (4 of 8 dihedral classes);
  12 exact module tests green. Nonperiodic tab 5 → 6, reptiles
  family 4 → 5. Registry 127 → 128 built-ins, memberships 141 → 142,
  per-tab [11,17,24,17,15,25,8,12,7,6]. Not a lattice table.
- 2026-11-21 — M137: Cyclotomic field layer — K₇ exact core
  (fields.rs) ✅: first landing of the §23e/§23i gate (design doc
  docs/cyclotomic-field-layer.md §7 step 1) — a generic degree-D
  exact-field layer (kmul convolution+reduction fold in i128 with
  loud overflow; knorm via fraction-free Bareiss Sylvester; ksign
  exact sign/order via interval-Horner bisection over a
  Sturm-certified dyadic isolating interval of b, steered by the
  EXACT sign of m(mid) — no floats in the engine) + the concrete
  K₇ = Q(2cos(2π/7)) core: minpoly x³+x²−2x−1, reduction
  b³ = 1+2b−b², b⁴ = −1−b+3b², isolating interval
  [2677868308/2³¹, 2677868309/2³¹] (all derived + certified in the
  scratch spike, re-asserted in Rust), and the P7 oblique frame
  (norm² = A²+B²+A·B·b, sine-free cross-sign, companion rotation
  R = [[0,−1],[1,b]]). 10 tests: minpoly zero-test; interval
  bracketing exact; frame battery (|U(k)|²=1, ΣU(k)=0, R^k
  consistency, float ≤1e−12); {7/1} and {7/3} star walks closing
  exactly with exact shoelaces 4+3b+b² and −8+b+5b²; the 29-element
  sign/order battery vs float; Sylvester norm identities. K₉/K₁₅ +
  the girih ROTM anchor follow (§7 step 2); first consumer family at
  step 3. No census change (registers no tilings; lib.rs gains
  exactly one line). Module carries #![allow(dead_code)] until a
  consumer family lands.
- 2026-11-21 — M138: Domino (table) substitution tiling ✅ (§23d):
  Lançon's limit-periodic domino substitution lands as ONE tiling,
  id `domino` ("Domino (table) substitution") — the Bielefeld entry
  says verbatim "Also known as 'table tiling.'", so "table" is an
  index alias, not a second entry. Pinned rule (entry
  https://tilings.math.uni-bielefeld.de/substitution/domino/): two
  domino prototiles on Z² — A = vertical 1×2, B = horizontal 2×1 —
  λ = 2 orientation-preserving into 4 children; substitution matrix
  S = [[2,2],[2,2]], column sums 4 = λ², PF eigenvalue 4 = λ², char
  poly x²−4x = x(x−4) pinned by exact 2×2 determinants; exact
  integer cell arithmetic on Z² end to end, floats only at the
  emission boundary; finite covered patch = the 2^d × 2^(d+1) block
  recentered on the origin, d = 5..8 picked by the view diagonal,
  nested views bit-identical on their intersection. Limit-periodicity
  measured, not asserted: the depth-g patch carries no proper
  within-patch period — it is itself the fundamental cell of the
  period-(2^g, 2^(g+1)) approximating periodic tiling: (2,4), (4,8),
  (8,16), (16,32), (32,64), (64,128) at g = 1..6, 2× per level
  ("full-block" manifest, kind-different from half-hex's in-patch
  p = (3/2)·2^g hierarchy). NOT cut-and-project / model-set: the
  entry states the dynamical spectrum has a continuous component
  ("Thus it cannot be a cut and project tiling") — Solomyak, ETDS
  17(3) 1997; Robinson, Indag. Math. 10(4) 1999, "On the table and
  the chair" — the PROPOSED §23d cut-and-project claim corrected
  accordingly. Battery: rule line (containment/disjoint/area),
  depth-2 figure blocks reproduced exactly (16 tiles each),
  zero-overlap single coverage (counts 4/16/64/256; ¼-cell grid
  points 128/512/2048/8192), period hierarchy, nested-view bit
  identity, default-view pin, trio
  coverage/rotated-scaled/determinism. Census: aperiodic family
  6 → 7; Aperiodic tab 7 → 8; built-ins 128 → 129; tab memberships
  142 → 143. Not a lattice table — no culling line.
- 2026-11-21 — M139: Cyclotomic field layer — K₉ + K₁₅ + the girih
  ROTM anchor (fields.rs) ✅: §7 step 2 of the design doc, fields.rs
  only (817 → 2123 lines), no census change. K₉ = Q(2cos(2π/9)) core
  (minpoly x³−3x+1; reduction rows per doc §2.1; isolating interval
  at 2⁻³¹; P9 frame; walks {9/1}, {9/4}) and K₁₅ =
  Q(2cos(2π/15)) core (minpoly x⁴−x³−4x²+4x+1; interval at 2⁻²⁶; P15
  frame; walks {15/1}, {15/7}; the exact √5-subfield probe
  b³−3b = 2cos(2π/5) — K₁₅ ⊃ Q(√5) pinned); L₁₀ = the girih quartic
  (a⁴ = 5a²−5, N(a) = 5) with the PERMANENT regression anchor: all
  20 shipped girih ROTM matrices rebuilt through the generic
  kmul::<4>/RED_L10 path at denominator /64 (girih-identical
  %64-assert-then-divide per product) and compared bit-for-bit
  against 80 pasted girih_qc integers — 20/20 exact. Generic layer
  extended (kbasis + koblique_norm2/dot2/cross/rot free fns; P7
  methods → behavior-preserving thin delegates); BISECT_CAP 10 → 4
  with the i128-envelope derivation in the doc comment (deg-4 worst
  case ≈ 2^125.6 < 2^127). All constants derived + Sturm-certified in
  the scratch spike (checkpointed at .scratch/fields-layer-k9-k15/ —
  derive_k9k15l10.py, self-contained, re-runs in ~2 min). Test
  battery: one generic machinery set called per field — 35 fields
  tests (10 K₇ + 10 K₉ + 11 K₁₅ + 4 L₁₀): minpoly zero-tests +
  float ≤1e−12; exact interval bracketing; frame batteries; star-walk
  closure + centered equivariance + exact shoelaces; sign/order
  batteries vs float; norm identities; ROTM anchor. Next: §7 step 3 —
  the first consumer family (GLB generalized binary n = 7, the
  cheapest XL; quasiperiodic-family landing).
- 2026-11-21 — M140: 2-uniform (3³.4²; 3².4.3.4)₂ `kuniform-n17` ✅
  (§2, the 2-uniform 20-list — n17 was the last missing 2-uniform
  sibling on the list alongside n6/n7): variant 2 of the Wikipedia
  33344-33434 two-uniform entry (File:2-uniform n17-1.svg — the −1
  filename-suffix trap pinned; the wiki n-numbers are not row-major,
  the file links are authoritative), the pgg sibling of M119 n16.
  12 tiles/cell (4 unit squares + 8 equilateral triangles in two
  4-orbits, seeds 0/1/2), edge = 1, area 4+2√3 = |det|; exact Q(√3)
  via the family H/S3 consts. Rectangular right-handed lattice
  b1 = (½, 1+√3/2), b2 = (−2−√3, 1) (b1 ⊥ b2 exactly, |b2| = 2|b1|);
  the Python coset audit proves L is the TRUE primitive period
  lattice (0 of 12 square-centroid coset reps and 0 of 1942 short
  shifts qualify as extra periods — no index-2 superlattice, the
  emission cell is the true cell). pgg measured, not declared: 4
  two-fold centers/cell on the half-period grid in exactly 2 group
  orbits with 2(c1−c2) = b1+b2 exact; one glide per axis (g² = T
  axis); 0 reflections (295 of 1097 bisector candidates
  full-checked) — pgg, not cmm. Torus census t = 3 tile orbits
  (4+4+4), e = 6 edge orbits (4+4+4+4+2+2), v = 2 vertex orbits
  (4+4) — matches the wiki's 3-isohedral/6-isotoxal data. Vertex
  figures 33344 (156) + 33434 (136) over the ±3 patch, both
  self-reverse, all 495 patch vertices valency 5; all 8 orbit reps
  pinned in the Rust vertex-figures test. Battery spike6.py (stdlib
  only, 18.6 s, all green) checkpointed at .scratch/kuniform-n17/;
  pin values permanently encoded in the Rust tests. Culling
  genuinely covered: the entry was added to the family
  lattice_tilings() list (the lib.rs extend is per-family), the
  brute-force guard runs n17. Census: TwoUniform tab 17 → 18;
  kuniform family 30 → 31; built-ins 129 → 130; memberships
  143 → 144. Still missing from the closed 20-list: n6, n7.
- 2026-11-21 — M141: Period doubling (2D) `period-doubling` ✅
  (§23d, the marginal-supersubstitution cluster): the 2D product of
  the 1D period-doubling substitution a → ab, b → aa (Bielefeld entry
  pinned live — 1D-only, internal space the 2-adic integers,
  Frettlöh–Sing). σ⊗σ on Z² unit squares: 4 prototile label classes
  (aa/ab/ba/bb, class = 2·w[col] + w[row], seeds 0–3) over ONE square
  prototile; λ = 2 into a 2×2 frame, child (c,r) label =
  (σ(x)_c, σ(y)_r); substitution matrix
  S = [[1,2,2,4],[1,0,2,0],[1,2,0,0],[1,0,0,0]] (column sums all 4 =
  λ²), PF eigenvalue 4, char poly x⁴−x³−12x²−4x+16 (eigenvalues
  {4, 1, −2, −2}) pinned by exact i64 24-permutation determinants
  (permutation count asserted — the M135 certification-guard lesson).
  Limit-periodic via approximating periodic tilings: the depth-g patch
  is the fundamental cell of the period-(2^g, 2^g) approximant,
  measured (2,2) … (64,64) at g = 1..6 — the ratio-1 square manifest
  that distinguishes it from domino's ratio-2 (2^g, 2^(g+1)); no
  proper within-patch period. Exact integer arithmetic on Z² end to
  end; depth rule = octave quantization of the view extent with
  patch-stability (nested-view bit-identity) tested. Battery: stdlib
  spike (substitution consistency g = 1..5, 0-overlap +
  irrational-shift coverage g = 1..4, label censuses S^g·e_aa =
  [1,1,1,1] → [9,3,3,1] → [25,15,15,9] → [121,55,55,25]) + 15/15 Rust
  tests. Classification mirrors domino (Aperiodic tab, method
  substitution, "limit-periodic"); the dispatch brief's draft
  "Nonperiodic" category was wrong — the worker's STOP branch caught
  it against the domino.rs/half_hex.rs precedent before coding. Spike
  checkpointed at .scratch/period-doubling-2d/. Census: Aperiodic tab
  8 → 9; aperiodic family 7 → 8; built-ins 130 → 131; memberships
  144 → 145. Remaining in the cluster: Thue–Morse 2D, period tripling
  (queued); Squiral (parked — rule-image-only source AND an
  infinite-edge prototile).
- 2026-11-21 — M142: Tangram `tangram` ✅ (§23d rep-tile dissection,
  the salvage wave's fourth landing): the classic five-prototile
  tangram (S M L Q P, u²-areas 1/2/4/2/2) as an area-conserving λ=√2
  substitution — S→{M}; M→{L}; L→{M,Q,P,S,S}; Q→{M,S,S}; P→{M,S,S}
  (sum of child areas = parent, no L child); A-matrix
  rows (S,M,L,Q,P): S (0,0,2,2,2), M (1,0,1,1,1), L (0,1,0,0,0),
  Q (0,0,1,0,0), P (0,0,1,0,0). Seed pinned by determinant search:
  two S triangles tile the √2×√2 square (S twist 0 at (0,0), S twist
  4 at (√2,√2), det=2) — the all-5-prototile seed search was negative
  for det ≤ 40; first L at g=4, all five prototiles at g=5.
  Homochiral P+: every substitution row is rotation-only and the seed
  has no P, so the P− mirror class is unreachable from the seed
  (P+ census = c_g[P] at every depth, P− = 0). Non-edge-to-edge with
  inherent T-junctions measured 0,0,0,6,10,18 over g=0..5; growth
  census g=0..19 = 2,2,2,10,18,26,66,138,242,506,1058,2026,4050,8282,
  16386,32586,65714,131258,261602,524458. Coordinates run on exact
  Z[√2] u-pairs end to end (√2·(a,b) = (2b,a)); floats only at
  emission; octave depth quantization from view extent with AABB
  culling sound because children ⊂ parent. Battery: Python spike +
  battery (zero-overlap exact Sutherland–Hodgman, 1521-point period
  spot-check g=2..5, chirality census, dissect-rule table pinned vs
  dump) + 12/12 Rust tests (9 exact-arithmetic module tests incl.
  substitution census over all proto×twist + the standard trio).
  Worktree landed on a stale base (M138: TwoUniform 17, built-ins
  129) — deltas re-derived against current main at integration:
  reptiles family 5 → 6; Nonperiodic tab 6 → 7; built-ins 131 → 132;
  memberships 145 → 146. Category stays Nonperiodic (limit-periodic;
  no Aperiodic evidence). Spike checkpointed at .scratch/tangram/.
- 2026-11-21 — M143: Thue–Morse (2D) `thue-morse` ✅ (§23d, the
  marginal-supersubstitution cluster's second landing): the 2D product
  of the classic 1D Thue–Morse morphism τ: 0 → 01, 1 → 10 (Bielefeld
  entry "Thue Morse" pinned live — 1D-only, 2 prototiles, Thue/
  Prouhet/Morse; the 2D rule pinned against JS99 = Allouche & Shallit,
  *The ubiquitous Prouhet-Thue-Morse sequence*, Springer 1999).
  τ⊗τ on Z² unit squares: 4 prototile label classes (00/01/10/11,
  child (c,r) of parent p = 2x+y gets class 2·(x⊕c) + (y⊕r), seeds
  0–3) over ONE square prototile; λ = 2 into a 2×2 frame. Substitution
  matrix S = the all-ones 4×4 — the Steinhaus property: every parent
  block contains each child class exactly once; column sums all 4 =
  λ², PF eigenvalue 4, char poly x⁴ − 4x³ (eigenvalues {4, 0, 0, 0},
  rank 1) pinned by exact i64 24-permutation determinants at
  x = 0..6 = [0, −3, −16, −27, 0, 125, 432]. THE distinguishing
  signature vs domino (M138, ratio-2 manifest) and period-doubling
  (M141, ratio-1 manifest): aperiodic at every depth — the depth-g
  patch carries NO proper translation period, measured g = 1..6 with
  all (g+1)²−1 proper candidate shifts rejected (3/8/15/24/35/48 per
  depth, 133 cumulative, zero survivors; each killed by the class-0
  (0,0) cell since w[2^k] = 1); singular continuous diffraction (JS99),
  not pure point. Classification: Aperiodic tab, method substitution,
  translational status "aperiodic", source "Allouche & Shallit".
  Exact integer arithmetic on Z² end to end; depth rule = octave
  quantization of the view extent copied from period-doubling;
  nested-view bit-identity tested. Battery: stdlib spike (word
  popcount closed forms g = 0..6, all-ones matrix by enumeration,
  pinned rule tuples, char-poly pins, consistency g = 1..5,
  0-overlap + coverage fine-grid/irrational-offset, no-period
  manifest, band simulation) + 15/15 Rust tests (11 in-module + the
  standard trio). Spike checkpointed at .scratch/thue-morse-2d/.
  Census: Aperiodic tab 9 → 10; aperiodic family 8 → 9; built-ins
  132 → 133; memberships 146 → 147. Remaining in the cluster: period
  tripling (queued — its Bielefeld entry has an empty Info section;
  rule must be reconstructed from the image + literature);
  Squiral (parked).
- 2026-11-22 — M144: Golden rhomboid `golden-rhomboid` ✅ (§23a, the
  Q(√φ) golden family; Andritz cluster; Bielefeld entry
  "golden-rhomboid-triangle" pinned live): the third Andritz-family
  φ-substitution — equilateral triangles side 1 (Y) and side φ (O)
  plus the 60/120 rhombus 1×φ (R); rules Y → O, O → O+Y+R, R → R+2O
  (areas 1 : φ² : 2φ in √3/4 units, all identities exact). Count
  matrix [[1,1,0],[2,1,1],[0,1,0]] in (R,O,Y) order, char poly
  λ³−2λ²−2λ+1, PF eigenvalue φ² pinned as an exact Q(√5) root
  (φ² = (1,1)). Exact 4-tuple Q(√5) arithmetic end to end (x =
  (a₀+a₁φ)/2, y = (b₀+b₁φ)·√3/2, φ·(a₀,a₁) = (a₁,a₀+a₁); floats only
  at emission). 18 anchored orientation classes (Y_k 0–5, O_k 6–11,
  R_A/B_k 12–17) collapsing to 10 unanchored sets under the
  authorized 120°-rotation quotient — used only as dissection
  distinctness keys; SHAPES/RULES stay anchored; live classes
  {0,1,6,7,12,14,15,16} pinned by d4/d12 censuses (13, 17
  unreachable). Single-seed unbounded growth: depth d covers
  φ^d·R₀ exactly, union = the 60° wedge {y ≥ 0, y ≤ √3·x} (views
  outside the wedge are background, no message); MAX_DEPTH 12
  (N₁₂ = 134 209 ≤ 250 000). T-junctions intrinsic to the
  substitution (first at depth 2: 2 points / 5 incidences; every
  T-point closes to exactly π on both sides in π/3 units) — a valid
  gap-free/overlap-free partition, NOT face-to-edge. Battery: stdlib
  spike (18/18 exact rule dissections — area sum, child-in-parent, no
  proper crossing/partial overlap/strict-inside vertex, co-oriented
  sequential boundary edges; depth count vectors (R,O,Y) d0..d12
  pinned; area law (0,4)·φ²ᵈ exact every depth; T-closure d2/d3/d4/
  d12; region-edge boundary partition d4/d12; Bielefeld patch
  cross-check 1525 Y : 4012 O : 2477 R, report-only) + 12/12 Rust
  tests (3-test trio + 9-test battery); battery's durable copy at
  tests/golden_rhomboid_battery_reference.py; spike checkpointed at
  .scratch/golden-rhomboid/. Census: Quasiperiodic tab 12 → 13;
  quasiperiodic family 11 → 12; built-ins 133 → 134; memberships
  147 → 148. Remaining in the Andritz cluster: Bowtie–Hexagon family
  (§23f, 4 entries, unblocked by the M131–M134 girih tables).
- 2026-11-22 — M145: 2-uniform duals n4/n3 `kuniform-dual-n4` +
  `kuniform-dual-n3` ✅ (§23k, the (4⁴; 3³.4²)₁/₂ cmm dual pair — the
  first of the two missing subscripted pairs the audit flagged; the
  (3⁶; 3³.4²)₁/₂ pair n14/n15 queues next): face-centroid duals of
  the in-repo exact primals via the oracle-validated `dual()` helper
  (M98/M102/M106/M120/M121 precedent), Q(√3/2) exact end to end.
  dual-n4: same centered-rectangular lattice ⟨(1,0),(1/2,2+√3/2)⟩ as
  the primal, det 2+√3/2; 3 faces/cell over the primal's 3 vertex
  orbits — 2 congruent pentagons (sides 1/√3 ×2, (3+√3)/6 ×2, 1;
  area 1/2+√3/4) over the two 3³.4² orbits + 1 unit square over the
  4⁴ orbit; Σ = |det| exact, edge-to-edge. dual-n3: det 3+√3/2; 4
  faces/cell — 2 pentagons (same shape) + 2 unit squares; Σ = |det|
  exact, edge-to-edge. Measured group cmm on BOTH — inherited from
  the registered Cmm primals, pinned by the exact S(g) audit with the
  cmm-vs-pmm discriminator (off-mirror 2-folds at (1/4,·) and
  (3/4,·)) and genuine h/v glide families; the primals' Cmm pins
  independently re-confirmed by the same audit. Spike journey
  documented with 5 programmatic retractions (an interim pmm/Pmm
  verdict was a buggy-lattice-reconstruction artifact — the fixed
  exact audit killed it; a coverage "failure" was an audit-range
  artifact, 0 bad points at the corrected bounds). Battery: stdlib
  spike (face censuses, exact areas, Σ = |det|, side²-class congruence
  with orbit-key margins ~2e-07, group audit run3 EXIT=0,
  coverage diagnostics) + 6 Rust tests (one trio per dual) +
  HARDENING: the shared Pmm/Cmm arm of `duals_match_measured_groups`
  gains a glide-closure probe (R180∘mirror at the cell-center 2-fold)
  — all 4 pmm/cmm cases pass, zero existing-test flips. Seeds [0,2,0]
  (n4) / [0,2,0,2] (n3) pinned by `dual_pattern_mode_is_proper_coloring`.
  Census: Laves tab 24 → 26 (duals are Laves-tab primaries); kuniform
  family 31 → 33; built-ins 134 → 136; memberships 148 → 150.
  Remaining §23k: the (3⁶; 3³.4²)₁/₂ pair (duals of kuniform-n14/
  kuniform-n15) — brief next; n6/n7 primals (the other 20-list gap).
- 2026-11-22 — M146: Period tripling (2D) `period-tripling` ✅ (§23d,
  the marginal-supersubstitution cluster's third landing — completing
  the ratio ladder 1/2/3): the rule was RECONSTRUCTED from the
  Bielefeld rule image ("period-tripling-2d", Info section empty — no
  tags, no references, nothing citable; a two-stage worker job: rule
  reconstruction from the image, then implementation from the pinned
  brief). The reconstruction was proven programmatically: the entry's
  patch image matches the simulated depth-4 tiling at exactly the
  predicted period-lattice offsets (81 offsets on the main diagonal =
  {(Δx,Δy): Δx ≡ Δy mod 81}; the depth-5 tiling at the two predicted
  cosets; the color-swapped reading at zero). Pinned rule: ONE unit-
  square prototile, 2 label classes (A gray, B blue), λ = 3 into 9
  children — A → 3×3 with B on the main diagonal (6A+3B), B → 3×3 all
  A; S = [[6,9],[3,0]], column sums 9 = λ², PF 9 = λ², char poly
  x²−6x−27 = (x−9)(x+3), second eigenvalue −3 = −λ — the MARGINAL
  case, pinned by exact 2-permutation determinant. NOT a σ⊗σ product:
  the 1D shadow σ: A → ABA, B → AAA (matrix [[2,3],[1,0]], char poly
  (x−3)(x+1)) is pulled back along x+y — the 3-adic cousin of
  period-doubling's A → AB, B → AA. Limit-periodic via approximating
  periodic tilings: period lattice exactly ⟨(1,−1), (3^g, 0)⟩
  (measured g = 1..5, sizes 3/9/27/81/243, no other periods) —
  period-1 along (1,−1) at every depth, axis-aligned (3^g, 3^g)
  strictly tripling per level (the ratio-3 signature completing
  domino's ratio-2 and period-doubling's ratio-1); the marginal second
  eigenvalue leaves exactly one ambiguous line — x+y = 0 carries B at
  odd depth / A at even depth, two limit tilings up to translation.
  Closed form: world cell (x,y) is B ⟺ v₃((x+y) mod 3^d) odd
  (v₃(0) := d). d = 5 single depth band (9^5 = 59049 ≤ MAX_TILES);
  emission x0 = c−121, y0 = 121−r; exact Z² arithmetic end to end,
  floats only at emission. Classification: Aperiodic tab (the
  category-trap from M141 pre-empted — all 9 prior family members
  verified Aperiodic before dispatch), method substitution,
  translational status limit-periodic, source None (image-derived).
  Battery: 14/14 Rust tests (10 module + 4 family block — char-poly
  pins x = 0..9, period manifest g = 1..5 strictly 3×, full-lattice
  census g = 1..4, g = 5 1D reduction with transverse period exactly
  243, defect-line parity, preview census A = 84/B = 36 over 120
  cells, nested-view bit-identity 5 windows) + the recon spike's
  measured pins. Census: Aperiodic tab 10 → 11; aperiodic family
  9 → 10; built-ins 136 → 137; memberships 150 → 151. The §23d
  marginal-supersubstitution cluster is now complete except Squiral
  (parked: rule-image-only source AND infinite-edge prototile).
- 2026-11-22 — M147: 2-uniform duals n14/n15 `kuniform-dual-n14` +
  `kuniform-dual-n15` ✅ (§23k COMPLETE — the (3⁶; 3³.4²)₁/₂ pmm/cmm
  pair, the second and final missing subscripted pair; §23k's
  audit-discovered duals all landed with M145+M147): face-centroid
  duals of the in-repo exact primals via the oracle-validated `dual()`
  helper, Q(√3) exact end to end. dual-n14: same primitive rectangular
  lattice ⟨(1,0),(0,1+√3)⟩ as the primal, det 1+√3; 3 faces/cell over
  the primal's 3 vertex orbits — 2 congruent pentagons (sides 1/√3 ×2,
  (3+√3)/6 ×2, 1; area 1/2+√3/4) over the two 3³.4² orbits + 1 regular
  hexagon (side 1/√3, area √3/2) over the 3⁶ orbit; Σ = |det| exact,
  edge-to-edge. Measured group pmm — all four 2-folds ON the mirror
  intersections, no off-mirror 2-folds, no true glides (the pmm
  signature; the new M145 glide-closure probe in
  `duals_match_measured_groups` verifies the negative). dual-n15: same
  centered-rectangular lattice ⟨(1,0),(1/2,1+3√3/2)⟩, det 1+3√3/2; 4
  faces/cell — 2 pentagons (same shape) + 2 regular hexagons; Σ =
  |det| exact, edge-to-edge. Measured group cmm — off-mirror 2-folds
  at (1/4,1+3√3/4) and (3/4,1+3√3/4) (the cmm-vs-pmm discriminator)
  plus genuine h/v glide families. Seeds [0,0,2] (n14) / [0,0,2,2]
  (n15) pinned by `dual_pattern_mode_is_proper_coloring`; case-driven
  batteries (convexity, edge-to-edge, area, primal-vertex-orbit
  correspondence, group matching) auto-cover both new cases. Battery:
  stdlib spike (face censuses, exact areas, Σ = |det|, group audit on
  both, seed search) + 6 Rust tests (one trio per dual). Census:
  Laves tab 26 → 28 (duals are Laves-tab primaries); kuniform family
  33 → 35; built-ins 137 → 139; memberships 151 → 153. §23k is now
  closed; the 2-uniform 20-list gap is down to the n6/n7 primals.
- 2026-11-22 — M148: 2-uniform primals n6/n7 `kuniform-n6` +
  `kuniform-n7` ✅ (the two [3.4².6; 3.6.3.6] forms — the LAST gap in
  the closed 2-uniform 20-list; the list is now COMPLETE in-repo):
  exact Q(√3) LatticeTiling data tables, one cell each, det 2+2√3.
  n6 (the ₂ form, Wikipedia `2-uniform n6`): point-touching flat-top
  unit-hexagon rows alternating with full unit-square rows, each
  inter-hexagon gap holding a counterposed up/down unit-triangle pair
  sharing the hexagon touch point as apex; CENTERED-rectangular
  lattice ⟨(2,0),(1,1+√3)⟩ (conventional cell 2×(2+2√3), centering
  vector (1,1+√3)); 5 tiles/cell (1 hex + 2 squares + 2 triangles,
  seeds 0–4); measured cmm — 2 two-folds/cell, one OFF the mirror
  lines at (0,1/2+√3) (the cmm-vs-pmm discriminator), true glides
  both axes. n7 (the ₁ form): the same 5-tile set with OPPOSITE
  triangle orientations — re-derived from the wiki SVG (a pure x-shift
  of n6 was tried and produced wrong vertex figures — the dead end is
  documented in the scratch NOTES); PRIMITIVE rectangular lattice
  ⟨(2,0),(0,1+√3)⟩, b1 ⊥ b2; measured pmm — 4 two-folds/cell all ON
  the mirror intersections, no true glides. Both forms: vertex figures
  3.6.3.6 (1 class) + 3.4².6 in two cyclic orderings ("3446"/"3644");
  exact enumeration gives 5 tile orbits per form vs the wiki's
  t = 3 (n6) / t = 4 (n7) — reported as a deviation per the n20
  precedent (e = 4 quoted, not re-enumerated). GUARD VALIDATION: the
  first n6 draft declared LatticeKind::Oblique and
  `declared_symmetry_is_legal_and_matches_the_measured_lattice`
  correctly rejected it ("cmm does not permit lattice kind oblique")
  — fixed to CenteredRectangular (n3 precedent); the legality guard
  bites. Battery: stdlib spike (Σ = |det| residual 0.0, edge-parity
  map 855 edges = 90 boundary + 765 shared, fine-grid coverage
  43560/82000 pts exactly once, orbit census 5/form, closure 10/10,
  exact group audit 2.1 s, pinned vertex-figure censuses) + 6 Rust
  tests (one trio per form) + exact Rust-tuple-vs-spike cross-check.
  Both entries landed in the family `lattice_tilings()` list (culling
  genuine). Census: TwoUniform tab 18 → 20 (the latent app_ui pins
  fired as predicted); kuniform family 35 → 37; built-ins 139 → 141;
  memberships 153 → 155. The 2-uniform 20-list is complete: the
  kuniform family's primal arc closes with 20 primals + 17 duals.
- 2026-11-22 — M149: GLB binary n = 7 `glb-binary-n7` ✅ (Pautze's
  generalized binary family at n = 7 — Hibma 2015 arXiv:1509.02053 /
  Pautze 2017 Symmetry 9(2) doi.org/10.3390/sym9020019; Bielefeld
  "generalized_binary"; ROADMAP §23e re-scope landed — the K₇
  cyclotomic layer's FIRST consumer, M137/M139 fields):
  three 7-fold rhombi (acute angles π/7, 2π/7, 3π/7; odd n gives
  (n−1)/2 = 3 prototiles), inflation λ = √(2+2cos(π/7)) =
  2cos(π/14) ≈ 1.9499, non-Pisot. Refinement law: per-parent
  refinement is a TWO-CHOICE pool (shape 1 → {1,6}, 2 → {5,2},
  3 → {3,4}) forced by compatible meshing at depth ≥ 2 — the pools
  are a derivation result beyond the published one-rule scheme, and
  eps-complementarity cascades through arbitrarily long adjacency
  chains, so NO local (shape, class) map drives refinement (probed
  and refuted, d_hier 6/7 key conflicts); certified generator =
  deterministic per-level DFS, all-pairs interior disjointness +
  eps-complementary bend bits on shared parent edges (compat
  ε_A + ε_B = 1), with the unculled parity battery pinning that
  spatial culling of the refinement frontier is UNSOUND. Arithmetic:
  K₇-twist pairs — every vertex (k₁, s₁k₂) ∈ K₇², K₇ = Q(b),
  b = 2cos(2π/7), dyadic denominators D = 2 (depth-stable), float
  emission only, all 6 488 certificate vertices on the even π/7
  class grid (0 odd at the depth-4 certificate); fan-area law: total
  twist area 2λ^{2d} exact per valid assembly; unculled growth
  3/10/35/126/462 (M^d row sums, M = [[2,1,0],[1,2,1],[0,1,3]],
  PF eigenpair λ²·v with v = (1, b²+b−1, b+1)). DEPTH CAP 5 — the
  deepest level constructed AND certified: σ²…σ⁵ pass the full
  staircase boundary certificate (single boundary cycle of 4·2^d
  even unit segments, consecutive pairings, closure on φ^{d−1}(B₀),
  exact area), while at depth 6 the candidate search exhausts — a
  legal patch need not admit a legal refinement. Deviation from the
  brief's depth-9 window policy, evidence-backed: tiles are
  unit-edge at every level over the fixed cell domain B₀ = φ(V)
  (bbox ≈ 3.43 × 1.65 cell units, presented bbox-centered), so depth
  sets fineness only — d = 5 resolves ≈ 13 tiles/cell-unit, finer
  than any view the area pre-guard admits; the cap also cut the
  module suite 5 min → 41 s. σ² census per fan pick: 16 two-choice
  combos, 9 disjoint-only, 4 meshed — disjointness alone NOT
  sufficient, eps-compatibility load-bearing. Tags: finite rotations
  + polytopal rhomb tiles; Pautze notes no Dₙ-symmetric patches; no
  pure-point claim. Pipeline: the rule was CERTIFIED in a prepass
  before implementation (λ = 2cos(π/14), 4-sector fan placement law
  = Pautze eq 78, even rules required for meshing, K₇-twist
  certificate 8816 vertices / 160 patches — the "module ≠ certified
  chain" divergence seen mid-development was a twist→poly conversion
  bug, fixed e-aware; the assembly tree provably branches, 8 valid
  σ³ per σ² set). Hygiene: fields.rs blanket allow(dead_code) → 41
  per-item allows as the module consumes the K₇ slice (fields tests
  35/35 unchanged). Battery: 12 module tests (σ⁵ pins: 462 tiles +
  area law + even grid + e ≤ 1; view bands preview 0.5570 /
  CLI 0.6238 ± 0.02) + standard registration surfaces. Census:
  Quasiperiodic tab 13 → 14; quasiperiodic family 12 → 13; built-ins
  141 → 142; memberships 155 → 156.
- 2026-11-22 — M150: Octiamond reptile `octiamond-reptile` ✅ (Bielefeld
  substitution index entry "Octiamond Reptile" — Folklore, Info section
  empty; rule reconstructed from the rule image and pinned by a UNIQUE
  exhaustive partition of the 3×-inflated 72-cell seed trapezoid TRAP3Q
  into 9 unit octiamonds, cross-confirmed per-component against the
  Bielefeld rule image, scores 0.911–0.991 vs runners-up ≤ 0.892; the
  blue-marked child is I+(2,2)): one trapezoidal 8-triomino prototile Q
  (8 unit triangles) on the triangular lattice, rep-9 with λ = 3,
  substitution matrix [9] → 9^n tiles, PF 9 = λ², all 12 orientation
  classes reachable (12×12 class matrix, rows AND columns sum to 9).
  Non-edge-to-edge — T-junctions INHERENT: measured census **8** (edge,
  foreign-corner) incidences in the rule's own depth-1 dissection
  (16 corner/edge pairs), seed-block ladder 18/226/2202/20286/183974
  over generations 0–4. The recon brief's 0-pin was a VACUOUS-metric
  artifact (unit-step boundary cycle — reads 0 for any patch, sphinx
  gen-1 included), adjudicated at integration with a hand-checked
  counterexample ((4,2) is a corner of R180+(4,2) and I+(4,0) strictly
  interior to I+(2,2)'s bottom edge) and machinery validated by
  reproducing the repo-pinned sphinx ladder 0/2/16/78/344 exactly;
  STOP protocol applied, measured pins landed. 12×6 parallelogram seed
  block = TRAP3Q ∪ R180 image (18 tiles, 144 cells, census-verified
  class multiset {1:2, 2:5, 4:1, 5:1, 6:1, 7:1, 9:5, 10:2}) tiles the
  plane exactly once under ⟨(12,0),(0,6)⟩; L2 nesting verified under
  the λ-scaled framing (0 overlaps, union 648/648, 81 distinct
  grandchildren — the brief's `3·t_j` grandchild formula was an index
  error, corrected to `3·t_i`); the Bielefeld patch's 1142 nine-tiled
  trapezoid panels form 6 table types (frequencies 259/196/186/183/
  162/156, one per conjugacy orbit of 12), all L1+L2-clean — iterable
  substitution structure in the patch. Aperiodic: no lattice period in
  any of the 4 patch crops (recon s25/s25b) →
  `TilingCategory::Nonperiodic`, translational "aperiodic" (NOT
  limit-periodic; the tile set admits periodic tilings, no einstein
  claim). Exact ℤ-lattice arithmetic end to end (H = √3/2 at emission
  only), tri-section-anchored depth quantization (EXTENT_UNIT 5.0,
  band (15,45] → depth 2), MAX_TILES 250_000 with pre/post guards.
  Battery: 11 in-module tests (L1 partition 72/72 max-cov 1 + rule
  uniqueness, L2 audit 81 distinct/0 overlaps, orientation closure 12,
  class matrix doubly-stochastic ×9, conjugation over all 12 parents,
  seed block 144/144 + plane tiling 0 missing/0 dups, T-junction +
  ladder census pins, growth battery) + 3 standard trio tests.
  Verification: cargo check --workspace zero warnings; cargo test
  --workspace 1059 passed / 0 failed. Census: Nonperiodic tab 7 → 8;
  reptiles family 6 → 7; built-ins 142 → 143; memberships 156 → 157.
- 2026-11-22 — M151: Andritz bowtie-and-hexagon `andritz-bowtie-hexagon`
  ✅ (Tilings Encyclopedia "Andritz bowtie and hexagon", Feb 2016 —
  Andritz 2016 fivefold substitution; islamic family, Quasiperiodic
  category): two unit-edge girih prototiles (bow-tie + elongated
  hexagon), inflation φ², substitution matrix [[2,3],[3,5]] with PF =
  φ⁴. Exact ℤ[ζ₅] σ-frame walk: the rule was derived in an exact Python
  prototype against the decoded Bielefeld reference patch (646 tiles,
  1383 ring-exact vertices) and re-verified in Rust — ring-exact unit
  edges and areas, the exact 253-pose inner patch reproduced
  pose-for-pose (both directions), level growth 27 → 168 → 1160, exact
  edge census (every interior edge shared by exactly 2 tiles) out to
  radius 8 at level 1 and 20 at level 2; edge-to-edge at every level.
  generate() runs a region-filtered wavefront BFS whose work stays
  linear in the view area via the inverse-image σ⁻¹ box chain. Battery:
  5 structural module tests + standard trio. Verification: cargo check
  --workspace zero warnings; cargo test --workspace 1068 passed / 0
  failed. Census: Quasiperiodic tab 14 → 15; islamic family 26 → 27;
  built-ins 143 → 144; memberships 157 → 158.
- 2026-11-22 — M152: Andritz 10gon-1 `andritz-bowtie-hexagon-10gon-1`
  ✅ (Tilings Encyclopedia "Andritz bowtie and hexagon 10gon-1", Feb
  2016 — the decagon-bearing member of the bowtie-hexagon fivefold
  family; islamic family, Quasiperiodic category): three unit-edge
  girih prototiles (bow-tie, elongated hexagon, regular decagon),
  inflation φ², substitution matrix [[2,3,0],[2,2,1],[5,0,6]] with PF
  = φ⁴. Exact ℤ[ζ₂₀] σ-frame walk: the rule was derived from the girih
  edge graph of the decoded Bielefeld reference patch (edge-BFS over
  ℤ[ζ₂₀], 3280/3280 shared-edge label agreement, zero conflicts) and
  cross-checked by per-instance tuple equality of every slot offset
  (26/26 bow-tie rows, 10/10 hexagon rows, 16/16 decagon flowers,
  22/22 identical taus). The frame rotates with the parent (ψ_B(r) =
  2r+3+10((r+1)%2), ψ_H(r) = 2r+8+10(r%2)); the decagon row is
  absolute (center + five edge-sharing petals at 2·apothem + five gap
  bow-ties), and decagon children carry no rotation; edge-to-edge at
  every level. Rust ring arithmetic validated against the Python
  Z[ζ₂₀] reference (kmul over t⁰…t¹⁹); the frame fixed point o is a
  walk invariant. Battery: structural module tests (unit edges +
  areas + C₂ symmetry, PF = φ⁴ + row-area identities < 1e-12, walk
  sizes [14, 86, 604], 54-pose decoded inner disk matched as sets both
  directions, exact edge census at levels ≤1 and ≤2, far-view graceful
  failure) + standard trio. Verification: cargo check --workspace
  zero warnings; cargo test --workspace 1077 passed / 0 failed.
  Census: Quasiperiodic tab 15 → 16; islamic family 27 → 28; built-ins
  144 → 145; memberships 158 → 159.
- 2026-11-23 — glb-binary-n7 plane-filling presentation (M149
  follow-up, no new tiling — non-M): `generate()` upgraded from the
  certified single-domain B₀ core (island, ~0.3% default-view
  coverage) to a PLANE-FILLING presentation matching the
  Penrose/Ammann–Beenker class: the parent patch is the seed-rhombus
  lattice (m,n) = V + m·a + n·b (exact `Tw` offsets, cls [0,2,14,16])
  over φ⁻¹(W + C₀) with SAT margin 0.05 float-safety culling (the
  V-lattice tiles the plane, so coverage is structural, not
  probabilistic), then refined d ∈ [3,5] levels by the SAME certified
  per-level DFS over the whole parent set — the eps-complementarity
  cascade meshes the lattice-cell seams, so the patch is ONE mesh, not
  per-cell islands (M149 probed and refuted any local (shape, class)
  map; cross-boundary meshing therefore runs the full compat
  predicate across shared parent edges). Certification: a 6-cascade
  `plane_mesh_audit` refines level-by-level and audits every level —
  edge parity (0 multi-edges), one simple closed rim walk, interior
  vertex links, shared-edge class agreement (0 bend-bit mismatches),
  bucketed all-pairs interior disjointness, and the exact fan-area law
  Σkids = λ²·Σparents (rel ≤ 3.3e-13); every audited patch spans
  multiple lattice cells, so the audit proves gap- and
  overlap-freedom ACROSS seed boundaries. Depth = largest d with
  area·λ^{2(d−1)}/0.75 ≤ VIEW_BUDGET (2500, floored at D_MIN = 3 —
  floor proven in-test); culling at emission only. Coverage census on
  the standard trio + a ±20 world-unit box: 0 holes / 0 multi / 0
  boundary-undecided AND continuous clipped-area identity
  Σarea(tile∩W) = area(W) to ≤ 2.4e-14 rel, all views; emitted tiles
  60–34 520 by view, all ≪ MAX_TILES 250 000; preview flow
  1241→974→982→281 count-stable, CLI pane 796 @ d=4. All 12 M149
  tests survive (10 untouched; default-view bands re-measured and
  re-pinned; `empty_view_outside_patch` → `far_view_covered` — its
  finite-domain premise is gone by design); σ²…σ⁵ staircase intact;
  standard trio bboxes unchanged. Module tests 12 → 15. Diff: the
  module + the quasiperiodic.md row note only; zero census deltas
  (registry id unchanged; family 13, built-ins, memberships untouched).
  Verification: cargo check --workspace zero warnings; cargo test
  --workspace 1080 passed / 0 failed.
- 2026-11-23 — M153: Andritz 10gon-2 `andritz-bowtie-hexagon-10gon-2`
  ✅ (Tilings Encyclopedia "Andritz bowtie and hexagon 10gon-2", Feb
  2016 — third member of the bowtie-hexagon fivefold family; islamic
  family, Quasiperiodic category): σ-frame map z ↦ τ − λz with
  λ = 3 + 2φ, μ = −(3+2φ), τ = (−3,0,20,0,12,0,24,0); three unit-edge
  girih prototiles (bow-tie, elongated hexagon, regular decagon).
  Exact ℤ[ζ₂₀] σ-frame walk with a FITTED per-slot orientation law:
  frames Ψ_D = 1, Ψ_K(r) = 2r + b_K + 10·[(2r mod 5) ≥ 3] (b_B = 5,
  b_H = 0); child r = (r+δ_j) mod 5 for B/H parents, absolute for DEC
  parents, 0 for DEC children; slot tables 27/38/96; D-row
  t⁴-symmetric, B/H trivial stabilizers (checked 1..19). KEY
  STRUCTURAL FINDING (documented, load-bearing): the clusters form a
  SHARED COVERING, not a disjoint inflation — PF(M) = 43.151 ≠ λ² =
  38.889, row areas exceed λ²·A by 7–17 %, 13 % of walk tiles
  double-claimed consistently; the module/tests therefore treat M as
  verified frame-claim tables (census rows [[11,14,2],[12,22,4],
  [35,35,26]] pinned, growth 41.758 vs PF within 4 %) and MEAN_AREA
  from the empirical patch census (3.0218392). The walk reproduces
  the decoded Bielefeld reference patch pose-for-pose both
  directions; edge census 50 @ r<4.5 / 187 @ r<8, every edge used
  exactly twice; coverage radii measured by ray-fire 3.2 / 20.15 /
  57.9 → COVERED = [3,20,57,160,460], MAX_LEVEL 4; region-filtered
  chain stays axis-aligned (μ real). One landing-2 template fix: the
  prototile ring test asserted e·ē = 1 but the canon rings here store
  doubled rings 2·(v−c), so unit edges have norm 4 — corrected to 4
  with the reasoning documented (the doubling is intentional and
  matches verts_exact/kv2 emission). Battery: 10 module tests +
  standard trio. Verification: cargo check --workspace zero warnings;
  cargo test --workspace 1090 passed / 0 failed. Census:
  Quasiperiodic tab 16 → 17; islamic family 28 → 29; built-ins
  145 → 146; memberships 159 → 160.
- 2026-11-23 — M154: Fibonacci × Fibonacci
  `fibonacci-fibonacci` ✅ (Bielefeld "fibonacci-fibonacci" entry —
  the LAST §23 salvage item; siblings pregnant-chairs M136, tangram
  M142, golden-rhomboid M144; quasiperiodic family): the 1-D Fibonacci
  substitution taken as a product with itself — 4 orientation classes
  (LL, LS, SL, SS) on 3 prototile shapes (φ×φ square, φ×1/1×φ
  rectangle, unit square; LS/SL one shape at two rotations). Linear
  inflation λ = τ² = φ²; module rule = TWO product steps (children
  tile the INFLATED parent, anchors scaled by λ — the naive sum
  misplaces every non-origin parent, spike-caught). Count matrix
  B = A₄² = [[4,2,2,1],[2,2,1,1],[2,1,2,1],[1,1,1,1]], the square of
  the 1-D product matrix A₄ = [[1,1,1,1],[1,0,1,0],[1,1,0,0],[1,0,0,0]]
  pinned from the entry's rule figure; char poly
  (x−1)²(x²−7x+1), PF eigenvalue τ⁴ = λ²; per-parent area laws
  (5,8),(3,5),(3,5),(2,3) = area·τ⁴ exact; exact dissection verified
  for BOTH the figure rule (φ) and the module rule (φ²). Battery:
  strict PIP + proper-crossing + T-junction checks at every growth
  step (none at any depth); growth totals F_{2d+2}² = 1, 9, 64, 441,
  3025 (d = 0..4), 20 736 (d 5), 142 129 (d 6) ≤ MAX_TILES 250 000
  (d 7 would exceed); exact patch bbox [0, φ^{2d+1}]²; Python↔Rust
  table diff: all 8 parent rows of both rule tables decode
  ordered-identical to the product derivation; tr(B^k) = 9, 49, 324,
  2209 exact (Newton). Exact Q(√5) integer pairs end to end, floats
  at emission only; finite patch recentered −φ⁷/2 (RECENTRE pin
  −14.517220926874318); depth cap 6; 4-fold point symmetry (LL rule
  verified 90°-symmetric); diffraction cited, not proven
  (golden-family convention). Presentation: scale default 1.0 (NOT
  the pre-spike 0.5 — at 0.5 the CLI pane exceeds the depth-3 patch
  and side-bands the background; pin_view.py f64 mirror produced all
  pins): probe ±10 → d = 3, 225 tiles; preview 143 tiles; CLI pane
  ±13.333×±10 → 315 tiles, check_coverage passing; rotated
  ±0.6 @ 0.6/25° → 9 tiles. Battery: 12 module tests + standard
  trio. Verification: cargo check --workspace zero warnings; cargo
  test --workspace measured at integration. Census: Quasiperiodic tab
  17 → 18; quasiperiodic family 13 → 14; built-ins 146 → 147;
  memberships 160 → 161. Orchestrator pin updates folded in (lib.rs
  census comment + per-tab vec + registry/membership asserts, tess-tui
  tab pins, README count/tabs/prose/catalog row, INDEX family size,
  CHANGELOG entry).
- 2026-11-23 — M155: Pentiamond AC factor-2
  `pentiamond-ac-factor-2` ✅ (Bielefeld substitution index entry
  "Pentiamond AC factor 2", folklore, rule image sole source; reptiles
  family, Nonperiodic category): λ = 2 two-prototile reptile on the
  triangular lattice — A → 3A + 1C, C → 2A + 2C with the achiral C
  double-labeled; M = [[3,2],[1,2]], char poly (x−1)(x−4), PF 4 = λ²,
  primitive index 1, per-A-seed closed form A_d = (2·4^d+1)/3; 3×5
  rectangular seed block (4 A poses + 2 C poses), whole-block
  homothety on the block lattice; limit-periodic construction with
  aperiodic limit — zero exact periods in the unit tiling (|v| ≤ 16,
  top near-miss 0.6953); chiral A balanced 1:1 at every depth;
  folkloric 9945-tile patch reproduced exactly by the pinned rule.
  Implementation notes: mirror dissections T1 = 8·T0 + (0,6) under
  pose pairs {g, g∘8}; canonical pin = table 0 for all poses (frame
  probe 108/108 + label invariance 24/24). One module root-cause fix
  during the battery: lam_scaled_keys must POSE each unit cell first
  (per-cell lex-min-vertex pose correction is kind-dependent) before
  scaling posed anchors — raw linear images of unit-cell anchors don't
  share a common translation frame (port-validated in Python against
  the spike ground truth before touching Rust: 168/168 key equality,
  120/120 pose equivalence). BRIEF DEVIATION (documented, load-bearing):
  brief item 6's cross-depth nesting claim ("each depth-(d−1) tile is
  the exact union of λ² depth-d tiles") is unsatisfiable and
  contradicted by the brief's own recon table (2→3 no-merges GENUINE);
  the rewritten `refinement_property` pins the true substitution
  refinement (per-tile λ² exact partition) plus the measured
  cross-depth census {62 disjoint, 34 bit-identical} @ d2/d3.
  Battery: 16 module tests incl. 3-test family trio.
  Verification: cargo check --workspace zero warnings; cargo test
  --workspace measured at integration. Census: Nonperiodic tab
  8 → 9; reptiles family 7 → 8; built-ins 147 → 148; memberships
  161 → 162.
- 2026-11-23 — M156: Pentiamond AC factor-3
  `pentiamond-ac-factor-3` ✅ (Bielefeld substitution index entry
  "Pentiamond AC factor 3", folklore, rule image sole source; reptiles
  family, Nonperiodic category): λ = 3 reptile — A → 6A + 3C,
  C → 5A + 4C (achiral C double-labeled, mirror pair via (8, (0,9)));
  A/C block [[6,5],[3,4]], char poly (x−9)(x−1), PF 9 = λ², primitive
  index 1, per-A-seed closed form A_d = (5·9^d+3)/8; 2×5 seed block
  (block charge −2 — the folklore patch's P2-heavy 617:1899 depth-2
  census, decaying to 1:1 in the limit, measured 0.9999 by d = 20);
  whole-block homothety on the block lattice; limit-periodic
  construction, aperiodic limit — zero exact periods (|v| ≤ 12, top
  near-miss 0.5712); folkloric 4024-tile patch reproduced exactly.
  D-ROW BLOCKER (flagged, never guessed): the entry header says 3
  prototiles but the D rule row is absent from the rule image AND the
  patch — implemented as A + C (2 prototiles), flagged in module
  header/description/registry classification/doc row; the A/C block
  is closed and the seed D-free, so the limit tiling is D-free at
  every depth regardless of the missing row (no sum-9 D column makes
  the 3×3 primitive — 55-candidate scan). Micro-deviation: λ = 3
  depth cap .min(37) (3⁴⁰ overflows i64), commented in code. Shares
  the M155 root-cause fix (posed-then-scaled lam_scaled_keys) and the
  documented refinement_property deviation (measured cross-depth
  census {235, 89} @ d2/d3); per-entry spike pins recalibrated
  108/108/972 (the prepass CHECK-5 numbers were cumulative).
  Battery: 16 module tests incl. 3-test family trio.
  Verification: cargo check --workspace zero warnings; guard subset
  green at integration. Census: Nonperiodic tab 9 → 10; reptiles
  family 8 → 9; built-ins 148 → 149; memberships 162 → 163.
- 2026-11-23 — M157: Pentiamond AD factor-2
  `pentiamond-ad-factor-2` ✅ (Bielefeld substitution index entry
  "Pentiamond AD factor 2", folklore, rule image sole source; reptiles
  family, Nonperiodic category): λ = 2 reptile — A → 2A + 2D,
  D → 2A + 2D (D dissection SELF-MIRROR — single table under both
  achiral labels); M = [[2,2],[2,2]] = rank-1 (x²−4x), PF 4 = λ²,
  primitive index 1, per-seed (2·4^{d−1}, 2·4^{d−1}); UNIQUE 1×5 seed
  block (2 D tiles, poses 2 and 8); whole-block homothety on the block
  lattice; limit-periodic construction, aperiodic limit — zero exact
  periods (|v| ≤ 10, top near-miss 0.6270); A balanced 1:1 at every
  depth; folkloric 9969-tile patch reproduced exactly (recon E PASS,
  coverage 1.0; the entry is depth-3 nested — the only one of the
  three, so its refinement_property carries the genuine cross-depth
  partition). Shares the M155 root-cause fix (posed-then-scaled
  lam_scaled_keys; measured cross-depth census {32, 0} @ d2/d3,
  {128, 0} @ d3/d4 — clean structure, never a split).
  Battery: 16 module tests incl. 3-test family trio.
  Verification: cargo check --workspace zero warnings; guard subset
  green at integration. Census: Nonperiodic tab 10 → 11; reptiles
  family 9 → 10; built-ins 149 → 150; memberships 163 → 164.
- 2026-11-23 — M158: WIS 12-fold (Watanabe–Ito–Soma)
  `wis-12-fold` ✅ (Bielefeld substitution index entry "WIS 12-fold",
  Watanabe, Ito & Soma, Acta Cryst. A 51:936, 1995; quasiperiodic
  family, Quasiperiodic category): equilateral triangle + square +
  30°/150° rhombus, dodecagonal symmetry, inflation λ = 2+√3
  (λ² = 7+4√3). Ships the ENTRY FIGURE's published rules (WSI95
  Fig. 2(iii) — the entry's non-unique ruleset), NOT the Bielefeld
  patch's undocumented ruleset: the figure's clusters have zero
  occurrences in the 792 k-node patch (tri 0/150 certified leaves, rho
  topologically distinct, sq 0 in the crop) and the patch is
  non-generic (no census oracle); the "correct set" attributed to
  Musesti–Paolini (2023) has no machine-readable record. Exact
  ℤ[√3] 4-tuple points (x = H(P+Q√3), y = H(R+S√3), H = s/2), edge
  vectors in the index-4 sublattice, floats only at emission.
  Non-conformal same-scale rules: children are prototiles at edge 2H
  (same size as the parent) in a jagged cluster whose union has area
  λ²·parent and CONTAINS the parent; the rho rule is a 3-piece
  disconnected cluster (9+2+2 tiles) with fitted lattice-snap piece
  offsets island (−2,−1,−3,−2) / pendant (2,1,1,0) — inter-piece gaps
  artist-chosen in the figure (fit residuals 0.0465/0.0378 H vs
  0.0928/0.0967 H as drawn; pendant offset a rhombus edge direction;
  the 13-tile rho supertile never occurs in the patch, so no patch
  census constrains the fit) — pieces share no edges and touch at
  exactly two pinches, each a 4-tile vertex with corner sum 8 (120°
  gap; pinch geometry figure-determined and certified).
  Certified level-1 (exact battery, spikes w10/w10b/w11/w12, ported
  in-module): every dissection one closed simple boundary per piece
  with all interior edges paired; 2-area identities
  (24,14)/(56,32)/(28,16) = λ² × parent; parent ⊆ union (exact Q⁴);
  angle census pinned (tri 16 = 1+15, sq 29 = 9+20, rho 20 = 2+18+2
  pinch, S(v) == want(v) everywhere); D2 non-composition pinned
  (drift guard: tri seed 155 tiles/770 overlap pairs, sq 358/3660,
  rho 179/1023, same-parent 0) — the rules do NOT compose at
  depth ≥ 2, so emission is the depth-1 single-parent square
  supertile (26 tiles, simply connected) as a finite patch, views
  outside are background (fibonacci_fib.rs precedent).
  Battery: 10 module tests (3-test family trio + 7 exact battery).
  Verification: cargo check --workspace zero warnings; full suite
  green at integration. Census: Quasiperiodic tab 18 → 19;
  quasiperiodic family 14 → 15; built-ins 150 → 151; memberships
  164 → 165.
- 2026-11-24 — M159: Girih-inspired 14-fold (Pautze)
  `girih-14-fold` ✅ (Bielefeld "Girih 14-fold" entry, Pautze,
  Symmetry 9(2):19, 2017; quasiperiodic family, Quasiperiodic
  category): 11 prototiles (R2/R3 rhombi, H26/H44/H45 hexagons,
  concave O29 + convex O36 octagons, concave D2 + D46/D56 decagons,
  regular 14-gon G14), inflation λ = 2b²+3b ≈ 6.850855
  (b = 2cos(2π/7), exact K₇ triple (0,3,2), Norm(λ) = −7),
  λ² ≈ 46.934215 = (8,20,9). Exact 1/2 ℤ[ζ₁₄] twist-pair geometry
  (dyadic K₇ pairs, sin(π/7) folded into shared exponents); certified
  rule tables (11 classes, 763 rule entries); corner-14-gon +
  edge-decagon + interior-kid panels (849 kids per full round);
  exact vertex-set dedup for straddling kids; G14 convex-fit seed
  generation with pad-expanded culling and MAX_TILES budget guards.
  Errata pinned in the per-tiling doc: the Bielefeld unit claim is
  false (Norm(λ) = −7); Pau2017 Table 7's λ = 5.801939 is
  contradicted by its own Figs 7.4/7.5 (measured 6.8507).
  Certified battery (9 items, ported in-module): rule-JSON rebuild of
  all 849 kid rings pinned entry-by-entry against the const tables;
  exact K₇ area law on all 11 effective-N′ rows; PF eigenvalue = λ²;
  module (dyadic exp ≤ 1) + 14-class edge-direction census; two-level
  R2@λ² patch (19 level-1 kids, 1813 pre-dedup → 1691 placements,
  emission histogram {1:1597, 2:76, 3:10, 4:6, 5:2}, 6474 AABB
  candidate pairs, zero overlapping pairs, dedup idempotence); O36
  rare-class coverage; λ measured on all 11 panels within 1e-3;
  panel render counts [19,17,26,45,43,51,66,67,87,105,323];
  budget/guard discipline ([0,60]² default → 1355 tiles).
  Battery: 12 module tests (3-test family trio + 9-item battery).
  Verification: cargo check --workspace zero warnings; full suite
  green at integration. Census: Quasiperiodic tab 19 → 20;
  quasiperiodic family 15 → 16; built-ins 151 → 152; memberships
  165 → 166.
- 2026-11-24 — M160: Andritz 10gon-3
  `andritz-bowtie-hexagon-10gon-3` ✅ (Tilings Encyclopedia "Andritz
  bowtie and hexagon 10gon-3", Feb 2016 — fourth member of the
  bowtie-hexagon fivefold family; islamic family, Quasiperiodic
  category): σ-frame map z ↦ μz + τ with μ = φ⁴·e^{72°i}; three
  unit-edge girih prototiles (bow-tie, elongated hexagon, regular
  decagon). Exact ℤ[ζ₂₀] σ-frame walk with frames Ψ_HEX = (8 − 8r)
  and Ψ_BOW = (4 − 8r) mod 20 from a 21-tile seed core; cluster
  census [[19,19,7],[26,29,9],[60,45,31]] mined from the decoded
  957-tile patch by unanimous union votes, then hardening pipeline
  (the landing's KEY FIX): raw union-vote slot tables (462/340/238)
  contained long-reach slots (|w|/2 up to 29.7) whose child claims
  land outside σ(parent) and decoded as accidental support — fired
  from far parents they became overlapping phantoms. Fix chain:
  region filter (child in σ(parent): |w|/2 ≤ φ⁴·circ(kind)) →
  45/66/136 rows, still 957/957 decoded at L2; then two
  configuration kills at zero decoded cost — H slot
  (13,0,-1,0,7,0,-9,0) (cleared the module test boxes, 233+46 → 0
  pairs) and far-field H (6,0,6,0,7,0,2,0), attributed as the source
  of all 12 remaining far-field H-in-D pairs against load-bearing
  D slot (-2,0,0,0,-2,0,1,0) (killing the D slot would lose 5
  decoded tiles) → final tables B 45 / H 64 / D 136; the frame law
  itself was certified optimal (Ψ optimal for 953/957 decoded, zero
  in-window violations for all 957).
  Shared covering again (42.7 % of walk tiles multi-claimed), so
  generation is by the verified walk. Battery (on the 1.76M-tile L3
  walk, all programmatic): decoded 957/957 reproduced with 0
  overlaps; walk r ≤ 16/30 → 0 pairs; window 683/683 exact decoded;
  all 3 module boxes exact clipped area (diff 0.0000, 0 gaps, 0
  overlaps); edge census exactly-2 to radius 13.1; conservative
  covered radii 3.4/23.4/164.2 → COVERED = [3,23,160,900]; 30-box
  far sweep at radii 45–450 → 0 phantom pairs (raw tables: 12 in
  region2, 279 in rkill5). Includes covers_distant_views and
  rotated_scaled (both failed on the stale tables — the fix's
  motivation). Battery: 11 module tests (3-test family trio + 8-item
  battery). Verification: cargo check --workspace zero warnings;
  full suite green at integration. Census: Quasiperiodic tab 20 →
  21; islamic family 29 → 30; built-ins 152 → 153; memberships
  166 → 167.
- 2026-11-24 — M161: Girih decagon patch
  `girih-decagon-patch` ✅ (ROADMAP "Full girih decagon patch" —
  the classic two-level girih figure of LS07; islamic family,
  Islamic category): a finite patch data table assembling one large
  level-1 decagon plus its 10 edge-adjacent large bow-ties, each
  large tile replaced by its closed Darb-i Imam cluster read from
  `girih_qc`'s certified tables (real children + the edge-marker
  trio on every unique large-tile edge — children-only leaves notch
  gaps). 611 small tiles = 270 bobbin + 220 bow-tie + 121 decagon
  (431 cluster children + 60 unique C-edges × 3 trios), 0 duplicate
  keys. Certification (Python spike + 15-test in-module mirror, all
  exact over Q(2cos 18°)): every tile edge unit; 4150 directed edges,
  1810 interior pairs shared exactly 2, 530 boundary, 0 T-junctions;
  1730 legal vertex fans; Euler χ = 2 (topological disk); exact
  10-fold symmetry; per-tile shoelace == table area (< 1e-12), patch
  total 1794.340677298660; DEC outline == IF²·A_DEC within 1e-9;
  coverage 33976 samples each exactly once; 61625-sample bbox sweep
  0 overlaps / 0 gaps; 467/611 tiles in the g2 lineage. Straps NOT
  shipped (stretch goal): strap chords cross the seed decagon's
  {10/3} star and planarizing the weave breaks the exact-partition
  contract — reported per brief. Not a LatticeTiling → no culling
  lines. islamic family 30 → 31; Islamic tab 25 → 26; built-ins
  153 → 154; memberships 167 → 168.
- 2026-11-24 — M162: isohedral (IH) type family — generator + five
  showcase instances
  `ih1-p1-hexagon`, `ih6-pgg-hexagon`, `ih7-p3-hexagon`,
  `ih27-pgg-pentagon`, `ih29-p4g-pentagon` ✅ (PROPOSED §13 — the
  Escher arc; isohedral family, Laves primary / PentagonTypes
  secondary on the two pentagons): a Kaplan/Tactile incidence-symbol
  engine over the 11 Laves topologies — vertex/translation/aspect
  linear forms as engine data, per-edge (fl, ro) orientation flags,
  adjacency symbol derived by exact geometric edge matching
  (BFS-closure verified in the spike, never hand-written); placement
  T_{a,i,j} = translate(i·t1 + j·t2) ∘ aspects[a] — a search-free
  (i,j)-box × aspects emission. All geometry in an exact Q(√3) field
  (canonical re + sq·√3, const-gcd i128 rationals): equal exact
  values are equal (re, sq) pairs, so one canonical f64 rounding per
  emitted coordinate makes every shared edge bit-identical; float
  params quantize to the 2⁻²⁰ grid and are handled exactly there.
  Five instances: IH1 p1 (3×J), IH6 pgg (2×U + 2×J), IH7 p3 (3×J —
  the Q(√3) record) hexagons on the 6³ net; IH27 pgg, IH29 p4g
  pentagons on the Cairo net (IH29's Klein-mixed straight edge;
  D-census asserts ROT 2 + ROFL 2 literally). Batteries per instance:
  coverage trio + irrational-offset exactly-once grid + shared-edge
  f64 bit-identity over a ±2 window (every (aspect, edge) pairing
  covered: 112 / 522 / 374 / 442 / 622 pairings) + exact pairing-D
  census + aspect-symmetry + transcription spot-checks vs the Python
  spike to 1e-9. The arc's hard-won fix (design doc §9.1): the engine
  initially read Tactile's 2×3 aspect matrix transposed — p1/pgg's
  symmetric linear parts hide a transposition (IH1/6/27 pass either
  way); p3/p4g rotations exposed it (IH7/29 failed coverage,
  aspect-symmetry, D-census, shared-edge); fixed against
  spike2.build_exact + tiling.cpp, first surfaced by the
  irrational-offset grid. Deviations: IH21 dropped (irreducible
  decimal coefficients — cannot pin exactly); pentagon_type
  cross-reference left None (classical 1–15 mapping unverified
  offline); `isohedral: Some(1)` = one isohedral class (classify.rs
  semantics), the IH type number lives in `method` text. The pinned
  17-card Wallpaper catalog untouched — the category rule held.
  isohedral family 0 → 5; Laves 28 → 33; Pentagon Types 15 → 17;
  built-ins 154 → 159; memberships 168 → 175.

- 2026-11-24 — M163: Kaplan §3.5 design-element motif library + three
  showcase instances
  `hankin-4-8-8-motifs`, `hankin-hex-motifs`, `najm-63-motifs` ✅
  (ROADMAP 22a; islamic family): `hankin_motifs.rs` — three
  parameterized d_n-symmetric design elements generated per substrate
  tile plus the per-tile motif-assignment entry point
  (`build_cells_with_motifs`, `None` keeps a tile plain). `Motif::Star`
  (n, s, θ) — one arm per edge contact, d_n both chiralities, cut at
  crossings, first s subpaths kept; `Motif::Rosette` (n, s, θ, h, φ) —
  knee at signed height h·(1−sin(π/n))·|A−M|, Lee's ideal at
  (π/n, 1, 0) pinned to 1e-13 (mpmath 40-dps reference 1.7e-16);
  `ExtendedStar`/`ExtendedRosette` — the §3.5.4 extension operator with
  the dissertation's θ/2 erratum corrected to θ′ = θ − π/n,
  r′ = r·cos θ/cos(π/n − θ) (battery-pinned across n = 5..12 × 3
  angles × star/rosette: F-identity 0.0, landings ≤ 9e-16). Merged
  figures are spurs ∪ post-knee rays — collinear-overlap-free for
  `arrangement_faces` (the collinearity parity check
  40Δ + (σ₁−σ₂)θ ≡ 0 (mod 180) killed θ = 40° on the enneagon).
  Additive non-breaking hooks in `hankin.rs`
  (`build_cells_with_segments` / `tile_faces_with_segments`, shared
  tail extracted as `assemble_cells`): regression proof dumps all ten
  engine-built hankin tilings' cell tables byte-identical before/after
  (292 lines, zero diff). Instances: octagon Rosette 22.5° + diamond
  ExtendedRosette 67.5° on the 4.8.8 substrate (14 cells, p4m); hex
  Rosette 30° + ExtendedStar 60° on 6³ (14 cells, pmm — the 2-colour
  assignment drops the substrate group, torus fold-test proven);
  dodecagon Rosette 15° + enneagon Star 35° on najm-63 (36 cells, p6m).
  Offset-tile spur-landing test (centre-relative, not origin-relative —
  real bug caught). Islamic 26 → 29; built-ins 159 → 162; memberships
  180 → 183.
- 2026-11-24 — **M164: Kaplan §3.7 decoration styles over the strapwork engine (22g).** The four ornamentation styles of diss. §3.7 land on the M57 walls layer, audit-first: **fill** (bipartite 2-colouring of the wall-map faces — the even-degree precondition audited across 24 substrates × 8 contact angles, 12/192 rows qualify, every classic δ among them), **outline** (equidistant-offset bands with mitered joins, l = d/sin(θ/2) derived from first principles and spike-pinned at 1e-9; clamp policy for near-degenerate θ), **interlace** (over-under at every crossing via strand-parity alternation, with the hard obligation proven: parity assignments are invariant under host-lattice translations, verified by ±2-cell torus folds on every classic-δ substrate — χ=0, one-over-per-crossing, no odd closures), and **outline-interlace composition**. Precondition gates are region-based (a covered patch cannot have an unshared host edge strictly inside the view — replaces the bbox margin that false-errored on sheared patches) with honest per-style errors at generic δ. Engine additions in `strapwork.rs` (planar map, strand walks, parity, faces, mitres, break gaps — incl. a real walk_one orbit-closure fix found by the audit), `render_svg_decorated` layered z-order in `svg.rs`, `--decoration/--band-w` CLI in tess-tui. Registered-module cross-check: wall segments ≡ partition internal edges (640/668/1030, 0 missing/extra) and wall-map faces ≡ registered cells face-for-face (301/301, 301/301, 491/491, area gap ≤ 5.3e-15); fill colour classes translation-invariant (1518 lattice-image comparisons, 0 failures). Plain render path byte-identical. No registry movement (162 built-ins / 183 memberships); tests 1273 → 1286.
- 2026-11-24 — **M164 fix (non-M): NaN band rings in the decoration modes.** Outline/interlace/outline-interlace emitted `<path>` data containing literal `NaN` at straight-through strand vertices (anti-parallel legs, θ = π): the join angle derived via `acos` loses precision near −1 (cos measured −0.9999999999999998 → θ = π − 2.1e-8, defeating the 1e-12 no-op test), the exact-mitre branch then divided by ~0 on the coincident offset parallels. Each affected vertex was independently poisoned — 10 824 NaN tokens in the single square@45/outline repro, ~10.6k outline / ~31.7k interlace-family cells across the 12-row matrix; per SVG grammar each corrupted ring rendered nothing. Fix: geometric straight-through detection (`|u·x| < 1e-6 && cos < 0` → plain offset feet, no mitre), hairpin/bevel clamp path unchanged; defense-in-depth emission guards (rings with any non-finite point dropped; over-lines skip non-finite segments). New `decoration_finiteness_battery` (tess-tilings): all 12 classic-δ rows × 4 styles assert zero non-finite coordinates and ≥1 decoration layer — the gap that let this through M164's battery (which checked layer counts and z-order, both NaN-blind). Post-fix: 48/48 matrix files NaN-free, ring counts unchanged, all landed pins green. Tests 1286 → 1287.
- 2026-11-24 — **M165: Islamic parquet deformations — per-contact-point δ over the strapwork engine (22d, diss. §3.4.1).** The contact angle becomes a function of the contact point's location: `walls_with_delta(tiles, f)` evaluates f at every edge midpoint (both tiles sharing the midpoint ⇒ the point-reflection ray-pair identity survives by construction — the per-point change is exactly hoisting the sin_cos into the edge loop); `walls()` remains byte-identical as the const-f wrapper. Three deterministic profiles (Linear/Radial/Sine, analytic, no RNG) exposed via `--deform` CLI (fixed-arity numeric params, honest parse/resolve errors, mutually exclusive with `--strapwork`). **Phase-A audit finding:** the closure envelope is a DISCRETE point set per substrate, not an interval — square (4.4.4.4) and truncated-square (4.8.8) close at exactly {45°, 90°}, trihexagonal (3.6.3.6) at {60°, 90°}, truncated-hexagonal (3.12.12) at {60°} only — derived by 1° + 0.1° fine sweeps, single-tile probes (4 walls at 45°/90° vs 40 walls at 46°) and an interior-exit probe (784/968 degree-1 exits are interior — genuine open-strand geometry, not window artefacts). Policy consequence: "clamp to the envelope" = snap-to-nearest-closed-angle (`snap_to_envelope`, ties→lower; NaN/empty→None), with `envelope_for(substrate)` + `validity_stats` exposed in tess-tilings (`envelope.rs`) for the 24h explorer. Geometry finding pinned by the battery: the §3.4.1 continuity identity is RAY-PAIR-level; wall-level piece equivalence does not survive varying f (mixed-angle seam tiles are genuine mixed-angle geometry — census-reported honestly, not special-cased). Fig. 3.7 reproduction shipped as the canonical smoke test (square, 1600×400 strip, two runs byte-identical). No registry movement (162/183); tests 1287 → 1300 (the worker's base predated the M164-fix finiteness test; re-derived at integration).
- 2026-11-24 — **M166: CAP tiling — the Baake–Gähler–Sadun cut-and-project tiling (aperiodic 11 → 12).** First instance of the projection (cut-and-project) generator class. The CAP tiling (arXiv:2305.05639) is the D-map deformation of the hat metatile lineage read as a TRUE cut-and-project scheme: shape vector (A, B, X, F, L) = (3φ, 3φ, 1, φ+ξ, 2φ), a φ² left eigenvector of σ̃₁ ⇒ limit-periodic structure (LIDS). Construction (paper §3): per view, a 96-grid anchor + wedge-seed probe, displacement-padded prune window, depth escalation 3..=6, deterministic-anchor BFS deformation map, full mark-ring emission, Sutherland–Hodgman region clipping (all four prototiles verified convex), exact clipped-area acceptance. Exact integer abcd arithmetic throughout — floats only at emission. Hat-lineage machinery duplicated verbatim (turtle.rs precedent, cross-referenced). Verification: walk tables close and match hat.rs `BOUNDARY` exactly; ring areas exact vs (√3/4)(15+27φ, 9+9φ, 14+22φ, 15+23φ) with corner counts 6/3/4/5 for all 24 orientations; depth-3 blob (442 metatiles, census (147, 22, 105, 168)) deforms conflict-free with the shape law exact in i64; pairwise convex-clip overlap census < 1e-9 with exact area sum and 210 complementary shared segments; blob nesting; census 10/64/442/3025/20737/142129; all 24 orientations occur; render smoke (39 tiles, 4 proto classes) consistent with the paper's frequencies f (F≈.38 > H≈.33 > P≈.24 ≫ T≈.06); Python batteries: BFS conflicts 0 at depths 1–3, emission clipped-area rel exactly 0.0. New: `families/aperiodic/cap.rs`, `docs/tilings/cap.md`, `docs/cap-cut-project-design.md`. Tests 1300 → 1311.
- 2026-11-25 — **M167: medallion interface — design doc + exact catalog re-derivations (24a, PROPOSED §24).** The description language for self-contained ornament modules (the medallion program's first sprint): a planar map on a declared convex hull + boundary contact points + ray exit angles + internal figure, all in exact fields, geometry-agnostic (absolute-geometry instances plug in later, 22f). Deliverables: `docs/medallion-interface-design.md` (§0 summary; §1 schema + normalization + the cos²-only angle convention + contact/exit semantics; §2 hole-primary placement + invariance lemma; §3 the three absorption obligations — (a) exact substrate landing, (b) inference obligation, (c) exact path absorption; §4 the five catalog records; §5 geometry-agnosticism flags; §6 spike numbers; §7 module shape; §8 deviations) and `crates/tess-tilings/src/medallion/` (mod.rs: Field/Contact/Absorption + the `Medallion` trait, `Zq` exact sign engine, `Qp` quadratic-field ops, the rot30 kernel, strictly-CCW/normalized-frame checks, exact `first_hit` with fraction-pair t; five instance files + host312.rs). Data + tests only — no engine, no `Tiling` impl, no registry change (163/184 unchanged). The five catalog re-derivations, each in its exact field with ray-exit cos² censuses: M53 khatam pic-4-8-8 (Q(√2), {8/2} chord octagon, 16 rays, absorption (a) exact — hull = landed truncated-square octagon at 1e-9); M56 pic-3-12-12 core (Q(√3), {12/4} chords, 24 rays, (a) exact — hull = landed truncated-hexagonal dodecagon); M161 girih decagon panel (level-10 cyclotomic, den 64, {10/3} strap, R² = a²−1 exact, strap closure exact, hull = landed DEC_V exact, path (b) verification deferred to 24b by design); {6/2} hexagram at T = 1/2 (Q(√3), tip triangles, exact chord-line landing + tangency); M158 WIS 12-star (raw Z[√3] 4-tuples, absorption (c) exact — rot30⁴ + V₀↦w₀ absorbs all 24 outline edges, every first hit on a dodecagon vertex). Spike battery (stdlib Python): schema 88/88, ray tables 5/5 instances, landing identities 0 misses over 16+24 rays; Rust 37 tests. Final-session root causes: the khatam substrate octagon lattice needed the half-cell offset ((2i+1)A/2, (2j+1)A/2) — the buggy placement produced 405 overlapping midpoints and a spurious t = 1−√2/2 hit class (Python-verified both ways); WIS tip-frame search needed a cyclic-polygon-order guard; girih R² pin corrected to a²−1; Zq::sgn discriminant typo. Doc-numeric audit: every design-doc number re-derived against the batteries (M53 hull 2+2√2, M161 hull area (5/2)a³−5a, strap face areas with the census verified to sum exactly to the decagon area). 24b consumes first: host312 + the WIS record (bind → shoot 24 rays → first-hit certificate = the absorption-engine template). Tests 1311 → 1348.
- 2026-11-25 — **M168: taylor-socolar plane-filling presentation — the CHT closed form (aperiodic, no census movement).** User directive "where applicable, fill the plane": `taylor-socolar` upgrades from the shipped forced 529-hexagon patch (M86, Fig. 18 of Socolar–Taylor) to the paper's actual plane-filling tiling — the CHT (centered hexagonal tiling), the specific LI-class member containing Fig. 18. Per-site O(1) state from the paper's §5 closed form: mirror frame `w = (v−1,u)`, level/key = min v2 of the frame coordinates, the 3-entry axis table `Ta: (0,1)→1, (1,0)→2, (1,1)→0`, panel decomposition (n ∈ {1,2,3}) with signed `div_euclid` floor division (truncating division is WRONG for i < 0), pBS/pRB via Q(n) = 2^{v2(n)}, parity p = pBS^pRB (Thm 5), and the 12-ray paper-folding free-bit table (Pfold ≡ A014577) with the census-pinned (b,c) constants (winner q-pattern 101011110000 — unique zero-census choice of 16384 at R=6, surviving R=14). Center c = (0,1) at state (0,0,1) — the unique cstate with R1 = R2 = 0 (the other 11 are test-only). Geometry untouched: same 8 convex pieces per hexagon, same decoration; `SITES` becomes test-only; the island contract test flips to a beyond-patch coverage test (clipped-area equality 1e-6 at a view well outside the old rhombus). Certification (all programmatic, pinned numbers reproduced exactly): closed form vs shipped table 529/529; off-spoke Thm-5 + per-bit identities 440/440; Pfold ≡ A014577 64/64; hex-window R1/R2/R3 zero-censuses at R=6/8/10/14 (0/684·0/612·0/648, 0/1200·0/1104·0/1152, 0/1860·0/1740·0/1800, 0/3612·0/3444·0/3528); the 12-cstate sweep (R3 = 0/3528 for all — Thm 3). Also verified: the citation miscite (arXiv:1008.3103 → **1003.4279**, JCTA 118 (2011) 2207–2231 — verified against arXiv directly; the local paper extract was right); the module's six 1008.3103 sites themselves were missed at the time and are fixed by the follow-up citation chore (git -S shows no commit ever added 1003.4279 to the module before it). The module header's stale 57.6% inflation-fit admission is resolved: the study's convention search missed the corner center + mirror frame; with the pinned construction the off-spoke closed form is exact. Tests 1348 → 1353.
- 2026-11-25 — **M169: (n,c)-monster medallion catalog entries (24c, medallion program — no census movement).** Kaplan's monsters (diss. §3.10: "the union of a pair of regular n-gons arranged so that exactly c full edges of each polygon lie inside the other", n > 6, 2 ≤ c ≤ ⌈n/2⌉−2) become exact catalog entries in the 24a medallion interface — six records: (8,2), (10,2), (10,3), (12,2), (12,3), (12,4), each as the **parallel representative pair**: one convex n-gon Medallion record placed twice (A = U, B = U + P), because the union is non-convex (24a's schema requires a strictly convex hull) and a half-turn about the centers' midpoint swaps A/B for any displacement, making the "c of each" definition automatic. **Findings recorded, not forced:** (i) a float census of rotated placements finds NO equal-count windows — every catalogued placement is a parallel pair, and the contact angle d·π/n is realized by the records' {n/d} star figure, not by a rotation; other contact angles (e.g. 45°/75° at (12,2)) need cos Δ ∉ Q(√3) — a shipped-field gap, documented; (ii) the displacement is pinned at the unique crossing-midpoint coincidence (P = M_i − M_j; t(ρ) strictly monotone through 1/2 exactly at the pin — battery-asserted), which for five of six entries is the midpoint of the certified c-count window; (iii) (12,2) forces d = 6 — the 90° {12/6} figure where the two exit rays at each contact coincide (cos² = 0), recorded honestly; M163's extension-operator guard excludes θ = 90° from extension while the standalone star emits the perpendicular arms. **M163 consistency:** Motif::Star's segment lines match the records' exit lines for all six records (a mismatch would have been a STOP). Windows certified by an exact vertex-on-edge-line event census; union outline = one exact cycle of 2(n−c−1)+2 nodes, overlap lens = 2c+4. n = 9/11 blocked on unshipped cubic/degree-5 fields; {8,12}/{9,12} strange-star sets deferred (separate placement technique, out of base scope); the Bonner/Castéra menagerie overlap-motif drawings are not recoverable from the text source — geometry is pinned, motifs never guessed from pictures (24b/24e target). Data + tests only (no Tiling, no registry entry): 17 new tests, all exact-field (Zq tuples (a+b√d)/den; L10 basis for n=10; f64 only in consumer views and the M163 cross-check). Tests 1356 → 1373.
- 2026-11-25 — **M170: PIC explorer — deterministic δ/θ sweep with programmatic scoring (24h; no census movement).** The generate-and-filter stage the pipeline lacked: `tess-tilings/src/sweep.rs` sweeps a host tiling's contact angle and scores every candidate programmatically. **δ axis:** multiples of 5° from 5°..85° per lattice substrate (112 ids) plus envelope-member refinement points for the 4 `envelope_for` substrates. **θ axis (islamic-gated):** per-class single offsets from {−15,−10,−5,0,+5,+10,+15}° on the two classic hankin substrates (4.8.8, 3.12.12). **Per-candidate feature vector:** validity (core vertices/exit/corner/bad/straight2/joint), wall map (degree histogram, strand count, face count, min face area), geometry (total wall length, strap density, crossings, min crossing angle), symmetry (translation/mirror/rotation invariance), feasibility, documented score (weights stated in-module), rank — honest named refusals, never silent gaps. Deterministic leaderboard (JSON+CSV, documented sort; two-run byte-identity test) and a top-K SVG gallery via the M164 decoration API. **CLI:** `tess-tui explore --tiling <id> [--out|--csv|--gallery|--top-k]` — pure-parse arm, honest named errors, 6 parse tests. **Sweep results:** 112 substrates, 1921 rows, 0 refusals, 0 closure violations. Leaderboard head: truncated-square δ=90° (p4m, score 0.686), square δ=90° and δ=45°, trihexagonal δ=90° — all envelope members, all validity zeros, full symmetry invariance; hankin-4-8-8-motifs θ offsets −5…+15° all partition completely (err 0.0). **Findings recorded:** the hankin-4-8-8 δ axis is honestly poor — the substrate's `tiles` are the 14-cell face partition, not the canonical octagon+diamond substrate, so θ is the meaningful axis for hankin substrates; the 3.12.12 classic motif assignment (Rosette(12, 15°, s=3) + Rosette(3, 60°, s=3)) is the worker's documented Kaplan-reading choice, feasible at all 7 offsets, pending a canonical landed assignment. tess-core strapwork +141 lines (the varying-f ray-pair continuity test + sweep-facing API); lib.rs gains `pub mod sweep` and drops an unused `Tiling` import from the M164 finiteness battery (test-module imports sit outside plain `cargo check`'s view — the gate now runs `--all-targets`, next entry). Winners are CANDIDATES, not entries — exact-pin registry follow-ups queued; the freedom-set records feed 24e. Sweep 12 + explore 6 + core 1 new tests: 1373 → 1392.
- 2026-11-25 — **M171: (9,c)-monster catalog records — exact cubic-field placements via the rotated family (extends M169/24c; no census movement).** The M169 note "n = 9 blocked on unshipped cubic fields" was half-stale: K₉ = Q(2cos(π/9)) shipped at M137/M139 — what was missing was the machinery. Building it surfaced a structural theorem before any code: **the parallel family is provably pinless at odd n** (T1 parity: no axis-parallel midpoint pin exists; the off-axis parallel pins Δ ≡ 2,7 carry TWO contact angles, 80°/40° — not shippable). The ship instead uses the **rotated π/9 family** — the odd-n half-turn family (−U₀ = R_{π/n}U₀ as vertex sets), so the M169 pinning architecture carries over intact with pin rule P = M_i − R·M_j. Two records: **(9,2) = Monster93 {9/3}** (θ = 60°, d = 3, cos² = 1/4 exact `Some((1,0,0,4))`, pin t = s = 1/2 exact midpoint) and **(9,3) = Monster91 {9/1}** (θ = 20° = π/n, d = 1, cos² = (2+b)/4 is cubic → `None`, pinned `cos293` exactly on the extension-guard boundary — the {12,2}-at-π/2 precedent; its window pin is NOT the midpoint, documented). (9,3) **exists**, overturning the earlier floor-band claim, which only saw parallel pins. Coordinates are exact `Nq = (u + v·w)/den` over K₉ (w = 2sin20°, w² = 2−b ∉ K₉, gcd-reduced, exact sign) — one additive `Field::K9` variant in the medallion hub (zero consumers outside n9, grep-verified); no decimal literals in record data. Band test corrected to ⌈n/2⌉−2 as `(n+1)/2 − 2` (identical for even n). Structure: union 2(n−c−1)+2 = 14/12 nodes, lens 2c+4 = 8/10, exposed n−c−1 per side — the M169 formulas hold for n = 9, hub `outlines()` cross-check green (the interim "cannot be reused" claim was wrong). **Certification hole found and fixed en route:** `verify_pair9`'s "out of Xk" crossing-adjacency assert was unreachable dead code (X nodes hit `continue` first); `certify` rewritten as a `(here, next)` pair-match so both half-edge pieces at every crossing are exact-tested. M163 exit-ray cross-check extended to **all eight catalog records** — both nonagons pass (18/18 exit-ray lines each). Hub docs rewritten: catalog table with Nq notation legend, d-forcing rule d ≡ 2(i−j)−1 (mod n), window certification (both exact σ-windows + probe behavior; exact probe battery settled (9,3) below-window as (2,2)-with-six, not (3,3)), and a new {9/1} novelty section. 7 new tests: 1392 → **1399**.
- 2026-11-25 — **M172: composition family — dodecagon-hexagram-honeycomb (24d instance 1, REDUCED SCOPE; Islamic 34 -> 35, built-ins 163 -> 164, memberships 184 -> 185).** First composition-family registry instance on a periodic net: honeycomb [6,3] host R = 1, lattice a1 = (sqrt3, 0), a2 = (sqrt3/2, 3/2), Hexagonal, group p6m, seeds 0/1/2. 9 tiles/cell = 1 dodecagon medallion (R = 1/2, vertices 30k deg) + 2 kissing {6/2} hexagrams (T = 1/2, tips = hexagram_62::tips_natural() rotated 30 deg, notches sqrt3/6) + 6 reflex interstice hexagons (2 dodecagon + 4 star edges, area sqrt3/6 - 1/8). Sigma areas = 3sqrt3/2 = |det| EXACT over Q(sqrt3); every shared vertex emitted as an integer (p + r*sqrt3, q + s*sqrt3)/12 tuple, so tile boundaries meet bit-identically. **Reduced scope per the brief's own Gate-2 clause** (24b still in flight): the DUAL-NET fill reading shipped — 0 seams, the circumcentric dual passes vertex-to-vertex through the partition (net-edge midpoints = dual-edge midpoints; each interstice hexagon has two opposite vertices on a dual line, glue sites documented); the INFERENCE-fill reading is explicitly DEFERRED to 24b — the contact figures + dual-line glue sites ship as the 24b feed (per-tiling doc "Fill reading" section + family arc 82 + module header). No STOP: contact geometry uniquely determined, every quantity in the shipped field. Name proposed-descriptive ("Dodecagon-Hexagram Composition (Honeycomb)") — no canonical literature name found (two web searches), id collision-checked. Verification (all programmatic): python authority batteries in .scratch/composition-24d/ (441 tiles over +-3x+-3: interior-area sum == boundary-walk area; 1628 edges shared exactly twice, 272 boundary, none >2; 0 vertices with angle sum != 360 — figures (120,240)x520, (150,210)x294, (60,75,75,150)x260, (60,60,120,120)x114; 5029-point fine grid at an irrational offset: 0 uncovered/0 double-covered; p6m probes rot60 31/31 + mirror-x 31/31 with rot30/rot90/mirror-15 negatives rejected; exact-tuple battery: cell area^2 (0,432)/144, dodecagon (216,0), star (0,72), interstice hexagon (-36,48), all 5 contact/kiss identities True, the 36 interstice hexagon edges land dodecagon:12/starA:12/starB:12 uniquely mod the lattice, rot30(tips_natural) == TIP set exactly). Rust: standard trio + 8 exact-battery tests (11 total); two bring-up bugs caught and fixed by the batteries (interstice hexagon area pin z(-3,4,12); the notch chord identity is CENTER-relative — 2(mid - c) = 3(notch - c) — the origin-relative form fails because the star is not origin-centered). Guards green including lattice_culling_matches_brute_force with the new family lattice_tilings() entry (no lib.rs culling line needed). Tests 1399 -> 1410.
- 2026-11-25 — **M173: Ammann A4 — the rectilinear chair set as an exact substitution tiling (Quasiperiodic 16 -> 17, built-ins 164 -> 165, memberships 185 -> 186).** The third rung of the owner's A1–A4 request (A2 = golden bee M104, A1 parked: no substitution structure), landed by the worker tier from the ammann-a3-a4 brief. Two prototiles in 10 oriented states — a chair (S_c^2 square, S_c = 2+sqrt2, minus two opposite unit corner notches) and a half-chair (S_c x lambda rectangle, tall or wide, minus one sqrt2 x 1 corner notch) — inflated by lambda = 1+sqrt2 (silver mean, same ring as Ammann–Beenker A5, which is why the Quasiperiodic family is the consistent home: A5 is registered there). All vertices and child anchors exact Z[sqrt2] pairs (m + n sqrt2); floats only at emission. The 10 rule tables (children + anchors) pinned verbatim from an independent Python certificate and re-asserted in-module: state shapes axis-aligned with exact areas A1 = 4+4sqrt2 (chair), A2 = 2+sqrt2 (HC), bbox + notch corner + dims, R90/Mx closure; D3b achirality (R90(rule_0) == Mx(rule_0) == rule_1 exactly); per-rule diagonal covariance; Sigma child areas = lambda^2 * A_parent EXACT for every rule. Count matrix [[3,4],[2,3]] (rows chair/HC, columns parent chair/HC), Perron lambda^2, Perron ratio 1:sqrt2. Non-edge-to-edge — T-junctions intrinsic to the rule frames (8 per chair rule, 5 per HC rule, 56 at level 2). Single-chair seed, inradius 1.0*scale. Honesty note pinned in the card: the 8-fold symmetric model-set claim is CITED (mld to Ammann–Beenker per the Bielefeld encyclopedia), not proven — model-set/diffraction FLC scope note; the Bielefeld finite patch ratio N_chair ~ 2014 : N_HC ~ 2798 deviates 1.78% from the Perron sqrt2 (finite-size effect, not a rule error). Wired via families/quasiperiodic.rs (register/classify/IDS/family census 16 -> 17; not a lattice tiling — no culling lines); family doc heading + catalog row + per-tiling link; new card docs/tilings/ammann-a4.md (H1 = registry name). Integration: lib.rs census comment M173 note + pins (tab vec index 8 21 -> 22, 164 -> 165, 185 -> 186); README census sentence/tab prose/family-table/eleven-families; INDEX quasiperiodic row 17/15 (no build arc — the family is already non-1:1 at 16/15); tess-tui app_ui.rs pins (membership literal + comment, tab vec, tab-bar count); PROPOSED A4 status + ROADMAP Ammann-sets line; CHANGELOG M173 entry. Landed pre-outage as e3fc839 on the worker's lagging base (fb027ed); cherry-picked clean onto post-M172 main (no conflicts), census re-derived at integration (worker's message claimed 153 -> 154 on its base — the deltas are the same +/-1 pattern).
- 2026-11-25 — **M174: 24b sprint 1 — hole-bearing substrates + the boundary-matched inference engine (medallion layer; NO registry change).** The core novel algorithm of the medallion program, landed by the frontier tier from docs/briefs/hole-inference.md. Four files, all inside medallion/ (1608 insertions): hostwis.rs (438 lines) — the WIS hole-bearing substrate fixture: a 20-edge CCW blob hole (exact 2-area (56,32), 8 reflex vertices), the hull-reading convention (24 ray exits → 20 strictly-inside instances), 596/656 substrate segments, 126/134 ray lines, the anatomy∩closure vertex set (40/56), H[k] = rot^4(H0) symmetry, SHIP_COS2 cos² 13-class table, the shipped census [16 TRI, 4 SQ, 6 RHO] pinned as ground truth with SQ_SUP as the declared (circularity-documented) answer key, 9 pinned tests including guided-walk reachability 26/26 at 4 configs; infer.rs (1079 lines) — the engine consuming ONLY the hole polygon + HULL_TBL/NOTCH_TBL/CENTRE_TBL: ray-line construction, substrate segments, deterministic backtracking jigsaw (frontier = first uncovered boundary edge CCW else minimal open edge; candidates [TRI, SQ, RHO30, RHO150]; constraints C1 boundary-exact / C2 level / C3 edge-to-edge + rule-3 T-junction kill + disjointness + hole closure + wedge / C4 exact 2-area + area identity), self-contained Z[sqrt3] exact arithmetic (i128 sgn3), all iteration over total orders (BTreeMap/BTreeSet, sorted-pair EK keys — order-invariant), and the freedom Report{by_census, fps, seq_hash, exhausted, invalid} making inference freedom a FIRST-CLASS output (the 24e feed, as designed); holes.rs (88) — the fixture module. Verification (all programmatic, exact arithmetic): exhaustive Segment-K20 battery 6480/6480 fills, 0 invalid, census fingerprint {(16,0,14):324, (16,1,12):1377, (16,2,10):2268, (16,3,8):1782, (16,4,6):648 <- shipped, (16,5,4):81} with the shipped census IN the fingerprint set; RayLineVset-K20 exhaustive: identical fill set across two independent constraint systems; RayLine capped run aborted >500 solutions, 0 invalid, tri=16 AND 2*sq+rho=14 in every class (freedom documented, never forced); determinism byte-identical re-run; field membership/finiteness legal-lattice checks. Ground-truth bar: M158 WIS reproduced from the general path; M161 Darb-i Imam STAGED behind a clean documented gap — the 14-fold field Q(2cos(pi/10)) is degree-4 and needs a new exact order/sign kernel + re-derived candidate generator (transfers: frontier rule, C1-C4, rule-3 kill, freedom output, determinism skeleton — the kernel is the degree-field-doc queue item). Un-gates 24d's inference-fill addendum + 24e. NO registry/census change (built-ins/memberships unchanged); tests 1422 -> 1431 (9 hostwis tests; also corrects the M173-era test literals, which were off by one — the M173 suite line quoted "1288 passed" from the pre-edit run, but the true M173 base was lib 1289 / total 1422, since A4 carried 12 tests, not the 11 quoted at M173). Delivery note: the fleet-outage crash zero-filled this job's untracked files mid-flight; the work was recovered BYTE-EXACTLY from the session DB (write payloads + all 10 edit calls replayed in order, every edit a unique match, cross-checked against the harness's own unified-diff patches) and re-verified (zero warnings on a fresh private target dir). The 6 lib-test warnings reported on the delivery base (b7c3e60-era) do not exist on current main (verified: named sites are doc-comments/cfg attrs; check --all-targets + test-build grep both clean).
- 2026-11-25 — **M175: 24h exact-pin follow-up #2 — the canonical 3.12.12 motif assignment settled against Kaplan's saved Taprats design; sweep θ-axis classic assignment replaced (no census movement).** The M170 open finding ("3.12.12 classic motif assignment is a documented Kaplan-reading choice pending a canonical landed assignment") is settled by the strongest available evidence class: Kaplan's OWN saved design. A census over all 24 official Taprats Example Designs finds exactly one 3.12² design — `Water Rings.tap` — parsed with a strict spec-compliant Java-serialization parser (exact consumption 0xd46/0xd46 bytes, 69 objects; the same parser consumes all 24 example files exactly, so the read is structural, not heuristic). Extracted assignment: dodecagon feature → Star(n=12, d=3, s=1), the classic [12/3] single-strap star, whose chords leave each edge-midpoint contact at exactly 45° (measured 45.000000000000°); triangle feature → ExplicitFigure (6 verts/6 edges) ≡ Star(3, 45°, 1) — vertices match to 4.3e-15 with identical edge multisets. Both classes MISMATCH the interim landed rosette reading (Rosette(12, 15°, s=3) / Rosette(3, 60°, s=3)); decisive corroboration: Taprats' own default rule is Rosette(n, 0, 3) for n>4, so Kaplan saving a star for the 12-gon is a deliberate authored override, not a default artifact. External agreement: the fixed θ=0 row partitions with 21 cells = the M99 registered hankin-3-12-12 face count (error 0, p6m), and all 7 offsets (30–60°) partition completely. Change: `sweep.rs` only (+28/−15) — `classic_motifs_3_12_12()` → Star{12,45,1}/Star{3,45,1}/Star{3,45,1} + provenance doc + test pin; the REGISTERED hankin-3-12-12 / pic-3-12-12 tilings are untouched (their geometry is its own verified work; the θ-axis overlay is the explorer's motif assignment the M170 finding was about). Verification: cargo check --workspace --all-targets 0 warnings; lib suite green on the folded tree (1298); full workspace suite green incl. both explore byte-determinism tests. Census: no movement (165 built-ins, 186 memberships, islamic 35); tests 1431 → 1431 (test pin edited, none added). Delivered by the strong tier; parsers/verifiers banked at .scratch/hankin-31212-canon/.
- 2026-11-25 — **M176: 24h exact-pin follow-up #3 — the hankin-4-8-8 θ-axis host verified canonical; M170's "substrate tiles are the 14-cell face partition" premise corrected (no census movement).** The follow-up-#3 brief premise — that the sweep's θ-axis substrate for `hankin-4-8-8-motifs` is the panel's 14-face RESULT partition — is FALSE: `sweep::theta_axis_tiles` serves `hankin_4_8_8::tiles()`, which IS the registered `truncated-square` (canonical octagon+diamond host; bit-identical f64 vertices, same A·Z² lattice). Host identity pinned in Rust (vertex multisets equal mod lattice; areas diamond 1 + octagon 2(1+√2), Σ = 3+2√2 exact; envelope {45, 90}) — a dedicated host id or θ row would be a byte-identical duplicate of the M163 offset-0 row, so the honest delivery is pins + docs. Both registered readings certified exactly on the one host. Bar A: θ=0 with the M163 motifs reproduces the registered `hankin-4-8-8-motifs` partition BYTE-IDENTICALLY (to_bits, every vertex); 14 faces, lens [4×4, 6×8, 8, 32]. Bar B: the δ=45° wall network on the host is exactly 28 fragments/cell (8 octagram chords × 3 + 4 diamond chords) and the fragment set EQUALS the registered `pic-4-8-8` edge-orbit set (56 directed -> 28 orbits mod lattice); an independent Python full-network certificate (5×5 patch -> 300 chords -> 700 fragments -> 322 bounded faces) matches every face to registered pic cells by exact vertex-set, 14/14 orbits per cell, Σ exact — the cert caught and fixed a real Q(√2) numeric-sort bug (tuple ordering ≠ numeric ordering in the field). Headline negative: θ=0 (the M163 rosette reading) and δ=45° (the pic reading) do NOT unify — two distinct 14-face partitions of the one host, 0/14 shared face areas at 1e-9 (pic areas exact at 1e-12: 1/8, 1/8+√2/4, 1/2, 1+√2). Change: `sweep.rs` only (+292 — module-doc section, 5 tests, 3 helpers); no production code, no pub-surface change. Verification: cargo check --workspace --all-targets 0 warnings; full workspace suite 1431 -> 1436 green on the delivery base (lib 1298 -> 1303, +5 = exactly the new pins; 1440 expected on the folded tree with the GUI-cache tests). Census: no movement (built-ins 165, memberships 186, islamic 35). Status flips: ROADMAP 24h follow-up #3 (worker-proposed wording) + the stale "Remaining candidate: 3.12.12" line settled to M175; PROPOSED 24h follow-up #3 block; lib.rs census M-list M176 note. Delivered by the strong tier; certs/report banked at .scratch/hankin-488-canon/.
- 2026-11-25 — **24d inference addendum: the deferred hole-inference fill reading on dodecagon-hexagram-honeycomb is NOT APPLICABLE (certified; no code change, no census movement).** The 24b engine (infer.rs: unit-kind-fixed tile areas :252; one-30°-pencil candidate generator :315; exact unit-area completion identity :717; WIS-record substrate :413) consumes only holes whose boundary is unit edges of one 30° pencil with a WIS hull reading and a unit-semiring area. The composition interstice (2 dodecagon + 4 star edges; interior angles exactly 210/75/120/120/120/75; 2-area (−36,48)) fails all three premises, provably and empirically: (1) its boundary edges lie in two disjoint 12-ray pencils offset 15° (star 0+30k, dodecagon 105+30k; separation cos² = (2−√3)/4 exact) while one engine run reaches one pencil only; (2) the two 75° corners admit no sum of tile corners (every cover sum ∈ 30°ℤ); (3) the C4 Diophantine identity has no nonnegative integer solution at any engine scale (at the star-edge unit scale k = 8 forces rational remainder −3/2 < 0; the same solver admits the shipped WIS blob census [16 TRI, 4 SQ, 6 RHO] with area (56,32), confirming the setup is the engine's own C4). Engine probe: 0 instances derived from this boundary (K20 = K24 = 0), 0 solutions with exhausted = true at Plain/RayLineVset/Segment × K20/K24 — complete searches. Resolution: the dual-net reading is this instance's only fill reading; a 75°-bearing prototile alphabet would be a new engine, not an extension. Certificate + probe banked at .scratch/24d-infer-addendum/. Delivered by the strong tier as a STOP (no commit from the worker); status flips in the card, module header, ROADMAP, PROPOSED.
- 2026-11-25 — **24b sprint-2 gap reclassified: the degree-4 girih field kernel already exists (L10, M139); PROPOSED's "14-fold field Q(2cos π/10)" was a label error; the real gap is engine-side (docs-only; no census movement).** The fields-doc job returned a STOP with the premise already-true: fields.rs ships L10 = Q(a), a = 2cos(π/10) = 2cos18°, minpoly x⁴−5x²+5 (ML10 :1071, RED_L10 a⁴=5a²−5 :1077, Sturm-certified ISO_L10 :1090, exact zero/mul/sign/norm via the const-deg engine, anchor battery incl. the bit-for-bit rebuild of the 20 shipped girih ROTM at /64) — landed 2026-11-21 as M139, four days BEFORE the M174 STOP that framed the kernel as missing. It is consumed at the medallion interface: Field::L10 (mod.rs), the M161 catalog record runs on crate::fields::L10 with the Kq rational wrapper + isolating-interval sign engine, cos² = (5−a²)/4 pinned exactly; docs/cyclotomic-field-layer.md is the shipped deg-3/4 design doc (L10 rule bit-verified), and medallion-interface-design.md states verbatim "L10 (degree 4, shipped) covers the girih decagon". Label resolution: girih-14 (M159) lives over K₇, degree 3; Darb-i Imam (M161) is 10-fold decagonal over L10 — PROPOSED's "the 14-fold field Q(2cos π/10)" conflated the two numerals; the field statement itself was correct. Real sprint-2 gap: infer.rs's input contract is hardwired to the landed M158 record tables + the 12-fold alphabet [TRI, SQ, RHO30, RHO150] over raw ℤ[√3]; what's needed is engine generalization onto the shipped L10//64 girih coordinates + a girih-alphabet candidate generator + an M161 substrate fixture, then the 611-tile (270 bobbin + 220 bow-tie + 121 decagon) cell-table reproduction from the general path. Kernel-adjacent residuals flagged: no L10 check_sign_order parity battery (K₇/K₉/K₁₅ have one) and Kq::mul's unreduced denominator growth (→ adopt the /64-closed convention). Status flips: PROPOSED 24b sprint-1 entry + ROADMAP 24b. Delivered by the strong tier as a STOP; re-issued as the sprint-2 engine-generalization job. Cert chain: .scratch/m161-fields-doc/STOP-REPORT.md.
- 2026-11-26 — **M177: 24e underdetermined-θ search — deterministic (θ, contact) search over documented freedom sets with ranked, byte-deterministic leaderboard emission (no census movement).** The search half of the 24h objective layer (PROPOSED 1995-1999): consume the freedom the landed engines already document, score every point against an explicit aesthetic objective, rank, emit top-k. Three independent axes (`theta_search.rs`, +1369): contact (24b-fill-freedom check ids square/truncated-square × `sweep::delta_grid` 18 rows = 36; refusals kept as honest rank-0 rows with reason strings); two-point (hankin-square/hex-two-point × θₑ 30–60° × δ_frac 0.0–0.5 = 84); motif-θ (4-8-8 5² + 3-12-12 5³ = 150); default check run 270 rows. Objective (lower better): band balance B = 1 − population CV of face widths (w = 2|area|/perimeter) at 100× weight, crossing density D = validity_stats(walls, region).joint/400 tie-breaks perfectly-balanced rows; hankin axes score B alone. Ranking scored-ascending with (B, crossings, δ) tie-breaks; emission `theta-search-leaderboard/v1` byte-deterministic (BTreeMap/BTreeSet orders, no RNG/time/hidden state; two-run byte-identity pinned). Validation: an independent Python mirror of the strapwork pipeline reproduces ALL 36 contact-row Joint counts exactly (square and ts full δ tables) + closed forms (sq-90 400 = 20×20 centers, D=1; sq-45 840 = 21×20+20×21 midpoints, D=2.1, faces 685 = 361+324; ts-90 145 = 64 octagon + 81 diamond centers, faces 224 tetrakis triangles of width exactly 1/2, multiset-equal to the registered tetrakis_square; ts-45 912 = 512+144+256, faces 849 = 512 tri + 224 pent + 49 sq + 64 oct, B = 0.2767532525240787 at 2.5e-16). Two documented discoveries: face census counts faces fully inside region.padding(-1e-9) — the square's integer lattice excludes 76 exact-boundary ring diamonds (685, not 761) while ts (irrational pitch) has none; and the hand-model "δ=90 tops both substrates" is FALSIFIED for the square — at δ=45 two rows tie at B=1 and the density tie-break ranks 840 (D=2.1) over 400 (D=1) — reported honestly, never tuned (ts δ=90 does top via the balance decade; 3-12-12 classic B=−0.593 is legitimate under the documented objective). Scope: theta_search.rs new; sweep.rs visibility-only (5 items → pub(crate): params_at_default, bbox_area, delta_grid, panic_payload_message, classic_motifs_3_12_12); lib.rs one module line. Bonner-corrections validation leg stays blocked (24g book-gated). Verification: cargo check --workspace --all-targets 0 warnings; full workspace suite run TWICE on the delivery tree (1439 = 1431+8; lib 1298→1306); on the folded tree tess-tilings 1307→1315, total 1460→1468. Census: no movement (built-ins 165, memberships 186). Delivered by the frontier tier; battery + PROGRESS.md banked at .scratch/theta-search-24e/.
- 2026-11-26 — **M178 (design): degree-5 cyclotomic field layer — K₁₁ = Q(2cos(2π/11)) with the N₁₁ sine extension designed and spike-validated (docs-only; no census movement).** The design doc unblocking 24c n=11 and the M171 degree-5 stall: docs/cyclotomic-field-degree5.md (+486), delivered by the frontier tier from the degree5-fields-design brief. Certified constants (exact int/Fraction spike, stdlib-only, 48/48 checks green; banked .scratch/degree5-fields-design/): m₁₁ = y⁵+y⁴−4y³−3y²+3y+1 derived two independent ways (exact Laurent identity x⁵·m(x+x⁻¹) = Φ₁₁; Newton power sums p₁..p₅ = −1, 9, −4, 25, −16), irreducibility certified (Eisenstein-at-11 on Φ₁₁(x+1) + mod-2 + separability); conjugates 2cos(2πk/11) k=1..5 summing −1; disc = 11⁴ ⇒ Z[α] is the maximal order (denominator-free D=1 core; degree 5 prime ⇒ no subfields — the K₁₅⊃Q(√5) complication absent); RED11 multiplication tower (b⁵..b⁸ low→high) + exact tower-wrap certificates C_k = C₁₁₋ₖ, C₁₁ = 2; ISO11 = [112911137, 112911138]/2²⁶ strictly increasing on the bracket (exact integer-sign bracket, Sturm-isolated; corrects the hand estimate 112921137 from the earlier plan); N₁₁ = K₁₁(w), w = 2sin(π/11), w² = 2−α, N(2−α) = m₁₁(2) = 11 prime ⇒ w ∉ K₁₁ (genuine quadratic extension, [Q(ζ₄₄)⁺:K₁₁] = 2); battery 55 elements (18 raw /64 + 14 pins + 23 strap/exit shapes): exact sign == float sign, all distinct (min gap 0.0559), exact total order == float order, 0 bisections across all 1485 pairwise comparator differences, Horner peaks 112/113 bits at s=26 (i64-impossible, i128-safe, 3 certified bisection rounds); N₁₁ geometry at /64: hull walk closes exactly, all 11 edges unit as exact N₁₁ elements, companion rotation about the exact centroid, worked pin example P = M₃ − R_{π/11}M₇, d-rule 2(i−j)−1 ≡ 2. n=13 probe pre-derived (m₁₃ = y⁶+y⁵−5y⁴−4y³+6y²+3y−1, disc = 13⁵, ISO13 = [118843896, 118843897]/2²⁶; D=6 envelope flag: sa = 5·26 exceeds i128 ⇒ s₀ drop or gcd-normalize needed — feeds the odd-n extension design job). FINDING flagged for a separate code job: shipped n9::Nq::sgn's mixed-sign arm (n9.rs:156–171) returns sgn(u) unconditionally (then_with returns self unless Equal), contradicting its own doc-comment product rule exactly on mixed-sign elements with u² < s·v² (worked counterexample u=1, v=−2 ⇒ 1−2w₉ ≈ −0.368 < 0 but the arm returns Greater); latent — the n9 battery never exercises that subset. Integration: INDEX + ROADMAP + PROPOSED pointer lines applied at pick; M178 assigned by landing order (A3 shifts to M179). Verification: cargo check --workspace --all-targets 0 warnings; census no movement (built-ins 165, memberships 186); tests 1475 = 78+37+1318+42 (docs-only, zero code movement). Delivered by the frontier tier; spike + log banked at .scratch/degree5-fields-design/.
- 2026-11-26 — **M179 (design): odd-n cyclotomic field layer extension — n = 13 (K₁₃ degree-6 class) + the general prime-n shape, spike-validated (docs-only; no census movement).** Successor to M178, delivered by the frontier tier from the odd-n-fields-design brief: docs/cyclotomic-field-odd-n.md (+843; replaces M178 §7's n=13 stub and CORRECTS its subfield claim — errata line added there at integration). Certified constants (exact int/Fraction spike, stdlib-only, 93/93 checks green, rc = 0; banked .scratch/odd-n-fields-design/): m₁₃ = y⁶+y⁵−5y⁴−4y³+6y²+3y−1 derived two independent ways (exact Laurent identity x⁶·m₁₃(x+x⁻¹) = Φ₁₃ + Newton power sums p₁..p₆ = −1,11,−4,31,−16,98), irreducibility via Rabin witnesses 5/3/7-class tests; disc = 13⁵ = 371293 (Bareiss) ⇒ Z[α] maximal — the prime-n lemma disc = p^((p−3)/2) re-verified at n = 11 AND 13 plus p ∈ {3,5,7,17,19,23}. SUBFIELD CORRECTION (M178 §7's "degree 6 prime ⇒ still no subfields" is false): Gal(K₁₃) ≅ Z/6 — unique quadratic Q(√13) (η = [5,−4,−8,2,2,0], η² = 13 exact) and unique cubic F₃ (h₃ = y³+y²−4y+1, disc 13²); K₁₃ = F₃·Q(√13), disc chain 13⁴·13 = 13⁵ consistent. RED13 rows b⁶..b¹⁰ (fold amplification 20/25/111/152/491) + tower wrap C_k = C₁₃₋ₖ, C₁₃ = 2. D = 6 i128-envelope RESOLVED with evidence: s = 26 is structurally impossible (the primitive α⁵ monomial alone overflows at 135 bits; gcd cannot help); resolution = the pre-registered s₀-drop to s = 20 (R1 battery single-pass decisiveness, R2 the m₁₃-side poly_sign_at mid-eval wall through BISECT_CAP = 4 — the binding gate, element-height-independent; R3 largest qualifying s): ISO13 = [1856935, 1856936]/2²⁰ Sturm-isolated, m₁₃′ strictly increasing; measured: battery acc peaks 113/127 bits, mid-eval walls 110/114/110/125 (all four bisection rounds certified — one more than K₁₁'s three), L1 ceiling 7.706×10⁶ vs battery max 20480 (377× margin), μ min 2332 / median 2.861×10⁵; gcd-normalize adopted SECONDARY in the N₁₃ mixed-sign arm only (median 12 / max 14 bits saved — cannot touch the monomial wall); i256 rejected (2× kernel cost for a wall the drop removes). Key methodological finding: the naive all-elements round-gate and the naive width-form "21.1-bit precision wall" are both wrong (per-pair μ ≥ 99.9 measured). N₁₃ sine extension: w = 2sin(π/13), w² = 2−α, N(2−α) = m₁₃(2) = 13 — genuine quadratic; the spike PROVES the general degree theorem w_n ∉ K_n for every odd n (2−α = −ζ⁻¹(1−ζ)² order argument; norm certificate = p at prime/prime-power n; Rabin witnesses 5/3/7 at n = 9/11/13; structural finding: n = 15/21 have NO witness at any prime — Gal(Q(w_n)) = (Z/4n)*/±1 exponent < φ(n) — irreducible over Q yet reducible mod every prime; the degree theorem there rests on the order argument). /64 hull geometry mirrors K₁₁ (closure, 13 unit edges exact with worst canonical coefficient 1, centroid-then-rotate companion rotation, pin P = M₃ − R_{π/13}M₇, d-rule → {13/4}); star-walk shoelace factors {13/1..13/6} exact. BATTERY_K13: 59 elements (62 constructed, 3 exact duplicates dropped — the battery is a set), 1711 comparator pairs, min gap 0.01199, heights to 4096, 0-bisection single-pass everywhere; N₁₃ battery 67 elements with 4 mandated wrong-shortcut mixed-sign regressions ((1,−3),(2,−5),(3,−7),(−1,3) — the n9 sgn lesson; (1,±2) become boundary controls since 1−2w > 0 at n = 13) and 4 exact-zero elements (zero-awareness). General prime-n recipe uniform in n; applicability now 7/9/11/13/15/17/21 at design level; composite discs MEASURED (n=9: 81 = 9², n=15: 1125 = 5·15², n=21: 453789 = 3³·7⁵ — the pre-spike guesses 225/1029 were wrong; conductor-discriminant formula verified); s_max(D) = 58/38/28/22/18/15 for D = 3..8 — D = 7 never arises (φ(n) = 14 unsolvable, brute-forced n ≤ 2000), next real wall D = 8 (K₁₇). Named n = 13 consumers: the 24c monster rung (the entry's "n ≤ 12" scope line flagged for extension in PROPOSED), 11/13-pointed-star hankin instances (PROPOSED M99 remainder), Harriss #18 n-fold @13; Say-awen 21-fold shares the D = 6 depth class. Consumer contract pre-specified: K13 fields.rs block (mirrors L10), Field::K13 additive tag, monster/n13.rs N13 with FieldEl, /64 convention per the M161 Kq catalog pattern, contact_cos2 = None. Integration: INDEX + ROADMAP + PROPOSED pointer lines applied at pick; M179 assigned by landing order (A3 shifts to M180). Verification: cargo check --workspace --all-targets 0 warnings; census no movement (built-ins 165, memberships 186); docs-only — zero code movement (tests 1498 = 78+40+1326+54 at the pick, per-crate counts live-verified). Delivered by the frontier tier; spike + log banked at .scratch/odd-n-fields-design/.
- 2026-11-26 — **M180: girih inference engine (24b sprint-2) — Plain-level generalization of the WIS substitution engine onto the girih family, S2/S3 exact (medallion layer only; no census movement).** Delivered by the strong tier (base 46b1658, cherry-picked onto c2bca4b): new `medallion/girih_infer.rs` (1245 lines — frontier/candidate machinery, self-contained exact-sign descent sgnL10 for K10 heights, conservative CELL=2.0 spatial cell-grid prefilters over hole edges/vertices and placed tiles, backtracker recursion) + `medallion/hostgirih.rs` (#![cfg(test)], 719 lines, 5-test battery) + mod.rs +2. Zero-touch guarantee: infer.rs/hostwis.rs/fields/families/registry untouched — WIS behavior byte-identical by construction, all existing WIS tests green unmodified. The 0/1810 finding (restated per ruling): 0 of 1810 landed interior edges lie on any carried exit-ray/{10/3}-chord line of the M161 record ⇒ C2 anatomy has no girih counterpart (the girih substitution absorbs its strapwork); the girih engine runs the Plain-equivalent stack C1+C3+C4 and the WIS anatomy levels (RayLine/Vset/Segment) stay WIS-substrate-only. Battery (live, 5 passed): sgnl10_matches_float (the Python prototype's exact-sign descent dropped the 5× factor on the √5 cross term — found and fixed pre-Rust, exhaustively checked over 330,321 elements: old wrong on 6,347, corrected wrong on 0); girih_decagon_hole_exhaustive_census (exhausted, 11 solutions, 11 distinct fingerprints, census {[0,0,1]:1, [3,1,0]:10}, two runs byte-identical); m161_rebuild_exact_identities (611 tiles, census 270/220/121, 4150 directed/2340 undirected unit edges, single 530-edge CCW hole, exact area identity boundary-shoelace = tile-sum = (0,−93440,0,59200)/64, Euler disk χ=2 over V=1730, corner cos² classes {(−16,0,16,0):2730, (64,0,−16,0):1420}, candidates = 7 on every hole edge); m161_guided_walk_general_path (611/611 steps through the engine's own frontier/candidate/C1–C4 machinery, zero patch data inside the engine); m161_freedom_set_capped_deterministic (cap hit, nodes == max+1, two runs byte-identical incl. seq_hash; the capped 530-edge run executes on a 512 MB scoped-thread stack — results stack-independent). S3 comparison: the shipped Darb-i Imam 611-tile cell table is reached as a member of the engine's freedom set with ZERO divergences — ground-truth bar (M158 + M161) now met in full. En-route fix: GirihTile::new seeded bbox as [INFINITY; 4] so bbox_disjoint never fired (pairwise phase consulted every placed tile; the prefilter was inert, never unsound — pinned assertions unchanged across the fix; girih suite 476 s → 40 s with the cell grids). Verification: full cargo test --workspace green at the pick base (1458 live incl. the +5 battery); CARGO_INCREMENTAL=0 cargo check --workspace --all-targets zero warnings; census no movement (built-ins 165, memberships 186); integrated onto c2bca4b with battery 5 green + census-pin suite green, totals 1517 = 78+40+1331+68 (live-verified at integration).
- 2026-11-26 — **M181 (design): 24c odd-n engine-side design — the (n,c) monster rungs at n = 11, 13 over the designed field layers, spike-validated (docs-only; no census movement).** Third in the cyclotomic design arc (M178 → M179 → this), delivered by the frontier tier: docs/medallion-24c-odd-n-design.md (+433). Per-record engine-side designs for SEVEN rotated-family rungs — (11,2), (11,3), (11,4), (13,2), (13,3), (13,4), (13,5) — deviation D1: the Kaplan band (p. 123) admits the Δ=1 rungs (11,4)/(13,5) beyond the brief's five, and the census settles them clean (also fixes (13,2)'s contact figure: raw d=7, canonical fold 6). All constants machine-settled exactly (stdlib Fraction spike, 226 checks / 0 failures, banked .scratch/24c-oddn-engine-design/): both crossings at t = s = 1/2 for every record, split slots, cos² tuples FULL-degree over K_n for all seven ⇒ contact_cos2 = None everywhere (pure-K records; the N-layer two-level rule stays mandated for the field batteries, not the geometry), |P|² tuples, exact windows (event counts 123/121/123 at n=11, 175/175/173/173 at n=13), t/s linearity with t₀∈{0,1} Δ-specific (V₂ lies on the π/n ray at every n; Δ≥3 gets genuine K t₀), window-pin verdict splits on Δ — Δ≥2 pins at the exact window midpoint, Δ=1 does not, matching the shipped (9,2)/(9,3) pair; the Δ=1 rung shares its σ_hi with the same-n Δ=2 rung. T1 parity theorem extended to prime n, proof COMPLETE (parallel pin-map injectivity on all n(n−1) ordered pairs, zero axis-parallel midpoint pins, two-distinct-angles census over 54/88/130 clean parallel pins) PLUS correction D2: the ROTATED pin map is 2-to-1 — P(i,j) = P(j+h+1, i+h) is the κ A/B-swap relabeling of one configuration (distinct pins (n²+n)/2 = 45/66/91); records canonicalize at the (Δ,0) labeling; the n=9 census5 orbit re-confirmed (O_A-rotation by 2πs/9 maps config(2,0) onto config(2+s,s) as vertex sets). D4: outside-convex-polygon classification is mixed-signs = outside (the all-negative reading is wrong — caught by the n9 oracle before any 11/13 claim). Field-consumption map: Field::K11/K13 tags, Kq over ISO11@s=26 / ISO13@s₀=20 exactly as contracted, /64 convention, M163 cross-check θ = d_canon·180/n; engine gaps mapped (n9::Nq width-3 hardcoding → genericize over D rather than triplicate; ISO-bracket plumbing per field); headroom measured — max bit height 22 < 2⁵⁰ (M164 bound), kmul peak 25 bits < 127, 0 bisections across 1.24M ksgn calls on the full census+record workload. 14-mandate verification-gate spec per record (doc §8), incl. field-level wrong-shortcut mixed-sign regressions (the n9 sgn lesson, mandated so the implementation cannot skip them) and a new n²-enumeration census regression as oracle pin. Integration: 24c scope line extended n ≤ 12 → n ≤ 13; PROPOSED closing note superseded (the "n = 11 blocked"/"13-rung" remarks retired); field docs' §5.3 consumer bullets added; ROADMAP status flip. Implementation job (7 catalog rows + N-layer genericization + batteries) queued: effort M, tier strong, after the K₁₁/K₁₃ fields land. Verification: cargo check --workspace --all-targets 0 warnings; census no movement (built-ins 165, memberships 186); docs-only — zero code movement. Delivered by the frontier tier; spike + green log + record constants banked at .scratch/24c-oddn-engine-design/.
- 2026-11-26 — **M182: odd-n prime cyclotomic K-layers land — `Field::K11` + `Field::K13` additive fields.rs blocks + sign-engine envelope erratum + peak-bit wrap guard (no census movement).** K₁₁ per M178, K₁₃ per M179: certified minpolys (exact Laurent identity, in-test F₂ Rabin irreducibility, disc 11⁴/13⁵ via knorm Sylvester), RED towers b⁵..b⁸ / b⁶..b¹⁰ with exact wrap certificates (N(b) = −1 both), ISO brackets ISO11 = [112911137, 112911138]/2²⁶ and ISO13 = [1856935, 1856936]/2²⁰ (M179 R-rule s₀), batteries BATTERY_K11 55 / BATTERY_K13 59 elements (3 exact dups dropped: (−17,9,0,0,0,0)×2, (−7,4,0,0,0,0)) — single-pass decisive with **0 bisections** across all 1485/1711 comparator pairs (interval-Horner peaks 113 bits), exact sign/order == float; medallion `Field` gains K11/K13 tags (declaration-only, M171-K9 shape). **Envelope erratum (M178 §2.2/§8-Q2, M179 §2.2, applied):** the binding i128 wall of `poly_sign_at` near a root is the Horner **intermediate** — step `A₁·num = 2^{deg·s′}·(m(y)−m₀)` ≈ |m₀|·2^{deg·s′}, wall **deg·s′+1** — not the final-value bits the design docs tabulated (K₁₁ walls 112/115/118/124, K₁₃ 110/114/110/125 reproduce exactly as final bits, asserted by `k11_k13_sign_engine_envelope`). Exact audit: ISO11@26 endpoints need 131 bits, CAP=4 rounds 136/141/146/151; ISO13@20 endpoints 121, rounds 127/133/139/145 (only round 1 i128-executable); shipped fields all fit (K₇/K₉ 94, K₁₅/L₁₀ 105/107). **Landing = option C:** constants stand (M178/M181 citations intact); bracket endpoint certification runs through an exact test-only 192-bit Horner (`poly_scan_192` in `check_iso_bracket`) with i128 parity wherever the envelope fits — every field keeps engine evidence; full-contract grids (deg 5 → s = 21, deg 6 → s = 16) documented, not shipped. **Hardening:** `poly_sign_at` Horner steps are now checked ops — past-envelope evaluation panics loudly in every profile (silent release wrap retired); non-firing live-proven on all certified workloads (brackets, batteries, full 1344-test tilings suite incl. census/monster paths). Integration: cherry-picked onto c4e30fa (M181), hunks disjoint, merged clean; M-number by landing order; STOP-gate (d) process on record (.scratch/oddn-fields-impl/GATE_REPORT.md — live A/B/C option audit before the pick). Next queued: the 24c rung implementation (7 catalog rows + N-layer genericization over D) — now unblocked. Verification: cargo check --workspace --all-targets 0 warnings; cargo test --workspace 1530 passed / 0 failed / 0 ignored (core 78, gui 40, tilings 1344, tui 68 — base 1517 + 13); census no movement (built-ins 165, memberships 186). Fresh verify_impl + envelope audits banked .scratch/oddn-fields-impl/. Delivered by the strong tier.
- 2026-11-26 — **M183 (design): §23i new-field tiers — Say-awen 17/21-fold field design; the D = 8 wall verdict (NON-QUALIFICATION) + K₂₁ verbatim transfer + the p = 43 square-class extension certificate (docs-only; no census movement).** Fourth in the cyclotomic design arc (M178 → M179 → M181 → this), delivered by the frontier tier: docs/cyclotomic-field-17-21-design.md (+587). **K₁₇ (degree 8):** constants fully certified (disc 17⁷ via the prime-n lemma — 17⁷ = 410,338,673 verified; mod-3 Rabin irreducibility; norm 17 prime — the M179 certificate transfers) but the D = 8 envelope stress test returns **NON-QUALIFICATION**: measured True-flag walls s = 11→111, 12→111, 13→126, 14→133 (R2 kill), 15→140 (R2 kill); R1 undecided pairs 4/2/2/1/1 kill {11,12,13}; structural worst-pair mu = 0.2559 < 1 at s = 13 (representation-robust: both c₈ = 0); α⁸ monomial 128 bits at the s = 15 ceiling — the partition over [11, 15] is complete: **no s qualifies**. This is the ladder's structural limit under the 127-bit i128 engine (the corrected deg·s′+1 intermediate law from M182 applies); Say-awen 17-fold is blocked on an engine decision (accumulator width vs non-consecutive isolation scheme vs reduced battery envelope) — the spike is the reusable stress harness for any such change. **K₂₁ (degree 6, composite 3·7):** transfers K₁₃'s depth class verbatim — s₀ = 21 (R2 wall exactly at the 127-bit boundary), battery 71/2485 single-pass green, acc peaks 118/118, disc 453789 verified. **Extension certificate:** the M179 prime-n norm route is VACUOUS at n = 21 (m₂₁(2) = Φ₂₁(1) = 1, a square; Φ₂₁(2) = 2359 is the full-field norm) and the Rabin route on m₂₁(2−y²) is structurally impossible (Gal(Q(ζ₈₄)⁺) exponent 6 < 12; DDF census exactly the pre-registered {5,11,17,19:{6:12}, 13,43:{2:12}}; inert-prime F_{p⁶} route vacuous) — replaced by a **p = 43 square-class witness**: roots [11,12,13,23,33,38], (2−α)²¹ ≡ −1 per-root and in F₄₃[x]/(m₂₁) ⟹ s non-square ⟹ [K₂₁(w₂₁):K₂₁] = 2. **Corrected §23i applicability table** (supersedes M179 §7.3's K₁₇/K₂₁ rows + fixes the stale "degree-3/4" claim): Tier 1 field shipped (Danzer 7-fold, Goodman-Strauss 7-fold rhomb, Nischke–Danzer deltoids — K₇; Harriss 9-fold — K₉+Nq; CAST 10-fold — L₁₀); Tier 2 shipped/designed (K₁₁/K₁₃ shipped M182; Say-awen 21-fold implementation-ready at the field layer); Tier 3 engine-blocked (Say-awen 17-fold). Corrections ledger (doc §8): sine-monotonicity INCREASING both fields; edge constant [4096]+[0]* = 64² numerator units; w₂₁ = 2cos(19π/42); α₂₁ ≈ 1.91115; H n=10 cosine-fold bug; two latent spike bugs fixed en route (pquo in DDF; %10-vs-abs fold). Spike: stdlib Fraction, 97/97 checks green rc = 0 (banked .scratch/sayawen-17-21-design/ with run1-3 baselines). Integration: PROPOSED §23i replaced verbatim per doc §6.2 (with the post-M182 shipped-status corrections the frontier's draft could not know); M179 §7.3 table updated (K₁₁/K₁₃ → shipped M182; K₁₇ → engine-blocked; K₂₁ → implementation-ready); ROADMAP status lines + M183 bullet; INDEX bullet. Open questions (doc §7): the D = 8 engine decision (the only 17-fold blocker); Say-awen 21 substitution-table work (the XL remainder); the K₂₁ tie-arm unreachability assert must cite the square-class certificate, not a norm value. Verification: cargo check --workspace --all-targets 0 warnings; census no movement (built-ins 165, memberships 186); docs-only — zero code movement. Delivered by the frontier tier; spike + green log + baselines banked at .scratch/sayawen-17-21-design/.
- 2026-11-26 — **M184: per-tiling `depth` parameter on glb-binary-n7 — retires the e01fc76 scale override, closes the TUI-thumb depth-4 gap (no census movement).** Fast-tier production landing (first pass post-recovery — the borderline-sizing watch verdict: NO rework, the class stays fast). glb_binary_n7.rs: `depth` Int param APPENDED at the END of the spec list (bounds 0..=5, default 0, step 1 — fingerprint-order safe); depth 0 → the budget rule byte-identical (D_MIN 3 / D_MAX 5 / VIEW_BUDGET 2500); 1..=5 → pinned exactly; out-of-range → named refusal (never a silent snap); +3 tests. tess-gui/ui.rs: GLB_BINARY_N7_PREVIEW_SCALE (e01fc76's scale=1.0 card override) retired → GLB_BINARY_N7_PREVIEW_DEPTH = 3 via the param; probe/window derivation unchanged. tess-tui/thumb.rs: the TUI-thumb depth-4 gap closed via the same param semantics; cache key from effective params. Byte-identity triple (worker-verified live, release): glb export default byte-identical (13046 tiles, 2249892 B; depth=3 pinned == default byte-identical — the budget rule already lands on 3), Penrose control byte-identical (2936), explore rows byte-identical (17). Depth ladder: 2 → 3816 tiles (0.198 s); 4 → 46423 (37.7 s); 5 → 169116 (2m9.7 s); 6/−1/x → refusals exit 1. Intentional card change (documented): GUI card 285 ms / 289 tiles at world scale 2.75 (was 134 ms / 2028 at scale 1.0); TUI thumb 275 ms / 274 (was 1.375 s / 915). Integration fix (orchestrator, anticipated by both briefs): the e5dc356 param-surface lock test simulated appending a hypothetical `depth` spec — the real landed spec collides by design; premise updated (real tail `,depth=0` pinned appended-not-spliced; generic append-detectability preserved via `future_param`). Verification: 0 warnings; suite on the tip 1548 = 84+40+1356+68, 0 failed; census no movement (165/186). Evidence: .scratch/depthparam-api-recon/ (REPORT + IMPL).
- 2026-11-26 — **M185 (docs-only): stale parameter-surface claims fixed — README third-param enumeration (9 tilings) + the nonexistent Choice-kind clause (no census movement).** The two stale claims the e5dc356 pins discovered and reported-not-fixed, now fixed by the same tier: README's "three families add a third" enumeration (named 3; actually 9 — star-and-cross, hexagram, Pythagorean, penrose_p2 + the isohedral family ih1/6/7/27/29 with shaped offsets declared before scale/rotation) rewritten accurately with the pinned scale/rotation paragraph byte-unchanged and the three named third-param anchors kept single-match; the lib.rs test comment asserting a Choice variant with a variant list replaced by the honest statement (ParamKind = Float/Int/Bool/Angle only; DESIGN.md already accurate, untouched). Verification: 0 warnings; census pins 76/76 live; 9/9 param-surface pins; docs-only. The pin-then-fix loop working as designed.
- 2026-11-26 — **M186: Ammann A3 non-edge-to-edge substitution lands — `ammann-a3` registry entry from the banked exact Z[φ] derivation (Quasiperiodic 17 → 18, built-ins 165 → 166, memberships 186 → 187).** Third rung of the owner's A1–A4 arc (A2 = golden bee M104, A4 = M173; A1 parked, no substitution structure). Delivered by the worker tier on recovery dispatch 1, zero STOPs: ammann_a3.rs (1610 lines) implements the three rectilinear Z-chairs (W1 = (15,23), H1 = (8,15) in Z[φ]; areas 402+641φ / 641+1043φ (= φ·A1) / 478+804φ) with 12 oriented states, substitution rows certified at the T-junction census pins 9/4 · 13/5 · 9/4 (levels 1–3 full-edge vs T-locations; level-2 = 6/9/7 tiles, 9/13/9 full-edge, 4/5/4 T, 0 split), count matrix N = [[1,1,0],[3,0,1],[2,0,1]] with N·A = φ²·A verified, non-edge-to-edge emission with T-junctions documented never hidden (the A4/M173 family precedent), single P2 body-centred seed, inradius (1+7φ)/2, declared_symmetry None. Derivation-vs-live: derive_a3.py + dump_a3_tables.py re-run green — 12/12 SHAPE + 12/12 RULE byte-match vs the banked certificate; the three en-route fixes were test-code only (Z[φ] conjugate N(3−2φ) = −1; count_rule column order; odd-parent frame swap) — production geometry/tables unchanged vs certificate. Wiring: family file only (register, classification arm, IDS, family census 17→18; nonperiodic — no lattice-culling line); census pins across lib.rs (per-tab vec 22→23, 165→166, 186→187, INDEX files anchor, metadata-token census), tess-tui + tess-gui registry-size pins (orchestrator-only crates per AGENTS.md — flipped under the M173 precedent, flagged); docs: README/INDEX/TAXONOMY/DESIGN/aperiodic/quasiperiodic + the per-tiling doc ammann-a3.md; the stale README/DESIGN "currently N tests" line set live (re-set to the true total at this amend — the count moved under it twice). Integration: cherry-pick conflict on the census-comment line resolved (M182 M-list entry preserved + A3's numbers + the M186 M-list entry); PROPOSED §23b A3 status flip; ROADMAP Ammann-sets line; CHANGELOG M184/M185/M186 entries appended (M184/M185 had been integrated without entries — repaired here). Verification: 0 warnings; battery 11/11 + census pins green in-tree and re-verified at the pick (tilings 1367 = 1356 + 11, tui 68, gui 40, core 84 live); insurance suite at the pick (hold until green). Delivered by the worker tier; spec + scripts + IMPL.md banked at .scratch/ammann-a3/.
- 2026-11-26 — **M187: 24c odd-n monster rungs implemented — rotodd.rs generic engine + seven (n,c) catalog rows at n = 11, 13 over the landed K11/K13 fields (no registry movement).** Delivered by the strong tier (base e5dc356, WIP milestones squashed at integration; live spike re-verified every banked M181 constant before the engine landed — all 7 pins x/y/|P|², all 7 cos² = (2+C_d(b))/4 exact, all 14 window endpoints exact, event counts 123/121/123 (n=11) and 175/175/173/173 (n=13), persistent = [], no STOP gate triggered). **N-layer genericization:** medallion/monster/rotodd.rs introduces the `Core` trait + two-level `Nq<F>` coordinates over K9/K11/K13 (rotations, hull_odd/pin_rot/cheb/conj, RotSpec + verify_pair_rot, RotGeom σ-census); n9.rs slimmed to the K9 instance with the full suite (1353 at the surgery commit, before n11/n13 existed) holding byte-identity; hub catalog extended to 15 rows. En-route engine bug found+fixed: RotGeom::new rv = rot_half(a[q]) (rotated vertex, per the spike's RV), was wrongly the rotated direction. **Batteries:** n11.rs 16 + n13.rs 7 tests — all 14 §8 mandates × 7 records ((11,2..4), (13,2..5)): pin exactness, crossings at t = s = 1/2, eAB/eBA, the d-rule with the (13,2) 7→6 fold, full-degree cos² orbits ⇒ contact_cos2 = None ×7, exact-K window census (midpoint verdicts, shared σ_hi at the Δ=1 rungs, below/above states), outline formulas + simplicity, κ maps, dirty = 0, purity mixed() == 0, M163 θ-checks ×15, n² 2-to-1 pin census (66/91 distinct pins, collinear fixed locus). K11 product-rule regression: 1−2w < 0 with flipped set exactly {(1,−2),(1,−3),(−1,2),(−1,3)}; K13: 4 mandated wrong-shortcut elements + (1,±2) boundary controls + sweep burden 24. §12 resolutions: N-layer = trait-over-core-field; §8.14 census ships as a release test; catalog ρ = σ = 1; D4 = mixed-signs-outside. fields.rs: all 14 K11/K13 #[allow(dead_code)] dropped (fully consumed); medallion/mod.rs Field::K11/K13 de-staled; monster module docs (family wiring n ∈ {8..13}, census-pin ladder + partner map, window table, {11/1}/{13/1} precedent). lib.rs untouched by the tier — M-list entry + this entry added at integration. Verification: 0 warnings; tess-tilings 1381 (1367 + 14), census filter 81 (77 + 4), tess-core 84 live on the squashed tree; squashed-tree-vs-delivered-tree diff empty on medallion/fields paths; insurance suite at integration (hold until green). Census: built-ins 166, memberships 187 — no registry movement (rungs are medallion-layer records, not registry tilings).
- 2026-11-26 — **M188: medallion third-layer composition design — absorbing the medallion layer into the strapwork weave (design doc, no code).** Delivered by the strong tier (Job 2 of the 24c dispatch, `docs/medallion-third-layer-design.md`, 411 lines). The design answers "how do the intermediate/interstitial layers, straps, and boundaries compose": the medallion figure (chords + marched exit rays of placed records) is absorbed at the wall stage into the same `Vec<Wall>` as the host strapwork *before* `build_map` — one Eulerian/two-strand map, so all four Kaplan decoration styles decide honesty on the fused map, not on separate layers. Suppression without clipping: PIC rays never cross tile interiors ⟹ suppression is by seed, no geometric clipper. Continuity law: at a contact, `θ_record = d·180/n = δ` — free via the M163 identity when δ matches, a named gate error otherwise (honest failure, never silent). Boundary surface: two default-empty `Tiling` hooks (`extra_walls`, `suppressed_seeds`), `walls_layered`, two `Medallion` f64 views, one error variant; `tess-core` names no medallion concept. Certification anchors: WIS/3.12.12 interstitial closure, khatam {8/2}@45° segment-multiset identity, M172 honeycomb δ=60 pass/δ=30 named fail; the M164 wall-stage reading cited. Nested medallions + field tower held as §10 backlog variations. Integration: PROPOSED §24 header pointer, ROADMAP 24b line, INDEX design-doc bullet added. No code, no registry movement, no test-count change.
- 2026-11-26 — **M189: living-docs drift audit + permanent de-staling of the hardcoded test counts (docs only, fast tier).** Delivered by the fast tier (audit script + findings banked at .scratch/docs-drift-audit/). Five files fixed: README.md:195 and docs/DESIGN.md:135 drop the hardcoded "currently 1559 tests" clause (staled 3× in one day: 1528→1559→1574 — the number is gone, so it can never stale again; "all green, zero warnings" retained); PATTERNS.MD:10 re-pinned 163→166 implemented tilings; AGENTS.md:12 dropped the stale "(848 tests)"; docs/families/aperiodic.md M168 status line now ends "166 tilings, 187 memberships, all current" (166/187 remain guard-pinned by census_surface_docs_match_registry; the test-count clause is explicitly unpinned). Report-only findings (orchestrator-owned, no action forced): TAXONOMY.md L3 M76 historical note (live counts at L10-11 match); docs/families/*.md M34..M146 frozen historical status lines; REVIEW.md pre-repo-era; PROPOSED/ROADMAP sprint-allocation test counts; M-token orphans — ROADMAP: M61 (parked wheel tiling, git de53cce, correctly headerless); PROPOSED: M61 (as above), M1/M2 (math glide labels, false positives), M15 (star-cross, pre-changelog, body-referenced in M36), M58 (pentagon type 7 bounded-negative, 7b409f1, deferred never recorded), M66–M71 (UI rework cbd9b23, landed+pushed, no headers), M73 (type-8 integer cell, never recorded). README (1 token) + INDEX (6 tokens) all match headers. CHANGELOG covers M18..M186 headers, 148 headers/159 lines. Bonus: per-tab specs [22,40,81,34,38,25,62,17,48,24,22] live registry-only (no doc claims them); per-tab counts in README/INDEX/TAXONOMY all match live and are guard-pinned. Verification at pick: census 81/81, param_surface 9/9, doc-sync targeted 9/9, tess-core 84, zero warnings. One self-caught hazard fixed en route: the aperiodic reword initially broke the guard's `number_before_suffix(" memberships,")` parse; reworded to "memberships, all current" and the guard passed.
- 2026-11-26 — **M190: user-review text/relabel batch — wallpaper-note cleanup, nonperiodic substitution vocabulary, pinwheel recategorized Aperiodic → Nonperiodic (fast tier, registry metadata move, built-ins 166 unchanged).** Delivered by the fast tier (commit 8b5123c on base 7dcb9f2) in response to the 2026-11-26 user app review; README/category.rs/pinwheel.md consequences applied at integration by the orchestrator (fast-tier guard had README edit-forbidden). **E1 (wallpaper dev-note):** browse.rs card strings no longer say "registered realizer" / "M85 closed the p3m1 gap" — the complete-catalog branch now reads "complete catalog: all 17 wallpaper groups represented", gaps read "not yet represented: …"; the browse.rs guard test pin reworded to match; internal M85/p3m1 comments elsewhere in the codebase intentionally untouched (not card strings). **E2 (substitution vocabulary):** the five limit-periodic substitution cards (domino, half-hex, period-doubling, Thue–Morse, period-tripling) drop the "Monotile" label (its emitter was the shared `prototiles: Some(1)` classification arm — removed in those five arms only; the four legitimate monotiles hat/turtle/spectre/Taylor–Socolar keep it) and drop `aperiodic_status` (tile-set aperiodicity is distinct from long-range order); card text now leads with "non-periodic substitution tiling" and states the tiles admit periodic tilings while the substitution forces nonperiodic order; "limit-periodic" retained strictly as the long-range-order tag. **E3 (pinwheel move):** the pinwheel's bare triangle tiles periodically — it is a nonperiodic substitution tiling, not an aperiodic tile set; declared category Aperiodic → Nonperiodic, arrangements.rs classification family list updated, card text "Non-periodic Pythagorean-triangle tiling (Conway–Radin pinwheel); homochiral; tiles in infinitely many orientations" (homochiral kept — meaningful for the scalene triangle). Census re-derived live: per-tab vec [11, 20, 33, 17, 17, 5, 30, 8, 23, **11, 12**] (Aperiodic 12→11, Nonperiodic 11→12); built-ins 166, memberships 187 unchanged (a move). **Integration additions:** README per-tab prose re-pinned (Aperiodic 11, pinwheel dropped from its member list; Nonperiodic 12, pinwheel added), category.rs doc comments de-staled (pinwheel moved to the Nonperiodic note), docs/tilings/pinwheel.md descriptor "aperiodic Pythagorean" → "non-periodic Pythagorean", M-list entry + this entry. Verification: cargo check --workspace --all-targets 0 warnings; tess-core 84/84; census filter 81/81; --lib tests:: tier green including readme_matches_registry and registry_groups_are_complete_and_canonically_ordered post-move; full insurance suite at integration (hold until green). **Insurance-suite repair:** the workspace run caught two tess-tui pins carrying the pre-move tab counts (`registry_tabs_classify_every_tiling_once` per-tab vec and the `tab_bar_shows_all_ten_categories_with_counts` Aperiodic/Nonperiodic literals) — re-pinned to [.., 11, 12] at integration; tess-tui 83/83 + tess-gui 40/40 green post-repair; full suite then GREEN 1588/0 (84+40+1381+83).
- 2026-11-27 — **M191: IH27/IH29 reclassified as isohedral variants — Pentagon Types tab drops to the 15 true types (fast tier, registry metadata move; built-ins 166 unchanged, memberships 187 → 185).** Delivered by the fast tier (commit a51bce2 on base 800d138), implementing the user review's finding that IH27/IH29 are not additional pentagon types — their edges are curved/notched deliberate deformations. Recon established the two entries were ALREADY Isohedral-primary (category.rs: "the pentagon interlocks also under Pentagon Types"); the defect was the Pentagon Types secondary cross-listing, so the fix drops the Pentagon secondary (secondary &[Laves, PentagonTypes] → &[Laves]) rather than moving the primary. **Census re-derived live from the failing guards:** per-tab vec [11, 20, 33, 17, **15**, 5, 30, 8, 23, 11, 12] (Pentagon 17→15, all others unchanged); memberships 187→**185** (extra secondaries 21→**19**). **Card changes:** names "IH27 pentagon (pgg)"→"IH27 (pgg)", "IH29 pentagon (p4g)"→"IH29 (p4g)"; both descriptions gain "Isohedral variant — deliberate edge deformation (curved/notched), not a pentagon type."; symmetry_notes gain the deliberate-deformation clause (ih27 keeps "1 U + 2 J edge orbits"; ih29 keeps the Klein-paired base edge). Classification arms moved off PentagonTypes in isohedral.rs. **Pins updated live:** lib.rs per-tab vec + memberships 185 (pin tests only, M-list untouched by the tier); tess-tui registry_tabs total 185 + counts vec, tab-bar Pentagon 15, pentagon_tab_orders_types_sequentially now expects exactly types 1..=15 (the two untyped-interlock padding entries dropped); README census sentence 185 + per-tab prose (Pentagon 15 without the interlocks clause; Isohedral prose states the two Cairo-net templates are isohedral variants); TAXONOMY.md Pentagon 15 + 185/19; docs/families/aperiodic.md M168 line 185; docs/families/isohedral.md table + prose; docs/tilings/ih27-pgg-pentagon.md + ih29-p4g-pentagon.md h1 + classification prose. **Integration additions (orchestrator):** TAXONOMY.md Aperiodic/Nonperiodic figures trued to M190's move (11/12 — stale since M190, no guard pins them), category.rs Isohedral doc comment de-staled ("the IH27/IH29 Cairo-net variants also under Laves"), M-list entry + this entry. Report-only: docs/isohedral-generator-design.md is a frozen M162-era design doc describing the old intent — historical, left untouched. Verification: 0 warnings; census 81; --lib tests:: 1328; tess-core 84; tess-tui 83; tess-gui 40 (all in the worktree); full insurance suite at integration (hold until green).
- 2026-11-27 — **M192: three mechanical coloring modes — Shape, Role, Metatile (fast tier, production; mode count 5 → 8; registry unchanged, built-ins 166, memberships 185).** Delivered by the fast tier (commit 8717837 on base 800d138), implementing the recon's mechanical batch from the 2026-11-26 user review's coloring asks. **Shape** (`shape_class(poly)` = vertex count clamped to 0..=7, triangle→…→10+-gon) colors by polygon shape class — the chord-based interpretation (bowtie = concave 6-vertex → bucket 3), documented as such. **Role** decodes the islamic shape-role signal already carried in seed (`seed % 8` into a dedicated 8-entry warm/earth ROLE_PALETTE, byte-pinned); per-family role-carrying verified (girih decagon/bowtie, Andritz seeds = proto, dodecagon honeycomb 0/1/2; single-orbit families girih_bowtie/girih_sc/girih_w85/hankin_* are Uniform by construction — documented, uniform under this mode). **Metatile** colors aperiodic cluster labels: hat cluster roles bucket to H/T/P/F (1..=4→H, 5→T, 6..=7→P, 8..=9→F); CAP reads through the proto-class fallback via a deliberate (proto, seed) pair exclusion; METATILE_PALETTE 9 entries byte-pinned. **Documented signature collisions (Tile carries no family identity):** hat/spectre/turtle-turtle share proto=0/seed 1..=9; reptiles (seed = orientation+1), pinwheel (seed 1..=5) and isohedral (seed = aspect index) share the same signature and take the same buckets; turtle hat-companion (proto=1) falls to the fallback — all structural limits of the no-Tile-change guard, in the metatile_class doc comment + report. **Plumbing:** ColorMode enum ALL 5→8 (Shape/Role/Metatile appended; existing 5-mode prefix untouched), next/parse/label/preview_mode coherent; BOTH decision sites synced (paint.rs tile_rgb + raster.rs color_index); tess-gui selector gains 3 modes with the RenderKey/cache_key UNTOUCHED — canvas_cache_recolors_without_regeneration passes unchanged (paint-time state stays out of the cache key); tess-tui palette256_role/palette256_metatile fields + selection arms; export --mode list + docs/USAGE.md row + docs/DESIGN.md color-modes section + README mode count. **Pins updated live:** color_mode_all_len_is_five → color_mode_all_len_is_eight; 8-step cycle test rewritten; USAGE export-contract pin 5→8 values; PATTERN_GRAY_PALETTE byte-pin and Gray/Proto index-space equivalence pins untouched and green; TUI/GUI byte-identity pins (card cache, thumbs, exports) all green. **Verification:** 0 warnings; tess-core 94 (84 + 10 new paint tests); tess-gui 40; tess-tui 83 (+16 thumb, +18 doc-pins); census 81; existing-mode byte-identity confirmed. Insurance suite at integration (hold until green).
- 2026-11-27 — **M193: tab display renames — Arrangements → "Bonds & dissections", Laves (Catalan duals) → "Duals & variants" (fast tier; label strings only; counts unchanged, built-ins 166, memberships 185).** Delivered by the fast tier (commit a15f29e on the M191 tip, picked cleanly onto the M192 tip), implementing the user review's naming items: "Arrangements is vague — 'Bonds & dissections' would say more" and "Catalan duals borrows a term for polyhedra — either rename it 'Duals & variants' or split it into sections". The orchestrator chose the rename slice; the 3-way Laves split (Laves 11 / 2-uniform duals 20 / IH → Isohedral) is deferred pending a composition recon and strong's A-batch freeing the family files. **Surfaces:** tess-core/src/category.rs label() (the single source of truth for both tab bars — enum variant names TilingCategory::Arrangements/::Laves UNCHANGED); tess-tui tab-bar pin labels (counts 33/8 unchanged); tess-tilings lib.rs README census-guard alias array + live comment (M-list untouched by the tier); tess-core/src/app.rs search_matches_category_label search term "laves" → "duals" (the one out-of-brief file the rename forces — flagged by the tier, accepted at integration: the test searches by display label); README eleven-tab prose; docs/TAXONOMY.md eleven-tab enumeration. **Verified non-coupling:** no export/parse format reads label() output (checked by the tier; no serialization keys off display strings); docs/INDEX.md, USAGE.md, DESIGN.md, docs/tilings/*.md, docs/2D_Tilings_Reference.md carry family-name references only — untouched. **Deferred (same review item):** within-tab ordering (stack→running→herringbone→basketweave; quasiperiodic symmetry grouping; aperiodic/nonperiodic sections) and the Laves tab 3-way split — the ordering slices touch family files strong currently owns; queued behind its A-batch. Verification: 0 warnings; tess-core 84 (its base predates M192's paint tests — reconciled at pick), tess-tui 83, census 81, --lib tests:: 1328, tess-gui 40; census guards green with new aliases (166/185/vec unchanged); insurance suite at integration (hold until green).
- 2026-11-27 — **M194: isohedral card-text restructure — the incidence symbol is now the surfaced identifier on all five IH cards (fast tier; text/render surfacing only; registry unchanged).** Delivered by the fast tier (commit ab5dc87), implementing the user review's Isohedral items: "Every description truncates before the incidence symbol, the one piece unique to that type. Put the symbol on its own line and move the Tactile credit to an About panel." **Card change (all five: IH1/IH6/IH7/IH27/IH29):** the summary line is now exactly `vertex symbol · IHn incidence symbol` (e.g. "V3.3.3.3.3.3 · IH1 incidence symbol") — the symbol is the `method` segment, the card's unique identifier; each description also leads with the symbol on its own line; the GUI fits it inside the existing truncation window (205/202px vs ~230px inner — NO clamp change, NO ui.rs edit), the TUI wraps it fully. **Tactile credit disposition:** all three card-text references (method/source/notes) removed; the credit now lives in docs/families/isohedral.md's new "Data attribution" paragraph — the About panel is an F-bucket GUI feature, deferred. Kaplan authorship retained in descriptions + family/per-tiling docs. **Dropped from summary lines (call-out, all preserved in docs, none test-pinned):** dual partner, isohedral degree ("1-isohedral" now documented in the family doc), monotile, cell content, source — giving the cards the fixed pattern the review asked for. The `laves_members_declare_ranks` guard stays green via explicit `dual_of: None` overrides on the arms (the helper lives in forbidden classify.rs). **Verification:** 0 warnings; census 81; --lib tests:: 1328; tess-core 84; tess-gui 40; no tess-tui changes (validated at integration); per-tab vec [11,20,33,17,15,5,30,8,23,11,12], built-ins 166, memberships 185 unchanged. Insurance suite at integration (hold until green).
- 2026-11-27 — **M195: Jeandel–Rao Wang tiles redrawn — each square becomes 4 corner-to-centre triangles colored by its outer edge's Wang label (fast tier, production; D5; registry unchanged).** Delivered by the fast tier (commit b8f9c24), implementing the user review's Aperiodic item: "Wang tiles don't read as Wang tiles. Draw each square as four colored triangles so edge matching is obvious." **Recon verdict:** the edge-label data is in-module (`TILES` — 11 × (N,E,S,W) palette indices 0..7, plus the exact-color test tables `TILE_COLORS`/`PALETTE_OF` with the pre-existing `jr_handoff_battery` asserting palette ids agree with exact colors) — no labels invented. **Geometry:** `local_pieces()` (5 sub-polygons: 4 edge bands width 0.08 + interior) replaced by `local_triangles()` — 4 CCW triangles joining the cell centroid to one full outer edge each (protos 1=N/2=E/3=S/4=W); `generate()` emits 4 protos per cell seeded `[n,e,s,w]` from `TILES[t]`. **Seam preservation:** each triangle's two outer vertices are the same two square corners the neighboring cell's triangle uses, computed by identical float expressions — shared edges byte-identical; the only new vertex (the centroid) is strictly interior, so no gaps on any shared edge; all four triangles CCW, area 0.25 each, per-cell sum exactly 1 (Python area/CCW pre-check + pinned area-sum test). **Palette:** the unchanged fixed 8-colour tess-core palette indexed by outer-edge label (documented merges: white→0, magenta/pink→5), so touching edges of adjacent tiles show the same color — the edge-matching rule becomes visible. **Pins updated live (no weakening):** `jr_partition_areas_sum_to_cell` SIZE·SIZE·5→·4 (40,000 triangles), seed 5-tuple→4-tuple, ×4-factor comment; `check_ref` signature [(u8,u32,[f64;8]);5]→[(u8,u32,[f64;6]);4]; REF0/REF1 re-derived for 4-triangle geometry; module doc/`description()` rewritten; docs/tilings/jeandel-rao.md + the docs/families/aperiodic.md jeandel-rao row (the partition-count line) re-synced. Registry identity unchanged (same id/name/category; built-ins 166, memberships 185, per-tab vec [11,20,33,17,15,5,30,8,23,11,12]; census 81 green in the worktree). **Verification:** 0 warnings; wang module 9/9; census 81; --lib tests:: 1328; tess-core 94; tess-tui 83; tess-gui 40 (worktree); insurance suite at integration (hold until green).
- 2026-11-27 — **M196: Orientation coloring mode — 4 undirected-axis buckets (fast tier, production; mode count 8 → 9; registry unchanged).** Delivered by the fast tier (commit 9499976 on the M192 tip, picked cleanly onto the M195 tip), implementing the user review's arrangements item: "In both basketweaves and herringbone, color by orientation consistently." **Decision function:** `orientation_axis_deg()` — every polygon edge's direction folded into the undirected [0,180) axis, edge-length-weighted at 0.2° resolution, argmax wins; quantized to 4 buckets at 45° spacing (0=horizontal, 1=diagonal, 2=vertical, 3=anti-diagonal). Vertex-order- and collinear-mid-side-invariant (2:1 dominoes with split long sides — e.g. `herringbone_45`'s 6-vertex polygons — still read their long-side direction); squares (no orientation) resolve to the higher axis; seed/proto never leak into the bucket. **Ramp:** `ORIENTATION_PALETTE` 4 entries (teal/pink/yellow/violet), byte-pinned, distinct from all other palettes. **Separation evidence (new focused integration test `tess-tilings/tests/orientation_separation.rs`, public API, region ±4):** basketweave → {0,2}, herringbone → {0,2}, herringbone-45 → {1,3} — the axis-aligned pair separates cleanly from the 45°-rotated pair; that is the mode's reason to exist. **Plumbing:** ColorMode ALL 8→9 (`Orientation` appended last; parse/label/next/preview coherent — preview_mode = self like the other structure modes); BOTH decision sites synced (paint tile_rgb + raster color_index); tess-gui selector +1 with RenderKey/cache_key untouched (recolor-without-regeneration invariant held); tess-tui palette256_orientation + selection arm; README "nine", USAGE --mode row 9 values, DESIGN.md color-modes section, main.rs expected_modes. **Documented limit:** rotation-seeded reptile shapes (4 rotations of one shape) collapse onto the two axis-aligned buckets (0°/180° and 90°/270° are the same undirected axis) — pattern mode still separates all four seeds. **Pins updated live:** color_mode_all_len_is_nine (renamed), cycle chain Metatile→Orientation→Single, USAGE export-contract 8→9, TUI selector list. Existing 8 modes byte-identical. **Verification:** 0 warnings; tess-core 98 (+4 orientation tests); tess-gui 40; tess-tui 83; tess-tilings lib 1381 + 1 separation test; registry unchanged (166/185). Insurance suite at integration (hold until green).
- 2026-11-27 — **M197: user-review math/correctness batch A — all twelve review items verdicted with programmatic evidence (strong tier, production; three new duals → built-ins 169; one real rendering bug fixed).** Delivered by the strong tier across three commits (ac97008 → 8ff8825 → 66bc6c9), rebased onto the M196 tip by the orchestrator with union census re-derivation. **Per-item verdicts:** **A1** (24c monster integral member) VERIFIED-AS-WAS — (2,2,1,4,3) shown hexagonal-lattice-obstructed via the norm form x²+3y²=120, pinned. **A2** FIXED — Rice dates for pentagon types 9/11/12 corrected to 1976 (only type 13 = 1977); new pinned discriminators `pentagon_13_uncolored_pattern_is_pgg` and `pentagon_14_lattice_rules_out_pg_glide_groups` (pgg vs p2 lattice math). **A3** FIXED/IMPLEMENTED — the last three 2-uniform duals (kuniform-dual-n7/n6/n17, (3.4².6; 3.6.3.6)₁/₂ and (3³.4²; 3².4.3.4)₂) built as face-centroid duals; measured symmetry groups pmm/cmm/pgg computed via the new `duals_match_measured_groups` pgg arm; checker seeds exhaustively searched and pinned; 9 coverage trios; **Laves 33 → 36, built-ins 166 → 169**; three new docs/tilings/kuniform-dual-{n7,n6,n17}.md. **A4** VERIFIED-AS-WAS — star-and-cross is the {8/3} octagram by construction; new pinned discriminator test on the emitted tile (notches on skip-3 tip chords ≤1e-12, off skip-2 ≥0.1r; notch/tip ratio 0.541196 = √((2−√2)/2) vs {8/2}'s 0.765368; apex π/4); the {8/2} two-square figure lives in pic-4-8-8 — user decision queued whether to keep {8/3}. **A5** FIXED — bogus `point_order: Some(6)` dropped from golden-rhomboid. **A6** VERIFIED-AS-WAS — basketweave-3 = triple-basketweave course bond (period 5); the 15-per-cell figure double-pinned (group battery enumeration + area identity 15·2=30=|det|). **A7** VERIFIED-AS-WAS — hankin-hex-motifs pmm measured from geometry (the two hexagons carry different motif classes: 24-seg rosette vs 12-seg extended star); single-construction relatives keep p6m; statements added in module + docs. **A8** FIXED (the review's Pregnant Chairs holes — a real bug): per-block refinement proven exact (every block, depths 1–3, area identity exact), ancestry trace pinned the failing chain — `cover_box` for small chairs used half-extent 2·√3ʳ while portrait dihedral classes (m∈{2,6}) span 2√3 ≈ 3.46, √3 wider, so left-reaching leaves were wrongly culled; fix 2√3ʳ⁺¹; 32-region sweep 23 bad → 0; permanent regression `pregnant_chair_left_edge_covers`. **A9** FIXED — ₁ subscript restored on the n16 dual label. **A10** VERIFIED-AS-WAS — stepped-hexomino card token = measured lattice kind (CenteredRectangular, pinned by legality test); tab "oblique" = p1's `conventional_lattice` display convention (browse::section_key/symmetry.rs both cite this tiling); reconciliation paragraphs added. **A11** FIXED — homochiral→chiral relabels per computed orientation census. **A12** FIXED — girih-decagon-patch recategorized Islamic → Nonperiodic (**Islamic 30 → 29, Nonperiodic 12 → 13**; "p1 · none" contradiction resolved as a finite patch — the A12 rider). **Union census (orchestrator, live-derived):** strong's base-relative 190 memberships predated M191's two dropped IH pentagon secondaries — the true union is **188 memberships = 169 primaries + 19 secondaries**; per-tab vec [11, 20, 36, 17, 15, 5, 29, 8, 23, 11, 13]; tess-tui registry_tabs + tab-bar counts, laves-sort 36, tess-gui card-sweep 169, README (incl. the auto-merged Nonperiodic 12→13 the conflict hunks missed), TAXONOMY, docs/families/{kuniform,aperiodic,pentagons}.md, docs/{INDEX,DESIGN,TAXONOMY}.md — all re-derived from failing guards. **Verification:** 0 warnings; tess-tilings 1394 (1393 + the fixed README guard; +9 A3 trios, +1 A4 discriminator, +1 A8 regression over the pre-A base); tess-core 94 (base-relative; 98 on main with M196); tess-tui 83; tess-gui 40 (41 post-M-about-panel pick); full workspace insurance suite at integration (hold until green).
- 2026-11-27 — **M198: user-review text batch — E10 herringbone-45 rotation note + E4 Ammann–Beenker glyph fix (fast tier + orchestrator; text only; registry unchanged, built-ins 169, memberships 188).** **E10** (fast tier, commit cc64eb3): the herringbone-45 card description gains "Same tiling as herringbone rotated 45° — same lattice, see herringbone." — the review's "same tiling rotated" observation answered with a note, not a dedupe (orchestrator decision: both entries stay); docs synced bidirectionally (docs/tilings/herringbone-45.md blockquote + herringbone.md See-also cross-reference). No guard pinned the description (grep-verified). **E4** (held at threshold by the fast tier with an exact blocker, completed by the orchestrator): the tofu-rendering U+2192 (`→`) in the two Ammann–Beenker cards' cut-project text ("R⁴ → R², octagonal window") is replaced with the font-safe wording "R4 plus R2, octagonal window" — two classification arms in families/quasiperiodic.rs (66/77), the tess-tui attr-lines pin (app_ui.rs:1896, the one file outside the tier's brief), and the tess-core classify.rs doc comment quoting the string. Docs never quoted the string (grep-verified). **Verification:** 0 warnings; census 81; tui 83; core 98; gui 41. Insurance suite at integration.
- 2026-11-27 — **M199: tess-gui About panel (fast tier, production; F4 tail — the M194 disposition's promised home for the Tactile credit).** Commit 7405ae1 (crates/tess-gui/src/ui.rs only, +141). Toggle: the free `H` key (guarded by `wants_keyboard_input()` so typing H in the Browse search field keeps its character) plus a title-bar "About" button (render-bar `right_to_left` idiom). Content: "tessellix" + CARGO_PKG_VERSION (no app.rs plumbing needed — workspace 0.1.0), a "Data attributions" section carrying only credits rendered content actually draws on — Kaplan/Tactile incidence-symbol data (the five shipped IH instances; docs/families/isohedral.md) and the Jeandel–Rao 11-tile Wang set (arXiv:1506.06492; docs/tilings/jeandel-rao.md) — plus the pointer "Per-tiling sources: docs/tilings/*.md". Bonner deliberately excluded: it appears only in source-gated/non-content-bearing comments (theta_search validation leg, medallion no-guessing note, girih W85 provenance comment whose rendered panel draws on Glezer/Wichmann data18) — not an attribution. The panel is a centered, non-resizable, non-modal egui Window, pure overlay — no cache key, no RenderKey change. **Test:** `about_panel_attribution_text_toggles` (headless, new `painted_text` helper for full un-elided galley text, 4 passes per state like `headless_grid_at` — a window's first pass is an invisible sizing pass): closed paints no attribution text, open paints every line. **Verification:** 0 warnings; tess-gui 41 (40 + 1); tess-core 98; tess-tui 83 untouched. Insurance suite at integration.
- 2026-11-27 — **M200: user-review text batch — E6 pentagon card titles + E12 stack-bond fill (fast tier, production; registry unchanged, built-ins 169, memberships 188).** Commit 7771c1d (4 files, +12/−5), orchestrator doc syncs on top. **E6:** the two Laves-family pentagon duals carry the review's naming as their registry card titles — `cairo-pentagonal` name → "Type 4 · Cairo" and `floret-pentagonal` name → "Type 5 · Floret" (ids, classifications, and census unchanged; both are Laves entries with `pentagon_type` 4/5 by id in classify.rs, not pentagons-family members, so no families/pentagons.rs change was needed and no tab-order guard moves). Doc syncs (orchestrator, guard-required): the `tiling_docs_match_registry` H1 contract forces docs/tilings/cairo-pentagonal.md → `# Type 4 · Cairo` and floret-pentagonal.md → `# Type 5 · Floret`, each keeping its old title as a historical parenthetical in the body; laves.md's face-config table cell (now the config `V3.3.4.3.4`, matching the column's semantics), both per-tiling-doc list entries, and the TAXONOMY Cairo-merge note updated; README's mention uses ids and needed no change. The tess-tui cairo attr-lines pin re-derived (the name no longer echoes the face config in summary line2). **E12:** stack bond's `seed_pattern` Uniform → Rows (row-parity fill, the running-bond sibling convention M41 uses) — the card rendered all-white because every tile shared one proto+seed → a single palette index under every mode; module doc comment documents the row-parity fill. Not a B2 palette declaration (no per-family palette machinery involved). Stale unpinned prose in the lib.rs census comment fixed in passing ("166 built-ins — 63 before" → 169; per-tab prose PentagonTypes 17 → 15, Aperiodic/Nonperiodic 12/12 → 11/13, comment order matched to `TilingCategory::ALL`). Historical CHANGELOG quotes (M76 era) intentionally untouched. **Verification:** 0 warnings; census 81; tess-tilings lib guards green post-sync; tess-core 98; tess-tui 83; tess-gui 41. Insurance suite at integration.
- 2026-11-27 — **M201: color mode 10 `Level` — substitution-hierarchy coloring (strong tier, production; E16/C3 follow-up; registry unchanged, built-ins 169, memberships 188; mode count 9 → 10).** Commit 146b3f3 (69 files, +444/−96 branch-side; orchestrator pick onto the M200 tip resolved the duplicate README Nonperiodic hunk as the pre-arranged no-op — main's e9a9daf already carried it). **Design (orchestrator decision, per the coloring recon's STOP flag):** a real `Tile.level: u8` field — substitution level tracked by the engine at tile emission, 0 = non-hierarchical — NOT seed/proto overloading (those already carry per-family signals). Uniform per view in congruent-child engines (level = the patch's position in the hierarchy, so the view's substitution octave reads as color — the E16 visibility mechanism), genuine per-tile lineage in the mixed-size irreptiles. **Threading:** 75 construction sites — 36 carry real depth across 31 engines (reptiles ≤40, pinwheel + penrose p1/p2/pentagon via new iteration counters capped at 24, ammann a3/a4/beenker/shield/stampfli 16–20, glb/golden-rhomboid/fibonacci/domino-family ≤12, girih seed-level N, wis fixed-1, turtle/spectre ≤5, fractalizing ≤16 view-monotone; golden bee/triangle emit **per-tile lineage exponents**, bee pinned to the 5..=10 window at the default view), 39 neutral `level: 0` (hat, cap, Taylor-Socolar, islamic, isohedral, pentagons, uniform/lattice/sweep). **Palette + class:** byte-pinned 9-entry `LEVEL_PALETTE` (neutral lavender-gray + chartreuse→mint→teal→sky→indigo→purple→magenta→rose), distinctness-tested against all six other ramps; `level_class` cycles 8 steps rather than clamping (clamp would collapse levels ≥8; aliasing needs levels 8 hierarchy steps apart in one view — never adjacent). Both decision sites synced (paint.rs tile_rgb + raster.rs color_index); `preview_mode` = self. **Separation tests** (tests/level_separation.rs, 3): golden-bee/golden-triangle ≥3 non-neutral buckets in one view (bee's raw levels pinned to exactly 5..=10); square/hexagon/basketweave exactly `{0}`; fractalizing strictly-increasing level across 5 shrinking views (the E16 zoom mechanism). **STOP documented:** no shipped uniform engine emits mixed depth within one view (one expansion depth per `generate`), so the ≥3-bucket claim rests on the irreptile lineage engines; uniform-engine hierarchy visibility is pinned through the fractalizing zoom series. **Plumbing:** gui selector +1 (RenderKey/cache_key untouched — recolor invariant holds), tui `palette256_level` both constructors + selection arm, main.rs expected_modes, README "ten", USAGE `--mode` + selector rows, DESIGN.md color-modes section. **Verification (branch battery + integration gates):** 0 warnings; tess-core 102 (+4 level tests); tess-tui 83; tess-gui 41; tess-tilings 1394 lib + 4 integration (3 new level_separation + orientation_separation). Insurance suite at integration.
- 2026-11-27 — **M202: user-review text batch — E13 quasiperiodic line-2 fit + E14 Hirschhorn rosette framing (fast tier, production; registry unchanged, built-ins 169, memberships 188).** Commit 9fd059c (3 files, +20/−29), picked onto the M201 tip cleanly. **E13:** the quasiperiodic cards' summary line 2 now fits the ~35-char gui truncation window — line 2 keeps `method + inflation factor` (and `prototiles` where it fits); `cut_project`/`tile_set`/`reflections`/`source`/`year` segments are dropped from the 11 arms whose windows were violated (never the factor or the count — the review's actual requirement); all 18 in-scope arms verified against a live window measurement; the tess-tui attr-lines pin re-derived ("substitution · 1+√2"). **Documented exceptions (queued, out of scope):** golden-rhomboid's name echoes all segments (browse.rs name-echo fallback — forbidden file), girih-14-fold sits at 39 chars (would need factor/count shortening — forbidden), 5 islamic.rs cards over-window (family not in scope), classify.rs:125 doc comment quotes a retired string (same accepted M194 pattern). **E14:** root-caused — the tiling was never off-center; SVG-text measurement shows it C6-symmetric about the origin (tile span ±16.9×±11.8, centered). The rosette's smallness was a **scale** problem: new `HIRSCHHORN_PARAMS` (scale default 2.5, rotation spec identical to `SIMILARITY_PARAMS`) frames the central rosette at ~66% of gui half-height (was ~25%). SVG export untouched (CLI forces scale 1.0); card previews are scale-invariant; the 8 hirschhorn tests pin explicit scales and pass unchanged; documented trade-off: the tui's 8 px/unit default shows the central slice — the user dials scale via the params strip. **Verification:** 0 warnings; census 81; tess-core 102; tess-tui 83; tess-gui 41; hirschhorn 8. Insurance suite at integration.
- 2026-11-27 — **M203: user-review text batch — E8 uniform-tab vertex-config notation + E11 pavers color-fill notes (fast tier, production; registry unchanged, built-ins 169, memberships 188).** Commit 70db355 (22 files, +65/−57), picked onto the M202 tip cleanly; one orchestrator one-liner on top (browse.rs:183 doc-comment echo example synced). **E8:** the three regular tilings are retitled with vertex symbols — "Square grid (4⁴)", "Triangular (3⁶)", "Hexagonal (6³)" — and four Archimedean configs compacted to the Two-Uniform convention: 4.8.8 → 4.8², 3.12.12 → 3.12², 3.3.3.3.6 → 3⁴.6, 3.3.3.4.4 → 3³.4² (names + descriptions + the classification `vertex_config` fields compacted together, so the summary-line skip logic — "a segment is skipped when the card name already says it" — stays coherent; compacting names alone would have re-echoed raw configs). Non-repetitive figures (3.6.3.6, 3.4.6.4, 4.6.12, 3.3.4.3.4) unchanged. The tess-tui square line-2 pin re-derived ("{4,4} · regular · self-dual" — the config now lives in the name). **Elongated-triangular:** the review offered "{3,6}:e or 'no standard operation'" — the tier rejected {3,6}:e on evidence (the card's classification arm carries no Schläfli symbol, unlike the other ten; uniform.md marks it non-Wythoffian; its construction comment describes alternating rows of triangles and squares directly) and shipped "No standard Wythoff operation — built directly as alternating rows of equilateral triangles and unit squares (Conway's isosnub quadrille)." **E11:** the four Bonds & dissections cards state their fill rules explicitly — basketweave + herringbone color by tile **orientation class** (cell seed stores 0 = horizontal / 1 = vertical → the M196 axis-aligned buckets {0,2}, never 45°/135°); running bond + stack bond color by **row parity** (`SeedPattern::Rows`; stack bond cites M200, which gave it the Rows seed) — the running-bond note pins why every horizontal stretcher sits in the 0° Orientation bucket: different rule, not a different lattice. Card notes mirrored as docs/tilings/*.md blockquotes (M198 precedent); pavers.md's table notes column deliberately untouched (doesn't quote description text). **Verification:** 0 warnings; census 81 (incl. ledger test); tess-tilings lib 1341; tess-core 102; tess-tui 83; tess-gui 41. Insurance suite at integration.
- 2026-11-27 — **M204: user-review batch — E7 tail (IH1 disposition + wavy-edge notes) + E13 exceptions (fast tier + orchestrator; registry unchanged, built-ins 169, memberships 188).** Commit cda94f1 (7 files, +17/−20), orchestrator dispositions on top. **IH1 all-white — root-caused, honest disposition (the tier's STOP upheld):** `engine.rs` emits tiles as `(poly, aspect_index)` and `data_ih1.rs` sets `NA: usize = 1`, so every IH1 tile ships seed=0 → palette index 0 uniform. A programmatic census over a ±6 window proves the contrast: ih1 seeds {0:150} single-index vs ih6 {0,1,2,3}, ih7 {0,1,2}, ih27/ih29 {0,1,2,3}. Unlike M200's stack bond (where `Rows` matches real stretcher rows), an oblique p1 mono-isohedral honeycomb has exactly one orbit-identical shape and no parity structure — introducing one would assert mathematics the tiling does not have. **Disposition: the uniform fill is the mathematics**; an explanatory note added to the card description and docs/tilings/ih1-p1-hexagon.md ("the even fill is the mathematics, not a rendering gap"). **Wavy-edge notes:** both IH27/IH29 variant cards (descriptions + symmetry notes), the isohedral family doc's variant paragraph, and the two per-tiling docs now state that the curved/notched per-orbit edge polylines are a deliberate deformation of the underlying IH27/IH29 incidence structure, not a pentagon type; IH29's docs add the accurate sub-note that the Klein-paired orbit forces the base edge to stay straight. **E13 exceptions:** girih-14-fold line2 39 → 22 chars (the b-definition parenthetical dropped from the line2 inflation string only — the definition survives in the full description and docs). The five islamic arms dropped their trailing source+year segments (Lu & Steinhardt 2007 / Andritz 2016 — both remain in descriptions and docs); the residual 45–53-char overflows were closed by the orchestrator's method abbreviations on the line2 surface only: "sigma-frame walk substitution" → "σ-frame walk" (four Andritz arms, 64–81 → ~32–36 chars) and "Darb-i Imam substitution" → "Darb-i Imam" (girih-quasiperiodic, 45 → 32; the girih-decagon-patch arm's "(finite patch)" wording intentionally untouched — the finite-patch character is the A12 disposition). Full method terms kept everywhere else (φ is precedent that Greek glyphs render font-safe). No tess-tui pins quoted any changed string (grep-verified). **Verification:** 0 warnings; census 81; tess-tilings lib 1394; tess-core 102; tess-tui 83; tess-gui 41. Insurance suite at integration.
