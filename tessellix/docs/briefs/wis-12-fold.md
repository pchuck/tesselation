# Brief: WIS-12-fold — Watanabe–Ito–Soma 12-fold substitution, reconstruction prepass

**Family:** quasiperiodic · **Candidate:** Bielefeld Substitution Tilings
Encyclopedia entry "Watanabe-Ito-Soma 12-fold" · **Effort:** L
(implementation dispatch, after unblock) · **Precedent to mirror:**
`crates/tess-tilings/src/families/quasiperiodic/stampfli.rs`,
`crates/tess-tilings/src/families/quasiperiodic/golden_rhomboid.rs`

### Status and scope

Research-only prepass, completed. This document is **docs-only**:
`crates/` is untouched in this commit. Implementation is **BLOCKED**
until (a) the GLB n=7 field-layer work lands (the quasiperiodic family
slot is owned by that session), and (b) the cluster-geometry
blockers below are resolved. Facts pinned with confidence are marked
PINNED; the rule-table entries are candidates under the STOP clause
and are marked AMBIGUOUS / UNCONFIRMED / BLOCKED. Never present the
candidate tables as the certified rule.

### Goal

Land the Watanabe–Ito–Soma 12-fold tiling in the quasiperiodic family:
3 prototiles (60° equilateral triangle, square, 30° rhombus),
dodecagonal, inflation **λ = 2+√3**, FLC w.r.t. rigid motions,
**non-conformal patch-based** substitution (jagged clusters, not
λ×-parent frames — proven below).

### Literature basis

- **[WSI95]** Watanabe, Soma, Ito, "Tilings by equilateral triangles,
  squares and rhombi", *Acta Cryst.* A51 (1995) 936–942.
- Bielefeld entry (live-verified 2026-11):
  <https://tilings.math.uni-bielefeld.de/substitution/watanabe-ito-soma-12-fold/>
  with assets `rule.png` (six panels, two per prototile), `patch.png`,
  `patch.pdf` under `/img/substitution/watanabe-ito-soma-12-fold/`.
- **Provenance trap (cite in the module header):** the entry page
  states verbatim that "the corresponding substitution rules given in
  Fig. 2(iii) of the paper are not unique ... For some time the exact
  substitution rules which generate the square level-3-supertile in
  Fig. 3 of the paper remained unclear. Finally Alessandro Musesti and
  Maurizio Paolini submitted the correct set of substitution rules in
  January 2023." → **the [WSI95] Fig. 2(iii) rules are NOT the rule
  source; the Bielefeld rule.png panels are (Musesti & Paolini,
  January 2023).**
- Entry parameters: 3 prototiles, λ = 2+√3, dodecagonal, FLC w.r.t.
  rigid motions.

### Pinned facts (prepass, exact)

**Basis convention (PINNED).** H = s/2 = 11.59765 content units
(s = 23.1953 = square edge in `patch.pdf` content units; PDF device
transform = 0.1×). Every patch vertex is an integer 4-tuple
(P,Q,R,S) ∈ ℤ⁴:

```
x = (s/2)·(P + Q√3),  y = (s/2)·(R + S√3)
```

i.e. **M = span_ℤ{(2,0),(0,2),(√3,1),(1,√3)} in H-units; the vertex
module is the full ℤ⁴** (a single-coset rank-4 ℤ[√3] fit fails,
residuals up to 0.5 H). Tile **edge** vectors live in the index-4
sublattice {P≡S (mod 2), Q≡R (mod 2)}. 30° rotation step, k mod 12:

```
rot4(P,Q,R,S) = ((3Q−R)/2, (P−S)/2, (P+3S)/2, (Q+R)/2)
```

Shape periods (footprint symmetry): tri 4, sq 3, rho 6.

**Prototiles (PINNED).** CCW, y-up, H-units, corners in (P,Q,R,S);
edge length 2H; 2·area in H² as ℤ[√3] pair (a,b) = a + b√3:

| tile | corners (P,Q,R,S) | 2·area |
|---|---|---|
| tri (equilateral △, 60°) | (0,0,0,0),(2,0,0,0),(1,0,0,1) | (0,2) |
| sq (square) | (0,0,0,0),(2,0,0,0),(2,0,2,0),(0,0,2,0) | (8,0) |
| rho (30° rhombus) | (0,0,0,0),(2,0,0,0),(2,1,1,0),(0,1,1,0) | (4,0) |

**Inflation + partition evidence (PINNED, exact ℤ[√3]).**
λ = 2+√3, λ² = 7+4√3 ≈ 13.928. Per-prototile supertile 2·area pairs
and compositions (the composition is the **unique** nonnegative
integer solution of the area equation; zero excess in every case):

| supertile | children | composition | 2·area | λ² check |
|---|---|---|---|---|
| tri-sup | 11 | 7 tri + 2 sq + 2 rho | (24,14) | (7,4)·(0,2) = (24,14) ✓ |
| rho-sup | 13 | 8 tri + 2 sq + 3 rho | (28,16) | (7,4)·(4,0) = (28,16) ✓ |
| sq-sup | 26 | 16 tri + 4 sq + 6 rho | (56,32) | (7,4)·(8,0) = (56,32) ✓ |

**Rule nature: non-conformal (PINNED).** A conformal λ×-parent frame
is impossible: λ·(2H) = (4+2√3)·H cannot be tiled by collinear 2H
child edge steps (collinear-edge proof; 0/143,892 frame candidates).
Each prototile expands to a jagged **cluster** tiled exactly by its
children (closed simple boundary, all interior edges paired, exact
area identity — certified in exact ℤ[√3]).

**Panel / scale facts (PINNED, measured).** `rule.png`: 6 panels,
panels 0/1 = tri-sup, 2/3 = rho-sup, 4/5 = sq-sup. Raster fit:
k (px/H) = 14.5644 (tri-sup, anchor px (104, 120.5)), 14.6088
(rho-sup, anchor (353, 158.5)), 14.6796 (sq-sup, anchor
(647, 81.5)); per-child IoU 0.619–0.923; ℤ⁴/M-snap residual
≈ 0.02–0.05 H ⇒ panel child placements are lattice-snapped and
accurate to ~0.05 H. `patch.pdf`: 37,635 tiles = **18,302 tri +
11,644 sq + 7,689 rho** (Bielefeld patch census); the tiling
occupies the lower half of a 490×490 H patch, ≈ 245 s per axis.
Note: 8,852 of the parsed tiles form a second interleaved tiling
layer with edge vectors outside the M lattice (e.g. ℤ⁴ (97,−55,1,0))
— not parse noise, not part of the rule.

### Rule table (candidates — STOP-clause verdicts)

Cluster geometry is **not** certifiable from the assets at hand:
s18b exact alignment gives 0 exact ℤ⁴ hits (best float residual
2.07 H against a ~0.05 H noise floor) and the s18d raster-IoU
discriminator shows no congruence class overlays the panel raster.
**STOP clause invoked: do not guess which class (if any) is the true
rule occurrence.** Candidates below are the closest measured
occurrences in the Bielefeld patch, recorded for the next source
attempt (better panel fit, or the Musesti–Paolini rule tables
directly).

**tri-sup candidate (AMBIGUOUS).** 15 certified congruence classes
exist in the patch; the one with the closest s08 panel alignment
(best float cost 2.07 H, 4/11 children ≥ 1.5 H off) is shown below.
Its own raster-IoU is 3.799/11 (worst 0.000) vs oracle 9.461/11;
the class-level best is 3.987 (4 of 15 groups, mirrored) — no
class overlays the panel:

| shape | krot (mod 12) | m (P,Q,R,S) | m in H-units |
|---|---|---|---|
| rho | 7 | (3,1,−1,−1) | (4.7321, −2.7321) |
| rho | 8 | (3,1,−1,−1) | (4.7321, −2.7321) |
| sq | 7 | (2,1,1,0) | (3.7321, 1.0000) |
| sq | 9 | (1,0,0,−1) | (1.0000, −1.7321) |
| tri | 7 | (1,0,0,−1) | (1.0000, −1.7321) |
| tri | 7 | (2,1,−1,−2) | (3.7321, −4.4641) |
| tri | 8 | (2,0,0,0) | (2.0000, 0.0000) |
| tri | 8 | (3,1,−3,−1) | (4.7321, −4.7321) |
| tri | 9 | (2,0,2,0) | (2.0000, 2.0000) |
| tri | 10 | (0,0,0,0) | (0.0000, 0.0000) |
| tri | 10 | (1,0,−2,−1) | (1.0000, −3.7321) |

Boundary: 15 corners; composition 7 tri + 2 sq + 2 rho; cluster
2·area (24,14) exact.

**rho-sup candidate (UNCONFIRMED).** One congruence class is fully
contained in the patch crop (13 certified leaves; not provably the
only class globally). Best measured: exact_hits = 0, raster-IoU
3.718/13 vs oracle 11.357/13:

| shape | krot (mod 12) | m (P,Q,R,S) | m in H-units |
|---|---|---|---|
| rho | 2 | (0,0,0,0) | (0.0000, 0.0000) |
| rho | 4 | (5,3,−1,−1) | (10.1962, −2.7321) |
| rho | 11 | (0,0,0,0) | (0.0000, 0.0000) |
| sq | 0 | (3,2,−2,−1) | (6.4641, −3.7321) |
| sq | 1 | (3,1,−1,−1) | (4.7321, −2.7321) |
| tri | 0 | (0,0,0,0) | (0.0000, 0.0000) |
| tri | 0 | (1,1,−1,−1) | (2.7321, −2.7321) |
| tri | 1 | (2,1,−1,0) | (3.7321, −1.0000) |
| tri | 1 | (5,2,−2,−1) | (8.4641, −3.7321) |
| tri | 2 | (4,2,−2,−2) | (7.4641, −5.4641) |
| tri | 2 | (5,3,−1,−1) | (10.1962, −2.7321) |
| tri | 11 | (2,0,0,0) | (2.0000, 0.0000) |
| tri | 11 | (3,1,−1,−1) | (4.7321, −2.7321) |

Boundary: 20 corners; composition 8 tri + 2 sq + 3 rho; cluster
2·area (28,16) exact.

**sq-sup (BLOCKED).** 0 certified leaves: exhaustive
composition-growth (792,000 nodes, 0 node-cap hits, all 47 feasible
components) found **no** exact-composition 26-tile set in the
patch. Composition (16/4/6) and 2·area (56,32) ARE pinned; the
cluster shape is not. No candidate table exists.

**Unblock path:** a higher-quality panel/vector source for the child
placements (Musesti–Paolini submission, or a re-render of
`rule.png` at higher resolution) that certifies one class per
prototile at IoU ≥ 0.95 of the s18d oracle calibration (or an
independent derivation, e.g. from the [WSI95] level-3 square
supertile in Fig. 3 of the paper).

### Verification battery (measured pins — what "done" means at
implementation)

Per-check numbers are prepass measurements (scratch:
`/tmp/opencode/wis12-recon/`); the implementation must reproduce or
supersede each:

1. **Composition census** (s18 composition-growth,
   ordered-frontier connected-set DFS + exact ℤ[√3] certification):
   tri-sup 150 certified leaves / 15 congruence classes; rho-sup
   13 / 1 class in crop; sq-sup 0 (exhaustive). Re-run on any new
   cluster source: the pinned cluster must certify
   (composition + area identity, closed simple boundary, all
   interior edges paired).
2. **Raster-IoU discriminator** (s18d, per-child best-IoU vs the
   `rule.png` panel raster, footprints at raw mod-12 krot):
   oracle (s08 self-overlay) tri-sup 9.461/11 (worst 0.619),
   rho-sup 11.357/13 (worst 0.747); no-match best 3.987 (tri, 4
   groups at 3.9868) and 3.718 (rho), worst child 0.000 in every
   group. Acceptance for the pinned rule: group sum within 0.95 of
   oracle **and** worst-child ≥ 0.6.
3. **Panel fit** (s08): per-child IoU 0.619–0.923; k within 5 % of
   the area-derived median; M-snap residual ≈ 0.02–0.05 H.
4. **Exact area identities** (ℤ[√3], 2·area): (24,14)/(28,16)/(56,32)
   per the table above, zero excess, S = 0.
5. **λ² law:** λ² · 2·area(parent) = 2·area(supertile) per row
   (table above), exact.
6. **Bielefeld patch census:** 18,302 tri / 11,644 sq / 7,689 rho
   (37,635 total; cross-check report only, `patch.pdf`).
7. **Substitution mechanics** (in-module Rust tests, mirroring
   `glb-binary-n7.md` battery shape): level-1 assembly per shape
   (closed boundary, edge pairing, count vector = composition),
   depth-3 assembly passing validate-level-equivalent checks,
   orientation census (all 12 dodecagonal edge directions present,
   per-shape krot distribution stable across levels), area law per
   rule line, PF = λ² = 7+4√3 to float tolerance.

### Files you own (implementation dispatch)

- `crates/tess-tilings/src/families/quasiperiodic/wis_12_fold.rs` —
  the new module (proposed name; registry id to be assigned in the
  family doc row).
- `crates/tess-tilings/src/families/quasiperiodic.rs` — module decl,
  `register` slice, `classification` arm, census `IDS`.
- `docs/families/quasiperiodic.md` — catalog row (guard-checked);
  build-arc entry only if the unblock introduces a new technique.
- `/tmp/opencode/wis-12-fold/` — scratch (Python spike, batteries).
- This prepass document itself: `docs/briefs/wis-12-fold.md`
  (docs-only commit).

**Do not touch anything else** — not `lib.rs` (census pin, guards,
culling test), not README, not `docs/INDEX.md`, not
`docs/history/CHANGELOG.md` (M number written by the orchestrator),
not other families, not the UI crates.

### Constraints (from AGENTS.md)

1. Exact geometry: vertices as (P,Q,R,S) integer tuples over the
   basis above (ℤ[√3] pairs (a,b) = a + b√3 for areas) — never
   decimal literals.
2. Verify numerically BEFORE writing Rust: Python spike, **stdlib
   only**, in `/tmp/opencode/wis-12-fold/`, foreground with
   per-step output, ≤ 240 s per experiment.
3. Emission: when several geometric tiles of one class share a
   vertex, emit each geometric tile exactly once (AGENTS.md step 3).
4. Tile budget `MAX_TILES = 250_000`; keep generators cheap.
5. Keep `cargo check --workspace` warning-free.
6. **Census deltas re-derived against main at dispatch time**
   (prepass snapshot: built-ins 141 → 142; memberships 155 → 156;
   quasiperiodic 12 → 13).
7. **No repo-wide `cargo fmt`** (convention: `cargo fmt` defaults
   only, no rustfmt/clippy config, no pre-commit hooks).
8. **Orchestrator census guards report verbatim, never fix:** the
   `lib.rs` guard tests (`registry_groups_are_complete_and_canonically_ordered`,
   `family_docs_match_registrations`, `readme_catalog_matches_registry`)
   are orchestrator-owned; a
   red guard goes into the report unchanged, and `LatticeTiling`
   culling-test lines are flagged for the orchestrator, not added.
9. No M number in commits, docs, or the report; the orchestrator
   assigns it at integration.

### Standard tests

The 3-test trio every tiling ships (deterministic generation, census
consistency, render smoke) — copy the shape from
`golden_rhomboid.rs`'s test module — plus the battery above as
in-module tests.

### Deliverable (worker report)

Per the fleet protocol in `tessellix/AGENTS.md`:

1. Diff-scope statement (exactly the owned files).
2. Verification evidence with exact numbers and pass counts
   (battery §1–§7 above; programmatic only — a visual look is
   recon, never evidence).
3. Expected census deltas re-derived against current main
   (prepass snapshot: family 12 → 13, built-ins 141 → 142,
   memberships 155 → 156; `LatticeTiling`? → remind the
   orchestrator about the culling line).
4. Proposed commit-message body (basis, cluster contents,
   verification).

---

## Appendix — recon scratch index (`/tmp/opencode/wis12-recon/`)

| file | role |
|---|---|
| `NOTES.md` | append log s00–s18e (every step, dead ends, bug fixes) |
| `s09_cert.py` | exact ℤ[√3] certification module (`certify`, `boundary`, `CORN`, `AREA_SH2`, `rot4`, …) |
| `s08_panels.py` / `s08_panels.json` | rule.png raster panel fit (anchors, k, per-child m + krot + iou) |
| `s07_verts.json` | 47,163 deduped vertices (snap grid source) |
| `tiles.json` | 37,635 parsed patch tiles (28,783 clean-indexable) |
| `s18_compgrow.py` + `s18_compgrow_{tri,rho,sq}-sup.json` | composition-growth solver + results (150/13/0 certified) |
| `s18b_analyze.py` + `s18_tri-sup.json` | translation-canonical congruence grouping + s08 cross-check |
| `s18d_iou.py` + `s18d_{tri,rho}-sup.json` | raster-IoU discriminator (oracle + no-match numbers) |
| `s18_rule_candidates.json` | the two candidate child tables above (machine-readable) |
