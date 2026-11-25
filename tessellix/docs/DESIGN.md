# Design

The full architecture reference. The README carries only a summary; this document is the detailed, versioned description of how tessellix works.

## Crate layout

```
tessellix/
├── Cargo.toml              # workspace (resolver 2, edition 2021, MIT)
└── crates/
    ├── tess-core/          # core abstractions — no external deps
    ├── tess-tilings/       # built-in generators, eleven family features
    ├── tess-tui/           # ratatui TUI + CLI (list / export)
    └── tess-gui/           # eframe/egui desktop app
```

## The `Tiling` trait (`tess-core`)

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

## Self-describing parameters

Every tiling declares its parameters as `ParamSpec { id, label, kind }`
where `kind` is `Float{min,max,step}`, `Int`, `Bool`, or `Angle` (with
wrap-around stepping). Both UIs render editors from these specs
generically — **adding a tiling never requires touching UI code**.

## The `LatticeTiling` framework (`tess-tilings`)

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

## Aperiodic and nonperiodic generators

The aperiodic, quasiperiodic and nonperiodic members implement `Tiling`
directly — substitution/deflation generators with exact-arithmetic
anchors and per-step region culling. Their per-generator algorithm
narratives (the Robinson-triangle deflation, the half-tile
glue-labelling tricks, the T-junction censuses, the ported data-table
patches) live in each tiling's own document, `docs/tilings/<id>.md`,
linked from the family docs; the family docs' numbered build arcs
(`docs/families/<family>.md`) record how each build was constructed
and verified.

## Shared UI state

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

## Rendering

- **Color modes** — `single` (one flat color), `pattern` (color by tile
  `seed`, the generator's choice), `proto` (color by prototile index),
  `gray` (monochrome: the proto index space drawn from an 8-step
  lightness ramp — the print/e-ink mode; browse thumbnails keep their
  color previews).
  Shared 8-color palette; the TUI maps it to the nearest xterm-256
  indices and rasterizes to character cells, the GUI draws convex
  polygons directly, and the CLI writes SVG paths (y-axis flipped to
  match the math orientation, hairline border stroke scaled to the view).

## Tests

`cargo test --workspace` (currently 1300 tests, all green, zero warnings):

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

---

*[Documentation index](INDEX.md)*
