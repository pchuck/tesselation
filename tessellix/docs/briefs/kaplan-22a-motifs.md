# Brief: kaplan-22a-motifs — the design-element motif library

**Family:** `islamic` (hankin engine lives there) · **Candidate:**
PROPOSED.md §22a (Kaplan dissertation Ch. 3, §3.5 extraction — the
dissertation full text is in-repo at
`.scratch/salvage-20261120/tmp/wallpaper_groups/kaplan_full.txt`,
keyed by PDF page; §3.5 is printed pp. ~60–75) · **Effort:** M–L ·
**Tier:** P1 · **Status note:** 22c (rosette transform) landed M99;
22b (najm) landed M99 — 22a is the remaining core Ch.-3 item before
the 22d/22e wave.

### Goal

Replace the hand-built per-tiling motif configs (the ten thin
`hankin_*.rs` modules each pin fixed ray angles) with a
**parameterized design-element library** and a **per-tile motif
assignment** API, per Kaplan diss. §3.5:

- **Star (n, r, s, θ):** path = one ray of length 2r from the east
  edge midpoint M; `{n/d}s` reparameterizes as θ = πd/n; heuristic
  s = 2 when n > 6 and θ > 2π/n, else s = 1. Replication:
  reflect/rotate the path by the polygon's symmetry group, keep the
  first s subpaths at each line of reflection (diss. Fig. 3.10).
- **Rosette (n, r, s, θ, h, φ):** star + one hexagon per concavity;
  path M→G→(ray). Lee's ideal: C on OA with AC = AM, G = MM′ ∩
  (line through C ∥ OM); natural height α = GH/AM; ideal rosette =
  (n, r, s, π/n, 1, 0). Most large rosettes s = 3 (central star's
  s + 1).
- **Extension operator** (wraps an inner motif one layer out):
  θ′ = θ/2 and r′ = r·[1/cos(π/n) − sinθ·tan(π/n)·cos(π/n − θ)];
  child gets (n, r′, s−1, θ′), rotated π/n. Extended rosettes are
  the historically prominent ones.
- **Assignment:** per-tile motif choice (star / rosette / extended
  rosette / inferred-fallback) on any substrate whose tiles are
  regular polygons — generalizing M94's fixed configs and M99's
  single-transform.

The M94 engine (`hankin.rs`, 544 ln) already provides: contact angle
θ, the δ two-point split with the lean-backward rule (diss. Fig. 3.5),
greedy min-cost ray pairing (diss. §3.4; the naive alternative has
(2n−1)!! pairings), face/interlace extraction. `rosette_transform.rs`
(M99) and `najm.rs` (M99) are the freshest consumers — mirror their
idioms. The existing thin modules (`hankin_4_8_8.rs`,
`hankin_3_4_6_4.rs`, `hankin_hex.rs`, …) are the regression baseline:
their emitted geometry must stay byte-identical.

### Deliverables

1. **Library module** `families/islamic/hankin_motifs.rs`: the three
   motif constructors + the assignment API, exact-arithmetic where the
   substrate is exact (uniform tilings: the H = √3/2 idiom; najm
   substrates: whatever exact field najm.rs ships). Signature design
   is yours — but it must let a per-substrate config express
   "tile at lattice site X gets motif Y" without hand-writing ray
   paths.
2. **Showcase registry instances (2–3)** as new thin modules on the
   assignment API — suggest, in increasing ambition:
   a. `hankin-4-8-8-motifs`: 4.8.8 with a rosette on the octagons +
      extended rosette layer (the M99 substrate, new motif content —
      do NOT modify the existing M99 entry);
   b. a hexagonal substrate (`hankin_hex` geometry) with mixed
      star/rosette assignment showing the (n, r, s, θ) parameter
      sweep;
   c. a najm substrate (`najm-63-2e-0-3e-ac`, p6m) with ideal
      rosettes on the 12-gons — the diss.'s flagship look.
   Each instance: catalog row + per-tiling doc + the standard battery.
3. **Regression proof:** a test (or harness output capture) that the
   existing hankin_* entries emit identical geometry before/after the
   engine change — if the library requires touching `hankin.rs`,
   the configs' outputs must not move.

### Verification battery

- Per motif: exact d_n symmetry (reflect/rotate the emitted path by
  2π/n and by a mirror — vertex sets must coincide); ray-endpoint
  census (each ray hits its tile edge midpoint or pinned contact
  point); the ideal rosette pins (n, r, s, π/n, 1, 0) reproduces
  Lee's construction (compare against diss. Fig. 3.13 proportions
  numerically); extension-operator formula verified symbolically at
  n = 5, 6, 8, 10, 12.
- Per instance: the 3-test trio + coverage battery (engine's own
  checks); ray-pairing cost reported; interlace face census
  (faces closed, edges shared exactly 2).
- Regression: existing hankin entries unchanged (byte-identical
  table output).
- `cargo check --workspace` zero warnings; full suite green except
  the two orchestrator census guards (verbatim capture).

## Files you own

- `crates/tess-tilings/src/families/islamic/hankin_motifs.rs` (new)
  + the 2–3 new instance modules (named per the ids above or your
  evidence-backed choice).
- `crates/tess-tilings/src/families/islamic/hankin.rs` — ONLY if the
  assignment API needs engine hooks; every change must keep the
  existing configs' outputs identical (regression proof required).
- `crates/tess-tilings/src/families/islamic.rs` — wiring for the new
  instances only (hand-applied hunks; NEVER rustfmt).
- `docs/families/islamic.md` — catalog rows + heading bumps +
  Per-tiling-documents link lines (one per instance).
- `docs/tilings/<id>.md` — one per-tiling document per instance.
- Scratch `/tmp/opencode/kaplan-22a/` (append-only NOTES.md),
  checkpoint to repo-root `.scratch/kaplan-22a/` at the end.

**Do not touch:** `rosette_transform.rs`, `najm.rs`, the existing
`hankin_*.rs` instance modules (read-only baselines — copy idioms),
`girih_*.rs`, `lib.rs`, README, CHANGELOG, INDEX, ROADMAP, PROPOSED,
other families, UI crates, AGENTS.md.

## Protocol

1. Read PROPOSED.md §22a + the diss. text (§3.5 pages in
   `kaplan_full.txt`) before designing; spike the motif constructors
   in Python (stdlib, ≤240 s/experiment) — pin the ideal rosette and
   the extension operator numerically first.
2. One worktree commit per landing if you stage the work (library
   first, then instances), or a single commit if it lands whole;
   subjects `islamic-family: hankin motif library (22a)` /
   `islamic-family: hankin motif instances` — NO M numbers, NO
   pushes; long messages via `git commit -F <file>`.
3. Census deltas: +1 per instance (islamic family 31 → …; Islamic
   tab; built-ins; memberships — re-derive against YOUR base's pins
   and report what you saw; capture the two guard failures verbatim).
4. Report: diff scope; the API shape (one paragraph); motif battery
   numbers (exact); instance list with ids + tile counts; regression
   proof statement; Rust pass counts + verbatim guard failures;
   proposed commit body; scratch checkpoint path.