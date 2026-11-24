//! IH27 (pgg pentagon) — the 1-isohedral pentagon template on the Cairo
//! net (V3.3.4.3.4, dual of the snub square), wallpaper group pgg, four
//! aspects. Pinned exact parameters (7/50, 7/15, 8/15); rational
//! coefficients.

use tess_core::params::{ParamId, ParamSpec};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::TilingCategory;

use crate::lattice::SIMILARITY_PARAMS;

use super::data_ih27::{AC, ADJ, CURVES, NA, NP, NV, ORBIT_D, ORI, PARAMS, SID, TC, VC};
use super::engine::TypeData;
use super::instance::IhTiling;

const SHAPE_IDS: [ParamId; NP] = [
    ParamId::new("ih27-p1"),
    ParamId::new("ih27-p2"),
    ParamId::new("ih27-p3"),
];

const DEFAULTS: [f64; NP] = [0.14, 7.0 / 15.0, 8.0 / 15.0];

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
    id: "ih27-pgg-pentagon",
    name: "IH27 pentagon (pgg)",
    description: "Kaplan isohedral template IH27: pentagon with one rotationally-paired (U) \
                  edge orbit and two freeform (J) orbits, wallpaper group pgg, four aspects; \
                  Cairo-net topology (V3.3.4.3.4, dual of the snub square) with exact \
                  per-orbit edge curves.",
    data: &DATA,
    shape_ids: &SHAPE_IDS,
    defaults: &DEFAULTS,
    specs: SPECS,
    group: WallpaperGroup::Pgg,
    lattice: LatticeKind::Rectangular,
    category: TilingCategory::Isohedral,
    secondary: &[TilingCategory::Laves, TilingCategory::PentagonTypes],
    symmetry_notes: "Kaplan IH27; pgg, 1 U + 2 J edge orbits; Tactile incidence-symbol data",
};

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::super::data_ih27::{EXPECTED_T1, EXPECTED_T2, EXPECTED_VERTS};
    use super::super::engine::test_support as sup;
    use super::*;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::tiling::Tiling as _;

    #[test]
    fn ih27_covers_region() {
        check_covers(
            &TILING,
            "ih27-pgg-pentagon",
            super::super::instance::region(-4.0, -3.0, 6.0, 5.0),
        );
    }

    #[test]
    fn ih27_rotated_scaled() {
        check_rotated(
            &TILING,
            "ih27-pgg-pentagon(rot30,scale0.7)",
            super::super::instance::region(-6.0, -6.0, 6.0, 6.0),
            0.7,
            30.0,
        );
    }

    #[test]
    fn ih27_deterministic() {
        check_det(
            &TILING,
            "ih27-pgg-pentagon",
            super::super::instance::region(-3.0, -3.0, 10.0, 10.0),
        );
    }

    #[test]
    fn ih27_covers_exactly_once() {
        let p = tess_core::params::ParamSet::default_for(TILING.params());
        sup::check_exactly_once(
            &TILING,
            "ih27-pgg-pentagon",
            &p,
            super::super::instance::region(-1.5, -1.5, 2.5, 2.0),
            14,
            11,
        );
    }

    #[test]
    fn ih27_shared_edge_identity() {
        let census =
            sup::check_shared_edge_identity(&DATA, &DEFAULTS, &SHAPE_IDS, "ih27-pgg-pentagon", 2);
        println!("ih27 shared-edge f64 census (orbit, direction) -> count: {census:?}");
        assert_eq!(
            census,
            BTreeMap::from([((0, "rev"), 82), ((1, "fwd"), 180), ((2, "fwd"), 180)])
        );
    }

    #[test]
    fn ih27_aspect_symmetry() {
        sup::check_aspect_symmetry(&DATA, &DEFAULTS, &SHAPE_IDS, "ih27-pgg-pentagon");
    }

    #[test]
    fn ih27_orbit_d_census() {
        let hist = sup::orbit_d_census(&DATA, &DEFAULTS, &SHAPE_IDS, "ih27-pgg-pentagon");
        assert_eq!(
            hist,
            BTreeMap::from([
                (0, BTreeMap::from([("ROT", 4)])),
                (1, BTreeMap::from([("ID", 8)])),
                (2, BTreeMap::from([("ID", 8)])),
            ])
        );
    }

    #[test]
    fn ih27_pinned_transcription_spot_check() {
        let g = sup::geom_at_defaults(&DATA, &DEFAULTS, &SHAPE_IDS);
        for (k, (ex, ey)) in EXPECTED_VERTS.iter().enumerate() {
            let (qx, qy) = g.verts[k];
            assert!(
                (qx.to_f64() - ex).abs() < 1e-9 && (qy.to_f64() - ey).abs() < 1e-9,
                "ih27 vertex {k}: exact ({qx:?}, {qy:?}) vs expected ({ex}, {ey})"
            );
        }
        assert!(
            (g.t1.0.to_f64() - EXPECTED_T1.0).abs() < 1e-9
                && (g.t1.1.to_f64() - EXPECTED_T1.1).abs() < 1e-9
                && (g.t2.0.to_f64() - EXPECTED_T2.0).abs() < 1e-9
                && (g.t2.1.to_f64() - EXPECTED_T2.1).abs() < 1e-9,
            "ih27 translation vectors drifted from the pinned values"
        );
    }

    #[test]
    fn ih27_pentagon_tiles_on_cairo_net() {
        // Topology cross-check (McLean: IH26–29 are topological type
        // 3².4.3.4): every interior tile has exactly 5 edges (pentagons).
        let g = sup::geom_at_defaults(&DATA, &DEFAULTS, &SHAPE_IDS);
        assert_eq!(DATA.nv, 5, "ih27 tiles are pentagons");
        // The lattice vectors span a non-degenerate cell.
        let det = g.t1.0.mul(g.t2.1).sub(g.t1.1.mul(g.t2.0));
        assert!(
            det.to_f64().abs() > 1.0,
            "ih27 lattice cell degenerate: {det:?}"
        );
    }
}
