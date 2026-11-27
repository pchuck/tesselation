//! Pentagons family: the convex pentagonal tiling census: types 1-3 and 6-15 as data tables,
//! plus the Hirschhorn non-edge-to-edge pentagon
//!
//! Owned here: module declarations, `register` (the family's registry
//! slice, in within-tab order), `classification` arms, the family's
//! lattice-culling entries and the family census pin. The family doc is
//! `docs/families/pentagons.md`.

use tess_core::Registry;
use tess_core::classify::TilingClass;

use crate::classify::{pentagon};

pub mod pentagon1;
pub mod pentagon2;
pub mod pentagon3;
pub mod pentagon7;
pub mod pentagon8;
pub mod pentagon6;
pub mod pentagon9;
pub mod pentagon10;
pub mod pentagon11;
pub mod pentagon12;
pub mod pentagon13;
pub mod pentagon14;
pub mod pentagon15;
pub mod hirschhorn;

/// The family's registry slice, in within-tab order.
pub fn register(mut r: Registry) -> Registry {
        r = r.register(pentagon2::spec());
        r = r.register(pentagon3::spec());
        r = r.register(pentagon1::spec());
        r = r.register(pentagon7::spec());
        r = r.register(pentagon8::spec());
        r = r.register(pentagon6::spec());
        r = r.register(pentagon9::spec());
        r = r.register(pentagon10::spec());
        r = r.register(pentagon11::spec());
        r = r.register(pentagon12::spec());
        r = r.register(pentagon13::spec());
        r = r.register(pentagon14::spec());
        r = r.register(pentagon15::spec());
        r = r.register(hirschhorn::HirschhornTiling::new());
    r
}

/// The family's classification arms (see `crate::classify` for the
/// composer and the shared constructor helpers).
pub(crate) fn classification(id: &str) -> Option<TilingClass> {
    // Arms are verbatim from the pre-split table (they yield
    // `TilingClass`); an unknown id returns None so the composer can
    // fall through to later families.
    Some(match id {
        "pentagon-1" => pentagon(&["Pentagon"], 1, 1, "Reinhardt", "1918", Some("achiral")),
        "pentagon-2" => pentagon(&["Pentagon"], 2, 1, "Reinhardt", "1918", Some("mixed")),
        "pentagon-3" => pentagon(&["Pentagon", "Wallpaper"], 3, 1, "Reinhardt", "1918", Some("homochiral")),
        "pentagon-6" => pentagon(&["Pentagon"], 6, 2, "Kershner", "1968", Some("achiral")),
        "pentagon-7" => pentagon(&["Pentagon"], 7, 2, "Kershner", "1968", Some("mixed")),
        "pentagon-8" => pentagon(&["Pentagon"], 8, 2, "Kershner", "1968", Some("mixed")),
        "pentagon-9" => pentagon(&["Pentagon"], 9, 2, "Rice", "1976", None),
        "pentagon-10" => pentagon(&["Pentagon"], 10, 3, "James", "1975", None),
        "pentagon-11" => pentagon(&["Pentagon"], 11, 2, "Rice", "1976", None),
        "pentagon-12" => pentagon(&["Pentagon"], 12, 2, "Rice", "1976", None),
        "pentagon-13" => pentagon(&["Pentagon"], 13, 2, "Rice", "1977", Some("mixed")),
        "pentagon-14" => pentagon(&["Pentagon"], 14, 3, "Stein", "1985", None),
        "pentagon-15" => pentagon(&["Pentagon"], 15, 3, "Mann et al.", "2015", None),
        "hirschhorn" => TilingClass {
            families: &["Pentagon", "Nonperiodic"],
            source: Some("Hirschhorn & Hunt"),
            year: Some("1985"),
            translational: Some("non-periodic"),
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
            ("pentagon-1", pentagon1::spec()),
            ("pentagon-2", pentagon2::spec()),
            ("pentagon-3", pentagon3::spec()),
            ("pentagon-6", pentagon6::spec()),
            ("pentagon-7", pentagon7::spec()),
            ("pentagon-8", pentagon8::spec()),
            ("pentagon-9", pentagon9::spec()),
            ("pentagon-10", pentagon10::spec()),
            ("pentagon-11", pentagon11::spec()),
            ("pentagon-12", pentagon12::spec()),
            ("pentagon-13", pentagon13::spec()),
            ("pentagon-14", pentagon14::spec()),
            ("pentagon-15", pentagon15::spec()),
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
        "pentagon-1",
        "pentagon-2",
        "pentagon-3",
        "pentagon-6",
        "pentagon-7",
        "pentagon-8",
        "pentagon-9",
        "pentagon-10",
        "pentagon-11",
        "pentagon-12",
        "pentagon-13",
        "pentagon-14",
        "pentagon-15",
        "hirschhorn",
    ];

    #[test]
    fn family_census_matches() {
        let r = register(Registry::new());
        assert_eq!(
            r.len(),
            14,
            "pentagons family census — update when adding a tiling to this family"
        );
        let have: BTreeSet<&str> = r.iter().map(|t| t.id()).collect();
        let want: BTreeSet<&str> = IDS.iter().copied().collect();
        assert_eq!(
            have, want,
            "pentagons family membership drift: register() vs IDS"
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
mod hirschhorn_tests {
    use crate::tests::{check_coverage, check_deterministic, check_rotated};
    use crate::hirschhorn;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn hirschhorn_covers_region() {
        let t = hirschhorn::HirschhornTiling::new();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(
            &t,
            "hirschhorn",
            &p,
            BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
        );
    }

    #[test]
    fn hirschhorn_rotated_scaled() {
        check_rotated(
            &hirschhorn::HirschhornTiling::new(),
            "hirschhorn(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn hirschhorn_deterministic() {
        // scale 0.5 keeps the grown patch small enough for a fast check.
        let t = hirschhorn::HirschhornTiling::new();
        let mut p = ParamSet::default_for(t.params());
        p.set(
            crate::lattice::SCALE_ID,
            tess_core::params::ParamValue::Float(0.5),
        );
        check_deterministic(
            &t,
            "hirschhorn",
            &p,
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(test)]
mod pentagon13_tests {
    use super::pentagon13::{side_a, side_c, side_d, s_lat, tile_area};
    use crate::tests::{check_coverage, check_det, check_rotated};
    use crate::pentagon13::spec;
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    fn shoelace_sign(poly: &Polygon) -> f64 {
        let n = poly.len();
        (0..n)
            .map(|i| {
                let (a, b) = (poly.pts[i], poly.pts[(i + 1) % n]);
                a.x * b.y - b.x * a.y
            })
            .sum()
    }

    #[test]
    fn pentagon_13_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(
            &t,
            "pentagon-13",
            &p,
            BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn pentagon_13_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-13(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(7.0, 7.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_13_deterministic() {
        check_det(
            &spec(),
            "pentagon-13",
            BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)),
        );
    }

    /// Unit structure of the primitive cell: 8 tiles of the exact
    /// Q(√3) prototile (A = C = D = 120°, B = E = 90°, d = 2a = 2e) on
    /// the rectangular lattice T1 = (3s, 3s), T2 = (s, −s), s = (9+√3)/13,
    /// with |det| = 8 × tile area and |T1| = 3|T2|, T1 ⊥ T2.
    #[test]
    fn pentagon_13_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.cells.len(), 8, "the primitive unit holds eight tiles");
        let s = s_lat();
        assert_eq!(t.lattice[0], Vec2::new(3.0 * s, 3.0 * s), "T1 = (3s, 3s)");
        assert_eq!(t.lattice[1], Vec2::new(s, -s), "T2 = (s, −s)");
        let det = t.lattice[0].cross(t.lattice[1]);
        assert!(
            (det.abs() - 8.0 * tile_area()).abs() < 1e-12,
            "lattice cell area {det} ≠ 8 × tile area"
        );
        let (t1, t2) = (t.lattice[0], t.lattice[1]);
        assert!((t1.dot(t2)).abs() < 1e-12, "lattice must be rectangular");
        assert!(
            (t1.length() - 3.0 * t2.length()).abs() < 1e-12,
            "|T1| must be 3×|T2|"
        );
        assert!((t2.length() - s * 2.0f64.sqrt()).abs() < 1e-12, "|T2| must be s√2");
        // Side lengths: a = e = (3√3+1)/13, d = 2a, c = (4−√3)/13.
        let a = side_a();
        let c = side_c();
        let d = side_d();
        assert!((d - 2.0 * a).abs() < 1e-12, "d = 2a");
        assert!(c < a && a < d);
        // Every cell: sides b, c, d/2, d/2, e, a and angles
        // 120°, 90°, 120°, 180° (the collinear d-midpoint), 120°, 90°.
        let want_sides = [1.0, c, d / 2.0, d / 2.0, a, a];
        let want_angles = [
            std::f64::consts::TAU / 3.0,
            std::f64::consts::FRAC_PI_2,
            std::f64::consts::TAU / 3.0,
            std::f64::consts::PI,
            std::f64::consts::TAU / 3.0,
            std::f64::consts::FRAC_PI_2,
        ];
        for cell in &t.cells {
            assert!(
                (cell.poly.area() - tile_area()).abs() < 1e-12,
                "cell area ≠ 9(14+3√3)/338"
            );
            let n = cell.poly.len();
            assert_eq!(n, 6, "cells carry the d-midpoint vertex");
            for i in 0..n {
                let (p0, p1, p2) =
                    (cell.poly.pts[(i + n - 1) % n], cell.poly.pts[i], cell.poly.pts[(i + 1) % n]);
                let len = (p2 - p1).length();
                assert!(
                    (len - want_sides[i]).abs() < 1e-12,
                    "side {i} = {len} ≠ {}",
                    want_sides[i]
                );
                let v1 = p0 - p1;
                let v2 = p2 - p1;
                let ang = (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos();
                assert!(
                    (ang - want_angles[i]).abs() < 1e-9,
                    "angle {i} = {} rad ≠ {}",
                    ang,
                    want_angles[i]
                );
            }
        }
    }

    /// 2-isohedral: seeds 0 and 1 are exactly the two chiral (isohedral)
    /// classes — four CCW cells and four CW cells — and every generated
    /// tile keeps the chirality of its seed (the cell map is a
    /// similarity, which cannot flip orientation).
    #[test]
    fn pentagon_13_seeds_track_the_chiral_pair() {
        let t = spec();
        let n0 = t
            .cells
            .iter()
            .filter(|c| c.seed == 0)
            .count();
        let n1 = t.cells.len() - n0;
        assert_eq!((n0, n1), (4, 4), "four tiles per chiral class");
        for c in &t.cells {
            let sign = if shoelace_sign(&c.poly) > 0.0 { 1 } else { -1 };
            let want = if c.seed == 0 { 1 } else { -1 };
            assert_eq!(sign, want, "seed {} cell has the wrong chirality", c.seed);
        }
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region: BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let mut seen = [false; 2];
        for tile in &r.tiles {
            assert!(tile.seed <= 1, "seeds must stay in {{0, 1}}");
            seen[tile.seed as usize] = true;
            let sign = if shoelace_sign(&tile.poly) > 0.0 { 1 } else { -1 };
            let want = if tile.seed == 0 { 1 } else { -1 };
            assert_eq!(sign, want, "generated tile flipped chirality (seed {})", tile.seed);
        }
        assert!(seen[0] && seen[1], "both chiral classes must appear");
    }

    /// The wallpaper group is p2 (pgg with the chiral pair distinguished):
    /// the 2-fold center is the midpoint of the prototile's long side
    /// (vertex M of the L0 cell), and 180° rotation about it preserves
    /// the tiling exactly.
    #[test]
    fn pentagon_13_c2_rotation_is_a_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let c = t.cells[0].poly.pts[3];
        let rot = |q: Vec2| c * 2.0 - q;
        let ra = BBox::new(v2(-4.0, -4.0), v2(10.0, 10.0));
        let cs: Vec<Vec2> = ra.corners().iter().map(|&q| rot(q)).collect();
        let rb = BBox::new(
            Vec2::new(
                cs.iter().map(|q| q.x).fold(f64::MAX, f64::min),
                cs.iter().map(|q| q.y).fold(f64::MAX, f64::min),
            ),
            Vec2::new(
                cs.iter().map(|q| q.x).fold(f64::MIN, f64::max),
                cs.iter().map(|q| q.y).fold(f64::MIN, f64::max),
            ),
        );
        let ra_tiles = t.generate(&p, &Query { region: ra });
        let rb_tiles = t.generate(&p, &Query { region: rb });
        assert!(ra_tiles.message.is_none(), "{:?}", ra_tiles.message);
        assert!(rb_tiles.message.is_none(), "{:?}", rb_tiles.message);
        assert!(!ra_tiles.tiles.is_empty());
        // Vertex key on a 1e-6 grid (distinct vertices are ≥ c = 0.174
        // apart; the two computations of a shared joint agree to ~1e-14).
        let key = |pts: &[Vec2]| {
            let mut v: Vec<(i64, i64)> = pts
                .iter()
                .map(|q| ((q.x * 1e6).round() as i64, (q.y * 1e6).round() as i64))
                .collect();
            v.sort_unstable();
            v
        };
        let set_b: std::collections::HashSet<Vec<(i64, i64)>> =
            rb_tiles.tiles.iter().map(|t| key(&t.poly.pts)).collect();
        let set_a: std::collections::HashSet<Vec<(i64, i64)>> = ra_tiles
            .tiles
            .iter()
            .map(|t| key(&t.poly.pts.iter().map(|q| rot(*q)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(set_a.len(), ra_tiles.tiles.len(), "rotation must be injective on the patch");
        assert_eq!(
            set_a, set_b,
            "180° rotation about the d-midpoint 2-fold center must preserve the tiling"
        );
    }

    #[test]
    fn pentagon_13_symmetry_metadata() {
        let t = spec();
        assert_eq!(t.id, "pentagon-13");
        assert_eq!(t.group, WallpaperGroup::P2);
        assert_eq!(t.lattice_kind, LatticeKind::Rectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the L0 cell (seed 0)");
    }

    /// Vertex-set key on the 1e-6 grid (shared joints computed along
    /// different routes agree to ~1e-14, distinct vertices are ≫1e-6 apart).
    fn key13(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|q| ((q.x * 1e6).round() as i64, (q.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Seed-blind table census: key of every cell polygon translated by
    /// n1·w1 + n2·w2, n ∈ [−9, 9].  A symmetry maps the base cell onto a
    /// table copy inside this window.
    fn shifted_cells(
        t: &crate::lattice::LatticeTiling,
    ) -> std::collections::HashMap<Vec<(i64, i64)>, (usize, i32, i32)> {
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let mut map = std::collections::HashMap::new();
        for (j, cl) in t.cells.iter().enumerate() {
            for n1 in -9..=9i32 {
                for n2 in -9..=9i32 {
                    let sh: Vec<Vec2> = cl
                        .poly
                        .pts
                        .iter()
                        .map(|q| {
                            Vec2::new(
                                q.x + (n1 as f64) * w1.x + (n2 as f64) * w2.x,
                                q.y + (n1 as f64) * w1.y + (n2 as f64) * w2.y,
                            )
                        })
                        .collect();
                    map.insert(key13(&sh), (j, n1, n2));
                }
            }
        }
        map
    }

    /// Seed-blind test for the isometry g(x) = M·x + v: every cell
    /// polygon maps onto a table cell plus a lattice translation, the
    /// (cell, translation) assignments being distinct.  Such a table map
    /// is injective, hence surjective on the (finite-cell) tiling, so g
    /// is a symmetry of the UNCOLOURED pattern (seeds ignored).
    fn table_symmetry(
        t: &crate::lattice::LatticeTiling,
        idx: &std::collections::HashMap<Vec<(i64, i64)>, (usize, i32, i32)>,
        m: (f64, f64, f64, f64),
        v: Vec2,
    ) -> bool {
        let (a, b, c, d) = m;
        let mut used = std::collections::HashSet::new();
        for cl in &t.cells {
            let img: Vec<Vec2> = cl
                .poly
                .pts
                .iter()
                .map(|q| Vec2::new(a * q.x + b * q.y + v.x, c * q.x + d * q.y + v.y))
                .collect();
            match idx.get(&key13(&img)) {
                None => return false,
                Some(&p) => {
                    if !used.insert(p) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Complete candidate list of translation parts v (mod the lattice)
    /// for a symmetry with linear part M: cell 0's vertex 0 must map to
    /// SOME vertex of SOME table cell plus a lattice translation, so
    /// v ≡ p_jk − M·p_00 (mod Λ) over all cells j and vertices k.
    /// Enumerating these finitely many candidates misses no symmetry.
    fn coset_candidates(
        t: &crate::lattice::LatticeTiling,
        m: (f64, f64, f64, f64),
    ) -> Vec<Vec2> {
        let (a, b, c, d) = m;
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.x * w2.y - w1.y * w2.x;
        let p0 = t.cells[0].poly.pts[0];
        let (m00, m01) = (a * p0.x + b * p0.y, c * p0.x + d * p0.y);
        let _ = det;
        let mut out = Vec::new();
        for cl in &t.cells {
            for q in &cl.poly.pts {
                out.push(Vec2::new(q.x - m00, q.y - m01));
            }
        }
        out
    }

    /// The UNCOLOURED type-13 pattern (chiral pair not distinguished) is
    /// pgg on the rectangular lattice; the shipped p2 is the
    /// seeds-distinguished (homochiral) subgroup — exactly the Wikipedia
    /// dichotomy "pgg, or p2 if mirror-image tiles are considered
    /// distinct".  Programmatic census over the complete candidate set
    /// (vertex correspondence, so nothing is missed):
    ///
    /// * the declared lattice is rectangular and NOT square (w1·w2 = 0,
    ///   |w1| = 3|w2|) and no shorter translation exists, so the only
    ///   possible point groups are subgroups of {I, −I, M1, M2, ±90°, M3, M4};
    /// * translations: exactly Λ (no extra identity-coset hits);
    /// * the −I coset contains symmetries (the shipped half-turn about
    ///   the long-side midpoint is one of them);
    /// * the two reflection cosets M1 (axis ∥ w1), M2 (axis ∥ w2) contain
    ///   half-period GLIDES — g² = a nonzero translation along the axis —
    ///   and NO pure reflection (v + Mv = 0) exists anywhere;
    /// * the ±90° rotations and the diagonal reflection cosets M3, M4
    ///   contain no symmetries (so the pattern is not p4g/p4m/cmm).
    ///
    /// Translations + half-turns + two perpendicular mirror-free glide
    /// families, nothing more: the uncoloured orbifold is 22* = pgg.
    /// The seeded label P2 stays pinned by
    /// `pentagon_13_c2_rotation_is_a_symmetry` and
    /// `pentagon_13_seeds_track_the_chiral_pair`.
    #[test]
    fn pentagon_13_uncolored_pattern_is_pgg() {
        let t = spec();
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let dot = w1.x * w2.x + w1.y * w2.y;
        let n1 = (w1.x * w1.x + w1.y * w1.y).sqrt();
        let n2 = (w2.x * w2.x + w2.y * w2.y).sqrt();
        assert!(dot.abs() < 1e-9 * n1 * n2, "w1 ⟂ w2 (rectangular lattice)");
        assert!(
            (n1 / n2 - 3.0).abs() < 1e-9,
            "|w1| = 3|w2|: rectangular, not square"
        );
        let idx = shifted_cells(&t);
        let hits = |m: (f64, f64, f64, f64)| -> Vec<Vec2> {
            coset_candidates(&t, m)
                .into_iter()
                .filter(|&v| table_symmetry(&t, &idx, m, v))
                .collect()
        };
        let is_zero_mod = |v: Vec2| -> bool {
            let det = w1.x * w2.y - w1.y * w2.x;
            let al = (v.x * w2.y - v.y * w2.x) / det;
            let be = (w1.x * v.y - w1.y * v.x) / det;
            (al - al.round()).abs() < 1e-6 && (be - be.round()).abs() < 1e-6
        };
        // (1) Translations: exactly the declared lattice.
        let id = (1.0, 0.0, 0.0, 1.0);
        let id_hits = hits(id);
        assert!(
            id_hits.iter().all(|&v| is_zero_mod(v)),
            "no translation outside the declared lattice: uncoloured translation \
             lattice is exactly ⟨w1, w2⟩ ({} extra hits)",
            id_hits.iter().filter(|&v| !is_zero_mod(*v)).count()
        );
        // (2) Half-turns exist — including the shipped 2-fold centre.
        let mi = (-1.0, 0.0, 0.0, -1.0);
        let rot_hits = hits(mi);
        assert!(
            !rot_hits.is_empty(),
            "the −I coset must contain half-turns"
        );
        let c0 = t.cells[0].poly.pts[3];
        assert!(
            table_symmetry(&t, &idx, mi, c0 * 2.0),
            "the shipped half-turn centre pts[3] must survive seed-blindness"
        );
        // (3) Reflection cosets: M1 axis ∥ (1,1) ∥ w1; M2 axis ∥ (1,−1) ∥ w2;
        //     M3 axis ∥ (1,0); M4 axis ∥ (0,1).
        let ms: [(f64, f64, f64, f64); 4] = [
            (0.0, 1.0, 1.0, 0.0),
            (0.0, -1.0, -1.0, 0.0),
            (1.0, 0.0, 0.0, -1.0),
            (-1.0, 0.0, 0.0, 1.0),
        ];
        let mut glide_counts = [0usize; 4];
        let mut pure_total = 0usize;
        let mut found_glide: Option<((f64, f64, f64, f64), Vec2)> = None;
        for (i, &m) in ms.iter().enumerate() {
            for v in hits(m) {
                // g² = translation by v + M·v, always along the mirror axis.
                let sq = Vec2::new(
                    v.x + m.0 * v.x + m.1 * v.y,
                    v.y + m.2 * v.x + m.3 * v.y,
                );
                let sqn = (sq.x * sq.x + sq.y * sq.y).sqrt();
                if sqn < 1e-7 * n2 {
                    pure_total += 1;
                } else {
                    glide_counts[i] += 1;
                    found_glide.get_or_insert((m, v));
                }
            }
        }
        assert_eq!(
            pure_total, 0,
            "no pure reflection anywhere (pgg, not pmm/cmm/p4m)"
        );
        assert!(
            glide_counts[0] > 0 && glide_counts[1] > 0,
            "both lattice-parallel reflection cosets contain glides: {:?}",
            glide_counts
        );
        assert_eq!(
            glide_counts[2], 0,
            "the diagonal reflection coset M3 (axis ∥ x) has no symmetry: the \
             point group is exactly {{I, −I, M1, M2}} (exotic finer-lattice \
             scenarios excluded)"
        );
        assert_eq!(glide_counts[3], 0, "the diagonal reflection coset M4 (axis ∥ y) likewise has none");
        // (4) No 4-fold rotations: the group is not p4g/p4m.
        for r4 in [
            (0.0, -1.0, 1.0, 0.0),
            (0.0, 1.0, -1.0, 0.0),
        ] {
            assert!(
                hits(r4).is_empty(),
                "no quarter-turn symmetry (group is not p4-family)"
            );
        }
        // (5) The glides are TRUE half-period glides: g² is a nonzero
        //     primitive lattice translation along the axis.  Check one.
        let (gm, gv) = found_glide.expect("glide found above");
        let sq = Vec2::new(
            gv.x + gm.0 * gv.x + gm.1 * gv.y,
            gv.y + gm.2 * gv.x + gm.3 * gv.y,
        );
        let prim = if gm.0 == 0.0 {
            w1
        } else {
            w2
        };
        let par = sq.x * prim.y - sq.y * prim.x;
        let scale = (sq.x * prim.x + sq.y * prim.y) / (prim.x * prim.x + prim.y * prim.y);
        assert!(
            par.abs() < 1e-7 * n1,
            "g² ∥ axis (glides along ⟨w⟩): cross {:.2e}",
            par
        );
        assert!(
            (scale - scale.round()).abs() < 1e-6 && scale.abs() > 0.4,
            "g² = {}·w is a nonzero lattice translation along the axis (true glide)",
            scale.round()
        );
    }
}

#[cfg(test)]
mod pentagon14_tests {
    use super::pentagon14::{beta, bp, bq, p, pq, q, s57, tile_area, w1, w2};
    use crate::tests::{check_coverage, check_det, check_rotated};
    use crate::pentagon14::spec;
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn pentagon_14_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(
            &t,
            "pentagon-14",
            &p,
            BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn pentagon_14_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-14(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(7.0, 7.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_14_deterministic() {
        check_det(
            &spec(),
            "pentagon-14",
            BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)),
        );
    }

    /// Unit structure of the primitive cell: 6 tiles of the exact
    /// Q(√57, β, p, bq) prototile (A = 90°, 2B + C = 360°, C + E = 180°,
    /// 2a = 2c = d = e) on the oblique lattice w1 = (2pq, −(13+9√57)/16),
    /// w2 = (−(β+2p), −(√57−1)/2), with |det| = 6 × tile area and
    /// w1 · w2 ≠ 0.
    #[test]
    fn pentagon_14_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-14");
        assert_eq!(t.group, WallpaperGroup::P2);
        assert_eq!(t.lattice_kind, LatticeKind::Oblique);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the D0 cell (seed 0)");
        assert_eq!(t.cells.len(), 6, "the primitive unit holds six tiles");
        assert_eq!(
            t.lattice[0],
            Vec2::new(2.0 * pq(), -(13.0 + 9.0 * s57()) / 16.0),
            "w1 = (2pq, −(13+9√57)/16)"
        );
        assert_eq!(
            t.lattice[1],
            Vec2::new(-(beta() + 2.0 * p()), -(s57() - 1.0) / 2.0),
            "w2 = (−(β+2p), −(√57−1)/2)"
        );
        let det = t.lattice[0].cross(t.lattice[1]);
        assert!(
            (det.abs() - 6.0 * tile_area()).abs() < 1e-12,
            "lattice cell area {det} ≠ 6 × tile area"
        );
        assert!(
            (t.lattice[0].dot(t.lattice[1])).abs() > 1.0,
            "lattice must be oblique (w1 · w2 ≠ 0)"
        );
        // Exact surd identities the cell table relies on.
        assert!((bp() - beta() * p()).abs() < 1e-12, "βp = (43−√57)/16");
        assert!((bq() - beta() * q()).abs() < 1e-12, "βq = √((438√57−2706)/256)");
        // Angles of the prototile, derived from the exact sin B = q.
        let b = beta();
        let b_ = std::f64::consts::PI - q().asin();
        let c_ = 2.0 * std::f64::consts::PI - 2.0 * b_;
        let d_ = 3.0 * std::f64::consts::FRAC_PI_2 - b_;
        let e_ = 2.0 * b_ - std::f64::consts::PI;
        // Per-class fingerprints: vertex count, side lengths and interior
        // angles in cyclic order (all cells are CCW).
        for cell in &t.cells {
            assert!(
                (cell.poly.area() - tile_area()).abs() < 1e-12,
                "cell area ≠ exact tile area"
            );
            let (want_sides, want_angles): (&[f64], &[f64]) = match cell.seed {
                0 => (
                    &[b, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
                    &[
                        std::f64::consts::FRAC_PI_2,
                        b_,
                        c_,
                        std::f64::consts::PI,
                        d_,
                        std::f64::consts::PI,
                        e_,
                    ],
                ),
                1 => (
                    &[1.0, 2.0, 2.0, 1.0, b],
                    &[std::f64::consts::FRAC_PI_2, e_, d_, c_, b_],
                ),
                _ => (
                    &[2.0, 1.0, 1.0, 1.0, b, 1.0],
                    &[
                        e_,
                        d_,
                        std::f64::consts::PI,
                        c_,
                        b_,
                        std::f64::consts::FRAC_PI_2,
                    ],
                ),
            };
            let n = cell.poly.len();
            assert_eq!(n, want_sides.len(), "seed {} vertex count", cell.seed);
            for i in 0..n {
                let (p0, p1, p2) = (
                    cell.poly.pts[(i + n - 1) % n],
                    cell.poly.pts[i],
                    cell.poly.pts[(i + 1) % n],
                );
                let len = (p2 - p1).length();
                assert!(
                    (len - want_sides[i]).abs() < 1e-12,
                    "side {i} = {len} ≠ {}",
                    want_sides[i]
                );
                let v1 = p0 - p1;
                let v2 = p2 - p1;
                let ang = (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos();
                assert!(
                    (ang - want_angles[i]).abs() < 1e-9,
                    "angle {i} = {} rad ≠ {}",
                    ang,
                    want_angles[i]
                );
            }
        }
    }

    /// 3-isohedral: seeds 0, 1, 2 are the three isohedral classes — two
    /// tiles of each in the primitive cell — and every generated tile
    /// keeps the vertex count and side multiset of its seed class.
    #[test]
    fn pentagon_14_isohedral_classes() {
        let t = spec();
        let mut counts = [0usize; 3];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(counts, [2, 2, 2], "two tiles per isohedral class");
        let params = ParamSet::default_for(t.params());
        let r = t.generate(&params, &Query { region: BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let b = beta();
        let mut seen = [false; 3];
        for tile in &r.tiles {
            let s = tile.seed as usize;
            assert!(s < 3, "seeds must stay in {{0, 1, 2}}");
            seen[s] = true;
            let want_sides: &[f64] = match tile.seed {
                0 => &[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, b],
                1 => &[1.0, 1.0, 2.0, 2.0, b],
                _ => &[1.0, 1.0, 1.0, 1.0, 2.0, b],
            };
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| {
                    (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length()
                })
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            assert_eq!(sides.len(), want_sides.len(), "seed {s} vertex count");
            for (got, want) in sides.iter().zip(want_sides) {
                assert!(
                    (got - want).abs() < 1e-12,
                    "seed {s} side {got} ∉ {{unit, 2, β}}"
                );
            }
        }
        assert!(
            seen[0] && seen[1] && seen[2],
            "all three isohedral classes must appear"
        );
    }

    /// The wallpaper group is p2: the 2-fold centre is the midpoint
    /// O = (β + p/2, q/2) of the segment joining the A-corners of the
    /// D0/D180 pair, and 180° rotation about it preserves the tiling
    /// (each class onto itself: D0 ↔ D180, M0 ↔ M180, MTHS ↔ MTHSp180).
    #[test]
    fn pentagon_14_c2_symmetry() {
        let t = spec();
        let params = ParamSet::default_for(t.params());
        let c = Vec2::new((2.0 * beta() + p()) / 2.0, q() / 2.0);
        let rot = |pt: Vec2| c * 2.0 - pt;
        let ra = BBox::new(v2(-4.0, -4.0), v2(10.0, 10.0));
        let cs: Vec<Vec2> = ra.corners().iter().map(|&pt| rot(pt)).collect();
        let rb = BBox::new(
            Vec2::new(
                cs.iter().map(|pt| pt.x).fold(f64::MAX, f64::min),
                cs.iter().map(|pt| pt.y).fold(f64::MAX, f64::min),
            ),
            Vec2::new(
                cs.iter().map(|pt| pt.x).fold(f64::MIN, f64::max),
                cs.iter().map(|pt| pt.y).fold(f64::MIN, f64::max),
            ),
        );
        let ra_tiles = t.generate(&params, &Query { region: ra });
        let rb_tiles = t.generate(&params, &Query { region: rb });
        assert!(ra_tiles.message.is_none(), "{:?}", ra_tiles.message);
        assert!(rb_tiles.message.is_none(), "{:?}", rb_tiles.message);
        assert!(!ra_tiles.tiles.is_empty());
        // Vertex key on a 1e-6 grid (distinct tiling vertices are ≥ 1
        // unit apart — every fine edge has length 1; the two routes to a
        // shared joint agree to ~1e-15).
        let key = |pts: &[Vec2]| {
            let mut v: Vec<(i64, i64)> = pts
                .iter()
                .map(|pt| ((pt.x * 1e6).round() as i64, (pt.y * 1e6).round() as i64))
                .collect();
            v.sort_unstable();
            v
        };
        let set_b: std::collections::HashSet<Vec<(i64, i64)>> =
            rb_tiles.tiles.iter().map(|t| key(&t.poly.pts)).collect();
        let set_a: std::collections::HashSet<Vec<(i64, i64)>> = ra_tiles
            .tiles
            .iter()
            .map(|t| key(&t.poly.pts.iter().map(|pt| rot(*pt)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(
            set_a.len(),
            ra_tiles.tiles.len(),
            "rotation must be injective on the patch"
        );
        assert_eq!(
            set_a, set_b,
            "180° rotation about the 2-fold centre O must preserve the tiling"
        );
    }

    /// The wallpaper group is p2, NOT pgg, for Stein's unique tiling:
    /// the translation lattice is a generic oblique lattice — w1·w2 ≠ 0
    /// (not rectangular) and |w1| ≠ |w2| (not rhombic) — so the
    /// normaliser of the lattice in O(2) is exactly {I, −I} and NO
    /// orientation-reversing isometry (mirror or glide) can normalise
    /// the translations at all.  Together with the verified half-turn
    /// (`pentagon_14_c2_symmetry`) this pins the group to p2 (orbifold
    /// 2222).  The user-reported "standard tilings are pgg" applies to
    /// the Rice types 9–13 (all pgg here on rectangular lattices,
    /// modulo chiral-pair colouring); a pgg group would require a
    /// rectangular translation lattice, which this oblique lattice is
    /// not — and Stein's tiling is unique up to the tile's own
    /// symmetries (MathWorld, Grünbaum), so no alternative pgg packing
    /// exists for this member.
    #[test]
    fn pentagon_14_lattice_rules_out_pg_glide_groups() {
        let (w1, w2) = (w1(), w2());
        let dot = w1.x * w2.x + w1.y * w2.y;
        let n1 = (w1.x * w1.x + w1.y * w1.y).sqrt();
        let n2 = (w2.x * w2.x + w2.y * w2.y).sqrt();
        // Generic oblique: not perpendicular, not equal-length.
        assert!(
            dot.abs() > 1e-3 * n1 * n2,
            "w1·w2 ≠ 0 (|cos| ≈ {:.3}): not a rectangular lattice, so no \
             pmm/pgg-family group can realise on it",
            dot / (n1 * n2)
        );
        assert!(
            (n1 / n2 - 1.0).abs() > 1e-3,
            "|w1| ≠ |w2| ({:.4} vs {:.4}): not a rhombic lattice, so no \
             cmm/p4-family point symmetry can normalise it",
            n1,
            n2
        );
        // |cos| far from 1 too: genuinely oblique, not degenerate.
        assert!((dot / (n1 * n2)).abs() < 0.999);
    }
}

#[cfg(test)]
mod pentagon15_tests {
    use super::pentagon15::{s3, side_c, tile_area};
    use crate::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon15::spec;
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Vertex-set key on a 1e-6 grid (distinct tiling vertices are ≥ 1
    /// apart; two computations of a shared joint agree to ~1e-15).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Verify that the isometry g = (M, t) preserves the tiling: the
    /// image of every tile intersecting `ra` must be a tile intersecting
    /// `rb` (the axis-aligned box of g(ra)).  With `exact = true` the
    /// image set must equal the `rb` tile set (g(ra) = rb as sets, the
    /// case of axis-aligned rectangles).
    fn check_symmetry(
        t: &LatticeTiling,
        params: &tess_core::params::ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            Vec2::new(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            Vec2::new(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ra_tiles = t.generate(params, &Query { region: ra });
        let rb_tiles = t.generate(params, &Query { region: rb });
        assert!(ra_tiles.message.is_none(), "{label}: {:?}", ra_tiles.message);
        assert!(rb_tiles.message.is_none(), "{label}: {:?}", rb_tiles.message);
        assert!(!ra_tiles.tiles.is_empty(), "{label}: no tiles");
        let set_b: std::collections::HashSet<Vec<(i64, i64)>> =
            rb_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let set_a: std::collections::HashSet<Vec<(i64, i64)>> = ra_tiles
            .tiles
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(
            set_a.len(),
            ra_tiles.tiles.len(),
            "{label}: isometry must be injective on the patch"
        );
        if exact {
            assert_eq!(
                set_a, set_b,
                "{label}: the isometry must preserve the tiling"
            );
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    #[test]
    fn pentagon_15_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(
            &t,
            "pentagon-15",
            &p,
            BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn pentagon_15_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-15(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(7.0, 7.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_15_deterministic() {
        check_det(
            &spec(),
            "pentagon-15",
            BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)),
        );
    }

    /// Unit structure of the primitive cell: 12 tiles of the exact
    /// Q(√3) prototile (A = 150°, B = 60°, C = 135°, D = 105°, E = 90°;
    /// a = 2, b = d = e = 1, c = √(2+√3)) on the rectangular lattice
    /// w1 = (−(1+√3)/2, (3+√3)/2) ⊥ w2 = ((9+6√3)/2, 3+3√3/2), with
    /// |det| = 12 × tile area = 15 + 9√3.
    #[test]
    fn pentagon_15_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-15");
        assert_eq!(t.cells.len(), 12, "the primitive unit holds twelve tiles");
        let s3 = s3();
        assert_eq!(
            t.lattice[0],
            Vec2::new(-(1.0 + s3) / 2.0, (3.0 + s3) / 2.0),
            "w1 = (−(1+√3)/2, (3+√3)/2)"
        );
        assert_eq!(
            t.lattice[1],
            Vec2::new((9.0 + 6.0 * s3) / 2.0, (6.0 + 3.0 * s3) / 2.0),
            "w2 = ((9+6√3)/2, 3+3√3/2)"
        );
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        assert!(
            (w1.dot(w2)).abs() < 1e-12,
            "lattice must be rectangular (w1 ⊥ w2)"
        );
        let det = w1.cross(w2);
        assert!(
            (det.abs() - 12.0 * tile_area()).abs() < 1e-12,
            "|det| {det} ≠ 12 × tile area = 15+9√3"
        );
        assert!(
            ((w1.x * w1.x + w1.y * w1.y) - (4.0 + 2.0 * s3)).abs() < 1e-12,
            "|w1|² = 4+2√3"
        );
        assert!(
            ((w2.x * w2.x + w2.y * w2.y) - (63.0 + 36.0 * s3)).abs() < 1e-12,
            "|w2|² = 63+36√3"
        );
        assert_eq!(t.cells[0].poly.pts[0], Vec2::ZERO, "T1's corner B sits at the origin");
        // Seeds follow (k−1) mod 3 — the three full-group isohedral
        // orbits {T1,T4,T7,T10}, {T2,T5,T8,T11}, {T3,T6,T9,T12}.
        for (k, cell) in t.cells.iter().enumerate() {
            assert_eq!(cell.seed, (k as u32) % 3, "cell {k} seed");
        }
        // Shipped prototile fingerprints: from corner B, CCW reads
        // (b,c,d,e,a) = (1, c, 1, 1, 2) with angles (B,C,D,E,A) =
        // (60°,135°,105°,90°,150°); the mirror cells read the reverses.
        let c = side_c();
        let fwd = (
            [1.0, c, 1.0, 1.0, 2.0],
            [
                std::f64::consts::FRAC_PI_3,
                3.0 * std::f64::consts::PI / 4.0,
                7.0 * std::f64::consts::PI / 12.0,
                std::f64::consts::FRAC_PI_2,
                5.0 * std::f64::consts::PI / 6.0,
            ],
        );
        let rev = (
            [1.0, 1.0, c, 1.0, 2.0],
            [
                5.0 * std::f64::consts::PI / 6.0,
                std::f64::consts::FRAC_PI_2,
                7.0 * std::f64::consts::PI / 12.0,
                3.0 * std::f64::consts::PI / 4.0,
                std::f64::consts::FRAC_PI_3,
            ],
        );
        let mut seen: std::collections::HashSet<Vec<(i64, i64)>> =
            std::collections::HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(
                (cell.poly.area() - tile_area()).abs() < 1e-12,
                "cell {k} area ≠ (5+3√3)/4"
            );
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            let n = cell.poly.len();
            assert_eq!(n, 5, "cell {k} vertex count");
            let sides: Vec<f64> = (0..n)
                .map(|i| (cell.poly.pts[(i + 1) % n] - cell.poly.pts[i]).length())
                .collect();
            let angles: Vec<f64> = (0..n)
                .map(|i| {
                    let (p0, p1, p2) = (
                        cell.poly.pts[(i + n - 1) % n],
                        cell.poly.pts[i],
                        cell.poly.pts[(i + 1) % n],
                    );
                    let (v1, v2) = (p0 - p1, p2 - p1);
                    (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
                })
                .collect();
            let match_f = sides.iter().zip(fwd.0).all(|(x, w)| (x - w).abs() < 1e-12)
                && angles.iter().zip(fwd.1).all(|(x, w)| (x - w).abs() < 1e-9);
            let match_r = sides.iter().zip(rev.0).all(|(x, w)| (x - w).abs() < 1e-12)
                && angles.iter().zip(rev.1).all(|(x, w)| (x - w).abs() < 1e-9);
            assert!(
                match_f || match_r,
                "cell {k} is not a congruent type-15 prototile"
            );
            assert!(
                seen.insert(vkey(&cell.poly.pts)),
                "cell {k} duplicates another cell"
            );
        }
    }

    /// 3-isohedral: seeds 0, 1, 2 are the three full-group isohedral
    /// classes (four tiles each in the primitive cell) and every
    /// generated tile keeps the prototile's side multiset
    /// {1, 1, 1, √(2+√3), 2}.
    #[test]
    fn pentagon_15_isohedral_classes() {
        let t = spec();
        let mut counts = [0usize; 3];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(counts, [4, 4, 4], "four tiles per isohedral class");
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region: BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let c = side_c();
        let mut seen = [false; 3];
        for tile in &r.tiles {
            let s = tile.seed as usize;
            assert!(s < 3, "seeds must stay in {{0, 1, 2}}");
            seen[s] = true;
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| {
                    (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length()
                })
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [1.0, 1.0, 1.0, c, 2.0];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!(
                    (got - w).abs() < 1e-12,
                    "tile side {got} ∉ {{1, c, 2}}"
                );
            }
        }
        assert!(seen[0] && seen[1] && seen[2], "all three classes must appear");
    }

    /// The wallpaper group is pgg: the half-turn about the 2-fold
    /// centre c0 = (√3, −1/2) and the two perpendicular glide
    /// reflections (axes ∥ w1 and ∥ w2) preserve the tiling, and no
    /// pure reflection does.  The glide representatives are the
    /// placement isometries of cells T4 and T10:
    /// g1 = (M_b, (−(5+√3)/2, (1−√3)/2)) with g1² = w1,
    /// g2 = (M_a, ((5+5√3)/2, (√3−3)/2)) with g2² = w2.
    #[test]
    fn pentagon_15_pgg_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let s3 = s3();
        // (i) 180° rotation about c0 = (√3, −1/2): g(A) = A as sets.
        let c = Vec2::new(s3, -1.0 / 2.0);
        let rot = |q: Vec2| c * 2.0 - q;
        let ra = BBox::new(v2(-4.0, -4.0), v2(10.0, 10.0));
        let cs: Vec<Vec2> = ra.corners().iter().map(|&q| rot(q)).collect();
        let rb = BBox::new(
            Vec2::new(
                cs.iter().map(|q| q.x).fold(f64::MAX, f64::min),
                cs.iter().map(|q| q.y).fold(f64::MAX, f64::min),
            ),
            Vec2::new(
                cs.iter().map(|q| q.x).fold(f64::MIN, f64::max),
                cs.iter().map(|q| q.y).fold(f64::MIN, f64::max),
            ),
        );
        let ra_tiles = t.generate(&p, &Query { region: ra });
        let rb_tiles = t.generate(&p, &Query { region: rb });
        assert!(ra_tiles.message.is_none(), "{:?}", ra_tiles.message);
        assert!(rb_tiles.message.is_none(), "{:?}", rb_tiles.message);
        let set_b: std::collections::HashSet<Vec<(i64, i64)>> =
            rb_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let set_a: std::collections::HashSet<Vec<(i64, i64)>> = ra_tiles
            .tiles
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|q| rot(*q)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(
            set_a.len(),
            ra_tiles.tiles.len(),
            "rotation must be injective on the patch"
        );
        assert_eq!(
            set_a, set_b,
            "180° rotation about the 2-fold centre c0 must preserve the tiling"
        );
        // (ii) the two perpendicular glides (image window is tilted, so
        // check the image set is contained in the tilted-window patch).
        let ma = (0.5, s3 / 2.0, s3 / 2.0, -0.5);
        let mb = (-0.5, -s3 / 2.0, -s3 / 2.0, 0.5);
        check_symmetry(
            &t,
            &p,
            mb,
            Vec2::new(-(5.0 + s3) / 2.0, (1.0 - s3) / 2.0),
            BBox::new(v2(-4.0, -4.0), v2(10.0, 10.0)),
            false,
            "glide g1 (axis ∥ w1)",
        );
        check_symmetry(
            &t,
            &p,
            ma,
            Vec2::new((5.0 + 5.0 * s3) / 2.0, (s3 - 3.0) / 2.0),
            BBox::new(v2(-4.0, -4.0), v2(10.0, 10.0)),
            false,
            "glide g2 (axis ∥ w2)",
        );
        // (iii) no pure reflection: ker(I+M_a) is the 120° line = the
        // w1 direction, ker(I+M_b) the 30° line = the w2 direction, so a
        // pure reflection (M, t) would need t = i·w1 (resp. i·w2).  No
        // such candidate may map the prototile T1 onto any tile.
        let big = BBox::new(v2(-26.0, -19.0), v2(26.0, 19.0));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        let set: std::collections::HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        for i in -2..=2 {
            // sanity: pure lattice translations are symmetries
            let tr1: Vec<Vec2> = prot.iter().map(|q| *q + w1 * i as f64).collect();
            let tr2: Vec<Vec2> = prot.iter().map(|q| *q + w2 * i as f64).collect();
            assert!(set.contains(&vkey(&tr1)), "T1 + {i}·w1 must be a tile");
            assert!(set.contains(&vkey(&tr2)), "T1 + {i}·w2 must be a tile");
            let img1: Vec<Vec2> =
                prot.iter().map(|q| apply_iso(ma, w1 * i as f64, *q)).collect();
            let img2: Vec<Vec2> =
                prot.iter().map(|q| apply_iso(mb, w2 * i as f64, *q)).collect();
            assert!(
                !set.contains(&vkey(&img1)),
                "(M_a, {i}·w1) must not be a symmetry (no mirrors in pgg)"
            );
            assert!(
                !set.contains(&vkey(&img2)),
                "(M_b, {i}·w2) must not be a symmetry (no mirrors in pgg)"
            );
        }
    }

    /// Window census (default parameters): culling is the tile AABB
    /// intersecting the region (+1e-9); every shipped coordinate lies on
    /// the (Z + Z√3)/2 grid, at least 0.0359 away from any integer
    /// boundary, so the counts are exact.
    #[test]
    fn pentagon_15_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::Pgg);
        assert_eq!(t.lattice_kind, LatticeKind::Rectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the T1 cell (seed 0)");
        let p = ParamSet::default_for(t.params());
        for (region, want) in [
            (BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)), 93),
            (BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)), 89),
            (BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)), 45),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }
}

#[cfg(test)]
mod pentagon9_tests {
    use super::pentagon9::{s3, tile_area};
    use crate::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon9::spec;
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Vertex-set key on a 1e-6 grid (distinct tiling vertices are ≥ 1/7
    /// apart — all coordinates lie on the (Z + Z√3)/7 grid — and two
    /// computations of a shared joint agree to ~1e-15).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Single-vertex key on the same 1e-6 grid.
    fn pkey(p: Vec2) -> (i64, i64) {
        ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64)
    }

    /// Verify that the isometry g = (M, t) preserves the tiling: the
    /// image of every tile intersecting `ra` must be a tile intersecting
    /// `rb` (the axis-aligned box of g(ra)).  With `exact = true` the
    /// image set must equal the `rb` tile set (the case g(ra) = rb).
    fn check_symmetry(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            Vec2::new(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            Vec2::new(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ra_tiles = t.generate(params, &Query { region: ra });
        let rb_tiles = t.generate(params, &Query { region: rb });
        assert!(ra_tiles.message.is_none(), "{label}: {:?}", ra_tiles.message);
        assert!(rb_tiles.message.is_none(), "{label}: {:?}", rb_tiles.message);
        assert!(!ra_tiles.tiles.is_empty(), "{label}: no tiles");
        let set_b: std::collections::HashSet<Vec<(i64, i64)>> =
            rb_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let set_a: std::collections::HashSet<Vec<(i64, i64)>> = ra_tiles
            .tiles
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(
            set_a.len(),
            ra_tiles.tiles.len(),
            "{label}: isometry must be injective on the patch"
        );
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    /// Directed-edge key: keys of its two endpoints on the 1e-6 grid.
    type EdgeKey = ((i64, i64), (i64, i64));

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    #[test]
    fn pentagon_9_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(
            &t,
            "pentagon-9",
            &p,
            BBox::new(v2(-25.0, -20.0), v2(25.0, 20.0)),
        );
    }

    #[test]
    fn pentagon_9_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-9(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(7.0, 7.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_9_deterministic() {
        check_det(
            &spec(),
            "pentagon-9",
            BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)),
        );
    }

    /// Unit structure of the primitive cell: 8 tiles of the exact Q(√3)
    /// prototile (A = C = 120°, cos B = 1/7, cos D = 23/98,
    /// cos E = −11/14; a = 6, b = c = d = e = 14) on the rectangular
    /// lattice w1 = (−30, 6√3) ⊥ w2 = (−141/7, −235√3/7), with
    /// |det| = 8 × tile area = 1128·√3, |w1|² = 1008 and
    /// |w2|² = 185556/49.  Every cell is a congruent copy of the
    /// prototile (cyclic (side, angle) fingerprint forward or reversed),
    /// CCW, and no two cells coincide.
    #[test]
    fn pentagon_9_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-9");
        assert_eq!(t.cells.len(), 8, "the primitive unit holds eight tiles");
        let s = s3();
        assert_eq!(t.lattice[0], Vec2::new(-30.0, 6.0 * s), "w1 = (−30, 6√3)");
        assert_eq!(
            t.lattice[1],
            Vec2::new(-141.0 / 7.0, -235.0 / 7.0 * s),
            "w2 = (−141/7, −235√3/7)"
        );
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        assert!((w1.dot(w2)).abs() < 1e-12, "lattice must be rectangular (w1 ⊥ w2)");
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        assert!(
            (det - 8.0 * tile_area()).abs() < 1e-12,
            "|det| {det} ≠ 8 × tile area = 1128√3"
        );
        assert!(
            ((w1.x * w1.x + w1.y * w1.y) - 1008.0).abs() < 1e-12,
            "|w1|² = 900 + 108 = 1008"
        );
        assert!(
            ((w2.x * w2.x + w2.y * w2.y) - 185556.0 / 49.0).abs() < 1e-9,
            "|w2|² = (141² + 3·235²)/49 = 185556/49"
        );
        // Seeds follow the two full-group isohedral orbits
        // {T1,T4,T5,T8} (0) and {T2,T3,T6,T7} (1).
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 1, 1, 0, 0, 1, 1, 0], "isohedral seed pattern");
        // The shipped prototile is T1, corner A at the origin:
        // A = (0,0), B = (14,0), C = (12,8√3), D = (−1,11√3), E = (−3,3√3).
        let proto = &t.cells[0].poly;
        for (i, want) in [
            (0.0, 0.0),
            (14.0, 0.0),
            (12.0, 8.0 * s),
            (-1.0, 11.0 * s),
            (-3.0, 3.0 * s),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(proto.pts[i], v2(want.0, want.1), "prototile corner {i}");
        }
        // Exact corner angle cosines in vertex order A, B, C, D, E.
        let cos = |i: usize| angle_at(proto, i).cos();
        assert!((cos(0) + 0.5).abs() < 1e-12, "cos A = −1/2");
        assert!((cos(1) - 1.0 / 7.0).abs() < 1e-12, "cos B = 1/7");
        assert!((cos(2) + 0.5).abs() < 1e-12, "cos C = −1/2");
        assert!((cos(3) - 23.0 / 98.0).abs() < 1e-12, "cos D = 23/98");
        assert!((cos(4) + 11.0 / 14.0).abs() < 1e-12, "cos E = −11/14");
        let ps = [
            side_of(proto, 0),
            side_of(proto, 1),
            side_of(proto, 2),
            side_of(proto, 3),
            side_of(proto, 4),
        ];
        let pa = [
            angle_at(proto, 0),
            angle_at(proto, 1),
            angle_at(proto, 2),
            angle_at(proto, 3),
            angle_at(proto, 4),
        ];
        let mut seen: std::collections::HashSet<Vec<(i64, i64)>> =
            std::collections::HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(
                (cell.poly.area() - tile_area()).abs() < 1e-12,
                "cell {k} area ≠ 141√3"
            );
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            let n = cell.poly.len();
            assert_eq!(n, 5, "cell {k} vertex count");
            // Side multiset {6, 14, 14, 14, 14}.
            let mut sides: Vec<f64> = (0..n).map(|i| side_of(&cell.poly, i)).collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [6.0, 14.0, 14.0, 14.0, 14.0];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "cell {k} side {got} ∉ {{6, 14}}");
            }
            // Cyclic (side, angle) fingerprint against the prototile,
            // forward or reversed (the cells realise both chiralities).
            let cs = (0..n).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..n).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            let rev = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(rot + 9 - i) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(rot + 5 - i) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd || rev, "cell {k} is not a congruent type-9 prototile");
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
        }
    }

    /// Edge-to-edge at the pentagon level over a 61-tile patch: no
    /// directed side is shared by two tiles, every side whose midpoint is
    /// strictly inside the window is matched by exactly one full side of
    /// one neighbour (reversed), and no vertex ever falls in the interior
    /// of a non-incident side (no T-junctions; the closest genuine
    /// near-miss in the patch is 13.6 units away).
    #[test]
    fn pentagon_9_edge_to_edge_gluing() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let big = BBox::new(v2(-60.0, -45.0), v2(60.0, 45.0));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 61, "patch size");
        let mut dirs: std::collections::HashMap<EdgeKey, usize> =
            std::collections::HashMap::new();
        for tile in &r.tiles {
            let n = tile.poly.len();
            for i in 0..n {
                let k = (pkey(tile.poly.pts[i]), pkey(tile.poly.pts[(i + 1) % n]));
                *dirs.entry(k).or_insert(0) += 1;
            }
        }
        assert!(
            dirs.values().all(|&c| c == 1),
            "a directed side may belong to only one tile"
        );
        for tile in &r.tiles {
            let n = tile.poly.len();
            for i in 0..n {
                let (a, b) = (tile.poly.pts[i], tile.poly.pts[(i + 1) % n]);
                let mid = (a + b) * 0.5;
                let d = (mid.x - big.min.x)
                    .min(big.max.x - mid.x)
                    .min(mid.y - big.min.y)
                    .min(big.max.y - mid.y);
                if d > 1e-6 {
                    assert_eq!(
                        dirs.get(&(pkey(b), pkey(a))),
                        Some(&1),
                        "interior side {a} -> {b} lacks its full-edge partner"
                    );
                }
            }
        }
        // No T-junction: no vertex in the interior of a non-incident edge.
        let verts: std::collections::HashSet<(i64, i64)> =
            r.tiles.iter().flat_map(|tile| tile.poly.pts.iter().map(|&p| pkey(p))).collect();
        for tile in &r.tiles {
            let n = tile.poly.len();
            for i in 0..n {
                let (a, b) = (tile.poly.pts[i], tile.poly.pts[(i + 1) % n]);
                let (dx, dy) = (b.x - a.x, b.y - a.y);
                let l2 = dx * dx + dy * dy;
                for vk in verts.iter().filter(|k| **k != pkey(a) && **k != pkey(b)) {
                    let (px, py) = (vk.0 as f64 / 1e6, vk.1 as f64 / 1e6);
                    let tt = (((px - a.x) * dx + (py - a.y) * dy) / l2).clamp(0.0, 1.0);
                    if tt == 0.0 || tt == 1.0 {
                        continue;
                    }
                    let q = a + (b - a) * tt;
                    assert!(
                        (px - q.x).hypot(py - q.y) > 1e-7,
                        "vertex {vk:?} lies in the interior of side {a} -> {b}"
                    );
                }
            }
        }
    }

    /// Vertex census: every window-interior vertex is a complete star of
    /// incident tile angles summing to exactly 360°, and the stars are
    /// exactly the three closed types {A,A,C} = 3·120°, {D,E,E} =
    /// 76.43° + 2·141.79° and {B,B,C,D} = 2·81.79° + 120° + 76.43°, all
    /// three present.
    #[test]
    fn pentagon_9_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let big = BBox::new(v2(-60.0, -45.0), v2(60.0, 45.0));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        let verts: std::collections::HashSet<(i64, i64)> =
            r.tiles.iter().flat_map(|tile| tile.poly.pts.iter().map(|&pt| pkey(pt))).collect();
        // Corner cosines of the prototile, A = C, B, D, E.
        let bins = [-0.5, 1.0 / 7.0, 23.0 / 98.0, -11.0 / 14.0];
        let mut stars: std::collections::BTreeMap<String, usize> =
            std::collections::BTreeMap::new();
        let mut checked = 0;
        for vk in &verts {
            let (px, py) = (vk.0 as f64 / 1e6, vk.1 as f64 / 1e6);
            let d = (px - big.min.x)
                .min(big.max.x - px)
                .min(py - big.min.y)
                .min(big.max.y - py);
            if d < 1e-6 {
                continue; // boundary: incident tiles may be culled
            }
            // Any tile having this corner has its AABB containing the
            // strictly interior point, hence was generated: the star is
            // complete.
            let mut letters = String::new();
            let mut sum = 0.0;
            for tile in &r.tiles {
                let n = tile.poly.len();
                for i in 0..n {
                    if pkey(tile.poly.pts[i]) != *vk {
                        continue;
                    }
                    let ang = angle_at(&tile.poly, i);
                    sum += ang;
                    let c = ang.cos();
                    let j = (0..4)
                        .min_by_key(|&j| ((c - bins[j]).abs() * 1e12) as i64)
                        .unwrap();
                    assert!((c - bins[j]).abs() < 1e-9, "stray corner cosine {c}");
                    letters.push(['A', 'B', 'D', 'E'][j]);
                }
            }
            assert!(
                (sum - std::f64::consts::TAU).abs() < 1e-9,
                "vertex {vk:?} angles sum to {sum} ≠ 2π"
            );
            let mut cs: Vec<char> = letters.chars().collect();
            cs.sort_unstable();
            let star: String = cs.into_iter().collect();
            assert!(
                star == "AAA" || star == "ABBD" || star == "DEE",
                "unexpected vertex star {star} at {vk:?}"
            );
            *stars.entry(star).or_insert(0) += 1;
            checked += 1;
        }
        assert!(checked >= 50, "expected a substantial interior census, got {checked}");
        for star in ["AAA", "ABBD", "DEE"] {
            assert!(stars.contains_key(star), "star type {star} never appears");
        }
    }

    /// 2-isohedral: seeds 0/1 are the two full-group isohedral classes,
    /// four tiles each in the primitive cell, both appear in every patch,
    /// and every generated tile keeps the prototile side multiset
    /// {6, 14, 14, 14, 14}.
    #[test]
    fn pentagon_9_isohedral_classes() {
        let t = spec();
        let mut counts = [0usize; 2];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(counts, [4, 4], "four tiles per isohedral class");
        let params = ParamSet::default_for(t.params());
        let r = t.generate(&params, &Query { region: BBox::new(v2(-60.0, -45.0), v2(60.0, 45.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let mut seen = [false; 2];
        for tile in &r.tiles {
            let s = tile.seed as usize;
            assert!(s < 2, "seeds must stay in {{0, 1}}");
            seen[s] = true;
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length())
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [6.0, 14.0, 14.0, 14.0, 14.0];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "seed {s} side {got} ∉ {{6, 14}}");
            }
        }
        assert!(seen[0] && seen[1], "both isohedral classes must appear");
    }

    /// The wallpaper group is pgg: the half-turn about the 2-fold centre
    /// c0 = (13, 4√3) preserves the tiling exactly, and the two
    /// perpendicular glide reflections
    /// g1 = (M_v, (167/7, −165√3/7)), g1² = w2,
    /// g2 = (M_u, (12, −8√3)),          g2² = −w1,
    /// with M_v the reflection in the w2 direction and M_u = −M_v the
    /// reflection in the w1 direction, preserve it.  Every symmetry with
    /// linear part M_v (resp. M_u) shifts by exactly half a primitive
    /// translation along its axis (the glide g1 (resp. g2) and all its
    /// lattice conjugates), so no pure reflection exists: none of the
    /// candidates (M, i·w) maps the prototile onto a tile.  All
    /// generators are exact Q(√3).
    #[test]
    fn pentagon_9_pgg_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let s = s3();
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let c0 = Vec2::new(13.0, 4.0 * s);
        // A window symmetric about c0: the half-turn must give exact
        // set equality of the tile patch.
        let ra = BBox::new(
            v2(c0.x - 30.0, c0.y - 24.0),
            v2(c0.x + 30.0, c0.y + 24.0),
        );
        check_symmetry(&t, &p, (-1.0, 0.0, 0.0, -1.0), c0 * 2.0, ra, true, "half-turn about c0");
        // The two perpendicular glides (image window is tilted, so check
        // the image set is contained in the tilted-window patch).
        let mv = (-11.0 / 14.0, 5.0 * s / 14.0, 5.0 * s / 14.0, 11.0 / 14.0);
        let mu = (11.0 / 14.0, -5.0 * s / 14.0, -5.0 * s / 14.0, -11.0 / 14.0);
        let gw1 = Vec2::new(167.0 / 7.0, -165.0 / 7.0 * s);
        let gw2 = Vec2::new(12.0, -8.0 * s);
        // Exact generator identities (floats of exact Q(√3) expressions).
        let (a1, b1, c1, d1) = mv;
        let gs1 = v2((a1 + 1.0) * gw1.x + b1 * gw1.y, c1 * gw1.x + (d1 + 1.0) * gw1.y);
        assert!((gs1 - w2).length() < 1e-9, "g1² must equal w2 exactly");
        let (a2, b2, c2, d2) = mu;
        let gs2 = v2((a2 + 1.0) * gw2.x + b2 * gw2.y, c2 * gw2.x + (d2 + 1.0) * gw2.y);
        assert!((gs2 + w1).length() < 1e-9, "g2² must equal −w1 exactly");
        check_symmetry(&t, &p, mv, gw1, ra, false, "glide g1 (axis ∥ w2)");
        check_symmetry(&t, &p, mu, gw2, ra, false, "glide g2 (axis ∥ w1)");
        // (M_v, w1 − w2)-style shifts: composing the glide with lattice
        // translations keeps it a symmetry — images of the prototile must
        // be tiles.
        let big = BBox::new(v2(-60.0, -45.0), v2(60.0, 45.0));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        let set: std::collections::HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();
        for i in -2..=2 {
            // sanity: pure lattice translations are symmetries
            let tr1: Vec<Vec2> = prot.iter().map(|q| *q + w1 * i as f64).collect();
            assert!(set.contains(&vkey(&tr1)), "T1 + {i}·w1 must be a tile");
        }
        for i in [0.0, -1.0] {
            let img: Vec<Vec2> = prot
                .iter()
                .map(|q| apply_iso(mv, gw1 + w2 * i, *q))
                .collect();
            assert!(
                set.contains(&vkey(&img)),
                "(M_v, g1's shift {i}·w2) must map T1 onto a tile"
            );
        }
        assert!(
            set.contains(&vkey(
                &prot.iter().map(|q| apply_iso(mu, gw2, *q)).collect::<Vec<_>>()
            )),
            "(M_u, w2-shift) must map T1 onto a tile"
        );
        // No pure reflection: every candidate (M, i·w) — zero or whole
        // lattice shifts along either axis — must fail to map T1 onto a
        // tile (all true symmetries with linear part M shift half a
        // primitive translation along the axis).
        for i in -2..=2 {
            for (m, w, mname, wname) in [
                (mv, w1, "M_v", "w1"),
                (mv, w2, "M_v", "w2"),
                (mu, w1, "M_u", "w1"),
                (mu, w2, "M_u", "w2"),
            ] {
                let img: Vec<Vec2> = prot
                    .iter()
                    .map(|q| apply_iso(m, w * i as f64, *q))
                    .collect();
                assert!(
                    !set.contains(&vkey(&img)),
                    "({mname}, {i}·{wname}) must not be a symmetry (no mirrors in pgg)"
                );
            }
        }
    }

    /// Window census (default parameters): culling is the tile AABB
    /// intersecting the region (+1e-9); every shipped coordinate lies on
    /// the (Z + Z√3)/7 grid, at least 1/7 away from any chosen integer
    /// boundary in x and ≥ 4.2e-4 in y, so the counts are exact and
    /// match the independent Python emulation of the lattice.rs culling.
    #[test]
    fn pentagon_9_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::Pgg);
        assert_eq!(t.lattice_kind, LatticeKind::Rectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the T1 cell (seed 0)");
        let p = ParamSet::default_for(t.params());
        for (region, want) in [
            (BBox::new(v2(-60.0, -45.0), v2(60.0, 45.0)), 61),
            (BBox::new(v2(-40.0, -30.0), v2(40.0, 30.0)), 32),
            (BBox::new(v2(-25.0, -20.0), v2(25.0, 20.0)), 16),
            (BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)), 4),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }
}

#[cfg(test)]
mod pentagon11_tests {
    use super::pentagon11::{cell_table, prototile, ratios, two_fold_centre};
    use crate::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon11::spec;
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Reflection matrix in the direction u (u must be unit): 2uuᵀ − I.
    fn mirror(u: Vec2) -> (f64, f64, f64, f64) {
        (2.0 * u.x * u.x - 1.0, 2.0 * u.x * u.y, 2.0 * u.x * u.y, 2.0 * u.y * u.y - 1.0)
    }

    /// Vertex-set key on a 1e-6 grid (distinct tiling vertices are ≥ 0.09
    /// apart — the shortest side is a = 0.0947 — and two computations of a
    /// shared joint agree to ~1e-15 because every cell is built from the
    /// same runtime prototile array).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    /// True when the tile's AABB lies inside `w`.  A tile whose AABB is
    /// inside `ra` has its polygon inside `ra`, so its image under any
    /// isometry g intersects g(ra) ⊆ bbox(g(ra)) — and a tile whose polygon
    /// intersects the region always lies inside the generator's padded
    /// index range — hence the image is always generated for rb.  Tiles
    /// whose AABB merely touches the boundary (polygon outside) are
    /// excluded: their images legitimately escape rb.
    fn aabb_inside(poly: &Polygon, w: BBox) -> bool {
        let bb = poly.aabb();
        bb.min.x >= w.min.x
            && bb.min.y >= w.min.y
            && bb.max.x <= w.max.x
            && bb.max.y <= w.max.y
    }

    /// Verify that the isometry g = (M, t) preserves the tiling on `ra`:
    /// images of interior-AABB tiles of ra must be tiles generated for the
    /// image window bbox(g(ra)).  With `exact = true` (the half-turn case,
    /// g(ra) = ra) the filtered image set must equal the interior-AABB
    /// tile set of the window.
    fn check_iso(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            v2(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            v2(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ta = t.generate(params, &Query { region: ra });
        let tb = t.generate(params, &Query { region: rb });
        assert!(ta.message.is_none(), "{label}: {:?}", ta.message);
        assert!(tb.message.is_none(), "{label}: {:?}", tb.message);
        let src: Vec<_> = ta.tiles.iter().filter(|tile| aabb_inside(&tile.poly, ra)).collect();
        assert!(!src.is_empty(), "{label}: no interior tiles");
        let set_b: HashSet<Vec<(i64, i64)>> = tb
            .tiles
            .iter()
            .filter(|tile| !exact || aabb_inside(&tile.poly, rb))
            .map(|tile| vkey(&tile.poly.pts))
            .collect();
        let set_a: HashSet<Vec<(i64, i64)>> = src
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(set_a.len(), src.len(), "{label}: isometry must be injective on the patch");
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    #[test]
    fn pentagon_11_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "pentagon-11", &p, BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)));
    }

    #[test]
    fn pentagon_11_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-11(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(7.0, 7.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_11_deterministic() {
        check_det(&spec(), "pentagon-11", BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)));
    }

    /// Unit structure of the primitive cell: 8 congruent copies of the
    /// runtime-computed Kershner prototile (A = 90°, B = 156°, C = 48°,
    /// D = 114°, E = 132°, sides a = r, b = s, c = 1, d = e = 1 + 2r) on
    /// the rectangular lattice w1 ⊥ w2 with det = 8 × tile area.  The
    /// closure identities hold to ~1e-12, every cell is CCW, no two cells
    /// coincide, and the seeds follow the two full-group isohedral orbits.
    #[test]
    fn pentagon_11_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-11");
        assert_eq!(t.cells.len(), 8, "the primitive unit holds eight tiles");
        assert_eq!(t.lattice[0], Vec2::new(-2.1433083997208975, -1.3918807493424277));
        assert_eq!(t.lattice[1], Vec2::new(1.6269879260579216, -2.5053417038859127));
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        assert!(w1.dot(w2).abs() < 1e-12, "lattice must be rectangular (w1 ⊥ w2)");
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        let a0 = t.cells[0].poly.area();
        assert!((det - 8.0 * a0).abs() < 1e-11, "det {det} ≠ 8 × tile area");
        let (r, s, d) = ratios();
        assert!((d - (1.0 + 2.0 * r)).abs() < 1e-15, "d = e = 1 + 2r");
        // The prototile closes: E = (0, a) with edge directions
        // 0°, 24°, 156°, 222° of lengths s, 1, d, d.
        let pr = prototile();
        assert!(pr[4].x.abs() < 1e-12, "closure E.x = 0, got {}", pr[4].x);
        assert!((pr[4].y - r).abs() < 1e-12, "closure E.y = a, got {}", pr[4].y);
        let proto = &t.cells[0].poly;
        assert_eq!(proto.pts[0], v2(0.0, 0.0), "prototile corner A at the origin");
        for (i, want) in [(0usize, 90.0f64), (1, 156.0), (2, 48.0), (3, 114.0), (4, 132.0)] {
            let got = angle_at(proto, i).to_degrees();
            assert!((got - want).abs() < 1e-9, "angle at vertex {i}: {got} ≠ {want}");
        }
        for (i, want) in [(0usize, s), (1, 1.0), (2, d), (3, d), (4, r)] {
            assert!((side_of(proto, i) - want).abs() < 1e-12, "side {i}");
        }
        // Seeds follow the two full-group isohedral orbits (each orbit
        // mixes both chiralities; the glides identify them).
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 1, 0, 1, 0, 1, 1, 0], "isohedral seed pattern");
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        let mut seen: HashSet<Vec<(i64, i64)>> = HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            assert!((cell.poly.area() - a0).abs() < 1e-12, "cell {k} area");
            assert_eq!(cell.poly.len(), 5, "cell {k} vertex count");
            // Side multiset {r, 1, s, d, d}.
            let mut sides: Vec<f64> = (0..5).map(|i| side_of(&cell.poly, i)).collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [r, 1.0, s, d, d];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "cell {k} side {got} ∉ {{r, 1, s, d}}");
            }
            // Cyclic (side, angle) fingerprint against the prototile,
            // forward or reversed (the cells realise both chiralities).
            let cs = (0..5).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..5).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            let rev = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(rot + 9 - i) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(rot + 5 - i) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd || rev, "cell {k} is not a congruent type-11 prototile");
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
        }
    }

    /// Window census (default parameters) and declared metadata: culling
    /// is the tile AABB intersecting the region (+1e-9), matching the
    /// independent Python emulation of the lattice.rs culling.
    #[test]
    fn pentagon_11_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::Pgg);
        assert_eq!(t.lattice_kind, LatticeKind::Rectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the cell-0 tile (seed 0)");
        let p = ParamSet::default_for(t.params());
        for (region, want) in [
            (BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)), 238),
            (BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)), 213),
            (BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)), 116),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }

    /// Refined edge census over 528 tiles (the tiling is NON-edge-to-edge
    /// at the pentagon level): of the sides whose midpoint lies strictly
    /// inside the window, 1882 are matched by exactly one full side and
    /// 210 are d-sides split at exactly the 2a + c parameters c/d and
    /// (c + a)/d; every resulting sub-edge is contained in exactly one
    /// full partner edge (c ↔ [0, c] of a d-side, each a-side inside its
    /// partner d-side, b ↔ b, unsplit d ↔ d).
    #[test]
    fn pentagon_11_refined_edge_census() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let region = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 528, "patch size");
        let (ra, _sb, da) = ratios();
        let verts: Vec<Vec<Vec2>> = r.tiles.iter().map(|tile| tile.poly.pts.clone()).collect();
        let n = verts.len();
        let mut n_split = 0;
        let mut n_full = 0;
        let mut bad: Vec<String> = Vec::new();
        for i in 0..n {
            for e in 0..5 {
                let (pp, qq) = (verts[i][e], verts[i][(e + 1) % 5]);
                let mid = (pp + qq) * 0.5;
                if !(mid.x > -7.5 && mid.x < 13.5 && mid.y > -7.5 && mid.y < 11.5) {
                    continue;
                }
                let dv = qq - pp;
                let l2 = dv.dot(dv);
                let l = l2.sqrt();
                // Split points: other tiles' vertices strictly inside the
                // open segment (collinear within 1e-7), binned at 1e-9 and
                // merged within 1e-8.
                let mut bins: BTreeSet<i64> = BTreeSet::new();
                for (j, vj) in verts.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    for &vv in vj.iter() {
                        let tt = (vv - pp).dot(dv) / l2;
                        if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                            continue;
                        }
                        let cross = (vv.x - pp.x) * dv.y - (vv.y - pp.y) * dv.x;
                        if cross.abs() / l < 1e-7 {
                            bins.insert((tt * 1e9).round() as i64);
                        }
                    }
                }
                let mut srt: Vec<i64> = Vec::new();
                for b in bins {
                    if srt.is_empty() || b - srt[srt.len() - 1] > 10 {
                        srt.push(b);
                    }
                }
                if srt.len() > 2 {
                    bad.push(format!("tile {i} edge {e}: {srt:?}"));
                    continue;
                }
                let mut ends: Vec<f64> = vec![0.0];
                ends.extend(srt.iter().map(|b| *b as f64 / 1e9));
                ends.push(1.0);
                for w in ends.windows(2) {
                    let aq = pp + dv * w[0];
                    let seg = dv * (w[1] - w[0]);
                    let lseg = seg.length();
                    let uh = seg * (1.0 / lseg);
                    let mut cnt = 0;
                    for (j, vj) in verts.iter().enumerate() {
                        if j == i {
                            continue;
                        }
                        for (a1, b1) in (0..5).map(|k| (vj[k], vj[(k + 1) % 5])) {
                            let c1 = (a1.x - aq.x) * uh.y - (a1.y - aq.y) * uh.x;
                            let c2 = (b1.x - aq.x) * uh.y - (b1.y - aq.y) * uh.x;
                            if c1.abs() > 1e-7 || c2.abs() > 1e-7 {
                                continue;
                            }
                            let ta = (a1 - aq).dot(uh);
                            let tb = (b1 - aq).dot(uh);
                            if ta.min(tb) <= 1e-7 * lseg && ta.max(tb) >= lseg * (1.0 - 1e-7) {
                                cnt += 1;
                            }
                        }
                    }
                    if cnt != 1 {
                        bad.push(format!(
                            "tile {i} edge {e} sub [{:.3}, {:.3}] has {cnt} covering edges",
                            w[0], w[1]
                        ));
                    }
                }
                if srt.len() == 2 {
                    n_split += 1;
                    assert!((l - da).abs() < 1e-6, "split side must be a d-side, got {l}");
                    // Split parameters up to edge orientation (mirrored
                    // cells are stored reversed, reading t -> 1 - t):
                    // the joints sit at c and c + a along the d-side.
                    let mut want = [1.0 / da, (1.0 + ra) / da];
                    want.sort_by(|x, y| x.partial_cmp(y).unwrap());
                    let flip = [1.0 - want[1], 1.0 - want[0]];
                    let got: Vec<f64> = srt.iter().map(|b| *b as f64 / 1e9).collect();
                    let matches = |w: &[f64; 2]| {
                        w.iter()
                            .zip(got.iter())
                            .all(|(x, y)| (x - y).abs() < 1e-6)
                    };
                    assert!(
                        matches(&want) || matches(&flip),
                        "split parameters {got:?} ≠ {{c/d, (c+a)/d}} or their mirror"
                    );
                } else {
                    n_full += 1;
                }
            }
        }
        assert!(bad.is_empty(), "refined edge census violations: {bad:?}");
        assert_eq!(n_full, 1882, "full (unsplit) edges");
        assert_eq!(n_split, 210, "d-sides split at the 2a + c joints");
    }

    /// Vertex-figure census over the window (−4, −4)–(8, 8) with margin
    /// 2.6: every interior vertex is a complete star — corner rays plus
    /// rays of edges passing through it (the T-junction points) — merging
    /// within 1e-4 rad, and its sectors are exactly one of the four
    /// figures {C,B,B} (24), {A,A,180°} (25), {D,D,E} (25) and
    /// {C,E,180°} (26), each summing to 360°; 100 vertices in total.
    #[test]
    fn pentagon_11_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let big = BBox::new(v2(-4.0, -4.0), v2(8.0, 8.0));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        const MARG: f64 = 2.6;
        let mut pts: Vec<Vec2> = Vec::new();
        for tile in &r.tiles {
            for q in &tile.poly.pts {
                if q.x > big.min.x + MARG
                    && q.x < big.max.x - MARG
                    && q.y > big.min.y + MARG
                    && q.y < big.max.y - MARG
                    && !pts.iter().any(|o| (*o - *q).length() < 1e-5)
                {
                    pts.push(*q);
                }
            }
        }
        let figs: [(u32, u32, u32); 4] =
            [(48, 132, 180), (48, 156, 156), (90, 90, 180), (114, 114, 132)];
        let mut counts: BTreeMap<(u32, u32, u32), usize> = BTreeMap::new();
        for pt in &pts {
            let mut rays: BTreeSet<i64> = BTreeSet::new();
            for tile in &r.tiles {
                let nn = tile.poly.len();
                for k in 0..nn {
                    if (tile.poly.pts[k] - *pt).length() >= 1e-5 {
                        continue;
                    }
                    for &q in [&tile.poly.pts[(k + nn - 1) % nn], &tile.poly.pts[(k + 1) % nn]] {
                        let dir = q - *pt;
                        let th = dir.y.atan2(dir.x).rem_euclid(std::f64::consts::TAU);
                        rays.insert((th * 1e5).round() as i64);
                    }
                }
                for e in 0..nn {
                    let (a, b) = (tile.poly.pts[e], tile.poly.pts[(e + 1) % nn]);
                    let ab = b - a;
                    let l2 = ab.dot(ab);
                    let tt = (*pt - a).dot(ab) / l2;
                    if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                        continue;
                    }
                    if (*pt - a).cross(ab).abs() / l2.sqrt() < 1e-7 {
                        let th0 = ab.y.atan2(ab.x);
                        for add in [0.0, std::f64::consts::PI] {
                            let th = (th0 + add).rem_euclid(std::f64::consts::TAU);
                            rays.insert((th * 1e5).round() as i64);
                        }
                    }
                }
            }
            let mut mr: Vec<f64> = Vec::new();
            for k in &rays {
                let rv = *k as f64 / 1e5;
                if mr.is_empty() || rv - mr[mr.len() - 1] > 1e-4 {
                    mr.push(rv);
                }
            }
            if mr.len() > 1 && mr[mr.len() - 1] - mr[0] > std::f64::consts::TAU - 1e-4 {
                mr.pop();
            }
            let mut gaps: Vec<f64> = (0..mr.len())
                .map(|i| (mr[(i + 1) % mr.len()] - mr[i]).rem_euclid(std::f64::consts::TAU))
                .collect();
            gaps.retain(|g| *g > 1e-4);
            gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let fig: Vec<u32> = gaps.iter().map(|g| g.to_degrees().round() as u32).collect();
            assert_eq!(fig.len(), 3, "vertex {pt:?}: unexpected figure {fig:?}");
            assert_eq!(fig.iter().sum::<u32>(), 360, "vertex {pt:?} figure {fig:?}");
            let f3 = (fig[0], fig[1], fig[2]);
            assert!(figs.contains(&f3), "unexpected vertex figure {f3:?} at {pt:?}");
            *counts.entry(f3).or_insert(0) += 1;
        }
        assert_eq!(pts.len(), 100, "total interior vertices");
        let want: BTreeMap<(u32, u32, u32), usize> = [
            ((48, 132, 180), 26),
            ((48, 156, 156), 24),
            ((90, 90, 180), 25),
            ((114, 114, 132), 25),
        ]
        .into_iter()
        .collect();
        assert_eq!(counts, want, "vertex-figure census");
    }

    /// The wallpaper group is pgg: the half-turn about c0 preserves the
    /// tiling exactly (interior-filtered set equality), the two
    /// perpendicular glides g1 = (M1, t1), g2 = (M2, t2) with axes along
    /// w1 and w2 preserve it (with every lattice-shifted variant, and so
    /// do their inverses), g1² = w1, g2² = −w2, and g2∘g1 is the half-turn
    /// about c0 itself.  Each glide shifts by exactly half a primitive
    /// translation along its axis, so no pure reflection exists: neither
    /// the perpendicular-part candidate (M, t − (t·u)u) nor any whole
    /// lattice shift (M, i·w) maps the prototile onto a tile.
    #[test]
    fn pentagon_11_pgg_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let c0 = two_fold_centre();
        let ra = BBox::new(v2(c0.x - 12.0, c0.y - 10.0), v2(c0.x + 12.0, c0.y + 10.0));
        check_iso(&t, &p, (-1.0, 0.0, 0.0, -1.0), c0 * 2.0, ra, true, "half-turn about c0");
        let u1 = w1 * (1.0 / w1.length());
        let u2 = w2 * (1.0 / w2.length());
        let m1 = mirror(u1);
        let m2 = mirror(u2);
        let t1 = cell_table()[2].1; // the 66° cell pair's anchor
        let t2 = cell_table()[6].1; // the 246° cell pair's anchor
        // Exact generator identities: g² = (I + M)·t.
        let gs1 = v2((m1.0 + 1.0) * t1.x + m1.1 * t1.y, m1.2 * t1.x + (m1.3 + 1.0) * t1.y);
        assert!((gs1 - w1).length() < 1e-9, "g1² must equal w1");
        let gs2 = v2((m2.0 + 1.0) * t2.x + m2.1 * t2.y, m2.2 * t2.x + (m2.3 + 1.0) * t2.y);
        assert!((gs2 + w2).length() < 1e-9, "g2² must equal −w2");
        // Each glide shifts half a primitive translation along its axis —
        // no conjugate of it is a pure reflection.
        for (tt, uu, ww, nm) in [(t1, u1, w1, "g1"), (t2, u2, w2, "g2")] {
            let half = 2.0 * tt.dot(uu) / ww.length();
            assert!((half - half.round()).abs() - 0.5 < 1e-6, "{nm} must shift half a period");
        }
        // g2∘g1 is a half-turn centred at c0 itself.
        let comp = v2(t2.x + m2.0 * t1.x + m2.1 * t1.y, t2.y + m2.2 * t1.x + m2.3 * t1.y);
        assert!((comp * 0.5 - c0).length() < 1e-9, "centre(g2∘g1) must be c0");
        // The glides preserve the tiling (image window is tilted, so check
        // containment of the images among the tiles generated there) — and
        // so do the inverses and the lattice-shifted variants.
        check_iso(&t, &p, m1, t1, ra, false, "glide g1 (axis ∥ w1)");
        check_iso(&t, &p, m1, v2(-(m1.0 * t1.x + m1.1 * t1.y), -(m1.2 * t1.x + m1.3 * t1.y)), ra, false, "glide g1⁻¹");
        check_iso(&t, &p, m2, t2, ra, false, "glide g2 (axis ∥ w2)");
        check_iso(&t, &p, m2, v2(-(m2.0 * t2.x + m2.1 * t2.y), -(m2.2 * t2.x + m2.3 * t2.y)), ra, false, "glide g2⁻¹");
        // Big patch for tile-image membership.
        let big = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        let set: HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();
        for i in -2..=2 {
            // sanity: pure lattice translations are symmetries
            for &w in [&w1, &w2] {
                let img: Vec<Vec2> = prot.iter().map(|q| *q + w * i as f64).collect();
                assert!(set.contains(&vkey(&img)), "cell0 + {i}·w must be a tile");
            }
        }
        for (i, j) in [(0.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (1.0, 0.0)] {
            let sh = w1 * i + w2 * j;
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m1, t1 + sh, *q)).collect();
            assert!(set.contains(&vkey(&img)), "(g1 + {i}·w1 + {j}·w2) must map cell0 onto a tile");
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m2, t2 + sh, *q)).collect();
            assert!(set.contains(&vkey(&img)), "(g2 + {i}·w1 + {j}·w2) must map cell0 onto a tile");
        }
        // No pure reflection: the mirror-class candidate carrying only the
        // perpendicular glide part (a reflection would need zero along-axis
        // shift), and every whole-lattice-shift candidate, must fail to map
        // the prototile onto a tile.
        let p1 = t1 - u1 * t1.dot(u1);
        let p2 = t2 - u2 * t2.dot(u2);
        for (m, tr, nm) in [(m1, p1, "(M1, t1⊥)"), (m2, p2, "(M2, t2⊥)")] {
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, tr, *q)).collect();
            assert!(!set.contains(&vkey(&img)), "{nm} must not be a symmetry (no mirrors in pgg)");
        }
        for i in -2..=2 {
            for (m, w, mn, wn) in
                [(m1, w1, "M1", "w1"), (m1, w2, "M1", "w2"), (m2, w1, "M2", "w1"), (m2, w2, "M2", "w2")]
            {
                let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, w * i as f64, *q)).collect();
                assert!(
                    !set.contains(&vkey(&img)),
                    "({mn}, {i}·{wn}) must not be a symmetry (no mirrors in pgg)"
                );
            }
        }
    }

    /// 2-isohedral: seeds 0/1 are the two full-group isohedral classes —
    /// the half-turn and both glide classes permute the eight cells
    /// mod-lattice by the closed permutations, the group they generate has
    /// exactly two orbits on the cells ({0,2,4,7} / {1,3,5,6}, matching
    /// the seeds), both seeds appear in every patch, and every generated
    /// tile keeps the prototile side multiset {a, c, b, d, d}.
    #[test]
    fn pentagon_11_isohedral_classes() {
        let t = spec();
        let (r, s, d) = ratios();
        let mut counts = [0usize; 2];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(counts, [4, 4], "four tiles per isohedral class");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        let u_of = |p: Vec2| {
            v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
        };
        let mod1 = |x: f64| (x + 0.5).rem_euclid(1.0) - 0.5;
        let table = cell_table();
        // A symmetry (M, t) permutes the cells: the A-corner (unique 90°
        // angle) maps to the A-corner of a cell whose anchor agrees mod
        // lattice and whose rotation matches (M·t rotates the tile by
        // ±θ).  Orientation-reversing maps swap the chiral families.
        let perm_of = |m: (f64, f64, f64, f64), tr: Vec2| -> Vec<usize> {
            let det_m = m.0 * m.3 - m.1 * m.2;
            let rot = m.2.atan2(m.0).to_degrees();
            let mut perm: Vec<usize> = Vec::new();
            for k in 0..8 {
                let (fam_k, a_k, th_k) = table[k];
                let ai = apply_iso(m, tr, a_k);
                let want_fam = if det_m > 0.0 { fam_k } else { !fam_k };
                let th_img = if det_m > 0.0 { rot + th_k } else { rot - th_k };
                let mut hit = None;
                for (j, &(fam_j, a_j, th_j)) in table.iter().enumerate() {
                    if fam_j != want_fam {
                        continue;
                    }
                    let du = u_of(ai - a_j);
                    if mod1(du.x).abs() < 1e-6
                        && mod1(du.y).abs() < 1e-6
                        && ((th_img - th_j + 180.0).rem_euclid(360.0) - 180.0).abs() < 1e-6
                    {
                        hit = Some(j);
                        break;
                    }
                }
                perm.push(hit.unwrap_or_else(|| panic!("cell {k} has no image")));
            }
            perm
        };
        let c0 = two_fold_centre();
        let u1 = w1 * (1.0 / w1.length());
        let u2 = w2 * (1.0 / w2.length());
        let perms = [
            perm_of((-1.0, 0.0, 0.0, -1.0), c0 * 2.0),
            perm_of(mirror(u1), table[2].1),
            perm_of(mirror(u2), table[6].1),
        ];
        assert_eq!(perms[0], vec![4, 5, 7, 6, 0, 1, 3, 2], "half-turn perm");
        assert_eq!(perms[1], vec![2, 3, 0, 1, 7, 6, 5, 4], "glide g1 perm");
        assert_eq!(perms[2], vec![7, 6, 4, 5, 2, 3, 1, 0], "glide g2 perm");
        // Two full-group orbits on the cells, exactly the seed classes.
        let mut seen = [false; 8];
        let mut orbits: Vec<Vec<usize>> = Vec::new();
        for st in 0..8 {
            if seen[st] {
                continue;
            }
            let mut orb = vec![st];
            seen[st] = true;
            let mut head = 0;
            while head < orb.len() {
                let x = orb[head];
                head += 1;
                for pm in &perms {
                    let y = pm[x];
                    if !seen[y] {
                        seen[y] = true;
                        orb.push(y);
                    }
                }
            }
            orb.sort_unstable();
            orbits.push(orb);
        }
        assert_eq!(orbits, vec![vec![0, 2, 4, 7], vec![1, 3, 5, 6]], "two full-group orbits");
        for orb in &orbits {
            let sd = t.cells[orb[0]].seed;
            assert!(orb.iter().all(|&k| t.cells[k].seed == sd), "orbit mixes isohedral seeds");
        }
        assert_ne!(t.cells[orbits[0][0]].seed, t.cells[orbits[1][0]].seed, "orbits differ");
        // Generated patches keep the side multiset and show both classes.
        let params = ParamSet::default_for(t.params());
        let rp = t.generate(&params, &Query { region: BBox::new(v2(-6.0, -6.0), v2(10.0, 10.0)) });
        assert!(rp.message.is_none(), "{:?}", rp.message);
        let mut seen_s = [false; 2];
        for tile in &rp.tiles {
            let sd = tile.seed as usize;
            assert!(sd < 2, "seeds must stay in {{0, 1}}");
            seen_s[sd] = true;
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length())
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [r, 1.0, s, d, d];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "seed {sd} side {got} ∉ {{a, c, b, d}}");
            }
        }
        assert!(seen_s[0] && seen_s[1], "both isohedral classes must appear");
    }
}

#[cfg(test)]
mod pentagon12_tests {
    use super::pentagon12::{cell_table, prototile, ratios, two_fold_centre};
    use crate::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon12::spec;
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Reflection matrix in the direction u (u must be unit): 2uuᵀ − I.
    fn mirror(u: Vec2) -> (f64, f64, f64, f64) {
        (2.0 * u.x * u.x - 1.0, 2.0 * u.x * u.y, 2.0 * u.x * u.y, 2.0 * u.y * u.y - 1.0)
    }

    /// Vertex-set key on a 1e-6 grid (shared joints agree to ~1e-15 between
    /// two computations of the same tile, since every cell is built from
    /// the same runtime prototile array).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    /// True when the tile's AABB lies inside `w`.  A tile whose AABB is
    /// inside `ra` has its polygon inside `ra`, so its image under any
    /// isometry g intersects g(ra) ⊆ bbox(g(ra)) — and a tile whose polygon
    /// intersects the region always lies inside the generator's padded
    /// index range — hence the image is always generated for rb.  Tiles
    /// whose AABB merely touches the boundary (polygon outside) are
    /// excluded: their images legitimately escape rb.
    fn aabb_inside(poly: &Polygon, w: BBox) -> bool {
        let bb = poly.aabb();
        bb.min.x >= w.min.x
            && bb.min.y >= w.min.y
            && bb.max.x <= w.max.x
            && bb.max.y <= w.max.y
    }

    /// Verify that the isometry g = (M, t) preserves the tiling on `ra`:
    /// images of interior-AABB tiles of ra must be tiles generated for the
    /// image window bbox(g(ra)).  With `exact = true` (the half-turn case,
    /// g(ra) = ra) the filtered image set must equal the interior-AABB
    /// tile set of the window.
    fn check_iso(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            v2(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            v2(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ta = t.generate(params, &Query { region: ra });
        let tb = t.generate(params, &Query { region: rb });
        assert!(ta.message.is_none(), "{label}: {:?}", ta.message);
        assert!(tb.message.is_none(), "{label}: {:?}", tb.message);
        let src: Vec<_> = ta.tiles.iter().filter(|tile| aabb_inside(&tile.poly, ra)).collect();
        assert!(!src.is_empty(), "{label}: no interior tiles");
        let set_b: HashSet<Vec<(i64, i64)>> = tb
            .tiles
            .iter()
            .filter(|tile| !exact || aabb_inside(&tile.poly, rb))
            .map(|tile| vkey(&tile.poly.pts))
            .collect();
        let set_a: HashSet<Vec<(i64, i64)>> = src
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(set_a.len(), src.len(), "{label}: isometry must be injective on the patch");
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    #[test]
    fn pentagon_12_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "pentagon-12", &p, BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)));
    }

    #[test]
    fn pentagon_12_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-12(rot30,scale0.7)",
            BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_12_deterministic() {
        check_det(&spec(), "pentagon-12", BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)));
    }

    /// Unit structure of the primitive cell: 8 congruent copies of the
    /// runtime-computed Kershner prototile (A = 90°, B = 150°, C = 60°,
    /// D = 120°, E = 120°, sides a = 1, b = √3, c = 1, d = 2, e = 1) on
    /// the rectangular lattice w1 ⊥ w2 with det = 12√3 = 8 × tile area
    /// (tile area 3√3/2).  The closure identities hold to ~1e-12, every
    /// cell is CCW, no two cells coincide, and the seeds follow the two
    /// full-group isohedral orbits.
    #[test]
    fn pentagon_12_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-12");
        assert_eq!(t.cells.len(), 8, "the primitive unit holds eight tiles");
        assert_eq!(t.lattice[0], Vec2::new(-2.598076211353316, -4.5));
        assert_eq!(t.lattice[1], Vec2::new(3.4641016151377544, -2.0));
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        assert!(w1.dot(w2).abs() < 1e-12, "lattice must be rectangular (w1 ⊥ w2)");
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        let a0 = t.cells[0].poly.area();
        assert!((det - 8.0 * a0).abs() < 1e-11, "det {det} ≠ 8 × tile area");
        assert!((det - 12.0 * f64::sqrt(3.0)).abs() < 1e-12, "det {det} ≠ 12√3");
        // The shipped B = 150° member is exact in Q(√3): a = 1, b = √3,
        // c = e = 1, d = 2.
        let (a, b, d) = ratios();
        let e = 2.0 * a - 1.0;
        assert!((a - 1.0).abs() < 1e-12, "a = 1, got {a}");
        assert!((b - f64::sqrt(3.0)).abs() < 1e-12, "b = √3, got {b}");
        assert!((d - 2.0).abs() < 1e-12, "d = 2, got {d}");
        assert!((e - 1.0).abs() < 1e-12, "e = 1, got {e}");
        assert!((d - 2.0 * a).abs() < 1e-12, "d = 2a");
        assert!((d - (1.0 + e)).abs() < 1e-12, "d = c + e");
        // The prototile closes: E = (0, a) with edge directions
        // 0°, 30°, 150°, 210° of lengths b, 1, d, e.
        let pr = prototile();
        assert!(pr[4].x.abs() < 1e-12, "closure E.x = 0, got {}", pr[4].x);
        assert!((pr[4].y - a).abs() < 1e-12, "closure E.y = a, got {}", pr[4].y);
        let proto = &t.cells[0].poly;
        assert_eq!(proto.pts[0], v2(0.0, 0.0), "prototile corner A at the origin");
        for (i, want) in [(0usize, 90.0f64), (1, 150.0), (2, 60.0), (3, 120.0), (4, 120.0)] {
            let got = angle_at(proto, i).to_degrees();
            assert!((got - want).abs() < 1e-9, "angle at vertex {i}: {got} ≠ {want}");
        }
        for (i, want) in [(0usize, b), (1, 1.0), (2, d), (3, 1.0), (4, a)] {
            assert!((side_of(proto, i) - want).abs() < 1e-12, "side {i}");
        }
        // Seeds follow the two full-group isohedral orbits (each orbit
        // mixes both chiralities; the glides identify them).
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 1, 1, 0, 0, 1, 1, 0], "isohedral seed pattern");
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        let mut seen: HashSet<Vec<(i64, i64)>> = HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            assert!((cell.poly.area() - a0).abs() < 1e-12, "cell {k} area");
            assert_eq!(cell.poly.len(), 5, "cell {k} vertex count");
            // Side multiset {1, 1, 1, √3, 2}.
            let mut sides: Vec<f64> = (0..5).map(|i| side_of(&cell.poly, i)).collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [1.0, 1.0, 1.0, b, d];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "cell {k} side {got} ∉ {{1, 1, 1, √3, 2}}");
            }
            // Cyclic (side, angle) fingerprint against the prototile,
            // forward or reversed (the cells realise both chiralities).
            let cs = (0..5).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..5).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            let rev = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(rot + 9 - i) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(rot + 5 - i) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd || rev, "cell {k} is not a congruent type-12 prototile");
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
        }
    }

    /// Window census (default parameters) and declared metadata: culling
    /// is the tile AABB intersecting the region (+1e-9), matching the
    /// independent Python emulation of the lattice.rs culling.
    #[test]
    fn pentagon_12_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::Pgg);
        assert_eq!(t.lattice_kind, LatticeKind::Rectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the cell-0 tile (seed 0)");
        let p = ParamSet::default_for(t.params());
        for (region, want) in [
            (BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)), 98),
            (BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)), 90),
            (BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)), 50),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }

    /// Refined edge census over 215 tiles (the tiling is NON-edge-to-edge
    /// at the pentagon level): of the sides whose midpoint lies strictly
    /// inside the window (−7.45, 13.55) × (−7.6, 11.4), 620 are
    /// matched by exactly one full side (b ↔ b, c ↔ c, e ↔ e)
    /// and 153 are d-sides split at their exact midpoint t = 1/2 into two
    /// unit halves, each contained in exactly one unit partner side (an
    /// a-, c- or e-side); no side carries any other T-junction.  The
    /// window is an exact count with a minimum boundary gap of 0.0888
    /// from any counted midpoint (verified against the exact Q(√3)
    /// tiling), so it is robust against the ~1e-15 float difference
    /// between the runtime-computed prototile and the exact surds.
    #[test]
    fn pentagon_12_refined_edge_census() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let region = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 215, "patch size");
        let (_ra, _bb, da) = ratios();
        let verts: Vec<Vec<Vec2>> = r.tiles.iter().map(|tile| tile.poly.pts.clone()).collect();
        let n = verts.len();
        let mut n_split = 0;
        let mut n_full = 0;
        let mut bad: Vec<String> = Vec::new();
        for i in 0..n {
            for e in 0..5 {
                let (pp, qq) = (verts[i][e], verts[i][(e + 1) % 5]);
                let mid = (pp + qq) * 0.5;
                if !(mid.x > -7.45 && mid.x < 13.55 && mid.y > -7.6 && mid.y < 11.4) {
                    continue;
                }
                let dv = qq - pp;
                let l2 = dv.dot(dv);
                let l = l2.sqrt();
                // Split points: other tiles' vertices strictly inside the
                // open segment (collinear within 1e-7), binned at 1e-9 and
                // merged within 1e-8.
                let mut bins: BTreeSet<i64> = BTreeSet::new();
                for (j, vj) in verts.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    for &vv in vj.iter() {
                        let tt = (vv - pp).dot(dv) / l2;
                        if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                            continue;
                        }
                        let cross = (vv.x - pp.x) * dv.y - (vv.y - pp.y) * dv.x;
                        if cross.abs() / l < 1e-7 {
                            bins.insert((tt * 1e9).round() as i64);
                        }
                    }
                }
                let mut srt: Vec<i64> = Vec::new();
                for b in bins {
                    if srt.is_empty() || b - srt[srt.len() - 1] > 10 {
                        srt.push(b);
                    }
                }
                if srt.len() > 1 {
                    bad.push(format!("tile {i} edge {e}: {srt:?}"));
                    continue;
                }
                let mut ends: Vec<f64> = vec![0.0];
                ends.extend(srt.iter().map(|b| *b as f64 / 1e9));
                ends.push(1.0);
                for w in ends.windows(2) {
                    let aq = pp + dv * w[0];
                    let seg = dv * (w[1] - w[0]);
                    let lseg = seg.length();
                    let uh = seg * (1.0 / lseg);
                    let mut cnt = 0;
                    for (j, vj) in verts.iter().enumerate() {
                        if j == i {
                            continue;
                        }
                        for (a1, b1) in (0..5).map(|k| (vj[k], vj[(k + 1) % 5])) {
                            let c1 = (a1.x - aq.x) * uh.y - (a1.y - aq.y) * uh.x;
                            let c2 = (b1.x - aq.x) * uh.y - (b1.y - aq.y) * uh.x;
                            if c1.abs() > 1e-7 || c2.abs() > 1e-7 {
                                continue;
                            }
                            let ta = (a1 - aq).dot(uh);
                            let tb = (b1 - aq).dot(uh);
                            if ta.min(tb) <= 1e-7 * lseg && ta.max(tb) >= lseg * (1.0 - 1e-7) {
                                cnt += 1;
                            }
                        }
                    }
                    if cnt != 1 {
                        bad.push(format!(
                            "tile {i} edge {e} sub [{:.3}, {:.3}] has {cnt} covering edges",
                            w[0], w[1]
                        ));
                    }
                }
                if srt.len() == 1 {
                    n_split += 1;
                    assert!((l - da).abs() < 1e-6, "split side must be a d-side, got {l}");
                    assert!(
                        (srt[0] as f64 / 1e9 - 0.5).abs() < 1e-6,
                        "d-side must split at its midpoint, got {srt:?}"
                    );
                } else {
                    n_full += 1;
                }
            }
        }
        assert!(bad.is_empty(), "refined edge census violations: {bad:?}");
        assert_eq!(n_full, 620, "full (unsplit) edges");
        assert_eq!(n_split, 153, "d-sides split at their midpoint");
    }

    /// Vertex-figure census over the window (−4, −4)–(8, 8) with margin
    /// 2.6: every interior vertex is a complete star — corner rays plus
    /// rays of edges passing through it (the T-junction points) — merging
    /// within 1e-4 rad, and its sectors are exactly one of the four
    /// figures {90°, 90°, 180°}, {60°, 120°, 180°}, {120°, 120°, 120°}
    /// and {60°, 150°, 150°}, each summing to 360° with exactly three
    /// rays; 35 vertices in total.
    #[test]
    fn pentagon_12_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let big = BBox::new(v2(-4.0, -4.0), v2(8.0, 8.0));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        const MARG: f64 = 2.6;
        let mut pts: Vec<Vec2> = Vec::new();
        for tile in &r.tiles {
            for q in &tile.poly.pts {
                if q.x > big.min.x + MARG
                    && q.x < big.max.x - MARG
                    && q.y > big.min.y + MARG
                    && q.y < big.max.y - MARG
                    && !pts.iter().any(|o| (*o - *q).length() < 1e-5)
                {
                    pts.push(*q);
                }
            }
        }
        let figs: [(u32, u32, u32); 4] =
            [(60, 120, 180), (60, 150, 150), (90, 90, 180), (120, 120, 120)];
        let mut counts: BTreeMap<(u32, u32, u32), usize> = BTreeMap::new();
        for pt in &pts {
            let mut rays: BTreeSet<i64> = BTreeSet::new();
            for tile in &r.tiles {
                let nn = tile.poly.len();
                for k in 0..nn {
                    if (tile.poly.pts[k] - *pt).length() >= 1e-5 {
                        continue;
                    }
                    for &q in [&tile.poly.pts[(k + nn - 1) % nn], &tile.poly.pts[(k + 1) % nn]] {
                        let dir = q - *pt;
                        let th = dir.y.atan2(dir.x).rem_euclid(std::f64::consts::TAU);
                        rays.insert((th * 1e5).round() as i64);
                    }
                }
                for e in 0..nn {
                    let (a, b) = (tile.poly.pts[e], tile.poly.pts[(e + 1) % nn]);
                    let ab = b - a;
                    let l2 = ab.dot(ab);
                    let tt = (*pt - a).dot(ab) / l2;
                    if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                        continue;
                    }
                    if (*pt - a).cross(ab).abs() / l2.sqrt() < 1e-7 {
                        let th0 = ab.y.atan2(ab.x);
                        for add in [0.0, std::f64::consts::PI] {
                            let th = (th0 + add).rem_euclid(std::f64::consts::TAU);
                            rays.insert((th * 1e5).round() as i64);
                        }
                    }
                }
            }
            let mut mr: Vec<f64> = Vec::new();
            for k in &rays {
                let rv = *k as f64 / 1e5;
                if mr.is_empty() || rv - mr[mr.len() - 1] > 1e-4 {
                    mr.push(rv);
                }
            }
            if mr.len() > 1 && mr[mr.len() - 1] - mr[0] > std::f64::consts::TAU - 1e-4 {
                mr.pop();
            }
            let mut gaps: Vec<f64> = (0..mr.len())
                .map(|i| (mr[(i + 1) % mr.len()] - mr[i]).rem_euclid(std::f64::consts::TAU))
                .collect();
            gaps.retain(|g| *g > 1e-4);
            gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let fig: Vec<u32> = gaps.iter().map(|g| g.to_degrees().round() as u32).collect();
            assert_eq!(fig.len(), 3, "vertex {pt:?}: unexpected figure {fig:?}");
            assert_eq!(fig.iter().sum::<u32>(), 360, "vertex {pt:?} figure {fig:?}");
            let f3 = (fig[0], fig[1], fig[2]);
            assert!(figs.contains(&f3), "unexpected vertex figure {f3:?} at {pt:?}");
            *counts.entry(f3).or_insert(0) += 1;
        }
        assert_eq!(pts.len(), 35, "total interior vertices");
        let want: BTreeMap<(u32, u32, u32), usize> = [
            ((60, 120, 180), 9),
            ((60, 150, 150), 10),
            ((90, 90, 180), 8),
            ((120, 120, 120), 8),
        ]
        .into_iter()
        .collect();
        assert_eq!(counts, want, "vertex-figure census");
    }

    /// The wallpaper group is pgg: the half-turn about c0 = (√3/4, 5/4)
    /// preserves the tiling exactly (interior-filtered set equality), the
    /// two perpendicular glides g1 = (M1, t1), g2 = (M2, t2) with axes
    /// along w1 and w2 preserve it (with every lattice-shifted variant,
    /// and so do their inverses), g1² = w1, g2² = −w2, and g2∘g1 is the
    /// half-turn about c0 itself.  Each glide shifts exactly half a
    /// primitive translation along its axis (g1 by +w1/2, g2 by −w2/2),
    /// so no conjugate of it is a pure reflection: neither the
    /// perpendicular-part candidate (M, t − (t·u)u) nor any whole lattice
    /// shift (M, i·w) maps the prototile onto a tile.
    #[test]
    fn pentagon_12_pgg_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let c0 = two_fold_centre();
        let ra = BBox::new(v2(c0.x - 12.0, c0.y - 10.0), v2(c0.x + 12.0, c0.y + 10.0));
        check_iso(&t, &p, (-1.0, 0.0, 0.0, -1.0), c0 * 2.0, ra, true, "half-turn about c0");
        let u1 = w1 * (1.0 / w1.length());
        let u2 = w2 * (1.0 / w2.length());
        let m1 = mirror(u1);
        let m2 = mirror(u2);
        let t1 = cell_table()[3].1; // the 120° mirror pair's anchor: (0, −3)
        let t2 = cell_table()[6].1; // the 300° pair's anchor: (−√3, 1)
        // Exact generator identities: g² = (I + M)·t.
        let gs1 = v2((m1.0 + 1.0) * t1.x + m1.1 * t1.y, m1.2 * t1.x + (m1.3 + 1.0) * t1.y);
        assert!((gs1 - w1).length() < 1e-9, "g1² must equal w1");
        let gs2 = v2((m2.0 + 1.0) * t2.x + m2.1 * t2.y, m2.2 * t2.x + (m2.3 + 1.0) * t2.y);
        assert!((gs2 + w2).length() < 1e-9, "g2² must equal −w2");
        // Each glide shifts exactly half a primitive translation along its
        // axis — g1 by +w1/2, g2 by −w2/2 — so no conjugate of it is a
        // pure reflection.
        let h1 = 2.0 * t1.dot(u1) / w1.length();
        assert!((h1 - 1.0).abs() < 1e-6, "g1 must shift exactly half of w1, got {h1}");
        let h2 = 2.0 * t2.dot(u2) / w2.length();
        assert!((h2 + 1.0).abs() < 1e-6, "g2 must shift exactly half of w2, got {h2}");
        // g2∘g1 is a half-turn centred at c0 itself.
        let comp = v2(t2.x + m2.0 * t1.x + m2.1 * t1.y, t2.y + m2.2 * t1.x + m2.3 * t1.y);
        assert!((comp * 0.5 - c0).length() < 1e-9, "centre(g2∘g1) must be c0");
        // The glides preserve the tiling (image window is tilted, so check
        // containment of the images among the tiles generated there) — and
        // so do the inverses and the lattice-shifted variants.
        check_iso(&t, &p, m1, t1, ra, false, "glide g1 (axis ∥ w1)");
        check_iso(
            &t,
            &p,
            m1,
            v2(-(m1.0 * t1.x + m1.1 * t1.y), -(m1.2 * t1.x + m1.3 * t1.y)),
            ra,
            false,
            "glide g1⁻¹",
        );
        check_iso(&t, &p, m2, t2, ra, false, "glide g2 (axis ∥ w2)");
        check_iso(
            &t,
            &p,
            m2,
            v2(-(m2.0 * t2.x + m2.1 * t2.y), -(m2.2 * t2.x + m2.3 * t2.y)),
            ra,
            false,
            "glide g2⁻¹",
        );
        // Big patch for tile-image membership.
        let big = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        let set: HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();
        for i in -2..=2 {
            // sanity: pure lattice translations are symmetries
            for &w in [&w1, &w2] {
                let img: Vec<Vec2> = prot.iter().map(|q| *q + w * i as f64).collect();
                assert!(set.contains(&vkey(&img)), "cell0 + {i}·w must be a tile");
            }
        }
        for (i, j) in [(0.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (1.0, 0.0)] {
            let sh = w1 * i + w2 * j;
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m1, t1 + sh, *q)).collect();
            assert!(set.contains(&vkey(&img)), "(g1 + {i}·w1 + {j}·w2) must map cell0 onto a tile");
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m2, t2 + sh, *q)).collect();
            assert!(set.contains(&vkey(&img)), "(g2 + {i}·w1 + {j}·w2) must map cell0 onto a tile");
        }
        // No pure reflection: the mirror-class candidate carrying only the
        // perpendicular glide part (a reflection would need zero along-axis
        // shift), and every whole-lattice-shift candidate, must fail to map
        // the prototile onto a tile.
        let p1 = t1 - u1 * t1.dot(u1);
        let p2 = t2 - u2 * t2.dot(u2);
        for (m, tr, nm) in [(m1, p1, "(M1, t1⊥)"), (m2, p2, "(M2, t2⊥)")] {
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, tr, *q)).collect();
            assert!(!set.contains(&vkey(&img)), "{nm} must not be a symmetry (no mirrors in pgg)");
        }
        for i in -2..=2 {
            for (m, w, mn, wn) in
                [(m1, w1, "M1", "w1"), (m1, w2, "M1", "w2"), (m2, w1, "M2", "w1"), (m2, w2, "M2", "w2")]
            {
                let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, w * i as f64, *q)).collect();
                assert!(
                    !set.contains(&vkey(&img)),
                    "({mn}, {i}·{wn}) must not be a symmetry (no mirrors in pgg)"
                );
            }
        }
    }

    /// 2-isohedral: seeds 0/1 are the two full-group isohedral classes —
    /// the half-turn and both glide classes permute the eight cells
    /// mod-lattice by the closed permutations, the group they generate has
    /// exactly two orbits on the cells ({0,3,4,7} / {1,2,5,6}, matching
    /// the seeds), both seeds appear in every patch, and every generated
    /// tile keeps the prototile side multiset {1, 1, 1, √3, 2}.
    #[test]
    fn pentagon_12_isohedral_classes() {
        let t = spec();
        let (_a, b, d) = ratios();
        let mut counts = [0usize; 2];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(counts, [4, 4], "four tiles per isohedral class");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        let u_of = |p: Vec2| {
            v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
        };
        let mod1 = |x: f64| (x + 0.5).rem_euclid(1.0) - 0.5;
        let table = cell_table();
        // A symmetry (M, t) permutes the cells: the A-corner (unique 90°
        // angle) maps to the A-corner of a cell whose anchor agrees mod
        // lattice and whose rotation matches (M·t rotates the tile by
        // ±θ).  Orientation-reversing maps swap the chiral families.
        let perm_of = |m: (f64, f64, f64, f64), tr: Vec2| -> Vec<usize> {
            let det_m = m.0 * m.3 - m.1 * m.2;
            let rot = m.2.atan2(m.0).to_degrees();
            let mut perm: Vec<usize> = Vec::new();
            for k in 0..8 {
                let (fam_k, a_k, th_k) = table[k];
                let ai = apply_iso(m, tr, a_k);
                let want_fam = if det_m > 0.0 { fam_k } else { !fam_k };
                let th_img = if det_m > 0.0 { rot + th_k } else { rot - th_k };
                let mut hit = None;
                for (j, &(fam_j, a_j, th_j)) in table.iter().enumerate() {
                    if fam_j != want_fam {
                        continue;
                    }
                    let du = u_of(ai - a_j);
                    if mod1(du.x).abs() < 1e-6
                        && mod1(du.y).abs() < 1e-6
                        && ((th_img - th_j + 180.0).rem_euclid(360.0) - 180.0).abs() < 1e-6
                    {
                        hit = Some(j);
                        break;
                    }
                }
                perm.push(hit.unwrap_or_else(|| panic!("cell {k} has no image")));
            }
            perm
        };
        let c0 = two_fold_centre();
        let u1 = w1 * (1.0 / w1.length());
        let u2 = w2 * (1.0 / w2.length());
        let perms = [
            perm_of((-1.0, 0.0, 0.0, -1.0), c0 * 2.0),
            perm_of(mirror(u1), table[3].1),
            perm_of(mirror(u2), table[6].1),
        ];
        assert_eq!(perms[0], vec![4, 5, 6, 7, 0, 1, 2, 3], "half-turn perm");
        assert_eq!(perms[1], vec![3, 2, 1, 0, 7, 6, 5, 4], "glide g1 perm");
        assert_eq!(perms[2], vec![7, 6, 5, 4, 3, 2, 1, 0], "glide g2 perm");
        // Two full-group orbits on the cells, exactly the seed classes.
        let mut seen = [false; 8];
        let mut orbits: Vec<Vec<usize>> = Vec::new();
        for st in 0..8 {
            if seen[st] {
                continue;
            }
            let mut orb = vec![st];
            seen[st] = true;
            let mut head = 0;
            while head < orb.len() {
                let x = orb[head];
                head += 1;
                for pm in &perms {
                    let y = pm[x];
                    if !seen[y] {
                        seen[y] = true;
                        orb.push(y);
                    }
                }
            }
            orb.sort_unstable();
            orbits.push(orb);
        }
        assert_eq!(orbits, vec![vec![0, 3, 4, 7], vec![1, 2, 5, 6]], "two full-group orbits");
        for orb in &orbits {
            let sd = t.cells[orb[0]].seed;
            assert!(orb.iter().all(|&k| t.cells[k].seed == sd), "orbit mixes isohedral seeds");
        }
        assert_ne!(t.cells[orbits[0][0]].seed, t.cells[orbits[1][0]].seed, "orbits differ");
        // Generated patches keep the side multiset and show both classes.
        let params = ParamSet::default_for(t.params());
        let rp = t.generate(&params, &Query { region: BBox::new(v2(-6.0, -6.0), v2(10.0, 10.0)) });
        assert!(rp.message.is_none(), "{:?}", rp.message);
        let mut seen_s = [false; 2];
        for tile in &rp.tiles {
            let sd = tile.seed as usize;
            assert!(sd < 2, "seeds must stay in {{0, 1}}");
            seen_s[sd] = true;
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length())
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [1.0, 1.0, 1.0, b, d];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "seed {sd} side {got} ∉ {{1, 1, 1, √3, 2}}");
            }
        }
        assert!(seen_s[0] && seen_s[1], "both isohedral classes must appear");
    }
}

#[cfg(test)]
mod pentagon10_tests {
    use super::pentagon10::{cell_table, prototile, ratios};
    use crate::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon10::spec;
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Vertex-set key on a 1e-6 grid (shared joints agree to ~1e-15 between
    /// two computations of the same tile, since every cell is built from
    /// the same runtime prototile array).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    /// True when the tile's AABB lies inside `w`.  A tile whose AABB is
    /// inside `ra` has its polygon inside `ra`, so its image under any
    /// isometry g intersects g(ra) ⊆ bbox(g(ra)) — and a tile whose polygon
    /// intersects the region always lies inside the generator's padded
    /// index range — hence the image is always generated for rb.  Tiles
    /// whose AABB merely touches the boundary (polygon outside) are
    /// excluded: their images legitimately escape rb.
    fn aabb_inside(poly: &Polygon, w: BBox) -> bool {
        let bb = poly.aabb();
        bb.min.x >= w.min.x
            && bb.min.y >= w.min.y
            && bb.max.x <= w.max.x
            && bb.max.y <= w.max.y
    }

    /// Verify that the isometry g = (M, t) preserves the tiling on `ra`:
    /// images of interior-AABB tiles of ra must be tiles generated for the
    /// image window bbox(g(ra)).  With `exact = true` (g(ra) = ra) the
    /// filtered image set must equal the interior-AABB tile set of the
    /// window.
    fn check_iso(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            v2(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            v2(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ta = t.generate(params, &Query { region: ra });
        let tb = t.generate(params, &Query { region: rb });
        assert!(ta.message.is_none(), "{label}: {:?}", ta.message);
        assert!(tb.message.is_none(), "{label}: {:?}", tb.message);
        let src: Vec<_> = ta.tiles.iter().filter(|tile| aabb_inside(&tile.poly, ra)).collect();
        assert!(!src.is_empty(), "{label}: no interior tiles");
        let set_b: HashSet<Vec<(i64, i64)>> = tb
            .tiles
            .iter()
            .filter(|tile| !exact || aabb_inside(&tile.poly, rb))
            .map(|tile| vkey(&tile.poly.pts))
            .collect();
        let set_a: HashSet<Vec<(i64, i64)>> = src
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(set_a.len(), src.len(), "{label}: isometry must be injective on the patch");
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    #[test]
    fn pentagon_10_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "pentagon-10", &p, BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)));
    }

    #[test]
    fn pentagon_10_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-10(rot30,scale0.7)",
            BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_10_deterministic() {
        check_det(&spec(), "pentagon-10", BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)));
    }

    /// Unit structure of the primitive cell: 6 tiles (the prototile at its
    /// four rotations about the origin plus the two mirrored cells at the
    /// shifted anchors (1, 2) and (−1, −2)) on the centered basis
    /// w1 = (3, 3), w2 = (−2, 5) with det = 21 = 6 × tile area (7/2).
    /// The index-2 sublattice Z(3,3) + Z(−7,7) is the primitive rectangle
    /// (sides 3√2 × 7√2) and its centre w2 = (−2, 5) lies in the lattice
    /// — the centered-rectangular structure that separates cmm from pmm.
    /// The shipped B = 90-degree member is exact: a = b = 2, c = e = 1,
    /// d = √2 (d² = c² + e²); every cell is CCW, no two cells coincide,
    /// and the seeds follow the three isohedral orbits.
    #[test]
    fn pentagon_10_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-10");
        assert_eq!(t.cells.len(), 6, "the primitive unit holds six tiles");
        assert_eq!(t.lattice[0], Vec2::new(3.0, 3.0));
        assert_eq!(t.lattice[1], Vec2::new(-2.0, 5.0));
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        // The declared centered basis is oblique (w1·w2 = 9); the
        // primitive rectangle Z(3,3) + Z(−7,7) = Z w1 + Z(−w1 + 2w2) is
        // orthogonal: (3,3)·(−7,7) = 0.
        assert!((w1.dot(w2) - 9.0).abs() < 1e-12, "centered basis w1·w2 = 9, got {}", w1.dot(w2));
        let w2p = -w1 + w2 * 2.0;
        assert_eq!(w2p, Vec2::new(-7.0, 7.0), "−w1 + 2w2 = (−7, 7)");
        assert!(w1.dot(w2p).abs() < 1e-12, "the primitive rectangle is orthogonal");
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        let a0 = t.cells[0].poly.area();
        assert!((a0 - 3.5).abs() < 1e-12, "tile area is 7/2, got {a0}");
        assert!((det - 6.0 * a0).abs() < 1e-11, "det {det} ≠ 6 × tile area");
        assert!((det - 21.0).abs() < 1e-12, "det {det} ≠ 21");
        // The shipped B = 90-degree member is exact: a = b = 2, c = e = 1,
        // d = √2 (d² = c² + e² closes the right angle at corner C).
        let (a, b, d) = ratios();
        let c = 1.0;
        let e = a - c;
        assert!((a - 2.0).abs() < 1e-12, "a = 2, got {a}");
        assert!((b - 2.0).abs() < 1e-12, "b = 2, got {b}");
        assert!((d - f64::sqrt(2.0)).abs() < 1e-12, "d = √2, got {d}");
        assert!((e - 1.0).abs() < 1e-12, "e = 1, got {e}");
        assert!((a - (c + e)).abs() < 1e-12, "a = c + e");
        assert!((b - (c + e)).abs() < 1e-12, "b = c + e");
        assert!((d * d - c * c - e * e).abs() < 1e-12, "d² = c² + e²");
        // The prototile closes: E = (0, a) with edge directions 0°, 90°,
        // 135° of lengths b, c, d (the closing edge E→A has direction
        // 270° and length a).
        let pr = prototile();
        assert!(pr[4].x.abs() < 1e-12, "closure E.x = 0, got {}", pr[4].x);
        assert!((pr[4].y - a).abs() < 1e-12, "closure E.y = a, got {}", pr[4].y);
        let proto = &t.cells[0].poly;
        assert_eq!(proto.pts[0], v2(0.0, 0.0), "prototile corner A at the origin");
        for (i, want) in [(0usize, 90.0f64), (1, 90.0), (2, 135.0), (3, 135.0), (4, 90.0)] {
            let got = angle_at(proto, i).to_degrees();
            assert!((got - want).abs() < 1e-9, "angle at vertex {i}: {got} ≠ {want}");
        }
        for (i, want) in [(0usize, b), (1, c), (2, d), (3, e), (4, a)] {
            assert!((side_of(proto, i) - want).abs() < 1e-12, "side {i}");
        }
        // Seeds follow the three full-group isohedral orbits.
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 1, 0, 1, 2, 2], "isohedral seed pattern");
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        let mut seen: HashSet<Vec<(i64, i64)>> = HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            assert!((cell.poly.area() - a0).abs() < 1e-12, "cell {k} area");
            assert_eq!(cell.poly.len(), 5, "cell {k} vertex count");
            // Side multiset {1, 1, √2, 2, 2}.
            let mut sides: Vec<f64> = (0..5).map(|i| side_of(&cell.poly, i)).collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [1.0, 1.0, f64::sqrt(2.0), 2.0, 2.0];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "cell {k} side {got} ∉ {{1, 1, √2, 2, 2}}");
            }
            // Cyclic (side, angle) fingerprint against the prototile,
            // forward or reversed (the cells realise both chiralities).
            let cs = (0..5).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..5).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            let rev = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(rot + 9 - i) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(rot + 5 - i) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd || rev, "cell {k} is not a congruent type-10 prototile");
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
        }
        // The two M cells are the mirrored chirality at the shifted
        // anchors (1, 2) and (−1, −2); at B = 90° the reflection is the
        // isosceles axis of the prototile, so as vertex sets they coincide
        // with the D cells at the same anchors.
        let table = cell_table();
        assert!(!table[0].0 && !table[1].0 && !table[2].0 && !table[3].0, "cells 0–3 are D cells");
        assert!(table[4].0 && table[5].0, "cells 4 and 5 carry the mirrored chirality");
        assert_eq!(table[4].1, v2(1.0, 2.0), "M90 anchor");
        assert_eq!(table[5].1, v2(-1.0, -2.0), "M270 anchor");
        assert_eq!(table[4].2, 90.0, "M90 rotation");
        assert_eq!(table[5].2, 270.0, "M270 rotation");
    }

    /// Window census (default parameters) and declared metadata: culling
    /// is the tile AABB intersecting the region (+1e-9), matching the
    /// independent Python emulation of the lattice.rs culling.
    #[test]
    fn pentagon_10_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::Cmm);
        assert_eq!(t.lattice_kind, LatticeKind::CenteredRectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the cell-0 tile (seed 0)");
        let p = ParamSet::default_for(t.params());
        for (region, want) in [
            (BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)), 77),
            (BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)), 73),
            (BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)), 41),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }

    /// Refined edge census over 164 tiles (the tiling is NON-edge-to-edge
    /// at the pentagon level): of the sides whose midpoint lies strictly
    /// inside the window (−7.75, 13.75) × (−7.75, 11.75), 518 are matched
    /// by exactly one full side and 76 are length-2 sides split at their
    /// exact midpoint t = 1/2 into two unit halves, each contained in
    /// exactly one unit partner side (a- or b-halves); no side carries any
    /// other T-junction.  Midpoints live in (1/2)Z² and the window bounds
    /// are quarter-integers one unit in, so the minimum boundary gap is
    /// exactly 1/4 — the count is robust against the ~1e-15 float residue
    /// of the runtime-computed prototile.
    #[test]
    fn pentagon_10_refined_edge_census() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let region = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 164, "patch size");
        let (a, _b, _d) = ratios();
        let verts: Vec<Vec<Vec2>> = r.tiles.iter().map(|tile| tile.poly.pts.clone()).collect();
        let n = verts.len();
        let mut n_split = 0;
        let mut n_full = 0;
        let mut bad: Vec<String> = Vec::new();
        for i in 0..n {
            for e in 0..5 {
                let (pp, qq) = (verts[i][e], verts[i][(e + 1) % 5]);
                let mid = (pp + qq) * 0.5;
                if !(mid.x > -7.75 && mid.x < 13.75 && mid.y > -7.75 && mid.y < 11.75) {
                    continue;
                }
                let dv = qq - pp;
                let l2 = dv.dot(dv);
                let l = l2.sqrt();
                // Split points: other tiles' vertices strictly inside the
                // open segment (collinear within 1e-7), binned at 1e-9 and
                // merged within 1e-8.
                let mut bins: BTreeSet<i64> = BTreeSet::new();
                for (j, vj) in verts.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    for &vv in vj.iter() {
                        let tt = (vv - pp).dot(dv) / l2;
                        if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                            continue;
                        }
                        let cross = (vv.x - pp.x) * dv.y - (vv.y - pp.y) * dv.x;
                        if cross.abs() / l < 1e-7 {
                            bins.insert((tt * 1e9).round() as i64);
                        }
                    }
                }
                let mut srt: Vec<i64> = Vec::new();
                for b in bins {
                    if srt.is_empty() || b - srt[srt.len() - 1] > 10 {
                        srt.push(b);
                    }
                }
                if srt.len() > 1 {
                    bad.push(format!("tile {i} edge {e}: {srt:?}"));
                    continue;
                }
                let mut ends: Vec<f64> = vec![0.0];
                ends.extend(srt.iter().map(|b| *b as f64 / 1e9));
                ends.push(1.0);
                for w in ends.windows(2) {
                    let aq = pp + dv * w[0];
                    let seg = dv * (w[1] - w[0]);
                    let lseg = seg.length();
                    let uh = seg * (1.0 / lseg);
                    let mut cnt = 0;
                    for (j, vj) in verts.iter().enumerate() {
                        if j == i {
                            continue;
                        }
                        for (a1, b1) in (0..5).map(|k| (vj[k], vj[(k + 1) % 5])) {
                            let c1 = (a1.x - aq.x) * uh.y - (a1.y - aq.y) * uh.x;
                            let c2 = (b1.x - aq.x) * uh.y - (b1.y - aq.y) * uh.x;
                            if c1.abs() > 1e-7 || c2.abs() > 1e-7 {
                                continue;
                            }
                            let ta = (a1 - aq).dot(uh);
                            let tb = (b1 - aq).dot(uh);
                            if ta.min(tb) <= 1e-7 * lseg && ta.max(tb) >= lseg * (1.0 - 1e-7) {
                                cnt += 1;
                            }
                        }
                    }
                    if cnt != 1 {
                        bad.push(format!(
                            "tile {i} edge {e} sub [{:.3}, {:.3}] has {cnt} covering edges",
                            w[0], w[1]
                        ));
                    }
                }
                if srt.len() == 1 {
                    n_split += 1;
                    assert!((l - a).abs() < 1e-6, "split side must be a length-2 side, got {l}");
                    assert!(
                        (srt[0] as f64 / 1e9 - 0.5).abs() < 1e-6,
                        "length-2 side must split at its midpoint, got {srt:?}"
                    );
                } else {
                    n_full += 1;
                }
            }
        }
        assert!(bad.is_empty(), "refined edge census violations: {bad:?}");
        assert_eq!(n_full, 518, "full (unsplit) edges");
        assert_eq!(n_split, 76, "length-2 sides split at their midpoint");
    }

    /// Vertex-figure census over the window (−8, −8)–(14, 12) with margin
    /// 2.6: every interior vertex is a complete star — corner rays plus
    /// rays of edges passing through it (the T-junction points) — merging
    /// within 1e-4 rad, and its sectors are exactly one of the three
    /// figures {90°, 135°, 135°}, {90°, 90°, 180°} (T-junctions, where a
    /// length-2 side is bisected) and {90°, 90°, 90°, 90°}, each summing
    /// to 360°; 134 vertices in total (71 + 50 + 13).
    #[test]
    fn pentagon_10_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let big = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        const MARG: f64 = 2.6;
        let mut pts: Vec<Vec2> = Vec::new();
        for tile in &r.tiles {
            for q in &tile.poly.pts {
                if q.x > big.min.x + MARG
                    && q.x < big.max.x - MARG
                    && q.y > big.min.y + MARG
                    && q.y < big.max.y - MARG
                    && !pts.iter().any(|o| (*o - *q).length() < 1e-5)
                {
                    pts.push(*q);
                }
            }
        }
        let figs: [Vec<u32>; 3] = [vec![90, 135, 135], vec![90, 90, 180], vec![90, 90, 90, 90]];
        let mut counts: BTreeMap<Vec<u32>, usize> = BTreeMap::new();
        for pt in &pts {
            let mut rays: BTreeSet<i64> = BTreeSet::new();
            for tile in &r.tiles {
                let nn = tile.poly.len();
                for k in 0..nn {
                    if (tile.poly.pts[k] - *pt).length() >= 1e-5 {
                        continue;
                    }
                    for &q in [&tile.poly.pts[(k + nn - 1) % nn], &tile.poly.pts[(k + 1) % nn]] {
                        let dir = q - *pt;
                        let th = dir.y.atan2(dir.x).rem_euclid(std::f64::consts::TAU);
                        rays.insert((th * 1e5).round() as i64);
                    }
                }
                for e in 0..nn {
                    let (a, b) = (tile.poly.pts[e], tile.poly.pts[(e + 1) % nn]);
                    let ab = b - a;
                    let l2 = ab.dot(ab);
                    let tt = (*pt - a).dot(ab) / l2;
                    if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                        continue;
                    }
                    if (*pt - a).cross(ab).abs() / l2.sqrt() < 1e-7 {
                        let th0 = ab.y.atan2(ab.x);
                        for add in [0.0, std::f64::consts::PI] {
                            let th = (th0 + add).rem_euclid(std::f64::consts::TAU);
                            rays.insert((th * 1e5).round() as i64);
                        }
                    }
                }
            }
            let mut mr: Vec<f64> = Vec::new();
            for k in &rays {
                let rv = *k as f64 / 1e5;
                if mr.is_empty() || rv - mr[mr.len() - 1] > 1e-4 {
                    mr.push(rv);
                }
            }
            if mr.len() > 1 && mr[mr.len() - 1] - mr[0] > std::f64::consts::TAU - 1e-4 {
                mr.pop();
            }
            let mut gaps: Vec<f64> = (0..mr.len())
                .map(|i| (mr[(i + 1) % mr.len()] - mr[i]).rem_euclid(std::f64::consts::TAU))
                .collect();
            gaps.retain(|g| *g > 1e-4);
            gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let fig: Vec<u32> = gaps.iter().map(|g| g.to_degrees().round() as u32).collect();
            assert!((3..=4).contains(&fig.len()), "vertex {pt:?}: unexpected figure {fig:?}");
            assert_eq!(fig.iter().sum::<u32>(), 360, "vertex {pt:?} figure {fig:?}");
            assert!(figs.contains(&fig), "unexpected vertex figure {fig:?} at {pt:?}");
            *counts.entry(fig.clone()).or_insert(0) += 1;
        }
        assert_eq!(pts.len(), 134, "total interior vertices");
        let want: BTreeMap<Vec<u32>, usize> =
            [
                (vec![90, 135, 135], 71),
                (vec![90, 90, 180], 50),
                (vec![90, 90, 90, 90], 13),
            ]
            .into_iter()
            .collect();
        assert_eq!(counts, want, "vertex-figure census");
    }

    /// The wallpaper group is cmm: the half-turn about the origin and the
    /// two mirrors in y = x and y = −x preserve the tiling exactly
    /// (interior-AABB set equality on the (−10, −10)–(10, 10) patch, 104
    /// tiles), and the genuine glide g = (R2, u), u = (5, −2) = w1 − w2
    /// in the centered basis, with axis x + y = 3/2 preserves it, and so
    /// does its inverse and the lattice-shifted variants.  g² =
    /// (I + M)·u = (7, −7) = w1 − 2w2 is a lattice translation, and the
    /// axis sits at perpendicular offset (3/4, 3/4) from the origin, so
    /// no conjugate of g is a pure reflection: the perpendicular-part
    /// candidate (R2, (3/2, 3/2)), a reflection in the glide axis
    /// itself, maps the prototile onto an unplaced region (the R2 mirror
    /// lines are at x + y = 3k, while the glide axes fill
    /// x + y = 3k/2).  R90 is not a symmetry: R90(M90@(1,2)) is the
    /// unplaced D180@(−2,1) region while R90(D0@(0,0)) is placed.
    #[test]
    fn pentagon_10_cmm_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        const MI: (f64, f64, f64, f64) = (-1.0, 0.0, 0.0, -1.0);
        const R1: (f64, f64, f64, f64) = (0.0, 1.0, 1.0, 0.0);
        const R2: (f64, f64, f64, f64) = (0.0, -1.0, -1.0, 0.0);
        const R90: (f64, f64, f64, f64) = (0.0, -1.0, 1.0, 0.0);
        let ra = BBox::new(v2(-10.0, -10.0), v2(10.0, 10.0));
        // All three non-identity point-group elements are pure isometries
        // about the origin and preserve the tiling exactly on ra.
        check_iso(&t, &p, MI, Vec2::ZERO, ra, true, "half-turn −I");
        check_iso(&t, &p, R1, Vec2::ZERO, ra, true, "mirror R1 (y = x)");
        check_iso(&t, &p, R2, Vec2::ZERO, ra, true, "mirror R2 (y = −x)");
        // The genuine glide g = (R2, u), u = (5, −2) = w1 − w2: g² =
        // (I + M)·u = (7, −7) = w1 − 2w2 is a lattice translation, and
        // the perpendicular glide part is (3/2, 3/2) = w1/2 — the axis
        // x + y = 3/2 at foot (3/4, 3/4).
        let u = w1 - w2;
        assert_eq!(u, v2(5.0, -2.0), "u = (5, −2) in the centered basis");
        let gs = v2((R2.0 + 1.0) * u.x + R2.1 * u.y, R2.2 * u.x + (R2.3 + 1.0) * u.y);
        assert!((gs - (w1 - w2 * 2.0)).length() < 1e-9, "g² must equal w1 − 2w2, got {gs:?}");
        assert!((gs - v2(7.0, -7.0)).length() < 1e-9, "g² = (7, −7)");
        let t_perp = v2(1.5, 1.5);
        assert!((t_perp - w1 * 0.5).length() < 1e-9, "perpendicular glide part");
        // The glide preserves the tiling (the image window is shifted, so
        // check containment of the images among the tiles generated there)
        // — and so does the inverse, g⁻¹ = (R2, −R2·u) = (R2, w2).
        check_iso(&t, &p, R2, u, ra, false, "glide g (axis x + y = 3/2)");
        check_iso(&t, &p, R2, w2, ra, false, "glide g⁻¹");
        // Big patch for tile-image membership.
        let big = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        let set: HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();
        // Every lattice-shifted variant of the glide maps the prototile
        // onto a tile (the R2 coset is the whole set {(R2, v) : v ∈ L},
        // since R2 ∈ O(L)) — as do the pure mirrors (R2, 0) and (R1, 0)
        // and the mirror-line translations (R2, (6, 6)) [x + y = 6] and
        // (R1, (7, −7)) [x − y = 7].
        for (i, j) in [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0)] {
            let sh = w1 * i + w2 * j;
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(R2, u + sh, *q)).collect();
            assert!(set.contains(&vkey(&img)), "(g + {i}·w1 + {j}·w2) must map cell0 onto a tile");
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(R2, w2 + sh, *q)).collect();
            assert!(
                set.contains(&vkey(&img)),
                "(g⁻¹ + {i}·w1 + {j}·w2) must map cell0 onto a tile"
            );
        }
        for (m, tr, nm) in
            [(R2, Vec2::ZERO, "(R2, 0) mirror x + y = 0"), (R1, Vec2::ZERO, "(R1, 0) mirror y = x")]
        {
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, tr, *q)).collect();
            assert!(set.contains(&vkey(&img)), "{nm} must map cell0 onto a tile");
        }
        for (m, tr, nm) in [
            (R2, v2(6.0, 6.0), "(R2, (6, 6)) mirror x + y = 6"),
            (R1, v2(7.0, -7.0), "(R1, (7, −7)) mirror x − y = 7"),
        ] {
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, tr, *q)).collect();
            assert!(set.contains(&vkey(&img)), "{nm} must map cell0 onto a tile");
        }
        // No conjugate of the glide is a pure reflection: the
        // perpendicular-part candidate (R2, t_perp) is the reflection in
        // the glide axis x + y = 3/2 itself, and the tiling has no mirror
        // line there — the R2 mirror lines are at x + y = 3k.
        let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(R2, t_perp, *q)).collect();
        assert!(
            !set.contains(&vkey(&img)),
            "(R2, t_perp) must not be a symmetry (no mirror on the glide axis)"
        );
        // Nor is an off-lattice R1 shift (R1, (1, 1)): its image region is
        // unplaced, and (1, 1) ∉ L.
        let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(R1, v2(1.0, 1.0), *q)).collect();
        assert!(
            !set.contains(&vkey(&img)),
            "(R1, (1, 1)) must not be a symmetry ((1, 1) ∉ L)"
        );
        // R90 is not a symmetry: R90(M90@(1,2)) is the unplaced D180@(−2,1)
        // region while R90(D0@(0,0)) is placed — the quarter-turn really
        // does not preserve the tiling.
        let m90 = t.cells[4].poly.pts.clone();
        let img: Vec<Vec2> = m90.iter().map(|q| apply_iso(R90, Vec2::ZERO, *q)).collect();
        assert!(!set.contains(&vkey(&img)), "R90(M90@(1,2)) region must be unplaced");
        let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(R90, Vec2::ZERO, *q)).collect();
        assert!(set.contains(&vkey(&img)), "R90(D0@(0,0)) region must be placed");
    }

    /// 3-isohedral: seeds 0/1/2 are the three full-group isohedral classes
    /// — the half-turn, both mirrors and the glide permute the six cells
    /// mod-lattice by the closed permutations, the group they generate has
    /// exactly three orbits on the cells ({0, 2} / {1, 3} / {4, 5},
    /// matching the seeds), all three seeds appear in a 73-tile patch, and
    /// every generated tile keeps the prototile side multiset
    /// {1, 1, √2, 2, 2}.  (At B = 90° the two chiralities coincide as
    /// vertex sets, so the cell-image mechanism is whole-polygon vkey
    /// matching with a lattice shift, not the unique-corner mechanism.)
    #[test]
    fn pentagon_10_isohedral_classes() {
        let t = spec();
        let mut counts = [0usize; 3];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(counts, [2, 2, 2], "two tiles per isohedral class");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        // A symmetry (M, t) permutes the cells: the whole-polygon vertex
        // set of the image must equal that of a cell shifted by a lattice
        // translation.
        let perm_of = |m: (f64, f64, f64, f64), tr: Vec2| -> Vec<usize> {
            let mut perm: Vec<usize> = Vec::new();
            for k in 0..6 {
                let img: Vec<Vec2> =
                    t.cells[k].poly.pts.iter().map(|q| apply_iso(m, tr, *q)).collect();
                let ik = vkey(&img);
                let mut hit = None;
                'outer: for j in 0..6 {
                    for ai in -2..=2 {
                        for bj in -2..=2 {
                            let sh = w1 * ai as f64 + w2 * bj as f64;
                            let cand: Vec<Vec2> =
                                t.cells[j].poly.pts.iter().map(|q| *q + sh).collect();
                            if vkey(&cand) == ik {
                                hit = Some(j);
                                break 'outer;
                            }
                        }
                    }
                }
                perm.push(hit.unwrap_or_else(|| panic!("cell {k} has no image")));
            }
            perm
        };
        let perms = [
            perm_of((-1.0, 0.0, 0.0, -1.0), Vec2::ZERO),
            perm_of((0.0, 1.0, 1.0, 0.0), Vec2::ZERO),
            perm_of((0.0, -1.0, -1.0, 0.0), Vec2::ZERO),
            perm_of((0.0, -1.0, -1.0, 0.0), w1),
        ];
        assert_eq!(perms[0], vec![2, 3, 0, 1, 5, 4], "half-turn perm");
        assert_eq!(perms[1], vec![0, 3, 2, 1, 5, 4], "mirror R1 perm");
        assert_eq!(perms[2], vec![2, 1, 0, 3, 4, 5], "mirror R2 perm");
        assert_eq!(perms[3], vec![2, 1, 0, 3, 4, 5], "glide perm");
        // Three full-group orbits on the cells, exactly the seed classes.
        let mut seen = [false; 6];
        let mut orbits: Vec<Vec<usize>> = Vec::new();
        for st in 0..6 {
            if seen[st] {
                continue;
            }
            let mut orb = vec![st];
            seen[st] = true;
            let mut head = 0;
            while head < orb.len() {
                let x = orb[head];
                head += 1;
                for pm in &perms {
                    let y = pm[x];
                    if !seen[y] {
                        seen[y] = true;
                        orb.push(y);
                    }
                }
            }
            orb.sort_unstable();
            orbits.push(orb);
        }
        assert_eq!(orbits, vec![vec![0, 2], vec![1, 3], vec![4, 5]], "three full-group orbits");
        for orb in &orbits {
            let sd = t.cells[orb[0]].seed;
            assert!(orb.iter().all(|&k| t.cells[k].seed == sd), "orbit mixes isohedral seeds");
        }
        assert_ne!(t.cells[orbits[0][0]].seed, t.cells[orbits[1][0]].seed, "orbits differ");
        assert_ne!(t.cells[orbits[0][0]].seed, t.cells[orbits[2][0]].seed, "orbits differ");
        // Generated patches keep the side multiset and show all three
        // classes.
        let params = ParamSet::default_for(t.params());
        let rp = t.generate(
            &params,
            &Query {
                region: BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
            },
        );
        assert!(rp.message.is_none(), "{:?}", rp.message);
        assert_eq!(rp.tiles.len(), 73, "window census");
        let mut seen_s = [false; 3];
        for tile in &rp.tiles {
            let sd = tile.seed as usize;
            assert!(sd < 3, "seeds must stay in {{0, 1, 2}}");
            seen_s[sd] = true;
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length())
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [1.0, 1.0, f64::sqrt(2.0), 2.0, 2.0];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!(
                    (got - w).abs() < 1e-12,
                    "seed {sd} side {got} ∉ {{1, 1, √2, 2, 2}}"
                );
            }
        }
        assert!(seen_s[0] && seen_s[1] && seen_s[2], "all three isohedral classes must appear");
    }
}

#[cfg(test)]
mod pentagon1_tests {
    use super::pentagon1::{cell_table, prototile, s3, tile_area, two_fold_centre};
    use crate::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon1::spec;
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Reflection matrix in the direction u (u must be unit): 2uuᵀ − I.
    fn mirror(u: Vec2) -> (f64, f64, f64, f64) {
        (2.0 * u.x * u.x - 1.0, 2.0 * u.x * u.y, 2.0 * u.x * u.y, 2.0 * u.y * u.y - 1.0)
    }

    /// Rotation matrix through `deg` degrees (counter-clockwise).
    fn rot(deg: f64) -> (f64, f64, f64, f64) {
        let r = deg * core::f64::consts::PI / 180.0;
        let (co, si) = (r.cos(), r.sin());
        (co, -si, si, co)
    }

    /// Translation part of the isometry with linear part `m` and fixed
    /// point `c` (a rotation centre, or a point on the mirror/glide
    /// axis): t = c − M·c.
    fn fixed_tr(m: (f64, f64, f64, f64), c: Vec2) -> Vec2 {
        let (a, b, cc, d) = m;
        v2(c.x - (a * c.x + b * c.y), c.y - (cc * c.x + d * c.y))
    }

    /// Vertex-set key on a 1e-6 grid (shared joints agree to ~1e-15 between
    /// two computations of the same tile, since every cell is built from
    /// the same runtime prototile array).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    /// True when the tile's AABB lies inside `w`.  A tile whose AABB is
    /// inside `ra` has its polygon inside `ra`, so its image under any
    /// isometry g intersects g(ra) ⊆ bbox(g(ra)) — and a tile whose polygon
    /// intersects the region always lies inside the generator's padded
    /// index range — hence the image is always generated for rb.
    fn aabb_inside(poly: &Polygon, w: BBox) -> bool {
        let bb = poly.aabb();
        bb.min.x >= w.min.x
            && bb.min.y >= w.min.y
            && bb.max.x <= w.max.x
            && bb.max.y <= w.max.y
    }

    /// Verify that the isometry g = (M, t) preserves the tiling on `ra`:
    /// images of interior-AABB tiles of ra must be tiles generated for the
    /// image window bbox(g(ra)).  With `exact = true` (the half-turn case,
    /// g(ra) = ra) the filtered image set must equal the interior-AABB
    /// tile set of the window.
    fn check_iso(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            v2(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            v2(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ta = t.generate(params, &Query { region: ra });
        let tb = t.generate(params, &Query { region: rb });
        assert!(ta.message.is_none(), "{label}: {:?}", ta.message);
        assert!(tb.message.is_none(), "{label}: {:?}", tb.message);
        let src: Vec<_> = ta.tiles.iter().filter(|tile| aabb_inside(&tile.poly, ra)).collect();
        assert!(!src.is_empty(), "{label}: no interior tiles");
        let set_b: HashSet<Vec<(i64, i64)>> = tb
            .tiles
            .iter()
            .filter(|tile| !exact || aabb_inside(&tile.poly, rb))
            .map(|tile| vkey(&tile.poly.pts))
            .collect();
        let set_a: HashSet<Vec<(i64, i64)>> = src
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(set_a.len(), src.len(), "{label}: isometry must be injective on the patch");
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    #[test]
    fn pentagon_1_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "pentagon-1", &p, BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)));
    }

    #[test]
    fn pentagon_1_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-1(rot30,scale0.7)",
            BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_1_deterministic() {
        check_det(&spec(), "pentagon-1", BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)));
    }

    /// Unit structure of the primitive cell: 2 congruent copies of the
    /// exact-Q(√3) type-1 prototile (A = B = C = 120°, D = E = 90°,
    /// a = b = 2, c = e = 1, d = 2√3, area 3√3) on the centered-
    /// rectangular lattice in its 60° rhombus basis w1 = (3, −√3),
    /// w2 = (3, √3), det = 6√3 = 2 × tile area (|w1|² = |w2|² = 12,
    /// w1·w2 = 6: the special a² = 3b² case in which a centered-
    /// rectangular cell is hexagonal-metric).  The closure identity
    /// holds to ~1e-15, both cells are CCW and directly congruent
    /// (cell 1 is the half-turn image of cell 0 about the hexagon
    /// centre (−2, 0) — same chirality, no mirror in the cell table),
    /// and the seeds mark the single isohedral orbit.
    #[test]
    fn pentagon_1_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-1");
        assert_eq!(t.cells.len(), 2, "the primitive unit holds two tiles");
        assert_eq!(t.lattice[0], v2(3.0, -s3()));
        assert_eq!(t.lattice[1], v2(3.0, s3()));
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        let a0 = t.cells[0].poly.area();
        assert!((det - 2.0 * a0).abs() < 1e-11, "det {det} ≠ 2 × tile area");
        assert!((det - 6.0 * s3()).abs() < 1e-11, "det {det} ≠ 6√3");
        assert!((a0 - tile_area()).abs() < 1e-11, "tile area ≠ 3√3");
        // 60° rhombus basis of the centered-rectangular lattice:
        // |w1|² = |w2|² = 12, w1·w2 = 6.
        assert!((w1.dot(w1) - 12.0).abs() < 1e-12, "|w1|² = 12");
        assert!((w2.dot(w2) - 12.0).abs() < 1e-12, "|w2|² = 12");
        assert!((w1.dot(w2) - 6.0).abs() < 1e-12, "w1·w2 = 6");
        // The prototile closes: edge directions 120°, 180°, 240°, 330° of
        // lengths 2, 2, 1, 2√3; closing edge E→A has direction 60°,
        // length 1.
        let pr = prototile();
        assert!((pr[4].x + 0.5).abs() < 1e-12, "closure E.x = −1/2, got {}", pr[4].x);
        assert!(
            (pr[4].y + s3() / 2.0).abs() < 1e-12,
            "closure E.y = −√3/2, got {}",
            pr[4].y
        );
        let proto = &t.cells[0].poly;
        assert_eq!(proto.pts[0], v2(0.0, 0.0), "prototile corner A at the origin");
        for (i, want) in [(0usize, 120.0f64), (1, 120.0), (2, 120.0), (3, 90.0), (4, 90.0)] {
            let got = angle_at(proto, i).to_degrees();
            assert!((got - want).abs() < 1e-9, "angle at vertex {i}: {got} ≠ {want}");
        }
        for (i, want) in [(0usize, 2.0f64), (1, 2.0), (2, 1.0), (3, 2.0 * s3()), (4, 1.0)] {
            assert!((side_of(proto, i) - want).abs() < 1e-12, "side {i}");
        }
        // Type-1 family identities in the member's own lettering: under
        // the relabelling (A′, B′, C′, D′, E′) = (C, D, E, A, B) the
        // family condition is B′ + C′ = D + E = 180°, equivalently
        // A + B + C = 360°.  Side identities: a = b, c = e.
        let d_angle = angle_at(proto, 3).to_degrees();
        let e_angle = angle_at(proto, 4).to_degrees();
        assert!((d_angle + e_angle - 180.0).abs() < 1e-9, "D + E = 180°");
        let abc: f64 = (0..3).map(|i| angle_at(proto, i).to_degrees()).sum();
        assert!((abc - 360.0).abs() < 1e-9, "A + B + C = 360°");
        assert!((side_of(proto, 0) - side_of(proto, 1)).abs() < 1e-12, "a = b");
        assert!((side_of(proto, 2) - side_of(proto, 4)).abs() < 1e-12, "c = e");
        // Single isohedral orbit: both seeds 0.
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 0], "1-isohedral seeds");
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        let mut seen: HashSet<Vec<(i64, i64)>> = HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            assert!((cell.poly.area() - a0).abs() < 1e-12, "cell {k} area");
            assert_eq!(cell.poly.len(), 5, "cell {k} vertex count");
            // Cyclic (side, angle) fingerprint against the prototile:
            // forward only — both cells carry the same chirality.
            let cs = (0..5).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..5).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd, "cell {k} is not a directly-congruent type-1 prototile");
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
        }
    }

    /// Window census (default parameters) and declared metadata: culling
    /// is the tile AABB intersecting the region (+1e-9), matching the
    /// independent Python emulation of the lattice.rs culling.
    #[test]
    fn pentagon_1_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::Cmm);
        assert_eq!(t.lattice_kind, LatticeKind::CenteredRectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the cell-0 tile (seed 0)");
        let p = ParamSet::default_for(t.params());
        for (region, want) in [
            (BBox::new(v2(-6.0, -4.0), v2(8.0, 6.0)), 42),
            (BBox::new(v2(-8.0, -6.0), v2(10.0, 8.0)), 66),
            (BBox::new(v2(-10.0, -8.0), v2(14.0, 10.0)), 114),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }

    /// Refined edge census over 94 tiles: the tiling is EDGE-TO-EDGE at
    /// the pentagon level.  Of the sides whose midpoint lies strictly
    /// inside the window (−9.4, 13.4) × (−7.4, 9.4), every one of the
    /// 374 is matched by exactly one full side of one neighbour and no
    /// side carries a T-junction (n_split = 0): whole hexagon sides pair
    /// full to full, each half side (length 1) pairs one-to-one with a
    /// neighbouring half side, and each cut d-side (2√3) pairs with the
    /// companion tile's d-side in the same hexagon.  The window is an
    /// exact count with a minimum boundary margin of 0.0388 from any
    /// counted midpoint (verified against the exact Q(√3) tiling), so it
    /// is robust against the ~1e-15 float difference between the
    /// runtime-computed prototile and the exact surds.
    #[test]
    fn pentagon_1_refined_edge_census() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let region = BBox::new(v2(-9.4, -7.4), v2(13.4, 9.4));
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 94, "patch size");
        let verts: Vec<Vec<Vec2>> = r.tiles.iter().map(|tile| tile.poly.pts.clone()).collect();
        let n = verts.len();
        let mut n_split = 0;
        let mut n_full = 0;
        let mut bad: Vec<String> = Vec::new();
        for i in 0..n {
            for e in 0..5 {
                let (pp, qq) = (verts[i][e], verts[i][(e + 1) % 5]);
                let mid = (pp + qq) * 0.5;
                if !(mid.x > -9.4 && mid.x < 13.4 && mid.y > -7.4 && mid.y < 9.4) {
                    continue;
                }
                let dv = qq - pp;
                let l2 = dv.dot(dv);
                let l = l2.sqrt();
                // Split points: other tiles' vertices strictly inside the
                // open segment (collinear within 1e-7), binned at 1e-9 and
                // merged within 1e-8.
                let mut bins: BTreeSet<i64> = BTreeSet::new();
                for (j, vj) in verts.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    for &vv in vj.iter() {
                        let tt = (vv - pp).dot(dv) / l2;
                        if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                            continue;
                        }
                        let cross = (vv.x - pp.x) * dv.y - (vv.y - pp.y) * dv.x;
                        if cross.abs() / l < 1e-7 {
                            bins.insert((tt * 1e9).round() as i64);
                        }
                    }
                }
                let mut srt: Vec<i64> = Vec::new();
                for b in bins {
                    if srt.is_empty() || b - srt[srt.len() - 1] > 10 {
                        srt.push(b);
                    }
                }
                if srt.len() > 1 {
                    bad.push(format!("tile {i} edge {e}: {srt:?}"));
                    continue;
                }
                let mut ends: Vec<f64> = vec![0.0];
                ends.extend(srt.iter().map(|b| *b as f64 / 1e9));
                ends.push(1.0);
                for w in ends.windows(2) {
                    let aq = pp + dv * w[0];
                    let seg = dv * (w[1] - w[0]);
                    let lseg = seg.length();
                    let uh = seg * (1.0 / lseg);
                    let mut cnt = 0;
                    for (j, vj) in verts.iter().enumerate() {
                        if j == i {
                            continue;
                        }
                        for (a1, b1) in (0..5).map(|k| (vj[k], vj[(k + 1) % 5])) {
                            let c1 = (a1.x - aq.x) * uh.y - (a1.y - aq.y) * uh.x;
                            let c2 = (b1.x - aq.x) * uh.y - (b1.y - aq.y) * uh.x;
                            if c1.abs() > 1e-7 || c2.abs() > 1e-7 {
                                continue;
                            }
                            let ta = (a1 - aq).dot(uh);
                            let tb = (b1 - aq).dot(uh);
                            if ta.min(tb) <= 1e-7 * lseg && ta.max(tb) >= lseg * (1.0 - 1e-7) {
                                cnt += 1;
                            }
                        }
                    }
                    if cnt != 1 {
                        bad.push(format!(
                            "tile {i} edge {e} sub [{:.3}, {:.3}] has {cnt} covering edges",
                            w[0], w[1]
                        ));
                    }
                }
                if srt.len() == 1 {
                    n_split += 1;
                } else {
                    n_full += 1;
                }
            }
        }
        assert!(bad.is_empty(), "refined edge census violations: {bad:?}");
        assert_eq!(n_full, 374, "full (unsplit) edges");
        assert_eq!(n_split, 0, "no T-junctions: the tiling is edge-to-edge");
    }

    /// Vertex-figure census over the window (−9.4, −7.4)–(13.4, 9.4):
    /// every interior vertex (margin 1e-3 from the window edge; the
    /// tightest counted vertex in this window sits 0.4 away, so the scan
    /// is boundary-safe) is a complete star — corner rays plus rays of
    /// edges passing through it (the T-junction points; none occur, the
    /// tiling is edge-to-edge) — merging within 1e-4 rad, and its
    /// sectors are exactly the two figures {120°, 120°, 120°} (the
    /// hexagon-corner vertices, 75 of them) and {90°, 90°, 90°, 90°}
    /// (the cut-midpoint vertices, 31 of them); 106 vertices in total.
    #[test]
    fn pentagon_1_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let big = BBox::new(v2(-9.4, -7.4), v2(13.4, 9.4));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        const MARG: f64 = 1e-3;
        let mut pts: Vec<Vec2> = Vec::new();
        for tile in &r.tiles {
            for q in &tile.poly.pts {
                if q.x > big.min.x + MARG
                    && q.x < big.max.x - MARG
                    && q.y > big.min.y + MARG
                    && q.y < big.max.y - MARG
                    && !pts.iter().any(|o| (*o - *q).length() < 1e-5)
                {
                    pts.push(*q);
                }
            }
        }
        // 3- and 4-valent figures coexist, so key the census by the
        // sorted angle vector.
        let figs: [Vec<u32>; 2] = [vec![120, 120, 120], vec![90, 90, 90, 90]];
        let mut counts: BTreeMap<Vec<u32>, usize> = BTreeMap::new();
        for pt in &pts {
            let mut rays: BTreeSet<i64> = BTreeSet::new();
            for tile in &r.tiles {
                let nn = tile.poly.len();
                for k in 0..nn {
                    if (tile.poly.pts[k] - *pt).length() >= 1e-5 {
                        continue;
                    }
                    for &q in [&tile.poly.pts[(k + nn - 1) % nn], &tile.poly.pts[(k + 1) % nn]] {
                        let dir = q - *pt;
                        let th = dir.y.atan2(dir.x).rem_euclid(std::f64::consts::TAU);
                        rays.insert((th * 1e5).round() as i64);
                    }
                }
                for e in 0..nn {
                    let (a, b) = (tile.poly.pts[e], tile.poly.pts[(e + 1) % nn]);
                    let ab = b - a;
                    let l2 = ab.dot(ab);
                    let tt = (*pt - a).dot(ab) / l2;
                    if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                        continue;
                    }
                    if (*pt - a).cross(ab).abs() / l2.sqrt() < 1e-7 {
                        let th0 = ab.y.atan2(ab.x);
                        for add in [0.0, std::f64::consts::PI] {
                            let th = (th0 + add).rem_euclid(std::f64::consts::TAU);
                            rays.insert((th * 1e5).round() as i64);
                        }
                    }
                }
            }
            let mut mr: Vec<f64> = Vec::new();
            for k in &rays {
                let rv = *k as f64 / 1e5;
                if mr.is_empty() || rv - mr[mr.len() - 1] > 1e-4 {
                    mr.push(rv);
                }
            }
            if mr.len() > 1 && mr[mr.len() - 1] - mr[0] > std::f64::consts::TAU - 1e-4 {
                mr.pop();
            }
            let mut gaps: Vec<f64> = (0..mr.len())
                .map(|i| (mr[(i + 1) % mr.len()] - mr[i]).rem_euclid(std::f64::consts::TAU))
                .collect();
            gaps.retain(|g| *g > 1e-4);
            gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let fig: Vec<u32> = gaps.iter().map(|g| g.to_degrees().round() as u32).collect();
            assert!(figs.contains(&fig), "unexpected vertex figure {fig:?} at {pt:?}");
            assert_eq!(fig.iter().sum::<u32>(), 360, "vertex {pt:?} figure {fig:?}");
            *counts.entry(fig).or_insert(0) += 1;
        }
        assert_eq!(pts.len(), 106, "total interior vertices");
        let want: BTreeMap<Vec<u32>, usize> =
            [(vec![120u32, 120, 120], 75), (vec![90u32, 90, 90, 90], 31)]
                .into_iter()
                .collect();
        assert_eq!(counts, want, "vertex-figure census");
    }

    /// The uncoloured wallpaper group is cmm (orbifold \*2222), pinned
    /// here by a coset-style audit over the 257-tile patch
    /// (−16, −14)–(20, 16):
    ///
    /// POSITIVE — the half-turn about c0 = (−2, 0), the hexagon centre
    /// that swaps the two cells, preserves the tiling exactly (set
    /// equality on the symmetric window); the lattice translations i·w1,
    /// j·w2 carry the prototile onto tiles for i, j ∈ [−2, 2]; the
    /// half-turn census over the quarter lattice c0 + (i·w1 + j·w2)/4
    /// finds 2-fold centres exactly at the four coset representatives
    /// c0 + Λ/2 — the hexagon centre (on both mirror families), the cut
    /// midpoint (−1/2, −√3/2) (on both), and the two off-mirror classes
    /// (−1/2, √3/2) and (1, 0) — while the bare quarter lattice
    /// (i·w1 + j·w2)/4 has none; the two perpendicular mirror families
    /// (60° lines at s = p·ê150 ≡ 0 mod √3, and 150° lines at
    /// t = p·ê60 ≡ −1 mod 3) and the six glide axes between them (60°
    /// at s ≡ √3/2 mod √3 with vector 3ê60 whose square is the lattice
    /// translation −w1 + 2w2; 150° at t ≡ 1/2 mod 3 with vector
    /// √3·ê150 whose square is −w1) all preserve the tiling; and the
    /// prototile is achiral — a −120° rotation about c0 maps it exactly
    /// onto its y-mirror — so orientation-reversing symmetries exist.
    ///
    /// NEGATIVE — the half-lattice shifts w1/2, w2/2, (w1+w2)/2,
    /// (w1−w2)/2 are not translations; 32 off-family mirror lines
    /// (60°/150° lines with non-mirror offsets, horizontal, vertical,
    /// and 30° lines) are not mirrors; 4 extra glide candidates are not
    /// glides; and no 60/120/240/300° rotation about any of the six
    /// hexagon-corner classes (with 3×3 lattice shifts) is a symmetry
    /// (216 probes).  With mirrors + glides, 2-folds on and off the
    /// mirrors, a centered-rectangular translation lattice (basis
    /// (6, 0), (0, 2√3), centering (3, √3) — not primitive-rectangular)
    /// and no 3-/6-folds, the group is cmm rather than pmm (which needs
    /// all 2-folds on mirrors of a primitive rectangular lattice) and
    /// rather than p2 (which has no orientation-reversing isometries).
    #[test]
    fn pentagon_1_wallpaper_group_is_cmm() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let c0 = two_fold_centre();
        let e60 = v2(0.5, 0.5 * s3());
        let e150 = v2(-0.5 * s3(), 0.5);

        // Big patch for tile-image membership: every probed image below
        // (prototile images under i, j ∈ [−2, 2] lattice shifts, and the
        // half-turn/mirror/rotation probes centred within one cell of the
        // origin) lies inside this footprint, so membership in the
        // generated set is the exact question.
        let big = BBox::new(v2(-16.0, -14.0), v2(20.0, 16.0));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        assert_eq!(big_tiles.tiles.len(), 257, "big patch tile count");
        let set: HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();

        // Positive: the exact half-turn about c0 (g(ra) = ra).
        let ra = BBox::new(v2(-14.0, -10.0), v2(10.0, 10.0));
        check_iso(
            &t,
            &p,
            (-1.0, 0.0, 0.0, -1.0),
            c0 * 2.0,
            ra,
            true,
            "half-turn about c0",
        );

        // Positive: lattice translations carry the prototile onto tiles.
        for i in -2..=2 {
            for &w in [&w1, &w2] {
                let img: Vec<Vec2> = prot.iter().map(|q| *q + w * i as f64).collect();
                assert!(set.contains(&vkey(&img)), "cell0 + {i}·{w:?} must be a tile");
            }
        }
        // Negative: half-lattice shifts are not translations.
        for h in [w1 * 0.5, w2 * 0.5, (w1 + w2) * 0.5, (w1 - w2) * 0.5] {
            let img: Vec<Vec2> = prot.iter().map(|q| *q + h).collect();
            assert!(
                !set.contains(&vkey(&img)),
                "half-lattice shift {h:?} is a symmetry"
            );
        }

        // Half-turn census over c0 + (i·w1 + j·w2)/4: the only 2-fold
        // centres are the four coset representatives c0 + Λ/2.
        let mut found: Vec<(i8, i8)> = Vec::new();
        for i in 0i8..4 {
            for j in 0i8..4 {
                let c = c0 + (w1 * i as f64 + w2 * j as f64) * 0.25;
                let img: Vec<Vec2> =
                    prot.iter().map(|q| apply_iso((-1.0, 0.0, 0.0, -1.0), c * 2.0, *q)).collect();
                if set.contains(&vkey(&img)) {
                    found.push((i, j));
                }
            }
        }
        assert_eq!(
            found,
            vec![(0, 0), (0, 2), (2, 0), (2, 2)],
            "2-fold centres must be exactly c0 + Λ/2"
        );
        // The bare quarter-lattice points (i·w1 + j·w2)/4 are not centres.
        for i in 0i8..4 {
            for j in 0i8..4 {
                let c = (w1 * i as f64 + w2 * j as f64) * 0.25;
                let img: Vec<Vec2> =
                    prot.iter().map(|q| apply_iso((-1.0, 0.0, 0.0, -1.0), c * 2.0, *q)).collect();
                assert!(
                    !set.contains(&vkey(&img)),
                    "bare quarter point ({i}, {j}) must not be a 2-fold centre"
                );
            }
        }
        // The four classes c0 + Λ/2 survive every whole-lattice shift.
        for h in [Vec2::ZERO, w1 * 0.5, w2 * 0.5, (w1 + w2) * 0.5] {
            for i in -1..=1i8 {
                for j in -1..=1i8 {
                    let c = c0 + h + w1 * i as f64 + w2 * j as f64;
                    let img: Vec<Vec2> = prot
                        .iter()
                        .map(|q| apply_iso((-1.0, 0.0, 0.0, -1.0), c * 2.0, *q))
                        .collect();
                    assert!(
                        set.contains(&vkey(&img)),
                        "half-turn about c0 + {h:?} + {i}·w1 + {j}·w2 must map cell0 onto a tile"
                    );
                }
            }
        }

        // Positive mirrors: 60° lines at s ≡ 0 (mod √3), direction ê60,
        // through the point s·ê150; 150° lines at t ≡ −1 (mod 3),
        // direction ê150, through t·ê60.
        let m60 = mirror(e60);
        let m150 = mirror(e150);
        for k in -1..=1i32 {
            let c = e150 * (k as f64 * s3());
            check_iso(&t, &p, m60, fixed_tr(m60, c), ra, false, "60° mirror");
        }
        for j in -1..=1i32 {
            let c = e60 * (3.0 * j as f64 - 1.0);
            check_iso(&t, &p, m150, fixed_tr(m150, c), ra, false, "150° mirror");
        }

        // Positive glides between the mirror families: the axis
        // reflection followed by the glide translation (each square lies
        // in Λ, so each axis is a genuine glide, not a mirror).
        let g60 = e60 * 3.0; // square: −w1 + 2w2
        let g150 = e150 * s3(); // square: −w1
        for k in -1..=1i32 {
            let c = e150 * ((k as f64 + 0.5) * s3());
            check_iso(&t, &p, m60, fixed_tr(m60, c) + g60, ra, false, "60° glide");
        }
        for j in -1..=1i32 {
            let c = e60 * (3.0 * j as f64 + 0.5);
            check_iso(&t, &p, m150, fixed_tr(m150, c) + g150, ra, false, "150° glide");
        }

        // Negative mirrors: 32 off-family lines — the reflected
        // prototile must not be a tile of the big patch.
        let mut n_neg = 0;
        for k in -1..=1i32 {
            for f in [0.25f64, 0.5, 0.75] {
                let c = e150 * (k as f64 + f) * s3();
                let img: Vec<Vec2> =
                    prot.iter().map(|q| apply_iso(m60, fixed_tr(m60, c), *q)).collect();
                assert!(
                    !set.contains(&vkey(&img)),
                    "60° mirror at s = √3({k} + {f}) is a symmetry"
                );
                n_neg += 1;
            }
        }
        for j in -1..=1i32 {
            for dd in [0.5f64, 1.0, 1.5] {
                let c = e60 * (3.0 * j as f64 - 1.0 + dd);
                let img: Vec<Vec2> =
                    prot.iter().map(|q| apply_iso(m150, fixed_tr(m150, c), *q)).collect();
                assert!(
                    !set.contains(&vkey(&img)),
                    "150° mirror at t = 3·{j} − 1 + {dd} is a symmetry"
                );
                n_neg += 1;
            }
        }
        let mx = mirror(v2(1.0, 0.0));
        let my = mirror(v2(0.0, 1.0));
        for y in [0.0f64, s3() / 4.0, s3() / 2.0, 3.0 * s3() / 4.0] {
            let c = v2(0.0, y);
            let img: Vec<Vec2> =
                prot.iter().map(|q| apply_iso(mx, fixed_tr(mx, c), *q)).collect();
            assert!(!set.contains(&vkey(&img)), "horizontal mirror y = {y} is a symmetry");
            n_neg += 1;
        }
        for x in [0.0f64, 0.75, 1.5, 2.25] {
            let c = v2(x, 0.0);
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(my, fixed_tr(my, c), *q)).collect();
            assert!(!set.contains(&vkey(&img)), "vertical mirror x = {x} is a symmetry");
            n_neg += 1;
        }
        let m30 = mirror(v2(0.5 * s3(), 0.5));
        for k in -1..=1i32 {
            let c = v2(-0.5, 0.5 * s3()) * (3.0 * k as f64 / 4.0);
            let img: Vec<Vec2> =
                prot.iter().map(|q| apply_iso(m30, fixed_tr(m30, c), *q)).collect();
            assert!(!set.contains(&vkey(&img)), "30° mirror (offset {k}) is a symmetry");
            n_neg += 1;
        }
        for k in -1..=1i32 {
            let c = v2(0.5 * s3(), 0.5) * (s3() * k as f64 / 4.0);
            let img: Vec<Vec2> =
                prot.iter().map(|q| apply_iso(m150, fixed_tr(m150, c), *q)).collect();
            assert!(
                !set.contains(&vkey(&img)),
                "150° mirror (offset {k}) is a symmetry"
            );
            n_neg += 1;
        }
        assert_eq!(n_neg, 32, "negative mirror line count");

        // Negative glides: glides on the mirror lines (which would
        // compose with the mirror into non-lattice translations), and
        // two axis directions with glide squares outside Λ.
        let g_neg = [
            (m60, v2(0.0, 0.0), g60),
            (m150, e60 * -1.0, g150),
            (mx, v2(0.0, 0.0), v2(1.5, 0.0)),
            (my, v2(0.0, 0.0), v2(0.0, s3() / 2.0)),
        ];
        for (m, c, g) in g_neg {
            let img: Vec<Vec2> =
                prot.iter().map(|q| apply_iso(m, fixed_tr(m, c) + g, *q)).collect();
            assert!(!set.contains(&vkey(&img)), "glide about {c:?} + {g:?} is a symmetry");
        }

        // No 60/120/240/300° rotation about any hexagon-corner class:
        // 6 corner classes × 3×3 lattice shifts × 4 angles = 216 probes.
        let corners = [
            v2(0.0, 0.0),
            v2(-1.0, s3()),
            v2(-3.0, s3()),
            v2(-4.0, 0.0),
            v2(-3.0, -s3()),
            v2(-1.0, -s3()),
        ];
        let mut n_rot = 0;
        for cv in corners {
            for i in -1..=1i8 {
                for j in -1..=1i8 {
                    let c = cv + w1 * i as f64 + w2 * j as f64;
                    for deg in [60.0f64, 120.0, 240.0, 300.0] {
                        let m = rot(deg);
                        let img: Vec<Vec2> =
                            prot.iter().map(|q| apply_iso(m, fixed_tr(m, c), *q)).collect();
                        assert!(
                            !set.contains(&vkey(&img)),
                            "{deg}° rotation about {c:?} is a symmetry"
                        );
                        n_rot += 1;
                    }
                }
            }
        }
        assert_eq!(n_rot, 216, "rotation negative probe count");

        // The prototile is achiral: R(−120°) about the hexagon centre c0
        // maps it exactly onto its y-mirror (A↦C′, B↦B′, C↦A′, D↦E′,
        // E↦D′), so orientation-reversing symmetries exist and the group
        // cannot be p2.
        let m_120 = rot(-120.0);
        let img: Vec<Vec2> =
            prot.iter().map(|q| apply_iso(m_120, fixed_tr(m_120, c0), *q)).collect();
        let pmir: Vec<Vec2> = prot.iter().map(|q| v2(q.x, -q.y)).collect();
        assert_eq!(
            vkey(&img),
            vkey(&pmir),
            "achirality: R(−120° about c0) must map the prototile onto its y-mirror"
        );
    }

    /// 1-isohedral: both cells carry seed 0 and the half-turn about c0
    /// = (−2, 0) permutes the cell table mod lattice as the transposition
    /// (0 1), so the two cells form a single full-group orbit; every
    /// generated tile keeps the prototile side multiset
    /// {1, 1, 2, 2, 2√3} and seed 0.
    #[test]
    fn pentagon_1_isohedral_single_orbit() {
        let t = spec();
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 0], "single isohedral class");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        let u_of = |p: Vec2| {
            v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
        };
        let mod1 = |x: f64| (x + 0.5).rem_euclid(1.0) - 0.5;
        let table = cell_table();
        let c0 = two_fold_centre();
        // The half-turn maps each cell's anchor to the other cell's
        // anchor mod lattice, rotating the tile by 180° (the table has
        // no mirror, so both cells keep the same chirality).
        let mut perm: Vec<usize> = Vec::new();
        for k in 0..2 {
            let (_fam_k, a_k, th_k) = table[k];
            let ai = apply_iso((-1.0, 0.0, 0.0, -1.0), c0 * 2.0, a_k);
            let th_img = 180.0 + th_k;
            let mut hit = None;
            for (j, &(_fam_j, a_j, th_j)) in table.iter().enumerate() {
                let du = u_of(ai - a_j);
                if mod1(du.x).abs() < 1e-6
                    && mod1(du.y).abs() < 1e-6
                    && ((th_img - th_j + 180.0).rem_euclid(360.0) - 180.0).abs() < 1e-6
                {
                    hit = Some(j);
                    break;
                }
            }
            perm.push(hit.unwrap_or_else(|| panic!("cell {k} has no half-turn image")));
        }
        assert_eq!(perm, vec![1, 0], "half-turn permutes the cells as (0 1)");
        // Generated patches: single seed, prototile side multiset kept.
        let params = ParamSet::default_for(t.params());
        let rp = t.generate(&params, &Query { region: BBox::new(v2(-6.0, -6.0), v2(10.0, 10.0)) });
        assert!(rp.message.is_none(), "{:?}", rp.message);
        assert_eq!(rp.tiles.len(), 69, "isohedral patch size");
        for tile in &rp.tiles {
            assert_eq!(tile.seed, 0, "single isohedral orbit -> seed 0 only");
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length())
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [1.0, 1.0, 2.0, 2.0, 2.0 * s3()];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "side {got} ∉ {{1, 1, 2, 2, 2√3}}");
            }
        }
    }
}

#[cfg(test)]
mod pentagon3_tests {
    use super::pentagon3::{cell_table, prototile, s3, tile_area, two_fold_centre};
    use crate::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon3::spec;
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Reflection matrix in the direction u (u must be unit): 2uuᵀ − I.
    fn mirror(u: Vec2) -> (f64, f64, f64, f64) {
        (2.0 * u.x * u.x - 1.0, 2.0 * u.x * u.y, 2.0 * u.x * u.y, 2.0 * u.y * u.y - 1.0)
    }

    /// Vertex-set key on a 1e-6 grid (shared joints agree to ~1e-15 between
    /// two computations of the same tile, since every cell is built from
    /// the same runtime prototile array).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    /// True when the tile's AABB lies inside `w`.  A tile whose AABB is
    /// inside `ra` has its polygon inside `ra`, so its image under any
    /// isometry g intersects g(ra) ⊆ bbox(g(ra)) — and a tile whose polygon
    /// intersects the region always lies inside the generator's padded
    /// index range — hence the image is always generated for rb.
    fn aabb_inside(poly: &Polygon, w: BBox) -> bool {
        let bb = poly.aabb();
        bb.min.x >= w.min.x
            && bb.min.y >= w.min.y
            && bb.max.x <= w.max.x
            && bb.max.y <= w.max.y
    }

    /// Verify that the isometry g = (M, t) preserves the tiling on `ra`:
    /// images of interior-AABB tiles of ra must be tiles generated for the
    /// image window bbox(g(ra)).  With `exact = true` (the half-turn case,
    /// g(ra) = ra) the filtered image set must equal the interior-AABB
    /// tile set of the window.
    fn check_iso(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            v2(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            v2(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ta = t.generate(params, &Query { region: ra });
        let tb = t.generate(params, &Query { region: rb });
        assert!(ta.message.is_none(), "{label}: {:?}", ta.message);
        assert!(tb.message.is_none(), "{label}: {:?}", tb.message);
        let src: Vec<_> = ta.tiles.iter().filter(|tile| aabb_inside(&tile.poly, ra)).collect();
        assert!(!src.is_empty(), "{label}: no interior tiles");
        let set_b: HashSet<Vec<(i64, i64)>> = tb
            .tiles
            .iter()
            .filter(|tile| !exact || aabb_inside(&tile.poly, rb))
            .map(|tile| vkey(&tile.poly.pts))
            .collect();
        let set_a: HashSet<Vec<(i64, i64)>> = src
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(set_a.len(), src.len(), "{label}: isometry must be injective on the patch");
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    #[test]
    fn pentagon_3_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "pentagon-3", &p, BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)));
    }

    #[test]
    fn pentagon_3_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-3(rot30,scale0.7)",
            BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_3_deterministic() {
        check_det(&spec(), "pentagon-3", BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)));
    }

    /// Unit structure of the primitive cell: 2 congruent copies of the
    /// exact-Q(√3) type-3 prototile (A = B = C = D = 120°, E = 60°,
    /// a = b = 2, c = 1, d = 4, e = 3, area 5√3) on the oblique lattice
    /// w1 = (1/2, 3√3/2), w2 = (−13/2, √3/2) with det = 10√3 = 2 × tile
    /// area.  The closure identity holds to ~1e-15, both cells are CCW
    /// and congruent (same chirality — no mirror in the cell table), and
    /// the seeds mark the single isohedral orbit.
    #[test]
    fn pentagon_3_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-3");
        assert_eq!(t.cells.len(), 2, "the primitive unit holds two tiles");
        assert_eq!(t.lattice[0], v2(0.5, 1.5 * s3()));
        assert_eq!(t.lattice[1], v2(-6.5, 0.5 * s3()));
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        let a0 = t.cells[0].poly.area();
        assert!((det - 2.0 * a0).abs() < 1e-11, "det {det} ≠ 2 × tile area");
        assert!((det - 10.0 * s3()).abs() < 1e-11, "det {det} ≠ 10√3");
        assert!((a0 - tile_area()).abs() < 1e-11, "tile area ≠ 5√3");
        // Oblique, not rectangular/hexagonal: w1·w2 ≠ 0, |w1| ≠ |w2|.
        assert!(w1.dot(w2).abs() > 1e-9, "the lattice must be oblique");
        assert!((w1.dot(w1) - 7.0).abs() < 1e-12, "|w1|² = 7");
        assert!((w2.dot(w2) - 43.0).abs() < 1e-12, "|w2|² = 43");
        // The prototile closes: edge directions 0°, 60°, 120°, 180° of
        // lengths 2, 2, 1, 4; closing edge E→A has direction 300°, length 3.
        let pr = prototile();
        assert!((pr[4].x + 1.5).abs() < 1e-12, "closure E.x = −3/2, got {}", pr[4].x);
        assert!((pr[4].y - 1.5 * s3()).abs() < 1e-12, "closure E.y = 3√3/2, got {}", pr[4].y);
        let proto = &t.cells[0].poly;
        assert_eq!(proto.pts[0], v2(0.0, 0.0), "prototile corner A at the origin");
        for (i, want) in [(0usize, 120.0f64), (1, 120.0), (2, 120.0), (3, 120.0), (4, 60.0)] {
            let got = angle_at(proto, i).to_degrees();
            assert!((got - want).abs() < 1e-9, "angle at vertex {i}: {got} ≠ {want}");
        }
        for (i, want) in [(0usize, 2.0f64), (1, 2.0), (2, 1.0), (3, 4.0), (4, 3.0)] {
            assert!((side_of(proto, i) - want).abs() < 1e-12, "side {i}");
        }
        // Type-3 family identities: a = b, d = c + e.
        assert!((side_of(proto, 0) - side_of(proto, 1)).abs() < 1e-12, "a = b");
        assert!((side_of(proto, 3) - (side_of(proto, 2) + side_of(proto, 4))).abs() < 1e-12, "d = c + e");
        // Single isohedral orbit: both seeds 0.
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 0], "1-isohedral seeds");
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        let mut seen: HashSet<Vec<(i64, i64)>> = HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            assert!((cell.poly.area() - a0).abs() < 1e-12, "cell {k} area");
            assert_eq!(cell.poly.len(), 5, "cell {k} vertex count");
            // Cyclic (side, angle) fingerprint against the prototile:
            // forward only — both cells carry the same chirality.
            let cs = (0..5).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..5).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd, "cell {k} is not a directly-congruent type-3 prototile");
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
        }
    }

    /// Window census (default parameters) and declared metadata: culling
    /// is the tile AABB intersecting the region (+1e-9), matching the
    /// independent Python emulation of the lattice.rs culling.
    #[test]
    fn pentagon_3_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::P2);
        assert_eq!(t.lattice_kind, LatticeKind::Oblique);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the cell-0 tile (seed 0)");
        let p = ParamSet::default_for(t.params());
        for (region, want) in [
            (BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)), 19),
            (BBox::new(v2(-6.0, -4.0), v2(8.0, 6.0)), 26),
            (BBox::new(v2(-8.0, -6.0), v2(10.0, 8.0)), 44),
            (BBox::new(v2(-10.0, -8.0), v2(14.0, 10.0)), 70),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }

    /// Refined edge census over 60 tiles (the tiling is NON-edge-to-edge
    /// at the pentagon level): of the sides whose midpoint lies strictly
    /// inside the window (−9.4, 13.4) × (−7.4, 9.4), 172 are matched by
    /// exactly one full side and 46 are d-sides split at their exact
    /// midpoint t = 1/2 into two length-2 halves, each contained in
    /// exactly one length-2 partner side (an a- or b-side); no side
    /// carries any other T-junction.  The window is an exact count with a
    /// minimum boundary margin of 0.0388 from any counted midpoint
    /// (verified against the exact Q(√3) tiling), so it is robust against
    /// the ~1e-15 float difference between the runtime-computed prototile
    /// and the exact surds.
    #[test]
    fn pentagon_3_refined_edge_census() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let region = BBox::new(v2(-9.4, -7.4), v2(13.4, 9.4));
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 60, "patch size");
        let verts: Vec<Vec<Vec2>> = r.tiles.iter().map(|tile| tile.poly.pts.clone()).collect();
        let n = verts.len();
        let mut n_split = 0;
        let mut n_full = 0;
        let mut bad: Vec<String> = Vec::new();
        for i in 0..n {
            for e in 0..5 {
                let (pp, qq) = (verts[i][e], verts[i][(e + 1) % 5]);
                let mid = (pp + qq) * 0.5;
                if !(mid.x > -9.4 && mid.x < 13.4 && mid.y > -7.4 && mid.y < 9.4) {
                    continue;
                }
                let dv = qq - pp;
                let l2 = dv.dot(dv);
                let l = l2.sqrt();
                // Split points: other tiles' vertices strictly inside the
                // open segment (collinear within 1e-7), binned at 1e-9 and
                // merged within 1e-8.
                let mut bins: BTreeSet<i64> = BTreeSet::new();
                for (j, vj) in verts.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    for &vv in vj.iter() {
                        let tt = (vv - pp).dot(dv) / l2;
                        if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                            continue;
                        }
                        let cross = (vv.x - pp.x) * dv.y - (vv.y - pp.y) * dv.x;
                        if cross.abs() / l < 1e-7 {
                            bins.insert((tt * 1e9).round() as i64);
                        }
                    }
                }
                let mut srt: Vec<i64> = Vec::new();
                for b in bins {
                    if srt.is_empty() || b - srt[srt.len() - 1] > 10 {
                        srt.push(b);
                    }
                }
                if srt.len() > 1 {
                    bad.push(format!("tile {i} edge {e}: {srt:?}"));
                    continue;
                }
                let mut ends: Vec<f64> = vec![0.0];
                ends.extend(srt.iter().map(|b| *b as f64 / 1e9));
                ends.push(1.0);
                for w in ends.windows(2) {
                    let aq = pp + dv * w[0];
                    let seg = dv * (w[1] - w[0]);
                    let lseg = seg.length();
                    let uh = seg * (1.0 / lseg);
                    let mut cnt = 0;
                    for (j, vj) in verts.iter().enumerate() {
                        if j == i {
                            continue;
                        }
                        for (a1, b1) in (0..5).map(|k| (vj[k], vj[(k + 1) % 5])) {
                            let c1 = (a1.x - aq.x) * uh.y - (a1.y - aq.y) * uh.x;
                            let c2 = (b1.x - aq.x) * uh.y - (b1.y - aq.y) * uh.x;
                            if c1.abs() > 1e-7 || c2.abs() > 1e-7 {
                                continue;
                            }
                            let ta = (a1 - aq).dot(uh);
                            let tb = (b1 - aq).dot(uh);
                            if ta.min(tb) <= 1e-7 * lseg && ta.max(tb) >= lseg * (1.0 - 1e-7) {
                                cnt += 1;
                            }
                        }
                    }
                    if cnt != 1 {
                        bad.push(format!(
                            "tile {i} edge {e} sub [{:.3}, {:.3}] has {cnt} covering edges",
                            w[0], w[1]
                        ));
                    }
                }
                if srt.len() == 1 {
                    n_split += 1;
                    assert!((l - 4.0).abs() < 1e-9, "split side must be the d-side, got {l}");
                    assert!(
                        (srt[0] as f64 / 1e9 - 0.5).abs() < 1e-6,
                        "d-side must split at its midpoint, got {srt:?}"
                    );
                } else {
                    n_full += 1;
                }
            }
        }
        assert!(bad.is_empty(), "refined edge census violations: {bad:?}");
        assert_eq!(n_full, 172, "full (unsplit) edges");
        assert_eq!(n_split, 46, "d-sides split at their midpoint");
    }

    /// Vertex-figure census over the window (−9.4, −7.4)–(13.4, 9.4):
    /// every interior vertex (margin 1e-3 from the window edge; the
    /// tightest counted midpoint in this window sits 0.0388 away, so the
    /// scan is boundary-safe) is a complete star — corner rays plus rays
    /// of edges passing through it (the T-junction points) — merging
    /// within 1e-4 rad, and its sectors are exactly the two figures
    /// {60°, 120°, 180°} (the A- and E-type points, 46 of them) and
    /// {120°, 120°, 120°} (the B-, C- and D-type points, 39 of them —
    /// the B- and D-classes coincide mod Λ); 85 vertices in total.
    #[test]
    fn pentagon_3_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let big = BBox::new(v2(-9.4, -7.4), v2(13.4, 9.4));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        const MARG: f64 = 1e-3;
        let mut pts: Vec<Vec2> = Vec::new();
        for tile in &r.tiles {
            for q in &tile.poly.pts {
                if q.x > big.min.x + MARG
                    && q.x < big.max.x - MARG
                    && q.y > big.min.y + MARG
                    && q.y < big.max.y - MARG
                    && !pts.iter().any(|o| (*o - *q).length() < 1e-5)
                {
                    pts.push(*q);
                }
            }
        }
        let figs: [(u32, u32, u32); 2] = [(60, 120, 180), (120, 120, 120)];
        let mut counts: BTreeMap<(u32, u32, u32), usize> = BTreeMap::new();
        for pt in &pts {
            let mut rays: BTreeSet<i64> = BTreeSet::new();
            for tile in &r.tiles {
                let nn = tile.poly.len();
                for k in 0..nn {
                    if (tile.poly.pts[k] - *pt).length() >= 1e-5 {
                        continue;
                    }
                    for &q in [&tile.poly.pts[(k + nn - 1) % nn], &tile.poly.pts[(k + 1) % nn]] {
                        let dir = q - *pt;
                        let th = dir.y.atan2(dir.x).rem_euclid(std::f64::consts::TAU);
                        rays.insert((th * 1e5).round() as i64);
                    }
                }
                for e in 0..nn {
                    let (a, b) = (tile.poly.pts[e], tile.poly.pts[(e + 1) % nn]);
                    let ab = b - a;
                    let l2 = ab.dot(ab);
                    let tt = (*pt - a).dot(ab) / l2;
                    if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                        continue;
                    }
                    if (*pt - a).cross(ab).abs() / l2.sqrt() < 1e-7 {
                        let th0 = ab.y.atan2(ab.x);
                        for add in [0.0, std::f64::consts::PI] {
                            let th = (th0 + add).rem_euclid(std::f64::consts::TAU);
                            rays.insert((th * 1e5).round() as i64);
                        }
                    }
                }
            }
            let mut mr: Vec<f64> = Vec::new();
            for k in &rays {
                let rv = *k as f64 / 1e5;
                if mr.is_empty() || rv - mr[mr.len() - 1] > 1e-4 {
                    mr.push(rv);
                }
            }
            if mr.len() > 1 && mr[mr.len() - 1] - mr[0] > std::f64::consts::TAU - 1e-4 {
                mr.pop();
            }
            let mut gaps: Vec<f64> = (0..mr.len())
                .map(|i| (mr[(i + 1) % mr.len()] - mr[i]).rem_euclid(std::f64::consts::TAU))
                .collect();
            gaps.retain(|g| *g > 1e-4);
            gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let fig: Vec<u32> = gaps.iter().map(|g| g.to_degrees().round() as u32).collect();
            assert_eq!(fig.len(), 3, "vertex {pt:?}: unexpected figure {fig:?}");
            assert_eq!(fig.iter().sum::<u32>(), 360, "vertex {pt:?} figure {fig:?}");
            let f3 = (fig[0], fig[1], fig[2]);
            assert!(figs.contains(&f3), "unexpected vertex figure {f3:?} at {pt:?}");
            *counts.entry(f3).or_insert(0) += 1;
        }
        assert_eq!(pts.len(), 85, "total interior vertices");
        let want: BTreeMap<(u32, u32, u32), usize> =
            [((60, 120, 180), 46), ((120, 120, 120), 39)].into_iter().collect();
        assert_eq!(counts, want, "vertex-figure census");
    }

    /// The wallpaper group is p2: the half-turn about c0 = (−3/4, 3√3/4)
    /// (the midpoint of the prototile's d-side) preserves the tiling
    /// exactly; all four classes d/2 + Λ/2 are realised half-turn centres
    /// (their half-turns map the cell-0 tile onto tiles); the lattice
    /// translations are symmetries; and no mirror or glide is — the
    /// prototile is chiral (side cycle (2, 2, 1, 4, 3) differs from every
    /// rotation of its reversal) and the tiling uses one chirality only,
    /// so no orientation-reversing candidate (any mirror direction through
    /// any tile vertex or half-turn centre, with any whole lattice shift)
    /// maps the prototile onto a tile.  The half-turns about the lattice
    /// points themselves and about the half-lattice points off
    /// d/2 + Λ/2 are not symmetries.
    #[test]
    fn pentagon_3_p2_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let c0 = two_fold_centre();
        let ra = BBox::new(v2(c0.x - 12.0, c0.y - 10.0), v2(c0.x + 12.0, c0.y + 10.0));
        check_iso(&t, &p, (-1.0, 0.0, 0.0, -1.0), c0 * 2.0, ra, true, "half-turn about c0");
        // Big patch for tile-image membership: every probed image below
        // (cell-0 images under i, j ∈ [−2, 2] lattice shifts, half-turns
        // and mirror candidates centred within one cell of the origin)
        // lies inside this footprint, so membership in the generated set
        // is the exact question.
        let big = BBox::new(v2(-16.0, -14.0), v2(20.0, 16.0));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        let set: HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();
        for i in -2..=2 {
            // sanity: pure lattice translations are symmetries
            for &w in [&w1, &w2] {
                let img: Vec<Vec2> = prot.iter().map(|q| *q + w * i as f64).collect();
                assert!(set.contains(&vkey(&img)), "cell0 + {i}·w must be a tile");
            }
        }
        // The four half-turn classes d/2 + Λ/2: 2c ≡ d mod Λ where d = E−A
        // = cell_table()[1].1, so (M, 2c) is a symmetry for every such c.
        let _d = cell_table()[1].1;
        let halves = [v2(0.0, 0.0), w1 * 0.5, w2 * 0.5, (w1 + w2) * 0.5];
        for h in halves {
            let c = c0 + h;
            for i in -1..=1 {
                for j in -1..=1 {
                    let tr = c * 2.0 + w1 * i as f64 + w2 * j as f64;
                    let img: Vec<Vec2> = prot.iter().map(|q| apply_iso((-1.0, 0.0, 0.0, -1.0), tr, *q)).collect();
                    assert!(
                        set.contains(&vkey(&img)),
                        "half-turn about {:?} + lattice must map cell0 onto a tile",
                        c
                    );
                }
            }
        }
        // Half-turns about the lattice points and the half-lattice points
        // off d/2 + Λ/2 are not symmetries (their squares are lattice
        // translations, so they would have to be centres of the p2 group).
        for c in [Vec2::ZERO, w1 * 0.5, w2 * 0.5, (w1 + w2) * 0.5] {
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso((-1.0, 0.0, 0.0, -1.0), c * 2.0, *q)).collect();
            assert!(!set.contains(&vkey(&img)), "half-turn about {c:?} must not be a symmetry");
        }
        // No orientation-reversing isometry: for every mirror direction
        // through every tile-vertex candidate centre (tile vertices of the
        // cell tiles, the half-turn centres, and the lattice/half-lattice
        // points) and every whole lattice shift, the mirrored prototile is
        // not a tile.  The chiral side cycle makes this exhaustive: any
        // mirror or glide has a mirror line through one of the realisable
        // fixed-point candidates mod the lattice, and a one-chirality
        // tiling of a chiral prototile admits none.
        let e60 = v2(0.5, 0.5 * s3());
        let dirs = [Vec2::new(1.0, 0.0), e60, v2(0.5 * s3(), 0.5), Vec2::new(0.0, 1.0)];
        let centres = [
            Vec2::ZERO,
            prot[1],
            prot[2],
            prot[3],
            prot[4],
            c0,
            w1 * 0.5,
            w2 * 0.5,
            (w1 + w2) * 0.5,
        ];
        for u in dirs {
            let m = mirror(u);
            for c in centres {
                for i in -1..=1 {
                    for j in -1..=1 {
                        let mc = v2(m.0 * c.x + m.1 * c.y, m.2 * c.x + m.3 * c.y);
                        let tr = v2(c.x - mc.x + w1.x * i as f64 + w2.x * j as f64, c.y - mc.y + w1.y * i as f64 + w2.y * j as f64);
                        let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, tr, *q)).collect();
                        assert!(
                            !set.contains(&vkey(&img)),
                            "mirror ({u:?} through {c:?} + {i}·w1 + {j}·w2) must not be a symmetry"
                        );
                    }
                }
            }
        }
    }

    /// 1-isohedral: both cells carry cell seed 0 (the shipped pattern
    /// seed is the decorative checkerboard lattice parity, not an orbit
    /// class) and the half-turn about c0 permutes the cell table mod
    /// lattice as the transposition (0 1), so the two cells form a
    /// single full-group orbit; every generated tile keeps the prototile
    /// side multiset {1, 2, 2, 3, 4}.
    #[test]
    fn pentagon_3_isohedral_single_orbit() {
        let t = spec();
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 0], "single isohedral class");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        let u_of = |p: Vec2| {
            v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
        };
        let mod1 = |x: f64| (x + 0.5).rem_euclid(1.0) - 0.5;
        let table = cell_table();
        let c0 = two_fold_centre();
        // The half-turn maps each cell's anchor to the other cell's anchor
        // mod lattice, rotating the tile by 180° (both chiralities equal:
        // the table has no mirror).
        let mut perm: Vec<usize> = Vec::new();
        for k in 0..2 {
            let (_fam_k, a_k, th_k) = table[k];
            let ai = apply_iso((-1.0, 0.0, 0.0, -1.0), c0 * 2.0, a_k);
            let th_img = 180.0 + th_k;
            let mut hit = None;
            for (j, &(_fam_j, a_j, th_j)) in table.iter().enumerate() {
                let du = u_of(ai - a_j);
                if mod1(du.x).abs() < 1e-6
                    && mod1(du.y).abs() < 1e-6
                    && ((th_img - th_j + 180.0).rem_euclid(360.0) - 180.0).abs() < 1e-6
                {
                    hit = Some(j);
                    break;
                }
            }
            perm.push(hit.unwrap_or_else(|| panic!("cell {k} has no half-turn image")));
        }
        assert_eq!(perm, vec![1, 0], "half-turn permutes the cells as (0 1)");
        // Generated patches: the single orbit carries no orbit seed, so
        // the shipped seed is the decorative checkerboard lattice parity
        // (0..1); prototile side multiset kept.
        let params = ParamSet::default_for(t.params());
        let rp = t.generate(&params, &Query { region: BBox::new(v2(-6.0, -6.0), v2(10.0, 10.0)) });
        assert!(rp.message.is_none(), "{:?}", rp.message);
        for tile in &rp.tiles {
            assert!(tile.seed < 2, "checkerboard seed must stay in {{0, 1}}");
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length())
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [1.0, 2.0, 2.0, 3.0, 4.0];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "side {got} ∉ {{1, 2, 2, 3, 4}}");
            }
        }
    }

    /// M112: the two cells of the primitive cell are one orbit
    /// (1-isohedral), so the shipped seed is the decorative
    /// `SeedPattern::Checker` lattice parity — a checkerboard over the
    /// oblique lattice basis, not an orbit class; a bounded view must
    /// show both parities (seeds stay in {0, 1}).
    #[test]
    fn pentagon_3_seed_is_decorative_checkerboard() {
        let t = spec();
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
mod pentagon2_tests {
    use super::pentagon2::{cell_table, prototile, s3, tile_area, two_fold_centre};
    use crate::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon2::spec;
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Reflection matrix in the direction u (u must be unit): 2uuᵀ − I.
    fn mirror(u: Vec2) -> (f64, f64, f64, f64) {
        (2.0 * u.x * u.x - 1.0, 2.0 * u.x * u.y, 2.0 * u.x * u.y, 2.0 * u.y * u.y - 1.0)
    }

    /// Vertex-set key on a 1e-6 grid (shared joints agree to ~1e-15 between
    /// two computations of the same tile, since every cell is built from
    /// the same runtime prototile array).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    /// True when the tile's AABB lies inside `w`.  A tile whose AABB is
    /// inside `ra` has its polygon inside `ra`, so its image under any
    /// isometry g intersects g(ra) ⊆ bbox(g(ra)) — and a tile whose polygon
    /// intersects the region always lies inside the generator's padded
    /// index range — hence the image is always generated for rb.
    fn aabb_inside(poly: &Polygon, w: BBox) -> bool {
        let bb = poly.aabb();
        bb.min.x >= w.min.x
            && bb.min.y >= w.min.y
            && bb.max.x <= w.max.x
            && bb.max.y <= w.max.y
    }

    /// Verify that the isometry g = (M, t) preserves the tiling on `ra`:
    /// images of interior-AABB tiles of ra must be tiles generated for the
    /// image window bbox(g(ra)).  With `exact = true` (the half-turn case,
    /// g(ra) = ra) the filtered image set must equal the interior-AABB
    /// tile set of the window.
    fn check_iso(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            v2(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            v2(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ta = t.generate(params, &Query { region: ra });
        let tb = t.generate(params, &Query { region: rb });
        assert!(ta.message.is_none(), "{label}: {:?}", ta.message);
        assert!(tb.message.is_none(), "{label}: {:?}", tb.message);
        let src: Vec<_> = ta.tiles.iter().filter(|tile| aabb_inside(&tile.poly, ra)).collect();
        assert!(!src.is_empty(), "{label}: no interior tiles");
        let set_b: HashSet<Vec<(i64, i64)>> = tb
            .tiles
            .iter()
            .filter(|tile| !exact || aabb_inside(&tile.poly, rb))
            .map(|tile| vkey(&tile.poly.pts))
            .collect();
        let set_a: HashSet<Vec<(i64, i64)>> = src
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(set_a.len(), src.len(), "{label}: isometry must be injective on the patch");
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    #[test]
    fn pentagon_2_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "pentagon-2", &p, BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)));
    }

    #[test]
    fn pentagon_2_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-2(rot30,scale0.7)",
            BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_2_deterministic() {
        check_det(&spec(), "pentagon-2", BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)));
    }

    /// Unit structure of the primitive cell: 4 congruent copies of the
    /// exact-Q(√3) type-2 prototile (A = B = D = 90°, C = 120°, E = 150°,
    /// a = 1, b = d = 1 + √3/2, c = e = 1/2, tile area 1 + √3/2) on the
    /// rectangular lattice w1 = (1/2, 1 + √3/2), w2 = (−2 − √3, 1) with
    /// w1 ⊥ w2 and det = 4 + 2√3 = 4 × tile area.  Both chiralities occur
    /// (two direct, two mirror cells), the seeds mark the single
    /// isohedral orbit, and every cell closes with no duplication.
    #[test]
    fn pentagon_2_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-2");
        assert_eq!(t.cells.len(), 4, "the primitive unit holds four tiles");
        assert_eq!(t.lattice[0], v2(0.5, 1.0 + s3() / 2.0));
        assert_eq!(t.lattice[1], v2(-2.0 - s3(), 1.0));
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        let a0 = t.cells[0].poly.area();
        assert!((det - 4.0 * a0).abs() < 1e-11, "det {det} ≠ 4 × tile area");
        assert!((det - (4.0 + 2.0 * s3())).abs() < 1e-11, "det {det} ≠ 4 + 2√3");
        assert!((a0 - tile_area()).abs() < 1e-11, "tile area ≠ 1 + √3/2");
        // Rectangular: w1 ⊥ w2, |w1|² = 2 + √3, |w2|² = 8 + 4√3.
        assert!(w1.dot(w2).abs() < 1e-12, "the lattice must be rectangular");
        assert!((w1.dot(w1) - (2.0 + s3())).abs() < 1e-12, "|w1|² = 2 + √3");
        assert!((w2.dot(w2) - (8.0 + 4.0 * s3())).abs() < 1e-12, "|w2|² = 8 + 4√3");
        // The prototile closes: edge directions 0°, 90°, 150°, 240° of
        // lengths b = 1 + √3/2, 1/2, b, 1/2; closing edge E→A has
        // direction 270°, length a = 1.
        let s = 1.0 + s3() / 2.0;
        let pr = prototile();
        assert!(pr[4].x.abs() < 1e-12, "closure E.x = 0, got {}", pr[4].x);
        assert!((pr[4].y - 1.0).abs() < 1e-12, "closure E.y = 1, got {}", pr[4].y);
        let proto = &t.cells[0].poly;
        assert_eq!(proto.pts[0], v2(0.0, 0.0), "prototile corner A at the origin");
        for (i, want) in [(0usize, 90.0f64), (1, 90.0), (2, 120.0), (3, 90.0), (4, 150.0)] {
            let got = angle_at(proto, i).to_degrees();
            assert!((got - want).abs() < 1e-9, "angle at vertex {i}: {got} ≠ {want}");
        }
        for (i, want) in [(0usize, s), (1, 0.5), (2, s), (3, 0.5), (4, 1.0)] {
            assert!((side_of(proto, i) - want).abs() < 1e-12, "side {i}");
        }
        // Type-2 family identities: c = e, d = b, B + D = 180°.
        assert!((side_of(proto, 1) - side_of(proto, 3)).abs() < 1e-12, "c = e");
        assert!((side_of(proto, 0) - side_of(proto, 2)).abs() < 1e-12, "d = b");
        let b_angle = angle_at(proto, 1).to_degrees();
        let d_angle = angle_at(proto, 3).to_degrees();
        assert!((b_angle + d_angle - 180.0).abs() < 1e-9, "B + D = 180°");
        // Chirality seed classes: the single isohedral orbit carries
        // the two chirality classes — direct pair 0, mirror pair 1.
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 0, 1, 1], "chirality seeds: direct 0, mirror 1");
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        // The mirrored prototile's CCW (side, angle) cycle, for the
        // reverse-cyclic fingerprint of the mirror cells.
        let ps_m: Vec<f64> = (0..5).map(|i| ps[(4 - i) % 5]).collect();
        let pa_m: Vec<f64> = (0..5).map(|i| pa[(5 - i) % 5]).collect();
        let mut seen: HashSet<Vec<(i64, i64)>> = HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            assert!((cell.poly.area() - a0).abs() < 1e-12, "cell {k} area");
            assert_eq!(cell.poly.len(), 5, "cell {k} vertex count");
            // Cyclic (side, angle) fingerprint against the prototile,
            // forward (direct cells) or reversed (mirror cells).
            let cs = (0..5).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..5).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            let rev = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps_m[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa_m[(i + rot) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd || rev, "cell {k} is not a congruent type-2 prototile");
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
        }
        // Both chiralities occur: two direct and two mirror cells.
        let fams: Vec<bool> = cell_table().iter().map(|c| c.0).collect();
        assert_eq!(fams.iter().filter(|f| **f).count(), 2, "two chiral pairs in the cell");
    }

    /// Window census (default parameters) and declared metadata: culling
    /// is the tile AABB intersecting the region (+1e-9), matching the
    /// independent Python emulation of the lattice.rs culling.
    #[test]
    fn pentagon_2_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::Pgg);
        assert_eq!(t.lattice_kind, LatticeKind::Rectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the cell-0 tile (seed 0)");
        let p = ParamSet::default_for(t.params());
        for (region, want) in [
            (BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)), 115),
            (BBox::new(v2(-8.0, -6.0), v2(10.0, 8.0)), 165),
            (BBox::new(v2(-10.0, -8.0), v2(14.0, 10.0)), 272),
            (
                BBox::new(
                    v2(-8.464101615137753, -8.165063509461095),
                    v2(16.678203230275507, 9.464101615137753),
                ),
                281,
            ),
            (
                BBox::new(v2(0.0, -1.4330127018922192), v2(3.982050807568877, 1.8660254037844386)),
                17,
            ),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }

    /// Refined edge census: the tiling is EDGE-TO-EDGE at the pentagon
    /// level.  Of the sides whose midpoint lies strictly inside the
    /// window (36-cell block inset by 0.05; minimum midpoint boundary
    /// margin 0.00144), every one of the 1182 is matched by exactly one
    /// full side of one neighbour, and no side carries a T-junction
    /// (zero split edges).  The window is an exact count, so it is robust
    /// against the ~1e-15 float difference between the runtime-computed
    /// prototile and the exact surds.
    #[test]
    fn pentagon_2_refined_edge_census() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let block = BBox::new(
            v2(-8.464101615137753, -8.165063509461095),
            v2(16.678203230275507, 9.464101615137753),
        );
        let region = BBox::new(block.min + v2(0.05, 0.05), block.max - v2(0.05, 0.05));
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 276, "patch size");
        let verts: Vec<Vec<Vec2>> = r.tiles.iter().map(|tile| tile.poly.pts.clone()).collect();
        let n = verts.len();
        let mut n_split = 0;
        let mut n_full = 0;
        let mut bad: Vec<String> = Vec::new();
        for i in 0..n {
            for e in 0..5 {
                let (pp, qq) = (verts[i][e], verts[i][(e + 1) % 5]);
                let mid = (pp + qq) * 0.5;
                if !(mid.x > region.min.x
                    && mid.x < region.max.x
                    && mid.y > region.min.y
                    && mid.y < region.max.y)
                {
                    continue;
                }
                let dv = qq - pp;
                let l2 = dv.dot(dv);
                // Split points: other tiles' vertices strictly inside the
                // open segment (collinear within 1e-7), binned at 1e-9 and
                // merged within 1e-8.
                let mut bins: BTreeSet<i64> = BTreeSet::new();
                for (j, vj) in verts.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    for &vv in vj.iter() {
                        let tt = (vv - pp).dot(dv) / l2;
                        if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                            continue;
                        }
                        let cross = (vv.x - pp.x) * dv.y - (vv.y - pp.y) * dv.x;
                        if cross.abs() / l2.sqrt() < 1e-7 {
                            bins.insert((tt * 1e9).round() as i64);
                        }
                    }
                }
                let mut srt: Vec<i64> = Vec::new();
                for b in bins {
                    if srt.is_empty() || b - srt[srt.len() - 1] > 10 {
                        srt.push(b);
                    }
                }
                let mut ends: Vec<f64> = vec![0.0];
                ends.extend(srt.iter().map(|b| *b as f64 / 1e9));
                ends.push(1.0);
                for w in ends.windows(2) {
                    let aq = pp + dv * w[0];
                    let seg = dv * (w[1] - w[0]);
                    let lseg = seg.length();
                    let uh = seg * (1.0 / lseg);
                    let mut cnt = 0;
                    for (j, vj) in verts.iter().enumerate() {
                        if j == i {
                            continue;
                        }
                        for (a1, b1) in (0..5).map(|k| (vj[k], vj[(k + 1) % 5])) {
                            let c1 = (a1.x - aq.x) * uh.y - (a1.y - aq.y) * uh.x;
                            let c2 = (b1.x - aq.x) * uh.y - (b1.y - aq.y) * uh.x;
                            if c1.abs() > 1e-7 || c2.abs() > 1e-7 {
                                continue;
                            }
                            let ta = (a1 - aq).dot(uh);
                            let tb = (b1 - aq).dot(uh);
                            if ta.min(tb) <= 1e-7 * lseg && ta.max(tb) >= lseg * (1.0 - 1e-7) {
                                cnt += 1;
                            }
                        }
                    }
                    if cnt != 1 {
                        bad.push(format!(
                            "tile {i} edge {e} sub [{:.3}, {:.3}] has {cnt} covering edges",
                            w[0], w[1]
                        ));
                    }
                }
                if srt.len() == 1 {
                    n_split += 1;
                    bad.push(format!("tile {i} edge {e}: split at {srt:?} (tiling is edge-to-edge)"));
                } else {
                    n_full += 1;
                }
            }
        }
        assert!(bad.is_empty(), "refined edge census violations: {bad:?}");
        assert_eq!(n_full, 1182, "full (unsplit) edges");
        assert_eq!(n_split, 0, "no split sides");
    }

    /// Vertex-figure census over the 36-cell block window: every interior
    /// vertex (margin 1e-3 from the window edge; the star-to-boundary
    /// margin is 0.067, so the scan is boundary-safe) is a complete star
    /// with sectors exactly {90°, 120°, 150°} (237 of them) or
    /// {90°, 90°, 90°, 90°} (118 of them); 355 vertices in total.  The
    /// edge-through-ray pass runs as a T-junction detector: because the
    /// tiling is edge-to-edge no edge passes through a vertex, so that
    /// pass contributes no rays here — a T-junction would inject a 180°
    /// sector and fail the figure check.
    #[test]
    fn pentagon_2_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let big = BBox::new(
            v2(-8.464101615137753, -8.165063509461095),
            v2(16.678203230275507, 9.464101615137753),
        );
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        const MARG: f64 = 1e-3;
        let mut pts: Vec<Vec2> = Vec::new();
        for tile in &r.tiles {
            for q in &tile.poly.pts {
                if q.x > big.min.x + MARG
                    && q.x < big.max.x - MARG
                    && q.y > big.min.y + MARG
                    && q.y < big.max.y - MARG
                    && !pts.iter().any(|o| (*o - *q).length() < 1e-5)
                {
                    pts.push(*q);
                }
            }
        }
        // 3-ray stars pad with a trailing zero: (90, 120, 150, 0).
        let figs = [(90u32, 120, 150, 0), (90, 90, 90, 90)];
        let mut counts: BTreeMap<(u32, u32, u32, u32), usize> = BTreeMap::new();
        for pt in &pts {
            let mut rays: BTreeSet<i64> = BTreeSet::new();
            for tile in &r.tiles {
                let nn = tile.poly.len();
                for k in 0..nn {
                    if (tile.poly.pts[k] - *pt).length() >= 1e-5 {
                        continue;
                    }
                    for &q in [&tile.poly.pts[(k + nn - 1) % nn], &tile.poly.pts[(k + 1) % nn]] {
                        let dir = q - *pt;
                        let th = dir.y.atan2(dir.x).rem_euclid(std::f64::consts::TAU);
                        rays.insert((th * 1e5).round() as i64);
                    }
                }
                for e in 0..nn {
                    let (a, b) = (tile.poly.pts[e], tile.poly.pts[(e + 1) % nn]);
                    let ab = b - a;
                    let l2 = ab.dot(ab);
                    let tt = (*pt - a).dot(ab) / l2;
                    if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                        continue;
                    }
                    if (*pt - a).cross(ab).abs() / l2.sqrt() < 1e-7 {
                        let th0 = ab.y.atan2(ab.x);
                        for add in [0.0, std::f64::consts::PI] {
                            let th = (th0 + add).rem_euclid(std::f64::consts::TAU);
                            rays.insert((th * 1e5).round() as i64);
                        }
                    }
                }
            }
            let mut mr: Vec<f64> = Vec::new();
            for k in &rays {
                let rv = *k as f64 / 1e5;
                if mr.is_empty() || rv - mr[mr.len() - 1] > 1e-4 {
                    mr.push(rv);
                }
            }
            if mr.len() > 1 && mr[mr.len() - 1] - mr[0] > std::f64::consts::TAU - 1e-4 {
                mr.pop();
            }
            let mut gaps: Vec<f64> = (0..mr.len())
                .map(|i| (mr[(i + 1) % mr.len()] - mr[i]).rem_euclid(std::f64::consts::TAU))
                .collect();
            gaps.retain(|g| *g > 1e-4);
            gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let fig: Vec<u32> = gaps.iter().map(|g| g.to_degrees().round() as u32).collect();
            assert!(
                fig.len() == 3 || fig.len() == 4,
                "vertex {pt:?}: unexpected figure {fig:?}"
            );
            assert_eq!(fig.iter().sum::<u32>(), 360, "vertex {pt:?} figure {fig:?}");
            let f4 = (
                fig.first().copied().unwrap_or(0),
                fig.get(1).copied().unwrap_or(0),
                fig.get(2).copied().unwrap_or(0),
                fig.get(3).copied().unwrap_or(0),
            );
            assert!(figs.contains(&f4), "unexpected vertex figure {f4:?} at {pt:?}");
            *counts.entry(f4).or_insert(0) += 1;
        }
        assert_eq!(pts.len(), 355, "total interior vertices");
        let want: BTreeMap<(u32, u32, u32, u32), usize> =
            [((90, 120, 150, 0), 237), ((90, 90, 90, 90), 118)].into_iter().collect();
        assert_eq!(counts, want, "vertex-figure census");
    }

    /// The wallpaper group is pgg: the half-turn about c0 = (1 + √3/2, 0)
    /// (the prototile's B vertex) preserves the tiling exactly; all four
    /// classes c0 + Λ/2 are realised half-turn centres (their half-turns
    /// map the cell-0 tile onto tiles); the two glide families (axes along
    /// the lattice directions, g1² = w1, g2² = −w2, composing to the
    /// half-turn about c0) and their lattice shifts map the prototile onto
    /// tiles; and no pure reflection — the perpendicular-part candidates
    /// on the glide axes, nor any whole-lattice-shift candidate — maps the
    /// prototile onto a tile.
    #[test]
    fn pentagon_2_pgg_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let c0 = two_fold_centre();
        let ra = BBox::new(v2(c0.x - 12.0, c0.y - 10.0), v2(c0.x + 12.0, c0.y + 10.0));
        check_iso(&t, &p, (-1.0, 0.0, 0.0, -1.0), c0 * 2.0, ra, true, "half-turn about c0");
        let u1 = w1 * (1.0 / w1.length());
        let u2 = w2 * (1.0 / w2.length());
        let m1 = mirror(u1);
        let m2 = mirror(u2);
        let t1 = cell_table()[2].1; // the 150° mirror pair's anchor
        let t2 = cell_table()[3].1; // the 330° mirror pair's anchor
        // Exact generator identities: g² = (I + M)·t.
        let gs1 = v2((m1.0 + 1.0) * t1.x + m1.1 * t1.y, m1.2 * t1.x + (m1.3 + 1.0) * t1.y);
        assert!((gs1 - w1).length() < 1e-9, "g1² must equal w1");
        let gs2 = v2((m2.0 + 1.0) * t2.x + m2.1 * t2.y, m2.2 * t2.x + (m2.3 + 1.0) * t2.y);
        assert!((gs2 + w2).length() < 1e-9, "g2² must equal −w2");
        // Each glide shifts by exactly half a primitive translation along
        // its axis — g1 by +1/2 of w1, g2 by −1/2 of w2 — so no power of
        // it is a pure reflection.
        let half1 = 2.0 * t1.dot(u1) / w1.length();
        assert!((half1 - 1.0).abs() < 1e-6, "g1 must shift half a period");
        let half2 = 2.0 * t2.dot(u2) / w2.length();
        assert!((half2 + 1.0).abs() < 1e-6, "g2 must shift half a period");
        // g2∘g1 is the half-turn centred at c0 itself.
        let comp = v2(t2.x + m2.0 * t1.x + m2.1 * t1.y, t2.y + m2.2 * t1.x + m2.3 * t1.y);
        assert!((comp * 0.5 - c0).length() < 1e-9, "centre(g2∘g1) must be c0");
        // The glides preserve the tiling (the image window is tilted, so
        // check containment of the images among the tiles generated there)
        // — and so do the inverses and the lattice-shifted variants.
        check_iso(&t, &p, m1, t1, ra, false, "glide g1 (axis ∥ w1)");
        check_iso(
            &t,
            &p,
            m1,
            v2(-(m1.0 * t1.x + m1.1 * t1.y), -(m1.2 * t1.x + m1.3 * t1.y)),
            ra,
            false,
            "glide g1⁻¹",
        );
        check_iso(&t, &p, m2, t2, ra, false, "glide g2 (axis ∥ w2)");
        check_iso(
            &t,
            &p,
            m2,
            v2(-(m2.0 * t2.x + m2.1 * t2.y), -(m2.2 * t2.x + m2.3 * t2.y)),
            ra,
            false,
            "glide g2⁻¹",
        );
        // Big patch for tile-image membership.
        let big = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        let set: HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();
        for i in -2..=2 {
            // sanity: pure lattice translations are symmetries
            for &w in [&w1, &w2] {
                let img: Vec<Vec2> = prot.iter().map(|q| *q + w * i as f64).collect();
                assert!(set.contains(&vkey(&img)), "cell0 + {i}·w must be a tile");
            }
        }
        // The four half-turn classes c0 + Λ/2: every such half-turn maps
        // the cell-0 tile onto a tile.
        let halves = [v2(0.0, 0.0), w1 * 0.5, w2 * 0.5, (w1 + w2) * 0.5];
        for h in halves {
            let c = c0 + h;
            for i in -1..=1 {
                for j in -1..=1 {
                    let tr = c * 2.0 + w1 * i as f64 + w2 * j as f64;
                    let img: Vec<Vec2> =
                        prot.iter().map(|q| apply_iso((-1.0, 0.0, 0.0, -1.0), tr, *q)).collect();
                    assert!(
                        set.contains(&vkey(&img)),
                        "half-turn about {c:?} + lattice must map cell0 onto a tile"
                    );
                }
            }
        }
        // Lattice-shifted glide variants.
        for (i, j) in [(0.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (1.0, 0.0)] {
            let sh = w1 * i + w2 * j;
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m1, t1 + sh, *q)).collect();
            assert!(
                set.contains(&vkey(&img)),
                "(g1 + {i}·w1 + {j}·w2) must map cell0 onto a tile"
            );
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m2, t2 + sh, *q)).collect();
            assert!(
                set.contains(&vkey(&img)),
                "(g2 + {i}·w1 + {j}·w2) must map cell0 onto a tile"
            );
        }
        // No pure reflection: the mirror-class candidates carrying only the
        // perpendicular glide part (a reflection would need zero along-axis
        // shift), and every whole-lattice-shift candidate, must fail to map
        // the prototile onto a tile.
        let p1 = t1 - u1 * t1.dot(u1);
        let p2 = t2 - u2 * t2.dot(u2);
        for (m, tr, nm) in [(m1, p1, "(M1, t1⊥)"), (m2, p2, "(M2, t2⊥)")] {
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, tr, *q)).collect();
            assert!(!set.contains(&vkey(&img)), "{nm} must not be a symmetry (no mirrors in pgg)");
        }
        for i in -2..=2 {
            for (m, w, mn, wn) in
                [(m1, w1, "M1", "w1"), (m1, w2, "M1", "w2"), (m2, w1, "M2", "w1"), (m2, w2, "M2", "w2")]
            {
                let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, w * i as f64, *q)).collect();
                assert!(
                    !set.contains(&vkey(&img)),
                    "({mn}, {i}·{wn}) must not be a symmetry (no mirrors in pgg)"
                );
            }
        }
    }

    /// 1-isohedral: the four cells carry the two chirality seed classes
    /// (direct pair seed 0, mirror pair seed 1), and the half-turn plus
    /// the two glide classes permute the cell table as a single geometric
    /// orbit — r = (0 1)(2 3), g1 = (0 2)(1 3), g2 = (0 3)(1 2), the
    /// glides doing the identifying across chiralities; every generated
    /// tile keeps the prototile side multiset {1/2, 1/2, 1, 1 + √3/2,
    /// 1 + √3/2} and its chirality seed.
    #[test]
    fn pentagon_2_isohedral_single_orbit() {
        let t = spec();
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 0, 1, 1], "chirality classes: direct pair 0, mirror pair 1");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        let u_of = |p: Vec2| {
            v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
        };
        let mod1 = |x: f64| (x + 0.5).rem_euclid(1.0) - 0.5;
        let table = cell_table();
        // Which cell does the isometry (M, tr) send each cell to?  The
        // image anchor must land on the target anchor mod lattice; a
        // direct image keeps the family and adds its rotation, a mirror
        // image flips the family and subtracts the mirror's axis angle
        // doubled from the rotation.
        let perm_of = |m: (f64, f64, f64, f64), tr: Vec2| -> Vec<usize> {
            let rot = m.2.atan2(m.0).to_degrees();
            let is_direct = m.0 * m.3 - m.1 * m.2 > 0.0;
            let mut perm: Vec<usize> = Vec::new();
            for k in 0..4 {
                let (fam_k, a_k, th_k) = table[k];
                let ai = apply_iso(m, tr, a_k);
                let (want_fam, th_img) = if is_direct {
                    (fam_k, rot + th_k)
                } else {
                    (!fam_k, rot - th_k)
                };
                let mut hit = None;
                for (j, &(fam_j, a_j, th_j)) in table.iter().enumerate() {
                    let du = u_of(ai - a_j);
                    if mod1(du.x).abs() < 1e-6
                        && mod1(du.y).abs() < 1e-6
                        && fam_j == want_fam
                        && ((th_img - th_j + 180.0).rem_euclid(360.0) - 180.0).abs() < 1e-6
                    {
                        hit = Some(j);
                        break;
                    }
                }
                perm.push(hit.unwrap_or_else(|| panic!("cell {k} has no image under the isometry")));
            }
            perm
        };
        let c0 = two_fold_centre();
        let r = perm_of((-1.0, 0.0, 0.0, -1.0), c0 * 2.0);
        assert_eq!(r, vec![1, 0, 3, 2], "half-turn permutes the cells as (0 1)(2 3)");
        let u1 = w1 * (1.0 / w1.length());
        let u2 = w2 * (1.0 / w2.length());
        let g1 = perm_of(mirror(u1), table[2].1);
        assert_eq!(g1, vec![2, 3, 0, 1], "glide g1 permutes the cells as (0 2)(1 3)");
        let g2 = perm_of(mirror(u2), table[3].1);
        assert_eq!(g2, vec![3, 2, 1, 0], "glide g2 permutes the cells as (0 3)(1 2)");
        // The three generators move cell 0 to every cell: one orbit.
        let mut orbit: BTreeSet<usize> = BTreeSet::new();
        let mut stack = vec![0usize];
        while let Some(k) = stack.pop() {
            if !orbit.insert(k) {
                continue;
            }
            for perm in [&r, &g1, &g2] {
                if !orbit.contains(&perm[k]) {
                    stack.push(perm[k]);
                }
            }
        }
        assert_eq!(orbit, BTreeSet::from([0usize, 1, 2, 3]), "single isohedral orbit");
        // Generated patches: the shipped coloring distinguishes the two
        // chiral classes — direct pair (proto 0, 1) seed 0, mirror pair
        // (proto 2, 3) seed 1 — and keeps the prototile side multiset.
        let params = ParamSet::default_for(t.params());
        let rp = t.generate(
            &params,
            &Query { region: BBox::new(v2(-6.0, -6.0), v2(10.0, 10.0)) },
        );
        assert!(rp.message.is_none(), "{:?}", rp.message);
        let s = 1.0 + s3() / 2.0;
        let mut seen = [false; 2];
        for tile in &rp.tiles {
            assert_eq!(tile.seed, u32::from(tile.proto >= 2), "tile kept its chirality seed");
            seen[tile.seed as usize] = true;
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length())
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [0.5, 0.5, 1.0, s, s];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!((got - w).abs() < 1e-12, "side {got} not in the prototile side multiset");
            }
        }
        assert!(seen[0] && seen[1], "both chiral seed classes must appear");
    }

    /// M112: the shipped coloring distinguishes the two chiralities —
    /// the direct chiral pair (cells 0, 1) carries seed 0, the mirror
    /// pair (cells 2, 3) carries seed 1 — and every generated tile
    /// keeps the seed class of its cell; a bounded view must show both
    /// classes.
    #[test]
    fn pentagon_2_seed_classes_track_chirality() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region: BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let mut seen = [false; 2];
        for tile in &r.tiles {
            let want = u32::from(tile.proto >= 2);
            assert_eq!(tile.seed, want, "proto {} must carry its cell's chirality seed", tile.proto);
            seen[tile.seed as usize] = true;
        }
        assert!(seen[0] && seen[1], "both chiral classes must appear");
    }
}

#[cfg(test)]
mod pentagon7_tests {
    use super::pentagon7::{angles, closure_f, raw_cells, two_fold_centre, MIRROR};
    use crate::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon7::spec;
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Reflection matrix in the direction u (u must be unit): 2uuᵀ − I.
    fn mirror(u: Vec2) -> (f64, f64, f64, f64) {
        (2.0 * u.x * u.x - 1.0, 2.0 * u.x * u.y, 2.0 * u.x * u.y, 2.0 * u.y * u.y - 1.0)
    }

    /// Vertex-set key on a 1e-6 grid (distinct tiling vertices are ≥ 0.5
    /// apart — the shortest side is 1/2 — and two computations of a shared
    /// joint agree to ~1e-15 because every cell is built from the same
    /// runtime prototile array).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    /// True when the tile's AABB lies inside `w`.  A tile whose AABB is
    /// inside `ra` has its polygon inside `ra`, so its image under any
    /// isometry g intersects g(ra) ⊆ bbox(g(ra)) — and a tile whose polygon
    /// intersects the region always lies inside the generator's padded
    /// index range — hence the image is always generated for rb.  Tiles
    /// whose AABB merely touches the boundary (polygon outside) are
    /// excluded: their images legitimately escape rb.
    fn aabb_inside(poly: &Polygon, w: BBox) -> bool {
        let bb = poly.aabb();
        bb.min.x >= w.min.x
            && bb.min.y >= w.min.y
            && bb.max.x <= w.max.x
            && bb.max.y <= w.max.y
    }

    /// Verify that the isometry g = (M, t) preserves the tiling on `ra`:
    /// images of interior-AABB tiles of ra must be tiles generated for the
    /// image window bbox(g(ra)).  With `exact = true` (the half-turn case,
    /// g(ra) = ra) the filtered image set must equal the interior-AABB
    /// tile set of the window.
    fn check_iso(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            v2(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            v2(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ta = t.generate(params, &Query { region: ra });
        let tb = t.generate(params, &Query { region: rb });
        assert!(ta.message.is_none(), "{label}: {:?}", ta.message);
        assert!(tb.message.is_none(), "{label}: {:?}", tb.message);
        let src: Vec<_> = ta.tiles.iter().filter(|tile| aabb_inside(&tile.poly, ra)).collect();
        assert!(!src.is_empty(), "{label}: no interior tiles");
        let set_b: HashSet<Vec<(i64, i64)>> = tb
            .tiles
            .iter()
            .filter(|tile| !exact || aabb_inside(&tile.poly, rb))
            .map(|tile| vkey(&tile.poly.pts))
            .collect();
        let set_a: HashSet<Vec<(i64, i64)>> = src
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(set_a.len(), src.len(), "{label}: isometry must be injective on the patch");
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    /// Centroid of a stored cell (mean of its five vertices).
    fn cent(pts: &[Vec2]) -> Vec2 {
        let s = pts.iter().fold(Vec2::ZERO, |acc, &p| acc + p);
        s / 5.0
    }

    #[test]
    fn pentagon_7_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "pentagon-7", &p, BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)));
    }

    #[test]
    fn pentagon_7_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-7(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(7.0, 7.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_7_deterministic() {
        check_det(&spec(), "pentagon-7", BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)));
    }

    /// Unit structure of the primitive cell: 8 congruent copies of the
    /// runtime-computed Kershner prototile (A = 58.8626451020303°,
    /// B = 148°, C = 132.8626451020303°, D = 94.2747097959394°,
    /// E = 106°, sides b = c = d = e = 1/2, a = 1.035794206008535) on the
    /// rectangular lattice w1 ⊥ w2 with det = 8 × tile area =
    /// 4.236326192143856.  The family identities hold to ~1e-12, every
    /// cell is CCW, no two cells coincide, every cell is exactly the
    /// applet construction (mirrors vertex-reversed), and the seeds follow
    /// the two full-group isohedral orbits.
    #[test]
    fn pentagon_7_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-7");
        assert_eq!(t.cells.len(), 8, "the primitive unit holds eight tiles");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        // The lattice is computed from the construction, not hardcoded:
        // w1 = cell2.p3 − cell0.p0 (both direct); w2 = −(cell5.p3 −
        // cell3.p1) (cells 3/5 are mirror-stored, so their raw p3/p1 sit
        // at stored indices 2/4).
        assert!(
            (t.lattice[0] - (t.cells[2].poly.pts[3] - t.cells[0].poly.pts[0])).length() < 1e-15,
            "w1 must be cell2.p3 − cell0.p0"
        );
        assert!(
            (t.lattice[1] + (t.cells[5].poly.pts[2] - t.cells[3].poly.pts[4])).length() < 1e-15,
            "w2 must be −(cell5.p3 − cell3.p1)"
        );
        assert!(w1.dot(w2).abs() < 1e-12, "lattice must be rectangular (w1 ⊥ w2)");
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        let a0 = t.cells[0].poly.area();
        assert!((det - 8.0 * a0).abs() < 1e-11, "det {det} ≠ 8 × tile area");
        assert!((det - 4.236326192143856).abs() < 1e-9, "det {det}");
        assert!((a0 - 0.529540774017982).abs() < 1e-9, "tile area {a0}");
        // The runtime-computed instance: angles in vertex order (A, B, C, D, E)
        // and the closure root.
        let ang = angles();
        for (got, want) in ang
            .iter()
            .zip([58.8626451020303, 148.0, 132.8626451020303, 94.2747097959394, 106.0].iter())
        {
            assert!((got - want).abs() < 1e-9, "angle {got} ≠ {want}");
        }
        assert!(closure_f(ang[2]).abs() < 1e-12, "the closure equation must hold at C");
        let [a, b, c, d, e] = ang;
        assert!((b + 2.0 * e - 360.0).abs() < 1e-12, "B + 2E = 360°");
        assert!((2.0 * c + d - 360.0).abs() < 1e-12, "2C + D = 360°");
        assert!((2.0 * a + b + d - 360.0).abs() < 1e-12, "2A + B + D = 360°");
        assert!((a - (c - b / 2.0)).abs() < 1e-12, "A = C − B/2");
        // The prototile (cell 0): sides b = c = d = e = 1/2, a =
        // 1.035794206008535; angles in vertex order.
        let proto = &t.cells[0].poly;
        for i in 0..4 {
            assert!((side_of(proto, i) - 0.5).abs() < 1e-15, "proto side {i} must be 1/2");
        }
        assert!((side_of(proto, 4) - 1.035794206008535).abs() < 1e-9, "proto side 4 = a");
        for (i, want) in [58.8626451020303, 148.0, 132.8626451020303, 94.2747097959394, 106.0]
            .iter()
            .enumerate()
        {
            let got = angle_at(proto, i).to_degrees();
            assert!((got - want).abs() < 1e-9, "proto angle at vertex {i}: {got} ≠ {want}");
        }
        // Seeds follow the two full-group isohedral orbits (each orbit
        // mixes both chiralities; the glides identify them).
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 1, 1, 0, 0, 1, 1, 0], "isohedral seed pattern");
        let a4 = side_of(proto, 4);
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        let raw = raw_cells();
        let mut seen: HashSet<Vec<(i64, i64)>> = HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            assert!((cell.poly.area() - a0).abs() < 1e-12, "cell {k} area");
            assert_eq!(cell.poly.len(), 5, "cell {k} vertex count");
            // Side multiset {1/2, 1/2, 1/2, 1/2, a}.
            let mut sides: Vec<f64> = (0..5).map(|i| side_of(&cell.poly, i)).collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [0.5, 0.5, 0.5, 0.5, a4];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!(
                    (got - w).abs() < 1e-12,
                    "cell {k} side {got} ∉ {{1/2, 1/2, 1/2, 1/2, a}}"
                );
            }
            // Cyclic (side, angle) fingerprint against the prototile,
            // forward or reversed (the cells realise both chiralities).
            let cs = (0..5).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..5).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            let rev = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(rot + 9 - i) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(rot + 5 - i) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd || rev, "cell {k} is not a congruent type-7 prototile");
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
            // Stored cells equal the raw applet construction exactly
            // (mirrors vertex-reversed to stay CCW).
            let want: [Vec2; 5] = if MIRROR[k] {
                [raw[k][0], raw[k][4], raw[k][3], raw[k][2], raw[k][1]]
            } else {
                raw[k]
            };
            assert_eq!(
                cell.poly.pts,
                want.to_vec(),
                "cell {k} must match the raw construction exactly"
            );
        }
    }

    /// Window census (default parameters) and declared metadata: culling
    /// is the tile AABB intersecting the region (+1e-9), matching the
    /// independent Python emulation of the lattice.rs culling.
    #[test]
    fn pentagon_7_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::Pgg);
        assert_eq!(t.lattice_kind, LatticeKind::Rectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the cell-0 tile (seed 0)");
        let p = ParamSet::default_for(t.params());
        for (region, want) in [
            (BBox::new(v2(-4.0, -3.0), v2(10.0, 10.0)), 398),
            (BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)), 369),
            (BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)), 193),
            (BBox::new(v2(-6.0, -6.0), v2(22.0, 14.0)), 1149),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }

    /// Refined edge census over 907 tiles: the tiling is edge-to-edge at
    /// the pentagon level — of the sides whose midpoint lies strictly
    /// inside the window, none is split by a foreign vertex, and each is
    /// covered by exactly one full partner side (3770 full edges, 0
    /// splits).
    #[test]
    fn pentagon_7_refined_edge_census() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let region = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 907, "patch size");
        let verts: Vec<Vec<Vec2>> = r.tiles.iter().map(|tile| tile.poly.pts.clone()).collect();
        let n = verts.len();
        let mut n_full = 0;
        let mut bad: Vec<String> = Vec::new();
        for i in 0..n {
            for e in 0..5 {
                let (pp, qq) = (verts[i][e], verts[i][(e + 1) % 5]);
                let mid = (pp + qq) * 0.5;
                if !(mid.x > -7.5 && mid.x < 13.5 && mid.y > -7.5 && mid.y < 11.5) {
                    continue;
                }
                let dv = qq - pp;
                let l2 = dv.dot(dv);
                let l = l2.sqrt();
                // Split points: other tiles' vertices strictly inside the
                // open segment (collinear within 1e-7), binned at 1e-9 and
                // merged within gap 10 bins.  Edge-to-edge: these bins must
                // be empty.
                let mut bins: BTreeSet<i64> = BTreeSet::new();
                for (j, vj) in verts.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    for &vv in vj.iter() {
                        let tt = (vv - pp).dot(dv) / l2;
                        if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                            continue;
                        }
                        let cross = (vv.x - pp.x) * dv.y - (vv.y - pp.y) * dv.x;
                        if cross.abs() / l < 1e-7 {
                            bins.insert((tt * 1e9).round() as i64);
                        }
                    }
                }
                let mut srt: Vec<i64> = Vec::new();
                for b in bins {
                    if srt.is_empty() || b - srt[srt.len() - 1] > 10 {
                        srt.push(b);
                    }
                }
                if !srt.is_empty() {
                    bad.push(format!("tile {i} edge {e}: split at {srt:?}"));
                    continue;
                }
                n_full += 1;
                // The whole edge must be covered by exactly one full
                // partner side.
                let uh = dv * (1.0 / l);
                let mut cnt = 0;
                for (j, vj) in verts.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    for (a1, b1) in (0..5).map(|k| (vj[k], vj[(k + 1) % 5])) {
                        let c1 = (a1.x - pp.x) * uh.y - (a1.y - pp.y) * uh.x;
                        let c2 = (b1.x - pp.x) * uh.y - (b1.y - pp.y) * uh.x;
                        if c1.abs() > 1e-7 || c2.abs() > 1e-7 {
                            continue;
                        }
                        let ta = (a1 - pp).dot(uh);
                        let tb = (b1 - pp).dot(uh);
                        if ta.min(tb) <= 1e-7 * l && ta.max(tb) >= l * (1.0 - 1e-7) {
                            cnt += 1;
                        }
                    }
                }
                if cnt != 1 {
                    bad.push(format!("tile {i} edge {e} has {cnt} covering partner sides"));
                }
            }
        }
        assert!(bad.is_empty(), "edge census violations: {bad:?}");
        assert_eq!(n_full, 3770, "full (unsplit) edges");
    }

    /// Vertex-figure census over the 907-tile patch (margin 2.0): every
    /// interior vertex is a complete star — corner rays plus rays of edges
    /// passing through it — merging within 1e-4 rad; the three vertex
    /// figures are {E,E,B} = (106,106,148) [271], {D,C,C} = (94,133,133)
    /// [266] and {A,A,D,B} = (59,59,94,148) [279], each summing to 360°;
    /// 816 vertices in total.
    #[test]
    fn pentagon_7_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let big = BBox::new(v2(-8.0, -8.0), v2(14.0, 12.0));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        const MARG: f64 = 2.0;
        let mut pts: Vec<Vec2> = Vec::new();
        for tile in &r.tiles {
            for q in &tile.poly.pts {
                if q.x > big.min.x + MARG
                    && q.x < big.max.x - MARG
                    && q.y > big.min.y + MARG
                    && q.y < big.max.y - MARG
                    && !pts.iter().any(|o| (*o - *q).length() < 1e-4)
                {
                    pts.push(*q);
                }
            }
        }
        let mut n3: BTreeMap<(u32, u32, u32), usize> = BTreeMap::new();
        let mut n4: BTreeMap<(u32, u32, u32, u32), usize> = BTreeMap::new();
        for pt in &pts {
            let mut rays: BTreeSet<i64> = BTreeSet::new();
            for tile in &r.tiles {
                let nn = tile.poly.len();
                for k in 0..nn {
                    if (tile.poly.pts[k] - *pt).length() >= 1e-5 {
                        continue;
                    }
                    for &q in [&tile.poly.pts[(k + nn - 1) % nn], &tile.poly.pts[(k + 1) % nn]] {
                        let dir = q - *pt;
                        let th = dir.y.atan2(dir.x).rem_euclid(std::f64::consts::TAU);
                        rays.insert((th * 1e5).round() as i64);
                    }
                }
                for e in 0..nn {
                    let (a, b) = (tile.poly.pts[e], tile.poly.pts[(e + 1) % nn]);
                    let ab = b - a;
                    let l2 = ab.dot(ab);
                    let tt = (*pt - a).dot(ab) / l2;
                    if tt <= 1e-6 || tt >= 1.0 - 1e-6 {
                        continue;
                    }
                    if (*pt - a).cross(ab).abs() / l2.sqrt() < 1e-7 {
                        let th0 = ab.y.atan2(ab.x);
                        for add in [0.0, std::f64::consts::PI] {
                            let th = (th0 + add).rem_euclid(std::f64::consts::TAU);
                            rays.insert((th * 1e5).round() as i64);
                        }
                    }
                }
            }
            let mut mr: Vec<f64> = Vec::new();
            for k in &rays {
                let rv = *k as f64 / 1e5;
                if mr.is_empty() || rv - mr[mr.len() - 1] > 1e-4 {
                    mr.push(rv);
                }
            }
            if mr.len() > 1 && mr[mr.len() - 1] - mr[0] > std::f64::consts::TAU - 1e-4 {
                mr.pop();
            }
            let mut gaps: Vec<f64> = (0..mr.len())
                .map(|i| (mr[(i + 1) % mr.len()] - mr[i]).rem_euclid(std::f64::consts::TAU))
                .collect();
            gaps.retain(|g| *g > 1e-4);
            gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let fig: Vec<u32> = gaps.iter().map(|g| g.to_degrees().round() as u32).collect();
            assert_eq!(fig.iter().sum::<u32>(), 360, "vertex {pt:?} figure {fig:?}");
            match fig.len() {
                3 => {
                    let f = (fig[0], fig[1], fig[2]);
                    assert!(
                        f == (106, 106, 148) || f == (94, 133, 133),
                        "unexpected vertex figure {f:?} at {pt:?}"
                    );
                    *n3.entry(f).or_insert(0) += 1;
                }
                4 => {
                    let f = (fig[0], fig[1], fig[2], fig[3]);
                    assert_eq!(f, (59, 59, 94, 148), "unexpected vertex figure {f:?} at {pt:?}");
                    *n4.entry(f).or_insert(0) += 1;
                }
                _ => panic!("vertex {pt:?}: unexpected figure {fig:?}"),
            }
        }
        assert_eq!(pts.len(), 816, "total interior vertices");
        let want3: BTreeMap<(u32, u32, u32), usize> = [
            ((106u32, 106, 148), 271usize),
            ((94, 133, 133), 266),
        ]
        .into_iter()
        .collect();
        assert_eq!(n3, want3, "three-tile vertex stars");
        let want4: BTreeMap<(u32, u32, u32, u32), usize> =
            [((59u32, 59, 94, 148), 279usize)].into_iter().collect();
        assert_eq!(n4, want4, "four-tile vertex star");
    }

    /// Gluing census on the 8-cell torus (mod the shipped lattice): the
    /// 40 edges form 20 partner pairs with no broken edges and equal
    /// lengths; the glued side-pair histogram is a-a:4, b-b:2, b-e:4,
    /// c-d:4, c-e:4, d-d:2; the 40 corners form 12 vertex orbits (eight of
    /// size 3, four of size 4) whose stars are (58.9, 58.9, 94.3, 148.0)
    /// ×4, (94.3, 132.9, 132.9) ×4 and (106, 106, 148) ×4.
    #[test]
    fn pentagon_7_cell_gluing_census() {
        type PtKey = (i64, i64);
        type EdgeKey = (PtKey, PtKey);
        let t = spec();
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        assert!(det > 0.0, "the shipped basis must be positively oriented");
        let u_of = |p: Vec2| {
            v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
        };
        let ukey = |p: Vec2| {
            let u = u_of(p);
            (
                ((u.x.rem_euclid(1.0) * 1e6).round() as i64) % 1_000_000,
                ((u.y.rem_euclid(1.0) * 1e6).round() as i64) % 1_000_000,
            )
        };
        let cells: Vec<Vec<Vec2>> = t.cells.iter().map(|c| c.poly.pts.clone()).collect();
        // Edges: 40 of them, each glued to exactly one partner edge (its
        // reverse) — 20 orbits, no broken edges.
        let mut partners: BTreeMap<EdgeKey, Vec<(usize, usize)>> = BTreeMap::new();
        let mut lens: Vec<f64> = Vec::new();
        for (k, pts) in cells.iter().enumerate() {
            for e in 0..5 {
                let (a, b) = (pts[e], pts[(e + 1) % 5]);
                lens.push((a - b).length());
                partners.entry((ukey(a), ukey(b))).or_default().push((k, e));
            }
        }
        assert_eq!(lens.len(), 40, "eight pentagons, five sides each");
        let label = |k: usize, e: usize| -> u8 {
            // A stored CCW edge e of cell k carries prototile side index
            // (e + 1) mod 5 when the cell is direct and (5 − e) mod 5 when
            // mirror-stored ("abcde" order: a = the long side, b = A→B,
            // c = B→C, d = C→D, e = D→E).
            let l = if MIRROR[k] {
                (5 - e) % 5
            } else {
                (e + 1) % 5
            };
            l as u8
        };
        let mut done = [false; 40];
        let mut n_orbits = 0;
        let mut side_pairs: BTreeMap<(u8, u8), usize> = BTreeMap::new();
        for idx in 0..40 {
            if done[idx] {
                continue;
            }
            let (k, e) = (idx / 5, idx % 5);
            let pts = &cells[k];
            let (a, b) = (pts[e], pts[(e + 1) % 5]);
            let list = partners.get(&(ukey(b), ukey(a))).cloned().unwrap_or_default();
            let list: Vec<(usize, usize)> = list.into_iter().filter(|x| *x != (k, e)).collect();
            assert_eq!(
                list.len(),
                1,
                "edge ({k},{e}) must have exactly one glued partner, got {list:?}"
            );
            let (j, f) = list[0];
            let jidx = j * 5 + f;
            assert!(!done[jidx], "edge ({j},{f}) was already paired");
            done[idx] = true;
            done[jidx] = true;
            n_orbits += 1;
            // Partner endpoints: a' = b + λ, b' = a + λ for one lattice
            // vector λ (0 for interior edges, ±w for boundary edges).
            let (pj, qj) = (cells[j][f], cells[j][(f + 1) % 5]);
            let lam = pj - b;
            let ulam = u_of(lam);
            let frac = |x: f64| x.rem_euclid(1.0).min(1.0 - x.rem_euclid(1.0));
            assert!(
                frac(ulam.x) < 1e-6 && frac(ulam.y) < 1e-6,
                "glued edges ({k},{e}) and ({j},{f}): shift {ulam:?} not a lattice vector"
            );
            let jitter = (a + lam - qj).length();
            assert!(
                jitter < 1e-9,
                "glued edges ({k},{e}) and ({j},{f}) disagree mod lattice by {jitter}"
            );
            assert!(
                (lens[idx] - lens[jidx]).abs() < 1e-9,
                "glued edges ({k},{e}) and ({j},{f}) must have equal length"
            );
            let (la, lb) = (label(k, e), label(j, f));
            let pair = if la <= lb {
                (la, lb)
            } else {
                (lb, la)
            };
            *side_pairs.entry(pair).or_insert(0) += 1;
        }
        assert_eq!(n_orbits, 20, "twenty edge orbits on the torus");
        let want_pairs: BTreeMap<(u8, u8), usize> = [
            ((0u8, 0u8), 4usize), // a-a
            ((1, 1), 2), // b-b
            ((1, 4), 4), // b-e
            ((2, 3), 4), // c-d
            ((2, 4), 4), // c-e
            ((3, 3), 2), // d-d
        ]
        .into_iter()
        .collect();
        assert_eq!(side_pairs, want_pairs, "glued side-pair census (a,b,c,d,e indices)");
        // Vertex orbits mod lattice and their stars.
        let mut orbits: BTreeMap<PtKey, Vec<(usize, usize)>> = BTreeMap::new();
        for (k, pts) in cells.iter().enumerate() {
            for (v, &p) in pts.iter().enumerate() {
                orbits.entry(ukey(p)).or_default().push((k, v));
            }
        }
        assert_eq!(orbits.len(), 12, "twelve vertex orbits on the torus");
        let mut sizes: Vec<usize> = orbits.values().map(|o| o.len()).collect();
        sizes.sort_unstable();
        assert_eq!(
            sizes,
            vec![3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4],
            "orbit sizes (eight 3s and four 4s)"
        );
        let mut stars: Vec<Vec<i64>> = Vec::new();
        for corners in orbits.values() {
            let mut tens: Vec<i64> = corners
                .iter()
                .map(|&(k, v)| {
                    (angle_at(&t.cells[k].poly, v).to_degrees() * 10.0).round() as i64
                })
                .collect();
            tens.sort_unstable();
            stars.push(tens);
        }
        stars.sort();
        let want_stars: Vec<Vec<i64>> = [
            vec![589, 589, 943, 1480],
            vec![589, 589, 943, 1480],
            vec![589, 589, 943, 1480],
            vec![589, 589, 943, 1480],
            vec![943, 1329, 1329],
            vec![943, 1329, 1329],
            vec![943, 1329, 1329],
            vec![943, 1329, 1329],
            vec![1060, 1060, 1480],
            vec![1060, 1060, 1480],
            vec![1060, 1060, 1480],
            vec![1060, 1060, 1480],
        ]
        .to_vec();
        assert_eq!(stars, want_stars, "vertex-orbit stars (degree tenths)");
    }

    /// The wallpaper group is pgg: the half-turn about c0 = (1/4, 0)
    /// preserves the tiling exactly (interior-filtered set equality), the
    /// two perpendicular glides g1 = (M1, t1), g2 = (M2, t2) with axes
    /// along w1 and w2 preserve it (with every lattice-shifted variant,
    /// and so do their inverses), g1² = w1, g2² = −w2, and g2∘g1 is the
    /// half-turn about c0 itself.  Each glide shifts by exactly half a
    /// primitive translation along its axis, so no power of it is a pure
    /// reflection: neither the perpendicular-part candidate nor any whole
    /// lattice shift (24 negative probes in total, plus two quarter-turn
    /// probes) maps the prototile onto a tile.
    #[test]
    fn pentagon_7_pgg_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let c0 = two_fold_centre();
        assert_eq!(c0, v2(0.25, 0.0), "the 2-fold centre is (1/4, 0)");
        let ra = BBox::new(v2(c0.x - 12.0, c0.y - 10.0), v2(c0.x + 12.0, c0.y + 10.0));
        check_iso(&t, &p, (-1.0, 0.0, 0.0, -1.0), c0 * 2.0, ra, true, "half-turn about c0");
        let u1 = w1 * (1.0 / w1.length());
        let u2 = w2 * (1.0 / w2.length());
        let m1 = mirror(u1);
        let m2 = mirror(u2);
        // Glide translations, re-derived from stored-cell centroids:
        // g1 sends cell 0's centroid to cell 3's, g2's to cell 7's.
        let cent0 = cent(&t.cells[0].poly.pts);
        let t1 = cent(&t.cells[3].poly.pts) - apply_iso(m1, Vec2::ZERO, cent0);
        let t2 = cent(&t.cells[7].poly.pts) - apply_iso(m2, Vec2::ZERO, cent0);
        assert!(
            (t1 - v2(0.1954787892179718, 1.2530607552235797)).length() < 1e-12,
            "t1 {t1:?}"
        );
        assert!(
            (t2 - v2(1.7467969065338207, -0.2321005970981298)).length() < 1e-12,
            "t2 {t2:?}"
        );
        // Exact generator identities: g² = (I + M)·t.
        let gs1 = v2((m1.0 + 1.0) * t1.x + m1.1 * t1.y, m1.2 * t1.x + (m1.3 + 1.0) * t1.y);
        assert!((gs1 - w1).length() < 1e-9, "g1² must equal w1");
        let gs2 = v2((m2.0 + 1.0) * t2.x + m2.1 * t2.y, m2.2 * t2.x + (m2.3 + 1.0) * t2.y);
        assert!((gs2 + w2).length() < 1e-9, "g2² must equal −w2");
        // Each glide shifts by exactly half a primitive translation along
        // its axis (g1 by +w1/2, g2 by −w2/2) — no power is a reflection.
        assert!((2.0 * t1.dot(u1) / w1.length() - 1.0).abs() < 1e-6, "g1 must shift +w1/2");
        assert!((2.0 * t2.dot(u2) / w2.length() + 1.0).abs() < 1e-6, "g2 must shift −w2/2");
        // g2∘g1 is the half-turn centred at c0 itself.
        let comp = v2(m2.0 * t1.x + m2.1 * t1.y + t2.x, m2.2 * t1.x + m2.3 * t1.y + t2.y);
        assert!((comp * 0.5 - c0).length() < 1e-9, "centre(g2∘g1) must be c0");
        // The glides preserve the tiling (the image window is tilted, so
        // check containment of the images among the tiles generated
        // there) — and so do the inverses.
        check_iso(&t, &p, m1, t1, ra, false, "glide g1 (axis ∥ w1)");
        check_iso(
            &t,
            &p,
            m1,
            v2(-(m1.0 * t1.x + m1.1 * t1.y), -(m1.2 * t1.x + m1.3 * t1.y)),
            ra,
            false,
            "glide g1⁻¹",
        );
        check_iso(&t, &p, m2, t2, ra, false, "glide g2 (axis ∥ w2)");
        check_iso(
            &t,
            &p,
            m2,
            v2(-(m2.0 * t2.x + m2.1 * t2.y), -(m2.2 * t2.x + m2.3 * t2.y)),
            ra,
            false,
            "glide g2⁻¹",
        );
        // Big patch for tile-image membership.
        let big = BBox::new(v2(-14.0, -12.0), v2(20.0, 16.0));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        assert_eq!(big_tiles.tiles.len(), 1922, "big patch tile count");
        let set: HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        assert_eq!(set.len(), 1922, "big patch tiles must be unique");
        // Every cell maps onto a tile under the three generators.
        type Iso = ((f64, f64, f64, f64), Vec2);
        let maps: [(&str, Iso); 3] = [
            ("ht", ((-1.0, 0.0, 0.0, -1.0), c0 * 2.0)),
            ("g1", (m1, t1)),
            ("g2", (m2, t2)),
        ];
        for k in 0..8 {
            for (nm, (mm, tt)) in &maps {
                let img: Vec<Vec2> =
                    t.cells[k].poly.pts.iter().map(|q| apply_iso(*mm, *tt, *q)).collect();
                assert!(set.contains(&vkey(&img)), "{nm}(cell {k}) must be a tile");
            }
        }
        let prot = t.cells[0].poly.pts.clone();
        for i in -2..=2 {
            for &w in [&w1, &w2] {
                let img: Vec<Vec2> = prot.iter().map(|q| *q + w * i as f64).collect();
                assert!(set.contains(&vkey(&img)), "cell0 + {i}·w must be a tile");
            }
        }
        // All 36 half-turns in the four classes c0 + Λ/2 map cell0 onto a
        // tile.
        for h in [Vec2::ZERO, w1 * 0.5, w2 * 0.5, (w1 + w2) * 0.5] {
            for i in -1..=1 {
                for j in -1..=1 {
                    let ctr = c0 + h + w1 * i as f64 + w2 * j as f64;
                    let img: Vec<Vec2> = prot
                        .iter()
                        .map(|q| apply_iso((-1.0, 0.0, 0.0, -1.0), ctr * 2.0, *q))
                        .collect();
                    assert!(
                        set.contains(&vkey(&img)),
                        "half-turn about c0 + {h:?} + {i}·w1 + {j}·w2 must map cell0 onto a tile"
                    );
                }
            }
        }
        // Lattice-shifted glides still map cell0 onto a tile.
        for (i, j) in [(0.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (1.0, 0.0)] {
            let sh = w1 * i + w2 * j;
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m1, t1 + sh, *q)).collect();
            assert!(
                set.contains(&vkey(&img)),
                "(g1 + {i}·w1 + {j}·w2) must map cell0 onto a tile"
            );
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m2, t2 + sh, *q)).collect();
            assert!(
                set.contains(&vkey(&img)),
                "(g2 + {i}·w1 + {j}·w2) must map cell0 onto a tile"
            );
        }
        // No pure reflection: the mirror-class candidates carrying only the
        // perpendicular glide part (a reflection would need zero along-axis
        // shift) and every whole-lattice-shift candidate must fail to map
        // the prototile onto a tile.
        let p1 = t1 - u1 * t1.dot(u1);
        let p2 = t2 - u2 * t2.dot(u2);
        for (m, tr, nm) in [(m1, p1, "(M1, t1⊥)"), (m2, p2, "(M2, t2⊥)")] {
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, tr, *q)).collect();
            assert!(
                !set.contains(&vkey(&img)),
                "{nm} must not be a symmetry (no mirrors in pgg)"
            );
        }
        for i in -2..=2 {
            for (m, w, mn, wn) in [
                (m1, w1, "M1", "w1"),
                (m1, w2, "M1", "w2"),
                (m2, w1, "M2", "w1"),
                (m2, w2, "M2", "w2"),
            ] {
                let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, w * i as f64, *q)).collect();
                assert!(
                    !set.contains(&vkey(&img)),
                    "({mn}, {i}·{wn}) must not be a symmetry (no mirrors in pgg)"
                );
            }
        }
        // No quarter-turn either.
        for (nm, tr) in [
            ("quarter-turn about the origin", Vec2::ZERO),
            ("quarter-turn at (1/4, −1/4)", v2(0.25, -0.25)),
        ] {
            let img: Vec<Vec2> = prot
                .iter()
                .map(|q| apply_iso((0.0, -1.0, 1.0, 0.0), tr, *q))
                .collect();
            assert!(!set.contains(&vkey(&img)), "{nm} must not be a symmetry");
        }
    }

    /// 2-isohedral: seeds 0/1 are the two full-group isohedral classes —
    /// the half-turn and both glide classes permute the eight cells
    /// mod-lattice by the closed permutations, the group they generate has
    /// exactly two orbits on the cells ({0,3,4,7} / {1,2,5,6}, matching
    /// the seeds), both seeds appear in every patch, and every generated
    /// tile keeps the prototile side multiset {1/2, 1/2, 1/2, 1/2, a}.
    #[test]
    fn pentagon_7_isohedral_classes() {
        let t = spec();
        let mut counts = [0usize; 2];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(counts, [4, 4], "four tiles per isohedral class");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        let u_of = |p: Vec2| {
            v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
        };
        let mod1 = |x: f64| (x + 0.5).rem_euclid(1.0) - 0.5;
        let c0 = two_fold_centre();
        let u1 = w1 * (1.0 / w1.length());
        let u2 = w2 * (1.0 / w2.length());
        let cent0 = cent(&t.cells[0].poly.pts);
        let t1 = cent(&t.cells[3].poly.pts) - apply_iso(mirror(u1), Vec2::ZERO, cent0);
        let t2 = cent(&t.cells[7].poly.pts) - apply_iso(mirror(u2), Vec2::ZERO, cent0);
        let cents: Vec<Vec2> = t.cells.iter().map(|c| cent(&c.poly.pts)).collect();
        // A symmetry (M, t) permutes the cells: cell k's centroid maps to
        // cell j's centroid mod lattice, for exactly one j.
        let perm_of = |m: (f64, f64, f64, f64), tr: Vec2| -> Vec<usize> {
            let mut perm: Vec<usize> = Vec::new();
            for k in 0..8 {
                let img = apply_iso(m, tr, cents[k]);
                let mut hits: Vec<usize> = Vec::new();
                for (j, &cj) in cents.iter().enumerate() {
                    let du = u_of(img - cj);
                    if mod1(du.x).abs() < 1e-6 && mod1(du.y).abs() < 1e-6 {
                        hits.push(j);
                    }
                }
                assert_eq!(hits.len(), 1, "cell {k} must have exactly one image, got {hits:?}");
                perm.push(hits[0]);
            }
            perm
        };
        let perms = [
            perm_of((-1.0, 0.0, 0.0, -1.0), c0 * 2.0),
            perm_of(mirror(u1), t1),
            perm_of(mirror(u2), t2),
        ];
        assert_eq!(perms[0], vec![4, 5, 6, 7, 0, 1, 2, 3], "half-turn perm");
        assert_eq!(perms[1], vec![3, 2, 1, 0, 7, 6, 5, 4], "glide g1 perm");
        assert_eq!(perms[2], vec![7, 6, 5, 4, 3, 2, 1, 0], "glide g2 perm");
        // Two full-group orbits on the cells, exactly the seed classes.
        let mut seen = [false; 8];
        let mut orbits: Vec<Vec<usize>> = Vec::new();
        for st in 0..8 {
            if seen[st] {
                continue;
            }
            let mut orb = vec![st];
            seen[st] = true;
            let mut head = 0;
            while head < orb.len() {
                let x = orb[head];
                head += 1;
                for pm in &perms {
                    let y = pm[x];
                    if !seen[y] {
                        seen[y] = true;
                        orb.push(y);
                    }
                }
            }
            orb.sort_unstable();
            orbits.push(orb);
        }
        assert_eq!(
            orbits,
            vec![vec![0, 3, 4, 7], vec![1, 2, 5, 6]],
            "two full-group orbits"
        );
        for orb in &orbits {
            let sd = t.cells[orb[0]].seed;
            assert!(
                orb.iter().all(|&k| t.cells[k].seed == sd),
                "orbit mixes isohedral seeds"
            );
        }
        assert_ne!(t.cells[orbits[0][0]].seed, t.cells[orbits[1][0]].seed, "orbits differ");
        // Generated patches: both classes present, side multiset kept.
        let params = ParamSet::default_for(t.params());
        let rp = t.generate(
            &params,
            &Query {
                region: BBox::new(v2(-6.0, -6.0), v2(10.0, 10.0)),
            },
        );
        assert!(rp.message.is_none(), "{:?}", rp.message);
        let a4 = side_of(&t.cells[0].poly, 4);
        let mut seen_s = [false; 2];
        for tile in &rp.tiles {
            let sd = tile.seed as usize;
            assert!(sd < 2, "seeds must stay in {{0, 1}}");
            seen_s[sd] = true;
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length())
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [0.5, 0.5, 0.5, 0.5, a4];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!(
                    (got - w).abs() < 1e-12,
                    "seed {sd} side {got} ∉ {{1/2 × 4, a}}"
                );
            }
        }
        assert!(seen_s[0] && seen_s[1], "both isohedral classes must appear");
    }

    /// The eight cells realise both chiralities of the (chiral) type-7
    /// prototile: cell 0 matches the prototile's cyclic (side, angle)
    /// sequence forward but not reversed, every cell matches exactly one
    /// of the two, and the counts are four forward and four reversed —
    /// the mirror half of the primitive unit.
    #[test]
    fn pentagon_7_enantiomer_mix() {
        let t = spec();
        let proto = &t.cells[0].poly;
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        let match_fwd = |cs: &[f64], ca: &[f64]| {
            (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            })
        };
        let match_rev = |cs: &[f64], ca: &[f64]| {
            (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(rot + 9 - i) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(rot + 5 - i) % 5]).abs() < 1e-9
                })
            })
        };
        // The prototile itself: forward match only (the family is chiral).
        assert!(match_fwd(&ps, &pa), "prototile must match forward");
        assert!(!match_rev(&ps, &pa), "prototile must be chiral (no reversed match)");
        let mut n_fwd = 0;
        let mut n_rev = 0;
        for (k, cell) in t.cells.iter().enumerate() {
            let cs: Vec<f64> = (0..5).map(|i| side_of(&cell.poly, i)).collect();
            let ca: Vec<f64> = (0..5).map(|i| angle_at(&cell.poly, i)).collect();
            let f = match_fwd(&cs, &ca);
            let r = match_rev(&cs, &ca);
            assert!(f != r, "cell {k} must match exactly one chirality");
            if f {
                n_fwd += 1;
            } else {
                n_rev += 1;
            }
        }
        assert_eq!((n_fwd, n_rev), (4, 4), "four direct and four mirror tiles");
    }
}

#[cfg(test)]
mod pentagon8_tests {
    use super::pentagon8::{s3, tile_area};
    use crate::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon8::spec;
    use std::collections::{BTreeMap, HashSet};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// The mirror-stored cells (0-based {1, 2, 4, 7} = T2, T3, T5, T8):
    /// the mirror half of the primitive unit, stored CCW with their
    /// vertex order reversed.
    const MIRROR: [bool; 8] = [false, true, true, false, true, false, false, true];

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        Vec2::new(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Vertex-set key on a 1e-6 grid (every coordinate lies on the Z[√3]
    /// grid, so distinct tiling vertices are ≥ 1 apart, and two
    /// computations of a shared joint agree to ~1e-15).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Single-vertex key on the same 1e-6 grid.
    fn pkey(p: Vec2) -> (i64, i64) {
        ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64)
    }

    /// Verify that the isometry g = (M, t) preserves the tiling: the
    /// image of every tile intersecting `ra` must be a tile intersecting
    /// `rb` (the axis-aligned box of g(ra)).  With `exact = true` the
    /// image set must equal the `rb` tile set (the case g(ra) = rb).
    fn check_symmetry(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            Vec2::new(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            Vec2::new(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ra_tiles = t.generate(params, &Query { region: ra });
        let rb_tiles = t.generate(params, &Query { region: rb });
        assert!(ra_tiles.message.is_none(), "{label}: {:?}", ra_tiles.message);
        assert!(rb_tiles.message.is_none(), "{label}: {:?}", rb_tiles.message);
        assert!(!ra_tiles.tiles.is_empty(), "{label}: no tiles");
        let set_b: std::collections::HashSet<Vec<(i64, i64)>> =
            rb_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let set_a: std::collections::HashSet<Vec<(i64, i64)>> = ra_tiles
            .tiles
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(
            set_a.len(),
            ra_tiles.tiles.len(),
            "{label}: isometry must be injective on the patch"
        );
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    /// Directed-edge key: keys of its two endpoints on the 1e-6 grid.
    type EdgeKey = ((i64, i64), (i64, i64));

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    /// Centroid of a stored cell (mean of its five vertices).
    fn cent(pts: &[Vec2]) -> Vec2 {
        let s = pts.iter().fold(Vec2::ZERO, |acc, &p| acc + p);
        s / 5.0
    }

    #[test]
    fn pentagon_8_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        let s = s3();
        check_coverage(
            &t,
            "pentagon-8",
            &p,
            BBox::new(v2(-25.0, -20.0 * s), v2(25.0, 20.0 * s)),
        );
    }

    #[test]
    fn pentagon_8_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-8(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(7.0, 7.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_8_deterministic() {
        check_det(&spec(), "pentagon-8", BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)));
    }

    /// Unit structure of the primitive cell: 8 tiles of the exact Q(√3)
    /// prototile (B = C = 120°, tan A = 3√3, cos D = 1/7, tan E = −√3/2,
    /// A = E − 60°; a = 6√7, b = c = d = e = 14) on the rectangular
    /// lattice w1 = (36, 24√3) ⊥ w2 = (−36, 18√3), with |det| = 8 × tile
    /// area = 1512·√3, |w1|² = 3024 and |w2|² = 2268.  Every cell is a
    /// congruent copy of the prototile (cyclic (side, angle) fingerprint
    /// forward or reversed), CCW, no two cells coincide, and the family
    /// condition identities hold to float precision.
    #[test]
    fn pentagon_8_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-8");
        assert_eq!(t.cells.len(), 8, "the primitive unit holds eight tiles");
        let s = s3();
        assert_eq!(t.lattice[0], v2(36.0, 24.0 * s), "w1 = (36, 24√3)");
        assert_eq!(t.lattice[1], v2(-36.0, 18.0 * s), "w2 = (−36, 18√3)");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        assert!(w1.dot(w2).abs() < 1e-12, "lattice must be rectangular (w1 ⊥ w2)");
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        assert!(
            (det - 8.0 * tile_area()).abs() < 1e-9,
            "det {det} ≠ 8 × tile area = 1512√3"
        );
        assert!(
            ((w1.x * w1.x + w1.y * w1.y) - 3024.0).abs() < 1e-9,
            "|w1|² = 1296 + 1728 = 3024"
        );
        assert!(
            ((w2.x * w2.x + w2.y * w2.y) - 2268.0).abs() < 1e-9,
            "|w2|² = 1296 + 972 = 2268"
        );
        // Seeds follow the two full-group isohedral orbits
        // {T1,T4,T5,T8} (0) and {T2,T3,T6,T7} (1).
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 1, 1, 0, 0, 1, 1, 0], "isohedral seed pattern");
        // The shipped prototile is T1, corner A at the origin:
        // A = (0,0), B = (14,0), C = (21,7√3), D = (14,14√3), E = (3,9√3).
        let proto = &t.cells[0].poly;
        for (i, want) in [(0.0, 0.0), (14.0, 0.0), (21.0, 7.0 * s), (14.0, 14.0 * s), (3.0, 9.0 * s)]
            .iter()
            .enumerate()
        {
            assert_eq!(proto.pts[i], v2(want.0, want.1), "prototile corner {i}");
        }
        // Exact corner angle cosines in vertex order A, B, C, D, E
        // (B = C = 120°).
        let cos = |i: usize| angle_at(proto, i).cos();
        assert!((cos(0) - 7.0f64.sqrt() / 14.0).abs() < 1e-12, "cos A = √7/14");
        assert!((cos(1) + 0.5).abs() < 1e-12, "cos B = −1/2");
        assert!((cos(2) + 0.5).abs() < 1e-12, "cos C = −1/2");
        assert!((cos(3) - 1.0 / 7.0).abs() < 1e-12, "cos D = 1/7");
        assert!((cos(4) + 2.0 * 7.0f64.sqrt() / 7.0).abs() < 1e-12, "cos E = −2√7/7");
        // Family condition identities, from the stored angles.
        let a = angle_at(proto, 0);
        let e = angle_at(proto, 4);
        assert!((a.tan() - 3.0 * s).abs() < 1e-9, "tan A = 3√3");
        assert!((e.tan() + s / 2.0).abs() < 1e-9, "tan E = −√3/2");
        assert!(
            (a + 60.0f64.to_radians() - e).abs() < 1e-9,
            "A = E − 60°"
        );
        assert!(
            (2.0 * angle_at(proto, 1) + angle_at(proto, 2) - 2.0 * std::f64::consts::PI).abs()
                < 1e-9,
            "2B + C = 360°"
        );
        assert!(
            (angle_at(proto, 3) + 2.0 * e - 2.0 * std::f64::consts::PI).abs() < 1e-9,
            "D + 2E = 360°"
        );
        let a_side = side_of(proto, 4);
        assert!(
            (a_side / 14.0 - 3.0 / 7.0f64.sqrt()).abs() < 1e-12,
            "a/b = 6√7/14 = 3/√7"
        );
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        let mut seen: HashSet<Vec<(i64, i64)>> = HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            assert!((cell.poly.area() - tile_area()).abs() < 1e-12, "cell {k} area ≠ 189√3");
            assert_eq!(cell.poly.len(), 5, "cell {k} vertex count");
            // Side multiset {14, 14, 14, 14, 6√7}.
            let mut sides: Vec<f64> = (0..5).map(|i| side_of(&cell.poly, i)).collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [14.0, 14.0, 14.0, 14.0, 6.0 * 7.0f64.sqrt()];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!(
                    (got - w).abs() < 1e-12,
                    "cell {k} side {got} ∉ {{14 × 4, 6√7}}"
                );
            }
            // Cyclic (side, angle) fingerprint against the prototile,
            // forward or reversed (the cells realise both chiralities).
            let cs = (0..5).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..5).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            let rev = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(rot + 9 - i) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(rot + 5 - i) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd || rev, "cell {k} is not a congruent type-8 prototile");
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
        }
    }

    /// Window census (default parameters): culling is the tile AABB
    /// intersecting the region (+1e-9); every shipped coordinate lies on
    /// the Z[√3] grid, and the half-integer window bounds keep every
    /// culling decision unambiguous, so the counts are exact and match
    /// the independent Python emulation of the lattice.rs culling.
    #[test]
    fn pentagon_8_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::Pgg);
        assert_eq!(t.lattice_kind, LatticeKind::Rectangular);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the T1 cell (seed 0)");
        let p = ParamSet::default_for(t.params());
        let s = s3();
        for (region, want) in [
            (BBox::new(v2(-60.5, -45.5 * s), v2(60.5, 45.5 * s)), 79),
            (BBox::new(v2(-40.5, -30.5 * s), v2(40.5, 30.5 * s)), 42),
            (BBox::new(v2(-25.5, -20.5 * s), v2(25.5, 20.5 * s)), 20),
            (BBox::new(v2(-4.5, -3.5 * s), v2(10.5, 10.5 * s)), 5),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }

    /// Edge-to-edge at the pentagon level over the 79-tile patch: no
    /// directed side is shared by two tiles (395 directed edges, 0
    /// duplicates), every side whose midpoint lies strictly inside the
    /// window (290 of them) is matched by exactly one full side of one
    /// neighbour (its reverse), and no vertex falls in the interior of a
    /// non-incident side (the closest genuine near-miss is 13.7 units
    /// away).
    #[test]
    fn pentagon_8_edge_to_edge_gluing() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let s = s3();
        let big = BBox::new(v2(-60.5, -45.5 * s), v2(60.5, 45.5 * s));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 79, "patch size");
        let mut dirs: std::collections::HashMap<EdgeKey, usize> =
            std::collections::HashMap::new();
        for tile in &r.tiles {
            let n = tile.poly.len();
            for i in 0..n {
                let k = (pkey(tile.poly.pts[i]), pkey(tile.poly.pts[(i + 1) % n]));
                *dirs.entry(k).or_insert(0) += 1;
            }
        }
        assert_eq!(dirs.len(), 395, "395 directed sides, all unique");
        assert!(
            dirs.values().all(|&c| c == 1),
            "a directed side may belong to only one tile"
        );
        let mut interior = 0;
        for tile in &r.tiles {
            let n = tile.poly.len();
            for i in 0..n {
                let (a, b) = (tile.poly.pts[i], tile.poly.pts[(i + 1) % n]);
                let mid = (a + b) * 0.5;
                let d = (mid.x - big.min.x)
                    .min(big.max.x - mid.x)
                    .min(mid.y - big.min.y)
                    .min(big.max.y - mid.y);
                if d > 1e-6 {
                    interior += 1;
                    assert_eq!(
                        dirs.get(&(pkey(b), pkey(a))),
                        Some(&1),
                        "interior side {a} -> {b} lacks its full-edge partner"
                    );
                }
            }
        }
        assert_eq!(
            interior, 290,
            "interior directed sides (each paired with its reverse)"
        );
        // No T-junction: no vertex in the interior of a non-incident edge.
        let verts: HashSet<(i64, i64)> =
            r.tiles.iter().flat_map(|tile| tile.poly.pts.iter().map(|&p| pkey(p))).collect();
        assert_eq!(verts.len(), 148, "distinct vertices in the patch");
        for tile in &r.tiles {
            let n = tile.poly.len();
            for i in 0..n {
                let (a, b) = (tile.poly.pts[i], tile.poly.pts[(i + 1) % n]);
                let (dx, dy) = (b.x - a.x, b.y - a.y);
                let l2 = dx * dx + dy * dy;
                for vk in verts.iter().filter(|k| **k != pkey(a) && **k != pkey(b)) {
                    let (px, py) = (vk.0 as f64 / 1e6, vk.1 as f64 / 1e6);
                    let tt = (((px - a.x) * dx + (py - a.y) * dy) / l2).clamp(0.0, 1.0);
                    if tt == 0.0 || tt == 1.0 {
                        continue;
                    }
                    let q = a + (b - a) * tt;
                    assert!(
                        (px - q.x).hypot(py - q.y) > 1e-7,
                        "vertex {vk:?} lies in the interior of side {a} -> {b}"
                    );
                }
            }
        }
    }

    /// Vertex census over the 79-tile patch: every window-interior vertex
    /// (91 of the 148 distinct vertices) is a complete star of incident
    /// tile angles summing to exactly 360°, and the stars are exactly the
    /// three closed types {A,A,C,D} = 2·79.11° + 120° + 81.79° (B = C
    /// merges in the letters: "AABD", 30 of them), {B,B,C} = 3·120°
    /// ("BBB", 36) and {D,E,E} = 81.79° + 2·139.11° ("DEE", 25).
    #[test]
    fn pentagon_8_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let s = s3();
        let big = BBox::new(v2(-60.5, -45.5 * s), v2(60.5, 45.5 * s));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        let verts: HashSet<(i64, i64)> =
            r.tiles.iter().flat_map(|tile| tile.poly.pts.iter().map(|&pt| pkey(pt))).collect();
        // Corner cosines: A = √7/14, B = C = −1/2, D = 1/7, E = −2√7/7.
        let bins = [7.0f64.sqrt() / 14.0, -0.5, 1.0 / 7.0, -2.0 * 7.0f64.sqrt() / 7.0];
        let mut stars: BTreeMap<String, usize> = BTreeMap::new();
        let mut checked = 0;
        for vk in &verts {
            let (px, py) = (vk.0 as f64 / 1e6, vk.1 as f64 / 1e6);
            let d = (px - big.min.x)
                .min(big.max.x - px)
                .min(py - big.min.y)
                .min(big.max.y - py);
            if d < 1e-6 {
                continue; // boundary: incident tiles may be culled
            }
            // Any tile having this corner has its AABB containing the
            // strictly interior point, hence was generated: the star is
            // complete.
            let mut letters = String::new();
            let mut sum = 0.0;
            for tile in &r.tiles {
                let n = tile.poly.len();
                for i in 0..n {
                    if pkey(tile.poly.pts[i]) != *vk {
                        continue;
                    }
                    let ang = angle_at(&tile.poly, i);
                    sum += ang;
                    let c = ang.cos();
                    let j = (0..4)
                        .min_by_key(|&j| ((c - bins[j]).abs() * 1e12) as i64)
                        .unwrap();
                    assert!((c - bins[j]).abs() < 1e-9, "stray corner cosine {c}");
                    letters.push(['A', 'B', 'D', 'E'][j]);
                }
            }
            assert!(
                (sum - std::f64::consts::TAU).abs() < 1e-9,
                "vertex {vk:?} angles sum to {sum} ≠ 2π"
            );
            let mut cs: Vec<char> = letters.chars().collect();
            cs.sort_unstable();
            let star: String = cs.into_iter().collect();
            assert!(
                star == "AABD" || star == "BBB" || star == "DEE",
                "unexpected vertex star {star} at {vk:?}"
            );
            *stars.entry(star).or_insert(0) += 1;
            checked += 1;
        }
        assert_eq!(checked, 91, "interior vertex census");
        assert_eq!(stars.get("AABD").copied().unwrap_or(0), 30, "{{A,A,C,D}} stars");
        assert_eq!(stars.get("BBB").copied().unwrap_or(0), 36, "{{B,B,C}} stars");
        assert_eq!(stars.get("DEE").copied().unwrap_or(0), 25, "{{D,E,E}} stars");
    }

    /// Gluing census on the 8-cell torus (mod the shipped lattice): the
    /// 40 edges form 20 partner pairs, every pair reverse-glued, with no
    /// broken edges and equal lengths; the glued side-pair histogram is
    /// a-a:4, b-c:4, b-d:4, c-c:2, d-e:4, e-e:2; the 40 corners form 12
    /// vertex orbits (eight of size 3, four of size 4) whose stars are
    /// (79.1, 79.1, 120.0, 81.8) ×4, (120.0, 120.0, 120.0) ×4 and
    /// (81.8, 139.1, 139.1) ×4.
    #[test]
    fn pentagon_8_cell_gluing_census() {
        type PtKey = (i64, i64);
        type EdgeKey2 = (PtKey, PtKey);
        let t = spec();
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        assert!(det > 0.0, "the shipped basis must be positively oriented");
        let u_of = |p: Vec2| {
            v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
        };
        let ukey = |p: Vec2| {
            let u = u_of(p);
            (
                ((u.x.rem_euclid(1.0) * 1e6).round() as i64) % 1_000_000,
                ((u.y.rem_euclid(1.0) * 1e6).round() as i64) % 1_000_000,
            )
        };
        let cells: Vec<Vec<Vec2>> = t.cells.iter().map(|c| c.poly.pts.clone()).collect();
        // Edges: 40 of them, each glued to exactly one partner edge (its
        // reverse) — 20 orbits, no broken edges.
        let mut partners: BTreeMap<EdgeKey2, Vec<(usize, usize)>> = BTreeMap::new();
        let mut lens: Vec<f64> = Vec::new();
        for (k, pts) in cells.iter().enumerate() {
            for e in 0..5 {
                let (a, b) = (pts[e], pts[(e + 1) % 5]);
                lens.push((a - b).length());
                partners.entry((ukey(a), ukey(b))).or_default().push((k, e));
            }
        }
        assert_eq!(lens.len(), 40, "eight pentagons, five sides each");
        let label = |k: usize, e: usize| -> u8 {
            // A stored CCW edge e of cell k carries prototile side index
            // (e + 1) mod 5 when the cell is direct and (4 − e) mod 5
            // when mirror-stored (side s joins vertices s−1 and s: a =
            // E→A, b = A→B, c = B→C, d = C→D, e = D→E).
            let l = if MIRROR[k] { (4 - e) % 5 } else { (e + 1) % 5 };
            l as u8
        };
        let mut done = [false; 40];
        let mut n_orbits = 0;
        let mut side_pairs: BTreeMap<(u8, u8), usize> = BTreeMap::new();
        for idx in 0..40 {
            if done[idx] {
                continue;
            }
            let (k, e) = (idx / 5, idx % 5);
            let pts = &cells[k];
            let (a, b) = (pts[e], pts[(e + 1) % 5]);
            // No same-direction partner: every torus pair is reverse-
            // glued (edge-to-edge).
            let same_dir: Vec<(usize, usize)> = partners
                .get(&(ukey(a), ukey(b)))
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter(|x| *x != (k, e))
                .collect();
            assert!(
                same_dir.is_empty(),
                "edge ({k},{e}) has a same-direction partner {same_dir:?}"
            );
            let list: Vec<(usize, usize)> = partners
                .get(&(ukey(b), ukey(a)))
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter(|x| *x != (k, e))
                .collect();
            assert_eq!(
                list.len(),
                1,
                "edge ({k},{e}) must have exactly one glued partner, got {list:?}"
            );
            let (j, f) = list[0];
            let jidx = j * 5 + f;
            assert!(!done[jidx], "edge ({j},{f}) was already paired");
            done[idx] = true;
            done[jidx] = true;
            n_orbits += 1;
            // Partner endpoints: a' = b + λ, b' = a + λ for one lattice
            // vector λ (0 for interior edges, ±w for boundary edges).
            let (pj, qj) = (cells[j][f], cells[j][(f + 1) % 5]);
            let lam = pj - b;
            let ulam = u_of(lam);
            let frac = |x: f64| x.rem_euclid(1.0).min(1.0 - x.rem_euclid(1.0));
            assert!(
                frac(ulam.x) < 1e-6 && frac(ulam.y) < 1e-6,
                "glued edges ({k},{e}) and ({j},{f}): shift {ulam:?} not a lattice vector"
            );
            let jitter = (a + lam - qj).length();
            assert!(
                jitter < 1e-9,
                "glued edges ({k},{e}) and ({j},{f}) disagree mod lattice by {jitter}"
            );
            assert!(
                (lens[idx] - lens[jidx]).abs() < 1e-9,
                "glued edges ({k},{e}) and ({j},{f}) must have equal length"
            );
            let (la, lb) = (label(k, e), label(j, f));
            let pair = if la <= lb { (la, lb) } else { (lb, la) };
            *side_pairs.entry(pair).or_insert(0) += 1;
        }
        assert_eq!(n_orbits, 20, "twenty edge orbits on the torus");
        let want_pairs: BTreeMap<(u8, u8), usize> = [
            ((0u8, 0u8), 4usize), // a-a
            ((1, 2), 4), // b-c
            ((1, 3), 4), // b-d
            ((2, 2), 2), // c-c
            ((3, 4), 4), // d-e
            ((4, 4), 2), // e-e
        ]
        .into_iter()
        .collect();
        assert_eq!(side_pairs, want_pairs, "glued side-pair census (a,b,c,d,e indices)");
        // Vertex orbits mod lattice and their stars.
        let mut orbits: BTreeMap<PtKey, Vec<(usize, usize)>> = BTreeMap::new();
        for (k, pts) in cells.iter().enumerate() {
            for (v, &p) in pts.iter().enumerate() {
                orbits.entry(ukey(p)).or_default().push((k, v));
            }
        }
        assert_eq!(orbits.len(), 12, "twelve vertex orbits on the torus");
        let mut sizes: Vec<usize> = orbits.values().map(|o| o.len()).collect();
        sizes.sort_unstable();
        assert_eq!(
            sizes,
            vec![3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4],
            "orbit sizes (eight 3s and four 4s)"
        );
        let mut stars: Vec<Vec<i64>> = Vec::new();
        for corners in orbits.values() {
            let mut tens: Vec<i64> = corners
                .iter()
                .map(|&(k, v)| {
                    (angle_at(&t.cells[k].poly, v).to_degrees() * 10.0).round() as i64
                })
                .collect();
            tens.sort_unstable();
            stars.push(tens);
        }
        stars.sort();
        let want_stars: Vec<Vec<i64>> = [
            vec![791, 791, 818, 1200],
            vec![791, 791, 818, 1200],
            vec![791, 791, 818, 1200],
            vec![791, 791, 818, 1200],
            vec![818, 1391, 1391],
            vec![818, 1391, 1391],
            vec![818, 1391, 1391],
            vec![818, 1391, 1391],
            vec![1200, 1200, 1200],
            vec![1200, 1200, 1200],
            vec![1200, 1200, 1200],
            vec![1200, 1200, 1200],
        ]
        .to_vec();
        assert_eq!(stars, want_stars, "vertex-orbit stars (degree tenths)");
    }

    /// The wallpaper group is pgg: the half-turn about the 2-fold centre
    /// c0 = (−1/2, 25√3/2) preserves the tiling exactly, and the two
    /// perpendicular glide reflections g1 = (M_u, t1 = (14, 14√3)) and
    /// g2 = (M_v, t2 = (57, 17√3)), with M_u the reflection in the w1
    /// direction, M_v = −M_u the reflection in the w2 direction, and axes
    /// along the lattice directions, preserve it (with every lattice-
    /// shifted variant, and so do the inverses), g1² = w1, g2² = −w2.
    /// Each glide shifts by exactly half a primitive translation along
    /// its axis (g1 by +w1/2, g2 by −w2/2), so no power of it is a pure
    /// reflection: none of the 20 candidates (M, i·w) with whole lattice
    /// shifts, the two perpendicular-part candidates, or the two
    /// quarter-turn probes maps the prototile onto a tile.  All
    /// generators are exact Q(√3).
    #[test]
    fn pentagon_8_pgg_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let s = s3();
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let c0 = v2(-0.5, 12.5 * s);
        // A window symmetric about c0: the half-turn must give exact
        // set equality of the tile patch.
        let ra = BBox::new(
            v2(c0.x - 30.0, c0.y - 24.0),
            v2(c0.x + 30.0, c0.y + 24.0),
        );
        check_symmetry(&t, &p, (-1.0, 0.0, 0.0, -1.0), c0 * 2.0, ra, true, "half-turn about c0");
        // M_u = reflection in the w1 direction (rows (−1/7, 4√3/7),
        // (4√3/7, 1/7)); M_v = −M_u = reflection in the w2 direction.
        let mu = (-1.0 / 7.0, 4.0 * s / 7.0, 4.0 * s / 7.0, 1.0 / 7.0);
        let mv = (1.0 / 7.0, -4.0 * s / 7.0, -4.0 * s / 7.0, -1.0 / 7.0);
        let t1 = v2(14.0, 14.0 * s);
        let t2 = v2(57.0, 17.0 * s);
        // Exact generator identities (floats of exact Q(√3) expressions):
        // g1² = w1, g2² = −w2.
        let gs1 = v2((mu.0 + 1.0) * t1.x + mu.1 * t1.y, mu.2 * t1.x + (mu.3 + 1.0) * t1.y);
        assert!((gs1 - w1).length() < 1e-9, "g1² must equal w1 exactly");
        let gs2 = v2((mv.0 + 1.0) * t2.x + mv.1 * t2.y, mv.2 * t2.x + (mv.3 + 1.0) * t2.y);
        assert!((gs2 + w2).length() < 1e-9, "g2² must equal −w2 exactly");
        // Each glide shifts by exactly half a primitive translation
        // along its axis (g1 by +w1/2, g2 by −w2/2) — no power is a
        // reflection.
        let u1 = w1 * (1.0 / w1.length());
        let u2 = w2 * (1.0 / w2.length());
        assert!(
            (2.0 * t1.dot(u1) / w1.length() - 1.0).abs() < 1e-9,
            "g1 must shift +w1/2"
        );
        assert!(
            (2.0 * t2.dot(u2) / w2.length() + 1.0).abs() < 1e-9,
            "g2 must shift −w2/2"
        );
        // The perpendicular parts of the glide shifts (a pure reflection
        // would keep them zero) are nonzero and exact.
        let p1 = t1 - u1 * t1.dot(u1);
        let p2 = t2 - u2 * t2.dot(u2);
        assert!((p1 - v2(-4.0, 2.0 * s)).length() < 1e-9, "t1⊥ = (−4, 2√3)");
        assert!((p2 - v2(39.0, 26.0 * s)).length() < 1e-9, "t2⊥ = (39, 26√3)");
        // The glides and their inverses preserve the tiling (the image
        // window is tilted, so check the image set is contained in the
        // tilted-window patch).
        check_symmetry(&t, &p, mu, t1, ra, false, "glide g1 (axis ∥ w1)");
        check_symmetry(
            &t,
            &p,
            mu,
            v2(-(mu.0 * t1.x + mu.1 * t1.y), -(mu.2 * t1.x + mu.3 * t1.y)),
            ra,
            false,
            "glide g1⁻¹",
        );
        check_symmetry(&t, &p, mv, t2, ra, false, "glide g2 (axis ∥ w2)");
        check_symmetry(
            &t,
            &p,
            mv,
            v2(-(mv.0 * t2.x + mv.1 * t2.y), -(mv.2 * t2.x + mv.3 * t2.y)),
            ra,
            false,
            "glide g2⁻¹",
        );
        // Big patch for tile-image membership: every probe image below
        // has its AABB intersecting the region, so every assertion is
        // non-vacuous.
        let big = BBox::new(v2(-80.0, -66.0 * s), v2(100.0, 66.0 * s));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        let set: HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();
        // g1 and g2 map the prototile exactly onto cells 4 and 7.
        let img4: Vec<Vec2> = prot.iter().map(|q| apply_iso(mu, t1, *q)).collect();
        let img7: Vec<Vec2> = prot.iter().map(|q| apply_iso(mv, t2, *q)).collect();
        assert_eq!(vkey(&img4), vkey(&t.cells[4].poly.pts), "g1 must map T1 exactly onto T5");
        assert_eq!(vkey(&img7), vkey(&t.cells[7].poly.pts), "g2 must map T1 exactly onto T8");
        // Sanity: pure lattice translations are symmetries.
        for i in -2..=2 {
            for w in [w1, w2] {
                let img: Vec<Vec2> = prot.iter().map(|q| *q + w * i as f64).collect();
                assert!(
                    set.contains(&vkey(&img)),
                    "T1 + {i}·lattice vector must be a tile"
                );
            }
        }
        // Lattice-shifted glides still map the prototile onto a tile.
        for (i, jj) in [(0.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (1.0, 0.0)] {
            let sh = w1 * i + w2 * jj;
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(mu, t1 + sh, *q)).collect();
            assert!(
                set.contains(&vkey(&img)),
                "(g1 + {i}·w1 + {jj}·w2) must map T1 onto a tile"
            );
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(mv, t2 + sh, *q)).collect();
            assert!(
                set.contains(&vkey(&img)),
                "(g2 + {i}·w1 + {jj}·w2) must map T1 onto a tile"
            );
        }
        // No pure reflection: every candidate (M, i·w) — zero or whole
        // lattice shifts along either axis — must fail to map the
        // prototile onto a tile (all true symmetries with linear part M
        // shift half a primitive translation along the axis).
        for i in -2..=2 {
            for (m, w, mname, wname) in [
                (mu, w1, "M_u", "w1"),
                (mu, w2, "M_u", "w2"),
                (mv, w1, "M_v", "w1"),
                (mv, w2, "M_v", "w2"),
            ] {
                let img: Vec<Vec2> = prot
                    .iter()
                    .map(|q| apply_iso(m, w * i as f64, *q))
                    .collect();
                assert!(
                    !set.contains(&vkey(&img)),
                    "({mname}, {i}·{wname}) must not be a symmetry (no mirrors in pgg)"
                );
            }
        }
        // The perpendicular-part candidates are pure reflections:
        // neither is a symmetry.
        for (m, tr, nm) in [(mu, p1, "(M_u, t1⊥)"), (mv, p2, "(M_v, t2⊥)")] {
            let img: Vec<Vec2> = prot.iter().map(|q| apply_iso(m, tr, *q)).collect();
            assert!(
                !set.contains(&vkey(&img)),
                "{nm} must not be a symmetry (no mirrors in pgg)"
            );
        }
        // No quarter-turn either (about the origin and about the 2-fold
        // centre).
        for (nm, tr) in [
            ("quarter-turn about the origin", Vec2::ZERO),
            ("quarter-turn about c0", v2(c0.x + c0.y, c0.y - c0.x)),
        ] {
            let img: Vec<Vec2> = prot
                .iter()
                .map(|q| apply_iso((0.0, -1.0, 1.0, 0.0), tr, *q))
                .collect();
            assert!(!set.contains(&vkey(&img)), "{nm} must not be a symmetry");
        }
    }

    /// 2-isohedral: seeds 0/1 are the two full-group isohedral classes —
    /// the half-turn and both glide classes permute the eight cells
    /// mod-lattice by the closed permutations, the group they generate
    /// has exactly two orbits on the cells ({0,3,4,7} / {1,2,5,6},
    /// matching the seeds), both seeds appear in every patch, and every
    /// generated tile keeps the prototile side multiset
    /// {14, 14, 14, 14, 6√7}.
    #[test]
    fn pentagon_8_isohedral_classes() {
        let t = spec();
        let s = s3();
        let mut counts = [0usize; 2];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(counts, [4, 4], "four tiles per isohedral class");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        let u_of = |p: Vec2| {
            v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
        };
        let mod1 = |x: f64| (x + 0.5).rem_euclid(1.0) - 0.5;
        let c0 = v2(-0.5, 12.5 * s);
        let mu = (-1.0 / 7.0, 4.0 * s / 7.0, 4.0 * s / 7.0, 1.0 / 7.0);
        let mv = (1.0 / 7.0, -4.0 * s / 7.0, -4.0 * s / 7.0, -1.0 / 7.0);
        let t1 = v2(14.0, 14.0 * s);
        let t2 = v2(57.0, 17.0 * s);
        let cents: Vec<Vec2> = t.cells.iter().map(|c| cent(&c.poly.pts)).collect();
        // A symmetry (M, t) permutes the cells: cell k's centroid maps to
        // cell j's centroid mod lattice, for exactly one j.
        let perm_of = |m: (f64, f64, f64, f64), tr: Vec2| -> Vec<usize> {
            let mut perm: Vec<usize> = Vec::new();
            for k in 0..8 {
                let img = apply_iso(m, tr, cents[k]);
                let mut hits: Vec<usize> = Vec::new();
                for (j, &cj) in cents.iter().enumerate() {
                    let du = u_of(img - cj);
                    if mod1(du.x).abs() < 1e-6 && mod1(du.y).abs() < 1e-6 {
                        hits.push(j);
                    }
                }
                assert_eq!(hits.len(), 1, "cell {k} must have exactly one image, got {hits:?}");
                perm.push(hits[0]);
            }
            perm
        };
        let perms = [
            perm_of((-1.0, 0.0, 0.0, -1.0), c0 * 2.0),
            perm_of(mu, t1),
            perm_of(mv, t2),
        ];
        assert_eq!(perms[0], vec![3, 2, 1, 0, 7, 6, 5, 4], "half-turn perm");
        assert_eq!(perms[1], vec![4, 5, 6, 7, 0, 1, 2, 3], "glide g1 perm");
        assert_eq!(perms[2], vec![7, 6, 5, 4, 3, 2, 1, 0], "glide g2 perm");
        // Two full-group orbits on the cells, exactly the seed classes.
        let mut seen = [false; 8];
        let mut orbits: Vec<Vec<usize>> = Vec::new();
        for st in 0..8 {
            if seen[st] {
                continue;
            }
            let mut orb = vec![st];
            seen[st] = true;
            let mut head = 0;
            while head < orb.len() {
                let x = orb[head];
                head += 1;
                for pm in &perms {
                    let y = pm[x];
                    if !seen[y] {
                        seen[y] = true;
                        orb.push(y);
                    }
                }
            }
            orb.sort_unstable();
            orbits.push(orb);
        }
        assert_eq!(
            orbits,
            vec![vec![0, 3, 4, 7], vec![1, 2, 5, 6]],
            "two full-group orbits"
        );
        for orb in &orbits {
            let sd = t.cells[orb[0]].seed;
            assert!(
                orb.iter().all(|&k| t.cells[k].seed == sd),
                "orbit mixes isohedral seeds"
            );
        }
        assert_ne!(t.cells[orbits[0][0]].seed, t.cells[orbits[1][0]].seed, "orbits differ");
        // Generated patches: both classes present, side multiset kept.
        let params = ParamSet::default_for(t.params());
        let rp = t.generate(
            &params,
            &Query {
                region: BBox::new(v2(-60.5, -45.5 * s), v2(60.5, 45.5 * s)),
            },
        );
        assert!(rp.message.is_none(), "{:?}", rp.message);
        let mut seen_s = [false; 2];
        for tile in &rp.tiles {
            let sd = tile.seed as usize;
            assert!(sd < 2, "seeds must stay in {{0, 1}}");
            seen_s[sd] = true;
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| {
                    (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length()
                })
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [14.0, 14.0, 14.0, 14.0, 6.0 * 7.0f64.sqrt()];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!(
                    (got - w).abs() < 1e-12,
                    "seed {sd} side {got} ∉ {{14 × 4, 6√7}}"
                );
            }
        }
        assert!(seen_s[0] && seen_s[1], "both isohedral classes must appear");
    }

    /// The eight cells realise both chiralities of the (chiral) type-8
    /// prototile: cell 0 matches the prototile's cyclic (side, angle)
    /// sequence forward but not reversed, every cell matches exactly one
    /// of the two, and the counts are four forward and four reversed —
    /// the mirror half of the primitive unit (T2, T3, T5, T8).
    #[test]
    fn pentagon_8_enantiomer_mix() {
        let t = spec();
        let proto = &t.cells[0].poly;
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        let match_fwd = |cs: &[f64], ca: &[f64]| {
            (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            })
        };
        let match_rev = |cs: &[f64], ca: &[f64]| {
            (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(rot + 9 - i) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(rot + 5 - i) % 5]).abs() < 1e-9
                })
            })
        };
        // The prototile itself: forward match only (the family is chiral).
        assert!(match_fwd(&ps, &pa), "prototile must match forward");
        assert!(!match_rev(&ps, &pa), "prototile must be chiral (no reversed match)");
        let mut n_fwd = 0;
        let mut n_rev = 0;
        for (k, cell) in t.cells.iter().enumerate() {
            let cs: Vec<f64> = (0..5).map(|i| side_of(&cell.poly, i)).collect();
            let ca: Vec<f64> = (0..5).map(|i| angle_at(&cell.poly, i)).collect();
            let f = match_fwd(&cs, &ca);
            let r = match_rev(&cs, &ca);
            assert!(f != r, "cell {k} must match exactly one chirality");
            if f {
                n_fwd += 1;
            } else {
                n_rev += 1;
            }
        }
        assert_eq!((n_fwd, n_rev), (4, 4), "four direct and four mirror tiles");
    }
}

#[cfg(test)]
mod pentagon6_tests {
    use super::pentagon6::{s3, tile_area, two_fold_centre};
    use crate::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::pentagon6::spec;
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use tess_core::category::TilingCategory;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// (x2/2, y2·√3/2) — every tile vertex lies on the half-integer ×
    /// integer·√3/2 grid, so the helper takes doubled numerators (same
    /// grid as the module's `c` helper).
    fn c(x2: f64, y2: f64) -> Vec2 {
        v2(x2 / 2.0, y2 * s3() / 2.0)
    }

    /// Apply the isometry p -> M·p + t, M given as rows (a, b), (c, d).
    fn apply_iso(m: (f64, f64, f64, f64), t: Vec2, p: Vec2) -> Vec2 {
        let (a, b, c, d) = m;
        v2(a * p.x + b * p.y + t.x, c * p.x + d * p.y + t.y)
    }

    /// Vertex-set key on a 1e-6 grid (every coordinate lies on the
    /// half-integer × integer·√3/2 grid, so distinct tiling vertices are
    /// ≥ 1/2 apart, and two computations of a shared joint agree to
    /// ~1e-15).
    fn vkey(pts: &[Vec2]) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = pts
            .iter()
            .map(|p| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64))
            .collect();
        v.sort_unstable();
        v
    }

    /// Single-vertex key on the same 1e-6 grid.
    fn pkey(p: Vec2) -> (i64, i64) {
        ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64)
    }

    /// Basis coordinates of `p` in the lattice (w1, w2).
    fn u_of(p: Vec2, w1: Vec2, w2: Vec2) -> Vec2 {
        let det = w1.cross(w2);
        v2((w2.y * p.x - w2.x * p.y) / det, (w1.x * p.y - w1.y * p.x) / det)
    }

    /// True when `v` is a lattice vector (both basis coordinates
    /// integral).
    fn is_lattice(v: Vec2, w1: Vec2, w2: Vec2) -> bool {
        let u = u_of(v, w1, w2);
        let f = |x: f64| x.rem_euclid(1.0).min(1.0 - x.rem_euclid(1.0));
        f(u.x) < 1e-9 && f(u.y) < 1e-9
    }

    /// Verify that the isometry g = (M, t) preserves the tiling: the
    /// image of every tile intersecting `ra` must be a tile intersecting
    /// `rb` (the axis-aligned box of g(ra)).  With `exact = true` the
    /// image set must equal the `rb` tile set (the case g(ra) = rb).
    fn check_symmetry(
        t: &LatticeTiling,
        params: &ParamSet,
        m: (f64, f64, f64, f64),
        tr: Vec2,
        ra: BBox,
        exact: bool,
        label: &str,
    ) {
        let g = |p: Vec2| apply_iso(m, tr, p);
        let cs: Vec<Vec2> = ra.corners().iter().map(|&p| g(p)).collect();
        let rb = BBox::new(
            v2(
                cs.iter().map(|p| p.x).fold(f64::MAX, f64::min),
                cs.iter().map(|p| p.y).fold(f64::MAX, f64::min),
            ),
            v2(
                cs.iter().map(|p| p.x).fold(f64::MIN, f64::max),
                cs.iter().map(|p| p.y).fold(f64::MIN, f64::max),
            ),
        );
        let ra_tiles = t.generate(params, &Query { region: ra });
        let rb_tiles = t.generate(params, &Query { region: rb });
        assert!(ra_tiles.message.is_none(), "{label}: {:?}", ra_tiles.message);
        assert!(rb_tiles.message.is_none(), "{label}: {:?}", rb_tiles.message);
        assert!(!ra_tiles.tiles.is_empty(), "{label}: no tiles");
        let set_b: HashSet<Vec<(i64, i64)>> =
            rb_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let set_a: HashSet<Vec<(i64, i64)>> = ra_tiles
            .tiles
            .iter()
            .map(|tile| vkey(&tile.poly.pts.iter().map(|&p| g(p)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(
            set_a.len(),
            ra_tiles.tiles.len(),
            "{label}: isometry must be injective on the patch"
        );
        if exact {
            assert_eq!(set_a, set_b, "{label}: the isometry must preserve the tiling");
        } else {
            for k in &set_a {
                assert!(set_b.contains(k), "{label}: missing image tile");
            }
        }
    }

    /// Directed-edge key: keys of its two endpoints on the 1e-6 grid.
    type EdgeKey = ((i64, i64), (i64, i64));

    /// Interior angle at vertex `i` of a CCW pentagon, and side length of
    /// edge `i` (from vertex `i` to `i + 1`).
    fn angle_at(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        let (p0, p1, p2) = (poly.pts[(i + n - 1) % n], poly.pts[i], poly.pts[(i + 1) % n]);
        let (v1, v2) = (p0 - p1, p2 - p1);
        (v1.dot(v2) / (v1.length() * v2.length())).clamp(-1.0, 1.0).acos()
    }

    fn side_of(poly: &Polygon, i: usize) -> f64 {
        let n = poly.len();
        (poly.pts[(i + 1) % n] - poly.pts[i]).length()
    }

    /// Centroid of a stored cell (mean of its five vertices).
    fn cent(pts: &[Vec2]) -> Vec2 {
        let s = pts.iter().fold(Vec2::ZERO, |acc, &p| acc + p);
        s / 5.0
    }

    #[test]
    fn pentagon_6_covers_region() {
        let t = spec();
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        let s = s3();
        check_coverage(
            &t,
            "pentagon-6",
            &p,
            BBox::new(v2(-20.0, -15.0 * s), v2(20.0, 15.0 * s)),
        );
    }

    #[test]
    fn pentagon_6_rotated_scaled() {
        check_rotated(
            &spec(),
            "pentagon-6(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(7.0, 7.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pentagon_6_deterministic() {
        check_det(&spec(), "pentagon-6", BBox::new(v2(-3.0, -3.0), v2(9.0, 9.0)));
    }

    /// Unit structure of the primitive cell: 4 tiles of the exact Q(√3)
    /// prototile (A = B = D = E = 120°, C = 60°; a = d = e = 1, b = c =
    /// 2; area 7√3/4) on the oblique lattice w1 = (1, 2√3), w2 =
    /// (−3, √3), with det = 4 × tile area = 7√3, |w1|² = 13, |w2|² = 12,
    /// w1·w2 = 3.  Every cell is a direct (rotated) copy of the
    /// prototile with the canonical alignment — vertex i of every cell
    /// carries prototile corner (A, B, C, D, E)[i] (s_k = 0 for all k) —
    /// CCW, no two cells coincide, and the family-condition identities
    /// hold to float precision.
    #[test]
    fn pentagon_6_unit_structure_is_exact() {
        let t = spec();
        assert_eq!(t.id, "pentagon-6");
        assert_eq!(t.cells.len(), 4, "the primitive unit holds four tiles");
        assert_eq!(t.lattice[0], c(2.0, 4.0), "w1 = (1, 2√3)");
        assert_eq!(t.lattice[1], c(-6.0, 2.0), "w2 = (−3, √3)");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        assert!(det > 0.0, "the lattice basis must be positively oriented");
        assert!(
            (det - 4.0 * tile_area()).abs() < 1e-9,
            "det {det} ≠ 4 × tile area = 7√3"
        );
        assert!((w1.dot(w1) - 13.0).abs() < 1e-12, "|w1|² = 1 + 12 = 13");
        assert!((w2.dot(w2) - 12.0).abs() < 1e-12, "|w2|² = 9 + 3 = 12");
        assert!((w1.dot(w2) - 3.0).abs() < 1e-12, "w1·w2 = −3 + 6 = 3");
        assert!(w1.dot(w2).abs() > 1e-9, "the lattice must be oblique");
        // Seeds follow the two full-group isohedral orbits {T0,T1} /
        // {T2,T3}.
        let seeds: Vec<u32> = t.cells.iter().map(|c| c.seed).collect();
        assert_eq!(seeds, vec![0, 0, 1, 1], "isohedral seed pattern");
        // The shipped prototile is T0, corner A at the origin:
        // A = (0,0), B = (1,0), C = (2,√3), D = (0,√3), E = (−1/2,√3/2).
        let proto = &t.cells[0].poly;
        let pin = [c(0.0, 0.0), c(2.0, 0.0), c(4.0, 2.0), c(0.0, 2.0), c(-1.0, 1.0)];
        for (i, &want) in pin.iter().enumerate() {
            assert_eq!(proto.pts[i], want, "prototile corner {i}");
        }
        // Exact corner angles in vertex order A, B, C, D, E (C = 60°).
        for (i, want) in [(0usize, 120.0f64), (1, 120.0), (2, 60.0), (3, 120.0), (4, 120.0)] {
            let got = angle_at(proto, i).to_degrees();
            assert!((got - want).abs() < 1e-9, "angle at vertex {i}: {got} ≠ {want}");
        }
        // Exact side lengths: a = d = e = 1, b = c = 2.
        for (i, want) in [(0usize, 1.0f64), (1, 2.0), (2, 2.0), (3, 1.0), (4, 1.0)] {
            assert!(
                (side_of(proto, i) - want).abs() < 1e-12,
                "side {i} (length {want})"
            );
        }
        // Type-6 family identities, from the stored values: A = 2C,
        // C + E = 180°, a = d = e, b = c.
        assert!(
            (angle_at(proto, 0) - 2.0 * angle_at(proto, 2)).abs() < 1e-9,
            "A = 2C"
        );
        assert!(
            (angle_at(proto, 2) + angle_at(proto, 4) - std::f64::consts::PI).abs() < 1e-9,
            "C + E = 180°"
        );
        assert!(
            (side_of(proto, 0) - side_of(proto, 3)).abs() < 1e-12
                && (side_of(proto, 0) - side_of(proto, 4)).abs() < 1e-12,
            "a = d = e"
        );
        assert!((side_of(proto, 1) - side_of(proto, 2)).abs() < 1e-12, "b = c");
        // Every cell: CCW, exact area, five vertices, and the canonical
        // alignment (vertex i carries corner (A,B,C,D,E)[i], edge e the
        // side e) — a direct congruent copy of the prototile (the tile
        // is achiral, so direct copies exist for every cell).
        let mut seen: HashSet<Vec<(i64, i64)>> = HashSet::new();
        for (k, cell) in t.cells.iter().enumerate() {
            assert!(cell.poly.area() > 0.0, "cell {k} must be stored CCW");
            assert!(
                (cell.poly.area() - tile_area()).abs() < 1e-12,
                "cell {k} area ≠ 7√3/4"
            );
            assert_eq!(cell.poly.len(), 5, "cell {k} vertex count");
            for (i, want) in [(0usize, 120.0f64), (1, 120.0), (2, 60.0), (3, 120.0), (4, 120.0)] {
                let got = angle_at(&cell.poly, i).to_degrees();
                assert!(
                    (got - want).abs() < 1e-9,
                    "cell {k} vertex {i} angle {got} ∉ (120,120,60,120,120)"
                );
            }
            for (i, want) in [(0usize, 1.0f64), (1, 2.0), (2, 2.0), (3, 1.0), (4, 1.0)] {
                assert!(
                    (side_of(&cell.poly, i) - want).abs() < 1e-12,
                    "cell {k} side {i} (length {want})"
                );
            }
            assert!(seen.insert(vkey(&cell.poly.pts)), "cell {k} duplicates another cell");
        }
    }

    /// Window census (default parameters): culling is the tile AABB
    /// intersecting the region (+1e-9); the quarter- and half-integer
    /// window bounds keep every culling decision unambiguous, so the
    /// counts are exact and match the independent Python emulation of
    /// the lattice.rs culling.
    #[test]
    fn pentagon_6_window_census() {
        let t = spec();
        assert_eq!(t.group, WallpaperGroup::P2);
        assert_eq!(t.lattice_kind, LatticeKind::Oblique);
        assert_eq!(t.category, TilingCategory::PentagonTypes);
        assert_eq!(t.cells[0].seed, 0, "the prototile is the T0 cell (seed 0)");
        let p = ParamSet::default_for(t.params());
        let s = s3();
        for (region, want) in [
            (
                BBox::new(v2(-60.25, -45.5 * s / 2.0), v2(60.25, 45.5 * s / 2.0)),
                3268,
            ),
            (BBox::new(v2(-30.25, -22.5 * s / 2.0), v2(30.25, 22.5 * s / 2.0)), 864),
            (BBox::new(v2(-10.25, -7.5 * s / 2.0), v2(10.25, 7.5 * s / 2.0)), 112),
            (BBox::new(v2(-2.25, -1.5 * s / 2.0), v2(3.25, 4.5 * s / 2.0)), 20),
        ] {
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{:?}", r.message);
            assert_eq!(r.tiles.len(), want, "window census for {region:?}");
        }
    }

    /// Edge-to-edge at the pentagon level over the 787-tile patch: no
    /// directed side is shared by two tiles (3935 directed edges, 0
    /// duplicates), every side whose midpoint lies strictly inside the
    /// window (3424 of them) is matched by exactly one full side of one
    /// neighbour (its reverse), and no vertex falls in the interior of a
    /// non-incident side.
    #[test]
    fn pentagon_6_edge_to_edge_gluing() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let s = s3();
        let big = BBox::new(v2(-20.25, -15.0 * s), v2(20.25, 15.0 * s));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), 787, "patch size");
        let mut dirs: std::collections::HashMap<EdgeKey, usize> =
            std::collections::HashMap::new();
        for tile in &r.tiles {
            let n = tile.poly.len();
            for i in 0..n {
                let k = (pkey(tile.poly.pts[i]), pkey(tile.poly.pts[(i + 1) % n]));
                *dirs.entry(k).or_insert(0) += 1;
            }
        }
        assert_eq!(dirs.len(), 3935, "3935 directed sides, all unique");
        assert!(
            dirs.values().all(|&c| c == 1),
            "a directed side may belong to only one tile"
        );
        let mut interior = 0;
        for tile in &r.tiles {
            let n = tile.poly.len();
            for i in 0..n {
                let (a, b) = (tile.poly.pts[i], tile.poly.pts[(i + 1) % n]);
                let mid = (a + b) * 0.5;
                let d = (mid.x - big.min.x)
                    .min(big.max.x - mid.x)
                    .min(mid.y - big.min.y)
                    .min(big.max.y - mid.y);
                if d > 1e-6 {
                    interior += 1;
                    assert_eq!(
                        dirs.get(&(pkey(b), pkey(a))),
                        Some(&1),
                        "interior side {a} -> {b} lacks its full-edge partner"
                    );
                }
            }
        }
        assert_eq!(
            interior,
            3424,
            "interior directed sides (each paired with its reverse)"
        );
        // No T-junction: no vertex in the interior of a non-incident edge.
        let verts: HashSet<(i64, i64)> =
            r.tiles.iter().flat_map(|tile| tile.poly.pts.iter().map(|&p| pkey(p))).collect();
        assert_eq!(verts.len(), 1286, "distinct vertices in the patch");
        for tile in &r.tiles {
            let n = tile.poly.len();
            for i in 0..n {
                let (a, b) = (tile.poly.pts[i], tile.poly.pts[(i + 1) % n]);
                let (dx, dy) = (b.x - a.x, b.y - a.y);
                let l2 = dx * dx + dy * dy;
                for vk in verts.iter().filter(|k| **k != pkey(a) && **k != pkey(b)) {
                    let (px, py) = (vk.0 as f64 / 1e6, vk.1 as f64 / 1e6);
                    let tt = (((px - a.x) * dx + (py - a.y) * dy) / l2).clamp(0.0, 1.0);
                    if tt == 0.0 || tt == 1.0 {
                        continue;
                    }
                    let q = a + (b - a) * tt;
                    assert!(
                        (px - q.x).hypot(py - q.y) > 1e-7,
                        "vertex {vk:?} lies in the interior of side {a} -> {b}"
                    );
                }
            }
        }
    }

    /// Vertex census over the 787-tile patch: every window-interior
    /// vertex (1022 of the 1286 distinct vertices) is a complete star of
    /// incident tile angles summing to exactly 360°, and the stars are
    /// exactly the two closed types {60°, 60°, 120°, 120°} ("AABB", 340)
    /// and {120°, 120°, 120°} ("BBB", 682).
    #[test]
    fn pentagon_6_vertex_stars() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let s = s3();
        let big = BBox::new(v2(-20.25, -15.0 * s), v2(20.25, 15.0 * s));
        let r = t.generate(&p, &Query { region: big });
        assert!(r.message.is_none(), "{:?}", r.message);
        let verts: HashSet<(i64, i64)> =
            r.tiles.iter().flat_map(|tile| tile.poly.pts.iter().map(|&pt| pkey(pt))).collect();
        // Corner cosines: 60° -> +1/2, 120° -> −1/2.
        let bins = [0.5f64, -0.5f64];
        let mut stars: BTreeMap<String, usize> = BTreeMap::new();
        let mut checked = 0;
        for vk in &verts {
            let (px, py) = (vk.0 as f64 / 1e6, vk.1 as f64 / 1e6);
            let d = (px - big.min.x)
                .min(big.max.x - px)
                .min(py - big.min.y)
                .min(big.max.y - py);
            if d < 1e-6 {
                continue; // boundary: incident tiles may be culled
            }
            // Any tile having this corner has its AABB containing the
            // strictly interior point, hence was generated: the star is
            // complete.
            let mut letters = String::new();
            let mut sum = 0.0;
            for tile in &r.tiles {
                let n = tile.poly.len();
                for i in 0..n {
                    if pkey(tile.poly.pts[i]) != *vk {
                        continue;
                    }
                    let ang = angle_at(&tile.poly, i);
                    sum += ang;
                    let c = ang.cos();
                    let j = (0..2)
                        .min_by_key(|&j| ((c - bins[j]).abs() * 1e12) as i64)
                        .unwrap();
                    assert!((c - bins[j]).abs() < 1e-9, "stray corner cosine {c}");
                    letters.push(['A', 'B'][j]);
                }
            }
            assert!(
                (sum - std::f64::consts::TAU).abs() < 1e-9,
                "vertex {vk:?} angles sum to {sum} ≠ 2π"
            );
            let mut cs: Vec<char> = letters.chars().collect();
            cs.sort_unstable();
            let star: String = cs.into_iter().collect();
            assert!(
                star == "AABB" || star == "BBB",
                "unexpected vertex star {star} at {vk:?}"
            );
            *stars.entry(star).or_insert(0) += 1;
            checked += 1;
        }
        assert_eq!(checked, 1022, "interior vertex census");
        assert_eq!(stars.get("AABB").copied().unwrap_or(0), 340, "{{60,60,120,120}} stars");
        assert_eq!(stars.get("BBB").copied().unwrap_or(0), 682, "{{120,120,120}} stars");
    }

    /// Gluing census of the 4-cell unit: the tiling is edge-to-edge at
    /// the pentagon level — every one of the 20 stored sides has exactly
    /// one full reverse neighbour among the stored sides (possibly
    /// shifted by a lattice vector): the partner map is an involution
    /// with 10 side-pair orbits, 4 pairs gluing in the unit (shift 0)
    /// and 6 pairs across the shifts ±w1, ±w2, ±(w1−w2), side-pair
    /// histogram a-a:1, a-d:2, b-b:1, b-c:2, c-c:1, d-e:2, e-e:1
    /// (every stored edge e of every cell carries prototile side e —
    /// the canonical alignment, s_k = 0 for all k, no mirror table).
    /// Mod the unit lattice the 20 directed sides wrap to 18 directed
    /// classes, with exactly one double-covered arc: the
    /// (2,√3)→(0,√3)≡(3,0) arc carries two lifts in each direction
    /// (T0e2, T3e1 / T1e2, T2e1; their other endpoints are lattice
    /// translates, (3,0) = (0,√3) − w2).  The 20 corners form 6 vertex
    /// orbits (sizes 3, 3, 3, 3, 4, 4) whose stars are 2×(60.0, 60.0,
    /// 120.0, 120.0) and 4×(120.0, 120.0, 120.0).
    #[test]
    fn pentagon_6_cell_gluing_census() {
        type PtKey = (i64, i64);
        type EdgeKey2 = (PtKey, PtKey);
        let t = spec();
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let det = w1.cross(w2);
        assert!(det > 0.0, "the shipped basis must be positively oriented");
        let ukey = |p: Vec2| {
            let u = u_of(p, w1, w2);
            (
                ((u.x.rem_euclid(1.0) * 1e6).round() as i64) % 1_000_000,
                ((u.y.rem_euclid(1.0) * 1e6).round() as i64) % 1_000_000,
            )
        };
        let cells: Vec<Vec<Vec2>> = t.cells.iter().map(|c| c.poly.pts.clone()).collect();
        // (a) Physical partner of the side (k,e) (tail a, head b): the
        // unique (j,f) != (k,e) with (j,f) + λ = reverse of (k,e) for a
        // lattice vector λ.
        let mut partner: BTreeMap<usize, (usize, usize, Vec2)> = BTreeMap::new();
        let mut lens: Vec<f64> = Vec::new();
        for (k, pts) in cells.iter().enumerate() {
            for e in 0..5 {
                let (a, b) = (pts[e], pts[(e + 1) % 5]);
                lens.push((a - b).length());
                let mut hits: Vec<(usize, usize, Vec2)> = Vec::new();
                for (j, pts2) in cells.iter().enumerate() {
                    for f in 0..5 {
                        if (j, f) == (k, e) {
                            continue;
                        }
                        let lam = b - pts2[f];
                        if is_lattice(lam, w1, w2)
                            && (pts2[(f + 1) % 5] + lam - a).length() < 1e-9
                        {
                            hits.push((j, f, lam));
                        }
                    }
                }
                assert_eq!(
                    hits.len(),
                    1,
                    "side ({k},{e}) must have exactly one full reverse partner, got {hits:?}"
                );
                partner.insert(k * 5 + e, hits.pop().unwrap());
            }
        }
        // (b) The partner map is an involution (with the inverse shift);
        // glued sides have equal length; every glue shift is one of
        // 0, ±w1, ±w2, ±(w1−w2), and exactly 4 pairs glue with shift 0.
        let in_set = |v: Vec2| {
            let near = |a: i64, b: i64| (v - (w1 * a as f64 + w2 * b as f64)).length() < 1e-9;
            [
                (0, 0),
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, -1),
                (-1, 1),
            ]
            .iter()
            .any(|&(a, b)| near(a, b))
        };
        let mut n_inunit = 0;
        let mut side_pairs: BTreeMap<(u8, u8), usize> = BTreeMap::new();
        for idx in 0..20 {
            let (k, e) = (idx / 5, idx % 5);
            let (j, f, lam) = partner[&idx];
            let pidx = j * 5 + f;
            assert!(
                partner[&pidx].0 == k && partner[&pidx].1 == e,
                "partner of ({k},{e}) is ({j},{f}) but back it is {:?}",
                partner[&pidx]
            );
            assert!(
                (partner[&pidx].2 + lam).length() < 1e-9,
                "inverse glue shift of ({k},{e}) must be −λ"
            );
            assert!(
                (lens[idx] - lens[pidx]).abs() < 1e-9,
                "glued sides ({k},{e}) and ({j},{f}) must have equal length"
            );
            assert!(
                in_set(lam),
                "glue shift {lam:?} of ({k},{e}) is outside 0, ±w1, ±w2, ±(w1−w2)"
            );
            if lam.length() < 1e-9 {
                n_inunit += 1;
            }
            // Canonical alignment: stored edge e carries prototile side
            // e for every cell; count each undirected orbit once.
            if idx < pidx {
                let pair = if e <= f { (e as u8, f as u8) } else { (f as u8, e as u8) };
                *side_pairs.entry(pair).or_insert(0) += 1;
            }
        }
        assert_eq!(n_inunit, 8, "exactly 4 pairs (8 sides) glue in the unit with shift 0");
        let want_pairs: BTreeMap<(u8, u8), usize> = [
            ((0u8, 0u8), 1usize), // a-a
            ((0, 3), 2),   // a-d
            ((1, 1), 1),   // b-b
            ((1, 2), 2),   // b-c
            ((2, 2), 1),   // c-c
            ((3, 4), 2),   // d-e
            ((4, 4), 1),   // e-e
        ]
        .into_iter()
        .collect();
        assert_eq!(side_pairs, want_pairs, "glued side-pair census (a,b,c,d,e indices)");
        // (c) Torus wrap: the 20 directed sides form 18 directed classes
        // mod the unit lattice — 16 single-lift classes and one
        // double-covered arc (with its reverse).
        let mut classes: BTreeMap<EdgeKey2, Vec<(usize, usize)>> = BTreeMap::new();
        for (k, pts) in cells.iter().enumerate() {
            for e in 0..5 {
                let (a, b) = (pts[e], pts[(e + 1) % 5]);
                classes.entry((ukey(a), ukey(b))).or_default().push((k, e));
            }
        }
        assert_eq!(classes.len(), 18, "18 directed torus classes");
        let mut double: BTreeMap<EdgeKey2, Vec<(usize, usize)>> = BTreeMap::new();
        for (key, lifts) in &classes {
            if lifts.len() > 1 {
                double.insert(*key, lifts.clone());
            }
        }
        assert_eq!(double.len(), 2, "exactly one double-covered arc (plus its reverse)");
        let q = ukey(c(0.0, 2.0)); // (0,√3), the class of (3,0): u = (3/7, 1/7)
        let p = ukey(c(4.0, 2.0)); // (2,√3): u = (5/7, 4/7)
        assert_eq!(
            double.get(&(q, p)),
            Some(&vec![(1, 2), (2, 1)]),
            "reverse double arc (0,√3)→(2,√3): lifts T1e2, T2e1 (tails differ by w2)"
        );
        assert_eq!(
            double.get(&(p, q)),
            Some(&vec![(0, 2), (3, 1)]),
            "double arc (2,√3)→(0,√3): lifts T0e2, T3e1 (heads differ by w2)"
        );
        // Vertex orbits mod lattice and their stars.
        let mut orbits: BTreeMap<PtKey, Vec<(usize, usize)>> = BTreeMap::new();
        for (k, pts) in cells.iter().enumerate() {
            for (v, &p) in pts.iter().enumerate() {
                orbits.entry(ukey(p)).or_default().push((k, v));
            }
        }
        assert_eq!(orbits.len(), 6, "six vertex orbits on the torus");
        let mut sizes: Vec<usize> = orbits.values().map(|o| o.len()).collect();
        sizes.sort_unstable();
        assert_eq!(sizes, vec![3, 3, 3, 3, 4, 4], "orbit sizes");
        let mut stars: Vec<Vec<i64>> = Vec::new();
        for corners in orbits.values() {
            let mut tens: Vec<i64> = corners
                .iter()
                .map(|&(k, v)| {
                    (angle_at(&t.cells[k].poly, v).to_degrees() * 10.0).round() as i64
                })
                .collect();
            tens.sort_unstable();
            stars.push(tens);
        }
        stars.sort();
        let want_stars: Vec<Vec<i64>> = [
            vec![600, 600, 1200, 1200],
            vec![600, 600, 1200, 1200],
            vec![1200, 1200, 1200],
            vec![1200, 1200, 1200],
            vec![1200, 1200, 1200],
            vec![1200, 1200, 1200],
        ]
        .to_vec();
        assert_eq!(stars, want_stars, "vertex-orbit stars (degree tenths)");
    }

    /// The wallpaper group is p2: the half-turn about the 2-fold centre
    /// c0 = (1/2, 0) preserves the tiling exactly, and the remaining
    /// three classes c0 + Λ/2 mod Λ — c0 + w1/2 = (1, √3), c0 + w2/2 =
    /// (−1, √3/2), c0 + (w1+w2)/2 = (−1/2, 3√3/2) — are exactly the
    /// remaining half-turn centre classes (every representative, with
    /// every ±w1, ±w2 shift, maps the cell-0 tile onto a tile).  The
    /// lattice points and the off-coset half-lattice points
    /// {0, w1/2, w2/2, (w1+w2)/2} are not centres (exhaustive search
    /// over all 14 candidate Λ/2 cosets finds exactly one centre coset).
    /// No mirror or glide: a symmetry g = t + R conjugates the
    /// translation group T = Λ to itself, so R must normalize Λ; a
    /// mirror's axis must permute the tiling's edge-direction set
    /// {0°, 60°, 120°} mod 180°, leaving exactly six candidate
    /// directions (0°, 30°, 60°, 90°, 120°, 150°), and none of them
    /// normalizes Λ.  (The 4-cell block check alone is not a
    /// completeness proof: it admits 4 spurious subset maps with
    /// R(Λ) ≠ Λ — the normalizer condition is the complete one.)  No
    /// 3- or 4-fold rotation normalizes Λ either, so the group is
    /// exactly p2.  All generators are exact Q(√3).
    #[test]
    fn pentagon_6_p2_symmetry() {
        let t = spec();
        let p = ParamSet::default_for(t.params());
        let s = s3();
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let c0 = two_fold_centre();
        assert_eq!(c0, v2(0.5, 0.0), "c0 = (1/2, 0)");
        // A window symmetric about c0: the half-turn must give exact
        // set equality of the tile patch.
        let ra = BBox::new(
            v2(c0.x - 16.0, c0.y - 12.0 * s),
            v2(c0.x + 16.0, c0.y + 12.0 * s),
        );
        check_symmetry(&t, &p, (-1.0, 0.0, 0.0, -1.0), c0 * 2.0, ra, true, "half-turn about c0");
        // Big patch for tile-image membership: every probe image below
        // lies inside this footprint, so membership in the generated set
        // is the exact question.
        let big = BBox::new(v2(-20.25, -15.0 * s), v2(20.25, 15.0 * s));
        let big_tiles = t.generate(&p, &Query { region: big });
        assert!(big_tiles.message.is_none(), "big patch: {:?}", big_tiles.message);
        let set: HashSet<Vec<(i64, i64)>> =
            big_tiles.tiles.iter().map(|tile| vkey(&tile.poly.pts)).collect();
        let prot = t.cells[0].poly.pts.clone();
        // Sanity: pure lattice translations are symmetries.
        for i in -2..=2 {
            for w in [w1, w2] {
                let img: Vec<Vec2> = prot.iter().map(|q| *q + w * i as f64).collect();
                assert!(
                    set.contains(&vkey(&img)),
                    "T0 + {i}·lattice vector must be a tile"
                );
            }
        }
        // The one centre coset, c0 + Λ/2: all four representatives
        // (mod Λ) are centres, with every whole lattice shift.
        for h in [Vec2::ZERO, w1 * 0.5, w2 * 0.5, (w1 + w2) * 0.5] {
            let c = c0 + h;
            for i in -1..=1 {
                for j in -1..=1 {
                    let tr = c * 2.0 + w1 * i as f64 + w2 * j as f64;
                    let img: Vec<Vec2> =
                        prot.iter().map(|q| apply_iso((-1.0, 0.0, 0.0, -1.0), tr, *q)).collect();
                    assert!(
                        set.contains(&vkey(&img)),
                        "half-turn about {:?} + lattice must map T0 onto a tile",
                        c
                    );
                }
            }
        }
        // The off-coset half-lattice points are not centres (a half-turn
        // about one has a lattice-translation square, so it would have to
        // be a centre of the p2 group; the only centre coset is c0 +
        // Λ/2, and 0 ∉ c0 + Λ/2).
        for c in [Vec2::ZERO, w1 * 0.5, w2 * 0.5, (w1 + w2) * 0.5] {
            let img: Vec<Vec2> =
                prot.iter().map(|q| apply_iso((-1.0, 0.0, 0.0, -1.0), c * 2.0, *q)).collect();
            assert!(
                !set.contains(&vkey(&img)),
                "half-turn about {c:?} must not be a symmetry"
            );
        }
        // No mirror or glide: their linear part must normalize Λ, and a
        // mirror's axis direction must permute the edge-direction set
        // {0°, 60°, 120°} mod 180° — exactly the six directions below,
        // none of which normalizes Λ.
        for deg in [0.0, 30.0, 60.0, 90.0, 120.0, 150.0] {
            let th = deg * std::f64::consts::PI / 180.0;
            let (u, wgt) = (th.cos(), th.sin());
            let m = (
                2.0 * u * u - 1.0,
                2.0 * u * wgt,
                2.0 * u * wgt,
                2.0 * wgt * wgt - 1.0,
            );
            assert!(
                !(is_lattice(apply_iso(m, Vec2::ZERO, w1), w1, w2)
                    && is_lattice(apply_iso(m, Vec2::ZERO, w2), w1, w2)),
                "reflection in direction {deg}° must not normalize the lattice (no mirrors/glides)"
            );
        }
        // No 3- or 4-fold rotation either (wallpaper rotations have
        // order 2, 3, 4, or 6; the 6-fold and 3-fold share R120, and
        // R60 is checked directly): none normalizes Λ.
        for (nm, m) in [
            ("R60", (0.5, -s / 2.0, s / 2.0, 0.5)),
            ("R90", (0.0, -1.0, 1.0, 0.0)),
            ("R120", (-0.5, -s / 2.0, s / 2.0, -0.5)),
        ] {
            assert!(
                !(is_lattice(apply_iso(m, Vec2::ZERO, w1), w1, w2)
                    && is_lattice(apply_iso(m, Vec2::ZERO, w2), w1, w2)),
                "{nm} must not normalize the lattice (no {nm} symmetry)"
            );
        }
    }

    /// 2-isohedral: seeds 0/1 are the two full-group isohedral classes —
    /// the half-turn about any centre (hence all four c0 + Λ/2
    /// representatives) permutes the four cells mod lattice by the
    /// closed permutation 0 ↔ 1, 2 ↔ 3, the group they generate has
    /// exactly two orbits on the cells ({0, 1} / {2, 3}, matching the
    /// seeds), both seeds appear in every patch, and every generated
    /// tile keeps the prototile side multiset {1, 1, 1, 2, 2}.
    #[test]
    fn pentagon_6_isohedral_classes() {
        let t = spec();
        let s = s3();
        let mut counts = [0usize; 2];
        for c in &t.cells {
            counts[c.seed as usize] += 1;
        }
        assert_eq!(counts, [2, 2], "two tiles per isohedral class");
        let (w1, w2) = (t.lattice[0], t.lattice[1]);
        let mod1 = |x: f64| (x + 0.5).rem_euclid(1.0) - 0.5;
        let c0 = two_fold_centre();
        let cents: Vec<Vec2> = t.cells.iter().map(|c| cent(&c.poly.pts)).collect();
        // A half-turn about centre c/2 (linear part −I) permutes the
        // cells: cell k's centroid maps to cell j's centroid mod
        // lattice, for exactly one j.
        let perm_of = |tr: Vec2| -> Vec<usize> {
            let mut perm: Vec<usize> = Vec::new();
            for k in 0..4 {
                let img = tr - cents[k];
                let mut hits: Vec<usize> = Vec::new();
                for (j, &cj) in cents.iter().enumerate() {
                    let du = u_of(img - cj, w1, w2);
                    if mod1(du.x).abs() < 1e-6 && mod1(du.y).abs() < 1e-6 {
                        hits.push(j);
                    }
                }
                assert_eq!(hits.len(), 1, "cell {k} must have exactly one image, got {hits:?}");
                perm.push(hits[0]);
            }
            perm
        };
        // All four c0 + Λ/2 representatives give the same cell
        // permutation.
        let mut perms = Vec::new();
        for h in [Vec2::ZERO, w1 * 0.5, w2 * 0.5, (w1 + w2) * 0.5] {
            let c = c0 + h;
            perms.push(perm_of(c * 2.0));
        }
        for (i, pm) in perms.iter().enumerate() {
            assert_eq!(*pm, vec![1, 0, 3, 2], "half-turn perm (rep {i})");
        }
        // Two full-group orbits on the cells, exactly the seed classes.
        let mut seen = [false; 4];
        let mut orbits: Vec<Vec<usize>> = Vec::new();
        for st in 0..4 {
            if seen[st] {
                continue;
            }
            let mut orb = vec![st];
            seen[st] = true;
            let mut head = 0;
            while head < orb.len() {
                let x = orb[head];
                head += 1;
                for pm in &perms {
                    let y = pm[x];
                    if !seen[y] {
                        seen[y] = true;
                        orb.push(y);
                    }
                }
            }
            orb.sort_unstable();
            orbits.push(orb);
        }
        assert_eq!(
            orbits,
            vec![vec![0, 1], vec![2, 3]],
            "two full-group orbits"
        );
        for orb in &orbits {
            let sd = t.cells[orb[0]].seed;
            assert!(
                orb.iter().all(|&k| t.cells[k].seed == sd),
                "orbit mixes isohedral seeds"
            );
        }
        assert_ne!(
            t.cells[orbits[0][0]].seed,
            t.cells[orbits[1][0]].seed,
            "orbits differ"
        );
        // Generated patches: both classes present, side multiset kept.
        let params = ParamSet::default_for(t.params());
        let rp = t.generate(
            &params,
            &Query {
                region: BBox::new(v2(-20.25, -15.0 * s), v2(20.25, 15.0 * s)),
            },
        );
        assert!(rp.message.is_none(), "{:?}", rp.message);
        let mut seen_s = [false; 2];
        for tile in &rp.tiles {
            let sd = tile.seed as usize;
            assert!(sd < 2, "seeds must stay in {{0, 1}}");
            seen_s[sd] = true;
            let mut sides: Vec<f64> = (0..tile.poly.len())
                .map(|i| {
                    (tile.poly.pts[(i + 1) % tile.poly.len()] - tile.poly.pts[i]).length()
                })
                .collect();
            sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let want = [1.0, 1.0, 1.0, 2.0, 2.0];
            for (got, w) in sides.iter().zip(want.iter()) {
                assert!(
                    (got - w).abs() < 1e-12,
                    "seed {sd} side {got} ∉ {{1 × 3, 2 × 2}}"
                );
            }
        }
        assert!(seen_s[0] && seen_s[1], "both isohedral classes must appear");
    }

    /// The type-6 prototile is ACHIRAL: its mirror image is directly
    /// congruent to it (unlike the other shipped pentagon families, the
    /// chiral-prototile argument cannot exclude mirrors here — the group
    /// is p2 by the normalizer argument in `pentagon_6_p2_symmetry`).
    /// Exact Q(√3) identities: as sets, mirror_y(T0) = R300·T0 +
    /// (−1/2, −√3/2); and pointwise, with MIR in the mirror's own CCW
    /// order (MIR[i] = mirror_y of corner (A,B,C,D,E)[4−i]),
    /// Rot60·MIR[i] + (−1/2, √3/2) = PIN[i] for every i.
    #[test]
    fn pentagon_6_achiral_tile() {
        let s = s3();
        // Pinned prototile (A, B, C, D, E).
        let pin = [c(0.0, 0.0), c(2.0, 0.0), c(4.0, 2.0), c(0.0, 2.0), c(-1.0, 1.0)];
        // The mirror image (reflection in the x-axis) in its own CCW
        // order: E', D', C', B', A'.
        let mir = [
            v2(pin[4].x, -pin[4].y),
            v2(pin[3].x, -pin[3].y),
            v2(pin[2].x, -pin[2].y),
            v2(pin[1].x, -pin[1].y),
            v2(pin[0].x, -pin[0].y),
        ];
        // Set form: mirror_y(T0) = R300·T0 + (−1/2, −√3/2).
        let r300 = (0.5, s / 2.0, -s / 2.0, 0.5);
        let set_r: BTreeSet<(i64, i64)> = pin
            .iter()
            .map(|p| pkey(apply_iso(r300, v2(-0.5, -s / 2.0), *p)))
            .collect();
        let set_m: BTreeSet<(i64, i64)> = mir.iter().map(|p| pkey(*p)).collect();
        assert_eq!(
            set_m, set_r,
            "mirror_y(T0) must equal R300·T0 + (−1/2, −√3/2) as a set"
        );
        // Pointwise: Rot60·MIR[i] + (−1/2, √3/2) = PIN[i] (same index) —
        // the mirror image is a direct congruent copy of the tile.
        let r60 = (0.5, -s / 2.0, s / 2.0, 0.5);
        for i in 0..5 {
            let img = apply_iso(r60, v2(-0.5, s / 2.0), mir[i]);
            assert!(
                (img - pin[i]).length() < 1e-12,
                "achirality: Rot60·MIR[{i}] + (−1/2, √3/2) = {img:?} ≠ PIN[{i}] = {:#?}",
                pin[i]
            );
        }
        // Every shipped cell is a direct congruent copy of the prototile
        // (the achiral shape admits no mirror-only cells).
        let t = spec();
        let proto = &t.cells[0].poly;
        let ps = (0..5).map(|i| side_of(proto, i)).collect::<Vec<_>>();
        let pa = (0..5).map(|i| angle_at(proto, i)).collect::<Vec<_>>();
        for (k, cell) in t.cells.iter().enumerate() {
            let cs = (0..5).map(|i| side_of(&cell.poly, i)).collect::<Vec<_>>();
            let ca = (0..5).map(|i| angle_at(&cell.poly, i)).collect::<Vec<_>>();
            let fwd = (0..5).any(|rot| {
                (0..5).all(|i| {
                    (cs[i] - ps[(i + rot) % 5]).abs() < 1e-12
                        && (ca[i] - pa[(i + rot) % 5]).abs() < 1e-9
                })
            });
            assert!(fwd, "cell {k} must be a directly congruent copy of the prototile");
        }
    }

    /// Floret kinship: the pinned prototile is congruent to the
    /// floret-pentagonal tile (the floret_pentagonal shape in laves.rs,
    /// defined locally here to avoid a feature dependency) — pin =
    /// R30⁷·(√3·FLORET6) + (2, √3), pointwise on all five vertices, and
    /// (k, t) = (7, (2, √3)) is the unique solution over the 12
    /// rotations.  Same pentagon shape (angles (120, 120, 60, 120, 120),
    /// sides a = d = e = 1, b = c = 2 at the √3 scale); different
    /// tilings: the floret's is p6, 1-isohedral, this one is p2,
    /// 2-isohedral.
    #[test]
    fn pentagon_6_floret_kinship() {
        let s = s3();
        // laves.rs floret_pentagonal(): H = √3/2; the five prototile
        // corners of one floret tile, in the laves vertex order:
        let h = s / 2.0;
        let f0 = v2(1.0, -2.0 * h / 3.0);
        let f1 = v2(1.5, -h / 3.0);
        let f2 = v2(1.5, h / 3.0);
        let f3 = v2(1.0, 2.0 * h / 3.0);
        let f4 = v2(0.0, 0.0);
        // Canonical order (60° corner at index 2): [f2, f3, f4, f0, f1].
        let f6 = [f2, f3, f4, f0, f1];
        // The floret at the shipped scale (√3 × the laves scale):
        // angles (120, 120, 60, 120, 120), sides 1, 2, 2, 1, 1 — the
        // same type-6 shape as the pinned prototile.
        let fs = [f6[0] * s, f6[1] * s, f6[2] * s, f6[3] * s, f6[4] * s];
        let poly = Polygon::from_slice(&[fs[0], fs[1], fs[2], fs[3], fs[4]]);
        for (i, want) in [(0usize, 120.0f64), (1, 120.0), (2, 60.0), (3, 120.0), (4, 120.0)] {
            let got = angle_at(&poly, i).to_degrees();
            assert!((got - want).abs() < 1e-9, "floret angle at vertex {i}: {got} ≠ {want}");
        }
        for (i, want) in [(0usize, 1.0f64), (1, 2.0), (2, 2.0), (3, 1.0), (4, 1.0)] {
            assert!(
                (side_of(&poly, i) - want).abs() < 1e-12,
                "floret side {i} (length {want})"
            );
        }
        // The congruence: pin = R30^k·(√3·FLORET6) + t, pointwise for
        // all five vertices, over all 12 rotations; the unique solution
        // is k = 7, t = (2, √3).
        let t = spec();
        let pin = t.cells[0].poly.pts.clone();
        let cs = [
            (1.0, 0.0),
            (s / 2.0, 0.5),
            (0.5, s / 2.0),
            (0.0, 1.0),
            (-0.5, s / 2.0),
            (-s / 2.0, 0.5),
            (-1.0, 0.0),
            (-s / 2.0, -0.5),
            (-0.5, -s / 2.0),
            (0.0, -1.0),
            (0.5, -s / 2.0),
            (s / 2.0, -0.5),
        ];
        let mut solutions: Vec<(usize, Vec2)> = Vec::new();
        for (k, &(cx, sy)) in cs.iter().enumerate() {
            let rot = (cx, -sy, sy, cx);
            let tr = pin[0] - apply_iso(rot, Vec2::ZERO, fs[0]);
            if (0..5).all(|i| (apply_iso(rot, tr, fs[i]) - pin[i]).length() < 1e-12) {
                solutions.push((k, tr));
            }
        }
        assert_eq!(solutions.len(), 1, "unique (k, t) over the 12 rotations: {solutions:?}");
        let (k, tr) = solutions[0];
        assert_eq!(k, 7, "k = 7 (R210)");
        assert!(
            (tr - v2(2.0, s)).length() < 1e-12,
            "t = (2, √3), got {tr:?}"
        );
    }
}


