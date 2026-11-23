# Work brief template

A brief is the unit of delegated tiling work — one brief, one worker,
one family (the collision-free unit; see the fleet protocol in
`tessellix/AGENTS.md`). A brief must be **self-contained**: the worker
cannot see this repository's conversation history. Fill every
`<...>`, keep the AGENTS.md constraints verbatim (the worker may not
have AGENTS.md open), and split anything bigger than effort M into
several briefs.

Worked examples in this directory: [`kuniform-n17.md`](kuniform-n17.md)
(periodic data-table family member) and
[`pregnant-chairs.md`](pregnant-chairs.md) (substitution rep-tile).

---

## Brief: `<brief-id>`

**Family:** `<family>` · **Candidate:** `<PROPOSED.md §ref / ROADMAP
queue item>` · **Effort:** `<S|M|L>` · **Precedent to mirror:**
`<module path(s)>`

### Goal

<One paragraph: what tiling lands, under what registry id, in which
family, with what headline property — wallpaper group for data tables,
substitution rule + period structure for generators.>

### Literature basis

<Exact sources: Wikipedia article + section anchor, paper, Tilings
Encyclopedia page. State what must be re-verified from the source
before coding (name, vertex configuration, numbering/identity) and
flag any known literature ambiguity from prior milestones. If a prior
milestone noted a numbering trap, quote it here.>

### Files you own

- `crates/tess-tilings/src/families/<family>/<name>.rs` — the new module.
- `crates/tess-tilings/src/families/<family>.rs` — module decl +
  `register` slice + `classification` arm(s) + census `IDS`
  (+ `lattice_tilings()` entry if the tiling is a lattice table).
- `docs/families/<family>.md` — the catalog row (guard-checked) plus
  the Per-tiling-documents link line; a numbered build arc only if the
  build introduces a new technique or hard-won lesson.
- `docs/tilings/<id>.md` — the new tiling's per-tiling document
  (guard-checked): H1 `# <registry name>` (must match `t.name()`
  exactly), metadata line (`` `<id>` · family link · wallpaper group ·
  lattice ``), the specification/algorithm, and the See-also block —
  copy the shape from any existing `docs/tilings/<id>.md`.
- `/tmp/opencode/<brief-id>/` — scratch (Python spike, batteries).

**Do not touch anything else.** Not README, not `lib.rs` (census pin,
guards, culling test), not `docs/history/CHANGELOG.md` (the M number
and its entry are written by the orchestrator at integration), not
other families, not the UI crates. Adding a tiling never requires UI
code; if it seems to, stop and report.

### Constraints (from AGENTS.md)

1. Exact geometry: express vertices in radicals via the `H = √3/2`
   constant — never decimal literals. (Measured-panel exception:
   AGENTS.md "Adding a periodic tiling" step 1.)
2. Verify numerically BEFORE writing Rust: Python spike, **stdlib
   only**, in `/tmp/opencode/<brief-id>/`, over a ±3×±3 lattice patch
   of the cell:
   - `sum(cell areas) == |det(a1, a2)|` to ~1e-15;
   - every interior edge shared by exactly 2 cells (key edges by
     endpoint pairs rounded to 6 digits — 9 digits create spurious
     T-junctions after translate round-trips);
   - every interior vertex angle multiset == the vertex figure;
   - a fine grid sample (shifted by an irrational offset) covered
     exactly once.
3. Emission gotcha: when several geometric tiles of one class share a
   vertex, the cell must emit each geometric tile exactly once
   (AGENTS.md step 3).
4. Name verified against Wikipedia/literature before coding.
5. Tile budget `MAX_TILES = 250_000`; keep generators cheap (the GUI
   regenerates every frame).
6. Keep `cargo check --workspace` warning-free.

### Verification battery (what "done" means)

<Per-brief programmatic checks, each mapping to a claim in the report:
coset symmetry audit for a wallpaper-group claim (pure-reflection vs
glide cosets), torus orbit census vs literature isohedral/isotoxal
counts, substitution-rule enumeration counts, dissection uniqueness by
exhaustive exact-integer search, period-lattice measurement on the
integer grid, PF-orbit counts per substitution depth. State the exact
numbers the literature predicts where known.>

### Standard tests

The 3-test trio every tiling ships (deterministic generation, census
consistency, render smoke) — copy the shape from the precedent module
— plus the family-specific battery above as in-module tests.

### Deliverable (worker report)

Per the fleet protocol in `tessellix/AGENTS.md`:

1. Diff-scope statement (exactly the owned files).
2. Verification evidence with exact numbers and pass counts.
3. Expected census deltas: family n → n+1; built-ins 150 → 151;
   memberships 164 → 165 (`<adjust per brief>`); `LatticeTiling`?
   → remind the orchestrator to add the culling-test line. Doc deltas
   come with the owned files: the family-doc row + Per-tiling-documents
   link and the new `docs/tilings/<id>.md`; the orchestrator updates
   the README census/prose/family-table count.
4. Proposed commit-message body (lattice, cell contents, verification).

The orchestrator assigns `M<n>` at integration — do not put an M
number in commits, docs, or the report.