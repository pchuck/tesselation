# Usage: TUI, CLI, and GUI

The full walkthrough of both front ends. For build commands and a quickstart see the [README](../README.md); the `export` flag table lives there too.

## TUI / CLI

`cargo run -p tess-tui` starts the terminal UI: a two-mode browser with a
**Browse** mode (category tabs + card grid) and a full-screen **Render**
mode with back navigation. With arguments it acts as a CLI:

```sh
cargo run -p tess-tui -- list
cargo run -p tess-tui -- export --tiling truncated-hexagonal \
    --width 1200 --height 900 --scale 2.0 --rotation 0 \
    --mode proto --out tiling.svg
```

`export` options:

| flag | default | meaning |
|---|---|---|
| `--tiling ID` | — (required) | tiling id, see `list` |
| `--width` / `--height` | 1200 / 800 | SVG canvas size in px |
| `--scale` | the tiling's declared scale (1.0 for most tilings) | uniform scale; explicit `--scale` wins (1.0 → ~20 world units tall) |
| `--rotation` | 0 | rotation in degrees |
| `--center X,Y` | origin | center the view at world (X, Y) — finite values |
| `--param ID=VALUE` | — (optional) | override a tiling parameter for this run (see `list` for the parameter's id/type); repeatable |
| `--mode` | pattern | `single` \| `pattern` \| `proto` \| `gray` \| `pattern-gray` \| `shape` \| `role` \| `metatile` \| `orientation` \| `level` |
| `--strapwork DEG` | off | overlay the Hankin PIC strapwork walls at contact angle DEG (measured from each tile edge) on any tiling — layer mode of the polygons-in-contact engine (`tess-core::strapwork`) |
| `--deform NAME PARAMS…` | off | Islamic parquet deformation (diss. §3.4.1): the contact angle varies as a function of point; NAME is `linear` (`linear a b x0 x1` — ramp along x: a at x0 → b at x1), `radial` (`radial a b cx cy r0 r1` — ramp along distance from centre (cx, cy): a at r0 → b at r1) or `sine` (`sine mid amp k` — oscillation along x: mid + amp·sin(k·x)); mutually exclusive with `--strapwork` |
| `--decoration STYLE` | off | over the strapwork walls (`--strapwork` or `--deform`, one required), render a Kaplan §3.7 ornamentation style — `fill` \| `outline` \| `interlace` \| `outline-interlace`; `--mode` is ignored under `--decoration`. See [rendering.md](rendering.md) |
| `--band-w W` | 0.1 | band width in world units for the decoration layer (requires `--decoration`) |
| `--medallion-weave` | off | medallion third layer (requires `--strapwork DEG`, mutually exclusive with `--deform`): fuse the tiling's own medallion record layer into the strapwork wall map — the hull-face seeds stop firing generic contact rays and the record's chords take over at the same contact midpoints; tilings without a medallion layer render byte-identically. See [medallion-third-layer-design.md](medallion-third-layer-design.md) |
| `--motif NAME` | off | §22e motif record layer (requires `--strapwork DEG`, mutually exclusive with `--medallion-weave`, refused with `--deform` and `--dual-overlay`): fuse the motif's monster medallion figure at every tiling vertex into the strapwork wall map — `22e` places the `{16/8}` hexadecagon monster on `ammann-beenker` and the `{10/2}` decagon monster on `penrose`, inflated so the medallions meet at the tile-edge midpoints; unknown motif names and non-carrier tilings are named refusals |
| `--highlight-supertiles` | off | outline supertile-member tiles (substitution level > 0) at 2× the tile-border stroke, so the substitution hierarchy reads as heavier grout — a byte-for-byte no-op on flat tilings (all level 0). Mutually exclusive with `--decoration` (the decoration replaces the tiling render) |
| `--mirror` | off | mirror-image (enantiomorph) export: every emitted tile vertex and strapwork wall endpoint is reflected about the frame center (x → 2·cx − x, y unchanged) before path stringification — the pattern mirrored, never an SVG transform. Composes with `--strapwork` / `--deform` / `--medallion-weave` / `--highlight-supertiles`. Mutually exclusive with `--decoration` (the decoration replaces the tiling render) |
| `--dual-overlay` | off | render the card and its centroid dual as two named groups in one frame — `<g id="primal">` (the primal tiles) followed by `<g id="dual">` (the centroid-dual tiles), sharing the scaffolding (paper, y-flip, background). The strapwork walls are emitted once, after both groups. Available for the 31 Laves/dual-and-regular LatticeTiling cards; the 5 isohedral cards, non-lattice cards and degenerate duals are named refusals. Composes with `--strapwork` / `--deform` / `--mirror` / `--highlight-supertiles`. Mutually exclusive with `--decoration` (the decoration replaces the tiling render); rejected with `--medallion-weave` (the weave fuses the medallion layer into the wall map, which the two-group overlay does not support). Self-dual `square` is a render-level note, not an error |
| `--symmetry-overlay` | off | overlay the certified wallpaper-group symmetry elements of a periodic tiling — the unit cell (gold outline), rotation centers (rose circles with `fold` spokes), mirror lines (solid green) and glide axes (dashed cyan) — as a `<g data-symmetry-overlay="true">` block after the strapwork walls. A byte-for-byte no-op on aperiodic (non-periodic) tilings; for the p1 group only the unit cell is emitted (no point-group elements). Mutually exclusive with `--decoration` (the decoration replaces the tiling render) |
| `--vertex-orbits` | off | dot the two vertex-figure-class orbits of the emitted tiles (the semicolon pair a 2-uniform card declares in its vertex configuration): every tile vertex whose incident tiles close a full turn is grouped by its cyclic vertex figure (e.g. `3.3.3.3.3.3` vs `3.3.6.6`) and painted as small filled circles — orbit 0 pink `rgb(236,72,153)`, orbit 1 lime `rgb(163,230,53)` — inside a `<g data-vertex-orbits="true">` block after the symmetry overlay (or the walls). Pure geometry: derived from the emitted tiles alone, so it works on aperiodic cards too. Byte-for-byte no-op when off (and when no complete vertex exists). Composes with `--strapwork` / `--deform` / `--mirror` / `--highlight-supertiles` / `--symmetry-overlay`. Mutually exclusive with `--decoration` (the decoration replaces the tiling render) and `--dual-overlay` (the dual frame's vertices are the primal's face centers) |
| `--highlight-reflected` | off | fill reflected tiles in a lightened accent of their own mode color (`paint::reflected_accent`: blended ≈55% toward white, so every class of the ramp stays distinguishable inside the highlighted subset). The hat card emits one reflected hat per cluster (the H1 role), so its view highlights genuinely flipped hats; turtle and spectre bake their substitution's reflection into the level placements, so the flag first moves pixels combined with `--mirror` (which flips tile chirality, making the lightened H1 hats the mirrored ones). Byte-for-byte no-op when off. Composes with `--strapwork` / `--deform` / `--mirror` / `--dual-overlay` / `--symmetry-overlay`. Mutually exclusive with `--decoration` (the decoration replaces the tiling render) |
| `--large-tile-outline` | off | outline the large tile of an aperiodic family card — hat and spectre are monohedral (every tile outlined), the turtle card outlines its turtle cells (not the Mystic Sigma's companion hat) — in vivid orange `rgb(255,122,26)` at 2× the tile-border stroke weight, inside a `<g data-large-tile-outline="true">` group appended after the border group(s) and before the wall pass. A byte-for-byte no-op on cards without an aperiodic family tag. Mutually exclusive with `--decoration` (the decoration replaces the tiling render) |
| `--out`, `-o` | tessellix.svg | output SVG file |

`explore` options:

| flag | default | meaning |
|---|---|---|
| `--tiling ID` | — (required) | tiling id, see `list` (any registered substrate — the δ axis is the meaningful axis for lattice substrates) |
| `--out PATH` | stdout | write the leaderboard JSON to PATH instead of printing it |
| `--csv PATH` | off | write a flat CSV variant of the leaderboard (one row per candidate; null cells for the non-applicable axis) |
| `--gallery DIR` | off | write one SVG per top-`--top-k` ranked row per axis into DIR (refused rows skipped, with a printed count) |
| `--top-k N` | 10 | gallery rows per axis; must be ≥ 1 (0 and non-numeric values are named parse errors) |

**explore** — `tess-tui explore --tiling ID` (M170) runs a deterministic sweep over one substrate — the `pic-explorer` — with no RNG, no time, and no filesystem access in the sweep itself (registry, region and scoring are all built in-memory), and writes nothing unless asked:

- the **δ axis** scores every candidate contact angle over the 17-point base grid (multiples of 5° from 5° to 85°) plus the exact validated envelope points for envelope-bearing substrates (e.g. the 90° square point, added by refinement);
- the **θ axis** (`islamic` feature, only the two classic hankin substrates `hankin-4-8-8-motifs` and `hankin-3-12-12`) scores per-substrate motif-angle offsets ∈ {−15°, …, +15°} — one offset applied to every class's classic motif angle (7 rows per substrate).

Every candidate is scored and recorded, including open-strand regimes — the M165 envelope is a closed-angle point set, not a requirement for rendering. A closed weave at a non-envelope δ (exits/corners/bad all zero) is recorded as a `closure_violation` finding, never silently dropped; refusals and incomplete partitions are named in `feasibility` (`refused: <reason>` / `incomplete_partition`), never hidden. The region is fixed at the M165 convention: 20×20 world units centered on the origin (±10). The score is deliberately not aesthetic — it separates closed weaves from open regimes, then ranks structural integrity, slivers and symmetry-invariance (weights stated in the module docs and in the JSON itself). Winners are candidates, not registry entries.

```sh
cargo run -p tess-tui -- explore --tiling square --top-k 3 --out leaderboard.json
```

Output is a `pic-explorer-leaderboard/v1` JSON document, byte-deterministic across runs (same bytes, two-run test): substrate, region, the sort contract (score ascending, ties by angle, refused sink with rank 0), both axes' score formulas, refused/violation counts, and one row per candidate — axis, substrate id, angle, rank, score, feasibility, and the feature-vector groups: δ rows carry `validity` (core vertices / exit / corner / bad / straight2 / joint), `wall_map` (degree histogram, strand count, face count, min face area), `geometry` (total wall length, strap density, crossings, min crossing angle, crossing density) and `symmetry` (translation / mirror / rotation invariance + fold note); θ rows carry `motifs`, `partition_complete`, `max_partition_error`, `n_cells`, `min_face_area`, `total_area`, `symmetry` and `feasibility_reason`. Lower score is better. The `--csv` variant flattens the same rows into one column-per-field row each; the `--gallery` SVGs are named `<substrate>-d<δ>-r<rank>.svg` / `<substrate>-t<offset>-r<rank>.svg` — δ rows via the decoration-first gallery (fill → interlace → outline-interlace → outline, band 0.1, plain-wall fallback), θ rows via cells-as-tiles. Field-by-field schema: the M170 entry in [CHANGELOG](history/CHANGELOG.md).

**Browse mode** — the tab bar lists the eleven display categories with card counts; below it a responsive card grid shows the active tab's
tilings as blocks with a half-block raster thumbnail, the tiling name (up to
two lines), a `group · lattice` line (wallpaper tilings), a short description
fragment, and `params: n` when the tiling has adjustable parameters. A
tiling can belong to more than one tab: its primary category (its family)
plus declared secondaries — the Wallpaper tab is one card per wallpaper
group (the group catalog; since M85 all 17 groups are represented), so it
opens with a catalog note reporting the completed catalog.
In the Wallpaper tab the cards are grouped under crystal-system section
headers (Oblique, Rectangular, Centered, Square, Hexagonal), ordered by
lattice kind then wallpaper group; all other tabs appear in registry order.
Keys: `Tab`/`Shift-Tab` or `1`–`9`/`0` switch tabs (per-tab cursor and
scroll position are remembered), `↑↓←→` or `hjkl` move the cursor (wraps),
`PgUp`/`PgDn` or the mouse wheel scroll the grid, `Enter`/`Space` open the
selected tiling in Render mode, `/` starts a search, `q` quits.

**Search** — `/` opens a prompt in the status bar; the query filters
cards by name, id, and description (case-insensitive) across all tabs.
The tab bar shows per-tab match counts and the active tab jumps to the
first tab with a match. `Enter` opens the highlighted match, `Esc`
cancels, `Tab`/`1`–`9`/`0` still switch tabs, `Backspace` deletes.

**Render mode** — the selected tiling fills the whole terminal (one
character cell = one pixel, no chrome but the status bar):
` name [id] · group · tiles n │ esc back · ←/→ prev/next · hjkl pan ·
+/- zoom · c color · r reset · p params `. `←`/`→` walk to the
previous/next tiling in the current tab's order (the Browse grid cursor
follows), `hjkl` pan, `+`/`-` or the mouse wheel zoom, `c` cycles the
color mode, `r` resets the view, `p` toggles a parameter overlay (when
the tiling has parameters) — while it is open, `←`/`→` step the parameter
under the cursor and `↑`/`↓` move the cursor. `Esc` or `b` returns to
Browse with the exact prior tab, cursor, and scroll position.

**Mouse** — in Browse: left-click a tab to switch, left-click a card to
select it and open Render, wheel to scroll the grid. In Render: wheel to
zoom. `Ctrl-C`/`Ctrl-Q` quit from any mode.

## Plane-filling tilings: taylor-socolar

`taylor-socolar` (M168) presents the paper's actual plane-filling CHT
closed form — `generate` serves the whole plane: every hexagon in the
query region is tiled and there is no patch boundary. M86 shipped the
paper's forced 529-hexagon patch (Fig. 18 of arXiv:1003.4279) as a
verbatim state table; M168 keeps the same rules and state table but
renders it as a plane-fill — the 529-site rhombus survives only as a
test-level reproduction anchor (closed form reproduces the patch
529/529, zero R1/R2/R3 violations on windows extending past it).

Verified at 1200×900 (proto mode; far-field runs byte-identical across identical invocations):

```sh
# center — the scale-1.0 frame (±13.3 x by ±10 y), 1781 tiles:
cargo run -p tess-tui -- export --tiling taylor-socolar \
    --width 1200 --height 900 --scale 1.0 --mode proto --out taylor-socolar.svg

# far-field — the frame (±33.3 x by ±25 y) extends well beyond the
# scale-1.0 patch extents (±29.45 by ±17.5); 25,779 of 64,969 tiles
# sit outside them, 64,969 tiles:
cargo run -p tess-tui -- export --tiling taylor-socolar \
    --width 1200 --height 900 --scale 0.4 --rotation 0 --mode proto --out far-field.svg

# far-field at 35 degrees of rotation — 64,819 tiles:
cargo run -p tess-tui -- export --tiling taylor-socolar \
    --width 1200 --height 900 --scale 0.4 --rotation 35 --mode proto --out far-field-rot.svg

# far-field centered at world (60, 0): the frame is translated exactly
# by (60, 0) — world x ∈ [26.67, 93.33] this run — 64,905 tiles:
cargo run -p tess-tui -- export --tiling taylor-socolar \
    --width 1200 --height 900 --scale 0.4 --center 60,0 --mode proto --out far-field-centered.svg

# CAP at the same world (60, 0) — the same frame, 1,016 tiles:
cargo run -p tess-tui -- export --tiling cap \
    --width 1200 --height 900 --scale 0.4 --center 60,0 --mode proto --out cap-far-field-centered.svg
```

`--center X,Y` centers the frame at world (X, Y) — the view is translated
exactly by (X, Y) while the frame size and scale are unchanged (finite
values; a malformed or non-finite value is refused with a named parse
error, no SVG written). The examples above are origin-centered (the
default); the last two put the same wide frame (scale 0.4 → world frame
±33.3 x by ±25 y) around world (60, 0) instead — a genuinely far-field
window, the patch no longer being near-centered in it. Verified:
taylor-socolar 64,905 tiles (vs 64,969 origin-centered) and CAP 1,016
tiles, all tokens finite, the background rect at x = 26.67 = 60 − 33.33.

**Finite patches beyond their domain.** `--center` translates the frame
exactly as for any tiling, but a finite patch does not intersect a window
far from it — `--center 500,500` on `wis-12-fold`, `golden-bee` or
`girih-decagon-patch` writes a rect-only SVG (a single background rect, zero
shapes, all tokens finite, exit 0, byte-identical across runs). At the rim
the frame catches a partial subset of the patch — `--center -14,0` → 15 of 26
tiles for wis-12-fold, `15,0` → 13 of 32 for golden-bee, and the nominal
`12,0` → 198 of 611 for girih-decagon-patch — still well-formed and
all-finite, never an error. Note the per-tiling edge: girih-decagon-patch's
patch spans ~±29.3 × ±30.7 world units, larger than the default 30×20 frame, so
its origin-centered rendering already shows 245 of 611 tiles; `--scale 0.05`
renders the full patch (26 / 32 / 611 tiles, no `MAX_TILES` trip).

Every view is subject to the tile budget: a view whose window would
exceed `MAX_TILES` (250,000 tiles, e.g. scale 0.15 at this canvas) is
refused — stderr `error: too many tiles for this view — zoom in`, exit 1,
no SVG written.

## GUI

`cargo run -p tess-gui` opens an egui window (1024×640, dark theme).
Like the TUI, it is a **single-pane, two-mode browser** over the shared
`tess-core::browse` model — no side panel in either mode:

- **Browse mode** — the whole window is the card grid. A tab band
  (the eleven category tabs with card counts + a search field, one
  wrapping row) sits over a responsive card grid: the column count is
  computed from the window width (target card ~240 px, fully dynamic
  — no upper bound) and the cards stretch to fill. Card size is
  zoomable three ways: ctrl+wheel / trackpad pinch (continuous), the
  tab-band `−`/`+` buttons and the `+`/`−` keys (both step exactly one
  column per press, so every press visibly reflows the grid; the
  column-count label between the buttons resets the default size).
  The Wallpaper tab
  (one card per wallpaper group — the group catalog; since M85 all 17
  groups are represented) opens with a catalog note reporting the
  completed catalog. Cards show crystal-system section headers
  (Wallpaper tab), a lazily generated preview (rasterized at default parameters
  by the shared `tess-core::raster`, same palette/proto-fallback
  semantics as the TUI thumbnails, cached per (id, params, mode, size),
  generated for visible cards only), the bold name (up to two lines,
  broken at a word boundary), the `group · lattice` line, and a
  truncated description. Search filters name/id/description
  case-insensitively; the tab row shows per-tab match counts and the
  active tab jumps to the first tab with a match. Clicking a card
  opens Render mode; per-tab selection is remembered.
- **Render mode** — the whole window is the canvas (drag to pan,
  scroll to zoom anchored at the cursor). A top strip offers `← back`,
  `◀`/`▶` prev/next (walking the current tab's order, wrapping), a
  params toggle, a zoom cluster (`−` / live zoom-percentage / `+`,
  where clicking the percentage resets 100%) and `⟲ reset`; the
  `+`/`−` keys zoom by the same step around the canvas center (so
  zoom works without wheel/pinch gestures, which some desktops strip).
  **Escape** and `← back` both return to
  Browse exactly as left (same tab + selection). A collapsible bottom
  panel (expanded by default on entry) holds the parameter editors,
  the color-mode selector (single / pattern / proto / gray / pattern-gray / shape / role / metatile / orientation / level), and the
  symmetry readout (wallpaper group, lattice kind, basis vectors,
  notes) — the readout is cached per tiling + parameter values and
  refreshed through a tiny fixed probe region, so panel rendering
  never triggers view-sized tile generation. The tiling is
  regenerated for the current viewport on every frame; if the view
  would exceed the tile budget (250 000 tiles), a "too many tiles for
  this view — zoom in" message is shown instead.

---

*[Documentation index](INDEX.md)*
