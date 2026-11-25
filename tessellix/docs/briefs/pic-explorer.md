# Brief: pic-explorer — δ/θ sweep + programmatic scoring over host tilings (24h)

> **LANDED as M170 (2026-11-25):** the sweep module + `explore` CLI are
> on main; the leaderboard and shortlist were banked at integration.
> This brief is kept as the dispatch record.
> **Preconditions MET (2026-11-24):** 22g (M164 + NaN fix, all four
> §3.7 styles render) and 22d (M165: `walls_with_delta`, envelope
> machinery) are landed. **Family:** none — sweep harness + CLI mode;
> **NO registry changes** (built-ins stay 162, memberships 183; tests
> 1300 + yours). **Tier:** fast/worker. **Effort:** M. PROPOSED §24h.

## Goal

The generate-and-filter stage the pipeline lacks (user-pinned framing):
sweep the contact angle over host tilings, score every candidate
programmatically, render ranked galleries via the 22g styles. Winners
become exact-pin registry candidates — **exploration proposes,
exactness disposes**. Deterministic end-to-end (no RNG; §15 policy).

## Landed surface you build on (verify, then use)

- `tess_core::strapwork::walls(tiles, δ)` / `walls_with_delta(tiles, f)`
  (M57/M165); `envelope_for(substrate)`, `validity_stats`,
  `snap_to_envelope`, `BuiltInProfile` (M165, `tess-tilings/src/
  envelope.rs`).
- **M165 finding that shapes this job:** the closure envelope is a
  DISCRETE point set per substrate ({45°,90°} squares,
  {60°,90°} trihexagonal, {60°} truncated-hexagonal). Generic δ
  produces open strands with degree-1 exits — **open-strand regimes
  are legitimate design space** (historical patterns have open
  straps), so the sweep scores ALL δ but records closure status per
  candidate (envelope member or not) and the style gates (fill/
  interlace need even-degree maps) report honestly per the M164
  region-gated preconditions.
- Per-tile-class θ: M163's motif assignment (`build_cells_with_motifs`,
  `Motif::Star/Rosette(n, s, θ)`) sets θ per tile class — the second
  sweep axis. First version: global δ sweep on every lattice-carrying
  substrate + per-tile-class θ sweeps on the hankin engine substrates
  (2-class substrates first: 4.8.8, 3.12.12 families).
- Render: `render_svg_with_walls` and M164's `render_svg_decorated`
  (fill/outline/interlace/outline-interlace — all NaN-free post-fix).

## The harness

New module in tess-tilings (it consumes registry substrates +
`envelope_for`; render via tess-core) — e.g. `sweep.rs`:

1. **Sweep grid** (deterministic): δ ∈ {multiples of 5° from 5° to 85°}
   per substrate (finer only where the closure census says a closure
   point is nearby — the envelope's 1e-1° fine-scan trick); per-tile
   class θ offsets ∈ {−15°, −10°, −5°, 0°, +5°, +10°, +15°} relative to
   the class's classic value on 2-class hankin substrates. Fixed patch
   window per substrate (the M165 CLI's region convention).
2. **Per-candidate feature vector** (all programmatic, all documented):
   closure status (envelope member); wall-map vertex degree histogram;
   crossing count + min crossing angle; min face area (sliver metric);
   strap density (total wall length / area); crossing density; strand
   count; symmetry checks — (a) lattice-translation invariance of the
   wall multiset on the overlap of shifted patches (the M163 fold
   trick), (b) claimed rotation/mirror invariance via sampled patch
   comparison where the substrate's classification asserts them.
3. **Leaderboard**: deterministic sort (documented key order; ties by
   substrate id then δ), exported as JSON + CSV. Every row carries the
   full feature vector — no hidden state.
4. **Gallery**: top-K per substrate rendered one SVG each, plain walls
   + the best 22g style the candidate qualifies for (gate-honest).
   File naming `gallery/<substrate>-d<δ>-r<rank>.svg`.
5. **CLI**: `tess-tui explore --tiling ID ...` (or a subcommand — your
   call, follow the landed parse_export pattern) with honest errors;
   deterministic output bytes across runs (two-run byte-identity test).

## Deliverables

- The harness + battery (determinism, feature-vector spot-pins vs hand
  computed values on small patches at 1e-9, gate-honesty, gallery
  element counts, leaderboard schema).
- **A shortlist**: ≤10 (substrate, δ/class-θ) winners by documented
  criteria, each with its feature vector — formatted as PROPOSED-ready
  candidate notes (in the report, NOT in PROPOSED/registry).
- NEW battery test (22d follow-up): the ray-pair continuity identity
  under VARYING f — at every shared edge midpoint of a deformed wall
  set, the two incident ray pairs are point reflections (1e-9). The
  M165 battery pinned this at constant f only; the sweep relies on the
  varying-f claim, so pin it.

## Scope guards

- No registry/census movement; no PROPOSED/ROADMAP edits (orchestrator
  owns); winners are report content only.
- Do NOT attempt aesthetic objectives beyond the three pinned metrics
  (band-balance/Castéra is 24e/24g territory, source-gated).
- Perf: if symmetry checks make the full grid slow, reduce sampling and
  document — determinism over coverage.

## STOP rules

- A feature metric that cannot be computed programmatically for a
  whole substrate class → STOP with the failing case.
- The sweep surfaces wall-set behaviour contradicting the M165 envelope
  table (e.g. a closed weave at a non-envelope δ) → STOP — that is an
  engine finding, report with evidence.
- CLI/parse drift vs the landed surface → STOP and report.

## Bookkeeping

One commit, explicit `git add` paths only (never `-A`), subject like
`core: PIC explorer — deterministic δ/θ sweep with programmatic
scoring (24h)`, message via `git commit -F <file>`; verify `git show
HEAD --stat`. Scratch in `.scratch/pic-explorer/` (worktree) with a
persistent copy of the leaderboard + shortlist to repo-root
`.scratch/pic-explorer/` before finishing. Report: commit hash,
feature-vector schema, leaderboard highlights, the shortlist with
feature vectors, gallery notes, test counts, deviations, follow-ups.