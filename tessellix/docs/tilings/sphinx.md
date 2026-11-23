# Sphinx (pentagonal hexiamond)

`sphinx` · family [Nonperiodic](../families/reptiles.md) (`reptiles`)

aperiodic limit-periodic λ=2 rep-4 substitution; tiles in 12 orientations; non-edge-to-edge

## Algorithm

**Sphinx** (`sphinx.rs`) — the pentagonal hexiamond rep-tile (six
  unit triangles of the triangular lattice, five boundary sides; λ = 2
  rep-4). Exact arithmetic derives everything: of the 12 free
  hexiamonds only the 3×1 rhombus and the sphinx are rep-4 (the sphinx
  the only pentagonal one), and the inflated sphinx has exactly ONE
  dissection, so the rule is forced. The dissection mixes chiralities,
  so tiles come in all 12 lattice-orientation classes (substitution
  matrix doubly stochastic: rows and columns sum to 4). Inherently
  non-edge-to-edge like the chair. Generation is exact integer
  arithmetic on the triangular lattice: the 2×3 lattice rhombus (the
  unique smallest rhombic patch — exactly two sphinxes, found by an
  exhaustive cover search) seeds origin-anchored blocks deflated a
  power-of-two-octave number of steps, so tiles always land at unit
  scale (area 3√3/2); "pattern" mode colors by orientation class.

## See also

- Family doc (catalog table + build arcs): [docs/families/reptiles.md](../families/reptiles.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling sphinx --mode proto --out sphinx.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
