# Golden rhomboid-triangle (Andritz φ-substitution)

`golden-rhomboid` · family [Quasiperiodic](../families/quasiperiodic.md) (`quasiperiodic`)

quasiperiodic: equilateral triangles side 1 (Y) and side φ (O) plus the 60/120 rhombus 1×φ (R), the Andritz family's third member (Bielefeld "golden-rhomboid-triangle", Andritz 5-fold cluster); rules Y → O, O → O+Y+R, R → R+2O (areas 1 : φ² : 2φ in √3/4 units, all identities exact); count matrix [[1,1,0],[2,1,1],[0,1,0]] in (R,O,Y) order, char poly λ³−2λ²−2λ+1, PF eigenvalue φ² pinned as an exact Q(√5) root (φ² = (1,1)); exact 4-tuple Q(√5) arithmetic end to end (x = (a₀+a₁φ)/2, y = (b₀+b₁φ)·√3/2, φ·(a₀,a₁) = (a₁,a₀+a₁); floats only at emission); 18 anchored orientation classes collapsing to 10 unanchored sets under the authorized 120°-rotation quotient (SHAPES/RULES stay anchored); single-seed unbounded growth — depth d covers φ^d·R₀ exactly, union = the 60° wedge {y ≥ 0, y ≤ √3·x} (views outside the wedge are background), depth cap 12 (N₁₂ = 134 209 ≤ 250 000); T-junctions intrinsic to the substitution (first at depth 2: 2 points / 5 incidences), every T-point closing to exactly π on both sides in π/3 units — a gap-free/overlap-free partition, not face-to-edge; depth count vectors and 18/18 exact rule dissections pinned (M144)

## Presentation

The default view is origin-centered (`preview_window`: aperiodic span 9.0 × mean tile, `Viewport::new` zero-centered at birth), and the covered domain — the intrinsic 60° wedge {y ≥ 0, y ≤ √3·x} with its vertex anchored at the origin vertex R₀ — cannot fill the frame: the window crops across the wedge's hypotenuse rather than fitting inside it. UI coverage measured 0.1958, holding across scales 1.0 / 3.5 / 0.3 / 20.0 and rotations −30° / −45° — scale is self-canceling (the preview window ∝ scale, so zooming re-crops the same fraction of the frame) and rotation only changes which hypotenuse cuts the frame.

The recenter has now landed: a `Tiling::default_view_center()` hook (a provided method on the `Tiling` trait, default `None`) is consumed at `App::new` and at `App::reset_view` — never at selection time, so a user's pan/zoom across tiling switches is preserved. This tiling overrides it with (5.14, 3.67): a *preview-scale* center along the unbounded wedge — not a union center — since the wedge is depth-capped growth with no finite union (a naive union center of the ±60 frame would sit at ≈ (31.0, 30.6) and crop any preview entirely out of the frame). Re-measured honestly at (5.14, 3.67) over the same derived window (point-sampled, 60×60 grid): coverage 0.1958 → 0.4947, tiles 47 → 113; the main view (1200×900 at zoom 60) rises from 66 tiles / 0.1958 to 141 tiles / 0.4475. The exact-arithmetic pins (wedge growth, the 18/18 rule dissections, the standard-coverage windows) are patch-derived and unaffected by the camera. Thumbnails deliberately stay identity views: the GUI `card_texture` / TUI `thumb.rs` window derivation is origin-centered by construction (its own ±10 probe + `preview_window`, not the hooked viewport) and stays so — the hook drives the main view only.

## See also

- Family doc (catalog table + build arcs): [docs/families/quasiperiodic.md](../families/quasiperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling golden-rhomboid --mode proto --out golden-rhomboid.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
