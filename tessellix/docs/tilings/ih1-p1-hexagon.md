# IH1 hexagon (p1)

`ih1-p1-hexagon` · family [Isohedral types](../families/isohedral.md) (`isohedral`) · wallpaper group `p1` · oblique lattice

Kaplan isohedral template IH1 — the unconstrained 1-isohedral hexagon tiling: a generic (asymmetric) hexagon on the 6³ honeycomb net (dual of the triangular tiling), wallpaper group p1, one aspect, three freeform (J) edge orbits. Shipped as the Tactile incidence symbol pinned at exact rational parameters (p1, p2, p3, p4) = (3/20, 3/5, 1/5, 11/20), lattice t1 = (0, −1), t2 = (257/200, 3/10); every vertex, edge curve and aspect coefficient is an exact `Q(√3)` element (here rational), with one canonical `f64` rounding per emitted coordinate (design doc §2). Verified by the standard coverage trio, the irrational-offset exactly-once grid, shared-edge bit-identity over a ±2 window (112 pairings: 40/40/32 per orbit, all emitted as reversals — bit-exact up to direction), exact pairing-D census (all three orbits ID-paired), aspect-symmetry, and transcription spot-checks of every vertex and lattice vector against the Python spike's floats to 1e-9 (design doc §9.1).

## See also

- Family doc (catalog table + build arcs): [docs/families/isohedral.md](../families/isohedral.md)
- Design doc (incidence symbols as engine data, exactness policy): [docs/isohedral-generator-design.md](../isohedral-generator-design.md)
- Render it: `cargo run -p tess-tui -- export --tiling ih1-p1-hexagon --mode pattern --out ih1-p1-hexagon.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*