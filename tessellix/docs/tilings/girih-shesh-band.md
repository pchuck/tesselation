# Girih Shesh-Band (6.6.6)

`girih-shesh-band` · family [Islamic](../families/islamic.md) (`islamic`) · wallpaper group `p2` · centered lattice

periodic girih strapwork band

## Algorithm

**Girih** (`girih.rs`, `girih_qc.rs`) — the girih prototiles with
  exact unit-edge radical geometry. `girih-shesh-band` is a periodic
  single-tile oblique-lattice band. `girih-quasiperiodic` generates the
  genuinely quasiperiodic star-and-cross pattern with the Darb-i Imam
  3-prototile substitution (Lu & Steinhardt 2007): decagon, bobbin and
  bow-tie, inflation 2(2+√5) = 2φ³, an edge-marker seam rule, exact
  arithmetic in Q(2cos 18°), an exact 10-fold rosette around the seed
  decagon, and edge-to-edge, T-junction-free patches at every
  generation (frozen core: tiles are depth-independent inside radius
  ~79).

## See also

- Family doc (catalog table + build arcs): [docs/families/islamic.md](../families/islamic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling girih-shesh-band --mode proto --out girih-shesh-band.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
