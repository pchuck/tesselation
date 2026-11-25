# PROPOSED.MD

Researched candidate backlog: additional tiling patterns worth adding to
tessellix, each with a verified literature basis, a construction path in
this codebase, an exact-arithmetic plan, and an effort estimate. This is
the research dossier behind the 🔭/❓ markers in docs/ROADMAP.md —
that file remains the status tracker (✅ implemented / 🚧 in progress / 🔭 proposed /
❓ research); when an entry here is adopted it gets an `M<n>` commit per
AGENTS.md, its status flips in docs/ROADMAP.md, and this entry is annotated.

Research method (2026-11-12): every candidate below was checked against
Wikipedia / primary papers / the Bielefeld Tilings Encyclopedia for
canonical status and a known construction path before inclusion. Claims
that still need a design-time check are marked **verify**. Nothing here
is "would be nice" without at least a known construction path.

Categories are the declared `TilingCategory` values (tess-core
`category.rs`). Uniform (11) and Laves (11: 8 primary + 3 self-dual secondaries) are **closed** — everything
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
| 8 | shield tiling ✅ DONE (M51); Socolar-12 blocked-on-source, wheel tiling = MLD-dual follow-up | Quasiperiodic | substitution | L | P1 |
| 9 | Lançon–Billard binary tiling | Quasiperiodic | substitution | M | P1 |
| 10 | Rep-tile siblings: T-tetromino, P-pentomino | Aperiodic | integer substitution | S–M each | P1 |
| 11 | Jeandel–Rao 11 Wang tiles ✅ DONE (M93, canonical 100x100 patch as a data table with geometric edge bands; aperiodicity cited, not re-proven; no generator) | Aperiodic | edge-colored Wang | L | P1 |
| 12 | Trilobite and crab (2-tile aperiodic set) | Aperiodic | L-substitution | L | P1 |
| 13 | Isohedral (IH) parameterized generator | Wallpaper | new generator class | XL | P2 |
| 14 | Hankin polygons-in-contact strapwork engine | Wallpaper | rendering/generator | XL | P2 |
| 15 | Random/statistical tilings (Truchet et al.) | — | needs determinism policy | M each | P2 |
| 16 | Hyperbolic {p,q} tilings | new category | geometry-core arc | XL | P2 |
| 17 | Socolar–Taylor monotile ✅ DONE (M86, decorated-hexagon port of the forced 529-site patch; the half-hex substitution hierarchy itself landed separately as M135) | Aperiodic | substitution + stripes | L–XL | P2 |
| 18 | n-fold rhomb family (7-fold GS/Harriss, CAST) | Quasiperiodic | degree-3 arithmetic | L+ | P2 |
| 19 | Second measured ornamental panel | Wallpaper | M35 pipeline | M–L | P2 |
| 20 | Fractalizing k-uniform (2+√3 dodecagon) ✅ DONE (M110, same-plane limit-periodic family over 3.12.12 — not a substitution) | Quasiperiodic | substitution on uniform basis | M | P2 |
| 22 | Kaplan Ch. 3 Islamic star-pattern extensions (22a design elements ✅(M163); 22b Najm template-tiling generator ✅(M99); 22c rosette transform ✅(M99); 22d parquet deformations ✅(M165); 22e quasiperiodic substrates + (n,c)-monsters + Kepler Aa; 22f absolute-geometry patterns; 22g decoration suite ✅(M164); 22h Moroccan/optimization/strange stars) | Islamic | motif/tiling generators + rendering | S–XL | P1/P2 |
| 23 | Bielefeld Tilings Encyclopedia survey — candidate clusters (golden-family Q(√φ), 8-fold Q(√2), 12-fold Q(√3), rep-tile/limit-periodic, generalized GLB binary §9-rescope, girih-family, pinwheel family, cubic/quartic field tiers) | mixed | see §23 | S–XL | P1/P2 |
| 24 | Medallion composition program — parameterized medallion interface + hole-bearing substrates + boundary-matched inference; two-level star-and-field compositions on periodic nets (see §24) | Islamic/engine | composition generator | XL (staged) | P1 |

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
  post-category-grouping workflow; fills the Wallpaper tab's group-realizer slots (see §21).
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
  changelog entry in docs/history/CHANGELOG.md.
- ✅ **M40 delivered (2026-11-12): herringbone** — `herringbone.rs`
  (feature `pavers`), chiral single-direction L-pairs
  `H(a,b) + V(a+2,b)` on the oblique lattice ⟨(1,3),(2,2)⟩ (area 4),
  2 split dominoes per cell. Verified group is **pgg** as expected
  for the generic 2:1 case: no mirrors and no 4-folds (chiral), 16
  2-fold centers per 4×4 period, two perpendicular glide families —
  diagonal axes `y = x + c` and anti-diagonal axes `y = −x + c` — with
  the off-family glides `(1−y,1−x)`, `(−x,y+1)`, `(x+1,−y)` absent.
  Exact coset symmetry check, forbidden list included; see the M40
  changelog entry in docs/history/CHANGELOG.md.
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
  forbidden list included; see the M41 changelog entry in docs/history/CHANGELOG.md.

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
  | p6m | 3⁶ ; 3².4.3.4 ✅ `kuniform-n18` (M87) | 3,3 |
  | p6m | 3.4.6.4 ; 3².4.3.4 ✅ `kuniform-n9` (M88) | 4,4 |
  | p6m | 3.4.6.4 ; 3³.4² ✅ `kuniform-n8` (M89) | 4,4 |
  | p6m | 3.4.6.4 ; 3.4².6 ✅ `kuniform-n5` (M90) | 5,5 |
  | p6m | 4.6.12 ; 3.4.6.4 (the 3-4-6-12 tiling) ✅ `kuniform-n1` (M91) | 4,4 |
  | p6m | 3⁶ ; 3².4.12 ✅ `kuniform-n13` (M92) | 4,4 |
  | p4m | 3.12.12 ; 3.4.3.12 | 3,3 |
  | p6m | 3⁶ ; 3².6² ✅ `kuniform-p6m` (M44) | 2,3 |
  | p6  | 3⁶ ; 3.4.6 (variant 1) ✅ `kuniform-n19` (M95) | 3,3 (group p6m exact: both mirror classes verified — the wiki's p6 label under-states it, see docs/families/kuniform.md §50) |
  | p6  | 3⁶ ; 3.4.6 (variant 2) ✅ `kuniform-n20` (M96) | 5,7 → e = 6 exact (M96 torus enumeration, see docs/families/kuniform.md §46) |
  | cmm | 3².6² ; 3.4.6 ✅ `kuniform-n12` (M97) | 2,4 (both confirmed exact, M97 orbit enumeration — t = 2, e = 4) |
  | pmm | 3.6.3.6 ; 3².6² ✅ `kuniform-pmm` (M48) | 2,3 |
  | cmm | 3.4².6 ; 3.6.3.6 (variant 2) | 3,4 |
  | pmm | 3.4².6 ; 3.6.3.6 (variant 1) | 4,4 |
  | p4g | 3³.4² ; 3².4.3.4 (variant 1) | 4,5 |
  | pgg | 3³.4² ; 3².4.3.4 (variant 2) | 3,6 |
  | cmm | 4⁴ ; 3³.4² (variant 1) ✅ `kuniform-n4` (M103) | 2,4 (confirmed exact, M103 torus enumeration; docs/families/kuniform.md §60) |
  | cmm | 4⁴ ; 3³.4² (variant 2) ✅ `kuniform-n3` (M108) | 3,5 (confirmed exact, M108 exact-class enumeration; docs/families/kuniform.md §62) |
  | pmm | 3⁶ ; 3³.4² (variant 1) ✅ `kuniform-n14` (M103) | 3,4 (confirmed exact, M103 torus enumeration; docs/families/kuniform.md §60) |
  | cmm | 3⁶ ; 3³.4² (variant 2) ✅ `kuniform-n15` (M108) | 4,5 (confirmed exact, M108 exact-class enumeration; docs/families/kuniform.md §62) |

- **Construction:** `LatticeTiling` tables like the Archimedean ones —
  each is a supercell of an existing uniform lattice (supercell areas
  2–12 primitives). Start with the six p6m entries (hexagonal lattice
  machinery already exact), then p4m/p4g/pgg. Pattern seeds by
  polygon type first (repo convention), vertex-orbit coloring as the
  alternative seed where it differs.
- **Scope note:** all 20 is the arc goal (like Laves 8/8); the first
  batch should be 3–5 tables. Duals of 2-uniform tilings are a cheap
  bonus family later (mixed floret/cairo-like duals) via the
  dual-construction helper (docs/ROADMAP.md research area 2).
  **First dual batch landed (M98):** the helper now exists —
  `dual()` in `tess-tilings/src/dual.rs`, oracle-validated exactly
  against all 7 hand-built Laves tables — and the four p6m duals
  `kuniform-dual-p6m/n18/n9/n8` are registered (Laves primaries).
  **Second dual batch landed (M102):** `kuniform-dual-n5/n1/n13/n20`
  registered — n20 chiral p6, the rest p6m (n5/n1/n13 with 6-fold
  centers off the origin). The 9 remaining duals of this list still
  queue behind the same helper; the square-family rows stay blocked on
  their unimplemented primals.
- **Verify:** the standard ±3×±3 battery per table (the vertex-figure
  check must verify *both* orbit figures, and the two-orbit structure
  itself — assert exactly 2 vertex classes over a large patch), plus
  the culling list.
- **Status:** implemented (M44 + M48 + M87 + M88 + M89 + M90 + M91
+ M92 + M96 + M97 + M95 + M103 + M108) — 15 of 20 entries landed, 5 of 20
  remain. The p6m entry 3⁶ ; 3².6² as
  `kuniform-p6m` (M44; index-3 honeycomb dissection, exact Q(√3)
  table, both vertex orbits + p6m generators verified by an
  exact-surd Python spike and Rust cell-set probes), the pmm entry
  3.6.3.6 ; 3².6² as `kuniform-pmm` (M48; hexagons on the rectangular
  lattice span{(2,0),(0,√3)} with up- and down-pointing triangle rows
  in the gaps, exact Q(√3) table, both vertex orbits + pmm generators
  verified the same way), and the three-entry p6m batch (M87
  `kuniform-n18` 3⁶ ; 3².4.3.4, M88 `kuniform-n9` 3.4.6.4 ;
  3².4.3.4, M89 `kuniform-n8` 3.4.6.4 ; 3³.4² — exact Q(√3) tables
  on 60° hexagonal bases with |b1| = |b2| (1+√3 for n18, 2+√3 for
  n9/n8), all three p6m *632 by numeric group determination about the
  hexagon center / 3⁶ vertex origin, both vertex orbits +
  edge-to-edge structure verified by the ±3×±3 Python battery before
  the Rust port, 4 tess-tilings tests each). The second p6m batch
  (M90 `kuniform-n5` 3.4.6.4 ; 3.4².6, M91 `kuniform-n1` 4.6.12 ;
  3.4.6.4 — the 3-4-6-12 tiling, the module's first 12-gon, M92
  `kuniform-n13` 3⁶ ; 3².4.12) lands exact Q(√3) tables on
  RIGHT-handed 60° hexagonal bases with |b1| = |b2| = 3+√3 (n5/n13,
  |det| = 9+6√3) and 2+2√3 (n1, |det| = 12+8√3); each has three
  figure strings over two vertex orbits — a chiral orbit pair
  (100% bidirectional mirror pairing) plus one achiral orbit — and
  the pinned canonical figure keys are the lex-min integer rotations
  (the battery's original check-3 canonicalized by string-rotation,
  which differs for 12-gon figures: ring labels 1246/1264 and
  12334/12433 map to the pinned 4612/4126 and 33412/33124); the
  battery's area-FD table needed the unit-dodecagon 2·area entry
  12+6√3 = (48,24) in radical form; check 4 (coverage) runs over
  the maximal fully-covered rectangle containing the block centre
  (the jagged 49-cell union bites past the 1-unit inset by up to
  0.675 units, e.g. at n13), 4 tess-tilings tests each, completing
  the six-entry p6m block of the closed list). The first p6 entry
  (M96 `kuniform-n20` 3⁶ ; 3⁴.6 — the flower construction: hexagon
  flowers on the index-13 sublattice ⟨(1,3),(−3,4)⟩ of the unit
  triangular lattice, 21 tiles/cell, 60° hexagonal basis
  |b1| = |b2| = √13, |det| = 13√3/2 RIGHT-handed; 2-uniformity and
  A:B = 1:1 pinned exactly by the class map k = 4X+3Y mod 13 (kernel
  = L13, R60 = ×4 with the corner and non-corner class sets each one
  orbit); p6 pinned by R60..R300 PASS + six negative mirror probes +
  a full translation scan finding exactly L13; exact torus orbit
  count t = 5 with triangle orbits {6,6,6,2}, edge orbits
  {6,6,6,6,6,3} → e = 6, correcting the 5,7 parenthetical in the
table above). The first cmm entry (M97 `kuniform-n12` 3².6² ;
  3⁴.6 — rows of edge-sharing hexagons with unit-triangle bands
  between the rows; 5 tiles/cell on the PRIMITIVE cell of the
  centered rectangular lattice ⟨(√3,0), (√3/2,5/2)⟩ (the centering
  vector as basis, |det| = 5√3/2; the M97 spike's 2-hex + 8-tri
  centered cell recovered tile-by-tile as S ⊎ (S + c) mod ℤb1 + ℤb2);
  cmm pinned on the emitted table — 8 translation probes + six
  2-fold probes (two translation classes of centers) + both mirror
  families PASS, valley 2-folds FAIL, R60/R90/R120 FAIL at every
  mirror intersection; all triangles one orbit (the origin 2-fold
  swaps the visual A/B pairs mod Λ) → t = 2, and edge orbits by
  adjacency + corner classes → e = 4, matching the 2,4 parenthetical
  in the table above exactly). The t = 3 sibling (M95 `kuniform-n19`
  3⁶ ; 3⁴.6 variant 1 — the flower construction on the index-9
  sublattice ⟨(3,0),(0,3)⟩ of the unit triangular lattice,
  13 tiles/cell, 60° hexagonal basis |b1| = |b2| = 3, |det| = 9√3/2
  RIGHT-handed; 2-uniformity and A:B = 1:3 pinned exactly by v mod 3
  (kernel = L9, R60 acting as a 2-cycle on the petal-tip classes and
  a 6-cycle on the corner classes); exact torus orbit count t = 3
  with triangle orbits {6,6}, e = 3 under the full D6 point group
  with edge orbits {6,12,3} — matching the table's 3,3 parenthetical,
  though the wiki's p6 group label under-states the symmetry: both
  mirror classes pass exactly, so the entry registers as p6m; see
  docs/families/kuniform.md §50). The two-entry cmm/pmm batch (M103
  `kuniform-n4` 4⁴ ; 3³.4² variant 1 — staggered two-square-tall bands
  with alternating-triangle one-row bands, basis ⟨(1,0), (1/2,
  2+√3/2)⟩, |det| = 2+√3/2 RIGHT-handed, centered rectangular lattice
  kind, cmm pinned by full scan — and `kuniform-n14` 3⁶ ; 3³.4²
  variant 1 — square rows with double triangle strips on the PRIMITIVE
  lattice ⟨(1,0), (0,1+√3)⟩, |det| = 1+√3, pmm pinned by full scan;
  both exact torus enumerations reproduce the wiki parentheticals
  (2,4) and (3,4); the sprint brief's stale row labels were corrected
  against the current wiki table first — see docs/families/kuniform.md §60). The
  sibling batch (M108 `kuniform-n3` 4⁴ ; 3³.4² variant 2 — staggered
  three-square-tall bands with alternating-triangle one-row bands,
  basis ⟨(1,0), (1/2, 3+√3/2)⟩, |det| = 3+√3/2, centered rectangular,
  cmm — and `kuniform-n15` 3⁶ ; 3³.4² variant 2 — square rows with
  triple triangle strips (every horizontal edge shared back-to-back),
  basis ⟨(1,0), (1/2, 1+3√3/2)⟩, |det| = 1+3√3/2, centered
  rectangular, cmm; both exact orbit-class enumerations reproduce the
  wiki parentheticals (3,5) and (4,5); stale brief row indices
  corrected against the saved wiki page first — see docs/families/kuniform.md §62).

---

## P1 — sprints

### 5. Convex pentagon census — the complete 15 types

- **Motif:** the 15 types of convex pentagonal tilings (Reinhardt 1918
  → Mann–McLoud–Von Derau 2015; completeness proved by Rao 2017). The
  repo already owns several members unknowingly; this arc makes the
  census explicit and complete, mirroring the Laves/Archimedean arcs.
- **M101 audit — the three pending existing-tiling assignments are
  verified and pinned by condition tests** (lettering convention:
  repo letters shift the Wikipedia letters by one, W-angle X =
  our-(X + 1 mod 5), side a joins A–B; 4 new tests, 0 new tilings):
  `cairo-pentagonal` = type 4 family (M76: `cairo-collinear` was
  the same V3.3.4.3.4 pattern with a different pentagon and was
  merged into it as a documented collinear-vertex variant — one
  card, 61 tilings; shift k=4: C = E = 90°, b = c, d = e);
  `prismatic-pentagonal` = type 1 (cmm; shifts k=1/k=3:
  C + D = 180°, A + B + E = 360°); `floret-pentagonal` = type 5
  (p6 six-tile rosette; shift k=3: B = 60°, E = 120°, a = b,
  d = e). Hirschhorn falsification: the "type 8" attribution holds
  for 0/10 letterings (5 cyclic shifts × 2 chiralities — despite
  b = c = d = e, the (2C + D, E + 2A) angle-sum pairs come out
  260/260 and 280/280, never 360/360; types 7 and 9 also fail) —
  the shape is actually type 1 (shift k=1: C + D = 180°,
  A + B + E = 360°; the identity lettering satisfies the C11-T1A
  subfamily A + B + C = 360°, C = 2D = 160°), and an independent
  disqualifier: type-8 tilings are periodic pgg, while hirschhorn
  is nonperiodic. Type-8 census coverage is unaffected: the
  dedicated `pentagon-8` module (M77) is registered
  unconditionally in `lib.rs` (the `pentagon-8` feature does not
  exist, so any cfg on it is inert).
- **Headline: the 15-type census is now COMPLETE in coverage** —
  every type 1–15 has a registered realizer: types 1, 4, 5 on the
  existing laves/cairo registry entries above; types 2, 3, 7, 9,
  10–15 on feature-gated dedicated `pentagon-N` modules; types 6,
  8 on the unconditional `pentagon6`/`pentagon8` modules (M55–M77
  arc).
- **The type conditions (Wikipedia, sides a..e vs angles A..E):**

  | type | conditions | group | unit | isohedral | e2e |
  |---|---|---|---|---|---|
  | 1 | B+C=180°, A+D+E=360° | p2/cmm/cm/pmg/pgg/p1 topologies | 2–4 | iso–3-iso | yes |
  | 2 | c=e, B+D=180° | pgg | 4 | iso | yes |
  | 3 | a=b, d=c+e, A=C=D=120° | p3 / p31m | 3 | iso | **no** |
  | 4 | b=c, d=e, B=D=90° | p4 / p4g | 4 | iso | yes |
  | 5 | a=b, d=e, A=60°, D=120° | p6 | 6 | iso | yes |
  | 6 | a=d=e, b=c, A=2C, C+E=180° (repo letters; in the table's Wikipedia letters the same family reads B+D=180°, 2B=E — W-A=our-B, W-B=our-C, W-C=our-D, W-D=our-E, W-E=our-A, side letters aligned) | p2 (authored, M74) | 4 | 2-iso | yes |
  | 7 | b=c=d=e, B+2E=2C+D=360° | pgg (verified, M72) | 8 | 2-iso | yes |
  | 8 | b=c=d=e, 2B+C=D+2E=360° | pgg (verified, M77) | 8 | 2-iso | yes |
  | 9 | b=c=d=e, 2A+C=D+2E=360° | pgg (verified, M59) | 8 | 2-iso | yes |
  | 10 | a=b=c+e, A=90°, B+E=180°, B+2C=360° | cmm (verified, M63) | 6 | 3-iso | **no** |
  | 11 | 2a+c=d=e, A=90°, 2B+C=360°, C+E=180° | pgg (verified, M60) | 8 | 2-iso | no |
  | 12 | 2a=d=c+e, A=90°, 2B+C=360°, C+E=180° | pgg/p2 | 8 | 2-iso | no |
  | 13 | d=2a=2e, B=E=90°, 2A+D=360° | pgg/p2 | 8 | 2-iso | no |
  | 14 | 2a=2c=d=e, A=90°, B≈145.34°, C≈69.32°, D≈124.66°, E≈110.68° | p2 | 6 | 3-iso | **no** |
  | 15 | a=c=e, b=2a, d=a√2/(√3−1)=a√(2+√3), A=150°, B=60°, C=135°, D=105°, E=90° | pgg (verified, M55) | 12 | 3-iso | **no** |

- **Priorities within the arc:** the fixed-shape showstoppers first —
  **type 14** (Stein 1985; exact proportions b/a =
  √((11√57−25)/8), sin B = (√57−3)/8 — a Q(√57) tiling, spectacular),
  **type 15** (MMVD 2015; the last-discovered convex pentagon; field
  exactly Q(√3), d = a√2/(√3−1) = a√(2+√3) — landed, M55), and
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
- **Status:** type 15 (MMVD 2015) implemented (M55) — the other
  fixed-shape showstopper: group **pgg confirmed** by exhaustive
  exact probe (25 translations + 25 half-turns + 50 glides + 0
  mirrors in a 5×5 window), the expected Q(√2,√3) field collapses
  to **exact Q(√3)** (all vertices (p+q√3)/2), and the dossier row's
  d was a typo — the correct form is d = a√2/(√3−1) = a√(2+√3)
  (= a(√6+√2)/2), verified exactly and now corrected in the row
  above. 12-tile rectangular unit (four congruent 3-tile blocks),
  3-isohedral (full-group orbits are three fours), non-edge-to-edge
  (4 T-split edges per cell). Type 14 (Stein 1985) implemented
  (M50) — the completely determined shape (2a = 2c = d = e,
  b = βa = √((11√57−25)/8 · a²), A = 90°), exact Q(√57, β)
  6-tile oblique unit, 3-isohedral, group p2 (2222). Type 13 (Rice)
  implemented (M49) — canonical A = 120° instance, exact Q(√3)
  8-tile unit, group p2. Type 9 (Rice 1977) implemented (M59) — the
  first edge-to-edge Rice type of the arc, pinned to the exact-Q(√3)
  member A = C = 120°, cos B = 1/7, cos D = 23/98, cos E = −11/14,
  a = 6, b = c = d = e = 14 (area 141√3, vertices on the
  (Z+Z√3)/7 grid): 8-tile rectangular unit, 2-isohedral (seeds
  0/1), group **pgg confirmed** (gauge-invariant quotient-table
  automorphism audit plus exact generator verification — half-turn
  c0 = (13, 4√3), perpendicular glides g1² = w2 and g2² = −w1, no
  mirrors). M101 completed the coverage audit: types 1, 4 and 5
  are verified on the existing registry entries —
  prismatic-pentagonal (type 1, repo lettering shifts k=1/k=3:
  C + D = 180°, A + B + E = 360°), cairo-pentagonal (type 4,
  shift k=4: C = E = 90°, b = c, d = e), floret-pentagonal
  (type 5, shift k=3: B = 60°, E = 120°, a = b, d = e) — pinned
  by four condition tests (M101: docs + 4 tests, 0 new tilings,
  686 → 690). The "hirschhorn = type 8" attribution is
  falsified (0/10 letterings; the shape is type 1, shift k=1 —
  type-8 coverage stays with the dedicated `pentagon-8` module,
  M77). Type 6 implemented in M74, 7 in M72, 8 in M77; 2, 3, 9,
  10, 11, 12, 13, 14 and 15 implemented. The 15-type census is
  now complete in coverage — every type 1–15 has a registered
  realizer. One type per task, delegated; all `pentagon1..15`
  features pre-created (type 4 maps to cairo-pentagonal, type 8
  to the dedicated pentagon-8 module).
  **Type 7: DONE (M72)** — the M58 bounded negative is superseded:
  it was an artifact of grid-restricted pinning (the family's
  published members sit off the 15°/30° and 22.5° grids the bounded
  search was pinned to, so the search could never have found the
  instance) plus two search-phase accounting bugs (mirror-edge
  direction, glued-corner jitter). The applet-grounded published
  instance (Jaap Scherphuis Tiling Viewer, N5-07, E = 106°) closes
  the family: b = c = d = e = 1/2, a ≈ 1.0357942, B = 148°,
  C ≈ 132.8626° (the unique root in (120°, 180°) of
  F(t) = sin(E−2t) − sin(t+E) − sin(E−t) − sin(E)), D ≈ 94.2747°,
  A = C − B/2 ≈ 58.8626° (the M58 brief corrections stand: A = C −
  B/2 confirmed; the "B < D" clause is not a convexity condition).
  The construction is bit-exact against the applet cell (max |Δ| =
  0.0 over 40 vertices + lattice), the tiling is edge-to-edge
  (907-tile census 3770 full / 0 split), vertex stars 816 = 271 +
  266 + 279, torus 20 edge orbits + 12 vertex orbits, group **pgg**
  (g1² = w1, g2² = −w2, half-turns c0 = (1/4, 0) + Λ/2, 24 negative
  probes), 8-tile 2-isohedral rectangular cell (four direct + four
  mirror), 11 tess-tilings tests (62 tilings, 476 tests). See
  docs/families/pentagons.md §25.
  **Type 8: DONE (M77)** — Kershner 1968 family b = c = d = e,
  2B + C = D + 2E = 360°, pinned to the exact Q(√3) member
  B = C = 120°: tan A = 3√3 (A ≈ 79.11°), cos D = 1/7
  (D ≈ 81.79°), tan E = −√3/2 (E ≈ 139.11°), A = E − 60°,
  a = 6√7, b = c = d = e = 14, tile area 189√3; all cell
  vertices on the Z[√3] grid (shipped table = verified M73
  integer cell at scale 42 divided by 3). 8-tile rectangular
  unit, four direct + four mirror (T2, T3, T5, T8), 2-isohedral
  (seeds [0,1,1,0,0,1,1,0], orbits {0,3,4,7}/{1,2,5,6}, each
  orbit mixing both chiralities); w1 = (36, 24√3),
  w2 = (−36, 18√3), w1 ⊥ w2, det = 1512√3 = 8·area. Group
  **pgg** by exact audit: perpendicular glides g1² = w1,
  g2² = −w2 (axis shifts +|w1|/2 and −|w2|/2 exactly,
  perpendicular parts (−4, 2√3) and (39, 26√3) nonzero — no
  power of a glide is a reflection); g1 maps T1 exactly onto
  T5, g2 maps T1 exactly onto T8; half-turn centre
  c0 = (−1/2, 25√3/2) (lattice coords (7/24, 11/36)); 22
  negative mirror probes + 2 quarter-turn probes.
  **EDGE-TO-EDGE**: 79-tile census 395 full / 0 split edges,
  290 interior directed edges each paired with its reverse,
  148 vertices, no T-junctions; torus 20 edge orbits, side
  pairs a-a:4 b-c:4 b-d:4 c-c:2 d-e:4 e-e:2 (all
  reverse-glued), 12 vertex orbits (eight 3s, four 4s). Vertex
  stars 91 = 30 × {A,A,C,D} + 36 × {B,B,C} + 25 × {D,E,E},
  every star 360°. Window census 79/42/20/5 at four fixed
  windows. 11 tess-tilings tests (62 tilings, 485 tests). See
  docs/families/pentagons.md §26.
  **Type 11: DONE (M60)** — Rice 1977 family 2a+c=d=e, A=90°,
  2B+C=360°, C+E=180° (⇒ C=360−2B, E=2B−180, D=270−B; convex
  90°<B<180°); pinned the reference construction's B=156° member at
  c=1 (A=90°, B=156°, C=48°, D=114°, E=132°, a=r=0.0946641…,
  b=s=1.0568030…, d=e=1+2r=1.1893282…; runtime-computed prototile —
  the arc's first bounded-precision instance, closure identities
  pinned to ~1e-12); 8-tile 2-iso unit (seeds 0/1, orbits mixing
  chiralities), NON-edge-to-edge — every d-side carries one c-side +
  two a-sides; rectangular lattice det = 8·area; **pgg** by
  anchor-pair symmetry audit on the exact model (perpendicular glides
  g1²=w1, g2²=−w2, half-turns at c0=(−0.3716…, 0.7601…), no mirrors);
  docs/families/quasiperiodic.md §19.
  **Type 12: DONE (M62)** — Rice 1977 family 2a = d = c + e,
  A = 90°, 2B + C = 360° (⇒ C = 360 − 2B, D = 270 − B,
  E = 2B − 180; convex 90° < B < 180°, b the second free
  parameter); pinned to the B = 150° member at c = 1 — a surd
  angle, so (unlike type 11) the shipped instance closes to exact
  Q(√3): A = 90°, B = 150°, C = 60°, D = E = 120°, a = 1,
  b = √3, c = 1, d = 2, e = 1 (area 3√3/2; runtime-computed
  prototile from the closed forms
  a = (sinB + cos2B)/(2cos2B − 2sinB + 1),
  b = cosB − 2a·cosB − (2a − 1)·sin2B, d = 2a, e = 2a − 1,
  surds pinned to 1e-12); 8-tile 2-iso rectangular unit (seeds 0/1:
  {0,3,4,7} / {1,2,5,6}, orbits mixing chiralities), w1 =
  (−3√3/2, −9/2) ⊥ w2 = (2√3, −2), det = 12√3 = 8·area;
  NON-edge-to-edge — every d-side splits at its exact midpoint into
  two unit halves, all other sides full-side matched (b↔b, c↔c,
  e↔e); refined census over 215 tiles: 620 full + 153 split
  (t = 1/2), no other T-junctions; vertex figures {90°,90°,180°} 8×,
  {60°,120°,180°} 9×, {120°,120°,120°} 8×, {60°,150°,150°} 10×
  (35 interior vertices). Group **pgg** by exact symmetry audit of
  the quotient table: perpendicular glides g1 = (M1, (0,−3)),
  g1² = w1, g2 = (M2, (−√3,1)), g2² = −w2, g2∘g1 = half-turn about
  c0 = (√3/4, 5/4); no mirrors (22 negative candidates). 9
  tess-tilings tests; the edge-census window was re-pinned to an
  exact-count window ((−7.45, 13.55) × (−7.6, 11.4), min boundary
  gap 0.0888 from any counted midpoint) after the ~1e-15 float noise
  of the runtime-computed prototile was identified. docs/families/pentagons.md §21.
  **Sprint allocation (2026-11-15): M63 = type 10** (a=b=c+e, A=90°,
  B+E=180°, B+2C=360°; 6-tile 3-iso unit, p2/cmm, NON-edge-to-edge —
  the last animation-bearing type; reference frames + M60/M62 pipeline),
  **M64 = type 3** (a=b, d=c+e, A=C=D=120°, B+E=180°; 3-tile iso unit,
  p3/p31m — conditions-first pin, no animation exists). Then
  types 2, 5 (conditions-first, no animations), type 1 last (topology
  zoo).

  **Sprint allocation (2026-11-16): M65 = type 2** (c = e, d = b,
  B + D = 180° — the edge-to-edge variation; the Wikipedia type-2
  section shows two isohedral pgg 4-tile-primitive-unit forms and marks
  the d = b one edge-to-edge; p2 when mirror tiles are counted
  distinct; conditions-first pin, no animation exists). M66–M71 (UI
  rework: TUI + GUI browser, single-pane mirror, grid fixes, dynamic
  columns) landed and pushed through 16300e1 in the same sprint. Then
  type 5, then type 1 (topology zoo).
  **Type 3: DONE (M64)** — Reinhardt 1918 family 3 (a = b, d = c + e,
  A = C = D = 120°, B + E = 180°); the dossier's e2e column was
  corrected first (d = c+e forces a T-junction; Bagina 2011 /
  Sugimoto 2012: only 8 edge-to-edge convex types, {3, 10, 11, 12,
  13, 14, 15} non-e2e) — this verdict supersedes the earlier
  re-scope note. Landed as `pentagon-3`, but
  NOT as the assumed 3-tile p3/p31m edge-to-edge unit.  The pinned
  instance is the integral member (a, b, c, d, e) = (2, 2, 1, 4, 3),
  A = B = C = D = 120°, E = 60° (area 5√3, exact Q(√3); the β = 90°
  candidate from the note is Cauchy-rigid against any 3-tile hexagonal
  lattice — |w|² = 102 + 66√3 admits no pq + rs = 33).  The classical
  three-tile 120° pinwheel admits **no periodic cell** for this
  member: hexagonal cell-3 dies on the norm form x² + 3y² = 120
  (cell-6/9: 240/360), a complete closure search over every lattice
  of determinant 15 in unit-side coordinates (both chiralities,
  T-junctions admitted, 1,735,315 pairing trials) is empty, and a
  parity lemma kills the R120 pinwheel boundary for every type-3 tile.
  The shipped structure is the discovered minimal periodic cell: **2
  tiles on an oblique lattice, group p2, NON-edge-to-edge** (only the
  d = 4 sides split, at their exact midpoints between two length-2
  partner sides; vertex figures {60°,120°,180°} and {120°,120°,120°}),
  1-isohedral, four half-turn classes d/2 + Λ/2, chiral prototile used
  in one chirality (no mirrors/glides).  docs/families/pentagons.md §23.
   **Type 10: DONE (M63)** — James 1975 family a = b = c + e,
   A = 90°, B + E = 180° (⇒ C = 180 − B/2, D = 90 + B/2,
   E = 180 − B; convex 90° < B < 180°, e/c the second free
   parameter) pinned to the B = 90° member at c = 1: A = B = E =
   90°, C = D = 135°, a = b = 2, c = e = 1, d = √2
   (d² = c² + e²; area 7/2) — isosceles, so the two
   chiralities coincide as vertex sets. 6-tile oblique unit
   w1 = (5,−2), w2 = (8,1), det = 21 = 6·area; 3-isohedral
   (seeds 0/1/2: {0,2}, {1,3}, {4,5}); NON-edge-to-edge — both
   length-2 sides split at their exact midpoints (integer
   T-junctions), refined census over 164 tiles: 518 full + 76 split
   at t = 1/2; vertex figures {90°,135°,135°} 71×,
   {90°,90°,180°} 50×, {90°,90°,90°,90°}
   13× (134 interior vertices). Group **cmm** (the premise "p2"
   was wrong: the tiling has mirrors and a half-turn) by exact
   symmetry audit — point group {I, −I, R1(y=x), R2(y=−x)}
   all four realised (104/104), mirror lines x+y = 3k and
   x−y = 7k, genuine glide g = T(5,−2)∘R2 with
   g² = (7,−7) = 3w1−w2 (axis x+y = 3/2, perpendicular
   offset (3/4,3/4), no pure-reflection conjugate), R90 negative
   (M90@(1,2) unplaced / D0 placed). Centered rectangular lattice:
   L = Z(5,−2)+Z(8,1) = {x−8y ≡ 0 mod 21} has no
   orthogonal Z-basis (min orthogonal pair det 42), while the
   index-2 sublattice L' = Z(3,3)+Z(−7,7) (3√2 × 7√2,
   det 42) has centre (−2,5) ∈ L\L' → cmm, not pmm;
   registered as Oblique since w1·w2 = 38 ≠ 0. 9 tess-tilings
   tests (59 tilings, 409 tests). docs/families/pentagons.md §22.
   **Type 2: DONE (M65)** — Reinhardt 1918 family 2 (c = e,
   d = b, B + D = 180°) — the arc's **edge-to-edge** variation —
   pinned to the exact-Q(√3) member a = 1: A = B = D = 90°,
   C = 120°, E = 150°, b = d = 1 + √3/2, c = e = 1/2 (area
   1 + √3/2, every vertex in Q(√3)). 4-tile rectangular
   primitive unit = **two chiral pairs** (the cell holds two
   direct + two mirror tiles); 1-isohedral (a single orbit: the
   two glide classes identify the chiralities, so all tiles
   carry seed 0). Lattice w1 = (1/2, 1 + √3/2) ⊥
   w2 = (−2 − √3, 1), det = 4 + 2√3 = 4 × area. Group **pgg**
   by exact audit of the quotient table: two perpendicular
   glide families with axes along the lattice directions,
   g1² = w1 and g2² = −w2 (each a true half-translation glide —
   no power is a pure reflection), composing to the half-turn
   about c0 = (1 + √3/2, 0) (the prototile's B vertex — NOT a
   lattice point); the four half-turn classes c0 + Λ/2 are
   realised; no mirrors (22 negative candidates). The prototile
   is chiral yet the tiling uses **both** chiralities —
   colouring the two chiralities turns the glides into pure
   reflections of each colour class and drops the group to
   **p2** (the article's note). EDGE-TO-EDGE at the pentagon
   level — the arc's second (after type 9): a refined edge
   census over 276 tiles finds 1182 full edges and zero
   splits. Vertex figures {90°,120°,150°} 237× +
   {90°,90°,90°,90°} 118× (355 interior vertices over the
   281-tile block window). 9 tess-tilings tests (61 tilings,
   465 tests). docs/families/pentagons.md §24.

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

### 7. Golden bee / Ammann chair (Ammann A2, inflation √φ) ✅ (M104)

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
- **Status:** landed (M104) as `golden-bee`, category Quasiperiodic
  (this module implements exactly Vince's canonical sequence; the
  exact Q(√φ) battery verifies the substitution mechanics, not the
  diffraction limit — the module header documents the scope). 8
  module tests; census 96 → 97 tilings, 108 → 109 memberships,
  Quasiperiodic 8 → 9, 674 → 682 tests (fork-local on 91dd843).
  Details in docs/families/kuniform.md §62 (provisional number; renumber at
  integration).

### 8. Socolar 12-fold (and the shield tiling) — shield DONE (M51) ✅

- **Wheel tiling: PARKED (M61, 2026-11-15) with a proven negative.**
  Attempt 1 (worker) pinned the edge-orthogonal dual map exactly in
  Q(√2,√3) and produced a valid 2-prototile tiling family W_d (chiral
  11-gon W11, area (2+√3)/4, and star 24-gon W24, area 3/2+√3; growth
  → θ² per depth) — but **no substitution exists at any inflation**:
  at θ an algebraic impossibility (n11 + 2√3·n24 = 2+√3 has no
  non-negative integer solution — odd √3 coefficient, scale-free), at
  θ² geometrically refuted (assignment-free union tests 0/270 and
  0/849 at both phases, uniform exact deficits). The TE documents the
  wheel only as a **windowed (cut-and-project) tiling** — no published
  substitution exists to extract. Evidence pack:
  /tmp/opencode/socolar/WHEEL_REPORT.md (248 lines) + m61_*.py.
  Owner decision (2026-11-15): park; the shield remains the class's
  substitution representative. Future options noted in the report:
  CPS/windowed implementation (new engine path) or the static W_0
  patch (both prototiles fully pinned); the inert `wheel = []` feature
  stays registered for a future CPS milestone.

- **Sprint allocation (updated 2026-11-15, coordinator):** the **wheel
  tiling (Nissen)** — shield's edge-orthogonal MLD dual — is **M61**,
  feature `wheel` (registered in default). Construction path: derive the
  wheel as the EXACT dual of the landed shield (`shield.rs`, 80693b5) —
  each edge replaced by the equal-length edge orthogonal at its midpoint
  (Bielefeld "MLD Class Shield and Socolar") — then verify MLD
  consistency (dual tiles ↔ shield tiles at matched depth). Reference
  assets in the task scratch (Bielefeld wheel pages + rule/patch images).

- **Result (M51, 2026-11-15):** the **shield tiling (Gähler)** shipped
  as `shield.rs` (feature `shield`, category Quasiperiodic). Pinned
  rule: inflation **θ = (√6+√2)/2 = 2cos 15°**, θ² = 2+√3 = Perron
  eigenvalue of the 8×8 kind-count matrix (char poly divisible by
  x²−4x+1, PF vector positive); 4 prototiles (shield hexagon, square,
  black/white triangle) × R/L matching labels = 8 substitution kinds
  (shield → 13 children, square → 5, black tri → 1 shield, white tri →
  3); children tile **kinked supertiles** (±¼-unit-square notches:
  shield 6 in, square 4 in, black tri 3 out, white tri 2 out + 1 in)
  on exact Q(√2,√3) anchors; 13-tile kinked-shield seed with centroid
  √2/2 + √6/3 and covered-disk inradius (1+√3)/2. Verified in exact
  Python before porting (per-kind disjointness + closed unit-edge
  boundary + area law, growth 13→31→151→493→1981, published 10-tile
  sequence, dodecagonal 12-direction frame; no local 30° center — the
  12-fold is Bragg/MLD, window a regular dodecagon). Follow-up: the
  **wheel tiling** (Nissen), shield's edge-orthogonal MLD dual, derives
  from the same tables via the MLD map.

- **Sprint allocation (updated 2026-11-14, coordinator):** Socolar-12
  itself is **BLOCKED ON SOURCE** — four attempts (two context-ceiling
  deaths, two API timeouts, zero task failures) culminated in a
  stop-report proving the published substitution figure is a
  *decorated* rule whose second decoration state's child patches appear
  in no accessible source (Klitzing page + Tilings Encyclopedia checked;
  the authoritative Socolar, Phys. Rev. B 39 (1989) 10519 is paywalled;
  exhaustive anchor-shift / 576-case frame / 92×144 decoration-variant
  scans leave zero consistent completions of the drawn tables — evidence
  pack preserved in the task scratch `REPORT.md`). Retargeted per owner
  decision: **M51 = the shield tiling (Gähler)**, feature `shield`,
  as this dossier's dodecagonal deliverable. The **wheel tiling**
  (Nissen) — shield's edge-orthogonal MLD dual — follows as a second
  commit if the shield lands well. A cut-and-project implementation of
  Socolar-12 (canonical, ~1–2 dev-days, new CPS engine path) remains a
  future research item; the socolar scratch's exact-arithmetic and
  pixel-audit tooling transfers.

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
  stripe-decoration layer (docs/ROADMAP.md research area 4).
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
- **Status:** landed (M93) — as a **finite-patch data-table port**
  with **geometric edge-band encoding**, not a generator:
  `jeandel-rao` (`jr_aperiodic.rs`) ships the author's canonical
  100x100 patch (arXiv:1506.06492), each unit cell partitioned into
  5 non-overlapping convex sub-polygons (4 edge bands of width 0.08
  + interior) whose palette colors make the Wang edge matching
  visible (H 0/9900, V 0/9900 on exact 9-color ids, re-derived in
  tests). The Sturmian/substitution generation policy was not
  ported (the study's independent DFS growth is a proven dead end);
  set aperiodicity and subshift minimality stay cited, not proven.
  Details in docs/families/aperiodic.md §43.

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

> **Correction (2026-11-24, pre-dispatch):** instances cannot take
> `Category::Wallpaper` as sketched below — the Wallpaper tab is a
> pinned 17-card group catalog (`lib.rs` asserts `groups.len() ==
> wall.cards.len() == 17`; any new Wallpaper membership, primary or
> secondary, adds an 18th card and fails the guard). Instances take
> `Laves` (Laves-topology types) or `PentagonTypes`; family scaffold =
> the new empty `isohedral` family. Brief:
> [isohedral-generator](briefs/isohedral-generator.md).
>
> **LANDED (2026-11-24, M162):** five instances — IH1 (p1), IH6 (pgg),
> IH7 (p3) hexagons on the 6³ net; IH27 (pgg), IH29 (p4g) pentagons on
> the Cairo net — primary Laves, PentagonTypes secondary on the
> pentagons; exact Q(√3) arithmetic; adjacency symbols derived by
> exact edge matching, never hand-written. IH21 dropped (irreducible
> decimal coefficients). Design doc:
> [isohedral-generator-design.md](docs/isohedral-generator-design.md).

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
    section + the docs/history/CHANGELOG.md M57 entry + this entry's
    Done bullet.
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
- **Status update (M94):** the *tiling-mode* half of this candidate
  landed: the inference engine (ray growth + greedy min-length
  pairing + planar arrangement to bounded faces, Kaplan §3.4) ships
  as `hankin::build_cells` (`hankin_ext`) with three first-class
  configurations — `hankin-4-8-8-rosette` (4.8.8, θK = 67.5°),
  `hankin-4-6-12` (4.6.12, θK = 60°), `hankin-hex-two-point` (6³,
  θK = 45°, δ = 0.5 two-point) — see docs/families/islamic.md §44. The layer-mode
  generalization (one engine × every substrate × 3 openings) remains
  open for further substrates/angles.
- **Status update (M99):** a fourth configuration landed —
  `hankin-3-12-12` on the truncated hexagonal 3.12.12 substrate
  (θK = 45°, δ = 0), the first hankin entry on that substrate and
  the first beside its PIC strapwork twin `pic-3-12-12` (M56):
  the shining twelve-pointed khatam (24-gon star core + 12 petals
  per dodecagon). θK = 45° chosen over 30°/60° (flat / spiky
  limits) for the mid-"shine" twelve-pointed star; p6m verified
  on a ±4-lattice window, D6-exact. See docs/families/islamic.md §53. A fifth
  configuration landed in the same milestone —
  `hankin-square-two-point` (θK = 45°, δ = 0.5 two-point) on the
  square 4.4.4.4 substrate: a decagonal star per unit square
  (four corner triangles, two pentagonal petals, one 10-gon core,
  7 faces); cmm (2*22) verified — the p4m 4-fold hypothesis
  falsified, diagonal mirror families x∓y=k with on- and
  off-mirror 2-folds, 0 unexpected outcomes on a 10-element
  radius-4 window battery. Substrate deviations are documented in
  the module headers. See docs/families/islamic.md §54. A sixth configuration
  landed in M100 — `hankin-3-6-3-6` on the trihexagonal 3.6.3.6
  substrate (θK = 60°, δ = 0): a regular {6/2} hexagram per
  hexagon (12-gon core with 60° tips and 240° notches plus six
  60°-lozenge rhombi, 7 faces) with the two equilateral triangle
  tiles unpaired (0 pairs) — 9 faces per lattice cell
  ⟨(2,0),(1,√3)⟩; p6m verified on a ±5-lattice window (517-face
  interior disk, all 12 D6 elements, 0 mismatches at 1e-6).
  θK = 60° chosen over 30°/45° (no star: core tips 120°/90°),
  67.5° (sharper variant, probed), and 90° (star collapse).
  Distinct from `pic-3-6-3-6` (same substrate, different
  construction — greedy inference vs. reflection chords).
  Deviations (no 40-dps Python prototype; machine-emitted
  expectations) documented in the module header. See docs/families/islamic.md §57. A seventh configuration landed in M100 — `hankin-3-4-6-4`
  on the rhombitrihexagonal 3.4.6.4 substrate (θK = 60°, δ = 0):
  a regular {6/2} hexagram per hexagon (12-gon core plus six
  60°-lozenge rhombi, 7 faces); the three unit squares unpaired
  (0 pairs, single-face); each equilateral triangle tile in the
  side-1/2 4-subdivision limit (star tips reach the edge
  midpoints exactly — 4 faces each) — 18 faces per lattice cell
  ⟨(1.5+√3/2, √3/2+1/2),(0,1+√3)⟩; p6m verified on a ±5-lattice
  window (553-face interior disk, all 12 D6 elements, 0
  mismatches at 1e-6). θK = 60° chosen over 30° (slivers), 45°
  (no hexagon star; asymmetric triangle motifs), 67.5° (sharper),
  90° (star collapse). Deviations documented in the module
  header. See docs/families/islamic.md, the Hankin sixfold rhombitrihex
  motif arc (numbered §56 there). An eighth configuration landed in
  M107 — `hankin-3-3-3-4-4` on the elongated-triangular 3.3.3.4.4
  substrate (θK = 60°, δ = 0; the all-CCW in-cell table — the CW
  down-triangle and CW square emit outward rays and pair with
  nothing at every probed angle): both triangle tiles in the
  side-1/2 4-subdivision limit (4 equilateral faces each) and the
  square as four corner quadrilaterals + a central octagon (5
  faces) — 13 faces per lattice cell
  ⟨(1,0),(1/2,1+√3/2)⟩ (det 1+√3/2, exact Q(√3)); cmm verified
  exactly over Q(√3) on a 551-face disk (adaptive R² — two face
  centroids at exactly distance 5; strict midpoint radius), all
  20 cmm elements 0-mismatch, negative controls fail. θK = 60°
  chosen over 30° (30° slivers), 45°/67.5° (sharper 45° minimum
  interior angle), 90° (triangle pairing degenerates, square star
  collapse). Deviations documented in the module header. See
  docs/families/islamic.md §63. A ninth configuration landed in M111 —
  `hankin-3-3-4-3-4` on the snub-square 3.3.4.3.4 substrate
  (θK = 60°, δ = 0; all twelve instances stored CCW in their
  in-cell positions): each square tile in its five-face motif
  (four corner quadrilaterals + a central octagon, 4 pairs), each
  triangle tile in the side-1/2 4-subdivision limit (3 pairs,
  four equilateral faces each) — 52 faces per lattice cell
  ⟨(1+√3,0),(0,1+√3)⟩ (det 4+2√3); tiles and lattice in Q(√3)
  with all face vertices in (ℤ+ℤ√3)/4 (the exact Q(√2,√3) audit:
  √2 and √6 coefficients identically zero — √2 only as the 45°
  glide-ray direction); p4g verified exactly on a 6292-face
  window: the face-set translation lattice is the centered
  lattice Λ′ = ⟨(L/2,L/2),(L/2,−L/2)⟩ with L = 1+√3 (the
  square-lattice half-steps fail 788/788), 4-fold centers
  {(L/4, L/4+1/2), (L/4, 3L/4+1/2), (3L/4, L/4+1/2), (3L/4,
  3L/4+1/2)}, 2-folds = those ∪ {(0,1/2), (0,(L+1)/2),
  (L/2,1/2), (L/2,(L+1)/2)}, glides off the L/8 grid — 45°
  lines y−x ≡ L/4+1/2, 3L/4+1/2 (mod L) of shift ±(L/4, L/4)
  and 135° lines y+x ≡ L/4+1/2, 3L/4+1/2 of shift ±(L/4, −L/4)
  (785 faces each, 0 mismatch), no mirrors (all 18 t = 0 mirror
  controls fail). θK = 60° chosen over 30° (30° slivers),
  45°/67.5° (sharper 45° minimum interior angle), 90° (triangle
  pairing degenerates, square star collapse). Deviations
  documented in the module header. See docs/families/islamic.md §66
  (provisional).

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
- **Status:** landed (M86) — as a **decorated-hexagon data-table
  port**, not via the substitution hierarchy: `taylor-socolar`
  (`taylor_socolar.rs`) ships the paper's forced 529-hexagon patch
  (Fig. 18), each hexagon partitioned into 13 decorated sub-polygons
  realizing the edge-midpoint stripe handoff rules exactly (AP-STUDY
  handoff; details in docs/families/aperiodic.md §36). The half-hex pseudo-inflation
  generator remains unimplemented, and the plane-tiling
  aperiodicity / limit-periodic hierarchy stays cited, not proven.

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

### 20. Fractalizing k-uniform tilings (2+√3 dodecagon dissection) ✅ (M110)

- **Landed (M110)** as `fractalizing-3.12.12` (`fractalizing.rs`,
  Quasiperiodic, docs/families/quasiperiodic.md §64). The verify step resolved the
  classification against the ground-truth figure
  (`Planar Fractalizing Truncated Hexagonal Tiling II.svg`): the
  family is **limit-periodic, not substitution-self-similar** — the
  levels are tilings of the same plane with child edge (2+√3)^{1−d},
  level d+1 = level d shrunk by 1/(2+√3) about the origin flower, a
  new period per level, each level individually 6-fold periodic
  (the earlier σ-patch substitution reading was disproven and the
  motif retired). Level-1 basis: hexagonal orbit of a 57-tile flower
  + 12 dodecagon classes/cell — 44 T + 12 S + 1 H + 12 D = 69 tiles
  per cell, cell 84 + 48.5√3, all vertices on the exact
  (m + n√3)/6 grid, edge-to-edge closure verified in exact Q(√3);
  level 0 = the uniform 3.12.12 table we already own, at edge 2+√3
  (its cell area equals the level-1 flower cell exactly).
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

### 21. Wallpaper group completion — all 17 groups realized (M76 roadmap queue, closed in M85)

- **Motif:** M76 made the Wallpaper tab a group catalog: all 17
  wallpaper groups have a registered realizer (pg →
  glide-hexagon-strips,
  pm → trapezoid-courses, cm → wavy-courses, pmg → shifted-trapezoid-courses,
  p1 → stepped-hexominoes, p2 → pentagon-3,
  p3 → chiral-cut-honeycomb, p31m → split-triangle-grid,
  p3m1 → alcove-cut-grid, pmm → kuniform-pmm, pgg → herringbone, p4 → windmill, p4m →
  truncated-square, p4g → snub-square, p6 → snub-trihexagonal,
  p6m → hexagon, cmm → elongated-triangular). The
  shared note (`tess_core::browse::wallpaper_gap_note`) rendered
  at the top of the tab in both UIs now reports the completed
  catalog — since M85 all 17 groups are represented, so there is no
  gap to name. (The
  17th group is pmg — the old `WallpaperGroup::Cmc` variant was a 3D
  space group misnamed as a 2D group; M76 renamed it `Pmg`.)
- **Queue (difficulty per audit §4d):** pm, pg (easy); pmg, cm
  (easy–medium); p3, p3m1 (medium); p31m (hard).
  - p1 ✅ **landed (M78)**: the catalog's first p1 realizer — the
    asymmetric 2-step staircase hexomino (unit squares (0,0),(0,1),
    (1,1),(1,2),(2,1),(2,2)) on its own translation lattice
    Λ = ⟨(3,1),(0,2)⟩ (det 6 = area, 1 tile per cell, 1-isohedral,
    edge-to-edge with 180° collinear corners). The queue item
    proposed an oblique chiral motif, but the exhaustive search
    (det-3/6/9/12/18 triominoes + det-5 pentominoes: 0 p1; the
    brief's det-3 L-triomino candidate is flawed at the partition
    level) found the minimal single-orbit p1 at area 6, and its
    lattice measures **centered-rectangular** (declared on the
    primitive cell; centered cell 6×2 with center (−3,1) ∈ Λ) —
    not oblique as first believed.
  - pm ✅ **landed (M79)**: the catalog's first pm realizer —
    trapezoid courses: two courses of trapezoids per 2×1 cell on the
    rectangular lattice Λ = ⟨(2,0),(0,1)⟩ (det 2 = area), four tiles,
    two incongruent trapezoid classes (areas 6/10 and 4/10, slant
    slope ±1/5, intercept 3/10) each with its mirror in x = 1,
    edge-to-edge. The mirror lines are exactly the vertical lines
    x = k; the incongruent prototile classes (c = 3/10 ≠ (1 − m)/2
    = 2/5 with slope m = 1/5) kill the half-turns and horizontal
    mirrors that would make it pmm.
  - pg ✅ **landed (M80)**: the catalog's first pg realizer —
    glide hexagon strips: two horizontal hexagon strips per 1×2 cell
    on the rectangular lattice Λ = ⟨(1,0),(0,2)⟩ (det 2 = area),
    four tiles (fat area 29/40, convex; waist area 11/40, 2 reflex
    vertices; A ≅ T and B ≅ U chiral twins), strip 1 the glide
    image of strip 0 under g(x,y) = (x+1/2, −y) with
    g² = (1,0) ∈ Λ. The only non-translation symmetries are the
    horizontal glides of the single family g·Λ (glide 1/2), on two
    parallel orbits y ∈ 2ℤ and y = 1+2ℤ — no mirrors, no rotations,
    no perpendicular glides (exact 16-coset probe); the strip-1
    finger is forced to x = 1/5 by the glide, edge-to-edge, five
    edge directions, no verticals.
  - cm ✅ **landed (M82)**: the catalog's first cm realizer — wavy
    courses: zigzag courses of height 4/3 (period-2 waves of slope
    ±1/3, peaks 1/3 above the valleys; Z(t−1) + 1 = 4/3 − Z(t)
    stacks the courses exactly), each column-course quadrilateral cut
    by a diagonal seam alternating slope ±1 with column-course parity
    — four triangles per primitive cell of
    Λ = ⟨(1,1),(2,0)⟩ (det 2, the x+y-even lattice, also
    square-metric — degenerate centered-rectangular realization), two
    incongruent classes (areas 1/3 and 2/3). The brief's original
    single-slope seam set had no mirror at all and was re-derived
    before landing; the seam alternation is what makes the vertical
    mirrors x = integer symmetries, with the glides (2j+1−x, y+1)
    (glide (0,1), g² = (0,2) ∈ Λ) on the half-integer axes — and
    since the centering (1,1) ∈ Λ, the mirror and glide
    representatives live in ONE coset of Λ (exact 144-coset probe:
    2 pos / 142 neg — no half-turns, no horizontal mirrors or
    glides, no rot90/diagonal parts).
  - pmg ✅ **landed (M81)**: the catalog's first pmg realizer —
    shifted trapezoid courses: two courses of M79's trapezoid classes
    per 2×2 cell on the rectangular lattice Λ = ⟨(2,0),(0,2)⟩ (det 4 =
    area, 8 tiles per cell): row 0 (y∈[0,1]) is M79's courses (P, mP,
    P', mP'), row 1 (y∈[1,2]) their mirror F(x,y) = (x, 2−y) shifted
    by (1,0) (R0, R1, R2, R3), two congruence classes. Group: vertical
    mirrors x = k, horizontal glides g(x,y) = (x+1, −y) (glide 1,
    g² = (2,0) ∈ Λ, two Λ-orbits y ∈ 2ℤ and y = 1+2ℤ), and half-turns
    about every (m+1/2, n) (two Λ-orbits, even-y/odd-y, exchanged by
    the mirrors); the horizontal mirrors are dead (F(P) = R0+(−1,0) is
    half a period off Λ) — what makes it pmg and not pmm (exact
    128-coset probe: exactly 4 of the 16 cosets positive).
  - p3 ✅ **landed (M83)**: the catalog's first p3 realizer —
    chiral-cut honeycomb: a chiral 3-armed Y carved into a side-2
    regular hexagon from the alternating vertices V0, V2, V4 (each
    arm V_i → W_i stops at W_i = mid(V_i, O) + delta, delta =
    (−1/8, −1/10) rotated 120° with each arm; the inner seams
    W_i → O close the Y) — 3 congruent hexagonal tiles per
    hexagonal primitive cell Λ = ⟨(2√3,0),(√3,3)⟩ (det 6√3 = 3 ×
    area 2√3), one chiral congruence class (tile1 = R120(tile0),
    tile2 = R240(tile0) exactly, no translate). Group p3 by exact
    ×320-integer probe: the only positive cosets of the 11 D6
    linear parts are the zero coset for R120 and R240 (candidate
    census 27/18/18/27/18 and 30 × 6), rot60/180/300 and all six
    reflection families negative, 25/25 lattice translations, 0/6
    off-lattice, rot120 centres in exactly three orbits
    c(i,j) = (√3·i, i+2j). Construction transcribed from the
    HX-STUDY handoff `hx_p3_cell.json` staged below, via the
    scratch emitter (no hand-typed coords) — the handoff's
    complete exact Q(√3) battery reproduced with zero deltas.
  - p31m ✅ **landed (M84)**: the catalog's first p31m realizer —
    split-triangle grid: each small side-2 equilateral triangle of
    the grid split by its orbit triangle — a generic Q(√3) point P0
    near a vertex and its two images under 120° about the triangle
    centroid — into an inner equilateral triangle + 3 corner quads
    (one per edge), the up-triangle split directly and the
    down-triangle its mirror — 8 tiles per primitive cell
    Λ = ⟨(2,0),(1,√3)⟩ (det 2√3 = 2 × area √3), two congruence
    classes (inner triangles achiral, corner quads chiral; 2
    seeds). Group p31m by exact ×240-integer probe: the only
    positive cosets of the 11 D6 linear parts are the zero coset
    for R120, R240 and the three edge-mirror reflection families
    (census 43×5 / 28×6), rot60/180/300 and the three remaining
    reflection families negative (excludes p6, p6m), 25/25 lattice
    translations, 0/5 off-lattice, rot120 centres in exactly three
    orbits (0,0), (1,√3/3), (2,2√3/3) — incidence census
    [on-mirror: T, F, F]: the grid-vertex centres lie on the
    mirrors, both centroid orbits on none — the p31m signature
    (p3m1 would put every 3-fold centre on a mirror). Construction
    transcribed from the HX-STUDY handoff `hx_p31m_cell.json`
    staged below, via the scratch emitter (no hand-typed coords) —
    the handoff's complete exact Q(√3) battery reproduced with zero
    deltas.
  - p3m1 ✅ **landed (M85)**: the catalog's first p3m1 realizer —
    alcove-cut grid: a side-2 triangular grid on the index-3
    sublattice Λ = ⟨(3,−√3),(3,√3)⟩ (det 6√3 = 3 × area 2√3) in
    which every grid triangle is cut from its alcove point — the
    orbit of the generic Q(√3) junction P0 = (8/7, 4√3/9) under the
    edge-line reflections of the up-triangle (six alcove points per
    cell) — to its three vertices: 18 tiles per cell, three
    congruence classes (six tiles each, each class mirror-closed —
    achiral as a set). Group p3m1 by exact ×126-integer probe: the
    only positive cosets of the 11 D6 linear parts are the zero
    coset for R120, R240 and the three grid-mirror reflection
    families (census 69×5 / 42×6), rot60/180/300 and the three
    remaining reflection families negative (excludes p6, p6m), 25/25
    lattice translations, 0/5 off-lattice, rot120 centres in exactly
    three orbits (0,0), (2,0), (4,0) — incidence census
    [on-mirror: T, T, T]: all three centre orbits lie on the mirrors
    — the *333 fingerprint that separates p3m1 from p31m (M84's
    [T, F, F]: exactly one orbit on mirrors). Construction
    transcribed from the HX-STUDY handoff `hx_p3m1_cell.json` staged
    below, via the scratch emitter (no hand-typed coords) — the
    handoff's complete exact Q(√3) battery reproduced with zero
    deltas. Closes the last gap of the 17-group catalog:
    `WALLPAPER_GROUP_GAPS` is now empty.
- **Construction:** per the adoption checklist — Python spike first
  (exact geometry + group probe), then the standard battery + a
  group-audit probe like the M40/M41 coset probes, then registry +
  census + README/family-doc updates.
- **Verify:** the M76 legality test
  (`declared_symmetry_is_legal_and_matches_the_measured_lattice`)
  already enforces (group, lattice kind) legality and
  measured-lattice agreement for any new entry;
  `registry_groups_are_complete_and_canonically_ordered` must be
  extended with the new tab sizes, and the shared gap note (both UIs)
  drops one group name per landing.
- **Status:** p1 landed (M78), pm landed (M79), pg landed (M80),
  pmg landed (M81), cm landed (M82), p3 landed (M83), p31m landed
  (M84), p3m1 landed (M85) — all 17 wallpaper groups now realized,
  one group per milestone; the un-realized queue is empty.
  `WALLPAPER_GROUP_GAPS` is empty and the shared gap note (both UIs)
  reports the completed catalog. The HX-STUDY handoff
  `hx_p3m1_cell.json` has been consumed.

### 23. Bielefeld Tilings Encyclopedia survey — candidate clusters (2026-11-20)

Survey of the substitution index at
<https://tilings.math.uni-bielefeld.de/substitution/> — **257 unique
entries** extracted and cross-mapped against the registry roster
(111 tilings / 123 memberships at survey time). Each entry below was
checked for canonical status and a construction path; machinery notes
state what the codebase is missing, not what the literature owes.

**Already covered (13 direct matches):** Ammann Chair (= golden-bee,
A2), Ammann–Beenker (A5), Penrose rhombs (P3), kite–dart (P2),
Pentagon–boat–star (P1), Darb-i Imam shrine (= girih-quasiperiodic),
pinwheel, chair, sphinx, hat, spectre, shield (Gähler), Stampfli
12-fold. Our fractalizing-3-12-12, turtle, Taylor–Socolar, Jeandel–Rao
and rep-tile entries sit alongside their own literature rather than on
this index.

### 23a. Q(√φ) golden family — cheapest landings, machinery already shipped

Golden bee's field layer (quadratic surd + φ-substitution) serves all
of these; each is a substitution or cut-and-project table.

- **Golden Triangle** (+ coloured variant; cut-and-project, pure
  point) — golden-triangle/golden-bee style dissection; effort S–M.
  ✅ landed M127 as `golden-triangle` (substitution realization).
- **Golden Pinwheel** — φ² inflation, dense tile orientations but
  *FLC*; pinwheel-style generator with a φ lattice; effort M.
- **Fibonacci×Fibonacci** — 3 prototiles, product substitution;
  effort S–M.
- **Cromwell Kite–Rhombus–Trapezium** — mld to Penrose, φ² inflation;
  effort M.
- **Golden Rhomboid–Triangle** — effort S–M. ✅ landed M144 as
  `golden-rhomboid` (Y/O/R φ-substitution: Y→O, O→O+Y+R, R→R+2O;
  PF φ² exact Q(√5); 18 anchored orientation classes → 10 unanchored
  sets under the 120°-rotation quotient; T-junctions intrinsic from
  depth 2, every T-point closing to exactly π; unbounded wedge growth).

### 23b. 8-fold Q(√2) family — next to Ammann–Beenker

- **Ammann A3** and **Ammann A4** — the missing rungs of the owner's
  A1–A4 request (A2 = golden bee ✅, A1 parked: no substitution
  structure, see Parked list). A3/A4 have substitution structure;
  effort M each. **Correction (2026-11-24, TE entries + orchestrator
  recon):** A3 is **φ-inflation** (TE: "uses the golden ratio as
  inflation factor"; three rectilinear Z-chairs, window uncomputed),
  NOT a Q(√2) member — this section's Q(√2) framing holds for A4 only
  (mld to Ammann–Beenker, λ = 2 chair + outlined second prototile).
  Brief: [ammann-a3-a4](briefs/ammann-a3-a4.md); dispatched (worker
  tier, two landings).
- **AB rhomb–triangle** — Ammann–Beenker with the triangle shown;
  effort S. ✅ landed M129 as `ammann-beenker-tri`.
- **Octagonal 1225 tile** — single octagonal prototile, 8-fold;
  effort M.
- **CAST 8-fold (DTO)** — dense tile orientations; needs the DTO
  presentation layer (see 23i); effort L.
- **Hofstetter 4-fold** — limit-periodic, quasiperiodic by
  superposition; effort M.

### 23c. 12-fold Q(√3) family — next to shield/stampfli

- **Watanabe–Ito–Soma 8-fold / 12-fold (+variant)** — classical
  substitution tables; effort M each.
- **Socolar 12-fold A** — closes #8's remaining item; effort M–L.
- **Square–triangle** and **Socolar square–triangle** — dodecagonal
  square–triangle substitutions; effort M each (also popular via the
  Bielefeld DTO entry).
- **Stampfli variants** — the index carries near-duplicates of our
  §6/§22 entries; audit before adopting.

### 23d. Rep-tile / limit-periodic family — next to chair/sphinx/p-pentomino

- **Pregnant chairs**, **Armchair** — chair-family rep-tiles; effort
  S each (chair machinery exists). **Armchair** ✅ landed M128.
  **Pregnant chairs** ✅ landed M136 (λ = √3 two-prototile rule,
  salvage-pinned).
- **Domino/table tiling (+variants)** and **half-hex family** —
  limit-periodic; **not cut-and-project** — dossier correction at
  landing: the entry's own text cites the continuous spectral
  component (Solomyak ETDS 17(3) 1997; Robinson Indag. Math. 10(4)
  1999, "On the table and the chair"), so the model-set claim is
  dropped. Domino ≡ table is ONE entry (index: "Also known as 'table
  tiling.'"). Half-hex also closes the #17 substitution-hierarchy
  gap. **Half-hex** ✅ landed M135 (measured period hierarchy p =
  (3/2)·2^g). **Domino (table)** ✅ landed M138 — limit-periodic via
  approximating periodic tilings: the depth-g patch is itself the
  fundamental cell of the period-(2^g, 2^(g+1)) approximant (measured
  g = 1..6, ratio 2; no proper within-patch period — "full-block"
  manifest, kind-different from half-hex).
- **Period doubling / period tripling / Thue–Morse 2D / Squiral** —
  marginal supersubstitutions; effort S–M each. **Period doubling** ✅
  landed M141 (the 2D product σ⊗σ of the 1D a→ab, b→aa rule — 4 label
  classes on unit squares, PF 4, char poly x⁴−x³−12x²−4x+16, measured
  (2^g, 2^g) full-block manifest g = 1..6, the ratio-1 signature vs
  domino's ratio-2). **Thue–Morse 2D** ✅ landed M143 (the τ⊗τ product
  of the classic 0 → 01, 1 → 10 morphism — all-ones substitution
  matrix, the Steinhaus property; PF 4, char poly x⁴−4x³; measured NO
  translation period at any depth g = 1..6, all (g+1)²−1 proper
  candidate shifts rejected with zero survivors — the aperiodic
  signature vs period-doubling's ratio-1 and domino's ratio-2;
  singular continuous diffraction, JS99). Squiral stays parked for a
  second reason beyond the rule-image-only source: its prototile has
  infinitely many straight edges (a depth-truncation design problem).
  **Period tripling** ✅ landed M146 — the rule was reconstructed from
  the rule image alone (the entry's Info section is empty; literature
  search came up empty, so no source is citable) and pinned by an
  81-offset window match of the entry's patch image against the
  simulated depth-4 tiling: one square prototile, 2 label classes,
  λ = 3, S = [[6,9],[3,0]], PF 9, second eigenvalue −3 = −λ (the
  marginal case); period lattice ⟨(1,−1), (3^g, 0)⟩, axis-aligned
  (3^g, 3^g) strictly tripling per level — the ratio-3 signature;
  closed form B ⟺ v₃(x+y mod 3^d) odd; the ambiguous x+y = 0 line
  (B odd depth / A even depth) is the marginal two-orbit limit
  structure.
- **Tangram** ✅ landed M142 — rep-tile dissection (λ = √2
  substitution S→{M}; M→{L}; L→{M,Q,P,S,S}; Q→{M,S,S}; P→{M,S,S},
  area-conserving, no L child); det=2 two-S seed pinned (all-5-prototile
  seed search negative for det ≤ 40 — first L at g=4, all five
  prototiles at g=5); homochiral P+ (rotation-only rows); non-edge-to-edge
  (measured T-junctions 0,0,0,6,10,18 over g=0..5); limit-periodic on
  the √2^(g+1) block lattice; exact Z[√2] arithmetic.
- **Pentiamond / small rep-tile set** — effort S each.

### 23e. Generalized Godrèche–Lançon–Billard binary — the §9 re-scope candidate

The index hosts both the classical [binary
tiling](https://tilings.math.uni-bielefeld.de/substitution/binary/)
(exactly the tiling machine-refuted in the §9 STOP: improper
substitution, needs Q(2cos π/10) ⊋ Q(√5)) **and** the [Generalized
GLB Binary](https://tilings.math.uni-bielefeld.de/substitution/generalized_binary/)
(Hibma 2015 / Pautze 2017 — the same Pautze paper the STOP pinned):
inflation √(2+2cos(π/n)), (n−1)/2 prototiles for odd n, FLC + finite
rotations + rhomb tiles, identical to the classical binary at n = 5.
**Re-scoping §9 to a specific larger odd n is viable** — n = 7, 9 live
in Q(2cos(π/7,9)) (degree 3), n = 15 in degree 4 — i.e. it needs a
cubic/quartic cyclotomic field layer we do not ship yet. Frame as its
own milestone with a field-layer design doc first; effort L+.
✅ **n = 7 landed (M149 `glb-binary-n7`)** — the K₇ field layer was
already shipped (M137/M139), so no new layer was needed; the n = 9
and n = 15 siblings remain queued (K₉ shipped M139; K₁₅ shipped M139).

### 23f. Girih / Islamic clusters (relevant to the paused girih fork)

- **Andritz Bowtie–Hexagon family** ✅ (landed M151/M152/M153/M160:
  bowtie-hexagon, 10gon-1, 10gon-2, 10gon-3; "12gon-3" in the wild is
  an image-directory alias of 10gon-3, not a separate entry —
  verified md5-identical source images, M160 arc close-out) —
  5-fold FLC model sets with documented substitutions; effort M each.
- **Girih-inspired 14-fold** — ✅ landed M159 (Pautze 2017; see the
  quasiperiodic family doc and `docs/tilings/girih-14-fold.md`).
- Note: the paused `girih-primitive` fork (§ pending owner decision)
  was revived as a brief and **landed M131–M134** (bowtie, rhombus,
  decagon, pentagon as periodic K-exact data tables beside
  shesh-band) — the Andritz Bowtie–Hexagon family and the
  girih-inspired 14-fold below are unblocked.

### 23g. Pinwheel family (dense orientations, FLC)

- **Uberpinwheel**, **cubic / quartic pinwheel**, the 10/13/65-tile
  self-similar variants — all share the pinwheel generator's
  irrational-rotation machinery; effort M each. Golden-pinwheel (23a)
  belongs to both clusters.

### 23h. Einstein family siblings

- **CAP (self-similar hat substitution)** — the natural sibling to
  our metatile hat; the substitution path the hat entry documents but
  does not run; effort M.
- **Hexagonal aperiodic monotile** — the spectre-adjacent soft
  variant; effort M.

### 23i. New-field tiers (cubic/quartic cyclotomic layers required)

- **Danzer 7-fold (+variants)**, **Harriss 9-fold rhomb**,
  **Goodman-Strauss 7-fold rhomb**, **CAST 10-fold**, the
  **Nischke–Danzer deltoids**, **Say-awen 17/21-fold** — each needs
  degree-3/4 arithmetic before any substitution table; effort XL per
  family (design docs first; overlaps #18).

### 23j. Out of scope for the registry as-is

- 1-D Rauzy-fractal / self-similar lattice-gas entries (no 2-D plane
  partition),
- the FASS-curve family (Gosper/Hilbert/Peano/dragon — curve
  decorations, not plane tilings),
- the F₄ recurrent-double-sequence entries (recurrence sequences, not
  substitutions),
- polytopal (3-D) entries.

### 23k. Audit-discovered additions (2026-11-20 UI audit)

Not from the Bielefeld index — found while auditing the tabs against
the owner's report:

- **Duals of the four subscripted 2-uniform pairs** — the Laves tab
  shows 11 of the 15 implemented 2-uniform duals; missing: duals of
  (4⁴; 3³.4²)₁/₂ and (3⁶; 3³.4²)₁/₂. effort M each (dual.rs
  machinery exists). *(The two structurally missing 2-uniform tilings
  (3.12.12; 3.4.3.12) and (3³.4²; 3².4.3.4) and their duals are being
  implemented in this sprint, M118–M121.)*
- **Stack bond, 45° herringbone, three-brick basketweave** — being
  implemented in this sprint (M114–M116); recorded here for the
  changelog trail.
- **Pentagon type 1** — being implemented in this sprint (M117); the
  §5 census note flips to 15/15 with types 4 (cairo) and 5 (floret)
  surfacing on the Pentagon tab as secondary members.
- **Robinson tiles** — the Parked entry stands (machinery duplicates
  penrose.rs); revisit only as a matching-rule decoration layer.
- **Fibonacci p1/p2-type quasiperiodic tilings** — see 23a
  (Fibonacci×Fibonacci).

## Parked / rejected (with reasons)

- **Quaquaversal tiling** — 3-space tiling; no 2D analogue (removed
  from the queue 2026-11-09; arc §7, now
  docs/history/not-applicable.md). 3D generally out of
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

### 22. Islamic star-pattern extensions from Kaplan's dissertation (Ch. 3 extraction, 2026-11-19)

Source: `reference/kaplan_diss_full_print.pdf` — Craig S. Kaplan,
*Computer Graphics and Geometric Ornamental Design* (UW 2002),
Chapter 3 "Islamic Star Patterns" (printed pp. 45–115; full text
extracted to scratch as `kaplan_full.txt`, keyed by PDF page).
Existing coverage to build on: the M53/M54/M56/M57 polygons-in-contact
strapwork engine and the M94 motif-inference tilings — the M94 engine
already implements the **contact angle θ**, the **δ two-point
split** with the lean-backward rule (diss. Fig. 3.5), greedy
**min-cost ray pairing** (diss. §3.4; the naive alternative has
(2n−1)!! pairings = 1·3·5⋯(n−1)), and face/interlace extraction.
The sub-items below are the additive delta, keyed `22a`–`22h`; each
can be adopted independently through the standard checklist.

- **22a. Design-element motif library (stars, rosettes, extension)**
  — diss. §3.5. Parameterized d_n-symmetric motifs for regular
  polygons in *any* substrate, replacing fixed per-tiling configs:
  - **Star (n, r, s, θ):** path = one ray of length 2r from the east
    edge midpoint M; {n/d}s notation reparameterizes as θ = πd/n;
    heuristic s = 2 when n > 6 and θ > 2π/n, else s = 1. Replication:
    reflect/rotate the path by the polygon's symmetry group, keep the
    first s subpaths at each line of reflection (diss. Fig. 3.10).
  - **Rosette (n, r, s, θ, h, φ):** star + one hexagon per
    concavity; path M→G→(ray). Lee's ideal: C on OA with AC = AM,
    G = MM′ ∩ (line through C ∥ OM); natural height α = GH/AM;
    ideal rosette = (n, r, s, π/n, 1, 0). Most large rosettes s = 3
    (central star's s + 1).
  - **Extension operator** (wraps an inner motif one layer out):
    θ′ = θ/2 and r′ = r·[1/cos(π/n) − sinθ·tan(π/n)·cos(π/n − θ)];
    child gets (n, r′, s−1, θ′) and is rotated by π/n. Extended
    rosettes are prominent historically.
  - **Path:** generalizes M94 — a per-tile motif *assignment*
    (star / rosette / extended rosette / inferred) on any substrate
    with regular polygons, instead of three hand-built configs.
  - **Effort** M–L, **tier** P1.
  - **Status:** landed (M163) — `hankin_motifs.rs` with per-tile
    motif assignment (`build_cells_with_motifs`), additive hooks in
    `hankin.rs` (regression-proven byte-identical on the ten existing
    engine-built hankin tilings), three showcase instances
    (`hankin-4-8-8-motifs` p4m, `hankin-hex-motifs` pmm,
    `najm-63-motifs` p6m). Erratum: the extension operator above
    prints the dissertation's θ′ = θ/2 — the battery-pinned reading
    is θ′ = θ − π/n with r′ = r·cos θ/cos(π/n − θ) (coincident only
    at θ = 2π/n; Lee's ideal rosette unchanged).
- **22b. Najm template-tiling generator** — diss. §3.6.1. A
  *parameterized* family of substrate tilings from the [p, q]
  generating triangle, replacing hand-coded templates:
  notation `([p,q]; m_A o_A, m_B o_B, m_C o_C; inflation)` — regular
  p·m_A-gons on the p-fold axis (m_A = 0 allowed; m_B = 1 forbidden),
  oriented v/e on the designated rays AB, BC, CA; inflation symbols
  A=r/B=r/C=r, AB/BC/AC (simultaneous, equal edge lengths), ABC (all
  three), then a sequential inflation list. Four inflation cases
  solved via **absolute trigonometry** (§2.1.4: Θ(x) = circle
  circumference, E(x) = equidistant-curve stretch, T(x) = Θ/E;
  per-geometry Θ = 2πx / 2πsinh x / 2π sin x, E = 1 / cosh x / cos x);
  the simultaneous-pair case is monotone in the common side length x
  (closed form exists; binary search recommended). Leftover space
  fills with bowtie tiles (Fig. 3.21: ([6,3]; 2e, 0, 3e; AC) =
  dodecagons + enneagons + bowties). Constraint: Euclidean Najm
  tilings have group [4,4] or [6,3] only — other groups stay
  hand-coded (Taprats-style) until the notation is extended.
  - **Path:** a tiling-generator module emitting registry tables from
    the notation (Euclidean first); feeds 22a assignments + M94
    inference. Spherical/hyperbolic instances belong to 22f.
  - **Effort** L, **tier** P1.
  - **Status:** landed (M99) — `najm.rs`, feature `najm`, with two
    Islamic substrates: `najm-63-2e-0-3e-ac` (Fig. 3.21, p6m) and
    `najm-44-3e-0-0-a` (Fig. 3.22, p4m). Radii cases 1–4 all
    implemented; hole tiles (bowties, stars) come from
    construction-specific formulas verified in the spike — a generic
    hole decomposition remains open. Deviation: the extracted case-3
    text `T(l) = T(x)/sin(π/n)` is missing a factor ½; re-derived and
    verified at 60 digits (see docs/families/islamic.md §56). Euclidean [4,4]/[6,3]
    only, as before.
- **22c. Rosette transform (Najm ↔ Hankin bridge)** — diss. §3.8.
  A tiling-to-tiling operation: a Najm tiling becomes the
  corresponding Hankin tiling (regular polygons ringed by irregular
  pentagons). Regular n-gon (n ≥ 5, radius r) → inner n-gon rotated
  π/n with radius
  r′ = r·(cos(π/n) − sin(π/n)·tan(π(n−2)/4n)),
  spokes from inner vertices to outer edge midpoints, r′ chosen so
  each spoke = half the inner side length; irregular tiles → inward
  perpendicular bisectors truncated at intersections (inference at
  θ = 90°), with a **cluster-averaging heuristic** for ray
  intersections inside the tile. Explains Bonner's observation that
  distinct tilings can produce the same design; Hankin tilings absorb
  angle mismatches, which is how designs with **11- and 13-pointed
  stars** (Bonner, diss. Fig. 3.34) become constructible at all.
  Bonner's contact-position adjustments (contacts moved off edge
  centers, e.g. 4.8.8 rosettes) remain unformalized in the literature
  — candidate heuristics here or in 22i.
  - **Effort** M, **tier** P1.
  - **Status update (M99): landed.** The transform module
    `rosette_transform.rs` (hankin_ext) implements the §3.8 bridge
    (both r′ forms; per-tile rules: regular n-gons → inner n-gon +
    trapezoid spokes, irregular tiles → perpendicular-bisector faces
    with cluster-averaged intersections), and the first registry
    instance `hankin-4-8-8-rosette-transform` (4.8.8, θK = θE = 45°,
    δ = 0) ships: 93-tile transformed table (16 triangles, 36 quads,
    32 pentagons, 8 hexagons, one 16-gon per cell), edge-to-edge,
    area census |det|, cell table p4m-exact to 6.5e-16 (docs/families/islamic.md §55). Remaining: 11/13-pointed-star angle-mismatch instances
    (need 22b/22e substrates) and Bonner's off-center contact
    adjustments (unformalized).
- **22d. Islamic parquet deformations** — diss. §3.4.1. θ becomes a
  *function of position*: run the M94 inference along a strip of a
  substrate tiling with θ = θ(x) varying smoothly; the result is a
  spatial-animation star pattern (Huff/Hofstadter style, diss.
  Figs. 3.7–3.8) with essentially no repetition.
  - **Path:** per-contact-point θ override in the hankin engine + a
    strip/export mode (naturally an export feature, not a registry
    entry). Continuity is structural: both tiles share the contact
    point, so θ = f(midpoint) preserves the point-reflection identity
    (brief verified against the M57 engine, 2026-11-24; Phase A
    delivers the per-substrate δ-validity envelope that feeds 24h).
    Dispatches only after 22g lands (same three files).
  - **Effort** M, **tier** P1 (rendering feature, no new tiling) —
    brief [parquet-deformations](briefs/parquet-deformations.md).
  - **Status: ✅ LANDED as M165 (2026-11-24).** `walls_with_delta`
    per-contact-point override (walls() byte-identical as the
    const-f wrapper; the per-point change is exactly hoisting the
    sin_cos into the edge loop — continuity is structural because
    both tiles share the midpoint). Three deterministic profiles
    (Linear/Radial/Sine) + `--deform` CLI (honest parse/resolve
    errors). **Phase-A audit finding:** the closure envelope is a
    DISCRETE point set, not an interval — square/truncated-square
    close at exactly {45°, 90°}, trihexagonal at {60°, 90°},
    truncated-hexagonal at {60°} (1°+0.1° sweeps + single-tile and
    interior-exit probes; 784/968 degree-1 exits are interior, i.e.
    genuine open-strand geometry, not window artefacts). Clamp
    policy therefore = snap-to-nearest-closed-angle
    (`snap_to_envelope`, ties→lower) with `envelope_for(substrate)`
    exposed in tess-tilings for the 24h explorer. Geometry finding
    pinned: the §3.4.1 continuity identity is ray-pair-level
    (point-reflected pairs at every shared midpoint); wall-level
    piece equivalence does NOT survive varying f (mixed-angle seam
    tiles), documented rather than forced. Tests 1287 → 1300.
- **22e. Quasiperiodic substrates and (n,c)-monster motifs** — diss.
  §3.10. Lattice-projection (Quasitiler) rhomb patches: an n-D
  integer lattice cut by a 2-plane yields rhombs with angles
  kπ/n; place a regular 2n-gon at every vertex (edge-midpoint
  orientation), inflate simultaneously along each rhomb edge (same
  case-3 equations as 22b). On thin rhombs the placed polygons
  overlap and must fuse into **(n,c)-monsters** (union of two regular
  n-gons with exactly c full edges of each inside the other; defined
  for n > 6, 2 ≤ c ≤ ⌈n/2⌉ − 2; Kepler coined "monster"). Motif
  menagerie for n ≤ 12 at contact angles kπ/n (diss. Fig. 3.40;
  Bonner's (10,2) at 2π/5, Castéra's alternate). 5-D gives Penrose
  P3 rhombs — already in the registry — so this item makes **Islamic
  star patterns on penrose/ammann-beenker substrates**; 4-D and 6-D
  cases new; rosette-transform variants (diss. Fig. 3.42b/d).
  Sub-item **Kepler's Aa** (§3.10.2): the Dessecker–Eberhart
  construction fills 36° rhombs with pentagons/decagons/(10,2)-
  monsters, laid edge-to-edge or offset by φ; five rhombs overlap
  around a pentacle at the origin — a nonperiodic tiling with global
  fivefold symmetry; the pentacle's pentagon ring needs a hand
  modification (diss. Fig. 3.44) before motifs infer well.
  - **Effort** L, **tier** P1/P2.
- **22f. Absolute-geometry star patterns (spherical + hyperbolic)** —
  diss. §3.6.2–3.6.4, §2.1.4. The whole construction is parallel-postulate
  free: inference works verbatim; the absolute star is
  θ = π/2 − φ with tan φ = 1/(E(r)·tan(πd/n)); the absolute rosette
  replaces the parallel-line step with G = ∠CAM-bisector ∩ MM′
  (generalized θ via A′ = rotate A by θ about M; ideal θ = |∠AMM′|);
  absolute extension via 4θ′ + 2∠ABA′ = 2π and T(r′) = T(d)/cos(π/n);
  outlining thickens lines to **equidistant curves** with mitered
  joins Θ(l) = Θ(w)/sin(θ/2). Spherical groups [3,3], [3,4], [3,5]
  (convex hulls of the tilings give **symmetrohedra** — all Platonic
  and Archimedean solids except the snubs); hyperbolic replication in
  the Poincaré disk (Dunham-style). Architecture: geometry-tagged
  generics (Najm's compile-time Euclidean/Spherical/Hyperbolic
  specialization).
  - **Path:** merges with and de-risks roadmap #16 (hyperbolic {p,q}
    arc) — the absolute-geometry core is the same prerequisite; star
    patterns are its first client.
  - **Effort** XL, **tier** P2.
- **22g. Decoration suite completion** — diss. §3.7. Four styles on
  the planar map: **fill** (designs have only 2- and 4-valent
  vertices, so the map is bipartite — 2-colouring as the base;
  traditional multi-colour conventions are future work, §3.11.1),
  **outline** (band width w via equidistant curves; mitered join as
  in 22f), **interlacing** (over-under at every crossing, consistent
  across the plane because it is decided on the *double* fundamental
  map of the orientation-preserving subgroup [p,q]⁺), and
  **outline-interlace composition**. Castéra's "correct" band widths
  (visual balance; §3.11.1) are an open formula-derivation target.
  - **Path:** audit parity with the M53–M57 strapwork engine and
    `tess_core::strapwork` layer mode first; implement the missing
    styles/controls rather than a parallel system.
  - **Effort** M, **tier** P1 (after the audit).
  - **Status: ✅ LANDED as M164 (2026-11-24).** Phase-A audit
    delivered (24 substrates × 8 δ, two windows, 192/192 agreement:
    12 even-degree rows, every classic δ clean with χ=0 torus
    folds); styles implemented in `strapwork.rs` (map/strands/
    parity/faces/mitres/breaks) + `render_svg_decorated` in
    `svg.rs` + `--decoration/--band-w` CLI; registered-module
    cross-check passed (wall segments ≡ partition edges 640/668/
    1030; faces ≡ cells 301/301/491, area gap ≤ 5.3e-15); fill
    colour classes translation-invariant (1518 comparisons, 0
    failures). Deviations documented in the brief report
    (region-gated preconditions replace the bbox margin; strand
    walks turn at deg-2 corners). Castéra band widths and the
    fill-underlay composition remain open follow-ups. Same-day
    follow-up fix: straight-through strand vertices (θ = π) were
    mis-detected as exact mitres via `acos` precision loss near −1
    (θ measured π − 2.1e-8 defeats the 1e-12 no-op test), dividing
    by ~0 and emitting NaN band rings in outline/interlace modes —
    replaced with geometric straight-through detection, emission
    guards, and a 12-row × 4-style finiteness battery.
- **22h. Moroccan finite designs; optimization-based motif
  correction; strange stars** — diss. §3.11. Research tier ❓:
  - Castéra's Moroccan/Andalusi tradition: finite designs around one
    large star (up to 96 points), built from a skeleton of stars and
    *safts*; inference becomes a puzzle-assembly problem over known
    Zellij tile shapes. No construction path chosen yet.
  - Optimization (§3.11.2): replace per-case Bonner corrections
    (moved contact positions/angles) with a search over an aesthetic
    objective; also the proposed tool for interfacing huge stars with
    surrounding low-order symmetry.
  - Strange stars (§3.11.4): which integer sets admit attractive
    periodic star patterns ({8,12}, {9,12}, {9,18} are immediate);
    nearly-regular polygons as a flexibility trick; the "Altair"
    near-miss tiling (diss. Fig. 3.3) induces 4/5/6/7/8-pointed
    stars in one pattern.

### 24. Medallion composition program — star-and-field two-level patterns (planned 2026-11-24)

The program the foundation has been building toward: high-degree
medallions (hexagrams {6/2}, dodecagrams {12/4}, rosettes, n-pointed
stars, khatam rings) placed on a periodic grid/lattice, their rays
absorbed by shapes that fit them exactly, and the interstitial space
filled with an elaborate complementary strap pattern — generalizing
the landed one-off medallion patterns (khatam M53–M56, najm M99,
girih primitives M131–M134, Andritz M151–M160, WIS 12-fold M158,
girih-14 M159, Darb-i Imam patch M161) into a **parameterized
composition system**. Every landed asset the program needs exists:
M57 `walls()` strap continuity over arbitrary hosts, M94 inference +
M163 per-tile motif assignment with the exact ray-landing extension
operator, the exact fields layer (M137/M139: K₇/K₉/K₁₅/L₁₀ over the
quadratic fields), and the §22 render styles.

User-pinned decisions (2026-11-24): **ground-truth-first** (the
general algorithm must reproduce landed cell tables before it draws
anything new), **registry instances** (compositions ship as tilings
with full batteries, not a render-only layer), **periodic nets
first** (Archimedean/Laves hosts + the landed 12/14-fold lattices;
quasiperiodic hosts later).

- **24a. Medallion interface & catalog** — the schema for self-contained
  ornament modules: planar map on a declared convex hull + boundary
  contact points + ray exit angles + internal figure, all in exact
  fields; geometry-agnostic (absolute-geometry instances plug in later,
  22f). Deliverable: design doc first, then exact re-derivations of
  4–6 landed medallions as certification-style instances (khatam
  4.8.8 star, {6/2} hexagram, {12/4} dodecagon rosette, girih decagon
  panels, WIS 12-point star). No registry change.
  - **Effort** L, **tier** strong (design doc) — brief
    [medallion-interface](briefs/medallion-interface.md).
- **24b. Hole-bearing substrates + boundary-matched inference** — the
  core novel algorithm. Hypothesis: given a host substrate with
  medallion-shaped holes whose boundary edges carry the medallion's
  ray data (not free Hankin contacts), inference in the interstice
  yields a consistent complementary pattern, and the classic two-level
  patterns are instances. **Ground-truth bar: reproduce the M158 WIS
  and M161 Darb-i Imam cell tables from the general path before
  generating anything new.** Subsumes the open 22b item (generic hole
  decomposition). Risk: existence/uniqueness of boundary-matched
  solutions; where inference is underdetermined (free θ), document the
  freedom instead of forcing a choice (feeds 24e).
  - **Effort** XL, **tier** frontier-candidate (novel derivation),
    after 24a.
- **24c. (n,c)-monster medallion generator** (the 22e medallion source)
  — overlapping regular n-gons fused at contact angles kπ/n (n ≤ 12,
  2 ≤ c ≤ ⌈n/2⌉−2) as additional catalog entries in exact fields;
  feeds 24a's catalog. Strange-star sets {8,12}, {9,12}, {9,18} first.
  - **Effort** M each, **tier** strong.
- **24d. Composition families on periodic nets** — unit cells with 2–3
  medallion classes (dodecagon medallions at hexagon centers +
  hexagrams at vertices, khatam rings, decagon-and-bowtie pairs);
  ship 2–3 registry instances with the full battery. Test both
  readings of "complementary fill": boundary-matched inference (24b)
  vs dual-net fill in the interstice (`dual.rs` machinery). After 24b.
  - **Effort** L per instance, **tier** worker/strong.
- **24e. Underdetermined-θ search** (the 22h optimization tool, scoped)
  — where 24b documents freedom, search (θ, contact position) against
  an aesthetic objective (band-balance à la Castéra, crossing density);
  validate against Bonner's documented corrections before trusting it.
  - **Effort** L, **tier** strong, after 24b.
- **24g. Bonner polygonal systems** — the *subdivision* path to
  multi-level patterns: nested polygonal systems (4-fold, 5-fold, …)
  subdividing into child polygons with prescribed motifs, producing
  multi-level results the inference path (24b) does not directly
  reach. Build one system exactly, then compare both paths on the
  Darb-i Imam ground truth (M161). Source: Bonner,
  *Islamic Geometric Patterns* (primary source acquisition flagged —
  the dissertation discusses the approach but does not document the
  systems).
  - **Effort** L per system, **tier** strong, source-gated.
- **24h. PIC explorer — angle sweep + programmatic scoring** — the
  generate-and-filter stage the pipeline lacks: sweep δ (and per-tile
  class θ via the M163 motif assignment) over a host tiling, score
  every candidate programmatically (symmetry preservation via
  `classify.rs`; sliver-free via minimum crossing angle and minimum
  face area; strap-line density per area), render ranked galleries via
  the 22g styles. Winners become exact-pin registry candidates —
  exploration proposes, exactness disposes. Also the substrate for
  24e. Deterministic sweep grid + deterministic scoring (no RNG; §15's
  determinism policy untouched).
  - **Effort** M, **tier** fast/worker, after 22g (render-surface
    conventions) and 24a (catalog hosts).
- **24f. Quasiperiodic medallion hosts** — dodecagonal shield (M51) and
  14-fold (M159) hosts with medallion-bearing patches; the
  Voronoi/Delaunay *dual method* (Ammann-bar / Robinson-triangle duals)
  noted as the exact construction path for Penrose-class hosts — the
  centroid dual (M98) is its periodic special case. Gated behind
  CAP/A4 file ownership (aperiodic/quasiperiodic families in flight).
  - **Effort** XL, **tier** frontier.
- **Enabler on demand:** fields-layer §8 items (K₁₆/K₂₀) if 16/20-fold
  medallions enter the catalog.

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
   cell contents, verification; flip the docs/ROADMAP.md queue status;
   annotate the
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

- 2026-11-19 — M100 (second commit): candidate #14 gained a seventh
  landed inference configuration — `hankin-3-4-6-4` on the
  rhombitrihexagonal 3.4.6.4 substrate (θK = 60°, δ = 0, p6m):
  regular {6/2} hexagram per hexagon (12-gon core with 60° tips /
  240° notches plus six 60°-lozenge rhombi — 7 faces); the three
  unit squares unpaired (0 pairs, single-face); each equilateral
  triangle tile in the side-1/2 4-subdivision limit (star tips
  reach the edge midpoints exactly — 4 faces each); 18 faces per
  lattice cell ⟨(1.5+√3/2, √3/2+1/2),(0,1+√3)⟩, det 4√3/2+3;
  p6m verified on a ±5-lattice window (553-face interior disk, 12
  D6 elements, 0 key mismatches at 1e-6). 6 new tests (1 engine
  self-consistency + 5 registry). Census: 92 → 93 tilings,
  104 → 105 memberships, Islamic 15 → 16, 644 → 650 tests
  (landed on main). Landed-note added to candidate #14.
- 2026-11-19 — M100: candidate #14 gained a sixth landed inference
  configuration — `hankin-3-6-3-6` on the trihexagonal 3.6.3.6
  substrate (θK = 60°, δ = 0, p6m): the classical regular {6/2}
  hexagram per hexagon — 12-gon core with 60° tips / 240° notches
  plus six 60°-lozenge rhombi (7 faces); the two equilateral
  triangle tiles pair with nothing (0 pairs) and stay single-face;
  9 faces per lattice cell ⟨(2,0),(1,√3)⟩, det 2√3; p6m verified
  on a ±5-lattice window (517-face interior disk, 12 D6 elements,
  0 key mismatches at 1e-6). 6 new tests (1 engine self-
  consistency + 5 registry). Census: 94 → 95 tilings, 106 → 107
  memberships, Islamic 17 → 18, 662 → 668 tests (landed on main
  after the najm landing; the fork's step numbers were on its own
  base). Landed-note added to candidate #14.
- 2026-11-19 — M99: candidate #14 gained two more landed inference
  configurations — `hankin-3-12-12` (3.12.12 substrate, θK = 45°,
  δ = 0, p6m; the shining twelve-pointed khatam, first hankin entry
  on the truncated hexagonal substrate) and
  `hankin-square-two-point` (square 4.4.4.4 substrate, θK = 45°,
  δ = 0.5 two-point, cmm; a decagonal star per unit square — four
  corner triangles, two pentagonal petals, one 10-gon core).
  Landed-note added to candidate #14.
- 2026-11-19 — §22: Islamic star-pattern extensions extracted from
  Kaplan's dissertation (reference/kaplan_diss_full_print.pdf,
  Ch. 3, pp. 45–115; full text in scratch kaplan_full.txt). Eight
  sub-items 22a–22h with the chapter's formulas transcribed:
  design elements (n,r,s,θ) / (n,r,s,θ,h,φ) + extension recursion;
  the Najm ([p,q]; m o; inflation) template-tiling notation with
  absolute-trig inflation equations; the rosette transform
  r′ = r(cos(π/n) − sin(π/n)tan(π(n−2)/4n)); parquet deformations
  θ(x); quasiperiodic substrates with (n,c)-monster motifs and
  Kepler's Aa; absolute-geometry patterns (spherical/hyperbolic,
  merges with #16); the §3.7 decoration styles; and the research
  tail (Moroccan saft skeletons, optimization correction, strange
  stars). Existing M53–M57/M94 coverage audited — items are the
  additive delta.
- 2026-11-12 — created from the old PATTERNS.MD's proposed/research sections
  (now docs/ROADMAP.md) +
  a literature sweep (20 candidates: 4 P0 quick wins, 8 P1 sprints,
  8 P2 arcs; 8 parked/rejected with reasons).
- 2026-11-17 — M76: (1) pentagon-census coverage line updated —
  `cairo-collinear` merged into `cairo-pentagonal` (same V3.3.4.3.4
  pattern, different pentagon; the card is now "Cairo Pentagonal
  (V3.3.4.3.4)" with the collinear-vertex variant documented in its
  description); (2) new roadmap queue §21: the 8 un-realized
  wallpaper groups (p1, pm, pg, cm, pmg, p3, p3m1, p31m) — the M76
  Wallpaper tab is a 9-of-17 group catalog with a shared gap note,
  and the 17th group is pmg, not cmc (Cmc is a 3D space group; the
  misnamed enum variant was renamed Cmc → Pmg).
- 2026-11-17 — M74: pentagon-census #5 — type-6 dossier row now
  carries the repo-convention conditions (a=d=e, b=c, A=2C,
  C+E=180°) with the Wikipedia-lettering note (W-A=our-B,
  W-B=our-C, W-C=our-D, W-D=our-E, W-E=our-A, sides unchanged; the
  article's B+D=180°, 2B=E is the same family in that convention),
  group recorded as p2 (authored, M74); the remaining-arc line
  drops type 6 (implemented) and now names 1 and 5 as the only
  remaining coverage-audit items.
- 2026-11-20 — §23: Bielefeld Tilings Encyclopedia substitution-index
  survey (257 entries) cross-mapped against the registry — candidate
  clusters 23a–23j (Q(√φ) golden family, 8-fold Q(√2) with Ammann
  A3/A4, 12-fold Q(√3) with Socolar-12 and Watanabe–Ito–Soma,
  rep-tile/limit-periodic, generalized GLB binary as the §9 re-scope
  candidate, girih-family items gated on the paused girih fork,
  pinwheel family, einstein siblings, cubic/quartic field tiers) plus
  23k audit-discovered additions (missing duals of the subscripted
  2-uniform pairs; this sprint's M114–M121 recorded).
- 2026-11-20 — M114–M121: arrangements trio, pentagon type 1,
  2-uniform batch 5 — Arrangements 5 → 8 (M114 stack-bond pmm,
  M115 herringbone-45 pgg — the M40 bond in its 45°-laid canonical
  frame, M116 basketweave-3 pmm on a documented index-3 superlattice);
  PentagonTypes 14 → 15 (M117 pentagon-1, the Reinhardt type-1
  prototype — measured **cmm**, not the folklore p2: the prototile is
  achiral, with two perpendicular mirror families and off-mirror
  half-turns; declared on the a²=3b² special centered-rectangular
  lattice with the legality-test allowance documented); TwoUniform
  15 → 17 and Laves 22 → 24 (M118 kuniform-n2 = the 3-4-3-12 tiling
  (3.12.12; 3.4.3.12) p4m + M120 its kite/triangle dual; M119
  kuniform-n16 = 33344-33434 variant 1 (3³.4²; 3².4.3.4) p4g —
  axis mirrors pure, diagonal cosets true ½-period glides — + M121
  its "key-lock" pentagon dual; both duals face-censused in exact
  ℚ(√3), areas summing to |det|). Census 111/125 → 122/136 (the
  M114–M121 batch landed before M123/M127–M129 on main). Still
  missing from the closed 20-list: n6, n7 (the two [3.4².6; 3.6.3.6]
  forms) — **n17** ✅ landed M140 (the pgg sibling of n16: t = 3,
  e = 6; exact Q(√3) proof that the emission cell is the primitive
  period cell; pgg measured — 4 two-fold centers in 2 group orbits,
  one glide per axis, 0/295 mirror candidates).
- 2026-11-20 — M122: classification schema + UI audit fixes —
  `TilingClass` (optional per-family fields in tess-core, declared in a
  new `tess-tilings/src/classify.rs` table covering all 111 tilings;
  UIs render only the fields that are present — nothing renders as
  "N/A"); card line 2 is now schema-composed (skips segments the name
  already states; description fallback when it adds nothing) and the
  nonperiodic "— · —" symmetry line is replaced by the declared
  translational character + point order ("quasiperiodic · 5-fold");
  tab orders are now declared-sort-driven (Pentagon types sequential
  2–15 with cairo=4/floret=5 joining as typed secondary members,
  Two-Uniform by vertex configuration with the ₁/₂ pairs adjacent,
  Laves by rank regulars→proper→duals, Wallpaper by the group's
  conventional lattice — p1/p2 oblique … p3*/p6* hexagonal, fixing
  the M76 measured-lattice sections); Ctrl+wheel resizes browse cards
  (width 22–44, height at half, thumbnail cache follows the size key);
  census 111/123 → 111/125 (PentagonTypes 12 → 14).
- 2026-11-20 — M123: preview-grid resizing everywhere — the owner's
  report ("the preview grid doesn't resize on mouse/trackpad or +/-
  actions") fixed in both UIs. TUI: the +/- keys (with =/_ variants)
  now resize the browse cards in Browse mode (previously only zooming
  the canvas in Render), and ctrl+wheel is joined by shift+wheel as an
  accepted alias (several terminals swallow or strip Ctrl on wheel
  events); Browse status bar now advertises "+/- card size". GUI: the
  browse card grid gained a real zoom — ctrl+wheel / trackpad pinch
  (egui's `zoom_delta`, which already diverts ctrl-scroll away from
  scrolling, so the grid never scroll-jumps) and +/- keys scale the
  card target width 150–480 px (keys step 16 px, guarded by
  `!ctx.wants_keyboard_input()` so the search box keeps its
  characters); the preview image scales with the card (4:3 kept,
  120×90 at default → 240×180 at max) and the card height follows
  (`card_inner_h`/`card_total_h` factored from the image height — the
  fixed 168 px `CARD_INNER_H` constant retired), stale preview
  textures of other sizes are pruned on every actual resize (cache is
  size-keyed); the grid keeps filling the window width exactly at
  every zoom level (headless-pass tested at default and zoomed sizes).
- 2026-11-20 — M127: **Golden Triangle** landed as `golden-triangle`
  (§23a ✅) — the √(−τ) reptile substitution: right triangle (legs 1,
  √τ; hypotenuse τ) dissected by its altitude into two similar
  children (scales τ^−1, τ^−1/2; area law τ^−2 + τ^−1 = 1), complex
  λ = i√τ inflation in exact Q(λ), uniform rule, chirality in the
  vertex order, finite covered patch; category Quasiperiodic on the
  Danzer–van Ophuysen 2000 family (the cut-and-project variant above
  stays open). Census: Quasiperiodic 10 → 11, 120/134 at the commit.
- 2026-11-20 — M128: **Armchair** landed (§23d ✅) — the L-tetromino
  rep-4, chair machinery mirrored: unique dissection (exhaustive
  enumeration, one child per orientation, rule forced), non-edge-to-
  edge T-junction at (2,2), limit-periodic with depth-g period
  (2^(g+1), 2^(g+1)). Census: Nonperiodic 4 → 5, 120/134 at the
  commit. **Pregnant chairs** remains open (effort S).
- 2026-11-20 — M129: **AB rhomb–triangle** landed as
  `ammann-beenker-tri` (§23b ✅) — the A5 substitution with the
  half-square triangles emitted as their own tile class (3 prototiles,
  hypotenuse-pairing step removed, same silver-mean deflation);
  PF-orbit census and edge-to-edge growth verified in exact Q(√2).
  Census: Quasiperiodic 11, 120/134 at the commit. Registry end state
  after the M127–M129 batch: 122 tilings / 136 memberships, pinned by
  the tess-tilings census test.
