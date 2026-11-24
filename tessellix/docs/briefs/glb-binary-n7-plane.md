# Brief: glb-binary-n7 plane-filling presentation (follow-up to M149)

> **LANDED as M149 follow-up (non-M)** (see docs/history/CHANGELOG.md). Do not dispatch.

> **Status (2026-11-22):** user directive — GLB binary n = 7 must fill the
> plane ("where applicable, fill the plane"). M149 landed the certified
> single-domain core; the literature tiling (Pautze's generalized binary)
> IS plane-filling, so this entry's island presentation is a generator-
> presentation choice, not mathematics. This brief upgrades the
> presentation. **Census is UNCHANGED** (same registry id, same family
> count, same built-ins) — this is a semantics/coverage upgrade, not a
> new tiling.

**Family:** `quasiperiodic` · **Module:**
`crates/tess-tilings/src/families/quasiperiodic/glb_binary_n7.rs`
(landed M149, 1406 ln, 12 tests — read ALL of it first) · **Prepass
brief (rule certification):** `docs/briefs/glb-binary-n7.md` (landed
`a783756`) · **Durable derivation record:**
`.scratch/glb-binary-n7/NOTES.md` (§1 MODEL, §2 RULE — quoted facts
below are from there; treat them as certified).

### Goal

`Tiling::generate` on `glb-binary-n7` must return **all unit tiles
intersecting ANY query bbox** (the tess-core contract) — i.e. every
view is covered by the tiling, up to the MAX_TILES budget and
view-driven depth quantization. Today `assemble(depth)` refines ONE
fixed seed big-tile domain B₀ = φ(V) (bbox ≈ 3.43 × 1.65 cell units;
the default 20-unit view shows σ³ = 35 tiles ≈ 0.3% canvas coverage —
measured island, export census 2026-11-22). Target: plane coverage in
the Penrose/Ammann–Beenker presentation class (their default-view
coverage measures ~111% incl. stroke overlap; content overflows the
view).

### Certified facts you build on (do NOT re-derive; cite NOTES.md)

- **Fan law (Pautze Fig 4 / eq 78):** rule k places a fan of four
  sectors around center O with sector sizes `(n−k−1, k, n−k+1, k)` in
  a-units, rays at `r0 + 2·cumsum`; sector sizes **sum to 2n = full
  circle at O for EVERY k** (a ∈ {0, n} sectors are EMPTY; k = n−1 is
  structurally impossible — 6-edge outline can't fill an 8-edge
  eps-region; proved). Sector a is filled by rhombus
  shape min(a, n−a). Children census per rule (n = 7): k=1 [2,1,0],
  k=2 [1,2,1], k=3 [0,1,3], k=4 [0,1,3], k=5 [1,2,1], k=6 [2,1,0].
- **Two-choice pool:** per parent shape the substitution admits TWO
  fan arrangements (rule k odd vs rule n−k even): shape 1 → k=1 only
  (k=6 proved impossible), shape 2 → {k=5, k=2}, shape 3 → {k=3,
  k=4}. Same child census both ways ⇒ substitution matrix well-
  defined: M7 = [[2,1,0],[1,2,1],[0,1,3]], PF eigenvalue λ² = b²+b+1.
- **eps-region model:** unit-edge children fill one of 16 staircase
  eps-regions of the big tile (side class h expands to U(h−1)+U(h+1)
  in either order); all 16 walks share the class MULTISET, differ in
  order (bump position). **Devices (bumps/indents) of NEIGHBORING
  parents mesh at shared vertices — this is the depth ≥ 2 global
  constraint** (NOTES §1) — i.e. cross-parent edge compatibility is
  part of the certified model, not an open problem.
- **Certified generator core (M149):** per-level DFS
  (`refine_level`), all-pairs interior disjointness +
  eps-complementary bend bits on shared parent edges (compat
  ε_A + ε_B = 1); unculled growth 3/10/35/126/462 (M7^d row sums);
  σ²…σ⁵ pass the staircase boundary certificate; depth ≥ 6 candidate
  search exhausts (a legal patch need not admit a legal refinement).
  Culling of the refinement frontier is UNSOUND (parity battery) —
  assemble full levels, cull only at emission.
- **Arithmetic:** K₇-twist pairs (k₁, s₁k₂) ∈ K₇², b = 2cos(2π/7),
  dyadic denominators D = 2, float emission only. Keep exactly this
  discipline — no float geometry anywhere in assembly.

### Architecture (worker decides with evidence; both routes start from the certified core)

**Route A — inflation/parenting (Penrose-style supertile anchors).**
Construct valid PARENT assignments level by level (parenting = the
inverse of the two-choice pool: a candidate parent must accept the
existing patch as one child-position's fan content with bend-bit
compatibility on ALL shared edges), anchor each supertile, deflate on
query. Pros: matches how penrose/ammann-beenker/stampfli present.
Cons: parent search must be certified (the pool is context-dependent).

**Route B — scale-lattice seeds (coarse-first).** The tiling is
self-similar: the λ^g-scaled coarse tiling arranges big tiles by the
SAME fan law (fans around centers O at every scale — sector sum 2n
means fans close full circles). Generate the coarse big-tile pattern
for the view first (fan assembly at the top scale), then refine each
coarse big tile to unit edge via the certified `refine_level` DFS,
with cross-domain candidates passing the same bend-bit compat on
shared parent edges (the "devices mesh at shared vertices" clause).
Pros: reuses the certified per-domain core verbatim; parenting never
needed. Cons: the coarse pattern generator is new machinery and must
be certified itself (edge parity over the coarse patch, fan closure).

Whichever route: **cross-boundary meshing is the crux.** A valid
multi-domain patch needs every shared parent edge to carry
complementary bend bits and every interior edge shared by exactly 2
tiles. The eps-cascade is global (a local map was probed and refuted
in M149) — do NOT reintroduce a local rule; run the same compat
predicate across domain boundaries.

### Battery (all pins programmatic; numbers measured in-test and reported)

1. **Plane coverage pin (THE new headline):** `check_coverage`-style
   sample census on LARGE views (at least the standard trio bboxes AND
   a ±20 world-unit box at unit emission): every sampled point covered
   exactly once; clipped-area equality to 1e-6 rel. No background
   holes anywhere in the view (contrast: M149's coverage ≈ 0.3%).
2. **Cross-boundary mesh audit:** over the assembled multi-domain
   patch: 0 bend-bit mismatches on shared parent edges; edge parity —
   every interior edge shared by exactly 2 tiles, boundary edges on
   the patch rim only; vertex census at rim continuity.
3. **Single-domain pins survive:** the σ²…σ⁵ staircase certificates
   and the 12 M149 tests remain valid as the per-domain core; update
   only what asserted island-only presentation (e.g. default-view
   bands — RE-MEASURE and pin the new bands; do not keep stale
   numbers).
4. **Budget + depth quantization:** view-driven depth still lands
   unit-edge tiles; pin tiles-per-default-view (expect O(10³–10⁴),
   well under MAX_TILES 250_000 — compute from domain density × σ^d
   and MEASURE); pin the depth clamp; per-step culling at emission
   only (frontier culling is unsound — do not cull assembly).
5. **Standard trio** in `quasiperiodic.rs` unchanged in shape
   (covers_region / rotated_scaled / deterministic) — they now pass on
   bboxes OUTSIDE the old B₀ domain too; extend their boxes if they
   were inside B₀ (say so in the report if so).
6. **Determinism** (geometry, proto, seed) — unchanged requirement.

### Census deltas: NONE

Registry id `glb-binary-n7` unchanged; quasiperiodic family stays 13;
built-ins stay 142; memberships stay 156. **If either orchestrator
census guard (`registry_groups_are_complete_and_canonically_ordered`,
`readme_catalog_matches_registry`) fires, that is UNEXPECTED — STOP
and report verbatim.** Update in your owned surfaces:
`docs/families/quasiperiodic.md` row (presentation note: "plane-
filling via nested fan hierarchy (M#-followup); M149 shipped the
certified single-domain core") + heading count unchanged.

### Files you own

- `crates/tess-tilings/src/families/quasiperiodic/glb_binary_n7.rs` —
  the presentation upgrade (keep the certified DFS core intact).
- `crates/tess-tilings/src/families/quasiperiodic.rs` — only if
  surfaces change (tests/battery); no registration/census changes.
- `docs/families/quasiperiodic.md` — row presentation note.
- Scratch `/tmp/opencode/glb-plane/` (append NOTES.md; the M149
  derivation record at `.scratch/glb-binary-n7/NOTES.md` is READ-ONLY
  reference).

**Do not touch:** `lib.rs`, `README.md`, `docs/INDEX.md`,
`docs/history/CHANGELOG.md`, `fields.rs`, any other family. NO M
number, NO pushes. One worktree commit:
`quasiperiodic-family: glb-binary-n7 plane-filling presentation (fan hierarchy)`.
Constraints: `cargo check --workspace` zero warnings; NO repo-wide
`cargo fmt`; exact K₇ arithmetic preserved; floats at emission only.

### STOP conditions

- Cross-boundary meshing cannot be certified at the scales the
  battery needs (compat exhaustion analogous to the depth-6
  candidate-search exhaustion) — a plane patch that cannot be proven
  gap-free/overlap-free must NOT ship; report the blocker with the
  exact failing census.
- Any M149 certificate (σ²…σ⁵ staircase, fan-area law 2λ^{2d},
  even-grid vertex census) regresses under the new assembly — STOP.
- The guards fire, or the census would move — STOP (this brief has
  zero census deltas).
Never guess; measure and report.