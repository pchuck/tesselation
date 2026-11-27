//! The nonagon monsters: the catalog's first ODD-n records, on the
//! exact unit-edge regular nonagon hull — `{9/3}` for (9,2) and `{9/1}`
//! for (9,3).
//!
//! Odd n has NO parallel monster record: the parallel displacement
//! census at n = 9 (scratch `monster-n9/census*.txt`) shows no
//! axis-parallel midpoint pin (parity theorem T1), and the off-axis
//! midpoint pins (Δ = i − j ≡ 2, 7 mod 9) carry TWO distinct contact
//! angles (80° and 40°) — no single `{n/d}` figure describes them.
//! The odd-n analog is the ROTATED pair: `B = R·A + P` with `R` the
//! rotation by `pi/n = 20°`. Because `-U₀ = R·U₀` as VERTEX SETS for
//! odd n, the half-turn about the centre midpoint swaps `A` and `B`
//! for every displacement — `κ(X) = c_A + c_B − X` maps `V_k^A ->
//! V_(k+4)^B`, `M_k^A -> M_(k+4)^B` and (bijectively) `V_m^B ->
//! V_(m+5)^A` (asserted exactly below) — so `eBA = {i + 4}` stays
//! automatic and the pinning rule becomes `P = M_i − R·M_j`. The
//! forced figure step shifts with the rotation (`B`-edge directions
//! carry an extra `pi/n`): `d ≡ 2·(i − j) − 1 (mod n)`, canonical
//! `min(d, n − d)` — verified on both certified crossings of both
//! records.
//!
//! **Outline structure** (exact battery `walk_out.txt`, re-certified
//! in `verify_pair9`): the catalog counts hold — the union is ONE
//! closed cycle of `2(n−c−1)+2` nodes, the lens ONE of `2c+4`, with
//! `n − c − 1` exposed vertices per polygon — but the split differs
//! from the parallel records: the rotation shifts the index frames,
//! so the union runs after `X0` along the `B`-arc `V_(eb0+1) ..
//! V_(eb1)` and after `X1` along the `A`-arc `V_(ea1+1) .. V_(ea0)`
//! (the lens takes the complementary arcs). The absorbed runs' END
//! vertices `V_(ea1+1)`, `V_(eb1+1)` are exposed TRANSITION vertices:
//! they walk the union while the inner half of their edge closes the
//! lens beyond a crossing. The hub's [`super::outlines`] rederives
//! the same cycles from the certified exposed/crossing data
//! (cross-checked assert), and every cycle is re-certified piece by
//! piece with exact midpoint half-plane tests.
//!
//! Both records live in `N = K₉(w)`, `w² = 2 − b` (`w = 2 sin 20°`):
//! the rotated frame needs `sin(2π/9) ∉ K₉`, so coordinates ship as
//! [`Nq`] = `(u + v·w)/den` over [`K9`], and the trait's `Field` gains
//! the `K9` variant (additive, zero other consumers). `contact_cos2`
//! returns the rational `1/4` for `{9/3}` and `None` for `{9/1}`
//! (`(2+b)/4` is cubic — the dec10/L10 precedent; the battery pins it
//! exactly). See the [module docs](super) for the catalog, pinning and
//! window certification.

use super::{gmidpoints, star_exits, star_figure, FieldEl, Gp, PairData, WindowProbe};
use crate::fields::K9;
use crate::medallion::{Absorption, Field, Medallion};

// =====================================================================
// N = K₉(w) coordinates with an exact denominator: the shipped `(u +
// v*w) / den` scheme now lives in the generic odd-n engine [`rotodd`]
// over the `K9` core-field implementation (M181 §7 gap 1); this
// module keeps the `K9` alias so every shipped path, constant and
// test reads exactly as before. The Python size battery (scratch
// `monster-n9/size_out.txt`) walked every operation of the verifier
// with this exact integer scheme: worst canonical coefficient 31 bits,
// denominator 27 bits, cross-products 26 bits, convolution
// intermediates 32 bits — far inside the i64 operating range the `K9`
// sign engine documents.
// =====================================================================

use crate::medallion::monster::rotodd::{self, RotSpec};

/// `(u + v*w) / den` over [`K9`] — the shipped nonagon coordinate type
/// (the `K9` instance of the generic [`rotodd::Nq`]).
pub type Nq = rotodd::Nq<K9>;

// =====================================================================
// The exact nonagon hull and the rotation constants.
// =====================================================================

/// `cos 40° = b/2` (exact; `b = 2cos(2pi/9)`).
pub fn c40() -> Nq {
    Nq::new(K9::B, K9::ZERO, 2)
}
/// `sin 40° = (b² + b − 2)/2 · w` (`= a w / 2`, `a = 2cos20°`,
/// `a² = b + 2`).
pub fn s40() -> Nq {
    Nq::new(K9::ZERO, K9([-2, 1, 1]), 2)
}
/// `cos 20° = (b² + b − 2)/2`.
pub fn c20() -> Nq {
    Nq::new(K9([-2, 1, 1]), K9::ZERO, 2)
}
/// `sin 20° = w/2`.
pub fn s20() -> Nq {
    Nq::new(K9::ZERO, K9::ONE, 2)
}

/// Rotation by 40° about the origin (the hull walk step) — the `K9`
/// instance of [`rotodd::rot_step`] (same constants: `c40`/`s40`).
pub use crate::medallion::monster::rotodd::rot_step as rot40;

/// Rotation by `pi/n = 20°` about the origin — the pair rotation `R`,
/// the `K9` instance of [`rotodd::rot_half`] (same constants:
/// `c20`/`s20`).
pub use crate::medallion::monster::rotodd::rot_half as rot20;

/// The record hull: the unit-edge regular nonagon, normalized frame
/// (`V0 = origin`, edge 0 along +x), built by walking unit edge
/// directions `u_(k+1) = rot40(u_k)` — the shared odd-n walk
/// [`rotodd::hull_odd`] at `n = 9`. The closure lemma `rot40^9 = id`
/// and `Sum u_k = 0` and the unit-length lemma `|u_k|² = 1` are
/// asserted exactly (the Python twin is scratch `monster-n9/exact.py`).
pub fn hull() -> Vec<Gp<Nq>> {
    rotodd::hull_odd::<K9>(9, "n9 hull")
}


/// `cos²(3·20°) = 1/4` — the (9,2) crossing and contact angle (60°).
pub(super) fn cos292() -> Nq {
    Nq::new(K9::ONE, K9::ZERO, 4)
}
/// `cos²(1·20°) = (2 + b)/4` — the (9,3) crossing and contact angle
/// (20°, i.e. `pi/n` — the recorded boundary of the M163 EXTENSION
/// guard; the standalone star emission path is unguarded, like the
/// `{12/6}` recorded degeneracy).
pub(super) fn cos293() -> Nq {
    Nq::new(K9([2, 1, 0]), K9::ZERO, 4)
}

/// The (9,2) pin `P = M₂ − R·M₀ = ((4+b)/4, (−6+3b+3b²)/4 · w)`
/// (`|P|² = (13+2b−2b²)/4`).
pub(super) fn pin92() -> Gp<Nq> {
    (
        Nq::new(K9([4, 1, 0]), K9::ZERO, 4),
        Nq::new(K9::ZERO, K9([-6, 3, 3]), 4),
    )
}
/// The (9,3) pin `P = M₁ − R·M₀ = ((6−b²)/4, (−3+b+b²)/4 · w)`
/// (`|P|² = (12−b−3b²)/4`).
pub(super) fn pin93() -> Gp<Nq> {
    (
        Nq::new(K9([6, 0, -1]), K9::ZERO, 4),
        Nq::new(K9::ZERO, K9([-3, 1, 1]), 4),
    )
}

/// The certified (9,2) window `(s_lo, s_hi)` — exact
/// vertex-on-edge-line events (scratch `window_out.txt`): below
/// `s_lo = (26−6b−2b²)/19` the absorbed count jumps to 3
/// (`eAB = [3,4,5]`, `eBA = [0,7,8]`, crossings relabel to
/// `(A2,B1)+(A6,B6)`); above `s_hi = (12+6b+2b²)/19` it drops to 1
/// (crossings `(A2,B8)+(A4,B6)`).
pub(super) fn win92() -> (Nq, Nq) {
    (
        Nq::new(K9([26, -6, -2]), K9::ZERO, 19),
        Nq::new(K9([12, 6, 2]), K9::ZERO, 19),
    )
}

/// The certified (9,3) window `(s_lo, s_hi)`: below
/// `s_lo = (4+2b−2b²)/3` the absorbed count drops to 2 and FOUR extra
/// transversals appear at the contacts (six crossings, scratch
/// `window_out.txt`); above the shared `s_hi = (12+6b+2b²)/19` it
/// drops to 2 with crossings `(A1,B8)+(A4,B5)`. Line-level collinear
/// events exist inside the window (they change no count — the state
/// census at σ = 0.8565/0.90/1.10 is identical to the pin's).
pub(super) fn win93() -> (Nq, Nq) {
    (
        Nq::new(K9([4, 2, -2]), K9::ZERO, 3),
        Nq::new(K9([12, 6, 2]), K9::ZERO, 19),
    )
}

// =====================================================================

use super::OutNode;

/// The `{9/3}` record: two tricigrams on the nonagon, contact angle
/// `3*pi/9 = 60` degrees (`cos^2 = 1/4`).
pub struct Monster93;

impl Medallion for Monster93 {
    const ID: &'static str = "nonagon {9/3} monster star (24c)";
    const FIELD: Field = Field::K9;
    const SYM: usize = 9;
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
    fn contact_cos2() -> Option<(i64, i64, i64, i64)> {
        Some((1, 0, 0, 4)) // cos^2 60 degrees is rational
    }
}

/// The `{9/1}` record: the midpoint nonagon on the nonagon, contact
/// angle `pi/9 = 20` degrees (`cos^2 = (2+b)/4` — cubic, pinned in the
/// battery; the trait tuple cannot encode it, the dec10/L10 precedent).
/// The step `d = 1` figure is a new catalog degeneracy (all landed
/// records have `d >= 2`), recorded like the `{12/6}` note; `theta =
/// pi/n` sits exactly on the M163 EXTENSION guard boundary, like
/// (12,2) at `pi/2` — the standalone star emission path is unguarded.
pub struct Monster91;

impl Medallion for Monster91 {
    const ID: &'static str = "nonagon {9/1} monster star (24c)";
    const FIELD: Field = Field::K9;
    const SYM: usize = 9;
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
    // cos^2 = (2+b)/4 is cubic: None (dec10/L10 precedent), pinned
    // exactly as cos293 in the batteries below.
}

/// The `{9/3}` figure as contact-index pairs.
pub fn figure3() -> Vec<(usize, usize)> {
    star_figure(9, 3)
}

/// The `{9/1}` figure as contact-index pairs.
pub fn figure1() -> Vec<(usize, usize)> {
    star_figure(9, 1)
}

/// The 18 directed exit rays of the `{9/3}` record.
pub fn exit_rays3() -> Vec<(usize, Gp<Nq>)> {
    star_exits(&gmidpoints(&hull()), 3)
}

/// The 18 directed exit rays of the `{9/1}` record.
pub fn exit_rays1() -> Vec<(usize, Gp<Nq>)> {
    star_exits(&gmidpoints(&hull()), 1)
}

/// The certified (9,2) placement: `eAB = [3,4]`, `eBA = [7,8]`,
/// crossings `(A2,B0)` and `(A5,B6)` at the edge midpoints, `P = M2 -
/// R*M0`, window `((26-6b-2b^2)/19, (12+6b+2b^2)/19)` with pin
/// `sigma = 1`. Union 14 nodes, lens 8.
pub fn pair92() -> PairData<Nq> {
    let (lo, hi) = win92();
    rotodd::verify_pair_rot(
        &RotSpec {
            name: "(9,2)",
            n: 9,
            p: pin92(),
            d: 3,
            e_ab: vec![3, 4],
            e_ba: vec![7, 8],
            cross: [(2, 0), (5, 6)],
            cos2: cos292(),
            s_lo: lo,
            s_hi: hi,
            // The certified crossing-parameter line: t = sigma/2 (so
            // the samples 1 ∓ 1/64 give 63/128 and 65/128 — the old
            // `t_increasing` flag is now `sgn k = Greater`), and
            // s(sigma) = a + ((5 − b − b²)/2)·sigma with a = 2cos 20°
            // (scratch `.scratch/rungs-24c-impl/extract_extra.py`).
            t0: Nq::ZERO,
            k: Nq::new(K9::ONE, K9::ZERO, 2),
            s0: Nq::new(K9([-2, 1, 1]), K9::ZERO, 1),
            ks: Nq::new(K9([5, -2, -2]), K9::ZERO, 2),
            union_cycle: vec![
                OutNode::X(0),
                OutNode::B(1),
                OutNode::B(2),
                OutNode::B(3),
                OutNode::B(4),
                OutNode::B(5),
                OutNode::B(6),
                OutNode::X(1),
                OutNode::A(6),
                OutNode::A(7),
                OutNode::A(8),
                OutNode::A(0),
                OutNode::A(1),
                OutNode::A(2),
            ],
            lens_cycle: vec![
                OutNode::X(0),
                OutNode::A(3),
                OutNode::A(4),
                OutNode::A(5),
                OutNode::X(1),
                OutNode::B(7),
                OutNode::B(8),
                OutNode::B(0),
            ],
        },
        &[
            // Above the window: only edge 3 (resp. 7) stays absorbed,
            // crossings relabel to (A2,B8) + (A4,B6).
            WindowProbe {
                rho: hi.add(Nq::ONE.scaled_div(1, 64)),
                ca: 1,
                cb: 1,
                crossings: Some(2),
            },
            // Below the window: the count jumps to 3
            // (eAB = [3,4,5], eBA = [0,7,8]), still two crossings,
            // relabeled (A2,B1) + (A6,B6).
            WindowProbe {
                rho: lo.sub(Nq::ONE.scaled_div(1, 64)),
                ca: 3,
                cb: 3,
                crossings: Some(2),
            },
        ],
    )
}

/// The certified (9,3) placement: `eAB = [2,3,4]`, `eBA = [6,7,8]`,
/// crossings `(A1,B0)` and `(A5,B5)` at the edge midpoints, `P = M1 -
/// R*M0`, window `((4+2b-2b^2)/3, (12+6b+2b^2)/19)` with pin
/// `sigma = 1`. Union 12 nodes, lens 10.
pub fn pair93() -> PairData<Nq> {
    let (lo, hi) = win93();
    rotodd::verify_pair_rot(
        &RotSpec {
            name: "(9,3)",
            n: 9,
            p: pin93(),
            d: 1,
            e_ab: vec![2, 3, 4],
            e_ba: vec![6, 7, 8],
            cross: [(1, 0), (5, 5)],
            cos2: cos293(),
            s_lo: lo,
            s_hi: hi,
            // The certified crossing-parameter line: t = (2 − sigma)/2
            // (samples 65/128 and 63/128 — the old `t_increasing =
            // false` is `sgn k = Less`); the s-line matches (9,2)'s.
            t0: Nq::ONE,
            k: Nq::new(K9([-1, 0, 0]), K9::ZERO, 2),
            s0: Nq::new(K9([-2, 1, 1]), K9::ZERO, 1),
            ks: Nq::new(K9([5, -2, -2]), K9::ZERO, 2),
            union_cycle: vec![
                OutNode::X(0),
                OutNode::B(1),
                OutNode::B(2),
                OutNode::B(3),
                OutNode::B(4),
                OutNode::B(5),
                OutNode::X(1),
                OutNode::A(6),
                OutNode::A(7),
                OutNode::A(8),
                OutNode::A(0),
                OutNode::A(1),
            ],
            lens_cycle: vec![
                OutNode::X(0),
                OutNode::A(2),
                OutNode::A(3),
                OutNode::A(4),
                OutNode::A(5),
                OutNode::X(1),
                OutNode::B(6),
                OutNode::B(7),
                OutNode::B(8),
                OutNode::B(0),
            ],
        },
        &[
            // Above the window: edge 4 (resp. 8) drops out, crossings
            // relabel to (A1,B8) + (A4,B5).
            WindowProbe {
                rho: hi.add(Nq::ONE.scaled_div(1, 64)),
                ca: 2,
                cb: 2,
                crossings: Some(2),
            },
            // Below the window: the absorbed run shifts (eAB = [3,4],
            // eBA = [7,8]) and FOUR extra transversals appear at the
            // contacts (six crossings). Exact battery
            // `winprobe_exact.txt`.
            WindowProbe {
                rho: lo.sub(Nq::ONE.scaled_div(1, 64)),
                ca: 2,
                cb: 2,
                crossings: Some(6),
            },
            // A rational probe deep below the window (sigma = 7/10)
            // shows the same state.
            WindowProbe {
                rho: Nq::ONE.scaled_div(7, 10),
                ca: 2,
                cb: 2,
                crossings: Some(6),
            },
        ],
    )
}

// =====================================================================
// Tests.
// =====================================================================

#[cfg(test)]
mod tests {
    use super::super::{gadd, gcentre, gdot, gmul_i, gsub, hull_sanity};
    use super::super::battery_record;
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn nonagon_hull_closes_and_is_normalized() {
        let h = hull();
        assert_eq!(h.len(), 9);
        hull_sanity("n9 hull", &h);
    }

    #[test]
    fn nq_field_axioms_on_certified_elements() {
        // Associativity/distributivity/commutativity spot checks via
        // exact equality, plus canonical form and sign consistency.
        let xs = [
            Nq::ONE,
            pin92().0,
            pin92().1,
            pin93().0,
            pin93().1,
            cos292(),
            cos293(),
            win92().0,
            win92().1,
            win93().0,
            c20().add(s20()),
            c40().mul(s40()),
        ];
        for &x in &xs {
            assert!(x.is_canonical());
            assert!(x.add(Nq::ZERO).eq_x(x));
            assert!(x.mul(Nq::ONE).eq_x(x));
            assert!(x.sub(x).sgn() == Ordering::Equal);
            assert!(x.mul(Nq::NEG_ONE).neg().eq_x(x));
            assert_eq!(x.sgn(), x.neg().sgn().reverse());
        }
        for &x in &xs {
            for &y in &xs {
                assert!(x.add(y).eq_x(y.add(x)));
                assert!(x.mul(y).eq_x(y.mul(x)));
                assert!(x.mul(y).is_canonical());
                // (x + y)^2 = x^2 + 2xy + y^2
                let l = x.add(y).mul(x.add(y));
                let r = x.mul(x).add(gmul_i(x, 2).mul(y)).add(y.mul(y));
                assert!(l.eq_x(r));
                // sign of a product
                let s = x.mul(y).sgn();
                let expect = match (x.sgn(), y.sgn()) {
                    (Ordering::Equal, _) | (_, Ordering::Equal) => Ordering::Equal,
                    (a, b) if a == b => Ordering::Greater,
                    _ => Ordering::Less,
                };
                assert_eq!(s, expect);
            }
        }
    }

    #[test]
    fn both_figures_pass_the_record_battery() {
        // The {9/3} record of (9,2) and the {9/1} record of (9,3).
        battery_record("<{9/3} record>", &hull(), 3, cos292());
        battery_record("<{9/1} record>", &hull(), 1, cos293());
        assert_eq!(<Monster93 as Medallion>::SYM, 9);
        assert_eq!(<Monster93 as Medallion>::FIELD, Field::K9);
        assert_eq!(<Monster93 as Medallion>::contact_cos2(), Some((1, 0, 0, 4)));
        assert_eq!(<Monster91 as Medallion>::SYM, 9);
        assert_eq!(<Monster91 as Medallion>::FIELD, Field::K9);
        assert_eq!(<Monster91 as Medallion>::contact_cos2(), None);
        assert_eq!(figure3().len(), 9);
        assert_eq!(figure1().len(), 9);
        assert_eq!(exit_rays3().len(), 18);
        assert_eq!(exit_rays1().len(), 18);
    }

    #[test]
    fn placement92_is_the_certified_pin() {
        let d = pair92();
        assert!(gp_eq(&d.p, &pin92()));
        assert_eq!(d.e_ab, vec![3, 4]);
        assert_eq!(d.e_ba, vec![7, 8]);
        // Strictly-outside vertices: `n - c - 1 = 6` per side; the
        // run-end transition vertices V2/V6 (A) and B0/B6 (B) are
        // exposed even though their edges' inner halves close the
        // lens (module docs, outline structure).
        assert_eq!(d.exposed_a, vec![0, 1, 2, 6, 7, 8]);
        assert_eq!(d.exposed_b, vec![1, 2, 3, 4, 5, 6]);
        let h = hull();
        let m = gmidpoints(&h);
        assert!(gp_eq(&d.cross_pts[0], &m[2]));
        assert!(gp_eq(&d.cross_pts[1], &m[5]));
        let p_of = |i: usize, j: usize| gsub(m[i], rot20(m[j]));
        assert!(gp_eq(&p_of(2, 0), &d.p));
        assert!(gp_eq(&p_of(5, 6), &d.p));
        // Union 2(9-2-1)+2 = 14 nodes; lens 2*2+4 = 8.
        assert_eq!(d.union_cycle.len(), 14);
        assert_eq!(d.lens_cycle.len(), 8);
    }

    #[test]
    fn placement93_is_the_certified_pin() {
        let d = pair93();
        assert!(gp_eq(&d.p, &pin93()));
        assert_eq!(d.e_ab, vec![2, 3, 4]);
        assert_eq!(d.e_ba, vec![6, 7, 8]);
        assert_eq!(d.exposed_a, vec![0, 1, 6, 7, 8]);
        assert_eq!(d.exposed_b, vec![1, 2, 3, 4, 5]);
        let h = hull();
        let m = gmidpoints(&h);
        assert!(gp_eq(&d.cross_pts[0], &m[1]));
        assert!(gp_eq(&d.cross_pts[1], &m[5]));
        let p_of = |i: usize, j: usize| gsub(m[i], rot20(m[j]));
        assert!(gp_eq(&p_of(1, 0), &d.p));
        assert!(gp_eq(&p_of(5, 5), &d.p));
        // Union 2(9-3-1)+2 = 12 nodes; lens 2*3+4 = 10.
        assert_eq!(d.union_cycle.len(), 12);
        assert_eq!(d.lens_cycle.len(), 10);
    }

    #[test]
    fn outline_cycles_close_in_float() {
        // The float reading of the exact cycles: consecutive nodes
        // distinct, nonzero enclosed area (the cycles themselves are
        // certified exactly in verify_pair9).
        for (d, xs) in [(pair92(), [2usize, 5]), (pair93(), [1, 5])] {
            let h = hull();
            let m = gmidpoints(&h);
            let pos = |t: &OutNode| -> (f64, f64) {
                match *t {
                    OutNode::A(k) => (h[k].0.fl(), h[k].1.fl()),
                    OutNode::B(k) => {
                        let q = gadd(rot20(h[k]), d.p);
                        (q.0.fl(), q.1.fl())
                    }
                    OutNode::X(k) => (m[xs[k]].0.fl(), m[xs[k]].1.fl()),
                }
            };
            for cyc in [&d.union_cycle, &d.lens_cycle] {
                let pts: Vec<(f64, f64)> = cyc.iter().map(pos).collect();
                for w in pts.windows(2) {
                    let dx = w[1].0 - w[0].0;
                    let dy = w[1].1 - w[0].1;
                    assert!(dx * dx + dy * dy > 1e-9, "outline has a degenerate edge");
                }
                let dd = pts.len();
                let mut area: f64 = 0.0;
                for i in 0..dd {
                    let (p1, p2) = (pts[i], pts[(i + 1) % dd]);
                    area += p1.0 * p2.1 - p1.1 * p2.0;
                }
                assert!(area.abs() > 1.0, "outline area degenerates");
            }
        }
    }

    #[test]
    fn windows_and_metric_identities() {
        let (lo2, hi) = win92();
        let (lo3, hi3) = win93();
        assert!(hi.eq_x(hi3)); // the shared upper event
        assert_eq!(lo2.sub(Nq::ONE).sgn(), Ordering::Less);
        assert_eq!(lo3.sub(Nq::ONE).sgn(), Ordering::Less);
        assert_eq!(hi.sub(Nq::ONE).sgn(), Ordering::Greater);
        // |P|^2 exact forms.
        assert!(gdot(pin92(), pin92()).eq_x(Nq::new(K9([13, 2, -2]), K9::ZERO, 4)));
        assert!(gdot(pin93(), pin93()).eq_x(Nq::new(K9([12, -1, -3]), K9::ZERO, 4)));
        // Centre-centre distances: |c_B - c|^2 = inradius^2 for (9,2)
        // (= (1+8b+4b^2)/12) and (2 r sin 10 deg)^2 for (9,3)
        // (= (16 - b - 5b^2)/12).
        let c = gcentre(&hull());
        for (p, want) in [
            (pin92(), Nq::new(K9([1, 8, 4]), K9::ZERO, 12)),
            (pin93(), Nq::new(K9([16, -1, -5]), K9::ZERO, 12)),
        ] {
            let dd = gsub(gadd(rot20(c), p), c);
            assert!(gdot(dd, dd).eq_x(want));
        }
        // cos^2 ordering sanity: (2+b)/4 > 1/4.
        assert_eq!(cos293().sub(cos292()).sgn(), Ordering::Greater);
    }

    /// Regression battery for the `sgn` mixed-sign arm (docs §4.3, §6.3):
    /// the shipped arm returned `sgn(u)` unconditionally (the
    /// `Ordering::then_with` returns `self` unless Equal), which
    /// disagrees with the product rule `sgn(u) · sgn(u² − s·v²)`
    /// exactly on mixed-sign elements with u² < s·v². The elements
    /// below pin both sides of the u² vs s·v² split, plus same-sign
    /// and zero controls. Expected signs are derived from the product
    /// rule (exact K₉ arithmetic) and cross-checked against the f64
    /// float view (`Nq::fl`, w₉ = 2 sin 20° from the shipped K9 float
    /// view).
    #[test]
    fn sgn_mixed_sign_product_rule_regression() {
        // Mandated element (docs §4.3, §6.3): 1 − 2·w₉ ≈ −0.368 < 0.
        // u = 1, v = −2: u² = 1 < s·4 (s = 2−b ≈ 0.468) → product
        // rule: Greater · Less = Less.
        let x = Nq::new(K9::ONE, K9([-2, 0, 0]), 1);
        assert_eq!(x.sgn(), Ordering::Less);
        assert!(x.fl() < 0.0, "1 − 2w₉ ≈ {} < 0", x.fl());

        // Mixed signs, both sides of the u² vs s·v² split:
        //   u = 1, v = −1:  u² = 1 > s·1 ≈ 0.468 → Greater · Greater = Greater
        //   u = −1, v = 1:  u² = 1 > s·1 ≈ 0.468 → Less · Greater = Less
        //   u = −1, v = 2:  u² = 1 < s·4 ≈ 1.87 → Less · Less = Greater
        let cases: [(K9, K9, Ordering); 3] = [
            (K9::ONE, K9([-1, 0, 0]), Ordering::Greater),
            (K9([-1, 0, 0]), K9::ONE, Ordering::Less),
            (K9([-1, 0, 0]), K9([2, 0, 0]), Ordering::Greater),
        ];
        for (u, v, expect) in cases {
            let x = Nq::new(u, v, 1);
            assert_eq!(x.sgn(), expect);
            // cross-check against the f64 view (w₉ from the shipped float path)
            let fl = x.fl();
            assert!(
                (fl > 0.0 && expect == Ordering::Greater) || (fl < 0.0 && expect == Ordering::Less),
                "f64 cross-check: {:?} fl = {} expect {:?}", x, fl, expect
            );
            // fl() must be consistent with the product rule's signs
            let su = u.sign();
            let term = u
                .mul(u)
                .sub(rotodd::core_s::<K9>().mul(v.mul(v)))
                .sign();
            let product_rule = if su == term { Ordering::Greater } else { Ordering::Less };
            assert_eq!(product_rule, expect, "product rule for {:?}", x);
        }
        // the u = −1, v = 2 element is the mirror image of 1 − 2w₉
        // (u = 1, v = −2); note sgn(−x) = sgn(x).reverse()
        let mirror = Nq::new(K9([-1, 0, 0]), K9([2, 0, 0]), 1);
        let original = Nq::new(K9::ONE, K9([-2, 0, 0]), 1);
        assert_eq!(mirror.sgn(), original.sgn().reverse());

        // Same-sign controls (the arm that was already correct):
        //   u = 1, v = 2:  sgn(u) = Greater; same-sign ⇒ Greater
        //   u = −1, v = −1: sgn(u) = Less; same-sign ⇒ Less
        let same_sign: [(K9, K9, Ordering); 2] = [
            (K9::ONE, K9([2, 0, 0]), Ordering::Greater),
            (K9([-1, 0, 0]), K9([-1, 0, 0]), Ordering::Less),
        ];
        for (u, v, expect) in same_sign {
            let x = Nq::new(u, v, 1);
            assert_eq!(x.sgn(), expect);
            let fl = x.fl();
            assert!(
                (fl > 0.0 && expect == Ordering::Greater) || (fl < 0.0 && expect == Ordering::Less),
                "f64 cross-check: {:?} fl = {} expect {:?}", x, fl, expect
            );
        }

        // Zero controls: u = 0 / v = 0 (zero factors ⇒ the `Equal` arms)
        let zero_v = Nq::new(K9::ONE, K9::ZERO, 1);
        let zero_u = Nq::new(K9::ZERO, K9::ONE, 1);
        assert_eq!(zero_v.sgn(), Ordering::Greater);
        assert_eq!(zero_u.sgn(), Ordering::Greater);
        let zero = Nq::ZERO;
        assert_eq!(zero.sgn(), Ordering::Equal);
    }

    fn gp_eq(p: &Gp<Nq>, q: &Gp<Nq>) -> bool {
        p.0.eq_x(q.0) && p.1.eq_x(q.1)
    }
}
