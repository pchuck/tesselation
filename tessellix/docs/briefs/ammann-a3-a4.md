# Brief: ammann-a3-a4 — the last two GS87 Ammann sets (A4, then A3)

> **RE-PIN (2026-11-24, post-STOP — orchestrator):** landing 1's
> certification proved the brief's original A4 pins (λ = 2, integer
> 13/31) wrong; re-pinned per the worker's exact-arithmetic evidence
> (`a4-cert.py` ALL PASS in Z[√2]): **λ = 1+√2, M = [[3,4],[2,3]]**
> (tr 6, det 1, Perron root 3+2√2 = λ²), **Q(√2) arithmetic** via the
> `ammann_beenker` (m,n) half-integer idiom (floats only at emission)
> — the 13/31 figures are the 1-px projection of Q(√2) geometry at
> u = 13 px. P1 = (2+√2)u square − two opposite u×u notches (both
> chiralities); P2 = (1+√2)u × (2+√2)u rect − one 1×√2 notch
> (half-chair, A2/A1 = 1/√2 exactly); row 1 = {3×P1, 4×P2}, row 2 =
> {2×P1, 3×P2}; 12 explicit child polygons per the worker's STOP
> report §5; TE "With Decoration" ornaments NOT shipped. Area laws
> close exactly: (1+√2)²A1 = 3A1+4A2, (1+√2)²A2 = 2A1+3A2. Brief text
> below that contradicts this block is superseded. A3 (landing 2)
> unchanged — the rule-figure pin stands; the TE patch's density
> mismatch (hand-drawn free tiling, not a region of the rule figure)
> is documented in the tiling doc, not a stop.
>
> **Status: ✅ A4 (landing 1) LANDED 2026-11-25 — M173, commit e3fc839**
> (integrated onto post-M172 main after the fleet outage; cherry-pick
> clean, census re-derived at integration: Quasiperiodic 16 -> 17,
> built-ins 164 -> 165, memberships 185 -> 186). A3 (landing 2)
> remains dispatched-pending.

**Family:** `quasiperiodic` (A5 = Ammann–Beenker precedent, M27/M129;
A2 = golden bee M104) · **Candidate:** PROPOSED.md §23b "Ammann A3 and
Ammann A4 — the missing rungs of the A1–A4 request (A2 ✅ golden bee,
A5 ✅ ammann-beenker; A1 parked: no substitution structure)"; effort M
each · **Effort:** M total (two landings, A4 first) · **Sources:**
Tilings Encyclopedia entries `ammann-a4` and `ammann-a3`
(https://tilings.math.uni-bielefeld.de/substitution/ammann-a4/ and
…/ammann-a3/), both citing **[GS87] Grünbaum & Shephard, Tilings and
Patterns, W.H. Freeman 1987** (rules published as figures there; the
TE rule/patch GIFs are the machine-readable figures).

## Certified structure (orchestrator recon 2026-11-24, .scratch/ammann-a34-recon/)

Recon toolkit + source images + extracted corner tables are
checkpointed at repo-root `.scratch/ammann-a34-recon/`
(`extract2.py`, `famtrace.py`, `clean.py`, the four TE GIFs, 1x PPMs).
Copy them into your scratch as the starting point; re-download from TE
if you prefer fresh bytes (URLs above; identical md5 expected).

Both sets are **rectilinear** (all edges axis-aligned, rotations 0/90/
180/270 only) — figure extraction is deterministic, no float fitting.

### A4 (landing 1 — 2 prototiles)

TE tags: Canonical Substitution Tiling, Finite Rotations,
**MLD Class Ammann** (mld to Ammann–Beenker — cite, don't prove),
Parallelogram Tiles, Polytopal Windowed Tiling, **With Decoration**.

- **P1** (filled chair, crisp 1x extraction @ bbox 44×44): corner
  cycle (local, y down) `(0,13),(13,13),(13,0),(44,0),(44,31),
  (31,31),(31,44),(0,44)` — a 44×44 square minus two opposite
  13×13 corner notches; edges {13,13,31,31,13,13,31,31}, area
  44²−2·13² = **1598** (raw pixel area 1585 — ±1px GIF noise; treat
  the symmetric 13/31 form as the exact hypothesis).
- **λ = 2 candidate for P1**: the row-1 cluster shows 4 same-size
  filled chairs (4 × 1598 = 4·1598 ✓ trivially — the classic chair
  substitution; the certification that matters is placements +
  patch overlay).
- **P2** (the second prototile) is drawn **unfilled** (outline) in
  the rule figure, bbox ≈ 33×46 — geometry NOT yet pinned. The
  figure's thin purple segments (lengths 44/77 at 1x) are **Ammann
  line decoration**, not tiles — do not ship them as geometry, but
  DO use them where they disambiguate child placements. OPEN
  QUESTION you must resolve: P2's exact polygon + its child
  multiset (row-2 cluster shows 2 filled chairs + 2 outlined
  shapes; whether the outlined shapes are P2 instances or
  decoration overlays decides the count matrix and λ). Candidates:
  λ=2 with children {2×P1, 2×P2} (forces area(P2)=1598,
  contradicts bbox 33×46 → likely false), or decoration-overlay
  interpretation (children {2×P1} + decoration → λ² = 2·1598/A2),
  or a third reading via patch overlay.
- Resolution path: **patch overlay** — the TE patch
  (`ammann-a4-patch.gif`, 1295×1298 at 1x) shows the substitution at
  many levels; fit your reconstructed clusters against occurrences
  (WIS raster self-overlay method, exact integer offsets).

### A3 (landing 2 — 3 prototiles)

TE tags: Euclidean Windowed Tiling, Finite Rotations,
Without Decoration, Self-Similar Substitution. TE: "The substitution
of this one uses the golden ratio as inflation factor" — **λ = φ**
(NOT Q(√2); PROPOSED.md's §23b "8-fold Q(√2) family" placement is
corrected by this entry — A3 is a φ-based set; the cut-and-project
window is uncomputed on TE → FLC scope note per the girih-qc
convention).

- **P1** (dark-blue chair, bbox 52×31): local cycle
  `(0,12),(20,12),(20,0),(52,0),(52,31),(0,31)`; edges
  {20,12,32,31,52,19}; area = 52·31 − 20·12 = **1372**. Edge
  structure is φ-paired: 20·φ≈32, 19·φ≈31, 52≈20·φ² — fit exact
  φ-rational lengths (unit u ≈ 12.1px at 1x; find the exact
  integer/u formulation and state it).
- **P2** (gold Z-chair, bbox 65×51): local cycle
  `(0,20),(33,20),(33,0),(52,0),(52,33),(65,33),(65,51),(0,51)`;
  edges {33,20,19,33,13,18,65,31}; area = 65·51 − 33·20 − 13·33 =
  **2226**.
- **P3** (gray Z-chair, bbox 52×52): local cycle
  `(0,20),(20,20),(20,0),(39,0),(39,33),(52,33),(52,52),(7,52),
  (7,31),(0,31)`; edges {20,20,19,33,13,19,45,21,7,11} — the 7/11
  steps may be real or GIF artifacts; resolve by patch overlay.
  Raw area 1728.
- **Clusters** (children drawn at prototile scale inside the φ×
  inflated parent): P1 → {P2, P1} (2 children); P2 → {P1, P3, P1,
  P1} (4 children); P3 → {P1, P3, P1} (3 children, possibly a 4th —
  raw pixel areas give row-3 sum 4444 vs φ²·1728 = 4529, an 85px²
  gap the exact-cover decomposition must close: either a missing
  4th child or corrected vertex tables).
- Area-law recon check (≈1e-3, pixel noise): P1: 2237+1352 = 3589
  vs φ²·1372 = 3592 ✓; P2: 1339+1404+1707+1404 = 5854 vs φ²·2226 =
  5829 ✓. Your exact tables must make these EXACT.

### Names / ids

TE slugs: `ammann-a4`, `ammann-a3`. Suggested registry `name()`:
"Ammann A4 (quasiperiodic)" / "Ammann A3 (quasiperiodic)" (Andritz
precedent). Per-tiling docs `docs/tilings/ammann-a4.md`,
`docs/tilings/ammann-a3.md` (H1 = name() exactly).

## Files you own

- `crates/tess-tilings/src/families/quasiperiodic/ammann_a4.rs`,
  `…/ammann_a3.rs` — one module per landing.
- `crates/tess-tilings/src/families/quasiperiodic.rs` — module decls,
  register slice, classification arms, census IDS (hand-applied
  hunks; NEVER rustfmt this file).
- `docs/families/quasiperiodic.md` — catalog rows + heading bumps +
  Per-tiling-documents link lines.
- `docs/tilings/ammann-a4.md`, `docs/tilings/ammann-a3.md` — the
  per-tiling documents (H1 = registry name() exactly; template shape:
  copy `docs/tilings/andritz-bowtie-hexagon-10gon-3.md`).
- Scratch `/tmp/opencode/ammann-a34/` (append-only `NOTES.md`), and a
  repo-root `.scratch/ammann-a34/` checkpoint at each landing.

**Do not touch:** `lib.rs`, README, CHANGELOG, INDEX, ROADMAP,
PROPOSED.md, other families/modules, the UI crates, AGENTS.md. The
`ammann_beenker*.rs`, `golden_bee.rs`, `golden_*.rs` modules are
read-only references (copy constant idioms from them).

## Protocol

1. **Certify before Rust.** Per landing: pin the exact vertex tables
   and cluster placements in Python (stdlib only, ≤240 s per
   experiment) using the recon toolkit; verify the area law EXACTLY
   in the fitted arithmetic; verify a level-2 substitution patch is
   edge-to-edge (interior edges shared exactly 2; 6-digit keys;
   irrational-offset coverage sample); overlay-fit against the TE
   patch image. If the figure contradicts the pinned structure
   (e.g. the A4 P2 open question resolves to nothing consistent),
   STOP and report instead of improvising.
2. Landing 1 = A4, landing 2 = A3. One worktree commit per landing,
   subjects `quasiperiodic-family: ammann-a4 substitution` /
   `quasiperiodic-family: ammann-a3 substitution`. NO M numbers, NO
   pushes. Long messages via `git commit -F <file>`.
3. Rust: exact radicals via the family's constant idioms — for A4 the
   arithmetic is integer (13/31 lattice; λ=2), for A3 it is
   φ-rational (reuse `golden_bee.rs` / `golden_triangle.rs` Q(√5)
   idioms; H-style constants as in the girih tables). Floats only at
   the emission boundary; never decimal literals in geometry.
4. Standard 3-test trio per tiling
   (`ammann_a4_covers_region`, `_rotated_scaled`, `_deterministic`)
   + the battery as in-module tests. MAX_TILES budget respected;
   generators cheap enough for per-frame GUI regeneration.
5. `cargo check --workspace` zero warnings; `cargo test --workspace`
   green on your base except the two orchestrator census guards —
   capture their verbatim failure output as delta evidence. Expected
   deltas per landing: quasiperiodic family 16 → 17 (A4) → 18 (A3);
   Quasiperiodic tab 21 → 22 → 23; built-ins 153 → 154 → 155;
   memberships 167 → 168 → 169 (single-category members; re-verify
   against your base's pins and report what you saw). Not lattice
   tilings → no culling lines.
6. FLC scope note per tiling (girih-qc convention): substitution
   mechanics verified; model-set/diffraction properties cited (A4:
   mld Ammann–Beenker per TE; A3: cut-and-project with window
   uncomputed per TE), not proven.
7. Report per landing (the usual): diff scope; pinned rule (exact
   vertex tables + placements + count matrix + λ); battery numbers
   with exact programmatic figures; Rust pass counts + verbatim
   guard failures; proposed commit body. Closing report: arc state +
   scratch checkpoint path.