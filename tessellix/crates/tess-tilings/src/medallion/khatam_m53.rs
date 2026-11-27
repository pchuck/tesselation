//! M53 — the khatam 4.8.8 medallion: exact record + verification battery
//! (design doc §4.1).
//!
//! Hankin polygons-in-contact at contact angle δ = 45° on the truncated
//! square (4.8.8) substrate: the medallion is the unit-edge regular
//! octagon in the normalized record frame (doc §1.2), its contacts are
//! the eight edge midpoints, the figure is the {8/2} chord star
//! (contact i → contact i + 2), and the 16 exit rays are the two chord
//! continuations at every contact. Declared symmetry: 8 (rotation by 45°
//! about the centre).
//!
//! Ground truth (doc §4.1): the landed generators
//! [`pic_4_8_8()`] (whose partition the Python re-derivation matched by
//! EXACT face-vertex multisets — cited here, re-verified on the landed
//! side by its own M53 battery) and [`truncated_square()`] (the
//! substrate octagon the record hull must fit at 1e-9). The path-(a)
//! absorption certificate runs exactly here: every exit ray, continued
//! out of a medallion re-installed on the 4.8.8 network patch, lands on
//! a network contact midpoint, first-landing multiset
//! `{√2 − 1: 8, 1: 8}` (certified by the scratch battery `landing_a.py`).
//!
//! [`pic_4_8_8()`]: crate::families::islamic::pic_4_8_8::pic_4_8_8
//! [`truncated_square()`]: crate::families::uniform::archimedean::truncated_square

use super::*;
use std::cmp::Ordering;
use std::collections::HashSet;

/// Hull vertices `(a + b√2, c + d√2) / 2` in the normalized frame:
/// V0 = origin, edge 0 = (1, 0), CCW — the unit-edge regular octagon.
pub const HULL_TBL: [(i64, i64, i64, i64); 8] = [
    (0, 0, 0, 0),
    (2, 0, 0, 0),
    (2, 1, 0, 1),
    (2, 1, 2, 1),
    (2, 0, 2, 2),
    (0, 0, 2, 2),
    (0, -1, 2, 1),
    (0, -1, 0, 1),
];

/// The record's trait surface (doc §1).
pub struct KhatamM53;

impl Medallion for KhatamM53 {
    const ID: &'static str = "pic-4-8-8 khatam octagon (M53)";
    const FIELD: Field = Field::Qsqrt2;
    const SYM: usize = 8;
    const PATH: Absorption = Absorption::LandingIdentity;
    fn contacts_f64() -> Vec<([f64; 2], Option<(usize, f64)>)> {
        contacts()
            .iter()
            .enumerate()
            .map(|(i, p)| (super::qp_f64(*p), Some((i, 0.5))))
            .collect()
    }
    fn figure_f64() -> Vec<([f64; 2], [f64; 2])> {
        let m = contacts();
        figure()
            .iter()
            .map(|(i, j)| (super::qp_f64(m[*i]), super::qp_f64(m[*j])))
            .collect()
    }

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
        Some((1, 0, 2, 2)) // (1 + 0√2) / 2 = cos² 45°
    }
}

/// Field element of Q(√2) at denominator 1.
fn z(a: i64, b: i64) -> Zq {
    Zq::new(a, b, 2, 1)
}

/// Hull vertices, exact.
pub fn hull() -> Vec<Qp> {
    HULL_TBL
        .iter()
        .map(|&(a, b, c, d)| (Zq::new(a, b, 2, 2), Zq::new(c, d, 2, 2)))
        .collect()
}

/// The eight contacts: every hull edge midpoint (t = 1/2).
pub fn contacts() -> Vec<Qp> {
    midpoints(&hull())
}

/// The figure: the eight {8/2} chords as contact-index pairs.
pub fn figure() -> Vec<(usize, usize)> {
    (0..8).map(|i| (i, (i + 2) % 8)).collect()
}

/// The 16 directed exit rays `(contact, direction)`: both chord
/// continuations `M_i − M_{i±2}` at every contact.
pub fn exit_rays() -> Vec<(usize, Qp)> {
    let m = contacts();
    let mut out = Vec::with_capacity(16);
    for s in [2usize, 6] {
        for i in 0..8 {
            out.push((i, vsub(m[i], m[(i + s) % 8])));
        }
    }
    out
}

// ---------------------------------------------------------------- substrate
/// Lattice constant A = 1 + √2 of the unit-edge 4.8.8 substrate.
fn a_lattice() -> Zq {
    z(1, 1)
}

/// Axis-aligned regular octagon, unit edges, centre `c` (the substrate's
/// octagon placement `(i·A + A/2, j·A + A/2) + A·Z²`).
fn octagon(c: Qp) -> Vec<Qp> {
    let h = Zq::new(1, 1, 2, 2); // (1+√2)/2
    let g = Zq::new(1, 0, 2, 2); // 1/2
    let (cx, cy) = c;
    let (hp, hm) = (h, h.neg());
    let (gp, gm) = (g, g.neg());
    let mut v = Vec::with_capacity(8);
    for (dx, dy) in [
        (&gp, &hp),
        (&gm, &hp),
        (&hm, &gp),
        (&hm, &gm),
        (&gm, &hm),
        (&gp, &hm),
        (&hp, &gm),
        (&hp, &gp),
    ] {
        v.push((cx.add(*dx), cy.add(*dy)));
    }
    v
}

/// Substrate square rotated 45°, unit edges, centre c (vertices at
/// c ± √2/2 on the axes).
fn square45(c: Qp) -> Vec<Qp> {
    let r = Zq::new(0, 1, 2, 2); // √2/2
    let (cx, cy) = c;
    vec![
        (cx.add(r), cy),
        (cx, cy.add(r)),
        (cx.sub(r), cy),
        (cx, cy.sub(r)),
    ]
}

/// Canonical key of an exact point (for deduplication).
pub(crate) fn key6(p: Qp) -> (i64, i64, i64, i64, i64, i64) {
    (p.0.a, p.0.b, p.0.den, p.1.a, p.1.b, p.1.den)
}

/// All distinct edge midpoints of the 4.8.8 substrate patch
/// `[-patch, patch]²` of octagon/square index cells (the network's
/// contact set; 335 at patch = 3).
pub fn substrate_mids(patch: i64) -> Vec<Qp> {
    let a = a_lattice();
    let mut out: Vec<Qp> = Vec::new();
    let mut seen: HashSet<(i64, i64, i64, i64, i64, i64)> = HashSet::new();
    for i in -patch..=patch {
        for j in -patch..=patch {
            let ij = (z(i, 0).mul(a), z(j, 0).mul(a));
            // Octagon centres sit at ((2i + 1)·A/2, (2j + 1)·A/2): the
            // square lattice of octagon centres is offset from the square
            // centres (i·A, j·A) by half a lattice cell in each direction.
            let cen = (qhalf(z(2 * i + 1, 0).mul(a)), qhalf(z(2 * j + 1, 0).mul(a)));
            for poly in [octagon(cen), square45(ij)] {
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

/// Path-(a) landing certificate on the exact substrate patch: the
/// medallion is the patch-centre octagon (i = j = 0); every one of the
/// 16 exit rays carries further network midpoints ahead, and the first
/// landing parameter t (point `P + t·d`, in units of the chord vector)
/// is exactly `√2 − 1` for 8 rays and `1` for 8. Returns
/// `(t = √2−1 count, t = 1 count, rays without any midpoint ahead)`.
pub fn landings(patch: i64) -> (usize, usize, usize) {
    let mids = substrate_mids(patch);
    let a = a_lattice();
    let m = midpoints(&octagon((qhalf(a), qhalf(a))));
    let (mut t_sqrt, mut t_one, mut fails) = (0, 0, 0);
    for s in [2usize, 6] {
        for i in 0..8 {
            let p = m[i];
            let d = vsub(p, m[(i + s) % 8]);
            let nd = norm2(d);
            assert!(nd.eq_x(Zq::new(3, 2, 2, 2)), "chord length²");
            let mut best: Option<Zq> = None; // dot(d, q − p), min > 0
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
                    } else if b.eq_x(z(-1, 1).mul(nd)) {
                        t_sqrt += 1;
                    } else {
                        panic!("unexpected first-landing t {:?}", b);
                    }
                }
            }
        }
    }
    (t_sqrt, t_one, fails)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_schema_invariants() {
        let h = hull();
        assert!(strictly_ccw(&h), "hull strictly convex + CCW");
        assert!(equal_edges(&h), "equal hull edges");
        let e0 = norm2(vsub(h[1], h[0]));
        assert!(e0.eq_x(Zq::whole(2, 1)), "unit edges");
        assert!(normalized_frame(&h, Zq::whole(2, 1)), "normalized frame");
        // area2 of the unit-edge regular octagon = 4 + 4√2.
        assert!(area2(&h).eq_x(Zq::whole(2, 4).add(z(0, 4))), "doubled area");
        let m = contacts();
        let fig = figure();
        assert_eq!(fig.len(), 8);
        // every contact is a figure endpoint (twice), the figure spans
        // all 8 contacts.
        for i in 0..8 {
            let n = fig.iter().filter(|(a, b)| *a == i || *b == i).count();
            assert_eq!(n, 2, "contact {i} figure degree");
        }
        // exits: 16, two per contact, exactly the chord continuations.
        let ex = exit_rays();
        assert_eq!(ex.len(), 16);
        for i in 0..8 {
            assert_eq!(ex.iter().filter(|(c, _)| *c == i).count(), 2);
        }
        for s in [2usize, 6] {
            for i in 0..8 {
                let want = vsub(m[i], m[(i + s) % 8]);
                assert!(
                    ex.iter().any(|(c, d)| *c == i && pt_eq(*d, want)),
                    "exit continuation at contact {i}"
                );
            }
        }
        // exits outward: dot(direction, contact − centre) > 0.
        let s = vertex_sum(&h);
        let c = (div_i(s.0, 8), div_i(s.1, 8));
        for (i, d) in &ex {
            let w = vsub(m[*i], c);
            assert_eq!(dot(*d, w).sgn(), Ordering::Greater, "outward");
        }
    }

    #[test]
    fn contact_angle_is_exactly_45_degrees() {
        let h = hull();
        let m = contacts();
        // cos²(chord, edge) = 1/2 ⟺ 2·dot² = |chord|²·|edge|², exact.
        for (a, b) in figure() {
            let d = vsub(m[b], m[a]);
            let e = vsub(h[(a + 1) % 8], h[a]);
            let lhs = z(2, 0).mul(dot(d, e).mul(dot(d, e)));
            let rhs = norm2(d).mul(norm2(e));
            assert!(lhs.eq_x(rhs), "contact {a}: cos² = 1/2");
        }
    }

    #[test]
    fn exit_direction_census() {
        // cos² with +x over the 16 exits: {0: 4, 1: 4, 1/2: 8}; all
        // exits share the single chord length² 3/2 + √2.
        let ex = exit_rays();
        let (mut n0, mut n1, mut nh) = (0, 0, 0);
        for (_, u) in &ex {
            let n = norm2(*u);
            assert!(n.eq_x(Zq::new(3, 2, 2, 2)), "chord length²");
            if u.0.sgn() == Ordering::Equal {
                n0 += 1;
            } else if u.1.sgn() == Ordering::Equal {
                n1 += 1;
            } else if u.0.mul(u.0).eq_x(u.1.mul(u.1)) {
                nh += 1;
            } else {
                panic!("unexpected exit direction");
            }
        }
        assert_eq!((n0, n1, nh), (4, 4, 8), "exit cos² census");
    }

    #[test]
    fn sym8_rotation_maps_record_to_itself() {
        // exact 45° rotation: R(x, y) = ((x − y)√2/2, (x + y)√2/2).
        let r2 = Zq::new(0, 1, 2, 2);
        let r45 = |p: Qp| (p.0.sub(p.1).mul(r2), p.0.add(p.1).mul(r2));
        let h = hull();
        let s = vertex_sum(&h);
        let c = (div_i(s.0, 8), div_i(s.1, 8));
        let rot_about = |p: Qp| vadd(r45(vsub(p, c)), c);
        // find the vertex offset of R(V0): it must map V_i → V_{i+m} ∀i.
        let w0 = rot_about(h[0]);
        let m = (0..8)
            .find(|&m| pt_eq(w0, h[m]))
            .expect("V0 image is a vertex");
        for i in 0..8 {
            assert!(pt_eq(rot_about(h[i]), h[(i + m) % 8]), "sym8 on vertex {i}");
        }
        // midpoints (contacts) and exit-ray directions follow.
        let mm = contacts();
        for i in 0..8 {
            assert!(pt_eq(rot_about(mm[i]), mm[(i + m) % 8]));
        }
        let mut got: Vec<_> = exit_rays()
            .into_iter()
            .map(|(i, d)| ((i + m) % 8, r45(d)))
            .collect();
        let mut want: Vec<_> = exit_rays();
        let sort = |v: &mut Vec<(usize, Qp)>| {
            v.sort_by_key(|(i, d)| (*i, key6(*d)));
        };
        sort(&mut got);
        sort(&mut want);
        assert!(
            got.iter()
                .zip(&want)
                .all(|((i, a), (j, b))| i == j && pt_eq(*a, *b)),
            "sym8 on the exit-ray set"
        );
    }

    #[test]
    fn landing_identity_on_substrate() {
        // Patch 3 is the certified minimum: a patch-2 (boundary-row)
        // medallion has rays whose first landing falls outside the patch.
        let (t_sqrt, t_one, fails) = landings(3);
        assert_eq!(fails, 0, "no ray escapes without a landing");
        assert_eq!(t_sqrt, 8, "t = √2 − 1");
        assert_eq!(t_one, 8, "t = 1");
        assert_eq!(substrate_mids(3).len(), 335, "network midpoint count");
    }

    #[test]
    fn hull_matches_landed_truncated_square_octagon() {
        use tess_core::{BBox, ParamSet, Query, Tiling, Vec2};
        let lt = crate::families::uniform::archimedean::truncated_square();
        let params = ParamSet::default_for(lt.params());
        let region = BBox::new(Vec2::new(-4.0, -4.0), Vec2::new(12.0, 12.0));
        let res = lt.generate(&params, &Query { region });
        assert!(res.message.is_none(), "{:?}", res.message);
        let octs: Vec<_> = res.tiles.iter().filter(|t| t.poly.len() == 8).collect();
        assert!(!octs.is_empty(), "the 4.8.8 tiling emits octagons");
        let h = hull();
        let hs = vertex_sum(&h);
        let hc = [hs.0.fl() / 8.0, hs.1.fl() / 8.0];
        let hp: Vec<[f64; 2]> = h
            .iter()
            .map(|p| [p.0.fl() - hc[0], p.1.fl() - hc[1]])
            .collect();
        let oct = &octs[0];
        let n = oct.poly.len();
        let mut tc = [0.0; 2];
        for p in &oct.poly.pts {
            tc[0] += p.x / n as f64;
            tc[1] += p.y / n as f64;
        }
        // unit edges in the landed data
        for k in 0..n {
            let a = oct.poly.pts[k];
            let b = oct.poly.pts[(k + 1) % n];
            let e2 = (b.x - a.x) * (b.x - a.x) + (b.y - a.y) * (b.y - a.y);
            assert!((e2 - 1.0).abs() < 1e-12, "landed octagon edges");
        }
        // congruence: rotation by a multiple of 45° + cyclic alignment.
        let mut found = false;
        for rot in 0..8 {
            let th = rot as f64 * std::f64::consts::FRAC_PI_4;
            let (cs, sn) = (th.cos(), th.sin());
            for shift in 0..8 {
                let ok = (0..8).all(|k| {
                    let q = hp[(shift + k) % 8];
                    let x = cs * q[0] - sn * q[1] + tc[0];
                    let y = sn * q[0] + cs * q[1] + tc[1];
                    let p = oct.poly.pts[k];
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
        assert!(found, "record hull fits a landed octagon at 1e-9");
    }
}
