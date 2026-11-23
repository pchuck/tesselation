# Taylor-Socolar striped hexagons

`taylor-socolar` · family [Aperiodic](../families/aperiodic.md) (`aperiodic`)

aperiodic decorated-hexagon monotile (Taylor & Socolar, arXiv:1008.3103); the paper's forced 529-hexagon patch (its Fig. 18), each hexagon partitioned into 8 decorated sub-polygons carrying the canonical chord + split-axis decoration (one thick vertex-to-vertex chord plus a centre-split red/blue axis) whose chords close into nested equilateral Taylor triangles, state rules realized exactly (R1 0/2992, R2 0/2816, R3 0/2904, re-derived in tests); finite covered patch, no period within hex-radius 30

## Algorithm

**Taylor-Socolar striped hexagon** (`taylor_socolar.rs`) — the
  decorated-hexagon aperiodic monotile (arXiv:1008.3103), ported as a
  data table from the audited upstream study (M86; decoration reworked
  in M86-fix to the canonical closed-triangle form): the paper's forced
  529-site patch (Fig. 18) with its 12 tile states `(a, rbt, sdt)`,
  every hexagon partitioned into 8 non-overlapping convex sub-polygons
  — the band around one thick vertex-to-vertex chord (proto 1, two
  halves), the two halves of the centre-split diameter (red proto 2 /
  blue proto 3, the red end picked by `rbt`), and 2 body pentagons +
  2 body triangles (proto 0). All cuts are exact in Q(√3) (area sum
  `3√3/2`, 0 overlaps, strict convexity), the chord rule
  `kappa = (a + 3·sdt) mod 6` pairs every interior chord end (0 of 1058
  unpaired) and is the only one of the 125 pairing rules among all 1296
  affine candidates that closes any ring at all, so the chord network
  **closes** into
  nested equilateral Taylor triangles on three scales (60 of side √3,
  12 of side 3√3, 2 of side 7√3) and every band colour is
  state-determined. The
  underlying rules are unchanged and re-derived in tests (R1 0/2992,
  R2 0/2816, R3 0/2904). The covered
  domain is the finite patch (girih precedent), not the plane.

## See also

- Family doc (catalog table + build arcs): [docs/families/aperiodic.md](../families/aperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling taylor-socolar --mode proto --out taylor-socolar.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
