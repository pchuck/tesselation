# AGENTS.md

Instructions for AI agents working in this repo. The project lives in
`tessellix/` and is hosted on GitHub as `pchuck/tesselation`. The git
root is `/workspace` (the repo was re-initialized as a single squashed
commit when it moved to GitHub); the original per-tiling commit history
is archived, gitignored, in `tessellix/.git.bak/`.

## Commands

```sh
cargo test --workspace        # full suite (745 tests); must stay green
cargo test -p tess-tilings <test_name>   # single test
cargo check --workspace       # convention: ZERO warnings (enforced on every commit)

cargo run -p tess-tui         # TUI — needs a REAL terminal (raw mode); do not run headless in scripts
cargo run -p tess-tui -- list
cargo run -p tess-tui -- export --tiling ID --width 1200 --height 900 \
    --scale F --rotation DEG --mode single|pattern|proto --out FILE
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
  generator's range-based culling — extend its `lattices` list when
  adding a new `LatticeTiling`.
- **`tess-tilings`** — the built-ins. Periodic tilings are **data tables**
  (`LatticeTiling` in `src/lattice.rs`: lattice basis, cell polygons,
  wallpaper group, seed pattern), not code. Each family is behind a cargo
  feature (all default-on); aperiodic families (penrose, pinwheel,
  hirschhorn, chair, sphinx, hat, turtle, spectre) implement `Tiling`
  directly.
- **`tess-tui`** (ratatui+crossterm; software rasterizer into char cells)
  and **`tess-gui`** (eframe/egui 0.29) are both thin layers over
  `tess_core::App`. Parameters are self-describing (`ParamSpec`), so
  **adding a tiling never requires touching UI code**.

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
5. Append the fn to `src/archimedean.rs` (+ header line), register in
   `registry()` in `src/lib.rs`, declare its category in the table
   (`TilingCategory`: Uniform / Laves / Wallpaper / Quasiperiodic /
   Aperiodic / Nonperiodic — see `tess-core/src/category.rs`; the UIs
   group their lists by it), add the 3 standard tests.

**Seeds:** main/largest tile = 0, other classes 1, 2, …; "pattern" color
= `seed % 8`, "proto" = `proto % 8` (8-color palette in
`tess-core/src/paint.rs`).

## Conventions

- One commit per tiling, message `M<n>: <name> tiling (<vertex figure>)`
  with a body stating the lattice, cell contents, and the verification
  performed (worked examples in the archived `tessellix/.git.bak/`
  history; PATTERNS.MD records the same per-tiling details).
- Keep `cargo check --workspace` warning-free.
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
