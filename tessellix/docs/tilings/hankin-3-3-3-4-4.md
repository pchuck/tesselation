# Hankin elongated-triangular motif (3.3.3.4.4)

`hankin-3-3-3-4-4` · family [Islamic](../families/islamic.md) (`islamic`) · wallpaper group `cmm` · centered lattice

Hankin greedy-inference motif on the elongated-triangular (3.3.3.4.4) substrate (θK = 60°, δ = 0; the down-triangle instance must run CCW — a CW tile emits outward rays and pairs with nothing): both triangle tiles in the side-1/2 4-subdivision limit (four equilateral triangles each), the square as four corner quadrilaterals + central octagon — 13 faces per cell ⟨(1,0),(1/2,1+√3/2)⟩ (det 1+√3/2), exact Q(√3); θK = 60° chosen over 30° (30° slivers), 45°/67.5° (sharper 45° tips), and 90° (triangle pairing degenerates, square star collapses); cmm verified exactly on a 551-face window: translations, 2-fold orbits {C+L} ∪ {C+a1+L} ∪ {(1/2,H+1/2)+L} ∪ {(±1/4,H/2)+L} ∪ {(1/4,H/2+1+H)}, mirrors x=k/2 and y=−1/2+k(1+H), glides y=H/2+k(1+H) shift a1/2 (M107)

## See also

- Family doc (catalog table + build arcs): [docs/families/islamic.md](../families/islamic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling hankin-3-3-3-4-4 --mode proto --out hankin-3-3-3-4-4.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
