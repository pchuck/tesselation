# Watanabe–Ito–Soma (12-fold)

`wis-12-fold` · family [Quasiperiodic](../families/quasiperiodic.md) (`quasiperiodic`)

The Bielefeld "WIS 12-fold" entry (Watanabe, Ito & Soma, Acta Cryst. A
51:936, 1995): equilateral triangle + square + 30°/150° rhombus,
dodecagonal symmetry, inflation λ = 2+√3 (λ² = 7+4√3).

**Caveat: the ENTRY FIGURE's published rules, not the patch's.** This
tiling ships WSI95 Fig. 2(iii) — the entry's non-unique ruleset — NOT
the Bielefeld patch's undocumented ruleset: the figure's clusters have
zero occurrences in the 792 k-node patch (tri 0/150 certified leaves,
rho topologically distinct, sq 0 in the crop) and the patch is
non-generic (no census oracle). The "correct set" attributed to
Musesti–Paolini (2023) has no machine-readable record.

## Specification

- **Geometry:** exact ℤ[√3] 4-tuple points (x = H(P+Q√3),
  y = H(R+S√3), H = s/2), edge vectors in the index-4 sublattice,
  floats only at emission.
- **Non-conformal same-scale rules:** each supertile's children are
  prototiles at edge 2H (the same size as the parent) in a jagged
  cluster whose union has area λ²·parent and CONTAINS the parent.
- **The rho rule is a 3-piece disconnected cluster** (9+2+2 tiles) with
  fitted lattice-snap piece offsets island (−2,−1,−3,−2) /
  pendant (2,1,1,0); the inter-piece gaps are artist-chosen in the
  figure, not figure-determined (fit residuals 0.0465/0.0378 H vs
  0.0928/0.0967 H as drawn; the pendant offset is a rhombus edge
  direction; the 13-tile rho supertile never occurs in the Bielefeld
  patch, so no patch census constrains the fit). Pieces share no edges
  and touch at exactly two pinches, each a 4-tile vertex with corner
  sum 8 (a 120° gap) — the pinch geometry is figure-determined and
  certified.
- **Patch cross-check (report only):** Bielefeld patch.pdf
  18 302 tri : 11 644 sq : 7 689 rho.

## Level-1 certification (exact-arithmetic battery)

- Every dissection has one closed simple boundary per piece with all
  interior edges paired; 2-area identities (24,14)/(56,32)/(28,16) =
  λ² × parent (exact ℤ[√3]); parent ⊆ union (exact Q⁴ containment).
- Tri supertile: 11 children (rho 2, sq 2, tri 7); square supertile:
  26 children (rho 6, sq 4, tri 16); rho cluster: 13 = 9+2+2 tiles,
  per-piece simple chains (10/5/5 edges), cross-piece vertex
  intersection exactly the two pinned pinches.
- Angle census pinned: tri 16 vertices (1 interior + 15 corner),
  sq 29 (9 + 20), rho 20 (2 + 18 + 2 pinch); S(v) == want(v) at every
  vertex.

## Emission: a finite patch

The figure's rules do **not** compose at depth ≥ 2 (pinned D2
drift-guard: tri seed 155 tiles / 770 overlap pairs, sq 358 / 3660,
rho 179 / 1023, same-parent overlaps 0 — the level-1 dissections remain
exact partitions; the failure is between different D1 parents). The
`Tiling` therefore emits the depth-1 single-parent square supertile
(26 tiles, simply connected) as a finite patch; views outside the patch
are background (contract-legal for finite patches, `fibonacci_fib.rs`
precedent).

## Presentation

The tiling is a fixed 26-tile depth-1 patch (the single-parent square supertile — 16 tri + 6 rho + 4 sq, simply connected); the rules do not compose at depth ≥ 2 (pinned D2 drift-guard: tri seed 155 tiles / 770 overlap pairs, sq 358 / 3660, rho 179 / 1023, same-parent overlaps 0 — the failure is between different D1 parents), so no depth lever exists and the frame cannot be re-framed by emission. The default view is origin-centered (`preview_window`: aperiodic span 9.0 × mean tile, `Viewport::new` zero-centered), so the 26-tile patch does not fill the frame; measured UI coverage is 0.1161 at scales 1.0 / 3.5 / 0.3 (0.1125 at 20.0 — M210's quoted 0.1194 was a different sampling over the same 26 tiles). Scale is self-canceling there too (the preview window ∝ scale, so zooming re-crops the same fraction of the frame). The E15 tier rejected a recenter as "pin-breaking"; the rejection conflates the view with the patch geometry — the exact-arithmetic pins (level-1 dissections, angle census, coverage windows) derive from the patch, not the camera, so a VIEW-center recenter would NOT break them. Deferred follow-up: a `Tiling::default_view_center()` hook on the `Tiling` trait (no such hook exists today — `preview_window` is origin-centered at `crates/tess-core/src/app.rs`, `Viewport::new` is zero-centered at `crates/tess-core/src/view.rs`, and both UIs read the origin), landed in one change across tess-core + both UIs, with live pin re-derivation per the M202 E14 precedent (Hirschhorn SCALE was a view-param change with live pin re-derivation).

## See also

- Family doc (catalog table + build arcs): [docs/families/quasiperiodic.md](../families/quasiperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling wis-12-fold --mode proto --out wis-12-fold.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*