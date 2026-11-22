# Brief: thue-morse-2d — §23d aperiodic product substitution

**Family:** `aperiodic` · **Candidate:** PROPOSED.md §23d "Period
doubling / period tripling / Thue–Morse 2D / Squiral — marginal
supersubstitutions; effort S–M each" (**this brief covers ONLY
Thue–Morse 2D** — period tripling and Squiral stay parked) ·
**Effort:** S · **Precedent to mirror:**
`families/aperiodic/period_doubling.rs` (M141 — the in-repo template
for the SAME product construction: 4 label classes on Z² unit squares,
exact integer triples, depth-quantized view rule, patch-stability
trio; read it first) and `domino.rs` (classification-arm shape).

### Goal

Land the 2D Thue–Morse substitution tiling under registry id
`thue-morse` in the `aperiodic` family. **Classification differs from
domino/period-doubling**: expected `TilingCategory::Aperiodic` with
translational status **"aperiodic"** (the Thue–Morse hull has no
periodic elements — its diffraction spectrum is singular continuous,
not pure point), method substitution. Evidence decides; report
deviations; STOP before coding Rust if your battery disagrees with
the no-periods expectation.

### Literature basis

- 1D rule: the classic Thue–Morse morphism **τ: 0 → 01, 1 → 10**
  (Bielefeld entry "Thue Morse",
  <https://tilings.math.uni-bielefeld.de/substitution/thue-morse/> —
  1D, 2 prototiles, discovered by Thue/Prouhet/Morse; reference JS99
  = Allouche & Shallit, *The ubiquitous Prouhet-Thue-Morse sequence*,
  Springer 1999). **The entry is 1D only**; the rule itself is the
  textbook morphism — pin it against JS99's statement, not the image.
- 2D member: the **product construction τ⊗τ** on Z² unit squares —
  4 label classes (00/01/10/11, class = 2·τ(row-letter index) … same
  encoding scheme as `period_doubling.rs`), each parent inflates λ = 2
  into a 2×2 frame, child (c, r) label = (τ(x)_c, τ(y)_r).
- **Substitution matrix = the all-ones 4×4**: every parent block
  contains each child class exactly once (the Steinhaus property).
  Consequences to verify in the spike: column sums all 4 = λ², PF
  eigenvalue 4, char poly x⁴ − 4x³ (eigenvalues {4, 0, 0, 0}). If
  your enumeration disagrees, STOP and report — the pin, not the
  code, is wrong.
- **Expected period structure — the distinguishing signature**:
  unlike domino (ratio-2 (2^g, 2^(g+1)) manifest, M138) and
  period-doubling (ratio-1 (2^g, 2^g) manifest, M141), the Thue–Morse
  structure is **aperiodic at every depth** — measure the minimal
  translation period of the depth-g patch for g = 1..6 with the same
  methodology as `period_doubling.rs`'s `period_hierarchy` test and
  expect **no period at any depth** (report the exact
  candidate-shift counts you rejected). A measured period ⇒ STOP and
  report before writing Rust.
- Exact arithmetic: pure Z² end to end, floats only at the
  emission/view boundary.

### Files you own

- `crates/tess-tilings/src/families/aperiodic/thue_morse.rs` — the
  new module.
- `crates/tess-tilings/src/families/aperiodic.rs` — module decl +
  register slice + classification arm + census IDS. (Not a lattice
  table → no `lattice_tilings()` change.)
- `docs/families/aperiodic.md` — catalog row (+ heading count).
- `/tmp/opencode/thue-morse-2d/` — scratch; append-only NOTES.md.

**Do not touch:** `lib.rs`, README, CHANGELOG, INDEX, ROADMAP,
PROPOSED.md, sibling aperiodic modules (`period_doubling.rs` /
`domino.rs` / `half_hex.rs` are read-only references), other families,
UI crates, AGENTS.md.

### Commit exception (this brief only)

One worktree commit, subject like
`aperiodic-family: thue-morse 2D substitution` — NO M numbers, NO
pushes. The orchestrator cherry-picks and assigns the M number at
integration.

### Constraints (the ones that bite here)

1. Exact Z² end to end; `MAX_TILES = 250_000`; depth rule = octave
   quantization of the view extent with patch-stability verified in
   tests (copy the trio pattern from `period_doubling.rs`).
2. Python spike first (stdlib only, ≤240 s per experiment, run in the
   FOREGROUND with visible per-step output):
   - substitution consistency: depth-g partition == depth-(g+1)
     partition (bucketed edge/vertex matching, rounded 6 digits);
   - 0 overlaps + exact single coverage (fine grid, irrational
     offset) over a ±3 patch;
   - substitution matrix all-ones verified by direct block
     enumeration; column sums 4; PF 4; char poly x⁴ − 4x³ (assert
     permutation-expansion counts if used — M135 lesson);
   - period manifest: NO period at any depth g = 1..6 (aperiodic
     signature) — report the rejected candidate-shift counts;
   - label census per depth vs S^g · e_00 (with S all-ones: depth-g
     census = [4^(g−1), …, 4^(g−1)]? — derive and pin whatever the
     exact recursion gives).
3. `cargo check --workspace` zero warnings; `cargo fmt` owned hunks
   only; `cargo test --workspace` green except the two orchestrator
   census guards (`registry_groups…`, `readme_catalog…`) — their
   verbatim failure output is your delta evidence.
4. You branch from main at M141 (131 built-ins / 145 memberships /
   Aperiodic tab 9 / aperiodic family 8), but totals may move while
   you work — report what your guards PRINTED, never hard-coded
   numbers.
5. Doc row wording: "4 prototile label classes (seeds 0–3) over one
   square prototile"; aperiodic, singular continuous diffraction
   (JS99); lattice field "—", group "—".

### Deliverable (worker report)

1. Diff-scope statement (exactly the owned files).
2. Pinned construction: entry URL, the 1D rule, τ⊗τ product
   definition, all-ones matrix, PF 4, char poly.
3. Battery numbers; Rust pass counts; the measured no-period manifest
   g = 1..6 with rejected-shift counts.
4. Expected census deltas as printed by your guards: aperiodic family
   8 → 9; Aperiodic tab 9 → 10; built-ins 131 → 132; memberships
   145 → 146. Not a lattice table → no culling lines.
5. Proposed commit-message body. NO M numbers.