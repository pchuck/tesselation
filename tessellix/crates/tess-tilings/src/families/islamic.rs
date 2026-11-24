//! Islamic family: Islamic-geometric constructions: hexagram, zellige, girih family,
//! PIC/hankin contact-ray engines and their substrate tables, khatam, najm
//!
//! Owned here: module declarations, `register` (the family's registry
//! slice, in within-tab order), `classification` arms, the family's
//! lattice-culling entries and the family census pin. The family doc is
//! `docs/families/islamic.md`.

use tess_core::Registry;
use tess_core::classify::TilingClass;

use crate::classify::{hankin_pic, hankin_greedy};

pub mod hexagram;
pub mod zellige;
pub mod girih;
pub mod girih_bowtie;
pub mod girih_decagon;
pub mod girih_pentagon;
pub mod girih_rhombus;
pub mod girih_qc;
pub mod girih_decagon_patch;
pub mod andritz_bowtie_hexagon;
pub mod andritz_bowtie_hexagon_10gon_1;
pub mod andritz_bowtie_hexagon_10gon_2;
pub mod andritz_bowtie_hexagon_10gon_3;
pub mod girih_sc;
pub mod girih_w85;
pub mod pic_4_8_8;
pub mod pic_3_6_3_6;
pub mod pic_3_12_12;
pub mod hankin;
pub mod hankin_motifs;
pub mod hankin_4_8_8;
pub mod hankin_4_6_12;
pub mod hankin_4_8_8_rosette_transform;
pub mod hankin_hex;
pub mod hankin_3_12_12;
pub mod hankin_square_two_point;
pub mod hankin_3_6_3_6;
pub mod hankin_3_4_6_4;
pub mod hankin_3_3_3_4_4;
pub mod hankin_3_3_4_3_4;
pub mod hankin_4_8_8_motifs;
pub mod hankin_hex_motifs;
pub mod khatam_girih;
pub mod najm;
pub mod najm_63_motifs;
pub mod rosette_transform;

/// The family's registry slice, in within-tab order.
pub fn register(mut r: Registry) -> Registry {
        r = r.register(hexagram::HexagramTiling);
        r = r.register(zellige::StarCrossTiling);
        r = r.register(girih::girih_shesh_band());
        r = r.register(girih_qc::GirihQuasiperiodicTiling);
        r = r.register(girih_decagon_patch::GirihDecagonPatchTiling);
        r = r.register(andritz_bowtie_hexagon::AndritzBowtieHexagonTiling);
        r = r.register(andritz_bowtie_hexagon_10gon_1::AndritzBowtieHexagon10gon1Tiling);
        r = r.register(andritz_bowtie_hexagon_10gon_2::AndritzBowtieHexagon10gon2Tiling);
        r = r.register(andritz_bowtie_hexagon_10gon_3::AndritzBowtieHexagon10gon3Tiling);
        r = r.register(girih_sc::girih_star_cross());
        r = r.register(girih_w85::girih_w85());
        r = r.register(pic_4_8_8::pic_4_8_8());
        r = r.register(pic_3_6_3_6::pic_3_6_3_6());
        r = r.register(pic_3_12_12::pic_3_12_12());
        r = r.register(hankin_4_8_8::hankin_4_8_8_rosette());
        r = r.register(hankin_4_6_12::hankin_4_6_12());
        r = r.register(hankin_hex::hankin_hex_two_point());
        r = r.register(hankin_3_12_12::hankin_3_12_12());
        r = r.register(hankin_square_two_point::hankin_square_two_point());
        r = r.register(hankin_4_8_8_rosette_transform::hankin_4_8_8_rosette_transform());
        r = r.register(hankin_3_6_3_6::hankin_3_6_3_6());
        r = r.register(hankin_3_4_6_4::hankin_3_4_6_4());
        r = r.register(hankin_3_3_3_4_4::hankin_3_3_3_4_4());
        r = r.register(hankin_3_3_4_3_4::hankin_3_3_4_3_4());
        r = r.register(hankin_4_8_8_motifs::hankin_4_8_8_motifs());
        r = r.register(hankin_hex_motifs::hankin_hex_motifs());
        r = r.register(khatam_girih::khatam_girih());
        r = r.register(najm::najm_63_2e_0_3e_ac());
        r = r.register(najm_63_motifs::najm_63_motifs());
        r = r.register(najm::najm_44_3e_0_0_a());
        r = r.register(girih_bowtie::girih_bowtie());
        r = r.register(girih_decagon::girih_decagon());
        r = r.register(girih_pentagon::girih_pentagon());
        r = r.register(girih_rhombus::girih_rhombus());
    r
}

/// The family's classification arms (see `crate::classify` for the
/// composer and the shared constructor helpers).
pub(crate) fn classification(id: &str) -> Option<TilingClass> {
    // Arms are verbatim from the pre-split table (they yield
    // `TilingClass`); an unknown id returns None so the composer can
    // fall through to later families.
    Some(match id {
        "star-cross" => TilingClass {
            families: &["Islamic"],
            method: Some("star-and-cross"),
            star_type: Some("{8/3}"),
            tile_set: Some("octagram stars + concave crosses"),
            ..TilingClass::EMPTY
        },
        "girih-shesh-band" => TilingClass {
            families: &["Islamic"],
            method: Some("girih strapwork"),
            tile_set: Some("shesh-band hexagon"),
            vertex_config: Some("6.6.6"),
            ..TilingClass::EMPTY
        },
        "girih-bowtie" => TilingClass {
            families: &["Islamic"],
            method: Some("girih prototile tiling"),
            tile_set: Some("girih bowtie (concave hexagon)"),
            ..TilingClass::EMPTY
        },
        "girih-decagon" => TilingClass {
            families: &["Islamic"],
            method: Some("girih prototile tiling"),
            tile_set: Some("girih decagon + bowtie infill"),
            vertex_config: Some("10.6.10"),
            ..TilingClass::EMPTY
        },
        "girih-pentagon" => TilingClass {
            families: &["Islamic"],
            method: Some("girih prototile tiling"),
            tile_set: Some("girih pentagon + hexagon + rhombus infill"),
            vertex_config: Some("5.5.4.4"),
            ..TilingClass::EMPTY
        },
        "girih-rhombus" => TilingClass {
            families: &["Islamic"],
            method: Some("girih prototile tiling"),
            tile_set: Some("girih rhombus"),
            ..TilingClass::EMPTY
        },
        "girih-star-cross" => TilingClass {
            families: &["Islamic"],
            method: Some("girih strapwork"),
            tile_set: Some("10-pointed star and cross"),
            star_type: Some("{10/3}"),
            ..TilingClass::EMPTY
        },
        "girih-w85" => TilingClass {
            families: &["Islamic"],
            method: Some("Wichmann panel data"),
            tile_set: Some("12-pointed star interlace"),
            ..TilingClass::EMPTY
        },
        "girih-quasiperiodic" => TilingClass {
            families: &["Islamic", "Quasiperiodic"],
            method: Some("Darb-i Imam substitution"),
            inflation: Some("2φ³"),
            point_order: Some(10),
            prototiles: Some(3),
            source: Some("Lu & Steinhardt"),
            year: Some("2007"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "girih-decagon-patch" => TilingClass {
            families: &["Islamic"],
            method: Some("Darb-i Imam substitution (finite patch)"),
            inflation: Some("2φ³"),
            point_order: Some(10),
            prototiles: Some(3),
            tile_set: Some("girih decagon + bow-tie closed-cluster infill"),
            source: Some("Lu & Steinhardt"),
            year: Some("2007"),
            translational: Some("finite patch"),
            ..TilingClass::EMPTY
        },
        "andritz-bowtie-hexagon" => TilingClass {
            families: &["Islamic", "Quasiperiodic"],
            method: Some("sigma-frame walk substitution"),
            inflation: Some("φ²"),
            point_order: Some(10),
            prototiles: Some(2),
            source: Some("Andritz"),
            year: Some("2016"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "andritz-bowtie-hexagon-10gon-1" => TilingClass {
            families: &["Islamic", "Quasiperiodic"],
            method: Some("sigma-frame walk substitution"),
            inflation: Some("φ²"),
            point_order: Some(10),
            prototiles: Some(3),
            source: Some("Andritz"),
            year: Some("2016"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "andritz-bowtie-hexagon-10gon-2" => TilingClass {
            families: &["Islamic", "Quasiperiodic"],
            method: Some("sigma-frame walk substitution"),
            inflation: Some("3 + 2φ"),
            point_order: Some(10),
            prototiles: Some(3),
            source: Some("Andritz"),
            year: Some("2016"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "andritz-bowtie-hexagon-10gon-3" => TilingClass {
            families: &["Islamic", "Quasiperiodic"],
            method: Some("sigma-frame walk substitution"),
            inflation: Some("φ⁴ (72° frame turn)"),
            point_order: Some(10),
            prototiles: Some(3),
            source: Some("Andritz"),
            year: Some("2016"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "pic-4-8-8" => hankin_pic("4.8.8", "θK 45°"),
        "pic-3-6-3-6" => hankin_pic("3.6.3.6", "θK 60°"),
        "pic-3-12-12" => hankin_pic("3.12.12", "θK 60°"),
        "hankin-4-8-8-rosette" => hankin_greedy("4.8.8", "θK 67.5°", &[]),
        "hankin-4-6-12" => hankin_greedy("4.6.12", "θK 60°", &[]),
        "hankin-hex-two-point" => TilingClass {
            families: &["Islamic"],
            method: Some("Kaplan two-point"),
            base_tiling: Some("6.6.6"),
            contact_angle: Some("θK 45°, δ = 1/2"),
            ..TilingClass::EMPTY
        },
        "hankin-square-two-point" => TilingClass {
            families: &["Islamic"],
            method: Some("Kaplan two-point"),
            base_tiling: Some("4.4.4.4"),
            contact_angle: Some("θK 45°, δ = 1/2"),
            ..TilingClass::EMPTY
        },
        "hankin-4-8-8-rosette-transform" => TilingClass {
            families: &["Islamic"],
            method: Some("rosette transform + Hankin inference"),
            base_tiling: Some("4.8.8"),
            contact_angle: Some("θK 45°"),
            ..TilingClass::EMPTY
        },
        "hankin-3-6-3-6" => hankin_greedy("3.6.3.6", "θK 60°", &["{6/2}"]),
        "hankin-3-4-6-4" => hankin_greedy("3.4.6.4", "θK 60°", &["{6/2}"]),
        "hankin-3-3-3-4-4" => hankin_greedy("3.3.3.4.4", "θK 60°", &[]),
        "hankin-3-3-4-3-4" => hankin_greedy("3.3.4.3.4", "θK 60°", &[]),
        "hankin-3-12-12" => hankin_greedy("3.12.12", "θK 45°", &[]),
        "hankin-4-8-8-motifs" => TilingClass {
            families: &["Islamic"],
            method: Some("Kaplan §3.5 motif library"),
            base_tiling: Some("4.8.8"),
            contact_angle: Some("θE 22.5° s 3; extended 67.5° s 2"),
            ..TilingClass::EMPTY
        },
        "hankin-hex-motifs" => TilingClass {
            families: &["Islamic"],
            method: Some("Kaplan §3.5 motif library"),
            base_tiling: Some("6.6.6"),
            contact_angle: Some("θE 30° s 2; extended 60° s 1"),
            ..TilingClass::EMPTY
        },
        "khatam-girih" => TilingClass {
            families: &["Islamic"],
            method: Some("girih decomposition"),
            star_type: Some("{8/3}"),
            tile_set: Some("kissing khatam star-and-cross"),
            ..TilingClass::EMPTY
        },
        "najm-63-2e-0-3e-ac" => TilingClass {
            families: &["Islamic"],
            method: Some("Najm template"),
            base_tiling: Some("[6,3]; 2e, 0, 3e (AC)"),
            tile_set: Some("dodecagon + enneagon substrate"),
            ..TilingClass::EMPTY
        },
        "najm-63-motifs" => TilingClass {
            families: &["Islamic"],
            method: Some("Kaplan §3.5 motif library"),
            base_tiling: Some("[6,3]; 2e, 0, 3e (AC)"),
            contact_angle: Some("rosette θE 15° s 3; star θE 35° s 2"),
            tile_set: Some("dodecagon + enneagon substrate"),
            ..TilingClass::EMPTY
        },
        "najm-44-3e-0-0-a" => TilingClass {
            families: &["Islamic"],
            method: Some("Najm template"),
            base_tiling: Some("[4,4]; 3e, 0, 0 (A)"),
            tile_set: Some("dodecagon + concave 4-point stars"),
            ..TilingClass::EMPTY
        },
        "hexagram" => TilingClass {
            // The name and description already carry "{6/2}"; the
            // schema adds nothing here and the card falls back to its
            // description fragment.
            families: &["Islamic"],
            ..TilingClass::EMPTY
        },
        _ => return None,
    })
}

/// This family's entries for the global lattice-culling cross-check
/// (`lattice_culling_tests` in `lib.rs`), one per islamic-family tiling
/// that carries a `LatticeTiling` table. Dedicated tiling types with no
/// table (e.g. hexagram, zellige star-cross, girih-quasiperiodic,
/// girih-decagon-patch, the andritz bowtie-hexagon set) are out
/// of scope by construction.
#[cfg(test)]
pub(crate) fn lattice_tilings() -> Vec<(&'static str, crate::lattice::LatticeTiling)> {
    vec![
            ("girih", girih::girih_shesh_band()),
            ("girih-bowtie", girih_bowtie::girih_bowtie()),
            ("girih-decagon", girih_decagon::girih_decagon()),
            ("girih-pentagon", girih_pentagon::girih_pentagon()),
            ("girih-rhombus", girih_rhombus::girih_rhombus()),
            ("girih-sc", girih_sc::girih_star_cross()),
            ("girih-w85", girih_w85::girih_w85()),
            ("pic-4-8-8", pic_4_8_8::pic_4_8_8()),
            ("pic-3-6-3-6", pic_3_6_3_6::pic_3_6_3_6()),
            ("pic-3-12-12", pic_3_12_12::pic_3_12_12()),
            ("khatam-girih", khatam_girih::khatam_girih()),
            ("hankin-4-8-8-rosette", hankin_4_8_8::hankin_4_8_8_rosette()),
            ("hankin-4-6-12", hankin_4_6_12::hankin_4_6_12()),
            ("hankin-hex-two-point", hankin_hex::hankin_hex_two_point()),
            ("hankin-3-12-12", hankin_3_12_12::hankin_3_12_12()),
            ("hankin-square-two-point", hankin_square_two_point::hankin_square_two_point()),
            ("hankin-4-8-8-rosette-transform", hankin_4_8_8_rosette_transform::hankin_4_8_8_rosette_transform()),
            ("hankin-3-6-3-6", hankin_3_6_3_6::hankin_3_6_3_6()),
            ("hankin-3-4-6-4", hankin_3_4_6_4::hankin_3_4_6_4()),
            ("hankin-3-3-3-4-4", hankin_3_3_3_4_4::hankin_3_3_3_4_4()),
            ("hankin-3-3-4-3-4", hankin_3_3_4_3_4::hankin_3_3_4_3_4()),
            ("hankin-4-8-8-motifs", hankin_4_8_8_motifs::hankin_4_8_8_motifs()),
            ("hankin-hex-motifs", hankin_hex_motifs::hankin_hex_motifs()),
            ("najm-63-motifs", najm_63_motifs::najm_63_motifs()),
            ("najm-63-2e-0-3e-ac", najm::najm_63_2e_0_3e_ac()),
            ("najm-44-3e-0-0-a", najm::najm_44_3e_0_0_a()),
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
        "star-cross",
        "girih-shesh-band",
        "girih-star-cross",
        "girih-w85",
        "girih-quasiperiodic",
        "girih-decagon-patch",
        "andritz-bowtie-hexagon",
        "andritz-bowtie-hexagon-10gon-1",
        "andritz-bowtie-hexagon-10gon-2",
        "andritz-bowtie-hexagon-10gon-3",
        "pic-4-8-8",
        "pic-3-6-3-6",
        "pic-3-12-12",
        "hankin-4-8-8-rosette",
        "hankin-4-6-12",
        "hankin-hex-two-point",
        "hankin-square-two-point",
        "hankin-4-8-8-rosette-transform",
        "hankin-3-6-3-6",
        "hankin-3-4-6-4",
        "hankin-3-3-3-4-4",
        "hankin-3-3-4-3-4",
        "hankin-3-12-12",
        "hankin-4-8-8-motifs",
        "hankin-hex-motifs",
        "khatam-girih",
        "najm-63-2e-0-3e-ac",
        "najm-63-motifs",
        "najm-44-3e-0-0-a",
        "hexagram",
        "girih-bowtie",
        "girih-decagon",
        "girih-pentagon",
        "girih-rhombus",
    ];

    #[test]
    fn family_census_matches() {
        let r = register(Registry::new());
        assert_eq!(
            r.len(),
            34,
            "islamic family census — update when adding a tiling to this family"
        );
        let have: BTreeSet<&str> = r.iter().map(|t| t.id()).collect();
        let want: BTreeSet<&str> = IDS.iter().copied().collect();
        assert_eq!(
            have, want,
            "islamic family membership drift: register() vs IDS"
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
mod zellige_tests {
    use crate::tests::{check_coverage, check_covers, check_det, check_rotated};
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
mod hexagram_tests {
    use crate::tests::{check_coverage, check_covers, check_det, check_rotated};
    use crate::hexagram;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::{Query, Tiling};

    /// √3 (a local const: `std::f64::consts::SQRT_3` is unstable in
    /// this toolchain — the `more_float_constants` gate).
    const SQRT_3: f64 = 1.7320508075688773;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn hexagram_covers_region() {
        check_covers(
            &hexagram::HexagramTiling,
            "hexagram",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn hexagram_rotated_scaled() {
        check_rotated(
            &hexagram::HexagramTiling,
            "hexagram(rot22.5,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            22.5,
        );
    }

    #[test]
    fn hexagram_deterministic() {
        check_det(
            &hexagram::HexagramTiling,
            "hexagram",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// The star tip radius parameter must produce a valid, gap-free
    /// tiling at the kissing default, at the critical T = 1/3 (tips
    /// land on the Voronoi-vertex ring), at gapped radii, and at the
    /// wide-gap floor.
    #[test]
    fn hexagram_gap_family() {
        let t = hexagram::HexagramTiling;
        for radius in [0.5_f64, 1.0 / 3.0, 0.45, 0.3, 0.15] {
            let mut p = ParamSet::default_for(t.params());
            p.set(hexagram::STAR_RADIUS_ID, ParamValue::Float(radius));
            check_coverage(
                &t,
                &format!("hexagram(r={radius})"),
                &p,
                BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
            );
        }
    }

    /// p6m generator probe on the generated output: for each generator
    /// (60-degree about a star center, 120-degree about the two
    /// hexagon centers C1, C2, mirror y -> -y, mirror in the a2 line),
    /// the exact image of every interior emitted star must equal an
    /// emitted star as a vertex set. "Interior" means centroid in
    /// [-8, 8]^2 — the image of every interior lattice point stays
    /// well inside the 16x16 view, so its image star is always
    /// emitted, even though it may itself sit outside the interior
    /// box (e.g. rot60 of P(3,9) = (-3, 10.4)); the lookup therefore
    /// runs over ALL emitted stars. The per-cell emission is not
    /// generator-invariant as a tile set (a generator maps the cell
    /// basis {a1,a2} to a skewed GL(2,Z) basis), but the interstice
    /// is the complement of the star layer, so preserving the star
    /// layer setwise preserves the whole tiling.
    #[test]
    fn hexagram_group_symmetry() {
        fn rot60(v: Vec2) -> Vec2 {
            // 60-degree rotation: [[1/2, -sqrt3/2], [sqrt3/2, 1/2]]
            v2(0.5 * v.x - 0.5 * SQRT_3 * v.y, 0.5 * SQRT_3 * v.x + 0.5 * v.y)
        }
        fn rot120(v: Vec2) -> Vec2 {
            // 120-degree rotation: [[-1/2, -sqrt3/2], [sqrt3/2, -1/2]]
            v2(-0.5 * v.x - 0.5 * SQRT_3 * v.y, 0.5 * SQRT_3 * v.x - 0.5 * v.y)
        }

        for radius in [0.5_f64, 0.3] {
            let tiling = hexagram::HexagramTiling;
            let mut p = ParamSet::default_for(tiling.params());
            p.set(hexagram::STAR_RADIUS_ID, ParamValue::Float(radius));
            let region = BBox::new(v2(-16.0, -16.0), v2(16.0, 16.0));
            let r = tiling.generate(&p, &Query { region });
            assert!(
                r.message.is_none(),
                "hexagram(r={radius}): {msg}",
                msg = r.message.unwrap_or("")
            );

            let centroid = |pts: &[Vec2]| {
                let c = pts.iter().fold(Vec2::ZERO, |acc, v| acc + *v);
                c / pts.len() as f64
            };
            // All emitted stars (proto 1), with centroids; the probe
            // runs over the interior subset (centroid in [±8]²) but
            // matches images against every emitted star.
            let stars: Vec<Vec<Vec2>> = r
                .tiles
                .iter()
                .filter(|tl| tl.proto == 1)
                .map(|tl| tl.poly.pts.clone())
                .collect();
            let centroids: Vec<Vec2> = stars.iter().map(|pts| centroid(pts)).collect();
            let interior: Vec<usize> = centroids
                .iter()
                .enumerate()
                .filter(|(_, c)| c.x.abs() < 8.0 && c.y.abs() < 8.0)
                .map(|(i, _)| i)
                .collect();
            assert!(!interior.is_empty(), "hexagram(r={radius}): no interior stars");

            let gens: [Box<dyn Fn(Vec2) -> Vec2>; 5] = [
                Box::new(rot60),
                Box::new(move |v: Vec2| {
                    let c1 = v2(0.5, SQRT_3 / 6.0);
                    c1 + rot120(v - c1)
                }),
                Box::new(move |v: Vec2| {
                    let c2 = v2(1.0, SQRT_3 / 3.0);
                    c2 + rot120(v - c2)
                }),
                Box::new(|v: Vec2| v2(v.x, -v.y)),
                Box::new(|v: Vec2| {
                    // reflection in the a2 line (60-degree axis)
                    v2(
                        -0.5 * v.x + 0.5 * SQRT_3 * v.y,
                        0.5 * SQRT_3 * v.x + 0.5 * v.y,
                    )
                }),
            ];
            for (gi, g) in gens.iter().enumerate() {
                for &si in &interior {
                    let s = &stars[si];
                    let img: Vec<Vec2> = s.iter().map(|v| g(*v)).collect();
                    let ic = centroid(&img);
                    let mut found = false;
                    for (oi, o) in stars.iter().enumerate() {
                        if (centroids[oi] - ic).length() > 1e-6 {
                            continue;
                        }
                        let mut ok = o.len() == img.len();
                        for v in &img {
                            if !o.iter().any(|w| (*v - *w).length() < 1e-9) {
                                ok = false;
                                break;
                            }
                        }
                        assert!(
                            ok,
                            "hexagram(r={radius}): generator {gi} does not map star to an emitted star"
                        );
                        found = true;
                        break;
                    }
                    assert!(
                        found,
                        "hexagram(r={radius}): generator {gi} image star not in emitted set"
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod girih_tests {
    use crate::tests::{check_covers, check_det, check_rotated};
    use crate::girih;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::ParamSet;
    use tess_core::tiling::{Query, Tiling};

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

    /// M112: single-tile-per-cell, single orbit — the shipped seed is
    /// the decorative `SeedPattern::Checker` lattice parity, so a
    /// bounded view must show both parities (seeds stay in {0, 1}).
    #[test]
    fn girih_shesh_band_seed_is_decorative_checkerboard() {
        let t = girih::girih_shesh_band();
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region: BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let mut seen = [false; 2];
        for tile in &r.tiles {
            assert!(tile.seed < 2, "checkerboard seed must stay in {{0, 1}}");
            seen[tile.seed as usize] = true;
        }
        assert!(seen[0] && seen[1], "both lattice-parity classes must appear");
    }
}

#[cfg(test)]
mod girih_sc_tests {
    use crate::tests::{check_covers, check_det, check_rotated};
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
mod khatam_girih_tests {
    use crate::tests::{check_covers, check_det, check_rotated};
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
mod girih_qc_tests {
    use crate::tests::{check_coverage, check_deterministic};
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

#[cfg(test)]
mod andritz_bowtie_hexagon_tests {
    use crate::families::islamic::andritz_bowtie_hexagon::AndritzBowtieHexagonTiling;
    use crate::tests::{check_coverage, check_deterministic};
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Unit scale (the walk geometry is defined at unit edges).
    fn params1() -> ParamSet {
        let t = AndritzBowtieHexagonTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        p
    }

    /// Standard coverage near the origin (the walk fixed point sits at
    /// ≈ (19.56, −5.83); `check_coverage` verifies culling, point
    /// sampling and exact clipped-area equality — no gaps, no overlaps).
    #[test]
    fn andritz_bowtie_hexagon_covers_region() {
        let t = AndritzBowtieHexagonTiling;
        check_coverage(
            &t,
            "andritz-bowtie-hexagon",
            &params1(),
            BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
        );
    }

    /// Coverage generalizes across anchor regions: off-origin views
    /// served at deeper BFS levels are covered exactly too (the sweep
    /// is region-filtered through the σ⁻¹ preimage box chain).
    #[test]
    fn andritz_bowtie_hexagon_covers_distant_views() {
        let t = AndritzBowtieHexagonTiling;
        let p = params1();
        for (cx, cy, w, h) in [(60.0, -30.0, 8.0, 8.0), (150.0, 80.0, 6.0, 6.0)] {
            check_coverage(
                &t,
                "andritz-bowtie-hexagon",
                &p,
                BBox::new(v2(cx - w, cy - h), v2(cx + w, cy + h)),
            );
        }
    }

    /// Coverage must survive a non-axis-aligned scale + rotation.
    #[test]
    fn andritz_bowtie_hexagon_rotated_scaled() {
        let t = AndritzBowtieHexagonTiling;
        let mut p = params1();
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(0.7));
        p.set(crate::lattice::ROTATION_ID, ParamValue::Angle(30.0));
        check_coverage(
            &t,
            "andritz-bowtie-hexagon(rot30,scale0.7)",
            &p,
            BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
        );
    }

    #[test]
    fn andritz_bowtie_hexagon_deterministic() {
        let t = AndritzBowtieHexagonTiling;
        check_deterministic(
            &t,
            "andritz-bowtie-hexagon",
            &params1(),
            BBox::new(v2(-10.0, -10.0), v2(10.0, 10.0)),
        );
    }
}

#[cfg(test)]
mod andritz_bowtie_hexagon_10gon_1_tests {
    use crate::families::islamic::andritz_bowtie_hexagon_10gon_1::AndritzBowtieHexagon10gon1Tiling;
    use crate::tests::{check_coverage, check_deterministic};
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Unit scale (the walk geometry is defined at unit edges).
    fn params1() -> ParamSet {
        let t = AndritzBowtieHexagon10gon1Tiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        p
    }

    /// Standard coverage near the walk's fixed point (o ≈ (16.14,
    /// −7.47)); `check_coverage` verifies culling, point sampling and
    /// exact clipped-area equality — no gaps, no overlaps.
    #[test]
    fn andritz_bowtie_hexagon_10gon_1_covers_region() {
        let t = AndritzBowtieHexagon10gon1Tiling;
        check_coverage(
            &t,
            "andritz-bowtie-hexagon-10gon-1",
            &params1(),
            BBox::new(v2(8.0, -14.0), v2(24.0, -2.0)),
        );
    }

    /// Coverage generalizes across anchor regions: off-origin views
    /// served at deeper BFS levels are covered exactly too (the sweep
    /// is region-filtered through the σ⁻¹ preimage box chain).
    #[test]
    fn andritz_bowtie_hexagon_10gon_1_covers_distant_views() {
        let t = AndritzBowtieHexagon10gon1Tiling;
        let p = params1();
        for (cx, cy, w, h) in [(70.0, -40.0, 8.0, 8.0), (150.0, 90.0, 6.0, 6.0)] {
            check_coverage(
                &t,
                "andritz-bowtie-hexagon-10gon-1",
                &p,
                BBox::new(v2(cx - w, cy - h), v2(cx + w, cy + h)),
            );
        }
    }

    /// Coverage must survive a non-axis-aligned scale + rotation.
    #[test]
    fn andritz_bowtie_hexagon_10gon_1_rotated_scaled() {
        let t = AndritzBowtieHexagon10gon1Tiling;
        let mut p = params1();
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(0.7));
        p.set(crate::lattice::ROTATION_ID, ParamValue::Angle(30.0));
        check_coverage(
            &t,
            "andritz-bowtie-hexagon-10gon-1(rot30,scale0.7)",
            &p,
            BBox::new(v2(8.0, -14.0), v2(24.0, -2.0)),
        );
    }

    #[test]
    fn andritz_bowtie_hexagon_10gon_1_deterministic() {
        let t = AndritzBowtieHexagon10gon1Tiling;
        check_deterministic(
            &t,
            "andritz-bowtie-hexagon-10gon-1",
            &params1(),
            BBox::new(v2(6.0, -18.0), v2(26.0, 2.0)),
        );
    }
}

#[cfg(test)]
mod andritz_bowtie_hexagon_10gon_2_tests {
    use crate::families::islamic::andritz_bowtie_hexagon_10gon_2::AndritzBowtieHexagon10gon2Tiling;
    use crate::tests::{check_coverage, check_deterministic};
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Unit scale (the walk geometry is defined at unit edges).
    fn params1() -> ParamSet {
        let t = AndritzBowtieHexagon10gon2Tiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        p
    }

    /// Standard coverage near the walk's fixed point (o ≈ (1.31,
    /// 6.36)); `check_coverage` verifies culling, point sampling and
    /// exact clipped-area equality — no gaps, no overlaps.
    #[test]
    fn andritz_bowtie_hexagon_10gon_2_covers_region() {
        let t = AndritzBowtieHexagon10gon2Tiling;
        check_coverage(
            &t,
            "andritz-bowtie-hexagon-10gon-2",
            &params1(),
            BBox::new(v2(-6.0, -1.0), v2(7.0, 12.0)),
        );
    }

    /// Coverage generalizes across anchor regions: off-origin views
    /// served at deeper BFS levels are covered exactly too (the sweep
    /// is region-filtered through the σ⁻¹ preimage box chain).
    #[test]
    fn andritz_bowtie_hexagon_10gon_2_covers_distant_views() {
        let t = AndritzBowtieHexagon10gon2Tiling;
        let p = params1();
        for (cx, cy, w, h) in [(70.0, -40.0, 8.0, 8.0), (150.0, 90.0, 6.0, 6.0)] {
            check_coverage(
                &t,
                "andritz-bowtie-hexagon-10gon-2",
                &p,
                BBox::new(v2(cx - w, cy - h), v2(cx + w, cy + h)),
            );
        }
    }

    /// Coverage must survive a non-axis-aligned scale + rotation.
    #[test]
    fn andritz_bowtie_hexagon_10gon_2_rotated_scaled() {
        let t = AndritzBowtieHexagon10gon2Tiling;
        let mut p = params1();
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(0.7));
        p.set(crate::lattice::ROTATION_ID, ParamValue::Angle(30.0));
        check_coverage(
            &t,
            "andritz-bowtie-hexagon-10gon-2(rot30,scale0.7)",
            &p,
            BBox::new(v2(-6.0, -1.0), v2(7.0, 12.0)),
        );
    }

    #[test]
    fn andritz_bowtie_hexagon_10gon_2_deterministic() {
        let t = AndritzBowtieHexagon10gon2Tiling;
        check_deterministic(
            &t,
            "andritz-bowtie-hexagon-10gon-2",
            &params1(),
            BBox::new(v2(-9.0, -12.0), v2(12.0, 22.0)),
        );
    }
}

#[cfg(test)]
mod andritz_bowtie_hexagon_10gon_3_tests {
    use crate::families::islamic::andritz_bowtie_hexagon_10gon_3::AndritzBowtieHexagon10gon3Tiling;
    use crate::tests::{check_coverage, check_deterministic};
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Unit scale (the walk geometry is defined at unit edges).
    fn params1() -> ParamSet {
        let t = AndritzBowtieHexagon10gon3Tiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        p
    }

    /// Standard coverage near the walk's fixed point (o ≈ (13.71,
    /// −6.88), served at BFS level 1); `check_coverage` verifies
    /// culling, point sampling and exact clipped-area equality — no
    /// gaps, no overlaps.
    #[test]
    fn andritz_bowtie_hexagon_10gon_3_covers_region() {
        let t = AndritzBowtieHexagon10gon3Tiling;
        check_coverage(
            &t,
            "andritz-bowtie-hexagon-10gon-3",
            &params1(),
            BBox::new(v2(6.0, -14.0), v2(21.0, 1.0)),
        );
    }

    /// Coverage generalizes across anchor regions: off-origin views
    /// served at deeper BFS levels are covered exactly too (the sweep
    /// is region-filtered through the σ⁻¹ preimage box chain).
    #[test]
    fn andritz_bowtie_hexagon_10gon_3_covers_distant_views() {
        let t = AndritzBowtieHexagon10gon3Tiling;
        let p = params1();
        for (cx, cy, w, h) in [(80.0, 40.0, 8.0, 8.0), (150.0, 90.0, 6.0, 6.0)] {
            check_coverage(
                &t,
                "andritz-bowtie-hexagon-10gon-3",
                &p,
                BBox::new(v2(cx - w, cy - h), v2(cx + w, cy + h)),
            );
        }
    }

    /// Coverage must survive a non-axis-aligned scale + rotation.
    #[test]
    fn andritz_bowtie_hexagon_10gon_3_rotated_scaled() {
        let t = AndritzBowtieHexagon10gon3Tiling;
        let mut p = params1();
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(0.7));
        p.set(crate::lattice::ROTATION_ID, ParamValue::Angle(30.0));
        check_coverage(
            &t,
            "andritz-bowtie-hexagon-10gon-3(rot30,scale0.7)",
            &p,
            BBox::new(v2(6.0, -14.0), v2(21.0, 1.0)),
        );
    }

    #[test]
    fn andritz_bowtie_hexagon_10gon_3_deterministic() {
        let t = AndritzBowtieHexagon10gon3Tiling;
        check_deterministic(
            &t,
            "andritz-bowtie-hexagon-10gon-3",
            &params1(),
            BBox::new(v2(3.0, -17.0), v2(24.0, 4.0)),
        );
    }
}

#[cfg(test)]
mod girih_w85_tests {
    use super::girih_w85::{girih_w85, s3, L, STRAP_W};
    use crate::lattice::LatticeTiling;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn girih_w85_covers_region() {
        check_covers(
            &girih_w85(),
            "girih-w85",
            BBox::new(v2(-120.0, -90.0), v2(560.0, 420.0)),
        );
    }

    #[test]
    fn girih_w85_rotated_scaled() {
        check_rotated(
            &girih_w85(),
            "girih-w85(rot30,scale0.7)",
            BBox::new(v2(-150.0, -120.0), v2(520.0, 400.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn girih_w85_deterministic() {
        check_det(
            &girih_w85(),
            "girih-w85",
            BBox::new(v2(-100.0, -100.0), v2(600.0, 400.0)),
        );
    }

    /// Vertex-figure census of the shipped cell: 42 tiles on the hexagonal
    /// lattice with Σ areas = |det| exactly (Q(√3) identity verified in the
    /// spike), the exact star identities, and every interior angle a
    /// multiple of 30°.
    #[test]
    fn girih_w85_unit_structure() {
        let t = girih_w85();
        assert_eq!(t.cells.len(), 42, "42 tiles per primitive cell");
        let mut counts = [0usize; 7];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(
            counts, [1, 12, 6, 2, 3, 6, 12],
            "seed census: star 1, kite 12, pentagon 6, triangle 2, rectangle 3, quad 6, ink hexagon 12"
        );
        // Hexagonal lattice: |a2| = L, 60° between, det = L²√3/2.
        assert_eq!(t.lattice[0], Vec2::new(L, 0.0), "a1 = (L, 0)");
        let det = t.lattice[0].cross(t.lattice[1]);
        assert!((det - L * L * s3() / 2.0).abs() < 1e-9, "det = L²√3/2");
        assert!((t.lattice[1].length() - L).abs() < 1e-9, "|a2| = L");
        // Exact area identity: Σ cells == |det| (verified at 0 error in
        // Fraction-Q(√3) arithmetic in the Python spike).
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!(
            (sum - det).abs() < 1e-7,
            "cell areas {sum} must sum to |det| = {det}"
        );
        // Star identities at the origin-centred 24-gon.
        let d = L / 4.0 - STRAP_W / 2.0; // 66.48
        let star = &t.cells[0];
        assert_eq!(star.seed, 0);
        let n = star.poly.len();
        assert_eq!(n, 24, "the 12-pointed star is a 24-gon");
        let edge = d * (1.0 - 1.0 / s3());
        for i in 0..n {
            let p = star.poly.pts[i];
            let q = star.poly.pts[(i + 1) % n];
            let r = (q - p).length();
            assert!((r - edge).abs() < 1e-9, "star edge {r} ≠ d(1−1/√3) = {edge}");
            let want = if i % 2 == 0 { d * 2.0f64.sqrt() } else { 2.0 * d / s3() };
            assert!(
                (p.length() - want).abs() < 1e-9,
                "vertex {i} radius {} ≠ {want}",
                p.length()
            );
        }
        // Every interior angle of every cell is a multiple of 30°.
        for c in &t.cells {
            let k = c.poly.len();
            for i in 0..k {
                let p0 = c.poly.pts[(i + k - 1) % k];
                let p1 = c.poly.pts[i];
                let p2 = c.poly.pts[(i + 1) % k];
                let (e1, e2) = (p1 - p0, p2 - p1);
                let turn = e1.cross(e2).atan2(e1.dot(e2)).to_degrees();
                let interior = 180.0 - turn; // CCW cells: reflex joints give interior > 180
                let u = (interior / 30.0).round();
                assert!(
                    u > 0.0 && u < 12.0 && (interior - 30.0 * u).abs() < 1e-6,
                    "cell angle {interior}° is not a multiple of 30°"
                );
            }
        }
    }

    /// A candidate symmetry acting on world points.
    type Isometry = Box<dyn Fn(Vec2) -> Vec2>;

    /// Does the transformed image of one table cell equal a table cell
    /// translated by an exact integer lattice combination? (Float version of
    /// the exact-Q(√3) membership test from the spike.)
    fn image_is_a_tile(t: &LatticeTiling, pts: &[Vec2], f: impl Fn(Vec2) -> Vec2) -> bool {
        let det = t.lattice[0].cross(t.lattice[1]);
        for cand in &t.cells {
            let n = cand.poly.len();
            if n != pts.len() {
                continue;
            }
            for sh in 0..n {
                let d = cand.poly.pts[sh] - f(pts[0]);
                let i = d.cross(t.lattice[1]) / det;
                let j = t.lattice[0].cross(d) / det;
                if (i - i.round()).abs() > 1e-6 || (j - j.round()).abs() > 1e-6 {
                    continue;
                }
                let off = t.lattice[0] * i.round() + t.lattice[1] * j.round();
                if (0..n).all(|k| (f(pts[k]) + off - cand.poly.pts[(sh + k) % n]).length() < 1e-7)
                {
                    return true;
                }
            }
        }
        false
    }

    /// Wallpaper group p6, positively and negatively pinned on the exact
    /// table: 6-fold rotations at the star centres (lattice points), 2-fold
    /// rotations at the midpoints of all three lattice-edge classes, 3-fold
    /// rotations at (a1+a2)/3 and 2(a1+a2)/3 — and *no* 30° rotations and
    /// *no* mirrors (the over/under interlace is chiral; Wichmann's *632 is
    /// the motif without the ink partition).
    #[test]
    fn girih_w85_symmetry_is_p6() {
        let t = girih_w85();
        let s = s3() / 2.0;
        let rot = |deg: f64, c: Vec2| {
            let (cc, ss) = match deg {
                30.0 => (s3() / 2.0, 0.5),
                60.0 => (0.5, s),
                120.0 => (-0.5, s),
                180.0 => (-1.0, 0.0),
                _ => unreachable!(),
            };
            move |q: Vec2| {
                let d = q - c;
                v2(c.x + cc * d.x - ss * d.y, c.y + ss * d.x + cc * d.y)
            }
        };
        let mirror = |deg: f64| {
            // reflection in the line through the origin at angle `deg`:
            // matrix [[cos 2a, sin 2a], [sin 2a, −cos 2a]]
            let (cc, ss) = match deg {
                0.0 => (1.0, 0.0),
                30.0 => (0.5, s),
                60.0 => (-0.5, s),
                90.0 => (-1.0, 0.0),
                _ => unreachable!(),
            };
            move |q: Vec2| v2(cc * q.x + ss * q.y, ss * q.x - cc * q.y)
        };
        let a1 = t.lattice[0];
        let a2 = t.lattice[1];
        // Positive generators.
        for (deg, c, label) in [
            (60.0, v2(0.0, 0.0), "rot60 @ star centre (0,0)"),
            (60.0, a1, "rot60 @ star centre a1"),
            (120.0, (a1 + a2) * (1.0 / 3.0), "rot120 @ (a1+a2)/3"),
            (120.0, (a1 + a2) * (2.0 / 3.0), "rot120 @ 2(a1+a2)/3"),
            (180.0, a1 * 0.5, "rot180 @ a1/2"),
            (180.0, a2 * 0.5, "rot180 @ a2/2"),
            (180.0, (a1 + a2) * 0.5, "rot180 @ (a1+a2)/2"),
        ] {
            let f = rot(deg, c);
            for cell in &t.cells {
                assert!(
                    image_is_a_tile(&t, &cell.poly.pts, f),
                    "{label} must preserve the tiling (cell seed {} fails)",
                    cell.seed
                );
            }
        }
        // Negative candidates: 30° rotations and every mirror class fail.
        // (If any mirror existed anywhere, composing with the 6-fold
        // rotations would put one through a star centre in one of the
        // tested axis classes — p6m is the only wallpaper group properly
        // containing p6.)
        let mid = a1 * 0.5;
        let m30mid = {
            let m = mirror(30.0);
            move |q: Vec2| m(q - mid) + mid
        };
        let negatives: Vec<(&str, Isometry)> = vec![
            ("rot30 @ star centre", Box::new(rot(30.0, v2(0.0, 0.0)))),
            ("rot30 @ a1/2", Box::new(rot(30.0, mid))),
            ("mirror 0° @ star", Box::new(mirror(0.0))),
            ("mirror 30° @ star", Box::new(mirror(30.0))),
            ("mirror 60° @ star", Box::new(mirror(60.0))),
            ("mirror 90° @ star", Box::new(mirror(90.0))),
            ("mirror 30° @ a1/2", Box::new(m30mid)),
        ];
        for (label, f) in &negatives {
            let fails = t
                .cells
                .iter()
                .filter(|cell| !image_is_a_tile(&t, &cell.poly.pts, f))
                .count();
            assert!(
                fails > 0,
                "{label} must NOT be a symmetry (a chiral interlace has none)"
            );
        }
    }

    #[test]
    fn girih_w85_symmetry_metadata() {
        let t = girih_w85();
        assert_eq!(t.id, "girih-w85");
        assert_eq!(t.group, WallpaperGroup::P6);
        assert_eq!(t.lattice_kind, LatticeKind::Hexagonal);
        assert_eq!(t.category, TilingCategory::Islamic);
    }
}

#[cfg(test)]
mod pic_4_8_8_tests {
    use crate::lattice::LatticeTiling;
    use super::pic_4_8_8::pic_4_8_8;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    const RT2: f64 = core::f64::consts::SQRT_2;
    const A: f64 = 1.0 + RT2;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    fn sub(a: Vec2, b: Vec2) -> Vec2 {
        v2(a.x - b.x, a.y - b.y)
    }

    fn norm(a: Vec2) -> f64 {
        (a.x * a.x + a.y * a.y).sqrt()
    }

    type Isometry = Box<dyn Fn(Vec2) -> Vec2>;

    /// Is `k` an exact member of the (1/4)·Z[√2] surd grid? Exhaustive
    /// over the table's bounded range; distinct q representatives differ
    /// by √2 ≫ 1e-9, so the decision is exact here.
    fn on_quarter_surd_grid(k: f64) -> bool {
        let k4 = 4.0 * k;
        (-60..=60).any(|q| {
            let p = k4 - q as f64 * RT2;
            (p - p.round()).abs() < 1e-9
        })
    }

    /// Positive symmetry test: the image of the face `pts` under `f` must
    /// coincide, vertex for vertex, with some orbit face plus an EXACT
    /// lattice translation (integer multiples of the basis vectors). With
    /// all coordinates on the (1/4)·Z[√2] grid the nearest non-lattice
    /// offset is ≥ √2/4 ≈ 0.35, so the 1e-9 tolerance cannot blur a
    /// genuine near-miss into a hit.
    fn image_is_a_face(t: &LatticeTiling, pts: &[Vec2], f: &dyn Fn(Vec2) -> Vec2) -> bool {
        let img: Vec<Vec2> = pts.iter().map(|p| f(*p)).collect();
        for cell in &t.cells {
            let v = &cell.poly.pts;
            if v.len() != img.len() {
                continue;
            }
            for i in 0..v.len() {
                let d = sub(img[0], v[i]);
                let (mx, my) = (d.x / A, d.y / A);
                if (mx - mx.round()).abs() > 1e-9 || (my - my.round()).abs() > 1e-9 {
                    continue;
                }
                let shift = v2(mx.round() * A, my.round() * A);
                // Vertex-SET match (orientation-free: mirrors reverse the
                // winding of the CCW-normalised table).
                let target: Vec<Vec2> = v.iter().map(|p| *p + shift).collect();
                let mut used = vec![false; target.len()];
                let mut ok = true;
                for p in &img {
                    let hit = target.iter().position(|q| norm(sub(*p, *q)) < 1e-9);
                    match hit {
                        Some(j) if !used[j] => used[j] = true,
                        _ => {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    return true;
                }
            }
        }
        false
    }

    fn all_faces(t: &LatticeTiling) -> Vec<Vec<Vec2>> {
        t.cells.iter().map(|c| c.poly.pts.clone()).collect()
    }

    #[test]
    fn pic_4_8_8_covers_region() {
        check_covers(
            &pic_4_8_8(),
            "pic-4-8-8",
            BBox::new(v2(-24.0, -18.0), v2(24.0, 18.0)),
        );
    }

    #[test]
    fn pic_4_8_8_rotated_scaled() {
        check_rotated(
            &pic_4_8_8(),
            "pic-4-8-8(rot20,scale0.6)",
            BBox::new(v2(-24.0, -18.0), v2(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn pic_4_8_8_deterministic() {
        check_det(
            &pic_4_8_8(),
            "pic-4-8-8",
            BBox::new(v2(-12.0, -9.0), v2(12.0, 9.0)),
        );
    }

    #[test]
    fn pic_4_8_8_unit_structure() {
        let t = pic_4_8_8();
        assert_eq!(t.cells.len(), 14);
        let by_seed = |s: u32| t.cells.iter().filter(|c| c.seed == s).count();
        assert_eq!(
            (by_seed(0), by_seed(1), by_seed(2), by_seed(3)),
            (1, 1, 4, 8),
            "orbit census: octagon, small square, 4 houses, 8 star tips"
        );

        // Exact Q(√2) construction: every coordinate is a (1/4)·Z[√2] surd.
        for c in &t.cells {
            for p in &c.poly.pts {
                assert!(on_quarter_surd_grid(p.x), "vertex x {} off the grid", p.x);
                assert!(on_quarter_surd_grid(p.y), "vertex y {} off the grid", p.y);
            }
        }

        // Σ(cell areas) = |det(lattice)| — exact over Q(√2) (spike):
        // (1+√2) + 1/2 + 4(1/8+√2/4) + 8(1/8) = 3+2√2 = (1+√2)².
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        assert!((det - A * A).abs() < 1e-12);
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-12, "Σ areas {sum} vs det {det}");

        // Octagon (seed 0): regular, side √2/2, centred on the octagon
        // centre (A/2, A/2).
        let oct = t.cells.iter().find(|c| c.seed == 0).unwrap();
        let v = &oct.poly.pts;
        assert_eq!(v.len(), 8);
        for i in 0..v.len() {
            let side = norm(sub(v[(i + 1) % 8], v[i]));
            assert!((side - RT2 / 2.0).abs() < 1e-12, "octagon side {side}");
        }
        let cen = v.iter().fold(Vec2::ZERO, |a, p| a + *p);
        assert!(norm(sub(cen / 8.0, v2(A / 2.0, A / 2.0))) < 1e-12);
        assert!((oct.poly.area() - A).abs() < 1e-12, "octagon area = 1+√2");

        // Small square (seed 1): side √2/2 (the [±√2/4]² khatam centre),
        // centred on the origin.
        let sq = t.cells.iter().find(|c| c.seed == 1).unwrap();
        let v = &sq.poly.pts;
        assert_eq!(v.len(), 4);
        for i in 0..4 {
            let side = norm(sub(v[(i + 1) % 4], v[i]));
            assert!((side - RT2 / 2.0).abs() < 1e-12, "square side {side}");
        }
        assert!((sq.poly.area() - 0.5).abs() < 1e-12, "square area = 1/2");

        // House pentagons (seed 2): area 1/8 + √2/4 each.
        let mut houses = 0;
        for c in t.cells.iter().filter(|c| c.seed == 2) {
            assert_eq!(c.poly.pts.len(), 5);
            assert!((c.poly.area() - (0.125 + RT2 / 4.0)).abs() < 1e-12);
            houses += 1;
        }
        assert_eq!(houses, 4);

        // Star tips (seed 3): isosceles right triangles, legs 1/2, area
        // exactly 1/8 — the octagram arm caps between the two overlaid
        // squares and the substrate walls.
        for c in t.cells.iter().filter(|c| c.seed == 3) {
            let v = &c.poly.pts;
            assert_eq!(v.len(), 3);
            assert!((c.poly.area() - 0.125).abs() < 1e-12, "tip area = 1/8");
            let right = (0..3).any(|i| {
                let e1 = sub(v[(i + 1) % 3], v[i]);
                let e2 = sub(v[(i + 2) % 3], v[i]);
                let dot = e1.x * e2.x + e1.y * e2.y;
                let l1 = e1.x * e1.x + e1.y * e1.y;
                let l2 = e2.x * e2.x + e2.y * e2.y;
                dot.abs() < 1e-12 && (l1 - 0.25).abs() < 1e-12 && (l2 - 0.25).abs() < 1e-12
            });
            assert!(right, "the tip is a right isosceles triangle with legs 1/2");
        }

        // Every interior angle is a multiple of 45° (Hankin contacts at
        // δ = 45° produce only 45°, 90° and 135° corners — the spike's
        // census).
        for c in &t.cells {
            let v = &c.poly.pts;
            let n = v.len();
            for i in 0..n {
                let e1 = sub(v[(i + n - 1) % n], v[i]);
                let e2 = sub(v[(i + 1) % n], v[i]);
                let ang = (e1.x * e2.y - e1.y * e2.x)
                    .abs()
                    .atan2(e1.x * e2.x + e1.y * e2.y);
                let k = (ang / std::f64::consts::FRAC_PI_4).round();
                assert!(
                    k >= 1.0 && (ang - k * std::f64::consts::FRAC_PI_4).abs() < 1e-9,
                    "angle {ang} at {:?} is not a multiple of 45°",
                    v[i]
                );
            }
        }
    }

    #[test]
    fn pic_4_8_8_symmetry_is_p4m() {
        let t = pic_4_8_8();
        let faces = all_faces(&t);

        // Positive probes: each must map every face to a face. All
        // matrices are exact integer/reflection maps (no trig).
        let c = v2(A / 2.0, A / 2.0); // octagon centre
        let e1 = v2(A / 2.0, 0.0); // octagon-octagon contact
        let e2 = v2(0.0, A / 2.0); // substrate-edge contact
        let positives: Vec<(&str, Isometry)> = vec![
            ("rot90 @ substrate square centre", Box::new(|p| v2(-p.y, p.x))),
            (
                "rot90 @ octagon centre",
                Box::new(move |p| {
                    let d = sub(p, c);
                    v2(c.x - d.y, c.y + d.x)
                }),
            ),
            (
                "rot180 @ octagon-octagon contact",
                Box::new(move |p| v2(2.0 * e1.x - p.x, 2.0 * e1.y - p.y)),
            ),
            (
                "rot180 @ square-square contact",
                Box::new(move |p| v2(2.0 * e2.x - p.x, 2.0 * e2.y - p.y)),
            ),
            ("mirror y = 0", Box::new(|p| v2(p.x, -p.y))),
            ("mirror x = 0", Box::new(|p| v2(-p.x, p.y))),
            ("mirror y = A/2", Box::new(move |p| v2(p.x, 2.0 * c.y - p.y))),
            ("mirror y = x", Box::new(|p| v2(p.y, p.x))),
            ("mirror y = -x", Box::new(|p| v2(-p.y, -p.x))),
        ];
        for (label, f) in &positives {
            for (i, face) in faces.iter().enumerate() {
                assert!(
                    image_is_a_face(&t, face, f),
                    "{label} must be a symmetry but fails on face {i}"
                );
            }
        }

        // Negative probes: the symmetry group is EXACTLY p4m — a quarter-
        // step rotation off the centres, a half-step twirl on a cell edge
        // line and a glide reflection must all fail on some face.
        let q = v2(A / 4.0, A / 4.0);
        let negatives: Vec<(&str, Isometry)> = vec![
            (
                "rot90 @ (A/4, A/4)",
                Box::new(move |p| {
                    let d = sub(p, q);
                    v2(q.x - d.y, q.y + d.x)
                }),
            ),
            (
                "rot180 @ (A/4, 0)",
                Box::new(move |p| v2(2.0 * q.x - p.x, -p.y)),
            ),
            ("glide (A/2, A/2−y)", Box::new(move |p| v2(p.x + A / 2.0, 2.0 * q.y - p.y))),
        ];
        for (label, f) in &negatives {
            let fails = faces.iter().filter(|face| !image_is_a_face(&t, face, f)).count();
            assert!(fails > 0, "{label} must NOT be a symmetry");
        }
    }

    #[test]
    fn pic_4_8_8_symmetry_metadata() {
        let t = pic_4_8_8();
        assert_eq!(t.id, "pic-4-8-8");
        assert_eq!(t.group, WallpaperGroup::P4m);
        assert_eq!(t.lattice_kind, LatticeKind::Square);
        assert_eq!(t.category, TilingCategory::Islamic);
    }
}

#[cfg(test)]
mod pic_3_6_3_6_tests {
    use crate::lattice::LatticeTiling;
    use super::pic_3_6_3_6::pic_3_6_3_6;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    /// √3 — `2·(√3/2)`, the substrate's apothem constant doubled (exact in
    /// f64); `consts::SQRT_3` is unstable at this crate's MSRV.
    const RT3: f64 = 2.0 * 0.866_025_403_784_438_6;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    fn sub(a: Vec2, b: Vec2) -> Vec2 {
        v2(a.x - b.x, a.y - b.y)
    }

    fn norm(a: Vec2) -> f64 {
        (a.x * a.x + a.y * a.y).sqrt()
    }

    type Isometry = Box<dyn Fn(Vec2) -> Vec2>;

    /// Rotation by 60°k about `c`, as an exact matrix in Q(√3) (no trig).
    fn rot(c: Vec2, k: usize) -> Isometry {
        let (co, si): (f64, f64) = match k % 6 {
            0 => (1.0, 0.0),
            1 => (0.5, RT3 / 2.0),
            2 => (-0.5, RT3 / 2.0),
            3 => (-1.0, 0.0),
            4 => (-0.5, -RT3 / 2.0),
            _ => (0.5, -RT3 / 2.0),
        };
        Box::new(move |p| {
            let d = sub(p, c);
            v2(c.x + co * d.x - si * d.y, c.y + si * d.x + co * d.y)
        })
    }

    /// Reflection in the line through `c` with unit direction `(u, w)`.
    fn refl(c: Vec2, u: f64, w: f64) -> Isometry {
        Box::new(move |p| {
            let d = sub(p, c);
            let pr = 2.0 * (u * d.x + w * d.y);
            v2(c.x + pr * u - d.x, c.y + pr * w - d.y)
        })
    }

    /// Is `k` an exact member of the (1/4)·Z[√3] surd grid? Exhaustive over
    /// the table's bounded range; for |q| ≤ 60 the closest q√3 gets to an
    /// integer is 97 − 56√3 ≈ 5.1e-3 ≫ 1e-9, so the decision is exact here.
    fn on_quarter_surd_grid(k: f64) -> bool {
        let k4 = 4.0 * k;
        (-60..=60).any(|q| {
            let p = k4 - q as f64 * RT3;
            (p - p.round()).abs() < 1e-9
        })
    }

    /// Positive symmetry test: the image of the face `pts` under `f` must
    /// coincide, vertex for vertex, with some orbit face plus an EXACT
    /// lattice translation `m·(2,0) + n·(1,√3)`. Table vertices live on the
    /// (1/4)·Z × (√3/4)·Z grid, whose lattice coordinates are multiples of
    /// 1/8 and 1/4, so the nearest non-lattice offset is ≈ 0.25 world units
    /// and the 1e-9 tolerance cannot blur a genuine near-miss into a hit.
    fn image_is_a_face(t: &LatticeTiling, pts: &[Vec2], f: &dyn Fn(Vec2) -> Vec2) -> bool {
        let img: Vec<Vec2> = pts.iter().map(|p| f(*p)).collect();
        for cell in &t.cells {
            let v = &cell.poly.pts;
            if v.len() != img.len() {
                continue;
            }
            for i in 0..v.len() {
                let d = sub(img[0], v[i]);
                // d = m·(2,0) + n·(1,√3)  ⟹  n = d.y/√3, m = (d.x − n)/2
                let n = d.y / RT3;
                let m = (d.x - n) / 2.0;
                if (m - m.round()).abs() > 1e-9 || (n - n.round()).abs() > 1e-9 {
                    continue;
                }
                let (mi, ni) = (m.round(), n.round());
                let shift = v2(2.0 * mi + ni, ni * RT3);
                // Vertex-SET match (orientation-free: mirrors reverse the
                // winding of the CCW-normalised table).
                let target: Vec<Vec2> = v.iter().map(|p| *p + shift).collect();
                let mut used = vec![false; target.len()];
                let mut ok = true;
                for p in &img {
                    let hit = target.iter().position(|q| norm(sub(*p, *q)) < 1e-9);
                    match hit {
                        Some(j) if !used[j] => used[j] = true,
                        _ => {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    return true;
                }
            }
        }
        false
    }

    fn all_faces(t: &LatticeTiling) -> Vec<Vec<Vec2>> {
        t.cells.iter().map(|c| c.poly.pts.clone()).collect()
    }

    #[test]
    fn pic_3_6_3_6_covers_region() {
        check_covers(
            &pic_3_6_3_6(),
            "pic-3-6-3-6",
            BBox::new(v2(-24.0, -18.0), v2(24.0, 18.0)),
        );
    }

    #[test]
    fn pic_3_6_3_6_rotated_scaled() {
        check_rotated(
            &pic_3_6_3_6(),
            "pic-3-6-3-6(rot20,scale0.6)",
            BBox::new(v2(-24.0, -18.0), v2(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn pic_3_6_3_6_deterministic() {
        check_det(
            &pic_3_6_3_6(),
            "pic-3-6-3-6",
            BBox::new(v2(-12.0, -9.0), v2(12.0, 9.0)),
        );
    }

    #[test]
    fn pic_3_6_3_6_unit_structure() {
        let t = pic_3_6_3_6();
        assert_eq!(t.cells.len(), 12);
        let by_seed = |s: u32| t.cells.iter().filter(|c| c.seed == s).count();
        assert_eq!(
            (by_seed(0), by_seed(1), by_seed(2)),
            (4, 6, 2),
            "orbit census: 4 hexagram-core hexagons, 6 hexagram tips, 2 midsegment triangles"
        );

        // Exact Q(√3) construction: every coordinate is a (1/4)·Z[√3] surd.
        for c in &t.cells {
            for p in &c.poly.pts {
                assert!(on_quarter_surd_grid(p.x), "vertex x {} off the grid", p.x);
                assert!(on_quarter_surd_grid(p.y), "vertex y {} off the grid", p.y);
            }
        }

        // Σ(cell areas) = |det(lattice)| — exact over Q(√3) (spike):
        // 4·(3√3/8) + 8·(√3/16) = 3√3/2 + √3/2 = 2√3.
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        assert!((det - 2.0 * RT3).abs() < 1e-12, "det = 2√3, got {det}");
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-12, "Σ areas {sum} vs det {det}");

        // Seed 0 — four regular hexagons of side 1/2 (the hexagram cores),
        // centred on the substrate hexagon centre and on the three
        // hexagon–hexagon contacts.
        let mut centres: Vec<(f64, f64)> = Vec::new();
        for c in t.cells.iter().filter(|c| c.seed == 0) {
            let v = &c.poly.pts;
            assert_eq!(v.len(), 6);
            for i in 0..6 {
                let side = norm(sub(v[(i + 1) % 6], v[i]));
                assert!((side - 0.5).abs() < 1e-12, "hexagon side {side}");
            }
            assert!(
                (c.poly.area() - 3.0 * RT3 / 8.0).abs() < 1e-12,
                "hexagon area = 3√3/8"
            );
            let cen = v.iter().fold(Vec2::ZERO, |a, p| a + *p);
            let cen = cen / 6.0;
            centres.push((cen.x, cen.y));
        }
        let want = [
            (0.0, 0.0),
            (1.0, 0.0),
            (0.5, RT3 / 2.0),
            (1.5, RT3 / 2.0),
        ];
        for w in want {
            assert!(
                centres.iter().any(|c| norm(sub(v2(c.0, c.1), v2(w.0, w.1))) < 1e-12),
                "no hexagram core centred on {:?}",
                w
            );
        }
        assert_eq!(centres.len(), 4);

        // Seeds 1 and 2 — eight equilateral triangles of side 1/2 and area
        // √3/16: the six hexagram arms and the two midsegment cuts.
        for c in t.cells.iter().filter(|c| c.seed != 0) {
            let v = &c.poly.pts;
            assert_eq!(v.len(), 3);
            for i in 0..3 {
                let side = norm(sub(v[(i + 1) % 3], v[i]));
                assert!((side - 0.5).abs() < 1e-12, "triangle side {side}");
            }
            assert!(
                (c.poly.area() - RT3 / 16.0).abs() < 1e-12,
                "triangle area = √3/16"
            );
        }
        // The tip apexes ride the six edge midpoints of the substrate hexagon
        // at O (circumradius 1, apothem √3/2) — the hexagram's arms.
        let m_ids = [(0.75, RT3 / 4.0), (0.0, RT3 / 2.0), (-0.75, RT3 / 4.0),
                     (-0.75, -RT3 / 4.0), (0.0, -RT3 / 2.0), (0.75, -RT3 / 4.0)];
        for m in m_ids {
            let hit = t.cells.iter().filter(|c| c.seed == 1).any(|c| {
                c.poly
                    .pts
                    .iter()
                    .any(|p| norm(sub(*p, v2(m.0, m.1))) < 1e-12)
            });
            assert!(hit, "no hexagram tip apex on the midpoint {:?}", m);
        }

        // Every interior angle is 60° or 120° — the sixfold engine at
        // δ = 60° produces only those two corners (the spike's census:
        // {60, 120} and nothing else).
        for c in &t.cells {
            let v = &c.poly.pts;
            let n = v.len();
            for i in 0..n {
                let e1 = sub(v[(i + n - 1) % n], v[i]);
                let e2 = sub(v[(i + 1) % n], v[i]);
                let ang = (e1.x * e2.y - e1.y * e2.x)
                    .abs()
                    .atan2(e1.x * e2.x + e1.y * e2.y);
                let k = (ang / std::f64::consts::FRAC_PI_6).round();
                assert!(
                    (ang - k * std::f64::consts::FRAC_PI_6).abs() < 1e-9,
                    "angle {ang} at {:?} is not a multiple of 30°",
                    v[i]
                );
                assert!(
                    (ang - std::f64::consts::FRAC_PI_3).abs() < 1e-9
                        || (ang - 2.0 * std::f64::consts::FRAC_PI_3).abs() < 1e-9,
                    "angle {ang} at {:?} is neither 60° nor 120°",
                    v[i]
                );
            }
        }
    }

    #[test]
    fn pic_3_6_3_6_symmetry_is_p6m() {
        let t = pic_3_6_3_6();
        let faces = all_faces(&t);

        // Positive probes: each must map every face to a face. Every centre
        // and axis below is an exact Q(√3) point and every map an exact
        // Q(√3) matrix (no trig).
        let o = v2(0.0, 0.0); // hexagon centre
        let w = v2(1.0, 0.0); // hexagon–hexagon contact
        let tri = v2(1.0, RT3 / 3.0); // substrate triangle centre
        let p0 = v2(0.5, 0.0); // wall midline point
        let positives: Vec<(&str, Isometry)> = vec![
            ("rot60 @ hexagon centre", rot(o, 1)),
            ("rot60 @ hexagon-hexagon contact", rot(w, 1)),
            ("rot120 @ substrate triangle centre", rot(tri, 2)),
            ("rot180 @ midline point", rot(p0, 3)),
            ("mirror y = 0", refl(o, 1.0, 0.0)),
            ("mirror x = 0", refl(o, 0.0, 1.0)),
            ("mirror y = √3", refl(v2(0.0, RT3), 1.0, 0.0)),
            ("mirror x = 2", refl(v2(2.0, 0.0), 0.0, 1.0)),
            ("mirror at 60° through O", refl(o, 0.5, RT3 / 2.0)),
        ];
        for (label, f) in &positives {
            for (i, face) in faces.iter().enumerate() {
                assert!(
                    image_is_a_face(&t, face, f),
                    "{label} must be a symmetry but fails on face {i}"
                );
            }
        }

        // Negative probes: the symmetry group is EXACTLY p6m — a right-angle
        // turn on the sixfold centre, a sixfold turn on the threefold centre,
        // a half turn off the 2-fold sites and a mirror between two mirror
        // lines must all fail on some face.
        let negatives: Vec<(&str, Isometry)> = vec![
            ("rot90 @ hexagon centre", Box::new(|p| v2(-p.y, p.x))),
            ("rot60 @ substrate triangle centre", rot(tri, 1)),
            (
                "rot180 @ (3/8, √3/8)",
                Box::new(|p| v2(0.75 - p.x, RT3 / 4.0 - p.y)),
            ),
            ("mirror y = √3/4", refl(v2(0.0, RT3 / 4.0), 1.0, 0.0)),
        ];
        for (label, f) in &negatives {
            let fails = faces.iter().filter(|face| !image_is_a_face(&t, face, f)).count();
            assert!(fails > 0, "{label} must NOT be a symmetry");
        }
    }

    #[test]
    fn pic_3_6_3_6_symmetry_metadata() {
        let t = pic_3_6_3_6();
        assert_eq!(t.id, "pic-3-6-3-6");
        assert_eq!(t.group, WallpaperGroup::P6m);
        assert_eq!(t.lattice_kind, LatticeKind::Hexagonal);
        assert_eq!(t.category, TilingCategory::Islamic);
    }
}

#[cfg(test)]
mod pic_3_12_12_tests {
    use crate::lattice::LatticeTiling;
    use super::pic_3_12_12::pic_3_12_12;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    /// √3 — `2·(√3/2)`, the substrate's apothem constant doubled (exact in
    /// f64); `consts::SQRT_3` is unstable at this crate's MSRV.
    const RT3: f64 = 2.0 * 0.866_025_403_784_438_6;
    /// The lattice basis (the 3.12.12 substrate's own translations):
    /// `a1 = (3/2+√3, 1+√3/2)`, `a2 = (0, 2+√3)`.
    const A1X: f64 = 1.5 + RT3;
    const A1Y: f64 = 1.0 + RT3 / 2.0;
    const A2Y: f64 = 2.0 + RT3;
    /// Substrate dodecagon apothem `1 + √3/2`.
    const RAP: f64 = 1.0 + RT3 / 2.0;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    fn sub(a: Vec2, b: Vec2) -> Vec2 {
        v2(a.x - b.x, a.y - b.y)
    }

    fn norm(a: Vec2) -> f64 {
        (a.x * a.x + a.y * a.y).sqrt()
    }

    fn cross(a: Vec2, b: Vec2) -> f64 {
        a.x * b.y - a.y * b.x
    }

    type Isometry = Box<dyn Fn(Vec2) -> Vec2>;

    /// Rotation by 60°·k about `c`, as an exact matrix in Q(√3) (no trig).
    fn rot(c: Vec2, k: usize) -> Isometry {
        let (co, si): (f64, f64) = match k % 6 {
            0 => (1.0, 0.0),
            1 => (0.5, RT3 / 2.0),
            2 => (-0.5, RT3 / 2.0),
            3 => (-1.0, 0.0),
            4 => (-0.5, -RT3 / 2.0),
            _ => (0.5, -RT3 / 2.0),
        };
        Box::new(move |p| {
            let d = sub(p, c);
            v2(c.x + co * d.x - si * d.y, c.y + si * d.x + co * d.y)
        })
    }

    /// Rotation by 30° about `c`; cos 30° = √3/2 and sin 30° = 1/2, so this
    /// too is exact over Q(√3). Used as a negative probe.
    fn rot30(c: Vec2) -> Isometry {
        Box::new(move |p| {
            let d = sub(p, c);
            v2(
                c.x + RT3 / 2.0 * d.x - 0.5 * d.y,
                c.y + 0.5 * d.x + RT3 / 2.0 * d.y,
            )
        })
    }

    /// Reflection in the line through `c` with unit direction `(u, w)`.
    fn refl(c: Vec2, u: f64, w: f64) -> Isometry {
        Box::new(move |p| {
            let d = sub(p, c);
            let pr = 2.0 * (u * d.x + w * d.y);
            v2(c.x + pr * u - d.x, c.y + pr * w - d.y)
        })
    }

    /// Is `k` an exact member of the (1/24)·Z[√3] surd grid? Exhaustive over
    /// the table's bounded numerators (|p| ≤ 33, |q| ≤ 18): for q ≠ Q the gap
    /// `|P + (Q−q)√3| ≥ 1/(|P| + |Q−q|√3) ≈ 7.5e-3`, so the 1e-9 decision is
    /// exact here — no near-miss can be rounded into a hit.
    fn on_twentyfourth_surd_grid(k: f64) -> bool {
        let k24 = 24.0 * k;
        (-60..=60).any(|q| {
            let p = k24 - q as f64 * RT3;
            (p - p.round()).abs() < 1e-9
        })
    }

    /// Every corner angle of a face, rounded to the 30° lattice; asserts
    /// outright that no corner falls off that lattice.
    fn corners(v: &[Vec2]) -> Vec<u32> {
        let n = v.len();
        (0..n)
            .map(|i| {
                let e1 = sub(v[(i + n - 1) % n], v[i]);
                let e2 = sub(v[(i + 1) % n], v[i]);
                let a = cross(e1, e2).abs().atan2(e1.x * e2.x + e1.y * e2.y);
                let k = (a / std::f64::consts::FRAC_PI_6).round();
                assert!(
                    (a - k * std::f64::consts::FRAC_PI_6).abs() < 1e-9,
                    "corner {a} at {:?} is not a multiple of 30°",
                    v[i]
                );
                (k * 30.0) as u32
            })
            .collect()
    }

    /// The 12 vertices of the substrate dodecagon centred at `c`: the vertex
    /// `(1 + √3/2, 1/2)` turned through 30°·k — all exact Q(√3) points.
    fn dodecagon_vertices(c: Vec2) -> Vec<Vec2> {
        let (co, si) = (RT3 / 2.0, 0.5);
        let mut p = v2(RAP, 0.5);
        let mut out = Vec::with_capacity(12);
        for _ in 0..12 {
            out.push(c + p);
            p = v2(co * p.x - si * p.y, si * p.x + co * p.y);
        }
        out
    }

    /// The 3.12.12 vertices (one triangle + two dodecagons each) around the
    /// origin: the vertices of the dodecagons on the nearby lattice points.
    fn substrate_vertices() -> Vec<Vec2> {
        let mut out: Vec<Vec2> = Vec::new();
        for m in -2..=2i32 {
            for n in -2..=2i32 {
                let c = v2(m as f64 * A1X, m as f64 * A1Y + n as f64 * A2Y);
                for p in dodecagon_vertices(c) {
                    if !out.iter().any(|q| norm(sub(*q, p)) < 1e-9) {
                        out.push(p);
                    }
                }
            }
        }
        out
    }

    /// Strict containment in a convex face (every face of this partition is
    /// convex: the corner census stops at 150°).
    fn strictly_inside(p: Vec2, v: &[Vec2]) -> bool {
        let n = v.len();
        let s: Vec<f64> = (0..n)
            .map(|i| cross(sub(v[(i + 1) % n], v[i]), sub(p, v[i])))
            .collect();
        s.iter().all(|c| *c > 1e-9) || s.iter().all(|c| *c < -1e-9)
    }

    /// Positive symmetry test: the image of the face `pts` under `f` must
    /// coincide, vertex for vertex, with some orbit face plus an EXACT
    /// lattice translation `m·a1 + n·a2`. Table vertices live on
    /// `(1/24)·Z[√3]`, so a lattice coordinate of a difference is
    /// `((12Q−6P) + (4P−6Q)√3)/72`; with the table's numerators bounded by
    /// 33 and 18, a non-lattice offset is at least ≈ 8e-6 away — three orders
    /// of magnitude above the 1e-9 tolerance, which therefore cannot blur a
    /// genuine near-miss into a hit.
    fn image_is_a_face(t: &LatticeTiling, pts: &[Vec2], f: &dyn Fn(Vec2) -> Vec2) -> bool {
        let img: Vec<Vec2> = pts.iter().map(|p| f(*p)).collect();
        for cell in &t.cells {
            let v = &cell.poly.pts;
            if v.len() != img.len() {
                continue;
            }
            for i in 0..v.len() {
                let d = sub(img[0], v[i]);
                // d = m·(3/2+√3, 1+√3/2) + n·(0, 2+√3)
                let m = d.x / A1X;
                let n = (d.y - m * A1Y) / A2Y;
                if (m - m.round()).abs() > 1e-9 || (n - n.round()).abs() > 1e-9 {
                    continue;
                }
                let (mi, ni) = (m.round(), n.round());
                let shift = v2(mi * A1X, mi * A1Y + ni * A2Y);
                // Vertex-SET match (orientation-free: mirrors reverse the
                // winding of the CCW-normalised table).
                let target: Vec<Vec2> = v.iter().map(|p| *p + shift).collect();
                let mut used = vec![false; target.len()];
                let mut ok = true;
                for p in &img {
                    let hit = target.iter().position(|q| norm(sub(*p, *q)) < 1e-9);
                    match hit {
                        Some(j) if !used[j] => used[j] = true,
                        _ => {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    return true;
                }
            }
        }
        false
    }

    fn all_faces(t: &LatticeTiling) -> Vec<Vec<Vec2>> {
        t.cells.iter().map(|c| c.poly.pts.clone()).collect()
    }

    #[test]
    fn pic_3_12_12_covers_region() {
        check_covers(
            &pic_3_12_12(),
            "pic-3-12-12",
            BBox::new(v2(-24.0, -18.0), v2(24.0, 18.0)),
        );
    }

    #[test]
    fn pic_3_12_12_rotated_scaled() {
        check_rotated(
            &pic_3_12_12(),
            "pic-3-12-12(rot20,scale0.6)",
            BBox::new(v2(-24.0, -18.0), v2(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn pic_3_12_12_deterministic() {
        check_det(
            &pic_3_12_12(),
            "pic-3-12-12",
            BBox::new(v2(-12.0, -9.0), v2(12.0, 9.0)),
        );
    }

    #[test]
    fn pic_3_12_12_unit_structure() {
        let t = pic_3_12_12();
        assert_eq!(t.cells.len(), 45);
        let by_seed = |s: u32| t.cells.iter().filter(|c| c.seed == s).count();
        assert_eq!(
            (by_seed(0), by_seed(1), by_seed(2), by_seed(3), by_seed(4), by_seed(5)),
            (1, 6, 12, 2, 12, 12),
            "orbit census: rosette core, 6 vertex pentagons, 12 band quads, \
             2 medial triangles, 12 inner quads, 12 star notches"
        );

        // Exact Q(√3) construction: every coordinate is a (1/24)·Z[√3] surd.
        for c in &t.cells {
            for p in &c.poly.pts {
                assert!(on_twentyfourth_surd_grid(p.x), "vertex x {} off the grid", p.x);
                assert!(on_twentyfourth_surd_grid(p.y), "vertex y {} off the grid", p.y);
            }
        }

        // Σ(cell areas) = |det(lattice)| — exact over Q(√3) (spike):
        // (3/2+3√3/4) + 6·(3/8+3√3/16) + 12·(1/8+√3/12) + 2·(√3/16)
        //   + 12·(1/16+√3/48) + 12·(√3/48) = 6 + 7√3/2.
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        assert!(
            (det - (6.0 + 3.5 * RT3)).abs() < 1e-12,
            "det = 6 + 7√3/2, got {det}"
        );
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-12, "Σ areas {sum} vs det {det}");

        // Seed 0 — one regular dodecagon of side 1/2 (the {12/4} star core)
        // centred on the substrate dodecagon at O.
        let cores: Vec<_> = t.cells.iter().filter(|c| c.seed == 0).collect();
        assert_eq!(cores.len(), 1);
        let v = &cores[0].poly.pts;
        assert_eq!(v.len(), 12);
        for i in 0..12 {
            let side = norm(sub(v[(i + 1) % 12], v[i]));
            assert!((side - 0.5).abs() < 1e-12, "core side {side}");
        }
        assert!(
            (cores[0].poly.area() - (1.5 + 0.75 * RT3)).abs() < 1e-12,
            "core area = 3/2 + 3√3/4"
        );
        let cen = v.iter().fold(Vec2::ZERO, |a, p| a + *p) / 12.0;
        assert!(norm(cen) < 1e-12, "core centred on O, got {cen}");
        assert_eq!(corners(v), vec![150; 12]);

        // Seed 1 — the six vertex pentagons: four long chords of length
        // ℓ = √(1/4+√3/8) plus the short midsegment side of length 1/2, and
        // each wraps exactly one 3.12.12 vertex.
        let subs = substrate_vertices();
        let mut wrapped = 0;
        for c in t.cells.iter().filter(|c| c.seed == 1) {
            let v = &c.poly.pts;
            assert_eq!(v.len(), 5);
            let ell = (0.25 + RT3 / 8.0).sqrt();
            let sides: Vec<f64> = (0..5)
                .map(|i| norm(sub(v[(i + 1) % 5], v[i])))
                .collect();
            let shorts = sides.iter().filter(|s| (**s - 0.5).abs() < 1e-12).count();
            let longs = sides.iter().filter(|s| (**s - ell).abs() < 1e-12).count();
            assert_eq!((shorts, longs), (1, 4), "pentagon sides {sides:?}");
            assert!(
                (c.poly.area() - (0.375 + 0.1875 * RT3)).abs() < 1e-12,
                "pentagon area = 3/8 + 3√3/16"
            );
            assert_eq!(
                {
                    let mut got = corners(v);
                    got.sort_unstable();
                    got
                },
                vec![90, 90, 120, 120, 120]
            );
            wrapped += subs.iter().filter(|p| strictly_inside(**p, v)).count();
        }
        assert_eq!(wrapped, 6, "one 3.12.12 vertex wrapped per pentagon");

        // Seeds 2, 4 — the two quadrilateral orbits (band quads with sides
        // ℓ, ℓ, m, m and inner quads with sides √3/6, √3/6, m, m, where
        // m = √(1/12+√3/24)).
        let m = (1.0 / 12.0 + RT3 / 24.0).sqrt();
        let ell = (0.25 + RT3 / 8.0).sqrt();
        for (seed, area, angles) in [(2u32, 0.125 + RT3 / 12.0, vec![60, 90, 90, 120]),
                                     (4, 1.0 / 16.0 + RT3 / 48.0, vec![60, 60, 90, 150])]
        {
            for c in t.cells.iter().filter(|c| c.seed == seed) {
                let v = &c.poly.pts;
                assert_eq!(v.len(), 4);
                let sides: Vec<f64> = (0..4)
                    .map(|i| norm(sub(v[(i + 1) % 4], v[i])))
                    .collect();
                let two = |s: f64| sides.iter().filter(|x| (**x - s).abs() < 1e-12).count();
                if seed == 2 {
                    assert_eq!((two(ell), two(m)), (2, 2), "band quad sides {sides:?}");
                } else {
                    assert_eq!((two(RT3 / 6.0), two(m)), (2, 2), "inner quad sides {sides:?}");
                }
                let mut got = corners(v);
                got.sort_unstable();
                assert_eq!(got, angles);
                assert!(
                    (c.poly.area() - area).abs() < 1e-12,
                    "seed {seed} quad area"
                );
            }
        }

        // Seed 3 — the midsegment medial triangle of each substrate triangle:
        // equilateral of side 1/2, area √3/16, centred on a substrate
        // triangle centre (1 + 2√3/3, 0) modulo the lattice.
        for c in t.cells.iter().filter(|c| c.seed == 3) {
            let v = &c.poly.pts;
            assert_eq!(v.len(), 3);
            for i in 0..3 {
                let side = norm(sub(v[(i + 1) % 3], v[i]));
                assert!((side - 0.5).abs() < 1e-12, "medial side {side}");
            }
            assert!(
                (c.poly.area() - RT3 / 16.0).abs() < 1e-12,
                "medial area = √3/16"
            );
            let cen = v.iter().fold(Vec2::ZERO, |a, p| a + *p) / 3.0;
            let hit = [-1.0, 1.0].iter().any(|s| {
                let w = v2(s * (1.0 + 2.0 * RT3 / 3.0), 0.0);
                let d = sub(cen, w);
                let m = d.x / A1X;
                let n = (d.y - m * A1Y) / A2Y;
                (m - m.round()).abs() < 1e-9 && (n - n.round()).abs() < 1e-9
            });
            assert!(hit, "medial triangle off a substrate triangle centre: {cen}");
        }

        // Seed 5 — the twelve star notches: isosceles triangles with two
        // chords of √3/6 against a 1/2 base, corners 30°/120°/30°.
        for c in t.cells.iter().filter(|c| c.seed == 5) {
            let v = &c.poly.pts;
            assert_eq!(v.len(), 3);
            let sides: Vec<f64> = (0..3)
                .map(|i| norm(sub(v[(i + 1) % 3], v[i])))
                .collect();
            let two = sides.iter().filter(|s| (**s - RT3 / 6.0).abs() < 1e-12).count();
            let base = sides.iter().filter(|s| (**s - 0.5).abs() < 1e-12).count();
            assert_eq!((two, base), (2, 1), "notch sides {sides:?}");
            assert!(
                (c.poly.area() - RT3 / 48.0).abs() < 1e-12,
                "notch area = √3/48"
            );
            let mut got = corners(v);
            got.sort_unstable();
            assert_eq!(got, vec![30, 30, 120]);
        }

        // No face contains a substrate vertex except the six pentagons: the
        // rosette pieces stay inside their own dodecagon and the medial
        // triangles inside their own triangle.
        for c in t.cells.iter().filter(|c| c.seed != 1) {
            let n = subs.iter().filter(|p| strictly_inside(**p, &c.poly.pts)).count();
            assert_eq!(n, 0, "seed {} face holds a substrate vertex", c.seed);
        }
    }

    #[test]
    fn pic_3_12_12_symmetry_is_p6m() {
        let t = pic_3_12_12();
        let faces = all_faces(&t);

        // Positive probes: each must map every face to a face. Every centre
        // and axis below is an exact Q(√3) point and every map an exact
        // Q(√3) matrix (no trig).
        let o = v2(0.0, 0.0); // substrate dodecagon centre
        let tri = v2(1.0 + 2.0 * RT3 / 3.0, 0.0); // substrate triangle centre
        let m1 = v2(RAP * RT3 / 2.0, RAP / 2.0); // dodecagon-dodecagon midpoint
        let positives: Vec<(&str, Isometry)> = vec![
            ("rot60 @ dodecagon centre", rot(o, 1)),
            ("rot120 @ dodecagon centre", rot(o, 2)),
            ("rot180 @ dodecagon centre", rot(o, 3)),
            ("rot120 @ substrate triangle centre", rot(tri, 2)),
            ("rot180 @ dodecagon-dodecagon midpoint", rot(m1, 3)),
            ("mirror y = 0", refl(o, 1.0, 0.0)),
            ("mirror x = 0", refl(o, 0.0, 1.0)),
            ("mirror at 30° through O", refl(o, RT3 / 2.0, 0.5)),
        ];
        for (label, f) in &positives {
            for (i, face) in faces.iter().enumerate() {
                assert!(
                    image_is_a_face(&t, face, f),
                    "{label} must be a symmetry but fails on face {i}"
                );
            }
        }

        // Negative probes: the symmetry group is EXACTLY p6m — a 30° or 90°
        // turn on the sixfold centre, a sixfold turn on the threefold centre,
        // half turns on the wrong sites and a mirror between two mirror lines
        // must all fail on some face.
        let m0 = v2(RAP, 0.0); // dodecagon-triangle midpoint
        let vert = v2(RAP, 0.5); // a 3.12.12 vertex
        let negatives: Vec<(&str, Isometry)> = vec![
            ("rot30 @ dodecagon centre", rot30(o)),
            ("rot90 @ dodecagon centre", Box::new(|p| v2(-p.y, p.x))),
            ("rot60 @ substrate triangle centre", rot(tri, 1)),
            ("rot180 @ dodecagon-triangle midpoint", rot(m0, 3)),
            ("rot180 @ substrate vertex", rot(vert, 3)),
            ("mirror y = √3/2", refl(v2(0.0, RT3 / 2.0), 1.0, 0.0)),
            ("mirror x = 1", refl(v2(1.0, 0.0), 0.0, 1.0)),
        ];
        for (label, f) in &negatives {
            let fails = faces
                .iter()
                .filter(|face| !image_is_a_face(&t, face, f))
                .count();
            assert!(fails > 0, "{label} must NOT be a symmetry");
        }
    }

    #[test]
    fn pic_3_12_12_symmetry_metadata() {
        let t = pic_3_12_12();
        assert_eq!(t.id, "pic-3-12-12");
        assert_eq!(t.group, WallpaperGroup::P6m);
        assert_eq!(t.lattice_kind, LatticeKind::Hexagonal);
        assert_eq!(t.category, TilingCategory::Islamic);
    }
}


/// M94 Hankin-method engine tests: output equivalence with the validated
/// 40-dps Python prototype (machine-transcribed expectations), engine
/// invariants, and the standard registry checks for the three shipped
/// configurations.
#[cfg(test)]
mod hankin_engine_tests {
    use super::hankin;
    use super::hankin_3_12_12;
    use super::hankin_3_3_3_4_4;
    use super::hankin_3_3_4_3_4;
    use super::hankin_3_4_6_4;
    use super::hankin_3_6_3_6;
    use super::hankin_4_6_12;
    use super::hankin_4_8_8;
    use super::hankin_4_8_8_rosette_transform;
    use super::hankin_hex;
    use super::hankin_square_two_point;
    use tess_core::geometry::Vec2;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Rebuild canonical faces from the flat machine-transcribed arrays.
    fn expected_faces(verts: &[f64], lens: &[usize]) -> Vec<Vec<Vec2>> {
        let mut out = Vec::new();
        let mut k = 0;
        for &n in lens {
            let face: Vec<Vec2> = verts[k..k + 2 * n]
                .chunks_exact(2)
                .map(|c| v2(c[0], c[1]))
                .collect();
            k += 2 * n;
            out.push(face);
        }
        out
    }

    fn assert_faces_match(actual: &[Vec<Vec2>], expected: &[Vec<Vec2>], label: &str) {
        assert_eq!(
            actual.len(),
            expected.len(),
            "{label}: face count {} vs expected {}",
            actual.len(),
            expected.len()
        );
        for (a, e) in actual.iter().zip(expected.iter()) {
            assert_eq!(a.len(), e.len(), "{label}: vertex count differs");
            for (p, q) in a.iter().zip(e.iter()) {
                assert!(
                    (p.x - q.x).abs() < 1e-9 && (p.y - q.y).abs() < 1e-9,
                    "{label}: vertex {p} vs expected ({}, {})",
                    q.x,
                    q.y
                );
            }
        }
    }

    #[test]
    fn engine_matches_prototype_4_8_8() {
        let (cells, report) = hankin::build_cells(
            &hankin_4_8_8::tiles(),
            hankin_4_8_8::THETA_E_DEG,
            hankin_4_8_8::DELTA,
        );
        assert!(report.complete, "octagon+diamond pairing must be complete");
        let lens: Vec<usize> = report.per_tile.iter().map(|r| r.faces).collect();
        assert_eq!(lens, vec![9, 5], "face census per tile class");
        for r in &report.per_tile {
            assert!((r.area_sum - r.tile_area).abs() < 1e-9);
            assert_eq!(r.rays, r.pairs * 2, "every ray paired exactly once");
        }
        // Output equivalence against the machine-transcribed prototype.
        let expected: Vec<Vec<Vec2>> = expected_faces(
            hankin_4_8_8::EXPECTED_0_VERTS,
            hankin_4_8_8::EXPECTED_0_LENS,
        )
        .into_iter()
        .chain(expected_faces(
            hankin_4_8_8::EXPECTED_1_VERTS,
            hankin_4_8_8::EXPECTED_1_LENS,
        ))
        .collect();
        let actual: Vec<Vec<Vec2>> = cells.iter().map(|c| c.poly.pts.clone()).collect();
        assert_faces_match(&actual, &expected, "hankin-4-8-8");
    }

    #[test]
    fn engine_matches_prototype_4_6_12() {
        let (cells, report) = hankin::build_cells(
            &hankin_4_6_12::tiles(),
            hankin_4_6_12::THETA_E_DEG,
            hankin_4_6_12::DELTA,
        );
        assert!(report.complete);
        let lens: Vec<usize> = report.per_tile.iter().map(|r| r.faces).collect();
        assert_eq!(lens, vec![13, 5, 5, 5, 7, 7], "face census per tile class");
        for r in &report.per_tile {
            assert!((r.area_sum - r.tile_area).abs() < 1e-9);
            assert_eq!(r.rays, r.pairs * 2);
        }
        let mut expected: Vec<Vec<Vec2>> = Vec::new();
        for k in 0..6 {
            expected.extend(match k {
                0 => expected_faces(hankin_4_6_12::EXPECTED_0_VERTS, hankin_4_6_12::EXPECTED_0_LENS),
                1 => expected_faces(hankin_4_6_12::EXPECTED_1_VERTS, hankin_4_6_12::EXPECTED_1_LENS),
                2 => expected_faces(hankin_4_6_12::EXPECTED_2_VERTS, hankin_4_6_12::EXPECTED_2_LENS),
                3 => expected_faces(hankin_4_6_12::EXPECTED_3_VERTS, hankin_4_6_12::EXPECTED_3_LENS),
                4 => expected_faces(hankin_4_6_12::EXPECTED_4_VERTS, hankin_4_6_12::EXPECTED_4_LENS),
                _ => expected_faces(hankin_4_6_12::EXPECTED_5_VERTS, hankin_4_6_12::EXPECTED_5_LENS),
            });
        }
        let actual: Vec<Vec<Vec2>> = cells.iter().map(|c| c.poly.pts.clone()).collect();
        assert_faces_match(&actual, &expected, "hankin-4-6-12");
    }

    #[test]
    fn engine_matches_prototype_hex_two_point() {
        let (cells, report) = hankin::build_cells(
            &hankin_hex::tiles(),
            hankin_hex::THETA_E_DEG,
            hankin_hex::DELTA,
        );
        assert!(report.complete);
        let r0 = &report.per_tile[0];
        assert_eq!((r0.rays, r0.pairs, r0.faces), (12, 6, 13));
        assert!((r0.area_sum - r0.tile_area).abs() < 1e-9);
        assert_faces_match(
            &cells.iter().map(|c| c.poly.pts.clone()).collect::<Vec<_>>(),
            &expected_faces(hankin_hex::EXPECTED_0_VERTS, hankin_hex::EXPECTED_0_LENS),
            "hankin-hex-two-point",
        );
    }

    #[test]
    fn engine_matches_emission_3_12_12() {
        // M99: no 40-dps Python prototype for this substrate — the
        // expectations were emitted from the Rust engine itself, so
        // this test pins the self-consistency of that emission.
        let (cells, report) = hankin::build_cells(
            &hankin_3_12_12::tiles(),
            hankin_3_12_12::THETA_E_DEG,
            hankin_3_12_12::DELTA,
        );
        assert!(report.complete);
        let lens: Vec<usize> = report.per_tile.iter().map(|r| r.faces).collect();
        assert_eq!(lens, vec![13, 4, 4], "face census per tile class");
        let rays: Vec<usize> = report.per_tile.iter().map(|r| r.rays).collect();
        let pairs: Vec<usize> = report.per_tile.iter().map(|r| r.pairs).collect();
        assert_eq!(rays, vec![24, 6, 6], "rays per tile class");
        assert_eq!(pairs, vec![12, 3, 3], "pairs per tile class");
        for r in &report.per_tile {
            assert!((r.area_sum - r.tile_area).abs() < 1e-9);
            assert_eq!(r.rays, r.pairs * 2);
        }
        let mut expected: Vec<Vec<Vec2>> = Vec::new();
        for k in 0..3 {
            expected.extend(match k {
                0 => expected_faces(hankin_3_12_12::EXPECTED_0_VERTS, hankin_3_12_12::EXPECTED_0_LENS),
                1 => expected_faces(hankin_3_12_12::EXPECTED_1_VERTS, hankin_3_12_12::EXPECTED_1_LENS),
                _ => expected_faces(hankin_3_12_12::EXPECTED_2_VERTS, hankin_3_12_12::EXPECTED_2_LENS),
            });
        }
        let actual: Vec<Vec<Vec2>> = cells.iter().map(|c| c.poly.pts.clone()).collect();
        assert_faces_match(&actual, &expected, "hankin-3-12-12");
    }

    #[test]
    fn engine_matches_emission_square_two_point() {
        // M99: no 40-dps Python prototype for this substrate — the
        // expectations were emitted from the Rust engine itself, so
        // this test pins the self-consistency of that emission.
        let (cells, report) = hankin::build_cells(
            &hankin_square_two_point::tiles(),
            hankin_square_two_point::THETA_E_DEG,
            hankin_square_two_point::DELTA,
        );
        assert!(report.complete);
        let r0 = &report.per_tile[0];
        assert_eq!((r0.rays, r0.pairs, r0.faces), (8, 4, 7));
        assert!((r0.area_sum - r0.tile_area).abs() < 1e-9);
        assert_faces_match(
            &cells.iter().map(|c| c.poly.pts.clone()).collect::<Vec<_>>(),
            &expected_faces(
                hankin_square_two_point::EXPECTED_0_VERTS,
                hankin_square_two_point::EXPECTED_0_LENS,
            ),
            "hankin-square-two-point",
        );
    }

    #[test]
    fn engine_matches_emission_3_6_3_6() {
        // M100: no 40-dps Python prototype for this substrate — the
        // expectations were emitted from the Rust engine itself, so
        // this test pins the self-consistency of that emission. The
        // hexagon tile pairs completely; the two triangle tiles pair
        // with nothing (0 pairs) and their motif stays the original
        // single-face triangle, so the whole-cell report is not
        // "complete" and per-tile completeness is asserted instead.
        let (cells, report) = hankin::build_cells(
            &hankin_3_6_3_6::tiles(),
            hankin_3_6_3_6::THETA_E_DEG,
            hankin_3_6_3_6::DELTA,
        );
        assert!(!report.complete, "triangle tiles are unpaired by design");
        let lens: Vec<usize> = report.per_tile.iter().map(|r| r.faces).collect();
        assert_eq!(lens, vec![7, 1, 1], "face census per tile class");
        let rays: Vec<usize> = report.per_tile.iter().map(|r| r.rays).collect();
        let pairs: Vec<usize> = report.per_tile.iter().map(|r| r.pairs).collect();
        assert_eq!(rays, vec![12, 6, 6], "rays per tile class");
        assert_eq!(pairs, vec![6, 0, 0], "pairs per tile class");
        assert!(report.per_tile[0].complete, "hexagon pairing must be complete");
        assert!(!report.per_tile[1].complete, "triangle tiles stay unpaired");
        assert!(!report.per_tile[2].complete, "triangle tiles stay unpaired");
        for r in &report.per_tile {
            assert!((r.area_sum - r.tile_area).abs() < 1e-9);
        }
        let mut expected: Vec<Vec<Vec2>> = Vec::new();
        for k in 0..3 {
            expected.extend(match k {
                0 => expected_faces(hankin_3_6_3_6::EXPECTED_0_VERTS, hankin_3_6_3_6::EXPECTED_0_LENS),
                1 => expected_faces(hankin_3_6_3_6::EXPECTED_1_VERTS, hankin_3_6_3_6::EXPECTED_1_LENS),
                _ => expected_faces(hankin_3_6_3_6::EXPECTED_2_VERTS, hankin_3_6_3_6::EXPECTED_2_LENS),
            });
        }
        let actual: Vec<Vec<Vec2>> = cells.iter().map(|c| c.poly.pts.clone()).collect();
        assert_faces_match(&actual, &expected, "hankin-3-6-3-6");
    }

    #[test]
    fn engine_matches_emission_3_4_6_4() {
        // M100: no 40-dps Python prototype for this substrate — the
        // expectations were emitted from the Rust engine itself, so
        // this test pins the self-consistency of that emission. The
        // hexagon tile pairs completely; the three square tiles pair
        // with nothing (0 pairs) and their motif stays the original
        // single-face square; the two triangle tiles sit in the
        // 4-subdivision limit (3 pairs, 4 side-1/2 equilateral
        // faces each), so the whole-cell report is not "complete"
        // and per-tile completeness is asserted instead.
        let (cells, report) = hankin::build_cells(
            &hankin_3_4_6_4::tiles(),
            hankin_3_4_6_4::THETA_E_DEG,
            hankin_3_4_6_4::DELTA,
        );
        assert!(!report.complete, "square tiles are unpaired by design");
        let lens: Vec<usize> = report.per_tile.iter().map(|r| r.faces).collect();
        assert_eq!(lens, vec![7, 1, 1, 1, 4, 4], "face census per tile instance");
        let rays: Vec<usize> = report.per_tile.iter().map(|r| r.rays).collect();
        let pairs: Vec<usize> = report.per_tile.iter().map(|r| r.pairs).collect();
        assert_eq!(rays, vec![12, 8, 8, 8, 6, 6], "rays per tile instance");
        assert_eq!(pairs, vec![6, 0, 0, 0, 3, 3], "pairs per tile instance");
        assert!(report.per_tile[0].complete, "hexagon pairing must be complete");
        for i in 1..4 {
            assert!(!report.per_tile[i].complete, "square tiles stay unpaired");
            assert_eq!(report.per_tile[i].faces, 1, "square motif stays a single face");
        }
        assert!(report.per_tile[4].complete, "triangle 4-subdivision is complete");
        assert!(report.per_tile[5].complete, "triangle 4-subdivision is complete");
        for r in &report.per_tile {
            assert!((r.area_sum - r.tile_area).abs() < 1e-9);
        }
        let mut expected: Vec<Vec<Vec2>> = Vec::new();
        for k in 0..6 {
            expected.extend(match k {
                0 => expected_faces(hankin_3_4_6_4::EXPECTED_0_VERTS, hankin_3_4_6_4::EXPECTED_0_LENS),
                1 => expected_faces(hankin_3_4_6_4::EXPECTED_1_VERTS, hankin_3_4_6_4::EXPECTED_1_LENS),
                2 => expected_faces(hankin_3_4_6_4::EXPECTED_2_VERTS, hankin_3_4_6_4::EXPECTED_2_LENS),
                3 => expected_faces(hankin_3_4_6_4::EXPECTED_3_VERTS, hankin_3_4_6_4::EXPECTED_3_LENS),
                4 => expected_faces(hankin_3_4_6_4::EXPECTED_4_VERTS, hankin_3_4_6_4::EXPECTED_4_LENS),
                _ => expected_faces(hankin_3_4_6_4::EXPECTED_5_VERTS, hankin_3_4_6_4::EXPECTED_5_LENS),
            });
        }
        let actual: Vec<Vec<Vec2>> = cells.iter().map(|c| c.poly.pts.clone()).collect();
        assert_faces_match(&actual, &expected, "hankin-3-4-6-4");
    }

    #[test]
    fn engine_matches_emission_3_3_3_4_4() {
        // M107: no 40-dps Python prototype for this substrate — the
        // expectations were emitted from the Rust engine itself, so
        // this test pins the self-consistency of that emission. All
        // three tile instances pair completely at θK = 60°: both
        // triangles in the side-1/2 4-subdivision limit (3 pairs, 4
        // equilateral faces each) and the square in its 5-face motif
        // (4 pairs, four corner quadrilaterals + central octagon).
        let (cells, report) = hankin::build_cells(
            &hankin_3_3_3_4_4::tiles(),
            hankin_3_3_3_4_4::THETA_E_DEG,
            hankin_3_3_3_4_4::DELTA,
        );
        assert!(report.complete, "all three tiles must pair completely");
        let lens: Vec<usize> = report.per_tile.iter().map(|r| r.faces).collect();
        assert_eq!(lens, vec![4, 4, 5], "face census per tile instance");
        let rays: Vec<usize> = report.per_tile.iter().map(|r| r.rays).collect();
        let pairs: Vec<usize> = report.per_tile.iter().map(|r| r.pairs).collect();
        assert_eq!(rays, vec![6, 6, 8], "rays per tile instance");
        assert_eq!(pairs, vec![3, 3, 4], "pairs per tile instance");
        for r in &report.per_tile {
            assert!(r.complete, "every tile pairs completely");
            assert!((r.area_sum - r.tile_area).abs() < 1e-9);
        }
        let mut expected: Vec<Vec<Vec2>> = Vec::new();
        for k in 0..3 {
            expected.extend(match k {
                0 => expected_faces(hankin_3_3_3_4_4::EXPECTED_0_VERTS, hankin_3_3_3_4_4::EXPECTED_0_LENS),
                1 => expected_faces(hankin_3_3_3_4_4::EXPECTED_1_VERTS, hankin_3_3_3_4_4::EXPECTED_1_LENS),
                _ => expected_faces(hankin_3_3_3_4_4::EXPECTED_2_VERTS, hankin_3_3_3_4_4::EXPECTED_2_LENS),
            });
        }
        let actual: Vec<Vec<Vec2>> = cells.iter().map(|c| c.poly.pts.clone()).collect();
        assert_faces_match(&actual, &expected, "hankin-3-3-3-4-4");
    }

    #[test]
    fn engine_matches_emission_3_3_4_3_4() {
        // M111: no 40-dps Python prototype for this substrate — the
        // expectations were emitted from a faithful f64 port of the
        // Rust engine (the M111 spike) and cross-audited exactly in
        // the Q(√2, √3) ring, so this test pins the self-consistency
        // of that emission. All twelve tile instances pair completely
        // at θK = 60°: each of the four square tiles in its 5-face
        // motif (4 pairs: four corner quadrilaterals + central
        // octagon) and each of the eight triangle tiles in the
        // side-1/2 4-subdivision limit (3 pairs, four equilateral
        // faces each) — 52 faces per lattice cell.
        let (cells, report) = hankin::build_cells(
            &hankin_3_3_4_3_4::tiles(),
            hankin_3_3_4_3_4::THETA_E_DEG,
            hankin_3_3_4_3_4::DELTA,
        );
        assert!(report.complete, "all twelve tiles must pair completely");
        let lens: Vec<usize> = report.per_tile.iter().map(|r| r.faces).collect();
        assert_eq!(
            lens,
            vec![5, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4],
            "face census per tile instance"
        );
        let rays: Vec<usize> = report.per_tile.iter().map(|r| r.rays).collect();
        let pairs: Vec<usize> = report.per_tile.iter().map(|r| r.pairs).collect();
        assert_eq!(
            rays,
            vec![8, 8, 8, 8, 6, 6, 6, 6, 6, 6, 6, 6],
            "rays per tile instance"
        );
        assert_eq!(
            pairs,
            vec![4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3],
            "pairs per tile instance"
        );
        for r in &report.per_tile {
            assert!(r.complete, "every tile pairs completely");
            assert!((r.area_sum - r.tile_area).abs() < 1e-9);
        }
        let expected = expected_faces(
            hankin_3_3_4_3_4::EXPECTED_N_VERTS,
            hankin_3_3_4_3_4::EXPECTED_LENS,
        );
        let actual: Vec<Vec<Vec2>> = cells.iter().map(|c| c.poly.pts.clone()).collect();
        assert_faces_match(&actual, &expected, "hankin-3-3-4-3-4");
    }

    #[test]
    fn theta_parameters_are_conjugate() {
        // Engine angle θe = 90° − Kaplan contact angle θK, for every
        // shipped configuration (the constants were emitted separately,
        // so this pins their relationship independently).
        assert!((hankin_4_8_8::THETA_E_DEG - (90.0 - hankin_4_8_8::THETA_K_DEG)).abs() < 1e-12);
        assert!((hankin_4_6_12::THETA_E_DEG - (90.0 - hankin_4_6_12::THETA_K_DEG)).abs() < 1e-12);
        assert!((hankin_hex::THETA_E_DEG - (90.0 - hankin_hex::THETA_K_DEG)).abs() < 1e-12);
assert!((hankin_3_12_12::THETA_E_DEG - (90.0 - hankin_3_12_12::THETA_K_DEG)).abs() < 1e-12);
        assert!((hankin_square_two_point::THETA_E_DEG - (90.0 - hankin_square_two_point::THETA_K_DEG)).abs() < 1e-12);
assert!((hankin_4_8_8_rosette_transform::THETA_E_DEG
            - (90.0 - hankin_4_8_8_rosette_transform::THETA_K_DEG))
            .abs()
            < 1e-12);
        assert!((hankin_3_6_3_6::THETA_E_DEG - (90.0 - hankin_3_6_3_6::THETA_K_DEG)).abs() < 1e-12);
        assert!((hankin_3_4_6_4::THETA_E_DEG - (90.0 - hankin_3_4_6_4::THETA_K_DEG)).abs() < 1e-12);
        assert!((hankin_3_3_3_4_4::THETA_E_DEG - (90.0 - hankin_3_3_3_4_4::THETA_K_DEG)).abs() < 1e-12);
        assert!((hankin_3_3_4_3_4::THETA_E_DEG - (90.0 - hankin_3_3_4_3_4::THETA_K_DEG)).abs() < 1e-12);
    }
}


/// M94 Hankin tiling tests: registry-level invariants for the three
/// configurations (cell census, seed orbits, area identity, symmetry
/// metadata, coverage).
#[cfg(test)]
mod hankin_4_8_8_tests {
    use super::hankin_4_8_8::hankin_4_8_8_rosette;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_4_8_8_covers_region() {
        check_covers(
            &hankin_4_8_8_rosette(),
            "hankin-4-8-8-rosette",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_4_8_8_rotated_scaled() {
        check_rotated(
            &hankin_4_8_8_rosette(),
            "hankin-4-8-8-rosette(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_4_8_8_deterministic() {
        check_det(
            &hankin_4_8_8_rosette(),
            "hankin-4-8-8-rosette",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_4_8_8_unit_structure() {
        let t = hankin_4_8_8_rosette();
        assert_eq!(t.cells.len(), 14);
        // Machine-transcribed seed layout: 4 orbit classes in canonical
        // face order.
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(
            seeds,
            crate::hankin_4_8_8::EXPECTED_SEEDS.to_vec(),
            "seed layout"
        );
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        let a = 1.0 + core::f64::consts::SQRT_2;
        assert!((det - a * a).abs() < 1e-9);
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_4_8_8_symmetry_metadata() {
        let t = hankin_4_8_8_rosette();
        assert_eq!(t.id, "hankin-4-8-8-rosette");
        assert_eq!(t.group, WallpaperGroup::P4m);
        assert_eq!(t.lattice_kind, LatticeKind::Square);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}

#[cfg(test)]
mod hankin_4_6_12_tests {
    use super::hankin_4_6_12::hankin_4_6_12;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_4_6_12_covers_region() {
        check_covers(
            &hankin_4_6_12(),
            "hankin-4-6-12",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_4_6_12_rotated_scaled() {
        check_rotated(
            &hankin_4_6_12(),
            "hankin-4-6-12(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_4_6_12_deterministic() {
        check_det(
            &hankin_4_6_12(),
            "hankin-4-6-12",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_4_6_12_unit_structure() {
        let t = hankin_4_6_12();
        assert_eq!(t.cells.len(), 42, "13 + 3×5 + 2×7 faces per lattice cell");
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(
            seeds,
            crate::hankin_4_6_12::EXPECTED_SEEDS.to_vec(),
            "seed layout"
        );
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        // √3 as a literal: std's SQRT_3 constant is still unstable.
        const RT3: f64 = 1.732_050_807_568_877_2;
        let s = 3.0 + RT3;
        assert!((det - s * s * RT3 / 2.0).abs() < 1e-9);
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_4_6_12_symmetry_metadata() {
        let t = hankin_4_6_12();
        assert_eq!(t.id, "hankin-4-6-12");
        assert_eq!(t.group, WallpaperGroup::P6m);
        assert_eq!(t.lattice_kind, LatticeKind::Hexagonal);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}

#[cfg(test)]
mod hankin_hex_tests {
    use super::hankin_hex::hankin_hex_two_point;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_hex_covers_region() {
        check_covers(
            &hankin_hex_two_point(),
            "hankin-hex-two-point",
            BBox::new(Vec2::new(-16.0, -14.0), Vec2::new(16.0, 14.0)),
        );
    }

    #[test]
    fn hankin_hex_rotated_scaled() {
        check_rotated(
            &hankin_hex_two_point(),
            "hankin-hex-two-point(rot20,scale0.6)",
            BBox::new(Vec2::new(-16.0, -14.0), Vec2::new(16.0, 14.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_hex_deterministic() {
        check_det(
            &hankin_hex_two_point(),
            "hankin-hex-two-point",
            BBox::new(Vec2::new(-8.0, -7.0), Vec2::new(8.0, 7.0)),
        );
    }

    #[test]
    fn hankin_hex_unit_structure() {
        let t = hankin_hex_two_point();
        assert_eq!(t.cells.len(), 13, "12 motif faces + 1 centre dodecagon");
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(
            seeds,
            crate::hankin_hex::EXPECTED_SEEDS.to_vec(),
            "seed layout"
        );
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        assert!((det - 3.0f64.sqrt() / 2.0 * 3.0).abs() < 1e-9, "det = |u1×u2| = 3√3/2");
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_hex_symmetry_metadata() {
        let t = hankin_hex_two_point();
        assert_eq!(t.id, "hankin-hex-two-point");
        assert_eq!(t.group, WallpaperGroup::P6m);
        assert_eq!(t.lattice_kind, LatticeKind::Hexagonal);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}


/// M99 Hankin twelvefold khatam tests: registry-level invariants for the
/// 3.12.12 configuration (cell census, seed orbits, area identity,
/// symmetry metadata, coverage).
#[cfg(test)]
mod hankin_3_12_12_tests {
    use super::hankin_3_12_12::hankin_3_12_12;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_3_12_12_covers_region() {
        check_covers(
            &hankin_3_12_12(),
            "hankin-3-12-12",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_3_12_12_rotated_scaled() {
        check_rotated(
            &hankin_3_12_12(),
            "hankin-3-12-12(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_3_12_12_deterministic() {
        check_det(
            &hankin_3_12_12(),
            "hankin-3-12-12",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_3_12_12_unit_structure() {
        let t = hankin_3_12_12();
        assert_eq!(t.cells.len(), 21, "13 + 4 + 4 faces per lattice cell");
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(
            seeds,
            crate::hankin_3_12_12::EXPECTED_SEEDS.to_vec(),
            "seed layout"
        );
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        // √3 as a literal: std's SQRT_3 constant is still unstable.
        const RT3: f64 = 1.732_050_807_568_877_2;
        assert!((det - (6.0 + 7.0 * RT3 / 2.0)).abs() < 1e-9);
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_3_12_12_symmetry_metadata() {
        let t = hankin_3_12_12();
        assert_eq!(t.id, "hankin-3-12-12");
        assert_eq!(t.group, WallpaperGroup::P6m);
        assert_eq!(t.lattice_kind, LatticeKind::Hexagonal);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}


/// M99 hankin-square-two-point tests: registry-level invariants
/// (cell census, seed orbits, area identity, symmetry metadata,
/// coverage).
#[cfg(test)]
mod hankin_square_two_point_tests {
    use super::hankin_square_two_point::hankin_square_two_point;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_square_two_point_covers_region() {
        check_covers(
            &hankin_square_two_point(),
            "hankin-square-two-point",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_square_two_point_rotated_scaled() {
        check_rotated(
            &hankin_square_two_point(),
            "hankin-square-two-point(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_square_two_point_deterministic() {
        check_det(
            &hankin_square_two_point(),
            "hankin-square-two-point",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_square_two_point_unit_structure() {
        let t = hankin_square_two_point();
        assert_eq!(t.cells.len(), 7, "4 + 2 + 1 faces per lattice cell");
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(
            seeds,
            crate::hankin_square_two_point::EXPECTED_SEEDS.to_vec(),
            "seed layout"
        );
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        assert!((det - 1.0).abs() < 1e-9, "unit square lattice det");
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_square_two_point_symmetry_metadata() {
        let t = hankin_square_two_point();
        assert_eq!(t.id, "hankin-square-two-point");
        assert_eq!(t.group, WallpaperGroup::Cmm);
        assert_eq!(t.lattice_kind, LatticeKind::CenteredRectangular);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}


/// M100 hankin-3-6-3-6 tests: registry-level invariants (cell census,
/// seed orbits, area identity, symmetry metadata, coverage).
#[cfg(test)]
mod hankin_3_6_3_6_tests {
    use super::hankin_3_6_3_6::hankin_3_6_3_6;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_3_6_3_6_covers_region() {
        check_covers(
            &hankin_3_6_3_6(),
            "hankin-3-6-3-6",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_3_6_3_6_rotated_scaled() {
        check_rotated(
            &hankin_3_6_3_6(),
            "hankin-3-6-3-6(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_3_6_3_6_deterministic() {
        check_det(
            &hankin_3_6_3_6(),
            "hankin-3-6-3-6",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_3_6_3_6_unit_structure() {
        let t = hankin_3_6_3_6();
        assert_eq!(t.cells.len(), 9, "7 + 1 + 1 faces per lattice cell");
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(
            seeds,
            crate::hankin_3_6_3_6::EXPECTED_SEEDS.to_vec(),
            "seed layout"
        );
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        // √3 as a literal: std's SQRT_3 constant is still unstable.
        const RT3: f64 = 1.732_050_807_568_877_2;
        assert!((det - 2.0 * RT3).abs() < 1e-9, "per-cell area 2√3");
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_3_6_3_6_symmetry_metadata() {
        let t = hankin_3_6_3_6();
        assert_eq!(t.id, "hankin-3-6-3-6");
        assert_eq!(t.group, WallpaperGroup::P6m);
        assert_eq!(t.lattice_kind, LatticeKind::Hexagonal);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}


/// M100 hankin-3-4-6-4 tests: registry-level invariants (cell census,
/// seed orbits, area identity, symmetry metadata, coverage).
#[cfg(test)]
mod hankin_3_4_6_4_tests {
    use super::hankin_3_4_6_4::hankin_3_4_6_4;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_3_4_6_4_covers_region() {
        check_covers(
            &hankin_3_4_6_4(),
            "hankin-3-4-6-4",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_3_4_6_4_rotated_scaled() {
        check_rotated(
            &hankin_3_4_6_4(),
            "hankin-3-4-6-4(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_3_4_6_4_deterministic() {
        check_det(
            &hankin_3_4_6_4(),
            "hankin-3-4-6-4",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_3_4_6_4_unit_structure() {
        let t = hankin_3_4_6_4();
        assert_eq!(t.cells.len(), 18, "7 + 3 + 8 faces per lattice cell");
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(
            seeds,
            crate::hankin_3_4_6_4::EXPECTED_SEEDS.to_vec(),
            "seed layout"
        );
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        // √3 as a literal: std's SQRT_3 constant is still unstable.
        const RT3: f64 = 1.732_050_807_568_877_2;
        assert!((det - (2.0 * RT3 + 3.0)).abs() < 1e-9, "per-cell area 4√3/2 + 3");
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_3_4_6_4_symmetry_metadata() {
        let t = hankin_3_4_6_4();
        assert_eq!(t.id, "hankin-3-4-6-4");
        assert_eq!(t.group, WallpaperGroup::P6m);
        assert_eq!(t.lattice_kind, LatticeKind::Hexagonal);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}


/// M107 hankin-3-3-3-4-4 tests: registry-level invariants (cell census,
/// seed orbits, area identity, symmetry metadata, coverage).
#[cfg(test)]
mod hankin_3_3_3_4_4_tests {
    use super::hankin_3_3_3_4_4::hankin_3_3_3_4_4;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_3_3_3_4_4_covers_region() {
        check_covers(
            &hankin_3_3_3_4_4(),
            "hankin-3-3-3-4-4",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_3_3_3_4_4_rotated_scaled() {
        check_rotated(
            &hankin_3_3_3_4_4(),
            "hankin-3-3-3-4-4(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_3_3_3_4_4_deterministic() {
        check_det(
            &hankin_3_3_3_4_4(),
            "hankin-3-3-3-4-4",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_3_3_3_4_4_unit_structure() {
        let t = hankin_3_3_3_4_4();
        assert_eq!(t.cells.len(), 13, "4 + 4 + 5 faces per lattice cell");
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(
            seeds,
            crate::hankin_3_3_3_4_4::EXPECTED_SEEDS.to_vec(),
            "seed layout"
        );
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        // √3 as a literal: std's SQRT_3 constant is still unstable.
        const RT3: f64 = 1.732_050_807_568_877_2;
        assert!((det - (1.0 + 0.5 * RT3)).abs() < 1e-9, "per-cell area 1 + √3/2");
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_3_3_3_4_4_symmetry_metadata() {
        let t = hankin_3_3_3_4_4();
        assert_eq!(t.id, "hankin-3-3-3-4-4");
        assert_eq!(t.group, WallpaperGroup::Cmm);
        assert_eq!(t.lattice_kind, LatticeKind::CenteredRectangular);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}


/// M111 hankin-3-3-4-3-4 tests: registry-level invariants (cell census,
/// seed orbits, area identity, symmetry metadata, coverage).
#[cfg(test)]
mod hankin_3_3_4_3_4_tests {
    use super::hankin_3_3_4_3_4::hankin_3_3_4_3_4;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_3_3_4_3_4_covers_region() {
        check_covers(
            &hankin_3_3_4_3_4(),
            "hankin-3-3-4-3-4",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_3_3_4_3_4_rotated_scaled() {
        check_rotated(
            &hankin_3_3_4_3_4(),
            "hankin-3-3-4-3-4(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_3_3_4_3_4_deterministic() {
        check_det(
            &hankin_3_3_4_3_4(),
            "hankin-3-3-4-3-4",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_3_3_4_3_4_unit_structure() {
        let t = hankin_3_3_4_3_4();
        assert_eq!(t.cells.len(), 52, "4×5 + 8×4 faces per lattice cell");
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(
            seeds,
            crate::hankin_3_3_4_3_4::EXPECTED_SEEDS.to_vec(),
            "seed layout"
        );
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        // √3 as a literal: std's SQRT_3 constant is still unstable.
        const RT3: f64 = 1.732_050_807_568_877_2;
        assert!((det - (4.0 + 2.0 * RT3)).abs() < 1e-9, "per-cell area (1 + √3)²");
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_3_3_4_3_4_symmetry_metadata() {
        let t = hankin_3_3_4_3_4();
        assert_eq!(t.id, "hankin-3-3-4-3-4");
        assert_eq!(t.group, WallpaperGroup::P4g);
        assert_eq!(t.lattice_kind, LatticeKind::Square);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}


// ---------------------------------------------------------------------
// M99: rosette transform (Kaplan dissertation §3.8) — the r′ formula,
// the two per-tile rules on the 4.8.8 substrate shapes, and the
// edge-to-edge / exactly-once invariants of the transformed table
// (the dual.rs house patterns, applied to the pre-inference template).
// ---------------------------------------------------------------------
#[cfg(test)]
mod rosette_transform_tests {
    use super::hankin_4_8_8_rosette_transform::{transformed_cells, LATTICE, TILE_0, TILE_1};
    use super::rosette_transform::{self, Rule};
    use crate::lattice::{similarity_map, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};
    use std::collections::HashMap;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::ParamSet;
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};
    use tess_core::view::ColorMode;
    use tess_core::TilingCategory;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Absolute shoelace area (house pattern, as in `dual::signed_area`).
    fn area(pts: &[Vec2]) -> f64 {
        let mut s = 0.0;
        for i in 0..pts.len() {
            let (a, b) = (pts[i], pts[(i + 1) % pts.len()]);
            s += a.x * b.y - b.x * a.y;
        }
        s.abs() / 2.0
    }

    /// Test-view lattice tiling of the transformed table: the edge
    /// census and exactly-once checks run through the Tiling trait.
    fn transformed_table() -> LatticeTiling {
        let (cells, _rep) = transformed_cells();
        LatticeTiling {
            id: "rosette-transformed-4-8-8",
            name: "rosette-transformed 4.8.8 (test view)",
            description: "test view of the M99 transformed substrate",
            lattice: LATTICE,
            cells,
            offset: Vec2::ZERO,
            group: WallpaperGroup::P4m,
            lattice_kind: LatticeKind::Square,
            category: TilingCategory::Islamic,
            secondary: &[],
            seed_pattern: SeedPattern::Uniform,
            params: &SIMILARITY_PARAMS,
            make_map: similarity_map,
            color_mode: ColorMode::Pattern,
        }
    }

    #[test]
    fn inner_radius_forms_agree_and_spoke_is_half_side() {
        // The two algebraic forms of the dissertation radius agree to
        // machine precision, and the spoke = half inner side identity
        // (the Rule-1 defining property) holds for the whole range.
        for n in 5usize..=12 {
            let a = rosette_transform::inner_radius(n, 1.0);
            let b = rosette_transform::inner_radius_closed(n, 1.0);
            assert!(
                (a - b).abs() <= 1e-12,
                "n={n}: dissertation form {a} vs closed form {b}"
            );
            let pi_n = std::f64::consts::PI / n as f64;
            let spoke = pi_n.cos() - a; // apothem (r=1) minus r′
            let half_side = a * pi_n.sin(); // s′/2 = r′·sin(π/n)
            assert!(
                (spoke - half_side).abs() <= 1e-12,
                "n={n}: spoke {spoke} vs half inner side {half_side}"
            );
        }
        // Anchor values (r = 1) at the ends of the range.
        assert!(
            (rosette_transform::inner_radius(5, 1.0) - 0.5095254494944288).abs() < 1e-12,
            "n=5 anchor"
        );
        assert!(
            (rosette_transform::inner_radius(12, 1.0) - 0.7673269879789604).abs() < 1e-12,
            "n=12 anchor"
        );
    }

    #[test]
    fn octagon_maps_to_inner_octagon_and_8_pentagons() {
        let (faces, rep) = rosette_transform::transform_tile(&TILE_0);
        assert_eq!(rep.rule, Rule::RegularPolygon);
        assert_eq!(rep.n, 8);
        assert_eq!(faces.len(), 9, "inner octagon + 8 corner pentagons");
        let n5 = faces.iter().filter(|f| f.len() == 5).count();
        let n8 = faces.iter().filter(|f| f.len() == 8).count();
        assert_eq!((n5, n8), (8, 1), "face census");
        let sum: f64 = faces.iter().map(|f| area(f)).sum();
        assert!(
            (sum - rep.tile_area).abs() < 1e-9,
            "area closure {sum} vs {}",
            rep.tile_area
        );
        // Pentagon side multiset: two spokes (s′/2), two half outer
        // edges (the substrate octagon has edge 1 → ½), one inner edge
        // (s′) — all derived from the dissertation formula.
        let mut acc = Vec2::ZERO;
        for p in TILE_0 {
            acc += p;
        }
        let c = acc / 8.0;
        let r = (TILE_0[0] - c).length();
        let pi8 = std::f64::consts::PI / 8.0;
        let rp = rosette_transform::inner_radius(8, r);
        let half_side = rp * pi8.sin(); // s′/2
        let s_prime = 2.0 * half_side;
        let mut want = [half_side, half_side, 0.5, 0.5, s_prime];
        want.sort_by(f64::total_cmp);
        for f in faces.iter().filter(|f| f.len() == 5) {
            let mut got: Vec<f64> = (0..5)
                .map(|k| (f[(k + 1) % 5] - f[k]).length())
                .collect();
            got.sort_by(f64::total_cmp);
            for (g, w) in got.iter().zip(want.iter()) {
                assert!((g - w).abs() < 1e-9, "pentagon side {g} vs {w}");
            }
        }
        // The inner face is the regular octagon of circumradius r′,
        // concentric with the substrate octagon.
        let inner = faces.iter().find(|f| f.len() == 8).unwrap();
        for &p in inner {
            assert!(
                ((p - c).length() - rp).abs() < 1e-9,
                "inner octagon vertex off radius: |{p} − c| vs r′"
            );
        }
    }

    #[test]
    fn square_maps_to_4_half_squares_centre_clustered() {
        // The n = 4 substrate tile falls to the bisector rule: the six
        // pairwise bisector intersections collapse to a single cluster
        // (all four meeting at the centre) and the map is four small
        // squares of side 1/2.
        let (faces, rep) = rosette_transform::transform_tile(&TILE_1);
        assert_eq!(rep.rule, Rule::Bisectors);
        assert_eq!(rep.n, 4);
        assert_eq!(faces.len(), 4, "four small squares");
        assert_eq!(
            rep.intersections,
            4,
            "4 of the 6 bisector pairs meet inside (opposite-side pairs are parallel)"
        );
        assert_eq!(rep.clusters, 1, "all 4 intersections cluster at the centre");
        assert!(rep.clustered, "Kaplan averaging heuristic must fire");
        let sum: f64 = faces.iter().map(|f| area(f)).sum();
        assert!(
            (sum - rep.tile_area).abs() < 1e-9,
            "area closure {sum} vs {}",
            rep.tile_area
        );
        // The only geometric points in the map: the centre, the 4
        // corners, and the 4 edge midpoints — every face vertex must
        // come from that set, and every point of the set must be used.
        let mut pts9: Vec<Vec2> = vec![Vec2::ZERO];
        for i in 0..4 {
            pts9.push(TILE_1[i]);
            pts9.push((TILE_1[i] + TILE_1[(i + 1) % 4]) / 2.0);
        }
        let mut used9 = [false; 9];
        for f in &faces {
            assert_eq!(f.len(), 4, "each face is a quadrilateral");
            for k in 0..4 {
                let e1 = f[(k + 1) % 4] - f[k];
                assert!((e1.length() - 0.5).abs() < 1e-9, "small square side");
                let e2 = f[(k + 2) % 4] - f[(k + 1) % 4];
                assert!(e1.dot(e2).abs() < 1e-9, "small square right angle");
            }
            for &p in f {
                let idx = pts9
                    .iter()
                    .position(|q| (*q - p).length() < 1e-9)
                    .unwrap_or_else(|| panic!("face vertex {p} not in the 9-point set"));
                used9[idx] = true;
            }
        }
        assert!(
            used9.iter().all(|&u| u),
            "every point of the 9-point set is a face vertex"
        );
    }

    #[test]
    fn transformed_488_area_census() {
        let (cells, rep) = transformed_cells();
        assert_eq!(cells.len(), 13, "1 oct + 8 pent + 4 sq per lattice cell");
        assert_eq!(rep.per_tile.len(), 2, "one report per substrate tile");
        for t in &rep.per_tile {
            assert!((t.area_sum - t.tile_area).abs() < 1e-9, "per-tile closure");
        }
        let a = 1.0 + core::f64::consts::SQRT_2;
        assert!((rep.lattice_area - a * a).abs() < 1e-12, "|det| = (1+√2)²");
        let sum: f64 = cells.iter().map(|c| area(&c.poly.pts)).sum();
        assert!(
            (sum - rep.lattice_area).abs() < 1e-9,
            "Σ face areas {sum} vs |det| {}",
            rep.lattice_area
        );
    }

    // ------------------------------------------------------------------
    // Edge census helpers (duplicates of the dual.rs test helpers —
    // those are private to the dual test module).
    // ------------------------------------------------------------------

    /// Inverse of the lattice matrix A = [a1 a2] (basis as columns).
    struct LatticeInv {
        m: [[f64; 2]; 2],
    }

    impl LatticeInv {
        fn new(a1: Vec2, a2: Vec2) -> Option<Self> {
            let det = a1.cross(a2);
            if det.abs() < 1e-12 {
                return None;
            }
            let d = 1.0 / det;
            Some(Self {
                m: [
                    [a2.y * d, -a2.x * d],
                    [-a1.y * d, a1.x * d],
                ],
            })
        }

        /// Lattice (u-space) coordinates of a cell-space point.
        fn apply(&self, p: Vec2) -> Vec2 {
            Vec2::new(
                self.m[0][0] * p.x + self.m[0][1] * p.y,
                self.m[1][0] * p.x + self.m[1][1] * p.y,
            )
        }
    }

    /// Undirected edge key on the 1e-6 grid (AGENTS.md edge-parity
    /// convention; shared edges between lattice translates agree to ~1
    /// ulp).
    fn edge_key(a: Vec2, b: Vec2) -> [i64; 4] {
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
    }

    /// The partner table edge for every table edge of `t`: for each
    /// edge of cell `ci`, the unique `(cell, lattice offset)` whose
    /// drawn edge is a lattice translate of it. Panics if any edge has
    /// a different number of partners than one (a table that is not
    /// edge-to-edge at the period level). Result is indexed per cell:
    /// `partners[ci][e]`.
    fn edge_partners(t: &LatticeTiling) -> Vec<Vec<(usize, i64, i64)>> {
        let (a1, a2) = (t.lattice[0], t.lattice[1]);
        let inv = LatticeInv::new(a1, a2).expect("non-degenerate lattice");
        let n = t.cells.len();
        let mut partners = Vec::new();
        for ci in 0..n {
            let pts = &t.cells[ci].poly.pts;
            let m = pts.len();
            let mut row = Vec::new();
            for e in 0..m {
                let p = pts[e];
                let q = pts[(e + 1) % m];
                let mut found: Vec<(usize, i64, i64)> = Vec::new();
                for cj in 0..n {
                    let p2 = &t.cells[cj].poly.pts;
                    let m2 = p2.len();
                    for f in 0..m2 {
                        if cj == ci && f == e {
                            continue;
                        }
                        for (pa, pb) in [
                            (p2[f], p2[(f + 1) % m2]),
                            (p2[(f + 1) % m2], p2[f]),
                        ] {
                            for (ta, tb) in [(p, q), (q, p)] {
                                let s = pa - ta;
                                let u = inv.apply(s);
                                if (u.x - u.x.round()).abs() > 1e-6
                                    || (u.y - u.y.round()).abs() > 1e-6
                                {
                                    continue;
                                }
                                if (pb - tb - s).length() > 1e-9 {
                                    continue;
                                }
                                found.push((cj, u.x.round() as i64, u.y.round() as i64));
                                break;
                            }
                        }
                    }
                }
                // Dedup: the same partner edge is one solution, but a
                // non-edge-to-edge table could carry several.
                found.sort();
                found.dedup();
                assert_eq!(
                    found.len(),
                    1,
                    "table edge {} of cell {} has {} partners",
                    e,
                    ci,
                    found.len()
                );
                row.push(found[0]);
            }
            partners.push(row);
        }
        partners
    }

    #[test]
    fn transformed_488_is_edge_to_edge() {
        let t = transformed_table();
        // Period level: every table edge has exactly one partner table
        // edge (0 unpaired edges at the period level).
        let partners = edge_partners(&t);
        let (a1, a2) = (t.lattice[0], t.lattice[1]);
        // Tile the window and count each edge by rounded endpoints:
        // interior edges exactly twice (0 T-junctions → n ≤ 2), the
        // partner's window membership decides the expected count.
        let w = 4i64;
        let mut counts: HashMap<[i64; 4], (usize, bool)> = HashMap::new();
        for i in -w..=w {
            for j in -w..=w {
                let tr = a1 * i as f64 + a2 * j as f64;
                for (ci, cell) in t.cells.iter().enumerate() {
                    let pts = &cell.poly.pts;
                    for e in 0..pts.len() {
                        let pa = pts[e] + tr;
                        let pb = pts[(e + 1) % pts.len()] + tr;
                        let (_pc, poi, pojj) = partners[ci][e];
                        let pi = i - poi;
                        let pj = j - pojj;
                        let inwin = (-w..=w).contains(&pi) && (-w..=w).contains(&pj);
                        let entry = counts.entry(edge_key(pa, pb)).or_insert((0, false));
                        entry.0 += 1;
                        entry.1 |= inwin;
                    }
                }
            }
        }
        for (k, (n, inwin)) in &counts {
            assert!(
                *n <= 2,
                "edge {k:?} shared by {n} tiles (T-junction?)"
            );
            if *inwin {
                assert_eq!(*n, 2, "unpaired interior edge {k:?}");
            } else {
                assert_eq!(
                    *n, 1,
                    "edge {k:?} appears {n} times though its partner lies outside the window"
                );
            }
        }
    }

    #[test]
    fn transformed_488_fine_grid_exactly_once() {
        let t = transformed_table();
        let region = BBox::new(v2(-6.5, -6.5), v2(6.5, 6.5));
        let step = 0.25;
        let nsteps = (region.size().x / step) as usize;
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "{:?}", r.message);
        let tiles = &r.tiles;
        for gy in 0..nsteps {
            for gx in 0..nsteps {
                let pt = region.min
                    + v2(step * (gx as f64 + 0.5), step * (gy as f64 + 0.5));
                let cnt = tiles.iter().filter(|t| t.poly.contains(pt)).count();
                assert!(cnt >= 1, "gap at {pt}");
                if cnt == 2 {
                    // Must be a point on the shared edge of the two tiles.
                    let idx: Vec<usize> = tiles
                        .iter()
                        .enumerate()
                        .filter(|(_, t)| t.poly.contains(pt))
                        .map(|(i, _)| i)
                        .collect();
                    let a = &tiles[idx[0]].poly;
                    let b = &tiles[idx[1]].poly;
                    let mut on_shared = false;
                    for e0 in 0..a.pts.len() {
                        let p0 = a.pts[e0];
                        let p1 = a.pts[(e0 + 1) % a.pts.len()];
                        let k = edge_key(p0, p1);
                        for e1 in 0..b.pts.len() {
                            if edge_key(b.pts[e1], b.pts[(e1 + 1) % b.pts.len()]) == k {
                                let ab = p1 - p0;
                                let tt =
                                    ((pt - p0).dot(ab) / ab.dot(ab)).clamp(0.0, 1.0);
                                on_shared = (p0 + ab * tt - pt).length() <= 1e-6;
                                break;
                            }
                        }
                        if on_shared {
                            break;
                        }
                    }
                    assert!(
                        on_shared,
                        "double-covered point {pt} not on a shared edge"
                    );
                } else if cnt > 2 {
                    let near = tiles
                        .iter()
                        .filter(|t| t.poly.pts.iter().any(|&v| (v - pt).length() <= 1e-6))
                        .count();
                    assert_eq!(
                        near, cnt,
                        "vertex fan count {near} != coverage {cnt} at {pt}"
                    );
                }
            }
        }
        // Clipped area closes against the region area.
        let mut area = 0.0;
        for t in tiles {
            if let Some(cl) = t.poly.clipped_to(&region) {
                area += cl.area();
            }
        }
        let ra = region.area();
        assert!(
            (area - ra).abs() <= 1e-6 * ra,
            "clipped area {area} vs region {ra} (rel {})",
            (area - ra) / ra
        );
    }
}


// ---------------------------------------------------------------------
// M99: hankin-4-8-8-rosette-transform — registry checks, inference
// census, and the p4m cell-table probe.
// ---------------------------------------------------------------------
#[cfg(test)]
mod hankin_4_8_8_rosette_transform_tests {
    use super::hankin;
    use super::hankin_4_8_8_rosette_transform::{
        hankin_4_8_8_rosette_transform, transformed_cells, DELTA, EXPECTED_LENS,
        EXPECTED_SEEDS, LATTICE, THETA_E_DEG,
    };
    use crate::tests::{check_covers, check_det, check_rotated};
    use std::collections::HashMap;
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_4_8_8_rosette_transform_covers_region() {
        check_covers(
            &hankin_4_8_8_rosette_transform(),
            "hankin-4-8-8-rosette-transform",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_4_8_8_rosette_transform_rotated_scaled() {
        check_rotated(
            &hankin_4_8_8_rosette_transform(),
            "hankin-4-8-8-rosette-transform(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_4_8_8_rosette_transform_deterministic() {
        check_det(
            &hankin_4_8_8_rosette_transform(),
            "hankin-4-8-8-rosette-transform",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_4_8_8_rosette_transform_unit_structure() {
        let t = hankin_4_8_8_rosette_transform();
        assert_eq!(
            t.cells.len(),
            93,
            "16×3 + 36×4 + 32×5 + 8×6 + 1×16 faces per lattice cell"
        );
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, EXPECTED_SEEDS.to_vec(), "seed layout");
        let lens: Vec<usize> = t.cells.iter().map(|c| c.poly.pts.len()).collect();
        assert_eq!(lens, EXPECTED_LENS.to_vec(), "face census layout");
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        let a = 1.0 + core::f64::consts::SQRT_2;
        assert!((det - a * a).abs() < 1e-9, "det = (1+√2)²");
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_4_8_8_rosette_transform_symmetry_metadata() {
        let t = hankin_4_8_8_rosette_transform();
        assert_eq!(t.id, "hankin-4-8-8-rosette-transform");
        assert_eq!(t.group, WallpaperGroup::P4m);
        assert_eq!(t.lattice_kind, LatticeKind::Square);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }

    #[test]
    fn hankin_4_8_8_rosette_transform_inference_census() {
        let (sub, _trep) = transformed_cells();
        let tiles: Vec<&[Vec2]> = sub.iter().map(|c| c.poly.pts.as_slice()).collect();
        let (cells, report) = hankin::build_cells(&tiles, THETA_E_DEG, DELTA);
        assert!(report.complete, "pairing must be complete at the selected θK");
        let lens: Vec<usize> = report.per_tile.iter().map(|r| r.faces).collect();
        // 8 pentagons → 8 faces each, inner octagon → 9, 4 small
        // squares → 5 each (machine-transcribed order).
        assert_eq!(
            lens,
            vec![8, 8, 8, 8, 8, 8, 8, 8, 9, 5, 5, 5, 5],
            "face census per transformed tile"
        );
        for r in &report.per_tile {
            assert!((r.area_sum - r.tile_area).abs() < 1e-9, "per-tile closure");
            assert_eq!(r.rays, r.pairs * 2, "every ray paired exactly once");
        }
        assert_eq!(cells.len(), 93, "inference table size");
        let a = 1.0 + core::f64::consts::SQRT_2;
        let sum: f64 = cells.iter().map(|c| c.poly.area().abs()).sum();
        assert!((sum - a * a).abs() < 1e-9, "Σ inference areas {sum} vs (1+√2)²");
    }

    /// Canonical face key: CCW (flipped images reversed), rotated to
    /// the lexicographically smallest vertex, rounded to a 1e-10 grid.
    fn canon_pts(pts: &[Vec2], flip: bool) -> Vec<Vec2> {
        let mut v: Vec<Vec2> = if flip {
            pts.iter().rev().copied().collect()
        } else {
            pts.to_vec()
        };
        let mut k = 0;
        for i in 1..v.len() {
            let dx = v[i].x - v[k].x;
            let dy = v[i].y - v[k].y;
            if dx < -1e-12 || (dx.abs() <= 1e-12 && dy < -1e-12) {
                k = i;
            }
        }
        v.rotate_left(k);
        v
    }

    fn key_of(v: &[Vec2]) -> (usize, Vec<i64>) {
        let mut k = Vec::with_capacity(v.len() * 2);
        for p in v {
            k.push((p.x * 1e10).round() as i64);
            k.push((p.y * 1e10).round() as i64);
        }
        (v.len(), k)
    }

    /// Multiset invariance of the cell table under a point symmetry
    /// `g` of the infinite tiling: the image of every table cell (up to
    /// a lattice translation and, for mirrors, orientation reversal)
    /// must coincide with a unique, still-unmatched table cell to
    /// 1e-9. Returns the worst vertex mismatch.
    fn assert_p4m_exact(label: &str, flip: bool, g: impl Fn(Vec2) -> Vec2) -> f64 {
        let t = hankin_4_8_8_rosette_transform();
        let (a1, a2) = (LATTICE[0], LATTICE[1]);
        let mut unused: HashMap<(usize, Vec<i64>), Vec<usize>> = HashMap::new();
        for (j, c) in t.cells.iter().enumerate() {
            let cp = canon_pts(&c.poly.pts, false);
            unused.entry(key_of(&cp)).or_default().push(j);
        }
        let mut worst = 0.0f64;
        for (i, c) in t.cells.iter().enumerate() {
            let img: Vec<Vec2> = c.poly.pts.iter().map(|p| g(*p)).collect();
            let mut best: Option<(f64, usize)> = None;
            for ta in -1i64..=1 {
                for tb in -1i64..=1 {
                    let tr = a1 * ta as f64 + a2 * tb as f64;
                    let shifted: Vec<Vec2> = img.iter().map(|p| *p + tr).collect();
                    let cp = canon_pts(&shifted, flip);
                    let key = key_of(&cp);
                    if let Some(v) = unused.get(&key) {
                        if let Some(&j) = v.first() {
                            let cj = canon_pts(&t.cells[j].poly.pts, false);
                            let d = (0..cp.len())
                                .map(|k| (cp[k] - cj[k]).length())
                                .fold(0.0f64, f64::max);
                            if d <= 1e-9 && best.is_none_or(|(bd, _)| d < bd) {
                                best = Some((d, j));
                            }
                        }
                    }
                }
            }
            let (d, j) = best.unwrap_or_else(|| {
                panic!("{label}: image of table cell {i} matches no table cell")
            });
            if let Some(v) = unused.get_mut(&key_of(&canon_pts(&t.cells[j].poly.pts, false)))
            {
                v.retain(|&x| x != j);
            }
            worst = worst.max(d);
        }
        let residual: usize = unused.values().map(Vec::len).sum();
        assert_eq!(
            residual,
            0,
            "{label}: {residual} table cells unmatched by the point symmetry"
        );
        assert!(worst <= 1e-9, "{label}: worst mismatch {worst}");
        worst
    }

    #[test]
    fn hankin_4_8_8_rosette_transform_cell_table_is_p4m_exact() {
        let a = LATTICE[0].x; // 1 + √2, the period side
        let c = a / 2.0;
        let mut worst = 0.0f64;
        worst = worst.max(assert_p4m_exact(
            "R90 about cell centre",
            false,
            |p| Vec2::new(c - (p.y - c), c + (p.x - c)),
        ));
        worst = worst.max(assert_p4m_exact(
            "R180 about cell centre",
            false,
            |p| Vec2::new(2.0 * c - p.x, 2.0 * c - p.y),
        ));
        worst = worst.max(assert_p4m_exact(
            "R270 about cell centre",
            false,
            |p| Vec2::new(c + (p.y - c), c - (p.x - c)),
        ));
        worst = worst.max(assert_p4m_exact(
            "x-mirror (line x = A/2)",
            true,
            |p| Vec2::new(a - p.x, p.y),
        ));
        worst = worst.max(assert_p4m_exact(
            "y-mirror (line y = A/2)",
            true,
            |p| Vec2::new(p.x, a - p.y),
        ));
        worst = worst.max(assert_p4m_exact(
            "diagonal mirror y = x",
            true,
            |p| Vec2::new(p.y, p.x),
        ));
        worst = worst.max(assert_p4m_exact(
            "anti-diagonal mirror y = A − x",
            true,
            |p| Vec2::new(a - p.y, a - p.x),
        ));
        assert!(
            worst <= 1e-9,
            "p4m point group not exact on the cell table (worst {worst})"
        );
    }
}

#[cfg(test)]
mod najm_tests {
    use crate::tests::{check_covers, check_det, check_rotated};
    use crate::najm::{self, Axis, Diad, Feature, Inflation, NajmSpec};
    use tess_core::geometry::{BBox, Vec2};

    const PI: f64 = std::f64::consts::PI;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    fn spec_63() -> NajmSpec {
        NajmSpec {
            p: 6,
            q: 3,
            mult: [2, 0, 3],
            orient: [Feature::Edge, Feature::Vertex, Feature::Edge],
            inflation: Inflation::Pair(Diad::Ac),
        }
    }

    fn spec_44() -> NajmSpec {
        NajmSpec {
            p: 4,
            q: 4,
            mult: [3, 0, 0],
            orient: [Feature::Edge, Feature::Vertex, Feature::Vertex],
            inflation: Inflation::Order(&[najm::Step::Inflate(Axis::A)]),
        }
    }

    /// Signed interior angle (degrees, may be reflex) at vertex `i` of a
    /// CCW polygon.
    fn interior_angle_deg(pts: &[Vec2], i: usize) -> f64 {
        let n = pts.len();
        let (p0, p1, p2) = (pts[(i + n - 1) % n], pts[i], pts[(i + 1) % n]);
        let (u, w) = (p0 - p1, p2 - p1);
        let unsigned = u.cross(w).abs().atan2(u.dot(w)) * 180.0 / PI;
        if u.cross(w) < 0.0 {
            unsigned
        } else {
            360.0 - unsigned
        }
    }

    fn centroid(pts: &[Vec2]) -> Vec2 {
        let mut c = Vec2::ZERO;
        for &p in pts {
            c += p;
        }
        c * (1.0 / pts.len() as f64)
    }

    fn angles(pts: &[Vec2]) -> Vec<f64> {
        let mut a: Vec<f64> = (0..pts.len())
            .map(|i| interior_angle_deg(pts, i))
            .collect();
        a.sort_by(|x, y| x.partial_cmp(y).unwrap());
        a
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    #[test]
    fn radii_match_the_spike() {
        // 60-digit spike values (t22b_spike.py), case 3 on AC.
        let rr = spec_63().radii();
        assert_eq!(rr[1], 0.0);
        assert!((rr[0] - 0.5962939229190058).abs() < 1e-14, "r_A = {}", rr[0]);
        assert!((rr[2] - 0.45123723482970224).abs() < 1e-14, "r_C = {}", rr[2]);
        // Exact identities of the case-3 solution.
        let x = 2.0 * rr[0] * (PI / 12.0).sin();
        assert!((x - 2.0 * rr[2] * (PI / 9.0).sin()).abs() < 1e-14, "equal chords");
        assert!(
            (rr[0] * (PI / 12.0).cos() + rr[2] * (PI / 9.0).cos() - 1.0).abs() < 1e-14,
            "extents sum to |AC| = 1"
        );
        let closed = 0.5 * x * ((PI / 12.0).cos() / (PI / 12.0).sin()
            + (PI / 9.0).cos() / (PI / 9.0).sin());
        assert!((closed - 1.0).abs() < 1e-14, "x = 2/(cot 15° + cot 20°)");

        // Case 2 at unit |AC|: r_A = |AB|/cos 15° = cos 45°/cos 15°;
        // rescaled to |AB| = 1 this is 1/cos 15° = √6 − √2 (exact).
        let r44 = spec_44().radii()[0];
        assert!((r44 - (PI / 4.0).cos() / (PI / 12.0).cos()).abs() < 1e-14);
        let ra = r44 / (PI / 4.0).cos();
        assert!((ra - (6.0f64.sqrt() - 2.0f64.sqrt())).abs() < 1e-14);
        assert!((ra * (PI / 12.0).cos() - 1.0).abs() < 1e-14, "apothem = 1");
    }

    #[test]
    fn najm_63_covers_region() {
        let t = najm::najm_63_2e_0_3e_ac();
        check_covers(&t, "najm-63-2e-0-3e-ac", BBox::new(v2(-2.5, -2.0), v2(7.5, 5.5)));
    }

    #[test]
    fn najm_63_rotated_scaled() {
        let t = najm::najm_63_2e_0_3e_ac();
        check_rotated(
            &t,
            "najm-63-2e-0-3e-ac",
            BBox::new(v2(-3.0, -3.0), v2(6.0, 5.0)),
            1.5,
            37.0,
        );
    }

    #[test]
    fn najm_63_deterministic() {
        let t = najm::najm_63_2e_0_3e_ac();
        check_det(&t, "najm-63-2e-0-3e-ac", BBox::new(v2(-2.5, -2.0), v2(7.5, 5.5)));
    }

    #[test]
    fn najm_44_covers_region() {
        let t = najm::najm_44_3e_0_0_a();
        check_covers(&t, "najm-44-3e-0-0-a", BBox::new(v2(-2.0, -2.0), v2(8.0, 8.0)));
    }

    #[test]
    fn najm_44_rotated_scaled() {
        let t = najm::najm_44_3e_0_0_a();
        check_rotated(
            &t,
            "najm-44-3e-0-0-a",
            BBox::new(v2(-3.0, -3.0), v2(7.0, 7.0)),
            2.5,
            64.0,
        );
    }

    #[test]
    fn najm_44_deterministic() {
        let t = najm::najm_44_3e_0_0_a();
        check_det(&t, "najm-44-3e-0-0-a", BBox::new(v2(-2.0, -2.0), v2(8.0, 8.0)));
    }

    #[test]
    fn najm_63_tile_shapes_and_orientations() {
        let t = najm::najm_63_2e_0_3e_ac();
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 1, 1, 2, 2, 2]);
        let lens: Vec<usize> = t.cells.iter().map(|c| c.poly.pts.len()).collect();
        assert_eq!(lens, vec![12, 9, 9, 6, 6, 6]);

        let dod = &t.cells[0].poly.pts;
        let c = centroid(dod);
        let r = (dod[0] - c).length();
        let side = (dod[1] - dod[0]).length();
        for i in 0..12 {
            assert!(((dod[i] - c).length() - r).abs() < 1e-12);
            assert!(((dod[(i + 1) % 12] - dod[i]).length() - side).abs() < 1e-12);
            assert!(close(interior_angle_deg(dod, i), 150.0));
        }

        // Both enneagons regular (radius/side/140° angles), equal areas.
        for k in 1..=2 {
            let e = &t.cells[k].poly.pts;
            let c = centroid(e);
            let r = (e[0] - c).length();
            let side = (e[1] - e[0]).length();
            for i in 0..9 {
                assert!(((e[i] - c).length() - r).abs() < 1e-12);
                assert!(((e[(i + 1) % 9] - e[i]).length() - side).abs() < 1e-12);
                assert!(close(interior_angle_deg(e, i), 140.0));
            }
        }
        assert!((t.cells[1].poly.area() - t.cells[2].poly.area()).abs() < 1e-14);

        // Orientation witnesses: a vertex ray of enne0 points at the
        // 2-fold point (√3/2, 0) (m_C = 3 flips the CB edge to a vertex);
        // an edge-midpoint ray of enne1 points at its partner a1.
        let s3 = 3.0f64.sqrt();
        let e0 = &t.cells[1].poly.pts;
        let c0 = centroid(e0);
        assert!(
            e0.iter()
                .any(|v| ((*v - c0).y.atan2((*v - c0).x) + PI / 2.0).abs() < 1e-9),
            "no enne0 vertex on the ray toward the 2-fold point"
        );
        let e1 = &t.cells[2].poly.pts;
        let c1 = centroid(e1);
        assert!(
            (0..9).any(|i| {
                let m = (e1[i] + e1[(i + 1) % 9]) * 0.5 - c1;
                (m.y.atan2(m.x) + PI / 2.0).abs() < 1e-9
            }),
            "no enne1 edge-midpoint ray toward a1"
        );

        // 2-fold witness: rot180 about (√3/2, 0) maps enne0's vertices
        // exactly onto enne1's translated by −a2 (phases 190° + 180° =
        // 250° mod 40° — the classes' orientations differ).
        let b0 = v2(s3 / 2.0, 0.0);
        let a2 = v2(s3 / 2.0, 1.5);
        for v in e0 {
            let w = b0 * 2.0 - *v + a2;
            assert!(
                e1.iter().any(|u| (*u - w).length() < 1e-12),
                "rot180(B0) does not map enne0 onto enne1 − a2"
            );
        }

        // Bowties: equilateral with the common edge x, angles
        // {70, 70, 70, 70, 220, 220}, equal areas.
        let x = 2.0 * r * (PI / 12.0).sin();
        for k in 3..6 {
            let b = &t.cells[k].poly.pts;
            for i in 0..6 {
                assert!(((b[(i + 1) % 6] - b[i]).length() - x).abs() < 1e-12);
            }
            let angs = angles(b);
            for (&want, &got) in [70.0, 70.0, 70.0, 70.0, 220.0, 220.0].iter().zip(&angs) {
                assert!(close(want, got), "bowtie angles {angs:?}");
            }
            assert!((t.cells[k].poly.area() - t.cells[3].poly.area()).abs() < 1e-14);
        }
    }

    #[test]
    fn najm_44_tile_shapes() {
        let t = najm::najm_44_3e_0_0_a();
        assert_eq!(t.cells.len(), 2);
        let s = 4.0 - 2.0 * 3.0f64.sqrt();

        let dod = &t.cells[0].poly.pts;
        assert_eq!(dod.len(), 12);
        for i in 0..12 {
            assert!(((dod[(i + 1) % 12] - dod[i]).length() - s).abs() < 1e-13);
            assert!(close(interior_angle_deg(dod, i), 150.0));
        }
        // The ±15° vertices sit exactly on x = 1 (the chord through B).
        assert_eq!(dod.iter().filter(|p| (p.x - 1.0).abs() < 1e-12).count(), 2);

        let star = &t.cells[1].poly.pts;
        assert_eq!(star.len(), 8);
        let c = v2(1.0, 1.0);
        let td = 3.0f64.sqrt() - 1.0;
        let nqr = (2.0 - 3.0f64.sqrt()) * 2.0f64.sqrt();
        let mut tips = 0;
        let mut notches = 0;
        for i in 0..8 {
            assert!(((star[(i + 1) % 8] - star[i]).length() - s).abs() < 1e-13);
            let rad = (star[i] - c).length();
            let ang = interior_angle_deg(star, i);
            if (rad - td).abs() < 1e-12 {
                assert!(close(ang, 60.0), "star tip angle {ang}");
                tips += 1;
            } else if (rad - nqr).abs() < 1e-12 {
                assert!(close(ang, 210.0), "star notch angle {ang}");
                notches += 1;
            } else {
                panic!("star vertex at unexpected radius {rad}");
            }
        }
        assert_eq!((tips, notches), (4, 4));
    }

    #[test]
    fn area_census_matches_the_cell() {
        let t1 = najm::najm_63_2e_0_3e_ac();
        let det1 = t1.lattice[0].cross(t1.lattice[1]).abs();
        assert!((det1 - 3.0 * 3.0f64.sqrt() / 2.0).abs() < 1e-14);
        let rr = spec_63().radii();
        let a12 = t1.cells[0].poly.area();
        let a9 = t1.cells[1].poly.area();
        let abt = t1.cells[3].poly.area();
        assert!((a12 - 3.0 * rr[0] * rr[0]).abs() < 1e-13, "12-gon = 3 r_A²");
        assert!(
            (a9 - 4.5 * rr[2] * rr[2] * (2.0 * PI / 9.0).sin()).abs() < 1e-13,
            "9-gon = (9/2) r_C² sin 40°"
        );
        assert!(
            (a12 + 2.0 * a9 + 3.0 * abt - det1).abs() < 1e-13,
            "1·A12 + 2·A9 + 3·Abt = |det|"
        );

        let t2 = najm::najm_44_3e_0_0_a();
        let det2 = t2.lattice[0].cross(t2.lattice[1]).abs();
        assert!((det2 - 4.0).abs() < 1e-14);
        let s3 = 3.0f64.sqrt();
        let a12 = t2.cells[0].poly.area();
        let ast = t2.cells[1].poly.area();
        assert!((a12 - (24.0 - 12.0 * s3)).abs() < 1e-12, "24 − 12√3");
        assert!((ast - (12.0 * s3 - 20.0)).abs() < 1e-12, "12√3 − 20");
        assert!((a12 + ast - det2).abs() < 1e-12, "sum = |det| = 4");
    }

    #[test]
    fn translation_lattices_are_primitive() {
        // Each entry holds exactly one 12-gon class per cell (centered at
        // the origin), and no other class has 12 vertices. Any translation
        // symmetry must map the dodecagon orbit to itself, so the
        // translation group is contained in the orbit's center set;
        // generating a patch and checking that the emitted dodecagon
        // centers are *precisely* the declared lattice points witnesses
        // the declared basis is primitive (no denser lattice underlies
        // the tiling).
        use tess_core::params::ParamSet;
        use tess_core::tiling::{Query, Tiling};
        for t in [najm::najm_63_2e_0_3e_ac(), najm::najm_44_3e_0_0_a()] {
            let lens: Vec<usize> = t.cells.iter().map(|c| c.poly.pts.len()).collect();
            assert_eq!(
                lens.iter().filter(|&&n| n == 12).count(),
                1,
                "{}: exactly one 12-gon class witnesses primitivity",
                t.id
            );
            let (a, b) = (t.lattice[0], t.lattice[1]);
            let det = a.cross(b);
            let coords =
                |p: Vec2| ((p.x * b.y - p.y * b.x) / det, (a.x * p.y - a.y * p.x) / det);
            let region = BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0));
            // Mirrors generate()'s inclusion rule: the cell's AABB,
            // translated to each lattice point, must intersect the region.
            let cell = t.cells.iter().find(|c| c.poly.pts.len() == 12).unwrap();
            let cc = centroid(&cell.poly.pts);
            let la = cell.poly.aabb();
            let hits = |c: Vec2| {
                c.x + la.max.x - cc.x > region.min.x
                    && c.x + la.min.x - cc.x < region.max.x
                    && c.y + la.max.y - cc.y > region.min.y
                    && c.y + la.min.y - cc.y < region.max.y
            };
            let pset = ParamSet::default_for(t.params());
            let r = t.generate(&pset, &Query { region });
            let mut counts: std::collections::BTreeMap<(i32, i32), u32> =
                std::collections::BTreeMap::new();
            let mut total = 0;
            for tile in &r.tiles {
                if tile.poly.pts.len() != 12 {
                    continue;
                }
                total += 1;
                let c = centroid(&tile.poly.pts);
                let (u, v) = coords(c);
                assert!(
                    (u - u.round()).abs() < 1e-7 && (v - v.round()).abs() < 1e-7,
                    "{}: dodecagon center {c:?} off the declared lattice ({u}, {v})",
                    t.id
                );
                *counts.entry((u.round() as i32, v.round() as i32)).or_insert(0) += 1;
            }
            assert!(total >= 9, "{}: patch too small to witness", t.id);
            // Every lattice point whose dodecagon meets the region appears
            // exactly once, and there are no others.
            let mut expected = 0;
            for iu in -8..=8i32 {
                for iv in -8..=8i32 {
                    if hits(a * iu as f64 + b * iv as f64) {
                        expected += 1;
                        assert_eq!(
                            counts.get(&(iu, iv)).copied(),
                            Some(1),
                            "{}: lattice point ({iu}, {iv}) has no unique dodecagon",
                            t.id
                        );
                    }
                }
            }
            assert_eq!(
                counts.values().sum::<u32>() as usize,
                expected,
                "{}: no dodecagon outside the lattice-point window",
                t.id
            );
        }
    }}



/// kaplan-22a motif-instance tests: trio + cell-table census (machine-
/// transcribed face tables), motif segment counts and symmetry metadata
/// for `hankin-4-8-8-motifs`.
#[cfg(test)]
mod hankin_4_8_8_motifs_tests {
    use super::hankin_4_8_8_motifs::{hankin_4_8_8_motifs, motifs, EXPECTED_CELL_LENS, EXPECTED_SEEDS, EXPECTED_VERTS};
    use super::hankin_motifs::motif_segments;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_4_8_8_motifs_covers_region() {
        check_covers(
            &hankin_4_8_8_motifs(),
            "hankin-4-8-8-motifs",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_4_8_8_motifs_rotated_scaled() {
        check_rotated(
            &hankin_4_8_8_motifs(),
            "hankin-4-8-8-motifs(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_4_8_8_motifs_deterministic() {
        check_det(
            &hankin_4_8_8_motifs(),
            "hankin-4-8-8-motifs",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_4_8_8_motifs_unit_structure() {
        let t = hankin_4_8_8_motifs();
        assert_eq!(t.cells.len(), 14);
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, EXPECTED_SEEDS.to_vec(), "seed layout");
        let lens: Vec<usize> = t.cells.iter().map(|c| c.poly.pts.len()).collect();
        assert_eq!(lens, EXPECTED_CELL_LENS.to_vec(), "cell vertex counts");
        let flat: Vec<f64> = t
            .cells
            .iter()
            .flat_map(|c| c.poly.pts.iter().flat_map(|p| [p.x, p.y]))
            .collect();
        assert_eq!(flat.len(), EXPECTED_VERTS.len());
        for (a, e) in flat.iter().zip(EXPECTED_VERTS) {
            assert!((a - e).abs() < 1e-9, "cell vertex {a} vs expected {e}");
        }
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        let a1 = 1.0 + core::f64::consts::SQRT_2;
        assert!((det - a1 * a1).abs() < 1e-9);
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_4_8_8_motifs_face_census() {
        // Ideal rosette (s = 3) on the octagon: 32 segments; extension
        // rosette (s = 2) on the diamond: 8 segments (4 spurs + 4
        // post-knee rays).
        let tiles = super::hankin_4_8_8::tiles();
        let ms = motifs();
        let segs: Vec<usize> = tiles
            .iter()
            .zip(&ms)
            .map(|(t, m)| m.as_ref().map_or(0, |m| motif_segments(t, m).len()))
            .collect();
        assert_eq!(segs, vec![32, 8], "motif segment counts");
        let t = hankin_4_8_8_motifs();
        let mut hist: std::collections::BTreeMap<usize, usize> = Default::default();
        for c in &t.cells {
            *hist.entry(c.poly.pts.len()).or_default() += 1;
        }
        assert_eq!(
            hist.into_iter().collect::<Vec<_>>(),
            vec![(4, 4), (6, 8), (8, 1), (32, 1)],
            "face lens census: 8 hexagonal petals + 32-gon core (octagon), 4 quads + octagon core (diamond)"
        );
    }

    #[test]
    fn hankin_4_8_8_motifs_symmetry_metadata() {
        let t = hankin_4_8_8_motifs();
        assert_eq!(t.id, "hankin-4-8-8-motifs");
        assert_eq!(t.group, WallpaperGroup::P4m);
        assert_eq!(t.lattice_kind, LatticeKind::Square);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}

/// kaplan-22a motif-instance tests for `hankin-hex-motifs`.
#[cfg(test)]
mod hankin_hex_motifs_tests {
    use super::hankin_hex_motifs::{hankin_hex_motifs, motifs, tiles, EXPECTED_CELL_LENS, EXPECTED_SEEDS, EXPECTED_VERTS};
    use super::hankin_motifs::motif_segments;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn hankin_hex_motifs_covers_region() {
        check_covers(
            &hankin_hex_motifs(),
            "hankin-hex-motifs",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
        );
    }

    #[test]
    fn hankin_hex_motifs_rotated_scaled() {
        check_rotated(
            &hankin_hex_motifs(),
            "hankin-hex-motifs(rot20,scale0.6)",
            BBox::new(Vec2::new(-24.0, -18.0), Vec2::new(24.0, 18.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn hankin_hex_motifs_deterministic() {
        check_det(
            &hankin_hex_motifs(),
            "hankin-hex-motifs",
            BBox::new(Vec2::new(-12.0, -9.0), Vec2::new(12.0, 9.0)),
        );
    }

    #[test]
    fn hankin_hex_motifs_unit_structure() {
        let t = hankin_hex_motifs();
        assert_eq!(t.cells.len(), 14);
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, EXPECTED_SEEDS.to_vec(), "seed layout");
        let lens: Vec<usize> = t.cells.iter().map(|c| c.poly.pts.len()).collect();
        assert_eq!(lens, EXPECTED_CELL_LENS.to_vec(), "cell vertex counts");
        let flat: Vec<f64> = t
            .cells
            .iter()
            .flat_map(|c| c.poly.pts.iter().flat_map(|p| [p.x, p.y]))
            .collect();
        assert_eq!(flat.len(), EXPECTED_VERTS.len());
        for (a, e) in flat.iter().zip(EXPECTED_VERTS) {
            assert!((a - e).abs() < 1e-9, "cell vertex {a} vs expected {e}");
        }
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        let rt3 = 1.7320508075688772;
        assert!((det - 3.0 * rt3).abs() < 1e-9);
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
    }

    #[test]
    fn hankin_hex_motifs_face_census() {
        // Ideal rosette (s = 2) on hexagon A: 24 segments; extension
        // star (s = 1) on hexagon B: 12 spurs (a star's leg line is
        // already the spur — extended stars emit spurs only).
        let ts = tiles();
        let ms = motifs();
        let segs: Vec<usize> = ts
            .iter()
            .zip(&ms)
            .map(|(t, m)| m.as_ref().map_or(0, |m| motif_segments(t, m).len()))
            .collect();
        assert_eq!(segs, vec![24, 12], "motif segment counts");
        let t = hankin_hex_motifs();
        let mut hist: std::collections::BTreeMap<usize, usize> = Default::default();
        for c in &t.cells {
            *hist.entry(c.poly.pts.len()).or_default() += 1;
        }
        assert_eq!(
            hist.into_iter().collect::<Vec<_>>(),
            vec![(4, 6), (6, 6), (12, 1), (24, 1)],
            "face lens census: 6 hexagonal petals + 24-gon core (A), 6 quads + 12-gon core (B)"
        );
    }

    #[test]
    fn hankin_hex_motifs_symmetry_metadata() {
        let t = hankin_hex_motifs();
        assert_eq!(t.id, "hankin-hex-motifs");
        assert_eq!(t.group, WallpaperGroup::Pmm);
        assert_eq!(t.lattice_kind, LatticeKind::Rectangular);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}

/// kaplan-22a motif-instance tests for `najm-63-motifs`.
#[cfg(test)]
mod najm_63_motifs_tests {
    use super::hankin_motifs::motif_segments;
    use super::najm_63_motifs::{motifs, substrate_tiles};
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};

    #[test]
    fn najm_63_motifs_covers_region() {
        check_covers(
            &super::najm_63_motifs::najm_63_motifs(),
            "najm-63-motifs",
            BBox::new(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0)),
        );
    }

    #[test]
    fn najm_63_motifs_rotated_scaled() {
        check_rotated(
            &super::najm_63_motifs::najm_63_motifs(),
            "najm-63-motifs(rot20,scale0.6)",
            BBox::new(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0)),
            0.6,
            20.0,
        );
    }

    #[test]
    fn najm_63_motifs_deterministic() {
        check_det(
            &super::najm_63_motifs::najm_63_motifs(),
            "najm-63-motifs",
            BBox::new(Vec2::new(-6.0, -6.0), Vec2::new(6.0, 6.0)),
        );
    }

    #[test]
    fn najm_63_motifs_unit_structure() {
        use super::najm_63_motifs::{EXPECTED_CELL_LENS, EXPECTED_SEEDS, EXPECTED_VERTS};
        let t = super::najm_63_motifs::najm_63_motifs();
        assert_eq!(t.cells.len(), 36);
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, EXPECTED_SEEDS.to_vec(), "seed layout");
        let lens: Vec<usize> = t.cells.iter().map(|c| c.poly.pts.len()).collect();
        assert_eq!(lens, EXPECTED_CELL_LENS.to_vec(), "cell vertex counts");
        let flat: Vec<f64> = t
            .cells
            .iter()
            .flat_map(|c| c.poly.pts.iter().flat_map(|p| [p.x, p.y]))
            .collect();
        assert_eq!(flat.len(), EXPECTED_VERTS.len());
        for (a, e) in flat.iter().zip(EXPECTED_VERTS) {
            assert!((a - e).abs() < 1e-9, "cell vertex {a} vs expected {e}");
        }
        let det = t.lattice[0].x * t.lattice[1].y - t.lattice[0].y * t.lattice[1].x;
        assert!((det - 2.598076211353316).abs() < 1e-9, "det = 3√3/2");
        let sum: f64 = t.cells.iter().map(|c| c.poly.area()).sum();
        assert!((sum - det).abs() < 1e-9, "Σ areas {sum} vs det {det}");
        // The substrate table is reused verbatim: the motif-free
        // substrate tiles are the registered Najm cells.
        let base = super::najm::najm_63_2e_0_3e_ac();
        let tiles = substrate_tiles(&base);
        assert_eq!(tiles.len(), 6);
    }

    #[test]
    fn najm_63_motifs_face_census() {
        // Ideal rosette (s = 3) on the dodecagon: 48 segments; star
        // (theta = 35 degrees, s = 2) on each enneagon: 18 segments;
        // bowties Motif::None: 0 segments each.
        let base = super::najm::najm_63_2e_0_3e_ac();
        let tiles = substrate_tiles(&base);
        let ms = motifs();
        let segs: Vec<usize> = tiles
            .iter()
            .zip(&ms)
            .map(|(t, m)| m.as_ref().map_or(0, |m| motif_segments(t, m).len()))
            .collect();
        assert_eq!(segs, vec![48, 18, 18, 0, 0, 0], "motif segment counts");
        let t = super::najm_63_motifs::najm_63_motifs();
        let mut hist: std::collections::BTreeMap<usize, usize> = Default::default();
        for c in &t.cells {
            *hist.entry(c.poly.pts.len()).or_default() += 1;
        }
        assert_eq!(
            hist.into_iter().collect::<Vec<_>>(),
            vec![(4, 18), (6, 15), (18, 2), (48, 1)],
            "face lens census: 12 petals + 48-gon core (dodecagon), 2 × (9 quads + 18-gon core) (enneagons), 3 plain bowties"
        );
    }

    #[test]
    fn najm_63_motifs_symmetry_metadata() {
        let t = super::najm_63_motifs::najm_63_motifs();
        assert_eq!(t.id, "najm-63-motifs");
        assert_eq!(t.group, WallpaperGroup::P6m);
        assert_eq!(t.lattice_kind, LatticeKind::Hexagonal);
        assert_eq!(t.category, TilingCategory::Islamic);
        assert_eq!(t.secondary, &[]);
    }
}

