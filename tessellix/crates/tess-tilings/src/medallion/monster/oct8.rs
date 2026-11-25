//! The (8,2) monster: a pair of `KhatamM53` octagon records placed at
//! `B = A + P`, `P = (√2/4)·D` along the vertex axis `D = V4 − V0`.
//!
//! See the [module docs](super) for the catalog, the pinning rule and
//! the window certification. The record side is the landed M53 record
//! (`{8/2}` figure, contact angle 45°, `cos² = 1/2`, landing identity)
//! — nothing new ships here beyond the certified placement.

use super::{
    verify_pair, Axis, PairData, PairSpec, WindowProbe, Gp,
};
use crate::medallion::khatam_m53;
use crate::medallion::Zq;

/// `ρ = √2/4` (the pin `P = ρ·D`, window `(0, √2/2)`).
pub(super) fn rho() -> Zq {
    Zq::new(0, 1, 2, 4)
}

/// `cos²(2π/8) = 1/2` — the crossing angle and the M53 contact angle.
pub(super) fn cos2() -> Zq {
    Zq::new(1, 0, 2, 2)
}

/// The record hull (the unit-edge regular octagon, normalized frame).
pub fn hull() -> Vec<Gp<Zq>> {
    khatam_m53::hull()
}

/// The certified (8,2) placement: `eAB = [3,4]`, `eBA = [0,7]`,
/// crossings `(A2,B1)` and `(A5,B6)` at the edge midpoints.
pub fn pair82() -> PairData<Zq> {
    verify_pair(
        &PairSpec {
            name: "(8,2)",
            hull_a: hull(),
            rho: rho(),
            axis: Axis::VertexAxis,
            d: 2,
            e_ab: vec![3, 4],
            cross: vec![(2, 1), (5, 6)],
            cos2: cos2(),
        },
        &[
            // Above the window (window is (0, √2/2)): no edge of
            // either polygon is inside the other, the boundaries
            // still cross twice.
            WindowProbe {
                rho: Zq::new(3, 0, 2, 4),
                ca: 0,
                cb: 0,
                crossings: Some(2),
            },
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::OutNode;
    use crate::medallion::khatam_m53::KhatamM53;
    use crate::medallion::{equal_edges, midpoints, strictly_ccw};
    use std::cmp::Ordering;

    #[test]
    fn octagon_hull_is_the_record_schema() {
        // Reuses the landed M53 table; the shared frame helpers re-check
        // strict convexity, unit edges and the normalized frame.
        let h = hull();
        assert!(strictly_ccw(&h));
        assert!(equal_edges(&h));
        assert_eq!(h[0], (Zq::zero(2), Zq::zero(2)));
        assert_eq!(h[1], (Zq::whole(2, 1), Zq::zero(2)));
    }

    #[test]
    fn record_is_the_reused_m53_star() {
        // The (8,2) monster's record IS KhatamM53: {8/2} figure step,
        // contact angle cos² = 1/2 (the trait tuple (1,0,2,2)).
        use crate::medallion::Medallion;
        assert_eq!(<KhatamM53 as Medallion>::SYM, 8);
        assert_eq!(
            <KhatamM53 as Medallion>::contact_cos2(),
            Some((1, 0, 2, 2))
        );
        super::super::battery_record("<M53 as (8,2) record>", &hull(), 2, cos2());
    }

    #[test]
    fn placement_is_the_certified_pin() {
        let d = pair82();
        // Exact displacement: P = (√2/4)·(1, 1+√2) = (√2/4, (2+√2)/4).
        assert_eq!(d.p.0, Zq::new(0, 1, 2, 4));
        assert_eq!(d.p.1, Zq::new(2, 1, 2, 4));
        assert_eq!(d.e_ab, vec![3, 4]);
        assert_eq!(d.e_ba, vec![0, 7]);
        assert_eq!(d.exposed_a, vec![0, 1, 2, 6, 7]);
        assert_eq!(d.exposed_b, vec![2, 3, 4, 5, 6]);
        // Crossings sit at edge midpoints, and P = M_i − M_j (the pin):
        // (A2, B1) and (A5, B6).
        let m = midpoints(&hull());
        assert_eq!(d.cross_pts[0], m[2]);
        assert_eq!(
            d.cross_pts[1],
            (Zq::new(0, -1, 2, 4), Zq::new(4, 3, 2, 4))
        );
        let p_of = |i: usize, j: usize| {
            (
                m[i].0.sub(m[j].0),
                m[i].1.sub(m[j].1),
            )
        };
        assert_eq!(p_of(2, 1), d.p);
        assert_eq!(p_of(5, 6), d.p);
    }

    #[test]
    fn outlines_are_exact_cycles() {
        let d = pair82();
        // Union: 2·(8−2−1) + 2 = 12 nodes; lens: 2·2 + 4 = 8 nodes.
        assert_eq!(d.union_cycle.len(), 12);
        assert_eq!(d.lens_cycle.len(), 8);
        // Every vertex index occurs exactly once across the two cycles
        // of its side, in the right one.
        for side in 0..2 {
            let mut seen = vec![0usize; 8];
            for cyc in [&d.union_cycle, &d.lens_cycle] {
                for t in cyc {
                    let k = match (t, side) {
                        (OutNode::A(k), 0) | (OutNode::B(k), 1) => *k,
                        _ => continue,
                    };
                    seen[k] += 1;
                }
            }
            assert_eq!(seen, vec![1; 8]);
            // The union carries exactly the exposed vertices.
            let mut u: Vec<usize> = d
                .union_cycle
                .iter()
                .filter_map(|t| match (t, side) {
                    (OutNode::A(k), 0) | (OutNode::B(k), 1) => Some(*k),
                    _ => None,
                })
                .collect();
            u.sort_unstable();
            let want = if side == 0 {
                &d.exposed_a
            } else {
                &d.exposed_b
            };
            assert_eq!(u, *want);
        }
        // Exact node positions close both cycles: consecutive nodes are
        // distinct and the float view of the traversal is a closed
        // polygon with nonzero area (the cycles are certified by
        // verify_pair; this pins the label-to-geometry reading).
        let m = midpoints(&hull());
        let pos = |t: &OutNode| -> (f64, f64) {
            match *t {
                OutNode::A(k) => (hull()[k].0.fl(), hull()[k].1.fl()),
                OutNode::B(k) => (
                    hull()[k].0.add(d.p.0).fl(),
                    hull()[k].1.add(d.p.1).fl(),
                ),
                OutNode::X(k) => (m[[2usize, 5][k]].0.fl(), m[[2usize, 5][k]].1.fl()),
            }
        };
        for cyc in [&d.union_cycle, &d.lens_cycle] {
            let pts: Vec<(f64, f64)> = cyc.iter().map(pos).collect();
            for w in pts.windows(2) {
                let dx = w[1].0 - w[0].0;
                let dy = w[1].1 - w[0].1;
                assert!(dx * dx + dy * dy > 1e-9, "outline has a degenerate edge");
            }
            let mut area: f64 = 0.0;
            for i in 0..pts.len() {
                let (a, b) = (pts[i], pts[(i + 1) % pts.len()]);
                area += a.0 * b.1 - a.1 * b.0;
            }
            assert!(area.abs() > 1.0, "outline area degenerates");
        }
    }

    #[test]
    fn pin_sits_strictly_inside_its_window() {
        // verify_pair already samples pin ± D/64; this pins the exact
        // window ends from the event census: (0, √2/2), pin √2/4.
        let lo = Zq::zero(2);
        let hi = Zq::new(0, 1, 2, 2); // √2/2
        assert_eq!(rho().sgn(), Ordering::Greater);
        assert_eq!(rho().sub(hi).sgn(), Ordering::Less);
        assert_eq!(rho().mul(Zq::whole(2, 2)), hi); // 2ρ = hi
        assert_eq!(lo.sgn(), Ordering::Equal);
    }
}
