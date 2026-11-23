# Najm template (4,4)

`najm-44-3e-0-0-a` · family [Islamic](../families/islamic.md) (`islamic`) · wallpaper group `p4m` · square lattice

Najm-template substrate (Kaplan diss. §3.6.1 Fig. 3.22 second row): dodecagons r = √6−√2 on 2Z² sharing chords across the odd grid lines + concave equilateral 4-pointed stars (side 4−2√3, tips 60°, notches 210°); exact Q(√2,√3), areas 24−12√3 and 12√3−20 per cell

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
- Render it: `cargo run -p tess-tui -- export --tiling najm-44-3e-0-0-a --mode proto --out najm-44-3e-0-0-a.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
