# Brief: two-level subdivision systems — polygonal-net path (24g)

> **Precondition MET (2026-11-25):** the Bonner research dossier is banked
> at `.scratch/bonner-24g/dossier.md` (746 lines, 9 free full-text sources
> alongside it) — read it FIRST, especially §1 (subdivision mechanics
> A/B/C), §2 (scale ratios, exact), §5 (taxonomy), §9 (feasibility verdict
> + this brief's skeleton), §11 (the 5 [SPEC] items). Then the landed
> surface: `crates/tess-tilings/src/families/islamic/girih_qc.rs`
> (certified substitution tables + markers) and
> `girih_decagon_patch.rs` (the M161 611-tile Darb-i Imam panel — the
> comparison target). **Family:** islamic — new module
> `families/islamic/subdivision_systems.rs` (+ tests). **Registry:**
> entries EXPECTED (census moves; orchestrator assigns M-numbers and owns
> PROPOSED/ROADMAP/CHANGELOG — you edit none of those; report the census
> deltas you expect). Tests baseline: **1356** + yours (re-derive at
> landing; the number moves with concurrent landings). **Tier:** strong.
> **Effort** L per system; this brief scopes ONE system's landing with
> the second as stretch. PROPOSED §24g.

## Goal

The polygonal-net **subdivision** path to two-level patterns — the
alternative to 24b's inference path (in flight; do not read or touch
`medallion/`). Build ONE system exactly, then compare both paths on the
Darb-i Imam ground truth (M161): same mosaic, two derivations — the L-S
bow-tie+decagon substitution net (landed) vs Bonner's Type A
decagon-pentagon-hexagon large-scale net (Cromwell's observation, dossier
§4). That comparison IS the deliverable's intellectual payload; the
comparison note goes in your report and the module docs, not PROPOSED.

## Ground-truth bar (repo-verifiable — these are the pins)

1. **Darb-i Imam Type A (headline):** from a level-1 net + per-class
   subdivision tables, reproduce the landed `girih_decagon_patch` panel:
   exact equality of the tile census (**611 = 270 bobbin + 220 bow-tie +
   121 decagon**), the edge structure (**431 children; 60 unique
   large-level edges; n_children + 3·uedges = 611**), seed decagon +
   10 edge-adjacent bow-ties each sharing exactly one edge,
   edge-to-edge + T-junction-free. Do NOT trust the dossier's secondary
   panel statistics (4150/1810/530) — derive everything from the landed
   tables and your construction; the landed tests are the oracle.
2. **Inflation factor falls out structurally:** 2φ³ = 4+2√5 must be
   DERIVED from the composite-edge rule (2 bobbin long-diagonals + 1
   decagon long-diagonal, Cromwell 2009's Isfahan/Topkapı rule, dossier
   §2) — never hard-coded. Cross-check: the landed `girih_qc` marker
   offset (3φ+1)/2 composes to the same value; show the identity in a
   test **[P]**.
3. **Second system (stretch, only if 1 lands cleanly):** Madar-i-Shah
   nested 5-fold per Webster 2020 (dossier §2, [S4] — complete free
   spec): edges Φ:Φ+1:Φ+2, decagon/bow-tie edge arrangements;
   verify **strict scale-invariance programmatically** (level-1 shape
   set == level-2 shape set, edge-length ratios exact in Q(√5)-within
   the shipped decagon field). Register separately if it lands.

## Architecture (from the dossier's verdict)

- Per-class subdivision tables as exact-arithmetic data in the style of
  `girih_qc` (certified tables; every row's provenance cited in a
  comment: landed table, Cromwell 2009 Fig-15 sentence, Webster 2020,
  Bonner 2003 — PDFs+txts sit in `.scratch/bonner-24g/`).
- Family-angle parameterization (36° acute / 72° middle / 108° obtuse /
  two-point) as data-driven contact-angle classes, matching the landed
  islamic family's motif conventions (M163) where they apply.
- Python spike FIRST (stdlib exact arithmetic): the Type A subdivision
  over one level-1 net cell, census printed and pinned in-file, BEFORE
  any Rust. The spike's numbers must equal the landed pins above.

## Scope guards

- **No fabricated tables** (dossier §9 STOP): every table row cites a
  source or is derived by exact construction from a cited figure.
- **No quasi-periodicity claims** — these systems are periodic/
  hierarchical (dossier §5.4); the historical record is exactly
  two-level (Cromwell's proof pentagon/bow-tie admit no subdivision).
- Two-level ONLY; iterating beyond level 2 is out of scope (note as
  extension). n=12 systems out of scope (G6); "polyzone"/"hemed" are
  unverified placeholder terms — never build-bearing (G5).
- Do not implement 24b's inference path or touch `medallion/` (two
  concurrent jobs live there).
- New fields: none — Q(2cos 18°)/Q(√5) machinery is shipped (girih_qc,
  girih_decagon_patch, M161's L10). A needed quantity outside shipped
  fields → STOP.

## STOP conditions

- A per-class subdivision rule cannot be pinned from cited sources →
  STOP, report G1–G3 (book-gated content; the eBook is USD 17.99 —
  propose purchase, never improvise).
- Your Type A construction reproduces the tile census but violates an
  edge-structure pin (or vice versa) → STOP with the exact divergence
  (a real finding about the two nets — Cromwell's large-net-vs-L-S-net
  distinction may be exactly here).
- Census deltas you cannot attribute to your own new registry entries
  → STOP and report.

## Bookkeeping

One commit, explicit `git add` paths only (never -A), subject like
`engine: two-level subdivision systems — Darb-i Imam Type A via the
polygonal-net path (24g)`, message via `git commit -F <file>`; verify
`git show HEAD --stat`. Scratch to repo-root `.scratch/subdivision-24g/`
(persistent) before finishing; copy any PDF/text you actually used into
it. Report: commit hash, expected census deltas (built-ins 163→?, per-tab
vectors), the Type A comparison table (landed-vs-derived, every pin),
the 2φ³ derivation, Madar-i-Shah status, test counts (re-derived),
deviations, what 24b's comparison note consumes.