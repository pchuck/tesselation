# Brief: kuniform-n17 — the pgg sibling of n16

> **LANDED as M140** (see docs/history/CHANGELOG.md). Do not dispatch.

**Family:** `kuniform` · **Candidate:** PROPOSED.md "Still missing from
the closed 20-list: … n17 (the pgg sibling of n16)"; queue context in
`docs/families/kuniform.md` §62 and the M119 changelog entry ·
**Effort:** M · **Precedent to mirror:**
`crates/tess-tilings/src/families/kuniform/kuniform.rs` (`kuniform_n16`,
M119) and the §62 arc for methodology.

### Goal

Land `kuniform-n17`: variant 2 of the Wikipedia 33344-33434 two-uniform
tiling — the same vertex-class pair as the landed `kuniform-n16`
(`3³.4²` + `3².4.3.4`) but arranged with **pgg** symmetry instead of
n16's p4g: no mirrors, two perpendicular glide families, 2-fold
centers, no 4-fold center anywhere. Data table on an exact
`LatticeTiling`, registered in the kuniform family, closing the third
of the three gaps in the closed 20-list (n6, n7 remain open — not this
brief).

### Literature basis

- Wikipedia "2-uniform tiling" list, the 33344-33434 entry (n16 is
  variant 1, n17 variant 2) and Krotenheerdt's closed list.
- **Known numbering trap (quote from the §62 arc):** one caption-extraction
  pass over a saved wiki page found "**no `n17` in the wiki set at
  all**", while the M119 changelog states "variant 2 = n17, pgg,
  remains open". Step 1 of this brief is to re-resolve n17's identity
  against the live Wikipedia list + Krotenheerdt's list and record the
  outcome (which figure/file is variant 2, what isohedral/isotoxal
  annotation it carries) before any geometry. If the sources
  contradict, stop and report — do not guess a numbering.

### Files you own

- `crates/tess-tilings/src/families/kuniform/kuniform.rs` — add
  `kuniform_n17` alongside `kuniform_n16` (same module file; it is a
  family file you own).
- `crates/tess-tilings/src/families/kuniform.rs` — module wiring:
  `register` slice, `classification` arm, census `IDS`,
  `lattice_tilings()` entry.
- `docs/families/kuniform.md` — the catalog row (guarded). Arc only if
  the basis derivation turns out arc-worthy.
- `/tmp/opencode/kuniform-n17/` — scratch.

**Do not touch:** `lib.rs` (census pin, guards, culling test), README,
`docs/history/CHANGELOG.md`, `docs/INDEX.md`, the duals module, other
families. The orchestrator wires the culling test and writes the
M-entry.

### Constraints

(Verbatim from AGENTS.md — see TEMPLATE.md §Constraints for the full
list; the ones that bite here:)

1. Exact geometry in ℚ(√3): n16's basis is
   ⟨(h,−h),(h,h)⟩, h = (3+√3)/2, det 6+3√3 — the **starting
   hypothesis** for n17, not a given; derive the true basis in the
   spike from the variant-2 figure. Never decimal literals.
2. Python spike first (stdlib only) over a ±3×±3 patch: area sum ==
   |det| to ~1e-15, interior edge parity == 2 (6-digit edge keys),
   vertex angle multisets == {3³.4², 3².4.3.4} classes, irrational
   grid coverage.
3. Emission gotcha: triangles at shared vertices (n16's cell emits 18
   tiles = 6 squares + 12 triangles with per-orbit emission choices);
   n17's cell will have its own orbit structure — derive it, don't
   copy n16's.

### Verification battery

- **Group claim, pgg, by coset audit** (the headline claim): every
  reflection candidate coset is NOT pure (contains off-mirror
  translations) — no mirror lines exist; both diagonal/axis glide
  families are true glides (an integral shift cannot cancel the
  half-period flip); 2-fold centers at the glide-intersection
  pattern; enumerate to confirm **no 4-fold center** (that is what
  separates n17 from n16's p4g).
- **Torus orbit census** vs the resolved wiki annotation for n17
  (n16's precedent: 4 tile orbits 8+4+4+2, 5 edge orbits =
  4-isohedral/5-isotoxal): tile/vertex/edge orbit counts and sizes
  must match what step 1 recorded.
- Vertex-class census per cell: exact counts of 3³.4² and 3².4.3.4
  vertices, agreeing between spike and Rust.
- Standard battery: area sum, edge parity, irrational coverage,
  translate round-trip (6-digit keys), culling-compatible lattice
  (`lattice_tilings()` entry shape copied from n16's).

### Standard tests

Copy the trio from `kuniform_n16`'s tests (`…_covers_region`,
classification, determinism) under a `kuniform_n17` naming.

### Deliverable (worker report)

1. Diff scope (only the files above).
2. The step-1 numbering resolution (source, anchor, annotation) — the
   changelog entry will quote it.
3. Battery numbers: coset-audit verdict per reflection candidate,
   orbit censuses, vertex-class counts, area/parity results; Rust test
   pass counts.
4. Census deltas: TwoUniform 17 → 18; built-ins 122 → 123;
   memberships 136 → 137; **is a `LatticeTiling`** → remind the
   orchestrator to add the `lattices.extend(...)` culling line.
5. Proposed commit-message body (lattice, cell contents, verification).
   No M number — assigned at integration.

Natural follow-up (separate brief, not this one): the face-centroid
dual `kuniform-dual-n17` (n16 → M121 precedent).