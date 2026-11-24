# Brief: pentiamond-set — three Bielefeld pentiamond reptile tilings (AC factor 2, AC factor 3, AD factor 2)

> **LANDED as M155–M157** (see docs/history/CHANGELOG.md). Do not dispatch.

> **Category note:** these land in the **`reptiles` family (Nonperiodic tab)**
> — NOT the aperiodic family. All existing reptiles declare
> `families: &["Nonperiodic"]` (verified in `families/reptiles.rs`,
> 2026-11-22). If your evidence flips this, STOP and report first.
>
> **Scope note:** ONE brief covers THREE tilings in one landing: reptiles
> census 6 → 9 (three registry ids, three classification arms, three
> family test trios, three doc rows).
>
> **Base census (main `8d09364`):** reptiles 6 / Nonperiodic tab 7
> (last element of the per-tab vec in `lib.rs`) / built-ins 137 /
> memberships 151. Expected after this landing: 9 / 10 / 140 / 154 — but
> other landings may move totals while you work; report exactly what your
> guards printed.
>
> **ac-f3 lands as 2 prototiles {A, C} although the entry header says
> "3 Prototiles":** the D rule row is ABSENT from the rule image and there
> are zero D parent regions in the patch — an evidence blocker, not a
> tooling failure. See "ac-f3 D row — blocker". The limit tiling is
> D-free regardless (A/C block closed, seed D-free ⇒ D count 0 at every
> depth), so the implementation is complete as a 2-tile tiling; flag the
> discrepancy in the report.
>
> **M numbers:** this brief contains NO M numbers; the orchestrator
> assigns them at integration. No (M#) placeholders are used anywhere in
> this document.

**Family:** `reptiles` · **Candidate:** PROPOSED.md §23d, line 1507:
"- **Pentiamond / small rep-tile set** — effort S each." · **Effort:** M
(three S-sized tilings in one brief) · **Precedent to mirror:**
`families/reptiles/sphinx.rs` (read it ALL first: `SYMS` block L85–98,
the `Sphinx` struct, `seed_sphinxes` L310, `deflate_step` L351 with the
soundness comment L346–350, and `generate()` L387–476 — `SymmetryMeta`
L388, `fail` closure L390–395, inverse-map region L400–412, area guard
L421, depth formula L440, seed+deflate loop L442–445, emission L454–468)
and `families/reptiles.rs` (the tangram classification arm L43–55 — the
pattern for a Bielefeld-sourced multi-prototile entry, and the test
trios L102–141 — the armchair pattern to copy). Also read
`families/reptiles/p_pentomino.rs`'s header comment (how a chiral-mixing
substitution is documented) and `docs/families/reptiles.md` (row style).

### Goal

Land three substitution tilings in the `reptiles` family under registry
ids `pentiamond-ac-factor-2`, `pentiamond-ac-factor-3`,
`pentiamond-ad-factor-2` (slugs mirror the Bielefeld entry slugs; "pentiamond"
currently has ZERO occurrences in `crates/`, `docs/families/`, `README.md`,
`docs/INDEX.md` — verified by grep 2026-11-22 at base `8d09364`), each
classified tangram-style: families `["Nonperiodic"]`, method
"substitution", inflation "λ = 2" or "λ = 3", prototiles 2 (ac-f3: 2
implemented of the entry's 3 — see blocker), translational "aperiodic",
source per the tangram pattern, unless your battery says otherwise.

Headline properties (all measured in recon — the evidence chain below is
the proof):

1. The rules were reconstructed **from the rule/patch images alone**
   (folklore entries, Info section empty on all three pages) and
   **reconstruct the entry's own patch PDFs exactly** — s06 E PASS:
   9945 / 4024 / 9969 unit tiles, cell coverage exactly 1.000000, zero
   bad tiles, for all three entries.
2. **Aperiodic limit** (zero exact translation periods at every tested
   level: patch window s05, level-2 patch s09, unit-level deflated block
   s14-F) via a **limit-periodic construction**: sphinx-style
   whole-block lattice seeding, period lattice
   ⟨(Wλ^d, 0), (0, Hλ^d)⟩ at depth d, periods grow without bound.
3. ac-f3 carries a **documented blocker** (D row unpinnable from the
   images) — implement the 2-prototile tiling and flag it.

### Ground truth: shapes, letters, lattice (pinned s00–s01)

The three prototile shapes are free pentiamonds (OEIS A000577, n = 5:
12 free pentiamonds); the set uses exactly three of them:

| letter | identity | chirality | canonical cell set (kind, q, r) | v0 (lex-min vertex) |
|---|---|---|---|---|
| **A** | P1 | **chiral**, \|stab\| = 1 (stab(A) = [9]) | `((0,0,0),(0,0,1),(0,1,0),(1,0,0),(1,0,1))` | (0, 0) |
| **C** | P3 | achiral, \|stab\| = 2 (stab = [8,9]) | `((0,0,1),(0,1,0),(0,1,1),(1,0,0),(1,0,1))` | (0, 1) |
| **D** | P0 | achiral, \|stab\| = 2 (stab = [8,9]) | `((0,0,0),(0,0,1),(0,0,2),(1,0,0),(1,0,1))` | (0, 0) |

The mirror of A (P2) is **unused by all three entries**. Letter identities
were pinned by matching the entry patch PDFs' fill-shape censuses
(s01): every fill of each color is one of exactly these shapes —
ac-f2 {A, C}, ac-f3 {A, C} (no D anywhere in the patch), ad-f2 {A, D}.

Lattice conventions (match `sphinx.rs` and recon's `tri.py`):

- Unit triangles on the hexagonal lattice: **UP(q, r)** = cells
  {(q,r), (q+1,r), (q,r+1)} (kind 0); **DN(q, r)** = {(q+1,r), (q,r+1),
  (q+1,r+1)} (kind 1). Cell tuples are `(kind, q, r)`; kind 0 = UP,
  1 = DN.
- World map: lattice point (q, r) → world **(q + r/2, (√3/2)·r)**
  (sphinx.rs `corners()` pattern; world x = q + r/2, world y = H·r with
  H = √3/2). A pentiamond is 5 unit triangles = 10 half-cells… i.e. its
  cell set has exactly 5 `(kind, q, r)` entries.
- Metric: f(Δq, Δr) = Δq² + ΔqΔr + Δr².
- **SYMS**: the 12 triangular-lattice symmetries, row-major [a, b, c, d],
  byte-identical to `sphinx.rs` L85–98: index **9 = identity** (index 0 is
  NOT), index **8 = the mirror** used for achiral pose pairs.
  `compose(h, k)` applies k first, then h. `det_sym(s) = a·d − b·c`;
  **P1 (rotation) class = det +1 = poses {1,2,5,6,9,10}**, P2 = det −1.
- **Pose labels:** a chiral letter (A) tile carries one of 12 poses
  (unique); an achiral letter (C, D) tile's 12 poses collapse to 6
  geometry classes as pairs {g, compose(g, 8)} — the two labels of a pair
  give pure translations of each other's posed cell set (verified s14-C),
  so a deterministic rule uses **g0 = the min-index pose of the class**.
  All 12 pose labels of every letter occur in each entry's patch
  (s02 `gseen`, 12/12 for A and for C/D).
- **Canonical outlines** (CCW, collinear steps merged, in (q, r) coords;
  mirrored/det −1 poses emit the REVERSED outline for CCW, per
  `sphinx.rs` `corners()`):
  - A: 5-gon `[(0,0),(2,0),(1,1),(1,2),(0,2)]` — squared edge lengths
    [4,1,1,1,4] (sides 2,1,1,1,2; s01 outline cycle (2,1,1,1,2)).
  - C: 7-gon `[(0,1),(1,0),(2,0),(1,1),(2,1),(1,2),(0,2)]` — all unit
    (s01 cycle (1,1,1,1,1,1,1)).
  - D: 4-gon `[(0,0),(1,0),(1,2),(0,3)]` — squared [1,4,1,9] (sides
    1,2,1,3; s01 cycle (3,1,2,1) rotated).
- Tile spans: A and C each fit a 2×2 cell square; D is a 1×3 strip
  (2 of its 12 poses).

### Literature basis (rule pinned from the images alone, recon 2026-11-22)

**Source:** the Bielefeld Tilings Encyclopedia substitution index
(`https://tilings.math.uni-bielefeld.de/substitution/`), three entries —
"Discovered by: Folklore" on every page, **Info section EMPTY** (no tags,
no references, no text — verified by parsing each page's HTML), rule PNG +
patch PNG + patch PDF (vector format) on each:

| entry | URL | prototiles (header) | λ | patch H (px) | patch tiles | patch cells |
|---|---|---|---|---|---|---|
| pentiamond-ac-factor-2 | `https://tilings.math.uni-bielefeld.de/substitution/pentiamond-ac-factor-2/` | 2 (A, C) | 2 | 3142.069336 | 9945 (A 6632, C 3313) | 49725 |
| pentiamond-ac-factor-3 | `https://tilings.math.uni-bielefeld.de/substitution/pentiamond-ac-factor-3/` | 3 (A, C, D) | 3 | 1988.279541 | 4024 (A 2516, C 1508) | 20120 |
| pentiamond-ad-factor-2 | `https://tilings.math.uni-bielefeld.de/substitution/pentiamond-ad-factor-2/` | 2 (A, D) | 2 | 3142.069336 | 9969 (A 4982, D 4987) | 49845 |

The adjacent entry "Octiamond Reptile" is **out of scope** — queue note
only, do not touch it.

**Evidence chain (all programmatic, stdlib-only Python, ≤240 s per
experiment; outputs in `/tmp/opencode/pentiamond-recon/`, full log in its
`NOTES.md`, 616 lines):**

1. **s01 — patch decode.** Each patch PDF: fills → 5-cell shapes
   (s01 run: every fill exactly 5 cells; 1 connected component each;
   cell coverage exactly-once: 49725 / 20120 / 49845; per-color shape
   census pins the letter↔shape identity above; worst geometric
   consistency rel err ≤ 0.00714). Patch windows (q, r):
   ac-f2 q[−128..127] r[−62..67]; ac-f3 q[−135..26] r[0..82];
   ad-f2 q[−73..182] r[−87..42].
2. **s02 — rule decode (source of truth: `s02_rules.pkl`; verbatim dump
   `brief_rules_dump.txt`).** Rule PNG rows decoded to child sets;
   normalized by applying INV[g] to absolute children and re-anchoring
   each table's union at mqr (0,0). Self-check: every table tiles the
   canonical λ-scaled parent region EXACTLY (disjoint, union == region,
   |T| = λ²). All 12 g-labels covered per shape; 105 / 136 / 109 parent
   regions excluded as crop-boundary orphans (incomplete children at the
   patch edge). Table copy counts per parent letter (number of patch
   parents seen in each dissection): ac-f2 A [1638], C [822, 822];
   ac-f3 A [267], C [165, 165]; ad-f2 A [1234], D [2462]. **ac-f3 D:
   `n_parents = 0` — bare marker, no `all_tables`.**
3. **s04c — per-row image match (independent confirmation).** Each rule
   row's parent figure matched against posed, λ-scaled canonical parents:
   `pixel = t + u·embed(s,(q,r))` with u ≈ 31.5 ± 1 px (measured ranges
   29.80–33.40). Per row: the matched pose, the matched child multiset,
   and the matching s02 table:
   - ac-f2 A row: pose **10**, {A:3, C:1}, 67 passing frames, 1 table = s02 table 0.
   - ac-f2 C row: poses **4 and 10** (two dissections), {A:2, C:2}, 134 frames, 2 tables = s02 table 1 (pose 4) + table 0 (pose 10).
   - ac-f3 A row: pose **10**, {A:6, C:3}, 40 frames, 1 table = s02 table 0.
   - ac-f3 C row: poses **4 and 10**, {A:5, C:4}, 72 frames, 2 tables = s02 table 1 (pose 4) + table 0 (pose 10).
   - **ac-f3: "MISSING ROWS (no rule row in the image): ['D']"** — zero D pixels in the rule image; only 2 parent-row components exist.
   - ad-f2 A row: pose **10**, {A:2, D:2}, 66 frames, 1 table = s02 table 0.
   - ad-f2 D row: poses **4 and 10**, {A:2, D:2}, 96 frames, **1 distinct table** (self-mirror; dedup n = 1).
4. **s03 — eigenstructure.** Matrices, char polys, PF, primitive index,
   patch-count vs PF-share cross-checks (details below).
5. **s06 — full patch reconstruction.** Seeded from the patch's own
   (undisclosed) multi-level seed block and deflated with the pinned
   rules: **E PASS on all three** — 9945 / 4024 / 9969 unit tiles
   (== patch census exactly), Sreg == patch cell set, cell-sum ==
   window area, coverage = 1.000000, bad_a = bad_b = 0. Folklore seed
   block structure: ac-f2 2565 tiles {L1: 105, L2: 2460}, dmax = 2;
   ac-f3 568 {L1: 136, L2: 432}, dmax = 2; ad-f2 1425 {L1: 109, L2: 933,
   L3: 383}, dmax = 3. This is the strongest pin: the rules reproduce the
   published patch to the last cell.
6. **s06diag + s09 — what the folklore patch IS (and is not).**
   s06diag (per unique level-3 candidate region, classify first sibling
   failure): ac-f2 9040 regions → cells_bad 8530 / cells_ok 510 /
   TRUE 0; ac-f3 3569 → 3528 / 41 / 0; ad-f2 8831 → 8230 / 520 (incl.
   81 TRUE, all P=A) / pair-fail 0. ⇒ The 2→3 no-merge for ac-f2/ac-f3
   is GENUINE, not a pose-check artifact: geometric level-3 slots exist
   (510/41 fit shape+letter) but the chiral-A children carry the WRONG
   orientation, rejected by the verified pose dynamics (compose(g,s0)
   exact on 13128/5373 pairs + E reconstruction bad 0/0); ad-f2 has
   real level-3 nesting (383 merged parents, dmax = 3). Level-2 period
   census (s09): **zero exact level-2 periods in all three patches**
   (ac-f2: 2460 L2 tiles, 2041 interior, near-miss tops v = (±48,∓48)
   0.3033/0.2978, (±32,0) 0.215–0.218; ac-f3: 432 L2, 284 interior, tops
   (±9,±18) 0.1629/0.1526 — noise; ad-f2: 933 L2, 701 interior, tops
   (±32,∓32) 0.1696/0.1611 — noise). ⇒ The folklore patch is a
   **depth-2 construction (depth-3 for ad-f2), NOT a fixed-point
   window** — the published patch does not exhibit the rule's fixed
   point, so it cannot be used as a view window.
7. **s05 — patch-window period census.** Letter-exact same-letter match
   over interior cells, full v-grid: **zero exact periods** (every v
   vacuous: interior < 500 — max interior 10 / 130 / 66); baselines
   P(same) = 0.5557 / 0.5314 / 0.5000; top near-misses ac-f2
   v = (±8,±8) 0.6180, (±8,0) 0.6095, (2,−4) 0.6062; ac-f3 (0,±3) 0.5759,
   (3,−2) 0.5740; ad-f2 (1,0) 0.6094, (0,1) 0.5962, (1,−1) 0.5944.
8. **s07 — single-tile containment (geometric fact, NOT generation).**
   Smallest single scaled tile containing the whole patch, verified 100%:
   ac-f2 level 8 (m = 128) **A pose 7 at t = (−128, −62)**;
   ac-f3 level 5 (m = 81) **A pose 7 at t = (−135, 0)**;
   ad-f2 level 8 (m = 128) **A pose 7 at t = (−73, −87)**; one level up,
   ac-f2 and ac-f3 each have 18/18 (letter, pose, t) containments
   (12 A poses + 6 C/D poses); ad-f2 has 16/18 (2 span-rejected).
   Combined with s06/s06diag/s09: the patch is contained in
   one large tile but is NOT single-tile-generated.
9. **s08b — seed-block negative (why the seed is DESIGNED).** No
   axis-aligned m×n cell rectangle with m, n ≤ 20 (m·n % 5 == 0, m·n
   ≥ 10) is a union of complete tiles in any of the three patches
   (straddle == 0 AND full coverage; brute-force cross-checked). ⇒ the
   generation seed block must be designed, not cut from the patch.
10. **s11/s12/s13 — seed blocks designed** (exact-cover search over the
    12 poses, with a chirality-charge matching constraint): the pinned
    blocks below; "coverage check: union == full rectangle -> True" for
    all three (both triangle kinds). Search provenance: ac-f2 3×5: 24
    solutions, 12 charge-matching (pinned = first); ac-f3 2×5: 8
    solutions, 8 charge-matching, 2 distinct physical blocks (pinned =
    first); ad-f2 1×5: 4 solutions, 2 distinct physical (unique).
11. **s14 — generation verification (ALL CHECKS PASS, 4.6 s full run).**
    See Battery spec §1–§8 for the per-entry pins.
12. **s14b — chirality dynamics.** Pose-dynamics recursion depths 0..20,
    letter counts == closed forms at every depth (asserted): ac-f2 A
    P1:P2 = exactly 1:1 at every depth; ac-f3 5:17 (d=1) → 68:134 (d=2) →
    731:1091 (d=3) → 0.7863 (d=4) → 0.9807 (d=9) → 0.9999 (d=20) —
    **converges to 1:1; the folklore patch's P2 bias (617:1899 ≈ 1:3.08)
    is a finite-depth artifact of their depth-2 construction, NOT a rule
    property**; ad-f2 exactly 1:1 at every depth.

### Pinned rule tables (verbatim from `brief_rules_dump.txt`)

Conventions: cell tuples `(kind, q, r)`, kind 0 = UP / 1 = DN. A table T
is a list of (Lj, Fj) pairs — Lj the child letter, Fj the child's cell
set in the **parent's un-posed frame, anchored at the union's mqr (0,0)**;
|T| = λ²; children partition the λ-scaled canonical parent region
exactly (s02 self-check). `mqr(S)` = (min q, min r) over cells, taken
separately per coordinate.

**pentiamond-ac-factor-2 (λ = 2).** A row: 1 table (1638 parent copies),
{A:3, C:1}:

```
A ((0,0,0),(0,0,1),(0,1,0),(1,0,0),(1,0,1))
A ((0,0,3),(0,1,3),(1,0,3),(1,1,2),(1,1,3))
A ((0,2,0),(0,2,1),(0,3,0),(1,1,0),(1,2,0))
C ((0,0,2),(0,1,1),(0,1,2),(1,0,2),(1,1,1))
```

C row: **2 tables** (822 + 822 copies; ONE rule under the achiral label
pair {g, g∘8}; table 1 = mirror 8·(table 0) + (0,6) in the identity
frame), {A:2, C:2}:

```
table 0:
A ((0,2,0),(0,2,1),(0,3,0),(1,1,0),(1,2,0))
A ((0,2,2),(0,2,3),(0,3,2),(1,1,3),(1,2,2))
C ((0,0,2),(0,1,1),(1,0,1),(1,0,2),(1,1,1))
C ((0,0,3),(0,1,2),(0,1,3),(1,0,3),(1,1,2))
table 1:
A ((0,2,0),(0,2,1),(0,3,0),(1,1,0),(1,2,0))
A ((0,2,2),(0,2,3),(0,3,2),(1,1,3),(1,2,2))
C ((0,0,2),(0,1,1),(0,1,2),(1,0,1),(1,1,1))
C ((0,0,3),(0,1,3),(1,0,2),(1,0,3),(1,1,2))
```

**pentiamond-ac-factor-3 (λ = 3).** A row: 1 table (267 parent copies),
{A:6, C:3}:

```
A ((0,0,0),(0,0,1),(0,1,0),(1,0,0),(1,0,1))
A ((0,0,3),(0,0,4),(0,1,3),(1,0,3),(1,1,2))
A ((0,1,4),(0,2,3),(1,1,3),(1,2,2),(1,2,3))
A ((0,2,0),(0,2,1),(0,3,0),(1,1,0),(1,2,0))
A ((0,2,4),(0,2,5),(1,1,5),(1,2,4),(1,2,5))
A ((0,4,0),(0,4,1),(0,5,0),(1,3,0),(1,4,0))
C ((0,0,2),(0,1,1),(0,1,2),(1,0,2),(1,1,1))
C ((0,0,5),(0,1,5),(1,0,4),(1,0,5),(1,1,4))
C ((0,2,2),(0,3,1),(0,3,2),(1,2,1),(1,3,1))
```

C row: **2 tables** (165 + 165; mirror pair via (8, (0,9))), {A:5, C:4}:

```
table 0:
A ((0,0,3),(0,1,2),(1,0,2),(1,1,1),(1,1,2))
A ((0,2,2),(0,2,3),(0,3,2),(1,2,2),(1,3,1))
A ((0,2,5),(0,3,5),(1,1,5),(1,2,4),(1,2,5))
A ((0,4,0),(0,4,1),(0,5,0),(1,3,0),(1,4,0))
A ((0,4,3),(0,4,4),(0,5,3),(1,3,4),(1,4,3))
C ((0,0,4),(0,1,3),(0,1,4),(1,0,3),(1,1,3))
C ((0,0,5),(0,1,5),(1,0,4),(1,0,5),(1,1,4))
C ((0,2,1),(0,3,0),(0,3,1),(1,2,0),(1,2,1))
C ((0,2,4),(0,3,3),(0,3,4),(1,2,3),(1,3,3))
table 1:
A ((0,0,5),(0,1,5),(1,0,5),(1,1,4),(1,1,5))
A ((0,2,1),(0,3,0),(1,1,1),(1,2,0),(1,2,1))
A ((0,2,3),(0,2,4),(0,3,3),(1,2,3),(1,3,3))
A ((0,4,0),(0,4,1),(0,5,0),(1,3,0),(1,4,0))
A ((0,4,3),(0,4,4),(0,5,3),(1,3,4),(1,4,3))
C ((0,0,3),(0,1,2),(1,0,2),(1,0,3),(1,1,2))
C ((0,0,4),(0,1,3),(0,1,4),(1,0,4),(1,1,3))
C ((0,2,2),(0,3,1),(0,3,2),(1,2,2),(1,3,1))
C ((0,2,5),(0,3,4),(0,3,5),(1,2,4),(1,2,5))
```

D row: **ABSENT** — `n_parents = 0` (bare marker `{'lam': 3,
'n_parents': 0}`, no tables). See blocker section.

**pentiamond-ad-factor-2 (λ = 2).** A row: 1 table (1234 parent copies),
{A:2, D:2}:

```
A ((0,0,1),(0,0,2),(0,1,1),(1,0,1),(1,0,2))
A ((0,0,3),(0,1,3),(1,0,3),(1,1,2),(1,1,3))
D ((0,0,0),(0,1,0),(0,2,0),(1,0,0),(1,1,0))
D ((0,1,2),(0,2,1),(0,3,0),(1,1,1),(1,2,0))
```

D row: **1 table** (2462 copies, **self-mirror** — the D dissection is
invariant under the achiral label pair), {A:2, D:2}:

```
A ((0,0,0),(0,0,1),(0,1,0),(1,0,0),(1,1,0))
A ((0,0,4),(0,0,5),(0,1,4),(1,0,4),(1,1,3))
D ((0,0,2),(0,0,3),(1,0,1),(1,0,2),(1,0,3))
D ((0,1,1),(0,1,2),(0,1,3),(1,1,1),(1,1,2))
```

**The double-table mechanism (why C has two tables and the rule is still
deterministic):** an achiral parent region (C or D) matches TWO poses
{g, g∘8} whose posed cell sets are pure translations of each other. Each
physical parent in the patch is counted twice — labeled (g, T0) and
(g∘8, T1) — and the two dissections are mirror-related:
T1 = 8·(T0) + t (t = (0,6) ac-f2 C, t = (0,9) ac-f3 C; ad-f2 D is
self-mirror, one table). The folklore patch uses BOTH tables (s06 B:
"C tables per pose: 0,1" for all 12 poses — both mirror dissections
appear in the patch), while the rule image shows ONE (pose 4 → table 1,
pose 10 → table 0). **Pinned canonical resolution (s14):** the
implementation uses **table index 0 for ALL poses of achiral letters**.
This is validated by (a) the frame probe: T0 tiles the posed region
R0(g) exactly for EVERY g (108/108 and 96/96 combos, s14-B), and (b)
label invariance: for the same physical region, children placed under
(g, T0) == children under (g∘8, T1) as cell-set multisets — 24/24
combos (ac-f2 C, ac-f3 C: 12 poses × cross-assignments (0,1),(1,0)) and
12/12 (ad-f2 D: 12 poses × (0,0)) (s14-C). The second dissection is
documented as a known alternative the folklore patch also uses; it is not
reconstructable from the rule image alone and is NOT implemented.

### Eigenstructure (exact; s03 + s14b)

Substitution matrices are child × parent (columns = parents, rows =
children), column sums = λ² (area law), verified.

**ac-f2** (order A, C): `M = [[3, 2], [1, 2]]`; char poly (x−1)(x−4) =
x² − 5x + 4, roots {1, 4}; **PF = 4 = λ²; primitive index 1**; PF
eigenvector (right) (2, 1), left (1, 1). Patch share A : C = 6632 : 3313
vs PF share 2 : 1 (diff 0.000201 — patch noise, not a rule defect).
Closed forms (per seed tile, d deflations): per A seed
A_d = (2·4^d + 1)/3, C_d = (4^d − 1)/3; per C seed
A_d = (2·4^d − 2)/3, C_d = (4^d + 2)/3. (d = 1: per A (3,1), per C (2,2);
d = 2: (11,5), (10,6).) Verified by s14b recursion depths 0..20.

**ac-f3** (order A, C; D open): A/C block
`[[6, 5], [3, 4]]`; char poly (x−9)(x−1) = x² − 10x + 9, roots {1, 9};
**PF = 9 = λ²; 2×2 block primitive index 1** (verified by direct
computation). The 3×3 placeholder with D column (0,0,9),
`[[6,5,0],[3,4,0],[0,0,9]]`, is **NOT primitive** (index None; char poly
x³ − 19x² + 99x − 81, roots {1, 9}); s03 scanned all 55 D columns
(dA,dC,dD) with dA+dC+dD = 9: every one keeps PF = 9, but **none makes
the full 3×3 primitive**. PF eigenvector (5, 3), left (1, 1), wᵀv = 8.
Patch share A : C = 2516 : 1508 ≈ 5 : 3 (diff 0.000249). Closed forms:
per A seed A_d = (5·9^d + 3)/8, C_d = (3·9^d − 3)/8; per C seed
A_d = (5·9^d − 5)/8, C_d = (3·9^d + 5)/8. (d = 1: (6,3), (5,4);
d = 2: (51,30), (50,31).) **The A/C block is closed (no A/C parent emits
D) and the seed block is D-free ⇒ the count of D tiles is 0 at every
depth, whatever the missing D column is** — the D row cannot affect the
limit tiling.

**ad-f2** (order A, D): `M = [[2, 2], [2, 2]]`; char poly x² − 4x =
x(x − 4), roots {0, 4}; **PF = 4 = λ²; primitive index 1**; PF
eigenvector (1, 1) (left (1, 1) too). M^d = 4^{d−1}·M for d ≥ 1: per
seed tile of either letter, (A_d, D_d) = (2·4^{d−1}, 2·4^{d−1}).
(d = 1: (2,2); d = 2: (8,8); d = 4: (128,128).) Patch share 4982 : 4987
≈ 1 : 1 (diff 0.000251).

### Aperiodicity (measured; never asserted)

- **Patch window** (s05): zero exact periods (vacuous — interiors < 500),
  baselines 0.5557 / 0.5314 / 0.5000, top near-misses as listed in
  evidence chain step 7.
- **Level-2 patch** (s09): zero exact level-2 periods in all three
  patches ⇒ the folklore patch is a finite depth-2/depth-3 construction,
  **not a fixed-point window** (it cannot serve as a view window for the
  limit tiling).
- **Unit-level deflated block** (s14-F): zero exact periods for |v| ≤
  16 / 12 / 10 respectively (exact = 100 % same-letter match on ≥ MININT
  interior cells, MININT 200 / 200 / 120); near-miss tops:
  - ac-f2: v = (±8, ±16) 0.6953 (interior 768), (0, ±16) 0.6944 (1152).
  - ac-f3: v = (±3, 0) 0.5712 (13770), (±3, ∓3) 0.5641 (13464).
  - ad-f2: v = (±1, ∓1) 0.6270 (2370), (±1, 0) 0.6267 (2400).
- **Conclusion:** translational status = **aperiodic** (the limit),
  reached by a **limit-periodic construction**: the depth-d tiling has
  period lattice ⟨(Wλ^d, 0), (0, Hλ^d)⟩ (in (q,r) cell coords; world
  (Wλ^d, 0) × (Hλ^d/2, (√3/2)Hλ^d)) which grows without bound. Report
  BOTH measured facts in the classification/report ("aperiodic"
  translational field; limit-periodic construction in the doc row).

### Single-tile containment (geometric fact only) and the reconciliation

s07: each folklore patch is 100% contained in a single scaled tile (pin
values in evidence chain step 8) — but s06/s06diag show the patch is NOT
generated from any single tile (dmax > 1 seed structure, 2→3 no-merge
genuineness) and s09 shows it is NOT a fixed-point window. The
containment is a geometric curiosity (the patch's bounding shape is
pentiamond-like) and must be presented as such — it does not license a
single-tile seed.

### Seed blocks (pinned s13 — the generation seed)

Conventions (from `s13_blocks.py`; source of truth `s13_blocks.pkl`):
each seed tile = (letter, gcanon, class, anchor, rel cells) where
**gcanon** = the min-pose class representative, **class** = its label
pair (chiral letters: singleton; achiral: {g, g∘8}), **anchor** = the
tile's mqr corner in block (q, r) coords, **rel cells** =
`norm(gcells(gcanon, SHAPES[letter]))` (mqr-anchored posed cell set);
**absolute cell = anchor + rel**. Block = W × H cell rectangle
(2·W·H unit triangles, both kinds); coverage union == the full
rectangle: verified True for all three. Chirality-charge pairs used in
the search: {1,7}, {2,3}, {4,10}, {5,11}, {8,9}.

**ac-f2: 3 × 5 block (30 cells, 6 tiles), block chirality charge 0
(A poses: two P1 = {6, 9}, two P2 = {3, 4}):**

```
A pose 3  class [3]     anchor (1,1)  rel ((0,1,1),(0,1,2),(1,0,1),(1,1,0),(1,1,1))
A pose 4  class [4]     anchor (1,3)  rel ((0,0,1),(0,1,1),(1,0,1),(1,1,0),(1,1,1))
A pose 6  class [6]     anchor (0,1)  rel ((0,0,1),(0,0,2),(0,1,1),(1,0,1),(1,1,0))
A pose 9  class [9]     anchor (0,0)  rel ((0,0,0),(0,0,1),(0,1,0),(1,0,0),(1,0,1))
C pose 1  class [1,7]   anchor (1,0)  rel ((0,0,1),(0,1,0),(0,1,1),(1,0,0),(1,1,0))
C pose 5  class [5,11]  anchor (0,3)  rel ((0,0,1),(0,1,0),(1,0,0),(1,0,1),(1,1,0))
```

**ac-f3: 2 × 5 block (20 cells, 4 tiles), block chirality charge −2
(both A seeds are P2): the P2-heavy start is what produces the folklore
patch's 617:1899 at depth 2, and it decays to 1:1 in the limit (s14b):**

```
A pose 4  class [4]     anchor (0,3)  rel ((0,0,1),(0,1,1),(1,0,1),(1,1,0),(1,1,1))
A pose 7  class [7]     anchor (0,0)  rel ((0,0,0),(0,0,1),(0,1,0),(1,0,0),(1,1,0))
C pose 1  class [1,7]   anchor (0,1)  rel ((0,0,1),(0,1,0),(0,1,1),(1,0,0),(1,1,0))
C pose 4  class [4,10]  anchor (0,2)  rel ((0,0,1),(0,1,1),(1,0,0),(1,0,1),(1,1,0))
```

**ad-f2: 1 × 5 block (10 cells, 2 tiles) — UNIQUE physical block:**

```
D pose 2  class [2,3]   anchor (0,2)  rel ((0,0,1),(0,0,2),(1,0,0),(1,0,1),(1,0,2))
D pose 8  class [8,9]   anchor (0,0)  rel ((0,0,0),(0,0,1),(0,0,2),(1,0,0),(1,0,1))
```

### Generation recipe (sphinx-style; s14-verified ALL PASS)

Mirror `sphinx.rs` `generate()` (L387–476) with these pinned differences:

1. **Seed = whole-block homothety, NOT per-tile.** Seed scale u = λ^d
   (d = the deflation depth from the view extent, see below). Block
   lattice in (q, r) coords: ⟨(W·u, 0), (0, H·u)⟩, anchored through the
   origin (sphinx's `seed_sphinxes` corner-projection pattern with a
   ±1-block margin). Within a block, seed tile i sits at anchor **u·A_i**
   with side u — i.e. the WHOLE block is homothetied by u about the block
   origin. (Per-tile homothety about each tile's own anchor is an offset
   Minkowski sum: neighbors overflow and it is NOT a tiling — this was
   tested and rejected in s14.) The seed union is exactly the u-scaled
   W×H rectangle: verified (24×40, 54×135, 16×80 at s14 depths).
   (Single-tile seed = a finite island, not a plane tiling — rejected on
   geometry, s07 containment is not generation.)
2. **Deflate d times, table 0 for every parent** (the canonical pin from
   the double-table mechanism). Placement formula, VERIFIED working
   (s14 `deflate_tile`; s02/s06 convention) for a parent of letter L,
   pose g, side λ^k (5·λ^{2k} unit cells — `scale_of` must test
   `n == 5·lam**(2k)`, not `lam**k`), region cell set C, table ti:
   - `R0 = lam_scaled(SHAPES[L], g, λ^k)` — **`lam_scaled` poses FIRST
     then homotheties about v0; the result is ALREADY posed — never wrap
     it in gcells** (pose-then-v0-homothety ≠ origin-homothety in
     general: the identity failed at (A, g = 0), s14 bug record).
   - `mR = mqr(R0)`; `U = union_j hom_origin(Fj, λ^{k−1})` where
     `hom_origin(cells, m)` = m-fold homothety of the cell REGION about
     the origin (per-cell `_anchor(c)` = (q, r) for UP, (q, r+1) for DN
     — the lex-min vertex).
   - `Gu = gcells(g, U)` (pose the union); `tau = mR − mqr(Gu)`;
     **frame probe (asserted): `trans(Gu, tau) == R0`.**
   - Region identity: `d = mqr(C) − mR`, assert `trans(R0, d) == C`.
   - **Child j cells = `trans(gcells(g, hom_origin(Fj, λ^{k−1})), d + tau)`.**
   - **Child pose = `compose(g, s0j)`**, where s0j = the min-index pose
     with `norm(gcells(s0j, SHAPES[Lj])) == norm(Fj)` (unique for chiral
     A; min of the pair for achiral C/D). `norm(S)` = re-anchor at mqr.
     mqr is NOT equivariant under pose composition — compare cell sets,
     never anchor bookkeeping.
   - `gcells(g, cells)` = `frozenset(apply_sym_cell(g, c) for c in cells)`
     (tri.py has no gcells; inline it).
3. **AABB cull every round** (sound: children never leave their parent's
   region — s14 asserts union == parent union each round), final cull
   against the query region + `MAX_TILES` guard with the standard
   "too many tiles for this view — zoom in" message (mirror sphinx L421,
   area guard scaled to the pentiamond tile area 5·(√3/4)).
4. **Depth from extent:** same shape as sphinx L440 with log2 → log_λ:
   `depth = ((extent / EXTENT_UNIT).log(λ).ceil().max(0.0) as u32).min(40)`.
   Choose EXTENT_UNIT so the standard trio's ±8 × ±6 view (scale 1.0)
   lands at a depth with unit-scale or coarse tiles (calibrate; pin in a
   view-stability test); the coverage test forces consistency.
5. **Emission:** `Tile { proto: letter_index, seed: 1 + pose, poly }`
   where letter_index = 0 for A, 1 for the second letter (C or D); pose
   = the tile's min-class pose label (0..11) — chiral A: exact; achiral:
   min(g, compose(g, 8)). All 12 poses of A occur in the limit (gseen
   12/12 in the patch; seed block carries 4 distinct A poses). World
   corners via (q, r) → (q + r/2, (√3/2)·r); emit the CCW outline from
   the canonical outlines above, REVERSED for det −1 poses (sphinx
   `corners()` pattern).
6. **Every depth is a valid plane tiling** with period
   (Wλ^d, 0) × (0, Hλ^d) ⇒ limit-periodic construction, aperiodic limit.
   **Nested views are REFINEMENTS, not bit-identical snapshots:** each
   shallower-view tile is the exact union of λ² deeper-view tiles with
   the same letter + pose (sphinx-style; pin this in a test).

**s14 verification summary (ALL PASS, full run 4.6 s):**

| entry | block | depth | frame probe | relabel invariance | seed | round 1 | round 2 | final (unit) | zero-period census |
|---|---|---|---|---|---|---|---|---|---|
| ac-f2 | 3×5, 6 tiles | 3 | 108/108 (P,g,k,ti) | 24/24 | 6 tiles side 8 → 24×40 rect | 24 side 4 {A:16, C:8}, A P1:P2 = 8:8 | 96 side 2 {A:64, C:32} 32:32 | 384 {A:256, C:128} 128:128, all valid | \|v\| ≤ 16: 0 exact; tops (±8,±16) 0.6953, (0,±16) 0.6944 |
| ac-f3 | 2×5, 4 tiles | 3 | 108/108 | 24/24 | 4 tiles side 27 → 54×135 rect | 36 side 9 {A:22, C:14}, P1:P2 = 5:17 | 324 side 3 {A:202, C:122} 68:134 | 2916 {A:1822, C:1094} 731:1091, all valid | \|v\| ≤ 12: 0 exact; tops (±3,0) 0.5712, (±3,∓3) 0.5641 |
| ad-f2 | 1×5, 2 tiles | 4 | 96/96 | 12/12 (single-table pair (0,0)) | 2 tiles side 16 → 16×80 rect | 8 side 8 {A:4, D:4} 2:2 | 32 side 4 {A:16, D:16} 8:8 | 512 {A:256, D:256} 128:128, all valid | \|v\| ≤ 10: 0 exact; tops (±1,∓1) 0.6270, (±1,0) 0.6267 |

Round-1 chirality pins are hand-derived and asserted in s14:
ac-f2 (8,8) from A seeds (2×P1 → (4,2) + 2×P2 → (2,4)) and C seeds
2×(1,1); ac-f3 (5,17) from A seeds both P2 → (0,12) plus C seeds
pose 1 (P1) + pose 4 (P2) injecting (4,1)+(1,4) = (5,5); ad-f2 (2,2)
from the two D seeds each injecting (1,1). **Per-round letter counts ==
M^round × block counts at every round** (asserted). Chirality dynamics
depths 0..20 == closed forms (s14b): ac-f2 1:1 every depth; ac-f3
5:17 → 68:134 → 731:1091 → 0.7863 (d4) → 0.9807 (d9) → 0.9999 (d20);
ad-f2 1:1 every depth.

### ac-f3 D row — blocker (STOP applied to this row only; the rest delivers)

- **No D rule row in the rule image:** s04c prints
  `MISSING ROWS (no rule row in the image): ['D']` — only 2
  parent-row components in the ac-f3 rule PNG, zero D pixels.
- **No D parent regions in the patch:** s02 `n_parents = 0` (bare marker
  `{'lam': 3, 'n_parents': 0}`, no `all_tables`); s01 decode: the ac-f3
  patch contains A and C fills only.
- The entry header says "3 Prototiles" (A, C, D). The D row is therefore
  **unpinnable from the evidence — do not guess a D dissection.**
- Immaterial to the limit tiling: the A/C block `[[6,5],[3,4]]` is closed
  (no A or C parent emits D) and the pinned seed block is D-free, so D
  count = 0 at every depth under ANY D column; s03 scanned all 55
  sum-9 D columns — every one keeps PF = 9 but none makes the 3×3
  primitive. The folklore patch itself is likewise D-free.
- **Decision:** implement the 2-prototile {A, C} tiling; set
  `prototiles: Some(2)` and **flag the 2-of-3 discrepancy in the
  worker report** (and, if the GUI/catalog wants a string, note "entry
  header lists 3 prototiles; D row absent from rule image and patch").

### Files you own

- `crates/tess-tilings/src/families/reptiles/pentiamond_ac2.rs`,
  `pentiamond_ac3.rs`, `pentiamond_ad2.rs` — the three new modules
  (underscore file-name convention like `p_pentomino.rs` /
  `pregnant_chair.rs`); each self-contained like `sphinx.rs` (own SYMS
  copy or a small shared helper within the family — mirror sphinx's
  structure; read `p_pentomino.rs` for the chiral-mix documentation style).
- `crates/tess-tilings/src/families/reptiles.rs` — module decls (3) +
  `register` slice (3 arms appended after tangram — within-tab order) +
  `classification` arms (3, custom inline `TilingClass` per the tangram
  pattern — the `rep_tile()` helper does NOT fit: it hard-codes
  inflation "λ = 2" and prototiles 1) + census `IDS` 6 → 9 + family
  census 6 → 9 + three test-trio blocks (mirror the armchair trio L102–141).
- `docs/families/reptiles.md` — three rows in the
  "### Rep-tiles (nonperiodic / limit-periodic)" table + heading
  "## Implemented (6 tilings)" → "(9 tilings)".
- No `lattice_tilings()` change (the reptiles family has no lattice
  table entries).

**Do not touch:** `lib.rs` (census guards fire per landing — report their
verbatim output, never fix), README, CHANGELOG, `docs/INDEX.md`, ROADMAP,
PROPOSED.md, sibling reptile modules (read-only references), other
families, UI crates, AGENTS.md.

### Commit exception (this brief only)

This research job commits ONE docs-only file:
`docs: pentiamond rule reconstruction brief (AC f2, AC f3, AD f2)` —
body lists the pinned rules + evidence chain, no M numbers. The future
implementation worker will make a separate commit (subject like
`reptiles-family: pentiamond set (AC f2, AC f3, AD f2)`) — no M numbers
in either; the orchestrator cherry-picks and assigns M numbers at
integration.

### Constraints (the ones that bite here)

1. **Exact integer arithmetic end to end:** i64 `(kind, q, r)` cell
   tuples; floats only at the emission boundary (no decimal literals in
   exact geometry — sphinx/M141 pattern). `MAX_TILES = 250_000`.
2. **Use the pinned placement formula verbatim** (Generation recipe
   step 2). No general identity between pose-then-v0-homothety and
   origin-homothety for multi-cell shapes (s14 failed an assert trying
   the shortcut at (A, g = 0)). `lam_scaled` output is ALREADY posed —
   never wrap in `gcells`.
3. **Table 0 for every pose** (canonical pin; double-table mechanism
   above). Do NOT implement the second achiral dissection; document it
   in the module header as the alternative the folklore patch also uses.
4. **Whole-block homothety seed only** (per-tile = Minkowski overflow,
   not a tiling; single-tile = finite island; patch-window repeat =
   invalid because the patch is level-2 aperiodic, s09).
5. **Scale bookkeeping:** a side-λ^k pentiamond holds **5·λ^{2k}** unit
   cells — `scale_of` must test `n == 5·lam**(2k)` (testing `lam**k`
   mis-scaled a side-8 seed as side-64, s14 bug record).
6. **Achiral letters:** pose label class = {g, compose(g, 8)}; emit the
   min-class pose; the two labels are pure translations (assert this in
   a test — the s14-C check).
7. **Depth formula:** log_λ, not log2 (sphinx L440 shape); clamp 0..40;
   pin the default-view depth in a view-stability test (octave-style).
8. **Census guards fire per landing — three times in this one brief**
   (reptiles family census, `lib.rs` per-tab vec Nonperiodic element,
   built-ins, memberships). Base at `8d09364`: 6 / 7 / 137 / 151;
   expected 9 / 10 / 140 / 154. **Never hard-code totals from your base**
   — other landings may move them; report exactly what your guards
   printed (their verbatim failure output IS the delta evidence).
9. **ac-f3: 2 prototiles implemented of the entry's 3** — blocker above;
   `prototiles: Some(2)` + flag in the report. The D row is NOT guessed.
10. **Grep gate (pre-state verified 2026-11-22 at `8d09364`):**
    `grep -rn "pentiamond" crates/ docs/families/ README.md docs/INDEX.md`
    → ZERO hits before your work. Your new files introduce the word;
    nothing OUTSIDE the owned files may mention it (PROPOSED.md's prose
    mention is pre-existing and fine).
11. `cargo check --workspace` zero warnings; `cargo fmt` owned hunks only
    (the repo is NOT fmt-clean — revert non-owned byproducts);
    `cargo test --workspace` green except the orchestrator census
    guards (item 8).

### Battery spec (already run in recon — the numbers below are MEASURED,
re-verify them in the Rust tests; every value is a pin)

Recon ran stdlib-Python spikes (`/tmp/opencode/pentiamond-recon/`,
≤240 s each, all checks passed; `NOTES.md` 616 lines). Per tiling, the
Rust test list (mirror sphinx's test block L479+ and the reptiles.rs
trio):

1. `rule_partition` — for EVERY parent letter P, pose g ∈ 0..11, scale
   k ∈ {1, 2} (ac-f3: {1, 2, 3}), table 0: frame probe
   `trans(gcells(g, U), tau) == R0` (union of posed λ^{k−1}-scaled child
   cells exactly tiles the posed λ^k-scaled parent region); plus the s02
   self-check in the identity frame: each table disjoint, union ==
   idcells, |T| = λ². Pins: ac-f2 108/108, ac-f3 108/108, ad-f2 96/96
   (P × pose × scale × table combos at s14 depths).
2. `achiral_label_invariance` — C (ac-f2, ac-f3): for every pose g,
   children placed under (g, T0) == under (compose(g,8), T1) as cell-set
   multisets, and cross (g, T1) == (compose(g,8), T0) — 24/24 combos
   each; D (ad-f2): (g, T0) == (compose(g,8), T0) — 12/12. Also assert
   the achiral pair's posed regions are pure translations (R0(g∘8) =
   R0(g) + t for all g).
3. `substitution_matrix_and_counts` — M derived from the tables == the
   pinned M (above); column sums λ²; closed-form per-seed pins at
   d = 1..4: ac-f2 per A (3,1)/(11,5)/(43,21)/(171,85), per C
   (2,2)/(10,6)/(42,22)/(170,86); ac-f3 per A (6,3)/(51,30)/(456,273),
   per C (5,4)/(50,31)/(455,274); ad-f2 per seed (2,2)/(8,8)/(32,32)/
   (128,128); and per-block pins {A:16,C:8}/{A:22,C:14}/{A:4,D:4} at
   round 1 (M × block counts at every round).
4. `char_poly_pin` — det(xI − M) by exact i64 expansion, pinned as
   COEFFICIENT VECTORS: 2×2 matrices by 2-permutation expansion (assert
   the permutation count was 2 was visited — M135 lesson), the ac-f3 3×3
   placeholder by 6-permutation expansion. ac-f2 [1, −5, 4]
   (x²−5x+4, roots {1, 4}); ac-f3 A/C block [1, −10, 9] (x²−10x+9,
   roots {1, 9}); ad-f2 [1, −4, 0] (x²−4x, roots {0, 4}); ac-f3 3×3
   placeholder [1, −19, 99, −81] = (x−1)(x−9)² (never primitive).
   PF = λ², primitive index 1 (ac-f3: the 2×2 A/C block; document that
   the 3×3 with ANY sum-9 D column is never primitive — s03 scan of all
   55).
5. `deflation_block_reconstruction` — seed from the pinned s13 block
   (exact tile list above), deflate d = 3/3/4 rounds: round-1 pins
   (counts AND A chirality P1:P2 = 8:8 / 5:17 / 2:2 — hand-derived,
   asserted); per-round letter counts == M^round × block counts;
   disjointness + union == seed rectangle at every round; final unit
   validity (5 unit cells, letter == canonical name, pose exact for A /
   within class for C, D; pins 384 / 2916 / 512 tiles).
6. `refinement_property` — nested views are refinements: each
   depth-(d−1) tile is the exact union of λ² depth-d tiles with the same
   letter + pose (pin on the block recon at d = 2 vs d = 3).
7. `chirality_pins` — A-tile P1:P2 by depth: ac-f2 1:1 at d = 1..6;
   ac-f3 5:17, 68:134, 731:1091 at d = 1..3 and ratio within 2 % of 1
   by d = 9 (s14b: converges to 1:1; the folklore patch's 617:1899 ≈
   1:3.08 is the finite-depth artifact — do NOT assert 1:1 at small
   depth); ad-f2 1:1 at d = 1..6.
8. `no_period_census` — final unit tiling of the depth-d block,
   same-letter match over interior cells for all |v| ≤ VBOX (16 / 12 /
   10), exact = 100 % on ≥ MININT (200 / 200 / 120) interior cells:
   ZERO exact periods; near-miss top pins (must be ≤ the pinned value
   in the sense "no v reaches 1.0"; report the measured tops):
   ac-f2 (±8,±16) 0.6953, (0,±16) 0.6944; ac-f3 (±3,0) 0.5712,
   (±3,∓3) 0.5641; ad-f2 (±1,∓1) 0.6270, (±1,0) 0.6267.
9. `standard trio` (per tiling, reptiles.rs armchair pattern
   L114–140) — `check_coverage` on ±8 × ±6 with `SCALE_ID` = 1.0;
   `check_rotated` scale 0.6, 25°, ±7 box; `check_det` ±5 box (helper
   names from `crate::tests`; test-function names follow the armchair
   trio: `*_covers_region` / `*_rotated_scaled` / `*_deterministic`).
10. `view_stability` — pin the default-view depth on the standard box
    (EXTENT_UNIT calibration, constraint 7); far-view background or
    zoom-in message per the area guard (sphinx L421 pattern).

Recon-only evidence (cite in the report, not Rust tests): s01 patch
censuses (9945/4024/9969 tiles, coverage exactly-once, worst geo err
0.00714); s02 table copy counts and gseen 12/12; s03 matrices/patch
shares; s04c row matches (poses 10 / {4,10} / u0 31.75–32.01 px, table
matches, MISSING ROWS ['D']); s05 baselines 0.5557/0.5314/0.5000;
s06 E PASS reconstruction (cov 1.000000, bad 0/0) + folklore seed
censuses {1:105,2:2460}/{1:136,2:432}/{1:109,2:933,3:383}, dmax 2/2/3,
patch A chirality 3316:3316 / 617:1899 / 2490:2492; s06diag: 2→3 no-merges GENUINE for ac-f2/ac-f3 (0 TRUE of the 510/41
cells_ok regions); ad-f2 81 TRUE A-parent dissections within its 383
merged level-3 parents (dmax = 3); s07 smallest single-tile containment (level 8 A pose 7 at
(−128,−62); level 5 A pose 7 at (−135,0); level 8 A pose 7 at
(−73,−87)) — geometric fact only; s08b no rectangle ≤ 20² cuttable from
any patch; s09 zero level-2 periods in all three patches; s14-E block-
shift same-letter ratios (no period): ac-f2 (1,0) 12/20 = 0.6000,
(0,1) 13/24 = 0.5417; ac-f3 (1,0) 8/10 = 0.8000, (0,1) 8/16 = 0.5000;
ad-f2 degenerate (8/8, n < 10 — single-letter block; the F census is
authoritative).

### Catalog rows (draft for `docs/families/reptiles.md`)

Heading: "## Implemented (6 tilings)" → "## Implemented (9 tilings)".
Three rows appended to the "### Rep-tiles (nonperiodic / limit-periodic)"
table:

| `pentiamond-ac-factor-2` | Pentiamonds A + C (2-prototile rep-4, triangular lattice) | Bielefeld substitution index entry "Pentiamond AC factor 2" (discovered: folklore; Info empty — rule image the sole source): λ = 2, A → 3A + 1C, C → 2A + 2C with the achiral C double-labeled (mirror dissections T1 = 8·T0 + (0,6) under pose pairs {g, g∘8}; canonical pin = table 0 for all poses, frame probe 108/108 + label invariance 24/24); M = [[3,2],[1,2]] char poly (x−1)(x−4), PF 4 = λ², primitive index 1, per-A-seed closed form A_d = (2·4^d+1)/3; 3×5 rectangular seed block (4 A poses 3/4/6/9 + 2 C poses 1/5), whole-block homothety on the (3·2^d, 0) × (0, 5·2^d) block lattice; limit-periodic construction, aperiodic limit — zero exact periods in the unit tiling (|v| ≤ 16, top near-miss 0.6953 < 1); chiral A balanced 1:1 at every depth; folkloric 9945-tile patch reproduced exactly by the pinned rule (recon E PASS, coverage 1.0); exact integer cell arithmetic, floats only at emission |
| `pentiamond-ac-factor-3` | Pentiamonds A + C (2-prototile rep-9, triangular lattice) | Bielefeld substitution index entry "Pentiamond AC factor 3" (discovered: folklore; Info empty; header says 3 prototiles but the **D row is absent from the rule image and the patch** — blocker, implemented as 2 prototiles, flagged): λ = 3, A → 6A + 3C, C → 5A + 4C (achiral C double-labeled, mirror pair via (8, (0,9)); canonical pin = table 0 for all poses, frame probe 108/108 + label invariance 24/24); A/C block [[6,5],[3,4]] char poly (x−9)(x−1), PF 9 = λ², primitive index 1 (no sum-9 D column makes the 3×3 primitive — 55-candidate scan), per-A-seed closed form A_d = (5·9^d+3)/8; 2×5 seed block (A poses 4,7 both P2 + C poses 1,4; block charge −2 — the folklore patch's P2-heavy 617:1899 depth-2 census, decaying to 1:1 in the limit, measured 0.9999 by d = 20); whole-block homothety on the (2·3^d, 0) × (0, 5·3^d) block lattice; limit-periodic construction, aperiodic limit — zero exact periods in the unit tiling (|v| ≤ 12, top near-miss 0.5712); folkloric 4024-tile patch reproduced exactly (recon E PASS, coverage 1.0); the A/C block is closed and the seed D-free ⇒ D count 0 at every depth regardless of the missing D row; exact integer cell arithmetic, floats only at emission |
| `pentiamond-ad-factor-2` | Pentiamonds A + D (2-prototile rep-4, triangular lattice) | Bielefeld substitution index entry "Pentiamond AD factor 2" (discovered: folklore; Info empty — rule image the sole source): λ = 2, A → 2A + 2D, D → 2A + 2D (D dissection SELF-MIRROR — single table under both achiral labels); M = [[2,2],[2,2]] = rank-1 (x²−4x), PF 4 = λ², primitive index 1, per-seed (2·4^{d−1}, 2·4^{d−1}); UNIQUE 1×5 seed block (2 D tiles, poses 2 and 8); whole-block homothety on the (1·2^d, 0) × (0, 5·2^d) block lattice; limit-periodic construction, aperiodic limit — zero exact periods in the unit tiling (|v| ≤ 10, top near-miss 0.6270); A balanced 1:1 at every depth; folkloric 9969-tile patch reproduced exactly (recon E PASS, coverage 1.0; the entry is depth-3 nested — the only one of the three); exact integer cell arithmetic, floats only at emission |

### Deliverable (worker report)

1. Diff-scope statement (exactly the owned files, nothing else) + the
   grep-gate result.
2. The pinned construction per entry: entry URL, rule tables (verbatim
   from this brief), M + char poly + PF + primitive index, closed
   forms, seed block, placement formula, table-0 pin + double-table
   deviation note, and the ac-f3 blocker (2 of 3 prototiles; D row
   absent from image and patch; limit tiling D-free regardless).
3. Battery numbers with exact values; Rust test list with pass counts;
   census guards' verbatim output (base totals at your branch point +
   your +3s: reptiles 6 → 9, Nonperiodic tab 7 → 10, built-ins 137 →
   140, memberships 151 → 154 — re-derive at your base, never copy).
4. Aperiodicity evidence as measured by your tests (zero exact periods
   with the near-miss tops) + the measured translational duality:
   "aperiodic" (limit) reached by a limit-periodic construction
   (period (Wλ^d, 0) × (0, Hλ^d) at depth d).
5. Proposed commit-message body (construction, verification,
   aperiodicity, blocker). NO M numbers — the orchestrator assigns
   them at integration.
