//! IH1 (p1 hexagon) — the unconstrained 1-isohedral hexagon tiling:
//! generic (asymmetric) hexagon on the 6³ honeycomb net, three freeform
//! (J) edge orbits, wallpaper group p1. Pinned exact parameters
//! (3/20, 3/5, 1/5, 11/20); all coefficients rational (Q ⊂ Q(√3)).

use tess_core::params::{ParamId, ParamSpec};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::TilingCategory;

use crate::lattice::SIMILARITY_PARAMS;

use super::data_ih1::{AC, ADJ, CURVES, NA, NP, NV, ORBIT_D, ORI, PARAMS, SID, TC, VC};
use super::engine::TypeData;
use super::instance::IhTiling;

/// Shape-parameter ids (engine order = `specs` order, shape params first).
const SHAPE_IDS: [ParamId; NP] = [
    ParamId::new("ih1-p1"),
    ParamId::new("ih1-p2"),
    ParamId::new("ih1-p3"),
    ParamId::new("ih1-p4"),
];

/// Float defaults as declared in `specs` (the pinned exact values of
/// `PARAMS` to UI precision; the engine recognizes these bit-exactly and
/// resolves to the pinned exact rationals — design doc §4).
const DEFAULTS: [f64; NP] = [0.15, 0.6, 0.2, 0.55];

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
        "p3 (vertex C offset)",
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
    id: "ih1-p1-hexagon",
    name: "IH1 hexagon (p1)",
    description: "Kaplan isohedral template IH1: the unconstrained asymmetric hexagon tiling, \
                  wallpaper group p1, three freeform (J) edge orbits; incidence symbol over the \
                  6^3 honeycomb net with shaped edges as exact per-orbit polylines.",
    data: &DATA,
    shape_ids: &SHAPE_IDS,
    defaults: &DEFAULTS,
    specs: SPECS,
    group: WallpaperGroup::P1,
    lattice: LatticeKind::Oblique,
    category: TilingCategory::Isohedral,
    secondary: &[TilingCategory::Laves],
    symmetry_notes: "Kaplan IH1; p1, three J edge orbits; Tactile incidence-symbol data",
};

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::super::data_ih1::{EXPECTED_T1, EXPECTED_T2, EXPECTED_VERTS};
    use super::super::engine::test_support as sup;
    use super::*;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::tiling::Tiling as _;

    #[test]
    fn ih1_covers_region() {
        check_covers(
            &TILING,
            "ih1-p1-hexagon",
            super::super::instance::region(-4.0, -3.0, 6.0, 5.0),
        );
    }

    #[test]
    fn ih1_rotated_scaled() {
        check_rotated(
            &TILING,
            "ih1-p1-hexagon(rot30,scale0.7)",
            super::super::instance::region(-6.0, -6.0, 6.0, 6.0),
            0.7,
            30.0,
        );
    }

    #[test]
    fn ih1_deterministic() {
        check_det(
            &TILING,
            "ih1-p1-hexagon",
            super::super::instance::region(-3.0, -3.0, 10.0, 10.0),
        );
    }

    #[test]
    fn ih1_covers_exactly_once() {
        let p = tess_core::params::ParamSet::default_for(TILING.params());
        sup::check_exactly_once(
            &TILING,
            "ih1-p1-hexagon",
            &p,
            super::super::instance::region(-1.5, -1.5, 2.5, 2.0),
            14,
            11,
        );
    }

    #[test]
    fn ih1_shared_edge_identity() {
        let census =
            sup::check_shared_edge_identity(&DATA, &DEFAULTS, &SHAPE_IDS, "ih1-p1-hexagon", 2);
        println!("ih1 shared-edge f64 census (orbit, direction) -> count: {census:?}");
        // All three orbits pair as reversals: each adjacency joins a
        // non-reversed edge (0..2) with a reversed one (3..5), so the
        // two tiles' emitted sequences run in opposite directions along
        // the shared edge (bit-identical up to that reversal).
        assert_eq!(
            census,
            BTreeMap::from([((0, "rev"), 40), ((1, "rev"), 40), ((2, "rev"), 32)])
        );
    }

    #[test]
    fn ih1_aspect_symmetry() {
        sup::check_aspect_symmetry(&DATA, &DEFAULTS, &SHAPE_IDS, "ih1-p1-hexagon");
    }

    #[test]
    fn ih1_orbit_d_census() {
        let hist = sup::orbit_d_census(&DATA, &DEFAULTS, &SHAPE_IDS, "ih1-p1-hexagon");
        assert_eq!(
            hist,
            BTreeMap::from([
                (0, BTreeMap::from([("ID", 2)])),
                (1, BTreeMap::from([("ID", 2)])),
                (2, BTreeMap::from([("ID", 2)])),
            ])
        );
    }

    #[test]
    fn ih1_pinned_transcription_spot_check() {
        let g = sup::geom_at_defaults(&DATA, &DEFAULTS, &SHAPE_IDS);
        for (k, (ex, ey)) in EXPECTED_VERTS.iter().enumerate() {
            let (qx, qy) = g.verts[k];
            assert!(
                (qx.to_f64() - ex).abs() < 1e-9 && (qy.to_f64() - ey).abs() < 1e-9,
                "ih1 vertex {k}: exact ({qx:?}, {qy:?}) vs expected ({ex}, {ey})"
            );
        }
        assert!(
            (g.t1.0.to_f64() - EXPECTED_T1.0).abs() < 1e-9
                && (g.t1.1.to_f64() - EXPECTED_T1.1).abs() < 1e-9
                && (g.t2.0.to_f64() - EXPECTED_T2.0).abs() < 1e-9
                && (g.t2.1.to_f64() - EXPECTED_T2.1).abs() < 1e-9,
            "ih1 translation vectors drifted from the pinned values"
        );
    }
}
