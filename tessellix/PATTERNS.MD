# PATTERNS.MD

Catalog of tiling patterns for tessellix: what exists, what is being
built, and what is worth adding next. This file is the working index
between the mathematical literature and the registry; the user-facing
summary of implemented tilings lives in `README.md`, the ground rules
for adding any of these in `AGENTS.md`.

Status markers:

- ✅ **implemented** — in the registry (`tess-tui -- list`)
- 🚧 **in progress** — current work queue, top item is next
- 🔭 **proposed** — vetted candidate with a known construction path
- ❓ **research** — promising but needs design/investigation before scheduling

## Implemented (53 tilings)

### Uniform (Archimedean) — the complete set of 11

| id | vertex figure | group | notes |
|---|---|---|---|
| `square` | 4.4.4.4 | p4m | regular; self-dual |
| `hexagon` | 6.6.6 | p6m | regular |
| `triangle` | 3.3.3.3.3.3 | p6m | regular |
| `truncated-square` | 4.8.8 | p4m | octagons + squares |
| `trihexagonal` | 3.6.3.6 | p6m | kagome |
| `snub-trihexagonal` | 3.3.3.3.6 | p6 | chiral |
| `elongated-triangular` | 3.3.3.4.4 | cmm | non-Wythoffian |
| `rhombitrihexagonal` | 3.4.6.4 | p6m | |
| `truncated-trihexagonal` | 4.6.12 | p6m | great rhombitrihexagonal |
| `truncated-hexagonal` | 3.12.12 | p6m | |
| `snub-square` | 3.3.4.3.4 | p4g | chiral-looking, mirror-symmetric |

The uniform family is **closed**: all 11 exist, so no further
Archimedean work is possible (or needed).

### Laves / Catalan duals — complete: 8 of 8 (+ 1 variant)

The whole Catalan family now mirrors the closed Archimedean set:
every dual is the face-centroid dual of an exact primal table.

| id | face config | group | primal | notes |
|---|---|---|---|---|
| `cairo-pentagonal` | V3.3.4.3.4 | p4g | snub-square | tangential pentagons; minimum-perimeter pentagonal tiling |
| `cairo-collinear` | V3.3.4.3.4 | p4g | — (flattened double-hex construction) | integer-coordinate variant; edges 4 and √10 |
| `tetrakis-square` | V4.8.8 | p4m | truncated-square | right isosceles triangles, 45°×8 at octagon centers; 4 proper pattern colors |
| `rhombille` | V3.6.3.6 | p6m | trihexagonal | 60°/120° rhombi, 3 per cell; pattern 3-colors by orientation (cube illusion) |
| `triakis-triangular` | V3.12.12 | p6m | truncated-hexagonal | 120°-apex isosceles triangles; triangular lattice of dodecagon centers, each cell split three ways |
| `deltoidal-trihexagonal` | V3.4.6.4 | p6m | rhombitrihexagonal | kites 60°/90°/120°/90°, 6 per cell |
| `kisrhombille` | V4.6.12 | p6m | truncated-trihexagonal | 30-60-90 right triangles, 12 per cell; bipartite 2-coloring |
| `prismatic-pentagonal` | V3.3.3.4.4 | cmm | elongated-triangular | pentagons (3×120° + 2×90°), 2 per cell; checker 4-coloring |
| `floret-pentagonal` | V3.3.3.3.6 | p6 | snub-trihexagonal | chiral pentagons (4×120° + 60°), 6 per cell; 6-color floret rosettes |

### Periodic decorative families

| id | motif | group | notes |
|---|---|---|---|
| `star-cross` | {8/3} star and cross | p4m | one-parameter star radius: gapped → kissing (khatam) |
| `hexagram` | hexagram and hexagon ({6/2}) | p6m | one-parameter star tip radius: gapped → kissing hexagram-and-hexagon tiling; stars on the hexagonal lattice, interstice split into two hexagons at the kissing radius |
| `girih-shesh-band` | girih strapwork band | p2 | single shesh-band hexagon, oblique lattice |
| `girih-star-cross` | 10-pointed girih star-and-cross strapwork | pmm | measured classical panel; 545-tile cell data table on the (77,45)/(0,90) oblique lattice (M35) |
| `girih-w85` | 12-pointed girih star interlace | p6 | Wichmann data18/W85 (Alhambra Mexuar): exact Q(√3) strap-line grid (L = 283.92, strap 9, all vertices (p+q√3)/50), 42 tiles per hexagonal cell; motif is *632 but the drawn over/under interlace is chiral → p6 (M52) |
| `pic-4-8-8` | Hankin PIC fourfold khatam (median) | p4m | polygons-in-contact strapwork on the 4.8.8 substrate at δ = 45°: rays run edge-midpoint to edge-midpoint ({8/2} two-square octagram per octagon, midpoint chords per square), face-to-face partition into octagon + small square + 4 house pentagons + 8 star tips, 14 faces per A·Z² cell (A = 1+√2), exact Q(√2) with all vertices (p+q√2)/4; un-interlaced, mirrors and glide probe verified (M53) |
| `pic-3-6-3-6` | Hankin PIC sixfold khatam | p6m | polygons-in-contact strapwork on the trihexagonal 3.6.3.6 substrate at δ = 60°: rays run edge-midpoint to edge-midpoint ({6/2} hexagram per hexagon, midsegments per triangle), face-to-face partition — the wall network *is* a half-scale 3.6.3.6 — into 4 hexagram-core hexagons + 6 hexagram tips + 2 midsegment triangles, 12 faces per ⟨(2,0),(1,√3)⟩ cell, Σ = 2√3 = |det| exact over Q(√3) with all vertices on the (p+q√3)/4 grid; un-interlaced, rot60/rot120/rot180 + five mirrors probe verified (M54) |
| `pic-3-12-12` | Hankin PIC twelvefold khatam | p6m | polygons-in-contact strapwork on the truncated-hexagonal 3.12.12 substrate at δ = 60°: the tangent-chord theorem closes a midpoint ray onto another midpoint only at δ = 180°·k/n, so δ = 60° is the single angle that closes both polygons — 12 chords `M_i→M_{i+4}` per dodecagon (the {12/4} rosette, four overlaid triangles) plus the triangle midsegments; face-to-face partition into 45 faces per ⟨(3/2+√3, 1+√3/2),(0,2+√3)⟩ cell in six orbits (1 rosette-core dodecagon + 6 vertex pentagons + 12 band quads + 2 medial triangles + 12 inner quads + 12 star notches), Σ = 6 + 7√3/2 = |det| exact over Q(√3) with all vertices on the (p+q√3)/24 grid and corners only 30–150°; un-interlaced, rot60/rot120/rot180 + three mirrors probe verified and 7 negative probes pin the group to p6m (M56) |
| `khatam-girih` | khatam ({8/3}) girih-tile decomposition | p4m | classical kissing star-and-cross cut into 8 rhombi + square + 4 pentagons per cell; exact Q(√2) kitab (M36) |
| `pythagorean` | two squares (Pythagorean) | p4 | one-parameter ratio r = b/a; each cell is the square on the hypotenuse dissected into the leg squares; T-junctions inherent; the plain p4m square grid at r = 1 |
| `windmill` | 1–2–√5 triangles around squares ("windmill") | p4 | periodic pinwheel arrangement; homochiral (no mirrors/glides); non-edge-to-edge T-junctions; naming waiver 2026-11-12 (M43) |

### Paver bonds (2:1 dominoes)

Practical paver bonds as `LatticeTiling` data tables: every domino is
emitted with its long side split into two collinear unit edges, so the
polygons are genuinely edge-to-edge at the unit edge scale (180°
figures at the split midpoints, `2.4.4`).

| id | motif | group | notes |
|---|---|---|---|
| `basketweave` | 2:1 dominoes in 2×2 alternating blocks | p4g | 45°-rotated square lattice ⟨(2,2),(2,−2)⟩, 4 split dominoes per cell; 4-folds at the even-even points, mirrors at the odd axes only (offset from the 4-fold centers — p4g, not p4m), six glide families; exact coset group probe (M39) |
| `herringbone` | 2:1 dominoes in chiral L-pairs | pgg | oblique lattice ⟨(1,3),(2,2)⟩, 2 split dominoes per cell; single-direction (chiral): 16 2-fold centers per 4×4 period, two perpendicular glide families (diagonal + anti-diagonal axes), no mirrors; exact coset group probe (M40) |
| `running-bond` | 2:1 dominoes in stretcher rows, half-brick offset | cmm | rectangular lattice ⟨(2,0),(0,2)⟩, 2 split dominoes per cell; mirrors at the integer x-axes and half-integer y-axes, 2-folds at (k, l+1/2) and (k+1/2, l), glides in both directions; exact coset group probe (M41) |
| `kuniform-p6m` | 2-uniform (3⁶; 3².6²) | p6m | hexagonal tiling with one of every three hexagons (index-3 sublattice m ≡ n mod 3 of the honeycomb centers) dissected into six-triangle fans; Wikipedia `2-uniform n10`, t = 2, e = 3 |
| `kuniform-pmm` | 2-uniform (3.6.3.6; 3².6²) | pmm | hexagonal tiling with rows of up- and down-pointing triangles filling the gaps between rows of hexagons (rectangular lattice, hexagons share full horizontal edges); Wikipedia `2-uniform n11`, t = 2, e = 3 |
| `pentagon-13` | Rice pentagon, type 13 (A = 120°) | p2 | 2-isohedral chiral pair (seeds 0/1), 8 tiles per rectangular cell, exact Q(√3); long side split into two collinear a-edges (M49) |
| `pentagon-14` | Stein pentagon, type 14 (completely determined) | p2 | 3-isohedral (seeds 0/1/2), 6 tiles per oblique cell, exact Q(√57, β) with β = √((11√57−25)/8); long sides split into two collinear unit fine edges — 7/5/6-vertex cells (M50) |

### Aperiodic / nonperiodic

| id | motif | notes |
|---|---|---|
| `penrose` | Penrose P3 (thick/thin rhombi) | Robinson-triangle substitution, φ-inflation, chirality-aware; sun-seeded deflation growth |
| `penrose-p1` | Penrose P1 (pentagon, star, boat, diamond) | exact φ² deflation: short-edge (1/φ) pieces merge across parent boundaries (G+G→D, CORE+2G→B, S2C+2G→S, S1C+G→S); single-seed pentagonal gasket with 5-fold notch channels; clean-disk radius R/φ² guarantees view coverage |
| `penrose_p2` | Penrose P2 (kites & darts) | glue-labelled half-tile substitution (A → 2A+O, O → A+O); halves pair along glue legs into kites/darts; sun/star seeds; unique labelling found by exhaustive search |
| `penrose-pentagon` | five pentagons (φ-substitution) | aperiodic set found by exhaustive pool search (strict-0 + 1-hole layers); M = [[0,1,0,1,0],[1,1,0,0,1],[2,0,0,0,3],[0,0,1,0,0],[1,1,0,0,1]] in order (50,105,451,468,484); single-seed deflation growth |
| `pinwheel` | 1–2–√5 right triangle | Conway–Radin; √5 inflation + arctan(1/2) twist; infinitely many tile orientations; view-independent octave-anchored patches |
| `chair` | Chair (L-tromino) | λ = 2 rep-4 substitution; the inflated chair's dissection is unique (exhaustive enumeration) so the rule is forced; non-edge-to-edge (T-junctions at every scale); limit-periodic; 4×3 seed blocks, one chair per orientation; octave-anchored like pinwheel |
| `sphinx` | Sphinx (pentagonal hexiamond) | λ = 2 rep-4 substitution on the triangular lattice; of 12 hexiamonds only the 3×1 rhombus and the sphinx are rep-4, and the sphinx's dissection is unique (rule forced); 12 orientation classes (dissection mixes chiralities), doubly stochastic substitution matrix; non-edge-to-edge; 2×3 rhombus seed (exactly 2 sphinxes, unique); octave-anchored like chair |
| `p-pentomino` | P-pentomino (rep-4 reptile) | λ = 2 rep-4 substitution; exactly two dissections of the inflated 20-cell P (exhaustive enumeration, class multisets {0,1,4,6} and {3,4,4,5}), canonical {0,1,4,6} used; 8 chiral orientation classes, doubly stochastic substitution matrix (rows/columns sum to 4); non-edge-to-edge (5 T-junctions per parent); 5×8 seed block, one tile per class (62 such tilings of the rectangle); octave-anchored like chair (M47) |
| `ammann-beenker` | Ammann–Beenker A5 (octagonal aperiodic) | half-tile substitution (two rhombus halves + square) with per-level mirror flips; exact Q(√2) geometry; edge-to-edge; 8-fold finite patches |
| `stampfli` | Stampfli 12-fold dodecagonal | squares + equilateral triangles + 30° rhombi; Stampfli substitution, inflation 2+√3 (Bielefeld encyclopedia rule); straddling unit children deduped by exact vertex set; 12-rhombus seed star, exact Z[√3]/4 anchors; edge-to-edge, T-junction-free, 12-fold symmetric at every generation |
| `hat` | Hat (aperiodic monotile, "einstein") | SMKG 2023; H/T/P/F metatile substitution (29-rule patch, 22 kept), exact Eisenstein placements; inflation φ² only in the limit (iterated outlines drift by design); tiles only with its mirror image; per-view seeded lineage blobs with exact clipped-area acceptance; non-edge-to-edge (565 midpoint T-junctions at depth 3) |
| `turtle` | Turtle ("hats in turtles", einstein family) | turtle (14 sides, area 10√3) + hat prototiles on Kaplan's exact hex lattice; 9-label 8+1 key-point substitution with per-level chirality flip; homochiral (no reflected tiles), edge-to-edge; turtle:hat ratio → 4+√15 |
| `spectre` | Spectre (chiral einstein Tile(1,1)) | single equal-edge prototile, aperiodic *without* reflections; 9-label D12 substitution (spectre.js-identical), exact Q(√3) surd geometry; edge-to-edge |
| `hirschhorn` | Hirschhorn–Hunt pentagon (140°/60°/160°/80°/100°) | not locally forced; C6-symmetric constrained-growth realization |
| `girih-quasiperiodic` | girih star-and-cross {10/3} | quasiperiodic; Darb-i Imam 3-prototile substitution (Lu & Steinhardt 2007, IF = 2φ³); exact 10-fold seed rosette, edge-to-edge at every generation |

## Arc: "complete the classics" — ✅ complete (M19–M29)

Order below is the build order (mechanical duals first, then the
direct implementation, then the rule-derivation heavyweights). One
commit per tiling, `M19` onward, each with the three standard tests,
the brute-force culling cross-check, and README/PATTERNS updates.

### 1. Laves completion — the 7 missing Catalan duals ✅ (M19–M25)

All seven were data tables derived by the cairo-pentagonal method:
dualize the existing exact primal table (face centroids in cyclic order
around each primal vertex); coordinates stay exact in Q(√3) (or Q(√2)),
module `crates/tess-tilings/src/laves.rs` behind feature `laves`.
M19 tetrakis-square, M20 rhombille, M21 triakis-triangular, M22
deltoidal-trihexagonal, M23 kisrhombille, M24 prismatic-pentagonal,
M25 floret-pentagonal — **the family is complete (8/8), mirroring the
closed Archimedean set.**

Every table was verified in exact quadratic-field arithmetic before
transcription (area sum == det, edge parity 2, vertex figures 360/n,
shifted-grid coverage, proper-coloring search), mechanically
cross-checked against the emitted Rust literals (vertex-by-vertex
numeric diff — caught an x/y slip in M23 the coverage test also
caught), and measured in the exported SVG (side/angle multisets).
`laves_pattern_mode_is_proper_coloring` (lib.rs) re-checks proper
pattern coloring on generated patches. The verification script lives
at `/tmp/opencode/laves_dual.py` (exact Q(√d) arithmetic, per-dual
battery) with Rust-ready literal emission (`--rust` flag).

### 2. Pythagorean (two squares) ✅ (M26)

- **Motif:** squares of two sizes, every square touching the other size
  on its sides; unilateral, equitransitive, *not* edge-to-edge
  (T-junctions inherent — documented like pinwheel's).
- **Symmetry:** p4, chiral (no mirrors) — confirmed both analytically (a
  mirror would need the lattice orientation arctan(a/b) ≡ 0 mod 45°, i.e.
  a = b) and numerically (38 four-fold centers, zero reflections in a
  generated patch); at r = 1 the tiling is the plain square grid, p4m.
- **Lattice:** rotated square lattice with the LARGER side first,
  `[(s1, s2), (−s2, s1)]`, cell = big square `[0,s1]²` + small square
  `[s1, s1+s2] × [0, s2]`, cell area exactly s1² + s2² = |det| (the
  square-on-hypotenuse dissection). **The plan's `[(b, a), (−a, b)]` is
  only valid for b ≥ a**: for r < 1 the small square at (b, 0) pokes into
  the big square one lattice step down-right (overlap b(a−b)), so the
  construction canonicalizes (s1, s2) = (max(a,b), min(a,b)); r < 1 is
  the r′ = 1/r family, congruent up to scale.
- **Implementation:** direct `Tiling` impl in `pythagorean.rs` (feature
  `pythagorean`, shared with pinwheel) with the lattice.rs index-range
  culling inline; third parameter `ratio` (0.2–10, step 0.05, default φ)
  following the star-cross pattern.
- **Verification:** Python sweep (coverage grid + sub-edge parity over a
  ±3 patch) across r ∈ {0.2 … 10}; the Rust `pythagorean_ratio_sweep_covers`
  test re-checks coverage at 7 ratios *plus* rotated/scaled views at
  steep ratios, and `pythagorean_culling_matches_brute_force` re-emits a
  fixed index window and requires bit-identical tile sets at r = 1.1
  (basis tilted 42°) and 2.5. **The sweep caught two real bugs before
  commit**: the r < 1 notch overflow above, and a culling bug (region
  bbox taken in cell space instead of lattice/u space — harmless for
  axis-aligned lattices like star-cross, wrong for rotated ones; fixed
  by mirroring lattice.rs's world→u inverse exactly).
- **Bonus tie-in (kept for future work):** irrational ratios give
  aperiodic 1-D cross sections (Klotz construction); default ratio φ
  gives Fibonacci-word cross sections.

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

## Follow-up: chair tiling — ✅ complete (M30)

Pulled from the proposed queue (a classic, and the pedagogical
monohedral contrast to the pinwheel). One commit with the standard
battery.

### 5. Chair tiling (L-tromino) ✅ (M30)

- **Motif:** the chair (L-tromino, 2×2 block of squares minus one
  quadrant), rep-4 rep-tile at λ = 2: a chair of square side s splits
  into 4 chairs of side s/2 — in the parent's frame at bbox origins
  (0,0), (s/2,s/2), (s,0), (0,s) with orientation deltas 0,0,1,3,
  rotated by 90°·o for a parent of orientation o and re-aligned to the
  parent bbox. Working orientation labels 0..3 = missing quadrant
  UR/UL/LL/LR; everything on the integer grid.
- **Rule is forced:** exhaustive enumeration of all L-tromino tilings
  of the inflated chair (12-cell region, exact integers,
  `chair_verify.py`) finds **exactly one** dissection, orientation
  multiset (0,0,1,3). No labelling search needed.
- **Non-edge-to-edge, provably:** the forced dissection lands a child
  vertex strictly inside a sibling's edge at every scale (canonical
  side-2 parent: LL child's reentrant vertex (2,1) in the middle of the
  center child's bottom edge; T-junction census 0, 12, 64, 256 over
  generations 0–3). Like the pinwheel, documented and asserted, not
  fixed. The correct invariants are stronger anyway: refined edge
  census (split every edge at foreign vertices → every refined segment
  shared by exactly 2 chairs or exactly on the patch boundary, lone
  segments totalling exactly the perimeter).
- **Verification (exact integers, `chair_verify.py`):** dissection cell
  census for all 4 orientations; seed enumeration (4×3 rectangle has 4
  L-tromino tilings, 2 use all orientations; chosen seed among them);
  growth g0–g3 from the 4×3 seed block: perfect cell tiling of the same
  rectangle every generation, refined edge-to-edge, exact area, count
  law 4^(g+1), orientations uniform (4^g each).
- **Implementation:** `chair.rs` (feature `chair`), integer `Chair
  {o, x, y, s}` grid (exact i64 arithmetic, f64 only at emit), seed =
  4×3 blocks (one chair per orientation, the all-orientation tiling of
  the enumeration) on the origin-anchored block lattice, per-step
  culling, depth quantized to powers of 2 from the view extent
  (octave-anchored like pinwheel; final chairs always land at unit
  scale, area 3·scale²). `proto` always 0; `seed` = orientation + 1
  (1..=4) for pattern mode. Notes: aperiodic, limit-periodic,
  non-edge-to-edge. 12 Rust tests: substitution-tiles-parent (exact
  cell census, all orientations), seed block, counts/area law,
  per-generation perfect tiling, the inherent T-junction, octave
  view-stability (bit-exact tiles under grow/pan/zoom), unit-scale
  emission, emitted-outline shape, rotation-aligned vertex lists
  (keeps all four classes sharing one gui triangulation-cache entry),
  and the standard battery.

## Follow-up: rep-tile sprint — 🚧 in progress (M32+)

The monohedral substitution track after the chair (M30): more rep-tiles
reusing the exact-integer playbook (derive the rule computationally,
verify in exact Python before Rust, octave-anchored generation).

### 6. Sphinx tiling (pentagonal hexiamond) ✅ (M32)

- **Motif:** the sphinx — a pentagonal hexiamond (6 unit triangles of
  the triangular lattice, 5 boundary sides), rep-4 rep-tile at λ = 2.
- **Everything machine-derived (`sphinx_verify.py`, exact integers on
  the triangular lattice, no literature transcription):**
  - group machinery: the 12 lattice symmetries generated by R (60° CCW)
    and a reflection, each verified to permute the 6 unit edge
    directions (det ±1);
  - enumeration of ALL free polyiamonds up to size 6 by growth
    (counts 1, 1, 1, 3, 4, 12 — the hexiamond family count, a
    self-check);
  - exhaustive rep-4 census: of the 12 hexiamonds exactly two are
    rep-4 — the 3×1 rhombus and the sphinx (the only pentagonal one);
  - the sphinx's dissection is UNIQUE (one exact cover of the inflated
    24-cell region by four side-1 sphinxes in all 12 orientations), so
    the rule is forced; the sphinx is asymmetric (trivial stabilizer),
    so every child transform is unique: children (class, raw anchor) =
    (1,(4,0)), (3,(2,1)), (8,(0,3)), (8,(0,6)) — three reflections and
    one rotation, hence ALL 12 orientation classes occur;
  - the conjugated rule (child class = parent∘k, child anchor =
    parent frame applied to the canonical anchor) tiles the subdivided
    parent for all 12 parent orientations;
  - substitution matrix M is 12×12 with every row AND column summing
    to 4 (uniform class growth from uniform seeds);
  - seed search: rhombi with unit-direction sides, smallest first —
    the 2×3 lattice rhombus (12 cells) is exactly tiled by **two**
    sphinxes (classes 9 = identity at the origin, 2 = 180° rotation at
    lattice (2,3)), the tiling is unique;
  - growth battery g0–g4 from the seed at scale 2^4: perfect
    unit-triangle census of the SAME 32×48-cell rhombus every
    generation (3072 cells), counts 2·4^g, class law = M^g applied to
    the seed distribution, total area invariant, refined edge census
    (every segment shared by exactly 2 sphinxes or on the patch
    boundary), final tiles exactly at unit scale.
- **Non-edge-to-edge, provably:** T-junction incidences per generation
  0, 2, 16, 78, 344 (child corners land strictly inside sibling edges
  at every scale) — the chair/pinwheel situation: documented and
  asserted, not fixed.
- **Implementation:** `sphinx.rs` (feature `sphinx`), integer
  `Sphinx {cls, q, r, s}` on the triangular lattice (exact i64
  arithmetic; world x = q + r/2, y = (√3/2)·r), tabulated 12-symmetry
  group with composition lookup, `CHILDREN` rule table, seed = 2×3
  rhombus blocks on the origin-anchored block lattice, per-step
  culling, depth quantized to powers of 2 from the view extent
  (octave-anchored like chair/pinwheel; final sphinxes always land at
  unit scale, area 3√3/2). `proto` always 0; `seed` = class + 1
  (1..=12) for pattern mode. Mirrored classes emit the transformed
  canonical outline reversed (CCW everywhere); rotation classes keep
  i↔i correspondence. Notes: aperiodic, limit-periodic,
  non-edge-to-edge. 11 Rust tests: group-table closure, emitted
  outlines vs an INDEPENDENT boundary-walk derivation (all 12
  classes), substitution-tiles-parent (exact census, all classes, two
  scales), seed block vs independently constructed rhombus cells,
  growth battery (count/class/area laws + perfect census + unit
  scale), the inherent T-junction (exact gen-1 count 2), octave
  view-stability, unit-scale emission, and the standard battery.
- **Status:** committed (M32). 32 tilings, 176 tests.

### 7. Quaquaversal tiling — ❌ not applicable (3D)

- **Finding:** the quaquaversal tiling (Conway–Radin 1998; "quaquaversal
  tilings and rotations", Invent. math. 132) is a tiling of **3-space**
  by 30-60-90 triangular prisms, with orientation group G(6,4) (60° and
  90° rotations about perpendicular axes) dense in SO(3). The child
  rotations leave the plane, so there is no 2D quaquaversal tiling; the
  2D dense-orientation analogue in this repo is already the pinwheel.
  Removed from the 2D queue (2026-11-09). A 3D extension of tess-core
  would be a separate large arc and is not planned.

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

### 9. Girih star-and-cross redo ✅ (M18 redo)

- **Motif:** the girih star-and-cross — the quasiperiodic girih pattern
  of Lu & Steinhardt (*Science* **315**, 1106 (2007)) as realized in the
  Darb-i Imam shrine, Isfahan: the classical 10-fold rosette (a decagon
  ringed by bow-tie "petals") at the seed, quasiperiodic order
  throughout. The original M18 entry embedded a static 41-tile patch
  whose individual tiles were valid but whose arrangement was wrong
  (no 10-fold symmetry, no growth); this redo replaces it with the
  genuine **Darb-i Imam 3-prototile substitution**, so the entry now
  generates the actual quasiperiodic tiling.
- **The substitution system (derived from Greg Egan's Darb-i Imam
  applet source, then re-derived exactly):** three unit-edge girih
  prototiles — DEC (regular decagon, circumradius φ), HEX (bobbin
  hexagon), BOW (bow-tie hexagon); 20 orientations in 18° steps;
  inflation factor IF = 2·(2+√5) = 2φ³ ≈ 8.4721. Each parent expands to
  a fixed cluster of unit children placed in the parent's frame scaled
  by IF; clusters protrude across the parent boundary and adjacent
  protrusions are *identical tiles* (this is what meshes the clusters).
  Seams are filled by the **edge rule**: every tile also emits one
  *edge marker* per interior edge of its closed cluster, and each
  marker substitutes to the straddling trio `[HEX, DEC, HEX]` at
  offsets ∓(3φ+1)/2 along the edge. The bookkeeping is exact: the
  markers of a generation are precisely the undirected edge set of
  that generation's patch (verified bijectively at generation 2:
  55 080 markers = 55 080 patch edges, zero duplicates).
- **Exact arithmetic:** positions live in K = Q(a), a = 2cos(π/10)
  (degree 4), stored as integer 4-tuples over (1, a, a², a³) at a
  global denominator 64 — every construction quantity has true
  denominator dividing 8, so the fixed-point representation is closed
  and exact (all generated coefficients stay ≡ 0 mod 8, inductively).
  The child tables were derived by BFS-propagating centres through
  exact edge-contact equations and cross-checked against Egan's
  doubles to ≤ 1.1e-14; rotations multiply exactly in K
  (a⁴ = 5a² − 5). Table counts: HEX parent → 57 children + 210
  markers, BOW → 25 + 114, DEC → 181 + 660.
- **Verification (Python prototype, then re-run in Rust):**
  generation 1 = 211 tiles (90 HEX, 80 BOW, 41 DEC) + 660 markers;
  generation 2 = 16 531 tiles (7 350, 6 740, 2 441) + 55 080 markers;
  for both: every edge unit (exact dot product), zero duplicate tiles,
  every interior edge matched by opposite orientations, zero
  T-junctions, legal vertex fans (disjoint sectors; interior vertices
  one closed 20-class cycle), Euler characteristic 2, exact 10-fold
  rotational symmetry (canonical per type: DEC 36°, HEX/BOW 180°).
  Frozen core: P₂ = P₃ exactly up to radius 79 (P₃ = P₄ to ≈ 643) —
  inside the core the tiles are depth-independent. Parent clusters
  cover their parents (dense sampling, all 3 types), which makes the
  covered-disk growth structural: measured ι₂ = 111.30, ι₃ = 910.01
  (ratio 8.18×/generation).
- **Generation for a view:** one global lineage from a seed unit
  decagon at the origin (plus its 10 boundary edges as markers).
  Depth selection from a conservative covered-radius table
  (111·8^(k−2), k ≤ 12 ⇒ covered radius ≈ 1.2e11), then the patch is
  built top-down with an **S-box window cascade** (each level keeps
  only objects whose children can reach the view; child offsets are
  bounded by 13.31 in child units, so an inter-level margin of
  13.31/IF ≈ 1.571 suffices — the cascade was proven equal to the
  full lineage exactly at generations 2 and 3, on- and off-origin).
  Acceptance is exact clipped-area equality against the view box
  (valid to ~1e5 in tiling coordinates; beyond that the depth-12
  patch is verified structurally — at rcorr ≈ 7.2e10 the position
  coefficients reach ~1e13 and kmul intermediates stay below
  ~3e16, two orders inside i64). Emission is unit-edge at every
  depth; zooming deepens the patch; the frozen core is
  depth-independent, so nested views are bit-identical there.
- **Implementation:** `girih_qc.rs` (feature `girih`), 3 prototiles
  (proto 0 = DEC, 1 = HEX, 2 = BOW; seed = proto). 12 Rust tests: the
  exact K arithmetic (unit edges, exact rotation tables), the
  substitution-table closure (markers == interior edges, bijective,
  offset bounds), generations 1–2 full structural battery, cascade ==
  full lineage, core depth-stability, view coverage at depths 2–5,
  depth-12 structural validity, view guards, and the standard trio
  plus distant-view coverage. TUI export visually verified against the
  Python prototype's render (10-fold rosette, tile-for-tile).
- **Status:** committed (M18 redo). 33 tilings, 197 tests.

## Follow-up: measured ornamental panel — ✅ complete (M35)

### 10. Girih star-and-cross, 10-fold periodic (measured) ✅ (M35)

- **Motif:** the classical *periodic* 10-pointed girih star-and-cross
  strapwork: 10-pointed stars on an oblique lattice, joined by chord
  straps with bobbin (elongated hexagon) motifs between the stars and
  small square/diamond ground faces at the diagonal positions — the
  well-known strapwork wallpaper that the quasiperiodic Darb-i Imam
  entry (`girih-quasiperiodic`, M18) is the aperiodic cousin of. Unlike
  M18's exact substitution system, this entry reproduces a measured
  classical panel as a data table: the artwork is decoded from a
  reference image and re-emitted as a `LatticeTiling` cell.
- **Decoded geometry:** reference panel registered at 1 px/unit;
  fundamental cell `[-38.5, 38.5] × [-45, 45]` (area 6930) on the
  oblique lattice `a1 = (77, 45)`, `a2 = (0, 90)`; star centres at the
  index lattice, bobbin centres at the half-lattice holes; wallpaper
  group **pmm** (mirror axes through the stars, confirmed numerically:
  mirror residuals 0.008–0.02 vs 0.005 noise baseline). Cell contents:
  99 white ground faces (the central 10-pointed pinwheel ≈ 443 u²,
  4 diagonal faces ≈ 320 u², 4 petals, 2 side corridors, 4 corners,
  small squares/diamonds) and 446 ink strap pieces.
- **Measurement pipeline (Python, shapely/scikit-image):** strap band
  families fitted to the ink (perpendicular dark-run profiles with
  ridge-snapped centrelines — the snapping objective is the
  threshold-crossing midpoint, not the peak, because JPEG blur makes
  the darkness profile asymmetric), duplicate-band merging
  (union-find on near-collinear candidates), square end-caps trimmed
  against neighbouring bands and the raster ink, band union clipped to
  the (dilated) raster ink polygon, residual ornament ink covered by
  skeleton-traced bands; white faces = exact complement of the emitted
  ink tiles (ink pieces themselves = medial-skeleton partition of the
  ink region, skeleton branches extended along their direction until
  they exit the ink, hole-containing cells freed by thin-corridor cuts
  into simple polygons), and a final conformalization pass (every tile
  edge split at foreign vertices, zero-length edges dropped) so shared
  boundaries have identical vertex sequences. Tiles classified into
  D2 orbit classes (IoU ≥ 0.8 under the cell's point group over the
  3×3 lattice patch): the big faces fall into clean orbits (pinwheel;
  4 diagonals; 4 petals; 2 corridors; 4 corners; …) with median
  within-orbit area spread 0.13 u²; pattern colors use the two-way
  white/ink semantic (M15 precedent).
- **Verification (on the exact shipped data, Python):** sum of cell
  areas = 6930 to 3.9e-16 relative; every interior edge shared by
  exactly 2 tiles (98 border edges once, 0 interior violations); every
  interior vertex angle sum = 360° exactly (0 violations at 1822
  interior vertices); fine grid over the ±3×±3 lattice patch covered
  exactly once (27 720/27 720 sample points); 3 000-point random
  partition test zero-gap/zero-overlap. Rust: the standard trio plus
  `lattice_culling_matches_brute_force` re-emission match.
- **Implementation:** `girih_sc.rs` (feature `girih_sc`), one
  `LatticeTiling` data table — 545 cells, coordinates as 4-decimal
  literals bit-identical to the verified JSON; seed 0 = white ground
  faces, 1 = ink strap pieces. First measured-panel entry (AGENTS.md
  convention amended: decimal data tables allowed with documented
  pipeline + exact partition verification). TUI export visually
  verified cell-against-cell against the reference panel (pinwheel,
  petals, chord straps, bobbin motifs, corner structures all
  correspond).
- **Status:** committed (M35). 37 tilings, 227 tests.

## Follow-up: khatam decomposition — ✅ complete (M36)

### 11. Khatam girih decomposition (8-point) ✅ (M36)

- **Motif:** the classical *khatam* ({8/3} star-and-cross, "khatam" =
  the 8-pointed star of two squares, one rotated 45°) decomposed
  girih-style: the kissing arrangement (M15's `star-cross` at
  `T = 1/2`) cut into a small prototile kitab, the way classical zellige
  panels cut the star-and-cross into piece molds. Sibling of M15 (the
  undivided one-parameter family) and of the girih entries M18/M35 on
  the 10-fold side.
- **The decomposition (exact, all angles multiples of 22.5°):**
  * each {8/3} star is cut by the 8 segments joining its centre to its
    notches into **8 congruent rhombi** (45°/135°) with vertices
    centre–notch–tip–notch; every side equals the notch radius
    ρ = √(4−2√2)/4 (in lattice units, star tip radius 1/2);
  * each interstice cross (the concave 16-gon between four kissing
    stars) splits into the **central square** through the four diagonal
    star-tip apexes (side (2−√2)/2) plus **4 congruent pentagons**
    (angles 135°, 90°, 90°, 112.5°, 112.5°; four sides ρ and one square
    side), each bounded by 4 star edges and 1 square edge.
  Per unit cell: 8 rhombi + 1 square + 4 pentagons = 13 tiles; areas
  8·(√2−1)/8 + (3−2√2)/2 + 4·(1/8) = **1 exactly**.
- **Edge-to-edge across the plane:** every edge is a full edge on both
  sides (no T-junctions, no gaps); vertex figures **4⁸** at the star
  centres, **90.135.135** at the star notches, **45.45.135.135** at the
  kissing points, **45.90.112.5.112.5** at the diagonal tips — exactly
  four vertex orbits. Wallpaper group **p4m** (square lattice; the cut
  is invariant under all its mirrors).
- **Verification (Python, stdlib, ±3×±3 patch = 637 tiles):** per-cell
  area sum = 1 to 1.1e-16; every emitted tile has exactly the claimed
  sides and angles; all 448 interior edges shared by exactly 2 tiles;
  all 257 interior vertex angle sums = 360° in exactly the four orbits
  above; a 29 580-point irrational-offset grid over [−2,2]² covered
  exactly once; the patch set invariant under the p4m mirror and 90°
  rotation. All vertices built from exact Q(√2) const expressions
  (no decimals). Rust: the standard trio plus
  `lattice_culling_matches_brute_force` (list extended); suite green.
- **Implementation:** `khatam_girih.rs` (feature `khatam_girih`), one
  `LatticeTiling` — 13 cells built programmatically from the exact
  constants (`t2 = 1/4`, `tn = (√2−1)/4`, `ts = √2/4`, 45° rotation in
  Q(√2)); each shared vertex computed once so edge partners are
  bit-identical. Seeds: 0 = pentagons (largest, 1/8), 1 = square,
  2 = rhombi (pattern view: gray crosses with blue square centres, red
  rhombus stars). TUI export verified against M15's `star-cross` at
  T = 1/2: identical silhouettes, cut lines added inside.
- **Status:** committed (M36). 38 tilings, 230 tests.

## Follow-up: penrose P1 + einstein family — ✅ complete (M31, M37, M38)

(Committed between M34 and M35 — numbering follows the milestone
labels, which are out of chronological order.)

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

## Follow-up: Stampfli 12-fold — ✅ complete (M45)

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
## Follow-up: convex pentagon census — 🚧 in progress (M49+; M49, M50 landed)

The PROPOSED.md #5 arc (all 15 convex pentagonal tiling types). Type 13
(Rice) lands first: its 8-tile primitive unit is small enough for a
plain data table; type 14 (Stein) second: a 6-tile unit, but with the
census's first exact-Q(√57) surd field.

### 15. Pentagon type 13 (Rice) ✅ (M49)

- **Motif:** Rice's type-13 convex pentagonal tiling (d = 2a = 2e,
  B = E = 90°, 2A + D = 360°, which forces C = A), canonical instance
  A = 120°. 2-isohedral: the two isohedral classes are the two
  chiralities of the scalene prototile, and each primitive unit holds
  eight tiles (Wikipedia: "each primitive unit contains eight tiles").
- **Geometry:** exact Q(√3), b = 1: a = e = (3√3+1)/13, c = (4−√3)/13,
  d = (6√3+2)/13 = 2a; angles A = C = D = 120°, B = E = 90°; tile area
  9(14+3√3)/338. Not edge-to-edge: the long side d is drawn as two
  collinear a-edges (6-vertex figure A,B,C,M,D,E with M the d-midpoint),
  so every joint is a full fine-edge against a full fine-edge
  (a = e = d/2) — the "non-edge-to-edge" sense is only the pentagon-level
  d = d/2 + d/2 split.
- **Unit cell:** 8 tiles on the rectangular lattice T1 = (3s, 3s),
  T2 = (s, −s) with s = (9+√3)/13 (T1 ⊥ T2, |T1| = 3|T2|,
  |det| = 6s² = 8 × tile area). Lc is the index-3 sublattice
  {α + β ≡ 0 (mod 3)} of the fine joint-group lattice; the joint group
  itself (det 8A/3) is a supergroup of the tiling's translations and
  cannot be used as the tiling lattice. All 48 tile vertices lie on the
  (p + q√3)/26 grid. The table was extracted from an exact Q(√3)
  forced-growth unit (joint tree: L1 = rot180(L0, mid-d); G0, G1, D0,
  Y0 = b-b reflections; Y1 = a-e rotation; D1 = rot180(D0, mid-d)) and
  verified before porting: 9×9 block (648 tiles) with zero overlaps and
  exactly-once coverage on an irrational-offset fine grid, edge parity
  all-2 in the interior (count-1 only on the outer ring), and interior
  vertex figures (120,120,120), (180,90,90), (180,180), (90,90,90,90).
- **Group:** pgg at the pattern level — two perpendicular glide
  families (±45° directions; g² a lattice translation,
  chirality-flipping), no mirrors — reduced to **p2** when the chiral
  pair is distinguished, which is the group shipped. The 2-fold center
  is the midpoint of the prototile's long side (vertex M of the L0
  cell); the C2 invariance is tested on generated patches.
- **Seeds:** 0 = CCW class {L0, L1, Y1, Y0}, 1 = its mirror
  {G0, G1, D0, D1}; the 2-coloring of pattern mode is exactly the
  chiral pair.
- **Verify:** standard trio (coverage, rotated-scaled, deterministic) +
  unit-structure test (exact side/angle/area/determinant identities),
  chirality-seed test, C2-rotation invariance test, and the
  brute-force culling cross-check.
- **Status:** committed (M49). 39 tilings, 218 tests at the commit.

### 16. Pentagon type 14 (Stein) ✅ (M50)

- **Motif:** Stein's type-14 convex pentagonal tiling (1985): the
  completely determined shape (no free parameters) with side
  constraints 2a = 2c = d = e (i.e. a = c = 1, d = e = 2) and
  b = βa, β = √((11√57−25)/8); angles A = 90°, B ≈ 145.34°,
  C ≈ 69.32°, D ≈ 124.66°, E ≈ 110.68° (2B + C = 360°,
  C + E = 180°). 3-isohedral: three isohedral classes, six tiles
  per primitive unit (Wikipedia: "each primitive unit contains six
  tiles", group p2 / orbifold 2222).
- **Geometry:** exact surds, a = c = 1: β = √((11√57−25)/8)
  (side b), q = (√57−3)/8 = sin B, p = √((6√57−2)/64) =
  sin(B − 90°); βp = (43−√57)/16 lies in Q(√57) exactly,
  βq = √((438√57−2706)/256); tile area β(3√57−1)/16 +
  p(√57−5)/4. Not edge-to-edge: each long side (length 2) is drawn
  as two collinear unit fine edges, giving 7-vertex D-class cells,
  5-vertex M-class cells and 6-vertex salmon cells.
- **Unit cell:** 6 tiles on the oblique lattice
  w1 = (2pq, −(13+9√57)/16), w2 = (−(β+2p), −(√57−1)/2)
  (|det| = 6 × tile area, w1 · w2 ≠ 0). All coordinates are exact
  surds in Q(√57, β, p, bq) — no decimal approximations shipped.
  The table was extracted from a verified image-propagation of the
  Wikipedia 18-tile patch (14 edge gluings, BFS growth to a 15-tile
  exact structure) and verified before porting: 21600-point
  exactly-once overlay (40×40 grid + 20000 random points over a
  24×24 window with ±5×±6 lattice translations), surd identities
  to ≤2.2e-16.
- **Group:** p2 (orbifold 2222). The 2-fold center is
  O = (β + p/2, q/2) — the midpoint joining the A-corners of the
  D0/D180 pair; the half-turn maps D0 ↔ D180, M0 ↔ M180 and
  MTHS ↔ MTHSp180 (shifted by a w2 translation), so each seed
  class maps onto itself and the group stays p2 with the classes
  distinguished. C2 invariance is tested on generated patches.
- **Seeds:** 0 = D class {D0, D180}, 1 = M class {M0, M180},
  2 = salmon class {MTHS, MTHSp180}; pattern mode assigns the three
  classes distinct colors of the 8-color palette (seed % 8).
- **Verify:** standard trio (coverage, rotated-scaled, deterministic) +
  unit-structure test (per-class exact side/angle/area/determinant
  fingerprints), isohedral-class test (per-tile vertex counts and
  side multisets), C2-rotation invariance test, and the brute-force
  culling cross-check.
- **Status:** committed (M50). 52 tilings, 332 tests at the commit.

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

### Aperiodic substitution (each needs rule derivation first)

- 🔭 **Socolar 12-fold shield tiling** — shield hexagons + rhombi +
  squares; hard (rule geometry).
- 🔭 **Ammann sets A1–A4** — the rest of Ammann's aperiodic families;
  moderate each once the AB machinery exists.

### Finite verified patches (patch-style entries)

- 🔭 **Long/short girih bands (10-fold)** — the other periodic girih
  band orientations complementing shesh-band.
- 🔭 **Full girih decagon patch** — the classic decagon-and-bowtie
  strapwork region with hexagon/bowtie/rhombus infill (Lu–Steinhardt
  figures), as a documentary overlay on the substitution entry.
- 🔭 **Penrose sun/star/cartwheel finite patches** — easy given P3
  machinery; mostly of documentary value.

### Research (no construction path chosen yet)

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
2. **Automated dual construction** — this arc dualizes 7 tables by
   hand-transcribed Python. A `dual(primal: LatticeTiling) -> Option<LatticeTiling>`
   helper (centroid dual, when all faces are regular) would make
   k-uniform duals cheap and remove transcription risk.
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

## Changelog of this file

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
  (52 tilings, 332 tests).
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
