# Half-hex (Socolar, limit-periodic)

`half-hex` · family [Aperiodic](../families/aperiodic.md) (`aperiodic`)

achiral half-hexagon prototile (trapezoid half of a unit hexagon, edges {2,1,1,1}), λ = 2 into 4 children (S[j][i] = 1 iff (j−i) mod 6 ∈ {0,2,3,4}); PF eigenvalue 4 = λ², char poly (x−4)(x−2)(x−1)²(x+1)²; limit-periodic — measured minimal x-period p = (3/2)·2^g = 6, 12, 24, 48, 96 at g = 2..6; exact √3-field (u,v) integer-pair arithmetic

## See also

- Family doc (catalog table + build arcs): [docs/families/aperiodic.md](../families/aperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling half-hex --mode proto --out half-hex.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
