//! Design-element motifs (Kaplan dissertation §3.5) for Hankin substrates.
//!
//! Three design elements are generated per substrate tile, together with
//! the per-tile motif assignment entry point
//! [`build_cells_with_motifs`] / [`hankin::build_cells_with_segments`]:
//!
//! * [`Motif::Star`] `(n, s, theta)` (§3.5.1): one arm path per edge
//!   midpoint ("contact") — the segment leaving the contact at angle
//!   `theta` to the edge — replicated under d_n **both chiralities**;
//!   every arm path is cut at its crossings with the other paths,
//!   keeping the first `s` subpaths.
//! * [`Motif::Rosette`] `(n, s, theta, h, phi)` (§3.5.2): the arm path
//!   bends at a knee G — the point on the leg-1 line (through the
//!   contact at angle `theta`) at signed height `h · α · |A − M|` above
//!   the contact radius, `α = 1 − sin(π/n)` — and leaves G along a ray
//!   turned `phi` from the inward radius. At `theta = π/n` (the *ideal*
//!   angle) with `h = 1`, `phi = 0` this is Lee's rosette: with `C` the
//!   point on the forward vertex radius at distance `|A − M|` from the
//!   vertex, the knee G is equidistant from the contact M and C, and
//!   the ray leaves G exactly through C; the knee identities are pinned
//!   in the tests to ~1e-13 (Python battery worst offset 1.7e-16).
//! * [`Motif::ExtendedStar`] / [`Motif::ExtendedRosette`] (§3.5.4):
//!   the extension operator. The designer fixes the *realized outer*
//!   contact angle `theta` at the parent edges, `π/n < theta < π/2`;
//!   the child element is the standalone motif at contact angle
//!   `theta′ = theta − π/n`, circumradius `r′ = r · cos(theta) /
//!   cos(π/n − theta)`, rotated π/n, drawn inside the parent. Every
//!   child arm line passes **exactly** through a parent edge midpoint
//!   on its backward extension (identity `−F·cos(theta) + cos(theta −
//!   π/n) = 0` with `F = cos theta / cos(π/n − theta)`; landing radius
//!   `r · cos(π/n)`), so the segment child-contact → parent-midpoint
//!   is the *extension spur*.
//!
//! **Erratum note.** The dissertation (2111) prints the child angle as
//! `θ/2`; that coincides with `θ − π/n` only at `θ = 2π/n`. The landing
//! reading above was validated symbolically and numerically (fit ratio
//! exact to 1e-16 across n = 5..20; landing offset ≤ 9e-16) and matches
//! the Alhambra figures it reproduces; the battery in this module pins
//! it.
//!
//! **Merged figure.** The spur is collinear with the child leg-1 line
//! (M→G for rosettes, the whole star arm), so the emitted figure is
//! spurs ∪ post-knee child rays — emitting the child leg-1 segments
//! separately would hand the arrangement collinear overlaps, which
//! [`hankin::arrangement_faces`] rejects. A star's whole path is its
//! leg line, so extended stars emit spurs only, for any `s`.
//!
//! Motif circumradius is not a free parameter: the element is inscribed
//! in its tile, `r` = tile circumradius (the §3.5 figures all share it).
//! Angles are degrees in the public API. Tiles are regular, CCW.

use tess_core::geometry::Vec2;

use super::hankin;
use crate::lattice::Cell;

/// Geometric tolerance, matching the engine.
const TOL: f64 = 1e-9;

/// Per-tile design element (Kaplan diss. §3.5). `n` must equal the tile's
/// vertex count; `s ≥ 1` is the number of subpaths kept per arm.
#[derive(Debug, Clone, Copy)]
pub enum Motif {
    /// Star: arms are single straight segments from each contact.
    Star { n: usize, theta_deg: f64, s: usize },
    /// Rosette: arms bend at a knee G, then follow a ray.
    Rosette {
        n: usize,
        theta_deg: f64,
        s: usize,
        h: f64,
        phi_deg: f64,
    },
    /// Extension (§3.5.4) of a star at realized outer angle `theta_deg`.
    ExtendedStar { n: usize, theta_deg: f64, s: usize },
    /// Extension (§3.5.4) of a rosette at realized outer angle `theta_deg`.
    ExtendedRosette {
        n: usize,
        theta_deg: f64,
        s: usize,
        h: f64,
        phi_deg: f64,
    },
}

fn sub(a: Vec2, b: Vec2) -> Vec2 {
    a - b
}
fn cross(a: Vec2, b: Vec2) -> f64 {
    a.x * b.y - a.y * b.x
}
#[cfg(test)] // test-side geometry only; production paths use cross/length
fn dot(a: Vec2, b: Vec2) -> f64 {
    a.x * b.x + a.y * b.y
}
fn unit(v: Vec2) -> Vec2 {
    let l = v.length();
    debug_assert!(l > TOL, "degenerate direction");
    v * (1.0 / l)
}
fn rot(v: Vec2, ang: f64) -> Vec2 {
    let (c, s) = (ang.cos(), ang.sin());
    Vec2::new(v.x * c - v.y * s, v.x * s + v.y * c)
}

fn tile_center(tile: &[Vec2]) -> Vec2 {
    let n = tile.len() as f64;
    let mut o = Vec2::ZERO;
    for p in tile {
        o = o + *p;
    }
    o * (1.0 / n)
}

/// Motif circumradius `r`: the tile's own circumradius.
fn circumradius(tile: &[Vec2]) -> f64 {
    let o = tile_center(tile);
    tile.iter().map(|p| sub(*p, o).length()).fold(0.0, f64::max)
}

/// True if `q` is inside (or on) the convex CCW tile.
fn in_tile(q: Vec2, tile: &[Vec2]) -> bool {
    for i in 0..tile.len() {
        let pa = tile[i];
        let pb = tile[(i + 1) % tile.len()];
        if cross(pb - pa, q - pa) < -TOL {
            return false;
        }
    }
    true
}

/// Ray-owning arm of a d_n motif: fixed polyline `pts` (starting at the
/// contact M) plus the growth direction `d` after `pts`, and the
/// chirality `sig` of the leg at the contact.
struct Arm {
    sig: i32,
    m: Vec2,
    pts: Vec<Vec2>,
    d: Vec2,
}

/// Arms of the standalone motif on `tile` (both chiralities at every
/// contact). `th`, `ph` are radians; `rosette` selects the knee model.
fn make_arms(tile: &[Vec2], rosette: bool, th: f64, h: f64, ph: f64) -> Vec<Arm> {
    let n = tile.len();
    let o = tile_center(tile);
    let mut arms = Vec::with_capacity(2 * n);
    for i in 0..n {
        let a = tile[i];
        let b = tile[(i + 1) % n];
        let t = unit(b - a);
        let mid = (a + b) * 0.5;
        for sig in [1i32, -1] {
            let d1 = if sig > 0 { rot(t, th) } else { rot(t, std::f64::consts::PI - th) };
            let mut pts = vec![mid];
            let d;
            if rosette {
                let a_pt = if sig > 0 { b } else { a };
                let uom = unit(mid - o);
                let am = sub(a_pt, mid).length();
                let alpha = 1.0 - (std::f64::consts::PI / n as f64).sin();
                let want = h * alpha * am;
                let sgn = if cross(a_pt - mid, uom) > 0.0 { 1.0 } else { -1.0 };
                let den = cross(d1, uom);
                assert!(den.abs() > 1e-12, "leg1 parallel to OM");
                let u = sgn * want / den;
                assert!(u > 0.0, "G behind M");
                pts.push(mid + d1 * u);
                d = rot(uom * -1.0, sig as f64 * ph);
            } else {
                d = d1;
            }
            arms.push(Arm { sig, m: mid, pts, d });
        }
    }
    arms
}

/// First positive ray parameter and segment fraction of
/// `ray(p, dir) × segment(u, v)`; parallel or behind misses give `None`.
fn hit_seg(p: Vec2, dir: Vec2, u: Vec2, v: Vec2) -> Option<(f64, f64)> {
    let ab = v - u;
    let det = cross(dir, ab);
    if det.abs() < 1e-12 {
        return None;
    }
    let w = u - p;
    let t = cross(w, ab) / det;
    let s = cross(w, dir) / det;
    if t > 1e-7 && (-TOL..=1.0 + TOL).contains(&s) {
        Some((t, s.clamp(0.0, 1.0)))
    } else {
        None
    }
}

/// Ray parameter of the first boundary crossing of the ray `o + t d`
/// against the convex tile (`t > TOL`), if any.
fn boundary_exit(o: Vec2, d: Vec2, tile: &[Vec2]) -> Option<f64> {
    let mut best: Option<f64> = None;
    for i in 0..tile.len() {
        let pa = tile[i];
        let e = tile[(i + 1) % tile.len()] - pa;
        let det = cross(d, e);
        if det.abs() < 1e-13 {
            continue;
        }
        let w = pa - o;
        let t = cross(w, e) / det;
        let s = cross(w, d) / det;
        if t > TOL && (-TOL..=1.0 + TOL).contains(&s) {
            if best.is_none_or(|b| t < b) {
                best = Some(t);
            }
        }
    }
    best
}

/// Arc-lengths (from `pts[0]`) at which the arm path (polyline `pts`,
/// then the ray along `d`) crosses the drawn geometry `polys`, capped at
/// `arc_max` on the ray part and kept only inside the tile. Sorted,
/// deduplicated at 1e-9; the shared contact anchor is excluded.
fn crossings_along(
    pts: &[Vec2],
    d: Vec2,
    polys: &[(Vec2, Vec2)],
    arc_max: f64,
    tile: &[Vec2],
) -> (Vec<f64>, f64) {
    let mut out: Vec<f64> = Vec::new();
    let mut l = 0.0;
    for k in 0..pts.len() - 1 {
        let (p0, p1) = (pts[k], pts[k + 1]);
        let seg = p1 - p0;
        let seglen = seg.length();
        for &(u, v) in polys {
            if let Some((t, _)) = hit_seg(p0, seg, u, v) {
                let arc = l + t * seglen;
                if arc > 1e-7 && in_tile(p0 + seg * t, tile) {
                    out.push(arc);
                }
            }
        }
        l += seglen;
    }
    let g = pts[pts.len() - 1];
    for &(u, v) in polys {
        if let Some((t, _)) = hit_seg(g, d, u, v) {
            let arc = l + t;
            if arc <= arc_max + 1e-9 && in_tile(g + d * t, tile) {
                out.push(arc);
            }
        }
    }
    out.sort_by(f64::total_cmp);
    let mut ded: Vec<f64> = Vec::with_capacity(out.len());
    for x in out {
        if ded.last().is_none_or(|&p| (p - x).abs() >= 1e-9) {
            ded.push(x);
        }
    }
    (ded, l)
}

/// Truncate the arm path at arc-length `cut` (extending along `d`
/// beyond the fixed polyline).
fn trunc_path(pts: &[Vec2], d: Vec2, cut: f64) -> Vec<Vec2> {
    let mut acc = 0.0;
    let mut out = vec![pts[0]];
    for k in 0..pts.len() - 1 {
        let seglen = sub(pts[k + 1], pts[k]).length();
        if acc + seglen >= cut {
            let f = if seglen > 0.0 {
                ((cut - acc) / seglen).clamp(0.0, 1.0)
            } else {
                0.0
            };
            out.push(pts[k] + sub(pts[k + 1], pts[k]) * f);
            return out;
        }
        acc += seglen;
        out.push(pts[k + 1]);
    }
    out.push(pts[pts.len() - 1] + d * (cut - acc).max(0.0));
    out
}

/// Drawn geometry of an arm set: fixed polylines plus ray stubs of
/// length `stub` (2r: far enough past any in-tile crossing).
fn polys_of(arms: &[Arm], stub: f64) -> Vec<(Vec2, Vec2)> {
    let mut polys = Vec::new();
    for arm in arms {
        for j in 0..arm.pts.len() - 1 {
            polys.push((arm.pts[j], arm.pts[j + 1]));
        }
        let g = arm.pts[arm.pts.len() - 1];
        polys.push((g, g + arm.d * stub));
    }
    polys
}

/// Arc-length where subpath `s` of the arm ends (standalone semantics:
/// crossings with the full replicated motif, arc cap `2r`, never past
/// the tile boundary).
fn arm_cut(arm: &Arm, polys: &[(Vec2, Vec2)], s: usize, tile: &[Vec2], r: f64) -> f64 {
    let (crossings, _) = crossings_along(&arm.pts, arm.d, polys, 2.0 * r, tile);
    let tmax = boundary_exit(arm.pts[arm.pts.len() - 1], arm.d, tile);
    let mut cut = if crossings.len() >= s {
        crossings[s - 1]
    } else {
        tmax.unwrap_or(crossings[crossings.len() - 1])
    };
    if let Some(t) = tmax {
        cut = cut.min(t);
    }
    cut
}

/// Segments of a standalone d_n motif (star or rosette) on a regular tile.
fn standalone_segments(
    tile: &[Vec2],
    rosette: bool,
    theta_deg: f64,
    s: usize,
    h: f64,
    phi_deg: f64,
) -> Vec<(Vec2, Vec2)> {
    assert!(s >= 1, "motif subpath count s >= 1");
    let r = circumradius(tile);
    let arms = make_arms(tile, rosette, theta_deg.to_radians(), h, phi_deg.to_radians());
    let polys = polys_of(&arms, 2.0 * r);
    let mut segs = Vec::new();
    for arm in &arms {
        let cut = arm_cut(arm, &polys, s, tile, r);
        let path = trunc_path(&arm.pts, arm.d, cut.max(0.0));
        for k in 0..path.len() - 1 {
            if sub(path[k + 1], path[k]).length() > TOL {
                segs.push((path[k], path[k + 1]));
            }
        }
    }
    segs
}

/// Child-fit ratio `r′/r = cos(theta) / cos(π/n − theta)` (diss. §3.5.4
/// formula, landing reading — see module docs).
fn f_fit(n: usize, theta: f64) -> f64 {
    let p = std::f64::consts::PI / n as f64;
    theta.cos() / (p - theta).cos()
}

/// Segments of the extended design element (§3.5.4) drawn in the parent
/// tile: `theta_deg` is the realized OUTER contact angle at the parent
/// edges, `π/n < theta < π/2`. Merged figure: spurs ∪ post-knee child
/// rays (module docs).
fn extended_segments(
    tile: &[Vec2],
    rosette: bool,
    theta_deg: f64,
    s: usize,
    h: f64,
    phi_deg: f64,
) -> Vec<(Vec2, Vec2)> {
    use std::f64::consts::PI;
    assert!(s >= 1, "motif subpath count s >= 1");
    let n = tile.len();
    let p = PI / n as f64;
    let th = theta_deg.to_radians();
    assert!(
        p + 1e-9 < th && th < PI / 2.0 - 1e-9,
        "extension needs pi/n < theta < pi/2"
    );
    let thc = th - p;
    let o = tile_center(tile);
    let r = circumradius(tile);
    let rho = r * f_fit(n, th);
    let b = r * p.cos(); // parent contact-circle radius
    let child: Vec<Vec2> = tile
        .iter()
        .map(|&q| {
            let v = sub(q, o);
            let ang = v.y.atan2(v.x) + p;
            let len = v.length() * (rho / r);
            o + Vec2::new(len * ang.cos(), len * ang.sin())
        })
        .collect();
    let mids: Vec<Vec2> = (0..n)
        .map(|i| (tile[i] + tile[(i + 1) % n]) * 0.5)
        .collect();
    let arms = make_arms(&child, rosette, thc, h, phi_deg.to_radians());
    let mut segs = Vec::new();
    // Extension spurs: child contact -> landing parent midpoint on the
    // backward arm line (exact: landing radius r cos(π/n)).
    for arm in &arms {
        let mrel = sub(arm.m, o);
        let alpha = mrel.y.atan2(mrel.x);
        let beta = alpha - arm.sig as f64 * p;
        let dev = |m: &Vec2| {
            let v = sub(*m, o);
            ((v.y.atan2(v.x) - beta + PI).rem_euclid(2.0 * PI) - PI).abs()
        };
        let land = *mids
            .iter()
            .min_by(|m1, m2| dev(m1).total_cmp(&dev(m2)))
            .expect("tile has edges");
        let dleg = unit(sub(arm.m, land));
        let legdir = if arm.pts.len() > 1 {
            unit(sub(arm.pts[1], arm.m))
        } else {
            arm.d
        };
        assert!(
            cross(dleg, legdir).abs() < 1e-9,
            "landing not collinear with arm line"
        );
        assert!(
            (sub(land, o).length() - b).abs() < 1e-9,
            "landing off contact circle"
        );
        segs.push((arm.m, land));
    }
    // Post-knee child rays, standalone semantics at (s − 1) on the
    // child tile. (A star's whole arm is its leg line — already drawn
    // by the spur — so extended stars emit spurs only.)
    if rosette && s - 1 >= 1 {
        let polys = polys_of(&arms, 2.0 * rho);
        for arm in &arms {
            let cut = arm_cut(arm, &polys, s - 1, &child, rho);
            let leglen = sub(arm.pts[1], arm.m).length();
            if cut > leglen + TOL {
                let g = arm.pts[arm.pts.len() - 1];
                let end = g + arm.d * (cut - leglen);
                if sub(end, g).length() > TOL {
                    segs.push((g, end));
                }
            }
        }
    }
    segs
}

/// Segments of `motif` on a regular substrate `tile`, clipped to the
/// tile. Assertion: `n` matches the tile's vertex count.
pub fn motif_segments(tile: &[Vec2], motif: &Motif) -> Vec<(Vec2, Vec2)> {
    match *motif {
        Motif::Star { n, theta_deg, s } => {
            assert_eq!(n, tile.len(), "motif n must match the tile");
            standalone_segments(tile, false, theta_deg, s, 1.0, 0.0)
        }
        Motif::Rosette {
            n,
            theta_deg,
            s,
            h,
            phi_deg,
        } => {
            assert_eq!(n, tile.len(), "motif n must match the tile");
            standalone_segments(tile, true, theta_deg, s, h, phi_deg)
        }
        Motif::ExtendedStar { n, theta_deg, s } => {
            assert_eq!(n, tile.len(), "motif n must match the tile");
            extended_segments(tile, false, theta_deg, s, 1.0, 0.0)
        }
        Motif::ExtendedRosette {
            n,
            theta_deg,
            s,
            h,
            phi_deg,
        } => {
            assert_eq!(n, tile.len(), "motif n must match the tile");
            extended_segments(tile, true, theta_deg, s, h, phi_deg)
        }
    }
}

/// Per-tile motif assignment: build the lattice cells of a Hankin
/// substrate whose tiles carry (optionally) design elements. A tile
/// with `None` keeps its plain outline (one face).
pub fn build_cells_with_motifs(
    tiles: &[&[Vec2]],
    motifs: &[Option<Motif>],
) -> (Vec<Cell>, hankin::HankinReport) {
    let segs: Vec<Vec<(Vec2, Vec2)>> = tiles
        .iter()
        .zip(motifs)
        .map(|(tile, m)| {
            m.as_ref()
                .map_or_else(Vec::new, |m| motif_segments(tile, m))
        })
        .collect();
    hankin::build_cells_with_segments(tiles, &segs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn ngon(n: usize, r: f64, phase: f64) -> Vec<Vec2> {
        (0..n)
            .map(|k| {
                let a = phase + 2.0 * PI * k as f64 / n as f64;
                Vec2::new(r * a.cos(), r * a.sin())
            })
            .collect()
    }

    /// Tolerant multiset equality of two segment lists.
    fn segs_eq(a: &[(Vec2, Vec2)], b: &[(Vec2, Vec2)], tol: f64) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut used = vec![false; b.len()];
        'outer: for &(p, q) in a {
            for (idx, &(r, s)) in b.iter().enumerate() {
                if !used[idx]
                    && ((sub(r, p).length() < tol && sub(s, q).length() < tol)
                        || (sub(r, q).length() < tol && sub(s, p).length() < tol))
                {
                    used[idx] = true;
                    continue 'outer;
                }
            }
            return false;
        }
        true
    }

    #[test]
    fn rosette_ideal_knee_identities() {
        // Lee ideal-rosette pins at theta = pi/n, h = 1, phi = 0, on
        // every arm: with A the leg vertex and C the point on the
        // vertex radius at distance |A - M| from the vertex, the knee G
        // is equidistant from M and C; the knee height
        // |cross(G - M, u_OM)| equals (1 - sin(pi/n))|A - M|; the ray
        // leaves G exactly through C; and G is exactly line(M, M') x
        // line(C parallel OM) (Lee's construction). Python battery
        // (mpmath 40 dps) worst offset 1.7e-16; thresholds here 1e-13.
        for n in [5usize, 6, 8, 10, 12] {
            let tile = ngon(n, 1.0, 0.0);
            let p = PI / n as f64;
            let arms = make_arms(&tile, true, p, 1.0, 0.0);
            assert_eq!(arms.len(), 2 * n);
            let mids: Vec<Vec2> = (0..n)
                .map(|k| (tile[k] + tile[(k + 1) % n]) * 0.5)
                .collect();
            for (idx, arm) in arms.iter().enumerate() {
                let g = arm.pts[1];
                let i = idx / 2;
                let a_pt = if arm.sig > 0 {
                    tile[(i + 1) % n]
                } else {
                    tile[i]
                };
                let am = sub(a_pt, arm.m).length();
                let c = sub(a_pt, unit(a_pt) * am);
                let gc = sub(g, c).length();
                let gm = sub(g, arm.m).length();
                assert!((gc - gm).abs() < 1e-13, "n={n}: |GC| vs |GM|");
                let uom = unit(arm.m);
                let ht = cross(sub(g, arm.m), uom).abs();
                assert!(
                    (ht - (1.0 - p.sin()) * am).abs() < 1e-13,
                    "n={n}: knee height"
                );
                let ray_off = cross(arm.d, sub(c, g)).abs();
                assert!(ray_off < 1e-13, "n={n}: ray not through C");
                let m_pr = if arm.sig > 0 {
                    mids[(i + 1) % n]
                } else {
                    mids[(i + n - 1) % n]
                };
                let dmm = sub(m_pr, arm.m);
                let det = cross(dmm, uom);
                let u = cross(sub(c, arm.m), uom) / det;
                let g_lee = arm.m + dmm * u;
                assert!(
                    sub(g, g_lee).length() < 1e-13,
                    "n={n}: |G - G_lee| {}",
                    sub(g, g_lee).length()
                );
            }
        }
    }

    #[test]
    fn star_d_n_symmetry_and_contacts() {
        // Both chiralities: the emitted figure is d_n-symmetric about
        // the tile centre; every endpoint is a contact or shared.
        let tile = ngon(7, 0.9, 0.3);
        let segs = motif_segments(&tile, &Motif::Star { n: 7, theta_deg: 30.0, s: 1 });
        assert_eq!(segs.len(), 14);
        let ca = (2.0 * PI / 7.0).cos();
        let sa = (2.0 * PI / 7.0).sin();
        let rotf = |p: Vec2| Vec2::new(ca * p.x - sa * p.y, sa * p.x + ca * p.y);
        let img: Vec<(Vec2, Vec2)> = segs.iter().map(|&(p, q)| (rotf(p), rotf(q))).collect();
        assert!(segs_eq(&segs, &img, 5e-9), "d_7 rotation symmetry");
        let mids: Vec<Vec2> = (0..7)
            .map(|i| (tile[i] + tile[(i + 1) % 7]) * 0.5)
            .collect();
        for &(p, _q) in &segs {
            let mid = mids.iter().any(|m| sub(*m, p).length() < 1e-9);
            let shared = segs
                .iter()
                .filter(|&&(r, s)| sub(r, p).length() < 1e-9 || sub(s, p).length() < 1e-9)
                .count();
            assert!(mid || shared >= 2, "free interior endpoint");
        }
    }

    #[test]
    fn extension_landing_exact() {
        // Extension pins: spurs only for the star; landing radius
        // r cos(pi/n) (parent midpoints); child contacts at radius
        // r cos(pi/n) * F(n, theta); the spur line meets the parent
        // edge at the realized angle theta (|sin| equality). Python
        // battery worst offsets 8.9e-16; thresholds here 1e-10.
        for n in [5usize, 6, 8, 12] {
            for thd in [180.0 / n as f64 + 5.0, (180.0 / n as f64 + 90.0) / 2.0, 85.0] {
                let tile = ngon(n, 1.0, 0.0);
                let th = thd.to_radians();
                let segs = motif_segments(
                    &tile,
                    &Motif::ExtendedStar { n, theta_deg: thd, s: 1 },
                );
                assert_eq!(segs.len(), 2 * n, "n={n} th={thd}: spurs only");
                let b = (PI / n as f64).cos();
                let rp = b * f_fit(n, th);
                for (p, q) in &segs {
                    assert!((q.length() - b).abs() < 1e-12, "landing radius");
                    assert!((p.length() - rp).abs() < 1e-12, "contact radius");
                    let mut best: Option<(f64, Vec2)> = None;
                    for i in 0..n {
                        let a = tile[i];
                        let c = tile[(i + 1) % n];
                        let d = sub(c, a);
                        let u = dot(sub(*q, a), d) / dot(d, d);
                        let proj = a + d * u.clamp(0.0, 1.0);
                        let dist = sub(*q, proj).length();
                        if best.is_none_or(|(bd, _)| dist < bd) {
                            best = Some((dist, unit(d)));
                        }
                    }
                    let (_d, t) = best.unwrap();
                    let line = unit(sub(*q, *p));
                    let sint = cross(line, t).abs();
                    assert!(
                        (sint - th.sin()).abs() < 1e-10,
                        "n={n} th={thd}: realized sin {sint}"
                    );
                }
            }
        }
    }

    #[test]
    fn extension_rosette_merges_collinear_parts() {
        // The merged figure (spurs + post-knee rays) has no collinear
        // overlaps: the engine accepts it and partitions the tile exactly.
        let tile = ngon(6, 1.0, PI / 6.0);
        let segs = motif_segments(
            &tile,
            &Motif::ExtendedRosette { n: 6, theta_deg: 60.0, s: 2, h: 1.0, phi_deg: 0.0 },
        );
        assert_eq!(segs.len(), 12);
        let (cells, report) = hankin::build_cells_with_segments(&[&tile[..]], &[segs]);
        assert!(report.complete);
        assert_eq!(cells.len(), report.per_tile[0].faces);
        assert_eq!(cells.len(), 7); // 6 quads + central 12-gon (battery3)
        let tile_area = 3.0 * 3.0f64.sqrt() / 2.0; // hexagon R = 1
        assert!((report.per_tile[0].area_sum - tile_area).abs() < 1e-12);
    }

    #[test]
    fn motifs_partition_every_configured_tile() {
        // Face censuses of the shipped instance motif choices (Python
        // battery3 sweep over the engine port).
        let cases: Vec<(Vec<Vec2>, Motif, usize)> = vec![
            (
                ngon(8, 1.0, PI / 8.0),
                Motif::Rosette { n: 8, theta_deg: 22.5, s: 3, h: 1.0, phi_deg: 0.0 },
                9,
            ),
            (
                ngon(6, 1.0, PI / 6.0),
                Motif::Rosette { n: 6, theta_deg: 30.0, s: 2, h: 1.0, phi_deg: 0.0 },
                7,
            ),
            (
                ngon(4, 1.0, 0.0),
                Motif::ExtendedRosette { n: 4, theta_deg: 67.5, s: 2, h: 1.0, phi_deg: 0.0 },
                5,
            ),
            (
                ngon(6, 1.0, PI / 6.0),
                Motif::ExtendedStar { n: 6, theta_deg: 60.0, s: 1 },
                7,
            ),
            (
                ngon(12, 1.0, PI / 12.0),
                Motif::Rosette { n: 12, theta_deg: 15.0, s: 3, h: 1.0, phi_deg: 0.0 },
                13,
            ),
            (
                ngon(9, 1.0, PI / 9.0),
                Motif::Star { n: 9, theta_deg: 15.0, s: 1 },
                10,
            ),
        ];
        for (tile, motif, want_faces) in cases {
            let segs = motif_segments(&tile, &motif);
            let (cells, report) = hankin::build_cells_with_segments(&[&tile[..]], &[segs]);
            assert_eq!(cells.len(), want_faces, "{motif:?}");
            assert!((report.per_tile[0].area_sum - report.per_tile[0].tile_area).abs() < 1e-12);
        }
    }

    #[test]
    fn none_motif_keeps_plain_tile() {
        let tile = ngon(7, 0.8, 0.0);
        let (cells, report) = build_cells_with_motifs(&[&tile[..]], &[None]);
        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].seed, 0);
        assert!((report.per_tile[0].area_sum - report.per_tile[0].tile_area).abs() < 1e-12);
    }

    /// Extension spurs must land exactly on PARENT edge midpoints for
    /// tiles whose centre is NOT the origin — centre-relative
    /// arithmetic; an origin-relative landing test passes only for
    /// origin-centred tiles and silently fails elsewhere.
    #[test]
    fn extension_spurs_land_on_parent_midpoints_offset_tile() {
        for (o, motif, want_segs) in [
            (
                Vec2::new(1.5, 2.5),
                Motif::ExtendedStar { n: 6, theta_deg: 60.0, s: 1 },
                12,
            ),
            (
                Vec2::new(-2.25, 0.75),
                Motif::ExtendedRosette {
                    n: 4,
                    theta_deg: 67.5,
                    s: 2,
                    h: 1.0,
                    phi_deg: 0.0,
                },
                8,
            ),
        ] {
            let n = match motif {
                Motif::ExtendedStar { n, .. } | Motif::ExtendedRosette { n, .. } => n,
                _ => unreachable!(),
            };
            let tile: Vec<Vec2> = (0..n)
                .map(|k| {
                    let a = PI / n as f64 + 2.0 * PI * k as f64 / n as f64;
                    o + Vec2::new(a.cos(), a.sin())
                })
                .collect();
            let segs = motif_segments(&tile, &motif);
            assert_eq!(segs.len(), want_segs, "segment count at o = {o:?}");
            let mids: Vec<Vec2> = (0..n)
                .map(|i| (tile[i] + tile[(i + 1) % n]) * 0.5)
                .collect();
            for (a, b) in &segs {
                let parent_hit = [sub(*a, o), sub(*b, o)]
                    .iter()
                    .any(|p| mids.iter().any(|m| sub(*p, sub(*m, o)).length() < 1e-9));
                assert!(
                    parent_hit,
                    "spur {a:?} -> {b:?} misses parent midpoints (o = {o:?})"
                );
            }
        }
    }
}
