# Arrangements family

Arrangement constructions: the Pythagorean two-squares family and the windmill (the periodic pinwheel) as data tables, plus the pinwheel substitution generator itself.

Code: `crates/tess-tilings/src/families/arrangements/` — cargo feature `arrangements` (default-on).
Adding a tiling here touches only this family: `families/arrangements.rs`
(module decl, register slice, classification arms, census `IDS`,
`lattice_tilings` entries) plus the new module and its doc row below.
Work briefs: `docs/briefs/TEMPLATE.md`; fleet protocol: `tessellix/AGENTS.md`.

## Implemented (3 tilings)

### Two-squares arrangements (periodic)

| id | motif | group | notes |
|---|---|---|---|
| `pythagorean` | two squares (Pythagorean) | p4 | one-parameter ratio r = b/a; each cell is the square on the hypotenuse dissected into the leg squares; T-junctions inherent; the plain p4m square grid at r = 1 |
| `windmill` | 1–2–√5 triangles around squares ("windmill") | p4 | periodic pinwheel arrangement; homochiral (no mirrors/glides); non-edge-to-edge T-junctions; naming waiver 2026-11-12 (M43) |

### Pinwheel (nonperiodic arrangement)

| id | motif | notes |
|---|---|---|
| `pinwheel` | 1–2–√5 right triangle | Conway–Radin; √5 inflation + arctan(1/2) twist; infinitely many tile orientations; view-independent octave-anchored patches |

## Build arcs

Numbered arcs keep their PATTERNS.MD §numbers — code comments and
briefs cite them as `§N`.

### 2. Pythagorean (two squares) ✅ (M26)

- **Motif:** squares of two sizes, every square touching the other size
  on its sides; unilateral, equitransitive, *not* edge-to-edge
  (T-junctions inherent — documented like pinwheel's).
- **Symmetry:** p4, chiral (no mirrors) — confirmed both analytically (a
  mirror would need the lattice orientation arctan(a/b) ≡ 0 mod 45°, i.e.
  a = b) and numerically (38 four-fold centers, zero reflections in a
  generated patch); at r = 1 the tiling is the plain square grid, p4m.
- **Lattice:** rotated square lattice with the LARGER side first,
  `[(s1, s2), (−s2, s1)]`, cell = big square `[0,s1]²` + small square
  `[s1, s1+s2] × [0, s2]`, cell area exactly s1² + s2² = |det| (the
  square-on-hypotenuse dissection). **The plan's `[(b, a), (−a, b)]` is
  only valid for b ≥ a**: for r < 1 the small square at (b, 0) pokes into
  the big square one lattice step down-right (overlap b(a−b)), so the
  construction canonicalizes (s1, s2) = (max(a,b), min(a,b)); r < 1 is
  the r′ = 1/r family, congruent up to scale.
- **Implementation:** direct `Tiling` impl in `pythagorean.rs` (feature
  `pythagorean`, shared with pinwheel) with the lattice.rs index-range
  culling inline; third parameter `ratio` (0.2–10, step 0.05, default φ)
  following the star-cross pattern.
- **Verification:** Python sweep (coverage grid + sub-edge parity over a
  ±3 patch) across r ∈ {0.2 … 10}; the Rust `pythagorean_ratio_sweep_covers`
  test re-checks coverage at 7 ratios *plus* rotated/scaled views at
  steep ratios, and `pythagorean_culling_matches_brute_force` re-emits a
  fixed index window and requires bit-identical tile sets at r = 1.1
  (basis tilted 42°) and 2.5. **The sweep caught two real bugs before
  commit**: the r < 1 notch overflow above, and a culling bug (region
  bbox taken in cell space instead of lattice/u space — harmless for
  axis-aligned lattices like star-cross, wrong for rotated ones; fixed
  by mirroring lattice.rs's world→u inverse exactly).
- **Bonus tie-in (kept for future work):** irrational ratios give
  aperiodic 1-D cross sections (Klotz construction); default ratio φ
  gives Fibonacci-word cross sections.


---

*Map: [docs/INDEX.md](../INDEX.md) · Changelog: [docs/history/CHANGELOG.md](../history/CHANGELOG.md) · Queue: [docs/ROADMAP.md](../ROADMAP.md)*
