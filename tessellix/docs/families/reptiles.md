# Rep-tiles family

Rep-tile substitutions (the chair family, sphinx, P-pentomino), each implementing `Tiling` directly with a dissection forced by exhaustive enumeration.

Code: `crates/tess-tilings/src/families/reptiles/` — cargo feature `reptiles` (default-on).
Adding a tiling here touches only this family: `families/reptiles.rs`
(module decl, register slice, classification arms, census `IDS`,
`lattice_tilings` entries) plus the new module and its doc row below.
Work briefs: `docs/briefs/TEMPLATE.md`; fleet protocol: `tessellix/AGENTS.md`.

## Implemented (9 tilings)

### Rep-tiles (nonperiodic / limit-periodic)

| id | motif | notes |
|---|---|---|
| `chair` | Chair (L-tromino) | λ = 2 rep-4 substitution; the inflated chair's dissection is unique (exhaustive enumeration) so the rule is forced; non-edge-to-edge (T-junctions at every scale); limit-periodic; 4×3 seed blocks, one chair per orientation; octave-anchored like pinwheel |
| `armchair` | Armchair (L-tetromino rep-4) | the chair-family line's L-tetromino member (2×3 bbox, area 4): the scale-2 inflated armchair has a unique L-tetromino dissection (exhaustive exact-integer enumeration — one child of each of the four orientations, rule forced); non-edge-to-edge (the canonical side-2 parent carries a T-junction at (2,2); T-junction counts 32/144/656/2736 per generation); limit-periodic — the depth-g block's only translation period is (2^(g+1), 2^(g+1)), measured on the integer grid; 4×4 seed block, one armchair per orientation; octave-anchored like chair (M128) |
| `sphinx` | Sphinx (pentagonal hexiamond) | λ = 2 rep-4 substitution on the triangular lattice; of 12 hexiamonds only the 3×1 rhombus and the sphinx are rep-4, and the sphinx's dissection is unique (rule forced); 12 orientation classes (dissection mixes chiralities), doubly stochastic substitution matrix; non-edge-to-edge; 2×3 rhombus seed (exactly 2 sphinxes, unique); octave-anchored like chair |
| `p-pentomino` | P-pentomino (rep-4 reptile) | λ = 2 rep-4 substitution; exactly two dissections of the inflated 20-cell P (exhaustive enumeration, class multisets {0,1,4,6} and {3,4,4,5}), canonical {0,1,4,6} used; 8 chiral orientation classes, doubly stochastic substitution matrix (rows/columns sum to 4); non-edge-to-edge (5 T-junctions per parent); 5×8 seed block, one tile per class (62 such tilings of the rectangle); octave-anchored like chair (M47) |
| `pregnant-chair` | Pregnant chairs (large + small) | λ = √3 two-prototile substitution (Bielefeld substitution index entry, E. Amezcua): the √3-inflated large chair (area 9√3) is tiled by ONE large + SIX small chairs, and the √3-inflated small chair (area 3√3) is exactly one large chair (area conserved: 3√3·√3 = 9√3); rule pinned (C1 = x-mirror of the published Bielefeld figure), non-edge-to-edge (11 T-junctions in the 14-tile depth-1 seed block); limit-periodic — the depth-g block period √3^g·(9, 0) × √3^g·(0, 2√3) grows without bound; 9×2√3 two-chair seed block (two large chairs) on the origin-anchored period lattice; chiral orientation census — m ∈ {0, 2, 6, 7} at every depth ≥ 1, the four other dihedral classes never occur; per-generation (L, S) leaf-count law (L, S) → (L + S, 6L): single seed 7 → 13 → 55 → 133 at depths 1/2/3/4; leaves are always unit scale, so depth changes the L/S mix, not tile size; octave-anchored like armchair (M128) |
| `tangram` | Tangram (classic five-piece dissection) | λ = √2 five-prototile substitution (two small triangles, medium triangle, large triangle, square, parallelogram — Bielefeld substitution index entry): S → {M}; M → {L}; L → {M, Q, P, S×2}; Q → {M, S×2}; P → {M, S×2} — area-conserving (u² areas S 1, M 2, L 4, Q 2, P 2); pinned seed det=2: two S triangles tile a √2×√2 square (second S 180°-rotated), origin-anchored √2^(g+1) block lattice, depth quantized to √2 octaves from the view extent so final tiles always land at natural unit scale; homochiral — P is pinned to the P+ (up-LEFT-slant) class, P− unreachable from the seed (every substitution row is rotation-only); non-edge-to-edge — T-junctions 0, 0, 0, 6, 10, 18 over generations 0–5; limit-periodic; exact ℤ[√2] u-pair arithmetic end to end, floats only at emission; 12 Rust tests (exact substitution census for all prototile×twist, exact T-junction census, growth count/area law, chirality, octave view-stability, unit-scale emission) plus the standard trio |
| `octiamond-reptile` | Octiamond reptile (trapezoidal 8-triomino) | λ = 3 rep-9 substitution on the triangular lattice (Bielefeld substitution index entry, "Folklore" — Info section EMPTY, no references; rule reconstructed from the rule image and pinned by a UNIQUE exhaustive partition of the 3×-inflated 72-cell seed trapezoid into 9 unit octiamonds, cross-confirmed per-component against the Bielefeld rule image, scores 0.911–0.991 vs runners-up ≤ 0.892; the blue-marked child is I+(2,2)); 1 prototile Q = trapezoid (0,0),(3,0),(2,√3),(1,√3) (8 unit triangles, area 2√3); supertile TRAP3Q (72 cells), PF = 9 = λ²; substitution matrix [9] → 9^n tiles per supertile; dissection mixes chirality (children: 4×I, R120, R180, R240, R300, M4 — all 12 orientation classes reachable from the seed; 12×12 class matrix, rows and columns sum to 9); non-edge-to-edge — T-junctions inherent (measured census, sphinx-validated machinery: 8 (edge, foreign corner) incidences in the rule's own depth-1 dissection; 18/226/2202/20286/183974 over seed generations 0–4); 12×6 parallelogram seed block = TRAP3Q ∪ R180 image (18 tiles, 144 cells), tiles the plane under ⟨(12,0),(0,6)⟩; L2 nesting verified under the λ-scaled framing (0 overlaps, 648/648, 81 distinct grandchildren, framing cross-validated on the sphinx rule); the Bielefeld patch's 1142 9-tiled trapezoid panels form 6 table types (frequencies 259/196/186/183/162/156, one per conjugacy orbit of 12), all L1+L2-clean — iterable substitution structure in the patch; aperiodic (no lattice period in any of the 4 patch crops); tri-section-anchored depth quantization, unit-scale emission (M150) |
| `pentiamond-ac-factor-2` | Pentiamonds A + C (2-prototile rep-4, triangular lattice) | Bielefeld substitution index entry "Pentiamond AC factor 2" (discovered: folklore; Info empty — rule image the sole source): λ = 2, A → 3A + 1C, C → 2A + 2C with the achiral C double-labeled (mirror dissections T1 = 8·T0 + (0,6) under pose pairs {g, g∘8}; canonical pin = table 0 for all poses, frame probe 108/108 + label invariance 24/24); M = [[3,2],[1,2]] char poly (x−1)(x−4), PF 4 = λ², primitive index 1, per-A-seed closed form A_d = (2·4^d+1)/3; 3×5 rectangular seed block (4 A poses 3/4/6/9 + 2 C poses 1/5), whole-block homothety on the (3·2^d, 0) × (0, 5·2^d) block lattice; limit-periodic construction, aperiodic limit — zero exact periods in the unit tiling (|v| ≤ 16, top near-miss 0.6953 < 1); chiral A balanced 1:1 at every depth; folkloric 9945-tile patch reproduced exactly by the pinned rule (recon E PASS, coverage 1.0); exact integer cell arithmetic, floats only at emission |
| `pentiamond-ac-factor-3` | Pentiamonds A + C (2-prototile rep-9, triangular lattice) | Bielefeld substitution index entry "Pentiamond AC factor 3" (discovered: folklore; Info empty; header says 3 prototiles but the **D row is absent from the rule image and the patch** — blocker, implemented as 2 prototiles, flagged): λ = 3, A → 6A + 3C, C → 5A + 4C (achiral C double-labeled, mirror pair via (8, (0,9)); canonical pin = table 0 for all poses, frame probe 108/108 + label invariance 24/24); A/C block [[6,5],[3,4]] char poly (x−9)(x−1), PF 9 = λ², primitive index 1 (no sum-9 D column makes the 3×3 primitive — 55-candidate scan), per-A-seed closed form A_d = (5·9^d+3)/8; 2×5 seed block (A poses 4,7 both P2 + C poses 1,4; block charge −2 — the folklore patch's P2-heavy 617:1899 depth-2 census, decaying to 1:1 in the limit, measured 0.9999 by d = 20); whole-block homothety on the (2·3^d, 0) × (0, 5·3^d) block lattice; limit-periodic construction, aperiodic limit — zero exact periods in the unit tiling (|v| ≤ 12, top near-miss 0.5712); folkloric 4024-tile patch reproduced exactly (recon E PASS, coverage 1.0); the A/C block is closed and the seed D-free ⇒ D count 0 at every depth regardless of the missing D row; exact integer cell arithmetic, floats only at emission |

## Build arcs

Numbered arcs keep their PATTERNS.MD §numbers — code comments and
briefs cite them as `§N`.

> Follow-up: chair tiling — ✅ complete (M30)
>
> Pulled from the proposed queue (a classic, and the pedagogical
> monohedral contrast to the pinwheel). One commit with the standard
> battery.


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


> Follow-up: rep-tile sprint — 🚧 in progress (M32+)
>
> The monohedral substitution track after the chair (M30): more rep-tiles
> reusing the exact-integer playbook (derive the rule computationally,
> verify in exact Python before Rust, octave-anchored generation).


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


---

*Map: [docs/INDEX.md](../INDEX.md) · Changelog: [docs/history/CHANGELOG.md](../history/CHANGELOG.md) · Queue: [docs/ROADMAP.md](../ROADMAP.md)*
