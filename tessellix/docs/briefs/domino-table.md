# Brief: domino-table — §23d limit-periodic domino/table substitutions

**Family:** `aperiodic` · **Candidate:** PROPOSED.md §23d "Domino/table
tiling (+variants) — limit-periodic, cut-and-project; effort M each" ·
**Effort:** M (1–2 landings: domino, table — as the live entries
decide) · **Precedent to mirror:** `families/aperiodic/half_hex.rs`
(M135, the same §23d limit-periodic cluster — read its `cover_box`
doc comment, exact-field tuple arithmetic, depth-quantization view
rule and test trio first) and `taylor_socolar.rs` (the tab's
limit-periodic classification precedent).

### Goal

Land the domino and/or table limit-periodic substitution tiling(s) in
the `aperiodic family`, classified like half-hex (`families
&["Aper
iodic"]`, method substitution, aperiodic_status/translational
"limit-periodic") unless your battery says otherwise (evidence
decides; report deviations). If both entries pin cleanly, land them
as two separate landings (one commit each).

### Literature basis

- Source: Bielefeld Tilings Encyclopedia substitution index — locate
  the **domino** and **table** entries. **Step 1 pins each rule from
  the live entry (figure + stated inflation + prototiles) before any
  coding.** If an entry is missing, renamed, or disagrees with the
  §23d assumption (limit-periodic, cut-and-project), STOP and report.
- Arithmetic field is decided by the pin (likely rational Z² for a
  domino square-substitution; check the entry). Whatever the field:
  exact integer/radical tuple arithmetic end to end, floats only at
  the emission boundary — never decimal literals in exact geometry
  (half-hex precedent: (u,v) integer pairs + one `sqrt()` at render).
- Limit-periodicity must be **measured, not asserted**: report the
  minimal translation period of the depth-g patch for g = 2..6 and
  its growth law (half-hex measured p = (3/2)·2^g).

### Files you own

- `crates/tess-tilings/src/families/aperiodic/<name>.rs` — one new
  module per tiling.
- `crates/tess-tilings/src/families/aperiodic.rs` — module decl +
  register slice + classification arm + census IDS. (Not a lattice
  table → no `lattice_tilings()` change.)
- `docs/families/aperiodic.md` — catalog rows (+ heading count).
- `/tmp/opencode/domino-table/` — scratch; append-only NOTES.md.

**Do not touch:** `lib.rs`, README, CHANGELOG, INDEX, ROADMAP,
PROPOSED.md, sibling aperiodic modules (`half_hex.rs` is a read-only
reference — copy patterns from it), other families, UI crates,
AGENTS.md.

### Commit exception (this brief only)

One worktree commit per landing, each passing the bar below, subjects
like `aperiodic-family: domino substitution` — NO M numbers, NO
pushes. The orchestrator cherry-picks per landing and assigns M
numbers.

### Constraints (the ones that bite here)

1. Exact arithmetic end to end; `MAX_TILES = 250_000`; depth rule =
   octave quantization of the view extent with **patch-stability
   verified in tests** (grown/panned/zoomed views keep bit-identical
   overlapping tiles — half-hex trio pattern).
2. Python spike first (stdlib only, ≤240 s per experiment): rule line
   disjoint/contained/area-sum in every orientation; depth count
   vectors vs the substitution matrix; PF eigenvalue consistency;
   period hierarchy g = 2..6; 0 overlaps over bucketed nearby pairs;
   fine-grid coverage; nested-view bit-identity.
3. Certification-guard lesson from M135: whenever you compute a char
   poly / determinant by permutation expansion, assert the full
   permutation count was visited (720 for 6×6).
4. `cargo check --workspace` zero warnings; `cargo fmt` owned hunks
   (repo is NOT fmt-clean — revert non-owned byproducts); `cargo test
   --workspace` green except the two orchestrator guards
   (`registry_groups…`, `readme_catalog…`) — their verbatim failure
   output is your delta evidence.

### Deliverable (worker report — per landing, emitted as each lands)

1. Diff scope for the landing.
2. The pinned rule (entry URL, prototiles, λ, placements, matrix,
   field).
3. Battery numbers; Rust pass counts; measured period hierarchy.
4. Census deltas: aperiodic family 6 → 7 (+1 per landing); Aperiodic
   tab 7 → 8; built-ins 128 → 129+; memberships 142 → 143+ (read the
   pinned totals in `lib.rs` at your start and report what you saw).
   Not lattice tables → no culling lines.
5. Proposed commit-message body per landing. NO M numbers.