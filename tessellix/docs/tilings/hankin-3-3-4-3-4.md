# Hankin snub-square motif (3.3.4.3.4)

`hankin-3-3-4-3-4` · family [Islamic](../families/islamic.md) (`islamic`) · wallpaper group `p4g` · square lattice

Hankin greedy-inference motif on the snub-square (3.3.4.3.4) substrate (θK = 60°, δ = 0; all twelve instances stored CCW in their in-cell positions): each square in its five-face motif (four corner quadrilaterals + central octagon), each triangle in the side-1/2 4-subdivision limit (four equilateral triangles) — 52 faces per cell ⟨(1+√3,0),(0,1+√3)⟩ (det (1+√3)² = 4+2√3); tiles and lattice live in Q(√3), √2 enters only via the 45° glide-ray directions (all face vertices in (ℤ+ℤ√3)/4); θK = 60° chosen over 30° (30° slivers), 45°/67.5° (sharper 45° tips), and 90° (triangle pairing degenerates, square star collapses); p4g verified exactly on a 6292-face window: the face-set translation lattice is the centered lattice Λ′ = ⟨(L/2,L/2),(L/2,−L/2)⟩ with L = 1+√3 (the square-lattice half-steps fail), 4-fold centers {(L/4,L/4+½),(L/4,3L/4+½),(3L/4,L/4+½),(3L/4,3L/4+½)} and 2-folds = those ∪ {(0,½),(0,(L+1)/2),(L/2,½),(L/2,(L+1)/2)}, glides on 45° lines y−x ≡ L/4+½, 3L/4+½ (mod L) and 135° lines y+x ≡ L/4+½, 3L/4+½ with ¼-period shifts ±(L/4,∓L/4) — off the L/8 grid; no mirrors (M111)

## See also

- Family doc (catalog table + build arcs): [docs/families/islamic.md](../families/islamic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling hankin-3-3-4-3-4 --mode proto --out hankin-3-3-4-3-4.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
