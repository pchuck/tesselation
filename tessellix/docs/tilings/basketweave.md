# Basketweave (2:1 dominoes)

`basketweave` · family [Wallpaper](../families/pavers.md) (`pavers`) · wallpaper group `p4g` · square lattice

paver bond: 2×2 blocks of two parallel 2:1 dominoes, checkerboard orientation; long sides split into unit edges

> Color-fill rule: the cell seed stores the tile's orientation class (0 = horizontal, 1 = vertical), so the default Pattern fill colors by tile orientation — the axis-aligned 0° and 90° buckets {0,2} in the Orientation mode (M196), never the 45°/135° diagonal buckets.

## See also

- Family doc (catalog table + build arcs): [docs/families/pavers.md](../families/pavers.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling basketweave --mode proto --out basketweave.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
