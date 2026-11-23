# Brief: CAP — self-similar hat substitution (Cut-and-Project tiling), implementation dispatch

**Family:** aperiodic · **Candidate:** PROPOSED.md §23h "CAP (self-similar
hat substitution)" · **Effort:** M (unchanged; this prepass removed the
geometry unknowns) · **Precedent to mirror:**
`crates/tess-tilings/src/families/aperiodic/hat.rs` (rule engine),
`crates/tess-tilings/src/families/aperiodic/spectre.rs` (exact surd-ring
substitution), `docs/briefs/wis-12-fold.md` (prepass format)

### Status and scope

Research-only prepass, **completed and certified** (spike clean, exit 0,
19 programmatic OK assertions). This document is **docs-only**: `crates/`
is untouched in this commit. Everything marked **PINNED** below was
verified by exact arithmetic in the number field K = ℚ(√5, √−3) (stdlib
`Fraction` spike, no floats in any certified claim) or by programmatic
extraction from the paper's own SVG figures; measured float numbers are
labeled *measured* and are recon, never evidence. Implementation is
**unblocked**: no STOP-clause item remains.

### Goal

Land the **CAP tiling** (Baake–Gähler–Sadun's cut-and-project tiling,
arXiv:2305.05639) in the aperiodic family: 4 prototiles (the deformed
metatiles T, H, P, F at the deformation fixed point — **not hats**), 6
orientations each (`r^k`, k = 0..5, r = rotation by +60°; **no
reflections**), inflation factor exactly **φ² linear / φ⁴ per-count**
with **fixed prototile shapes at every level** (no outline drift), MLD
to a φ²-rescaling (LIDS), forced by the same 29-rule / 22-kept patch
combinatorics our `hat` module already implements. Registry id: `cap`.
Suggested tier: `worker` (escalate per protocol on failure).

### Literature basis

- **[BGS23]** Baake, Gähler, Sadun, "Dynamics and topology of the Hat
  family of tilings" — arXiv:2305.05639. The CAP tiling is constructed
  and analyzed there as the self-similar element of the 44-dimensional
  hat deformation family (abstract; pure-point dynamical spectrum, MLD
  to the hat tiling via the deformation). Full text saved at
  `~/.local/share/opencode/tool-output/tool_2085a57e1001je7t0t43chXv4p`
  (ar5iv render). Key locations (all line-verified): abstract (44-dim
  family); μ-table eq. (12) lines 955–958; CAP shape-vector identity
  line 1020 + cohomologous form / σ̃₁ / LIDS lines 1023–1030; return
  module t₀ line 1062; Fig 2 caption line 481 + Lemma 7 and proof
  lines 492–514 (the "consistent pseudo-inflation with a complete
  'belt' of positive thickness" quote is line 509); Fig 3 caption line
  525 (fractiles); Fig 4 caption lines 533–541 (mark semantics; "the
  marker is always on the left side of the edge" is line 537); Fig 5
  caption line 549 (edge reference orientations, L non-directional);
  vertex relations lines ~560–600 (A₊=r⁵X₋ etc.); Fig 6 caption line
  997 (control points iφξ, −iφξ, iφ, iφ); frequency vector f ~lines
  1337–1345; areas / density / return module ~lines 1355–1370.
- **Corroboration:** Baake, Gähler, Mazáč, Mitchell, "Diffraction of
  the Hat and Spectre tilings and some of their relatives" —
  arXiv:2502.03268, J. Math. Phys. 66 (2025) 092707 — treats CAP as
  the self-similar model-set representative in the hat tiling's
  topological conjugacy class and computes its (pure-point)
  diffraction.
- **Relationship to our hat module:** [BGS23] "as in [17]" where [17] is
  the hat paper (arXiv:2303.10798) — the edge-mark system is the *same*
  one `hat.rs` implements (certified below, 25/25 marks). The
  substitution *combinatorics* (29 rules, 22 kept, boundary edge-class
  cycles) is the *same* one `hat.rs` encodes (certified below).
- **What was re-verified from the source before this brief:** all four
  prototile shapes (exact closure in K from the paper's Fig 6 SVG), all
  four supertile outlines (exact φ²-scaled copies, Fig 2 SVG), the
  inflation-matrix census (Fig 2 shaded tiles = `hat.rs` KEPT×RULES
  columns), the count/frequency algebra (exact), the control-point
  chirality (exact), and the mark table (exact, 25/25). Nothing is
  taken on trust from the figures' visual appearance.

### Frame convention (PINNED — get this wrong and everything mirrors)

SVG figure frames are y-down. Define the **algebra frame** by
`canon(d) = conj(d)·e^{+iπ/6}` for a display offset `d` read as a complex
`(dx + i·dy_svg)`; inverse `d_display = conj(canon)·e^{+iπ/6}`. Proof
this is the paper's frame, not its mirror: the Fig 6 control-point dots
vs caption pins (T: `iφξ`, H: `−iφξ`, P: `iφ`, F: `iφ`, stated in the
y-up *figure* frame = algebra frame rotated by −30°) match **direct**
for all four tiles (max err 7.9e-5 units; conjugate hypothesis off by
≥ 1.6). In the algebra frame the control offsets are
**T: −φ, H: +φ, P: +φξ², F: +φξ²** (all in ℤ[ξ,φ]).

Exact arithmetic: K = ℚ(√5, √−3) as pairs of ℚ(√5) pairs
`((a,b),(c,d))` meaning `(a + b√5) + (c + d√5)·√−3`, with
ξ = (1+√−3)/2, φ = (1+√5)/2; multiplication
`(a₁a₂ − 3b₁b₂, a₁b₂ + a₂b₁)` per ℚ(√5) slot pair. Edge vectors live in
ℤ[ξ,φ] = {a + bξ : a,b ∈ ℤ[φ]}; **placements** live in the return module
R = t₀·ℤ[ξ,φ] with t₀ = 3φ+2−ξ = φ²(1+ξ)(φ−ξ). Areas are β-parts × √3,
i.e. live in ℚ(√5).

### Pinned facts

**Shape vector (PINNED, exact; [BGS23] line 1020, μ-vectors eq. (12)
lines 955–958).** Metatile edge
classes A, B, X, F, L with reference vectors (algebra frame; A/B/X/L at
60° multiples, F at +22.2388°):

| class | vector | length |
|---|---|---|
| A | 3φ | 4.854102 |
| B | 3φ | 4.854102 |
| X | 1 | 1 |
| F | φ+ξ | φ√2 = 2.288246 (arg +22.2388°) |
| L | 2φ | 3.236068 |

Base directions (orientation k = 0) in the algebra frame: A, B, X, L
point along the **positive real axis** (A and B are the same vector —
they differ only as mark-carrying classes); F = φ+ξ points at
**+22.2388°**. An edge of class c and orientation k is the vector
`V[c]·ξ^k` with ξ = e^{iπ/3}; k ∈ {0..5}, no conjugates.

Verified as the μ-combination μ₁+(φ+ξ)μ₂+2φμ₃ with μ₁ = (1+ξ⁵,1+ξ⁵,1,0,0),
μ₂ = (ξ+ξ²,ξ+ξ²,0,1,0), μ₃ = (1+ξ⁵,1+ξ⁵,0,0,1) — exact in K. (Cohomologous
to (φ+1)μ₁+(φ+ξ)μ₂; (φ+1, φ+ξ) is the left eigenvector of
σ̃₁ = [[2+ξ⁵,2],[ξ,ξ]] with eigenvalue φ² — re-proven exact.)

**Prototiles (PINNED, exact closure in K; extracted from Fig 6
`tiles.svg` at u = 9.921379 px/unit, then certified).** Anticlockwise
(class, k) edge sequences in reference orientation:

```
T: (A,1)(A,3)(B,5)                                    [3 edges]
H: (A,5)(X,5)(X,0)(B,1)(X,1)(X,2)(B,3)(X,3)(X,4)      [9 edges]
P: (A,1)(L,3)(X,3)(X,4)(B,4)(L,0)(X,0)(X,1)           [8 edges]
F: (B,4)(L,0)(X,0)(X,1)(L,1)(X,1)(F,2)(F,3)(X,4)      [9 edges]
```

Key structural fact: **the deformed shapes have genuine 0° turns** —
consecutive same-orientation edges are exactly collinear (e.g. H's
A(ξ⁵),X(ξ⁵) pair merges into one straight run of length 3φ+1). This is
impossible for the un-deformed hat metatiles and is the visible signature
of the fixed point.

**Areas (PINNED, exact; = [BGS23] published values).** In units a = √3/4:
T = 9+9φ, H = 15+27φ, P = 14+22φ, F = 15+23φ (equivalently 2·Area/√3 =
27/4+9/4√5, 57/4+27/4√5, 25/2+11/2√5, 53/4+23/4√5).

**Supertile outlines (PINNED, exact; Fig 2 `infp0.svg`).** All four
thick outlines are **exactly φ²-scaled copies of the prototypes, rotation
ξ⁰** — closure exact in K per outline, per-edge displacements exact. The
substitution is genuinely self-similar at outline level; `hat.rs`'s
per-level outline recomputation is gone.

**Inflation matrix / count law (PINNED, exact).** M (rows children
H,T,P,F × cols parents H,T,P,F) = the `hat.rs` KEPT×RULES type counts:

```
M = [[3,1,2,2],
     [1,0,0,0],
     [3,0,1,1],
     [3,0,2,3]]
```

- M·f = φ⁴·f exactly for the published frequency vector
  f = (1/3, 5/3−φ, 2φ−3, 2−φ).
- char poly p(x) = x⁴ − 7x³ + 7x − 1; p(φ⁴) = 0 exactly in ℚ(√5);
  PF(M) = φ⁴ = 3φ+2 = 6.854101966.
- Column sums (= KEPT_LEN, children per parent): [10, 1, 5, 6]; total
  kept per 4-parent generation = 22.
- Count recurrences (children of one H): 10 → 64 → 442 (M, M², M³
  column-H sums; ratios 6.40, 6.90625 → φ⁴ = 6.854102).
- **Figure 2 census (measured, programmatic):** the 22 shaded children
  in Fig 2, assigned to parents by grid-sample overlap, give per-parent
  (H,T,P,F) counts T→(1,0,0,0), H→(3,1,3,3), P→(2,0,1,2), F→(2,0,1,3)
  = M's columns **exactly**.

**Pseudo-inflation, not stone inflation (PINNED; [BGS23] Lemma 7 proof).**
Per-parent exact area deviations (Σᵢ M[i][j]·Areaᵢ − φ⁴·Areaⱼ, units a):

| parent | deviation (exact) | relative |
|---|---|---|
| H | +(105/2 + 45/2·√5)·a | +25.56% |
| T | −(105/2 + 45/2·√5)·a | −63.66% |
| P | −(35 + 15·√5)·a | −20.16% |
| F | −(35/2 + 15/2·√5)·a | −9.58% |

The kept children do **not** tile the φ²-scaled parent outline exactly;
the rule is a *consistent pseudo-inflation with a belt of positive
thickness* (paper's words). Measured belt quantification (float
grid-sample, recon only): straddling children per parent T/H/P/F =
0/6/2/3; inside-fractions of straddlers 0.39–0.74 (P-children-of-H
≈ 0.62, F-children ≈ 0.39–0.74); fully-inside children measure 1.000.
**Implementation consequence: never clip or contain children by parent
outlines — placement is rule-driven (edge matching), exactly as in
`hat.rs`, and belt children legitimately straddle.** The *fractiles*
(paper Fig 3: iterated outlines rescaled back) do have an exact stone
inflation, MLD to the CAP tiling — noted for possible follow-up, not
this dispatch.

**Mark system (PINNED, exact; Fig 4 `tiles2.svg` vs `hat.rs` BOUNDARY).**
All 25 edge markers extracted programmatically (16 X/F barb strokes + 9
A/B markers; L edges unmarked, asserted) match `hat.rs`'s BOUNDARY signs
**25/25** under: (a) class cycles identical to the prototype edge
sequences above under rotations T:0, H:2, P:5, F:1 (H's figure walk is
the reversed winding; T/P/F are not); (b) the sign map
`sign_hatrs = −(marker on y-up-left of the hat.rs-order traversal)`.
Paper caption (pinned): *"In the positive direction, the marker is always
on the left side of the edge"*; L non-directional (r³L = −L). So
`hat.rs`'s boundary traversal runs along the paper's **negative** edge
directions — one uniform global flip, no per-edge exceptions. The
paper's vertex relations are consistent with this (A₊=r⁵X₋, A₋=r³X₋,
B₊=X₋, B₋=r²X₋, F₊=X₊, L₊=X₋).

**Three substitutions that must not be conflated** (all three live in
our tree):

1. `hat.rs` — H/T/P/F metatile substitution **at hat level**: φ² only in
   the limit, outlines drift per level, emits hats (non-edge-to-edge,
   565 midpoint T-junctions at depth 3).
2. `turtle.rs`/`spectre.rs` — 9-label Spectre+Mystic substitution,
   4+√15 area inflation, per-level chirality flip.
3. **CAP (this dispatch)** — metatile-only 4-prototile substitution on
   the deformed fixed-point shapes: exactly φ² linear / φ⁴ count per
   level, **fixed shapes**, pseudo-inflation with belt. Edge classes
   A,B,X,F,L match like-for-like; in the certified Fig 2 depth-1 patch
   every shared boundary carries the **same (class,k) decomposition on
   both sides** (per-raw-edge exactness walk over all 22 tiles),
   including the collinear 0°-turn runs (straight-through vertices).
   Do not assume global edge-to-edge-ness at all levels — assert it
   per assembled level in the battery (item 5).

### Errata vs `hat.rs` / `docs/families/aperiodic.md` (drift claim)

`aperiodic.md` hat row and build arc say the inflation is "φ² only in
the limit (iterated outlines drift by design)". That is **true for the
hat-level hierarchy our hat module runs** and is exactly what CAP
removes: CAP is the fixed point of the same deformation family — the
(φ+1, φ+ξ) eigenvector of σ̃₁ freezes the shapes, supertile outlines
become *exactly* φ²-self-similar (measured, exact in K), and drift
disappears by construction. No error in the hat rows; add one sentence
to the hat build arc at integration ("the self-similar fixed-point
version of this hierarchy is the CAP tiling, see `cap`") — that row
edit belongs to the implementation dispatch, not this docs commit.

### Generation recipe (for the implementer)

1. Exact ring: implement ℤ[ξ,φ] pairs-of-ℚ(√5)-pairs arithmetic (spike
   `spike2.py` lines 20–64 is the reference implementation — port it
   verbatim; `kf`/`kstr` for debug floats). Edge vector of class c,
   orientation k: `V[c] · ξ^k` with the V table above.
2. Prototypes: the four (class,k) sequences above → vertex lists by
   cumulative exact addition; assert exact closure (spike Part A).
3. Substitution machinery: transplant `hat.rs`'s RULES (29 rules,
   `Seed`/`Edge`/`Gap`), KEPT, and BOUNDARY **verbatim** — the
   combinatorics is certified identical (class cycles + marks 25/25).
   Replace Eisenstein placement arithmetic with ℤ[ξ,φ]; delete the
   per-level outline-recomputation drift logic; outlines are fixed
   φ²-scaled prototype copies (rotation ξ⁰ at every level, verified).
4. Emission: deformed metatiles H/T/P/F at depth g inside the view
   window (finite covered patch, `hat.rs`/`half-hex` precedent); no
   reflections; orientations `r^k` only. Control points (T: −φ, H: +φ,
   P: +φξ², F: +φξ² relative to the pinned black-dot corner) available
   as debug overlay if wanted — optional.
5. Keep `hat.rs` untouched; `cap.rs` is a sibling module in the same
   family.

### Verification battery (what "done" means)

Every claim programmatic, exact unless labeled *measured*; each battery
number below was **measured in the prepass spike** (`spike2.out`) and
must be re-derived, not copied:

1. **Prototype closure:** each prototype's edge-displacement sum ≡ 0
   exactly in K; edge lengths = V table to float noise < 1e-3 (spike:
   spread ≤ 1.1e-4 px over all 29 edges).
2. **Shape-vector identity:** μ₁+(φ+ξ)μ₂+2φμ₃ = (3φ, 3φ, 1, φ+ξ, 2φ)
   exact; (φ+1, φ+ξ)·σ̃₁ = φ²·(φ+1, φ+ξ) exact.
3. **Area identities:** per-prototype 2·Area/√3 = the four ℚ(√5) pairs
   above, exact; area/a = published vector, exact.
4. **Count law:** M·f = φ⁴·f exact; p(φ⁴) = 0 exact; per-parent kept
   counts = M columns; depth-1/2/3 child counts of one H = 10/64/442.
5. **Assembly:** depth-1 supertile per parent from RULES: closed
   boundary, every interior metatile-edge shared by exactly 2 children,
   boundary edge-class cycle = φ²-scaled prototype cycle (exact);
   per-raw-edge placement walk exact (spike tolerance 0.2 px at
   u₂ = 8.503920 px/unit; all 22 Fig 2 placements certified).
6. **Pseudo-inflation deviations:** the four exact deviation table
   values above reproduced from assembled supertiles; *measured*
   straddle fractions in the 0.39–0.74 band for the 11 belt children,
   1.000 for the 11 interior children (grid-sample, float).
7. **Aperiodicity smoke:** depth-g patch carries no translation period
   (probe grid up to the patch diagonal; CAP is aperiodic per [BGS23] —
   the FLC/LIDS statement is cited, not re-proven).
8. **Mark honoring:** every shared edge in every assembled patch has
   matching `hat.rs`-convention marks (0 violations), mirroring hat.rs's
   label check.

### Files you own (implementation dispatch)

- `crates/tess-tilings/src/families/aperiodic/cap.rs` — the new module.
- `crates/tess-tilings/src/families/aperiodic.rs` — module decl +
  `register` slice + `classification` arm + census `IDS`.
- `docs/families/aperiodic.md` — catalog row (guard-checked) + the one
  hat-row cross-reference sentence (see Errata).
- `/tmp/opencode/cap/` — scratch (port of the prepass spike, batteries).
- Salvage checkpoint at the end: `.scratch/cap/` (git-ignored via
  `.git/info/exclude`).

**Do not touch anything else** — not `lib.rs` (census pin, guards,
culling test), not README, not `docs/INDEX.md`, not
`docs/history/CHANGELOG.md` (M number written by the orchestrator), not
`hat.rs`/`turtle.rs`/`spectre.rs`, not the UI crates.

### Constraints (from AGENTS.md)

1. Exact geometry: ℤ[ξ,φ] pair-of-ℚ(√5)-pairs arithmetic end to end —
   never decimal literals; floats only for view/window math and
   debug prints.
2. Verify numerically BEFORE writing Rust: Python spike, **stdlib
   only**, foreground, per-step output, ≤ 240 s per experiment.
3. Emission: when several geometric tiles of one class share a vertex,
   emit each geometric tile exactly once (AGENTS.md step 3).
4. Tile budget `MAX_TILES = 250_000`; keep generators cheap (φ⁴ ≈ 6.85
   count growth per level — depth ~7 exhausts the budget; pick depth by
   window like `hat.rs` does).
5. Keep `cargo check --workspace` warning-free.
6. **Census deltas must be re-derived against main at dispatch time**
   (stale-base warning: this prepass snapshot was taken at `166ba10`
   built-ins 146 → 147, memberships 160 → 161, aperiodic 10 → 11 —
   if main has moved, re-count; the +1 shape of the delta is what
   matters, the absolute numbers above may be stale).
7. No repo-wide `cargo fmt`.
8. Orchestrator census guards report verbatim, never fix.
9. No M number in commits, docs, or the report.

### Standard tests

The 3-test trio every tiling ships (deterministic generation, census
consistency, render smoke) — copy the shape from `hat.rs`'s test module
— plus the battery above as in-module tests.

### Deliverable (worker report)

Per the fleet protocol in `tessellix/AGENTS.md`:

1. Diff-scope statement (exactly the owned files).
2. Verification evidence with exact numbers and pass counts (battery
   §1–§8; programmatic only — a visual look is recon, never evidence).
3. Expected census deltas re-derived against current main (see §6
   stale-base warning).
4. Proposed commit-message body (shape vector, rule transplant, belt
   deviation table, verification).

---

## Appendix — prepass scratch index (`/tmp/opencode/cap-recon/`)

| file | role |
|---|---|
| `NOTES.md` | session log: source resolution, all stages, gotchas |
| `spike2.py` | master spike — exact K arithmetic, prototype extraction (A), algebra (A2), Fig 2 outlines + shaded census + per-raw-edge walk (B), control-point chirality pin (C), belt stats (C2), Fig 4 mark extraction vs hat.rs (D); runs clean end-to-end, 19 OK assertions |
| `spike2.out` | full measured output of the final clean run |
| `fig4_marks.py` | standalone Part D prototype (superseded by spike2 Part D) |
| `spike_cap.py` | first-pass spike (superseded by spike2.py) |
| `arxiv_hat.xml` | saved arXiv listing/metadata for the hat paper |
| `assets/tiles.svg, tiles2.svg, edges.svg, infp0.svg, frac0.svg, patch.svg` | the paper's Fig 6/4/5/2/3/7 vector assets |
| `cap_paper.html` | ar5iv render of arXiv:2305.05639 (scratch top level, not assets/) |

Saved primary source:
`~/.local/share/opencode/tool-output/tool_2085a57e1001je7t0t43chXv4p`
(full text of arXiv:2305.05639, with the line-pinned sections listed
under Literature basis).