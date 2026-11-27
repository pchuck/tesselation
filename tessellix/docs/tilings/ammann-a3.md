# Ammann A3 (quasiperiodic)

`ammann-a3` · family [Quasiperiodic](../families/quasiperiodic.md) (`quasiperiodic`)

quasiperiodic rectilinear three-chair set (Grünbaum–Shephard, *Tilings and Patterns*, 1987): three rectilinear Z-chairs — P1 (a W1×H1 box with one TL notch), P2 (the largest: a W2×H2 box with TL and TR notches, A2 = φ·A1) and P3 (a W1 square with TL, TR and BL notches) — inflated by φ, the golden ratio; all vertices and child anchors in exact Z[φ] (φ² = φ + 1, y-down convention), floats only at emission; 12 oriented states (3 classes × 4 rotations) with the three certified substitution rows S1/S2/S3 pinned verbatim from an independent Python certificate (12/12 SHAPE + 12/12 RULE byte-match, per-rule R90 covariance live-verified, exact per-row area laws φ²A1 = A2 + A1, φ²A2 = 3A1 + A3, φ²A3 = 2A1 + A3 for areas A1 = (402,641), A2 = (641,1043), A3 = (478,804) in Z[φ]); count matrix [[1,1,0],[0,3,1],[0,2,1]] in (P2,P1,P3) order with N·A = φ²A exact row by row; non-edge-to-edge — T-junctions intrinsic to the rule frames (level 1: S1 clean, S2 and S3 carry 5 full-edge defects + 3 T-locations each; level-2 pinned census: 6/9/7 tiles, 9/13/9 full-edge defects, 4/5/4 T-locations, 0 split defects); single P2 seed body-centred, inradius b/2 = (1 + 7φ)/2 ≈ 6.1631; chiral — R90 rotation covariance within each class orbit only (no mirror); substitution-limit area fractions 52.36/32.36/15.28 % (count-limit 61.80/23.61/14.59 %) — the TE-patch measured 51.5/33.2/15.3 agree within ~1 pt (finite-size deviation, not a rule error)

## See also

- Family doc (catalog table + build arcs): [docs/families/quasiperiodic.md](../families/quasiperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling ammann-a3 --mode proto --out ammann-a3.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
