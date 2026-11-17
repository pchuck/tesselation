# 2D Tilings Reference: Uniform, Laves, Wallpaper, Quasiperiodic, Aperiodic, Nonperiodic

Nov 16, 2026 · @p c

## How to read this

Five of the nine categories below are finite and fully listed, one is enumerated by computer, and three are open-ended, so those are sampled, not exhausted. Scope is the Euclidean plane only: 3D tilings, 1D sequences and hyperbolic tilings are excluded.

| Category | Status | Size |
| --- | --- | --- |
| Uniform (Archimedean) tilings | Complete | 11 |
| k-uniform tilings | Enumerated by computer to k = 13 | Infinite family |
| Laves tilings (duals) | Complete | 11 |
| Wallpaper groups | Complete | 17 |
| Isohedral / isogonal / isotoxal types | Complete | 81 or 93 / 91 / 26 |
| Convex monohedral tiles | Complete (pentagon proof still awaiting peer review) | all triangles and quadrilaterals, 15 pentagons, 3 hexagons |
| Quasiperiodic tilings | Representative list | Open-ended |
| Aperiodic tile sets | Representative list | Open-ended |
| Nonperiodic tilings that are not aperiodic | Representative list | Uncountable |

**Markers.** *Verified* means the fact was confirmed against a source retrieved on 2026-11-16 (listed at the end). *Unverified* means it comes from general knowledge and was not re-checked.

## 1. Uniform (Archimedean) tilings

There are exactly 11 uniform tilings: 3 regular and 8 semiregular. A uniform tiling is edge-to-edge, uses convex regular polygons, and every vertex looks the same (vertex-transitive). The vertex configurations and the 3 + 8 split are verified; the symmetry groups are from general knowledge.

| Vertex config | Name | Wallpaper group (unverified) |
| --- | --- | --- |
| 3.3.3.3.3.3 | Triangular (regular) | p6m |
| 4.4.4.4 | Square (regular) | p4m |
| 6.6.6 | Hexagonal (regular) | p6m |
| 3.3.3.3.6 | Snub hexagonal (chiral, two mirror forms) | p6 |
| 3.3.3.4.4 | Elongated triangular | cmm |
| 3.3.4.3.4 | Snub square | p4g |
| 3.4.6.4 | Rhombitrihexagonal | p6m |
| 3.6.3.6 | Trihexagonal (kagome) | p6m |
| 3.12.12 | Truncated hexagonal | p6m |
| 4.6.12 | Truncated trihexagonal | p6m |
| 4.8.8 | Truncated square | p4m |

Grünbaum and Shephard also call non-edge-to-edge isogonal tilings by regular polygons "uniform". Coxeter's stricter definition excludes them. That wider family is uncountable (see section 10).

## 2. k-uniform tilings

A k-uniform tiling is an edge-to-edge tiling by convex regular polygons with k vertex orbits. The counts are known by exhaustive computer search up to k = 13 and grow roughly 2x per step; there is no closed form. All values below are verified (OEIS A068599).

| k | Count | Found by |
| --- | --- | --- |
| 1 | 11 | Classical |
| 2 | 20 | Classical |
| 3 | 61 | Chavey 1984 |
| 4 | 151 | Galebach 2002 |
| 5 | 332 | Galebach 2002 |
| 6 | 673 | Galebach 2003 |
| 7 | 1,472 | Galebach 2020 |
| 8 | 2,850 | Čtrnáct 2021 |
| 9 | 5,960 | Čtrnáct 2021 |
| 10 | 11,866 | Čtrnáct 2021 |
| 11 | 24,459 | Čtrnáct 2021 |
| 12 | 49,794 | Čtrnáct 2021 |
| 13 | 103,082 | Čtrnáct 2021 |

The tilings that actually use k distinct vertex types (k-Archimedean) number 11, 20, 39, 33, 15, 10 and 7 for k = 1 to 7. For example, the 61 three-uniform tilings split into 39 with three vertex types and 22 with two.

The full catalogs, with images, are Brian Galebach's n-Uniform Tilings page (k up to 6) and the Čtrnáct–Kopczyński tessellation catalog. Galebach's method was never published in full, and some authors treat his lists as exhaustive. Čtrnáct's independent search reproduced them for k ≤ 6.

## 3. Laves tilings (Catalan duals)

There are exactly 11 Laves tilings, one dual for each uniform tiling, so the count follows directly from section 1. Each is face-transitive (every tile is equivalent) and has the same symmetry group as its uniform original. The dual relationships are verified; the common names and tile shapes are from general knowledge (unverified).

| Dual of | Laves symbol | Common name | Tile shape |
| --- | --- | --- | --- |
| 3.3.3.3.3.3 | V3.3.3.3.3.3 | Hexagonal | Regular hexagon |
| 4.4.4.4 | V4.4.4.4 | Square (self-dual) | Square |
| 6.6.6 | V6.6.6 | Triangular | Equilateral triangle |
| 3.3.3.3.6 | V3.3.3.3.6 | Floret pentagonal | Irregular pentagon |
| 3.3.3.4.4 | V3.3.3.4.4 | Prismatic pentagonal | Irregular pentagon |
| 3.3.4.3.4 | V3.3.4.3.4 | Cairo pentagonal | Irregular pentagon |
| 3.4.6.4 | V3.4.6.4 | Deltoidal trihexagonal | Kite |
| 3.6.3.6 | V3.6.3.6 | Rhombille | Rhombus (60/120 degrees) |
| 3.12.12 | V3.12.12 | Triakis triangular | Isosceles triangle |
| 4.6.12 | V4.6.12 | Kisrhombille (bisected hexagonal) | 30-60-90 right triangle |
| 4.8.8 | V4.8.8 | Tetrakis square | Isosceles right triangle |

The duals of the three regular tilings are themselves regular, so only the other eight are new shapes. The three pentagonal Laves tilings are among the convex pentagon tilings in section 6.

## 4. Wallpaper (plane crystallographic) groups

There are exactly 17 wallpaper groups, the ways a pattern can repeat in two independent directions. This is a classical theorem (Fedorov 1891, Pólya 1924). The notation below is from general knowledge, not re-checked (unverified), though several orbifold symbols appear in the pentagon-tiling sources.

| Lattice | Group | Orbifold | Highest rotation | Mirrors |
| --- | --- | --- | --- | --- |
| Oblique | p1 | o | none | none |
| Oblique | p2 | 2222 | 2-fold | none |
| Rectangular | pm | \*\* | none | yes |
| Rectangular | pg | ×× | none | glide only |
| Rectangular | pmm | \*2222 | 2-fold | yes |
| Rectangular | pmg | 22\* | 2-fold | yes |
| Rectangular | pgg | 22× | 2-fold | glide only |
| Centered rectangular | cm | \*× | none | yes |
| Centered rectangular | cmm | 2\*22 | 2-fold | yes |
| Square | p4 | 442 | 4-fold | none |
| Square | p4m | \*442 | 4-fold | yes |
| Square | p4g | 4\*2 | 4-fold | yes |
| Hexagonal | p3 | 333 | 3-fold | none |
| Hexagonal | p3m1 | \*333 | 3-fold | yes |
| Hexagonal | p31m | 3\*3 | 3-fold | yes |
| Hexagonal | p6 | 632 | 6-fold | none |
| Hexagonal | p6m | \*632 | 6-fold | yes |

Only 2-, 3-, 4- and 6-fold rotations are possible in a periodic pattern (the crystallographic restriction). 5-, 7-, 8- and higher-fold symmetry appears only in the quasiperiodic tilings of section 7.

## 5. Isohedral, isogonal and isotoxal tiling types

Grünbaum and Shephard classified periodic tilings by which parts their symmetry group acts transitively on. All counts below are verified, and the totals depend on what you count as a "type".

| Property | Symmetry acts transitively on | Number of types | Notes |
| --- | --- | --- | --- |
| Isohedral | Tiles | 93 (or 81) | 93 types of marked tilings (IH1 to IH93); 81 of them can be shown with unmarked tiles. Heesch and Kienzle's older scheme gives 28. |
| Isogonal | Vertices | 91 | Each tile has at least 3 neighbors. Of these, 11 combinatorial types are realized by the Archimedean tilings. |
| Isotoxal | Edges | 26 | Normal (bounded) tilings only. 25 of the 26 are also isohedral, isogonal or both. |

The 81 and 93 figures are both correct, so state which one you mean.

## 6. Convex monohedral tiles

Every triangle and every quadrilateral tiles the plane, exactly 15 types of convex pentagon do, exactly 3 types of convex hexagon do, and no convex polygon with 7 or more sides does. This replaces my earlier "3 triangle types, 4 quadrilateral types", which was wrong.

| Sides | Which tile | Source |
| --- | --- | --- |
| 3 | All triangles | Classical |
| 4 | All quadrilaterals, convex or not | Classical |
| 5 | 15 types | Reinhardt 1918 to Rao 2017 |
| 6 | 3 types | Reinhardt 1918 |
| 7 or more | None | Reinhardt 1918 |

**The 15 pentagon types.** All 15 tile periodically. Only 8 can tile edge-to-edge (Bagina 2011, Sugimoto 2012).

| Types | Found by | Year |
| --- | --- | --- |
| 1 to 5 | Reinhardt | 1918 |
| 6 to 8 | Kershner | 1968 |
| 10 | James | 1975 |
| 9, 11, 12, 13 | Rice (amateur) | 1976 to 1977 |
| 14 | Stein | 1985 |
| 15 | Mann, McLoud-Mann, Von Derau (computer search) | 2015 |

**Completeness status.** Rao's computer-assisted proof (2017) checked 371 corner configurations and found no further types. Hales independently verified it. The sources I found still describe it as awaiting peer review, and I found no journal publication. A 2025 paper notes the written hexagon classification rests on only two old proofs, Reinhardt (1918) and Bollobás (1963), but the result itself is not in doubt.

**Consequence.** No convex polygon tiles only aperiodically, so any aperiodic monotile must be non-convex.

**Isohedral counts by tile shape** (one source, Jaap Scherphuis): 14 isohedral tilings by a triangle, 56 by a quadrilateral, 24 by a pentagon and 13 by a hexagon.

## 7. Quasiperiodic tilings

Quasiperiodic tilings are nonperiodic but have long-range order, with a diffraction pattern of sharp peaks and rotational symmetry forbidden in periodic patterns (5, 8, 10, 12 and so on). This list is representative, not exhaustive: the Bielefeld Tilings Encyclopedia holds hundreds of substitution tilings, and the cut-and-project method generates an unbounded family.

| Tiling | Symmetry | Tiles and date | Notes |
| --- | --- | --- | --- |
| Penrose P1 | 5-fold | 6 tiles, 1974 | Pentagon-based. Verified. |
| Penrose P2 (kite and dart) | 5/10-fold | 2 tiles, 1977 | Verified. |
| Penrose P3 (rhombs) | 5/10-fold | 2 tiles, 1978 | Verified. |
| Robinson triangles, "starfish, ivy leaf, hex", Cromwell kite-rhombus-trapezium | 5/10-fold | 4, 3 and 3 tiles | All locally derivable from the Penrose tilings. Verified. |
| Ammann–Beenker (Ammann A5) | 8-fold | 2 tiles, 1982 | Ammann A4 is locally derivable from it. Verified. |
| Ammann A2 and A3 | Golden-ratio family | 2 and 3 tiles, 1986 | Ammann bars give the matching rules. Quasiperiodic status is unverified. |
| Golden triangle | 5/10-fold | 10 tiles, matching rules 2001 | Dual to Ammann A2. Verified. |
| Socolar | 12-fold | 3 tiles, 1989 | Locally derivable from the shield tiling. Verified. |
| Gähler shield | 12-fold | 4 tiles, 1988 | Same tiling class as Socolar. Verified. |
| Square-triangle (attributed to Stampfli) | 12-fold | 5 tiles, 1986 | Tile count verified; attribution unverified. |
| Tübingen triangle | 10-fold | 2 triangles | A Penrose-type triangle substitution. Cut-and-project origin unverified. |
| Danzer 7-fold | 7/14-fold | 3 triangles; 6-tile matching set 1996 | Substitution tiling. Whether it is quasiperiodic in the diffraction sense is unverified. |
| Generalized n-fold rhomb tilings (de Bruijn, Harriss) | n-fold | Rhombs | Cut-and-project family; Harriss generalized a 7-fold substitution rule to arbitrary n. Verified. |
| Hat and spectre tilings | Hierarchical | 1 tile | Analyzed as quasicrystalline by Socolar (2023). See section 8. |

**What is not on this list.** Gummelt's decagon is a covering (overlapping copies under matching rules) that reproduces Penrose tilings, not a tiling. The Robinson and Ammann A1 tilings are hierarchical (nested lattices and trees), not cut-and-project tilings. Robinson and Ammann A1 are listed in section 8, and the covering/tiling distinction is repeated in section 10.

## 8. Aperiodic tile sets

A tile set is aperiodic if it tiles the plane but only nonperiodically, so Penrose, Ammann and the hat all qualify. The list of sets is open-ended: a July 2026 arXiv paper (Akiyama, Hamada, Ito) builds aperiodic sets from Sturmian words for any quadratic-irrational slope, and the hat family is a continuum.

**Aperiodic monotiles ("einsteins")**

| Tile | Year | What it does |
| --- | --- | --- |
| Taylor–Socolar | 2010 | Disconnected tile with matching rules; its tilings need reflected copies. Verified. |
| Hat | 2023 | 13-sided, built from 8 kites. Every tiling mixes reflected and unreflected copies. Verified. |
| Turtle | 2023 | Built from 10 kites; a member of the same family as the hat. Verified. |
| Tile(a,b) family | 2023 | A continuum containing the hat and turtle. Every member is aperiodic except Tile(0,1), Tile(1,1) and Tile(1,0), which also tile periodically. Verified. |
| Tile(1,1) | 2023 | 14-sided. Weakly chiral: aperiodic only if reflections are forbidden. Verified. |
| Spectre | 2023 | Strictly chiral: aperiodic even when reflections are allowed (*Combinatorial Theory* 2024). Verified. |

"Einstein" has no single definition. With matching rules allowed, even a 2 by 1 rectangle can be made aperiodic, so the interesting cases need connected tiles and no matching rules.

**Small plane tile sets** (counts from Wikipedia's list of aperiodic sets)

| Set | Tiles | Year |
| --- | --- | --- |
| Robinson tiles | 6 | 1971 |
| Penrose P1 | 6 | 1974 |
| Penrose P2 (kite and dart) | 2 | 1977 |
| Penrose P3 (rhombs) | 2 | 1978 |
| Ammann A1 | 6 | 1977 |
| Ammann A2 | 2 | 1986 |
| Ammann A3 | 3 | 1986 |
| Ammann A4 | 2 | 1986 |
| Ammann A5 (Ammann–Beenker) | 2 | 1982 |
| Shield | 4 | 1988 |
| Socolar | 3 | 1989 |
| Binary tiles | 2 | 1988 |
| Square-triangle | 5 | 1986 |
| Pinwheel (Conway–Radin) | 1, in infinitely many orientations | 1994 (matching rules) |
| Danzer triangles | 6 | 1996 |
| Penrose hexagon-triangle | 3 | 1997 |
| Golden triangle | 10 | 2001 |
| Robinson triangle; starfish, ivy leaf and hex | 4; 3 | Undated |

The Penrose sets, Robinson triangles and the starfish set generate locally equivalent tilings, so they are one family in different dress. All entries in this table are verified.

**Wang tile sets** (unit squares with colored edges, translations only)

| Tiles | Colors | Found by | Year |
| --- | --- | --- | --- |
| 20,426 |  | Berger | 1966 |
| 104 (103 by a later count) |  | Berger |  |
| 40 |  | Läuchli |  |
| 24 |  | Robinson | 1977 |
| 16 |  | Ammann | 1978 or 1986, depending on source |
| 14 | 6 | Kari | 1996 |
| 13 | 5 | Čulik | 1996 |
| 11 | 4 | Jeandel and Rao | 2015 (published 2021) |

Jeandel and Rao proved by exhaustive search that 11 tiles and 4 colors are both minimal. Mozes (1989) showed any substitution tiling can be converted to a Wang set. All entries above are verified.

## 9. Nonperiodic tilings that are not aperiodic

These tilings have no translational symmetry, but the same tile or tile set can also tile periodically, so the tiles do not force nonperiodicity. They are uncountable in number, so the examples below are the named families.

| Tiling | Why it has no period | Why it is not aperiodic | Status |
| --- | --- | --- | --- |
| Sphinx tiling | Four sphinxes tile a larger sphinx, repeated forever (a rep-tile) | Two sphinxes form a parallelogram that tiles periodically | Verified |
| Voderberg spiral | A monohedral nonagon spirals outward; no translations | The nonagon also tiles periodically | Verified |
| Other spiral tilings (Grünbaum–Shephard) | Rotational spiral structure | Periodic arrangements exist for the same tiles | Verified for the family; individual examples unverified |
| Staggered or fault-line tilings | Rows of squares or bricks shifted by arbitrary offsets | The same tiles tile in a regular grid | Verified |
| Strips of squares and triangles | Any sequence of square strips and triangle strips tiles; uncountably many sequences | Regular alternation is periodic | Verified (simple argument) |
| Versatile tiles, such as a decagon with a bite | Can be arranged nonperiodically | Also tile periodically | Verified |
| Van Dongen one-tile triangle (2021) | Isosceles right triangle with substitution rules | The bare triangle tiles periodically; only the marked version is forced nonperiodic | Verified |
| Klaassen seeded tilings (2021) | One tile grown from a seed under rules | The nonperiodicity comes from the seed and growth rules, not from the tile alone | Source verified; reading unverified |
| Chair tiling (L-tromino substitution) | Hierarchical substitution | L-trominoes also tile periodically | Unverified |
| Truchet, random and Voronoi tilings | Random choices | Regular choices give periodic tilings | Unverified |
| Fractal tilings (Gosper island, Sierpinski-type rep-tiles) | Self-similar boundaries | Tiles can also be arranged periodically | Unverified |

No convex polygon is in this list as an aperiodic case, since every tiling convex polygon admits a periodic tiling (section 6). Hyperbolic tilings {p,q} with (p-2)(q-2) > 4 are an infinite family but live outside the Euclidean plane, so they are excluded here.

## 10. Open-ended categories and what is excluded

Six kinds of tilings cannot be listed in full, and several nearby topics are deliberately left out.

**Cannot be listed**

| Category | Why |
| --- | --- |
| k-uniform tilings for k of 14 or more | The count keeps growing about 2x per step; no closed form is known. |
| Non-edge-to-edge tilings by regular polygons | Uncountably many, with continuously shifting contact points. Grünbaum and Shephard's book notes nobody has attempted a full classification. |
| Edge-to-edge regular-polygon tilings that are not uniform | Uncountably many; any sequence of square and triangle strips works. |
| Tilings by regular star polygons | Kepler and Grünbaum–Shephard started the enumeration; Grünbaum–Shephard's list is incomplete. |
| Substitution and quasiperiodic tilings | The Bielefeld Tilings Encyclopedia is the standard catalog. |
| Aperiodic tile sets | New constructions keep appearing (Sturmian-lattice sets, 2025 to 2026). |

**Excluded as outside the 2D Euclidean scope**

- 3D: Schmitt–Conway–Danzer biprism, 3D Socolar–Taylor tile, Penrose rhombohedra, Wang cubes. Verified except Wang cubes.
- 1D sequences: Fibonacci word, Thue–Morse.
- Hyperbolic tilings and the Böröczky hyperbolic tile, which is only weakly aperiodic.
- Spherical tilings, which have a separate classification.
- Coverings: Gummelt's decagon is an overlapping cover that reproduces Penrose tilings, not a tiling.

## 11. Corrections to the earlier list

The first version of this list had nine errors or overlaps, all fixed above.

| Earlier claim | Correction |
| --- | --- |
| "3 triangle types, 4 quadrilateral types" | Every triangle and every quadrilateral tiles the plane (section 6). |
| Sphinx listed as aperiodic | It tiles periodically (two sphinxes make a parallelogram), so it is nonperiodic only (section 9). |
| 3D items in a 2D list | Schmitt–Conway–Danzer biprism, Wang cubes and the 3D Socolar–Taylor tile removed (section 10). |
| 1D items in a 2D list | Fibonacci word and Thue–Morse removed. |
| Gummelt decagon listed as a quasiperiodic tiling | It is an overlapping covering, not a tiling. |
| "Kari–Čulik 13" listed beside Čulik 13; "Robinson (56)" | Duplicate merged; Wang history rebuilt from sources, and the 56 dropped as unconfirmed. |
| "81 isohedral types" with no context | 81 (unmarked) and 93 (marked) are both correct; section 5 says which is which. |
| k-uniform counts stopped at k = 7 | Extended to k = 13. |
| Ammann–Beenker and Ammann A5, and Socolar and shield, listed as separate tilings | Each pair is one locally derivable class. |

Two items were also downgraded to unverified: Danzer's 7-fold tiling as quasiperiodic, and the Stampfli attribution of the square-triangle tiling.

## 12. Sources

All sources were retrieved through web search on 2026-11-16; I read the returned excerpts, not every page in full.

- [OEIS A068599, number of n-uniform tilings](https://oeis.org/A068599)
- [Wikipedia: List of k-uniform tilings](https://en.wikipedia.org/wiki/List_of_k-uniform_tilings)
- [Wikipedia: Euclidean tilings by convex regular polygons](https://en.wikipedia.org/wiki/Euclidean_tilings_by_convex_regular_polygons)
- [Wikipedia: Pentagonal tiling](https://en.wikipedia.org/wiki/Pentagonal_tiling)
- [Quanta Magazine: Pentagon tiling proof](https://quantamagazine.org/pentagon-tiling-proof-solves-century-old-math-problem-20170711)
- [CNRS via ScienceDaily: only 15 pentagonal tiles](https://www.sciencedaily.com/releases/2017/09/170929115008.htm)
- [Klaassen, equilateral convex tilings (arXiv 2506.18473)](https://arxiv.org/pdf/2506.18473)
- [Hexagon tiling proof (arXiv 2508.01404)](https://arxiv.org/pdf/2508.01404)
- [Jaap's Puzzle Page: tilings](https://jaapsch.net/tilings)
- [Grünbaum and Shephard, 91 isogonal types](https://www.ams.org/journals/tran/1978-242-00/S0002-9947-1978-0496813-3/)
- [Grünbaum and Shephard, Isotoxal tilings](https://msp.org/pjm/1978/76-2/p13.xhtml)
- [Kaplan and Schattschneider, Isohedral tiles and notations](https://isohedral.ca/isohedral-tiles-tilings-and-notations/)
- [Wikipedia: List of aperiodic sets of tiles](https://en.wikipedia.org/wiki/List_of_aperiodic_sets_of_tiles)
- [Tilings Encyclopedia, Bielefeld](https://tilings.math.uni-bielefeld.de/glossary/matching-rules)
- [Smith et al., A chiral aperiodic monotile (arXiv 2305.17743)](https://arxiv.org/abs/2305.17743v2)
- [Planar aperiodic tile sets: from Wang tiles to the Hat and Spectre monotiles](https://arxiv.org/abs/2310.06759)
- [Jeandel and Rao, An aperiodic set of 11 Wang tiles](https://arxiv.org/abs/1506.06492)
- [Gardner, Extraordinary nonperiodic tiling (sphinx, Voderberg)](https://web.ma.utexas.edu/users/radin/gardner.pdf)
- [Wikipedia: Voderberg tiling](https://en.wikipedia.org/wiki/Voderberg_tiling)
- [Van Dongen, aperiodic tiling from one triangle](https://www.scribd.com/document/665658984/2111-03979)
- [Klaassen, forcing nonperiodic tilings with one tile using a seed](https://ar5iv.labs.arxiv.org/html/2109.09384)
- [Gummelt, Penrose tilings as coverings of decagons](https://doi.org/10.1007/bf00239998)
- [Akiyama, Hamada and Ito, Aperiodic tile sets from Sturmian lattices (arXiv 2607.14693)](https://arxiv.org/abs/2607.14693)
- [Adams et al., tilings of the sphere by regular polygons](https://arxiv.org/pdf/2101.10743)
