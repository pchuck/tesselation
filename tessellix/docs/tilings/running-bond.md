# Running bond (2:1 dominoes)

`running-bond` · family [Wallpaper](../families/pavers.md) (`pavers`) · wallpaper group `cmm` · centered lattice

paver bond: stretcher rows of 2:1 dominoes, each row offset by half a brick; long sides split into unit edges

> Color-fill rule: the pattern seed alternates with the lattice row index j (`SeedPattern::Rows`), so the default Pattern fill colors by row parity — alternate courses get alternate fills — not by tile orientation; every domino is a horizontal stretcher (a single orientation class), so all tiles fall in the 0° bucket of the Orientation mode.

## See also

- Family doc (catalog table + build arcs): [docs/families/pavers.md](../families/pavers.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling running-bond --mode proto --out running-bond.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
