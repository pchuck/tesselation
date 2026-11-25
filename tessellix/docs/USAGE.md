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
| `--scale` | 1.0 | uniform scale (1.0 → ~20 world units tall) |
| `--rotation` | 0 | rotation in degrees |
| `--center X,Y` | origin | center the view at world (X, Y) — finite values |
| `--mode` | pattern | `single` \| `pattern` \| `proto` \| `gray` |
| `--strapwork DEG` | off | overlay the Hankin PIC strapwork walls at contact angle DEG (measured from each tile edge) on any tiling — layer mode of the polygons-in-contact engine (`tess-core::strapwork`) |
| `--decoration STYLE` | off | over the `--strapwork` walls (required), render a Kaplan §3.7 ornamentation style — `fill` \| `outline` \| `interlace` \| `outline-interlace`; `--mode` is ignored under `--decoration`. See [rendering.md](rendering.md) |
| `--band-w W` | 0.1 | band width in world units for the decoration layer (requires `--decoration`) |
| `--out`, `-o` | tessellix.svg | output SVG file |

**Browse mode** — the tab bar lists the ten display categories with card counts; below it a responsive card grid shows the active tab's
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
# by (60, 0) — world x ∈ [26.2, 93.8] this run — 64,905 tiles:
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
patch spans ~±17 × ±12 world units, larger than the default 30×20 frame, so
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
  (the ten category tabs with card counts + a search field, one
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
  semantics as the TUI thumbnails, cached per (id, params, size),
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
  the color-mode selector (single / pattern / proto / gray), and the
  symmetry readout (wallpaper group, lattice kind, basis vectors,
  notes) — the readout is cached per tiling + parameter values and
  refreshed through a tiny fixed probe region, so panel rendering
  never triggers view-sized tile generation. The tiling is
  regenerated for the current viewport on every frame; if the view
  would exceed the tile budget (250 000 tiles), a "too many tiles for
  this view — zoom in" message is shown instead.

---

*[Documentation index](INDEX.md)*
