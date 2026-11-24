# Brief: girih-decagon-patch — the classic two-level girih decagon region

**Family:** `islamic` · **Candidate:** ROADMAP "Full girih decagon
patch — the classic decagon-and-bowtie strapwork region with
hexagon/bowtie/rhombus infill (Lu–Steinhardt figures), as a
documentary overlay on the substitution entry" · **Effort:** M ·
**Style:** patch-style data-table entry (M93 `jeandel-rao` 100×100
table with geometric edge bands is the battery precedent; M149 GLB
plane presentation is the presentation precedent).

### Goal

Ship the classic two-level girih figure as a finite patch registry
entry `girih-decagon-patch`: a region of the Darb-i Imam large-scale
decagon-and-bowtie network (the level-1 structure of
`girih-quasiperiodic`, LS07 Science 2007), with each large tile
infilled by its small girih tiles — the "documentary overlay" — and,
as the stretch goal, the strapwork bands that make the figure read as
girih ornament.

**The content is already certified in-repo — this brief assembles and
presents it; it does not re-derive the substitution.**

- `families/islamic/girih_qc.rs` (2768 ln, M~50s girih-quasiperiodic):
  the Darb-i Imam 3-prototile substitution (decagon, bobbin, bow-tie;
  IF = 2φ³, exact Q(2cos 18°), edge-marker seam rule, frozen core).
  Its tables contain the level-1 infill of the large decagon and
  bow-tie — extract/read them, do not modify them.
- `families/islamic/girih_{bowtie,decagon,pentagon,rhombus}.rs` +
  `girih.rs` (M131–M134 + shesh-band): the five primitive shapes as
  exact unit-edge tables on the 36° frame — the small-tile geometry
  vocabulary (copy shapes; the small tiles of the infill must be
  congruent to these).
- M93 battery: sample coverage exactly once (irrational-offset grid),
  interior edges shared exactly 2 (6-digit keys), closed-or-boundary
  wedges, shoelace vs table area < 1e-15.

### Region and presentation

1. **Primary deliverable (tiles-only):** a finite patch table whose
   default view contains one full large decagon plus its adjacent
   large bow-ties (choose the minimal such region; make the bbox
   configurable like the other patch entries), each large tile
   replaced by its small-tile infill from the girih_qc tables.
   Contract-legal by construction if the infills are exact covers —
   certify: per-large-tile area law (sum of small areas = large area,
   exact over the family field), edge parity across the whole patch
   (interior edges exactly 2, T-junction-free), coverage battery.
2. **Stretch goal (strapwork bands):** the classic ornament reads
   through the STRAPS — thin quads running along small-tile edges at
   a fixed offset (M93 edge-band encoding, width ~0.08–0.1 unit; also
   `khatam_girih.rs`/`hexagram.rs` strap precedents). If you ship
   bands, the patch must still partition exactly: straps + infill
   polygons tile the region with no gaps/overlaps, and straps must be
   CONTINUOUS across small-tile edges (each strap line enters and
   leaves every tile at matched positions — the girih property; pin
   the offset once and verify continuity on every strap in the
   region). If the band partition proves messy, ship tiles-only and
   report the blocker — do not ship an overlapping overlay.

### Sources / documentation

- LS07: Lu & Steinhardt, Science 315 (2007) — the two-level girih
  figures (documentary reference for the LOOK; the geometry authority
  is girih_qc's own certified tables).
- `docs/tilings/girih-quasiperiodic.md` + the islamic family doc's
  girih arcs — cross-link them.
- Per-tiling doc `docs/tilings/girih-decagon-patch.md` (H1 = registry
  `name()`, e.g. "Girih Decagon Patch (Darb-i Imam, two-level)";
  template shape: `docs/tilings/andritz-bowtie-hexagon-10gon-3.md`).

## Files you own

- `crates/tess-tilings/src/families/islamic/girih_decagon_patch.rs`
  (new; a data-table `Tiling` — not a `LatticeTiling`, no
  `lattice_tilings()` change).
- `crates/tess-tilings/src/families/islamic.rs` — module decl +
  register slice + classification arm + census IDS (hand-applied
  hunks; NEVER rustfmt this file). Category: `TilingCategory::Islamic`
  (a documentary patch of an islamic pattern — same tab as
  `girih-star-cross`; if the evidence says otherwise, report).
- `docs/families/islamic.md` — catalog row + heading bump +
  Per-tiling-documents link line.
- `docs/tilings/girih-decagon-patch.md` — the per-tiling document.
- Scratch `/tmp/opencode/girih-decagon-patch/` (append-only NOTES.md),
  checkpoint to repo-root `.scratch/girih-decagon-patch/` at the end.

**Do not touch:** `girih_qc.rs` and all M131–M134 modules (read-only
sources — copy from them), `lib.rs`, README, CHANGELOG, INDEX,
ROADMAP, PROPOSED.md, other families, UI crates, AGENTS.md.

## Protocol

1. Spike first (Python stdlib, ≤240 s/experiment): extract the
   large-decagon and large-bow-tie infills from girih_qc's tables
   (read the Rust const tables; do not re-run the generator's search),
   re-assemble them on the exact family geometry, and run the M93
   battery on the assembled patch BEFORE writing Rust.
2. Rust: exact radicals via the family constants; floats only at the
   emission boundary; `MAX_TILES = 250_000` respected (the default
   view must render comfortably; report the tile count).
3. Standard 3-test trio (`girih_decagon_patch_covers_region`,
   `_rotated_scaled`, `_deterministic`) + the battery as in-module
   tests.
4. `cargo check --workspace` zero warnings; `cargo test --workspace`
   green except the two orchestrator census guards (verbatim capture
   = delta evidence). Expected deltas: islamic family 30 → 31;
   Islamic tab 25 → 26; built-ins 153 → 154; memberships 167 → 168
   (re-verify against your base and report what you saw).
5. One worktree commit, subject `islamic-family: girih decagon patch
   (two-level documentary overlay)` — NO M number, NO push; long
   message via `git commit -F <file>`.
6. Report: diff scope; the assembled region spec (which large tiles,
   tile counts, strap decision); battery numbers (exact); Rust pass
   counts + verbatim guard failures; proposed commit body; scratch
   checkpoint path.