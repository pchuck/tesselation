# Turtle (hats in turtles, einstein family)

`turtle` · family [Aperiodic](../families/aperiodic.md) (`aperiodic`)

aperiodic einstein family (SMKG 2023); chiral hats-in-turtles: turtle + hat prototiles, 9-label substitution, homochiral (all tiles unreflected — the plain turtle tiling mixes reflected copies), edge-to-edge hex-lattice geometry

## Algorithm

**Turtle** (`turtle.rs`) — "hats in turtles", the two-prototile
  member of the einstein family (SMKG 2023, arXiv:2305.17743): 14-gon
  turtle + hat on the exact hex lattice, 9 matching-rule labels
  (Γ..Ψ, Γ the Mystic turtle+hat compound), 8+1 key-point substitution
  with a per-level chirality flip that makes the global tiling
  homochiral (odd-depth patches are mirrored into the root placement,
  so every emitted tile is unreflected). Generation is per-view: the
  query region is covered by one rigid depth-Δ patch (depths 1–5,
  full-patch counts 9/71/559/4401/34649, max inscribed radius ≈129.9)
  whose centroid is snapped to a 4-hex-unit anchor grid near the view
  center; area ratios converge to 4+√15 per depth. Edge-to-edge
  (all vertices are hex-lattice points; the √3-edge midpoints are not),
  verified by the Python exact-integer reference against Kaplan's
  spectre.js at every depth.

## See also

- Family doc (catalog table + build arcs): [docs/families/aperiodic.md](../families/aperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling turtle --mode proto --out turtle.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
