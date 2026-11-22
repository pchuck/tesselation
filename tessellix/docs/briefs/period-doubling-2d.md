# Brief: period-doubling-2d — §23d limit-periodic product substitution

**Family:** `aperiodic` · **Candidate:** PROPOSED.md §23d "Period
doubling / period tripling / Thue–Morse 2D / Squiral — marginal
supersubstitutions; effort S–M each" (**this brief covers ONLY period
doubling 2D** — Thue–Morse, period tripling, Squiral stay parked) ·
**Effort:** S–M · **Precedent to mirror:**
`families/aperiodic/domino.rs` (M138 — the closest sibling: square
block substitution, exact Z², limit-periodic, full-block period
manifest measured g = 1..6; read its `cover_box` doc comment, exact
integer tuple arithmetic, depth-quantization view rule and test trio
first) and `families/aperiodic/half_hex.rs` (the same §23d cluster's
depth rule + patch-stability trio pattern).

### Goal

Land the 2D period-doubling substitution tiling under registry id
`period-doubling` in the `aperiodic` family, classified like domino
(same group tags, category `Nonperiodic`, method substitution,
translational status "limit-periodic") **unless your battery says
otherwise** — evidence decides; report deviations. Expected headline
property: limit-periodic via approximating periodic tilings with a
measured (2^g, 2^g) full-block period manifest — the ratio-1 signature
that distinguishes it from domino's ratio-2 (2^g, 2^(g+1)).

### Literature basis

- 1D rule **already pinned** (orchestrator, 2026-11-21) from the live
  Bielefeld entry
  <https://tilings.math.uni-bielefeld.de/substitution/period-doubling/>:
  symbolic substitution **a → ab, b → aa**, 2 prototiles, glossary
  tags One Dimensional / p-adic Windowed Tiling / Rep-Tiles /
  Self-Similar Substitution, internal space = the 2-adic integers,
  "in some sense the simplest cut and project tiling" ([FS]
  Frettlöh–Sing, *Computing modular coincidences*). **The entry is 1D
  only** — there is no 2D rule figure to copy.
- The 2D tiling is the standard **product construction** σ⊗σ: unit
  squares of Z² labeled by pairs (x, y) ∈ {a, b}²; each depth step
  inflates by 2 and the square at block position (i, j) receives label
  (σ(x)_i, σ(y)_j). That yields **4 prototile label classes**
  (aa, ab, ba, bb) over one geometric square prototile, 4 tiles per
  step, PF eigenvalue 4 (every column of the substitution matrix sums
  to 4 — verify this in the spike), internal space Z₂ × Z₂.
- Derive the substitution matrix in the spike and cross-check against
  the direct block enumeration: from (a,a) → {aa, ab, ba, bb} one
  each; from (a,b) → aa×2 + ba×2; from (b,a) → aa×2 + ab×2; from
  (b,b) → aa×4. If your enumeration disagrees with this table, STOP
  and report — the pin, not the code, is wrong.
- Expected period structure (**measured, never asserted**): the
  depth-g patch should be the fundamental cell of a period-(2^g, 2^g)
  approximant (both axes double per step). Measure the minimal
  translation period of the depth-g patch for g = 1..6 and state the
  growth law whatever it is; also check for extra/sub-periods (domino
  found none within patches — "full-block" manifest). If you measure
  a different law or a genuinely periodic patch, STOP and report
  before writing Rust.
- Exact arithmetic: pure Z² end to end (integer tuples), floats only
  at the emission boundary — never decimal literals in exact geometry.

### Files you own

- `crates/tess-tilings/src/families/aperiodic/period_doubling.rs` —
  the new module.
- `crates/tess-tilings/src/families/aperiodic.rs` — module decl +
  register slice + classification arm + census IDS. (Not a lattice
  table → no `lattice_tilings()` change.)
- `docs/families/aperiodic.md` — catalog row (+ heading count).
- `/tmp/opencode/period-doubling-2d/` — scratch; append-only
  NOTES.md.

**Do not touch:** `lib.rs`, README, CHANGELOG, INDEX, ROADMAP,
PROPOSED.md, sibling aperiodic modules (`domino.rs` / `half_hex.rs`
are read-only references — copy patterns from them), other families,
UI crates, AGENTS.md.

### Commit exception (this brief only)

One worktree commit, subject like
`aperiodic-family: period-doubling 2D substitution` — NO M numbers,
NO pushes. The orchestrator cherry-picks and assigns the M number at
integration.

### Constraints (the ones that bite here)

1. Exact Z² end to end; `MAX_TILES = 250_000`; depth rule = octave
   quantization of the view extent with **patch-stability verified in
   tests** (grown/panned/zoomed views keep bit-identical overlapping
   tiles — half-hex/domino trio pattern).
2. Python spike first (stdlib only, ≤240 s per experiment; run
   experiments in the FOREGROUND with visible per-step output — do not
   leave results trapped in a dead background shell):
   - substitution consistency: the depth-g block partition is exactly
     the depth-(g+1) partition (bucketed edge/vertex matching, rounded
     to 6 digits);
   - 0 overlaps (bucketed nearby pairs) + exact single coverage on a
     fine grid shifted by an irrational offset, over a ±3 patch;
   - substitution matrix column sums all 4; PF eigenvalue 4 (if you
     compute a char poly / determinant by permutation expansion,
     assert the full permutation count was visited — M135 lesson);
   - period manifest: minimal translation period of the depth-g patch,
     g = 1..6, growth law stated, extra-period check;
   - per-depth label census vs the substitution matrix rows.
3. `cargo check --workspace` zero warnings; `cargo fmt` owned hunks
   only (repo is NOT fmt-clean — revert non-owned byproducts);
   `cargo test --workspace` green except the two orchestrator guards
   (`registry_groups…`, `readme_catalog…`) — their verbatim failure
   output is your delta evidence.
4. You branch from main at M140 (130 built-ins / 144 memberships /
   Nonperiodic tab 6 / aperiodic family 7), but other landings may
   move these totals while you work — **never hard-code totals from
   your base**; report exactly what your guards printed and the
   orchestrator re-derives deltas at integration.
5. The 4 classes are LABELS on identical unit squares (seeds 0..3 by
   class): say "4 prototile label classes, one square prototile" in
   the doc row so the census claim matches the geometry. Lattice field
   "—", group "—" (nonperiodic), like the domino row.

### Deliverable (worker report)

1. Diff-scope statement (exactly the owned files, nothing else).
2. The pinned construction: entry URL, the 1D rule, the σ⊗σ product
   definition, substitution matrix, PF eigenvalue.
3. Battery numbers with exact values; Rust test list with pass
   counts; the measured period manifest g = 1..6.
4. Expected census deltas as printed by your guards (base totals +
   your +1s): aperiodic family 7 → 8; Nonperiodic tab 6 → 7 (if your
   evidence flips the category to Aperiodic: tab 8 → 9 instead — STOP
   first and report); built-ins +1; memberships +1. Not a lattice
   table → remind the orchestrator of nothing (no culling lines).
5. Proposed commit-message body (construction, verification, period
   manifest). NO M numbers.