//! IH29 (p4g pentagon) — the 1-isohedral pentagon template on the Cairo
//! net (V3.3.4.3.4, dual of the snub square), wallpaper group p4g, four
//! aspects. Its base edge orbit pairs through BOTH ROT and ROFL (the
//! Klein four-group), forcing the base edge straight (design doc §1.2);
//! the second orbit is freeform (J). Pinned exact parameter (3/8);
//! rational coefficients.

use tess_core::params::{ParamId, ParamSpec};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::TilingCategory;

use crate::lattice::SIMILARITY_PARAMS;

use super::data_ih29::{AC, ADJ, CURVES, NA, NP, NV, ORBIT_D, ORI, PARAMS, SID, TC, VC};
use super::engine::TypeData;
use super::instance::IhTiling;

const SHAPE_IDS: [ParamId; NP] = [ParamId::new("ih29-p1")];

const DEFAULTS: [f64; NP] = [0.375];

static SPECS: &[ParamSpec] = &[
    ParamSpec::float(
        SHAPE_IDS[0],
        "p1 (vertex A offset)",
        0.0,
        1.0,
        0.01,
        DEFAULTS[0],
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
    id: "ih29-p4g-pentagon",
    name: "IH29 pentagon (p4g)",
    description: "Kaplan isohedral template IH29: pentagon whose base edge orbit pairs through \
                  both rotation and reflection (Klein group, forcing a straight edge) plus one \
                  freeform (J) orbit, wallpaper group p4g, four aspects; Cairo-net topology \
                  (V3.3.4.3.4, dual of the snub square).",
    data: &DATA,
    shape_ids: &SHAPE_IDS,
    defaults: &DEFAULTS,
    specs: SPECS,
    group: WallpaperGroup::P4g,
    lattice: LatticeKind::Square,
    category: TilingCategory::Isohedral,
    secondary: &[TilingCategory::Laves, TilingCategory::PentagonTypes],
    symmetry_notes: "Kaplan IH29; p4g, straight base edge (Klein-paired orbit) + J orbit; \
                     Tactile incidence-symbol data",
};

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::super::data_ih29::{EXPECTED_T1, EXPECTED_T2, EXPECTED_VERTS};
    use super::super::engine::test_support as sup;
    use super::super::field::Q;
    use super::*;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::tiling::Tiling as _;

    #[test]
    fn ih29_covers_region() {
        check_covers(
            &TILING,
            "ih29-p4g-pentagon",
            super::super::instance::region(-8.0, -8.0, 10.0, 10.0),
        );
    }

    #[test]
    fn ih29_rotated_scaled() {
        check_rotated(
            &TILING,
            "ih29-p4g-pentagon(rot30,scale0.7)",
            super::super::instance::region(-8.0, -8.0, 8.0, 8.0),
            0.7,
            30.0,
        );
    }

    #[test]
    fn ih29_deterministic() {
        check_det(
            &TILING,
            "ih29-p4g-pentagon",
            super::super::instance::region(-8.0, -8.0, 10.0, 10.0),
        );
    }

    #[test]
    fn ih29_covers_exactly_once() {
        let p = tess_core::params::ParamSet::default_for(TILING.params());
        sup::check_exactly_once(
            &TILING,
            "ih29-p4g-pentagon",
            &p,
            super::super::instance::region(-2.0, -2.0, 2.5, 2.0),
            12,
            10,
        );
    }

    #[test]
    fn ih29_shared_edge_identity() {
        let census =
            sup::check_shared_edge_identity(&DATA, &DEFAULTS, &SHAPE_IDS, "ih29-p4g-pentagon", 2);
        println!("ih29 shared-edge f64 census (orbit, direction) -> count: {census:?}");
        // Orbit 0 is the mixed Klein orbit: ROFL pairings emit forward
        // (ROFL fixes the baseline pointwise), ROT pairings reversed.
        // Orbit 1's J pairings split by reversal-status parity of the
        // joined edges (engine classifies the emitted sequences, after
        // each side's flag application; spike4's raw-curve census
        // labeled the same pairings "fwd" throughout).
        assert_eq!(
            census,
            BTreeMap::from([
                ((0, "fwd"), 32),
                ((0, "rev"), 50),
                ((1, "fwd"), 180),
                ((1, "rev"), 180),
            ])
        );
    }

    #[test]
    fn ih29_aspect_symmetry() {
        sup::check_aspect_symmetry(&DATA, &DEFAULTS, &SHAPE_IDS, "ih29-p4g-pentagon");
    }

    #[test]
    fn ih29_orbit_d_census() {
        let hist = sup::orbit_d_census(&DATA, &DEFAULTS, &SHAPE_IDS, "ih29-p4g-pentagon");
        assert_eq!(
            hist,
            BTreeMap::from([
                // The mixed orbit: both generators appear among the four
                // base-edge pairings — the case that forces a straight
                // canonical curve (design doc §1.2).
                (0, BTreeMap::from([("ROFL", 2), ("ROT", 2)])),
                (1, BTreeMap::from([("ID", 16)])),
            ])
        );
    }

    #[test]
    fn ih29_pinned_transcription_spot_check() {
        let g = sup::geom_at_defaults(&DATA, &DEFAULTS, &SHAPE_IDS);
        for (k, (ex, ey)) in EXPECTED_VERTS.iter().enumerate() {
            let (qx, qy) = g.verts[k];
            assert!(
                (qx.to_f64() - ex).abs() < 1e-9 && (qy.to_f64() - ey).abs() < 1e-9,
                "ih29 vertex {k}: exact ({qx:?}, {qy:?}) vs expected ({ex}, {ey})"
            );
        }
        assert!(
            (g.t1.0.to_f64() - EXPECTED_T1.0).abs() < 1e-9
                && (g.t1.1.to_f64() - EXPECTED_T1.1).abs() < 1e-9
                && (g.t2.0.to_f64() - EXPECTED_T2.0).abs() < 1e-9
                && (g.t2.1.to_f64() - EXPECTED_T2.1).abs() < 1e-9,
            "ih29 translation vectors drifted from the pinned values"
        );
    }

    #[test]
    fn ih29_base_edge_is_straight() {
        // The Klein-paired orbit (orbit 0) pins a straight edge: its
        // canonical curve must have no interior control points.
        let base = DATA.curves[0];
        assert_eq!(base.len(), 4, "orbit 0 curve = endpoints only, flat (x,y)");
        assert_eq!(
            (base[0], base[1], base[2], base[3]),
            (Q::zero(), Q::zero(), Q::one(), Q::zero()),
            "orbit 0 canonical curve is the unit baseline"
        );
    }
}
