# Andritz 10gon-1 (quasiperiodic)

`andritz-bowtie-hexagon-10gon-1` · family [Islamic](../families/islamic.md) (`islamic`)

quasiperiodic Andritz 2016 fivefold substitution 10gon-1 (Tilings Encyclopedia, Feb 2016): three unit-edge girih prototiles (bow-tie, elongated hexagon, regular decagon), inflation φ², substitution matrix [[2,3,0],[2,2,1],[5,0,6]], PF = φ⁴; exact ℤ[ζ₂₀] σ-frame walk — the rule was derived from the girih edge graph of the decoded Bielefeld reference patch (edge-BFS over ℤ[ζ₂₀], 3280/3280 shared-edge label agreement, zero conflicts) and cross-checked by per-instance tuple equality of every slot offset (26/26 bow-tie rows, 10/10 hexagon rows, 16/16 decagon flowers, 22/22 identical taus); the frame rotates with the parent (ψ_B(r) = 2r+3+10((r+1)%2), ψ_H(r) = 2r+8+10(r%2)), the decagon row is absolute (center + five edge-sharing petals at 2·apothem + five gap bow-ties), decagon children carry no rotation; edge-to-edge at every level (M152)

## See also

- Family doc (catalog table + build arcs): [docs/families/islamic.md](../families/islamic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling andritz-bowtie-hexagon-10gon-1 --mode proto --out andritz-bowtie-hexagon-10gon-1.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
