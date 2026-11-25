//! Medallion interface — schema, exact catalog records, verification
//! batteries (design doc: `docs/medallion-interface-design.md`).
//!
//! A **medallion** is a self-contained high-degree ornament pinned by a
//! five-part exact interface record: a planar map on a declared convex
//! hull (normalized: vertex 0 at the origin, edge 0 along +x, CCW),
//! boundary **contacts** (corner or exact edge parameter), the internal
//! **figure** (exact segments whose endpoints are exactly the contacts),
//! **exit rays** (two outward exact direction vectors per contact), and
//! a declared cyclic symmetry + [`Absorption`] path. Contact
//! `t`-parameters and all `cos^2` angle data are similarity invariants,
//! so a medallion binds to a host hole without computing the scale
//! factor in the field (doc §2).
//!
//! This module is **data + tests only** — no [`Tiling`], no registry
//! entry (built-ins stay 162, memberships 183). It is the ground-truth
//! catalog for the §24b inference work: the five re-derived instances in
//! the submodules carry the exact tables the doc §4 tabulates, and
//! [`host312`] is the verified bind-and-absorb pipeline that 24b
//! consumes first (doc §7).
//!
//! Angle facts never appear as square roots or floats: every angle
//! claim is a `cos^2` identity cross-multiplied into the field, or a
//! zero cross product (doc §1.3). f64 lives only in the landed
//! cross-check tests (1e-9 vs the registered generators' output) and in
//! the [`Medallion`] trait's f64 consumer views.
//!
//! [`Tiling`]: tess_core::Tiling

pub mod girih_m161;
pub mod hexagram_62;
pub mod host312;
pub mod khatam_m53;
pub mod monster;
pub mod rosette_m56;
pub mod wis_m158;

use std::cmp::Ordering;

/// The exact field a record lives in (doc §1.1).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Field {
    /// Q(sqrt 2) — the 4.8.8 khatam PIC family.
    Qsqrt2,
    /// Q(sqrt 3) — the 3.12.12 PIC family, the hexagram, the WIS star
    /// (its raw `Z[sqrt3]` 4-tuple lattice embeds here at world = raw/2).
    Qsqrt3,
    /// Q(a), a = 2cos(pi/10), a^4 = 5a^2 - 5 — the girih field
    /// [`L10`](crate::fields::L10).
    L10,
}

/// A boundary contact of the internal figure (doc §1.4).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Contact {
    /// A hull vertex (star tips: hexagram, WIS).
    Corner(usize),
    /// The exact rational parameter `t_num / t_den` in the interior of
    /// hull edge `edge` (the catalog uses 1/2 throughout).
    Edge { edge: usize, t_num: i64, t_den: i64 },
}

/// How the surrounding pattern absorbs the exit rays (doc §3).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Absorption {
    /// (a) M163 extension-operator landing identity: the ray lands on
    /// the contact midpoint of a subsequent tile of the network
    /// (M53, M56, the hexagram's translated stars).
    LandingIdentity,
    /// (b) M94-style inference on boundary cells (catalogued panel
    /// motifs; the girih decagon's BOW/HEX neighbours). The landing
    /// verification is the §24b obligation (doc §4.3).
    BoundaryInference,
    /// (c) Direct interstitial continuation into declared tiles (the
    /// WIS star onto the 3.12.12 host patch, [`host312`]).
    Interstitial,
}

/// Common schema surface of a medallion record. The exact tables live
/// in the instance modules (each field has its own coordinate type);
/// these views are the frame the host binding works in, always in the
/// normalized record frame of doc §1.2.
pub trait Medallion {
    /// Catalog label.
    const ID: &'static str;
    /// Declared exact field.
    const FIELD: Field;
    /// Cyclic symmetry order (rotation about the centre).
    const SYM: usize;
    /// Declared ray-absorption obligation.
    const PATH: Absorption;
    /// Hull vertices, normalized frame, as f64 (consumer view).
    fn hull_f64() -> Vec<[f64; 2]>;
    /// Directed exit rays: `(contact index, direction)` as f64.
    fn exit_rays_f64() -> Vec<(usize, [f64; 2])>;
    /// Exact `cos^2` of the contact angle against the hull edge,
    /// encoded as `(a, b, d, den)` with value `(a + b sqrt(d)) / den`
    /// (`d = 0` for a rational). `None` for corner-contact records
    /// (their angle datum is the exit-ray multiset instead) and for the
    /// L10 instance, whose `cos^2 = (5 - a^2)/4` is pinned exactly in
    /// its own battery (doc §4.3).
    fn contact_cos2() -> Option<(i64, i64, i64, i64)> {
        None
    }
}

// =====================================================================
// Exact quadratic engine — (a + b*sqrt(d)) / den, integer numerators,
// positive denominator, gcd-reduced (a canonical form). Everything the
// batteries compute is polynomial (norm, cross, dot, shoelace) or an
// exact sign/equality decision; there is no field division on any
// checked path except the ray-parameter comparison, which is
// division-free (cross-multiplication, see [`first_hit`]).
// =====================================================================

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// gcd reduction of up to five integers (shared with the L10 wrapper).
pub(crate) fn gcd5(vals: [i64; 5]) -> i64 {
    vals.into_iter().fold(0, gcd).max(1)
}

/// Exact element `(a + b*sqrt(d)) / den`, `den > 0`, gcd-reduced —
/// the shipped quadratic `zcmp` sign pattern generalized to a small
/// struct (the catalog's `Q(sqrt2)`/`Q(sqrt3)` layers, doc §1.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Zq {
    pub a: i64,
    pub b: i64,
    pub d: i64,
    pub den: i64,
}

impl Zq {
    /// Integer value at denominator 1.
    pub const fn whole(d: i64, v: i64) -> Zq {
        Zq {
            a: v,
            b: 0,
            d,
            den: 1,
        }
    }
    /// Element `(a + b sqrt d) / den`, gcd-reduced.
    pub fn new(a: i64, b: i64, d: i64, den: i64) -> Zq {
        assert!(den > 0);
        let g = gcd(gcd(a, b), den);
        Zq {
            a: a / g,
            b: b / g,
            d,
            den: den / g,
        }
    }
    /// Zero in the field of discriminant `d`.
    pub fn zero(d: i64) -> Zq {
        Zq {
            a: 0,
            b: 0,
            d,
            den: 1,
        }
    }
    pub fn add(self, o: Zq) -> Zq {
        assert_eq!(self.d, o.d);
        Zq::new(
            self.a * o.den + o.a * self.den,
            self.b * o.den + o.b * self.den,
            self.d,
            self.den * o.den,
        )
    }
    pub fn sub(self, o: Zq) -> Zq {
        self.add(o.neg())
    }
    pub fn neg(self) -> Zq {
        Zq {
            a: -self.a,
            b: -self.b,
            ..self
        }
    }
    /// Field product: `(a + b r)(c + e r) = (ac + d b e) + (a e + b c) r`.
    pub fn mul(self, o: Zq) -> Zq {
        let a = self.a as i128 * o.a as i128 + self.d as i128 * self.b as i128 * o.b as i128;
        let b = self.a as i128 * o.b as i128 + self.b as i128 * o.a as i128;
        Zq::new(a as i64, b as i64, self.d, self.den * o.den)
    }
    /// Exact integer scaling (a field multiply by an integer).
    pub fn scale(self, c: i64) -> Zq {
        Zq::new(self.a * c, self.b * c, self.d, self.den)
    }
    /// Multiply by `sqrt(d)` in-field: `(a + b r) r = d b + a r`.
    pub fn sqrt_mul(self) -> Zq {
        Zq::new(self.d * self.b, self.a, self.d, self.den)
    }
    /// Exact sign. For mixed signs of `a`, `b` (with `d` squarefree):
    /// `sgn(a + b r) = sgn(a^2 - d b^2)` when `a > 0` and its negation
    /// when `a < 0`, since `a + b r = (a^2 - d b^2) / (a - b r)` and
    /// `a - b r > 0` exactly when `a > 0 >= b` (shipped `zcmp` pattern).
    pub fn sgn(self) -> Ordering {
        if self.a >= 0 && self.b >= 0 {
            return if self.a == 0 && self.b == 0 {
                Ordering::Equal
            } else {
                Ordering::Greater
            };
        }
        if self.a <= 0 && self.b <= 0 {
            return if self.a == 0 && self.b == 0 {
                Ordering::Equal
            } else {
                Ordering::Less
            };
        }
        let disc =
            self.a as i128 * self.a as i128 - self.d as i128 * self.b as i128 * self.b as i128;
        let o = disc.cmp(&0);
        if self.a > 0 {
            o
        } else {
            o.reverse()
        }
    }
    /// Exact equality across denominators (cross-multiplication).
    pub fn eq_x(self, o: Zq) -> bool {
        self.d == o.d
            && self.a as i128 * o.den as i128 == o.a as i128 * self.den as i128
            && self.b as i128 * o.den as i128 == o.b as i128 * self.den as i128
    }
    /// Float value — landed-cross-check and consumer views only.
    pub fn fl(self) -> f64 {
        (self.a as f64 + self.b as f64 * (self.d as f64).sqrt()) / self.den as f64
    }
}

/// An exact point: a pair of field elements.
pub type Qp = (Zq, Zq);

/// Vector from `o` to `p`.
pub fn vsub(p: Qp, o: Qp) -> Qp {
    (p.0.sub(o.0), p.1.sub(o.1))
}
/// Vector sum.
pub fn vadd(p: Qp, o: Qp) -> Qp {
    (p.0.add(o.0), p.1.add(o.1))
}
/// Componentwise field product (scaling a vector by a field element).
pub fn vmul(p: Qp, s: Zq) -> Qp {
    (p.0.mul(s), p.1.mul(s))
}
/// Exact-integer scaling of a vector.
pub fn vscale(p: Qp, c: i64) -> Qp {
    (p.0.scale(c), p.1.scale(c))
}
/// Exact midpoint.
pub fn midpoint(a: Qp, b: Qp) -> Qp {
    (qhalf(a.0.add(b.0)), qhalf(a.1.add(b.1)))
}
/// Midpoints of a polygon's edges (the PIC contact set of a tile).
pub fn midpoints(poly: &[Qp]) -> Vec<Qp> {
    let n = poly.len();
    (0..n)
        .map(|i| midpoint(poly[i], poly[(i + 1) % n]))
        .collect()
}
/// Squared length.
pub fn norm2(u: Qp) -> Zq {
    u.0.mul(u.0).add(u.1.mul(u.1))
}
/// Dot product.
pub fn dot(u: Qp, v: Qp) -> Zq {
    u.0.mul(v.0).add(u.1.mul(v.1))
}
/// z-component of the cross product u x v.
pub fn cross(u: Qp, v: Qp) -> Zq {
    u.0.mul(v.1).sub(u.1.mul(v.0))
}
/// Cross product (b - a) x (c - a).
pub fn cross3(a: Qp, b: Qp, c: Qp) -> Zq {
    cross(vsub(b, a), vsub(c, a))
}
/// Doubled signed area, CCW-positive.
pub fn area2(poly: &[Qp]) -> Zq {
    let d = poly[0].0.d;
    let mut acc = Zq::zero(d);
    for i in 0..poly.len() {
        let (x1, y1) = poly[i];
        let (x2, y2) = poly[(i + 1) % poly.len()];
        acc = acc.add(x1.mul(y2).sub(x2.mul(y1)));
    }
    acc
}
/// Vertex sum `n * centroid` (keeps centroid comparisons integral).
pub fn vertex_sum(poly: &[Qp]) -> Qp {
    poly.iter()
        .fold((Zq::zero(poly[0].0.d), Zq::zero(poly[0].0.d)), |s, p| {
            (s.0.add(p.0), s.1.add(p.1))
        })
}

/// Divide an element by 2 exactly (gcd reduction keeps integrality).
pub fn qhalf(z: Zq) -> Zq {
    Zq::new(z.a, z.b, z.d, z.den * 2)
}

/// Rotate by +30 degrees in Q(sqrt 3): `(x, y) -> ((x sqrt3 - y)/2,
/// (x + y sqrt3)/2)`.
pub fn rot30(p: Qp) -> Qp {
    (
        qhalf(p.0.sqrt_mul().sub(p.1)),
        qhalf(p.0.add(p.1.sqrt_mul())),
    )
}

/// Rotate by 30*n degrees (order 12).
pub fn rot30n(p: Qp, n: usize) -> Qp {
    let mut p = p;
    for _ in 0..n % 12 {
        p = rot30(p);
    }
    p
}

/// Strictly convex + CCW: every consecutive triple turns left.
pub fn strictly_ccw(poly: &[Qp]) -> bool {
    (0..poly.len()).all(|i| {
        cross3(
            poly[i],
            poly[(i + 1) % poly.len()],
            poly[(i + 2) % poly.len()],
        )
        .sgn()
            == Ordering::Greater
    })
}

/// All hull edges have the same squared length.
pub fn equal_edges(poly: &[Qp]) -> bool {
    let e0 = norm2(vsub(poly[1], poly[0]));
    (1..poly.len()).all(|i| norm2(vsub(poly[(i + 1) % poly.len()], poly[i])).eq_x(e0))
}

/// Normalized-frame check (doc §1.2): V0 = origin, edge 0 = `unit` along +x.
pub fn normalized_frame(poly: &[Qp], unit: Zq) -> bool {
    let d = poly[0].0.d;
    poly[0].0.eq_x(Zq::zero(d))
        && poly[0].1.eq_x(Zq::zero(d))
        && poly[1].1.eq_x(Zq::zero(d))
        && poly[1].0.eq_x(unit)
}

/// Exact division by a positive integer (a field multiply by `1/k`, kept
/// as a denominator — used for centroids).
pub fn div_i(z: Zq, k: i64) -> Zq {
    Zq::new(z.a, z.b, z.d, z.den * k)
}

/// Point equality, exact.
pub fn pt_eq(p: Qp, q: Qp) -> bool {
    p.0.eq_x(q.0) && p.1.eq_x(q.1)
}

// =====================================================================
// Exact ray-vs-polygon first hit (division-free). Used by the path-(c)
// absorption battery in [`host312`].
// =====================================================================

/// One candidate ray/edge intersection: the ray `P + t u` meets edge
/// `edge` of `tile` at `t = t_num / t_den` (exact elements of one `d`,
/// `t_den > 0` normalized; `t` is never actually divided).
#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub t_num: Zq,
    pub t_den: Zq,
    pub tile: usize,
    pub edge: usize,
}

/// First intersection of ray `(p, u)` with the edges of `tiles`.
/// Comparisons cross-multiply ([`cmp_t`]); no division happens.
pub fn first_hit(p: Qp, u: Qp, tiles: &[Vec<Qp>]) -> Option<Hit> {
    let mut best: Option<Hit> = None;
    for (ti, poly) in tiles.iter().enumerate() {
        let n = poly.len();
        for j in 0..n {
            let a = poly[j];
            let b = poly[(j + 1) % n];
            let e = vsub(b, a);
            let mut den = cross(u, e);
            let mut num_t = cross(vsub(a, p), e);
            let mut num_s = cross(vsub(a, p), u);
            if den.sgn() == Ordering::Equal {
                continue;
            }
            if den.sgn() == Ordering::Less {
                den = den.neg();
                num_t = num_t.neg();
                num_s = num_s.neg();
            }
            // require t > 0 and 0 <= s <= 1 (den > 0 now)
            if num_t.sgn() != Ordering::Greater {
                continue;
            }
            if num_s.sgn() == Ordering::Less {
                continue;
            }
            if den.sub(num_s).sgn() == Ordering::Less {
                continue;
            }
            let cand = Hit {
                t_num: num_t,
                t_den: den,
                tile: ti,
                edge: j,
            };
            best = match best {
                None => Some(cand),
                Some(h) => {
                    if cmp_t(&cand, &h) == Ordering::Less {
                        Some(cand)
                    } else {
                        Some(h)
                    }
                }
            };
        }
    }
    best
}

/// Exact comparison of two hit parameters (cross-multiplication; both
/// denominators are positive, so the sign of the difference decides).
pub fn cmp_t(a: &Hit, b: &Hit) -> Ordering {
    a.t_num.mul(b.t_den).sub(b.t_num.mul(a.t_den)).sgn()
}

/// Is `t_num / t_den` exactly the value `v`?
pub fn t_is(h: &Hit, v: Zq) -> bool {
    v.mul(h.t_den).eq_x(h.t_num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zq_engine_exact_identities() {
        let d = 3;
        let s3 = Zq::new(0, 1, d, 1);
        // (sqrt3)^2 = 3 exactly
        assert!(s3.mul(s3).eq_x(Zq::whole(d, 3)));
        // sign engine against the quadratic zcmp rule
        assert_eq!(Zq::new(5, -3, d, 1).sgn(), Ordering::Less); // 5 < 3sqrt3
        assert_eq!(Zq::new(-5, 3, d, 1).sgn(), Ordering::Greater);
        assert_eq!(Zq::new(1, -1, d, 1).sgn(), Ordering::Less); // 1 < sqrt3
        assert_eq!(Zq::new(-1, 1, d, 1).sgn(), Ordering::Greater);
        assert_eq!(Zq::new(0, 0, d, 1).sgn(), Ordering::Equal);
        // gcd reduction and cross-denominator equality
        assert!(Zq::new(2, 4, d, 6).eq_x(Zq::new(1, 2, d, 3)));
        let f = Zq::new(3, 2, d, 4);
        assert!((f.fl() - (3.0 + 2.0 * 3f64.sqrt()) / 4.0).abs() < 1e-15);
        // rot30 is an exact isometry of order 12
        let p = (Zq::new(1, 2, d, 1), Zq::new(-3, 1, d, 1));
        let r = rot30n(p, 12);
        assert!(r.0.eq_x(p.0) && r.1.eq_x(p.1));
        let n0 = norm2(p);
        let n1 = norm2(rot30n(p, 5));
        assert!(n0.eq_x(n1));
        // midpoint is exact
        let m = midpoint(p, (Zq::zero(d), Zq::zero(d)));
        assert!(vscale(m, 2).0.eq_x(p.0) && vscale(m, 2).1.eq_x(p.1));
    }

    #[test]
    fn first_hit_quadrants_and_parameters() {
        // Ray from origin along +x into a unit square centred at (2, 0).
        let square = vec![vec![
            (Zq::whole(3, 1), Zq::whole(3, -1)),
            (Zq::whole(3, 3), Zq::whole(3, -1)),
            (Zq::whole(3, 3), Zq::whole(3, 1)),
            (Zq::whole(3, 1), Zq::whole(3, 1)),
        ]];
        let o = (Zq::zero(3), Zq::zero(3));
        let ux = (Zq::whole(3, 1), Zq::zero(3));
        let h = first_hit(o, ux, &square).expect("hit");
        assert_eq!(h.tile, 0);
        assert_eq!(h.edge, 3); // closing edge (1,1) -> (1,-1)
        assert!(t_is(&h, Zq::whole(3, 1))); // t = 1
                                            // miss case: ray along -y never reaches the square
        let uy = (Zq::zero(3), Zq::whole(3, -1));
        assert!(first_hit(o, uy, &square).is_none());
    }
}
