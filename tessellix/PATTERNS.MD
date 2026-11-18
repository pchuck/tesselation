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

## Implemented (69 tilings)

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

### Laves / Catalan duals — complete: 8 of 8

The whole Catalan family now mirrors the closed Archimedean set:
every dual is the face-centroid dual of an exact primal table.

| id | face config | group | primal | notes |
|---|---|---|---|---|
| `cairo-pentagonal` | Cairo Pentagonal (V3.3.4.3.4) | p4g | snub-square | tangential pentagons; minimum-perimeter pentagonal tiling; the collinear-vertex variant (143.13/108.43/108.43/90/90°, edges 4×√10 + 4, integral vertices, square 72-area cell) is documented in the card description (M76) |
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
| `girih-shesh-band` | girih strapwork band | p2 | single shesh-band hexagon, centered rectangular lattice (M76: kind corrected from oblique) |
| `girih-star-cross` | 10-pointed girih star-and-cross strapwork (periodic) | cmm | measured classical panel; 545-tile cell data table on the (77,45)/(0,90) centered-rectangular basis (M35; M76: group pmm → cmm, lattice kind → centered rectangular) |
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
| `herringbone` | 2:1 dominoes in chiral L-pairs | pgg | rectangular lattice ⟨(2,2),(1,−1)⟩ (M76), 2 split dominoes per cell; single-direction (chiral): 16 2-fold centers per 4×4 period, two perpendicular glide families (diagonal + anti-diagonal axes), no mirrors; exact coset group probe (M40) |
| `running-bond` | 2:1 dominoes in stretcher rows, half-brick offset | cmm | centered rectangular lattice ⟨(2,0),(1,1)⟩ (M76; the measured lattice is the √2-degenerate square lattice — the legality test's documented exception), 1 split domino per cell; mirrors at the integer x-axes and half-integer y-axes, 2-folds at (k, l+1/2) and (k+1/2, l), glides in both directions; exact coset group probe (M41) |
| `kuniform-p6m` | 2-uniform (3⁶; 3².6²) | p6m | hexagonal tiling with one of every three hexagons (index-3 sublattice m ≡ n mod 3 of the honeycomb centers) dissected into six-triangle fans; Wikipedia `2-uniform n10`, t = 2, e = 3 |
| `kuniform-pmm` | 2-uniform (3.6.3.6; 3².6²) | pmm | hexagonal tiling with rows of up- and down-pointing triangles filling the gaps between rows of hexagons (rectangular lattice, hexagons share full horizontal edges); Wikipedia `2-uniform n11`, t = 2, e = 3 |
| `pentagon-13` | Rice pentagon, type 13 (1977, A = 120°) | p2 | 2-isohedral chiral pair (seeds 0/1), 8 tiles per rectangular cell, exact Q(√3); long side split into two collinear a-edges (M49) |
| `pentagon-14` | Stein pentagon, type 14 (completely determined) | p2 | 3-isohedral (seeds 0/1/2), 6 tiles per oblique cell, exact Q(√57, β) with β = √((11√57−25)/8); long sides split into two collinear unit fine edges — 7/5/6-vertex cells (M50) |
| `pentagon-15` | Mann pentagon, type 15 (MMVD 2015) | pgg | 3-isohedral (seeds 0/1/2), 12 tiles per rectangular cell = 4 congruent 3-tile blocks, exact Q(√3) (a = 2, b = d = e = 1, c = √(2+√3), all vertices on the (p+q√3)/2 grid); non-edge-to-edge — 4 T-split edges per cell (M55) |
| `pentagon-9` | Rice pentagon, type 9 (Rice 1977) | pgg | 2-isohedral (seeds 0/1), 8 tiles per rectangular cell, exact Q(√3) (A = C = 120°, cos B = 1/7, cos D = 23/98, cos E = −11/14, a = 6, b = c = d = e = 14; vertices on the (Z+Z√3)/7 grid); edge-to-edge, vertex stars {A,A,C}, {D,E,E}, {B,B,C,D}; two perpendicular glide families squaring to the primitive translations, no mirrors (M59) |
| `pentagon-11` | Rice pentagon, type 11 (Rice 1977) | pgg | 2-isohedral (seeds 0/1, each orbit mixing both chiralities), 8 tiles per rectangular cell, runtime-computed prototile with bounded-precision floats (A = 90°, B = 156°, C = 48°, D = 114°, E = 132°, a = r = 0.0946641…, b = s = 1.0568030…, c = 1, d = e = 1 + 2r — the family conditions 2a + c = d = e, 2B + C = 360°, C + E = 180° hold to ~1e-12); non-edge-to-edge — every d-side carries one c-side and two a-sides; vertex stars {C,B,B}, {A,A,180°}, {D,D,E}, {C,E,180°}; two perpendicular glide families squaring to w1 and −w2, no mirrors (M60) |
| `pentagon-12` | Rice pentagon, type 12 (Rice 1977) | pgg | 2-isohedral (seeds 0/1: {0,3,4,7} / {1,2,5,6}, each orbit mixing both chiralities), 8 tiles per rectangular cell, runtime-computed prototile closed to exact Q(√3) surds at the shipped B = 150° member (A = 90°, B = 150°, C = 60°, D = E = 120°, a = 1, b = √3, c = 1, d = 2, e = 1 — the family conditions 2a = d = c + e, 2B + C = 360° hold to ~1e-12); non-edge-to-edge — every d-side splits at its exact midpoint into two unit halves, all other sides full-side matched (b↔b, c↔c, e↔e); vertex stars {90°,90°,180°}, {60°,120°,180°}, {120°,120°,120°}, {60°,150°,150°}; two perpendicular glide families squaring to w1 and −w2, no mirrors (M62) |
| `pentagon-10` | James pentagon, type 10 (James 1975) | cmm | 3-isohedral (seeds 0/1/2: {0,2}, {1,3}, {4,5}), 6 tiles per centered-rectangular cell w1 = (3,3), w2 = (−2,5), det 21 = 6 × area (7/2) (M76 basis; the M63 cell basis (5,−2)/(8,1) spans the same lattice); exact at the shipped B = 90° member (A = B = E = 90°, C = D = 135°, a = b = 2, c = e = 1, d = √2; family a = b = c + e, A = 90°, B + E = 180° with d² = c² + e²); non-edge-to-edge — both length-2 sides split at their exact midpoints into two unit halves (integer T-junctions), all other sides full-side matched (c↔c, d↔d, e↔e); vertex figures {90°,135°,135°} 71×, {90°,90°,180°} 50×, {90°,90°,90°,90°} 13× (134 interior vertices); group cmm by exact audit — mirrors x+y = 3k and x−y = 7k, half-turn at the origin, genuine glide g = T(5,−2)∘R2 with g² = (7,−7) (axis x+y = 3/2, no pure-reflection conjugate), R90 negative; centered rectangular lattice (index-2 sublattice Z(3,3)+Z(−7,7) with centre (−2,5) ∈ L∖L', no orthogonal Z-basis for L) — cmm, not pmm (M63) |
| `pentagon-3` | Reinhardt pentagon, type 3 (family 3, 1918) | p2 | 1-isohedral (single orbit), 2 tiles per oblique cell, exact Q(√3) (pinned integral member (a,b,c,d,e) = (2,2,1,4,3): A = B = C = D = 120°, E = 60° — a = b, d = c + e, B + E = 180°; area 5√3); non-edge-to-edge — only the d = 4 sides split, at their exact midpoints between two length-2 (a/b) partner sides, all other sides full-side matched; vertex stars {60°,120°,180°} (A/E-type) and {120°,120°,120°} (B/C/D-type); chiral prototile, one chirality used — the classical three-tile type-3 pinwheel admits no periodic cell for this member (complete det-15 closure search + norm-form x²+3y²=120), so the shipped structure is the discovered minimal 2-tile p2 cell, four half-turn classes d/2 + Λ/2, no mirrors or glides (M64) |
| `pentagon-6` | Kershner pentagon, type 6 (1968) | p2 | 2-isohedral (seeds 0/1, orbits {0,1}/{2,3}), 4 tiles per oblique cell w1 = (1, 2√3), w2 = (−3, √3) (det = 7√3 = 4 × area 7√3/4), exact Q(√3) at the unique 30°-grid-pinnable member (A = B = D = E = 120°, C = 60°, a = d = e = 1, b = c = 2; A = 2C, C + E = 180°); edge-to-edge — all 20 stored sides exactly once reverse-glued under an involution (4 in-unit pairs + 6 across ±w1, ±w2, ±(w1−w2); census a-a:1, a-d:2, b-b:1, b-c:2, c-c:1, d-e:2, e-e:1), 20 directed sides → 18 torus classes with one double-covered arc; vertex stars {60°,60°,120°,120°} 340 + {120°,120°,120°} 682 in the 787-tile window (1022 of 1286 interior vertices, all 360°); p2: half-turns one coset c0 + Λ/2 (c0 = (1/2, 0)), no mirrors — 22-ray search, 0 genuine, no candidate direction normalizes Λ; achiral prototile congruent to the floret-pentagonal tile (one pentagon, two tilings: p2 2-iso vs floret p6 1-iso) (M74) |
| `pentagon-8` | Kershner pentagon, type 8 (1968) | pgg | 2-isohedral (seeds 0/1, orbits {0,3,4,7}/{1,2,5,6}, each orbit mixing both chiralities), 8 tiles per rectangular cell w1 = (36, 24√3), w2 = (−36, 18√3) (w1 ⊥ w2, det = 1512√3 = 8 × area 189√3), exact Q(√3) at the shipped B = C = 120° member (tan A = 3√3, cos D = 1/7, tan E = −√3/2, A = E − 60°, a = 6√7, b = c = d = e = 14; vertices on the Z[√3] grid); edge-to-edge — all 20 torus side pairs reverse-glued (a-a:4, b-c:4, b-d:4, c-c:2, d-e:4, e-e:2); vertex stars {A,A,C,D} ×30, {B,B,C} ×36, {D,E,E} ×25 (all sum 360°); two perpendicular glide families squaring to w1 and −w2, no mirrors (M77) |

### Aperiodic / nonperiodic

| id | motif | notes |
|---|---|---|
| `penrose` | Penrose P3 (thick/thin rhombi) | Robinson-triangle substitution, φ-inflation, chirality-aware; sun-seeded deflation growth |
| `penrose-p1` | Penrose P1 (pentagon, star, boat, diamond) | exact φ² deflation: short-edge (1/φ) pieces merge across parent boundaries (G+G→D, CORE+2G→B, S2C+2G→S, S1C+G→S); single-seed pentagonal gasket with 5-fold notch channels; clean-disk radius R/φ² guarantees view coverage |
| `penrose_p2` | Penrose P2 (kites & darts) | glue-labelled half-tile substitution (A → 2A+O, O → A+O); halves pair along glue legs into kites/darts; sun/star seeds; unique labelling found by exhaustive search |
| `penrose-pentagon` | five pentagons (φ-substitution) | aperiodic set found by exhaustive pool search (strict-0 + 1-hole layers); M = [[0,1,0,1,0],[1,1,0,0,1],[2,0,0,0,3],[0,0,1,0,0],[1,1,0,0,1]] in order (50,105,451,468,484); single-seed deflation growth; non-edge-to-edge (φ-substitution children meet parent edges at t = 1/φ) |
| `pinwheel` | 1–2–√5 right triangle | Conway–Radin; √5 inflation + arctan(1/2) twist; infinitely many tile orientations; view-independent octave-anchored patches |
| `chair` | Chair (L-tromino) | λ = 2 rep-4 substitution; the inflated chair's dissection is unique (exhaustive enumeration) so the rule is forced; non-edge-to-edge (T-junctions at every scale); limit-periodic; 4×3 seed blocks, one chair per orientation; octave-anchored like pinwheel |
| `sphinx` | Sphinx (pentagonal hexiamond) | λ = 2 rep-4 substitution on the triangular lattice; of 12 hexiamonds only the 3×1 rhombus and the sphinx are rep-4, and the sphinx's dissection is unique (rule forced); 12 orientation classes (dissection mixes chiralities), doubly stochastic substitution matrix; non-edge-to-edge; 2×3 rhombus seed (exactly 2 sphinxes, unique); octave-anchored like chair |
| `p-pentomino` | P-pentomino (rep-4 reptile) | λ = 2 rep-4 substitution; exactly two dissections of the inflated 20-cell P (exhaustive enumeration, class multisets {0,1,4,6} and {3,4,4,5}), canonical {0,1,4,6} used; 8 chiral orientation classes, doubly stochastic substitution matrix (rows/columns sum to 4); non-edge-to-edge (5 T-junctions per parent); 5×8 seed block, one tile per class (62 such tilings of the rectangle); octave-anchored like chair (M47) |
| `ammann-beenker` | Ammann–Beenker A5 (octagonal aperiodic) | half-tile substitution (two rhombus halves + square) with per-level mirror flips; exact Q(√2) geometry; edge-to-edge; 8-fold finite patches |
| `stampfli` | Stampfli 12-fold dodecagonal | squares + equilateral triangles + 30° rhombi; Stampfli substitution, inflation 2+√3 (Bielefeld encyclopedia rule); straddling unit children deduped by exact vertex set; 12-rhombus seed star, exact Z[√3]/4 anchors; edge-to-edge, T-junction-free, 12-fold symmetric at every generation |
| `shield` | Shield tiling (Gähler, dodecagonal 12-fold) | shield hexagon + square + black/white triangles, each in two labelled orientations R/L — 4 prototiles / 8 substitution kinds carrying Gähler's matching rules; inflation θ = 2cos15° = (√6+√2)/2, PF Perron θ² = 2+√3; children tile kinked supertiles (±¼-unit-square notches); exact Q(√2,√3) geometry, all edges on the 12 dodecagonal directions; Bragg 12-fold (no local 30° center) |
| `hat` | Hat (aperiodic monotile, "einstein") | SMKG 2023; H/T/P/F metatile substitution (29-rule patch, 22 kept), exact Eisenstein placements; inflation φ² only in the limit (iterated outlines drift by design); tiles only with its mirror image; per-view seeded lineage blobs with exact clipped-area acceptance; non-edge-to-edge (565 midpoint T-junctions at depth 3) |
| `turtle` | Turtle ("hats in turtles", einstein family) | turtle (14 sides, area 10√3) + hat prototiles on Kaplan's exact hex lattice; 9-label 8+1 key-point substitution with per-level chirality flip; homochiral (all tiles unreflected — the plain turtle tiling mixes reflected copies), edge-to-edge; turtle:hat ratio → 4+√15 |
| `spectre` | Spectre (chiral einstein Tile(1,1)) | single equal-edge prototile, aperiodic *without* reflections; 9-label D12 substitution (spectre.js-identical), exact Q(√3) surd geometry; edge-to-edge |
| `hirschhorn` | Hirschhorn–Hunt pentagon (140°/60°/160°/80°/100°) | not locally forced; C6-symmetric constrained-growth realization |
| `girih-quasiperiodic` | girih star-and-cross {10/3} (quasiperiodic) | quasiperiodic; Darb-i Imam 3-prototile substitution (Lu & Steinhardt 2007, IF = 2φ³); exact 10-fold seed rosette, edge-to-edge at every generation |

## Gallery taxonomy (M76)

M76 restructured the browse gallery into ten tabs
(`TilingCategory::ALL`): **Uniform** (11) · **Two-Uniform** (2) ·
**Laves (Catalan duals)** (11) · **Wallpaper** (15) · **Pentagon
Types** (12) · **Islamic** (9) · **Arrangements** (5) ·
**Quasiperiodic** (8) · **Aperiodic** (4) · **Nonperiodic** (4) —
81 memberships over the 69 implemented tilings; the 12 extra are
secondary memberships. (Counts at this commit; the block pre-dated
M74/M77/M78/M79, was re-synced in M81, and again in M83.)

- **Multi-tab membership.** `Tiling::secondary_categories()` (default
  `&[]`) lets a tiling appear in additional tabs: `triangle`,
  `square` and `hexagon` also appear under Laves (self-dual regular
  patterns), and fifteen tilings appear under the Wallpaper catalog.
  `browse_tabs` appends secondary categories to each card's tabs and
  sorts each tab by `section_key`, which now guards on the *tab's*
  category, not the card's (a Wallpaper tab is all-secondary).
  Both UIs (ratatui + egui) pin the ten-tab layout, and the headless
  egui geometry test pins the new card budget (2-line card titles,
  no clipping at 1200×900, 640×800 and 2560×900).
- **Wallpaper catalog.** The Wallpaper tab is a group catalog, not a
  motif list: one card per realized wallpaper group — 15 of the 17:
  p1 → stepped-hexominoes, p2 → pentagon-3, p3 → chiral-cut-honeycomb,
  pg → glide-hexagon-strips,
  pm → trapezoid-courses, cm → wavy-courses,
  pmg → shifted-trapezoid-courses,
  pmm → kuniform-pmm, pgg → herringbone, p4 → windmill,
  p4m → truncated-square, p4g → snub-square, p6 →
  snub-trihexagonal, p6m → hexagon, cmm →
  elongated-triangular. The two un-realized groups —
  p3m1, p31m —
  are named in a shared gap note
  (`tess_core::browse::wallpaper_gap_note`) rendered at the top of
  the tab in both UIs; no placeholder cards. (Historic bug fixed in
  M76: the 17th group is **pmg**, not cmc — Cmc is a 3D space group;
  the misnamed `WallpaperGroup::Cmc` variant is now `Pmg`.)
- **LatticeKind::CenteredRectangular.** A fifth lattice kind. The
  legality test
  (`declared_symmetry_is_legal_and_matches_the_measured_lattice`)
  pins each declared (group, lattice kind) against the 17-group
  table and re-measures the lattice from the declared basis vectors
  alone (hexagonal probe → orthogonal unimodular pair →
  centered-rectangular probe; tol 1e-6), never calling `generate()`.
  Re-declarations: pentagon-10 → centered, basis (3,3)/(−2,5);
  running-bond → centered, basis (2,0)/(1,1) 1-domino cell — the
  only declared-c-rect whose measured lattice is a √2-degenerate
  square lattice, the test's documented exception (point group 2mm,
  no 4-fold); girih-shesh-band → centered (p2); girih-star-cross →
  centered with group corrected pmm → cmm (the (77,45)/(0,90) basis
  is a primitive cell of the centered rectangular lattice);
  elongated-triangular and prismatic-pentagonal → centered;
  herringbone → rectangular, basis normalized to (2,2)/(1,−1).
- **Cairo merge.** `cairo-collinear` was the same Laves pattern
  (V3.3.4.3.4, dual of snub-square) as `cairo-pentagonal` with a
  different pentagon shape, so it was dropped as a separate card;
  `cairo-pentagonal` is now named "Cairo Pentagonal (V3.3.4.3.4)"
  and its description documents the collinear-vertex variant
  (143.13/108.43/108.43/90/90°, edges 4×√10 + 4, integral vertices,
  square 72-area cell). 62 → 61 tilings.

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
## Follow-up: convex pentagon census — 🚧 in progress (M49+; M49, M50, M55, M59, M60, M62, M63, M64, M65, M72, M74, M77 landed)

The PROPOSED.md #5 arc (all 15 convex pentagonal tiling types). Type 13
(Rice) lands first: its 8-tile primitive unit is small enough for a
plain data table; type 14 (Stein) second: a 6-tile unit, but with the
census's first exact-Q(√57) surd field; type 15 (Mann/McLoud-Mann/
Von Derau 2015) third: a 12-tile rectangular unit in exact Q(√3) —
the arc's first pgg — with two perpendicular glide families; type 9
(Rice 1977) fourth: an 8-tile rectangular unit in exact Q(√3), the
arc's second pgg and its first edge-to-edge pentagon-level Rice type;
type 11 (Rice 1977) fifth: the arc's first non-surd instance — a
bounded-precision 8-tile rectangular unit pinned to the reference
animation's B = 156° member of Rice's family, and the arc's first
non-edge-to-edge pgg; type 12 (Rice 1977) sixth: the arc's first
surd-pinnable Rice instance — the B = 150° member of the
2a = d = c + e family closes to exact Q(√3) (a = 1, b = √3, c = 1,
d = 2, e = 1), 8-tile rectangular unit, group pgg, non-edge-to-edge
with every d-side bisected by a T-junction; type 10 (James 1975)
seventh: the arc's first cmm and first centered-rectangular lattice —
the B = 90° member of the a = b = c + e, d² = c² + e² family
(isosceles, chiralities coincide as vertex sets), 6-tile oblique unit
with det 21, non-edge-to-edge with both length-2 sides bisected; cmm
proven by exact audit, mirrors and a half-turn ruling out p2 and the
centered lattice L' = Z(3,3)+Z(−7,7) (centre (−2,5) ∈ L∖L') ruling
out pmm; type 3 (Reinhardt family 3) eighth: the pinned integral
member (2, 2, 1, 4, 3) — the arc's first 2-tile unit and its first
chiral one-orbit p2, after a complete search proved the classical
three-tile 120° pinwheel admits no periodic cell for this member;
type 2 (Reinhardt family 2) ninth: the c = e, d = b, B + D = 180°
edge-to-edge variation, pinned to the exact-Q(√3) member a = 1
(A = B = D = 90°, C = 120°, E = 150°, b = d = 1 + √3/2,
c = e = 1/2) — the arc's second pentagon-level edge-to-edge
(after type 9) and its first two-chiral-pair unit: a 4-tile
rectangular cell whose glides map each tile onto its mirror image,
group pgg (colouring the two chiralities drops the group to p2),
vertex figures {90°, 120°, 150°} and {90°, 90°, 90°, 90°};
type 7 (Kershner 1968) tenth: the arc's first non-surd edge-to-edge
pgg — published E = 106° member (A ≈ 58.86°, B = 148°,
C ≈ 132.86°, D ≈ 94.27°, a ≈ 1.0357942, b = c = d = e = 1/2),
runtime-computed prototile (C = root in (120°,180°) of the applet's
transcendental closure equation; not surd-pinnable); 8-tile rectangular
cell, four direct + four mirror, 2-isohedral (seeds 0/1, orbits
{0,3,4,7}/{1,2,5,6}), pgg (g1² = w1, g2² = −w2, half-turns c0 =
(1/4,0) + Λ/2), edge-to-edge, vertex stars {106°,106°,148°},
{94.27°,132.86°,132.86°}, {58.86°,58.86°,94.27°,148°} — closing the
M58 deferral (grid-pinning artifact + two search-phase accounting
bugs; see PROPOSED.md #5); type 8 (Kershner 1968) eleventh: the
exact-Q(√3) B = C = 120° member of the b = c = d = e,
2B + C = D + 2E = 360° family (tan A = 3√3, cos D = 1/7,
tan E = −√3/2, A = E − 60°, a = 6√7, b = c = d = e = 14, tile area
189√3); 8-tile rectangular cell w1 = (36, 24√3), w2 = (−36, 18√3)
(w1 ⊥ w2, det = 1512√3 = 8 × area), four direct + four mirror,
2-isohedral (seeds 0/1, orbits {0,3,4,7}/{1,2,5,6}, each orbit mixing
both chiralities), pgg (g1² = w1, g2² = −w2, half-turn centre
c0 = (−1/2, 25√3/2)), edge-to-edge, vertex stars
{A,A,C,D} (79.1°, 79.1°, 120°, 81.8°) ×30, {B,B,C} (3 × 120°) ×36,
{D,E,E} (81.8°, 139.1°, 139.1°) ×25 — the arc's second Kershner;
type 6 (Kershner 1968) twelfth: the a = d = e, b = c, A = 2C,
C + E = 180° family's unique 30°-grid-pinnable member (A = B = D =
E = 120°, C = 60°, a = d = e = 1, b = c = 2, area 7√3/4); 4-tile
oblique cell w1 = (1, 2√3), w2 = (−3, √3) (det = 7√3 = 4 × area),
2-isohedral (seeds 0/1, orbits {T0,T1}/{T2,T3}); p2 (half-turns
one coset c0 + Λ/2, c0 = (1/2, 0); 22-ray mirror search 0 genuine,
no edge direction normalizes Λ); edge-to-edge — the 20 sides
reverse-glue pairwise (4 in-unit + 6 shifted pairs), 18 directed
torus classes with one double-covered arc, vertex stars
{60°,60°,120°,120°} + {120°,120°,120°}; the prototile is achiral
and congruent to the floret-pentagonal tile — one pentagon, two
tilings (p2 2-isohedral vs the floret's p6 1-isohedral).

Type 7 is implemented (M72) — see §25, type 8 (M77) — see §26, and
type 6 (M74) — see §27;
the M58 bounded negative is superseded (grid-pinning artifact + two
search-phase accounting bugs: the published instance is off-grid, so
the grid search could never have found it). Remaining in the arc:
types 1 and 5 covered by existing registry entries (coverage audit
documentation pending).

### 15. Pentagon type 13 (Rice 1977) ✅ (M49)

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
- **Status:** committed (M49). 48 tilings, 297 tests at the commit.

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
- **Status:** committed (M50). 51 tilings, 332 tests at the commit.

### 17. Pentagon type 15 (Mann/McLoud-Mann/Von Derau 2015) ✅ (M55)

- **Motif:** the last-discovered convex pentagonal tiling type (MMVD
  2015): the fixed-shape prototile with a = c = e, b = 2a,
  d = a√2/(√3−1) = a√(2+√3) (the dossier row's (a+√2)/(√3−1) was a
  typo) and angles A = 150°, B = 60°, C = 135°, D = 105°, E = 90°
  (A + E = C + D = 240°). 3-isohedral: three isohedral classes under
  the full pattern group (each a four of tiles), twelve tiles per
  primitive unit.
- **Geometry:** shipped at 2× the paper scale with the cyclic
  relabelling (shipped A = paper E, …): a = 2, b = d = e = 1,
  c = √(2+√3), tile area (5+3√3)/4. Exact Q(√3) — every cell vertex
  lies on the (p + q√3)/2 grid; no decimal approximations shipped.
  Not edge-to-edge at the pentagon level: a side of length 2 can meet
  two collinear unit sides of a neighbour (T-junction); 4 of the 60
  cell edges are T-split, the other 56 are full matches.
- **Unit cell:** 12 tiles on the rectangular lattice
  w1 = (−(1+√3)/2, (3+√3)/2), w2 = ((9+6√3)/2, 3+3√3/2)
  (w1 ⊥ w2, |w1|² = 4+2√3, |w2|² = 63+36√3,
  |det| = 12 × tile area = 15+9√3). The 12-tile cell decomposes into
  four congruent 3-tile blocks. Extracted from a verified exact-Q(√3)
  forced construction and checked before porting: zero overlaps,
  exactly-once coverage on a 20400-point sample, all 60 central
  edges exactly glued, and an exhaustive symmetry audit.
- **Group:** **pgg** (confirmed by the exhaustive exact audit —
  25 translations + 25 half-turns + 50 glides + 0 mirrors in a 5×5
  window, exactly 100 symmetries). Two perpendicular glide families:
  g1² = w1 (axes ∥ w1) and g2² = w2 (axes ∥ w2), each
  chirality-flipping; no mirrors (the reflection-free test asserts
  no (M, t) with M a reflection maps a tile onto the tiling). The
  2-fold centers form the coset c0 + (1/2)L with c0 = (√3, −1/2) —
  four per primitive cell, not on the tiling lattice L itself.
  Seeds 0/1/2 ship the three full-group isohedral classes
  {T1,T4,T7,T10} / {T2,T5,T8,T11} / {T3,T6,T9,T12}; restricted to the
  p2 translation + half-turn subgroup the six orbits are the six
  chirality-labeled pairs {T1,T7}, {T2,T8}, …, {T6,T12}.
- **Verify:** standard trio (coverage, rotated-scaled, deterministic)
  + unit-structure test (exact lattice facts, |det| = 12·area,
  per-tile side/angle fingerprints) + isohedral-class test + pgg
  symmetry test (exact half-turn patch equality; g1/g2 image-window
  containment; negative mirror probes) + window-census test
  (93/89/45 tiles at three fixed windows), plus the brute-force
  culling cross-check.
- **Status:** committed (M55). 54 tilings, 353 tests at the commit.

### 18. Pentagon type 9 (Rice 1977) ✅ (M59)

- **Motif:** Rice's type-9 convex pentagonal tiling — a
  two-parameter family with b = c = d = e and 2A + C = D + 2E = 360°.
  The shipped instance pins the exact-Q(√3) member A = C = 120°,
  cos B = 1/7 (B ≈ 81.79°), cos D = 23/98 (D ≈ 76.43°),
  cos E = −11/14 (E ≈ 141.79°), a = 6, b = c = d = e = 14 (tile
  area 141√3). 2-isohedral: two full-group orbits
  {T1,T4,T5,T8} / {T2,T3,T6,T7}, eight tiles per primitive unit.
- **Geometry:** edge-to-edge at the pentagon level (every side is
  matched by exactly one full side of one neighbour — the quotient
  census pairs equal lengths only: a↔a ×4, b↔c ×4, b↔d ×4, c↔c ×2,
  d↔e ×4, e↔e ×2, 40 directed rules). The three vertex stars are
  {A,A,C}, {D,E,E} and {B,B,C,D} — 120°+120°+120°,
  76.43°+2·141.79° and 2·81.79°+120°+76.43°, each exactly 360°.
  Every cell vertex lies on the (Z + Z√3)/7 grid; no decimal
  approximations shipped.
- **Unit cell:** 8 tiles on the rectangular lattice
  w1 = (−30, 6√3), w2 = (−141/7, −235√3/7)
  (w1 ⊥ w2, |w1|² = 1008, |w2|² = 185556/49,
  det = 1128√3 = 8 × tile area). Extracted from a verified
  exact-Q(√3) construction and checked before porting: zero
  overlaps, exactly-once coverage on an 81×71-point sample, complete
  full-edge gluing with the three closed vertex stars, and an exact
  symmetry audit of the quotient table.
- **Group:** **pgg**. The half-turn centres c0 = (13, 4√3) and its
  images; two perpendicular glide families
  g1 = (M_v, (167/7, −165√3/7)) with axis ∥ w2 and g1² = w2, and
  g2 = (M_u, (12, −8√3)) with axis ∥ w1 and g2² = −w1, where M_v is
  reflection about the w2 direction (fixing w2, negating w1) and
  M_u = −M_v about the w1 direction (all exact Q(√3)). No mirrors:
  every symmetry with a glide's linear part shifts by exactly half a
  primitive translation along its axis, so no conjugate is a pure
  reflection. Confirmed
  two ways: a gauge-invariant automorphism audit of the exact
  quotient table (perm + lattice gauge over Z), then exact geometric
  verification of the seeded generators on the reconstruction.
  Seeds 0/1 ship the two full-group isohedral orbits; the glides
  identify the two chiralities inside each orbit.
- **Verify:** standard trio (coverage, rotated-scaled, deterministic)
  + unit-structure test (exact lattice facts, det = 8·area, per-tile
  cyclic (side, angle) fingerprints, exact corner cosines) +
  edge-to-edge gluing census (61-tile patch: no duplicate directed
  side, every interior side full-matched, zero T-junctions) +
  vertex-star census (interior stars exactly {A,A,C}, {D,E,E},
  {B,B,C,D}, sums 2π) + isohedral-class test + pgg symmetry test
  (exact half-turn patch equality; g1/g2 image-window containment;
  g1² = w2, g2² = −w1; negative mirror probes) + window-census test
  (61/32/16/4 tiles at four fixed windows, cross-checked against an
  independent Python emulation of the lattice.rs culling), plus the
  brute-force culling cross-check.
- **Status:** committed (M59). 55 tilings, 362 tests at the commit.


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

### 20. Pentagon type 11 (Rice 1977) ✅ (M60)

- **Motif:** Rice's type-11 convex pentagonal tiling — the
  two-parameter family with 2a + c = d = e, A = 90°, 2B + C = 360°,
  C + E = 180° (so C = 360° − 2B, E = 2B − 180°, D = 270° − B;
  convex for 90° < B < 180°, side b the second free parameter).
  The shipped instance pins the reference construction's B = 156°
  member at scale c = 1: A = 90°, B = 156°, C = 48°, D = 114°,
  E = 132°, a = r = 0.0946641…, b = s = 1.0568030…,
  d = e = 1 + 2r = 1.1893282… (tile area 0.9542866…).  156° is
  not a surd angle — the arc's first bounded-precision instance:
  the prototile is computed at runtime from the closed forms
  r = (2 sin B − cos 2B)/(1 + 2 cos 2B − 2 sin B) and
  s = −2r cos B − (1 + 2r) sin 2B, and every closure identity is
  pinned by tests to ~1e-12.  2-isohedral: two full-group orbits
  {cell 0, 2, 4, 7} / {cell 1, 3, 5, 6}, eight tiles per primitive
  unit, each orbit mixing both chiralities (the glides identify
  them).
- **Geometry:** NON-edge-to-edge at the pentagon level — every long
  side d = e is covered by one partner c-side plus two partner
  a-sides, exactly the family condition 2a + c = d; every other
  side is matched by exactly one full side of one neighbour.  The
  refined census over 528 tiles: 1882 full edges + 210 d-sides
  split at the parameters c/d and (c + a)/d, every resulting
  sub-edge contained in exactly one partner edge, no other
  T-junctions.  The four vertex stars are {C,B,B} = 48° + 2·156°,
  {A,A,180°} = 2·90° + 180°, {D,D,E} = 2·114° + 132° and
  {C,E,180°} = 48° + 132° + 180° — each exactly 360°.
- **Unit cell:** 8 cells as (mirror, anchor, rotation) placements
  of the runtime prototile (vertex A, the unique 90° corner, is
  pinned by the anchor) on the rectangular lattice
  w1 = (−2.1433083997208975, −1.3918807493424277),
  w2 = (1.6269879260579216, −2.5053417038859127)
  (w1 ⊥ w2 to 1e-12, det = 7.63429309180… = 8 × tile area).
  Extracted from a verified construction and re-verified on the
  model before porting: zero overlaps, exactly-once coverage, the
  complete refined edge gluing with the four closed vertex figures,
  and an exact symmetry audit of the quotient table.
- **Group:** **pgg**, decided by anchor-pair symmetry enumeration
  on the exact model (the 90° corner A is symmetry-distinguished):
  exactly four classes mod lattice — identity, half-turn, and two
  orientation-reversing classes.  Half-turns centred at
  c0 = (−0.37157…, 0.76010…) and its images
  (c0 + {0, w1/2, w2/2, (w1 + w2)/2}); two perpendicular glide
  families g1 = (M1, anchor66) with axis ∥ w1 and g1² = w1, and
  g2 = (M2, anchor246) with axis ∥ w2 and g2² = −w2, where
  M_i = 2u_i u_iᵀ − I is reflection in the lattice direction; each
  glide shifts exactly half a primitive translation along its axis,
  so no conjugate of it is a pure reflection (no mirrors).  g2∘g1
  is the half-turn about c0 itself.
- **Verify:** standard trio (coverage, rotated-scaled,
  deterministic) + unit-structure test (closure E = (0, a), angle
  multiset {90°, 156°, 48°, 114°, 132°}, per-cell cyclic
  (side, angle) fingerprints forward/reversed, det = 8·area) +
  refined-edge census test (528-tile patch: full/split counts,
  split parameters up to orientation, one covering edge per
  sub-edge) + vertex-figure census (100 interior vertices:
  {C,B,B} 24×, {A,A,180°} 25×, {D,D,E} 25×, {C,E,180°} 26×, via
  corner- and through-edge rays merged at 1e-4 rad) +
  isohedral-class test (closed mod-lattice permutations of the
  three generators, two orbits = the two seeds) + pgg-symmetry test
  (interior-AABB-filtered exact half-turn patch equality — the
  boundary ring that AABB culling may include without true
  intersection is excluded; glide containment with lattice shifts
  and inverses; g1² = w1, g2² = −w2, centre(g2∘g1) = c0;
  mirror-absence probes) + window-census test (238/213/116 tiles at
  three fixed windows, cross-checked against an independent Python
  emulation of the lattice.rs culling), plus the brute-force
  culling cross-check.
- **Status:** committed (M60). 57 tilings, 371 tests at the commit.

### 21. Pentagon type 12 (Rice 1977) ✅ (M62)

- **Motif:** Rice's type-12 convex pentagonal tiling — the
  two-parameter family with 2a = d = c + e, A = 90°, 2B + C = 360°
  (so C = 360° − 2B, E = 2B − 180°, D = 270° − B; convex for
  90° < B < 180°, side b the second free parameter, side a running
  E→A).  The shipped instance pins the B = 150° member at scale
  c = 1 — a surd angle, so unlike type 11 the instance closes to
  exact Q(√3) surds: A = 90°, B = 150°, C = 60°, D = E = 120°,
  a = 1, b = √3, c = 1, d = 2, e = 1 (tile area 3√3/2, and
  8·area = 12√3 = the lattice determinant).  The prototile is
  computed at runtime from the closed forms
  a = (sin B + cos 2B)/(2 cos 2B − 2 sin B + 1),
  b = cos B − 2a·cos B − (2a − 1)·sin 2B, d = 2a, e = 2a − 1,
  and the surds a = 1, b = √3, d = 2, e = 1 are pinned by tests to
  1e-12.  2-isohedral: two full-group orbits {cell 0, 3, 4, 7}
  (seed 0) and {cell 1, 2, 5, 6} (seed 1), each orbit mixing both
  chiralities (the glides identify them), eight tiles per primitive
  unit.
- **Geometry:** NON-edge-to-edge at the pentagon level — the long
  side d = 2 is split at its exact midpoint t = 1/2 into two unit
  halves, each contained in exactly one unit partner side (a- or
  c-halves on one side, a- or e-halves on the other); every other
  side is matched by exactly one full side of one neighbour
  (b ↔ b, c ↔ c, e ↔ e).  The refined census over a 215-tile patch:
  620 full edges + 153 d-sides split at t = 1/2, every sub-edge
  contained in exactly one partner edge, no other T-junctions.  The
  four vertex figures are {90°, 90°, 180°} (8×), {60°, 120°, 180°}
  (9×), {120°, 120°, 120°} (8×) and {60°, 150°, 150°} (10×) — each
  360° with exactly three rays; the two straight-angle figures are
  the T-junctions where a d-side is bisected.
- **Unit cell:** 8 cells as (mirror, anchor, rotation) placements
  of the runtime prototile (vertex A, the unique 90° corner, is
  pinned by the anchor) on the rectangular lattice
  w1 = (−2.598076211353316, −4.5) = (−3√3/2, −9/2),
  w2 = (3.4641016151377544, −2.0) = (2√3, −2)
  (w1 ⊥ w2 to 1e-12, det = 12√3 = 8 × tile area, pinned to 1e-12).
  Extracted from a verified exact-Q(√3) construction and re-verified
  on the model before porting: zero overlaps, exactly-once
  coverage, the complete refined edge gluing with the four closed
  vertex figures, and an exact symmetry audit of the quotient table.
- **Group:** **pgg**, decided by the exact symmetry audit of the
  quotient table: two perpendicular glide families with axes along
  the lattice directions — g1 = (M1, (0, −3)) with g1² = w1 and
  g2 = (M2, (−√3, 1)) with g2² = −w2, where M_i is reflection in
  the i-th lattice direction — each glide shifts exactly half a
  primitive translation along its axis (g1 by +w1/2, g2 by −w2/2),
  so no power of either is a pure reflection (all 22 mirror
  candidates, the perpendicular-residual reflections and the whole
  lattice shifts, fail).  Half-turns are centred at
  c0 = (√3/4, 5/4) = (0.4330127018922193, 1.25) and its images;
  g2∘g1 is the half-turn about c0 itself.
- **Verify:** standard trio (coverage, rotated-scaled,
  deterministic) + unit-structure test (8 congruent copies of the
  runtime prototile: angles {90°, 150°, 60°, 120°, 120°}, side
  multiset {1, 1, 1, √3, 2} at 1e-12, closure E = (0, a), cyclic
  (side, angle) fingerprints forward/reversed, seed pattern
  [0,1,1,0,0,1,1,0], lattice literals, w1 ⊥ w2, det = 8·area =
  12√3) + refined-edge census test (215-tile patch: 620 full + 153
  split at t = 1/2, one covering edge per sub-edge; the midpoint
  window (−7.45, 13.55) × (−7.6, 11.4) is an exact count with a
  minimum boundary gap of 0.0888 from any counted midpoint,
  verified in Q(√3) — robust against the ~1e-15 float difference
  between the runtime prototile and the exact surds) +
  vertex-figure census (35 interior vertices: {90°, 90°, 180°} 8×,
  {60°, 120°, 180°} 9×, {120°, 120°, 120°} 8×, {60°, 150°, 150°}
  10×) + isohedral-class test (locked cell permutations of the
  generators, two orbits = the two seeds) + pgg-symmetry test
  (interior-AABB-filtered exact half-turn patch equality; glide
  containment with inverses and lattice shifts; g1² = w1,
  g2² = −w2, centre(g2∘g1) = c0; half-periods +1/2 and −1/2 pinned
  to 1e-6; 22 negative mirror probes) + window-census test (98/90/50
  tiles at three fixed windows, cross-checked against the
  independent Python emulation of the culling).
- **Status:** committed (M62). 58 tilings, 400 tests at the commit.

### 22. Pentagon type 10 (James 1975) ✅ (M63)

- **Motif:** James's type-10 convex pentagonal tiling — the
  two-parameter family with a = b = c + e, A = 90°, B + E = 180°
  (so E = 180° − B, C = 180° − B/2, D = 90° + B/2; convex for
  90° < B < 180°, e/c the second free parameter, side a running
  E→A).  The shipped instance pins the B = 90° member at scale
  c = 1 — exact, no surd angles: A = B = E = 90°, C = D = 135°,
  a = b = 2, c = e = 1, d = √2 (d² = c² + e² closes the right
  angle at corner C; tile area 7/2, and 6·area = 21 = the lattice
  determinant).  At B = 90° the prototile is isosceles — symmetric
  in the line y = x, with vertex A (where the two length-2 sides
  meet) fixed and the axis crossing the midpoint of side d — so the
  two chiralities coincide as vertex sets: an M-cell and the D-cell at
  the same anchor with matching rotation span exactly the same region.
  The mirrored M-prototile is nonetheless built explicitly (reflected
  in y = x, stored reversed to stay CCW) so the cell table records
  which chirality each tile carries.  3-isohedral: the three
  full-group orbits are {cell 0, cell 2} (seed 0), {cell 1, cell 3}
  (seed 1) and {cell 4, cell 5} (seed 2); six tiles per primitive unit.
- **Geometry:** NON-edge-to-edge at the pentagon level — the two
  length-2 sides a and b are each split at their exact midpoint (an
  integer T-junction) into two unit halves, each half matched by
  exactly one unit partner side; the c, d and e sides are matched by
  exactly one full side of one neighbour (c↔c, d↔d, e↔e).
  The refined census over a 164-tile patch: 518 full edges + 76 split
  at t = 1/2 (594 edges with midpoints in the count window), every
  sub-edge contained in exactly one partner edge, no other T-junctions.
  Interior vertex stars: 134 vertices = 71× {90°, 135°, 135°}
  + 50× {90°, 90°, 180°} (the T-junctions) + 13×
  {90°, 90°, 90°, 90°}; every figure sums to 360°.
- **Unit cell:** 6 cells as (mirror, anchor, rotation) placements of
  the runtime prototile (vertex A is pinned by the anchor): the four
  D-rotations 0°/90°/180°/270° at the origin, plus M90 at
  (1, 2) and M270 at (−1, −2), on the oblique lattice
  w1 = (5, −2), w2 = (8, 1) with |det(w1, w2)| = 21 = 6 × area.
  Extracted from a verified exact construction and re-verified on the
  model before porting: zero overlaps, exactly-once coverage, the
  complete refined edge gluing, and an exact symmetry audit of the
  quotient table.
- **Group:** **cmm**, decided by the exact symmetry audit: point
  group D2 = {I, −I, R1(y = x), R2(y = −x)} — all four realised
  exactly (104/104 interior-AABB-filtered patch equality); R90 is *not*
  a symmetry (R90(M90@(1,2)) lands on a region no placed tile
  occupies, while R90(D0) is placed).  Mirror lines: x + y = 3k (R2,
  including the origin axis y = −x) and x − y = 7k (R1, including
  y = x); the glide axes fill the families x + y = 3k/2 \ 3k and
  x − y = 7k/2.  The representative genuine glide g = T(5,−2)∘R2
  has axis x + y = 3/2 at perpendicular offset (3/4, 3/4) from the
  origin, g² = T_{(7,−7)} = 3w1 − w2 ∈ L, and no conjugate of
  g is a pure reflection.  Since w1 ∈ L, the symmetry coset of g is
  w1 + L = L: every (R, s) with s ∈ L is a symmetry.  True negatives
  (images verified unplaced on the 164-tile patch): the reflection in
  the glide axis itself, (R2, (3/2, 3/2)), (R1, (1, 1)) with
  (1, 1) ∉ L, and the R90 chiral pair.  The translation lattice
  L = Z(5,−2) + Z(8,1) = {x − 8y ≡ 0 (mod 21)} is **centered
  rectangular**: the index-2 sublattice L' = Z(3,3) + Z(−7,7) is a
  primitive rectangle (sides 3√2 × 7√2, det 42) whose centre
  (−2, 5) = ((3,3) + (−7,7))/2 lies in L \ L'; L itself has no
  orthogonal Z-basis (the minimum determinant over orthogonal integer
  pairs is 42, never 21).  That centered lattice is what separates cmm
  from pmm, and the mirrors (plus the half-turn) rule out p2.  The
  primitive basis L (det 21, w1·w2 = 38 ≠ 0) is registered as
  `Oblique`; the centered-rectangular structure lives in L'.
- **Verify:** standard trio (coverage, rotated-scaled, deterministic)
  + unit-structure test (6 cells: four D + two M, seed pattern
  [0,1,0,1,2,2], all angles 90°/135° exact, side multiset
  {1, 1, √2, 2, 2}, closure E = (0, 2) at 1e-12, cyclic (side, angle)
  fingerprints forward/reversed, lattice literals, w1·w2 = 38
  (oblique), det = 6·area = 21) + refined-edge census test (164-tile
  patch: 518 full + 76 split at t = 1/2, one covering edge per
  sub-edge; the midpoint window (−7.75, 13.75) × (−7.75, 11.75)
  is an exact count with a minimum boundary gap of 1/4) +
  vertex-figure census (134 interior vertices: {90°,135°,135°}
  71×, {90°,90°,180°} 50×, {90°,90°,90°,90°}
  13×) + isohedral-class test (locked cell permutations of the
  generators, three orbits = the three seeds; the 73-tile patch
  (−5,−4,9,8) shows all three seeds; side multiset {1, 1, √2, 2, 2})
  + cmm-symmetry test (interior-AABB-filtered exact patch equality for
  −I, R1, R2 at 104/104; glide containment for g and g⁻¹ on the
  164-tile patch, 0 missing; g² = 3w1 − w2 and the perpendicular
  part pinned to 1e-9; negative probes (R2, (1.5, 1.5)) and
  (R1, (1, 1)) unplaced, R90 chiral pair M90@(1,2) unplaced / D0
  placed) + window-census test (77/73/41 tiles at three fixed windows,
  cross-checked against the independent Python emulation of the
  culling).
- **Status:** committed (M63). 59 tilings, 409 tests at the commit.
### 23. Pentagon type 3 (Reinhardt family 3) ✅ (M64)

- **Motif:** Reinhardt's type-3 convex pentagonal tiling — the
  one-parameter family `a = b`, `d = c + e`, `A = C = D = 120°`,
  `B + E = 180°` — pinned to its integral member `(a, b, c, d, e) =
  (2, 2, 1, 4, 3)` with `A = B = C = D = 120°`, `E = 60°` (area 5√3,
  all vertices in Q(√3)).  The member is simultaneously type-1 under
  the relabelling (A′,B′,C′,D′,E′) = (D,E,A,B,C); the type-3 identity
  is what is registered.  The uncoloured group is **p2**, not the
  brief's assumed p3/p31m: the classical three-tile 120° pinwheel
  admits **no** periodic cell for this member — a complete closure
  search over every lattice of determinant 15 in unit-side coordinates
  (both chiralities, T-junctions admitted, ~1.7 × 10⁶ pairing trials)
  is empty, the hexagonal-lattice routes die on the norm-form test
  x² + 3y² = 120 (and its cell-6/9 scalings 240/360), and a parity
  lemma kills the R120 pinwheel boundary for every type-3 tile.  The
  shipped structure is the discovered minimal periodic cell: **2
  tiles** — the prototile plus its half-turn image about the midpoint
  c0 = (−3/4, 3√3/4) of the long side d — on the oblique lattice
  w1 = (1/2, 3√3/2), w2 = (−13/2, √3/2) (xy: (0.5, 2.598076211353316),
  (−6.5, 0.8660254037844386)), det = 10√3 = 2 × tile area.
- **Group:** **p2** (orbifold 2222) by structure + exact audit: the
  prototile is chiral (side cycle (2,2,1,4,3) differs from every
  rotation of its reversal (3,4,1,2,2)) and the tiling uses one
  chirality only, so no orientation-reversing isometry — mirror or
  glide — can preserve it (144 negative probes: 4 mirror directions ×
  9 candidate centres × 9 whole lattice shifts); realised tile
  orientations are {0°, 180°}, excluding 3- and 4-folds.  The
  half-turn centres are exactly the four classes d/2 + Λ/2 mod Λ
  (c0 = (−0.75, 1.299038105676658) and c0 + w1/2, c0 + w2/2, c0 +
  (w1+w2)/2); the lattice points and the other half-lattice points
  are not centres (negative probes).
- **Edge structure:** NON-edge-to-edge — only the d = 4 sides are
  split, each at its exact midpoint t = 1/2 between the ends of two
  length-2 partner sides (a- or b-sides); every other side is matched
  by exactly one full side of one neighbour.  The vertex figures are
  {60°, 120°, 180°} at the A-/E-type points and {120°, 120°, 120°} at
  the B-/C-/D-type points (per cell: 2 + 4; the B- and D-classes
  coincide mod Λ because D = B + w1).
- **Verify:** standard trio (coverage, rotated-scaled, deterministic)
  + unit-structure test (2 directly-congruent copies of the runtime
  prototile: angles {120°, 120°, 120°, 120°, 60°}, sides 2, 2, 1, 4,
  3 at 1e-12, closure E = (−3/2, 3√3/2), family identities a = b and
  d = c + e, seeds [0, 0], lattice literals, det = 2·area = 10√3,
  |w1|² = 7, |w2|² = 43, oblique) + refined-edge census test (60-tile
  patch: 172 full + 46 d-sides split at t = 1/2, one covering edge per
  sub-edge, no other T-junctions; the window (−9.4, 13.4) × (−7.4,
  9.4) is an exact count with minimum boundary margin 0.0388 from any
  counted midpoint, verified in exact Q(√3)) + vertex-figure census
  (85 interior vertices: {60°, 120°, 180°} 46×, {120°, 120°, 120°}
  39×) + p2-symmetry test (interior-AABB-filtered exact half-turn
  patch equality about c0; all four d/2 + Λ/2 half-turn classes with
  lattice shifts map the cell-0 tile onto tiles; lattice translations;
  negative probes for the non-centre half-turns and every
  orientation-reversing candidate) + isohedral test (half-turn permutes
  the cell table as (0 1) mod lattice — a single orbit, seed 0 only;
  generated patches keep the side multiset {1, 2, 2, 3, 4}) +
  window-census test (19/26/44/70 tiles at four fixed windows,
  cross-checked against the independent Python emulation of the
  culling).
- **Status:** committed (M64). 60 tilings, 409 tests at the commit.

### 24. Pentagon type 2 (Reinhardt family 2) ✅ (M65)

- **Motif:** Reinhardt's type-2 convex pentagonal tiling — the
  one-parameter family `c = e`, `d = b`, `B + D = 180°` — pinned to
  the exact-Q(√3) member `a = 1`: `A = B = D = 90°`, `C = 120°`,
  `E = 150°`, `b = d = 1 + √3/2`, `c = e = 1/2` (tile area
  `1 + √3/2`, every vertex in Q(√3)).  This is the family's
  **edge-to-edge** variation (the Wikipedia type-2 section's other
  isohedral pgg form is T-split).  The prototile is **chiral** —
  its (side, angle) cycle ((1+√3/2, 90°), (1/2, 120°),
  (1+√3/2, 90°), (1/2, 150°), (1, 90°)) is inequivalent to every
  cyclic rotation of its reversal — yet the tiling uses **both**
  chiralities: the primitive cell holds two chiral pairs (two
  direct + two mirror tiles).
- **Group:** **pgg** (orbifold 2\*22) by structure plus exact audit
  of the quotient table: two perpendicular glide families with axes
  along the lattice directions, `g1² = w1` and `g2² = −w2` — each a
  true half-translation glide (the along-axis shift is exactly
  half a primitive translation — the test verifies `2·t1·u1/|w1|
  = +1` and `2·t2·u2/|w2| = −1`, i.e. shifts of +|w1|/2 and −|w2|/2
  — so no power of either
  glide is a pure reflection) — composing to the half-turn about
  `c0 = (1 + √3/2, 0)`, the prototile's B vertex, **not** a lattice
  point.  The half-turn centres are exactly the four classes
  `c0 + Λ/2`.  No mirrors (22 negative candidates: the two
  perpendicular-part-only mirror classes plus every whole-lattice
  shift of them, i ∈ −2..2).  Colouring the two chiralities (direct
  vs mirror) turns each glide into a pure reflection of its colour
  class and drops the group to **p2** — the classical reference's
  note.
- **Lattice & cell:** **4 tiles** per rectangular cell, two chiral
  pairs.  `w1 = (1/2, 1 + √3/2)`, `w2 = (−2 − √3, 1)`, with
  `w1 ⊥ w2`, `|w1|² = 2 + √3`, `|w2|² = 8 + 4√3`,
  `det = 4 + 2√3 = 4 × tile area`.  The four cells: the prototile
  at (0, 0) (rotation 0°), its half-turn image at (2 + √3, 0)
  (rotation 180°), and the two mirror tiles at
  (1 + √3/2, 1/2) (rotation 150°) and ((3 + √3)/2, (1 + √3)/2)
  (rotation 330°).
- **Edge structure:** **EDGE-TO-EDGE** at the pentagon level — the
  arc's second (after type 9): every side is matched by exactly one
  full side of one neighbour, no T-junctions (refined edge census
  over the 276-tile block window: 1182 full edges, 0 split edges,
  minimum midpoint boundary margin 0.00144, an exact count robust
  against the ~1e-15 float noise of the runtime-computed prototile).
  The two vertex figures are {90°, 120°, 150°} (237) and
  {90°, 90°, 90°, 90°} (118) — 355 interior vertices over the
  281-tile block, every star complete (star-to-boundary margin
  0.067; the edge-through-ray pass contributes no rays because no
  edge passes through a vertex).
- **Verify:** standard trio (coverage, rotated-scaled,
  deterministic) + unit-structure test (4 congruent copies of the
  runtime prototile: angles {90°, 90°, 120°, 90°, 150°} at 1e-9,
  sides 1 + √3/2, 1/2, 1 + √3/2, 1/2, 1 at 1e-12, closure
  E = (0, 1), family identities c = e, d = b, B + D = 180°, seeds
  [0, 0, 0, 0], lattice literals, det = 4·area = 4 + 2√3,
  |w1|² = 2 + √3, |w2|² = 8 + 4√3, w1 ⊥ w2, both chiralities
  present) + window-census test (115 / 165 / 272 / 281 / 17 tiles
  at five fixed windows, cross-checked against the independent
  Python emulation of the culling) + refined-edge census (276-tile
  window: 1182 full + 0 split edges) + vertex-figure census (355
  interior vertices: {90°, 120°, 150°} 237×, {90°, 90°, 90°, 90°}
  118×) + pgg-symmetry test (exact half-turn about c0 on the
  interior-AABB patch; g1, g1⁻¹, g2, g2⁻¹ and four lattice-shifted
  glide variants map the prototile onto tiles; all four c0 + Λ/2
  half-turn classes with 9 lattice shifts; 10 lattice translations;
  generator identities g1² = w1, g2² = −w2, g2∘g1 = half-turn about
  c0; 22 negative mirror probes) + isohedral test (half-turn + the
  two glide classes permute the cell table as a single orbit
  r = (0 1)(2 3), g1 = (0 2)(1 3), g2 = (0 3)(1 2), the glides
  identifying the chiralities; every generated tile keeps the
  prototile side multiset {1/2, 1/2, 1, 1 + √3/2, 1 + √3/2} and
  seed 0).
- **Status:** committed (M65). 61 tilings, 465 tests at the commit.

### 25. Pentagon type 7 (Kershner 1968) ✅ (M72)

- **Motif:** Kershner's type-7 convex pentagonal tiling — the
  one-parameter family `b = c = d = e`, `B + 2E = 2C + D = 360°` —
  pinned to the published E = 106° member (Jaap Scherphuis Tiling
  Viewer, N5-07; the applet slider defaults to 110°): `A ≈ 58.86°`,
  `B = 148°`, `C ≈ 132.86°`, `D ≈ 94.27°`, `a ≈ 1.0357942`,
  `b = c = d = e = 1/2`.  The family's closure equation is
  transcendental in E, so the instance is **not surd-pinnable**: the
  prototile is computed at runtime, C as the unique root in
  (120°, 180°) of `F(t) = sin(E−2t) − sin(t+E) − sin(E−t) − sin(E)`
  (bisection to 1e-3, then ≤ 200 Newton steps), and `A = C − B/2`.
  The cell is 2-isohedral: four direct + four mirror tiles.  This
  closes the M58 deferral: the published instance is off the 15°/30°
  (and 22.5°) grids the bounded search was pinned to, so the search
  could never have found it.
- **Group:** **pgg** (orbifold 2\*22) by structure plus exact audit:
  two perpendicular glide families with axes along the lattice
  directions, `g1² = w1`, `g2² = −w2` — each a true half-translation
  glide (along-axis shift +|w1|/2 and −|w2|/2 respectively, verified
  to 1e-6, so no power of either glide is a pure reflection);
  `g2∘g1` is the half-turn about `c0 = (1/4, 0)`, and the half-turn
  centres are exactly the four classes `c0 + Λ/2` (all 36 verified
  with 9 lattice shifts each).  No mirrors (22 negative probes: the
  two perpendicular-part-only candidates plus 20 whole-lattice
  shifts) and no quarter-turns (2 probes) — 24 negative probes in
  total.
- **Lattice & cell:** **8 tiles** per rectangular cell.  The lattice
  is computed from the construction, not hardcoded:
  `w1 = cell2.p3 − cell0.p0 = (1.4422756957517922,
  1.02096015812545)` (both cells direct) and
  `w2 = −(cell5.p3 − cell3.p1) = (−1.3851382485043073,
  1.9567376994827812)` (cells 3 and 5 are mirror-stored, so their
  raw p3/p1 sit at stored indices 2 and 4).  Rectangular (w1 ⊥ w2)
  with `|w1 × w2| = 4.236326192143856 = 8 × tile area` (tile area
  0.529540774017982).  Seeds `[0, 1, 1, 0, 0, 1, 1, 0]`: the two
  isohedral classes are the orbits `{0,3,4,7}` and `{1,2,5,6}` (each
  mixing both chiralities).
- **Edge structure:** **EDGE-TO-EDGE** at the pentagon level — the
  arc's first non-surd edge-to-edge pgg.  Refined edge census over
  the 907-tile block window: 3770 full edges, 0 split edges.  Torus
  gluing census: the 40 cell edges form exactly 20 partner orbits
  (no broken edges, glued pairs equal-length to 1e-9, partner shift
  a lattice vector), with glued side pairs a-a:4, b-b:2, b-e:4,
  c-d:4, c-e:4, d-d:2.  Vertex stars: 816 = 271 × {106°, 106°,
  148°} + 266 × {94°, 133°, 133°} + 279 × {59°, 59°, 94°, 148°}
  (figures rounded to the degree, every star complete); torus
  vertex orbits: 12 (eight of size 3, four of size 4).
- **Verify:** standard trio (coverage, rotated-scaled, deterministic)
  + unit-structure test (8 congruent copies of the runtime prototile:
  angles {58.86°, 148°, 132.86°, 94.27°, 106°} at 1e-9, sides
  1/2 × 4 + a at 1e-12/1e-9, family identities B + 2E = 2C + D =
  360°, 2A + B + D = 360°, A = C − B/2, closure F(C) = 0, seed
  pattern, lattice identities w1 = cell2.p3 − cell0.p0 and
  w2 = −(cell5.p3 − cell3.p1), rectangular, det = 8·area, stored
  cells bit-exact against the raw construction) + window-census test
  (398 / 369 / 193 / 1149 tiles at four fixed windows) +
  refined-edge census (907 tiles: 3770 full, 0 split) +
  vertex-figure census (816 interior vertices: 271 × {106°,
  106°, 148°}, 266 × {94°, 133°, 133°}, 279 × {59°, 59°, 94°,
  148°}) + cell-gluing census (20 edge orbits, side-pair histogram,
  12 vertex orbits) + pgg-symmetry test (exact half-turn about
  c0 = (1/4, 0) on the interior-AABB patch; g1, g1⁻¹, g2, g2⁻¹ and
  lattice-shifted glide variants preserve the tiling; g1² = w1,
  g2² = −w2, g2∘g1 = half-turn about c0; half-shifts +w1/2 and
  −w2/2; all 36 half-turns across the four classes c0 + Λ/2; every
  cell maps onto a tile under the three generators and 10 lattice
  translations on the 1922-tile big patch; 24 negative probes) +
  isohedral test (cell permutations ht = (0 4)(1 5)(2 6)(3 7),
  g1 = (0 3)(1 2)(4 7)(5 6), g2 = (0 7)(1 6)(2 5)(3 4); exactly two
  full-group orbits {0,3,4,7}/{1,2,5,6} = the seed classes; every
  generated tile keeps the prototile side multiset {1/2, 1/2, 1/2, 1/2, a}) +
  enantiomer-mix test (prototile chiral — forward cyclic match only —
  and the eight cells split four direct / four mirror).
- **Status:** committed (M72). 62 tilings, 476 tests at the commit.

### 26. Pentagon type 8 (Kershner 1968) ✅ (M77)

- **Motif:** Kershner's type-8 convex pentagonal tiling — the
  two-parameter family `b = c = d = e`, `2B + C = D + 2E = 360°` —
  pinned to the exact Q(√3) member `B = C = 120°`:
  `tan A = 3√3` (A ≈ 79.11°), `cos D = 1/7` (D ≈ 81.79°),
  `tan E = −√3/2` (E ≈ 139.11°), `A = E − 60°`, `a = 6√7`,
  `b = c = d = e = 14`; tile area 189√3. All eight cell vertices
  sit on the Z[√3] grid (integer x, integer·√3 y); the shipped
  table is the verified M73 integer cell at scale 42 divided by 3.
- **Group:** **pgg** (orbifold 2\*22) by structure plus exact audit:
  two perpendicular glide families with axes along the lattice
  directions, `g1² = w1`, `g2² = −w2` — each a true half-translation
  glide (along-axis shift +|w1|/2 and −|w2|/2 exactly, so no power
  of either glide is a pure reflection); half-turns centred at
  `c0 = (−1/2, 25√3/2)` (lattice coords (7/24, 11/36)) and its
  lattice images. No mirrors (22 negative probes: the two
  perpendicular-part-only candidates plus 20 whole-lattice shifts)
  and no quarter-turns (2 probes) — 24 negative probes in total.
- **Lattice & cell:** **8 tiles** per rectangular cell,
  `w1 = (36, 24√3)`, `w2 = (−36, 18√3)`, `w1 ⊥ w2` exactly,
  `|det(w1, w2)| = 1512√3 = 8 × tile area`. Seeds `[0, 1, 1, 0,
  0, 1, 1, 0]`: the two isohedral classes are the orbits
  `{0,3,4,7}` and `{1,2,5,6}` (each mixing both chiralities).
  Cells T2, T3, T5, T8 are mirror placements (stored CCW with
  vertex order reversed).
- **Edge structure:** **EDGE-TO-EDGE** at the pentagon level.
  Refined edge census over the 79-tile window (−25, −20√3)–
  (25, 20√3): 395 directed full edges, 0 duplicates, 290 interior
  directed edges each paired with its reverse, 148 vertices, 91
  interior, no T-junctions (minimum corner-to-opposite-edge
  distance 13.7477 ≫ 1e-7). Torus gluing census: the 40 cell edges
  form exactly 20 partner orbits (no broken edges, all pairs
  reverse-glued, partner shift a lattice vector, equal lengths),
  with glued side pairs a-a:4, b-c:4, b-d:4, c-c:2, d-e:4, e-e:2.
  Vertex stars: 91 interior = 30 × {A,A,C,D} + 36 × {B,B,C} +
  25 × {D,E,E}, every star complete; torus vertex orbits: 12 (eight
  of size 3, four of size 4).
- **Verify:** standard trio (coverage, rotated-scaled,
  deterministic) + unit-structure test (8 cells: prototile corners
  and lattice vectors bit-exact Q(√3) surds, corner cosines
  √7/14, −1/2, −1/2, 1/7, −2√7/7, tan A = 3√3, tan E = −√3/2,
  A = E − 60°, 2B + C = D + 2E = 360°, a/b = 3/√7, lattice
  (36, 24√3)/(−36, 18√3) exactly perpendicular, det = 8·area,
  |w1|² = 3024, |w2|² = 2268, seeds [0,1,1,0,0,1,1,0], per-cell
  CCW + area + side multiset {14 ×4, 6√7} + chirality fingerprints
  + uniqueness) + window-census test (79 / 42 / 20 / 5 tiles at
  four fixed windows) + refined-edge census (79 tiles: 395 full
  edges, 0 split, 290 interior directed edges all paired, 148
  vertices, no T-junctions) + vertex-figure census (91 interior
  vertices: 30 × {A,A,C,D}, 36 × {B,B,C}, 25 × {D,E,E}) +
  cell-gluing census (20 edge orbits, side-pair histogram, 12
  vertex orbits, orbit stars in degree tenths) + pgg-symmetry test
  (exact half-turn about c0 = (−1/2, 25√3/2) on the c0-symmetric
  patch; g1, g1⁻¹, g2, g2⁻¹ and 8 lattice-shifted glide variants
  preserve the tiling; g1² = w1, g2² = −w2 exactly, axis shifts
  +1/2 / −1/2, perpendicular parts (−4, 2√3) and (39, 26√3) exact;
  g1 maps T1 exactly onto T5, g2 maps T1 exactly onto T8; 24
  negative probes) + isohedral test (cell permutations ht =
  [3,2,1,0,7,6,5,4], g1 = [4,5,6,7,0,1,2,3], g2 = [7,6,5,4,3,2,1,0];
  exactly two full-group orbits {0,3,4,7}/{1,2,5,6} = the seed
  classes; both seeds appear in the patch; every generated tile
  keeps the prototile side multiset) + enantiomer-mix test
  (prototile chiral — forward cyclic match only — and the eight
  cells split four direct / four mirror).
- **Status:** committed (M77). 62 tilings, 485 tests at the commit.

### 27. Pentagon type 6 (Kershner 1968) ✅ (M74)

- **Motif:** Kershner's type-6 convex pentagonal tiling — the family
  `a = d = e`, `b = c`, `A = 2C`, `C + E = 180°` (in Wikipedia's
  shifted lettering W-A = our B, W-B = our C, W-C = our D,
  W-D = our E, W-E = our A, the same family reads
  `a = b`, `c = d = e`, `2B = E`, `B + D = 180°`) — pinned to its
  unique 30°-grid member (A, B, C, D, E) = (120°, 120°, 60°, 120°,
  120°) with (a, b, c, d, e) = (1, 2, 2, 1, 1), tile area 7√3/4;
  exact Q(√3) pin A = (0,0), B = (1,0), C = (2,√3), D = (0,√3),
  E = (−1/2, √3/2). The prototile is achiral (its mirror is a 300°
  rotation plus translation of the pin) and congruent to the
  floret-pentagonal tile (pin = R30⁷·(√3·FLORET6) + (2,√3), unique
  over the 12 rotations) — one pentagon, two tilings: this p2
  2-isohedral one vs the floret's p6 1-isohedral.
- **Group:** **p2** by structure plus exact audit: the half-turns
  form one coset `c0 + Λ/2`, `c0 = (1/2, 0)` — the c0-symmetric
  window is exact set-equal under the half-turn, all four coset
  representatives are centres with every whole-lattice shift, and
  the off-coset half-lattice points are not centres (their
  half-turns square to a lattice translation). No mirror or glide:
  a mirror axis must permute the edge-direction set {0°, 60°,
  120°} mod 180° — the six candidates 0, 30, 60, 90, 120, 150° —
  and none normalizes Λ (nor do R60, R90, R120 — no 3-, 4-, or
  6-fold rotations). A complete 22-ray mirror enumeration found 0
  genuine candidates; the normalizer condition is load-bearing —
  a block-image check alone admits four spurious subset maps with
  R(Λ) ⊊ Λ.
- **Lattice & cell:** **4 tiles** per oblique cell,
  `w1 = (1, 2√3)`, `w2 = (−3, √3)`,
  `|det(w1, w2)| = 7√3 = 4 × tile area` (|w1|² = 13, |w2|² = 12,
  w1·w2 = 3). Seeds `[0, 0, 1, 1]`: the two isohedral classes are
  the orbits `{T0, T1}` / `{T2, T3}`; the half-turn about any
  centre permutes the cells mod Λ by the closed permutation
  0 ↔ 1, 2 ↔ 3. All cells are direct placements with vertex i =
  the prototile's (A,B,C,D,E)[i] in every cell (the 60° corner at
  index 2).
- **Edge structure:** **EDGE-TO-EDGE** at the pentagon level.
  Every one of the 20 stored sides has exactly one physical
  reverse neighbour (cell j, side f, shift λ); the partner map is
  an involution with λ′ = −λ, equal lengths, and shift ∈ {0, ±w1,
  ±w2, ±(w1−w2)} — 4 in-unit pairs (8 sides at λ = 0) + 6 shifted
  pairs; side-pair census a-a:1, a-d:2, b-b:1, b-c:2, c-c:1,
  d-e:2, e-e:1 (10 undirected orbits). On the torus the 20
  directed sides wrap to 18 directed side classes; one arc is
  double-covered — (2,√3)→(0,√3) ≡ (3,0) — carrying lifts T0e2,
  T3e1 in that direction and T1e2, T2e1 reversed (endpoint-
  congruent, not translates of each other); the other 16 classes
  have exactly one lift. 787-tile window (±20.25 × ±15√3): 3935
  directed full edges all unique, 3424 interior directed edges
  each paired with its reverse, 1286 vertices, 1022 interior, no
  T-junctions. Torus Euler: 6 − 10 + 4 = 0.
- **Vertex stars:** 1022 interior = 340 × {60°,60°,120°,120°} +
  682 × {120°,120°,120°}, every star complete. Torus vertex
  orbits: 6 (sizes 3, 3, 3, 3, 4, 4) — stars 2 ×
  {60°,60°,120°,120°} + 4 × {120°,120°,120°}, all summing to 360°.
- **Verify:** standard trio (coverage, rotated-scaled,
  deterministic) + unit-structure test (4 cells: prototile corners
  bit-exact Q(√3) surds, corner angles 120/120/60/120/120, side
  multiset {1,1,1,2,2}, lattice (1, 2√3)/(−3, √3) with det =
  4·area, |w1|² = 13, |w2|² = 12, w1·w2 = 3, seeds [0,0,1,1],
  per-cell CCW + area) + window-census test (3268/864/112/20 tiles
  at four fixed windows) + edge-to-edge gluing test (787 tiles:
  3935 full directed edges all unique, 3424 interior directed
  edges each paired with its reverse, 1286 vertices) +
  vertex-figure census (1022 interior vertices: 340 ×
  {60°,60°,120°,120°}, 682 × {120°,120°,120°}) + cell-gluing
  census (per-side physical partner search — exactly one
  (j, f, λ) with λ a lattice vector and endpoint agreement;
  involution with λ′ = −λ; shifts in {0, ±w1, ±w2, ±(w1−w2)}; 8
  in-unit sides; side-pair histogram; torus: 18 directed classes,
  the double-covered arc (2,√3)→(0,√3) ≡ (3,0) with lifts
  {(0,2),(3,1)} forward and {(1,2),(2,1)} reversed; 6 vertex
  orbits (3,3,3,3,4,4), orbit stars) + p2-symmetry test (exact
  half-turn about c0 = (1/2, 0) on the c0-symmetric window; all
  four c0 + Λ/2 representatives are centres with whole-lattice
  shifts; off-coset half-lattice points are not centres; the six
  mirror-direction candidates and R60/R90/R120 all fail to
  normalize Λ) + isohedral test (half-turns permute the cells
  0 ↔ 1, 2 ↔ 3; the group they generate has exactly two orbits
  {0,1}/{2,3} = the seed classes; both seeds appear in every
  patch; every generated tile keeps the prototile side multiset)
  + achiral test (mirror_y(T0) is set-equal to R300·T0 +
  (−1/2, −√3/2), pointwise Rot60·MIR[i] + (−1/2, √3/2) =
  PIN[i] with MIR in the mirror's own CCW order) + floret-kinship
  test (pin = R30⁷·(√3·FLORET6) + (2,√3) pointwise, FLORET6 =
  [f2,f3,f4,f0,f1]; unique (k, t) = (7, (2,√3)) over the 12
  rotations — floret constants defined locally, no laves.rs
  dependency).
- **Status:** committed (M74). 63 tilings, 497 tests at the commit.

### 28. Stepped hexominoes ✅ (M78)

- **Motif:** the asymmetric 2-step staircase hexomino — unit squares
  (0,0), (0,1), (1,1), (1,2), (2,1), (2,2) (width 3, height 3,
  area 6) — one of the 12 free hexominoes, with a trivial
  automorphism group (no non-identity rotation or mirror of the
  plane maps it to a translate of itself). The catalog's first
  **p1** realizer — the tenth and, for now, last-represented card
  of the Wallpaper group catalog is p1's (7 of the 17 groups still
  unrepresented).
- **Group:** **p1** by structure plus exact probe: the normalizer of
  Λ in the plane's isometry group is the centered-rectangular Klein
  four-group {I, R180, Mx, My}, and the asymmetric staircase breaks
  all three non-identity members — R180, a horizontal mirror, and a
  vertical mirror each map the hexomino onto a different shape
  (exact vertex-set comparison) — while the remaining D4 candidates
  (R90, R270, Md, Ma) do not normalize Λ in the first place. An
  independent 12-tile patch carries no non-identity symmetry, and
  all seven coset candidates normalize to False.
- **Lattice & cell:** 1 tile per **centered-rectangular** cell,
  declared on the *primitive* basis `w1 = (3,1)`, `w2 = (0,2)`
  (det 6 = area) — the same convention as girih-star-cross, which
  keeps a primitive cell of its centered lattice. The centered cell
  is `⟨(0,2), (−6,0)⟩`: a 6×2 rectangle (area 12 = 2·det) whose
  center (−3,1) = −w1 + w2 is a lattice point. Structural note:
  every tile spans exactly the columns {3k, 3k+1, 3k+2}, so the
  columns x ≡ 2 (mod 3) are *complete* vertical tile-boundary lines
  — the tiling decomposes into width-3 vertical bands. Harmless to
  the group: (3,0) ∉ Λ, so no two bands are congruent by a tiling
  symmetry. (The early "oblique" guess for this lattice was wrong:
  the Rust `measured_kind` stage-3 probe — |u×v| = 2|det|, u ⊥ v,
  (u+v)/2 ∈ L — finds the centered witness (0,2)/(−6,0), and the
  legality test pins the declaration to it.)
- **Edge structure:** **EDGE-TO-EDGE** at the unit-edge level under
  the collinear-vertex convention: the 12-vertex polygon carries the
  four interior corners (2,1), (3,2), (2,3), (0,1) as 180°
  (collinear) points, so every boundary edge is a unit edge; every
  interior unit edge is shared by exactly two tiles (exact
  T-junction test over windows W = 4/12/20).
- **Vertex stars:** W = 12 window (529 vertices): (90°,270°) × 184,
  (180°,180°) × 172, (360°) × 92, (90°,90°,90°,90°) × 81 — every
  figure sums to 360°. The (360°) class is the single lattice point
  strictly inside each tile — (2,2) + λ, the interior point of the
  2×2 block the hexomino contains — a degenerate single-sector
  figure.
- **Verify:** standard trio (coverage, rotated-scaled,
  deterministic) + the exact p1 probe
  (`stepped_hexominoes_wallpaper_group_is_p1`: the 7-candidate
  split — R90/R270/Md/Ma fail the Λ-normalizer, R180/Mx/My fail
  the exact vertex-set shape test — (I, v) is a symmetry for every
  v in a wide Λ window, no non-identity M is a symmetry for any v,
  positive sanity at (3,1)/(0,2), negative sanity at (0,1) and
  (3,0) with (3,0) ∉ Λ) + the brute-force culling cross-check +
  the legality test (measured_kind stage 3 returns
  CenteredRectangular, matching the declaration) + the Python
  transcript (`m78_final_check.py`): Bravais centered-rectangular
  (witness ((6,0), (0,−2), center (3,−1))); coverage 0-bad at
  W = 4/8/16; the 6 cell translates pairwise incongruent mod Λ;
  e2e True at W = 4/12/20; the full D4 probe — normalizer
  {I, R180, Mx, My}, every coset candidate False, 0 positive
  non-identity symmetries; the independent B = 12 patch symmetry
  test (empty); the vertex-figure census above; the complete-
  boundary-column scan (all x = 3k+2 in [−12,12)); the polygon
  (shoelace 2·area = 12, CCW; interior angles 90°×6 + 180°×4 +
  270°×2, sum 1800 = 180°·(12−2)).
- **Status:** committed (M78). 64 tilings, 501 tests at the commit.

### 29. Trapezoid courses ✅ (M79)

- **Motif:** two courses of trapezoids per 2×1 cell on the
  rectangular lattice `Λ = ⟨(2,0),(0,1)⟩` (det 2): the upper course
  `P = (0,3/10),(1,1/2),(1,1),(0,1)` (area 6/10) and its mirror `mP`
  in `x = 1` (area 6/10), the lower course `P' = (0,0),(1,0),(1,1/2),
  (0,3/10)` (area 4/10) and its mirror `mP'` (area 4/10). The courses
  meet along slants of slope `±1/5` (intercept `3/10`). The catalog's
  first **pm** realizer — the eleventh represented card of the
  Wallpaper group catalog (6 of the 17 groups still unrepresented).
- **Group:** **pm** by structure plus exact probe: the mirror lines
  are exactly the vertical lines `x = k`, `k ∈ ℤ` — `x = 1` exchanges
  `P ↔ mP` and `P' ↔ mP'` within one cell, `x = 0` does the same
  across a cell boundary (a lattice translate). No horizontal mirror
  and no half-turn: a horizontal mirror or a half-turn centered on
  `y = 1/2 + n` would send the upper course (area 6/10, horizontal
  top at `y = 1`) to the lower course (area 4/10) — impossible by
  area — and the `y = 1 + n` centers map a trapezoid to a slant-
  reversed copy that is not a tile (exact vertex-set probe). The
  structural reason is the **incongruence of the two prototile
  classes** (areas 6/10 vs 4/10; squared-edge multisets differ): a
  pmm tiling would need a half-turn or a second mirror family, and
  both are killed. The shape parameters are chosen for this: slope
  `m = 1/5 ∉ {0, ±1}` (no parallelogram/axis degeneracy) and
  intercept `c = 3/10 ≠ (1 − m)/2 = 2/5` — with `c = (1 − m)/2` the
  two trapezoids would be congruent, half-turns would appear, and
  the group would be pmm.
- **Lattice & cell:** 4 tiles per **rectangular** (primitive) cell,
  basis `(2,0)`, `(0,1)`, det 2 = total tile area; measured kind
  Rectangular, matching the declaration (legality test
  `declared_symmetry_is_legal_and_matches_the_measured_lattice`).
- **Edge structure:** **EDGE-TO-EDGE**: every edge paired exactly
  twice, no T-junctions (exact segment census over windows
  W = 2/3/5). Edge classes: the slants (slope `±1/5`, length
  `√65/5`), each pairing one upper with one lower trapezoid; the
  horizontal unit edges at `y = 0`/`y = 1` (paired between vertical
  cell translates); the vertical edges at `x ∈ {0,1,2}` of lengths
  `3/10`, `1/2`, `7/10`.
- **Vertex stars:** every interior vertex figure sums to 360°:
  `{arctan(5) = 78.6901° × 2, 180° − arctan(5) = 101.3099° × 2}` at
  the slant endpoints, `{90° × 4}` at the horizontal-line vertices
  (121 interior vertices in the W = 3 window).
- **Verify:** standard trio (coverage, rotated-scaled,
  deterministic) + the exact pm probe
  (`trapezoid_courses_wallpaper_group_is_pm`, mirroring the full
  Python battery `m79_verify.py`: 15 positive candidates — lattice
  translations and the vertical mirrors `x = k` with lattice-
  translate composites — against 53 negative candidates: 5 off-
  lattice translations, 3 half-integer vertical mirrors, 18 half-
  turn centers, 9 horizontal mirrors/glides, 8 rot90/rot270, 3
  diagonal + 3 anti-diagonal mirrors/glides, 4 vertical glides;
  plus the structural mirror sanity `x = 1` pairing `P ↔ mP`,
  `P' ↔ mP'` with no translate) + the brute-force culling
  cross-check + the legality test + the Python transcript: convex
  CCW polygons, areas 6/10·6/10·4/10·4/10 summing to det 2,
  boundary-only cell partition with union = cell, e2e at W = 2/3/5,
  the vertex-star census above, P/P' incongruent (area +
  squared-edge multisets), rot60/rot120 lattice-kill.
- **Status:** committed (M79). 65 tilings, 505 tests at the commit.
  Family: §31 (M81) stacks these same courses in rows of height 1
  with alternate rows flipped in `y = 1` and shifted by `(1,0)` —
  the pmg upgrade of this pm pattern.

### 30. Glide hexagon strips ✅ (M80)

- **Motif:** two horizontal hexagon strips per 1×2 cell on the
  rectangular lattice `Λ = ⟨(1,0), (0,2)⟩` (det 2): strip 0
  (`y ∈ [0,1]`) is the row `A, B, A+(1,0), B+(1,0), …` and strip 1
  (`y ∈ [1,2]`) is its glide image. The four cell tiles (CCW): `A`
  = `(0,0),(1/2,0),(7/10,2/5),(1/2,1),(0,1),(−1/4,1/2)` (fat,
  convex, area 29/40), `B` = `(1/2,0),(1,0),(3/4,1/2),(1,1),(1/2,1),
  (7/10,2/5)` (waist, two reflex vertices, area 11/40),
  `T = g(A) + (−1,2)` (fat, area 29/40) and `U = g(B) + (−1,2)`
  (waist, area 11/40) for the glide `g(x,y) = (x + 1/2, −y)`,
  `g² = (1,0) ∈ Λ`. `A ≅ T` and `B ≅ U` are chiral twins (related
  by a reflection, not by any direct isometry); the fat and waist
  classes are incongruent by area (29/40 vs 11/40) and reflex count
  (0 vs 2) — though all four tiles share the identical squared-edge
  multiset `{1/4, 1/4, 1/5, 2/5, 5/16, 5/16}`. The strip-1 finger is
  forced to `x = 1/5` by the glide, `g(7/10,2/5) + (−1,2) =
  (1/5,8/5)`; a hand-placed finger at `2/5` would break the glide
  and drop the group to p1. The catalog's first **pg** realizer —
  the twelfth represented card of the Wallpaper group catalog (5 of
  the 17 groups still unrepresented).
- **Group:** **pg** by structure plus exact probe: translations
  plus ONE horizontal glide family (glide 1/2), no mirrors, no
  rotations, no perpendicular glides. The glide `g` (axis `y = 0`)
  maps the tile set onto itself (`g(A) = T+(1,−2)`, `g(B) =
  U+(1,−2)`, `g(T) = A+(0,−2)`, `g(U) = B+(0,−2)`), and the
  composites `g ∘ (a, 2b)` carry two parallel glide-line orbits,
  `y ∈ 2ℤ` and `y = 1 + 2ℤ` (e.g. `Φ = T_{(−1,2)} ∘ g =
  (x − 1/2, 2 − y) = g ∘ (−1,−2)` is the glide on `y = 1`,
  `Φ² = (−1,0) ∈ Λ`) — one glide direction, two orbits. No
  horizontal mirror or other glide: the vertex levels mod 1 carry
  x-positions `0: (1/2)ℤ`, `1/2: ±1/4 + ℤ` (line-dependent),
  `2/5: 7/10 + ℤ`, `3/5: 1/5 + ℤ`, and the only candidate surviving
  the finger-tip case split is the half-glide `s ≡ 1/2, 2c ≡ 1`,
  killed by the elbow `(−1/4, 1/2) ↦ (1/4, 1/2)`, which is not a
  vertex. No vertical mirror: `x = c` must preserve the half-unit
  edge pattern on `y ∈ ℤ` (forcing `2c ∈ (1/2)ℤ`) and map the
  finger tip `(7/10, 2/5)` to a same-level vertex, forcing
  `2c ≡ 7/5 (mod 1)`; `7/5 ∉ (1/2)ℤ`. No half-turns: centers with
  `b ≡ 0` and `b ≡ 1/2 (mod 1)` both map the finger tip to level
  `3/5`, which carries vertices only at `x ≡ 1/5 + ℤ`, forcing
  `2a ≡ 9/10 (mod 1)` against the edge-pattern constraint
  `2a ∈ (1/2)ℤ`; `9/10 ∉ (1/2)ℤ`. Diagonal mirrors/glides and
  90°/270°/120°/240° rotations do not normalize `Λ` (and there are
  no vertical edges for a 90° rotation to map). The exact probe
  confirms the complete 16-coset enumeration: only the identity
  translation coset and the single glide coset are positive — so
  the translation subgroup is exactly `Λ` (`g² = (1,0)` is a
  primitive translation, not a double one).
- **Lattice & cell:** 4 tiles per **rectangular** (primitive) cell,
  basis `(1,0)`, `(0,2)`, det 2 = 29/40 + 11/40 + 29/40 + 11/40 =
  total tile area; measured kind Rectangular, matching the
  declaration (legality test
  `declared_symmetry_is_legal_and_matches_the_measured_lattice`).
- **Edge structure:** **EDGE-TO-EDGE**: every edge paired exactly
  twice, no T-junctions (exact segment census over the 396-tile
  window: 352 interior edges, 868 vertices; clipped coverage
  72 = 72). 24 tile-edges per cell in five directions (mod 180°),
  no verticals: 8 horizontal of length 1/2 (on `y ∈ ℤ`); the
  slope-`±2` family, 4 edges of length √5/5 (finger links) plus 8
  of length √5/4 (chain edges); and the slope-`±3` family, 4 edges
  of length √10/5 (notch sides).
- **Vertex stars:** every one of the 265 interior vertex figures
  sums to 360°: finger tips `(7/10,2/5)+Λ`, `(1/5,8/5)+Λ` →
  `{225°, 135°}` (two tiles); notch elbows `(3/4,1/2)+Λ`,
  `(1/4,3/2)+Λ` → `{233.1301°, 126.8699°}` = `{360° −
  arccos(−3/5), arccos(−3/5)}` (two tiles); interface `(0,1)+Λ`,
  `(1/2,1)+Λ` → `{116.5651°, 108.4349°, 71.5651°, 63.4349°}`
  (= `{180° − arctan 2, 180° − arctan 3, arctan 3, arctan 2}`,
  four tiles); baseline `(0,0)`, `(1/2,0)`, `(0,2)`, `(1/2,2) + Λ`
  → `{116.5651° × 2, 63.4349° × 2}` (four tiles).
- **Seeds:** `proto` = cell index (registry order `[A, T, B, U]`);
  `seed` = shape class — 0 = the fat hexagons `A, T`, 1 = the waist
  hexagons `B, U`.
- **Verify:** standard trio (coverage, rotated-scaled,
  deterministic) + the exact pg probe
  (`glide_hexagon_strips_wallpaper_group_is_pg`, mirroring the full
  Python battery `m80_verify.py`: positive = lattice translations
  plus the glide family `g · (0,2)^k`; negative = the complete
  16-coset enumeration for every candidate linear part — I,
  M_MIRY, M_MIRX, M_ROT2 — plus the non-normalizing parts, each
  sampled at multiple centers) + the brute-force culling
  cross-check + the legality test + the Python transcript: CCW
  polygons with areas 29/40, 11/40, 29/40, 11/40 summing to det 2,
  the 396-tile window partition (352 interior edges paired twice,
  868 vertices, no T-junctions, clipped coverage 72 = 72), the
  265-vertex star census above, the congruence census (`A ≅ T`,
  `B ≅ U` reflection-only; all other pairs incongruent), battery
  74 positive (49 translations + 25 glides) / 62 exact-coset
  negative + 6 non-normalizing parts.
- **Status:** committed (M80). 66 tilings, 509 tests at the commit.

### 31. Shifted trapezoid courses ✅ (M81)

- **Motif:** the trapezoid-course row of §29 (M79, pm) stacked in
  rows of height 1 per 2×2 cell on the rectangular lattice
  `Λ = ⟨(2,0),(0,2)⟩` (det 4): row 0 (`y ∈ [0,1]`) is the M79 pm
  cell `P, mP, P', mP'`, and row 1 (`y ∈ [1,2]`) is `F(row 0) +
  (1,0)` with `F(x,y) = (x, 2−y)` — the row flipped in `y = 1` and
  **shifted by `(1,0)`**, half the tiling's x-period. The eight
  cell tiles (CCW): the row-0 four unchanged; `R0 = (1,1),(2,1),
  (2,15/10),(1,17/10)`, `R1 = (1,17/10),(2,15/10),(2,2),(1,2)`,
  `R2 = (2,1),(3,1),(3,17/10),(2,15/10)`, `R3 = (2,15/10),(3,17/10),
  (3,2),(2,2)` (areas 6/10, 4/10, 6/10, 4/10). The flip preserves
  congruence, so exactly two shape classes: the P-class `{P, mP,
  R0, R2}` (area 6/10, squared edges {25, 49, 100, 104}) and the
  P'-class `{P', mP', R1, R3}` (area 4/10, {9, 25, 100, 104}).
  Family: §29 trapezoid-courses (M79, pm) — same row, same
  trapezoids; stacking with the alternate-row flip + half-period
  shift is what turns the pm pattern into pmg. The catalog's first
  **pmg** realizer — the thirteenth represented card of the Wallpaper
  group catalog (4 of the 17 groups still unrepresented).
- **Group:** **pmg** by structure plus exact probe: the translation
  subgroup is exactly `Λ`; one vertical mirror family `x = k`,
  `k ∈ ℤ` (`x = 1` pairs `P ↔ mP`, `P' ↔ mP'` within row 0 and
  `R0 ↔ R2 + (−2,0)`, `R1 ↔ R3 + (−2,0)` within row 1); one
  horizontal glide family `g(x,y) = (x + 1, −y)` (axis `y = 0`,
  glide 1, `g² = (2,0) ∈ Λ`): `g(P) = R0 + (0,−2)`, `g(P') =
  R1 + (0,−2)`, `g(mP) = R2 + (0,−2)`, `g(mP') = R3 + (0,−2)`, and
  the composites `g ∘ (2i, 2j)` carry glide 1 on every axis
  `y = j`, `j ∈ ℤ` (two lattice orbits, even-`y` and odd-`y`; e.g.
  `g ∘ (0,2) = (x+1, 2−y)` maps `P` to `R0` with no translate at
  all); and half-turns about `(m + 1/2, n)` for **all** `m, n ∈ ℤ`
  — four per 2×2 cell, the even-y centers as well (about `(1/2, 1)`,
  `P → R2 + (−2,0)`; about `(1/2, 0)`, `P → R2 + (−2,−2)`); the
  centers form two Λ-orbits (even-y / odd-y) exchanged by the
  mirror `x = 1`, the mirror × glide intersections. It is pmg, not
  pmm, because of the `(1,0)` row shift: the horizontal mirror
  `y = 1`, `(x,y) ↦ (x, 2−y)`, sends `P` to `F(P) = R0 + (−1,0)`
  — the right tile shape half a tiling period off the lattice (the
  unshifted flip is not a tiling symmetry), and the glide-`1/2`
  `(x + 1/2, −y)` sends the slant endpoint `(0,3/10)` to
  `(1/2,−3/10)`, which is not a vertex. The complete 128-coset
  enumeration (eight D4 linear parts that normalize the square-
  metric Λ × 16 cosets) is positive in exactly four cosets — the
  identity, the mirror `x = k`, the glide `(1/2, 0)`, the half-turn
  `(1/2, 0)` — so there is no p4/p4m/p4g/cmm upgrade (rot90/rot270,
  diagonal and anti-diagonal mirrors/glides are negative in all 16
  cosets each).
- **Lattice & cell:** 8 tiles per **rectangular** (primitive) cell,
  basis `(2,0)`, `(0,2)`, det 4 = 6/10·4 + 4/10·4 = total tile
  area. The basis is equal-length and orthogonal — the lattice is
  square METRIC — but the point group has no 4-fold, so the Bravais
  kind stays Rectangular: the legality test's degenerate-
  realization exception (running-bond precedent) admits a declared
  Rectangular lattice whose basis measures Square.
- **Edge structure:** **EDGE-TO-EDGE**: every edge paired exactly
  twice, no T-junctions (exact segment census over the 180-clipped-
  tile window `W = [−4,4]²`: 188 interior edges, 448 vertices,
  clipped coverage 6400 = 6400, 0 overlaps). Edge classes (exact
  lengths): the slants (slope `±1/5`, length `√65/5`), pairing
  within each row — row 0: `P ↔ P'`, `mP ↔ mP'` at the `3/10`/`1/2`
  levels; row 1: `R0 ↔ R1`, `R2 ↔ R3` at the `15/10`/`17/10`
  levels; the horizontal unit edges at `y ∈ ℤ`, pairing row-1
  bottoms with row-0 tops **across cell boundaries** (the `(1,0)`
  shift: `P`'s top `(0,1)–(1,1)` pairs with the bottom of
  `R2 + (−2,0)`); and the vertical edges at `x ∈ ℤ` of lengths
  `3/10`, `1/2`, `7/10`.
- **Vertex stars:** every one of the 105 interior vertex figures
  sums to 360°: the slant endpoints (the `3/10`/`1/2`-level points
  of row 0 and the `15/10`/`17/10`-level points of row 1) →
  `{arctan(5) = 78.6901° × 2, 180° − arctan(5) = 101.3099° × 2}`
  (four tiles); the flat–flat seam vertices `(k, n)`, `k, n ∈ ℤ` →
  `{90° × 4}` (four tiles) — the same two stars M79 carries.
- **Seeds:** `proto` = cell index (registry order `[P, mP, P', mP',
  R0, R1, R2, R3]`); `seed` = shape class — 0 = the 6/10 trapezoids
  `P, mP, R0, R2`, 1 = the 4/10 trapezoids `P', mP', R1, R3`.
- **Verify:** standard trio (coverage, rotated-scaled,
  deterministic) + the exact pmg probe
  (`shifted_trapezoid_courses_wallpaper_group_is_pmg`, mirroring the
  full Python battery `m81_verify.py`: 48 positive candidates — 25
  lattice translations, 5 vertical mirrors `x = k` (`k = −2..2`),
  9 glide samples `(x + 1 + 2i, −y + 2j)`, 9 half-turns about
  `(m + 1/2, n)` — against the complete 128-coset enumeration (8
  normalizing D4 linear parts × 16 cosets: 4 pos / 124 neg) plus the
  named negatives: (1,0), (0,1), (1,1), (1/2,0) translations; the
  horizontal mirrors `y = 0` (the unshifted flip) and `y = 1/2`; the
  glide-1/2 `(x + 1/2, −y)`; the vertical mirror `x = 1/2`; vertical
  glides with slide (0,1); the off-set half-turn centers
  `(0,0), (1/2,1/2), (0,1/2), (1,1/2)`; structural sanity: the
  mirror `x = 1` pairs `P ↔ mP`, `P' ↔ mP'` with no translate, the
  glide `(x, 2−y)` maps `P → R0`, `P' → R1`, `mP → R2`, `mP' → R3`
  with no translate, the half-turn about `(1/2, 1)` maps
  `P → R2 + (−2,0)`, and the even-y center `(1/2, 0)` maps
  `P → R2 + (−2,−2)`) + the brute-force culling cross-check + the
  legality test + the Python transcript: 8 convex CCW tiles with
  doubled areas 120, 120, 80, 80, 120, 80, 120, 80 summing to 800 =
  10² · det Λ, the two-class congruence census, the window partition
  (188 interior edges paired twice, 448 vertices, no T-junctions,
  clipped coverage 6400 = 6400, 0 overlaps among 180 clipped tiles),
  the 105-vertex star census above.
- **Status:** committed (M81). 67 tilings, 513 tests at the commit.

### 32. Wavy courses ✅ (M82)

- **Motif:** a course (horizontal strip) `k` is bounded by two
  phase-shifted zigzag waves of period 2 — the bottom wave
  `Z(x−k) + k` and the top wave `4/3 − Z(x−k) + k`, where
  `Z(t) = |t|/3` on `[-1,1]` extended with period 2 (valleys at the
  even integers, peaks of height `1/3` at the odd integers, slopes
  `±1/3`); the identity `Z(t−1) + 1 = 4/3 − Z(t)` makes course `k`'s
  top wave coincide exactly with course `k+1`'s bottom wave, so the
  courses stack without gaps or overlaps (vertical width
  `4/3 − 2Z(x−k) ∈ [2/3, 4/3]`). Each column-course quadrilateral
  (unit width, area 1) is cut by one diagonal seam, and the seam
  ALTERNATES SLOPE with the column-course parity: `j−k` even → seam
  `(j,k) → (j+1,k+1)` (slope +1); `j−k` odd → seam `(j,k+1) →
  (j+1,k)` (slope −1). The two seam families are mirror images of
  each other in every vertical line — that alternation is what makes
  the vertical mirrors symmetries (a single-seam-slope construction
  has no mirror at all — the M82 brief's original construction was
  re-derived; see the honest notes in `M82_REPORT.md`). The four cell
  tiles (column 0 and column 1 of course 0, CCW): `T_low = (0,0),
  (1,1/3), (1,1)` (area 1/3), `T_high = (0,0), (1,1), (0,4/3)` (area
  2/3), `T_low' = (1,1/3), (2,0), (1,1)` (area 1/3), `T_high' =
  (1,1), (2,0), (2,4/3)` (area 2/3). Two congruence classes:
  `C1 = {T_low, T_low'}` (squared edges `{4/9, 10/9, 2}` — the 2/3
  edge) and `C2 = {T_high, T_high'}` (`{10/9, 16/9, 2}` — the 4/3
  edge) — the classes are INCONGRUENT (areas 1/3 vs 2/3), so no
  symmetry can mix them. Family: the paver-course series (§29–§31).
  The catalog's first **cm** realizer — the fourteenth represented
  card of the Wallpaper group catalog (3 of the 17 groups still
  unrepresented).
- **Group:** **cm** by structure plus exact probe: the translation
  subgroup is exactly `Λ = ⟨(1,1),(2,0)⟩` (the x+y-even lattice of
  ℤ², det 2); one vertical mirror family `x = j` for **all** integers
  `j` — the mirror `x = 0` maps `T_low(0,0)` exactly onto
  `T_low'(−1,0)` (equivalently the cell tile `T_low' + (−2,0)`) and
  `T_high(0,0)` onto `T_high'(−1,0)` (C1 → C1, C2 → C2); one vertical
  glide family `g_j(x,y) = (2j+1−x, y+1)` — axes `x = j + 1/2`, glide
  vector `(0,1)`, `g_j² = (0,2) ∈ Λ` — with `g_j` mapping
  `T_low → T_low' + (−1,1)` and `T_high → T_high' + (−1,1)`. The
  mirrors and the glides share the linear part (x ↦ −x) and form ONE
  coset of Λ: `(1,1) ∈ Λ` (a generator), so every glide
  representative `(2j+1, 1)` differs from the mirror representative
  `(2j, 0)` by a lattice vector. The complete 144-coset enumeration
  (the eight D4 linear parts that normalize the square-metric Λ × 18
  cosets) is positive in exactly two cosets — the identity coset and
  the mirror/glide coset (2 pos / 142 neg) — so there are no
  half-turns, no horizontal mirrors or glides, and no rot90/rot270/
  diagonal/anti-diagonal upgrade (the edge directions ±1 and ±1/3 are
  closed under `s ↦ −s` but not under `s ↦ 1/s`).
- **Lattice & cell:** 4 tiles per **centered-rectangular** primitive
  cell, basis `(1,1)`, `(2,0)`, det 2 = 1/3 + 2/3 + 1/3 + 2/3 = total
  tile area. The lattice is ALSO square — `⟨(1,1),(−1,1)⟩` is an
  orthogonal equal-length basis — so the lattice-legality measurement
  reports Square and the declared CenteredRectangular is admitted by
  the degenerate-realization clause (running-bond / M78 / M81
  precedent).
- **Edge structure:** **EDGE-TO-EDGE**: every tile edge is a wave
  segment (slope `±1/3`), a seam (slope `±1`), or a vertical boundary
  segment, and every such edge is shared by exactly two tiles (window
  `[−4,4]²` census: 124 interior edges each paired exactly twice, no
  T-junctions among 121 vertices, clipped coverage 576 = 576, 0
  overlaps among 200 clipped tiles, 0 gaps on a 48×48 fine grid).
- **Vertex stars:** every one of the 53 interior vertex figures sums
  to 360° (no 180° angles): the integer-y wave vertices →
  `{45° × 4, arctan(1/2) = 26.5651° × 2, arctan(2) = 63.4349° × 2}`
  (eight tiles; 25 in the window); the third-y vertices (the single
  zigzag peaks at `y = k + 1/3`) → `{90° + arctan(1/3) =
  108.4349° × 2, 90° − arctan(1/3) = 71.5651° × 2}` (four tiles; 28
  in the window).
- **Seeds:** `proto` = cell index (registry order `[T_low, T_high,
  T_low', T_high']`); `seed` = shape class — 0 = C1 (area 1/3),
  1 = C2 (area 2/3).
- **Verify:** standard trio (coverage, rotated-scaled, deterministic)
  + the exact cm probe (`wavy_courses_wallpaper_group_is_cm`,
  mirroring the full Python battery `m82_verify.py`: 25 lattice
  translations, 5 vertical mirrors `x = k` (`k = −2..2`), 3 glide
  samples, composites (0,2)/(0,−2) — against the complete
  144-coset enumeration (8 normalizing D4 linear parts × 18 cosets:
  2 pos / 142 neg) plus the named negatives: the translations (1,0),
  (0,1), (1,1/3), (0,1/3); the vertical mirror `x = 1/2`; the wrong-
  slide vertical glides (slide (0,1) on an integer axis, slide (0,2)
  on a half-integer axis); the horizontal mirrors `y = 0, 1/2`; the
  horizontal glide; half-turns about twelve thirds-grid centers;
  rot90/270/diagonal/anti-diagonal spot checks; structural sanity:
  the mirror `x = 0` pairs `T_low ↔ T_low' + (−2,0)` and
  `T_high ↔ T_high' + (−2,0)`, the glide `(1−x, y+1)` maps
  `T_low → T_low' + (−1,1)`, and the centering `(1,1)` is a
  symmetry) + the brute-force culling cross-check + the legality test
  + the Python transcript: 4 convex CCW cell tiles with doubled areas
  6, 12, 6, 12 (×3) summing to 36 = 9 · 2 · det Λ, the two
  incongruent classes, the window partition above, the 53-vertex star
  census above.
- **Status:** committed (M82). 68 tilings, 517 tests at the commit.

### 33. Chiral-cut honeycomb ✅ (M83)

- **Motif:** a chiral 3-armed Y carved into a side-2 regular
  hexagon, centred at O, from the alternating vertices V0, V2,
  V4: each arm V_i → W_i runs from the vertex toward the centre
  but stops at W_i, where W_i = mid(V_i, O) + delta with
  delta = (−1/8, −1/10) ROTATED by 120° with each arm — and the
  three inner seams W_i → O close the Y. The Y cuts the hexagon
  into 3 congruent hexagonal tiles [O, W0, V0, V1, V2, W2],
  [O, W2, V2, V3, V4, W4], [O, W4, V4, V5, V0, W0]: ONE chiral
  congruence class, with tile1 = R120(tile0) and tile2 =
  R240(tile0) EXACTLY (no translate). The delta is what breaks
  the honeycomb's 6-fold and mirror symmetries down to exactly
  p3 (the delta-0 limit is the plain 3-sector fan, p6m). Family:
  the HX-STUDY chiral honeycomb series (the `hx_p3_cell.json`
  handoff, PROPOSED §21). The catalog's first **p3** realizer —
  the fifteenth represented card of the Wallpaper group catalog
  (2 of the 17 groups still unrepresented).
- **Group:** **p3** by structure plus exact probe: 120°/240°
  rotations and translations ONLY — no mirrors, no glides, no
  60°/180°. The ×320 exact-integer probe (all 11 D6 linear parts
  as integer 2L matrices, applied to the 100 raw cell-vertex
  candidates per part, deduped by the coupled-floor `reduce320`)
  is positive in exactly two cosets of the translation group —
  the zero coset, for R120 and R240 — and negative for
  rot60/rot180/rot300 and all six reflection families (tested
  counts 18/27/18/27/18 and 30 × 6); 25/25 lattice translations
  positive, 0/6 named off-lattice negatives. The rot120 centres
  form exactly three orbits `c(i,j) = (√3·i, i+2j)`,
  representatives (0,0), (√3,1), (2√3,2), each with
  `c − R120·c ∈ Λ`.
- **Lattice & cell:** 3 tiles per **hexagonal** primitive cell,
  basis `a1 = (2√3,0)`, `a2 = (√3,3)` (|a1| = |a2| = 2√3, 60°),
  det `6√3` = 3 × tile area `2√3`.
- **Edge structure:** **EDGE-TO-EDGE**: 450 interior edge pairs
  in the standard window, 0 unpaired, 0 bad; clipped coverage
  4096 = 4096, 0 overlaps, 0 gaps. Squared-edge multiset per
  tile (×320², lex-sorted): (94784,−12800) × 2 (inner seams
  W_iO, 1481/1600 − √3/8), (115264,12800) × 2 (arm seams
  W_iV_i, 1801/1600 + √3/8), (409600,0) × 2 (hexagon sides, 4).
- **Vertex stars:** 170 vertex figures, all summing to 360°
  (0 bad): 60 × `{120° × 3}` (the 25 hexagon centres O + 35 odd
  corners V1, V3, V5), 35 × 6-angle even-corner stars
  `{≈58.8079° × 3, ≈61.1921° × 3}` (V0, V2, V4 — six tiles
  meet at each, the Y arms of the three adjacent hexagons
  subdividing the plain 120° angles), and 75 × 2-angle bends
  `{≈177.1677°, ≈182.8323°}` at the arm-tip points W_i (the Y
  arms kink by ≈2.83° where the outer seam V_i → W_i meets the
  inner seam W_i → O).
- **Seeds:** `proto` = cell index `[tile0, tile1, tile2]`;
  `seed` = 0 (one chiral congruence class, `seeds [0,0,0]`).
- **Verify:** standard trio (coverage, rotated-scaled,
  deterministic) + the exact p3 probe
  (`chiral_cut_honeycomb_wallpaper_group_is_p3`, mirroring the
  full Python battery `m83_verify.py`: the 11-part candidate
  census with tested counts 27/18/18/27/18 and 30 × 6, R120/R240
  positive in the zero coset only, 25/25 lattice translations,
  0/6 off-lattice negatives, the three centre orbits with
  `c − R120·c ∈ Λ`, the structural R120/R240 tile cycling, and
  the cell arithmetic) + the brute-force culling cross-check +
  the legality test + the Python transcript: 3 convex CCW
  hexagonal cell tiles with doubled areas (0,409600) (×320²,
  i.e. area 2√3 each) summing to (0,1228800) = 2·det·320², the
  squared-edge multiset above, the 450/0/0 edge-to-edge
  partition, and the 170-vertex star census above.
- **Status:** committed (M83). 69 tilings, 521 tests at the commit.

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

- ✅ **Socolar 12-fold shield tiling (M51).** *Shield hexagons +
  squares + black/white triangles on the dodecagonal frame; PF
  Perron 2+√3; the class's substitution representative. The original
  Socolar-12 decoration tables are blocked-on-source (PROPOSED.md #8);
  the wheel tiling (shield's MLD dual) is parked (M61) — see the
  Research entry below.*
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
