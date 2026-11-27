//! M158 — the WIS 12-point star medallion: exact record + verification
//! battery (design doc §4.5).
//!
//! Twelve rhomb prototiles placed at successive 30-degree rotations
//! about a common acute vertex (the [WSI95] dodecagonal quasiperiodic
//! star). The medallion is recorded on its convex hull: the regular
//! unit-edge dodecagon through the twelve star tips, in the normalized
//! record frame. Contacts are the twelve corner tips; the internal
//! figure is the 24 unit tip-notch outline edges of the star; the 24
//! exit rays are the two notch→tip chord continuations at each tip.
//! Declared symmetry: 12.
//!
//! Coordinates: the shipped WIS lattice — raw `Z[sqrt3]` 4-tuples
//! `(P, Q, R, S)` with world value `((P + Q sqrt3)/2, (R + S sqrt3)/2)`,
//! i.e. exactly [`Qp`] over `Q(sqrt3)` at denominator 2 (doc §1.1).
//!
//! Ground truth (doc §4.5, honesty rule): occurrence of the full
//! 12-around-vertex star inside the shipped [WSI95] tiling is NOT
//! certified by landed data (the 26-tile patch contains no such vertex,
//! and M158 states the Bielefeld image is not a machine oracle). The
//! ground truth here is the medallion's exact geometry, cross-checked
//! against the shipped `pub(crate)` prototile data: the battery
//! recomputes the hull, centre, notches, figure and exit rays from
//! [`RHO_CORN`] + [`rot4`] alone and requires EXACT agreement with the
//! record tables below. Path (c): the 24 rays bind onto the 3.12.12
//! dodecagon hole and are absorbed exactly — [`crate::medallion::host312`].
//!
//! [`Qp`]: super::Qp
//! [`RHO_CORN`]: crate::families::quasiperiodic::wis_12_fold::RHO_CORN
//! [`rot4`]: crate::families::quasiperiodic::wis_12_fold::rot4

use super::*;

use crate::families::quasiperiodic::wis_12_fold::P4;

/// Hull (the tip dodecagon) in raw lattice coordinates, normalized
/// frame: V0 = origin, edge 0 = raw (2, 0, 0, 0) = world (1, 0), CCW.
pub const HULL_TBL: [P4; 12] = [
    (0, 0, 0, 0),
    (2, 0, 0, 0),
    (2, 1, 1, 0),
    (3, 1, 1, 1),
    (3, 1, 3, 1),
    (2, 1, 3, 2),
    (2, 0, 4, 2),
    (0, 0, 4, 2),
    (0, -1, 3, 2),
    (-1, -1, 3, 1),
    (-1, -1, 1, 1),
    (0, -1, 1, 0),
];

/// The twelve star notches (interior concave corners of the 24-gon
/// outline), raw, in the same normalization.
pub const NOTCH_TBL: [P4; 12] = [
    (1, 0, 0, 1),
    (1, -1, 3, 1),
    (1, 1, 1, 1),
    (1, 1, 3, 1),
    (3, 0, 2, 1),
    (2, 0, 2, 0),
    (1, -1, 1, 1),
    (0, 0, 2, 0),
    (2, 0, 2, 2),
    (0, 0, 2, 2),
    (1, 0, 4, 1),
    (-1, 0, 2, 1),
];

/// The star centre (= the common acute vertex), raw: the rotation
/// centre of the 12-fold symmetry, world ((1 + sqrt3)/2, (2 + sqrt3)/2).
pub const CENTRE_TBL: P4 = (1, 0, 2, 1);

/// Raw 4-tuple negation (test helper, normalization search).
#[cfg(test)]
fn neg4(p: P4) -> P4 {
    (-p.0, -p.1, -p.2, -p.3)
}
/// Raw 4-tuple difference.
fn sub4(a: P4, b: P4) -> P4 {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2, a.3 - b.3)
}

/// Squared world norm times 4 (raw units): the exact `Z[sqrt3]` value
/// `(a + 3b + c + 3d, 2(ab + cd))` of `|p|^2` (the `_n` census of the
/// Python battery `wis_medallion.py`).
fn n4(p: P4) -> (i64, i64) {
    (
        p.0 * p.0 + 3 * p.1 * p.1 + p.2 * p.2 + 3 * p.3 * p.3,
        2 * (p.0 * p.1 + p.2 * p.3),
    )
}

/// Raw lattice point -> exact world point in `Q(sqrt3)` (world = raw/2).
pub fn to4(p: P4) -> Qp {
    (Zq::new(p.0, p.1, 3, 2), Zq::new(p.2, p.3, 3, 2))
}

/// Hull vertices, exact world coordinates.
pub fn hull() -> Vec<Qp> {
    HULL_TBL.iter().copied().map(to4).collect()
}

/// The twelve notches, exact world coordinates.
pub fn notches() -> Vec<Qp> {
    NOTCH_TBL.iter().copied().map(to4).collect()
}

/// The star centre, exact world coordinates.
pub fn centre() -> Qp {
    to4(CENTRE_TBL)
}

/// The internal figure: the 24 unit tip-notch outline edges, recorded
/// as `(tip, notch)` pairs (hull order, then notch-table order).
pub fn figure() -> Vec<(Qp, Qp)> {
    let mut out = Vec::with_capacity(24);
    for t in HULL_TBL {
        for n in NOTCH_TBL {
            if n4(sub4(t, n)) == (4, 0) {
                out.push((to4(t), to4(n)));
            }
        }
    }
    out
}

/// The 24 directed exit rays `(tip index, direction)`: at every tip the
/// continuations `tip - notch` of the two outline edges arriving there.
pub fn exit_rays() -> Vec<(usize, Qp)> {
    let mut out = Vec::with_capacity(24);
    for (i, t) in HULL_TBL.iter().enumerate() {
        for n in NOTCH_TBL {
            if n4(sub4(*t, n)) == (4, 0) {
                out.push((i, to4(sub4(*t, n))));
            }
        }
    }
    out
}

/// The record's trait surface.
pub struct WisM158;

impl Medallion for WisM158 {
    const ID: &'static str = "WIS 12-point star (M158)";
    const FIELD: Field = Field::Qsqrt3;
    const SYM: usize = 12;
    const PATH: Absorption = Absorption::Interstitial;
    fn contacts_f64() -> Vec<([f64; 2], Option<(usize, f64)>)> {
        hull()
            .iter()
            .map(|p| (super::qp_f64(*p), None))
            .collect()
    }
    fn figure_f64() -> Vec<([f64; 2], [f64; 2])> {
        figure()
            .iter()
            .map(|(a, b)| (super::qp_f64(*a), super::qp_f64(*b)))
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
    // Corner-contact record: the angle datum is the exit-ray multiset.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::families::quasiperiodic::wis_12_fold::{add4, rot4, RHO_CORN};
    use std::cmp::Ordering;

    fn z(a: i64, b: i64) -> Zq {
        Zq::new(a, b, 3, 1)
    }

    /// World-angle sort key for raw tuples (ordering helper only; every
    /// equality decision in this battery is exact integer arithmetic).
    fn raw_angle(p: P4) -> f64 {
        let s3 = 3f64.sqrt();
        (p.2 as f64 + p.3 as f64 * s3).atan2(p.0 as f64 + p.1 as f64 * s3)
    }

    #[test]
    fn record_schema_invariants() {
        let h = hull();
        assert!(strictly_ccw(&h), "hull strictly convex + CCW");
        assert!(equal_edges(&h), "equal hull edges");
        assert!(norm2(vsub(h[1], h[0])).eq_x(Zq::whole(3, 1)), "unit edges");
        assert!(normalized_frame(&h, Zq::whole(3, 1)));
        // doubled area: the unit regular dodecagon, 12 + 6 sqrt3.
        assert!(
            area2(&h).eq_x(Zq::whole(3, 12).add(z(0, 6))),
            "hull doubled area"
        );
        // tip/notch radii: |tip - c|^2 = 2 + sqrt3, |notch - c|^2 = 1.
        let c = centre();
        for p in &h {
            assert!(norm2(vsub(*p, c)).eq_x(z(2, 1)), "tip radius^2");
        }
        for p in &notches() {
            assert!(norm2(vsub(*p, c)).eq_x(Zq::whole(3, 1)), "notch radius^2");
        }
        // corner contacts: all twelve tips, each a figure endpoint twice.
        let fig = figure();
        assert_eq!(fig.len(), 24);
        for i in 0..12 {
            let tip = h[i];
            let n = fig
                .iter()
                .filter(|(a, b)| pt_eq(*a, tip) || pt_eq(*b, tip))
                .count();
            assert_eq!(n, 2, "tip {i} figure degree");
            let nn = fig.iter().filter(|(a, _)| pt_eq(*a, tip)).count();
            assert_eq!(nn, 2, "tips recorded first");
        }
        // exits: 24, two per tip, exactly the tip - notch continuations.
        let ex = exit_rays();
        assert_eq!(ex.len(), 24);
        for i in 0..12 {
            assert_eq!(ex.iter().filter(|(c, _)| *c == i).count(), 2);
        }
        for (a, b) in &fig {
            let i = (0..12).find(|&i| pt_eq(h[i], *a)).unwrap();
            assert!(
                ex.iter().any(|(c, d)| *c == i && pt_eq(*d, vsub(*a, *b))),
                "exit continuation at tip {i}"
            );
        }
        // exits outward: dot(direction, tip - centre) > 0.
        for (i, d) in &ex {
            let w = vsub(h[*i], c);
            assert_eq!(dot(*d, w).sgn(), Ordering::Greater, "outward");
        }
        // every figure edge is unit: tip-notch distance^2 = 1.
        for (a, b) in &fig {
            assert!(
                norm2(vsub(*a, *b)).eq_x(Zq::whole(3, 1)),
                "unit outline edge"
            );
        }
    }

    #[test]
    fn star_area_identity() {
        // The 24-gon outline is a single alternating tip/notch cycle
        // (walk the figure adjacency); its doubled area is exactly 12,
        // and hull2 = star2 + 12 * gap2 with gap2 = sqrt3/2 (the twelve
        // unit-equilateral hull-vs-star gap triangles), all exact.
        let fig: Vec<(usize, usize)> = {
            let mut out = Vec::new();
            for (i, t) in HULL_TBL.iter().enumerate() {
                for (j, n) in NOTCH_TBL.iter().enumerate() {
                    if n4(sub4(*t, *n)) == (4, 0) {
                        out.push((i, j));
                    }
                }
            }
            out
        };
        assert_eq!(fig.len(), 24);
        // degrees 2/2 everywhere (a single 24-cycle to be shown by walk).
        for i in 0..12 {
            assert_eq!(fig.iter().filter(|(t, _)| *t == i).count(), 2);
            assert_eq!(fig.iter().filter(|(_, n)| *n == i).count(), 2);
        }
        // walk the cycle from tip 0.
        let mut walk: Vec<Qp> = vec![to4(HULL_TBL[0])];
        let edges = fig;
        let mut nxt = edges.iter().find(|(t, _)| *t == 0).unwrap().1;
        walk.push(to4(NOTCH_TBL[nxt]));
        let mut prev_tip = 0usize;
        let mut steps = 2;
        while steps < 24 {
            let t2 = *edges
                .iter()
                .find(|(t, n)| *n == nxt && *t != prev_tip)
                .map(|(t, _)| t)
                .expect("notch degree");
            walk.push(to4(HULL_TBL[t2]));
            steps += 1;
            if steps == 24 {
                break;
            }
            let n2 = *edges
                .iter()
                .find(|(t, n)| *t == t2 && *n != nxt)
                .map(|(_, n)| n)
                .expect("tip degree");
            walk.push(to4(NOTCH_TBL[n2]));
            steps += 1;
            nxt = n2;
            prev_tip = t2;
        }
        assert_eq!(walk.len(), 24, "single 24-cycle");
        // the cycle closes on a figure edge back to tip 0
        assert!(
            n4(sub4(NOTCH_TBL[nxt], HULL_TBL[0])) == (4, 0),
            "cycle closes on a unit figure edge"
        );
        // doubled areas: star = 12, hull - star = 12 * sqrt3/2 = 6 sqrt3.
        assert!(area2(&walk).eq_x(Zq::whole(3, 12)), "star doubled area 12");
        let gap = area2(&hull()).sub(area2(&walk));
        assert!(
            gap.eq_x(z(0, 6)),
            "hull2 - star2 = 12 * (sqrt3 / 2) gap triangles"
        );
    }

    #[test]
    fn recomputation_from_shipped_prototile_data() {
        // Ground truth cross-check (doc §4.5): recompute the whole
        // record from the shipped RHO_CORN + rot4 prototile data.
        let mut tips: Vec<P4> = Vec::new();
        let mut nots: Vec<P4> = Vec::new();
        for k in 0..12i8 {
            for c in RHO_CORN {
                let r = rot4(c, k);
                if n4(r) == (8, 4) {
                    if !tips.contains(&r) {
                        tips.push(r);
                    }
                } else if n4(r) == (4, 0) && r != (0, 0, 0, 0) {
                    if !nots.contains(&r) {
                        nots.push(r);
                    }
                }
            }
        }
        assert_eq!(tips.len(), 12, "twelve tips");
        assert_eq!(nots.len(), 12, "twelve notches");
        // normalization search: shift = -tip so the shifted, angle-
        // sorted cycle starts at 0 with edge 0 = raw (2, 0, 0, 0).
        let mut found: Option<(P4, Vec<P4>)> = None;
        for c in &tips {
            let s = neg4(*c);
            let mut sh: Vec<P4> = tips.iter().map(|p| add4(*p, s)).collect();
            sh.sort_by(|a, b| {
                raw_angle(*a)
                    .partial_cmp(&raw_angle(*b))
                    .unwrap()
                    .then(a.0.cmp(&b.0).then(a.2.cmp(&b.2)))
            });
            for st in 0..12 {
                let poly: Vec<P4> = sh[st..].iter().chain(&sh[..st]).copied().collect();
                // canonical: V0 = origin, edge 0 = raw (2, 0, 0, 0), and
                // a genuine cyclic polygon order (every consecutive pair
                // is a unit hull edge — this rejects any candidate where
                // the angle fan around the vertex straddles the atan2
                // cut and the sorted list wraps).
                if poly[0] == (0, 0, 0, 0)
                    && sub4(poly[1], poly[0]) == (2, 0, 0, 0)
                    && (0..12).all(|i| n4(sub4(poly[(i + 1) % 12], poly[i])) == (4, 0))
                {
                    found = Some((s, poly));
                    break;
                }
            }
            if found.is_some() {
                break;
            }
        }
        let (shift, order) = found.expect("normalization found");
        // EXACT regression against the shipped record tables.
        assert_eq!(order.as_slice(), HULL_TBL, "hull recomputed exactly");
        assert_eq!(shift, CENTRE_TBL, "centre recomputed exactly");
        let mut recomputed_notches: Vec<P4> = nots.iter().map(|n| add4(*n, shift)).collect();
        let mut want_notches: Vec<P4> = NOTCH_TBL.to_vec();
        recomputed_notches.sort_by_key(|p| (p.0, p.1, p.2, p.3));
        want_notches.sort_by_key(|p| (p.0, p.1, p.2, p.3));
        assert_eq!(
            recomputed_notches, want_notches,
            "notches recomputed exactly"
        );
        // notch radii and the figure/exits from the recomputed data.
        for n in &recomputed_notches {
            assert_eq!(n4(sub4(*n, shift)), (4, 0), "notch radius 1");
        }
        let fig_re: Vec<(usize, usize)> = {
            let mut out = Vec::new();
            for (i, t) in HULL_TBL.iter().enumerate() {
                for (j, n) in recomputed_notches.iter().enumerate() {
                    if n4(sub4(*t, *n)) == (4, 0) {
                        out.push((i, j));
                    }
                }
            }
            out
        };
        assert_eq!(fig_re.len(), 24, "figure recomputed: 24 unit edges");
        // the recomputed exit directions agree with the record exactly.
        let ex = exit_rays();
        for (i, n) in &fig_re {
            let d = to4(sub4(HULL_TBL[*i], recomputed_notches[*n]));
            assert!(
                ex.iter().any(|(c, w)| *c == *i && pt_eq(*w, d)),
                "exit ray recomputed"
            );
        }
    }

    #[test]
    fn exit_directions_on_the_thirty_degree_grid() {
        // Every exit direction is parallel to exactly one of the twelve
        // unit lattice directions (30-degree grid), and the cos^2-with-x
        // census is {0: 4, 1: 4, 1/4: 8, 3/4: 8} (|u|^2 = 1).
        let ex = exit_rays();
        let mut grid: Vec<Qp> = Vec::new();
        for j in 0..12 {
            grid.push(rot30n((Zq::whole(3, 1), Zq::zero(3)), j));
        }
        let (mut n0, mut n1, mut nq, mut nt) = (0, 0, 0, 0);
        for (_, u) in &ex {
            let hits = grid
                .iter()
                .filter(|w| cross(*u, **w).sgn() == Ordering::Equal)
                .count();
            assert_eq!(hits, 2, "exit direction lies on one grid line");
            let n = norm2(*u);
            assert!(n.eq_x(Zq::whole(3, 1)), "unit exit direction");
            if u.0.sgn() == Ordering::Equal {
                n0 += 1;
            } else if u.1.sgn() == Ordering::Equal {
                n1 += 1;
            } else if u.0.mul(u.0).scale(4).eq_x(Zq::whole(3, 1)) {
                nq += 1; // cos^2 = 1/4
            } else if u.0.mul(u.0).scale(4).eq_x(Zq::whole(3, 3)) {
                nt += 1; // cos^2 = 3/4
            } else {
                panic!("unexpected exit direction");
            }
        }
        assert_eq!((n0, n1, nq, nt), (4, 4, 8, 8), "exit cos^2 census");
    }

    #[test]
    fn sym12_rotation_maps_record_to_itself() {
        // rotation by +30 degrees about the centre (exact, Q(sqrt3)):
        // hull vertices, notches, figure and exit-ray set all map to
        // themselves with one common vertex offset m.
        let h = hull();
        let c = centre();
        let rot_about = |p: Qp| vadd(rot30(vsub(p, c)), c);
        let w0 = rot_about(h[0]);
        let m = (0..12)
            .find(|&m| pt_eq(w0, h[m]))
            .expect("V0 image is a vertex");
        assert_eq!(m, 1, "CCW generator");
        for i in 0..12 {
            assert!(pt_eq(rot_about(h[i]), h[(i + m) % 12]), "sym12 vertex {i}");
        }
        // notches as a set
        let ns = notches();
        for n in &ns {
            let r = rot_about(*n);
            assert!(ns.iter().any(|q| pt_eq(*q, r)), "notch maps to notch");
        }
        // figure and exit rays map to themselves
        let fig = figure();
        for (a, b) in &fig {
            let (ra, rb) = (rot_about(*a), rot_about(*b));
            assert!(
                fig.iter()
                    .any(|(x, y)| (pt_eq(*x, ra) && pt_eq(*y, rb))
                        || (pt_eq(*x, rb) && pt_eq(*y, ra))),
                "figure edge maps"
            );
        }
        let ex = exit_rays();
        for (i, d) in &ex {
            let rd = rot30(*d);
            assert!(
                ex.iter().any(|(j, w)| *j == (i + m) % 12 && pt_eq(*w, rd)),
                "sym12 on exit ray {i}"
            );
        }
    }
}
