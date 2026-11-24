# Hankin motif khatam (4.8.8)

`hankin-4-8-8-motifs` · family [Islamic](../families/islamic.md) (`islamic`) · wallpaper group `p4m` · square lattice

Kaplan §3.5 design-element motifs (diss. §3.5) placed by the per-tile motif-assignment API (`hankin_motifs::build_cells_with_motifs`) on the 4.8.8 substrate of `hankin-4-8-8-rosette` (tile statics reused bit-identically): the octagon carries the **ideal rosette** (θ = π/8 = 22.5°, h = 1, φ = 0 — Lee's rosette — at depth s = 3: 32 segments, 8 hexagonal petals + one 32-gon core) and the diamond carries the **extension** of the rosette at realized outer contact angle 3π/8 = 67.5°, depth s = 2 (child element (4, r·F, s−1, π/8) rotated π/8 with F = cos θ/cos(π/n − θ) = √2−1: 4 spurs + 4 post-knee rays, 4 quads + one octagonal core) — 14 faces per ⟨(1+√2,0),(0,1+√2)⟩ cell, Σ areas = |det| exact.

## Specification

- **Construction.** Not greedy inference: each tile's figure is the §3.5 design element generated on its own contacts (both d_n chiralities), then the merged arrangement against the tile boundary partitions each tile. The extension spurs (child contact → landing parent midpoint) are exact: the child arm line passes through the parent-edge midpoint on its backward extension because −F·cos θ + cos(θ − π/n) = 0 identically and the landing radius is r·cos(π/n) — so rosette and extended rosette meet in the classical four-ray Hankin starburst at every octagon–diamond chord. The spur is collinear with the child leg-1 line, so the emitted figure is spurs ∪ post-knee child rays (emitting the leg segments too would hand the engine collinear overlaps, which `hankin::arrangement_faces` rejects).
- **Cell table.** 14 cells: seeds 0×8 hexagonal petals + 1 = 1 × 32-gon (octagon faces), 2×4 quads + 3 = 1 octagon (diamond faces); |det| = 3 + 2√2.
- **Group p4m**, verified exactly on the torus segment multiset of a ±2-cell window: rot90 about the octagon centre, rot90 about the diamond centre, mirrors y = 0 and y = x hold; rot45, half-lattice translations and rot60 fail (the motif battery's fold test normalizes the lattice, so a failed fold is a genuine group exclusion).

## Verification

- Motif library battery (Rust unit tests in `hankin_motifs`): Lee ideal-rosette knee identities (n = 5, 6, 8, 10, 12, worst offset < 1e-13, Python mpmath 40-dps reference 1.7e-16); extension landing exactness and F-identity across n = 5..12 × three angles × star/rosette.
- Instance tests: coverage / rotated-scaled / deterministic trio; full 208-float cell-vertex table machine-transcribed from a debug build (matched to 1e-9); seed layout, per-cell vertex counts, face lens census {(6, 8), (32, 1), (4, 4), (8, 1)}, motif segment counts (32, 8), area sum vs |det|; symmetry metadata.
- Regression: the existing engine-built hankin cell tables are byte-identical before/after the additive engine hook (`build_cells_with_segments`).

## Sources

- Kaplan (2111), *Beyond the Islamic Geometric Standard: Algorithms for Design and Analysis of Islamic Star Patterns*, §3.4–3.5 (Hankin inference, design elements, extension operator).
- Erratum: the dissertation prints the extension child angle as θ/2; the correct reading verified here is θ′ = θ − π/n (coincides with θ/2 only at θ = 2π/n).

## See also

- Family doc (catalog table + build arcs): [docs/families/islamic.md](../families/islamic.md)
- Substrate: [`hankin-4-8-8-rosette`](hankin-4-8-8-rosette.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling hankin-4-8-8-motifs --mode proto --out hankin-4-8-8-motifs.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
