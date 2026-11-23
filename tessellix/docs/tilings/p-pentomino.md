# P-pentomino (rep-4 reptile, limit-periodic)

`p-pentomino` · family [Nonperiodic](../families/reptiles.md) (`reptiles`)

aperiodic limit-periodic λ=2 rep-4 substitution; tiles in 8 chiral orientation classes; non-edge-to-edge

## Algorithm

**P-pentomino** (`p_pentomino.rs`) — the P-pentomino rep-tile (five
  squares; 2×2 block plus one extra), λ = 2 rep-4. Because the P is
  chiral, its dissection mixes chiralities: exhaustive enumeration finds
  exactly two dissections of the inflated P (class multisets {0,1,4,6}
  and {3,4,4,5}); the canonical one is used, and the 8 orientation
  classes (4 rotations × 2 chiralities) stay equally numerous (the
  substitution matrix is doubly stochastic). Inherently
  non-edge-to-edge (five T-junctions per parent at every scale).
  Generation is exact integer arithmetic: 5×8 seed blocks (one P per
  orientation class — 62 such tilings of the rectangle exist, this one
  is class-complete) on an origin-anchored lattice, deflated a
  power-of-two-octave number of steps so tiles always land at unit
  scale (area 5); "pattern" mode colors by orientation class.

## See also

- Family doc (catalog table + build arcs): [docs/families/reptiles.md](../families/reptiles.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling p-pentomino --mode proto --out p-pentomino.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
