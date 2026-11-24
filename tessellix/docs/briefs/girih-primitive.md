# Brief: girih-primitive — the four missing classical girih primitives

> **LANDED as M131–M134** (see docs/history/CHANGELOG.md). Do not dispatch.

**Family:** `islamic` · **Candidate:** revival of the pre-rework
`girih-primitive` fork (PROPOSED.md §23f note; the fork's disposition
is now "revived as a brief" — the old fork/branch is gone with the old
machine, so this runs fresh from main) · **Effort:** L · **Tier:**
strong · **Precedent to mirror:** `crates/tess-tilings/src/families/islamic/girih.rs`
(`girih_shesh_band` — the fifth primitive, already shipped), and the
exact-field conventions of `girih_qc.rs` (+ `girih_sc.rs`,
`girih_w85.rs`, `khatam_girih.rs`, `najm.rs` for table style).

### Goal

Land the four missing classical girih primitive tiles as exact data
tables (`LatticeTiling`s, like shesh-band) in documented periodic
edge-to-edge arrangements:

- **girih-decagon** — regular decagon, angles 144° ×10
- **girih-bowtie** — bowtie, angles 216°,72°,72° ×2
- **girih-rhombus** — golden rhombus, angles 72°,108°,72°,108°
- **girih-pentagon** — regular pentagon, angles 108° ×5

Suggested landing sequence: **decagon → bowtie → rhombus + pentagon**
(three landings; the classic decagonal patch assembles
decagon + bowtie + rhombus + pentagon, which is the natural
consistency check across the four tables). A cell may contain forced
infill pieces where the documented arrangement requires them (emission
gotcha applies).

### Literature basis + salvage

- Sources: the classical girih primitives (Lu–Steinhardt figures;
  Bielefeld Tilings Encyclopedia girih pages). Verify each shape and
  its documented periodic arrangement before pinning tables.
- **SALVAGE — a dead strong-tier session's exact-arithmetic
  prototypes, archived at
  `/home/pcharles/dev/pchuck/tesselation/.scratch/salvage-20261120/tmp/girih_prim/`
  (local to this machine; guidance ONLY — never battery-green,
  re-verify everything you reuse):**
  - `gexact.py` — exact arithmetic over **K = Q(a)**, a = 2cos(π/10),
    minpoly **a⁴ − 5a² + 5 = 0** ("field rule verbatim from
    `girih_qc.rs` kmul" — the shipped Rust module already carries this
    quartic layer; check it first). Elements: 4-tuples of Fractions
    c0 + c1·a + c2·a² + c3·a³; fmul reductions
    a⁴ = 5a²−5, a⁵ = 5a³−5a, a⁶ = 20a²−25. Derived constants with
    exact algebraic certificates: √5 = a²−2, φ = (a²−1)/2,
    cos36° = (a²−2)/2, sin36° = (a³−3a)/2.
    CAUTION recorded in its header: float outputs in that environment
    proved unreliable — validate by exact tuple algebra, cross-checked
    against `girih_qc.rs`'s shipped reference tables.
  - `gengine.py` — periodic-tiling search prototype: tiles
    DEC/BOW/RHOMB/PENT (+HEX = shesh-band); **all edge directions are
    multiples of 36°** under the vertex-up convention, so placements
    only rotate by multiples of 36° and coordinates live in
    x ∈ Q(√5), y ∈ S36·Q(√5); vertex-angle bookkeeping in integer
    sectors (10 per full turn: DEC corner 4, HEX 2/4, BOW 2/6,
    RHOMB 2/3, PENT 3).
  - `gtest_r.py`, `gmod.py`, `gdbg.py`, `gexact_old_backup.py` —
    debug/test helpers; salvage as guidance only.
  - Derivation hint from the dead session (decagon): verify chord
    directions / sector structure (**consecutive edge sector
    difference −1 mod 10 = 324°**) before pinning tables. Keep an
    append-only `NOTES.md` in your scratch dir so state survives a
    session death.
- **BANS:** never view or dump `paper.pdf` (1.7 MB) or `som.pdf`
  (5.8 MB) in the archive dir; extract small text slices via script if
  truly needed, else derive from the girih.rs precedents. Keep tool
  outputs small; final report < 40 lines.

### Files you own

- `crates/tess-tilings/src/families/islamic/girih_decagon.rs`,
  `girih_bowtie.rs`, `girih_rhombus.rs`, `girih_pentagon.rs` — the new
  modules (or one module file following `girih.rs` conventions, your
  choice — state it).
- `crates/tess-tilings/src/families/islamic.rs` — module decls +
  `register` slices + `classification` arms + census `IDS` +
  `lattice_tilings()` entries (these ARE lattice tables — mirror
  shesh-band's entry shape).
- `docs/families/islamic.md` — the catalog rows (guarded).
- `/tmp/opencode/girih-primitive/` — scratch.

**Do not touch:** `lib.rs` (census pin, guards, culling test — you add
`lattice_tilings()` entries in the family file and FLAG the culling
lines in your report), README, `docs/history/CHANGELOG.md`,
`docs/INDEX.md`, the sibling girih modules (read-only references),
other families, the UI crates.

### Commit exception (this brief only)

Land per shape: make **one worktree commit per landing**
(decagon / bowtie / rhombus+pentagon), each passing the bar below, so
the orchestrator can cherry-pick three per-tiling commits and assign
M numbers. **Never push; never touch shared files; commit message
subjects without any M number** (e.g. `girih-primitive: decagon data
table`).

### Constraints (from AGENTS.md, the ones that bite here)

1. Exact geometry — never decimal literals. Reuse the shipped
   36°-grid surd/K-layer conventions from `girih_qc.rs`/`girih.rs`
   where they suffice; the quartic K = Q(2cos(π/10)) layer is the
   fallback for anything the surd layer cannot pin (the salvaged
   `gexact.py` shows the tuple arithmetic).
2. Python spike first (stdlib only, ≤240 s per experiment), per
   shape: `sum(cell areas) == |det(a1,a2)|` to ~1e-15; interior edge
   parity == 2 (6-digit keys); interior vertex angle multisets == the
   documented figure; fine-grid sample (irrational offset) covered
   exactly once.
3. Emission gotcha: if a cell carries infill pieces shared across
   cells, each geometric tile is emitted by exactly one cell
   (AGENTS.md step 3).
4. Tile budget `MAX_TILES = 250_000`; keep generators cheap.
5. Keep `cargo check --workspace` warning-free.

### Verification battery

- Per shape: the periodic-table battery above, plus the sector/census
  checks (edge directions on the 36° frame; angle multisets in integer
  sectors).
- **Assembly cross-check:** decagon + bowtie + rhombus (+pentagon)
  tables must be mutually consistent — shared edge lengths/directions
  on the 36° frame; the classic decagonal patch assembles without gap
  or overlap in the spike (exact where the surd layer allows, 6-digit
  keys otherwise).
- Rust: each shape's table reproduces the spike's numbers; the 3
  standard tests per tiling (`girih_decagon_covers_region`, …
  `_rotated_scaled`, `…_deterministic`, etc.).
- Optional recon: `export --mode proto` SVG parse. Evidence is the
  numbers, not the look.

### Standard tests

The 3-test trio per tiling, plus the battery as in-module tests —
copy the shapes from `girih.rs`/`girih_qc.rs` test modules.

### Deliverable (worker report)

Per landing (three reports, or one report with three clearly separated
landing sections):

1. Diff scope for that landing (only the owned files).
2. Geometry pin: source, exact vertices, arrangement, infill pieces
   (if any), field layer used.
3. Battery numbers per shape; Rust pass counts.
4. Census deltas per landing: islamic family +1 (22 → 26 cumulative);
   Islamic tab +1 (21 → 25 cumulative); built-ins +1; memberships +1
   each (read the pinned totals in `lib.rs` at your start; the
   orchestrator resolves cumulative state at integration). IS a
   `LatticeTiling` ×4 → remind: orchestrator adds four
   `lattices.extend(...)` culling lines.
5. Proposed commit-message body per landing. No M numbers — assigned
   at integration.