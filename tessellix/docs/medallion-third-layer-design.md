# Medallion composition third-layer design — absorbing the medallion layer into the strapwork weave

> Design job (2026-11-26): "Design a method for absorbing a third layer
> into the medallion composition." This doc is **design only — no code
> ships with it**; the M number is assigned at integration. Companion
> surfaces: the M164 rendering docs ([rendering.md](rendering.md)), the
> M57 layer-mode engine (`tess-core::strapwork`), the M167 medallion
> interface ([medallion-interface-design.md](medallion-interface-design.md))
> and the 24c monster catalog (`tess-tilings::medallion::monster`),
> and the M172 composition instance
> (`tess-tilings::families::islamic::dodecagon_hexagram_honeycomb`).
> Two alternative readings — **nested medallions** and the **field
> tower** — are held as backlog variations (§10), not designed here.

## 1. The problem: a two-layer surface and a data-only medallion

`docs/rendering.md` opens: "Two rendering layers on top of the tilings
(`--strapwork` and `--decoration`)". The pipeline (verified at
`crates/tess-tui/src/main.rs:406–434`) is exactly

```text
tiles   = tiling.generate(region)
walls   = strapwork::walls(&tiles, delta)        // or walls_with_delta(f)
svg     = plan_decoration(&walls, ..)  -> render_svg_decorated(region, plan)
        | render_svg_with_walls(&tiles, region, mode, &walls)   // undecorated
```

Layer 1 is the host tiling (tile faces, `--mode` painting); layer 2 is
the Hankin PIC wall network — rays fired at **every tile-edge
midpoint** at contact angle δ, split at crossings, deduplicated on a
1e-6 grid (`strapwork.rs` module docs). The decoration styles
(Kaplan §3.7) are a *paint* stage over the wall map: they consume
`Vec<Wall>` and nothing else (which is why `--decoration` ignores
`--mode` — the plan paints paper + faces + bands itself).

Meanwhile the medallion program (24a catalog M167, 24c monsters M169 +
the n = 11/13 rungs, M163 motif machinery, 24d composition families
M172) is **data-only on the render side**: `medallion/mod.rs` states
"this module is data + tests only — no `Tiling`, no registry entry".
A registered composition like M172 renders as a partition whose
medallion classes are just tile faces; running `--strapwork 60` over
it fires the **generic** midpoint rule at every face — the records'
own declared geometry (contacts, figure chords, exit rays, absorption
path) disappears into the generic weave. There is no surface today
that draws a catalog record's internal figure and exit-ray structure
onto a host, let alone weaves it with the host strapwork under the
four decoration styles.

**The third layer is the medallion layer**: a per-composition source
of wall geometry that comes from *declared medallion records* rather
than the midpoint rule. This doc designs how it is **absorbed** — not
stacked as a separate SVG group — into the existing wall map, so the
decoration gates (`fill` Eulerian, `interlace` two-strand) and paints
run on the *fused* network exactly as they do today.

## 2. Facts the design binds to (code-verified)

F1. `Wall { a, b }` (`strapwork.rs:68`) is the engine's universal
    currency: a plain f64 segment. `build_map` keys vertices on a
    1e-6 grid (`key_pt`, GRID = 1e6), classifies vertices (`VKind`:
    degree-1 end-caps and clipped rims among them), and the
    decoration preconditions are honest errors naming counts and a
    witness point (`DecorationError`).
F2. Rays never cross tiles: a midpoint ray runs **inside its own
    tile** to that tile's boundary and stops; the network is
    continuous across an edge only because the adjacent tile fires
    the point-reflected pair at the shared midpoint — "continuous
    across the edge, straight through at M" (`strapwork.rs` module
    docs). **Consequence: suppression is exact and clipper-free** —
    dropping a tile's ray-firing removes exactly its contribution,
    and continuation through an edge midpoint happens iff the other
    side fires a strand collinear at M (§4.2).
F3. The `Medallion` trait (`medallion/mod.rs:105`) already carries the
    f64 handoff views `hull_f64()` and `exit_rays_f64()`
    (`(contact index, direction)` pairs), plus exact `PATH:
    Absorption` ∈ {LandingIdentity, BoundaryInference, Interstitial}.
    The contact POINTS and figure chords are exact data in the
    instance modules (the figure's endpoints are exactly the
    contacts, doc §1.4) but have no f64 view yet.
F4. `first_hit` (`medallion/mod.rs`, division-free march,
    cross-multiplied parameters) marches an exit ray over a declared
    polygon list and returns the landing tile/edge/parameter; the
    host312 pipeline absorbs the WIS star's 24 rays over the 3.12.12
    patch with the exact first-hit multiset `{1+√3: 12, 2+√3: 12}`,
    all landing on *vertices* of the declared neighbour tiles
    (interface doc §4, table row WIS).
F5. **M163 identity**: `Motif::Star` at θ = d·180/n emits, per hull
    edge midpoint, the two arms `rot(tangent, ±θ)` — the same lines as
    the record's exit rays (cross-checked for all fifteen catalog
    monster records). The record ray-set and the motif star arms are
    one and the same geometry in two emitters.
F6. The layer-mode anchors: at classic δ the generic engine over
    `square`/`hexagon`-family substrates reproduces the registered
    PIC partitions exactly (truncated-square @ 45° = `pic-4-8-8`
    internal edges, etc. — `strapwork_layer_mode` tests), and the
    12-row classic-δ table pins where each style gate passes.
F7. Dependency direction: `tess-strapwork` lives in `tess-core`
    (zero external deps); the medallion records and their exact
    fields live in `tess-tilings`. `tess-core` must never name a
    record, a field, or `cos²` — the layer must cross the boundary as
    **f64 wall segments + a suppression hint** and nothing else.
F8. Strapwork/decoration is currently a TUI export surface only
    (grep: `--strapwork` appears in `tess-tui`, never `tess-gui`).

## 3. Definition of the third layer

A **medallion layer instance** is one placed catalog record on the
host: `(record, center, scale, rotation)`. Its content is three parts
of the five-part interface record:

1. **hull** — the placement footprint (already `hull_f64`);
2. **figure chords** — the exact internal segments whose endpoints
   are exactly the contacts (the `{n/d}` star or khatam join of the
   record);
3. **exit rays** — the directed arms at each contact (already
   `exit_rays_f64`, points from the new contact view), marched into
   the host until their declared absorption landing (F4).

Rendering policy: the figure chords and the marched exit rays are
**walls**. They join the generic wall network in the one `Vec<Wall>`
that `build_map` consumes, and the four styles paint the fused map
with no further knowledge. The hull itself is NOT a wall by default
(it is geometry only: footprint for suppression and the march's
launch polygon); `--decoration outline` then draws the medallion
band's side lines exactly as it draws any strand's.

What the third layer is **not**: not a paint layer (decoration stays
layer 2's paint stage), not a face layer (the interstice fill reading
— dual-net vs inference — stays the tiling generator's business,
24d's verdicts unchanged), not a coordinate system (the field-tower
reading is backlog, §10).

## 4. The absorption method

### 4.1 Where it hooks: per-face routing, one trait default method

The generic firing rule is per-tile (F2), so absorption is a routing
decision **at the tile**, and the data stays on the tilings side:

```text
// tess-core::tiling (default impl — every existing tiling unchanged)
trait Tiling {
    /// Wall-geometry layers this tiling contributes besides the
    /// generic midpoint rule, already in world f64.
    fn extra_walls(&self, region: &BBox) -> Vec<Wall> { vec![] }
    /// Seeds whose generic midpoint firing is suppressed because a
    /// medallion layer replaces them (default: none).
    fn suppressed_seeds(&self) -> Vec<u32> { vec![] }
}
```

and in `strapwork`:

```text
pub fn walls_layered(tiles: &[Tile], delta_deg: f64) -> Vec<Wall>
//  = walls_fired(tiles minus suppressed-seed tiles, δ)
//  ∪ extra  (the caller's third-layer segments)
//  fused through the SAME dedup grid + sort (F1), so output stays
//  deterministic, ordered, direction-independent (module invariant).
```

The TUI pipeline becomes: generate tiles; `extra =
tiling.extra_walls(region)`; if `extra` is non-empty and suppression
is active, use `walls_layered`; everything downstream (gates,
decoration, SVG) is literally unchanged. UI code never learns about
medallions (AGENTS: "adding a tiling never requires touching UI
code" is preserved — default-empty methods, no new widgets). The
suppression + geometry are **derived by the tiling itself** from the
same exact tables that generate the faces — bit-identical placement,
no second source of truth.

Alternative hook shapes rejected (§9.2): a `Layer` object tree in
`tess-core` (over-abstract before a second kind exists); a CLI
`--medallion ID@…` placement language (duplicates placement data that
the composition tiling already owns, and violates the
self-describing-parameter rule for non-composition hosts).

### 4.2 The continuity law at contacts

A suppressed medallion face leaves each of its edge midpoints M
shared with an unsuppressed neighbour **open** unless the medallion
layer fires a strand through M collinear with the neighbour's incoming
ray (F2: "straight through at M"). Hence the fusion law:

> **(L)** For every hull edge midpoint M shared with an emitting
> face: M is a declared contact of the record, and the record's exit
> ray at M meets the shared edge at the same angle δ — i.e. the
> contact angle matches the weave: **θ_record = d·180/n = δ** for the
> monster family (M163 convention). Hull midpoints that are NOT
> contacts (the record may use a subset of the 2n contact sites, and
> corner contacts sit at vertices instead) are closed by rule (§4.3):
> a ray arriving at such an M ends and is caught by the degree-1
> gate — an honest error naming the mismatch, not a silent stub.

By F5 the emitted medallion arms at M are exactly the `Motif::Star`
lines at θ = δ, so when (L) holds the fused strand is the same
straight line the generic pair would have formed — the weave does not
"know" the medallion happened; it only gains the interior chords.

Two regimes fall out, both legal:

* **coincide** — the medallion sits on a substrate where the generic
  weave at δ already draws its exit-ray lines (e.g. the khatam {8/2}
  record on the 4.8.8 host at δ = 45°): suppression + layer emission
  is a *no-op on the ray network* and the layer's payload is the
  figure chords (and the record's identity, for tests). The anchor
  suite (§7, A2) pins this: the fused map at coincidence must equal
  the generic map ∪ chords, segment for segment on the dedup grid.
* **override** — δ ≠ the record's natural δ for some faces, or the
  medallion's hull edges are not host edges at all (a record placed
  in a hole, host312-style): then (L) constrains which δ may be
  requested at all; violations fail the gates, naming the contact.

### 4.3 The march: exit rays to their declared landing

An exit ray is not a full line to the view rim: it is marched
segment-by-segment through the faces it crosses until its declared
absorption landing, and each inter-face hop is one `Wall`:

* path **(a) LandingIdentity** — landing at the contact midpoint of
  a subsequent tile (the M163 extension-operator identity; the
  march's final hop terminates at that midpoint, where the host's own
  firing continues the strand);
* path **(b) BoundaryInference** — landing on a declared boundary
  cell feature (M94-style; the march stops exactly on the declared
  edge, and the closure of the weave across that boundary is the
  24b obligation — the layer emits up to the boundary and the gate
  verifies Eulerian closure of what the composition claims);
* path **(c) Interstitial** — landing on declared neighbour-tile
  vertices/edges (WIS→3.12.12: all 24 rays land exactly on vertices
  of the six neighbour dodecagons; the march is `first_hit`-faithful
  and the landing multiset is the anchor).

The march runs **on the tess-tilings side, exactly** (the records'
own field arithmetic + `first_hit`, division-free), converted to f64
at the end — the same "exactness asserted where the host is exact,
f64 elsewhere" convention the engine already states (`strapwork.rs`
module docs). The design does NOT march in f64: landing sites are
exact points converted once.

The march's polygon list is the tiling's faces in the ray's path
window; a ray whose declared landing cannot be reached (no face
sequence, or overshoot past the view) is a build-time STOP for the
composition author, surfaced by the gate as an open end (§4.4), never
truncated silently.

### 4.4 Gates: the fused map answers the existing questions

No new style, no new gate *logic* — the third layer is in scope of
exactly the checks that exist, which is what makes the absorption
honest:

* `fill` still requires an Eulerian fused map (2- and 4-valent): a
  figure chord ending at a corner contact meeting an odd number of
  generic rays is caught (odd-degree witness error, unchanged code);
* `interlace` still requires every crossing to be exactly
  two-stranded — this now covers chord × ray, chord × chord and ray ×
  ray crossings inside medallions (rosette cores are chord-bounded by
  construction, F2 of the engine docs);
* `outline` / `outline-interlace` thicken the fused strands with the
  existing miter/bevel machinery — medallion arms miter at contacts
  like any join;
* the degree-1 / open-end gate gains ONE message variant when the
  open end is a medallion ray: the record `ID`, its declared
  `Absorption` path, and the witness point, telling the user the
  weave at this δ does not absorb this record (the honesty principle
  of `DecorationError`, verbatim style).

## 5. Interface additions (minimal, enumerated)

1. `Tiling::extra_walls(&self, region) -> Vec<Wall>` — default empty
   (§4.1). Doc-only change for every existing tiling.
2. `Tiling::suppressed_seeds(&self) -> Vec<u32>` — default empty.
   (If a composition needs suppression to *not* imply the record's
   hull footprint, the seed IS the footprint: the M172 dodecagon
   faces are exactly the hulls. No new geometry type crosses.)
3. `Medallion::contacts_f64() -> Vec<([f64; 2], Option<Zq-tuple>)>` —
   the contact points (corner index or edge + t as f64); and
   `Medallion::figure_f64() -> Vec<([f64; 2], [f64; 2])>` — the
   figure chords. Pure additive views over data every instance
   already pins exactly in its battery; they mirror `hull_f64`'s
   normalized frame.
4. `strapwork::walls_layered(tiles, delta, extra)` — the fused
   constructor (§4.1); `walls()` delegates with `extra = []`, so the
   M57 surface is bit-identical.
5. One `DecorationError` message variant (open medallion ray, §4.4).

`tess-core` names no medallion concept beyond the doc comments:
`extra_walls` is just "more walls", which is the whole point of the
`Wall` currency (F1).

## 6. Exactness policy (two-tier, restated for this layer)

* Placement, contacts, figure chords, landings: **exact** in the
  record's field, by construction (the tiling derives layer geometry
  from the same exact tables that emit its faces; `first_hit` is
  division-free).
* The fused map is f64 (engine convention): every f64 claim is
  verified against an exact or analytic anchor to **1e-9** (the
  M167 precedent: "exactly where the landed data is exact … and to
  1e-9 where the registry carries f64"), never against a picture.
* Dedup: fused segments pass the 1e-6 grid; anchor A2 additionally
  checks the coincidence union has NO spurious split points (segment
  count equality, not just point-set containment).

## 7. Gate and battery plan (programmatic; nothing is "looks right")

Spike (stdlib Python ≤ 240 s, pinned to `.scratch/` before any Rust):
re-implement walls-with-suppression + chord/ray fusion over three
substrates; assert fused vertex degrees (Eulerian), two-strand
crossings, and segment-count identity for the coincidence regime.

Rust anchors at landing (per-record claims marked **[P]** = exact):

* **A1 WIS-interstitial closure.** Host `3.12.12`-patch tiling
  (host312's twelve declared neighbours), third layer = the WIS star
  record, generic firing suppressed inside the hull, δ irrelevant.
  Assert: marched landings reproduce the host312 first-hit multiset
  `{1+√3: 12, 2+√3: 12}` in f64 to 1e-9 **[P — same march data]**;
  the fused map closes (zero degree-1 ends inside the view) and all
  four styles gate.
* **A2 khatam coincidence.** Host 4.8.8-family substrate, δ = 45°,
  layer = the {8/2} khatam record: fused map = generic map ∪ figure
  chords exactly (sorted segment multiset equality on the dedup
  grid, 0 spurious vertices) **[P by M53/M57 ground truth]**.
* **A3 M172 composition.** `dodecagon-hexagram-honeycomb` at δ ∈
  {30, 60}: dodecagon seed suppressed + {12/d} dodecagon-class
  record layer, hexagram faces carry the hexagram_62 layer at θ = 60.
  δ = 60 must pass the closure gates (θ matching, law (L)); δ = 30
  must FAIL with the new named-record error (law (L) violated —
  the honesty case). Segment/face/strand counts pinned in both.
* **A4 regression:** every existing row of the classic-δ table
  (12 × 4) and the `decoration_finiteness_battery` +
  `strapwork_layer_mode` anchors stay green untouched (default-empty
  layers ⇒ identical code path); `walls()` delegation is
  byte-identical output.
* **A5 determinism:** `extra_walls` is pure (no RNG, ordered);
  fused output identical across repeated calls and under tile-emit
  permutation of the input (the existing dedup determinism test,
  extended with a layer).

## 8. Scope, non-goals, STOP list

* Scope: the absorption METHOD + the two trait default methods +
  `walls_layered` + one error variant + the anchors. First carrier
  instances: A1/A2/A3 (composition tilings supplying their own
  layers). The monster catalog does NOT become a tiling here (it
  stays data + tests; a monster-composition tiling would be its own
  brief).
* Non-goals: no new decoration style, no GUI surface (F8), no SVG
  group separation, no per-ray user placement language, no change to
  the interstice-fill verdicts of 24d, no registry movement beyond
  whatever the carrier instance needs (it may need none — A1/A2 can
  ship as tests over existing hosts with test-side layer suppliers).
* STOP list: a record whose declared absorption path cannot be
  marched to its landing on the host (report, never truncate); a
  fused map that needs a NEW gate to close (the design is wrong —
  absorption must be answerable by the existing gates); any need to
  reference medallion types from `tess-core` (F7 violated); any
  classic-δ table regression (STOP, investigate before landing).

## 9. Decisions taken (and rejected alternatives)

1. **Absorb at the wall stage, not the paint or SVG stage** — the
   decoration styles are defined on wall maps; a parallel SVG group
   could not take part in face two-colouring or strand parity, and
   "composition" would be a pile-up, not a weave.
2. **Tiling-supplied data, not a layer object model** — one default
   method pair, no new trait object; the tiling is the only extension
   point (AGENTS) and already owns the exact placement tables that a
   layer needs.
3. **Suppression is by seed** — the medallion faces ARE the hulls in
   every known instance; footprints-with-clipping were rejected
   (a clipper is exactly the machinery F2 makes unnecessary).
4. **The march is exact on the tilings side** — reusing `first_hit`
   keeps landing sites polynomial-identity exact and matches
   host312/M163 evidence; an f64 march would make A1 an
   approximations-all-the-way-down check.
5. **Law (L) as the continuity contract** — the M163 identity makes
   it free at matching δ; at non-matching δ the failure mode is an
   honest gate error, not a broken picture.

## 10. Backlog variations (explicitly deferred)

* **Nested medallions** — a medallion whose interstice or declared
  hole carries another record: the layer API already permits it (the
  march's polygon list can itself stop at a sub-hull; recursion depth
  2 needs `suppressed_seeds` to nest by seed class), but the closure
  semantics of chained absorption paths (a ray absorbed by a record
  that itself absorbs) need their own design pass. Deferred.
* **Field tower** — a third COORDINATE layer (K ⊂ L ⊂ M over the
  record fields) for hosts whose scale/rotation data leaves the
  two-level `Nq` shape: orthogonal to this doc (nothing here binds
  the third layer to two-level coordinates — the handoff is f64),
  tracked as a field-program item.
* **N-layer generalization** — if a THIRD kind of layer (e.g. a
  stripe-decoration source, ROADMAP research area 4) materializes,
  the pair of default methods generalizes to an ordered layer list
  with per-layer suppression sets; deliberately not built now.

## 11. Implementation shape (for the eventual code job)

Estimated: `tess-core` ≈ +150 lines (two trait defaults,
`walls_layered` ≈ `walls` refactor ~40, error variant, tests);
`medallion` trait + 5 instance view impls ≈ +120 (additive, each
view re-derives from the same exact tables and gets a battery line
tying `figure_f64` to the exact figure); A1–A3 suites ≈ +300; docs:
`rendering.md` gains a "Medallion layer (third layer)" section, the
carrier's family/per-tiling docs gain the usual rows. Tier: worker
(standard plumbing) with a strong review pass on A2's segment-
multiset identity. Effort: S–M. No UI changes (F8 + defaults).
