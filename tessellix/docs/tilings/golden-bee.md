# Golden bee (Ammann A2 chair)

`golden-bee` · family [Quasiperiodic](../families/quasiperiodic.md) (`quasiperiodic`)

quasiperiodic irreptile (first in the registry): concave hexagon with five right angles and one reflex, edge lengths {s⁻⁴…s} (s = √φ), dissecting into two similar copies of unequal size (scales φ^−1/2 and φ^−1, area law s⁻² + s⁻⁴ = 1); √φ-inflation substitution, 8 orientation classes, exact Q(√φ), T-junctions intrinsic at the s^−4 edge ratio; finite covered patch (Vince 2014; Durand–Shen–Vereshchagin 2020)

## Presentation

The default view crops into the patch interior. The substitution-depth floor D_PRESENT = 8 (2^8 = 256 chairs) makes the probe-derived preview window — a smaller window than the whole patch at any scale — cover ≥ 0.85 of the frame, so the 256-chair structure is visible instead of the depth-5 floor (32 chairs). A `depth` parameter (Int, appended last, default 0) overrides the floor: 0 keeps the view-anchored auto rule above byte-identically (the default); 1..=17 pins the exact substitution level; out-of-range values are a named refusal, never a silent snap.

## See also

- Family doc (catalog table + build arcs): [docs/families/quasiperiodic.md](../families/quasiperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling golden-bee --mode proto --out golden-bee.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
