# Brief: CAP — true cut-and-project re-derivation (post-STOP), frontier dispatch

**Family:** `aperiodic` · **Registry id:** `cap` · **Effort:** L (novel
derivation — design doc first) · **Tier:** frontier · **Predecessor
brief:** [cap-hat-substitution.md](cap-hat-substitution.md) (geometry
prepass — its PINNED facts survive; its *construction model* is dead)

> **RE-DERIVATION MANDATE (2026-11-24, post-STOP — orchestrator,
> decision A of the CAP fork):** the predecessor brief's implementation
> model — transplant `hat.rs`'s tree expansion onto the CAP substitution
> — is **proven impossible** by the fast-tier worker's exact probes
> (`.scratch/cap/`: depth-6 two-sided coverage fails by 1263×; 348.4
> units of H–H overlap over 11 pairs in the H-core, invariant to any
> `t_P`/`t_F` choice; the pile-up is intrinsic to fixed-shape children
> at fixed offsets under a φ⁴-per-count count law). This matches
> [BGS23] Lemma 7: CAP's substitution is a **pseudo-inflation, not a
> stone inflation** — children do not tile the parent outline. The task
> is now: **emit the true CAP tiling from the paper's own construction**
> (the Lemma 7 consistent-pseudo-inflation limit / the return-module
> structure — your derivation, documented), NOT from the raw tree
> expansion. Do not reuse the dead model.

## Goal

Land the **CAP tiling** (Baake–Gähler–Sadun, "Dynamics and topology of
the Hat family of tilings", arXiv:2305.05639) in the aperiodic family as
a **genuine non-overlapping tiling**: the standard trio's two-sided
`check_coverage` (`|covered − want| ≤ 1e-6·want`) MUST pass — no
relaxed tests. 4 prototiles (deformed metatiles T, H, P, F at the
deformation fixed point — not hats), 6 orientations each (`r^k`,
k = 0..5, no reflections), fixed prototile shapes at every level,
inflation φ² linear / φ⁴ per-count, MLD to a φ²-rescaling (LIDS).

## Sources

- **[BGS23] full text:** was saved at
  `~/.local/share/opencode/tool-output/tool_2085a57e1001je7t0t43chXv4p`
  (ar5iv render) — if wiped, re-fetch
  `https://arxiv.org/abs/2305.05639` (ar5iv HTML renders best).
  Line-verified key locations from the predecessor prepass: μ-table
  eq. (12) lines 955–958; CAP shape-vector identity line 1020 +
  cohomologous form / σ̃₁ / LIDS lines 1023–1030; **return module t₀
  line 1062**; Fig 2 caption line 481 + **Lemma 7 and proof lines
  492–514** (the consistent-pseudo-inflation construction — the core of
  your derivation).
- **Worker scratch (read-only evidence):** `.scratch/cap/` —
  `cap_sim.py` (the pinned 29-rule/22-kept patch + expansion sim),
  `probe_h_isolation.py` / `probe_density.py` / `search_tf.py` + verbatim
  outputs (the STOP evidence), exact `t_H`/`t_T` tables.
- **Predecessor brief PINNED facts that survive:** shape vector
  (Q(√5,√−3) exact), areas (a = √3/4 units, = published values), the
  29-rule / 22-kept patch, orientations, the φ²/φ⁴ inflation, the K =
  ℚ(√5, √−3) Fraction-spike arithmetic style.

## Deliverables (in order)

1. **Design doc** `docs/cap-cut-project-design.md` FIRST — the
   derivation: what construction you emit (Lemma 7's consistent
   pseudo-inflation limit carried with the belt/decoration structure, a
   return-module/window emission, or another reading the paper supports
   — justify against the text), why it cannot pile up, and the
   termination/budget story for query-driven emission. Stop-and-report
   if the paper contradicts a pin — no improvisation.
2. **Python spike** (stdlib Fraction, K = ℚ(√5,√−3) exact; ≤240 s per
   experiment): construct the tiling exactly; verify non-overlap and
   two-sided coverage on real regions BEFORE Rust. Battery: pairwise
   overlap census = 0; two-sided coverage on ≥3 regions; orientation
   census (6 per prototile, no reflections); area law (φ⁴ per count,
   φ² linear); fixed-shape invariance across levels.
3. **Rust:** `families/aperiodic/cap.rs` (query-driven generator —
   emission shape may mirror `hat.rs`'s cascade/cull plumbing; the
   MODEL may not) + hand-applied wiring hunks on
   `families/aperiodic.rs` (register slice, classification arm, census
   `IDS`, family census pin; NEVER rustfmt `families/aperiodic.rs` or
   `families/mod.rs`) + catalog row + per-tiling doc
   `docs/tilings/cap.md` + family-doc row/link + standard trio +
   in-module mirror of the spike battery. Exact arithmetic per the
   `spectre.rs` surd-ring precedent; floats only at emission.
4. **Census capture:** aperiodic family 10 → 11, A tab 11 → 12,
   built-ins 159 → 160, memberships 175 → 176 on YOUR base — capture
   the guard failures verbatim; the orchestrator re-derives vs main at
   integration.

## Ownership (exclusive)

`families/aperiodic/cap.rs` + new per-tiling doc + scratch
(`/tmp/opencode/cap2/` checkpointed to `.scratch/cap2/`); hand-applied
hunks on `families/aperiodic.rs`; `docs/families/aperiodic.md` rows;
this brief + design doc. Read-only: `hat.rs`, `spectre.rs`,
`.scratch/cap/`. Do NOT touch: other families, `lib.rs`, README,
CHANGELOG, INDEX, ROADMAP, PROPOSED, UI crates (the orchestrator owns
the two census guards — capture verbatim).

## Protocol

One worktree commit when it lands (subject
`aperiodic-family: CAP tiling (true cut-and-project construction)`),
NO M number, NO pushes, long messages via `git commit -F`. Design doc
+ spike before ANY Rust. Every count/closure claim programmatic.
`cargo check --workspace` zero warnings; full suite green on your base
except the two orchestrator census guards (verbatim capture). Before
any edit: `pwd` + `git log -1 --format="%h %s"` — verify the
cap-cut-project-impl worktree; report your base's HEAD in the final
report.
---
**Status: ✅ LANDED as M166 (2026-11-24)** — true cut-and-project construction
landed in `families/aperiodic/cap.rs` (exact integer abcd arithmetic; D-map
deformation of the hat metatile lineage; shape vector a φ² left eigenvector
of σ̃₁ ⇒ LIDS). 11 cap tests + census deltas integrated by the orchestrator.
