# Brief: half-hex — Socolar's limit-periodic half-hexagon substitution

> **LANDED as M135** (see docs/history/CHANGELOG.md). Do not dispatch.

**Family:** `aperiodic` · **Candidate:** PROPOSED.md §23d "half-hex
family — limit-periodic, cut-and-project; half-hex also closes the #17
substitution-hierarchy gap; effort M" · **Effort:** M · **Precedent to
mirror:** `crates/tess-tilings/src/families/aperiodic.rs` — the
`taylor-socolar` classification arm (the shipped limit-periodic
sibling) and its module's test shapes; substitution-generator
structure (rule tables, seed block, STEP_CAP) from
`families/quasiperiodic/golden_bee.rs`.

### Goal

Land `half-hex`: the classical half-hexagon substitution tiling
(Socolar; Bielefeld entry "half-hex") as a substitution generator —
expected ONE prototile (half of a regular hexagon: an isosceles
trapezoid), integer inflation, category **limit-periodic**, registry
id `half-hex`, family `aperiodic`, Aperiodic tab. NOT a lattice
table. The landing closes the #17 substitution-hierarchy gap.

### Literature basis

- Source: Bielefeld Tilings Encyclopedia entry "half-hex" (Socolar).
  **Step 1 is the literature pin: fetch the live entry, pin the
  prototile's exact shape, the substitution (children count and
  placements), and the inflation λ before coding.** If the live entry
  disagrees with any hypothesis below, STOP and report the
  discrepancy — do not bend either side silently.
- Hypotheses to verify (not givens): hexagonal geometry ⇒ exact
  arithmetic in √3-field pairs (the `H = √3/2` conventions of the
  periodic tables; pregnant-chairs-style Z[√3] pair arithmetic is the
  template); inflation **λ = 2**; small integer child count per
  parent; **limit-periodic** — the defining property is a hierarchy
  of growing translation periods, which is exactly what the battery
  must measure.

### Files you own

- `crates/tess-tilings/src/families/aperiodic/half_hex.rs` — the new
  module (mirror `golden_bee.rs` structure; take test shapes from the
  aperiodic family's existing modules).
- `crates/tess-tilings/src/families/aperiodic.rs` — module decl +
  `register` slice + `classification` arm + census `IDS`. (No
  `lattice_tilings()` change — not a lattice table.)
- `docs/families/aperiodic.md` — the catalog row (guarded).
- `/tmp/opencode/half-hex/` — scratch; keep an append-only `NOTES.md`
  there so state survives a session death.

**Do not touch:** `lib.rs`, README, `docs/history/CHANGELOG.md`,
`docs/INDEX.md`, `docs/ROADMAP.md`, PROPOSED.md, the sibling aperiodic
modules (`hat.rs`, `turtle.rs`, `spectre.rs`, `taylor_socolar.rs`,
`jeandel_rao.rs` — read-only references), other families, the UI
crates, AGENTS.md.

### Constraints (from AGENTS.md, the ones that bite here)

1. Exact √3-field integer-pair arithmetic end to end — floats only at
   the emission boundary. Never decimal literals.
2. Python spike first (stdlib only, ≤240 s per experiment): derive the
   rule table and battery from the pinned entry; per-parent dissection
   checks; strict PIP + proper-crossing overlap checks at every growth
   step.
3. Classification arm: construct the `TilingClass` literally,
   mirroring the `taylor-socolar` arm's shape (`families:
   &["Aperiodic"]`, `method: Some("substitution")`, `prototiles`,
   `source`, `year`, `aperiodic_status: Some("limit-periodic")`,
   `translational: Some("limit-periodic")` — fill source/year from
   your literature pin). Do NOT use the `rep_tile` helper.
4. Tile budget `MAX_TILES = 250_000`: pick `STEP_CAP` so a depth-g
   render stays far under it (growth is λ² per generation in area).
5. Keep `cargo check --workspace` warning-free; `cargo fmt`.

### Verification battery

- **Rule-line checks per parent:** children tile the inflated parent
  exactly (pairwise disjoint interiors; contained; area sum == parent
  × λ²; exact trapezoid intersection arithmetic).
- **Growth to depth 4:** edge-to-edge at every step (strict PIP +
  proper-crossing overlap checks); count vectors follow the pinned
  substitution matrix; PF eigenvalue confirmed numerically; area law
  per generation.
- **Limit-periodicity evidence (the point of the landing):** measure
  the translation-period sequence of the depth-g patch's tile-outer
  structure for g = 2..6 on the exact integer lattice; report the
  sequence and show it grows strictly (the substitution hierarchy).
  If the patch measures plain periodic (fixed bounded periods) or
  aperiodic (no period structure at all), STOP and report — the
  dossier says limit-periodic, and a different verdict changes the
  classification fields.
- Python numbers first, then the same numbers reproduced by the Rust
  in-module tests.
- Optional recon: `cargo run -p tess-tui -- export --tiling half-hex
  --mode proto …` and parse the SVG (class counts vs the count
  vector). Evidence is the numbers, not the look.

### Standard tests

The trio (`half_hex_covers_region`, `half_hex_rotated_scaled`,
`half_hex_deterministic`) plus the battery above as in-module tests
(`half_hex_tests`), shaped like the aperiodic family's existing test
modules.

### Deliverable (worker report)

1. Diff scope (only the files above).
2. The pinned rule: exact prototile vertices, child placements, λ,
   substitution matrix; the literature source for each.
3. Battery numbers: rule-line checks, depth-4 counts, PF eigenvalue,
   the measured period hierarchy, overlap checks; Rust pass counts.
4. Census deltas: aperiodic family 5 → 6; Aperiodic tab +1 (read the
   pinned totals in `lib.rs` at your start and report what you saw);
   built-ins +1; memberships +1. NOT a lattice table → no culling
   line.
5. Proposed commit-message body (rule, arithmetic, verification). No
   M number — assigned at integration.