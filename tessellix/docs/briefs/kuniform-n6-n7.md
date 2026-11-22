# Brief: kuniform-n6-n7 — the LAST gap in the closed 2-uniform 20-list

**Family:** `kuniform` · **Candidate:** PROPOSED — "Still missing from the
closed 20-list: n6, n7 (the two [3.4².6; 3.6.3.6] forms)" · **Effort:** M
(2 landings, one commit each) · **Precedents to mirror (read FIRST):**
`families/kuniform/kuniform.rs` — `kuniform_n14` (M103, L1761: strip-style
primal, seeds by orbit) and `kuniform_n17` (M140, L2347: the pgg sibling
with the exact Q(√3) primitive-period-cell proof) are the freshest primal
landings; `kuniform_n15` (M108) is the cmm/centered mirror. Read their
doc rows in `docs/families/kuniform.md` and the AGENTS.md
"Adding a periodic tiling" workflow (steps 1–5) before writing anything.

### Goal

Land `kuniform-n6` and `kuniform-n7`: the two Wikipedia 2-uniform forms
sharing the vertex pair **[3.4².6; 3.6.3.6]**, as exact Q(√3)
`LatticeTiling` data tables. Two distinct arrangements of the same two
vertex types (the ₁/₂ pattern of the n3/n4 and n14/n15 pairs). The
closed 20-list of 2-uniform tilings completes with this pair: 18 of 20
are in-repo (M103/M108/M114–M121/M140 landed the rest), n6/n7 are the
only missing primaries.

### Construction + verification basis

- **Wikipedia first:** pin the two forms' vertex configurations, t
  (vertex-orbit count), e (edge-orbit count), translational units, and
  declared symmetry groups from Wikipedia's 2-uniform tilings table
  (the n14/n15 rows cite "Wikipedia `2-uniform n14/15`, t, e" — mirror
  that citation style). The two forms differ in arrangement — identify
  WHICH is ₁ vs ₂ per Wikipedia's numbering and say so in the doc rows.
- **Exact geometry:** unit-edge triangular-lattice field Q(√3) (the
  module's `H` = √3/2 and `S3` = √3 constants) — never decimal literals.
  The 3.4².6 vertex carries squares; derive the exact translational unit
  from the arrangement. Prove the emission cell is the primitive period
  cell for its declared lattice (the n17 proof pattern) — or, if the
  natural cell is a supercell, document the index and why.
- **Python spike FIRST** (`/tmp/opencode/kuniform-n6-n7/`, stdlib only,
  ≤240 s per experiment, FOREGROUND with visible per-step output,
  append-only NOTES.md): rebuild each cell over a ±3×±3 patch —
  Σ(cell areas) = |det| to ~1e-15; every interior edge shared by exactly
  2 cells (6-digit rounding); interior vertex angle multisets = the
  vertex figures; fine-grid coverage (irrational offset) exactly once;
  tile-orbit census matches t. Cross-check the Rust table's tuples
  against the spike exactly.
- **Group measurement:** copy the FIXED exact audit
  (`.scratch/duals-n14-n15/group_audit_14_15.py` at the repo root, or
  `/tmp/opencode/duals-n4-n3/group_audit.py`) and adapt. MEASURE both
  forms' groups; expected direction per Wikipedia (one likely p6m) —
  evidence decides; report deviations. Reuse the M145/M147 glide-probe
  lessons: a pmm-vs-cmm-class call needs the off-mirror 2-fold check.
- **Emission gotcha (AGENTS.md step 3):** when several geometric tiles
  of one class share a vertex, the cell must emit each geometric tile
  exactly once (orbit picking) — "edges overlapping 3 ways" in the check
  means the emission set is wrong.
- **First thing:** `grep -rn "kuniform_n6\|kuniform_n7\|kuniform-n6\|kuniform-n7"
  crates/ docs/families/ README.md docs/INDEX.md` — must be ABSENT
  (substring traps: n16/n17 contain "n1", not "n6"/"n7" as full ids).
  If present, STOP and report.

### Files you own

- `crates/tess-tilings/src/families/kuniform/kuniform.rs` — two new
  `LatticeTiling` fns + test trios; register ×2 (AFTER dual-n15, keeping
  the 20-list sibling adjacency convention: ₁ before ₂); classification
  arms ×2 (two_uniform, mirror the n14/n15 arm strings with the
  [3.4².6; 3.6.3.6] vertex pair); census IDS 35 → 36 → 37;
  `lattice_tilings()` entries ×2 (the entries MUST land in the family
  list — culling genuine, not vacuous).
- `docs/families/kuniform.md` — two catalog rows (after dual-n15's, or
  adjacent to the primal block if the doc groups primals separately —
  match the doc's own organization), heading count 35 → 36 → 37,
  (M#) placeholders, batch note if one exists for primal batches.
- `/tmp/opencode/kuniform-n6-n7/` — scratch; append-only NOTES.md.

**Do not touch:** `dual.rs` (closed at M147), `lib.rs`, README,
CHANGELOG, INDEX, ROADMAP, PROPOSED.md, `app_ui.rs` (TWO latent pins
will fire: `assert_eq!(tu.cards.len(), 18)` at ~L1825 → 19 → 20, and
the `("Two-Uniform", 18)` tab-bar pin at ~L1188 — report verbatim,
never fix), any other family, UI crates.

### Constraints (the ones that bite here)

1. Exact Q(√3) end to end; `MAX_TILES = 250_000`.
2. `cargo check --workspace` ZERO warnings; NO repo-wide `cargo fmt`
   (rustfmt reflows pre-existing sibling hunks — hand-match your hunks
   to house style); `cargo test --workspace` green EXCEPT the two
   orchestrator census guards — verbatim output per commit is your delta
   evidence.
3. Your base is main at M147 (built-ins 139 / memberships 153 / Laves
   28 / TwoUniform 18 / kuniform family 35). Totals may move if another
   landing integrates while you work — report what your guards PRINTED,
   never hard-coded numbers; deltas transfer between bases, absolutes
   never do (the M145 integration lesson).
4. Seeds: main/largest tile class 0, others 1, 2 …; pin them in the
   doc rows like n14's "Seeds: 0 square, 1 = {…}, 2 = {…}".
5. Test trio per tiling (`*_covers_region`, `*_rotated_scaled`,
   `*_deterministic`) mirroring the n14/n15 trio style.

### Deliverable (worker report)

1. Diff-scope statement (exactly the owned files, both commits).
2. The pinned forms: ₁/₂ assignment per Wikipedia, vertex configs, t/e,
   lattice basis + det (exact), tiles/cell + orbit structure, measured
   groups with evidence.
3. Spike numbers (area sums, edge parity, orbit censuses, coverage) and
   Rust test counts.
4. Census deltas as printed by the guards PER COMMIT: kuniform family
   35 → 36 → 37; TwoUniform tab 18 → 19 → 20 (the L1825 + tab-bar pins
   fire; report verbatim); built-ins 139 → 140 → 141; memberships
   153 → 154 → 155. Flag that BOTH entries landed in the family
   `lattice_tilings()` list.
5. Proposed commit-message bodies. NO M numbers.