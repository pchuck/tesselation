# Brief: golden-rhomboid — the Andritz φ-substitution

> **LANDED as M144** (see docs/history/CHANGELOG.md). Do not dispatch.

**Family:** `quasiperiodic` · **Candidate:** PROPOSED.md §23a
"Golden Rhomboid–Triangle — effort S–M" · **Effort:** M ·
**Precedent to mirror:** `crates/tess-tilings/src/families/quasiperiodic/golden_bee.rs`
and `golden_triangle.rs` (M127) — same field layer and substitution
structure.

### Goal

Land `golden-rhomboid`: the Bielefeld "golden-rhomboid-triangle"
substitution tiling — three prototiles on the φ lattice, inflation
λ = φ, category Quasiperiodic. A substitution generator like its
golden-family siblings (not a lattice table).

### Literature basis + salvage

- Source: Bielefeld Tilings Encyclopedia entry
  "golden-rhomboid-triangle"; the rhomboid was discovered by **Andritz**.
  Re-verify the rule and shapes against the live entry before coding.
- **SALVAGE — a dead pre-rework session left a 27 KB stdlib battery,
  `battery.py`, archived at
  `/home/pcharles/dev/pchuck/tesselation/.scratch/salvage-20261120/tmp/grt/battery.py`
  (local to this machine; the pinned math below is the durable copy).
  Its lines up to the distinctness assert are verified sound; it FAILS
  at line 293. Read it in slices (sed/grep), never one dump.**
- Pinned rule (battery header, verified): prototiles **Y** = equilateral
  triangle side 1, **O** = equilateral triangle side φ, **R** = 60°/120°
  rhomboid 1×φ (area 2φ in Y-units). Inflation **λ = φ**. Areas
  (√3/4 units) **1 : φ² : 2φ** (φ² = φ+1). Substitution
  **Y → O; O → O + Y + R; R → R + 2O** — each line area-exact
  (φ²·1 = φ²; φ²+1+2φ = φ⁴; 2φ+2φ² = 2φ·φ²).
- Pinned exact model (battery header): a point is the integer 4-tuple
  `(a0, a1, b0, b1)` ⇔ x = (a0 + a1·φ)/2, y = (b0 + b1·φ)·√3/2; φ acts
  linearly, φ·(a0,a1) = (a1, a0+a1); exact 60° rotation
  `rot60(a0,a1,b0,b1) = ((a0−3b0)/2, (a1−3b1)/2, (a0+b0)/2, (a1+b1)/2)`
  (parity-protected). The battery's `AREA_WANT`/`LEN_WANT` tables pin
  the **18 anchored orientation frames**: Y_k = 0..5, O_k = 6..11,
  R_A_k = 12..14, R_B_k = 15..17.
- **The line-293 failure and the authorized fix:** the failing block
  asserts the 18 frames pairwise distinct *as unanchored sets* via
  `norm_key` — but equilateral 120°-rotation invariance makes Y/O
  frames collide (the saved `duplicate class 2/0`). Fix the keying
  CORRECTLY, not just the number: quotient Y and O frame keys by the
  120° rotation (6 → 2 sets each); R frames have no 120°
  self-symmetry and stay **6 distinct sets** — total
  **10 unanchored sets (Y:2, O:2, R:6)**. Keep `SHAPES`/`RULES`
  anchored (the placement tables consume anchored frames); state the
  anchored-vs-unanchored semantics choice and how proto classes map to
  orientation frames in the report. Then run the FULL battery to
  green, repairing post-293 sections minimally per the pinned rule
  (list every repair). Save the fixed battery as
  `/tmp/opencode/golden-rhomboid/battery_fixed.py` (original
  untouched) and copy it into the worktree as
  `crates/tess-tilings/tests/golden_rhomboid_battery_reference.py`.
- Optional cross-check: the battery header pins the dead session's
  density comparison against the source patch — **1525 Y : 4012 O :
  2477 R**, rhomboid direction-pair classes **1025/938/514** (from
  `fills.json`, 890 KB — script-slice only if you use it; never dump).
- Battery invariants this rule forces: the substitution count matrix
  `[[(0,1,0)], [(1,1,1)], [(0,2,1)]]` (rows Y, O, R) has PF eigenvalue
  **φ² = λ²** — your derived count vectors must reproduce it, and the
  per-parent area law must hold exactly (children area = parent × φ²).

### Files you own

- `crates/tess-tilings/src/families/quasiperiodic/golden_rhomboid.rs`
  — the new module (mirror `golden_bee.rs`: exact field arithmetic,
  substitution rule tables, seed block, in-module tests).
- `crates/tess-tilings/src/families/quasiperiodic.rs` — module decl +
  `register` slice + `classification` arm + census `IDS`. (No
  `lattice_tilings()` change — not a lattice table.)
- `docs/families/quasiperiodic.md` — the catalog row (guarded); arc
  only if arc-worthy.
- `/tmp/opencode/golden-rhomboid/` — scratch.

**Do not touch:** `lib.rs`, README, `docs/history/CHANGELOG.md`,
`docs/INDEX.md`, the sibling modules (`golden_bee.rs`,
`golden_triangle.rs` are read-only references — copy, don't edit),
other families, the UI crates.

### Constraints (from AGENTS.md, the ones that bite here)

1. Exact arithmetic in **Q(√5)** integer pairs end to end (golden bee
   precedent shows the field layer) — floats only at the emission
   boundary. Never decimal literals.
2. Python spike first (stdlib only, ≤240 s per experiment): pin the
   child **placements** (coordinates inside the inflated parent) from
   the source figure; the pinned rule above is the invariant, the
   figure is the geometry.
3. **Frame-distinctness trap:** see the salvage section — the 18 → 10
   unanchored-sets fix is the authorized repair; do not "fix" it by
   loosening the assert to a magic number.
4. Tile budget `MAX_TILES = 250_000`: pick `STEP_CAP` so a depth-g
   render stays far under it (growth is φ² per generation in count
   vector mass).
5. Keep `cargo check --workspace` warning-free.

### Verification battery

- **Rule-line checks per parent:** each substitution line's children
  tile the inflated parent exactly (area sum == parent × φ²; no
  interior overlaps; edge/vertex incidence consistent).
- **Growth to depth 4:** edge-to-edge (strict PIP + proper-crossing
  overlap checks); count vectors follow the substitution matrix;
  PF eigenvalue φ² confirmed numerically; area law per generation.
- **Frame census:** 10 distinct unanchored frame sets (Y:2, O:2, R:6)
  under the semantics you chose; chirality behavior stated.
- Python numbers first, then the same numbers reproduced by the Rust
  in-module tests.
- Optional recon: `cargo run -p tess-tui -- export --tiling
  golden-rhomboid --mode proto …` and parse the SVG (class counts at a
  depth vs the count vector). Evidence is the numbers, not the look.

### Standard tests

Copy the trio from `golden_bee.rs`'s test module shape
(`golden_rhomboid_covers_region`, `golden_rhomboid_rotated_scaled`,
`golden_rhomboid_deterministic`) plus the battery above as in-module
tests (`golden_rhomboid_tests`).

### Scope note (golden-family convention)

The battery verifies substitution mechanics; diffraction/pure-point
spectra are cited, not proven — mirror `golden_triangle.rs`'s header
note. Document the rule source (Bielefeld; Andritz) and the
substitution matrix in the module header.

### Deliverable (worker report)

1. Diff scope (only the files above).
2. The pinned placements and the frame-semantics choice (anchored vs
   unanchored; the 18→10 collapse).
3. Battery numbers: rule-line checks, depth-4 count vectors, PF
   eigenvalue, frame census; Rust pass counts.
4. Census deltas: quasiperiodic family 11 → 12; Quasiperiodic tab
   12 → 13; built-ins +1; memberships +1 (read the pinned totals in
   `lib.rs` at your start — if another tiling landed first, the
   orchestrator resolves cumulative state at integration; report what
   you saw). NOT a lattice table → no culling line.
5. Proposed commit-message body (rule, field, verification). No M
   number — assigned at integration.