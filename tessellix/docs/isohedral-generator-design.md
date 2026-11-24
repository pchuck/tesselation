# The isohedral (IH) generator layer — design doc

Status: IMPLEMENTED (engine + 5 showcase instances; this document is the
specification the code implements). Companion spike: `.scratch/isohedral/`
(`spike1.py`–`spike4.py`, `types.json`, `rust_data.json`, `NOTES.md`;
evidence index in §9). Sources: Grünbaum–Shephard, *Tilings and Patterns*
([GS87], the IH1–IH93 classification); Craig Kaplan's isohedral.ca
incidence-symbol tables — via the machine-readable Tactile table
(`tiling_arraydecl.inc`, Kaplan's own 81-type data file, vendored in the
scratch checkpoint) as the primary authority; Tom McLean's per-type
templates (jaapsch.net mirror) for group/topology cross-checks.
**[P]** = proven/verified by exact computation below; **[C]** = cited.

---

## 0. One-paragraph summary

An IH type is an **incidence symbol** (tile symbol + adjacency symbol) over
a topology; operationally, the engine stores each type as the
Tactile-equivalent record — vertices, edge-orientation flags, translation
vectors, and aspect transforms as *exact linear forms in the parameters*,
plus a per-(aspect, edge) adjacency table — and generates the tiling as
{aspect transforms} × {lattice translates}. Shaped edges are exact
canonical polylines pinned **per edge orbit**; the two sides of any shared
edge evaluate the *same exact field elements* through *different exact
pipelines*, so the emitted `f64` sequences are bit-identical up to
reversal ([P], §3). All arithmetic lives in `Q(√3)` (rationals are the
subfield `b=0`), in canonical `a + b√3` form with rational `a, b` — unique
representation ⇒ one canonical float rounding per coordinate at the
emission boundary ([P], §2). Showcase instances pin exact parameters and
exact edge shapes; the engine itself accepts arbitrary `f64` parameters
behind the documented policy (§4). Instances take **primary Laves**
(their verified topologies are the 6³ honeycomb and V3².4.3.4 Cairo
nets — [P], §6); the Wallpaper tab is never touched (§6).

## 1. The edge-orbit data model (incidence symbols as data)

### 1.1 What an IH type is

Grünbaum–Shephard's IH1–IH93 enumerate the monohedral edge-to-edge tiling
types: each type fixes (i) a **topology** (combinatorial tiling — for the
types used here, a lattice tiling of n-gons), (ii) an **incidence
symbol** `⟨a₁a₂…⟩/⟨b₁b₂…⟩` recording how one tile's edges are identified
with its neighbors' (tile symbol: which edges are congruent and with which
orientation; adjacency symbol: which tile-aspect sits across each edge),
and (iii) a continuous parameter space of tile proportions and edge
shapes. Following Kaplan's Tactile encoding (the machine-readable
authority; isohedral.ca), the incidence symbol is stored *constructively*:

```text
TypeRecord {
  np, na, nv          // #parameters, #aspects, #vertices (edges = nv)
  vc : [Q; 2·nv·(np+1)]   // vertex x/y linear forms in the parameters
  tc : [Q; 4·(np+1)]      // translation vectors t1, t2 (x,y blocks)
  ac : [Q; 6·na·(np+1)]   // aspect transforms (2×3 linear forms)
  ori: [bool; 2·nv]       // per edge (fl, ro) orientation flags
  adj: [[i32; 4]; na·nv]  // adjacency symbol: (aspect′, di, dj, edge′)
}
```

* `Q` = element of `Q(√3)` (§2); each block of `np+1` coefficients is a
  linear form `c₀p₁ + … + c_{np−1}p_{np} + c_{np}` evaluated at the
  chosen parameters.
* `ori[k] = (fl, ro)` → edge `k`'s unit-frame map
  `E_k = match(v_k, v_{k+1}) ∘ M_orients[2·fl + ro]` with

  ```text
  match(p, q)   : (x,y) ↦ p + x·(q−p) + y·R90(q−p)   // rotation-scaling
  M_orients[0] = ID   (x,y) ↦ (x, y)
  M_orients[1] = ROT  (x,y) ↦ (1−x, −y)   // 180° about (½,0)
  M_orients[2] = FLIP (x,y) ↦ (1−x, y)    // mirror x = ½
  M_orients[3] = ROFL (x,y) ↦ (x, −y)     // mirror y = 0
  ```

  and `reversals[k] = (fl ≠ ro)`. (Verified against Tactile
  `tiling.cpp::recompute()` — [P], spike4/NOTES.)
* A tile's placement is `T_{a,i,j} = translate(i·t1 + j·t2) ∘ aspects[a]`;
  the tiling is exactly `{T_{a,i,j} : a ∈ aspects, (i,j) ∈ ℤ²}` — this is
  Tactile's `FillRegionIterator` semantics, and the reason generation
  needs no search (§5).
* **The adjacency symbol is engine data, not documentation**: `adj[a·nv+k]`
  names the neighbor across that edge as `(a′, di, dj, l′)` — i.e. across
  edge `k` of any tile `(a, i, j)` lies edge `l′` of tile
  `(a′, i+di, j+dj)`. Offsets are exact integers, valid at every lattice
  position. The table is *derived* (never hand-written): generated from a
  spike patch by exact geometric edge matching, then verified by BFS
  closure (every window tile reachable from the seed through `adj` alone)
  and by the exact D-census below ([P], spike1/spike4).

### 1.2 Edge orbits and the D-classification

Tactile's `edge_shape_ids` partition the `nv` edges into **orbits** (the
tile symbol's letter classes: same letter = congruent edge). For the
generator, the operative fact is finer: for a shared world edge between
tile sides `X = (T_X, k)` and `Y = (T_Y, l)`,

```text
D = (T_X ∘ E_k)⁻¹ ∘ (T_Y ∘ E_l)
```

maps the canonical unit frame to itself, and because both pipelines
describe the *same* world segment, `D ∈ M_orients` ([P]: `D` is an exact
unit-frame isometry fixing `{(0,0),(1,0)}` setwise; the exact spike
asserts membership per pairing). `D` is the *pairing's* orientation
signature:

| D     | classical edge class | constraint on the canonical curve |
|-------|----------------------|-----------------------------------|
| ID    | J (freeform)         | none |
| ROT   | U                    | 180°-symmetric about (½, 0) |
| FLIP  | S                    | mirror-symmetric about x = ½ |
| ROFL  | I (straight)         | curve must lie on the baseline |
| {ROT, ROFL} together | I    | Klein-group invariance ⇒ straight |

(These are exactly Tactile's `EdgeShape = {J, U, S, I}` invariance
classes; Tactile constructs U/S curves from half-curves via TSPI
splitting — the engine instead pins **full invariant curves** directly,
which is simpler and exact.) An orbit whose pairings mix D classes is
governed by the subgroup `H ⊆ {ID, ROT, FLIP, ROFL}` those classes
generate; the engine pins the curve class from `H` (most restrictive
wins: any ROFL ⇒ straight). IH29's orbit 0 is the shipped example of a
mixed orbit ([P], spike4: `{ROT, ROFL}` ⇒ straight base edge).

## 2. Exact arithmetic and the bit-identity proof

### 2.1 The field

All exact geometry lives in `K = Q(√3)`, represented canonically as
`a + b√3` with `a, b ∈ ℚ` (normalized rationals, `b ≥ 0` sign-canonical
per element). `ℚ ⊂ K` is the subfield `b = 0`, so the four rational-only
showcase types and the one `Q(√3)` type share one arithmetic. Uniqueness
of the canonical form gives the key engine property:

> two computations that produce the same exact value produce the same
> `(a, b)` pair, hence the same float at the boundary.

Multiplication `(a + b√3)(c + d√3) = (ac + 3bd) + (ad + bc)√3`; inversion
is the `ℚ(√3)` conjugation formula. Rationals are `i128/i128` pairs —
coefficient, parameter, and curve denominators (≤ 60, ≤ 50, ≤ 12 in the
shipped data) compound through ≤ 4 composition levels to far below the
i128 range [P: bounded by the spike's exact runs]. IH7's Tactile
coefficients are `√3`-decimals; each was **fitted** to `a + b√3` with
small rationals (residual < 10⁻¹¹, e.g. `0.288675134595 = √3/6`,
`1.44337567297 = 5√3/6`, `4.33012701892 = 5√3/2`) and the fitted record
re-verified end-to-end (§7) [P].

### 2.2 Shared-edge bit-identity (the IH-specific invariant)

Both sides of a shared edge emit the canonical curve of their orbit
through their own pipeline: side X emits `T_X·E_k·C_orb(k)`, side Y emits
`T_Y·E_l·C_orb(l)`. The engine never evaluates a float dot product
mid-pipeline: every coordinate is computed as an exact `K`-element (the
full composed linear form), then rounded to `f64` once by the canonical
emission `fl(a) + fl(b)·fl(√3)` (fixed evaluation order). Therefore:

1. If `D = ID`: `T_X·E_k = T_Y·E_l` as exact matrices; equal exact
   inputs ⇒ bit-identical sequences in the same order.
2. If `D = ROT` or `FLIP` (the u-reversing classes): the canonical curve
   is pinned D-invariant as a *set* (§1.2), so Y's sequence is X's
   reversed; each *value* is again one exact `K`-element computed two
   ways ⇒ bit-identical tuples, reversed order.
3. `ROFL`-orbits pin straight edges (the only baseline-invariant simple
   arc), collapsing to case 1 with `fl, ro` flags differing.

This is exactly the contract "both sides of an edge emit bit-identical
point sequences (up to reversal)", and the battery checks it as exact
`f64` sequence equality over a window of tiles — a genuine test of both
pipelines, since a wrong adjacency or D-class would produce different
exact values and different floats [P: spike4 ran this check in exact
arithmetic; the Rust battery repeats it on the emitted floats].

### 2.3 Polygon assembly order

Tile polygons are assembled per edge in boundary order: edge `k`'s
canonical point list is oriented `v_k → v_{k+1}` (reversed first when
`reversals[k]`), the duplicate joint with the previous edge is dropped,
and the wrap joint is dropped by the same rule. (`spike4` caught this:
emitting canonical order un-oriented produces backward edge runs on
reversed edges, and duplicated wrap vertices produce degenerate zero-length
segments [P].)

## 3. Edge shapes — exact canonical curves per orbit

Per orbit the engine pins one exact canonical polyline: endpoints
`(0,0)`, `(1,0)` (fixed by the unit frame) plus interior control points
in `K`. The shipped shapes (chosen once per orbit class, documented per
instance):

```text
J  (ID):        (1/8, 1/12), (1/2, −1/12), (7/8, 1/12)
U  (ROT):       (1/8, 1/12), (7/8, −1/12)
S  (FLIP):      (1/8, 1/12), (1/2, −1/12), (7/8, 1/12)
I  (ROFL/Klein): (no interior points — straight edge)
IH7 showpiece J: (1/6, 1/10), (2/5, −1/12), (3/5, 1/8), (5/6, −1/10)
```

The J/S shape is FLIP-symmetric (harmless extra symmetry on a J orbit);
all satisfy their class constraints exactly (`spike4` verifies set
equality under every `D ∈ H` per orbit [P]). Amplitudes ≤ 1/10 keep the
wiggled polygons simple (battery: zero self-intersections, sample tiles
per instance [P]) and tiles convex-ish polygons with
`poly.contains` well-defined (the tess-core contract is simple polygons,
not convexity).

## 4. Parameter encoding and the float-parameter policy

* **Shipped instances** pin exact rational parameters (binary-fraction
  style values, e.g. IH1 `p = (3/20, 3/5, 1/5, 11/20)`); the UI `ParamSpec`s
  declare the same values as floats with the pinned exact values as
  defaults. All geometry the generator *emits for the shipped defaults*
  is exact `K` arithmetic end-to-end.
* **The engine accepts arbitrary `f64` parameters** (that is the point of
  the IH family — continuous tile proportions). Policy: a float parameter
  is interpreted as the exact rational `Fr(f64)` (binary fractions are
  exact rationals), so *any* parameter choice yields exact geometry by
  the same pipeline; the `√3`-decimals of Tactile are replaced by the
  fitted exact coefficients (§2.1). Edge-shape control points are NOT
  exposed as runtime parameters in the shipped instances — each orbit
  carries its one pinned exact canonical shape (the brief's default
  position, adopted).
* The battery runs on the exact shipped instances (default parameters),
  not on swept float parameters — exactness claims are made only where
  the arithmetic is exact [C: brief §Exactness policy].

## 5. Generation algorithm

Direct `Tiling` implementation (the `penrose.rs` pattern; no
`LatticeTiling` tables):

1. Assemble the exact record at the requested parameters: vertices,
   `E_k`, aspects, `t1, t2` (all exact `K`).
2. Map the query region into lattice coordinates with the float inverse
   of `[t1 | t2]` (composition with the UI's `similarity_map`, so
   scale/rotation view parameters work unchanged); inflate by the
   instance's tile radius to a `(i, j)` bounding box.
3. For each `(i, j)` in the box and each aspect `a`: build the tile
   polygon (§2.3) exactly, convert to `f64` at emission, cull by exact
   vertex-bbox vs. region.
4. Pre-generation budget against `MAX_TILES` (area bound, penrose-style);
   refuse with a message instead of OOM.
5. `periodic = [t1·map, t2·map]` (world coordinates);
   `declared_symmetry = (group, conventional lattice)` per instance.

Determinism is structural: no RNG, no HashMap iteration order in the
emitted geometry (tiles are emitted in `(j, i, a)` order) [P: `_det`
battery].

## 6. Category policy

* **The Wallpaper tab is untouchable**: it is a pinned 17-card group
  catalog; any Wallpaper membership (primary or secondary) adds an 18th
  card and breaks `registry_groups_are_complete_and_canonically_ordered`.
  No IH instance joins it (the IH types are *realizations*, not group
  representatives). `WALLPAPER_GROUP_GAPS` is not modified.
* **Laves preferred**: the brief's rule is primary `Laves` when the
  instance's topology is one of the 11 Laves nets. The shipped topologies
  are verified programmatically (dual census [P], spike4/§9):
  - IH1, IH6, IH7: every primal vertex degree 3; dual vertices all
    degree 6, dual faces all triangles ⇒ primal = **6³ honeycomb net**
    (Laves V3⁶).
  - IH27, IH29: dual vertices all degree 5, dual faces
    170 triangles + 84 quadrilaterals, corner-degree pattern
    **(3,3,4,3,4)** around every dual vertex ⇒ primal topology
    **V3².4.3.4** (the Cairo-pentagonal Laves net; matches McLean's
    "topological type 3².4.3.4" for IH26–29).
  Hence **all five instances declare primary `Laves`**; IH27/IH29 add
  secondary `PentagonTypes` (the in-repo precedent is
  `floret-pentagonal`: Laves primary + PentagonTypes secondary).
* `pentagon_type` is deliberately left **unset** on IH27/IH29: they are
  Kaplan's isohedral pentagon templates, and no verified cross-reference
  to the classical 1–15 numbering was available offline; the Pentagon tab
  sorts untyped cards last (`browse.rs`), so the honest absence costs
  only ordering, not correctness.

## 7. The five showcase instances

All data from the pinned spike records (fitted exact coefficients;
adjacency derived + BFS-verified; curves pinned per orbit):

| id | IH type | tile | group | aspects/verts | orbits (D classes) | topology | category |
|----|---------|------|-------|---------------|--------------------|----------|----------|
| `ih1-p1-hexagon` | IH1 | hexagon | p1 | 1/6 | 3 orbits, all ID | 6³ | Laves |
| `ih6-pgg-hexagon` | IH6 | hexagon | pgg | 4/6 | 2 ID + 2 ROT | 6³ | Laves |
| `ih7-p3-hexagon` | IH7 | hexagon | p3 | 3/6 | 3 orbits, all ID | 6³ | Laves |
| `ih27-pgg-pentagon` | IH27 | pentagon | pgg | 4/5 | 2 ID + 1 ROT | V3².4.3.4 | Laves (+PentagonTypes) |
| `ih29-p4g-pentagon` | IH29 | pentagon | p4g | 4/5 | 1 ID + 1 Klein-straight | V3².4.3.4 | Laves (+PentagonTypes) |

Exact parameters: IH1 `(3/20, 3/5, 1/5, 11/20)`; IH6
`(1/8, 1/2, 9/40, 5/8, 1/2)`; IH7 `(3/5, 1/5)`; IH27
`(7/50, 7/15, 8/15)`; IH29 `(3/8)`. IH1 is pinned to a *generic*
(asymmetric, no extra symmetry) hexagon — the regular hexagon is
deliberately avoided (its type assignment is ambiguous under full p6m
symmetry). Groups cross-checked against McLean's pages (IH1 p1, IH6 pgg,
IH7 p3, IH27 pgg, IH29 p4g) and the aspect/lattice structure of the
records [C + P].

Set selection notes (deviations from PROPOSED §13, all evidence-backed):
§13's "IH5–IH7 pentagon interlocks" is wrong — IH5/IH6/IH7 are hexagon
templates (Tactile `num_vertices = 6`; McLean's hexagon page). The
pentagon templates are IH21–IH29. IH47 (p2 parallelogram) was considered
and set aside in favor of the pentagon pair. IH21 was dropped during
pinning: its Tactile coefficients are irreducibly messy decimals
(no small `a + b√d` fit), which would put the *type data itself* outside
the exactness policy.

## 8. What is deliberately NOT built

* Tactile's U/S **half-curve splitting** (TSPI_U/TSPI_S): only needed to
  *construct* invariant curves from arbitrary half-curves; the engine
  pins full invariant curves instead (§1.2, §3).
* The other 76 IH types: the engine is data-driven (§1.1), so adding a
  type = adding a verified record + curves + battery; nothing in the
  engine is instance-specific.
* Runtime edge-shape parameters (§4).

## 9. Verification appendix (spike evidence index)

All in `.scratch/isohedral/` (checkpointed from `/tmp/opencode/isohedral/`):

* `types.json` — all 81 Tactile records parsed with per-array length
  asserts (`parse_tactile.py`, `spike1.load_rec`).
* `spike1.py` — structural survey of 11 candidate types: zero bad
  segments (every segment shared by ≤ 2 tiles), BFS closure over the
  adjacency table alone covers the whole window, adjacency
  None-count = window-rim only, D-class histogram per type.
* `spike2.py` — exact `Q(√3)` field; rational snap (residual < 10⁻¹¹) of
  every rational-type coefficient; IH7 `√3`-decimal fitting (residual
  < 10⁻¹¹, small-denominator `a + b√3`).
* `spike3.py` — chosen exact params: structural battery per instance
  (bad=0, BFS closure, 0 T-junctions, convex base tiles, degree census)
  + exact D membership per pairing.
* `spike4.py` — final pinning: per-orbit D-uniformity/subgroup census,
  curve invariance (exact set equality under every `D ∈ H`), **exact
  world shared-edge identity** over the window (both pipelines, census
  per orbit: IH1 112; IH6 360 fwd + 162 rev; IH7 374; IH27
  360 fwd + 82 rev; IH29 392 fwd + 50 rev), polygon simplicity
  (0 self-intersections), and `rust_data.json` (the pinned tables the
  Rust data modules transcribe).
* Rust battery (per instance): `<id>_covers_region`,
  `<id>_rotated_scaled`, `<id>_deterministic`, `<id>_covers_exactly_once`
  (irrational-offset grid), `<id>_shared_edge_identity` (exact f64
  sequence equality up to reversal + edge census),
  `<id>_aspect_symmetry` (each aspect maps the patch tile multiset to
  itself mod the translation lattice), `<id>_orbit_D_census` (exact
  D-classification per pairing vs. the pinned table).

### 9.1 Final engine battery numbers ([P], the shipped `#[cfg(test)]` pins)

The spike4 census labels classified the *raw canonical curves* (before
the per-edge reversal flags are applied); the Rust battery classifies
the *emitted polygon edge sequences* (after each side's
`reversals[k] = (fl ≠ ro)` application). Both prove the same invariant
— bit-identical `f64` sequences up to reversal — but the direction
labels differ wherever an adjacency joins edges of opposite
reversal-status. The shipped pins (engine convention, identity
verified, every `(aspect, edge)` pairing covered, ±2 window):

| instance | shared-edge census (orbit, dir) → count |
|---|---|
| IH1 | (0,rev) 40 · (1,rev) 40 · (2,rev) 32 — every adjacency joins a (f,f) edge with a (f,t) edge |
| IH6 | (0,rev) 82 · (1,fwd) 180 · (2,rev) 80 · (3,fwd) 180 |
| IH7 | (0,rev) 150 · (1,rev) 112 · (2,rev) 112 |
| IH27 | (0,rev) 82 · (1,fwd) 180 · (2,fwd) 180 |
| IH29 | (0,fwd) 32 · (0,rev) 50 · (1,fwd) 180 · (1,rev) 180 |

Orbit-D censuses (exact, per the pinned `ORBIT_D` tables): IH1
{0:ID 2, 1:ID 2, 2:ID 2}; IH6 {0:ROT 4, 1:ID 8, 2:ROT 4, 3:ID 8};
IH7 {0:ID 6, 1:ID 6, 2:ID 6}; IH27 {0:ROT 4, 1:ID 8, 2:ID 8};
IH29 {0:ROT 2 + ROFL 2 (the mixed Klein orbit that forces its
straight base edge), 1:ID 16}.

**Layout lesson ([P], caught by the Rust battery):** the aspect
coefficient block stores Tactile's 2×3 matrix row-major
(`[xx xy tx / yx yy ty]`), so the affine reads off in natural order
`(a,b,c,d,e,f) = (blk00, blk01, blk10, blk11, blk02, blk12)`. A
transposed reading of the 2×2 linear part is **invisible for the
symmetric matrices of p1/pgg** (identity, mirror axes) — IH1/IH6/IH27
pass either way — and only bites for the non-symmetric rotations of
p3/p4g (IH7/IH29), whose first full battery run failed coverage,
aspect-symmetry, D-census and shared-edge identity until the
transposition was fixed against `spike2.build_exact`/`tiling.cpp`.
The irrational-offset exactly-once grid is what surfaced it first
(gaps at lattice positions the wrong aspect transforms never reached).