# Isohedral types family

The parameterized isohedral (IH) types — Grünbaum–Shephard's IH1–IH93
monohedral tiling types realized as incidence symbols (tile symbol +
adjacency symbol) over their topologies, with shaped edges as exact
per-orbit polylines so tiles stay polygons and the coverage contracts
hold. The "Escher arc" (PROPOSED.md §13); design doc:
[docs/isohedral-generator-design.md](../isohedral-generator-design.md).

Code: `crates/tess-tilings/src/families/isohedral/` — cargo feature
`isohedral` (default-on).
Adding a tiling here touches only this family: `families/isohedral.rs`
(module decl, register slice, classification arms, census `IDS`,
`lattice_tilings` entries) plus the new module and its doc row below.
Work briefs: `docs/briefs/TEMPLATE.md`; fleet protocol: `tessellix/AGENTS.md`.

## Implemented (5 tilings)

| id | IH type | topology | group | notes |
|---|---|---|---|---|
| `ih1-p1-hexagon` | IH1 | 6³ honeycomb (dual: triangular) | p1 | unconstrained hexagon; 3 freeform (J) edge orbits; 4 shape params; 1 aspect |
| `ih6-pgg-hexagon` | IH6 | 6³ honeycomb (dual: triangular) | pgg | 2 rotation-paired (U) + 2 freeform (J) orbits; 5 shape params; 4 aspects |
| `ih7-p3-hexagon` | IH7 | 6³ honeycomb (dual: triangular) | p3 | 3 freeform (J) orbits with showpiece curves; record in Q(√3); 2 shape params; 3 aspects |
| `ih27-pgg-pentagon` | IH27 | Cairo V3.3.4.3.4 (dual: snub square) | pgg | pentagon; 1 U + 2 J orbits; 3 shape params; 4 aspects; secondary PentagonTypes |
| `ih29-p4g-pentagon` | IH29 | Cairo V3.3.4.3.4 (dual: snub square) | p4g | pentagon; Klein-paired base orbit (straight edge) + J orbit; 1 shape param; 4 aspects; secondary PentagonTypes |

All five are 1-isohedral realizations pinned at exact rational (or
`Q(√3)`, IH7) parameter values; the engine also accepts arbitrary float
parameters on the documented 2⁻²⁰ quantization policy (design doc §4).
The pentagon templates sit on the Cairo net, mirroring the
`cairo-pentagonal` classification precedent (primary Laves, secondary
PentagonTypes); the family never touches the Wallpaper tab.

## Per-tiling documents

- [`ih1-p1-hexagon`](../tilings/ih1-p1-hexagon.md) — IH1, p1, generic hexagon on the 6³ net
- [`ih6-pgg-hexagon`](../tilings/ih6-pgg-hexagon.md) — IH6, pgg, U/J hexagon on the 6³ net
- [`ih7-p3-hexagon`](../tilings/ih7-p3-hexagon.md) — IH7, p3, showpiece hexagon in Q(√3)
- [`ih27-pgg-pentagon`](../tilings/ih27-pgg-pentagon.md) — IH27, pgg, pentagon on the Cairo net
- [`ih29-p4g-pentagon`](../tilings/ih29-p4g-pentagon.md) — IH29, p4g, pentagon with a straight base edge

## Build arcs

No numbered build arcs yet — the arc's technique story (incidence
symbols as engine data, the exact `Q(√3)` field, the single-rounding
bit-identity policy, the float-parameter quantization contract) is
told in the design doc
[docs/isohedral-generator-design.md](../isohedral-generator-design.md),
whose §9/§9.1 carry the per-spike and per-instance verification
evidence.