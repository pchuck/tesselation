# Stack bond (2:1 dominoes)

`stack-bond` · family [Wallpaper](../families/pavers.md) (`pavers`) · wallpaper group `pmm` · rectangular lattice

paver bond: rows of 2:1 dominoes stacked directly on top of one another — the running bond without its half-brick offset; one split domino per cell on the rectangular lattice ⟨(2,0),(0,1)⟩, long sides split into unit edges (180° vertex figures at the split midpoints) (M114)

> Color-fill rule: the pattern seed alternates with the lattice row index j (`SeedPattern::Rows`, M200), so the default Pattern fill colors by row parity — alternate courses get alternate fills — not by tile orientation; every domino is a horizontal stretcher (a single orientation class), so all tiles fall in the 0° bucket of the Orientation mode.

## See also

- Family doc (catalog table + build arcs): [docs/families/pavers.md](../families/pavers.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling stack-bond --mode proto --out stack-bond.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
