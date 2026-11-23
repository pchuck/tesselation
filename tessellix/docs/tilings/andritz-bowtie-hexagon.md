# Andritz Bowtie-and-Hexagon (quasiperiodic)

`andritz-bowtie-hexagon` · family [Islamic](../families/islamic.md) (`islamic`)

quasiperiodic Andritz 2016 fivefold substitution (Tilings Encyclopedia, Feb 2016): two unit-edge girih prototiles (bow-tie + elongated hexagon), inflation φ², substitution matrix [[2,3],[3,5]], PF = φ⁴; exact ℤ[ζ₅] σ-frame walk from a 27-tile seed core — reproduces the decoded Bielefeld reference patch pose-for-pose (646 tiles, 1383 ring-exact vertices, exact 253-pose inner patch matched as sets both directions), level growth 27→168→1160, edge census: every interior edge shared by exactly 2 tiles out to radius 8 (level 1) / 20 (level 2); region-filtered wavefront BFS via the inverse-image σ⁻¹ box chain, work linear in view area; edge-to-edge at every level (M151)

## See also

- Family doc (catalog table + build arcs): [docs/families/islamic.md](../families/islamic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling andritz-bowtie-hexagon --mode proto --out andritz-bowtie-hexagon.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
