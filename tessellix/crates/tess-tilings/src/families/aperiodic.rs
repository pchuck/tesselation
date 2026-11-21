//! Aperiodic family: aperiodic monotiles and forcing sets: hat, turtle, spectre,
//! Taylor-Socolar, Jeandel-Rao, half-hex
//!
//! Owned here: module declarations, `register` (the family's registry
//! slice, in within-tab order), `classification` arms, the family's
//! lattice-culling entries and the family census pin. The family doc is
//! `docs/families/aperiodic.md`.

use tess_core::classify::TilingClass;
use tess_core::Registry;

pub mod half_hex;
pub mod hat;
pub mod jr_aperiodic;
pub mod spectre;
pub mod taylor_socolar;
pub mod turtle;

/// The family's registry slice, in within-tab order.
pub fn register(mut r: Registry) -> Registry {
    r = r.register(hat::HatTiling);
    r = r.register(turtle::TurtleTiling);
    r = r.register(spectre::SpectreTiling);
    r = r.register(taylor_socolar::TaylorSocolarTiling);
    r = r.register(jr_aperiodic::JeandelRaoTiling);
    r = r.register(half_hex::HalfHexTiling);
    r
}

/// The family's classification arms (see `crate::classify` for the
/// composer and the shared constructor helpers).
pub(crate) fn classification(id: &str) -> Option<TilingClass> {
    // Arms are verbatim from the pre-split table (they yield
    // `TilingClass`); an unknown id returns None so the composer can
    // fall through to later families.
    Some(match id {
        "hat" => TilingClass {
            families: &["Aperiodic"],
            method: Some("metatile substitution"),
            inflation: Some("φ²"),
            prototiles: Some(1),
            reflections: Some(true),
            chirality: Some("mixed"),
            source: Some("Smith et al."),
            year: Some("2023"),
            aperiodic_status: Some("einstein"),
            translational: Some("aperiodic"),
            ..TilingClass::EMPTY
        },
        "turtle" => TilingClass {
            families: &["Aperiodic"],
            method: Some("9-label substitution"),
            prototiles: Some(1),
            chirality: Some("homochiral"),
            source: Some("Smith et al."),
            year: Some("2023"),
            aperiodic_status: Some("einstein"),
            translational: Some("aperiodic"),
            ..TilingClass::EMPTY
        },
        "spectre" => TilingClass {
            families: &["Aperiodic"],
            method: Some("9-label substitution"),
            prototiles: Some(1),
            chirality: Some("homochiral"),
            source: Some("Smith et al."),
            year: Some("2023"),
            aperiodic_status: Some("chiral einstein"),
            translational: Some("aperiodic"),
            ..TilingClass::EMPTY
        },
        "taylor-socolar" => TilingClass {
            families: &["Aperiodic"],
            method: Some("substitution"),
            prototiles: Some(1),
            source: Some("Taylor & Socolar"),
            year: Some("2010"),
            aperiodic_status: Some("strongly aperiodic"),
            translational: Some("aperiodic"),
            ..TilingClass::EMPTY
        },
        "jeandel-rao" => TilingClass {
            families: &["Aperiodic"],
            method: Some("Wang tiles"),
            prototiles: Some(11),
            source: Some("Jeandel & Rao"),
            year: Some("2015"),
            aperiodic_status: Some("strongly aperiodic"),
            tile_set: Some("Wang tile set"),
            translational: Some("aperiodic"),
            ..TilingClass::EMPTY
        },
        "half-hex" => TilingClass {
            families: &["Aperiodic"],
            method: Some("substitution"),
            prototiles: Some(1),
            source: Some("Grünbaum & Shephard; Frettlöh"),
            year: Some("1987"),
            aperiodic_status: Some("limit-periodic"),
            translational: Some("limit-periodic"),
            ..TilingClass::EMPTY
        },
        _ => return None,
    })
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
        "hat",
        "turtle",
        "spectre",
        "taylor-socolar",
        "jeandel-rao",
        "half-hex",
    ];

    #[test]
    fn family_census_matches() {
        let r = register(Registry::new());
        assert_eq!(
            r.len(),
            6,
            "aperiodic family census — update when adding a tiling to this family"
        );
        let have: BTreeSet<&str> = r.iter().map(|t| t.id()).collect();
        let want: BTreeSet<&str> = IDS.iter().copied().collect();
        assert_eq!(
            have, want,
            "aperiodic family membership drift: register() vs IDS"
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
mod hat_tests {
    use crate::hat;
    use crate::tests::{check_coverage, check_det, check_rotated};
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

#[cfg(test)]
mod turtle_tests {
    use crate::tests::{check_coverage, check_det, check_rotated};
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

#[cfg(test)]
mod spectre_tests {
    use crate::spectre;
    use crate::tests::{check_coverage, check_det, check_rotated};
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

#[cfg(test)]
mod half_hex_tests {
    use super::half_hex;
    use crate::tests::{check_coverage, check_det, check_rotated};
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn half_hex_covers_region() {
        let t = half_hex::HalfHexTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "half-hex", &p, BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)));
    }

    /// The finite covered patch (2^d-scaled half-hexagon, d picked from
    /// the view diagonal) is view-anchored, so distant views are in-
    /// domain when their picked depth's patch contains them: three far
    /// windows (d = 7 / 8 / 8 respectively) verify the depth rule, the
    /// recentering and the clip-area accounting away from the origin.
    #[test]
    fn half_hex_covers_large_and_distant_views() {
        let t = half_hex::HalfHexTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        for (cx, cy, w, h) in [
            (45.0, -12.0, 0.3, 0.2),
            (-85.0, 15.0, 0.15, 0.1),
            (150.0, 40.0, 0.06, 0.04),
        ] {
            check_coverage(
                &t,
                "half-hex",
                &p,
                BBox::new(v2(cx - w, cy - h), v2(cx + w, cy + h)),
            );
        }
    }

    #[test]
    fn half_hex_rotated_scaled() {
        check_rotated(
            &half_hex::HalfHexTiling,
            "half-hex(rot25,scale0.6)",
            BBox::new(v2(-3.0, -3.0), v2(3.0, 3.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn half_hex_deterministic() {
        check_det(
            &half_hex::HalfHexTiling,
            "half-hex",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}
