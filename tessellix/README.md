# tessellix

A Rust tiling generator with two front ends: an interactive terminal UI
(TUI) and a desktop GUI. It generates 2D tilings of the plane on demand —
uniform/Archimedean, wallpaper, Penrose, Pythagorean, and monohedral
nonperiodic families — and renders them live, with pan/zoom, tunable
parameters, three color modes, and SVG export.

- **License:** MIT
- **Rust:** edition 2021, MSRV 1.85
- **Dependencies:** `tess-core` is dependency-free; the TUI uses
  `ratatui` + `crossterm`, the GUI uses `eframe`/`egui`. No geometry or
  rendering libraries elsewhere — all geometry is implemented in-house.

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

`export` options:

| flag | default | meaning |
|---|---|---|
| `--tiling ID` | — (required) | tiling id, see `list` |
| `--width` / `--height` | 1200 / 800 | SVG canvas size in px |
| `--scale` | 1.0 | uniform scale (1.0 → ~20 world units tall) |
| `--rotation` | 0 | rotation in degrees |
| `--mode` | pattern | `single` \| `pattern` \| `proto` |
| `--strapwork DEG` | off | overlay the Hankin PIC strapwork walls at contact angle DEG (measured from each tile edge) on any tiling — layer mode of the polygons-in-contact engine (`tess-core::strapwork`) |
| `--out`, `-o` | tessellix.svg | output SVG file |

**Browse mode** — the tab bar lists the ten display categories with card
counts (`Uniform (11)   Two-Uniform (11)   Laves (15)   Wallpaper (17)   Pentagon
Types (12)   Islamic (14)   Arrangements (5)   Quasiperiodic (8)   Aperiodic (6)
Nonperiodic (4)`); below it a responsive card grid shows the active tab's
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

### GUI

`cargo run -p tess-gui` opens an egui window (1024×640, dark theme).
Like the TUI, it is a **single-pane, two-mode browser** over the shared
`tess-core::browse` model — no side panel in either mode:

- **Browse mode** — the whole window is the card grid. A tab band
  (the ten category tabs with card counts + a search field, one
  wrapping row) sits over a responsive card grid: the column count is
  computed from the window width (target card ~240 px, fully dynamic
  — no upper bound) and the cards stretch to fill. The Wallpaper tab
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
  params toggle, and `⟲ reset`; **Escape** and `← back` both return to
  Browse exactly as left (same tab + selection). A collapsible bottom
  panel (expanded by default on entry) holds the parameter editors,
  the color-mode selector (single / pattern / proto), and the
  symmetry readout (wallpaper group, lattice kind, basis vectors,
  notes) — the readout is cached per tiling + parameter values and
  refreshed through a tiny fixed probe region, so panel rendering
  never triggers view-sized tile generation. The tiling is
  regenerated for the current viewport on every frame; if the view
  would exceed the tile budget (250 000 tiles), a "too many tiles for
  this view — zoom in" message is shown instead.

## Supported tilings

95 built-in tilings (107 tab memberships), all discoverable in both UIs
via the registry (`tess-tui -- list`). Every tiling declares a primary
display category — its family — plus optional secondary tabs (e.g. the
self-dual regular tilings also appear in Laves, and seventeen periodic
tilings anchor the per-group Wallpaper catalog — all 17 groups). The
ten tabs:
**Uniform** (11), **Two-Uniform** (11), **Laves / Catalan duals** (15,
12 primary + 3 self-dual secondaries), **Wallpaper** (17: one card per
wallpaper group — the full 17-group catalog, completed by M85),
**Pentagon Types** (12), **Islamic** (18), **Arrangements** (5),
**Quasiperiodic** (8: Penrose family, Ammann–Beenker, Stampfli, shield,
girih Darb-i Imam), **Aperiodic** (6: einstein family, pinwheel, Taylor-Socolar,
Jeandel-Rao 11-tile Wang tiles), and
**Nonperiodic** (4: limit-periodic rep-tiles, Hirschhorn). Categories
are declared on the tiling (never derived from its symmetry), so the
grouping is honest about what the mathematics distinguishes; see
`tess-core/src/category.rs`.

| id | name | type | wallpaper group | lattice |
|---|---|---|---|---|
| `square` | Square grid (4.4.4.4) | uniform | p4m | square |
| `hexagon` | Hexagonal (6.6.6) | uniform | p6m | hexagonal |
| `triangle` | Triangular (3.3.3.3.3.3) | uniform | p6m | hexagonal |
| `truncated-square` | Truncated square (4.8.8) | Archimedean | p4m | square |
| `trihexagonal` | Trihexagonal (3.6.3.6) | Archimedean | p6m | hexagonal |
| `snub-trihexagonal` | Snub trihexagonal (3.3.3.3.6) | Archimedean, chiral | p6 | hexagonal |
| `elongated-triangular` | Elongated triangular (3.3.3.4.4) | Archimedean | cmm | centered |
| `rhombitrihexagonal` | Rhombitrihexagonal (3.4.6.4) | Archimedean | p6m | hexagonal |
| `truncated-trihexagonal` | Truncated trihexagonal (4.6.12) | Archimedean | p6m | hexagonal |
| `truncated-hexagonal` | Truncated hexagonal (3.12.12) | Archimedean | p6m | hexagonal |
| `snub-square` | Snub square (3.3.4.3.4) | Archimedean | p4g | square |
| `star-cross` | Star and cross ({8/3}) | Islamic star-and-cross family (one-parameter) | p4m | square |
| `hexagram` | Hexagram and hexagon ({6/2}) | hexagram star-and-hexagon family (one-parameter) | p6m | hexagonal |
| `cairo-pentagonal` | Cairo Pentagonal (V3.3.4.3.4) | pentagonal (Catalan dual of the snub square); includes the integer-coordinate collinear variant (angles 143.13°/108.43°/108.43°/90°/90°, edges 4×√10+4, square 72-area cell) | p4g | square |
| `tetrakis-square` | Tetrakis square (V4.8.8) | Laves (Catalan dual of the truncated square) | p4m | square |
| `rhombille` | Rhombille (V3.6.3.6) | Laves (Catalan dual of the trihexagonal); 3-colored cube illusion | p6m | hexagonal |
| `triakis-triangular` | Triakis triangular (V3.12.12) | Laves (Catalan dual of the truncated hexagonal) | p6m | hexagonal |
| `deltoidal-trihexagonal` | Deltoidal trihexagonal (V3.4.6.4) | Laves (Catalan dual of the rhombitrihexagonal) | p6m | hexagonal |
| `kisrhombille` | Kisrhombille (V4.6.12) | Laves (Catalan dual of the truncated trihexagonal) | p6m | hexagonal |
| `prismatic-pentagonal` | Prismatic pentagonal (V3.3.3.4.4) | Laves (Catalan dual of the elongated triangular) | cmm | centered |
| `floret-pentagonal` | Floret pentagonal (V3.3.3.3.6) | Laves (Catalan dual of the snub trihexagonal), chiral | p6 | hexagonal |
| `girih-shesh-band` | Girih shesh-band (6.6.6) | periodic girih strapwork band | p2 | centered |
| `girih-star-cross` | Girih star-and-cross (periodic, 10-fold) | measured classical strapwork panel: 10-pointed stars, bobbins, chord straps; 545-tile cell data table | cmm | centered |
| `girih-w85` | Girih 12-pointed star interlace (W85) | Wichmann panel data18/W85 (Alhambra, Mexuar patio): exact Q(√3) strap grid L = 283.92, strap 9; 42 tiles per hexagonal cell; drawn interlace chiral | p6 | hexagonal |
| `pic-4-8-8` | Hankin PIC fourfold khatam | Hankin polygons-in-contact strapwork on the 4.8.8 substrate (contact angle δ = 45°): midpoint-chord walls — {8/2} octagram in each octagon — partitioning the plane into octagon + small square + 4 house pentagons + 8 star tips per cell, exact Q(√2) | p4m | square |
| `pic-3-6-3-6` | Hankin PIC sixfold khatam | Hankin polygons-in-contact strapwork on the trihexagonal (3.6.3.6) substrate (contact angle δ = 60°): midpoint-chord walls — {6/2} hexagram in each hexagon, midsegments in each triangle — partitioning the plane into 4 hexagram-core hexagons + 6 hexagram tips + 2 midsegment triangles per cell (a half-scale 3.6.3.6), exact Q(√3) | p6m | hexagonal |
| `pic-3-12-12` | Hankin PIC twelvefold khatam | Hankin polygons-in-contact strapwork on the truncated-hexagonal (3.12.12) substrate (contact angle δ = 60° — the only angle closing both the dodecagon and the triangle): midpoint-chord walls — {12/4} twelvefold rosette in each dodecagon, midsegments in each triangle — partitioning the plane into rosette-core dodecagon + 6 vertex pentagons + 12 band quads + 2 medial triangles + 12 inner quads + 12 star notches per cell, exact Q(√3) | p6m | hexagonal |
| `hankin-4-8-8-rosette` | Hankin inference fourfold rosette | Hankin motif inference (Kaplan §3.4) on the 4.8.8 substrate (θK = 67.5°, δ = 0): greedy min-length contact-ray pairing — 8-petal rosette + core octagon in each octagon (9 faces), 4-pointed star + core diamond in each diamond (5 faces) per cell ⟨(1+√2,0),(0,1+√2)⟩ (det 3+2√2), exact Q(√2); the inference counterpart of `pic-4-8-8`'s reflection-variant chord walls (M94) | p4m | square |
| `hankin-4-6-12` | Hankin inference twelvefold | Hankin motif inference on the 4.6.12 substrate (θK = 60°, δ = 0): 12-petal dodecagon rosette (13 faces), 4-pointed stars in the three squares (5 each), 6-petal hexagon rosettes (7 each) — 42 faces per cell ⟨s(1,0), s(1/2,√3/2)⟩ with s = 3+√3 (det 9+6√3), exact Q(√3) (M94) | p6m | hexagonal |
| `hankin-hex-two-point` | Hankin two-point weave | Hankin two-point motif inference on the unit hexagon (θK = 45°, split δ = 0.5 — rays leave edge split points and cross): 12 interlaced bracket-and-diamond faces around a central dodecagon per cell ⟨(3/2,√3/2),(0,√3)⟩ (det 3√3/2); Kaplan Fig 3.6 row 1 col c (M94) | p6m | hexagonal |
| `hankin-3-12-12` | Hankin twelvefold khatam (3.12.12) | Hankin greedy-inference motif on the truncated hexagonal (3.12.12) substrate (θK = 45°, δ = 0): dodecagon twelve-pointed star — 24-gon core + 12 petals (13 faces) — triangle 4-pointed stars with hexagon tips (4 each), 21 faces per cell ⟨(3/2+√3, 1+√3/2), (0, 2+√3)⟩ (det 6+7√3/2), exact Q(√3); θK = 45° chosen over 30°/60° (flat / spiky limits) for the shining star (90° tips, 150° notches); p6m verified on a ±4-lattice window, D6-exact (M99) | p6m | hexagonal |
| `hankin-square-two-point` | Hankin two-point square weave (4.4.4.4) | Hankin two-point motif inference on the unit square (4.4.4.4) substrate (θK = 45°, split δ = 0.5 — rays leave edge split points and cross): a decagonal star per cell — four corner triangles, two pentagonal petals, one 10-gon core — 7 faces per unit cell ⟨(1,0),(0,1)⟩ (det 1); cmm (2*22) — diagonal mirror families x∓y=k with 2-folds on-mirror at vertices/cell-centers and off-mirror at edge midpoints, no 4-folds (M99) | cmm | centered |
| `hankin-4-8-8-rosette-transform` | Hankin rosette-transform khatam | Kaplan rosette transform (§3.8) of the 4.8.8 substrate at θK = 45° — each octagon maps to a concentric inner octagon + 8 pentagons (r′ radius, spokes = half inner side), each square to 4 half-squares (bisectors meet in a centre cluster) — then Hankin motif inference (θE = 45°) on the transformed table: 16 triangles, 36 quads, 32 pentagons, 8 hexagons and one 16-gon per cell ⟨(1+√2,0),(0,1+√2)⟩ (det 3+2√2), exact Q(√2); the cell table is p4m-exact to 6.5e-16 (M99) | p4m | square |
| `hankin-3-6-3-6` | Hankin sixfold hexagram (3.6.3.6) | Hankin greedy-inference motif on the trihexagonal (3.6.3.6) substrate (θK = 60°, δ = 0): each hexagon becomes a regular hexagram — a 12-gon core with 60° tips and 240° notches (vertices alternating on circles of radius 1/2 and √3/2) plus six 60°-lozenge rhombi (7 faces); the two equilateral triangle tiles pair with no contact rays (0 pairs) and stay single faces — 9 faces per cell ⟨(2,0),(1,√3)⟩ (det 2√3), exact Q(√3); θK = 60° chosen over 30°/45° (no star: core tips 120°/90°) and 67.5°/90° (sharper variant / star collapse) for the classical {6/2} hexagram; p6m verified on a ±5-lattice window, D6-exact (M100) | p6m | hexagonal |
| `khatam-girih` | Khatam girih decomposition (8-point) | classical kissing {8/3} star-and-cross cut into 8 rhombi + square + 4 pentagons per cell (exact Q(√2) kitab) | p4m | square |
| `najm-63-2e-0-3e-ac` | Najm template ([6,3]; 2e, 0, 3e; AC) | Najm-template substrate (Kaplan diss. §3.6.1 Fig. 3.21) from the najm generator: regular dodecagons + enneagons sharing equal chords, case-3 inflation x = 2/(cot 15° + cot 20°), + concave equilateral bowties (70°/220°) at the 2-fold points; 1 doz + 2 enne + 3 bowties per cell, |det| = 3√3/2 | p6m | hexagonal |
| `najm-44-3e-0-0-a` | Najm template ([4,4]; 3e, 0, 0; A) | Najm-template substrate (Kaplan diss. §3.6.1 Fig. 3.22 second row): dodecagons r = √6−√2 on 2Z² sharing chords across the odd grid lines + concave equilateral 4-pointed stars (side 4−2√3, tips 60°, notches 210°); exact Q(√2,√3), areas 24−12√3 and 12√3−20 per cell | p4m | square |
| `basketweave` | Basketweave (2:1 dominoes) | paver bond: 2×2 blocks of two parallel 2:1 dominoes, checkerboard orientation; long sides split into unit edges | p4g | square |
| `herringbone` | Herringbone (2:1 dominoes) | paver bond: chiral L-pairs of 2:1 dominoes, single-direction; long sides split into unit edges | pgg | rectangular |
| `running-bond` | Running bond (2:1 dominoes) | paver bond: stretcher rows of 2:1 dominoes, each row offset by half a brick; long sides split into unit edges | cmm | centered |
| `stepped-hexominoes` | Stepped hexominoes | paver tiling: asymmetric 2-step staircase hexomino, one tile per cell on the centered rectangular lattice ⟨(3,1),(0,2)⟩ (primitive cell; centered cell 6×2 with center (−3,1) in the lattice), edge-to-edge with 180° at the four collinear corners; the catalog's first p1 realizer — the centered-rectangular normalizer {I, R180, Mx, My} is broken by the asymmetric staircase | p1 | centered |
| `trapezoid-courses` | Trapezoid courses | paver tiling: two courses of trapezoids per 2×1 cell on the rectangular lattice ⟨(2,0),(0,1)⟩ (det 2): four trapezoids, two incongruent shapes (areas 3/5 and 2/5) each with its mirror in x = 1, edge-to-edge (slant slope 1/5); the catalog's first pm realizer — vertical mirror lines x = integer, no horizontal mirrors or half-turns (the incongruent prototile classes kill pmm) | pm | rectangular |
| `glide-hexagon-strips` | Glide hexagon strips | paver tiling: two interlocking hexagon strips per 1×2 cell on the rectangular lattice ⟨(1,0),(0,2)⟩ (det 2): fat (area 29/40) and waist (area 11/40) hexagons, strip 1 the glide image of strip 0 under g(x,y) = (x+1/2, −y) with g² = (1,0); edge-to-edge, five edge directions, no verticals; the catalog's first pg realizer — one horizontal glide family (glide 1/2; axes y ∈ ℤ in two orbits, y ∈ 2ℤ and y = 1+2ℤ), no mirrors, no rotations | pg | rectangular |
| `shifted-trapezoid-courses` | Shifted trapezoid courses | paver tiling: the trapezoid-course row stacked in rows of height 1 per 2×2 cell on the rectangular lattice ⟨(2,0),(0,2)⟩ (det 4), alternate rows reflected in y = 1 and shifted by (1,0): eight trapezoids, two incongruent shapes (areas 3/5 and 2/5) × mirror × row-flip; edge-to-edge (slant slope 1/5); the catalog's first pmg realizer — vertical mirror lines x ∈ ℤ, one horizontal glide family (glide 1, axes y ∈ ℤ), half-turns at (m+1/2, n) for all m, n ∈ ℤ; the (1,0) row shift kills the horizontal mirrors that would make it pmm | pmg | rectangular |
| `wavy-courses` | Wavy courses | paver tiling: zigzag courses of height 4/3 (waves of slope ±1/3, peaks 1/3 above the valleys) per 2×2 cell on the centered-rectangular lattice ⟨(1,1),(2,0)⟩ (det 2; the x+y-even lattice, also square-metric): each column-course quadrilateral split by a diagonal seam alternating slope +1/−1 with column–course parity — four triangles, two incongruent shapes (areas 1/3 and 2/3, squared edges {4,10,18} vs {10,16,18} ×9); edge-to-edge; the catalog's first cm realizer — mirror lines x = integer (all of them), glides (2j+1−x, y+1) on the half-integer axes with glide (0,1), no half-turns, no horizontal mirrors; the lattice's square metric is a degenerate centered-rectangular realization | cm | centered |
| `chiral-cut-honeycomb` | Chiral-cut honeycomb | paver tiling: a side-2 hexagon (area 6√3 = det Λ) cut by a chiral three-armed Y from the alternating vertices to the centre, each arm bending once (Wk = mid(Vk,O) + rotated delta, delta = (−1/8,−1/10) defined once and rotating with the arm), per hexagonal cell ⟨(2√3,0),(√3,3)⟩ (det 6√3): three tiles, one chiral congruence class (all seed 0); edge-to-edge (450 interior edges, 0 T-junctions); the catalog's first p3 realizer — 120°/240° rotations about three 3-fold centre orbits (hex centres, even corners, odd corners — the "333" fingerprint) plus translations only, no 60°/180°/300° rotations, no mirrors (M83, HX-STUDY handoff) | p3 | hexagonal |
| `split-triangle-grid` | Split-triangle grid | paver tiling: side-2 triangular grid in which every small equilateral triangle is split by its orbit triangle (a generic Q(√3) point near a vertex, rotated 120° about the triangle's centroid) into an inner equilateral triangle + 3 corner quads, per hexagonal cell ⟨(2,0),(1,√3)⟩ (det 2√3): 8 tiles, two congruence classes (inner triangles achiral, corner quads chiral); edge-to-edge (750 interior edges, 0 T-junctions); the catalog's first p31m realizer — mirrors exactly on the grid lines (0°/60°/120° families), rot120 centres in three orbits (grid vertices, up/down-triangle centroids) with the on-mirror incidence (grid-vertex orbit on mirrors, both centroid orbits off) that separates p31m from p3m1, no 60°/180°/300° rotations, no glides (M84, HX-STUDY handoff) | p31m | hexagonal |
| `alcove-cut-grid` | Alcove-cut grid | paver tiling: side-2 triangular grid on the index-3 sublattice in which every grid triangle is cut from its alcove point — the orbit of one generic Q(√3) junction under the edge-line mirrors of the up-triangle — to its three vertices, per hexagonal cell ⟨(3,−√3),(3,√3)⟩ (det 6√3): 18 tiles, three congruence classes (each mirror-closed, achiral as a set); edge-to-edge (1350 interior edges, 0 T-junctions); the catalog's p3m1 realizer — the *333 fingerprint (all three rot120 centre orbits on mirrors, the contrast to M84's p31m one-orbit-on-mirrors), mirrors in the 0°/60°/120° families, no 60°/180°/300° rotations, no glides; closes the last gap of the 17-group wallpaper catalog (M85, HX-STUDY handoff) | p3m1 | hexagonal |
| `pythagorean` | Pythagorean (two squares) | periodic two-square family (one-parameter ratio); square on the hypotenuse per cell | p4 | square |
| `windmill` | Periodic pinwheel arrangement (p4) | Conway's periodic "spin" of the pinwheel motif: 1–2–√5 triangles around squares; homochiral, non-edge-to-edge | p4 | square |
| `kuniform-p6m` | 2-uniform (3⁶; 3².6²) | 2-uniform k-uniform: hexagonal tiling with one of every three hexagons dissected into six-triangle fans (Wikipedia `2-uniform n10`) | p6m | hexagonal |
| `kuniform-pmm` | 2-uniform (3.6.3.6; 3².6²) | 2-uniform k-uniform: hexagonal tiling with rows of up- and down-pointing triangles filling the gaps between rows of hexagons (Wikipedia `2-uniform n11`) | pmm | rectangular |
| `kuniform-n18` | 2-uniform (3⁶; 3².4.3.4) | 2-uniform k-uniform: squares + two triangle orbits around the 3⁶ vertices (Wikipedia `2-uniform n18`, t = 3, e = 3); 60° hexagonal basis, \|b1\| = \|b2\| = 1+√3, det 3+2√3 (left-handed); origin a 3⁶ vertex = 6-fold center; 11 tiles/cell (M87) | p6m | hexagonal |
| `kuniform-n9` | 2-uniform (3.4.6.4; 3².4.3.4) | 2-uniform k-uniform: hexagons ringed by squares with triangle fills (Wikipedia `2-uniform n9`, t = 4, e = 4); 60° hexagonal basis, \|b1\| = \|b2\| = 2+√3, det 6+7√3/2 (left-handed); origin a hexagon center = 6-fold center; 15 tiles/cell (M88) | p6m | hexagonal |
| `kuniform-n8` | 2-uniform (3.4.6.4; 3³.4²) | 2-uniform k-uniform: the second p6m 3.4.6.4 form — hexagons ringed by squares with triangle fills, one square–square edge per cell, 2-folds at square–square edge midpoints (cf. n9's triangle–triangle) (Wikipedia `2-uniform n8`, t = 4, e = 4); same basis class as n9 (det 6+7√3/2); origin a hexagon center; 15 tiles/cell (M89) | p6m | hexagonal |
| `kuniform-n5` | 2-uniform (3.4.6.4; 3.4².6) | 2-uniform k-uniform: hexagons ringed by squares with triangle fills, two hexagon orbits + two square orbits + one triangle orbit (Wikipedia `2-uniform n5`, t = 5, e = 5); 60° hexagonal basis, \|b1\| = \|b2\| = 3+√3, det 9+6√3 (right-handed); 18 tiles/cell; the 3.4².6 vertex orbit is chiral (M90) | p6m | hexagonal |
| `kuniform-n1` | 2-uniform (4.6.12; 3.4.6.4) | 2-uniform k-uniform: the 3-4-6-12 tiling — dodecagons alternating with hexagons, squares and triangles; first 12-gon in the module (Wikipedia `2-uniform n1`, t = 4, e = 4); 60° hexagonal basis, \|b1\| = \|b2\| = 2+2√3, det 12+8√3 (right-handed); 12 tiles/cell (M91) | p6m | hexagonal |
| `kuniform-n13` | 2-uniform (3⁶; 3².4.12) | 2-uniform k-uniform: dodecagons ringed by squares and two triangle orbits around 3⁶ vertices (Wikipedia `2-uniform n13`, t = 4, e = 4); 60° hexagonal basis, \|b1\| = \|b2\| = 3+√3, det 9+6√3 (right-handed); 16 tiles/cell; 6-fold center off-ring (M92) | p6m | hexagonal |
| `kuniform-n20` | 2-uniform (3⁶; 3⁴.6) | 2-uniform k-uniform: flower form — unit triangular lattice with hexagons dissected at the points of an index-13 sublattice, the two-uniform family's first p6 entry (Wikipedia `2-uniform n20`, t = 5; e = 6 exact vs. the wiki's parenthetical 7); 60° hexagonal basis, \|b1\| = \|b2\| = √13, det 13√3/2 (right-handed); 21 tiles/cell; A:B = 1:1 vertex orbits pinned by k = 4X+3Y mod 13 (M96) | p6 | hexagonal |
| `kuniform-n12` | 2-uniform (3².6²; 3⁴.6) | 2-uniform k-uniform: rows of edge-sharing hexagons with unit-triangle bands between the rows, the two-uniform family's first cmm entry (Wikipedia `2-uniform n12`, t = 2, e = 4); centered rectangular basis ⟨(√3, 0), (√3/2, 5/2)⟩ with the centering vector as basis, det 5√3/2 (right-handed); 5 tiles/cell (1 hexagon + 4 triangles, the 2-hexagon + 8-triangle centered cell is S ⊎ (S + c)); both vertex orbits 2 translation classes per cell (M97) | cmm | centered |
| `kuniform-n19` | 2-uniform (3⁶; 3⁴.6)₁ | 2-uniform k-uniform: the t = 3 flower form of the 3⁴.6 pair — unit triangular lattice with hexagons dissected at the points of the index-9 sublattice ⟨(3,0),(0,3)⟩ (Wikipedia `2-uniform n19`, t = 3, e = 3); 60° hexagonal basis, \|b1\| = \|b2\| = 3, det 9√3/2 (right-handed); 13 tiles/cell; A:B = 1:3 vertex orbits pinned by v mod 3; both mirror classes verified exactly, so the wiki's p6 group label under-states the symmetry — this is the achiral p6m form (n20 is the chiral p6 form) (M95) | p6m | hexagonal |
| `kuniform-dual-p6m` | Dual of 2-uniform (3⁶; 3².6²) | face-centroid dual of `kuniform-p6m` (p6m, det 9√3/2): 1 regular hexagon (side 1/√3, area √3/2) + 6 congruent quadrilaterals (sides 1/√3, 2√3/3, 2√3/3, √3 in cyclic order; angles 60°/60°/120°/120°; area 2√3/3) per cell, sum 9√3/2 = \|det\|, edge-to-edge, 6-fold center at the origin (M98) | p6m | hexagonal |
| `kuniform-dual-n18` | Dual of 2-uniform (3⁶; 3².4.3.4) | face-centroid dual of `kuniform-n18` (p6m, det 3+2√3): 1 regular hexagon (side 1/√3) + 6 congruent pentagons (one side 1/√3, four sides (3+√3)/6; angles 90°/90°/120°/120°/120°; area (2+√3)/4) per cell, sum 3+2√3 = \|det\|, edge-to-edge, 6-fold center at the origin (M98) | p6m | hexagonal |
| `kuniform-dual-n9` | Dual of 2-uniform (3.4.6.4; 3².4.3.4) | face-centroid dual of `kuniform-n9` (p6m, det 6+7√3/2): 6 congruent quadrilaterals (two sides (3+√3)/6, two sides (1+√3)/2; angles 60°/90°/90°/120°; area (3+2√3)/6) + 6 congruent pentagons (congruent to the `kuniform-dual-n18` pentagons; area (2+√3)/4) per cell, sum 6+7√3/2 = \|det\|, edge-to-edge, 6-fold center at the origin (M98) | p6m | hexagonal |
| `kuniform-dual-n8` | Dual of 2-uniform (3.4.6.4; 3³.4²) | face-centroid dual of `kuniform-n8` (p6m, det 6+7√3/2): 6 congruent quadrilaterals (same shape as the `kuniform-dual-n9` quadrilaterals) + 6 congruent pentagons (two sides 1/√3, two sides (3+√3)/6, one side 1; angles 90°/90°/120°/120°/120°; area (2+√3)/4 — equal area to the n18/n9 dual pentagons, different shape) per cell, sum 6+7√3/2 = \|det\|, edge-to-edge, 6-fold center at the origin (M98) | p6m | hexagonal |
| `pentagon-9` | Pentagon type 9 (Rice) | pentagonal (Rice 1977): A = C = 120°, cos B = 1/7, cos D = 23/98, cos E = −11/14, a = 6, b = c = d = e = 14; 2-isohedral, 8 tiles per rectangular cell, two perpendicular glide families squaring to the primitive translations | pgg | rectangular |
| `pentagon-10` | Pentagon type 10 (James) | pentagonal (James 1975): A = 90°, B + E = 180°, a = b = c + e, d² = c² + e² (reference instance B = 90°, c = 1: A = B = E = 90°, C = D = 135°, a = b = 2, c = e = 1, d = √2); 3-isohedral, 6 tiles per cell, non-edge-to-edge (each length-2 side split at its exact midpoint), mirrors y = x and y = −x, half-turn at the origin, genuine glide (axis x + y = 3/2), centered rectangular lattice | cmm | centered |
| `pentagon-11` | Pentagon type 11 (Rice) | pentagonal (Rice 1977): A = 90°, B = 156°, C = 48°, D = 114°, E = 132°, 2a + c = d = e (reference instance with c = 1); 2-isohedral, 8 tiles per rectangular cell, non-edge-to-edge (every d-side carries one c-side and two a-sides), two perpendicular glide families squaring to w1 and −w2 | pgg | rectangular |
| `pentagon-12` | Pentagon type 12 (Rice) | pentagonal (Rice 1977): A = 90°, B = 150°, C = 60°, D = E = 120°, 2a = d = c + e (reference instance with c = 1); 2-isohedral, 8 tiles per rectangular cell, non-edge-to-edge (every d-side is split at its exact midpoint into two unit halves), two perpendicular glide families squaring to w1 and −w2 | pgg | rectangular |
| `pentagon-2` | Pentagon type 2 (Reinhardt) | pentagonal (Reinhardt 1918 family 2): A = B = D = 90°, C = 120°, E = 150°, a = 1, b = d = 1 + √3/2, c = e = 1/2 (c = e, d = b, B + D = 180°), exact Q(√3); 1-isohedral, 4 tiles (two chiral pairs) per rectangular cell, edge-to-edge, two perpendicular glide families squaring to w1 and −w2 | pgg | rectangular |
| `pentagon-3` | Pentagon type 3 (Reinhardt) | pentagonal (Reinhardt 1918 family 3): A = B = C = D = 120°, E = 60°, a = b = 2, c = 1, d = 4, e = 3 (a = b, d = c + e), exact Q(√3); 1-isohedral, 2 tiles per oblique cell, non-edge-to-edge (every d-side is split at its exact midpoint between two length-2 sides); chiral prototile, one chirality used — the classical three-tile type-3 pinwheel admits no periodic cell for this member | p2 | oblique |
| `pentagon-6` | Pentagon type 6 (Kershner) | pentagonal (Kershner 1968): A = B = D = E = 120°, C = 60°, a = d = e = 1, b = c = 2 (A = 2C, C + E = 180°); the unique 30°-grid member, exact Q(√3); achiral prototile (congruent to the floret-pentagonal tile), 2-isohedral, 4 tiles per oblique cell, edge-to-edge (every side paired with exactly one full reverse partner, 10 side-pair orbits) | p2 | oblique |
| `pentagon-7` | Pentagon type 7 (Kershner) | pentagonal (Kershner 1968): b = c = d = e, B + 2E = 2C + D = 360° (published instance E = 106°: A ≈ 58.86°, B = 148°, C ≈ 132.86°, D ≈ 94.27°, a ≈ 1.0357942, b = c = d = e = 1/2, prototile computed at runtime); 2-isohedral, 8 tiles (four direct, four mirror) per rectangular cell, edge-to-edge, two perpendicular glide families squaring to w1 and −w2 | pgg | rectangular |
| `pentagon-8` | Pentagon type 8 (Kershner) | pentagonal (Kershner 1968): B = C = 120°, tan A = 3√3 (A ≈ 79.11°), cos D = 1/7 (D ≈ 81.79°), tan E = −√3/2 (E ≈ 139.11°), A = E − 60°, a = 6√7, b = c = d = e = 14; 2-isohedral, 8 tiles (four direct, four mirror) per rectangular cell, edge-to-edge, two perpendicular glide families squaring to w1 and −w2 | pgg | rectangular |
| `pentagon-13` | Pentagon type 13 (Rice, 1977) | pentagonal (Rice 1977, A = 120°): d = 2a = 2e, B = E = 90°; 2-isohedral chiral pair, 8 tiles per cell | p2 | rectangular |
| `pentagon-14` | Pentagon type 14 (Stein) | pentagonal (Stein 1985, completely determined): 2a = 2c = d = e, b = βa, A = 90°; 3-isohedral, 6 tiles per oblique cell | p2 | oblique |
| `pentagon-15` | Pentagon type 15 (Mann) | pentagonal (Mann / McLoud-Mann / Von Derau 2015): A = 150°, B = 60°, C = 135°, D = 105°, E = 90°, a = 2b, c² = 2+√3; 3-isohedral, 12 tiles per rectangular cell, two perpendicular glides | pgg | rectangular |
| `penrose` | Penrose (P3) | quasiperiodic, 5-fold local symmetry | — | — |
| `penrose-p1` | Penrose P1 (pentagon, star, boat, diamond) | quasiperiodic 4-prototile φ²-deflation with short-edge merges across parent boundaries; 5-fold local symmetry | — | — |
| `penrose_p2` | Penrose (P2) | quasiperiodic kite-dart tiling by half-tile substitution; local 5-fold symmetry | — | — |
| `penrose-pentagon` | Penrose Pentagon (5-tile φ-substitution) | quasiperiodic five-pentagon substitution tiling; 5-fold local symmetry; non-edge-to-edge (φ-substitution split points at t = 1/φ on parent edges) | — | — |
| `ammann-beenker` | Ammann-Beenker (A5) | quasiperiodic square+45°-rhombus by substitution; local 8-fold symmetry | — | — |
| `stampfli` | Stampfli (12-fold) | quasiperiodic square+triangle+30°-rhombus by substitution (inflation 2+√3); exact dodecagonal seed star, local 12-fold symmetry | — | — |
| `shield` | Gähler shield (12-fold) | quasiperiodic shield-hexagon+square+triangle substitution (Gähler; inflation √(2+√3), kinked supertiles, 4 protos / 8 labelled kinds); dodecagonal frame, Bragg 12-fold, no local 30° center | — | — |
| `pinwheel` | Pinwheel (1–2–√5) | aperiodic Pythagorean; tiles in infinitely many orientations | — | — |
| `hat` | Hat (aperiodic monotile) | aperiodic einstein (SMKG 2023); H/T/P/F metatile substitution; tiles only with its mirror image; non-edge-to-edge | — | — |
| `turtle` | Turtle (hats in turtles) | aperiodic einstein family (SMKG 2023); chiral hats-in-turtles: turtle + hat prototiles, 9-label substitution, homochiral (all tiles unreflected — the plain turtle tiling mixes reflected copies), edge-to-edge hex-lattice geometry | — | — |
| `spectre` | Spectre (chiral einstein, Tile(1,1)) | aperiodic chiral einstein family (SMKG 2023); single equal-edge prototile, aperiodic without reflections, 9-label substitution, exact Q(√3) geometry | — | — |
| `taylor-socolar` | Taylor-Socolar striped hexagon | aperiodic decorated-hexagon monotile (Taylor & Socolar, arXiv:1008.3103); the paper's forced 529-hexagon patch (its Fig. 18), each hexagon partitioned into 8 decorated sub-polygons carrying the canonical chord + split-axis decoration (one thick vertex-to-vertex chord plus a centre-split red/blue axis) whose chords close into nested equilateral Taylor triangles, state rules realized exactly (R1 0/2992, R2 0/2816, R3 0/2904, re-derived in tests); finite covered patch, no period within hex-radius 30 | — | — |
| `jeandel-rao` | Jeandel-Rao 11-tile Wang tiles | aperiodic 11-tile edge-colored Wang set (Jeandel-Rao, arXiv:1506.06492); the author's canonical 100x100 patch, each unit cell partitioned into 5 sub-polygons whose edge bands carry the Wang edge colors (H 0/9900, V 0/9900, exact 9-color ids, re-derived in tests); finite covered patch, no period |t|<=30 | — | — |
| `hirschhorn` | Hirschhorn pentagon | nonperiodic monohedral (Hirschhorn–Hunt 1985) | — | — |
| `chair` | Chair (L-tromino) | aperiodic limit-periodic λ=2 substitution; tiles in 4 orientations; non-edge-to-edge | — | — |
| `sphinx` | Sphinx (pentagonal hexiamond) | aperiodic limit-periodic λ=2 rep-4 substitution; tiles in 12 orientations; non-edge-to-edge | — | — |
| `p-pentomino` | P-pentomino (rep-4 reptile, limit-periodic) | aperiodic limit-periodic λ=2 rep-4 substitution; tiles in 8 chiral orientation classes; non-edge-to-edge | — | — |
| `girih-quasiperiodic` | Girih star-and-cross (quasiperiodic, {10/3}) | quasiperiodic girih via the Darb-i Imam 3-prototile substitution (Lu & Steinhardt 2007), IF = 2φ³; exact 10-fold seed rosette | — | — |

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

## Design

### Crate layout

```
tessellix/
├── Cargo.toml              # workspace (resolver 2, edition 2021, MIT)
└── crates/
    ├── tess-core/          # core abstractions — no external deps
    ├── tess-tilings/       # built-in generators, one cargo feature each
    ├── tess-tui/           # ratatui TUI + CLI (list / export)
    └── tess-gui/           # eframe/egui desktop app
```

### The `Tiling` trait (`tess-core`)

Everything is a generator behind one trait:

```rust
pub trait Tiling {
    fn id(&self) -> &'static str;          // e.g. "square"
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn params(&self) -> &'static [ParamSpec];
    fn default_color_mode(&self) -> ColorMode { ColorMode::Pattern }
    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult;
}
```

- **`Query`** carries only a `BBox` region; generators return exactly the
  tiles intersecting it — no gaps, no overlaps, nothing outside.
- **`Tile`** = `{ proto, poly, seed }`: a concrete polygon in world
  coordinates, a prototile index, and a small integer the generator chose
  to drive "pattern" coloring (e.g. checkerboard parity, triangle
  orbits).
- **`TilingResult`** = tiles + `SymmetryMeta` (wallpaper group, lattice
  kind, notes) + `Option<[Vec2; 2]>` periodic basis + an optional refusal
  message (the tile budget).
- Generators are **stateless**; all variation flows through
  **`ParamSet`**.

### Self-describing parameters

Every tiling declares its parameters as `ParamSpec { id, label, kind }`
where `kind` is `Float{min,max,step}`, `Int`, `Bool`, or `Angle` (with
wrap-around stepping). Both UIs render editors from these specs
generically — **adding a tiling never requires touching UI code**.

### The `LatticeTiling` framework (`tess-tilings`)

All 15 periodic tilings are **data tables**, not code. A
`LatticeTiling` is:

- a lattice basis (two cell-space vectors),
- a set of cell polygons (prototiles, which may extend beyond the unit
  cell — only their lattice translates need to tile the plane),
- a cell-to-world map (default: uniform scale then rotation, but any
  `Affine` works, which is how anisotropic families could be expressed),
- symmetry metadata (wallpaper group, lattice kind),
- a seed pattern (`Uniform`, `Checker` — `(i+j) mod 2` checkerboard, or
  `Triad` — `(i-j) mod 3` 3-coloring for hex/triangle lattices).

Generation inverts the lattice map to the query region corners, derives
the index ranges that can intersect it (per-cell extents are measured in
lattice coordinates — cell vertices mapped through the inverse basis —
so sheared or rotated bases cannot under-cull), transforms each cell
once per parameter set, then translates — with per-tile AABB culling
against the region. A brute-force test re-emits every tile over a fixed
index window and requires the exact same tile set as the generator.

**Parameterized-lattice exception:** the Pythagorean two-square tiling
(`pythagorean.rs`) is periodic but its lattice depends on a parameter
(the square ratio), so it cannot be a static data table; it implements
`Tiling` directly with this same u-space index-range culling inline
(region corners inverted through the *world* lattice map, per-cell
extents in u coordinates) and carries its own brute-force culling
cross-check plus a ratio-sweep coverage test.

### Aperiodic generators

- **Penrose P3** (`penrose.rs`) — Robinson-triangle substitution
  (`O → 2O + A`, `A → O + A`) with exact φ-arithmetic child placements,
  chirality-aware (left/right-handed parents get mirrored child
  patterns). Grows outwards from the "sun" seed by repeated deflation,
  culling to the region at every step so overlaps are structurally
  impossible; triangles are paired back into thick/thin rhombi at the end.
- **Penrose P1** (`penrose_p1.rs`) — the classic four-prototile set
  (pentagon, 10-point star, boat, diamond) by exact φ² deflation: each
  tile is disassembled into short-edge (1/φ) pieces — pentagon flower
  gnomons, boat cores, 1/2-tip stars — that merge *across* parent
  boundaries by four rules (G+G→D, CORE+2G→B, S2C+2G→S, S1C+G→S);
  unmatched short edges are discarded, which makes the single-seed
  patch a pentagonal gasket with five 5-fold notch channels opening at
  the boundary (the clean interior disk of radius R/φ² is tiled
  exactly; views are scaled so the query box fits inside it).
- **Penrose P2** (`penrose_p2.rs`) — the kite-and-dart tiling, built on
  *glue-labelled half-tiles*: kite halves `A` (36-72-72) and dart halves
  `O` (36-108-36) substitute by `A → 2A + O`, `O → A + O` and pair along
  their glue legs into kites (two `A`) and darts (two `O`) — half-darts
  pair across parent boundaries, which is why growth seeds with the
  sun/star patches (single tiles do not deflate consistently, the
  classical P2 caveat). The child placements are the unique globally
  consistent labelling for this half-tile frame (found by an exhaustive
  8192-combination search over frame/glue assignments, pruned by
  sun/star deflation checks); verified in exact arithmetic over
  Q(φ, sin 36°) and re-checked by Rust tests (census law
  `K′ = 2K + D, D′ = K + D`, edge-to-edge merged patches, kite/dart
  side patterns). Sun or star seed selectable via a `star seed` param.
- **Ammann-Beenker A5** (`ammann_beenker.rs`) — squares and 45° rhombi
  under the silver-mean substitution λ = 1+√2 (square → 3 squares + 4
  rhombi, rhombus → 2 squares + 3 rhombi). Works on half-squares: the
  two same-shaped `RSquare`/`LSquare` labels carry the matching
  information (every interior square diagonal joins an R and an L
  half). Seeds with the 8-rhombus star (inradius √2/2·scale), deflates
  with per-step culling exactly like Penrose, and pairs halves back into
  squares on shared hypotenuses; every child vertex lies in Z[√2].
- **Stampfli 12-fold** (`stampfli.rs`) — squares, equilateral triangles
  and 30° rhombi under Stampfli's substitution (inflation λ = 2+√3,
  rule from the Bielefeld Tilings Encyclopedia): square → 1 square +
  12 rhombi + 20 triangles, rhombus → 2 squares + 3 rhombi + 12
  triangles, triangle → 6 rhombi + 10 triangles (unit children per
  parent, edge partition 1 : √3/2 : √3/2 : 1). Some children straddle
  parent edges with exactly one vertex outside; neighbouring parents
  generate the same straddler, so pieces are deduped by their exact
  vertex set. Seeds with the 12-rhombus star at scale λ^n and deflates
  to unit pieces with per-step culling; all anchors stay exact in
  Z[√3]/4, giving edge-to-edge, T-junction-free patches with exact
  12-fold symmetry about the seed at every generation.
- **Gähler shield** (`shield.rs`) — the dodecagonal shield tiling
  (Gähler 1988): shield hexagon, square and two unit triangles, each in
  two labelled orientations R/L (the labels carry the matching rules,
  so 8 substitution kinds collapse to 4 output prototiles). Inflation
  θ = 2cos 15° = (√6+√2)/2, Perron eigenvalue of the 8×8 count matrix
  θ² = 2+√3. Children tile a *kinked* supertile (±¼-unit-square
  triangular notches; shield 6 in, square 4 in, black tri 3 out, white
  tri 2 out + 1 in), verified per kind in exact `Q(√2,√3)` arithmetic.
  Seeds with the 13-tile kinked-shield supertile (area centroid exact at
  √2/2 + √6/3, covering a disk of inradius (1+√3)/2 per scale) and
  deflates with per-step culling. All edges lie on the 12 dodecagonal
  directions 15°·k (matching window: regular dodecagon); the 12-fold
  character is Bragg/MLD — no local 30° rotation center exists (every
  tile angle ≥ 60°).
- **Pinwheel** (`pinwheel.rs`) — Conway–Radin tiling by the 1–2–√5
  right triangle; √5 inflation with an `arctan(1/2)` twist, so tiles
  occur in infinitely many orientations. Labeled-vertex child placement
  handles both chiralities. The substitution is not edge-to-edge (four
  children meet at the midpoint of a long leg); those T-junctions are
  inherent and verified by tests, not bugs. The emitted patch is
  view-independent: seed grids are anchored to the origin and quantized
  to √5 octaves, so panning/zooming only re-culls a fixed global tiling
  (the seed depth steps once when the view extent crosses a √5 octave),
  and "pattern" mode colors tiles by their substitution slot — the five
  distinct child roles of the rule.
- **Chair** (`chair.rs`) — the L-tromino substitution tiling (λ = 2):
  each chair (2×2 squares minus one quadrant) splits into 4 half-size
  chairs. An exhaustive enumeration shows the inflated chair has
  exactly ONE L-tromino dissection, so the rule is forced; it is
  inherently non-edge-to-edge (a child vertex lands inside a sibling's
  edge at every scale — asserted by tests, like the pinwheel's
  T-junctions), and the tiling is limit-periodic. Generation is exact
  integer arithmetic: 4×3 seed blocks (one chair per orientation) on an
  origin-anchored lattice, deflated a power-of-two-octave number of
  steps so tiles always land at unit scale; panning/zooming within an
  octave only re-culls a fixed global tiling. "Pattern" mode colors by
  orientation (the four orientation classes are equally numerous at
  every generation).
- **Sphinx** (`sphinx.rs`) — the pentagonal hexiamond rep-tile (six
  unit triangles of the triangular lattice, five boundary sides; λ = 2
  rep-4). Exact arithmetic derives everything: of the 12 free
  hexiamonds only the 3×1 rhombus and the sphinx are rep-4 (the sphinx
  the only pentagonal one), and the inflated sphinx has exactly ONE
  dissection, so the rule is forced. The dissection mixes chiralities,
  so tiles come in all 12 lattice-orientation classes (substitution
  matrix doubly stochastic: rows and columns sum to 4). Inherently
  non-edge-to-edge like the chair. Generation is exact integer
  arithmetic on the triangular lattice: the 2×3 lattice rhombus (the
  unique smallest rhombic patch — exactly two sphinxes, found by an
  exhaustive cover search) seeds origin-anchored blocks deflated a
  power-of-two-octave number of steps, so tiles always land at unit
  scale (area 3√3/2); "pattern" mode colors by orientation class.
- **P-pentomino** (`p_pentomino.rs`) — the P-pentomino rep-tile (five
  squares; 2×2 block plus one extra), λ = 2 rep-4. Because the P is
  chiral, its dissection mixes chiralities: exhaustive enumeration finds
  exactly two dissections of the inflated P (class multisets {0,1,4,6}
  and {3,4,4,5}); the canonical one is used, and the 8 orientation
  classes (4 rotations × 2 chiralities) stay equally numerous (the
  substitution matrix is doubly stochastic). Inherently
  non-edge-to-edge (five T-junctions per parent at every scale).
  Generation is exact integer arithmetic: 5×8 seed blocks (one P per
  orientation class — 62 such tilings of the rectangle exist, this one
  is class-complete) on an origin-anchored lattice, deflated a
  power-of-two-octave number of steps so tiles always land at unit
  scale (area 5); "pattern" mode colors by orientation class.
- **Turtle** (`turtle.rs`) — "hats in turtles", the two-prototile
  member of the einstein family (SMKG 2023, arXiv:2305.17743): 14-gon
  turtle + hat on the exact hex lattice, 9 matching-rule labels
  (Γ..Ψ, Γ the Mystic turtle+hat compound), 8+1 key-point substitution
  with a per-level chirality flip that makes the global tiling
  homochiral (odd-depth patches are mirrored into the root placement,
  so every emitted tile is unreflected). Generation is per-view: the
  query region is covered by one rigid depth-Δ patch (depths 1–5,
  full-patch counts 9/71/559/4401/34649, max inscribed radius ≈129.9)
  whose centroid is snapped to a 4-hex-unit anchor grid near the view
  center; area ratios converge to 4+√15 per depth. Edge-to-edge
  (all vertices are hex-lattice points; the √3-edge midpoints are not),
  verified by the Python exact-integer reference against Kaplan's
  spectre.js at every depth.
- **Spectre** (`spectre.rs`) — the chiral einstein Tile(1,1) (SMKG
  2023, arXiv:2305.17743): a single 14-gon prototile with all unit
  edges and area exactly 3+3√3, aperiodic *without reflections*. Same
  nine-label substitution as the turtle (identical `SUPER_RULES` /
  `T_RULES` tables), but the Mystic compound is two spectres — the
  second rotated 30° about the origin — so ring children occur at
  30°-multiple orientations and every placement is an exact D12
  transform over Q(√3): vertices are stored as surds
  `((a + b√3)/2ⁿ, (c + d√3)/2ⁿ)` and 30° rotation closes on them.
  Odd-depth patches are mirrored into the root placement so every
  emitted tile is unreflected; edge-to-edge, per-view rigid patches
  snapped to a 4-world-unit grid (depths 1–5, full-patch counts
  9/71/559/4401/34649, max inscribed radius ≈90.4), verified
  pose-identical to Kaplan's spectre.js at every depth 0–5 by the
  exact Python surd reference.
- **Taylor-Socolar striped hexagon** (`taylor_socolar.rs`) — the
  decorated-hexagon aperiodic monotile (arXiv:1008.3103), ported as a
  data table from the audited upstream study (M86; decoration reworked
  in M86-fix to the canonical closed-triangle form): the paper's forced
  529-site patch (Fig. 18) with its 12 tile states `(a, rbt, sdt)`,
  every hexagon partitioned into 8 non-overlapping convex sub-polygons
  — the band around one thick vertex-to-vertex chord (proto 1, two
  halves), the two halves of the centre-split diameter (red proto 2 /
  blue proto 3, the red end picked by `rbt`), and 2 body pentagons +
  2 body triangles (proto 0). All cuts are exact in Q(√3) (area sum
  `3√3/2`, 0 overlaps, strict convexity), the chord rule
  `kappa = (a + 3·sdt) mod 6` pairs every interior chord end (0 of 1058
  unpaired) and is the only one of the 125 pairing rules among all 1296
  affine candidates that closes any ring at all, so the chord network
  **closes** into
  nested equilateral Taylor triangles on three scales (60 of side √3,
  12 of side 3√3, 2 of side 7√3) and every band colour is
  state-determined. The
  underlying rules are unchanged and re-derived in tests (R1 0/2992,
  R2 0/2816, R3 0/2904). The covered
  domain is the finite patch (girih precedent), not the plane.
- **Jeandel-Rao 11-tile Wang tiles** (`jr_aperiodic.rs`) — the smallest
  known aperiodic set of Wang tiles (edge-colored unit squares, no
  rotations or reflections; Jeandel-Rao, arXiv:1506.06492), ported as a
  data table from the audited upstream study (M93): the author's
  canonical 100x100 patch, every unit cell partitioned into 5
  non-overlapping convex sub-polygons — four edge bands of width 0.08
  plus the interior remainder — whose band colors make the Wang edge
  matching visible exactly where the handoff colors agree (H 0/9900,
  V 0/9900 on exact 9-color ids, re-derived in tests); tile frequencies
  and a radius-30 no-period probe are pinned against the study.
  Aperiodicity is cited (machine-verified upstream), not re-proved here;
  the covered domain is the finite patch (Taylor-Socolar precedent), not
  the plane.
- **Hirschhorn** (`hirschhorn.rs`) — the 140°/60°/160°/80°/100° convex
  pentagon (n=6 of the C11-T1A Sugimoto family). Since the tiling is not
  locally forced, the generator constructs one valid realization by
  constrained 6-fold-symmetric growth: at each step it enumerates every
  legal placement across the boundary (both chiralities, all template
  edges/directions), filters by local rules (no overlap, edge-to-edge,
  angle sums ≤ 360° with no unclosable residual gap, no duplicates),
  groups candidates into complete 60° rotation orbits, and commits the
  first orbit that closes vertices, keeps the patch a topological disk,
  and strands no dead boundary edge. Every patch is C6-symmetric about
  the seed; tile frames land on multiples of 20°.
- **Girih** (`girih.rs`, `girih_qc.rs`) — the girih prototiles with
  exact unit-edge radical geometry. `girih-shesh-band` is a periodic
  single-tile oblique-lattice band. `girih-quasiperiodic` generates the
  genuinely quasiperiodic star-and-cross pattern with the Darb-i Imam
  3-prototile substitution (Lu & Steinhardt 2007): decagon, bobbin and
  bow-tie, inflation 2(2+√5) = 2φ³, an edge-marker seam rule, exact
  arithmetic in Q(2cos 18°), an exact 10-fold rosette around the seed
  decagon, and edge-to-edge, T-junction-free patches at every
  generation (frozen core: tiles are depth-independent inside radius
  ~79).
- **Najm** (`najm.rs`, feature `najm`) — the Najm template-tiling
  generator (Kaplan diss. §3.6.1, roadmap 22b): notation
  `([p,q]; m_A o_A, m_B o_B, m_C o_C; inflation)` → circumradii
  (`NajmSpec::radii`, inflation cases 1–4 via the absolute-trig extent
  equations) → substrate cell tables. First two entries:
  `najm-63-2e-0-3e-ac` (dodecagons + enneagons + bowties) and
  `najm-44-3e-0-0-a` (dodecagons + 4-pointed stars). Hole tiles use
  construction-specific formulas; a generic hole decomposition and the
  motif layers (22a/22c) are future work.

### Shared UI state

`tess-core::App` holds the registry, the selected tiling, per-tiling
parameter sets (parameters are remembered per tiling within a session),
the viewport (center + zoom, with cursor-anchored zoom), the color mode,
and the search filter. The TUI and GUI are thin layers over it.

`tess-core::browse` is the generic browser model both UIs consume:
`CardMeta` (a card's static display data — name, category, wallpaper
group, lattice kind, notes, parameter count, description), `Tab` (a
category and its cards), `browse_tabs` (one tab per display category in
`TilingCategory::ALL` order, Wallpaper cards ordered by crystal system
then wallpaper group, the other tabs in registry order), and
`section_key` (a card's crystal-system section header, used by both
UIs' card grids for the oblique/rectangular/centered/square/hexagonal
grouping). A tiling's **primary** category is its family; it may also
declare `secondary_categories`, so a card can appear in more than one
tab — the Wallpaper tab is one card per wallpaper group (the group
catalog, complete since M85: all 17 groups are represented), and the
tab's note (`wallpaper_gap_note`) reports the completed catalog.
The model knows nothing about tiling ids or rendering — the TUI and the
egui GUI render these tabs and cards; card previews rasterize through
the shared `tess-core::raster` (the TUI to character cells over its
256-cube palette, the GUI to texture pixels mapped straight into
`paint::PALETTE`).

### Rendering

- **Color modes** — `single` (one flat color), `pattern` (color by tile
  `seed`, the generator's choice), `proto` (color by prototile index).
  Shared 8-color palette; the TUI maps it to the nearest xterm-256
  indices and rasterizes to character cells, the GUI draws convex
  polygons directly, and the CLI writes SVG paths (y-axis flipped to
  match the math orientation, hairline border stroke scaled to the view).

## Tests

`cargo test --workspace` (currently 668 tests, all green, zero
warnings):

- **`tess-core`** — parameter defaults/stepping/wrapping, registry,
  viewport anchoring, palette mapping.
- **`tess-tilings`** — for every tiling: `covers_region` (sample points
  are inside ≥ 1 tile, and total clipped tile area matches the region
  area to 1e-6 relative — catching gaps *and* overlaps),
  `rotated_scaled` (coverage holds under non-axis-aligned scale +
  rotation), and `deterministic` (two generations produce identical
  geometry, prototile classes, and pattern seeds).
- **Invariant censuses** — pinwheel intersection rule and T-junction
  census, chair census laws (counts 4^(g+1), uniform orientations,
  exact per-generation cell tilings, inherent T-junction), sphinx
  census laws (counts 2·4^g, class law = substitution-matrix powers,
  exact per-generation unit-triangle tilings, refined edge census,
  inherent T-junctions), p-pentomino census laws (counts 8·4^g,
  uniform 8 chiral orientation classes, exact per-generation cell
  tilings, inherent T-junctions), hat invariants (metatile + hat count tables
  for 5 levels × 4 types, exact nesting, no-overlap patch with the
  565-midpoint T-junction census, integer-grid matching rules, all 9
  cluster roles + the reflected hat, outline geometry in all 12
  orientations), turtle invariants (outline geometry in all 12
  orientations, substitution counts against the verified reference,
  full-patch edge census with Euler χ = 1 at every depth, anchor-depth
  table validation, deterministic patch stability across views),
  spectre invariants (outline geometry in all 24 D12 orientations,
  substitution counts against the verified reference incl. the Mystic
  Γ-leaf law, full-patch edge census with Euler χ = 1, anchor-depth
  table validation, deterministic patch stability across views),
  Taylor-Socolar invariants (the handoff battery re-derived from the
  shipped table reproduces the study's JSON numbers R1 0/2992,
  R2 0/2816, R3 0/2904; world stripe points agree across all 2992
  shared edges to 1e-12; the whole-patch emission is exactly
  529 × 13 pieces matched signature-by-signature with each hexagon's
  piece areas summing to 3√3/2; a no-period probe over the 2790
  hex-radius-≤30 translations finds none),
  Jeandel-Rao invariants (the handoff battery re-derived from the
  shipped 100×100 patch reproduces the study's JSON numbers H
  0/9900 and V 0/9900 edge mismatches on the exact 9 edge-color ids;
  tile frequencies match the handoff table (sum 10000); the
  per-cell 5-piece partition has areas summing to 1 exactly; a
  no-period probe over the 2820 translations with |t| ≤ 30 finds
  none),
  Hirschhorn
  patch invariants (Euler χ = 1, exact C6 symmetry,
  edge ownership ≤ 2, 20° frame quantization), and a full edge/vertex
  census of the embedded girih patch. For the periodic tables, a
  brute-force test re-emits every tile over a fixed index window and
  requires the exact same tile set as the generator's range-based culling.

The periodic tilings' geometry was additionally verified analytically
before implementation: for each, the cell area exactly equals
`|det(lattice)|`, every interior edge is shared by exactly two cells, and
every interior vertex realizes the stated vertex figure.

## Adding a tiling

**Periodic (wallpaper) tiling** — the common case:

1. Write a function returning a `LatticeTiling` data table (see
   `archimedean.rs` for 8 examples): lattice basis, cell polygons,
   wallpaper group, lattice kind, seed pattern.
2. Gate it behind a cargo feature and register it in `registry()` in
   `tess-tilings/src/lib.rs`.
3. Add the standard three tests (`covers_region`, `rotated_scaled`,
   `deterministic`) using `check_coverage` / `check_deterministic`.

Both UIs then pick it up automatically.

**Aperiodic / continuous family** — implement `tess_core::Tiling`
directly (see `penrose.rs`, `ammann_beenker.rs`, `pinwheel.rs`,
`hirschhorn.rs`), declare its category in the impl (`TilingCategory` —
see `tess-core/src/category.rs`), register it, and test it the same way.
