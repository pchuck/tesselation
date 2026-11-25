# Brief: (n,c)-monster medallion generator (24c)

> **Precondition MET (2026-11-25):** 24a landed as M167 — the medallion
> interface, `Zq`/`Qp` exact fields, the cos²-only angle convention, the
> three absorption obligations, and five catalog records. Read
> `docs/medallion-interface-design.md` FIRST, then the module
> `crates/tess-tilings/src/medallion/`. **Family:** none — catalog
> instances inside `medallion/`; **NO registry changes** (built-ins 163,
> memberships 184; tests baseline **1348** + yours). **Tier:** strong.
> **Effort** M (per catalog entry; this brief scopes one session's
> landing). PROPOSED §24c.

## Goal

The (n,c)-monsters — Kaplan's dissertation calls them "a taxonomy of
monsters and the motifs that can be associated with them" — become
exact catalog entries in the 24a interface: for regular n-gon pairs
(n ≤ 12, 2 ≤ c ≤ ⌈n/2⌉−2) fused at contact angles d·π/n, the exact
placement, the exact union outline, the crossing vertices, and the
ray-exit data per constituent n-gon — everything the 24d composition
families need to place monster motifs on periodic nets.

**Source (in-repo extract, verbatim-verified):**
`.scratch/salvage-20261120/tmp/wallpaper_groups/kaplan_full.txt`
lines ~3010–3070 (dissertation pp. 123–106). Definition
(p. 105, verbatim): *"For every n>6 and 2 ≤ c ≤ ⌈n/2⌉−2, define the
(n,c)-monster as the union of a pair of regular n-gons arranged so that
exactly c full edges of each polygon lie inside the other."* Contact
angles: multiples of π/n (the menagerie's columns, p. 106). Also in
scope per PROPOSED §24c: the **strange-star set {8,12}** (dissertation
§3.11.4, extract lines ~3219–3224: sets like {8,12} "follow immediately
from the tiling notation of Section 3.6" — regular polygons placed at
rhomb-tiling vertices, inflated to edge midpoints, holes filled).

## Architecture decision (pinned — read before coding)

The 24a schema requires a **strictly convex** hull
(`strictly_ccw`, `medallion/mod.rs`): every consecutive triple turns
left. A monster **union** is non-convex (reflex vertices at the
boundary crossings). Therefore a monster is catalogued NOT as one fused
record but as:

1. **Two convex n-gon medallion records** (the standard {n/d}-star
   motif per n-gon at the chosen contact angle), in their **exact
   relative placement** — a placement record fixing the rotation
   (crossing angle = d·π/n as a cos² identity) and the center
   displacement (exactly the value that puts exactly c full edges of
   each inside the other), all in the instance's exact field;
2. **The overlap table**: the exact crossing vertices (in the field),
   which edges cross, the union outline traversal (for 24d), and the
   overlap region's boundary segments;
3. **Ray-exit data per n-gon** consistent with the landed M163 motif
   convention (cross-check against the strapwork engine's rays for
   `Motif::Star` at the same contact angle — a mismatch is a STOP-grade
   finding).

The dissertation's own framing supports this: *"the inference algorithm
performs poorly on monsters"* — the fused form is composition work
(24d), and the tailored overlap motifs (Bonner's (10,2) set, Castéra's
2π/5 alternate, the Fig 3.40 menagerie) are **visual sources, not
numeric ground truth**. Do NOT attempt to re-derive a tailored overlap
motif from the pictures. What you derive is the geometry + the standard
star-motif exits; the overlap-motif gap is documented as the 24b/24e
target.

## Scope

- **n ∈ {8, 10, 12}** (shipped fields: Q(√2), Q(√3); n=10 check the
  field inventory — the M161 record used `L10`/2cos 18° degree-4, and
  plain decagon vertices may live in Q(√5)-within-L10; use what's
  shipped, never a new field), **all legal c** for each n
  (8: c ∈ {2}; check ⌈n/2⌉−2 = 2 for n=8 — verify the arithmetic:
  ⌈8/2⌉−2 = 2, so c=2 only; 10: c ∈ {2,3}; 12: c ∈ {2,3,4}), contact
  angles d·π/n for the d values the motif convention supports.
- **{8,12} strange-star set** (stretch, only if the catalog entries
  land cleanly with session budget left): the rhomb-vertex placement
  technique of dissertation §3.6/§3.11 (regular polygons at rhomb
  vertices of a rhombus tiling with 45° and 60° rhombs, oriented
  edge-midpoint-on-edge, inflated to meet at edge centers) producing
  the periodic 8+12-pointed star arrangement; verified as a placement
  table over one lattice cell, NOT a new registry tiling.
- **n = 9, 11 are OUT of scope** (cubic/degree-5 fields — 2cos(2π/9)
  is degree 3; the shipped field inventory has no cubic). Flag them as
  blocked-on-field in the report and PROPOSED-facing notes; do NOT
  force them. {9,12}/{9,18} sets therefore defer to a field-layer
  follow-up.

## Verification (all programmatic)

- Python spike (stdlib, exact `fractions`/integers — no floats in the
  derivations; floats only for sanity printing): for each (n,c,d)
  record — the placement condition verified combinatorially (count
  edges of A strictly inside B = c, edges of B inside A = c, via exact
  orientation tests at 1e-… no: exact sign tests in the field); the
  crossing angle identity cos²(crossing) = cos²(d·π/n) **[P]**; the
  union outline closes and its vertices are exactly the n-gon vertices
  + crossing points **[P]**; every record passes the 24a module's own
  checks (`strictly_ccw` per n-gon, normalization, contact t-parameters
  in (0,1), exit-ray cos² censuses) **[P]**.
- Rust: the records as medallion instance files + a placement-record
  struct; tests mirror the spike pins (field-exact, no f64 except at
  the 1e-… similarity-invariant comparisons the landed module already
  uses). Cross-check at least one (n,c,d) against the strapwork
  engine: place the two star motifs via `Motif::Star` on the two
  n-gons and confirm the ray directions agree with the record's exits
  (cos² table) **[P]**.
- New battery test echoing the M164 lesson: every emitted exact number
  in every record is finite and in the declared field (no ad-hoc
  floats).

## Scope guards

- No registry/census movement; no PROPOSED/ROADMAP/CHANGELOG edits
  (orchestrator owns); no schema changes to the 24a module's trait
  (if you believe the schema must extend to non-convex hulls, STOP and
  report the design argument instead).
- No visual-source chasing (Bonner/Castéra menagerie pictures).
- {9,12}/{9,18} and any n > 12: out of scope.

## STOP conditions

- A legal (n,c,d) whose placement cannot be pinned exactly in a
  shipped field → STOP with the failing case (it may be a real
  field-layer gap).
- The M163 cross-check disagrees with your exit rays → STOP (engine
  finding).
- You find yourself needing a cubic or degree-5 field → STOP (scope
  says n ∈ {8,10,12}).

## Bookkeeping

One commit, explicit `git add` paths only (never -A), subject like
`engine: (n,c)-monster medallion catalog entries — exact placements and
ray-exit records (24c)`, message via `git commit -F <file>`; verify
`git show HEAD --stat`. Scratch to repo-root `.scratch/monster-24c/`
(persistent) before finishing. Report: commit hash, per-record table
(n, c, d, field, placement constants, crossing count), M163 cross-check
result, {8,12} status (landed or deferred + why), test counts,
deviations, what 24d consumes first.