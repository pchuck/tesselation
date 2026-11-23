# Pentiamond AC factor 3 (A + C rep-9)

`pentiamond-ac-factor-3` · family [Nonperiodic](../families/reptiles.md) (`reptiles`)

Bielefeld substitution index entry "Pentiamond AC factor 3" (discovered: folklore; Info empty; header says 3 prototiles but the **D row is absent from the rule image and the patch** — blocker, implemented as 2 prototiles, flagged): λ = 3, A → 6A + 3C, C → 5A + 4C (achiral C double-labeled, mirror pair via (8, (0,9)); canonical pin = table 0 for all poses, frame probe 108/108 + label invariance 24/24); A/C block [[6,5],[3,4]] char poly (x−9)(x−1), PF 9 = λ², primitive index 1 (no sum-9 D column makes the 3×3 primitive — 55-candidate scan), per-A-seed closed form A_d = (5·9^d+3)/8; 2×5 seed block (A poses 4,7 both P2 + C poses 1,4; block charge −2 — the folklore patch's P2-heavy 617:1899 depth-2 census, decaying to 1:1 in the limit, measured 0.9999 by d = 20); whole-block homothety on the (2·3^d, 0) × (0, 5·3^d) block lattice; limit-periodic construction, aperiodic limit — zero exact periods in the unit tiling (|v| ≤ 12, top near-miss 0.5712); folkloric 4024-tile patch reproduced exactly (recon E PASS, coverage 1.0); the A/C block is closed and the seed D-free ⇒ D count 0 at every depth regardless of the missing D row; exact integer cell arithmetic, floats only at emission (M156)

## See also

- Family doc (catalog table + build arcs): [docs/families/reptiles.md](../families/reptiles.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling pentiamond-ac-factor-3 --mode proto --out pentiamond-ac-factor-3.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
