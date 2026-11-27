# IH29 (p4g)

`ih29-p4g-pentagon` · family [Isohedral types](../families/isohedral.md) (`isohedral`) · wallpaper group `p4g` · square lattice

Kaplan isohedral template IH29 — the 1-isohedral pentagon tiling whose base edge orbit pairs through BOTH generators of the Klein four-group (ROT and ROFL appear among its pairings), forcing that orbit's canonical curve to be the straight baseline; the second orbit is freeform (J). Topology: the Cairo net V3.3.4.3.4 (dual of the snub square; McLean topological type 3².4.3.4), wallpaper group p4g, four aspects, square diagonal lattice t1/t2 = (±4.8125, ∓4.8125). Classified as an isohedral variant (primary Isohedral, secondary Laves) — deliberate edge deformation, not a pentagon type. Shipped as the Tactile incidence symbol pinned at the exact rational parameter p1 = 3/8, all coefficients exact `Q(√3)` elements (rational here). Verified by the standard coverage trio, the irrational-offset exactly-once grid, shared-edge bit-identity over a ±2 window (442 pairings: the Klein orbit emits 32 forward (ROFL pairings fix the baseline pointwise) + 50 reversed (ROT pairings), the J orbit 180 forward + 180 reversed by reversal-flag parity), exact pairing-D census ({ROT 2 + ROFL 2, ID 16} — the mixed orbit asserted literally), the straight-base-edge check, aspect-symmetry, and transcription spot-checks to 1e-9 (design doc §9.1).

## See also

- Family doc (catalog table + build arcs): [docs/families/isohedral.md](../families/isohedral.md)
- Design doc (incidence symbols as engine data, exactness policy): [docs/isohedral-generator-design.md](../isohedral-generator-design.md)
- Render it: `cargo run -p tess-tui -- export --tiling ih29-p4g-pentagon --mode pattern --out ih29-p4g-pentagon.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*