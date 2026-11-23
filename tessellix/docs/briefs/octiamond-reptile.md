# Brief: octiamond-reptile — Bielefeld Octiamond Reptile substitution (λ = 3)

> **Status of this brief (2026-11-22):** research-only recon is COMPLETE;
> the rule is pinned and triple-confirmed (unique partition of the
> supertile + per-component pixel pin against the Bielefeld rule image +
> cell-set equivalences vs the Bielefeld patch panels). All numbers below
> are MEASURED (stdlib Python, exact integer cell arithmetic on the
> triangular lattice; evidence chain in
> `/tmp/opencode/octiamond-recon/NOTES.md`, outputs s27–s36). No part of
> the pin rests on a visual guess.
>
> **Category / translational decision (evidence-fixed, do not flip):**
> the tiling lands in the **Nonperiodic tab** (it is a non-repeating
> realization of a tile that admits periodic tilings — the reptile
> property), and its translational status is **"aperiodic" — NOT
> "limit-periodic"**: the Bielefeld patch (all 4 crops of the same
> tiling) admits no lattice period in any of the 4 fitted-lattice frames
> (recon s25/s25b). The `rep_tile` helper's "limit-periodic" string
> (chair/sphinx/armchair/P-pentomino) does not apply to this entry. If
> your battery produces evidence that the realized tiling IS
> limit-periodic with a measured growing period manifest, **STOP and
> report first** (period-tripling-2d errata precedent) — do not land
> the flip silently.

**Family:** `reptiles` · **Candidate:** Bielefeld Tilings Encyclopedia
entry "Octiamond Reptile",
<https://tilings.math.uni-bielefeld.de/substitution/octiamond-reptile/>
(slug `octiamond-reptile`) · **Effort:** M · **Precedent to mirror:**
`families/reptiles/sphinx.rs` (read ALL of it first — the tabulated
12-symmetry group, `CANONICAL_CELLS`, `CHILDREN` rule table, seed
blocks on an origin-anchored block lattice, `deflate_step` per-step
culling, depth quantization, unit-scale emission, and the full test
battery) + `families/reptiles.rs` (register slice, classification arm
with the **tangram inline-class precedent** L43–55, census `IDS`,
per-tiling test trio).

### Goal

Land the Bielefeld **octiamond reptile** under registry id
`octiamond-reptile` in the `reptiles` family: the trapezoidal 8-triomino
of the triangular lattice (the "octiamond", 8 unit equilateral
triangles), a **rep-9 reptile at λ = 3** — the 3×-inflated 3Q
trapezoid (72 unit triangles) admits **exactly one** partition into 9
unit octiamonds (rule forced by exhaustive search, unique), the
substitution is iterated on a 12×6 parallelogram seed block (18 tiles),
depth quantized to powers of 3 (tri-section-anchored; the λ = 3 analog
of the chair family's octave anchoring), final tiles emitted at unit
scale. Headline properties: 1 prototile, substitution matrix [9]
(9^n tiles per supertile at depth n; PF eigenvalue 9 = λ²), the
dissection mixes chirality (all 12 orientation classes reachable from
the seed), aperiodic.

### Literature basis (rule pinned in recon, 2026-11-22)

- **Source:** the Bielefeld entry above. Entry metadata (parsed from the
  page): **"Number of Prototiles: 1"**, **"Discovered by: Folklore"**;
  the Info section is **EMPTY** (no tags, no text, **no references** —
  contrast the sphinx entry which cites [LM01]). The entry ships a rule
  image (9 child positions, one marked blue) and a large patch image.
  → the pin rests entirely on the image + patch evidence below; the
  classification arm cites `source: Some("Bielefeld substitution
  index")` (entry's own "Folklore" attribution is not a citable
  source); `year: None`.
- **Shape facts (exact, on the triangular lattice; world = q·e1 + r·e2,
  e1 = (1,0), e2 = (1/2, √3/2), i.e. world(q,r) = (q + r/2, (√3/2)·r)):**
  - prototile **Q** (trapezoidal octiamond): lattice vertices
    `(0,0), (3,0), (1,2), (0,2)` — world `(0,0), (3,0), (2,√3), (1,√3)`;
    8 unit triangles, area 2√3.
  - supertile **TRAP3Q** (the 3×-inflated seed trapezoid): lattice
    vertices `(0,0), (9,0), (3,6), (0,6)`; 72 unit cells, area 18√3.
  - 9Q: lattice vertices `(0,0), (27,0), (9,18), (0,18)`; 648 cells,
    area 162√3.
  - λ = 3; the supertile is exactly 9 unit Q tiles (PF = 9 = λ²).
  - **Seed block:** the 12×6 lattice parallelogram
    `TRAP3Q ∪ R180-about-(6,3)(TRAP3Q)` — 144 cells = 18 Q tiles,
    disjoint union verified; **tiles the plane exactly once under the
    block lattice ⟨(12,0), (0,6)⟩** (programmatic check: 0 missing / 0
    duplicate cells over a 48×36 lattice window, recon s36). This is
    the implementation's seed (analog of sphinx's 2×3 rhombus).
  - **Q has a 2-representation degeneracy:** every cell set is
    represented by exactly 2 (op, translation) pairs (verified), so
    op-labels of a partition are ambiguous — all recon matching was
    done on exact **cell sets**, not labels.
- **The pinned rule** (I-frame: parent Q at the origin, identity
  orientation; children are unit Q tiles; op-index map
  I=9, R60=5, R120=6, R180=2, R240=1, R300=10, M0=11, M1=7, M2=3,
  M3=0, M4=4, M5=8 — the same 12-symmetry convention as
  `sphinx.rs::SYMS`; child cell set = `apply_sym(op, cells(Q)) +
  (q,r)`):

  ```text
  R240 + (9,0)    R180 + (4,2)    M4  + (2,6)    R120 + (0,5)
  I    + (0,0)    I    + (2,2)★   I    + (2,4)    I    + (4,0)    R300 + (3,4)
  ```

  op-index tuples (sorted, as in the recon engine):
  `(1,9,0), (2,4,2), (4,2,6), (6,0,5), (9,0,0), (9,2,2), (9,2,4),
  (9,4,0), (10,3,4)`.  **★ = I+(2,2) is the child the Bielefeld rule
  image marks blue** (pixel pin, score 0.991 — see below).
  Conjugation to arbitrary parent classes (mirror `sphinx.rs`): child
  class = `compose(parent, child_op)`; child anchor in the parent's
  frame = `apply_sym(parent, anchor)`, inflated by λ = 3 (sphinx's
  `(s/2)` child-scale factor becomes `(s/3)`; keep integer arithmetic —
  scale s is a power of 3 so s/3 stays an integer).
- **Evidence chain (all programmatic; details in NOTES.md s27–s36):**
  1. **s27 patch panel census.** The 4 Bielefeld patch crops (one
     tiling: all 4 groups share the device scale e1 ≈ (14.1725, 0.0)px,
     e2 ≈ (7.0865, 12.2743)px, different origins) contain **1142
     9-tiled TRAP3Q panels** (g0 193 / g1 468 / g2 302 / g3 179),
     forming **6 table types** (6 conjugacy orbits of size 12, one type
     per orbit, all 6 mutually non-conjugate) with frequencies
     **259 / 196 / 186 / 183 / 162 / 156**. All 6 types are L1-clean
     AND L2-clean under the corrected λ-scaled framing (each: 0
     overlaps, 648/648, 0 holes, 0 spills, 81 distinct grandchildren)
     — the patch contains iterable substitution structure.
  2. **s33 — THE PIN.** Backtracking over all 12 ops × the 9 image
     sub-pixel anchor positions (positions trusted from the image,
     residuals < 0.25; cell-mask pruning) finds **EXACTLY ONE**
     partition of TRAP3Q: the table above. It is L1-clean (72/72,
     union == TRAP3Q, max coverage 1) and L2-clean (0 overlaps,
     648/648, 0 holes, 0 spills, 81 distinct grandchildren, 0 dup
     pairs).
  3. **s35 — per-component pixel pin.** Each of the 9 rule-image
     components matched against all 12 ops × 9 positions: every
     component's best match is exactly its s33 op/position, scores
     **0.911–0.991**, all runners-up **≤ 0.892**; the reconstructed
     table == the s33 table (True) and ≠ type-E table (False).
     The blue-marked component is **I+(2,2)** (score 0.991).
  4. **s34 — cell-set equivalences + panel census.** `M5+(6,3)` ≡
     `R240+(9,0)` (identical 8-cell sets — the 2-representation
     degeneracy in action) and `M0+(1,2)` ≡ `R180+(4,2)`; the rule's
     D6-conjugacy orbit (12 labeled tables) contains **ZERO** labeled
     patch panels in any group; the type-E panel (×186, the "7shared"
     type) shares **all 9 slots with the rule as cell sets** (7 exact
     tuples + 2 via the 2-rep degeneracy). Entry-sharing histogram over
     all 1142 panels: 1shared 638 / 5shared 318 / 7shared 186
     (per-group 7shared = 43/64/59/20).
  5. **s28 framing fix, cross-validated on the sphinx rule.** Correct
     L2 nesting: L2 panel i = `M_i·T + λ·t_i` (here λ = 3, T = TRAP3Q,
     t_i = the child translation); grandchild (i outer, j inner) =
     `(M_i∘M_j, M_i·t_j + λ·t_i)`. Validated on the sphinx (λ = 2)
     rule: clean 0/96/0/0 with λ·t_i, defective 45/63/96 with 1·t_i.
     **Any earlier finding using unscaled t_i is INVALID.** Under the
     corrected framing the corrupted image-inherited "pinned" table
     (recon s22) is defective: L1 4 duplicate cells, L2 7 overlaps /
     599 of 648 / 129 holes / 80 spills — and no single-child ±1
     repair exists (s30, REJECTED). Do not resurrect it.
  6. **Coloring convention (for report/interpretation only, not
     emission):** the patch colors **unit triangles** (cell level):
     tan / blue, blue = the 3^g-descendant union of the marked child
     **I+(2,2)**; tiles commonly carry BOTH colors (a per-slot census
     shows mixed ~50/50 fractions) — the colors are NON-DISCRIMINATING
     for rule identification (s31) and exist to display the fractal
     descendant structure, not to label tiles.
  7. **s25/s25b:** no lattice period in any patch crop → aperiodic
     (see errata block at top).
  8. **T-junctions:** **ERRATUM (integration adjudication 2026-11-22):**
     the original "0 incidences, edge-to-edge" pin was WRONG — the
     recon s36 measurement ran on a unit-step boundary cycle (every
     edge pre-split to length 1, so no corner can be strictly interior:
     a vacuous metric that reads 0 for ANY patch, including sphinx
     gen-1, contradicting the repo-pinned sphinx value 2). Hand-checked
     counterexample: (4,2) is a corner of R180+(4,2) and I+(4,0) and
     lies strictly interior to I+(2,2)'s bottom edge (2,2)→(5,2).
     MEASURED truth (machinery validated on the sphinx ladder
     0/2/16/78/344, two independent machineries agreeing): the rule's
     own L1 dissection has **8** distinct T-points / **16**
     (corner, edge) incidence pairs — non-edge-to-edge like the sphinx;
     the battery pins the per-generation census as measured
     (gen-0 = 8; seed-block ladder 18/226/2202/20286/183974).
- **Closed form:** one prototile → substitution matrix **[9]**; a
  supertile at depth n yields **9^n** tiles; PF eigenvalue 9 = λ² with
  **λ = 3**. (The 12×12 orientation-class matrix M has every row and
  every column summing to 9 — measured; the class growth from a uniform
  seed is uniform, sphinx-style "doubly stochastic ×9".)
- **Translational / category:** `TilingCategory::Nonperiodic`
  (module, mirror `sphinx.rs` L379–381); `translational:
  Some("aperiodic")`; `aperiodic_status: None` (not established — the
  tile set admits periodic tilings, so einstein-type claims do not
  apply; do not invent one).

### Classification arm (build INLINE in `reptiles.rs` — do NOT touch
`classify.rs`)

`crate::classify::rep_tile` hardcodes `inflation "λ = 2"` and
`translational "limit-periodic"` — WRONG for this entry (λ = 3,
aperiodic). Do **not** call it and do **not** modify `classify.rs`.
Mirror the **tangram inline-class precedent** (`reptiles.rs` L43–55):

```rust
"octiamond-reptile" => TilingClass {
    families: &["Nonperiodic"],
    method: Some("substitution"),
    inflation: Some("λ = 3"),
    prototiles: Some(1),
    tile_set: Some("octiamond reptile (trapezoidal 8-triomino of the triangular lattice)"),
    reflections: Some(true),
    chirality: Some("mixed"),
    translational: Some("aperiodic"),
    source: Some("Bielefeld substitution index"),
    ..TilingClass::EMPTY
},
```

- `reflections: Some(true)` / `chirality: Some("mixed")`: the rule's
  9 children include one reflection (M4) alongside rotations, and the
  seed (below) closes under composition onto **all 12 orientation
  classes** (measured) — the dissection mixes chirality (sphinx-style).
- If a guard or the orchestrator wants a year, there is none:
  `year: None` (entry has no reference).

### Files you own

- `crates/tess-tilings/src/families/reptiles/octiamond_reptile.rs` —
  the new module (mirror `sphinx.rs`'s structure: tabulated 12-symmetry
  group + composition lookup, `CANONICAL_CELLS` for Q, the `CHILDREN`
  rule table (the pinned I-frame table above; conjugation convention
  as stated above, with the λ = 3 child-scale factor `(s/3)` instead of
  sphinx's `(s/2)`), seed = the 12×6 parallelogram block (18 tiles:
  the 9 s33 children plus their R180-about-(6,3) images, on the
  origin-anchored block lattice), per-step culling, depth quantized to
  **powers of 3** from the view extent (tri-section-anchored; final
  tiles land at unit scale, area 2√3), `proto` always 0, `seed` =
  class + 1 (1..=12) for pattern mode, plus the in-module battery
  below).
- `crates/tess-tilings/src/families/reptiles.rs` — module decl +
  `register` slice + the inline classification arm above + census
  `IDS` (add `"octiamond-reptile"`) + the family census assert
  (`r.len()` 6 → 7, the worker-updated family-owned pin) + the
  per-tiling standard-trio test block (mirror `sphinx_tests`, boxes
  ±8×±6 / 0.6·25°·±7 / ±5).
- `docs/families/reptiles.md` — the catalog row (draft below) in the
  "Rep-tiles" table + the heading `## Implemented (6 tilings)` →
  `## Implemented (7 tilings)`. Row-only is fine (no new build arc —
  the build reuses the exact-integer substitution playbook verbatim).
- `/tmp/opencode/octiamond-recon/` — scratch (append-only NOTES.md
  already holds the full evidence chain + spike outputs).

**Do not touch anything else.** Not `lib.rs` (the per-tab census
`vec![11, 20, 28, 17, 15, 25, 8, 13, 11, 7]`, the `r.len() == 141`
built-in pin, the `memberships == 155` pin, the README/INDEX guard
tests — those are orchestrator-owned), not `classify.rs`, not README,
not `docs/INDEX.md` (family size 6 → 7 there is the orchestrator's
integration edit), not `docs/history/CHANGELOG.md` (the M number and
its entry are written by the orchestrator at integration), not
`families/reptiles/sphinx.rs` or any sibling module (read-only
references — copy patterns from them), not other families, not the UI
crates. Adding a tiling never requires UI code; if it seems to, stop
and report.

**Commit exception (this brief only):** one worktree commit, subject
`reptiles-family: octiamond reptile (Bielefeld substitution, λ = 3)`
— NO M number, NO pushes. The orchestrator cherry-picks and assigns
the M number at integration.

### Constraints (the ones that bite here)

1. Exact ℤ lattice arithmetic end to end (lattice cells `(kind, q, r)`,
   i64; world = (q + r/2, (√3/2)·r) only at the emission boundary —
   `H = √3/2` constant, never decimal literals, sphinx pattern).
2. `MAX_TILES = 250_000`: a full-depth-g patch emits 18·9^g tiles —
   g = 4 → 118098 (in budget), g = 5 → 1062882 (over). Per-step
   culling (sphinx `deflate_step` pattern) keeps views cheap; measure
   and pin the effective depth clamp in the battery.
3. `cargo check --workspace` zero warnings; `cargo fmt` **owned hunks
   only** — do NOT run repo-wide `cargo fmt` (the repo is not
   fmt-clean; revert any non-owned byproducts). `cargo test
   --workspace` green **except the two orchestrator-owned census
   guards** (`registry_groups_are_complete_and_canonically_ordered`
   and `readme_catalog_matches_registry`): they fire on every landing
   by design — capture their **verbatim** failure output as your delta
   evidence and **never fix them yourself**.
4. **Stale-base warning:** you branch from a base with **reptiles
   family 6 / Nonperiodic tab 7 / built-ins 141 / memberships 155**
   (2026-11-22), but other landings may move these totals while you
   work. **The deltas transfer, the absolutes never do** — never
   hard-code base totals; report exactly what your guards printed and
   the orchestrator re-derives deltas at integration. Expected deltas
   **for this tiling**: reptiles family 6 → 7 (you update the
   family-owned census + `IDS`); Nonperiodic tab 7 → 8; built-ins 141
   → 142; memberships 155 → 156 (orchestrator-owned — guards fire,
   you report).
5. Registry id `octiamond-reptile` (no collision — verified against
   `crates/`, `docs/families/`, README, INDEX at base on 2026-11-22:
   `grep -rn "octiamond"` → zero hits).
6. **STOP conditions:** if any battery pin below disagrees with the
   measured value, if the rule table does not reproduce in Rust from
   the pins, or if the evidence flips the translational/category
   decision (errata block) — STOP, record the blocker, and report;
   never guess.

### Battery spec (all numbers below are MEASURED in recon — re-verify
each in the Rust tests; every value is a pin)

Recon ran a stdlib Python spike (≤240 s per experiment, exact integer
cell arithmetic; all outputs in `/tmp/opencode/octiamond-recon/`,
NOTES.md s27–s36). Rust test list (mirror the `sphinx.rs` battery,
sphinx test names annotated):

1. `symmetry_table_is_the_triangular_group` (mirror sphinx L505) — the
   12-op composition table closes; every op permutes the 6 unit-edge
   directions; det ±1; `compose(9, ·) = ·`.
2. `emitted_outlines_are_octiamonds` (mirror sphinx L544) — emitted
   tile outlines vs an INDEPENDENT boundary-walk derivation, all 12
   classes; mirrored classes emit the reversed canonical outline
   (CCW everywhere).
3. `substitution_tiles_parent_exactly` (mirror sphinx L653) — for
   EVERY parent class (conjugated table): 9 children × 8 unit
   triangles = 72 cells, pairwise disjoint, union == the 3×-inflated
   parent's 72-cell trapezoid (L1 pins: I-frame max coverage 1, 0
   overlaps — s33).
4. `l2_nesting_audit` (the corrected framing — s28/s36) — L2 panel i =
   `M_i·TRAP3Q + 3·t_i` (λ = 3 scale on the child translations —
   mandatory; the unscaled 1·t_i variant is defective and must not be
   used): **0 overlaps, union 648/648, 0 holes, 0 spills**;
   grandchildren `(M_i∘M_j, M_i·t_j + 3·t_i)`: **81 entries, 81
   distinct, 0 exact-dup pairs** (s33/s36; ERRATUM 2026-11-22: the
   original `3·t_j` was an index error — it collapses to 54 distinct
   grandchildren; the correct λ-scaled composition carries the OUTER
   child's anchor at scale 3, derived independently at integration). The framing was
   cross-validated on the sphinx λ = 2 rule (clean 0/96/0/0 with
   λ·t_i; 45/63/96 without) — if this audit fails while the sphinx
   control passes, STOP and report.
5. `seed_block_tiles_parallelogram` (mirror sphinx `seed_block_tiles_rhombus`)
   — the 12×6 block `TRAP3Q ∪ R180-about-(6,3)(TRAP3Q)`: 144 cells,
   disjoint union, tiles the plane exactly once under
   ⟨(12,0), (0,6)⟩ (0 missing / 0 dups over a 48×36 lattice window —
   s36). Seed class multiset pin (18 tiles; ERRATUM 2026-11-22 — the
   original list summed to 15 ≠ 18; corrected to the measured,
   census-verified multiset, op-index: count):
   {1:2, 2:5, 4:1, 5:1, 6:1, 7:1, 9:5, 10:2} — i.e. R240×2, R180×5,
   M4×1, R60×1, R120×1, M1×1, I×5, R300×2.
6. `growth_battery` (mirror sphinx `refined_growth_battery`) —
   generations 0–4 from the seed at a fixed scale (sphinx uses 2^4;
   for λ = 3 use 3^k with k large enough that the final tiles are
   unit scale): a perfect unit-triangle census of the SAME inflated
   parallelogram every generation; count law **18·9^g** (g = 0..4:
   18, 162, 1458, 13122, 118098); class distribution = M^g applied to
   the seed multiset (M: 12×12, rows AND columns summing to 9 —
   pin the row/column sums in-test); total area invariant; refined
   edge census (every segment shared by exactly 2 tiles or on the
   patch boundary).
7. `t_junctions_census` (mirror sphinx `t_junctions_are_inherent`) —
   gen-0 (the rule's own dissection) = **0** incidences (measured);
   measure and pin the deeper-generation values in-test and report
   them (sphinx's were 0, 2, 16, 78, 344 — this entry may be 0 at
   every scale; report what you measure, do not assume).
8. `orientation_closure` (new, measured s36) — the reachable
   orientation classes from the seed under the rule = **all 12**
   (mixed chirality; pin the closure).
9. `patch_is_stable_under_view_changes_within_a_trisection` (mirror
   sphinx's octave view-stability, L905) — bit-exact tiles under
   grow/pan/zoom within one tri-section (the λ = 3 quantization).
10. `emitted_tiles_are_unit_scale` (mirror sphinx L982).
11. Standard trio in `reptiles.rs` (mirror the `sphinx_tests` block
    verbatim in shape): `octiamond_reptile_covers_region` on the
    ±8 × ±6 box (SCALE_ID 1.0), `octiamond_reptile_rotated_scaled`
    (scale 0.6, 25°, ±7 box), `octiamond_reptile_deterministic` (±5
    box).

Recon-only context (NOT Rust pins, for your report): the 6 s27 patch
table types and their frequencies (259/196/186/183/162/156, all
L1+L2-clean under the corrected framing), the zero rule-orbit labeled
panels, the type-E 9/9 cell-set sharing (7 exact + 2 via the 2-rep
degeneracy), the s35 per-component pixel scores (0.911–0.991 best;
runners-up ≤ 0.892; blue = I+(2,2) @ 0.991), the closed-form counts
9^n, and the aperiodicity of the patch crops (no lattice period,
s25/s25b).

### Catalog row (draft for `docs/families/reptiles.md`)

Add to the "Rep-tiles" table; update `## Implemented (6 tilings)` →
`## Implemented (7 tilings)`. End the row with the `(M#)` placeholder
as drafted — the orchestrator replaces it with the assigned M number at
integration; do NOT commit an M number:

| `octiamond-reptile` | Octiamond reptile (trapezoidal 8-triomino) | λ = 3 rep-9 substitution on the triangular lattice (Bielefeld substitution index entry, "Folklore" — Info section EMPTY, no references; rule reconstructed from the rule image and pinned by a UNIQUE exhaustive partition of the 3×-inflated 72-cell seed trapezoid into 9 unit octiamonds, cross-confirmed per-component against the Bielefeld rule image, scores 0.911–0.991 vs runners-up ≤ 0.892; the blue-marked child is I+(2,2)); 1 prototile Q = trapezoid (0,0),(3,0),(2,√3),(1,√3) (8 unit triangles, area 2√3); supertile TRAP3Q (72 cells), PF = 9 = λ²; substitution matrix [9] → 9^n tiles per supertile; dissection mixes chirality (children: 4×I, R120, R180, R240, R300, M4 — all 12 orientation classes reachable from the seed; 12×12 class matrix, rows and columns sum to 9); edge-to-edge depth-1 subdivision (0 T-junction incidences); 12×6 parallelogram seed block = TRAP3Q ∪ R180 image (18 tiles, 144 cells), tiles the plane under ⟨(12,0),(0,6)⟩; L2 nesting verified under the λ-scaled framing (0 overlaps, 648/648, 81 distinct grandchildren, framing cross-validated on the sphinx rule); the Bielefeld patch's 1142 9-tiled trapezoid panels form 6 table types (frequencies 259/196/186/183/162/156, one per conjugacy orbit of 12), all L1+L2-clean — iterable substitution structure in the patch; aperiodic (no lattice period in any of the 4 patch crops); tri-section-anchored depth quantization, unit-scale emission (M#) |

### Deliverable (worker report)

1. Diff-scope statement (exactly the owned files, nothing else).
2. The pinned construction: entry URL, the 9-entry rule table, the
   conjugation convention, the seed block, the closed form (matrix
   [9], 9^n, λ = 3), the chirality/orientation census.
3. Battery numbers with exact values (the 11 Rust tests above, pass
   counts; the measured T-junction census; the measured depth clamp).
4. Expected census deltas **as printed by your guards** (base totals +
   your +1s): reptiles family 6 → 7 (family-owned, you update);
   Nonperiodic tab 7 → 8; built-ins 141 → 142; memberships 155 → 156
   (orchestrator-owned — the two guards `registry_groups_are_complete_
   and_canonically_ordered` and `readme_catalog_matches_registry` fire;
   report their verbatim output, never fix them). Not a `LatticeTiling`
   → no culling-test lines.
5. Proposed commit-message body (construction, verification, battery
   numbers). NO M numbers, NO pushes.
