# Brief: isohedral-generator — IH1–IH93 machinery + showcase types

**Family:** `isohedral` (NEW — scaffolded empty by the orchestrator;
cargo feature `isohedral` default-on; see `families/isohedral.rs`) ·
**Candidate:** PROPOSED.md §13 · **Effort:** L · **Tier:** frontier ·
**Sources:** Grünbaum–Shephard, *Tilings and Patterns* (the IH1–IH93
classification; [GS87] already the repo's cited edition); Craig
Kaplan, https://isohedral.ca (the incidence-symbol + parameter tables
per type — the machine-readable authority); Tom McLean's per-type
templates mirrored at jaapsch.net (edge-shaping conventions). The
dossier sketch lives in PROPOSED §13; this brief corrects ONE of its
claims (category, below) and scaffolds the rest.

## What you build

1. **Design doc** `docs/isohedral-generator-design.md` (standalone
   design-doc precedent: `docs/cyclotomic-field-layer.md`) — the
   edge-orbit data model, the incidence-symbol representation, the
   parameter table encoding per type, the exactness policy, and the
   category policy (below). Write this FIRST; the engine implements it.
2. **Engine** — `families/isohedral/` modules: an IH type = incidence
   symbol (tile symbol + adjacency symbol) over a topology, generator
   inputs = IH type + edge-shape parameters + tile proportions; shaped
   edges become **exact parameterized polylines shared per edge orbit**
   (both sides of an edge emit bit-identical point sequences — tiles
   stay polygons, the coverage/determinism contracts hold unchanged).
   The 11 Laves topologies are already implemented exactly in
   `families/laves/` — read-only reference for topology geometry.
3. **3–5 showcase registry instances** (not all 93) — the §13 sketch:
   IH1 (p1 hexagon), the IH5–IH7 pgg/p4g pentagon interlocks, one
   hexagon-ring type — but VERIFY each type's topology + adjacency
   symbol against Kaplan's tables before pinning it, and choose the
   final set so every instance satisfies the category rule below.
4. **Docs:** family-doc rows (`docs/families/isohedral.md` — heading
   bump 0 → N, one Implemented row + one Per-tiling-documents link per
   instance), one per-tiling doc per instance (H1 = registry `name()`
   exactly; template: `docs/tilings/andritz-bowtie-hexagon-10gon-3.md`).

## The category rule (hard constraint — corrects PROPOSED §13)

§13 says "Category Wallpaper". **That is not shippable:** the Wallpaper
tab is a pinned 17-card group catalog — `lib.rs` asserts every
Wallpaper-tab card carries a group, `groups.len() == wall.cards.len()`,
and `groups.len() == 17` (the 17 groups minus the gap table). ANY new
Wallpaper membership — primary **or secondary** — adds an 18th card and
breaks the guard. Do not touch `WALLPAPER_GROUP_GAPS` to make room.

Rule, in preference order:

- **Primary `TilingCategory::Laves`** for instances whose topology is
  one of the 11 Laves nets (the §13 sketch's showcase types are
  naturally these: 6³ for IH1; the pentagon interlocks on 3³.4² /
  3².4.3.4). The Laves tab already hosts topological realizations
  beyond the pure duals (self-dual regulars as secondaries); census
  pins move, no structure guard.
- **Primary `TilingCategory::PentagonTypes`** only for pentagon-shaped
  instances if one doesn't fit Laves.
- If a type you want has neither home, either pick a different type or
  STOP and report the taxonomy conflict — do not invent a category
  (the ten tabs are UI-pinned) and do not break a guard.

Classification: construct `TilingClass` inline in
`families/isohedral.rs::classification()` (the `floret-pentagonal` arm
in `families/laves.rs` shows the inline-struct idiom);
`isohedral: Some(n)` already exists on `TilingClass` — record the type
number there. Do NOT add a helper to `classify.rs` (conflict surface
with other in-flight jobs).

## Exactness policy (the design question the design doc must settle)

Shaped-edge parameters are intrinsically continuous; the repo bar is
exact radicals with floats only at the emission boundary. Default
position to adopt unless the design doc argues otherwise: **showcase
instances pin exact edge shapes** (rational/√ control points per type —
each showcased type's edge orbits get ONE exact canonical shape each,
documented in the per-tiling doc), while the ENGINE accepts arbitrary
float parameters behind a documented emission-boundary policy. The
battery runs on the shipped instances (exact), not on float params.

## Battery

- Per instance: standard 3-test trio
  (`<id>_covers_region`, `_rotated_scaled`, `_deterministic`) +
  coverage exactly once (irrational-offset sample grid), determinism.
- **Shared-edge identity (the IH-specific invariant):** for every
  interior edge, both adjacent tiles' boundary point sequences over
  the shared span must be identical up to reversal — verify
  programmatically (exact point-sequence comparison), report the edge
  census.
- **Symmetry check:** each instance's declared wallpaper-group
  generators map the emitted tile multiset to itself (programmatic:
  apply the group ops, compare vertex sets up to the tiling's
  translation lattice).
- Edge-orbit census: number of orbits per type matches Kaplan's table.

## Files you own

- `docs/isohedral-generator-design.md` (new).
- `crates/tess-tilings/src/families/isohedral.rs` — register slice,
  classification arms, census `IDS`, `lattice_tilings` (hand-applied
  hunks; NEVER rustfmt this file or `families/mod.rs`).
- `crates/tess-tilings/src/families/isohedral/*.rs` — engine + one
  module per instance (`cargo fmt` allowed on YOUR new modules only).
- `docs/families/isohedral.md` — heading bump + rows + link lines.
- `docs/tilings/<id>.md` — one per instance.
- Scratch `/tmp/opencode/isohedral/` (append-only `NOTES.md`),
  checkpoint to repo-root `.scratch/isohedral/` at the end.

**Do not touch:** `families/mod.rs`, `lib.rs` (the two orchestrator
census guards are EXPECTED to fail — capture verbatim), `classify.rs`,
`Cargo.toml` (the family feature exists), README, CHANGELOG, INDEX,
ROADMAP, PROPOSED, other families (incl. `laves/` — read-only), UI
crates, AGENTS.md.

## Protocol

1. Design doc first; then engine; then instances. Spike each
   showcased type's geometry in Python (stdlib, ≤240 s/experiment)
   before Rust — pin the incidence symbol, the edge orbits, and the
   exact canonical edge shapes; verify the shared-edge identity on the
   spike output.
2. `cargo check --workspace` zero warnings; `cargo test --workspace`
   green on your base except the two orchestrator census guards
   (verbatim capture = delta evidence). Expected deltas: isohedral
   family 0 → N; built-ins 153 → 153+N; memberships 167 → 167+N;
   chosen tab +N per instance (re-derive against YOUR base's pins and
   report what you saw).
3. One worktree commit per landing if you stage (design doc + engine,
   then instances) or a single commit if it lands whole; subjects
   `isohedral-family: IH design doc + generator engine` /
   `isohedral-family: IH showcase instances` — NO M numbers, NO pushes;
   long messages via `git commit -F <file>`.
4. Report: diff scope; design-doc summary (data model + exactness +
   category policies, one paragraph each); instance list (type,
   topology, group, edge-orbit count, category) with battery numbers
   (exact); Rust pass counts + verbatim guard failures; proposed
   commit body; scratch checkpoint path.