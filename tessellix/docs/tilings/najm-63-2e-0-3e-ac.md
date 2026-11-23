# Najm template (6,3)

`najm-63-2e-0-3e-ac` · family [Islamic](../families/islamic.md) (`islamic`) · wallpaper group `p6m` · hexagonal lattice

Najm-template substrate (Kaplan diss. §3.6.1 Fig. 3.21) from the najm generator: regular dodecagons + enneagons sharing equal chords, case-3 inflation x = 2/(cot 15° + cot 20°), + concave equilateral bowties (70°/220°) at the 2-fold points; 1 doz + 2 enne + 3 bowties per cell, |det| = 3√3/2

## Algorithm

**Najm** (`families/islamic/najm.rs`, part of the `islamic` family
  feature) — the Najm template-tiling
  generator (Kaplan diss. §3.6.1, roadmap 22b): notation
  `([p,q]; m_A o_A, m_B o_B, m_C o_C; inflation)` → circumradii
  (`NajmSpec::radii`, inflation cases 1–4 via the absolute-trig extent
  equations) → substrate cell tables. First two entries:
  `najm-63-2e-0-3e-ac` (dodecagons + enneagons + bowties) and
  `najm-44-3e-0-0-a` (dodecagons + 4-pointed stars). Hole tiles use
  construction-specific formulas; a generic hole decomposition and the
  motif layers (22a/22c) are future work.

## See also

- Family doc (catalog table + build arcs): [docs/families/islamic.md](../families/islamic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling najm-63-2e-0-3e-ac --mode proto --out najm-63-2e-0-3e-ac.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
