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

#[cfg(feature = "ammann_beenker")]
pub mod ammann_beenker;
#[cfg(feature = "archimedean")]
pub mod archimedean;
#[cfg(feature = "cairo")]
pub mod cairo;
#[cfg(feature = "chair")]
pub mod chair;
#[cfg(feature = "girih")]
pub mod girih;
#[cfg(feature = "girih_ext")]
pub mod girih_w85;
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
#[cfg(feature = "pentagon13")]
pub mod pentagon13;

/// Registry of all enabled built-in tilings.
pub fn registry() -> tess_core::Registry {
    // `mut` is only used when at least one tiling feature is enabled.
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
    #[cfg(feature = "stampfli")]
    {
        r = r.register(stampfli::StampfliTiling);
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
    #[cfg(feature = "pentagon13")]
    {
        r = r.register(pentagon13::spec());
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
        r = r.register(cairo::cairo_collinear());
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
    #[cfg(feature = "khatam_girih")]
    {
        r = r.register(khatam_girih::khatam_girih());
    }
    #[cfg(feature = "kuniform")]
    {
        r = r.register(kuniform::kuniform_p6m());
        r = r.register(kuniform::kuniform_pmm());
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
    }
    r
}

#[cfg(test)]
mod tests {
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
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
    /// The declared-category grouping both UIs render: categories in
    /// canonical [`TilingCategory::ALL`] order, registration order
    /// preserved within each category, and no tiling dropped or
    /// duplicated. The per-category census pins today's taxonomy —
    /// extend it when adding a tiling (and the README's count).
    #[test]
    fn registry_groups_are_complete_and_canonically_ordered() {
        let r = super::registry();
        let grouped = r.grouped();

        let pos = |c: &tess_core::TilingCategory| {
            tess_core::TilingCategory::ALL
                .iter()
                .position(|a| a == c)
                .expect("declared categories are ALL variants")
        };
        for w in grouped.windows(2) {
            assert!(pos(&w[0].0) < pos(&w[1].0), "categories out of canonical order");
        }

        // Completeness: exactly a partition of the registry.
        let total: usize = grouped.iter().map(|(_, ts)| ts.len()).sum();
        assert_eq!(total, r.len(), "grouping must not drop tilings");
        let mut ids: Vec<&str> =
            grouped.iter().flat_map(|(_, ts)| ts.iter().map(|t| t.id())).collect();
        ids.sort_unstable();
        let mut expect: Vec<&str> = r.iter().map(|t| t.id()).collect();
        expect.sort_unstable();
        assert_eq!(ids, expect, "grouping must not duplicate tilings");

        // Registration order preserved within each category.
        for (c, ts) in &grouped {
            let registry_order: Vec<&str> =
                r.iter().filter(|t| t.category() == *c).map(|t| t.id()).collect();
            let got: Vec<&str> = ts.iter().map(|t| t.id()).collect();
            assert_eq!(got, registry_order, "{}: within-category order", c);
        }

        // Census of today's taxonomy.
        let counts: std::collections::BTreeMap<&str, usize> =
            grouped.iter().map(|(c, ts)| (c.label(), ts.len())).collect();
        let expect_counts: std::collections::BTreeMap<&str, usize> = [
            ("Uniform", 11),
            ("Laves (Catalan duals)", 9),
("Wallpaper", 14),
            ("Quasiperiodic", 7),
            ("Aperiodic", 7),
            ("Nonperiodic", 1),
        ]
        .into_iter()
        .collect();
        assert_eq!(counts, expect_counts, "category census (update when adding tilings)");
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
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::tiling::Tiling;

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

    #[test]
    fn cairo_collinear_covers_region() {
        check_covers(
            &cairo::cairo_collinear(),
            "cairo-collinear",
            BBox::new(v2(-16.0, -12.0), v2(16.0, 12.0)),
        );
    }

    #[test]
    fn cairo_collinear_rotated_scaled() {
        check_rotated(
            &cairo::cairo_collinear(),
            "cairo-collinear(rot30,scale0.7)",
            BBox::new(v2(-12.0, -12.0), v2(12.0, 12.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn cairo_collinear_deterministic() {
        check_det(
            &cairo::cairo_collinear(),
            "cairo-collinear",
            BBox::new(v2(-12.0, -10.0), v2(12.0, 10.0)),
        );
    }

    /// Defining properties of the collinear form: every vertex is integral,
    /// every edge has length 4 or √10, interior vertex figures are exactly
    /// the two types (degree 3 and degree 4), and the collinearity property
    /// — every interior edge fragment's supporting line carries a
    /// non-adjacent fragment.
    #[test]
    fn cairo_collinear_integer_properties() {
        use std::collections::{BTreeSet, HashMap};
        fn gcd(mut a: i64, mut b: i64) -> i64 {
            while b != 0 {
                let t = a % b;
                a = b;
                b = t;
            }
            a
        }
        type LineKey = (i64, i64, i64);
        type Frag = ((i64, i64), (i64, i64));
        let t = cairo::cairo_collinear();
        let p = tess_core::params::ParamSet::default_for(t.params());
        let region = BBox::new(v2(-40.0, -40.0), v2(40.0, 40.0));
        let r = t.generate(&p, &tess_core::tiling::Query { region });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert!(!r.tiles.is_empty());
        assert_eq!(
            r.tiles
                .iter()
                .map(|t| t.proto)
                .collect::<BTreeSet<_>>()
                .len(),
            4,
            "expect exactly 4 proto classes"
        );

        let sq10 = 10f64.sqrt();
        let mut edge_set: BTreeSet<Frag> = BTreeSet::new();
        for tile in &r.tiles {
            assert_eq!(tile.poly.pts.len(), 5);
            for v in &tile.poly.pts {
                assert!(
                    (v.x - v.x.round()).abs() < 1e-9 && (v.y - v.y.round()).abs() < 1e-9,
                    "non-integral vertex {v:?}"
                );
            }
            for i in 0..5 {
                let a = tile.poly.pts[i];
                let b = tile.poly.pts[(i + 1) % 5];
                let len = (b - a).length();
                assert!(
                    (len - 4.0).abs() < 1e-9 || (len - sq10).abs() < 1e-9,
                    "edge length {len} not 4 or sqrt(10)"
                );
                let ka = (a.x.round() as i64, a.y.round() as i64);
                let kb = (b.x.round() as i64, b.y.round() as i64);
                let (lo, hi) = if ka <= kb { (ka, kb) } else { (kb, ka) };
                edge_set.insert((lo, hi));
            }
        }

        // Supporting-line key: reduced integral direction + offset.
        let line_key = |lo: (i64, i64), hi: (i64, i64)| -> LineKey {
            let (mut dx, mut dy) = (hi.0 - lo.0, hi.1 - lo.1);
            let g = gcd(dx.abs(), dy.abs());
            dx /= g;
            dy /= g;
            if dx < 0 || (dx == 0 && dy < 0) {
                dx = -dx;
                dy = -dy;
            }
            (dx, dy, dy * lo.0 - dx * lo.1)
        };

        let mut lines: HashMap<LineKey, Vec<Frag>> = HashMap::new();
        let mut deg: HashMap<(i64, i64), u32> = HashMap::new();
        for &(lo, hi) in &edge_set {
            *deg.entry(lo).or_insert(0) += 1;
            *deg.entry(hi).or_insert(0) += 1;
            lines.entry(line_key(lo, hi)).or_default().push((lo, hi));
        }

        // Interior vertices show only the two vertex-figure degrees.
        for (&(vx, vy), &d) in &deg {
            if vx.abs() <= 20 && vy.abs() <= 20 {
                assert!(d == 3 || d == 4, "vertex ({vx},{vy}) has degree {d}");
            }
        }

        // Collinearity: every interior fragment's supporting line carries a
        // non-adjacent fragment.
        for &(lo, hi) in &edge_set {
            if lo.0.abs() > 20 || lo.1.abs() > 20 || hi.0.abs() > 20 || hi.1.abs() > 20 {
                continue;
            }
            let mates = lines
                .get(&line_key(lo, hi))
                .expect("fragment's own line missing");
            assert!(
                mates
                    .iter()
                    .any(|&(x, y)| x != lo && x != hi && y != lo && y != hi),
                "fragment ({lo:?}->{hi:?}) has no non-adjacent line mate"
            );
        }
    }
}

#[cfg(test)]
#[cfg(feature = "girih")]
mod girih_tests {
    use super::tests::{check_covers, check_det, check_rotated};
    use crate::girih;
    use tess_core::geometry::{BBox, Vec2};

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
    use tess_core::geometry::{BBox, Vec2};
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

#[cfg(test)]
#[cfg(feature = "pavers")]
mod pavers_tests {
    use super::pavers_exact_is_sym;
    use super::tests::{check_covers, check_det, check_rotated};
    use crate::basketweave;
    use crate::herringbone;
    use crate::running_bond;
    use tess_core::geometry::{BBox, Vec2};

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
    feature = "khatam_girih",
feature = "kuniform",
    feature = "laves",
    feature = "pavers",
    feature = "pentagon13",
    feature = "windmill"
))]
mod lattice_culling_tests {
    use super::lattice::LatticeTiling;
    use crate::{
        archimedean, basketweave, cairo, girih, girih_sc, girih_w85, hexagon, herringbone,
        khatam_girih, kuniform, laves, pentagon13, running_bond, square, triangle, windmill,
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
            ("cairo-collinear", cairo::cairo_collinear()),
            ("girih", girih::girih_shesh_band()),
            ("girih-sc", girih_sc::girih_star_cross()),
            ("girih-w85", girih_w85::girih_w85()),
            ("khatam-girih", khatam_girih::khatam_girih()),
            ("kuniform-p6m", kuniform::kuniform_p6m()),
            ("kuniform-pmm", kuniform::kuniform_pmm()),
            ("tetrakis-square", laves::tetrakis_square()),
            ("rhombille", laves::rhombille()),
            ("triakis-triangular", laves::triakis_triangular()),
            ("deltoidal-trihexagonal", laves::deltoidal_trihexagonal()),
            ("kisrhombille", laves::kisrhombille()),
            ("prismatic-pentagonal", laves::prismatic_pentagonal()),
            ("floret-pentagonal", laves::floret_pentagonal()),
("windmill", windmill::spec()),
            ("basketweave", basketweave::basketweave()),
            ("herringbone", herringbone::herringbone()),
            ("running-bond", running_bond::running_bond()),
            ("pentagon-13", pentagon13::spec()),
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
        assert_eq!(t.category, TilingCategory::Wallpaper);
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
        assert_eq!(t.category, TilingCategory::Wallpaper);
    }
}
