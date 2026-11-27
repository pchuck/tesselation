//! The tridecagon monsters: the four band rungs of odd n = 13 on the
//! exact unit-edge regular tridecagon hull — `{13/6}` for (13,2),
//! `{13/5}` for (13,3), `{13/3}` for (13,4) and `{13/1}` for (13,5)
//! (design doc `docs/medallion-24c-odd-n-design.md` §2, the Δ = 4, 3,
//! 2, 1 rungs of the ladder Δ = h − c with h = 6).
//!
//! These are rotated pairs (`B = R·A + P`, `R` the rotation by
//! `pi/13`) exactly as shipped for n = 9 (see [`n9`](super::n9) for the
//! parity story); the engine is the shared generic [`rotodd`] over the
//! certified degree-6 field [`K13`]. The N-layer coordinate type is
//! `N = K₁₃(w)`, `w² = 2 − b` (`w = 2 sin(pi/13)`), with `a =
//! 2cos(pi/13) = (1,3,−3,−4,1,1)` INSIDE `K₁₃` (odd prime n puts
//! `zeta_2n` in `Q(zeta_n)`); `a² = 2 + b`, `a > 0`, re-asserted
//! exactly in the battery below.
//!
//! All four records have full-degree `cos²(d pi/13)` (the Galois orbit
//! of `cos²` has exactly 6 distinct conjugates — battery certified), so
//! `contact_cos2` returns `None` for all four (the n9/dec10/L11
//! precedent) with the exact tuple pinned here. The (13,2) figure step
//! is `d = 6` — the canonical fold of the raw `2(Δ−0)−1 = 7` (the
//! design doc deviation D2: at n = 13 the (13,2) rung is the FIRST
//! record whose raw step exceeds `n/2`, and `{13/6}` = `{13/7}` is the
//! same star polygon). Windows, crossing-parameter lines and outline
//! cycles are the M181 §2/§3 certified constants; the Δ = 2 rung has
//! `t0 = 0`, the Δ = 1 rung `t0 = 1` (the σ = 0 intersection is `V_Δ`,
//! on the `pi/n` ray at every n — deviation D3), while Δ = 4 and Δ = 3
//! carry genuine `K₁₃` `t₀` elements. See the [module docs](super) for
//! the catalog and the hub verifier machinery.

use super::{gmidpoints, star_exits, star_figure, FieldEl, Gp, PairData, WindowProbe};
use crate::fields::K13;
use crate::medallion::{Absorption, Field, Medallion};
use crate::medallion::monster::rotodd::{self, RotSpec};

/// `(u + v*w) / den` over [`K13`] — the N-layer coordinate type (the
/// `K13` instance of the generic [`rotodd::Nq`]).
pub type Nq = rotodd::Nq<K13>;

/// The record hull: the unit-edge regular tridecagon, normalized frame
/// (`V0 = origin`, edge 0 along +x), via the shared odd-n walk
/// (closure `rot^(13) = id`, unit edge lengths asserted exactly).
pub fn hull() -> Vec<Gp<Nq>> {
    rotodd::hull_odd::<K13>(13, "n13 hull")
}

/// `a = 2cos(pi/13) = (1,3,−3,−4,1,1)` — the K₁₃ slot of the rotation
/// constants (via the [`rotodd::Core`] instance; `a² = 2 + b`, `a > 0`
/// re-asserted exactly in the battery).
pub fn a13() -> K13 {
    <K13 as rotodd::Core>::a()
}

// =====================================================================
// Certified constants (design doc §2/§3; scratch
// `.scratch/24c-oddn-engine-design/record_constants.txt`).
// =====================================================================

/// `cos²(6 pi/13) = (1−3b+3b²+4b³−b⁴−b⁵)/4` — the (13,2) contact
/// angle (d = 6, the folded step).
pub(super) fn cos2132() -> Nq {
    Nq::new(K13([1, -3, 3, 4, -1, -1]), K13::ZERO, 4)
}
/// `cos²(5 pi/13) = (2+5b−5b³+b⁵)/4` — the (13,3) contact angle.
pub(super) fn cos2133() -> Nq {
    Nq::new(K13([2, 5, 0, -5, 0, 1]), K13::ZERO, 4)
}
/// `cos²(3 pi/13) = (2−3b+b³)/4` — the (13,4) contact angle.
pub(super) fn cos2134() -> Nq {
    Nq::new(K13([2, -3, 0, 1, 0, 0]), K13::ZERO, 4)
}
/// `cos²(pi/13) = (2 + b)/4` — the (13,5) contact angle (d = 1: `pi/n`,
/// exactly on the M163 EXTENSION guard boundary, the recorded
/// precedent of `{9/1}` and `{12/6}`).
pub(super) fn cos2135() -> Nq {
    Nq::new(K13([2, 1, 0, 0, 0, 0]), K13::ZERO, 4)
}

/// The (13,2) pin `P = M₄ − R·M₀` (`|P|² = (2−9b+6b²+13b³−2b⁵)/4`).
pub(super) fn pin132() -> Gp<Nq> {
    (
        Nq::new(K13([1, -7, 1, 6, 0, -1]), K13::ZERO, 4),
        Nq::new(K13::ZERO, K13([0, -1, -9, -4, 4, 2]), 4),
    )
}
/// The (13,3) pin `P = M₃ − R·M₀` (`|P|² = (2b+14b²+8b³−3b⁴−2b⁵)/4`).
pub(super) fn pin133() -> Gp<Nq> {
    (
        Nq::new(K13([-1, -4, 5, 5, -1, -1]), K13::ZERO, 4),
        Nq::new(K13::ZERO, K13([2, 0, -11, -5, 4, 2]), 4),
    )
}
/// The (13,4) pin `P = M₂ − R·M₀` (`|P|² = (2−b+14b²+11b³−4b⁴−3b⁵)/4`).
pub(super) fn pin134() -> Gp<Nq> {
    (
        Nq::new(K13([1, -1, 4, 4, -1, -1]), K13::ZERO, 4),
        Nq::new(K13::ZERO, K13([2, 4, -9, -7, 3, 2]), 4),
    )
}
/// The (13,5) pin `P = M₁ − R·M₀` (`|P|² = (3−7b+9b²+12b³−3b⁴−3b⁵)/4`).
pub(super) fn pin135() -> Gp<Nq> {
    (
        Nq::new(K13([3, -2, 3, 4, -1, -1]), K13::ZERO, 4),
        Nq::new(K13::ZERO, K13([0, 3, -3, -4, 1, 1]), 4),
    )
}

/// The certified (13,2) window: `s_lo = (272+304b−72b²−426b³+22b⁴+
/// 96b⁵)/131`, `s_hi = (−10−304b+72b²+426b³−22b⁴−96b⁵)/131` — an exact
/// MIDPOINT pin (`s_lo + s_hi = 2`, Δ = 4 ≥ 2, design §3). 175 sigma
/// events, none persistent.
pub(super) fn win132() -> (Nq, Nq) {
    (
        Nq::new(K13([272, 304, -72, -426, 22, 96]), K13::ZERO, 131),
        Nq::new(K13([-10, -304, 72, 426, -22, -96]), K13::ZERO, 131),
    )
}
/// The certified (13,3) window (midpoint pin; 175 events).
pub(super) fn win133() -> (Nq, Nq) {
    (
        Nq::new(K13([1442, -516, -218, 426, 14, -96]), K13::ZERO, 911),
        Nq::new(K13([380, 516, 218, -426, -14, 96]), K13::ZERO, 911),
    )
}
/// The certified (13,4) window (midpoint pin; 173 events).
pub(super) fn win134() -> (Nq, Nq) {
    (
        Nq::new(K13([52, -110, -22, 160, 2, -38]), K13::ZERO, 53),
        Nq::new(K13([54, 110, 22, -160, -2, 38]), K13::ZERO, 53),
    )
}
/// The certified (13,5) window — OFF-midpoint (Δ = 1), sharing `s_hi`
/// with (13,4): the adjacent c-windows meet at that relabeling event
/// (design §3). 173 events.
pub(super) fn win135() -> (Nq, Nq) {
    (
        Nq::new(K13([78, -98, 62, 96, -26, -18]), K13::ZERO, 79),
        Nq::new(K13([54, 110, 22, -160, -2, 38]), K13::ZERO, 53),
    )
}

// =====================================================================
// The four records and their certified placements.
// =====================================================================

use super::OutNode;

/// The `{13/6}` record: two tridecagrams on the tridecagon, contact
/// angle `6*pi/13` (raw step 7 folded — deviation D2; `cos²` of full
/// degree 6, so `None` in the trait).
pub struct Monster136;

impl Medallion for Monster136 {
    const ID: &'static str = "tridecagon {13/6} monster star (24c)";
    const FIELD: Field = Field::K13;
    const SYM: usize = 13;
    const PATH: Absorption = Absorption::BoundaryInference;

    fn hull_f64() -> Vec<[f64; 2]> {
        hull().iter().map(|q| [q.0.fl(), q.1.fl()]).collect()
    }
    fn exit_rays_f64() -> Vec<(usize, [f64; 2])> {
        exit_rays6()
            .into_iter()
            .map(|(i, d)| (i, [d.0.fl(), d.1.fl()]))
            .collect()
    }
    // cos^2 of full degree 6: None (n9/dec10 precedent), pinned
    // exactly as cos2132 in the battery below.
}

/// The `{13/5}` record: contact angle `5*pi/13`.
pub struct Monster135;

impl Medallion for Monster135 {
    const ID: &'static str = "tridecagon {13/5} monster star (24c)";
    const FIELD: Field = Field::K13;
    const SYM: usize = 13;
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
}

/// The `{13/3}` record: contact angle `3*pi/13`.
pub struct Monster133;

impl Medallion for Monster133 {
    const ID: &'static str = "tridecagon {13/3} monster star (24c)";
    const FIELD: Field = Field::K13;
    const SYM: usize = 13;
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

/// The `{13/1}` record: the midpoint tridecagon on the tridecagon,
/// contact angle `pi/13`. The step `d = 1` figure is the catalog
/// degeneracy recorded for `{9/1}`; `theta = pi/n` sits exactly on the
/// M163 EXTENSION guard boundary (design §8.13).
pub struct Monster131;

impl Medallion for Monster131 {
    const ID: &'static str = "tridecagon {13/1} monster star (24c)";
    const FIELD: Field = Field::K13;
    const SYM: usize = 13;
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

/// The `{13/6}` figure as contact-index pairs.
pub fn figure6() -> Vec<(usize, usize)> {
    star_figure(13, 6)
}
/// The `{13/5}` figure as contact-index pairs.
pub fn figure5() -> Vec<(usize, usize)> {
    star_figure(13, 5)
}
/// The `{13/3}` figure as contact-index pairs.
pub fn figure3() -> Vec<(usize, usize)> {
    star_figure(13, 3)
}
/// The `{13/1}` figure as contact-index pairs.
pub fn figure1() -> Vec<(usize, usize)> {
    star_figure(13, 1)
}

/// The 26 directed exit rays of the `{13/6}` record.
pub fn exit_rays6() -> Vec<(usize, Gp<Nq>)> {
    star_exits(&gmidpoints(&hull()), 6)
}
/// The 26 directed exit rays of the `{13/5}` record.
pub fn exit_rays5() -> Vec<(usize, Gp<Nq>)> {
    star_exits(&gmidpoints(&hull()), 5)
}
/// The 26 directed exit rays of the `{13/3}` record.
pub fn exit_rays3() -> Vec<(usize, Gp<Nq>)> {
    star_exits(&gmidpoints(&hull()), 3)
}
/// The 26 directed exit rays of the `{13/1}` record.
pub fn exit_rays1() -> Vec<(usize, Gp<Nq>)> {
    star_exits(&gmidpoints(&hull()), 1)
}

/// The certified (13,2) placement: `eAB = [5,6]`, `eBA = [11,12]`,
/// crossings `(A4,B0)` and `(A7,B10)` at the edge midpoints, `P = M4 -
/// R*M0`, window `((272+304b-72b^2-426b^3+22b^4+96b^5)/131,
/// (-10-304b+72b^2+426b^3-22b^4-96b^5)/131)` (midpoint pin) with pin
/// `sigma = 1`. Union 22 nodes, lens 8.
pub fn pair132() -> PairData<Nq> {
    let (lo, hi) = win132();
    rotodd::verify_pair_rot(
        &RotSpec {
            name: "(13,2)",
            n: 13,
            p: pin132(),
            d: 6,
            e_ab: vec![5, 6],
            e_ba: vec![11, 12],
            cross: [(4, 0), (7, 10)],
            cos2: cos2132(),
            s_lo: lo,
            s_hi: hi,
            // t0 is a genuine K13 element at Delta = 4 (deviation D3);
            // s0/ks per the certified line (design S3 table).
            t0: Nq::new(K13([0, 0, 6, 3, -2, -1]), K13::ZERO, 1),
            k: Nq::new(K13([1, 0, -12, -6, 4, 2]), K13::ZERO, 2),
            s0: Nq::new(K13([-1, 4, 4, -1, -1, 0]), K13::ZERO, 1),
            ks: Nq::new(K13([3, -8, -8, 2, 2, 0]), K13::ZERO, 2),
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
                OutNode::B(9),
                OutNode::B(10),
                OutNode::X(1),
                OutNode::A(8),
                OutNode::A(9),
                OutNode::A(10),
                OutNode::A(11),
                OutNode::A(12),
                OutNode::A(0),
                OutNode::A(1),
                OutNode::A(2),
                OutNode::A(3),
                OutNode::A(4),
            ],
            lens_cycle: vec![
                OutNode::X(0),
                OutNode::A(5),
                OutNode::A(6),
                OutNode::A(7),
                OutNode::X(1),
                OutNode::B(11),
                OutNode::B(12),
                OutNode::B(0),
            ],
        },
        &[
            // Above the window: only edge 6 (resp. 12) stays absorbed.
            WindowProbe {
                rho: hi.add(Nq::ONE.scaled_div(1, 64)),
                ca: 1,
                cb: 1,
                crossings: Some(2),
            },
            // Below the window: the count jumps to 3
            // (eAB = [5,6,7], eBA = [0,11,12]), still two crossings.
            WindowProbe {
                rho: lo.sub(Nq::ONE.scaled_div(1, 64)),
                ca: 3,
                cb: 3,
                crossings: Some(2),
            },
        ],
    )
}

/// The certified (13,3) placement: `eAB = [4,5,6]`, `eBA = [10,11,12]`,
/// crossings `(A3,B0)` and `(A7,B9)`, `P = M3 - R*M0`, window
/// `((1442-516b-218b^2+426b^3+14b^4-96b^5)/911,
/// (380+516b+218b^2-426b^3-14b^4+96b^5)/911)` (midpoint pin). Union 20
/// nodes, lens 10.
pub fn pair133() -> PairData<Nq> {
    let (lo, hi) = win133();
    rotodd::verify_pair_rot(
        &RotSpec {
            name: "(13,3)",
            n: 13,
            p: pin133(),
            d: 5,
            e_ab: vec![4, 5, 6],
            e_ba: vec![10, 11, 12],
            cross: [(3, 0), (7, 9)],
            cos2: cos2133(),
            s_lo: lo,
            s_hi: hi,
            // Delta = 3: also a genuine K13 t0 line.
            t0: Nq::new(K13([2, 0, -4, 0, 1, 0]), K13::ZERO, 1),
            k: Nq::new(K13([-3, 0, 8, 0, -2, 0]), K13::ZERO, 2),
            s0: Nq::new(K13([0, -2, -3, 1, 1, 0]), K13::ZERO, 1),
            ks: Nq::new(K13([1, 4, 6, -2, -2, 0]), K13::ZERO, 2),
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
                OutNode::B(9),
                OutNode::X(1),
                OutNode::A(8),
                OutNode::A(9),
                OutNode::A(10),
                OutNode::A(11),
                OutNode::A(12),
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
                OutNode::A(7),
                OutNode::X(1),
                OutNode::B(10),
                OutNode::B(11),
                OutNode::B(12),
                OutNode::B(0),
            ],
        },
        &[
            // Above the window: edge 6 (resp. 12) drops out.
            WindowProbe {
                rho: hi.add(Nq::ONE.scaled_div(1, 64)),
                ca: 2,
                cb: 2,
                crossings: Some(2),
            },
            // Below the window: the count jumps to 4
            // (eAB = [4,5,6,7], eBA = [0,10,11,12]).
            WindowProbe {
                rho: lo.sub(Nq::ONE.scaled_div(1, 64)),
                ca: 4,
                cb: 4,
                crossings: Some(2),
            },
        ],
    )
}

/// The certified (13,4) placement: `eAB = [3,4,5,6]`, `eBA =
/// [9,10,11,12]`, crossings `(A2,B0)` and `(A7,B8)`, `P = M2 - R*M0`,
/// window `((52-110b-22b^2+160b^3+2b^4-38b^5)/53,
/// (54+110b+22b^2-160b^3-2b^4+38b^5)/53)` (midpoint pin). Union 18
/// nodes, lens 12.
pub fn pair134() -> PairData<Nq> {
    let (lo, hi) = win134();
    rotodd::verify_pair_rot(
        &RotSpec {
            name: "(13,4)",
            n: 13,
            p: pin134(),
            d: 3,
            e_ab: vec![3, 4, 5, 6],
            e_ba: vec![9, 10, 11, 12],
            cross: [(2, 0), (7, 8)],
            cos2: cos2134(),
            s_lo: lo,
            s_hi: hi,
            // Delta = 2: the n9 t0 = 0 pattern (V2 on the pi/n ray).
            t0: Nq::ZERO,
            k: Nq::new(K13([1, 0, 0, 0, 0, 0]), K13::ZERO, 2),
            s0: Nq::new(K13([1, 3, -3, -4, 1, 1]), K13::ZERO, 1),
            ks: Nq::new(K13([-1, -6, 6, 8, -2, -2]), K13::ZERO, 2),
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
                OutNode::A(8),
                OutNode::A(9),
                OutNode::A(10),
                OutNode::A(11),
                OutNode::A(12),
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
                OutNode::A(7),
                OutNode::X(1),
                OutNode::B(9),
                OutNode::B(10),
                OutNode::B(11),
                OutNode::B(12),
                OutNode::B(0),
            ],
        },
        &[
            // Above the window: edge 6 (resp. 12) drops out.
            WindowProbe {
                rho: hi.add(Nq::ONE.scaled_div(1, 64)),
                ca: 3,
                cb: 3,
                crossings: Some(2),
            },
            // Below the window: the count jumps to 5
            // (eAB = [3,4,5,6,7], eBA = [0,9,10,11,12]).
            WindowProbe {
                rho: lo.sub(Nq::ONE.scaled_div(1, 64)),
                ca: 5,
                cb: 5,
                crossings: Some(2),
            },
        ],
    )
}

/// The certified (13,5) placement (the Δ = 1 rung, deviation D1 —
/// clean, with a full window): `eAB = [2,3,4,5,6]`, `eBA =
/// [8,9,10,11,12]`, crossings `(A1,B0)` and `(A7,B7)`, `P = M1 -
/// R*M0`, window `((78-98b+62b^2+96b^3-26b^4-18b^5)/79,
/// (54+110b+22b^2-160b^3-2b^4+38b^5)/53)` — OFF-midpoint, upper end
/// shared with (13,4). Union 16 nodes, lens 14.
pub fn pair135() -> PairData<Nq> {
    let (lo, hi) = win135();
    rotodd::verify_pair_rot(
        &RotSpec {
            name: "(13,5)",
            n: 13,
            p: pin135(),
            d: 1,
            e_ab: vec![2, 3, 4, 5, 6],
            e_ba: vec![8, 9, 10, 11, 12],
            cross: [(1, 0), (7, 7)],
            cos2: cos2135(),
            s_lo: lo,
            s_hi: hi,
            // Delta = 1: the n9 t0 = 1, k = -1/2 pattern.
            t0: Nq::ONE,
            k: Nq::new(K13([-1, 0, 0, 0, 0, 0]), K13::ZERO, 2),
            s0: Nq::new(K13([1, 3, -3, -4, 1, 1]), K13::ZERO, 1),
            ks: Nq::new(K13([-1, -6, 6, 8, -2, -2]), K13::ZERO, 2),
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
                OutNode::A(8),
                OutNode::A(9),
                OutNode::A(10),
                OutNode::A(11),
                OutNode::A(12),
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
                OutNode::A(7),
                OutNode::X(1),
                OutNode::B(8),
                OutNode::B(9),
                OutNode::B(10),
                OutNode::B(11),
                OutNode::B(12),
                OutNode::B(0),
            ],
        },
        &[
            // Above the window: the absorbed run slides to [2,3,4,5]
            // (eBA = [8,9,10,11]) at the shared relabeling event,
            // crossings stay two.
            WindowProbe {
                rho: hi.add(Nq::ONE.scaled_div(1, 64)),
                ca: 4,
                cb: 4,
                crossings: Some(2),
            },
            // Below the window: the absorbed run shifts to [3,4,5,6]
            // and FOUR extra transversals appear at the contacts
            // (six crossings) — the (9,3)-below pattern at the
            // Delta = 1 rung.
            WindowProbe {
                rho: lo.sub(Nq::ONE.scaled_div(1, 64)),
                ca: 4,
                cb: 4,
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
        norm: [i64; 6],
        cos2: fn() -> Nq,
        win: fn() -> (Nq, Nq),
        /// t0 (den 1), k (den 2), s0 (den 1), ks (den 2).
        t0: [i64; 6],
        k: [i64; 6],
        s0: [i64; 6],
        ks: [i64; 6],
        /// sigma-event count of the ray census.
        events: usize,
        /// state `(cA, cB, crossings)` just outside the window.
        below: (usize, usize, usize),
        above: (usize, usize, usize),
        /// pin == window midpoint (the Delta >= 2 verdict).
        midpoint: bool,
        pair: fn() -> PairData<Nq>,
    }

    const REC132: Rec = Rec {
        name: "(13,2)",
        c: 2,
        dl: 4,
        d: 6,
        pin: pin132,
        norm: [2, -9, 6, 13, 0, -2],
        cos2: cos2132,
        win: win132,
        t0: [0, 0, 6, 3, -2, -1],
        k: [1, 0, -12, -6, 4, 2],
        s0: [-1, 4, 4, -1, -1, 0],
        ks: [3, -8, -8, 2, 2, 0],
        events: 175,
        below: (3, 3, 2),
        above: (1, 1, 2),
        midpoint: true,
        pair: pair132,
    };
    const REC133: Rec = Rec {
        name: "(13,3)",
        c: 3,
        dl: 3,
        d: 5,
        pin: pin133,
        norm: [0, 2, 14, 8, -3, -2],
        cos2: cos2133,
        win: win133,
        t0: [2, 0, -4, 0, 1, 0],
        k: [-3, 0, 8, 0, -2, 0],
        s0: [0, -2, -3, 1, 1, 0],
        ks: [1, 4, 6, -2, -2, 0],
        events: 175,
        below: (4, 4, 2),
        above: (2, 2, 2),
        midpoint: true,
        pair: pair133,
    };
    const REC134: Rec = Rec {
        name: "(13,4)",
        c: 4,
        dl: 2,
        d: 3,
        pin: pin134,
        norm: [2, -1, 14, 11, -4, -3],
        cos2: cos2134,
        win: win134,
        t0: [0, 0, 0, 0, 0, 0],
        k: [1, 0, 0, 0, 0, 0],
        s0: [1, 3, -3, -4, 1, 1],
        ks: [-1, -6, 6, 8, -2, -2],
        events: 173,
        below: (5, 5, 2),
        above: (3, 3, 2),
        midpoint: true,
        pair: pair134,
    };
    const REC135: Rec = Rec {
        name: "(13,5)",
        c: 5,
        dl: 1,
        d: 1,
        pin: pin135,
        norm: [3, -7, 9, 12, -3, -3],
        cos2: cos2135,
        win: win135,
        t0: [1, 0, 0, 0, 0, 0],
        k: [-1, 0, 0, 0, 0, 0],
        s0: [1, 3, -3, -4, 1, 1],
        ks: [-1, -6, 6, 8, -2, -2],
        events: 173,
        below: (4, 4, 6),
        above: (4, 4, 2),
        midpoint: false,
        pair: pair135,
    };

    const RECS: [Rec; 4] = [REC132, REC133, REC134, REC135];

    /// Mandate 1 + 4 + 7 data: pin exactness `P = M_Delta - R*M_0`,
    /// `|P|^2`, the Chebyshev identity of `cos^2`, and the certified
    /// crossing-parameter line.
    #[test]
    fn certified_constants_are_exact() {
        // a = 2cos(pi/13) in K13: a^2 = 2 + b and a > 0.
        let a = a13();
        assert!(a.mul(a) == K13([2, 1, 0, 0, 0, 0]), "a^2 != 2 + b");
        assert_eq!(a.sign(), Ordering::Greater);
        let m = gmidpoints(&hull());
        for r in &RECS {
            let p = (r.pin)();
            // P = M_Delta - R*M_0 exactly (mandate 1).
            let want = rotodd::pin_rot::<K13>(&m, r.dl, 0);
            assert!(p.0.eq_x(want.0) && p.1.eq_x(want.1), "{}: pin", r.name);
            // |P|^2 = norm/4 (mandate 1).
            assert!(
                gdot(p, p).eq_x(Nq::new(K13(r.norm), K13::ZERO, 4)),
                "{}: |P|^2",
                r.name
            );
            // cos^2 == (2 + C_d(b))/4 exactly (mandate 4).
            let cw = Nq::new(K13::ONE.scale(2).add(cheb::<K13>(r.d)), K13::ZERO, 4);
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
            // canonical min(d, n - d) (mandate 4; at (13,2) this folds
            // the raw 7 to 6 — deviation D2).
            for (i, j) in [(r.dl, 0usize), (7, r.dl + 6)] {
                let dif = (2 * (i as i64 - j as i64) - 1).rem_euclid(13);
                assert_eq!(dif.min(13 - dif), r.d as i64, "{}: d-rule", r.name);
            }
            // The certified t/s line (mandate 7): t0 + k = s0 + ks =
            // 1/2, k and ks nonzero, direction recorded.
            let half = Nq::ONE.scaled_div(1, 2);
            let t0 = Nq::new(K13(r.t0), K13::ZERO, 1);
            let k = Nq::new(K13(r.k), K13::ZERO, 2);
            let s0 = Nq::new(K13(r.s0), K13::ZERO, 1);
            let ks = Nq::new(K13(r.ks), K13::ZERO, 2);
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
    /// k = 1..6, has exactly 6 distinct values (full degree) — the
    /// certification that the trait's `(a, b, d, den)` tuple CANNOT
    /// carry it, hence `contact_cos2 = None` (asserted too).
    #[test]
    fn cos2_orbit_is_full_degree_and_trait_hides_it() {
        for r in &RECS {
            let u = ((r.cos2)()).u;
            let mut orbit: BTreeSet<[i64; 6]> = BTreeSet::new();
            for kk in 1..7usize {
                orbit.insert(conj::<K13>(u, kk).coords());
            }
            assert_eq!(orbit.len(), 6, "{}: cos^2 orbit", r.name);
        }
        assert_eq!(<Monster136 as Medallion>::contact_cos2(), None);
        assert_eq!(<Monster135 as Medallion>::contact_cos2(), None);
        assert_eq!(<Monster133 as Medallion>::contact_cos2(), None);
        assert_eq!(<Monster131 as Medallion>::contact_cos2(), None);
        for s in [
            <Monster136 as Medallion>::SYM,
            <Monster135 as Medallion>::SYM,
            <Monster133 as Medallion>::SYM,
            <Monster131 as Medallion>::SYM,
        ] {
            assert_eq!(s, 13);
        }
        for f in [
            <Monster136 as Medallion>::FIELD,
            <Monster135 as Medallion>::FIELD,
            <Monster133 as Medallion>::FIELD,
            <Monster131 as Medallion>::FIELD,
        ] {
            assert_eq!(f, Field::K13);
        }
        assert_eq!(figure6().len(), 13);
        assert_eq!(figure5().len(), 13);
        assert_eq!(figure3().len(), 13);
        assert_eq!(figure1().len(), 13);
        assert_eq!(exit_rays6().len(), 26);
        assert_eq!(exit_rays5().len(), 26);
        assert_eq!(exit_rays3().len(), 26);
        assert_eq!(exit_rays1().len(), 26);
    }

    /// Mandate 6 + 10 + 11: the sigma-ray census at the implemented
    /// field — exact event list (K₁₃ comparator), dedupe, no event at
    /// `sigma = 1`, the maximal window at the §3 bounds, the midpoint
    /// verdict, probes at the window midpoint and `1 ∓ 1/64` with a
    /// constant state key, bounding events real (the adjacent states
    /// differ), `dirty = empty` at the pin, and ZERO mixed-sign
    /// N-layer elements produced along the way.
    #[test]
    fn window_census_at_the_implemented_field() {
        let one = (Nq::ONE, Nq::ONE);
        for r in &RECS {
            let g = RotGeom::new(13, (r.pin)());
            // Mandate 10: the pin state is the clean monster state —
            // (c, c, the two certified crossing pairs, dirty = 0).
            let pin_key = g.state(&one);
            let h = 6usize;
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
            assert_eq!(d.e_ab, (r.dl + 1..=6).collect::<Vec<usize>>(), "{}: eAB", r.name);
            let want_ba: Vec<usize> = d.e_ab.iter().map(|i| (i + 6) % 13).collect();
            assert_eq!(d.e_ba, want_ba, "{}: eBA", r.name);
            // The crossing points are the A-midpoints M_Delta and
            // M_(h+1) (mandate 2).
            assert!(
                d.cross_pts[0].0.eq_x(m[r.dl].0) && d.cross_pts[0].1.eq_x(m[r.dl].1),
                "{}: X0",
                r.name
            );
            assert!(d.cross_pts[1].0.eq_x(m[7].0) && d.cross_pts[1].1.eq_x(m[7].1), "{}: X1", r.name);
            // The pinning identity from both crossings (mandate 1).
            for (i, j) in [(r.dl, 0usize), (7, r.dl + 6)] {
                let q = rotodd::pin_rot::<K13>(&m, i, j);
                assert!(q.0.eq_x(d.p.0) && q.1.eq_x(d.p.1), "{}: pin from ({i},{j})", r.name);
            }
            // Mandate 8: outline counts 2(n-c-1)+2 / 2c+4 and the
            // exposed-set sizes n - c - 1.
            assert_eq!(d.union_cycle.len(), 2 * (13 - r.c - 1) + 2, "{}: union", r.name);
            assert_eq!(d.lens_cycle.len(), 2 * r.c + 4, "{}: lens", r.name);
            assert_eq!(d.exposed_a.len(), 13 - r.c - 1, "{}: exposed A", r.name);
            assert_eq!(d.exposed_b.len(), 13 - r.c - 1, "{}: exposed B", r.name);
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
    /// 2-to-1 with partner `(i,j) -> (j+h+1, i+h)`, `(n²+n)/2 = 91`
    /// distinct pins, every non-fixed class has exactly the two
    /// partner labels, and the fixed locus `i - j = h + 1` is the
    /// collinear edge-pair degeneracy (dirty contacts, no transversal
    /// crossing). The spike is the reference oracle (design §4).
    #[test]
    fn census_n_squared_pin_map() {
        let n = 13usize;
        let h = 6usize;
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
                let p = rotodd::pin_rot::<K13>(&m, i, j);
                let ku = key(p.0);
                let kv = key(p.1);
                classes
                    .entry((ku.0, ku.1, ku.2, kv.0, kv.1, kv.2))
                    .or_default()
                    .push((i, j));
            }
        }
        assert_eq!(classes.len(), (n * n + n) / 2, "distinct pins");
        for (_k, labels) in &classes {
            for &(i, j) in labels {
                let ip = (j + h + 1) % n;
                let jp = (i + h) % n;
                let p2 = rotodd::pin_rot::<K13>(&m, ip, jp);
                let p = rotodd::pin_rot::<K13>(&m, i, j);
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
            let p = rotodd::pin_rot::<K13>(&m, i, j);
            let g = RotGeom::new(n, p);
            let st = g.state(&((Nq::ONE, Nq::ONE)));
            assert!(st.3 > 0, "fixed pin ({i},{j}) has no dirty contact");
            assert!(st.2.is_empty(), "fixed pin ({i},{j}) has crossings");
        }
    }

    /// Mandate 12 (M179 §4.3/§6.3): the N13 two-level sign rule — the
    /// mixed-sign PRODUCT rule `sgn(u) · sgn(u² − s·v²)`, never the
    /// `sgn(u)` shortcut. At n = 13 the wrong-shortcut burden sits on
    /// `(1,−3)`, `(2,−5)`, `(3,−7)`, `(−1,3)` (`u² < s·v²` there),
    /// while `(1,±2)` are BOUNDARY CONTROLS (`w ≈ 0.4786 < 1/2` makes
    /// `1 − 2w ≈ +0.043 > 0`: `u² > s·v²`, the shortcut accidentally
    /// right). The sweep pins the rule on every small element,
    /// cross-checks each sign against the float view, and pins the
    /// wrong-shortcut burden exactly — so no shortcut engine can pass.
    #[test]
    fn sgn_mixed_sign_product_rule_regression() {
        let s = K13::ONE.scale(2).sub(K13::B);
        // The four mandated wrong-shortcut elements (M179 §4.3).
        let wrong: [([i64; 6], [i64; 6]); 4] = [
            ([1, 0, 0, 0, 0, 0], [-3, 0, 0, 0, 0, 0]),
            ([2, 0, 0, 0, 0, 0], [-5, 0, 0, 0, 0, 0]),
            ([3, 0, 0, 0, 0, 0], [-7, 0, 0, 0, 0, 0]),
            ([-1, 0, 0, 0, 0, 0], [3, 0, 0, 0, 0, 0]),
        ];
        for (cu, cv) in wrong {
            let u = K13(cu);
            let v = K13(cv);
            let x = Nq::new(u, v, 1);
            // u^2 < s v^2: the product factor is negative, so the
            // product rule FLIPS sgn(u) here (n = 13: s = 2 - b ~
            // 0.3175, s v^2 > u^2 on all four elements).
            let gap = u.mul(u).sub(s.mul(v.mul(v))).sign();
            assert_eq!(gap, Ordering::Less, "not a product-rule element");
            let shortcut = u.sign();
            let real = x.sgn();
            assert_ne!(real, shortcut, "shortcut must be WRONG on this element");
            assert!(
                (real == Ordering::Less && x.fl() < 0.0)
                    || (real == Ordering::Greater && x.fl() > 0.0),
                "f64 cross-check"
            );
        }
        // The boundary controls: 1 − 2w > 0 at n = 13 (w < 1/2) — the
        // shortcut is accidentally RIGHT, product rule and sgn(u)
        // agree, both match the float.
        for (cu, cv) in [(1i64, -2i64), (-1, 2), (1, 2), (-1, -2)] {
            let u = K13::ONE.scale(cu);
            let v = K13::ONE.scale(cv);
            let x = Nq::new(u, v, 1);
            assert_eq!(
                u.mul(u).sub(s.mul(v.mul(v))).sign(),
                Ordering::Greater,
                "control u² > s v² ({cu}, {cv})"
            );
            assert_eq!(x.sgn(), x.u.sign(), "boundary control ({cu}, {cv})");
            let fl = x.fl();
            assert_eq!(
                fl > 0.0,
                x.sgn() == Ordering::Greater,
                "control float ({cu}, {cv})"
            );
        }
        // Sweep small elements; the expected sign recomputed from the
        // rule with exact K13 arithmetic; the flipped set is pinned by
        // size and float-agreement of every element.
        let mut flipped = 0usize;
        let mut mixed = 0usize;
        for cu in -8i64..=8 {
            for cv in -8i64..=8 {
                let u = K13::ONE.scale(cu);
                let v = K13::ONE.scale(cv);
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
                let fl = x.fl();
                assert!(
                    (fl > 0.0 && expect == Ordering::Greater)
                        || (fl < 0.0 && expect == Ordering::Less)
                        || fl == 0.0 && expect == Ordering::Equal,
                    "f64 cross-check ({cu}, {cv})"
                );
                if su != Ordering::Equal && sv != Ordering::Equal && su != sv {
                    mixed += 1;
                    if su != expect {
                        flipped += 1;
                    }
                }
            }
        }
        // 2*8*8 = 128 mixed-sign pairs in the sweep; the flips are the
        // |u| < |v| w pairs with w ~ 0.4786 (|u|, |v| <= 8): for
        // |v| = 3..8 the counts are 1, 1, 2, 2, 3, 3 per side —
        // 2 * (1 + 1 + 2 + 2 + 3 + 3) = 24.
        assert_eq!(mixed, 128, "mixed-sign population");
        assert_eq!(flipped, 24, "wrong-shortcut burden");
    }

    #[test]
    fn hull_closes_and_is_normalized() {
        let h = hull();
        assert_eq!(h.len(), 13);
        hull_sanity("n13 hull", &h);
        // The float view is sane (emission path).
        for r in [
            <Monster136 as Medallion>::hull_f64(),
            <Monster135 as Medallion>::hull_f64(),
            <Monster133 as Medallion>::hull_f64(),
            <Monster131 as Medallion>::hull_f64(),
        ] {
            assert_eq!(r.len(), 13);
        }
    }
}
