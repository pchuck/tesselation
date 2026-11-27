# Gallery taxonomy

M76 restructured the browse gallery into ten tabs
(`TilingCategory::ALL`) — eleven since 2026-11-24, when the Isohedral
tab was added for the Kaplan IH instances —: **Uniform** (11) · **Two-Uniform** (20) ·
**Duals & variants** (36) · **Wallpaper** (17) · **Pentagon
Types** (15) · **Isohedral** (5) · **Islamic** (29) · **Bonds & dissections** (8) ·
**Quasiperiodic** (23) · **Aperiodic** (11) · **Nonperiodic** (13) —
190 memberships over the 169 implemented tilings; the 21 extra are
secondary memberships. (Counts at this commit; the per-tab figures
are pinned by `registry_groups_are_complete_and_canonically_ordered`.)

- **Multi-tab membership.** `Tiling::secondary_categories()` (default
  `&[]`) lets a tiling appear in additional tabs: `triangle`,
  `square` and `hexagon` also appear under Laves (self-dual regular
  patterns), and seventeen tilings appear under the Wallpaper
  catalog.
  `browse_tabs` appends secondary categories to each card's tabs and
  sorts each tab by `section_key`, which now guards on the *tab's*
  category, not the card's (a Wallpaper tab is all-secondary).
  The ratatui UI pins the eleven-tab layout and per-tab counts, and the headless
  egui geometry test pins the new card budget (2-line card titles,
  no clipping at 1200×900, 640×800 and 2560×900).
- **Wallpaper catalog.** The Wallpaper tab is a group catalog, not a
  motif list: one card per realized wallpaper group — all 17:
  p1 → stepped-hexominoes, p2 → pentagon-3, p3 → chiral-cut-honeycomb,
  p31m → split-triangle-grid,
  p3m1 → alcove-cut-grid,
  pg → glide-hexagon-strips,
  pm → trapezoid-courses, cm → wavy-courses,
  pmg → shifted-trapezoid-courses,
  pmm → kuniform-pmm, pgg → herringbone, p4 → windmill,
  p4m → truncated-square, p4g → snub-square, p6 →
  snub-trihexagonal, p6m → hexagon, cmm →
  elongated-triangular. The catalog is complete since M85 (the
  alcove-cut grid closed the p3m1 gap); the shared note
  (`tess_core::browse::wallpaper_gap_note`) rendered at the top of
  the tab in both UIs now reports the completed catalog — no
  placeholder cards. (Historic bug fixed in
  M76: the 17th group is **pmg**, not cmc — Cmc is a 3D space group;
  the misnamed `WallpaperGroup::Cmc` variant is now `Pmg`.)
- **LatticeKind::CenteredRectangular.** A fifth lattice kind. The
  legality test
  (`declared_symmetry_is_legal_and_matches_the_measured_lattice`)
  pins each declared (group, lattice kind) against the 17-group
  table and re-measures the lattice from the declared basis vectors
  alone (hexagonal probe → orthogonal unimodular pair →
  centered-rectangular probe; tol 1e-6), never calling `generate()`.
  Re-declarations: pentagon-10 → centered, basis (3,3)/(−2,5);
  running-bond → centered, basis (2,0)/(1,1) 1-domino cell — the
  only declared-c-rect whose measured lattice is a √2-degenerate
  square lattice, the test's documented exception (point group 2mm,
  no 4-fold); girih-shesh-band → centered (p2); girih-star-cross →
  centered with group corrected pmm → cmm (the (77,45)/(0,90) basis
  is a primitive cell of the centered rectangular lattice);
  elongated-triangular and prismatic-pentagonal → centered;
  herringbone → rectangular, basis normalized to (2,2)/(1,−1).
- **Cairo merge.** `cairo-collinear` was the same Laves pattern
  (V3.3.4.3.4, dual of snub-square) as `cairo-pentagonal` with a
  different pentagon shape, so it was dropped as a separate card;
  `cairo-pentagonal` is now named "Cairo Pentagonal (V3.3.4.3.4)"
  and its description documents the collinear-vertex variant
  (143.13/108.43/108.43/90/90°, edges 4×√10 + 4, integral vertices,
  square 72-area cell). 62 → 61 tilings.
