# Ammann A4 (quasiperiodic)

`ammann-a4` · family [Quasiperiodic](../families/quasiperiodic.md) (`quasiperiodic`)

quasiperiodic rectilinear chair set (Grünbaum–Shephard, *Tilings and Patterns*, 1987): a chair (a square of side S_c = 2+√2 with two opposite unit corner notches) and a half-chair (a S_c × λ rectangle with one √2 × 1 corner notch, tall or wide), inflated by λ = 1+√2 (silver mean, same as Ammann–Beenker A5); 10 oriented states in an exact Z[√2] rule table pinned by an independent Python cert (chair rule D3b-achiral, per-rule diag covariance, exact λ² area laws); non-edge-to-edge (T-junctions intrinsic: 8 per chair rule, 5 per HC rule, 56 at level 2); 8-fold symmetric model set (rectilinear D4 tiles) cited, not proven (mld Ammann–Beenker per the Bielefeld encyclopedia); Bielefeld patch N_chair ≈ 2014 : N_HC ≈ 2798 (ratio 1.3891 vs the Perron ratio √2 — 1.78 % finite-size deviation, not a rule error)

## See also

- Family doc (catalog table + build arcs): [docs/families/quasiperiodic.md](../families/quasiperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling ammann-a4 --mode proto --out ammann-a4.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
