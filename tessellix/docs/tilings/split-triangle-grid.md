# Split-triangle grid

`split-triangle-grid` · family [Wallpaper](../families/pavers.md) (`pavers`) · wallpaper group `p31m` · hexagonal lattice

paver tiling: side-2 triangular grid in which every small equilateral triangle is split by its orbit triangle (a generic Q(√3) point near a vertex, rotated 120° about the triangle's centroid) into an inner equilateral triangle + 3 corner quads, per hexagonal cell ⟨(2,0),(1,√3)⟩ (det 2√3): 8 tiles, two congruence classes (inner triangles achiral, corner quads chiral); edge-to-edge (750 interior edges, 0 T-junctions); the catalog's first p31m realizer — mirrors exactly on the grid lines (0°/60°/120° families), rot120 centres in three orbits (grid vertices, up/down-triangle centroids) with the on-mirror incidence (grid-vertex orbit on mirrors, both centroid orbits off) that separates p31m from p3m1, no 60°/180°/300° rotations, no glides (M84, HX-STUDY handoff)

## See also

- Family doc (catalog table + build arcs): [docs/families/pavers.md](../families/pavers.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling split-triangle-grid --mode proto --out split-triangle-grid.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
