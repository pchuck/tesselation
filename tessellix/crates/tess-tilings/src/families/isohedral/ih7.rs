//! IH7 (p3 hexagon) — the 1-isohedral hexagon template with all three
//! edge orbits freeform (J) and 120°-rotated aspects, wallpaper group
//! p3, three aspects. The one shipped instance in Q(√3): Tactile's
//! √3-decimal coefficients are fitted exactly to a + b√3 (residual
//! < 1e-11; design doc §2.1). Pinned exact parameters (3/5, 1/5).
//! Edge orbits carry the showpiece J curves (4 interior control points).

use tess_core::params::{ParamId, ParamSpec};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::TilingCategory;

use crate::lattice::SIMILARITY_PARAMS;

use super::data_ih7::{AC, ADJ, CURVES, NA, NP, NV, ORBIT_D, ORI, PARAMS, SID, TC, VC};
use super::engine::TypeData;
use super::instance::IhTiling;

const SHAPE_IDS: [ParamId; NP] = [ParamId::new("ih7-p1"), ParamId::new("ih7-p2")];

const DEFAULTS: [f64; NP] = [0.6, 0.2];

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
    id: "ih7-p3-hexagon",
    name: "IH7 hexagon (p3)",
    description: "Kaplan isohedral template IH7: hexagon with three freeform (J) edge orbits \
                  under 120°-rotated aspects, wallpaper group p3, three aspects; the shipped \
                  record lives in Q(√3) (fitted exact coefficients) and carries the showpiece \
                  four-point edge curves.",
    data: &DATA,
    shape_ids: &SHAPE_IDS,
    defaults: &DEFAULTS,
    specs: SPECS,
    group: WallpaperGroup::P3,
    lattice: LatticeKind::Hexagonal,
    category: TilingCategory::Isohedral,
    secondary: &[TilingCategory::Laves],
    symmetry_notes: "Kaplan IH7; p3, 3 J edge orbits; Tactile incidence-symbol data",
};

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::super::data_ih7::{EXPECTED_T1, EXPECTED_T2, EXPECTED_VERTS};
    use super::super::engine::test_support as sup;
    use super::*;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::tiling::Tiling as _;

    #[test]
    fn ih7_covers_region() {
        check_covers(
            &TILING,
            "ih7-p3-hexagon",
            super::super::instance::region(-4.0, -3.0, 6.0, 5.0),
        );
    }

    #[test]
    fn ih7_rotated_scaled() {
        check_rotated(
            &TILING,
            "ih7-p3-hexagon(rot30,scale0.7)",
            super::super::instance::region(-6.0, -6.0, 6.0, 6.0),
            0.7,
            30.0,
        );
    }

    #[test]
    fn ih7_deterministic() {
        check_det(
            &TILING,
            "ih7-p3-hexagon",
            super::super::instance::region(-3.0, -3.0, 10.0, 10.0),
        );
    }

    #[test]
    fn ih7_covers_exactly_once() {
        let p = tess_core::params::ParamSet::default_for(TILING.params());
        sup::check_exactly_once(
            &TILING,
            "ih7-p3-hexagon",
            &p,
            super::super::instance::region(-1.5, -1.5, 2.5, 2.0),
            14,
            11,
        );
    }

    #[test]
    fn ih7_shared_edge_identity() {
        let census =
            sup::check_shared_edge_identity(&DATA, &DEFAULTS, &SHAPE_IDS, "ih7-p3-hexagon", 2);
        println!("ih7 shared-edge f64 census (orbit, direction) -> count: {census:?}");
        // All three orbits pair as reversals: the adjacency joins edges of
        // opposite reversal-status ((f,f) ↔ (f,t)), so the two tiles'
        // emitted sequences run in opposite directions along the shared
        // edge (bit-identical up to that reversal). spike4's raw-curve
        // census labeled the same pairings "fwd" before flag application.
        assert_eq!(
            census,
            BTreeMap::from([((0, "rev"), 150), ((1, "rev"), 112), ((2, "rev"), 112)])
        );
    }

    #[test]
    fn ih7_aspect_symmetry() {
        sup::check_aspect_symmetry(&DATA, &DEFAULTS, &SHAPE_IDS, "ih7-p3-hexagon");
    }

    #[test]
    fn ih7_orbit_d_census() {
        let hist = sup::orbit_d_census(&DATA, &DEFAULTS, &SHAPE_IDS, "ih7-p3-hexagon");
        assert_eq!(
            hist,
            BTreeMap::from([
                (0, BTreeMap::from([("ID", 6)])),
                (1, BTreeMap::from([("ID", 6)])),
                (2, BTreeMap::from([("ID", 6)])),
            ])
        );
    }

    #[test]
    fn ih7_pinned_transcription_spot_check() {
        let g = sup::geom_at_defaults(&DATA, &DEFAULTS, &SHAPE_IDS);
        for (k, (ex, ey)) in EXPECTED_VERTS.iter().enumerate() {
            let (qx, qy) = g.verts[k];
            assert!(
                (qx.to_f64() - ex).abs() < 1e-9 && (qy.to_f64() - ey).abs() < 1e-9,
                "ih7 vertex {k}: exact ({qx:?}, {qy:?}) vs expected ({ex}, {ey})"
            );
        }
        assert!(
            (g.t1.0.to_f64() - EXPECTED_T1.0).abs() < 1e-9
                && (g.t1.1.to_f64() - EXPECTED_T1.1).abs() < 1e-9
                && (g.t2.0.to_f64() - EXPECTED_T2.0).abs() < 1e-9
                && (g.t2.1.to_f64() - EXPECTED_T2.1).abs() < 1e-9,
            "ih7 translation vectors drifted from the pinned values"
        );
    }

    #[test]
    fn ih7_non_rational_coefficients_survive_exact_arithmetic() {
        // The IH7 record genuinely uses Q(√3) elements: at least one
        // vertex coordinate must be non-rational (b ≠ 0) and the aspect
        // transforms must be exact 120° rotations.
        let g = sup::geom_at_defaults(&DATA, &DEFAULTS, &SHAPE_IDS);
        assert!(
            g.verts
                .iter()
                .any(|(x, y)| !x.is_rational() || !y.is_rational()),
            "ih7 coefficients should include non-rational Q(√3) elements"
        );
    }
}
