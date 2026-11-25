# Taylor-Socolar striped hexagons

`taylor-socolar` · family [Aperiodic](../families/aperiodic.md) (`aperiodic`)

aperiodic decorated-hexagon monotile (Taylor & Socolar, arXiv:1003.4279); **plane-filling** CHT closed-form presentation (M168): `generate` serves the whole plane — per-site O(1) state assignment reproducing the paper's forced 529-hexagon patch (its Fig. 18) state-for-state, each hexagon partitioned into 8 decorated sub-polygons carrying the canonical chord + split-axis decoration whose chords close into nested equilateral Taylor triangles, state rules realized exactly (R1 0/2992, R2 0/2816, R3 0/2904 on the shipped patch; zero R1/R2/R3 violations on hex windows R=6/8/10/14 extending beyond it)

## Algorithm

**Taylor-Socolar striped hexagon** (`taylor_socolar.rs`) — the
  decorated-hexagon aperiodic monotile (arXiv:1003.4279), ported as a
  data table from the audited upstream study (M86; decoration reworked
  in M86-fix to the canonical closed-triangle form): the paper's forced
  529-site patch (Fig. 18) with its 12 tile states `(a, rbt, sdt)`,
  every hexagon partitioned into 8 non-overlapping convex sub-polygons
  — the band around one thick vertex-to-vertex chord (proto 1, two
  halves), the two halves of the centre-split diameter (red proto 2 /
  blue proto 3, the red end picked by `rbt`), and 2 body pentagons +
  2 body triangles (proto 0). All cuts are exact in Q(√3) (area sum
  `3√3/2`, 0 overlaps, strict convexity), the chord rule
  `kappa = (a + 3·sdt) mod 6` pairs every interior chord end (0 of the 1010 interior
  ends unpaired; the remaining 48 of the 1058 ends are open boundary ends) and is the only one of the 125 pairing rules among all 1296
  affine candidates that closes any ring at all, so the chord network
  **closes** into
  nested equilateral Taylor triangles on three scales (60 of side √3,
  12 of side 3√3, 2 of side 7√3) and every band colour is
  state-determined. The
  underlying rules are unchanged and re-derived in tests (R1 0/2992,
  R2 0/2816, R3 0/2904). **M168 plane upgrade:** the covered domain is
  the whole plane — the paper's CHT (centered hexagonal tiling)
  realized by the §5 closed form (mirror frame `w = (v−1,u)`, min-v2
  level/key, the 3-entry axis table, panel decomposition with signed
  `div_euclid` floor division, the 12-ray paper-folding free-bit
  table; center `c = (0,1)`, state `(0,0,1)`), O(1) per site, O(window)
  emission. Certification: the closed form reproduces the shipped patch
  **529/529**; Thm-5 parity + per-bit identities 440/440; Pfold ≡
  A014577 64/64; zero R1/R2/R3 violations on hex windows R=6/8/10/14
  (the windows extend beyond the old rhombus in all directions); only
  cstate `(0,0,1)` is R1=R2=0 (the other 11 are test-only).

## See also

- Family doc (catalog table + build arcs): [docs/families/aperiodic.md](../families/aperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling taylor-socolar --mode proto --out taylor-socolar.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
