# Fibonacci × Fibonacci (product substitution)

`fibonacci-fibonacci` · family [Quasiperiodic](../families/quasiperiodic.md) (`quasiperiodic`)

quasiperiodic Bielefeld "fibonacci-fibonacci" entry: the 1-D Fibonacci substitution taken as a product with itself — 4 orientation classes (LL, LS, SL, SS) on 3 prototile shapes (LS/SL one shape at two rotations); linear inflation λ = τ² = φ², module rule = two product steps (children tile the INFLATED parent, anchors scaled by λ); count matrix B = A₄² = [[4,2,2,1],[2,2,1,1],[2,1,2,1],[1,1,1,1]], the square of the product matrix A₄ = [[1,1,1,1],[1,0,1,0],[1,1,0,0],[1,0,0,0]] pinned from the entry's rule figure; char poly (x−1)²(x²−7x+1), PF eigenvalue τ⁴ = λ², per-parent area law child = parent·τ⁴ exact; strict PIP + proper-crossing + T-junction checks at every growth step; exact Q(√5) integer pairs end to end, floats only at emission; finite patch recentered −φ⁷/2 (depth-3 floor origin-symmetric), depth cap 6 (totals F_{2d+2}²: 1, 9, 64, 441, 3025, 20 736, 142 129 ≤ MAX_TILES 250 000); 4-fold point symmetry; diffraction cited, not proven (golden-family convention) (M154)

## See also

- Family doc (catalog table + build arcs): [docs/families/quasiperiodic.md](../families/quasiperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling fibonacci-fibonacci --mode proto --out fibonacci-fibonacci.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
