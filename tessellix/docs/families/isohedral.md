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

## Implemented (0 tilings)

| id | IH type | topology | group | notes |
|---|---|---|---|---|

## Per-tiling documents

## Build arcs