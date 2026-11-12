# Code Review: tessellix

**Date:** 2026-11-07
**HEAD reviewed:** `ab70a17` ("M18: Girih star-and-cross tiling"), clean working tree
**Scope:** all of `tess-core`, `tess-tilings`, `tess-tui`, `tess-gui` (19 source files, ~8,000 lines including tests), manifests, README, AGENTS.md, and git history (29 commits).
**Method:** full read of every source file; empirical verification of build/test/CLI claims.

> **Fix status — all findings addressed on 2026-11-07.** See the
> [Fixes applied](#fixes-applied) section at the bottom for what changed
> per finding and how each was verified. Suite is now **97/97 green**,
> `cargo check` **and** `cargo clippy --workspace --all-targets` both at
> **zero warnings** (clippy was not part of the repo bar before — it now
> is, modulo nothing). Pre-existing `cargo fmt` drift elsewhere in the
> repo (compact data tables, multi-item enums) was deliberately left
> untouched so the diff stays reviewable.

## Verified claims

| Claim | Result |
|---|---|
| `cargo check --workspace` zero warnings (repo convention) | ✅ 0 warnings |
| `cargo test --workspace` all green | ✅ 94/94 pass (7.4 s) |
| Feature gating (`--no-default-features`) compiles | ✅ (1 cosmetic warning: unused `mut` in `registry()` when no features are on) |
| CLI `list` / `export` work | ✅ smoke-tested, including edge cases |
| Adding a tiling requires no UI changes | ✅ registry-driven; confirmed by reading both frontends |
| No `unsafe` anywhere | ✅ confirmed by grep |

## Verdict

High-quality, unusually disciplined codebase. Zero `unsafe`, dependency-free core, thin UI layers that genuinely never need touching when adding a tiling. The standout is **test rigor for geometric code**: substitution rules proven by area plus thousands of random point samples, the pinwheel intersection rule and T-junction census, Hirschhorn patch invariants (Euler χ = 1, exact C6 symmetry, edge ownership ≤ 2, 20° frame quantization), and a full edge/vertex census of the embedded girih patch. Git history shows this process catching real bugs (OOM on zoom-out, GUI concave-fill crash, TUI idle spin, Ctrl-C swallow), with fixes regressed into tests (`all_tiles_triangulate`, `penrose_cache_valid_for_all_tiles`).

The issues found below are mostly latent fragilities and documentation staleness, not active defects.

## Findings — Medium

### M1. README and AGENTS.md are stale

README says "13 built-in tilings" and lists 13; the registry now registers **19** (verified via `tess-tui list`): missing snub-square, star-cross, cairo-pentagonal, cairo-collinear, girih-shesh-band, girih-quasiperiodic. README's "currently 66 tests" → actually 94. AGENTS.md claims "3.3.4.3.4 = snub square — not in this repo" (added in M14) and "~30 s, 66 tests".

### M2. Lattice culling bounds are correct only incidentally (`tess-tilings/src/lattice.rs:150–171`)

`cmin`/`cmax` are the cell polygons' **cell-space** AABB, but they are used as offsets in **lattice-index (u) space**. These coincide only when the cells' u-space extents happen to be dominated by their cell-space extents within the ±1 padding. This was checked by hand for all 15 current tables — all safe with wide margin (e.g. trihexagonal needs ±0.5/±1.0 in u-space; the padding provides ±1.5/±1.73) — but nothing enforces the relationship. A future table with a tall or strongly sheared cell relative to its lattice basis would silently drop edge tiles → gaps at region borders, and the coverage tests might still slip through depending on sample placement.

**Suggested fix:** inverse-transform each cell AABB's corners through `u(c) = A⁻¹(c − offset)` (exact for the bounding box) instead of using raw cell-space extents; optionally add a test asserting culling completeness (no tile just outside the generated index range intersects the query region).

### M3. Hirschhorn growth can stall the UI thread (`tess-tilings/src/hirschhorn.rs:926–997`)

`generate()` grows the patch synchronously (up to 250k tiles / 100k grow steps) while holding the cache mutex. The first render of a large-but-budget-passing view means seconds to minutes with the UI frozen. The commit history shows this hang class was patched via caching, but there is no time-slicing.

Two sub-issues:

- The `steps > MAX_GROW_STEPS` exit path **does not set `maxed`** (the other two failure paths do), so every subsequent frame re-attempts up to 100k grow steps — repeated multi-second stalls.
- That path returns the message "too many tiles for this view — zoom in", which is misleading: the real cause is growth slowness or frontier deadlock, and zooming in does not necessarily help.

### M4. GUI runs a full generation just for metadata, and generates twice per frame (`tess-gui/src/ui.rs:120`)

`self.app.result(1, 1)` in the side panel executes `Tiling::generate` every frame in order to read `symmetry` / `periodic` — data that is essentially static per tiling. For Hirschhorn this call can also trigger growth toward wherever the user has panned. A cheap metadata accessor (or per-tiling cache) would decouple panel rendering from tile generation.

## Findings — Low

### L1. NaN hole in the CLI (`tess-tui/src/main.rs`)

`--scale` rejects NaN via `!(scale > 0.0)`, but `--rotation NaN` is accepted → silently writes a **0-tile SVG and exits 0** (verified). Same for `--param x=NaN`. Suggest `is_finite()` on all parsed floats.

### L2. `check_deterministic` compares only polygon points (`tess-tilings/src/lib.rs:172–180`)

`proto` and `seed` are not compared, so a generator that nondeterministically shuffled seeds would still pass. Trivial to extend.

### L3. Dead defensive branch would break C6 symmetry if ever reached (`tess-tilings/src/hirschhorn.rs:679`)

`if orbit.len() <= 3 || orbit.len() == 6` accepts orbits of 1–3 tiles — which would violate the exact-C6 invariant the tests assert — while rejecting 4–5. Under the documented invariants orbits are always length 6, so the branch is unreachable; if it ever did trigger, symmetry would silently break. Prefer `assert_eq!(orbit.len(), 6)` or handle all short lengths consistently.

### L4. `Affine::inverse` silently returns identity on singular input (`tess-core/src/geometry.rs:180–184`)

Currently unreachable (parameter specs clamp scale ≥ 0.1, and `LatticeTiling::generate` has its own degenerate-determinant check), but callers (all four aperiodic generators) would then generate geometry in the wrong space with no signal. A `debug_assert!` or an `Option` return would surface the failure.

### L5. `Polygon::aabb` panics on empty input (`tess-core/src/geometry.rs:304`)

Indexes `pts[0]` without a guard. Latent — all generators emit ≥ 3-gons and the renderers skip shorter polygons — but cheap to harden.

### L6. `ParamKind::Bool` carries no default (`tess-core/src/params.rs:45`)

`default_value()` hardcodes `false`; bool parameters cannot default to on. Inconsistent with Float/Int/Angle, which all carry defaults. Currently unused by any tiling.

### L7. `Registry::register` silently overwrites duplicate ids (`tess-core/src/registry.rs:22`)

A second tiling registered under an existing id would corrupt `by_id` lookup while `iter()` lists both. A `debug_assert!` would catch registration typos immediately.

### L8. One-ulp constant error in `girih.rs:32`, and clippy is not clean

`SQ5 = 2.2360679774997898` is 1 ulp above √5 (correct f64: `2.23606797749979`, as `pinwheel.rs` uses). Harmless numerically (relative error ~4e-17 against 1e-9 tolerances) but contradicts the "exact radicals" convention in AGENTS.md.

Clippy (not part of the repo's stated bar — only `cargo check` is — but worth a pass): **2 errors** (`clippy::approx_constant` on the `1.4142135623730951` literals in `archimedean.rs:19` and `zellige.rs:22`) and **14 warnings** (`excessive_precision`, `map_or` → `is_none_or`, an unnecessary cast in `zellige.rs:113`, missing `Default` for `HirschhornTiling`, `clone` on `Copy` type, doc-list over-indentation).

### L9. `girih-quasiperiodic` deviates from the `Tiling` coverage contract by design (`tess-tilings/src/girih_qc.rs`)

It returns a fixed 41-tile patch; queries extending beyond the patch render background with no refusal message. This is honestly documented in the module docs and tested with a custom `check_patch`, but `Tiling`'s docs (`tess-core/src/tiling.rs:69–73`) state the "no gaps" contract unconditionally. The trait documentation (or a `finite: bool` flag on `TilingResult`) should note the exception so implementors and UIs can reason about it.

### L10. Test-suite maintainability

`tess-tilings/src/lib.rs` is ~900 lines of near-identical 3-test boilerplate × 15 tilings (a macro or helper would eliminate most of it), and the xorshift `Rng` is duplicated in four test modules. Also dead code: the `in_at_least_one` counter in `check_coverage` (`lib.rs:136,168`) is computed then discarded via `let _ =`.

### L11. Cosmetics

- TUI `list` uses `{:<12}` so long ids (`girih-quasiperiodic`, 18 chars) misalign the columns.
- `App::tiling()` panics via `expect` on an empty registry while `App::new()` tolerates one — inconsistent handling of an edge case that cannot currently occur.

## Positives worth keeping as-is

- **Generators are OOM-safe.** Every generator enforces `MAX_TILES` *before* expensive work via a cheap area lower bound (the Penrose OOM fix pattern, consistently applied in lattice/zellige/hirschhorn/pinwheel).
- **Failure is graceful and user-visible.** Every refusal path returns a message the GUI shows on-canvas; the lattice degenerate-determinant, Penrose far-from-seed, pinwheel, and Hirschhorn refusals are all reachable and tested.
- **The GUI tri-cache keyed on `(proto, vertex_count)`** is a subtle correctness fix (Penrose emits 3-vertex boundary triangles sharing a proto with 4-vertex rhombi; reusing the wrong triangulation crashed egui or left tiles half-unfilled) with a dedicated regression test — exactly the right pattern.
- **Culling soundness arguments are written down where they belong** (child-never-leaves-parent for both substitution tilings), and the tests actually verify them.
- **Geometry is built on exact radicals**, not decimal approximations, per the AGENTS.md convention — with the one-ulp exception noted in L8.

## Suggested priority

1. M2 — make lattice culling structurally correct (small, localized change + test).
2. M3 — set `maxed` on the `MAX_GROW_STEPS` path (one-line) and consider a per-frame time budget for growth.
3. M4 — decouple symmetry metadata from full generation in the GUI.
4. L1 — `is_finite()` validation in the CLI.
5. M1 — refresh README (19 tilings, 94 tests) and AGENTS.md.
6. Everything else opportunistically.

## Fixes applied

All findings from this review were fixed on 2026-11-07. Post-fix state:
**97/97 tests green** (94 original + 3 new), `cargo check --workspace
--all-targets` and `cargo clippy --workspace --all-targets` both at zero
warnings (clippy now clean — it was not part of the repo bar before),
`--no-default-features` build clean, CLI smoke tests pass. The diff was
kept minimal: incidental `cargo fmt` reformatting of untouched code
(compact data tables, enums) was reverted, preserving the authors'
hand-tuned layout.

| Finding | Fix | Verification |
|---|---|---|
| **M1** stale docs | README: 19 tilings (full table incl. snub-square, star-cross, 2× Cairo, girih ×2), 97 tests, star-radius parameter, girih section, cached-panel note, culling description. AGENTS.md: 97 tests / ~15 s, snub-square now "in this repo as `snub-square`", `check_deterministic` now documents proto+seed comparison, brute-force culling test documented with instructions to extend its list. | `tess-tui list` matches README table row-for-row |
| **M2** lattice culling bounds | `lattice.rs`: per-cell extents now measured in **u space** — every cell vertex mapped through `u(p) = A⁻¹p` (basis as columns) — with a `debug_assert` on the basis determinant and a rewritten comment explaining why cell-space extents were wrong. | New `lattice_culling_matches_brute_force` test: re-emits tiles over a fixed ±40 index window for **all 15 lattice tilings** × (default, rot30/scale0.7) and requires bit-identical tile sets (proto, seed, all vertex bits) |
| **M3** Hirschhorn stall | `MAX_GROW_STEPS` path now sets `maxed = true` (consistent with the other failure paths) and returns an honest message ("growth did not converge for this view"); comment notes the path is belt-and-braces since complete 6-orbits commit per step. | Read-verified; other maxed paths already tested |
| **M4** GUI double generation | `App::result_region(BBox)` added to `tess-core`; GUI side panel caches `(tiling id + all param values)` → symmetry/periodic metadata and refreshes it via a tiny ±0.5 probe region at the origin. Panning/zooming never touches generation for the panel. | New tests `metadata_probe_covers_every_tiling` (probe yields tiles for **every** registered tiling) and `metadata_cache_tracks_tiling_switch` |
| **L1** CLI NaN | New `finite()` parser used by `--scale`, `--rotation`, and Float/Angle `--param` values: rejects NaN and ±inf with explicit messages. | Smoke-tested: `--rotation NaN`, `--scale inf`, `--param scale=NaN` all exit 1 with clear errors; normal exports unaffected |
| **L2** deterministic check | `check_deterministic` now also compares `proto` and `seed` per tile. | Part of the 79 green tess-tilings tests |
| **L3** dead orbit branch | `orbit.len() <= 3 \|\| orbit.len() == 6` → `orbit.len() == 6` only, with a comment explaining why partial orbits would break exact C6 symmetry. | Hirschhorn tests (coverage, rotated, deterministic) still green |
| **L4** `Affine::inverse` | Added `debug_assert!(det.abs() >= 1e-15, ...)` before the silent-identity fallback; release stays total, debug builds fail at the call site. | Compiles; all generators unaffected (det bounded away from 0) |
| **L5** `Polygon::aabb` panic | Empty input now returns a degenerate zero-area `BBox` via `pts.first()`, with a doc comment. | Compiles; all real polygons are ≥ 3-gons |
| **L6** `Bool` default | `ParamKind::Bool { default: bool }`; `ParamSpec::bool(id, label, default)`; TUI/GUI match arms updated. Existing tests extended to cover `default = true`. | tess-core 15/15 green |
| **L7** registry dup ids | `Registry::register` gained `debug_assert!` on duplicate ids with the offending id in the message. | Compiles; registry tests green |
| **L8** √5 ulp + clippy | `SQ5` corrected to `2.236_067_977_499_79` (girih.rs); `SQRT2` literals replaced with `f64::consts::SQRT_2` in zellige.rs/archimedean.rs; all 2 clippy errors + 14 warnings fixed (doc-list indentation, `is_none_or`, useless cast, missing `Default` for `HirschhornTiling`, `clone` on Copy, excessive precision in penrose/pinwheel, complex-type aliases in tests, items-after-test-module, useless `format!`, negated comparison). | `cargo clippy --workspace --all-targets` = **0 warnings** |
| **L9** coverage contract | `Tiling::generate` trait docs now state the finite-patch exception explicitly (patch = covered domain; UIs render background beyond it). | Doc-only |
| **L10** test maintainability | Shared `crate::tests::Rng` (was duplicated 4×); `check_covers` / `check_rotated` / `check_det` helpers eliminate the 3-test boilerplate (lib.rs test code ~900 → ~560 lines, regions/params preserved exactly); dead `in_at_least_one` counter removed. | Suite green; every original test preserved 1:1 (regions, scales, rotations byte-compared against `git show HEAD`) |
| **L11** cosmetics | TUI `list` column width now derives from the longest id; `App::new` asserts non-empty registry (fail fast, consistent with `App::tiling`). | `tess-tui list` output aligned |

### Notes

- The five coverage tests that explicitly pin `scale = 1.0` (triangle,
  4.8.8, penrose, pinwheel, hirschhorn) keep that behavior in the
  refactored helpers — semantics preserved, not silently defaulted.
- `README.md` "Tests" section now also names the invariant censuses
  (pinwheel T-junctions, Hirschhorn χ = 1/C6/edge-ownership/20° frames,
  girih patch census, Cairo collinear integer properties) that the
  review found but the old README did not mention.
- New test count: 15 tess-core + 79 tess-tilings + 3 tess-gui = 97.
  Runtime ~15 s.