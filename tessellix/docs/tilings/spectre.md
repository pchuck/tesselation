# Spectre (chiral einstein, Tile(1,1))

`spectre` · family [Aperiodic](../families/aperiodic.md) (`aperiodic`)

aperiodic chiral einstein family (SMKG 2023); single equal-edge prototile, aperiodic without reflections, 9-label substitution, exact Q(√3) geometry

## Algorithm

**Spectre** (`spectre.rs`) — the chiral einstein Tile(1,1) (SMKG
  2023, arXiv:2305.17743): a single 14-gon prototile with all unit
  edges and area exactly 3+3√3, aperiodic *without reflections*. Same
  nine-label substitution as the turtle (identical `SUPER_RULES` /
  `T_RULES` tables), but the Mystic compound is two spectres — the
  second rotated 30° about the origin — so ring children occur at
  30°-multiple orientations and every placement is an exact D12
  transform over Q(√3): vertices are stored as surds
  `((a + b√3)/2ⁿ, (c + d√3)/2ⁿ)` and 30° rotation closes on them.
  Odd-depth patches are mirrored into the root placement so every
  emitted tile is unreflected; edge-to-edge, per-view rigid patches
  snapped to a 4-world-unit grid (depths 1–5, full-patch counts
  9/71/559/4401/34649, max inscribed radius ≈90.4), verified
  pose-identical to Kaplan's spectre.js at every depth 0–5 by the
  exact Python surd reference.

## See also

- Family doc (catalog table + build arcs): [docs/families/aperiodic.md](../families/aperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling spectre --mode proto --out spectre.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
