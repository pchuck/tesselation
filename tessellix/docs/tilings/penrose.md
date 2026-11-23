# Penrose P3 (rhombs)

`penrose` · family [Quasiperiodic](../families/quasiperiodic.md) (`quasiperiodic`)

quasiperiodic, 5-fold local symmetry

## Algorithm

**Penrose P3** (`penrose.rs`) — Robinson-triangle substitution
  (`O → 2O + A`, `A → O + A`) with exact φ-arithmetic child placements,
  chirality-aware (left/right-handed parents get mirrored child
  patterns). Grows outwards from the "sun" seed by repeated deflation,
  culling to the region at every step so overlaps are structurally
  impossible; triangles are paired back into thick/thin rhombi at the end.

## See also

- Family doc (catalog table + build arcs): [docs/families/quasiperiodic.md](../families/quasiperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling penrose --mode proto --out penrose.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
