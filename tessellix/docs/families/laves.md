# Laves / Catalan duals family

The Laves/Catalan duals: the eight face-centroid duals completing the Archimedean mirror, as exact data tables.

Code: `crates/tess-tilings/src/families/laves/` — cargo feature `laves` (default-on).
Adding a tiling here touches only this family: `families/laves.rs`
(module decl, register slice, classification arms, census `IDS`,
`lattice_tilings` entries) plus the new module and its doc row below.
Work briefs: `docs/briefs/TEMPLATE.md`; fleet protocol: `tessellix/AGENTS.md`.

## Implemented (8 tilings)

The whole Catalan family now mirrors the closed Archimedean set:
every dual is the face-centroid dual of an exact primal table.

| id | face config | group | primal | notes |
|---|---|---|---|---|
| `cairo-pentagonal` | V3.3.4.3.4 | p4g | snub-square | "Type 4 · Cairo"; tangential pentagons; minimum-perimeter pentagonal tiling; the collinear-vertex variant (143.13/108.43/108.43/90/90°, edges 4×√10 + 4, integral vertices, square 72-area cell) is documented in the card description (M76) |
| `tetrakis-square` | V4.8.8 | p4m | truncated-square | right isosceles triangles, 45°×8 at octagon centers; 4 proper pattern colors |
| `rhombille` | V3.6.3.6 | p6m | trihexagonal | 60°/120° rhombi, 3 per cell; pattern 3-colors by orientation (cube illusion) |
| `triakis-triangular` | V3.12.12 | p6m | truncated-hexagonal | 120°-apex isosceles triangles; triangular lattice of dodecagon centers, each cell split three ways |
| `deltoidal-trihexagonal` | V3.4.6.4 | p6m | rhombitrihexagonal | kites 60°/90°/120°/90°, 6 per cell |
| `kisrhombille` | V4.6.12 | p6m | truncated-trihexagonal | 30-60-90 right triangles, 12 per cell; bipartite 2-coloring |
| `prismatic-pentagonal` | V3.3.3.4.4 | cmm | elongated-triangular | pentagons (3×120° + 2×90°), 2 per cell; checker 4-coloring |
| `floret-pentagonal` | V3.3.3.3.6 | p6 | snub-trihexagonal | chiral pentagons (4×120° + 60°), 6 per cell; 6-color floret rosettes |

## Build arcs

Numbered arcs keep their PATTERNS.MD §numbers — code comments and
briefs cite them as `§N`.

> Arc: "complete the classics" — ✅ complete (M19–M29)
>
> Order below is the build order (mechanical duals first, then the
> direct implementation, then the rule-derivation heavyweights). One
> commit per tiling, `M19` onward, each with the three standard tests,
> the brute-force culling cross-check, and README/PATTERNS updates.


### 1. Laves completion — the 7 missing Catalan duals ✅ (M19–M25)

All seven were data tables derived by the cairo-pentagonal method:
dualize the existing exact primal table (face centroids in cyclic order
around each primal vertex); coordinates stay exact in Q(√3) (or Q(√2)),
module `crates/tess-tilings/src/families/laves.rs` behind feature `laves`.
M19 tetrakis-square, M20 rhombille, M21 triakis-triangular, M22
deltoidal-trihexagonal, M23 kisrhombille, M24 prismatic-pentagonal,
M25 floret-pentagonal — **the family is complete (8/8), mirroring the
closed Archimedean set.**

Every table was verified in exact quadratic-field arithmetic before
transcription (area sum == det, edge parity 2, vertex figures 360/n,
shifted-grid coverage, proper-coloring search), mechanically
cross-checked against the emitted Rust literals (vertex-by-vertex
numeric diff — caught an x/y slip in M23 the coverage test also
caught), and measured in the exported SVG (side/angle multisets).
`laves_pattern_mode_is_proper_coloring` (families/laves.rs) re-checks proper
pattern coloring on generated patches. The verification script lives
at `/tmp/opencode/laves_dual.py` (exact Q(√d) arithmetic, per-dual
battery) with Rust-ready literal emission (`--rust` flag).

## Per-tiling documents

Each member's full specification — algorithm, exact geometry, verification, sources — lives in its own document under [docs/tilings/](../tilings/):

- [`cairo-pentagonal`](../tilings/cairo-pentagonal.md) — Type 4 · Cairo (V3.3.4.3.4)
- [`tetrakis-square`](../tilings/tetrakis-square.md) — Tetrakis Square (V4.8.8)
- [`rhombille`](../tilings/rhombille.md) — Rhombille (V3.6.3.6)
- [`triakis-triangular`](../tilings/triakis-triangular.md) — Triakis Triangular (V3.12.12)
- [`deltoidal-trihexagonal`](../tilings/deltoidal-trihexagonal.md) — Deltoidal Trihexagonal (V3.4.6.4)
- [`kisrhombille`](../tilings/kisrhombille.md) — Kisrhombille (V4.6.12)
- [`prismatic-pentagonal`](../tilings/prismatic-pentagonal.md) — Prismatic Pentagonal (V3.3.3.4.4)
- [`floret-pentagonal`](../tilings/floret-pentagonal.md) — Type 5 · Floret (V3.3.3.3.6)

---

*Map: [docs/INDEX.md](../INDEX.md) · Changelog: [docs/history/CHANGELOG.md](../history/CHANGELOG.md) · Queue: [docs/ROADMAP.md](../ROADMAP.md)*
