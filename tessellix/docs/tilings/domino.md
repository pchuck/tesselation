# Domino (table) substitution

`domino` · family [Aperiodic](../families/aperiodic.md) (`aperiodic`)

Lançon's limit-periodic domino substitution (Bielefeld entry "domino" — "Also known as 'table tiling.'", one entry); A = vertical 1×2, B = horizontal 2×1 on Z², λ = 2 orientation-preserving into 4 children; S = [[2,2],[2,2]], PF eigenvalue 4 = λ², char poly x²−4x; limit-periodic via approximating periodic tilings — the depth-g patch is itself the fundamental cell of the period-(2^g, 2^(g+1)) approximant, measured (2,4), (4,8), (8,16), (16,32), (32,64), (64,128) at g = 1..6, no proper within-patch period; not cut-and-project (continuous spectral component — Solomyak 1997, Robinson 1999); exact integer arithmetic on Z² (M138)

## See also

- Family doc (catalog table + build arcs): [docs/families/aperiodic.md](../families/aperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling domino --mode proto --out domino.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
