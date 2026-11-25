# CAP (cut-and-project tiling)

`cap` · family [Aperiodic](../families/aperiodic.md) (`aperiodic`)

Baake–Gähler–Sadun's CAP tiling (arXiv:2305.05639, 2023): the four hat metatiles T/H/P/F deformed by the edge displacement vector (A, B, X, F, L) = (3φ, 3φ, 1, φ+ξ, 2φ); aperiodic, homochiral (6 orientations, no reflections), inflation φ² (LIDS), MLD to the φ²-rescaling, pure point spectrum

## Specification

CAP is a cut-and-project tiling of the plane whose four prototiles are
the hat metatiles with every boundary edge replaced by a longer
straight segment: A/B-class edges stretch to `3φ`, X edges stay at `1`,
F edges become `φ+ξ`, and L edges become `2φ` (the paper's §3 shape
vector, a left-eigenvector of σ̃₁ with eigenvalue φ² — the LIDS
property that makes CAP MLD to its φ²-rescaling). Each prototile occurs
in six rotations `r^k` and **no** reflected version (homochiral). Exact
areas: `v = (√3/4)·(15+27φ, 9+9φ, 14+22φ, 15+23φ)` for (H, T, P, F)
≈ (25.41, 10.20, 21.48, 22.61), with the paper's frequency vector
`f = (1/3, 5/3−φ, 2φ−3, 2−φ)` giving `v·f = 5√3φ²` exactly. All four
tiles are convex.

## Algorithm

**Deformed metatile substitution** (`cap.rs`) — the construction is a
D-map deformation of the hat metatile lineage (`hat.rs`'s 29-rule
substitution, duplicated verbatim). Every hat boundary-walk segment
`m·ξ^{kd}` carrying matching-rule label λ is replaced by the straight
CAP edge `λ_label·ξ^{kd+k}`; D is extended over a patch's walk-mark
mesh by propagation from one anchor tile (gauge `D(t₀) = t₀`), which is
well-defined because the matching rules force the same replacement on
both sides of every shared edge (the paper's Lemma 7 "belt" makes legal
patches rigidly interconnected — the Rust tests assert the propagation
is conflict-free and that every tile's deformed ring equals the shared
prototile shape translated by `D(tile origin)`, exactly in `ℤ[ξ,φ]`).
Per view: similarity-map region handling, the 96-grid anchor + wedge
seeds and depth escalation 3..=6 over the H-lineage (blob reach ≈ φ²
per level: 148/410/1096/2892 at depths 3..6, nested blobs), a
displacement-padded prune window (deformation moves tiles by up to
~0.25·|t|), one BFS deformation map per candidate (deterministic anchor
choice), full mark-ring emission clipped to the region by
Sutherland–Hodgman (valid — all tiles are convex), and exact
clipped-area acceptance `covered ≥ want·(1−1e−9)` that catches gaps and
overlaps at once (depth escalation on failure). All construction
arithmetic is exact integer abcd (`a + bφ + cξ + dφξ`); floats appear
only at emission.

## Verification

In-module Rust battery: walk tables close and match hat.rs's `BOUNDARY`
(marks, labels, lengths, order); prototile rings CCW with exact area
`v` and corner counts 6/3/4/5 for all 24 orientations; depth-3 blob
(442 metatiles, census (147, 22, 105, 168)) deforms conflict-free with
the shape law exact in `i64`; depth-2 patch (64 tiles) has zero
pairwise overlap (convex-clip census), exact area sum, and 210
complementary shared-walk segments; blob nesting across depths; census
table 10/64/442/3025/20737/142129; all 24 (type, rotation) orientations
occur in a generated view; plus the standard covers/deterministic/
rotated-scaled trio. An independent stdlib-Python battery (design doc
`docs/cap-cut-project-design.md`) verified BFS conflicts = 0 at depths
1–3, cross-level D agreement, and full emission coverage (clipped-area
relative error exactly 0.0).

## See also

- Family doc (catalog table + build arcs): [docs/families/aperiodic.md](../families/aperiodic.md)
- Design/derivation record: [docs/cap-cut-project-design.md](../cap-cut-project-design.md)
- The undeformed construction: [`hat`](hat.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling cap --mode proto --out cap.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*