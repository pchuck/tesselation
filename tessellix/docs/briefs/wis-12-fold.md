# Brief: WIS-12-fold — Watanabe–Ito–Soma 12-fold substitution, rule tables certified

> **LANDED as M158** (see docs/history/CHANGELOG.md). Do not dispatch.

**Family:** quasiperiodic · **Candidate:** Bielefeld Substitution Tilings
Encyclopedia entry "Watanabe-Ito-Soma 12-fold" · **Effort:** L
(implementation dispatch, after unblock) · **Precedent to mirror:**
`crates/tess-tilings/src/families/quasiperiodic/stampfli.rs`,
`crates/tess-tilings/src/families/quasiperiodic/golden_rhomboid.rs`

### Status and scope

Research prepass **and unblock, completed**. This document is
**docs-only**: `crates/` is untouched in this commit. Implementation
remains **BLOCKED** until the GLB n=7 field-layer work lands (the
quasiperiodic family slot is owned by that session); the former
cluster-geometry blocker is **resolved** — all three rule tables
below are CERTIFIED under the STOP-clause acceptance, which is the
STOP clause's own condition for flipping the flags. No unverified
table is presented as certified.

| panel | group / oracle | oracle ratio | worst-child | flag: prepass → now |
|---|---|---|---|---|
| tri-sup | 8.8751 / 8.9036 | 99.68 % | 0.7547 | AMBIGUOUS → **CERTIFIED** |
| rho-sup | 10.2591 / 10.2591 | 100.0 % | 0.7543 | UNCONFIRMED → **CERTIFIED** |
| sq-sup | 20.4073 / 20.7403 | 98.39 % | 0.6684 | BLOCKED → **CERTIFIED** |

**Scope caveat (measured, not speculation):** the certified tables
are certified **against the `rule.png` figure**, and they do
**not** generate the Bielefeld patch: tri-sup 0 occurrences among
150 patch supertile leaves (0 of 15 congruence classes), rho-sup
full-13 cluster 0 occurrences in all 12 rotations (blob 0/5,472
patch hexes), sq-sup 0 closed 26-tile patches in the crop
(§"Figure vs patch inconsistency"). The figure is attributed to the
non-unique [WSI95] Fig. 2(iii) ruleset; the Musesti–Paolini 2023
"correct set" (the one that generates the patch) has no
machine-readable record. Read §"Rule provenance (corrected)" and
§"Implementation consequence" before dispatch.

### Goal

Land the Watanabe–Ito–Soma 12-fold tiling in the quasiperiodic family:
3 prototiles (60° equilateral triangle, square, 30° rhombus),
dodecagonal, inflation **λ = 2+√3**, FLC w.r.t. rigid motions,
**non-conformal patch-based** substitution (jagged clusters, not
λ×-parent frames — proven below). Rule tables now certified below;
see status for the figure/patch caveat.

### Literature basis

- **[WSI95]** Y. Watanabe, T. Soma, M. Ito, "A new quasiperiodic
  tiling with dodecagonal symmetry", *Acta Cryst.* A **51**(6) (1995)
  936–942, DOI **10.1107/s0108767395009160** (citation verified
  against Crossref this session; the prepass's shorthand "Tilings by
  equilateral triangles, squares and rhombi" is not the paper's
  title). Full text paywalled and Cloudflare-walled; no OA repository
  copy (OpenAlex `W1973554396`: is_oa=false,
  any_repository_has_fulltext=false, 3 citers; Semantic Scholar:
  CLOSED). The reconstructed abstract (OpenAlex inverted index)
  confirms the tile set (thin rhombus, regular triangle, square,
  derived from a 12-fold zonogon) and a self-similar inflation rule.
- Bielefeld entry (live-verified 2026-11, re-fetched this session):
  <https://tilings.math.uni-bielefeld.de/substitution/watanabe-ito-soma-12-fold/>
  with assets `rule.png` (six panels, two per prototile), `patch.png`,
  `patch.pdf` under `/img/substitution/watanabe-ito-soma-12-fold/`.
  The entry's **vector download is the patch only** — no rule-table
  vector. A variant entry
  (`…/substitution/watanabe-ito-soma-12-fold-variant/`, "a variant
  … with more symmetry") ships `rule.gif` + `patch.gif` only.
- **Rule provenance (CORRECTED this session).** The prepass
  checkpoint attributed the `rule.png` panels to the Musesti–Paolini
  2023 submission. That attribution is **measured-false**: the
  decoded panel tables have zero occurrences in the Bielefeld patch
  (§"Figure vs patch inconsistency"), so the panels cannot be the
  set MP submitted as the patch's "correct set". Parsimonious
  attribution: **`rule.png` is the [WSI95] Fig. 2(iii) figure** —
  the very ruleset the entry page says "are not unique" — and the
  MP-2023 "correct set" is available only implicitly (as the patch
  itself). Source hunt for a machine-verifiable published rule set:
  **NEGATIVE** (w07_sources.md): no DOI/arXiv/index record for any
  Musesti or Paolini tiling paper 2022–2024 (Crossref, OpenAlex,
  S2); arXiv:2102.06046 (Schaad–Stampfli) is a *cousin* tiling —
  same three base shapes, inflation **1+√3 ≠ 2+√3** — which merely
  classifies WSI95 among the 2+√3 tilings; the Bridges 2021
  Stampfli–Schaad talk gives its rules as slide figures only.
- Entry parameters: 3 prototiles, λ = 2+√3, dodecagonal, FLC w.r.t.
  rigid motions.

### Pinned facts (prepass, exact)

**Basis convention (PINNED).** H = s/2 = 11.59765 content units
(s = 23.1953 = square edge in `patch.pdf` content units; PDF device
transform = 0.1×). Every patch vertex is an integer 4-tuple
(P,Q,R,S) ∈ ℤ⁴:

```
x = (s/2)·(P + Q√3),  y = (s/2)·(R + S√3)
```

i.e. **M = span_ℤ{(2,0),(0,2),(√3,1),(1,√3)} in H-units; the vertex
module is the full ℤ⁴** (a single-coset rank-4 ℤ[√3] fit fails,
residuals up to 0.5 H). Tile **edge** vectors live in the index-4
sublattice {P≡S (mod 2), Q≡R (mod 2)}. 30° rotation step, k mod 12:

```
rot4(P,Q,R,S) = ((3Q−R)/2, (P−S)/2, (P+3S)/2, (Q+R)/2)
```

Shape periods (footprint symmetry): tri 4, sq 3, rho 6.

**Prototiles (PINNED).** CCW, y-up, H-units, corners in (P,Q,R,S);
edge length 2H; 2·area in H² as ℤ[√3] pair (a,b) = a + b√3:

| tile | corners (P,Q,R,S) | 2·area |
|---|---|---|
| tri (equilateral △, 60°) | (0,0,0,0),(2,0,0,0),(1,0,0,1) | (0,2) |
| sq (square) | (0,0,0,0),(2,0,0,0),(2,0,2,0),(0,0,2,0) | (8,0) |
| rho (30° rhombus) | (0,0,0,0),(2,0,0,0),(2,1,1,0),(0,1,1,0) | (4,0) |

**Inflation + partition evidence (PINNED, exact ℤ[√3]).**
λ = 2+√3, λ² = 7+4√3 ≈ 13.928. Per-prototile supertile 2·area pairs
and compositions (the composition is the **unique** nonnegative
integer solution of the area equation; zero excess in every case):

| supertile | children | composition | 2·area | λ² check |
|---|---|---|---|---|
| tri-sup | 11 | 7 tri + 2 sq + 2 rho | (24,14) | (7,4)·(0,2) = (24,14) ✓ |
| rho-sup | 13 | 8 tri + 2 sq + 3 rho | (28,16) | (7,4)·(4,0) = (28,16) ✓ |
| sq-sup | 26 | 16 tri + 4 sq + 6 rho | (56,32) | (7,4)·(8,0) = (56,32) ✓ |

**Rule nature: non-conformal (PINNED).** A conformal λ×-parent frame
is impossible: λ·(2H) = (4+2√3)·H cannot be tiled by collinear 2H
child edge steps (collinear-edge proof; 0/143,892 frame candidates).
Each prototile expands to a jagged **cluster** tiled exactly by its
children (closed simple boundary, all interior edges paired, exact
area identity — certified in exact ℤ[√3]). Exception inside the
figure: the **rho-sup panel is three disconnected drawn pieces**
(9+2+2 tiles, inter-piece gaps ≈ 1.9 H) — its assembly is certified
piecewise (§Rule tables, rho-sup) and the inter-piece offsets are not
fixed by the figure.

**Panel / scale facts (PINNED, measured).** `rule.png`: 6 panels,
panels 0/1 = tri-sup, 2/3 = rho-sup, 4/5 = sq-sup. Raster fit:
k (px/H) = 14.5644 (tri-sup, anchor px (104, 120.5)), 14.6088
(rho-sup, anchor (353, 158.5)), 14.6796 (sq-sup, anchor
(647, 81.5)); per-child IoU 0.619–0.923; ℤ⁴/M-snap residual
≈ 0.02–0.05 H ⇒ panel child placements are lattice-snapped and
accurate to ~0.05 H. `patch.pdf`: 37,635 tiles = **18,302 tri +
11,644 sq + 7,689 rho** (Bielefeld patch census); the tiling
occupies the lower half of a 490×490 H patch, ≈ 245 s per axis.
Note: 8,852 of the parsed tiles form a second interleaved tiling
layer with edge vectors outside the M lattice (e.g. ℤ⁴ (97,−55,1,0))
— not parse noise, not part of the rule.

### Rule tables (CERTIFIED — measured against the `rule.png` figure)

All three tables below are the exact ℤ⁴ child placements of the
`rule.png` figure panels. Each is certified in exact ℤ[√3]
(disjointness, edge pairing, closed simple boundary, exact area
identity) and accepted against the brief's raster criterion
(§Verification battery, item 2). They are the **figure's** rules,
not the patch's (§Figure vs patch inconsistency).

**Reading the tables.** Rows are in panel child order `ch` (w05
decode: sorted by (shape, centroid x, centroid y) within the panel).
`krot` mod 12 is the figure-frame rotation; `m = (P,Q,R,S)` is the
anchor corner in the pinned basis (x = P+Q√3, y = R+S√3, in H units;
the H-unit column is a convenience, not a source of truth). Frames:

- **tri-sup / sq-sup: walk frame.** The cluster boundary chain is
  walked CCW from `k0 = 0` (root corner at the origin); rows are in
  the standard frame, which is k0-invariant (all 12 k0 values pass
  the exact solve identically — the sweep is a gauge).
- **rho-sup: per-piece frames.** The drawn cluster is three
  disconnected pieces; each piece is anchored at its rho anchor
  (m = (0,0,0,0)). Inter-piece offsets: see rho-sup pins (not fixed
  by the figure).

**Method (shared).** (1) Panel decode per the w05 conventions:
exact area-classified shape, convex hull of each tile's pixel
component, undirected k-candidates from edge directions. (2)
Placements — tri/sq: the raster boundary is walked (15 / 20 edges
snapped to the 30° grid, k0 = 0), a per-panel affine drift
correction is fit from walk corners to exact ℤ⁴ boundary corners,
and the interior children are solved by exact ℤ⁴ DFS with full-edge
demand propagation; rho: shared edges are detected on stub-merged
raster hulls (chipped corners replaced by line-intersection merges;
real edges 1.5–2.0 H vs stubs ≤ 0.22 H) and each piece is assembled
by exact ℤ⁴ DFS — m pinned by the shared-edge equation, orientation
by cyclic (k,s) matching against the hull direction sequence (the
rho k↔k+6 ambiguity is real and is resolved only by the cyclic
shift search). (3) Certification in exact ℤ[√3]. (4) Scoring:
per-child self-overlay IoU at 30 px/H, cell-center convention,
**in the corrected frame — table and oracle in the same frame and
grid** (oracle = per-child best unit prototile over all krot and ℤ⁴ m
within a 1.0 H window of the child's raster centroid).

**Acceptance (STOP clause, verbatim):** "group sum within 0.95 of
oracle **and** worst-child ≥ 0.6" — **MET on all three panels**
(numbers below). Prepass negative control: no patch congruence class
overlays any panel (best no-match 3.987/11 tri — 4 of 15 groups at
3.9868; 3.718/13 rho; worst-child 0.000 in every group), so a
positive match can only come from the figure's own geometry.

#### tri-sup — CERTIFIED, 99.68 % of oracle

11 children (7 tri + 2 sq + 2 rho), 2·area (24,14) exact, 15 boundary
edges. 9 boundary children (all but ch5, ch7 — the 2 interior
triangles exactly fill a 2-triangle hole, 2·area (0,4): (24,10)
boundary + (0,4) hole = (24,14)).

| ch | shape | krot (mod 12) | m (P,Q,R,S) | m in H-units |
|---|---|---|---|---|
| ch0 | rho | 1 | (−1,−1,−1,−1) | (−2.7321, −2.7321) |
| ch1 | rho | 0 | (−1,−1,−1,−1) | (−2.7321, −2.7321) |
| ch2 | sq | 0 | (0,0,0,0) | (0.0000, 0.0000) |
| ch3 | sq | 2 | (1,1,−1,−1) | (2.7321, −2.7321) |
| ch4 | tri | 3 | (0,0,0,0) | (0.0000, 0.0000) |
| ch5 | tri | 0 | (−1,0,0,−1) | (−1.0000, −1.7321) |
| ch6 | tri | 3 | (1,0,−2,−1) | (1.0000, −3.7321) |
| ch7 | tri | 10 | (0,0,0,0) | (0.0000, 0.0000) |
| ch8 | tri | 0 | (0,0,2,0) | (0.0000, 2.0000) |
| ch9 | tri | 3 | (2,1,−1,0) | (3.7321, −1.0000) |
| ch10 | tri | 0 | (1,1,−1,−1) | (2.7321, −2.7321) |

Pins (w06r_tri.json / w06r_tri.log):

- Walk: N=15, k0=0, snap residual 0.964°; raster endpoint residuals
  min/med/max 0.00792 / 0.10891 / 0.47496 H; per-child boundary
  drift 0.0352–0.1442 H (median 0.0884).
- Drift correction (exact walk frame → panel px): A =
  [[1.086155, −0.003608], [−0.001632, 1.090952]], t = (2.785537,
  4.145813); area scale **1.088548** (principal 1.044971 / 1.041701,
  rotation −0.1903°); inliers 22/30.
- Uniqueness: 18 raw interior completions → 2 unique geometries
  (ch5/ch7 assignment swap); the winner covers all 11 raster masks;
  the loser scores group 7.2614, worst 0.0000 → rejected.
- Raster acceptance (30 px/H, walk frame): per-child [0.7788, 0.7835,
  0.9062, 0.9071, 0.7675, 0.8109, 0.7664, 0.8058, 0.8048, 0.7547,
  0.7893]; group **8.8751** vs oracle **8.9036** (99.68 % ≥ 95 %
  ✓); worst **0.7547** (ch9) ≥ 0.6 ✓. Note: the oracle is a
  per-child raster-local optimum, and its argmax can sit on a
  *different* lattice tile than the table row — ch9 table
  (k3, m(2,1,−1,0)) = 0.7547 vs oracle (k3, m(−3,4,−1,0)) = 0.7833,
  a 0.196 H shift toward the drawn position; the table row is the
  exact edge-matched placement.
- Native panel frame (table translated by T = (1,1,−3,4) into the
  panel's native k-gauge, no scale/rotation): group 6.4551, worst
  0.3127 (ch8) — k-miscalibration plus the ~1.045×/−0.19° panel
  drift; reported for completeness, not the acceptance frame.

#### rho-sup — CERTIFIED, 100.0 % of oracle (aligned == oracle per child)

13 children (8 tri + 2 sq + 3 rho), 2·area (28,16) exact.

**Structure (verified from the raster hulls).** The drawn cluster is
**three disconnected pieces** — inter-piece gaps ≈ 1.9 H (intentional,
not drift; within-piece tile gaps 1–3 px as in the other panels):

- **blob** = ch1, ch3, ch4, ch5, ch7, ch8, ch9, ch10, ch12 (6 tri +
  2 sq + 1 rho, 2·area (20,12)); 10 internal / 10 boundary edges;
- **island** = ch0, ch6 (1 rho + 1 tri, 2·area (4,2)); 1 internal /
  5 boundary edges;
- **pendant** = ch2, ch11 (1 rho + 1 tri, 2·area (4,2)); 1 internal /
  5 boundary edges.

Drawn totals: I = 12, B = 20 (44 tile edges). The 12 shared edges
(gaps 0.1204–0.2075 H): blob ch1e3↔ch7e1, ch1e2↔ch8e2, ch1e0↔ch9e1,
ch1e1↔ch10e2, ch3e3↔ch5e0, ch3e2↔ch7e0, ch3e1↔ch9e2, ch4e3↔ch8e0,
ch4e0↔ch10e1, ch4e1↔ch12e2; island ch0e1↔ch6e1; pendant
ch2e3↔ch11e0. Each piece certifies: hex, disjoint, edge-paired,
area-identity, contained.

**Exact assembly.** Per-piece exact ℤ⁴ DFS (anchor = the piece's
rho; m pinned by the shared-edge equation; orientation by cyclic
(k,s) match): blob 130,784 DFS nodes → **exactly 2 solutions**,
island 8 → 2, pendant 8 → 2; every solution certifies, and the
second of each pair is a gauge copy (rho anchor (k,s)↔(k+6,s+2) with
the matching m shift). Radial centroid residuals (H): blob 1.9974
(sum over 9 tiles ≈ 0.22 mean), island 0.1267, pendant 0.1195.

| ch | piece | shape | krot (mod 12) | m (P,Q,R,S) | m in H-units |
|---|---|---|---|---|---|
| ch0 | island | rho | 1 | (0,0,0,0) | (0.0000, 0.0000) |
| ch1 | blob | rho | 10 | (0,0,0,0) | (0.0000, 0.0000) |
| ch2 | pendant | rho | 1 | (0,0,0,0) | (0.0000, 0.0000) |
| ch3 | blob | sq | 0 | (−1,0,−2,−1) | (−1.0000, −3.7321) |
| ch4 | blob | sq | 0 | (0,1,−1,0) | (1.7321, −1.0000) |
| ch5 | blob | tri | 3 | (−1,0,−2,−1) | (−1.0000, −3.7321) |
| ch6 | island | tri | 4 | (2,1,1,0) | (3.7321, 1.0000) |
| ch7 | blob | tri | 0 | (−1,0,0,−1) | (−1.0000, −1.7321) |
| ch8 | blob | tri | 3 | (0,1,−1,0) | (1.7321, −1.0000) |
| ch9 | blob | tri | 1 | (1,0,−2,−1) | (1.0000, −3.7321) |
| ch10 | blob | tri | 2 | (1,1,−1,−1) | (2.7321, −2.7321) |
| ch11 | pendant | tri | 2 | (0,0,0,0) | (0.0000, 0.0000) |
| ch12 | blob | tri | 1 | (2,1,−1,0) | (3.7321, −1.0000) |

**Global frame (w06t).** Linear 8-unknown least squares — global
(a,b) = (s·cosθ, s·sinθ) plus a free translation per piece; the
system is linear, so the optimum is unique and global. Result:
**s = 1.121601, θ = −0.3037°** (a = 1.121585, b = −0.005945); piece
translations (H): blob (3.9110, 7.0179), island (−0.2620, −0.2193),
pendant (8.0605, 8.1207). Centroid fit residuals min/med/max
**0.0098 / 0.0282 / 0.0528 H** (max 0.77 px) — the figure is
lattice-exact; leave-one-tile-out refits agree (s 1.1204–1.1233,
θ −0.44°…−0.20°, RSS 0.0129–0.0134).

**Raster acceptance (30 px/H, corrected frame).** Per-child
[0.7585, 0.7725, 0.7668, 0.8678, 0.8730, 0.7845, 0.7543, 0.7742,
0.8083, 0.7961, 0.7692, 0.7577, 0.7762]; group **10.2591** = oracle
**10.2591** (100 % ≥ 95 % ✓); worst **0.7543** (ch6) ≥ 0.6 ✓. The
certified placement **is** the raster-optimal unit-prototile
placement for every child (aligned == oracle per child). Where the
oracle argmax is tied, it prints a different (k,m) at the same IoU
(ch1: table (k10, m(0,0,0,0)) vs oracle (k4, m(1,1,−1,−1)); ch6:
table (k4, m(2,1,1,0)) vs oracle (k0, m(0,1,1,0))) — same values,
different lattice tiles; the table row is the exact edge-matched
assembly.

**Inter-piece offsets (NOT measured — stated honestly).** The
per-piece free translations leave the relative piece positions
unconstrained by the fit; the drawn gaps (≈ 1.9 H) are
artist-chosen. Drawn offsets in panel H (blob anchor ch1 →
island/pendant anchors): island (−4.1035, −4.1918) — off-lattice,
nearest (1,−3,1,−3) at 0.09275 H; pendant (4.1800, 4.1008) —
nearest (−1,3,−1,3) at 0.09672 H. Fitted offsets (drawn offsets
pushed through the inverse global similarity, exact frame): island
(−3.68628, −6.4722) → 0.04648 H from lattice (−2,−1,−3,−2); pendant
(3.69438, 1.00285) → 0.03778 H from (2,1,1,0). If an implementation
wants a connected rho supertile, the two snaps are candidates — the
figure does not decide between them.

**Method note (bug fixed, re-run clean).** The first fit ran a
transposed rotation model (y-row [cx, cy] of the (a,b) normal
equations — a reflection, not a rotation) and returned a garbage
(s, θ) that mimicked "the figure is not lattice-exact". Correct
model: x′ = a·x − b·y, y′ = b·x + a·y (y-row [cy, cx] in (a,b)).
With the fixed model the 8-unknown fit is a unique global LSQ
optimum; a bad fit now means inconsistent data, never a local
minimum. Clean re-run: JSON identical to the previous one modulo
time_s (22.0 s).

#### sq-sup — CERTIFIED, 98.39 % of oracle

26 children (16 tri + 4 sq + 6 rho), 2·area (56,32) exact, 20
boundary edges, simply connected (no hole: boundary 2·area = total
= (56,32)); 13 boundary / 13 interior children.

| ch | shape | krot (mod 12) | m (P,Q,R,S) | m in H-units |
|---|---|---|---|---|
| ch0 | rho | 2 | (−1,0,−2,−1) | (−1.0000, −3.7321) |
| ch1 | rho | 1 | (−1,0,−2,−1) | (−1.0000, −3.7321) |
| ch2 | rho | 0 | (−1,0,−2,−1) | (−1.0000, −3.7321) |
| ch3 | rho | 5 | (1,1,1,1) | (2.7321, 2.7321) |
| ch4 | rho | 4 | (2,2,0,0) | (5.4641, 0.0000) |
| ch5 | rho | 3 | (3,2,−2,−1) | (6.4641, −3.7321) |
| ch6 | sq | 1 | (0,0,0,0) | (0.0000, 0.0000) |
| ch7 | sq | 0 | (0,1,−1,0) | (1.7321, −1.0000) |
| ch8 | sq | 2 | (1,2,−2,−1) | (4.4641, −3.7321) |
| ch9 | sq | 0 | (1,2,0,1) | (4.4641, 1.7321) |
| ch10 | tri | 0 | (−2,0,0,0) | (−2.0000, 0.0000) |
| ch11 | tri | 1 | (−1,0,0,1) | (−1.0000, 1.7321) |
| ch12 | tri | 1 | (0,0,−2,0) | (0.0000, −2.0000) |
| ch13 | tri | 3 | (0,1,−1,0) | (1.7321, −1.0000) |
| ch14 | tri | 0 | (−1,1,−1,−1) | (0.7321, −2.7321) |
| ch15 | tri | 2 | (0,1,1,0) | (1.7321, 1.0000) |
| ch16 | tri | 3 | (1,1,−3,−1) | (2.7321, −4.7321) |
| ch17 | tri | 2 | (1,1,−1,−1) | (2.7321, −2.7321) |
| ch18 | tri | 0 | (0,1,1,0) | (1.7321, 1.0000) |
| ch19 | tri | 1 | (1,1,1,1) | (2.7321, 2.7321) |
| ch20 | tri | 3 | (1,2,0,1) | (4.4641, 1.7321) |
| ch21 | tri | 1 | (2,1,−1,0) | (3.7321, −1.0000) |
| ch22 | tri | 3 | (2,2,−2,0) | (5.4641, −2.0000) |
| ch23 | tri | 0 | (1,2,−2,−1) | (4.4641, −3.7321) |
| ch24 | tri | 2 | (2,2,0,0) | (5.4641, 0.0000) |
| ch25 | tri | 2 | (3,2,0,−1) | (6.4641, −1.7321) |

Pins (w06s_sq.json / w06s_run.log):

- Walk: N=20, k0=0, snap residual 4.992° (match_h raised 0.55→0.7
  for chain closure).
- Drift correction: A = [[1.083644, −0.002439], [−0.004552,
  1.084653]], t = (2.137367, 5.241340); area scale **1.084143**
  (principal 1.042841 / 1.039605, rotation −0.1290°); inliers 31/40;
  endpoint residuals min/med/max 0.0066 / 0.1395 / 0.5646 H (max
  8.29 px); per-child boundary drift median 0.1091 H, max 0.1854 H.
- Uniqueness: exactly **1** boundary solution (shape+krot edge
  filter, DFS 21 nodes); 5 interior solutions (DFS 53 nodes) —
  winner 20.4073 / worst 0.6684; losers 17.4732, 17.4567, 15.3461,
  15.3446 (worst 0.0000 each) → rejected.
- Raster acceptance (30 px/H, walk frame): per-child [0.6883, 0.6746,
  0.6684, 0.8724, 0.7353, 0.7436, 0.7449, 0.7947, 0.8649, 0.7385,
  0.7751, 0.7294, 0.7422, 0.7847, 0.7378, 0.7524, 0.8287, 0.8238,
  0.8261, 0.7937, 0.8367, 0.8218, 0.8069, 0.8403, 0.8825, 0.8996];
  group **20.4073** vs oracle **20.7403** (98.39 % ≥ 95 % ✓); worst
  **0.6684** (ch2) ≥ 0.6 ✓.
- Native panel frame (table translated by T = (−1,2,0,3) into the
  panel's native k-gauge, no scale/rotation): group 13.7421, worst
  0.2887 (ch0) — drift; not the acceptance frame.

### Figure vs patch inconsistency (measured 2026-11-23)

Patch-side ground truth: 37,635 parsed tiles (18,302 tri / 11,644
sq / 7,689 rho); 8,852 of them form the second interleaved layer
(edge vectors outside the M lattice — excluded); the clean,
rule-relevant population is **28,783** tiles.

- **tri-sup (w06v):** the certified table occurs **0 times** in the
  patch in all 12 rotations (exact clean-index corner-set matching).
  The patch contains 150 certified (7,2,2) supertile leaves =
  **15 congruence classes** (occupancies 21, 11×10, 8, 7, 3, 1); the
  table matches **0 of the 15**. (Prepass: no patch class overlays
  the panel either — best no-match group 3.987/11, 4 of 15 groups at
  3.9868, worst-child 0.000.)
- **rho-sup (w06u, w06t):** the panel blob (9-tile (6,2,1) hex,
  2·area (20,12), 10 internal / 10 boundary edges) matches **0 of
  5,472** patch (6,2,1) 9-tile hexes (census: 16,830 roots /
  523,523 growth nodes). Already topologically distinct: every patch
  hex has 8 or 9 internal edges (boundary 14 or 12 edges; census
  1008 + 45 certified), while the panel blob has 10. The patch's
  connected 13-tile (8,2,3) supertile (13 certified leaves, **one**
  congruence class; reference leaf clean indices [27256, 27259,
  27260, 27261, 27265, 27266, 27268, 28114, 28115, 28117, 28264,
  28265, 28266]) has boundary direction multiset
  {0°×4, 30°×4, 60°×2, **90°×4**, 120°×2, 150°×4}; the drawn panel
  cluster (I=12, B=20) has **no 90° edges** ({0°×4, 30°×8, 60°×4,
  120°×2, 150°×2}) — congruence-distinct on direction multisets
  alone. The 2-tile motifs *do* occur: island (rho k1 + tri k4 at
  (2,1,1,0)) 1,147 times, pendant (rho k1 + tri k2 at (0,0,0,0))
  1,105 times, each in 6 of 12 global rotations (kr is identified
  only up to shape symmetry — tri period 4, rho period 6 — so the
  totals count each physical motif several times; read them as
  presence, not census). The full 13-tile assembly with
  nearest-lattice inter-piece snaps (0.04648 / 0.03778 H): **0 in
  all 12 rotations**.
- **sq-sup:** **0** closed 26-tile (16,4,6) patches in the crop
  (exhaustive composition-growth: 792,000 nodes, 0 node-cap hits,
  all 47 feasible components).

**Conclusion:** the rule figure and the patch are generated by
**different rule sets** — exactly as the entry page's own
provenance note anticipates ("the corresponding substitution rules
given in Fig. 2(iii) of the paper are not unique … the correct set
was submitted by Musesti and Paolini in January 2023"). The figure's
tables are a valid rule set for the tile set (each rule is a closed,
edge-paired, exact-area cluster); they are not the patch's rule set.

### Implementation consequence (decision for the orchestrator)

1. **Ship the figure's rules.** Valid and fully certified (exact
   assembly, area law, λ² law). But the generated tiling is *not*
   the Bielefeld patch; the module header must state that and the
   provenance (§Rule provenance), and any census/pattern guards must
   be written against the figure's own invariants, not the patch's.
   The rho rule ships a **3-piece disconnected cluster**; inter-piece
   offsets are free (use the two lattice snaps above, or keep the
   pieces as drawn).
2. **Defer** until a machine-readable MP-2023 source appears (none
   exists as of this commit — w07_sources.md, §GAPS).

The prepass candidate tables (closest patch-side occurrences) are
superseded and removed from this brief; they remain in the prepass
checkpoint (`.scratch/wis12-recon/NOTES.md` +
`s18_rule_candidates.json`) for the record.

### Verification battery (measured pins — what "done" means at
implementation)

Items 1–4 and 6–8 are research-side pins (scratch:
`/tmp/opencode/wis12-unblock/`; read-only checkpoint
`.scratch/wis12-recon/`); item 5 (in-module Rust) ships with the
implementation.

1. **Composition census** (s18 composition-growth,
   ordered-frontier connected-set DFS + exact ℤ[√3] certification):
   tri-sup 150 certified leaves / 15 congruence classes; rho-sup
   13 / 1 class in crop; sq-sup 0 (exhaustive). The pinned figure
   clusters all certify (composition + area identity, closed simple
   boundary, all interior edges paired) — verified this session for
   all three (per-piece for rho). Re-run on any new cluster source.
2. **Raster-IoU acceptance** (per-child self-overlay vs the
   `rule.png` panel raster, table and oracle in the same
   affine-corrected frame, 30 px/H cell-center). Criterion, verbatim:
   "group sum within 0.95 of oracle **and** worst-child ≥ 0.6."
   **MET:** tri 8.8751 / 8.9036 (99.68 %), worst 0.7547; rho
   10.2591 / 10.2591 (100 %), worst 0.7543; sq 20.4073 / 20.7403
   (98.39 %), worst 0.6684. Native panel-frame numbers (for
   reference, NOT the acceptance frame): tri 6.4551 / worst 0.3127
   (T=(1,1,−3,4)); sq 13.7421 / worst 0.2887 (T=(−1,2,0,3)); rho
   1.4025 / worst 0.0000 (translation-only). The shortfall there is
   k-miscalibration plus the ~1.04×/≤0.19° panel drift.
3. **Panel fit** (s08): per-child IoU 0.619–0.923; k within 5 % of
   the area-derived median (14.5644 / 14.6088 / 14.6796 px/H);
   M-snap residual ≈ 0.02–0.05 H ⇒ panel child placements are
   lattice-snapped.
4. **Exact area identities** (ℤ[√3], 2·area): tri (24,14) = (24,10)
   boundary + (0,4) hole; rho (28,16) = (20,12) + (4,2) + (4,2)
   (three pieces); sq (56,32) simply connected. Zero excess, S = 0.
5. **λ² law:** λ² · 2·area(parent) = 2·area(supertile) per row
   ((7,4)·(0,2) = (24,14); (7,4)·(4,0) = (28,16); (7,4)·(8,0) =
   (56,32)), exact.
6. **Bielefeld patch census:** 18,302 tri / 11,644 sq / 7,689 rho
   (37,635 total; 28,783 clean-index; cross-check report only,
   `patch.pdf`).
7. **Substitution mechanics** (in-module Rust tests, mirroring
   `glb-binary-n7.md` battery shape): level-1 assembly per shape
   (closed boundary, edge pairing, count vector = composition; rho:
   3-piece cluster), depth-3 assembly passing validate-level-
   equivalent checks, orientation census (all 12 dodecagonal edge
   directions present, per-shape krot distribution stable across
   levels), area law per rule line, PF = λ² = 7+4√3 to float
   tolerance.
8. **Figure/patch fingerprint (research-side guard):** the zeros are
   the fingerprint that the tables are the figure's rules, not the
   patch's — tri 0/15 classes (150 leaves), rho blob 0/5,472 hexes
   and full-13 0/12 rotations, sq 0 in crop. Any change to a table
   must re-run both the acceptance (item 2) and these zeros.

### Files you own (implementation dispatch)

- `crates/tess-tilings/src/families/quasiperiodic/wis_12_fold.rs` —
  the new module (proposed name; registry id to be assigned in the
  family doc row).
- `crates/tess-tilings/src/families/quasiperiodic.rs` — module decl,
  `register` slice, `classification` arm, census `IDS`.
- `docs/families/quasiperiodic.md` — catalog row (guard-checked);
  build-arc entry only if the unblock introduces a new technique.
- `/tmp/opencode/wis12-unblock/` — scratch (Python spike, batteries;
  read-only checkpoint `.scratch/wis12-recon/`).
- This prepass document itself: `docs/briefs/wis-12-fold.md`
  (docs-only commit).

**Do not touch anything else** — not `lib.rs` (census pin, guards,
culling test), not README, not `docs/INDEX.md`, not
`docs/history/CHANGELOG.md` (M number written by the orchestrator),
not other families, not the UI crates.

### Constraints (from AGENTS.md)

1. Exact geometry: vertices as (P,Q,R,S) integer tuples over the
   basis above (ℤ[√3] pairs (a,b) = a + b√3 for areas) — never
   decimal literals.
2. Verify numerically BEFORE writing Rust: Python spike, **stdlib
   only**, in `/tmp/opencode/wis12-unblock/`, foreground with
   per-step output, ≤ 240 s per experiment.
3. Emission: when several geometric tiles of one class share a
   vertex, emit each geometric tile exactly once (AGENTS.md step 3).
4. Tile budget `MAX_TILES = 250_000`; keep generators cheap.
5. Keep `cargo check --workspace` warning-free.
6. **Census deltas re-derived against main at dispatch time**
   (prepass snapshot: built-ins 141 → 142; memberships 155 → 156;
   quasiperiodic 12 → 13).
7. **No repo-wide `cargo fmt`** (convention: `cargo fmt` defaults
   only, no rustfmt/clippy config, no pre-commit hooks).
8. **Orchestrator census guards report verbatim, never fix:** the
   `lib.rs` guard tests (`registry_groups_are_complete_and_canonically_ordered`,
   `family_docs_match_registrations`, `readme_catalog_matches_registry`)
   are orchestrator-owned; a
   red guard goes into the report unchanged, and `LatticeTiling`
   culling-test lines are flagged for the orchestrator, not added.
9. No M number in commits, docs, or the report; the orchestrator
   assigns it at integration.

### Standard tests

The 3-test trio every tiling ships (deterministic generation, census
consistency, render smoke) — copy the shape from
`golden_rhomboid.rs`'s test module — plus the battery above as
in-module tests.

### Deliverable (worker report)

Per the fleet protocol in `tessellix/AGENTS.md`:

1. Diff-scope statement (exactly the owned files).
2. Verification evidence with exact numbers and pass counts
   (battery §1–§8 above; programmatic only — a visual look is
   recon, never evidence).
3. Expected census deltas re-derived against current main
   (prepass snapshot: family 12 → 13, built-ins 141 → 142,
   memberships 155 → 156; `LatticeTiling`? → remind the
   orchestrator about the culling line).
4. Proposed commit-message body (basis, cluster contents,
   verification).

---

## Appendix — recon & unblock scratch

Read-only prepass checkpoint:
`/home/pcharles/dev/pchuck/tesselation/.scratch/wis12-recon/`

| file | role |
|---|---|
| `NOTES.md` | append log s00–s18e (every step, dead ends, bug fixes) |
| `s09_cert.py` | exact ℤ[√3] certification module (`certify`, `boundary`, `CORN`, `AREA_SH2`, `rot4`, …) |
| `s08_panels.py` / `s08_panels.json` | rule.png raster panel fit (anchors, k, per-child m + krot + iou) |
| `s07_verts.json` | 47,163 deduped vertices (snap grid source) |
| `tiles.json` | 37,635 parsed patch tiles (28,783 clean-indexable) |
| `s18_compgrow.py` + `s18_compgrow_{tri,rho,sq}-sup.json` | composition-growth solver + results (150/13/0 certified) |
| `s18b_analyze.py` + `s18_tri-sup.json` | translation-canonical congruence grouping + s08 cross-check |
| `s18d_iou.py` + `s18d_{tri,rho}-sup.json` | raster-IoU discriminator (oracle + no-match numbers) |
| `s18_rule_candidates.json` | the prepass candidate child tables (superseded, for the record) |

Unblock scratch (this session): `/tmp/opencode/wis12-unblock/`
(⚠ volatile — the durable record of every certified number is this
brief):

| file | role |
|---|---|
| `w05_panels.py` / `w05_panels.json` | rule.png panel decode (shapes, hulls, k-candidates, contacts) — the decode the tables rest on |
| `s09_cert.py` | exact certification module (copy of the checkpoint's) |
| `w06r_tri_final.py` / `w06r_tri.json` / `w06r_tri.log` | tri-sup certification (walk, affine drift correction, DFS, scoring) |
| `w06s_sqsup.py` / `w06s_sq.json` / `w06s_run.log` | sq-sup certification |
| `w06t_rho.py` / `w06t_rho.json` / `w06t_run.log` | rho-sup certification (stub merge, shared-edge DFS, global fit, patch occurrences) |
| `w06u_patchrho.py` / `w06u_patchrho.json` | patch rho-sup 13-cluster (tiles + 12 internal edges) + blob9 census (5,472 hexes, 0 panel matches) |
| `w06v_patchmatch.py` / `w06v_patchmatch.json` | tri panel table vs patch (0 occurrences; 15 classes) |
| `w07_sources.md` | published-source hunt (WSI95, MP-2023 negative, arXiv cousin, Bridges) |
| `w06b_solve.py` … `w06p_affine.py` | intermediate attempts and dead ends (hard-intersection DFS, union-candidate DFS, CSP, 15-px matching, affine diagnostics) — for the record |
