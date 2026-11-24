# Najm motif plate ([6,3]; 2e, 0, 3e; AC)

`najm-63-motifs` · family [Islamic](../families/islamic.md) (`islamic`) · wallpaper group `p6m` · hexagonal lattice

Kaplan §3.5 design-element motifs (`hankin_motifs::build_cells_with_motifs`) on the Najm template substrate `najm-63-2e-0-3e-ac` (diss. §3.6.1 Fig. 3.21; its six cell polygons — dodecagon, two enneagons, three concave bowties — are reused verbatim): the dodecagon carries the **ideal rosette** (θ = π/12 = 15°, h = 1, φ = 0, depth s = 3: 48 segments, 12 hexagonal petals + one 48-gon core), each enneagon carries an **18-point star ring** (θ = 35°, depth s = 2: 18 segments, 9 quads + one 18-gon core), and the bowties stay plain via `Motif::None` (single faces) — 36 faces per ⟨(√3,0),(√3/2,3/2)⟩ cell, Σ areas = |det| exact.

## Specification

- **Angle choice on the enneagon.** The ideal 20° star (s = 1) is a shallow near-invisible rim band (depth ≈ 0.013 world units ≈ 3–6 px); θ = 20° at higher depth is degenerate (arms pass exactly through adjacent midpoints → duplicated chords, engine-rejected); θ = 40°, s = 2 has nine genuine collinear-overlap pairs (measured overlap 0.236/0.391 at perpendicular distance ~1e-16 — `hankin::arrangement_faces` panics on these); θ = 25° and 35° are structurally collinear-free by the parity check (for regular n-gons two arms are collinear iff 40Δ + (σ₁−σ₂)θ ≡ 0 mod 180). θ = 35°, s = 2 renders a clean deep 18-pointed ring (depth 0.064) and was measured overlap-free.
- **Motif::None.** A tile with no motif emits zero segments; the engine returns the tile itself as one face — the API's way of keeping infill tiles plain while neighbouring tiles carry design elements.
- **Group p6m**, verified exactly on the torus segment multiset of a ±2-cell window: rot60 and rot120 about the dodecagon centre and the mirrors through it at 0° and 30° hold; rot60 about the enneagon centre and half-lattice translations fail.

## Verification

- Instance tests: coverage / rotated-scaled / deterministic trio; full 492-float cell-vertex table machine-transcribed from a debug build (matched to 1e-9; the Python battery reference agrees to 2.3e-16); seed layout {0×12 petals, 1 = 48-gon, 2×18 quads, 3×2 18-gon cores, 4×3 bowties}, per-cell vertex counts, face lens census {(6, 15), (48, 1), (4, 18), (18, 2)}, motif segment counts (48, 18, 18, 0, 0, 0), area sum vs |det|; symmetry metadata.
- The substrate cells are taken from the registered `najm_63_2e_0_3e_ac()` build at runtime, so the tile table is bit-identical to the landed substrate by construction.

## Sources

- Kaplan (2111), *Beyond the Islamic Geometric Standard*, §3.5 (design elements, extension), §3.6.1 (Najm template, Fig. 3.21).

## See also

- Family doc (catalog table + build arcs): [docs/families/islamic.md](../families/islamic.md)
- Substrate: [`najm-63-2e-0-3e-ac`](najm-63-2e-0-3e-ac.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling najm-63-motifs --mode proto --out najm-63-motifs.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
