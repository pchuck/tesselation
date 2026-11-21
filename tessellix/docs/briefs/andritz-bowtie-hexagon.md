# Brief: andritz-bowtie-hexagon — the Andritz 5-fold FLC family

**Family:** `islamic` · **Candidate:** PROPOSED.md §23f "Andritz
Bowtie–Hexagon family (4 entries: small/large bowtie, hexagon
variants) — 5-fold FLC model sets with documented substitutions;
effort M each" · **Effort:** L (four M landings) · **Precedent to
mirror:** `crates/tess-tilings/src/families/islamic/girih_qc.rs`
(girih-quasiperiodic, the Darb-i Imam 3-prototile φ-substitution —
same family, same field, same FLC-note convention) and the fresh
girih-primitive tables M131–M134 (`girih_bowtie.rs` etc. — the
primitive shapes you will substitute are now shipped data tables).

### Goal

Land the four Andritz 5-fold substitution tilings as substitution
generators in the `islamic` family (likely Quasiperiodic tab, exactly
like girih-qc — mirror its classification arm; the evidence decides).
Suggested landing sequence: pin all four rules first, then land in
increasing complexity. Per-tiling ids named after their live
Bielefeld entries (e.g. `andritz-bowtie`, `andritz-large-bowtie`,
`andritz-hexagon`, … — use the entry slugs).

### Literature basis

- Source: Bielefeld Tilings Encyclopedia — search the substitution
  index for the **Andritz**-authored entries (the same discoverer as
  the golden rhomboid, M-paper pending). There should be four: small
  bowtie, large bowtie, hexagon variants. **Step 1 pins all four
  rules from the live entries (figures + stated inflation) before any
  coding.**
- Geometry: 5-fold, so everything sits in **Q(√5)** with the 36°
  frame constants the family already ships (SQ5/C36/C72/S36/S72 in
  the girih modules); the primitive tile shapes (bowtie, rhombus,
  pentagon, decagon, hexagon) are the M131–M134 tables' exact
  polygons — reuse those vertex tables (copy, don't import across
  modules unless the family file structure allows a shared helper in
  `families/islamic/`).
- Each entry documents a substitution on girih primitives. Verify per
  rule: stated inflation λ; children tile the inflated parent exactly;
  substitution count matrix PF eigenvalue = λᵈ consistent with areas.

### Files you own

- `crates/tess-tilings/src/families/islamic/andritz_<name>.rs` — one
  new module per tiling.
- `crates/tess-tilings/src/families/islamic.rs` — module decls +
  `register` slices + `classification` arms + census `IDS`. (No
  `lattice_tilings()` change — substitution generators, not lattice
  tables.)
- `docs/families/islamic.md` — the catalog rows (guarded).
- `/tmp/opencode/andritz-bowtie-hexagon/` — scratch; append-only
  `NOTES.md`.

**Do not touch:** `lib.rs`, README, `docs/history/CHANGELOG.md`,
`docs/INDEX.md`, `docs/ROADMAP.md`, PROPOSED.md, the sibling islamic
modules (girih_qc.rs and the girih_* M131–M134 tables are read-only
references — copy from them), other families, the UI crates,
AGENTS.md.

### Commit exception (this brief only)

One worktree commit per landing, each passing the bar below, subjects
like `andritz-family: bowtie substitution` — NO M numbers, NO pushes.
The orchestrator cherry-picks per tiling and assigns M numbers.

### Constraints (from AGENTS.md, the ones that bite here)

1. Exact Q(√5) arithmetic end to end — floats only at the emission
   boundary; never decimal literals; reuse the family's radical
   constants.
2. Python spike first (stdlib only, ≤240 s per experiment), per rule:
   dissection exactness (disjoint interiors, contained, area sum =
   parent × λ²), growth to depth 3–4 edge-to-edge (strict PIP +
   proper-crossing checks), count vectors vs the pinned matrix.
3. FLC/finite-rotations scope note per the girih-qc convention:
   substitution mechanics verified; model-set/diffraction properties
   cited from the entry, not proven.
4. Tile budget `MAX_TILES = 250_000`; keep STEP_CAP far under it.
5. `cargo check --workspace` zero warnings; `cargo fmt` on your
   hunks; `cargo test --workspace` green except the two orchestrator
   guards (registry pin, README catalog) — their verbatim failure
   output is your delta evidence.

### Verification battery

- Per rule: the spike battery above; the 3-test trio
  (`andritz_<name>_covers_region`, `_rotated_scaled`,
  `_deterministic`) plus in-module battery tests.
- Cross-rule consistency where entries share tiles: substituted child
  shapes match the M131–M134 primitive tables' exact vertices.
- Report the measured tile-per-generation growth for each tiling.

### Deliverable (worker report — per landing, emitted as each lands)

1. Diff scope for the landing.
2. The pinned rule (source entry URL, prototiles, λ, placements,
   matrix).
3. Battery numbers; Rust pass counts.
4. Census deltas: islamic family +1 per landing (26 → 30 cumulative);
   tab +1 on the evidence-decided tab (read the pinned totals in
   `lib.rs` at your start and report what you saw). NOT lattice
   tables → no culling lines.
5. Proposed commit-message body per landing. NO M numbers.