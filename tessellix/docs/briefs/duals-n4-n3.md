# Brief: duals-n4-n3 — §23k audit-discovered 2-uniform duals (square pair)

> **LANDED as M145** (see docs/history/CHANGELOG.md). Do not dispatch.

**Family:** `kuniform` (the duals register in the kuniform family, per
the `kuniform-dual-n8` / `kuniform-dual-n16` precedent — they DISPLAY
on the Laves tab via their category) · **Candidate:** PROPOSED.md §23k
"Duals of the four subscripted 2-uniform pairs … missing: duals of
(4⁴; 3³.4²)₁/₂ and (3⁶; 3³.4²)₁/₂. effort M each (dual.rs machinery
exists)" — **this brief covers ONLY the (4⁴; 3³.4²) pair: the duals of
`kuniform-n4` (…₁, M103) and `kuniform-n3` (…₂, M108)**; the
(3⁶; 3³.4²) pair (n14/n15) is a separate later brief · **Effort:** M
(2 landings, one commit each) · **Precedent to mirror:**
`families/kuniform/dual.rs` — read `kuniform_dual_n8` (M98) and
`kuniform_dual_n16` (M121, the most recent) FIRST: helper usage, face
census, group-measurement evidence, classification arm, doc-row shape,
and their wiring in `families/kuniform.rs` (register slice,
classification arm, census IDS, `lattice_tilings()` list) and
`docs/families/kuniform.md` (rows at "dual of 2-uniform …").

### Goal

Land `kuniform-dual-n4` and `kuniform-dual-n3`: the face-centroid
duals of the two (4⁴; 3³.4²) 2-uniform tilings, constructed with the
existing `dual()` helper over the primals' exact cells, registered in
the kuniform family exactly like the earlier duals (same wiring
surfaces), classified with MEASURED groups (expect cmm inherited from
the primals — dual-n8 kept p6m and dual-n16 kept p4g, but measure;
evidence decides, report deviations). Two landings, one commit each.

### Construction + verification basis

- **Primals (read their modules + doc rows first):**
  `kuniform-n4` — cmm, staggered two-square-tall bands alternating
  with alternating-triangle bands one triangle high; centered
  rectangular basis ⟨(1,0),(1/2, 2+√3/2)⟩, orthogonal index-2 pair
  (1,0),(0, 4+√3), det 2+√3/2; 4 tiles/cell (2 squares + 2 triangles);
  t = 2, e = 4; vertex orbits 4444 (1 class/cell) + 33344 (2
  classes/cell). `kuniform-n3` — cmm, three-square-tall bands; basis
  ⟨(1,0),(1/2, 3+√3/2)⟩, det 3+√3/2; 5 tiles/cell; t = 3, e = 5;
  4444 and 33344 orbits, 2 classes/cell each. Both exact Q(√3) — the
  dual inherits exact coordinates (face centroids of exact vertices
  stay in Q(√3)).
- **Expected face census** (derive, don't assume): one dual face per
  primal vertex class in the cell — n4 has 3 vertex classes/cell → 3
  faces/cell; n3 has 4 → 4 faces/cell. Verify: Σ face areas = |det|
  exactly (Q(√3) arithmetic), every interior edge shared by exactly 2
  faces (edge-to-edge), face shapes cross-checked against the vertex
  figures of the primal (the face at a 4444 vertex has 4 edges, at a
  33344 vertex 5 edges), and the standard coverage battery.
- **Python spike FIRST** (`/tmp/opencode/duals-n4-n3/`, stdlib only,
  ≤240 s per experiment, FOREGROUND with visible per-step output):
  recompute the primal cells from the module's exact bases, build the
  face-centroid duals, verify area sums / edge parity / face-shape
  census / fine-grid coverage (irrational offset) for BOTH primals
  before touching Rust. Cross-check the helper's output against your
  spike (they must agree to the last exact tuple).
- **Group measurement:** mirror the dual-n16 coset-audit evidence
  (torus enumeration over the cell; mirror/glide/2-fold census). If
  the measured group is NOT cmm, STOP and report before registering.
- **First thing:** `grep -rn "kuniform-dual-n4\|kuniform-dual-n3"
  crates/ docs/` — both must be ABSENT (the §23k audit says missing).
  If present, STOP and report.

### Files you own

- `crates/tess-tilings/src/families/kuniform/dual.rs` — the two new
  dual fns + tests.
- `crates/tess-tilings/src/families/kuniform.rs` — register ×2,
  classification ×2, census IDS 31 → 33, `lattice_tilings()` ×2 (the
  duals are lattice tables — the per-family list feeds the lib.rs
  culling extend automatically; make sure the entries land in the
  FAMILY list, not nowhere).
- `docs/families/kuniform.md` — two catalog rows (after dual-n16's)
  + heading count if present.
- `/tmp/opencode/duals-n4-n3/` — scratch; append-only NOTES.md.

**Do not touch:** `lib.rs`, README, CHANGELOG, INDEX, ROADMAP,
PROPOSED.md, `app_ui.rs` (a LAVES-tab count assert near L1844 WILL
fire — that pin is the orchestrator's; report it verbatim), any other
family, UI crates, AGENTS.md.

### Commit exception (this brief only)

TWO worktree commits, one per tiling, subjects like
`kuniform-family: kuniform-dual-n4 (dual of 2-uniform (4⁴; 3³.4²)₁)`
then `…-n3 …` — NO M numbers, NO pushes. The orchestrator cherry-picks
both in order and assigns M numbers at integration.

### Constraints (the ones that bite here)

1. Exact Q(√3) end to end — the primals' tuples are exact; never
   decimal literals in geometry. `MAX_TILES = 250_000`.
2. `cargo check --workspace` zero warnings; `cargo fmt` owned hunks
   only (rustfmt follows `mod` decls into siblings — revert any
   sibling reflow; the M142/M143 landing reports both hit this);
   `cargo test --workspace` green EXCEPT the expected orchestrator
   guards (`registry_groups…`, `readme_catalog…`) — verbatim output is
   your delta evidence. After dual-n4's commit the guards print ONE
   set of deltas; after dual-n3's, another — capture both.
3. Your base is main at M143 (133 built-ins / 147 memberships / Laves
   tab 24 / Two-Uniform 18 / kuniform family 31). Totals may move if
   another landing integrates while you work — report what your guards
   PRINTED, never hard-coded numbers.
4. Doc-row wording mirrors the dual-n8/dual-n16 rows: "face-centroid
   dual of `kuniform-nX` … constructed by the `dual()` helper;
   N faces/cell …; Σ = |det|, edge-to-edge; group + evidence (M#)" —
   leave the M# to the orchestrator (write (M#) as a placeholder and
   note it in the report).

### Deliverable (worker report)

1. Diff-scope statement (exactly the owned files, both commits).
2. The pinned duals: face census per cell, exact face shapes (sides +
   angles + areas in Q(√3)), Σ = |det| for both, measured groups with
   evidence.
3. Spike + Rust test numbers; coverage battery results.
4. Census deltas as printed by your guards, PER COMMIT: kuniform
   family 31 → 32 → 33; Laves tab 24 → 25 → 26 (category census —
   the L1844 assert fires; report verbatim); built-ins 133 → 134 →
   135; memberships 147 → 148 → 149. Flag explicitly that BOTH duals
   entered the family `lattice_tilings()` list (culling is genuine,
   not vacuous).
5. Proposed commit-message bodies. NO M numbers.