//! The hexadecagon monster: the **(16,3)** catalog rung on the regular
//! unit-edged 16-gon in K₁₆ = Q(b), b = 2cos(π/8) ([`K16`]), plus the
//! exact AB-substrate battery that grounds the rung.
//!
//! See the [module docs](super) for the catalog, the pinning rule and
//! the window certification. The record figure is `{16/8}`: eight
//! diameters joining opposite edge midpoints, contact angle `90°`
//! (`cos² = 0`) — the `{12/6}` precedent, so the two exit rays at each
//! contact **coincide** (`i + 8 ≡ i − 8`) and the battery records this
//! explicitly.
//!
//! # The pin and its window (exact, scratch `pin16.py`)
//!
//! The displacement pins midpoints `P = M₆ − M₂` along the mid-edge
//! axis `D = M₈ − M₀`, i.e. `ρ = √2/2 = (b² − 2)/2`. The exact
//! vertex-on-edge-line event census gives the certified window
//! `(b² − b − 1, b − 1)` — and the pin is its exact midpoint:
//! `2ρ = (b² − b − 1) + (b − 1)` identically (`√2 − b + b − 1 + 1 =
//! √2`). Inside: `eAB = [7, 8, 9]`, `eBA = {i + 8}` = `[0, 1, 15]`, two
//! transversal crossings `(A₆, B₂)`, `(A₁₀, B₁₄)` at `t = 1/2` with
//! the pin identity `P = M₆ − M₂`. `d = 2 (6 − 2) ≡ 8 (mod 16)` is the
//! forced figure step (perpendicular crossings, `cos² = 0` exactly).
//!
//! # The AB substrate (monster 000 of the §22e Stage-1 recon)
//!
//! The tiling pair behind the rung — two apothem-½ 16-gons with
//! absolute normals at `22.5k°`, centres `A = (−2 − 2√2, −1 − √2)` and
//! `B = (−3 − 3√2/2, −1 − √2/2)` — overlaps with `c = 3` **both ways**
//! (strict = closed: no tangency), a 10-vertex lens (`2c + 4`), exact
//! overlap area `(9 − 16b − 5b² + 6b³)/2 = 0.108325753…` and union
//! `(7 − 16b − 11b² + 10b³)/2 = 1.482973186…` — the banked recon
//! floats, re-derived exactly here. Its two crossings sit at
//! parameters `√2/2` and `1 − √2/2`, **not** at midpoints: the AB pair
//! itself is not a `verify_pair` pin, so the catalog record is the
//! midpoint-pinned `(16, 3)` placement above (same `c = 3` lens
//! combinatorics) and the AB pair is certified by its own exact
//! battery below.
//!
//! # Why a denominator wrapper
//!
//! `N(b) = 2` makes `b⁻¹ = (4b − b³)/2 ∉ Z[b]`, and the 16-gon frame
//! trig is half-integral (cos 22.5° = b/2, sin 22.5° = d/2, midpoints
//! /4, the exact clip divides). [`Kq16`] is `Z[b]` over an integer
//! denominator, gcd-reduced — the [`super::super::girih_m161::Kq`]
//! pattern one degree up.
//!
//! # The octagon variant — documented NON-record
//!
//! An octagon pair at the same lens opening closes the `(8, 2)` band
//! only through a **vertex tangency**: `c_strict = 0`, `c_closed = 2`.
//! That is outside the catalog's strict-interior definition of `c` and
//! degenerate for the crossing model, so it is recorded here as a
//! comment only, never as a catalog record.

use super::{
    gmidpoints, star_exits, star_figure, verify_pair, Axis, FieldEl, PairData, PairSpec,
    WindowProbe, Gp,
};
// Consumed only by the batteries (the AB certificate and the schema run).
#[cfg(test)]
use super::{assert_general_position, battery_record, gedges_inside, gcrossings};
use crate::fields::{kmul, K16, RED16};
use crate::medallion::{Absorption, Field, Medallion};
use std::cmp::Ordering;

// =====================================================================
// Kq16 — Z[b] over an integer denominator (gcd-reduced, den > 0)
// =====================================================================

/// An exact K₁₆ element `n / den` with `n ∈ Z[b]`, `den > 0`, always
/// gcd-reduced (a canonical form: equality is tuple equality after
/// reduction, cross-checked by [`Kq16::eq_x`]).
#[derive(Clone, Copy, Debug)]
pub struct Kq16 {
    /// The numerator in the power basis of b.
    pub n: K16,
    /// The positive integer denominator.
    pub den: i64,
}

/// gcd over the five integers `(c0..c3, den)` (den > 0 input, the
/// result divides every entry).
fn gcd5(vals: [i64; 5]) -> i64 {
    fn g(a: i64, b: i64) -> i64 {
        let (mut a, mut b) = (a.abs(), b.abs());
        while b != 0 {
            let t = a % b;
            (a, b) = (b, t);
        }
        a
    }
    let mut d = g(vals[0], vals[1]);
    for v in &vals[1..4] {
        d = g(d, *v);
    }
    g(d, vals[4]).max(1)
}

impl Kq16 {
    /// The zero element.
    pub const ZERO: Kq16 = Kq16 {
        n: K16::ZERO,
        den: 1,
    };

    /// Construct from basis coefficients and a positive denominator,
    /// gcd-reducing (the canonical form). `den != 0` is required.
    pub fn new(c: [i64; 4], den: i64) -> Kq16 {
        assert!(den != 0, "Kq16: zero denominator");
        let (s, d) = if den < 0 { (-1, -den) } else { (1, den) };
        let r = gcd5([c[0] * s, c[1] * s, c[2] * s, c[3] * s, d]);
        Kq16 {
            n: K16([c[0] * s / r, c[1] * s / r, c[2] * s / r, c[3] * s / r]),
            den: d / r,
        }
    }

    /// The integer `v`.
    #[inline]
    pub fn int(v: i64) -> Kq16 {
        Kq16 {
            n: K16([v, 0, 0, 0]),
            den: 1,
        }
    }

    /// Lift a basis element over 1 (already canonical).
    #[inline]
    pub fn lift(n: K16) -> Kq16 {
        Kq16 { n, den: 1 }
    }

    #[inline]
    pub fn add(self, o: Kq16) -> Kq16 {
        Kq16::new(
            self.n.scale(o.den).add(o.n.scale(self.den)).0,
            self.den * o.den,
        )
    }

    #[inline]
    pub fn sub(self, o: Kq16) -> Kq16 {
        Kq16::new(
            self.n.scale(o.den).sub(o.n.scale(self.den)).0,
            self.den * o.den,
        )
    }

    #[inline]
    pub fn neg(self) -> Kq16 {
        Kq16 {
            n: self.n.neg(),
            den: self.den,
        }
    }

    #[inline]
    pub fn mul(self, o: Kq16) -> Kq16 {
        Kq16::new(self.n.mul(o.n).0, self.den * o.den)
    }

    /// Scaling by an integer (exact, kept unreduced-denominator then
    /// canonicalized).
    #[inline]
    pub fn scale(self, c: i64) -> Kq16 {
        Kq16::new(self.n.scale(c).0, self.den)
    }

    /// Exact sign: `den > 0`, so the sign is the numerator's
    /// ([`K16::sign`] — the isolating-interval engine).
    #[inline]
    pub fn sgn(self) -> Ordering {
        self.n.sign()
    }

    /// Cross-multiplied equality (agrees with canonical-form tuple
    /// equality; the form the generic batteries use).
    #[inline]
    pub fn eq_x(self, o: Kq16) -> bool {
        self.n.scale(o.den) == o.n.scale(self.den)
    }

    /// Float view — cross-check/emission use only.
    #[inline]
    pub fn fl(self) -> f64 {
        self.n.float() / self.den as f64
    }

    /// Field inverse: `1 / (n / d) = d·adj_x / N(x)` where the
    /// adjugate-column `adj_x · e₀` is the row-0 cofactor vector of
    /// the multiplication matrix `M_x` and `N(x) = det M_x` (expanded
    /// along row 0 — the 3×3 minors below; `knorm` is cross-checked
    /// against this det in the tests). Requires `self != 0`.
    pub fn inv(self) -> Kq16 {
        assert!(!self.n.is_zero(), "Kq16: inverse of zero");
        // M_x: column j is x·b^j (i64 heights stay tiny; products run
        // in i128).
        let m = mul_matrix(self.n);
        // Cofactors C_{0i} of row 0 (minors: rows 1..3, drop col i).
        let mut cof = [0i128; 4];
        for i in 0..4 {
            cof[i] = minor3(&m, i);
            if i & 1 == 1 {
                cof[i] = -cof[i];
            }
        }
        let det: i128 = (0..4).map(|i| m[0][i] as i128 * cof[i]).sum();
        assert!(det != 0, "Kq16: nonzero element with zero norm");
        // M⁻¹ e₀ = adj(M) e₀ / det = (C00..C03)/det; multiply by the
        // denominator d.
        Kq16::new(
            [
                (cof[0] * self.den as i128) as i64,
                (cof[1] * self.den as i128) as i64,
                (cof[2] * self.den as i128) as i64,
                (cof[3] * self.den as i128) as i64,
            ],
            // det is a norm of a small-height element here.
            det as i64,
        )
    }
}

/// The multiplication matrix of `x` (ROW-major): entry `[r][c]` is the
/// basis coefficient `r` of `x·b^c` (via the certified [`kmul`] fold).
fn mul_matrix(x: K16) -> [[i64; 4]; 4] {
    let cols: [[i64; 4]; 4] = std::array::from_fn(|j| {
        let mut e = [0i64; 4];
        e[j] = 1;
        kmul(x.0, e, &RED16)
    });
    std::array::from_fn(|r| std::array::from_fn(|c| cols[c][r]))
}

/// det of `M` with row 0 and column `skip` dropped (rows 1..3), by the
/// cyclic (Sarrus) expansion along the minor's first row:
/// `Σ_p a_{0p}(a_{1,q}a_{2,r} − a_{1,r}a_{2,q})`, `(p, q, r)` cyclic —
/// the signs are already carried by the cycle, so no extra `(−1)^p`.
fn minor3(m: &[[i64; 4]; 4], skip: usize) -> i128 {
    let e = |r: usize, c: usize| m[r][if c < skip { c } else { c + 1 }] as i128;
    let mut acc = 0i128;
    for p in 0..3 {
        let q = (p + 1) % 3;
        let r = (p + 2) % 3;
        acc += e(1, p) * (e(2, q) * e(3, r) - e(2, r) * e(3, q));
    }
    acc
}

impl FieldEl for Kq16 {
    fn add(self, o: Self) -> Self {
        Kq16::add(self, o)
    }
    fn sub(self, o: Self) -> Self {
        Kq16::sub(self, o)
    }
    fn neg(self) -> Self {
        Kq16::neg(self)
    }
    fn mul(self, o: Self) -> Self {
        Kq16::mul(self, o)
    }
    fn sgn(self) -> Ordering {
        Kq16::sgn(self)
    }
    fn eq_x(self, o: Self) -> bool {
        Kq16::eq_x(self, o)
    }
    fn fl(self) -> f64 {
        Kq16::fl(self)
    }
    fn zero_like(&self) -> Self {
        Kq16::ZERO
    }
    fn int_like(&self, v: i64) -> Self {
        Kq16::int(v)
    }
    fn scaled_div(self, num: i64, den: i64) -> Self {
        Kq16::new(self.n.scale(num).0, self.den * den)
    }
}

// =====================================================================
// The record hull: unit-edged regular 16-gon, normalized frame
// =====================================================================

/// cos 22.5° = b/2.
fn c22() -> Kq16 {
    Kq16::new([0, 1, 0, 0], 2)
}

/// sin 22.5° = d/2 with d = √(2−√2) = b³ − 3b (pinned exactly by the
/// K16 battery).
fn s22() -> Kq16 {
    Kq16::new([0, -3, 0, 1], 2)
}

/// The 16 unit edge vectors `E_k = (cos 22.5k°, sin 22.5k°)`, built by
/// exact 22.5° rotation from `(1, 0)`.
fn edges16() -> Vec<Gp<Kq16>> {
    let (c, s) = (c22(), s22());
    let mut e = vec![(Kq16::int(1), Kq16::ZERO)];
    for _ in 1..16 {
        let (x, y) = *e.last().unwrap();
        e.push((x.mul(c).sub(y.mul(s)), x.mul(s).add(y.mul(c))));
    }
    e
}

/// The record hull: V₀ = origin, edge 0 along +x, unit edges
/// (the normalized frame of doc §1.2).
pub fn hull() -> Vec<Gp<Kq16>> {
    let e = edges16();
    let mut v = vec![(Kq16::ZERO, Kq16::ZERO)];
    for k in 0..15 {
        let (px, py) = v[k];
        v.push((px.add(e[k].0), py.add(e[k].1)));
    }
    v
}

/// `ρ = √2/2 = (b² − 2)/2` for (16,3) (mid-edge axis, window
/// `(b² − b − 1, b − 1)`, pin = window midpoint exactly).
pub(super) fn rho163() -> Kq16 {
    Kq16::new([-2, 0, 1, 0], 2)
}

/// `cos²(8π/16) = 0` — the (16,3) crossing/contact angle (diameters,
/// the `{12/6}` pattern).
pub(super) fn cos2163() -> Kq16 {
    Kq16::ZERO
}

/// Window lower end `b² − b − 1 = √2 − b` (exact event value from the
/// axis census).
// (Battery-only: the AB certificate and the window ends are consumed
// by the tests below, so the shipped build carries no dead code.)
#[cfg(test)]
fn win_lo() -> Kq16 {
    Kq16::lift(K16([-1, -1, 1, 0]))
}

/// Window upper end `b − 1` (exact event value from the axis census).
#[cfg(test)]
fn win_hi() -> Kq16 {
    Kq16::lift(K16([-1, 1, 0, 0]))
}

/// The `{16/8}` record: eight diameters (opposite edge-midpoint
/// pairs), contact angle `90°` (`cos² = 0`); the two exit rays at each
/// contact coincide.
pub struct Monster168;

impl Medallion for Monster168 {
    const ID: &'static str = "hexadecagon {16/8} monster diameters (24c)";
    const FIELD: Field = Field::K16;
    const SYM: usize = 16;
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
        Some((0, 0, 2, 1)) // cos² 90°
    }
}

/// The `{16/8}` figure as contact-index pairs (eight diameters, each
/// chord listed from both ends).
pub fn figure() -> Vec<(usize, usize)> {
    star_figure(16, 8)
}

/// The 32 directed exit-ray rows of the `{16/8}` record — 16 distinct
/// lines because both rows per contact coincide (`i + 8 ≡ i − 8`, the
/// `{12/6}` degeneracy).
pub fn exit_rays() -> Vec<(usize, Gp<Kq16>)> {
    star_exits(&gmidpoints(&hull()), 8)
}

/// The certified (16,3) placement: `P = (√2/2)·D` on the mid-edge
/// axis, `eAB = [7, 8, 9]`, `eBA = [0, 1, 15]`, crossings `(A₆, B₂)`
/// and `(A₁₀, B₁₄)`.
pub fn pair163() -> PairData<Kq16> {
    verify_pair(
        &PairSpec {
            name: "(16,3)",
            hull_a: hull(),
            rho: rho163(),
            axis: Axis::MidEdgeAxis,
            d: 8,
            e_ab: vec![7, 8, 9],
            cross: vec![(6, 2), (10, 14)],
            cos2: cos2163(),
        },
        &[
            // Deep inside the window's lower neighbour regime (below
            // the first event at b² − 2b… the exact event ladder of
            // the axis census): c = 5 counts still carry 2 crossings.
            WindowProbe {
                rho: Kq16::new([-33, -32, 32, 0], 32),
                ca: 5,
                cb: 5,
                crossings: Some(2),
            },
            // Just above the window end b − 1: the count drops to
            // (1, 1), crossings remain.
            WindowProbe {
                rho: Kq16::new([-31, 32, 0, 0], 32),
                ca: 1,
                cb: 1,
                crossings: Some(2),
            },
            // Near-zero displacement (the (5,5) regime): the polygons
            // nearly coincide and the boundaries no longer cross
            // transversally.
            WindowProbe {
                rho: Kq16::new([1, 0, 0, 0], 100),
                ca: 5,
                cb: 5,
                crossings: Some(0),
            },
            // Beyond the disjointness event ρ = 1: nothing inside, no
            // crossings.
            WindowProbe {
                rho: Kq16::new([37, 0, 0, 0], 32),
                ca: 0,
                cb: 0,
                crossings: Some(0),
            },
        ],
    )
}

// =====================================================================
// The AB substrate pair (apothem ½, monster 000 of the §22e recon)
// =====================================================================

/// `c₁₆ = 1/(2+b) = (2b² − b³)/2` — the vertex offset factor of the
/// absolute-normal polygon formula `V_k = C + c₁₆·(n_k + n_{k+1})`
/// (`(2+b)·(2b² − b³) = 4b² − b⁴ = 2` exactly).
#[cfg(test)]
fn c16() -> Kq16 {
    Kq16::new([0, 0, 2, -1], 2)
}

/// The apothem-½ 16-gon about centre `C` with absolute normals
/// (unit vectors) at `22.5k°` — the recon's `q2_ab_p1` construction.
#[cfg(test)]
fn polygon_abs(c: Gp<Kq16>) -> Vec<Gp<Kq16>> {
    let e = edges16();
    (0..16)
        .map(|k| {
            let (nx, ny) = e[k];
            let (mx, my) = e[(k + 1) % 16];
            (
                c.0.add(c16().mul(nx.add(mx))),
                c.1.add(c16().mul(ny.add(my))),
            )
        })
        .collect()
}

/// Shoelace area (exact, CCW-positive).
#[cfg(test)]
fn shoelace(poly: &[Gp<Kq16>]) -> Kq16 {
    let n = poly.len();
    let mut acc = Kq16::ZERO;
    for i in 0..n {
        let (p, q) = (poly[i], poly[(i + 1) % n]);
        acc = acc.add(p.0.mul(q.1).sub(p.1.mul(q.0)));
    }
    acc.scaled_div(1, 2)
}

/// Exact Sutherland–Hodgman clip of a convex polygon by a convex
/// polygon (half-plane tests by exact sign; edge intersections via the
/// [`Kq16`] field inverse — the `pin16.py` clip ported verbatim).
#[cfg(test)]
fn clip_exact(subject: &[Gp<Kq16>], clf: &[Gp<Kq16>]) -> Vec<Gp<Kq16>> {
    let mut out = subject.to_vec();
    for j in 0..clf.len() {
        let (a, b) = (clf[j], clf[(j + 1) % clf.len()]);
        let e = (b.0.sub(a.0), b.1.sub(a.1));
        let inp = std::mem::take(&mut out);
        if inp.is_empty() {
            break;
        }
        let m = inp.len();
        for i in 0..m {
            let (p, q) = (inp[i], inp[(i + 1) % m]);
            let sp = e.0.mul(p.1.sub(a.1)).sub(e.1.mul(p.0.sub(a.0))).sgn();
            let sq = e.0.mul(q.1.sub(a.1)).sub(e.1.mul(q.0.sub(a.0))).sgn();
            let inter = || {
                let u = (q.0.sub(p.0), q.1.sub(p.1));
                let den = u.0.mul(e.1).sub(u.1.mul(e.0));
                let num = a.0.sub(p.0).mul(e.1).sub(a.1.sub(p.1).mul(e.0));
                let t = num.mul(den.inv());
                (p.0.add(u.0.mul(t)), p.1.add(u.1.mul(t)))
            };
            if sq != Ordering::Less {
                if sp == Ordering::Less {
                    out.push(inter());
                }
                out.push(q);
            } else if sp != Ordering::Less {
                out.push(inter());
            }
        }
        // Exact consecutive dedup (no numerics).
        let mut ded: Vec<Gp<Kq16>> = Vec::with_capacity(out.len());
        for p in out.drain(..) {
            if ded
                .last()
                .is_none_or(|q| !(p.0.eq_x(q.0) && p.1.eq_x(q.1)))
            {
                ded.push(p);
            }
        }
        if ded.len() > 1 && ded[0].0.eq_x(ded.last().unwrap().0) && ded[0].1.eq_x(ded.last().unwrap().1) {
            ded.pop();
        }
        out = ded;
    }
    out
}

/// Centre A of monster 000: `(−2 − 2√2, −1 − √2)`.
#[cfg(test)]
fn centre_a() -> Gp<Kq16> {
    let sq2 = Kq16::lift(K16::SQRT2);
    (Kq16::int(-2).sub(sq2.scale(2)), Kq16::int(-1).sub(sq2))
}

/// Centre B of monster 000: `(−3 − 3√2/2, −1 − √2/2)`.
#[cfg(test)]
fn centre_b() -> Gp<Kq16> {
    let sq2 = Kq16::lift(K16::SQRT2);
    (
        Kq16::int(-3).sub(sq2.mul(Kq16::new([3, 0, 0, 0], 2))),
        Kq16::int(-1).sub(sq2.scaled_div(1, 2)),
    )
}

/// `t₁₆ = tan(π/16) = (2 − b)/d` — the raw/normalized edge scale
/// (raw edge length `2·½·tan(π/16)`).
#[cfg(test)]
fn t16() -> Kq16 {
    Kq16::int(2)
        .sub(Kq16::lift(K16::B))
        .mul(Kq16::lift(K16::DD).inv())
}

/// The full AB-substrate certificate (module docs): centres, `c = 3`
/// both ways with strict = closed, general position, the two exact
/// crossings at `t = √2/2` and `1 − √2/2`, the 10-vertex lens with the
/// banked exact overlap/union areas, the unit-edge rescaling, and the
/// displacement invariants (`|B − A|² = 2 − √2`, unit-edge displacement
/// of length `2 + b` along the `112.5°` direction).
#[cfg(test)]
fn ab_substrate_battery() {
    let (ca, cb) = (centre_a(), centre_b());
    // Centre floats (cross-check only): A ≈ (−4.828427125, −2.414213562),
    // B ≈ (−5.121320344, −1.707106781).
    assert!((ca.0.fl() - (-4.82842712474619)).abs() < 1e-9);
    assert!((ca.1.fl() - (-2.414213562373095)).abs() < 1e-9);
    assert!((cb.0.fl() - (-5.121320343559643)).abs() < 1e-9);
    assert!((cb.1.fl() - (-1.7071067811865477)).abs() < 1e-9);
    // c16 identity: (2+b)·c16 = 1 exactly.
    let two_plus_b = Kq16::int(2).add(Kq16::lift(K16::B));
    assert!(two_plus_b.mul(c16()).eq_x(Kq16::int(1)), "c16 inverse");

    let va = polygon_abs(ca);
    let vb = polygon_abs(cb);
    // Definition counts: strict [3,4,5] / [11,12,13], strict = closed
    // (no tangency), half-turn pairing eBA = eAB + 8.
    let eab = gedges_inside(&va, &vb);
    let eba = gedges_inside(&vb, &va);
    assert_eq!(eab, vec![3, 4, 5], "AB eAB");
    assert_eq!(eba, vec![11, 12, 13], "AB eBA");
    // The raw scale is tangency-free: interior-edge counting agrees
    // with the edge counting because no vertex lies on an edge LINE at
    // all — general position covers both degeneracy kinds:
    assert_general_position(&va, &vb, "AB(16,3)");

    // Crossings: exactly two, on (A₂, B₁₄) and (A₆, B₁₀), parameters
    // exactly √2/2 and 1 − √2/2 (NOT the midpoint pin of verify_pair).
    let cr = gcrossings(&va, &vb);
    assert_eq!(cr.len(), 2, "AB crossings");
    assert_eq!((cr[0].ea, cr[0].eb), (2, 14), "AB crossing 0");
    assert_eq!((cr[1].ea, cr[1].eb), (6, 10), "AB crossing 1");
    let half_pin = rho163();
    let t0 = cr[0].t_num.mul(cr[0].den.inv());
    let t1 = cr[1].t_num.mul(cr[1].den.inv());
    assert!(t0.eq_x(half_pin), "AB t0 != √2/2");
    assert!(t1.eq_x(Kq16::int(1).sub(half_pin)), "AB t1 != 1 − √2/2");

    // Lens and areas, exact (the banked recon constants).
    let ov = clip_exact(&va, &vb);
    assert_eq!(ov.len(), 10, "AB lens vertices (2c+4)");
    let area = shoelace(&ov);
    assert!(
        area.eq_x(Kq16::new([9, -16, -5, 6], 2)),
        "AB overlap != (9−16b−5b²+6b³)/2"
    );
    assert!((area.fl() - 0.108325753).abs() < 1e-9, "AB overlap float");
    let poly = shoelace(&va);
    // Apothem-½ 16-gon area = 4·tan(π/16) exactly.
    assert!(poly.eq_x(t16().scale(4)), "16-gon area != 4 t16");
    assert!((poly.fl() - 0.795649470).abs() < 1e-9, "16-gon float");
    let un = poly.add(poly).sub(area);
    assert!(
        un.eq_x(Kq16::new([7, -16, -11, 10], 2)),
        "AB union != (7−16b−11b²+10b³)/2"
    );
    assert!((un.fl() - 1.482973186).abs() < 1e-9, "AB union float");

    // Unit-edge normalization: areas scale by 1/t₁₆², lengths by 1/t₁₆.
    let lam = t16().mul(t16()).inv();
    assert!(
        area.mul(lam).eq_x(Kq16::new([-1, -12, 1, 4], 2)),
        "unit-edge overlap"
    );
    assert!((area.mul(lam).fl() - 2.737840511).abs() < 1e-9);
    assert!(
        un.mul(lam).eq_x(Kq16::new([-15, -20, 15, 12], 2)),
        "unit-edge union"
    );
    assert!((un.mul(lam).fl() - 37.480875426).abs() < 1e-6);
    assert!(poly.mul(lam).eq_x(Kq16::lift(K16([-4, -8, 4, 4]))), "unit-edge 16-gon");
    assert!((poly.mul(lam).fl() - 4.0 / (std::f64::consts::PI / 16.0).tan()).abs() < 1e-8);

    // Displacement invariants.
    let dd = (cb.0.sub(ca.0), cb.1.sub(ca.1));
    let d2 = dd.0.mul(dd.0).add(dd.1.mul(dd.1));
    assert!(
        d2.eq_x(Kq16::lift(K16([4, 0, -1, 0]))),
        "|B−A|² != 2 − √2 (= 4 − b²)"
    );
    let p = (dd.0.mul(t16().inv()), dd.1.mul(t16().inv()));
    assert!(
        p.0.eq_x(Kq16::new([2, 6, -1, -2], 2)) && p.1.eq_x(Kq16::new([0, 2, 1, 0], 2)),
        "unit-edge displacement tuple"
    );
    let p2 = p.0.mul(p.0).add(p.1.mul(p.1));
    // |P| = 2 + b: |P|² = b⁴ + 4b + 4 = 8b² + 4b − 2… expand: (2+b)² = 4 + 4b + b².
    assert!(
        p2.eq_x(Kq16::lift(K16([4, 4, 1, 0]))),
        "|P|² != (2+b)²"
    );
    // Direction 112.5°: parallel to (cos 112.5°, sin 112.5°) = (−d/2, b/2).
    let dir = (s22().neg(), c22());
    assert_eq!(
        p.0.mul(dir.1).sub(p.1.mul(dir.0)).sgn(),
        Ordering::Equal,
        "P not along 112.5°"
    );
    assert_eq!(
        p.0.mul(dir.0).add(p.1.mul(dir.1)).sgn(),
        Ordering::Greater,
        "P anti-parallel to 112.5°"
    );
}

// =====================================================================
// Tests
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::medallion::Medallion;

    #[test]
    fn kq16_wrapper_inverts_and_normalizes() {
        // Canonical reduction: 2/4 = 1/2 and negative-denominator
        // sign flow; the b inverse matches the pinned 2/b rule.
        assert_eq!(Kq16::new([2, 0, 0, 0], 4).den, 2);
        assert_eq!(Kq16::new([-2, 0, 0, 0], -4).n, K16([1, 0, 0, 0]));
        let b = Kq16::lift(K16::B);
        let inv_b = b.inv();
        assert!(inv_b.eq_x(Kq16::new([0, 4, 0, -1], 2)), "1/b form");
        assert!(b.mul(inv_b).eq_x(Kq16::int(1)), "b·b⁻¹ != 1");
        // More inverses, each checked by multiplication.
        for c in [
            Kq16::int(3),
            Kq16::lift(K16::DD),
            Kq16::lift(K16([1, -1, 0, 1])),
            Kq16::new([-1, -1, 1, 0], 7),
        ] {
            let y = c.inv();
            assert!(
                c.mul(y).eq_x(Kq16::int(1)),
                "Kq16 inverse check: {c:?} -> {y:?}"
            );
        }
        // The cofactor determinant agrees with knorm (Sylvester).
        let m = mul_matrix(K16::SQRT2);
        let det = (0..4)
            .map(|i| {
                let mut c = minor3(&m, i);
                if i & 1 == 1 {
                    c = -c;
                }
                m[0][i] as i128 * c
            })
            .sum::<i128>();
        assert_eq!(det, K16::SQRT2.norm(), "adjugate det vs knorm");
        // sgn follows the numerator (den > 0 invariant).
        assert_eq!(Kq16::new([-1, 0, 1, 0], 2).sgn(), Ordering::Greater);
        assert_eq!(Kq16::new([3, 0, -1, 0], 5).sgn(), Ordering::Less);
    }

    #[test]
    fn hexadecagon_hull_is_the_normalized_unit_edge_schema() {
        let h = hull();
        assert_eq!(h.len(), 16);
        // V0 = 0, edge0 = +x unit.
        assert_eq!(h[0].0.sgn(), Ordering::Equal);
        assert_eq!(h[0].1.sgn(), Ordering::Equal);
        assert!(h[1].0.eq_x(Kq16::int(1)) && h[1].1.sgn() == Ordering::Equal);
        // Equal unit edges (exact squared lengths), strict convex CCW.
        for k in 0..16 {
            let e = (
                h[(k + 1) % 16].0.sub(h[k].0),
                h[(k + 1) % 16].1.sub(h[k].1),
            );
            assert!(e.0.mul(e.0).add(e.1.mul(e.1)).eq_x(Kq16::int(1)), "edge {k}");
        }
        // hull_sanity runs inside verify_pair/battery_record; the edge
        // angles step by 22.5° (E₁ = V₂ − V₁ = (b/2, d/2)).
        let h1e = (h[2].0.sub(h[1].0), h[2].1.sub(h[1].1));
        assert!(h1e.0.eq_x(c22()) && h1e.1.eq_x(s22()), "edge 1 direction");
    }

    #[test]
    fn hexadecagon_record_passes_the_schema_battery() {
        battery_record("{16/8}", &hull(), 8, cos2163());
        assert_eq!(<Monster168 as Medallion>::contact_cos2(), Some((0, 0, 2, 1)));
        assert_eq!(<Monster168 as Medallion>::SYM, 16);
        // Figure {16/8}: eight diameters listed from both ends.
        let f = figure();
        assert_eq!(f.len(), 16);
        for (a, b) in &f {
            assert_eq!((a + 8) % 16, *b);
        }
        // Documented degeneracy: 32 rows, both rows of a contact equal,
        // the diameter run outward from both ends ({12/6} pattern).
        let ex = exit_rays();
        assert_eq!(ex.len(), 32);
        let m = gmidpoints(&hull());
        for i in 0..16 {
            let rows: Vec<&Gp<Kq16>> = ex.iter().filter(|(c, _)| *c == i).map(|(_, v)| v).collect();
            assert_eq!(rows.len(), 2);
            assert!(
                rows[0].0.eq_x(rows[1].0) && rows[0].1.eq_x(rows[1].1),
                "coincident exit rows at contact {i}"
            );
            let want = (m[i].0.sub(m[(i + 8) % 16].0), m[i].1.sub(m[(i + 8) % 16].1));
            assert!(rows[0].0.eq_x(want.0) && rows[0].1.eq_x(want.1));
        }
    }

    #[test]
    fn placement_is_the_certified_pin() {
        let d = pair163();
        assert_eq!(d.e_ab, vec![7, 8, 9]);
        assert_eq!(d.e_ba, vec![0, 1, 15]);
        assert_eq!(d.exposed_a.len(), 12);
        assert_eq!(d.exposed_b.len(), 12);
        assert_eq!(d.union_cycle.len(), 26);
        assert_eq!(d.lens_cycle.len(), 10);
        // P = M₆ − M₂ on the vertical mid-edge axis; float 3.554865846.
        let m = gmidpoints(&hull());
        let pin = (m[6].0.sub(m[2].0), m[6].1.sub(m[2].1));
        assert!(d.p.0.eq_x(pin.0) && d.p.1.eq_x(pin.1));
        assert_eq!(d.p.0.sgn(), Ordering::Equal);
        assert!((d.p.1.fl() - 3.554865846).abs() < 1e-9);
    }

    #[test]
    fn pin_is_the_exact_midpoint_of_the_certified_window() {
        // Window (b² − b − 1, b − 1) from the exact axis event census
        // (scratch pin16.py); 2ρ = lo + hi identically.
        let (lo, hi) = (win_lo(), win_hi());
        assert!(rho163().scale(2).eq_x(lo.add(hi)), "2rho != lo + hi");
        assert_eq!(rho163().sub(lo).sgn(), Ordering::Greater);
        assert_eq!(rho163().sub(hi).sgn(), Ordering::Less);
        // lo ≈ 0.566454497, hi ≈ 0.847759065, pin ≈ 0.707106781.
        assert!((lo.fl() - 0.566454497).abs() < 1e-9);
        assert!((hi.fl() - 0.847759065).abs() < 1e-9);
        assert!((rho163().fl() - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-15);
    }

    #[test]
    fn ab_substrate_pair_is_the_monster_000_lens() {
        ab_substrate_battery();
    }
}

