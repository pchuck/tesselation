//! Quasiperiodic family: substitution-based quasiperiodic sets: Penrose (P3/P1/P2/pentagons),
//! Ammann-Beenker (+rhomb-triangle), shield, stampfli, fractalizing, golden bee,
//! golden triangle and golden rhomboid
//!
//! Owned here: module declarations, `register` (the family's registry
//! slice, in within-tab order), `classification` arms, the family's
//! lattice-culling entries and the family census pin. The family doc is
//! `docs/families/quasiperiodic.md`.

use tess_core::Registry;
use tess_core::classify::TilingClass;

pub mod ammann_beenker;
pub mod ammann_beenker_tri;
pub mod shield;
pub mod stampfli;
pub mod golden_bee;
pub mod golden_triangle;
pub mod golden_rhomboid;
pub mod fractalizing;
pub mod penrose;
pub mod penrose_p1;
pub mod penrose_p2;
pub mod penrose_pentagon;
pub mod glb_binary_n7;
pub mod fibonacci_fib;
pub mod wis_12_fold;
pub mod girih_14_fold;
pub mod ammann_a4;
pub mod ammann_a3;

/// The family's registry slice, in within-tab order.
pub fn register(mut r: Registry) -> Registry {
        r = r.register(ammann_beenker::AmmannBeenkerTiling);
        r = r.register(ammann_beenker_tri::AmmannBeenkerTriTiling);
        r = r.register(shield::ShieldTiling);
        r = r.register(stampfli::StampfliTiling);
        r = r.register(golden_bee::GoldenBeeTiling);
        r = r.register(golden_triangle::GoldenTriangleTiling);
        r = r.register(golden_rhomboid::GoldenRhomboidTiling);
        r = r.register(fractalizing::FractalizingTiling);
        r = r.register(penrose::PenroseTiling);
        r = r.register(penrose_p1::PenroseP1Tiling);
        r = r.register(penrose_p2::PenroseP2Tiling);
        r = r.register(penrose_pentagon::PenrosePentagonTiling);
        r = r.register(glb_binary_n7::GlbBinaryN7Tiling);
        r = r.register(fibonacci_fib::FibonacciFibTiling);
        r = r.register(wis_12_fold::Wis12FoldTiling);
        r = r.register(girih_14_fold::Girih14FoldTiling);
        r = r.register(ammann_a4::AmmannA4Tiling);
        r = r.register(ammann_a3::AmmannA3Tiling);
    r
}

/// The family's classification arms (see `crate::classify` for the
/// composer and the shared constructor helpers).
pub(crate) fn classification(id: &str) -> Option<TilingClass> {
    // Arms are verbatim from the pre-split table (they yield
    // `TilingClass`); an unknown id returns None so the composer can
    // fall through to later families.
    Some(match id {
        "ammann-beenker" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("1+√2"),
            point_order: Some(8),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "ammann-beenker-tri" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("1+√2"),
            point_order: Some(8),
            translational: Some("quasiperiodic"),
            prototiles: Some(3),
            ..TilingClass::EMPTY
        },
        "shield" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("√(2+√3)"),
            point_order: Some(12),
            source: Some("Gähler"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "stampfli" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("2+√3"),
            point_order: Some(12),
            source: Some("Stampfli"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "golden-bee" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("√φ"),
            source: Some("Vince"),
            year: Some("2014"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "golden-triangle" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("√(−τ)"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "golden-rhomboid" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("φ"),
            // No point_order: the shipped tiling is the single-seed 60°
            // wedge grown from R0 (see golden_rhomboid.rs "Covered
            // domain") — no nontrivial rotation maps it to itself, so a
            // 6-fold badge would overstate the geometry (review A5).
            // The substitution's 18 anchored frames are C6-equivariant,
            // but equivariant frames are not a realized symmetry.
            source: Some("Andritz"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "fractalizing-3-12-12" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("fractalizing substitution"),
            base_tiling: Some("3.12.12"),
            inflation: Some("2+√3"),
            point_order: Some(12),
            translational: Some("limit-periodic"),
            ..TilingClass::EMPTY
        },
        "penrose" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("φ"),
            point_order: Some(5),
            source: Some("Penrose"),
            year: Some("1974"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "penrose-p1" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("pentagon deflation"),
            inflation: Some("φ²"),
            point_order: Some(5),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "penrose_p2" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("half-tile substitution"),
            inflation: Some("φ"),
            point_order: Some(5),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "penrose-pentagon" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("φ"),
            point_order: Some(5),
            tile_set: Some("five pentagons"),
            chirality: Some("homochiral"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "glb-binary-n7" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("√(2+2cos(π/7))"),
            point_order: Some(7),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "fibonacci-fibonacci" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("φ²"),
            point_order: Some(4),
            prototiles: Some(3),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "wis-12-fold" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("2+√3"),
            point_order: Some(12),
            prototiles: Some(3),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "girih-14-fold" => TilingClass {
            families: &["Quasiperiodic"],
            inflation: Some("2b²+3b (b = 2cos(2π/7))"),
            point_order: Some(14),
            prototiles: Some(11),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "ammann-a4" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("1+√2"),
            point_order: Some(8),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "ammann-a3" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("φ"),
            prototiles: Some(3),
            translational: Some("quasiperiodic"),
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
        "ammann-beenker",
        "ammann-beenker-tri",
        "shield",
        "stampfli",
        "golden-bee",
        "golden-triangle",
        "golden-rhomboid",
        "fractalizing-3-12-12",
        "penrose",
        "penrose-p1",
        "penrose_p2",
        "penrose-pentagon",
        "glb-binary-n7",
        "fibonacci-fibonacci",
        "wis-12-fold",
        "girih-14-fold",
        "ammann-a4",
        "ammann-a3",
    ];

    #[test]
    fn family_census_matches() {
        let r = register(Registry::new());
        assert_eq!(
            r.len(),
            18,
            "quasiperiodic family census — update when adding a tiling to this family"
        );
        let have: BTreeSet<&str> = r.iter().map(|t| t.id()).collect();
        let want: BTreeSet<&str> = IDS.iter().copied().collect();
        assert_eq!(
            have, want,
            "quasiperiodic family membership drift: register() vs IDS"
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
mod penrose_tests {
    use crate::tests::{check_coverage, check_det, check_rotated};
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

#[cfg(test)]
mod penrose_p1_tests {
    use crate::tests::{check_covers, check_det, check_rotated};
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

#[cfg(test)]
mod penrose_p2_tests {
    use crate::tests::{check_coverage, check_det, check_rotated};
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

#[cfg(test)]
mod penrose_pentagon_tests {
    use crate::tests::{check_coverage, check_det, check_rotated};
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

#[cfg(test)]
mod ammann_beenker_tests {
    use crate::tests::{check_coverage, check_det, check_rotated};
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

#[cfg(test)]
mod ammann_beenker_tri_tests {
    use crate::tests::{check_coverage, check_det, check_rotated};
    use crate::ammann_beenker_tri;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn ammann_beenker_tri_covers_region() {
        let t = ammann_beenker_tri::AmmannBeenkerTriTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(
            &t,
            "ammann-beenker-tri",
            &p,
            BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
        );
    }

    #[test]
    fn ammann_beenker_tri_rotated_scaled() {
        check_rotated(
            &ammann_beenker_tri::AmmannBeenkerTriTiling,
            "ammann-beenker-tri(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn ammann_beenker_tri_deterministic() {
        check_det(
            &ammann_beenker_tri::AmmannBeenkerTriTiling,
            "ammann-beenker-tri",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(test)]
mod stampfli_tests {
    use crate::tests::{check_coverage, check_det, check_rotated};
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

#[cfg(test)]
mod fractalizing_tests {
    use crate::tests::{check_coverage, check_det, check_rotated};
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

#[cfg(test)]
mod shield_tests {
    use crate::tests::{check_coverage, check_det, check_rotated};
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

