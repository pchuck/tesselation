//! Laves family: Laves/Catalan duals (the 7 proper duals + prismatic/floret pentagons)
//! and the cairo pentagonal (Laves primary, Pentagon-types secondary)
//!
//! Owned here: module declarations, `register` (the family's registry
//! slice, in within-tab order), `classification` arms, the family's
//! lattice-culling entries and the family census pin. The family doc is
//! `docs/families/laves.md`.

use tess_core::Registry;
use tess_core::classify::TilingClass;

use crate::classify::{laves};

pub mod cairo;
pub mod laves;

/// The family's registry slice, in within-tab order.
pub fn register(mut r: Registry) -> Registry {
        r = r.register(cairo::cairo_pentagonal());
        r = r.register(laves::tetrakis_square());
        r = r.register(laves::rhombille());
        r = r.register(laves::triakis_triangular());
        r = r.register(laves::deltoidal_trihexagonal());
        r = r.register(laves::kisrhombille());
        r = r.register(laves::prismatic_pentagonal());
        r = r.register(laves::floret_pentagonal());
    r
}

/// The family's classification arms (see `crate::classify` for the
/// composer and the shared constructor helpers).
pub(crate) fn classification(id: &str) -> Option<TilingClass> {
    // Arms are verbatim from the pre-split table (they yield
    // `TilingClass`); an unknown id returns None so the composer can
    // fall through to later families.
    Some(match id {
        "tetrakis-square" => laves(&["Laves"], "V4.8.8", "truncated square"),
        "rhombille" => laves(&["Laves"], "V3.6.3.6", "trihexagonal"),
        "triakis-triangular" => laves(&["Laves"], "V3.12.12", "truncated hexagonal"),
        "deltoidal-trihexagonal" => laves(&["Laves"], "V3.4.6.4", "rhombitrihexagonal"),
        "kisrhombille" => laves(&["Laves"], "V4.6.12", "truncated trihexagonal"),
        "prismatic-pentagonal" => laves(&["Laves"], "V3.3.3.4.4", "elongated triangular"),
        "floret-pentagonal" => TilingClass {
            pentagon_type: Some(5),
            isohedral: Some(1),
            ..laves(&["Laves", "Pentagon"], "V3.3.3.3.6", "snub trihexagonal")
        },
        "cairo-pentagonal" => TilingClass {
            pentagon_type: Some(4),
            isohedral: Some(1),
            ..laves(&["Laves", "Pentagon"], "V3.3.4.3.4", "snub square")
        },
        _ => return None,
    })
}

/// This family's entries for the global lattice-culling cross-check
/// (`lattice_culling_tests` in `lib.rs`).
#[cfg(test)]
pub(crate) fn lattice_tilings() -> Vec<(&'static str, crate::lattice::LatticeTiling)> {
    vec![
            ("cairo", cairo::cairo_pentagonal()),
            ("tetrakis-square", laves::tetrakis_square()),
            ("rhombille", laves::rhombille()),
            ("triakis-triangular", laves::triakis_triangular()),
            ("deltoidal-trihexagonal", laves::deltoidal_trihexagonal()),
            ("kisrhombille", laves::kisrhombille()),
            ("prismatic-pentagonal", laves::prismatic_pentagonal()),
            ("floret-pentagonal", laves::floret_pentagonal()),
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
        "tetrakis-square",
        "rhombille",
        "triakis-triangular",
        "deltoidal-trihexagonal",
        "kisrhombille",
        "prismatic-pentagonal",
        "floret-pentagonal",
        "cairo-pentagonal",
    ];

    #[test]
    fn family_census_matches() {
        let r = register(Registry::new());
        assert_eq!(
            r.len(),
            8,
            "laves family census — update when adding a tiling to this family"
        );
        let have: BTreeSet<&str> = r.iter().map(|t| t.id()).collect();
        let want: BTreeSet<&str> = IDS.iter().copied().collect();
        assert_eq!(
            have, want,
            "laves family membership drift: register() vs IDS"
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
mod cairo_tests {
    use crate::tests::{check_covers, check_det, check_rotated};
    use crate::cairo;
    use tess_core::geometry::{BBox, Polygon, Vec2};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn cairo_pentagonal_covers_region() {
        check_covers(
            &cairo::cairo_pentagonal(),
            "cairo-pentagonal",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn cairo_pentagonal_rotated_scaled() {
        check_rotated(
            &cairo::cairo_pentagonal(),
            "cairo-pentagonal(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn cairo_pentagonal_deterministic() {
        check_det(
            &cairo::cairo_pentagonal(),
            "cairo-pentagonal",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// Interior angles (degrees) at the five vertices of a cell pentagon.
    fn interior_angles(poly: &Polygon) -> [f64; 5] {
        let v = &poly.pts;
        let mut a = [0.0; 5];
        for i in 0..5 {
            let u = v[(i + 4) % 5] - v[i];
            let w = v[(i + 1) % 5] - v[i];
            a[i] = (u.dot(w) / (u.length() * w.length()))
                .clamp(-1.0, 1.0)
                .acos()
                .to_degrees();
        }
        a
    }

    /// Squared lengths of the five edges of a cell pentagon.
    fn squared_sides(poly: &Polygon) -> [f64; 5] {
        let v = &poly.pts;
        let mut s = [0.0; 5];
        for i in 0..5 {
            let e = v[(i + 1) % 5] - v[i];
            s[i] = e.dot(e);
        }
        s
    }

    /// M101 pentagon-census audit (PROPOSED.md §5): the cairo cell
    /// pentagons are type 4 in the repo lettering — all eight
    /// congruent (angles {90° ×2, 120° ×3}; squared sides
    /// {(2+√3)/6 ×4, 1/3}; per-pentagon area (2 + √3)/4, exact in
    /// Q(√3)) — and the first cell satisfies the type-4 conditions at
    /// ccw shift k=4 (A = v4, B = v0, C = v1, D = v2, E = v3):
    /// C = E = 90°, b = c, d = e (repo letters shift the Wikipedia
    /// letters by one: W-angle X = our-(X + 1 mod 5), side a joins
    /// A–B).
    #[test]
    fn cairo_pentagonal_is_pentagon_type_4() {
        let t = cairo::cairo_pentagonal();
        let long2 = (2.0 + 3.0f64.sqrt()) / 6.0; // (2 + √3)/6
        let area = (2.0 + 3.0f64.sqrt()) / 4.0; // (2 + √3)/4
        for cell in &t.cells {
            let mut ang: Vec<f64> = interior_angles(&cell.poly).to_vec();
            ang.sort_by(f64::total_cmp);
            let want = [90.0, 90.0, 120.0, 120.0, 120.0];
            assert!(
                ang.iter().zip(&want).all(|(x, y)| (x - y).abs() < 1e-9),
                "angles {ang:?}"
            );
            let mut s2: Vec<f64> = squared_sides(&cell.poly).to_vec();
            s2.sort_by(f64::total_cmp);
            let want: Vec<f64> =
                vec![1.0 / 3.0, long2, long2, long2, long2];
            assert!(
                s2.iter().zip(&want).all(|(x, y)| (x - y).abs() < 1e-12),
                "squared sides {s2:?}"
            );
            assert!(
                (cell.poly.area() - area).abs() < 1e-12,
                "area {} != (2 + √3)/4",
                cell.poly.area()
            );
        }
        // Type-4 lettering on the first cell, ccw shift k=4.
        let poly = &t.cells[0].poly;
        let ang = interior_angles(poly);
        assert!((ang[1] - 90.0).abs() < 1e-9, "C = v1: {}", ang[1]);
        assert!((ang[3] - 90.0).abs() < 1e-9, "E = v3: {}", ang[3]);
        let v = &poly.pts;
        let vtx = |i: usize| v[(i + 4) % 5]; // letter i (0 = A .. 4 = E)
        let side2 = |i: usize, j: usize| {
            let d = vtx(i) - vtx(j);
            d.dot(d)
        };
        assert!(
            (side2(1, 2) - side2(2, 3)).abs() < 1e-12,
            "b = c: {} vs {}",
            side2(1, 2),
            side2(2, 3)
        );
        assert!(
            (side2(3, 4) - side2(4, 0)).abs() < 1e-12,
            "d = e: {} vs {}",
            side2(3, 4),
            side2(4, 0)
        );
    }

}

#[cfg(test)]
mod laves_tests {
    use crate::tests::{check_covers, check_det, check_rotated};
    use crate::lattice::LatticeTiling;
    use crate::laves;
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::ParamSet;
    use tess_core::tiling::{Query, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn tetrakis_square_covers_region() {
        check_covers(
            &laves::tetrakis_square(),
            "tetrakis-square",
            BBox::new(v2(-4.0, -3.0), v2(12.0, 12.0)),
        );
    }

    #[test]
    fn tetrakis_square_rotated_scaled() {
        check_rotated(
            &laves::tetrakis_square(),
            "tetrakis-square(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn tetrakis_square_deterministic() {
        check_det(
            &laves::tetrakis_square(),
            "tetrakis-square",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn rhombille_covers_region() {
        check_covers(
            &laves::rhombille(),
            "rhombille",
            BBox::new(v2(-4.0, -3.0), v2(12.0, 12.0)),
        );
    }

    #[test]
    fn rhombille_rotated_scaled() {
        check_rotated(
            &laves::rhombille(),
            "rhombille(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn rhombille_deterministic() {
        check_det(
            &laves::rhombille(),
            "rhombille",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn triakis_triangular_covers_region() {
        check_covers(
            &laves::triakis_triangular(),
            "triakis-triangular",
            BBox::new(v2(-4.0, -3.0), v2(12.0, 12.0)),
        );
    }

    #[test]
    fn triakis_triangular_rotated_scaled() {
        check_rotated(
            &laves::triakis_triangular(),
            "triakis-triangular(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn triakis_triangular_deterministic() {
        check_det(
            &laves::triakis_triangular(),
            "triakis-triangular",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn deltoidal_trihexagonal_covers_region() {
        check_covers(
            &laves::deltoidal_trihexagonal(),
            "deltoidal-trihexagonal",
            BBox::new(v2(-4.0, -3.0), v2(12.0, 12.0)),
        );
    }

    #[test]
    fn deltoidal_trihexagonal_rotated_scaled() {
        check_rotated(
            &laves::deltoidal_trihexagonal(),
            "deltoidal-trihexagonal(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn deltoidal_trihexagonal_deterministic() {
        check_det(
            &laves::deltoidal_trihexagonal(),
            "deltoidal-trihexagonal",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn kisrhombille_covers_region() {
        check_covers(
            &laves::kisrhombille(),
            "kisrhombille",
            BBox::new(v2(-6.0, -4.0), v2(16.0, 14.0)),
        );
    }

    #[test]
    fn kisrhombille_rotated_scaled() {
        check_rotated(
            &laves::kisrhombille(),
            "kisrhombille(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(8.0, 8.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kisrhombille_deterministic() {
        check_det(
            &laves::kisrhombille(),
            "kisrhombille",
            BBox::new(v2(-4.0, -4.0), v2(12.0, 12.0)),
        );
    }

    #[test]
    fn prismatic_pentagonal_covers_region() {
        check_covers(
            &laves::prismatic_pentagonal(),
            "prismatic-pentagonal",
            BBox::new(v2(-4.0, -3.0), v2(12.0, 12.0)),
        );
    }

    #[test]
    fn prismatic_pentagonal_rotated_scaled() {
        check_rotated(
            &laves::prismatic_pentagonal(),
            "prismatic-pentagonal(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn prismatic_pentagonal_deterministic() {
        check_det(
            &laves::prismatic_pentagonal(),
            "prismatic-pentagonal",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn floret_pentagonal_covers_region() {
        check_covers(
            &laves::floret_pentagonal(),
            "floret-pentagonal",
            BBox::new(v2(-6.0, -5.0), v2(14.0, 13.0)),
        );
    }

    #[test]
    fn floret_pentagonal_rotated_scaled() {
        check_rotated(
            &laves::floret_pentagonal(),
            "floret-pentagonal(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(8.0, 8.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn floret_pentagonal_deterministic() {
        check_det(
            &laves::floret_pentagonal(),
            "floret-pentagonal",
            BBox::new(v2(-5.0, -5.0), v2(11.0, 11.0)),
        );
    }

    /// Interior angles (degrees) at the five vertices of a cell pentagon.
    fn interior_angles(poly: &Polygon) -> [f64; 5] {
        let v = &poly.pts;
        let mut a = [0.0; 5];
        for i in 0..5 {
            let u = v[(i + 4) % 5] - v[i];
            let w = v[(i + 1) % 5] - v[i];
            a[i] = (u.dot(w) / (u.length() * w.length()))
                .clamp(-1.0, 1.0)
                .acos()
                .to_degrees();
        }
        a
    }

    /// Squared lengths of the five edges of a cell pentagon.
    fn squared_sides(poly: &Polygon) -> [f64; 5] {
        let v = &poly.pts;
        let mut s = [0.0; 5];
        for i in 0..5 {
            let e = v[(i + 1) % 5] - v[i];
            s[i] = e.dot(e);
        }
        s
    }

    /// M101 pentagon-census audit (PROPOSED.md §5): both prismatic
    /// cell pentagons are type 1 in the repo lettering — C + D = 180°
    /// and A + B + E = 360° (repo letters shift the Wikipedia letters
    /// by one: W-angle X = our-(X + 1 mod 5), side a joins A–B) —
    /// checked at their cyclic shifts: the up pentagon at ccw k=1
    /// (A = v1, B = v2, C = v3, D = v4, E = v0), the down pentagon at
    /// k=3 (A = v3, B = v4, C = v0, D = v1, E = v2). Angles are
    /// exactly 90°/120°; squared sides {1/3 ×2, (2+√3)/6 ×2, 1} and
    /// the per-pentagon area (1 + √3/2)/2 = 1/2 + √3/4 are exact in
    /// Q(√3).
    #[test]
    fn prismatic_pentagonal_is_pentagon_type_1() {
        let t = laves::prismatic_pentagonal();
        let long2 = (2.0 + 3.0f64.sqrt()) / 6.0; // (2 + √3)/6
        let area = (1.0 + 3.0f64.sqrt() / 2.0) / 2.0; // 1/2 + √3/4
        let cases: [(usize, &Polygon); 2] =
            [(1, &t.cells[0].poly), (3, &t.cells[1].poly)];
        for (k, poly) in cases {
            let ang = interior_angles(poly);
            let at = |i: usize| ang[(i + k) % 5]; // letter i (0 = A .. 4 = E)
            let c = at(2);
            let d = at(3);
            assert!(
                (c + d - 180.0).abs() < 1e-9,
                "k={k}: C + D = {c} + {d} != 180"
            );
            assert!(
                (at(0) + at(1) + at(4) - 360.0).abs() < 1e-9,
                "k={k}: A + B + E = {} + {} + {} != 360",
                at(0),
                at(1),
                at(4)
            );
            for x in &ang {
                assert!(
                    (x - 90.0).abs() < 1e-9 || (x - 120.0).abs() < 1e-9,
                    "k={k}: angle {x} is not exactly 90°/120°"
                );
            }
            let mut s2: Vec<f64> = squared_sides(poly).to_vec();
            s2.sort_by(f64::total_cmp);
            let want: Vec<f64> = vec![1.0 / 3.0, 1.0 / 3.0, long2, long2, 1.0];
            assert!(
                s2.iter().zip(&want).all(|(x, y)| (x - y).abs() < 1e-12),
                "k={k}: squared sides {s2:?}"
            );
            assert!(
                (poly.area() - area).abs() < 1e-12,
                "k={k}: area {} != 1/2 + √3/4",
                poly.area()
            );
        }
    }

    /// M101 pentagon-census audit (PROPOSED.md §5): the floret cell
    /// pentagons are type 5 in the repo lettering — all six share the
    /// shape (angles {60°, 120° ×4}; squared sides {1/3 ×3, 4/3 ×2};
    /// per-pentagon area 7√3/12, exact in Q(√3)), and the first cell
    /// satisfies the type-5 conditions at ccw shift k=3 (A = v3,
    /// B = v4, C = v0, D = v1, E = v2): B = 60°, E = 120°, a = b,
    /// d = e (repo letters shift the Wikipedia letters by one; side
    /// a joins A–B).
    #[test]
    fn floret_pentagonal_is_pentagon_type_5() {
        let t = laves::floret_pentagonal();
        let area = 7.0 * 3.0f64.sqrt() / 12.0; // 7√3/12
        for cell in &t.cells {
            let mut ang: Vec<f64> = interior_angles(&cell.poly).to_vec();
            ang.sort_by(f64::total_cmp);
            let want = [60.0, 120.0, 120.0, 120.0, 120.0];
            assert!(
                ang.iter().zip(&want).all(|(x, y)| (x - y).abs() < 1e-9),
                "angles {ang:?}"
            );
            let mut s2: Vec<f64> = squared_sides(&cell.poly).to_vec();
            s2.sort_by(f64::total_cmp);
            let want: Vec<f64> =
                vec![1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0, 4.0 / 3.0, 4.0 / 3.0];
            assert!(
                s2.iter().zip(&want).all(|(x, y)| (x - y).abs() < 1e-12),
                "squared sides {s2:?}"
            );
            assert!(
                (cell.poly.area() - area).abs() < 1e-12,
                "area {} != 7√3/12",
                cell.poly.area()
            );
        }
        // Type-5 lettering on the first cell, ccw shift k=3.
        let poly = &t.cells[0].poly;
        let ang = interior_angles(poly);
        assert!((ang[4] - 60.0).abs() < 1e-9, "B = v4: {}", ang[4]);
        assert!((ang[2] - 120.0).abs() < 1e-9, "E = v2: {}", ang[2]);
        let v = &poly.pts;
        let vtx = |i: usize| v[(i + 3) % 5]; // letter i (0 = A .. 4 = E)
        let side2 = |i: usize, j: usize| {
            let d = vtx(i) - vtx(j);
            d.dot(d)
        };
        assert!(
            (side2(0, 1) - side2(1, 2)).abs() < 1e-12,
            "a = b: {} vs {}",
            side2(0, 1),
            side2(1, 2)
        );
        assert!(
            (side2(3, 4) - side2(4, 0)).abs() < 1e-12,
            "d = e: {} vs {}",
            side2(3, 4),
            side2(4, 0)
        );
    }

    /// The Laves tables claim "pattern" mode is a proper coloring of the
    /// dual adjacency graph (the Python pre-verification searched for
    /// exactly that). Re-check it on a generated patch: every pair of
    /// tiles sharing an edge must differ in `seed % 8`.
    #[test]
    fn laves_pattern_mode_is_proper_coloring() {
        use std::collections::HashMap;
        let tilings: Vec<(&str, LatticeTiling)> = vec![
            ("tetrakis-square", laves::tetrakis_square()),
            ("rhombille", laves::rhombille()),
            ("triakis-triangular", laves::triakis_triangular()),
            ("deltoidal-trihexagonal", laves::deltoidal_trihexagonal()),
            ("kisrhombille", laves::kisrhombille()),
            ("prismatic-pentagonal", laves::prismatic_pentagonal()),
            ("floret-pentagonal", laves::floret_pentagonal()),
        ];
        let region = BBox::new(v2(-5.0, -4.0), v2(7.0, 6.0));
        for (name, lt) in &tilings {
            let p = ParamSet::default_for(lt.params());
            let r = lt.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{name}: {:?}", r.message);
            // Key each undirected edge by rounded endpoints (1e-6 grid,
            // matching the AGENTS.md edge-parity convention); shared edges
            // between lattice translates agree to ~1 ulp.
            let key = |a: Vec2, b: Vec2| {
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
            };
            let mut edges: HashMap<[i64; 4], Vec<u32>> = HashMap::new();
            for t in &r.tiles {
                let n = t.poly.pts.len();
                for w in 0..n {
                    let a = t.poly.pts[w];
                    let b = t.poly.pts[(w + 1) % n];
                    edges.entry(key(a, b)).or_default().push(t.seed % 8);
                }
            }
            for (k, seeds) in &edges {
                assert!(
                    seeds.len() <= 2,
                    "{name}: edge {k:?} shared by {} tiles (not edge-to-edge?)",
                    seeds.len()
                );
                if seeds.len() == 2 {
                    assert!(
                        seeds[0] != seeds[1],
                        "{name}: adjacent tiles share pattern color {} (edge {k:?})",
                        seeds[0]
                    );
                }
            }
        }
    }
}

