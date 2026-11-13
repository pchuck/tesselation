# PROPOSED.MD

Researched candidate backlog: additional tiling patterns worth adding to
tessellix, each with a verified literature basis, a construction path in
this codebase, an exact-arithmetic plan, and an effort estimate. This is
the research dossier behind the 🔭/❓ markers in PATTERNS.MD — that file
remains the status tracker (✅ implemented / 🚧 in progress / 🔭 proposed /
❓ research); when an entry here is adopted it gets an `M<n>` commit per
AGENTS.md, its status flips in PATTERNS.MD, and this entry is annotated.

Research method (2026-11-12): every candidate below was checked against
Wikipedia / primary papers / the Bielefeld Tilings Encyclopedia for
canonical status and a known construction path before inclusion. Claims
that still need a design-time check are marked **verify**. Nothing here
is "would be nice" without at least a known construction path.

Categories are the declared `TilingCategory` values (tess-core
`category.rs`). Uniform (11) and Laves (8+1) are **closed** — everything
new lands in Wallpaper, Quasiperiodic, or Aperiodic (or, for the
hyperbolic arc, a future category variant).

Effort: **S** ≤ half a session (table + standard battery) · **M** = 1–2
sessions · **L** = multi-session · **XL** = arc (design doc first).

## Summary

| # | candidate | category | path | effort | tier |
|---|---|---|---|---|---|
| 1 | Paver trio (basketweave, herringbone, running bond) | Wallpaper | `LatticeTiling` tables | S ×3 | P0 |
| 2 | Hexagram-and-hexagon ({6/2} star-and-cross) | Wallpaper | `LatticeTiling` table | S–M | P0 |
| 3 | Windmill (periodic pinwheel cousin) | Wallpaper | table or direct | S–M | P0 |
| 4 | 2-uniform tilings (20 types, first batch) | Wallpaper | `LatticeTiling` tables | M each | P0 |
| 5 | Pentagon census (types 1–15) | Wallpaper | tables + 2 direct | S–L each | P1 |
| 6 | Stampfli 12-fold (2+√3) | Quasiperiodic | substitution | M–L | P1 |
| 7 | Golden bee / Ammann chair (A2, √φ) | Quasiperiodic* | substitution | M–L | P1 |
| 8 | Socolar / shield 12-fold | Quasiperiodic | substitution | L | P1 |
| 9 | Lançon–Billard binary tiling | Quasiperiodic | substitution | M | P1 |
| 10 | Rep-tile siblings: T-tetromino, P-pentomino | Aperiodic | integer substitution | S–M each | P1 |
| 11 | Jeandel–Rao 11 Wang tiles | Aperiodic | Sturmian/substitution | L | P1 |
| 12 | Trilobite and crab (2-tile aperiodic set) | Aperiodic | L-substitution | L | P1 |
| 13 | Isohedral (IH) parameterized generator | Wallpaper | new generator class | XL | P2 |
| 14 | Hankin polygons-in-contact strapwork engine | Wallpaper | rendering/generator | XL | P2 |
| 15 | Random/statistical tilings (Truchet et al.) | — | needs determinism policy | M each | P2 |
| 16 | Hyperbolic {p,q} tilings | new category | geometry-core arc | XL | P2 |
| 17 | Socolar–Taylor monotile (half-hex hierarchy) | Aperiodic | substitution + stripes | L–XL | P2 |
| 18 | n-fold rhomb family (7-fold GS/Harriss, CAST) | Quasiperiodic | degree-3 arithmetic | L+ | P2 |
| 19 | Second measured ornamental panel | Wallpaper | M35 pipeline | M–L | P2 |
| 20 | Fractalizing k-uniform (2+√3 dodecagon) | Quasiperiodic | substitution on uniform basis | M | P2 |

\* golden-bee tilings are nonperiodic and (canonical form) quasiperiodic;
confirm the diffraction claim during the spike before declaring.

---

## P0 — quick wins (data tables, standard battery only)

### 1. Paver trio: basketweave, herringbone, running bond

- **Motif:** the three canonical brick/paver bonds in 2:1 rectangles —
  basketweave (2×2 blocks of alternating dominoes), herringbone
  (rotated pairs), running bond (stretcher rows, half offset). Herringbone
  and basketweave are among the most-recognized real-world patterns not
  yet in the registry; running bond is its plain sibling.
- **Expected groups (verify with the pythagorean-style symmetry probe):**
  basketweave p4m (90° rotation swaps block orientation), herringbone
  pgg (glide structure; the generic-ratio case), running bond cmm
  (half-offset rows).
- **Construction:** pure `LatticeTiling` tables in a shared `pavers.rs`
  (feature `pavers`); 2:1 rectangles only, so every vertex is exact
  rationals. Cells: basketweave 4×4 with 8 dominoes; herringbone an
  L-shaped 2-brick cell; running bond 2 bricks. Pattern seeds = brick
  orientation classes.
- **Verify:** the standard ±3×±3 partition battery (area sum, edge
  parity 2, vertex figures, shifted-grid coverage); culling list
  extension; census test update. Herringbone/running-bond cells are
  **not edge-to-edge** at brick scale in the naive decomposition — pick
  the cell decomposition that is (split rectangles so every interior
  edge is shared), exactly like the pythagorean T-junction lesson.
- **Why now:** cheapest possible new entries; good first issues for the
  post-category-grouping workflow; fills the "Wallpaper (5)" section.
- ✅ **M39 delivered (2026-11-12): basketweave** — `basketweave.rs`
  (feature `pavers`), 45°-rotated square lattice ⟨(2,2),(2,−2)⟩ with 4
  split dominoes per cell; long sides split into two unit edges so the
  emitted polygons are edge-to-edge at unit scale (180° figures at the
  split midpoints). Verified group is **p4g**, *not* the expected p4m
  above: the 4-fold centers sit at every even-even point, but the
  mirrors run only along the odd axes `x = 2k+1` / `y = 2k+1` —
  offset from every 4-fold center, with no diagonal mirrors (the 90°
  swap sends the mirror set to the even axes, which are not mirrors).
  Exact coset symmetry check, forbidden list included; see the M39
  changelog entry in PATTERNS.MD.
- ✅ **M40 delivered (2026-11-12): herringbone** — `herringbone.rs`
  (feature `pavers`), chiral single-direction L-pairs
  `H(a,b) + V(a+2,b)` on the oblique lattice ⟨(1,3),(2,2)⟩ (area 4),
  2 split dominoes per cell. Verified group is **pgg** as expected
  for the generic 2:1 case: no mirrors and no 4-folds (chiral), 16
  2-fold centers per 4×4 period, two perpendicular glide families —
  diagonal axes `y = x + c` and anti-diagonal axes `y = −x + c` — with
  the off-family glides `(1−y,1−x)`, `(−x,y+1)`, `(x+1,−y)` absent.
  Exact coset symmetry check, forbidden list included; see the M40
  changelog entry in PATTERNS.MD.
- ✅ **M41 delivered (2026-11-12): running bond — trio complete** —
  `running_bond.rs` (feature `pavers`), stretcher rows of 2:1 dominoes
  offset by half a brick on the rectangular lattice ⟨(2,0),(0,2)⟩,
  2 split dominoes per cell. Verified group is **cmm** as expected:
  mirrors at every integer `x = k` axis and at the half-integer
  `y = l + 1/2` axes, 2-folds at `(k, l+1/2)` (on the mirrors) and
  `(k+1/2, l)` (between the rows), two glide families per direction
  (reps `(x+1,−y)`, `(x+1,4−y)`, `(1−x,y+1)`, `(3−x,y+1)`), no
  diagonal mirrors, no 4-folds, and the off-axis glides
  `(x+1,1−y)`, `(−x,y+1)` absent. Exact coset symmetry check,
  forbidden list included; see the M41 changelog entry in PATTERNS.MD.

### 2. Hexagram-and-hexagon ({6/2} star-and-cross)

- **Motif:** the Star-of-David star-and-cross on the triangular lattice —
  the 6-pointed sibling of the implemented {8/3} `star-cross` (M15/M36)
  and the 10-pointed girih family (M18/M35). Completes the classical
  6/8/10 star-and-cross trio and points at a 12-point follow-up (#14).
- **Construction:** one-parameter family like star-cross (star tip
  radius vs lattice spacing: gapped → kissing → interlaced), exact in
  the `H = √3/2` triangular-lattice constants; expected group p6m
  (**verify** the kissing threshold and group numerically). Cell
  decomposition (hexagrams, hexagons, triangles) fixed in the exact-
  geometry spike; same emission-orbit gotcha as 3.12.12 applies where
  triangles are shared.
- **Verify:** standard battery + culling list; a `star-cross`-style
  parameter sweep test (coverage at the critical radii).
- **Status:** implemented (M42).

### 3. Windmill tiling

- **Motif:** the periodic arrangement of 1–2–√5 right triangles around
  squares (Conway's "spin" of the pinwheel motif) — the periodic
  companion to the aperiodic `pinwheel`, reusing its triangle verbatim.
- **Construction:** direct `Tiling` impl or small table; Q(√5) not
  needed — triangle vertices are integer pairs rotated by arctan(1/2)
  only for the *aperiodic* version; the periodic windmill uses fixed
  orientations, so exact integer coordinates + one rotation constant.
  **verify** the canonical name/reference for this arrangement during
  the spike (candidate: p4g tiling by pinwheel triangles + squares) and
  confirm the vertex figure by hand before coding (AGENTS.md step 4).
- **Verify:** standard trio + symmetry probe for the declared group.
- **Status:** implemented (M43) — naming-rule waiver approved by repo owner 2026-11-12

### 4. 2-uniform tilings — first batch of the closed set of 20

- **Motif:** the 2-isogonal (demiregular) tilings: edge-to-edge tilings
  by regular polygons with exactly two vertex orbits (Krotenheerdt's
  complete list; Grünbaum–Shephard 1987). The natural successor to the
  completed Laves arc, and every one of them is a finite data table on
  the existing uniform lattices.
- **The closed list (Wikipedia, List of k-uniform tilings; t = tile
  orbits, e = edge orbits):**

  | group | vertex configurations | t,e |
  |---|---|---|
  | p6m | 3⁶ ; 3².4.3.4 | 3,3 |
  | p6m | 3.4.6.4 ; 3².4.3.4 | 4,4 |
  | p6m | 3.4.6.4 ; 3³.4² | 4,4 |
  | p6m | 3.4.6.4 ; 3.4².6 | 5,5 |
  | p6m | 4.6.12 ; 3.4.6.4 (the 3-4-6-12 tiling) | 4,4 |
  | p6m | 3⁶ ; 3².4.12 | 4,4 |
  | p4m | 3.12.12 ; 3.4.3.12 | 3,3 |
  | p6m | 3⁶ ; 3².6² | 2,3 |
  | p6  | 3⁶ ; 3.4.6 (variant 1) | 3,3 |
  | p6  | 3⁶ ; 3.4.6 (variant 2) | 5,7 |
  | cmm | 3².6² ; 3.4.6 | 2,4 |
  | pmm | 3.6.3.6 ; 3².6² | 2,3 |
  | cmm | 3.4².6 ; 3.6.3.6 (variant 2) | 3,4 |
  | pmm | 3.4².6 ; 3.6.3.6 (variant 1) | 4,4 |
  | p4g | 3³.4² ; 3².4.3.4 (variant 1) | 4,5 |
  | pgg | 3³.4² ; 3².4.3.4 (variant 2) | 3,6 |
  | cmm | 4⁴ ; 3³.4² (variant 1) | 2,4 |
  | cmm | 4⁴ ; 3³.4² (variant 2) | 3,5 |
  | pmm | 3⁶ ; 3³.4² (variant 1) | 3,4 |
  | cmm | 3⁶ ; 3³.4² (variant 2) | 4,5 |

- **Construction:** `LatticeTiling` tables like the Archimedean ones —
  each is a supercell of an existing uniform lattice (supercell areas
  2–12 primitives). Start with the six p6m entries (hexagonal lattice
  machinery already exact), then p4m/p4g/pgg. Pattern seeds by
  polygon type first (repo convention), vertex-orbit coloring as the
  alternative seed where it differs.
- **Scope note:** all 20 is the arc goal (like Laves 8/8); the first
  batch should be 3–5 tables. Duals of 2-uniform tilings are a cheap
  bonus family later (mixed floret/cairo-like duals) via the
  dual-construction helper (PATTERNS.MD research area 2).
- **Verify:** the standard ±3×±3 battery per table (the vertex-figure
  check must verify *both* orbit figures, and the two-orbit structure
  itself — assert exactly 2 vertex classes over a large patch), plus
  the culling list.
- **Status:** implemented (M44 + M48):
  the p6m entry 3⁶ ; 3².6² as `kuniform-p6m` (index-3 honeycomb
  dissection, exact Q(√3) table, both vertex orbits + p6m
  generators verified by an exact-surd Python spike and Rust
  cell-set probes), and the pmm entry 3.6.3.6 ; 3².6² as
  `kuniform-pmm` (hexagons on the rectangular lattice
  span{(2,0),(0,√3)} with up- and down-pointing triangle rows in
  the gaps, exact Q(√3) table, both vertex orbits + pmm
  generators verified the same way).

---

## P1 — sprints

### 5. Convex pentagon census — the complete 15 types

- **Motif:** the 15 types of convex pentagonal tilings (Reinhardt 1918
  → Mann–McLoud–Von Derau 2015; completeness proved by Rao 2017). The
  repo already owns several members unknowingly; this arc makes the
  census explicit and complete, mirroring the Laves/Archimedean arcs.
- **Current coverage (verify type assignments at design time):**
  `cairo-pentagonal` / `cairo-collinear` = type 4 family; likely
  type 1 (`prismatic-pentagonal`, cmm topology) and type 5
  (`floret-pentagonal`, p6 six-tile rosette); `hirschhorn` almost
  certainly type 8 (angle multiset {60,80,100,140,160} matches
  2B+C = D+2E = 360°, b=c=d=e).
- **The type conditions (Wikipedia, sides a..e vs angles A..E):**

  | type | conditions | group | unit | isohedral | e2e |
  |---|---|---|---|---|---|
  | 1 | B+C=180°, A+D+E=360° | p2/cmm/cm/pmg/pgg/p1 topologies | 2–4 | iso–3-iso | yes |
  | 2 | c=e, B+D=180° | pgg | 4 | iso | yes |
  | 3 | a=b, d=c+e, A=C=D=120° | p3 / p31m | 3 | iso | yes |
  | 4 | b=c, d=e, B=D=90° | p4 / p4g | 4 | iso | yes |
  | 5 | a=b, d=e, A=60°, D=120° | p6 | 6 | iso | yes |
  | 6 | a=d=e, b=c, B+D=180°, 2B=E | p2 | 4 | 2-iso | yes |
  | 7 | b=c=d=e, B+2E=2C+D=360° | pgg/p2 | 8 | 2-iso | yes |
  | 8 | b=c=d=e, 2B+C=D+2E=360° | pgg/p2 | 8 | 2-iso | yes |
  | 9 | b=c=d=e, 2A+C=D+2E=360° | pgg/p2 | 8 | 2-iso | yes |
  | 10 | a=b=c+e, A=90°, B+E=180°, B+2C=360° | p2 / cmm | 6 | 3-iso | **no** |
  | 11 | 2a+c=d=e, A=90°, 2B+C=360°, C+E=180° | pgg/p2 | 8 | 2-iso | no |
  | 12 | 2a=d=c+e, A=90°, 2B+C=360°, C+E=180° | pgg/p2 | 8 | 2-iso | no |
  | 13 | d=2a=2e, B=E=90°, 2A+D=360° | pgg/p2 | 8 | 2-iso | no |
  | 14 | 2a=2c=d=e, A=90°, B≈145.34°, C≈69.32°, D≈124.66°, E≈110.68° | p2 | 6 | 3-iso | **no** |
  | 15 | a=c=e, b=2a, d=(a+√2)/(√3−1), A=150°, B=60°, C=135°, D=105°, E=90° | pgg/p2 | 12 | 3-iso | **no** |

- **Priorities within the arc:** the fixed-shape showstoppers first —
  **type 14** (Stein 1985; exact proportions b/a =
  √((11√57−25)/8), sin B = (√57−3)/8 — a Q(√57) tiling, spectacular),
  **type 15** (MMVD 2015; the last-discovered convex pentagon; exact
  surd field Q(√2,√3)-flavored — **verify** the exact d form), and
  **type 13** (Rice; d=2a=2e right-angle pentagon, integer-friendly).
  Then the parameterized pgg/p2 families (7, 9, 11, 12) with one
  canonical default parameter set each, then 2/3/5/10, then type 1
  topologies (pick the classic p2 2-tile primitive unit first).
- **Construction:** data tables for the fixed and 2-tile-unit types;
  direct impls or bigger tables for the 8/12-tile 2-isohedral units
  (chirality pairs → pattern seeds). Non-edge-to-edge types (10, 11,
  12, 13, 14, 15 — Rice's type 9 is the only edge-to-edge member of
  her four) follow the chair/pinwheel documentation policy (assert
  refined edge census instead of edge parity).
- **Verify:** standard battery + explicit vertex-figure/angle-multiset
  census per type + chirality-pair counting where p2-underlying.
- **Effort:** S for type 13; M for most; L for 14/15 exact fields.
- **References:** Wikipedia "Pentagonal tiling" (15-type table +
  wallpaper groups); Rao 2017 completeness; Bagina 2011
  (8 edge-to-edge types); arXiv:2501.07090 (monotiles per type family).
- **Status:** type 14 (Stein 1985) implemented (M50) — the completely
  determined shape (2a = 2c = d = e, b = βa = √((11√57−25)/8 · a²),
  A = 90°), exact Q(√57, β) 6-tile oblique unit, 3-isohedral,
  group p2 (2222). Type 13 (Rice) implemented (M49) — canonical
  A = 120° instance, exact Q(√3) 8-tile unit, group p2.
  **Next: type 15 (MMVD 2015) = M55, in progress** — the other
  fixed-shape showstopper; dossier row above (a = c = e, b = 2a,
  d = (a+√2)/(√3−1), 150°/60°/135°/105°/90°), exact field expected
  Q(√2, √3) — **verify the exact d form and the group (pgg vs p2)
  by exact probe**; 12-tile primitive cell, 3-isohedral.
  Remaining after 15 (M56+), in the arc's priority order:
  then 7/9/11/12 (parameterized 2-iso families), then
  2/3/5/10, then 1 (p2 topology). Existing tilings likely cover 4
  (cairo), 1 (prismatic-pentagonal), 5 (floret), 8 (hirschhorn) —
  verify the assignments at design time; implement only uncovered
  types. One type per task, delegated; all `pentagon1..15` features
  pre-created (types 4/8 map to cairo/hirschhorn pending verification).

### 6. Stampfli 12-fold quasiperiodic tiling (inflation 2+√3)

- **Motif:** the classic dodecagonal quasiperiodic tiling of squares,
  equilateral triangles and 30° rhombi with quasiperiodically repeated
  12-fold rosettes (Stampfli & Schaad, Bridges 2021). The dodecagonal
  sibling of the implemented Penrose (5-fold), Ammann–Beenker (8-fold)
  and girih Darb-i Imam (10-fold) substitution entries — this one
  entry completes the classical rotation-symmetry series 5/8/10/12.
- **Construction:** substitution with inflation factor 2+√3 derived
  from the 12-rhomb/triangle rosette (the paper gives the dissections;
  rule variants exist — pick the mirror-symmetric canonical one).
  Exact arithmetic in Q(√3) (all prototile vertices on the 30° grid);
  machinery mirrors `ammann_beenker.rs` (seed rosette → deflation
  growth → per-step culling → edge census). PF-law check: inflated
  area n_s + n_r/2 + (√3/4)n_t must equal (2+√3)² = 7+4√3 times the
  parent area.
- **Verify:** exact-Q(√3) Python battery first (per-parent area/
  coverage/sub-edge parity, edge-to-edge growth to depth 4, 12-fold
  symmetry of the seed rosette), Python↔Rust table diff (caught real
  bugs twice before), standard trio.
- **Stretch siblings:** the 1+√3 family (Schaad–Stampfli,
  arXiv:2102.06046 — half-triangle prototiles, 9 recombination rules);
  Frettlöh's Rorschach tiling (1+√3, non-unit PV — harder, park). Any
  n+m√3 inflation admits 12-fold quasiperiodic tilings (the paper's
  closing remark) — the 2+√3 entry is the canonical first.
- **References:** Bridges 2021 paper (stampfliSchaad.pdf);
  arXiv:2102.06046; tilings.math.uni-bielefeld.de 12-fold entries
  (Socolar, Schlottmann, Watanabe).
- **Status:** implemented on branch stampfli (pending integration);
  committed as M45.

### 7. Golden bee / Ammann chair (Ammann A2, inflation √φ)

- **Motif:** the golden bee — a concave hexagon (five right angles and
  one reflex) that dissects into **two smaller similar copies of
  unequal size** (scale factors φ^−1/2 and φ^−1; area fractions 1/φ,
  1/φ²). Found by Robert Ammann (the Tilings Encyclopedia's "Ammann
  Chair", his aperiodic set A2), independently rediscovered by Karl
  Scherer; the only polygon other than non-isosceles right triangles
  that partitions into a non-congruent similar pair (Vince 2014).
  The canonical golden-bee tilings are self-similar, nonperiodic and
  quasiperiodic, with uncountably many variants (Vince, Amer. Math.
  Monthly 121 (2014) "Self-Similar Polygonal Tiling"; structure of A2
  tilings: Durand–Shen–Vereshchagin, DCG 63 (2020)).
- **Why it fits:** the first **irreptile** substitution entry (mixed
  child scales) — a genuinely new machinery variant beyond the
  chair/sphinx integer playbook, but still two prototiles and a PV
  inflation (√φ is the dominant root of x⁴−x²−1, a Pisot number).
  Two sizes of one shape also make pattern coloring trivial (seed =
  size class).
- **Construction:** substitution "scale by √φ, split each inflated
  large tile into L + S" (Vince Fig. 4); depth from view extent as
  usual, but **octave anchoring needs rethinking** — the per-step
  scale is √φ, so tile sizes form a 2-, 3-, ...-adic hierarchy, not
  pure powers of 2. Exact arithmetic in Q(√φ) (degree 4; basis
  {1, φ, √φ, φ√φ} integer 4-tuples — the girih_qc fixed-point trick).
- **Verify:** exact-field Python battery (two-piece dissection exact,
  area law, boundary partitions), growth to depth 4–5, nested-view
  bit-identity under the new anchoring, standard trio.
- **Open (declare at commit):** Quasiperiodic vs Aperiodic — Vince
  proves the canonical sequence quasiperiodic; if the diffraction
  claim stays outside our verification reach, declare Aperiodic with
  a documented note (chair precedent).

### 8. Socolar 12-fold (and the shield tiling)

- **Sprint allocation (2026-11-13):** the shield tiling (Gähler) is
  the first deliverable (M51, delegated, feature `socolar`); Socolar's
  own 1989 rule table and the wheel tiling (MLD duals) follow if the
  class lands well.

- **Motif:** Socolar's 1989 dodecagonal quasicrystal tiling — three
  prototiles (30° rhombus, square, regular hexagon) with a published
  substitution (Tilings Encyclopedia "Socolar"; Socolar, Phys. Rev. B
  39 (1989) 10519). Its MLD-class siblings: the **shield tiling**
  (Gähler; irregular hexagon ("shield") + square + two triangles,
  inflation √(2+√3) — the smallest dodecagonal inflation known) and
  the **wheel tiling** (Nissen) — wheel ↔ shield is the edge-
  orthogonal dual map (each edge replaced by the equal-length edge
  orthogonal at its midpoint, Bielefeld "MLD Class Shield and
  Socolar"), so implementing one directly yields the other.
- **Construction:** substitution tables from the encyclopedia rule;
  Q(√3) for Socolar; shield lands in Q(√2,√3) (√(2+√3) = (√6+√2)/2)
  — degree-4 arithmetic, heavier than AB, comparable to golden bee.
- **Verify:** PF-matrix area law from the encyclopedia's substitution
  matrix, edge-to-edge growth battery, 12-fold seed symmetry, table
  diff, standard trio. Do **one** of the MLD class first (Socolar:
  published rule table; shield: prettier prototiles) and derive the
  second from the MLD map as a follow-up commit.

### 9. Lançon–Billard binary tiling

- **Motif:** the golden-mean rhomb tiling with two rhombus shapes in
  two size classes (36° and 72° rhombi, "binary tiling" in the
  substitution literature; Lançon–Billard 1988; Pautze's CAST survey
  arXiv:1606.06858 lists it as the φ-case of cyclotomic aperiodic
  substitution tilings). Visually distinct from Penrose despite the
  shared φ-inflation family — two rhombus *shapes* instead of one.
- **Construction:** substitution rule from the Bielefeld/Lançon–Billard
  line; Q(φ) arithmetic (lightest possible quasiperiodic field);
  reuses the penrose.rs P3 machinery almost wholesale (Robinson-
  triangle-style half-tile bookkeeping if the rule is stated on
  halves — **verify** the rule's canonical form at spike time).
- **Verify:** penrose-style battery (exact φ placements, growth,
  edge census) + standard trio.

### 10. Rep-tile siblings: T-tetromino, P-pentomino

- **Motif:** the two remaining classical rep-4 polyominoes beyond the
  implemented chair (L-tromino): the **T-tetromino** and the
  **P-pentomino**, both rep-4 at λ = 2 (MathWorld irreptile table:
  L-triomino, L-tetromino, P-pentomino all 4 × scale-1/2). Cheap,
  charming siblings of chair.rs; same limit-periodic structure.
  (Note: the **X-pentomino is not a rep-tile** — a rep-tile must cover
  a corner of its hull — rejected; see the park list.)
- **Construction:** chair.rs playbook verbatim: integer grid, rule
  derived by exhaustive exact enumeration of the inflated tile
  (`chair_verify.py` pattern — the T's dissection may not be unique;
  enumerate, pick and document one canonical dissection, assert its
  stability under all orientations), 4×k seed block search, octave
  anchoring, non-edge-to-edge census.
- **Verify:** chair-style battery (substitution-tiles-parent census,
  growth laws, T-junction census, octave stability) + standard trio.
- **Effort:** S–M each; a natural single commit each or one combined.
- **Status (branch `reptiles`):** **P-pentomino ✅ implemented** (M47,
  pending integration): exhaustive enumeration finds exactly two
  dissections of the inflated 20-cell P (class multisets {0,1,4,6}
  and {3,4,4,5}); the P is chiral, so the dissection necessarily mixes
  chiralities (8 orientation classes, doubly stochastic substitution
  matrix); non-edge-to-edge (5 T-junctions per parent); 5×8 seed
  block with one tile per class; octave-anchored integer-grid
  generation like chair. **T-tetromino ❌ dropped: proven *not*
  rep-4** — exhaustive exact search finds 0 tilings of the inflated
  16-cell T by four T's; the corner cell (0,0) is coverable only by
  the bar (0,0),(1,0),(2,0) + stem (1,1), which isolates cell (0,1)
  (neighbours (0,0),(1,1) taken, (0,2) outside the region). T-tetromino
  appears in no rep-tile list (Wikipedia "Rep-tile": the axis-parallel
  rep-n² polyominoes are the L-tromino, L-tetromino and P-pentomino).

### 11. Jeandel–Rao aperiodic set of 11 Wang tiles

- **Motif:** the smallest known aperiodic Wang tile set — 11 tiles
  over 4 colors, proven minimal by exhaustive search (Jeandel & Rao,
  "An aperiodic set of 11 Wang tiles", Adv. Combin. 2021). Historical
  context in-repo: Berger 20,426 → Robinson 56 → Kari 14 → Culik 13
  (Kari–Culik, 5 colors; aperiodicity is number-theoretic — the
  2ᵃ≠3ᵇ argument, no known hierarchical proof) → Jeandel–Rao 11,
  minimal. A Wang entry would be tessellix's first edge-*colored*
  square-tile family and the natural testbed for the Ammann-bar /
  stripe-decoration layer (PATTERNS.MD research area 4).
- **Construction:** tiles are unit squares (polygon trivial); the work
  is the *generation policy*: (a) deterministic Sturmian/rotation-
  sequence rows — Kari–Culik tilings arise from Beatty sequences and
  the JR space admits substitution/sofic descriptions (Labbé;
  **verify** the exact canonical generation reference at spike time);
  (b) the existing App machinery is stateless-per-params, so a
  deterministic construction (no RNG) is required — seeded-RNG
  patches belong to #15's policy work instead.
- **Verify:** adjacency legality of the emitted patch (all shared
  edges color-matched) as a new standard test for this family;
  patch grows with the view; determinism trio unchanged.
- **Flags:** 11 prototiles > 8-color palette → extend the palette or
  color by edge-color multiset; TUI/GUI need no changes (tiles are
  polygons; seeds carry the classes).

### 12. Trilobite and crab (Goodman-Strauss's 2-tile aperiodic set)

- **Motif:** one of the smallest known aperiodic sets: two tiles —
  "trilobite" and "crab" — enforcing the L-substitution hierarchy
  (Goodman-Strauss, "A small aperiodic set of planar tiles"). Pairs
  naturally with the implemented chair: the underlying structure is
  the L-substitution (the chair's own substitution), with the two
  compound prototiles marking the hierarchy.
- **Construction:** generate via the L-substitution patches (chair
  machinery), then classify/substitute trilobite and crab placements
  per the paper's marking rules; the emitted prototiles are compound
  polyomino-ish shapes — **verify** the canonical tile shapes and
  marking system against the paper at spike time (the fiddliest part).
- **Verify:** chair-style growth battery on the underlying hierarchy +
  an isohedral-count/trichotomy test (patches contain only the two
  prototiles in legal configurations) + standard trio.

---

## P2 — arcs and research (design doc before code)

### 13. Isohedral (IH) parameterized generator — the Escher arc

- **Motif:** the 93 isohedral tiling types IH1–IH93 (Grünbaum–Shephard;
  81 unmarked + 12 marked-only): every periodic monohedral tiling
  type, parameterized — the machinery behind "Escher-style" interlock-
  ing decorative tiles (Escher's own classification; Kaplan's
  isohedral.ca publishes the full incidence-symbol + parameter tables;
  Tom McLean's per-type templates, mirrored at jaapsch.net, give the
  edge-shaping conventions).
- **Design sketch:** each IH type = incidence symbol (tile symbol +
  adjacency symbol) over one of the 11 Laves *topologies* — which the
  repo already implements exactly. Generator inputs: IH type + edge
  shape parameters + tile proportions; shaped edges become **exact
  parameterized polylines** shared per edge orbit (both sides of an
  edge emit bit-identical point sequences), so tiles stay polygons and
  the coverage/determinism contracts hold unchanged. Category
  Wallpaper; each instantiated type is a `Tiling` (or a compact
  `IhTiling` struct) declaring its group.
- **Scope:** start with 3–5 showcase types (e.g. IH1 p1 hexagon,
  IH5-IH7 pgg/p4g pentagon interlocks, IH47-ish hexagon ring), not all
  93; the arc completes the census later. Needs a small design doc
  (edge-orbit data model, parameter bounds per type) before code.

### 14. Hankin polygons-in-contact strapwork engine ✅ (M53 fourfold + M54 sixfold + M56 twelvefold tilings + M57 layer mode)

- **Sprint allocation (2026-11-13):** second deliverable of the girih
  extension arc (M53, delegated to the strong track, feature
  `hankin_ext`). Design-first per the question below; for the first
  concrete deliverable prefer **tiling mode** on one substrate (the
  strapwork partition as a registry tiling, M35 precedent), with the
  general layer mode documented as the follow-up direction. Suggested
  substrate: 4.8.8 (fourfold khatam family), distinct from the just-
  shipped 12-point W85 panel. **Follow-up (M54 ✅, same feature
  `hankin_ext`):** PIC tiling mode on the 3.6.3.6 substrate (sixfold
  khatam family, exact Q(√3)) — second substrate proves the method
  generalizes. **Third substrate (M56 ✅, same feature):**
  the 3.12.12 truncated-hexagonal substrate — completes Kaplan's
  canonical trio (4.8.8, 3.6.3.6, 3.12.12) and yields the twelvefold
  rosette family; exact Q(√3); δ forced to 60°, group p6m verified.
- **Layer-mode design (M57, coordinator decisions 2026-11-13):**
  - Engine in **tess-core** `strapwork.rs`: `walls(tiles, δ) ->
    Vec<(Vec2, Vec2)>` — per edge midpoint, one ray per adjacent tile
    at contact angle δ; rays terminate at ray–ray intersections or the
    tile boundary. Pure function of (tiles, δ) — fits the determinism
    policy. f64 geometry (hosts are arbitrary); exactness is asserted
    only where the host is exact.
  - **Verification anchor:** for a regular registry substrate, layer
    walls at the canonical δ must equal the corresponding registered
    PIC tiling's internal edges exactly (1e-9): 4.8.8@45° ↔ `pic-4-8-8`,
    3.6.3.6@60° ↔ `pic-3-6-3-6`, 3.12.12@60° ↔ `pic-3-12-12`. Three
    cross-check tests in tess-tilings tie layer mode to the proven
    tiling mode.
  - **Rendering:** SVG only this round — `export --strapwork <δ>`
    strokes the wall network over the tile fill (border color, width
    ∝ view). TUI/GUI live overlay deferred (documented follow-up).
  - **No new registry tiling:** layer mode is a rendering/generator
    facility; the built-in count stays 53. Docs: README export
    section + PATTERNS.MD M57 changelog + this entry's Done bullet.
- **Done (M53):** the design question is answered **tiling mode**: the
  strapwork region partition is itself a face-to-face periodic tiling,
  so `pic-4-8-8` (`pic_4_8_8.rs`) registers it directly (M35 precedent)
  — Hankin PIC at contact angle δ = 45° on the 4.8.8 substrate (the
  *median* fourfold star): every ray runs midpoint-to-midpoint, so the
  walls are the opposite-midpoint chords — the {8/2} two-square octagram
  in each octagon, the four midpoint chords in each substrate square —
  partitioning the plane into 14 faces per `A·Z²` cell (regular octagon
  1+√2, small square 1/2, 4 house pentagons 1/8+√2/4, 8 star tips 1/8;
  Σ = 3+2√2 = |det| exactly), every vertex a (p+q√2)/4 surd. The
  un-interlaced partition verifies exactly **p4m** (positive rot90/mirror
  probes pass, quarter-step rot90, half-step rot180 and the glide all
  fail). The general **layer mode** (wall network over an arbitrary
  host substrate without registering a partition, one engine × every
  substrate × acute/median/obtuse openings) stays the documented
  follow-up direction.
- **Done (M54):** the method generalizes — `pic-3-6-3-6`
  (`pic_3_6_3_6.rs`, same `hankin_ext` feature) lays the engine on the
  trihexagonal 3.6.3.6 substrate as the **sixfold** khatam. Here the
  contact angle is forced rather than chosen: the ray from the far end of
  a unit edge meets this edge's midline at depth ½·tan(90°−δ), and the
  next midpoint's ray closes onto exactly that depth (√3/4) only at
  **δ = 60°**, where each chord spans two midpoint steps — at δ = 30° the
  chords span four steps and retrace the same lines, at δ = 90° the rays
  hit the substrate vertices. So the walls are the {6/2} hexagram in each
  hexagon plus the three midsegments in each triangle, and the partition
  is exactly a **half-scale 3.6.3.6**: 12 faces per `⟨(2,0),(1,√3)⟩` cell
  (4 hexagram-core hexagons 3√3/8, 6 hexagram tips √3/16, 2 midsegment
  triangles √3/16; Σ = 2√3 = |det| exactly over Q(√3)), every vertex on
  the (p+q√3)/4 grid. The un-interlaced partition verifies exactly **p6m**
  (positive rot60 at two inequivalent centres, rot120, rot180 and five
  mirrors pass; rot90, rot60 on the threefold centre, an off-site rot180
  and the mirror y = √3/4 all fail).
- **Done (M56):** the trio is complete — `pic-3-12-12`
  (`pic_3_12_12.rs`, same `hankin_ext` feature) lays the engine on the
  truncated-hexagonal 3.12.12 substrate as the **twelvefold** khatam
  (khatam-e shamsa). δ is forced again, and here it is forced by a single
  clean fact: the substrate midpoints of a regular n-gon lie on its
  incircle 360°/n apart, and a ray leaving a midpoint at angle δ to the
  edge leaves it at angle δ to the *tangent*, so by the tangent-chord
  theorem it meets the incircle again after a central angle of exactly 2δ
  — it lands head-on at another midpoint iff **δ = 180°·k/n**. The
  dodecagon admits {15°, 30°, 45°, 60°, 75°, 90°}, the triangle only 60°,
  so δ = 60° is the *unique* contact angle closing both polygons (δ = 90°,
  the other dodecagon candidate, sends the triangle rays along their
  normals through the substrate vertices). The walls are the 12 chords
  `M_i → M_{i+4}` per dodecagon — the {12/4} star of four overlaid
  equilateral triangles — plus the three midsegments per triangle: 18
  walls per cell, face-to-face with 0 T-junctions. A cell of the substrate
  lattice `⟨(3/2+√3, 1+√3/2), (0, 2+√3)⟩` holds **45 faces in six orbits**
  (rosette-core dodecagon 3/2+3√3/4, 6 vertex pentagons 3/8+3√3/16, 12
  band quads 1/8+√3/12, 2 medial triangles √3/16, 12 inner quads
  1/16+√3/48, 12 star notches √3/48; Σ = 6 + 7√3/2 = |det| exactly over
  Q(√3)), every vertex a (p+q√3)/24 surd and every corner 30°–150°. The
  midsegments quarter each substrate triangle (medial quarter = a face,
  corner quarters = pentagon tips) and the chords cut each dodecagon into
  the core, twelve each of the three inner orbits and twelve
  half-pentagon corners — the classical twelvefold rosette. The
  un-interlaced partition verifies exactly **p6m** (positive rot60/rot120/
  rot180 at the dodecagon centre, rot120 at a substrate triangle centre,
  rot180 at a dodecagon–dodecagon midpoint and the mirrors y = 0, x = 0,
  30° through O; rot30, rot90, rot60 on the threefold centre, rot180 on the
  dodecagon–triangle midpoint and on a substrate vertex, and the mirrors
  y = √3/2, x = 1 all fail). Note the substrate has **no squares** —
  3.12.12 is dodecagons and triangles only — so unlike M53 there is no
  square-fill family here; the vertex pentagons and medial triangles carry
  that role.
- **Done (M57):** the general **layer mode** — `tess-core`
  `strapwork.rs` (no feature gate: rendering/generator infrastructure),
  `walls(tiles: &[Tile], delta_deg: f64) -> Vec<Wall>`, a pure f64
  function of (tiles, δ) for an *arbitrary* host tiling: at each edge
  midpoint two rays enter each adjacent tile at contact angle δ
  symmetric about the inward normal, the adjacent tile fires the
  point-reflected pair so walls are continuous across edges, straight
  through at M; inside a tile a ray runs to its boundary exit and is
  split at every crossing with the tile's other rays (crossings
  subdivide rather than terminate — first-crossing termination would
  drop the rosette-core edges the registered partitions contain, and
  the PIC modules are ground truth), collinear strand ends and
  near-parallel rays guarded, output endpoint-deduped on a 1e-6 grid
  and sorted, deterministic. The **three tiling-mode cross-checks**
  tie it to the proven tables: registry substrate → engine walls ≡
  registered internal edges, zero missing / zero extra, worst endpoint
  2e-15 (bar 1e-9), equal length sums, window-invariant —
  4.8.8@45° ↔ `pic-4-8-8` (640 segments), 3.6.3.6@60° ↔
  `pic-3-6-3-6` (668), 3.12.12@60° ↔ `pic-3-12-12` (1030).
  **Rendering:** SVG only — `export --strapwork <δ>` strokes the wall
  network in the border color (width 3× the tile border, ∝ view,
  round caps); flag absent = byte-identical export. No new registry
  tiling (built-in count stays 53). **Remaining follow-up:** the
  TUI/GUI live overlay (wall stroke in `App`-level rendering).

- **Motif:** the generative alternative to measured panels (M35):
  Hankin's polygons-in-contact method — given a substrate tiling and a
  contact angle δ, emit the Islamic star-pattern strapwork (Kaplan,
  "Islamic star patterns from polygons in contact", 2005; Bonner's
  acute/median/obtuse families; the Tiling Atlas's system organization).
  Runs on substrates the registry already has: 4.8.8 (fourfold A →
  khatam), 4.6.12/3.12.12 (12-point Seljuk rosettes), 3.6.3.6
  (6-point), girih tiles (fivefold).
- **Design question (the doc's job):** rendering layer (strapwork as a
  decoration over tiles, like the Ammann-bar idea) vs. first-class
  tilings (the strapwork region partition *is* a tiling, M35-style).
  Both are defensible; the layer version is far more general (one
  engine × every substrate × 3 openings = dozens of patterns).
- **Verify:** for layer mode — strapwork lines cross edges at
  midpoints by construction; for tiling mode — the M35 partition
  battery generalized.

### 15. Random/statistical tilings (blocked on a determinism policy)

- **Candidates:** Truchet tile mazes (quarter-circle tiles on a square
  grid), random rhombille heights (lozenge dimer model), random
  square–triangle tilings, domino tilings of the Aztec diamond
  (height function colorable). All are classic, all need one decision
  first: a **seeded determinism policy** — propose `ParamSpec::Int`
  seed parameter, generator = pure function of (seed, view), i.e.
  stateless per parameter set, which keeps `check_deterministic`
  meaningful; document that different seeds give different (equally
  legal) tilings, like the hat's view-anchored reseeding but explicit.
  Truchet is the natural first (S effort once the policy exists).

### 16. Hyperbolic tilings {p,q} (Poincaré disk)

- **Motif:** the {p,q} regular tilings of H² (7 of the 17 wallpaper
  constructions have hyperbolic analogues; infinitely many {p,q} with
  (p−2)(q−2) > 4). A separate geometry core arc: Poincaré-disk
  renderer (the TUI rasterizer draws Euclidean lines; hyperbolic
  lines are arcs), {p,q} reflection-tree generation, and a new
  `TilingCategory` variant (one-variant + `ALL` slot change, UIs
  unaffected — the category-grouping design already future-proofs
  this). Start with {4,5} or {5,4}; view = disk region instead of
  BBox. Large; only after P0/P1 drains.

### 17. Socolar–Taylor monotile (and the half-hex hierarchy)

- **Motif:** the Socolar–Taylor single aperiodic hexagonal tile with
  printed matching rules (arXiv:1003.4279; Socolar & Taylor, JCTA
  118 (2011)) — forces a limit-periodic hierarchy closely related to
  the (1+ε+ε²)-tilings and **half-hex tilings**; the Lee–Moody
  analysis (Acta Cryst. A 73 (2017)) ties it to Penrose-hexagon
  double-hex tilings. Related modern option: the Walton–Whittaker
  hexagonal monotile with purely edge-to-edge orientational rules
  (arXiv:1907.10139, J. Inst. Math. Jussieu 2023).
- **Construction path:** generate via the underlying substitution —
  the arrowed half-hex substitution / pseudo-inflations (per the
  Walton–Whittaker paper's own remark) — not by constraint search.
  The tile markings (black stripes + colored segments) are a
  stripe-decoration layer over hexagons: same plumbing as #11/#14.
- **Flags:** matching rules are next-nearest-neighbor (ST) — emitted
  patches are substitution-legal, which is the honest scope; say so
  in the module docs (repo precedent: substitution + seed aperiodicity,
  matching rules not enforced). Park behind the stripe layer.

### 18. n-fold rhomb substitution family (7-fold; CAST framework)

- **Motif:** Goodman-Strauss's 7-fold rhomb tiling and Harriss's
  n-fold rhomb family (any n; Harriss 2004), unified by Pautze's
  Cyclotomic Aperiodic Substitution Tilings (arXiv:1606.06858), which
  gives construction recipes across n. 7-fold is the smallest n the
  registry lacks (5 = penrose, 8 = AB, 10 = girih-qc, 12 = #6/#8).
- **Cost flag:** 7-fold arithmetic lives in Q(2cos(π/7)) — **degree 3**,
  the first cubic field in the repo (everything so far is degree ≤ 4
  quadratic/SQRT-of-quadratic). Integer 3-tuple exact arithmetic is
  straightforward but new; do after #6/#8 establish the 12-fold
  machinery.

### 19. Second measured ornamental panel ✅ (M52)

- **Sprint allocation (2026-11-13):** first deliverable of the girih
  extension arc (M52, delegated to the strong track, feature
  `girih_ext`) — prefer the p6m 12-point panel; the Hankin PIC engine
  (#14) is the follow-up arc after one concrete pattern lands.
- **Done (M52):** `girih-w85` (`girih_w85.rs`) ships the Wichmann
  data18/W85 panel (Alhambra, Mexuar patio) — not measured pixel-wise
  but cracked to an **exact line-grid construction over Q(√3)** (lattice
  L = 283.92, strap 9, all vertices on the (p + q√3)/50 grid), 42 tiles
  per hexagonal cell. Wichmann's *632 is the un-interlaced motif; the
  drawn over/under interlace is chiral, so the verified group is **p6**
  (exact Q(√3) cell-set probe: rot60 at the stars, rot180 at the three
  lattice-edge midpoints, rot120 at (a1+a2)/3, no mirrors, no rot30).

- **Motif:** reuse the M35 measurement pipeline on a second classical
  panel. Candidates from Wichmann's tiling search database
  (tilingsearch.mit.edu): the widespread 12-pointed p6m star pattern
  (e.g. Alhambra Mexuar patio, data18/W85 — 11th-c. origin, Morocco to
  India), a Seljuk 12/10/9-point combination (Karatay-Han), or a
  Baghdad brick panel (T02474, 15° angles). Prefer a p6m 12-point
  panel: it extends the ornamental family toward the 12-fold arc and
  pairs with #14's generative engine as its validation case.

### 20. Fractalizing k-uniform tilings (2+√3 dodecagon dissection)

- **Motif:** the "fractalizing" construction (List of k-uniform
  tilings, Wikipedia): dilate a uniform tiling by 2+√3 and fill the
  dissected dodecagons — e.g. the 3.12.12 basis fractalized yields a
  quasiperiodic-looking substitution tiling on the square lattice;
  4.6.12 basis with 3+√3. A cute bridge entry: starts from a table we
  already own, needs one dodecagon dissection rule.
- **verify:** the exact dissection rule and the tiling's classification
  (quasiperiodic vs limit-periodic) during the spike; category
  declared then (likely Quasiperiodic).

---

## Parked / rejected (with reasons)

- **Quaquaversal tiling** — 3-space tiling; no 2D analogue (removed
  from the queue 2026-11-09, PATTERNS.MD #7). 3D generally out of
  scope (incl. Culik–Kari's 21 Wang cubes).
- **X-pentomino as a rep-tile entry** — not a rep-tile: a polyomino
  rep-tile must cover a corner of its rectangular hull (necessary
  condition, Golomb); the X-pentomino fails (MathOverflow 427281).
  The known 3-4-5 scale dissection is an irrep curiosity, not a
  plane-substitution source.
- **Danzer ABC tiling** — substitution is classical, but the original
  aperiodicity rules needed repairs and the literature is messy;
  #8/#9/#17 are cleaner representatives of their structure classes.
- **Gummelt decagon** — a *covering* (overlapping cluster), not a
  tiling; same rationale as quaquaversal.
- **Ammann A1 (6-piece binary-tree set)** — aperiodic by matching
  rules with uncountably many tilings and **no substitution
  structure** (indentation/protrusion jigsaw forcing a binary-tree
  hierarchy); our generation machinery is substitution-first. Park
  until a matching-rule-enforcement design exists (would share #17's
  plumbing).
- **Squiral (spiral square) tiling** — promising nonperiodic
  substitution (Bielefeld "Squiral"); park until its rule table is
  verified from the encyclopedia (not done in this pass).
- **Robinson's 6-tile set as a standalone entry** — the generalized
  Robinson-triangle machinery is already inside `penrose.rs`; a
  separate entry would duplicate it. (Revisit only as a documentary
  "matching-rule marked tiles" layer.)
- **Kari–Culik 13 Wang tiles** — superseded by #11 (JR11 is smaller
  *and* minimal); keep as the historical note inside #11.

## Adoption checklist (per AGENTS.md, per candidate)

1. Spike in Python (stdlib only, /tmp/opencode): exact geometry +
   verification battery BEFORE any Rust (the workflow that has caught
   real bugs every time).
2. Name + vertex figure verified against Wikipedia/literature (step 4).
3. Feature flag, module, registry entry, **declared category** (new
   census counts! extend `registry_groups_are_complete_and_canonically_ordered`
   in tess-tilings/src/lib.rs and the README count).
4. If `LatticeTiling`: extend `lattice_culling_matches_brute_force`.
5. Standard trio + family-specific battery; `proto`/`seed` docs.
6. One commit `M<n>: <name> tiling (<vertex figure>)` with lattice,
   cell contents, verification; flip PATTERNS.MD status; annotate the
   entry here with the M number.

## Suggested order (next milestones, M39+)

M39–41: paver trio + hexagram (#1, #2 — three commits, quick wins) ·
M42: first 2-uniform batch, p6m entries (#4) · M43–44: pentagon types
13 and 14 (#5 showstoppers) · M45: Stampfli 12-fold (#6) — then
re-evaluate against golden bee (#7) vs Wang JR11 (#11) vs the IH
design doc (#13). Rationale: cheap wins keep momentum; the pentagon
census has a hard completeness ceiling (15/15) in the repo's own
style; Stampfli completes the 5/8/12 rotation series with machinery
that already exists twice.

## Sources (consulted 2026-11-12)

- Wikipedia: Pentagonal tiling; List of k-uniform tilings;
  k-uniform tiling; Demiregular tiling; Socolar tiling;
  Socolar–Taylor tile; Ammann A1 tilings; Ammann–Beenker tiling;
  Rep-tile; Anisohedral tiling; 3-4-6-12 tiling.
- MathWorld: Golden Bee; Irreptile; Rep-Tile; Wang's Conjecture;
  Heesch Tiling Notation.
- Tilings Encyclopedia (Bielefeld): Ammann (person), Ammann-Beenker,
  Ammann Chair, Socolar, Shield, MLD Class Shield and Socolar,
  Wheel Tiling, Rorschach.
- Papers: Stampfli & Schaad, Bridges 2021 (12-fold, 2+√3); Schaad &
  Stampfli, arXiv:2102.06046 (1+√3); Socolar, Phys. Rev. B 39 (1989);
  Socolar & Taylor, arXiv:1003.4279 / JCTA 118 (2011); Lee & Moody,
  Acta Cryst. A 73 (2017); Walton & Whittaker, arXiv:1907.10139;
  Goodman-Strauss, Annals 147 (1998) and trilobite-and-crab paper;
  Vince, Amer. Math. Monthly 121 (2014) / arXiv:1611.02218;
  Durand, Shen & Vereshchagin, DCG 63 (2020); Akiyama, DCG 48 (2012);
  Jeandel & Rao, Adv. Combin. (2021); Culik, Disc. Math. 160 (1996);
  Siefken, arXiv:1410.1572; Eigen–Navarro–Prasad (2007);
  Kari–Labbé–Mostert (2025); Pautze, arXiv:1606.06858 (CAST);
  Kaplan, polygons-in-contact (2005) and isohedral.ca IH tables.
- Databases: tilingsearch.mit.edu (Wichmann); tiling-atlas.vercel.app
  (Bonner systems); jaapsch.net/tilings (McLean IH templates mirror).

## Changelog of this file

- 2026-11-12 — created from PATTERNS.MD's proposed/research sections +
  a literature sweep (20 candidates: 4 P0 quick wins, 8 P1 sprints,
  8 P2 arcs; 8 parked/rejected with reasons).