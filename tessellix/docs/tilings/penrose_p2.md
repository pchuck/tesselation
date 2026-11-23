# Penrose P2 (kite and dart)

`penrose_p2` · family [Quasiperiodic](../families/quasiperiodic.md) (`quasiperiodic`)

quasiperiodic kite-dart tiling by half-tile substitution; local 5-fold symmetry

## Algorithm

**Penrose P2** (`penrose_p2.rs`) — the kite-and-dart tiling, built on
  *glue-labelled half-tiles*: kite halves `A` (36-72-72) and dart halves
  `O` (36-108-36) substitute by `A → 2A + O`, `O → A + O` and pair along
  their glue legs into kites (two `A`) and darts (two `O`) — half-darts
  pair across parent boundaries, which is why growth seeds with the
  sun/star patches (single tiles do not deflate consistently, the
  classical P2 caveat). The child placements are the unique globally
  consistent labelling for this half-tile frame (found by an exhaustive
  8192-combination search over frame/glue assignments, pruned by
  sun/star deflation checks); verified in exact arithmetic over
  Q(φ, sin 36°) and re-checked by Rust tests (census law
  `K′ = 2K + D, D′ = K + D`, edge-to-edge merged patches, kite/dart
  side patterns). Sun or star seed selectable via a `star seed` param.

## See also

- Family doc (catalog table + build arcs): [docs/families/quasiperiodic.md](../families/quasiperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling penrose_p2 --mode proto --out penrose_p2.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
