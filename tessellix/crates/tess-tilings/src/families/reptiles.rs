//! Reptiles family: rep-tile nonperiodic layouts: chair, armchair, sphinx, P-pentomino
//!
//! Owned here: module declarations, `register` (the family's registry
//! slice, in within-tab order), `classification` arms, the family's
//! lattice-culling entries and the family census pin. The family doc is
//! `docs/families/reptiles.md`.

use tess_core::classify::TilingClass;
use tess_core::Registry;

use crate::classify::rep_tile;

pub mod armchair;
pub mod chair;
pub mod octiamond_reptile;
pub mod p_pentomino;
pub mod pregnant_chair;
pub mod sphinx;
pub mod tangram;

/// The family's registry slice, in within-tab order.
pub fn register(mut r: Registry) -> Registry {
    r = r.register(armchair::ArmchairTiling);
    r = r.register(chair::ChairTiling);
    r = r.register(octiamond_reptile::OctiamondReptileTiling);
    r = r.register(sphinx::SphinxTiling);
    r = r.register(p_pentomino::PentPentominoTiling);
    r = r.register(pregnant_chair::PregnantChairTiling);
    r = r.register(tangram::TangramTiling);
    r
}

/// The family's classification arms (see `crate::classify` for the
/// composer and the shared constructor helpers).
pub(crate) fn classification(id: &str) -> Option<TilingClass> {
    // Arms are verbatim from the pre-split table (they yield
    // `TilingClass`); an unknown id returns None so the composer can
    // fall through to later families.
    Some(match id {
        "armchair" => rep_tile("L-tetromino"),
        "chair" => rep_tile("L-tromino"),
        "sphinx" => rep_tile("sphinx hexiamond"),
        "p-pentomino" => rep_tile("P-pentomino"),
        "pregnant-chair" => rep_tile("pregnant chairs (large + small)"),
        "tangram" => TilingClass {
            families: &["Nonperiodic"],
            method: Some("substitution"),
            inflation: Some("√2"),
            prototiles: Some(5),
            tile_set: Some(
                "tangram: five pieces (two small triangles, medium triangle, large triangle, square, parallelogram)",
            ),
            translational: Some("limit-periodic"),
            chirality: Some("homochiral"),
            source: Some("Bielefeld substitution index"),
            ..TilingClass::EMPTY
        },
        "octiamond-reptile" => TilingClass {
            families: &["Nonperiodic"],
            method: Some("substitution"),
            inflation: Some("λ = 3"),
            prototiles: Some(1),
            tile_set: Some("octiamond reptile (trapezoidal 8-triomino of the triangular lattice)"),
            reflections: Some(true),
            chirality: Some("mixed"),
            translational: Some("aperiodic"),
            source: Some("Bielefeld substitution index"),
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
        "armchair",
        "chair",
        "octiamond-reptile",
        "sphinx",
        "p-pentomino",
        "pregnant-chair",
        "tangram",
    ];

    #[test]
    fn family_census_matches() {
        let r = register(Registry::new());
        assert_eq!(
            r.len(),
            7,
            "reptiles family census — update when adding a tiling to this family"
        );
        let have: BTreeSet<&str> = r.iter().map(|t| t.id()).collect();
        let want: BTreeSet<&str> = IDS.iter().copied().collect();
        assert_eq!(
            have, want,
            "reptiles family membership drift: register() vs IDS"
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
mod armchair_tests {
    use crate::armchair;
    use crate::tests::{check_coverage, check_det, check_rotated};
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn armchair_covers_region() {
        let t = armchair::ArmchairTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "armchair", &p, BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)));
    }

    #[test]
    fn armchair_rotated_scaled() {
        check_rotated(
            &armchair::ArmchairTiling,
            "armchair(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn armchair_deterministic() {
        check_det(
            &armchair::ArmchairTiling,
            "armchair",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(test)]
mod chair_tests {
    use crate::chair;
    use crate::tests::{check_coverage, check_det, check_rotated};
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

#[cfg(test)]
mod octiamond_reptile_tests {
    use super::octiamond_reptile;
    use crate::tests::{check_coverage, check_det, check_rotated};
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn octiamond_reptile_covers_region() {
        let t = octiamond_reptile::OctiamondReptileTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(
            &t,
            "octiamond-reptile",
            &p,
            BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
        );
    }

    #[test]
    fn octiamond_reptile_rotated_scaled() {
        check_rotated(
            &octiamond_reptile::OctiamondReptileTiling,
            "octiamond-reptile(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn octiamond_reptile_deterministic() {
        check_det(
            &octiamond_reptile::OctiamondReptileTiling,
            "octiamond-reptile",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(test)]
mod sphinx_tests {
    use crate::sphinx;
    use crate::tests::{check_coverage, check_det, check_rotated};
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

#[cfg(test)]
mod p_pentomino_tests {
    use crate::p_pentomino;
    use crate::tests::{check_coverage, check_det, check_rotated};
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
        check_coverage(
            &t,
            "p-pentomino",
            &p,
            BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
        );
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

#[cfg(test)]
mod tangram_tests {
    use super::tangram;
    use crate::tests::{check_coverage, check_det, check_rotated};
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn tangram_covers_region() {
        let t = tangram::TangramTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "tangram", &p, BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)));
    }

    #[test]
    fn tangram_rotated_scaled() {
        check_rotated(
            &tangram::TangramTiling,
            "tangram(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn tangram_deterministic() {
        check_det(
            &tangram::TangramTiling,
            "tangram",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}
