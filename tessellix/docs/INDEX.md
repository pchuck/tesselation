# Documentation index

tessellix's documentation is organized for parallel contribution: one
file per family (the collision-free unit of doc work), one document per
tiling, shared cross-cutting reference docs, and frozen history.

## Tiers

1. **[README.md](../README.md)** — the overview: what tessellix is,
   build/run quickstart, the eleven families with doc links, and pointers
   into the tiers below.
2. **Per-family docs** (`docs/families/<family>.md`) — each family's
   catalog table + numbered build arcs + links to its members.
3. **Per-tiling documents** (`docs/tilings/<id>.md`, 166 files) — each
   tiling's full specification: algorithm, exact geometry, verification,
   sources.
4. **Cross-cutting reference** — DESIGN (architecture), USAGE (TUI/CLI/
   GUI walkthrough), 2D_Tilings_Reference (math background).

## Per-family docs

| Family | Doc | Tilings | Arcs |
|---|---|---|---|
| uniform | [docs/families/uniform.md](families/uniform.md) | 11 | 0 |
| quasiperiodic | [docs/families/quasiperiodic.md](families/quasiperiodic.md) | 18 | 15 |
| arrangements | [docs/families/arrangements.md](families/arrangements.md) | 3 | 2 |
| pentagons | [docs/families/pentagons.md](families/pentagons.md) | 14 | 13 |
| laves | [docs/families/laves.md](families/laves.md) | 8 | 1 |
| kuniform | [docs/families/kuniform.md](families/kuniform.md) | 37 | 24 |
| reptiles | [docs/families/reptiles.md](families/reptiles.md) | 10 | 10 |
| aperiodic | [docs/families/aperiodic.md](families/aperiodic.md) | 11 | 10 |
| pavers | [docs/families/pavers.md](families/pavers.md) | 14 | 14 |
| islamic | [docs/families/islamic.md](families/islamic.md) | 35 | 28 |
| isohedral | [docs/families/isohedral.md](families/isohedral.md) | 5 | 1 |

A family doc owns its catalog table (guarded:
`family_docs_match_registrations` fails the suite on drift from the
registry), its numbered build arcs (§N — code comments cite these), and
its Per-tiling-documents link section (guarded:
`tiling_docs_match_registry`). Per-tiling documents are guarded too:
every registered tiling must own `docs/tilings/<id>.md` titled with its
registry name.

## Cross-cutting

- [docs/DESIGN.md](DESIGN.md) — the architecture reference: crate
  layout, the `Tiling` trait contract, self-describing parameters, the
  `LatticeTiling` framework, generator design, UI state, rendering, and
  the test-suite description
- [docs/USAGE.md](USAGE.md) — the TUI/CLI and GUI walkthrough: Browse
  mode, search, Render mode, keybindings, mouse behavior, all `export`
  flags
- [docs/rendering.md](rendering.md) — the strapwork/decoration render
  surface (M164): walls layer, the four Kaplan §3.7 styles, precondition
  errors, classic-δ table, decoration finiteness battery (the M164 NaN
  gap — fixed)
- [docs/ROADMAP.md](ROADMAP.md) — vetted-candidates queue + research areas
  (dossiers in [PROPOSED.md](../PROPOSED.md))
- [docs/cyclotomic-field-layer.md](cyclotomic-field-layer.md) — exact
  degree-3/4 cyclotomic arithmetic design doc (§23e/§23i gate: oblique
  U(k) frame, certified minpolies, Sturm sign engine, applicability
  table; stdlib spike + log in `.scratch/cyclotomic-design/`)
- [docs/cyclotomic-field-degree5.md](cyclotomic-field-degree5.md) —
  degree-5 field design doc (M178): K₁₁ = Q(2cos(2π/11)) + N₁₁ sine
  extension, spike-validated, no code yet (unblocks 24c n=11)
- [docs/cyclotomic-field-odd-n.md](cyclotomic-field-odd-n.md) —
  odd-n cyclotomic field layer extension design doc (M179): K₁₃ =
  Q(2cos(2π/13)) degree-6 class + the general prime-n shape, the D=6
  i128-envelope resolution (s₀-drop to 20), spike-validated, no code yet
- [docs/medallion-24c-odd-n-design.md](medallion-24c-odd-n-design.md) —
  24c odd-n engine-side design doc (M181): seven (n,c) monster rungs at
  n = 11, 13 over the designed field layers, spike-validated, no code
  yet (unblocks the 24c n=11/13 rung implementation)
- [docs/cyclotomic-field-17-21-design.md](cyclotomic-field-17-21-design.md) —
  §23i new-field-tiers design doc (M183): K₁₇ degree-8 constants
  certified but D = 8 NON-QUALIFICATION at the engine wall; K₂₁
  composite degree-6 transfers verbatim (s₀ = 21, p = 43 square-class
  extension certificate); corrected per-family applicability table,
  spike-validated, no code yet
- [docs/TAXONOMY.md](TAXONOMY.md) — the eleven-tab gallery taxonomy (M76; Isohedral added 2026-11-24)
- [docs/2D_Tilings_Reference.md](2D_Tilings_Reference.md) — mathematical
  background (uniform/Laves/wallpaper/quasiperiodic/aperiodic reference)
- [docs/briefs/](briefs/) — work briefs for delegated tiling jobs
  (`TEMPLATE.md` plus worked examples)

## Frozen history

- [docs/history/CHANGELOG.md](history/CHANGELOG.md) — the milestone
  changelog (append-only; orchestrator integrates)
- [docs/history/not-applicable.md](history/not-applicable.md) —
  rejected / not-applicable investigations (§7 quaquaversal)

## Where new content goes

- **New tiling in family F** → dispatch a brief
  ([docs/briefs/TEMPLATE.md](briefs/TEMPLATE.md); worked examples in
  that directory), code in `families/F/`, row + Per-tiling-documents
  link in `docs/families/F.md`, and the per-tiling document
  `docs/tilings/<id>.md`; the orchestrator integrates per the fleet
  protocol in [AGENTS.md](../AGENTS.md)
- **New milestone M<n>** → one changelog line in
  `docs/history/CHANGELOG.md` (appended at integration)
- **New candidate** → queue entry in `docs/ROADMAP.md`, dossier in
  `PROPOSED.md`
