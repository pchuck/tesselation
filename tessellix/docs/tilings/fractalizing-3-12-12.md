# Fractalizing 3.12.12

`fractalizing-3-12-12` · family [Quasiperiodic](../families/quasiperiodic.md) (`quasiperiodic`)

limit-periodic fractalizing family over the uniform 3.12.12 (Chavey, Tilings by regular polygons III): same-plane levels, each shrunk by 2+√3, level ≥ 1 a hexagonal flower orbit of 44 triangles + 12 squares + 1 hexagon + 12 dodecagons per cell (69 tiles, cell 84 + 48.5√3 = the level-0 3.12.12 cell at edge 2+√3, exactly); all level-1 vertices on the exact (m + n√3)/6 grid, edge-to-edge closure verified in exact Q(√3) (0 overlaps, 0 T-junctions, exact single coverage)

## Algorithm

**Fractalizing 3.12.12** (`fractalizing.rs`) — Chavey's
  *fractalizing* dodecagon-dense family (Wikipedia "fractalizing
  k-uniform tilings"): a sequence of edge-to-edge tilings of the same
  plane, level d having child edge p^{1−d} for p = 2+√3, the level-(d+1)
  plane being the level-d plane shrunk by 1/p about the origin flower —
  limit-periodic (a new period per level), hence Quasiperiodic like
  Stampfli, though each individual level is 6-fold periodic. Level 0 is
  the uniform 3.12.12 at edge p; levels ≥ 1 are the hexagonal orbit of
  a 57-tile flower (1 H + 12 S + 44 T) plus 12 dodecagon classes per
  cell (69 tiles, cell 84 + 48.5√3 — exactly the level-0 cell). All
  level-1 vertices lie in the exact (m + n√3)/6 grid, so the tables,
  anchors and dodecagon centers are pure integer Z[√3] until a single
  float multiply; closure was verified in exact Q(√3) (0 overlaps,
  edges ≤ 2, 0 T-junctions, exact single coverage, per-cell census
  forced by the area identity) and is re-checked by an i128 battery in
  the module tests.

## Hierarchy visibility

Under zoom the hierarchy becomes visible via successive substitution levels (fractalizing behavior) — each view refines one substitution octave, so the level ramp reads as a color change as you drill in. In Level color mode (M201) the current depth uniformity is shown directly: every tile in one view carries the same level, so a view reads as a single flat depth bucket. As of M225 the depth is also directly selectable per card via its `depth` parameter (0 = the view-anchored auto rule, 1..=16 pinned deeper substitutions with a named refusal above the tile budget), so the hierarchy can be pinned outright instead of revealed by zooming.

## See also

- Family doc (catalog table + build arcs): [docs/families/quasiperiodic.md](../families/quasiperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling fractalizing-3-12-12 --mode proto --out fractalizing-3-12-12.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
