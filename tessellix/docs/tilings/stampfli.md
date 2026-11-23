# Stampfli (12-fold)

`stampfli` · family [Quasiperiodic](../families/quasiperiodic.md) (`quasiperiodic`)

quasiperiodic square+triangle+30°-rhombus by substitution (inflation 2+√3); exact dodecagonal seed star, local 12-fold symmetry

## Algorithm

**Stampfli 12-fold** (`stampfli.rs`) — squares, equilateral triangles
  and 30° rhombi under Stampfli's substitution (inflation λ = 2+√3,
  rule from the Bielefeld Tilings Encyclopedia): square → 1 square +
  12 rhombi + 20 triangles, rhombus → 2 squares + 3 rhombi + 12
  triangles, triangle → 6 rhombi + 10 triangles (unit children per
  parent, edge partition 1 : √3/2 : √3/2 : 1). Some children straddle
  parent edges with exactly one vertex outside; neighbouring parents
  generate the same straddler, so pieces are deduped by their exact
  vertex set. Seeds with the 12-rhombus star at scale λ^n and deflates
  to unit pieces with per-step culling; all anchors stay exact in
  Z[√3]/4, giving edge-to-edge, T-junction-free patches with exact
  12-fold symmetry about the seed at every generation.

## See also

- Family doc (catalog table + build arcs): [docs/families/quasiperiodic.md](../families/quasiperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling stampfli --mode proto --out stampfli.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
