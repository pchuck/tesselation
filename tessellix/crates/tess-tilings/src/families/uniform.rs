//! Uniform family: the 11 Archimedean/regular layouts (square, hexagon, triangle and
//! the eight 2-uniform-in-one-orbit Archimedeans)
//!
//! Owned here: module declarations, `register` (the family's registry
//! slice, in within-tab order), `classification` arms, the family's
//! lattice-culling entries and the family census pin. The family doc is
//! `docs/families/uniform.md`.

use tess_core::Registry;
use tess_core::classify::TilingClass;

pub mod square;
pub mod hexagon;
pub mod triangle;
pub mod archimedean;

/// The family's registry slice, in within-tab order.
pub fn register(mut r: Registry) -> Registry {
        r = r.register(square::spec());
        r = r.register(hexagon::spec());
        r = r.register(triangle::spec());
        r = r.register(archimedean::truncated_square());
        r = r.register(archimedean::trihexagonal());
        r = r.register(archimedean::snub_trihexagonal());
        r = r.register(archimedean::elongated_triangular());
        r = r.register(archimedean::rhombitrihexagonal());
        r = r.register(archimedean::truncated_trihexagonal());
        r = r.register(archimedean::truncated_hexagonal());
        r = r.register(archimedean::snub_square());
    r
}

/// The family's classification arms (see `crate::classify` for the
/// composer and the shared constructor helpers).
pub(crate) fn classification(id: &str) -> Option<TilingClass> {
    // Arms are verbatim from the pre-split table (they yield
    // `TilingClass`); an unknown id returns None so the composer can
    // fall through to later families.
    Some(match id {
        "square" => TilingClass {
            families: &["Uniform", "Laves"],
            vertex_config: Some("4.4.4.4"),
            schlafli: Some("{4,4}"),
            uniformity: Some("regular"),
            dual_of: Some("self-dual"),
            k: Some(1),
            laves_rank: Some(0),
            ..TilingClass::EMPTY
        },
        "triangle" => TilingClass {
            families: &["Uniform", "Laves"],
            vertex_config: Some("3.3.3.3.3.3"),
            schlafli: Some("{3,6}"),
            uniformity: Some("regular"),
            dual_of: Some("self-dual"),
            k: Some(1),
            laves_rank: Some(0),
            ..TilingClass::EMPTY
        },
        "hexagon" => TilingClass {
            families: &["Uniform", "Laves", "Wallpaper"],
            vertex_config: Some("6.6.6"),
            schlafli: Some("{6,3}"),
            uniformity: Some("regular"),
            dual_of: Some("self-dual"),
            k: Some(1),
            laves_rank: Some(0),
            ..TilingClass::EMPTY
        },
        "truncated-square" => TilingClass {
            families: &["Uniform", "Wallpaper"],
            vertex_config: Some("4.8.8"),
            schlafli: Some("t{4,4}"),
            uniformity: Some("semiregular"),
            dual_of: Some("tetrakis square"),
            k: Some(1),
            ..TilingClass::EMPTY
        },
        "truncated-hexagonal" => TilingClass {
            families: &["Uniform"],
            vertex_config: Some("3.12.12"),
            schlafli: Some("t{6,3}"),
            uniformity: Some("semiregular"),
            dual_of: Some("triakis triangular"),
            k: Some(1),
            ..TilingClass::EMPTY
        },
        "truncated-trihexagonal" => TilingClass {
            families: &["Uniform"],
            vertex_config: Some("4.6.12"),
            schlafli: Some("tr{3,6}"),
            uniformity: Some("semiregular"),
            dual_of: Some("kisrhombille"),
            k: Some(1),
            ..TilingClass::EMPTY
        },
        "rhombitrihexagonal" => TilingClass {
            families: &["Uniform"],
            vertex_config: Some("3.4.6.4"),
            schlafli: Some("rr{3,6}"),
            uniformity: Some("semiregular"),
            dual_of: Some("deltoidal trihexagonal"),
            k: Some(1),
            ..TilingClass::EMPTY
        },
        "trihexagonal" => TilingClass {
            families: &["Uniform"],
            vertex_config: Some("3.6.3.6"),
            schlafli: Some("r{3,6}"),
            uniformity: Some("semiregular"),
            dual_of: Some("rhombille"),
            k: Some(1),
            ..TilingClass::EMPTY
        },
        "elongated-triangular" => TilingClass {
            families: &["Uniform", "Wallpaper"],
            vertex_config: Some("3.3.3.4.4"),
            uniformity: Some("semiregular"),
            dual_of: Some("prismatic pentagonal"),
            k: Some(1),
            ..TilingClass::EMPTY
        },
        "snub-square" => TilingClass {
            families: &["Uniform", "Wallpaper"],
            vertex_config: Some("3.3.4.3.4"),
            schlafli: Some("s{4,4}"),
            uniformity: Some("semiregular"),
            dual_of: Some("cairo pentagonal"),
            k: Some(1),
            ..TilingClass::EMPTY
        },
        "snub-trihexagonal" => TilingClass {
            families: &["Uniform", "Wallpaper"],
            vertex_config: Some("3.3.3.3.6"),
            schlafli: Some("sr{3,6}"),
            uniformity: Some("semiregular"),
            dual_of: Some("floret pentagonal"),
            k: Some(1),
            // The prototiles (regular triangle/hexagon) are achiral, so
            // "homochiral" would say nothing: the chirality here is the
            // tiling's own — no reflection symmetry (p6, not p6m).
            chirality: Some("chiral"),
            ..TilingClass::EMPTY
        },
        _ => return None,
    })
}

/// This family's entries for the global lattice-culling cross-check
/// (`lattice_culling_tests` in `lib.rs`).
#[cfg(test)]
pub(crate) fn lattice_tilings() -> Vec<(&'static str, crate::lattice::LatticeTiling)> {
    vec![
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
    ]
}

#[cfg(test)]
mod census {
    use super::*;
    use std::collections::BTreeSet;

    /// The family's registry ids — by construction also its
    /// classification arm ids. Update alongside any membership change
    /// (this test fails on drift in either direction: a tiling
    /// registered but not classified, or classified but not
    /// registered).
    const IDS: &[&str] = &[
        "square",
        "triangle",
        "hexagon",
        "truncated-square",
        "truncated-hexagonal",
        "truncated-trihexagonal",
        "rhombitrihexagonal",
        "trihexagonal",
        "elongated-triangular",
        "snub-square",
        "snub-trihexagonal",
    ];

    #[test]
    fn family_census_matches() {
        let r = register(Registry::new());
        assert_eq!(
            r.len(),
            11,
            "uniform family census — update when adding a tiling to this family"
        );
        let have: BTreeSet<&str> = r.iter().map(|t| t.id()).collect();
        let want: BTreeSet<&str> = IDS.iter().copied().collect();
        assert_eq!(
            have, want,
            "uniform family membership drift: register() vs IDS"
        );
        for id in IDS {
            assert!(
                classification(id).is_some(),
                "{id} has no classification arm in this family"
            );
        }
    }
}

#[cfg(test)]
mod square_tests {
    use crate::tests::{check_covers, check_det, check_rotated};
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

#[cfg(test)]
mod hexagon_tests {
    use crate::tests::{check_covers, check_det, check_rotated};
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

#[cfg(test)]
mod triangle_tests {
    use crate::tests::{check_coverage, check_det, check_rotated};
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

#[cfg(test)]
mod archimedean_tests {
    use crate::tests::{check_coverage, check_covers, check_det, check_rotated};
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

