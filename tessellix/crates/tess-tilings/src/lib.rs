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
#[cfg(feature = "girih_qc")]
pub mod girih_qc;
#[cfg(feature = "girih_sc")]
pub mod girih_sc;
#[cfg(feature = "khatam_girih")]
pub mod khatam_girih;
#[cfg(feature = "laves")]
pub mod laves;
#[cfg(feature = "hexagon")]
pub mod hexagon;
#[cfg(feature = "hirschhorn")]
pub mod hirschhorn;
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
#[cfg(feature = "square")]
pub mod square;
#[cfg(feature = "triangle")]
pub mod triangle;
#[cfg(feature = "zellige")]
pub mod zellige;

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
    #[cfg(feature = "khatam_girih")]
    {
        r = r.register(khatam_girih::khatam_girih());
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
            ("Wallpaper", 5),
            ("Quasiperiodic", 6),
            ("Aperiodic", 6),
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

/// Brute-force cross-check of [`crate::lattice::LatticeTiling`]'s
/// index-range culling, the one part of the generator that could silently
/// drop (or invent) tiles at the edges of the query region: the per-cell
/// offsets that pad the index ranges are measured in lattice coordinates
/// (cell vertices mapped through `u(p) = A⁻¹ p`), and getting that wrong
/// for a sheared or tall cell would produce gaps that sample-based
/// coverage tests can miss. This test re-emits every tile from the public
/// table data over a wide fixed index window — no range math at all — and
/// requires exactly the same tile set as `generate`.
#[cfg(all(
    test,
    feature = "square",
    feature = "hexagon",
    feature = "triangle",
    feature = "archimedean",
    feature = "cairo",
    feature = "girih",
    feature = "girih_sc",
    feature = "khatam_girih",
    feature = "laves"
))]
mod lattice_culling_tests {
    use super::lattice::LatticeTiling;
    use crate::{
        archimedean, cairo, girih, girih_sc, hexagon, khatam_girih, laves, square, triangle,
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
            ("khatam-girih", khatam_girih::khatam_girih()),
            ("tetrakis-square", laves::tetrakis_square()),
            ("rhombille", laves::rhombille()),
            ("triakis-triangular", laves::triakis_triangular()),
            ("deltoidal-trihexagonal", laves::deltoidal_trihexagonal()),
            ("kisrhombille", laves::kisrhombille()),
            ("prismatic-pentagonal", laves::prismatic_pentagonal()),
            ("floret-pentagonal", laves::floret_pentagonal()),
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
