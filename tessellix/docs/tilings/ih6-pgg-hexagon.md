# IH6 hexagon (pgg)

`ih6-pgg-hexagon` · family [Isohedral types](../families/isohedral.md) (`isohedral`)

Kaplan isohedral template IH6 — the 1-isohedral hexagon tiling with two rotationally-paired (U) edge orbits and two freeform (J) orbits on the 6³ honeycomb net (dual of the triangular tiling), wallpaper group pgg, four aspects. Shipped as the Tactile incidence symbol pinned at exact rational parameters (p1 … p5) = (1/8, 1/2, 9/40, 5/8, 1/2), rectangular lattice t1 = (0, 2), t2 = (7/4, 0); all coefficients exact `Q(√3)` elements (rational here), one canonical `f64` rounding per emitted coordinate. Verified by the standard coverage trio, the irrational-offset exactly-once grid, shared-edge bit-identity over a ±2 window (522 pairings: U orbits pair as reversals — 82 + 80 — and J orbits forward — 180 + 180), exact pairing-D census ({ROT 4, ID 8, ROT 4, ID 8} per orbit), aspect-symmetry, and transcription spot-checks to 1e-9 (design doc §9.1).

## See also

- Family doc (catalog table + build arcs): [docs/families/isohedral.md](../families/isohedral.md)
- Design doc (incidence symbols as engine data, exactness policy): [docs/isohedral-generator-design.md](../isohedral-generator-design.md)
- Render it: `cargo run -p tess-tui -- export --tiling ih6-pgg-hexagon --mode pattern --out ih6-pgg-hexagon.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*