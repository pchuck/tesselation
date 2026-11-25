# Brief: decoration-suite — Kaplan §3.7 rendering styles over the strapwork engine

> **Family:** none — tess-core rendering infrastructure + tess-tui CLI surface.
> **NO registry changes, NO family files, NO census movement** (built-ins stay
> 162, memberships 183, tests 1273 + yours). The registered PIC modules are
> **ground truth** (PROPOSED #14) — audit against them, never edit them.
> **Tier:** strong (Effort M, PROPOSED P1 "after the audit"). **Candidate:**
> PROPOSED.md §22g / ROADMAP 22g.

## Current state (orchestrator audit — verify, then build on it)

- `tess_core::strapwork::walls(tiles, delta_deg) -> Vec<Wall>` (M57): the wall
  network over an arbitrary host tiling — at every tile-edge midpoint two rays
  enter each adjacent tile at contact angle δ, symmetric about the inward
  normal; the adjacent tile fires the point-reflected pair (continuity across
  edges); inside a tile each ray runs to its boundary exit and is **split at
  every crossing**; output ordered + deduped via BTreeMap (deterministic).
  Guards: degenerate tiles emit nothing; near-parallel rays give no crossing.
- `tess_core::svg::render_svg_with_walls(tiles, bbox, mode, &walls)`: renders
  walls as zero-width `<line>` strokes (viewport-derived stroke-width, round
  caps) — **no band geometry, no crossings treatment, no z-order**.
- Surface: walls render ONLY in tess-tui's SVG export CLI (`main.rs:195`,
  `--delta` flag). The interactive TUI/GUI do not touch strapwork — keep it
  that way (blast radius).
- Cross-check anchors (tess-tilings, `strapwork_layer_mode` tests): the engine
  on registry substrates at classic δ reproduces the registered PIC tilings'
  internal edges exactly (endpoint-canonical 1e-6 keys).

## The §3.7 styles (diss. pp. 90–93; full text extracted at
`.scratch/salvage-20261120/tmp/wallpaper_groups/kaplan_full.txt` lines 2679–2760
— read it first)

1. **Fill** (§3.7.1): 2-colour the faces of the star-pattern line map. Kaplan's
   precondition: designs have only 2- and 4-valent vertices ⇒ the face
   adjacency graph is bipartite (a planar map all of whose vertices have even
   degree has a face 2-colouring — standard dual-bipartite result). Tradition:
   one colour class white, the other receives colour(s).
2. **Outline** (§3.7.2): thicken each wall to band width w via equidistant
   offsets; **mitered join** where two segments meet at angle θ: the mitre
   point sits on the angle bisector at distance l from the corner with
   Θ(l) = Θ(w)/sin(θ/2) in Kaplan's absolute-trig notation — in our EUCLIDEAN
   engine derive it from first principles and **document the half-width
   convention** (the dissertation's Θ-form, Θ(x)=2πx here, reconciles to
   l = d/sin(θ/2) with d = offset distance; transcribe nothing you have not
   derived). θ→0 spikes: pick a clamp policy (cap l at a small multiple of w)
   and test it.
3. **Interlace** (§3.7.3): over-under at every crossing, globally consistent.
   Kaplan decides it on the double fundamental map of [p,q]⁺ and replicates
   with direct isometries only. We have no fundamental maps — our substitute:
   **strand-parity alternation** (follow each strand through its crossings,
   alternate over/under) + a **consistency obligation**: the assignment must
   be invariant under the host lattice's translations (same crossing in
   periodic images ⇒ same parity). Prove it by folding: replicate the
   assignment to a ±2-cell torus window and compare before/after fold.
   If some substrate fails consistency, STOP and report — that is a finding
   about the strand structure, not something to special-case away.
4. **Composition** (§3.7.4): outlined or interlaced rendering over a filled
   one; and outline-interlace (treat the interlaced rendering as a new map
   and outline it). SVG z-order makes the over/under read: the over-strand's
   band is drawn after the under-strand's band with a background-coloured
   break gap.

## Phase A — audit (deliverable: numbers, not prose)

Build the planar map from `walls()` output on the registered PIC/hankin
substrates (pic-4-8-8, pic-3-6-3-6, pic-3-12-12, khatam pair, hankin-*
inference entries, and the M163 motif instances) over a δ grid (classic δ ±
off-classic values). Census, per (substrate, δ): vertex degree histogram
(midpoint joints vs crossing X-joints vs **degree-1 boundary exits**), crossing
count, strand count, face count of the wall-only map, and whether the
even-degree precondition holds. Cross-check the classic-δ face structure
against the registered modules' own cells (engine ground truth). **Phase B
may only target styles whose preconditions the audit establishes.** A fill
precondition failing at generic δ is an EXPECTED outcome (degree-1 exits) —
report the δ-regimes where each style is well-defined instead of forcing it.

## Phase B — implement (spike first, per the adoption checklist)

Python stdlib spike in `/tmp/opencode` (or `.scratch/decoration-suite/`):
exact mitre geometry, the 2-colouring, the parity walk, the torus fold —
then port. Rust changes confined to:

- `crates/tess-core/src/strapwork.rs` — map construction (vertices/strand
  graph from the wall segments), face extraction, 2-colouring, band/mitre
  geometry, parity assignment.
- `crates/tess-core/src/svg.rs` — decoration render modes (fill underlay +
  band overlay + break gaps for interlace; keep the existing plain mode
  byte-identical when no decoration is requested).
- `crates/tess-tui/src/main.rs` — CLI flags (`--decoration
  fill|outline|interlace|outline-interlace`, `--band-w W`) with honest
  errors when a style's precondition fails on the requested (tiling, δ).
- `crates/tess-core/src/view.rs` only if ColorMode needs a variant.

## Verification battery (all programmatic; "looks right" is not a deliverable)

- Bipartite fill: adjacency-checked 2-colouring; same face class across
  lattice translations gets the same colour (multi-tile patch test).
- Outline: band polygons derived exactly; mitre points pinned against the
  Python spike (1e-9); clamp policy exercised at a near-degenerate θ.
- Interlace: exactly one strand over per crossing; alternation along each
  strand; **periodic consistency via the torus fold** (the hard obligation);
  crossing census matches the walls() split points.
- SVG: parse the output — path/line counts, z-order of over/under bands,
  plain-mode output byte-identical when no decoration requested.
- Existing anchors stay green: `strapwork_layer_mode` cross-checks untouched.

## STOP rules

- Interlace parity inconsistent on any classic-δ substrate after a genuine
  attempt at the parity construction — STOP with the folded evidence.
- Mitre geometry or band offsets cannot be pinned to the spike at 1e-9 — STOP.
- You find the audit contradicting this brief's current-state section — STOP
  and report (the audit above is recon, not gospel).

## Bookkeeping

One commit, explicit `git add` paths only (never `-A`), subject like
`core: Kaplan §3.7 decoration styles — fill/outline/interlace over the
strapwork engine (22g)`, message via `git commit -F <file>`; verify
`git show HEAD --stat`. Scratch in `.scratch/decoration-suite/` (worktree) —
copy hard-won derivation notes to the persistent repo-root `.scratch/` too.
Report: commit hash, audit numbers, battery highlights, δ-regime table
(which styles are well-defined where), deviations, follow-ups (Castéra band
widths remain open — PROPOSED says so; do not attempt them).