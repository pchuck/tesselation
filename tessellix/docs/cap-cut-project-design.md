# CAP tiling — derivation and emission design (`cap`)

**Family:** `aperiodic` · **Registry id:** `cap` · **Status:** design for
`families/aperiodic/cap.rs` · **Base:** worktree `cap-cut-project-impl`,
HEAD `141e43b` at design time.

**Sources.** Baake–Gähler–Sadun, *Dynamics and topology of the Hat family
of tilings*, arXiv:2305.05639 [math.HO] — cited below as [BGS23] with the
ar5iv line anchors used during derivation: shape-vector choice and LIDS
(lines 1007–1030), return module t₀ (1062), Fig 6 tiles/control points
(965–975), Lemma 7 + Fig 2 (479–514), edge/vertex AP data (532–699),
CPS + window (1141–1236), frequency/area vectors (1338–1375). All
programmatic claims in this document were verified by the stdlib-only
exact batteries in `.scratch/cap2/` (`cap_spike.py`, `probe9.py`,
`probe10.py`, `probe11.py`; results in `results*.txt`). Nothing here
rests on a visual impression.

## 1. The object

The CAP tiling ([BGS23] §3, Fig 6/7) is the aperiodic tiling whose
prototiles are the four hat **metatiles** T, H, P, F — not the hats —
with every boundary edge replaced by its deformed length. Each prototile
occurs in the six rotations `r^k`, k = 0..5, and **not** in a reflected
version (Fig 6 caption: "Each tile occurs in six different orientations,
but not in a reflected version"). The tiling has inflation φ² linear /
φ⁴ per-count, is MLD to its φ²-rescaling (LIDS, lines 1023–1030), and
its control points form a cut-and-project model set with pure point
spectrum (§3–4).

The paper fixes the construction by choosing the edge displacement
vector ("we work directly with the shape of the T, H, P and F
meta-tiles, or equivalently the vector displacements along the A, B, X,
F and L edges. We choose", lines 1007–1010):

```
(A, B, X, F, L) = (3φ, 3φ, 1, φ+ξ, 2φ) = μ₁ + (φ+ξ)μ₂ + 2φμ₃
```

with μᵢ the H¹ eigenvectors of eq. (12) (lines 955–958); this class is
cohomologous to (φ+1, φ+ξ), a left-eigenvector of σ̃₁ with eigenvalue
φ². The tiles drawn in Fig 6 are exactly the metatiles with these edge
lengths. Our emitter realizes precisely this object; there is no
improvisation and no contradiction with any pin.

Exact tile areas (a = √3/4, order (T, H, P, F), lines 1370–1375):

```
v = a·(9+9φ, 15+27φ, 14+22φ, 15+23φ)      ≈ (10.20, 25.41, 21.48, 22.61)
f = (5/3−φ, 1/3, 2φ−3, 2−φ)               (frequency vector, line 1345)
v·f = 5√3φ²
```

The battery reproduces both vectors exactly: Σf·A = 5√3φ² holds exactly
in ℚ(φ) (`cap_spike.py`), and the depth-6 blob census (T,H,P,F) =
(6912, 47377, 33552, 54288) matches f to 5 decimal places
(0.048633, 0.333357, 0.236059, 0.381971).

## 2. The construction (what is emitted and why it is the paper's tiling)

### 2.1 Metatile boundary walks and the deformation D

Each metatile type has a boundary walk on the unit Eisenstein lattice
ℤ[ξ] (the hat-lattice, step ξ = e^{iπ/3}); consecutive walk vertices
("marks") are one or more lattice steps apart, and every walk segment
carries one of the five edge classes A, B, X, F, L (the labels come from
the [BGS23] Fig 4/5 edge data; sub-labels distinguish orientation
variants of the same class). The walk tables (mark (a,b), label, step
count m) are:

```
T: (0,0)#1·6, (6,0)#1·6, (0,6)#2·6
H: (0,0)#4·2, (2,-2)#3·6, (8,-2)#5·2, (10,-2)#4·2, (10,0)#3·6,
   (4,6)#5·2, (2,8)#4·2, (0,8)#0·6, (0,2)#5·2
P: (0,0)#8·2, (2,-2)#5·2, (4,-4)#4·2, (6,-4)#1·6, (12,-4)#8·2,
   (10,-2)#5·2, (8,0)#4·2, (6,0)#2·6
F: (0,0)#8·2, (2,-2)#5·2, (4,-4)#4·2, (6,-4)#8·2, (8,-4)#5·2,
   (10,-4)#6·2, (10,-2)#7·2, (8,0)#4·2, (6,0)#2·6
```

(label → class: 0–3 → A/B, 4–5 → X, 6–7 → F, 8 → L.)

**The deformation D.** For a placed metatile (type, rotation k,
Eisenstein origin t), each walk segment with direction class `kd` (the
segment is m·ξ^{kd+k}) and label `lab` is mapped by D to the straight
segment

```
λ_lab · ξ^{kd+k},   λ = (A,B,X,F,L) = (3φ, 3φ, 1, φ+ξ, 2φ) per class
```

i.e. the hat-space lattice displacement m·ξ^{kd+k} is replaced by the
CAP edge displacement of its class in the same direction. D is extended
to all walk-mark vertices by propagation: D(z₀) is fixed at one anchor
vertex and every other vertex value is the sum of segment deltas along
any mesh path. This is exactly the paper's construction: the CAP tile is
the metatile whose edges carry the displacements (3φ,3φ,1,φ+ξ,2φ)
(Fig 6), and the matching rules force the same replacement on both sides
of every shared edge, so the deformed metatiles of a legal hat-metatile
patch fit edge-to-edge into a CAP patch.

All positions live in ℤ[ξ,φ] (abcd = a + bφ + cξ + dφξ, φ²=φ+1,
ξ²=ξ−1, ξ³=−1): the segment deltas above are integral abcd 4-vectors
(A,B ↦ (0,3,0,0) = 3φ; X ↦ (1,0,0,0); F ↦ (0,1,1,0) = φ+ξ; L ↦
(0,2,0,0) = 2φ, rotated by ξ^k via (a,b,c,d) ↦ (−c,−d,a+c,b+d)). The
whole deformation combinatorics is therefore **integer-exact**; floating
point appears only when a vertex is converted to world coordinates at
emission (x = a+bφ+(c+dφ)/2, y = (c+dφ)·√3/2).

### 2.2 Consistency: why D cannot contradict itself (the no-pile-up proof)

D is well-defined on a patch iff every mesh cycle carries zero total
delta. This was verified **exactly** (integer arithmetic, no epsilons)
on the nested lineage blobs:

| depth | metatiles | walk-mark vertices | directed segments | conflicts | non-integral | per-corner mismatches |
|-------|-----------|--------------------|-------------------|-----------|--------------|-----------------------|
| 1     | 10        | 135                | 247               | 0         | 0            | 0                     |
| 2     | 64        | 797                | 1701              | 0         | 0            | 0                     |
| 3     | 442       | 5000               | 11657             | 0         | 0            | 0                     |

(`probe9.py`/`probe10.py`: BFS from D(0)=0 over the full mark mesh;
"conflicts" counts distinct path values arriving at one vertex;
"per-corner" checks the tile law of §2.3 at every instance.) Because the
depth-d blob is contained in the depth-(d+1) blob **at the same frame
positions** (the substitution's child 0 is the H metatile at the origin,
nested at every level — verified), the three D-maps agree exactly on
their common domain: 135 = |D₁∩D₂| vertices, 0 disagreements; 797 =
|D₂∩D₃|, 0 disagreements. One common frame, one deformation. A
contradiction inside any finite patch would already appear as a conflict
on these blobs; none does, at any depth tested.

The structural reason is [BGS23] Lemma 7 (lines 492–514): the metatile
substitution **forces the border** — the level-one supertiles enforce
patches ("consistent pseudo-inflation with a complete 'belt' of positive
thickness", Fig 2) that extend beyond the supertile outline, and under
iteration the determined margin grows as fast as the supertile. Legal
hat-metatile patches are therefore rigidly interconnected across shared
walk edges, which is precisely the connectivity the BFS needs; the
edge-consistency of the λ table across the belt is what the zero
conflict count certifies.

### 2.3 The tile law (shape invariance)

Because the segment delta depends only on (label, direction class), not
on position, every placed metatile of type (ty, k) has a D-image whose
mark ring is **the same polygon up to translation**. Verified exactly in
ℚ(φ) for every type and every rotation present in the depth-3 blob:

```
simplified D-ring of (ty,k,t)  ==  rot_k(cap_out0[ty]) + D(t)
```

with cap_out0 the four CAP prototile outlines (H: 9 marks → 6 corners,
T: 3 → 3, P: 8 → 4, F: 9 → 5; collinear marks sit mid-edge, as the
hats' 13-point rings do). The prototile areas match the paper's
𝒗 = a(9+9φ, 15+27φ, 14+22φ, 15+23φ) exactly in ℚ(φ). So the emitter may
(and does) build each tile polygon directly from the D-values of its own
mark ring — shared mesh vertices yield **bit-identical** f64 coordinates
in adjacent tiles.

### 2.4 Patch agreement (the CAP patch is the D-image of the hat patch)

The CAP tiling's combinatorics coincide with the hat metatile
combinatorics (the deformation changes shapes, not adjacencies). Verified
exactly, level by level, by building the CAP patch through the cap recipe
chain (self-contained, no D-lookups on composed supertile anchors) and
comparing against the D-images of the hat-lineage placements:

- level 1: cap patch₁ placements == D₁-images of the 29 hat RULES
  children (`probe9.py` X2, exact);
- level 2: composed cap content (201 tiles) == D-images of the hat
  level-2 content, exact sorted equality on abcd keys (`probe9.py` X3);
- level 3: composed fine set cap_patch₃ × KEPT₂ × KEPT₁ (1376 raw =
  1376 unique) == D₃-images of the hat collar₃, exact (`probe10.py`
  X3@L3; 22/29 supertile anchor origins independently D₃-verified, the
  other 7 are non-mesh scaffolding anchors — see §2.5).

The level-3 collar census is (H,T,P,F) = (459, 67, 325, 525); both
collar batteries show a **single closed boundary walk**, zero pairwise
overlaps, and exact areas (hat space 28825 A′; cap space 18011.918288 in
ℚ(φ) at level 3; 4209 / 2629.153462 at level 2). Orientation census: all
six rotations of every type appear, no reflections.

### 2.5 Why the pseudo-inflation does not obstruct emission (the dead model is not used)

The predecessor brief's model — tree-expanding the CAP substitution with
fixed-shape children — is proven impossible (predecessor STOP evidence;
[BGS23] Lemma 7: "children do not tile the parent outline"). This design
does not use it. What we emit is the **level-0 tile set of the actual
tiling**: the D-images of the metatiles that survive in legal patches
(the KEPT content). The recipe outlines of higher levels are
scaffolding for rule alignment only — their shapes are not φ²-similar
to anything (`try_match_scaled` = NO at levels 1–3, by exact check) and
the supertile content does not fill its outline (the H supertile is a
triangle outline containing 1 H metatile of content area 201 A′ <
33φ⁴ ≈ 226 — the "fjords"). None of that matters for the emitter: the
hat-lineage machinery already produces the correct level-0 metatile set
at every depth (its level-1 content is the paper's own Fig 2 assignment,
"the original one from [17]"), the belt guarantees connectivity (§2.2),
and the deformation maps that set to the CAP tiles. Supertile anchors at
depth ≥ 1 are generally **not** walk-mark vertices (their recipe
outlines have no corner at the frame origin) — the emitter never
D-images them; only level-0 metatile origins are mesh vertices (mark 0
of each type's walk is (0,0)).

## 3. Emission architecture (query-driven, mirrors `hat.rs`)

The plumbing reuses hat.rs's proven cascade; only the tile production
step differs (no demote — metatiles are the tiles — plus the deformation
map).

```
generate(params, query):
  1. region  := bbox of inv(query corners)            (tiling space)
     fail "empty view" if degenerate
     fail "too many tiles…" if area > MAX_TILES·A_MIN, A_MIN = area(T)
       = (9√3/4)φ² ≈ 10.20  (smallest CAP tile; cf. hat.rs's 16H)
  2. (a,b)-window of the region (+ metatile pad 13, mark-ring bbox span
     ≤ 12), center (ca,cb), hex radius, 96-grid anchor, bucket m,
     wedge candidates (7m/10,−7m/10),(m,0),(7m/5,−7m/5),(2m,0),(14m/5,−14m/5)
     — identical to hat.rs (the blob sits up-left of its origin tile in
     the same wedge for both lineages; measured: origin tile at the
     max-a/min-b corner of the blob bbox)
  3. for disp_coef in (0.35, 1.0):                     (see §5 displacement)
       for each candidate seed:
         for depth in 3..=6:
           blob := KEPT-rule descent of the depth-d H lineage (RULES/KEPT
                   tables, integer placements), window-culled
           BFS the walk-mark mesh of the culled blob from the culled tile
             nearest the view center (deterministic tie-break), gauge
             D(anchor) = anchor; conflicts must be 0, else next
           emit tiles: polygon := D-images of the tile's mark ring
             (§2.3), aabb-cull against the region (D-image aabb!)
           accept iff Σ clipped_area(poly, region) ≥ want·(1−1e−9)
             — exact acceptance catches gaps AND overlaps at once
  4. transform to world, aabb-cull against query.region, proto/seed
     per tile, post-check tiles.len() ≤ MAX_TILES
```

**Termination / budget.** The blob reach (max hex-distance of metatile
origins from the blob's origin tile) is measured at E₃=148, E₄=410,
E₅=1096, E₆=2892 — growth ≈ φ² per level (the LIDS scaling), and the
depth-(d+1) blob contains the depth-d one at fixed positions, so
acceptance is monotone in depth. A square view of side S has hex radius
0.7887·S and its first candidate needs reach ≈ 1.7·radius + 85 + pads;
at depth 6 this succeeds for S ≤ ~2080, while the tile budget caps S at
√(250 000 · 10.20) ≈ 1597. **Every view within the budget terminates at
depth ≤ 6**; larger views return the standard "too many tiles for this
view — zoom in" message (the same ceiling behaviour hat.rs has).
Depth-6 blob: 142 129 metatiles (census §1) — the expansion is
integer-exact and cached per generate call as hat.rs does.

**Determinism.** Same params + query ⇒ same region, anchor, bucket,
candidate list, blob, culled order, BFS anchor tile (total order on
(distance², type, k, position)), same D-map values (conflicts = 0 makes
the value unique, the traversal order is irrelevant), same emission
order (the culled blob order) ⇒ bit-identical output. Verified by
regenerating every battery region twice and comparing all placements and
all polygon floats exactly (`probe11.py`).

**Rotated/scaled queries.** Handled exactly as hat.rs: the tiling-space
region is the inverse image of the world query box under the similarity
map (scale then rotate); all selection happens in tiling space; the
final polygons are transformed forward and culled against the world box.
Verified with scale 1.3, rotation 17° (`probe11.py` R4).

## 4. The gauge (deliberate design decision)

The deformation map is translation-covariant: anchoring the BFS at t₀
with D(t₀) = t₀ yields D_{t₀}(z) = D₀(z − t₀) + t₀ for the canonical
D₀ (D₀(0) = 0). The emitter fixes the gauge **per generation** at the
culled tile nearest the view center, which keeps the displacement small
near the view (§5) and the cull window tight for views arbitrarily far
from the origin (the battery's R2 at x ≈ 330 exercises this; an absolute
gauge would need a 0.25·|t|-growing window and is unusable at distance).
Consequence: panning across an anchor change translates the emitted
patch by a metatile-scale gauge vector — a valid translate of the same
tiling (the hull is translation-invariant), with coverage, exactness and
determinism unaffected. This is documented behaviour of the family
member, not a defect: any query-driven emitter of a deformation-defined
tiling must choose a gauge per query.

## 5. The displacement law and the padded window

The deformation is a homogenized linear field locally: |D(t) − t| ≲
0.25·|t| measured against the BFS anchor (global max ratio ≈ 0.25 at
distances 320–340; near the anchor the absolute displacement is small).
Two consequences, both in the emitter:

1. the cull window must be padded by the *displacement* of the farthest
   culled tile, not a constant: `disp_pad = coef·radius + 2` with
   coef 0.35 on the first sweep and 1.0 on the retry (the exact
   acceptance makes any shortfall merely a rejection, never a wrong
   tile);
2. the BFS domain is the whole culled blob (its mesh is connected by
   the belt, §2.2); a culled tile outside the BFS component is skipped,
   and if it intersected the region the acceptance would fail and force
   escalation — rejection, never corruption.

## 6. Cut-and-project classification pins (documentation context)

These paper facts anchor the tiling's identity and the per-tiling doc;
the emitter itself needs none of them (it realizes the deformation
directly):

- control points (Fig 6, lines 965–975): relative to the marked corners,
  at iφξ (T), −iφξ (H), iφ (P), iφ (F); "Apart from the H-tiles, the
  control points are outside their tile, but never coincide with a
  control point of a neighboring tile"; control points of a patch occur
  in triples inside H tiles (Fig 7);
- return module R = t₀·ℤ[ξ,φ] with t₀ = 3φ+2−ξ = φ²(1+ξ)(φ−ξ) (eq. 15,
  line 1062) — invariant under inflation; return vectors generated by
  φt₀, t₀, φξt₀, ξt₀;
- CPS (eq. 16, lines 1180–1210): lattice ℒ = Minkowski embedding of
  ℤ[ξ,φ] (basis {(φ,1−φ), (1,1), (φξ,(1−φ)ξ⁵), (ξ,ξ⁵)}), ⋆ = (φ ↦ φ′,
  ξ ↦ ξ̄); unit-cell volume 15/4, dens(ℒ) = 4/15; window = regular
  hexagon of edge φ, area (3/2)φ²√3; model-set density
  ρ₁ = (2/5)√3φ² equals the tiling's own density
  ρ₂ = |3φ+2−ξ|²/(𝒗·𝒇) = (2/5)√3φ² — the paper's consistency check
  (Remark 12), confirming the CPS lattice choice;
- per-type windows Wᵢ from the displacement-matrix IFS
  Wᵢ = ⋃ⱼ φ⁻²Wⱼ + Dᵢⱼ^⋆ (partly fractal boundaries, Koch type,
  d_H = log(2+√3)/(2 log φ) ≈ 1.36837649, eq. 17);
- pure point spectrum via the Akiyama–Lee refinement of Solomyak's
  overlap algorithm (lines 1090–1105) + the density proof of Lemma 10.

## 7. Relationship to `hat.rs` (reused vs new)

Reused unchanged in structure: similarity-map region handling, the
96-grid anchor + wedge candidates, the depth escalation 3..=6, the
window prune with positive pads, the exact clipped-area acceptance
(≥ want·(1−1e−9)), the two-stage cull (tiling-space region, then world
query box), the MAX_TILES precheck/postcheck, and the RULES/KEPT
level-building + lineage collect (hat.rs's `build_levels`/`collect_metas`
produce exactly the blobs used by the batteries — the depth-d hat counts
4, 25, 169, 1156, 7921, 54 289 hats reproduce from our metatile censuses
10, 64, 442, 3025, 20 737, 142 129 via the fixed hat-per-metatile
dissections 4/1/2/2).

New in `cap.rs`: the walk tables and λ table (§2.1), the integer abcd
BFS deformation map with the local gauge (§4), mark-ring emission from
D-values (§2.3), the displacement-padded window (§5), CAP prototiles and
seeds, and the CAP-specific constants (A_min budget, pad 13).

Prototile indexing: `proto` 0..3 = (H, T, P, F) — hat.rs's type order
(the paper's face order permuted so the lineage code is shared verbatim).
Pattern seeds follow the largest-first convention: H (largest,
area 25.41) → 0, F (22.61) → 1, P (21.48) → 2, T (10.20) → 3.

## 8. Verification plan (the Rust mirror of the spike battery)

1. `cap_walk_tables_match_hat_boundary` — the walk tables close and
   match hat.rs's `BOUNDARY` (marks, labels, lengths, order).
2. `cap_outline_geometry` — the four mark rings: CCW, exact areas vs 𝒗
   (1e-9), corner counts 6/3/4/5, for all 24 (type, rotation) rings.
3. `cap_deformation_consistent_depth3` — depth-3 blob: BFS conflicts == 0,
   census (H,T,P,F) = (147, 22, 105, 168) over the 442 metatiles, tile
   law of §2.3 for every instance (exact in `i64` abcd).
4. `cap_blob_patch_is_legal_depth2` — pairwise convex-clip overlap census
   (< 1e-9) on the depth-2 (64-tile) blob D-images; exact Σ area == the
   census total; 210 complementary shared-walk segments.
5. `cap_blob_nested_across_depths` + `cap_blob_count_table` — the
   depth-d blob is a subset of depth-(d+1); census 10/64/442/3025/
   20737/142129 with the implied hat counts 25/169/1156/7921/54289.
6. `cap_covers_region` / `cap_covers_large_and_distant_views` /
   `cap_rotated_scaled` / `cap_deterministic` — the standard trio
   (`check_coverage` two-sided, `check_deterministic`) on hat.rs-scale
   regions plus far views (x ≈ 483) exercising the local gauge.
7. `cap_orientation_census` — all 6 rotations of all 4 types inside one
   generated view; each emitted tile matches exactly one of the 24 base
   rings (no reflections).

## 9. Result summary (evidence index)

- `.scratch/cap2/results.txt` — Part 1 spike: closure/convexity/area
  exactness; level-1 patch exact; kept unions connected, single boundary
  walks.
- `results9.txt` — D₁ conflicts 0; X2 exact; H1/H2 batteries (201 tiles,
  0 overlaps, area 4209 A′ exact, cap 2629.153462 exact); B2/D1∩D2
  agreement; C1 697 collinear marks; PF eigenvalue φ⁴.
- `results10.txt` — level-3 collar (1376 tiles, census (459,67,325,525)),
  D₃ (5000 vertices, 0 conflicts, integral), displacement law, C3
  batteries (0 overlaps, areas exact, single walk, 4777 collinear
  marks), X3@L3 exact composition, orientation census (6 per type).
- `results11.txt` — emission battery: shape tie-in (origin H ring ==
  cap_out0[H]); R1/R2/R3/R4 all green (coverage grids + 400 random
  points, clipped-area equality rel ≤ 1e-6 — measured 0.0, culling
  exactness, bit-exact determinism, all-orientation census on R3, far
  lineage R2, scale+rotation R4); shape-law addendum for all four types
  and all rotations.
- depth-6 measurement (this document §3): 142 129 metatiles, reach 2892,
  census matches 𝒇 to 5 decimals.