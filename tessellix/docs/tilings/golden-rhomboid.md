# Golden rhomboid-triangle (Andritz φ-substitution)

`golden-rhomboid` · family [Quasiperiodic](../families/quasiperiodic.md) (`quasiperiodic`)

quasiperiodic: equilateral triangles side 1 (Y) and side φ (O) plus the 60/120 rhombus 1×φ (R), the Andritz family's third member (Bielefeld "golden-rhomboid-triangle", Andritz 5-fold cluster); rules Y → O, O → O+Y+R, R → R+2O (areas 1 : φ² : 2φ in √3/4 units, all identities exact); count matrix [[1,1,0],[2,1,1],[0,1,0]] in (R,O,Y) order, char poly λ³−2λ²−2λ+1, PF eigenvalue φ² pinned as an exact Q(√5) root (φ² = (1,1)); exact 4-tuple Q(√5) arithmetic end to end (x = (a₀+a₁φ)/2, y = (b₀+b₁φ)·√3/2, φ·(a₀,a₁) = (a₁,a₀+a₁); floats only at emission); 18 anchored orientation classes collapsing to 10 unanchored sets under the authorized 120°-rotation quotient (SHAPES/RULES stay anchored); single-seed unbounded growth — depth d covers φ^d·R₀ exactly, union = the 60° wedge {y ≥ 0, y ≤ √3·x} (views outside the wedge are background), depth cap 12 (N₁₂ = 134 209 ≤ 250 000); T-junctions intrinsic to the substitution (first at depth 2: 2 points / 5 incidences), every T-point closing to exactly π on both sides in π/3 units — a gap-free/overlap-free partition, not face-to-edge; depth count vectors and 18/18 exact rule dissections pinned (M144)

## See also

- Family doc (catalog table + build arcs): [docs/families/quasiperiodic.md](../families/quasiperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling golden-rhomboid --mode proto --out golden-rhomboid.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
