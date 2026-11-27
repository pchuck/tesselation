//! IH6 (pgg hexagon) — the 1-isohedral hexagon template with two pairs
//! of rotationally-paired (U) edges and two freeform (J) orbits, wall-
//! paper group pgg, four aspects. Pinned exact parameters
//! (1/8, 1/2, 9/40, 5/8, 1/2); rational coefficients.

use tess_core::params::{ParamId, ParamSpec};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::TilingCategory;

use crate::lattice::SIMILARITY_PARAMS;

use super::data_ih6::{AC, ADJ, CURVES, NA, NP, NV, ORBIT_D, ORI, PARAMS, SID, TC, VC};
use super::engine::TypeData;
use super::instance::IhTiling;

const SHAPE_IDS: [ParamId; NP] = [
    ParamId::new("ih6-p1"),
    ParamId::new("ih6-p2"),
    ParamId::new("ih6-p3"),
    ParamId::new("ih6-p4"),
    ParamId::new("ih6-p5"),
];

const DEFAULTS: [f64; NP] = [0.125, 0.5, 0.225, 0.625, 0.5];

static SPECS: &[ParamSpec] = &[
    ParamSpec::float(
        SHAPE_IDS[0],
        "p1 (vertex A offset)",
        0.0,
        1.0,
        0.01,
        DEFAULTS[0],
    ),
    ParamSpec::float(
        SHAPE_IDS[1],
        "p2 (vertex B offset)",
        0.0,
        1.0,
        0.01,
        DEFAULTS[1],
    ),
    ParamSpec::float(
        SHAPE_IDS[2],
        "p3 (edge C shape)",
        0.0,
        1.0,
        0.01,
        DEFAULTS[2],
    ),
    ParamSpec::float(
        SHAPE_IDS[3],
        "p4 (vertex D offset)",
        0.0,
        1.0,
        0.01,
        DEFAULTS[3],
    ),
    ParamSpec::float(
        SHAPE_IDS[4],
        "p5 (edge E shape)",
        0.0,
        1.0,
        0.01,
        DEFAULTS[4],
    ),
    SIMILARITY_PARAMS[0],
    SIMILARITY_PARAMS[1],
];

static DATA: TypeData = TypeData {
    np: NP,
    na: NA,
    nv: NV,
    sid: SID,
    ori: ORI,
    adj: ADJ,
    params: PARAMS,
    vc: VC,
    tc: TC,
    ac: AC,
    curves: CURVES,
    orbit_d: ORBIT_D,
};

pub(crate) const TILING: IhTiling = IhTiling {
    id: "ih6-pgg-hexagon",
    name: "IH6 hexagon (pgg)",
    description: "IH6 incidence symbol\nKaplan isohedral template IH6 — hexagon with two rotationally-paired (U) \
                  edge pairs and two freeform (J) orbits, wallpaper group pgg, four aspects; \
                  incidence symbol over the 6³ honeycomb net with exact per-orbit edge curves.",
    data: &DATA,
    shape_ids: &SHAPE_IDS,
    defaults: &DEFAULTS,
    specs: SPECS,
    group: WallpaperGroup::Pgg,
    lattice: LatticeKind::Rectangular,
    category: TilingCategory::Isohedral,
    secondary: &[TilingCategory::Laves],
    symmetry_notes: "Kaplan IH6; pgg, 2 U + 2 J edge orbits",
};

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::super::data_ih6::{EXPECTED_T1, EXPECTED_T2, EXPECTED_VERTS};
    use super::super::engine::test_support as sup;
    use super::*;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::tiling::Tiling as _;

    #[test]
    fn ih6_covers_region() {
        check_covers(
            &TILING,
            "ih6-pgg-hexagon",
            super::super::instance::region(-4.0, -3.0, 6.0, 5.0),
        );
    }

    #[test]
    fn ih6_rotated_scaled() {
        check_rotated(
            &TILING,
            "ih6-pgg-hexagon(rot30,scale0.7)",
            super::super::instance::region(-6.0, -6.0, 6.0, 6.0),
            0.7,
            30.0,
        );
    }

    #[test]
    fn ih6_deterministic() {
        check_det(
            &TILING,
            "ih6-pgg-hexagon",
            super::super::instance::region(-3.0, -3.0, 10.0, 10.0),
        );
    }

    #[test]
    fn ih6_covers_exactly_once() {
        let p = tess_core::params::ParamSet::default_for(TILING.params());
        sup::check_exactly_once(
            &TILING,
            "ih6-pgg-hexagon",
            &p,
            super::super::instance::region(-1.5, -1.5, 2.5, 2.0),
            14,
            11,
        );
    }

    #[test]
    fn ih6_shared_edge_identity() {
        let census =
            sup::check_shared_edge_identity(&DATA, &DEFAULTS, &SHAPE_IDS, "ih6-pgg-hexagon", 2);
        println!("ih6 shared-edge f64 census (orbit, direction) -> count: {census:?}");
        // U orbits pair by 180° rotation (reversed sequences), J orbits
        // pair identically.
        assert_eq!(
            census,
            BTreeMap::from([
                ((0, "rev"), 82),
                ((1, "fwd"), 180),
                ((2, "rev"), 80),
                ((3, "fwd"), 180),
            ])
        );
    }

    #[test]
    fn ih6_aspect_symmetry() {
        sup::check_aspect_symmetry(&DATA, &DEFAULTS, &SHAPE_IDS, "ih6-pgg-hexagon");
    }

    #[test]
    fn ih6_orbit_d_census() {
        let hist = sup::orbit_d_census(&DATA, &DEFAULTS, &SHAPE_IDS, "ih6-pgg-hexagon");
        assert_eq!(
            hist,
            BTreeMap::from([
                (0, BTreeMap::from([("ROT", 4)])),
                (1, BTreeMap::from([("ID", 8)])),
                (2, BTreeMap::from([("ROT", 4)])),
                (3, BTreeMap::from([("ID", 8)])),
            ])
        );
    }

    #[test]
    fn ih6_pinned_transcription_spot_check() {
        let g = sup::geom_at_defaults(&DATA, &DEFAULTS, &SHAPE_IDS);
        for (k, (ex, ey)) in EXPECTED_VERTS.iter().enumerate() {
            let (qx, qy) = g.verts[k];
            assert!(
                (qx.to_f64() - ex).abs() < 1e-9 && (qy.to_f64() - ey).abs() < 1e-9,
                "ih6 vertex {k}: exact ({qx:?}, {qy:?}) vs expected ({ex}, {ey})"
            );
        }
        assert!(
            (g.t1.0.to_f64() - EXPECTED_T1.0).abs() < 1e-9
                && (g.t1.1.to_f64() - EXPECTED_T1.1).abs() < 1e-9
                && (g.t2.0.to_f64() - EXPECTED_T2.0).abs() < 1e-9
                && (g.t2.1.to_f64() - EXPECTED_T2.1).abs() < 1e-9,
            "ih6 translation vectors drifted from the pinned values"
        );
    }
}
