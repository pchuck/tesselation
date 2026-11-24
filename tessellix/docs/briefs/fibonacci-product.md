# Brief: fibonacci-product — Fibonacci×Fibonacci

> **LANDED as M154** (see docs/history/CHANGELOG.md). Do not dispatch.

**Family:** `quasiperiodic` · **Candidate:** PROPOSED.md §23a
"Fibonacci×Fibonacci — 3 prototiles, product substitution; effort
S–M" · **Effort:** M · **Precedent to mirror:**
`crates/tess-tilings/src/families/quasiperiodic/golden_bee.rs` and
`golden_triangle.rs` (same field layer; M127 arc for methodology).

### Goal

Land `fibonacci-fibonacci`: the **product substitution** of the
Fibonacci substitution with itself (Bielefeld entry
"fibonacci-fibonacci") — **3 prototiles**, inflation **λ = τ²**
(τ = φ), substitution matrix = the square of the Fibonacci
substitution's matrix. Category Quasiperiodic. A substitution
generator like its golden-family siblings (not a lattice table).

### Literature basis + salvage

- Source: Bielefeld Tilings Encyclopedia entry "fibonacci-fibonacci".
  Pin the 3 prototile shapes and the rule from the live entry's figure
  before coding.
- **SALVAGE (from the lost pre-rework session — recon notes only, no
  battery survives):** archived recon at
  `/home/pcharles/dev/pchuck/tesselation/.scratch/salvage-20261120/tmp/`:
  `fibfib_rule.png` (4.3 KB rule image — view **at most once**, then
  pin the interpretation in the battery; the battery decides) and
  `fibfib_patch.pdf` (**NEVER view**). Inflation τ²; per-parent area
  law children = parent × **τ⁴** (linear τ² inflation); count vectors
  follow the **Fibonacci-squared recurrences** — i.e. the count matrix
  is the square of the 2-D Fibonacci substitution's matrix, so its PF
  eigenvalue must come out **τ⁴ = λ²** (battery invariant; if your
  derived matrix disagrees, stop and report rather than bending the
  rule).
- Discipline carried over from the failed attempts: view the source
  rule figure **at most once**, then pin the interpretation in the
  battery — the battery decides. Never render/inspect large patch
  dumps; keep tool outputs small.

### Files you own

- `crates/tess-tilings/src/families/quasiperiodic/fibonacci_fib.rs` —
  the new module (mirror `golden_bee.rs`).
- `crates/tess-tilings/src/families/quasiperiodic.rs` — module decl +
  `register` slice + `classification` arm + census `IDS`. (No
  `lattice_tilings()` change — not a lattice table.)
- `docs/families/quasiperiodic.md` — the catalog row (guarded); arc
  only if arc-worthy.
- `/tmp/opencode/fibonacci-product/` — scratch.

**Do not touch:** `lib.rs`, README, `docs/history/CHANGELOG.md`,
`docs/INDEX.md`, the sibling modules, other families, the UI crates.

### Constraints (from AGENTS.md, the ones that bite here)

1. Exact arithmetic in **Q(√5)** integer pairs end to end (τ² = φ+1
   stays in the field) — floats only at the emission boundary. Never
   decimal literals.
2. Python spike first (stdlib only, ≤240 s per experiment): derive the
   battery from the pinned rule; per-parent area law; strict PIP +
   proper-crossing overlap checks at every growth step.
3. Tile budget `MAX_TILES = 250_000`: pick `STEP_CAP` so a depth-g
   render stays far under it.
4. Keep `cargo check --workspace` warning-free.

### Verification battery

- **Product structure:** the count matrix equals the square of the
  Fibonacci substitution matrix (assert element-wise); PF eigenvalue
  τ⁴ = λ²; 3 prototiles with the area vector satisfying the area law
  exactly.
- **Rule-line checks per parent:** children tile the inflated parent
  exactly (area sum == parent × τ⁴; no interior overlaps; edge/vertex
  incidence consistent).
- **Growth to depth 4:** edge-to-edge at every step; count vectors per
  the squared recurrences; Python↔Rust table diff (the Rust rule
  tables must reproduce the spike's placements identically).
- Python numbers first, then the same numbers reproduced by the Rust
  in-module tests.
- Optional recon: `export --mode proto` SVG parse (class counts vs the
  count vector). Evidence is the numbers, not the look.

### Standard tests

Copy the trio from `golden_bee.rs`'s test module shape
(`fibonacci_fib_covers_region`, `fibonacci_fib_rotated_scaled`,
`fibonacci_fib_deterministic`) plus the battery above as in-module
tests (`fibonacci_fib_tests`).

### Scope note (golden-family convention)

Substitution mechanics verified; diffraction cited, not proven —
mirror `golden_triangle.rs`'s header note. Document the product
structure (matrix = Fibonacci matrix squared) and source in the module
header.

### Deliverable (worker report)

1. Diff scope (only the files above).
2. The pinned rule: 3 prototile shapes, placements, and how the
   product structure manifests (which matrix entries merged).
3. Battery numbers: matrix-squared assertion, PF eigenvalue, per-parent
   area law, depth-4 count vectors, overlap checks; Rust pass counts.
4. Census deltas: quasiperiodic family n → n+1; Quasiperiodic tab
   n → n+1 (read the pinned totals in `lib.rs` at your start — if
   `golden-rhomboid` landed first, it took 12 → 13 and you take
   13 → 14; the orchestrator resolves cumulative state at
   integration; report what you saw). Built-ins +1; memberships +1.
   NOT a lattice table → no culling line.
5. Proposed commit-message body (rule, field, verification). No M
   number — assigned at integration.