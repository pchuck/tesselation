# Hankin motif hex weave (6³)

`hankin-hex-motifs` · family [Islamic](../families/islamic.md) (`islamic`) · wallpaper group `pmm` · rectangular lattice

Kaplan §3.5 design-element motifs (`hankin_motifs::build_cells_with_motifs`) on the regular hexagonal (6³) substrate in a rectangular 2-hex cell: the **rosetted hexagon** (ideal θ = π/6 = 30°, h = 1, φ = 0, depth s = 2: 24 segments, 6 hexagonal petals + one 24-gon core) at the origin alternates with the **extended star** (realized outer contact angle π/3 = 60°, depth s = 1: child element (6, r·F, s−1, π/6) rotated π/6 with F = cos 60°/cos(30° − 60°); 12 spurs — a star's whole arm is its leg line — giving 6 quads + one 12-gon core) at (√3/2, 3/2) — 14 faces per ⟨(√3,0),(0,3)⟩ cell (|det| = 3√3), Σ areas = |det| exact.

## Specification

- **Substrate.** Flat-top unit hexagons (vertices at 30° + 60k, edge 1): class A at the origin, class B at (√3/2, 3/2), sharing the edge between (0,1) and (√3/2, 1/2) exactly; the 2-hex rectangular cell (det 3√3) is the primitive translation cell of the motif pattern (the primitive hexagonal cell cannot carry two motif classes).
- **Symmetry breaking.** The class assignment is deliberately 2-coloured: a 60° rotation about hexagon A maps A onto a class-B centre, swapping rosettes and extended stars, so it is **not** a symmetry — the pattern group drops from the substrate's p6m to pmm.
- **Group pmm**, verified exactly on the torus segment multiset of a ±2-cell window: mirrors x = 0, y = 0, x = √3/2, y = 3/2 and rot180 about the hexagon centre and about the shared-edge midpoint hold; rot60/rot120 about both centres, the 30° mirror, rot90 and the half-diagonal translation all fail.

## Verification

- Instance tests: coverage / rotated-scaled / deterministic trio; full 192-float cell-vertex table machine-transcribed from a debug build (matched to 1e-9; the Python battery reference agrees to 4.5e-16); seed layout, per-cell vertex counts, face lens census {(6, 6), (24, 1), (4, 6), (12, 1)}, motif segment counts (24, 12), area sum vs |det|; symmetry metadata.
- The shipped hexagon tables use exact literals (√3/2, 1/2, 0, 1) rather than trig evaluations, so shared vertices and edge midpoints are bit-exact; the motif is derived from the tile, so contacts and landing points stay exact under this representation.

## Sources

- Kaplan (2111), *Beyond the Islamic Geometric Standard*, §3.5 (design elements), §3.5.4 (extension operator; child angle θ′ = θ − π/n, erratum against the printed θ/2).

## See also

- Family doc (catalog table + build arcs): [docs/families/islamic.md](../families/islamic.md)
- Same substrate, greedy inference: [`hankin-hex-two-point`](hankin-hex-two-point.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling hankin-hex-motifs --mode proto --out hankin-hex-motifs.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
