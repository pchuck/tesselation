# Brief: composition-families — medallion-class unit cells on periodic nets (24d, instance 1)

> **Status: ✅ instance 1 LANDED 2026-11-25 — M172, reduced scope**
> (dual-net fill reading shipped, 0 seams; inference reading deferred
> to 24b — boundary docs shipped as the 24b feed; commit 4fc5400,
> integrated as M172). **Family:**
> islamic. **Candidate:** PROPOSED §24d (first-listed instance).
> **Effort:** L. **Precondition:** 24a medallion interface
> (M167) + 24c monster catalog (M169, extended M171) landed.

## Goal

Ship the first **composition-family** registry instance: a periodic
unit cell carrying **2–3 medallion classes** placed on a periodic net,
with the interstice filled by a complementary pattern — and test BOTH
readings of "complementary fill" on the same instance:

1. **Dual-net fill** — the interstice is filled by faces of the dual
   net, in the style of the landed `families/kuniform/dual.rs` house
   patterns (READ-ONLY reference: it lives in another family's
   directory; borrow the technique, never edit the file).
2. **Boundary-matched inference fill** — the interstice boundaries
   (the medallion classes' ray-exit/contact structure) are filled by
   the 24b hole-inference machinery once it lands.

Instance 1 (PROPOSED's first-listed): **dodecagon medallions at
hexagon centers + hexagrams at vertices** — dodecagon-class medallions
centered on the hexagon centers of a hexagonal (6.6.6)-based net,
hexagram-class medallions at the net vertices, interstice filled by
the readings above. The worker derives the actual contact geometry
exactly (which edges of the dodecagon touch what, at which rotation)
from the landed medallion records + net geometry — **never from a
picture**. Registry: one new islamic-family tiling, full battery.

The comparison is the intellectual payload: the same instance built
both ways, with a documented verdict on where each reading applies
(edge-to-edge vs overlap seams, which freedom degrees each consumes,
what each leaves for 24e). The comparison goes in the module docs +
the per-tiling doc + the report.

## Ground truth and geometry sources (pinned)

- **Medallion classes come from the landed M167 catalog + M163 motif
  machinery** — `crates/tess-tilings/src/medallion/`:
  `hexagram_62.rs` (the hexagram medallion record — the vertex class),
  `khatam_m53.rs` / `rosette_m56.rs` / `host312.rs` (dodecagon-class
  precedents and the 3.12.12 host), `mod.rs` (the `Medallion` trait,
  `Zq` exact arithmetic, `first_hit`, cos²-only angle convention).
  Re-derive placements in the exact shipped fields; a quantity outside
  the shipped fields is a STOP (see the 24a/24c field-gap notes in
  `medallion/monster/mod.rs`).
- **The underlying net**: the hexagonal (6.6.6) lattice machinery is
  the same one the islamic family's periodic tables use
  (`LatticeTiling`, `H = √3/2` exact forms). The medallions are
  *additional* geometry on the net, not a substitution of it.
- **Reference composition**: PROPOSED §24d names the composition
  classically (dodecagons at hexagon centers, hexagrams at vertices —
  a standard 12-fold ornamental arrangement). If the exact contact
  rule cannot be pinned from the landed records + first principles
  (which edges meet, edge-to-edge or with straps), STOP and report
  the underdetermination as a 24e freedom-set finding — do not pick
  silently. The interstice fill is where freedom lives; the medallion
  placements themselves must be exact.

## Files you own

- `crates/tess-tilings/src/families/islamic/<name>.rs` — the new
  module (cell construction + both fill readings + tests).
- `crates/tess-tilings/src/families/islamic.rs` — module decl +
  `register` slice + `classification` arm(s) + census `IDS`
  (+ `lattice_tilings()` if the instance is a lattice table).
- `docs/families/islamic.md` — the catalog row (guard-checked) + the
  Per-tiling-documents link line; a numbered build arc (the fill
  comparison is arc-worthy).
- `docs/tilings/<id>.md` — the per-tiling document (guard-checked):
  H1 = `t.name()` exactly, metadata line, the composition spec + the
  fill-reading comparison, See-also.
- Scratch: **ABSOLUTE path**
  `/home/pcharles/dev/pchuck/tesselation/.scratch/composition-24d/`
  (mkdir -p; Python spikes, batteries, the comparison table). Never
  the worktree's own `.scratch/`.

**Do not touch anything else.** Not README, not `lib.rs`, not
`docs/history/CHANGELOG.md`, not `PROPOSED.md`/`ROADMAP.md`, not
`families/kuniform/dual.rs` or any other family's files, not the UI
crates, not `medallion/` (consume it read-only).

## Constraints (from AGENTS.md — verbatim obligations)

1. Exact geometry: express vertices in radicals via the `H = √3/2`
   constant — never decimal literals (medallion records use the `Zq`
   `(a+b√d)/den` tuples; stay in them).
2. Verify numerically BEFORE writing Rust: Python spike, **stdlib
   only**, in the scratch dir, over a ±3×±3 lattice patch of the cell:
   - `sum(cell areas) == |det(a1, a2)|` to ~1e-15;
   - every interior edge shared by exactly 2 cells (key edges by
     endpoint pairs rounded to 6 digits);
   - every interior vertex angle multiset == the vertex figure;
   - a fine grid sample (shifted by an irrational offset) covered
     exactly once.
3. Emission gotcha: when several geometric tiles of one class share a
   vertex, the cell must emit each geometric tile exactly once.
4. Name verified against the literature before coding (Wikipedia or
   the repo's cited sources); if the composition has no canonical
   name, propose a descriptive registry name and say so.
5. Tile budget `MAX_TILES = 250_000`; keep generation cheap (GUI
   regenerates every frame).
6. `cargo check --workspace` zero warnings; full suite green.

## Dispatch gates (orchestrator-owned; worker ignores this section)

- **Gate 1 — islamic family free:** 24g (`subdivision_systems.rs`,
  in flight) must land first — family files are the collision unit.
- **Gate 2 — 24b landed:** the inference-fill reading consumes 24b's
  machinery. If Gate 1 opens while 24b is still in flight, the
  orchestrator may dispatch a reduced scope (dual-net fill + interstice
  boundary documentation as the 24b feed) with the inference reading
  as an explicitly deferred phase — the report must then say which
  reading shipped.
- Companion briefs (later instances: khatam rings;
  decagon-and-bowtie pairs) get their own files after instance 1's
  comparison verdict — do not start them.

## Verification battery (what "done" means)

- The standard 3-test trio (`*_covers_region`, `*_rotated_scaled`,
  `*_deterministic`) copied from an islamic precedent module
  (`andritz_bowtie_hexagon.rs` or `khatam_girih.rs` are the closest
  multi-class precedents).
- Composition exactness: the cell's medallion-class tiles are exactly
  the landed records' geometry under the derived placement (vertex
  sets equal in the exact field — a test asserting record→cell
  identity, the M169 cross-check style).
- Interstice partition exactness (Python, then pinned in Rust):
  `sum(interstice face areas) == cell area − sum(medallion areas)`
  exactly in the field; the medallion boundaries are edge-to-edge with
  the interstice faces under reading 1 (any seam must be a documented
  finding, not a tolerated fudge).
- Both readings produce a **complete** partition (the fill comparison
  table lists, per reading: face count, face class count, edge-to-edge
  status, freedom degrees consumed, symmetry of the result).
- Symmetry claim verified by the `classify.rs` coset audit if a
  wallpaper group is claimed for the filled cell.
- Expected census deltas in the report: family n → n+1, built-ins
  163 → 164, memberships 184 → 185 (re-derive against main at report
  time — the orchestrator assigns the M number and edits the shared
  files).

## Deliverable (worker report)

Per the fleet protocol in `tessellix/AGENTS.md`: diff-scope statement
(exactly the owned files); verification evidence with exact numbers;
expected census deltas; proposed commit-message body; **the fill
comparison table** (both readings, same instance) and the verdict; any
STOP hit, verbatim. One commit, subject like
`engine: composition family — dodecagon/hexagram medallion cell with dual-net vs inference fill (24d instance 1)`, message via
`git commit -F /tmp/opencode/composition-24d-commit-msg.txt`
(session-unique filename). The orchestrator assigns the M number at
integration.