# Two-uniform (k-uniform) and duals family

The k-uniform (two-uniform) tables and their face-centroid duals: ids are the Wikipedia 2-uniform numbers; every dual is a Laves-tab primary entry.

Code: `crates/tess-tilings/src/families/kuniform/` — cargo feature `kuniform` (default-on).
Adding a tiling here touches only this family: `families/kuniform.rs`
(module decl, register slice, classification arms, census `IDS`,
`lattice_tilings` entries) plus the new module and its doc row below.
Work briefs: `docs/briefs/TEMPLATE.md`; fleet protocol: `tessellix/AGENTS.md`.

## Implemented (37 tilings)

Vertex-2-uniform tilings as `LatticeTiling` data tables; ids are the
Wikipedia 2-uniform numbers (`n*`), plus the two named classic forms
(`kuniform-p6m`, `kuniform-pmm`). The two (3⁶; 3⁴.6) flower forms are
distinguished by subscript since the M113 naming audit: n20 ₁ is the
chiral p6 form (index-13 sublattice flowers, no mirrors), n19 ₂ the
achiral p6m form (index-9 flowers, both mirror classes pass exactly).

| id | motif | group | notes |
|---|---|---|---|
| `kuniform-p6m` | 2-uniform (3⁶; 3².6²) | p6m | hexagonal tiling with one of every three hexagons (index-3 sublattice m ≡ n mod 3 of the honeycomb centers) dissected into six-triangle fans; Wikipedia `2-uniform n10`, t = 2, e = 3 |
| `kuniform-pmm` | 2-uniform (3.6.3.6; 3².6²) | pmm | hexagonal tiling with rows of up- and down-pointing triangles filling the gaps between rows of hexagons (rectangular lattice, hexagons share full horizontal edges); Wikipedia `2-uniform n11`, t = 2, e = 3 |
| `kuniform-n18` | 2-uniform (3⁶; 3².4.3.4) | p6m | squares + two triangle orbits around the 3⁶ vertices; 60° hexagonal basis, \|b1\| = \|b2\| = 1+√3, det 3+2√3 (left-handed); origin a 3⁶ vertex = 6-fold center; 11 tiles/cell; Wikipedia `2-uniform n18`, t = 3, e = 3 (M87) |
| `kuniform-n9` | 2-uniform (3.4.6.4; 3².4.3.4) | p6m | hexagons ringed by squares with triangle fills; 60° hexagonal basis, \|b1\| = \|b2\| = 2+√3, det 6+7√3/2 (left-handed); origin a hexagon center = 6-fold center; 15 tiles/cell; Wikipedia `2-uniform n9`, t = 4, e = 4 (M88) |
| `kuniform-n8` | 2-uniform (3.4.6.4; 3³.4²) | p6m | the second p6m 3.4.6.4 form — hexagons ringed by squares with triangle fills, one square–square edge per cell (2-folds at square–square edge midpoints, cf. n9's triangle–triangle); same basis class as n9 (det 6+7√3/2); origin a hexagon center; 15 tiles/cell; Wikipedia `2-uniform n8`, t = 4, e = 4 (M89) |
| `kuniform-n5` | 2-uniform (3.4.6.4; 3.4².6) | p6m | hexagons ringed by squares and triangles; 60° hexagonal basis, \|b1\| = \|b2\| = 3+√3, det 9+6√3 (right-handed); origin a hexagon center, 6-fold center c the cell's minority-hexagon center (not O); 18 tiles/cell; the 3.4².6 vertex orbit is chiral (three figure strings, M90 report); Wikipedia `2-uniform n5`, t = 5, e = 5 (M90) |
| `kuniform-n1` | 2-uniform (4.6.12; 3.4.6.4) | p6m | the 3-4-6-12 tiling: dodecagons alternating with hexagons, squares and triangles; 60° hexagonal basis, \|b1\| = \|b2\| = 2+2√3, det 12+8√3 (right-handed); origin a hexagon center, 6-fold center c the cell's dodecagon center (not O); 12 tiles/cell; Wikipedia `2-uniform n1`, t = 4, e = 4 (M91) |
| `kuniform-n13` | 2-uniform (3⁶; 3².4.12) | p6m | dodecagons ringed by squares and two triangle orbits around 3⁶ vertices; 60° hexagonal basis, \|b1\| = \|b2\| = 3+√3, det 9+6√3 (right-handed); origin a 3⁶ vertex, 6-fold center c the cell's dodecagon center (off-ring, not O); 16 tiles/cell; Wikipedia `2-uniform n13`, t = 4, e = 4 (M92) |
| `kuniform-n20` | 2-uniform (3⁶; 3⁴.6) | p6 | flower form: unit triangular lattice, hexagons dissected at the points of the index-13 sublattice ⟨(1,3),(−3,4)⟩; 60° hexagonal basis, \|b1\| = \|b2\| = √13, det 13√3/2 (right-handed); origin a hexagon center = 6-fold point; 21 tiles/cell; the two-uniform family's first p6 entry — R60 PASS + six mirror probes FAIL + full translation scan pin p6; Wikipedia `2-uniform n20`, t = 5 (e = 6 exact; the wiki parenthetical's e = 7 disagrees, see §46) (M96) |
| `kuniform-n12` | 2-uniform (3².6²; 3⁴.6) | cmm | rows of edge-sharing hexagons with unit-triangle bands between the rows; centered rectangular primitive basis ⟨(√3,0),(√3/2,5/2)⟩, det 5√3/2 (right-handed) — the centering vector is a basis vector; origin a hexagon center = 2-fold point; 5 tiles/cell (1 hex + 4 tri; the 10-tile centered cell is S ⊎ (S + c)); the two-uniform family's first cmm entry — six 2-fold probes + both mirror families PASS, valley 2-folds FAIL, no 3/4/6-fold; Wikipedia `2-uniform n12`, t = 2, e = 4 (both confirmed exactly) (M97) |
| `kuniform-n19` | 2-uniform (3⁶; 3⁴.6)₁ | p6m | the t = 3 flower form of the 3⁴.6 pair: unit triangular lattice, hexagons dissected at the points of the index-9 sublattice ⟨(3,0),(0,3)⟩; 60° hexagonal basis, \|b1\| = \|b2\| = 3, det 9√3/2 (right-handed); origin a hexagon center = 6-fold point; 13 tiles/cell (1 hex + 6 petals + 6 gap triangles); Wikipedia `2-uniform n19`, t = 3, e = 3 — e = 3 confirmed under the full group (edge orbits {6,12,3}; under p6 alone the petal edges split 6+6 giving e = 4); the wiki table's p6 label under-states the symmetry: both mirror classes (axes 0°/30°/60°/90°) pass exactly, so the group is p6m — n20 is the genuinely chiral p6 form (M95) |
| `kuniform-n4` | 2-uniform (4⁴; 3³.4²)₁ | cmm | staggered two-square-tall bands alternating with alternating-triangle bands one triangle high; centered rectangular basis ⟨(1,0),(1/2, 2+√3/2)⟩ with orthogonal index-2 pair (1,0), (0, 4+√3), det 2+√3/2 (right-handed); 4 tiles/cell (2 squares + 2 triangles); Wikipedia `2-uniform n4`, t = 2, e = 4; vertex orbits 4444 (1 class/cell) + 33344 (2 classes/cell) (M103) |
| `kuniform-n14` | 2-uniform (3⁶; 3³.4²)₁ | pmm | single square rows alternating with double triangle strips — two counterposed unit-triangle rows, strip height √3; rectangular (primitive) basis ⟨(1,0),(0,1+√3)⟩, det 1+√3 (right-handed); 5 tiles/cell (1 square + 4 triangles in three orbits {strip1-up, strip2-down}, {strip1-down, strip2-up}); Wikipedia `2-uniform n14`, t = 3, e = 4; 3⁶ vertex orbit 1 class/cell, 33344 2 classes/cell (M103) |
| `kuniform-n3` | 2-uniform (4⁴; 3³.4²)₂ | cmm | staggered three-square-tall bands alternating with alternating-triangle bands one triangle high — the n4 sibling; centered rectangular basis ⟨(1,0),(1/2, 3+√3/2)⟩ with orthogonal index-2 pair (1,0), (0, 6+√3), det 3+√3/2 (right-handed); 5 tiles/cell (3 squares + 2 triangles in orbits {rowA, rowC}, {rowB}, {triangles}); Wikipedia `2-uniform n3`, t = 3, e = 5; 4444 and 33344 vertex orbits, 2 classes/cell each (M108) |
| `kuniform-n15` | 2-uniform (3⁶; 3³.4²)₂ | cmm | staggered square rows alternating with triple triangle strips — three unit-triangle rows, every horizontal edge shared back-to-back, square rows flanked by one triangle orbit; centered rectangular basis ⟨(1,0),(1/2, 1+3√3/2)⟩ with orthogonal index-2 pair (1,0), (0, 2+3√3), det 1+3√3/2 (right-handed); 7 tiles/cell (1 square + 6 triangles in orbits {flanker pair}, {yellow pair}, {cyan pair}); Wikipedia `2-uniform n15`, t = 4, e = 5; 3⁶ and 33344 vertex orbits, 2 classes/cell each (M108) |
| `kuniform-n2` | 2-uniform (3.12.12; 3.4.3.12) | p4m | the 3-4-3-12 tiling: dodecagons kissing flat-edge-to-flat-edge on a square lattice, 1 unit square at the origin + 4 unit edge triangles (apex = dodecagon corner) filling each four-dodecagon hole; square basis ⟨(2+√3,0),(0,2+√3)⟩, det 7+4√3 exact; 6 tiles/cell; Wikipedia `2-uniform n2`, t = 3, e = 3 (torus orbits 3 tiles / 2 vertex classes / 3 edges — matching the wiki parenthetical exactly); p4m by coset audit (order 8 mod L, D4, no true glides) (M118) |
| `kuniform-n16` | 2-uniform (3³.4²; 3².4.3.4) | p4g | staggered square rows and triangle bands in the snub-square-related form, variant 1 of the wiki's 33344-33434 tiling (variant 2 = n17, pgg — landed below); 45°-rotated square basis ⟨(h,−h),(h,h)⟩ with h = (3+√3)/2, det 6+3√3 exact; 18 tiles/cell (6 squares + 12 triangles; 4 vertex classes of 3³.4² and 8 of 3².4.3.4); Wikipedia `2-uniform n16`, 4-isohedral / 5-isotoxal (torus census 4 tile orbits of sizes 8+4+4+2 + 5 edge orbits); p4g by coset audit — axis-reflection cosets pure, both diagonal cosets true glides (axial shift 3/4 of a diagonal lattice period, no integral shift cancels) (M119) |
| `kuniform-n17` | 2-uniform (3³.4²; 3².4.3.4)₂ | pgg | variant 2 of the wiki's 33344-33434 tiling, the pgg sibling of n16: stair-stepped unit squares and triangles (triangles in groups of 4, squares in groups of 2); rectangular (primitive) basis ⟨(1/2, 1+√3/2), (−2−√3, 1)⟩ (right-handed, b1 ⊥ b2, \|b2\| = 2\|b1\|, det 4+2√3 exact); 12 tiles/cell (4 squares + 8 triangles in orbits 4+4+4); 4 2-fold centers/cell in two group orbits, one glide per axis, no reflections — coset audit: period lattice exactly L (0 of 12 square-centroid coset reps pass, 0 of 1942 short shifts; 1097 mirror candidates, 295 full-checked, 0 reflections); Wikipedia `2-uniform n17` (figure 2-uniform_n17-1.svg), 3-isohedral / 6-isotoxal (torus census 3 tile orbits 4+4+4 + 6 edge orbits 4+4+4+4+2+2 + 2 vertex orbits 4+4) |
| `kuniform-n6` | 2-uniform (3.4².6; 3.6.3.6)₂ | cmm | the (3.4².6; 3.6.3.6)₂ form: point-touching flat-top unit hexagon rows (width 2) alternating with full unit-square rows, each inter-hexagon gap holding a counterposed up/down unit-triangle pair sharing the hexagon touch point as apex; centered-rectangular lattice, primitive basis ⟨(2,0),(1,1+√3)⟩ (conventional cell 2 × (2+2√3), centering vector (1,1+√3)), det 2+2√3 exact; 5 tiles/cell (1 hex + 2 squares + 2 triangles); origin at the 3.4².6 vertex (0,0); Wikipedia `2-uniform n6` (figure 2-uniform n6.svg, caption links the rhombitrihexagonal-tiling related-tilings section), t = 3, e = 4 (the exact enumeration gives 5 tile orbits — reported as a deviation, cf. n20; e quoted, not re-enumerated); cmm: 2 2-folds/cell, one OFF the mirror lines (0, 1/2+√3) — the cmm-vs-pmm discriminator, h-mirrors y = √3/2, 1+3√3/2, v-mirrors x = 1/2, 3/2, true glides both axes; vertex figures 3.6.3.6 ("3636", 1 class) + 3.4².6 in two cyclic orderings ("3446": 2 classes, "3644": 2 classes) (M148) |
| `kuniform-n7` | 2-uniform (3.4².6; 3.6.3.6)₁ | pmm | the (3.4².6; 3.6.3.6)₁ form — the pmm sibling of n6: point-touching flat-top unit hexagon rows (width 2, hexagon centers on x ≡ 0 mod 2) alternating with full unit-square rows (S on the hexagon top edge, S over the inter-hexagon gap), each inter-hexagon gap holding a counterposed up/down unit-triangle pair sharing the hexagon touch point (odd x, y = √3/2 mod B) as apex; rectangular P basis ⟨(2,0),(0,1+√3)⟩, b1 ⊥ b2, det 2+2√3 exact; 5 tiles/cell (1 hex + 2 squares + 2 triangles); origin at the 3.4².6 vertex (0,0); Wikipedia `2-uniform n7` (figure 2-uniform n7.svg), t = 4, e = 4 (the exact enumeration gives 5 tile orbits — reported as a deviation, cf. n20; e quoted, not re-enumerated); pmm: 4 2-folds/cell, all on mirror intersections (0,√3/2),(0,1/2+√3),(1,√3/2),(1,1/2+√3) — no off-mirror 2-folds, the pmm-vs-cmm discriminator; h-mirrors y = √3/2, 1/2+√3, v-mirrors x = 0, 1, no true glides; vertex figures 3.6.3.6 ("3636", 1 class) + 3.4².6 in two cyclic orderings ("3446": 2 classes, "3644": 2 classes) (M148) |

### Two-uniform duals (face-centroid duals, Laves tab)

Face-centroid duals of the tables above, constructed by the `dual()`
helper (first batch M98, second batch M102, rectangular/cmm batch
M106, square-lattice batch M120–M121, (4⁴; 3³.4²)₁/₂ cmm dual pair
M145, (3⁶; 3³.4²)₁/₂ pmm/cmm dual pair M147). Every dual is a Laves-tab primary entry.
| id | motif | group | notes |
|---|---|---|---|
| `kuniform-dual-p6m` | dual of 2-uniform (3⁶; 3².6²) | p6m | face-centroid dual of `kuniform-p6m`, constructed by the `dual()` helper (M98); 7 faces/cell — 1 regular hexagon (side 1/√3, area √3/2) + 6 congruent quadrilaterals (sides 1/√3, 2√3/3, 2√3/3, √3; angles 60°/60°/120°/120°; area 2√3/3); Σ = 9√3/2 = \|det\|, edge-to-edge, 6-fold center at the origin (M98) |
| `kuniform-dual-n18` | dual of 2-uniform (3⁶; 3².4.3.4) | p6m | face-centroid dual of `kuniform-n18`, constructed by the `dual()` helper; 7 faces/cell — 1 regular hexagon (side 1/√3) + 6 congruent pentagons (one side 1/√3, four sides (3+√3)/6; angles 90°/90°/120°/120°/120°; area (2+√3)/4); Σ = 3+2√3 = \|det\|, edge-to-edge, origin a 3⁶ vertex = 6-fold center (M98) |
| `kuniform-dual-n9` | dual of 2-uniform (3.4.6.4; 3².4.3.4) | p6m | face-centroid dual of `kuniform-n9`, constructed by the `dual()` helper; 12 faces/cell — 6 congruent quadrilaterals (two sides (3+√3)/6, two sides (1+√3)/2; angles 60°/90°/90°/120°; area (3+2√3)/6) + 6 pentagons congruent to the n18-dual pentagons (area (2+√3)/4); Σ = 6+7√3/2 = \|det\|, edge-to-edge, origin a hexagon center = 6-fold center (M98) |
| `kuniform-dual-n8` | dual of 2-uniform (3.4.6.4; 3³.4²) | p6m | face-centroid dual of `kuniform-n8`, constructed by the `dual()` helper; 12 faces/cell — 6 quadrilaterals (same shape as the n9-dual quadrilaterals) + 6 congruent pentagons (two sides 1/√3, two sides (3+√3)/6, one side 1; angles 90°/90°/120°/120°/120°; area (2+√3)/4 — equal area, different shape than the n18/n9-dual pentagons); Σ = 6+7√3/2 = \|det\|, edge-to-edge, origin a hexagon center (M98) |
| `kuniform-dual-n5` | dual of 2-uniform (3.4.6.4; 3.4².6) | p6m | face-centroid dual of `kuniform-n5`, constructed by the `dual()` helper; 18 faces/cell — 6 quadrilaterals (sides (3+√3)/6, (1+√3)/2, (1+√3)/2, (3+√3)/6; angles 120°/90°/60°/90°; area (3+2√3)/6) + 12 congruent quadrilaterals (sides 2/√3, (1+√3)/2, 1, (3+√3)/6; angles 120°/60°/90°/90°; same area, different shape — two quadrilateral classes, the only dual with same-arity twin classes); Σ = 9+6√3 = \|det\|, edge-to-edge, 6-fold center at (3+√3, 2+2√3) (M102) |
| `kuniform-dual-n1` | dual of 2-uniform (4.6.12; 3.4.6.4) | p6m | face-centroid dual of `kuniform-n1`, constructed by the `dual()` helper; 18 faces/cell — 12 congruent 30°/60°/90° triangles (sides 1+√3, (3+√3)/2, (1+√3)/2; area (3+2√3)/4) + 6 congruent quadrilaterals (sides (1+√3)/2, (3+√3)/6, (3+√3)/6, (1+√3)/2; angles 60°/90°/120°/90°; area (3+2√3)/6); Σ = 12+8√3 = \|det\|, edge-to-edge, 6-fold center at (3(1+√3)/2, (3+√3)/2) (M102) |
| `kuniform-dual-n13` | dual of 2-uniform (3⁶; 3².4.12) | p6m | face-centroid dual of `kuniform-n13`, constructed by the `dual()` helper; 14 faces/cell — 2 regular hexagons (side 1/√3, area √3/2) + 12 congruent quadrilaterals (sides 1/√3, (3+√3)/6, (3+√3)/2, 1+2/√3; angles 120°/120°/90°/30°; area (9+5√3)/12); Σ = 9+6√3 = \|det\|, edge-to-edge, 6-fold center at (5(1+√3)/2, 5(3+√3)/2) (M102) |
| `kuniform-dual-n20` | dual of 2-uniform (3⁶; 3⁴.6) | p6 | face-centroid dual of `kuniform-n20`, constructed by the `dual()` helper; 12 faces/cell — 6 regular hexagons (side 1/√3, area √3/2) + 6 congruent pentagons (sides 1/√3 ×3, 2/√3 ×2; angles 60°/120°/120°/120°/120°; area 7√3/12); Σ = 13√3/2 = \|det\|, edge-to-edge, 6-fold center at the origin; chiral p6 — all six mirrors fail (matches the primal) (M102) |
| `kuniform-dual-pmm` | dual of 2-uniform (3.6.3.6; 3².6²) | pmm | face-centroid dual of `kuniform-pmm`, constructed by the `dual()` helper; 3 faces/cell — 1 rhombus (side 2/√3, angles 60°/120°, area 2√3/3) + 2 congruent quadrilaterals (sides 1/√3, 2/√3, 2/√3, √3; angles 60°/60°/120°/120°; area 2√3/3); Σ = 2√3 = \|det\|, edge-to-edge, four 2-fold centers per cell (0,0), (1,0), (0,√3/2), (1,√3/2) all on mirror intersections, mirrors horizontal/vertical through all of them (M106) |
| `kuniform-dual-n12` | dual of 2-uniform (3².6²; 3⁴.6) | cmm | face-centroid dual of `kuniform-n12`, constructed by the `dual()` helper; 4 faces/cell — 2 congruent quadrilaterals (sides 1/√3, 2/√3, 2/√3, √3; angles 60°/60°/120°/120°; area 2√3/3) + 2 congruent pentagons (sides 1/√3 ×3, 2/√3 ×2; angles 60°/120°/120°/120°/120°; area 7√3/12 — the n20-dual pentagon); Σ = 5√3/2 = \|det\|, edge-to-edge, 2-folds at (0,0) and (√3/2,0) on the mirror intersections plus the glide-only (off-mirror) centers (√3/4,5/4) and (3√3/4,5/4) (M106) |
| `kuniform-dual-n19` | dual of 2-uniform (3⁶; 3⁴.6) | p6m | face-centroid dual of `kuniform-n19`, constructed by the `dual()` helper; 8 faces/cell — 6 congruent pentagons (the 3⁴.6 orbit; sides 1/√3 ×3, 2/√3 ×2; area 7√3/12) + 2 regular hexagons (the 3⁶ orbit; side 1/√3, area √3/2); Σ = 9√3/2 = \|det\|, edge-to-edge, 6-fold center at the origin (the primal's flower center, where the six pentagons meet), 2-folds additionally at (3/2,0), (3/4,3√3/4), (9/4,3√3/4) (M106) |
| `kuniform-dual-n2` | dual of 2-uniform (3.12.12; 3.4.3.12) | p4m | face-centroid dual of `kuniform-n2` — same square lattice ⟨(2+√3,0),(0,2+√3)⟩, det 7+4√3, inherited; 8 faces/cell over the primal's 8 vertex orbits — 4 congruent kites (the 3.4.3.12 orbit; sides (3+√3)/6, (2+√3)/√3, (2+√3)/√3, (3+√3)/6; angles 90°/120°/30°/120°; area 3/4+5√3/12) + 4 congruent isosceles triangles (the 3.12.12 orbit; sides (2+√3)/√3 ×2 over base 2+√3; angles 120°/30°/30°); Σ = 7+4√3 = \|det\|, edge-to-edge, 4-fold center at the origin; p4m — every orientation-reversing coset a pure reflection (zero glide cosets), face/vertex/edge orbits 2/3/3 matching the primal's e = 3 (M120) |
| `kuniform-dual-n16` | dual of 2-uniform (3³.4²; 3².4.3.4) | p4g | face-centroid dual of `kuniform-n16` — same 45°-rotated square lattice ⟨(h,−h),(h,h)⟩, h = (3+√3)/2, det 6+3√3, inherited — the key-lock tiling; 12 faces/cell over the primal's 12 vertex orbits — 8 congruent pentagons (the 3².4.3.4 orbit; sides (3+√3)/6 ×4, 1/√3; angles 120°/90°/120°/120°/90°) + 4 congruent pentagons (the 3³.4² orbit; sides (3+√3)/6, 1, (3+√3)/6, 1/√3, 1/√3 — the unit side crossing two squares' shared edge; angles 120°/90°/90°/120°/120°), both areas 1/2+√3/4; Σ = 6+3√3 = \|det\|, edge-to-edge; p4g — axis mirrors pure (meeting at the corner-2 point), both diagonal cosets true half-period glides, 4-fold centers at ((√3−1)/4,(5+√3)/4) + partners with no mirror through any of them; tile/vertex/edge orbits 2 (8+4) / 4 / 5 (M121) |
| `kuniform-dual-n4` | dual of 2-uniform (4⁴; 3³.4²)₁ | cmm | face-centroid dual of `kuniform-n4` — same centered-rectangular lattice ⟨(1,0),(1/2,2+√3/2)⟩, det 2+√3/2, inherited; 3 faces/cell over the primal's 3 vertex orbits — 2 congruent pentagons (over the two 3³.4² vertex orbits; sides 1/√3 ×2, (3+√3)/6 ×2, 1 in cyclic order; area 1/2+√3/4) + 1 square (over the 4⁴ vertex orbit; side 1, area 1); Σ = 2+√3/2 = \|det\|, edge-to-edge; cmm — 2-folds at (0,1) and (1/2,1) on the mirror intersections plus off-mirror 2-folds at (1/4,2+√3/4) and (3/4,2+√3/4) (the cmm-vs-pmm discriminator), h-mirrors y = 1+m(2+√3/2), v-mirrors x = k/2, h-glides y = 2+√3/4+m(2+√3/2), v-glides x = 1/4,3/4 (M145) |
| `kuniform-dual-n3` | dual of 2-uniform (4⁴; 3³.4²)₂ | cmm | face-centroid dual of `kuniform-n3` — same centered-rectangular lattice ⟨(1,0),(1/2,3+√3/2)⟩, det 3+√3/2, inherited; 4 faces/cell over the primal's 4 vertex orbits — 2 congruent pentagons (over the two 3³.4² vertex orbits; sides 1/√3 ×2, (3+√3)/6 ×2, 1 in cyclic order; area 1/2+√3/4) + 2 squares (over the two 4⁴ vertex orbits; side 1, area 1); Σ = 3+√3/2 = \|det\|, edge-to-edge; cmm — 2-folds at (0,3/2) and (1/2,3/2) on the mirror intersections plus off-mirror 2-folds at (1/4,3+√3/4) and (3/4,3+√3/4) (the cmm-vs-pmm discriminator), h-mirrors y = 3/2+m(3+√3/2), v-mirrors x = k/2, h-glides y = 3+√3/4+m(3+√3/2), v-glides x = 1/4,3/4 (M145) |
| `kuniform-dual-n14` | dual of 2-uniform (3⁶; 3³.4²)₁ | pmm | face-centroid dual of `kuniform-n14` — same rectangular lattice ⟨(1,0),(0,1+√3)⟩, det 1+√3, inherited; 3 faces/cell over the primal's 3 vertex orbits — 2 congruent pentagons (over the two 3³.4² vertex orbits; sides 1/√3 ×2, (3+√3)/6 ×2, 1 in cyclic order; area 1/2+√3/4) + 1 regular hexagon (over the 3⁶ vertex orbit; side 1/√3, area √3/2); Σ = 1+√3 = \|det\|, edge-to-edge; pmm — 2-folds at (0,√3/2), (0,1/2+√3), (1/2,√3/2), (1/2,1/2+√3) all on the mirror intersections (no off-mirror 2-folds), h-mirrors y = √3/2+m(1+√3) and y = 1/2+√3+m(1+√3), v-mirrors x = k/2, no true glides (M147) |
| `kuniform-dual-n15` | dual of 2-uniform (3⁶; 3³.4²)₂ | cmm | face-centroid dual of `kuniform-n15` — same centered-rectangular lattice ⟨(1,0),(1/2,1+3√3/2)⟩, det 1+3√3/2, inherited; 4 faces/cell over the primal's 4 vertex orbits — 2 pentagons (over the two 3³.4² vertex orbits; sides 1/√3 ×2, (3+√3)/6 ×2, 1 in cyclic order; area 1/2+√3/4) + 2 regular hexagons (over the two 3⁶ vertex orbits; side 1/√3, area √3/2); Σ = 1+3√3/2 = \|det\|, edge-to-edge; cmm — 2-folds at (0,1/2) and (1/2,1/2) on the mirror intersections plus off-mirror 2-folds at (1/4,1+3√3/4) and (3/4,1+3√3/4) (the cmm-vs-pmm discriminator), h-mirrors y = 1/2+m(1+3√3/2), v-mirrors x = k/2, h-glides y = 1+3√3/4+m(1+3√3/2), v-glides x = 1/4,3/4 (M147) |

## Build arcs

Numbered arcs keep their PATTERNS.MD §numbers — code comments and
briefs cite them as `§N`.

### 37. 2-uniform (3⁶; 3².4.3.4) ✅ (M87)

- **Motif:** first of the three-entry p6m batch of the closed
  2-uniform list (Krotenheerdt's 20 two-vertex-orbit tilings by
  regular polygons; Wikipedia `2-uniform n18`, t = 3 tile orbits,
  e = 3 edge orbits): squares and two triangle orbits, with the
  3⁶ vertices (six triangles around a point) as the lattice origin.
  The list's entries have no individual proper names, so the tiling
  is named by its vertex configurations (house style, as for
  `kuniform-p6m`/`kuniform-pmm`).
- **Group:** **p6m** (*632), determined numerically BEFORE the Rust
  port: R60..R300 about the origin plus all six mirrors through it
  verified on the full 49-cell window (battery check 5: 5/5
  rotations T; the global group test `m87x_sym3.py` — 361 window
  sites vs 49 reference sites, 735/735 matches, 0 color changes).
  A near-miss was caught and fixed along the way: an earlier probe
  built its "R60" with a 30° angle (`th = m·π/6` mislabeled as
  60°) and reported a false R60 failure — the tiling had been
  misdiagnosed as a twin of itself under R30. See the M87 report.
- **Lattice & cell:** 60° hexagonal basis with |b1| = |b2| =
  1+√3: b1 = (0, 1+√3), b2 = ((3+√3)/2, (1+√3)/2) — left-handed
  (|det| = 3+2√3 ≈ 6.4641, handled by the generator). The cell
  (parallelogram [0,b1]×[0,b2], offset 0) holds 11 tiles: 3 squares
  (seed 0, the largest class — no hexagon in this tiling) and 8
  equilateral triangles, 6 from the first triangle orbit (seed 1)
  and 2 from the second (seed 2). Area 3·1 + 8·(√3/4) = 3+2√3 =
  |det| exact in Q(√3). Note: the tile set is not contained in the
  unit parallelogram — the lattice generator's u-space culling pads
  the index ranges, as designed.
- **Seeds:** squares 0; first triangle orbit 1 (#ffff00, 6/cell);
  second 2 (#808000, 2/cell). The orbit→color mapping was
  established by vertex-matching the cell's triangles against the
  Wikipedia reference SVG (SVG offset solved: (−0.000135,
  −189.230455); tiles 0–5 yellow, 6–7 golden).
- **Edge structure:** **EDGE-TO-EDGE**: 834 interior edge pairs in
  the 7×7 (49-cell) window, 0 unpaired, 0 T-junctions (boundary
  walk closed), 96 boundary edges.
- **Vertex stars:** exactly two figures — 333333 (36 vertices, 1
  lattice orbit: the origin) and 33434 (260 vertices, 6 orbits per
  cell, reps (500,866), (500,1866), (1000,2732), (1366,1366),
  (1866,2232), (1866,3232) in u-space ×1000).
- **Coverage:** 140×140 fine grid (irrational-shifted) over the
  window: 19600 points, 9800 exactly-once in the safe interior, 336
  on tile boundaries, max count 2 — no gaps, no overlaps.
- **Rust:** `kuniform_n18()` in `kuniform.rs` (feature `kuniform`) —
  radical-expr table through `H = √3/2`, `S3 = 2H` (no decimal
  literals), CCW vertex order, first cell inlined. Standard trio +
  the two-orbit vertex-figure test (counts + orbit representatives
  pinned verbatim from the Python battery) + added to the
  `lattice_culling_matches_brute_force` list (default params and
  0.7-scale/30°-rotation).
- **Status:** landed (M87).
  73 tilings, 85 memberships, 542 tests at this step.


### 38. 2-uniform (3.4.6.4; 3².4.3.4) ✅ (M88)

- **Motif:** second entry of the p6m batch — Wikipedia `2-uniform
  n9`, t = 4, e = 4: hexagons ringed by six squares, the gaps
  filled by two triangle orbits. The origin is the **hexagon face
  center** (v0-relative (−1,0,0,−1)) — a genuine 6-fold rotation
  center of the tiling, not a vertex.
- **Group:** **p6m** (*632), verified numerically before the Rust
  port: R60..R300 about the hexagon center plus all six mirrors
  through it (`m87x_sym3.py` global test, 0 color changes);
  3-fold centers at the golden-triangle centroids (2 per cell —
  yellow centroids are NOT 3-folds), 2-fold centers at the
  triangle–triangle edge midpoints; square centers tested negative.
- **Lattice & cell:** 60° hexagonal basis with |b1| = |b2| =
  2+√3: b1 = (−(2+√3), 0), b2 = (1+√3/2, −(3/2+√3)) —
  left-handed (|det| = 6+7√3/2 ≈ 12.0622). The cell holds 15 tiles:
  1 hexagon (seed 0, the largest class), 6 squares (seed 1), and 8
  equilateral triangles, 6 from the first orbit (seed 2, #ffff00)
  and 2 from the second (seed 3, #808000). Area 3√3/2 + 6·1 +
  8·(√3/4) = 6+7√3/2 = |det| exact in Q(√3).
- **Edge structure:** **EDGE-TO-EDGE**: 1255 interior edge pairs in
  the 7×7 window, 0 unpaired, 0 T-junctions (boundary walk closed),
  136 boundary edges.
- **Vertex stars:** exactly two figures — 33434 (234 vertices, 6
  orbits, reps (−2366,−1366), (−1866,−500), (−1366,−1366),
  (−500,−1866), (0,−2732), (500,−1866)) and 3464 (287 vertices, 6
  orbits, reps (−2732,0), (−1366,−2366), (−1000,0), (−500,−866),
  (500,−866), (1366,−2366)). Ray sets: 33434 {0,60,120,210,270}°,
  3464 {0,60,150,270}°.
- **Coverage:** 160×160 fine grid: 25600 points, 14256 exactly-once
  in the safe interior, 144 on tile boundaries, max count 2.
- **Rust:** `kuniform_n9()` in `kuniform.rs` (feature `kuniform`) —
  same conventions as M87 (radical exprs, CCW, first cell inlined);
  standard trio + two-orbit vertex-figure test + culling list.
  Negative orbit representatives are expected: reps live in
  [0,b1)×[0,b2), which extends to negative coordinates for the
  left-handed basis.
- **Status:** landed (M88).
  74 tilings, 86 memberships, 546 tests at this step.


### 39. 2-uniform (3.4.6.4; 3³.4²) ✅ (M89)

- **Motif:** third entry of the p6m batch — Wikipedia `2-uniform
  n8`, t = 4, e = 4: the **second** p6m form of the 3.4.6.4
  configuration. Structurally close to M88's n9 but distinct: the
  second vertex figure is 3³.4² (three triangles + two squares at
  one orbit), one square–square edge per cell, and the 2-fold
  centers sit at the **square–square** edge midpoints (n9's are at
  the triangle–triangle midpoints). Origin: the hexagon face
  center, a genuine 6-fold center as in M88.
- **Group:** **p6m** (*632), verified numerically before the Rust
  port: R60..R300 about the hexagon center plus all six mirrors
  through it (`m87x_sym3.py`, 0 color changes); 3-fold centers at
  the golden-triangle centroids (2 per cell), 2-fold centers at the
  square–square edge midpoints; square centers tested negative.
- **Lattice & cell:** 60° hexagonal basis with |b1| = |b2| =
  2+√3: b1 = (0, 2+√3), b2 = (−(3/2+√3), −(1+√3/2)) —
  left-handed (|det| = 6+7√3/2 ≈ 12.0622, same cell area as M88).
  The cell holds 15 tiles: 1 hexagon (seed 0), 6 squares (seed 1),
  8 equilateral triangles — 6 from the first orbit (seed 2,
  #ffff00) and 2 from the second (seed 3, #808000). Area
  3√3/2 + 6·1 + 8·(√3/4) = 6+7√3/2 = |det| exact in Q(√3).
- **Edge structure:** **EDGE-TO-EDGE**: 1261 interior edge pairs in
  the 7×7 window, 0 unpaired, 0 T-junctions (boundary walk closed),
  124 boundary edges; exactly one square–square edge per cell.
- **Vertex stars:** exactly two figures — 33344 (260 vertices, 6
  orbits, reps (−2732,0), (−1866,−500), (−1866,500), (−1366,1366),
  (−1366,2366), (−500,1866)) and 3464 (267 vertices, 6 orbits,
  reps (−2732,−1000), (−2732,1000), (−2232,1866), (−1000,0),
  (−500,866), (−500,2866)). Ray set: 33344 {30,120,210,270,330}°.
- **Coverage:** 160×160 fine grid: 25600 points, 14256 exactly-once
  in the safe interior, 168 on tile boundaries, max count 2.
- **Rust:** `kuniform_n8()` in `kuniform.rs` (feature `kuniform`) —
  same conventions as M87/M88; standard trio + two-orbit
  vertex-figure test + culling list.
- **Status:** landed (M89).
  75 tilings, 87 memberships, 550 tests at this step.


### 40. 2-uniform (3.4.6.4; 3.4².6) ✅ (M90)

- **Motif:** fourth entry of the p6m batch — Wikipedia `2-uniform
  n5`, t = 5 tile orbits, e = 5 edge orbits: hexagons ringed by
  squares, the gaps closed by a triangle orbit; two hexagon orbits
  and two square orbits (five tile orbits in all). Origin: a
  hexagon face center.
- **Group:** **p6m** (*632), determined numerically BEFORE the Rust
  port: R60..R300 about the 6-fold center c plus all six mirrors
  through it verified on the 49-cell window (battery check 5,
  `m90x_sym_n5.json`, 0 color changes). c is the center of the
  cell's lone #00ff00 hexagon — O-frame (4.732051, 5.464102), the
  exact O-relative tuple (6,2,4,4) — so c ≠ O (O itself is a
  hexagon center).
- **Lattice & cell:** 60° hexagonal basis with |b1| = |b2| =
  3+√3: b1 = (3+√3, 0), b2 = (−(3+√3)/2, 3(1+√3)/2) — right-handed
  (|det| = 9+6√3 ≈ 19.3923; the first right-handed basis of the
  batch, M87–M89 used left-handed ones). The cell holds 18 tiles:
  3 hexagons (2 seed 0 #008000 — the largest class, 1 seed 1
  #00ff00), 9 squares (6 seed 2 #ff0000, 3 seed 3 #ff8989), and
  6 equilateral triangles (seed 4 #ffff00). Area 3·(3√3/2) + 9·1 +
  6·(√3/4) = 9+6√3 = |det| exact in Q(√3).
- **Edge structure:** **EDGE-TO-EDGE**: 1681 interior edge pairs in
  the 7×7 (49-cell) window, 0 unpaired, 0 T-junctions (boundary
  walk closed), 166 boundary edges.
- **Vertex stars:** three figure strings over exactly two vertex
  orbits — the 3.4².6 orbit is **CHIRAL**: its figure "3446" (266
  vertices, 6 orbits) differs from the mirror orbit's "3644" (267,
  6 orbits; 100% bidirectional mirror pairing), while the 3.4.6.4
  orbit "3464" (267, 6 orbits) is achiral. Orbit reps pinned in
  the Rust test (×1000 of u-space): 3446 (−1866,3232), (−500,1866),
  (−500,3598), (500,866), (1000,2732), (3732,0); 3464 (1500,866),
  (1500,1866), (2366,366), (2366,2366), (3232,866), (3232,1866);
  3644 (−1000,2732), (−500,866), (500,1866), (500,3598), (1000,0),
  (1866,3232).
- **Coverage:** 160×160 fine grid: 25600 points, 18492 exactly-once
  in the safe interior (the maximal fully-covered rectangle
  containing the block centre, with a ≥4-unit guard), 0 on tile
  boundaries, max count 1.
- **Rust:** `kuniform_n5()` in `kuniform.rs` (feature `kuniform`) —
  radical-expr table through `H = √3/2` (no decimal literals), CCW
  vertex order, first cell inlined; standard trio + the
  three-figure vertex test (counts + orbit reps pinned verbatim
  from the Python battery) + added to the
  `lattice_culling_matches_brute_force` list.
- **Status:** landed (M90).
  76 tilings, 88 memberships, 554 tests at this step.


### 41. 2-uniform (4.6.12; 3.4.6.4) ✅ (M91)

- **Motif:** fifth entry of the p6m batch — Wikipedia `2-uniform
  n1`, t = 4, e = 4: the 3-4-6-12 tiling — the first in the
  module to carry **dodecagons**: dodecagons alternating with
  hexagons, squares and triangles. Origin: a hexagon face center.
- **Group:** **p6m** (*632), determined numerically before the
  port: R60..R300 about c plus the six mirrors through it
  (`m90x_sym_n1.json`, 0 color changes). c is the center of the
  cell's dodecagon — O-frame (4.098076, 2.366025), the exact
  O-relative tuple (3,3,3,1) — so c ≠ O (O is a hexagon center).
- **Lattice & cell:** 60° hexagonal basis with |b1| = |b2| =
  2+2√3: b1 = (2+2√3, 0), b2 = (1+√3/2, 3+√3/2) — right-handed
  (|det| = 12+8√3 ≈ 25.8564, the largest cell of the batch). The
  cell holds 12 tiles: 1 dodecagon (seed 0, the largest class),
  3 hexagons (seed 1), 6 squares (seed 2), 2 equilateral triangles
  (seed 3). Area (6+3√3) + 3·(3√3/2) + 6·1 + 2·(√3/4) = 12+8√3 =
  |det| exact in Q(√3).
- **Edge structure:** **EDGE-TO-EDGE**: 1386 interior edge pairs in
  the 7×7 window, 0 unpaired, 0 T-junctions (boundary walk closed),
  168 boundary edges.
- **Vertex stars:** three figure strings over two vertex orbits —
  the 4.6.12 orbit is CHIRAL: "4612" (266, 6 orbits) differs from
  the mirror orbit's "4126" (266, 6 orbits; 100% bidirectional
  pairing; ring-concatenation labels "1246"/"1264"), while "3464"
  (267, 6 orbits) is achiral. The pinned strings are the
  canonical lex-min integer rotations of the figure's side-length
  list — the battery's original check-3 canonicalized by
  string-rotation instead, which for 12-gon figures gives the ring
  labels, not the pinned keys (the canonization fix; see the M90
  report). Orbit reps pinned in the Rust test (×1000 of u-space):
  3464 (866,500), (1366,1366), (1866,500), (6330,4232),
  (6830,3366), (7330,4232); 4126 (2232,1866), (2732,3732),
  (3598,500), (4598,4232), (5464,1000), (5964,2866); 4612
  (2232,2866), (2732,1000), (3598,4232), (4598,500), (5464,3732),
  (5964,1866).
- **Coverage:** 160×160 fine grid: 25600 points, 19044 exactly-once
  in the safe interior, 0 on tile boundaries, max count 1.
- **Rust:** `kuniform_n1()` in `kuniform.rs` (feature `kuniform`) —
  same conventions as M87–M90; the dodecagon is emitted as a
  12-gon through the same `H`-radical table; standard trio + the
  three-figure vertex test + culling list.
- **Status:** landed (M91).
  77 tilings, 89 memberships, 558 tests at this step.


### 42. 2-uniform (3⁶; 3².4.12) ✅ (M92)

- **Motif:** sixth entry of the p6m batch — Wikipedia `2-uniform
  n13`, t = 4, e = 4: a dodecagon ringed by squares and two
  triangle orbits around the 3⁶ vertices. Origin: the 3⁶ vertex
  itself.
- **Group:** **p6m** (*632), determined numerically before the
  port: R60..R300 about c plus the six mirrors through it
  (`m90x_sym_n13.json`, 0 color changes). c is the center of the
  cell's dodecagon — O-frame (6.830127, 11.830127), which is
  OFF-RING: the dodecagon-center radius (√6+√2)/2 ∉ Q(√3), so c
  exists only as a float relative to O (O is a 3⁶ vertex; c ≠ O).
- **Lattice & cell:** 60° hexagonal basis with |b1| = |b2| =
  3+√3: b1 = ((3+3√3)/2, (3+√3)/2), b2 = (0, 3+√3) — right-handed
  (|det| = 9+6√3 ≈ 19.3923, same cell area as M90's n5). The cell
  holds 16 tiles: 1 dodecagon (seed 0, the largest class), 3
  squares (seed 1), and 12 equilateral triangles — 6 from the
  first triangle orbit (seed 2, #808000) and 6 from the second
  (seed 3, #ffff00; the two orbits tie at 6/6 per cell, so the
  seed order is the documented first-appearance order in the cell).
  Area (6+3√3) + 3·1 + 12·(√3/4) = 9+6√3 = |det| exact in Q(√3).
- **Edge structure:** **EDGE-TO-EDGE**: 1346 interior edge pairs in
  the 7×7 window, 0 unpaired, 0 T-junctions (boundary walk closed),
  248 boundary edges.
- **Vertex stars:** three figure strings over two vertex orbits —
  the 3².4.12 orbit is CHIRAL: "33412" (241, 6 orbits) differs
  from the mirror orbit's "33124" (240, 6 orbits; 100%
  bidirectional pairing; ring labels "12334"/"12433"), while the
  3⁶ orbit "333333" (84, 2 orbit reps — the 6-fold vertices) is
  achiral. Orbit reps pinned in the Rust test (×1000 of u-space):
  33124 (0,3732), (500,1866), (866,500), (1366,3366),
  (2232,1866), (3232,2866); 333333 (0,0), (1366,2366); 33412
  (0,1000), (500,2866), (866,4232), (1366,1366), (2232,2866),
  (3232,1866).
- **Coverage:** the 28800-point fine grid over the jagged safe
  region (maximal fully-covered rectangle containing the block
  centre; the 49-cell union's jagged boundary bites 0.125–0.675
  units past the 1-unit inset, with a ≥4-unit guard): 16674
  exactly-once, 326 on tile boundaries, max count 2 — grid lines
  coincide with tile edges in this tiling, so the on-boundary
  points are expected, not a defect.
- **Rust:** `kuniform_n13()` in `kuniform.rs` (feature `kuniform`) —
  same conventions; standard trio + the three-figure vertex test
  + culling list.
- **Status:** landed (M92).
  78 tilings, 90 memberships, 562 tests at this step.


### 46. 2-uniform (3⁶; 3⁴.6) ✅ (M96)

- **Motif:** first chiral (p6) entry of the two-uniform module —
  Wikipedia `2-uniform n20`, t = 5: the flower construction — the
  unit triangular lattice with the 6-face flower at each point of
  an index-13 sublattice L13 = ⟨B1 = (1,3), B2 = (−3,4)⟩ dissected
  into a regular unit hexagon, every other triangle intact. Origin:
  a hexagon center = the 6-fold point.
- **Group:** **p6** (632), pinned by exact + numeric probes
  (m96b_c13.py): R60..R300 about the hexagon center PASS 5791/5791
  tiles on the radius-46 window; ALL six mirror probes through the
  hexagon center FAIL (4273/5791 = 73.8% — no mirror exists), and
  mirror/rotation probes through every A- and B-class point fail
  too; the translation scan finds exactly the six shortest vectors
  of L13 (norm² 13) and their multiples — the translation group is
  exactly L13.
- **Lattice & cell:** 60° hexagonal basis with |b1| = |b2| = √13:
  b1 = (5/2, 3H), b2 = (−1, 4H) — right-handed, |det| = 13√3/2 =
  13H = 26·(√3/4) exactly. Cell: 1 hexagon + 20 triangles (the 26
  lattice faces per cell minus the 6-flower); torus census V = 12,
  E = 33, F = 21, V−E+F = 0 (m96b_torus.py, exact integers).
  Hard-core: min center-center norm² = 13 ≥ 7, flowers are vertex-
  disjoint. Area 6·(√3/4)·… exact: hexagon 6 + triangles 20 units
  of √3/4 = 13H = |det|.
- **2-uniformity (exact modular arithmetic):** the class map
  k(v) = 4X+3Y mod 13 has kernel exactly L13; R60 acts as k → 4k;
  the hexagon-corner classes {1,3,4,9,10,12} form ONE orbit (the
  3⁴.6 vertices, B) and the remaining classes {2,5,6,7,8,11} form
  ONE orbit (the 3⁶ vertices, A) — exactly two vertex orbits with
  A:B = 6:6 = 1:1 per cell.
- **Orbits:** triangles in 4 orbits of sizes {6,6,6,2} (seeds 1–4:
  the six hexagon-edge triangles; two corner-triangle orbits; the
  two far triangles centered on 3-fold points) plus the hexagon =
  t = 5 (matches the wiki); edge orbits {6,6,6,6,6,3} → **e = 6**,
  not the wiki parenthetical's 7 (that annotation is unreliable —
  it also lists e = 1 for the snub hexagonal, impossible since
  triangle–triangle and triangle–hexagon edges cannot share an
  orbit). The wiki figure itself confirms this tiling: its four
  triangle colors reproduce {6,6,6,2} (261/259/86 colored =
  3:3:1), and 31/38 hexagons carry the complete 6-edge+12-corner
  triangle ring (the 7 partials are exactly the outermost rim).
- **Uniqueness:** an edge-to-edge {3⁶, 3⁴.6} tiling is exactly a
  flower construction (a unit hexagon with lattice-aligned edges
  must center on a lattice point; lattice transport around flowers
  has no holonomy), 2-uniform with A:B = 1:1 forces hexagon density
  1/13, and the extracted figure's hexagon-center differences are
  all ≡ 0 mod 13 — so C is a single L13 coset: this tiling. Among
  flower p6 candidates the 6 corner classes forming one R60 orbit
  requires ord_d(4) = 6, first possible at d = 13.
- **Edge structure & coverage (m96b_battery.py, on the emitted table):**
  edge-to-edge over the ±3×±3 patch: 1534 interior edge pairs, 0
  unpaired, 0 T-junctions; the 166 boundary single edges close into
  loops (0 odd-degree vertices). Fine grid of 1600 irrational-offset
  points over the safe dual rectangle [−2.4,3.4]² (the 49-cell union's
  jagged boundary bites ≤ ~0.35 dual units past the parallelogram, so
  the inset rectangle is fully inside): 1600 exactly-once, max count 1.
- **Rust:** `kuniform_n20()` in `kuniform.rs` (feature `kuniform`)
  — standard trio + vertex-figure test pinning 333333 (266 interior,
  6 orbit reps) and 33336 (240, 6 orbit reps), expected values
  reproduced bit-exact by the Python replica m96b_rust_expect.py.
- **Status:** landed (M96).
  79 tilings, 91 memberships, 566 tests at this step (fork state;
  landed on main at 87 tilings, 99 memberships, 618 tests, after
  M94 + M98).


### 48. p6m 2-uniform duals — first batch: p6m, n18, n9, n8 ✅ (M98)

- **Motif:** the face-centroid duals of the first four p6m 2-uniform
  primals (`kuniform-p6m`, `kuniform-n18`, `kuniform-n9`,
  `kuniform-n8`) — the first k-uniform duals in the registry and the
  first landing of the "mixed floret" dual family proposed in Research
  area 2. Every dual is *constructed at runtime* by the new `dual()`
  helper from the registered primal table — no hand-transcribed dual
  geometry anywhere.
- **The `dual()` helper (research area 2 landed):**
  `dual(primal: &LatticeTiling) -> Option<LatticeTiling>` in
  `tess-tilings/src/dual.rs`. Centroid dual: one dual vertex at each
  primal face centroid (vertex average — exact since the primal faces
  are regular), one dual edge per primal edge, one dual face per
  primal vertex = the polygon of the surrounding face centroids sorted
  CCW by atan2 around the vertex. The dual **shares the primal
  lattice** (same basis, same LatticeKind, zero offset); each dual
  face is emitted by exactly one cell, and the cell count equals the
  number of distinct vertex orbits. Orbit bookkeeping: each primal
  vertex is grouped by its fractional lattice coordinate `frac(A⁻¹ p)`
  on a 1e6 grid (grouping only — never a coordinate source); the orbit
  representative is the *stored* table vertex (a grid-reconstructed
  representative drifted ~5e-7 and broke the 1e-9 oracle), and each
  incident cell contributes `v_rep + (centroid_f − p_f)`. `build()`
  fills the spec (Laves primary, `secondary: &[]`, "Dual of 2-uniform
  (…)" display) and `proper_seeds()` assigns congruence-aware
  proper-coloring seeds (base = congruence-class index by first
  appearance; a greedy bump keeps edge-adjacent dual faces differing
  in `seed % 8`). Returns `None` when the primal has no cells or a
  vertex orbit is not fully consistent across cells.
- **Oracle gate:** `dual()` applied to all seven Archimedean primals
  reproduces the seven hand-built Laves tables exactly — same lattice
  basis, same cell count, cells matching as a bijection up to lattice
  translation + polygon vertex rotation/reversal, max vertex residual
  0.0 (pairing found by lattice-translation search + DFS bipartite
  matching). Gated test `dual_reproduces_laves_tables`
  (`#[cfg(all(feature = "archimedean", feature = "laves"))]`).
- **The four duals (per cell, 6-fold center at the origin):**
  - `kuniform-dual-p6m` (det 9√3/2): 1 regular hexagon (side 1/√3,
    area √3/2) + 6 congruent quadrilaterals (sides 1/√3, 2√3/3,
    2√3/3, √3; angles 60°/60°/120°/120°; area 2√3/3); Σ = 9√3/2.
  - `kuniform-dual-n18` (det 3+2√3): 1 regular hexagon (side 1/√3)
    + 6 congruent pentagons (one side 1/√3, four sides (3+√3)/6;
    angles 90°/90°/120°/120°/120°; area (2+√3)/4); Σ = 3+2√3.
  - `kuniform-dual-n9` (det 6+7√3/2): 6 congruent quadrilaterals
    (two sides (3+√3)/6, two sides (1+√3)/2; angles 60°/90°/90°/
    120°; area (3+2√3)/6) + 6 pentagons congruent to the n18-dual
    pentagons (area (2+√3)/4); Σ = 6+7√3/2.
  - `kuniform-dual-n8` (det 6+7√3/2): 6 quadrilaterals (same shape as
    the n9-dual quadrilaterals) + 6 congruent pentagons (two sides
    1/√3, two sides (3+√3)/6, one side 1; angles 90°/90°/120°/
    120°/120°; area (2+√3)/4 — equal area, different shape than the
    n18/n9-dual pentagons); Σ = 6+7√3/2.
- **Group:** all four **p6m**, determined numerically (house
  `probe_symmetry` replica + lattice-preservation check): R60/R120/
  R180/R240/R300 about the origin and the six mirrors through it all
  hold on the generated patch; R30 fails. The 6-fold center is the
  origin in each case (p6m: fan apex; n18: 3⁶ vertex; n9/n8:
  hexagon center).
- **Verification (19 new tests, all in `dual.rs`):** convex + CCW
  per face (both face orbits checked separately, per-face areas vs
  exact Q(√3) values to 1e-9); edge-to-edge over a ±4 tile window
  (0 unpaired, 0 T-junctions); boundary walks closed; per-cell Σ face
  areas = |det| to 1e-9; fine-grid exactly-once coverage (0.25 grid,
  double cover allowed only within 1e-6 of a shared edge/vertex);
  proper coloring by 1e-6 edge-key multiset over `generate()`; p6m
  group + R30 negative; orbit correspondence (BTreeSet of primal
  orbit keys over all stored vertices == dual cell count, and
  per-orbit incident-tile recount == dual face degree); the
  coverage/rotation/|det| trio per dual. Seed maxima observed: p6m
  {0,1,2}, n18 {0..3}, n9 3, n8 4 (all < 8).
- **Category placement:** Laves **primary only**, `secondary: &[]` —
  these are 2-uniform duals, not entries of the Two-Uniform arc
  (deviation from the brief's "Two-Uniform secondary" prose; the
  brief's own census numbers are self-consistent only without it).
- **Rust:** `dual.rs` (helper + four constructors
  `kuniform_dual_p6m/n18/n9/n8()`, feature `kuniform`), registry +4
  (after the k-uniform block, before laves), culling list +4, census
  79 → 83 tilings / 91 → 95 memberships (Laves 11 → 15 = 12 primary +
  3 self-dual secondaries; Two-Uniform unchanged at 8), tess-tui tab
  pins updated to the same counts.
- **Status:** landed (M98). 83 tilings, 95 memberships, 590 tests at
  this step (fork state; landed on main at 86 tilings, 98 memberships,
  614 tests, after M94). Full bar green: cargo
  check + clippy (both target sets) with zero warnings, cargo test
  --workspace 590/590. First batch of the k-uniform dual family —
  the remaining p6m duals (n5, n1, n13) and the pmm duals queue
  behind the same helper.
<!-- §43–§45 are reserved for the concurrent M93–M95 two-uniform
     entries; §46 below is the pre-assigned M96 slot. -->


### 49. 2-uniform (3².6²; 3⁴.6) ✅ (M97)

- **Motif:** first cmm entry of the two-uniform module —
  Wikipedia `2-uniform n12`, t = 2: rows of edge-sharing regular
  hexagons (consecutive centers √3 apart — a full shared edge) with
  the bands between adjacent rows filled by unit triangles; odd
  rows offset by half a period (the 2-hexagon + 8-triangle centered
  cell of the M97 spike). Origin: a hexagon center = a 2-fold point.
- **Group:** **cmm**, pinned by tile-set probes on the EMITTED Rust
  table (m97_rust_expect.py): 8 translation probes PASS (±b1, ±c —
  the centering vector c = (√3/2, 5/2) IS a basis vector, and the
  centering rectangle's b2 = −b1 + 2c); the 2-fold probes PASS at
  origin, b1/2, c, c+b1/2, c+b2/2, c+(b1+b2)/2 (the two translation
  classes of 2-fold centers, each hit at three named points), while
  2-folds at the valley points (√3/2, 1/2) and (0, 1/2) FAIL
  (245/245 tiles off); mirrors of BOTH families x = √3·k/2 and
  y = 5·k/2 PASS (6 probes); R60/R90/R120 about every mirror
  intersection FAIL → no 3-, 4- or 6-fold anywhere.
- **Lattice & cell:** centered-rectangular lattice
  Λ = ⟨b1 = (√3, 0), c = (√3/2, 5/2)⟩ declared on the PRIMITIVE cell
  (the running-bond / stepped-hexominoes cmm convention; the
  rectangular cell ⟨b1, (0,5)⟩ of the spike is the centered one,
  and the measurement's witness orthogonal pair (√3,0), (0,5) has
  cross 5√3 = 2·|det| with cell center c ∈ Λ). det = 5√3/2
  (right-handed). Cell: 1 hexagon + 4 triangles — areas 3√3/2 +
  4·√3/4 = 5√3/2 = |det| exactly; the spike's 10-tile centered cell
  is S ⊎ (S + c) mod ℤb1 + ℤb2 tile-by-tile (verified). Torus
  census V = 4, E = 9, F = 5, V−E+F = 0.
- **2-uniformity:** exactly two vertex orbits — 3².6² (the hex–hex
  junctions along shared hexagon edges: 2 hexagons + 2 triangles)
  and 3⁴.6 (four triangles + one hexagon corner) — two translation
  classes each per primitive cell (4 + 4 reps per centered cell,
  matching the spike's pinned (0,2),(0,8),(1,3),(1,7) and (0,4),
  (0,6),(1,1),(1,9) in the half-unit frame), A:B = 1:1.
- **Orbits:** hexagons one orbit; ALL triangles one orbit — the
  2-fold rotation at the origin swaps the two visual triangle pairs
  modulo Λ and the mirrors pair them within a band — so **t = 2**;
  edge orbits **e = 4**, split by adjacency and corner classes:
  hex–hex [3².6²|3².6²], hex–tri [3⁴.6|3².6²], tri–tri
  [3⁴.6|3⁴.6], tri–tri [3².6²|3⁴.6] (patch orbit sizes
  {365,196,182,84}). Both match the wiki parenthetical (2,4).
- **Edge structure & coverage (m97_rust_expect.py, emitted
  table):** ±3×±3 patch of Λ: area exact (49·12 + 196·2 = 980
  half-unit units = 49·|det|); 406 interior edge pairs, 70 boundary
  single edges (0 interior unpaired), 0 triple edges, 0 T-junctions;
  fine grid of 8000 irrational-offset points in the safe interior
  box: exactly-once 1.0000, zero 0, two 0. The original centered-
  cell battery (m95_n12_battery.py, cells [-3,3]², 490 tiles) gives
  the cross-check: 937 unique edges, 110 unpaired (0 interior),
  0 triple, 0 T-junctions, coverage exactly-once 1.0000.
- **Rust:** `kuniform_n12()` in `kuniform.rs` (feature `kuniform`)
  — standard trio + vertex-figure test pinning 3366 (78 interior,
  reps (866,500), (1732,2000)) and 33336 (84, reps (866,1500),
  (1732,1000)), expected values reproduced bit-exact by the Python
  replica m97_rust_expect.py. All five cell polygons CCW (the
  spike's dump had mixed triangle orientations; each polygon
  normalized by signed area so the figure walk closes).
- **Status:** landed (M97). 88 tilings, 100 memberships, 622 tests
  at this step.


### 50. 2-uniform [3⁶; 3⁴.6]₁ (n19) ✅ (M95)

- **Motif:** the t = 3 member of the two-vertex-orbit 3⁴.6 pair —
  Wikipedia `2-uniform n19`, `[3⁶; 3⁴.6]₁ (t=3, e=3)`, the row the
  wiki table annotates p6. The construction is the flower form on the
  index-9 sublattice: the unit triangular lattice Λ with the 6-face
  flower at each point of L9 = ⟨B1 = (3,0), B2 = (0,3)⟩ (lattice
  coordinates; index 9, min center-center norm² 9) dissected into a
  regular unit hexagon, all other unit triangles intact. Flowers never
  touch (center spacing 3 vs. the norm² ≥ 7 hard-core bound); along a
  center line neighbors leave a one-unit gap spanned by two unit
  triangles. 13 tiles/cell (1 hex + 6 petal triangles + 6 gap
  triangles), 60° hexagonal basis |b1| = |b2| = 3, |det| = 9√3/2
  (right-handed), origin a hexagon center = 6-fold point; torus census
  V = 8, E = 21, F = 13, V − E + F = 0.
- **Exact solve (m95x_torus.py, integer arithmetic throughout):**
  2-uniformity pinned by v mod 3 (kernel L9) — the 3⁶ orbit is the
  petal-tip classes {(1,1),(2,2)} (one R60 2-cycle; tips at distance
  √3 from the flower centers, 3-fold points), the 3⁴.6 orbit the six
  hexagon-corner classes (one R60 6-cycle); A:B = 2:6 = 1:3 per cell.
  Triangle orbits {6,6} (petals, gap triangles) + the hexagon → t = 3.
  Cross-check: the coordination sequences match Galebach's #19 (= RCSR
  layer net krb) 12/12 terms — B (deg 5) [1,5,10,15,22,27,31,38,43,
  47,54,59] = A301722, A (deg 6) [1,6,9,18,21,24,36,36,39,54,51,54] =
  A301720 (m95x_coordseq.py).
- **The group is p6m, not the wiki's p6:** both mirror classes pass
  exactly — the x-axis mirror F: (X,Y)→(X+Y,−Y) and the 30° mirror
  M30: (X,Y)→(Y,X) (lattice coordinates) normalize L9 and carry every
  one of the 13 table tiles to a table position mod L9 (torus-exact);
  under the full point group D6 (12 elements, distinctness-checked on
  the torus) the vertex orbits stay {2,6}, the triangle orbits {6,6},
  and the 21 edges fall as {6, 12, 3} (hexagon sides; the 12 petal
  edges — the mirror merges the two C6 petal-edge orbits of 6, which
  is why the rotation subgroup alone would give e = 4; the 3 Q–Q
  edges) → **e = 3**, matching the wiki parenthetical. The wiki table
  lists p6 for *both* 3⁴.6 variants; only n20 (t = 5) is genuinely
  chiral (all six of its mirror probes fail). For n19 the p6 label
  under-states the symmetry — same annotation-reliability class as
  n20's e = 7 (§46).
- **±3×±3 window battery (m95x_battery.py, exact vertex-set
  matching):** edge-to-edge pairing over the 7×7-cell patch — 941
  interior edges pair-perfect, 176 rim singles (0 odd boundary
  vertices, each re-pairs in the ±4 patch), 0 edges with >2 sides, 0
  T-junctions; fine grid 1600/1600 exactly-once (max coverage 1);
  vertex figures exact per class (A: 6 triangles = 3⁶, B: 4 triangles
  + 1 hexagon = 3⁴.6); R60–R300 PASS 181/181 (radius-6 window);
  mirrors at axes 0°/30°/60°/90° PASS 181/181, mid-axes 15°/45°/75°
  FAIL 0/181; translation scan over all 64 Λ candidates with norm² ≤
  20 (both directions tested) finds exactly ±(3,0), ±(0,3), ±(3,−3)
  (lattice coords) — the translation lattice is exactly 3Λ, min
  period norm² 9.
- **Rust:** `kuniform_n19()` in `kuniform.rs` (module header
  "Ten entries" → "Eleven entries"), registered after
  `kuniform_n12` (the n12 fork landed first; both appended after
  `kuniform_n20` on their forks); 4 tests
  (`covers_region`, `rotated_scaled` 0.7/30°, `deterministic`,
  `vertex_figures`). The vertex-figure test pins the ±3×±3 patch
  census from the bit-exact Python replica `m95x_rust_expect.py`:
  figure `333333` count 72 with orbit reps {(1500,866),(3000,1732)}
  — the two petal-tip classes are exactly the two L9-translation
  orbits — and figure `33336` count 241 with reps {(500,866),
  (1000,0),(1000,1732),(2000,0),(2000,1732),(2500,866)} (one per
  hexagon-corner class). Census (cumulative on main, after M97):
  88 → 89 tilings, 100 → 101 memberships, TwoUniform 10 → 11.
- **Status:** landed (M95). 88 tilings, 100 memberships, 622 tests
  at this step on the worker fork (landed on main at 89 tilings,
  101 memberships, 626 tests, after M97). Full bar green. 11 of
  the 20 closed-list entries landed, 9 remain.


### 59. K-uniform duals batch 2 (n5/n1/n13/n20) ✅ (M102)

- **Motif:** the face-centroid duals of the second k-uniform batch —
  `kuniform-n5`, `kuniform-n1`, `kuniform-n13`, `kuniform-n20` —
  constructed at runtime by the same `dual()` helper that produced the
  M98 batch; no hand-transcribed dual geometry. Batch 2 closes the
  p6-family duals queued in M98's outlook and lands the family's
  first chiral (p6) dual. *Provisional number — §48's numbering is
  already out of sequence (a duplicate §56), and this fork's M-number
  will be renumbered at integration.*
- **The four duals (per cell):**
  - `kuniform-dual-n5` (det 9+6√3): 18 faces — 6 congruent
    quadrilaterals (the 3.4.6.4 orbit; sides (3+√3)/6, (1+√3)/2,
    (1+√3)/2, (3+√3)/6 in cyclic order; angles 120°/90°/60°/90°;
    area (3+2√3)/6) + 12 congruent quadrilaterals (the 3.4².6 orbit;
    sides 2/√3, (1+√3)/2, 1, (3+√3)/6; angles 120°/60°/90°/90°;
    same area, different shape). Two quadrilateral classes: the
    shared census test now matches a class by (vertex count, count in
    cell) — unique for all 8 duals. Σ = 9+6√3.
  - `kuniform-dual-n1` (det 12+8√3): 18 faces — 12 congruent
    30°/60°/90° triangles (the 4.6.12 orbit; sides 1+√3, (3+√3)/2,
    (1+√3)/2 in cyclic order; area (3+2√3)/4) + 6 congruent
    quadrilaterals (the 3.4.6.4 orbit; sides (1+√3)/2, (3+√3)/6,
    (3+√3)/6, (1+√3)/2; angles 60°/90°/120°/90°; area (3+2√3)/6).
    Σ = 12+8√3.
  - `kuniform-dual-n13` (det 9+6√3): 14 faces — 2 regular hexagons
    (the 3⁶ orbit; side 1/√3, area √3/2) + 12 congruent
    quadrilaterals (the 3².4.12 orbit; sides 1/√3, (3+√3)/6,
    (3+√3)/2, 1+2/√3 in cyclic order; angles 120°/120°/90°/30°;
    area (9+5√3)/12). Σ = 9+6√3.
  - `kuniform-dual-n20` (det 13√3/2): 12 faces — 6 regular hexagons
    (the 3⁶ orbit; side 1/√3, area √3/2) + 6 congruent pentagons
    (the 3⁴.6 orbit; sides 1/√3 ×3, 2/√3 ×2 in cyclic order; angles
    60°/120°/120°/120°/120°; area 7√3/12). Σ = 13√3/2.
- **Group:** measured, not assumed. The M98 duals and
  `kuniform-dual-n20` carry their 6-fold center at the origin, but
  n5/n1/n13 do not (the origin is only a 3-fold/2-fold center of those
  tables — R60 about the origin fails), so the symmetry test was
  generalized: probes are now centered on a per-case `center`
  (rotations about it, mirrors through lines through it), and the
  lattice-preservation check applies to the map's linear part
  `g(x) − g(0)`. Measured groups: n5/n1/n13 **p6m** with 6-fold
  centers at (3+√3, 2+2√3) / (3(1+√3)/2, (3+√3)/2) /
  (5(1+√3)/2, 5(3+√3)/2) respectively — R60–R300 about the center and
  all six mirrors through it hold, R30 fails; `kuniform-dual-n20`
  **p6** — R60–R300 about the origin hold, all six mirrors through it
  fail (chiral, matching the primal's declared group), R30 fails.
  Pinned by `duals_match_measured_groups` (renamed from
  `duals_are_p6m_about_the_origin`; the `Case` table gained `group`
  and `center` fields).
- **Verification (12 new tests):** the per-dual
  covers_region/rotated_scaled/deterministic trio (house pattern),
  plus the shared battery now covering all 8 duals: convex + CCW per
  face with per-class per-face areas vs. exact Q(√3) values to 1e-9
  (class lookup by vertex count AND count in cell), edge-to-edge over
  a ±4-tile window (0 unpaired, 0 T-junctions), exactly-once fine-grid
  coverage, proper coloring, orbit correspondence, and the measured-
  group symmetry probes above.
- **Category placement:** Laves primary only, `secondary: &[]`, as in
  the M98 batch (2-uniform duals, not Two-Uniform arc entries).
- **Rust:** `dual.rs` +4 constructors
  (`kuniform_dual_n5/n1/n13/n20()`, feature `kuniform`), registry +4
  (second `#[cfg(feature = "kuniform")]` block, after
  `kuniform-dual-n8`), culling list +4, census 96 → 100 tilings /
  108 → 112 memberships (Laves 15 → 19 = 16 primary + 3 self-dual
  secondaries), tess-tui tab pins updated to the same counts.
- **Status:** landed (M102, fork-local numbering — renumbered at
  integration). 100 tilings, 112 memberships, 686 tests. Full bar
  green: cargo check + clippy (both target sets) zero warnings, cargo
  test --workspace 686/686. Remaining k-uniform duals after this
  batch: pmm and n12; the square-family rows (n19 dual, etc.) stay
  blocked on unimplemented primals.


### 60. 2-uniform [4⁴; 3³.4²]₁ (n4) + [3⁶; 3³.4²]₁ (n14) ✅ (M103)

- **Numbering note:** renumbered from the fork's provisional §61 to
  §60 by landing order (§59 = k-uniform duals batch 2, M102).
- **Motif:** two entries from Krotenheerdt's closed list of 20
  two-vertex-orbit tilings: Wikipedia `2-uniform n4`,
  `[4⁴; 3³.4²]₁ (t=2, e=4)`, annotated cmm, and Wikipedia
  `2-uniform n14`, `[3⁶; 3³.4²]₁ (t=3, e=4)`, annotated pmm. Label
  caveat: the sprint brief carried stale row labels for these indices;
  re-fetching the current wiki table and mapping the SVG filenames to
  their caption cells gives `2-uniform_n4.svg` ↔ `[44; 33.42]1(t=2,
  e=4)` and `2-uniform_n14.svg` ↔ `[36; 33.42]1(t=3, e=4)` — and both
  independent torus enumerations below reproduce exactly those t/e,
  confirming the corrected labels. n4 construction: horizontal square
  bands two unit squares tall, alternating with one-row bands of
  alternating up/down unit triangles (height H = √3/2); consecutive
  square bands staggered by half a unit (the b2 shift), triangle-band
  vertices join both square rows edge-to-edge. Basis b1 = (1,0),
  b2 = (1/2, 2+√3/2), |det| = 2+√3/2; cell = 2 squares + 2 triangles.
  n14 construction: rows of edge-sharing unit squares alternating
  with double triangle strips — two counterposed unit-triangle rows
  joined tip-to-base along the strip midline (total strip height √3),
  square row sitting on the strip top line. Primitive basis
  b1 = (1,0), b2 = (0, 1+√3), |det| = 1+√3; cell = 1 square +
  4 triangles.
- **Exact solve (tku3_core.py / tku3_run.py, Q(√3) surd-pair
  arithmetic):** n4 — area identity |det| = Σ areas exact; ±6 patch
  edge census 1235 distinct edges, 104 rim singles, 0 edges with >2
  sides; coverage 1.0; interior figures 33344: 300, 4444: 156; full
  symmetry scan passes only R180 (52 centers), mirror0 (16 axes),
  mirror90 (156 axes); translation scan (radius 3.2) finds exactly
  ⟨b1, b2⟩ — the cmm signature. Torus 2×2-cell quotient with the
  68 deduped generator classes: F = 4, V = 3 per cell; tile orbits
  t = 2 (squares 2/cell + triangles 2/cell); edge orbits e = 4
  (sizes 1+2+2+2 = 7); vertex orbits 4444 (1/cell — the four-square
  crossing inside a band) and 33344 (2/cell — band junctions), both
  single-figure → 2-uniform. n14 — same battery: area identity exact;
  1404 edges, 104 rim singles, 0 triple; coverage 1.0; figures
  333333: 156, 33344: 300; scan passes only R180 (56), mirror0 (56),
  mirror90 (164) — pmm; translation scan finds exactly
  ⟨(1,0), (0,1+√3)⟩ (the earlier candidate's 3-periodic square row
  is disproved: (1,0) is a period, so the PRIMITIVE lattice is the
  correct one). Torus: F = 5, V = 3/cell; tile orbits t = 3
  (square 1/cell + two triangle orbits {strip1-up, strip2-down} and
  {strip1-down, strip2-up} of 2/cell — the same pairing as the wiki
  figure's two triangle colors, level-census-verified against
  `wiki_n14.svg`: dark = up-based-at-square-top ∪ down-based-at-next-square-bottom,
  yellow = the complementary pair at the strip midline); edge orbits
  e = 4 (sizes 1+1+2+4 = 8); vertex orbits 3⁶ (1/cell, strip-midline
  junctions) and 3³.4² (2/cell, square-row corners) → 2-uniform.
  Both t/e match the wiki parentheticals (2,4) and (3,4) exactly.
- **Rust:** `kuniform_n4()` / `kuniform_n14()` in `kuniform.rs`
  (module header "Eleven entries" → "Thirteen entries"), registered
  after `kuniform_n19`; 4 tests each (`covers_region`,
  `rotated_scaled` 0.7/30°, `deterministic`, `vertex_figures`). The
  vertex-figure tests pin the ±3×±3 patch census from the bit-exact
  Python replica `rust_expect` (tku3_core.py): n4 figure `33344`
  count 78 with reps {(0,0), (1000,2000)} (the two band-junction
  translation classes) and `4444` count 42 with rep {(1000,1000)};
  n14 figure `333333` count 42 with rep {(500,866)} and `33344`
  count 78 with reps {(0,0), (0,1732)}. One winding fix at port time:
  the spike's `st2_up` triangle (winding-agnostic Python battery) is
  reversed in the Rust table so the CCW sector walk closes.
- **Census:** 96 → 98 tilings, 108 → 110 memberships, TwoUniform
  11 → 13, 674 → 682 tests.
- **Status:** landed (M103). Full bar green. 13 of the 20 closed-list
  entries landed, 7 remain. Not landed this batch: `2-uniform n16`
  (`[3³.4²; 3².4.3.4]₁ (t=4, e=5)`, p4g, stretch goal) — its wiki
  SVG resisted strict exact probing (the figure's own tile-extraction
  noise is ~3% at unit-edge scale), but a noise-tolerant translation
  probe extracts its translation lattice: square lattice
  ⟨((3+√3)/2, (3+√3)/2), (−(3+√3)/2, (3+√3)/2)⟩, area 6+3√3
  (8 vertices/cell ⇒ t = 4 with 2 reps/orbit, consistent with the
  wiki parenthetical); full combinatorial derivation + construction
  stays open.


### 61. K-uniform duals batch 3 (pmm/n12/n19) ✅ (M106)

- **Motif:** the face-centroid duals of the remaining non-square
  2-uniform primals — `kuniform-pmm` (pmm, rectangular),
  `kuniform-n12` (cmm, centered rectangular), `kuniform-n19` (p6m,
  hexagonal L9 flower lattice) — constructed at runtime by the same
  `dual()` helper that produced the M98/M102 batches; no
  hand-transcribed dual geometry. Batch 3 is the first to leave the
  hexagonal 60° basis: it lands the family's first pmm- and cmm-duals,
  and the first dual of a non-flower hexagonal tiling (the n19 dual's
  lattice is the primal's L9 itself). *Provisional number — this
  fork's numbering is already out of sequence (duplicate §56), and the
  M-number will be renumbered at integration.*
- **The three duals (per cell):**
  - `kuniform-dual-pmm` (rectangular a1 = (2,0), a2 = (0,√3),
    det 2√3): 3 faces — 1 rhombus (the 3.6.3.6 orbit — the shared
    vertex of a hexagon pair; sides 2/√3 ×4, angles 60°/120° in
    cyclic order, area 2√3/3, Voronoi vertex (1,0)) + 2 congruent
    quadrilaterals (the 3².6² orbit — the hexagon side corners, two
    translation classes under the rectangular lattice; sides 1/√3,
    2/√3, 2/√3, √3 in cyclic order; angles 60°/60°/120°/120°; area
    2√3/3). Σ = 2√3.
  - `kuniform-dual-n12` (centered rectangular a1 = (√3,0),
    a2 = (√3/2,5/2), det 5√3/2): 4 faces — 2 congruent quadrilaterals
    (the 3².6² orbit; sides 1/√3, 2/√3, 2/√3, √3; angles
    60°/60°/120°/120°; area 2√3/3) + 2 congruent pentagons (the 3⁴.6
    orbit; sides 1/√3 ×3 and 2/√3 ×2; angles 60°/120°/120°/120°/120°;
    area 7√3/12 — the same 3⁴.6 dual pentagon as
    `kuniform-dual-n20`). Σ = 5√3/2.
  - `kuniform-dual-n19` (hexagonal L9, det 9√3/2): 8 faces — 6
    congruent pentagons (the 3⁴.6 orbit — one per corner of the
    origin flower, each a distinct translation class; sides 1/√3 ×3
    and 2/√3 ×2; area 7√3/12) + 2 regular hexagons (the 3⁶ orbit —
    the petal tips; side 1/√3, area √3/2, at (3/2, √3/2) and
    (0, √3)). Σ = 9√3/2.
  Each primal's vertex orbits split into more translation classes than
  its full-group orbits (pmm: 3, n12: 4, n19: 8 per cell), so the
  dual cell carries 3, 4, and 8 faces respectively — the census test
  keys classes by (vertex count, count in cell), as since M102.
- **Group:** measured, not assumed. The centroid dual is equivariant
  and involutive, so every dual keeps its primal's group: pmm stays
  pmm, cmm stays cmm, p6m stays p6m — but the center structure of the
  rectangular/centered-rectangular cases is new, so the symmetry test
  was generalized: `Case` gained `lattice_kind` (per-case lattice
  assertion replacing the blanket Hexagonal check) and `mirror_dirs`,
  and `duals_match_measured_groups` now branches on group — P6m/P6
  cases keep the C6 + six-mirrors-through-6-fold-center probes with
  R30 negative; Pmm/Cmm cases probe R180 about the measured
  mirror-intersection 2-fold center (positive), R30/R60/R90/R120
  about it (all negative), the two measured axis-parallel mirrors
  through the center (positive), the 45° bisector mirror (negative),
  and R180 about an off-center point (negative). Measured: the pmm
  dual's 2-folds are (0,0), (1,0), (0,√3/2), (1,√3/2) — one per
  mirror intersection — with horizontal/vertical mirrors through all
  of them; the n12 dual's 2-folds are (0,0) and (√3/2,0) on the
  mirror intersections plus (√3/4,5/4) and (3√3/4,5/4) off the
  mirrors (glide-only — no mirror lines pass through them); the n19
  dual's 6-fold center is the origin (the primal's flower center,
  where its six pentagons meet) with 2-folds additionally at (3/2,0),
  (3/4,3√3/4), (9/4,3√3/4).
- **Verification (9 new tests):** the per-dual
  covers_region/rotated_scaled/deterministic trio (house pattern);
  the shared battery now covers all 11 duals — convex + CCW per face
  with per-class per-face areas vs. exact Q(√3) values to 1e-9 (class
  lookup by vertex count AND count in cell), edge-to-edge over a
  ±4-tile window (0 unpaired, 0 T-junctions), exactly-once fine-grid
  coverage, proper coloring, orbit correspondence, and the
  measured-group symmetry probes above.
- **Category placement:** Laves primary only, `secondary: &[]`, as in
  the M98/M102 batches (2-uniform duals, not Two-Uniform arc
  entries).
- **Rust:** `dual.rs` +3 constructors (`kuniform_dual_pmm/n12/n19()`,
  feature `kuniform`), registry +3 (second `#[cfg(feature =
  "kuniform")]` block, after `kuniform-dual-n20`), culling list +3,
  census 100 → 103 tilings / 112 → 115 memberships (Laves 19 → 22 =
  19 primary + 3 self-dual secondaries; 11 machine-dualized k-uniform
  duals across M98 + M102 + M106), tess-tui tab pins updated to the
  same counts.
- **Status:** landed (M106, fork-local numbering — renumbered at
  integration). 103 tilings, 115 memberships, 695 tests. Full bar
  green: cargo check + clippy (both target sets) zero warnings, cargo
  test --workspace --all-features 695/695. Remaining k-uniform duals:
  the 9 square-family rows stay blocked on unimplemented primals.


### 62. 2-uniform [4⁴; 3³.4²]₂ (n3) + [3⁶; 3³.4²]₂ (n15) ✅ (M108)

- **Numbering note:** renumbered from the fork's provisional §61 to
  §62 by landing order (§61 = k-uniform duals batch 3, M106).
- **Motif:** the two sibling forms of the M103 pair from Krotenheerdt's
  closed list: Wikipedia `2-uniform n3`, `[4⁴; 3³.4²]₂ (t=3, e=5)`,
  and Wikipedia `2-uniform n15`, `[3⁶; 3³.4²]₂ (t=4, e=5)`. Label
  caveat (third occurrence): the sprint brief's row indices were stale
  again; anchor-based caption extraction from the saved wiki page
  (caption follows the image inside the same table cell) gives
  `n3.svg` ↔ `[44; 33.42]2 (t=3, e=5)` and `n15.svg` ↔
  `[36; 33.42]2 (t=4, e=5)`, with `n4`/`n14` the already-landed
  siblings and **no `n17` in the wiki set at all**; both torus
  enumerations below reproduce the corrected (3,5)/(4,5) exactly,
  confirming the mapping. The brief's p4g stretch row resolves to the
  `n16` row — see Status.
- **Constructions.** n3: three-square-tall bands (rows R,C,R —
  band-height 3) alternating with one-row alternating-triangle bands
  (height H); consecutive bands staggered by half a unit (forced by
  the triangle band sharing full edges with both square rows). Basis
  b1 = (1,0), b2 = (1/2, 3+√3/2), |det| = 3+√3/2; cell = 3 squares +
  2 triangles; the row color pattern R,C,R is one mirror-fused orbit
  plus one singleton (3 tile orbits total). n15: single square rows
  alternating with TRIPLE triangle strips (three rows, height 3H) in
  which every horizontal edge is shared back-to-back by one triangle
  above and one below; the two lines bounding a square row carry a
  single flanker orbit (down triangle on the bottom edge, up triangle
  on the top edge, squares' own x grid); inner lines step half a unit
  at a time. Basis b1 = (1,0), b2 = (1/2, 1+3√3/2),
  |det| = 1+3√3/2; cell = 1 square + 6 triangles in three pairs.
  Both lattices are centered-rectangular: the orthogonal index-2
  pairs (1,0), (0, 6+√3) and (1,0), (0, 2+3√3) generate sublattices
  whose centering vectors (b2 in each case) they lack — so both
  entries are cmm, not pmm.
- **Exact solve (tku4_core.py / tku4_run.py / tku4_orbits.py,
  Q(√3) surd-pair arithmetic):** both tables decoded from the wiki
  SVGs by level analysis (base-line census with raw centroid grids —
  mod-1 grids cannot distinguish the back-to-back mesh). Area identity
  |det| = Σ areas exact; coverage 1.0 over ±6-cell patches;
  0 edges with >2 sides. **Torus pitfall (new):** the M103-style
  2×2-cell torus quotient returns t = 8 for n15 — a parity artifact:
  the half-period staggering splits lattice-orbit classes by cell
  parity at quotient size 2. Replaced by the exact affine action on
  lattice-orbit classes (fundamental-domain reduction of every
  generator image, permutation orbit closure — no quotient-size
  dependence): n3 tile orbits t = 3 (sizes 1+2+2), edge orbits
  e = 5 (sizes 1+2+2+2+2 over 9 edge classes), vertex orbits 4444
  and 33344 (2 classes/cell each); n15 F = 7, E = 11, V = 4
  (V−E+F = 0), tile orbits t = 4 (sizes 1+2+2+2 — exactly the wiki
  figure's four colors: square, flanker pair, yellow pair, cyan
  pair), edge orbits e = 5 (sizes 1+2+2+2+4), vertex orbits 3⁶ and
  33344 (2 classes/cell each). Both t/e match the wiki
  parentheticals (3,5) and (4,5) exactly. Full scans: n3 R180
  (72 centers) + mirror0 (36) + mirror90 (208), nothing else;
  n15 R180 (9) + mirror0 (9) + mirror90 (203), nothing else —
  with the centered lattices, the cmm signature. The horizontal
  mirror through a square-row center fuses each same-colored pair in
  n15 once the half-period lattice shift is composed in (verified as
  exact orbit classes, not assumed from color).
- **Rust:** `kuniform_n3()` / `kuniform_n15()` in `kuniform.rs`
  (module header "Thirteen entries" → "Fifteen entries"), registered
  after `kuniform_n14`; 4 tests each (`covers_region`,
  `rotated_scaled` 0.7/30°, `deterministic`, `vertex_figures`). The
  vertex-figure tests pin the ±3×±3 patch census from the bit-exact
  Python replica `rust_expect`: n3 figure `33344` count 72 with reps
  {(0,0), (1000,3000)} and `4444` count 84 with reps
  {(1000,1000), (1000,2000)}; n15 figure `333333` count 78 with reps
  {(500,2732), (1000,1866)} and `33344` count 78 with reps
  {(500,0), (500,1000)}. One winding fix at port time (batch-3
  lesson re-applied): the n3 `t_dn` vertex order came out CW in the
  first port and the CCW sector walk failed with
  `figure walk gap: sector at 120, expected 60` — reversed to the
  spike's CCW order.
- **Census:** 102 → 104 tilings, 114 → 116 memberships, TwoUniform
  13 → 15, 698 → 706 tests (fork-local).
- **Status:** landed (M108, fork-local numbering). 15 of the 20
  closed-list entries landed, 5 remain. Not landed this batch: the
  p4g stretch — its brief row resolves to the wiki `n16` entry
  (`[3³.4²; 3².4.3.4]₁ (t=4, e=5)`), whose landing was explicitly
  excluded by the batch brief; the §60 blocker stands (lattice
  ⟨((3+√3)/2, (3+√3)/2), (−(3+√3)/2, (3+√3)/2)⟩, area 6+3√3,
  8 vertices/cell; combinatorial derivation open).


---

*Map: [docs/INDEX.md](../INDEX.md) · Changelog: [docs/history/CHANGELOG.md](../history/CHANGELOG.md) · Queue: [docs/ROADMAP.md](../ROADMAP.md)*
