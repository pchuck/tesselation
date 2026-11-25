//! The dodecagon monsters: `{12/2}` (for (12,4)) and `{12/6}` (for
//! (12,2)) records on the shipped M56 dodecagon hull (`rosette_m56`,
//! the unit-edge regular dodecagon in `Q(√3)`), plus the three
//! certified pair placements.
//!
//! See the [module docs](super) for the catalog, the pinning rule and
//! the window certification. The (12,3) entry reuses the landed
//! `RosetteM56` record (`{12/4}`, `cos² = 1/4`, landing identity).
//! The (12,2) figure step is `d = 6`: `{12/6}` joins opposite edge
//! midpoints — six diameters — so the two exit rays at each contact
//! **coincide** (`i + 6 ≡ i − 6`); the battery records this explicitly.

use super::{
    gmidpoints, star_exits, star_figure, verify_pair, Axis,
    PairData, PairSpec, WindowProbe, Gp,
};
use crate::medallion::rosette_m56;
use crate::medallion::{Absorption, Field, Medallion, Zq};

/// The record hull (the unit-edge regular dodecagon, normalized
/// frame — the table M56 ships).
pub fn hull() -> Vec<Gp<Zq>> {
    rosette_m56::hull()
}

/// `ρ = (1 + √3)/4` for (12,2) (vertex axis, window `(1/2, √3/2)`).
pub(super) fn rho122() -> Zq {
    Zq::new(1, 1, 3, 4)
}

/// `ρ = 1/2` for (12,3) (mid-edge axis, window `(0, √3 − 1)`).
pub(super) fn rho123() -> Zq {
    Zq::new(1, 0, 3, 2)
}

/// `ρ = 1/4` for (12,4) (vertex axis, window `(0, 1/2)`).
pub(super) fn rho124() -> Zq {
    Zq::new(1, 0, 3, 4)
}

/// `cos²(6π/12) = 0` — the (12,2) crossing/contact angle (diameters).
pub(super) fn cos2122() -> Zq {
    Zq::new(0, 0, 3, 1)
}

/// `cos²(4π/12) = 1/4` — the (12,3) angle (the M56 record's own).
pub(super) fn cos2123() -> Zq {
    Zq::new(1, 0, 3, 4)
}

/// `cos²(2π/12) = 3/4` — the (12,4) crossing/contact angle.
pub(super) fn cos2124() -> Zq {
    Zq::new(3, 0, 3, 4)
}

/// The `{12/2}` record: two hexagons on the dodecagon, contact angle
/// `2π/12 = 30°` (`cos² = 3/4`).
pub struct Monster122;

impl Medallion for Monster122 {
    const ID: &'static str = "dodecagon {12/2} monster star (24c)";
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
        Some((3, 0, 3, 4)) // cos² 30°
    }
}

/// The `{12/6}` record: six diameters (opposite edge-midpoint pairs),
/// contact angle `90°` (`cos² = 0`); the two exit rays at each contact
/// coincide.
pub struct Monster126;

impl Medallion for Monster126 {
    const ID: &'static str = "dodecagon {12/6} monster diameters (24c)";
    const FIELD: Field = Field::Qsqrt3;
    const SYM: usize = 12;
    const PATH: Absorption = Absorption::LandingIdentity;

    fn hull_f64() -> Vec<[f64; 2]> {
        hull().iter().map(|p| [p.0.fl(), p.1.fl()]).collect()
    }
    fn exit_rays_f64() -> Vec<(usize, [f64; 2])> {
        exit_rays6()
            .into_iter()
            .map(|(i, d)| (i, [d.0.fl(), d.1.fl()]))
            .collect()
    }
    fn contact_cos2() -> Option<(i64, i64, i64, i64)> {
        Some((0, 0, 3, 1)) // cos² 90°
    }
}

/// The `{12/2}` figure as contact-index pairs.
pub fn figure2() -> Vec<(usize, usize)> {
    star_figure(12, 2)
}

/// The `{12/6}` figure as contact-index pairs (six diameters, each
/// chord listed from both ends).
pub fn figure6() -> Vec<(usize, usize)> {
    star_figure(12, 6)
}

/// The 24 directed exit rays of the `{12/2}` record.
pub fn exit_rays() -> Vec<(usize, Gp<Zq>)> {
    star_exits(&gmidpoints(&hull()), 2)
}

/// The 24 directed exit-ray rows of the `{12/6}` record — 12 distinct
/// lines because both rows per contact coincide (`i + 6 ≡ i − 6`).
pub fn exit_rays6() -> Vec<(usize, Gp<Zq>)> {
    star_exits(&gmidpoints(&hull()), 6)
}

/// The certified (12,2) placement: `P = ((1+√3)/4)·D` on the vertex
/// axis, `eAB = [5,6]`, `eBA = [0,11]`, crossings `(A4,B1)` and
/// `(A7,B10)`.
pub fn pair122() -> PairData<Zq> {
    verify_pair(
        &PairSpec {
            name: "(12,2)",
            hull_a: hull(),
            rho: rho122(),
            axis: Axis::VertexAxis,
            d: 6,
            e_ab: vec![5, 6],
            cross: vec![(4, 1), (7, 10)],
            cos2: cos2122(),
        },
        &[
            // Below the window (lower end 1/2): the count jumps to
            // c = 4 (the (12,4) regime), crossings remain.
            WindowProbe {
                rho: Zq::new(2, 0, 3, 5),
                ca: 4,
                cb: 4,
                crossings: Some(2),
            },
            // Above the window (upper end √3/2): no full edge inside.
            WindowProbe {
                rho: Zq::new(0, 1, 3, 2).add(Zq::new(1, 0, 3, 32)),
                ca: 0,
                cb: 0,
                crossings: Some(2),
            },
        ],
    )
}

/// The certified (12,3) placement: `P = (1/2)·D` on the mid-edge
/// axis, `eAB = [5,6,7]`, `eBA = [0,1,11]`, crossings `(A4,B2)` and
/// `(A8,B10)`. The record side is the landed `RosetteM56`.
pub fn pair123() -> PairData<Zq> {
    verify_pair(
        &PairSpec {
            name: "(12,3)",
            hull_a: hull(),
            rho: rho123(),
            axis: Axis::MidEdgeAxis,
            d: 4,
            e_ab: vec![5, 6, 7],
            cross: vec![(4, 2), (8, 10)],
            cos2: cos2123(),
        },
        &[
            // Above the window (upper end √3 − 1): count drops to (1,1).
            WindowProbe {
                rho: Zq::new(0, 1, 3, 1)
                    .sub(Zq::whole(3, 1))
                    .add(Zq::new(1, 0, 3, 32)),
                ca: 1,
                cb: 1,
                crossings: Some(2),
            },
            // Near the lower end the pair still counts (3,3) full
            // edges but the boundaries no longer cross transversally.
            WindowProbe {
                rho: Zq::new(1, 0, 3, 100),
                ca: 3,
                cb: 3,
                crossings: Some(0),
            },
        ],
    )
}

/// The certified (12,4) placement: `P = (1/4)·D` on the vertex axis,
/// `eAB = [4,5,6,7]`, `eBA = [0,1,10,11]`, crossings `(A3,B2)` and
/// `(A8,B9)`.
pub fn pair124() -> PairData<Zq> {
    verify_pair(
        &PairSpec {
            name: "(12,4)",
            hull_a: hull(),
            rho: rho124(),
            axis: Axis::VertexAxis,
            d: 2,
            e_ab: vec![4, 5, 6, 7],
            cross: vec![(3, 2), (8, 9)],
            cos2: cos2124(),
        },
        &[
            // At the window's upper end (1/2) the crossing edges go
            // collinear: counts (2,2), no transversal crossing.
            WindowProbe {
                rho: Zq::new(1, 0, 3, 2),
                ca: 2,
                cb: 2,
                crossings: Some(0),
            },
            // Near zero displacement: the count jumps to (4,4) — the
            // polygons nearly coincide — and the boundaries still
            // cross transversally twice.
            WindowProbe {
                rho: Zq::new(1, 0, 3, 100),
                ca: 4,
                cb: 4,
                crossings: Some(2),
            },
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{battery_record, gmidpoints};
    use crate::medallion::rosette_m56::RosetteM56;
    use crate::medallion::{equal_edges, midpoints, strictly_ccw, Contact};
    use std::cmp::Ordering;

    fn zq(v: (i64, i64, i64)) -> Zq {
        Zq::new(v.0, v.1, 3, v.2)
    }

    #[test]
    fn dodecagon_records_pass_the_schema_battery() {
        battery_record("{12/2}", &hull(), 2, cos2124());
        battery_record("{12/6}", &hull(), 6, cos2122());
        battery_record("{12/4} (M56 as (12,3) record)", &hull(), 4, cos2123());
        // The hull is the M56 table; schema helpers re-check it.
        assert!(strictly_ccw(&rosette_m56::hull()));
        assert!(equal_edges(&rosette_m56::hull()));
        // Trait cos² tuples agree with the battery constants.
        assert_eq!(<Monster122 as Medallion>::contact_cos2(), Some((3, 0, 3, 4)));
        assert_eq!(<Monster126 as Medallion>::contact_cos2(), Some((0, 0, 3, 1)));
        assert_eq!(<RosetteM56 as Medallion>::contact_cos2(), Some((1, 0, 3, 4)));
        assert_eq!(cos2124(), zq((3, 0, 4)));
        assert_eq!(cos2123(), zq((1, 0, 4)));
        assert_eq!(cos2122(), Zq::zero(3));
    }

    #[test]
    fn twelve_six_exits_are_degenerate_coincident_pairs() {
        // Documented degeneracy: 24 rows, both rows of a contact equal,
        // 12 distinct directed rays = the six diameter lines run
        // outward from both ends.
        let ex = exit_rays6();
        assert_eq!(ex.len(), 24);
        let m = midpoints(&hull());
        for i in 0..12 {
            let a = ex.iter().find(|(c, _)| *c == i);
            assert!(a.is_some());
            let rows: Vec<&Gp<Zq>> = ex.iter().filter(|(c, _)| *c == i).map(|(_, v)| v).collect();
            assert_eq!(rows.len(), 2);
            assert_eq!(rows[0], rows[1], "coincident exit rows at {i}");
            let want = (m[i].0.sub(m[(i + 6) % 12].0), m[i].1.sub(m[(i + 6) % 12].1));
            assert_eq!(*rows[0], want);
        }
        // Figure {12/6} is six diameters listed from both ends.
        let f = figure6();
        assert_eq!(f.len(), 12);
        for (a, b) in &f {
            assert_eq!((a + 6) % 12, *b);
        }
    }

    #[test]
    fn placements_are_the_certified_pins() {
        let m = gmidpoints(&hull());
        let d = pair122();
        // P = ((1+√3)/4)·(1, 2+√3) = ((1+√3)/4, (5+3√3)/4).
        assert_eq!(d.p.0, zq((1, 1, 4)));
        assert_eq!(d.p.1, zq((5, 3, 4)));
        assert_eq!(d.e_ab, vec![5, 6]);
        assert_eq!(d.e_ba, vec![0, 11]);
        assert_eq!(d.exposed_a.len(), 9);
        assert_eq!(d.union_cycle.len(), 20);
        assert_eq!(d.lens_cycle.len(), 8);
        let pin = (m[4].0.sub(m[1].0), m[4].1.sub(m[1].1));
        assert_eq!(pin, d.p);

        let d = pair123();
        // P = (1/2)·(0, 2+√3) = (0, (2+√3)/2).
        assert_eq!(d.p.0, Zq::zero(3));
        assert_eq!(d.p.1, zq((2, 1, 2)));
        assert_eq!(d.e_ab, vec![5, 6, 7]);
        assert_eq!(d.e_ba, vec![0, 1, 11]);
        assert_eq!(d.union_cycle.len(), 18);
        assert_eq!(d.lens_cycle.len(), 10);
        let pin = (m[4].0.sub(m[2].0), m[4].1.sub(m[2].1));
        assert_eq!(pin, d.p);

        let d = pair124();
        // P = (1/4)·(1, 2+√3) = (1/4, (2+√3)/4).
        assert_eq!(d.p.0, zq((1, 0, 4)));
        assert_eq!(d.p.1, zq((2, 1, 4)));
        assert_eq!(d.e_ab, vec![4, 5, 6, 7]);
        assert_eq!(d.e_ba, vec![0, 1, 10, 11]);
        assert_eq!(d.exposed_a.len(), 7);
        assert_eq!(d.union_cycle.len(), 16);
        assert_eq!(d.lens_cycle.len(), 12);
        let pin = (m[3].0.sub(m[2].0), m[3].1.sub(m[2].1));
        assert_eq!(pin, d.p);
    }

    #[test]
    fn pins_are_strictly_inside_their_certified_windows() {
        // Window ends from the exact vertex-on-edge-line event census
        // (scratch spike2_events.py), re-checked exactly here.
        // (12,2): (1/2, √3/2), pin (1+√3)/4 = midpoint of the ends.
        let lo = Zq::new(1, 0, 3, 2);
        let hi = Zq::new(0, 1, 3, 2);
        assert_eq!(rho122().sub(lo).sgn(), Ordering::Greater);
        assert_eq!(rho122().sub(hi).sgn(), Ordering::Less);
        assert_eq!(rho122().mul(Zq::whole(3, 2)), lo.add(hi));
        // (12,3): (0, √3 − 1), pin 1/2 interior.
        let hi = Zq::new(-1, 1, 3, 1);
        assert_eq!(rho123().sgn(), Ordering::Greater);
        assert_eq!(rho123().sub(hi).sgn(), Ordering::Less);
        // (12,4): (0, 1/2), pin 1/4 = midpoint of the ends.
        let hi = Zq::new(1, 0, 3, 2);
        assert!(rho124().add(rho124()).eq_x(hi));
    }

    #[test]
    fn absorbed_contacts_are_edge_midpoints_schema() {
        // The 24a contact schema for the new dodecagon records: every
        // figure contact is an edge midpoint t = 1/2 (the record pair
        // absorbed by the monster union keeps exactly this schema).
        let m = gmidpoints(&hull());
        for (a, b) in figure2().iter().chain(figure6().iter()) {
            for &i in &[*a, *b] {
                let c = Contact::Edge {
                    edge: i,
                    t_num: 1,
                    t_den: 2,
                };
                if let Contact::Edge { edge, t_num, t_den } = c {
                    assert_eq!((t_num, t_den), (1, 2));
                    // Midpoint read-back agrees with the table.
                    let h = hull();
                    let mid = (
                        super::super::FieldEl::scaled_div(h[edge].0.add(h[(edge + 1) % 12].0), 1, 2),
                        super::super::FieldEl::scaled_div(h[edge].1.add(h[(edge + 1) % 12].1), 1, 2),
                    );
                    assert_eq!(mid, m[edge]);
                } else {
                    unreachable!();
                }
            }
        }
    }
}
