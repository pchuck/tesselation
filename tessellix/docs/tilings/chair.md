# Chair (L-tromino)

`chair` · family [Nonperiodic](../families/reptiles.md) (`reptiles`)

aperiodic limit-periodic λ=2 substitution; tiles in 4 orientations; non-edge-to-edge

## Algorithm

**Chair** (`chair.rs`) — the L-tromino substitution tiling (λ = 2):
  each chair (2×2 squares minus one quadrant) splits into 4 half-size
  chairs. An exhaustive enumeration shows the inflated chair has
  exactly ONE L-tromino dissection, so the rule is forced; it is
  inherently non-edge-to-edge (a child vertex lands inside a sibling's
  edge at every scale — asserted by tests, like the pinwheel's
  T-junctions), and the tiling is limit-periodic. Generation is exact
  integer arithmetic: 4×3 seed blocks (one chair per orientation) on an
  origin-anchored lattice, deflated a power-of-two-octave number of
  steps so tiles always land at unit scale; panning/zooming within an
  octave only re-culls a fixed global tiling. "Pattern" mode colors by
  orientation (the four orientation classes are equally numerous at
  every generation).

## See also

- Family doc (catalog table + build arcs): [docs/families/reptiles.md](../families/reptiles.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling chair --mode proto --out chair.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
