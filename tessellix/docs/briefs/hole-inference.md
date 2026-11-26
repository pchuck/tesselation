# Brief: hole-bearing substrates + boundary-matched inference (24b)

> **Precondition MET (2026-11-25):** 24a landed as M167 — the medallion
> interface, the five catalog records, `host312` (the verified bind →
> shoot 24 rays → first-hit certificate), and the cos²-only angle
> convention. Read `docs/medallion-interface-design.md` FIRST (§2
> hole-primary placement + invariance lemma; §3 the three absorption
> obligations), then the module `crates/tess-tilings/src/medallion/`.
> **Family:** none — engine + tests inside `medallion/`; **NO registry
> changes** (built-ins 163, memberships 184; tests baseline **1348** +
> yours). **Tier:** frontier. **Effort** XL — this brief scopes the
> first landing (sprint 1 + stretch); the XL remainder is staged by
> report. PROPOSED §24b.
>
> **Status: ✅ sprint 1 LANDED 2026-11-25 — M174, commit 6856ece**
> (integrated onto post-M173 main; WIS substrate + inference engine +
> exhaustive battery; M158 reproduced, M161 staged behind the
> degree-4 field-kernel gap; XL remainder staged by report. Delivery
> recovered byte-exactly from the session DB after the fleet-outage
> crash zero-filled the untracked job files — recovery verified
> against the harness's own edit patches).

## Goal

The core novel algorithm of the medallion program. Hypothesis (PROPOSED
§24b, verbatim intent): *given a host substrate with medallion-shaped
holes whose boundary edges carry the medallion's ray data (not free
Hankin contacts), inference in the interstice yields a consistent
complementary pattern — and the classic two-level patterns are
instances.*

**Ground-truth bar (user-pinned, in order):**
1. **WIS bar (sprint 1):** from the general path — a hole-bearing
   substrate whose holes are bounded by the M158 WIS 12-star record's
   hull + boundary rays — the inference engine reproduces the shipped
   `wis-12-fold` cell table (the landed module's prototile/cell data).
   The 24a record's absorption-(c) certificate (all 24 rays first-hit
   on dodecagon vertices, `host312`) is the template: generalize
   "absorb the rays" into "infer the interstice consistent with the
   rays".
2. **Darb-i Imam bar (stretch):** the same path reproduces the M161
   girih-decagon-panel cell table (611 tiles — the census the Bonner
   dossier flags as the comparison target for BOTH 24b and 24g). If
   sprint 1 lands cleanly and the session has budget, attempt it; a
   clean STOP with a precise gap report is an acceptable sprint-1
   landing.

## Landed surface you build on (verify, then use)

- `medallion/mod.rs`: Field/Contact/Absorption, the `Medallion` trait,
  `Zq` exact sign engine, `Qp` ops, rot30 kernel, strictly-CCW checks,
  exact `first_hit` with fraction-pair t.
- The five catalog records (khatam_m53, rosette_m56, girih_m161,
  hexagram_62, wis_m158) — each with hull, contacts, exit rays, cos²
  data, absorption path. **Consumption order per the 24a report:**
  host312 + the WIS record first; then the path-(a) records (M53/M56)
  wired to the M163 extension operator; then the M161 path-(b)
  BOW/HEX inference.
- The M163 motif/extension-operator machinery and the M94 strapwork
  engine (contact semantics; the cos² convention is shared).
- The Bonner dossier (`/home/pcharles/dev/pchuck/tesselation/.scratch/
  bonner-24g/dossier.md`) — context for what the *subdivision* path
  (24g) does differently; Cromwell's observation that Bonner uses a
  different large-scale net (decagon+pentagon+hexagon) for the same
  mosaic is exactly the 24b-vs-24g comparison; do not implement 24g
  here.

## The engine (shape guidance, not a spec)

New files under `crates/tess-tilings/src/medallion/` (keep `mod.rs`
edits to one-line `pub mod` declarations — a concurrent 24c job adds
sibling instance files; minimize collision surface):

1. **Hole-bearing substrates** (`holes.rs` or similar): given a host
   net (periodic or patch) and a medallion record, construct the host
   with medallion-shaped holes whose boundary edges carry the
   record's ray data. The 24a invariance lemma (§2) is the tool: bind
   by similarity invariants (t-parameters + cos² data), never compute
   the scale factor in the field.
2. **Boundary-matched inference** (`infer.rs` or similar): in the
   interstice, derive the complementary pattern consistent with the
   hole-boundary rays. Deterministic (no RNG). Where the inference is
   underdetermined (free θ / free contact position), DO NOT force a
   choice — enumerate/document the freedom as first-class output
   (feeds 24e's search); this is a PROPOSED-pinned requirement.
3. **Reproduction certificates**: for the WIS bar, a test-level
   comparison of the inferred pattern against the shipped cell table —
   the **cos²-multiset convention is the mutation-testable comparison
   key** (24a report's phrase): compare angle data as cos² multisets
   per vertex class, plus the exact cell census (counts per prototile
   class), plus hull/area identities. f64 only where the landed
   modules already use it (1e-9 similarity-invariant comparisons);
   everything else field-exact.

## Verification (all programmatic)

- Python spike (stdlib, exact arithmetic) BEFORE Rust: the WIS
  sprint — build the hole-bearing substrate, run the inference,
  reproduce the WIS cell table, print the comparison census. Pin the
  spike's numbers in-file.
- Rust tests mirror the spike: the WIS bar as the headline test;
  determinism (two runs byte-identical); the freedom-documentation
  test (an underdetermined case returns the freedom set, not a
  forced choice); field-membership/finiteness battery over every
  emitted exact number (the M164 NaN lesson).
- The 24a catalog records' landed tests stay green (you are adding,
  not modifying — if a landed pin must change, that's a STOP).

## Scope guards

- No registry/census movement; no PROPOSED/ROADMAP/CHANGELOG edits;
  no changes to the landed `Medallion` trait or the five record files
  (add sibling modules; if the trait must grow, STOP and report the
  design argument — 24a-design decisions are not unilateral).
- 24g subdivision machinery: out of scope (dossier context only).
- 24d compositions: out of scope (but the engine's API should make
  them natural — placement records + inference as separable steps).

## STOP conditions

- The WIS bar fails: the general path cannot reproduce the shipped
  cell table → STOP with the exact divergence (this is a real
  algorithmic finding — the hypothesis needs refinement, and the
  report must say what the interstice actually admits).
- Existence/uniqueness pathologies (the PROPOSED-flagged risk): a
  hole-boundary ray configuration with NO consistent completion, or
  with completion(s) that contradict the landed table → STOP and
  report the case; document, don't special-case.
- A required exact quantity leaves the shipped fields → STOP.
- You find yourself editing a landed record file or the trait → STOP.

## Bookkeeping

One commit, explicit `git add` paths only (never -A), subject like
`engine: medallion hole-bearing substrates + boundary-matched
inference — WIS ground-truth bar (24b)`, message via
`git commit -F <file>`; verify `git show HEAD --stat`. Scratch to
repo-root `.scratch/medallion-infer/` (persistent) before finishing.
Report: commit hash, the inference algorithm as implemented (the
general path, not a WIS special-case — say precisely what is general),
WIS bar result (census comparison), M161 stretch status, the freedom
sets found (24e feed), test counts, deviations, what 24d/24e consume
first.