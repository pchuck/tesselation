# Medallion interface — design doc

Status: LANDED — the schema, the five exact catalog re-derivations and
their batteries ship together with this doc
(`crates/tess-tilings/src/medallion/`, data + tests only, zero registry
impact: built-ins stay 165, memberships 186). Inference (placing new
medallions, absorbing new ray sets — §24b) is explicitly out of scope;
§7 states what 24b consumes first. Companion spikes (stdlib Python, all
assertions green): `.scratch/medallion-interface/` — `spike.py` (schema
+ binding), `ray_tables.py` (exit-ray tables), `landing_a.py` (path-(a)
landing identities on the network patch), plus the per-instance
re-derivations `m53_khatam.py`, `m56_rosette.py`, `m161_decagon.py`,
`hexagram_star.py`, `wis_medallion.py` and the exact field layer
`xfield.py`. **[P]** = proven by exact computation in the spikes/tests
below (the assertions are the executable proof); **[C]** = cited
(shipped code or literature).

---

## 0. One-paragraph summary

A **medallion** is a self-contained high-degree ornament pinned by a
five-part **exact interface record**: a planar map drawn on a declared
**convex hull** (vertices in the instance's exact number field,
normalized so vertex 0 is the origin and edge 0 points along +x, CCW),
**boundary contacts** (every figure strand meets the hull at a declared
corner or at an exact edge parameter t ∈ (0,1)), the **internal figure**
(exact segments whose endpoints are exactly the contacts), **exit rays**
(two directed exact direction vectors per contact, the figure continued
outward), and a declared **cyclic symmetry + absorption path** — which
of the three landing mechanisms (M163 extension-operator identity,
M94-style boundary inference, direct interstitial continuation) the
surrounding pattern uses to swallow those rays. Everything is exact:
coordinates in `Q(√2)`, `Q(√3)` or `Q(2cos 18°)`; *angles never appear
as square roots or floats* — every angle claim is a `cos²` identity or a
zero cross product **[P]**. Because the contact t-parameters and all
`cos²` data are **similarity invariants**, a medallion binds to a host
hole (its declared convex hull scaled to the hole) without ever
computing the scale factor in the field; the spike binds the WIS 12-point
star to a unit-dodecagon hole of an exact 3.12.12 patch and absorbs all
24 exit rays with exact hit parameters **[P]**. The catalog of five
re-derived instances (§4) is the ground truth 24b must reproduce; all
five agree with the landed module geometry (exactly where the landed
data is exact — e.g. the M161 hull vs `DEC_V` — and to 1e-9 where the
registry carries f64) **[P]**.

---

## 1. The interface record

### 1.1 Schema

```text
Medallion :=
  id            string                     (registry or catalog label)
  field         Q(√2) | Q(√3) | L10 = Q(a), a = 2cos(π/10) | WIS-Z[√3]
  hull          exact polygon, normalized (V0 = origin, edge0 = +x, CCW)
  hull_edgesq   exact — squared hull edge length (scale datum)
  sym           cyclic symmetry order (rotation about the centre)
  contacts      [(Corner i) | (Edge i, t)], t exact in (0,1)
  figure        [(a, b)] exact segments; endpoints == contacts exactly
  exits         [(contact i, exact direction vector)] — 2 per contact,
                both outward
  path          (a) | (b) | (c)            — the absorption obligation
```

The Rust module (`crates/tess-tilings/src/medallion/`) mirrors this:
`Field`, `Contact`, `Absorption` enums + the `Medallion` trait (ID,
field, symmetry order, path, f64 views for consumers: `hull_f64`, `exit_rays_f64`,
`contact_cos2`), with
the exact integer tables per instance. Coordinate storage follows the
shipped conventions **[C: fields.rs, wis_12_fold.rs]**:

* `Q(√2)`, `Q(√3)` — element `(a, b)` with value `(a + b√d)/den`, `i64`
  numerators, per-record global denominator (2 for the octagon, 4 for
  the dodecagon and hexagram hulls). All arithmetic in the batteries is
  *polynomial* (norm, cross, dot, shoelace — no division), so a single
  exact sign rule suffices: `sgn(a + b√d)` by comparing `a²` against
  `d·b²` in i128 (the shipped quadratic `zcmp` pattern).
* `L10` — `[i64; 4]` at denominator 64, `a⁴ = 5a² − 5`, consumed through
  the shipped `fields::L10` (add/sub/mul/sign/float) — the girih layer
  **[C: M139]**.
* WIS — `Z[√3]` 4-tuples `(p₀,p₁,p₂,p₃)` meaning
  `((p₀ + p₁√3)/2, (p₂ + p₃√3)/2)` — the shipped `P4` world frame,
  world = raw/2 **[C: wis_12_fold.rs]**.

### 1.2 Normalization convention **[P]**

Hull vertex 0 is the origin; edge 0 points along +x; vertices CCW. This
choice is *load-bearing for exactness*: every catalog instance then has
all coordinates in its declared field with a small fixed denominator —
no square root of a field element ever needs to be extracted (a general
"centroid at origin" normalization would force halving rotated
coordinates; a "tip-0-at-angle-0" normalization would force sin/cos of
15°, which is outside `Q(√3)` — the WIS tips sit at 15° + 30k). The
scale datum lives in `hull_edgesq` (recorded per instance), *not* in the
coordinates' size.

### 1.3 Angle convention: squared norms and cos² only **[P]**

No square root appears in any record or check. Angle facts are carried
as:

* direction vectors (exact, in the field) — collinearity is `cross == 0`
  exactly, perpendicularity `dot == 0` exactly;
* `cos²` of an angle (invariant under direction sign, so it expresses
  undirected chord/contact angles without choosing an orientation);
* squared lengths (`edgesq`, circumradius², hit-parameter numerators).

The cos² values the catalog uses, with their exact field representations:

| value | field | representation | where |
|---|---|---|---|
| `1/2` | `Q(√2)` | 45° | M53 contact angle |
| `1/4`, `3/4` | `Q(√3)` | 60°/30° | M56 contact, hexagram exits |
| `(5 − a²)/4` | `L10` | cos² 54° | M161 contact angle |
| `a²/4` | `L10` | cos² 18° | M161 exit rays |
| `2 + √3`, `2 − √3` | `Q(√3)` | squared radii | WIS tips |

### 1.4 Contact and exit semantics **[P]**

* `Corner i` — the figure strand terminates *at* hull vertex i (star
  tips: hexagram, WIS).
* `Edge i, t` — the strand meets the interior of hull edge i at the
  exact parameter t; the catalog uses t = 1/2 throughout (PIC contact
  midpoints).
* **Invariant (checked for all five instances):** every contact equals
  some figure endpoint exactly, and vice versa — the figure adds no
  free-floating endpoints.
* **Exits:** each contact carries exactly 2 directed rays, the figure
  chords continued past the contact; outwardness is the exact check
  `dot(d, contact − centre) > 0`.

---

## 2. Placement contract

**Scale policy: the hole is primary.** The host declares a hole polygon
(its own normalized frame) plus the adjacent tiles it will use for ray
absorption; the medallion's hull is mapped *onto* the hole by a
similarity. Justification: the host lattice geometry is already
certified exact by its own family module, while a medallion arrives as
decoration data; sizing the hole to the medallion would invalidate the
host's certified coordinates. The scale factor `s² = hole_edgesq /
medallion_edgesq` is only ever applied at final emission — the binding
itself consumes only similarity invariants (t-parameters, `cos²` data,
angle grids), so it stays exact even when `s` is a square root outside
the medallion field.

**Binding.** A binding is a similarity S (rotation + translation, CCW
preserved) sending hull vertex/edge 0 to the declared hole
vertex/edge 0. For a regular hull it is determined by the image of edge
0. Spike demonstration **[P]** (§6): WIS 12-star onto a unit-dodecagon
hole w₀…w₁₁ of an exact 3.12.12 local patch (w_k = rot30^k(w₀),
w₀ = ((2+√3)/2, ½), radius² 2 + √3): hole edge 0 sits at direction 120°,
so `S = rot30⁴ ∘ (V₀ ↦ w₀)`; `S(V_i) = w_i` holds for all 12 vertices
exactly, and the medallion centre lands exactly on the hole centre.

**Invariance lemma (why one alignment demonstration suffices) [P].** If
the medallion's exit directions and the hole's edge directions both live
in one angle grid (here: multiples of 30°), then the relative
ray/tile configuration depends on the contact-offset only through a
rotation *within the grid* — binding at offset j₀ + m is the m·30°
rotation of the binding at j₀. Both facts are exact for the WIS record:
its 24 exit directions are the 30° multiples (ray frame = edge frame),
and the hole's edge directions are the 30° multiples rotated by 120°.
The spike verifies the absorption for one alignment and the 12-fold
cyclic symmetry of record + patch carries the rest.

---

## 3. Ray-landing obligations

An exit ray must be **absorbable** by the surrounding pattern through
one of three declared mechanisms:

* **(a) M163 extension-operator landing identity** **[C:
  families/islamic/hankin_motifs.rs]**. The ray, continued outward,
  lands exactly on the contact midpoint of a subsequent tile — the ray
  line is a union of network wall-chords, so the extension operator
  walks it. *Certificate:* every exit ray line carries further contact
  midpoints ahead; the first landing parameter t (measured in units of
  |d|², the chord squared length — a similarity ratio) is exact.
* **(b) M94-style inference on boundary cells** **[C:
  families/islamic/hankin.rs]**. The ray enters a boundary panel whose
  motif content is catalogued data (the girih patch's BOW/HEX panels);
  absorption is by motif inference, not midpoint landing. *Certificate
  (24b):* panel assignment + entry/exit parameters recovered from the
  panel data.
* **(c) Direct interstitial continuation.** The declared interstitial
  tiles are known exactly; absorption is a ray-shooting computation.
  *Certificate:* every exit ray has a first hit on a declared tile with
  exact parameter.

Catalog assignment — and what is *verified* here vs deferred to 24b:

| instance | path | 24a verification |
|---|---|---|
| M53 khatam octagon | (a) | **[P]** all 16 exit rays land on network contact midpoints, patch 3, first-landing t multiset `{√2−1: 8, 1: 8}` |
| M56 {12/4} rosette | (a) | **[P]** all 24 exit rays land on network contact midpoints, patch 3, first-landing t multiset `{2√3/3−1: 12, 1: 12}` |
| {6/2} hexagram | (a) | **[P]** chord-line landing: every chord line of the star is also a chord line of the stars translated by ±a₁, ±a₂, ±(a₂−a₁) (exact line identity: direction cross = 0 + invariant c²·norm²) |
| M161 girih decagon | (b) | obligation pinned (contact data + panel types BOW/HEX); landing verification is a 24b job by design |
| WIS 12-star | (c) | **[P]** all 24 exit rays absorbed by the 12 declared adjacent tiles of the 3.12.12 patch: first-hit t multiset `{1+√3: 12, 2+√3: 12}`, all 24 land exactly on *vertices* of the six neighbour dodecagons (4 rays each, 0 on the triangles) |

---

## 4. Catalog ground truth

These are the five records 24b must reproduce. Coordinates are the
normalized-frame tables shipped in
`crates/tess-tilings/src/medallion/` (same tables; the Rust module
carries the machine-readable copy).

### 4.1 Khatam octagon — `pic-4-8-8` (M53) **[P]**

* Field `Q(√2)`; hull = regular octagon, edgesq 1, area `2 + 2√2`
  (2-area `4 + 4√2`), sym 8. Hull table: numerators `(a + b√2, c + d√2)`
  over 2 — see `medallion/khatam_m53.rs` (8 rows). The record hull is
  the substrate octagon of the network: it fits the landed
  `truncated_square` octagon at 1e-9, and its edge midpoints are exactly
  the PIC wall contacts of the landed `pic_4_8_8` octagon cell
  **[C: archimedean.rs, pic_4_8_8.rs]**.
* Contacts: `Edge i, 1/2` for all 8 edges. Figure: the 8 wall chords
  `M_i → M_{i+2}` (indices mod 8) — the PIC δ = 45° rule, derived by
  ray-marching from each midpoint and *asserting* every exit is exactly
  a midpoint (the closing condition).
* Contact angle: every chord meets its wall at `cos² = 1/2` exactly
  (single-valued, all 8 contacts).
* Exits: 16 directed, directions `0°, 45°, …, 315°` (2 each);
  cos²-with-x multiset `{0: 4, 1/2: 8, 1: 4}`.
* Path (a): landing on network midpoints verified patch-3, first-landing
  t `{√2−1: 8, 1: 8}`.
* Re-derivation cross-check vs landed: full PIC network over a 5×5-tile
  substrate patch → 300 deduplicated chords → 322 arrangement faces;
  the 14 faces with centroid in the fundamental cell match the landed
  `pic_4_8_8` cell *exactly* — area multiset and vertex multiset:
  1 inner octagon (bounded by the 8 wall chords) `1+√2`, 1 square `1/2`,
  8 triangles `1/8`, 4 pentagons `1/8 + √2/4`; total `3 + 2√2` = the
  cell area `A²`, `A = 1 + √2`.

### 4.2 {12/4} rosette — `pic-3-12-12` core (M56) **[P]**

* Field `Q(√3)`; hull = regular dodecagon, edgesq 1, area `6 + 3√3`,
  sym 12 (`medallion/rosette_m56.rs`, 12 rows at `/4`).
* Substrate facts (used by the binding demo, §4.5): at the 3.12.12
  vertex figure, each dodecagon strictly alternates 6 triangle-edges
  and 6 dodecagon-edges; dodecagon centres form the triangular lattice
  generated by `(2+√3)e^{±i60}`; cell area `6 + 7√3/2` = 1 dodecagon +
  2 triangles exactly.
* Contacts: `Edge i, 1/2` × 12. Figure: the 12 chords `M_i → M_{i+4}` —
  the δ = 60° PIC rule (ray-march + closing assertion).
* Contact angle: `cos² = 1/4` at every contact.
* Exits: 24 directed, directions `30k`, cos² multiset
  `{0: 4, 1/4: 8, 3/4: 8, 1: 4}`.
* Path (a): landing verified patch-3, first-landing t
  `{2√3/3 − 1: 12, 1: 12}`.
* Cross-check vs landed: 5×5 patch network → 510 chords → 1107 faces;
  the 45 cell faces match the landed `pic_3_12_12` cells: exact area
  multiset, exact total area `6 + 7√3/2`, and exact vertex multiset
  after the landed frame's 30° rotation. Vertex angle census
  `{30: 24, 60: 42, 90: 48, 120: 42, 150: 24}`.

### 4.3 Girih decagon patch panel (M161) **[P]**

* Field `L10 = Q(a)`, `a = 2cos 18°`, `a⁴ = 5a² − 5`, denominator 64 —
  the shipped girih field **[C: fields.rs L10]**. Hull = regular
  decagon, edgesq 1, 2-area `5a³ − 10a` (area `(5/2)a³ − 5a =
  (5/2)√(5 + 2√5) ≈ 7.6942`).
* Contacts: `Edge i, 1/2` × 10. Figure: the 10 {10/3} strap chords
  `M_i → M_{i±3}` (10 undirected chords = 20 directed contact pairs —
  exactly the girih strap of the landed decagon panel).
* Contact angle: `cos² = (5 − a²)/4` at every contact, single-valued.
* Exits: 20 directed, directions `18° + 36k`; cos² multiset
  `{0: 4, (5−a²)/4: 8, a²/4: 8}`.
* Internal arrangement: the 10-chord strap + boundary dissect the
  decagon into 31 faces — 1 central decagon of area `5a³/4 − 25a/8`,
  two quadrilateral orbits of 10 with areas `3a³/8 − 5a/4` and `a/8`,
  and 10 triangles of area `15a/16 − a³/4`; the exact sum is
  `(5/2)a³ − 5a` = the decagon area **[P: `m161_decagon.py`]**.
* Cross-check vs landed: the normalized hull coincides **exactly** with
  the landed `DEC_V` table (after rotation 3·36° and alignment at
  vertex 0) — an exact tuple match in L10, stronger than the 1e-9
  requirement **[C: girih_decagon_patch.rs DEC_V]**.
* Path (b): the decagon sits in the landed patch surrounded by BOW/HEX
  panels; this instance pins the *obligation* — 24b must reproduce the
  ray → panel entry/exit inference. (24a deliberately does not verify
  it; no fudge.)

### 4.4 {6/2} hexagram, kissing radius T = 1/2 **[P]**

*Interpretation note:* the catalog's "star-and-cross" entry is the
landed **{6/2} family** `hexagram.rs`, evaluated at its maximal (kissing)
parameter T = 1/2 — the measurement-derived M35 `girih-star-cross`
artwork is a different, measured object; this record re-derives the
exact family instance. **[C: families/islamic/hexagram.rs, T ∈
[0.05, 1/2], default = 0.5]**
* Field `Q(√3)`; hull = the tip hexagon, edgesq `1/4`, sym 6.
* Contacts: `Corner i` for all 6 hull vertices (the star tips — at
  T = 1/2 each tip is exactly a lattice-edge midpoint, the kiss point
  with a neighbouring hexagon).
* Figure: the 6 sides of the two tip triangles `{0,2,4}`, `{1,3,5}` —
  i.e. chords `V_i → V_{i+2}`, which *is* the {6/2} star polygon
  drawn as two triangles. The central hexagon (chord intersections) has
  area `√3/8` (circumradius² `1/12`); the star area is `√3·T² = √3/4`;
  the fundamental cell (lattice det = `√3/2`) satisfies
  `cell = star + 2·hexagon` exactly.
* Exits: 12 directed, directions `30° + 60k` (2 each); cos² multiset
  `{3/4: 8, 0: 4}`.
* Path (a): chord-line landing verified exactly — every chord line of
  the origin star repeats as a chord line of the stars translated by
  `±a₁, ±a₂, ±(a₂−a₁)` (line identity via direction cross = 0 plus the
  invariant `c²·norm²`). Additionally the exit-ray lines sit at distance
  exactly `1/4` from each neighbouring star centre = the chord distance
  `R/2` — the rays are tangents of the neighbour's chord circle, i.e.
  they are *also* interstitial-ready (path (c) data).

### 4.5 WIS 12-point star (M158) **[P]**

*Honesty rule:* the assembled 12-rho star's occurrence inside the
[WSI95] dodecagonal tiling is **not certified** by landed data — the
shipped 26-tile patch census contains no 12-rho vertex. The ground
truth of this instance is its own exact geometry, cross-checked against
the shipped prototile data (`RHO_CORN`, `rot4`, edge directions, area
table) **[C: families/quasiperiodic/wis_12_fold.rs]**.
* Construction: the 12 rho prototiles (30°/150°, unit edges) sharing
  their acute corner at one point. Corner-sum census (30° units):
  `{12: centre ×1, 10: notches ×12, 1: tips ×12}` — 360° at the centre,
  300° at each notch, 30° at each tip, exactly.
* The star *outline* is a 24-gon (12 tips + 12 notches), **all 24 edges
  unit**; star area 6 (2-area 12). The declared **convex hull** is the
  regular dodecagon through the 12 tips: unit edges, circumradius²
  `2 + √3`, 2-area `12 + 6√3`. The 12 notches lie strictly inside the
  hull, and the 12 gap triangles (hull minus star) are unit equilateral
  with the exact identity `hull₂ = star₂ + 12·gap₂`:
  `(12, 6) = (12, 0) + (0, 6)` in `(a, b)` for `a + b√3`.
* Field/lattice: `Z[√3]` 4-tuples (shipped `P4`, world = raw/2); the
  normalized record (rot 0, tip cycle start 8) is in
  `medallion/wis_m158.rs` — hull 12 rows, centre `(1,0,2,1)`, 12 notch
  rows. Tips: the norm-`(8,4)` corners of the 12 `rot4` orbits of
  `RHO_CORN`; notches: norm-`(4,0)` corners — recomputed from the
  shipped tables inside the Rust test battery, not transcribed.
* Contacts: `Corner i` × 12 (the tips; notches are *interior* and are
  not boundary contacts). Figure: the 24 unit outline edges
  (tip–notch segments).
* Exits: 24 directed = the outline-edge continuations at the tips; in
  the record frame the directions are the multiples of 30° (2 each, on
  the 6 undirected grid lines); cos² multiset
  `{0: 4, 1/4: 8, 3/4: 8, 1: 4}`. (The radial tip lines 15° + 30k and
  notch lines 30k are internal-figure data only — kept in the module
  as the rho-edge/radius facts, not as exit rays.)
* sym 12 (cyclic, exact `rot4` on raw tuples).
* Path (c), spike-verified: bound to the unit-dodecagon hole of the
  exact 3.12.12 patch (§2, §6): binding `rot30⁴ + V₀ ↦ w₀` exact; all
  24 exit rays absorbed by the 12 declared adjacent tiles; first-hit
  t multiset `{1+√3: 12, 2+√3: 12}`; **all 24 first hits land exactly on
  vertices of the six neighbour dodecagons** (4 rays per dodecagon,
  none on the six triangles).

---

## 5. Geometry-agnosticism flags

What survives in absolute geometry (22f) and what does not:

* **Absolute (similarity invariants — safe in any frame):** contact
  t-parameters; all `cos²` angle data (contact angles, exit-ray angles
  *relative to the hull edge grid*); the first-hit t parameters of
  absorption certificates (ratios along rays, measured in units of the
  chord length); area *ratios*; the absorption census; convexity, CCW
  and cyclicity assertions (affine-invariant classes).
* **Not absolute:** coordinates (only the normalized frame is
  meaningful); `hull_edgesq` itself (the scale datum — it transforms as
  s²); distances in field units; the *absolute* direction vectors of
  exit rays (they are absolute within the placement, and the placement
  contract fixes that frame).
* **Euclidean assumptions, flagged:** (i) angles — the whole angle
  layer is Euclidean `cos²`; in the absolute-geometry program these are
  re-read via Θ (the mitre formula precedent: contact angles are
  absolute *through* Θ **[C: docs/ROADMAP.md §22f]**), the coordinates
  are not; (ii) rays are straight lines and absorption is first-hit —
  both are straightedge facts, no circle functions are used anywhere
  (circumradius² appears only as a squared-norm identity); (iii) the
  binding rotation must live in the host × medallion field — always
  satisfied in the catalog (all rotations here are grid multiples
  already in the instance field).
* No degree-5 or non-shipped field is required by any instance — the
  STOP rule (c) did not fire: L10 (degree 4, shipped) covers the girih
  decagon; `Q(√2)`, `Q(√3)` cover the rest.

---

## 6. Spike numbers **[P]**

`spike.py` (schema + binding, exit 0):

* schema invariants per instance (CCW, strict convexity, V0/edge0
  normalization, equal hull edges, contact↔chord-endpoint coincidence,
  exits outward, 2 exits per contact): **88/88 pass** — 16 (M53) + 20
  (M56) + 18 (M161) + 14 (hexagram) + 20 (WIS);
* single-valued contact angles: `1/2` (M53), `1/4` (M56),
  `K10(5/4, 0, −1/4, 0) = (5−a²)/4` (M161) — exact;
* WIS hull recomputed from shipped `RHO_CORN`/`rot4` reproduces the
  pinned table exactly (regression assert);
* binding: `S(V_i) = w_i`, all 12 vertices, exact; centre → hole centre
  `(0,0)` exactly;
* absorption: 24/24 rays absorbed; first-hit t multiset `{1+√3: 12,
  2+√3: 12}`; all 24 hits are exact dodecagon vertices; tile census
  4 rays × 6 dodecagons, 0 × 6 triangles; bound exit-ray directions:
  the 12 multiples of 30°.

`ray_tables.py`: exit-ray angle censuses + cos² multisets exactly as
tabulated in §4 (assertions green). `landing_a.py`: path-(a) patch
certificates as in §3 (0 failures over 16 + 24 rays; first-landing t
multisets as quoted). Per-instance re-derivations (§4): all exact
matches quoted above (M53 vertex-multiset, M56 vertex-multiset after
30°, M161 exact `DEC_V` match, hexagram kissing-cell identity, WIS
hull/star/gap area identity `(12,6) = (12,0)+(0,6)`).

---

## 7. Module shape and placement — decision: `tess-tilings`

`crates/tess-tilings/src/medallion/` (one `pub mod medallion;` line in
`lib.rs`), **not** `tess-core`. Reasons:

1. **Dependency direction.** The verification batteries must reach the
   landed ground truth: `RHO_CORN`/`rot4` are `pub(crate)` in
   `wis_12_fold`, and the f64 spot-checks call the registered tilings'
   `generate()` (`pic_4_8_8()`, `pic_3_12_12()`, `HexagramTiling`,
   `GirihDecagonPatchTiling`). `tess-core` cannot depend on
   `tess-tilings` — the data would be un-verifiable there.
2. **It is derived data, not an abstraction.** `tess-core` is the
   zero-dependency trait layer; the medallion records are
   family-specific *data* whose meaning (paths (a)/(b)) is defined by
   tess-tilings' M163/M94 machinery. Exact-geometry layers living
   inside `tess-tilings` is the established pattern (`fields.rs`,
   `classify.rs` — cross-cutting *data* module precedent).
3. **Consumer proximity.** 24b's inference engine extends the M163
   extension operator and sits beside the families it consumes.

Layout: `mod.rs` (schema types + exact quadratic-engine helpers + the
common ray/tile exact intersection kernel), one file per instance
(`khatam_m53.rs`, `rosette_m56.rs`, `girih_m161.rs`, `hexagram_62.rs`,
`wis_m158.rs`: tables + `Medallion` impl + battery), `host312.rs` (the
3.12.12 host-hole patch model + binding/absorption tests — the
demonstration 24b starts from). Everything is `pub` = the API surface
24b consumes; zero registry/census impact (no `Tiling`, no registration).

**What 24b consumes first:** (1) `host312` + the WIS record — the
already-verified path-(c) pipeline (bind → shoot 24 rays → first-hit
certificate) is the template for the absorption engine; (2) the path-(a)
records (M53/M56/hexagram) with their first-landing certificates — wire
to the M163 extension operator; (3) the M161 record + BOW/HEX panel
tables — the path-(b) inference, which 24a deliberately leaves open.
The `cos²`-multiset convention (§1.3) is the comparison key: 24b's
inferred ray sets must reproduce the §4 multisets, mutation-testable
without floats.

---

## 8. Deviations and STOP-rule status

* **Hexagram interpretation** (§4.4): catalogued as the exact landed
  {6/2} family at T = 1/2 rather than the measured M35 artwork — the
  artwork is f64 measurement data and cannot carry an exact interface
  record; the family instance is the exact object the artwork depicts.
* **WIS honesty rule** (§4.5): the star's tiling-occurrence is not
  certified by landed data; only its exact geometry cross-checked
  against shipped prototile tables is.
* **M161 path (b)** landing verification is deferred to 24b *by design*
  (§4.3) — the brief's path (b) is an inference obligation, not a
  24a computation.
* STOP rule (a) did not fire: corner contacts + exact direction vectors
  + t-parameters + one path enum expressed all five instances with no
  ad-hoc fields (spike: 88/88 schema invariants). STOP rule (b) did not
  fire: all five re-derivations agree with landed ground truth (§4).
  STOP rule (c) did not fire (§5).

## 9. See also

`docs/cyclotomic-field-layer.md` (M137 — the field layer §4.3 leans on),
`docs/isohedral-generator-design.md` (M162 — the design-doc-first
pattern), `docs/tilings/pic-4-8-8.md`, `docs/tilings/pic-3-12-12.md`,
`docs/tilings/hexagram.md`, `docs/families/islamic.md` (the landed
ground truth), `docs/briefs/medallion-interface.md` (this brief).
