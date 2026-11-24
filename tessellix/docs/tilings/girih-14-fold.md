# Girih-inspired 14-fold (Pautze)

`girih-14-fold` · family [Quasiperiodic](../families/quasiperiodic.md) (`quasiperiodic`)

The Bielefeld "Girih 14-fold" entry ([Pau2017] S. Pautze, "Aperiodic
tilings with fourteenfold symmetry", *Symmetry* 9(2), 19 (2017),
doi:10.3390/sym9020019): eleven prototiles — rhombi R2/R3, hexagons
H26/H44/H45, octagons O29 (concave)/O36, decagons D2 (concave)/D46/D56
and the regular 14-gon G14 — with 14-fold point symmetry and inflation
λ = 2b² + 3b where b = 2cos(2π/7) (minpoly x³ + x² − 2x − 1).

## Specification

- **Inflation:** λ = 2b² + 3b = (0, 3, 2) in the b-basis ≈
  6.850855075327; λ² = (8, 20, 9) ≈ 46.93421526313568;
  **Norm(λ) = −7** — λ is *not* a unit of K₇ = Q(b) (the Bielefeld
  entry's unit claim is false; see Errata).
- **Exact arithmetic:** every vertex is a point of the module ½Z[ζ₁₄],
  carried as a *twist pair* (k₁, k₂, e): z = val(k₁) + i·sin(π/7)·val(k₂)
  scaled by 2⁻ᵉ, with (k₁, k₂) ∈ K₇² (the crate's exact cubic field,
  `fields.rs`). Twist multiplication folds sin²(π/7) = (2−b)/4 into a
  shared exponent; rotations are exact multiplies by ζ^j from the
  certified 14-row table; mirrors conjugate (k₁, k₂) → (k₁, −k₂).
  Every generated vertex normalizes to dyadic exponent ≤ 1 (asserted);
  f64 appears only at emission and in the coverage windows.
- **Canonical rings:** per class, the vertex ring is the unit-edge walk
  v₀ = 0, edge directions d_i in π/7 units, turns d_{i+1} = d_i + 7 −
  cyc[i+1] (mod 14); the walk closes exactly and its exact shoelace
  equals the certified area S (2·area/sin(π/7)) for all 11 classes.
  O29 and D2 are concave; all exact predicates are reflex-safe.
- **Rule system:** each class substitutes into a certified panel of
  whole kids — nv corner 14-gons centered at the parent vertices, nv
  edge decagons (class D2) centered on the exact edge midpoints
  (mirror-symmetric across the edge, half inside; adjacent parents
  emit the *identical* decagon — the FAM invariant F_d = F_{d+7}), and
  the rule-table interior kids (763 certified entries). The *effective*
  kid matrix N′ (interior counts + nv/2 for the shared edge decagons +
  Σcyc/14 for the corner 14-gon wedges) satisfies the exact K₇ area law
  Σ_c N′_rc·S_c = λ²·S_r on all 11 rows, and its PF eigenvalue is
  exactly λ². One full 11-panel round emits 849 kids
  (19+17+26+45+43+51+66+67+87+105+323).

## Generation

A single centered G14 seed at level N (world scale λᴺ, anchor
−λᴺ·CG where CG = R·ζ³ is the exact 14-gon centroid, R = 1+b): G14 is
convex, so the view bbox fits the seed iff its four corners do — the
minimal N ≥ 1 with the inverse-mapped bbox inside λᴺ·G14 is chosen
(cap N_MAX = 9). Per level downward: substitute every parent (corner →
edge → interior, the certified order), cull kids whose pad-expanded
AABB cannot reach the view, cap at MAX_TILES, then dedup by exact
vertex set (sorted dyadic keys) — straddling corner rings and edge
decagons are bit-identical across incident parents, so shared kids
collapse to one placement. Coverage is exact panel closure: the kids
tile their parent polygon plus certified straddle pokes that
neighbouring parents emit identically; the culling pads (derived from
the measured kid-vertex excursion δ_v over the 11 drawing-frame
panels) bound work only, never correctness. Views beyond the area
pre-guard (MAX_TILES·sin(2π/7) tile-units²) or the seed cap refuse
with a message; a degenerate (zero-area) view returns `empty view`.

## Verification (in-module battery, all exact)

1. **Rule-table rebuild** — all 849 kid rings rebuilt from the
   certified brief JSON (`docs/briefs/girih-14-fold-rule.json`, parsed
   by an in-test reader) and pinned entry-by-entry against the const
   tables: rot table, cyc walks, areas S, λ/λ²/R triples, all 763 rule
   entries, N′, PF float; per-kid sorted-dyadic-key equality, module
   validity (exponent ≤ 1), vertex uniqueness; the effective-N′
   formula (interior + nv/2 + Σcyc/14) re-derived per class; the FAM
   edge-decagon invariant and the verify_cg identities (Σ CV[G14] =
   14·CG, CV[G14][i] − CG = R·ζ^{(10+i) mod 14}).
2. **Area law** — Σ_c N′_rc·S_c − λ²·S_r = 0 in exact K₇ for all 11 rows.
3. **PF eigenvalue** — power iteration on N′ converges to λ² =
   46.93421526313568 (1e-9 against the certified float).
4. **Module + direction census** — every vertex of all 849 panel rings
   has dyadic exponent ≤ 1; every edge decomposes exactly as
   λⁿ·ζ₁₄^k and the global direction census finds exactly 14 classes.
5. **Two-level patch** (R2 seed at scale λ²) — 19 level-1 kids (mix
   2·R2 + 4·R3 + 4·H26 + 1·H45 + 4·D2 + 4·G14), all 171 pairs exactly
   interior-disjoint; level 0: 1813 pre-dedup emissions → 1691 distinct
   placements (emission histogram 1×1597, 2×76, 3×10, 4×6, 5×2), post
   mix (R2 366, R3 562, H26 161, H44 132, H45 170, D2 134, D46 24,
   G14 142), pinned pre/post S-sums, zero overlapping pairs (AABB
   prefilter → 6474 candidate pairs → exact predicates, all disjoint),
   and dedup idempotence (re-running substitution adds no placement).
6. **Rare-class O36** — the O36 parent panel carries 66 kids and *no*
   interior O36 kid (N′[O36][O36] = 0); a D56-seeded panel has exactly
   2 interior O36 kids, and each promotes to a well-formed 66-kid panel.
7. **λ measurement** — max parent edge / max edge-kid edge per panel =
   λ within 1e-3 for all 11 panels (measured exactly λ here).
8. **Panel render match** — per-panel whole-kid counts equal
   nv corner + nv edge + interior N for all 11 panels; every kid ring
   equals the JSON-driven ring.
9. **Standard trio + budget** — `covers_region` ([0,8]²),
   `rotated_scaled` (scale 0.6, rot 25°), `deterministic`; the [0,60]²
   default view terminates at 1355 tiles (budget 250 000) with a
   coherent patch; scale-0.1 zoom-out refuses via the pre-guard
   ("too many tiles for this view — zoom in"); zero-area views return
   `empty view`.

## Errata

- The Bielefeld entry states the inflation factor is a **unit** of its
  ring. False: Norm(λ) = λ(b₁)·λ(b₂)·λ(b₃) over the three conjugates
  b_j = 2cos(2jπ/7) (6.850855 · −0.939001 · 1.088142, exactly −7 in
  K₇) — λ is an algebraic integer of norm −7, not a unit (Norm ±1).
- [Pau2017] Table 7 lists the n = 7 inflation as ≈ 5.801939; the
  paper's own Figs 7.4/7.5 substitution drafts measure 6.8507 ± 0.0006
  = λ ≈ 6.850855 (the figures win; certified brief erratum).
- The Bielefeld patch specimen is **non-generic** (O36 absent, deviant
  PF frequencies on rare classes) and is never used as a census oracle
  here; all statistics come from self-generated patches.

## Sources

- Bielefeld Tilings Encyclopedia, "Girih 14-fold":
  <https://tilings.math.uni-bielefeld.de/substitution/girih-14fold/>
- S. Pautze, "Aperiodic tilings with fourteenfold symmetry", *Symmetry*
  9(2), 19 (2017). doi:[10.3390/sym9020019](https://doi.org/10.3390/sym9020019)
  · arXiv:[1606.06858](https://arxiv.org/abs/1606.06858)

## See also

- Family doc (catalog table + build arcs): [docs/families/quasiperiodic.md](../families/quasiperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling girih-14-fold --mode proto --out girih-14-fold.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*