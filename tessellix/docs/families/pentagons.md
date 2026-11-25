# Pentagon types family

The convex pentagonal tiling census as `LatticeTiling` data tables — 13 of the 15 types here, types 4/5 realized by the Laves-tab duals, plus the Hirschhorn–Hunt non-edge-to-edge member.

Code: `crates/tess-tilings/src/families/pentagons/` — cargo feature `pentagons` (default-on).
Adding a tiling here touches only this family: `families/pentagons.rs`
(module decl, register slice, classification arms, census `IDS`,
`lattice_tilings` entries) plus the new module and its doc row below.
Work briefs: `docs/briefs/TEMPLATE.md`; fleet protocol: `tessellix/AGENTS.md`.

## Implemented (14 tilings)

The convex pentagonal tiling census as `LatticeTiling` data tables;
ids are the type numbers. Types 4 and 5 are realized by the Laves-tab
Catalan duals `cairo-pentagonal` and `floret-pentagonal` (section
above, and typed Pentagon-members since M122); the other 13 types are
here. The M101 coverage audit verified types 1/4/5 and falsified the
shipped hirschhorn member's type-8 label (it is type 1).

| id | motif | group | notes |
|---|---|---|---|
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
| `pentagon-1` | Reinhardt pentagon, type 1 (1918) | cmm | 1-isohedral, 2 tiles per centered-rectangular cell (60°-rhombus basis, the a² = 3b² hexagonal-metric special case — declared CenteredRectangular per the cmm convention), exact Q(√3) at the regular-hexagon member: a side-2 hexagon cut by the segment joining the midpoints of two opposite edges (A = B = C = 120°, D = E = 90°, sides (2, 2, 1, 2√3, 1), tile area 3√3, cell det 6√3 = 2 × area); edge-to-edge — vertex stars {120°,120°,120°} at hexagon corners, {90°,90°,90°,90°} at cut midpoints; measured cmm (the brief's p2 pin overridden by the audit): achiral prototile, two perpendicular mirror families (60°/150° lines) + glides between them, four half-turn classes (two on-mirror, two off — the off-mirror 2-folds rule out pmm, the reversing isometries rule out p2); type-1 exclusive under all 10 dihedral relabellings, distinct from the also-type-1 `prismatic-pentagonal` (different pentagon) (M117) |
| `pentagon-2` | Reinhardt pentagon, type 2 (family 2, 1918) | pgg | 1-isohedral, 4 tiles (two chiral pairs) per rectangular cell, exact Q(√3) at the pinned a = 1 member (A = B = D = 90°, C = 120°, E = 150°, sides a = 1, b = d = 1 + √3/2, c = e = 1/2; the family conditions c = e, d = b, B + D = 180°); edge-to-edge; two perpendicular glide families squaring to w1 and −w2, no mirrors; M112 seed audit: seeds track chirality — the direct pair carries 0, the mirror pair 1 (the same coloring that drops the group to p2) (M65) |
| `pentagon-7` | Kershner pentagon, type 7 (1968) | pgg | 2-isohedral (seeds 0/1 — four direct, four mirror), 8 tiles per rectangular cell, edge-to-edge (every side matched by exactly one full partner side); family b = c = d = e, B + 2E = 2C + D = 360° at the published E = 106° member (A ≈ 58.86°, B = 148°, C ≈ 132.86°, D ≈ 94.27°, a ≈ 1.0357942, b = c = d = e = 1/2 — prototile computed at runtime, the family's published members are not surd angles); two perpendicular glide families squaring to w1 and −w2, no mirrors (M72) |

### Non-edge-to-edge member (no build arc)

| id | motif | notes |
|---|---|---|
| `hirschhorn` | Hirschhorn–Hunt pentagon (140°/60°/160°/80°/100°) | not locally forced; C6-symmetric constrained-growth realization |

## Build arcs

Numbered arcs keep their PATTERNS.MD §numbers — code comments and
briefs cite them as `§N`.

> Follow-up: convex pentagon census — 🚧 in progress (M49+; M49, M50, M55, M59, M60, M62, M63, M64, M65, M72, M74, M77 landed)
>
> The PROPOSED.md #5 arc (all 15 convex pentagonal tiling types). Type 13
> (Rice) lands first: its 8-tile primitive unit is small enough for a
> plain data table; type 14 (Stein) second: a 6-tile unit, but with the
> census's first exact-Q(√57) surd field; type 15 (Mann/McLoud-Mann/
> Von Derau 2015) third: a 12-tile rectangular unit in exact Q(√3) —
> the arc's first pgg — with two perpendicular glide families; type 9
> (Rice 1977) fourth: an 8-tile rectangular unit in exact Q(√3), the
> arc's second pgg and its first edge-to-edge pentagon-level Rice type;
> type 11 (Rice 1977) fifth: the arc's first non-surd instance — a
> bounded-precision 8-tile rectangular unit pinned to the reference
> animation's B = 156° member of Rice's family, and the arc's first
> non-edge-to-edge pgg; type 12 (Rice 1977) sixth: the arc's first
> surd-pinnable Rice instance — the B = 150° member of the
> 2a = d = c + e family closes to exact Q(√3) (a = 1, b = √3, c = 1,
> d = 2, e = 1), 8-tile rectangular unit, group pgg, non-edge-to-edge
> with every d-side bisected by a T-junction; type 10 (James 1975)
> seventh: the arc's first cmm and first centered-rectangular lattice —
> the B = 90° member of the a = b = c + e, d² = c² + e² family
> (isosceles, chiralities coincide as vertex sets), 6-tile oblique unit
> with det 21, non-edge-to-edge with both length-2 sides bisected; cmm
> proven by exact audit, mirrors and a half-turn ruling out p2 and the
> centered lattice L' = Z(3,3)+Z(−7,7) (centre (−2,5) ∈ L∖L') ruling
> out pmm; type 3 (Reinhardt family 3) eighth: the pinned integral
> member (2, 2, 1, 4, 3) — the arc's first 2-tile unit and its first
> chiral one-orbit p2, after a complete search proved the classical
> three-tile 120° pinwheel admits no periodic cell for this member;
> type 2 (Reinhardt family 2) ninth: the c = e, d = b, B + D = 180°
> edge-to-edge variation, pinned to the exact-Q(√3) member a = 1
> (A = B = D = 90°, C = 120°, E = 150°, b = d = 1 + √3/2,
> c = e = 1/2) — the arc's second pentagon-level edge-to-edge
> (after type 9) and its first two-chiral-pair unit: a 4-tile
> rectangular cell whose glides map each tile onto its mirror image,
> group pgg (colouring the two chiralities drops the group to p2),
> vertex figures {90°, 120°, 150°} and {90°, 90°, 90°, 90°};
> type 7 (Kershner 1968) tenth: the arc's first non-surd edge-to-edge
> pgg — published E = 106° member (A ≈ 58.86°, B = 148°,
> C ≈ 132.86°, D ≈ 94.27°, a ≈ 1.0357942, b = c = d = e = 1/2),
> runtime-computed prototile (C = root in (120°,180°) of the applet's
> transcendental closure equation; not surd-pinnable); 8-tile rectangular
> cell, four direct + four mirror, 2-isohedral (seeds 0/1, orbits
> {0,3,4,7}/{1,2,5,6}), pgg (g1² = w1, g2² = −w2, half-turns c0 =
> (1/4,0) + Λ/2), edge-to-edge, vertex stars {106°,106°,148°},
> {94.27°,132.86°,132.86°}, {58.86°,58.86°,94.27°,148°} — closing the
> M58 deferral (grid-pinning artifact + two search-phase accounting
> bugs; see PROPOSED.md #5); type 8 (Kershner 1968) eleventh: the
> exact-Q(√3) B = C = 120° member of the b = c = d = e,
> 2B + C = D + 2E = 360° family (tan A = 3√3, cos D = 1/7,
> tan E = −√3/2, A = E − 60°, a = 6√7, b = c = d = e = 14, tile area
> 189√3); 8-tile rectangular cell w1 = (36, 24√3), w2 = (−36, 18√3)
> (w1 ⊥ w2, det = 1512√3 = 8 × area), four direct + four mirror,
> 2-isohedral (seeds 0/1, orbits {0,3,4,7}/{1,2,5,6}, each orbit mixing
> both chiralities), pgg (g1² = w1, g2² = −w2, half-turn centre
> c0 = (−1/2, 25√3/2)), edge-to-edge, vertex stars
> {A,A,C,D} (79.1°, 79.1°, 120°, 81.8°) ×30, {B,B,C} (3 × 120°) ×36,
> {D,E,E} (81.8°, 139.1°, 139.1°) ×25 — the arc's second Kershner;
> type 6 (Kershner 1968) twelfth: the a = d = e, b = c, A = 2C,
> C + E = 180° family's unique 30°-grid-pinnable member (A = B = D =
> E = 120°, C = 60°, a = d = e = 1, b = c = 2, area 7√3/4); 4-tile
> oblique cell w1 = (1, 2√3), w2 = (−3, √3) (det = 7√3 = 4 × area),
> 2-isohedral (seeds 0/1, orbits {T0,T1}/{T2,T3}); p2 (half-turns
> one coset c0 + Λ/2, c0 = (1/2, 0); 22-ray mirror search 0 genuine,
> no edge direction normalizes Λ); edge-to-edge — the 20 sides
> reverse-glue pairwise (4 in-unit + 6 shifted pairs), 18 directed
> torus classes with one double-covered arc, vertex stars
> {60°,60°,120°,120°} + {120°,120°,120°}; the prototile is achiral
> and congruent to the floret-pentagonal tile — one pentagon, two
> tilings (p2 2-isohedral vs the floret's p6 1-isohedral).
>
> Type 7 is implemented (M72) — see §25, type 8 (M77) — see §26, and
> type 6 (M74) — see §27;
> the M58 bounded negative is superseded (grid-pinning artifact + two
> search-phase accounting bugs: the published instance is off-grid, so
> the grid search could never have found it). Remaining in the arc:
> types 1 and 5 covered by existing registry entries (coverage audit
> documentation pending).


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

### 28. Pentagon type 1 (Reinhardt) ✅ (M117)

- **Motif:** the regular-hexagon member of Reinhardt's type 1 — a side-2 hexagon cut by the segment joining the midpoints of two opposite edges (cut unique up to the hexagon's symmetry; (A,B,C,D,E) = (120°,120°,120°,90°,90°), sides (2,2,1,2√3,1), tile area 3√3, all in Q(√3)) (`pentagon-1`, family pentagons; M117).
- **Key facts:** 1-isohedral, 2 tiles per centered-rectangular cell (60°-rhombus basis, det 6√3 = 2 × area; the legality test now documents the a² = 3b² hexagonal-metric special realization); edge-to-edge; vertex stars {120°,120°,120°} / {90°,90°,90°,90°}; measured **cmm** overriding the brief's p2 pin (achiral prototile — a −120° rotation about the hexagon centre maps it onto its y-mirror; two perpendicular mirror families + glides between them; four half-turn classes, two off every mirror — ruling out pmm — plus orientation-reversing isometries ruling out p2); type-1 exclusive under all 10 dihedral relabellings; distinct from the shipped prismatic-pentagonal (also type 1, different pentagon); Python audit 31/31 before porting; 9-test in-module suite (see CHANGELOG M117).

## Per-tiling documents

Each member's full specification — algorithm, exact geometry, verification, sources — lives in its own document under [docs/tilings/](../tilings/):

- [`pentagon-13`](../tilings/pentagon-13.md) — Pentagon type 13 (Rice, 1977)
- [`pentagon-14`](../tilings/pentagon-14.md) — Pentagon type 14 (Stein)
- [`pentagon-15`](../tilings/pentagon-15.md) — Pentagon type 15 (Mann)
- [`pentagon-9`](../tilings/pentagon-9.md) — Pentagon type 9 (Rice)
- [`pentagon-11`](../tilings/pentagon-11.md) — Pentagon type 11 (Rice)
- [`pentagon-12`](../tilings/pentagon-12.md) — Pentagon type 12 (Rice)
- [`pentagon-10`](../tilings/pentagon-10.md) — Pentagon type 10 (James)
- [`pentagon-3`](../tilings/pentagon-3.md) — Pentagon type 3 (Reinhardt)
- [`pentagon-6`](../tilings/pentagon-6.md) — Pentagon type 6 (Kershner)
- [`pentagon-8`](../tilings/pentagon-8.md) — Pentagon type 8 (Kershner)
- [`pentagon-1`](../tilings/pentagon-1.md) — Pentagon type 1 (Reinhardt)
- [`pentagon-2`](../tilings/pentagon-2.md) — Pentagon type 2 (Reinhardt)
- [`pentagon-7`](../tilings/pentagon-7.md) — Pentagon type 7 (Kershner)
- [`hirschhorn`](../tilings/hirschhorn.md) — Hirschhorn pentagon

---

*Map: [docs/INDEX.md](../INDEX.md) · Changelog: [docs/history/CHANGELOG.md](../history/CHANGELOG.md) · Queue: [docs/ROADMAP.md](../ROADMAP.md)*
