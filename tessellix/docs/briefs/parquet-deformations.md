# Brief: parquet-deformations — Islamic parquet deformations (22d, diss. §3.4.1)

> **⚠ Dispatch precondition: 22g (decoration-suite) must be LANDED first** —
> 22d edits the same three files 22g just restructured
> (`strapwork.rs`, `svg.rs`, `tess-tui/src/main.rs`). Base on post-22g main.
>
> **Family:** none — walls-layer rendering feature (PROPOSED's own scoping:
> "naturally an export feature, not a registry entry"). **NO registry
> changes** — built-ins stay 162, memberships 183; tests baseline + yours.
> **Tier:** worker. **Effort:** M.

## The construction (diss. §3.4.1, pp. 57–58; extract at
`.scratch/salvage-20261120/tmp/wallpaper_groups/kaplan_full.txt` lines
1918–1942 — read it first)

Parquet deformations (William Huff → Hofstadter) are "spatial animations":
geometric drawings that transition smoothly **in space**. Kaplan's Islamic
version: lay out a strip of the template tiling and run the inference with
**the contact angle at every contact point determined by the location of
that point in the strip** (diss. Fig. 3.7: angle varying with the x position
of the ray's starting point; two more examples in Fig. 3.8). The result is a
gently changing design, still recognizably Islamic, with essentially no
repetition.

**Why continuity survives the deformation (the load-bearing fact):** the
angle is a function of the *contact point*, and both tiles adjacent to an
edge share that point. Our engine already fires point-reflected ray pairs at
every edge midpoint (the M57 continuity contract, empirically pinned by the
`strapwork_layer_mode` cross-checks); substituting δ → f(m) with m the
shared midpoint preserves the reflection identity *structurally*: tile A's
pair {±cos f(m)·t + sin f(m)·nin} reflects about m to exactly tile B's pair
(nin flips sign with the winding side). Nothing about the crossing-split,
trimming, or dedup logic changes.

**Terminology guard:** our `walls(tiles, delta_deg)` δ IS the contact angle
(the dissertation's θ in the inference chapter; Kaplan reserves δ for the
*contact-split distance* of his two-point extension — a different parameter,
already landed separately in the two-point hankin instances). Deforming the
split distance is OUT of scope (note as follow-up).

## Current state (orchestrator audit 2026-11-24 — verify, then build)

- `tess_core::strapwork::walls(tiles, delta_deg)` (strapwork.rs:118):
  computes `(sd, cd) = delta_deg.to_radians().sin_cos()` ONCE, then per
  tile, per edge midpoint `m` builds the two rays as
  `d = lean·cd·t + sd·nin` (lean = ±1). **The per-point change is exactly:
  hoist the sin_cos into the edge loop with δ = f(m).** Guards (degenerate
  tile, near-parallel ray, exit trimming) are per-ray already.
- `walls()` must remain byte-identical for f ≡ const (keep it as a
  delegating wrapper; the regression anchor).
- CLI (post-22g): `parse_export` pure/testable; flags `--walls/--delta`,
  `--decoration/--band-w`; render window derives from aspect
  (`world_h = 20/scale`, `world_w` from width/height) — a wide aspect IS a
  strip. Follow 22g's conventions (honest errors, pure parsing).
- svg.rs: walls render unchanged — a deformed wall set is just a wall set;
  22g's decorated modes compose for free if the plumbing allows (do not
  force it; plain render is the deliverable).

## Phase A — audit (deliverable: the δ-validity envelope)

For each registered engine substrate (the `strapwork_layer_mode` set), the
per-tile geometry restricts *which* contact angles produce clean closed
strands: at a corner of interior angle φ, rays from the two adjacent edges
meet inside iff the angle is small enough relative to φ; out-of-range
angles produce degree-1 boundary-exit debris. Deliverable: per substrate,
the **valid angle envelope** (which [δ_min, δ_max] keeps every tile's
strand structure closed), derived programmatically (per-tile minimum
interior angle vs observed degree-1 counts over a δ sweep). This envelope
is the clamp domain for Phase B's deformation profiles AND the substrate
scoring input for 24h. Report it as a table; do not force profiles outside
it (clamping to the envelope is the correct behaviour, not a limitation).

## Phase B — implement

- `strapwork.rs`: `walls_with_delta(tiles, f: impl Fn(Vec2) -> f64) ->
  Vec<Wall>` (f = contact angle in degrees at point p); `walls()` delegates
  with `|_| delta_deg`.
- Deterministic built-in profiles (analytic, no RNG; §15 determinism
  policy): `linear` (δ = a + (b−a)·(x−x₀)/(x₁−x₀)), `radial` (δ by distance
  from a centre), `sine` (δ = mid + amp·sin(k·x)). Clamp to the Phase-A
  envelope. Parameters via CLI flags following 22g's parse pattern
  (`--deform linear|radial|sine` + numeric params; exact flag spelling is
  yours, keep it honest and testable).
- Gallery export: one SVG (or a small set) showing the strip morph — the
  Fig. 3.7 reproduction on the square substrate (tight stars → open
  crosses along x) is the canonical smoke test.

## Verification battery (all programmatic)

- **Byte-identity anchor:** f ≡ const reproduces `walls()` output exactly
  (canonical wall keys) on all engine substrates at several δ.
- **Continuity:** for a deformed wall set, at every shared edge midpoint
  the two incident ray pairs are point reflections about the midpoint
  (endpoint-canonical check, tolerance-free on the 1e-6 grid).
- **Piecewise-constant equivalence:** a per-column constant profile must
  reproduce, within each column, exactly the constant-δ walls for that
  column's δ (the exact bridge between deformed and classic patterns).
- Crossing-split invariants preserved (every ray split at every crossing;
  census matches the walls() machinery on the piecewise-constant case).
- Determinism: two runs, identical bytes.
- CLI: honest errors (unknown profile, params outside envelope → named
  error, not a silent clamp… decide and document one policy).

## STOP rules

- Continuity identity violated on any substrate/profile → STOP (engine
  finding — the reflection argument above says it cannot; evidence beats
  argument).
- The Phase-A envelope cannot be characterized programmatically → STOP
  with the sweep data (24h needs this table; do not hand-wave it).
- 22g's landed surface differs from the current-state section → STOP and
  report the drift before coding.

## Bookkeeping

One commit, explicit `git add` paths only (never `-A`), subject like
`core: Islamic parquet deformations — per-contact-point delta over the
strapwork engine (22d)`, message via `git commit -F <file>`; verify
`git show HEAD --stat`. Scratch in `.scratch/parquet-deformations/`
(worktree) with a persistent copy of the envelope table + derivation notes
to repo-root `.scratch/parquet-deformations/` before finishing. Report:
commit hash, envelope table, battery highlights, files, test counts
before/after, deviations, follow-ups (split-distance deformation, 24h
integration).