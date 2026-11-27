# Stepped hexominoes

`stepped-hexominoes` · family [Wallpaper](../families/pavers.md) (`pavers`) · wallpaper group `p1` · centered lattice

paver tiling: asymmetric 2-step staircase hexomino, one tile per cell on the centered rectangular lattice ⟨(3,1),(0,2)⟩ (primitive cell; centered cell 6×2 with center (−3,1) in the lattice), edge-to-edge with 180° at the four collinear corners; the catalog's first p1 realizer — the centered-rectangular normalizer {I, R180, Mx, My} is broken by the asymmetric staircase

The card's `centered` lattice token is the measured kind of the
translation lattice ⟨(3,1),(0,2)⟩ (centered cell 6×2, center (−3,1) ∈ Λ).
The Wallpaper tab files the card under its **oblique** heading: tab
sections use the *conventional* lattice of the declared group
(`WallpaperGroup::conventional_lattice`: p1 → oblique), a display
convention that lays out the 17 groups as textbooks do, not a second
measurement of this lattice.

## See also

- Family doc (catalog table + build arcs): [docs/families/pavers.md](../families/pavers.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling stepped-hexominoes --mode proto --out stepped-hexominoes.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
