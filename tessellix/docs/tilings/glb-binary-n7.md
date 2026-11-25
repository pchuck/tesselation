# GLB binary, generalized (n = 7)

`glb-binary-n7` · family [Quasiperiodic](../families/quasiperiodic.md) (`quasiperiodic`)

Pautze's generalized binary family at n = 7 (Hibma 2015 / Pautze 2017), the K₇ field layer's first consumer: three 7-fold rhombi (acute angles π/7, 2π/7, 3π/7), inflation λ = √(2+2cos(π/7)) = 2cos(π/14) ≈ 1.9499, non-Pisot; per-parent refinement is a two-choice pool (shape 1 → {1} — rule 6 has no certified survivor frame, 2 → {5,2}, 3 → {3,4}) forced by compatible meshing at depth ≥ 2 — eps-complementarity cascades through arbitrarily long adjacency chains, so no local (shape, class) map drives refinement (probed and refuted); certified generator = deterministic per-level DFS, all-pairs interior disjointness + eps-complementary bend bits on shared parent edges (compat ε_A + ε_B = 1), with the unculled parity battery pinning that spatial culling of the refinement frontier is UNSOUND; K₇-twist exact arithmetic — every vertex a twist pair (k₁, s₁k₂) ∈ K₇², K₇ = Q(b), b = 2cos(2π/7), dyadic denominators D = 2 (depth-stable), float emission only, all certificate vertices on the even π/7 class grid (6 488 vertices, 0 odd at the depth-4 certificate); fan-area law: every valid assembly carries total twist area 2λ^{2d} exact; unculled growth 3/10/35/126/462 (M^d row sums, M = [[2,1,0],[1,2,1],[0,1,3]], PF eigenpair λ²·v with v = (1, b²+b−1, b+1)); depth cap 5 = the deepest level constructed AND certified — σ²…σ⁵ pass the full staircase boundary certificate (single boundary cycle of 4·2^d even unit segments, consecutive pairings, closure on φ^{d−1}(B₀), exact area), while at depth 6 the candidate search exhausts: a legal patch need not admit a legal refinement; plane-filling presentation (M152): the parent patch is a lattice of the certified seed V = unit shape-1 rhombus, translated by m·(v₁−v₀)+n·(v₂−v₁) over φ⁻¹(W + C₀) with SAT margin (0.05 float-safety culling), the same certified DFS refined d ∈ [3, 5] levels so cell seams mesh (no finite domain, no empty background); the framing reference for the presentation is the seed big tile B₀ = φ(V) (bbox ≈ 3.43 × 1.65 cell units), with C₀ = bbox center of φ(V) as the recentering origin; tiles unit-edge at every level, so depth sets refinement level only; σ² census per fan pick: 16 two-choice combos, 9 disjoint-only, 4 meshed — disjointness alone NOT sufficient, eps-compatibility load-bearing; finite rotations + polytopal rhomb tiles, Pautze notes no Dₙ-symmetric patches, no pure-point claim (M149)

## See also

- Family doc (catalog table + build arcs): [docs/families/quasiperiodic.md](../families/quasiperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling glb-binary-n7 --mode proto --out glb-binary-n7.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
