//! Built-in tiling generators for tessellix.
//!
//! # Adding a new tiling
//!
//! Tilings live in families (`crate::families`) — the unit of parallel
//! work. To add a tiling to an existing family, create the tiling
//! module, declare it in the family file, and add it to that family's
//! `register` slice, `classification` arms, census `IDS` list (and
//! `lattice_tilings` when it is a lattice table): only the family file
//! and the new module change. See `families/mod.rs` and
//! `docs/briefs/` for the fleet protocol.
//!
//! New families are orchestrator edits: a module under `families/`, one
//! line in `families::chain`, one in `families::classify_into`, one
//! feature in `Cargo.toml`, one re-export group below.

pub mod lattice;

/// The per-tiling classification table (always compiled — it is data,
/// not a family).
pub mod classify;

pub use classify::classification;

// M86/M93: Taylor-Socolar and Jeandel-Rao deliberately carry no per-tiling
// feature gate (pre-refactor, the former was ungated despite a non-default
// `socolar` feature existing, and the latter had no feature at all). They
// remain unconditional members of the default-on `aperiodic` family.

/// Registry of all enabled built-in tilings.
pub mod families;

// Root re-exports: the pre-refactor `crate::<module>` paths keep working
// for moved code; new code should use `crate::families::<family>::<module>`.
// These are static — a new tiling does not add a line here.
#[cfg(feature = "uniform")]
pub use families::uniform::{square, hexagon, triangle, archimedean};
#[cfg(feature = "quasiperiodic")]
pub use families::quasiperiodic::{ammann_beenker, ammann_beenker_tri, shield, stampfli, golden_bee, golden_triangle, fractalizing, penrose, penrose_p1, penrose_p2, penrose_pentagon};
#[cfg(feature = "arrangements")]
pub use families::arrangements::{pinwheel, pythagorean, windmill};
#[cfg(feature = "pentagons")]
pub use families::pentagons::{pentagon1, pentagon2, pentagon3, pentagon7, pentagon8, pentagon6, pentagon9, pentagon10, pentagon11, pentagon12, pentagon13, pentagon14, pentagon15, hirschhorn};
#[cfg(feature = "laves")]
pub use families::laves::{cairo, laves};
#[cfg(feature = "kuniform")]
pub use families::kuniform::{kuniform, dual};
#[cfg(feature = "reptiles")]
pub use families::reptiles::{armchair, chair, sphinx, p_pentomino};
#[cfg(feature = "aperiodic")]
pub use families::aperiodic::{hat, turtle, spectre, taylor_socolar, jr_aperiodic};
#[cfg(feature = "pavers")]
pub use families::pavers::{basketweave, herringbone, running_bond, stepped_hexominoes, trapezoid_courses, glide_hexagon_strips, shifted_trapezoid_courses, wavy_courses, chiral_cut_honeycomb, split_triangle_grid, alcove_cut_grid, stack_bond, herringbone_45, basketweave_3};
#[cfg(feature = "islamic")]
pub use families::islamic::{hexagram, zellige, girih, girih_qc, girih_sc, girih_w85, pic_4_8_8, pic_3_6_3_6, pic_3_12_12, hankin, hankin_4_8_8, hankin_4_6_12, hankin_4_8_8_rosette_transform, hankin_hex, hankin_3_12_12, hankin_square_two_point, hankin_3_6_3_6, hankin_3_4_6_4, hankin_3_3_3_4_4, hankin_3_3_4_3_4, khatam_girih, najm, rosette_transform};

// ---------------------------------------------------------------------------
// Registry composition. The ten families register in chain order
// (`families::chain`); within a tab, cards appear in family-chain order
// and then in the family's registration order. Adding a tiling to an
// existing family never edits this file.
// ---------------------------------------------------------------------------

pub fn registry() -> tess_core::Registry {
    families::chain(tess_core::Registry::new())
}

#[cfg(test)]
mod tests {
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    /// Deterministic xorshift64 rng, shared by the test modules in this
    /// crate (`crate::tests::Rng`).
    pub(crate) struct Rng(pub(crate) u64);
    impl Rng {
        pub(crate) fn new(seed: u64) -> Self {
            Self(seed)
        }
        pub(crate) fn f64(&mut self) -> f64 {
            let mut v = self.0;
            v ^= v << 13;
            v ^= v >> 7;
            v ^= v << 17;
            self.0 = v;
            (v >> 11) as f64 * (1.0 / 9007199254740992.0)
        }
    }

    /// Verify a tiling exactly covers `region`: every sample point is in at
    /// least one tile, and the total clipped area matches the region area
    /// (catches both gaps and overlaps).
    pub fn check_coverage(t: &dyn Tiling, name: &str, params: &ParamSet, region: BBox) {
        let r = t.generate(params, &Query { region });
        assert!(
            r.message.is_none(),
            "{name}: {msg}",
            msg = r.message.unwrap()
        );
        assert!(!r.tiles.is_empty(), "{name}: no tiles generated");

        // Culling: every tile intersects the region.
        for tile in &r.tiles {
            assert!(
                tile.poly.aabb().padding(1e-9).intersects(region),
                "{name}: tile outside region: {:?}",
                tile.poly.aabb()
            );
        }

        let sz = region.size();
        for gy in 0..40 {
            for gx in 0..40 {
                let p = region.min
                    + Vec2::new(
                        sz.x * (gx as f64 + 0.5) / 40.0,
                        sz.y * (gy as f64 + 0.5) / 40.0,
                    );
                let n = r.tiles.iter().filter(|t| t.poly.contains(p)).count();
                assert!(n >= 1, "{name}: gap at {p}");
            }
        }
        let mut rng = Rng::new(0x9E37_79B9_7F4A_7C15);
        for _ in 0..400 {
            let p = region.min + Vec2::new(rng.f64() * sz.x, rng.f64() * sz.y);
            let n = r.tiles.iter().filter(|t| t.poly.contains(p)).count();
            assert!(n >= 1, "{name}: gap at {p}");
        }

        let mut area = 0.0;
        for tile in &r.tiles {
            if let Some(c) = tile.poly.clipped_to(&region) {
                area += c.area();
            }
        }
        let ra = region.area();
        assert!(
            (area - ra).abs() <= 1e-6 * ra,
            "{name}: area mismatch (tiles {area} vs region {ra})"
        );
    }

    /// Verify generation is deterministic (geometry, prototile class, and
    /// pattern seed must all be identical).
    pub fn check_deterministic(t: &dyn Tiling, name: &str, params: &ParamSet, region: BBox) {
        let q = Query { region };
        let a = t.generate(params, &q);
        let b = t.generate(params, &q);
        assert_eq!(a.tiles.len(), b.tiles.len(), "{name}: tile count differs");
        for (ta, tb) in a.tiles.iter().zip(b.tiles.iter()) {
            assert_eq!(ta.poly.pts, tb.poly.pts, "{name}: tile geometry differs");
            assert_eq!(ta.proto, tb.proto, "{name}: tile proto differs");
            assert_eq!(ta.seed, tb.seed, "{name}: tile seed differs");
        }
    }

    /// The standard `covers_region` check at default parameters.
    pub(crate) fn check_covers(t: &dyn Tiling, name: &str, region: BBox) {
        let p = ParamSet::default_for(t.params());
        check_coverage(t, name, &p, region);
    }

    /// The standard `rotated_scaled` check (coverage must survive a
    /// non-axis-aligned scale + rotation).
    pub(crate) fn check_rotated(
        t: &dyn Tiling,
        name: &str,
        region: BBox,
        scale: f64,
        rotation: f64,
    ) {
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(scale));
        p.set(crate::lattice::ROTATION_ID, ParamValue::Angle(rotation));
        check_coverage(t, name, &p, region);
    }

    /// The standard `deterministic` check at default parameters.
    pub(crate) fn check_det(t: &dyn Tiling, name: &str, region: BBox) {
        let p = ParamSet::default_for(t.params());
        check_deterministic(t, name, &p, region);
    }

    /// The GUI fills each tile with ear-clipped triangles; verify the
    /// triangulation of every generated tile is valid: n−2 triangles and
    /// area conserved to high precision (a sign of no overfill/gaps).
    /// The declared-category grouping both UIs render, via the shared
    /// [`tess_core::browse_tabs`] model: one tab per category in
    /// canonical [`TilingCategory::ALL`] order, each tiling in its
    /// primary tab plus every tab from its
    /// [`Tiling::secondary_categories`]. The per-tab census pins today's
    /// taxonomy — extend it when adding a tiling (and the README's
    /// count; the doc-sync guards
    /// `readme_catalog_matches_registry` and
    /// `family_docs_match_registrations` below fail the suite if
    /// the docs are not updated).
    #[test]
    fn registry_groups_are_complete_and_canonically_ordered() {
        let r = super::registry();
        let tabs = tess_core::browse_tabs(&r);

        // Exactly one tab per category, in canonical ALL order.
        assert_eq!(tabs.len(), tess_core::TilingCategory::ALL.len());
        for (tab, c) in tabs.iter().zip(tess_core::TilingCategory::ALL.iter()) {
            assert_eq!(tab.category, *c, "tabs follow TilingCategory::ALL order");
        }

        // Completeness: every tiling in its primary tab, every declared
        // secondary present, nothing else.
        let memberships: usize = tabs.iter().map(|t| t.cards.len()).sum();
        for t in r.iter() {
            let primary = tabs.iter().find(|tab| tab.category == t.category()).unwrap();
            assert!(
                primary.card_index_of(t.id()).is_some(),
                "primary tab missing {}",
                t.id()
            );
            for sec in t.secondary_categories() {
                let tab = tabs.iter().find(|tab| tab.category == *sec).unwrap();
                assert!(
                    tab.card_index_of(t.id()).is_some(),
                    "secondary tab {} missing {}",
                    sec,
                    t.id()
                );
            }
        }
        let expected_memberships: usize =
            r.len() + r.iter().map(|t| t.secondary_categories().len()).sum::<usize>();
        assert_eq!(memberships, expected_memberships, "membership census");

// Census of today's taxonomy (M127+M128+M129+M131+M132+M133+M134+M135+M136). 128 built-ins — 63 before
        // the cairo-collinear merge into cairo-pentagonal (M76) —
        // across the 10 tabs, in TilingCategory::ALL order:
        //   Uniform 11 (8 Archimedean + triangle/square/hexagon),
        //   TwoUniform 17, Laves 24 (21 primary: 8 incl. cairo-pentagonal
        //   + 13 machine-dualized k-uniform duals (M98, M102, M106, M120,
        //   M121) + 3 regular secondaries), Wallpaper 17 (the group
        //   catalog — one card per represented group; M85 completed it),
//   PentagonTypes 15, Islamic 25, Arrangements 8,
//   Quasiperiodic 12, Aperiodic 7, Nonperiodic 6.
// Membership census 142 = 128 primaries + 14 secondaries (3 Laves
        // + 9 Wallpaper + 2 PentagonTypes: cairo-pentagonal (type 4)
        // and floret-pentagonal (type 5) join the Pentagon tab as
        // typed secondary members, M122).
        // History: M50 16 Wallpaper under default features -> M55 17
        // (pentagon-15) -> M59 20 (pentagon-9) -> M60 21 (pentagon-11)
        // -> M62 22 (pentagon-12) -> M63 23 (pentagon-10) -> M64 24
        // (pentagon-3) -> M65 25 (pentagon-2) -> M72 26 (pentagon-7)
        // -> M76: the old single Wallpaper bucket (26) is replaced by
        // the 10-tab scheme above -> M77 PentagonTypes 10 -> 11
        // (pentagon-8) -> M74 PentagonTypes 11 -> 12 (pentagon-6)
        // -> M78 Wallpaper 9 -> 10 (stepped-hexominoes, p1)
        // -> M79 Wallpaper 10 -> 11 (trapezoid-courses, pm)
        // -> M80 Wallpaper 11 -> 12 (glide-hexagon-strips, pg)
        // -> M81 Wallpaper 12 -> 13 (shifted-trapezoid-courses, pmg)
        // -> M82 Wallpaper 13 -> 14 (wavy-courses, cm)
        // -> M83 Wallpaper 14 -> 15 (chiral-cut-honeycomb, p3)
        // -> M84 Wallpaper 15 -> 16 (split-triangle-grid, p31m)
        // -> M85 Wallpaper 16 -> 17 (alcove-cut-grid, p3m1 — the last
        // gap; catalog complete) -> M86 Aperiodic 4 -> 5
        // (taylor-socolar) -> M87 TwoUniform 2 -> 3 (kuniform-n18)
// -> M88 (kuniform-n9) -> M89 (kuniform-n8)
        // -> M90 TwoUniform 5 -> 6 (kuniform-n5) -> M91 (kuniform-n1)
// -> M92 (kuniform-n13) -> M93 Aperiodic 5 -> 6 (jeandel-rao)
// -> M94 Islamic 9 -> 12 (hankin-4-8-8-rosette,
        // hankin-4-6-12, hankin-hex-two-point)
        // -> M98 Laves 11 -> 15 (the dual() helper dualizes
        // kuniform-p6m / -n18 / -n9 / -n8; Laves-primary only).
        // -> M96 TwoUniform 8 -> 9 (kuniform-n20, the module's
        // first p6 entry)
        // -> M97 TwoUniform 9 -> 10 (kuniform-n12, the module's
        // first cmm entry)
        // -> M95 TwoUniform 10 -> 11 (kuniform-n19; the frontier
        // solve corrected the group to p6m — mirrors pass torus-exact,
// e = 3 under D6, matching the wiki's (t=3, e=3) exactly)
// -> M99 Islamic 13 -> 14 (hankin-square-two-point, the
        // two-point hankin entry on the square 4.4.4.4 substrate;
// θK = 45°, δ = 0.5, cmm, docs/families/islamic.md §54).
        // -> M99 Islamic 14 -> 15 (hankin-4-8-8-rosette-transform,
        // the rosette transform of the 4.8.8 substrate at θK = 45°,
        // docs/families/islamic.md §55).
        // -> M99 Islamic 15 -> 17 (najm-63-2e-0-3e-ac, najm-44-3e-0-0-a;
        // roadmap item 22b: the najm template generator, diss. §3.6.1,
        // docs/families/islamic.md §56).
        // -> M100 Islamic 17 -> 18 (hankin-3-6-3-6, the hankin
        // inference entry on the trihexagonal 3.6.3.6 substrate;
// θK = 60° — the regular hexagram, docs/families/islamic.md §57).
        // -> M100 Islamic 18 -> 19 (hankin-3-4-6-4, the hankin
        // inference entry on the rhombitrihexagonal 3.4.6.4
        // substrate; θK = 60°, docs/families/islamic.md §58).
// -> M102 Laves 15 -> 19 (the dual() helper dualizes
        // kuniform-n5 / -n1 / -n13 / -n20, k-uniform batch 2;
        // Laves-primary only; provisional numbering, renumbered at
        // integration).
// -> M103 TwoUniform 11 -> 13 (kuniform-n4, the cmm
        // 4⁴;3³.4² form, and kuniform-n14, the pmm 3⁶;3³.4² form;
        // torus-exact t/e vs the wiki parentheticals (2,4)/(3,4),
        // docs/families/kuniform.md §60 — the fork's provisional §61 renumbered
        // by landing order).
// -> M106 Laves 19 -> 22 (the dual() helper dualizes
        // kuniform-pmm / -n12 / -n19, k-uniform batch 3: rectangular,
        // centered-rectangular and hexagonal primals; Laves-primary
        // only; docs/families/kuniform.md §61 — the fork's provisional §64
        // renumbered by landing order).
        // -> M108 TwoUniform 13 -> 15 (kuniform-n3, the cmm
        // 4⁴;3³.4² sibling, and kuniform-n15, the cmm 3⁶;3³.4²
        // sibling; torus-exact t/e vs the wiki parentheticals
        // (3,5)/(4,5), docs/families/kuniform.md §62 — the fork's provisional §61
        // renumbered by landing order).
        // -> M107 Islamic 19 -> 20 (hankin-3-3-3-4-4, the hankin
        // inference entry on the elongated-triangular 3.3.3.4.4
        // substrate; θK = 60°, cmm, docs/families/islamic.md §63 — the fork's
        // provisional §65 renumbered by landing order).
// -> M104 Quasiperiodic 8 -> 9 (golden-bee, the Ammann A2
        // chair as a √φ-inflating irreptile substitution; Vince AMM
        // 121 (2014) / Durand–Shen–Vereshchagin DCG 63 (2020)
        // limit-periodic nuance documented; docs/families/quasiperiodic.md §64 — the
        // fork's provisional §62 renumbered by landing order; landed
        // after M107).
// -> M110 Quasiperiodic 9 -> 10 (fractalizing-3-12-12, Chavey's
        // limit-periodic dodecagon-dense fractalizing family over the
        // uniform 3.12.12 — flower-orbit planes of triangles, squares,
        // hexagons and dodecagons, each level shrunk by 2+sqrt3;
        // docs/families/quasiperiodic.md §65 — the fork's provisional §64 renumbered by
        // landing order; landed after M104).
        // -> M111 Islamic 20 -> 21 (hankin-3-3-4-3-4, the hankin
        // inference entry on the snub-square 3.3.4.3.4 substrate —
        // first chiral substrate; θK = 60°, measured p4g on the
        // centered lattice ⟨(L/2,L/2),(L/2,−L/2)⟩; docs/families/islamic.md §66 —
        // the fork's provisional claim, number free at landing;
        // landed after M110).
        // -> M114 Arrangements 5 -> 6 (stack-bond, the pmm stacked
        // 2:1 domino bond on the rectangular lattice <(2,0),(0,1)>;
        // one split domino per cell).
        // -> M115 Arrangements 6 -> 7 (herringbone-45, the M40 bond in
        // its 45-degree-laid canonical frame, pgg on the rectangular
        // lattice <(0,2*sqrt(2)),(sqrt(2),0)>).
        // -> M116 Arrangements 7 -> 8 (basketweave-3, the triple
        // basketweave paver bond: three horizontal courses plus one
        // vertical course, period 5; pmm on the rectangular lattice
        // <(6,0),(0,5)> — an index-3 superlattice of the true
        // <(2,0),(0,5)>; 15 split dominoes per cell).
        // -> M117 PentagonTypes 14 -> 15 (pentagon-1, the Reinhardt
        // type-1 prototype: two pentagons per hexagonal-metric cell;
        // measured cmm — the prototile is achiral — on the a²=3b²
        // special centered-rectangular lattice).
        // -> M118 TwoUniform 15 -> 16 (kuniform-n2, the p4m
        // 3.12.12;3.4.3.12 form — the 3-4-3-12 tiling: dodecagons
        // kissing flat-edge-to-flat-edge with square+4-triangle fills;
        // torus-exact t/e vs the wiki parentheticals (3,3); the
        // missing-number hunt of the closed 20-list, batch 4).
        // -> M119 TwoUniform 16 -> 17 (kuniform-n16, the p4g
        // 3³.4²;3².4.3.4 form — variant 1 of the 33344-33434 tiling;
        // 4 tile orbits (8+4+4+2 = 18 tiles) / 5 edge orbits measured
        // on the torus, matching the wiki's 4-isohedral/5-isotoxal;
        // measured coset structure: D4, axis mirrors pure, diagonal
        // cosets true glides — p4g).
        // -> M120 Laves 22 -> 23 (kuniform-dual-n2, the face-centroid
        // dual of the 3-4-3-12 tiling: four kites (3.4.3.12 orbit,
        // sides (3+√3)/6 & (2+√3)/√3, area 3/4+5√3/12) plus four
        // isosceles triangles (3.12.12 orbit, base 2+√3, area
        // 1+7√3/12) per cell; areas sum to 7+4√3 = |det|; measured
        // p4m — every group coset mod lattice represented by an
        // operation fixing the primal square center).
        // -> M121 Laves 23 -> 24 (kuniform-dual-n16, the face-centroid
        // "key-lock" dual of the p4g 3³.4²;3².4.3.4 tiling: eight
        // pentagons (3².4.3.4 orbit) plus four pentagons (3³.4² orbit),
        // both classes exactly 1/2+√3/4 — areas sum to 6+3√3 = |det|;
        // measured p4g — 4-fold center ((√3−1)/4, (5+√3)/4), pure axis
        // mirrors through the corner-2 point (−1, 1/2), both diagonal
        // classes true half-period glides, no mirror through the 4-fold
        // center).
        // -> M127 Quasiperiodic 10 -> 11 (golden-triangle, the √(−τ)-
        // inflating right-triangle reptile: legs 1, √τ, hypotenuse τ,
        // altitude cut into two similar children (τ⁻¹, τ^−1/2), both
        // mirror chiralities, Fibonacci λ-inflation λL = L ∪ S,
        // λS = L in exact Q(λ), λ = i√τ; Danzer–van Ophuysen 2000
        // triangle-pair family).
        // -> M128 Nonperiodic 4 -> 5 (armchair, the L-tetromino rep-4
        // substitution: the unique dissection of the scale-2 inflated
        // armchair into four half-side armchairs, one of each
        // orientation; limit-periodic — the depth-g block's only
        // translation period is (2^(g+1), 2^(g+1)); non-edge-to-edge
        // T-junctions at every scale).
        // -> M129 Quasiperiodic 10 -> 11 (ammann-beenker-tri, the
        // Bielefeld "Ammann-Beenker rhomb triangle" variant of the
        // classic A5: the half-square triangles are emitted as their
        // own tile class — 3 prototiles, rhombus + two mirror-image
        // half-squares — instead of being paired into squares; same
        // silver-mean deflation, H = 2P + U hypotenuse parity).
        let sizes: Vec<usize> = tabs.iter().map(|t| t.cards.len()).collect();
        assert_eq!(
            sizes,
vec![11, 17, 24, 17, 15, 25, 8, 12, 7, 6],
            "per-tab census (update when adding tilings)"
        );
assert_eq!(r.len(), 128, "built-in tiling count (update when adding tilings)");
        assert_eq!(memberships, 142, "membership census (update when adding tilings)");

        // The Wallpaper tab is the group catalog: exactly one card per
        // represented group, and the represented groups are precisely
        // the 17 groups minus the gap table.
        let wall = &tabs
            .iter()
            .find(|t| t.category == tess_core::TilingCategory::Wallpaper)
            .unwrap();
        let mut groups: Vec<&WallpaperGroup> = wall
            .cards
            .iter()
            .filter_map(|c| c.group.as_ref())
            .collect();
        groups.sort_unstable_by_key(|g| {
            WallpaperGroup::ALL
                .iter()
                .position(|a| *a == **g)
                .unwrap()
        });
        assert_eq!(groups.len(), wall.cards.len(), "every Wallpaper card carries a group");
        assert_eq!(groups.len(), 17, "seventeen represented wallpaper groups");
        let expected: Vec<&WallpaperGroup> = WallpaperGroup::ALL
            .iter()
            .filter(|g| !tess_core::browse::WALLPAPER_GROUP_GAPS.contains(*g))
            .collect();
        assert_eq!(groups, expected, "represented groups = 17 - gaps");
    }

    /// Doc-sync guard: the README's "Supported tilings" section must
    /// mirror the registry — the census sentence ("N built-in tilings
    /// (M tab memberships)"), the per-tab counts in "The ten tabs:"
    /// prose, and the catalog table (every registry id, exactly once,
    /// no stale rows). Without this guard the README drifted to
    /// 119/133 while the registry moved to 122/136 (11 rows never
    /// landed); this test fails the suite until the docs catch up.
    #[test]
    fn readme_catalog_matches_registry() {
        /// The number immediately preceding `phrase` (digits only).
        fn phrase_number(text: &str, phrase: &str, what: &str) -> usize {
            let i = text
                .find(phrase)
                .unwrap_or_else(|| panic!("README phrase missing: {what}"));
            let before = text[..i].trim_end();
            let start = before
                .rfind(|c: char| !c.is_ascii_digit())
                .map_or(0, |p| p + 1);
            before[start..]
                .parse()
                .unwrap_or_else(|_| panic!("README number not parseable: {what}"))
        }

        let readme = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../README.md"),
        )
        .expect("README.md readable from crates/tess-tilings");
        let r = super::registry();
        let tabs = tess_core::browse_tabs(&r);
        let memberships: usize = tabs.iter().map(|t| t.cards.len()).sum();

        // Census sentence.
        assert_eq!(
            phrase_number(&readme, " built-in tilings (", "built-in census"),
            r.len(),
            "README built-in tiling count"
        );
        assert_eq!(
            phrase_number(&readme, " tab memberships)", "membership census"),
            memberships,
            "README tab-membership count"
        );

        // Per-tab prose counts, in canonical order. The README's Laves
        // alias is "Laves / Catalan duals" (the category label uses
        // parentheses).
        let aliases = [
            "Uniform",
            "Two-Uniform",
            "Laves / Catalan duals",
            "Wallpaper",
            "Pentagon Types",
            "Islamic",
            "Arrangements",
            "Quasiperiodic",
            "Aperiodic",
            "Nonperiodic",
        ];
        for (tab, alias) in tabs.iter().zip(aliases) {
            let marker = format!("**{alias}** (");
            let i = readme
                .find(&marker)
                .unwrap_or_else(|| panic!("README per-tab prose missing for {alias}"));
            let digits: String = readme[i + marker.len()..]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            let n: usize = digits
                .parse()
                .unwrap_or_else(|_| panic!("README per-tab count not parseable for {alias}"));
            assert_eq!(n, tab.cards.len(), "README per-tab count for {alias}");
        }

        // Catalog table: header row, then `| `id` | ...` data rows.
        let table_start = readme
            .find("| id | name | type |")
            .expect("README catalog table header row");
        let mut ids: Vec<&str> = readme[table_start..]
            .lines()
            .skip(2) // header + separator
            .take_while(|l| l.starts_with("| `"))
            .map(|l| {
                let rest = &l["| `".len()..];
                rest.split('`')
                    .next()
                    .expect("catalog row id has a closing backtick")
            })
            .collect();
        ids.sort_unstable();
        let mut expected: Vec<&str> = r.iter().map(|t| t.id()).collect();
        expected.sort_unstable();
        assert_eq!(
            ids, expected,
            "README catalog table must list every registry id exactly once (missing, \
             stale, or duplicated rows)"
        );
    }

    /// Doc-sync guard for the per-family docs: each
    /// `docs/families/<family>.md` must state its family's registered
    /// size in the "Implemented (N tilings)" heading and list every
    /// registered id of that family exactly once across its tables —
    /// and no others (the drift that shipped 119 rows for a 122-tiling
    /// registry — 27 rows never landed, and a mis-merge absorbed the
    /// two-uniform/dual/pentagon tables into the paver-bonds table
    /// without section headers — is what this guard prevents, now per
    /// family so a failure names the one file to fix).
    #[test]
    fn family_docs_match_registrations() {
        let mut cases: Vec<(&str, fn(tess_core::Registry) -> tess_core::Registry)> = Vec::new();
        #[cfg(feature = "uniform")]
        cases.push(("uniform", crate::families::uniform::register));
        #[cfg(feature = "quasiperiodic")]
        cases.push(("quasiperiodic", crate::families::quasiperiodic::register));
        #[cfg(feature = "arrangements")]
        cases.push(("arrangements", crate::families::arrangements::register));
        #[cfg(feature = "pentagons")]
        cases.push(("pentagons", crate::families::pentagons::register));
        #[cfg(feature = "laves")]
        cases.push(("laves", crate::families::laves::register));
        #[cfg(feature = "kuniform")]
        cases.push(("kuniform", crate::families::kuniform::register));
        #[cfg(feature = "reptiles")]
        cases.push(("reptiles", crate::families::reptiles::register));
        #[cfg(feature = "aperiodic")]
        cases.push(("aperiodic", crate::families::aperiodic::register));
        #[cfg(feature = "pavers")]
        cases.push(("pavers", crate::families::pavers::register));
        #[cfg(feature = "islamic")]
        cases.push(("islamic", crate::families::islamic::register));
        assert!(!cases.is_empty(), "at least one family doc is guarded");

        for (fam, register) in cases {
            let doc = std::fs::read_to_string(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join(format!("../../docs/families/{fam}.md")),
            )
            .unwrap_or_else(|e| panic!("docs/families/{fam}.md readable: {e}"));
            let r = register(tess_core::Registry::new());

            let heading = doc
                .find("## Implemented (")
                .unwrap_or_else(|| panic!("docs/families/{fam}.md '## Implemented (' heading"));
            let heading_end = heading + "## Implemented (".len();
            let digits: String = doc[heading_end..]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            let n: usize = digits
                .parse()
                .unwrap_or_else(|_| panic!("docs/families/{fam}.md census not parseable"));
            assert_eq!(n, r.len(), "docs/families/{fam}.md Implemented census heading");

            // The Implemented section runs to the family's Build arcs.
            let section_end = doc[heading_end..]
                .find("\n## Build arcs")
                .map_or(doc.len(), |i| heading_end + i);
            let mut ids: Vec<&str> = doc[heading..section_end]
                .lines()
                .filter_map(|l| l.strip_prefix("| `"))
                .map(|rest| {
                    rest.split('`')
                        .next()
                        .expect("Implemented row id has a closing backtick")
                })
                .collect();
            ids.sort_unstable();
            let mut expected: Vec<&str> = r.iter().map(|t| t.id()).collect();
            expected.sort_unstable();
            assert_eq!(
                ids, expected,
                "docs/families/{fam}.md Implemented section must list every {fam} \
                 id exactly once (missing, stale, or duplicated rows)"
            );
        }
    }

    /// Part A legality test (M76). Two independent claims per tiling:
    ///
    /// 1. *Group legality*: the declared (group, lattice kind) pair is a
    ///    row of the crystallographic table (owner doc §4,
    ///    `docs/2D_Tilings_Reference.md`): p1/p2 admit any lattice;
    ///    pm/pg/pmm/pmg/pgg are primitive-rectangular only; cm/cmm are
    ///    centered-rectangular only; p4/p4m/p4g are square only;
    ///    p3/p3m1/p31m/p6/p6m are hexagonal only.
    /// 2. *Basis measurement*: when a fixed lattice basis is declared
    ///    (`Tiling::lattice_basis`), the lattice it spans must actually
    ///    be of the declared kind. Measured from the basis vectors
    ///    alone — no `generate()` call — via [`measured_kind`]. The
    ///    measurement reports the lattice's MAXIMAL metric type, so two
    ///    degenerate realizations are admitted: a rectangular or
    ///    centered-rectangular lattice whose vectors happen to form a
    ///    square metric lattice (equal-length orthogonal vectors) is
    ///    measured as Square even though the pattern's point group has
    ///    no 4-fold — a square-metric translation lattice under a
    ///    ≤2-fold group is a valid rectangular/c-rectangular Bravais
    ///    realization (running-bond: L = {x+y even}, constant √2).
    ///    Similarly, a centered-rectangular lattice whose vectors happen
    ///    to form a hexagonal metric lattice (a 60° equal-length basis,
    ///    the special a² = 3b² case) is measured as Hexagonal even
    ///    though the pattern's point group has no 3-/6-fold
    ///    (pentagon-1: the cmm tiling on Λ = Z(3,−√3) + Z(3,√3)).
    ///    Cross-combos in the other direction are NOT admitted: a
    ///    declared Square must measure Square, a declared Oblique must
    ///    measure Oblique.
    #[test]
    fn declared_symmetry_is_legal_and_matches_the_measured_lattice() {
        /// The lattice kinds a wallpaper group may sit on (the 17-group
        /// legality table; see the test docs).
        fn legal_kinds(g: WallpaperGroup) -> &'static [LatticeKind] {
            match g {
                WallpaperGroup::P1 | WallpaperGroup::P2 => &[
                    LatticeKind::Oblique,
                    LatticeKind::Rectangular,
                    LatticeKind::CenteredRectangular,
                    LatticeKind::Square,
                    LatticeKind::Hexagonal,
                ],
                WallpaperGroup::Pm
                | WallpaperGroup::Pg
                | WallpaperGroup::Pmm
                | WallpaperGroup::Pmg
                | WallpaperGroup::Pgg => &[LatticeKind::Rectangular],
                WallpaperGroup::Cm | WallpaperGroup::Cmm => &[LatticeKind::CenteredRectangular],
                WallpaperGroup::P4 | WallpaperGroup::P4m | WallpaperGroup::P4g => {
                    &[LatticeKind::Square]
                }
                WallpaperGroup::P3
                | WallpaperGroup::P3m1
                | WallpaperGroup::P31m
                | WallpaperGroup::P6
                | WallpaperGroup::P6m => &[LatticeKind::Hexagonal],
            }
        }

        /// Measure the lattice type of L = Z a + Z b from the basis
        /// vectors alone. Priority: hexagonal (an equal-length 60/120°
        /// basis exists), then orthogonal basis (square when the pair is
        /// equal-length, rectangular otherwise), then centered
        /// rectangular (an orthogonal pair spanning an index-2
        /// sublattice whose cell center is a lattice point), else
        /// oblique. Every stage is an exact invariant of the lattice, so
        /// the search bounds only affect finding the certificate, never
        /// the answer.
        fn measured_kind(a: Vec2, b: Vec2) -> LatticeKind {
            let det = a.cross(b);
            let tol = 1e-6 * a.dot(a).max(b.dot(b)).max(1.0);
            let span = |m: i32, n: i32, p: i32, q: i32| -> (Vec2, Vec2) {
                (a * m as f64 + b * n as f64, a * p as f64 + b * q as f64)
            };
            let is_basis = |d: f64| (d - det).abs() <= tol || (d + det).abs() <= tol;

            // Stage 1: hexagonal — a 60/120° equal-length basis of L.
            for m in -2..=2 {
                for n in -2..=2 {
                    if m == 0 && n == 0 {
                        continue;
                    }
                    for p in -2..=2 {
                        for q in -2..=2 {
                            if p == 0 && q == 0 {
                                continue;
                            }
                            let (u, v) = span(m, n, p, q);
                            if !is_basis(u.cross(v)) {
                                continue;
                            }
                            let (u2, v2) = (u.dot(u), v.dot(v));
                            if (u2 - v2).abs() > tol {
                                continue;
                            }
                            // 2 u·v = ±|u|²  <=>  60° or 120°.
                            if (2.0 * u.dot(v) - u2).abs() <= tol
                                || (2.0 * u.dot(v) + u2).abs() <= tol
                            {
                                return LatticeKind::Hexagonal;
                            }
                        }
                    }
                }
            }

            // Stage 2: an orthogonal basis of L — square when the pair is
            // equal-length, rectangular otherwise. (A centered lattice
            // never has an orthogonal basis, so this stage cannot
            // misclassify one.)
            //
            // NOTE: the lattice kinds checked here are the MAXIMAL metric
            // type of the lattice itself. A centered-rectangular Bravais
            // lattice may, as a special parameter, be hexagonal-metric
            // (a² = 3b², so a 60° equal-length basis exists) even under
            // a ≤2-fold group; that is admitted below.
            for m in -3..=3 {
                for n in -3..=3 {
                    if m == 0 && n == 0 {
                        continue;
                    }
                    for p in -3..=3 {
                        for q in -3..=3 {
                            if p == 0 && q == 0 {
                                continue;
                            }
                            let (u, v) = span(m, n, p, q);
                            if !is_basis(u.cross(v)) || u.dot(v).abs() > tol {
                                continue;
                            }
                            return if (u.dot(u) - v.dot(v)).abs() <= tol {
                                LatticeKind::Square
                            } else {
                                LatticeKind::Rectangular
                            };
                        }
                    }
                }
            }

            // Stage 3: centered rectangular — an orthogonal pair (u, v)
            // spanning an index-2 sublattice with the cell center
            // (u+v)/2 back in L (even coefficient sums). A primitive
            // rectangular or oblique lattice has no such pair.
            for m in -3..=3 {
                for n in -3..=3 {
                    if m == 0 && n == 0 {
                        continue;
                    }
                    for p in -3..=3 {
                        for q in -3..=3 {
                            if p == 0 && q == 0 {
                                continue;
                            }
                            if (m + p) % 2 != 0 || (n + q) % 2 != 0 {
                                continue;
                            }
                            let (u, v) = span(m, n, p, q);
                            if (u.cross(v) - 2.0 * det).abs() > tol
                                && (u.cross(v) + 2.0 * det).abs() > tol
                            {
                                continue;
                            }
                            if u.dot(v).abs() > tol {
                                continue;
                            }
                            return LatticeKind::CenteredRectangular;
                        }
                    }
                }
            }

            LatticeKind::Oblique
        }

        for t in super::registry().iter() {
            let Some(meta) = t.declared_symmetry() else {
                continue; // aperiodic / quasiperiodic / parameterized
            };
            assert!(
                legal_kinds(meta.group).contains(&meta.lattice),
                "{}: {} does not permit lattice kind {}",
                t.id(),
                meta.group,
                meta.lattice
            );
            if let Some([a, b]) = t.lattice_basis() {
                let measured = measured_kind(a, b);
                assert!(
                    measured == meta.lattice
                        || (measured == LatticeKind::Square
                            && matches!(
                                meta.lattice,
                                LatticeKind::Rectangular | LatticeKind::CenteredRectangular
                            ))
                        || (measured == LatticeKind::Hexagonal
                            && meta.lattice == LatticeKind::CenteredRectangular),
                    "{}: declared {} but the declared basis measures as {}",
                    t.id(),
                    meta.lattice,
                    measured
                );
            }
        }
    }

    #[test]
    fn all_tiles_triangulate() {
        let v2 = |x: f64, y: f64| Vec2::new(x, y);
        let region = BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0));
        for t in super::registry().iter() {
            let mut p = ParamSet::default_for(t.params());
            p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
            let r = t.generate(&p, &Query { region });
            assert!(
                r.message.is_none(),
                "{}: {msg}",
                t.name(),
                msg = r.message.unwrap_or("")
            );
            assert!(!r.tiles.is_empty(), "{}: no tiles", t.name());
            for tile in &r.tiles {
                let tris = tile.poly.triangulate();
                assert_eq!(
                    tris.len(),
                    tile.poly.pts.len() - 2,
                    "{}: triangle count",
                    t.name()
                );
                let area: f64 = tris
                    .iter()
                    .map(|[a, b, c]| {
                        let (a, b, c) = (
                            tile.poly.pts[*a as usize],
                            tile.poly.pts[*b as usize],
                            tile.poly.pts[*c as usize],
                        );
                        (b - a).cross(c - a).abs() * 0.5
                    })
                    .sum();
                let pa = tile.poly.area();
                assert!(
                    (area - pa).abs() <= 1e-9 * pa.max(1e-6),
                    "{}: triangulated area {area} != polygon area {pa}",
                    t.name()
                );
            }
        }
    }
}

#[cfg(test)]
mod lattice_culling_tests {
    use super::lattice::LatticeTiling;
    // The lattice entries come from the family modules
    // (`families::<family>::lattice_tilings()`); a family feature
    // adds its own entries, so this list can never go stale.
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::ParamSet;
    use tess_core::tiling::{Query, Tile, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Independent emission: enumerate a wide fixed index window straight
    /// from the public table data and keep tiles whose world AABB
    /// intersects the region.
    fn brute_force(lt: &LatticeTiling, params: &ParamSet, region: &BBox, window: i64) -> Vec<Tile> {
        let t = (lt.make_map)(params);
        let l1 = t.transform(lt.lattice[0]);
        let l2 = t.transform(lt.lattice[1]);
        let origin = t.transform(lt.offset);
        let world_cells: Vec<(Polygon, u32)> = lt
            .cells
            .iter()
            .map(|c| (c.poly.transform(&t), c.seed))
            .collect();
        let mut out = Vec::new();
        for i in -window..=window {
            for j in -window..=window {
                let tr = origin + l1 * i as f64 + l2 * j as f64;
                for (ci, (poly, seed)) in world_cells.iter().enumerate() {
                    let bb = poly.aabb();
                    let bb = BBox::new(bb.min + tr, bb.max + tr);
                    if bb.intersects(*region) {
                        out.push(Tile {
                            proto: ci as u8,
                            poly: poly.translated(tr),
                            seed: lt.seed_pattern.base(i, j).wrapping_add(*seed),
                        });
                    }
                }
            }
        }
        out
    }

    /// Canonical sort key: (proto, seed, all vertex bits). Both
    /// implementations build tile geometry with identical operations, so
    /// equal keys imply bit-identical geometry.
    fn key(tile: &Tile) -> (u8, u32, Vec<u64>) {
        let mut bits = Vec::with_capacity(tile.poly.pts.len() * 2);
        for p in &tile.poly.pts {
            bits.push(p.x.to_bits());
            bits.push(p.y.to_bits());
        }
        (tile.proto, tile.seed, bits)
    }

    fn compare(lt: &LatticeTiling, params: &ParamSet, region: BBox, label: &str) {
        let got = lt.generate(params, &Query { region });
        assert!(got.message.is_none(), "{label}: {:?}", got.message);
        assert!(!got.tiles.is_empty(), "{label}: no tiles");
        let mut a: Vec<_> = got.tiles.iter().map(key).collect();
        let mut b: Vec<_> = brute_force(lt, params, &region, 40)
            .iter()
            .map(key)
            .collect();
        a.sort();
        b.sort();
        assert_eq!(
            a.len(),
            b.len(),
            "{label}: tile count {} vs {}",
            a.len(),
            b.len()
        );
        for (ka, kb) in a.iter().zip(b.iter()) {
            assert_eq!(ka, kb, "{label}: tile mismatch");
        }
    }

    #[test]
    fn lattice_culling_matches_brute_force() {
    let mut lattices: Vec<(&str, LatticeTiling)> = Vec::new();
    #[cfg(feature = "uniform")]
    lattices.extend(crate::families::uniform::lattice_tilings());
    #[cfg(feature = "arrangements")]
    lattices.extend(crate::families::arrangements::lattice_tilings());
    #[cfg(feature = "pentagons")]
    lattices.extend(crate::families::pentagons::lattice_tilings());
    #[cfg(feature = "laves")]
    lattices.extend(crate::families::laves::lattice_tilings());
    #[cfg(feature = "kuniform")]
    lattices.extend(crate::families::kuniform::lattice_tilings());
    #[cfg(feature = "pavers")]
    lattices.extend(crate::families::pavers::lattice_tilings());
    #[cfg(feature = "islamic")]
    lattices.extend(crate::families::islamic::lattice_tilings());
        for (name, lt) in &lattices {
            let specs = lt.params();
            // Default parameters and a rotated/scaled variant: the rotated
            // case is what makes naive cell-space padding wrong for
            // non-axis-aligned lattices.
            compare(
                lt,
                &ParamSet::default_for(specs),
                BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
                name,
            );
            let mut p = ParamSet::default_for(specs);
            p.set(
                crate::lattice::SCALE_ID,
                tess_core::params::ParamValue::Float(0.7),
            );
            p.set(
                crate::lattice::ROTATION_ID,
                tess_core::params::ParamValue::Angle(30.0),
            );
            compare(
                lt,
                &p,
                BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)),
                &format!("{name}(rot30,scale0.7)"),
            );
        }
    }
}


/// M57 layer-mode verification anchors: the generic strapwork engine in
/// `tess_core::strapwork` laid on each canonical registry substrate at
/// the canonical contact angle must reproduce the registered PIC
/// partition's internal edges **exactly** (1e-9 on endpoints, 1e-6 key
/// grid) — layer mode and tiling mode pinned to each other (PROPOSED #14).
#[cfg(all(test, feature = "islamic", feature = "uniform"))]
mod strapwork_layer_mode {
    use std::collections::BTreeMap;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::ParamSet;
    use tess_core::strapwork::{walls, Wall};
    use tess_core::tiling::{Query, Tiling};

    type SegKey = ((i64, i64), (i64, i64));

    fn key_pt(p: Vec2) -> (i64, i64) {
        ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64)
    }

    /// Endpoint-canonical (order- and direction-independent) key.
    fn key_seg(a: Vec2, b: Vec2) -> SegKey {
        let (ka, kb) = (key_pt(a), key_pt(b));
        if ka <= kb {
            (ka, kb)
        } else {
            (kb, ka)
        }
    }

    fn inside(p: Vec2, r: f64) -> bool {
        p.x.abs() <= r && p.y.abs() <= r
    }

    /// Engine walls over the tiles of `substrate` intersecting `region`,
    /// kept inside the radius-`r` box (where coverage is complete).
    fn engine_set(substrate: &dyn Tiling, delta_deg: f64, region: BBox, r: f64) -> BTreeMap<SegKey, Wall> {
        let params = ParamSet::default_for(substrate.params());
        let res = substrate.generate(&params, &Query { region });
        walls(&res.tiles, delta_deg)
            .into_iter()
            .filter(|w| inside(w.a, r) && inside(w.b, r))
            .map(|w| (key_seg(w.a, w.b), w))
            .collect()
    }

    /// Internal edges of the registered PIC partition: edges shared by
    /// exactly 2 generated faces, kept inside the radius-`r` box.
    fn pic_set(pic: &dyn Tiling, region: BBox, r: f64) -> BTreeMap<SegKey, [Vec2; 2]> {
        let params = ParamSet::default_for(pic.params());
        let res = pic.generate(&params, &Query { region });
        let mut count: BTreeMap<SegKey, u32> = BTreeMap::new();
        let mut raw: BTreeMap<SegKey, [Vec2; 2]> = BTreeMap::new();
        for tile in &res.tiles {
            let q = &tile.poly.pts;
            for i in 0..q.len() {
                let (a, b) = (q[i], q[(i + 1) % q.len()]);
                let k = key_seg(a, b);
                *count.entry(k).or_default() += 1;
                raw.entry(k).or_insert([a, b]);
            }
        }
        raw.into_iter()
            .filter(|(k, seg)| count[k] == 2 && inside(seg[0], r) && inside(seg[1], r))
            .collect()
    }

    fn dist(a: Vec2, b: Vec2) -> f64 {
        (a - b).length()
    }

    /// The anchor: engine walls ≡ PIC internal edges — same key sets,
    /// endpoints to 1e-9 (best pairing), equal length sums; and stable
    /// against window growth (no boundary artifacts on either side).
    fn check_anchor(substrate_id: &str, pic_id: &str, delta_deg: f64, region: BBox, r: f64) {
        let registry = crate::registry();
        let substrate = registry.get(substrate_id).expect(substrate_id);
        let pic = registry.get(pic_id).expect(pic_id);
        let grown = BBox::new(
            Vec2::new(region.min.x - 3.0, region.min.y - 3.0),
            Vec2::new(region.max.x + 3.0, region.max.y + 3.0),
        );

        let eng = engine_set(substrate, delta_deg, region, r);
        let pic_edges = pic_set(pic, region, r);
        assert!(!eng.is_empty() && !pic_edges.is_empty(), "{pic_id}: empty anchor sets");

        let missing: Vec<_> = pic_edges.keys().copied().filter(|k| !eng.contains_key(k)).collect();
        let extra: Vec<_> = eng.keys().copied().filter(|k| !pic_edges.contains_key(k)).collect();
        assert!(
            missing.is_empty() && extra.is_empty(),
            "{substrate_id}@{delta_deg}° vs {pic_id}: engine={} pic={} missing={missing:?} extra={extra:?}",
            eng.len(),
            pic_edges.len()
        );

        let mut worst = 0.0f64;
        let mut len_e = 0.0f64;
        let mut len_p = 0.0f64;
        for (k, w) in &eng {
            let seg = &pic_edges[k];
            let pair = dist(w.a, seg[0])
                .max(dist(w.b, seg[1]))
                .min(dist(w.a, seg[1]).max(dist(w.b, seg[0])));
            worst = worst.max(pair);
            len_e += w.length();
            len_p += dist(seg[0], seg[1]);
        }
        assert!(worst < 1e-9, "{pic_id}: worst endpoint mismatch {worst:e}");
        assert!((len_e - len_p).abs() < 1e-9, "{pic_id}: length sums {len_e} vs {len_p}");

        // Window-size invariance (same filter radius, grown region).
        assert_eq!(eng.keys().collect::<Vec<_>>(), engine_set(substrate, delta_deg, grown, r).keys().collect::<Vec<_>>(), "{substrate_id}: engine set shifted with the window");
        assert_eq!(pic_edges.keys().collect::<Vec<_>>(), pic_set(pic, grown, r).keys().collect::<Vec<_>>(), "{pic_id}: internal-edge set shifted with the window");
    }

    #[test]
    fn layer_walls_4_8_8_match_pic_partition() {
        check_anchor(
            "truncated-square",
            "pic-4-8-8",
            45.0,
            BBox::new(Vec2::new(-8.0, -8.0), Vec2::new(8.0, 8.0)),
            6.0,
        );
    }

    #[test]
    fn layer_walls_3_6_3_6_match_pic_partition() {
        check_anchor(
            "trihexagonal",
            "pic-3-6-3-6",
            60.0,
            BBox::new(Vec2::new(-8.0, -8.0), Vec2::new(8.0, 8.0)),
            5.0,
        );
    }

    #[test]
    fn layer_walls_3_12_12_match_pic_partition() {
        check_anchor(
            "truncated-hexagonal",
            "pic-3-12-12",
            60.0,
            BBox::new(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0)),
            6.0,
        );
    }
}
