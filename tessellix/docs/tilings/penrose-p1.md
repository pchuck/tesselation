# Penrose P1 (pentagon, star, boat, diamond)

`penrose-p1` · family [Quasiperiodic](../families/quasiperiodic.md) (`quasiperiodic`)

quasiperiodic 4-prototile φ²-deflation with short-edge merges across parent boundaries; 5-fold local symmetry

## Algorithm

**Penrose P1** (`penrose_p1.rs`) — the classic four-prototile set
  (pentagon, 10-point star, boat, diamond) by exact φ² deflation: each
  tile is disassembled into short-edge (1/φ) pieces — pentagon flower
  gnomons, boat cores, 1/2-tip stars — that merge *across* parent
  boundaries by four rules (G+G→D, CORE+2G→B, S2C+2G→S, S1C+G→S);
  unmatched short edges are discarded, which makes the single-seed
  patch a pentagonal gasket with five 5-fold notch channels opening at
  the boundary (the clean interior disk of radius R/φ² is tiled
  exactly; views are scaled so the query box fits inside it).

## See also

- Family doc (catalog table + build arcs): [docs/families/quasiperiodic.md](../families/quasiperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling penrose-p1 --mode proto --out penrose-p1.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
