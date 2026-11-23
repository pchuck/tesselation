# Hirschhorn pentagon

`hirschhorn` · family [Pentagon Types](../families/pentagons.md) (`pentagons`)

nonperiodic monohedral (Hirschhorn–Hunt 1985)

## Algorithm

**Hirschhorn** (`hirschhorn.rs`) — the 140°/60°/160°/80°/100° convex
  pentagon (n=6 of the C11-T1A Sugimoto family). Since the tiling is not
  locally forced, the generator constructs one valid realization by
  constrained 6-fold-symmetric growth: at each step it enumerates every
  legal placement across the boundary (both chiralities, all template
  edges/directions), filters by local rules (no overlap, edge-to-edge,
  angle sums ≤ 360° with no unclosable residual gap, no duplicates),
  groups candidates into complete 60° rotation orbits, and commits the
  first orbit that closes vertices, keeps the patch a topological disk,
  and strands no dead boundary edge. Every patch is C6-symmetric about
  the seed; tile frames land on multiples of 20°.

## See also

- Family doc (catalog table + build arcs): [docs/families/pentagons.md](../families/pentagons.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling hirschhorn --mode proto --out hirschhorn.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
