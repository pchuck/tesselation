# IH27 (pgg)

`ih27-pgg-pentagon` · family [Isohedral types](../families/isohedral.md) (`isohedral`) · wallpaper group `pgg` · rectangular lattice

Kaplan isohedral template IH27 — the 1-isohedral pentagon tiling with one rotationally-paired (U) edge orbit and two freeform (J) orbits on the Cairo net V3.3.4.3.4 (dual of the snub square; McLean topological type 3².4.3.4), wallpaper group pgg, four aspects. Classified as an isohedral variant (primary Isohedral, secondary Laves) — the waviness of the curved/notched per-orbit edge polylines is a deliberate deformation of the underlying IH27 incidence structure, not a pentagon type (no classical 1–15 cross-reference is pinned). Shipped as the Tactile incidence symbol pinned at exact rational parameters (p1, p2, p3) = (7/50, 7/15, 8/15), rectangular lattice t1 = (0, 5/3), t2 = (−1.6253…, 0); all coefficients exact `Q(√3)` elements (rational here). Verified by the standard coverage trio, the irrational-offset exactly-once grid, shared-edge bit-identity over a ±2 window (442 pairings: U orbit reversed — 82 — J orbits forward — 180 + 180), exact pairing-D census ({ROT 4, ID 8, ID 8}), aspect-symmetry, the pentagon/degree checks of the Cairo net, and transcription spot-checks to 1e-9 (design doc §9.1).

## See also

- Family doc (catalog table + build arcs): [docs/families/isohedral.md](../families/isohedral.md)
- Design doc (incidence symbols as engine data, exactness policy): [docs/isohedral-generator-design.md](../isohedral-generator-design.md)
- Render it: `cargo run -p tess-tui -- export --tiling ih27-pgg-pentagon --mode pattern --out ih27-pgg-pentagon.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*