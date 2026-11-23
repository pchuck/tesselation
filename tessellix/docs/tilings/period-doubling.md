# Period-doubling (2D product) substitution

`period-doubling` · family [Aperiodic](../families/aperiodic.md) (`aperiodic`)

limit-periodic product substitution: the 1D period-doubling rule a → ab, b → aa (Bielefeld entry "period doubling", Frettlöh–Sing; internal space the 2-adic integers, "in some sense the simplest cut and project tiling") squared as σ⊗σ on Z² unit squares — 4 prototile label classes (aa/ab/ba/bb, class = 2·w[col] + w[row], seeds 0–3) over one square prototile, λ = 2 into a 2×2 frame, child (c,r) label = (σ(x)_c, σ(y)_r); S = [[1,2,2,4],[1,0,2,0],[1,2,0,0],[1,0,0,0]] (column sums all 4 = λ²), PF eigenvalue 4, char poly x⁴−x³−12x²−4x+16 (eigenvalues {4, 1, −2, −2}, exact i64 permutation-expansion determinants); limit-periodic via approximating periodic tilings — the depth-g patch is the fundamental cell of the period-(2^g, 2^g) approximant, measured (2,2) … (64,64) at g = 1..6, no proper within-patch period (the ratio-1 manifest vs domino's ratio-2 (2^g, 2^(g+1))); exact integer arithmetic on Z² (M141)

## See also

- Family doc (catalog table + build arcs): [docs/families/aperiodic.md](../families/aperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling period-doubling --mode proto --out period-doubling.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
