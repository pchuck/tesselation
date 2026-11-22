# Brief: period-tripling-2d — marginal supersubstitution (λ = 3, 2 label classes)

> **Category correction vs dispatch note:** the dispatch's guard
> expectation "Nonperiodic tab 7 → 8" repeats exactly the error the M141
> errata recorded: all nine existing aperiodic-family members declare
> `TilingCategory::Aperiodic` (verified in source, 2026-11-22 —
> `domino.rs`, `half_hex.rs`, `hat.rs`, `jr_aperiodic.rs`,
> `period_doubling.rs`, `spectre.rs`, `taylor_socolar.rs`,
> `thue_morse.rs`, `turtle.rs`). The new member lands in the
> **Aperiodic tab (10 → 11)**; Nonperiodic stays 7. If your evidence
> flips this, STOP and report first.

**Family:** `aperiodic` · **Candidate:** PROPOSED.md §23d "Period
doubling / period tripling / Thue–Morse 2D / Squiral — marginal
supersubstitutions; effort S–M each" (**this brief covers ONLY period
tripling 2D** — period doubling ✅ M141, Thue–Morse ✅ M143, Squiral
stays parked) · **Effort:** S–M · **Precedent to mirror:**
`families/aperiodic/period_doubling.rs` (M141 — the closest sibling:
square block substitution, exact Z², limit-periodic, full-block period
manifest; read it ALL before writing code — the `RULES` tuple table,
the closed-form test, the `patch_period` doc comment, the
depth-quantization view rule, the emission recentering, and the test
trio) and `families/aperiodic/domino.rs` (M138 — the `cover_box` doc
comment and the in-tile-offset degeneracy note).

### Goal

Land the 2D period-tripling substitution tiling under registry id
`period-tripling` in the `aperiodic` family, classified like
period-doubling (same group tags, category `Aperiodic`, method
substitution, aperiodic/translational status "limit-periodic") **unless
your battery says otherwise** — evidence decides; report deviations.
Expected headline property: limit-periodic via approximating periodic
tilings with a measured **(3^g, 3^g) axis-aligned period manifest — the
ratio-3 ("period-TRIPPLING") signature** that distinguishes it from
period-doubling's ratio-1 (2^g, 2^g) and domino's ratio-2 (2^g, 2^(g+1))
— PLUS a true **period-1 direction**: the depth-g period lattice is
exactly ⟨(1, −1), (3^g, 0)⟩, so the (1, −1) direction is 1-periodic at
EVERY depth (the signature of the marginal second eigenvalue −λ of the
substitution matrix; no other member of §23d has a period-1 direction).

### Literature basis (rule pinned in recon, 2026-11-22)

- **Source:** the Bielefeld Tilings Encyclopedia entry "Period Tripling
  2D", <https://tilings.math.uni-bielefeld.de/substitution/period-tripling-2D/>
  (slug `period-tripling-2d`; the image paths use uppercase "2D"). Entry
  header: **2 prototiles**. The Info section is **EMPTY** (no tags, no
  references, no text — verified by parsing the page HTML) — the rule
  image is the only source, and there is NO 1D "period-tripling" entry on
  the site (index checked: only period-tripling-2d, period-doubling,
  thue-morse, thue-morse-2d).
- **Literature cross-check: INCONCLUSIVE.** arXiv API queries (2026-11-22):
  "supersubstitution" (any position): **0 hits** (the term does not appear
  on arXiv at all); "period tripling" + substitution: 0;
  "marginal supersubstitution": 0; "triad substitution": 0; "3-adic" +
  substitution + tiling: 0; Baake + Grimm + marginal + substitution: 1
  hit, unrelated; Baake + "supersubstitution": 0. The websearch backend
  was down all recon session. The entry itself cites nothing. → **The pin
  rests entirely on the image evidence below; no source name is citable —
  pin `source: None` in the classification arm.**
- **How the rule was pinned (evidence chain, all programmatic):**
  1. `rule.png` (200×224) decode: 20 cells = a left column of two single
     cells (gray top, blue bottom) beside a right column of two 3×3 grids
     (top grid: blue on the main diagonal; bottom grid: all gray). Row
     pairing is by vertical centering; **the image contains no arrow
     pixels** — it shows each parent beside its inflated frame.
     ⇒ gray A → 3×3 with blue B on the main diagonal (6 A + 3 B);
     blue B → 3×3 all A.
  2. `patch.png` (3000×3000 = an 80×80 window, 1580 blue cells)
     window-matched against the simulated A-root tiling: the **depth-4
     (period-81) tiling matches the 80×80 window at exactly 81 offsets,
     and they are precisely the main diagonal (k,k), k = 0..80** (image
     coordinates) — exactly the predicted period lattice {Δx ≡ Δy mod 81}.
     The depth-5 (period-243) tiling matches the same 80×80 window at
     exactly 486 offsets, precisely {(ox, oy) : ox − oy ≡ 81 or 162
     (mod 243)} (measured airtight: labels depend only on c − r, so a
     sweep over d = ox − oy over all 243 residues characterizes the full
     243×243 offset space; all 729 in-lattice offsets then brute-forced
     2D). These are the two cosets of the depth-4 lattice on which the
     depth-5 cell at the window's diagonal is A (v₃ = 4); the d ≡ 0 coset
     — the patch's own B diagonal, v₃(0) := 5 — matches at 0 offsets
     because the displayed window carries the A-line (even-depth)
     diagonal. This is direct evidence for the marginal two-orbit limit
     structure. The color-swapped reading: 0 matching offsets —
     excluded. The window's own main diagonal is 0 blue / 80 gray —
     consistent with the even-depth diagonal-parity law (re-verified
     from the decoded grid).
  3. **Residual ambiguity (documented, immaterial):** the image carries
     no arrows/axes, so which image edge is +y is underdetermined — the
     two readings are related by a vertical mirror. Every depth-g patch
     is self-TRANSPOSE (the label depends on (c − r) mod 3^g through an
     EVEN function — verified), so the x−y vs x+y diagonal convention is
     immaterial to the rule content, the matrix, or the period structure.
     Pinned convention: **array row 0 = top of the world block**
     (reproduces the picture as drawn; see the world closed form below).
- **The pinned rule** (array frame, row 0 = top of the parent's inflated
  3×3 block, k = 3·r + c order; A = gray (136,136,136), B = blue
  (0,75,170) — the two classes the entry header calls "2 prototiles", on
  ONE geometric unit-square prototile):

  ```text
  parent A (class 0):
     (B,0,0) (A,1,0) (A,2,0)
     (A,0,1) (B,1,1) (A,2,1)
     (A,0,2) (A,1,2) (B,2,2)

  parent B (class 1):  (A, c, r) for all nine frame cells
  ```

- **Eigenstructure (recon spike, exact 2-permutation determinants,
  permutation count asserted — M135 lesson):** S (child × parent, order
  A, B) = [[6, 9], [3, 0]]; column sums 9, 9 = λ² (λ = 3); char poly
  det(xI − S) = x² − 6x − 27 = (x − 9)(x + 3); **PF eigenvalue 9 = λ²;
  second eigenvalue −3 = −λ — the MARGINAL case** (|λ₂| = λ). Pin values
  of det(xI − S) at x = 0..9: −27, −32, −35, −36, −35, −32, −27, −20,
  −11, 0.
- **Closed form (world coordinates — the Rust test anchor).** With the
  depth-d patch recentered (below), the cell (x, y) with |x|, |y| ≤
  (3^d − 1)/2 is **B iff v₃((x + y) mod 3^d) is odd, with v₃(0) := d**
  (v₃ = the 3-adic valuation); everything else is A. Equivalently in
  array coordinates (c, r), row 0 at top: B iff v₃((c − r) mod 3^g) is
  odd, v₃(0) := g. Verified on every cell, g = 1..5 (9 … 59049 cells).
- **Period structure (measured, never asserted):**
  - axis-aligned minimal period (full-block filter, M141 `patch_period`
    style), g = 1..5: **(3,3), (9,9), (27,27), (81,81), (243,243)** —
    strictly 3× growing, NO proper within-patch period at any depth;
  - full period lattice by exhaustive 2D census, g = 1..4 (1D reduction
    at g = 5): exactly **{(a,b) mod 3^g : a + b ≡ 0 (mod 3^g)}, size
    3^g**, generated by **(1, −1) and (3^g, 0)** — the (1, −1) direction
    is period-1 at ALL depths (a true period of the limit tiling), the
    transverse axis-aligned period is 3^g exactly (no proper 1D period
    < 3^g at any g = 1..5);
  - **defect-line parity (the marginal ambiguity):** the line x + y = 0
    (= the array main diagonal) is all B at odd g, all A at even g,
    g = 1..5. Limit: exactly **two limit tilings up to translation** —
    the line x + y = 0 carries A (even-depth subsequence) or B
    (odd-depth subsequence); everywhere off the line the label is B iff
    v₃(x + y) is odd (x + y ≠ 0).
- **1D shadow (the "period-tripling" 1D rule — 3-adic cousin of
  period-doubling):** the label depends only on s = x + y, and the
  transverse sequence is the fixed point of the centered morphism
  **σ: A → ABA, B → AAA** (matrix [[2,3],[1,0]], column sums 3 = λ;
  char poly x² − 2x − 3 = (x−3)(x+1), eigenvalues 3, −1; pin values at
  x = 0..5: −3, −4, −3, 0, 5, 12). The 2D rule is NOT a σ⊗σ product
  (that needs 4 label classes; there are only 2 here) — it is the 1D
  sequence pulled back along x + y (1-periodic in the (1, −1)
  direction).
- **Exact arithmetic:** pure Z² end to end (integer cell tuples), floats
  only at the emission boundary — never decimal literals in exact
  geometry (M141 pattern).

### Files you own

- `crates/tess-tilings/src/families/aperiodic/period_tripling.rs` —
  the new module.
- `crates/tess-tilings/src/families/aperiodic.rs` — module decl +
  register slice + classification arm + census `IDS` + the four
  family-level tests (mirror the `period_doubling_tests` block
  verbatim in shape: `covers_region` on a ±8 × ±6 box,
  `covers_large_and_distant_views`, `rotated_scaled`,
  `deterministic`, via `crate::tests::{check_coverage, check_det,
  check_rotated}`). (Not a lattice table → **no** `lattice_tilings()`
  change.)
- `docs/families/aperiodic.md` — catalog row + heading count
  ("## Implemented (9 tilings)" → "(10 tilings)").
- `/tmp/opencode/period-tripling-recon/` — scratch (append-only
  NOTES.md already holds the full evidence chain + spike outputs).

**Do not touch:** `lib.rs`, README, CHANGELOG, INDEX, ROADMAP,
PROPOSED.md, sibling aperiodic modules (`period_doubling.rs` /
`domino.rs` / `thue_morse.rs` are read-only references — copy patterns
from them), other families, UI crates, AGENTS.md.

### Commit exception (this brief only)

One worktree commit, subject like
`aperiodic-family: period-tripling 2D marginal supersubstitution` —
NO M numbers, NO pushes. The orchestrator cherry-picks and assigns the
M number at integration.

### Constraints (the ones that bite here)

1. Exact Z² end to end; `MAX_TILES = 250_000`; **the depth rule
   degenerates to a constant d = 5**: 9^5 = 59049 ≤ MAX_TILES <
   9^6 = 531441, so `D_PRESENT = MAX_DEPTH = 5` (SINGLE band). Still
   write the standard zoom + clamp form with **log3 (not log2)** —
   zoom = 0 for E ≥ 243√2 (= √118098) else ceil(log3(√118098 / E)),
   d = clamp(zoom, 5, 5) — so the pattern matches M141. Nested-view
   bit identity holds trivially within the single band; pin it in the
   test anyway. **Far-view trap:** the covered domain is
   |x|, |y| ≤ 121, so M141's far-view coordinate (100, 100) is INSIDE
   our domain and would NOT be background — use a view at
   (150, 150)–(150.5, 150.5) (150 ≥ 122 ⇒ outside).
2. Emission mapping (pin it exactly): array cell (c, r), row 0 = top,
   of the 3^d × 3^d block, sh = (3^d − 1)/2 = 121 at d = 5, emits the
   unit square [x0, x0+1) × [y0, y0+1) with **x0 = c − sh, y0 = sh − r**
   (row 0 at the TOP edge; covered domain [−121, 122)²). Seed = class A
   (the A-root patch is the one the Bielefeld patch image matches).
   The pin test asserts `RULES` == the depth-2 closed form (fixed-point
   pin, recon-verified on 81/729/6561 block checks) — NOT a σ⊗σ
   derivation (the rule is not a product).
3. `cargo check --workspace` zero warnings; `cargo fmt` owned hunks
   only (repo is NOT fmt-clean — revert non-owned byproducts);
   `cargo test --workspace` green except the two orchestrator guards
   (`registry_groups…`, `readme_catalog…`) — their verbatim failure
   output is your delta evidence.
4. You branch from a base with **aperiodic family 9 / built-ins 134 /
   memberships 148 / Aperiodic tab 10 / Nonperiodic tab 7**
   (2026-11-22), but other landings may move these totals while you
   work — **never hard-code totals from your base**; report exactly
   what your guards printed and the orchestrator re-derives deltas at
   integration.
5. The 2 classes are LABELS on identical unit squares (seeds 0..1):
   say "2 prototile label classes, one square prototile" in the doc row
   so the census claim matches the geometry (the Bielefeld header's
   "2 prototiles" = 2 label classes). Lattice field "—", group "—"
   (no Bravais lattice, no space group — P1), like the domino /
   period-doubling rows. `SymmetryMeta::new(WallpaperGroup::P1,
   LatticeKind::None).with_notes(...)` per M141; `periodic: None` in
   the `TilingResult` (M141 sets it None even for the periodic
   approximant — mirror that).
6. Category `Aperiodic` (see errata block at top); classification arm:
   method "substitution", prototiles Some(1), aperiodic_status
   "limit-periodic", translational "limit-periodic", **source: None**
   (the entry has no reference — nothing to cite; if a guard or the
   orchestrator wants a string, use Some("Bielefeld Tilings
   Encyclopedia") and report the substitution), year: None.
7. Registry id `period-tripling` (no collision — verified against
   crates/, docs/, README, INDEX on 2026-11-22; PROPOSED.md mentions
   the slug in prose only).

### Battery spec (already run in recon — the numbers below are MEASURED,
re-verify them in the Rust tests; every value is a pin)

Recon ran a stdlib Python spike in the foreground (≤240 s per
experiment, all checks passed; outputs in
`/tmp/opencode/period-tripling-recon/NOTES.md`): substitution matrix
+ column sums; char poly by exact 2-permutation determinant (count
asserted); depth censuses g = 1..5 vs S^g·e; closed form on every cell
g = 1..5; axis-aligned + full-lattice period census; 1D reduction at
g = 5; 0-overlap + exactly-1 coverage on a 15-px/cell fine grid with
irrational offsets (0.43841, 0.44880) g = 1..5; the 1D shadow
morphism on every anti-diagonal g = 1..5; the patch.png 81-offset
match. The Rust test list (mirror M141's structure):

1. `rule_line` — for each parent the 9 children are the 9 distinct 3×3
   frame cells (containment + disjointness + equal area ⇒ closure).
2. `closed_form_pin` — RULES is the fixed point of the closed form:
   for g = 1..4 the depth-(g+1) block of every cell equals
   RULES[its depth-g label]; plus the world closed form
   B ⟺ v₃((x+y) mod 243) odd (v₃(0) := 5) on the full d = 5 patch.
3. `substitution_matrix_and_counts` — S derived from RULES ==
   [[6,9],[3,0]] (child × parent); column sums 9; S^g·e pinned at
   g = 1..5: from A (A,B) = (6,3), (63,18), (540,189), (4941,1620),
   (44226,14823); from B (9,0), (54,27), (567,162), (4860,1701),
   (44469,14580); totals 9^g = 9, 81, 729, 6561, 59049; patch census
   agrees at every level.
4. `char_poly` — det(xI − S) by exact i64 2-permutation expansion
   (assert the permutation count = 2 was visited — M135 lesson) at
   x = 0..9: −27, −32, −35, −36, −35, −32, −27, −20, −11, 0; assert
   the closed form x² − 6x − 27 and the roots 9 and −3 (PF 9 = λ²;
   marginal second eigenvalue −3 = −λ).
5. `no_overlap_and_coverage_depths` — pinned tile counts 9/81/729/
   6561/59049 at g = 1..5; owner grid: every frame cell owned by
   exactly one tile.
6. `period_hierarchy` — full-block-filtered minimal axis-aligned
   period (px, py) = (3^g, 3^g) at g = 1..5, strictly 3× growing per
   level (the "period tripling" manifest vs M141's 2× ratio-1 and
   domino's ratio-2); plus the full period-lattice census at g = 1..4:
   exactly {(a,b): a + b ≡ 0 mod 3^g}, size 3^g, no other periods
   (document the unit-square offset degeneracy in the `patch_period`
   doc comment, M141 style — the overlap-only probe without the
   p | L filter must not be used); plus a 1D-reduction check that the
   transverse 1D period is exactly 3^5 (no proper one).
7. `defect_line_parity` — the main diagonal (array c = r; world
   x + y = 0) is all B at odd g, all A at even g, g = 1..5: the
   marginal limit ambiguity (two limit tilings up to translation).
8. `nested_view_bit_identical` — five windows in the single d = 5
   band (d = 5 at EVERY view size — the band has no extent cutoff, so
   every window passes), true containments B5 ⊂ B1, B1 ⊂ B2, B4 ⊂ B2
   (reuse M141's window set verbatim — its E² values 0.18–0.405 are
   in-band trivially); every tile of a smaller view reappears
   bit-identically (proto, seed, per-vertex f64 bits) in each larger
   one containing it.
9. `default_view_fills_pane` — (a) probe ±10 (E² = 800, in band) → d = 5,
   mean tile side 1.0, `preview_window(mean side, 4/3)` = ±6 × ±4.5
   (E² = 225, in band) non-empty with BOTH label classes (recon
   measured the window census A = 84, B = 36 over its 120 integer cells)
   and a second probe round that is a fixed point; (b) CLI pane
   ±(40/3) × ±10 (E² ≈ 1111, in band) non-empty.
10. `far_view_is_background` — view (150, 150)–(150.5, 150.5):
    outside the covered domain (150 ≥ 122), empty, no refusal message.
11. Family block in `aperiodic.rs` — the four tests mirroring
    `period_doubling_tests`: `period_tripling_covers_region` on the
    ±8 × ±6 box; `period_tripling_covers_large_and_distant_views`
    (M141's three far windows (14.5, 14.5), (12, 24), (24, 48) are all
    inside our d = 5 domain |x|, |y| ≤ 121 — reuse verbatim);
    `period_tripling_rotated_scaled` (scale 0.6, 25°, ±3 box);
    `period_tripling_deterministic`.

### Catalog row (draft for `docs/families/aperiodic.md`)

| `period-tripling` | 2 prototile label classes, one square prototile | Bielefeld entry "period-tripling-2D" — Info section EMPTY (no tags, no references); rule reconstructed from the rule image and pinned by an 81-offset window match of the patch image against the simulated depth-4 tiling (exactly the predicted period lattice); literature cross-check inconclusive (no arXiv hits for "supersubstitution" or "period tripling"; no source citable): one unit-square prototile carrying 2 label classes (A gray, B blue — the header's "2 prototiles"), λ = 3 substitution into 9 children (parent A → 3×3 with B on the main diagonal, 6 A + 3 B; parent B → 3×3 all A; substitution matrix S = [[6,9],[3,0]] (child × parent), column sums = 9 = λ², PF eigenvalue 9 = λ², char poly x²−6x−27 = (x−9)(x+3) — second eigenvalue −3 = −λ, the **marginal** case, pinned by exact 2-permutation determinant); finite covered patch = the 3^d × 3^d block recentered on the origin, d = 5 (9^5 = 59049 ≤ MAX_TILES < 9^6 — single depth band), nested views bit-identical; exact Z² integer-cell arithmetic end to end, floats only at emission; limit-periodic via **approximating periodic tilings** — the depth-g patch's period lattice is exactly ⟨(1,−1), (3^g, 0)⟩ (measured g = 1..5, size 3^g, no other periods): the (1,−1) direction is period-1 at all depths, the axis-aligned period is (3^g, 3^g) = (3,3), (9,9), (27,27), (81,81), (243,243), strictly 3× growing per level ("period tripling" — vs period-doubling's ratio-1 (2^g, 2^g) and domino's ratio-2 (2^g, 2^(g+1))); the limit tiling is 1-periodic along (1,−1) and 3-adic limit-periodic across it (1D shadow σ: A → ABA, B → AAA — the 3-adic cousin of period-doubling's A → AB, B → AA); the marginal second eigenvalue leaves exactly one ambiguous line: x+y = 0 carries B at odd depth / A at even depth — two limit tilings up to translation; aperiodic family — no Bravais lattice, no space group (lattice "—", group "—") |

### Deliverable (worker report)

1. Diff-scope statement (exactly the owned files, nothing else).
2. The pinned construction: entry URL, the 2×9 rule table, the closed
   form, substitution matrix, char poly, PF eigenvalue, the marginal
   second eigenvalue, the period-lattice law, the 1D shadow morphism.
3. Battery numbers with exact values; Rust test list with pass counts;
   the measured period manifest g = 1..5 (axis-aligned + full lattice +
   defect-line parity).
4. Expected census deltas as printed by your guards (base totals +
   your +1s): aperiodic family 9 → 10; Aperiodic tab 10 → 11
   (Nonperiodic 7 → 7, UNCHANGED — if your evidence flips the
   category, STOP first and report); built-ins +1; memberships +1.
   Not a lattice table → no `lattice_tilings()` change, no culling
   lines.
5. Proposed commit-message body (construction, verification, period
   manifest). NO M numbers.
