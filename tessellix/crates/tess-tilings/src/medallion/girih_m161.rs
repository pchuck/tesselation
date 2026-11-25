//! M161 — the girih decagon panel medallion: exact record + verification
//! battery (design doc §4.3).
//!
//! The medallion is the unit-edge regular girih decagon of the Darb-i
//! Imam patch in the normalized record frame (doc §1.2), living exactly
//! in the girih field `L10 = Q(a)`, `a = 2cos(pi/10)` — the landed
//! denominator-64 convention of the patch tables. Contacts are the ten
//! edge midpoints, the figure is the `{10/3}` decagram strap (contact
//! `i -> i + 3`: the tangent-chord closure `2*54 = 3*36` on the midpoint
//! incircle), and the 20 exit rays are the two chord continuations at
//! every contact. Declared symmetry: 10 (rotation by 36 degrees about
//! the centre).
//!
//! Ground truth (doc §4.3): the landed `girih-decagon-patch` table
//! `DEC_V` (the Python battery certified an EXACT rotation-and-align
//! match of this record hull against it — scratch `m161_final.txt`,
//! rotation 36*3, aligned at vertex 0 — re-verified here by the 1e-9
//! emitted-tile fit) and the exact closure identity: each figure chord,
//! marched forward from its contact, hits the decagon boundary exactly
//! at the contact midpoint three edges on. Strap TILES are not shipped
//! on the patch ("strapwork NOT shipped"), so this is path (b)
//! [`Absorption::BoundaryInference`]: ray absorption through BOW/HEX
//! neighbours is the §24b obligation (doc §4.3), pinned only by the
//! declared path here.

use std::cmp::Ordering;

use super::gcd5;
use crate::fields::L10;

// =====================================================================
// L10 coordinates with an exact denominator: `(numerator) / den`,
// `den > 0`, gcd-reduced so intermediate denominators never balloon.
// Every comparison cross-multiplies; there is no division on any path.
// =====================================================================

/// An element of `Q(a)` over a positive integer denominator.
#[derive(Clone, Copy, Debug)]
pub struct Kq {
    pub n: L10,
    pub den: i64,
}

impl Kq {
    /// `numerator / den`, gcd-reduced (den > 0 required).
    pub fn new(numerator: [i64; 4], den: i64) -> Kq {
        assert!(den > 0);
        let g = gcd5([numerator[0], numerator[1], numerator[2], numerator[3], den]);
        Kq {
            n: L10([
                numerator[0] / g,
                numerator[1] / g,
                numerator[2] / g,
                numerator[3] / g,
            ]),
            den: den / g,
        }
    }
    /// The zero element.
    pub fn zero() -> Kq {
        Kq {
            n: L10([0, 0, 0, 0]),
            den: 1,
        }
    }
    /// Exact sum (cross-multiplication).
    pub fn add(self, o: Kq) -> Kq {
        let mut num = [0i64; 4];
        for i in 0..4 {
            num[i] = self.n.0[i] * o.den + o.n.0[i] * self.den;
        }
        Kq::new(num, self.den * o.den)
    }
    /// Exact difference.
    pub fn sub(self, o: Kq) -> Kq {
        let mut num = [0i64; 4];
        for i in 0..4 {
            num[i] = self.n.0[i] * o.den - o.n.0[i] * self.den;
        }
        Kq::new(num, self.den * o.den)
    }
    /// Exact negation.
    pub fn neg(self) -> Kq {
        Kq {
            n: self.n.neg(),
            den: self.den,
        }
    }
    /// Field product.
    pub fn mul(self, o: Kq) -> Kq {
        Kq {
            n: self.n.mul(o.n),
            den: self.den * o.den,
        }
    }
    /// Exact sign (the shipped isolating-interval engine; `den > 0`).
    pub fn sgn(self) -> Ordering {
        self.n.sign()
    }
    /// Exact equality across denominators (cross-multiplication).
    pub fn eq_x(self, o: Kq) -> bool {
        (0..4).all(|i| self.n.0[i] * o.den == o.n.0[i] * self.den)
    }
    /// Float value — landed cross-check views only.
    pub fn fl(self) -> f64 {
        self.n.float() / self.den as f64
    }
}

/// A point of the girih plane.
pub type Kp = (Kq, Kq);

fn ksub(p: Kp, o: Kp) -> Kp {
    (p.0.sub(o.0), p.1.sub(o.1))
}
#[cfg(test)]
fn kadd(p: Kp, o: Kp) -> Kp {
    (p.0.add(o.0), p.1.add(o.1))
}
#[cfg(test)]
fn kdot(u: Kp, v: Kp) -> Kq {
    u.0.mul(v.0).add(u.1.mul(v.1))
}
#[cfg(test)]
fn kcross(u: Kp, v: Kp) -> Kq {
    u.0.mul(v.1).sub(u.1.mul(v.0))
}
#[cfg(test)]
fn knorm2(u: Kp) -> Kq {
    kdot(u, u)
}
/// Exact midpoint.
fn kmid(a: Kp, b: Kp) -> Kp {
    (
        Kq::new(a.0.add(b.0).n.0, a.0.add(b.0).den * 2),
        Kq::new(a.1.add(b.1).n.0, a.1.add(b.1).den * 2),
    )
}
#[cfg(test)]
/// Doubled signed area, CCW-positive.
fn karea2(poly: &[Kp]) -> Kq {
    let mut acc = Kq::zero();
    for i in 0..poly.len() {
        let (x1, y1) = poly[i];
        let (x2, y2) = poly[(i + 1) % poly.len()];
        acc = acc.add(x1.mul(y2).sub(x2.mul(y1)));
    }
    acc
}
#[cfg(test)]
fn kpt_eq(p: Kp, q: Kp) -> bool {
    p.0.eq_x(q.0) && p.1.eq_x(q.1)
}

// =====================================================================
// The record
// =====================================================================

/// Hull vertices as L10 numerators over 64, basis `(1, a, a^2, a^3)`,
/// normalized frame: V0 = origin, edge 0 = (1, 0), CCW.
pub const HULL_TBL: [([i64; 4], [i64; 4]); 10] = [
    ([0, 0, 0, 0], [0, 0, 0, 0]),
    ([64, 0, 0, 0], [0, 0, 0, 0]),
    ([0, 0, 32, 0], [0, -96, 0, 32]),
    ([-96, 0, 64, 0], [0, -64, 0, 32]),
    ([0, 0, 32, 0], [0, -32, 0, 32]),
    ([64, 0, 0, 0], [0, -128, 0, 64]),
    ([0, 0, 0, 0], [0, -128, 0, 64]),
    ([64, 0, -32, 0], [0, -32, 0, 32]),
    ([160, 0, -64, 0], [0, -64, 0, 32]),
    ([64, 0, -32, 0], [0, -96, 0, 32]),
];

/// The record's trait surface.
pub struct GirihM161;

impl super::Medallion for GirihM161 {
    const ID: &'static str = "girih decagon patch panel (M161)";
    const FIELD: super::Field = super::Field::L10;
    const SYM: usize = 10;
    const PATH: super::Absorption = super::Absorption::BoundaryInference;

    fn hull_f64() -> Vec<[f64; 2]> {
        hull().iter().map(|p| [p.0.fl(), p.1.fl()]).collect()
    }
    fn exit_rays_f64() -> Vec<(usize, [f64; 2])> {
        exit_rays()
            .into_iter()
            .map(|(i, d)| (i, [d.0.fl(), d.1.fl()]))
            .collect()
    }
    // cos^2 = (5 - a^2)/4 is not of the trait's rational-sqrt tuple
    // form; the battery below pins it exactly in L10 (doc §4.3).
}

/// Hull vertices, exact.
pub fn hull() -> Vec<Kp> {
    HULL_TBL
        .iter()
        .map(|&(x, y)| (Kq::new(x, 64), Kq::new(y, 64)))
        .collect()
}

/// The ten contacts: every hull edge midpoint (`t = 1/2`).
pub fn contacts() -> Vec<Kp> {
    let h = hull();
    (0..10).map(|i| kmid(h[i], h[(i + 1) % 10])).collect()
}

/// The figure: the ten `{10/3}` strap chords as contact-index pairs.
pub fn figure() -> Vec<(usize, usize)> {
    (0..10).map(|i| (i, (i + 3) % 10)).collect()
}

/// The 20 directed exit rays `(contact, direction)`: both chord
/// continuations `M_i - M_{i±3}` at every contact.
pub fn exit_rays() -> Vec<(usize, Kp)> {
    let m = contacts();
    let mut out = Vec::with_capacity(20);
    for s in [3usize, 7] {
        for i in 0..10 {
            out.push((i, ksub(m[i], m[(i + s) % 10])));
        }
    }
    out
}

#[cfg(test)]
/// Rotation by +36 degrees in L10: cos = (a^2 - 2)/2, sin = (a^3 - 3a)/2.
fn rot36(p: Kp) -> Kp {
    let c = Kq::new([-2, 0, 1, 0], 2);
    let s = Kq::new([0, -3, 0, 1], 2);
    (p.0.mul(c).sub(p.1.mul(s)), p.0.mul(s).add(p.1.mul(c)))
}

#[cfg(test)]
/// The centre of the record (vertex sum / 10).
fn centre() -> Kp {
    let h = hull();
    let mut s = (Kq::zero(), Kq::zero());
    for p in &h {
        s = (s.0.add(p.0), s.1.add(p.1));
    }
    (
        Kq::new(s.0.n.0, s.0.den * 10),
        Kq::new(s.1.n.0, s.1.den * 10),
    )
}

#[cfg(test)]
/// The unit element.
fn one() -> Kq {
    Kq::new([1, 0, 0, 0], 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_schema_invariants() {
        let h = hull();
        // strict convexity + CCW: every consecutive triple turns left.
        for i in 0..10 {
            let a = h[i];
            let b = h[(i + 1) % 10];
            let c = h[(i + 2) % 10];
            assert_eq!(
                kcross(ksub(b, a), ksub(c, a)).sgn(),
                Ordering::Greater,
                "convex at vertex {i}"
            );
        }
        // unit edges, normalized frame.
        for i in 0..10 {
            assert!(
                knorm2(ksub(h[(i + 1) % 10], h[i])).eq_x(one()),
                "edge {i} is unit"
            );
        }
        assert!(h[0].0.eq_x(Kq::zero()) && h[0].1.eq_x(Kq::zero()));
        assert!(h[1].0.eq_x(one()) && h[1].1.eq_x(Kq::zero()));
        // doubled area = 5a^3 - 10a (unit regular decagon: area
        // (5/2) sqrt(5 + 2 sqrt 5), exact in L10).
        assert!(karea2(&h).eq_x(Kq::new([0, -10, 0, 5], 1)), "area2");
        // circumradius^2 single-valued = a^2 - 1 (unit regular decagon:
        // R = 1 / (2 sin 18deg), R^2 = (3 + sqrt5)/2 = a^2 - 1).
        let c = centre();
        for p in &h {
            assert!(
                knorm2(ksub(*p, c)).eq_x(Kq::new([-1, 0, 1, 0], 1)),
                "circumradius^2 = a^2 - 1"
            );
        }
        // figure degrees and exit pairing.
        let fig = figure();
        for i in 0..10 {
            let n = fig.iter().filter(|(a, b)| *a == i || *b == i).count();
            assert_eq!(n, 2, "contact {i} figure degree");
        }
        let m = contacts();
        let ex = exit_rays();
        assert_eq!(ex.len(), 20);
        for i in 0..10 {
            assert_eq!(ex.iter().filter(|(cc, _)| *cc == i).count(), 2);
        }
        for s in [3usize, 7] {
            for i in 0..10 {
                let want = ksub(m[i], m[(i + s) % 10]);
                assert!(
                    ex.iter().any(|(cc, d)| *cc == i && kpt_eq(*d, want)),
                    "exit continuation at contact {i}"
                );
            }
        }
        // exits outward: dot(direction, contact - centre) > 0.
        for (i, d) in &ex {
            let w = ksub(m[*i], c);
            assert_eq!(kdot(*d, w).sgn(), Ordering::Greater, "outward");
        }
    }

    #[test]
    fn strap_exits_land_on_contacts_closure() {
        // The path-(b) core identity (certified Python-side against the
        // same math, scratch m161_decagon.py): march each figure chord
        // forward from its contact; the FIRST forward boundary hit is
        // exactly the contact midpoint three edges on, at t = 1
        // (division-free comparisons, same pattern as first_hit).
        let h = hull();
        let m = contacts();
        for (a, b) in figure() {
            let p = m[a];
            let d = ksub(m[b], p);
            let mut best: Option<(Kq, Kq)> = None; // (t_num, t_den)
            for j in 0..10 {
                let e = ksub(h[(j + 1) % 10], h[j]);
                let mut den = kcross(d, e);
                if den.sgn() == Ordering::Equal {
                    continue;
                }
                let am = ksub(h[j], p);
                let mut nt = kcross(am, e);
                let mut ns = kcross(am, d);
                if den.sgn() == Ordering::Less {
                    den = den.neg();
                    nt = nt.neg();
                    ns = ns.neg();
                }
                if nt.sgn() != Ordering::Greater
                    || ns.sgn() == Ordering::Less
                    || den.sub(ns).sgn() == Ordering::Less
                {
                    continue;
                }
                best = match best {
                    None => Some((nt, den)),
                    Some((bt, bd)) => {
                        if nt.mul(bd).sub(bt.mul(den)).sgn() == Ordering::Less {
                            Some((nt, den))
                        } else {
                            Some((bt, bd))
                        }
                    }
                };
            }
            let (nt, den) = best.expect("chord ray hits the boundary");
            assert!(
                nt.eq_x(den),
                "strap chord {a}->{b}: first exit is the contact midpoint at t = 1"
            );
        }
    }

    #[test]
    fn contact_angle_is_single_valued() {
        // cos^2(chord, edge) = (5 - a^2)/4 = 1 - C36^2 = cos^2 54 deg,
        // exact for all ten chords: 4 * dot^2 == (5 - a^2) |u|^2 |e|^2.
        let h = hull();
        let m = contacts();
        let five_minus_a2 = Kq::new([5, 0, -1, 0], 1);
        for (a, b) in figure() {
            let u = ksub(m[b], m[a]);
            let e = ksub(h[(a + 1) % 10], h[a]);
            let d = kdot(u, e);
            let lhs = Kq::new([4, 0, 0, 0], 1).mul(d).mul(d);
            let rhs = five_minus_a2.mul(knorm2(u)).mul(knorm2(e));
            assert!(lhs.eq_x(rhs), "contact {a}: cos^2 = (5 - a^2)/4");
        }
    }

    #[test]
    fn exit_direction_census() {
        // cos^2 with +x over the 20 exits: {0: 4, (5 - a^2)/4: 8,
        // a^2/4: 8}; all exits share the single chord length^2.
        let ex = exit_rays();
        let (mut n0, mut n54, mut n18) = (0, 0, 0);
        let mut chord: Option<Kq> = None;
        let five_minus_a2 = Kq::new([5, 0, -1, 0], 1);
        let a2 = Kq::new([0, 0, 1, 0], 1);
        let four = Kq::new([4, 0, 0, 0], 1);
        for (_, u) in &ex {
            let n = knorm2(*u);
            match chord {
                None => chord = Some(n),
                Some(c) => assert!(n.eq_x(c), "single chord length^2"),
            }
            if u.0.sgn() == Ordering::Equal {
                n0 += 1;
            } else if four.mul(u.0).mul(u.0).eq_x(five_minus_a2.mul(n)) {
                n54 += 1;
            } else if four.mul(u.0).mul(u.0).eq_x(a2.mul(n)) {
                n18 += 1;
            } else {
                panic!("unexpected exit direction");
            }
        }
        assert_eq!((n0, n54, n18), (4, 8, 8), "exit cos^2 census");
    }

    #[test]
    fn sym10_rotation_maps_record_to_itself() {
        let h = hull();
        let c = centre();
        let rot_about = |p: Kp| kadd(rot36(ksub(p, c)), c);
        let w0 = rot_about(h[0]);
        let mut m = None;
        for j in 0..10 {
            if kpt_eq(w0, h[j]) {
                m = Some(j);
            }
        }
        let m = m.expect("V0 image is a vertex");
        for i in 0..10 {
            assert!(kpt_eq(rot_about(h[i]), h[(i + m) % 10]), "sym10 vertex {i}");
        }
        let mm = contacts();
        for i in 0..10 {
            assert!(kpt_eq(rot_about(mm[i]), mm[(i + m) % 10]));
        }
        // exit-ray set maps to itself.
        let ex = exit_rays();
        for (i, d) in &ex {
            let rd = rot36(*d);
            assert!(
                ex.iter().any(|(j, w)| *j == (i + m) % 10 && kpt_eq(*w, rd)),
                "sym10 on exit ray {i}"
            );
        }
    }

    #[test]
    fn hull_fits_a_landed_patch_decagon() {
        // Landed cross-check: the emitted `girih-decagon-patch` decagon
        // tiles are regular decagons; the record hull is congruent to
        // them at 1e-9 after unit-edge normalization (the seed cluster
        // itself is unit-edged, so a scale-1 fit occurs). The exact
        // table-level match vs the shipped DEC_V table is certified in
        // scratch m161_final.txt (rotation 36*3, aligned at vertex 0).
        use crate::families::islamic::girih_decagon_patch::GirihDecagonPatchTiling;
        use tess_core::{BBox, ParamSet, Query, Tiling, Vec2};
        let res = GirihDecagonPatchTiling.generate(
            &ParamSet::default_for(GirihDecagonPatchTiling.params()),
            &Query {
                region: BBox::new(Vec2::new(-60.0, -60.0), Vec2::new(60.0, 60.0)),
            },
        );
        assert!(res.message.is_none(), "{:?}", res.message);
        let decs: Vec<_> = res.tiles.iter().filter(|t| t.poly.len() == 10).collect();
        assert!(decs.len() >= 10, "the patch emits decagons");
        let h = hull();
        let c = centre();
        let hp: Vec<[f64; 2]> = h
            .iter()
            .map(|p| [p.0.fl() - c.0.fl(), p.1.fl() - c.1.fl()])
            .collect();
        let mut unit_fit = false;
        for tile in &decs {
            let n = tile.poly.len();
            let mut tc = [0.0; 2];
            for p in &tile.poly.pts {
                tc[0] += p.x / n as f64;
                tc[1] += p.y / n as f64;
            }
            // regularity of the landed tile
            let e0 = {
                let a = tile.poly.pts[0];
                let b = tile.poly.pts[1];
                ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt()
            };
            let r0 = (tile.poly.pts[0].x - tc[0]).powi(2) + (tile.poly.pts[0].y - tc[1]).powi(2);
            let mut regular = true;
            for k in 0..n {
                let a = tile.poly.pts[k];
                let b = tile.poly.pts[(k + 1) % n];
                let e2 = (b.x - a.x).powi(2) + (b.y - a.y).powi(2);
                let r2 = (a.x - tc[0]).powi(2) + (a.y - tc[1]).powi(2);
                if (e2 - e0 * e0).abs() > 1e-9 || (r2 - r0).abs() > 1e-9 {
                    regular = false;
                }
            }
            assert!(regular, "landed decagon tile is regular");
            // scale-normalized congruence with cyclic alignment; the
            // tile is CCW-ordered for the comparison and the rotation
            // angle is anchored on the candidate first correspondence
            // (substitution orientations are not assumed on-grid).
            let mut q: Vec<[f64; 2]> = tile
                .poly
                .pts
                .iter()
                .map(|p| [(p.x - tc[0]) / e0, (p.y - tc[1]) / e0])
                .collect();
            let mut area = 0.0;
            for k in 0..10 {
                let a = q[k];
                let b = q[(k + 1) % 10];
                area += a[0] * b[1] - b[0] * a[1];
            }
            if area < 0.0 {
                q.reverse();
            }
            let fits = (0..10).any(|shift| {
                let th = (q[0][1]).atan2(q[0][0]) - hp[shift][1].atan2(hp[shift][0]);
                let (coss, sinn) = (th.cos(), th.sin());
                (0..10).all(|k| {
                    let v = hp[(shift + k) % 10];
                    let x = coss * v[0] - sinn * v[1];
                    let y = sinn * v[0] + coss * v[1];
                    (x - q[k][0]).abs() < 1e-9 && (y - q[k][1]).abs() < 1e-9
                })
            });
            assert!(fits, "record hull fits a landed decagon at 1e-9");
            if (e0 - 1.0).abs() < 1e-9 {
                unit_fit = true;
            }
        }
        assert!(unit_fit, "the patch carries unit-edge seed decagons");
    }
}
