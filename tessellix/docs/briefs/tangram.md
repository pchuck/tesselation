# Brief: tangram — the classic dissection as a substitution

**Family:** `reptiles` (host — dossier §23d "rep-tile dissection, next
to chair/sphinx") · **Candidate:** PROPOSED.md §23d "Tangram — effort
S–M" · **Effort:** M · **Precedent to mirror:**
`crates/tess-tilings/src/families/reptiles/armchair.rs` (substitution
structure) and `crates/tess-tilings/src/families/quasiperiodic/ammann_beenker.rs`
(Q(√2) exact-arithmetic handling).

### Goal

Land `tangram`: the classic tangram dissection run as a substitution
(Bielefeld "tangram" entry). **5 prototiles** — 3 right-isosceles
triangle sizes + square + parallelogram (the 7-piece set has 5 distinct
classes). All geometry sits on the 45° grid ⇒ exact arithmetic in
**Z[√2]** integer pairs. Category decided by the battery: Nonperiodic
(limit-periodic, the chair/armchair precedent) vs Aperiodic — the
evidence decides, documented in the module header.

### Literature basis + salvage

- Source: Bielefeld Tilings Encyclopedia entry "tangram"
  (`tangram.html` page fact, pinned by a dead pre-rework session:
  **Number of Prototiles: 5** = 3 right-isosceles triangle sizes +
  square + parallelogram).
- **SALVAGE (lost session, two attempts died pre-battery — recon
  notes only):** the archived page snapshot
  `/home/pcharles/dev/pchuck/tesselation/.scratch/salvage-20261120/tmp/tangram/tangram.html`
  pins the page facts: **Number of Prototiles: 5** (3 right-isosceles
  triangle sizes + square + parallelogram), properties
  *Self-Similar Substitution*, *Finite Rotations*, *Polytopal Tiles*.
  The rule/patch images were **not** preserved — fetch the rule image
  from the live page (tilings.math.uni-bielefeld.de/substitution/tangram/),
  view it **at most once**, then pin the interpretation in the
  battery: **the image guides, the battery decides.** Never render or
  inspect large patch dumps.
- Handle the **parallelogram's chirality explicitly** — it is the one
  non-mirror-symmetric piece. Track both chiral states if the rule
  needs them; report how many proto classes you ship and why.

### Files you own

- `crates/tess-tilings/src/families/reptiles/tangram.rs` — the new
  module (mirror `armchair.rs` structure: rule tables, seed block,
  STEP_CAP, in-module tests; take the Z[√2] arithmetic style from
  `ammann_beenker.rs`).
- `crates/tess-tilings/src/families/reptiles.rs` — module decl +
  `register` slice + `classification` arm + census `IDS`. (No
  `lattice_tilings()` change — not a lattice table.)
- `docs/families/reptiles.md` — the catalog row (guarded).
- `/tmp/opencode/tangram/` — scratch.

**Do not touch:** `lib.rs`, README, `docs/history/CHANGELOG.md`,
`docs/INDEX.md`, the sibling modules (`chair.rs`, `armchair.rs`,
`sphinx.rs`, `p_pentomino.rs` are read-only references), other
families, the UI crates.

### Constraints (from AGENTS.md, the ones that bite here)

1. Exact **Z[√2]** integer-pair arithmetic end to end — floats only at
   the emission boundary. Never decimal literals.
2. Python spike first (stdlib only, ≤240 s per experiment): exact
   dissection per parent; the dissection placements come from the
   source figure, verified exhaustively where feasible.
3. Classification arm: do **not** use the `rep_tile` helper (it
   hardcodes one prototile and λ = 2) — construct the `TilingClass`
   literally, mirroring the `golden-bee` arm's shape in
   `families/quasiperiodic.rs` (method "substitution", inflation,
   prototiles, tile_set, translational per evidence).
4. Tile budget `MAX_TILES = 250_000`: pick `STEP_CAP` so a depth-g
   render stays far under it.
5. Keep `cargo check --workspace` warning-free.

### Verification battery

- **Dissection checks per parent:** each rule line's children tile the
  inflated parent exactly (exact rect/polygon intersection: pairwise
  disjoint; contained; area sum == parent × inflation²).
- **Growth to depth 3–4:** edge-to-edge (strict PIP + proper-crossing
  overlap checks); piece counts per generation follow the substitution
  matrix; area law per generation.
- **Period structure:** measure the depth-g patch's translation
  periods on the integer grid for g = 2..5; report the period sequence
  — this decides the category (limit-periodic ⇒ Nonperiodic; aperiodic
  evidence ⇒ Aperiodic flip; periodic ⇒ stop and report). State the
  T-junction counts per generation.
- **Chirality census:** parallelogram states per parent; where the
  rule breaks mirror symmetry.
- Python numbers first, then the same numbers reproduced by the Rust
  in-module tests.
- Optional recon: `export --mode proto` SVG parse (class counts vs the
  count vector). Evidence is the numbers, not the look.

### Standard tests

Copy the trio from `armchair.rs`'s test module shape
(`tangram_covers_region`, `tangram_rotated_scaled`,
`tangram_deterministic`) plus the battery above as in-module tests
(`tangram_tests`).

### Deliverable (worker report)

1. Diff scope (only the files above).
2. The pinned rule: 5 prototile shapes (exact Z[√2] coordinates),
   placements, chiral-class decision.
3. Battery numbers: dissection checks, depth-4 counts, period
   sequence, T-junction counts, chirality census; Rust pass counts.
4. Category verdict with the evidence line; census deltas: reptiles
   family 4 → 5; tab +1 on the category you landed (Nonperiodic 5 → 6
   default; Aperiodic 6 → 7 if the evidence flips — read the pinned
   totals in `lib.rs` at your start and report what you saw);
   built-ins +1; memberships +1. NOT a lattice table → no culling
   line.
5. Proposed commit-message body (rule, arithmetic, verification). No M
   number — assigned at integration.