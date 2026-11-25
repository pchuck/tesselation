//! The {6/2} hexagram medallion at the kissing radius T = 1/2 — exact
//! record + verification battery (design doc §4.4).
//!
//! Interpretation (doc §4.4): the medallion is the exact landed
//! hexagram family at the kissing parameter, NOT the measured M35
//! artwork table. Hull = the tip hexagon (unit of the record: tips at
//! radius 1/2, hull edges 1/2), corner contacts at the six tips, figure
//! = the two superposed equilateral triangles {0,2,4} + {1,3,5} whose
//! overlay is the hexagram, and the 12 exit rays are the chord
//! continuations at the tips (directions 30° + 60k). Declared symmetry: 6.
//!
//! Ground truth (doc §4.4): the landed `hexagram` family
//! ([`HexagramTiling`], default `star_radius` = 1/2 = kissing). The
//! path-(a) certificate here is the chord-line landing identity — every
//! chord line of the star at the origin is again a chord line of a
//! translated star (six-direction line grid), certified exactly in
//! Q(√3) — plus the exact kissing structure (tips at lattice-edge
//! midpoints) and the tangent identity (the tip-0 exit lines sit at the
//! chord distance 1/4 from the neighbouring star centre).
//!
//! [`HexagramTiling`]: crate::families::islamic::hexagram::HexagramTiling

use super::*;

/// Hull vertices `(a + b√3, c + d√3) / 4`, normalized frame: V0 =
/// origin, edge 0 = (1/2, 0), CCW — the tip hexagon.
pub const HULL_TBL: [(i64, i64, i64, i64); 6] = [
    (0, 0, 0, 0),
    (2, 0, 0, 0),
    (3, 0, 0, 1),
    (2, 0, 0, 2),
    (0, 0, 0, 2),
    (-1, 0, 0, 1),
];

/// The figure as hull-vertex index pairs: triangles {0,2,4} and {1,3,5}.
pub const FIGURE: [(usize, usize); 6] = [(0, 2), (2, 4), (4, 0), (1, 3), (3, 5), (5, 1)];

/// The record's trait surface.
pub struct Hexagram62;

impl Medallion for Hexagram62 {
    const ID: &'static str = "{6/2} hexagram, kissing T = 1/2";
    const FIELD: Field = Field::Qsqrt3;
    const SYM: usize = 6;
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
    // Corner-contact record: the angle datum is the exit-ray multiset.
}

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

/// The figure as exact segments (endpoints are the corner contacts).
pub fn figure() -> Vec<(Qp, Qp)> {
    let h = hull();
    FIGURE.iter().map(|&(a, b)| (h[a], h[b])).collect()
}

/// The 12 directed exit rays `(corner index, direction)`: at each tip,
/// the continuations of the two chord sides arriving there.
pub fn exit_rays() -> Vec<(usize, Qp)> {
    let h = hull();
    let mut out = Vec::with_capacity(12);
    for k in 0..6 {
        for (a, b) in FIGURE {
            if b == k {
                out.push((k, vsub(h[b], h[a])));
            }
            if a == k {
                out.push((k, vsub(h[a], h[b])));
            }
        }
    }
    out
}

// ---------------------------------------------------------------- natural frame
/// Exact unit direction at angle 60k.
fn dir60(k: usize) -> Qp {
    match k % 6 {
        0 => (z(1, 0), z(0, 0)),
        1 => (Zq::new(1, 0, 3, 2), Zq::new(0, 1, 3, 2)),
        2 => (Zq::new(-1, 0, 3, 2), Zq::new(0, 1, 3, 2)),
        3 => (z(-1, 0), z(0, 0)),
        4 => (Zq::new(-1, 0, 3, 2), Zq::new(0, -1, 3, 2)),
        _ => (Zq::new(1, 0, 3, 2), Zq::new(0, -1, 3, 2)),
    }
}

/// Exact unit direction at angle 30° + 60k (the exit-ray grid).
pub fn dir30(k: usize) -> Qp {
    match k % 6 {
        0 => (Zq::new(0, 1, 3, 2), Zq::new(1, 0, 3, 2)),
        1 => (z(0, 0), z(1, 0)),
        2 => (Zq::new(0, -1, 3, 2), Zq::new(1, 0, 3, 2)),
        3 => (Zq::new(0, -1, 3, 2), Zq::new(-1, 0, 3, 2)),
        4 => (z(0, 0), z(-1, 0)),
        _ => (Zq::new(0, 1, 3, 2), Zq::new(-1, 0, 3, 2)),
    }
}

/// Star tips at the kissing radius T = 1/2 (natural frame, angle 60k).
pub fn tips_natural() -> Vec<Qp> {
    let t = Zq::new(1, 0, 3, 2); // T = 1/2
    (0..6).map(|k| vmul(dir60(k), t)).collect()
}

/// Lattice basis of the landed hexagram tiling: a1 = (1, 0),
/// a2 = (1/2, √3/2).
pub fn lattice() -> (Qp, Qp) {
    (
        (z(1, 0), z(0, 0)),
        (Zq::new(1, 0, 3, 2), Zq::new(0, 1, 3, 2)),
    )
}

/// Chord segments (the two triangles) in the NATURAL frame (test helper).
#[cfg(test)]
fn segs_natural() -> Vec<(Qp, Qp)> {
    let t = tips_natural();
    vec![
        (t[0], t[2]),
        (t[2], t[4]),
        (t[4], t[0]),
        (t[1], t[3]),
        (t[3], t[5]),
        (t[5], t[1]),
    ]
}

/// Line invariant (direction, cross(a, direction)): two segments are on
/// the same line iff directions are parallel and c²/|d|² agrees.
#[cfg(test)]
fn same_line(a: &(Qp, Qp), b: &(Qp, Qp)) -> bool {
    let d1 = vsub(a.1, a.0);
    let d2 = vsub(b.1, b.0);
    if cross(d1, d2).sgn() != Ordering::Equal {
        return false;
    }
    let c1 = cross(a.0, d1);
    let c2 = cross(b.0, d2);
    c1.mul(c1).mul(norm2(d2)).eq_x(c2.mul(c2).mul(norm2(d1)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::medallion::khatam_m53::key6;
    use std::cmp::Ordering;

    #[test]
    fn record_schema_invariants() {
        let h = hull();
        assert!(strictly_ccw(&h));
        assert!(equal_edges(&h));
        // edgesq = 1/4, edge 0 = (1/2, 0).
        assert!(
            norm2(vsub(h[1], h[0])).eq_x(Zq::new(1, 0, 3, 4)),
            "edgesq 1/4"
        );
        assert!(normalized_frame(&h, Zq::new(1, 0, 3, 2)));
        // doubled area of the regular hexagon with edge 1/2: 3√3/4.
        assert!(area2(&h).eq_x(Zq::new(0, 3, 3, 4)), "doubled area");
        // corner contacts = all six tips, each a figure endpoint twice.
        for k in 0..6 {
            let n = FIGURE.iter().filter(|(a, b)| *a == k || *b == k).count();
            assert_eq!(n, 2, "tip {k} figure degree");
        }
        let ex = exit_rays();
        assert_eq!(ex.len(), 12);
        for k in 0..6 {
            assert_eq!(ex.iter().filter(|(c, _)| *c == k).count(), 2);
        }
        // exits are exactly the directed chord continuations.
        let fig = figure();
        for (a, b) in &fig {
            let ka = (0..6).find(|&k| pt_eq(h[k], *a)).unwrap();
            let kb = (0..6).find(|&k| pt_eq(h[k], *b)).unwrap();
            assert!(ex.iter().any(|(c, d)| *c == kb && pt_eq(*d, vsub(*b, *a))));
            assert!(ex.iter().any(|(c, d)| *c == ka && pt_eq(*d, vsub(*a, *b))));
        }
        // outward.
        let s = vertex_sum(&h);
        let c = (div_i(s.0, 6), div_i(s.1, 6));
        for (k, d) in &ex {
            let w = vsub(h[*k], c);
            assert_eq!(dot(*d, w).sgn(), Ordering::Greater, "outward");
        }
    }

    #[test]
    fn exit_directions_are_the_30_plus_60k_grid() {
        // Every exit direction is parallel to exactly one of the 12 unit
        // grid directions, and the cos²-with-x census is {0: 4, 3/4: 8}.
        let ex = exit_rays();
        let mut grid: Vec<Qp> = Vec::new();
        for k in 0..6 {
            grid.push(dir60(k));
            grid.push(dir30(k));
        }
        let mut n0 = 0;
        let mut n34 = 0;
        for (_, u) in &ex {
            let hits: Vec<usize> = grid
                .iter()
                .enumerate()
                .filter(|(_, w)| cross(*u, **w).sgn() == Ordering::Equal)
                .map(|(j, _)| j)
                .collect();
            assert_eq!(hits.len(), 2, "exit direction lies on one grid line");
            let n = norm2(*u);
            if u.0.sgn() == Ordering::Equal {
                n0 += 1;
            } else if u.1.mul(u.1).scale(4).eq_x(n) {
                n34 += 1;
            } else {
                panic!("unexpected exit direction");
            }
        }
        assert_eq!((n0, n34), (4, 8), "cos^2 census {{0: 4, 3/4: 8}}");
    }

    #[test]
    fn kissing_tips_sit_on_lattice_edge_midpoints() {
        let t = tips_natural();
        let (a1, a2) = lattice();
        let h = qhalf;
        let mids = [
            (h(a1.0), h(a1.1)),
            (h(a2.0), h(a2.1)),
            (h(vsub(a2, a1).0), h(vsub(a2, a1).1)),
            (h(a1.0.neg()), h(a1.1.neg())),
            (h(a2.0.neg()), h(a2.1.neg())),
            (h(vsub(a1, a2).0), h(vsub(a1, a2).1)),
        ];
        for k in 0..6 {
            assert!(pt_eq(t[k], mids[k]), "tip {k} = lattice-edge midpoint");
        }
    }

    #[test]
    fn record_frame_is_the_rotated_natural_frame() {
        // hn = rot30⁸(tips) − rot30⁸(tip0): pins the normalization
        // convention exactly.
        let t = tips_natural();
        let hn0 = rot30n(t[0], 8);
        for k in 0..6 {
            let hn = vsub(rot30n(t[k], 8), hn0);
            assert!(pt_eq(hn, hull()[k]), "hull vertex {k}");
        }
        // figure chords and exit-ray directions follow the map.
        let sn = segs_natural();
        let fig = figure();
        for (a, b) in &sn {
            let ra = vsub(rot30n(*a, 8), hn0);
            let rb = vsub(rot30n(*b, 8), hn0);
            assert!(
                fig.iter()
                    .any(|(x, y)| (pt_eq(*x, ra) && pt_eq(*y, rb))
                        || (pt_eq(*x, rb) && pt_eq(*y, ra))),
                "chord maps"
            );
        }
        let ex = exit_rays();
        for (a, b) in &sn {
            let d = vsub(*b, *a);
            let rd = rot30n(d, 8);
            let kb = (0..6)
                .find(|&k| pt_eq(hull()[k], vsub(rot30n(*b, 8), hn0)))
                .unwrap();
            assert!(
                ex.iter().any(|(c, u)| *c == kb && pt_eq(*u, rd)),
                "exit ray maps"
            );
        }
    }

    #[test]
    fn chord_line_landing_identity() {
        // Every chord line of the origin star is a chord line of a star
        // translated by a lattice vector (path (a), M163 pattern).
        let sn = segs_natural();
        let (a1, a2) = lattice();
        let d = vsub(a2, a1);
        let trans = [a1, a2, d, vscale(a1, -1), vscale(a2, -1), vscale(d, -1)];
        for seg in &sn {
            let hits: Vec<usize> = trans
                .iter()
                .enumerate()
                .filter(|(_, t)| {
                    sn.iter()
                        .any(|s| same_line(seg, &(vadd(s.0, **t), vadd(s.1, **t))))
                })
                .map(|(j, _)| j)
                .collect();
            assert!(!hits.is_empty(), "chord line repeats on a translated star");
        }
    }

    #[test]
    fn exit_lines_are_tangent_to_the_neighbour_chord_circle() {
        // The two tip-0 exit lines (directions 30° and 150° as lines)
        // sit at the chord distance R/2 = 1/4 from the neighbouring
        // star centre (1, 0): signed distance squared == 1/16, exact.
        let t = tips_natural();
        let (a1, _) = lattice();
        for k in [0usize, 2] {
            let w = dir30(k); // unit directions 30° and 150°
            let c = cross(vsub(a1, t[0]), w);
            assert!(
                c.mul(c).eq_x(Zq::new(1, 0, 3, 16)),
                "tangent identity for exit line {k}"
            );
        }
    }

    #[test]
    fn sym6_rotation_maps_record_to_itself() {
        let h = hull();
        let s = vertex_sum(&h);
        let c = (div_i(s.0, 6), div_i(s.1, 6));
        let rot = |p: Qp| vadd(rot30n(vsub(p, c), 2), c);
        let w0 = rot(h[0]);
        let m = (0..6)
            .find(|&m| pt_eq(w0, h[m]))
            .expect("image is a vertex");
        for i in 0..6 {
            assert!(pt_eq(rot(h[i]), h[(i + m) % 6]), "sym6 vertex {i}");
        }
        let mut got: Vec<_> = exit_rays()
            .into_iter()
            .map(|(i, d)| ((i + m) % 6, rot30n(d, 2)))
            .collect();
        let mut want: Vec<_> = exit_rays();
        let sort = |v: &mut Vec<(usize, Qp)>| v.sort_by_key(|(i, d)| (*i, key6(*d)));
        sort(&mut got);
        sort(&mut want);
        assert!(got
            .iter()
            .zip(&want)
            .all(|((i, a), (j, b))| i == j && pt_eq(*a, *b)));
    }

    #[test]
    fn record_matches_landed_hexagram_star_tile() {
        use crate::families::islamic::hexagram::HexagramTiling;
        use tess_core::{BBox, ParamSet, Query, Tiling, Vec2};
        let params = ParamSet::default_for(HexagramTiling.params());
        let region = BBox::new(Vec2::new(-2.0, -2.0), Vec2::new(3.0, 3.0));
        let res = HexagramTiling.generate(&params, &Query { region });
        assert!(res.message.is_none(), "{:?}", res.message);
        let stars: Vec<_> = res.tiles.iter().filter(|t| t.poly.len() == 12).collect();
        assert!(!stars.is_empty(), "the tiling emits 12-gon stars");
        let st = &stars[0];
        let n = 12;
        let mut cc = [0.0; 2];
        for p in &st.poly.pts {
            cc[0] += p.x / n as f64;
            cc[1] += p.y / n as f64;
        }
        // kissing structure: six tips at radius 1/2, six notches at 1/12.
        let (mut tips, mut nots) = (0, 0);
        for p in &st.poly.pts {
            let r2 = (p.x - cc[0]) * (p.x - cc[0]) + (p.y - cc[1]) * (p.y - cc[1]);
            if (r2 - 0.25).abs() < 1e-9 {
                tips += 1;
            } else if (r2 - 1.0 / 12.0).abs() < 1e-9 {
                nots += 1;
            }
        }
        assert_eq!((tips, nots), (6, 6), "star tip/notch radii at T = 1/2");
        // the tip set is congruent to the record hull (rotation by a
        // multiple of 60° + cyclic alignment).
        let mut tip_pts: Vec<[f64; 2]> = st
            .poly
            .pts
            .iter()
            .map(|p| [p.x - cc[0], p.y - cc[1]])
            .filter(|q| {
                let r2 = q[0] * q[0] + q[1] * q[1];
                (r2 - 0.25).abs() < 1e-9
            })
            .collect();
        tip_pts.sort_by(|a, b| a[1].atan2(a[0]).partial_cmp(&b[1].atan2(b[0])).unwrap());
        let h = hull();
        let hs = vertex_sum(&h);
        let hc = [hs.0.fl() / 6.0, hs.1.fl() / 6.0];
        let mut found = false;
        for rot in (0..6).rev() {
            let th = rot as f64 * std::f64::consts::FRAC_PI_3;
            let (cs, sn) = (th.cos(), th.sin());
            for shift in 0..6 {
                let ok = (0..6).all(|k| {
                    let p = h[(shift + k) % 6];
                    let q = [p.0.fl() - hc[0], p.1.fl() - hc[1]];
                    let x = cs * q[0] - sn * q[1];
                    let y = sn * q[0] + cs * q[1];
                    let w = tip_pts[k];
                    (x - w[0]).abs() < 1e-9 && (y - w[1]).abs() < 1e-9
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
        assert!(found, "record hull matches a landed star's tips at 1e-9");
    }
}
