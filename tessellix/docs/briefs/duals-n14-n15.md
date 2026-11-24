# Brief: duals-n14-n15 — §23k audit-discovered 2-uniform duals (triangle pair)

> **LANDED as M147** (see docs/history/CHANGELOG.md). Do not dispatch.

**Family:** `kuniform` (the duals register in the kuniform family per the
M145 precedent — they DISPLAY on the Laves tab via their category) ·
**Candidate:** PROPOSED.md §23k — the second missing subscripted pair,
the duals of (3⁶; 3³.4²)₁/₂ = `kuniform-n14` (M103) and `kuniform-n15`
(M108); the first pair (n4/n3, M145) just landed and is the FRESHEST
mirror · **Effort:** M (2 landings, one commit each) · **Precedent to
mirror:** `families/kuniform/dual.rs` — read `kuniform_dual_n4` and
`kuniform_dual_n3` (M145, the most recent: `with_checker_seeds` helper,
Case entries, test trios, the glide-closure probe now inside
`duals_match_measured_groups`) FIRST, then `kuniform_dual_n8`/`n16` —
helper usage, face census, group-measurement evidence, classification
arm, doc-row shape, and their wiring in `families/kuniform.rs`
(register slice, classification arm, census IDS, `lattice_tilings()`
list) and `docs/families/kuniform.md` (rows after dual-n3's).

### Goal

Land `kuniform-dual-n14` and `kuniform-dual-n15`: the face-centroid
duals of the two (3⁶; 3³.4²) 2-uniform tilings, constructed with the
existing `dual()` helper over the primals' exact Q(√3) cells,
registered in the kuniform family exactly like M145, classified with
MEASURED groups (expect pmm for the n14 dual — the primal is pmm — and
cmm for the n15 dual — the primal is cmm; measure with the exact
S(g)-style audit, evidence decides, report deviations). Two landings,
one commit each.

### Construction + verification basis

- **Primals (read their modules + doc rows first):**
  `kuniform-n14` — pmm, single square rows alternating with double
  triangle strips (two counterposed unit-triangle rows, strip height
  √3); PRIMITIVE rectangular basis ⟨(1,0),(0,1+√3)⟩, det 1+√3
  (right-handed); 5 tiles/cell (1 square + 4 triangles in three
  orbits); t = 3, e = 4; vertex orbits: 3⁶ (1 class/cell) + 33344
  (2 classes/cell) → **3 vertex classes/cell → expect 3 dual
  faces/cell** (1 hexagon over the 3⁶ orbit + 2 pentagons over the
  33344 orbits). `kuniform-n15` — cmm, staggered square rows
  alternating with triple triangle strips; centered rectangular basis
  ⟨(1,0),(1/2, 1+3√3/2)⟩ with orthogonal index-2 pair (1,0),(0,2+3√3),
  det 1+3√3/2; 7 tiles/cell (1 square + 6 triangles); t = 4, e = 5;
  3⁶ and 33344 orbits, 2 classes/cell each → **4 vertex classes/cell
  → expect 4 dual faces/cell** (2 hexagons + 2 pentagons). Both exact
  Q(√3) — the dual inherits exact coordinates.
- **Expected face census** (derive, don't assume): the face at a 3⁶
  vertex has 6 edges, at a 33344 vertex 5. Verify: Σ face areas =
  |det| exactly (Q(√3) arithmetic), every interior edge shared by
  exactly 2 faces (edge-to-edge), face-shape census cross-checked
  against the primal vertex figures, standard coverage battery.
- **Python spike FIRST** (`/tmp/opencode/duals-n14-n15/`, stdlib only,
  ≤240 s per experiment, FOREGROUND with visible per-step output):
  recompute the primal cells from the module's exact bases, build the
  face-centroid duals, verify area sums / edge parity / face-shape
  census / fine-grid coverage (irrational offset) for BOTH primals
  before touching Rust. Cross-check the helper's output against your
  spike (exact tuple equality).
- **Group measurement — REUSE the fixed audit:** your previous job's
  `/tmp/opencode/duals-n4-n3/group_audit.py` is the FIXED exact audit
  (the raw-xy lattice-reconstruction bug is documented in that job's
  NOTES.md — retractions R1–R3). COPY it into
  `/tmp/opencode/duals-n14-n15/` and adapt (n15 is CENTERED
  rectangular — the exact-L-coordinate reconstruction matters there;
  n14 is primitive rectangular, the simpler case). The Rust
  `duals_match_measured_groups` arm now has the glide-closure probe
  (M145 hardening) — a wrong Pmm/Cmm pin can no longer pass silently,
  but still MEASURE, don't assume.
- **First thing:** `grep -rn "kuniform-dual-n14\\|kuniform-dual-n15"
  crates/ docs/` — both must be ABSENT. If present, STOP and report.

### Files you own

- `crates/tess-tilings/src/families/kuniform/dual.rs` — the two new
  dual fns + tests (mirror the M145 n4/n3 fns: `with_checker_seeds`,
  Case entries, trio).
- `crates/tess-tilings/src/families/kuniform.rs` — register ×2,
  classification ×2, census IDS 33 → 35, `lattice_tilings()` ×2 (the
  entries MUST land in the FAMILY list).
- `docs/families/kuniform.md` — two catalog rows (after dual-n3's) +
  heading count + batch-list note (add "…(3⁶; 3³.4²)₁/₂ pair M#" with
  the M# placeholder).
- `/tmp/opencode/duals-n14-n15/` — scratch; append-only NOTES.md.

**Do not touch:** `lib.rs`, README, CHANGELOG, INDEX, ROADMAP,
PROPOSED.md, `app_ui.rs` (the Laves-tab pin near L1858 now reads 26 and
WILL fire at 27, 28 — report it verbatim, never fix it), any other
family, UI crates, AGENTS.md.

### Commit exception (this brief only)

TWO worktree commits, one per tiling, subjects like
`kuniform-family: kuniform-dual-n14 (dual of 2-uniform (3⁶; 3³.4²)₁)`
then `…-n15 …` — NO M numbers, NO pushes. The orchestrator cherry-picks
both in order and assigns M numbers at integration. Doc rows carry
(M#) placeholders — flag them in the report.

### Constraints (the ones that bite here)

1. Exact Q(√3) end to end — the primals' tuples are exact; never
   decimal literals in geometry. `MAX_TILES = 250_000`.
2. `cargo check --workspace` zero warnings; NO repo-wide `cargo fmt`
   (rustfmt reflows ~40 pre-existing sibling hunks in the owned files —
   hand-match your hunks to house style, the M145 report documents
   this); `cargo test --workspace` green EXCEPT the expected
   orchestrator guards — verbatim output per commit is your delta
   evidence.
3. Your base is main at M145 (136 built-ins / 150 memberships / Laves
   tab 26 / Two-Uniform 18 / kuniform family 33). Totals may move if
   another landing integrates while you work — report what your guards
   PRINTED, never hard-coded numbers; do NOT treat another landing's
   absolute counts as your expected deltas (the M145 integration had
   to re-derive exactly this way).
4. Doc-row wording mirrors the dual-n4/dual-n3 rows: face census per
   cell, exact side lists + areas in Q(√3), Σ = |det|, edge-to-edge,
   measured group + the discriminating evidence (off-mirror 2-folds /
   glides), (M#) placeholder.

### Deliverable (worker report)

1. Diff-scope statement (exactly the owned files, both commits).
2. The pinned duals: face census per cell, exact face shapes (sides +
   areas in Q(√3)), Σ = |det| for both, measured groups with evidence.
3. Spike + Rust test numbers; coverage battery results.
4. Census deltas as printed by your guards, PER COMMIT: kuniform
   family 33 → 34 → 35; Laves tab 26 → 27 → 28 (the L1858 assert
   fires; report verbatim); built-ins 136 → 137 → 138; memberships
   150 → 151 → 152. Flag explicitly that BOTH duals entered the family
   `lattice_tilings()` list (culling genuine, not vacuous).
5. Proposed commit-message bodies. NO M numbers.