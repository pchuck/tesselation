# Dodecagon-Hexagram Composition (Honeycomb)

`dodecagon-hexagram-honeycomb` · family [Islamic](../families/islamic.md) (`islamic`) · wallpaper group `p6m` · hexagonal lattice

composition-family medallion cell (24d instance 1): regular dodecagons (circumradius 1/2) at the hexagon centers and kissing {6/2} hexagrams (tip radius 1/2) at the vertices of the honeycomb net, interstices filled edge-to-edge by one reflex-hexagon class — 9 tiles/cell, exact Q(√3)

## Composition

On the honeycomb `[6,3]` net with vertex radius `R = 1` (hexagon edge 1,
centers on the triangular lattice `a1 = (√3, 0)`, `a2 = (√3/2, 3/2)`):

- **dodecagon** (seed 0) — regular, circumradius `1/2`, at every hexagon
  center, vertices at `30k°`. The six vertices at `30° + 60k` are exactly
  the midpoints of the center–vertex segments; the six at `60k` lie on
  the center–center (dual) lines. `edge² = (2 − √3)/4`, area `3/4`.
- **hexagram** (seed 1) — the {6/2} star at the kissing tip radius
  `T = 1/2` (tips at `30° + 60k`, notches at `60° + 60k` at radius
  `√3/6`), emitted as the simple 12-gon in monotone tip–notch order, at
  every net vertex — two per cell at `VA = (0, 1)` and `VB = (√3/2, 1/2)`.
  Area `√3/4`. The tip set is `hexagram_62::tips_natural()` rotated 30°
  (the composition turns the record half a step so the tips face the
  dual-edge midpoints).
- **interstice hexagon** (seed 2) — one reflex hexagon class, six per
  site, bounded by two dodecagon edges and four star edges; its reflex
  corner (210°) wraps exactly around a dodecagon vertex on a dual line.
  Area `√3/6 − 1/8`.

The configuration is uniquely determined by the two contact identities:
every star tip pointing at a hexagon center lands on the dodecagon
vertex at the midpoint of that center–vertex segment, and every star
tip pointing along a net edge lands on the net-edge midpoint, kissing
the neighbor star tip-to-tip. Per cell
`3/4 + 2·(√3/4) + 6·(√3/6 − 1/8) = 3√3/2 = |det(a1, a2)|` exactly.
Interior vertex figures: `60.60.120.120` (kissing points),
`60.75.75.150` (dodecagon-vertex contacts), `120.240` (star notches),
`150.210` (dodecagon vertices on the dual lines).

## Fill reading (24d, reduced scope)

| reading | faces/cell | interstice faces | edge-to-edge | seams | status |
|---|---|---|---|---|---|
| **dual-net fill** | 9 | 6 reflex hexagons | yes | 0 | **shipped** |
| hole-inference (24b) | — | — | — | — | deferred |

The honeycomb circumcentric dual (triangles at the net vertices, edge
spacing √3) passes **vertex-to-vertex** through the shipped partition:
its edges run along the center–center lines through the dodecagon `60k`
vertices and the kissing tip points (net-edge midpoints = dual-edge
midpoints), cutting no tile interior; each interstice hexagon has two
opposite vertices on one dual line. The dual-net reading therefore reads
the fill directly off the dual: the glue sites for assembly are the
hexagon long diagonals along the dual lines. The hole-inference reading
(inferring the interstices from medallion contact figures, per 24b) is
deferred — its boundary documentation is this section plus the module
header; `families/kuniform/dual.rs` was read only, not modified.

## Verification

Python battery (stdlib, ±3×±3 patch, 441 tiles): Σ interior face areas
== outer boundary area exactly; 1628 interior edges shared by exactly 2
tiles, none by more (0 T-junctions); vertex angle sums all 360° in the
four orbits above; irrational-offset fine grid (5029 pts, r < 2)
covered exactly once; contact battery 882/882 midpoint contacts and
294/294 dual-edge carry checks; group probes: rot60 and mirror-x fix
31/31 tile orbits, rot30/rot90/mirror-15 reject 30/31 — p6m.

Rust in-cell battery (exact `Q(√3)` via the medallion `Zq` engine):
dodecagon regularity (R² = 1/4, edge² = (2−√3)/4, twelve 30° turns)
with the similarity invariant `edge²/R²` equal to the
`rosette_m56::dodecagon` record's; star tip/notch radii, all 24 star
edge² = 1/12, notch chord identity `2(mid − c) = 3(notch − c)`, and tip
set == `hexagram_62::tips_natural()` rotated 30° at both centers; the
contact/kiss identities as exact point equalities; per-hexagon edge
lengths (2 dodecagon + 4 star); the 36 interstice edges landing on the
12 dodecagon + 24 star boundary edges exactly once each modulo the
lattice; cell area closure `Σ area2 = 3√3`. All vertices are emitted as
`(p + r√3, q + s√3)/12` integer tuples, so shared vertices are
bit-identical. Standard trio (coverage, rotated-scaled, deterministic)
plus the lattice-culling guard via the family `lattice_tilings()` entry.

## See also

- Family doc (catalog table + build arcs): [docs/families/islamic.md](../families/islamic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling dodecagon-hexagram-honeycomb --mode proto --out dodecagon-hexagram-honeycomb.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
