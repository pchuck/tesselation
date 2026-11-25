# Brief: medallion-interface — schema design doc + exact catalog re-derivations (§24a)

> **Program opener for §24** (PROPOSED §24 / ROADMAP "Medallion composition
> program"). User-pinned: ground-truth-first, registry instances (later
> sprints), periodic nets first. **Deliverable 1 is a design doc, not code.**

**Family:** none for the schema (new cross-cutting module; catalog
re-derivations are certification-style, no registry changes — built-ins stay
162, memberships 183). **Effort:** L. **Tier:** strong (design doc) with
frontier escalation on the schema derivation.

## Goal

Pin the **medallion interface**: the data contract that lets a self-contained
high-degree ornament (n-pointed star, hexagram {6/2}, dodecagram {12/4},
rosette, khatam ring) be placed on a host lattice and have its rays absorbed
exactly by the surrounding pattern. Design doc first
(`docs/medallion-interface-design.md`), sufficient for a reviewer to
implement without re-deriving anything — the house pattern that worked for
the cyclotomic layer (M137) and the isohedral generator (M162).

## What the doc must pin

1. **The interface record**: planar map on a declared convex hull; boundary
   contact points (position on each hull edge); ray exit angles per contact;
   the internal strap figure; declared symmetry; the exact field each
   instance lives in. Everything exact — coordinates in the instance's
   number field, not f64.
2. **Placement contract**: how a medallion binds to a host hole — which
   hull edge maps to which substrate hole edge, orientation conventions,
   scale policy (medallion sized by the hole, or hole sized by the
   medallion — pick one and justify).
3. **Ray-landing obligations**: exit rays must be absorbable by (a) the M163
   extension operator's landing identity (ray → parent edge midpoint),
   (b) M94-style inference on boundary cells, or (c) direct interstitial
   continuation. Enumerate which obligations each catalog instance
   exercises.
4. **Catalog re-derivations** (certification-style, exact): khatam 4.8.8
   star (M53), {6/2} hexagram (the star-and-cross entry), {12/4} dodecagon
   rosette (pic-3-12-12's core), the girih decagon patch panels (M161),
   WIS 12-point star (M158). Each re-derived in its exact field, with its
   boundary ray data tabulated — these tables are the ground truth 24b
   will reproduce.
5. **Geometry-agnosticism**: which parts of the schema survive in absolute
   geometry (22f) — flag Euclidean-only assumptions explicitly (the mitre
   formula is absolute via Θ; contact angles are absolute; coordinates
   are not).

## Spike first (adoption checklist)

Python stdlib in `.scratch/medallion-interface/`: represent 2–3 medallions
in the candidate schema, place one on a host hole, and demonstrate the ray
data suffices for exact binding — before the design doc is declared final.
The doc cites the spike's numbers.

## Scope guards

- No registry changes, no family edits, no render changes (22g owns
  `svg.rs`/CLI). The schema module lands in `tess-tilings` (or tess-core —
  the doc recommends which, with reasons) as data + tests only.
- Do NOT attempt 24b (the inference algorithm) — this brief only builds the
  interface and the ground-truth catalog.
- Re-derivations must agree with the landed modules' geometry (spot-check
  against the registered PIC/khatam internals, 1e-9) — a disagreement is a
  STOP-and-report finding, not something to fudge.

## STOP rules

- Two catalog medallions whose ray data cannot be expressed in the candidate
  schema without ad-hoc fields → STOP, propose the schema revision.
- A re-derivation disagreeing with the landed ground truth → STOP with both
  computations shown.
- Exactness requiring a field we do not ship (e.g. degree-5) → STOP and
  scope the instance out (do not approximate).

## Bookkeeping

Design doc + spike + (if the doc declares the module shape) the catalog
module with its batteries in ONE landing; explicit `git add` paths only;
subject like `engine: medallion interface design doc + exact catalog
re-derivations (24a)`. Report: doc path, spike numbers, per-medallion field
+ ray-exit tables, deviations, what 24b should consume first.
---
**Status: ✅ LANDED as M167 (2026-11-25)** — design doc + medallion module +
five exact catalog re-derivations (M53 khatam, M56 rosette core, M161 girih
decagon, {6/2} hexagram, M158 WIS 12-star), 37 tests, no registry change.
24b consumes host312 + the WIS record first.
