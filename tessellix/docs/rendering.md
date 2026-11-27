# Rendering star patterns: strapwork and decoration styles

Two rendering layers on top of the tilings (`--strapwork` and
`--decoration`, both built on the polygons-in-contact engine in
`tess-core::strapwork` — the walls are the star patterns' strapwork),
plus an optional **medallion third layer** (`--medallion-weave`,
below). This is the single source for the surface; the README summarizes it.

## Walls layer

`--strapwork DEG` overlays the Hankin PIC strapwork walls on **any** tiling
(plain SVG export — not a separate render mode). At each tile-edge
midpoint a symmetric pair of rays enters each neighbouring tile at the
contact angle δ, measured from the edge. The rays run to their tile boundary,
are split at every crossing, and are re-collected as distinct walls.

- Small δ packs the rays tight along the edges (tight stars).
- Large δ pushes the rays deep into the tiles (open crosses).
- Classic angles close the weave: 45° on square hosts, 60° on hexagonal
  hosts.

Worked example (verified):

```sh
cargo run -p tess-tui -- export --tiling hexagon --width 1200 --height 900 \
    --strapwork 60 --out hexagon-walls-60.svg
```

237 tiles, 4266 `<line>` elements, 0 `data-deco` attributes, 0 NaN.

## Decoration styles

`--decoration STYLE` + `--band-w W` (band width in world units, default 0.1)
brings the four Kaplan §3.7 ornamentation styles over the walls:

- **fill** (§3.7.1) — two-colour the wall-map faces (bipartite); needs an
  Eulerian wall map (2- and 4-valent vertices only).
- **outline** (§3.7.2) — thicken each wall to an equidistant band, mitered
  joins at `l = d / sin(θ/2)`, bevel fallback for near-degenerate θ.
- **interlace** (§3.7.3) — filled bands with over/under at every crossing,
  decided by strand-parity alternation (proven translation-invariant by
  torus folds).
- **outline-interlace** (§3.7.4) — outline drawn over the interlaced
  rendering.

`--decoration` **requires `--strapwork` or `--deform`**; the decoration paints paper +
pattern, so **`--mode` is ignored** (verified: identical output with
`--mode single`, `--mode pattern`, `--mode proto`). Gates run on vertices
strictly inside the view (window-rim artefacts are kept outside by the view
region's hairline inset — `region.padding(-1e-9)` — so only genuine openings
fail), and failures are honest errors naming the actual regime
rather than rendering a broken pattern.

Worked example (verified):

```sh
cargo run -p tess-tui -- export --tiling hexagon --width 1200 --height 900 \
    --strapwork 60 --decoration fill --band-w 0.1 --out hexagon-fill-60.svg
```

237 tiles, 1807 faces (1220 lapis `data-deco="face0"` + 587 bone
`data-deco="face1"`), 1 paper rect, 0 NaN.

At generic δ the precondition gates fail loudly (exit 1, no file written):

```sh
cargo run -p tess-tui -- export --tiling square --width 1200 --height 900 \
    --strapwork 30 --decoration fill
# error: decoration 'fill' precondition failed: a ray ends on a host edge
# inside the view (a 2-valent end-cap at (-13.289, -9.000)) — the wall map
# does not close into a face tiling at this angle

cargo run -p tess-tui -- export --tiling snub-square --width 1200 --height 900 \
    --strapwork 60 --decoration interlace
# error: decoration 'interlace' precondition failed: 2300 open wall end(s)
# inside the view (degree-1 exits or ray end-caps, e.g. at (-13.227, -9.178))
# — strands do not close into a weave at this angle; generic contact angles
# give open strands, a classic angle (e.g. 45° on squares, 60° on hexagonal
# hosts) is required
```

Other regimes, also verified at the same scale:

- **fill, odd-degree map** (`hexagram --strapwork 45 --decoration fill`):
  `the wall map has 29792 odd-degree vertex/vertices inside the view (e.g. at (-13.308, -8.771)) — fill needs an Eulerian map (2- and 4-valent vertices only, Kaplan §3.7.1); generic contact angles open the strands, try a classic angle`
- **interlace, multi-strand crossings** (`triangle --strapwork 90 --decoration interlace`): `1829 crossing(s) are not crossed by exactly two strands inside the view (e.g. at (-13.000, -9.815)) — multi-strand crossings have no total over/under order`
- **bad style** (`--decoration weave`): `bad --decoration 'weave': expected fill | outline | interlace | outline-interlace`
- **no `--strapwork`** with `--decoration`: `--decoration requires --strapwork DEG or --deform PROFILE (the decoration decorates the strapwork walls)`
- **bad `--band-w`** (0 or negative): `--band-w must be a positive number`

## Medallion layer (third layer)

`--medallion-weave` (requires `--strapwork DEG`, mutually exclusive with
`--deform`) fuses the tiling's **medallion record** into the wall map:
the tile seeds that form the medallion's hull footprint stop firing
generic contact rays, and the record's exact figure chords take over at
the same edge midpoints; the chords fuse through the same 1e-6 dedup
grid, split at every crossing and T-junction with the generic walls
(design doc [medallion-third-layer-design.md](medallion-third-layer-design.md)).

- A tiling without a medallion layer renders **byte-identically** to the
  two-layer pipeline; flag off changes nothing anywhere.
- When the weave fails with an open end lying on the layer, the error
  **names the record** (the honesty case):
  `… — the witness lies on the medallion layer 'M172 {12/4} dodecagon ⊕ {6/2} hexagram' (declared absorption LandingIdentity): the weave at this contact angle does not absorb the record (law (L) requires theta_record = delta)`
- One composition currently carries a layer: `dodecagon-hexagram-honeycomb`
  (dodecagon `{12/4}` edge-midpoint chords + hexagram `{6/2}` tip
  triangles; seeds 0 and 1 suppressed).
- **Coincidence regime** (verified, anchor A2): where a record chord class
  coincides with the generic firing at the host's classic angle, the fused
  map equals the full generic map exactly (sorted segment-multiset
  equality on the dedup grid, all four styles pass): 3.12.12 @ 60° with
  `{12/4}` chords, 4.8.8 @ 45° with `{8/2}` chords.
- The M172 composition has **no δ closing the fused weave** — the dodecagon
  chords need θ = 60° while the star chords need 90° — so `fill` fails on
  odd degree, `interlace` and `outline-interlace` fail with the named error
  above, and `outline` renders (design doc §7 A3 blocker; tests pin the
  counts).

## Classic-δ table

The 12 even-degree rows that pass all four styles' gates — verified at
1200×900, scale 1.0 (✓ = gates pass and the SVG is written). At generic δ the
same substrates fail (e.g. square @ 30°, hexagram @ 45°).

| Substrate | δ° | fill | outline | interlace | outline-interlace |
|---|---|:---:|:---:|:---:|:---:|
| square | 45 | ✓ | ✓ | ✓ | ✓ |
| square | 90 | ✓ | ✓ | ✓ | ✓ |
| hexagon | 30 | ✓ | ✓ | ✓ | ✓ |
| hexagon | 60 | ✓ | ✓ | ✓ | ✓ |
| triangle | 30 | ✓ | ✓ | ✓ | ✓ |
| triangle | 60 | ✓ | ✓ | ✓ | ✓ |
| trihexagonal | 60 | ✓ | ✓ | ✓ | ✓ |
| pic-3-6-3-6 | 60 | ✓ | ✓ | ✓ | ✓ |
| truncated-hexagonal | 60 | ✓ | ✓ | ✓ | ✓ |
| truncated-square | 45 | ✓ | ✓ | ✓ | ✓ |
| snub-trihexagonal | 60 | ✓ | ✓ | ✓ | ✓ |
| hexagram | 60 | ✓ | ✓ | ✓ | ✓ |

The three registered-module anchors — truncated-square @ 45°
(pic-4-8-8), trihexagonal @ 60° (pic-3-6-3-6), truncated-hexagonal @ 60°
(pic-3-12-12) — reproduce the registered PIC partitions exactly (wall
segments ≡ internal edges, verified by the `strapwork_layer_mode` anchors).

## Known gap (code, not docs) — fixed

At an earlier HEAD, `outline` / `interlace` / `outline-interlace` exported
successfully but wrote SVG path data containing `NaN` components in the
band/side-line rings: at a straight-through strand vertex (anti-parallel
legs, θ = π), `join_geometry` computed θ via `acos` — which loses
precision near −1 (measured θ = π − 2.1e-8 here, too far from the 1e-12
no-op tolerance) and fell through to the exact-mitre branch, whose two
parallel legs made `isect` divide by ~0 (0/0 → `NaN`). The fix detects the
straight-through case geometrically (cross product vanishes, θ ≈ π) and
emits the plain offset feet; `deco_d` and the `<line data-deco="over">`
path also drop non-finite coordinates defensively. The 12×4 classic-δ
matrix and the `decoration_finiteness_battery` test assert zero
non-finite coordinates. `fill` and the plain-walls render were always
clean.
