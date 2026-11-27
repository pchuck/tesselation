//! The hendecagon monsters: the three band rungs of odd n = 11 on the
//! exact unit-edge regular hendecagon hull — `{11/5}` for (11,2),
//! `{11/3}` for (11,3) and `{11/1}` for (11,4) (design doc
//! `docs/medallion-24c-odd-n-design.md` §2, the Δ = 3, 2, 1 rungs of
//! the ladder Δ = h − c with h = 5).
//!
//! These are rotated pairs (`B = R·A + P`, `R` the rotation by
//! `pi/11`) exactly as shipped for n = 9 (see [`n9`](super::n9) for the
//! parity story); the engine is the shared generic [`rotodd`] over the
//! certified degree-5 field [`K11`]. The N-layer coordinate type is
//! `N = K₁₁(w)`, `w² = 2 − b` (`w = 2 sin(pi/11)`): the rotated frame
//! needs `sin(2pi/11) ∉ K₁₁`, while `a = 2cos(pi/11)` IS inside `K₁₁`
//! (odd prime n puts `zeta_2n` in `Q(zeta_n)`) — the constant
//! `a = (1,−2,−3,1,1)` with `a² = 2 + b`, `a > 0`, re-asserted exactly
//! in the battery below.
//!
//! All three records have full-degree `cos²(d pi/11)` (the Galois
//! orbit of `cos²` has exactly 5 distinct conjugates — battery
//! certified), so `contact_cos2` returns `None` for all three (the
//! dec10/L10/n9-cos293 precedent) with the exact tuple pinned here.
//! Windows, crossing-parameter lines and outline cycles are the
//! M181 §2/§3 certified constants; the exact `t(sigma)`/`s(sigma)`
//! linearity shows the Δ-specific pattern: `t₀ = 0` for Δ = 2, `t₀ = 1`
//! for Δ = 1 (the σ = 0 intersection is `V₂`, on the `pi/n` ray at
//! every n — deviation D3 of the design doc), while Δ = 3 carries a
//! genuine K₁₁ `t₀`. See the [module docs](super) for the catalog and
//! the hub verifier machinery.

use super::{gmidpoints, star_exits, star_figure, FieldEl, Gp, PairData, WindowProbe};
use crate::fields::K11;
use crate::medallion::{Absorption, Field, Medallion};
use crate::medallion::monster::rotodd::{self, RotSpec};

/// `(u + v*w) / den` over [`K11`] — the N-layer coordinate type (the
/// `K11` instance of the generic [`rotodd::Nq`]).
pub type Nq = rotodd::Nq<K11>;

/// The record hull: the unit-edge regular hendecagon, normalized
/// frame (`V0 = origin`, edge 0 along +x), via the shared odd-n walk
/// (closure `rot^(11) = id`, unit edge lengths asserted exactly).
pub fn hull() -> Vec<Gp<Nq>> {
    rotodd::hull_odd::<K11>(11, "n11 hull")
}

/// `a = 2cos(pi/11) = (1,−2,−3,1,1)` — the K₁₁ slot of the rotation
/// constants (via the [`rotodd::Core`] instance; `a² = 2 + b`, `a > 0`
/// re-asserted exactly in the battery).
pub fn a11() -> K11 {
    <K11 as rotodd::Core>::a()
}

// =====================================================================
// Certified constants (design doc §2/§3; scratch
// `.scratch/24c-oddn-engine-design/record_constants.txt`).
// =====================================================================

/// `cos²(5 pi/11) = (1 + 2b + 3b² − b³ − b⁴)/4` — the (11,2) contact
/// angle (d = 5).
pub(super) fn cos2112() -> Nq {
    Nq::new(K11([1, 2, 3, -1, -1]), K11::ZERO, 4)
}
/// `cos²(3 pi/11) = (2 − 3b + b³)/4` — the (11,3) contact angle.
pub(super) fn cos2113() -> Nq {
    Nq::new(K11([2, -3, 0, 1, 0]), K11::ZERO, 4)
}
/// `cos²(pi/11) = (2 + b)/4` — the (11,4) contact angle (d = 1: `pi/n`,
/// exactly on the M163 EXTENSION guard boundary, the recorded
/// precedent of `{9/1}` and `{12/6}`).
pub(super) fn cos2114() -> Nq {
    Nq::new(K11([2, 1, 0, 0, 0]), K11::ZERO, 4)
}

/// The (11,2) pin `P = M₃ − R·M₀` (`|P|² = (2+9b+10b²−b³−2b⁴)/4`).
pub(super) fn pin112() -> Gp<Nq> {
    (
        Nq::new(K11([-1, 1, 5, 0, -1]), K11::ZERO, 4),
        Nq::new(K11::ZERO, K11([-2, -7, -3, 4, 2]), 4),
    )
}
/// The (11,3) pin `P = M₂ − R·M₀` (`|P|² = (4+14b+10b²−4b³−3b⁴)/4`).
pub(super) fn pin113() -> Gp<Nq> {
    (
        Nq::new(K11([1, 4, 4, -1, -1]), K11::ZERO, 4),
        Nq::new(K11::ZERO, K11([0, -6, -5, 3, 2]), 4),
    )
}
/// The (11,4) pin `P = M₁ − R·M₀` (`|P|² = (3+8b+9b²−3b³−3b⁴)/4`).
pub(super) fn pin114() -> Gp<Nq> {
    (
        Nq::new(K11([3, 3, 3, -1, -1]), K11::ZERO, 4),
        Nq::new(K11::ZERO, K11([0, -2, -3, 1, 1]), 4),
    )
}

/// The certified (11,2) window: `s_lo = 4+2b−8b²+2b⁴`, `s_hi =
/// −2−2b+8b²−2b⁴` — an exact MIDPOINT pin (`s_lo + s_hi = 2`, Δ = 3 ≥
/// 2, design §3). 123 sigma events, none persistent.
pub(super) fn win112() -> (Nq, Nq) {
    (
        Nq::new(K11([4, 2, -8, 0, 2]), K11::ZERO, 1),
        Nq::new(K11([-2, -2, 8, 0, -2]), K11::ZERO, 1),
    )
}
/// The certified (11,3) window (midpoint pin; its upper end is the
/// relabeling event shared with (11,4); 121 events).
pub(super) fn win113() -> (Nq, Nq) {
    (
        Nq::new(K11([12, 16, 14, -10, -2]), K11::ZERO, 23),
        Nq::new(K11([34, -16, -14, 10, 2]), K11::ZERO, 23),
    )
}
/// The certified (11,4) window — OFF-midpoint (Δ = 1), sharing `s_hi`
/// with (11,3): the adjacent c-windows meet at that relabeling event
/// (design §3). 123 events.
pub(super) fn win114() -> (Nq, Nq) {
    (
        Nq::new(K11([22, -8, 86, -2, -26]), K11::ZERO, 43),
        Nq::new(K11([34, -16, -14, 10, 2]), K11::ZERO, 23),
    )
}

// =====================================================================
// The three records and their certified placements.
// =====================================================================

use super::OutNode;

/// The `{11/5}` record: two pentadecagrams on the hendecagon, contact
/// angle `5*pi/11` (`cos²` of full degree 5 — `None` in the trait).
pub struct Monster115;

impl Medallion for Monster115 {
    const ID: &'static str = "hendecagon {11/5} monster star (24c)";
    const FIELD: Field = Field::K11;
    const SYM: usize = 11;
    const PATH: Absorption = Absorption::BoundaryInference;

    fn hull_f64() -> Vec<[f64; 2]> {
        hull().iter().map(|q| [q.0.fl(), q.1.fl()]).collect()
    }
    fn exit_rays_f64() -> Vec<(usize, [f64; 2])> {
        exit_rays5()
            .into_iter()
            .map(|(i, d)| (i, [d.0.fl(), d.1.fl()]))
            .collect()
    }
    // cos^2 of full degree 5: None (dec10/L10/n9 precedent), pinned
    // exactly as cos2112 in the battery below.
}

/// The `{11/3}` record: contact angle `3*pi/11`.
pub struct Monster113;

impl Medallion for Monster113 {
    const ID: &'static str = "hendecagon {11/3} monster star (24c)";
    const FIELD: Field = Field::K11;
    const SYM: usize = 11;
    const PATH: Absorption = Absorption::BoundaryInference;

    fn hull_f64() -> Vec<[f64; 2]> {
        hull().iter().map(|q| [q.0.fl(), q.1.fl()]).collect()
    }
    fn exit_rays_f64() -> Vec<(usize, [f64; 2])> {
        exit_rays3()
            .into_iter()
            .map(|(i, d)| (i, [d.0.fl(), d.1.fl()]))
            .collect()
    }
}

/// The `{11/1}` record: the midpoint hendecagon on the hendecagon,
/// contact angle `pi/11`. The step `d = 1` figure is the catalog
/// degeneracy recorded for `{9/1}`; `theta = pi/n` sits exactly on the
/// M163 EXTENSION guard boundary (design §8.13).
pub struct Monster111;

impl Medallion for Monster111 {
    const ID: &'static str = "hendecagon {11/1} monster star (24c)";
    const FIELD: Field = Field::K11;
    const SYM: usize = 11;
    const PATH: Absorption = Absorption::BoundaryInference;

    fn hull_f64() -> Vec<[f64; 2]> {
        hull().iter().map(|q| [q.0.fl(), q.1.fl()]).collect()
    }
    fn exit_rays_f64() -> Vec<(usize, [f64; 2])> {
        exit_rays1()
            .into_iter()
            .map(|(i, d)| (i, [d.0.fl(), d.1.fl()]))
            .collect()
    }
}

/// The `{11/5}` figure as contact-index pairs.
pub fn figure5() -> Vec<(usize, usize)> {
    star_figure(11, 5)
}
/// The `{11/3}` figure as contact-index pairs.
pub fn figure3() -> Vec<(usize, usize)> {
    star_figure(11, 3)
}
/// The `{11/1}` figure as contact-index pairs.
pub fn figure1() -> Vec<(usize, usize)> {
    star_figure(11, 1)
}

/// The 22 directed exit rays of the `{11/5}` record.
pub fn exit_rays5() -> Vec<(usize, Gp<Nq>)> {
    star_exits(&gmidpoints(&hull()), 5)
}
/// The 22 directed exit rays of the `{11/3}` record.
pub fn exit_rays3() -> Vec<(usize, Gp<Nq>)> {
    star_exits(&gmidpoints(&hull()), 3)
}
/// The 22 directed exit rays of the `{11/1}` record.
pub fn exit_rays1() -> Vec<(usize, Gp<Nq>)> {
    star_exits(&gmidpoints(&hull()), 1)
}

/// The certified (11,2) placement: `eAB = [4,5]`, `eBA = [9,10]`,
/// crossings `(A3,B0)` and `(A6,B8)` at the edge midpoints, `P = M3 -
/// R*M0`, window `(4+2b-8b^2+2b^4, -2-2b+8b^2-2b^4)` (midpoint pin)
/// with pin `sigma = 1`. Union 18 nodes, lens 8.
pub fn pair112() -> PairData<Nq> {
    let (lo, hi) = win112();
    rotodd::verify_pair_rot(
        &RotSpec {
            name: "(11,2)",
            n: 11,
            p: pin112(),
            d: 5,
            e_ab: vec![4, 5],
            e_ba: vec![9, 10],
            cross: [(3, 0), (6, 8)],
            cos2: cos2112(),
            s_lo: lo,
            s_hi: hi,
            // t0 is a genuine K11 element at Delta = 3 (deviation D3);
            // s0/ks per the certified line (design S3 table).
            t0: Nq::new(K11([0, -3, -3, 1, 1]), K11::ZERO, 1),
            k: Nq::new(K11([1, 6, 6, -2, -2]), K11::ZERO, 2),
            s0: Nq::new(K11([-1, -5, -2, 2, 1]), K11::ZERO, 1),
            ks: Nq::new(K11([3, 10, 4, -4, -2]), K11::ZERO, 2),
            union_cycle: vec![
                OutNode::X(0),
                OutNode::B(1),
                OutNode::B(2),
                OutNode::B(3),
                OutNode::B(4),
                OutNode::B(5),
                OutNode::B(6),
                OutNode::B(7),
                OutNode::B(8),
                OutNode::X(1),
                OutNode::A(7),
                OutNode::A(8),
                OutNode::A(9),
                OutNode::A(10),
                OutNode::A(0),
                OutNode::A(1),
                OutNode::A(2),
                OutNode::A(3),
            ],
            lens_cycle: vec![
                OutNode::X(0),
                OutNode::A(4),
                OutNode::A(5),
                OutNode::A(6),
                OutNode::X(1),
                OutNode::B(9),
                OutNode::B(10),
                OutNode::B(0),
            ],
        },
        &[
            // Above the window: only edge 4 (resp. 9) stays absorbed.
            WindowProbe {
                rho: hi.add(Nq::ONE.scaled_div(1, 64)),
                ca: 1,
                cb: 1,
                crossings: Some(2),
            },
            // Below the window: the count jumps to 3
            // (eAB = [4,5,6], eBA = [0,9,10]), still two crossings.
            WindowProbe {
                rho: lo.sub(Nq::ONE.scaled_div(1, 64)),
                ca: 3,
                cb: 3,
                crossings: Some(2),
            },
        ],
    )
}

/// The certified (11,3) placement: `eAB = [3,4,5]`, `eBA = [8,9,10]`,
/// crossings `(A2,B0)` and `(A6,B7)`, `P = M2 - R*M0`, window
/// `((12+16b+14b^2-10b^3-2b^4)/23, (34-16b-14b^2+10b^3+2b^4)/23)`
/// (midpoint pin). Union 16 nodes, lens 10.
pub fn pair113() -> PairData<Nq> {
    let (lo, hi) = win113();
    rotodd::verify_pair_rot(
        &RotSpec {
            name: "(11,3)",
            n: 11,
            p: pin113(),
            d: 3,
            e_ab: vec![3, 4, 5],
            e_ba: vec![8, 9, 10],
            cross: [(2, 0), (6, 7)],
            cos2: cos2113(),
            s_lo: lo,
            s_hi: hi,
            // Delta = 2: the n9 t0 = 0 pattern (V2 on the pi/n ray).
            t0: Nq::ZERO,
            k: Nq::new(K11([1, 0, 0, 0, 0]), K11::ZERO, 2),
            s0: Nq::new(K11([1, -2, -3, 1, 1]), K11::ZERO, 1),
            ks: Nq::new(K11([-1, 4, 6, -2, -2]), K11::ZERO, 2),
            union_cycle: vec![
                OutNode::X(0),
                OutNode::B(1),
                OutNode::B(2),
                OutNode::B(3),
                OutNode::B(4),
                OutNode::B(5),
                OutNode::B(6),
                OutNode::B(7),
                OutNode::X(1),
                OutNode::A(7),
                OutNode::A(8),
                OutNode::A(9),
                OutNode::A(10),
                OutNode::A(0),
                OutNode::A(1),
                OutNode::A(2),
            ],
            lens_cycle: vec![
                OutNode::X(0),
                OutNode::A(3),
                OutNode::A(4),
                OutNode::A(5),
                OutNode::A(6),
                OutNode::X(1),
                OutNode::B(8),
                OutNode::B(9),
                OutNode::B(10),
                OutNode::B(0),
            ],
        },
        &[
            // Above the window: edge 5 (resp. 10) drops out.
            WindowProbe {
                rho: hi.add(Nq::ONE.scaled_div(1, 64)),
                ca: 2,
                cb: 2,
                crossings: Some(2),
            },
            // Below the window: the count jumps to 4
            // (eAB = [3,4,5,6], eBA = [0,8,9,10]).
            WindowProbe {
                rho: lo.sub(Nq::ONE.scaled_div(1, 64)),
                ca: 4,
                cb: 4,
                crossings: Some(2),
            },
        ],
    )
}

/// The certified (11,4) placement (the Δ = 1 rung, deviation D1 —
/// clean, with a full window): `eAB = [2,3,4,5]`, `eBA = [7,8,9,10]`,
/// crossings `(A1,B0)` and `(A6,B6)`, `P = M1 - R*M0`, window
/// `((22-8b+86b^2-2b^3-26b^4)/43, (34-16b-14b^2+10b^3+2b^4)/23)` —
/// OFF-midpoint, upper end shared with (11,3). Union 14 nodes, lens
/// 12.
pub fn pair114() -> PairData<Nq> {
    let (lo, hi) = win114();
    rotodd::verify_pair_rot(
        &RotSpec {
            name: "(11,4)",
            n: 11,
            p: pin114(),
            d: 1,
            e_ab: vec![2, 3, 4, 5],
            e_ba: vec![7, 8, 9, 10],
            cross: [(1, 0), (6, 6)],
            cos2: cos2114(),
            s_lo: lo,
            s_hi: hi,
            // Delta = 1: the n9 t0 = 1, k = -1/2 pattern.
            t0: Nq::ONE,
            k: Nq::new(K11([-1, 0, 0, 0, 0]), K11::ZERO, 2),
            s0: Nq::new(K11([1, -2, -3, 1, 1]), K11::ZERO, 1),
            ks: Nq::new(K11([-1, 4, 6, -2, -2]), K11::ZERO, 2),
            union_cycle: vec![
                OutNode::X(0),
                OutNode::B(1),
                OutNode::B(2),
                OutNode::B(3),
                OutNode::B(4),
                OutNode::B(5),
                OutNode::B(6),
                OutNode::X(1),
                OutNode::A(7),
                OutNode::A(8),
                OutNode::A(9),
                OutNode::A(10),
                OutNode::A(0),
                OutNode::A(1),
            ],
            lens_cycle: vec![
                OutNode::X(0),
                OutNode::A(2),
                OutNode::A(3),
                OutNode::A(4),
                OutNode::A(5),
                OutNode::A(6),
                OutNode::X(1),
                OutNode::B(7),
                OutNode::B(8),
                OutNode::B(9),
                OutNode::B(10),
                OutNode::B(0),
            ],
        },
        &[
            // Above the window: edge 5 (resp. 10) drops out (eAB =
            // [2,3,4], the run slides with the shared relabeling
            // event), crossings stay two.
            WindowProbe {
                rho: hi.add(Nq::ONE.scaled_div(1, 64)),
                ca: 3,
                cb: 3,
                crossings: Some(2),
            },
            // Below the window: the absorbed run shifts to [3,4,5]
            // and FOUR extra transversals appear at the contacts
            // (six crossings) — the (9,3)-below pattern at the
            // Delta = 1 rung.
            WindowProbe {
                rho: lo.sub(Nq::ONE.scaled_div(1, 64)),
                ca: 3,
                cb: 3,
                crossings: Some(6),
            },
        ],
    )
}

// =====================================================================
// Tests — the M181 §8 battery, per mandate.
// =====================================================================

#[cfg(test)]
mod tests {
    use super::super::{gdot, hull_sanity};
    use super::*;
    use crate::medallion::monster::rotodd::{cheb, conj, Core, RotGeom};
    use std::cmp::Ordering;
    use std::collections::{BTreeMap, BTreeSet};

    /// Per-record certified data for the battery tables.
    struct Rec {
        name: &'static str,
        /// Band index c.
        c: usize,
        /// Ladder offset Delta = h - c.
        dl: usize,
        /// Canonical figure step d.
        d: usize,
        pin: fn() -> Gp<Nq>,
        /// `|P|^2` numerator over den 4.
        norm: [i64; 5],
        cos2: fn() -> Nq,
        win: fn() -> (Nq, Nq),
        /// t0 (den 1), k (den 2), s0 (den 1), ks (den 2).
        t0: [i64; 5],
        k: [i64; 5],
        s0: [i64; 5],
        ks: [i64; 5],
        /// sigma-event count of the ray census.
        events: usize,
        /// state `(cA, cB, crossings)` just outside the window.
        below: (usize, usize, usize),
        above: (usize, usize, usize),
        /// pin == window midpoint (the Delta >= 2 verdict).
        midpoint: bool,
        pair: fn() -> PairData<Nq>,
    }

    const REC112: Rec = Rec {
        name: "(11,2)",
        c: 2,
        dl: 3,
        d: 5,
        pin: pin112,
        norm: [2, 9, 10, -1, -2],
        cos2: cos2112,
        win: win112,
        t0: [0, -3, -3, 1, 1],
        k: [1, 6, 6, -2, -2],
        s0: [-1, -5, -2, 2, 1],
        ks: [3, 10, 4, -4, -2],
        events: 123,
        below: (3, 3, 2),
        above: (1, 1, 2),
        midpoint: true,
        pair: pair112,
    };
    const REC113: Rec = Rec {
        name: "(11,3)",
        c: 3,
        dl: 2,
        d: 3,
        pin: pin113,
        norm: [4, 14, 10, -4, -3],
        cos2: cos2113,
        win: win113,
        t0: [0, 0, 0, 0, 0],
        k: [1, 0, 0, 0, 0],
        s0: [1, -2, -3, 1, 1],
        ks: [-1, 4, 6, -2, -2],
        events: 121,
        below: (4, 4, 2),
        above: (2, 2, 2),
        midpoint: true,
        pair: pair113,
    };
    const REC114: Rec = Rec {
        name: "(11,4)",
        c: 4,
        dl: 1,
        d: 1,
        pin: pin114,
        norm: [3, 8, 9, -3, -3],
        cos2: cos2114,
        win: win114,
        t0: [1, 0, 0, 0, 0],
        k: [-1, 0, 0, 0, 0],
        s0: [1, -2, -3, 1, 1],
        ks: [-1, 4, 6, -2, -2],
        events: 123,
        below: (3, 3, 6),
        above: (3, 3, 2),
        midpoint: false,
        pair: pair114,
    };

    const RECS: [Rec; 3] = [REC112, REC113, REC114];

    /// Mandate 1 + 4 + 7 data: pin exactness `P = M_Delta - R*M_0`,
    /// `|P|^2`, the Chebyshev identity of `cos^2`, and the certified
    /// crossing-parameter line.
    #[test]
    fn certified_constants_are_exact() {
        // a = 2cos(pi/11) in K11: a^2 = 2 + b and a > 0.
        let a = a11();
        assert!(a.mul(a) == K11([2, 1, 0, 0, 0]), "a^2 != 2 + b");
        assert_eq!(a.sign(), Ordering::Greater);
        let m = gmidpoints(&hull());
        for r in &RECS {
            let p = (r.pin)();
            // P = M_Delta - R*M_0 exactly (mandate 1).
            let want = rotodd::pin_rot::<K11>(&m, r.dl, 0);
            assert!(p.0.eq_x(want.0) && p.1.eq_x(want.1), "{}: pin", r.name);
            // |P|^2 = norm/4 (mandate 1).
            assert!(
                gdot(p, p).eq_x(Nq::new(K11(r.norm), K11::ZERO, 4)),
                "{}: |P|^2",
                r.name
            );
            // cos^2 == (2 + C_d(b))/4 exactly (mandate 4).
            let cw = Nq::new(K11::ONE.scale(2).add(cheb::<K11>(r.d)), K11::ZERO, 4);
            assert!((r.cos2)().eq_x(cw), "{}: cos^2 != (2 + C_d)/4", r.name);
            // 0 < cos^2 < 1 (mandate 4).
            assert_eq!((r.cos2)().sgn(), Ordering::Greater, "{}: cos^2 <= 0", r.name);
            assert_eq!(
                (r.cos2)().sub(Nq::ONE).sgn(),
                Ordering::Less,
                "{}: cos^2 >= 1",
                r.name
            );
            // The d-rule on both crossings: d = 2(i - j) - 1 mod n,
            // canonical min(d, n - d) (mandate 4; the verifier re-checks
            // against the live crossing pairs).
            for (i, j) in [(r.dl, 0usize), (6, r.dl + 5)] {
                let dif = (2 * (i as i64 - j as i64) - 1).rem_euclid(11);
                assert_eq!(dif.min(11 - dif), r.d as i64, "{}: d-rule", r.name);
            }
            // The certified t/s line (mandate 7): t0 + k = s0 + ks =
            // 1/2, k and ks nonzero, direction recorded.
            let half = Nq::ONE.scaled_div(1, 2);
            let t0 = Nq::new(K11(r.t0), K11::ZERO, 1);
            let k = Nq::new(K11(r.k), K11::ZERO, 2);
            let s0 = Nq::new(K11(r.s0), K11::ZERO, 1);
            let ks = Nq::new(K11(r.ks), K11::ZERO, 2);
            assert!(t0.add(k).eq_x(half), "{}: t0 + k != 1/2", r.name);
            assert!(s0.add(ks).eq_x(half), "{}: s0 + ks != 1/2", r.name);
            assert_ne!(k.sgn(), Ordering::Equal, "{}: k = 0", r.name);
            assert_ne!(ks.sgn(), Ordering::Equal, "{}: ks = 0", r.name);
            // k > 0 for Delta >= 2, k < 0 for the Delta = 1 rung
            // (design S3).
            let want_k = if r.dl == 1 {
                Ordering::Less
            } else {
                Ordering::Greater
            };
            assert_eq!(k.sgn(), want_k, "{}: k direction", r.name);
        }
    }

    /// Mandate 5: the Galois orbit of `cos^2` under `b -> C_k(b)`,
    /// k = 1..5, has exactly 5 distinct values (full degree) — the
    /// certification that the trait's `(a, b, d, den)` tuple CANNOT
    /// carry it, hence `contact_cos2 = None` (asserted too).
    #[test]
    fn cos2_orbit_is_full_degree_and_trait_hides_it() {
        for r in &RECS {
            let u = ((r.cos2)()).u;
            let mut orbit: BTreeSet<[i64; 6]> = BTreeSet::new();
            for kk in 1..6usize {
                orbit.insert(conj::<K11>(u, kk).coords());
            }
            assert_eq!(orbit.len(), 5, "{}: cos^2 orbit", r.name);
        }
        assert_eq!(<Monster115 as Medallion>::contact_cos2(), None);
        assert_eq!(<Monster113 as Medallion>::contact_cos2(), None);
        assert_eq!(<Monster111 as Medallion>::contact_cos2(), None);
        for s in [
            <Monster115 as Medallion>::SYM,
            <Monster113 as Medallion>::SYM,
            <Monster111 as Medallion>::SYM,
        ] {
            assert_eq!(s, 11);
        }
        for f in [
            <Monster115 as Medallion>::FIELD,
            <Monster113 as Medallion>::FIELD,
            <Monster111 as Medallion>::FIELD,
        ] {
            assert_eq!(f, Field::K11);
        }
        assert_eq!(figure5().len(), 11);
        assert_eq!(figure3().len(), 11);
        assert_eq!(figure1().len(), 11);
        assert_eq!(exit_rays5().len(), 22);
        assert_eq!(exit_rays3().len(), 22);
        assert_eq!(exit_rays1().len(), 22);
    }

    /// Mandate 6 + 10 + 11: the sigma-ray census at the implemented
    /// field — exact event list (K₁₁ comparator), dedupe, no event at
    /// `sigma = 1`, the maximal window at the §3 bounds, the midpoint
    /// verdict, probes at the window midpoint and `1 ∓ 1/64` with a
    /// constant state key, bounding events real (the adjacent states
    /// differ), `dirty = empty` at the pin, and ZERO mixed-sign
    /// N-layer elements produced along the way.
    #[test]
    fn window_census_at_the_implemented_field() {
        let one = (Nq::ONE, Nq::ONE);
        for r in &RECS {
            let g = RotGeom::new(11, (r.pin)());
            // Mandate 10: the pin state is the clean monster state —
            // (c, c, the two certified crossing pairs, dirty = 0).
            let pin_key = g.state(&one);
            let h = 5usize;
            assert_eq!(
                pin_key,
                (
                    r.c,
                    r.c,
                    vec![(r.dl, 0usize), (h + 1, r.dl + h)],
                    0usize
                ),
                "{}: pin state",
                r.name
            );
            // The maximal window around sigma = 1 (mandate 6). The
            // walk itself re-verifies every bounding event (it only
            // stops where the adjacent state differs) and that no
            // event sits AT sigma = 1.
            let (lo, hi, n_events, persistent) = g.window(&pin_key);
            assert_eq!(n_events, r.events, "{}: event count", r.name);
            assert!(persistent.is_empty(), "{}: persistent incidences", r.name);
            // The window equals the §3 bounds EXACTLY.
            let (wlo, whi) = (r.win)();
            assert!(g.eq_sig_const(&lo, &wlo), "{}: s_lo", r.name);
            assert!(g.eq_sig_const(&hi, &whi), "{}: s_hi", r.name);
            // Midpoint verdict (the Delta >= 2 / Delta = 1 split).
            let is_mid = wlo.add(whi).eq_x(Nq::whole(2));
            assert_eq!(is_mid, r.midpoint, "{}: midpoint verdict", r.name);
            // Probes: the window midpoint and 1 ∓ 1/64 all keep the
            // pin state (mandate 6; `t != 1/2` at 1 ∓ 1/64 is
            // certified by the verifier's samples).
            assert_eq!(&g.state(&g.mid_sig(&lo, &hi)), &pin_key, "{}: mid", r.name);
            for d in [-1i64, 1] {
                let s = (Nq::ONE.scaled_div(64 + d, 64), Nq::ONE);
                assert_eq!(&g.state(&s), &pin_key, "{}: 1 {d}/64", r.name);
            }
            // Probes strictly outside: the state count pattern of the
            // (c, c, crossings) tabulation, and each bounding event is
            // REAL — the adjacent states differ across it.
            let (evs, _) = g.events();
            let li = evs
                .iter()
                .position(|(s, _)| g.cmp_sig(s, &lo) == Ordering::Equal)
                .unwrap();
            let hii = evs
                .iter()
                .position(|(s, _)| g.cmp_sig(s, &hi) == Ordering::Equal)
                .unwrap();
            assert!(li >= 1 && hii + 1 < evs.len(), "{}: interior bounds", r.name);
            let below = g.state(&g.mid_sig(&evs[li - 1].0, &lo));
            let above = g.state(&g.mid_sig(&hi, &evs[hii + 1].0));
            assert_ne!(&below, &pin_key, "{}: lower event not real", r.name);
            assert_ne!(&above, &pin_key, "{}: upper event not real", r.name);
            assert_eq!(
                (below.0, below.1, below.2.len(), below.3),
                (r.below.0, r.below.1, r.below.2, 0),
                "{}: below-window state",
                r.name
            );
            assert_eq!(
                (above.0, above.1, above.2.len(), above.3),
                (r.above.0, r.above.1, r.above.2, 0),
                "{}: above-window state",
                r.name
            );
            // Mandate 11: purity — ZERO mixed-sign N-layer elements
            // were signed anywhere above.
            assert_eq!(g.mixed(), 0, "{}: mixed-sign hits", r.name);
        }
    }

    /// Mandates 2, 3, 8, 9 run inside [`rotodd::verify_pair_rot`]; the
    /// per-entry data checks are here (the shipped n9 pattern).
    #[test]
    fn placements_are_the_certified_tables() {
        let h = hull();
        let m = gmidpoints(&h);
        for r in &RECS {
            let d = (r.pair)();
            let p = (r.pin)();
            assert!(p.0.eq_x(d.p.0) && p.1.eq_x(d.p.1), "{}: P", r.name);
            assert_eq!(d.e_ab.len(), r.c, "{}: c_A", r.name);
            assert_eq!(d.e_ba.len(), r.c, "{}: c_B", r.name);
            assert_eq!(d.e_ab, (r.dl + 1..=5).collect::<Vec<usize>>(), "{}: eAB", r.name);
            let want_ba: Vec<usize> = d.e_ab.iter().map(|i| (i + 5) % 11).collect();
            assert_eq!(d.e_ba, want_ba, "{}: eBA", r.name);
            // The crossing points are the A-midpoints M_Delta and
            // M_(h+1) (mandate 2).
            assert!(d.cross_pts[0].0.eq_x(m[r.dl].0) && d.cross_pts[0].1.eq_x(m[r.dl].1), "{}: X0", r.name);
            assert!(d.cross_pts[1].0.eq_x(m[6].0) && d.cross_pts[1].1.eq_x(m[6].1), "{}: X1", r.name);
            // The pinning identity from both crossings (mandate 1).
            for (i, j) in [(r.dl, 0usize), (6, r.dl + 5)] {
                let q = rotodd::pin_rot::<K11>(&m, i, j);
                assert!(q.0.eq_x(d.p.0) && q.1.eq_x(d.p.1), "{}: pin from ({i},{j})", r.name);
            }
            // Mandate 8: outline counts 2(n-c-1)+2 / 2c+4 and the
            // exposed-set sizes n - c - 1.
            assert_eq!(d.union_cycle.len(), 2 * (11 - r.c - 1) + 2, "{}: union", r.name);
            assert_eq!(d.lens_cycle.len(), 2 * r.c + 4, "{}: lens", r.name);
            assert_eq!(d.exposed_a.len(), 11 - r.c - 1, "{}: exposed A", r.name);
            assert_eq!(d.exposed_b.len(), 11 - r.c - 1, "{}: exposed B", r.name);
            // Mandate 8 (simplicity): no repeated vertex per side and
            // X0 != X1 as distinct crossing nodes.
            for cyc in [&d.union_cycle, &d.lens_cycle] {
                for side_a in [true, false] {
                    let vs: Vec<usize> = cyc
                        .iter()
                        .filter_map(|t| match (t, side_a) {
                            (OutNode::A(k), true) => Some(*k),
                            (OutNode::B(k), false) => Some(*k),
                            _ => None,
                        })
                        .collect();
                    let set: BTreeSet<usize> = vs.iter().copied().collect();
                    assert_eq!(set.len(), vs.len(), "{}: outline repeats a vertex", r.name);
                }
                assert_eq!(
                    cyc.iter().filter(|t| matches!(**t, OutNode::X(_))).count(),
                    2,
                    "{}: X0 != X1",
                    r.name
                );
            }
        }
    }

    /// Mandate 14: the full n² rotated pin census — the pin map is
    /// 2-to-1 with partner `(i,j) -> (j+h+1, i+h)`, `(n²+n)/2 = 66`
    /// distinct pins, every non-fixed class has exactly the two
    /// partner labels, and the fixed locus `i - j = h + 1` is the
    /// collinear edge-pair degeneracy (dirty contacts, no transversal
    /// crossing). The spike is the reference oracle (design §4).
    #[test]
    fn census_n_squared_pin_map() {
        let n = 11usize;
        let h = 5usize;
        let m = gmidpoints(&hull());
        // Canonical Nq key: the (u, v, den) integer tuple (canonical
        // form => exact value equality).
        let key = |q: Nq| (q.u.coords(), q.v.coords(), q.den);
        let mut classes: BTreeMap<
            ([i64; 6], [i64; 6], i64, [i64; 6], [i64; 6], i64),
            Vec<(usize, usize)>,
        > = BTreeMap::new();
        for i in 0..n {
            for j in 0..n {
                let p = rotodd::pin_rot::<K11>(&m, i, j);
                let ku = key(p.0);
                let kv = key(p.1);
                classes
                    .entry((ku.0, ku.1, ku.2, kv.0, kv.1, kv.2))
                    .or_default()
                    .push((i, j));
            }
        }
        assert_eq!(classes.len(), (n * n + n) / 2, "distinct pins");
        for (k, labels) in &classes {
            let _ = k;
            for &(i, j) in labels {
                let ip = (j + h + 1) % n;
                let jp = (i + h) % n;
                let p2 = rotodd::pin_rot::<K11>(&m, ip, jp);
                let p = rotodd::pin_rot::<K11>(&m, i, j);
                assert!(p.0.eq_x(p2.0) && p.1.eq_x(p2.1), "partner identity at ({i},{j})");
                assert!(
                    labels.contains(&(ip, jp)),
                    "partner not in class at ({i},{j})"
                );
            }
            let fixed = labels.len() == 1;
            if fixed {
                let (i, j) = labels[0];
                assert_eq!((i + n - j) % n, h + 1, "singleton off the fixed locus");
            } else {
                assert_eq!(labels.len(), 2, "class off the fixed locus not a pair");
            }
        }
        // The fixed locus: collinear degeneracy — dirty vertex-on-edge
        // contacts at every fixed pin (the 8-contact overlap: the
        // state key has crossings = empty and dirty > 0).
        for j in 0..n {
            let i = (j + h + 1) % n;
            let p = rotodd::pin_rot::<K11>(&m, i, j);
            let g = RotGeom::new(n, p);
            let st = g.state(&((Nq::ONE, Nq::ONE)));
            assert!(st.3 > 0, "fixed pin ({i},{j}) has no dirty contact");
            assert!(st.2.is_empty(), "fixed pin ({i},{j}) has crossings");
        }
    }

    /// Mandate 12 (M178 §4.2/§6.2): the N11 two-level sign rule — the
    /// mixed-sign PRODUCT rule `sgn(u + v w) = sgn(u) · sgn(u² − s·v²)`
    /// for `u`, `v` of opposite sign (`s = 2 − b = w²`), `sgn(u)` on a
    /// zero `v` and `sgn(v)` on a zero `u`, and the common sign when
    /// `u`, `v` share it (both slots carry `w > 0`). The MANDATED
    /// element `1 − 2w ≈ −0.1269 < 0` must sign Less via the product
    /// rule while a `sgn(u)`-only shortcut returns Greater (the G8
    /// case). The sweep below pins the rule on every small element,
    /// cross-checks each mixed sign against the float view, and pins
    /// the wrong-shortcut burden EXACTLY: the flipped set is the four
    /// elements `(±1, ∓2)`, `(±1, ∓3)` (`|u| < |v| w` with `w ≈
    /// 0.5635`), so no shortcut engine can pass.
    #[test]
    fn sgn_mixed_sign_product_rule_regression() {
        let s = K11::ONE.scale(2).sub(K11::B);
        // The mandated G8 element (M178 §6.2): 1 − 2w < 0, u² < s v².
        let x = Nq::new(K11::ONE, K11([-2, 0, 0, 0, 0]), 1);
        assert_eq!(x.sgn(), Ordering::Less, "1 − 2w must be negative");
        assert!(x.fl() < 0.0);
        assert_eq!(
            K11::ONE.mul(K11::ONE).sub(s.mul(K11([-2, 0, 0, 0, 0]).mul(K11([-2, 0, 0, 0, 0])))).sign(),
            Ordering::Less,
            "G8 must be a product-rule case (u² − s v² < 0, the shortcut's u² > s v² fails)"
        );
        // Sweep small elements; the expected sign recomputed from the
        // rule with exact K11 arithmetic.
        let mut flipped: Vec<(i64, i64)> = Vec::new();
        for cu in -3i64..=3 {
            for cv in -3i64..=3 {
                let u = K11::ONE.scale(cu);
                let v = K11::ONE.scale(cv);
                let x = Nq::new(u, v, 1);
                let su = u.sign();
                let sv = v.sign();
                let expect = match (su, sv) {
                    (Ordering::Equal, o) | (o, Ordering::Equal) => o,
                    (Ordering::Greater, Ordering::Greater)
                    | (Ordering::Less, Ordering::Less) => su,
                    (su, _) => {
                        let prod = u.mul(u).sub(s.mul(v.mul(v))).sign();
                        assert_ne!(prod, Ordering::Equal, "u² = s v² is impossible");
                        if su == prod {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    }
                };
                assert_eq!(x.sgn(), expect, "sgn({cu}, {cv})");
                // Float cross-check on every signed element.
                let fl = x.fl();
                assert!(
                    (fl > 0.0 && expect == Ordering::Greater)
                        || (fl < 0.0 && expect == Ordering::Less)
                        || fl == 0.0 && expect == Ordering::Equal,
                    "f64 cross-check ({cu}, {cv})"
                );
                if su != Ordering::Equal && sv != Ordering::Equal && su != sv {
                    if su != expect {
                        flipped.push((cu, cv));
                    }
                }
            }
        }
        flipped.sort_unstable();
        assert_eq!(
            flipped,
            vec![(-1, 2), (-1, 3), (1, -3), (1, -2)],
            "wrong-shortcut burden"
        );
    }

    #[test]
    fn hull_closes_and_is_normalized() {
        let h = hull();
        assert_eq!(h.len(), 11);
        hull_sanity("n11 hull", &h);
        // The float view is sane (emission path).
        for r in [
            <Monster115 as Medallion>::hull_f64(),
            <Monster113 as Medallion>::hull_f64(),
            <Monster111 as Medallion>::hull_f64(),
        ] {
            assert_eq!(r.len(), 11);
        }
    }
}
