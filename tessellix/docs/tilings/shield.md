# Gähler shield

`shield` · family [Quasiperiodic](../families/quasiperiodic.md) (`quasiperiodic`)

quasiperiodic shield-hexagon+square+triangle substitution (Gähler; inflation √(2+√3), kinked supertiles, 4 protos / 8 labelled kinds); dodecagonal frame, Bragg 12-fold, no local 30° center

## Algorithm

**Gähler shield** (`shield.rs`) — the dodecagonal shield tiling
  (Gähler 1988): shield hexagon, square and two unit triangles, each in
  two labelled orientations R/L (the labels carry the matching rules,
  so 8 substitution kinds collapse to 4 output prototiles). Inflation
  θ = 2cos 15° = (√6+√2)/2, Perron eigenvalue of the 8×8 count matrix
  θ² = 2+√3. Children tile a *kinked* supertile (±¼-unit-square
  triangular notches; shield 6 in, square 4 in, black tri 3 out, white
  tri 2 out + 1 in), verified per kind in exact `Q(√2,√3)` arithmetic.
  Seeds with the 13-tile kinked-shield supertile (area centroid exact at
  √2/2 + √6/3, covering a disk of inradius (1+√3)/2 per scale) and
  deflates with per-step culling. All edges lie on the 12 dodecagonal
  directions 15°·k (matching window: regular dodecagon); the 12-fold
  character is Bragg/MLD — no local 30° rotation center exists (every
  tile angle ≥ 60°).

## See also

- Family doc (catalog table + build arcs): [docs/families/quasiperiodic.md](../families/quasiperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling shield --mode proto --out shield.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
