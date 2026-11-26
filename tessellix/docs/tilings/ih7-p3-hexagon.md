# IH7 hexagon (p3)

`ih7-p3-hexagon` · family [Isohedral types](../families/isohedral.md) (`isohedral`) · wallpaper group `p3` · hexagonal lattice

Kaplan isohedral template IH7 — the 1-isohedral hexagon tiling with three freeform (J) edge orbits under 120°-rotated aspects on the 6³ honeycomb net (dual of the triangular tiling), wallpaper group p3, three aspects. The family's one non-rational record: Tactile's `√3`-decimal coefficients are fitted exactly to small-denominator `a + b√3` elements (residual < 1e-11; design doc §2.1), pinned at parameters (p1, p2) = (3/5, 1/5) with hexagonal lattice t1/t2 = (±1.5121…, −0.8730…); the edge orbits carry the showpiece four-interior-point curves. Verified by the standard coverage trio, the irrational-offset exactly-once grid, shared-edge bit-identity over a ±2 window (374 pairings, all emitted as reversals, bit-exact up to direction — the `Q(√3)` field arithmetic rounds equal exact values to bit-identical floats on both pipeline sides), exact pairing-D census (all three orbits ID-paired, 6 each), aspect-symmetry, a dedicated check that the record genuinely leaves the rational subfield, and transcription spot-checks to 1e-9 (design doc §9.1).

## See also

- Family doc (catalog table + build arcs): [docs/families/isohedral.md](../families/isohedral.md)
- Design doc (incidence symbols as engine data, exactness policy): [docs/isohedral-generator-design.md](../isohedral-generator-design.md)
- Render it: `cargo run -p tess-tui -- export --tiling ih7-p3-hexagon --mode pattern --out ih7-p3-hexagon.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*