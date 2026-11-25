//! The decagon monsters: `{10/4}` (for (10,2)) and `{10/2}` (for
//! (10,3)) records on the shipped girih-decagon hull (`girih_m161`,
//! the unit-edge regular decagon in `L10`), plus the two certified
//! pair placements.
//!
//! See the [module docs](super) for the catalog, the pinning rule and
//! the window certification. Both records live in `L10` (`a = 2cos
//! π/10`, `√5 = 2a² − 5`); the trait's rational-`sqrt` `contact_cos2`
//! tuple cannot encode `(3 ∓ √5)/8`, so — exactly like `GirihM161` —
//! the records return `None` and the batteries pin the angle exactly
//! in `L10`.

use super::{
    gmidpoints, star_exits, star_figure, verify_pair, Axis,
    PairData, PairSpec, WindowProbe, Gp,
};
use crate::medallion::girih_m161::{self, Kq};
use crate::medallion::{Absorption, Field, Medallion};


/// The shared record hull: the girih decagon's exact unit-edge regular
/// decagon (normalized frame; the same table M161 ships).
pub fn hull() -> Vec<Gp<Kq>> {
    girih_m161::hull()
}

/// `ρ = √5/4 = (2a² − 5)/4` for (10,2) (vertex axis, certified window
/// `(sin 18°, cos 36°)`).
pub(super) fn rho102() -> Kq {
    Kq::new([-5, 0, 2, 0], 4)
}

/// `ρ = (√5 − 1)/4 = (a² − 3)/2` for (10,3) (mid-edge axis, certified
/// window `(0, φ − 1)`).
pub(super) fn rho103() -> Kq {
    Kq::new([-3, 0, 1, 0], 2)
}

/// `cos²(4π/10) = (3 − √5)/8` — the (10,2) crossing/contact angle.
/// In the L10 basis `√5 = 2a² − 5`, so `3 − √5 = 8 − 2a²`.
pub(super) fn cos2102() -> Kq {
    Kq::new([8, 0, -2, 0], 8)
}

/// `cos²(2π/10) = (3 + √5)/8` — the (10,3) crossing/contact angle
/// (`3 + √5 = 2a² − 2`).
pub(super) fn cos2103() -> Kq {
    Kq::new([-2, 0, 2, 0], 8)
}

/// The `{10/4}` record: two pentagrams on the decagon, contact angle
/// `4π/10 = 72°` (`cos² = (3 − √5)/8`).
pub struct Monster104;

impl Medallion for Monster104 {
    const ID: &'static str = "decagon {10/4} monster star (24c)";
    const FIELD: Field = Field::L10;
    const SYM: usize = 10;
    const PATH: Absorption = Absorption::BoundaryInference;

    fn hull_f64() -> Vec<[f64; 2]> {
        hull().iter().map(|p| [p.0.fl(), p.1.fl()]).collect()
    }
    fn exit_rays_f64() -> Vec<(usize, [f64; 2])> {
        exit_rays()
            .into_iter()
            .map(|(i, d)| (i, [d.0.fl(), d.1.fl()]))
            .collect()
    }
    // cos² = (3 − √5)/8 is not of the trait's rational-sqrt tuple
    // form; the battery below pins it exactly in L10 (M161 precedent).
}

/// The `{10/2}` record: two pentagons on the decagon, contact angle
/// `2π/10 = 36°` (`cos² = (3 + √5)/8`).
pub struct Monster102;

impl Medallion for Monster102 {
    const ID: &'static str = "decagon {10/2} monster star (24c)";
    const FIELD: Field = Field::L10;
    const SYM: usize = 10;
    const PATH: Absorption = Absorption::BoundaryInference;

    fn hull_f64() -> Vec<[f64; 2]> {
        hull().iter().map(|p| [p.0.fl(), p.1.fl()]).collect()
    }
    fn exit_rays_f64() -> Vec<(usize, [f64; 2])> {
        exit_rays2()
            .into_iter()
            .map(|(i, d)| (i, [d.0.fl(), d.1.fl()]))
            .collect()
    }
}

/// The `{10/4}` figure as contact-index pairs.
pub fn figure4() -> Vec<(usize, usize)> {
    star_figure(10, 4)
}

/// The `{10/2}` figure as contact-index pairs.
pub fn figure2() -> Vec<(usize, usize)> {
    star_figure(10, 2)
}

/// The 20 directed exit rays of the `{10/4}` record: both chord
/// continuations `M_i − M_{i±4}` at every contact.
pub fn exit_rays() -> Vec<(usize, Gp<Kq>)> {
    star_exits(&gmidpoints(&hull()), 4)
}

/// The 20 directed exit rays of the `{10/2}` record.
pub fn exit_rays2() -> Vec<(usize, Gp<Kq>)> {
    star_exits(&gmidpoints(&hull()), 2)
}

/// The certified (10,2) placement: `P = (√5/4)·D` on the vertex axis,
/// `eAB = [4,5]`, `eBA = [0,9]`, crossings `(A3,B1)` and `(A6,B8)`.
pub fn pair102() -> PairData<Kq> {
    verify_pair(
        &PairSpec {
            name: "(10,2)",
            hull_a: hull(),
            rho: rho102(),
            axis: Axis::VertexAxis,
            d: 4,
            e_ab: vec![4, 5],
            cross: vec![(3, 1), (6, 8)],
            cos2: cos2102(),
        },
        &[
            // Above the window (upper end cos 36° = (a² − 2)/2):
            // counts drop to (0,0), the boundaries still cross
            // transversally twice.
            WindowProbe {
                rho: Kq::new([-2, 0, 1, 0], 2).add(Kq::new([1, 0, 0, 0], 32)),
                ca: 0,
                cb: 0,
                crossings: Some(2),
            },
            // Below the window (lower end sin 18° = (a² − 3)/2): the
            // boundaries slide into collinear edge overlap — the
            // full-edge counts stay (2,2) but there is no transversal
            // crossing.
            WindowProbe {
                rho: Kq::new([-3, 0, 1, 0], 2).sub(Kq::new([1, 0, 0, 0], 64)),
                ca: 2,
                cb: 2,
                crossings: Some(0),
            },
        ],
    )
}

/// The certified (10,3) placement: `P = ((√5−1)/4)·D` on the mid-edge
/// axis, `eAB = [4,5,6]`, `eBA = [0,1,9]`, crossings `(A3,B2)` and
/// `(A7,B8)`.
pub fn pair103() -> PairData<Kq> {
    verify_pair(
        &PairSpec {
            name: "(10,3)",
            hull_a: hull(),
            rho: rho103(),
            axis: Axis::MidEdgeAxis,
            d: 2,
            e_ab: vec![4, 5, 6],
            cross: vec![(3, 2), (7, 8)],
            cos2: cos2103(),
        },
        &[
            // Above the window (upper end phi − 1 = a² − 3): the
            // count drops to (1,1), crossings remain 2.
            WindowProbe {
                rho: Kq::new([-3, 0, 1, 0], 1).add(Kq::new([1, 0, 0, 0], 32)),
                ca: 1,
                cb: 1,
                crossings: Some(2),
            },
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{battery_record, gmidpoints};

    fn kp_eq(p: Gp<Kq>, q: Gp<Kq>) -> bool {
        p.0.eq_x(q.0) && p.1.eq_x(q.1)
    }

    #[test]
    fn decagon_records_pass_the_schema_battery() {
        battery_record("{10/4}", &hull(), 4, cos2102());
        battery_record("{10/2}", &hull(), 2, cos2103());
        // The hull is exactly the M161 table.
        let g = girih_m161::hull();
        assert_eq!(hull().len(), g.len());
        for (p, q) in hull().iter().zip(&g) {
            assert!(kp_eq(*p, *q));
        }
        // Figures and exits are the star pattern.
        assert_eq!(figure4(), star_figure(10, 4));
        assert_eq!(figure2().len(), 10);
        assert_eq!(exit_rays().len(), 20);
        assert_eq!(exit_rays2().len(), 20);
    }

    #[test]
    fn contact_cos2_is_none_and_the_l10_angle_is_pinned() {
        // L10 precedent (M161): the trait tuple cannot hold these.
        assert_eq!(<Monster104 as Medallion>::contact_cos2(), None);
        assert_eq!(<Monster102 as Medallion>::contact_cos2(), None);
        // Exact independent pins: the chord M_0→M_d against the unit
        // edge satisfies cos²·|chord|²·|edge|² = dot² for the declared
        // constant, and the two constants are the conjugate pair
        // (sum 3/4, product 1/16 — i.e. exactly (3 ∓ √5)/8).
        let h = hull();
        let m = gmidpoints(&h);
        for (d, cos2) in [(4usize, cos2102()), (2usize, cos2103())] {
            let chord = (m[d].0.sub(m[0].0), m[d].1.sub(m[0].1));
            let edge = (h[1].0.sub(h[0].0), h[1].1.sub(h[0].1));
            let dot = chord.0.mul(edge.0).add(chord.1.mul(edge.1));
            let n2 = chord.0.mul(chord.0).add(chord.1.mul(chord.1));
            let e2 = edge.0.mul(edge.0).add(edge.1.mul(edge.1));
            assert!(cos2.mul(n2).mul(e2).eq_x(dot.mul(dot)), "cos² pin d = {d}");
        }
        assert!(cos2102().add(cos2103()).eq_x(Kq::new([3, 0, 0, 0], 4)));
        assert!(cos2102().mul(cos2103()).eq_x(Kq::new([1, 0, 0, 0], 16)));
    }

    #[test]
    fn placements_are_the_certified_pins() {
        let d = pair102();
        // P = (√5/4)·D with D = (1, −2 + a³) gives ((2a²−5)/4, a³/4).
        assert!(d.p.0.eq_x(Kq::new([-5, 0, 2, 0], 4)));
        assert!(d.p.1.eq_x(Kq::new([0, 0, 0, 1], 4)));
        assert_eq!(d.e_ab, vec![4, 5]);
        assert_eq!(d.e_ba, vec![0, 9]);
        assert_eq!(d.exposed_a, vec![0, 1, 2, 3, 7, 8, 9]);
        assert_eq!(d.exposed_b, vec![2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(d.union_cycle.len(), 16);
        assert_eq!(d.lens_cycle.len(), 8);
        // Pin identity P = M_3 − M_1 (crossing pair (A3, B1)).
        let m = gmidpoints(&hull());
        let pin = (m[3].0.sub(m[1].0), m[3].1.sub(m[1].1));
        assert!(kp_eq(pin, d.p));
        assert!(kp_eq(d.cross_pts[0], m[3]));

        let d = pair103();
        // P = ((a² − 3)/2)·(0, −2 + a³) = (0, a/2).
        assert!(d.p.0.eq_x(Kq::zero()));
        assert!(d.p.1.eq_x(Kq::new([0, 1, 0, 0], 2)));
        assert_eq!(d.e_ab, vec![4, 5, 6]);
        assert_eq!(d.e_ba, vec![0, 1, 9]);
        assert_eq!(d.exposed_a, vec![0, 1, 2, 3, 8, 9]);
        assert_eq!(d.union_cycle.len(), 14);
        assert_eq!(d.lens_cycle.len(), 10);
        let pin = (m[3].0.sub(m[2].0), m[3].1.sub(m[2].1));
        assert!(kp_eq(pin, d.p));
    }
}
