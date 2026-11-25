# Pavers family

Paver bonds and the wallpaper-catalog completion: one `LatticeTiling` realizer per missing wallpaper group — the 17-group catalog closed at M85.

Code: `crates/tess-tilings/src/families/pavers/` — cargo feature `pavers` (default-on).
Adding a tiling here touches only this family: `families/pavers.rs`
(module decl, register slice, classification arms, census `IDS`,
`lattice_tilings` entries) plus the new module and its doc row below.
Work briefs: `docs/briefs/TEMPLATE.md`; fleet protocol: `tessellix/AGENTS.md`.

## Implemented (14 tilings)

Practical paver bonds as `LatticeTiling` data tables: every domino is
emitted with its long side split into two collinear unit edges, so the
polygons are genuinely edge-to-edge at the unit edge scale (180°
figures at the split midpoints, `2.4.4`).

| id | motif | group | notes |
|---|---|---|---|
| `basketweave` | 2:1 dominoes in 2×2 alternating blocks | p4g | 45°-rotated square lattice ⟨(2,2),(2,−2)⟩, 4 split dominoes per cell; 4-folds at the even-even points, mirrors at the odd axes only (offset from the 4-fold centers — p4g, not p4m), six glide families; exact coset group probe (M39) |
| `herringbone` | 2:1 dominoes in chiral L-pairs | pgg | rectangular lattice ⟨(2,2),(1,−1)⟩ (M76), 2 split dominoes per cell; single-direction (chiral): 16 2-fold centers per 4×4 period, two perpendicular glide families (diagonal + anti-diagonal axes), no mirrors; exact coset group probe (M40) |
| `running-bond` | 2:1 dominoes in stretcher rows, half-brick offset | cmm | centered rectangular lattice ⟨(2,0),(1,1)⟩ (M76; the measured lattice is the √2-degenerate square lattice — the legality test's documented exception), 1 split domino per cell; mirrors at the integer x-axes and half-integer y-axes, 2-folds at (k, l+1/2) and (k+1/2, l), glides in both directions; exact coset group probe (M41) |
| `stack-bond` | 2:1 dominoes stacked directly on top of one another (no offset) | pmm | rectangular lattice ⟨(2,0),(0,1)⟩ (det 2), 1 split domino per cell — the running bond without its half-brick offset; figures 4.4.4.4 at even-x points and 2.2 at odd-x split midpoints; mirrors x = k and y = n/2, 2-folds at (k, n/2), no glides; exact coset probe, ROT4/DIAG/ANTI negative (M114) |
| `herringbone-45` | the herringbone bond in its 45°-laid canonical frame | pgg | rectangular lattice ⟨(0,2√2),(√2,0)⟩ (det 4), 2 split 45° dominoes per cell (the L-pair) — M40 rotated by 45°; 2-folds at (√2/4 + k√2/2, 3√2/4 + ℓ√2), vertical glides x = k√2/2 (glide √2), horizontal glides y = ℓ√2 + √2/4 (glide √2/2) descended from M40's diagonal/anti-diagonal glides, no mirrors, no 4-folds; shape-set coset probe (rotated cells break the axis-aligned bbox probe) + conjugated M40 group cross-check (M115) |
| `basketweave-3` | three horizontal courses + one vertical course of 2:1 dominoes (period 5) | pmm | rectangular lattice ⟨(6,0),(0,5)⟩ (det 30) — an index-3 superlattice of the true ⟨(2,0),(0,5)⟩, kept for the whole-cluster cell convention (cf. M39); 15 split dominoes per cell (9 horizontal + 6 vertical); mirrors x = k and y = 2.5k + 1.5, 2-folds at (k, 2.5k′ + 1.5), no 4-folds, no glides; exact Python battery before porting: cell area sum 30 = \|det\|, 6474-point fine grid covered once, 296 interior unit edges each shared by exactly 2 (M116) |

### Paver tilings (wallpaper-catalog completion, M78–M85)

One realizer per missing wallpaper group, each a `LatticeTiling` data
table, each edge-to-edge (0 T-junctions, exact cell-area sums) —
closing the last gap (p3m1) of the 17-group wallpaper catalog at M85
(HX-STUDY handoff). The first realizer of each group is noted.
| id | motif | group | notes |
|---|---|---|---|
| `stepped-hexominoes` | asymmetric 2-step staircase hexomino | p1 | one tile per cell on the centered rectangular lattice ⟨(3,1),(0,2)⟩ (primitive cell; centered cell 6×2 with center (−3,1)), edge-to-edge with 180° at the four collinear corners; the catalog's first p1 realizer — the centered-rectangular normalizer {I, R180, Mx, My} broken by the asymmetric staircase (M78) |
| `trapezoid-courses` | two courses of trapezoids per 2×1 cell | pm | rectangular lattice ⟨(2,0),(0,1)⟩ (det 2): four trapezoids, two incongruent shapes (areas 3/5 and 2/5) each with its mirror in x = 1, edge-to-edge (slant slope 1/5); the catalog's first pm realizer — vertical mirror lines x = integer, no horizontal mirrors or half-turns (the incongruent prototile classes kill pmm) (M79) |
| `glide-hexagon-strips` | two interlocking hexagon strips per 1×2 cell | pg | rectangular lattice ⟨(1,0),(0,2)⟩ (det 2): fat (area 29/40) and waist (area 11/40) hexagons, strip 1 the glide image of strip 0 under g(x,y) = (x+1/2, −y) with g² = (1,0); edge-to-edge, five edge directions, no verticals; the catalog's first pg realizer — one horizontal glide family (glide 1/2; axes y ∈ ℤ in two orbits), no mirrors, no rotations (M80) |
| `shifted-trapezoid-courses` | the trapezoid-course row stacked in rows of height 1, alternate rows reflected and shifted | pmg | rectangular lattice ⟨(2,0),(0,2)⟩ (det 4), alternate rows reflected in y = 1 and shifted by (1,0): eight trapezoids, two incongruent shapes × mirror × row-flip; edge-to-edge (slant slope 1/5); the catalog's first pmg realizer — vertical mirrors x ∈ ℤ, one horizontal glide family (glide 1), half-turns at (m+1/2, n); the (1,0) row shift kills the horizontal mirrors that would make it pmm (M81) |
| `wavy-courses` | zigzag courses of height 4/3 (waves of slope ±1/3) | cm | centered-rectangular lattice ⟨(1,1),(2,0)⟩ (det 2; the x+y-even lattice, also square-metric): each column-course quadrilateral split by a diagonal seam alternating slope +1/−1 with column–course parity — four triangles, two incongruent shapes (areas 1/3 and 2/3, squared edges {4,10,18} vs {10,16,18} ×9); edge-to-edge; the catalog's first cm realizer — mirrors x = integer, glides (2j+1−x, y+1) with glide (0,1), no half-turns, no horizontal mirrors (M82) |
| `chiral-cut-honeycomb` | side-2 hexagon cut by a chiral three-armed Y from alternating vertices | p3 | hexagonal cell ⟨(2√3,0),(√3,3)⟩ (det 6√3): three tiles, one chiral congruence class (all seed 0); edge-to-edge (450 interior edges, 0 T-junctions); the catalog's first p3 realizer — 120°/240° rotations about three 3-fold centre orbits (the "333" fingerprint) plus translations only; each arm bends once (Wk = mid(Vk,O) + rotated delta, delta = (−1/8,−1/10) rotating with the arm — straight cuts would be the p6m rhombille) (M83) |
| `split-triangle-grid` | side-2 triangular grid, every small triangle split by its orbit point | p31m | hexagonal cell ⟨(2,0),(1,√3)⟩ (det 2√3): 8 tiles, two congruence classes (inner triangles achiral, corner quads chiral); edge-to-edge (750 interior edges, 0 T-junctions); the catalog's first p31m realizer — mirrors exactly on the grid lines (0°/60°/120° families), rot120 centres in three orbits with the on-mirror incidence (grid-vertex orbit on mirrors, both centroid orbits off) separating p31m from p3m1 (M84) |
| `alcove-cut-grid` | side-2 triangular grid on the index-3 sublattice, every grid triangle cut from its alcove point | p3m1 | hexagonal cell ⟨(3,−√3),(3,√3)⟩ (det 6√3): 18 tiles, three congruence classes (each mirror-closed, achiral as a set); edge-to-edge (1350 interior edges, 0 T-junctions); the catalog's p3m1 realizer — the *333 fingerprint (all three rot120 centre orbits on mirrors, the contrast to M84's p31m one-orbit-on-mirror), mirrors in the 0°/60°/120° families; closes the last gap of the 17-group wallpaper catalog (M85) |

## Build arcs

Numbered arcs keep their PATTERNS.MD §numbers — code comments and
briefs cite them as `§N`.

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
  `√26/5`), each pairing one upper with one lower trapezoid; the
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
  lengths): the slants (slope `±1/5`, length `√26/5`), pairing
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
  2·10² · det Λ, the two-class congruence census, the window partition
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
  three inner seams W_i → O close the Y. Read as a near-rhombille:
  with straight cuts this would be the p6m `rhombille` entry; the
  ±2.83° bend at each W_i breaks every mirror and realizes p3
  instead — the cut lines' slight unevenness is the design. The
  Y cuts the hexagon into 3 congruent hexagonal tiles [O, W0, V0, V1, V2, W2],
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


### 34. Split-triangle grid ✅ (M84)

- **Motif:** a side-2 triangular grid in which every small
  equilateral triangle is split by its **orbit triangle** — a
  generic `Q(√3)` point `P0 = (3/5, −1/40 + 2√3/15)` near a vertex,
  rotated 120° about the triangle's centroid to `P1, P2` — into an
  inner equilateral triangle `(P0, P1, P2)` plus three corner quads
  (one per edge). Each hexagonal cell holds the up-triangle `U =
  (0,0),(2,0),(1,√3)` split directly and the down-triangle `D =
  (1,√3),(3,√3),(2,0)` split by the mirror image of the same
  construction (reflected in `y = 0`, shifted by `a2`). The generic
  `Q(√3)` seed `P0` (on no 30° line) is what breaks the remaining
  6-fold/2-fold structure of the plain grid down to exactly p31m.
  Family: the HX-STUDY chiral grid series (the `hx_p31m_cell.json`
  handoff, PROPOSED §21). The catalog's first **p31m** realizer —
  the sixteenth represented card of the Wallpaper group catalog
  (1 of the 17 groups still un-realized).
- **Group:** **p31m** by structure plus exact probe: translations +
  120°/240° rotations + the three mirror families on the grid lines
  (directions 0°/60°/120°), and nothing else. The ×240 exact-integer
  probe (all 11 D6 linear parts as integer 2L matrices shared with
  the p3 probe, 100 raw cell-vertex candidates per part, deduped by
  the floor coset reduction) is positive in exactly five cosets of
  the translation group — the zero cosets of rot120, rot240, refl0,
  refl60, refl120 — and negative for rot60/rot180/rot300 and the
  30°/90°/150° reflection families (tested counts 43 × 5 and 28 ×
  6); 25/25 lattice translations positive, 0/5 named off-lattice
  negatives. The rot120 centres form exactly three orbits
  `c(i,j) ×240 = (240i, 0, 0, 80(i+2j))`, representatives (0,0),
  (1,√3/3), (2,2√3/3), each with `c − R120·c ∈ Λ`. **Incidence
  discriminator (p31m vs p3m1):** the grid-vertex orbit lies on the
  mirrors while both centroid orbits lie off every mirror
  (on-mirror `[true, false, false]`); p3m1 would put all three
  orbits on mirrors.
- **Lattice & cell:** 8 tiles per **hexagonal** primitive cell,
  basis `a1 = (2,0)`, `a2 = (1,√3)` (|a1| = |a2| = 2, 60°), det
  `2√3` = 2 × triangle area `√3`.
- **Edge structure:** **EDGE-TO-EDGE**: 750 interior edge pairs in
  the standard window, 0 unpaired, 0 bad; clipped coverage 4096 =
  4096, 0 overlaps, 0 gaps. Squared-edge multiset per tile class
  (×240², lex-sorted): triangles (48492,1728) × 3; quads
  (23844,−384) × 2, (48492,1728), (230400,0).
- **Vertex stars:** 186 vertex figures, all summing to 360° (0
  bad), two types — grid-vertex stars (TWELVE corner quads meet:
  `≈18.9442° × 6 + ≈41.0558° × 6`) and inner-triangle-vertex stars
  (three tiles meet: `60°, ≈126.0665°, ≈173.9335°`).
- **Seeds:** `proto` = cell index `[tile0..tile7]`; two congruence
  classes: the inner equilateral triangles `seed = 0` (achiral),
  the corner quads `seed = 1` (chiral) — `seeds [0,1,1,1,0,1,1,1]`.
- **Verify:** standard trio (coverage, rotated-scaled,
  deterministic) + the exact p31m probe
  (`split_triangle_grid_wallpaper_group_is_p31m`, mirroring the
  full Python battery `m84_verify.py`: the 11-part candidate census
  with tested counts 43 × 5 / 28 × 6, positives only at the zero
  coset of rot120/rot240/refl0/refl60/refl120, 25/25 lattice
  translations, 0/5 off-lattice negatives, the three centre orbits
  with `c − R120·c ∈ Λ`, the on-mirror incidence [T,F,F], and the
  cell arithmetic: doubled areas ×240² triangles (2592,24246),
  quads (−864,30318), sum (0,230400) = 2·det·240², squared-edge
  multiset above) + the brute-force culling cross-check + the
  legality test + the Python transcript: the exact-Fraction battery
  reproducing the handoff with zero deltas (area sum 2√3 = det,
  e2e 750/0/0, stars 186/0, coverage 4096/0/0).
 - **Status:** committed (M84). 70 tilings, 525 tests at the commit.


### 35. Alcove-cut grid ✅ (M85)

- **Motif:** a side-2 triangular grid on the index-3 sublattice in
  which every grid triangle is cut from its **alcove point** — the
  orbit of one generic `Q(√3)` junction `P0 = (8/7, 4√3/9)` under the
  words in the reflections in the up-triangle's edge lines — to its
  three vertices (three tiles per grid triangle). Each hexagonal cell
  holds six grid triangles (three up, three down); under the mirror
  arrangement below every grid triangle has trivial stabilizer, so
  decorating one alcove extends equivariantly to the whole tiling.
  Family: the HX-STUDY chiral grid series (the `hx_p3m1_cell.json`
  handoff, PROPOSED §21). The catalog's first **p3m1** realizer —
  the seventeenth represented card of the Wallpaper group catalog;
  **M85 closes the last gap of the 17-group catalog.**
- **Group:** **p3m1** (*333*) by structure plus exact probe:
  translations + 120°/240° rotations + the three mirror families on
  the grid lines (directions 0°/60°/120°), and nothing else. The
  ×126 exact-integer probe (all 11 D6 linear parts as integer 2L
  matrices shared with the p3 probe, 169 raw cell-vertex candidates
  per part (13×13), deduped by the floor coset reduction) is positive
  in exactly five cosets of the translation group — the zero cosets
  of rot120, rot240, refl0, refl60, refl120 — and negative for
  rot60/rot180/rot300 and the 30°/90°/150° reflection families
  (tested counts 69 × 5 and 42 × 6); 25/25 lattice translations
  positive, 0/5 named off-lattice negatives. The rot120 centres form
  exactly three orbits `c(i,j) ×126 = (126(2i+j), 0, 0, 126j)`,
  representatives (0,0), (2,0), (4,0), each with `c − R120·c ∈ Λ`.
  **Incidence discriminator (p3m1 vs p31m):** all three centre orbits
  lie on the mirrors (on-mirror `[true, true, true]`) — the *333
  fingerprint; M84's p31m probe pins the contrast ([T,F,F]: exactly
  one orbit on mirrors).
- **Lattice & cell:** 18 tiles per **hexagonal** primitive cell,
  basis `a1 = (3,−√3)`, `a2 = (3,√3)` (|a1| = |a2| = 2√3, 60°), det
  `6√3` = 3 × the side-2 grid's lattice det `2√3` (index-3
  sublattice; the cell holds six grid triangles).
- **Edge structure:** **EDGE-TO-EDGE**: 1350 interior edge pairs in
  the standard window, 0 unpaired, 0 bad; clipped coverage 4096 =
  4096, 0 overlaps, 0 gaps. Squared-edge multiset per tile class
  (×126², lex-sorted — the imaginary part is 0 on every edge):
  class 0 `(21072,0), (30144,0), (63504,0)`; class 1
  `(15024,0), (21072,0), (63504,0)`; class 2
  `(15024,0), (30144,0), (63504,0)`.
- **Vertex stars:** 245 vertex figures, all summing to 360° (0
  bad), four types — the alcove junctions (three tiles meet:
  `≈104.110°, ≈115.519°, ≈140.372°`) and, one type per centre orbit,
  the grid-vertex stars (twelve tiles meet): `(0,0)` orbit
  `≈26.037° × 6 + ≈33.963° × 6`, `(2,0)` orbit `≈18.073° × 6 +
  ≈41.927° × 6`, `(4,0)` orbit `≈21.555° × 6 + ≈38.444° × 6`.
- **Seeds:** `proto` = cell index `[tile0..tile17]`; three
  congruence classes (six tiles each — the three piece shapes, each
  realized in up- and down-triangle orientations), areas `4√3/9`,
  `13√3/63`, `22√3/63` — `seeds [0,1,2,1,0,2,1,0,2,0,1,2,0,1,2,1,0,2]`.
  Every tile is scalene (chiral individually), but each class is
  mirror-closed under `y = 0` (three mirror pairs) — achiral as a set.
- **Verify:** standard trio (coverage, rotated-scaled,
  deterministic) + the exact p3m1 probe
  (`alcove_cut_grid_wallpaper_group_is_p3m1`, mirroring the full
  Python battery `m85_emit.py`: the 11-part candidate census with
  tested counts 69 × 5 / 42 × 6, positives only at the zero coset of
  rot120/rot240/refl0/refl60/refl120, 25/25 lattice translations,
  0/5 off-lattice negatives, the three centre orbits with
  `c − R120·c ∈ Λ`, the on-mirror incidence [T,T,T], and the cell
  arithmetic: doubled areas ×126² per tile (0,14112) / (0,6552) /
  (0,11088) by class, sum (0,190512) = 2·det·126², squared-edge
  multiset above) + the brute-force culling cross-check + the
  legality test + the Python transcript: the exact-Fraction battery
  reproducing the handoff with zero deltas (area sum 6√3 = det,
  e2e 1350/0/0, stars 245/0, coverage 4096/0/0).
- **Completion step:** the last of the 17 wallpaper groups is now
  represented: `WALLPAPER_GROUP_GAPS` is empty
  (`[WallpaperGroup; 0] = []`) and the shared note
  (`wallpaper_gap_note`), rendered at the top of the Wallpaper tab in
  both UIs, reports the completed catalog instead of a gap ("complete
  catalog: all 17 wallpaper groups have a registered realizer (M85
  closed the p3m1 gap)"). No placeholder cards; the tess-core test
  pins the empty table and the note, the TUI pins the note prefix,
  and the tess-tilings census pins Wallpaper 17.
- **Status:** committed (M85). 71 tilings, 529 tests at the commit.

### 36. Basketweave (2:1 dominoes) ✅ (M39)

- **Motif:** the classical basketweave bond as a `LatticeTiling` data table (`basketweave`, feature `pavers`; family pavers; M39) — the first of the three original paver bonds.
- **Key facts:** 45°-rotated square lattice ⟨(2,2),(2,−2)⟩ (area 8), 2 H + 2 V split dominoes per cell; each long side split into two unit edges (mid-side vertex), so vertex figures carry 180° at the split midpoints (`2.4.4`) and 4.4.4.4 at the even-even block corners; verified exactly in a Python spike (area sum = |det|, edge parity, vertex figures, exact-once coverage, rigorous coset symmetry check); group **p4g**, not the naive p4m guess — 4-folds at every even-even point, mirrors at the odd axes only (no diagonal mirrors), and six glide families (odd-axis vertical/horizontal glides absent: T(2,0)/T(0,2) are not tiling translations) (see CHANGELOG M39).

### 37. Herringbone (2:1 dominoes) ✅ (M40)

- **Motif:** the chiral single-direction herringbone bond as a `LatticeTiling` data table (`herringbone`, feature `pavers`; family pavers; M40).
- **Key facts:** L-pairs H(a,b) + V(a+2,b) at every point of the oblique lattice ⟨(1,3),(2,2)⟩ (area 4), one 2-domino cell; each long side split into two unit edges (180° figures at the split midpoints); verified exactly in the Python spike; group **pgg** as expected for the generic 2:1 case — chiral (no mirrors, no 4-folds), 16 2-fold centers per 4×4 period, two perpendicular glide families (diagonal axes y = x + c, anti-diagonal axes y = −x + c), off-family glides absent (see CHANGELOG M40).

### 38. Running bond (2:1 dominoes) ✅ (M41)

- **Motif:** the stretcher (running) bond as a `LatticeTiling` data table (`running-bond`, feature `pavers`; family pavers; M41) — the first and final of the original trio.
- **Key facts:** rows of 2:1 dominoes offset by half a brick, centered rectangular lattice ⟨(2,0),(1,1)⟩ (det 2), 1 split domino per cell (long sides split into two unit edges, 180° figures at the split midpoints); verified exactly in the Python spike; group **cmm** — mirrors at every integer x = k axis and half-integer y = l + 1/2 axes, 2-folds at (k, l+1/2) (on the mirrors) and (k+1/2, l) (between the rows), two glide families per direction, no diagonal mirrors or 4-folds, off-axis glides absent (see CHANGELOG M41).

### 39. Stack bond (2:1 dominoes) ✅ (M114)

- **Motif:** rows of 2:1 dominoes stacked directly on one another — the running bond without its half-brick offset (`stack-bond`, family pavers; M114).
- **Key facts:** lattice ⟨(2,0),(0,1)⟩ (area 2), one split horizontal domino per cell ([0,2]×[0,1], long sides split into two collinear unit edges — 180° figures at the split midpoints); measured pmm (mirrors x = k and y = n/2, 2-folds at (k, n/2), no glides; coset probe 128-per-coset exact); figures 4.4.4.4 at even-x points, 2.2 at odd-x split midpoints; LatticeTiling data table, same paver feature as §36–§38 (see CHANGELOG M114).

### 40. Herringbone 45° (2:1 dominoes) ✅ (M115)

- **Motif:** the M40 herringbone bond in its 45°-laid canonical frame (`herringbone-45`, family pavers; M115) — same motif as §37, different frame.
- **Key facts:** exact radical geometry via std::f64::consts::SQRT_2; lattice ⟨(0,2√2),(√2,0)⟩ (area 4), two split 45° dominoes per cell (the L-pair); measured pgg — 2-folds at (√2/4 + k√2/2, 3√2/4 + ℓ√2), vertical glides x = k√2/2 (glide √2), horizontal glides y = ℓ√2 + √2/4 (glide √2/2), descended from M40's diagonal/anti-diagonal glides under the 45° rotation; verified in integer P-coords (world = (√2/2)·P): cell area sum 8 = |det|, 3906-point fine grid covered once, 192 interior unit edges each shared by exactly 2, all 113 interior vertex figures 2.4.4; the coset probe uses the new shape-set helper (rotated cells break the axis-aligned bbox probe) plus the independently measured M40 group conjugated by the rotation (see CHANGELOG M115).

### 41. Basketweave three-brick (2:1 dominoes) ✅ (M116)

- **Motif:** the triple-basketweave paver bond — three courses of horizontal 2:1 dominoes stacked without offset, then one course of vertical dominoes, repeating with vertical period 5 (`basketweave-3`, family pavers; M116).
- **Key facts:** lattice ⟨(6,0),(0,5)⟩ (area 30) an index-3 superlattice of the true ⟨(2,0),(0,5)⟩, kept for the whole-cluster cell convention (cf. §36/M39); 15 split dominoes per cell (9 horizontal + 6 vertical, long sides split into collinear unit edges); measured pmm (mirrors x = k and y = 2.5k + 1.5, 2-folds at (k, 2.5k′ + 1.5), no 4-folds, no essential glides); verified exactly in Python before porting: cell area sum 30 = |det|, fine shifted grid covered exactly once (6474 points), 296 interior unit edges each shared by exactly 2, vertex figures sum to 360 (see CHANGELOG M116).

## Per-tiling documents

Each member's full specification — algorithm, exact geometry, verification, sources — lives in its own document under [docs/tilings/](../tilings/):

- [`basketweave`](../tilings/basketweave.md) — Basketweave (2:1 dominoes)
- [`herringbone`](../tilings/herringbone.md) — Herringbone (2:1 dominoes)
- [`running-bond`](../tilings/running-bond.md) — Running bond (2:1 dominoes)
- [`stack-bond`](../tilings/stack-bond.md) — Stack bond (2:1 dominoes)
- [`herringbone-45`](../tilings/herringbone-45.md) — Herringbone 45° (2:1 dominoes)
- [`basketweave-3`](../tilings/basketweave-3.md) — Basketweave three-brick (2:1 dominoes)
- [`stepped-hexominoes`](../tilings/stepped-hexominoes.md) — Stepped hexominoes
- [`trapezoid-courses`](../tilings/trapezoid-courses.md) — Trapezoid courses
- [`glide-hexagon-strips`](../tilings/glide-hexagon-strips.md) — Glide hexagon strips
- [`shifted-trapezoid-courses`](../tilings/shifted-trapezoid-courses.md) — Shifted trapezoid courses
- [`wavy-courses`](../tilings/wavy-courses.md) — Wavy courses
- [`chiral-cut-honeycomb`](../tilings/chiral-cut-honeycomb.md) — Chiral-cut honeycomb
- [`split-triangle-grid`](../tilings/split-triangle-grid.md) — Split-triangle grid
- [`alcove-cut-grid`](../tilings/alcove-cut-grid.md) — Alcove-cut grid

---

*Map: [docs/INDEX.md](../INDEX.md) · Changelog: [docs/history/CHANGELOG.md](../history/CHANGELOG.md) · Queue: [docs/ROADMAP.md](../ROADMAP.md)*
