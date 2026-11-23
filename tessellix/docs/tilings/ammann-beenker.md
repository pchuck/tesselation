# Ammann-Beenker (A5)

`ammann-beenker` · family [Quasiperiodic](../families/quasiperiodic.md) (`quasiperiodic`)

quasiperiodic square+45°-rhombus by substitution; local 8-fold symmetry

## Algorithm

**Ammann-Beenker A5** (`ammann_beenker.rs`) — squares and 45° rhombi
  under the silver-mean substitution λ = 1+√2 (square → 3 squares + 4
  rhombi, rhombus → 2 squares + 3 rhombi). Works on half-squares: the
  two same-shaped `RSquare`/`LSquare` labels carry the matching
  information (every interior square diagonal joins an R and an L
  half). Seeds with the 8-rhombus star (inradius √2/2·scale), deflates
  with per-step culling exactly like Penrose, and pairs halves back into
  squares on shared hypotenuses; every child vertex lies in Z[√2].

## See also

- Family doc (catalog table + build arcs): [docs/families/quasiperiodic.md](../families/quasiperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling ammann-beenker --mode proto --out ammann-beenker.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
