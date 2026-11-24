# Brief: pregnant-chair — the chair family's next rep-tile

> **LANDED as M136** (see docs/history/CHANGELOG.md). Do not dispatch.

**Family:** `reptiles` · **Candidate:** PROPOSED.md §23d ("Pregnant
chairs … effort S each"); the M128 changelog note "**Pregnant chairs**
remains open (effort S)" · **Effort:** S · **Precedent to mirror:**
`crates/tess-tilings/src/families/reptiles/armchair.rs` (M128) and
`chair.rs` (M30) — the machinery is a direct mirror.

### Goal

Land `pregnant-chair` (name to be confirmed in step 1 — see below):
the chair-family rep-tile flagged open after armchair. `Tiling`
substitution generator like its siblings (not a lattice table):
exact-integer inflation, dissection rule, limit-periodic depth-g
period structure. Category Nonperiodic.

### Literature basis

- PROPOSED.md §23d's source set (Bielefeld Tilings Encyclopedia
  rep-tile pages and the references they cite).
- **Pin the geometry in step 1, before any code:** the exact parent
  shape (polyomino/polyiamond coordinates on the unit grid), the
  inflation factor (rep-k), and the dissection. The chair (M30) and
  armchair (M128) precedents are exact-integer L-shapes with forced
  dissections; expect the same flavor, but **derive** the pregnant
  chair's shape and rule from the source rather than assuming — the
  name covers variants in the literature (singular/plural,
  "pregnant chair" vs "chair with bump"). Record: source page, the
  figure's coordinates, rep-k, and whether the source claims a unique
  or non-unique dissection. If sources disagree on the shape, stop
  and report.

### Files you own

- `crates/tess-tilings/src/families/reptiles/pregnant_chair.rs` — the
  new module (mirror `armchair.rs` structure: rule tables, seed block,
  STEP_CAP, in-module tests).
- `crates/tess-tilings/src/families/reptiles.rs` — module decl +
  `register` slice + `classification` arm + census `IDS`. (No
  `lattice_tilings()` change — the reptiles are `Tiling`
  implementations, not lattice tables.)
- `docs/families/reptiles.md` — the catalog row (guarded).
- `/tmp/opencode/pregnant-chair/` — scratch.

**Do not touch:** `lib.rs`, README, `docs/history/CHANGELOG.md`,
`docs/INDEX.md`, the sibling modules (`chair.rs`, `armchair.rs` are
read-only references — copy, don't edit), other families.

### Constraints

(Verbatim from AGENTS.md; the ones that bite here:)

1. **Exact integers only** — the whole spike and module run in
   exact-integer arithmetic (chair/armchair precedent: no floats
   anywhere in the rule). Coordinates on the unit grid; inflation
   scale is an integer (rep-4 ⇒ scale 2) unless step 1 proves
   otherwise.
2. Python spike first (stdlib only) — but the battery below replaces
   the lattice-cell checks: this is a substitution generator, so the
   spike verifies the **rule** (per-parent coverage, no overlaps,
   dissection counts) across generations, not a periodic cell.
3. Tile budget `MAX_TILES = 250_000`: pick `STEP_CAP` so a depth-g
   render stays far under it (armchair precedent: k^g tiles per
   generation; budget the cap accordingly).
4. Name verified against the source in step 1 (registry id
   `pregnant-chair` is provisional — report the verified name).

### Verification battery

- **Dissection census by exhaustive exact-integer enumeration** of all
  placements of k prototiles (all orientations) inside the inflated
  parent: report the count and compare with the source's claim
  (armchair precedent: unique, one child per orientation, rule
  forced).
- **Substitution correctness per generation** (depths 1–4): every
  parent replaced by the rule's children; union covers the parent
  exactly (area sum == parent area; no interior overlaps; edge/vertex
  incidence consistent).
- **Limit structure** (mirror armchair's): measure the depth-g block's
  translation periods on the integer grid for g = 2..5; report the
  period sequence and the T-junction counts per generation. If the
  tiling turns out periodic (not limit-periodic), report that instead
  — the category flips to the evidence.
- **Chirality/orientation census:** children orientations per parent,
  mirror behavior (chair family is chiral; state where).
- Python numbers first, then the same numbers reproduced by the Rust
  in-module tests.

### Standard tests

Copy the trio from `armchair.rs`'s test module shape
(deterministic generation, census consistency, render smoke) plus the
battery above as in-module tests (`pregnant_chair_tests`).

### Deliverable (worker report)

1. Diff scope (only the files above).
2. The step-1 geometry pin: source, shape coordinates, rep-k,
   dissection count, verified name.
3. Battery numbers: dissection census, per-generation coverage and
   counts, period sequence, T-junction counts; Rust pass counts.
4. Census deltas: Nonperiodic 5 → 6; built-ins 122 → 123;
   memberships 136 → 137; **not a lattice table** → no culling line.
5. Proposed commit-message body (shape, rule, verification). No M
   number — assigned at integration.