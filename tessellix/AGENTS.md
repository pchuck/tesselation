# AGENTS.md

Instructions for AI agents working in this repo. The project lives in
`tessellix/` and is hosted on GitHub as `pchuck/tesselation`. The git
root is `/workspace` (the repo was re-initialized as a single squashed
commit when it moved to GitHub); the original per-tiling commit history
is archived, gitignored, in `tessellix/.git.bak/`.

## Commands

```sh
cargo test --workspace        # full suite; must stay green
cargo test -p tess-tilings <test_name>   # single test
cargo check --workspace --all-targets  # convention: ZERO warnings, test targets included (enforced on every commit)

cargo run -p tess-tui         # TUI — needs a REAL terminal (raw mode); do not run headless in scripts
cargo run -p tess-tui -- list
cargo run -p tess-tui -- export --tiling ID --width 1200 --height 900 \
    --scale F --rotation DEG --mode single|pattern|proto|gray|pattern-gray --out FILE
cargo run -p tess-gui         # egui window — needs a display (headless containers: can't run)
```

Visual verification without a display: `export --mode proto` (colors by
prototile index, so tile classes are distinguishable) and inspect the SVG.
`--scale 1.0` gives a 20-world-unit-tall view; use ~2.0 to zoom into one
fundamental region.

## Architecture (what is NOT obvious from filenames)

- **`tess-core`** — all abstractions, zero external deps. The
  `Tiling` trait (`src/tiling.rs`) is the only extension point.
  **Contract: `generate` must return exactly the tiles intersecting the
  query `BBox` — no gaps, no overlaps, no outside tiles.** The test
  helpers `check_coverage` (sample points + clipped-area equality to
  1e-6 rel.) and `check_deterministic` (geometry, proto, AND seed) live
  in `tess-tilings/src/lib.rs` and enforce this; every tiling has 3
  standard tests: `*_covers_region`, `*_rotated_scaled`,
  `*_deterministic`. For the periodic tables,
  `lattice_culling_matches_brute_force` additionally re-emits every tile
  over a fixed index window and requires the exact same tile set as the
  generator's range-based culling — add the entry to the family's
  `lattice_tilings()` list when adding a new `LatticeTiling`.
- **`tess-tilings`** — the built-ins. Periodic tilings are **data tables**
  (`LatticeTiling` in `src/lattice.rs`: lattice basis, cell polygons,
  wallpaper group, seed pattern), not code. Tilings are grouped into ten
  **families** (`src/families/`: uniform, quasiperiodic, arrangements,
  pentagons, laves, kuniform, reptiles, aperiodic, pavers, islamic) —
  each family is a cargo feature (all default-on) and owns its modules,
  registration slice, classification arms and census pin; a tiling is
  added by touching only its family file. The aperiodic /
  nonperiodic members (penrose, pinwheel, hirschhorn, chair, sphinx,
  hat, turtle, spectre, …) implement `Tiling` directly.
- **`tess-tui`** (ratatui+crossterm; software rasterizer into char cells)
  and **`tess-gui`** (eframe/egui 0.29) are both thin layers over
  `tess_core::App`. Parameters are self-describing (`ParamSpec`), so
  **adding a tiling never requires touching UI code**.
- **`docs/`** — documentation in three guard-checked tiers: the
  per-tiling documents (`docs/tilings/<id>.md`, one per registry
  member: algorithm, exact geometry, verification, sources; guarded by
  `tiling_docs_match_registry`), one doc per family
  (`docs/families/<family>.md`: the family's catalog table, its
  numbered build arcs, and its Per-tiling-documents link section;
  guarded by `family_docs_match_registrations`), and the cross-cutting
  reference docs `docs/DESIGN.md` (architecture; carries the full
  Tests description) and `docs/USAGE.md` (TUI/CLI/GUI walkthrough).
  The README is the guard-checked overview (`readme_matches_registry`).
  The candidate queue and research areas live in `docs/ROADMAP.md`, the
  gallery taxonomy in `docs/TAXONOMY.md`, the append-only milestone
  changelog in `docs/history/CHANGELOG.md`; `docs/INDEX.md` is the map.

## Adding a periodic tiling (the common case)

1. Exact geometry first: express vertices in radicals via the `H = √3/2`
   constant (as in the other tables) — never decimal literals. Exception:
   measured ornamental panels (image-derived data tables, e.g. M35
   `girih-star-cross`) may use decimal literals, but must document the
   measurement pipeline and verify the cell partition exactly (step 2
   below) on the rounded data that ships.
2. Verify numerically BEFORE writing Rust (Python, stdlib only) over a
   ±3×±3 lattice patch of your cell:
   - `sum(cell areas) == |det(a1,a2)|` to ~1e-15;
   - every interior edge shared by **exactly 2** cells (key edges by
     rounded endpoint pairs — round to 6 digits; 9 digits create
     spurious T-junctions after translate round-trips);
   - every interior vertex angle multiset == the vertex figure;
   - a fine grid sample (shifted by an irrational offset) covered
     exactly once. This workflow has caught real bugs (wrong edge parity).
3. **Emission gotcha:** when several geometric tiles of one class share a
   vertex, the cell must contain copies so that each geometric tile is
   emitted exactly once by exactly one cell (e.g. 3.12.12: each triangle
   is shared by 3 dodecagons, so a cell emits 2 of the 6 surrounding
   triangles — pick one edge from each 60°-invariant orbit). If edges
   "overlap 3 ways" in the check, the emission set is wrong.
4. Name with the Wikipedia-verified title + vertex figure
   (e.g. "Truncated Hexagonal (3.12.12)"); verify the name before coding
   (4.6.12 = truncated trihexagonal / great rhombitrihexagonal;
   3.6.3.6 = trihexagonal; 3.3.4.3.4 = snub square — in this repo as
   `snub-square`).
5. Append the fn to the family's module (`families/<family>/<name>.rs`,
   e.g. `families/uniform/archimedean.rs`), then edit only the family
   file `families/<family>.rs`: module decl + `register` slice +
   `classification` arm + census `IDS` (+ `lattice_tilings()` for
   lattice tables). Declare its category in the module
   (`TilingCategory`: Uniform / Laves / Wallpaper / Quasiperiodic /
   Aperiodic / Nonperiodic — see `tess-core/src/category.rs`; the UIs
   group their lists by it), add the 3 standard tests, add the catalog
   row + Per-tiling-documents link to `docs/families/<family>.md`
   (guard-checked), and write the per-tiling document
   `docs/tilings/<id>.md` (guard-checked) — plus a numbered build arc
   when the build introduces a new technique or a hard-won lesson
   (row-only is otherwise fine; M127–M129 precedent).

## Fleet protocol (parallel agents)

The family refactor made **one family = one collision-free unit**: a
family's code files and its family doc never overlap with another
family's. When several agents work in parallel, each executes exactly
one brief (template: `docs/briefs/TEMPLATE.md`, worked examples in the
same directory) and touches only its own surfaces.

**Worker ownership** (for the brief's duration, exclusive):

- `crates/tess-tilings/src/families/<family>/` — the new module(s).
- `crates/tess-tilings/src/families/<family>.rs` — module decl,
  `register` slice, `classification` arms, census `IDS`,
  `lattice_tilings()` entries.
- `docs/families/<family>.md` — the catalog row (and arc if
  arc-worthy) plus the Per-tiling-documents link line.
- `docs/tilings/<id>.md` — the new tiling's per-tiling document
  (`# <registry name>` H1, metadata line, specification / algorithm,
  See-also; guarded by `tiling_docs_match_registry`).
- Scratch under `/tmp/opencode/<brief-id>/` (Python spikes, batteries).

**Orchestrator-only** (workers never edit):

- `crates/tess-tilings/src/lib.rs` — the pinned census
  (`registry_groups_are_complete_and_canonically_ordered` + the census
  comment block), the doc/README guard tests, and the
  `lattice_culling_tests` `lattices.extend(...)` lines. A
  `LatticeTiling` brief therefore flags the new tiling in its report
  instead of wiring the culling test itself.
- `README.md` (census sentence + per-tab prose + family-table count),
  `docs/INDEX.md` (family sizes).
- `docs/history/CHANGELOG.md` — append-only, integrated by the
  orchestrator.
- `docs/ROADMAP.md` (queue status flips) and `PROPOSED.md` (dossier
  annotations).
- Every other family's files, `tess-core`, `tess-tui`, `tess-gui`.
  Adding a tiling never requires UI code (parameters are
  self-describing); if a brief seems to, stop and report — the design
  is wrong.

A brief whose *goal* is itself cross-cutting (a core trait, taxonomy,
UI feature) is not dispatched this way — run it solo on the affected
files with family work paused there.

**M-number allocation is orchestrator-only.** Workers do not know
their M number and never write changelog entries; numbers are assigned
strictly sequentially at integration (the last entry in
`docs/history/CHANGELOG.md` is the head). The worker's report proposes
the commit-message body; the orchestrator writes the final
`M<n>: <name> tiling (<vertex figure>)` subject.

**Worker report** (what "done" means):

1. Diff-scope statement: exactly the owned files, nothing else.
2. Verification evidence, programmatic only — the Python battery's
   numbers (area sums, edge parity, orbit censuses, dissection counts)
   and the Rust test list with exact pass counts. A visual look is
   recon, never evidence.
3. Expected census deltas (family n → n+1; built-ins; memberships;
   `LatticeTiling`? → remind the orchestrator about the culling line).
4. Proposed commit-message body (lattice, cell contents, verification).

**Integration checklist** (orchestrator, per landing):

1. Scope check: `git diff --stat` shows only owned files.
2. `cargo check --workspace` zero warnings; `cargo test --workspace`
   green.
3. Guards green (`family_docs_match_registrations`,
   `tiling_docs_match_registry`, `readme_matches_registry`,
   `registry_groups_are_complete_and_canonically_ordered`); add the
   culling-test line first if the tiling is a lattice table.
4. Shared-file updates: README census + per-tab prose + family-table
   count, `docs/INDEX.md` size, ROADMAP status flip, PROPOSED
   annotation, CHANGELOG append.
5. Assign `M<n>` and land **one commit per tiling** with the
   shared-file updates folded in — the M-commit is the unit of history.

**Seeds:** main/largest tile = 0, other classes 1, 2, …; "pattern" color
= `seed % 8`, "proto" = `proto % 8` (8-color palette in
`tess-core/src/paint.rs`).

## Conventions

- One commit per tiling, message `M<n>: <name> tiling (<vertex figure>)`
  with a body stating the lattice, cell contents, and the verification
  performed (worked examples in the archived `tessellix/.git.bak/`
  history; the tiling's family doc `docs/families/<family>.md` records
  the same per-tiling details).
- Keep `cargo check --workspace --all-targets` warning-free (the
  `--all-targets` matters: plain `cargo check` skips `#[cfg(test)]`
  modules, where warnings otherwise hide).
- Tile budget: `MAX_TILES = 250_000` (`lattice.rs`) — exceeding it
  returns the "too many tiles for this view — zoom in" message, which the
  GUI shows on-canvas; keep generators cheap (the GUI regenerates every
  frame).
- No CI, no rustfmt/clippy config, no pre-commit hooks — `cargo fmt`
  defaults only.

## Environment quirks

- `/tmp` is wiped between sessions; use `/tmp/opencode` for scratch.
  Verification scripts (rth_check.py, tt12_*, th13_check.py, svg2png.py —
  a stdlib SVG→PNG rasterizer for inspecting exports) were kept there and
  are the templates to copy.
- Python 3.11, **stdlib only** (no shapely/PIL/cairosvg). Cap experiments
  at ~240 s.
- Long shell/commit messages can be garbled by transcript rendering —
  always re-verify file/commit state on disk after big writes
  (`grep -c`, `wc -l`, `git log -1 --format=%B`).
