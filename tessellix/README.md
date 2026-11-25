# tessellix

A Rust tiling generator with two front ends: an interactive terminal UI
(TUI) and a desktop GUI. It generates 2D tilings of the plane on demand —
uniform/Archimedean, wallpaper, Penrose, Pythagorean, and monohedral
nonperiodic families — and renders them live, with pan/zoom, tunable
parameters, four color modes, and SVG export.

- **License:** MIT
- **Rust:** edition 2021, MSRV 1.85
- **Dependencies:** `tess-core` is dependency-free; the TUI uses
  `ratatui` + `crossterm`, the GUI uses `eframe`/`egui`. No geometry or
  rendering libraries elsewhere — all geometry is implemented in-house.
- **Documentation:** this file is the overview. Per-family detail lives
  in `docs/families/<family>.md`, each tiling's full specification in
  [`docs/tilings/<id>.md`](docs/tilings/) (algorithm, exact geometry,
  verification, sources), the architecture reference in
  [docs/DESIGN.md](docs/DESIGN.md), the UI walkthrough in
  [docs/USAGE.md](docs/USAGE.md) — the map is
  [docs/INDEX.md](docs/INDEX.md).

## Build and run

```sh
cargo build                 # build everything
cargo test --workspace      # run the full test suite

cargo run -p tess-tui       # terminal UI (needs a real terminal)
cargo run -p tess-gui       # desktop GUI (egui window)
```

### TUI / CLI

`cargo run -p tess-tui` starts the terminal UI: a two-mode browser with a
**Browse** mode (category tabs + card grid) and a full-screen **Render**
mode with back navigation. With arguments it acts as a CLI:

```sh
cargo run -p tess-tui -- list
cargo run -p tess-tui -- export --tiling truncated-hexagonal \
    --width 1200 --height 900 --scale 2.0 --rotation 0 \
    --mode proto --out tiling.svg
```

All `export` flags (`--tiling`, `--width/--height`, `--scale`,
`--rotation`, `--mode`, `--strapwork`, `--decoration`, `--band-w`,
`--out`) are tabulated in
[docs/USAGE.md](docs/USAGE.md), together with the full walkthrough of
Browse mode, search, Render mode, keybindings, and mouse behavior.

### GUI

`cargo run -p tess-gui` opens an egui window (1024×640, dark theme) —
the same two-mode browser over the shared browse model, with a
responsive card grid, lazily rasterized previews, drag-pan/scroll-zoom
rendering, and a collapsible parameter panel. Details in
[docs/USAGE.md](docs/USAGE.md).

### Rendering star patterns: strapwork and decoration styles

Two export layers built on the polygons-in-contact (Hankin PIC) engine:
`--strapwork DEG` overlays the strapwork walls on any tiling (plain SVG;
δ measured from each tile edge — small δ packs tight stars, large δ opens
crosses; classic 45° on square / 60° on hexagonal hosts), and
`--decoration STYLE` + `--band-w W` (default 0.1) renders the four Kaplan
§3.7 ornamentation styles — `fill`, `outline`, `interlace`,
`outline-interlace` — over those walls. `--decoration` requires
`--strapwork`; `--mode` is ignored under `--decoration`; generic δ fails
with honest precondition errors naming the regime.

```sh
cargo run -p tess-tui -- export --tiling hexagon --width 1200 --height 900 \
    --strapwork 60 --out hexagon-walls-60.svg
# 237 tiles, 4266 <line> elements, 0 NaN

cargo run -p tess-tui -- export --tiling hexagon --width 1200 --height 900 \
    --strapwork 60 --decoration fill --band-w 0.1 --out hexagon-fill-60.svg
# 237 tiles, 1807 faces (1220 lapis + 587 bone), 0 NaN
```

Twelve even-degree classic-δ rows pass all four styles (square @
45°/90°, hexagon @ 30°/60°, triangle @ 30°/60°, trihexagonal/
hexagram/pic-3-6-3-6/truncated-hexagonal/snub-trihexagonal @ 60°,
truncated-square @ 45°). Full detail — per-style preconditions and error
text and the complete table — is in
[docs/rendering.md](docs/rendering.md). All four styles render cleanly
(zero non-finite coordinates; verified on the full 12×4 matrix and by the
`decoration_finiteness_battery` test).

## Supported tilings

163 built-in tilings (184 tab memberships), all discoverable in both UIs
via the registry (`tess-tui -- list`). Every tiling declares a primary
display category — its family — plus optional secondary tabs (e.g. the
self-dual regular tilings also appear in Laves, and seventeen periodic
tilings anchor the per-group Wallpaper catalog — all 17 groups). The
eleven tabs:
**Uniform** (11), **Two-Uniform** (20), **Laves / Catalan duals** (33,
25 primary + 3 self-dual secondaries + 5 isohedral-type realizations),
**Wallpaper** (17: one card per
wallpaper group — the full 17-group catalog, completed by M85),
**Pentagon Types** (17: types 1–3 and 6–15 as primary entries, plus
cairo-pentagonal (type 4), floret-pentagonal (type 5) as typed
secondary members and the untyped isohedral pentagon interlocks
ih27/ih29), **Isohedral** (5: Kaplan IH templates — IH1/IH6/IH7
hexagons on the 6³ net, IH27/IH29 pentagons on the Cairo net),
**Islamic** (29), **Arrangements** (8),
**Quasiperiodic** (21: Penrose family, Ammann–Beenker (both variants),
Stampfli, shield, golden bee, golden triangle, golden rhomboid,
fractalizing 3.12.12, girih Darb-i Imam, GLB binary n=7, Andritz
bowtie-hexagon, Andritz 10gon-1, Andritz 10gon-2, Andritz 10gon-3,
fibonacci-fibonacci,
wis 12-fold, girih 14-fold), **Aperiodic** (12: einstein family, CAP
cut-and-project, pinwheel,
Taylor-Socolar, Jeandel-Rao 11-tile Wang tiles, half-hex, domino
(table), period-doubling, thue-morse, period-tripling), and
**Nonperiodic** (11: limit-periodic rep-tiles, Hirschhorn,
pregnant-chair, tangram, octiamond reptile, pentiamond AC factor-2,
pentiamond AC factor-3, pentiamond AD factor-2). Categories
are declared on the tiling (never derived from its symmetry), so the
grouping is honest about what the mathematics distinguishes; see
`tess-core/src/category.rs`.

Every periodic tiling reports its wallpaper group, lattice kind, and (for
periodic ones) its lattice basis as analytic symmetry metadata. The
framework models all 17 wallpaper groups and the five lattice kinds
(oblique, rectangular, centered rectangular, square, hexagonal).

Each periodic tiling accepts the same two parameters, discovered by the
UIs: **scale** (0.1–50, step 0.05) and **rotation** (0–360°, step 5°).
Three families add a third: star-and-cross takes **star radius**
(0.05–0.5), spanning gapped layouts to the kissing (khatam) case;
hexagram takes **star tip radius** (0.05–0.5, default 0.5), spanning
gapped layouts to the kissing hexagram-and-hexagon case; and the
Pythagorean two-square family takes the **square ratio** b/a
(0.2–10, default φ).

### The eleven families

| Family (tab) | Tilings | Doc |
|---|---|---|
| [Uniform](docs/families/uniform.md) | 11 | the three regular + eight semiregular (Archimedean) patterns — a closed set |
| [Two-Uniform](docs/families/kuniform.md) | 37 | k-uniform tilings: 20 two-uniform forms + 17 face-centroid duals (the duals' primary tab is Laves) |
| [Laves / Catalan duals](docs/families/laves.md) | 8 | the eight Laves (Catalan-dual) tilings — a closed set; hosts the two-uniform duals |
| [Wallpaper](docs/families/pavers.md) | 14 | paver bonds + the wallpaper-catalog completers (first realizers of p1, pm, pg, pmg, cm, p3, p31m, p3m1) |
| [Pentagon Types](docs/families/pentagons.md) | 14 | pentagonal types 1–15 (the 15-type census; types 4–5 dual-typed) |
| [Islamic](docs/families/islamic.md) | 34 | girih strapwork panels, Hankin/PIC ornament, khatam + najm templates, Andritz fivefold substitutions, the Darb-i Imam decagon documentary patch |
| [Arrangements](docs/families/arrangements.md) | 3 | Pythagorean two-square, periodic pinwheel spin, pinwheel substitution |
| [Quasiperiodic](docs/families/quasiperiodic.md) | 16 | Penrose family, Ammann–Beenker, Stampfli, shield, golden A2 members, GLB binary n = 7, fibonacci product, WIS 12-fold, girih 14-fold |
| [Aperiodic](docs/families/aperiodic.md) | 11 | einstein family (hat, turtle, spectre), CAP cut-and-project, Wang tiles, Taylor-Socolar, limit-periodic substitution set |
| [Nonperiodic](docs/families/reptiles.md) | 10 | rep-tiles (chair, armchair, sphinx, P-pentomino, tangram, octiamond + pentiamonds), pregnant-chair |
| [Isohedral types](docs/families/isohedral.md) | 5 | parameterized isohedral (IH) types — incidence symbols over their topologies with exact per-orbit shaped edges (the Escher arc; design doc docs/isohedral-generator-design.md) |

Tab counts above include secondary memberships; the **Tilings** column
counts a family's registered members, whose primary tab may belong to
another family (the two-uniform duals are Laves-categorized, girih
Darb-i Imam and the Andritz substitutions Quasiperiodic-categorized).

Each family doc carries the family's catalog table and its numbered
build arcs, and links every member's per-tiling document —
`docs/tilings/<id>.md` — where the full specification lives: algorithm,
exact geometry, verification evidence, and sources. Mathematical
background: [docs/2D_Tilings_Reference.md](docs/2D_Tilings_Reference.md);
gallery taxonomy: [docs/TAXONOMY.md](docs/TAXONOMY.md).

## Design

Four crates: `tess-core` holds the `Tiling` trait — generate exactly the
tiles intersecting a query bbox, no gaps/overlaps/outside tiles — plus
self-describing parameters, the browse model, and software rendering;
`tess-tilings` holds the 162 built-ins in eleven families (periodic tilings
are `LatticeTiling` **data tables**, aperiodic/nonperiodic ones direct
substitution generators with exact-arithmetic anchors); `tess-tui`
(ratatui) and `tess-gui` (egui) are thin front ends over the shared
`tess_core::App`. Because parameters are self-describing (`ParamSpec`),
adding a tiling never requires touching UI code.

Full details — trait contract, the lattice framework, per-generator
algorithm notes, UI state, rendering, and the test suite — in
[docs/DESIGN.md](docs/DESIGN.md); the exact degree-3/4 cyclotomic
arithmetic layer is designed in
[docs/cyclotomic-field-layer.md](docs/cyclotomic-field-layer.md).

## Tests

`cargo test --workspace` — currently 1356 tests, all green, zero
warnings. Every tiling ships the 3-test standard trio:
`covers_region` (sample points inside ≥ 1 tile and total clipped tile
area equal to the region area to 1e-6 relative — catching gaps *and*
overlaps), `rotated_scaled`, and `deterministic`. Periodic tables add a
brute-force culling cross-check; the substitution generators carry
invariant censuses (count laws, exact per-generation dissections,
T-junction censuses, no-period probes). Doc-sync guards
(`readme_matches_registry`, `family_docs_match_registrations`,
`tiling_docs_match_registry`) fail the suite when any document drifts
from the registry. Full description in [docs/DESIGN.md](docs/DESIGN.md).

## Adding a tiling

The common case is a periodic data table; the full contract and the
fleet protocol (parallel workers, one family = one collision-free unit)
live in [AGENTS.md](AGENTS.md). The short version:

1. **Geometry first** — exact radicals via the `H = √3/2` constant,
   never decimal literals; verify the cell numerically in Python before
   writing Rust (area sum == |det|, every interior edge shared by
   exactly 2 cells, vertex figures, grid-sample coverage).
2. **One family** — new module `families/<family>/<name>.rs`, then only
   the family file `families/<family>.rs`: module decl, `register`
   slice, classification arm, census `IDS` (+ `lattice_tilings()` for
   lattice tables). Nothing outside the family changes; both UIs pick
   the tiling up automatically.
3. **Tests** — the 3-test standard trio via `check_coverage` /
   `check_deterministic` (plus family-specific invariants).
4. **Docs** — catalog row (+ build arc if arc-worthy) in
   `docs/families/<family>.md`, and the per-tiling document
   `docs/tilings/<id>.md`; both are guard-checked against the registry.