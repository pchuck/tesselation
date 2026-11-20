//! Built-in tiling generators for tessellix.
//!
//! # Adding a new tiling
//!
//! For periodic (wallpaper) tilings, add a module that returns a
//! [`lattice::LatticeTiling`] data table (see `square.rs` / `hexagon.rs`),
//! gate it behind a cargo feature, and register it in [`registry`].
//!
//! For aperiodic or continuous families, implement [`tess_core::Tiling`]
//! directly and register it the same way. UIs discover everything through
//! the registry and the tiling's parameter specs.

pub mod lattice;

/// The per-tiling classification table (always compiled — it is data,
/// not a family).
pub mod classify;

pub use classify::classification;

#[cfg(feature = "ammann_beenker")]
pub mod ammann_beenker;
#[cfg(feature = "archimedean")]
pub mod archimedean;
#[cfg(feature = "cairo")]
pub mod cairo;
#[cfg(feature = "chair")]
pub mod chair;
#[cfg(feature = "kuniform")]
pub mod dual;
#[cfg(feature = "girih")]
pub mod girih;
#[cfg(feature = "girih_ext")]
pub mod girih_w85;
#[cfg(feature = "hankin_ext")]
pub mod pic_3_12_12;
#[cfg(feature = "hankin_ext")]
pub mod pic_3_6_3_6;
#[cfg(feature = "hankin_ext")]
pub mod pic_4_8_8;
#[cfg(feature = "hankin_ext")]
pub mod hankin;
#[cfg(feature = "hankin_ext")]
pub mod hankin_4_8_8;
#[cfg(feature = "hankin_ext")]
pub mod hankin_4_6_12;
#[cfg(feature = "hankin_ext")]
pub mod hankin_4_8_8_rosette_transform;
#[cfg(feature = "hankin_ext")]
pub mod hankin_hex;
#[cfg(feature = "hankin_ext")]
pub mod hankin_3_12_12;
#[cfg(feature = "hankin_ext")]
pub mod hankin_square_two_point;
#[cfg(feature = "hankin_ext")]
pub mod rosette_transform;
#[cfg(feature = "hankin_ext")]
pub mod hankin_3_6_3_6;
#[cfg(feature = "hankin_ext")]
pub mod hankin_3_4_6_4;
#[cfg(feature = "hankin_ext")]
pub mod hankin_3_3_3_4_4;
#[cfg(feature = "hankin_ext")]
pub mod hankin_3_3_4_3_4;
#[cfg(feature = "girih_qc")]
pub mod girih_qc;
#[cfg(feature = "girih_sc")]
pub mod girih_sc;
#[cfg(feature = "khatam_girih")]
pub mod khatam_girih;
#[cfg(feature = "kuniform")]
pub mod kuniform;
#[cfg(feature = "laves")]
pub mod laves;
#[cfg(feature = "najm")]
pub mod najm;
#[cfg(feature = "hexagram")]
pub mod hexagram;
#[cfg(feature = "hexagon")]
pub mod hexagon;
#[cfg(feature = "hirschhorn")]
pub mod hirschhorn;
#[cfg(feature = "reptiles")]
pub mod p_pentomino;
#[cfg(feature = "penrose")]
pub mod penrose;
#[cfg(feature = "penrose_p1")]
pub mod penrose_p1;
#[cfg(feature = "penrose_p2")]
pub mod penrose_p2;
#[cfg(feature = "penrose_pentagon")]
pub mod penrose_pentagon;
#[cfg(feature = "pythagorean")]
pub mod pinwheel;
#[cfg(feature = "pythagorean")]
pub mod pythagorean;
#[cfg(feature = "sphinx")]
pub mod sphinx;
#[cfg(feature = "hat")]
pub mod hat;
#[cfg(feature = "turtle")]
pub mod turtle;
#[cfg(feature = "spectre")]
pub mod spectre;
#[cfg(feature = "stampfli")]
pub mod stampfli;
#[cfg(feature = "golden_bee")]
pub mod golden_bee;
#[cfg(feature = "fractalizing")]
pub mod fractalizing;
#[cfg(feature = "shield")]
pub mod shield;
#[cfg(feature = "square")]
pub mod square;
#[cfg(feature = "triangle")]
pub mod triangle;
#[cfg(feature = "windmill")]
pub mod windmill;
#[cfg(feature = "zellige")]
pub mod zellige;
#[cfg(feature = "pavers")]
pub mod basketweave;
#[cfg(feature = "pavers")]
pub mod herringbone;
#[cfg(feature = "pavers")]
pub mod running_bond;
#[cfg(feature = "pavers")]
pub mod stepped_hexominoes;
#[cfg(feature = "pavers")]
pub mod trapezoid_courses;
#[cfg(feature = "pavers")]
pub mod glide_hexagon_strips;
#[cfg(feature = "pavers")]
pub mod shifted_trapezoid_courses;
#[cfg(feature = "pavers")]
pub mod wavy_courses;
#[cfg(feature = "pavers")]
pub mod chiral_cut_honeycomb;
#[cfg(feature = "pavers")]
pub mod split_triangle_grid;
#[cfg(feature = "pavers")]
pub mod alcove_cut_grid;
#[cfg(feature = "pavers")]
pub mod stack_bond;
#[cfg(feature = "pavers")]
pub mod herringbone_45;
#[cfg(feature = "pavers")]
pub mod basketweave_3;
#[cfg(feature = "pentagon1")]
pub mod pentagon1;
#[cfg(feature = "pentagon2")]
pub mod pentagon2;
#[cfg(feature = "pentagon3")]
pub mod pentagon3;
#[cfg(feature = "pentagon7")]
pub mod pentagon7;
pub mod pentagon6;
pub mod pentagon8;
#[cfg(feature = "pentagon9")]
pub mod pentagon9;
#[cfg(feature = "pentagon10")]
pub mod pentagon10;
#[cfg(feature = "pentagon11")]
pub mod pentagon11;
#[cfg(feature = "pentagon12")]
pub mod pentagon12;
#[cfg(feature = "pentagon13")]
pub mod pentagon13;
#[cfg(feature = "pentagon14")]
pub mod pentagon14;
#[cfg(feature = "pentagon15")]
pub mod pentagon15;
// Unconditional: the `socolar` feature exists in Cargo.toml but is not a
// default feature (M86 deviation — M74-style), so the module and its
// registration are ungated.
pub mod taylor_socolar;
// M93: the Jeandel-Rao aperiodic tiling has no feature gate in
// Cargo.toml (untouchable per the task constraints), so the module and
// its registration are unconditional.
pub mod jr_aperiodic;

/// Registry of all enabled built-in tilings.
pub fn registry() -> tess_core::Registry {
    // `mut` is only needed when at least one tiling is registered
    // (pentagon8 is unconditional, so this allow is inert).
    #[allow(unused_mut)]
    let mut r = tess_core::Registry::new();
    #[cfg(feature = "square")]
    {
        r = r.register(square::spec());
    }
    #[cfg(feature = "hexagon")]
    {
        r = r.register(hexagon::spec());
    }
    #[cfg(feature = "hexagram")]
    {
        r = r.register(hexagram::HexagramTiling);
    }
    #[cfg(feature = "triangle")]
    {
        r = r.register(triangle::spec());
    }
    #[cfg(feature = "archimedean")]
    {
        r = r.register(archimedean::truncated_square());
        r = r.register(archimedean::trihexagonal());
        r = r.register(archimedean::snub_trihexagonal());
        r = r.register(archimedean::elongated_triangular());
        r = r.register(archimedean::rhombitrihexagonal());
        r = r.register(archimedean::truncated_trihexagonal());
        r = r.register(archimedean::truncated_hexagonal());
        r = r.register(archimedean::snub_square());
    }
    #[cfg(feature = "ammann_beenker")]
    {
        r = r.register(ammann_beenker::AmmannBeenkerTiling);
    }
    #[cfg(feature = "shield")]
    {
        r = r.register(shield::ShieldTiling);
    }
    #[cfg(feature = "stampfli")]
    {
        r = r.register(stampfli::StampfliTiling);
    }
    #[cfg(feature = "golden_bee")]
    {
        r = r.register(golden_bee::GoldenBeeTiling);
    }
    #[cfg(feature = "fractalizing")]
    {
        r = r.register(fractalizing::FractalizingTiling);
    }
    #[cfg(feature = "penrose")]
    {
        r = r.register(penrose::PenroseTiling);
    }
    #[cfg(feature = "penrose_p1")]
    {
        r = r.register(penrose_p1::PenroseP1Tiling);
    }
    #[cfg(feature = "penrose_p2")]
    {
        r = r.register(penrose_p2::PenroseP2Tiling);
    }
    #[cfg(feature = "penrose_pentagon")]
    {
        r = r.register(penrose_pentagon::PenrosePentagonTiling);
    }
    #[cfg(feature = "pythagorean")]
    {
        r = r.register(pinwheel::PinwheelTiling);
        r = r.register(pythagorean::PythagoreanTiling);
    }
    #[cfg(feature = "pentagon2")]
    {
        r = r.register(pentagon2::spec());
    }
    #[cfg(feature = "pentagon3")]
    {
        r = r.register(pentagon3::spec());
    }
    #[cfg(feature = "pentagon1")]
    {
        r = r.register(pentagon1::spec());
    }
    #[cfg(feature = "pentagon7")]
    {
        r = r.register(pentagon7::spec());
    }
    {
        // Unconditional: the pentagon8 feature does not exist in
        // Cargo.toml (M77 deviation — see PORT_REPORT.md), so the
        // module and its registration are ungated.
        r = r.register(pentagon8::spec());
    }
    {
        // Unconditional: the pentagon6 feature exists in Cargo.toml
        // but is not a default feature (M74 deviation — M77-style), so
        // the module and its registration are ungated.  Registered
        // after pentagon-8: the Pentagon-types tab lists pentagon-6
        // after pentagon-8.
        r = r.register(pentagon6::spec());
    }
    #[cfg(feature = "pentagon9")]
    {
        r = r.register(pentagon9::spec());
    }
    #[cfg(feature = "pentagon10")]
    {
        r = r.register(pentagon10::spec());
    }
    #[cfg(feature = "pentagon11")]
    {
        r = r.register(pentagon11::spec());
    }
    #[cfg(feature = "pentagon12")]
    {
        r = r.register(pentagon12::spec());
    }
    #[cfg(feature = "pentagon13")]
    {
        r = r.register(pentagon13::spec());
    }
    #[cfg(feature = "pentagon14")]
    {
        r = r.register(pentagon14::spec());
    }
    #[cfg(feature = "pentagon15")]
    {
        r = r.register(pentagon15::spec());
    }
    #[cfg(feature = "hirschhorn")]
    {
        r = r.register(hirschhorn::HirschhornTiling::new());
    }
    #[cfg(feature = "zellige")]
    {
        r = r.register(zellige::StarCrossTiling);
    }
    #[cfg(feature = "cairo")]
    {
        r = r.register(cairo::cairo_pentagonal());
    }
    #[cfg(feature = "chair")]
    {
        r = r.register(chair::ChairTiling);
    }
    #[cfg(feature = "sphinx")]
    {
        r = r.register(sphinx::SphinxTiling);
    }
    #[cfg(feature = "reptiles")]
    {
        r = r.register(p_pentomino::PentPentominoTiling);
    }
    #[cfg(feature = "hat")]
    {
        r = r.register(hat::HatTiling);
    }
    #[cfg(feature = "turtle")]
    {
        r = r.register(turtle::TurtleTiling);
    }
    #[cfg(feature = "spectre")]
    {
        r = r.register(spectre::SpectreTiling);
    }
    #[cfg(feature = "girih")]
    {
        r = r.register(girih::girih_shesh_band());
    }
    #[cfg(feature = "girih_qc")]
    {
        r = r.register(girih_qc::GirihQuasiperiodicTiling);
    }
    #[cfg(feature = "girih_sc")]
    {
        r = r.register(girih_sc::girih_star_cross());
    }
    #[cfg(feature = "girih_ext")]
    {
        r = r.register(girih_w85::girih_w85());
    }
    #[cfg(feature = "hankin_ext")]
    {
        r = r.register(pic_4_8_8::pic_4_8_8());
        r = r.register(pic_3_6_3_6::pic_3_6_3_6());
        r = r.register(pic_3_12_12::pic_3_12_12());
        r = r.register(hankin_4_8_8::hankin_4_8_8_rosette());
        r = r.register(hankin_4_6_12::hankin_4_6_12());
        r = r.register(hankin_hex::hankin_hex_two_point());
r = r.register(hankin_3_12_12::hankin_3_12_12());
        r = r.register(hankin_square_two_point::hankin_square_two_point());
r = r.register(hankin_4_8_8_rosette_transform::hankin_4_8_8_rosette_transform());
        r = r.register(hankin_3_6_3_6::hankin_3_6_3_6());
        r = r.register(hankin_3_4_6_4::hankin_3_4_6_4());
        r = r.register(hankin_3_3_3_4_4::hankin_3_3_3_4_4());
        r = r.register(hankin_3_3_4_3_4::hankin_3_3_4_3_4());
    }
    #[cfg(feature = "khatam_girih")]
    {
        r = r.register(khatam_girih::khatam_girih());
    }
    #[cfg(feature = "najm")]
    {
        r = r.register(najm::najm_63_2e_0_3e_ac());
        r = r.register(najm::najm_44_3e_0_0_a());
    }
    #[cfg(feature = "kuniform")]
    {
        r = r.register(kuniform::kuniform_p6m());
        r = r.register(kuniform::kuniform_pmm());
        r = r.register(kuniform::kuniform_n18());
        r = r.register(kuniform::kuniform_n9());
        r = r.register(kuniform::kuniform_n8());
        r = r.register(kuniform::kuniform_n5());
        r = r.register(kuniform::kuniform_n1());
        r = r.register(kuniform::kuniform_n13());
        r = r.register(kuniform::kuniform_n20());
r = r.register(kuniform::kuniform_n12());
        r = r.register(kuniform::kuniform_n19());
        r = r.register(kuniform::kuniform_n4());
        r = r.register(kuniform::kuniform_n14());
        r = r.register(kuniform::kuniform_n3());
        r = r.register(kuniform::kuniform_n15());
        r = r.register(kuniform::kuniform_n2());
        r = r.register(kuniform::kuniform_n16());
    }
    #[cfg(feature = "kuniform")]
    {
        r = r.register(dual::kuniform_dual_p6m());
        r = r.register(dual::kuniform_dual_n18());
        r = r.register(dual::kuniform_dual_n9());
        r = r.register(dual::kuniform_dual_n8());
        r = r.register(dual::kuniform_dual_n5());
        r = r.register(dual::kuniform_dual_n1());
        r = r.register(dual::kuniform_dual_n13());
        r = r.register(dual::kuniform_dual_n20());
        r = r.register(dual::kuniform_dual_pmm());
        r = r.register(dual::kuniform_dual_n12());
        r = r.register(dual::kuniform_dual_n19());
        r = r.register(dual::kuniform_dual_n2());
    }
    #[cfg(feature = "laves")]
    {
        r = r.register(laves::tetrakis_square());
        r = r.register(laves::rhombille());
        r = r.register(laves::triakis_triangular());
        r = r.register(laves::deltoidal_trihexagonal());
        r = r.register(laves::kisrhombille());
        r = r.register(laves::prismatic_pentagonal());
        r = r.register(laves::floret_pentagonal());
    }
    #[cfg(feature = "windmill")]
    {
        r = r.register(windmill::spec());
    }
    #[cfg(feature = "pavers")]
    {
        r = r.register(basketweave::basketweave());
        r = r.register(herringbone::herringbone());
        r = r.register(running_bond::running_bond());
        r = r.register(stepped_hexominoes::stepped_hexominoes());
        r = r.register(trapezoid_courses::trapezoid_courses());
        r = r.register(glide_hexagon_strips::glide_hexagon_strips());
        r = r.register(shifted_trapezoid_courses::shifted_trapezoid_courses());
        r = r.register(wavy_courses::wavy_courses());
        r = r.register(chiral_cut_honeycomb::chiral_cut_honeycomb());
        r = r.register(split_triangle_grid::split_triangle_grid());
        r = r.register(alcove_cut_grid::alcove_cut_grid());
        r = r.register(stack_bond::stack_bond());
        r = r.register(herringbone_45::herringbone_45());
        r = r.register(basketweave_3::basketweave_3());
    }
    {
        // Unconditional: the socolar feature exists in Cargo.toml
        // but is not a default feature (M86 deviation — M74-style), so
        // the module and its registration are ungated.
        r = r.register(taylor_socolar::TaylorSocolarTiling);
    }
    {
        // Unconditional: no feature gate exists in Cargo.toml for the
        // Jeandel-Rao tiling (M93), so the registration is ungated.
        r = r.register(jr_aperiodic::JeandelRaoTiling);
    }
    r
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
    /// count).
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

// Census of today's taxonomy (M120). 118 built-ins — 63 before
        // the cairo-collinear merge into cairo-pentagonal (M76) —
        // across the 10 tabs, in TilingCategory::ALL order:
        //   Uniform 11 (8 Archimedean + triangle/square/hexagon),
        //   TwoUniform 17, Laves 23 (20 primary: 8 incl. cairo-pentagonal
        //   + 12 machine-dualized k-uniform duals (M98, M102, M106, M120) +
        //   3 regular secondaries), Wallpaper 17 (the group catalog —
        //   one card per represented group; M85 completed it),
//   PentagonTypes 15, Islamic 21, Arrangements 8,
//   Quasiperiodic 10, Aperiodic 6, Nonperiodic 4.
// Membership census 132 = 118 primaries + 14 secondaries (3 Laves
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
// θK = 45°, δ = 0.5, cmm, PATTERNS.MD §54).
        // -> M99 Islamic 14 -> 15 (hankin-4-8-8-rosette-transform,
        // the rosette transform of the 4.8.8 substrate at θK = 45°,
        // PATTERNS.MD §55).
        // -> M99 Islamic 15 -> 17 (najm-63-2e-0-3e-ac, najm-44-3e-0-0-a;
        // roadmap item 22b: the najm template generator, diss. §3.6.1,
        // PATTERNS.MD §56).
        // -> M100 Islamic 17 -> 18 (hankin-3-6-3-6, the hankin
        // inference entry on the trihexagonal 3.6.3.6 substrate;
// θK = 60° — the regular hexagram, PATTERNS.MD §57).
        // -> M100 Islamic 18 -> 19 (hankin-3-4-6-4, the hankin
        // inference entry on the rhombitrihexagonal 3.4.6.4
        // substrate; θK = 60°, PATTERNS.MD §58).
// -> M102 Laves 15 -> 19 (the dual() helper dualizes
        // kuniform-n5 / -n1 / -n13 / -n20, k-uniform batch 2;
        // Laves-primary only; provisional numbering, renumbered at
        // integration).
// -> M103 TwoUniform 11 -> 13 (kuniform-n4, the cmm
        // 4⁴;3³.4² form, and kuniform-n14, the pmm 3⁶;3³.4² form;
        // torus-exact t/e vs the wiki parentheticals (2,4)/(3,4),
        // PATTERNS.MD §60 — the fork's provisional §61 renumbered
        // by landing order).
// -> M106 Laves 19 -> 22 (the dual() helper dualizes
        // kuniform-pmm / -n12 / -n19, k-uniform batch 3: rectangular,
        // centered-rectangular and hexagonal primals; Laves-primary
        // only; PATTERNS.MD §61 — the fork's provisional §64
        // renumbered by landing order).
        // -> M108 TwoUniform 13 -> 15 (kuniform-n3, the cmm
        // 4⁴;3³.4² sibling, and kuniform-n15, the cmm 3⁶;3³.4²
        // sibling; torus-exact t/e vs the wiki parentheticals
        // (3,5)/(4,5), PATTERNS.MD §62 — the fork's provisional §61
        // renumbered by landing order).
        // -> M107 Islamic 19 -> 20 (hankin-3-3-3-4-4, the hankin
        // inference entry on the elongated-triangular 3.3.3.4.4
        // substrate; θK = 60°, cmm, PATTERNS.MD §63 — the fork's
        // provisional §65 renumbered by landing order).
// -> M104 Quasiperiodic 8 -> 9 (golden-bee, the Ammann A2
        // chair as a √φ-inflating irreptile substitution; Vince AMM
        // 121 (2014) / Durand–Shen–Vereshchagin DCG 63 (2020)
        // limit-periodic nuance documented; PATTERNS.MD §64 — the
        // fork's provisional §62 renumbered by landing order; landed
        // after M107).
// -> M110 Quasiperiodic 9 -> 10 (fractalizing-3-12-12, Chavey's
        // limit-periodic dodecagon-dense fractalizing family over the
        // uniform 3.12.12 — flower-orbit planes of triangles, squares,
        // hexagons and dodecagons, each level shrunk by 2+sqrt3;
        // PATTERNS.MD §65 — the fork's provisional §64 renumbered by
        // landing order; landed after M104).
        // -> M111 Islamic 20 -> 21 (hankin-3-3-4-3-4, the hankin
        // inference entry on the snub-square 3.3.4.3.4 substrate —
        // first chiral substrate; θK = 60°, measured p4g on the
        // centered lattice ⟨(L/2,L/2),(L/2,−L/2)⟩; PATTERNS.MD §66 —
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
        let sizes: Vec<usize> = tabs.iter().map(|t| t.cards.len()).collect();
        assert_eq!(
            sizes,
vec![11, 17, 23, 17, 15, 21, 8, 10, 6, 4],
            "per-tab census (update when adding tilings)"
        );
assert_eq!(r.len(), 118, "built-in tiling count (update when adding tilings)");
        assert_eq!(memberships, 132, "membership census (update when adding tilings)");

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

#[cfg(all(test, feature = "square"))]
mod square_tests {
    use super::tests::{check_covers, check_det, check_rotated};
    use crate::square;
    use tess_core::geometry::{BBox, Vec2};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn square_covers_region() {
        check_covers(
            &square::spec(),
            "square",
            BBox::new(v2(-3.0, -2.0), v2(17.0, 13.0)),
        );
    }

    #[test]
    fn square_rotated_scaled() {
        check_rotated(
            &square::spec(),
            "square(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(4.0, 4.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn square_deterministic() {
        check_det(
            &square::spec(),
            "square",
            BBox::new(v2(-2.0, -2.0), v2(12.0, 10.0)),
        );
    }
}

#[cfg(all(test, feature = "hexagon"))]
mod hexagon_tests {
    use super::tests::{check_covers, check_det, check_rotated};
    use crate::hexagon;
    use tess_core::geometry::{BBox, Vec2};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn hexagon_covers_region() {
        check_covers(
            &hexagon::spec(),
            "hexagon",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 11.0)),
        );
    }

    #[test]
    fn hexagon_rotated_scaled() {
        check_rotated(
            &hexagon::spec(),
            "hexagon(rot20,scale1.5)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            1.5,
            20.0,
        );
    }

    #[test]
    fn hexagon_deterministic() {
        check_det(
            &hexagon::spec(),
            "hexagon",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }
}

#[cfg(all(test, feature = "triangle"))]
mod triangle_tests {
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::triangle;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn triangle_covers_region() {
        let t = triangle::spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(
            &t,
            "triangle",
            &p,
            BBox::new(v2(-3.0, -2.0), v2(13.0, 11.0)),
        );
    }

    #[test]
    fn triangle_rotated_scaled() {
        check_rotated(
            &triangle::spec(),
            "triangle(rot15,scale0.6)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.6,
            15.0,
        );
    }

    #[test]
    fn triangle_deterministic() {
        check_det(
            &triangle::spec(),
            "triangle",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }
}

#[cfg(all(test, feature = "archimedean"))]
mod archimedean_tests {
    use super::tests::{check_coverage, check_covers, check_det, check_rotated};
    use crate::archimedean;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn truncated_square_covers_region() {
        let t = archimedean::truncated_square();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "4.8.8", &p, BBox::new(v2(-4.0, -3.0), v2(12.0, 12.0)));
    }

    #[test]
    fn truncated_square_rotated_scaled() {
        check_rotated(
            &archimedean::truncated_square(),
            "4.8.8(rot20,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            20.0,
        );
    }

    #[test]
    fn truncated_square_deterministic() {
        check_det(
            &archimedean::truncated_square(),
            "4.8.8",
            BBox::new(v2(-2.0, -2.0), v2(8.0, 8.0)),
        );
    }

    #[test]
    fn trihexagonal_covers_region() {
        check_covers(
            &archimedean::trihexagonal(),
            "3.6.3.6",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn trihexagonal_rotated_scaled() {
        check_rotated(
            &archimedean::trihexagonal(),
            "3.6.3.6(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn trihexagonal_deterministic() {
        check_det(
            &archimedean::trihexagonal(),
            "3.6.3.6",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn snub_trihexagonal_covers_region() {
        check_covers(
            &archimedean::snub_trihexagonal(),
            "3.3.3.3.6",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn snub_trihexagonal_rotated_scaled() {
        check_rotated(
            &archimedean::snub_trihexagonal(),
            "3.3.3.3.6(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn snub_trihexagonal_deterministic() {
        check_det(
            &archimedean::snub_trihexagonal(),
            "3.3.3.3.6",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn elongated_triangular_covers_region() {
        check_covers(
            &archimedean::elongated_triangular(),
            "3.3.3.4.4",
            BBox::new(v2(-3.0, -2.0), v2(11.0, 10.0)),
        );
    }

    #[test]
    fn elongated_triangular_rotated_scaled() {
        check_rotated(
            &archimedean::elongated_triangular(),
            "3.3.3.4.4(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn elongated_triangular_deterministic() {
        check_det(
            &archimedean::elongated_triangular(),
            "3.3.3.4.4",
            BBox::new(v2(-3.0, -2.0), v2(10.0, 9.0)),
        );
    }

    #[test]
    fn rhombitrihexagonal_covers_region() {
        check_covers(
            &archimedean::rhombitrihexagonal(),
            "3.4.6.4",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn rhombitrihexagonal_rotated_scaled() {
        check_rotated(
            &archimedean::rhombitrihexagonal(),
            "3.4.6.4(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn rhombitrihexagonal_deterministic() {
        check_det(
            &archimedean::rhombitrihexagonal(),
            "3.4.6.4",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn truncated_trihexagonal_covers_region() {
        check_covers(
            &archimedean::truncated_trihexagonal(),
            "4.6.12",
            BBox::new(v2(-4.0, -4.0), v2(16.0, 16.0)),
        );
    }

    #[test]
    fn truncated_trihexagonal_rotated_scaled() {
        check_rotated(
            &archimedean::truncated_trihexagonal(),
            "4.6.12(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn truncated_trihexagonal_deterministic() {
        check_det(
            &archimedean::truncated_trihexagonal(),
            "4.6.12",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn truncated_hexagonal_covers_region() {
        check_covers(
            &archimedean::truncated_hexagonal(),
            "3.12.12",
            BBox::new(v2(-4.0, -4.0), v2(16.0, 16.0)),
        );
    }

    #[test]
    fn truncated_hexagonal_rotated_scaled() {
        check_rotated(
            &archimedean::truncated_hexagonal(),
            "3.12.12(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn truncated_hexagonal_deterministic() {
        check_det(
            &archimedean::truncated_hexagonal(),
            "3.12.12",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn snub_square_covers_region() {
        check_covers(
            &archimedean::snub_square(),
            "3.3.4.3.4",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn snub_square_rotated_scaled() {
        check_rotated(
            &archimedean::snub_square(),
            "3.3.4.3.4(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn snub_square_deterministic() {
        check_det(
            &archimedean::snub_square(),
            "3.3.4.3.4",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }
}

#[cfg(all(test, feature = "penrose"))]
mod penrose_tests {
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::penrose;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn penrose_covers_region() {
        let t = penrose::PenroseTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "penrose", &p, BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)));
    }

    #[test]
    fn penrose_rotated_scaled() {
        check_rotated(
            &penrose::PenroseTiling,
            "penrose(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn penrose_deterministic() {
        check_det(
            &penrose::PenroseTiling,
            "penrose",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(all(test, feature = "penrose_p1"))]
mod penrose_p1_tests {
    use super::tests::{check_covers, check_det, check_rotated};
    use crate::penrose_p1;
    use tess_core::geometry::{BBox, Vec2};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn penrose_p1_covers_region() {
        check_covers(
            &penrose_p1::PenroseP1Tiling,
            "penrose_p1",
            BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
        );
    }

    #[test]
    fn penrose_p1_rotated_scaled() {
        check_rotated(
            &penrose_p1::PenroseP1Tiling,
            "penrose_p1(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn penrose_p1_deterministic() {
        check_det(
            &penrose_p1::PenroseP1Tiling,
            "penrose_p1",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(all(test, feature = "penrose_p2"))]
mod penrose_p2_tests {
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::penrose_p2;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn penrose_p2_covers_region() {
        let t = penrose_p2::PenroseP2Tiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "penrose_p2", &p, BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)));
    }

    #[test]
    fn penrose_p2_rotated_scaled() {
        check_rotated(
            &penrose_p2::PenroseP2Tiling,
            "penrose_p2(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn penrose_p2_deterministic() {
        check_det(
            &penrose_p2::PenroseP2Tiling,
            "penrose_p2",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }

    #[test]
    fn penrose_p2_star_seed_covers_region() {
        let t = penrose_p2::PenroseP2Tiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        p.set(
            penrose_p2::SEED_STAR_ID,
            ParamValue::Bool(true),
        );
        check_coverage(&t, "penrose_p2(star)", &p, BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)));
    }
}

#[cfg(all(test, feature = "penrose_pentagon"))]
mod penrose_pentagon_tests {
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::penrose_pentagon;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn penrose_pentagon_covers_region() {
        let t = penrose_pentagon::PenrosePentagonTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "penrose_pentagon", &p, BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)));
    }

    #[test]
    fn penrose_pentagon_rotated_scaled() {
        check_rotated(
            &penrose_pentagon::PenrosePentagonTiling,
            "penrose_pentagon(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn penrose_pentagon_deterministic() {
        check_det(
            &penrose_pentagon::PenrosePentagonTiling,
            "penrose_pentagon",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(all(test, feature = "chair"))]
mod chair_tests {
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::chair;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn chair_covers_region() {
        let t = chair::ChairTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "chair", &p, BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)));
    }

    #[test]
    fn chair_rotated_scaled() {
        check_rotated(
            &chair::ChairTiling,
            "chair(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn chair_deterministic() {
        check_det(
            &chair::ChairTiling,
            "chair",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(all(test, feature = "sphinx"))]
mod sphinx_tests {
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::sphinx;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn sphinx_covers_region() {
        let t = sphinx::SphinxTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "sphinx", &p, BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)));
    }

    #[test]
    fn sphinx_rotated_scaled() {
        check_rotated(
            &sphinx::SphinxTiling,
            "sphinx(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn sphinx_deterministic() {
        check_det(
            &sphinx::SphinxTiling,
            "sphinx",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(all(test, feature = "reptiles"))]
mod p_pentomino_tests {
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::p_pentomino;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn p_pentomino_covers_region() {
        let t = p_pentomino::PentPentominoTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "p-pentomino", &p, BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)));
    }

    #[test]
    fn p_pentomino_rotated_scaled() {
        check_rotated(
            &p_pentomino::PentPentominoTiling,
            "p-pentomino(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn p_pentomino_deterministic() {
        check_det(
            &p_pentomino::PentPentominoTiling,
            "p-pentomino",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(all(test, feature = "hat"))]
mod hat_tests {
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::hat;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn hat_covers_region() {
        let t = hat::HatTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "hat", &p, BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)));
    }

    /// Seeded coverage generalizes across anchor cells: large and
    /// far-from-origin views (different 96-grid anchors) are covered
    /// too, each verified by the exact clipped-area accounting of
    /// `check_coverage`.
    #[test]
    fn hat_covers_large_and_distant_views() {
        let t = hat::HatTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        for (cx, cy, w, h) in [
            (47.3, -91.2, 40.0, 30.0),
            (-133.0, 71.0, 60.0, 45.0),
            (90.0, -40.0, 90.0, 66.0),
            (483.0, 217.0, 30.0, 22.0),
        ] {
            check_coverage(
                &t,
                "hat",
                &p,
                BBox::new(v2(cx - w, cy - h), v2(cx + w, cy + h)),
            );
        }
    }

    #[test]
    fn hat_rotated_scaled() {
        check_rotated(
            &hat::HatTiling,
            "hat(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn hat_deterministic() {
        check_det(
            &hat::HatTiling,
            "hat",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(all(test, feature = "turtle"))]
mod turtle_tests {
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::turtle;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn turtle_covers_region() {
        let t = turtle::TurtleTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "turtle", &p, BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)));
    }

    /// Rigid-patch coverage generalizes across anchor cells and depths:
    /// large and far-from-origin views (different anchors and depths 4
    /// and 5) are covered too, each verified by the exact clipped-area
    /// accounting of `check_coverage`.
    #[test]
    fn turtle_covers_large_and_distant_views() {
        let t = turtle::TurtleTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        for (cx, cy, w, h) in [
            (47.3, -91.2, 40.0, 30.0),
            (-133.0, 71.0, 60.0, 45.0),
            (90.0, -40.0, 90.0, 66.0),
            (483.0, 217.0, 30.0, 22.0),
        ] {
            check_coverage(
                &t,
                "turtle",
                &p,
                BBox::new(v2(cx - w, cy - h), v2(cx + w, cy + h)),
            );
        }
    }

    #[test]
    fn turtle_rotated_scaled() {
        check_rotated(
            &turtle::TurtleTiling,
            "turtle(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn turtle_deterministic() {
        check_det(
            &turtle::TurtleTiling,
            "turtle",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(all(test, feature = "spectre"))]
mod spectre_tests {
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::spectre;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn spectre_covers_region() {
        let t = spectre::SpectreTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "spectre", &p, BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)));
    }

    /// Rigid-patch coverage generalizes across anchor cells and depths:
    /// large and far-from-origin views (different anchors and depth 5)
    /// are covered too, each verified by the exact clipped-area
    /// accounting of `check_coverage`.
    #[test]
    fn spectre_covers_large_and_distant_views() {
        let t = spectre::SpectreTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        for (cx, cy, w, h) in [
            (47.3, -91.2, 40.0, 30.0),
            (-133.0, 71.0, 60.0, 45.0),
            (90.0, -40.0, 70.0, 45.0),
            (483.0, 217.0, 30.0, 22.0),
        ] {
            check_coverage(
                &t,
                "spectre",
                &p,
                BBox::new(v2(cx - w, cy - h), v2(cx + w, cy + h)),
            );
        }
    }

    #[test]
    fn spectre_rotated_scaled() {
        check_rotated(
            &spectre::SpectreTiling,
            "spectre(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn spectre_deterministic() {
        check_det(
            &spectre::SpectreTiling,
            "spectre",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(all(test, feature = "ammann_beenker"))]
mod ammann_beenker_tests {
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::ammann_beenker;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn ammann_beenker_covers_region() {
        let t = ammann_beenker::AmmannBeenkerTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(
            &t,
            "ammann-beenker",
            &p,
            BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
        );
    }

    #[test]
    fn ammann_beenker_rotated_scaled() {
        check_rotated(
            &ammann_beenker::AmmannBeenkerTiling,
            "ammann-beenker(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn ammann_beenker_deterministic() {
        check_det(
            &ammann_beenker::AmmannBeenkerTiling,
            "ammann-beenker",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(all(test, feature = "stampfli"))]
mod stampfli_tests {
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::stampfli::StampfliTiling;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn stampfli_covers_region() {
        let t = StampfliTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(
            &t,
            "stampfli",
            &p,
            BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
        );
    }

    #[test]
    fn stampfli_rotated_scaled() {
        check_rotated(
            &StampfliTiling,
            "stampfli(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn stampfli_deterministic() {
        check_det(
            &StampfliTiling,
            "stampfli",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(all(test, feature = "fractalizing"))]
mod fractalizing_tests {
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::fractalizing::FractalizingTiling;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn fractalizing_covers_region() {
        let t = FractalizingTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        // level-0 seed plane (view diagonal >= 6):
        check_coverage(
            &t,
            "fractalizing(d0)",
            &p,
            BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
        );
        // level-1 flower-orbit plane (diagonal 5 -> depth 1):
        check_coverage(
            &t,
            "fractalizing(d1)",
            &p,
            BBox::new(v2(-2.0, -1.5), v2(2.0, 1.5)),
        );
    }

    #[test]
    fn fractalizing_rotated_scaled() {
        check_rotated(
            &FractalizingTiling,
            "fractalizing(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn fractalizing_deterministic() {
        check_det(
            &FractalizingTiling,
            "fractalizing",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(all(test, feature = "pythagorean"))]
mod pinwheel_tests {
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::pinwheel;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn pinwheel_covers_region() {
        let t = pinwheel::PinwheelTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "pinwheel", &p, BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)));
    }

    #[test]
    fn pinwheel_rotated_scaled() {
        check_rotated(
            &pinwheel::PinwheelTiling,
            "pinwheel(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn pinwheel_deterministic() {
        check_det(
            &pinwheel::PinwheelTiling,
            "pinwheel",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(all(test, feature = "pythagorean"))]
mod pythagorean_tests {
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::similarity_map;
    use crate::pythagorean::{PythagoreanTiling, RATIO_ID};
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::{Query, Tile, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn pythagorean_covers_region() {
        let t = PythagoreanTiling;
        let p = ParamSet::default_for(t.params());
        check_coverage(
            &t,
            "pythagorean",
            &p,
            BBox::new(v2(-4.0, -3.0), v2(12.0, 12.0)),
        );
    }

    #[test]
    fn pythagorean_rotated_scaled() {
        check_rotated(
            &PythagoreanTiling,
            "pythagorean(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pythagorean_deterministic() {
        check_det(
            &PythagoreanTiling,
            "pythagorean",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// The lattice depends on the ratio parameter, so coverage must hold
    /// across the whole parameter range, not just at the default —
    /// including both extremes, the r = 1 edge-to-edge case, and the
    /// default φ.
    #[test]
    fn pythagorean_ratio_sweep_covers() {
        for r in [0.2, 0.5, 1.0, 1.618_033_988_749_895, 2.0, 3.7, 10.0] {
            let t = PythagoreanTiling;
            let mut p = ParamSet::default_for(t.params());
            p.set(RATIO_ID, ParamValue::Float(r));
            check_coverage(
                &t,
                &format!("pythagorean(r={r})"),
                &p,
                BBox::new(v2(-3.0, -2.0), v2(9.0, 8.0)),
            );
        }
        // Rotated + scaled views must survive too: the culling inverts
        // the world lattice map, which only matches the cell-space map
        // when the rotation is 0.
        for r in [0.5, 1.1] {
            let t = PythagoreanTiling;
            let mut p = ParamSet::default_for(t.params());
            p.set(RATIO_ID, ParamValue::Float(r));
            p.set(crate::lattice::ROTATION_ID, ParamValue::Angle(30.0));
            p.set(crate::lattice::SCALE_ID, ParamValue::Float(0.7));
            check_coverage(
                &t,
                &format!("pythagorean(r={r},rot30,scale0.7)"),
                &p,
                BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            );
        }
    }

    /// The family is not a `LatticeTiling`, so it is not covered by
    /// `lattice_culling_matches_brute_force`; this is its own version:
    /// re-emit every tile over a fixed index window straight from the
    /// construction (no range math at all) and require the exact same
    /// tile set — geometry bit-for-bit, proto, and seed — as the
    /// generator's u-space range culling. Runs at a steep lattice
    /// rotation (r = 1.1 tilts the basis by 42°, where a cell-space bbox
    /// mistaken for the u-space one would cull wrongly) and at a mild
    /// one (r = 2.5).
    #[test]
    fn pythagorean_culling_matches_brute_force() {
        let t = PythagoreanTiling;
        for r in [1.1, 2.5] {
            let mut p = ParamSet::default_for(t.params());
            p.set(RATIO_ID, ParamValue::Float(r));

            // Brute force: every (i, j) in a fixed window, every cell. Mirrors the
            // generator's construction exactly (same reads, same op order) so
            // the comparison is bit-for-bit.
            let r = p.float(RATIO_ID, 1.618_033_988_749_895).clamp(0.2, 10.0);
            let (s1, s2) = if r >= 1.0 { (r, 1.0) } else { (1.0, r) };
            let map = similarity_map(&p);
            let a1 = map.transform(Vec2::new(s1, s2));
            let a2 = map.transform(Vec2::new(-s2, s1));
            let big = Polygon::from_slice(&[
                Vec2::new(0.0, 0.0),
                Vec2::new(s1, 0.0),
                Vec2::new(s1, s1),
                Vec2::new(0.0, s1),
            ])
            .transform(&map);
            let small = Polygon::from_slice(&[
                Vec2::new(s1, 0.0),
                Vec2::new(s1 + s2, 0.0),
                Vec2::new(s1 + s2, s2),
                Vec2::new(s1, s2),
            ])
            .transform(&map);
            let aabbs = [big.aabb(), small.aabb()];
            let region = BBox::new(v2(-3.0, -2.5), v2(5.0, 4.5));
            let window = 8i64;
            let mut brute: Vec<Tile> = Vec::new();
            for i in -window..=window {
                for j in -window..=window {
                    let tr = a1 * i as f64 + a2 * j as f64;
                    for (ci, poly) in [&big, &small].into_iter().enumerate() {
                        let bb = aabbs[ci];
                        let bb = BBox::new(bb.min + tr, bb.max + tr);
                        if bb.intersects(region) {
                            brute.push(Tile {
                                proto: ci as u8,
                                poly: poly.translated(tr),
                                seed: ci as u32,
                            });
                        }
                    }
                }
            }

            let got = t.generate(&p, &Query { region });
            assert!(got.message.is_none(), "r={r}: {:?}", got.message);

            fn key(tile: &Tile) -> (u8, u32, Vec<u64>) {
                let mut bits = Vec::with_capacity(tile.poly.pts.len() * 2);
                for pt in &tile.poly.pts {
                    bits.push(pt.x.to_bits());
                    bits.push(pt.y.to_bits());
                }
                (tile.proto, tile.seed, bits)
            }
            let mut a = brute;
            let mut b = got.tiles;
            a.sort_by_key(key);
            b.sort_by_key(key);
            assert_eq!(
                a.len(),
                b.len(),
                "r={r}: tile count {} vs {}",
                a.len(),
                b.len()
            );
            for (ta, tb) in a.iter().zip(b.iter()) {
                assert_eq!(key(ta), key(tb), "r={r}: tile mismatch");
            }
        }
    }
}

#[cfg(all(test, feature = "hirschhorn"))]
mod hirschhorn_tests {
    use super::tests::{check_coverage, check_deterministic, check_rotated};
    use crate::hirschhorn;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn hirschhorn_covers_region() {
        let t = hirschhorn::HirschhornTiling::new();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(
            &t,
            "hirschhorn",
            &p,
            BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
        );
    }

    #[test]
    fn hirschhorn_rotated_scaled() {
        check_rotated(
            &hirschhorn::HirschhornTiling::new(),
            "hirschhorn(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn hirschhorn_deterministic() {
        // scale 0.5 keeps the grown patch small enough for a fast check.
        let t = hirschhorn::HirschhornTiling::new();
        let mut p = ParamSet::default_for(t.params());
        p.set(
            crate::lattice::SCALE_ID,
            tess_core::params::ParamValue::Float(0.5),
        );
        check_deterministic(
            &t,
            "hirschhorn",
            &p,
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(all(test, feature = "zellige"))]
mod zellige_tests {
    use super::tests::{check_coverage, check_covers, check_det, check_rotated};
    use crate::zellige;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::ParamSet;
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn star_cross_covers_region() {
        check_covers(
            &zellige::StarCrossTiling,
            "{8/3}",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn star_cross_rotated_scaled() {
        check_rotated(
            &zellige::StarCrossTiling,
            "{8/3}(rot22.5,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            22.5,
        );
    }

    #[test]
    fn star_cross_deterministic() {
        check_det(
            &zellige::StarCrossTiling,
            "{8/3}",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// The star-radius parameter must produce a valid, gap-free tiling at
    /// the kissing default AND at gapped radii (the cross grows from a
    /// 16-gon to a 20-gon as the gap opens).
    #[test]
    fn star_cross_gap_family() {
        let t = zellige::StarCrossTiling;
        for radius in [0.45_f64, 0.3, 0.15] {
            let mut p = ParamSet::default_for(t.params());
            p.set(
                zellige::STAR_RADIUS_ID,
                tess_core::params::ParamValue::Float(radius),
            );
            check_coverage(
                &t,
                &format!("{{8/3}}(r={radius})"),
                &p,
                BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
            );
        }
    }
}

#[cfg(all(test, feature = "hexagram"))]
mod hexagram_tests {
    use super::tests::{check_coverage, check_covers, check_det, check_rotated};
    use crate::hexagram;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::{Query, Tiling};

    /// √3 (a local const: `std::f64::consts::SQRT_3` is unstable in
    /// this toolchain — the `more_float_constants` gate).
    const SQRT_3: f64 = 1.7320508075688773;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn hexagram_covers_region() {
        check_covers(
            &hexagram::HexagramTiling,
            "hexagram",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn hexagram_rotated_scaled() {
        check_rotated(
            &hexagram::HexagramTiling,
            "hexagram(rot22.5,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            22.5,
        );
    }

    #[test]
    fn hexagram_deterministic() {
        check_det(
            &hexagram::HexagramTiling,
            "hexagram",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// The star tip radius parameter must produce a valid, gap-free
    /// tiling at the kissing default, at the critical T = 1/3 (tips
    /// land on the Voronoi-vertex ring), at gapped radii, and at the
    /// wide-gap floor.
    #[test]
    fn hexagram_gap_family() {
        let t = hexagram::HexagramTiling;
        for radius in [0.5_f64, 1.0 / 3.0, 0.45, 0.3, 0.15] {
            let mut p = ParamSet::default_for(t.params());
            p.set(hexagram::STAR_RADIUS_ID, ParamValue::Float(radius));
            check_coverage(
                &t,
                &format!("hexagram(r={radius})"),
                &p,
                BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
            );
        }
    }

    /// p6m generator probe on the generated output: for each generator
    /// (60-degree about a star center, 120-degree about the two
    /// hexagon centers C1, C2, mirror y -> -y, mirror in the a2 line),
    /// the exact image of every interior emitted star must equal an
    /// emitted star as a vertex set. "Interior" means centroid in
    /// [-8, 8]^2 — the image of every interior lattice point stays
    /// well inside the 16x16 view, so its image star is always
    /// emitted, even though it may itself sit outside the interior
    /// box (e.g. rot60 of P(3,9) = (-3, 10.4)); the lookup therefore
    /// runs over ALL emitted stars. The per-cell emission is not
    /// generator-invariant as a tile set (a generator maps the cell
    /// basis {a1,a2} to a skewed GL(2,Z) basis), but the interstice
    /// is the complement of the star layer, so preserving the star
    /// layer setwise preserves the whole tiling.
    #[test]
    fn hexagram_group_symmetry() {
        fn rot60(v: Vec2) -> Vec2 {
            // 60-degree rotation: [[1/2, -sqrt3/2], [sqrt3/2, 1/2]]
            v2(0.5 * v.x - 0.5 * SQRT_3 * v.y, 0.5 * SQRT_3 * v.x + 0.5 * v.y)
        }
        fn rot120(v: Vec2) -> Vec2 {
            // 120-degree rotation: [[-1/2, -sqrt3/2], [sqrt3/2, -1/2]]
            v2(-0.5 * v.x - 0.5 * SQRT_3 * v.y, 0.5 * SQRT_3 * v.x - 0.5 * v.y)
        }

        for radius in [0.5_f64, 0.3] {
            let tiling = hexagram::HexagramTiling;
            let mut p = ParamSet::default_for(tiling.params());
            p.set(hexagram::STAR_RADIUS_ID, ParamValue::Float(radius));
            let region = BBox::new(v2(-16.0, -16.0), v2(16.0, 16.0));
            let r = tiling.generate(&p, &Query { region });
            assert!(
                r.message.is_none(),
                "hexagram(r={radius}): {msg}",
                msg = r.message.unwrap_or("")
            );

            let centroid = |pts: &[Vec2]| {
                let c = pts.iter().fold(Vec2::ZERO, |acc, v| acc + *v);
                c / pts.len() as f64
            };
            // All emitted stars (proto 1), with centroids; the probe
            // runs over the interior subset (centroid in [±8]²) but
            // matches images against every emitted star.
            let stars: Vec<Vec<Vec2>> = r
                .tiles
                .iter()
                .filter(|tl| tl.proto == 1)
                .map(|tl| tl.poly.pts.clone())
                .collect();
            let centroids: Vec<Vec2> = stars.iter().map(|pts| centroid(pts)).collect();
            let interior: Vec<usize> = centroids
                .iter()
                .enumerate()
                .filter(|(_, c)| c.x.abs() < 8.0 && c.y.abs() < 8.0)
                .map(|(i, _)| i)
                .collect();
            assert!(!interior.is_empty(), "hexagram(r={radius}): no interior stars");

            let gens: [Box<dyn Fn(Vec2) -> Vec2>; 5] = [
                Box::new(rot60),
                Box::new(move |v: Vec2| {
                    let c1 = v2(0.5, SQRT_3 / 6.0);
                    c1 + rot120(v - c1)
                }),
                Box::new(move |v: Vec2| {
                    let c2 = v2(1.0, SQRT_3 / 3.0);
                    c2 + rot120(v - c2)
                }),
                Box::new(|v: Vec2| v2(v.x, -v.y)),
                Box::new(|v: Vec2| {
                    // reflection in the a2 line (60-degree axis)
                    v2(
                        -0.5 * v.x + 0.5 * SQRT_3 * v.y,
                        0.5 * SQRT_3 * v.x + 0.5 * v.y,
                    )
                }),
            ];
            for (gi, g) in gens.iter().enumerate() {
                for &si in &interior {
                    let s = &stars[si];
                    let img: Vec<Vec2> = s.iter().map(|v| g(*v)).collect();
                    let ic = centroid(&img);
                    let mut found = false;
                    for (oi, o) in stars.iter().enumerate() {
                        if (centroids[oi] - ic).length() > 1e-6 {
                            continue;
                        }
                        let mut ok = o.len() == img.len();
                        for v in &img {
                            if !o.iter().any(|w| (*v - *w).length() < 1e-9) {
                                ok = false;
                                break;
                            }
                        }
                        assert!(
                            ok,
                            "hexagram(r={radius}): generator {gi} does not map star to an emitted star"
                        );
                        found = true;
                        break;
                    }
                    assert!(
                        found,
                        "hexagram(r={radius}): generator {gi} image star not in emitted set"
                    );
                }
            }
        }
    }
}

#[cfg(test)]
#[cfg(feature = "cairo")]
mod cairo_tests {
    use super::tests::{check_covers, check_det, check_rotated};
    use crate::cairo;
    use tess_core::geometry::{BBox, Polygon, Vec2};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn cairo_pentagonal_covers_region() {
        check_covers(
            &cairo::cairo_pentagonal(),
            "cairo-pentagonal",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn cairo_pentagonal_rotated_scaled() {
        check_rotated(
            &cairo::cairo_pentagonal(),
            "cairo-pentagonal(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn cairo_pentagonal_deterministic() {
        check_det(
            &cairo::cairo_pentagonal(),
            "cairo-pentagonal",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// Interior angles (degrees) at the five vertices of a cell pentagon.
    fn interior_angles(poly: &Polygon) -> [f64; 5] {
        let v = &poly.pts;
        let mut a = [0.0; 5];
        for i in 0..5 {
            let u = v[(i + 4) % 5] - v[i];
            let w = v[(i + 1) % 5] - v[i];
            a[i] = (u.dot(w) / (u.length() * w.length()))
                .clamp(-1.0, 1.0)
                .acos()
                .to_degrees();
        }
        a
    }

    /// Squared lengths of the five edges of a cell pentagon.
    fn squared_sides(poly: &Polygon) -> [f64; 5] {
        let v = &poly.pts;
        let mut s = [0.0; 5];
        for i in 0..5 {
            let e = v[(i + 1) % 5] - v[i];
            s[i] = e.dot(e);
        }
        s
    }

    /// M101 pentagon-census audit (PROPOSED.md §5): the cairo cell
    /// pentagons are type 4 in the repo lettering — all eight
    /// congruent (angles {90° ×2, 120° ×3}; squared sides
    /// {(2+√3)/6 ×4, 1/3}; per-pentagon area (2 + √3)/4, exact in
    /// Q(√3)) — and the first cell satisfies the type-4 conditions at
    /// ccw shift k=4 (A = v4, B = v0, C = v1, D = v2, E = v3):
    /// C = E = 90°, b = c, d = e (repo letters shift the Wikipedia
    /// letters by one: W-angle X = our-(X + 1 mod 5), side a joins
    /// A–B).
    #[test]
    fn cairo_pentagonal_is_pentagon_type_4() {
        let t = cairo::cairo_pentagonal();
        let long2 = (2.0 + 3.0f64.sqrt()) / 6.0; // (2 + √3)/6
        let area = (2.0 + 3.0f64.sqrt()) / 4.0; // (2 + √3)/4
        for cell in &t.cells {
            let mut ang: Vec<f64> = interior_angles(&cell.poly).to_vec();
            ang.sort_by(f64::total_cmp);
            let want = [90.0, 90.0, 120.0, 120.0, 120.0];
            assert!(
                ang.iter().zip(&want).all(|(x, y)| (x - y).abs() < 1e-9),
                "angles {ang:?}"
            );
            let mut s2: Vec<f64> = squared_sides(&cell.poly).to_vec();
            s2.sort_by(f64::total_cmp);
            let want: Vec<f64> =
                vec![1.0 / 3.0, long2, long2, long2, long2];
            assert!(
                s2.iter().zip(&want).all(|(x, y)| (x - y).abs() < 1e-12),
                "squared sides {s2:?}"
            );
            assert!(
                (cell.poly.area() - area).abs() < 1e-12,
                "area {} != (2 + √3)/4",
                cell.poly.area()
            );
        }
        // Type-4 lettering on the first cell, ccw shift k=4.
        let poly = &t.cells[0].poly;
        let ang = interior_angles(poly);
        assert!((ang[1] - 90.0).abs() < 1e-9, "C = v1: {}", ang[1]);
        assert!((ang[3] - 90.0).abs() < 1e-9, "E = v3: {}", ang[3]);
        let v = &poly.pts;
        let vtx = |i: usize| v[(i + 4) % 5]; // letter i (0 = A .. 4 = E)
        let side2 = |i: usize, j: usize| {
            let d = vtx(i) - vtx(j);
            d.dot(d)
        };
        assert!(
            (side2(1, 2) - side2(2, 3)).abs() < 1e-12,
            "b = c: {} vs {}",
            side2(1, 2),
            side2(2, 3)
        );
        assert!(
            (side2(3, 4) - side2(4, 0)).abs() < 1e-12,
            "d = e: {} vs {}",
            side2(3, 4),
            side2(4, 0)
        );
    }

}

#[cfg(test)]
#[cfg(feature = "girih")]
mod girih_tests {
    use super::tests::{check_covers, check_det, check_rotated};
    use crate::girih;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::ParamSet;
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn girih_shesh_band_covers_region() {
        check_covers(
            &girih::girih_shesh_band(),
            "girih-shesh-band",
            BBox::new(v2(-8.0, -6.0), v2(16.0, 14.0)),
        );
    }

    #[test]
    fn girih_shesh_band_rotated_scaled() {
        check_rotated(
            &girih::girih_shesh_band(),
            "girih-shesh-band(rot30,scale0.7)",
            BBox::new(v2(-12.0, -12.0), v2(12.0, 12.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn girih_shesh_band_deterministic() {
        check_det(
            &girih::girih_shesh_band(),
            "girih-shesh-band",
            BBox::new(v2(-10.0, -10.0), v2(10.0, 10.0)),
        );
    }

    /// M112: single-tile-per-cell, single orbit — the shipped seed is
    /// the decorative `SeedPattern::Checker` lattice parity, so a
    /// bounded view must show both parities (seeds stay in {0, 1}).
    #[test]
    fn girih_shesh_band_seed_is_decorative_checkerboard() {
        let t = girih::girih_shesh_band();
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region: BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let mut seen = [false; 2];
        for tile in &r.tiles {
            assert!(tile.seed < 2, "checkerboard seed must stay in {{0, 1}}");
            seen[tile.seed as usize] = true;
        }
        assert!(seen[0] && seen[1], "both lattice-parity classes must appear");
    }
}

#[cfg(test)]
#[cfg(feature = "girih_sc")]
mod girih_sc_tests {
    use super::tests::{check_covers, check_det, check_rotated};
    use crate::girih_sc;
    use tess_core::geometry::{BBox, Vec2};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn girih_star_cross_covers_region() {
        check_covers(
            &girih_sc::girih_star_cross(),
            "girih-star-cross",
            BBox::new(v2(-30.0, -35.0), v2(85.0, 55.0)),
        );
    }

    #[test]
    fn girih_star_cross_rotated_scaled() {
        check_rotated(
            &girih_sc::girih_star_cross(),
            "girih-star-cross(rot30,scale0.7)",
            BBox::new(v2(-20.0, -20.0), v2(20.0, 20.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn girih_star_cross_deterministic() {
        check_det(
            &girih_sc::girih_star_cross(),
            "girih-star-cross",
            BBox::new(v2(-15.0, -15.0), v2(40.0, 30.0)),
        );
    }
}

#[cfg(test)]
#[cfg(feature = "khatam_girih")]
mod khatam_girih_tests {
    use super::tests::{check_covers, check_det, check_rotated};
    use crate::khatam_girih;
    use tess_core::geometry::{BBox, Vec2};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn khatam_girih_covers_region() {
        check_covers(
            &khatam_girih::khatam_girih(),
            "khatam-girih",
            BBox::new(v2(-2.5, -1.5), v2(3.5, 2.5)),
        );
    }

    #[test]
    fn khatam_girih_rotated_scaled() {
        check_rotated(
            &khatam_girih::khatam_girih(),
            "khatam-girih(rot30,scale0.7)",
            BBox::new(v2(-3.0, -3.0), v2(3.0, 3.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn khatam_girih_deterministic() {
        check_det(
            &khatam_girih::khatam_girih(),
            "khatam-girih",
            BBox::new(v2(-2.0, -2.0), v2(3.0, 3.0)),
        );
    }
}

#[cfg(test)]
#[cfg(feature = "laves")]
mod laves_tests {
    use super::tests::{check_covers, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::laves;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::ParamSet;
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn tetrakis_square_covers_region() {
        check_covers(
            &laves::tetrakis_square(),
            "tetrakis-square",
            BBox::new(v2(-4.0, -3.0), v2(12.0, 12.0)),
        );
    }

    #[test]
    fn tetrakis_square_rotated_scaled() {
        check_rotated(
            &laves::tetrakis_square(),
            "tetrakis-square(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn tetrakis_square_deterministic() {
        check_det(
            &laves::tetrakis_square(),
            "tetrakis-square",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn rhombille_covers_region() {
        check_covers(
            &laves::rhombille(),
            "rhombille",
            BBox::new(v2(-4.0, -3.0), v2(12.0, 12.0)),
        );
    }

    #[test]
    fn rhombille_rotated_scaled() {
        check_rotated(
            &laves::rhombille(),
            "rhombille(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn rhombille_deterministic() {
        check_det(
            &laves::rhombille(),
            "rhombille",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn triakis_triangular_covers_region() {
        check_covers(
            &laves::triakis_triangular(),
            "triakis-triangular",
            BBox::new(v2(-4.0, -3.0), v2(12.0, 12.0)),
        );
    }

    #[test]
    fn triakis_triangular_rotated_scaled() {
        check_rotated(
            &laves::triakis_triangular(),
            "triakis-triangular(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn triakis_triangular_deterministic() {
        check_det(
            &laves::triakis_triangular(),
            "triakis-triangular",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn deltoidal_trihexagonal_covers_region() {
        check_covers(
            &laves::deltoidal_trihexagonal(),
            "deltoidal-trihexagonal",
            BBox::new(v2(-4.0, -3.0), v2(12.0, 12.0)),
        );
    }

    #[test]
    fn deltoidal_trihexagonal_rotated_scaled() {
        check_rotated(
            &laves::deltoidal_trihexagonal(),
            "deltoidal-trihexagonal(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn deltoidal_trihexagonal_deterministic() {
        check_det(
            &laves::deltoidal_trihexagonal(),
            "deltoidal-trihexagonal",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn kisrhombille_covers_region() {
        check_covers(
            &laves::kisrhombille(),
            "kisrhombille",
            BBox::new(v2(-6.0, -4.0), v2(16.0, 14.0)),
        );
    }

    #[test]
    fn kisrhombille_rotated_scaled() {
        check_rotated(
            &laves::kisrhombille(),
            "kisrhombille(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(8.0, 8.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kisrhombille_deterministic() {
        check_det(
            &laves::kisrhombille(),
            "kisrhombille",
            BBox::new(v2(-4.0, -4.0), v2(12.0, 12.0)),
        );
    }

    #[test]
    fn prismatic_pentagonal_covers_region() {
        check_covers(
            &laves::prismatic_pentagonal(),
            "prismatic-pentagonal",
            BBox::new(v2(-4.0, -3.0), v2(12.0, 12.0)),
        );
    }

    #[test]
    fn prismatic_pentagonal_rotated_scaled() {
        check_rotated(
            &laves::prismatic_pentagonal(),
            "prismatic-pentagonal(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn prismatic_pentagonal_deterministic() {
        check_det(
            &laves::prismatic_pentagonal(),
            "prismatic-pentagonal",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn floret_pentagonal_covers_region() {
        check_covers(
            &laves::floret_pentagonal(),
            "floret-pentagonal",
            BBox::new(v2(-6.0, -5.0), v2(14.0, 13.0)),
        );
    }

    #[test]
    fn floret_pentagonal_rotated_scaled() {
        check_rotated(
            &laves::floret_pentagonal(),
            "floret-pentagonal(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(8.0, 8.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn floret_pentagonal_deterministic() {
        check_det(
            &laves::floret_pentagonal(),
            "floret-pentagonal",
            BBox::new(v2(-5.0, -5.0), v2(11.0, 11.0)),
        );
    }

    /// Interior angles (degrees) at the five vertices of a cell pentagon.
    fn interior_angles(poly: &Polygon) -> [f64; 5] {
        let v = &poly.pts;
        let mut a = [0.0; 5];
        for i in 0..5 {
            let u = v[(i + 4) % 5] - v[i];
            let w = v[(i + 1) % 5] - v[i];
            a[i] = (u.dot(w) / (u.length() * w.length()))
                .clamp(-1.0, 1.0)
                .acos()
                .to_degrees();
        }
        a
    }

    /// Squared lengths of the five edges of a cell pentagon.
    fn squared_sides(poly: &Polygon) -> [f64; 5] {
        let v = &poly.pts;
        let mut s = [0.0; 5];
        for i in 0..5 {
            let e = v[(i + 1) % 5] - v[i];
            s[i] = e.dot(e);
        }
        s
    }

    /// M101 pentagon-census audit (PROPOSED.md §5): both prismatic
    /// cell pentagons are type 1 in the repo lettering — C + D = 180°
    /// and A + B + E = 360° (repo letters shift the Wikipedia letters
    /// by one: W-angle X = our-(X + 1 mod 5), side a joins A–B) —
    /// checked at their cyclic shifts: the up pentagon at ccw k=1
    /// (A = v1, B = v2, C = v3, D = v4, E = v0), the down pentagon at
    /// k=3 (A = v3, B = v4, C = v0, D = v1, E = v2). Angles are
    /// exactly 90°/120°; squared sides {1/3 ×2, (2+√3)/6 ×2, 1} and
    /// the per-pentagon area (1 + √3/2)/2 = 1/2 + √3/4 are exact in
    /// Q(√3).
    #[test]
    fn prismatic_pentagonal_is_pentagon_type_1() {
        let t = laves::prismatic_pentagonal();
        let long2 = (2.0 + 3.0f64.sqrt()) / 6.0; // (2 + √3)/6
        let area = (1.0 + 3.0f64.sqrt() / 2.0) / 2.0; // 1/2 + √3/4
        let cases: [(usize, &Polygon); 2] =
            [(1, &t.cells[0].poly), (3, &t.cells[1].poly)];
        for (k, poly) in cases {
            let ang = interior_angles(poly);
            let at = |i: usize| ang[(i + k) % 5]; // letter i (0 = A .. 4 = E)
            let c = at(2);
            let d = at(3);
            assert!(
                (c + d - 180.0).abs() < 1e-9,
                "k={k}: C + D = {c} + {d} != 180"
            );
            assert!(
                (at(0) + at(1) + at(4) - 360.0).abs() < 1e-9,
                "k={k}: A + B + E = {} + {} + {} != 360",
                at(0),
                at(1),
                at(4)
            );
            for x in &ang {
                assert!(
                    (x - 90.0).abs() < 1e-9 || (x - 120.0).abs() < 1e-9,
                    "k={k}: angle {x} is not exactly 90°/120°"
                );
            }
            let mut s2: Vec<f64> = squared_sides(poly).to_vec();
            s2.sort_by(f64::total_cmp);
            let want: Vec<f64> = vec![1.0 / 3.0, 1.0 / 3.0, long2, long2, 1.0];
            assert!(
                s2.iter().zip(&want).all(|(x, y)| (x - y).abs() < 1e-12),
                "k={k}: squared sides {s2:?}"
            );
            assert!(
                (poly.area() - area).abs() < 1e-12,
                "k={k}: area {} != 1/2 + √3/4",
                poly.area()
            );
        }
    }

    /// M101 pentagon-census audit (PROPOSED.md §5): the floret cell
    /// pentagons are type 5 in the repo lettering — all six share the
    /// shape (angles {60°, 120° ×4}; squared sides {1/3 ×3, 4/3 ×2};
    /// per-pentagon area 7√3/12, exact in Q(√3)), and the first cell
    /// satisfies the type-5 conditions at ccw shift k=3 (A = v3,
    /// B = v4, C = v0, D = v1, E = v2): B = 60°, E = 120°, a = b,
    /// d = e (repo letters shift the Wikipedia letters by one; side
    /// a joins A–B).
    #[test]
    fn floret_pentagonal_is_pentagon_type_5() {
        let t = laves::floret_pentagonal();
        let area = 7.0 * 3.0f64.sqrt() / 12.0; // 7√3/12
        for cell in &t.cells {
            let mut ang: Vec<f64> = interior_angles(&cell.poly).to_vec();
            ang.sort_by(f64::total_cmp);
            let want = [60.0, 120.0, 120.0, 120.0, 120.0];
            assert!(
                ang.iter().zip(&want).all(|(x, y)| (x - y).abs() < 1e-9),
                "angles {ang:?}"
            );
            let mut s2: Vec<f64> = squared_sides(&cell.poly).to_vec();
            s2.sort_by(f64::total_cmp);
            let want: Vec<f64> =
                vec![1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0, 4.0 / 3.0, 4.0 / 3.0];
            assert!(
                s2.iter().zip(&want).all(|(x, y)| (x - y).abs() < 1e-12),
                "squared sides {s2:?}"
            );
            assert!(
                (cell.poly.area() - area).abs() < 1e-12,
                "area {} != 7√3/12",
                cell.poly.area()
            );
        }
        // Type-5 lettering on the first cell, ccw shift k=3.
        let poly = &t.cells[0].poly;
        let ang = interior_angles(poly);
        assert!((ang[4] - 60.0).abs() < 1e-9, "B = v4: {}", ang[4]);
        assert!((ang[2] - 120.0).abs() < 1e-9, "E = v2: {}", ang[2]);
        let v = &poly.pts;
        let vtx = |i: usize| v[(i + 3) % 5]; // letter i (0 = A .. 4 = E)
        let side2 = |i: usize, j: usize| {
            let d = vtx(i) - vtx(j);
            d.dot(d)
        };
        assert!(
            (side2(0, 1) - side2(1, 2)).abs() < 1e-12,
            "a = b: {} vs {}",
            side2(0, 1),
            side2(1, 2)
        );
        assert!(
            (side2(3, 4) - side2(4, 0)).abs() < 1e-12,
            "d = e: {} vs {}",
            side2(3, 4),
            side2(4, 0)
        );
    }

    /// The Laves tables claim "pattern" mode is a proper coloring of the
    /// dual adjacency graph (the Python pre-verification searched for
    /// exactly that). Re-check it on a generated patch: every pair of
    /// tiles sharing an edge must differ in `seed % 8`.
    #[test]
    fn laves_pattern_mode_is_proper_coloring() {
        use std::collections::HashMap;
        let tilings: Vec<(&str, LatticeTiling)> = vec![
            ("tetrakis-square", laves::tetrakis_square()),
            ("rhombille", laves::rhombille()),
            ("triakis-triangular", laves::triakis_triangular()),
            ("deltoidal-trihexagonal", laves::deltoidal_trihexagonal()),
            ("kisrhombille", laves::kisrhombille()),
            ("prismatic-pentagonal", laves::prismatic_pentagonal()),
            ("floret-pentagonal", laves::floret_pentagonal()),
        ];
        let region = BBox::new(v2(-5.0, -4.0), v2(7.0, 6.0));
        for (name, lt) in &tilings {
            let p = ParamSet::default_for(lt.params());
            let r = lt.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{name}: {:?}", r.message);
            // Key each undirected edge by rounded endpoints (1e-6 grid,
            // matching the AGENTS.md edge-parity convention); shared edges
            // between lattice translates agree to ~1 ulp.
            let key = |a: Vec2, b: Vec2| {
                let mut k = [
                    (a.x * 1e6).round() as i64,
                    (a.y * 1e6).round() as i64,
                    (b.x * 1e6).round() as i64,
                    (b.y * 1e6).round() as i64,
                ];
                if (k[0], k[1]) > (k[2], k[3]) {
                    k.swap(0, 2);
                    k.swap(1, 3);
                }
                k
            };
            let mut edges: HashMap<[i64; 4], Vec<u32>> = HashMap::new();
            for t in &r.tiles {
                let n = t.poly.pts.len();
                for w in 0..n {
                    let a = t.poly.pts[w];
                    let b = t.poly.pts[(w + 1) % n];
                    edges.entry(key(a, b)).or_default().push(t.seed % 8);
                }
            }
            for (k, seeds) in &edges {
                assert!(
                    seeds.len() <= 2,
                    "{name}: edge {k:?} shared by {} tiles (not edge-to-edge?)",
                    seeds.len()
                );
                if seeds.len() == 2 {
                    assert!(
                        seeds[0] != seeds[1],
                        "{name}: adjacent tiles share pattern color {} (edge {k:?})",
                        seeds[0]
                    );
                }
            }
        }
    }
}

#[cfg(test)]
#[cfg(feature = "girih_qc")]
mod girih_qc_tests {
    use super::tests::{check_coverage, check_deterministic};
    use crate::girih_qc::GirihQuasiperiodicTiling;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Unit scale (the substitution geometry is defined at unit edges).
    fn params1() -> ParamSet {
        let t = GirihQuasiperiodicTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        p
    }

    /// Standard coverage: the depth-2 lineage covers a disk of radius
    /// ≈ 111.3 around the seed decagon, so a ±8 view is comfortably
    /// inside the frozen core.  `check_coverage` verifies culling,
    /// point sampling and exact clipped-area equality (no gaps, no
    /// overlaps).
    #[test]
    fn girih_qc_covers_region() {
        let t = GirihQuasiperiodicTiling;
        check_coverage(
            &t,
            "girih-qc",
            &params1(),
            BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
        );
    }

    /// Seeded coverage generalizes across anchor regions: off-origin
    /// views served at deeper depths (3 and 4) are covered exactly too.
    #[test]
    fn girih_qc_covers_distant_views() {
        let t = GirihQuasiperiodicTiling;
        let p = params1();
        for (cx, cy, w, h) in [
            (47.3, -91.2, 20.0, 20.0),
            (-300.0, 220.0, 25.0, 25.0),
            (500.0, 300.0, 25.0, 25.0),
        ] {
            check_coverage(
                &t,
                "girih-qc",
                &p,
                BBox::new(v2(cx - w, cy - h), v2(cx + w, cy + h)),
            );
        }
    }

    /// Coverage must survive a non-axis-aligned scale + rotation.
    #[test]
    fn girih_qc_rotated_scaled() {
        let t = GirihQuasiperiodicTiling;
        let mut p = params1();
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(0.7));
        p.set(crate::lattice::ROTATION_ID, ParamValue::Angle(30.0));
        check_coverage(
            &t,
            "girih-qc(rot30,scale0.7)",
            &p,
            BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
        );
    }

    #[test]
    fn girih_qc_deterministic() {
        let t = GirihQuasiperiodicTiling;
        check_deterministic(
            &t,
            "girih-qc",
            &params1(),
            BBox::new(v2(-10.0, -10.0), v2(10.0, 10.0)),
        );
    }
}

#[cfg(all(test, feature = "windmill"))]
mod windmill_tests {
    use super::tests::{check_covers, check_det, check_rotated};
    use crate::windmill;
    use std::collections::{BTreeSet, HashSet};
    use tess_core::geometry::{BBox, Vec2};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn windmill_covers_region() {
        check_covers(
            &windmill::spec(),
            "windmill",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
        );
    }

    #[test]
    fn windmill_rotated_scaled() {
        check_rotated(
            &windmill::spec(),
            "windmill(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn windmill_deterministic() {
        check_det(
            &windmill::spec(),
            "windmill",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
        );
    }

    /// The vertex figures, pinned exactly. Emit the 3×3-block patch
    /// [−6, 6]² straight from the table data (blocks i, j ∈ [−2, 1]² —
    /// no culling) and census every integer vertex of [−3, 3]²:
    /// * 9 lattice vertices (residue (0,0) mod 3) with figure 90⁴;
    /// * 24 T-junctions — 6 of each residue class
    ///   {(0,1), (0,2), (1,0), (2,0)} mod 3 — with figure
    ///   180·90·arctan(1/2)·arctan(2): the 180° is the straight angle
    ///   of the length-2 leg the junction splits (the non-edge-to-edge
    ///   mid-side T-junction, as in `chair.rs`);
    /// * no vertex at the other four residue classes (they lie in tile
    ///   interiors); every occupied figure sums to 360°.
    #[test]
    fn windmill_vertex_figures() {
        const PI: f64 = std::f64::consts::PI;
        const TAU: f64 = std::f64::consts::TAU;
        let spec = windmill::spec();
        // Blocks [−2, 1]²: the patch [−6, 6]², so every vertex of
        // [−3, 3]² is fully surrounded by emitted tiles.
        let mut tiles: Vec<Vec<[i64; 2]>> = Vec::new();
        for i in -2..=1 {
            for j in -2..=1 {
                for c in &spec.cells {
                    tiles.push(
                        c.poly
                            .pts
                            .iter()
                            .map(|p| [p.x.round() as i64 + 3 * i, p.y.round() as i64 + 3 * j])
                            .collect(),
                    );
                }
            }
        }
        assert_eq!(tiles.len(), 80, "5 tiles × 16 blocks");

        /// Interior angle (radians) at the corner `k` of the CCW
        /// polygon `t`, where `p` is the vertex.
        fn corner_angle(t: &[[i64; 2]], k: usize, p: [i64; 2]) -> f64 {
            let n = t.len();
            let a = t[(k + n - 1) % n];
            let b = t[(k + 1) % n];
            let (ux, uy) = (a[0] - p[0], a[1] - p[1]);
            let (vx, vy) = (b[0] - p[0], b[1] - p[1]);
            f64::atan2(
                (ux * vy - uy * vx).unsigned_abs() as f64,
                (ux * vx + uy * vy) as f64,
            )
        }

        /// `p` in the relative interior of the open segment (a, b)?
        /// Collinear + inside the closed box + not an endpoint (the box
        /// test must be non-strict: a vertical/horizontal segment has
        /// its constant coordinate equal at `p`).
        fn on_open_segment(p: [i64; 2], a: [i64; 2], b: [i64; 2]) -> bool {
            if p == a || p == b {
                return false;
            }
            let cross = (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]);
            if cross != 0 {
                return false;
            }
            p[0] >= a[0].min(b[0])
                && p[0] <= a[0].max(b[0])
                && p[1] >= a[1].min(b[1])
                && p[1] <= a[1].max(b[1])
        }

        // The T-junction figure: the straight angle of the split leg,
        // the square's 90° corner, and the two 1–2–√5 acute angles.
        let t_expect = [f64::atan(0.5), f64::atan(2.0), PI / 2.0, PI];
        let assert_figure = |got: &[f64], want: &[f64], p: [i64; 2], label: &str| {
            assert!(
                got.len() == want.len()
                    && got.iter().zip(want).all(|(g, e)| (g - e).abs() < 1e-9),
                "{label} at {p:?}: {got:?} != {want:?}"
            );
        };

        let mut a_count = 0usize;
        let mut t_counts = [0usize; 4];
        for x in -3..=3 {
            for y in -3..=3 {
                let p = [x, y];
                let mut angles: Vec<f64> = Vec::new();
                for t in &tiles {
                    let n = t.len();
                    let mut corner = false;
                    for k in 0..n {
                        if t[k] == p {
                            angles.push(corner_angle(t, k, p));
                            corner = true;
                            break;
                        }
                    }
                    if !corner {
                        // A T-junction sits on the open midpoint of a
                        // length-2 leg: that tile contributes a
                        // straight angle.
                        for k in 0..n {
                            if on_open_segment(p, t[k], t[(k + 1) % n]) {
                                angles.push(PI);
                                break;
                            }
                        }
                    }
                }
                angles.sort_by(|u, v| u.partial_cmp(v).expect("angles are finite"));
                let rem = |v: i64| v.rem_euclid(3);
                match (rem(x), rem(y)) {
                    (0, 0) => {
                        a_count += 1;
                        assert_figure(&angles, &[PI / 2.0; 4], p, "lattice-vertex figure (90⁴)");
                    }
                    (0, 1) | (0, 2) | (1, 0) | (2, 0) => {
                        let idx = match (rem(x), rem(y)) {
                            (0, 1) => 0,
                            (0, 2) => 1,
                            (1, 0) => 2,
                            _ => 3,
                        };
                        t_counts[idx] += 1;
                        assert_figure(
                            &angles,
                            &t_expect,
                            p,
                            "T-junction figure (180·90·arctan(1/2)·arctan(2))",
                        );
                    }
                    _ => {
                        assert!(
                            angles.is_empty(),
                            "vertex figure at the tile-interior point {p:?}: {angles:?}"
                        );
                    }
                }
                if !angles.is_empty() {
                    let sum: f64 = angles.iter().sum();
                    assert!(
                        (sum - TAU).abs() < 1e-9,
                        "vertex figure at {p:?} sums to {sum} rad, not 2π"
                    );
                }
            }
        }
        assert_eq!(a_count, 9, "the 90⁴ orbit is 3×3 in [−3, 3]²");
        let class_labels = ["(0,1)", "(0,2)", "(1,0)", "(2,0)"];
        for (i, n) in t_counts.iter().enumerate() {
            assert_eq!(
                *n,
                6,
                "T-junction orbit {i} (residue class {} mod 3) has 6 vertices in [−3, 3]²",
                class_labels[i]
            );
        }
    }

    /// The wallpaper group is p4, pinned in both directions. The
    /// positive side: 4-fold centres at the motif centres (3i+1.5,
    /// 3j+1.5) *and* the lattice points (3i, 3j), 2-folds at all four
    /// centre classes mod 3, translations 3Z²; the negative side (the
    /// distinctive assertions): no mirrors, no glides, no short
    /// translations — p4m/p4g would add mirrors or glides, p2 would
    /// lack the 4-folds. Homochirality (every triangle the same
    /// chirality sign) is what rules out p4g at the source.
    #[test]
    fn windmill_symmetry_p4() {
        // Window: all tiles whose vertices lie in [−9, 9]² (blocks
        // i, j ∈ [−3, 2]²), emitted straight from the table data.
        let spec = windmill::spec();
        let mut tiles: Vec<Vec<[i64; 2]>> = Vec::new();
        for i in -3..=2 {
            for j in -3..=2 {
                for c in &spec.cells {
                    tiles.push(
                        c.poly
                            .pts
                            .iter()
                            .map(|p| [p.x.round() as i64 + 3 * i, p.y.round() as i64 + 3 * j])
                            .collect(),
                    );
                }
            }
        }
        let key = |t: &[[i64; 2]]| {
            let mut k = t.to_vec();
            k.sort_unstable();
            k
        };
        let set: HashSet<Vec<[i64; 2]>> = tiles.iter().map(|t| key(t)).collect();
        assert_eq!(tiles.len(), 180, "5 tiles × 36 blocks");
        assert_eq!(set.len(), 180, "no two tiles share a vertex set");

        /// Restricted probe: apply `g` and `g⁻¹` to every window tile.
        /// Images whose vertices all stay in the [−9, 9]² window must
        /// land on window tiles; images that leave the integer vertex
        /// lattice (half-integer offsets) prove non-symmetry, and
        /// images that leave the window are skipped (they are checked
        /// by other tiles' inverses, or are the candidate's business
        /// outside the window).
        fn probe<G: Fn([i64; 2]) -> [f64; 2], I: Fn([i64; 2]) -> [f64; 2]>(
            tiles: &[Vec<[i64; 2]>],
            set: &HashSet<Vec<[i64; 2]>>,
            g: G,
            gi: I,
        ) -> bool {
            let check_dir = |dir: &dyn Fn([i64; 2]) -> [f64; 2]| -> bool {
                for t in tiles {
                    let mut img: Vec<[i64; 2]> = Vec::with_capacity(t.len());
                    for &v in t {
                        let w = dir(v);
                        if (w[0].round() - w[0]).abs() > 1e-6 || (w[1].round() - w[1]).abs() > 1e-6 {
                            // Leaves Z²: the image can be no tile.
                            return false;
                        }
                        img.push([w[0].round() as i64, w[1].round() as i64]);
                    }
                    if !img.iter().all(|v| v[0].abs() <= 9 && v[1].abs() <= 9) {
                        continue; // image left the window
                    }
                    let mut k = img;
                    k.sort_unstable();
                    if !set.contains(&k) {
                        return false;
                    }
                }
                true
            };
            check_dir(&|p| g(p)) && check_dir(&|p| gi(p))
        }

        // --- positive: the p4 point group at every centre class ---
        for (cx, cy) in [
            (0.0, 0.0),
            (3.0, 0.0),
            (0.0, 3.0),
            (3.0, 3.0),
            (1.5, 1.5),
            (4.5, 1.5),
            (1.5, 4.5),
        ] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [cx - (p[1] as f64 - cy), cy + (p[0] as f64 - cx)]
            };
            let gi = |p: [i64; 2]| -> [f64; 2] {
                [cx + (p[1] as f64 - cy), cy - (p[0] as f64 - cx)]
            };
            assert!(
                probe(&tiles, &set, g, gi),
                "R90 about ({cx}, {cy}) must be a 4-fold symmetry"
            );
        }
        for (cx, cy) in [
            (0.0, 0.0),
            (1.5, 0.0),
            (0.0, 1.5),
            (1.5, 1.5),
            (3.0, 0.0),
            (0.0, 3.0),
        ] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [2.0 * cx - p[0] as f64, 2.0 * cy - p[1] as f64]
            };
            assert!(
                probe(&tiles, &set, g, g),
                "R180 about ({cx}, {cy}) must be a 2-fold symmetry"
            );
        }
        for (dx, dy) in [(3, 0), (0, 3)] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [p[0] as f64 + dx as f64, p[1] as f64 + dy as f64]
            };
            let gi = |p: [i64; 2]| -> [f64; 2] {
                [p[0] as f64 - dx as f64, p[1] as f64 - dy as f64]
            };
            assert!(
                probe(&tiles, &set, g, gi),
                "lattice translation ({dx}, {dy}) must be a symmetry"
            );
        }

        // --- negative: no 4-/2-fold centres off the four classes ---
        for (cx, cy) in [(1.5, 0.0), (0.5, 0.5)] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [cx - (p[1] as f64 - cy), cy + (p[0] as f64 - cx)]
            };
            let gi = |p: [i64; 2]| -> [f64; 2] {
                [cx + (p[1] as f64 - cy), cy - (p[0] as f64 - cx)]
            };
            assert!(
                !probe(&tiles, &set, g, gi),
                "R90 about ({cx}, {cy}) must NOT be a symmetry (no extra 4-fold centres)"
            );
        }
        {
            let (cx, cy) = (0.5, 0.5);
            let g = |p: [i64; 2]| -> [f64; 2] {
                [2.0 * cx - p[0] as f64, 2.0 * cy - p[1] as f64]
            };
            assert!(
                !probe(&tiles, &set, g, g),
                "R180 about (0.5, 0.5) must NOT be a symmetry (no off-class 2-folds)"
            );
        }

        // --- negative: no mirrors (the complete axis set is pinned by
        // the 4-fold centre set {0, 1.5} mod 3) ---
        for c in [0.0, 1.5] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [2.0 * c - p[0] as f64, p[1] as f64]
            };
            assert!(
                !probe(&tiles, &set, g, g),
                "the mirror line x = {c} must NOT be a symmetry (p4 has no mirrors)"
            );
        }
        for c in [0.0, 1.5] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [p[0] as f64, 2.0 * c - p[1] as f64]
            };
            assert!(
                !probe(&tiles, &set, g, g),
                "the mirror line y = {c} must NOT be a symmetry"
            );
        }
        {
            let g = |p: [i64; 2]| -> [f64; 2] { [p[1] as f64, p[0] as f64] };
            assert!(
                !probe(&tiles, &set, g, g),
                "the mirror line y = x must NOT be a symmetry"
            );
        }
        {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [p[1] as f64 - 1.5, p[0] as f64 + 1.5]
            };
            assert!(
                !probe(&tiles, &set, g, g),
                "the mirror line y = x + 1.5 must NOT be a symmetry"
            );
        }
        for c in [3.0, 4.5] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [c - p[1] as f64, c - p[0] as f64]
            };
            assert!(
                !probe(&tiles, &set, g, g),
                "the mirror line y = -x + {c} must NOT be a symmetry"
            );
        }

        // --- negative: no glides (each mirror axis with the half-
        // period along it; half-lattice vectors 3/2) ---
        for c in [0.0, 1.5] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [2.0 * c - p[0] as f64, p[1] as f64 + 1.5]
            };
            let gi = |p: [i64; 2]| -> [f64; 2] {
                [2.0 * c - p[0] as f64, p[1] as f64 - 1.5]
            };
            assert!(
                !probe(&tiles, &set, g, gi),
                "the glide on x = {c} (half-period (0, 1.5)) must NOT be a symmetry"
            );
        }
        for c in [0.0, 1.5] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [p[0] as f64 + 1.5, 2.0 * c - p[1] as f64]
            };
            let gi = |p: [i64; 2]| -> [f64; 2] {
                [p[0] as f64 - 1.5, 2.0 * c - p[1] as f64]
            };
            assert!(
                !probe(&tiles, &set, g, gi),
                "the glide on y = {c} (half-period (1.5, 0)) must NOT be a symmetry"
            );
        }
        {
            // y = x, half-period (1.5, 1.5)
            let g = |p: [i64; 2]| -> [f64; 2] {
                [p[1] as f64 + 1.5, p[0] as f64 + 1.5]
            };
            let gi = |p: [i64; 2]| -> [f64; 2] {
                [p[1] as f64 - 1.5, p[0] as f64 - 1.5]
            };
            assert!(
                !probe(&tiles, &set, g, gi),
                "the glide on y = x (half-period (1.5, 1.5)) must NOT be a symmetry"
            );
        }
        {
            // y = x + 1.5, half-period (1.5, 1.5): g(x, y) = (y, x + 3)
            let g = |p: [i64; 2]| -> [f64; 2] { [p[1] as f64, p[0] as f64 + 3.0] };
            let gi = |p: [i64; 2]| -> [f64; 2] { [p[1] as f64 - 3.0, p[0] as f64] };
            assert!(
                !probe(&tiles, &set, g, gi),
                "the glide on y = x + 1.5 (half-period (1.5, 1.5)) must NOT be a symmetry"
            );
        }
        for c in [3.0, 4.5] {
            // y = -x + c, half-period (1.5, -1.5)
            let g = |p: [i64; 2]| -> [f64; 2] {
                [c - p[1] as f64 + 1.5, c - p[0] as f64 - 1.5]
            };
            let gi = |p: [i64; 2]| -> [f64; 2] {
                [c - p[1] as f64 - 1.5, c - p[0] as f64 + 1.5]
            };
            assert!(
                !probe(&tiles, &set, g, gi),
                "the glide on y = -x + {c} (half-period (1.5, -1.5)) must NOT be a symmetry"
            );
        }

        // --- negative: no translations shorter than 3Z² ---
        for (dx, dy) in [(1, 0), (0, 1), (2, 0), (0, 2)] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [p[0] as f64 + dx as f64, p[1] as f64 + dy as f64]
            };
            let gi = |p: [i64; 2]| -> [f64; 2] {
                [p[0] as f64 - dx as f64, p[1] as f64 - dy as f64]
            };
            assert!(
                !probe(&tiles, &set, g, gi),
                "translation ({dx}, {dy}) must NOT be a symmetry (minimal translations are 3Z²)"
            );
        }

        // --- homochirality: every triangle the same chirality sign ---
        let signs: BTreeSet<i32> = tiles
            .iter()
            .filter(|t| t.len() == 3)
            .map(|t| {
                // The right-angle vertex: the two edge vectors dot to 0
                // (exact — integer coordinates).
                for k in 0..t.len() {
                    let a = t[(k + t.len() - 1) % t.len()];
                    let b = t[(k + 1) % t.len()];
                    let (ux, uy) = (a[0] - t[k][0], a[1] - t[k][1]);
                    let (vx, vy) = (b[0] - t[k][0], b[1] - t[k][1]);
                    if ux * vx + uy * vy == 0 {
                        // Cross of the legs in the CCW vertex order —
                        // one sign across the tiling (p4, not p4g).
                        return (ux * vy - uy * vx).signum() as i32;
                    }
                }
                panic!("no right angle in triangle {t:?}");
            })
            .collect();
        assert_eq!(
            signs.len(),
            1,
            "all 100 window triangles are homochiral (p4, not p4g): signs = {signs:?}"
        );
    }
}

/// Brute-force cross-check of [`crate::lattice::LatticeTiling`]'s
/// index-range culling, the one part of the generator that could silently
/// drop (or invent) tiles at the edges of the query region: the per-cell
/// offsets that pad the index ranges are measured in lattice coordinates
/// (cell vertices mapped through `u(p) = A⁻¹ p`), and getting that wrong
/// for a sheared or tall cell would produce gaps that sample-based
/// coverage tests can miss. This test re-emits every tile from the public
/// table data over a wide fixed index window — no range math at all — and
/// requires exactly the same tile set as `generate`.
/// Exact wallpaper-group probe for the paver data tables.
///
/// All paver geometry is integral, and every candidate isometry
/// `g = (M, v)` we want to test has `M` with {0, ±1} entries and `v` a
/// half-integer vector, so membership can be tested exactly in
/// *doubled* coordinates (world point `p` ↔ `P = 2p`). A candidate is
/// a symmetry iff `M` maps the lattice basis into the domino-origin
/// lattice `L`, and for every unit-cell domino the image domino's
/// origin `t` (the min corner of the image, i.e. `M·o + c0 + v` with
/// `c0` the min corner of `M([0,w]×[0,h])`) lands in that kind's
/// origin-coset set `O(kind)` — the rigorous coset check of the
/// Python spike. All coordinates below are doubled: the lattice basis,
/// the cell origins/extents (`kind` 0 = horizontal, 1 = vertical),
/// and the translations `V = 2v`.
#[cfg(all(test, feature = "pavers"))]
fn pavers_exact_is_sym(
    m: [i64; 4], // M = [[m[0], m[1]], [m[2], m[3]]]
    v: [i64; 2], // doubled translation V = 2v
    basis: &[(i64, i64); 2],
    cells: &[(i64, i64, i64, i64, u8)],
    in_lattice: impl Fn(i64, i64) -> bool,
    in_origin: impl Fn(u8, i64, i64) -> bool,
) -> bool {
    let mv = |p: (i64, i64)| (m[0] * p.0 + m[1] * p.1, m[2] * p.0 + m[3] * p.1);
    // M must map the lattice to itself; all candidates have det ±1, so
    // checking the basis vectors suffices.
    for &(bx, by) in basis {
        let (x, y) = mv((bx, by));
        if !in_lattice(x, y) {
            return false;
        }
    }
    for &(ox, oy, w, h, _kind) in cells {
        let cs = [mv((0, 0)), mv((w, 0)), mv((0, h)), mv((w, h))];
        let c0x = cs.iter().map(|c| c.0).min().unwrap();
        let c0y = cs.iter().map(|c| c.1).min().unwrap();
        let dx = cs.iter().map(|c| c.0).max().unwrap() - c0x;
        let dy = cs.iter().map(|c| c.1).max().unwrap() - c0y;
        let kind2 = u8::from(dx == 2 && dy == 4); // image domino kind: 2×4 (doubled) = V
        let t = (
            m[0] * ox + m[1] * oy + c0x + v[0],
            m[2] * ox + m[3] * oy + c0y + v[1],
        );
        if !in_origin(kind2, t.0, t.1) {
            return false;
        }
    }
    true
}

/// Exact shape-set wallpaper-group probe for the rotated (non
/// axis-aligned) paver data tables.
///
/// `pavers_exact_is_sym` keys dominoes by their axis-aligned bbox,
/// which breaks once a cell is rotated 45°. This probe works with
/// bare vertex sets instead: a candidate isometry `g = (M, v)` is a
/// symmetry iff `M` maps the lattice basis into the tiling lattice
/// and, for every source shape `S` in the cell list, the image
/// `M·S + v` equals some target shape `T` translated by a lattice
/// vector `p` — `p` is determined as the min-corner anchor (unique:
/// a bounded shape has no nonzero self-translation). All coordinates
/// are the table's integer P-coords, where the lattice basis, the
/// shape vertices, and the translations are all integral.
#[cfg(all(test, feature = "pavers"))]
fn shape_set_is_sym(
    m: [i64; 4], // M = [[m[0], m[1]], [m[2], m[3]]]
    v: [i64; 2],
    basis: &[(i64, i64); 2],
    shapes: &[[[i64; 2]; 6]],
    in_lattice: impl Fn(i64, i64) -> bool,
) -> bool {
    let mv = |p: [i64; 2]| [
        m[0] * p[0] + m[1] * p[1],
        m[2] * p[0] + m[3] * p[1],
    ];
    // M must map the lattice to itself; all candidates have det ±1, so
    // checking the basis vectors suffices.
    for &(bx, by) in basis {
        let q = mv([bx, by]);
        if !in_lattice(q[0], q[1]) {
            return false;
        }
    }
    for s in shapes {
        let img: Vec<[i64; 2]> = s
            .iter()
            .map(|p| {
                let q = mv(*p);
                [q[0] + v[0], q[1] + v[1]]
            })
            .collect();
        let img_minx = img.iter().map(|p| p[0]).min().unwrap();
        let img_miny = img.iter().map(|p| p[1]).min().unwrap();
        let mut matched = false;
        for t in shapes {
            let t_minx = t.iter().map(|p| p[0]).min().unwrap();
            let t_miny = t.iter().map(|p| p[1]).min().unwrap();
            let px = img_minx - t_minx;
            let py = img_miny - t_miny;
            if !in_lattice(px, py) {
                continue;
            }
            let mut a = img.clone();
            let mut b: Vec<[i64; 2]> =
                t.iter().map(|p| [p[0] + px, p[1] + py]).collect();
            a.sort_unstable();
            b.sort_unstable();
            if a == b {
                matched = true;
                break;
            }
        }
        if !matched {
            return false;
        }
    }
    true
}

#[cfg(test)]
#[cfg(feature = "pavers")]
mod pavers_tests {
    use super::pavers_exact_is_sym;
    use super::shape_set_is_sym;
    use super::tests::{check_covers, check_det, check_rotated};
    use crate::basketweave;
    use crate::herringbone;
    use crate::glide_hexagon_strips;
    use crate::running_bond;
    use crate::shifted_trapezoid_courses;
    use crate::stepped_hexominoes;
    use crate::trapezoid_courses;
    use crate::wavy_courses;
    use crate::alcove_cut_grid;
    use crate::chiral_cut_honeycomb;
    use crate::split_triangle_grid;
    use crate::stack_bond;
    use crate::herringbone_45;
    use crate::basketweave_3;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::ParamSet;
    use tess_core::tiling::{Query, Tiling};

    /// Candidate matrices (row-major) used by the paver group probes.
    const M_ROT4: [i64; 4] = [0, -1, 1, 0];
    const M_ROT2: [i64; 4] = [-1, 0, 0, -1];
    const M_MIRX: [i64; 4] = [-1, 0, 0, 1]; // mirror of x = c: (2c−x, y)
    const M_MIRY: [i64; 4] = [1, 0, 0, -1]; // mirror of y = c: (x, 2c−y)
    const M_DIAG: [i64; 4] = [0, 1, 1, 0]; // glide/mirror of y = x + c
    const M_ANTI: [i64; 4] = [0, -1, -1, 0]; // glide/mirror of y = −x + c

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn basketweave_covers_region() {
        check_covers(
            &basketweave::basketweave(),
            "basketweave",
            BBox::new(v2(-3.0, -3.0), v2(11.0, 9.0)),
        );
    }

    #[test]
    fn basketweave_rotated_scaled() {
        check_rotated(
            &basketweave::basketweave(),
            "basketweave(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn basketweave_deterministic() {
        check_det(
            &basketweave::basketweave(),
            "basketweave",
            BBox::new(v2(-2.0, -2.0), v2(8.0, 6.0)),
        );
    }

    /// Lattice-membership / origin-coset predicates for the basketweave
    /// table, in doubled coordinates. L = {x,y even, x+y ≡ 0 (mod 4)};
    /// H origins = L ∪ (L + (0,1)), V origins = L + (2,0) ∪ L + (3,0).
    fn bw_in_lattice(x: i64, y: i64) -> bool {
        x.rem_euclid(4) == 0 && y.rem_euclid(4) == 0 && (x + y).rem_euclid(8) == 0
    }

    fn bw_in_origin(kind: u8, x: i64, y: i64) -> bool {
        match kind {
            0 => bw_in_lattice(x, y) || bw_in_lattice(x, y - 2),
            _ => bw_in_lattice(x - 4, y) || bw_in_lattice(x - 6, y),
        }
    }

    #[test]
    fn basketweave_wallpaper_group_is_p4g() {
        // Unit-cell dominoes in doubled coordinates: H (0,0), H (0,1),
        // V (2,0), V (3,0); kind 0 = H, 1 = V.
        let basis = [(4, 4), (4, -4)];
        let cells: [(i64, i64, i64, i64, u8); 4] =
            [(0, 0, 4, 2, 0), (0, 2, 4, 2, 0), (4, 0, 2, 4, 1), (6, 0, 2, 4, 1)];
        let sym = |m: [i64; 4], v: [i64; 2]| {
            pavers_exact_is_sym(m, v, &basis, &cells, bw_in_lattice, bw_in_origin)
        };

        // 4-folds at every even-even point; 2-folds at odd-odd;
        // mirrors at the odd axes; six glide families.
        let expected = [
            (M_ROT4, [0, 0], "rot4(0,0)"),
            (M_ROT4, [4, -4], "rot4(2,0)"),
            (M_ROT4, [4, 4], "rot4(0,2)"),
            (M_ROT4, [8, 0], "rot4(2,2)"),
            (M_ROT4, [8, -8], "rot4(4,0)"),
            (M_ROT4, [8, 8], "rot4(0,4)"),
            (M_ROT4, [12, -4], "rot4(4,2)"),
            (M_ROT4, [16, 0], "rot4(4,4)"),
            (M_ROT2, [4, 4], "rot2(1,1)"),
            (M_ROT2, [12, 12], "rot2(3,3)"),
            (M_ROT2, [4, 12], "rot2(1,3)"),
            (M_ROT2, [12, 4], "rot2(3,1)"),
            (M_MIRX, [4, 0], "mir_x(1)"),
            (M_MIRX, [12, 0], "mir_x(3)"),
            (M_MIRY, [0, 4], "mir_y(1)"),
            (M_MIRY, [0, 12], "mir_y(3)"),
            (M_DIAG, [0, 4], "glide(y, x+2)"),
            (M_DIAG, [4, 0], "glide(y+2, x)"),
            (M_ANTI, [4, 0], "glide(2-y, -x)"),
            (M_ANTI, [8, 4], "glide(4-y, 2-x)"),
            (M_MIRX, [0, 4], "glide(-x, y+2)"),
            (M_MIRY, [4, 0], "glide(x+2, -y)"),
        ];
        for (m, v, label) in expected {
            assert!(
                sym(m, v),
                "{label} must be a symmetry of the p4g basketweave"
            );
        }

        // The rest of the candidate families are absent: this is what
        // pins p4g (vs p4m / cmm / pgg).
        let forbidden = [
            // diagonal mirrors, integer and half-integer offsets
            (M_DIAG, [0, 0], "mir_d(0)"),
            (M_DIAG, [-2, 2], "mir_d(1)"),
            (M_DIAG, [-4, 4], "mir_d(2)"),
            (M_DIAG, [-6, 6], "mir_d(3)"),
            (M_DIAG, [-8, 8], "mir_d(4)"),
            (M_ANTI, [0, 0], "mir_a(0)"),
            (M_ANTI, [2, 2], "mir_a(1)"),
            (M_ANTI, [4, 4], "mir_a(2)"),
            (M_ANTI, [6, 6], "mir_a(3)"),
            (M_ANTI, [8, 8], "mir_a(4)"),
            (M_DIAG, [-1, 1], "mir_dh(1/2)"),
            (M_DIAG, [-3, 3], "mir_dh(3/2)"),
            (M_DIAG, [-5, 5], "mir_dh(5/2)"),
            (M_DIAG, [-7, 7], "mir_dh(7/2)"),
            (M_DIAG, [-9, 9], "mir_dh(9/2)"),
            (M_ANTI, [1, 1], "mir_ah(1/2)"),
            (M_ANTI, [3, 3], "mir_ah(3/2)"),
            (M_ANTI, [5, 5], "mir_ah(5/2)"),
            (M_ANTI, [7, 7], "mir_ah(7/2)"),
            (M_ANTI, [9, 9], "mir_ah(9/2)"),
            // even-axis and half-integer mirrors
            (M_MIRX, [0, 0], "mir_x(0)"),
            (M_MIRX, [8, 0], "mir_x(2)"),
            (M_MIRX, [16, 0], "mir_x(4)"),
            (M_MIRY, [0, 0], "mir_y(0)"),
            (M_MIRY, [0, 8], "mir_y(2)"),
            (M_MIRY, [0, 16], "mir_y(4)"),
            (M_MIRX, [2, 0], "mir_xhalf(1/2)"),
            (M_MIRX, [6, 0], "mir_xhalf(3/2)"),
            (M_MIRX, [10, 0], "mir_xhalf(5/2)"),
            (M_MIRX, [14, 0], "mir_xhalf(7/2)"),
            (M_MIRX, [18, 0], "mir_xhalf(9/2)"),
            (M_MIRY, [0, 2], "mir_yhalf(1/2)"),
            (M_MIRY, [0, 6], "mir_yhalf(3/2)"),
            (M_MIRY, [0, 10], "mir_yhalf(5/2)"),
            (M_MIRY, [0, 14], "mir_yhalf(7/2)"),
            (M_MIRY, [0, 18], "mir_yhalf(9/2)"),
            // 4-folds away from even-even centers
            (M_ROT4, [4, 0], "rot4(1,1)"),
            (M_ROT4, [12, 0], "rot4(3,3)"),
            (M_ROT4, [8, 4], "rot4(1,3)"),
            (M_ROT4, [8, -4], "rot4(3,1)"),
            (M_ROT4, [2, 0], "rot4(1/2,1/2)"),
            (M_ROT4, [2, -2], "rot4(0,1)"),
            // odd-axis vertical/horizontal glides (T(2,0), T(0,2) are
            // not tiling translations)
            (M_MIRX, [4, 4], "glide(2-x, y+2)"),
            (M_MIRY, [4, 4], "glide(x+2, 2-y)"),
        ];
        for (m, v, label) in forbidden {
            assert!(
                !sym(m, v),
                "{label} must NOT be a symmetry of the basketweave"
            );
        }
    }

    #[test]
    fn herringbone_covers_region() {
        check_covers(
            &herringbone::herringbone(),
            "herringbone",
            BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
        );
    }

    #[test]
    fn herringbone_rotated_scaled() {
        check_rotated(
            &herringbone::herringbone(),
            "herringbone(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn herringbone_deterministic() {
        check_det(
            &herringbone::herringbone(),
            "herringbone",
            BBox::new(v2(-4.0, -4.0), v2(8.0, 4.0)),
        );
    }

    /// Lattice-membership / origin-coset predicates for the herringbone
    /// table, in doubled coordinates. L = ⟨(1,3),(2,2)⟩ = {(x,y): y−x
    /// even, 3x−y ≡ 0 (mod 4)}; H origins = L, V origins = L + (2,0).
    fn hb_in_lattice(x: i64, y: i64) -> bool {
        (y - x).rem_euclid(4) == 0 && (3 * x - y).rem_euclid(8) == 0
    }

    fn hb_in_origin(kind: u8, x: i64, y: i64) -> bool {
        match kind {
            0 => hb_in_lattice(x, y),
            _ => hb_in_lattice(x - 4, y),
        }
    }

    #[test]
    fn herringbone_wallpaper_group_is_pgg() {
        // Unit-cell dominoes in doubled coordinates: H (0,0), V (2,0);
        // kind 0 = H, 1 = V.
        let basis = [(2, 6), (4, 4)];
        let cells: [(i64, i64, i64, i64, u8); 2] =
            [(0, 0, 4, 2, 0), (4, 0, 2, 4, 1)];
        let sym = |m: [i64; 4], v: [i64; 2]| {
            pavers_exact_is_sym(m, v, &basis, &cells, hb_in_lattice, hb_in_origin)
        };

        // 16 2-fold centers mod 4, two perpendicular glide families.
        let expected = [
            (M_ROT2, [0, 6], "rot2(0,3/2)"),
            (M_ROT2, [0, 14], "rot2(0,7/2)"),
            (M_ROT2, [4, 2], "rot2(1,1/2)"),
            (M_ROT2, [4, 10], "rot2(1,5/2)"),
            (M_ROT2, [2, 4], "rot2(1/2,1)"),
            (M_ROT2, [2, 12], "rot2(1/2,3)"),
            (M_ROT2, [8, 6], "rot2(2,3/2)"),
            (M_ROT2, [8, 14], "rot2(2,7/2)"),
            (M_ROT2, [12, 2], "rot2(3,1/2)"),
            (M_ROT2, [12, 10], "rot2(3,5/2)"),
            (M_ROT2, [6, 0], "rot2(3/2,0)"),
            (M_ROT2, [6, 8], "rot2(3/2,2)"),
            (M_ROT2, [10, 4], "rot2(5/2,1)"),
            (M_ROT2, [10, 12], "rot2(5/2,3)"),
            (M_ROT2, [14, 0], "rot2(7/2,0)"),
            (M_ROT2, [14, 8], "rot2(7/2,2)"),
            // diagonal glide family (axes y = x + c)
            (M_DIAG, [2, 2], "glide(y+1, x+1)"),
            (M_DIAG, [0, 4], "glide(y, x+2)"),
            (M_DIAG, [-2, 6], "glide(y-1, x+3)"),
            (M_DIAG, [-4, 8], "glide(y-2, x+4)"),
            // anti-diagonal glide family (axes y = -x + c)
            (M_ANTI, [2, 0], "glide(1-y, -x)"),
            (M_ANTI, [-2, -4], "glide(-1-y, -x-2)"),
        ];
        for (m, v, label) in expected {
            assert!(
                sym(m, v),
                "{label} must be a symmetry of the pgg herringbone"
            );
        }

        // No mirrors of any family, no 4-folds, no off-family glides,
        // and 2-folds only at the 16 listed centers: this is what
        // pins pgg (vs cmm / p4g / p4m).
        let forbidden = [
            // horizontal/vertical mirrors, integer and half-integer axes
            (M_MIRX, [0, 0], "mir_x(0)"),
            (M_MIRX, [4, 0], "mir_x(1)"),
            (M_MIRX, [8, 0], "mir_x(2)"),
            (M_MIRX, [12, 0], "mir_x(3)"),
            (M_MIRX, [16, 0], "mir_x(4)"),
            (M_MIRY, [0, 0], "mir_y(0)"),
            (M_MIRY, [0, 4], "mir_y(1)"),
            (M_MIRY, [0, 8], "mir_y(2)"),
            (M_MIRY, [0, 12], "mir_y(3)"),
            (M_MIRY, [0, 16], "mir_y(4)"),
            (M_MIRX, [2, 0], "mir_xhalf(1/2)"),
            (M_MIRX, [6, 0], "mir_xhalf(3/2)"),
            (M_MIRX, [10, 0], "mir_xhalf(5/2)"),
            (M_MIRX, [14, 0], "mir_xhalf(7/2)"),
            (M_MIRX, [18, 0], "mir_xhalf(9/2)"),
            (M_MIRY, [0, 2], "mir_yhalf(1/2)"),
            (M_MIRY, [0, 6], "mir_yhalf(3/2)"),
            (M_MIRY, [0, 10], "mir_yhalf(5/2)"),
            (M_MIRY, [0, 14], "mir_yhalf(7/2)"),
            (M_MIRY, [0, 18], "mir_yhalf(9/2)"),
            // diagonal mirrors, integer and half-integer offsets
            (M_DIAG, [0, 0], "mir_d(0)"),
            (M_DIAG, [-2, 2], "mir_d(1)"),
            (M_DIAG, [-4, 4], "mir_d(2)"),
            (M_DIAG, [-6, 6], "mir_d(3)"),
            (M_DIAG, [-8, 8], "mir_d(4)"),
            (M_DIAG, [-1, 1], "mir_dh(1/2)"),
            (M_DIAG, [-3, 3], "mir_dh(3/2)"),
            (M_DIAG, [-5, 5], "mir_dh(5/2)"),
            (M_DIAG, [-7, 7], "mir_dh(7/2)"),
            (M_ANTI, [0, 0], "mir_a(0)"),
            (M_ANTI, [2, 2], "mir_a(1)"),
            (M_ANTI, [4, 4], "mir_a(2)"),
            (M_ANTI, [6, 6], "mir_a(3)"),
            (M_ANTI, [8, 8], "mir_a(4)"),
            (M_ANTI, [1, 1], "mir_ah(1/2)"),
            (M_ANTI, [3, 3], "mir_ah(3/2)"),
            (M_ANTI, [5, 5], "mir_ah(5/2)"),
            (M_ANTI, [7, 7], "mir_ah(7/2)"),
            // off-family glides
            (M_ANTI, [2, 2], "glide(1-y, 1-x)"),
            (M_MIRX, [0, 2], "glide(-x, y+1)"),
            (M_MIRY, [2, 0], "glide(x+1, -y)"),
            // no 4-folds
            (M_ROT4, [0, 0], "rot4(0,0)"),
            (M_ROT4, [4, 0], "rot4(1,1)"),
            (M_ROT4, [8, 0], "rot4(2,2)"),
            // 2-folds only at the 16 expected centers
            (M_ROT2, [0, 0], "rot2(0,0)"),
            (M_ROT2, [4, 4], "rot2(1,1)"),
            (M_ROT2, [0, 4], "rot2(0,1)"),
            (M_ROT2, [4, 12], "rot2(1,3)"),
            (M_ROT2, [8, 4], "rot2(2,1)"),
            (M_ROT2, [12, 12], "rot2(3,3)"),
            (M_ROT2, [0, 10], "rot2(0,5/2)"),
            (M_ROT2, [2, 2], "rot2(1/2,1/2)"),
            (M_ROT2, [6, 6], "rot2(3/2,3/2)"),
            (M_ROT2, [10, 10], "rot2(5/2,5/2)"),
            (M_ROT2, [14, 14], "rot2(7/2,7/2)"),
        ];
        for (m, v, label) in forbidden {
            assert!(
                !sym(m, v),
                "{label} must NOT be a symmetry of the herringbone"
            );
        }
    }

    #[test]
    fn running_bond_covers_region() {
        check_covers(
            &running_bond::running_bond(),
            "running-bond",
            BBox::new(v2(-4.0, -4.0), v2(8.0, 6.0)),
        );
    }

    #[test]
    fn running_bond_rotated_scaled() {
        check_rotated(
            &running_bond::running_bond(),
            "running-bond(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn running_bond_deterministic() {
        check_det(
            &running_bond::running_bond(),
            "running-bond",
            BBox::new(v2(-3.0, -3.0), v2(9.0, 5.0)),
        );
    }

    /// Lattice-membership / origin-set predicates for the running-bond
    /// table, in doubled coordinates. L = 2ℤ²; domino origins = L ∪
    /// (L + (1,1)) (all horizontal — there are no vertical dominoes).
    fn rb_in_lattice(x: i64, y: i64) -> bool {
        x.rem_euclid(4) == 0 && y.rem_euclid(4) == 0
    }

    fn rb_in_origin(kind: u8, x: i64, y: i64) -> bool {
        match kind {
            0 => rb_in_lattice(x, y) || rb_in_lattice(x - 2, y - 2),
            _ => false, // no vertical dominoes in the running bond
        }
    }

    #[test]
    fn running_bond_wallpaper_group_is_cmm() {
        // Unit-cell dominoes in doubled coordinates: H (0,0),
        // H (1,1); kind 0 = H.
        let basis = [(4, 0), (0, 4)];
        let cells: [(i64, i64, i64, i64, u8); 2] =
            [(0, 0, 4, 2, 0), (2, 2, 4, 2, 0)];
        let sym = |m: [i64; 4], v: [i64; 2]| {
            pavers_exact_is_sym(m, v, &basis, &cells, rb_in_lattice, rb_in_origin)
        };

        // Mirrors at the integer x-axes and half-integer y-axes;
        // 2-folds at (k, l+1/2) on the mirrors and at (k+1/2, l)
        // between the rows; two glide families per direction.
        let expected = [
            (M_MIRX, [0, 0], "mir_x(0)"),
            (M_MIRX, [4, 0], "mir_x(1)"),
            (M_MIRX, [8, 0], "mir_x(2)"),
            (M_MIRX, [12, 0], "mir_x(3)"),
            (M_MIRX, [16, 0], "mir_x(4)"),
            (M_MIRY, [0, 2], "mir_yhalf(1/2)"),
            (M_MIRY, [0, 6], "mir_yhalf(3/2)"),
            (M_MIRY, [0, 10], "mir_yhalf(5/2)"),
            (M_MIRY, [0, 14], "mir_yhalf(7/2)"),
            (M_MIRY, [0, 18], "mir_yhalf(9/2)"),
            // 2-folds on the mirrors: (k, l+1/2)
            (M_ROT2, [0, 2], "rot2(0,1/2)"),
            (M_ROT2, [0, 6], "rot2(0,3/2)"),
            (M_ROT2, [0, 10], "rot2(0,5/2)"),
            (M_ROT2, [0, 14], "rot2(0,7/2)"),
            (M_ROT2, [4, 2], "rot2(1,1/2)"),
            (M_ROT2, [4, 6], "rot2(1,3/2)"),
            (M_ROT2, [4, 10], "rot2(1,5/2)"),
            (M_ROT2, [4, 14], "rot2(1,7/2)"),
            (M_ROT2, [8, 2], "rot2(2,1/2)"),
            (M_ROT2, [8, 6], "rot2(2,3/2)"),
            (M_ROT2, [8, 10], "rot2(2,5/2)"),
            (M_ROT2, [8, 14], "rot2(2,7/2)"),
            (M_ROT2, [12, 2], "rot2(3,1/2)"),
            (M_ROT2, [12, 6], "rot2(3,3/2)"),
            (M_ROT2, [12, 10], "rot2(3,5/2)"),
            (M_ROT2, [12, 14], "rot2(3,7/2)"),
            (M_ROT2, [16, 2], "rot2(4,1/2)"),
            (M_ROT2, [16, 6], "rot2(4,3/2)"),
            (M_ROT2, [16, 10], "rot2(4,5/2)"),
            (M_ROT2, [16, 14], "rot2(4,7/2)"),
            // 2-folds between the rows: (k+1/2, l)
            (M_ROT2, [2, 0], "rot2(1/2,0)"),
            (M_ROT2, [2, 4], "rot2(1/2,1)"),
            (M_ROT2, [2, 8], "rot2(1/2,2)"),
            (M_ROT2, [2, 12], "rot2(1/2,3)"),
            (M_ROT2, [2, 16], "rot2(1/2,4)"),
            (M_ROT2, [6, 0], "rot2(3/2,0)"),
            (M_ROT2, [6, 4], "rot2(3/2,1)"),
            (M_ROT2, [6, 8], "rot2(3/2,2)"),
            (M_ROT2, [6, 12], "rot2(3/2,3)"),
            (M_ROT2, [6, 16], "rot2(3/2,4)"),
            (M_ROT2, [10, 0], "rot2(5/2,0)"),
            (M_ROT2, [10, 4], "rot2(5/2,1)"),
            (M_ROT2, [10, 8], "rot2(5/2,2)"),
            (M_ROT2, [10, 12], "rot2(5/2,3)"),
            (M_ROT2, [10, 16], "rot2(5/2,4)"),
            (M_ROT2, [14, 0], "rot2(7/2,0)"),
            (M_ROT2, [14, 4], "rot2(7/2,1)"),
            (M_ROT2, [14, 8], "rot2(7/2,2)"),
            (M_ROT2, [14, 12], "rot2(7/2,3)"),
            (M_ROT2, [14, 16], "rot2(7/2,4)"),
            // two glide families per direction
            (M_MIRY, [2, 0], "glide(x+1, -y)"),
            (M_MIRY, [2, 8], "glide(x+1, 4-y)"),
            (M_MIRX, [2, 2], "glide(1-x, y+1)"),
            (M_MIRX, [6, 2], "glide(3-x, y+1)"),
        ];
        for (m, v, label) in expected {
            assert!(
                sym(m, v),
                "{label} must be a symmetry of the cmm running bond"
            );
        }

        // No diagonal mirrors, no 4-folds, no integer-axis y-mirrors,
        // no off-axis glides, and 2-folds only at the two center
        // families: this is what pins cmm (vs cmm' / pmm / p4g).
        let forbidden = [
            // diagonal mirrors, integer and half-integer offsets
            (M_DIAG, [0, 0], "mir_d(0)"),
            (M_DIAG, [-2, 2], "mir_d(1)"),
            (M_DIAG, [-4, 4], "mir_d(2)"),
            (M_DIAG, [-6, 6], "mir_d(3)"),
            (M_DIAG, [-8, 8], "mir_d(4)"),
            (M_DIAG, [-1, 1], "mir_dh(1/2)"),
            (M_DIAG, [-3, 3], "mir_dh(3/2)"),
            (M_DIAG, [-5, 5], "mir_dh(5/2)"),
            (M_DIAG, [-7, 7], "mir_dh(7/2)"),
            (M_ANTI, [0, 0], "mir_a(0)"),
            (M_ANTI, [2, 2], "mir_a(1)"),
            (M_ANTI, [4, 4], "mir_a(2)"),
            (M_ANTI, [6, 6], "mir_a(3)"),
            (M_ANTI, [8, 8], "mir_a(4)"),
            (M_ANTI, [1, 1], "mir_ah(1/2)"),
            (M_ANTI, [3, 3], "mir_ah(3/2)"),
            (M_ANTI, [5, 5], "mir_ah(5/2)"),
            (M_ANTI, [7, 7], "mir_ah(7/2)"),
            // integer-axis y-mirrors and half-integer x-mirrors
            (M_MIRY, [0, 0], "mir_y(0)"),
            (M_MIRY, [0, 4], "mir_y(1)"),
            (M_MIRY, [0, 8], "mir_y(2)"),
            (M_MIRY, [0, 12], "mir_y(3)"),
            (M_MIRY, [0, 16], "mir_y(4)"),
            (M_MIRX, [2, 0], "mir_xhalf(1/2)"),
            (M_MIRX, [6, 0], "mir_xhalf(3/2)"),
            (M_MIRX, [10, 0], "mir_xhalf(5/2)"),
            (M_MIRX, [14, 0], "mir_xhalf(7/2)"),
            (M_MIRX, [18, 0], "mir_xhalf(9/2)"),
            // 2-folds off the two center families
            (M_ROT2, [0, 0], "rot2(0,0)"),
            (M_ROT2, [4, 4], "rot2(1,1)"),
            (M_ROT2, [8, 8], "rot2(2,2)"),
            (M_ROT2, [12, 12], "rot2(3,3)"),
            (M_ROT2, [16, 16], "rot2(4,4)"),
            (M_ROT2, [2, 2], "rot2(1/2,1/2)"),
            (M_ROT2, [6, 6], "rot2(3/2,3/2)"),
            (M_ROT2, [10, 10], "rot2(5/2,5/2)"),
            (M_ROT2, [14, 14], "rot2(7/2,7/2)"),
            // no 4-folds
            (M_ROT4, [0, 0], "rot4(0,0)"),
            (M_ROT4, [4, 0], "rot4(1,1)"),
            // off-axis glides
            (M_MIRY, [2, 2], "glide(x+1, 1-y)"),
            (M_MIRX, [0, 2], "glide(-x, y+1)"),
        ];
        for (m, v, label) in forbidden {
            assert!(
                !sym(m, v),
                "{label} must NOT be a symmetry of the running bond"
            );
        }
    }

    /// Stack-bond (M114) standard trio.
    #[test]
    fn stack_bond_covers_region() {
        check_covers(
            &stack_bond::stack_bond(),
            "stack-bond",
            BBox::new(v2(-3.0, -2.0), v2(9.0, 6.0)),
        );
    }

    #[test]
    fn stack_bond_rotated_scaled() {
        check_rotated(
            &stack_bond::stack_bond(),
            "stack-bond(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn stack_bond_deterministic() {
        check_det(
            &stack_bond::stack_bond(),
            "stack-bond",
            BBox::new(v2(-2.0, -1.0), v2(8.0, 5.0)),
        );
    }

    /// Lattice-membership / origin-set predicates for the stack-bond
    /// table, in doubled coordinates. L = <(4,0),(0,2)>; domino
    /// origins = L (all horizontal — there are no vertical dominoes).
    fn sb_in_lattice(x: i64, y: i64) -> bool {
        x.rem_euclid(4) == 0 && y.rem_euclid(2) == 0
    }

    fn sb_in_origin(kind: u8, x: i64, y: i64) -> bool {
        match kind {
            0 => sb_in_lattice(x, y),
            _ => false, // no vertical dominoes in the stack bond
        }
    }

    #[test]
    fn stack_bond_wallpaper_group_is_pmm() {
        // Unit-cell domino in doubled coordinates: H (0,0);
        // kind 0 = H.
        let basis = [(4, 0), (0, 2)];
        let cells: [(i64, i64, i64, i64, u8); 1] = [(0, 0, 4, 2, 0)];
        let sym = |m: [i64; 4], v: [i64; 2]| {
            pavers_exact_is_sym(m, v, &basis, &cells, sb_in_lattice, sb_in_origin)
        };

        // Mirrors at the integer x-axes and the half-integer
        // y-axes (row boundaries and row midlines); 2-fold centers at
        // every mirror intersection (k, n/2).
        let expected = [
            (M_MIRX, [0, 0], "mir_x(0)"),
            (M_MIRX, [4, 0], "mir_x(1)"),
            (M_MIRX, [8, 0], "mir_x(2)"),
            (M_MIRX, [-4, 0], "mir_x(-1)"),
            (M_MIRX, [-8, 0], "mir_x(-2)"),
            (M_MIRY, [0, 0], "mir_y(0)"),
            (M_MIRY, [0, 2], "mir_y(1/2)"),
            (M_MIRY, [0, 4], "mir_y(1)"),
            (M_MIRY, [0, 6], "mir_y(3/2)"),
            (M_MIRY, [0, -2], "mir_y(-1/2)"),
            (M_MIRY, [0, -4], "mir_y(-1)"),
            // 2-folds at the mirror intersections (k, n/2)
            (M_ROT2, [0, 0], "rot2(0,0)"),
            (M_ROT2, [4, 0], "rot2(1,0)"),
            (M_ROT2, [0, 2], "rot2(0,1/2)"),
            (M_ROT2, [4, 2], "rot2(1,1/2)"),
            (M_ROT2, [8, 4], "rot2(2,1)"),
            (M_ROT2, [-4, 6], "rot2(-1,3/2)"),
            (M_ROT2, [0, -2], "rot2(0,-1/2)"),
            (M_ROT2, [12, -4], "rot2(3,-1)"),
        ];
        for (m, v, label) in expected {
            assert!(
                sym(m, v),
                "{label} must be a symmetry of the pmm stack bond"
            );
        }

        // No half-integer x-mirrors, no y-mirrors off y = n/2, no
        // 2-folds off the (k, n/2) grid, no 4-folds, no diagonal
        // mirrors, no glides: this is what pins pmm (vs cmm / p4m /
        // pgg).
        let forbidden = [
            // half-integer x-axes are not mirrors
            (M_MIRX, [2, 0], "mir_xhalf(1/2)"),
            (M_MIRX, [6, 0], "mir_xhalf(3/2)"),
            (M_MIRX, [-2, 0], "mir_xhalf(-1/2)"),
            // y-mirrors only at y = n/2
            (M_MIRY, [0, 1], "mir_y(1/4)"),
            (M_MIRY, [0, 3], "mir_y(3/4)"),
            (M_MIRY, [0, 5], "mir_y(5/4)"),
            // 2-folds only at the (k, n/2) intersections
            (M_ROT2, [2, 0], "rot2(1/2,0)"),
            (M_ROT2, [2, 2], "rot2(1/2,1/2)"),
            (M_ROT2, [6, 2], "rot2(3/2,1/2)"),
            (M_ROT2, [0, 1], "rot2(0,1/4)"),
            (M_ROT2, [4, 1], "rot2(1,1/4)"),
            // no 4-folds
            (M_ROT4, [0, 0], "rot4(0,0)"),
            (M_ROT4, [4, 0], "rot4(1,0)"),
            (M_ROT4, [0, 2], "rot4(0,1/2)"),
            // no diagonal mirrors
            (M_DIAG, [0, 0], "mir_d(0)"),
            (M_DIAG, [4, 0], "mir_d(1)"),
            (M_DIAG, [2, 2], "mir_d(1/2)"),
            (M_ANTI, [0, 0], "mir_a(0)"),
            (M_ANTI, [4, 0], "mir_a(1)"),
            (M_ANTI, [2, 2], "mir_a(1/2)"),
            // no glides (the rigid sibling of the running bond's cmm)
            (M_MIRX, [2, 2], "glide(1-x, y+1)"),
            (M_MIRY, [2, 0], "glide(x+1, -y)"),
        ];
        for (m, v, label) in forbidden {
            assert!(
                !sym(m, v),
                "{label} must NOT be a symmetry of the stack bond"
            );
        }
    }

    /// Herringbone-45 (M115) standard trio.
    #[test]
    fn herringbone_45_covers_region() {
        check_covers(
            &herringbone_45::herringbone_45(),
            "herringbone-45",
            BBox::new(v2(-4.0, -4.0), v2(8.0, 8.0)),
        );
    }

    #[test]
    fn herringbone_45_rotated_scaled() {
        check_rotated(
            &herringbone_45::herringbone_45(),
            "herringbone-45(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn herringbone_45_deterministic() {
        check_det(
            &herringbone_45::herringbone_45(),
            "herringbone-45",
            BBox::new(v2(-2.0, -2.0), v2(8.0, 6.0)),
        );
    }

    /// Lattice-membership predicate for the herringbone-45 table, in
    /// its integer P-coords: L'' = <(2,0),(0,4)> = { x even,
    /// y ≡ 0 (mod 4) }; world = (√2/2)·P.
    fn h45_in_lattice(x: i64, y: i64) -> bool {
        x.rem_euclid(2) == 0 && y.rem_euclid(4) == 0
    }

    #[test]
    fn herringbone_45_wallpaper_group_is_pgg() {
        // Cell shapes in integer P-coords (world = (√2/2)·P), each in
        // cyclic order; the shapes are rotated hexagons, so the probe
        // uses the vertex-set criterion rather than the axis-aligned
        // bbox probe.
        let basis = [(2, 0), (0, 4)];
        let shapes: [[[i64; 2]; 6]; 2] = [
            [[0, 0], [1, 1], [2, 2], [1, 3], [0, 2], [-1, 1]],
            [[2, 2], [3, 3], [2, 4], [1, 5], [0, 4], [1, 3]],
        ];
        let sym = |m: [i64; 4], v: [i64; 2]| {
            shape_set_is_sym(m, v, &basis, &shapes, h45_in_lattice)
        };

        // pgg: translations L''; 2-folds at v ≡ (1,3) mod (2,4);
        // vertical glide axes (MIRX, v ≡ (0,2) mod (2,4)); horizontal
        // glide axes (MIRY, v ≡ (1,1) mod (2,4)); no mirrors, no
        // 4-folds, no diagonal glides.
        let expected = [
            (M_ROT2, [1, 3], "rot2 center (1/2,3/2)"),
            (M_ROT2, [3, 3], "rot2 center (3/2,3/2)"),
            (M_ROT2, [-1, 3], "rot2 center (-1/2,3/2)"),
            (M_ROT2, [1, 7], "rot2 center (1/2,7/2)"),
            (M_ROT2, [1, -1], "rot2 center (1/2,-1/2)"),
            (M_ROT2, [3, -1], "rot2 center (3/2,-1/2)"),
            (M_ROT2, [-1, -1], "rot2 center (-1/2,-1/2)"),
            (M_MIRX, [0, 2], "glide x=0, shift 1"),
            (M_MIRX, [2, 2], "glide x=1, shift 1"),
            (M_MIRX, [0, 6], "glide x=0, shift 3"),
            (M_MIRX, [-2, 2], "glide x=-1, shift 1"),
            (M_MIRX, [0, -2], "glide x=0, shift -1"),
            (M_MIRY, [1, 1], "glide y=1/2, shift 1/2"),
            (M_MIRY, [3, 1], "glide y=1/2, shift 3/2"),
            (M_MIRY, [-1, 1], "glide y=1/2, shift -1/2"),
            (M_MIRY, [1, 5], "glide y=5/2, shift 1/2"),
            (M_MIRY, [1, -3], "glide y=-3/2, shift 1/2"),
            (M_MIRY, [3, 5], "glide y=5/2, shift 3/2"),
        ];
        for (m, v, label) in expected {
            assert!(sym(m, v), "{label} must be a symmetry of the 45° herringbone");
        }

        // No pure mirrors of any family (pgg is chiral): the MIRX
        // mirror lines (v_y = 0 mod 4) and MIRY mirror lines (v_x
        // even) are all absent, as are off-family glides, 4-folds,
        // and diagonal glides.
        let forbidden = [
            (M_MIRX, [0, 0], "mir x=0"),
            (M_MIRX, [2, 0], "mir x=1"),
            (M_MIRX, [4, 0], "mir x=2"),
            (M_MIRY, [0, 0], "mir y=0"),
            (M_MIRY, [0, 4], "mir y=2"),
            (M_MIRY, [2, 2], "mir y=1"),
            // 2-folds off the (1,3) mod (2,4) family
            (M_ROT2, [0, 0], "rot2(0,0)"),
            (M_ROT2, [2, 0], "rot2(1,0)"),
            (M_ROT2, [0, 4], "rot2(0,2)"),
            (M_ROT2, [1, 1], "rot2(1/2,1/2)"),
            (M_ROT2, [2, 2], "rot2(1,1)"),
            (M_ROT2, [1, 5], "rot2(1/2,5/2)"),
            // no 4-folds
            (M_ROT4, [0, 0], "rot4(0,0)"),
            (M_ROT4, [2, 0], "rot4(1,0)"),
            (M_ROT4, [1, 3], "rot4 at a 2-fold"),
            (M_ROT4, [0, 4], "rot4(0,2)"),
            // no diagonal mirrors or glides
            (M_DIAG, [0, 0], "glide/mir y=x"),
            (M_DIAG, [2, 0], "glide/mir y=x-1"),
            (M_DIAG, [1, 1], "glide/mir y=x+1/2"),
            (M_ANTI, [0, 0], "glide/mir y=-x"),
            (M_ANTI, [2, 0], "glide/mir y=-x+1"),
            (M_ANTI, [1, 1], "glide/mir y=-x+1/2"),
        ];
        for (m, v, label) in forbidden {
            assert!(
                !sym(m, v),
                "{label} must NOT be a symmetry of the 45° herringbone"
            );
        }
    }

    /// Basketweave-three (M116) standard trio.
    #[test]
    fn basketweave_3_covers_region() {
        check_covers(
            &basketweave_3::basketweave_3(),
            "basketweave-3",
            BBox::new(v2(-4.0, -4.0), v2(10.0, 9.0)),
        );
    }

    #[test]
    fn basketweave_3_rotated_scaled() {
        check_rotated(
            &basketweave_3::basketweave_3(),
            "basketweave-3(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn basketweave_3_deterministic() {
        check_det(
            &basketweave_3::basketweave_3(),
            "basketweave-3",
            BBox::new(v2(-2.0, -2.0), v2(12.0, 8.0)),
        );
    }

    /// Lattice-membership / origin-set predicates for the basketweave-3
    /// table, in doubled coordinates. The declared cell is
    /// ⟨(12,0),(0,10)⟩; H-domino origins sit at x ≡ 0 (mod 4), y ≡
    /// 0/2/4 (mod 10), V-domino origins at x even, y ≡ 6 (mod 10).
    fn bw3_in_lattice(x: i64, y: i64) -> bool {
        x.rem_euclid(12) == 0 && y.rem_euclid(10) == 0
    }

    fn bw3_in_origin(kind: u8, x: i64, y: i64) -> bool {
        match kind {
            0 => x.rem_euclid(4) == 0 && matches!(y.rem_euclid(10), 0 | 2 | 4),
            1 => x.rem_euclid(2) == 0 && y.rem_euclid(10) == 6,
            _ => false,
        }
    }

    #[test]
    fn basketweave_3_wallpaper_group_is_pmm() {
        const M_I: [i64; 4] = [1, 0, 0, 1];
        const M_ROT4_270: [i64; 4] = [0, 1, -1, 0]; // R270
        // The 15 split dominoes of one 6×5 cell in doubled coordinates:
        // 9 H dominoes (4×2, kind 0) at x ∈ {0,4,8}, y ∈ {0,2,4}, and 6
        // V dominoes (2×4, kind 1) at y = 6, x even.
        let basis = [(12, 0), (0, 10)];
        let mut cells: Vec<(i64, i64, i64, i64, u8)> = Vec::with_capacity(15);
        for oy in [0i64, 2, 4] {
            for ox in [0i64, 4, 8] {
                cells.push((ox, oy, 4, 2, 0));
            }
        }
        for ox in (0..12).step_by(2) {
            cells.push((ox, 6, 2, 4, 1));
        }
        let sym = |m: [i64; 4], v: [i64; 2]| {
            pavers_exact_is_sym(m, v, &basis, &cells, bw3_in_lattice, bw3_in_origin)
        };

        // pmm: translations on the true lattice <(2,0),(0,5)> (doubled:
        // v_x ≡ 0 mod 4, v_y ≡ 0 mod 10 — finer than the declared cell,
        // which is an index-3 superlattice); mirrors at every vertical
        // axis x = k; mirrors at the horizontal axes y = 2.5k + 1.5
        // (doubled: v_y ≡ 6 mod 10); 2-folds at every mirror
        // intersection (k, 2.5k' + 1.5).
        let expected = [
            (M_I, [0, 0], "T(0,0)"),
            (M_I, [4, 0], "T(2,0)"),
            (M_I, [8, 0], "T(4,0)"),
            (M_I, [12, 0], "T(6,0) — declared cell"),
            (M_I, [0, 10], "T(0,5) — declared cell"),
            (M_I, [4, 10], "T(2,5)"),
            (M_I, [-4, 0], "T(-2,0)"),
            (M_I, [-12, 10], "T(-6,5)"),
            (M_MIRX, [0, 0], "mir_x(0)"),
            (M_MIRX, [4, 0], "mir_x(1)"),
            (M_MIRX, [8, 0], "mir_x(2)"),
            (M_MIRX, [-4, 0], "mir_x(-1)"),
            (M_MIRX, [0, 10], "mir_x(0) + T(0,5)"),
            (M_MIRX, [4, 10], "mir_x(1) + T(0,5)"),
            (M_MIRY, [0, 6], "mir_y(3/2)"),
            (M_MIRY, [4, 6], "mir_y(3/2) + T(2,0)"),
            (M_MIRY, [8, 6], "mir_y(3/2) + T(4,0)"),
            (M_MIRY, [0, 16], "mir_y(4)"),
            (M_MIRY, [4, 16], "mir_y(4) + T(2,0)"),
            (M_MIRY, [12, 16], "mir_y(4) + T(6,0)"),
            (M_MIRY, [0, -4], "mir_y(-1)"),
            (M_ROT2, [0, 6], "rot2(0,3/2)"),
            (M_ROT2, [4, 6], "rot2(1,3/2)"),
            (M_ROT2, [8, 16], "rot2(2,4)"),
            (M_ROT2, [12, 16], "rot2(3,4)"),
            (M_ROT2, [0, -4], "rot2(0,-1)"),
            (M_ROT2, [-4, 6], "rot2(-1,3/2)"),
        ];
        for (m, v, label) in expected {
            assert!(
                sym(m, v),
                "{label} must be a symmetry of the basketweave-3"
            );
        }

        // No half-integer x-mirrors, no y-mirrors off the 2.5k+1.5
        // family, no essential glides (a half-period shift along a
        // mirror carries the even-x horizontal courses to odd x, and
        // shifts the vertical walls off the integer x-axes), no
        // 2-folds off the mirror intersections, no 4-folds, no
        // diagonal mirrors.
        let forbidden = [
            // half-integer x-axes are not mirrors
            (M_MIRX, [2, 0], "mir_xhalf(1/2)"),
            (M_MIRX, [6, 0], "mir_xhalf(3/2)"),
            // y-mirrors only at y = 2.5k + 1.5
            (M_MIRY, [0, 0], "mir_y(0)"),
            (M_MIRY, [4, 0], "mir_y(0) + T(2,0)"),
            (M_MIRY, [0, 10], "mir_y(5/2)"),
            (M_MIRY, [4, 10], "mir_y(5/2) + T(2,0)"),
            // glides along the mirrors (half-period shifts)
            (M_MIRX, [0, 5], "glide x=0, shift 5/2"),
            (M_MIRX, [4, 5], "glide x=1, shift 5/2"),
            (M_MIRY, [2, 6], "glide y=3/2, shift 1"),
            (M_MIRY, [2, 16], "glide y=4, shift 1"),
            // mirrors shifted by a non-lattice amount
            (M_MIRX, [0, 6], "mir_x(0) + T(0,3)"),
            (M_MIRX, [4, 6], "mir_x(1) + T(0,3)"),
            // 2-folds only at the mirror intersections
            (M_ROT2, [0, 0], "rot2(0,0)"),
            (M_ROT2, [4, 0], "rot2(1,0)"),
            (M_ROT2, [0, 10], "rot2(0,5/2)"),
            (M_ROT2, [2, 6], "rot2(1/2,3/2)"),
            // no 4-folds
            (M_ROT4, [0, 0], "rot4(0,0)"),
            (M_ROT4, [4, 0], "rot4(1,0)"),
            (M_ROT4_270, [0, 0], "rot4_270(0,0)"),
            (M_ROT4_270, [12, 10], "rot4_270(3,5/2)"),
            // no diagonal mirrors or glides
            (M_DIAG, [0, 0], "mir_d(0)"),
            (M_DIAG, [4, 6], "mir_d(3/2)"),
            (M_ANTI, [0, 0], "mir_a(0)"),
            (M_ANTI, [0, 16], "mir_a(4)"),
        ];
        for (m, v, label) in forbidden {
            assert!(
                !sym(m, v),
                "{label} must NOT be a symmetry of the basketweave-3"
            );
        }
    }

    #[test]
    fn stepped_hexominoes_covers_region() {
        check_covers(
            &stepped_hexominoes::stepped_hexominoes(),
            "stepped-hexominoes",
            BBox::new(v2(-4.0, -4.0), v2(12.0, 10.0)),
        );
    }

    #[test]
    fn stepped_hexominoes_rotated_scaled() {
        check_rotated(
            &stepped_hexominoes::stepped_hexominoes(),
            "stepped-hexominoes(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn stepped_hexominoes_deterministic() {
        check_det(
            &stepped_hexominoes::stepped_hexominoes(),
            "stepped-hexominoes",
            BBox::new(v2(-3.0, -3.0), v2(9.0, 5.0)),
        );
    }

    /// Lattice membership for the stepped-hexominoes table:
    /// Λ = ⟨(3,1),(0,2)⟩ = {(3i, i + 2j) : i, j ∈ ℤ}.
    fn sh_in_lat(x: i64, y: i64) -> bool {
        x.rem_euclid(3) == 0 && (y - x / 3).rem_euclid(2) == 0
    }

    /// The prototile's 12 vertex set (the CCW corners of the emitted
    /// polygon, four of them 180° collinear — the shape test runs on
    /// the unit-edge data).
    const SH_VERTS: [(i64, i64); 12] = [
        (0, 0), (1, 0), (1, 1), (2, 1), (3, 1), (3, 2),
        (3, 3), (2, 3), (1, 3), (1, 2), (0, 2), (0, 1),
    ];

    /// Does M normalize Λ? Every candidate here is in GL(2, ℤ)
    /// (|det| = 1), so both basis images in Λ imply M(Λ) = Λ.
    fn sh_normalizes(m: [i64; 4]) -> bool {
        sh_in_lat(3 * m[0] + m[1], 3 * m[2] + m[3]) && sh_in_lat(2 * m[1], 2 * m[3])
    }

    /// M(S) = S + t for a unique t (the prototile is a bounded
    /// polygon, hence has no nontrivial self-translation)? Returns t,
    /// found by aligning the lexicographic-min vertices and verified
    /// by sorted set comparison.
    fn sh_shape(m: [i64; 4]) -> Option<(i64, i64)> {
        let mut mv: Vec<(i64, i64)> = SH_VERTS
            .iter()
            .map(|&(x, y)| (m[0] * x + m[1] * y, m[2] * x + m[3] * y))
            .collect();
        let min = |v: &[(i64, i64)]| *v.iter().min().unwrap();
        let t = (min(&mv).0 - min(&SH_VERTS).0, min(&mv).1 - min(&SH_VERTS).1);
        let mut moved: Vec<(i64, i64)> = SH_VERTS
            .iter()
            .map(|&(x, y)| (x + t.0, y + t.1))
            .collect();
        mv.sort_unstable();
        moved.sort_unstable();
        (mv == moved).then_some(t)
    }

    /// The exact p1 criterion: g = (M, v) is a symmetry of the tiling
    /// iff M normalizes Λ, M(S) = S + t for some t ∈ Λ, and v ∈ Λ
    /// (one orbit per cell, so the shape match is translation-exact:
    /// the image of the origin tile must land on a lattice tile).
    fn sh_is_sym(m: [i64; 4], v: [i64; 2]) -> bool {
        sh_normalizes(m)
            && sh_shape(m).is_some_and(|t| sh_in_lat(t.0, t.1))
            && sh_in_lat(v[0], v[1])
    }

    #[test]
    fn stepped_hexominoes_wallpaper_group_is_p1() {
        const M_ROT4_270: [i64; 4] = [0, 1, -1, 0]; // R270
        // The seven non-identity D4 candidates, as
        // (M, normalizes Λ, M(S) = S + t). R90/R270/Md/Ma fail the
        // normalizer; R180/Mx/My normalize but break the asymmetric
        // staircase shape.
        let table: [([i64; 4], bool, bool); 7] = [
            (M_ROT4, false, false),     // R90
            (M_ROT2, true, false),      // R180
            (M_ROT4_270, false, false), // R270
            (M_MIRX, true, false),      // mirrors of x = c
            (M_MIRY, true, false),      // mirrors of y = c
            (M_DIAG, false, false),     // mirrors/glides of y = x + c
            (M_ANTI, false, false),     // mirrors/glides of y = −x + c
        ];
        for (m, norm, shape) in table {
            assert_eq!(sh_normalizes(m), norm, "normalizer of {m:?}");
            assert_eq!(sh_shape(m).is_some(), shape, "shape of {m:?}");
        }

        // Every isometry of the tiling is a lattice translation:
        // (I, v) is a symmetry for every v in a wide Λ window, and no
        // non-identity M is a symmetry for any v.
        let window: Vec<[i64; 2]> = (
            -2i64..=2
        )
            .flat_map(|i| {
                (-3i64..=3).map(move |j| {
                    [
                        3 * i,
                        i + 2 * j,
                    ]
                })
            })
            .collect();
        for v in &window {
            assert!(
                sh_is_sym([1, 0, 0, 1], *v),
                "translation {v:?} must be a symmetry"
            );
        }
        for (m, _, _) in table {
            for v in &window {
                assert!(
                    !sh_is_sym(m, *v),
                    "({m:?}, {v:?}) must NOT be a symmetry"
                );
            }
        }

        // Positive sanity at the two basis vectors, negative sanity
        // just off the lattice — including (3,0), which is absent from
        // Λ, so the width-3 vertical bands are not congruent by a
        // tiling symmetry.
        assert!(sh_is_sym([1, 0, 0, 1], [3, 1]));
        assert!(sh_is_sym([1, 0, 0, 1], [0, 2]));
        assert!(!sh_is_sym([1, 0, 0, 1], [0, 1]));
        assert!(!sh_is_sym([1, 0, 0, 1], [3, 0]));
        assert!(!sh_in_lat(3, 0));
    }

    /// Trapezoid-courses (M79) standard trio.
    #[test]
    fn trapezoid_courses_covers_region() {
        check_covers(
            &trapezoid_courses::trapezoid_courses(),
            "trapezoid-courses",
            BBox::new(v2(-5.0, -3.0), v2(11.0, 7.0)),
        );
    }

    #[test]
    fn trapezoid_courses_rotated_scaled() {
        check_rotated(
            &trapezoid_courses::trapezoid_courses(),
            "trapezoid-courses(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn trapezoid_courses_deterministic() {
        check_det(
            &trapezoid_courses::trapezoid_courses(),
            "trapezoid-courses",
            BBox::new(v2(-4.0, -3.0), v2(10.0, 6.0)),
        );
    }

    /// Glide-hexagon-strips (M80) standard trio.
    #[test]
    fn glide_hexagon_strips_covers_region() {
        check_covers(
            &glide_hexagon_strips::glide_hexagon_strips(),
            "glide-hexagon-strips",
            BBox::new(v2(-5.0, -3.0), v2(11.0, 7.0)),
        );
    }

    #[test]
    fn glide_hexagon_strips_rotated_scaled() {
        check_rotated(
            &glide_hexagon_strips::glide_hexagon_strips(),
            "glide-hexagon-strips(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn glide_hexagon_strips_deterministic() {
        check_det(
            &glide_hexagon_strips::glide_hexagon_strips(),
            "glide-hexagon-strips",
            BBox::new(v2(-4.0, -3.0), v2(10.0, 6.0)),
        );
    }

    /// Shifted-trapezoid-courses (M81) standard trio.
    #[test]
    fn shifted_trapezoid_courses_covers_region() {
        check_covers(
            &shifted_trapezoid_courses::shifted_trapezoid_courses(),
            "shifted-trapezoid-courses",
            BBox::new(v2(-5.0, -3.0), v2(11.0, 7.0)),
        );
    }

    #[test]
    fn shifted_trapezoid_courses_rotated_scaled() {
        check_rotated(
            &shifted_trapezoid_courses::shifted_trapezoid_courses(),
            "shifted-trapezoid-courses(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn shifted_trapezoid_courses_deterministic() {
        check_det(
            &shifted_trapezoid_courses::shifted_trapezoid_courses(),
            "shifted-trapezoid-courses",
            BBox::new(v2(-4.0, -3.0), v2(10.0, 6.0)),
        );
    }

    /// Wavy-courses (M82) standard trio.
    #[test]
    fn wavy_courses_covers_region() {
        check_covers(
            &wavy_courses::wavy_courses(),
            "wavy-courses",
            BBox::new(v2(-5.0, -3.0), v2(11.0, 7.0)),
        );
    }

    #[test]
    fn wavy_courses_rotated_scaled() {
        check_rotated(
            &wavy_courses::wavy_courses(),
            "wavy-courses(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn wavy_courses_deterministic() {
        check_det(
            &wavy_courses::wavy_courses(),
            "wavy-courses",
            BBox::new(v2(-4.0, -3.0), v2(10.0, 6.0)),
        );
    }

    /// Chiral-cut-honeycomb (M83) standard trio.
    #[test]
    fn chiral_cut_honeycomb_covers_region() {
        check_covers(
            &chiral_cut_honeycomb::chiral_cut_honeycomb(),
            "chiral-cut-honeycomb",
            BBox::new(v2(-5.0, -3.0), v2(11.0, 7.0)),
        );
    }

    #[test]
    fn chiral_cut_honeycomb_rotated_scaled() {
        check_rotated(
            &chiral_cut_honeycomb::chiral_cut_honeycomb(),
            "chiral-cut-honeycomb(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn chiral_cut_honeycomb_deterministic() {
        check_det(
            &chiral_cut_honeycomb::chiral_cut_honeycomb(),
            "chiral-cut-honeycomb",
            BBox::new(v2(-4.0, -3.0), v2(10.0, 6.0)),
        );
    }

    /// Split-triangle-grid (M84) standard trio.
    #[test]
    fn split_triangle_grid_covers_region() {
        check_covers(
            &split_triangle_grid::split_triangle_grid(),
            "split-triangle-grid",
            BBox::new(v2(-5.0, -3.0), v2(11.0, 7.0)),
        );
    }

    #[test]
    fn split_triangle_grid_rotated_scaled() {
        check_rotated(
            &split_triangle_grid::split_triangle_grid(),
            "split-triangle-grid(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn split_triangle_grid_deterministic() {
        check_det(
            &split_triangle_grid::split_triangle_grid(),
            "split-triangle-grid",
            BBox::new(v2(-4.0, -3.0), v2(10.0, 6.0)),
        );
    }

    /// Alcove-cut-grid (M85) standard trio.
    #[test]
    fn alcove_cut_grid_covers_region() {
        check_covers(
            &alcove_cut_grid::alcove_cut_grid(),
            "alcove-cut-grid",
            BBox::new(v2(-5.0, -3.0), v2(11.0, 7.0)),
        );
    }

    #[test]
    fn alcove_cut_grid_rotated_scaled() {
        check_rotated(
            &alcove_cut_grid::alcove_cut_grid(),
            "alcove-cut-grid(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn alcove_cut_grid_deterministic() {
        check_det(
            &alcove_cut_grid::alcove_cut_grid(),
            "alcove-cut-grid",
            BBox::new(v2(-4.0, -3.0), v2(10.0, 6.0)),
        );
    }

    /// Lattice membership for the trapezoid-courses table, in doubled
    /// (×10, integer) coordinates: Λ10 = ⟨(20,0),(0,10)⟩ =
    /// {(20i, 10j) : i, j ∈ ℤ}.
    fn tc_in_lat(x: i64, y: i64) -> bool {
        x.rem_euclid(20) == 0 && y.rem_euclid(10) == 0
    }

    /// The four cell tiles in ×10 coordinates, CCW:
    /// P (6/10 area, slant rising to the right), mP (its mirror in
    /// x = 1), P' (4/10 area, slant rising), mP' (its mirror in x = 1).
    const TC_TILES: [[(i64, i64); 4]; 4] = [
        [(0, 3), (10, 5), (10, 10), (0, 10)],
        [(10, 5), (20, 3), (20, 10), (10, 10)],
        [(0, 0), (10, 0), (10, 5), (0, 3)],
        [(10, 0), (20, 0), (20, 3), (10, 5)],
    ];

    /// Does M normalize Λ10? Every candidate here is in GL(2, ℤ)
    /// (|det| = 1), so both basis images in Λ10 imply M(Λ10) = Λ10.
    fn tc_normalizes(m: [i64; 4]) -> bool {
        tc_in_lat(20 * m[0], 20 * m[2]) && tc_in_lat(10 * m[1], 10 * m[3])
    }

    /// The image of tile `t` under g = (M, v), as a sorted 4-point set.
    fn tc_image(t: &[(i64, i64); 4], m: [i64; 4], v: [i64; 2]) -> [(i64, i64); 4] {
        let mut out: Vec<(i64, i64)> = t
            .iter()
            .map(|&(x, y)| (m[0] * x + m[1] * y + v[0], m[2] * x + m[3] * y + v[1]))
            .collect();
        out.sort_unstable();
        let mut arr: [(i64, i64); 4] = [(0, 0); 4];
        for (i, p) in out.iter().enumerate() {
            arr[i] = *p;
        }
        arr
    }

    fn tc_sorted(t: &[(i64, i64); 4]) -> [(i64, i64); 4] {
        let mut out = *t;
        out.sort_unstable();
        out
    }

    /// Is the sorted set `a` a translate of the sorted set `u` by a
    /// vector of Λ10? Tries every (a_i, u_j) anchor pair; a match on
    /// the first pair fixes λ and the full-set check confirms it.
    fn tc_translate_of(a: &[(i64, i64); 4], u: &[(i64, i64); 4]) -> bool {
        (0..4).into_iter().flat_map(|i| (0..4).map(move |j| (a[i].0 - u[j].0, a[i].1 - u[j].1))).any(|(dx, dy)| {
            tc_in_lat(dx, dy)
                && (0..4)
                    .all(|k| (a[k].0, a[k].1) == (u[k].0 + dx, u[k].1 + dy))
        })
    }

    /// Does g = (M, v) map every cell tile onto a cell tile plus a
    /// lattice translate — the exact symmetry criterion for a 4-tile
    /// cell (with M normalizing Λ10, the image of the full tiling is
    /// then exactly the tiling)?
    fn tc_maps_cells(m: [i64; 4], v: [i64; 2]) -> bool {
        TC_TILES.iter().all(|t| {
            let a = tc_image(t, m, v);
            TC_TILES
                .iter()
                .any(|u| tc_translate_of(&a, &tc_sorted(u)))
        })
    }

    /// The exact pm criterion: g = (M, v) is a symmetry of the tiling
    /// iff M normalizes Λ10 and it maps every cell tile onto a cell
    /// tile plus a lattice translate.
    fn tc_is_sym(m: [i64; 4], v: [i64; 2]) -> bool {
        tc_normalizes(m) && tc_maps_cells(m, v)
    }

    /// Exact pm probe: mirrors at x = integer are symmetries; half-turns,
    /// horizontal mirrors/glides, the 90° rotations, the diagonal and
    /// anti-diagonal mirrors/glides, off-lattice translations, and the
    /// half-integer vertical mirrors are all not — mirroring the full
    /// Python battery (`m79_verify.py`: 15 positive / 53 negative).
    #[test]
    fn trapezoid_courses_wallpaper_group_is_pm() {
        // Structural sanity: the mirror x = 1 pairs the two chiral
        // trapezoids exactly, with no translate at all — P ↔ mP and
        // P' ↔ mP' (hence x = 1 is a genuine mirror line of the tiling).
        assert_eq!(tc_image(&TC_TILES[0], M_MIRX, [20, 0]), tc_sorted(&TC_TILES[1]));
        assert_eq!(tc_image(&TC_TILES[1], M_MIRX, [20, 0]), tc_sorted(&TC_TILES[0]));
        assert_eq!(tc_image(&TC_TILES[2], M_MIRX, [20, 0]), tc_sorted(&TC_TILES[3]));
        assert_eq!(tc_image(&TC_TILES[3], M_MIRX, [20, 0]), tc_sorted(&TC_TILES[2]));

        // Positive: the lattice translations (a window of Λ10).
        for i in -1..=1 {
            for j in -1..=1 {
                assert!(
                    tc_is_sym([1, 0, 0, 1], [20 * i, 10 * j]),
                    "translation ({}i, {}j) must be a symmetry",
                    i,
                    j
                );
            }
        }
        // Positive: the vertical mirror lines x = k (k = −1..2), and
        // their composites with lattice translations.
        for c in -1i64..=2 {
            assert!(tc_is_sym(M_MIRX, [20 * c, 0]), "mirror x = {c} must be a symmetry");
        }
        assert!(tc_is_sym(M_MIRX, [0, 10]), "mirror x=0 + (0,1)");
        assert!(tc_is_sym(M_MIRX, [20, -10]), "mirror x=1 + (0,−1)");

        // Negative: off-lattice translations.
        for v in [
            [10, 0],
            [0, 5],
            [5, 5],
            [20, 5],
            [10, 10],
        ] {
            assert!(!tc_is_sym([1, 0, 0, 1], v), "off-lattice translation {v:?}");
        }
        // Negative: half-integer vertical mirrors (x = k + 1/2).
        for c in [-10i64, 10, 30] {
            assert!(!tc_is_sym(M_MIRX, [c, 0]), "mirror x = {c}/10 must not be");
        }
        // Negative: half-turns about the 18 candidate centers from the
        // Python battery (every center mod Λ/2 that the pattern could
        // plausibly carry), given in exact ×10 integer coordinates:
        // half-turn g(p) = 2c − p has v = 2c, so v×10 = (2cx10, 2cy10).
        for (cx10, cy10) in [
            (10, 5),
            (5, 5),
            (15, 5),
            (0, 5),
            (5, 0),
            (10, 0),
            (0, 0),
            (20, 0),
            (0, 10),
            (20, 10),
            (10, 10),
            (5, 10),
            (15, 10),
            (20, 5),
            (0, 3),
            (10, 7),
            (15, 3),
            (5, 7),
        ] {
            let v = [2 * cx10, 2 * cy10];
            assert!(
                !tc_is_sym(M_ROT2, v),
                "half-turn about ({cx10},{cy10})/10 must not be a symmetry"
            );
        }
        // Negative: horizontal mirrors and horizontal glides
        // ((x, 2c−y) + (s, 0) for s ∈ {0, 1, 2}, c ∈ {0, 1/2, 1}).
        for s in [0i64, 10, 20] {
            for c in [0i64, 10, 20] {
                assert!(!tc_is_sym(M_MIRY, [s, c]), "horizontal mirror/glide ({s},{c})");
            }
        }
        // Negative: 90° rotations (R90 and R270 about candidate centers;
        // neither normalizes Λ10 in the first place).
        const M_ROT4_270: [i64; 4] = [0, 1, -1, 0];
        for v in [[0, 0], [20, 10], [20, 0], [0, 10]] {
            assert!(!tc_is_sym(M_ROT4, v), "rot90 at {v:?}");
            assert!(!tc_is_sym(M_ROT4_270, v), "rot270 at {v:?}");
        }
        // Negative: diagonal and anti-diagonal mirrors/glides.
        for v in [[0, 0], [5, 5], [10, 10]] {
            assert!(!tc_is_sym(M_DIAG, v), "diagonal mirror/glide at {v:?}");
        }
        for v in [[0, 0], [10, 10], [15, 15]] {
            assert!(!tc_is_sym(M_ANTI, v), "anti-diagonal mirror/glide at {v:?}");
        }
        // Negative: vertical glides (mirror x = c + half-period shift s).
        for c in [0i64, 20] {
            for s in [5i64, 15] {
                assert!(!tc_is_sym(M_MIRX, [c, s]), "vertical glide (x={c}/10, shift {s}/10)");
            }
        }

        // Sanity: the two prototile classes are incongruent — different
        // area and different squared-edge multisets (the structural
        // reason half-turns and horizontal mirrors cannot be symmetries).
        let area10 = |t: &[(i64, i64); 4]| -> i64 {
            (0..4)
                .map(|i| t[i].0 * t[(i + 1) % 4].1 - t[(i + 1) % 4].0 * t[i].1)
                .sum::<i64>()
        };
        assert_ne!(area10(&TC_TILES[0]), area10(&TC_TILES[2]), "P and P' areas differ");
    }

    /// Lattice membership for the glide-hexagon-strips table, in ×20
    /// (integer) coordinates: Λ20 = ⟨(20,0),(0,40)⟩ = {(20i, 40j) :
    /// i, j ∈ ℤ}.
    fn gh_in_lat(x: i64, y: i64) -> bool {
        x.rem_euclid(20) == 0 && y.rem_euclid(40) == 0
    }

    /// The four cell tiles in ×20 coordinates, CCW, in cell order
    /// [A, T, B, U]: A/T the fat hexagons (strips 0/1), B/U the waist
    /// hexagons (strips 0/1).
    const GH_TILES: [[(i64, i64); 6]; 4] = [
        [(0, 0), (10, 0), (14, 8), (10, 20), (0, 20), (-5, 10)],
        [(-10, 40), (-15, 30), (-10, 20), (0, 20), (4, 32), (0, 40)],
        [(10, 0), (20, 0), (15, 10), (20, 20), (10, 20), (14, 8)],
        [(0, 40), (4, 32), (0, 20), (10, 20), (5, 30), (10, 40)],
    ];

    /// Does M normalize Λ20? Every candidate here is in GL(2, ℤ)
    /// (|det| = 1), so both basis images in Λ20 imply M(Λ20) = Λ20.
    fn gh_normalizes(m: [i64; 4]) -> bool {
        gh_in_lat(20 * m[0], 20 * m[2]) && gh_in_lat(40 * m[1], 40 * m[3])
    }

    /// The image of tile `t` under g = (M, v), as a sorted 6-point set.
    fn gh_image(t: &[(i64, i64); 6], m: [i64; 4], v: [i64; 2]) -> [(i64, i64); 6] {
        let mut out: Vec<(i64, i64)> = t
            .iter()
            .map(|&(x, y)| (m[0] * x + m[1] * y + v[0], m[2] * x + m[3] * y + v[1]))
            .collect();
        out.sort_unstable();
        let mut arr: [(i64, i64); 6] = [(0, 0); 6];
        for (i, p) in out.iter().enumerate() {
            arr[i] = *p;
        }
        arr
    }

    /// The sorted 6-point set `t + (dx, dy)`.
    fn gh_shifted(t: &[(i64, i64); 6], dx: i64, dy: i64) -> [(i64, i64); 6] {
        let mut out: Vec<(i64, i64)> = t.iter().map(|&(x, y)| (x + dx, y + dy)).collect();
        out.sort_unstable();
        let mut arr: [(i64, i64); 6] = [(0, 0); 6];
        for (i, p) in out.iter().enumerate() {
            arr[i] = *p;
        }
        arr
    }

    fn gh_sorted(t: &[(i64, i64); 6]) -> [(i64, i64); 6] {
        let mut out = *t;
        out.sort_unstable();
        out
    }

    /// Is the sorted set `a` a translate of the sorted set `u` by a
    /// vector of Λ20? Tries every (a_i, u_j) anchor pair; a match on
    /// the first pair fixes λ and the full-set check confirms it.
    fn gh_translate_of(a: &[(i64, i64); 6], u: &[(i64, i64); 6]) -> bool {
        (0..6)
            .into_iter()
            .flat_map(|i| (0..6).map(move |j| (a[i].0 - u[j].0, a[i].1 - u[j].1)))
            .any(|(dx, dy)| {
                gh_in_lat(dx, dy)
                    && (0..6)
                        .all(|k| (a[k].0, a[k].1) == (u[k].0 + dx, u[k].1 + dy))
            })
    }

    /// Does g = (M, v) map every cell tile onto a cell tile plus a
    /// lattice translate — the exact symmetry criterion for a 4-tile
    /// cell (with M normalizing Λ20, the image of the full tiling is
    /// then exactly the tiling)?
    fn gh_maps_cells(m: [i64; 4], v: [i64; 2]) -> bool {
        GH_TILES.iter().all(|t| {
            let a = gh_image(t, m, v);
            GH_TILES.iter().any(|u| gh_translate_of(&a, &gh_sorted(u)))
        })
    }

    /// The exact pg criterion: g = (M, v) is a symmetry of the tiling
    /// iff M normalizes Λ20 and it maps every cell tile onto a cell
    /// tile plus a lattice translate.
    fn gh_is_sym(m: [i64; 4], v: [i64; 2]) -> bool {
        gh_normalizes(m) && gh_maps_cells(m, v)
    }

    /// Exact pg probe: the lattice translations and the single glide
    /// family g·Λ are symmetries; every off-lattice translation coset,
    /// every non-glide horizontal mirror/glide coset, all 16 vertical
    /// mirror/glide cosets, all 16 half-turn cosets, and every
    /// non-normalizing linear part (diagonal and anti-diagonal
    /// mirrors/glides, 90°/270° rotations) are not — mirroring the
    /// full Python battery (`m80_verify.py`: 74 positive candidates,
    /// 62 exact-coset negatives, 6 non-normalizing parts).
    #[test]
    fn glide_hexagon_strips_wallpaper_group_is_pg() {
        // Structural sanity: the glide (M_MIRY, (1/2, 0)) — ×20,
        // (10, 0) — is a symmetry and swaps the strips: g(A) = T +
        // (20,−40), g(T) = A + (0,−40), g(B) = U + (20,−40), g(U) =
        // B + (0,−40), exact vertex-set equality. The strip-1 finger
        // at (4,32)/20 = (1/5,8/5) is the glide image of the strip-0
        // finger at (14,8)/20 = (7/10,2/5) — its x-position is forced
        // by the glide.
        assert_eq!(gh_image(&GH_TILES[0], M_MIRY, [10, 0]), gh_shifted(&GH_TILES[1], 20, -40));
        assert_eq!(gh_image(&GH_TILES[1], M_MIRY, [10, 0]), gh_shifted(&GH_TILES[0], 0, -40));
        assert_eq!(gh_image(&GH_TILES[2], M_MIRY, [10, 0]), gh_shifted(&GH_TILES[3], 20, -40));
        assert_eq!(gh_image(&GH_TILES[3], M_MIRY, [10, 0]), gh_shifted(&GH_TILES[2], 0, -40));

        // Positive: the 9 lattice translations (a window of Λ20).
        for i in -1..=1 {
            for j in -1..=1 {
                assert!(
                    gh_is_sym([1, 0, 0, 1], [20 * i, 40 * j]),
                    "translation ({}i, {}j) must be a symmetry",
                    i,
                    j
                );
            }
        }
        // Positive: the 9 glide samples (M_MIRY, (10 + 20i, 40j)) —
        // the family g·Λ (glide 1/2, axes y = integer).
        for i in -1..=1 {
            for j in -1..=1 {
                assert!(
                    gh_is_sym(M_MIRY, [10 + 20 * i, 40 * j]),
                    "glide ((1/2 + {}i), {}j) must be a symmetry",
                    i,
                    j
                );
            }
        }

        // Complete coset enumeration: for each of I, M_MIRY, M_MIRX,
        // M_ROT2, all 16 v-cosets mod Λ20 (x20 ∈ {0,5,10,15}, y20 ∈
        // {0,10,20,30}). Exactly one coset of I (the identity) and
        // exactly one of M_MIRY (the glide coset (10,0)) are
        // positive; all 16 of M_MIRX and all 16 of M_ROT2 are
        // negative — 64 checks, 2 pos / 62 neg.
        for (mi, m) in [
            [1, 0, 0, 1],
            M_MIRY,
            M_MIRX,
            M_ROT2,
        ]
        .into_iter()
        .enumerate()
        {
            for &x20 in &[0i64, 5, 10, 15] {
                for &y20 in &[0i64, 10, 20, 30] {
                    let expected = match mi {
                        0 => x20 == 0 && y20 == 0,
                        1 => x20 == 10 && y20 == 0,
                        _ => false,
                    };
                    assert_eq!(
                        gh_is_sym(m, [x20, y20]),
                        expected,
                        "coset ({x20},{y20}) of linear part {mi}"
                    );
                }
            }
        }

        // Non-normalizing linear parts: R90, R270, diagonal and
        // anti-diagonal mirror/glide linear parts send (20,0) to
        // (0,±20) ∉ Λ20, so no such isometry acts at any center.
        const M_ROT4_270: [i64; 4] = [0, 1, -1, 0];
        for m in [M_ROT4, M_ROT4_270, M_DIAG, M_ANTI] {
            assert!(!gh_normalizes(m), "part {m:?} must not normalize Λ20");
            for v in [[0, 0], [20, 20], [20, 0], [0, 40], [10, 20]] {
                assert!(!gh_is_sym(m, v), "{m:?} at {v:?}");
            }
        }

        // Sanity: the ×20 cell tiles are CCW with doubled areas 580,
        // 580, 220, 220 (areas 290, 290, 110, 110 = 29/40, 29/40,
        // 11/40, 11/40 times 20²), summing to 800 = 20² · det Λ.
        let gh_area2 = |t: &[(i64, i64); 6]| -> i64 {
            (0..6)
                .map(|i| t[i].0 * t[(i + 1) % 6].1 - t[(i + 1) % 6].0 * t[i].1)
                .sum::<i64>()
        };
        assert_eq!(gh_area2(&GH_TILES[0]), 580, "A ×20 doubled area");
        assert_eq!(gh_area2(&GH_TILES[1]), 580, "T ×20 doubled area");
        assert_eq!(gh_area2(&GH_TILES[2]), 220, "B ×20 doubled area");
        assert_eq!(gh_area2(&GH_TILES[3]), 220, "U ×20 doubled area");
        assert_eq!(
            gh_area2(&GH_TILES[0])
                + gh_area2(&GH_TILES[1])
                + gh_area2(&GH_TILES[2])
                + gh_area2(&GH_TILES[3]),
            1600,
            "cell doubled areas sum to 2·20²·det"
        );
    }

    /// Lattice membership for the shifted-trapezoid-courses table, in
    /// ×10 (integer) coordinates: Λ10 = ⟨(20,0),(0,20)⟩ = {(20i, 20j)
    /// : i, j ∈ ℤ}.
    fn st_in_lat(x: i64, y: i64) -> bool {
        x.rem_euclid(20) == 0 && y.rem_euclid(20) == 0
    }

    /// The eight cell tiles in ×10 coordinates, CCW, in cell order
    /// [P, mP, P', mP', R0, R1, R2, R3]: row 0 (y ∈ [0,10]) is the
    /// trapezoid-course row; row 1 (y ∈ [10,20]) is its reflection
    /// F(x,y) = (x, 2−y) shifted by (1,0). R2, R3 straddle the cell's
    /// right edge (x up to 30).
    const ST_TILES: [[(i64, i64); 4]; 8] = [
        [(0, 3), (10, 5), (10, 10), (0, 10)],
        [(10, 5), (20, 3), (20, 10), (10, 10)],
        [(0, 0), (10, 0), (10, 5), (0, 3)],
        [(10, 0), (20, 0), (20, 3), (10, 5)],
        [(10, 10), (20, 10), (20, 15), (10, 17)],
        [(10, 17), (20, 15), (20, 20), (10, 20)],
        [(20, 10), (30, 10), (30, 17), (20, 15)],
        [(20, 15), (30, 17), (30, 20), (20, 20)],
    ];

    /// Does M normalize Λ10? Every candidate here is in GL(2, ℤ)
    /// (|det| = 1), so both basis images in Λ10 imply M(Λ10) = Λ10.
    fn st_normalizes(m: [i64; 4]) -> bool {
        st_in_lat(20 * m[0], 20 * m[2]) && st_in_lat(20 * m[1], 20 * m[3])
    }

    /// The image of tile `t` under g = (M, v), as a sorted 4-point set.
    fn st_image(t: &[(i64, i64); 4], m: [i64; 4], v: [i64; 2]) -> [(i64, i64); 4] {
        let mut out: Vec<(i64, i64)> = t
            .iter()
            .map(|&(x, y)| (m[0] * x + m[1] * y + v[0], m[2] * x + m[3] * y + v[1]))
            .collect();
        out.sort_unstable();
        let mut arr: [(i64, i64); 4] = [(0, 0); 4];
        for (i, p) in out.iter().enumerate() {
            arr[i] = *p;
        }
        arr
    }

    fn st_sorted(t: &[(i64, i64); 4]) -> [(i64, i64); 4] {
        let mut out = *t;
        out.sort_unstable();
        out
    }

    /// The sorted 4-point set `t + (dx, dy)`.
    fn st_shifted(t: &[(i64, i64); 4], dx: i64, dy: i64) -> [(i64, i64); 4] {
        let mut out: Vec<(i64, i64)> = t.iter().map(|&(x, y)| (x + dx, y + dy)).collect();
        out.sort_unstable();
        let mut arr: [(i64, i64); 4] = [(0, 0); 4];
        for (i, p) in out.iter().enumerate() {
            arr[i] = *p;
        }
        arr
    }

    /// Is the sorted set `a` a translate of the sorted set `u` by a
    /// vector of Λ10? Tries every (a_i, u_j) anchor pair; a match on
    /// the first pair fixes λ and the full-set check confirms it.
    fn st_translate_of(a: &[(i64, i64); 4], u: &[(i64, i64); 4]) -> bool {
        (0..4)
            .into_iter()
            .flat_map(|i| (0..4).map(move |j| (a[i].0 - u[j].0, a[i].1 - u[j].1)))
            .any(|(dx, dy)| {
                st_in_lat(dx, dy)
                    && (0..4)
                        .all(|k| (a[k].0, a[k].1) == (u[k].0 + dx, u[k].1 + dy))
            })
    }

    /// Does g = (M, v) map every cell tile onto a cell tile plus a
    /// lattice translate — the exact symmetry criterion for an 8-tile
    /// cell (with M normalizing Λ10, the image of the full tiling is
    /// then exactly the tiling)?
    fn st_maps_cells(m: [i64; 4], v: [i64; 2]) -> bool {
        ST_TILES.iter().all(|t| {
            let a = st_image(t, m, v);
            ST_TILES.iter().any(|u| st_translate_of(&a, &st_sorted(u)))
        })
    }

    /// The exact pmg criterion: g = (M, v) is a symmetry of the tiling
    /// iff M normalizes Λ10 and it maps every cell tile onto a cell
    /// tile plus a lattice translate.
    fn st_is_sym(m: [i64; 4], v: [i64; 2]) -> bool {
        st_normalizes(m) && st_maps_cells(m, v)
    }

    /// Exact pmg probe: the lattice translations, the vertical mirror
    /// lines x = k, the glide family g·Λ (g(x,y) = (x+1, −y), glide 1,
    /// axes y = integer), and the half-turns about (m+1/2, n) — for
    /// ALL m, n ∈ ℤ, including the even-y centers — are symmetries;
    /// the complete 128-coset enumeration (all eight D4 linear parts
    /// that normalize Λ10 × 16 cosets) shows no other isometry coset
    /// is positive, and every named negative from the Python battery
    /// (`m81_verify.py`: 48 positive candidates, 128-coset
    /// enumeration 4 pos / 124 neg) is killed.
    #[test]
    fn shifted_trapezoid_courses_wallpaper_group_is_pmg() {
        // Structural sanity: the mirror x = 1 pairs the chiral
        // trapezoids within each row (P ↔ mP, P' ↔ mP'), with no
        // translate at all.
        assert_eq!(st_image(&ST_TILES[0], M_MIRX, [20, 0]), st_sorted(&ST_TILES[1]));
        assert_eq!(st_image(&ST_TILES[1], M_MIRX, [20, 0]), st_sorted(&ST_TILES[0]));
        assert_eq!(st_image(&ST_TILES[2], M_MIRX, [20, 0]), st_sorted(&ST_TILES[3]));
        assert_eq!(st_image(&ST_TILES[3], M_MIRX, [20, 0]), st_sorted(&ST_TILES[2]));
        // The glide (M_MIRY, (1, 2)) — ×10, (10, 20): (x, 2−y) — maps
        // row 0 onto row 1 exactly: P → R0, P' → R1, mP → R2,
        // mP' → R3 (no translate). The same family on the axis y = 0,
        // (x, −y) — ×10, (10, 0) — maps P → R0 + (0,−2).
        assert_eq!(st_image(&ST_TILES[0], M_MIRY, [10, 20]), st_sorted(&ST_TILES[4]));
        assert_eq!(st_image(&ST_TILES[2], M_MIRY, [10, 20]), st_sorted(&ST_TILES[5]));
        assert_eq!(st_image(&ST_TILES[1], M_MIRY, [10, 20]), st_sorted(&ST_TILES[6]));
        assert_eq!(st_image(&ST_TILES[3], M_MIRY, [10, 20]), st_sorted(&ST_TILES[7]));
        assert_eq!(st_image(&ST_TILES[0], M_MIRY, [10, 0]), st_shifted(&ST_TILES[4], 0, -20));
        // The half-turn about (1/2, 1) — ×10, v = (10, 20) — maps P
        // to R2 + (−2,0); the EVEN-Y centre (1/2, 0) — v = (10, 0) —
        // maps P to R2 + (−2,−2) (the even-y centers are centers too:
        // four per 2×2 cell, (m+1/2, n) for all m, n).
        assert_eq!(
            st_image(&ST_TILES[0], M_ROT2, [10, 20]),
            st_shifted(&ST_TILES[6], -20, 0)
        );
        assert_eq!(
            st_image(&ST_TILES[0], M_ROT2, [10, 0]),
            st_shifted(&ST_TILES[6], -20, -20)
        );

        // Positive: the 9 lattice translations (a window of Λ10).
        for i in -1..=1 {
            for j in -1..=1 {
                assert!(
                    st_is_sym([1, 0, 0, 1], [20 * i, 20 * j]),
                    "translation ({}i, {}j) must be a symmetry",
                    i,
                    j
                );
            }
        }
        // Positive: the vertical mirror lines x = k (k = −1..2), and
        // their composites with lattice translations.
        for c in -1i64..=2 {
            assert!(st_is_sym(M_MIRX, [20 * c, 0]), "mirror x = {c} must be a symmetry");
        }
        assert!(st_is_sym(M_MIRX, [0, 20]), "mirror x=0 + (0,2)");
        assert!(st_is_sym(M_MIRX, [20, -20]), "mirror x=1 + (0,−2)");
        // Positive: the 9 glide samples (M_MIRY, (10 + 20i, 20j)) —
        // the family g·Λ, (x+1+2i, −y+2j) (glide 1, axes y = integer).
        for i in -1..=1 {
            for j in -1..=1 {
                assert!(
                    st_is_sym(M_MIRY, [10 + 20 * i, 20 * j]),
                    "glide (1 + {}i, {}j) must be a symmetry",
                    i,
                    j
                );
            }
        }
        // Positive: the 9 half-turns (M_ROT2, (10 + 20m, 20n)) —
        // centers (1/2 + m, n), ALL integers m, n.
        for m in -1..=1 {
            for n in -1..=1 {
                assert!(
                    st_is_sym(M_ROT2, [10 + 20 * m, 20 * n]),
                    "half-turn ({} + 1/2, {}j) must be a symmetry",
                    m,
                    n
                );
            }
        }

        // Complete coset enumeration: for each of the eight D4 linear
        // parts that normalize Λ10 (a square-metric lattice of side
        // 20), all 16 v-cosets mod Λ10 (x10, y10 ∈ {0,5,10,15}).
        // Exactly one coset of I (the identity), one of M_MIRX (the
        // mirrors x = k), one of M_MIRY (the glide coset (10,0)) and
        // one of M_ROT2 (the half-turn coset (10,0)) are positive; all
        // 16 cosets of the four D4-extra parts (rot90, rot270,
        // diagonal, anti-diagonal) are negative — 128 checks, 4 pos /
        // 124 neg (no p4/p4m/p4g/cmm upgrade).
        for (pi, m) in [
            [1, 0, 0, 1],
            M_MIRX,
            M_MIRY,
            M_ROT2,
            M_ROT4,
            [0, 1, -1, 0],
            M_DIAG,
            M_ANTI,
        ]
        .into_iter()
        .enumerate()
        {
            for &x10 in &[0i64, 5, 10, 15] {
                for &y10 in &[0i64, 5, 10, 15] {
                    let expected = match pi {
                        0 => x10 == 0 && y10 == 0,
                        1 => x10 == 0 && y10 == 0,
                        2 | 3 => x10 == 10 && y10 == 0,
                        _ => false,
                    };
                    assert_eq!(
                        st_is_sym(m, [x10, y10]),
                        expected,
                        "coset ({x10},{y10}) of linear part {pi}"
                    );
                }
            }
        }

        // Named negatives (from the Python battery): off-lattice
        // translations.
        for v in [[10, 0], [0, 10], [10, 10], [5, 0]] {
            assert!(!st_is_sym([1, 0, 0, 1], v), "off-lattice translation {v:?}");
        }
        // Horizontal mirrors: y = 0 is the unshifted flip F (lands P
        // at R0 + (−1,0), half a period off); y = 1/2 likewise.
        assert!(!st_is_sym(M_MIRY, [0, 0]), "horizontal mirror y = 0 must not be");
        assert!(!st_is_sym(M_MIRY, [0, 10]), "horizontal mirror y = 1/2 must not be");
        // The glide-1/2 (x + 1/2, −y): right parity, wrong slant
        // endpoint image.
        assert!(!st_is_sym(M_MIRY, [5, 0]), "glide (1/2, axis y = 0) must not be");
        // Vertical mirrors/glides off the x = k family.
        assert!(!st_is_sym(M_MIRX, [10, 0]), "vertical mirror x = 1/2 must not be");
        assert!(!st_is_sym(M_MIRX, [0, 10]), "vertical glide (x = 0, slide (0,1))");
        assert!(!st_is_sym(M_MIRX, [10, 10]), "vertical glide (x = 1/2, slide (0,1))");
        // Off-set half-turn centers: (0,0), (1/2,1/2), (0,1/2),
        // (1,1/2) — v = 2·center ×10.
        for v in [[0, 0], [10, 10], [0, 10], [20, 10]] {
            assert!(!st_is_sym(M_ROT2, v), "off-set half-turn v = {v:?}");
        }

        // Sanity: the ×10 cell tiles are CCW with doubled areas 120,
        // 120, 80, 80, 120, 80, 120, 80 (areas 6/10, 6/10, 4/10, 4/10,
        // 6/10, 4/10, 6/10, 4/10), summing to 800 = 10² · det Λ.
        let st_area2 = |t: &[(i64, i64); 4]| -> i64 {
            (0..4)
                .map(|i| t[i].0 * t[(i + 1) % 4].1 - t[(i + 1) % 4].0 * t[i].1)
                .sum::<i64>()
        };
        let areas: Vec<i64> = ST_TILES.iter().map(st_area2).collect();
        assert_eq!(
            areas,
            vec![120, 120, 80, 80, 120, 80, 120, 80],
            "cell doubled areas (P, mP, P', mP', R0, R1, R2, R3)"
        );
        assert_eq!(areas.iter().sum::<i64>(), 800, "cell doubled areas sum to 10²·det");
    }

    /// Lattice membership for the wavy-courses table, in ×3 (integer)
    /// coordinates: Λ3 = ⟨(3,3),(6,0)⟩ = {(x,y) : y ≡ 0 (mod 3), x−y ≡
    /// 0 (mod 6)} — the x+y-even lattice of ℤ², det 18.
    fn wc_in_lat(x: i64, y: i64) -> bool {
        y.rem_euclid(3) == 0 && (x - y).rem_euclid(6) == 0
    }

    /// The four cell tiles in ×3 coordinates, CCW, in cell order
    /// [T_low, T_high, T_low', T_high']: column 0 (even parity) and
    /// column 1 (odd parity) of course 0.
    const WC_TILES: [[(i64, i64); 3]; 4] = [
        [(0, 0), (3, 1), (3, 3)],
        [(0, 0), (3, 3), (0, 4)],
        [(3, 1), (6, 0), (3, 3)],
        [(3, 3), (6, 0), (6, 4)],
    ];

    /// Does M normalize Λ3? Every candidate here is in GL(2, ℤ)
    /// (|det| = 1), so both basis images in Λ3 imply M(Λ3) = Λ3 (the
    /// lattice is square-metric, so all eight D4 parts normalize it).
    fn wc_normalizes(m: [i64; 4]) -> bool {
        wc_in_lat(3 * m[0] + 3 * m[1], 3 * m[2] + 3 * m[3])
            && wc_in_lat(6 * m[0], 6 * m[2])
    }

    /// The image of tile `t` under g = (M, v), as a sorted 3-point set.
    fn wc_image(t: &[(i64, i64); 3], m: [i64; 4], v: [i64; 2]) -> [(i64, i64); 3] {
        let mut out: Vec<(i64, i64)> = t
            .iter()
            .map(|&(x, y)| (m[0] * x + m[1] * y + v[0], m[2] * x + m[3] * y + v[1]))
            .collect();
        out.sort_unstable();
        let mut arr: [(i64, i64); 3] = [(0, 0); 3];
        for (i, p) in out.iter().enumerate() {
            arr[i] = *p;
        }
        arr
    }

    fn wc_sorted(t: &[(i64, i64); 3]) -> [(i64, i64); 3] {
        let mut out = *t;
        out.sort_unstable();
        out
    }

    /// The sorted 3-point set `t + (dx, dy)`.
    fn wc_shifted(t: &[(i64, i64); 3], dx: i64, dy: i64) -> [(i64, i64); 3] {
        let mut out: Vec<(i64, i64)> = t.iter().map(|&(x, y)| (x + dx, y + dy)).collect();
        out.sort_unstable();
        let mut arr: [(i64, i64); 3] = [(0, 0); 3];
        for (i, p) in out.iter().enumerate() {
            arr[i] = *p;
        }
        arr
    }

    /// Is the sorted set `a` a translate of the sorted set `u` by a
    /// vector of Λ3? Tries every (a_i, u_j) anchor pair; a match on the
    /// first pair fixes λ and the full-set check confirms it.
    fn wc_translate_of(a: &[(i64, i64); 3], u: &[(i64, i64); 3]) -> bool {
        (0..3)
            .into_iter()
            .flat_map(|i| (0..3).map(move |j| (a[i].0 - u[j].0, a[i].1 - u[j].1)))
            .any(|(dx, dy)| {
                wc_in_lat(dx, dy)
                    && (0..3).all(|k| (a[k].0, a[k].1) == (u[k].0 + dx, u[k].1 + dy))
            })
    }

    /// Does g = (M, v) map every cell tile onto a cell tile plus a
    /// lattice translate — the exact symmetry criterion for a 4-tile
    /// cell (with M normalizing Λ3, the image of the full tiling is
    /// then exactly the tiling)?
    fn wc_maps_cells(m: [i64; 4], v: [i64; 2]) -> bool {
        WC_TILES.iter().all(|t| {
            let a = wc_image(t, m, v);
            WC_TILES.iter().any(|u| wc_translate_of(&a, &wc_sorted(u)))
        })
    }

    /// The exact cm criterion: g = (M, v) is a symmetry of the tiling
    /// iff M normalizes Λ3 and it maps every cell tile onto a cell
    /// tile plus a lattice translate.
    fn wc_is_sym(m: [i64; 4], v: [i64; 2]) -> bool {
        wc_normalizes(m) && wc_maps_cells(m, v)
    }

    /// Exact cm probe: the lattice translations, the vertical mirror
    /// lines x = k, and the glide family g_j·Λ (g_j(x,y) = (2j+1−x,
    /// y+1), glide (0,1), half-integer axes) are symmetries; the
    /// complete 144-coset enumeration (all eight D4 linear parts that
    /// normalize the square-metric Λ3 × 18 cosets, x3 ∈ {0..5},
    /// y3 ∈ {0..2}) shows no other isometry coset is positive —
    /// exactly the identity coset and the mirror/glide coset (2 pos /
    /// 142 neg; the glide representatives (6j+3,3) lie in the same
    /// coset as the mirrors (6j,0) because (3,3) ∈ Λ3) — and every
    /// named negative from the Python battery (`m82_verify.py`) is
    /// killed.
    #[test]
    fn wavy_courses_wallpaper_group_is_cm() {
        // Structural sanity: the mirror x = 0 (M_MIRX, v = (0,0))
        // swaps the even-column tiles with the odd-column tiles of
        // course 0 — T_low ↔ T_low' + (−2,0), T_high ↔ T_high' +
        // (−2,0), and back (C1 stays C1, C2 stays C2 — no class
        // mixing).
        assert_eq!(wc_image(&WC_TILES[0], M_MIRX, [0, 0]), wc_shifted(&WC_TILES[2], -6, 0));
        assert_eq!(wc_image(&WC_TILES[1], M_MIRX, [0, 0]), wc_shifted(&WC_TILES[3], -6, 0));
        assert_eq!(wc_image(&WC_TILES[2], M_MIRX, [0, 0]), wc_shifted(&WC_TILES[0], -6, 0));
        assert_eq!(wc_image(&WC_TILES[3], M_MIRX, [0, 0]), wc_shifted(&WC_TILES[1], -6, 0));
        // The glide g_0(x,y) = (1−x, y+1) — ×3, (M_MIRX, (3,3)) — maps
        // T_low → T_low' + (−1,1) and T_high → T_high' + (−1,1); its
        // square is the lattice translation (0,2).
        assert_eq!(wc_image(&WC_TILES[0], M_MIRX, [3, 3]), wc_shifted(&WC_TILES[2], -3, 3));
        assert_eq!(wc_image(&WC_TILES[1], M_MIRX, [3, 3]), wc_shifted(&WC_TILES[3], -3, 3));
        // The centering translation (1,1) — ×3, (3,3) — is a symmetry.
        assert!(wc_is_sym([1, 0, 0, 1], [3, 3]), "centering (1,1) must be a symmetry");

        // Positive: the 25 lattice translations (a window of Λ3).
        for i in -2..=2 {
            for j in -2..=2 {
                assert!(
                    wc_is_sym([1, 0, 0, 1], [3 * i + 6 * j, 3 * i]),
                    "translation ({}i + {}j·(2,0)) must be a symmetry",
                    i,
                    j
                );
            }
        }
        // Positive: the vertical mirror lines x = k (k = −2..2), v =
        // (6k, 0) in ×3, and their composites with lattice
        // translations.
        for c in -2i64..=2 {
            assert!(
                wc_is_sym(M_MIRX, [6 * c, 0]),
                "mirror x = {c} must be a symmetry"
            );
        }
        assert!(wc_is_sym(M_MIRX, [0, 6]), "mirror x=0 + (0,2): (0,2) in Λ");
        assert!(wc_is_sym(M_MIRX, [6, -6]), "mirror x=1 + (0,−2)");
        // Positive: the glide samples g_j (j = −1..1) — ×3, v =
        // (6j+3, 3) — axes x = j + 1/2, glide (0,1).
        for j in -1..=1 {
            assert!(
                wc_is_sym(M_MIRX, [6 * j + 3, 3]),
                "glide axis x = {}+1/2 must be a symmetry",
                j
            );
        }

        // Complete coset enumeration: for each of the eight D4 linear
        // parts that normalize Λ3 (a square-metric lattice), all 18
        // v-cosets mod Λ3 (x3 ∈ 0..5, y3 ∈ 0..2). For a fixed part m
        // that normalizes Λ3, the group elements with part m form
        // either nothing or a single coset of Λ3, so one rep per coset
        // is the complete census. Exactly one coset of I (the
        // identity) and one of M_MIRX (the mirrors x = k together with
        // the glides (6j+3,3) — (3,3) ∈ Λ3 puts them in ONE coset) are
        // positive; all 18 cosets of the other six parts are negative
        // — 144 checks, 2 pos / 142 neg (no p2/cmm/p4 upgrade).
        const M_ROT4_270: [i64; 4] = [0, 1, -1, 0];
        for (pi, m) in [
            [1, 0, 0, 1],
            M_MIRX,
            M_MIRY,
            M_ROT2,
            M_ROT4,
            M_ROT4_270,
            M_DIAG,
            M_ANTI,
        ]
        .into_iter()
        .enumerate()
        {
            for x3 in 0..6 {
                for y3 in 0..3 {
                    let expected = match pi {
                        0 => x3 == 0 && y3 == 0,
                        1 => x3 == 0 && y3 == 0,
                        _ => false,
                    };
                    assert_eq!(
                        wc_is_sym(m, [x3, y3]),
                        expected,
                        "coset ({x3},{y3}) of linear part {pi}"
                    );
                }
            }
        }

        // Named negatives (from the Python battery): off-lattice
        // translations.
        for v in [[3, 0], [0, 3], [3, 1], [0, 1]] {
            assert!(!wc_is_sym([1, 0, 0, 1], v), "off-lattice translation {v:?}");
        }
        // Vertical mirror off the x = k family.
        assert!(!wc_is_sym(M_MIRX, [3, 0]), "vertical mirror x = 1/2 must not be");
        // Vertical glides with the wrong slide: the integer-axis
        // glide (0,1) and the half-integer-axis glide (0,2) are NOT
        // symmetries (the glide coset is the one containing (3,3)).
        assert!(!wc_is_sym(M_MIRX, [0, 3]), "vertical glide (x = 0, slide (0,1))");
        assert!(!wc_is_sym(M_MIRX, [3, 6]), "vertical glide (x = 1/2, slide (0,2))");
        // Horizontal mirrors and the horizontal glide.
        assert!(!wc_is_sym(M_MIRY, [0, 0]), "horizontal mirror y = 0 must not be");
        assert!(!wc_is_sym(M_MIRY, [0, 3]), "horizontal mirror y = 1/2 must not be");
        assert!(!wc_is_sym(M_MIRY, [3, 3]), "horizontal glide (axis y = 1/2) must not be");
        // Half-turns about the thirds-grid centers the pattern could
        // plausibly carry — v = 2·center in ×3.
        for (cx, cy) in [
            (0, 0),
            (3, 3),
            (0, 3),
            (3, 1),
            (3, 0),
            (6, 0),
            (3, 2),
            (6, 4),
            (0, 2),
            (6, 3),
            (9, 0),
            (6, 6),
        ] {
            assert!(!wc_is_sym(M_ROT2, [2 * cx, 2 * cy]), "half-turn about ({cx},{cy})/3");
        }
        // Extra D4 parts: spot checks (all 18 cosets each already
        // proven negative by the enumeration).
        for m in [M_ROT4, M_ROT4_270, M_DIAG, M_ANTI] {
            for v in [[0, 0], [3, 3], [6, 6], [3, 0], [0, 3]] {
                assert!(!wc_is_sym(m, v), "extra D4 part {m:?} at {v:?}");
            }
        }

        // Sanity: the ×3 cell tiles are CCW with doubled areas 6, 12,
        // 6, 12 (areas 1/3, 2/3, 1/3, 2/3), summing to 36 = 9 · 2 ·
        // det Λ, and the two congruence classes are incongruent (C1
        // carries the 2/3 edge — squared 4 ×9 — where C2 carries the
        // 4/3 edge — squared 16 ×9).
        let wc_area2 = |t: &[(i64, i64); 3]| -> i64 {
            (0..3)
                .map(|i| t[i].0 * t[(i + 1) % 3].1 - t[(i + 1) % 3].0 * t[i].1)
                .sum::<i64>()
        };
        let areas: Vec<i64> = WC_TILES.iter().map(wc_area2).collect();
        assert_eq!(
            areas,
            vec![6, 12, 6, 12],
            "cell doubled areas (T_low, T_high, T_low', T_high')"
        );
        assert_eq!(areas.iter().sum::<i64>(), 36, "cell doubled areas sum to 9·2·det");
        let wc_sq_edges = |t: &[(i64, i64); 3]| -> Vec<i64> {
            let mut e: Vec<i64> = (0..3)
                .map(|i| {
                    let dx = t[i].0 - t[(i + 1) % 3].0;
                    let dy = t[i].1 - t[(i + 1) % 3].1;
                    dx * dx + dy * dy
                })
                .collect();
            e.sort_unstable();
            e
        };
        assert_eq!(
            wc_sq_edges(&WC_TILES[0]),
            vec![4, 10, 18],
            "C1 squared edges ×9 (2/3, sqrt(10)/3, sqrt(2))"
        );
        assert_eq!(
            wc_sq_edges(&WC_TILES[1]),
            vec![10, 16, 18],
            "C2 squared edges ×9 (4/3 edge present)"
        );
    }

    // ============ chiral-cut-honeycomb (M83, p3) ============

    /// The ten distinct cell vertices, ×320, in first-encounter
    /// tile order (O, W0, V0, V1, V2, W2, V3, V4, W4, V5): the
    /// p3 probe candidate set.
    const P3_VERTS: [[i64; 4]; 10] = [
        [0, 0, 0, 0],
        [-40, 128, 160, 0],
        [0, 320, 320, 0],
        [0, 640, 0, 0],
        [0, 320, -320, 0],
        [20, 176, -144, -20],
        [0, -320, -320, 0],
        [0, -640, 0, 0],
        [20, -304, -16, 20],
        [0, -320, 320, 0],
    ];

    /// The three cell tiles, ×320, CCW, 6 vertices each: tile k =
    /// [O, W_{2k}, V_{2k}, V_{2k+1}, V_{2k+2}, W_{2k+4}]. R120 about
    /// O cycles tile0 -> tile1 -> tile2 exactly (no translate).
    const P3_TILES: [[[i64; 4]; 6]; 3] = [
        [
            [0, 0, 0, 0],
            [-40, 128, 160, 0],
            [0, 320, 320, 0],
            [0, 640, 0, 0],
            [0, 320, -320, 0],
            [20, 176, -144, -20],
        ],
        [
            [0, 0, 0, 0],
            [20, 176, -144, -20],
            [0, 320, -320, 0],
            [0, -320, -320, 0],
            [0, -640, 0, 0],
            [20, -304, -16, 20],
        ],
        [
            [0, 0, 0, 0],
            [20, -304, -16, 20],
            [0, -640, 0, 0],
            [0, -320, 320, 0],
            [0, 320, 320, 0],
            [-40, 128, 160, 0],
        ],
    ];

    /// Off-lattice translations that are NOT symmetries (×320):
    /// (√3,0) [the vertex coset], a2/2, (1,0), (3,0), (1/2,0),
    /// (√3, 1+√3) [off-lattice in both slots].
    const P3_TRANSLATION_NEGS: [[i64; 4]; 6] = [
        [0, 0, 320, 0],
        [0, 480, 160, 0],
        [320, 0, 0, 0],
        [960, 0, 0, 0],
        [160, 0, 0, 0],
        [0, 320, 320, 320],
    ];

    /// The 11 D6 linear parts (non-identity), exact ×320, as the
    /// INTEGER matrix 2L (row-major 4×4) in (A,B,C,D) coords: applying
    /// the map to a ×320 point p is (2L·p)/2, exact division on the
    /// vertex set. Re-derived from exact cos/sin values and cross-
    /// checked against the hx_engine D6 definitions in m83_verify.py.
    const P3_LIDENT: [[i64; 4]; 4] = [
        [2, 0, 0, 0],
        [0, 2, 0, 0],
        [0, 0, 2, 0],
        [0, 0, 0, 2],
    ];
    const P3_LROT60: [[i64; 4]; 4] = [
        [1, 0, 0, -3],
        [0, 1, 3, 0],
        [0, -1, 1, 0],
        [1, 0, 0, 1],
    ];
    const P3_LROT120: [[i64; 4]; 4] = [
        [-1, 0, 0, -3],
        [0, -1, 3, 0],
        [0, -1, -1, 0],
        [1, 0, 0, -1],
    ];
    const P3_LROT180: [[i64; 4]; 4] = [
        [-2, 0, 0, 0],
        [0, -2, 0, 0],
        [0, 0, -2, 0],
        [0, 0, 0, -2],
    ];
    const P3_LROT240: [[i64; 4]; 4] = [
        [-1, 0, 0, 3],
        [0, -1, -3, 0],
        [0, 1, -1, 0],
        [-1, 0, 0, -1],
    ];
    const P3_LROT300: [[i64; 4]; 4] = [
        [1, 0, 0, 3],
        [0, 1, -3, 0],
        [0, 1, 1, 0],
        [-1, 0, 0, 1],
    ];
    const P3_LREFL0: [[i64; 4]; 4] = [
        [2, 0, 0, 0],
        [0, -2, 0, 0],
        [0, 0, 2, 0],
        [0, 0, 0, -2],
    ];
    const P3_LREFL30: [[i64; 4]; 4] = [
        [1, 0, 0, 3],
        [0, -1, 3, 0],
        [0, 1, 1, 0],
        [1, 0, 0, -1],
    ];
    const P3_LREFL60: [[i64; 4]; 4] = [
        [-1, 0, 0, 3],
        [0, 1, 3, 0],
        [0, 1, -1, 0],
        [1, 0, 0, 1],
    ];
    const P3_LREFL90: [[i64; 4]; 4] = [
        [-2, 0, 0, 0],
        [0, 2, 0, 0],
        [0, 0, -2, 0],
        [0, 0, 0, 2],
    ];
    const P3_LREFL120: [[i64; 4]; 4] = [
        [-1, 0, 0, -3],
        [0, 1, -3, 0],
        [0, -1, -1, 0],
        [-1, 0, 0, 1],
    ];
    const P3_LREFL150: [[i64; 4]; 4] = [
        [1, 0, 0, -3],
        [0, -1, -3, 0],
        [0, -1, 1, 0],
        [-1, 0, 0, -1],
    ];

    /// Lattice membership in ×320 integer coordinates:
    /// Λ320 = ⟨(0,0,640,0), (0,960,320,0)⟩, i.e. A = D = 0,
    /// B ≡ 0 (mod 960), C − B/3 ≡ 0 (mod 640).
    fn p3_in_lat(v: [i64; 4]) -> bool {
        v[0] == 0
            && v[3] == 0
            && v[1].rem_euclid(960) == 0
            && (v[2] - v[1] / 3).rem_euclid(640) == 0
    }

    /// Coupled-floor coset reduction for points with A = D = 0:
    /// i-coord = (3C−B)/1920, j-coord = B/960, canonical iff both lie
    /// in [0,1)². The two floors are COUPLED (the i-floor depends on
    /// B): decoupled floors misplace representatives.
    fn p3_reduce320(v: [i64; 4]) -> [i64; 4] {
        let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
        let fj = b.div_euclid(960);
        let bm = b - 960 * fj;
        let fi = (3 * c - b).div_euclid(1920);
        let cm = c - 320 * fj - 640 * fi;
        [a, bm, cm, d]
    }

    /// Apply g = (L, v) to a ×320 point: L is given as the integer
    /// matrix 2L (basis images have denominator ≤ 2, so the division
    /// is exact on the vertex set).
    fn p3_apply(l2: &[[i64; 4]; 4], v: [i64; 4], p: [i64; 4]) -> [i64; 4] {
        let mut out = [0i64; 4];
        for i in 0..4 {
            let s = (0..4).map(|j| l2[i][j] * p[j]).sum::<i64>();
            assert!(s % 2 == 0, "2L·p must be even, got {s}");
            out[i] = s / 2 + v[i];
        }
        out
    }

    /// Does g = (L, v) map every cell tile onto a cell tile plus a
    /// lattice translate — the exact symmetry criterion for the
    /// 3-tile cell?
    fn p3_maps_cells(l2: &[[i64; 4]; 4], v: [i64; 4]) -> bool {
        for t in P3_TILES {
            let mut gt: Vec<[i64; 4]> = t.iter().map(|p| p3_apply(l2, v, *p)).collect();
            gt.sort_unstable();
            let mut ok = false;
            for u in P3_TILES {
                let mut us = u.to_vec();
                us.sort_unstable();
                'anchors: for i in 0..6 {
                    for j in 0..6 {
                        let lam = [
                            gt[i][0] - us[j][0],
                            gt[i][1] - us[j][1],
                            gt[i][2] - us[j][2],
                            gt[i][3] - us[j][3],
                        ];
                        if !p3_in_lat(lam) {
                            continue;
                        }
                        let mut all = true;
                        for k in 0..6 {
                            if gt[k][0] != us[k][0] + lam[0]
                                || gt[k][1] != us[k][1] + lam[1]
                                || gt[k][2] != us[k][2] + lam[2]
                                || gt[k][3] != us[k][3] + lam[3]
                            {
                                all = false;
                                break;
                            }
                        }
                        if all {
                            ok = true;
                            break 'anchors;
                        }
                    }
                }
                if ok {
                    break;
                }
            }
            if !ok {
                return false;
            }
        }
        true
    }

    /// Does the linear part normalize the lattice? (Both basis images
    /// in Λ; finite index with |det| = 1 then gives equality.)
    fn p3_normalizes(l2: &[[i64; 4]; 4]) -> bool {
        p3_in_lat(p3_apply(l2, [0; 4], [0, 0, 640, 0]))
            && p3_in_lat(p3_apply(l2, [0; 4], [0, 960, 320, 0]))
    }

    /// The exact p3 criterion: g = (L, v) is a symmetry of the tiling
    /// iff L normalizes Λ320 and it maps every cell tile onto a cell
    /// tile plus a lattice translate.
    fn p3_is_sym(l2: &[[i64; 4]; 4], v: [i64; 4]) -> bool {
        p3_normalizes(l2) && p3_maps_cells(l2, v)
    }

    /// Exact p3 probe (×320 integer scheme, mirroring `m83_verify.py`):
    /// positives are exactly the translations (25/25 in a ±2 window)
    /// and the zero coset of rot120 and of rot240 — everything else is
    /// negative across the full candidate census (the HX-STUDY battery
    /// numbers: 18/27/18/27/18 tested cosets for the five rotations,
    /// 30 for each of the six reflections), six off-lattice
    /// translations are killed, and the three rot120 centre orbits
    /// (hex centre, even corner, odd corner — p3's "333" fingerprint,
    /// all genuinely 3-fold) reduce from the closed form
    /// c(i,j) = (I − R120)⁻¹(i·a1 + j·a2) = (√3·i, i + 2j),
    /// ×320: (0, 320(i+2j), 320i, 0).
    #[test]
    fn chiral_cut_honeycomb_wallpaper_group_is_p3() {
        // Structural sanity: R120 cycles the cell tiles exactly about
        // O (no translate) and R240 = R120².
        let im = |l2: &[[i64; 4]; 4], t: &[[i64; 4]; 6]| -> Vec<[i64; 4]> {
            let mut v: Vec<[i64; 4]> = t.iter().map(|p| p3_apply(l2, [0; 4], *p)).collect();
            v.sort_unstable();
            v
        };
        let sorted = |t: &[[i64; 4]; 6]| -> Vec<[i64; 4]> {
            let mut v = t.to_vec();
            v.sort_unstable();
            v
        };
        assert_eq!(
            im(&P3_LROT120, &P3_TILES[0]),
            sorted(&P3_TILES[1]),
            "R120(tile0) == tile1 exactly"
        );
        assert_eq!(
            im(&P3_LROT240, &P3_TILES[0]),
            sorted(&P3_TILES[2]),
            "R240(tile0) == tile2 exactly"
        );
        assert_eq!(im(&P3_LIDENT, &P3_TILES[0]), sorted(&P3_TILES[0]));

        // Positive: the 25 lattice translations of the ±2 window.
        for i in -2i64..=2 {
            for j in -2i64..=2 {
                assert!(
                    p3_is_sym(&P3_LIDENT, [0, 960 * j, 640 * i + 320 * j, 0]),
                    "lattice translation ({i},{j}) must be a symmetry"
                );
            }
        }
        // Negative: the six off-lattice translations.
        for v in P3_TRANSLATION_NEGS {
            assert!(
                !p3_is_sym(&P3_LIDENT, v),
                "off-lattice {v:?} must not be a symmetry"
            );
        }

        // Full candidate census for the 11 non-identity D6 parts:
        // candidates = {q − L·p : p, q in the 10-vertex set}, reduced
        // to one canonical rep per coset. Only the zero coset of
        // rot120 and of rot240 is positive; the other nine parts are
        // entirely negative (no 60°/180°/300° rotations, no mirror in
        // any of the six families, no glides — p3, not p6 or any
        // pmm-family upgrade).
        const PARTS: [(&str, &[[i64; 4]; 4], usize, bool); 11] = [
            ("rot60", &P3_LROT60, 18, false),
            ("rot120", &P3_LROT120, 27, true),
            ("rot180", &P3_LROT180, 18, false),
            ("rot240", &P3_LROT240, 27, true),
            ("rot300", &P3_LROT300, 18, false),
            ("refl0", &P3_LREFL0, 30, false),
            ("refl30", &P3_LREFL30, 30, false),
            ("refl60", &P3_LREFL60, 30, false),
            ("refl90", &P3_LREFL90, 30, false),
            ("refl120", &P3_LREFL120, 30, false),
            ("refl150", &P3_LREFL150, 30, false),
        ];
        for (name, l2, tested, has_pos) in PARTS {
            assert!(p3_normalizes(l2), "{name}: linear part must normalize the lattice");
            let mut cands: Vec<[i64; 4]> = Vec::new();
            for p in P3_VERTS {
                let mp = p3_apply(l2, [0; 4], p);
                for q in P3_VERTS {
                    let v = p3_reduce320([
                        q[0] - mp[0],
                        q[1] - mp[1],
                        q[2] - mp[2],
                        q[3] - mp[3],
                    ]);
                    if !cands.contains(&v) {
                        cands.push(v);
                    }
                }
            }
            assert_eq!(cands.len(), tested, "{name}: tested coset count");
            let mut pos = 0;
            for v in &cands {
                let sym = p3_is_sym(l2, *v);
                if sym {
                    pos += 1;
                }
                assert_eq!(
                    sym,
                    has_pos && *v == [0, 0, 0, 0],
                    "{name}: candidate {v:?}"
                );
            }
            assert_eq!(pos, usize::from(has_pos), "{name}: positive cosets");
        }

        // The three rot120 centre orbits mod the lattice: hex centre,
        // even corner, odd corner — all three genuinely 3-fold.
        let mut orb: Vec<[i64; 4]> = Vec::new();
        for i in -3i64..=3 {
            for j in -3i64..=3 {
                let v = p3_reduce320([0, 320 * (i + 2 * j), 320 * i, 0]);
                if !orb.contains(&v) {
                    orb.push(v);
                }
            }
        }
        orb.sort_unstable();
        let want_orb: [[i64; 4]; 3] = [[0, 0, 0, 0], [0, 320, 320, 0], [0, 640, 640, 0]];
        assert_eq!(orb, want_orb.to_vec(), "rot120 centre orbits mod lattice");
        // Each orbit rep c satisfies c − R120·c ∈ Λ (R120 about c is
        // a symmetry).
        for c in want_orb {
            let d = p3_apply(&P3_LROT120, [0; 4], c);
            assert!(
                p3_in_lat([c[0] - d[0], c[1] - d[1], c[2] - d[2], c[3] - d[3]]),
                "orbit rep {c:?}: c − R120·c in Λ"
            );
        }

        // Cell arithmetic ×320²: doubled area per tile (0, 409600) =
        // 4√3·320², sum (0, 1228800) = 2·det·320², and the
        // squared-edge multiset 2×(409600,0), 2×(94784,−12800),
        // 2×(115264,12800) identical across the three tiles (one
        // congruence class).
        let mut areas: Vec<(i64, i64)> = Vec::new();
        let mut edges: Vec<Vec<(i64, i64)>> = Vec::new();
        for t in P3_TILES {
            let mut area = (0i64, 0i64);
            let mut es: Vec<(i64, i64)> = Vec::new();
            for i in 0..6 {
                let (u, w) = (t[i], t[(i + 1) % 6]);
                area.0 += u[0] * w[1] + 3 * u[2] * w[3] - w[0] * u[1] - 3 * w[2] * u[3];
                area.1 += u[0] * w[3] + u[2] * w[1] - w[0] * u[3] - w[2] * u[1];
                let (da, db, dc, dd) = (w[0] - u[0], w[1] - u[1], w[2] - u[2], w[3] - u[3]);
                es.push((
                    da * da + db * db + 3 * dc * dc + 3 * dd * dd,
                    2 * da * dc + 2 * db * dd,
                ));
            }
            es.sort_unstable();
            edges.push(es.clone());
            areas.push(area);
        }
        assert_eq!(areas, vec![(0, 409600); 3], "doubled tile areas ×320²");
        assert_eq!(
            areas[0].0 + areas[1].0 + areas[2].0,
            0,
            "real part of doubled cell area"
        );
        assert_eq!(
            areas[0].1 + areas[1].1 + areas[2].1,
            1_228_800,
            "sum = 2·det·320²"
        );
        let want_edges: Vec<(i64, i64)> = vec![
            (94784, -12800),
            (94784, -12800),
            (115264, 12800),
            (115264, 12800),
            (409600, 0),
            (409600, 0),
        ];
        for e in &edges {
            assert_eq!(*e, want_edges, "squared-edge multiset per tile");
        }
    }

    // ============ split-triangle-grid (M84, p31m) ============

    /// The ten distinct cell vertices, ×240, in first-encounter tile
    /// order: the p31m probe candidate set.
    const P31M_VERTS: [[i64; 4]; 10] = [
        [0, 0, 0, 0],
        [144, -6, 0, 32],
        [216, 3, -3, 152],
        [240, 0, 0, 240],
        [360, 3, 3, 56],
        [384, 6, 0, 208],
        [456, -3, -3, 88],
        [480, 0, 0, 0],
        [600, -3, 3, 184],
        [720, 0, 0, 240],
    ];

    /// The eight cell tiles, ×240, CCW, JSON order: tile 0 the inner
    /// equilateral triangle of the up-triangle U, tiles 1–3 its corner
    /// quads (one per edge), tile 4 the inner triangle of the
    /// down-triangle D (the mirror image of tile 0), tiles 5–7 its
    /// corner quads. Tiles 0 and 4 are triangles (3 real vertices; the
    /// 4th slot is padding [0,0,0,0]).
    const P31M_TILE_VERTS: [[[i64; 4]; 4]; 8] = [
        [
            [144, -6, 0, 32],
            [360, 3, 3, 56],
            [216, 3, -3, 152],
            [0, 0, 0, 0],
        ],
        [
            [0, 0, 0, 0],
            [480, 0, 0, 0],
            [360, 3, 3, 56],
            [144, -6, 0, 32],
        ],
        [
            [480, 0, 0, 0],
            [240, 0, 0, 240],
            [216, 3, -3, 152],
            [360, 3, 3, 56],
        ],
        [
            [240, 0, 0, 240],
            [0, 0, 0, 0],
            [144, -6, 0, 32],
            [216, 3, -3, 152],
        ],
        [
            [456, -3, -3, 88],
            [600, -3, 3, 184],
            [384, 6, 0, 208],
            [0, 0, 0, 0],
        ],
        [
            [384, 6, 0, 208],
            [600, -3, 3, 184],
            [720, 0, 0, 240],
            [240, 0, 0, 240],
        ],
        [
            [600, -3, 3, 184],
            [456, -3, -3, 88],
            [480, 0, 0, 0],
            [720, 0, 0, 240],
        ],
        [
            [456, -3, -3, 88],
            [384, 6, 0, 208],
            [240, 0, 0, 240],
            [480, 0, 0, 0],
        ],
    ];
    const P31M_TILE_LEN: [usize; 8] = [3, 4, 4, 4, 3, 4, 4, 4];

    /// Off-lattice translations that are NOT symmetries, ×240: (1,0),
    /// (0,1), (1/2,0), (1/2, √3/2), (√3,0).
    const P31M_TRANSLATION_NEGS: [[i64; 4]; 5] = [
        [240, 0, 0, 0],
        [0, 240, 0, 0],
        [120, 0, 0, 0],
        [120, 0, 0, 120],
        [0, 0, 240, 0],
    ];

    /// Lattice membership in ×240 integer coordinates:
    /// Λ240 = ⟨(480,0,0,0), (240,0,0,240)⟩, i.e. B = C = 0,
    /// D ≡ 0 (mod 240), A − D ≡ 0 (mod 480).
    fn p31m_in_lat(v: [i64; 4]) -> bool {
        v[1] == 0
            && v[2] == 0
            && v[3].rem_euclid(240) == 0
            && (v[0] - v[3]).rem_euclid(480) == 0
    }

    /// Coset reduction for ×240 points: j = ⌊D/240⌉ (div_euclid),
    /// subtract j·a2, then i on A−240j mod 480; canonical reps have
    /// A ∈ [0,480), D ∈ [0,240) (B, C pass through — Λ has no B, C
    /// content).
    fn p31m_reduce(v: [i64; 4]) -> [i64; 4] {
        let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
        let j = d.div_euclid(240);
        let dm = d - 240 * j;
        let am = a - 240 * j;
        let i = am.div_euclid(480);
        [am - 480 * i, b, c, dm]
    }

    /// Apply g = (L, v) to a ×240 point: L is given as the integer
    /// matrix 2L (basis images have denominator ≤ 2, so the division
    /// is exact on the vertex set and the centre reps).
    fn p31m_apply(l2: &[[i64; 4]; 4], v: [i64; 4], p: [i64; 4]) -> [i64; 4] {
        let mut out = [0i64; 4];
        for i in 0..4 {
            let s = (0..4).map(|j| l2[i][j] * p[j]).sum::<i64>();
            assert!(s % 2 == 0, "2L·p must be even, got {s}");
            out[i] = s / 2 + v[i];
        }
        out
    }

    /// Does g = (L, v) map every cell tile onto a cell tile plus a
    /// lattice translate — the exact symmetry criterion for the
    /// 8-tile cell? Tiles 0 and 4 are triangles, the other six quads,
    /// so the vertex count of the target must match before anchor
    /// matching.
    fn p31m_maps_cells(l2: &[[i64; 4]; 4], v: [i64; 4]) -> bool {
        for t in 0..8 {
            let n = P31M_TILE_LEN[t];
            let mut gt: Vec<[i64; 4]> = (0..n)
                .map(|k| p31m_apply(l2, v, P31M_TILE_VERTS[t][k]))
                .collect();
            gt.sort_unstable();
            let mut ok = false;
            'tiles: for u in 0..8 {
                if P31M_TILE_LEN[u] != n {
                    continue;
                }
                let mut us: Vec<[i64; 4]> =
                    (0..n).map(|k| P31M_TILE_VERTS[u][k]).collect();
                us.sort_unstable();
                'anchors: for i in 0..n {
                    for j in 0..n {
                        let lam = [
                            gt[i][0] - us[j][0],
                            gt[i][1] - us[j][1],
                            gt[i][2] - us[j][2],
                            gt[i][3] - us[j][3],
                        ];
                        if !p31m_in_lat(lam) {
                            continue;
                        }
                        let mut all = true;
                        for k in 0..n {
                            if gt[k][0] != us[k][0] + lam[0]
                                || gt[k][1] != us[k][1] + lam[1]
                                || gt[k][2] != us[k][2] + lam[2]
                                || gt[k][3] != us[k][3] + lam[3]
                            {
                                all = false;
                                break;
                            }
                        }
                        if all {
                            ok = true;
                            break 'anchors;
                        }
                    }
                }
                if ok {
                    break 'tiles;
                }
            }
            if !ok {
                return false;
            }
        }
        true
    }

    /// Does the linear part normalize the lattice? (Both basis images
    /// in Λ; finite index with |det| = 1 then gives equality.)
    fn p31m_normalizes(l2: &[[i64; 4]; 4]) -> bool {
        p31m_in_lat(p31m_apply(l2, [0; 4], [480, 0, 0, 0]))
            && p31m_in_lat(p31m_apply(l2, [0; 4], [240, 0, 0, 240]))
    }

    /// The exact p31m criterion: g = (L, v) is a symmetry of the
    /// tiling iff L normalizes Λ240 and it maps every cell tile onto a
    /// cell tile plus a lattice translate.
    fn p31m_is_sym(l2: &[[i64; 4]; 4], v: [i64; 4]) -> bool {
        p31m_normalizes(l2) && p31m_maps_cells(l2, v)
    }

    /// Is centre c on one of the tiling's mirror lines? The positive
    /// mirror cosets are exactly the zero cosets of the three grid-line
    /// families refl0/refl60/refl120 (the probe below), whose mirror
    /// lines all pass through the origin, so c lies on a mirror iff
    /// c − L·c ∈ Λ for one of the three linear parts.
    fn p31m_on_mirror(c: [i64; 4]) -> bool {
        for l2 in [&P3_LREFL0, &P3_LREFL60, &P3_LREFL120] {
            let d = p31m_apply(l2, [0; 4], c);
            if p31m_in_lat([c[0] - d[0], c[1] - d[1], c[2] - d[2], c[3] - d[3]]) {
                return true;
            }
        }
        false
    }

    /// Exact p31m probe (×240 integer scheme, mirroring
    /// `m84_verify.py`): positives are exactly the translations
    /// (25/25 in a ±2 window) and the zero coset of rot120, rot240
    /// and the three grid-line mirror families refl0/refl60/refl120 —
    /// everything else negative across the full candidate census
    /// (43 tested cosets for the five positive parts, 28 for the six
    /// negative parts: no 60°/180°/300° rotation, so not p6 or p6m;
    /// no 30°/90°/150° mirror family, no glides). Five off-lattice
    /// translations are killed. The three rot120 centre orbits — grid
    /// vertices, up-triangle centroids, down-triangle centroids —
    /// reduce from the closed form c(i,j) ×240 =
    /// (240i, 0, 0, 80(i+2j)), and the on-mirror incidence is
    /// [grid vertex: on, centroids: off, off]: exactly ONE of the three
    /// orbits on mirrors, which is p31m's fingerprint — p3m1 would put
    /// all three on mirrors.
    #[test]
    fn split_triangle_grid_wallpaper_group_is_p31m() {
        // Positive: the 25 lattice translations of the ±2 window.
        for i in -2i64..=2 {
            for j in -2i64..=2 {
                assert!(
                    p31m_is_sym(&P3_LIDENT, [480 * i + 240 * j, 0, 0, 240 * j]),
                    "lattice translation ({i},{j}) must be a symmetry"
                );
            }
        }
        // Negative: the five off-lattice translations.
        for v in P31M_TRANSLATION_NEGS {
            assert!(
                !p31m_is_sym(&P3_LIDENT, v),
                "off-lattice {v:?} must not be a symmetry"
            );
        }

        // Full candidate census for the 11 non-identity D6 parts:
        // candidates = {q − L·p : p, q in the 10-vertex set}, reduced
        // to one canonical rep per coset. Only the zero coset is
        // positive, and only for the five p31m parts (rot120, rot240,
        // refl0, refl60, refl120).
        const PARTS: [(&str, &[[i64; 4]; 4], usize, bool); 11] = [
            ("rot60", &P3_LROT60, 28, false),
            ("rot120", &P3_LROT120, 43, true),
            ("rot180", &P3_LROT180, 28, false),
            ("rot240", &P3_LROT240, 43, true),
            ("rot300", &P3_LROT300, 28, false),
            ("refl0", &P3_LREFL0, 43, true),
            ("refl30", &P3_LREFL30, 28, false),
            ("refl60", &P3_LREFL60, 43, true),
            ("refl90", &P3_LREFL90, 28, false),
            ("refl120", &P3_LREFL120, 43, true),
            ("refl150", &P3_LREFL150, 28, false),
        ];
        for (name, l2, tested, has_pos) in PARTS {
            assert!(
                p31m_normalizes(l2),
                "{name}: linear part must normalize the lattice"
            );
            let mut cands: Vec<[i64; 4]> = Vec::new();
            for p in P31M_VERTS {
                let mp = p31m_apply(l2, [0; 4], p);
                for q in P31M_VERTS {
                    let v = p31m_reduce([
                        q[0] - mp[0],
                        q[1] - mp[1],
                        q[2] - mp[2],
                        q[3] - mp[3],
                    ]);
                    if !cands.contains(&v) {
                        cands.push(v);
                    }
                }
            }
            assert_eq!(cands.len(), tested, "{name}: tested coset count");
            let mut pos = 0;
            for v in &cands {
                let sym = p31m_is_sym(l2, *v);
                if sym {
                    pos += 1;
                }
                assert_eq!(
                    sym,
                    has_pos && *v == [0, 0, 0, 0],
                    "{name}: candidate {v:?}"
                );
            }
            assert_eq!(pos, usize::from(has_pos), "{name}: positive cosets");
        }

        // The three rot120 centre orbits mod the lattice: grid
        // vertices, up-triangle centroids, down-triangle centroids
        // (closed form c(i,j) ×240 = (240i, 0, 0, 80(i+2j))).
        let mut orb: Vec<[i64; 4]> = Vec::new();
        for i in -3i64..=3 {
            for j in -3i64..=3 {
                let v = p31m_reduce([240 * i, 0, 0, 80 * (i + 2 * j)]);
                if !orb.contains(&v) {
                    orb.push(v);
                }
            }
        }
        orb.sort_unstable();
        let want_orb: [[i64; 4]; 3] = [[0, 0, 0, 0], [0, 0, 0, 160], [240, 0, 0, 80]];
        assert_eq!(orb, want_orb.to_vec(), "rot120 centre orbits mod lattice");
        // Each orbit rep c satisfies c − R120·c ∈ Λ (R120 about c is
        // a symmetry).
        for c in want_orb {
            let d = p31m_apply(&P3_LROT120, [0; 4], c);
            assert!(
                p31m_in_lat([c[0] - d[0], c[1] - d[1], c[2] - d[2], c[3] - d[3]]),
                "orbit rep {c:?}: c − R120·c in Λ"
            );
        }
        // Incidence — the p31m/p3m1 discriminator: exactly one of the
        // three centre orbits lies on a mirror (the grid-vertex orbit);
        // both centroid orbits are off every mirror. In p3m1 all three
        // orbits would be on mirrors.
        assert!(
            p31m_on_mirror([0, 0, 0, 0]),
            "grid-vertex centres lie on the grid-line mirrors"
        );
        assert!(!p31m_on_mirror([240, 0, 0, 80]), "up-centroid centres off every mirror");
        assert!(
            !p31m_on_mirror([0, 0, 0, 160]),
            "down-centroid centres off every mirror"
        );

        // Cell arithmetic ×240²: doubled area (re, im) per tile — the
        // two inner triangles (2592, 24246), the six corner quads
        // (−864, 30318); sum (0, 230400) = 2·det·240² (det = 2√3).
        // Squared-edge multiset: two congruence classes — triangles
        // (48492,1728) × 3, quads (23844,−384) × 2, (48492,1728),
        // (230400,0).
        let mut areas: Vec<(i64, i64)> = Vec::new();
        let mut edges: Vec<Vec<(i64, i64)>> = Vec::new();
        for (t, n) in P31M_TILE_VERTS.iter().zip(P31M_TILE_LEN.iter()) {
            let mut area = (0i64, 0i64);
            let mut es: Vec<(i64, i64)> = Vec::new();
            for i in 0..*n {
                let (u, w) = (t[i], t[(i + 1) % n]);
                area.0 += u[0] * w[1] + 3 * u[2] * w[3] - w[0] * u[1] - 3 * w[2] * u[3];
                area.1 += u[0] * w[3] + u[2] * w[1] - w[0] * u[3] - w[2] * u[1];
                let (da, db, dc, dd) = (
                    w[0] - u[0],
                    w[1] - u[1],
                    w[2] - u[2],
                    w[3] - u[3],
                );
                es.push((
                    da * da + db * db + 3 * dc * dc + 3 * dd * dd,
                    2 * da * dc + 2 * db * dd,
                ));
            }
            es.sort_unstable();
            edges.push(es.clone());
            areas.push(area);
        }
        let want_areas: [(i64, i64); 8] = [
            (2592, 24246),
            (-864, 30318),
            (-864, 30318),
            (-864, 30318),
            (2592, 24246),
            (-864, 30318),
            (-864, 30318),
            (-864, 30318),
        ];
        assert_eq!(areas, want_areas.to_vec(), "doubled tile areas ×240²");
        assert_eq!(
            areas.iter().map(|a| a.0).sum::<i64>(),
            0,
            "real part of doubled cell area"
        );
        assert_eq!(
            areas.iter().map(|a| a.1).sum::<i64>(),
            230_400,
            "sum = 2·det·240² (det 2√3)"
        );
        let want_tri: Vec<(i64, i64)> = vec![(48492, 1728); 3];
        let want_quad: Vec<(i64, i64)> = vec![
            (23844, -384),
            (23844, -384),
            (48492, 1728),
            (230400, 0),
        ];
        for (e, n) in edges.iter().zip(P31M_TILE_LEN.iter()) {
            let want = if *n == 3 { &want_tri } else { &want_quad };
            assert_eq!(
                *e, *want,
                "squared-edge multiset (two congruence classes: achiral triangles, chiral quads)"
            );
        }
    }

    // ============ alcove-cut-grid (M85, p3m1) ============

    /// The 13 distinct cell vertices, ×126, sorted by (A, D): the
    /// p3m1 probe candidate set.
    const P3M1_VERTS: [[i64; 4]; 13] = [
        [0, 0, 0, 0],
        [126, 0, 0, -126],
        [126, 0, 0, 126],
        [144, 0, 0, -56],
        [144, 0, 0, 56],
        [222, 0, 0, -82],
        [222, 0, 0, 82],
        [252, 0, 0, 0],
        [378, 0, 0, -126],
        [378, 0, 0, 126],
        [390, 0, 0, -26],
        [390, 0, 0, 26],
        [504, 0, 0, 0],
    ];

    /// The 18 cell tiles, ×126, CCW, JSON order: each small triangle
    /// (U0, D0, U1, D1, U2, D2 of the side-2 cell) cut into three
    /// triangles from its alcove point. All 18 tiles are triangles.
    const P3M1_TILE_VERTS: [[[i64; 4]; 3]; 18] = [
        [
            [144, 0, 0, 56],
            [0, 0, 0, 0],
            [252, 0, 0, 0],
        ],
        [
            [144, 0, 0, 56],
            [252, 0, 0, 0],
            [126, 0, 0, 126],
        ],
        [
            [144, 0, 0, 56],
            [126, 0, 0, 126],
            [0, 0, 0, 0],
        ],
        [
            [144, 0, 0, -56],
            [126, 0, 0, -126],
            [252, 0, 0, 0],
        ],
        [
            [144, 0, 0, -56],
            [252, 0, 0, 0],
            [0, 0, 0, 0],
        ],
        [
            [144, 0, 0, -56],
            [0, 0, 0, 0],
            [126, 0, 0, -126],
        ],
        [
            [222, 0, 0, 82],
            [126, 0, 0, 126],
            [252, 0, 0, 0],
        ],
        [
            [222, 0, 0, 82],
            [252, 0, 0, 0],
            [378, 0, 0, 126],
        ],
        [
            [222, 0, 0, 82],
            [378, 0, 0, 126],
            [126, 0, 0, 126],
        ],
        [
            [222, 0, 0, -82],
            [378, 0, 0, -126],
            [252, 0, 0, 0],
        ],
        [
            [222, 0, 0, -82],
            [252, 0, 0, 0],
            [126, 0, 0, -126],
        ],
        [
            [222, 0, 0, -82],
            [126, 0, 0, -126],
            [378, 0, 0, -126],
        ],
        [
            [390, 0, 0, 26],
            [378, 0, 0, 126],
            [252, 0, 0, 0],
        ],
        [
            [390, 0, 0, 26],
            [252, 0, 0, 0],
            [504, 0, 0, 0],
        ],
        [
            [390, 0, 0, 26],
            [504, 0, 0, 0],
            [378, 0, 0, 126],
        ],
        [
            [390, 0, 0, -26],
            [504, 0, 0, 0],
            [252, 0, 0, 0],
        ],
        [
            [390, 0, 0, -26],
            [252, 0, 0, 0],
            [378, 0, 0, -126],
        ],
        [
            [390, 0, 0, -26],
            [378, 0, 0, -126],
            [504, 0, 0, 0],
        ],
    ];

    /// Off-lattice translations that are NOT symmetries, ×126: (1,0),
    /// (0,1), (1/2,0), (1/2, √3/2), (√3,0).
    const P3M1_TRANSLATION_NEGS: [[i64; 4]; 5] = [
        [126, 0, 0, 0],
        [0, 126, 0, 0],
        [63, 0, 0, 0],
        [63, 0, 0, 63],
        [0, 0, 126, 0],
    ];

    /// Lattice membership in ×126 integer coordinates:
    /// Λ126 = ⟨(378,0,0,−126), (378,0,0,126)⟩, i.e. B = C = 0,
    /// A ≡ 0 (mod 378), D ≡ 0 (mod 126), (D/126) − (A/378) ≡ 0 (mod 2).
    fn p3m1_in_lat(v: [i64; 4]) -> bool {
        v[1] == 0
            && v[2] == 0
            && v[0].rem_euclid(378) == 0
            && v[3].rem_euclid(126) == 0
            && ((v[3] / 126) - (v[0] / 378)).rem_euclid(2) == 0
    }

    /// Coset reduction for ×126 points: i = ⌊(A+3D)/756⌉,
    /// j = ⌊(A−3D)/756⌉ (div_euclid, matching Python `//`); canonical
    /// reps have A ∈ [0,756), D ∈ [−126,126) (B, C pass through — Λ
    /// has no B, C content).
    fn p3m1_reduce(v: [i64; 4]) -> [i64; 4] {
        let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
        let i = (a + 3 * d).div_euclid(756);
        let j = (a - 3 * d).div_euclid(756);
        [a - 378 * (i + j), b, c, d - 126 * (i - j)]
    }

    /// Apply g = (L, v) to a ×126 point: L is given as the integer
    /// matrix 2L (basis images have denominator ≤ 2, so the division
    /// is exact on the vertex set and the centre reps).
    fn p3m1_apply(l2: &[[i64; 4]; 4], v: [i64; 4], p: [i64; 4]) -> [i64; 4] {
        let mut out = [0i64; 4];
        for i in 0..4 {
            let s = (0..4).map(|j| l2[i][j] * p[j]).sum::<i64>();
            assert!(s % 2 == 0, "2L·p must be even, got {s}");
            out[i] = s / 2 + v[i];
        }
        out
    }

    /// Does g = (L, v) map every cell tile onto a cell tile plus a
    /// lattice translate — the exact symmetry criterion for the
    /// 18-tile cell? All 18 tiles are triangles (3 real vertices
    /// each, no padding).
    fn p3m1_maps_cells(l2: &[[i64; 4]; 4], v: [i64; 4]) -> bool {
        for gtile in &P3M1_TILE_VERTS {
            let mut gt: Vec<[i64; 4]> = (0..3)
                .map(|k| p3m1_apply(l2, v, gtile[k]))
                .collect();
            gt.sort_unstable();
            let mut ok = false;
            'tiles: for utile in &P3M1_TILE_VERTS {
                let mut us: Vec<[i64; 4]> = (0..3).map(|k| utile[k]).collect();
                us.sort_unstable();
                'anchors: for i in 0..3 {
                    for j in 0..3 {
                        let lam = [
                            gt[i][0] - us[j][0],
                            gt[i][1] - us[j][1],
                            gt[i][2] - us[j][2],
                            gt[i][3] - us[j][3],
                        ];
                        if !p3m1_in_lat(lam) {
                            continue;
                        }
                        let mut all = true;
                        for k in 0..3 {
                            if gt[k][0] != us[k][0] + lam[0]
                                || gt[k][1] != us[k][1] + lam[1]
                                || gt[k][2] != us[k][2] + lam[2]
                                || gt[k][3] != us[k][3] + lam[3]
                            {
                                all = false;
                                break;
                            }
                        }
                        if all {
                            ok = true;
                            break 'anchors;
                        }
                    }
                }
                if ok {
                    break 'tiles;
                }
            }
            if !ok {
                return false;
            }
        }
        true
    }

    /// Does the linear part normalize the lattice? (Both basis images
    /// in Λ; finite index with |det| = 1 then gives equality.)
    fn p3m1_normalizes(l2: &[[i64; 4]; 4]) -> bool {
        p3m1_in_lat(p3m1_apply(l2, [0; 4], [378, 0, 0, -126]))
            && p3m1_in_lat(p3m1_apply(l2, [0; 4], [378, 0, 0, 126]))
    }

    /// The exact p3m1 criterion: g = (L, v) is a symmetry of the
    /// tiling iff L normalizes Λ126 and it maps every cell tile onto a
    /// cell tile plus a lattice translate.
    fn p3m1_is_sym(l2: &[[i64; 4]; 4], v: [i64; 4]) -> bool {
        p3m1_normalizes(l2) && p3m1_maps_cells(l2, v)
    }

    /// Is centre c on one of the tiling's mirror lines? The positive
    /// mirror cosets are exactly the zero cosets of the three grid-line
    /// families refl0/refl60/refl120 (the probe below), whose mirror
    /// lines all pass through the origin, so c lies on a mirror iff
    /// c − L·c ∈ Λ for one of the three linear parts.
    fn p3m1_on_mirror(c: [i64; 4]) -> bool {
        for l2 in [&P3_LREFL0, &P3_LREFL60, &P3_LREFL120] {
            let d = p3m1_apply(l2, [0; 4], c);
            if p3m1_in_lat([c[0] - d[0], c[1] - d[1], c[2] - d[2], c[3] - d[3]]) {
                return true;
            }
        }
        false
    }

    /// Exact p3m1 probe (×126 integer scheme, mirroring
    /// `m85_emit.py` from the HX-STUDY handoff): positives are exactly
    /// the translations (25/25 in a ±2 window) and the zero coset of
    /// rot120, rot240 and the three grid-line mirror families
    /// refl0/refl60/refl120 — everything else negative across the full
    /// candidate census (69 tested cosets for the five positive parts,
    /// 42 for the six negative parts: no 60°/180°/300° rotation, so
    /// not p6 or p6m; no 30°/90°/150° mirror family, no glides). Five
    /// off-lattice translations are killed. The three rot120 centre
    /// orbits — the (0,0), (2,0), (4,0) grid-vertex orbits — reduce
    /// from the closed form c(i,j) ×126 = (126(2i+j), 0, 0, 126j),
    /// and the on-mirror incidence is [T, T, T]: all three orbits on
    /// mirrors, the *333 fingerprint — M84's p31m probe pins the
    /// contrast ([T, F, F]: one orbit on, two off).
    #[test]
    fn alcove_cut_grid_wallpaper_group_is_p3m1() {
        // Positive: the 25 lattice translations of the ±2 window.
        for i in -2i64..=2 {
            for j in -2i64..=2 {
                assert!(
                    p3m1_is_sym(&P3_LIDENT, [378 * (i + j), 0, 0, 126 * (j - i)]),
                    "lattice translation ({i},{j}) must be a symmetry"
                );
            }
        }
        // Negative: the five off-lattice translations.
        for v in P3M1_TRANSLATION_NEGS {
            assert!(
                !p3m1_is_sym(&P3_LIDENT, v),
                "off-lattice {v:?} must not be a symmetry"
            );
        }

        // Full candidate census for the 11 non-identity D6 parts:
        // candidates = {q − L·p : p, q in the 13-vertex set}, reduced
        // to one canonical rep per coset. Only the zero coset is
        // positive, and only for the five p3m1 parts (rot120, rot240,
        // refl0, refl60, refl120).
        const PARTS: [(&str, &[[i64; 4]; 4], usize, bool); 11] = [
            ("rot60", &P3_LROT60, 42, false),
            ("rot120", &P3_LROT120, 69, true),
            ("rot180", &P3_LROT180, 42, false),
            ("rot240", &P3_LROT240, 69, true),
            ("rot300", &P3_LROT300, 42, false),
            ("refl0", &P3_LREFL0, 69, true),
            ("refl30", &P3_LREFL30, 42, false),
            ("refl60", &P3_LREFL60, 69, true),
            ("refl90", &P3_LREFL90, 42, false),
            ("refl120", &P3_LREFL120, 69, true),
            ("refl150", &P3_LREFL150, 42, false),
        ];
        for (name, l2, tested, has_pos) in PARTS {
            assert!(
                p3m1_normalizes(l2),
                "{name}: linear part must normalize the lattice"
            );
            let mut cands: Vec<[i64; 4]> = Vec::new();
            for p in P3M1_VERTS {
                let mp = p3m1_apply(l2, [0; 4], p);
                for q in P3M1_VERTS {
                    let v = p3m1_reduce([
                        q[0] - mp[0],
                        q[1] - mp[1],
                        q[2] - mp[2],
                        q[3] - mp[3],
                    ]);
                    if !cands.contains(&v) {
                        cands.push(v);
                    }
                }
            }
            assert_eq!(cands.len(), tested, "{name}: tested coset count");
            let mut pos = 0;
            for v in &cands {
                let sym = p3m1_is_sym(l2, *v);
                if sym {
                    pos += 1;
                }
                assert_eq!(
                    sym,
                    has_pos && *v == [0, 0, 0, 0],
                    "{name}: candidate {v:?}"
                );
            }
            assert_eq!(pos, usize::from(has_pos), "{name}: positive cosets");
        }

        // The three rot120 centre orbits mod the lattice: the (0,0),
        // (2,0), (4,0) grid-vertex orbits (closed form c(i,j) ×126 =
        // (126(2i+j), 0, 0, 126j)).
        let mut orb: Vec<[i64; 4]> = Vec::new();
        for i in -3i64..=3 {
            for j in -3i64..=3 {
                let v = p3m1_reduce([126 * (2 * i + j), 0, 0, 126 * j]);
                if !orb.contains(&v) {
                    orb.push(v);
                }
            }
        }
        orb.sort_unstable();
        let want_orb: [[i64; 4]; 3] = [[0, 0, 0, 0], [252, 0, 0, 0], [504, 0, 0, 0]];
        assert_eq!(orb, want_orb.to_vec(), "rot120 centre orbits mod lattice");
        // Each orbit rep c satisfies c − R120·c ∈ Λ (R120 about c is
        // a symmetry).
        for c in want_orb {
            let d = p3m1_apply(&P3_LROT120, [0; 4], c);
            assert!(
                p3m1_in_lat([c[0] - d[0], c[1] - d[1], c[2] - d[2], c[3] - d[3]]),
                "orbit rep {c:?}: c − R120·c in Λ"
            );
        }
        // Incidence — the p3m1/p31m discriminator: all THREE centre
        // orbits lie on mirrors, the *333 fingerprint. In p31m (M84's
        // split-triangle-grid probe) exactly one of the three orbits
        // is on a mirror: [T, T, T] here vs [T, F, F] there.
        for c in want_orb {
            assert!(
                p3m1_on_mirror(c),
                "orbit rep {c:?}: all three centre orbits on mirrors (p3m1)"
            );
        }

        // Cell arithmetic ×126²: doubled area (re, im) per tile — the
        // three congruence classes (membership
        // [0,1,2,1,0,2,1,0,2,0,1,2,0,1,2,1,0,2]): class 0 (6 tiles)
        // (0, 14112), class 1 (6 tiles) (0, 6552), class 2 (6 tiles)
        // (0, 11088); sum (0, 190512) = 2·det·126² (det = 6√3).
        // Squared-edge multiset: three congruence classes — class 0
        // {(21072,0), (30144,0), (63504,0)}, class 1
        // {(15024,0), (21072,0), (63504,0)}, class 2
        // {(15024,0), (30144,0), (63504,0)} (achiral as sets: each
        // class is mirror-closed under y = 0).
        let mut areas: Vec<(i64, i64)> = Vec::new();
        let mut edges: Vec<Vec<(i64, i64)>> = Vec::new();
        for t in P3M1_TILE_VERTS.iter() {
            let mut area = (0i64, 0i64);
            let mut es: Vec<(i64, i64)> = Vec::new();
            for i in 0..3 {
                let (u, w) = (t[i], t[(i + 1) % 3]);
                area.0 += u[0] * w[1] + 3 * u[2] * w[3] - w[0] * u[1] - 3 * w[2] * u[3];
                area.1 += u[0] * w[3] + u[2] * w[1] - w[0] * u[3] - w[2] * u[1];
                let (da, db, dc, dd) = (
                    w[0] - u[0],
                    w[1] - u[1],
                    w[2] - u[2],
                    w[3] - u[3],
                );
                es.push((
                    da * da + db * db + 3 * dc * dc + 3 * dd * dd,
                    2 * da * dc + 2 * db * dd,
                ));
            }
            es.sort_unstable();
            edges.push(es.clone());
            areas.push(area);
        }
        let want_areas: [(i64, i64); 18] = [
            (0, 14112), (0, 6552), (0, 11088), (0, 6552), (0, 14112), (0, 11088),
            (0, 6552), (0, 14112), (0, 11088), (0, 14112), (0, 6552), (0, 11088),
            (0, 14112), (0, 6552), (0, 11088), (0, 6552), (0, 14112), (0, 11088),
        ];
        assert_eq!(areas, want_areas.to_vec(), "doubled tile areas ×126²");
        assert_eq!(
            areas.iter().map(|a| a.0).sum::<i64>(),
            0,
            "real part of doubled cell area"
        );
        assert_eq!(
            areas.iter().map(|a| a.1).sum::<i64>(),
            190_512,
            "sum = 2·det·126² (det 6√3)"
        );
        const MEMBERSHIP: [usize; 18] = [0, 1, 2, 1, 0, 2, 1, 0, 2, 0, 1, 2, 0, 1, 2, 1, 0, 2];
        let want_edges: [Vec<(i64, i64)>; 3] = [
            vec![(21072, 0), (30144, 0), (63504, 0)],
            vec![(15024, 0), (21072, 0), (63504, 0)],
            vec![(15024, 0), (30144, 0), (63504, 0)],
        ];
        for (e, cls) in edges.iter().zip(MEMBERSHIP.iter()) {
            assert_eq!(
                *e, want_edges[*cls],
                "squared-edge multiset (three congruence classes, achiral as sets)"
            );
        }
    }

    /// M112: `SeedPattern::Rows` — the tile at lattice index (i, j)
    /// sits at world y = j on the centered-rectangular lattice
    /// <(2,0),(1,1)>, so the row-index parity is exactly the course
    /// parity: every generated tile's seed must equal the parity of
    /// its row, and a bounded view must show both parities.
    #[test]
    fn running_bond_seed_rows_are_course_parity() {
        let t = running_bond::running_bond();
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region: BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let mut seen = [false; 2];
        for tile in &r.tiles {
            // Default params (scale 1, rotation 0): a domino of course
            // j occupies y = [j, j+1], so its aabb min.y is j exactly.
            let j = tile.poly.aabb().min.y.round() as i64;
            let want = (((j % 2) + 2) % 2) as u32;
            assert_eq!(tile.seed, want, "seed at course {j} must equal the course parity");
            seen[tile.seed as usize] = true;
        }
        assert!(seen[0] && seen[1], "both courses must appear");
    }

    /// M112: single hexomino per cell, single orbit — the shipped seed
    /// is the decorative `SeedPattern::Checker` lattice parity, so a
    /// bounded view must show both parities (seeds stay in {0, 1}).
    #[test]
    fn stepped_hexominoes_seed_is_decorative_checkerboard() {
        let t = stepped_hexominoes::stepped_hexominoes();
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region: BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let mut seen = [false; 2];
        for tile in &r.tiles {
            assert!(tile.seed < 2, "checkerboard seed must stay in {{0, 1}}");
            seen[tile.seed as usize] = true;
        }
        assert!(seen[0] && seen[1], "both lattice-parity classes must appear");
    }

    /// M112: the three tiles of the cell are R120 images of one chiral
    /// class (a single orbit) — the shipped seed is the decorative
    /// `SeedPattern::Checker` lattice parity, so a bounded view must
    /// show both parities (seeds stay in {0, 1}).
    #[test]
    fn chiral_cut_honeycomb_seed_is_decorative_checkerboard() {
        let t = chiral_cut_honeycomb::chiral_cut_honeycomb();
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region: BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let mut seen = [false; 2];
        for tile in &r.tiles {
            assert!(tile.seed < 2, "checkerboard seed must stay in {{0, 1}}");
            seen[tile.seed as usize] = true;
        }
        assert!(seen[0] && seen[1], "both lattice-parity classes must appear");
    }
}

/// Tests for the 2-uniform (k-uniform) `LatticeTiling` tables.
#[cfg(all(test, feature = "kuniform"))]
mod kuniform_tests {
    use super::lattice::LatticeTiling;
    use super::tests::{check_covers, check_det, check_rotated};
    use crate::kuniform;
    use std::collections::{BTreeSet, HashMap};
    use tess_core::geometry::{BBox, Vec2};

    const H: f64 = 0.8660254037844386; // √3/2

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn kuniform_p6m_covers_region() {
        check_covers(
            &kuniform::kuniform_p6m(),
            "kuniform-p6m",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn kuniform_p6m_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_p6m(),
            "kuniform-p6m(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_p6m_deterministic() {
        check_det(
            &kuniform::kuniform_p6m(),
            "kuniform-p6m",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// One expected figure orbit in the ±3×±3 patch census.
    struct Figure<'a> {
        /// Canonical figure string (lexicographically smallest cyclic
        /// rotation, e.g. "3366", "3636", "333333").
        figure: &'a str,
        /// Interior vertices of that figure in the patch.
        count: usize,
        /// Orbit reps in [0,b1]×[0,b2], 3-decimal quantized.
        orbits: &'a [(i32, i32)],
    }

    /// Per-vertex sector record: (start angle deg, span deg, side count).
    type Sector = (f64, f64, u32);

    /// Emit the ±3×±3 patch straight from the table (the same
    /// emission the Python spike used), deduplicate vertices by
    /// 6-decimal key, and check the interior-vertex census: the
    /// angle-sum-360 vertices carry exactly the declared figures,
    /// with the expected counts and translation-orbit structure.
    fn check_vertex_figures(lt: &LatticeTiling, expect: &[Figure]) {
        let b1 = lt.lattice[0];
        let b2 = lt.lattice[1];
        let det = b1.cross(b2);
        // Inverse of the lattice map p -> [b1 b2] * u (the same
        // world->lattice inverse as the generator, in cell space).
        let inv = |p: Vec2| -> Vec2 {
            Vec2::new(
                (b2.y * p.x - b2.x * p.y) / det,
                (-b1.y * p.x + b1.x * p.y) / det,
            )
        };
        let key = |p: Vec2| -> (i64, i64) {
            ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64)
        };

        // Per vertex key: the accumulated interior angle (60° triangle
        // corner, 120° hexagon corner) and the CCW sectors
        // (start angle deg, span deg, side count).
        let mut angles: HashMap<(i64, i64), f64> = HashMap::new();
        let mut sectors: HashMap<(i64, i64), Vec<Sector>> = HashMap::new();
        for i in -3i64..=3 {
            for j in -3i64..=3 {
                let tr = b1 * i as f64 + b2 * j as f64;
                for c in &lt.cells {
                    let pts = c.poly.translated(tr).pts;
                    let n = pts.len();
                    let span = (n - 2) as f64 * 180.0 / n as f64;
                    for k in 0..n {
                        let v = pts[k];
                        let b = pts[(k + 1) % n];
                        let kk = key(v);
                        *angles.entry(kk).or_insert(0.0) += span;
                        // Sector of this CCW polygon at v: CCW from the
                        // v->next direction, spanning the interior angle.
                        let start = f64::atan2(b.y - v.y, b.x - v.x).to_degrees();
                        let start = if start < 0.0 { start + 360.0 } else { start };
                        sectors.entry(kk).or_default().push((start, span, n as u32));
                    }
                }
            }
        }

        // Canonical cyclic figure (lex-smallest rotation of the
        // side-count sequence) of one interior vertex.
        let figure_of = |secs: &[Sector]| -> String {
            let mut s: Vec<Sector> = secs.to_vec();
            s.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            // Sectors of an interior vertex tile the 360 degrees:
            // walk them CCW from the smallest start angle.
            let mut cur = s[0].0;
            for (s0, sp, _) in s.iter() {
                assert!(
                    (*s0 - cur).abs() < 1e-6,
                    "figure walk gap: sector at {s0}, expected {cur}"
                );
                cur += sp;
                if cur >= 360.0 - 1e-9 {
                    cur -= 360.0;
                }
            }
            assert!(
                (cur - s[0].0).abs() < 1e-6,
                "figure walk must close, ends at {cur}"
            );
            let seq: Vec<u32> = s.iter().map(|t| t.2).collect();
            let mut best = seq.clone();
            for r in 1..seq.len() {
                let mut rot = Vec::with_capacity(seq.len());
                rot.extend_from_slice(&seq[r..]);
                rot.extend_from_slice(&seq[..r]);
                if rot < best {
                    best = rot;
                }
            }
            best.iter().map(|n| n.to_string()).collect()
        };

        let mut figs: HashMap<String, Vec<(i64, i64)>> = HashMap::new();
        for (kk, sum) in &angles {
            if (sum - 360.0).abs() < 1e-6 {
                let secs = sectors
                    .get(kk)
                    .unwrap_or_else(|| panic!("interior vertex without sectors"));
                let f = figure_of(secs);
                figs.entry(f).or_default().push(*kk);
            }
        }

        for e in expect {
            let vs = figs.get(e.figure).cloned().unwrap_or_default();
            assert_eq!(
                vs.len(),
                e.count,
                "{}: {} interior vertices in the ±3 patch, expected {}",
                e.figure,
                vs.len(),
                e.count
            );
            // Orbit reps: v - A*floor(A^-1 v), quantized to 3
            // decimals (a 1e-5 guard keeps vertices exactly on a
            // lattice line in the correct fundamental cell).
            let mut orbits: BTreeSet<(i32, i32)> = BTreeSet::new();
            for &kk in &vs {
                let p = v2(kk.0 as f64 * 1e-6, kk.1 as f64 * 1e-6);
                let u = inv(p);
                let fx = (u.x + 1e-5).floor();
                let fy = (u.y + 1e-5).floor();
                let rep = p - b1 * fx - b2 * fy;
                orbits.insert(
                    ((rep.x * 1000.0).round() as i32, (rep.y * 1000.0).round() as i32),
                );
            }
            let want: BTreeSet<(i32, i32)> = e.orbits.iter().copied().collect();
            assert_eq!(&orbits, &want, "{}: orbit reps in [0,b1]×[0,b2]", e.figure);
        }
        let declared: BTreeSet<&str> = expect.iter().map(|e| e.figure).collect();
        for f in figs.keys() {
            assert!(
                declared.contains(f.as_str()),
                "unexpected vertex figure {f} (declared: {declared:?})"
            );
        }
    }

    /// Interior vertex figures over the ±3×±3 patch: one 3⁶ orbit
    /// (the fan apexes) and six 3².6² orbits — the honeycomb-vertex
    /// orbit splits into six translation orbits under the index-3
    /// translation lattice. Counts and reps match the Python spike.
    #[test]
    fn kuniform_p6m_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_p6m(),
            &[
                Figure {
                    figure: "333333",
                    count: 49,
                    orbits: &[(0, 0)],
                },
                Figure {
                    figure: "3366",
                    count: 241,
                    orbits: &[
                        (500, 866),
                        (1000, 0),
                        (1000, 1732),
                        (2000, 0),
                        (2000, 1732),
                        (2500, 866),
                    ],
                },
            ],
        );
    }

    /// Cell-set symmetry probe (the Python spike's exact test in
    /// floats): `g` must send every table cell to some table cell
    /// plus a lattice translation — Cramer solve for the shift,
    /// integrality within 1e-6, vertex geometry within 1e-9. With
    /// `symmetry = false` (a negative probe) at least one cell must
    /// fail to map — individual cells can match (a hexagon is
    /// invariant under 60° about its own centre) without the
    /// mapping extending to the whole tiling.
    fn probe_symmetry(lt: &LatticeTiling, g: impl Fn(Vec2) -> Vec2, symmetry: bool, label: &str) {
        let b1 = lt.lattice[0];
        let b2 = lt.lattice[1];
        let det = b1.cross(b2);
        let inv = |p: Vec2| -> Vec2 {
            Vec2::new(
                (b2.y * p.x - b2.x * p.y) / det,
                (-b1.y * p.x + b1.x * p.y) / det,
            )
        };
        let cells: Vec<Vec<Vec2>> = lt.cells.iter().map(|c| c.poly.pts.clone()).collect();
        let mut mismatched = 0;
        for (i, cp) in cells.iter().enumerate() {
            let gp: Vec<Vec2> = cp.iter().map(|p| g(*p)).collect();
            let mut matched = false;
            for cj in &cells {
                let mut hit = false;
                for w in cj {
                    let t = gp[0] - *w;
                    let u = inv(t);
                    // A symmetry shift must be a lattice translation.
                    if (u.x - u.x.round()).abs() > 1e-6 || (u.y - u.y.round()).abs() > 1e-6 {
                        continue;
                    }
                    let tr = b1 * u.x.round() + b2 * u.y.round();
                    let shifted: Vec<Vec2> = cj.iter().map(|w| *w + tr).collect();
                    hit = gp.iter().all(|p| {
                        shifted.iter().any(|s| (*p - *s).length() < 1e-9)
                    }) && shifted.iter().all(|s| gp.iter().any(|p| (*p - *s).length() < 1e-9));
                    if hit {
                        break;
                    }
                }
                if hit {
                    matched = true;
                    break;
                }
            }
            if symmetry {
                assert!(
                    matched,
                    "{label}: table cell {i} expected a match, none found"
                );
            } else if !matched {
                mismatched += 1;
            }
        }
        assert!(
            symmetry || mismatched > 0,
            "{label}: every table cell matched — expected a negative probe to fail"
        );
    }

    /// The declared p6m generators act on the cell set: 60°/120°
    /// rotations about the fan apex at the origin, the mirror y = 0,
    /// and the mirror along the 60° line.
    #[test]
    fn kuniform_p6m_symmetry() {
        let lt = kuniform::kuniform_p6m();
        probe_symmetry(
            &lt,
            |p| p.rotated(std::f64::consts::PI / 3.0),
            true,
            "R60 about (0,0)",
        );
        probe_symmetry(
            &lt,
            |p| p.rotated(2.0 * std::f64::consts::PI / 3.0),
            true,
            "R120 about (0,0)",
        );
        probe_symmetry(&lt, |p| v2(p.x, -p.y), true, "mirror y=0");
        probe_symmetry(
            &lt,
            |p| {
                let d = v2(0.5, H);
                d * (2.0 * p.dot(d)) - p
            },
            true,
            "mirror line at 60 deg",
        );
    }

    #[test]
    fn kuniform_pmm_covers_region() {
        check_covers(
            &kuniform::kuniform_pmm(),
            "kuniform-pmm",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn kuniform_pmm_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_pmm(),
            "kuniform-pmm(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_pmm_deterministic() {
        check_det(
            &kuniform::kuniform_pmm(),
            "kuniform-pmm",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: one 3.6.3.6
    /// orbit (the hexagon-corner vertices (2i+1, √3j)) and two 3².6²
    /// orbits (the triangle-base vertices (2i+1/2, H+√3j)). Counts
    /// and reps match the Python spike.
    #[test]
    fn kuniform_pmm_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_pmm(),
            &[
                Figure {
                    figure: "3636",
                    count: 36,
                    orbits: &[(1000, 0)],
                },
                Figure {
                    figure: "3366",
                    count: 78,
                    orbits: &[(500, 866), (1500, 866)],
                },
            ],
        );
    }

    fn rot_about(deg: f64, c: Vec2, p: Vec2) -> Vec2 {
        c + (p - c).rotated(deg.to_radians())
    }

    /// The declared pmm generators act on the cell set: mirrors
    /// x = 0, y = 0, y = H and the half-turns at (1, 0) and (1, H).
    /// Negative probes: no 6-, 9-, or 12-fold rotation about any of
    /// (0,0), (1,0), (1,H) — pmm has none (lattice argument: R60
    /// sends b1 = (2,0) to (1, √3) = b1/2 + b2, not a lattice
    /// vector).
    #[test]
    fn kuniform_pmm_symmetry() {
        let lt = kuniform::kuniform_pmm();
        probe_symmetry(&lt, |p| v2(-p.x, p.y), true, "mirror x=0");
        probe_symmetry(&lt, |p| v2(p.x, -p.y), true, "mirror y=0");
        probe_symmetry(&lt, |p| v2(p.x, 2.0 * H - p.y), true, "mirror y=H");
        probe_symmetry(&lt, |p| v2(2.0 - p.x, -p.y), true, "180 deg about (1,0)");
        probe_symmetry(&lt, |p| v2(2.0 - p.x, 2.0 * H - p.y), true, "180 deg about (1,H)");
        for c in [Vec2::ZERO, v2(1.0, 0.0), v2(1.0, H)] {
            for deg in [60.0, 90.0, 120.0] {
                probe_symmetry(
                    &lt,
                    move |p| rot_about(deg, c, p),
                    false,
                    &format!("{deg} deg about ({c})"),
                );
            }
        }
    }

    #[test]
    fn kuniform_n18_covers_region() {
        check_covers(
            &kuniform::kuniform_n18(),
            "kuniform-n18",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn kuniform_n18_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n18(),
            "kuniform-n18(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n18_deterministic() {
        check_det(
            &kuniform::kuniform_n18(),
            "kuniform-n18",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch for the p6m
    /// tiling (3⁶; 3².4.3.4): one 3⁶ orbit (the origin's six-triangle
    /// vertex class) and six 3².4.3.4 orbits. Counts and reps match
    /// the Python battery.
    #[test]
    fn kuniform_n18_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n18(),
            &[
                Figure {
                    figure: "333333",
                    count: 36,
                    orbits: &[(0, 0)],
                },
                Figure {
                    figure: "33434",
                    count: 260,
                    orbits: &[
                        (500, 866),
                        (500, 1866),
                        (1000, 2732),
                        (1366, 1366),
                        (1866, 2232),
                        (1866, 3232),
                    ],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n9_covers_region() {
        check_covers(
            &kuniform::kuniform_n9(),
            "kuniform-n9",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn kuniform_n9_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n9(),
            "kuniform-n9(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n9_deterministic() {
        check_det(
            &kuniform::kuniform_n9(),
            "kuniform-n9",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch for the p6m
    /// tiling (3.4.6.4; 3².4.3.4): six 3².4.3.4 orbits (the
    /// mirror-only, palindromic vertices) and six 3.4.6.4 orbits.
    /// Counts and reps match the Python battery.
    #[test]
    fn kuniform_n9_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n9(),
            &[
                Figure {
                    figure: "33434",
                    count: 234,
                    orbits: &[
                        (-2366, -1366),
                        (-1866, -500),
                        (-1366, -1366),
                        (-500, -1866),
                        (0, -2732),
                        (500, -1866),
                    ],
                },
                Figure {
                    figure: "3464",
                    count: 287,
                    orbits: &[
                        (-2732, 0),
                        (-1366, -2366),
                        (-1000, 0),
                        (-500, -866),
                        (500, -866),
                        (1366, -2366),
                    ],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n8_covers_region() {
        check_covers(
            &kuniform::kuniform_n8(),
            "kuniform-n8",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn kuniform_n8_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n8(),
            "kuniform-n8(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n8_deterministic() {
        check_det(
            &kuniform::kuniform_n8(),
            "kuniform-n8",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch for the p6m
    /// tiling (3.4.6.4; 3³.4²): six 3³.4² orbits and six 3.4.6.4
    /// orbits. Counts and reps match the Python battery.
    #[test]
    fn kuniform_n8_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n8(),
            &[
                Figure {
                    figure: "33344",
                    count: 260,
                    orbits: &[
                        (-2732, 0),
                        (-1866, -500),
                        (-1866, 500),
                        (-1366, 1366),
                        (-1366, 2366),
                        (-500, 1866),
                    ],
                },
                Figure {
                    figure: "3464",
                    count: 267,
                    orbits: &[
                        (-2732, -1000),
                        (-2732, 1000),
                        (-2232, 1866),
                        (-1000, 0),
                        (-500, 866),
                        (-500, 2866),
                    ],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n5_covers_region() {
        check_covers(
            &kuniform::kuniform_n5(),
            "kuniform-n5",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn kuniform_n5_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n5(),
            "kuniform-n5(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n5_deterministic() {
        check_det(
            &kuniform::kuniform_n5(),
            "kuniform-n5",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch for the p6m
    /// tiling (3.4.6.4; 3.4².6): the achiral 3.4.6.4 orbit ("3464")
    /// and the CHIRAL 3.4.4.6 orbit, whose figure "3446" differs from
    /// its mirror image "3644" (three figure strings, two orbits).
    /// Counts and reps match the Python battery.
    #[test]
    fn kuniform_n5_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n5(),
            &[
                Figure {
                    figure: "3446",
                    count: 266,
                    orbits: &[
                        (-1866, 3232),
                        (-500, 1866),
                        (-500, 3598),
                        (500, 866),
                        (1000, 2732),
                        (3732, 0),
                    ],
                },
                Figure {
                    figure: "3464",
                    count: 267,
                    orbits: &[
                        (1500, 866),
                        (1500, 1866),
                        (2366, 366),
                        (2366, 2366),
                        (3232, 866),
                        (3232, 1866),
                    ],
                },
                Figure {
                    figure: "3644",
                    count: 267,
                    orbits: &[
                        (-1000, 2732),
                        (-500, 866),
                        (500, 1866),
                        (500, 3598),
                        (1000, 0),
                        (1866, 3232),
                    ],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n1_covers_region() {
        check_covers(
            &kuniform::kuniform_n1(),
            "kuniform-n1",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn kuniform_n1_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n1(),
            "kuniform-n1(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n1_deterministic() {
        check_det(
            &kuniform::kuniform_n1(),
            "kuniform-n1",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch for the p6m
    /// 3-4-6-12 tiling (4.6.12; 3.4.6.4): the achiral 3.4.6.4 orbit
    /// ("3464") and the CHIRAL 4.6.12 orbit, whose figure "4612"
    /// differs from its mirror image "4126" (three figure strings, two
    /// orbits — the canonical strings are the Vec<u32> lex-min
    /// rotations; the string-concatenation ring labels are "1246"/
    /// "1264"). Counts and reps match the Python battery.
    #[test]
    fn kuniform_n1_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n1(),
            &[
                Figure {
                    figure: "3464",
                    count: 267,
                    orbits: &[
                        (866, 500),
                        (1366, 1366),
                        (1866, 500),
                        (6330, 4232),
                        (6830, 3366),
                        (7330, 4232),
                    ],
                },
                Figure {
                    figure: "4126",
                    count: 266,
                    orbits: &[
                        (2232, 1866),
                        (2732, 3732),
                        (3598, 500),
                        (4598, 4232),
                        (5464, 1000),
                        (5964, 2866),
                    ],
                },
                Figure {
                    figure: "4612",
                    count: 266,
                    orbits: &[
                        (2232, 2866),
                        (2732, 1000),
                        (3598, 4232),
                        (4598, 500),
                        (5464, 3732),
                        (5964, 1866),
                    ],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n13_covers_region() {
        check_covers(
            &kuniform::kuniform_n13(),
            "kuniform-n13",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn kuniform_n13_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n13(),
            "kuniform-n13(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n13_deterministic() {
        check_det(
            &kuniform::kuniform_n13(),
            "kuniform-n13",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch for the p6m
    /// tiling (3⁶; 3².4.12): the achiral 3⁶ orbit ("333333", the
    /// 6-fold vertices) and the CHIRAL 3².4.12 orbit, whose figure
    /// "33412" differs from its mirror image "33124" (three figure
    /// strings, two orbits — the canonical strings are the Vec<u32>
    /// lex-min rotations; the string-concatenation ring labels are
    /// "12334"/"12433"). Counts and reps match the Python battery.
    #[test]
    fn kuniform_n13_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n13(),
            &[
                Figure {
                    figure: "33124",
                    count: 240,
                    orbits: &[
                        (0, 3732),
                        (500, 1866),
                        (866, 500),
                        (1366, 3366),
                        (2232, 1866),
                        (3232, 2866),
                    ],
                },
                Figure {
                    figure: "333333",
                    count: 84,
                    orbits: &[(0, 0), (1366, 2366)],
                },
                Figure {
                    figure: "33412",
                    count: 241,
                    orbits: &[
                        (0, 1000),
                        (500, 2866),
                        (866, 4232),
                        (1366, 1366),
                        (2232, 2866),
                        (3232, 1866),
                    ],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n20_covers_region() {
        check_covers(
            &kuniform::kuniform_n20(),
            "kuniform-n20",
            BBox::new(v2(-4.0, -4.0), v2(12.0, 12.0)),
        );
    }

    #[test]
    fn kuniform_n20_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n20(),
            "kuniform-n20(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n20_deterministic() {
        check_det(
            &kuniform::kuniform_n20(),
            "kuniform-n20",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the achiral 3⁶
    /// orbit (6 translation classes per cell, 266 interior points) and
    /// the achiral 3⁴.6 orbit (6 classes, 240 interior points — the
    /// counts differ only through patch boundary truncation). Both
    /// counts and all 12 orbit reps match the Python torus battery
    /// (m96b_rust_expect.py).
    #[test]
    fn kuniform_n20_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n20(),
            &[
                Figure {
                    figure: "333333",
                    count: 266,
                    orbits: &[
                        (0, 1732),
                        (500, 2598),
                        (500, 4330),
                        (1000, 1732),
                        (1000, 3464),
                        (1500, 4330),
                    ],
                },
                Figure {
                    figure: "33336",
                    count: 240,
                    orbits: &[
                        (-500, 2598),
                        (0, 3464),
                        (500, 866),
                        (1000, 5196),
                        (1500, 2598),
                        (2000, 3464),
                    ],
                },
            ],
        );
    }

    #[test]
fn kuniform_n12_covers_region() {
        check_covers(
            &kuniform::kuniform_n12(),
            "kuniform-n12",
            BBox::new(v2(-4.0, -4.0), v2(8.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_n19_covers_region() {
        check_covers(
            &kuniform::kuniform_n19(),
            "kuniform-n19",
            BBox::new(v2(-4.0, -4.0), v2(9.0, 9.0)),
        );
    }

    #[test]
fn kuniform_n12_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n12(),
            "kuniform-n12(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(4.0, 4.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n19_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n19(),
            "kuniform-n19(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
fn kuniform_n12_deterministic() {
        check_det(
            &kuniform::kuniform_n12(),
            "kuniform-n12",
            BBox::new(v2(-3.0, -3.0), v2(6.0, 6.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the 3².6² orbit
    /// (2 translation classes per primitive cell — the hex–hex·tri–tri
    /// junctions along the shared hexagon edges, 78 interior points)
    /// and the 3⁴.6 orbit (2 classes, 84 interior points — four
    /// triangles plus a hexagon corner; the counts differ only through
    /// patch boundary truncation). Both counts and all 4 orbit reps
    /// match the Python battery (m97_rust_expect.py).
    #[test]
    fn kuniform_n12_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n12(),
            &[
                Figure {
                    figure: "3366",
                    count: 78,
                    orbits: &[(866, 500), (1732, 2000)],
                },
                Figure {
                    figure: "33336",
                    count: 84,
                    orbits: &[(866, 1500), (1732, 1000)],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n19_deterministic() {
        check_det(
            &kuniform::kuniform_n19(),
            "kuniform-n19",
            BBox::new(v2(-3.0, -3.0), v2(8.0, 8.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the achiral 3⁶
    /// orbit (2 translation classes per cell under L9 — the two petal-tip
    /// classes, 72 interior points) and the achiral 3⁴.6 orbit (6
    /// classes — the hexagon corners, 241 interior points). Both counts
    /// and all 8 orbit reps match the Python torus battery replica
    /// (m95x_rust_expect.py).
    #[test]
    fn kuniform_n19_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n19(),
            &[
                Figure {
                    figure: "333333",
                    count: 72,
                    orbits: &[(1500, 866), (3000, 1732)],
                },
                Figure {
                    figure: "33336",
                    count: 241,
                    orbits: &[
                        (500, 866),
                        (1000, 0),
                        (1000, 1732),
                        (2000, 0),
                        (2000, 1732),
                        (2500, 866),
                    ],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n4_covers_region() {
        check_covers(
            &kuniform::kuniform_n4(),
            "kuniform-n4",
            BBox::new(v2(-4.0, -4.0), v2(9.0, 9.0)),
        );
    }

    #[test]
    fn kuniform_n4_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n4(),
            "kuniform-n4(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(4.0, 4.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n4_deterministic() {
        check_det(
            &kuniform::kuniform_n4(),
            "kuniform-n4",
            BBox::new(v2(-3.0, -3.0), v2(6.0, 6.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the 3³.4² orbit
    /// (2 translation classes per cell — the square-band/triangle-band
    /// junctions, 78 interior points) and the 4⁴ orbit (1 class — the
    /// crossing of four squares inside a band, 42 interior points).
    /// Both counts and all 3 orbit reps match the Python torus battery
    /// (tku3_run.py rust_expect).
    #[test]
    fn kuniform_n4_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n4(),
            &[
                Figure {
                    figure: "33344",
                    count: 78,
                    orbits: &[(0, 0), (1000, 2000)],
                },
                Figure {
                    figure: "4444",
                    count: 42,
                    orbits: &[(1000, 1000)],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n14_covers_region() {
        check_covers(
            &kuniform::kuniform_n14(),
            "kuniform-n14",
            BBox::new(v2(-4.0, -4.0), v2(9.0, 9.0)),
        );
    }

    #[test]
    fn kuniform_n14_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n14(),
            "kuniform-n14(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(4.0, 4.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n14_deterministic() {
        check_det(
            &kuniform::kuniform_n14(),
            "kuniform-n14",
            BBox::new(v2(-3.0, -3.0), v2(6.0, 6.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the 3⁶ orbit
    /// (1 translation class per cell — the double-strip junction line,
    /// 42 interior points) and the 3³.4² orbit (2 classes — the
    /// square-row corners, 78 interior points). Both counts and all 3
    /// orbit reps match the Python torus battery (tku3_run.py
    /// rust_expect).
    #[test]
    fn kuniform_n14_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n14(),
            &[
                Figure {
                    figure: "333333",
                    count: 42,
                    orbits: &[(500, 866)],
                },
                Figure {
                    figure: "33344",
                    count: 78,
                    orbits: &[(0, 0), (0, 1732)],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n3_covers_region() {
        check_covers(
            &kuniform::kuniform_n3(),
            "kuniform-n3",
            BBox::new(v2(-4.0, -4.0), v2(9.0, 9.0)),
        );
    }

    #[test]
    fn kuniform_n3_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n3(),
            "kuniform-n3(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(4.0, 4.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n3_deterministic() {
        check_det(
            &kuniform::kuniform_n3(),
            "kuniform-n3",
            BBox::new(v2(-3.0, -3.0), v2(6.0, 6.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the 3³.4² orbit
    /// (2 translation classes per cell — the square-band/triangle-band
    /// junctions, 72 interior points) and the 4⁴ orbit (2 classes — the
    /// interior crossings of a three-row band, 84 interior points).
    /// Both counts and all 4 orbit reps match the Python exact battery
    /// (tku4_run.py rust_expect).
    #[test]
    fn kuniform_n3_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n3(),
            &[
                Figure {
                    figure: "33344",
                    count: 72,
                    orbits: &[(0, 0), (1000, 3000)],
                },
                Figure {
                    figure: "4444",
                    count: 84,
                    orbits: &[(1000, 1000), (1000, 2000)],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n15_covers_region() {
        check_covers(
            &kuniform::kuniform_n15(),
            "kuniform-n15",
            BBox::new(v2(-4.0, -4.0), v2(9.0, 9.0)),
        );
    }

    #[test]
    fn kuniform_n15_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n15(),
            "kuniform-n15(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(4.0, 4.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n15_deterministic() {
        check_det(
            &kuniform::kuniform_n15(),
            "kuniform-n15",
            BBox::new(v2(-3.0, -3.0), v2(6.0, 6.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the 3⁶ orbit
    /// (2 translation classes per cell — inside the triple strips,
    /// 78 interior points) and the 3³.4² orbit (2 classes — the
    /// square-row corners, 78 interior points). Both counts and all 4
    /// orbit reps match the Python exact battery (tku4_run.py
    /// rust_expect).
    #[test]
    fn kuniform_n15_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n15(),
            &[
                Figure {
                    figure: "333333",
                    count: 78,
                    orbits: &[(500, 2732), (1000, 1866)],
                },
                Figure {
                    figure: "33344",
                    count: 78,
                    orbits: &[(500, 0), (500, 1000)],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n2_covers_region() {
        check_covers(
            &kuniform::kuniform_n2(),
            "kuniform-n2",
            BBox::new(v2(-4.0, -4.0), v2(8.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_n2_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n2(),
            "kuniform-n2(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n2_deterministic() {
        check_det(
            &kuniform::kuniform_n2(),
            "kuniform-n2",
            BBox::new(v2(-3.0, -3.0), v2(6.0, 6.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the 3.12.12 orbit
    /// ("31212", the dodecagon–dodecagon kiss points, 4 translation
    /// classes, 156 interior points) and the 3.4.3.12 orbit ("34312",
    /// the square corners, 4 classes, 169 interior points). Counts and
    /// reps match the Python exact battery (tku5_n2j.py).
    #[test]
    fn kuniform_n2_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n2(),
            &[
                Figure {
                    figure: "31212",
                    count: 156,
                    orbits: &[(0, 1366), (0, 2366), (1366, 0), (2366, 0)],
                },
                Figure {
                    figure: "34312",
                    count: 169,
                    orbits: &[(500, 500), (500, 3232), (3232, 500), (3232, 3232)],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n16_covers_region() {
        check_covers(
            &kuniform::kuniform_n16(),
            "kuniform-n16",
            BBox::new(v2(-4.0, -4.0), v2(8.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_n16_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n16(),
            "kuniform-n16(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n16_deterministic() {
        check_det(
            &kuniform::kuniform_n16(),
            "kuniform-n16",
            BBox::new(v2(-3.0, -3.0), v2(6.0, 6.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the 3³.4² orbit
    /// ("33344", 4 translation classes, 182 interior points — the
    /// square-band junction vertices) and the 3².4.3.4 orbit
    /// ("33434", 8 classes, 352 interior points — snub-square-type
    /// vertices; the 4:8 class ratio matches the cell's 4 + 8
    /// vertices). Counts and reps match the Python exact battery
    /// (tku5_n16k.py), which uses the same right-handed basis
    /// b1 = (h,−h), b2 = (h,h) as the table.
    #[test]
    fn kuniform_n16_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n16(),
            &[
                Figure {
                    figure: "33344",
                    count: 182,
                    orbits: &[(866, 500), (1366, -1366), (1866, 500), (3732, 0)],
                },
                Figure {
                    figure: "33434",
                    count: 352,
                    orbits: &[
                        (0, 0),
                        (866, -500),
                        (1866, -500),
                        (1866, 1500),
                        (2366, -1366),
                        (2732, 0),
                        (2732, 1000),
                        (3232, -866),
                    ],
                },
            ],
        );
    }
}
#[cfg(all(
    test,
    feature = "square",
    feature = "hexagon",
    feature = "triangle",
    feature = "archimedean",
    feature = "cairo",
    feature = "girih",
    feature = "girih_sc",
    feature = "girih_ext",
    feature = "hankin_ext",
    feature = "khatam_girih",
    feature = "kuniform",
    feature = "laves",
    feature = "pavers",
    feature = "pentagon1",
    feature = "pentagon2",
    feature = "pentagon3",
    feature = "pentagon7",
    feature = "pentagon9",
    feature = "pentagon10",
    feature = "pentagon11",
    feature = "pentagon12",
    feature = "pentagon13",
    feature = "pentagon14",
    feature = "pentagon15",
    feature = "windmill"
))]
mod lattice_culling_tests {
    use super::lattice::LatticeTiling;
    use crate::{
        alcove_cut_grid, archimedean, basketweave, cairo, chiral_cut_honeycomb, dual, girih,
        girih_sc,
        girih_w85, glide_hexagon_strips, hexagon, herringbone,
khatam_girih, kuniform, laves, pentagon1, pentagon2, pentagon3, pentagon6, pentagon7, pentagon8, pentagon9, pentagon10, pentagon11, pentagon12,
        pentagon13,
        pentagon14, pentagon15,
        pic_3_12_12,
        pic_3_6_3_6, pic_4_8_8, running_bond, shifted_trapezoid_courses,
        split_triangle_grid, square, stack_bond,
        herringbone_45,
        basketweave_3,
        stepped_hexominoes, triangle, trapezoid_courses, wavy_courses, windmill,
    };
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
        let lattices: Vec<(&str, LatticeTiling)> = vec![
            ("square", square::spec()),
            ("hexagon", hexagon::spec()),
            ("triangle", triangle::spec()),
            ("4.8.8", archimedean::truncated_square()),
            ("3.6.3.6", archimedean::trihexagonal()),
            ("3.3.3.3.6", archimedean::snub_trihexagonal()),
            ("3.3.3.4.4", archimedean::elongated_triangular()),
            ("3.4.6.4", archimedean::rhombitrihexagonal()),
            ("4.6.12", archimedean::truncated_trihexagonal()),
            ("3.12.12", archimedean::truncated_hexagonal()),
            ("3.3.4.3.4", archimedean::snub_square()),
            ("cairo", cairo::cairo_pentagonal()),
            ("girih", girih::girih_shesh_band()),
            ("girih-sc", girih_sc::girih_star_cross()),
            ("girih-w85", girih_w85::girih_w85()),
            ("pic-4-8-8", pic_4_8_8::pic_4_8_8()),
            ("pic-3-6-3-6", pic_3_6_3_6::pic_3_6_3_6()),
            ("pic-3-12-12", pic_3_12_12::pic_3_12_12()),
            ("khatam-girih", khatam_girih::khatam_girih()),
            ("kuniform-p6m", kuniform::kuniform_p6m()),
            ("kuniform-pmm", kuniform::kuniform_pmm()),
            ("kuniform-n18", kuniform::kuniform_n18()),
            ("kuniform-n9", kuniform::kuniform_n9()),
            ("kuniform-n8", kuniform::kuniform_n8()),
            ("kuniform-n5", kuniform::kuniform_n5()),
            ("kuniform-n1", kuniform::kuniform_n1()),
            ("kuniform-n13", kuniform::kuniform_n13()),
            ("kuniform-n2", kuniform::kuniform_n2()),
            ("kuniform-n16", kuniform::kuniform_n16()),
            ("tetrakis-square", laves::tetrakis_square()),
            ("rhombille", laves::rhombille()),
            ("triakis-triangular", laves::triakis_triangular()),
            ("deltoidal-trihexagonal", laves::deltoidal_trihexagonal()),
            ("kisrhombille", laves::kisrhombille()),
            ("prismatic-pentagonal", laves::prismatic_pentagonal()),
            ("floret-pentagonal", laves::floret_pentagonal()),
            ("kuniform-dual-p6m", dual::kuniform_dual_p6m()),
            ("kuniform-dual-n18", dual::kuniform_dual_n18()),
            ("kuniform-dual-n9", dual::kuniform_dual_n9()),
            ("kuniform-dual-n8", dual::kuniform_dual_n8()),
            ("kuniform-dual-n5", dual::kuniform_dual_n5()),
            ("kuniform-dual-n1", dual::kuniform_dual_n1()),
            ("kuniform-dual-n13", dual::kuniform_dual_n13()),
            ("kuniform-dual-n20", dual::kuniform_dual_n20()),
            ("kuniform-dual-pmm", dual::kuniform_dual_pmm()),
            ("kuniform-dual-n12", dual::kuniform_dual_n12()),
            ("kuniform-dual-n19", dual::kuniform_dual_n19()),
            ("kuniform-dual-n2", dual::kuniform_dual_n2()),
            ("windmill", windmill::spec()),
            ("basketweave", basketweave::basketweave()),
            ("herringbone", herringbone::herringbone()),
            ("running-bond", running_bond::running_bond()),
            ("stepped-hexominoes", stepped_hexominoes::stepped_hexominoes()),
            ("trapezoid-courses", trapezoid_courses::trapezoid_courses()),
            ("glide-hexagon-strips", glide_hexagon_strips::glide_hexagon_strips()),
            (
                "shifted-trapezoid-courses",
                shifted_trapezoid_courses::shifted_trapezoid_courses(),
            ),
            ("wavy-courses", wavy_courses::wavy_courses()),
            (
                "chiral-cut-honeycomb",
                chiral_cut_honeycomb::chiral_cut_honeycomb(),
            ),
            (
                "split-triangle-grid",
                split_triangle_grid::split_triangle_grid(),
            ),
            (
                "alcove-cut-grid",
                alcove_cut_grid::alcove_cut_grid(),
            ),
("stack-bond", stack_bond::stack_bond()),
            ("herringbone-45", herringbone_45::herringbone_45()),
            ("basketweave-3", basketweave_3::basketweave_3()),
            ("pentagon-1", pentagon1::spec()),
            ("pentagon-2", pentagon2::spec()),
            ("pentagon-3", pentagon3::spec()),
            ("pentagon-6", pentagon6::spec()),
            ("pentagon-7", pentagon7::spec()),
            ("pentagon-8", pentagon8::spec()),
            ("pentagon-9", pentagon9::spec()),
            ("pentagon-10", pentagon10::spec()),
            ("pentagon-11", pentagon11::spec()),
            ("pentagon-12", pentagon12::spec()),
            ("pentagon-13", pentagon13::spec()),
            ("pentagon-14", pentagon14::spec()),
            ("pentagon-15", pentagon15::spec()),
        ];
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

#[cfg(all(test, feature = "pentagon13"))]
mod pentagon13_tests {
    use super::pentagon13::{side_a, side_c, side_d, s_lat, tile_area};
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::pentagon13::spec;
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    fn shoelace_sign(poly: &Polygon) -> f64 {
        let n = poly.len();
        (0..n)
            .map(|i| {
                let (a, b) = (poly.pts[i], poly.pts[(i + 1) % n]);
                a.x * b.y - b.x * a.y
            })
            .sum()
    }

    #[test]
    fn pentagon_13_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(
            &t,
            "pentagon-13",
            &p,
            BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn pentagon_13_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-13(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(7.0, 7.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_13_deterministic() {
        check_det(
            &spec(),
            "pentagon-13",
            BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)),
        );
    }

    /// Unit structure of the primitive cell: 8 tiles of the exact
    /// Q(√3) prototile (A = C = D = 120°, B = E = 90°, d = 2a = 2e) on
    /// the rectangular lattice T1 = (3s, 3s), T2 = (s, −s), s = (9+√3)/13,
    /// with |det| = 8 × tile area and |T1| = 3|T2|, T1 ⊥ T2.
    #[test]
    fn pentagon_13_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.cells.len(), 8, "the primitive unit holds eight tiles");
        let s = s_lat();
        assert_eq!(t.lattice[0], Vec2::new(3.0 * s, 3.0 * s), "T1 = (3s, 3s)");
        assert_eq!(t.lattice[1], Vec2::new(s, -s), "T2 = (s, −s)");
        let det = t.lattice[0].cross(t.lattice[1]);
        assert!(
            (det.abs() - 8.0 * tile_area()).abs() < 1e-12,
            "lattice cell area {det} ≠ 8 × tile area"
        );
        let (t1, t2) = (t.lattice[0], t.lattice[1]);
        assert!((t1.dot(t2)).abs() < 1e-12, "lattice must be rectangular");
        assert!(
            (t1.length() - 3.0 * t2.length()).abs() < 1e-12,
            "|T1| must be 3×|T2|"
        );
        assert!((t2.length() - s * 2.0f64.sqrt()).abs() < 1e-12, "|T2| must be s√2");
        // Side lengths: a = e = (3√3+1)/13, d = 2a, c = (4−√3)/13.
        let a = side_a();
        let c = side_c();
        let d = side_d();
        assert!((d - 2.0 * a).abs() < 1e-12, "d = 2a");
        assert!(c < a && a < d);
        // Every cell: sides b, c, d/2, d/2, e, a and angles
        // 120°, 90°, 120°, 180° (the collinear d-midpoint), 120°, 90°.
        let want_sides = [1.0, c, d / 2.0, d / 2.0, a, a];
        let want_angles = [
            std::f64::consts::TAU / 3.0,
            std::f64::consts::FRAC_PI_2,
            std::f64::consts::TAU / 3.0,
            std::f64::consts::PI,
            std::f64::consts::TAU / 3.0,
            std::f64::consts::FRAC_PI_2,
        ];
        for cell in &t.cells {
            assert!(
                (cell.poly.area() - tile_area()).abs() < 1e-12,
                "cell area ≠ 9(14+3√3)/338"
            );
            let n = cell.poly.len();
            assert_eq!(n, 6, "cells carry the d-midpoint vertex");
            for i in 0..n {
                let (p0, p1, p2) =
                    (cell.poly.pts[(i + n - 1) % n], cell.poly.pts[i], cell.poly.pts[(i + 1) % n]);
                let len = (p2 - p1).length();
                assert!(
                    (len - want_sides[i]).abs() < 1e-12,
                    "side {i} = {len} ≠ {}",
                    want_sides[i]
                );
                let v1 = p0 - p1;
                let v2 = p2 - p1;
                let ang = (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos();
                assert!(
                    (ang - want_angles[i]).abs() < 1e-9,
                    "angle {i} = {} rad ≠ {}",
                    ang,
                    want_angles[i]
                );
            }
        }
    }

    /// 2-isohedral: seeds 0 and 1 are exactly the two chiral (isohedral)
    /// classes — four CCW cells and four CW cells — and every generated
    /// tile keeps the chirality of its seed (the cell map is a
    /// similarity, which cannot flip orientation).
    #[test]
    fn pentagon_13_seeds_track_the_chiral_pair() {
        let t = spec();
        let n0 = t
            .cells
            .iter()
            .filter(|c| c.seed == 0)
            .count();
        let n1 = t.cells.len() - n0;
        assert_eq!((n0, n1), (4, 4), "four tiles per chiral class");
        for c in &t.cells {
            let sign = if shoelace_sign(&c.poly) > 0.0 { 1 } else { -1 };
            let want = if c.seed == 0 { 1 } else { -1 };
            assert_eq!(sign, want, "seed {} cell has the wrong chirality", c.seed);
        }
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region: BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let mut seen = [false; 2];
        for tile in &r.tiles {
            assert!(tile.seed <= 1, "seeds must stay in {{0, 1}}");
            seen[tile.seed as usize] = true;
            let sign = if shoelace_sign(&tile.poly) > 0.0 { 1 } else { -1 };
            let want = if tile.seed == 0 { 1 } else { -1 };
            assert_eq!(sign, want, "generated tile flipped chirality (seed {})", tile.seed);
        }
        assert!(seen[0] && seen[1], "both chiral classes must appear");
    }

    /// The wallpaper group is p2 (pgg with the chiral pair distinguished):
    /// the 2-fold center is the midpoint of the prototile's long side
    /// (vertex M of the L0 cell), and 180° rotation about it preserves
    /// the tiling exactly.
    #[test]
    fn pentagon_13_c2_rotation_is_a_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let c = t.cells[0].poly.pts[3];
        let rot = |q: Vec2| c * 2.0 - q;
        let ra = BBox::new(v2(-4.0, -4.0), v2(10.0, 10.0));
        let cs: Vec<Vec2> = ra.corners().iter().map(|&q| rot(q)).collect();
        let rb = BBox::new(
            Vec2::new(
                cs.iter().map(|q| q.x).fold(f64::MAX, f64::min),
                cs.iter().map(|q| q.y).fold(f64::MAX, f64::min),
            ),
            Vec2::new(
                cs.iter().map(|q| q.x).fold(f64::MIN, f64::max),
                cs.iter().map(|q| q.y).fold(f64::MIN, f64::max),
            ),
        );
        let ra_tiles = t.generate(&p, &Query { region: ra });
        let rb_tiles = t.generate(&p, &Query { region: rb });
        assert!(ra_tiles.message.is_none(), "{:?}", ra_tiles.message);
        assert!(rb_tiles.message.is_none(), "{:?}", rb_tiles.message);
        assert!(!ra_tiles.tiles.is_empty());
        // Vertex key on a 1e-6 grid (distinct vertices are ≥ c = 0.174
        // apart; the two computations of a shared joint agree to ~1e-14).
        let key = |pts: &[Vec2]| {
            let mut v: Vec<(i64, i64)> = pts
                .iter()
                .map(|q| ((q.x * 1e6).round() as i64, (q.y * 1e6).round() as i64))
                .collect();
            v.sort_unstable();
            v
        };
        let set_b: std::collections::HashSet<Vec<(i64, i64)>> =
            rb_tiles.tiles.iter().map(|t| key(&t.poly.pts)).collect();
        let set_a: std::collections::HashSet<Vec<(i64, i64)>> = ra_tiles
            .tiles
            .iter()
            .map(|t| key(&t.poly.pts.iter().map(|q| rot(*q)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(set_a.len(), ra_tiles.tiles.len(), "rotation must be injective on the patch");
        assert_eq!(
            set_a, set_b,
            "180° rotation about the d-midpoint 2-fold center must preserve the tiling"
        );
    }

    #[test]
    fn pentagon_13_symmetry_metadata() {
        let t = spec();
        assert_eq!(t.id, "pentagon-13");
        assert_eq!(t.group, WallpaperGroup::P2);
        assert_eq!(t.lattice_kind, LatticeKind::Rectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the L0 cell (seed 0)");
    }
}

#[cfg(all(test, feature = "girih_ext"))]
mod girih_w85_tests {
    use super::girih_w85::{girih_w85, s3, L, STRAP_W};
    use super::lattice::LatticeTiling;
    use super::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn girih_w85_covers_region() {
        check_covers(
            &girih_w85(),
            "girih-w85",
            BBox::new(v2(-120.0, -90.0), v2(560.0, 420.0)),
        );
    }

    #[test]
    fn girih_w85_rotated_scaled() {
        check_rotated(
            &girih_w85(),
            "girih-w85(rot30,scale0.7)",
            BBox::new(v2(-150.0, -120.0), v2(520.0, 400.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn girih_w85_deterministic() {
        check_det(
            &girih_w85(),
            "girih-w85",
            BBox::new(v2(-100.0, -100.0), v2(600.0, 400.0)),
        );
    }

    /// Vertex-figure census of the shipped cell: 42 tiles on the hexagonal
    /// lattice with Σ areas = |det| exactly (Q(√3) identity verified in the
    /// spike), the exact star identities, and every interior angle a
    /// multiple of 30°.
    #[test]
    fn girih_w85_unit_structure() {
        let t = girih_w85();
        assert_eq!(t.cells.len(), 42, "42 tiles per primitive cell");
        let mut counts = [0usize; 7];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(
            counts, [1, 12, 6, 2, 3, 6, 12],
            "seed census: star 1, kite 12, pentagon 6, triangle 2, rectangle 3, quad 6, ink hexagon 12"
        );
        // Hexagonal lattice: |a2| = L, 60° between, det = L²√3/2.
        assert_eq!(t.lattice[0], Vec2::new(L, 0.0), "a1 = (L, 0)");
        let det = t.lattice[0].cross(t.lattice[1]);
        assert!((det - L * L * s3() / 2.0).abs() < 1e-9, "det = L²√3/2");
        assert!((t.lattice[1].length() - L).abs() < 1e-9, "|a2| = L");
        // Exact area identity: Σ cells == |det| (verified at 0 error in
        // Fraction-Q(√3) arithmetic in the Python spike).
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!(
            (sum - det).abs() < 1e-7,
            "cell areas {sum} must sum to |det| = {det}"
        );
        // Star identities at the origin-centred 24-gon.
        let d = L / 4.0 - STRAP_W / 2.0; // 66.48
        let star = &t.cells[0];
        assert_eq!(star.seed, 0);
        let n = star.poly.len();
        assert_eq!(n, 24, "the 12-pointed star is a 24-gon");
        let edge = d * (1.0 - 1.0 / s3());
        for i in 0..n {
            let p = star.poly.pts[i];
            let q = star.poly.pts[(i + 1) % n];
            let r = (q - p).length();
            assert!((r - edge).abs() < 1e-9, "star edge {r} ≠ d(1−1/√3) = {edge}");
            let want = if i % 2 == 0 { d * 2.0f64.sqrt() } else { 2.0 * d / s3() };
            assert!(
                (p.length() - want).abs() < 1e-9,
                "vertex {i} radius {} ≠ {want}",
                p.length()
            );
        }
        // Every interior angle of every cell is a multiple of 30°.
        for c in &t.cells {
            let k = c.poly.len();
            for i in 0..k {
                let p0 = c.poly.pts[(i + k - 1) % k];
                let p1 = c.poly.pts[i];
                let p2 = c.poly.pts[(i + 1) % k];
                let (e1, e2) = (p1 - p0, p2 - p1);
                let turn = e1.cross(e2).atan2(e1.dot(e2)).to_degrees();
                let interior = 180.0 - turn; // CCW cells: reflex joints give interior > 180
                let u = (interior / 30.0).round();
                assert!(
                    u > 0.0 && u < 12.0 && (interior - 30.0 * u).abs() < 1e-6,
                    "cell angle {interior}° is not a multiple of 30°"
                );
            }
        }
    }

    /// A candidate symmetry acting on world points.
    type Isometry = Box<dyn Fn(Vec2) -> Vec2>;

    /// Does the transformed image of one table cell equal a table cell
    /// translated by an exact integer lattice combination? (Float version of
    /// the exact-Q(√3) membership test from the spike.)
    fn image_is_a_tile(t: &LatticeTiling, pts: &[Vec2], f: impl Fn(Vec2) -> Vec2) -> bool {
        let det = t.lattice[0].cross(t.lattice[1]);
        for cand in &t.cells {
            let n = cand.poly.len();
            if n != pts.len() {
                continue;
            }
            for sh in 0..n {
                let d = cand.poly.pts[sh] - f(pts[0]);
                let i = d.cross(t.lattice[1]) / det;
                let j = t.lattice[0].cross(d) / det;
                if (i - i.round()).abs() > 1e-6 || (j - j.round()).abs() > 1e-6 {
                    continue;
                }
                let off = t.lattice[0] * i.round() + t.lattice[1] * j.round();
                if (0..n).all(|k| (f(pts[k]) + off - cand.poly.pts[(sh + k) % n]).length() < 1e-7)
                {
                    return true;
                }
            }
        }
        false
    }

    /// Wallpaper group p6, positively and negatively pinned on the exact
    /// table: 6-fold rotations at the star centres (lattice points), 2-fold
    /// rotations at the midpoints of all three lattice-edge classes, 3-fold
    /// rotations at (a1+a2)/3 and 2(a1+a2)/3 — and *no* 30° rotations and
    /// *no* mirrors (the over/under interlace is chiral; Wichmann's *632 is
    /// the motif without the ink partition).
    #[test]
    fn girih_w85_symmetry_is_p6() {
        let t = girih_w85();
        let s = s3() / 2.0;
        let rot = |deg: f64, c: Vec2| {
            let (cc, ss) = match deg {
                30.0 => (s3() / 2.0, 0.5),
                60.0 => (0.5, s),
                120.0 => (-0.5, s),
                180.0 => (-1.0, 0.0),
                _ => unreachable!(),
            };
            move |q: Vec2| {
                let d = q - c;
                v2(c.x + cc * d.x - ss * d.y, c.y + ss * d.x + cc * d.y)
            }
        };
        let mirror = |deg: f64| {
            // reflection in the line through the origin at angle `deg`:
            // matrix [[cos 2a, sin 2a], [sin 2a, −cos 2a]]
            let (cc, ss) = match deg {
                0.0 => (1.0, 0.0),
                30.0 => (0.5, s),
                60.0 => (-0.5, s),
                90.0 => (-1.0, 0.0),
                _ => unreachable!(),
            };
            move |q: Vec2| v2(cc * q.x + ss * q.y, ss * q.x - cc * q.y)
        };
        let a1 = t.lattice[0];
        let a2 = t.lattice[1];
        // Positive generators.
        for (deg, c, label) in [
            (60.0, v2(0.0, 0.0), "rot60 @ star centre (0,0)"),
            (60.0, a1, "rot60 @ star centre a1"),
            (120.0, (a1 + a2) * (1.0 / 3.0), "rot120 @ (a1+a2)/3"),
            (120.0, (a1 + a2) * (2.0 / 3.0), "rot120 @ 2(a1+a2)/3"),
            (180.0, a1 * 0.5, "rot180 @ a1/2"),
            (180.0, a2 * 0.5, "rot180 @ a2/2"),
            (180.0, (a1 + a2) * 0.5, "rot180 @ (a1+a2)/2"),
        ] {
            let f = rot(deg, c);
            for cell in &t.cells {
                assert!(
                    image_is_a_tile(&t, &cell.poly.pts, f),
                    "{label} must preserve the tiling (cell seed {} fails)",
                    cell.seed
                );
            }
        }
        // Negative candidates: 30° rotations and every mirror class fail.
        // (If any mirror existed anywhere, composing with the 6-fold
        // rotations would put one through a star centre in one of the
        // tested axis classes — p6m is the only wallpaper group properly
        // containing p6.)
        let mid = a1 * 0.5;
        let m30mid = {
            let m = mirror(30.0);
            move |q: Vec2| m(q - mid) + mid
        };
        let negatives: Vec<(&str, Isometry)> = vec![
            ("rot30 @ star centre", Box::new(rot(30.0, v2(0.0, 0.0)))),
            ("rot30 @ a1/2", Box::new(rot(30.0, mid))),
            ("mirror 0° @ star", Box::new(mirror(0.0))),
            ("mirror 30° @ star", Box::new(mirror(30.0))),
            ("mirror 60° @ star", Box::new(mirror(60.0))),
            ("mirror 90° @ star", Box::new(mirror(90.0))),
            ("mirror 30° @ a1/2", Box::new(m30mid)),
        ];
        for (label, f) in &negatives {
            let fails = t
                .cells
                .iter()
                .filter(|cell| !image_is_a_tile(&t, &cell.poly.pts, f))
                .count();
            assert!(
                fails > 0,
                "{label} must NOT be a symmetry (a chiral interlace has none)"
            );
        }
    }

    #[test]
    fn girih_w85_symmetry_metadata() {
        let t = girih_w85();
        assert_eq!(t.id, "girih-w85");
        assert_eq!(t.group, WallpaperGroup::P6);
        assert_eq!(t.lattice_kind, LatticeKind::Hexagonal);
        assert_eq!(t.category, TilingCategory::Islamic);
    }
}

#[cfg(all(test, feature = "hankin_ext"))]
mod pic_4_8_8_tests {
    use super::lattice::LatticeTiling;
    use super::pic_4_8_8::pic_4_8_8;
    use super::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    const RT2: f64 = core::f64::consts::SQRT_2;
    const A: f64 = 1.0 + RT2;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    fn sub(a: Vec2, b: Vec2) -> Vec2 {
        v2(a.x - b.x, a.y - b.y)
    }

    fn norm(a: Vec2) -> f64 {
        (a.x * a.x + a.y * a.y).sqrt()
    }

    type Isometry = Box<dyn Fn(Vec2) -> Vec2>;

    /// Is `k` an exact member of the (1/4)·Z[√2] surd grid? Exhaustive
    /// over the table's bounded range; distinct q representatives differ
    /// by √2 ≫ 1e-9, so the decision is exact here.
    fn on_quarter_surd_grid(k: f64) -> bool {
        let k4 = 4.0 * k;
        (-60..=60).any(|q| {
            let p = k4 - q as f64 * RT2;
            (p - p.round()).abs() < 1e-9
        })
    }

    /// Positive symmetry test: the image of the face `pts` under `f` must
    /// coincide, vertex for vertex, with some orbit face plus an EXACT
    /// lattice translation (integer multiples of the basis vectors). With
    /// all coordinates on the (1/4)·Z[√2] grid the nearest non-lattice
    /// offset is ≥ √2/4 ≈ 0.35, so the 1e-9 tolerance cannot blur a
    /// genuine near-miss into a hit.
    fn image_is_a_face(t: &LatticeTiling, pts: &[Vec2], f: &dyn Fn(Vec2) -> Vec2) -> bool {
        let img: Vec<Vec2> = pts.iter().map(|p| f(*p)).collect();
        for cell in &t.cells {
            let v = &cell.poly.pts;
            if v.len() != img.len() {
                continue;
            }
            for i in 0..v.len() {
                let d = sub(img[0], v[i]);
                let (mx, my) = (d.x / A, d.y / A);
                if (mx - mx.round()).abs() > 1e-9 || (my - my.round()).abs() > 1e-9 {
                    continue;
                }
                let shift = v2(mx.round() * A, my.round() * A);
                // Vertex-SET match (orientation-free: mirrors reverse the
                // winding of the CCW-normalised table).
                let target: Vec<Vec2> = v.iter().map(|p| *p + shift).collect();
                let mut used = vec![false; target.len()];
                let mut ok = true;
                for p in &img {
                    let hit = target.iter().position(|q| norm(sub(*p, *q)) < 1e-9);
                    match hit {
                        Some(j) if !used[j] => used[j] = true,
                        _ => {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    return true;
                }
            }
        }
        false
    }

    fn all_faces(t: &LatticeTiling) -> Vec<Vec<Vec2>> {
        t.cells.iter().map(|c| c.poly.pts.clone()).collect()
    }

    #[test]
    fn pic_4_8_8_covers_region() {
        check_covers(
            &pic_4_8_8(),
            "pic-4-8-8",
            BBox::new(v2(-24.0, -18.0), v2(24.0, 18.0)),
        );
    }

    #[test]
    fn pic_4_8_8_rotated_scaled() {
        check_rotated(
            &pic_4_8_8(),
            "pic-4-8-8(rot20,scale0.6)",
            BBox::new(v2(-24.0, -18.0), v2(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn pic_4_8_8_deterministic() {
        check_det(
            &pic_4_8_8(),
            "pic-4-8-8",
            BBox::new(v2(-12.0, -9.0), v2(12.0, 9.0)),
        );
    }

    #[test]
    fn pic_4_8_8_unit_structure() {
        let t = pic_4_8_8();
        assert_eq!(t.cells.len(), 14);
        let by_seed = |s: u32| t.cells.iter().filter(|c| c.seed == s).count();
        assert_eq!(
            (by_seed(0), by_seed(1), by_seed(2), by_seed(3)),
            (1, 1, 4, 8),
            "orbit census: octagon, small square, 4 houses, 8 star tips"
        );

        // Exact Q(√2) construction: every coordinate is a (1/4)·Z[√2] surd.
        for c in &t.cells {
            for p in &c.poly.pts {
                assert!(on_quarter_surd_grid(p.x), "vertex x {} off the grid", p.x);
                assert!(on_quarter_surd_grid(p.y), "vertex y {} off the grid", p.y);
            }
        }

        // Σ(cell areas) = |det(lattice)| — exact over Q(√2) (spike):
        // (1+√2) + 1/2 + 4(1/8+√2/4) + 8(1/8) = 3+2√2 = (1+√2)².
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        assert!((det - A * A).abs() < 1e-12);
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-12, "Σ areas {sum} vs det {det}");

        // Octagon (seed 0): regular, side √2/2, centred on the octagon
        // centre (A/2, A/2).
        let oct = t.cells.iter().find(|c| c.seed == 0).unwrap();
        let v = &oct.poly.pts;
        assert_eq!(v.len(), 8);
        for i in 0..v.len() {
            let side = norm(sub(v[(i + 1) % 8], v[i]));
            assert!((side - RT2 / 2.0).abs() < 1e-12, "octagon side {side}");
        }
        let cen = v.iter().fold(Vec2::ZERO, |a, p| a + *p);
        assert!(norm(sub(cen / 8.0, v2(A / 2.0, A / 2.0))) < 1e-12);
        assert!((oct.poly.area() - A).abs() < 1e-12, "octagon area = 1+√2");

        // Small square (seed 1): side √2/2 (the [±√2/4]² khatam centre),
        // centred on the origin.
        let sq = t.cells.iter().find(|c| c.seed == 1).unwrap();
        let v = &sq.poly.pts;
        assert_eq!(v.len(), 4);
        for i in 0..4 {
            let side = norm(sub(v[(i + 1) % 4], v[i]));
            assert!((side - RT2 / 2.0).abs() < 1e-12, "square side {side}");
        }
        assert!((sq.poly.area() - 0.5).abs() < 1e-12, "square area = 1/2");

        // House pentagons (seed 2): area 1/8 + √2/4 each.
        let mut houses = 0;
        for c in t.cells.iter().filter(|c| c.seed == 2) {
            assert_eq!(c.poly.pts.len(), 5);
            assert!((c.poly.area() - (0.125 + RT2 / 4.0)).abs() < 1e-12);
            houses += 1;
        }
        assert_eq!(houses, 4);

        // Star tips (seed 3): isosceles right triangles, legs 1/2, area
        // exactly 1/8 — the octagram arm caps between the two overlaid
        // squares and the substrate walls.
        for c in t.cells.iter().filter(|c| c.seed == 3) {
            let v = &c.poly.pts;
            assert_eq!(v.len(), 3);
            assert!((c.poly.area() - 0.125).abs() < 1e-12, "tip area = 1/8");
            let right = (0..3).any(|i| {
                let e1 = sub(v[(i + 1) % 3], v[i]);
                let e2 = sub(v[(i + 2) % 3], v[i]);
                let dot = e1.x * e2.x + e1.y * e2.y;
                let l1 = e1.x * e1.x + e1.y * e1.y;
                let l2 = e2.x * e2.x + e2.y * e2.y;
                dot.abs() < 1e-12 && (l1 - 0.25).abs() < 1e-12 && (l2 - 0.25).abs() < 1e-12
            });
            assert!(right, "the tip is a right isosceles triangle with legs 1/2");
        }

        // Every interior angle is a multiple of 45° (Hankin contacts at
        // δ = 45° produce only 45°, 90° and 135° corners — the spike's
        // census).
        for c in &t.cells {
            let v = &c.poly.pts;
            let n = v.len();
            for i in 0..n {
                let e1 = sub(v[(i + n - 1) % n], v[i]);
                let e2 = sub(v[(i + 1) % n], v[i]);
                let ang = (e1.x * e2.y - e1.y * e2.x)
                    .abs()
                    .atan2(e1.x * e2.x + e1.y * e2.y);
                let k = (ang / std::f64::consts::FRAC_PI_4).round();
                assert!(
                    k >= 1.0 && (ang - k * std::f64::consts::FRAC_PI_4).abs() < 1e-9,
                    "angle {ang} at {:?} is not a multiple of 45°",
                    v[i]
                );
            }
        }
    }

    #[test]
    fn pic_4_8_8_symmetry_is_p4m() {
        let t = pic_4_8_8();
        let faces = all_faces(&t);

        // Positive probes: each must map every face to a face. All
        // matrices are exact integer/reflection maps (no trig).
        let c = v2(A / 2.0, A / 2.0); // octagon centre
        let e1 = v2(A / 2.0, 0.0); // octagon-octagon contact
        let e2 = v2(0.0, A / 2.0); // substrate-edge contact
        let positives: Vec<(&str, Isometry)> = vec![
            ("rot90 @ substrate square centre", Box::new(|p| v2(-p.y, p.x))),
            (
                "rot90 @ octagon centre",
                Box::new(move |p| {
                    let d = sub(p, c);
                    v2(c.x - d.y, c.y + d.x)
                }),
            ),
            (
                "rot180 @ octagon-octagon contact",
                Box::new(move |p| v2(2.0 * e1.x - p.x, 2.0 * e1.y - p.y)),
            ),
            (
                "rot180 @ square-square contact",
                Box::new(move |p| v2(2.0 * e2.x - p.x, 2.0 * e2.y - p.y)),
            ),
            ("mirror y = 0", Box::new(|p| v2(p.x, -p.y))),
            ("mirror x = 0", Box::new(|p| v2(-p.x, p.y))),
            ("mirror y = A/2", Box::new(move |p| v2(p.x, 2.0 * c.y - p.y))),
            ("mirror y = x", Box::new(|p| v2(p.y, p.x))),
            ("mirror y = -x", Box::new(|p| v2(-p.y, -p.x))),
        ];
        for (label, f) in &positives {
            for (i, face) in faces.iter().enumerate() {
                assert!(
                    image_is_a_face(&t, face, f),
                    "{label} must be a symmetry but fails on face {i}"
                );
            }
        }

        // Negative probes: the symmetry group is EXACTLY p4m — a quarter-
        // step rotation off the centres, a half-step twirl on a cell edge
        // line and a glide reflection must all fail on some face.
        let q = v2(A / 4.0, A / 4.0);
        let negatives: Vec<(&str, Isometry)> = vec![
            (
                "rot90 @ (A/4, A/4)",
                Box::new(move |p| {
                    let d = sub(p, q);
                    v2(q.x - d.y, q.y + d.x)
                }),
            ),
            (
                "rot180 @ (A/4, 0)",
                Box::new(move |p| v2(2.0 * q.x - p.x, -p.y)),
            ),
            ("glide (A/2, A/2−y)", Box::new(move |p| v2(p.x + A / 2.0, 2.0 * q.y - p.y))),
        ];
        for (label, f) in &negatives {
            let fails = faces.iter().filter(|face| !image_is_a_face(&t, face, f)).count();
            assert!(fails > 0, "{label} must NOT be a symmetry");
        }
    }

    #[test]
    fn pic_4_8_8_symmetry_metadata() {
        let t = pic_4_8_8();
        assert_eq!(t.id, "pic-4-8-8");
        assert_eq!(t.group, WallpaperGroup::P4m);
        assert_eq!(t.lattice_kind, LatticeKind::Square);
        assert_eq!(t.category, TilingCategory::Islamic);
    }
}


#[cfg(all(test, feature = "pentagon14"))]
mod pentagon14_tests {
    use super::pentagon14::{beta, bp, bq, p, pq, q, s57, tile_area};
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::pentagon14::spec;
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn pentagon_14_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(
            &t,
            "pentagon-14",
            &p,
            BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn pentagon_14_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-14(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(7.0, 7.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_14_deterministic() {
        check_det(
            &spec(),
            "pentagon-14",
            BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)),
        );
    }

    /// Unit structure of the primitive cell: 6 tiles of the exact
    /// Q(√57, β, p, bq) prototile (A = 90°, 2B + C = 360°, C + E = 180°,
    /// 2a = 2c = d = e) on the oblique lattice w1 = (2pq, −(13+9√57)/16),
    /// w2 = (−(β+2p), −(√57−1)/2), with |det| = 6 × tile area and
    /// w1 · w2 ≠ 0.
    #[test]
    fn pentagon_14_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-14");
        assert_eq!(t.group, WallpaperGroup::P2);
        assert_eq!(t.lattice_kind, LatticeKind::Oblique);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the D0 cell (seed 0)");
        assert_eq!(t.cells.len(), 6, "the primitive unit holds six tiles");
        assert_eq!(
            t.lattice[0],
            Vec2::new(2.0 * pq(), -(13.0 + 9.0 * s57()) / 16.0),
            "w1 = (2pq, −(13+9√57)/16)"
        );
        assert_eq!(
            t.lattice[1],
            Vec2::new(-(beta() + 2.0 * p()), -(s57() - 1.0) / 2.0),
            "w2 = (−(β+2p), −(√57−1)/2)"
        );
        let det = t.lattice[0].cross(t.lattice[1]);
        assert!(
            (det.abs() - 6.0 * tile_area()).abs() < 1e-12,
            "lattice cell area {det} ≠ 6 × tile area"
        );
        assert!(
            (t.lattice[0].dot(t.lattice[1])).abs() > 1.0,
            "lattice must be oblique (w1 · w2 ≠ 0)"
        );
        // Exact surd identities the cell table relies on.
        assert!((bp() - beta() * p()).abs() < 1e-12, "βp = (43−√57)/16");
        assert!((bq() - beta() * q()).abs() < 1e-12, "βq = √((438√57−2706)/256)");
        // Angles of the prototile, derived from the exact sin B = q.
        let b = beta();
        let b_ = std::f64::consts::PI - q().asin();
        let c_ = 2.0 * std::f64::consts::PI - 2.0 * b_;
        let d_ = 3.0 * std::f64::consts::FRAC_PI_2 - b_;
        let e_ = 2.0 * b_ - std::f64::consts::PI;
        // Per-class fingerprints: vertex count, side lengths and interior
        // angles in cyclic order (all cells are CCW).
        for cell in &t.cells {
            assert!(
                (cell.poly.area() - tile_area()).abs() < 1e-12,
                "cell area ≠ exact tile area"
            );
            let (want_sides, want_angles): (&[f64], &[f64]) = match cell.seed {
                0 => (
                    &[b, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
                    &[
                        std::f64::consts::FRAC_PI_2,
                        b_,
                        c_,
                        std::f64::consts::PI,
                        d_,
                        std::f64::consts::PI,
                        e_,
                    ],
                ),
                1 => (
                    &[1.0, 2.0, 2.0, 1.0, b],
                    &[std::f64::consts::FRAC_PI_2, e_, d_, c_, b_],
                ),
                _ => (
                    &[2.0, 1.0, 1.0, 1.0, b, 1.0],
                    &[
                        e_,
                        d_,
                        std::f64::consts::PI,
                        c_,
                        b_,
                        std::f64::consts::FRAC_PI_2,
                    ],
                ),
            };
            let n = cell.poly.len();
            assert_eq!(n, want_sides.len(), "seed {} vertex count", cell.seed);
            for i in 0..n {
                let (p0, p1, p2) = (
                    cell.poly.pts[(i + n - 1) % n],
                    cell.poly.pts[i],
                    cell.poly.pts[(i + 1) % n],
                );
                let len = (p2 - p1).length();
                assert!(
                    (len - want_sides[i]).abs() < 1e-12,
                    "side {i} = {len} ≠ {}",
                    want_sides[i]
                );
                let v1 = p0 - p1;
                let v2 = p2 - p1;
                let ang = (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos();
                assert!(
                    (ang - want_angles[i]).abs() < 1e-9,
                    "angle {i} = {} rad ≠ {}",
                    ang,
                    want_angles[i]
                );
            }
        }
    }

    /// 3-isohedral: seeds 0, 1, 2 are the three isohedral classes — two
    /// tiles of each in the primitive cell — and every generated tile
    /// keeps the vertex count and side multiset of its seed class.
    #[test]
    fn pentagon_14_isohedral_classes() {
        let t = spec();
        let mut counts = [0usize; 3];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(counts, [2, 2, 2], "two tiles per isohedral class");
        let params = ParamSet::default_for(t.params());
        let r = t.generate(&params, &Query { region: BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let b = beta();
        let mut seen = [false; 3];
        for tile in &r.tiles {
            let s = tile.seed as usize;
            assert!(s < 3, "seeds must stay in {{0, 1, 2}}");
            seen[s] = true;
            let want_sides: &[f64] = match tile.seed {
                0 => &[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, b],
                1 => &[1.0, 1.0, 2.0, 2.0, b],
                _ => &[1.0, 1.0, 1.0, 1.0, 2.0, b],
            };
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| {
                    (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length()
                })
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            assert_eq!(sides.len(), want_sides.len(), "seed {s} vertex count");
            for (got, want) in sides.iter().zip(want_sides) {
                assert!(
                    (got - want).abs() < 1e-12,
                    "seed {s} side {got} ∉ {{unit, 2, β}}"
                );
            }
        }
        assert!(
            seen[0] && seen[1] && seen[2],
            "all three isohedral classes must appear"
        );
    }

    /// The wallpaper group is p2: the 2-fold centre is the midpoint
    /// O = (β + p/2, q/2) of the segment joining the A-corners of the
    /// D0/D180 pair, and 180° rotation about it preserves the tiling
    /// (each class onto itself: D0 ↔ D180, M0 ↔ M180, MTHS ↔ MTHSp180).
    #[test]
    fn pentagon_14_c2_symmetry() {
        let t = spec();
        let params = ParamSet::default_for(t.params());
        let c = Vec2::new((2.0 * beta() + p()) / 2.0, q() / 2.0);
        let rot = |pt: Vec2| c * 2.0 - pt;
        let ra = BBox::new(v2(-4.0, -4.0), v2(10.0, 10.0));
        let cs: Vec<Vec2> = ra.corners().iter().map(|&pt| rot(pt)).collect();
        let rb = BBox::new(
            Vec2::new(
                cs.iter().map(|pt| pt.x).fold(f64::MAX, f64::min),
                cs.iter().map(|pt| pt.y).fold(f64::MAX, f64::min),
            ),
            Vec2::new(
                cs.iter().map(|pt| pt.x).fold(f64::MIN, f64::max),
                cs.iter().map(|pt| pt.y).fold(f64::MIN, f64::max),
            ),
        );
        let ra_tiles = t.generate(&params, &Query { region: ra });
        let rb_tiles = t.generate(&params, &Query { region: rb });
        assert!(ra_tiles.message.is_none(), "{:?}", ra_tiles.message);
        assert!(rb_tiles.message.is_none(), "{:?}", rb_tiles.message);
        assert!(!ra_tiles.tiles.is_empty());
        // Vertex key on a 1e-6 grid (distinct tiling vertices are ≥ 1
        // unit apart — every fine edge has length 1; the two routes to a
        // shared joint agree to ~1e-15).
        let key = |pts: &[Vec2]| {
            let mut v: Vec<(i64, i64)> = pts
                .iter()
                .map(|pt| ((pt.x * 1e6).round() as i64, (pt.y * 1e6).round() as i64))
                .collect();
            v.sort_unstable();
            v
        };
        let set_b: std::collections::HashSet<Vec<(i64, i64)>> =
            rb_tiles.tiles.iter().map(|t| key(&t.poly.pts)).collect();
        let set_a: std::collections::HashSet<Vec<(i64, i64)>> = ra_tiles
            .tiles
            .iter()
            .map(|t| key(&t.poly.pts.iter().map(|pt| rot(*pt)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(
            set_a.len(),
            ra_tiles.tiles.len(),
            "rotation must be injective on the patch"
        );
        assert_eq!(
            set_a, set_b,
            "180° rotation about the 2-fold centre O must preserve the tiling"
        );
    }
}

#[cfg(all(test, feature = "hankin_ext"))]
mod pic_3_6_3_6_tests {
    use super::lattice::LatticeTiling;
    use super::pic_3_6_3_6::pic_3_6_3_6;
    use super::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    /// √3 — `2·(√3/2)`, the substrate's apothem constant doubled (exact in
    /// f64); `consts::SQRT_3` is unstable at this crate's MSRV.
    const RT3: f64 = 2.0 * 0.866_025_403_784_438_6;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    fn sub(a: Vec2, b: Vec2) -> Vec2 {
        v2(a.x - b.x, a.y - b.y)
    }

    fn norm(a: Vec2) -> f64 {
        (a.x * a.x + a.y * a.y).sqrt()
    }

    type Isometry = Box<dyn Fn(Vec2) -> Vec2>;

    /// Rotation by 60°k about `c`, as an exact matrix in Q(√3) (no trig).
    fn rot(c: Vec2, k: usize) -> Isometry {
        let (co, si): (f64, f64) = match k % 6 {
            0 => (1.0, 0.0),
            1 => (0.5, RT3 / 2.0),
            2 => (-0.5, RT3 / 2.0),
            3 => (-1.0, 0.0),
            4 => (-0.5, -RT3 / 2.0),
            _ => (0.5, -RT3 / 2.0),
        };
        Box::new(move |p| {
            let d = sub(p, c);
            v2(c.x + co * d.x - si * d.y, c.y + si * d.x + co * d.y)
        })
    }

    /// Reflection in the line through `c` with unit direction `(u, w)`.
    fn refl(c: Vec2, u: f64, w: f64) -> Isometry {
        Box::new(move |p| {
            let d = sub(p, c);
            let pr = 2.0 * (u * d.x + w * d.y);
            v2(c.x + pr * u - d.x, c.y + pr * w - d.y)
        })
    }

    /// Is `k` an exact member of the (1/4)·Z[√3] surd grid? Exhaustive over
    /// the table's bounded range; for |q| ≤ 60 the closest q√3 gets to an
    /// integer is 97 − 56√3 ≈ 5.1e-3 ≫ 1e-9, so the decision is exact here.
    fn on_quarter_surd_grid(k: f64) -> bool {
        let k4 = 4.0 * k;
        (-60..=60).any(|q| {
            let p = k4 - q as f64 * RT3;
            (p - p.round()).abs() < 1e-9
        })
    }

    /// Positive symmetry test: the image of the face `pts` under `f` must
    /// coincide, vertex for vertex, with some orbit face plus an EXACT
    /// lattice translation `m·(2,0) + n·(1,√3)`. Table vertices live on the
    /// (1/4)·Z × (√3/4)·Z grid, whose lattice coordinates are multiples of
    /// 1/8 and 1/4, so the nearest non-lattice offset is ≈ 0.25 world units
    /// and the 1e-9 tolerance cannot blur a genuine near-miss into a hit.
    fn image_is_a_face(t: &LatticeTiling, pts: &[Vec2], f: &dyn Fn(Vec2) -> Vec2) -> bool {
        let img: Vec<Vec2> = pts.iter().map(|p| f(*p)).collect();
        for cell in &t.cells {
            let v = &cell.poly.pts;
            if v.len() != img.len() {
                continue;
            }
            for i in 0..v.len() {
                let d = sub(img[0], v[i]);
                // d = m·(2,0) + n·(1,√3)  ⟹  n = d.y/√3, m = (d.x − n)/2
                let n = d.y / RT3;
                let m = (d.x - n) / 2.0;
                if (m - m.round()).abs() > 1e-9 || (n - n.round()).abs() > 1e-9 {
                    continue;
                }
                let (mi, ni) = (m.round(), n.round());
                let shift = v2(2.0 * mi + ni, ni * RT3);
                // Vertex-SET match (orientation-free: mirrors reverse the
                // winding of the CCW-normalised table).
                let target: Vec<Vec2> = v.iter().map(|p| *p + shift).collect();
                let mut used = vec![false; target.len()];
                let mut ok = true;
                for p in &img {
                    let hit = target.iter().position(|q| norm(sub(*p, *q)) < 1e-9);
                    match hit {
                        Some(j) if !used[j] => used[j] = true,
                        _ => {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    return true;
                }
            }
        }
        false
    }

    fn all_faces(t: &LatticeTiling) -> Vec<Vec<Vec2>> {
        t.cells.iter().map(|c| c.poly.pts.clone()).collect()
    }

    #[test]
    fn pic_3_6_3_6_covers_region() {
        check_covers(
            &pic_3_6_3_6(),
            "pic-3-6-3-6",
            BBox::new(v2(-24.0, -18.0), v2(24.0, 18.0)),
        );
    }

    #[test]
    fn pic_3_6_3_6_rotated_scaled() {
        check_rotated(
            &pic_3_6_3_6(),
            "pic-3-6-3-6(rot20,scale0.6)",
            BBox::new(v2(-24.0, -18.0), v2(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn pic_3_6_3_6_deterministic() {
        check_det(
            &pic_3_6_3_6(),
            "pic-3-6-3-6",
            BBox::new(v2(-12.0, -9.0), v2(12.0, 9.0)),
        );
    }

    #[test]
    fn pic_3_6_3_6_unit_structure() {
        let t = pic_3_6_3_6();
        assert_eq!(t.cells.len(), 12);
        let by_seed = |s: u32| t.cells.iter().filter(|c| c.seed == s).count();
        assert_eq!(
            (by_seed(0), by_seed(1), by_seed(2)),
            (4, 6, 2),
            "orbit census: 4 hexagram-core hexagons, 6 hexagram tips, 2 midsegment triangles"
        );

        // Exact Q(√3) construction: every coordinate is a (1/4)·Z[√3] surd.
        for c in &t.cells {
            for p in &c.poly.pts {
                assert!(on_quarter_surd_grid(p.x), "vertex x {} off the grid", p.x);
                assert!(on_quarter_surd_grid(p.y), "vertex y {} off the grid", p.y);
            }
        }

        // Σ(cell areas) = |det(lattice)| — exact over Q(√3) (spike):
        // 4·(3√3/8) + 8·(√3/16) = 3√3/2 + √3/2 = 2√3.
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        assert!((det - 2.0 * RT3).abs() < 1e-12, "det = 2√3, got {det}");
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-12, "Σ areas {sum} vs det {det}");

        // Seed 0 — four regular hexagons of side 1/2 (the hexagram cores),
        // centred on the substrate hexagon centre and on the three
        // hexagon–hexagon contacts.
        let mut centres: Vec<(f64, f64)> = Vec::new();
        for c in t.cells.iter().filter(|c| c.seed == 0) {
            let v = &c.poly.pts;
            assert_eq!(v.len(), 6);
            for i in 0..6 {
                let side = norm(sub(v[(i + 1) % 6], v[i]));
                assert!((side - 0.5).abs() < 1e-12, "hexagon side {side}");
            }
            assert!(
                (c.poly.area() - 3.0 * RT3 / 8.0).abs() < 1e-12,
                "hexagon area = 3√3/8"
            );
            let cen = v.iter().fold(Vec2::ZERO, |a, p| a + *p);
            let cen = cen / 6.0;
            centres.push((cen.x, cen.y));
        }
        let want = [
            (0.0, 0.0),
            (1.0, 0.0),
            (0.5, RT3 / 2.0),
            (1.5, RT3 / 2.0),
        ];
        for w in want {
            assert!(
                centres.iter().any(|c| norm(sub(v2(c.0, c.1), v2(w.0, w.1))) < 1e-12),
                "no hexagram core centred on {:?}",
                w
            );
        }
        assert_eq!(centres.len(), 4);

        // Seeds 1 and 2 — eight equilateral triangles of side 1/2 and area
        // √3/16: the six hexagram arms and the two midsegment cuts.
        for c in t.cells.iter().filter(|c| c.seed != 0) {
            let v = &c.poly.pts;
            assert_eq!(v.len(), 3);
            for i in 0..3 {
                let side = norm(sub(v[(i + 1) % 3], v[i]));
                assert!((side - 0.5).abs() < 1e-12, "triangle side {side}");
            }
            assert!(
                (c.poly.area() - RT3 / 16.0).abs() < 1e-12,
                "triangle area = √3/16"
            );
        }
        // The tip apexes ride the six edge midpoints of the substrate hexagon
        // at O (circumradius 1, apothem √3/2) — the hexagram's arms.
        let m_ids = [(0.75, RT3 / 4.0), (0.0, RT3 / 2.0), (-0.75, RT3 / 4.0),
                     (-0.75, -RT3 / 4.0), (0.0, -RT3 / 2.0), (0.75, -RT3 / 4.0)];
        for m in m_ids {
            let hit = t.cells.iter().filter(|c| c.seed == 1).any(|c| {
                c.poly
                    .pts
                    .iter()
                    .any(|p| norm(sub(*p, v2(m.0, m.1))) < 1e-12)
            });
            assert!(hit, "no hexagram tip apex on the midpoint {:?}", m);
        }

        // Every interior angle is 60° or 120° — the sixfold engine at
        // δ = 60° produces only those two corners (the spike's census:
        // {60, 120} and nothing else).
        for c in &t.cells {
            let v = &c.poly.pts;
            let n = v.len();
            for i in 0..n {
                let e1 = sub(v[(i + n - 1) % n], v[i]);
                let e2 = sub(v[(i + 1) % n], v[i]);
                let ang = (e1.x * e2.y - e1.y * e2.x)
                    .abs()
                    .atan2(e1.x * e2.x + e1.y * e2.y);
                let k = (ang / std::f64::consts::FRAC_PI_6).round();
                assert!(
                    (ang - k * std::f64::consts::FRAC_PI_6).abs() < 1e-9,
                    "angle {ang} at {:?} is not a multiple of 30°",
                    v[i]
                );
                assert!(
                    (ang - std::f64::consts::FRAC_PI_3).abs() < 1e-9
                        || (ang - 2.0 * std::f64::consts::FRAC_PI_3).abs() < 1e-9,
                    "angle {ang} at {:?} is neither 60° nor 120°",
                    v[i]
                );
            }
        }
    }

    #[test]
    fn pic_3_6_3_6_symmetry_is_p6m() {
        let t = pic_3_6_3_6();
        let faces = all_faces(&t);

        // Positive probes: each must map every face to a face. Every centre
        // and axis below is an exact Q(√3) point and every map an exact
        // Q(√3) matrix (no trig).
        let o = v2(0.0, 0.0); // hexagon centre
        let w = v2(1.0, 0.0); // hexagon–hexagon contact
        let tri = v2(1.0, RT3 / 3.0); // substrate triangle centre
        let p0 = v2(0.5, 0.0); // wall midline point
        let positives: Vec<(&str, Isometry)> = vec![
            ("rot60 @ hexagon centre", rot(o, 1)),
            ("rot60 @ hexagon-hexagon contact", rot(w, 1)),
            ("rot120 @ substrate triangle centre", rot(tri, 2)),
            ("rot180 @ midline point", rot(p0, 3)),
            ("mirror y = 0", refl(o, 1.0, 0.0)),
            ("mirror x = 0", refl(o, 0.0, 1.0)),
            ("mirror y = √3", refl(v2(0.0, RT3), 1.0, 0.0)),
            ("mirror x = 2", refl(v2(2.0, 0.0), 0.0, 1.0)),
            ("mirror at 60° through O", refl(o, 0.5, RT3 / 2.0)),
        ];
        for (label, f) in &positives {
            for (i, face) in faces.iter().enumerate() {
                assert!(
                    image_is_a_face(&t, face, f),
                    "{label} must be a symmetry but fails on face {i}"
                );
            }
        }

        // Negative probes: the symmetry group is EXACTLY p6m — a right-angle
        // turn on the sixfold centre, a sixfold turn on the threefold centre,
        // a half turn off the 2-fold sites and a mirror between two mirror
        // lines must all fail on some face.
        let negatives: Vec<(&str, Isometry)> = vec![
            ("rot90 @ hexagon centre", Box::new(|p| v2(-p.y, p.x))),
            ("rot60 @ substrate triangle centre", rot(tri, 1)),
            (
                "rot180 @ (3/8, √3/8)",
                Box::new(|p| v2(0.75 - p.x, RT3 / 4.0 - p.y)),
            ),
            ("mirror y = √3/4", refl(v2(0.0, RT3 / 4.0), 1.0, 0.0)),
        ];
        for (label, f) in &negatives {
            let fails = faces.iter().filter(|face| !image_is_a_face(&t, face, f)).count();
            assert!(fails > 0, "{label} must NOT be a symmetry");
        }
    }

    #[test]
    fn pic_3_6_3_6_symmetry_metadata() {
        let t = pic_3_6_3_6();
        assert_eq!(t.id, "pic-3-6-3-6");
        assert_eq!(t.group, WallpaperGroup::P6m);
        assert_eq!(t.lattice_kind, LatticeKind::Hexagonal);
        assert_eq!(t.category, TilingCategory::Islamic);
    }
}
#[cfg(all(test, feature = "hankin_ext"))]
mod pic_3_12_12_tests {
    use super::lattice::LatticeTiling;
    use super::pic_3_12_12::pic_3_12_12;
    use super::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    /// √3 — `2·(√3/2)`, the substrate's apothem constant doubled (exact in
    /// f64); `consts::SQRT_3` is unstable at this crate's MSRV.
    const RT3: f64 = 2.0 * 0.866_025_403_784_438_6;
    /// The lattice basis (the 3.12.12 substrate's own translations):
    /// `a1 = (3/2+√3, 1+√3/2)`, `a2 = (0, 2+√3)`.
    const A1X: f64 = 1.5 + RT3;
    const A1Y: f64 = 1.0 + RT3 / 2.0;
    const A2Y: f64 = 2.0 + RT3;
    /// Substrate dodecagon apothem `1 + √3/2`.
    const RAP: f64 = 1.0 + RT3 / 2.0;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    fn sub(a: Vec2, b: Vec2) -> Vec2 {
        v2(a.x - b.x, a.y - b.y)
    }

    fn norm(a: Vec2) -> f64 {
        (a.x * a.x + a.y * a.y).sqrt()
    }

    fn cross(a: Vec2, b: Vec2) -> f64 {
        a.x * b.y - a.y * b.x
    }

    type Isometry = Box<dyn Fn(Vec2) -> Vec2>;

    /// Rotation by 60°·k about `c`, as an exact matrix in Q(√3) (no trig).
    fn rot(c: Vec2, k: usize) -> Isometry {
        let (co, si): (f64, f64) = match k % 6 {
            0 => (1.0, 0.0),
            1 => (0.5, RT3 / 2.0),
            2 => (-0.5, RT3 / 2.0),
            3 => (-1.0, 0.0),
            4 => (-0.5, -RT3 / 2.0),
            _ => (0.5, -RT3 / 2.0),
        };
        Box::new(move |p| {
            let d = sub(p, c);
            v2(c.x + co * d.x - si * d.y, c.y + si * d.x + co * d.y)
        })
    }

    /// Rotation by 30° about `c`; cos 30° = √3/2 and sin 30° = 1/2, so this
    /// too is exact over Q(√3). Used as a negative probe.
    fn rot30(c: Vec2) -> Isometry {
        Box::new(move |p| {
            let d = sub(p, c);
            v2(
                c.x + RT3 / 2.0 * d.x - 0.5 * d.y,
                c.y + 0.5 * d.x + RT3 / 2.0 * d.y,
            )
        })
    }

    /// Reflection in the line through `c` with unit direction `(u, w)`.
    fn refl(c: Vec2, u: f64, w: f64) -> Isometry {
        Box::new(move |p| {
            let d = sub(p, c);
            let pr = 2.0 * (u * d.x + w * d.y);
            v2(c.x + pr * u - d.x, c.y + pr * w - d.y)
        })
    }

    /// Is `k` an exact member of the (1/24)·Z[√3] surd grid? Exhaustive over
    /// the table's bounded numerators (|p| ≤ 33, |q| ≤ 18): for q ≠ Q the gap
    /// `|P + (Q−q)√3| ≥ 1/(|P| + |Q−q|√3) ≈ 7.5e-3`, so the 1e-9 decision is
    /// exact here — no near-miss can be rounded into a hit.
    fn on_twentyfourth_surd_grid(k: f64) -> bool {
        let k24 = 24.0 * k;
        (-60..=60).any(|q| {
            let p = k24 - q as f64 * RT3;
            (p - p.round()).abs() < 1e-9
        })
    }

    /// Every corner angle of a face, rounded to the 30° lattice; asserts
    /// outright that no corner falls off that lattice.
    fn corners(v: &[Vec2]) -> Vec<u32> {
        let n = v.len();
        (0..n)
            .map(|i| {
                let e1 = sub(v[(i + n - 1) % n], v[i]);
                let e2 = sub(v[(i + 1) % n], v[i]);
                let a = cross(e1, e2).abs().atan2(e1.x * e2.x + e1.y * e2.y);
                let k = (a / std::f64::consts::FRAC_PI_6).round();
                assert!(
                    (a - k * std::f64::consts::FRAC_PI_6).abs() < 1e-9,
                    "corner {a} at {:?} is not a multiple of 30°",
                    v[i]
                );
                (k * 30.0) as u32
            })
            .collect()
    }

    /// The 12 vertices of the substrate dodecagon centred at `c`: the vertex
    /// `(1 + √3/2, 1/2)` turned through 30°·k — all exact Q(√3) points.
    fn dodecagon_vertices(c: Vec2) -> Vec<Vec2> {
        let (co, si) = (RT3 / 2.0, 0.5);
        let mut p = v2(RAP, 0.5);
        let mut out = Vec::with_capacity(12);
        for _ in 0..12 {
            out.push(c + p);
            p = v2(co * p.x - si * p.y, si * p.x + co * p.y);
        }
        out
    }

    /// The 3.12.12 vertices (one triangle + two dodecagons each) around the
    /// origin: the vertices of the dodecagons on the nearby lattice points.
    fn substrate_vertices() -> Vec<Vec2> {
        let mut out: Vec<Vec2> = Vec::new();
        for m in -2..=2i32 {
            for n in -2..=2i32 {
                let c = v2(m as f64 * A1X, m as f64 * A1Y + n as f64 * A2Y);
                for p in dodecagon_vertices(c) {
                    if !out.iter().any(|q| norm(sub(*q, p)) < 1e-9) {
                        out.push(p);
                    }
                }
            }
        }
        out
    }

    /// Strict containment in a convex face (every face of this partition is
    /// convex: the corner census stops at 150°).
    fn strictly_inside(p: Vec2, v: &[Vec2]) -> bool {
        let n = v.len();
        let s: Vec<f64> = (0..n)
            .map(|i| cross(sub(v[(i + 1) % n], v[i]), sub(p, v[i])))
            .collect();
        s.iter().all(|c| *c > 1e-9) || s.iter().all(|c| *c < -1e-9)
    }

    /// Positive symmetry test: the image of the face `pts` under `f` must
    /// coincide, vertex for vertex, with some orbit face plus an EXACT
    /// lattice translation `m·a1 + n·a2`. Table vertices live on
    /// `(1/24)·Z[√3]`, so a lattice coordinate of a difference is
    /// `((12Q−6P) + (4P−6Q)√3)/72`; with the table's numerators bounded by
    /// 33 and 18, a non-lattice offset is at least ≈ 8e-6 away — three orders
    /// of magnitude above the 1e-9 tolerance, which therefore cannot blur a
    /// genuine near-miss into a hit.
    fn image_is_a_face(t: &LatticeTiling, pts: &[Vec2], f: &dyn Fn(Vec2) -> Vec2) -> bool {
        let img: Vec<Vec2> = pts.iter().map(|p| f(*p)).collect();
        for cell in &t.cells {
            let v = &cell.poly.pts;
            if v.len() != img.len() {
                continue;
            }
            for i in 0..v.len() {
                let d = sub(img[0], v[i]);
                // d = m·(3/2+√3, 1+√3/2) + n·(0, 2+√3)
                let m = d.x / A1X;
                let n = (d.y - m * A1Y) / A2Y;
                if (m - m.round()).abs() > 1e-9 || (n - n.round()).abs() > 1e-9 {
                    continue;
                }
                let (mi, ni) = (m.round(), n.round());
                let shift = v2(mi * A1X, mi * A1Y + ni * A2Y);
                // Vertex-SET match (orientation-free: mirrors reverse the
                // winding of the CCW-normalised table).
                let target: Vec<Vec2> = v.iter().map(|p| *p + shift).collect();
                let mut used = vec![false; target.len()];
                let mut ok = true;
                for p in &img {
                    let hit = target.iter().position(|q| norm(sub(*p, *q)) < 1e-9);
                    match hit {
                        Some(j) if !used[j] => used[j] = true,
                        _ => {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    return true;
                }
            }
        }
        false
    }

    fn all_faces(t: &LatticeTiling) -> Vec<Vec<Vec2>> {
        t.cells.iter().map(|c| c.poly.pts.clone()).collect()
    }

    #[test]
    fn pic_3_12_12_covers_region() {
        check_covers(
            &pic_3_12_12(),
            "pic-3-12-12",
            BBox::new(v2(-24.0, -18.0), v2(24.0, 18.0)),
        );
    }

    #[test]
    fn pic_3_12_12_rotated_scaled() {
        check_rotated(
            &pic_3_12_12(),
            "pic-3-12-12(rot20,scale0.6)",
            BBox::new(v2(-24.0, -18.0), v2(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn pic_3_12_12_deterministic() {
        check_det(
            &pic_3_12_12(),
            "pic-3-12-12",
            BBox::new(v2(-12.0, -9.0), v2(12.0, 9.0)),
        );
    }

    #[test]
    fn pic_3_12_12_unit_structure() {
        let t = pic_3_12_12();
        assert_eq!(t.cells.len(), 45);
        let by_seed = |s: u32| t.cells.iter().filter(|c| c.seed == s).count();
        assert_eq!(
            (by_seed(0), by_seed(1), by_seed(2), by_seed(3), by_seed(4), by_seed(5)),
            (1, 6, 12, 2, 12, 12),
            "orbit census: rosette core, 6 vertex pentagons, 12 band quads, \
             2 medial triangles, 12 inner quads, 12 star notches"
        );

        // Exact Q(√3) construction: every coordinate is a (1/24)·Z[√3] surd.
        for c in &t.cells {
            for p in &c.poly.pts {
                assert!(on_twentyfourth_surd_grid(p.x), "vertex x {} off the grid", p.x);
                assert!(on_twentyfourth_surd_grid(p.y), "vertex y {} off the grid", p.y);
            }
        }

        // Σ(cell areas) = |det(lattice)| — exact over Q(√3) (spike):
        // (3/2+3√3/4) + 6·(3/8+3√3/16) + 12·(1/8+√3/12) + 2·(√3/16)
        //   + 12·(1/16+√3/48) + 12·(√3/48) = 6 + 7√3/2.
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        assert!(
            (det - (6.0 + 3.5 * RT3)).abs() < 1e-12,
            "det = 6 + 7√3/2, got {det}"
        );
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-12, "Σ areas {sum} vs det {det}");

        // Seed 0 — one regular dodecagon of side 1/2 (the {12/4} star core)
        // centred on the substrate dodecagon at O.
        let cores: Vec<_> = t.cells.iter().filter(|c| c.seed == 0).collect();
        assert_eq!(cores.len(), 1);
        let v = &cores[0].poly.pts;
        assert_eq!(v.len(), 12);
        for i in 0..12 {
            let side = norm(sub(v[(i + 1) % 12], v[i]));
            assert!((side - 0.5).abs() < 1e-12, "core side {side}");
        }
        assert!(
            (cores[0].poly.area() - (1.5 + 0.75 * RT3)).abs() < 1e-12,
            "core area = 3/2 + 3√3/4"
        );
        let cen = v.iter().fold(Vec2::ZERO, |a, p| a + *p) / 12.0;
        assert!(norm(cen) < 1e-12, "core centred on O, got {cen}");
        assert_eq!(corners(v), vec![150; 12]);

        // Seed 1 — the six vertex pentagons: four long chords of length
        // ℓ = √(1/4+√3/8) plus the short midsegment side of length 1/2, and
        // each wraps exactly one 3.12.12 vertex.
        let subs = substrate_vertices();
        let mut wrapped = 0;
        for c in t.cells.iter().filter(|c| c.seed == 1) {
            let v = &c.poly.pts;
            assert_eq!(v.len(), 5);
            let ell = (0.25 + RT3 / 8.0).sqrt();
            let sides: Vec<f64> = (0..5)
                .map(|i| norm(sub(v[(i + 1) % 5], v[i])))
                .collect();
            let shorts = sides.iter().filter(|s| (**s - 0.5).abs() < 1e-12).count();
            let longs = sides.iter().filter(|s| (**s - ell).abs() < 1e-12).count();
            assert_eq!((shorts, longs), (1, 4), "pentagon sides {sides:?}");
            assert!(
                (c.poly.area() - (0.375 + 0.1875 * RT3)).abs() < 1e-12,
                "pentagon area = 3/8 + 3√3/16"
            );
            assert_eq!(
                {
                    let mut got = corners(v);
                    got.sort_unstable();
                    got
                },
                vec![90, 90, 120, 120, 120]
            );
            wrapped += subs.iter().filter(|p| strictly_inside(**p, v)).count();
        }
        assert_eq!(wrapped, 6, "one 3.12.12 vertex wrapped per pentagon");

        // Seeds 2, 4 — the two quadrilateral orbits (band quads with sides
        // ℓ, ℓ, m, m and inner quads with sides √3/6, √3/6, m, m, where
        // m = √(1/12+√3/24)).
        let m = (1.0 / 12.0 + RT3 / 24.0).sqrt();
        let ell = (0.25 + RT3 / 8.0).sqrt();
        for (seed, area, angles) in [(2u32, 0.125 + RT3 / 12.0, vec![60, 90, 90, 120]),
                                     (4, 1.0 / 16.0 + RT3 / 48.0, vec![60, 60, 90, 150])]
        {
            for c in t.cells.iter().filter(|c| c.seed == seed) {
                let v = &c.poly.pts;
                assert_eq!(v.len(), 4);
                let sides: Vec<f64> = (0..4)
                    .map(|i| norm(sub(v[(i + 1) % 4], v[i])))
                    .collect();
                let two = |s: f64| sides.iter().filter(|x| (**x - s).abs() < 1e-12).count();
                if seed == 2 {
                    assert_eq!((two(ell), two(m)), (2, 2), "band quad sides {sides:?}");
                } else {
                    assert_eq!((two(RT3 / 6.0), two(m)), (2, 2), "inner quad sides {sides:?}");
                }
                let mut got = corners(v);
                got.sort_unstable();
                assert_eq!(got, angles);
                assert!(
                    (c.poly.area() - area).abs() < 1e-12,
                    "seed {seed} quad area"
                );
            }
        }

        // Seed 3 — the midsegment medial triangle of each substrate triangle:
        // equilateral of side 1/2, area √3/16, centred on a substrate
        // triangle centre (1 + 2√3/3, 0) modulo the lattice.
        for c in t.cells.iter().filter(|c| c.seed == 3) {
            let v = &c.poly.pts;
            assert_eq!(v.len(), 3);
            for i in 0..3 {
                let side = norm(sub(v[(i + 1) % 3], v[i]));
                assert!((side - 0.5).abs() < 1e-12, "medial side {side}");
            }
            assert!(
                (c.poly.area() - RT3 / 16.0).abs() < 1e-12,
                "medial area = √3/16"
            );
            let cen = v.iter().fold(Vec2::ZERO, |a, p| a + *p) / 3.0;
            let hit = [-1.0, 1.0].iter().any(|s| {
                let w = v2(s * (1.0 + 2.0 * RT3 / 3.0), 0.0);
                let d = sub(cen, w);
                let m = d.x / A1X;
                let n = (d.y - m * A1Y) / A2Y;
                (m - m.round()).abs() < 1e-9 && (n - n.round()).abs() < 1e-9
            });
            assert!(hit, "medial triangle off a substrate triangle centre: {cen}");
        }

        // Seed 5 — the twelve star notches: isosceles triangles with two
        // chords of √3/6 against a 1/2 base, corners 30°/120°/30°.
        for c in t.cells.iter().filter(|c| c.seed == 5) {
            let v = &c.poly.pts;
            assert_eq!(v.len(), 3);
            let sides: Vec<f64> = (0..3)
                .map(|i| norm(sub(v[(i + 1) % 3], v[i])))
                .collect();
            let two = sides.iter().filter(|s| (**s - RT3 / 6.0).abs() < 1e-12).count();
            let base = sides.iter().filter(|s| (**s - 0.5).abs() < 1e-12).count();
            assert_eq!((two, base), (2, 1), "notch sides {sides:?}");
            assert!(
                (c.poly.area() - RT3 / 48.0).abs() < 1e-12,
                "notch area = √3/48"
            );
            let mut got = corners(v);
            got.sort_unstable();
            assert_eq!(got, vec![30, 30, 120]);
        }

        // No face contains a substrate vertex except the six pentagons: the
        // rosette pieces stay inside their own dodecagon and the medial
        // triangles inside their own triangle.
        for c in t.cells.iter().filter(|c| c.seed != 1) {
            let n = subs.iter().filter(|p| strictly_inside(**p, &c.poly.pts)).count();
            assert_eq!(n, 0, "seed {} face holds a substrate vertex", c.seed);
        }
    }

    #[test]
    fn pic_3_12_12_symmetry_is_p6m() {
        let t = pic_3_12_12();
        let faces = all_faces(&t);

        // Positive probes: each must map every face to a face. Every centre
        // and axis below is an exact Q(√3) point and every map an exact
        // Q(√3) matrix (no trig).
        let o = v2(0.0, 0.0); // substrate dodecagon centre
        let tri = v2(1.0 + 2.0 * RT3 / 3.0, 0.0); // substrate triangle centre
        let m1 = v2(RAP * RT3 / 2.0, RAP / 2.0); // dodecagon-dodecagon midpoint
        let positives: Vec<(&str, Isometry)> = vec![
            ("rot60 @ dodecagon centre", rot(o, 1)),
            ("rot120 @ dodecagon centre", rot(o, 2)),
            ("rot180 @ dodecagon centre", rot(o, 3)),
            ("rot120 @ substrate triangle centre", rot(tri, 2)),
            ("rot180 @ dodecagon-dodecagon midpoint", rot(m1, 3)),
            ("mirror y = 0", refl(o, 1.0, 0.0)),
            ("mirror x = 0", refl(o, 0.0, 1.0)),
            ("mirror at 30° through O", refl(o, RT3 / 2.0, 0.5)),
        ];
        for (label, f) in &positives {
            for (i, face) in faces.iter().enumerate() {
                assert!(
                    image_is_a_face(&t, face, f),
                    "{label} must be a symmetry but fails on face {i}"
                );
            }
        }

        // Negative probes: the symmetry group is EXACTLY p6m — a 30° or 90°
        // turn on the sixfold centre, a sixfold turn on the threefold centre,
        // half turns on the wrong sites and a mirror between two mirror lines
        // must all fail on some face.
        let m0 = v2(RAP, 0.0); // dodecagon-triangle midpoint
        let vert = v2(RAP, 0.5); // a 3.12.12 vertex
        let negatives: Vec<(&str, Isometry)> = vec![
            ("rot30 @ dodecagon centre", rot30(o)),
            ("rot90 @ dodecagon centre", Box::new(|p| v2(-p.y, p.x))),
            ("rot60 @ substrate triangle centre", rot(tri, 1)),
            ("rot180 @ dodecagon-triangle midpoint", rot(m0, 3)),
            ("rot180 @ substrate vertex", rot(vert, 3)),
            ("mirror y = √3/2", refl(v2(0.0, RT3 / 2.0), 1.0, 0.0)),
            ("mirror x = 1", refl(v2(1.0, 0.0), 0.0, 1.0)),
        ];
        for (label, f) in &negatives {
            let fails = faces
                .iter()
                .filter(|face| !image_is_a_face(&t, face, f))
                .count();
            assert!(fails > 0, "{label} must NOT be a symmetry");
        }
    }

    #[test]
    fn pic_3_12_12_symmetry_metadata() {
        let t = pic_3_12_12();
        assert_eq!(t.id, "pic-3-12-12");
        assert_eq!(t.group, WallpaperGroup::P6m);
        assert_eq!(t.lattice_kind, LatticeKind::Hexagonal);
        assert_eq!(t.category, TilingCategory::Islamic);
    }
}

/// M94 Hankin-method engine tests: output equivalence with the validated
/// 40-dps Python prototype (machine-transcribed expectations), engine
/// invariants, and the standard registry checks for the three shipped
/// configurations.
#[cfg(all(test, feature = "hankin_ext"))]
mod hankin_engine_tests {
    use super::hankin;
    use super::hankin_3_12_12;
    use super::hankin_3_3_3_4_4;
    use super::hankin_3_3_4_3_4;
    use super::hankin_3_4_6_4;
    use super::hankin_3_6_3_6;
    use super::hankin_4_6_12;
    use super::hankin_4_8_8;
    use super::hankin_4_8_8_rosette_transform;
    use super::hankin_hex;
    use super::hankin_square_two_point;
    use tess_core::geometry::Vec2;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Rebuild canonical faces from the flat machine-transcribed arrays.
    fn expected_faces(verts: &[f64], lens: &[usize]) -> Vec<Vec<Vec2>> {
        let mut out = Vec::new();
        let mut k = 0;
        for &n in lens {
            let face: Vec<Vec2> = verts[k..k + 2 * n]
                .chunks_exact(2)
                .map(|c| v2(c[0], c[1]))
                .collect();
            k += 2 * n;
            out.push(face);
        }
        out
    }

    fn assert_faces_match(actual: &[Vec<Vec2>], expected: &[Vec<Vec2>], label: &str) {
        assert_eq!(
            actual.len(),
            expected.len(),
            "{label}: face count {} vs expected {}",
            actual.len(),
            expected.len()
        );
        for (a, e) in actual.iter().zip(expected.iter()) {
            assert_eq!(a.len(), e.len(), "{label}: vertex count differs");
            for (p, q) in a.iter().zip(e.iter()) {
                assert!(
                    (p.x - q.x).abs() < 1e-9 && (p.y - q.y).abs() < 1e-9,
                    "{label}: vertex {p} vs expected ({}, {})",
                    q.x,
                    q.y
                );
            }
        }
    }

    #[test]
    fn engine_matches_prototype_4_8_8() {
        let (cells, report) = hankin::build_cells(
            &hankin_4_8_8::tiles(),
            hankin_4_8_8::THETA_E_DEG,
            hankin_4_8_8::DELTA,
        );
        assert!(report.complete, "octagon+diamond pairing must be complete");
        let lens: Vec<usize> = report.per_tile.iter().map(|r| r.faces).collect();
        assert_eq!(lens, vec![9, 5], "face census per tile class");
        for r in &report.per_tile {
            assert!((r.area_sum - r.tile_area).abs() < 1e-9);
            assert_eq!(r.rays, r.pairs * 2, "every ray paired exactly once");
        }
        // Output equivalence against the machine-transcribed prototype.
        let expected: Vec<Vec<Vec2>> = expected_faces(
            hankin_4_8_8::EXPECTED_0_VERTS,
            hankin_4_8_8::EXPECTED_0_LENS,
        )
        .into_iter()
        .chain(expected_faces(
            hankin_4_8_8::EXPECTED_1_VERTS,
            hankin_4_8_8::EXPECTED_1_LENS,
        ))
        .collect();
        let actual: Vec<Vec<Vec2>> = cells.iter().map(|c| c.poly.pts.clone()).collect();
        assert_faces_match(&actual, &expected, "hankin-4-8-8");
    }

    #[test]
    fn engine_matches_prototype_4_6_12() {
        let (cells, report) = hankin::build_cells(
            &hankin_4_6_12::tiles(),
            hankin_4_6_12::THETA_E_DEG,
            hankin_4_6_12::DELTA,
        );
        assert!(report.complete);
        let lens: Vec<usize> = report.per_tile.iter().map(|r| r.faces).collect();
        assert_eq!(lens, vec![13, 5, 5, 5, 7, 7], "face census per tile class");
        for r in &report.per_tile {
            assert!((r.area_sum - r.tile_area).abs() < 1e-9);
            assert_eq!(r.rays, r.pairs * 2);
        }
        let mut expected: Vec<Vec<Vec2>> = Vec::new();
        for k in 0..6 {
            expected.extend(match k {
                0 => expected_faces(hankin_4_6_12::EXPECTED_0_VERTS, hankin_4_6_12::EXPECTED_0_LENS),
                1 => expected_faces(hankin_4_6_12::EXPECTED_1_VERTS, hankin_4_6_12::EXPECTED_1_LENS),
                2 => expected_faces(hankin_4_6_12::EXPECTED_2_VERTS, hankin_4_6_12::EXPECTED_2_LENS),
                3 => expected_faces(hankin_4_6_12::EXPECTED_3_VERTS, hankin_4_6_12::EXPECTED_3_LENS),
                4 => expected_faces(hankin_4_6_12::EXPECTED_4_VERTS, hankin_4_6_12::EXPECTED_4_LENS),
                _ => expected_faces(hankin_4_6_12::EXPECTED_5_VERTS, hankin_4_6_12::EXPECTED_5_LENS),
            });
        }
        let actual: Vec<Vec<Vec2>> = cells.iter().map(|c| c.poly.pts.clone()).collect();
        assert_faces_match(&actual, &expected, "hankin-4-6-12");
    }

    #[test]
    fn engine_matches_prototype_hex_two_point() {
        let (cells, report) = hankin::build_cells(
            &hankin_hex::tiles(),
            hankin_hex::THETA_E_DEG,
            hankin_hex::DELTA,
        );
        assert!(report.complete);
        let r0 = &report.per_tile[0];
        assert_eq!((r0.rays, r0.pairs, r0.faces), (12, 6, 13));
        assert!((r0.area_sum - r0.tile_area).abs() < 1e-9);
        assert_faces_match(
            &cells.iter().map(|c| c.poly.pts.clone()).collect::<Vec<_>>(),
            &expected_faces(hankin_hex::EXPECTED_0_VERTS, hankin_hex::EXPECTED_0_LENS),
            "hankin-hex-two-point",
        );
    }

    #[test]
    fn engine_matches_emission_3_12_12() {
        // M99: no 40-dps Python prototype for this substrate — the
        // expectations were emitted from the Rust engine itself, so
        // this test pins the self-consistency of that emission.
        let (cells, report) = hankin::build_cells(
            &hankin_3_12_12::tiles(),
            hankin_3_12_12::THETA_E_DEG,
            hankin_3_12_12::DELTA,
        );
        assert!(report.complete);
        let lens: Vec<usize> = report.per_tile.iter().map(|r| r.faces).collect();
        assert_eq!(lens, vec![13, 4, 4], "face census per tile class");
        let rays: Vec<usize> = report.per_tile.iter().map(|r| r.rays).collect();
        let pairs: Vec<usize> = report.per_tile.iter().map(|r| r.pairs).collect();
        assert_eq!(rays, vec![24, 6, 6], "rays per tile class");
        assert_eq!(pairs, vec![12, 3, 3], "pairs per tile class");
        for r in &report.per_tile {
            assert!((r.area_sum - r.tile_area).abs() < 1e-9);
            assert_eq!(r.rays, r.pairs * 2);
        }
        let mut expected: Vec<Vec<Vec2>> = Vec::new();
        for k in 0..3 {
            expected.extend(match k {
                0 => expected_faces(hankin_3_12_12::EXPECTED_0_VERTS, hankin_3_12_12::EXPECTED_0_LENS),
                1 => expected_faces(hankin_3_12_12::EXPECTED_1_VERTS, hankin_3_12_12::EXPECTED_1_LENS),
                _ => expected_faces(hankin_3_12_12::EXPECTED_2_VERTS, hankin_3_12_12::EXPECTED_2_LENS),
            });
        }
        let actual: Vec<Vec<Vec2>> = cells.iter().map(|c| c.poly.pts.clone()).collect();
        assert_faces_match(&actual, &expected, "hankin-3-12-12");
    }

    #[test]
    fn engine_matches_emission_square_two_point() {
        // M99: no 40-dps Python prototype for this substrate — the
        // expectations were emitted from the Rust engine itself, so
        // this test pins the self-consistency of that emission.
        let (cells, report) = hankin::build_cells(
            &hankin_square_two_point::tiles(),
            hankin_square_two_point::THETA_E_DEG,
            hankin_square_two_point::DELTA,
        );
        assert!(report.complete);
        let r0 = &report.per_tile[0];
        assert_eq!((r0.rays, r0.pairs, r0.faces), (8, 4, 7));
        assert!((r0.area_sum - r0.tile_area).abs() < 1e-9);
        assert_faces_match(
            &cells.iter().map(|c| c.poly.pts.clone()).collect::<Vec<_>>(),
            &expected_faces(
                hankin_square_two_point::EXPECTED_0_VERTS,
                hankin_square_two_point::EXPECTED_0_LENS,
            ),
            "hankin-square-two-point",
        );
    }

    #[test]
    fn engine_matches_emission_3_6_3_6() {
        // M100: no 40-dps Python prototype for this substrate — the
        // expectations were emitted from the Rust engine itself, so
        // this test pins the self-consistency of that emission. The
        // hexagon tile pairs completely; the two triangle tiles pair
        // with nothing (0 pairs) and their motif stays the original
        // single-face triangle, so the whole-cell report is not
        // "complete" and per-tile completeness is asserted instead.
        let (cells, report) = hankin::build_cells(
            &hankin_3_6_3_6::tiles(),
            hankin_3_6_3_6::THETA_E_DEG,
            hankin_3_6_3_6::DELTA,
        );
        assert!(!report.complete, "triangle tiles are unpaired by design");
        let lens: Vec<usize> = report.per_tile.iter().map(|r| r.faces).collect();
        assert_eq!(lens, vec![7, 1, 1], "face census per tile class");
        let rays: Vec<usize> = report.per_tile.iter().map(|r| r.rays).collect();
        let pairs: Vec<usize> = report.per_tile.iter().map(|r| r.pairs).collect();
        assert_eq!(rays, vec![12, 6, 6], "rays per tile class");
        assert_eq!(pairs, vec![6, 0, 0], "pairs per tile class");
        assert!(report.per_tile[0].complete, "hexagon pairing must be complete");
        assert!(!report.per_tile[1].complete, "triangle tiles stay unpaired");
        assert!(!report.per_tile[2].complete, "triangle tiles stay unpaired");
        for r in &report.per_tile {
            assert!((r.area_sum - r.tile_area).abs() < 1e-9);
        }
        let mut expected: Vec<Vec<Vec2>> = Vec::new();
        for k in 0..3 {
            expected.extend(match k {
                0 => expected_faces(hankin_3_6_3_6::EXPECTED_0_VERTS, hankin_3_6_3_6::EXPECTED_0_LENS),
                1 => expected_faces(hankin_3_6_3_6::EXPECTED_1_VERTS, hankin_3_6_3_6::EXPECTED_1_LENS),
                _ => expected_faces(hankin_3_6_3_6::EXPECTED_2_VERTS, hankin_3_6_3_6::EXPECTED_2_LENS),
            });
        }
        let actual: Vec<Vec<Vec2>> = cells.iter().map(|c| c.poly.pts.clone()).collect();
        assert_faces_match(&actual, &expected, "hankin-3-6-3-6");
    }

    #[test]
    fn engine_matches_emission_3_4_6_4() {
        // M100: no 40-dps Python prototype for this substrate — the
        // expectations were emitted from the Rust engine itself, so
        // this test pins the self-consistency of that emission. The
        // hexagon tile pairs completely; the three square tiles pair
        // with nothing (0 pairs) and their motif stays the original
        // single-face square; the two triangle tiles sit in the
        // 4-subdivision limit (3 pairs, 4 side-1/2 equilateral
        // faces each), so the whole-cell report is not "complete"
        // and per-tile completeness is asserted instead.
        let (cells, report) = hankin::build_cells(
            &hankin_3_4_6_4::tiles(),
            hankin_3_4_6_4::THETA_E_DEG,
            hankin_3_4_6_4::DELTA,
        );
        assert!(!report.complete, "square tiles are unpaired by design");
        let lens: Vec<usize> = report.per_tile.iter().map(|r| r.faces).collect();
        assert_eq!(lens, vec![7, 1, 1, 1, 4, 4], "face census per tile instance");
        let rays: Vec<usize> = report.per_tile.iter().map(|r| r.rays).collect();
        let pairs: Vec<usize> = report.per_tile.iter().map(|r| r.pairs).collect();
        assert_eq!(rays, vec![12, 8, 8, 8, 6, 6], "rays per tile instance");
        assert_eq!(pairs, vec![6, 0, 0, 0, 3, 3], "pairs per tile instance");
        assert!(report.per_tile[0].complete, "hexagon pairing must be complete");
        for i in 1..4 {
            assert!(!report.per_tile[i].complete, "square tiles stay unpaired");
            assert_eq!(report.per_tile[i].faces, 1, "square motif stays a single face");
        }
        assert!(report.per_tile[4].complete, "triangle 4-subdivision is complete");
        assert!(report.per_tile[5].complete, "triangle 4-subdivision is complete");
        for r in &report.per_tile {
            assert!((r.area_sum - r.tile_area).abs() < 1e-9);
        }
        let mut expected: Vec<Vec<Vec2>> = Vec::new();
        for k in 0..6 {
            expected.extend(match k {
                0 => expected_faces(hankin_3_4_6_4::EXPECTED_0_VERTS, hankin_3_4_6_4::EXPECTED_0_LENS),
                1 => expected_faces(hankin_3_4_6_4::EXPECTED_1_VERTS, hankin_3_4_6_4::EXPECTED_1_LENS),
                2 => expected_faces(hankin_3_4_6_4::EXPECTED_2_VERTS, hankin_3_4_6_4::EXPECTED_2_LENS),
                3 => expected_faces(hankin_3_4_6_4::EXPECTED_3_VERTS, hankin_3_4_6_4::EXPECTED_3_LENS),
                4 => expected_faces(hankin_3_4_6_4::EXPECTED_4_VERTS, hankin_3_4_6_4::EXPECTED_4_LENS),
                _ => expected_faces(hankin_3_4_6_4::EXPECTED_5_VERTS, hankin_3_4_6_4::EXPECTED_5_LENS),
            });
        }
        let actual: Vec<Vec<Vec2>> = cells.iter().map(|c| c.poly.pts.clone()).collect();
        assert_faces_match(&actual, &expected, "hankin-3-4-6-4");
    }

    #[test]
    fn engine_matches_emission_3_3_3_4_4() {
        // M107: no 40-dps Python prototype for this substrate — the
        // expectations were emitted from the Rust engine itself, so
        // this test pins the self-consistency of that emission. All
        // three tile instances pair completely at θK = 60°: both
        // triangles in the side-1/2 4-subdivision limit (3 pairs, 4
        // equilateral faces each) and the square in its 5-face motif
        // (4 pairs, four corner quadrilaterals + central octagon).
        let (cells, report) = hankin::build_cells(
            &hankin_3_3_3_4_4::tiles(),
            hankin_3_3_3_4_4::THETA_E_DEG,
            hankin_3_3_3_4_4::DELTA,
        );
        assert!(report.complete, "all three tiles must pair completely");
        let lens: Vec<usize> = report.per_tile.iter().map(|r| r.faces).collect();
        assert_eq!(lens, vec![4, 4, 5], "face census per tile instance");
        let rays: Vec<usize> = report.per_tile.iter().map(|r| r.rays).collect();
        let pairs: Vec<usize> = report.per_tile.iter().map(|r| r.pairs).collect();
        assert_eq!(rays, vec![6, 6, 8], "rays per tile instance");
        assert_eq!(pairs, vec![3, 3, 4], "pairs per tile instance");
        for r in &report.per_tile {
            assert!(r.complete, "every tile pairs completely");
            assert!((r.area_sum - r.tile_area).abs() < 1e-9);
        }
        let mut expected: Vec<Vec<Vec2>> = Vec::new();
        for k in 0..3 {
            expected.extend(match k {
                0 => expected_faces(hankin_3_3_3_4_4::EXPECTED_0_VERTS, hankin_3_3_3_4_4::EXPECTED_0_LENS),
                1 => expected_faces(hankin_3_3_3_4_4::EXPECTED_1_VERTS, hankin_3_3_3_4_4::EXPECTED_1_LENS),
                _ => expected_faces(hankin_3_3_3_4_4::EXPECTED_2_VERTS, hankin_3_3_3_4_4::EXPECTED_2_LENS),
            });
        }
        let actual: Vec<Vec<Vec2>> = cells.iter().map(|c| c.poly.pts.clone()).collect();
        assert_faces_match(&actual, &expected, "hankin-3-3-3-4-4");
    }

    #[test]
    fn engine_matches_emission_3_3_4_3_4() {
        // M111: no 40-dps Python prototype for this substrate — the
        // expectations were emitted from a faithful f64 port of the
        // Rust engine (the M111 spike) and cross-audited exactly in
        // the Q(√2, √3) ring, so this test pins the self-consistency
        // of that emission. All twelve tile instances pair completely
        // at θK = 60°: each of the four square tiles in its 5-face
        // motif (4 pairs: four corner quadrilaterals + central
        // octagon) and each of the eight triangle tiles in the
        // side-1/2 4-subdivision limit (3 pairs, four equilateral
        // faces each) — 52 faces per lattice cell.
        let (cells, report) = hankin::build_cells(
            &hankin_3_3_4_3_4::tiles(),
            hankin_3_3_4_3_4::THETA_E_DEG,
            hankin_3_3_4_3_4::DELTA,
        );
        assert!(report.complete, "all twelve tiles must pair completely");
        let lens: Vec<usize> = report.per_tile.iter().map(|r| r.faces).collect();
        assert_eq!(
            lens,
            vec![5, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4],
            "face census per tile instance"
        );
        let rays: Vec<usize> = report.per_tile.iter().map(|r| r.rays).collect();
        let pairs: Vec<usize> = report.per_tile.iter().map(|r| r.pairs).collect();
        assert_eq!(
            rays,
            vec![8, 8, 8, 8, 6, 6, 6, 6, 6, 6, 6, 6],
            "rays per tile instance"
        );
        assert_eq!(
            pairs,
            vec![4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3],
            "pairs per tile instance"
        );
        for r in &report.per_tile {
            assert!(r.complete, "every tile pairs completely");
            assert!((r.area_sum - r.tile_area).abs() < 1e-9);
        }
        let expected = expected_faces(
            hankin_3_3_4_3_4::EXPECTED_N_VERTS,
            hankin_3_3_4_3_4::EXPECTED_LENS,
        );
        let actual: Vec<Vec<Vec2>> = cells.iter().map(|c| c.poly.pts.clone()).collect();
        assert_faces_match(&actual, &expected, "hankin-3-3-4-3-4");
    }

    #[test]
    fn theta_parameters_are_conjugate() {
        // Engine angle θe = 90° − Kaplan contact angle θK, for every
        // shipped configuration (the constants were emitted separately,
        // so this pins their relationship independently).
        assert!((hankin_4_8_8::THETA_E_DEG - (90.0 - hankin_4_8_8::THETA_K_DEG)).abs() < 1e-12);
        assert!((hankin_4_6_12::THETA_E_DEG - (90.0 - hankin_4_6_12::THETA_K_DEG)).abs() < 1e-12);
        assert!((hankin_hex::THETA_E_DEG - (90.0 - hankin_hex::THETA_K_DEG)).abs() < 1e-12);
assert!((hankin_3_12_12::THETA_E_DEG - (90.0 - hankin_3_12_12::THETA_K_DEG)).abs() < 1e-12);
        assert!((hankin_square_two_point::THETA_E_DEG - (90.0 - hankin_square_two_point::THETA_K_DEG)).abs() < 1e-12);
assert!((hankin_4_8_8_rosette_transform::THETA_E_DEG
            - (90.0 - hankin_4_8_8_rosette_transform::THETA_K_DEG))
            .abs()
            < 1e-12);
        assert!((hankin_3_6_3_6::THETA_E_DEG - (90.0 - hankin_3_6_3_6::THETA_K_DEG)).abs() < 1e-12);
        assert!((hankin_3_4_6_4::THETA_E_DEG - (90.0 - hankin_3_4_6_4::THETA_K_DEG)).abs() < 1e-12);
        assert!((hankin_3_3_3_4_4::THETA_E_DEG - (90.0 - hankin_3_3_3_4_4::THETA_K_DEG)).abs() < 1e-12);
        assert!((hankin_3_3_4_3_4::THETA_E_DEG - (90.0 - hankin_3_3_4_3_4::THETA_K_DEG)).abs() < 1e-12);
    }
}

/// M94 Hankin tiling tests: registry-level invariants for the three
/// configurations (cell census, seed orbits, area identity, symmetry
/// metadata, coverage).
#[cfg(all(test, feature = "hankin_ext"))]
mod hankin_4_8_8_tests {
    use super::hankin_4_8_8::hankin_4_8_8_rosette;
    use super::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_4_8_8_covers_region() {
        check_covers(
            &hankin_4_8_8_rosette(),
            "hankin-4-8-8-rosette",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_4_8_8_rotated_scaled() {
        check_rotated(
            &hankin_4_8_8_rosette(),
            "hankin-4-8-8-rosette(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_4_8_8_deterministic() {
        check_det(
            &hankin_4_8_8_rosette(),
            "hankin-4-8-8-rosette",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_4_8_8_unit_structure() {
        let t = hankin_4_8_8_rosette();
        assert_eq!(t.cells.len(), 14);
        // Machine-transcribed seed layout: 4 orbit classes in canonical
        // face order.
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(
            seeds,
            crate::hankin_4_8_8::EXPECTED_SEEDS.to_vec(),
            "seed layout"
        );
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        let a = 1.0 + core::f64::consts::SQRT_2;
        assert!((det - a * a).abs() < 1e-9);
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_4_8_8_symmetry_metadata() {
        let t = hankin_4_8_8_rosette();
        assert_eq!(t.id, "hankin-4-8-8-rosette");
        assert_eq!(t.group, WallpaperGroup::P4m);
        assert_eq!(t.lattice_kind, LatticeKind::Square);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}

#[cfg(all(test, feature = "hankin_ext"))]
mod hankin_4_6_12_tests {
    use super::hankin_4_6_12::hankin_4_6_12;
    use super::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_4_6_12_covers_region() {
        check_covers(
            &hankin_4_6_12(),
            "hankin-4-6-12",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_4_6_12_rotated_scaled() {
        check_rotated(
            &hankin_4_6_12(),
            "hankin-4-6-12(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_4_6_12_deterministic() {
        check_det(
            &hankin_4_6_12(),
            "hankin-4-6-12",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_4_6_12_unit_structure() {
        let t = hankin_4_6_12();
        assert_eq!(t.cells.len(), 42, "13 + 3×5 + 2×7 faces per lattice cell");
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(
            seeds,
            crate::hankin_4_6_12::EXPECTED_SEEDS.to_vec(),
            "seed layout"
        );
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        // √3 as a literal: std's SQRT_3 constant is still unstable.
        const RT3: f64 = 1.732_050_807_568_877_2;
        let s = 3.0 + RT3;
        assert!((det - s * s * RT3 / 2.0).abs() < 1e-9);
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_4_6_12_symmetry_metadata() {
        let t = hankin_4_6_12();
        assert_eq!(t.id, "hankin-4-6-12");
        assert_eq!(t.group, WallpaperGroup::P6m);
        assert_eq!(t.lattice_kind, LatticeKind::Hexagonal);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}

#[cfg(all(test, feature = "hankin_ext"))]
mod hankin_hex_tests {
    use super::hankin_hex::hankin_hex_two_point;
    use super::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_hex_covers_region() {
        check_covers(
            &hankin_hex_two_point(),
            "hankin-hex-two-point",
            BBox::new(Vec2::new(-16.0, -14.0), Vec2::new(16.0, 14.0)),
        );
    }

    #[test]
    fn hankin_hex_rotated_scaled() {
        check_rotated(
            &hankin_hex_two_point(),
            "hankin-hex-two-point(rot20,scale0.6)",
            BBox::new(Vec2::new(-16.0, -14.0), Vec2::new(16.0, 14.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_hex_deterministic() {
        check_det(
            &hankin_hex_two_point(),
            "hankin-hex-two-point",
            BBox::new(Vec2::new(-8.0, -7.0), Vec2::new(8.0, 7.0)),
        );
    }

    #[test]
    fn hankin_hex_unit_structure() {
        let t = hankin_hex_two_point();
        assert_eq!(t.cells.len(), 13, "12 motif faces + 1 centre dodecagon");
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(
            seeds,
            crate::hankin_hex::EXPECTED_SEEDS.to_vec(),
            "seed layout"
        );
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        assert!((det - 3.0f64.sqrt() / 2.0 * 3.0).abs() < 1e-9, "det = |u1×u2| = 3√3/2");
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_hex_symmetry_metadata() {
        let t = hankin_hex_two_point();
        assert_eq!(t.id, "hankin-hex-two-point");
        assert_eq!(t.group, WallpaperGroup::P6m);
        assert_eq!(t.lattice_kind, LatticeKind::Hexagonal);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}

/// M99 Hankin twelvefold khatam tests: registry-level invariants for the
/// 3.12.12 configuration (cell census, seed orbits, area identity,
/// symmetry metadata, coverage).
#[cfg(all(test, feature = "hankin_ext"))]
mod hankin_3_12_12_tests {
    use super::hankin_3_12_12::hankin_3_12_12;
    use super::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_3_12_12_covers_region() {
        check_covers(
            &hankin_3_12_12(),
            "hankin-3-12-12",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_3_12_12_rotated_scaled() {
        check_rotated(
            &hankin_3_12_12(),
            "hankin-3-12-12(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_3_12_12_deterministic() {
        check_det(
            &hankin_3_12_12(),
            "hankin-3-12-12",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_3_12_12_unit_structure() {
        let t = hankin_3_12_12();
        assert_eq!(t.cells.len(), 21, "13 + 4 + 4 faces per lattice cell");
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(
            seeds,
            crate::hankin_3_12_12::EXPECTED_SEEDS.to_vec(),
            "seed layout"
        );
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        // √3 as a literal: std's SQRT_3 constant is still unstable.
        const RT3: f64 = 1.732_050_807_568_877_2;
        assert!((det - (6.0 + 7.0 * RT3 / 2.0)).abs() < 1e-9);
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_3_12_12_symmetry_metadata() {
        let t = hankin_3_12_12();
        assert_eq!(t.id, "hankin-3-12-12");
        assert_eq!(t.group, WallpaperGroup::P6m);
        assert_eq!(t.lattice_kind, LatticeKind::Hexagonal);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}

/// M99 hankin-square-two-point tests: registry-level invariants
/// (cell census, seed orbits, area identity, symmetry metadata,
/// coverage).
#[cfg(all(test, feature = "hankin_ext"))]
mod hankin_square_two_point_tests {
    use super::hankin_square_two_point::hankin_square_two_point;
    use super::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_square_two_point_covers_region() {
        check_covers(
            &hankin_square_two_point(),
            "hankin-square-two-point",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_square_two_point_rotated_scaled() {
        check_rotated(
            &hankin_square_two_point(),
            "hankin-square-two-point(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_square_two_point_deterministic() {
        check_det(
            &hankin_square_two_point(),
            "hankin-square-two-point",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_square_two_point_unit_structure() {
        let t = hankin_square_two_point();
        assert_eq!(t.cells.len(), 7, "4 + 2 + 1 faces per lattice cell");
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(
            seeds,
            crate::hankin_square_two_point::EXPECTED_SEEDS.to_vec(),
            "seed layout"
        );
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        assert!((det - 1.0).abs() < 1e-9, "unit square lattice det");
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_square_two_point_symmetry_metadata() {
        let t = hankin_square_two_point();
        assert_eq!(t.id, "hankin-square-two-point");
        assert_eq!(t.group, WallpaperGroup::Cmm);
        assert_eq!(t.lattice_kind, LatticeKind::CenteredRectangular);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}

/// M100 hankin-3-6-3-6 tests: registry-level invariants (cell census,
/// seed orbits, area identity, symmetry metadata, coverage).
#[cfg(all(test, feature = "hankin_ext"))]
mod hankin_3_6_3_6_tests {
    use super::hankin_3_6_3_6::hankin_3_6_3_6;
    use super::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_3_6_3_6_covers_region() {
        check_covers(
            &hankin_3_6_3_6(),
            "hankin-3-6-3-6",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_3_6_3_6_rotated_scaled() {
        check_rotated(
            &hankin_3_6_3_6(),
            "hankin-3-6-3-6(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_3_6_3_6_deterministic() {
        check_det(
            &hankin_3_6_3_6(),
            "hankin-3-6-3-6",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_3_6_3_6_unit_structure() {
        let t = hankin_3_6_3_6();
        assert_eq!(t.cells.len(), 9, "7 + 1 + 1 faces per lattice cell");
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(
            seeds,
            crate::hankin_3_6_3_6::EXPECTED_SEEDS.to_vec(),
            "seed layout"
        );
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        // √3 as a literal: std's SQRT_3 constant is still unstable.
        const RT3: f64 = 1.732_050_807_568_877_2;
        assert!((det - 2.0 * RT3).abs() < 1e-9, "per-cell area 2√3");
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_3_6_3_6_symmetry_metadata() {
        let t = hankin_3_6_3_6();
        assert_eq!(t.id, "hankin-3-6-3-6");
        assert_eq!(t.group, WallpaperGroup::P6m);
        assert_eq!(t.lattice_kind, LatticeKind::Hexagonal);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}

/// M100 hankin-3-4-6-4 tests: registry-level invariants (cell census,
/// seed orbits, area identity, symmetry metadata, coverage).
#[cfg(all(test, feature = "hankin_ext"))]
mod hankin_3_4_6_4_tests {
    use super::hankin_3_4_6_4::hankin_3_4_6_4;
    use super::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_3_4_6_4_covers_region() {
        check_covers(
            &hankin_3_4_6_4(),
            "hankin-3-4-6-4",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_3_4_6_4_rotated_scaled() {
        check_rotated(
            &hankin_3_4_6_4(),
            "hankin-3-4-6-4(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_3_4_6_4_deterministic() {
        check_det(
            &hankin_3_4_6_4(),
            "hankin-3-4-6-4",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_3_4_6_4_unit_structure() {
        let t = hankin_3_4_6_4();
        assert_eq!(t.cells.len(), 18, "7 + 3 + 8 faces per lattice cell");
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(
            seeds,
            crate::hankin_3_4_6_4::EXPECTED_SEEDS.to_vec(),
            "seed layout"
        );
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        // √3 as a literal: std's SQRT_3 constant is still unstable.
        const RT3: f64 = 1.732_050_807_568_877_2;
        assert!((det - (2.0 * RT3 + 3.0)).abs() < 1e-9, "per-cell area 4√3/2 + 3");
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_3_4_6_4_symmetry_metadata() {
        let t = hankin_3_4_6_4();
        assert_eq!(t.id, "hankin-3-4-6-4");
        assert_eq!(t.group, WallpaperGroup::P6m);
        assert_eq!(t.lattice_kind, LatticeKind::Hexagonal);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}

/// M107 hankin-3-3-3-4-4 tests: registry-level invariants (cell census,
/// seed orbits, area identity, symmetry metadata, coverage).
#[cfg(all(test, feature = "hankin_ext"))]
mod hankin_3_3_3_4_4_tests {
    use super::hankin_3_3_3_4_4::hankin_3_3_3_4_4;
    use super::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_3_3_3_4_4_covers_region() {
        check_covers(
            &hankin_3_3_3_4_4(),
            "hankin-3-3-3-4-4",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_3_3_3_4_4_rotated_scaled() {
        check_rotated(
            &hankin_3_3_3_4_4(),
            "hankin-3-3-3-4-4(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_3_3_3_4_4_deterministic() {
        check_det(
            &hankin_3_3_3_4_4(),
            "hankin-3-3-3-4-4",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_3_3_3_4_4_unit_structure() {
        let t = hankin_3_3_3_4_4();
        assert_eq!(t.cells.len(), 13, "4 + 4 + 5 faces per lattice cell");
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(
            seeds,
            crate::hankin_3_3_3_4_4::EXPECTED_SEEDS.to_vec(),
            "seed layout"
        );
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        // √3 as a literal: std's SQRT_3 constant is still unstable.
        const RT3: f64 = 1.732_050_807_568_877_2;
        assert!((det - (1.0 + 0.5 * RT3)).abs() < 1e-9, "per-cell area 1 + √3/2");
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_3_3_3_4_4_symmetry_metadata() {
        let t = hankin_3_3_3_4_4();
        assert_eq!(t.id, "hankin-3-3-3-4-4");
        assert_eq!(t.group, WallpaperGroup::Cmm);
        assert_eq!(t.lattice_kind, LatticeKind::CenteredRectangular);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}

/// M111 hankin-3-3-4-3-4 tests: registry-level invariants (cell census,
/// seed orbits, area identity, symmetry metadata, coverage).
#[cfg(all(test, feature = "hankin_ext"))]
mod hankin_3_3_4_3_4_tests {
    use super::hankin_3_3_4_3_4::hankin_3_3_4_3_4;
    use super::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_3_3_4_3_4_covers_region() {
        check_covers(
            &hankin_3_3_4_3_4(),
            "hankin-3-3-4-3-4",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_3_3_4_3_4_rotated_scaled() {
        check_rotated(
            &hankin_3_3_4_3_4(),
            "hankin-3-3-4-3-4(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_3_3_4_3_4_deterministic() {
        check_det(
            &hankin_3_3_4_3_4(),
            "hankin-3-3-4-3-4",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_3_3_4_3_4_unit_structure() {
        let t = hankin_3_3_4_3_4();
        assert_eq!(t.cells.len(), 52, "4×5 + 8×4 faces per lattice cell");
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(
            seeds,
            crate::hankin_3_3_4_3_4::EXPECTED_SEEDS.to_vec(),
            "seed layout"
        );
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        // √3 as a literal: std's SQRT_3 constant is still unstable.
        const RT3: f64 = 1.732_050_807_568_877_2;
        assert!((det - (4.0 + 2.0 * RT3)).abs() < 1e-9, "per-cell area (1 + √3)²");
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_3_3_4_3_4_symmetry_metadata() {
        let t = hankin_3_3_4_3_4();
        assert_eq!(t.id, "hankin-3-3-4-3-4");
        assert_eq!(t.group, WallpaperGroup::P4g);
        assert_eq!(t.lattice_kind, LatticeKind::Square);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}

/// M57 layer-mode verification anchors: the generic strapwork engine in
/// `tess_core::strapwork` laid on each canonical registry substrate at
/// the canonical contact angle must reproduce the registered PIC
/// partition's internal edges **exactly** (1e-9 on endpoints, 1e-6 key
/// grid) — layer mode and tiling mode pinned to each other (PROPOSED #14).
#[cfg(all(test, feature = "hankin_ext", feature = "archimedean"))]
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
#[cfg(all(test, feature = "pentagon15"))]
mod pentagon15_tests {
    use super::pentagon15::{s3, side_c, tile_area};
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon15::spec;
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Vertex-set key on a 1e-6 grid (distinct tiling vertices are ≥ 1
    /// apart; two computations of a shared joint agree to ~1e-15).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Verify that the isometry g = (M, t) preserves the tiling: the
    /// image of every tile intersecting `ra` must be a tile intersecting
    /// `rb` (the axis-aligned box of g(ra)).  With `exact = true` the
    /// image set must equal the `rb` tile set (g(ra) = rb as sets, the
    /// case of axis-aligned rectangles).
    fn check_symmetry(
        t: &LatticeTiling,
        params: &tess_core::params::ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            Vec2::new(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            Vec2::new(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ra_tiles = t.generate(params, &Query { region: ra });
        let rb_tiles = t.generate(params, &Query { region: rb });
        assert!(ra_tiles.message.is_none(), "{label}: {:?}", ra_tiles.message);
        assert!(rb_tiles.message.is_none(), "{label}: {:?}", rb_tiles.message);
        assert!(!ra_tiles.tiles.is_empty(), "{label}: no tiles");
        let set_b: std::collections::HashSet<Vec<(i64, i64)>> =
            rb_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let set_a: std::collections::HashSet<Vec<(i64, i64)>> = ra_tiles
            .tiles
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(
            set_a.len(),
            ra_tiles.tiles.len(),
            "{label}: isometry must be injective on the patch"
        );
        if exact {
            assert_eq!(
                set_a, set_b,
                "{label}: the isometry must preserve the tiling"
            );
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    #[test]
    fn pentagon_15_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(
            &t,
            "pentagon-15",
            &p,
            BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn pentagon_15_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-15(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(7.0, 7.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_15_deterministic() {
        check_det(
            &spec(),
            "pentagon-15",
            BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)),
        );
    }

    /// Unit structure of the primitive cell: 12 tiles of the exact
    /// Q(√3) prototile (A = 150°, B = 60°, C = 135°, D = 105°, E = 90°;
    /// a = 2, b = d = e = 1, c = √(2+√3)) on the rectangular lattice
    /// w1 = (−(1+√3)/2, (3+√3)/2) ⊥ w2 = ((9+6√3)/2, 3+3√3/2), with
    /// |det| = 12 × tile area = 15 + 9√3.
    #[test]
    fn pentagon_15_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-15");
        assert_eq!(t.cells.len(), 12, "the primitive unit holds twelve tiles");
        let s3 = s3();
        assert_eq!(
            t.lattice[0],
            Vec2::new(-(1.0 + s3) / 2.0, (3.0 + s3) / 2.0),
            "w1 = (−(1+√3)/2, (3+√3)/2)"
        );
        assert_eq!(
            t.lattice[1],
            Vec2::new((9.0 + 6.0 * s3) / 2.0, (6.0 + 3.0 * s3) / 2.0),
            "w2 = ((9+6√3)/2, 3+3√3/2)"
        );
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        assert!(
            (w1.dot(w2)).abs() < 1e-12,
            "lattice must be rectangular (w1 ⊥ w2)"
        );
        let det = w1.cross(w2);
        assert!(
            (det.abs() - 12.0 * tile_area()).abs() < 1e-12,
            "|det| {det} ≠ 12 × tile area = 15+9√3"
        );
        assert!(
            ((w1.x * w1.x + w1.y * w1.y) - (4.0 + 2.0 * s3)).abs() < 1e-12,
            "|w1|² = 4+2√3"
        );
        assert!(
            ((w2.x * w2.x + w2.y * w2.y) - (63.0 + 36.0 * s3)).abs() < 1e-12,
            "|w2|² = 63+36√3"
        );
        assert_eq!(t.cells[0].poly.pts[0], Vec2::ZERO, "T1's corner B sits at the origin");
        // Seeds follow (k−1) mod 3 — the three full-group isohedral
        // orbits {T1,T4,T7,T10}, {T2,T5,T8,T11}, {T3,T6,T9,T12}.
        for (k, cell) in t.cells.iter().enumerate() {
            assert_eq!(cell.seed, (k as u32) % 3, "cell {k} seed");
        }
        // Shipped prototile fingerprints: from corner B, CCW reads
        // (b,c,d,e,a) = (1, c, 1, 1, 2) with angles (B,C,D,E,A) =
        // (60°,135°,105°,90°,150°); the mirror cells read the reverses.
        let c = side_c();
        let fwd = (
            [1.0, c, 1.0, 1.0, 2.0],
            [
                std::f64::consts::FRAC_PI_3,
                3.0 * std::f64::consts::PI / 4.0,
                7.0 * std::f64::consts::PI / 12.0,
                std::f64::consts::FRAC_PI_2,
                5.0 * std::f64::consts::PI / 6.0,
            ],
        );
        let rev = (
            [1.0, 1.0, c, 1.0, 2.0],
            [
                5.0 * std::f64::consts::PI / 6.0,
                std::f64::consts::FRAC_PI_2,
                7.0 * std::f64::consts::PI / 12.0,
                3.0 * std::f64::consts::PI / 4.0,
                std::f64::consts::FRAC_PI_3,
            ],
        );
        let mut seen: std::collections::HashSet<Vec<(i64, i64)>> =
            std::collections::HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(
                (cell.poly.area() - tile_area()).abs() < 1e-12,
                "cell {k} area ≠ (5+3√3)/4"
            );
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            let n = cell.poly.len();
            assert_eq!(n, 5, "cell {k} vertex count");
            let sides: Vec<f64> = (0..n)
                .map(|i| (cell.poly.pts[(i + 1) % n] - cell.poly.pts[i]).length())
                .collect();
            let angles: Vec<f64> = (0..n)
                .map(|i| {
                    let (p0, p1, p2) = (
                        cell.poly.pts[(i + n - 1) % n],
                        cell.poly.pts[i],
                        cell.poly.pts[(i + 1) % n],
                    );
                    let (v1, v2) = (p0 - p1, p2 - p1);
                    (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
                })
                .collect();
            let match_f = sides.iter().zip(fwd.0).all(|(x, w)| (x - w).abs() < 1e-12)
                && angles.iter().zip(fwd.1).all(|(x, w)| (x - w).abs() < 1e-9);
            let match_r = sides.iter().zip(rev.0).all(|(x, w)| (x - w).abs() < 1e-12)
                && angles.iter().zip(rev.1).all(|(x, w)| (x - w).abs() < 1e-9);
            assert!(
                match_f || match_r,
                "cell {k} is not a congruent type-15 prototile"
            );
            assert!(
                seen.insert(vkey(&cell.poly.pts)),
                "cell {k} duplicates another cell"
            );
        }
    }

    /// 3-isohedral: seeds 0, 1, 2 are the three full-group isohedral
    /// classes (four tiles each in the primitive cell) and every
    /// generated tile keeps the prototile's side multiset
    /// {1, 1, 1, √(2+√3), 2}.
    #[test]
    fn pentagon_15_isohedral_classes() {
        let t = spec();
        let mut counts = [0usize; 3];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(counts, [4, 4, 4], "four tiles per isohedral class");
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region: BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let c = side_c();
        let mut seen = [false; 3];
        for tile in &r.tiles {
            let s = tile.seed as usize;
            assert!(s < 3, "seeds must stay in {{0, 1, 2}}");
            seen[s] = true;
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| {
                    (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length()
                })
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [1.0, 1.0, 1.0, c, 2.0];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!(
                    (got - w).abs() < 1e-12,
                    "tile side {got} ∉ {{1, c, 2}}"
                );
            }
        }
        assert!(seen[0] && seen[1] && seen[2], "all three classes must appear");
    }

    /// The wallpaper group is pgg: the half-turn about the 2-fold
    /// centre c0 = (√3, −1/2) and the two perpendicular glide
    /// reflections (axes ∥ w1 and ∥ w2) preserve the tiling, and no
    /// pure reflection does.  The glide representatives are the
    /// placement isometries of cells T4 and T10:
    /// g1 = (M_b, (−(5+√3)/2, (1−√3)/2)) with g1² = w1,
    /// g2 = (M_a, ((5+5√3)/2, (√3−3)/2)) with g2² = w2.
    #[test]
    fn pentagon_15_pgg_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let s3 = s3();
        // (i) 180° rotation about c0 = (√3, −1/2): g(A) = A as sets.
        let c = Vec2::new(s3, -1.0 / 2.0);
        let rot = |q: Vec2| c * 2.0 - q;
        let ra = BBox::new(v2(-4.0, -4.0), v2(10.0, 10.0));
        let cs: Vec<Vec2> = ra.corners().iter().map(|&q| rot(q)).collect();
        let rb = BBox::new(
            Vec2::new(
                cs.iter().map(|q| q.x).fold(f64::MAX, f64::min),
                cs.iter().map(|q| q.y).fold(f64::MAX, f64::min),
            ),
            Vec2::new(
                cs.iter().map(|q| q.x).fold(f64::MIN, f64::max),
                cs.iter().map(|q| q.y).fold(f64::MIN, f64::max),
            ),
        );
        let ra_tiles = t.generate(&p, &Query { region: ra });
        let rb_tiles = t.generate(&p, &Query { region: rb });
        assert!(ra_tiles.message.is_none(), "{:?}", ra_tiles.message);
        assert!(rb_tiles.message.is_none(), "{:?}", rb_tiles.message);
        let set_b: std::collections::HashSet<Vec<(i64, i64)>> =
            rb_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let set_a: std::collections::HashSet<Vec<(i64, i64)>> = ra_tiles
            .tiles
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|q| rot(*q)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(
            set_a.len(),
            ra_tiles.tiles.len(),
            "rotation must be injective on the patch"
        );
        assert_eq!(
            set_a, set_b,
            "180° rotation about the 2-fold centre c0 must preserve the tiling"
        );
        // (ii) the two perpendicular glides (image window is tilted, so
        // check the image set is contained in the tilted-window patch).
        let ma = (0.5, s3 / 2.0, s3 / 2.0, -0.5);
        let mb = (-0.5, -s3 / 2.0, -s3 / 2.0, 0.5);
        check_symmetry(
            &t,
            &p,
            mb,
            Vec2::new(-(5.0 + s3) / 2.0, (1.0 - s3) / 2.0),
            BBox::new(v2(-4.0, -4.0), v2(10.0, 10.0)),
            false,
            "glide g1 (axis ∥ w1)",
        );
        check_symmetry(
            &t,
            &p,
            ma,
            Vec2::new((5.0 + 5.0 * s3) / 2.0, (s3 - 3.0) / 2.0),
            BBox::new(v2(-4.0, -4.0), v2(10.0, 10.0)),
            false,
            "glide g2 (axis ∥ w2)",
        );
        // (iii) no pure reflection: ker(I+M_a) is the 120° line = the
        // w1 direction, ker(I+M_b) the 30° line = the w2 direction, so a
        // pure reflection (M, t) would need t = i·w1 (resp. i·w2).  No
        // such candidate may map the prototile T1 onto any tile.
        let big = BBox::new(v2(-26.0, -19.0), v2(26.0, 19.0));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        let set: std::collections::HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        for i in -2..=2 {
            // sanity: pure lattice translations are symmetries
            let tr1: Vec<Vec2> = prot.iter().map(|q| *q + w1 * i as f64).collect();
            let tr2: Vec<Vec2> = prot.iter().map(|q| *q + w2 * i as f64).collect();
            assert!(set.contains(&vkey(&tr1)), "T1 + {i}·w1 must be a tile");
            assert!(set.contains(&vkey(&tr2)), "T1 + {i}·w2 must be a tile");
            let img1: Vec<Vec2> =
                prot.iter().map(|q| apply_iso(ma, w1 * i as f64, *q)).collect();
            let img2: Vec<Vec2> =
                prot.iter().map(|q| apply_iso(mb, w2 * i as f64, *q)).collect();
            assert!(
                !set.contains(&vkey(&img1)),
                "(M_a, {i}·w1) must not be a symmetry (no mirrors in pgg)"
            );
            assert!(
                !set.contains(&vkey(&img2)),
                "(M_b, {i}·w2) must not be a symmetry (no mirrors in pgg)"
            );
        }
    }

    /// Window census (default parameters): culling is the tile AABB
    /// intersecting the region (+1e-9); every shipped coordinate lies on
    /// the (Z + Z√3)/2 grid, at least 0.0359 away from any integer
    /// boundary, so the counts are exact.
    #[test]
    fn pentagon_15_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::Pgg);
        assert_eq!(t.lattice_kind, LatticeKind::Rectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the T1 cell (seed 0)");
        let p = ParamSet::default_for(t.params());
        for (region, want) in [
            (BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)), 93),
            (BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)), 89),
            (BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)), 45),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }
}

#[cfg(all(test, feature = "pentagon9"))]
mod pentagon9_tests {
    use super::pentagon9::{s3, tile_area};
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon9::spec;
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Vertex-set key on a 1e-6 grid (distinct tiling vertices are ≥ 1/7
    /// apart — all coordinates lie on the (Z + Z√3)/7 grid — and two
    /// computations of a shared joint agree to ~1e-15).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Single-vertex key on the same 1e-6 grid.
    fn pkey(p: Vec2) -> (i64, i64) {
        ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64)
    }

    /// Verify that the isometry g = (M, t) preserves the tiling: the
    /// image of every tile intersecting `ra` must be a tile intersecting
    /// `rb` (the axis-aligned box of g(ra)).  With `exact = true` the
    /// image set must equal the `rb` tile set (the case g(ra) = rb).
    fn check_symmetry(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            Vec2::new(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            Vec2::new(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ra_tiles = t.generate(params, &Query { region: ra });
        let rb_tiles = t.generate(params, &Query { region: rb });
        assert!(ra_tiles.message.is_none(), "{label}: {:?}", ra_tiles.message);
        assert!(rb_tiles.message.is_none(), "{label}: {:?}", rb_tiles.message);
        assert!(!ra_tiles.tiles.is_empty(), "{label}: no tiles");
        let set_b: std::collections::HashSet<Vec<(i64, i64)>> =
            rb_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let set_a: std::collections::HashSet<Vec<(i64, i64)>> = ra_tiles
            .tiles
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(
            set_a.len(),
            ra_tiles.tiles.len(),
            "{label}: isometry must be injective on the patch"
        );
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    /// Directed-edge key: keys of its two endpoints on the 1e-6 grid.
    type EdgeKey = ((i64, i64), (i64, i64));

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    #[test]
    fn pentagon_9_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(
            &t,
            "pentagon-9",
            &p,
            BBox::new(v2(-25.0, -20.0), v2(25.0, 20.0)),
        );
    }

    #[test]
    fn pentagon_9_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-9(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(7.0, 7.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_9_deterministic() {
        check_det(
            &spec(),
            "pentagon-9",
            BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)),
        );
    }

    /// Unit structure of the primitive cell: 8 tiles of the exact Q(√3)
    /// prototile (A = C = 120°, cos B = 1/7, cos D = 23/98,
    /// cos E = −11/14; a = 6, b = c = d = e = 14) on the rectangular
    /// lattice w1 = (−30, 6√3) ⊥ w2 = (−141/7, −235√3/7), with
    /// |det| = 8 × tile area = 1128·√3, |w1|² = 1008 and
    /// |w2|² = 185556/49.  Every cell is a congruent copy of the
    /// prototile (cyclic (side, angle) fingerprint forward or reversed),
    /// CCW, and no two cells coincide.
    #[test]
    fn pentagon_9_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-9");
        assert_eq!(t.cells.len(), 8, "the primitive unit holds eight tiles");
        let s = s3();
        assert_eq!(t.lattice[0], Vec2::new(-30.0, 6.0 * s), "w1 = (−30, 6√3)");
        assert_eq!(
            t.lattice[1],
            Vec2::new(-141.0 / 7.0, -235.0 / 7.0 * s),
            "w2 = (−141/7, −235√3/7)"
        );
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        assert!((w1.dot(w2)).abs() < 1e-12, "lattice must be rectangular (w1 ⊥ w2)");
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        assert!(
            (det - 8.0 * tile_area()).abs() < 1e-12,
            "|det| {det} ≠ 8 × tile area = 1128√3"
        );
        assert!(
            ((w1.x * w1.x + w1.y * w1.y) - 1008.0).abs() < 1e-12,
            "|w1|² = 900 + 108 = 1008"
        );
        assert!(
            ((w2.x * w2.x + w2.y * w2.y) - 185556.0 / 49.0).abs() < 1e-9,
            "|w2|² = (141² + 3·235²)/49 = 185556/49"
        );
        // Seeds follow the two full-group isohedral orbits
        // {T1,T4,T5,T8} (0) and {T2,T3,T6,T7} (1).
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 1, 1, 0, 0, 1, 1, 0], "isohedral seed pattern");
        // The shipped prototile is T1, corner A at the origin:
        // A = (0,0), B = (14,0), C = (12,8√3), D = (−1,11√3), E = (−3,3√3).
        let proto = &t.cells[0].poly;
        for (i, want) in [
            (0.0, 0.0),
            (14.0, 0.0),
            (12.0, 8.0 * s),
            (-1.0, 11.0 * s),
            (-3.0, 3.0 * s),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(proto.pts[i], v2(want.0, want.1), "prototile corner {i}");
        }
        // Exact corner angle cosines in vertex order A, B, C, D, E.
        let cos = |i: usize| angle_at(proto, i).cos();
        assert!((cos(0) + 0.5).abs() < 1e-12, "cos A = −1/2");
        assert!((cos(1) - 1.0 / 7.0).abs() < 1e-12, "cos B = 1/7");
        assert!((cos(2) + 0.5).abs() < 1e-12, "cos C = −1/2");
        assert!((cos(3) - 23.0 / 98.0).abs() < 1e-12, "cos D = 23/98");
        assert!((cos(4) + 11.0 / 14.0).abs() < 1e-12, "cos E = −11/14");
        let ps = [
            side_of(proto, 0),
            side_of(proto, 1),
            side_of(proto, 2),
            side_of(proto, 3),
            side_of(proto, 4),
        ];
        let pa = [
            angle_at(proto, 0),
            angle_at(proto, 1),
            angle_at(proto, 2),
            angle_at(proto, 3),
            angle_at(proto, 4),
        ];
        let mut seen: std::collections::HashSet<Vec<(i64, i64)>> =
            std::collections::HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(
                (cell.poly.area() - tile_area()).abs() < 1e-12,
                "cell {k} area ≠ 141√3"
            );
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            let n = cell.poly.len();
            assert_eq!(n, 5, "cell {k} vertex count");
            // Side multiset {6, 14, 14, 14, 14}.
            let mut sides: Vec<f64> = (0..n).map(|i| side_of(&cell.poly, i)).collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [6.0, 14.0, 14.0, 14.0, 14.0];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "cell {k} side {got} ∉ {{6, 14}}");
            }
            // Cyclic (side, angle) fingerprint against the prototile,
            // forward or reversed (the cells realise both chiralities).
            let cs = (0..n).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..n).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            let rev = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(rot + 9 - i) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(rot + 5 - i) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd || rev, "cell {k} is not a congruent type-9 prototile");
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
        }
    }

    /// Edge-to-edge at the pentagon level over a 61-tile patch: no
    /// directed side is shared by two tiles, every side whose midpoint is
    /// strictly inside the window is matched by exactly one full side of
    /// one neighbour (reversed), and no vertex ever falls in the interior
    /// of a non-incident side (no T-junctions; the closest genuine
    /// near-miss in the patch is 13.6 units away).
    #[test]
    fn pentagon_9_edge_to_edge_gluing() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let big = BBox::new(v2(-60.0, -45.0), v2(60.0, 45.0));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 61, "patch size");
        let mut dirs: std::collections::HashMap<EdgeKey, usize> =
            std::collections::HashMap::new();
        for tile in &r.tiles {
            let n = tile.poly.len();
            for i in 0..n {
                let k = (pkey(tile.poly.pts[i]), pkey(tile.poly.pts[(i + 1) % n]));
                *dirs.entry(k).or_insert(0) += 1;
            }
        }
        assert!(
            dirs.values().all(|&c| c == 1),
            "a directed side may belong to only one tile"
        );
        for tile in &r.tiles {
            let n = tile.poly.len();
            for i in 0..n {
                let (a, b) = (tile.poly.pts[i], tile.poly.pts[(i + 1) % n]);
                let mid = (a + b) * 0.5;
                let d = (mid.x - big.min.x)
                    .min(big.max.x - mid.x)
                    .min(mid.y - big.min.y)
                    .min(big.max.y - mid.y);
                if d > 1e-6 {
                    assert_eq!(
                        dirs.get(&(pkey(b), pkey(a))),
                        Some(&1),
                        "interior side {a} -> {b} lacks its full-edge partner"
                    );
                }
            }
        }
        // No T-junction: no vertex in the interior of a non-incident edge.
        let verts: std::collections::HashSet<(i64, i64)> =
            r.tiles.iter().flat_map(|tile| tile.poly.pts.iter().map(|&p| pkey(p))).collect();
        for tile in &r.tiles {
            let n = tile.poly.len();
            for i in 0..n {
                let (a, b) = (tile.poly.pts[i], tile.poly.pts[(i + 1) % n]);
                let (dx, dy) = (b.x - a.x, b.y - a.y);
                let l2 = dx * dx + dy * dy;
                for vk in verts.iter().filter(|k| **k != pkey(a) && **k != pkey(b)) {
                    let (px, py) = (vk.0 as f64 / 1e6, vk.1 as f64 / 1e6);
                    let tt = (((px - a.x) * dx + (py - a.y) * dy) / l2).clamp(0.0, 1.0);
                    if tt == 0.0 || tt == 1.0 {
                        continue;
                    }
                    let q = a + (b - a) * tt;
                    assert!(
                        (px - q.x).hypot(py - q.y) > 1e-7,
                        "vertex {vk:?} lies in the interior of side {a} -> {b}"
                    );
                }
            }
        }
    }

    /// Vertex census: every window-interior vertex is a complete star of
    /// incident tile angles summing to exactly 360°, and the stars are
    /// exactly the three closed types {A,A,C} = 3·120°, {D,E,E} =
    /// 76.43° + 2·141.79° and {B,B,C,D} = 2·81.79° + 120° + 76.43°, all
    /// three present.
    #[test]
    fn pentagon_9_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let big = BBox::new(v2(-60.0, -45.0), v2(60.0, 45.0));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        let verts: std::collections::HashSet<(i64, i64)> =
            r.tiles.iter().flat_map(|tile| tile.poly.pts.iter().map(|&pt| pkey(pt))).collect();
        // Corner cosines of the prototile, A = C, B, D, E.
        let bins = [-0.5, 1.0 / 7.0, 23.0 / 98.0, -11.0 / 14.0];
        let mut stars: std::collections::BTreeMap<String, usize> =
            std::collections::BTreeMap::new();
        let mut checked = 0;
        for vk in &verts {
            let (px, py) = (vk.0 as f64 / 1e6, vk.1 as f64 / 1e6);
            let d = (px - big.min.x)
                .min(big.max.x - px)
                .min(py - big.min.y)
                .min(big.max.y - py);
            if d < 1e-6 {
                continue; // boundary: incident tiles may be culled
            }
            // Any tile having this corner has its AABB containing the
            // strictly interior point, hence was generated: the star is
            // complete.
            let mut letters = String::new();
            let mut sum = 0.0;
            for tile in &r.tiles {
                let n = tile.poly.len();
                for i in 0..n {
                    if pkey(tile.poly.pts[i]) != *vk {
                        continue;
                    }
                    let ang = angle_at(&tile.poly, i);
                    sum += ang;
                    let c = ang.cos();
                    let j = (0..4)
                        .min_by_key(|&j| ((c - bins[j]).abs() * 1e12) as i64)
                        .unwrap();
                    assert!((c - bins[j]).abs() < 1e-9, "stray corner cosine {c}");
                    letters.push(['A', 'B', 'D', 'E'][j]);
                }
            }
            assert!(
                (sum - std::f64::consts::TAU).abs() < 1e-9,
                "vertex {vk:?} angles sum to {sum} ≠ 2π"
            );
            let mut cs: Vec<char> = letters.chars().collect();
            cs.sort_unstable();
            let star: String = cs.into_iter().collect();
            assert!(
                star == "AAA" || star == "ABBD" || star == "DEE",
                "unexpected vertex star {star} at {vk:?}"
            );
            *stars.entry(star).or_insert(0) += 1;
            checked += 1;
        }
        assert!(checked >= 50, "expected a substantial interior census, got {checked}");
        for star in ["AAA", "ABBD", "DEE"] {
            assert!(stars.contains_key(star), "star type {star} never appears");
        }
    }

    /// 2-isohedral: seeds 0/1 are the two full-group isohedral classes,
    /// four tiles each in the primitive cell, both appear in every patch,
    /// and every generated tile keeps the prototile side multiset
    /// {6, 14, 14, 14, 14}.
    #[test]
    fn pentagon_9_isohedral_classes() {
        let t = spec();
        let mut counts = [0usize; 2];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(counts, [4, 4], "four tiles per isohedral class");
        let params = ParamSet::default_for(t.params());
        let r = t.generate(&params, &Query { region: BBox::new(v2(-60.0, -45.0), v2(60.0, 45.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let mut seen = [false; 2];
        for tile in &r.tiles {
            let s = tile.seed as usize;
            assert!(s < 2, "seeds must stay in {{0, 1}}");
            seen[s] = true;
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length())
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [6.0, 14.0, 14.0, 14.0, 14.0];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "seed {s} side {got} ∉ {{6, 14}}");
            }
        }
        assert!(seen[0] && seen[1], "both isohedral classes must appear");
    }

    /// The wallpaper group is pgg: the half-turn about the 2-fold centre
    /// c0 = (13, 4√3) preserves the tiling exactly, and the two
    /// perpendicular glide reflections
    /// g1 = (M_v, (167/7, −165√3/7)), g1² = w2,
    /// g2 = (M_u, (12, −8√3)),          g2² = −w1,
    /// with M_v the reflection in the w2 direction and M_u = −M_v the
    /// reflection in the w1 direction, preserve it.  Every symmetry with
    /// linear part M_v (resp. M_u) shifts by exactly half a primitive
    /// translation along its axis (the glide g1 (resp. g2) and all its
    /// lattice conjugates), so no pure reflection exists: none of the
    /// candidates (M, i·w) maps the prototile onto a tile.  All
    /// generators are exact Q(√3).
    #[test]
    fn pentagon_9_pgg_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let s = s3();
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let c0 = Vec2::new(13.0, 4.0 * s);
        // A window symmetric about c0: the half-turn must give exact
        // set equality of the tile patch.
        let ra = BBox::new(
            v2(c0.x - 30.0, c0.y - 24.0),
            v2(c0.x + 30.0, c0.y + 24.0),
        );
        check_symmetry(&t, &p, (-1.0, 0.0, 0.0, -1.0), c0 * 2.0, ra, true, "half-turn about c0");
        // The two perpendicular glides (image window is tilted, so check
        // the image set is contained in the tilted-window patch).
        let mv = (-11.0 / 14.0, 5.0 * s / 14.0, 5.0 * s / 14.0, 11.0 / 14.0);
        let mu = (11.0 / 14.0, -5.0 * s / 14.0, -5.0 * s / 14.0, -11.0 / 14.0);
        let gw1 = Vec2::new(167.0 / 7.0, -165.0 / 7.0 * s);
        let gw2 = Vec2::new(12.0, -8.0 * s);
        // Exact generator identities (floats of exact Q(√3) expressions).
        let (a1, b1, c1, d1) = mv;
        let gs1 = v2((a1 + 1.0) * gw1.x + b1 * gw1.y, c1 * gw1.x + (d1 + 1.0) * gw1.y);
        assert!((gs1 - w2).length() < 1e-9, "g1² must equal w2 exactly");
        let (a2, b2, c2, d2) = mu;
        let gs2 = v2((a2 + 1.0) * gw2.x + b2 * gw2.y, c2 * gw2.x + (d2 + 1.0) * gw2.y);
        assert!((gs2 + w1).length() < 1e-9, "g2² must equal −w1 exactly");
        check_symmetry(&t, &p, mv, gw1, ra, false, "glide g1 (axis ∥ w2)");
        check_symmetry(&t, &p, mu, gw2, ra, false, "glide g2 (axis ∥ w1)");
        // (M_v, w1 − w2)-style shifts: composing the glide with lattice
        // translations keeps it a symmetry — images of the prototile must
        // be tiles.
        let big = BBox::new(v2(-60.0, -45.0), v2(60.0, 45.0));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        let set: std::collections::HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();
        for i in -2..=2 {
            // sanity: pure lattice translations are symmetries
            let tr1: Vec<Vec2> = prot.iter().map(|q| *q + w1 * i as f64).collect();
            assert!(set.contains(&vkey(&tr1)), "T1 + {i}·w1 must be a tile");
        }
        for i in [0.0, -1.0] {
            let img: Vec<Vec2> = prot
                .iter()
                .map(|q| apply_iso(mv, gw1 + w2 * i, *q))
                .collect();
            assert!(
                set.contains(&vkey(&img)),
                "(M_v, g1's shift {i}·w2) must map T1 onto a tile"
            );
        }
        assert!(
            set.contains(&vkey(
                &prot.iter().map(|q| apply_iso(mu, gw2, *q)).collect::<Vec<_>>()
            )),
            "(M_u, w2-shift) must map T1 onto a tile"
        );
        // No pure reflection: every candidate (M, i·w) — zero or whole
        // lattice shifts along either axis — must fail to map T1 onto a
        // tile (all true symmetries with linear part M shift half a
        // primitive translation along the axis).
        for i in -2..=2 {
            for (m, w, mname, wname) in [
                (mv, w1, "M_v", "w1"),
                (mv, w2, "M_v", "w2"),
                (mu, w1, "M_u", "w1"),
                (mu, w2, "M_u", "w2"),
            ] {
                let img: Vec<Vec2> = prot
                    .iter()
                    .map(|q| apply_iso(m, w * i as f64, *q))
                    .collect();
                assert!(
                    !set.contains(&vkey(&img)),
                    "({mname}, {i}·{wname}) must not be a symmetry (no mirrors in pgg)"
                );
            }
        }
    }

    /// Window census (default parameters): culling is the tile AABB
    /// intersecting the region (+1e-9); every shipped coordinate lies on
    /// the (Z + Z√3)/7 grid, at least 1/7 away from any chosen integer
    /// boundary in x and ≥ 4.2e-4 in y, so the counts are exact and
    /// match the independent Python emulation of the lattice.rs culling.
    #[test]
    fn pentagon_9_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::Pgg);
        assert_eq!(t.lattice_kind, LatticeKind::Rectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the T1 cell (seed 0)");
        let p = ParamSet::default_for(t.params());
        for (region, want) in [
            (BBox::new(v2(-60.0, -45.0), v2(60.0, 45.0)), 61),
            (BBox::new(v2(-40.0, -30.0), v2(40.0, 30.0)), 32),
            (BBox::new(v2(-25.0, -20.0), v2(25.0, 20.0)), 16),
            (BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)), 4),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }
}

#[cfg(all(test, feature = "shield"))]
mod shield_tests {
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::shield::ShieldTiling;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn shield_covers_region() {
        let t = ShieldTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(
            &t,
            "shield",
            &p,
            BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
        );
    }

    #[test]
    fn shield_rotated_scaled() {
        check_rotated(
            &ShieldTiling,
            "shield(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn shield_deterministic() {
        check_det(
            &ShieldTiling,
            "shield",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }

    /// Proto census + seed-value sanity on a default-parameter view.
    #[test]
    fn shield_proto_census() {
        let t = ShieldTiling;
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region: BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let mut counts = [0usize; 4];
        for tile in &r.tiles {
            assert!((0..4).contains(&tile.proto), "proto out of range");
            assert_eq!(tile.seed, u32::from(tile.proto), "seed must equal proto");
            counts[tile.proto as usize] += 1;
        }
        // All four prototiles must appear (shields, squares, both
        // triangles): the kinked seed contains 3 squares and 10
        // triangles, and substitution preserves all kinds.
        assert!(counts[0] >= 10, "shields: {counts:?}");
        assert!(counts[1] >= 10, "squares: {counts:?}");
        assert!(counts[2] >= 10, "black triangles: {counts:?}");
        assert!(counts[3] >= 10, "white triangles: {counts:?}");
        assert!(counts.iter().sum::<usize>() >= 100, "tile count");
    }

    use tess_core::tiling::Query;
}
#[cfg(all(test, feature = "pentagon11"))]
mod pentagon11_tests {
    use super::pentagon11::{cell_table, prototile, ratios, two_fold_centre};
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon11::spec;
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Reflection matrix in the direction u (u must be unit): 2uuᵀ − I.
    fn mirror(u: Vec2) -> (f64, f64, f64, f64) {
        (2.0 * u.x * u.x - 1.0, 2.0 * u.x * u.y, 2.0 * u.x * u.y, 2.0 * u.y * u.y - 1.0)
    }

    /// Vertex-set key on a 1e-6 grid (distinct tiling vertices are ≥ 0.09
    /// apart — the shortest side is a = 0.0947 — and two computations of a
    /// shared joint agree to ~1e-15 because every cell is built from the
    /// same runtime prototile array).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    /// True when the tile's AABB lies inside `w`.  A tile whose AABB is
    /// inside `ra` has its polygon inside `ra`, so its image under any
    /// isometry g intersects g(ra) ⊆ bbox(g(ra)) — and a tile whose polygon
    /// intersects the region always lies inside the generator's padded
    /// index range — hence the image is always generated for rb.  Tiles
    /// whose AABB merely touches the boundary (polygon outside) are
    /// excluded: their images legitimately escape rb.
    fn aabb_inside(poly: &Polygon, w: BBox) -> bool {
        let bb = poly.aabb();
        bb.min.x >= w.min.x
            && bb.min.y >= w.min.y
            && bb.max.x <= w.max.x
            && bb.max.y <= w.max.y
    }

    /// Verify that the isometry g = (M, t) preserves the tiling on `ra`:
    /// images of interior-AABB tiles of ra must be tiles generated for the
    /// image window bbox(g(ra)).  With `exact = true` (the half-turn case,
    /// g(ra) = ra) the filtered image set must equal the interior-AABB
    /// tile set of the window.
    fn check_iso(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            v2(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            v2(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ta = t.generate(params, &Query { region: ra });
        let tb = t.generate(params, &Query { region: rb });
        assert!(ta.message.is_none(), "{label}: {:?}", ta.message);
        assert!(tb.message.is_none(), "{label}: {:?}", tb.message);
        let src: Vec<_> = ta.tiles.iter().filter(|tile| aabb_inside(&tile.poly, ra)).collect();
        assert!(!src.is_empty(), "{label}: no interior tiles");
        let set_b: HashSet<Vec<(i64, i64)>> = tb
            .tiles
            .iter()
            .filter(|tile| !exact || aabb_inside(&tile.poly, rb))
            .map(|tile| vkey(&tile.poly.pts))
            .collect();
        let set_a: HashSet<Vec<(i64, i64)>> = src
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(set_a.len(), src.len(), "{label}: isometry must be injective on the patch");
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    #[test]
    fn pentagon_11_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "pentagon-11", &p, BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)));
    }

    #[test]
    fn pentagon_11_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-11(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(7.0, 7.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_11_deterministic() {
        check_det(&spec(), "pentagon-11", BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)));
    }

    /// Unit structure of the primitive cell: 8 congruent copies of the
    /// runtime-computed Kershner prototile (A = 90°, B = 156°, C = 48°,
    /// D = 114°, E = 132°, sides a = r, b = s, c = 1, d = e = 1 + 2r) on
    /// the rectangular lattice w1 ⊥ w2 with det = 8 × tile area.  The
    /// closure identities hold to ~1e-12, every cell is CCW, no two cells
    /// coincide, and the seeds follow the two full-group isohedral orbits.
    #[test]
    fn pentagon_11_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-11");
        assert_eq!(t.cells.len(), 8, "the primitive unit holds eight tiles");
        assert_eq!(t.lattice[0], Vec2::new(-2.1433083997208975, -1.3918807493424277));
        assert_eq!(t.lattice[1], Vec2::new(1.6269879260579216, -2.5053417038859127));
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        assert!(w1.dot(w2).abs() < 1e-12, "lattice must be rectangular (w1 ⊥ w2)");
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        let a0 = t.cells[0].poly.area();
        assert!((det - 8.0 * a0).abs() < 1e-11, "det {det} ≠ 8 × tile area");
        let (r, s, d) = ratios();
        assert!((d - (1.0 + 2.0 * r)).abs() < 1e-15, "d = e = 1 + 2r");
        // The prototile closes: E = (0, a) with edge directions
        // 0°, 24°, 156°, 222° of lengths s, 1, d, d.
        let pr = prototile();
        assert!(pr[4].x.abs() < 1e-12, "closure E.x = 0, got {}", pr[4].x);
        assert!((pr[4].y - r).abs() < 1e-12, "closure E.y = a, got {}", pr[4].y);
        let proto = &t.cells[0].poly;
        assert_eq!(proto.pts[0], v2(0.0, 0.0), "prototile corner A at the origin");
        for (i, want) in [(0usize, 90.0f64), (1, 156.0), (2, 48.0), (3, 114.0), (4, 132.0)] {
            let got = angle_at(proto, i).to_degrees();
            assert!((got - want).abs() < 1e-9, "angle at vertex {i}: {got} ≠ {want}");
        }
        for (i, want) in [(0usize, s), (1, 1.0), (2, d), (3, d), (4, r)] {
            assert!((side_of(proto, i) - want).abs() < 1e-12, "side {i}");
        }
        // Seeds follow the two full-group isohedral orbits (each orbit
        // mixes both chiralities; the glides identify them).
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 1, 0, 1, 0, 1, 1, 0], "isohedral seed pattern");
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        let mut seen: HashSet<Vec<(i64, i64)>> = HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            assert!((cell.poly.area() - a0).abs() < 1e-12, "cell {k} area");
            assert_eq!(cell.poly.len(), 5, "cell {k} vertex count");
            // Side multiset {r, 1, s, d, d}.
            let mut sides: Vec<f64> = (0..5).map(|i| side_of(&cell.poly, i)).collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [r, 1.0, s, d, d];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "cell {k} side {got} ∉ {{r, 1, s, d}}");
            }
            // Cyclic (side, angle) fingerprint against the prototile,
            // forward or reversed (the cells realise both chiralities).
            let cs = (0..5).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..5).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            let rev = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(rot + 9 - i) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(rot + 5 - i) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd || rev, "cell {k} is not a congruent type-11 prototile");
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
        }
    }

    /// Window census (default parameters) and declared metadata: culling
    /// is the tile AABB intersecting the region (+1e-9), matching the
    /// independent Python emulation of the lattice.rs culling.
    #[test]
    fn pentagon_11_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::Pgg);
        assert_eq!(t.lattice_kind, LatticeKind::Rectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the cell-0 tile (seed 0)");
        let p = ParamSet::default_for(t.params());
        for (region, want) in [
            (BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)), 238),
            (BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)), 213),
            (BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)), 116),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }

    /// Refined edge census over 528 tiles (the tiling is NON-edge-to-edge
    /// at the pentagon level): of the sides whose midpoint lies strictly
    /// inside the window, 1882 are matched by exactly one full side and
    /// 210 are d-sides split at exactly the 2a + c parameters c/d and
    /// (c + a)/d; every resulting sub-edge is contained in exactly one
    /// full partner edge (c ↔ [0, c] of a d-side, each a-side inside its
    /// partner d-side, b ↔ b, unsplit d ↔ d).
    #[test]
    fn pentagon_11_refined_edge_census() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let region = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 528, "patch size");
        let (ra, _sb, da) = ratios();
        let verts: Vec<Vec<Vec2>> = r.tiles.iter().map(|tile| tile.poly.pts.clone()).collect();
        let n = verts.len();
        let mut n_split = 0;
        let mut n_full = 0;
        let mut bad: Vec<String> = Vec::new();
        for i in 0..n {
            for e in 0..5 {
                let (pp, qq) = (verts[i][e], verts[i][(e + 1) % 5]);
                let mid = (pp + qq) * 0.5;
                if !(mid.x > -7.5 && mid.x < 13.5 && mid.y > -7.5 && mid.y < 11.5) {
                    continue;
                }
                let dv = qq - pp;
                let l2 = dv.dot(dv);
                let l = l2.sqrt();
                // Split points: other tiles' vertices strictly inside the
                // open segment (collinear within 1e-7), binned at 1e-9 and
                // merged within 1e-8.
                let mut bins: BTreeSet<i64> = BTreeSet::new();
                for (j, vj) in verts.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    for &vv in vj.iter() {
                        let tt = (vv - pp).dot(dv) / l2;
                        if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                            continue;
                        }
                        let cross = (vv.x - pp.x) * dv.y - (vv.y - pp.y) * dv.x;
                        if cross.abs() / l < 1e-7 {
                            bins.insert((tt * 1e9).round() as i64);
                        }
                    }
                }
                let mut srt: Vec<i64> = Vec::new();
                for b in bins {
                    if srt.is_empty() || b - srt[srt.len() - 1] > 10 {
                        srt.push(b);
                    }
                }
                if srt.len() > 2 {
                    bad.push(format!("tile {i} edge {e}: {srt:?}"));
                    continue;
                }
                let mut ends: Vec<f64> = vec![0.0];
                ends.extend(srt.iter().map(|b| *b as f64 / 1e9));
                ends.push(1.0);
                for w in ends.windows(2) {
                    let aq = pp + dv * w[0];
                    let seg = dv * (w[1] - w[0]);
                    let lseg = seg.length();
                    let uh = seg * (1.0 / lseg);
                    let mut cnt = 0;
                    for (j, vj) in verts.iter().enumerate() {
                        if j == i {
                            continue;
                        }
                        for (a1, b1) in (0..5).map(|k| (vj[k], vj[(k + 1) % 5])) {
                            let c1 = (a1.x - aq.x) * uh.y - (a1.y - aq.y) * uh.x;
                            let c2 = (b1.x - aq.x) * uh.y - (b1.y - aq.y) * uh.x;
                            if c1.abs() > 1e-7 || c2.abs() > 1e-7 {
                                continue;
                            }
                            let ta = (a1 - aq).dot(uh);
                            let tb = (b1 - aq).dot(uh);
                            if ta.min(tb) <= 1e-7 * lseg && ta.max(tb) >= lseg * (1.0 - 1e-7) {
                                cnt += 1;
                            }
                        }
                    }
                    if cnt != 1 {
                        bad.push(format!(
                            "tile {i} edge {e} sub [{:.3}, {:.3}] has {cnt} covering edges",
                            w[0], w[1]
                        ));
                    }
                }
                if srt.len() == 2 {
                    n_split += 1;
                    assert!((l - da).abs() < 1e-6, "split side must be a d-side, got {l}");
                    // Split parameters up to edge orientation (mirrored
                    // cells are stored reversed, reading t -> 1 - t):
                    // the joints sit at c and c + a along the d-side.
                    let mut want = [1.0 / da, (1.0 + ra) / da];
                    want.sort_by(|x, y| x.partial_cmp(y).unwrap());
                    let flip = [1.0 - want[1], 1.0 - want[0]];
                    let got: Vec<f64> = srt.iter().map(|b| *b as f64 / 1e9).collect();
                    let matches = |w: &[f64; 2]| {
                        w.iter()
                            .zip(got.iter())
                            .all(|(x, y)| (x - y).abs() < 1e-6)
                    };
                    assert!(
                        matches(&want) || matches(&flip),
                        "split parameters {got:?} ≠ {{c/d, (c+a)/d}} or their mirror"
                    );
                } else {
                    n_full += 1;
                }
            }
        }
        assert!(bad.is_empty(), "refined edge census violations: {bad:?}");
        assert_eq!(n_full, 1882, "full (unsplit) edges");
        assert_eq!(n_split, 210, "d-sides split at the 2a + c joints");
    }

    /// Vertex-figure census over the window (−4, −4)–(8, 8) with margin
    /// 2.6: every interior vertex is a complete star — corner rays plus
    /// rays of edges passing through it (the T-junction points) — merging
    /// within 1e-4 rad, and its sectors are exactly one of the four
    /// figures {C,B,B} (24), {A,A,180°} (25), {D,D,E} (25) and
    /// {C,E,180°} (26), each summing to 360°; 100 vertices in total.
    #[test]
    fn pentagon_11_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let big = BBox::new(v2(-4.0, -4.0), v2(8.0, 8.0));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        const MARG: f64 = 2.6;
        let mut pts: Vec<Vec2> = Vec::new();
        for tile in &r.tiles {
            for q in &tile.poly.pts {
                if q.x > big.min.x + MARG
                    && q.x < big.max.x - MARG
                    && q.y > big.min.y + MARG
                    && q.y < big.max.y - MARG
                    && !pts.iter().any(|o| (*o - *q).length() < 1e-5)
                {
                    pts.push(*q);
                }
            }
        }
        let figs: [(u32, u32, u32); 4] =
            [(48, 132, 180), (48, 156, 156), (90, 90, 180), (114, 114, 132)];
        let mut counts: BTreeMap<(u32, u32, u32), usize> = BTreeMap::new();
        for pt in &pts {
            let mut rays: BTreeSet<i64> = BTreeSet::new();
            for tile in &r.tiles {
                let nn = tile.poly.len();
                for k in 0..nn {
                    if (tile.poly.pts[k] - *pt).length() >= 1e-5 {
                        continue;
                    }
                    for &q in [&tile.poly.pts[(k + nn - 1) % nn], &tile.poly.pts[(k + 1) % nn]] {
                        let dir = q - *pt;
                        let th = dir.y.atan2(dir.x).rem_euclid(std::f64::consts::TAU);
                        rays.insert((th * 1e5).round() as i64);
                    }
                }
                for e in 0..nn {
                    let (a, b) = (tile.poly.pts[e], tile.poly.pts[(e + 1) % nn]);
                    let ab = b - a;
                    let l2 = ab.dot(ab);
                    let tt = (*pt - a).dot(ab) / l2;
                    if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                        continue;
                    }
                    if (*pt - a).cross(ab).abs() / l2.sqrt() < 1e-7 {
                        let th0 = ab.y.atan2(ab.x);
                        for add in [0.0, std::f64::consts::PI] {
                            let th = (th0 + add).rem_euclid(std::f64::consts::TAU);
                            rays.insert((th * 1e5).round() as i64);
                        }
                    }
                }
            }
            let mut mr: Vec<f64> = Vec::new();
            for k in &rays {
                let rv = *k as f64 / 1e5;
                if mr.is_empty() || rv - mr[mr.len() - 1] > 1e-4 {
                    mr.push(rv);
                }
            }
            if mr.len() > 1 && mr[mr.len() - 1] - mr[0] > std::f64::consts::TAU - 1e-4 {
                mr.pop();
            }
            let mut gaps: Vec<f64> = (0..mr.len())
                .map(|i| (mr[(i + 1) % mr.len()] - mr[i]).rem_euclid(std::f64::consts::TAU))
                .collect();
            gaps.retain(|g| *g > 1e-4);
            gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let fig: Vec<u32> = gaps.iter().map(|g| g.to_degrees().round() as u32).collect();
            assert_eq!(fig.len(), 3, "vertex {pt:?}: unexpected figure {fig:?}");
            assert_eq!(fig.iter().sum::<u32>(), 360, "vertex {pt:?} figure {fig:?}");
            let f3 = (fig[0], fig[1], fig[2]);
            assert!(figs.contains(&f3), "unexpected vertex figure {f3:?} at {pt:?}");
            *counts.entry(f3).or_insert(0) += 1;
        }
        assert_eq!(pts.len(), 100, "total interior vertices");
        let want: BTreeMap<(u32, u32, u32), usize> = [
            ((48, 132, 180), 26),
            ((48, 156, 156), 24),
            ((90, 90, 180), 25),
            ((114, 114, 132), 25),
        ]
        .into_iter()
        .collect();
        assert_eq!(counts, want, "vertex-figure census");
    }

    /// The wallpaper group is pgg: the half-turn about c0 preserves the
    /// tiling exactly (interior-filtered set equality), the two
    /// perpendicular glides g1 = (M1, t1), g2 = (M2, t2) with axes along
    /// w1 and w2 preserve it (with every lattice-shifted variant, and so
    /// do their inverses), g1² = w1, g2² = −w2, and g2∘g1 is the half-turn
    /// about c0 itself.  Each glide shifts by exactly half a primitive
    /// translation along its axis, so no pure reflection exists: neither
    /// the perpendicular-part candidate (M, t − (t·u)u) nor any whole
    /// lattice shift (M, i·w) maps the prototile onto a tile.
    #[test]
    fn pentagon_11_pgg_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let c0 = two_fold_centre();
        let ra = BBox::new(v2(c0.x - 12.0, c0.y - 10.0), v2(c0.x + 12.0, c0.y + 10.0));
        check_iso(&t, &p, (-1.0, 0.0, 0.0, -1.0), c0 * 2.0, ra, true, "half-turn about c0");
        let u1 = w1 * (1.0 / w1.length());
        let u2 = w2 * (1.0 / w2.length());
        let m1 = mirror(u1);
        let m2 = mirror(u2);
        let t1 = cell_table()[2].1; // the 66° cell pair's anchor
        let t2 = cell_table()[6].1; // the 246° cell pair's anchor
        // Exact generator identities: g² = (I + M)·t.
        let gs1 = v2((m1.0 + 1.0) * t1.x + m1.1 * t1.y, m1.2 * t1.x + (m1.3 + 1.0) * t1.y);
        assert!((gs1 - w1).length() < 1e-9, "g1² must equal w1");
        let gs2 = v2((m2.0 + 1.0) * t2.x + m2.1 * t2.y, m2.2 * t2.x + (m2.3 + 1.0) * t2.y);
        assert!((gs2 + w2).length() < 1e-9, "g2² must equal −w2");
        // Each glide shifts half a primitive translation along its axis —
        // no conjugate of it is a pure reflection.
        for (tt, uu, ww, nm) in [(t1, u1, w1, "g1"), (t2, u2, w2, "g2")] {
            let half = 2.0 * tt.dot(uu) / ww.length();
            assert!((half - half.round()).abs() - 0.5 < 1e-6, "{nm} must shift half a period");
        }
        // g2∘g1 is a half-turn centred at c0 itself.
        let comp = v2(t2.x + m2.0 * t1.x + m2.1 * t1.y, t2.y + m2.2 * t1.x + m2.3 * t1.y);
        assert!((comp * 0.5 - c0).length() < 1e-9, "centre(g2∘g1) must be c0");
        // The glides preserve the tiling (image window is tilted, so check
        // containment of the images among the tiles generated there) — and
        // so do the inverses and the lattice-shifted variants.
        check_iso(&t, &p, m1, t1, ra, false, "glide g1 (axis ∥ w1)");
        check_iso(&t, &p, m1, v2(-(m1.0 * t1.x + m1.1 * t1.y), -(m1.2 * t1.x + m1.3 * t1.y)), ra, false, "glide g1⁻¹");
        check_iso(&t, &p, m2, t2, ra, false, "glide g2 (axis ∥ w2)");
        check_iso(&t, &p, m2, v2(-(m2.0 * t2.x + m2.1 * t2.y), -(m2.2 * t2.x + m2.3 * t2.y)), ra, false, "glide g2⁻¹");
        // Big patch for tile-image membership.
        let big = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        let set: HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();
        for i in -2..=2 {
            // sanity: pure lattice translations are symmetries
            for &w in [&w1, &w2] {
                let img: Vec<Vec2> = prot.iter().map(|q| *q + w * i as f64).collect();
                assert!(set.contains(&vkey(&img)), "cell0 + {i}·w must be a tile");
            }
        }
        for (i, j) in [(0.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (1.0, 0.0)] {
            let sh = w1 * i + w2 * j;
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m1, t1 + sh, *q)).collect();
            assert!(set.contains(&vkey(&img)), "(g1 + {i}·w1 + {j}·w2) must map cell0 onto a tile");
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m2, t2 + sh, *q)).collect();
            assert!(set.contains(&vkey(&img)), "(g2 + {i}·w1 + {j}·w2) must map cell0 onto a tile");
        }
        // No pure reflection: the mirror-class candidate carrying only the
        // perpendicular glide part (a reflection would need zero along-axis
        // shift), and every whole-lattice-shift candidate, must fail to map
        // the prototile onto a tile.
        let p1 = t1 - u1 * t1.dot(u1);
        let p2 = t2 - u2 * t2.dot(u2);
        for (m, tr, nm) in [(m1, p1, "(M1, t1⊥)"), (m2, p2, "(M2, t2⊥)")] {
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, tr, *q)).collect();
            assert!(!set.contains(&vkey(&img)), "{nm} must not be a symmetry (no mirrors in pgg)");
        }
        for i in -2..=2 {
            for (m, w, mn, wn) in
                [(m1, w1, "M1", "w1"), (m1, w2, "M1", "w2"), (m2, w1, "M2", "w1"), (m2, w2, "M2", "w2")]
            {
                let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, w * i as f64, *q)).collect();
                assert!(
                    !set.contains(&vkey(&img)),
                    "({mn}, {i}·{wn}) must not be a symmetry (no mirrors in pgg)"
                );
            }
        }
    }

    /// 2-isohedral: seeds 0/1 are the two full-group isohedral classes —
    /// the half-turn and both glide classes permute the eight cells
    /// mod-lattice by the closed permutations, the group they generate has
    /// exactly two orbits on the cells ({0,2,4,7} / {1,3,5,6}, matching
    /// the seeds), both seeds appear in every patch, and every generated
    /// tile keeps the prototile side multiset {a, c, b, d, d}.
    #[test]
    fn pentagon_11_isohedral_classes() {
        let t = spec();
        let (r, s, d) = ratios();
        let mut counts = [0usize; 2];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(counts, [4, 4], "four tiles per isohedral class");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        let u_of = |p: Vec2| {
            v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
        };
        let mod1 = |x: f64| (x + 0.5).rem_euclid(1.0) - 0.5;
        let table = cell_table();
        // A symmetry (M, t) permutes the cells: the A-corner (unique 90°
        // angle) maps to the A-corner of a cell whose anchor agrees mod
        // lattice and whose rotation matches (M·t rotates the tile by
        // ±θ).  Orientation-reversing maps swap the chiral families.
        let perm_of = |m: (f64, f64, f64, f64), tr: Vec2| -> Vec<usize> {
            let det_m = m.0 * m.3 - m.1 * m.2;
            let rot = m.2.atan2(m.0).to_degrees();
            let mut perm: Vec<usize> = Vec::new();
            for k in 0..8 {
                let (fam_k, a_k, th_k) = table[k];
                let ai = apply_iso(m, tr, a_k);
                let want_fam = if det_m > 0.0 { fam_k } else { !fam_k };
                let th_img = if det_m > 0.0 { rot + th_k } else { rot - th_k };
                let mut hit = None;
                for (j, &(fam_j, a_j, th_j)) in table.iter().enumerate() {
                    if fam_j != want_fam {
                        continue;
                    }
                    let du = u_of(ai - a_j);
                    if mod1(du.x).abs() < 1e-6
                        && mod1(du.y).abs() < 1e-6
                        && ((th_img - th_j + 180.0).rem_euclid(360.0) - 180.0).abs() < 1e-6
                    {
                        hit = Some(j);
                        break;
                    }
                }
                perm.push(hit.unwrap_or_else(|| panic!("cell {k} has no image")));
            }
            perm
        };
        let c0 = two_fold_centre();
        let u1 = w1 * (1.0 / w1.length());
        let u2 = w2 * (1.0 / w2.length());
        let perms = [
            perm_of((-1.0, 0.0, 0.0, -1.0), c0 * 2.0),
            perm_of(mirror(u1), table[2].1),
            perm_of(mirror(u2), table[6].1),
        ];
        assert_eq!(perms[0], vec![4, 5, 7, 6, 0, 1, 3, 2], "half-turn perm");
        assert_eq!(perms[1], vec![2, 3, 0, 1, 7, 6, 5, 4], "glide g1 perm");
        assert_eq!(perms[2], vec![7, 6, 4, 5, 2, 3, 1, 0], "glide g2 perm");
        // Two full-group orbits on the cells, exactly the seed classes.
        let mut seen = [false; 8];
        let mut orbits: Vec<Vec<usize>> = Vec::new();
        for st in 0..8 {
            if seen[st] {
                continue;
            }
            let mut orb = vec![st];
            seen[st] = true;
            let mut head = 0;
            while head < orb.len() {
                let x = orb[head];
                head += 1;
                for pm in &perms {
                    let y = pm[x];
                    if !seen[y] {
                        seen[y] = true;
                        orb.push(y);
                    }
                }
            }
            orb.sort_unstable();
            orbits.push(orb);
        }
        assert_eq!(orbits, vec![vec![0, 2, 4, 7], vec![1, 3, 5, 6]], "two full-group orbits");
        for orb in &orbits {
            let sd = t.cells[orb[0]].seed;
            assert!(orb.iter().all(|&k| t.cells[k].seed == sd), "orbit mixes isohedral seeds");
        }
        assert_ne!(t.cells[orbits[0][0]].seed, t.cells[orbits[1][0]].seed, "orbits differ");
        // Generated patches keep the side multiset and show both classes.
        let params = ParamSet::default_for(t.params());
        let rp = t.generate(&params, &Query { region: BBox::new(v2(-6.0, -6.0), v2(10.0, 10.0)) });
        assert!(rp.message.is_none(), "{:?}", rp.message);
        let mut seen_s = [false; 2];
        for tile in &rp.tiles {
            let sd = tile.seed as usize;
            assert!(sd < 2, "seeds must stay in {{0, 1}}");
            seen_s[sd] = true;
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length())
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [r, 1.0, s, d, d];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "seed {sd} side {got} ∉ {{a, c, b, d}}");
            }
        }
        assert!(seen_s[0] && seen_s[1], "both isohedral classes must appear");
    }
}

#[cfg(all(test, feature = "pentagon12"))]
mod pentagon12_tests {
    use super::pentagon12::{cell_table, prototile, ratios, two_fold_centre};
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon12::spec;
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Reflection matrix in the direction u (u must be unit): 2uuᵀ − I.
    fn mirror(u: Vec2) -> (f64, f64, f64, f64) {
        (2.0 * u.x * u.x - 1.0, 2.0 * u.x * u.y, 2.0 * u.x * u.y, 2.0 * u.y * u.y - 1.0)
    }

    /// Vertex-set key on a 1e-6 grid (shared joints agree to ~1e-15 between
    /// two computations of the same tile, since every cell is built from
    /// the same runtime prototile array).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    /// True when the tile's AABB lies inside `w`.  A tile whose AABB is
    /// inside `ra` has its polygon inside `ra`, so its image under any
    /// isometry g intersects g(ra) ⊆ bbox(g(ra)) — and a tile whose polygon
    /// intersects the region always lies inside the generator's padded
    /// index range — hence the image is always generated for rb.  Tiles
    /// whose AABB merely touches the boundary (polygon outside) are
    /// excluded: their images legitimately escape rb.
    fn aabb_inside(poly: &Polygon, w: BBox) -> bool {
        let bb = poly.aabb();
        bb.min.x >= w.min.x
            && bb.min.y >= w.min.y
            && bb.max.x <= w.max.x
            && bb.max.y <= w.max.y
    }

    /// Verify that the isometry g = (M, t) preserves the tiling on `ra`:
    /// images of interior-AABB tiles of ra must be tiles generated for the
    /// image window bbox(g(ra)).  With `exact = true` (the half-turn case,
    /// g(ra) = ra) the filtered image set must equal the interior-AABB
    /// tile set of the window.
    fn check_iso(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            v2(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            v2(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ta = t.generate(params, &Query { region: ra });
        let tb = t.generate(params, &Query { region: rb });
        assert!(ta.message.is_none(), "{label}: {:?}", ta.message);
        assert!(tb.message.is_none(), "{label}: {:?}", tb.message);
        let src: Vec<_> = ta.tiles.iter().filter(|tile| aabb_inside(&tile.poly, ra)).collect();
        assert!(!src.is_empty(), "{label}: no interior tiles");
        let set_b: HashSet<Vec<(i64, i64)>> = tb
            .tiles
            .iter()
            .filter(|tile| !exact || aabb_inside(&tile.poly, rb))
            .map(|tile| vkey(&tile.poly.pts))
            .collect();
        let set_a: HashSet<Vec<(i64, i64)>> = src
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(set_a.len(), src.len(), "{label}: isometry must be injective on the patch");
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    #[test]
    fn pentagon_12_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "pentagon-12", &p, BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)));
    }

    #[test]
    fn pentagon_12_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-12(rot30,scale0.7)",
            BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_12_deterministic() {
        check_det(&spec(), "pentagon-12", BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)));
    }

    /// Unit structure of the primitive cell: 8 congruent copies of the
    /// runtime-computed Kershner prototile (A = 90°, B = 150°, C = 60°,
    /// D = 120°, E = 120°, sides a = 1, b = √3, c = 1, d = 2, e = 1) on
    /// the rectangular lattice w1 ⊥ w2 with det = 12√3 = 8 × tile area
    /// (tile area 3√3/2).  The closure identities hold to ~1e-12, every
    /// cell is CCW, no two cells coincide, and the seeds follow the two
    /// full-group isohedral orbits.
    #[test]
    fn pentagon_12_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-12");
        assert_eq!(t.cells.len(), 8, "the primitive unit holds eight tiles");
        assert_eq!(t.lattice[0], Vec2::new(-2.598076211353316, -4.5));
        assert_eq!(t.lattice[1], Vec2::new(3.4641016151377544, -2.0));
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        assert!(w1.dot(w2).abs() < 1e-12, "lattice must be rectangular (w1 ⊥ w2)");
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        let a0 = t.cells[0].poly.area();
        assert!((det - 8.0 * a0).abs() < 1e-11, "det {det} ≠ 8 × tile area");
        assert!((det - 12.0 * f64::sqrt(3.0)).abs() < 1e-12, "det {det} ≠ 12√3");
        // The shipped B = 150° member is exact in Q(√3): a = 1, b = √3,
        // c = e = 1, d = 2.
        let (a, b, d) = ratios();
        let e = 2.0 * a - 1.0;
        assert!((a - 1.0).abs() < 1e-12, "a = 1, got {a}");
        assert!((b - f64::sqrt(3.0)).abs() < 1e-12, "b = √3, got {b}");
        assert!((d - 2.0).abs() < 1e-12, "d = 2, got {d}");
        assert!((e - 1.0).abs() < 1e-12, "e = 1, got {e}");
        assert!((d - 2.0 * a).abs() < 1e-12, "d = 2a");
        assert!((d - (1.0 + e)).abs() < 1e-12, "d = c + e");
        // The prototile closes: E = (0, a) with edge directions
        // 0°, 30°, 150°, 210° of lengths b, 1, d, e.
        let pr = prototile();
        assert!(pr[4].x.abs() < 1e-12, "closure E.x = 0, got {}", pr[4].x);
        assert!((pr[4].y - a).abs() < 1e-12, "closure E.y = a, got {}", pr[4].y);
        let proto = &t.cells[0].poly;
        assert_eq!(proto.pts[0], v2(0.0, 0.0), "prototile corner A at the origin");
        for (i, want) in [(0usize, 90.0f64), (1, 150.0), (2, 60.0), (3, 120.0), (4, 120.0)] {
            let got = angle_at(proto, i).to_degrees();
            assert!((got - want).abs() < 1e-9, "angle at vertex {i}: {got} ≠ {want}");
        }
        for (i, want) in [(0usize, b), (1, 1.0), (2, d), (3, 1.0), (4, a)] {
            assert!((side_of(proto, i) - want).abs() < 1e-12, "side {i}");
        }
        // Seeds follow the two full-group isohedral orbits (each orbit
        // mixes both chiralities; the glides identify them).
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 1, 1, 0, 0, 1, 1, 0], "isohedral seed pattern");
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        let mut seen: HashSet<Vec<(i64, i64)>> = HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            assert!((cell.poly.area() - a0).abs() < 1e-12, "cell {k} area");
            assert_eq!(cell.poly.len(), 5, "cell {k} vertex count");
            // Side multiset {1, 1, 1, √3, 2}.
            let mut sides: Vec<f64> = (0..5).map(|i| side_of(&cell.poly, i)).collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [1.0, 1.0, 1.0, b, d];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "cell {k} side {got} ∉ {{1, 1, 1, √3, 2}}");
            }
            // Cyclic (side, angle) fingerprint against the prototile,
            // forward or reversed (the cells realise both chiralities).
            let cs = (0..5).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..5).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            let rev = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(rot + 9 - i) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(rot + 5 - i) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd || rev, "cell {k} is not a congruent type-12 prototile");
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
        }
    }

    /// Window census (default parameters) and declared metadata: culling
    /// is the tile AABB intersecting the region (+1e-9), matching the
    /// independent Python emulation of the lattice.rs culling.
    #[test]
    fn pentagon_12_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::Pgg);
        assert_eq!(t.lattice_kind, LatticeKind::Rectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the cell-0 tile (seed 0)");
        let p = ParamSet::default_for(t.params());
        for (region, want) in [
            (BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)), 98),
            (BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)), 90),
            (BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)), 50),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }

    /// Refined edge census over 215 tiles (the tiling is NON-edge-to-edge
    /// at the pentagon level): of the sides whose midpoint lies strictly
    /// inside the window (−7.45, 13.55) × (−7.6, 11.4), 620 are
    /// matched by exactly one full side (b ↔ b, c ↔ c, e ↔ e)
    /// and 153 are d-sides split at their exact midpoint t = 1/2 into two
    /// unit halves, each contained in exactly one unit partner side (an
    /// a-, c- or e-side); no side carries any other T-junction.  The
    /// window is an exact count with a minimum boundary gap of 0.0888
    /// from any counted midpoint (verified against the exact Q(√3)
    /// tiling), so it is robust against the ~1e-15 float difference
    /// between the runtime-computed prototile and the exact surds.
    #[test]
    fn pentagon_12_refined_edge_census() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let region = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 215, "patch size");
        let (_ra, _bb, da) = ratios();
        let verts: Vec<Vec<Vec2>> = r.tiles.iter().map(|tile| tile.poly.pts.clone()).collect();
        let n = verts.len();
        let mut n_split = 0;
        let mut n_full = 0;
        let mut bad: Vec<String> = Vec::new();
        for i in 0..n {
            for e in 0..5 {
                let (pp, qq) = (verts[i][e], verts[i][(e + 1) % 5]);
                let mid = (pp + qq) * 0.5;
                if !(mid.x > -7.45 && mid.x < 13.55 && mid.y > -7.6 && mid.y < 11.4) {
                    continue;
                }
                let dv = qq - pp;
                let l2 = dv.dot(dv);
                let l = l2.sqrt();
                // Split points: other tiles' vertices strictly inside the
                // open segment (collinear within 1e-7), binned at 1e-9 and
                // merged within 1e-8.
                let mut bins: BTreeSet<i64> = BTreeSet::new();
                for (j, vj) in verts.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    for &vv in vj.iter() {
                        let tt = (vv - pp).dot(dv) / l2;
                        if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                            continue;
                        }
                        let cross = (vv.x - pp.x) * dv.y - (vv.y - pp.y) * dv.x;
                        if cross.abs() / l < 1e-7 {
                            bins.insert((tt * 1e9).round() as i64);
                        }
                    }
                }
                let mut srt: Vec<i64> = Vec::new();
                for b in bins {
                    if srt.is_empty() || b - srt[srt.len() - 1] > 10 {
                        srt.push(b);
                    }
                }
                if srt.len() > 1 {
                    bad.push(format!("tile {i} edge {e}: {srt:?}"));
                    continue;
                }
                let mut ends: Vec<f64> = vec![0.0];
                ends.extend(srt.iter().map(|b| *b as f64 / 1e9));
                ends.push(1.0);
                for w in ends.windows(2) {
                    let aq = pp + dv * w[0];
                    let seg = dv * (w[1] - w[0]);
                    let lseg = seg.length();
                    let uh = seg * (1.0 / lseg);
                    let mut cnt = 0;
                    for (j, vj) in verts.iter().enumerate() {
                        if j == i {
                            continue;
                        }
                        for (a1, b1) in (0..5).map(|k| (vj[k], vj[(k + 1) % 5])) {
                            let c1 = (a1.x - aq.x) * uh.y - (a1.y - aq.y) * uh.x;
                            let c2 = (b1.x - aq.x) * uh.y - (b1.y - aq.y) * uh.x;
                            if c1.abs() > 1e-7 || c2.abs() > 1e-7 {
                                continue;
                            }
                            let ta = (a1 - aq).dot(uh);
                            let tb = (b1 - aq).dot(uh);
                            if ta.min(tb) <= 1e-7 * lseg && ta.max(tb) >= lseg * (1.0 - 1e-7) {
                                cnt += 1;
                            }
                        }
                    }
                    if cnt != 1 {
                        bad.push(format!(
                            "tile {i} edge {e} sub [{:.3}, {:.3}] has {cnt} covering edges",
                            w[0], w[1]
                        ));
                    }
                }
                if srt.len() == 1 {
                    n_split += 1;
                    assert!((l - da).abs() < 1e-6, "split side must be a d-side, got {l}");
                    assert!(
                        (srt[0] as f64 / 1e9 - 0.5).abs() < 1e-6,
                        "d-side must split at its midpoint, got {srt:?}"
                    );
                } else {
                    n_full += 1;
                }
            }
        }
        assert!(bad.is_empty(), "refined edge census violations: {bad:?}");
        assert_eq!(n_full, 620, "full (unsplit) edges");
        assert_eq!(n_split, 153, "d-sides split at their midpoint");
    }

    /// Vertex-figure census over the window (−4, −4)–(8, 8) with margin
    /// 2.6: every interior vertex is a complete star — corner rays plus
    /// rays of edges passing through it (the T-junction points) — merging
    /// within 1e-4 rad, and its sectors are exactly one of the four
    /// figures {90°, 90°, 180°}, {60°, 120°, 180°}, {120°, 120°, 120°}
    /// and {60°, 150°, 150°}, each summing to 360° with exactly three
    /// rays; 35 vertices in total.
    #[test]
    fn pentagon_12_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let big = BBox::new(v2(-4.0, -4.0), v2(8.0, 8.0));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        const MARG: f64 = 2.6;
        let mut pts: Vec<Vec2> = Vec::new();
        for tile in &r.tiles {
            for q in &tile.poly.pts {
                if q.x > big.min.x + MARG
                    && q.x < big.max.x - MARG
                    && q.y > big.min.y + MARG
                    && q.y < big.max.y - MARG
                    && !pts.iter().any(|o| (*o - *q).length() < 1e-5)
                {
                    pts.push(*q);
                }
            }
        }
        let figs: [(u32, u32, u32); 4] =
            [(60, 120, 180), (60, 150, 150), (90, 90, 180), (120, 120, 120)];
        let mut counts: BTreeMap<(u32, u32, u32), usize> = BTreeMap::new();
        for pt in &pts {
            let mut rays: BTreeSet<i64> = BTreeSet::new();
            for tile in &r.tiles {
                let nn = tile.poly.len();
                for k in 0..nn {
                    if (tile.poly.pts[k] - *pt).length() >= 1e-5 {
                        continue;
                    }
                    for &q in [&tile.poly.pts[(k + nn - 1) % nn], &tile.poly.pts[(k + 1) % nn]] {
                        let dir = q - *pt;
                        let th = dir.y.atan2(dir.x).rem_euclid(std::f64::consts::TAU);
                        rays.insert((th * 1e5).round() as i64);
                    }
                }
                for e in 0..nn {
                    let (a, b) = (tile.poly.pts[e], tile.poly.pts[(e + 1) % nn]);
                    let ab = b - a;
                    let l2 = ab.dot(ab);
                    let tt = (*pt - a).dot(ab) / l2;
                    if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                        continue;
                    }
                    if (*pt - a).cross(ab).abs() / l2.sqrt() < 1e-7 {
                        let th0 = ab.y.atan2(ab.x);
                        for add in [0.0, std::f64::consts::PI] {
                            let th = (th0 + add).rem_euclid(std::f64::consts::TAU);
                            rays.insert((th * 1e5).round() as i64);
                        }
                    }
                }
            }
            let mut mr: Vec<f64> = Vec::new();
            for k in &rays {
                let rv = *k as f64 / 1e5;
                if mr.is_empty() || rv - mr[mr.len() - 1] > 1e-4 {
                    mr.push(rv);
                }
            }
            if mr.len() > 1 && mr[mr.len() - 1] - mr[0] > std::f64::consts::TAU - 1e-4 {
                mr.pop();
            }
            let mut gaps: Vec<f64> = (0..mr.len())
                .map(|i| (mr[(i + 1) % mr.len()] - mr[i]).rem_euclid(std::f64::consts::TAU))
                .collect();
            gaps.retain(|g| *g > 1e-4);
            gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let fig: Vec<u32> = gaps.iter().map(|g| g.to_degrees().round() as u32).collect();
            assert_eq!(fig.len(), 3, "vertex {pt:?}: unexpected figure {fig:?}");
            assert_eq!(fig.iter().sum::<u32>(), 360, "vertex {pt:?} figure {fig:?}");
            let f3 = (fig[0], fig[1], fig[2]);
            assert!(figs.contains(&f3), "unexpected vertex figure {f3:?} at {pt:?}");
            *counts.entry(f3).or_insert(0) += 1;
        }
        assert_eq!(pts.len(), 35, "total interior vertices");
        let want: BTreeMap<(u32, u32, u32), usize> = [
            ((60, 120, 180), 9),
            ((60, 150, 150), 10),
            ((90, 90, 180), 8),
            ((120, 120, 120), 8),
        ]
        .into_iter()
        .collect();
        assert_eq!(counts, want, "vertex-figure census");
    }

    /// The wallpaper group is pgg: the half-turn about c0 = (√3/4, 5/4)
    /// preserves the tiling exactly (interior-filtered set equality), the
    /// two perpendicular glides g1 = (M1, t1), g2 = (M2, t2) with axes
    /// along w1 and w2 preserve it (with every lattice-shifted variant,
    /// and so do their inverses), g1² = w1, g2² = −w2, and g2∘g1 is the
    /// half-turn about c0 itself.  Each glide shifts exactly half a
    /// primitive translation along its axis (g1 by +w1/2, g2 by −w2/2),
    /// so no conjugate of it is a pure reflection: neither the
    /// perpendicular-part candidate (M, t − (t·u)u) nor any whole lattice
    /// shift (M, i·w) maps the prototile onto a tile.
    #[test]
    fn pentagon_12_pgg_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let c0 = two_fold_centre();
        let ra = BBox::new(v2(c0.x - 12.0, c0.y - 10.0), v2(c0.x + 12.0, c0.y + 10.0));
        check_iso(&t, &p, (-1.0, 0.0, 0.0, -1.0), c0 * 2.0, ra, true, "half-turn about c0");
        let u1 = w1 * (1.0 / w1.length());
        let u2 = w2 * (1.0 / w2.length());
        let m1 = mirror(u1);
        let m2 = mirror(u2);
        let t1 = cell_table()[3].1; // the 120° mirror pair's anchor: (0, −3)
        let t2 = cell_table()[6].1; // the 300° pair's anchor: (−√3, 1)
        // Exact generator identities: g² = (I + M)·t.
        let gs1 = v2((m1.0 + 1.0) * t1.x + m1.1 * t1.y, m1.2 * t1.x + (m1.3 + 1.0) * t1.y);
        assert!((gs1 - w1).length() < 1e-9, "g1² must equal w1");
        let gs2 = v2((m2.0 + 1.0) * t2.x + m2.1 * t2.y, m2.2 * t2.x + (m2.3 + 1.0) * t2.y);
        assert!((gs2 + w2).length() < 1e-9, "g2² must equal −w2");
        // Each glide shifts exactly half a primitive translation along its
        // axis — g1 by +w1/2, g2 by −w2/2 — so no conjugate of it is a
        // pure reflection.
        let h1 = 2.0 * t1.dot(u1) / w1.length();
        assert!((h1 - 1.0).abs() < 1e-6, "g1 must shift exactly half of w1, got {h1}");
        let h2 = 2.0 * t2.dot(u2) / w2.length();
        assert!((h2 + 1.0).abs() < 1e-6, "g2 must shift exactly half of w2, got {h2}");
        // g2∘g1 is a half-turn centred at c0 itself.
        let comp = v2(t2.x + m2.0 * t1.x + m2.1 * t1.y, t2.y + m2.2 * t1.x + m2.3 * t1.y);
        assert!((comp * 0.5 - c0).length() < 1e-9, "centre(g2∘g1) must be c0");
        // The glides preserve the tiling (image window is tilted, so check
        // containment of the images among the tiles generated there) — and
        // so do the inverses and the lattice-shifted variants.
        check_iso(&t, &p, m1, t1, ra, false, "glide g1 (axis ∥ w1)");
        check_iso(
            &t,
            &p,
            m1,
            v2(-(m1.0 * t1.x + m1.1 * t1.y), -(m1.2 * t1.x + m1.3 * t1.y)),
            ra,
            false,
            "glide g1⁻¹",
        );
        check_iso(&t, &p, m2, t2, ra, false, "glide g2 (axis ∥ w2)");
        check_iso(
            &t,
            &p,
            m2,
            v2(-(m2.0 * t2.x + m2.1 * t2.y), -(m2.2 * t2.x + m2.3 * t2.y)),
            ra,
            false,
            "glide g2⁻¹",
        );
        // Big patch for tile-image membership.
        let big = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        let set: HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();
        for i in -2..=2 {
            // sanity: pure lattice translations are symmetries
            for &w in [&w1, &w2] {
                let img: Vec<Vec2> = prot.iter().map(|q| *q + w * i as f64).collect();
                assert!(set.contains(&vkey(&img)), "cell0 + {i}·w must be a tile");
            }
        }
        for (i, j) in [(0.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (1.0, 0.0)] {
            let sh = w1 * i + w2 * j;
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m1, t1 + sh, *q)).collect();
            assert!(set.contains(&vkey(&img)), "(g1 + {i}·w1 + {j}·w2) must map cell0 onto a tile");
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m2, t2 + sh, *q)).collect();
            assert!(set.contains(&vkey(&img)), "(g2 + {i}·w1 + {j}·w2) must map cell0 onto a tile");
        }
        // No pure reflection: the mirror-class candidate carrying only the
        // perpendicular glide part (a reflection would need zero along-axis
        // shift), and every whole-lattice-shift candidate, must fail to map
        // the prototile onto a tile.
        let p1 = t1 - u1 * t1.dot(u1);
        let p2 = t2 - u2 * t2.dot(u2);
        for (m, tr, nm) in [(m1, p1, "(M1, t1⊥)"), (m2, p2, "(M2, t2⊥)")] {
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, tr, *q)).collect();
            assert!(!set.contains(&vkey(&img)), "{nm} must not be a symmetry (no mirrors in pgg)");
        }
        for i in -2..=2 {
            for (m, w, mn, wn) in
                [(m1, w1, "M1", "w1"), (m1, w2, "M1", "w2"), (m2, w1, "M2", "w1"), (m2, w2, "M2", "w2")]
            {
                let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, w * i as f64, *q)).collect();
                assert!(
                    !set.contains(&vkey(&img)),
                    "({mn}, {i}·{wn}) must not be a symmetry (no mirrors in pgg)"
                );
            }
        }
    }

    /// 2-isohedral: seeds 0/1 are the two full-group isohedral classes —
    /// the half-turn and both glide classes permute the eight cells
    /// mod-lattice by the closed permutations, the group they generate has
    /// exactly two orbits on the cells ({0,3,4,7} / {1,2,5,6}, matching
    /// the seeds), both seeds appear in every patch, and every generated
    /// tile keeps the prototile side multiset {1, 1, 1, √3, 2}.
    #[test]
    fn pentagon_12_isohedral_classes() {
        let t = spec();
        let (_a, b, d) = ratios();
        let mut counts = [0usize; 2];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(counts, [4, 4], "four tiles per isohedral class");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        let u_of = |p: Vec2| {
            v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
        };
        let mod1 = |x: f64| (x + 0.5).rem_euclid(1.0) - 0.5;
        let table = cell_table();
        // A symmetry (M, t) permutes the cells: the A-corner (unique 90°
        // angle) maps to the A-corner of a cell whose anchor agrees mod
        // lattice and whose rotation matches (M·t rotates the tile by
        // ±θ).  Orientation-reversing maps swap the chiral families.
        let perm_of = |m: (f64, f64, f64, f64), tr: Vec2| -> Vec<usize> {
            let det_m = m.0 * m.3 - m.1 * m.2;
            let rot = m.2.atan2(m.0).to_degrees();
            let mut perm: Vec<usize> = Vec::new();
            for k in 0..8 {
                let (fam_k, a_k, th_k) = table[k];
                let ai = apply_iso(m, tr, a_k);
                let want_fam = if det_m > 0.0 { fam_k } else { !fam_k };
                let th_img = if det_m > 0.0 { rot + th_k } else { rot - th_k };
                let mut hit = None;
                for (j, &(fam_j, a_j, th_j)) in table.iter().enumerate() {
                    if fam_j != want_fam {
                        continue;
                    }
                    let du = u_of(ai - a_j);
                    if mod1(du.x).abs() < 1e-6
                        && mod1(du.y).abs() < 1e-6
                        && ((th_img - th_j + 180.0).rem_euclid(360.0) - 180.0).abs() < 1e-6
                    {
                        hit = Some(j);
                        break;
                    }
                }
                perm.push(hit.unwrap_or_else(|| panic!("cell {k} has no image")));
            }
            perm
        };
        let c0 = two_fold_centre();
        let u1 = w1 * (1.0 / w1.length());
        let u2 = w2 * (1.0 / w2.length());
        let perms = [
            perm_of((-1.0, 0.0, 0.0, -1.0), c0 * 2.0),
            perm_of(mirror(u1), table[3].1),
            perm_of(mirror(u2), table[6].1),
        ];
        assert_eq!(perms[0], vec![4, 5, 6, 7, 0, 1, 2, 3], "half-turn perm");
        assert_eq!(perms[1], vec![3, 2, 1, 0, 7, 6, 5, 4], "glide g1 perm");
        assert_eq!(perms[2], vec![7, 6, 5, 4, 3, 2, 1, 0], "glide g2 perm");
        // Two full-group orbits on the cells, exactly the seed classes.
        let mut seen = [false; 8];
        let mut orbits: Vec<Vec<usize>> = Vec::new();
        for st in 0..8 {
            if seen[st] {
                continue;
            }
            let mut orb = vec![st];
            seen[st] = true;
            let mut head = 0;
            while head < orb.len() {
                let x = orb[head];
                head += 1;
                for pm in &perms {
                    let y = pm[x];
                    if !seen[y] {
                        seen[y] = true;
                        orb.push(y);
                    }
                }
            }
            orb.sort_unstable();
            orbits.push(orb);
        }
        assert_eq!(orbits, vec![vec![0, 3, 4, 7], vec![1, 2, 5, 6]], "two full-group orbits");
        for orb in &orbits {
            let sd = t.cells[orb[0]].seed;
            assert!(orb.iter().all(|&k| t.cells[k].seed == sd), "orbit mixes isohedral seeds");
        }
        assert_ne!(t.cells[orbits[0][0]].seed, t.cells[orbits[1][0]].seed, "orbits differ");
        // Generated patches keep the side multiset and show both classes.
        let params = ParamSet::default_for(t.params());
        let rp = t.generate(&params, &Query { region: BBox::new(v2(-6.0, -6.0), v2(10.0, 10.0)) });
        assert!(rp.message.is_none(), "{:?}", rp.message);
        let mut seen_s = [false; 2];
        for tile in &rp.tiles {
            let sd = tile.seed as usize;
            assert!(sd < 2, "seeds must stay in {{0, 1}}");
            seen_s[sd] = true;
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length())
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [1.0, 1.0, 1.0, b, d];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "seed {sd} side {got} ∉ {{1, 1, 1, √3, 2}}");
            }
        }
        assert!(seen_s[0] && seen_s[1], "both isohedral classes must appear");
    }
}

#[cfg(all(test, feature = "pentagon10"))]
mod pentagon10_tests {
    use super::pentagon10::{cell_table, prototile, ratios};
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon10::spec;
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Vertex-set key on a 1e-6 grid (shared joints agree to ~1e-15 between
    /// two computations of the same tile, since every cell is built from
    /// the same runtime prototile array).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    /// True when the tile's AABB lies inside `w`.  A tile whose AABB is
    /// inside `ra` has its polygon inside `ra`, so its image under any
    /// isometry g intersects g(ra) ⊆ bbox(g(ra)) — and a tile whose polygon
    /// intersects the region always lies inside the generator's padded
    /// index range — hence the image is always generated for rb.  Tiles
    /// whose AABB merely touches the boundary (polygon outside) are
    /// excluded: their images legitimately escape rb.
    fn aabb_inside(poly: &Polygon, w: BBox) -> bool {
        let bb = poly.aabb();
        bb.min.x >= w.min.x
            && bb.min.y >= w.min.y
            && bb.max.x <= w.max.x
            && bb.max.y <= w.max.y
    }

    /// Verify that the isometry g = (M, t) preserves the tiling on `ra`:
    /// images of interior-AABB tiles of ra must be tiles generated for the
    /// image window bbox(g(ra)).  With `exact = true` (g(ra) = ra) the
    /// filtered image set must equal the interior-AABB tile set of the
    /// window.
    fn check_iso(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            v2(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            v2(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ta = t.generate(params, &Query { region: ra });
        let tb = t.generate(params, &Query { region: rb });
        assert!(ta.message.is_none(), "{label}: {:?}", ta.message);
        assert!(tb.message.is_none(), "{label}: {:?}", tb.message);
        let src: Vec<_> = ta.tiles.iter().filter(|tile| aabb_inside(&tile.poly, ra)).collect();
        assert!(!src.is_empty(), "{label}: no interior tiles");
        let set_b: HashSet<Vec<(i64, i64)>> = tb
            .tiles
            .iter()
            .filter(|tile| !exact || aabb_inside(&tile.poly, rb))
            .map(|tile| vkey(&tile.poly.pts))
            .collect();
        let set_a: HashSet<Vec<(i64, i64)>> = src
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(set_a.len(), src.len(), "{label}: isometry must be injective on the patch");
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    #[test]
    fn pentagon_10_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "pentagon-10", &p, BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)));
    }

    #[test]
    fn pentagon_10_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-10(rot30,scale0.7)",
            BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_10_deterministic() {
        check_det(&spec(), "pentagon-10", BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)));
    }

    /// Unit structure of the primitive cell: 6 tiles (the prototile at its
    /// four rotations about the origin plus the two mirrored cells at the
    /// shifted anchors (1, 2) and (−1, −2)) on the centered basis
    /// w1 = (3, 3), w2 = (−2, 5) with det = 21 = 6 × tile area (7/2).
    /// The index-2 sublattice Z(3,3) + Z(−7,7) is the primitive rectangle
    /// (sides 3√2 × 7√2) and its centre w2 = (−2, 5) lies in the lattice
    /// — the centered-rectangular structure that separates cmm from pmm.
    /// The shipped B = 90-degree member is exact: a = b = 2, c = e = 1,
    /// d = √2 (d² = c² + e²); every cell is CCW, no two cells coincide,
    /// and the seeds follow the three isohedral orbits.
    #[test]
    fn pentagon_10_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-10");
        assert_eq!(t.cells.len(), 6, "the primitive unit holds six tiles");
        assert_eq!(t.lattice[0], Vec2::new(3.0, 3.0));
        assert_eq!(t.lattice[1], Vec2::new(-2.0, 5.0));
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        // The declared centered basis is oblique (w1·w2 = 9); the
        // primitive rectangle Z(3,3) + Z(−7,7) = Z w1 + Z(−w1 + 2w2) is
        // orthogonal: (3,3)·(−7,7) = 0.
        assert!((w1.dot(w2) - 9.0).abs() < 1e-12, "centered basis w1·w2 = 9, got {}", w1.dot(w2));
        let w2p = -w1 + w2 * 2.0;
        assert_eq!(w2p, Vec2::new(-7.0, 7.0), "−w1 + 2w2 = (−7, 7)");
        assert!(w1.dot(w2p).abs() < 1e-12, "the primitive rectangle is orthogonal");
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        let a0 = t.cells[0].poly.area();
        assert!((a0 - 3.5).abs() < 1e-12, "tile area is 7/2, got {a0}");
        assert!((det - 6.0 * a0).abs() < 1e-11, "det {det} ≠ 6 × tile area");
        assert!((det - 21.0).abs() < 1e-12, "det {det} ≠ 21");
        // The shipped B = 90-degree member is exact: a = b = 2, c = e = 1,
        // d = √2 (d² = c² + e² closes the right angle at corner C).
        let (a, b, d) = ratios();
        let c = 1.0;
        let e = a - c;
        assert!((a - 2.0).abs() < 1e-12, "a = 2, got {a}");
        assert!((b - 2.0).abs() < 1e-12, "b = 2, got {b}");
        assert!((d - f64::sqrt(2.0)).abs() < 1e-12, "d = √2, got {d}");
        assert!((e - 1.0).abs() < 1e-12, "e = 1, got {e}");
        assert!((a - (c + e)).abs() < 1e-12, "a = c + e");
        assert!((b - (c + e)).abs() < 1e-12, "b = c + e");
        assert!((d * d - c * c - e * e).abs() < 1e-12, "d² = c² + e²");
        // The prototile closes: E = (0, a) with edge directions 0°, 90°,
        // 135° of lengths b, c, d (the closing edge E→A has direction
        // 270° and length a).
        let pr = prototile();
        assert!(pr[4].x.abs() < 1e-12, "closure E.x = 0, got {}", pr[4].x);
        assert!((pr[4].y - a).abs() < 1e-12, "closure E.y = a, got {}", pr[4].y);
        let proto = &t.cells[0].poly;
        assert_eq!(proto.pts[0], v2(0.0, 0.0), "prototile corner A at the origin");
        for (i, want) in [(0usize, 90.0f64), (1, 90.0), (2, 135.0), (3, 135.0), (4, 90.0)] {
            let got = angle_at(proto, i).to_degrees();
            assert!((got - want).abs() < 1e-9, "angle at vertex {i}: {got} ≠ {want}");
        }
        for (i, want) in [(0usize, b), (1, c), (2, d), (3, e), (4, a)] {
            assert!((side_of(proto, i) - want).abs() < 1e-12, "side {i}");
        }
        // Seeds follow the three full-group isohedral orbits.
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 1, 0, 1, 2, 2], "isohedral seed pattern");
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        let mut seen: HashSet<Vec<(i64, i64)>> = HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            assert!((cell.poly.area() - a0).abs() < 1e-12, "cell {k} area");
            assert_eq!(cell.poly.len(), 5, "cell {k} vertex count");
            // Side multiset {1, 1, √2, 2, 2}.
            let mut sides: Vec<f64> = (0..5).map(|i| side_of(&cell.poly, i)).collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [1.0, 1.0, f64::sqrt(2.0), 2.0, 2.0];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "cell {k} side {got} ∉ {{1, 1, √2, 2, 2}}");
            }
            // Cyclic (side, angle) fingerprint against the prototile,
            // forward or reversed (the cells realise both chiralities).
            let cs = (0..5).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..5).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            let rev = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(rot + 9 - i) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(rot + 5 - i) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd || rev, "cell {k} is not a congruent type-10 prototile");
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
        }
        // The two M cells are the mirrored chirality at the shifted
        // anchors (1, 2) and (−1, −2); at B = 90° the reflection is the
        // isosceles axis of the prototile, so as vertex sets they coincide
        // with the D cells at the same anchors.
        let table = cell_table();
        assert!(!table[0].0 && !table[1].0 && !table[2].0 && !table[3].0, "cells 0–3 are D cells");
        assert!(table[4].0 && table[5].0, "cells 4 and 5 carry the mirrored chirality");
        assert_eq!(table[4].1, v2(1.0, 2.0), "M90 anchor");
        assert_eq!(table[5].1, v2(-1.0, -2.0), "M270 anchor");
        assert_eq!(table[4].2, 90.0, "M90 rotation");
        assert_eq!(table[5].2, 270.0, "M270 rotation");
    }

    /// Window census (default parameters) and declared metadata: culling
    /// is the tile AABB intersecting the region (+1e-9), matching the
    /// independent Python emulation of the lattice.rs culling.
    #[test]
    fn pentagon_10_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::Cmm);
        assert_eq!(t.lattice_kind, LatticeKind::CenteredRectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the cell-0 tile (seed 0)");
        let p = ParamSet::default_for(t.params());
        for (region, want) in [
            (BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)), 77),
            (BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)), 73),
            (BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)), 41),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }

    /// Refined edge census over 164 tiles (the tiling is NON-edge-to-edge
    /// at the pentagon level): of the sides whose midpoint lies strictly
    /// inside the window (−7.75, 13.75) × (−7.75, 11.75), 518 are matched
    /// by exactly one full side and 76 are length-2 sides split at their
    /// exact midpoint t = 1/2 into two unit halves, each contained in
    /// exactly one unit partner side (a- or b-halves); no side carries any
    /// other T-junction.  Midpoints live in (1/2)Z² and the window bounds
    /// are quarter-integers one unit in, so the minimum boundary gap is
    /// exactly 1/4 — the count is robust against the ~1e-15 float residue
    /// of the runtime-computed prototile.
    #[test]
    fn pentagon_10_refined_edge_census() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let region = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 164, "patch size");
        let (a, _b, _d) = ratios();
        let verts: Vec<Vec<Vec2>> = r.tiles.iter().map(|tile| tile.poly.pts.clone()).collect();
        let n = verts.len();
        let mut n_split = 0;
        let mut n_full = 0;
        let mut bad: Vec<String> = Vec::new();
        for i in 0..n {
            for e in 0..5 {
                let (pp, qq) = (verts[i][e], verts[i][(e + 1) % 5]);
                let mid = (pp + qq) * 0.5;
                if !(mid.x > -7.75 && mid.x < 13.75 && mid.y > -7.75 && mid.y < 11.75) {
                    continue;
                }
                let dv = qq - pp;
                let l2 = dv.dot(dv);
                let l = l2.sqrt();
                // Split points: other tiles' vertices strictly inside the
                // open segment (collinear within 1e-7), binned at 1e-9 and
                // merged within 1e-8.
                let mut bins: BTreeSet<i64> = BTreeSet::new();
                for (j, vj) in verts.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    for &vv in vj.iter() {
                        let tt = (vv - pp).dot(dv) / l2;
                        if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                            continue;
                        }
                        let cross = (vv.x - pp.x) * dv.y - (vv.y - pp.y) * dv.x;
                        if cross.abs() / l < 1e-7 {
                            bins.insert((tt * 1e9).round() as i64);
                        }
                    }
                }
                let mut srt: Vec<i64> = Vec::new();
                for b in bins {
                    if srt.is_empty() || b - srt[srt.len() - 1] > 10 {
                        srt.push(b);
                    }
                }
                if srt.len() > 1 {
                    bad.push(format!("tile {i} edge {e}: {srt:?}"));
                    continue;
                }
                let mut ends: Vec<f64> = vec![0.0];
                ends.extend(srt.iter().map(|b| *b as f64 / 1e9));
                ends.push(1.0);
                for w in ends.windows(2) {
                    let aq = pp + dv * w[0];
                    let seg = dv * (w[1] - w[0]);
                    let lseg = seg.length();
                    let uh = seg * (1.0 / lseg);
                    let mut cnt = 0;
                    for (j, vj) in verts.iter().enumerate() {
                        if j == i {
                            continue;
                        }
                        for (a1, b1) in (0..5).map(|k| (vj[k], vj[(k + 1) % 5])) {
                            let c1 = (a1.x - aq.x) * uh.y - (a1.y - aq.y) * uh.x;
                            let c2 = (b1.x - aq.x) * uh.y - (b1.y - aq.y) * uh.x;
                            if c1.abs() > 1e-7 || c2.abs() > 1e-7 {
                                continue;
                            }
                            let ta = (a1 - aq).dot(uh);
                            let tb = (b1 - aq).dot(uh);
                            if ta.min(tb) <= 1e-7 * lseg && ta.max(tb) >= lseg * (1.0 - 1e-7) {
                                cnt += 1;
                            }
                        }
                    }
                    if cnt != 1 {
                        bad.push(format!(
                            "tile {i} edge {e} sub [{:.3}, {:.3}] has {cnt} covering edges",
                            w[0], w[1]
                        ));
                    }
                }
                if srt.len() == 1 {
                    n_split += 1;
                    assert!((l - a).abs() < 1e-6, "split side must be a length-2 side, got {l}");
                    assert!(
                        (srt[0] as f64 / 1e9 - 0.5).abs() < 1e-6,
                        "length-2 side must split at its midpoint, got {srt:?}"
                    );
                } else {
                    n_full += 1;
                }
            }
        }
        assert!(bad.is_empty(), "refined edge census violations: {bad:?}");
        assert_eq!(n_full, 518, "full (unsplit) edges");
        assert_eq!(n_split, 76, "length-2 sides split at their midpoint");
    }

    /// Vertex-figure census over the window (−8, −8)–(14, 12) with margin
    /// 2.6: every interior vertex is a complete star — corner rays plus
    /// rays of edges passing through it (the T-junction points) — merging
    /// within 1e-4 rad, and its sectors are exactly one of the three
    /// figures {90°, 135°, 135°}, {90°, 90°, 180°} (T-junctions, where a
    /// length-2 side is bisected) and {90°, 90°, 90°, 90°}, each summing
    /// to 360°; 134 vertices in total (71 + 50 + 13).
    #[test]
    fn pentagon_10_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let big = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        const MARG: f64 = 2.6;
        let mut pts: Vec<Vec2> = Vec::new();
        for tile in &r.tiles {
            for q in &tile.poly.pts {
                if q.x > big.min.x + MARG
                    && q.x < big.max.x - MARG
                    && q.y > big.min.y + MARG
                    && q.y < big.max.y - MARG
                    && !pts.iter().any(|o| (*o - *q).length() < 1e-5)
                {
                    pts.push(*q);
                }
            }
        }
        let figs: [Vec<u32>; 3] = [vec![90, 135, 135], vec![90, 90, 180], vec![90, 90, 90, 90]];
        let mut counts: BTreeMap<Vec<u32>, usize> = BTreeMap::new();
        for pt in &pts {
            let mut rays: BTreeSet<i64> = BTreeSet::new();
            for tile in &r.tiles {
                let nn = tile.poly.len();
                for k in 0..nn {
                    if (tile.poly.pts[k] - *pt).length() >= 1e-5 {
                        continue;
                    }
                    for &q in [&tile.poly.pts[(k + nn - 1) % nn], &tile.poly.pts[(k + 1) % nn]] {
                        let dir = q - *pt;
                        let th = dir.y.atan2(dir.x).rem_euclid(std::f64::consts::TAU);
                        rays.insert((th * 1e5).round() as i64);
                    }
                }
                for e in 0..nn {
                    let (a, b) = (tile.poly.pts[e], tile.poly.pts[(e + 1) % nn]);
                    let ab = b - a;
                    let l2 = ab.dot(ab);
                    let tt = (*pt - a).dot(ab) / l2;
                    if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                        continue;
                    }
                    if (*pt - a).cross(ab).abs() / l2.sqrt() < 1e-7 {
                        let th0 = ab.y.atan2(ab.x);
                        for add in [0.0, std::f64::consts::PI] {
                            let th = (th0 + add).rem_euclid(std::f64::consts::TAU);
                            rays.insert((th * 1e5).round() as i64);
                        }
                    }
                }
            }
            let mut mr: Vec<f64> = Vec::new();
            for k in &rays {
                let rv = *k as f64 / 1e5;
                if mr.is_empty() || rv - mr[mr.len() - 1] > 1e-4 {
                    mr.push(rv);
                }
            }
            if mr.len() > 1 && mr[mr.len() - 1] - mr[0] > std::f64::consts::TAU - 1e-4 {
                mr.pop();
            }
            let mut gaps: Vec<f64> = (0..mr.len())
                .map(|i| (mr[(i + 1) % mr.len()] - mr[i]).rem_euclid(std::f64::consts::TAU))
                .collect();
            gaps.retain(|g| *g > 1e-4);
            gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let fig: Vec<u32> = gaps.iter().map(|g| g.to_degrees().round() as u32).collect();
            assert!((3..=4).contains(&fig.len()), "vertex {pt:?}: unexpected figure {fig:?}");
            assert_eq!(fig.iter().sum::<u32>(), 360, "vertex {pt:?} figure {fig:?}");
            assert!(figs.contains(&fig), "unexpected vertex figure {fig:?} at {pt:?}");
            *counts.entry(fig.clone()).or_insert(0) += 1;
        }
        assert_eq!(pts.len(), 134, "total interior vertices");
        let want: BTreeMap<Vec<u32>, usize> =
            [
                (vec![90, 135, 135], 71),
                (vec![90, 90, 180], 50),
                (vec![90, 90, 90, 90], 13),
            ]
            .into_iter()
            .collect();
        assert_eq!(counts, want, "vertex-figure census");
    }

    /// The wallpaper group is cmm: the half-turn about the origin and the
    /// two mirrors in y = x and y = −x preserve the tiling exactly
    /// (interior-AABB set equality on the (−10, −10)–(10, 10) patch, 104
    /// tiles), and the genuine glide g = (R2, u), u = (5, −2) = w1 − w2
    /// in the centered basis, with axis x + y = 3/2 preserves it, and so
    /// does its inverse and the lattice-shifted variants.  g² =
    /// (I + M)·u = (7, −7) = w1 − 2w2 is a lattice translation, and the
    /// axis sits at perpendicular offset (3/4, 3/4) from the origin, so
    /// no conjugate of g is a pure reflection: the perpendicular-part
    /// candidate (R2, (3/2, 3/2)), a reflection in the glide axis
    /// itself, maps the prototile onto an unplaced region (the R2 mirror
    /// lines are at x + y = 3k, while the glide axes fill
    /// x + y = 3k/2).  R90 is not a symmetry: R90(M90@(1,2)) is the
    /// unplaced D180@(−2,1) region while R90(D0@(0,0)) is placed.
    #[test]
    fn pentagon_10_cmm_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        const MI: (f64, f64, f64, f64) = (-1.0, 0.0, 0.0, -1.0);
        const R1: (f64, f64, f64, f64) = (0.0, 1.0, 1.0, 0.0);
        const R2: (f64, f64, f64, f64) = (0.0, -1.0, -1.0, 0.0);
        const R90: (f64, f64, f64, f64) = (0.0, -1.0, 1.0, 0.0);
        let ra = BBox::new(v2(-10.0, -10.0), v2(10.0, 10.0));
        // All three non-identity point-group elements are pure isometries
        // about the origin and preserve the tiling exactly on ra.
        check_iso(&t, &p, MI, Vec2::ZERO, ra, true, "half-turn −I");
        check_iso(&t, &p, R1, Vec2::ZERO, ra, true, "mirror R1 (y = x)");
        check_iso(&t, &p, R2, Vec2::ZERO, ra, true, "mirror R2 (y = −x)");
        // The genuine glide g = (R2, u), u = (5, −2) = w1 − w2: g² =
        // (I + M)·u = (7, −7) = w1 − 2w2 is a lattice translation, and
        // the perpendicular glide part is (3/2, 3/2) = w1/2 — the axis
        // x + y = 3/2 at foot (3/4, 3/4).
        let u = w1 - w2;
        assert_eq!(u, v2(5.0, -2.0), "u = (5, −2) in the centered basis");
        let gs = v2((R2.0 + 1.0) * u.x + R2.1 * u.y, R2.2 * u.x + (R2.3 + 1.0) * u.y);
        assert!((gs - (w1 - w2 * 2.0)).length() < 1e-9, "g² must equal w1 − 2w2, got {gs:?}");
        assert!((gs - v2(7.0, -7.0)).length() < 1e-9, "g² = (7, −7)");
        let t_perp = v2(1.5, 1.5);
        assert!((t_perp - w1 * 0.5).length() < 1e-9, "perpendicular glide part");
        // The glide preserves the tiling (the image window is shifted, so
        // check containment of the images among the tiles generated there)
        // — and so does the inverse, g⁻¹ = (R2, −R2·u) = (R2, w2).
        check_iso(&t, &p, R2, u, ra, false, "glide g (axis x + y = 3/2)");
        check_iso(&t, &p, R2, w2, ra, false, "glide g⁻¹");
        // Big patch for tile-image membership.
        let big = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        let set: HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();
        // Every lattice-shifted variant of the glide maps the prototile
        // onto a tile (the R2 coset is the whole set {(R2, v) : v ∈ L},
        // since R2 ∈ O(L)) — as do the pure mirrors (R2, 0) and (R1, 0)
        // and the mirror-line translations (R2, (6, 6)) [x + y = 6] and
        // (R1, (7, −7)) [x − y = 7].
        for (i, j) in [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0)] {
            let sh = w1 * i + w2 * j;
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(R2, u + sh, *q)).collect();
            assert!(set.contains(&vkey(&img)), "(g + {i}·w1 + {j}·w2) must map cell0 onto a tile");
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(R2, w2 + sh, *q)).collect();
            assert!(
                set.contains(&vkey(&img)),
                "(g⁻¹ + {i}·w1 + {j}·w2) must map cell0 onto a tile"
            );
        }
        for (m, tr, nm) in
            [(R2, Vec2::ZERO, "(R2, 0) mirror x + y = 0"), (R1, Vec2::ZERO, "(R1, 0) mirror y = x")]
        {
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, tr, *q)).collect();
            assert!(set.contains(&vkey(&img)), "{nm} must map cell0 onto a tile");
        }
        for (m, tr, nm) in [
            (R2, v2(6.0, 6.0), "(R2, (6, 6)) mirror x + y = 6"),
            (R1, v2(7.0, -7.0), "(R1, (7, −7)) mirror x − y = 7"),
        ] {
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, tr, *q)).collect();
            assert!(set.contains(&vkey(&img)), "{nm} must map cell0 onto a tile");
        }
        // No conjugate of the glide is a pure reflection: the
        // perpendicular-part candidate (R2, t_perp) is the reflection in
        // the glide axis x + y = 3/2 itself, and the tiling has no mirror
        // line there — the R2 mirror lines are at x + y = 3k.
        let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(R2, t_perp, *q)).collect();
        assert!(
            !set.contains(&vkey(&img)),
            "(R2, t_perp) must not be a symmetry (no mirror on the glide axis)"
        );
        // Nor is an off-lattice R1 shift (R1, (1, 1)): its image region is
        // unplaced, and (1, 1) ∉ L.
        let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(R1, v2(1.0, 1.0), *q)).collect();
        assert!(
            !set.contains(&vkey(&img)),
            "(R1, (1, 1)) must not be a symmetry ((1, 1) ∉ L)"
        );
        // R90 is not a symmetry: R90(M90@(1,2)) is the unplaced D180@(−2,1)
        // region while R90(D0@(0,0)) is placed — the quarter-turn really
        // does not preserve the tiling.
        let m90 = t.cells[4].poly.pts.clone();
        let img: Vec<Vec2> = m90.iter().map(|q| apply_iso(R90, Vec2::ZERO, *q)).collect();
        assert!(!set.contains(&vkey(&img)), "R90(M90@(1,2)) region must be unplaced");
        let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(R90, Vec2::ZERO, *q)).collect();
        assert!(set.contains(&vkey(&img)), "R90(D0@(0,0)) region must be placed");
    }

    /// 3-isohedral: seeds 0/1/2 are the three full-group isohedral classes
    /// — the half-turn, both mirrors and the glide permute the six cells
    /// mod-lattice by the closed permutations, the group they generate has
    /// exactly three orbits on the cells ({0, 2} / {1, 3} / {4, 5},
    /// matching the seeds), all three seeds appear in a 73-tile patch, and
    /// every generated tile keeps the prototile side multiset
    /// {1, 1, √2, 2, 2}.  (At B = 90° the two chiralities coincide as
    /// vertex sets, so the cell-image mechanism is whole-polygon vkey
    /// matching with a lattice shift, not the unique-corner mechanism.)
    #[test]
    fn pentagon_10_isohedral_classes() {
        let t = spec();
        let mut counts = [0usize; 3];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(counts, [2, 2, 2], "two tiles per isohedral class");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        // A symmetry (M, t) permutes the cells: the whole-polygon vertex
        // set of the image must equal that of a cell shifted by a lattice
        // translation.
        let perm_of = |m: (f64, f64, f64, f64), tr: Vec2| -> Vec<usize> {
            let mut perm: Vec<usize> = Vec::new();
            for k in 0..6 {
                let img: Vec<Vec2> =
                    t.cells[k].poly.pts.iter().map(|q| apply_iso(m, tr, *q)).collect();
                let ik = vkey(&img);
                let mut hit = None;
                'outer: for j in 0..6 {
                    for ai in -2..=2 {
                        for bj in -2..=2 {
                            let sh = w1 * ai as f64 + w2 * bj as f64;
                            let cand: Vec<Vec2> =
                                t.cells[j].poly.pts.iter().map(|q| *q + sh).collect();
                            if vkey(&cand) == ik {
                                hit = Some(j);
                                break 'outer;
                            }
                        }
                    }
                }
                perm.push(hit.unwrap_or_else(|| panic!("cell {k} has no image")));
            }
            perm
        };
        let perms = [
            perm_of((-1.0, 0.0, 0.0, -1.0), Vec2::ZERO),
            perm_of((0.0, 1.0, 1.0, 0.0), Vec2::ZERO),
            perm_of((0.0, -1.0, -1.0, 0.0), Vec2::ZERO),
            perm_of((0.0, -1.0, -1.0, 0.0), w1),
        ];
        assert_eq!(perms[0], vec![2, 3, 0, 1, 5, 4], "half-turn perm");
        assert_eq!(perms[1], vec![0, 3, 2, 1, 5, 4], "mirror R1 perm");
        assert_eq!(perms[2], vec![2, 1, 0, 3, 4, 5], "mirror R2 perm");
        assert_eq!(perms[3], vec![2, 1, 0, 3, 4, 5], "glide perm");
        // Three full-group orbits on the cells, exactly the seed classes.
        let mut seen = [false; 6];
        let mut orbits: Vec<Vec<usize>> = Vec::new();
        for st in 0..6 {
            if seen[st] {
                continue;
            }
            let mut orb = vec![st];
            seen[st] = true;
            let mut head = 0;
            while head < orb.len() {
                let x = orb[head];
                head += 1;
                for pm in &perms {
                    let y = pm[x];
                    if !seen[y] {
                        seen[y] = true;
                        orb.push(y);
                    }
                }
            }
            orb.sort_unstable();
            orbits.push(orb);
        }
        assert_eq!(orbits, vec![vec![0, 2], vec![1, 3], vec![4, 5]], "three full-group orbits");
        for orb in &orbits {
            let sd = t.cells[orb[0]].seed;
            assert!(orb.iter().all(|&k| t.cells[k].seed == sd), "orbit mixes isohedral seeds");
        }
        assert_ne!(t.cells[orbits[0][0]].seed, t.cells[orbits[1][0]].seed, "orbits differ");
        assert_ne!(t.cells[orbits[0][0]].seed, t.cells[orbits[2][0]].seed, "orbits differ");
        // Generated patches keep the side multiset and show all three
        // classes.
        let params = ParamSet::default_for(t.params());
        let rp = t.generate(
            &params,
            &Query {
                region: BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
            },
        );
        assert!(rp.message.is_none(), "{:?}", rp.message);
        assert_eq!(rp.tiles.len(), 73, "window census");
        let mut seen_s = [false; 3];
        for tile in &rp.tiles {
            let sd = tile.seed as usize;
            assert!(sd < 3, "seeds must stay in {{0, 1, 2}}");
            seen_s[sd] = true;
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length())
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [1.0, 1.0, f64::sqrt(2.0), 2.0, 2.0];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!(
                    (got - w).abs() < 1e-12,
                    "seed {sd} side {got} ∉ {{1, 1, √2, 2, 2}}"
                );
            }
        }
        assert!(seen_s[0] && seen_s[1] && seen_s[2], "all three isohedral classes must appear");
    }
}
#[cfg(all(test, feature = "pentagon1"))]
mod pentagon1_tests {
    use super::pentagon1::{cell_table, prototile, s3, tile_area, two_fold_centre};
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon1::spec;
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Reflection matrix in the direction u (u must be unit): 2uuᵀ − I.
    fn mirror(u: Vec2) -> (f64, f64, f64, f64) {
        (2.0 * u.x * u.x - 1.0, 2.0 * u.x * u.y, 2.0 * u.x * u.y, 2.0 * u.y * u.y - 1.0)
    }

    /// Rotation matrix through `deg` degrees (counter-clockwise).
    fn rot(deg: f64) -> (f64, f64, f64, f64) {
        let r = deg * core::f64::consts::PI / 180.0;
        let (co, si) = (r.cos(), r.sin());
        (co, -si, si, co)
    }

    /// Translation part of the isometry with linear part `m` and fixed
    /// point `c` (a rotation centre, or a point on the mirror/glide
    /// axis): t = c − M·c.
    fn fixed_tr(m: (f64, f64, f64, f64), c: Vec2) -> Vec2 {
        let (a, b, cc, d) = m;
        v2(c.x - (a * c.x + b * c.y), c.y - (cc * c.x + d * c.y))
    }

    /// Vertex-set key on a 1e-6 grid (shared joints agree to ~1e-15 between
    /// two computations of the same tile, since every cell is built from
    /// the same runtime prototile array).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    /// True when the tile's AABB lies inside `w`.  A tile whose AABB is
    /// inside `ra` has its polygon inside `ra`, so its image under any
    /// isometry g intersects g(ra) ⊆ bbox(g(ra)) — and a tile whose polygon
    /// intersects the region always lies inside the generator's padded
    /// index range — hence the image is always generated for rb.
    fn aabb_inside(poly: &Polygon, w: BBox) -> bool {
        let bb = poly.aabb();
        bb.min.x >= w.min.x
            && bb.min.y >= w.min.y
            && bb.max.x <= w.max.x
            && bb.max.y <= w.max.y
    }

    /// Verify that the isometry g = (M, t) preserves the tiling on `ra`:
    /// images of interior-AABB tiles of ra must be tiles generated for the
    /// image window bbox(g(ra)).  With `exact = true` (the half-turn case,
    /// g(ra) = ra) the filtered image set must equal the interior-AABB
    /// tile set of the window.
    fn check_iso(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            v2(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            v2(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ta = t.generate(params, &Query { region: ra });
        let tb = t.generate(params, &Query { region: rb });
        assert!(ta.message.is_none(), "{label}: {:?}", ta.message);
        assert!(tb.message.is_none(), "{label}: {:?}", tb.message);
        let src: Vec<_> = ta.tiles.iter().filter(|tile| aabb_inside(&tile.poly, ra)).collect();
        assert!(!src.is_empty(), "{label}: no interior tiles");
        let set_b: HashSet<Vec<(i64, i64)>> = tb
            .tiles
            .iter()
            .filter(|tile| !exact || aabb_inside(&tile.poly, rb))
            .map(|tile| vkey(&tile.poly.pts))
            .collect();
        let set_a: HashSet<Vec<(i64, i64)>> = src
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(set_a.len(), src.len(), "{label}: isometry must be injective on the patch");
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    #[test]
    fn pentagon_1_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "pentagon-1", &p, BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)));
    }

    #[test]
    fn pentagon_1_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-1(rot30,scale0.7)",
            BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_1_deterministic() {
        check_det(&spec(), "pentagon-1", BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)));
    }

    /// Unit structure of the primitive cell: 2 congruent copies of the
    /// exact-Q(√3) type-1 prototile (A = B = C = 120°, D = E = 90°,
    /// a = b = 2, c = e = 1, d = 2√3, area 3√3) on the centered-
    /// rectangular lattice in its 60° rhombus basis w1 = (3, −√3),
    /// w2 = (3, √3), det = 6√3 = 2 × tile area (|w1|² = |w2|² = 12,
    /// w1·w2 = 6: the special a² = 3b² case in which a centered-
    /// rectangular cell is hexagonal-metric).  The closure identity
    /// holds to ~1e-15, both cells are CCW and directly congruent
    /// (cell 1 is the half-turn image of cell 0 about the hexagon
    /// centre (−2, 0) — same chirality, no mirror in the cell table),
    /// and the seeds mark the single isohedral orbit.
    #[test]
    fn pentagon_1_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-1");
        assert_eq!(t.cells.len(), 2, "the primitive unit holds two tiles");
        assert_eq!(t.lattice[0], v2(3.0, -s3()));
        assert_eq!(t.lattice[1], v2(3.0, s3()));
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        let a0 = t.cells[0].poly.area();
        assert!((det - 2.0 * a0).abs() < 1e-11, "det {det} ≠ 2 × tile area");
        assert!((det - 6.0 * s3()).abs() < 1e-11, "det {det} ≠ 6√3");
        assert!((a0 - tile_area()).abs() < 1e-11, "tile area ≠ 3√3");
        // 60° rhombus basis of the centered-rectangular lattice:
        // |w1|² = |w2|² = 12, w1·w2 = 6.
        assert!((w1.dot(w1) - 12.0).abs() < 1e-12, "|w1|² = 12");
        assert!((w2.dot(w2) - 12.0).abs() < 1e-12, "|w2|² = 12");
        assert!((w1.dot(w2) - 6.0).abs() < 1e-12, "w1·w2 = 6");
        // The prototile closes: edge directions 120°, 180°, 240°, 330° of
        // lengths 2, 2, 1, 2√3; closing edge E→A has direction 60°,
        // length 1.
        let pr = prototile();
        assert!((pr[4].x + 0.5).abs() < 1e-12, "closure E.x = −1/2, got {}", pr[4].x);
        assert!(
            (pr[4].y + s3() / 2.0).abs() < 1e-12,
            "closure E.y = −√3/2, got {}",
            pr[4].y
        );
        let proto = &t.cells[0].poly;
        assert_eq!(proto.pts[0], v2(0.0, 0.0), "prototile corner A at the origin");
        for (i, want) in [(0usize, 120.0f64), (1, 120.0), (2, 120.0), (3, 90.0), (4, 90.0)] {
            let got = angle_at(proto, i).to_degrees();
            assert!((got - want).abs() < 1e-9, "angle at vertex {i}: {got} ≠ {want}");
        }
        for (i, want) in [(0usize, 2.0f64), (1, 2.0), (2, 1.0), (3, 2.0 * s3()), (4, 1.0)] {
            assert!((side_of(proto, i) - want).abs() < 1e-12, "side {i}");
        }
        // Type-1 family identities in the member's own lettering: under
        // the relabelling (A′, B′, C′, D′, E′) = (C, D, E, A, B) the
        // family condition is B′ + C′ = D + E = 180°, equivalently
        // A + B + C = 360°.  Side identities: a = b, c = e.
        let d_angle = angle_at(proto, 3).to_degrees();
        let e_angle = angle_at(proto, 4).to_degrees();
        assert!((d_angle + e_angle - 180.0).abs() < 1e-9, "D + E = 180°");
        let abc: f64 = (0..3).map(|i| angle_at(proto, i).to_degrees()).sum();
        assert!((abc - 360.0).abs() < 1e-9, "A + B + C = 360°");
        assert!((side_of(proto, 0) - side_of(proto, 1)).abs() < 1e-12, "a = b");
        assert!((side_of(proto, 2) - side_of(proto, 4)).abs() < 1e-12, "c = e");
        // Single isohedral orbit: both seeds 0.
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 0], "1-isohedral seeds");
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        let mut seen: HashSet<Vec<(i64, i64)>> = HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            assert!((cell.poly.area() - a0).abs() < 1e-12, "cell {k} area");
            assert_eq!(cell.poly.len(), 5, "cell {k} vertex count");
            // Cyclic (side, angle) fingerprint against the prototile:
            // forward only — both cells carry the same chirality.
            let cs = (0..5).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..5).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd, "cell {k} is not a directly-congruent type-1 prototile");
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
        }
    }

    /// Window census (default parameters) and declared metadata: culling
    /// is the tile AABB intersecting the region (+1e-9), matching the
    /// independent Python emulation of the lattice.rs culling.
    #[test]
    fn pentagon_1_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::Cmm);
        assert_eq!(t.lattice_kind, LatticeKind::CenteredRectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the cell-0 tile (seed 0)");
        let p = ParamSet::default_for(t.params());
        for (region, want) in [
            (BBox::new(v2(-6.0, -4.0), v2(8.0, 6.0)), 42),
            (BBox::new(v2(-8.0, -6.0), v2(10.0, 8.0)), 66),
            (BBox::new(v2(-10.0, -8.0), v2(14.0, 10.0)), 114),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }

    /// Refined edge census over 94 tiles: the tiling is EDGE-TO-EDGE at
    /// the pentagon level.  Of the sides whose midpoint lies strictly
    /// inside the window (−9.4, 13.4) × (−7.4, 9.4), every one of the
    /// 374 is matched by exactly one full side of one neighbour and no
    /// side carries a T-junction (n_split = 0): whole hexagon sides pair
    /// full to full, each half side (length 1) pairs one-to-one with a
    /// neighbouring half side, and each cut d-side (2√3) pairs with the
    /// companion tile's d-side in the same hexagon.  The window is an
    /// exact count with a minimum boundary margin of 0.0388 from any
    /// counted midpoint (verified against the exact Q(√3) tiling), so it
    /// is robust against the ~1e-15 float difference between the
    /// runtime-computed prototile and the exact surds.
    #[test]
    fn pentagon_1_refined_edge_census() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let region = BBox::new(v2(-9.4, -7.4), v2(13.4, 9.4));
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 94, "patch size");
        let verts: Vec<Vec<Vec2>> = r.tiles.iter().map(|tile| tile.poly.pts.clone()).collect();
        let n = verts.len();
        let mut n_split = 0;
        let mut n_full = 0;
        let mut bad: Vec<String> = Vec::new();
        for i in 0..n {
            for e in 0..5 {
                let (pp, qq) = (verts[i][e], verts[i][(e + 1) % 5]);
                let mid = (pp + qq) * 0.5;
                if !(mid.x > -9.4 && mid.x < 13.4 && mid.y > -7.4 && mid.y < 9.4) {
                    continue;
                }
                let dv = qq - pp;
                let l2 = dv.dot(dv);
                let l = l2.sqrt();
                // Split points: other tiles' vertices strictly inside the
                // open segment (collinear within 1e-7), binned at 1e-9 and
                // merged within 1e-8.
                let mut bins: BTreeSet<i64> = BTreeSet::new();
                for (j, vj) in verts.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    for &vv in vj.iter() {
                        let tt = (vv - pp).dot(dv) / l2;
                        if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                            continue;
                        }
                        let cross = (vv.x - pp.x) * dv.y - (vv.y - pp.y) * dv.x;
                        if cross.abs() / l < 1e-7 {
                            bins.insert((tt * 1e9).round() as i64);
                        }
                    }
                }
                let mut srt: Vec<i64> = Vec::new();
                for b in bins {
                    if srt.is_empty() || b - srt[srt.len() - 1] > 10 {
                        srt.push(b);
                    }
                }
                if srt.len() > 1 {
                    bad.push(format!("tile {i} edge {e}: {srt:?}"));
                    continue;
                }
                let mut ends: Vec<f64> = vec![0.0];
                ends.extend(srt.iter().map(|b| *b as f64 / 1e9));
                ends.push(1.0);
                for w in ends.windows(2) {
                    let aq = pp + dv * w[0];
                    let seg = dv * (w[1] - w[0]);
                    let lseg = seg.length();
                    let uh = seg * (1.0 / lseg);
                    let mut cnt = 0;
                    for (j, vj) in verts.iter().enumerate() {
                        if j == i {
                            continue;
                        }
                        for (a1, b1) in (0..5).map(|k| (vj[k], vj[(k + 1) % 5])) {
                            let c1 = (a1.x - aq.x) * uh.y - (a1.y - aq.y) * uh.x;
                            let c2 = (b1.x - aq.x) * uh.y - (b1.y - aq.y) * uh.x;
                            if c1.abs() > 1e-7 || c2.abs() > 1e-7 {
                                continue;
                            }
                            let ta = (a1 - aq).dot(uh);
                            let tb = (b1 - aq).dot(uh);
                            if ta.min(tb) <= 1e-7 * lseg && ta.max(tb) >= lseg * (1.0 - 1e-7) {
                                cnt += 1;
                            }
                        }
                    }
                    if cnt != 1 {
                        bad.push(format!(
                            "tile {i} edge {e} sub [{:.3}, {:.3}] has {cnt} covering edges",
                            w[0], w[1]
                        ));
                    }
                }
                if srt.len() == 1 {
                    n_split += 1;
                } else {
                    n_full += 1;
                }
            }
        }
        assert!(bad.is_empty(), "refined edge census violations: {bad:?}");
        assert_eq!(n_full, 374, "full (unsplit) edges");
        assert_eq!(n_split, 0, "no T-junctions: the tiling is edge-to-edge");
    }

    /// Vertex-figure census over the window (−9.4, −7.4)–(13.4, 9.4):
    /// every interior vertex (margin 1e-3 from the window edge; the
    /// tightest counted vertex in this window sits 0.4 away, so the scan
    /// is boundary-safe) is a complete star — corner rays plus rays of
    /// edges passing through it (the T-junction points; none occur, the
    /// tiling is edge-to-edge) — merging within 1e-4 rad, and its
    /// sectors are exactly the two figures {120°, 120°, 120°} (the
    /// hexagon-corner vertices, 75 of them) and {90°, 90°, 90°, 90°}
    /// (the cut-midpoint vertices, 31 of them); 106 vertices in total.
    #[test]
    fn pentagon_1_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let big = BBox::new(v2(-9.4, -7.4), v2(13.4, 9.4));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        const MARG: f64 = 1e-3;
        let mut pts: Vec<Vec2> = Vec::new();
        for tile in &r.tiles {
            for q in &tile.poly.pts {
                if q.x > big.min.x + MARG
                    && q.x < big.max.x - MARG
                    && q.y > big.min.y + MARG
                    && q.y < big.max.y - MARG
                    && !pts.iter().any(|o| (*o - *q).length() < 1e-5)
                {
                    pts.push(*q);
                }
            }
        }
        // 3- and 4-valent figures coexist, so key the census by the
        // sorted angle vector.
        let figs: [Vec<u32>; 2] = [vec![120, 120, 120], vec![90, 90, 90, 90]];
        let mut counts: BTreeMap<Vec<u32>, usize> = BTreeMap::new();
        for pt in &pts {
            let mut rays: BTreeSet<i64> = BTreeSet::new();
            for tile in &r.tiles {
                let nn = tile.poly.len();
                for k in 0..nn {
                    if (tile.poly.pts[k] - *pt).length() >= 1e-5 {
                        continue;
                    }
                    for &q in [&tile.poly.pts[(k + nn - 1) % nn], &tile.poly.pts[(k + 1) % nn]] {
                        let dir = q - *pt;
                        let th = dir.y.atan2(dir.x).rem_euclid(std::f64::consts::TAU);
                        rays.insert((th * 1e5).round() as i64);
                    }
                }
                for e in 0..nn {
                    let (a, b) = (tile.poly.pts[e], tile.poly.pts[(e + 1) % nn]);
                    let ab = b - a;
                    let l2 = ab.dot(ab);
                    let tt = (*pt - a).dot(ab) / l2;
                    if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                        continue;
                    }
                    if (*pt - a).cross(ab).abs() / l2.sqrt() < 1e-7 {
                        let th0 = ab.y.atan2(ab.x);
                        for add in [0.0, std::f64::consts::PI] {
                            let th = (th0 + add).rem_euclid(std::f64::consts::TAU);
                            rays.insert((th * 1e5).round() as i64);
                        }
                    }
                }
            }
            let mut mr: Vec<f64> = Vec::new();
            for k in &rays {
                let rv = *k as f64 / 1e5;
                if mr.is_empty() || rv - mr[mr.len() - 1] > 1e-4 {
                    mr.push(rv);
                }
            }
            if mr.len() > 1 && mr[mr.len() - 1] - mr[0] > std::f64::consts::TAU - 1e-4 {
                mr.pop();
            }
            let mut gaps: Vec<f64> = (0..mr.len())
                .map(|i| (mr[(i + 1) % mr.len()] - mr[i]).rem_euclid(std::f64::consts::TAU))
                .collect();
            gaps.retain(|g| *g > 1e-4);
            gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let fig: Vec<u32> = gaps.iter().map(|g| g.to_degrees().round() as u32).collect();
            assert!(figs.contains(&fig), "unexpected vertex figure {fig:?} at {pt:?}");
            assert_eq!(fig.iter().sum::<u32>(), 360, "vertex {pt:?} figure {fig:?}");
            *counts.entry(fig).or_insert(0) += 1;
        }
        assert_eq!(pts.len(), 106, "total interior vertices");
        let want: BTreeMap<Vec<u32>, usize> =
            [(vec![120u32, 120, 120], 75), (vec![90u32, 90, 90, 90], 31)]
                .into_iter()
                .collect();
        assert_eq!(counts, want, "vertex-figure census");
    }

    /// The uncoloured wallpaper group is cmm (orbifold \*2222), pinned
    /// here by a coset-style audit over the 257-tile patch
    /// (−16, −14)–(20, 16):
    ///
    /// POSITIVE — the half-turn about c0 = (−2, 0), the hexagon centre
    /// that swaps the two cells, preserves the tiling exactly (set
    /// equality on the symmetric window); the lattice translations i·w1,
    /// j·w2 carry the prototile onto tiles for i, j ∈ [−2, 2]; the
    /// half-turn census over the quarter lattice c0 + (i·w1 + j·w2)/4
    /// finds 2-fold centres exactly at the four coset representatives
    /// c0 + Λ/2 — the hexagon centre (on both mirror families), the cut
    /// midpoint (−1/2, −√3/2) (on both), and the two off-mirror classes
    /// (−1/2, √3/2) and (1, 0) — while the bare quarter lattice
    /// (i·w1 + j·w2)/4 has none; the two perpendicular mirror families
    /// (60° lines at s = p·ê150 ≡ 0 mod √3, and 150° lines at
    /// t = p·ê60 ≡ −1 mod 3) and the six glide axes between them (60°
    /// at s ≡ √3/2 mod √3 with vector 3ê60 whose square is the lattice
    /// translation −w1 + 2w2; 150° at t ≡ 1/2 mod 3 with vector
    /// √3·ê150 whose square is −w1) all preserve the tiling; and the
    /// prototile is achiral — a −120° rotation about c0 maps it exactly
    /// onto its y-mirror — so orientation-reversing symmetries exist.
    ///
    /// NEGATIVE — the half-lattice shifts w1/2, w2/2, (w1+w2)/2,
    /// (w1−w2)/2 are not translations; 32 off-family mirror lines
    /// (60°/150° lines with non-mirror offsets, horizontal, vertical,
    /// and 30° lines) are not mirrors; 4 extra glide candidates are not
    /// glides; and no 60/120/240/300° rotation about any of the six
    /// hexagon-corner classes (with 3×3 lattice shifts) is a symmetry
    /// (216 probes).  With mirrors + glides, 2-folds on and off the
    /// mirrors, a centered-rectangular translation lattice (basis
    /// (6, 0), (0, 2√3), centering (3, √3) — not primitive-rectangular)
    /// and no 3-/6-folds, the group is cmm rather than pmm (which needs
    /// all 2-folds on mirrors of a primitive rectangular lattice) and
    /// rather than p2 (which has no orientation-reversing isometries).
    #[test]
    fn pentagon_1_wallpaper_group_is_cmm() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let c0 = two_fold_centre();
        let e60 = v2(0.5, 0.5 * s3());
        let e150 = v2(-0.5 * s3(), 0.5);

        // Big patch for tile-image membership: every probed image below
        // (prototile images under i, j ∈ [−2, 2] lattice shifts, and the
        // half-turn/mirror/rotation probes centred within one cell of the
        // origin) lies inside this footprint, so membership in the
        // generated set is the exact question.
        let big = BBox::new(v2(-16.0, -14.0), v2(20.0, 16.0));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        assert_eq!(big_tiles.tiles.len(), 257, "big patch tile count");
        let set: HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();

        // Positive: the exact half-turn about c0 (g(ra) = ra).
        let ra = BBox::new(v2(-14.0, -10.0), v2(10.0, 10.0));
        check_iso(
            &t,
            &p,
            (-1.0, 0.0, 0.0, -1.0),
            c0 * 2.0,
            ra,
            true,
            "half-turn about c0",
        );

        // Positive: lattice translations carry the prototile onto tiles.
        for i in -2..=2 {
            for &w in [&w1, &w2] {
                let img: Vec<Vec2> = prot.iter().map(|q| *q + w * i as f64).collect();
                assert!(set.contains(&vkey(&img)), "cell0 + {i}·{w:?} must be a tile");
            }
        }
        // Negative: half-lattice shifts are not translations.
        for h in [w1 * 0.5, w2 * 0.5, (w1 + w2) * 0.5, (w1 - w2) * 0.5] {
            let img: Vec<Vec2> = prot.iter().map(|q| *q + h).collect();
            assert!(
                !set.contains(&vkey(&img)),
                "half-lattice shift {h:?} is a symmetry"
            );
        }

        // Half-turn census over c0 + (i·w1 + j·w2)/4: the only 2-fold
        // centres are the four coset representatives c0 + Λ/2.
        let mut found: Vec<(i8, i8)> = Vec::new();
        for i in 0i8..4 {
            for j in 0i8..4 {
                let c = c0 + (w1 * i as f64 + w2 * j as f64) * 0.25;
                let img: Vec<Vec2> =
                    prot.iter().map(|q| apply_iso((-1.0, 0.0, 0.0, -1.0), c * 2.0, *q)).collect();
                if set.contains(&vkey(&img)) {
                    found.push((i, j));
                }
            }
        }
        assert_eq!(
            found,
            vec![(0, 0), (0, 2), (2, 0), (2, 2)],
            "2-fold centres must be exactly c0 + Λ/2"
        );
        // The bare quarter-lattice points (i·w1 + j·w2)/4 are not centres.
        for i in 0i8..4 {
            for j in 0i8..4 {
                let c = (w1 * i as f64 + w2 * j as f64) * 0.25;
                let img: Vec<Vec2> =
                    prot.iter().map(|q| apply_iso((-1.0, 0.0, 0.0, -1.0), c * 2.0, *q)).collect();
                assert!(
                    !set.contains(&vkey(&img)),
                    "bare quarter point ({i}, {j}) must not be a 2-fold centre"
                );
            }
        }
        // The four classes c0 + Λ/2 survive every whole-lattice shift.
        for h in [Vec2::ZERO, w1 * 0.5, w2 * 0.5, (w1 + w2) * 0.5] {
            for i in -1..=1i8 {
                for j in -1..=1i8 {
                    let c = c0 + h + w1 * i as f64 + w2 * j as f64;
                    let img: Vec<Vec2> = prot
                        .iter()
                        .map(|q| apply_iso((-1.0, 0.0, 0.0, -1.0), c * 2.0, *q))
                        .collect();
                    assert!(
                        set.contains(&vkey(&img)),
                        "half-turn about c0 + {h:?} + {i}·w1 + {j}·w2 must map cell0 onto a tile"
                    );
                }
            }
        }

        // Positive mirrors: 60° lines at s ≡ 0 (mod √3), direction ê60,
        // through the point s·ê150; 150° lines at t ≡ −1 (mod 3),
        // direction ê150, through t·ê60.
        let m60 = mirror(e60);
        let m150 = mirror(e150);
        for k in -1..=1i32 {
            let c = e150 * (k as f64 * s3());
            check_iso(&t, &p, m60, fixed_tr(m60, c), ra, false, "60° mirror");
        }
        for j in -1..=1i32 {
            let c = e60 * (3.0 * j as f64 - 1.0);
            check_iso(&t, &p, m150, fixed_tr(m150, c), ra, false, "150° mirror");
        }

        // Positive glides between the mirror families: the axis
        // reflection followed by the glide translation (each square lies
        // in Λ, so each axis is a genuine glide, not a mirror).
        let g60 = e60 * 3.0; // square: −w1 + 2w2
        let g150 = e150 * s3(); // square: −w1
        for k in -1..=1i32 {
            let c = e150 * ((k as f64 + 0.5) * s3());
            check_iso(&t, &p, m60, fixed_tr(m60, c) + g60, ra, false, "60° glide");
        }
        for j in -1..=1i32 {
            let c = e60 * (3.0 * j as f64 + 0.5);
            check_iso(&t, &p, m150, fixed_tr(m150, c) + g150, ra, false, "150° glide");
        }

        // Negative mirrors: 32 off-family lines — the reflected
        // prototile must not be a tile of the big patch.
        let mut n_neg = 0;
        for k in -1..=1i32 {
            for f in [0.25f64, 0.5, 0.75] {
                let c = e150 * (k as f64 + f) * s3();
                let img: Vec<Vec2> =
                    prot.iter().map(|q| apply_iso(m60, fixed_tr(m60, c), *q)).collect();
                assert!(
                    !set.contains(&vkey(&img)),
                    "60° mirror at s = √3({k} + {f}) is a symmetry"
                );
                n_neg += 1;
            }
        }
        for j in -1..=1i32 {
            for dd in [0.5f64, 1.0, 1.5] {
                let c = e60 * (3.0 * j as f64 - 1.0 + dd);
                let img: Vec<Vec2> =
                    prot.iter().map(|q| apply_iso(m150, fixed_tr(m150, c), *q)).collect();
                assert!(
                    !set.contains(&vkey(&img)),
                    "150° mirror at t = 3·{j} − 1 + {dd} is a symmetry"
                );
                n_neg += 1;
            }
        }
        let mx = mirror(v2(1.0, 0.0));
        let my = mirror(v2(0.0, 1.0));
        for y in [0.0f64, s3() / 4.0, s3() / 2.0, 3.0 * s3() / 4.0] {
            let c = v2(0.0, y);
            let img: Vec<Vec2> =
                prot.iter().map(|q| apply_iso(mx, fixed_tr(mx, c), *q)).collect();
            assert!(!set.contains(&vkey(&img)), "horizontal mirror y = {y} is a symmetry");
            n_neg += 1;
        }
        for x in [0.0f64, 0.75, 1.5, 2.25] {
            let c = v2(x, 0.0);
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(my, fixed_tr(my, c), *q)).collect();
            assert!(!set.contains(&vkey(&img)), "vertical mirror x = {x} is a symmetry");
            n_neg += 1;
        }
        let m30 = mirror(v2(0.5 * s3(), 0.5));
        for k in -1..=1i32 {
            let c = v2(-0.5, 0.5 * s3()) * (3.0 * k as f64 / 4.0);
            let img: Vec<Vec2> =
                prot.iter().map(|q| apply_iso(m30, fixed_tr(m30, c), *q)).collect();
            assert!(!set.contains(&vkey(&img)), "30° mirror (offset {k}) is a symmetry");
            n_neg += 1;
        }
        for k in -1..=1i32 {
            let c = v2(0.5 * s3(), 0.5) * (s3() * k as f64 / 4.0);
            let img: Vec<Vec2> =
                prot.iter().map(|q| apply_iso(m150, fixed_tr(m150, c), *q)).collect();
            assert!(
                !set.contains(&vkey(&img)),
                "150° mirror (offset {k}) is a symmetry"
            );
            n_neg += 1;
        }
        assert_eq!(n_neg, 32, "negative mirror line count");

        // Negative glides: glides on the mirror lines (which would
        // compose with the mirror into non-lattice translations), and
        // two axis directions with glide squares outside Λ.
        let g_neg = [
            (m60, v2(0.0, 0.0), g60),
            (m150, e60 * -1.0, g150),
            (mx, v2(0.0, 0.0), v2(1.5, 0.0)),
            (my, v2(0.0, 0.0), v2(0.0, s3() / 2.0)),
        ];
        for (m, c, g) in g_neg {
            let img: Vec<Vec2> =
                prot.iter().map(|q| apply_iso(m, fixed_tr(m, c) + g, *q)).collect();
            assert!(!set.contains(&vkey(&img)), "glide about {c:?} + {g:?} is a symmetry");
        }

        // No 60/120/240/300° rotation about any hexagon-corner class:
        // 6 corner classes × 3×3 lattice shifts × 4 angles = 216 probes.
        let corners = [
            v2(0.0, 0.0),
            v2(-1.0, s3()),
            v2(-3.0, s3()),
            v2(-4.0, 0.0),
            v2(-3.0, -s3()),
            v2(-1.0, -s3()),
        ];
        let mut n_rot = 0;
        for cv in corners {
            for i in -1..=1i8 {
                for j in -1..=1i8 {
                    let c = cv + w1 * i as f64 + w2 * j as f64;
                    for deg in [60.0f64, 120.0, 240.0, 300.0] {
                        let m = rot(deg);
                        let img: Vec<Vec2> =
                            prot.iter().map(|q| apply_iso(m, fixed_tr(m, c), *q)).collect();
                        assert!(
                            !set.contains(&vkey(&img)),
                            "{deg}° rotation about {c:?} is a symmetry"
                        );
                        n_rot += 1;
                    }
                }
            }
        }
        assert_eq!(n_rot, 216, "rotation negative probe count");

        // The prototile is achiral: R(−120°) about the hexagon centre c0
        // maps it exactly onto its y-mirror (A↦C′, B↦B′, C↦A′, D↦E′,
        // E↦D′), so orientation-reversing symmetries exist and the group
        // cannot be p2.
        let m_120 = rot(-120.0);
        let img: Vec<Vec2> =
            prot.iter().map(|q| apply_iso(m_120, fixed_tr(m_120, c0), *q)).collect();
        let pmir: Vec<Vec2> = prot.iter().map(|q| v2(q.x, -q.y)).collect();
        assert_eq!(
            vkey(&img),
            vkey(&pmir),
            "achirality: R(−120° about c0) must map the prototile onto its y-mirror"
        );
    }

    /// 1-isohedral: both cells carry seed 0 and the half-turn about c0
    /// = (−2, 0) permutes the cell table mod lattice as the transposition
    /// (0 1), so the two cells form a single full-group orbit; every
    /// generated tile keeps the prototile side multiset
    /// {1, 1, 2, 2, 2√3} and seed 0.
    #[test]
    fn pentagon_1_isohedral_single_orbit() {
        let t = spec();
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 0], "single isohedral class");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        let u_of = |p: Vec2| {
            v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
        };
        let mod1 = |x: f64| (x + 0.5).rem_euclid(1.0) - 0.5;
        let table = cell_table();
        let c0 = two_fold_centre();
        // The half-turn maps each cell's anchor to the other cell's
        // anchor mod lattice, rotating the tile by 180° (the table has
        // no mirror, so both cells keep the same chirality).
        let mut perm: Vec<usize> = Vec::new();
        for k in 0..2 {
            let (_fam_k, a_k, th_k) = table[k];
            let ai = apply_iso((-1.0, 0.0, 0.0, -1.0), c0 * 2.0, a_k);
            let th_img = 180.0 + th_k;
            let mut hit = None;
            for (j, &(_fam_j, a_j, th_j)) in table.iter().enumerate() {
                let du = u_of(ai - a_j);
                if mod1(du.x).abs() < 1e-6
                    && mod1(du.y).abs() < 1e-6
                    && ((th_img - th_j + 180.0).rem_euclid(360.0) - 180.0).abs() < 1e-6
                {
                    hit = Some(j);
                    break;
                }
            }
            perm.push(hit.unwrap_or_else(|| panic!("cell {k} has no half-turn image")));
        }
        assert_eq!(perm, vec![1, 0], "half-turn permutes the cells as (0 1)");
        // Generated patches: single seed, prototile side multiset kept.
        let params = ParamSet::default_for(t.params());
        let rp = t.generate(&params, &Query { region: BBox::new(v2(-6.0, -6.0), v2(10.0, 10.0)) });
        assert!(rp.message.is_none(), "{:?}", rp.message);
        assert_eq!(rp.tiles.len(), 69, "isohedral patch size");
        for tile in &rp.tiles {
            assert_eq!(tile.seed, 0, "single isohedral orbit -> seed 0 only");
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length())
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [1.0, 1.0, 2.0, 2.0, 2.0 * s3()];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "side {got} ∉ {{1, 1, 2, 2, 2√3}}");
            }
        }
    }
}
#[cfg(all(test, feature = "pentagon3"))]
mod pentagon3_tests {
    use super::pentagon3::{cell_table, prototile, s3, tile_area, two_fold_centre};
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon3::spec;
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Reflection matrix in the direction u (u must be unit): 2uuᵀ − I.
    fn mirror(u: Vec2) -> (f64, f64, f64, f64) {
        (2.0 * u.x * u.x - 1.0, 2.0 * u.x * u.y, 2.0 * u.x * u.y, 2.0 * u.y * u.y - 1.0)
    }

    /// Vertex-set key on a 1e-6 grid (shared joints agree to ~1e-15 between
    /// two computations of the same tile, since every cell is built from
    /// the same runtime prototile array).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    /// True when the tile's AABB lies inside `w`.  A tile whose AABB is
    /// inside `ra` has its polygon inside `ra`, so its image under any
    /// isometry g intersects g(ra) ⊆ bbox(g(ra)) — and a tile whose polygon
    /// intersects the region always lies inside the generator's padded
    /// index range — hence the image is always generated for rb.
    fn aabb_inside(poly: &Polygon, w: BBox) -> bool {
        let bb = poly.aabb();
        bb.min.x >= w.min.x
            && bb.min.y >= w.min.y
            && bb.max.x <= w.max.x
            && bb.max.y <= w.max.y
    }

    /// Verify that the isometry g = (M, t) preserves the tiling on `ra`:
    /// images of interior-AABB tiles of ra must be tiles generated for the
    /// image window bbox(g(ra)).  With `exact = true` (the half-turn case,
    /// g(ra) = ra) the filtered image set must equal the interior-AABB
    /// tile set of the window.
    fn check_iso(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            v2(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            v2(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ta = t.generate(params, &Query { region: ra });
        let tb = t.generate(params, &Query { region: rb });
        assert!(ta.message.is_none(), "{label}: {:?}", ta.message);
        assert!(tb.message.is_none(), "{label}: {:?}", tb.message);
        let src: Vec<_> = ta.tiles.iter().filter(|tile| aabb_inside(&tile.poly, ra)).collect();
        assert!(!src.is_empty(), "{label}: no interior tiles");
        let set_b: HashSet<Vec<(i64, i64)>> = tb
            .tiles
            .iter()
            .filter(|tile| !exact || aabb_inside(&tile.poly, rb))
            .map(|tile| vkey(&tile.poly.pts))
            .collect();
        let set_a: HashSet<Vec<(i64, i64)>> = src
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(set_a.len(), src.len(), "{label}: isometry must be injective on the patch");
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    #[test]
    fn pentagon_3_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "pentagon-3", &p, BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)));
    }

    #[test]
    fn pentagon_3_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-3(rot30,scale0.7)",
            BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_3_deterministic() {
        check_det(&spec(), "pentagon-3", BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)));
    }

    /// Unit structure of the primitive cell: 2 congruent copies of the
    /// exact-Q(√3) type-3 prototile (A = B = C = D = 120°, E = 60°,
    /// a = b = 2, c = 1, d = 4, e = 3, area 5√3) on the oblique lattice
    /// w1 = (1/2, 3√3/2), w2 = (−13/2, √3/2) with det = 10√3 = 2 × tile
    /// area.  The closure identity holds to ~1e-15, both cells are CCW
    /// and congruent (same chirality — no mirror in the cell table), and
    /// the seeds mark the single isohedral orbit.
    #[test]
    fn pentagon_3_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-3");
        assert_eq!(t.cells.len(), 2, "the primitive unit holds two tiles");
        assert_eq!(t.lattice[0], v2(0.5, 1.5 * s3()));
        assert_eq!(t.lattice[1], v2(-6.5, 0.5 * s3()));
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        let a0 = t.cells[0].poly.area();
        assert!((det - 2.0 * a0).abs() < 1e-11, "det {det} ≠ 2 × tile area");
        assert!((det - 10.0 * s3()).abs() < 1e-11, "det {det} ≠ 10√3");
        assert!((a0 - tile_area()).abs() < 1e-11, "tile area ≠ 5√3");
        // Oblique, not rectangular/hexagonal: w1·w2 ≠ 0, |w1| ≠ |w2|.
        assert!(w1.dot(w2).abs() > 1e-9, "the lattice must be oblique");
        assert!((w1.dot(w1) - 7.0).abs() < 1e-12, "|w1|² = 7");
        assert!((w2.dot(w2) - 43.0).abs() < 1e-12, "|w2|² = 43");
        // The prototile closes: edge directions 0°, 60°, 120°, 180° of
        // lengths 2, 2, 1, 4; closing edge E→A has direction 300°, length 3.
        let pr = prototile();
        assert!((pr[4].x + 1.5).abs() < 1e-12, "closure E.x = −3/2, got {}", pr[4].x);
        assert!((pr[4].y - 1.5 * s3()).abs() < 1e-12, "closure E.y = 3√3/2, got {}", pr[4].y);
        let proto = &t.cells[0].poly;
        assert_eq!(proto.pts[0], v2(0.0, 0.0), "prototile corner A at the origin");
        for (i, want) in [(0usize, 120.0f64), (1, 120.0), (2, 120.0), (3, 120.0), (4, 60.0)] {
            let got = angle_at(proto, i).to_degrees();
            assert!((got - want).abs() < 1e-9, "angle at vertex {i}: {got} ≠ {want}");
        }
        for (i, want) in [(0usize, 2.0f64), (1, 2.0), (2, 1.0), (3, 4.0), (4, 3.0)] {
            assert!((side_of(proto, i) - want).abs() < 1e-12, "side {i}");
        }
        // Type-3 family identities: a = b, d = c + e.
        assert!((side_of(proto, 0) - side_of(proto, 1)).abs() < 1e-12, "a = b");
        assert!((side_of(proto, 3) - (side_of(proto, 2) + side_of(proto, 4))).abs() < 1e-12, "d = c + e");
        // Single isohedral orbit: both seeds 0.
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 0], "1-isohedral seeds");
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        let mut seen: HashSet<Vec<(i64, i64)>> = HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            assert!((cell.poly.area() - a0).abs() < 1e-12, "cell {k} area");
            assert_eq!(cell.poly.len(), 5, "cell {k} vertex count");
            // Cyclic (side, angle) fingerprint against the prototile:
            // forward only — both cells carry the same chirality.
            let cs = (0..5).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..5).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd, "cell {k} is not a directly-congruent type-3 prototile");
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
        }
    }

    /// Window census (default parameters) and declared metadata: culling
    /// is the tile AABB intersecting the region (+1e-9), matching the
    /// independent Python emulation of the lattice.rs culling.
    #[test]
    fn pentagon_3_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::P2);
        assert_eq!(t.lattice_kind, LatticeKind::Oblique);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the cell-0 tile (seed 0)");
        let p = ParamSet::default_for(t.params());
        for (region, want) in [
            (BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)), 19),
            (BBox::new(v2(-6.0, -4.0), v2(8.0, 6.0)), 26),
            (BBox::new(v2(-8.0, -6.0), v2(10.0, 8.0)), 44),
            (BBox::new(v2(-10.0, -8.0), v2(14.0, 10.0)), 70),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }

    /// Refined edge census over 60 tiles (the tiling is NON-edge-to-edge
    /// at the pentagon level): of the sides whose midpoint lies strictly
    /// inside the window (−9.4, 13.4) × (−7.4, 9.4), 172 are matched by
    /// exactly one full side and 46 are d-sides split at their exact
    /// midpoint t = 1/2 into two length-2 halves, each contained in
    /// exactly one length-2 partner side (an a- or b-side); no side
    /// carries any other T-junction.  The window is an exact count with a
    /// minimum boundary margin of 0.0388 from any counted midpoint
    /// (verified against the exact Q(√3) tiling), so it is robust against
    /// the ~1e-15 float difference between the runtime-computed prototile
    /// and the exact surds.
    #[test]
    fn pentagon_3_refined_edge_census() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let region = BBox::new(v2(-9.4, -7.4), v2(13.4, 9.4));
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 60, "patch size");
        let verts: Vec<Vec<Vec2>> = r.tiles.iter().map(|tile| tile.poly.pts.clone()).collect();
        let n = verts.len();
        let mut n_split = 0;
        let mut n_full = 0;
        let mut bad: Vec<String> = Vec::new();
        for i in 0..n {
            for e in 0..5 {
                let (pp, qq) = (verts[i][e], verts[i][(e + 1) % 5]);
                let mid = (pp + qq) * 0.5;
                if !(mid.x > -9.4 && mid.x < 13.4 && mid.y > -7.4 && mid.y < 9.4) {
                    continue;
                }
                let dv = qq - pp;
                let l2 = dv.dot(dv);
                let l = l2.sqrt();
                // Split points: other tiles' vertices strictly inside the
                // open segment (collinear within 1e-7), binned at 1e-9 and
                // merged within 1e-8.
                let mut bins: BTreeSet<i64> = BTreeSet::new();
                for (j, vj) in verts.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    for &vv in vj.iter() {
                        let tt = (vv - pp).dot(dv) / l2;
                        if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                            continue;
                        }
                        let cross = (vv.x - pp.x) * dv.y - (vv.y - pp.y) * dv.x;
                        if cross.abs() / l < 1e-7 {
                            bins.insert((tt * 1e9).round() as i64);
                        }
                    }
                }
                let mut srt: Vec<i64> = Vec::new();
                for b in bins {
                    if srt.is_empty() || b - srt[srt.len() - 1] > 10 {
                        srt.push(b);
                    }
                }
                if srt.len() > 1 {
                    bad.push(format!("tile {i} edge {e}: {srt:?}"));
                    continue;
                }
                let mut ends: Vec<f64> = vec![0.0];
                ends.extend(srt.iter().map(|b| *b as f64 / 1e9));
                ends.push(1.0);
                for w in ends.windows(2) {
                    let aq = pp + dv * w[0];
                    let seg = dv * (w[1] - w[0]);
                    let lseg = seg.length();
                    let uh = seg * (1.0 / lseg);
                    let mut cnt = 0;
                    for (j, vj) in verts.iter().enumerate() {
                        if j == i {
                            continue;
                        }
                        for (a1, b1) in (0..5).map(|k| (vj[k], vj[(k + 1) % 5])) {
                            let c1 = (a1.x - aq.x) * uh.y - (a1.y - aq.y) * uh.x;
                            let c2 = (b1.x - aq.x) * uh.y - (b1.y - aq.y) * uh.x;
                            if c1.abs() > 1e-7 || c2.abs() > 1e-7 {
                                continue;
                            }
                            let ta = (a1 - aq).dot(uh);
                            let tb = (b1 - aq).dot(uh);
                            if ta.min(tb) <= 1e-7 * lseg && ta.max(tb) >= lseg * (1.0 - 1e-7) {
                                cnt += 1;
                            }
                        }
                    }
                    if cnt != 1 {
                        bad.push(format!(
                            "tile {i} edge {e} sub [{:.3}, {:.3}] has {cnt} covering edges",
                            w[0], w[1]
                        ));
                    }
                }
                if srt.len() == 1 {
                    n_split += 1;
                    assert!((l - 4.0).abs() < 1e-9, "split side must be the d-side, got {l}");
                    assert!(
                        (srt[0] as f64 / 1e9 - 0.5).abs() < 1e-6,
                        "d-side must split at its midpoint, got {srt:?}"
                    );
                } else {
                    n_full += 1;
                }
            }
        }
        assert!(bad.is_empty(), "refined edge census violations: {bad:?}");
        assert_eq!(n_full, 172, "full (unsplit) edges");
        assert_eq!(n_split, 46, "d-sides split at their midpoint");
    }

    /// Vertex-figure census over the window (−9.4, −7.4)–(13.4, 9.4):
    /// every interior vertex (margin 1e-3 from the window edge; the
    /// tightest counted midpoint in this window sits 0.0388 away, so the
    /// scan is boundary-safe) is a complete star — corner rays plus rays
    /// of edges passing through it (the T-junction points) — merging
    /// within 1e-4 rad, and its sectors are exactly the two figures
    /// {60°, 120°, 180°} (the A- and E-type points, 46 of them) and
    /// {120°, 120°, 120°} (the B-, C- and D-type points, 39 of them —
    /// the B- and D-classes coincide mod Λ); 85 vertices in total.
    #[test]
    fn pentagon_3_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let big = BBox::new(v2(-9.4, -7.4), v2(13.4, 9.4));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        const MARG: f64 = 1e-3;
        let mut pts: Vec<Vec2> = Vec::new();
        for tile in &r.tiles {
            for q in &tile.poly.pts {
                if q.x > big.min.x + MARG
                    && q.x < big.max.x - MARG
                    && q.y > big.min.y + MARG
                    && q.y < big.max.y - MARG
                    && !pts.iter().any(|o| (*o - *q).length() < 1e-5)
                {
                    pts.push(*q);
                }
            }
        }
        let figs: [(u32, u32, u32); 2] = [(60, 120, 180), (120, 120, 120)];
        let mut counts: BTreeMap<(u32, u32, u32), usize> = BTreeMap::new();
        for pt in &pts {
            let mut rays: BTreeSet<i64> = BTreeSet::new();
            for tile in &r.tiles {
                let nn = tile.poly.len();
                for k in 0..nn {
                    if (tile.poly.pts[k] - *pt).length() >= 1e-5 {
                        continue;
                    }
                    for &q in [&tile.poly.pts[(k + nn - 1) % nn], &tile.poly.pts[(k + 1) % nn]] {
                        let dir = q - *pt;
                        let th = dir.y.atan2(dir.x).rem_euclid(std::f64::consts::TAU);
                        rays.insert((th * 1e5).round() as i64);
                    }
                }
                for e in 0..nn {
                    let (a, b) = (tile.poly.pts[e], tile.poly.pts[(e + 1) % nn]);
                    let ab = b - a;
                    let l2 = ab.dot(ab);
                    let tt = (*pt - a).dot(ab) / l2;
                    if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                        continue;
                    }
                    if (*pt - a).cross(ab).abs() / l2.sqrt() < 1e-7 {
                        let th0 = ab.y.atan2(ab.x);
                        for add in [0.0, std::f64::consts::PI] {
                            let th = (th0 + add).rem_euclid(std::f64::consts::TAU);
                            rays.insert((th * 1e5).round() as i64);
                        }
                    }
                }
            }
            let mut mr: Vec<f64> = Vec::new();
            for k in &rays {
                let rv = *k as f64 / 1e5;
                if mr.is_empty() || rv - mr[mr.len() - 1] > 1e-4 {
                    mr.push(rv);
                }
            }
            if mr.len() > 1 && mr[mr.len() - 1] - mr[0] > std::f64::consts::TAU - 1e-4 {
                mr.pop();
            }
            let mut gaps: Vec<f64> = (0..mr.len())
                .map(|i| (mr[(i + 1) % mr.len()] - mr[i]).rem_euclid(std::f64::consts::TAU))
                .collect();
            gaps.retain(|g| *g > 1e-4);
            gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let fig: Vec<u32> = gaps.iter().map(|g| g.to_degrees().round() as u32).collect();
            assert_eq!(fig.len(), 3, "vertex {pt:?}: unexpected figure {fig:?}");
            assert_eq!(fig.iter().sum::<u32>(), 360, "vertex {pt:?} figure {fig:?}");
            let f3 = (fig[0], fig[1], fig[2]);
            assert!(figs.contains(&f3), "unexpected vertex figure {f3:?} at {pt:?}");
            *counts.entry(f3).or_insert(0) += 1;
        }
        assert_eq!(pts.len(), 85, "total interior vertices");
        let want: BTreeMap<(u32, u32, u32), usize> =
            [((60, 120, 180), 46), ((120, 120, 120), 39)].into_iter().collect();
        assert_eq!(counts, want, "vertex-figure census");
    }

    /// The wallpaper group is p2: the half-turn about c0 = (−3/4, 3√3/4)
    /// (the midpoint of the prototile's d-side) preserves the tiling
    /// exactly; all four classes d/2 + Λ/2 are realised half-turn centres
    /// (their half-turns map the cell-0 tile onto tiles); the lattice
    /// translations are symmetries; and no mirror or glide is — the
    /// prototile is chiral (side cycle (2, 2, 1, 4, 3) differs from every
    /// rotation of its reversal) and the tiling uses one chirality only,
    /// so no orientation-reversing candidate (any mirror direction through
    /// any tile vertex or half-turn centre, with any whole lattice shift)
    /// maps the prototile onto a tile.  The half-turns about the lattice
    /// points themselves and about the half-lattice points off
    /// d/2 + Λ/2 are not symmetries.
    #[test]
    fn pentagon_3_p2_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let c0 = two_fold_centre();
        let ra = BBox::new(v2(c0.x - 12.0, c0.y - 10.0), v2(c0.x + 12.0, c0.y + 10.0));
        check_iso(&t, &p, (-1.0, 0.0, 0.0, -1.0), c0 * 2.0, ra, true, "half-turn about c0");
        // Big patch for tile-image membership: every probed image below
        // (cell-0 images under i, j ∈ [−2, 2] lattice shifts, half-turns
        // and mirror candidates centred within one cell of the origin)
        // lies inside this footprint, so membership in the generated set
        // is the exact question.
        let big = BBox::new(v2(-16.0, -14.0), v2(20.0, 16.0));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        let set: HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();
        for i in -2..=2 {
            // sanity: pure lattice translations are symmetries
            for &w in [&w1, &w2] {
                let img: Vec<Vec2> = prot.iter().map(|q| *q + w * i as f64).collect();
                assert!(set.contains(&vkey(&img)), "cell0 + {i}·w must be a tile");
            }
        }
        // The four half-turn classes d/2 + Λ/2: 2c ≡ d mod Λ where d = E−A
        // = cell_table()[1].1, so (M, 2c) is a symmetry for every such c.
        let _d = cell_table()[1].1;
        let halves = [v2(0.0, 0.0), w1 * 0.5, w2 * 0.5, (w1 + w2) * 0.5];
        for h in halves {
            let c = c0 + h;
            for i in -1..=1 {
                for j in -1..=1 {
                    let tr = c * 2.0 + w1 * i as f64 + w2 * j as f64;
                    let img: Vec<Vec2> = prot.iter().map(|q| apply_iso((-1.0, 0.0, 0.0, -1.0), tr, *q)).collect();
                    assert!(
                        set.contains(&vkey(&img)),
                        "half-turn about {:?} + lattice must map cell0 onto a tile",
                        c
                    );
                }
            }
        }
        // Half-turns about the lattice points and the half-lattice points
        // off d/2 + Λ/2 are not symmetries (their squares are lattice
        // translations, so they would have to be centres of the p2 group).
        for c in [Vec2::ZERO, w1 * 0.5, w2 * 0.5, (w1 + w2) * 0.5] {
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso((-1.0, 0.0, 0.0, -1.0), c * 2.0, *q)).collect();
            assert!(!set.contains(&vkey(&img)), "half-turn about {c:?} must not be a symmetry");
        }
        // No orientation-reversing isometry: for every mirror direction
        // through every tile-vertex candidate centre (tile vertices of the
        // cell tiles, the half-turn centres, and the lattice/half-lattice
        // points) and every whole lattice shift, the mirrored prototile is
        // not a tile.  The chiral side cycle makes this exhaustive: any
        // mirror or glide has a mirror line through one of the realisable
        // fixed-point candidates mod the lattice, and a one-chirality
        // tiling of a chiral prototile admits none.
        let e60 = v2(0.5, 0.5 * s3());
        let dirs = [Vec2::new(1.0, 0.0), e60, v2(0.5 * s3(), 0.5), Vec2::new(0.0, 1.0)];
        let centres = [
            Vec2::ZERO,
            prot[1],
            prot[2],
            prot[3],
            prot[4],
            c0,
            w1 * 0.5,
            w2 * 0.5,
            (w1 + w2) * 0.5,
        ];
        for u in dirs {
            let m = mirror(u);
            for c in centres {
                for i in -1..=1 {
                    for j in -1..=1 {
                        let mc = v2(m.0 * c.x + m.1 * c.y, m.2 * c.x + m.3 * c.y);
                        let tr = v2(c.x - mc.x + w1.x * i as f64 + w2.x * j as f64, c.y - mc.y + w1.y * i as f64 + w2.y * j as f64);
                        let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, tr, *q)).collect();
                        assert!(
                            !set.contains(&vkey(&img)),
                            "mirror ({u:?} through {c:?} + {i}·w1 + {j}·w2) must not be a symmetry"
                        );
                    }
                }
            }
        }
    }

    /// 1-isohedral: both cells carry cell seed 0 (the shipped pattern
    /// seed is the decorative checkerboard lattice parity, not an orbit
    /// class) and the half-turn about c0 permutes the cell table mod
    /// lattice as the transposition (0 1), so the two cells form a
    /// single full-group orbit; every generated tile keeps the prototile
    /// side multiset {1, 2, 2, 3, 4}.
    #[test]
    fn pentagon_3_isohedral_single_orbit() {
        let t = spec();
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 0], "single isohedral class");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        let u_of = |p: Vec2| {
            v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
        };
        let mod1 = |x: f64| (x + 0.5).rem_euclid(1.0) - 0.5;
        let table = cell_table();
        let c0 = two_fold_centre();
        // The half-turn maps each cell's anchor to the other cell's anchor
        // mod lattice, rotating the tile by 180° (both chiralities equal:
        // the table has no mirror).
        let mut perm: Vec<usize> = Vec::new();
        for k in 0..2 {
            let (_fam_k, a_k, th_k) = table[k];
            let ai = apply_iso((-1.0, 0.0, 0.0, -1.0), c0 * 2.0, a_k);
            let th_img = 180.0 + th_k;
            let mut hit = None;
            for (j, &(_fam_j, a_j, th_j)) in table.iter().enumerate() {
                let du = u_of(ai - a_j);
                if mod1(du.x).abs() < 1e-6
                    && mod1(du.y).abs() < 1e-6
                    && ((th_img - th_j + 180.0).rem_euclid(360.0) - 180.0).abs() < 1e-6
                {
                    hit = Some(j);
                    break;
                }
            }
            perm.push(hit.unwrap_or_else(|| panic!("cell {k} has no half-turn image")));
        }
        assert_eq!(perm, vec![1, 0], "half-turn permutes the cells as (0 1)");
        // Generated patches: the single orbit carries no orbit seed, so
        // the shipped seed is the decorative checkerboard lattice parity
        // (0..1); prototile side multiset kept.
        let params = ParamSet::default_for(t.params());
        let rp = t.generate(&params, &Query { region: BBox::new(v2(-6.0, -6.0), v2(10.0, 10.0)) });
        assert!(rp.message.is_none(), "{:?}", rp.message);
        for tile in &rp.tiles {
            assert!(tile.seed < 2, "checkerboard seed must stay in {{0, 1}}");
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length())
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [1.0, 2.0, 2.0, 3.0, 4.0];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "side {got} ∉ {{1, 2, 2, 3, 4}}");
            }
        }
    }

    /// M112: the two cells of the primitive cell are one orbit
    /// (1-isohedral), so the shipped seed is the decorative
    /// `SeedPattern::Checker` lattice parity — a checkerboard over the
    /// oblique lattice basis, not an orbit class; a bounded view must
    /// show both parities (seeds stay in {0, 1}).
    #[test]
    fn pentagon_3_seed_is_decorative_checkerboard() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region: BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let mut seen = [false; 2];
        for tile in &r.tiles {
            assert!(tile.seed < 2, "checkerboard seed must stay in {{0, 1}}");
            seen[tile.seed as usize] = true;
        }
        assert!(seen[0] && seen[1], "both lattice-parity classes must appear");
    }
}

#[cfg(all(test, feature = "pentagon2"))]
mod pentagon2_tests {
    use super::pentagon2::{cell_table, prototile, s3, tile_area, two_fold_centre};
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon2::spec;
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Reflection matrix in the direction u (u must be unit): 2uuᵀ − I.
    fn mirror(u: Vec2) -> (f64, f64, f64, f64) {
        (2.0 * u.x * u.x - 1.0, 2.0 * u.x * u.y, 2.0 * u.x * u.y, 2.0 * u.y * u.y - 1.0)
    }

    /// Vertex-set key on a 1e-6 grid (shared joints agree to ~1e-15 between
    /// two computations of the same tile, since every cell is built from
    /// the same runtime prototile array).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    /// True when the tile's AABB lies inside `w`.  A tile whose AABB is
    /// inside `ra` has its polygon inside `ra`, so its image under any
    /// isometry g intersects g(ra) ⊆ bbox(g(ra)) — and a tile whose polygon
    /// intersects the region always lies inside the generator's padded
    /// index range — hence the image is always generated for rb.
    fn aabb_inside(poly: &Polygon, w: BBox) -> bool {
        let bb = poly.aabb();
        bb.min.x >= w.min.x
            && bb.min.y >= w.min.y
            && bb.max.x <= w.max.x
            && bb.max.y <= w.max.y
    }

    /// Verify that the isometry g = (M, t) preserves the tiling on `ra`:
    /// images of interior-AABB tiles of ra must be tiles generated for the
    /// image window bbox(g(ra)).  With `exact = true` (the half-turn case,
    /// g(ra) = ra) the filtered image set must equal the interior-AABB
    /// tile set of the window.
    fn check_iso(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            v2(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            v2(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ta = t.generate(params, &Query { region: ra });
        let tb = t.generate(params, &Query { region: rb });
        assert!(ta.message.is_none(), "{label}: {:?}", ta.message);
        assert!(tb.message.is_none(), "{label}: {:?}", tb.message);
        let src: Vec<_> = ta.tiles.iter().filter(|tile| aabb_inside(&tile.poly, ra)).collect();
        assert!(!src.is_empty(), "{label}: no interior tiles");
        let set_b: HashSet<Vec<(i64, i64)>> = tb
            .tiles
            .iter()
            .filter(|tile| !exact || aabb_inside(&tile.poly, rb))
            .map(|tile| vkey(&tile.poly.pts))
            .collect();
        let set_a: HashSet<Vec<(i64, i64)>> = src
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(set_a.len(), src.len(), "{label}: isometry must be injective on the patch");
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    #[test]
    fn pentagon_2_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "pentagon-2", &p, BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)));
    }

    #[test]
    fn pentagon_2_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-2(rot30,scale0.7)",
            BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_2_deterministic() {
        check_det(&spec(), "pentagon-2", BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)));
    }

    /// Unit structure of the primitive cell: 4 congruent copies of the
    /// exact-Q(√3) type-2 prototile (A = B = D = 90°, C = 120°, E = 150°,
    /// a = 1, b = d = 1 + √3/2, c = e = 1/2, tile area 1 + √3/2) on the
    /// rectangular lattice w1 = (1/2, 1 + √3/2), w2 = (−2 − √3, 1) with
    /// w1 ⊥ w2 and det = 4 + 2√3 = 4 × tile area.  Both chiralities occur
    /// (two direct, two mirror cells), the seeds mark the single
    /// isohedral orbit, and every cell closes with no duplication.
    #[test]
    fn pentagon_2_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-2");
        assert_eq!(t.cells.len(), 4, "the primitive unit holds four tiles");
        assert_eq!(t.lattice[0], v2(0.5, 1.0 + s3() / 2.0));
        assert_eq!(t.lattice[1], v2(-2.0 - s3(), 1.0));
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        let a0 = t.cells[0].poly.area();
        assert!((det - 4.0 * a0).abs() < 1e-11, "det {det} ≠ 4 × tile area");
        assert!((det - (4.0 + 2.0 * s3())).abs() < 1e-11, "det {det} ≠ 4 + 2√3");
        assert!((a0 - tile_area()).abs() < 1e-11, "tile area ≠ 1 + √3/2");
        // Rectangular: w1 ⊥ w2, |w1|² = 2 + √3, |w2|² = 8 + 4√3.
        assert!(w1.dot(w2).abs() < 1e-12, "the lattice must be rectangular");
        assert!((w1.dot(w1) - (2.0 + s3())).abs() < 1e-12, "|w1|² = 2 + √3");
        assert!((w2.dot(w2) - (8.0 + 4.0 * s3())).abs() < 1e-12, "|w2|² = 8 + 4√3");
        // The prototile closes: edge directions 0°, 90°, 150°, 240° of
        // lengths b = 1 + √3/2, 1/2, b, 1/2; closing edge E→A has
        // direction 270°, length a = 1.
        let s = 1.0 + s3() / 2.0;
        let pr = prototile();
        assert!(pr[4].x.abs() < 1e-12, "closure E.x = 0, got {}", pr[4].x);
        assert!((pr[4].y - 1.0).abs() < 1e-12, "closure E.y = 1, got {}", pr[4].y);
        let proto = &t.cells[0].poly;
        assert_eq!(proto.pts[0], v2(0.0, 0.0), "prototile corner A at the origin");
        for (i, want) in [(0usize, 90.0f64), (1, 90.0), (2, 120.0), (3, 90.0), (4, 150.0)] {
            let got = angle_at(proto, i).to_degrees();
            assert!((got - want).abs() < 1e-9, "angle at vertex {i}: {got} ≠ {want}");
        }
        for (i, want) in [(0usize, s), (1, 0.5), (2, s), (3, 0.5), (4, 1.0)] {
            assert!((side_of(proto, i) - want).abs() < 1e-12, "side {i}");
        }
        // Type-2 family identities: c = e, d = b, B + D = 180°.
        assert!((side_of(proto, 1) - side_of(proto, 3)).abs() < 1e-12, "c = e");
        assert!((side_of(proto, 0) - side_of(proto, 2)).abs() < 1e-12, "d = b");
        let b_angle = angle_at(proto, 1).to_degrees();
        let d_angle = angle_at(proto, 3).to_degrees();
        assert!((b_angle + d_angle - 180.0).abs() < 1e-9, "B + D = 180°");
        // Chirality seed classes: the single isohedral orbit carries
        // the two chirality classes — direct pair 0, mirror pair 1.
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 0, 1, 1], "chirality seeds: direct 0, mirror 1");
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        // The mirrored prototile's CCW (side, angle) cycle, for the
        // reverse-cyclic fingerprint of the mirror cells.
        let ps_m: Vec<f64> = (0..5).map(|i| ps[(4 - i) % 5]).collect();
        let pa_m: Vec<f64> = (0..5).map(|i| pa[(5 - i) % 5]).collect();
        let mut seen: HashSet<Vec<(i64, i64)>> = HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            assert!((cell.poly.area() - a0).abs() < 1e-12, "cell {k} area");
            assert_eq!(cell.poly.len(), 5, "cell {k} vertex count");
            // Cyclic (side, angle) fingerprint against the prototile,
            // forward (direct cells) or reversed (mirror cells).
            let cs = (0..5).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..5).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            let rev = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps_m[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa_m[(i + rot) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd || rev, "cell {k} is not a congruent type-2 prototile");
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
        }
        // Both chiralities occur: two direct and two mirror cells.
        let fams: Vec<bool> = cell_table().iter().map(|c| c.0).collect();
        assert_eq!(fams.iter().filter(|f| **f).count(), 2, "two chiral pairs in the cell");
    }

    /// Window census (default parameters) and declared metadata: culling
    /// is the tile AABB intersecting the region (+1e-9), matching the
    /// independent Python emulation of the lattice.rs culling.
    #[test]
    fn pentagon_2_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::Pgg);
        assert_eq!(t.lattice_kind, LatticeKind::Rectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the cell-0 tile (seed 0)");
        let p = ParamSet::default_for(t.params());
        for (region, want) in [
            (BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)), 115),
            (BBox::new(v2(-8.0, -6.0), v2(10.0, 8.0)), 165),
            (BBox::new(v2(-10.0, -8.0), v2(14.0, 10.0)), 272),
            (
                BBox::new(
                    v2(-8.464101615137753, -8.165063509461095),
                    v2(16.678203230275507, 9.464101615137753),
                ),
                281,
            ),
            (
                BBox::new(v2(0.0, -1.4330127018922192), v2(3.982050807568877, 1.8660254037844386)),
                17,
            ),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }

    /// Refined edge census: the tiling is EDGE-TO-EDGE at the pentagon
    /// level.  Of the sides whose midpoint lies strictly inside the
    /// window (36-cell block inset by 0.05; minimum midpoint boundary
    /// margin 0.00144), every one of the 1182 is matched by exactly one
    /// full side of one neighbour, and no side carries a T-junction
    /// (zero split edges).  The window is an exact count, so it is robust
    /// against the ~1e-15 float difference between the runtime-computed
    /// prototile and the exact surds.
    #[test]
    fn pentagon_2_refined_edge_census() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let block = BBox::new(
            v2(-8.464101615137753, -8.165063509461095),
            v2(16.678203230275507, 9.464101615137753),
        );
        let region = BBox::new(block.min + v2(0.05, 0.05), block.max - v2(0.05, 0.05));
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 276, "patch size");
        let verts: Vec<Vec<Vec2>> = r.tiles.iter().map(|tile| tile.poly.pts.clone()).collect();
        let n = verts.len();
        let mut n_split = 0;
        let mut n_full = 0;
        let mut bad: Vec<String> = Vec::new();
        for i in 0..n {
            for e in 0..5 {
                let (pp, qq) = (verts[i][e], verts[i][(e + 1) % 5]);
                let mid = (pp + qq) * 0.5;
                if !(mid.x > region.min.x
                    && mid.x < region.max.x
                    && mid.y > region.min.y
                    && mid.y < region.max.y)
                {
                    continue;
                }
                let dv = qq - pp;
                let l2 = dv.dot(dv);
                // Split points: other tiles' vertices strictly inside the
                // open segment (collinear within 1e-7), binned at 1e-9 and
                // merged within 1e-8.
                let mut bins: BTreeSet<i64> = BTreeSet::new();
                for (j, vj) in verts.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    for &vv in vj.iter() {
                        let tt = (vv - pp).dot(dv) / l2;
                        if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                            continue;
                        }
                        let cross = (vv.x - pp.x) * dv.y - (vv.y - pp.y) * dv.x;
                        if cross.abs() / l2.sqrt() < 1e-7 {
                            bins.insert((tt * 1e9).round() as i64);
                        }
                    }
                }
                let mut srt: Vec<i64> = Vec::new();
                for b in bins {
                    if srt.is_empty() || b - srt[srt.len() - 1] > 10 {
                        srt.push(b);
                    }
                }
                let mut ends: Vec<f64> = vec![0.0];
                ends.extend(srt.iter().map(|b| *b as f64 / 1e9));
                ends.push(1.0);
                for w in ends.windows(2) {
                    let aq = pp + dv * w[0];
                    let seg = dv * (w[1] - w[0]);
                    let lseg = seg.length();
                    let uh = seg * (1.0 / lseg);
                    let mut cnt = 0;
                    for (j, vj) in verts.iter().enumerate() {
                        if j == i {
                            continue;
                        }
                        for (a1, b1) in (0..5).map(|k| (vj[k], vj[(k + 1) % 5])) {
                            let c1 = (a1.x - aq.x) * uh.y - (a1.y - aq.y) * uh.x;
                            let c2 = (b1.x - aq.x) * uh.y - (b1.y - aq.y) * uh.x;
                            if c1.abs() > 1e-7 || c2.abs() > 1e-7 {
                                continue;
                            }
                            let ta = (a1 - aq).dot(uh);
                            let tb = (b1 - aq).dot(uh);
                            if ta.min(tb) <= 1e-7 * lseg && ta.max(tb) >= lseg * (1.0 - 1e-7) {
                                cnt += 1;
                            }
                        }
                    }
                    if cnt != 1 {
                        bad.push(format!(
                            "tile {i} edge {e} sub [{:.3}, {:.3}] has {cnt} covering edges",
                            w[0], w[1]
                        ));
                    }
                }
                if srt.len() == 1 {
                    n_split += 1;
                    bad.push(format!("tile {i} edge {e}: split at {srt:?} (tiling is edge-to-edge)"));
                } else {
                    n_full += 1;
                }
            }
        }
        assert!(bad.is_empty(), "refined edge census violations: {bad:?}");
        assert_eq!(n_full, 1182, "full (unsplit) edges");
        assert_eq!(n_split, 0, "no split sides");
    }

    /// Vertex-figure census over the 36-cell block window: every interior
    /// vertex (margin 1e-3 from the window edge; the star-to-boundary
    /// margin is 0.067, so the scan is boundary-safe) is a complete star
    /// with sectors exactly {90°, 120°, 150°} (237 of them) or
    /// {90°, 90°, 90°, 90°} (118 of them); 355 vertices in total.  The
    /// edge-through-ray pass runs as a T-junction detector: because the
    /// tiling is edge-to-edge no edge passes through a vertex, so that
    /// pass contributes no rays here — a T-junction would inject a 180°
    /// sector and fail the figure check.
    #[test]
    fn pentagon_2_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let big = BBox::new(
            v2(-8.464101615137753, -8.165063509461095),
            v2(16.678203230275507, 9.464101615137753),
        );
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        const MARG: f64 = 1e-3;
        let mut pts: Vec<Vec2> = Vec::new();
        for tile in &r.tiles {
            for q in &tile.poly.pts {
                if q.x > big.min.x + MARG
                    && q.x < big.max.x - MARG
                    && q.y > big.min.y + MARG
                    && q.y < big.max.y - MARG
                    && !pts.iter().any(|o| (*o - *q).length() < 1e-5)
                {
                    pts.push(*q);
                }
            }
        }
        // 3-ray stars pad with a trailing zero: (90, 120, 150, 0).
        let figs = [(90u32, 120, 150, 0), (90, 90, 90, 90)];
        let mut counts: BTreeMap<(u32, u32, u32, u32), usize> = BTreeMap::new();
        for pt in &pts {
            let mut rays: BTreeSet<i64> = BTreeSet::new();
            for tile in &r.tiles {
                let nn = tile.poly.len();
                for k in 0..nn {
                    if (tile.poly.pts[k] - *pt).length() >= 1e-5 {
                        continue;
                    }
                    for &q in [&tile.poly.pts[(k + nn - 1) % nn], &tile.poly.pts[(k + 1) % nn]] {
                        let dir = q - *pt;
                        let th = dir.y.atan2(dir.x).rem_euclid(std::f64::consts::TAU);
                        rays.insert((th * 1e5).round() as i64);
                    }
                }
                for e in 0..nn {
                    let (a, b) = (tile.poly.pts[e], tile.poly.pts[(e + 1) % nn]);
                    let ab = b - a;
                    let l2 = ab.dot(ab);
                    let tt = (*pt - a).dot(ab) / l2;
                    if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                        continue;
                    }
                    if (*pt - a).cross(ab).abs() / l2.sqrt() < 1e-7 {
                        let th0 = ab.y.atan2(ab.x);
                        for add in [0.0, std::f64::consts::PI] {
                            let th = (th0 + add).rem_euclid(std::f64::consts::TAU);
                            rays.insert((th * 1e5).round() as i64);
                        }
                    }
                }
            }
            let mut mr: Vec<f64> = Vec::new();
            for k in &rays {
                let rv = *k as f64 / 1e5;
                if mr.is_empty() || rv - mr[mr.len() - 1] > 1e-4 {
                    mr.push(rv);
                }
            }
            if mr.len() > 1 && mr[mr.len() - 1] - mr[0] > std::f64::consts::TAU - 1e-4 {
                mr.pop();
            }
            let mut gaps: Vec<f64> = (0..mr.len())
                .map(|i| (mr[(i + 1) % mr.len()] - mr[i]).rem_euclid(std::f64::consts::TAU))
                .collect();
            gaps.retain(|g| *g > 1e-4);
            gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let fig: Vec<u32> = gaps.iter().map(|g| g.to_degrees().round() as u32).collect();
            assert!(
                fig.len() == 3 || fig.len() == 4,
                "vertex {pt:?}: unexpected figure {fig:?}"
            );
            assert_eq!(fig.iter().sum::<u32>(), 360, "vertex {pt:?} figure {fig:?}");
            let f4 = (
                fig.first().copied().unwrap_or(0),
                fig.get(1).copied().unwrap_or(0),
                fig.get(2).copied().unwrap_or(0),
                fig.get(3).copied().unwrap_or(0),
            );
            assert!(figs.contains(&f4), "unexpected vertex figure {f4:?} at {pt:?}");
            *counts.entry(f4).or_insert(0) += 1;
        }
        assert_eq!(pts.len(), 355, "total interior vertices");
        let want: BTreeMap<(u32, u32, u32, u32), usize> =
            [((90, 120, 150, 0), 237), ((90, 90, 90, 90), 118)].into_iter().collect();
        assert_eq!(counts, want, "vertex-figure census");
    }

    /// The wallpaper group is pgg: the half-turn about c0 = (1 + √3/2, 0)
    /// (the prototile's B vertex) preserves the tiling exactly; all four
    /// classes c0 + Λ/2 are realised half-turn centres (their half-turns
    /// map the cell-0 tile onto tiles); the two glide families (axes along
    /// the lattice directions, g1² = w1, g2² = −w2, composing to the
    /// half-turn about c0) and their lattice shifts map the prototile onto
    /// tiles; and no pure reflection — the perpendicular-part candidates
    /// on the glide axes, nor any whole-lattice-shift candidate — maps the
    /// prototile onto a tile.
    #[test]
    fn pentagon_2_pgg_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let c0 = two_fold_centre();
        let ra = BBox::new(v2(c0.x - 12.0, c0.y - 10.0), v2(c0.x + 12.0, c0.y + 10.0));
        check_iso(&t, &p, (-1.0, 0.0, 0.0, -1.0), c0 * 2.0, ra, true, "half-turn about c0");
        let u1 = w1 * (1.0 / w1.length());
        let u2 = w2 * (1.0 / w2.length());
        let m1 = mirror(u1);
        let m2 = mirror(u2);
        let t1 = cell_table()[2].1; // the 150° mirror pair's anchor
        let t2 = cell_table()[3].1; // the 330° mirror pair's anchor
        // Exact generator identities: g² = (I + M)·t.
        let gs1 = v2((m1.0 + 1.0) * t1.x + m1.1 * t1.y, m1.2 * t1.x + (m1.3 + 1.0) * t1.y);
        assert!((gs1 - w1).length() < 1e-9, "g1² must equal w1");
        let gs2 = v2((m2.0 + 1.0) * t2.x + m2.1 * t2.y, m2.2 * t2.x + (m2.3 + 1.0) * t2.y);
        assert!((gs2 + w2).length() < 1e-9, "g2² must equal −w2");
        // Each glide shifts by exactly half a primitive translation along
        // its axis — g1 by +1/2 of w1, g2 by −1/2 of w2 — so no power of
        // it is a pure reflection.
        let half1 = 2.0 * t1.dot(u1) / w1.length();
        assert!((half1 - 1.0).abs() < 1e-6, "g1 must shift half a period");
        let half2 = 2.0 * t2.dot(u2) / w2.length();
        assert!((half2 + 1.0).abs() < 1e-6, "g2 must shift half a period");
        // g2∘g1 is the half-turn centred at c0 itself.
        let comp = v2(t2.x + m2.0 * t1.x + m2.1 * t1.y, t2.y + m2.2 * t1.x + m2.3 * t1.y);
        assert!((comp * 0.5 - c0).length() < 1e-9, "centre(g2∘g1) must be c0");
        // The glides preserve the tiling (the image window is tilted, so
        // check containment of the images among the tiles generated there)
        // — and so do the inverses and the lattice-shifted variants.
        check_iso(&t, &p, m1, t1, ra, false, "glide g1 (axis ∥ w1)");
        check_iso(
            &t,
            &p,
            m1,
            v2(-(m1.0 * t1.x + m1.1 * t1.y), -(m1.2 * t1.x + m1.3 * t1.y)),
            ra,
            false,
            "glide g1⁻¹",
        );
        check_iso(&t, &p, m2, t2, ra, false, "glide g2 (axis ∥ w2)");
        check_iso(
            &t,
            &p,
            m2,
            v2(-(m2.0 * t2.x + m2.1 * t2.y), -(m2.2 * t2.x + m2.3 * t2.y)),
            ra,
            false,
            "glide g2⁻¹",
        );
        // Big patch for tile-image membership.
        let big = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        let set: HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();
        for i in -2..=2 {
            // sanity: pure lattice translations are symmetries
            for &w in [&w1, &w2] {
                let img: Vec<Vec2> = prot.iter().map(|q| *q + w * i as f64).collect();
                assert!(set.contains(&vkey(&img)), "cell0 + {i}·w must be a tile");
            }
        }
        // The four half-turn classes c0 + Λ/2: every such half-turn maps
        // the cell-0 tile onto a tile.
        let halves = [v2(0.0, 0.0), w1 * 0.5, w2 * 0.5, (w1 + w2) * 0.5];
        for h in halves {
            let c = c0 + h;
            for i in -1..=1 {
                for j in -1..=1 {
                    let tr = c * 2.0 + w1 * i as f64 + w2 * j as f64;
                    let img: Vec<Vec2> =
                        prot.iter().map(|q| apply_iso((-1.0, 0.0, 0.0, -1.0), tr, *q)).collect();
                    assert!(
                        set.contains(&vkey(&img)),
                        "half-turn about {c:?} + lattice must map cell0 onto a tile"
                    );
                }
            }
        }
        // Lattice-shifted glide variants.
        for (i, j) in [(0.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (1.0, 0.0)] {
            let sh = w1 * i + w2 * j;
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m1, t1 + sh, *q)).collect();
            assert!(
                set.contains(&vkey(&img)),
                "(g1 + {i}·w1 + {j}·w2) must map cell0 onto a tile"
            );
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m2, t2 + sh, *q)).collect();
            assert!(
                set.contains(&vkey(&img)),
                "(g2 + {i}·w1 + {j}·w2) must map cell0 onto a tile"
            );
        }
        // No pure reflection: the mirror-class candidates carrying only the
        // perpendicular glide part (a reflection would need zero along-axis
        // shift), and every whole-lattice-shift candidate, must fail to map
        // the prototile onto a tile.
        let p1 = t1 - u1 * t1.dot(u1);
        let p2 = t2 - u2 * t2.dot(u2);
        for (m, tr, nm) in [(m1, p1, "(M1, t1⊥)"), (m2, p2, "(M2, t2⊥)")] {
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, tr, *q)).collect();
            assert!(!set.contains(&vkey(&img)), "{nm} must not be a symmetry (no mirrors in pgg)");
        }
        for i in -2..=2 {
            for (m, w, mn, wn) in
                [(m1, w1, "M1", "w1"), (m1, w2, "M1", "w2"), (m2, w1, "M2", "w1"), (m2, w2, "M2", "w2")]
            {
                let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, w * i as f64, *q)).collect();
                assert!(
                    !set.contains(&vkey(&img)),
                    "({mn}, {i}·{wn}) must not be a symmetry (no mirrors in pgg)"
                );
            }
        }
    }

    /// 1-isohedral: the four cells carry the two chirality seed classes
    /// (direct pair seed 0, mirror pair seed 1), and the half-turn plus
    /// the two glide classes permute the cell table as a single geometric
    /// orbit — r = (0 1)(2 3), g1 = (0 2)(1 3), g2 = (0 3)(1 2), the
    /// glides doing the identifying across chiralities; every generated
    /// tile keeps the prototile side multiset {1/2, 1/2, 1, 1 + √3/2,
    /// 1 + √3/2} and its chirality seed.
    #[test]
    fn pentagon_2_isohedral_single_orbit() {
        let t = spec();
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 0, 1, 1], "chirality classes: direct pair 0, mirror pair 1");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        let u_of = |p: Vec2| {
            v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
        };
        let mod1 = |x: f64| (x + 0.5).rem_euclid(1.0) - 0.5;
        let table = cell_table();
        // Which cell does the isometry (M, tr) send each cell to?  The
        // image anchor must land on the target anchor mod lattice; a
        // direct image keeps the family and adds its rotation, a mirror
        // image flips the family and subtracts the mirror's axis angle
        // doubled from the rotation.
        let perm_of = |m: (f64, f64, f64, f64), tr: Vec2| -> Vec<usize> {
            let rot = m.2.atan2(m.0).to_degrees();
            let is_direct = m.0 * m.3 - m.1 * m.2 > 0.0;
            let mut perm: Vec<usize> = Vec::new();
            for k in 0..4 {
                let (fam_k, a_k, th_k) = table[k];
                let ai = apply_iso(m, tr, a_k);
                let (want_fam, th_img) = if is_direct {
                    (fam_k, rot + th_k)
                } else {
                    (!fam_k, rot - th_k)
                };
                let mut hit = None;
                for (j, &(fam_j, a_j, th_j)) in table.iter().enumerate() {
                    let du = u_of(ai - a_j);
                    if mod1(du.x).abs() < 1e-6
                        && mod1(du.y).abs() < 1e-6
                        && fam_j == want_fam
                        && ((th_img - th_j + 180.0).rem_euclid(360.0) - 180.0).abs() < 1e-6
                    {
                        hit = Some(j);
                        break;
                    }
                }
                perm.push(hit.unwrap_or_else(|| panic!("cell {k} has no image under the isometry")));
            }
            perm
        };
        let c0 = two_fold_centre();
        let r = perm_of((-1.0, 0.0, 0.0, -1.0), c0 * 2.0);
        assert_eq!(r, vec![1, 0, 3, 2], "half-turn permutes the cells as (0 1)(2 3)");
        let u1 = w1 * (1.0 / w1.length());
        let u2 = w2 * (1.0 / w2.length());
        let g1 = perm_of(mirror(u1), table[2].1);
        assert_eq!(g1, vec![2, 3, 0, 1], "glide g1 permutes the cells as (0 2)(1 3)");
        let g2 = perm_of(mirror(u2), table[3].1);
        assert_eq!(g2, vec![3, 2, 1, 0], "glide g2 permutes the cells as (0 3)(1 2)");
        // The three generators move cell 0 to every cell: one orbit.
        let mut orbit: BTreeSet<usize> = BTreeSet::new();
        let mut stack = vec![0usize];
        while let Some(k) = stack.pop() {
            if !orbit.insert(k) {
                continue;
            }
            for perm in [&r, &g1, &g2] {
                if !orbit.contains(&perm[k]) {
                    stack.push(perm[k]);
                }
            }
        }
        assert_eq!(orbit, BTreeSet::from([0usize, 1, 2, 3]), "single isohedral orbit");
        // Generated patches: the shipped coloring distinguishes the two
        // chiral classes — direct pair (proto 0, 1) seed 0, mirror pair
        // (proto 2, 3) seed 1 — and keeps the prototile side multiset.
        let params = ParamSet::default_for(t.params());
        let rp = t.generate(
            &params,
            &Query { region: BBox::new(v2(-6.0, -6.0), v2(10.0, 10.0)) },
        );
        assert!(rp.message.is_none(), "{:?}", rp.message);
        let s = 1.0 + s3() / 2.0;
        let mut seen = [false; 2];
        for tile in &rp.tiles {
            assert_eq!(tile.seed, u32::from(tile.proto >= 2), "tile kept its chirality seed");
            seen[tile.seed as usize] = true;
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length())
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [0.5, 0.5, 1.0, s, s];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "side {got} not in the prototile side multiset");
            }
        }
        assert!(seen[0] && seen[1], "both chiral seed classes must appear");
    }

    /// M112: the shipped coloring distinguishes the two chiralities —
    /// the direct chiral pair (cells 0, 1) carries seed 0, the mirror
    /// pair (cells 2, 3) carries seed 1 — and every generated tile
    /// keeps the seed class of its cell; a bounded view must show both
    /// classes.
    #[test]
    fn pentagon_2_seed_classes_track_chirality() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region: BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let mut seen = [false; 2];
        for tile in &r.tiles {
            let want = u32::from(tile.proto >= 2);
            assert_eq!(tile.seed, want, "proto {} must carry its cell's chirality seed", tile.proto);
            seen[tile.seed as usize] = true;
        }
        assert!(seen[0] && seen[1], "both chiral classes must appear");
    }
}

#[cfg(all(test, feature = "pentagon7"))]
mod pentagon7_tests {
    use super::pentagon7::{angles, closure_f, raw_cells, two_fold_centre, MIRROR};
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon7::spec;
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Reflection matrix in the direction u (u must be unit): 2uuᵀ − I.
    fn mirror(u: Vec2) -> (f64, f64, f64, f64) {
        (2.0 * u.x * u.x - 1.0, 2.0 * u.x * u.y, 2.0 * u.x * u.y, 2.0 * u.y * u.y - 1.0)
    }

    /// Vertex-set key on a 1e-6 grid (distinct tiling vertices are ≥ 0.5
    /// apart — the shortest side is 1/2 — and two computations of a shared
    /// joint agree to ~1e-15 because every cell is built from the same
    /// runtime prototile array).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    /// True when the tile's AABB lies inside `w`.  A tile whose AABB is
    /// inside `ra` has its polygon inside `ra`, so its image under any
    /// isometry g intersects g(ra) ⊆ bbox(g(ra)) — and a tile whose polygon
    /// intersects the region always lies inside the generator's padded
    /// index range — hence the image is always generated for rb.  Tiles
    /// whose AABB merely touches the boundary (polygon outside) are
    /// excluded: their images legitimately escape rb.
    fn aabb_inside(poly: &Polygon, w: BBox) -> bool {
        let bb = poly.aabb();
        bb.min.x >= w.min.x
            && bb.min.y >= w.min.y
            && bb.max.x <= w.max.x
            && bb.max.y <= w.max.y
    }

    /// Verify that the isometry g = (M, t) preserves the tiling on `ra`:
    /// images of interior-AABB tiles of ra must be tiles generated for the
    /// image window bbox(g(ra)).  With `exact = true` (the half-turn case,
    /// g(ra) = ra) the filtered image set must equal the interior-AABB
    /// tile set of the window.
    fn check_iso(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            v2(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            v2(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ta = t.generate(params, &Query { region: ra });
        let tb = t.generate(params, &Query { region: rb });
        assert!(ta.message.is_none(), "{label}: {:?}", ta.message);
        assert!(tb.message.is_none(), "{label}: {:?}", tb.message);
        let src: Vec<_> = ta.tiles.iter().filter(|tile| aabb_inside(&tile.poly, ra)).collect();
        assert!(!src.is_empty(), "{label}: no interior tiles");
        let set_b: HashSet<Vec<(i64, i64)>> = tb
            .tiles
            .iter()
            .filter(|tile| !exact || aabb_inside(&tile.poly, rb))
            .map(|tile| vkey(&tile.poly.pts))
            .collect();
        let set_a: HashSet<Vec<(i64, i64)>> = src
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(set_a.len(), src.len(), "{label}: isometry must be injective on the patch");
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    /// Centroid of a stored cell (mean of its five vertices).
    fn cent(pts: &[Vec2]) -> Vec2 {
        let s = pts.iter().fold(Vec2::ZERO, |acc, &p| acc + p);
        s / 5.0
    }

    #[test]
    fn pentagon_7_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "pentagon-7", &p, BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)));
    }

    #[test]
    fn pentagon_7_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-7(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(7.0, 7.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_7_deterministic() {
        check_det(&spec(), "pentagon-7", BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)));
    }

    /// Unit structure of the primitive cell: 8 congruent copies of the
    /// runtime-computed Kershner prototile (A = 58.8626451020303°,
    /// B = 148°, C = 132.8626451020303°, D = 94.2747097959394°,
    /// E = 106°, sides b = c = d = e = 1/2, a = 1.035794206008535) on the
    /// rectangular lattice w1 ⊥ w2 with det = 8 × tile area =
    /// 4.236326192143856.  The family identities hold to ~1e-12, every
    /// cell is CCW, no two cells coincide, every cell is exactly the
    /// applet construction (mirrors vertex-reversed), and the seeds follow
    /// the two full-group isohedral orbits.
    #[test]
    fn pentagon_7_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-7");
        assert_eq!(t.cells.len(), 8, "the primitive unit holds eight tiles");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        // The lattice is computed from the construction, not hardcoded:
        // w1 = cell2.p3 − cell0.p0 (both direct); w2 = −(cell5.p3 −
        // cell3.p1) (cells 3/5 are mirror-stored, so their raw p3/p1 sit
        // at stored indices 2/4).
        assert!(
            (t.lattice[0] - (t.cells[2].poly.pts[3] - t.cells[0].poly.pts[0])).length() < 1e-15,
            "w1 must be cell2.p3 − cell0.p0"
        );
        assert!(
            (t.lattice[1] + (t.cells[5].poly.pts[2] - t.cells[3].poly.pts[4])).length() < 1e-15,
            "w2 must be −(cell5.p3 − cell3.p1)"
        );
        assert!(w1.dot(w2).abs() < 1e-12, "lattice must be rectangular (w1 ⊥ w2)");
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        let a0 = t.cells[0].poly.area();
        assert!((det - 8.0 * a0).abs() < 1e-11, "det {det} ≠ 8 × tile area");
        assert!((det - 4.236326192143856).abs() < 1e-9, "det {det}");
        assert!((a0 - 0.529540774017982).abs() < 1e-9, "tile area {a0}");
        // The runtime-computed instance: angles in vertex order (A, B, C, D, E)
        // and the closure root.
        let ang = angles();
        for (got, want) in ang
            .iter()
            .zip([58.8626451020303, 148.0, 132.8626451020303, 94.2747097959394, 106.0].iter())
        {
            assert!((got - want).abs() < 1e-9, "angle {got} ≠ {want}");
        }
        assert!(closure_f(ang[2]).abs() < 1e-12, "the closure equation must hold at C");
        let [a, b, c, d, e] = ang;
        assert!((b + 2.0 * e - 360.0).abs() < 1e-12, "B + 2E = 360°");
        assert!((2.0 * c + d - 360.0).abs() < 1e-12, "2C + D = 360°");
        assert!((2.0 * a + b + d - 360.0).abs() < 1e-12, "2A + B + D = 360°");
        assert!((a - (c - b / 2.0)).abs() < 1e-12, "A = C − B/2");
        // The prototile (cell 0): sides b = c = d = e = 1/2, a =
        // 1.035794206008535; angles in vertex order.
        let proto = &t.cells[0].poly;
        for i in 0..4 {
            assert!((side_of(proto, i) - 0.5).abs() < 1e-15, "proto side {i} must be 1/2");
        }
        assert!((side_of(proto, 4) - 1.035794206008535).abs() < 1e-9, "proto side 4 = a");
        for (i, want) in [58.8626451020303, 148.0, 132.8626451020303, 94.2747097959394, 106.0]
            .iter()
            .enumerate()
        {
            let got = angle_at(proto, i).to_degrees();
            assert!((got - want).abs() < 1e-9, "proto angle at vertex {i}: {got} ≠ {want}");
        }
        // Seeds follow the two full-group isohedral orbits (each orbit
        // mixes both chiralities; the glides identify them).
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 1, 1, 0, 0, 1, 1, 0], "isohedral seed pattern");
        let a4 = side_of(proto, 4);
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        let raw = raw_cells();
        let mut seen: HashSet<Vec<(i64, i64)>> = HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            assert!((cell.poly.area() - a0).abs() < 1e-12, "cell {k} area");
            assert_eq!(cell.poly.len(), 5, "cell {k} vertex count");
            // Side multiset {1/2, 1/2, 1/2, 1/2, a}.
            let mut sides: Vec<f64> = (0..5).map(|i| side_of(&cell.poly, i)).collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [0.5, 0.5, 0.5, 0.5, a4];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!(
                    (got - w).abs() < 1e-12,
                    "cell {k} side {got} ∉ {{1/2, 1/2, 1/2, 1/2, a}}"
                );
            }
            // Cyclic (side, angle) fingerprint against the prototile,
            // forward or reversed (the cells realise both chiralities).
            let cs = (0..5).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..5).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            let rev = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(rot + 9 - i) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(rot + 5 - i) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd || rev, "cell {k} is not a congruent type-7 prototile");
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
            // Stored cells equal the raw applet construction exactly
            // (mirrors vertex-reversed to stay CCW).
            let want: [Vec2; 5] = if MIRROR[k] {
                [raw[k][0], raw[k][4], raw[k][3], raw[k][2], raw[k][1]]
            } else {
                raw[k]
            };
            assert_eq!(
                cell.poly.pts,
                want.to_vec(),
                "cell {k} must match the raw construction exactly"
            );
        }
    }

    /// Window census (default parameters) and declared metadata: culling
    /// is the tile AABB intersecting the region (+1e-9), matching the
    /// independent Python emulation of the lattice.rs culling.
    #[test]
    fn pentagon_7_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::Pgg);
        assert_eq!(t.lattice_kind, LatticeKind::Rectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the cell-0 tile (seed 0)");
        let p = ParamSet::default_for(t.params());
        for (region, want) in [
            (BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)), 398),
            (BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)), 369),
            (BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)), 193),
            (BBox::new(v2(-6.0, -6.0), v2(22.0, 14.0)), 1149),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }

    /// Refined edge census over 907 tiles: the tiling is edge-to-edge at
    /// the pentagon level — of the sides whose midpoint lies strictly
    /// inside the window, none is split by a foreign vertex, and each is
    /// covered by exactly one full partner side (3770 full edges, 0
    /// splits).
    #[test]
    fn pentagon_7_refined_edge_census() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let region = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 907, "patch size");
        let verts: Vec<Vec<Vec2>> = r.tiles.iter().map(|tile| tile.poly.pts.clone()).collect();
        let n = verts.len();
        let mut n_full = 0;
        let mut bad: Vec<String> = Vec::new();
        for i in 0..n {
            for e in 0..5 {
                let (pp, qq) = (verts[i][e], verts[i][(e + 1) % 5]);
                let mid = (pp + qq) * 0.5;
                if !(mid.x > -7.5 && mid.x < 13.5 && mid.y > -7.5 && mid.y < 11.5) {
                    continue;
                }
                let dv = qq - pp;
                let l2 = dv.dot(dv);
                let l = l2.sqrt();
                // Split points: other tiles' vertices strictly inside the
                // open segment (collinear within 1e-7), binned at 1e-9 and
                // merged within gap 10 bins.  Edge-to-edge: these bins must
                // be empty.
                let mut bins: BTreeSet<i64> = BTreeSet::new();
                for (j, vj) in verts.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    for &vv in vj.iter() {
                        let tt = (vv - pp).dot(dv) / l2;
                        if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                            continue;
                        }
                        let cross = (vv.x - pp.x) * dv.y - (vv.y - pp.y) * dv.x;
                        if cross.abs() / l < 1e-7 {
                            bins.insert((tt * 1e9).round() as i64);
                        }
                    }
                }
                let mut srt: Vec<i64> = Vec::new();
                for b in bins {
                    if srt.is_empty() || b - srt[srt.len() - 1] > 10 {
                        srt.push(b);
                    }
                }
                if !srt.is_empty() {
                    bad.push(format!("tile {i} edge {e}: split at {srt:?}"));
                    continue;
                }
                n_full += 1;
                // The whole edge must be covered by exactly one full
                // partner side.
                let uh = dv * (1.0 / l);
                let mut cnt = 0;
                for (j, vj) in verts.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    for (a1, b1) in (0..5).map(|k| (vj[k], vj[(k + 1) % 5])) {
                        let c1 = (a1.x - pp.x) * uh.y - (a1.y - pp.y) * uh.x;
                        let c2 = (b1.x - pp.x) * uh.y - (b1.y - pp.y) * uh.x;
                        if c1.abs() > 1e-7 || c2.abs() > 1e-7 {
                            continue;
                        }
                        let ta = (a1 - pp).dot(uh);
                        let tb = (b1 - pp).dot(uh);
                        if ta.min(tb) <= 1e-7 * l && ta.max(tb) >= l * (1.0 - 1e-7) {
                            cnt += 1;
                        }
                    }
                }
                if cnt != 1 {
                    bad.push(format!("tile {i} edge {e} has {cnt} covering partner sides"));
                }
            }
        }
        assert!(bad.is_empty(), "edge census violations: {bad:?}");
        assert_eq!(n_full, 3770, "full (unsplit) edges");
    }

    /// Vertex-figure census over the 907-tile patch (margin 2.0): every
    /// interior vertex is a complete star — corner rays plus rays of edges
    /// passing through it — merging within 1e-4 rad; the three vertex
    /// figures are {E,E,B} = (106,106,148) [271], {D,C,C} = (94,133,133)
    /// [266] and {A,A,D,B} = (59,59,94,148) [279], each summing to 360°;
    /// 816 vertices in total.
    #[test]
    fn pentagon_7_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let big = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        const MARG: f64 = 2.0;
        let mut pts: Vec<Vec2> = Vec::new();
        for tile in &r.tiles {
            for q in &tile.poly.pts {
                if q.x > big.min.x + MARG
                    && q.x < big.max.x - MARG
                    && q.y > big.min.y + MARG
                    && q.y < big.max.y - MARG
                    && !pts.iter().any(|o| (*o - *q).length() < 1e-4)
                {
                    pts.push(*q);
                }
            }
        }
        let mut n3: BTreeMap<(u32, u32, u32), usize> = BTreeMap::new();
        let mut n4: BTreeMap<(u32, u32, u32, u32), usize> = BTreeMap::new();
        for pt in &pts {
            let mut rays: BTreeSet<i64> = BTreeSet::new();
            for tile in &r.tiles {
                let nn = tile.poly.len();
                for k in 0..nn {
                    if (tile.poly.pts[k] - *pt).length() >= 1e-5 {
                        continue;
                    }
                    for &q in [&tile.poly.pts[(k + nn - 1) % nn], &tile.poly.pts[(k + 1) % nn]] {
                        let dir = q - *pt;
                        let th = dir.y.atan2(dir.x).rem_euclid(std::f64::consts::TAU);
                        rays.insert((th * 1e5).round() as i64);
                    }
                }
                for e in 0..nn {
                    let (a, b) = (tile.poly.pts[e], tile.poly.pts[(e + 1) % nn]);
                    let ab = b - a;
                    let l2 = ab.dot(ab);
                    let tt = (*pt - a).dot(ab) / l2;
                    if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                        continue;
                    }
                    if (*pt - a).cross(ab).abs() / l2.sqrt() < 1e-7 {
                        let th0 = ab.y.atan2(ab.x);
                        for add in [0.0, std::f64::consts::PI] {
                            let th = (th0 + add).rem_euclid(std::f64::consts::TAU);
                            rays.insert((th * 1e5).round() as i64);
                        }
                    }
                }
            }
            let mut mr: Vec<f64> = Vec::new();
            for k in &rays {
                let rv = *k as f64 / 1e5;
                if mr.is_empty() || rv - mr[mr.len() - 1] > 1e-4 {
                    mr.push(rv);
                }
            }
            if mr.len() > 1 && mr[mr.len() - 1] - mr[0] > std::f64::consts::TAU - 1e-4 {
                mr.pop();
            }
            let mut gaps: Vec<f64> = (0..mr.len())
                .map(|i| (mr[(i + 1) % mr.len()] - mr[i]).rem_euclid(std::f64::consts::TAU))
                .collect();
            gaps.retain(|g| *g > 1e-4);
            gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let fig: Vec<u32> = gaps.iter().map(|g| g.to_degrees().round() as u32).collect();
            assert_eq!(fig.iter().sum::<u32>(), 360, "vertex {pt:?} figure {fig:?}");
            match fig.len() {
                3 => {
                    let f = (fig[0], fig[1], fig[2]);
                    assert!(
                        f == (106, 106, 148) || f == (94, 133, 133),
                        "unexpected vertex figure {f:?} at {pt:?}"
                    );
                    *n3.entry(f).or_insert(0) += 1;
                }
                4 => {
                    let f = (fig[0], fig[1], fig[2], fig[3]);
                    assert_eq!(f, (59, 59, 94, 148), "unexpected vertex figure {f:?} at {pt:?}");
                    *n4.entry(f).or_insert(0) += 1;
                }
                _ => panic!("vertex {pt:?}: unexpected figure {fig:?}"),
            }
        }
        assert_eq!(pts.len(), 816, "total interior vertices");
        let want3: BTreeMap<(u32, u32, u32), usize> = [
            ((106u32, 106, 148), 271usize),
            ((94, 133, 133), 266),
        ]
        .into_iter()
        .collect();
        assert_eq!(n3, want3, "three-tile vertex stars");
        let want4: BTreeMap<(u32, u32, u32, u32), usize> =
            [((59u32, 59, 94, 148), 279usize)].into_iter().collect();
        assert_eq!(n4, want4, "four-tile vertex star");
    }

    /// Gluing census on the 8-cell torus (mod the shipped lattice): the
    /// 40 edges form 20 partner pairs with no broken edges and equal
    /// lengths; the glued side-pair histogram is a-a:4, b-b:2, b-e:4,
    /// c-d:4, c-e:4, d-d:2; the 40 corners form 12 vertex orbits (eight of
    /// size 3, four of size 4) whose stars are (58.9, 58.9, 94.3, 148.0)
    /// ×4, (94.3, 132.9, 132.9) ×4 and (106, 106, 148) ×4.
    #[test]
    fn pentagon_7_cell_gluing_census() {
        type PtKey = (i64, i64);
        type EdgeKey = (PtKey, PtKey);
        let t = spec();
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        assert!(det > 0.0, "the shipped basis must be positively oriented");
        let u_of = |p: Vec2| {
            v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
        };
        let ukey = |p: Vec2| {
            let u = u_of(p);
            (
                ((u.x.rem_euclid(1.0) * 1e6).round() as i64) % 1_000_000,
                ((u.y.rem_euclid(1.0) * 1e6).round() as i64) % 1_000_000,
            )
        };
        let cells: Vec<Vec<Vec2>> = t.cells.iter().map(|c| c.poly.pts.clone()).collect();
        // Edges: 40 of them, each glued to exactly one partner edge (its
        // reverse) — 20 orbits, no broken edges.
        let mut partners: BTreeMap<EdgeKey, Vec<(usize, usize)>> = BTreeMap::new();
        let mut lens: Vec<f64> = Vec::new();
        for (k, pts) in cells.iter().enumerate() {
            for e in 0..5 {
                let (a, b) = (pts[e], pts[(e + 1) % 5]);
                lens.push((a - b).length());
                partners.entry((ukey(a), ukey(b))).or_default().push((k, e));
            }
        }
        assert_eq!(lens.len(), 40, "eight pentagons, five sides each");
        let label = |k: usize, e: usize| -> u8 {
            // A stored CCW edge e of cell k carries prototile side index
            // (e + 1) mod 5 when the cell is direct and (5 − e) mod 5 when
            // mirror-stored ("abcde" order: a = the long side, b = A→B,
            // c = B→C, d = C→D, e = D→E).
            let l = if MIRROR[k] {
                (5 - e) % 5
            } else {
                (e + 1) % 5
            };
            l as u8
        };
        let mut done = [false; 40];
        let mut n_orbits = 0;
        let mut side_pairs: BTreeMap<(u8, u8), usize> = BTreeMap::new();
        for idx in 0..40 {
            if done[idx] {
                continue;
            }
            let (k, e) = (idx / 5, idx % 5);
            let pts = &cells[k];
            let (a, b) = (pts[e], pts[(e + 1) % 5]);
            let list = partners.get(&(ukey(b), ukey(a))).cloned().unwrap_or_default();
            let list: Vec<(usize, usize)> = list.into_iter().filter(|x| *x != (k, e)).collect();
            assert_eq!(
                list.len(),
                1,
                "edge ({k},{e}) must have exactly one glued partner, got {list:?}"
            );
            let (j, f) = list[0];
            let jidx = j * 5 + f;
            assert!(!done[jidx], "edge ({j},{f}) was already paired");
            done[idx] = true;
            done[jidx] = true;
            n_orbits += 1;
            // Partner endpoints: a' = b + λ, b' = a + λ for one lattice
            // vector λ (0 for interior edges, ±w for boundary edges).
            let (pj, qj) = (cells[j][f], cells[j][(f + 1) % 5]);
            let lam = pj - b;
            let ulam = u_of(lam);
            let frac = |x: f64| x.rem_euclid(1.0).min(1.0 - x.rem_euclid(1.0));
            assert!(
                frac(ulam.x) < 1e-6 && frac(ulam.y) < 1e-6,
                "glued edges ({k},{e}) and ({j},{f}): shift {ulam:?} not a lattice vector"
            );
            let jitter = (a + lam - qj).length();
            assert!(
                jitter < 1e-9,
                "glued edges ({k},{e}) and ({j},{f}) disagree mod lattice by {jitter}"
            );
            assert!(
                (lens[idx] - lens[jidx]).abs() < 1e-9,
                "glued edges ({k},{e}) and ({j},{f}) must have equal length"
            );
            let (la, lb) = (label(k, e), label(j, f));
            let pair = if la <= lb {
                (la, lb)
            } else {
                (lb, la)
            };
            *side_pairs.entry(pair).or_insert(0) += 1;
        }
        assert_eq!(n_orbits, 20, "twenty edge orbits on the torus");
        let want_pairs: BTreeMap<(u8, u8), usize> = [
            ((0u8, 0u8), 4usize), // a-a
            ((1, 1), 2), // b-b
            ((1, 4), 4), // b-e
            ((2, 3), 4), // c-d
            ((2, 4), 4), // c-e
            ((3, 3), 2), // d-d
        ]
        .into_iter()
        .collect();
        assert_eq!(side_pairs, want_pairs, "glued side-pair census (a,b,c,d,e indices)");
        // Vertex orbits mod lattice and their stars.
        let mut orbits: BTreeMap<PtKey, Vec<(usize, usize)>> = BTreeMap::new();
        for (k, pts) in cells.iter().enumerate() {
            for (v, &p) in pts.iter().enumerate() {
                orbits.entry(ukey(p)).or_default().push((k, v));
            }
        }
        assert_eq!(orbits.len(), 12, "twelve vertex orbits on the torus");
        let mut sizes: Vec<usize> = orbits.values().map(|o| o.len()).collect();
        sizes.sort_unstable();
        assert_eq!(
            sizes,
            vec![3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4],
            "orbit sizes (eight 3s and four 4s)"
        );
        let mut stars: Vec<Vec<i64>> = Vec::new();
        for corners in orbits.values() {
            let mut tens: Vec<i64> = corners
                .iter()
                .map(|&(k, v)| {
                    (angle_at(&t.cells[k].poly, v).to_degrees() * 10.0).round() as i64
                })
                .collect();
            tens.sort_unstable();
            stars.push(tens);
        }
        stars.sort();
        let want_stars: Vec<Vec<i64>> = [
            vec![589, 589, 943, 1480],
            vec![589, 589, 943, 1480],
            vec![589, 589, 943, 1480],
            vec![589, 589, 943, 1480],
            vec![943, 1329, 1329],
            vec![943, 1329, 1329],
            vec![943, 1329, 1329],
            vec![943, 1329, 1329],
            vec![1060, 1060, 1480],
            vec![1060, 1060, 1480],
            vec![1060, 1060, 1480],
            vec![1060, 1060, 1480],
        ]
        .to_vec();
        assert_eq!(stars, want_stars, "vertex-orbit stars (degree tenths)");
    }

    /// The wallpaper group is pgg: the half-turn about c0 = (1/4, 0)
    /// preserves the tiling exactly (interior-filtered set equality), the
    /// two perpendicular glides g1 = (M1, t1), g2 = (M2, t2) with axes
    /// along w1 and w2 preserve it (with every lattice-shifted variant,
    /// and so do their inverses), g1² = w1, g2² = −w2, and g2∘g1 is the
    /// half-turn about c0 itself.  Each glide shifts by exactly half a
    /// primitive translation along its axis, so no power of it is a pure
    /// reflection: neither the perpendicular-part candidate nor any whole
    /// lattice shift (24 negative probes in total, plus two quarter-turn
    /// probes) maps the prototile onto a tile.
    #[test]
    fn pentagon_7_pgg_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let c0 = two_fold_centre();
        assert_eq!(c0, v2(0.25, 0.0), "the 2-fold centre is (1/4, 0)");
        let ra = BBox::new(v2(c0.x - 12.0, c0.y - 10.0), v2(c0.x + 12.0, c0.y + 10.0));
        check_iso(&t, &p, (-1.0, 0.0, 0.0, -1.0), c0 * 2.0, ra, true, "half-turn about c0");
        let u1 = w1 * (1.0 / w1.length());
        let u2 = w2 * (1.0 / w2.length());
        let m1 = mirror(u1);
        let m2 = mirror(u2);
        // Glide translations, re-derived from stored-cell centroids:
        // g1 sends cell 0's centroid to cell 3's, g2's to cell 7's.
        let cent0 = cent(&t.cells[0].poly.pts);
        let t1 = cent(&t.cells[3].poly.pts) - apply_iso(m1, Vec2::ZERO, cent0);
        let t2 = cent(&t.cells[7].poly.pts) - apply_iso(m2, Vec2::ZERO, cent0);
        assert!(
            (t1 - v2(0.1954787892179718, 1.2530607552235797)).length() < 1e-12,
            "t1 {t1:?}"
        );
        assert!(
            (t2 - v2(1.7467969065338207, -0.2321005970981298)).length() < 1e-12,
            "t2 {t2:?}"
        );
        // Exact generator identities: g² = (I + M)·t.
        let gs1 = v2((m1.0 + 1.0) * t1.x + m1.1 * t1.y, m1.2 * t1.x + (m1.3 + 1.0) * t1.y);
        assert!((gs1 - w1).length() < 1e-9, "g1² must equal w1");
        let gs2 = v2((m2.0 + 1.0) * t2.x + m2.1 * t2.y, m2.2 * t2.x + (m2.3 + 1.0) * t2.y);
        assert!((gs2 + w2).length() < 1e-9, "g2² must equal −w2");
        // Each glide shifts by exactly half a primitive translation along
        // its axis (g1 by +w1/2, g2 by −w2/2) — no power is a reflection.
        assert!((2.0 * t1.dot(u1) / w1.length() - 1.0).abs() < 1e-6, "g1 must shift +w1/2");
        assert!((2.0 * t2.dot(u2) / w2.length() + 1.0).abs() < 1e-6, "g2 must shift −w2/2");
        // g2∘g1 is the half-turn centred at c0 itself.
        let comp = v2(m2.0 * t1.x + m2.1 * t1.y + t2.x, m2.2 * t1.x + m2.3 * t1.y + t2.y);
        assert!((comp * 0.5 - c0).length() < 1e-9, "centre(g2∘g1) must be c0");
        // The glides preserve the tiling (the image window is tilted, so
        // check containment of the images among the tiles generated
        // there) — and so do the inverses.
        check_iso(&t, &p, m1, t1, ra, false, "glide g1 (axis ∥ w1)");
        check_iso(
            &t,
            &p,
            m1,
            v2(-(m1.0 * t1.x + m1.1 * t1.y), -(m1.2 * t1.x + m1.3 * t1.y)),
            ra,
            false,
            "glide g1⁻¹",
        );
        check_iso(&t, &p, m2, t2, ra, false, "glide g2 (axis ∥ w2)");
        check_iso(
            &t,
            &p,
            m2,
            v2(-(m2.0 * t2.x + m2.1 * t2.y), -(m2.2 * t2.x + m2.3 * t2.y)),
            ra,
            false,
            "glide g2⁻¹",
        );
        // Big patch for tile-image membership.
        let big = BBox::new(v2(-14.0, -12.0), v2(20.0, 16.0));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        assert_eq!(big_tiles.tiles.len(), 1922, "big patch tile count");
        let set: HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        assert_eq!(set.len(), 1922, "big patch tiles must be unique");
        // Every cell maps onto a tile under the three generators.
        type Iso = ((f64, f64, f64, f64), Vec2);
        let maps: [(&str, Iso); 3] = [
            ("ht", ((-1.0, 0.0, 0.0, -1.0), c0 * 2.0)),
            ("g1", (m1, t1)),
            ("g2", (m2, t2)),
        ];
        for k in 0..8 {
            for (nm, (mm, tt)) in &maps {
                let img: Vec<Vec2> =
                    t.cells[k].poly.pts.iter().map(|q| apply_iso(*mm, *tt, *q)).collect();
                assert!(set.contains(&vkey(&img)), "{nm}(cell {k}) must be a tile");
            }
        }
        let prot = t.cells[0].poly.pts.clone();
        for i in -2..=2 {
            for &w in [&w1, &w2] {
                let img: Vec<Vec2> = prot.iter().map(|q| *q + w * i as f64).collect();
                assert!(set.contains(&vkey(&img)), "cell0 + {i}·w must be a tile");
            }
        }
        // All 36 half-turns in the four classes c0 + Λ/2 map cell0 onto a
        // tile.
        for h in [Vec2::ZERO, w1 * 0.5, w2 * 0.5, (w1 + w2) * 0.5] {
            for i in -1..=1 {
                for j in -1..=1 {
                    let ctr = c0 + h + w1 * i as f64 + w2 * j as f64;
                    let img: Vec<Vec2> = prot
                        .iter()
                        .map(|q| apply_iso((-1.0, 0.0, 0.0, -1.0), ctr * 2.0, *q))
                        .collect();
                    assert!(
                        set.contains(&vkey(&img)),
                        "half-turn about c0 + {h:?} + {i}·w1 + {j}·w2 must map cell0 onto a tile"
                    );
                }
            }
        }
        // Lattice-shifted glides still map cell0 onto a tile.
        for (i, j) in [(0.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (1.0, 0.0)] {
            let sh = w1 * i + w2 * j;
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m1, t1 + sh, *q)).collect();
            assert!(
                set.contains(&vkey(&img)),
                "(g1 + {i}·w1 + {j}·w2) must map cell0 onto a tile"
            );
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m2, t2 + sh, *q)).collect();
            assert!(
                set.contains(&vkey(&img)),
                "(g2 + {i}·w1 + {j}·w2) must map cell0 onto a tile"
            );
        }
        // No pure reflection: the mirror-class candidates carrying only the
        // perpendicular glide part (a reflection would need zero along-axis
        // shift) and every whole-lattice-shift candidate must fail to map
        // the prototile onto a tile.
        let p1 = t1 - u1 * t1.dot(u1);
        let p2 = t2 - u2 * t2.dot(u2);
        for (m, tr, nm) in [(m1, p1, "(M1, t1⊥)"), (m2, p2, "(M2, t2⊥)")] {
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, tr, *q)).collect();
            assert!(
                !set.contains(&vkey(&img)),
                "{nm} must not be a symmetry (no mirrors in pgg)"
            );
        }
        for i in -2..=2 {
            for (m, w, mn, wn) in [
                (m1, w1, "M1", "w1"),
                (m1, w2, "M1", "w2"),
                (m2, w1, "M2", "w1"),
                (m2, w2, "M2", "w2"),
            ] {
                let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, w * i as f64, *q)).collect();
                assert!(
                    !set.contains(&vkey(&img)),
                    "({mn}, {i}·{wn}) must not be a symmetry (no mirrors in pgg)"
                );
            }
        }
        // No quarter-turn either.
        for (nm, tr) in [
            ("quarter-turn about the origin", Vec2::ZERO),
            ("quarter-turn at (1/4, −1/4)", v2(0.25, -0.25)),
        ] {
            let img: Vec<Vec2> = prot
                .iter()
                .map(|q| apply_iso((0.0, -1.0, 1.0, 0.0), tr, *q))
                .collect();
            assert!(!set.contains(&vkey(&img)), "{nm} must not be a symmetry");
        }
    }

    /// 2-isohedral: seeds 0/1 are the two full-group isohedral classes —
    /// the half-turn and both glide classes permute the eight cells
    /// mod-lattice by the closed permutations, the group they generate has
    /// exactly two orbits on the cells ({0,3,4,7} / {1,2,5,6}, matching
    /// the seeds), both seeds appear in every patch, and every generated
    /// tile keeps the prototile side multiset {1/2, 1/2, 1/2, 1/2, a}.
    #[test]
    fn pentagon_7_isohedral_classes() {
        let t = spec();
        let mut counts = [0usize; 2];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(counts, [4, 4], "four tiles per isohedral class");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        let u_of = |p: Vec2| {
            v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
        };
        let mod1 = |x: f64| (x + 0.5).rem_euclid(1.0) - 0.5;
        let c0 = two_fold_centre();
        let u1 = w1 * (1.0 / w1.length());
        let u2 = w2 * (1.0 / w2.length());
        let cent0 = cent(&t.cells[0].poly.pts);
        let t1 = cent(&t.cells[3].poly.pts) - apply_iso(mirror(u1), Vec2::ZERO, cent0);
        let t2 = cent(&t.cells[7].poly.pts) - apply_iso(mirror(u2), Vec2::ZERO, cent0);
        let cents: Vec<Vec2> = t.cells.iter().map(|c| cent(&c.poly.pts)).collect();
        // A symmetry (M, t) permutes the cells: cell k's centroid maps to
        // cell j's centroid mod lattice, for exactly one j.
        let perm_of = |m: (f64, f64, f64, f64), tr: Vec2| -> Vec<usize> {
            let mut perm: Vec<usize> = Vec::new();
            for k in 0..8 {
                let img = apply_iso(m, tr, cents[k]);
                let mut hits: Vec<usize> = Vec::new();
                for (j, &cj) in cents.iter().enumerate() {
                    let du = u_of(img - cj);
                    if mod1(du.x).abs() < 1e-6 && mod1(du.y).abs() < 1e-6 {
                        hits.push(j);
                    }
                }
                assert_eq!(hits.len(), 1, "cell {k} must have exactly one image, got {hits:?}");
                perm.push(hits[0]);
            }
            perm
        };
        let perms = [
            perm_of((-1.0, 0.0, 0.0, -1.0), c0 * 2.0),
            perm_of(mirror(u1), t1),
            perm_of(mirror(u2), t2),
        ];
        assert_eq!(perms[0], vec![4, 5, 6, 7, 0, 1, 2, 3], "half-turn perm");
        assert_eq!(perms[1], vec![3, 2, 1, 0, 7, 6, 5, 4], "glide g1 perm");
        assert_eq!(perms[2], vec![7, 6, 5, 4, 3, 2, 1, 0], "glide g2 perm");
        // Two full-group orbits on the cells, exactly the seed classes.
        let mut seen = [false; 8];
        let mut orbits: Vec<Vec<usize>> = Vec::new();
        for st in 0..8 {
            if seen[st] {
                continue;
            }
            let mut orb = vec![st];
            seen[st] = true;
            let mut head = 0;
            while head < orb.len() {
                let x = orb[head];
                head += 1;
                for pm in &perms {
                    let y = pm[x];
                    if !seen[y] {
                        seen[y] = true;
                        orb.push(y);
                    }
                }
            }
            orb.sort_unstable();
            orbits.push(orb);
        }
        assert_eq!(
            orbits,
            vec![vec![0, 3, 4, 7], vec![1, 2, 5, 6]],
            "two full-group orbits"
        );
        for orb in &orbits {
            let sd = t.cells[orb[0]].seed;
            assert!(
                orb.iter().all(|&k| t.cells[k].seed == sd),
                "orbit mixes isohedral seeds"
            );
        }
        assert_ne!(t.cells[orbits[0][0]].seed, t.cells[orbits[1][0]].seed, "orbits differ");
        // Generated patches: both classes present, side multiset kept.
        let params = ParamSet::default_for(t.params());
        let rp = t.generate(
            &params,
            &Query {
                region: BBox::new(v2(-6.0, -6.0), v2(10.0, 10.0)),
            },
        );
        assert!(rp.message.is_none(), "{:?}", rp.message);
        let a4 = side_of(&t.cells[0].poly, 4);
        let mut seen_s = [false; 2];
        for tile in &rp.tiles {
            let sd = tile.seed as usize;
            assert!(sd < 2, "seeds must stay in {{0, 1}}");
            seen_s[sd] = true;
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length())
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [0.5, 0.5, 0.5, 0.5, a4];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!(
                    (got - w).abs() < 1e-12,
                    "seed {sd} side {got} ∉ {{1/2 × 4, a}}"
                );
            }
        }
        assert!(seen_s[0] && seen_s[1], "both isohedral classes must appear");
    }

    /// The eight cells realise both chiralities of the (chiral) type-7
    /// prototile: cell 0 matches the prototile's cyclic (side, angle)
    /// sequence forward but not reversed, every cell matches exactly one
    /// of the two, and the counts are four forward and four reversed —
    /// the mirror half of the primitive unit.
    #[test]
    fn pentagon_7_enantiomer_mix() {
        let t = spec();
        let proto = &t.cells[0].poly;
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        let match_fwd = |cs: &[f64], ca: &[f64]| {
            (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            })
        };
        let match_rev = |cs: &[f64], ca: &[f64]| {
            (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(rot + 9 - i) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(rot + 5 - i) % 5]).abs() < 1e-9
                })
            })
        };
        // The prototile itself: forward match only (the family is chiral).
        assert!(match_fwd(&ps, &pa), "prototile must match forward");
        assert!(!match_rev(&ps, &pa), "prototile must be chiral (no reversed match)");
        let mut n_fwd = 0;
        let mut n_rev = 0;
        for (k, cell) in t.cells.iter().enumerate() {
            let cs: Vec<f64> = (0..5).map(|i| side_of(&cell.poly, i)).collect();
            let ca: Vec<f64> = (0..5).map(|i| angle_at(&cell.poly, i)).collect();
            let f = match_fwd(&cs, &ca);
            let r = match_rev(&cs, &ca);
            assert!(f != r, "cell {k} must match exactly one chirality");
            if f {
                n_fwd += 1;
            } else {
                n_rev += 1;
            }
        }
        assert_eq!((n_fwd, n_rev), (4, 4), "four direct and four mirror tiles");
    }
}

#[cfg(test)]
mod pentagon8_tests {
    use super::pentagon8::{s3, tile_area};
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon8::spec;
    use std::collections::{BTreeMap, HashSet};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// The mirror-stored cells (0-based {1, 2, 4, 7} = T2, T3, T5, T8):
    /// the mirror half of the primitive unit, stored CCW with their
    /// vertex order reversed.
    const MIRROR: [bool; 8] = [false, true, true, false, true, false, false, true];

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Vertex-set key on a 1e-6 grid (every coordinate lies on the Z[√3]
    /// grid, so distinct tiling vertices are ≥ 1 apart, and two
    /// computations of a shared joint agree to ~1e-15).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Single-vertex key on the same 1e-6 grid.
    fn pkey(p: Vec2) -> (i64, i64) {
        ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64)
    }

    /// Verify that the isometry g = (M, t) preserves the tiling: the
    /// image of every tile intersecting `ra` must be a tile intersecting
    /// `rb` (the axis-aligned box of g(ra)).  With `exact = true` the
    /// image set must equal the `rb` tile set (the case g(ra) = rb).
    fn check_symmetry(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            Vec2::new(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            Vec2::new(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ra_tiles = t.generate(params, &Query { region: ra });
        let rb_tiles = t.generate(params, &Query { region: rb });
        assert!(ra_tiles.message.is_none(), "{label}: {:?}", ra_tiles.message);
        assert!(rb_tiles.message.is_none(), "{label}: {:?}", rb_tiles.message);
        assert!(!ra_tiles.tiles.is_empty(), "{label}: no tiles");
        let set_b: std::collections::HashSet<Vec<(i64, i64)>> =
            rb_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let set_a: std::collections::HashSet<Vec<(i64, i64)>> = ra_tiles
            .tiles
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(
            set_a.len(),
            ra_tiles.tiles.len(),
            "{label}: isometry must be injective on the patch"
        );
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    /// Directed-edge key: keys of its two endpoints on the 1e-6 grid.
    type EdgeKey = ((i64, i64), (i64, i64));

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    /// Centroid of a stored cell (mean of its five vertices).
    fn cent(pts: &[Vec2]) -> Vec2 {
        let s = pts.iter().fold(Vec2::ZERO, |acc, &p| acc + p);
        s / 5.0
    }

    #[test]
    fn pentagon_8_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        let s = s3();
        check_coverage(
            &t,
            "pentagon-8",
            &p,
            BBox::new(v2(-25.0, -20.0 * s), v2(25.0, 20.0 * s)),
        );
    }

    #[test]
    fn pentagon_8_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-8(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(7.0, 7.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_8_deterministic() {
        check_det(&spec(), "pentagon-8", BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)));
    }

    /// Unit structure of the primitive cell: 8 tiles of the exact Q(√3)
    /// prototile (B = C = 120°, tan A = 3√3, cos D = 1/7, tan E = −√3/2,
    /// A = E − 60°; a = 6√7, b = c = d = e = 14) on the rectangular
    /// lattice w1 = (36, 24√3) ⊥ w2 = (−36, 18√3), with |det| = 8 × tile
    /// area = 1512·√3, |w1|² = 3024 and |w2|² = 2268.  Every cell is a
    /// congruent copy of the prototile (cyclic (side, angle) fingerprint
    /// forward or reversed), CCW, no two cells coincide, and the family
    /// condition identities hold to float precision.
    #[test]
    fn pentagon_8_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-8");
        assert_eq!(t.cells.len(), 8, "the primitive unit holds eight tiles");
        let s = s3();
        assert_eq!(t.lattice[0], v2(36.0, 24.0 * s), "w1 = (36, 24√3)");
        assert_eq!(t.lattice[1], v2(-36.0, 18.0 * s), "w2 = (−36, 18√3)");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        assert!(w1.dot(w2).abs() < 1e-12, "lattice must be rectangular (w1 ⊥ w2)");
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        assert!(
            (det - 8.0 * tile_area()).abs() < 1e-9,
            "det {det} ≠ 8 × tile area = 1512√3"
        );
        assert!(
            ((w1.x * w1.x + w1.y * w1.y) - 3024.0).abs() < 1e-9,
            "|w1|² = 1296 + 1728 = 3024"
        );
        assert!(
            ((w2.x * w2.x + w2.y * w2.y) - 2268.0).abs() < 1e-9,
            "|w2|² = 1296 + 972 = 2268"
        );
        // Seeds follow the two full-group isohedral orbits
        // {T1,T4,T5,T8} (0) and {T2,T3,T6,T7} (1).
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 1, 1, 0, 0, 1, 1, 0], "isohedral seed pattern");
        // The shipped prototile is T1, corner A at the origin:
        // A = (0,0), B = (14,0), C = (21,7√3), D = (14,14√3), E = (3,9√3).
        let proto = &t.cells[0].poly;
        for (i, want) in [(0.0, 0.0), (14.0, 0.0), (21.0, 7.0 * s), (14.0, 14.0 * s), (3.0, 9.0 * s)]
            .iter()
            .enumerate()
        {
            assert_eq!(proto.pts[i], v2(want.0, want.1), "prototile corner {i}");
        }
        // Exact corner angle cosines in vertex order A, B, C, D, E
        // (B = C = 120°).
        let cos = |i: usize| angle_at(proto, i).cos();
        assert!((cos(0) - 7.0f64.sqrt() / 14.0).abs() < 1e-12, "cos A = √7/14");
        assert!((cos(1) + 0.5).abs() < 1e-12, "cos B = −1/2");
        assert!((cos(2) + 0.5).abs() < 1e-12, "cos C = −1/2");
        assert!((cos(3) - 1.0 / 7.0).abs() < 1e-12, "cos D = 1/7");
        assert!((cos(4) + 2.0 * 7.0f64.sqrt() / 7.0).abs() < 1e-12, "cos E = −2√7/7");
        // Family condition identities, from the stored angles.
        let a = angle_at(proto, 0);
        let e = angle_at(proto, 4);
        assert!((a.tan() - 3.0 * s).abs() < 1e-9, "tan A = 3√3");
        assert!((e.tan() + s / 2.0).abs() < 1e-9, "tan E = −√3/2");
        assert!(
            (a + 60.0f64.to_radians() - e).abs() < 1e-9,
            "A = E − 60°"
        );
        assert!(
            (2.0 * angle_at(proto, 1) + angle_at(proto, 2) - 2.0 * std::f64::consts::PI).abs()
                < 1e-9,
            "2B + C = 360°"
        );
        assert!(
            (angle_at(proto, 3) + 2.0 * e - 2.0 * std::f64::consts::PI).abs() < 1e-9,
            "D + 2E = 360°"
        );
        let a_side = side_of(proto, 4);
        assert!(
            (a_side / 14.0 - 3.0 / 7.0f64.sqrt()).abs() < 1e-12,
            "a/b = 6√7/14 = 3/√7"
        );
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        let mut seen: HashSet<Vec<(i64, i64)>> = HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            assert!((cell.poly.area() - tile_area()).abs() < 1e-12, "cell {k} area ≠ 189√3");
            assert_eq!(cell.poly.len(), 5, "cell {k} vertex count");
            // Side multiset {14, 14, 14, 14, 6√7}.
            let mut sides: Vec<f64> = (0..5).map(|i| side_of(&cell.poly, i)).collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [14.0, 14.0, 14.0, 14.0, 6.0 * 7.0f64.sqrt()];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!(
                    (got - w).abs() < 1e-12,
                    "cell {k} side {got} ∉ {{14 × 4, 6√7}}"
                );
            }
            // Cyclic (side, angle) fingerprint against the prototile,
            // forward or reversed (the cells realise both chiralities).
            let cs = (0..5).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..5).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            let rev = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(rot + 9 - i) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(rot + 5 - i) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd || rev, "cell {k} is not a congruent type-8 prototile");
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
        }
    }

    /// Window census (default parameters): culling is the tile AABB
    /// intersecting the region (+1e-9); every shipped coordinate lies on
    /// the Z[√3] grid, and the half-integer window bounds keep every
    /// culling decision unambiguous, so the counts are exact and match
    /// the independent Python emulation of the lattice.rs culling.
    #[test]
    fn pentagon_8_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::Pgg);
        assert_eq!(t.lattice_kind, LatticeKind::Rectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the T1 cell (seed 0)");
        let p = ParamSet::default_for(t.params());
        let s = s3();
        for (region, want) in [
            (BBox::new(v2(-60.5, -45.5 * s), v2(60.5, 45.5 * s)), 79),
            (BBox::new(v2(-40.5, -30.5 * s), v2(40.5, 30.5 * s)), 42),
            (BBox::new(v2(-25.5, -20.5 * s), v2(25.5, 20.5 * s)), 20),
            (BBox::new(v2(-4.5, -3.5 * s), v2(10.5, 10.5 * s)), 5),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }

    /// Edge-to-edge at the pentagon level over the 79-tile patch: no
    /// directed side is shared by two tiles (395 directed edges, 0
    /// duplicates), every side whose midpoint lies strictly inside the
    /// window (290 of them) is matched by exactly one full side of one
    /// neighbour (its reverse), and no vertex falls in the interior of a
    /// non-incident side (the closest genuine near-miss is 13.7 units
    /// away).
    #[test]
    fn pentagon_8_edge_to_edge_gluing() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let s = s3();
        let big = BBox::new(v2(-60.5, -45.5 * s), v2(60.5, 45.5 * s));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 79, "patch size");
        let mut dirs: std::collections::HashMap<EdgeKey, usize> =
            std::collections::HashMap::new();
        for tile in &r.tiles {
            let n = tile.poly.len();
            for i in 0..n {
                let k = (pkey(tile.poly.pts[i]), pkey(tile.poly.pts[(i + 1) % n]));
                *dirs.entry(k).or_insert(0) += 1;
            }
        }
        assert_eq!(dirs.len(), 395, "395 directed sides, all unique");
        assert!(
            dirs.values().all(|&c| c == 1),
            "a directed side may belong to only one tile"
        );
        let mut interior = 0;
        for tile in &r.tiles {
            let n = tile.poly.len();
            for i in 0..n {
                let (a, b) = (tile.poly.pts[i], tile.poly.pts[(i + 1) % n]);
                let mid = (a + b) * 0.5;
                let d = (mid.x - big.min.x)
                    .min(big.max.x - mid.x)
                    .min(mid.y - big.min.y)
                    .min(big.max.y - mid.y);
                if d > 1e-6 {
                    interior += 1;
                    assert_eq!(
                        dirs.get(&(pkey(b), pkey(a))),
                        Some(&1),
                        "interior side {a} -> {b} lacks its full-edge partner"
                    );
                }
            }
        }
        assert_eq!(
            interior, 290,
            "interior directed sides (each paired with its reverse)"
        );
        // No T-junction: no vertex in the interior of a non-incident edge.
        let verts: HashSet<(i64, i64)> =
            r.tiles.iter().flat_map(|tile| tile.poly.pts.iter().map(|&p| pkey(p))).collect();
        assert_eq!(verts.len(), 148, "distinct vertices in the patch");
        for tile in &r.tiles {
            let n = tile.poly.len();
            for i in 0..n {
                let (a, b) = (tile.poly.pts[i], tile.poly.pts[(i + 1) % n]);
                let (dx, dy) = (b.x - a.x, b.y - a.y);
                let l2 = dx * dx + dy * dy;
                for vk in verts.iter().filter(|k| **k != pkey(a) && **k != pkey(b)) {
                    let (px, py) = (vk.0 as f64 / 1e6, vk.1 as f64 / 1e6);
                    let tt = (((px - a.x) * dx + (py - a.y) * dy) / l2).clamp(0.0, 1.0);
                    if tt == 0.0 || tt == 1.0 {
                        continue;
                    }
                    let q = a + (b - a) * tt;
                    assert!(
                        (px - q.x).hypot(py - q.y) > 1e-7,
                        "vertex {vk:?} lies in the interior of side {a} -> {b}"
                    );
                }
            }
        }
    }

    /// Vertex census over the 79-tile patch: every window-interior vertex
    /// (91 of the 148 distinct vertices) is a complete star of incident
    /// tile angles summing to exactly 360°, and the stars are exactly the
    /// three closed types {A,A,C,D} = 2·79.11° + 120° + 81.79° (B = C
    /// merges in the letters: "AABD", 30 of them), {B,B,C} = 3·120°
    /// ("BBB", 36) and {D,E,E} = 81.79° + 2·139.11° ("DEE", 25).
    #[test]
    fn pentagon_8_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let s = s3();
        let big = BBox::new(v2(-60.5, -45.5 * s), v2(60.5, 45.5 * s));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        let verts: HashSet<(i64, i64)> =
            r.tiles.iter().flat_map(|tile| tile.poly.pts.iter().map(|&pt| pkey(pt))).collect();
        // Corner cosines: A = √7/14, B = C = −1/2, D = 1/7, E = −2√7/7.
        let bins = [7.0f64.sqrt() / 14.0, -0.5, 1.0 / 7.0, -2.0 * 7.0f64.sqrt() / 7.0];
        let mut stars: BTreeMap<String, usize> = BTreeMap::new();
        let mut checked = 0;
        for vk in &verts {
            let (px, py) = (vk.0 as f64 / 1e6, vk.1 as f64 / 1e6);
            let d = (px - big.min.x)
                .min(big.max.x - px)
                .min(py - big.min.y)
                .min(big.max.y - py);
            if d < 1e-6 {
                continue; // boundary: incident tiles may be culled
            }
            // Any tile having this corner has its AABB containing the
            // strictly interior point, hence was generated: the star is
            // complete.
            let mut letters = String::new();
            let mut sum = 0.0;
            for tile in &r.tiles {
                let n = tile.poly.len();
                for i in 0..n {
                    if pkey(tile.poly.pts[i]) != *vk {
                        continue;
                    }
                    let ang = angle_at(&tile.poly, i);
                    sum += ang;
                    let c = ang.cos();
                    let j = (0..4)
                        .min_by_key(|&j| ((c - bins[j]).abs() * 1e12) as i64)
                        .unwrap();
                    assert!((c - bins[j]).abs() < 1e-9, "stray corner cosine {c}");
                    letters.push(['A', 'B', 'D', 'E'][j]);
                }
            }
            assert!(
                (sum - std::f64::consts::TAU).abs() < 1e-9,
                "vertex {vk:?} angles sum to {sum} ≠ 2π"
            );
            let mut cs: Vec<char> = letters.chars().collect();
            cs.sort_unstable();
            let star: String = cs.into_iter().collect();
            assert!(
                star == "AABD" || star == "BBB" || star == "DEE",
                "unexpected vertex star {star} at {vk:?}"
            );
            *stars.entry(star).or_insert(0) += 1;
            checked += 1;
        }
        assert_eq!(checked, 91, "interior vertex census");
        assert_eq!(stars.get("AABD").copied().unwrap_or(0), 30, "{{A,A,C,D}} stars");
        assert_eq!(stars.get("BBB").copied().unwrap_or(0), 36, "{{B,B,C}} stars");
        assert_eq!(stars.get("DEE").copied().unwrap_or(0), 25, "{{D,E,E}} stars");
    }

    /// Gluing census on the 8-cell torus (mod the shipped lattice): the
    /// 40 edges form 20 partner pairs, every pair reverse-glued, with no
    /// broken edges and equal lengths; the glued side-pair histogram is
    /// a-a:4, b-c:4, b-d:4, c-c:2, d-e:4, e-e:2; the 40 corners form 12
    /// vertex orbits (eight of size 3, four of size 4) whose stars are
    /// (79.1, 79.1, 120.0, 81.8) ×4, (120.0, 120.0, 120.0) ×4 and
    /// (81.8, 139.1, 139.1) ×4.
    #[test]
    fn pentagon_8_cell_gluing_census() {
        type PtKey = (i64, i64);
        type EdgeKey2 = (PtKey, PtKey);
        let t = spec();
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        assert!(det > 0.0, "the shipped basis must be positively oriented");
        let u_of = |p: Vec2| {
            v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
        };
        let ukey = |p: Vec2| {
            let u = u_of(p);
            (
                ((u.x.rem_euclid(1.0) * 1e6).round() as i64) % 1_000_000,
                ((u.y.rem_euclid(1.0) * 1e6).round() as i64) % 1_000_000,
            )
        };
        let cells: Vec<Vec<Vec2>> = t.cells.iter().map(|c| c.poly.pts.clone()).collect();
        // Edges: 40 of them, each glued to exactly one partner edge (its
        // reverse) — 20 orbits, no broken edges.
        let mut partners: BTreeMap<EdgeKey2, Vec<(usize, usize)>> = BTreeMap::new();
        let mut lens: Vec<f64> = Vec::new();
        for (k, pts) in cells.iter().enumerate() {
            for e in 0..5 {
                let (a, b) = (pts[e], pts[(e + 1) % 5]);
                lens.push((a - b).length());
                partners.entry((ukey(a), ukey(b))).or_default().push((k, e));
            }
        }
        assert_eq!(lens.len(), 40, "eight pentagons, five sides each");
        let label = |k: usize, e: usize| -> u8 {
            // A stored CCW edge e of cell k carries prototile side index
            // (e + 1) mod 5 when the cell is direct and (4 − e) mod 5
            // when mirror-stored (side s joins vertices s−1 and s: a =
            // E→A, b = A→B, c = B→C, d = C→D, e = D→E).
            let l = if MIRROR[k] { (4 - e) % 5 } else { (e + 1) % 5 };
            l as u8
        };
        let mut done = [false; 40];
        let mut n_orbits = 0;
        let mut side_pairs: BTreeMap<(u8, u8), usize> = BTreeMap::new();
        for idx in 0..40 {
            if done[idx] {
                continue;
            }
            let (k, e) = (idx / 5, idx % 5);
            let pts = &cells[k];
            let (a, b) = (pts[e], pts[(e + 1) % 5]);
            // No same-direction partner: every torus pair is reverse-
            // glued (edge-to-edge).
            let same_dir: Vec<(usize, usize)> = partners
                .get(&(ukey(a), ukey(b)))
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter(|x| *x != (k, e))
                .collect();
            assert!(
                same_dir.is_empty(),
                "edge ({k},{e}) has a same-direction partner {same_dir:?}"
            );
            let list: Vec<(usize, usize)> = partners
                .get(&(ukey(b), ukey(a)))
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter(|x| *x != (k, e))
                .collect();
            assert_eq!(
                list.len(),
                1,
                "edge ({k},{e}) must have exactly one glued partner, got {list:?}"
            );
            let (j, f) = list[0];
            let jidx = j * 5 + f;
            assert!(!done[jidx], "edge ({j},{f}) was already paired");
            done[idx] = true;
            done[jidx] = true;
            n_orbits += 1;
            // Partner endpoints: a' = b + λ, b' = a + λ for one lattice
            // vector λ (0 for interior edges, ±w for boundary edges).
            let (pj, qj) = (cells[j][f], cells[j][(f + 1) % 5]);
            let lam = pj - b;
            let ulam = u_of(lam);
            let frac = |x: f64| x.rem_euclid(1.0).min(1.0 - x.rem_euclid(1.0));
            assert!(
                frac(ulam.x) < 1e-6 && frac(ulam.y) < 1e-6,
                "glued edges ({k},{e}) and ({j},{f}): shift {ulam:?} not a lattice vector"
            );
            let jitter = (a + lam - qj).length();
            assert!(
                jitter < 1e-9,
                "glued edges ({k},{e}) and ({j},{f}) disagree mod lattice by {jitter}"
            );
            assert!(
                (lens[idx] - lens[jidx]).abs() < 1e-9,
                "glued edges ({k},{e}) and ({j},{f}) must have equal length"
            );
            let (la, lb) = (label(k, e), label(j, f));
            let pair = if la <= lb { (la, lb) } else { (lb, la) };
            *side_pairs.entry(pair).or_insert(0) += 1;
        }
        assert_eq!(n_orbits, 20, "twenty edge orbits on the torus");
        let want_pairs: BTreeMap<(u8, u8), usize> = [
            ((0u8, 0u8), 4usize), // a-a
            ((1, 2), 4), // b-c
            ((1, 3), 4), // b-d
            ((2, 2), 2), // c-c
            ((3, 4), 4), // d-e
            ((4, 4), 2), // e-e
        ]
        .into_iter()
        .collect();
        assert_eq!(side_pairs, want_pairs, "glued side-pair census (a,b,c,d,e indices)");
        // Vertex orbits mod lattice and their stars.
        let mut orbits: BTreeMap<PtKey, Vec<(usize, usize)>> = BTreeMap::new();
        for (k, pts) in cells.iter().enumerate() {
            for (v, &p) in pts.iter().enumerate() {
                orbits.entry(ukey(p)).or_default().push((k, v));
            }
        }
        assert_eq!(orbits.len(), 12, "twelve vertex orbits on the torus");
        let mut sizes: Vec<usize> = orbits.values().map(|o| o.len()).collect();
        sizes.sort_unstable();
        assert_eq!(
            sizes,
            vec![3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4],
            "orbit sizes (eight 3s and four 4s)"
        );
        let mut stars: Vec<Vec<i64>> = Vec::new();
        for corners in orbits.values() {
            let mut tens: Vec<i64> = corners
                .iter()
                .map(|&(k, v)| {
                    (angle_at(&t.cells[k].poly, v).to_degrees() * 10.0).round() as i64
                })
                .collect();
            tens.sort_unstable();
            stars.push(tens);
        }
        stars.sort();
        let want_stars: Vec<Vec<i64>> = [
            vec![791, 791, 818, 1200],
            vec![791, 791, 818, 1200],
            vec![791, 791, 818, 1200],
            vec![791, 791, 818, 1200],
            vec![818, 1391, 1391],
            vec![818, 1391, 1391],
            vec![818, 1391, 1391],
            vec![818, 1391, 1391],
            vec![1200, 1200, 1200],
            vec![1200, 1200, 1200],
            vec![1200, 1200, 1200],
            vec![1200, 1200, 1200],
        ]
        .to_vec();
        assert_eq!(stars, want_stars, "vertex-orbit stars (degree tenths)");
    }

    /// The wallpaper group is pgg: the half-turn about the 2-fold centre
    /// c0 = (−1/2, 25√3/2) preserves the tiling exactly, and the two
    /// perpendicular glide reflections g1 = (M_u, t1 = (14, 14√3)) and
    /// g2 = (M_v, t2 = (57, 17√3)), with M_u the reflection in the w1
    /// direction, M_v = −M_u the reflection in the w2 direction, and axes
    /// along the lattice directions, preserve it (with every lattice-
    /// shifted variant, and so do the inverses), g1² = w1, g2² = −w2.
    /// Each glide shifts by exactly half a primitive translation along
    /// its axis (g1 by +w1/2, g2 by −w2/2), so no power of it is a pure
    /// reflection: none of the 20 candidates (M, i·w) with whole lattice
    /// shifts, the two perpendicular-part candidates, or the two
    /// quarter-turn probes maps the prototile onto a tile.  All
    /// generators are exact Q(√3).
    #[test]
    fn pentagon_8_pgg_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let s = s3();
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let c0 = v2(-0.5, 12.5 * s);
        // A window symmetric about c0: the half-turn must give exact
        // set equality of the tile patch.
        let ra = BBox::new(
            v2(c0.x - 30.0, c0.y - 24.0),
            v2(c0.x + 30.0, c0.y + 24.0),
        );
        check_symmetry(&t, &p, (-1.0, 0.0, 0.0, -1.0), c0 * 2.0, ra, true, "half-turn about c0");
        // M_u = reflection in the w1 direction (rows (−1/7, 4√3/7),
        // (4√3/7, 1/7)); M_v = −M_u = reflection in the w2 direction.
        let mu = (-1.0 / 7.0, 4.0 * s / 7.0, 4.0 * s / 7.0, 1.0 / 7.0);
        let mv = (1.0 / 7.0, -4.0 * s / 7.0, -4.0 * s / 7.0, -1.0 / 7.0);
        let t1 = v2(14.0, 14.0 * s);
        let t2 = v2(57.0, 17.0 * s);
        // Exact generator identities (floats of exact Q(√3) expressions):
        // g1² = w1, g2² = −w2.
        let gs1 = v2((mu.0 + 1.0) * t1.x + mu.1 * t1.y, mu.2 * t1.x + (mu.3 + 1.0) * t1.y);
        assert!((gs1 - w1).length() < 1e-9, "g1² must equal w1 exactly");
        let gs2 = v2((mv.0 + 1.0) * t2.x + mv.1 * t2.y, mv.2 * t2.x + (mv.3 + 1.0) * t2.y);
        assert!((gs2 + w2).length() < 1e-9, "g2² must equal −w2 exactly");
        // Each glide shifts by exactly half a primitive translation
        // along its axis (g1 by +w1/2, g2 by −w2/2) — no power is a
        // reflection.
        let u1 = w1 * (1.0 / w1.length());
        let u2 = w2 * (1.0 / w2.length());
        assert!(
            (2.0 * t1.dot(u1) / w1.length() - 1.0).abs() < 1e-9,
            "g1 must shift +w1/2"
        );
        assert!(
            (2.0 * t2.dot(u2) / w2.length() + 1.0).abs() < 1e-9,
            "g2 must shift −w2/2"
        );
        // The perpendicular parts of the glide shifts (a pure reflection
        // would keep them zero) are nonzero and exact.
        let p1 = t1 - u1 * t1.dot(u1);
        let p2 = t2 - u2 * t2.dot(u2);
        assert!((p1 - v2(-4.0, 2.0 * s)).length() < 1e-9, "t1⊥ = (−4, 2√3)");
        assert!((p2 - v2(39.0, 26.0 * s)).length() < 1e-9, "t2⊥ = (39, 26√3)");
        // The glides and their inverses preserve the tiling (the image
        // window is tilted, so check the image set is contained in the
        // tilted-window patch).
        check_symmetry(&t, &p, mu, t1, ra, false, "glide g1 (axis ∥ w1)");
        check_symmetry(
            &t,
            &p,
            mu,
            v2(-(mu.0 * t1.x + mu.1 * t1.y), -(mu.2 * t1.x + mu.3 * t1.y)),
            ra,
            false,
            "glide g1⁻¹",
        );
        check_symmetry(&t, &p, mv, t2, ra, false, "glide g2 (axis ∥ w2)");
        check_symmetry(
            &t,
            &p,
            mv,
            v2(-(mv.0 * t2.x + mv.1 * t2.y), -(mv.2 * t2.x + mv.3 * t2.y)),
            ra,
            false,
            "glide g2⁻¹",
        );
        // Big patch for tile-image membership: every probe image below
        // has its AABB intersecting the region, so every assertion is
        // non-vacuous.
        let big = BBox::new(v2(-80.0, -66.0 * s), v2(100.0, 66.0 * s));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        let set: HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();
        // g1 and g2 map the prototile exactly onto cells 4 and 7.
        let img4: Vec<Vec2> = prot.iter().map(|q| apply_iso(mu, t1, *q)).collect();
        let img7: Vec<Vec2> = prot.iter().map(|q| apply_iso(mv, t2, *q)).collect();
        assert_eq!(vkey(&img4), vkey(&t.cells[4].poly.pts), "g1 must map T1 exactly onto T5");
        assert_eq!(vkey(&img7), vkey(&t.cells[7].poly.pts), "g2 must map T1 exactly onto T8");
        // Sanity: pure lattice translations are symmetries.
        for i in -2..=2 {
            for w in [w1, w2] {
                let img: Vec<Vec2> = prot.iter().map(|q| *q + w * i as f64).collect();
                assert!(
                    set.contains(&vkey(&img)),
                    "T1 + {i}·lattice vector must be a tile"
                );
            }
        }
        // Lattice-shifted glides still map the prototile onto a tile.
        for (i, jj) in [(0.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (1.0, 0.0)] {
            let sh = w1 * i + w2 * jj;
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(mu, t1 + sh, *q)).collect();
            assert!(
                set.contains(&vkey(&img)),
                "(g1 + {i}·w1 + {jj}·w2) must map T1 onto a tile"
            );
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(mv, t2 + sh, *q)).collect();
            assert!(
                set.contains(&vkey(&img)),
                "(g2 + {i}·w1 + {jj}·w2) must map T1 onto a tile"
            );
        }
        // No pure reflection: every candidate (M, i·w) — zero or whole
        // lattice shifts along either axis — must fail to map the
        // prototile onto a tile (all true symmetries with linear part M
        // shift half a primitive translation along the axis).
        for i in -2..=2 {
            for (m, w, mname, wname) in [
                (mu, w1, "M_u", "w1"),
                (mu, w2, "M_u", "w2"),
                (mv, w1, "M_v", "w1"),
                (mv, w2, "M_v", "w2"),
            ] {
                let img: Vec<Vec2> = prot
                    .iter()
                    .map(|q| apply_iso(m, w * i as f64, *q))
                    .collect();
                assert!(
                    !set.contains(&vkey(&img)),
                    "({mname}, {i}·{wname}) must not be a symmetry (no mirrors in pgg)"
                );
            }
        }
        // The perpendicular-part candidates are pure reflections:
        // neither is a symmetry.
        for (m, tr, nm) in [(mu, p1, "(M_u, t1⊥)"), (mv, p2, "(M_v, t2⊥)")] {
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, tr, *q)).collect();
            assert!(
                !set.contains(&vkey(&img)),
                "{nm} must not be a symmetry (no mirrors in pgg)"
            );
        }
        // No quarter-turn either (about the origin and about the 2-fold
        // centre).
        for (nm, tr) in [
            ("quarter-turn about the origin", Vec2::ZERO),
            ("quarter-turn about c0", v2(c0.x + c0.y, c0.y - c0.x)),
        ] {
            let img: Vec<Vec2> = prot
                .iter()
                .map(|q| apply_iso((0.0, -1.0, 1.0, 0.0), tr, *q))
                .collect();
            assert!(!set.contains(&vkey(&img)), "{nm} must not be a symmetry");
        }
    }

    /// 2-isohedral: seeds 0/1 are the two full-group isohedral classes —
    /// the half-turn and both glide classes permute the eight cells
    /// mod-lattice by the closed permutations, the group they generate
    /// has exactly two orbits on the cells ({0,3,4,7} / {1,2,5,6},
    /// matching the seeds), both seeds appear in every patch, and every
    /// generated tile keeps the prototile side multiset
    /// {14, 14, 14, 14, 6√7}.
    #[test]
    fn pentagon_8_isohedral_classes() {
        let t = spec();
        let s = s3();
        let mut counts = [0usize; 2];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(counts, [4, 4], "four tiles per isohedral class");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        let u_of = |p: Vec2| {
            v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
        };
        let mod1 = |x: f64| (x + 0.5).rem_euclid(1.0) - 0.5;
        let c0 = v2(-0.5, 12.5 * s);
        let mu = (-1.0 / 7.0, 4.0 * s / 7.0, 4.0 * s / 7.0, 1.0 / 7.0);
        let mv = (1.0 / 7.0, -4.0 * s / 7.0, -4.0 * s / 7.0, -1.0 / 7.0);
        let t1 = v2(14.0, 14.0 * s);
        let t2 = v2(57.0, 17.0 * s);
        let cents: Vec<Vec2> = t.cells.iter().map(|c| cent(&c.poly.pts)).collect();
        // A symmetry (M, t) permutes the cells: cell k's centroid maps to
        // cell j's centroid mod lattice, for exactly one j.
        let perm_of = |m: (f64, f64, f64, f64), tr: Vec2| -> Vec<usize> {
            let mut perm: Vec<usize> = Vec::new();
            for k in 0..8 {
                let img = apply_iso(m, tr, cents[k]);
                let mut hits: Vec<usize> = Vec::new();
                for (j, &cj) in cents.iter().enumerate() {
                    let du = u_of(img - cj);
                    if mod1(du.x).abs() < 1e-6 && mod1(du.y).abs() < 1e-6 {
                        hits.push(j);
                    }
                }
                assert_eq!(hits.len(), 1, "cell {k} must have exactly one image, got {hits:?}");
                perm.push(hits[0]);
            }
            perm
        };
        let perms = [
            perm_of((-1.0, 0.0, 0.0, -1.0), c0 * 2.0),
            perm_of(mu, t1),
            perm_of(mv, t2),
        ];
        assert_eq!(perms[0], vec![3, 2, 1, 0, 7, 6, 5, 4], "half-turn perm");
        assert_eq!(perms[1], vec![4, 5, 6, 7, 0, 1, 2, 3], "glide g1 perm");
        assert_eq!(perms[2], vec![7, 6, 5, 4, 3, 2, 1, 0], "glide g2 perm");
        // Two full-group orbits on the cells, exactly the seed classes.
        let mut seen = [false; 8];
        let mut orbits: Vec<Vec<usize>> = Vec::new();
        for st in 0..8 {
            if seen[st] {
                continue;
            }
            let mut orb = vec![st];
            seen[st] = true;
            let mut head = 0;
            while head < orb.len() {
                let x = orb[head];
                head += 1;
                for pm in &perms {
                    let y = pm[x];
                    if !seen[y] {
                        seen[y] = true;
                        orb.push(y);
                    }
                }
            }
            orb.sort_unstable();
            orbits.push(orb);
        }
        assert_eq!(
            orbits,
            vec![vec![0, 3, 4, 7], vec![1, 2, 5, 6]],
            "two full-group orbits"
        );
        for orb in &orbits {
            let sd = t.cells[orb[0]].seed;
            assert!(
                orb.iter().all(|&k| t.cells[k].seed == sd),
                "orbit mixes isohedral seeds"
            );
        }
        assert_ne!(t.cells[orbits[0][0]].seed, t.cells[orbits[1][0]].seed, "orbits differ");
        // Generated patches: both classes present, side multiset kept.
        let params = ParamSet::default_for(t.params());
        let rp = t.generate(
            &params,
            &Query {
                region: BBox::new(v2(-60.5, -45.5 * s), v2(60.5, 45.5 * s)),
            },
        );
        assert!(rp.message.is_none(), "{:?}", rp.message);
        let mut seen_s = [false; 2];
        for tile in &rp.tiles {
            let sd = tile.seed as usize;
            assert!(sd < 2, "seeds must stay in {{0, 1}}");
            seen_s[sd] = true;
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| {
                    (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length()
                })
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [14.0, 14.0, 14.0, 14.0, 6.0 * 7.0f64.sqrt()];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!(
                    (got - w).abs() < 1e-12,
                    "seed {sd} side {got} ∉ {{14 × 4, 6√7}}"
                );
            }
        }
        assert!(seen_s[0] && seen_s[1], "both isohedral classes must appear");
    }

    /// The eight cells realise both chiralities of the (chiral) type-8
    /// prototile: cell 0 matches the prototile's cyclic (side, angle)
    /// sequence forward but not reversed, every cell matches exactly one
    /// of the two, and the counts are four forward and four reversed —
    /// the mirror half of the primitive unit (T2, T3, T5, T8).
    #[test]
    fn pentagon_8_enantiomer_mix() {
        let t = spec();
        let proto = &t.cells[0].poly;
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        let match_fwd = |cs: &[f64], ca: &[f64]| {
            (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            })
        };
        let match_rev = |cs: &[f64], ca: &[f64]| {
            (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(rot + 9 - i) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(rot + 5 - i) % 5]).abs() < 1e-9
                })
            })
        };
        // The prototile itself: forward match only (the family is chiral).
        assert!(match_fwd(&ps, &pa), "prototile must match forward");
        assert!(!match_rev(&ps, &pa), "prototile must be chiral (no reversed match)");
        let mut n_fwd = 0;
        let mut n_rev = 0;
        for (k, cell) in t.cells.iter().enumerate() {
            let cs: Vec<f64> = (0..5).map(|i| side_of(&cell.poly, i)).collect();
            let ca: Vec<f64> = (0..5).map(|i| angle_at(&cell.poly, i)).collect();
            let f = match_fwd(&cs, &ca);
            let r = match_rev(&cs, &ca);
            assert!(f != r, "cell {k} must match exactly one chirality");
            if f {
                n_fwd += 1;
            } else {
                n_rev += 1;
            }
        }
        assert_eq!((n_fwd, n_rev), (4, 4), "four direct and four mirror tiles");
    }
}

#[cfg(test)]
mod pentagon6_tests {
    use super::pentagon6::{s3, tile_area, two_fold_centre};
    use super::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon6::spec;
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// (x2/2, y2·√3/2) — every tile vertex lies on the half-integer ×
    /// integer·√3/2 grid, so the helper takes doubled numerators (same
    /// grid as the module's `c` helper).
    fn c(x2: f64, y2: f64) -> Vec2 {
        v2(x2 / 2.0, y2 * s3() / 2.0)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        v2(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Vertex-set key on a 1e-6 grid (every coordinate lies on the
    /// half-integer × integer·√3/2 grid, so distinct tiling vertices are
    /// ≥ 1/2 apart, and two computations of a shared joint agree to
    /// ~1e-15).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Single-vertex key on the same 1e-6 grid.
    fn pkey(p: Vec2) -> (i64, i64) {
        ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64)
    }

    /// Basis coordinates of `p` in the lattice (w1, w2).
    fn u_of(p: Vec2, w1: Vec2, w2: Vec2) -> Vec2 {
        let det = w1.cross(w2);
        v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
    }

    /// True when `v` is a lattice vector (both basis coordinates
    /// integral).
    fn is_lattice(v: Vec2, w1: Vec2, w2: Vec2) -> bool {
        let u = u_of(v, w1, w2);
        let f = |x: f64| x.rem_euclid(1.0).min(1.0 - x.rem_euclid(1.0));
        f(u.x) < 1e-9 && f(u.y) < 1e-9
    }

    /// Verify that the isometry g = (M, t) preserves the tiling: the
    /// image of every tile intersecting `ra` must be a tile intersecting
    /// `rb` (the axis-aligned box of g(ra)).  With `exact = true` the
    /// image set must equal the `rb` tile set (the case g(ra) = rb).
    fn check_symmetry(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            v2(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            v2(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ra_tiles = t.generate(params, &Query { region: ra });
        let rb_tiles = t.generate(params, &Query { region: rb });
        assert!(ra_tiles.message.is_none(), "{label}: {:?}", ra_tiles.message);
        assert!(rb_tiles.message.is_none(), "{label}: {:?}", rb_tiles.message);
        assert!(!ra_tiles.tiles.is_empty(), "{label}: no tiles");
        let set_b: HashSet<Vec<(i64, i64)>> =
            rb_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let set_a: HashSet<Vec<(i64, i64)>> = ra_tiles
            .tiles
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(
            set_a.len(),
            ra_tiles.tiles.len(),
            "{label}: isometry must be injective on the patch"
        );
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    /// Directed-edge key: keys of its two endpoints on the 1e-6 grid.
    type EdgeKey = ((i64, i64), (i64, i64));

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    /// Centroid of a stored cell (mean of its five vertices).
    fn cent(pts: &[Vec2]) -> Vec2 {
        let s = pts.iter().fold(Vec2::ZERO, |acc, &p| acc + p);
        s / 5.0
    }

    #[test]
    fn pentagon_6_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        let s = s3();
        check_coverage(
            &t,
            "pentagon-6",
            &p,
            BBox::new(v2(-20.0, -15.0 * s), v2(20.0, 15.0 * s)),
        );
    }

    #[test]
    fn pentagon_6_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-6(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(7.0, 7.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_6_deterministic() {
        check_det(&spec(), "pentagon-6", BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)));
    }

    /// Unit structure of the primitive cell: 4 tiles of the exact Q(√3)
    /// prototile (A = B = D = E = 120°, C = 60°; a = d = e = 1, b = c =
    /// 2; area 7√3/4) on the oblique lattice w1 = (1, 2√3), w2 =
    /// (−3, √3), with det = 4 × tile area = 7√3, |w1|² = 13, |w2|² = 12,
    /// w1·w2 = 3.  Every cell is a direct (rotated) copy of the
    /// prototile with the canonical alignment — vertex i of every cell
    /// carries prototile corner (A, B, C, D, E)[i] (s_k = 0 for all k) —
    /// CCW, no two cells coincide, and the family-condition identities
    /// hold to float precision.
    #[test]
    fn pentagon_6_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-6");
        assert_eq!(t.cells.len(), 4, "the primitive unit holds four tiles");
        assert_eq!(t.lattice[0], c(2.0, 4.0), "w1 = (1, 2√3)");
        assert_eq!(t.lattice[1], c(-6.0, 2.0), "w2 = (−3, √3)");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        assert!(
            (det - 4.0 * tile_area()).abs() < 1e-9,
            "det {det} ≠ 4 × tile area = 7√3"
        );
        assert!((w1.dot(w1) - 13.0).abs() < 1e-12, "|w1|² = 1 + 12 = 13");
        assert!((w2.dot(w2) - 12.0).abs() < 1e-12, "|w2|² = 9 + 3 = 12");
        assert!((w1.dot(w2) - 3.0).abs() < 1e-12, "w1·w2 = −3 + 6 = 3");
        assert!(w1.dot(w2).abs() > 1e-9, "the lattice must be oblique");
        // Seeds follow the two full-group isohedral orbits {T0,T1} /
        // {T2,T3}.
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 0, 1, 1], "isohedral seed pattern");
        // The shipped prototile is T0, corner A at the origin:
        // A = (0,0), B = (1,0), C = (2,√3), D = (0,√3), E = (−1/2,√3/2).
        let proto = &t.cells[0].poly;
        let pin = [c(0.0, 0.0), c(2.0, 0.0), c(4.0, 2.0), c(0.0, 2.0), c(-1.0, 1.0)];
        for (i, &want) in pin.iter().enumerate() {
            assert_eq!(proto.pts[i], want, "prototile corner {i}");
        }
        // Exact corner angles in vertex order A, B, C, D, E (C = 60°).
        for (i, want) in [(0usize, 120.0f64), (1, 120.0), (2, 60.0), (3, 120.0), (4, 120.0)] {
            let got = angle_at(proto, i).to_degrees();
            assert!((got - want).abs() < 1e-9, "angle at vertex {i}: {got} ≠ {want}");
        }
        // Exact side lengths: a = d = e = 1, b = c = 2.
        for (i, want) in [(0usize, 1.0f64), (1, 2.0), (2, 2.0), (3, 1.0), (4, 1.0)] {
            assert!(
                (side_of(proto, i) - want).abs() < 1e-12,
                "side {i} (length {want})"
            );
        }
        // Type-6 family identities, from the stored values: A = 2C,
        // C + E = 180°, a = d = e, b = c.
        assert!(
            (angle_at(proto, 0) - 2.0 * angle_at(proto, 2)).abs() < 1e-9,
            "A = 2C"
        );
        assert!(
            (angle_at(proto, 2) + angle_at(proto, 4) - std::f64::consts::PI).abs() < 1e-9,
            "C + E = 180°"
        );
        assert!(
            (side_of(proto, 0) - side_of(proto, 3)).abs() < 1e-12
                && (side_of(proto, 0) - side_of(proto, 4)).abs() < 1e-12,
            "a = d = e"
        );
        assert!((side_of(proto, 1) - side_of(proto, 2)).abs() < 1e-12, "b = c");
        // Every cell: CCW, exact area, five vertices, and the canonical
        // alignment (vertex i carries corner (A,B,C,D,E)[i], edge e the
        // side e) — a direct congruent copy of the prototile (the tile
        // is achiral, so direct copies exist for every cell).
        let mut seen: HashSet<Vec<(i64, i64)>> = HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            assert!(
                (cell.poly.area() - tile_area()).abs() < 1e-12,
                "cell {k} area ≠ 7√3/4"
            );
            assert_eq!(cell.poly.len(), 5, "cell {k} vertex count");
            for (i, want) in [(0usize, 120.0f64), (1, 120.0), (2, 60.0), (3, 120.0), (4, 120.0)] {
                let got = angle_at(&cell.poly, i).to_degrees();
                assert!(
                    (got - want).abs() < 1e-9,
                    "cell {k} vertex {i} angle {got} ∉ (120,120,60,120,120)"
                );
            }
            for (i, want) in [(0usize, 1.0f64), (1, 2.0), (2, 2.0), (3, 1.0), (4, 1.0)] {
                assert!(
                    (side_of(&cell.poly, i) - want).abs() < 1e-12,
                    "cell {k} side {i} (length {want})"
                );
            }
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
        }
    }

    /// Window census (default parameters): culling is the tile AABB
    /// intersecting the region (+1e-9); the quarter- and half-integer
    /// window bounds keep every culling decision unambiguous, so the
    /// counts are exact and match the independent Python emulation of
    /// the lattice.rs culling.
    #[test]
    fn pentagon_6_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::P2);
        assert_eq!(t.lattice_kind, LatticeKind::Oblique);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the T0 cell (seed 0)");
        let p = ParamSet::default_for(t.params());
        let s = s3();
        for (region, want) in [
            (
                BBox::new(v2(-60.25, -45.5 * s / 2.0), v2(60.25, 45.5 * s / 2.0)),
                3268,
            ),
            (BBox::new(v2(-30.25, -22.5 * s / 2.0), v2(30.25, 22.5 * s / 2.0)), 864),
            (BBox::new(v2(-10.25, -7.5 * s / 2.0), v2(10.25, 7.5 * s / 2.0)), 112),
            (BBox::new(v2(-2.25, -1.5 * s / 2.0), v2(3.25, 4.5 * s / 2.0)), 20),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }

    /// Edge-to-edge at the pentagon level over the 787-tile patch: no
    /// directed side is shared by two tiles (3935 directed edges, 0
    /// duplicates), every side whose midpoint lies strictly inside the
    /// window (3424 of them) is matched by exactly one full side of one
    /// neighbour (its reverse), and no vertex falls in the interior of a
    /// non-incident side.
    #[test]
    fn pentagon_6_edge_to_edge_gluing() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let s = s3();
        let big = BBox::new(v2(-20.25, -15.0 * s), v2(20.25, 15.0 * s));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 787, "patch size");
        let mut dirs: std::collections::HashMap<EdgeKey, usize> =
            std::collections::HashMap::new();
        for tile in &r.tiles {
            let n = tile.poly.len();
            for i in 0..n {
                let k = (pkey(tile.poly.pts[i]), pkey(tile.poly.pts[(i + 1) % n]));
                *dirs.entry(k).or_insert(0) += 1;
            }
        }
        assert_eq!(dirs.len(), 3935, "3935 directed sides, all unique");
        assert!(
            dirs.values().all(|&c| c == 1),
            "a directed side may belong to only one tile"
        );
        let mut interior = 0;
        for tile in &r.tiles {
            let n = tile.poly.len();
            for i in 0..n {
                let (a, b) = (tile.poly.pts[i], tile.poly.pts[(i + 1) % n]);
                let mid = (a + b) * 0.5;
                let d = (mid.x - big.min.x)
                    .min(big.max.x - mid.x)
                    .min(mid.y - big.min.y)
                    .min(big.max.y - mid.y);
                if d > 1e-6 {
                    interior += 1;
                    assert_eq!(
                        dirs.get(&(pkey(b), pkey(a))),
                        Some(&1),
                        "interior side {a} -> {b} lacks its full-edge partner"
                    );
                }
            }
        }
        assert_eq!(
            interior,
            3424,
            "interior directed sides (each paired with its reverse)"
        );
        // No T-junction: no vertex in the interior of a non-incident edge.
        let verts: HashSet<(i64, i64)> =
            r.tiles.iter().flat_map(|tile| tile.poly.pts.iter().map(|&p| pkey(p))).collect();
        assert_eq!(verts.len(), 1286, "distinct vertices in the patch");
        for tile in &r.tiles {
            let n = tile.poly.len();
            for i in 0..n {
                let (a, b) = (tile.poly.pts[i], tile.poly.pts[(i + 1) % n]);
                let (dx, dy) = (b.x - a.x, b.y - a.y);
                let l2 = dx * dx + dy * dy;
                for vk in verts.iter().filter(|k| **k != pkey(a) && **k != pkey(b)) {
                    let (px, py) = (vk.0 as f64 / 1e6, vk.1 as f64 / 1e6);
                    let tt = (((px - a.x) * dx + (py - a.y) * dy) / l2).clamp(0.0, 1.0);
                    if tt == 0.0 || tt == 1.0 {
                        continue;
                    }
                    let q = a + (b - a) * tt;
                    assert!(
                        (px - q.x).hypot(py - q.y) > 1e-7,
                        "vertex {vk:?} lies in the interior of side {a} -> {b}"
                    );
                }
            }
        }
    }

    /// Vertex census over the 787-tile patch: every window-interior
    /// vertex (1022 of the 1286 distinct vertices) is a complete star of
    /// incident tile angles summing to exactly 360°, and the stars are
    /// exactly the two closed types {60°, 60°, 120°, 120°} ("AABB", 340)
    /// and {120°, 120°, 120°} ("BBB", 682).
    #[test]
    fn pentagon_6_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let s = s3();
        let big = BBox::new(v2(-20.25, -15.0 * s), v2(20.25, 15.0 * s));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        let verts: HashSet<(i64, i64)> =
            r.tiles.iter().flat_map(|tile| tile.poly.pts.iter().map(|&pt| pkey(pt))).collect();
        // Corner cosines: 60° -> +1/2, 120° -> −1/2.
        let bins = [0.5f64, -0.5f64];
        let mut stars: BTreeMap<String, usize> = BTreeMap::new();
        let mut checked = 0;
        for vk in &verts {
            let (px, py) = (vk.0 as f64 / 1e6, vk.1 as f64 / 1e6);
            let d = (px - big.min.x)
                .min(big.max.x - px)
                .min(py - big.min.y)
                .min(big.max.y - py);
            if d < 1e-6 {
                continue; // boundary: incident tiles may be culled
            }
            // Any tile having this corner has its AABB containing the
            // strictly interior point, hence was generated: the star is
            // complete.
            let mut letters = String::new();
            let mut sum = 0.0;
            for tile in &r.tiles {
                let n = tile.poly.len();
                for i in 0..n {
                    if pkey(tile.poly.pts[i]) != *vk {
                        continue;
                    }
                    let ang = angle_at(&tile.poly, i);
                    sum += ang;
                    let c = ang.cos();
                    let j = (0..2)
                        .min_by_key(|&j| ((c - bins[j]).abs() * 1e12) as i64)
                        .unwrap();
                    assert!((c - bins[j]).abs() < 1e-9, "stray corner cosine {c}");
                    letters.push(['A', 'B'][j]);
                }
            }
            assert!(
                (sum - std::f64::consts::TAU).abs() < 1e-9,
                "vertex {vk:?} angles sum to {sum} ≠ 2π"
            );
            let mut cs: Vec<char> = letters.chars().collect();
            cs.sort_unstable();
            let star: String = cs.into_iter().collect();
            assert!(
                star == "AABB" || star == "BBB",
                "unexpected vertex star {star} at {vk:?}"
            );
            *stars.entry(star).or_insert(0) += 1;
            checked += 1;
        }
        assert_eq!(checked, 1022, "interior vertex census");
        assert_eq!(stars.get("AABB").copied().unwrap_or(0), 340, "{{60,60,120,120}} stars");
        assert_eq!(stars.get("BBB").copied().unwrap_or(0), 682, "{{120,120,120}} stars");
    }

    /// Gluing census of the 4-cell unit: the tiling is edge-to-edge at
    /// the pentagon level — every one of the 20 stored sides has exactly
    /// one full reverse neighbour among the stored sides (possibly
    /// shifted by a lattice vector): the partner map is an involution
    /// with 10 side-pair orbits, 4 pairs gluing in the unit (shift 0)
    /// and 6 pairs across the shifts ±w1, ±w2, ±(w1−w2), side-pair
    /// histogram a-a:1, a-d:2, b-b:1, b-c:2, c-c:1, d-e:2, e-e:1
    /// (every stored edge e of every cell carries prototile side e —
    /// the canonical alignment, s_k = 0 for all k, no mirror table).
    /// Mod the unit lattice the 20 directed sides wrap to 18 directed
    /// classes, with exactly one double-covered arc: the
    /// (2,√3)→(0,√3)≡(3,0) arc carries two lifts in each direction
    /// (T0e2, T3e1 / T1e2, T2e1; their other endpoints are lattice
    /// translates, (3,0) = (0,√3) − w2).  The 20 corners form 6 vertex
    /// orbits (sizes 3, 3, 3, 3, 4, 4) whose stars are 2×(60.0, 60.0,
    /// 120.0, 120.0) and 4×(120.0, 120.0, 120.0).
    #[test]
    fn pentagon_6_cell_gluing_census() {
        type PtKey = (i64, i64);
        type EdgeKey2 = (PtKey, PtKey);
        let t = spec();
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        assert!(det > 0.0, "the shipped basis must be positively oriented");
        let ukey = |p: Vec2| {
            let u = u_of(p, w1, w2);
            (
                ((u.x.rem_euclid(1.0) * 1e6).round() as i64) % 1_000_000,
                ((u.y.rem_euclid(1.0) * 1e6).round() as i64) % 1_000_000,
            )
        };
        let cells: Vec<Vec<Vec2>> = t.cells.iter().map(|c| c.poly.pts.clone()).collect();
        // (a) Physical partner of the side (k,e) (tail a, head b): the
        // unique (j,f) != (k,e) with (j,f) + λ = reverse of (k,e) for a
        // lattice vector λ.
        let mut partner: BTreeMap<usize, (usize, usize, Vec2)> = BTreeMap::new();
        let mut lens: Vec<f64> = Vec::new();
        for (k, pts) in cells.iter().enumerate() {
            for e in 0..5 {
                let (a, b) = (pts[e], pts[(e + 1) % 5]);
                lens.push((a - b).length());
                let mut hits: Vec<(usize, usize, Vec2)> = Vec::new();
                for (j, pts2) in cells.iter().enumerate() {
                    for f in 0..5 {
                        if (j, f) == (k, e) {
                            continue;
                        }
                        let lam = b - pts2[f];
                        if is_lattice(lam, w1, w2)
                            && (pts2[(f + 1) % 5] + lam - a).length() < 1e-9
                        {
                            hits.push((j, f, lam));
                        }
                    }
                }
                assert_eq!(
                    hits.len(),
                    1,
                    "side ({k},{e}) must have exactly one full reverse partner, got {hits:?}"
                );
                partner.insert(k * 5 + e, hits.pop().unwrap());
            }
        }
        // (b) The partner map is an involution (with the inverse shift);
        // glued sides have equal length; every glue shift is one of
        // 0, ±w1, ±w2, ±(w1−w2), and exactly 4 pairs glue with shift 0.
        let in_set = |v: Vec2| {
            let near = |a: i64, b: i64| (v - (w1 * a as f64 + w2 * b as f64)).length() < 1e-9;
            [
                (0, 0),
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, -1),
                (-1, 1),
            ]
            .iter()
            .any(|&(a, b)| near(a, b))
        };
        let mut n_inunit = 0;
        let mut side_pairs: BTreeMap<(u8, u8), usize> = BTreeMap::new();
        for idx in 0..20 {
            let (k, e) = (idx / 5, idx % 5);
            let (j, f, lam) = partner[&idx];
            let pidx = j * 5 + f;
            assert!(
                partner[&pidx].0 == k && partner[&pidx].1 == e,
                "partner of ({k},{e}) is ({j},{f}) but back it is {:?}",
                partner[&pidx]
            );
            assert!(
                (partner[&pidx].2 + lam).length() < 1e-9,
                "inverse glue shift of ({k},{e}) must be −λ"
            );
            assert!(
                (lens[idx] - lens[pidx]).abs() < 1e-9,
                "glued sides ({k},{e}) and ({j},{f}) must have equal length"
            );
            assert!(
                in_set(lam),
                "glue shift {lam:?} of ({k},{e}) is outside 0, ±w1, ±w2, ±(w1−w2)"
            );
            if lam.length() < 1e-9 {
                n_inunit += 1;
            }
            // Canonical alignment: stored edge e carries prototile side
            // e for every cell; count each undirected orbit once.
            if idx < pidx {
                let pair = if e <= f { (e as u8, f as u8) } else { (f as u8, e as u8) };
                *side_pairs.entry(pair).or_insert(0) += 1;
            }
        }
        assert_eq!(n_inunit, 8, "exactly 4 pairs (8 sides) glue in the unit with shift 0");
        let want_pairs: BTreeMap<(u8, u8), usize> = [
            ((0u8, 0u8), 1usize), // a-a
            ((0, 3), 2),   // a-d
            ((1, 1), 1),   // b-b
            ((1, 2), 2),   // b-c
            ((2, 2), 1),   // c-c
            ((3, 4), 2),   // d-e
            ((4, 4), 1),   // e-e
        ]
        .into_iter()
        .collect();
        assert_eq!(side_pairs, want_pairs, "glued side-pair census (a,b,c,d,e indices)");
        // (c) Torus wrap: the 20 directed sides form 18 directed classes
        // mod the unit lattice — 16 single-lift classes and one
        // double-covered arc (with its reverse).
        let mut classes: BTreeMap<EdgeKey2, Vec<(usize, usize)>> = BTreeMap::new();
        for (k, pts) in cells.iter().enumerate() {
            for e in 0..5 {
                let (a, b) = (pts[e], pts[(e + 1) % 5]);
                classes.entry((ukey(a), ukey(b))).or_default().push((k, e));
            }
        }
        assert_eq!(classes.len(), 18, "18 directed torus classes");
        let mut double: BTreeMap<EdgeKey2, Vec<(usize, usize)>> = BTreeMap::new();
        for (key, lifts) in &classes {
            if lifts.len() > 1 {
                double.insert(*key, lifts.clone());
            }
        }
        assert_eq!(double.len(), 2, "exactly one double-covered arc (plus its reverse)");
        let q = ukey(c(0.0, 2.0)); // (0,√3), the class of (3,0): u = (3/7, 1/7)
        let p = ukey(c(4.0, 2.0)); // (2,√3): u = (5/7, 4/7)
        assert_eq!(
            double.get(&(q, p)),
            Some(&vec![(1, 2), (2, 1)]),
            "reverse double arc (0,√3)→(2,√3): lifts T1e2, T2e1 (tails differ by w2)"
        );
        assert_eq!(
            double.get(&(p, q)),
            Some(&vec![(0, 2), (3, 1)]),
            "double arc (2,√3)→(0,√3): lifts T0e2, T3e1 (heads differ by w2)"
        );
        // Vertex orbits mod lattice and their stars.
        let mut orbits: BTreeMap<PtKey, Vec<(usize, usize)>> = BTreeMap::new();
        for (k, pts) in cells.iter().enumerate() {
            for (v, &p) in pts.iter().enumerate() {
                orbits.entry(ukey(p)).or_default().push((k, v));
            }
        }
        assert_eq!(orbits.len(), 6, "six vertex orbits on the torus");
        let mut sizes: Vec<usize> = orbits.values().map(|o| o.len()).collect();
        sizes.sort_unstable();
        assert_eq!(sizes, vec![3, 3, 3, 3, 4, 4], "orbit sizes");
        let mut stars: Vec<Vec<i64>> = Vec::new();
        for corners in orbits.values() {
            let mut tens: Vec<i64> = corners
                .iter()
                .map(|&(k, v)| {
                    (angle_at(&t.cells[k].poly, v).to_degrees() * 10.0).round() as i64
                })
                .collect();
            tens.sort_unstable();
            stars.push(tens);
        }
        stars.sort();
        let want_stars: Vec<Vec<i64>> = [
            vec![600, 600, 1200, 1200],
            vec![600, 600, 1200, 1200],
            vec![1200, 1200, 1200],
            vec![1200, 1200, 1200],
            vec![1200, 1200, 1200],
            vec![1200, 1200, 1200],
        ]
        .to_vec();
        assert_eq!(stars, want_stars, "vertex-orbit stars (degree tenths)");
    }

    /// The wallpaper group is p2: the half-turn about the 2-fold centre
    /// c0 = (1/2, 0) preserves the tiling exactly, and the remaining
    /// three classes c0 + Λ/2 mod Λ — c0 + w1/2 = (1, √3), c0 + w2/2 =
    /// (−1, √3/2), c0 + (w1+w2)/2 = (−1/2, 3√3/2) — are exactly the
    /// remaining half-turn centre classes (every representative, with
    /// every ±w1, ±w2 shift, maps the cell-0 tile onto a tile).  The
    /// lattice points and the off-coset half-lattice points
    /// {0, w1/2, w2/2, (w1+w2)/2} are not centres (exhaustive search
    /// over all 14 candidate Λ/2 cosets finds exactly one centre coset).
    /// No mirror or glide: a symmetry g = t + R conjugates the
    /// translation group T = Λ to itself, so R must normalize Λ; a
    /// mirror's axis must permute the tiling's edge-direction set
    /// {0°, 60°, 120°} mod 180°, leaving exactly six candidate
    /// directions (0°, 30°, 60°, 90°, 120°, 150°), and none of them
    /// normalizes Λ.  (The 4-cell block check alone is not a
    /// completeness proof: it admits 4 spurious subset maps with
    /// R(Λ) ≠ Λ — the normalizer condition is the complete one.)  No
    /// 3- or 4-fold rotation normalizes Λ either, so the group is
    /// exactly p2.  All generators are exact Q(√3).
    #[test]
    fn pentagon_6_p2_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let s = s3();
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let c0 = two_fold_centre();
        assert_eq!(c0, v2(0.5, 0.0), "c0 = (1/2, 0)");
        // A window symmetric about c0: the half-turn must give exact
        // set equality of the tile patch.
        let ra = BBox::new(
            v2(c0.x - 16.0, c0.y - 12.0 * s),
            v2(c0.x + 16.0, c0.y + 12.0 * s),
        );
        check_symmetry(&t, &p, (-1.0, 0.0, 0.0, -1.0), c0 * 2.0, ra, true, "half-turn about c0");
        // Big patch for tile-image membership: every probe image below
        // lies inside this footprint, so membership in the generated set
        // is the exact question.
        let big = BBox::new(v2(-20.25, -15.0 * s), v2(20.25, 15.0 * s));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        let set: HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();
        // Sanity: pure lattice translations are symmetries.
        for i in -2..=2 {
            for w in [w1, w2] {
                let img: Vec<Vec2> = prot.iter().map(|q| *q + w * i as f64).collect();
                assert!(
                    set.contains(&vkey(&img)),
                    "T0 + {i}·lattice vector must be a tile"
                );
            }
        }
        // The one centre coset, c0 + Λ/2: all four representatives
        // (mod Λ) are centres, with every whole lattice shift.
        for h in [Vec2::ZERO, w1 * 0.5, w2 * 0.5, (w1 + w2) * 0.5] {
            let c = c0 + h;
            for i in -1..=1 {
                for j in -1..=1 {
                    let tr = c * 2.0 + w1 * i as f64 + w2 * j as f64;
                    let img: Vec<Vec2> =
                        prot.iter().map(|q| apply_iso((-1.0, 0.0, 0.0, -1.0), tr, *q)).collect();
                    assert!(
                        set.contains(&vkey(&img)),
                        "half-turn about {:?} + lattice must map T0 onto a tile",
                        c
                    );
                }
            }
        }
        // The off-coset half-lattice points are not centres (a half-turn
        // about one has a lattice-translation square, so it would have to
        // be a centre of the p2 group; the only centre coset is c0 +
        // Λ/2, and 0 ∉ c0 + Λ/2).
        for c in [Vec2::ZERO, w1 * 0.5, w2 * 0.5, (w1 + w2) * 0.5] {
            let img: Vec<Vec2> =
                prot.iter().map(|q| apply_iso((-1.0, 0.0, 0.0, -1.0), c * 2.0, *q)).collect();
            assert!(
                !set.contains(&vkey(&img)),
                "half-turn about {c:?} must not be a symmetry"
            );
        }
        // No mirror or glide: their linear part must normalize Λ, and a
        // mirror's axis direction must permute the edge-direction set
        // {0°, 60°, 120°} mod 180° — exactly the six directions below,
        // none of which normalizes Λ.
        for deg in [0.0, 30.0, 60.0, 90.0, 120.0, 150.0] {
            let th = deg * std::f64::consts::PI / 180.0;
            let (u, wgt) = (th.cos(), th.sin());
            let m = (
                2.0 * u * u - 1.0,
                2.0 * u * wgt,
                2.0 * u * wgt,
                2.0 * wgt * wgt - 1.0,
            );
            assert!(
                !(is_lattice(apply_iso(m, Vec2::ZERO, w1), w1, w2)
                    && is_lattice(apply_iso(m, Vec2::ZERO, w2), w1, w2)),
                "reflection in direction {deg}° must not normalize the lattice (no mirrors/glides)"
            );
        }
        // No 3- or 4-fold rotation either (wallpaper rotations have
        // order 2, 3, 4, or 6; the 6-fold and 3-fold share R120, and
        // R60 is checked directly): none normalizes Λ.
        for (nm, m) in [
            ("R60", (0.5, -s / 2.0, s / 2.0, 0.5)),
            ("R90", (0.0, -1.0, 1.0, 0.0)),
            ("R120", (-0.5, -s / 2.0, s / 2.0, -0.5)),
        ] {
            assert!(
                !(is_lattice(apply_iso(m, Vec2::ZERO, w1), w1, w2)
                    && is_lattice(apply_iso(m, Vec2::ZERO, w2), w1, w2)),
                "{nm} must not normalize the lattice (no {nm} symmetry)"
            );
        }
    }

    /// 2-isohedral: seeds 0/1 are the two full-group isohedral classes —
    /// the half-turn about any centre (hence all four c0 + Λ/2
    /// representatives) permutes the four cells mod lattice by the
    /// closed permutation 0 ↔ 1, 2 ↔ 3, the group they generate has
    /// exactly two orbits on the cells ({0, 1} / {2, 3}, matching the
    /// seeds), both seeds appear in every patch, and every generated
    /// tile keeps the prototile side multiset {1, 1, 1, 2, 2}.
    #[test]
    fn pentagon_6_isohedral_classes() {
        let t = spec();
        let s = s3();
        let mut counts = [0usize; 2];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(counts, [2, 2], "two tiles per isohedral class");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let mod1 = |x: f64| (x + 0.5).rem_euclid(1.0) - 0.5;
        let c0 = two_fold_centre();
        let cents: Vec<Vec2> = t.cells.iter().map(|c| cent(&c.poly.pts)).collect();
        // A half-turn about centre c/2 (linear part −I) permutes the
        // cells: cell k's centroid maps to cell j's centroid mod
        // lattice, for exactly one j.
        let perm_of = |tr: Vec2| -> Vec<usize> {
            let mut perm: Vec<usize> = Vec::new();
            for k in 0..4 {
                let img = tr - cents[k];
                let mut hits: Vec<usize> = Vec::new();
                for (j, &cj) in cents.iter().enumerate() {
                    let du = u_of(img - cj, w1, w2);
                    if mod1(du.x).abs() < 1e-6 && mod1(du.y).abs() < 1e-6 {
                        hits.push(j);
                    }
                }
                assert_eq!(hits.len(), 1, "cell {k} must have exactly one image, got {hits:?}");
                perm.push(hits[0]);
            }
            perm
        };
        // All four c0 + Λ/2 representatives give the same cell
        // permutation.
        let mut perms = Vec::new();
        for h in [Vec2::ZERO, w1 * 0.5, w2 * 0.5, (w1 + w2) * 0.5] {
            let c = c0 + h;
            perms.push(perm_of(c * 2.0));
        }
        for (i, pm) in perms.iter().enumerate() {
            assert_eq!(*pm, vec![1, 0, 3, 2], "half-turn perm (rep {i})");
        }
        // Two full-group orbits on the cells, exactly the seed classes.
        let mut seen = [false; 4];
        let mut orbits: Vec<Vec<usize>> = Vec::new();
        for st in 0..4 {
            if seen[st] {
                continue;
            }
            let mut orb = vec![st];
            seen[st] = true;
            let mut head = 0;
            while head < orb.len() {
                let x = orb[head];
                head += 1;
                for pm in &perms {
                    let y = pm[x];
                    if !seen[y] {
                        seen[y] = true;
                        orb.push(y);
                    }
                }
            }
            orb.sort_unstable();
            orbits.push(orb);
        }
        assert_eq!(
            orbits,
            vec![vec![0, 1], vec![2, 3]],
            "two full-group orbits"
        );
        for orb in &orbits {
            let sd = t.cells[orb[0]].seed;
            assert!(
                orb.iter().all(|&k| t.cells[k].seed == sd),
                "orbit mixes isohedral seeds"
            );
        }
        assert_ne!(
            t.cells[orbits[0][0]].seed,
            t.cells[orbits[1][0]].seed,
            "orbits differ"
        );
        // Generated patches: both classes present, side multiset kept.
        let params = ParamSet::default_for(t.params());
        let rp = t.generate(
            &params,
            &Query {
                region: BBox::new(v2(-20.25, -15.0 * s), v2(20.25, 15.0 * s)),
            },
        );
        assert!(rp.message.is_none(), "{:?}", rp.message);
        let mut seen_s = [false; 2];
        for tile in &rp.tiles {
            let sd = tile.seed as usize;
            assert!(sd < 2, "seeds must stay in {{0, 1}}");
            seen_s[sd] = true;
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| {
                    (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length()
                })
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [1.0, 1.0, 1.0, 2.0, 2.0];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!(
                    (got - w).abs() < 1e-12,
                    "seed {sd} side {got} ∉ {{1 × 3, 2 × 2}}"
                );
            }
        }
        assert!(seen_s[0] && seen_s[1], "both isohedral classes must appear");
    }

    /// The type-6 prototile is ACHIRAL: its mirror image is directly
    /// congruent to it (unlike the other shipped pentagon families, the
    /// chiral-prototile argument cannot exclude mirrors here — the group
    /// is p2 by the normalizer argument in `pentagon_6_p2_symmetry`).
    /// Exact Q(√3) identities: as sets, mirror_y(T0) = R300·T0 +
    /// (−1/2, −√3/2); and pointwise, with MIR in the mirror's own CCW
    /// order (MIR[i] = mirror_y of corner (A,B,C,D,E)[4−i]),
    /// Rot60·MIR[i] + (−1/2, √3/2) = PIN[i] for every i.
    #[test]
    fn pentagon_6_achiral_tile() {
        let s = s3();
        // Pinned prototile (A, B, C, D, E).
        let pin = [c(0.0, 0.0), c(2.0, 0.0), c(4.0, 2.0), c(0.0, 2.0), c(-1.0, 1.0)];
        // The mirror image (reflection in the x-axis) in its own CCW
        // order: E', D', C', B', A'.
        let mir = [
            v2(pin[4].x, -pin[4].y),
            v2(pin[3].x, -pin[3].y),
            v2(pin[2].x, -pin[2].y),
            v2(pin[1].x, -pin[1].y),
            v2(pin[0].x, -pin[0].y),
        ];
        // Set form: mirror_y(T0) = R300·T0 + (−1/2, −√3/2).
        let r300 = (0.5, s / 2.0, -s / 2.0, 0.5);
        let set_r: BTreeSet<(i64, i64)> = pin
            .iter()
            .map(|p| pkey(apply_iso(r300, v2(-0.5, -s / 2.0), *p)))
            .collect();
        let set_m: BTreeSet<(i64, i64)> = mir.iter().map(|p| pkey(*p)).collect();
        assert_eq!(
            set_m, set_r,
            "mirror_y(T0) must equal R300·T0 + (−1/2, −√3/2) as a set"
        );
        // Pointwise: Rot60·MIR[i] + (−1/2, √3/2) = PIN[i] (same index) —
        // the mirror image is a direct congruent copy of the tile.
        let r60 = (0.5, -s / 2.0, s / 2.0, 0.5);
        for i in 0..5 {
            let img = apply_iso(r60, v2(-0.5, s / 2.0), mir[i]);
            assert!(
                (img - pin[i]).length() < 1e-12,
                "achirality: Rot60·MIR[{i}] + (−1/2, √3/2) = {img:?} ≠ PIN[{i}] = {:#?}",
                pin[i]
            );
        }
        // Every shipped cell is a direct congruent copy of the prototile
        // (the achiral shape admits no mirror-only cells).
        let t = spec();
        let proto = &t.cells[0].poly;
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        for (k, cell) in t.cells.iter().enumerate() {
            let cs = (0..5).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..5).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd, "cell {k} must be a directly congruent copy of the prototile");
        }
    }

    /// Floret kinship: the pinned prototile is congruent to the
    /// floret-pentagonal tile (the floret_pentagonal shape in laves.rs,
    /// defined locally here to avoid a feature dependency) — pin =
    /// R30⁷·(√3·FLORET6) + (2, √3), pointwise on all five vertices, and
    /// (k, t) = (7, (2, √3)) is the unique solution over the 12
    /// rotations.  Same pentagon shape (angles (120, 120, 60, 120, 120),
    /// sides a = d = e = 1, b = c = 2 at the √3 scale); different
    /// tilings: the floret's is p6, 1-isohedral, this one is p2,
    /// 2-isohedral.
    #[test]
    fn pentagon_6_floret_kinship() {
        let s = s3();
        // laves.rs floret_pentagonal(): H = √3/2; the five prototile
        // corners of one floret tile, in the laves vertex order:
        let h = s / 2.0;
        let f0 = v2(1.0, -2.0 * h / 3.0);
        let f1 = v2(1.5, -h / 3.0);
        let f2 = v2(1.5, h / 3.0);
        let f3 = v2(1.0, 2.0 * h / 3.0);
        let f4 = v2(0.0, 0.0);
        // Canonical order (60° corner at index 2): [f2, f3, f4, f0, f1].
        let f6 = [f2, f3, f4, f0, f1];
        // The floret at the shipped scale (√3 × the laves scale):
        // angles (120, 120, 60, 120, 120), sides 1, 2, 2, 1, 1 — the
        // same type-6 shape as the pinned prototile.
        let fs = [f6[0] * s, f6[1] * s, f6[2] * s, f6[3] * s, f6[4] * s];
        let poly = Polygon::from_slice(&[fs[0], fs[1], fs[2], fs[3], fs[4]]);
        for (i, want) in [(0usize, 120.0f64), (1, 120.0), (2, 60.0), (3, 120.0), (4, 120.0)] {
            let got = angle_at(&poly, i).to_degrees();
            assert!((got - want).abs() < 1e-9, "floret angle at vertex {i}: {got} ≠ {want}");
        }
        for (i, want) in [(0usize, 1.0f64), (1, 2.0), (2, 2.0), (3, 1.0), (4, 1.0)] {
            assert!(
                (side_of(&poly, i) - want).abs() < 1e-12,
                "floret side {i} (length {want})"
            );
        }
        // The congruence: pin = R30^k·(√3·FLORET6) + t, pointwise for
        // all five vertices, over all 12 rotations; the unique solution
        // is k = 7, t = (2, √3).
        let t = spec();
        let pin = t.cells[0].poly.pts.clone();
        let cs = [
            (1.0, 0.0),
            (s / 2.0, 0.5),
            (0.5, s / 2.0),
            (0.0, 1.0),
            (-0.5, s / 2.0),
            (-s / 2.0, 0.5),
            (-1.0, 0.0),
            (-s / 2.0, -0.5),
            (-0.5, -s / 2.0),
            (0.0, -1.0),
            (0.5, -s / 2.0),
            (s / 2.0, -0.5),
        ];
        let mut solutions: Vec<(usize, Vec2)> = Vec::new();
        for (k, &(cx, sy)) in cs.iter().enumerate() {
            let rot = (cx, -sy, sy, cx);
            let tr = pin[0] - apply_iso(rot, Vec2::ZERO, fs[0]);
            if (0..5).all(|i| (apply_iso(rot, tr, fs[i]) - pin[i]).length() < 1e-12) {
                solutions.push((k, tr));
            }
        }
        assert_eq!(solutions.len(), 1, "unique (k, t) over the 12 rotations: {solutions:?}");
        let (k, tr) = solutions[0];
        assert_eq!(k, 7, "k = 7 (R210)");
        assert!(
            (tr - v2(2.0, s)).length() < 1e-12,
            "t = (2, √3), got {tr:?}"
        );
    }
}

// ---------------------------------------------------------------------
// M99: rosette transform (Kaplan dissertation §3.8) — the r′ formula,
// the two per-tile rules on the 4.8.8 substrate shapes, and the
// edge-to-edge / exactly-once invariants of the transformed table
// (the dual.rs house patterns, applied to the pre-inference template).
// ---------------------------------------------------------------------
#[cfg(all(test, feature = "hankin_ext"))]
mod rosette_transform_tests {
    use super::hankin_4_8_8_rosette_transform::{transformed_cells, LATTICE, TILE_0, TILE_1};
    use super::rosette_transform::{self, Rule};
    use crate::lattice::{similarity_map, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};
    use std::collections::HashMap;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::ParamSet;
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};
    use tess_core::view::ColorMode;
    use tess_core::TilingCategory;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Absolute shoelace area (house pattern, as in `dual::signed_area`).
    fn area(pts: &[Vec2]) -> f64 {
        let mut s = 0.0;
        for i in 0..pts.len() {
            let (a, b) = (pts[i], pts[(i + 1) % pts.len()]);
            s += a.x * b.y - b.x * a.y;
        }
        s.abs() / 2.0
    }

    /// Test-view lattice tiling of the transformed table: the edge
    /// census and exactly-once checks run through the Tiling trait.
    fn transformed_table() -> LatticeTiling {
        let (cells, _rep) = transformed_cells();
        LatticeTiling {
            id: "rosette-transformed-4-8-8",
            name: "rosette-transformed 4.8.8 (test view)",
            description: "test view of the M99 transformed substrate",
            lattice: LATTICE,
            cells,
            offset: Vec2::ZERO,
            group: WallpaperGroup::P4m,
            lattice_kind: LatticeKind::Square,
            category: TilingCategory::Islamic,
            secondary: &[],
            seed_pattern: SeedPattern::Uniform,
            params: &SIMILARITY_PARAMS,
            make_map: similarity_map,
            color_mode: ColorMode::Pattern,
        }
    }

    #[test]
    fn inner_radius_forms_agree_and_spoke_is_half_side() {
        // The two algebraic forms of the dissertation radius agree to
        // machine precision, and the spoke = half inner side identity
        // (the Rule-1 defining property) holds for the whole range.
        for n in 5usize..=12 {
            let a = rosette_transform::inner_radius(n, 1.0);
            let b = rosette_transform::inner_radius_closed(n, 1.0);
            assert!(
                (a - b).abs() <= 1e-12,
                "n={n}: dissertation form {a} vs closed form {b}"
            );
            let pi_n = std::f64::consts::PI / n as f64;
            let spoke = pi_n.cos() - a; // apothem (r=1) minus r′
            let half_side = a * pi_n.sin(); // s′/2 = r′·sin(π/n)
            assert!(
                (spoke - half_side).abs() <= 1e-12,
                "n={n}: spoke {spoke} vs half inner side {half_side}"
            );
        }
        // Anchor values (r = 1) at the ends of the range.
        assert!(
            (rosette_transform::inner_radius(5, 1.0) - 0.5095254494944288).abs() < 1e-12,
            "n=5 anchor"
        );
        assert!(
            (rosette_transform::inner_radius(12, 1.0) - 0.7673269879789604).abs() < 1e-12,
            "n=12 anchor"
        );
    }

    #[test]
    fn octagon_maps_to_inner_octagon_and_8_pentagons() {
        let (faces, rep) = rosette_transform::transform_tile(&TILE_0);
        assert_eq!(rep.rule, Rule::RegularPolygon);
        assert_eq!(rep.n, 8);
        assert_eq!(faces.len(), 9, "inner octagon + 8 corner pentagons");
        let n5 = faces.iter().filter(|f| f.len() == 5).count();
        let n8 = faces.iter().filter(|f| f.len() == 8).count();
        assert_eq!((n5, n8), (8, 1), "face census");
        let sum: f64 = faces.iter().map(|f| area(f)).sum();
        assert!(
            (sum - rep.tile_area).abs() < 1e-9,
            "area closure {sum} vs {}",
            rep.tile_area
        );
        // Pentagon side multiset: two spokes (s′/2), two half outer
        // edges (the substrate octagon has edge 1 → ½), one inner edge
        // (s′) — all derived from the dissertation formula.
        let mut acc = Vec2::ZERO;
        for p in TILE_0 {
            acc += p;
        }
        let c = acc / 8.0;
        let r = (TILE_0[0] - c).length();
        let pi8 = std::f64::consts::PI / 8.0;
        let rp = rosette_transform::inner_radius(8, r);
        let half_side = rp * pi8.sin(); // s′/2
        let s_prime = 2.0 * half_side;
        let mut want = [half_side, half_side, 0.5, 0.5, s_prime];
        want.sort_by(f64::total_cmp);
        for f in faces.iter().filter(|f| f.len() == 5) {
            let mut got: Vec<f64> = (0..5)
                .map(|k| (f[(k + 1) % 5] - f[k]).length())
                .collect();
            got.sort_by(f64::total_cmp);
            for (g, w) in got.iter().zip(want.iter()) {
                assert!((g - w).abs() < 1e-9, "pentagon side {g} vs {w}");
            }
        }
        // The inner face is the regular octagon of circumradius r′,
        // concentric with the substrate octagon.
        let inner = faces.iter().find(|f| f.len() == 8).unwrap();
        for &p in inner {
            assert!(
                ((p - c).length() - rp).abs() < 1e-9,
                "inner octagon vertex off radius: |{p} − c| vs r′"
            );
        }
    }

    #[test]
    fn square_maps_to_4_half_squares_centre_clustered() {
        // The n = 4 substrate tile falls to the bisector rule: the six
        // pairwise bisector intersections collapse to a single cluster
        // (all four meeting at the centre) and the map is four small
        // squares of side 1/2.
        let (faces, rep) = rosette_transform::transform_tile(&TILE_1);
        assert_eq!(rep.rule, Rule::Bisectors);
        assert_eq!(rep.n, 4);
        assert_eq!(faces.len(), 4, "four small squares");
        assert_eq!(
            rep.intersections,
            4,
            "4 of the 6 bisector pairs meet inside (opposite-side pairs are parallel)"
        );
        assert_eq!(rep.clusters, 1, "all 4 intersections cluster at the centre");
        assert!(rep.clustered, "Kaplan averaging heuristic must fire");
        let sum: f64 = faces.iter().map(|f| area(f)).sum();
        assert!(
            (sum - rep.tile_area).abs() < 1e-9,
            "area closure {sum} vs {}",
            rep.tile_area
        );
        // The only geometric points in the map: the centre, the 4
        // corners, and the 4 edge midpoints — every face vertex must
        // come from that set, and every point of the set must be used.
        let mut pts9: Vec<Vec2> = vec![Vec2::ZERO];
        for i in 0..4 {
            pts9.push(TILE_1[i]);
            pts9.push((TILE_1[i] + TILE_1[(i + 1) % 4]) / 2.0);
        }
        let mut used9 = [false; 9];
        for f in &faces {
            assert_eq!(f.len(), 4, "each face is a quadrilateral");
            for k in 0..4 {
                let e1 = f[(k + 1) % 4] - f[k];
                assert!((e1.length() - 0.5).abs() < 1e-9, "small square side");
                let e2 = f[(k + 2) % 4] - f[(k + 1) % 4];
                assert!(e1.dot(e2).abs() < 1e-9, "small square right angle");
            }
            for &p in f {
                let idx = pts9
                    .iter()
                    .position(|q| (*q - p).length() < 1e-9)
                    .unwrap_or_else(|| panic!("face vertex {p} not in the 9-point set"));
                used9[idx] = true;
            }
        }
        assert!(
            used9.iter().all(|&u| u),
            "every point of the 9-point set is a face vertex"
        );
    }

    #[test]
    fn transformed_488_area_census() {
        let (cells, rep) = transformed_cells();
        assert_eq!(cells.len(), 13, "1 oct + 8 pent + 4 sq per lattice cell");
        assert_eq!(rep.per_tile.len(), 2, "one report per substrate tile");
        for t in &rep.per_tile {
            assert!((t.area_sum - t.tile_area).abs() < 1e-9, "per-tile closure");
        }
        let a = 1.0 + core::f64::consts::SQRT_2;
        assert!((rep.lattice_area - a * a).abs() < 1e-12, "|det| = (1+√2)²");
        let sum: f64 = cells.iter().map(|c| area(&c.poly.pts)).sum();
        assert!(
            (sum - rep.lattice_area).abs() < 1e-9,
            "Σ face areas {sum} vs |det| {}",
            rep.lattice_area
        );
    }

    // ------------------------------------------------------------------
    // Edge census helpers (duplicates of the dual.rs test helpers —
    // those are private to the dual test module).
    // ------------------------------------------------------------------

    /// Inverse of the lattice matrix A = [a1 a2] (basis as columns).
    struct LatticeInv {
        m: [[f64; 2]; 2],
    }

    impl LatticeInv {
        fn new(a1: Vec2, a2: Vec2) -> Option<Self> {
            let det = a1.cross(a2);
            if det.abs() < 1e-12 {
                return None;
            }
            let d = 1.0 / det;
            Some(Self {
                m: [
                    [a2.y * d, -a2.x * d],
                    [-a1.y * d, a1.x * d],
                ],
            })
        }

        /// Lattice (u-space) coordinates of a cell-space point.
        fn apply(&self, p: Vec2) -> Vec2 {
            Vec2::new(
                self.m[0][0] * p.x + self.m[0][1] * p.y,
                self.m[1][0] * p.x + self.m[1][1] * p.y,
            )
        }
    }

    /// Undirected edge key on the 1e-6 grid (AGENTS.md edge-parity
    /// convention; shared edges between lattice translates agree to ~1
    /// ulp).
    fn edge_key(a: Vec2, b: Vec2) -> [i64; 4] {
        let mut k = [
            (a.x * 1e6).round() as i64,
            (a.y * 1e6).round() as i64,
            (b.x * 1e6).round() as i64,
            (b.y * 1e6).round() as i64,
        ];
        if (k[0], k[1]) > (k[2], k[3]) {
            k.swap(0, 2);
            k.swap(1, 3);
        }
        k
    }

    /// The partner table edge for every table edge of `t`: for each
    /// edge of cell `ci`, the unique `(cell, lattice offset)` whose
    /// drawn edge is a lattice translate of it. Panics if any edge has
    /// a different number of partners than one (a table that is not
    /// edge-to-edge at the period level). Result is indexed per cell:
    /// `partners[ci][e]`.
    fn edge_partners(t: &LatticeTiling) -> Vec<Vec<(usize, i64, i64)>> {
        let (a1, a2) = (t.lattice[0], t.lattice[1]);
        let inv = LatticeInv::new(a1, a2).expect("non-degenerate lattice");
        let n = t.cells.len();
        let mut partners = Vec::new();
        for ci in 0..n {
            let pts = &t.cells[ci].poly.pts;
            let m = pts.len();
            let mut row = Vec::new();
            for e in 0..m {
                let p = pts[e];
                let q = pts[(e + 1) % m];
                let mut found: Vec<(usize, i64, i64)> = Vec::new();
                for cj in 0..n {
                    let p2 = &t.cells[cj].poly.pts;
                    let m2 = p2.len();
                    for f in 0..m2 {
                        if cj == ci && f == e {
                            continue;
                        }
                        for (pa, pb) in [
                            (p2[f], p2[(f + 1) % m2]),
                            (p2[(f + 1) % m2], p2[f]),
                        ] {
                            for (ta, tb) in [(p, q), (q, p)] {
                                let s = pa - ta;
                                let u = inv.apply(s);
                                if (u.x - u.x.round()).abs() > 1e-6
                                    || (u.y - u.y.round()).abs() > 1e-6
                                {
                                    continue;
                                }
                                if (pb - tb - s).length() > 1e-9 {
                                    continue;
                                }
                                found.push((cj, u.x.round() as i64, u.y.round() as i64));
                                break;
                            }
                        }
                    }
                }
                // Dedup: the same partner edge is one solution, but a
                // non-edge-to-edge table could carry several.
                found.sort();
                found.dedup();
                assert_eq!(
                    found.len(),
                    1,
                    "table edge {} of cell {} has {} partners",
                    e,
                    ci,
                    found.len()
                );
                row.push(found[0]);
            }
            partners.push(row);
        }
        partners
    }

    #[test]
    fn transformed_488_is_edge_to_edge() {
        let t = transformed_table();
        // Period level: every table edge has exactly one partner table
        // edge (0 unpaired edges at the period level).
        let partners = edge_partners(&t);
        let (a1, a2) = (t.lattice[0], t.lattice[1]);
        // Tile the window and count each edge by rounded endpoints:
        // interior edges exactly twice (0 T-junctions → n ≤ 2), the
        // partner's window membership decides the expected count.
        let w = 4i64;
        let mut counts: HashMap<[i64; 4], (usize, bool)> = HashMap::new();
        for i in -w..=w {
            for j in -w..=w {
                let tr = a1 * i as f64 + a2 * j as f64;
                for (ci, cell) in t.cells.iter().enumerate() {
                    let pts = &cell.poly.pts;
                    for e in 0..pts.len() {
                        let pa = pts[e] + tr;
                        let pb = pts[(e + 1) % pts.len()] + tr;
                        let (_pc, poi, pojj) = partners[ci][e];
                        let pi = i - poi;
                        let pj = j - pojj;
                        let inwin = (-w..=w).contains(&pi) && (-w..=w).contains(&pj);
                        let entry = counts.entry(edge_key(pa, pb)).or_insert((0, false));
                        entry.0 += 1;
                        entry.1 |= inwin;
                    }
                }
            }
        }
        for (k, (n, inwin)) in &counts {
            assert!(
                *n <= 2,
                "edge {k:?} shared by {n} tiles (T-junction?)"
            );
            if *inwin {
                assert_eq!(*n, 2, "unpaired interior edge {k:?}");
            } else {
                assert_eq!(
                    *n, 1,
                    "edge {k:?} appears {n} times though its partner lies outside the window"
                );
            }
        }
    }

    #[test]
    fn transformed_488_fine_grid_exactly_once() {
        let t = transformed_table();
        let region = BBox::new(v2(-6.5, -6.5), v2(6.5, 6.5));
        let step = 0.25;
        let nsteps = (region.size().x / step) as usize;
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "{:?}", r.message);
        let tiles = &r.tiles;
        for gy in 0..nsteps {
            for gx in 0..nsteps {
                let pt = region.min
                    + v2(step * (gx as f64 + 0.5), step * (gy as f64 + 0.5));
                let cnt = tiles.iter().filter(|t| t.poly.contains(pt)).count();
                assert!(cnt >= 1, "gap at {pt}");
                if cnt == 2 {
                    // Must be a point on the shared edge of the two tiles.
                    let idx: Vec<usize> = tiles
                        .iter()
                        .enumerate()
                        .filter(|(_, t)| t.poly.contains(pt))
                        .map(|(i, _)| i)
                        .collect();
                    let a = &tiles[idx[0]].poly;
                    let b = &tiles[idx[1]].poly;
                    let mut on_shared = false;
                    for e0 in 0..a.pts.len() {
                        let p0 = a.pts[e0];
                        let p1 = a.pts[(e0 + 1) % a.pts.len()];
                        let k = edge_key(p0, p1);
                        for e1 in 0..b.pts.len() {
                            if edge_key(b.pts[e1], b.pts[(e1 + 1) % b.pts.len()]) == k {
                                let ab = p1 - p0;
                                let tt =
                                    ((pt - p0).dot(ab) / ab.dot(ab)).clamp(0.0, 1.0);
                                on_shared = (p0 + ab * tt - pt).length() <= 1e-6;
                                break;
                            }
                        }
                        if on_shared {
                            break;
                        }
                    }
                    assert!(
                        on_shared,
                        "double-covered point {pt} not on a shared edge"
                    );
                } else if cnt > 2 {
                    let near = tiles
                        .iter()
                        .filter(|t| t.poly.pts.iter().any(|&v| (v - pt).length() <= 1e-6))
                        .count();
                    assert_eq!(
                        near, cnt,
                        "vertex fan count {near} != coverage {cnt} at {pt}"
                    );
                }
            }
        }
        // Clipped area closes against the region area.
        let mut area = 0.0;
        for t in tiles {
            if let Some(cl) = t.poly.clipped_to(&region) {
                area += cl.area();
            }
        }
        let ra = region.area();
        assert!(
            (area - ra).abs() <= 1e-6 * ra,
            "clipped area {area} vs region {ra} (rel {})",
            (area - ra) / ra
        );
    }
}

// ---------------------------------------------------------------------
// M99: hankin-4-8-8-rosette-transform — registry checks, inference
// census, and the p4m cell-table probe.
// ---------------------------------------------------------------------
#[cfg(all(test, feature = "hankin_ext"))]
mod hankin_4_8_8_rosette_transform_tests {
    use super::hankin;
    use super::hankin_4_8_8_rosette_transform::{
        hankin_4_8_8_rosette_transform, transformed_cells, DELTA, EXPECTED_LENS,
        EXPECTED_SEEDS, LATTICE, THETA_E_DEG,
    };
    use super::tests::{check_covers, check_det, check_rotated};
    use std::collections::HashMap;
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_4_8_8_rosette_transform_covers_region() {
        check_covers(
            &hankin_4_8_8_rosette_transform(),
            "hankin-4-8-8-rosette-transform",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_4_8_8_rosette_transform_rotated_scaled() {
        check_rotated(
            &hankin_4_8_8_rosette_transform(),
            "hankin-4-8-8-rosette-transform(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_4_8_8_rosette_transform_deterministic() {
        check_det(
            &hankin_4_8_8_rosette_transform(),
            "hankin-4-8-8-rosette-transform",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_4_8_8_rosette_transform_unit_structure() {
        let t = hankin_4_8_8_rosette_transform();
        assert_eq!(
            t.cells.len(),
            93,
            "16×3 + 36×4 + 32×5 + 8×6 + 1×16 faces per lattice cell"
        );
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, EXPECTED_SEEDS.to_vec(), "seed layout");
        let lens: Vec<usize> = t.cells.iter().map(|c| c.poly.pts.len()).collect();
        assert_eq!(lens, EXPECTED_LENS.to_vec(), "face census layout");
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        let a = 1.0 + core::f64::consts::SQRT_2;
        assert!((det - a * a).abs() < 1e-9, "det = (1+√2)²");
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_4_8_8_rosette_transform_symmetry_metadata() {
        let t = hankin_4_8_8_rosette_transform();
        assert_eq!(t.id, "hankin-4-8-8-rosette-transform");
        assert_eq!(t.group, WallpaperGroup::P4m);
        assert_eq!(t.lattice_kind, LatticeKind::Square);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }

    #[test]
    fn hankin_4_8_8_rosette_transform_inference_census() {
        let (sub, _trep) = transformed_cells();
        let tiles: Vec<&[Vec2]> = sub.iter().map(|c| c.poly.pts.as_slice()).collect();
        let (cells, report) = hankin::build_cells(&tiles, THETA_E_DEG, DELTA);
        assert!(report.complete, "pairing must be complete at the selected θK");
        let lens: Vec<usize> = report.per_tile.iter().map(|r| r.faces).collect();
        // 8 pentagons → 8 faces each, inner octagon → 9, 4 small
        // squares → 5 each (machine-transcribed order).
        assert_eq!(
            lens,
            vec![8, 8, 8, 8, 8, 8, 8, 8, 9, 5, 5, 5, 5],
            "face census per transformed tile"
        );
        for r in &report.per_tile {
            assert!((r.area_sum - r.tile_area).abs() < 1e-9, "per-tile closure");
            assert_eq!(r.rays, r.pairs * 2, "every ray paired exactly once");
        }
        assert_eq!(cells.len(), 93, "inference table size");
        let a = 1.0 + core::f64::consts::SQRT_2;
        let sum: f64 = cells.iter().map(|c| c.poly.area().abs()).sum();
        assert!((sum - a * a).abs() < 1e-9, "Σ inference areas {sum} vs (1+√2)²");
    }

    /// Canonical face key: CCW (flipped images reversed), rotated to
    /// the lexicographically smallest vertex, rounded to a 1e-10 grid.
    fn canon_pts(pts: &[Vec2], flip: bool) -> Vec<Vec2> {
        let mut v: Vec<Vec2> = if flip {
            pts.iter().rev().copied().collect()
        } else {
            pts.to_vec()
        };
        let mut k = 0;
        for i in 1..v.len() {
            let dx = v[i].x - v[k].x;
            let dy = v[i].y - v[k].y;
            if dx < -1e-12 || (dx.abs() <= 1e-12 && dy < -1e-12) {
                k = i;
            }
        }
        v.rotate_left(k);
        v
    }

    fn key_of(v: &[Vec2]) -> (usize, Vec<i64>) {
        let mut k = Vec::with_capacity(v.len() * 2);
        for p in v {
            k.push((p.x * 1e10).round() as i64);
            k.push((p.y * 1e10).round() as i64);
        }
        (v.len(), k)
    }

    /// Multiset invariance of the cell table under a point symmetry
    /// `g` of the infinite tiling: the image of every table cell (up to
    /// a lattice translation and, for mirrors, orientation reversal)
    /// must coincide with a unique, still-unmatched table cell to
    /// 1e-9. Returns the worst vertex mismatch.
    fn assert_p4m_exact(label: &str, flip: bool, g: impl Fn(Vec2) -> Vec2) -> f64 {
        let t = hankin_4_8_8_rosette_transform();
        let (a1, a2) = (LATTICE[0], LATTICE[1]);
        let mut unused: HashMap<(usize, Vec<i64>), Vec<usize>> = HashMap::new();
        for (j, c) in t.cells.iter().enumerate() {
            let cp = canon_pts(&c.poly.pts, false);
            unused.entry(key_of(&cp)).or_default().push(j);
        }
        let mut worst = 0.0f64;
        for (i, c) in t.cells.iter().enumerate() {
            let img: Vec<Vec2> = c.poly.pts.iter().map(|p| g(*p)).collect();
            let mut best: Option<(f64, usize)> = None;
            for ta in -1i64..=1 {
                for tb in -1i64..=1 {
                    let tr = a1 * ta as f64 + a2 * tb as f64;
                    let shifted: Vec<Vec2> = img.iter().map(|p| *p + tr).collect();
                    let cp = canon_pts(&shifted, flip);
                    let key = key_of(&cp);
                    if let Some(v) = unused.get(&key) {
                        if let Some(&j) = v.first() {
                            let cj = canon_pts(&t.cells[j].poly.pts, false);
                            let d = (0..cp.len())
                                .map(|k| (cp[k] - cj[k]).length())
                                .fold(0.0f64, f64::max);
                            if d <= 1e-9 && best.is_none_or(|(bd, _)| d < bd) {
                                best = Some((d, j));
                            }
                        }
                    }
                }
            }
            let (d, j) = best.unwrap_or_else(|| {
                panic!("{label}: image of table cell {i} matches no table cell")
            });
            if let Some(v) = unused.get_mut(&key_of(&canon_pts(&t.cells[j].poly.pts, false)))
            {
                v.retain(|&x| x != j);
            }
            worst = worst.max(d);
        }
        let residual: usize = unused.values().map(Vec::len).sum();
        assert_eq!(
            residual,
            0,
            "{label}: {residual} table cells unmatched by the point symmetry"
        );
        assert!(worst <= 1e-9, "{label}: worst mismatch {worst}");
        worst
    }

    #[test]
    fn hankin_4_8_8_rosette_transform_cell_table_is_p4m_exact() {
        let a = LATTICE[0].x; // 1 + √2, the period side
        let c = a / 2.0;
        let mut worst = 0.0f64;
        worst = worst.max(assert_p4m_exact(
            "R90 about cell centre",
            false,
            |p| Vec2::new(c - (p.y - c), c + (p.x - c)),
        ));
        worst = worst.max(assert_p4m_exact(
            "R180 about cell centre",
            false,
            |p| Vec2::new(2.0 * c - p.x, 2.0 * c - p.y),
        ));
        worst = worst.max(assert_p4m_exact(
            "R270 about cell centre",
            false,
            |p| Vec2::new(c + (p.y - c), c - (p.x - c)),
        ));
        worst = worst.max(assert_p4m_exact(
            "x-mirror (line x = A/2)",
            true,
            |p| Vec2::new(a - p.x, p.y),
        ));
        worst = worst.max(assert_p4m_exact(
            "y-mirror (line y = A/2)",
            true,
            |p| Vec2::new(p.x, a - p.y),
        ));
        worst = worst.max(assert_p4m_exact(
            "diagonal mirror y = x",
            true,
            |p| Vec2::new(p.y, p.x),
        ));
        worst = worst.max(assert_p4m_exact(
            "anti-diagonal mirror y = A − x",
            true,
            |p| Vec2::new(a - p.y, a - p.x),
        ));
        assert!(
            worst <= 1e-9,
            "p4m point group not exact on the cell table (worst {worst})"
        );
    }
}

#[cfg(all(test, feature = "najm"))]
mod najm_tests {
    use super::tests::{check_covers, check_det, check_rotated};
    use crate::najm::{self, Axis, Diad, Feature, Inflation, NajmSpec};
    use tess_core::geometry::{BBox, Vec2};

    const PI: f64 = std::f64::consts::PI;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    fn spec_63() -> NajmSpec {
        NajmSpec {
            p: 6,
            q: 3,
            mult: [2, 0, 3],
            orient: [Feature::Edge, Feature::Vertex, Feature::Edge],
            inflation: Inflation::Pair(Diad::Ac),
        }
    }

    fn spec_44() -> NajmSpec {
        NajmSpec {
            p: 4,
            q: 4,
            mult: [3, 0, 0],
            orient: [Feature::Edge, Feature::Vertex, Feature::Vertex],
            inflation: Inflation::Order(&[najm::Step::Inflate(Axis::A)]),
        }
    }

    /// Signed interior angle (degrees, may be reflex) at vertex `i` of a
    /// CCW polygon.
    fn interior_angle_deg(pts: &[Vec2], i: usize) -> f64 {
        let n = pts.len();
        let (p0, p1, p2) = (pts[(i + n - 1) % n], pts[i], pts[(i + 1) % n]);
        let (u, w) = (p0 - p1, p2 - p1);
        let unsigned = u.cross(w).abs().atan2(u.dot(w)) * 180.0 / PI;
        if u.cross(w) < 0.0 {
            unsigned
        } else {
            360.0 - unsigned
        }
    }

    fn centroid(pts: &[Vec2]) -> Vec2 {
        let mut c = Vec2::ZERO;
        for &p in pts {
            c += p;
        }
        c * (1.0 / pts.len() as f64)
    }

    fn angles(pts: &[Vec2]) -> Vec<f64> {
        let mut a: Vec<f64> = (0..pts.len())
            .map(|i| interior_angle_deg(pts, i))
            .collect();
        a.sort_by(|x, y| x.partial_cmp(y).unwrap());
        a
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    #[test]
    fn radii_match_the_spike() {
        // 60-digit spike values (t22b_spike.py), case 3 on AC.
        let rr = spec_63().radii();
        assert_eq!(rr[1], 0.0);
        assert!((rr[0] - 0.5962939229190058).abs() < 1e-14, "r_A = {}", rr[0]);
        assert!((rr[2] - 0.45123723482970224).abs() < 1e-14, "r_C = {}", rr[2]);
        // Exact identities of the case-3 solution.
        let x = 2.0 * rr[0] * (PI / 12.0).sin();
        assert!((x - 2.0 * rr[2] * (PI / 9.0).sin()).abs() < 1e-14, "equal chords");
        assert!(
            (rr[0] * (PI / 12.0).cos() + rr[2] * (PI / 9.0).cos() - 1.0).abs() < 1e-14,
            "extents sum to |AC| = 1"
        );
        let closed = 0.5 * x * ((PI / 12.0).cos() / (PI / 12.0).sin()
            + (PI / 9.0).cos() / (PI / 9.0).sin());
        assert!((closed - 1.0).abs() < 1e-14, "x = 2/(cot 15° + cot 20°)");

        // Case 2 at unit |AC|: r_A = |AB|/cos 15° = cos 45°/cos 15°;
        // rescaled to |AB| = 1 this is 1/cos 15° = √6 − √2 (exact).
        let r44 = spec_44().radii()[0];
        assert!((r44 - (PI / 4.0).cos() / (PI / 12.0).cos()).abs() < 1e-14);
        let ra = r44 / (PI / 4.0).cos();
        assert!((ra - (6.0f64.sqrt() - 2.0f64.sqrt())).abs() < 1e-14);
        assert!((ra * (PI / 12.0).cos() - 1.0).abs() < 1e-14, "apothem = 1");
    }

    #[test]
    fn najm_63_covers_region() {
        let t = najm::najm_63_2e_0_3e_ac();
        check_covers(&t, "najm-63-2e-0-3e-ac", BBox::new(v2(-2.5, -2.0), v2(7.5, 5.5)));
    }

    #[test]
    fn najm_63_rotated_scaled() {
        let t = najm::najm_63_2e_0_3e_ac();
        check_rotated(
            &t,
            "najm-63-2e-0-3e-ac",
            BBox::new(v2(-3.0, -3.0), v2(6.0, 5.0)),
            1.5,
            37.0,
        );
    }

    #[test]
    fn najm_63_deterministic() {
        let t = najm::najm_63_2e_0_3e_ac();
        check_det(&t, "najm-63-2e-0-3e-ac", BBox::new(v2(-2.5, -2.0), v2(7.5, 5.5)));
    }

    #[test]
    fn najm_44_covers_region() {
        let t = najm::najm_44_3e_0_0_a();
        check_covers(&t, "najm-44-3e-0-0-a", BBox::new(v2(-2.0, -2.0), v2(8.0, 8.0)));
    }

    #[test]
    fn najm_44_rotated_scaled() {
        let t = najm::najm_44_3e_0_0_a();
        check_rotated(
            &t,
            "najm-44-3e-0-0-a",
            BBox::new(v2(-3.0, -3.0), v2(7.0, 7.0)),
            2.5,
            64.0,
        );
    }

    #[test]
    fn najm_44_deterministic() {
        let t = najm::najm_44_3e_0_0_a();
        check_det(&t, "najm-44-3e-0-0-a", BBox::new(v2(-2.0, -2.0), v2(8.0, 8.0)));
    }

    #[test]
    fn najm_63_tile_shapes_and_orientations() {
        let t = najm::najm_63_2e_0_3e_ac();
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 1, 1, 2, 2, 2]);
        let lens: Vec<usize> = t.cells.iter().map(|c| c.poly.pts.len()).collect();
        assert_eq!(lens, vec![12, 9, 9, 6, 6, 6]);

        let dod = &t.cells[0].poly.pts;
        let c = centroid(dod);
        let r = (dod[0] - c).length();
        let side = (dod[1] - dod[0]).length();
        for i in 0..12 {
            assert!(((dod[i] - c).length() - r).abs() < 1e-12);
            assert!(((dod[(i + 1) % 12] - dod[i]).length() - side).abs() < 1e-12);
            assert!(close(interior_angle_deg(dod, i), 150.0));
        }

        // Both enneagons regular (radius/side/140° angles), equal areas.
        for k in 1..=2 {
            let e = &t.cells[k].poly.pts;
            let c = centroid(e);
            let r = (e[0] - c).length();
            let side = (e[1] - e[0]).length();
            for i in 0..9 {
                assert!(((e[i] - c).length() - r).abs() < 1e-12);
                assert!(((e[(i + 1) % 9] - e[i]).length() - side).abs() < 1e-12);
                assert!(close(interior_angle_deg(e, i), 140.0));
            }
        }
        assert!((t.cells[1].poly.area() - t.cells[2].poly.area()).abs() < 1e-14);

        // Orientation witnesses: a vertex ray of enne0 points at the
        // 2-fold point (√3/2, 0) (m_C = 3 flips the CB edge to a vertex);
        // an edge-midpoint ray of enne1 points at its partner a1.
        let s3 = 3.0f64.sqrt();
        let e0 = &t.cells[1].poly.pts;
        let c0 = centroid(e0);
        assert!(
            e0.iter()
                .any(|v| ((*v - c0).y.atan2((*v - c0).x) + PI / 2.0).abs() < 1e-9),
            "no enne0 vertex on the ray toward the 2-fold point"
        );
        let e1 = &t.cells[2].poly.pts;
        let c1 = centroid(e1);
        assert!(
            (0..9).any(|i| {
                let m = (e1[i] + e1[(i + 1) % 9]) * 0.5 - c1;
                (m.y.atan2(m.x) + PI / 2.0).abs() < 1e-9
            }),
            "no enne1 edge-midpoint ray toward a1"
        );

        // 2-fold witness: rot180 about (√3/2, 0) maps enne0's vertices
        // exactly onto enne1's translated by −a2 (phases 190° + 180° =
        // 250° mod 40° — the classes' orientations differ).
        let b0 = v2(s3 / 2.0, 0.0);
        let a2 = v2(s3 / 2.0, 1.5);
        for v in e0 {
            let w = b0 * 2.0 - *v + a2;
            assert!(
                e1.iter().any(|u| (*u - w).length() < 1e-12),
                "rot180(B0) does not map enne0 onto enne1 − a2"
            );
        }

        // Bowties: equilateral with the common edge x, angles
        // {70, 70, 70, 70, 220, 220}, equal areas.
        let x = 2.0 * r * (PI / 12.0).sin();
        for k in 3..6 {
            let b = &t.cells[k].poly.pts;
            for i in 0..6 {
                assert!(((b[(i + 1) % 6] - b[i]).length() - x).abs() < 1e-12);
            }
            let angs = angles(b);
            for (&want, &got) in [70.0, 70.0, 70.0, 70.0, 220.0, 220.0].iter().zip(&angs) {
                assert!(close(want, got), "bowtie angles {angs:?}");
            }
            assert!((t.cells[k].poly.area() - t.cells[3].poly.area()).abs() < 1e-14);
        }
    }

    #[test]
    fn najm_44_tile_shapes() {
        let t = najm::najm_44_3e_0_0_a();
        assert_eq!(t.cells.len(), 2);
        let s = 4.0 - 2.0 * 3.0f64.sqrt();

        let dod = &t.cells[0].poly.pts;
        assert_eq!(dod.len(), 12);
        for i in 0..12 {
            assert!(((dod[(i + 1) % 12] - dod[i]).length() - s).abs() < 1e-13);
            assert!(close(interior_angle_deg(dod, i), 150.0));
        }
        // The ±15° vertices sit exactly on x = 1 (the chord through B).
        assert_eq!(dod.iter().filter(|p| (p.x - 1.0).abs() < 1e-12).count(), 2);

        let star = &t.cells[1].poly.pts;
        assert_eq!(star.len(), 8);
        let c = v2(1.0, 1.0);
        let td = 3.0f64.sqrt() - 1.0;
        let nqr = (2.0 - 3.0f64.sqrt()) * 2.0f64.sqrt();
        let mut tips = 0;
        let mut notches = 0;
        for i in 0..8 {
            assert!(((star[(i + 1) % 8] - star[i]).length() - s).abs() < 1e-13);
            let rad = (star[i] - c).length();
            let ang = interior_angle_deg(star, i);
            if (rad - td).abs() < 1e-12 {
                assert!(close(ang, 60.0), "star tip angle {ang}");
                tips += 1;
            } else if (rad - nqr).abs() < 1e-12 {
                assert!(close(ang, 210.0), "star notch angle {ang}");
                notches += 1;
            } else {
                panic!("star vertex at unexpected radius {rad}");
            }
        }
        assert_eq!((tips, notches), (4, 4));
    }

    #[test]
    fn area_census_matches_the_cell() {
        let t1 = najm::najm_63_2e_0_3e_ac();
        let det1 = t1.lattice[0].cross(t1.lattice[1]).abs();
        assert!((det1 - 3.0 * 3.0f64.sqrt() / 2.0).abs() < 1e-14);
        let rr = spec_63().radii();
        let a12 = t1.cells[0].poly.area();
        let a9 = t1.cells[1].poly.area();
        let abt = t1.cells[3].poly.area();
        assert!((a12 - 3.0 * rr[0] * rr[0]).abs() < 1e-13, "12-gon = 3 r_A²");
        assert!(
            (a9 - 4.5 * rr[2] * rr[2] * (2.0 * PI / 9.0).sin()).abs() < 1e-13,
            "9-gon = (9/2) r_C² sin 40°"
        );
        assert!(
            (a12 + 2.0 * a9 + 3.0 * abt - det1).abs() < 1e-13,
            "1·A12 + 2·A9 + 3·Abt = |det|"
        );

        let t2 = najm::najm_44_3e_0_0_a();
        let det2 = t2.lattice[0].cross(t2.lattice[1]).abs();
        assert!((det2 - 4.0).abs() < 1e-14);
        let s3 = 3.0f64.sqrt();
        let a12 = t2.cells[0].poly.area();
        let ast = t2.cells[1].poly.area();
        assert!((a12 - (24.0 - 12.0 * s3)).abs() < 1e-12, "24 − 12√3");
        assert!((ast - (12.0 * s3 - 20.0)).abs() < 1e-12, "12√3 − 20");
        assert!((a12 + ast - det2).abs() < 1e-12, "sum = |det| = 4");
    }

    #[test]
    fn translation_lattices_are_primitive() {
        // Each entry holds exactly one 12-gon class per cell (centered at
        // the origin), and no other class has 12 vertices. Any translation
        // symmetry must map the dodecagon orbit to itself, so the
        // translation group is contained in the orbit's center set;
        // generating a patch and checking that the emitted dodecagon
        // centers are *precisely* the declared lattice points witnesses
        // the declared basis is primitive (no denser lattice underlies
        // the tiling).
        use tess_core::params::ParamSet;
        use tess_core::tiling::{Query, Tiling};
        for t in [najm::najm_63_2e_0_3e_ac(), najm::najm_44_3e_0_0_a()] {
            let lens: Vec<usize> = t.cells.iter().map(|c| c.poly.pts.len()).collect();
            assert_eq!(
                lens.iter().filter(|&&n| n == 12).count(),
                1,
                "{}: exactly one 12-gon class witnesses primitivity",
                t.id
            );
            let (a, b) = (t.lattice[0], t.lattice[1]);
            let det = a.cross(b);
            let coords =
                |p: Vec2| ((p.x * b.y - p.y * b.x) / det, (a.x * p.y - a.y * p.x) / det);
            let region = BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0));
            // Mirrors generate()'s inclusion rule: the cell's AABB,
            // translated to each lattice point, must intersect the region.
            let cell = t.cells.iter().find(|c| c.poly.pts.len() == 12).unwrap();
            let cc = centroid(&cell.poly.pts);
            let la = cell.poly.aabb();
            let hits = |c: Vec2| {
                c.x + la.max.x - cc.x > region.min.x
                    && c.x + la.min.x - cc.x < region.max.x
                    && c.y + la.max.y - cc.y > region.min.y
                    && c.y + la.min.y - cc.y < region.max.y
            };
            let pset = ParamSet::default_for(t.params());
            let r = t.generate(&pset, &Query { region });
            let mut counts: std::collections::BTreeMap<(i32, i32), u32> =
                std::collections::BTreeMap::new();
            let mut total = 0;
            for tile in &r.tiles {
                if tile.poly.pts.len() != 12 {
                    continue;
                }
                total += 1;
                let c = centroid(&tile.poly.pts);
                let (u, v) = coords(c);
                assert!(
                    (u - u.round()).abs() < 1e-7 && (v - v.round()).abs() < 1e-7,
                    "{}: dodecagon center {c:?} off the declared lattice ({u}, {v})",
                    t.id
                );
                *counts.entry((u.round() as i32, v.round() as i32)).or_insert(0) += 1;
            }
            assert!(total >= 9, "{}: patch too small to witness", t.id);
            // Every lattice point whose dodecagon meets the region appears
            // exactly once, and there are no others.
            let mut expected = 0;
            for iu in -8..=8i32 {
                for iv in -8..=8i32 {
                    if hits(a * iu as f64 + b * iv as f64) {
                        expected += 1;
                        assert_eq!(
                            counts.get(&(iu, iv)).copied(),
                            Some(1),
                            "{}: lattice point ({iu}, {iv}) has no unique dodecagon",
                            t.id
                        );
                    }
                }
            }
            assert_eq!(
                counts.values().sum::<u32>() as usize,
                expected,
                "{}: no dodecagon outside the lattice-point window",
                t.id
            );
        }
    }}
