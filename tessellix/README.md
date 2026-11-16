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

`cargo run -p tess-tui` starts the terminal UI. With arguments it acts as
a CLI:

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

**TUI keys:** `↑`/`↓` select tiling (wraps, respects the search filter) ·
`Tab` switch list ↔ params · `←`/`→` adjust the parameter under the cursor
· `h`/`j`/`k`/`l` pan · `+`/`-` zoom · `c` cycle color mode · `r` reset view
· `/` search · `q` quit (from the list; first exits params mode).

### GUI

`cargo run -p tess-gui` opens an egui window (1024×640, dark theme):

- **Left panel** — filter box, tiling list grouped into collapsible
  categories (Uniform, Laves/Catalan duals, Wallpaper, Quasiperiodic,
  Aperiodic, Nonperiodic — declared per tiling, so new tilings sort
  themselves; the search filter also matches category names and shows
  flat results), parameter editors (drag
  values within each parameter's declared range), color-mode selector
  (single / pattern / proto), and symmetry readout (wallpaper group,
  lattice kind, basis vectors, notes). The readout is cached per tiling
  + parameter values and refreshed through a tiny fixed probe region —
  panel rendering never triggers view-sized tile generation.
- **Canvas** — drag to pan, scroll to zoom (anchored at the cursor). The
  tiling is regenerated for the current viewport on every frame; if the
  view would exceed the tile budget (250 000 tiles), a
  "too many tiles for this view — zoom in" message is shown instead.

## Supported tilings

59 built-in tilings, all discoverable in both UIs via the registry
(`tess-tui -- list`). Every tiling declares one of six canonical
display categories — **Uniform** (11), **Laves / Catalan duals** (9),
**Wallpaper** (23), **Quasiperiodic** (8: Penrose family, Ammann–Beenker,
Stampfli, shield, girih Darb-i Imam), **Aperiodic** (7: einstein family, limit-periodic
rep-tiles, pinwheel), and **Nonperiodic** (1: Hirschhorn) — and both
UIs group their lists by it. Categories are declared on the tiling
(never derived from its symmetry), so the grouping is honest about
what the mathematics distinguishes; see `tess-core/src/category.rs`.

| id | name | type | wallpaper group | lattice |
|---|---|---|---|---|
| `square` | Square grid (4.4.4.4) | uniform | p4m | square |
| `hexagon` | Hexagonal (6.6.6) | uniform | p6m | hexagonal |
| `triangle` | Triangular (3.3.3.3.3.3) | uniform | p6m | hexagonal |
| `truncated-square` | Truncated square (4.8.8) | Archimedean | p4m | square |
| `trihexagonal` | Trihexagonal (3.6.3.6) | Archimedean | p6m | hexagonal |
| `snub-trihexagonal` | Snub trihexagonal (3.3.3.3.6) | Archimedean, chiral | p6 | hexagonal |
| `elongated-triangular` | Elongated triangular (3.3.3.4.4) | Archimedean | cmm | rectangular |
| `rhombitrihexagonal` | Rhombitrihexagonal (3.4.6.4) | Archimedean | p6m | hexagonal |
| `truncated-trihexagonal` | Truncated trihexagonal (4.6.12) | Archimedean | p6m | hexagonal |
| `truncated-hexagonal` | Truncated hexagonal (3.12.12) | Archimedean | p6m | hexagonal |
| `snub-square` | Snub square (3.3.4.3.4) | Archimedean | p4g | square |
| `star-cross` | Star and cross ({8/3}) | Islamic star-and-cross family (one-parameter) | p4m | square |
| `hexagram` | Hexagram and hexagon ({6/2}) | hexagram star-and-hexagon family (one-parameter) | p6m | hexagonal |
| `cairo-pentagonal` | Cairo pentagonal (3.3.3 / 4.4.4.4) | pentagonal (Catalan dual of the snub square) | p4g | square |
| `cairo-collinear` | Cairo collinear (3.3.3 / 4.4.4.4) | pentagonal, integer-coordinate collinear form | p4g | square |
| `tetrakis-square` | Tetrakis square (V4.8.8) | Laves (Catalan dual of the truncated square) | p4m | square |
| `rhombille` | Rhombille (V3.6.3.6) | Laves (Catalan dual of the trihexagonal); 3-colored cube illusion | p6m | hexagonal |
| `triakis-triangular` | Triakis triangular (V3.12.12) | Laves (Catalan dual of the truncated hexagonal) | p6m | hexagonal |
| `deltoidal-trihexagonal` | Deltoidal trihexagonal (V3.4.6.4) | Laves (Catalan dual of the rhombitrihexagonal) | p6m | hexagonal |
| `kisrhombille` | Kisrhombille (V4.6.12) | Laves (Catalan dual of the truncated trihexagonal) | p6m | hexagonal |
| `prismatic-pentagonal` | Prismatic pentagonal (V3.3.3.4.4) | Laves (Catalan dual of the elongated triangular) | cmm | rectangular |
| `floret-pentagonal` | Floret pentagonal (V3.3.3.3.6) | Laves (Catalan dual of the snub trihexagonal), chiral | p6 | hexagonal |
| `girih-shesh-band` | Girih shesh-band (6.6.6) | periodic girih strapwork band | p2 | oblique |
| `girih-star-cross` | Girih star-and-cross, 10-fold | measured classical strapwork panel: 10-pointed stars, bobbins, chord straps; 545-tile cell data table | pmm | oblique |
| `girih-w85` | Girih 12-pointed star interlace (W85) | Wichmann panel data18/W85 (Alhambra, Mexuar patio): exact Q(√3) strap grid L = 283.92, strap 9; 42 tiles per hexagonal cell; drawn interlace chiral | p6 | hexagonal |
| `pic-4-8-8` | Hankin PIC fourfold khatam | Hankin polygons-in-contact strapwork on the 4.8.8 substrate (contact angle δ = 45°): midpoint-chord walls — {8/2} octagram in each octagon — partitioning the plane into octagon + small square + 4 house pentagons + 8 star tips per cell, exact Q(√2) | p4m | square |
| `pic-3-6-3-6` | Hankin PIC sixfold khatam | Hankin polygons-in-contact strapwork on the trihexagonal (3.6.3.6) substrate (contact angle δ = 60°): midpoint-chord walls — {6/2} hexagram in each hexagon, midsegments in each triangle — partitioning the plane into 4 hexagram-core hexagons + 6 hexagram tips + 2 midsegment triangles per cell (a half-scale 3.6.3.6), exact Q(√3) | p6m | hexagonal |
| `pic-3-12-12` | Hankin PIC twelvefold khatam | Hankin polygons-in-contact strapwork on the truncated-hexagonal (3.12.12) substrate (contact angle δ = 60° — the only angle closing both the dodecagon and the triangle): midpoint-chord walls — {12/4} twelvefold rosette in each dodecagon, midsegments in each triangle — partitioning the plane into rosette-core dodecagon + 6 vertex pentagons + 12 band quads + 2 medial triangles + 12 inner quads + 12 star notches per cell, exact Q(√3) | p6m | hexagonal |
| `khatam-girih` | Khatam girih decomposition (8-point) | classical kissing {8/3} star-and-cross cut into 8 rhombi + square + 4 pentagons per cell (exact Q(√2) kitab) | p4m | square |
| `basketweave` | Basketweave (2:1 dominoes) | paver bond: 2×2 blocks of two parallel 2:1 dominoes, checkerboard orientation; long sides split into unit edges | p4g | square |
| `herringbone` | Herringbone (2:1 dominoes) | paver bond: chiral L-pairs of 2:1 dominoes, single-direction; long sides split into unit edges | pgg | oblique |
| `running-bond` | Running bond (2:1 dominoes) | paver bond: stretcher rows of 2:1 dominoes, each row offset by half a brick; long sides split into unit edges | cmm | rectangular |
| `pythagorean` | Pythagorean (two squares) | periodic two-square family (one-parameter ratio); square on the hypotenuse per cell | p4 | square |
| `windmill` | Periodic pinwheel arrangement (p4) | Conway's periodic "spin" of the pinwheel motif: 1–2–√5 triangles around squares; homochiral, non-edge-to-edge | p4 | square |
| `kuniform-p6m` | 2-uniform (3⁶; 3².6²) | 2-uniform k-uniform: hexagonal tiling with one of every three hexagons dissected into six-triangle fans (Wikipedia `2-uniform n10`) | p6m | hexagonal |
| `kuniform-pmm` | 2-uniform (3.6.3.6; 3².6²) | 2-uniform k-uniform: hexagonal tiling with rows of up- and down-pointing triangles filling the gaps between rows of hexagons (Wikipedia `2-uniform n11`) | pmm | rectangular |
| `pentagon-9` | Pentagon type 9 (Rice) | pentagonal (Rice 1977): A = C = 120°, cos B = 1/7, cos D = 23/98, cos E = −11/14, a = 6, b = c = d = e = 14; 2-isohedral, 8 tiles per rectangular cell, two perpendicular glide families squaring to the primitive translations | pgg | rectangular |
| `pentagon-10` | Pentagon type 10 (Kershner) | pentagonal (Kershner 1968): A = 90°, B + E = 180°, a = b = c + e, d² = c² + e² (reference instance B = 90°, c = 1: A = B = E = 90°, C = D = 135°, a = b = 2, c = e = 1, d = √2); 3-isohedral, 6 tiles per cell, non-edge-to-edge (each length-2 side split at its exact midpoint), mirrors y = x and y = −x, half-turn at the origin, genuine glide (axis x + y = 3/2), centered rectangular lattice | cmm | oblique |
| `pentagon-11` | Pentagon type 11 (Kershner) | pentagonal (Kershner 1968): A = 90°, B = 156°, C = 48°, D = 114°, E = 132°, 2a + c = d = e (reference instance with c = 1); 2-isohedral, 8 tiles per rectangular cell, non-edge-to-edge (every d-side carries one c-side and two a-sides), two perpendicular glide families squaring to w1 and −w2 | pgg | rectangular |
| `pentagon-12` | Pentagon type 12 (Kershner) | pentagonal (Kershner 1968): A = 90°, B = 150°, C = 60°, D = E = 120°, 2a = d = c + e (reference instance with c = 1); 2-isohedral, 8 tiles per rectangular cell, non-edge-to-edge (every d-side is split at its exact midpoint into two unit halves), two perpendicular glide families squaring to w1 and −w2 | pgg | rectangular |
| `pentagon-13` | Pentagon type 13 (Rice) | pentagonal (Rice type 13, A = 120°): d = 2a = 2e, B = E = 90°; 2-isohedral chiral pair, 8 tiles per cell | p2 | rectangular |
| `pentagon-14` | Pentagon type 14 (Stein) | pentagonal (Stein 1985, completely determined): 2a = 2c = d = e, b = βa, A = 90°; 3-isohedral, 6 tiles per oblique cell | p2 | oblique |
| `pentagon-15` | Pentagon type 15 (Mann) | pentagonal (Mann / McLoud-Mann / Von Derau 2015): A = 150°, B = 60°, C = 135°, D = 105°, E = 90°, a = 2b, c² = 2+√3; 3-isohedral, 12 tiles per rectangular cell, two perpendicular glides | pgg | rectangular |
| `penrose` | Penrose (P3) | aperiodic, 5-fold local symmetry | — | — |
| `penrose-p1` | Penrose P1 (pentagon, star, boat, diamond) | aperiodic 4-prototile φ²-deflation with short-edge merges across parent boundaries; 5-fold local symmetry | — | — |
| `penrose_p2` | Penrose (P2) | aperiodic kite-dart tiling by half-tile substitution; local 5-fold symmetry | — | — |
| `penrose-pentagon` | Penrose Pentagon (5-tile φ-substitution) | aperiodic five-pentagon substitution tiling; 5-fold local symmetry | — | — |
| `ammann-beenker` | Ammann-Beenker (A5) | aperiodic square+45°-rhombus by substitution; local 8-fold symmetry | — | — |
| `stampfli` | Stampfli (12-fold) | aperiodic square+triangle+30°-rhombus by substitution (inflation 2+√3); exact dodecagonal seed star, local 12-fold symmetry | — | — |
| `shield` | Gähler shield (12-fold) | aperiodic shield-hexagon+square+triangle substitution (Gähler; inflation √(2+√3), kinked supertiles, 4 protos / 8 labelled kinds); dodecagonal frame, Bragg 12-fold, no local 30° center | — | — |
| `pinwheel` | Pinwheel (1–2–√5) | aperiodic Pythagorean; tiles in infinitely many orientations | — | — |
| `chair` | Chair (L-tromino) | aperiodic limit-periodic λ=2 substitution; tiles in 4 orientations; non-edge-to-edge | — | — |
| `sphinx` | Sphinx (pentagonal hexiamond) | aperiodic limit-periodic λ=2 rep-4 substitution; tiles in 12 orientations; non-edge-to-edge | — | — |
| `p-pentomino` | P-pentomino (rep-4 reptile, limit-periodic) | aperiodic limit-periodic λ=2 rep-4 substitution; tiles in 8 chiral orientation classes; non-edge-to-edge | — | — |
| `hat` | Hat (aperiodic monotile) | aperiodic einstein (SMKG 2023); H/T/P/F metatile substitution; tiles only with its mirror image; non-edge-to-edge | — | — |
| `turtle` | Turtle (hats in turtles) | aperiodic einstein family (SMKG 2023); turtle + hat prototiles, 9-label substitution, homochiral (no reflections), edge-to-edge hex-lattice geometry | — | — |
| `spectre` | Spectre (chiral einstein, Tile(1,1)) | aperiodic chiral einstein family (SMKG 2023); single equal-edge prototile, aperiodic without reflections, 9-label substitution, exact Q(√3) geometry | — | — |
| `hirschhorn` | Hirschhorn pentagon | nonperiodic monohedral (Hirschhorn–Hunt 1985) | — | — |
| `girih-quasiperiodic` | Girih star-and-cross ({10/3}) | quasiperiodic girih via the Darb-i Imam 3-prototile substitution (Lu & Steinhardt 2007), IF = 2φ³; exact 10-fold seed rosette | — | — |

Every periodic tiling reports its wallpaper group, lattice kind, and (for
periodic ones) its lattice basis as analytic symmetry metadata. The
framework models all 17 wallpaper groups and the four lattice kinds
(oblique, rectangular, square, hexagonal).

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

### Shared UI state

`tess-core::App` holds the registry, the selected tiling, per-tiling
parameter sets (parameters are remembered per tiling within a session),
the viewport (center + zoom, with cursor-anchored zoom), the color mode,
and the search filter. The TUI and GUI are thin layers over it.

### Rendering

- **Color modes** — `single` (one flat color), `pattern` (color by tile
  `seed`, the generator's choice), `proto` (color by prototile index).
  Shared 8-color palette; the TUI maps it to the nearest xterm-256
  indices and rasterizes to character cells, the GUI draws convex
  polygons directly, and the CLI writes SVG paths (y-axis flipped to
  match the math orientation, hairline border stroke scaled to the view).

## Tests

`cargo test --workspace` (currently 409 tests, all green, zero
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
  Hirschhorn
  patch invariants (Euler χ = 1, exact C6 symmetry,
  edge ownership ≤ 2, 20° frame quantization), a full edge/vertex census
  of the embedded girih patch, and the Cairo collinear integer
  properties. For the periodic tables, a brute-force test re-emits every
  tile over a fixed index window and requires the exact same tile set as
  the generator's range-based culling.

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
