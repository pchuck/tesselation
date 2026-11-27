# Golden triangle (√−τ reptile)

`golden-triangle` · family [Quasiperiodic](../families/quasiperiodic.md) (`quasiperiodic`)

quasiperiodic: the right triangle (legs 1, √τ; hypotenuse τ) dissecting by its altitude into two similar children (scales τ^−1 and τ^−1/2, area law τ^−2 + τ^−1 = 1); √(−τ)-inflation substitution (λ = i√τ, λ² = −τ, λL = L ∪ S, λS = L), both mirror chiralities, dense rotations, T-junctions intrinsic at the τ^−2 hypotenuse ratio; finite covered patch — the seed triangle itself (Danzer–van Ophuysen 2000) (M127)

## Presentation

The default view crops into the patch interior. The substitution-depth floor D_PRESENT = 8 (2^8 = 256 triangles) makes the probe-derived preview window — a smaller window than the whole patch at any scale — cover ~50% of the frame (the diagonal-half ceiling: the unit triangle's area is exactly half its AABB), up from the depth-5 floor at ~14%. A `depth` parameter (Int, appended last, default 0) overrides the floor: 0 keeps the view-anchored auto rule above byte-identically (the default); 1..=17 pins the exact substitution level; out-of-range values are a named refusal, never a silent snap.

## See also

- Family doc (catalog table + build arcs): [docs/families/quasiperiodic.md](../families/quasiperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling golden-triangle --mode proto --out golden-triangle.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
