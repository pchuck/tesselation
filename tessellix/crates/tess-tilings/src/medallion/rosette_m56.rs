//! M56 — the {12/4} twelvefold rosette medallion (pic-3-12-12 core):
//! exact record + verification battery (design doc §4.2).
//!
//! Hankin polygons-in-contact at contact angle δ = 60° on the
//! truncated-hexagonal 3.12.12 substrate: the medallion is the
//! unit-edge regular dodecagon in the normalized record frame, its
//! contacts are the twelve edge midpoints, the figure is the {12/4}
//! star (contact i → contact i + 4), and the 24 exit rays are the two
//! chord continuations at every contact. Declared symmetry: 12.
//!
//! Ground truth (doc §4.2): the landed partition in
//! [`pic_3_12_12()`] (the Python re-derivation matched it by EXACT
//! translation-invariant face-vertex multisets; the landed side carries
//! its own M56 battery), and the substrate dodecagon of
//! [`truncated_hexagonal()`] which the record hull must fit at 1e-9.
//! The path-(a) certificate runs exactly here on the 3.12.12 network
//! patch: first-landing multiset `{2√3/3 − 1: 12, 1: 12}` (scratch
//! battery `landing_a.py`).
//!
//! [`pic_3_12_12()`]: crate::families::islamic::pic_3_12_12::pic_3_12_12
//! [`truncated_hexagonal()`]: crate::families::uniform::archimedean::truncated_hexagonal

use super::*;
use std::cmp::Ordering;
use std::collections::HashSet;

use crate::medallion::khatam_m53::key6;

/// Hull vertices `(a + b√3, c + d√3) / 4`, normalized frame: V0 =
/// origin, edge 0 = (1, 0), CCW — the unit-edge regular dodecagon.
pub const HULL_TBL: [(i64, i64, i64, i64); 12] = [
    (0, 0, 0, 0),
    (4, 0, 0, 0),
    (4, 2, 2, 0),
    (6, 2, 2, 2),
    (6, 2, 6, 2),
    (4, 2, 6, 4),
    (4, 0, 8, 4),
    (0, 0, 8, 4),
    (0, -2, 6, 4),
    (-2, -2, 6, 2),
    (-2, -2, 2, 2),
    (0, -2, 2, 0),
];

/// The record's trait surface.
pub struct RosetteM56;

impl Medallion for RosetteM56 {
    const ID: &'static str = "pic-3-12-12 {12/4} rosette (M56)";
    const FIELD: Field = Field::Qsqrt3;
    const SYM: usize = 12;
    const PATH: Absorption = Absorption::LandingIdentity;

    fn hull_f64() -> Vec<[f64; 2]> {
        hull().iter().map(|p| [p.0.fl(), p.1.fl()]).collect()
    }
    fn exit_rays_f64() -> Vec<(usize, [f64; 2])> {
        exit_rays()
            .into_iter()
            .map(|(i, d)| (i, [d.0.fl(), d.1.fl()]))
            .collect()
    }
    fn contact_cos2() -> Option<(i64, i64, i64, i64)> {
        Some((1, 0, 3, 4)) // cos² 60°
    }
}

/// Field element of Q(√3) at denominator 1 (battery helper).
#[cfg(test)]
fn z(a: i64, b: i64) -> Zq {
    Zq::new(a, b, 3, 1)
}

/// Hull vertices, exact.
pub fn hull() -> Vec<Qp> {
    HULL_TBL
        .iter()
        .map(|&(a, b, c, d)| (Zq::new(a, b, 3, 4), Zq::new(c, d, 3, 4)))
        .collect()
}

/// The twelve contacts: every hull edge midpoint (t = 1/2).
pub fn contacts() -> Vec<Qp> {
    midpoints(&hull())
}

/// The figure: the twelve {12/4} chords as contact-index pairs.
pub fn figure() -> Vec<(usize, usize)> {
    (0..12).map(|i| (i, (i + 4) % 12)).collect()
}

/// The 24 directed exit rays `(contact, direction)`.
pub fn exit_rays() -> Vec<(usize, Qp)> {
    let m = contacts();
    let mut out = Vec::with_capacity(24);
    for s in [4usize, 8] {
        for i in 0..12 {
            out.push((i, vsub(m[i], m[(i + s) % 12])));
        }
    }
    out
}

// ---------------------------------------------------------------- substrate
// The exact 3.12.12 substrate of the re-derivation: dodecagons with
// edge k centred on direction 30k at centres D0 + i·A1 + j·A2, and
// unit triangles in the six hole directions (deduplicated).
fn apothem() -> Zq {
    Zq::new(2, 1, 3, 2) // AP = (2+√3)/2
}
fn a1() -> Qp {
    (Zq::new(2, 1, 3, 2), Zq::new(3, 2, 3, 2)) // 2·AP·e^{i60}
}
fn a2() -> Qp {
    (Zq::new(-2, -1, 3, 2), Zq::new(3, 2, 3, 2)) // 2·AP·e^{i120}
}
fn t_off() -> Zq {
    Zq::new(3, 2, 3, 3) // AP + √3/6 = 1 + 2√3/3
}
fn d0() -> Qp {
    (Zq::zero(3), t_off().neg())
}

/// Dodecagon with vertex k at angle −15° + 30k, circumradius² 2 + √3.
pub fn dodecagon(c: Qp) -> Vec<Qp> {
    let base = (apothem(), Zq::new(-1, 0, 3, 2));
    (0..12).map(|k| vadd(rot30n(base, k), c)).collect()
}

/// Unit triangle in hole direction 90° + 60k of the dodecagon at c.
fn triangle(c: Qp, k: usize) -> Vec<Qp> {
    let t = vadd(c, rot30n((Zq::zero(3), t_off()), 2 * k));
    let base = [
        (Zq::new(-1, 0, 3, 2), Zq::new(0, -1, 3, 6)), // (−1/2, −√3/6)
        (Zq::new(1, 0, 3, 2), Zq::new(0, -1, 3, 6)),  // (1/2, −√3/6)
        (Zq::zero(3), Zq::new(0, 1, 3, 3)),           // (0, √3/3)
    ];
    base.iter().map(|b| vadd(t, rot30n(*b, 2 * k))).collect()
}

/// All distinct edge midpoints of the 3.12.12 substrate patch (the
/// network's contact set; 552 at patch = 3).
pub fn substrate_mids(patch: i64) -> Vec<Qp> {
    let mut out: Vec<Qp> = Vec::new();
    let mut seen: HashSet<(i64, i64, i64, i64, i64, i64)> = HashSet::new();
    let mut tri_seen: HashSet<[(i64, i64, i64, i64, i64, i64); 3]> = HashSet::new();
    let (a1v, a2v, c0) = (a1(), a2(), d0());
    for i in -patch..=patch {
        for j in -patch..=patch {
            let c = (
                c0.0.add(a1v.0.scale(i)).add(a2v.0.scale(j)),
                c0.1.add(a1v.1.scale(i)).add(a2v.1.scale(j)),
            );
            let mut tiles: Vec<Vec<Qp>> = vec![dodecagon(c)];
            for k in 0..6 {
                let tri = triangle(c, k);
                let mut key3 = [key6(tri[0]), key6(tri[1]), key6(tri[2])];
                key3.sort();
                if tri_seen.insert(key3) {
                    tiles.push(tri);
                }
            }
            for poly in tiles {
                for m in midpoints(&poly) {
                    if seen.insert(key6(m)) {
                        out.push(m);
                    }
                }
            }
        }
    }
    out
}

/// Path-(a) landing certificate (patch-3 medallion = dodecagon at D0):
/// first-landing multiset `{2√3/3 − 1: 12, 1: 12}`, no misses.
pub fn landings(patch: i64) -> (usize, usize, usize) {
    let mids = substrate_mids(patch);
    let m = midpoints(&dodecagon(d0()));
    let (mut t_near, mut t_one, mut fails) = (0, 0, 0);
    for s in [4usize, 8] {
        for i in 0..12 {
            let p = m[i];
            let d = vsub(p, m[(i + s) % 12]);
            let nd = norm2(d);
            // |chord|² = 21/4 + 3√3, single-valued.
            assert!(nd.eq_x(Zq::new(21, 12, 3, 4)), "chord length²");
            let mut best: Option<Zq> = None;
            for q in &mids {
                if key6(*q) == key6(p) {
                    continue;
                }
                let w = vsub(*q, p);
                if cross(d, w).sgn() != Ordering::Equal {
                    continue;
                }
                let tnum = dot(d, w);
                if tnum.sgn() != Ordering::Greater {
                    continue;
                }
                let better = match best {
                    None => true,
                    Some(b) => tnum.sub(b).sgn() == Ordering::Less,
                };
                if better {
                    best = Some(tnum);
                }
            }
            match best {
                None => fails += 1,
                Some(b) => {
                    if b.eq_x(nd) {
                        t_one += 1;
                    } else if b.eq_x(Zq::new(-3, 2, 3, 3).mul(nd)) {
                        t_near += 1;
                    } else {
                        panic!("unexpected first-landing t {:?}", b);
                    }
                }
            }
        }
    }
    (t_near, t_one, fails)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_schema_invariants() {
        let h = hull();
        assert!(strictly_ccw(&h));
        assert!(equal_edges(&h));
        assert!(norm2(vsub(h[1], h[0])).eq_x(Zq::whole(3, 1)), "unit edges");
        assert!(normalized_frame(&h, Zq::whole(3, 1)));
        // doubled area of the unit dodecagon: 12 + 6√3.
        assert!(
            area2(&h).eq_x(Zq::whole(3, 12).add(z(0, 6))),
            "doubled area"
        );
        let m = contacts();
        let fig = figure();
        assert_eq!(fig.len(), 12);
        for i in 0..12 {
            let n = fig.iter().filter(|(a, b)| *a == i || *b == i).count();
            assert_eq!(n, 2, "contact {i} figure degree");
        }
        let ex = exit_rays();
        assert_eq!(ex.len(), 24);
        for i in 0..12 {
            assert_eq!(ex.iter().filter(|(c, _)| *c == i).count(), 2);
        }
        for s in [4usize, 8] {
            for i in 0..12 {
                let want = vsub(m[i], m[(i + s) % 12]);
                assert!(
                    ex.iter().any(|(c, d)| *c == i && pt_eq(*d, want)),
                    "exit continuation at contact {i}"
                );
            }
        }
        let s = vertex_sum(&h);
        let c = (div_i(s.0, 12), div_i(s.1, 12));
        for (i, d) in &ex {
            let w = vsub(m[*i], c);
            assert_eq!(dot(*d, w).sgn(), Ordering::Greater, "outward");
        }
    }

    #[test]
    fn contact_angle_is_exactly_60_degrees() {
        let h = hull();
        let m = contacts();
        // cos² = 1/4 ⟺ 4·dot² = |chord|²·|edge|².
        for (a, b) in figure() {
            let d = vsub(m[b], m[a]);
            let e = vsub(h[(a + 1) % 12], h[a]);
            let lhs = z(4, 0).mul(dot(d, e).mul(dot(d, e)));
            let rhs = norm2(d).mul(norm2(e));
            assert!(lhs.eq_x(rhs), "contact {a}: cos² = 1/4");
        }
    }

    #[test]
    fn exit_direction_census() {
        // cos² with +x over the 24 exits: {0: 4, 1: 4, 1/4: 8, 3/4: 8}.
        let ex = exit_rays();
        let (mut n0, mut n1, mut nq, mut nt) = (0, 0, 0, 0);
        for (_, u) in &ex {
            let n = norm2(*u);
            assert!(n.eq_x(Zq::new(21, 12, 3, 4)), "chord length²");
            if u.0.sgn() == Ordering::Equal {
                n0 += 1;
            } else if u.1.sgn() == Ordering::Equal {
                n1 += 1;
            } else if u.0.mul(u.0).scale(4).eq_x(n) {
                nq += 1; // cos² = 1/4
            } else if u.1.mul(u.1).scale(4).eq_x(n) {
                nt += 1; // cos² = 3/4
            } else {
                panic!("unexpected exit direction");
            }
        }
        assert_eq!((n0, n1, nq, nt), (4, 4, 8, 8), "exit cos² census");
    }

    #[test]
    fn sym12_rotation_maps_record_to_itself() {
        let h = hull();
        let s = vertex_sum(&h);
        let c = (div_i(s.0, 12), div_i(s.1, 12));
        let rot_about = |p: Qp| vadd(rot30(vsub(p, c)), c);
        let w0 = rot_about(h[0]);
        let m = (0..12)
            .find(|&m| pt_eq(w0, h[m]))
            .expect("V0 image is a vertex");
        for i in 0..12 {
            assert!(pt_eq(rot_about(h[i]), h[(i + m) % 12]), "sym12 vertex {i}");
        }
        let mm = contacts();
        for i in 0..12 {
            assert!(pt_eq(rot_about(mm[i]), mm[(i + m) % 12]));
        }
        let mut got: Vec<_> = exit_rays()
            .into_iter()
            .map(|(i, d)| ((i + m) % 12, rot30(d)))
            .collect();
        let mut want: Vec<_> = exit_rays();
        let sort = |v: &mut Vec<(usize, Qp)>| {
            v.sort_by_key(|(i, d)| (*i, key6(*d)));
        };
        sort(&mut got);
        sort(&mut want);
        assert!(got
            .iter()
            .zip(&want)
            .all(|((i, a), (j, b))| i == j && pt_eq(*a, *b)));
    }

    #[test]
    fn landing_identity_on_substrate() {
        let (t_near, t_one, fails) = landings(3);
        assert_eq!(fails, 0);
        assert_eq!(t_near, 12, "t = 2√3/3 − 1");
        assert_eq!(t_one, 12, "t = 1");
        assert_eq!(substrate_mids(3).len(), 552, "network midpoint count");
    }

    #[test]
    fn hull_matches_landed_truncated_hexagonal_dodecagon() {
        use tess_core::{BBox, ParamSet, Query, Tiling, Vec2};
        let lt = crate::families::uniform::archimedean::truncated_hexagonal();
        let params = ParamSet::default_for(lt.params());
        let region = BBox::new(Vec2::new(-6.0, -6.0), Vec2::new(6.0, 6.0));
        let res = lt.generate(&params, &Query { region });
        assert!(res.message.is_none(), "{:?}", res.message);
        let dodecs: Vec<_> = res.tiles.iter().filter(|t| t.poly.len() == 12).collect();
        assert!(!dodecs.is_empty(), "3.12.12 emits dodecagons");
        let h = hull();
        let hs = vertex_sum(&h);
        let hc = [hs.0.fl() / 12.0, hs.1.fl() / 12.0];
        let hp: Vec<[f64; 2]> = h
            .iter()
            .map(|p| [p.0.fl() - hc[0], p.1.fl() - hc[1]])
            .collect();
        let dod = &dodecs[0];
        let n = dod.poly.len();
        let mut tc = [0.0; 2];
        for p in &dod.poly.pts {
            tc[0] += p.x / n as f64;
            tc[1] += p.y / n as f64;
        }
        // unit edges and circumradius² 2 + √3 in the landed tile.
        let r2 = 2.0 + 3f64.sqrt();
        for k in 0..n {
            let a = dod.poly.pts[k];
            let b = dod.poly.pts[(k + 1) % n];
            let e2 = (b.x - a.x) * (b.x - a.x) + (b.y - a.y) * (b.y - a.y);
            assert!((e2 - 1.0).abs() < 1e-12, "landed dodecagon edges");
            let rr = (a.x - tc[0]) * (a.x - tc[0]) + (a.y - tc[1]) * (a.y - tc[1]);
            assert!((rr - r2).abs() < 1e-9, "landed circumradius²");
        }
        let mut found = false;
        for rot in 0..12 {
            let th = rot as f64 * std::f64::consts::PI / 6.0;
            let (cs, sn) = (th.cos(), th.sin());
            for shift in 0..12 {
                let ok = (0..12).all(|k| {
                    let q = hp[(shift + k) % 12];
                    let x = cs * q[0] - sn * q[1] + tc[0];
                    let y = sn * q[0] + cs * q[1] + tc[1];
                    let p = dod.poly.pts[k];
                    (x - p.x).abs() < 1e-9 && (y - p.y).abs() < 1e-9
                });
                if ok {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        assert!(found, "record hull fits a landed dodecagon at 1e-9");
    }
}
