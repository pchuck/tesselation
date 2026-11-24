//! Hankin's method inference engine — Kaplan dissertation §3.4, f64 port.
//!
//! Kaplan's restatement of Hankin's construction for star patterns:
//! from every substrate-edge midpoint ("contact") two rays depart into
//! the tile, symmetric about the inward normal, each making the contact
//! angle θK with the edge line. Every ray is grown until it meets
//! another ray; pairs are chosen greedily in order of total induced
//! segment length (|a−q| + |b−q|); same-contact rays never pair (their
//! crossing is shared); colinear head-on rays join into one straight
//! segment of cost |a−b|. The paired segments, arranged against the tile
//! boundary, partition the tile into the motif faces.
//!
//! The TWO-POINT variant (§3.4, Figure 3.5) splits each contact position
//! along the edge by ±δ/2, with the rays leaving the split points so
//! that same-contact rays cross (the "lean backward" rule: the ray from
//! p+(δ/2)t heads toward −t). Neighbour tiles compute the same split
//! points on the shared edge, so motifs line up for any global δ.
//!
//! ## Faithfulness of the f64 port
//!
//! Every configuration shipped through this engine was validated in a
//! 40-dps mpmath prototype (thresholds 1e-25) that also measured the
//! margins which make a 1e-9-tolerance f64 port exact-to-1e-12:
//!
//! * minimum accepted-candidate ray parameter 3.5e-2, minimum
//!   arrangement piece length 2.5e-1, minimum half-edge angular gap
//!   7.9e-1, minimum face area 6.3e-2, minimum canonical sort-key gap
//!   1.8e-1 — each ≥ 7 orders of magnitude above [`TOL`];
//! * the greedy outcome is *order-invariant*: the mpmath candidate tie
//!   groups (exact equal costs — the constructions are highly symmetric)
//!   may be permuted arbitrarily (reversed, 8 seeded random
//!   permutations) without changing the paired segment set, and distinct
//!   cost classes are separated by ≥ 1.3e-1 ≫ f64 rounding noise. So a
//!   deterministic f64 greedy (costs quantized to 1e-9 buckets, then
//!   candidate indices) provably reproduces the validated pairing.
//!
//! Ray starts, meeting points and split points are snapped (|v| < 1e-12
//! → 0) before canonicalization, identically to the prototype's
//! transcription step, so canonical face keys agree bit-for-bit modulo
//! ~1e-15 arithmetic noise.

use tess_core::geometry::{Polygon, Vec2};

use crate::lattice::Cell;

/// Geometric tolerance (see the module doc: validated margins ≥ 3.5e-2).
const TOL: f64 = 1e-9;

/// Cost-quantum for greedy ordering: mathematically equal costs (exact
/// ties in the mpmath prototype) fall into one bucket; distinct cost
/// classes are ≥ 1.3e-1 apart, so bucket boundaries never split them.
const COST_QUANTUM: f64 = 1e-9;

#[derive(Clone, Copy)]
struct Ray {
    /// Edge index; two rays of one contact never pair.
    contact: usize,
    start: Vec2,
    dir: Vec2,
}

/// Per-tile engine report (used by tests to pin engine invariants).
pub struct TileReport {
    pub rays: usize,
    pub pairs: usize,
    pub complete: bool,
    pub faces: usize,
    pub area_sum: f64,
    pub tile_area: f64,
}

/// Whole-cell engine report.
pub struct HankinReport {
    pub complete: bool,
    pub per_tile: Vec<TileReport>,
}

fn unit(v: Vec2) -> Vec2 {
    let l = v.length();
    debug_assert!(l > TOL, "degenerate direction");
    v / l
}

fn shoelace(pts: &[Vec2]) -> f64 {
    let n = pts.len();
    let mut a = 0.0;
    for i in 0..n {
        let p = pts[i];
        let q = pts[(i + 1) % n];
        a += p.x * q.y - q.x * p.y;
    }
    a * 0.5
}

/// Snap arithmetic dust (|v| < 1e-12 → 0), same rule as the prototype's
/// transcription step.
fn snap(v: Vec2) -> Vec2 {
    Vec2::new(
        if v.x.abs() < 1e-12 { 0.0 } else { v.x },
        if v.y.abs() < 1e-12 { 0.0 } else { v.y },
    )
}

/// Canonical face: snapped, CCW, rotated to the lexicographically
/// smallest vertex first. Identical to the prototype's canonicalization;
/// the min-vertex selection uses a 1e-12 comparison tolerance so that
/// mirror-symmetric coordinates that differ only by float rounding
/// (≤ a few ulp between the mpmath and f64 engines) resolve to the same
/// rotation on both sides — distinct vertices are ≥ 1e-9 apart by the
/// arrangement merge, so the tolerance never merges real candidates.
fn canon_face(mut pts: Vec<Vec2>) -> Vec<Vec2> {
    for p in pts.iter_mut() {
        *p = snap(*p);
    }
    if shoelace(&pts) < 0.0 {
        pts.reverse();
    }
    let mut k = 0;
    for i in 1..pts.len() {
        let dx = pts[i].x - pts[k].x;
        let dy = pts[i].y - pts[k].y;
        if dx < -1e-12 || (dx.abs() <= 1e-12 && dy < -1e-12) {
            k = i;
        }
    }
    pts.rotate_left(k);
    pts
}

/// Tolerant scalar comparison: |a − b| ≤ 1e-12 counts as equal. Mirror
/// ties in the canonical keys therefore resolve identically in the
/// engine and in the emit pipeline (which uses the same rule), while
/// genuinely distinct coordinates are ≥ 1e-9 apart.
fn cmp_tol(a: f64, b: f64) -> std::cmp::Ordering {
    let d = a - b;
    if d.abs() <= 1e-12 {
        std::cmp::Ordering::Equal
    } else if d < 0.0 {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Greater
    }
}

fn face_cmp(a: &[Vec2], b: &[Vec2]) -> std::cmp::Ordering {
    a.len().cmp(&b.len()).then_with(|| {
        for (p, q) in a.iter().zip(b.iter()) {
            match cmp_tol(p.x, q.x) {
                std::cmp::Ordering::Equal => {}
                other => return other,
            }
            match cmp_tol(p.y, q.y) {
                std::cmp::Ordering::Equal => {}
                other => return other,
            }
        }
        std::cmp::Ordering::Equal
    })
}

/// 2n rays for an n-gon: per CCW edge, contact = midpoint, t = unit edge
/// direction, n = inward normal (left of t). Split points p ∓ (δ/2)t;
/// the ray from p−(δ/2)t leans toward +t, the ray from p+(δ/2)t toward
/// −t, so same-contact rays cross (dissertation Figure 3.5).
fn rays_for_tile(tile: &[Vec2], theta_e_deg: f64, delta: f64) -> Vec<Ray> {
    let rad = theta_e_deg.to_radians();
    let (st, ct) = rad.sin_cos();
    let mut out = Vec::with_capacity(2 * tile.len());
    for i in 0..tile.len() {
        let a = tile[i];
        let b = tile[(i + 1) % tile.len()];
        let t = unit(b - a);
        let n = Vec2::new(-t.y, t.x);
        let p = (a + b) / 2.0;
        let off = t * (delta / 2.0);
        out.push(Ray {
            contact: i,
            start: p - off,
            dir: Vec2::new(st * t.x + ct * n.x, st * t.y + ct * n.y),
        });
        out.push(Ray {
            contact: i,
            start: p + off,
            dir: Vec2::new(-st * t.x + ct * n.x, -st * t.y + ct * n.y),
        });
    }
    out
}

/// Forward intersection of two rays, q inside-or-on the tile.
fn ray_pair_intersection(r1: &Ray, r2: &Ray, tile: &[Vec2]) -> Option<(Vec2, f64, f64)> {
    let (a, d1) = (r1.start, r1.dir);
    let (b, d2) = (r2.start, r2.dir);
    let det = d1.cross(d2);
    if det.abs() < TOL {
        return None; // parallel: no single meeting point
    }
    let ab = b - a;
    let t1 = ab.cross(d2) / det;
    let t2 = ab.cross(d1) / det;
    if t1 < -TOL || t2 < -TOL {
        return None;
    }
    let q = a + d1 * t1;
    for i in 0..tile.len() {
        let pa = tile[i];
        let pb = tile[(i + 1) % tile.len()];
        if (pb - pa).cross(q - pa) < -TOL {
            return None; // outside the tile
        }
    }
    Some((q, t1, t2))
}

/// Greedy min-cost pairing (Kaplan §3.4). Returns the paired motif
/// segments, the pair count, and whether every ray was used.
fn infer_segments(
    tile: &[Vec2],
    theta_e_deg: f64,
    delta: f64,
) -> (Vec<(Vec2, Vec2)>, usize, bool) {
    let rays = rays_for_tile(tile, theta_e_deg, delta);
    let n = rays.len();

    struct Cand {
        cost: f64,
        i: usize,
        j: usize,
        q: Option<Vec2>,
    }
    let mut cands: Vec<Cand> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            if rays[i].contact == rays[j].contact {
                continue; // same contact: never paired (shared crossing)
            }
            let (a, d1) = (rays[i].start, rays[i].dir);
            let (b, d2) = (rays[j].start, rays[j].dir);
            // colinear head-on: one straight segment a->b, cost |a-b|
            if d1.cross(d2).abs() < TOL && d1.cross(b - a).abs() < TOL {
                if d1.dot(b - a) > TOL && d2.dot(a - b) > TOL {
                    cands.push(Cand { cost: (b - a).length(), i, j, q: None });
                }
                continue;
            }
            let Some((q, t1, t2)) = ray_pair_intersection(&rays[i], &rays[j], tile) else {
                continue;
            };
            if t1 < TOL || t2 < TOL {
                continue; // crossing at a ray start: zero-length edge
            }
            let cost = (q - a).length() + (q - b).length();
            cands.push(Cand { cost, i, j, q: Some(q) });
        }
    }
    // Deterministic greedy order: cost bucket (mathematical ties share a
    // bucket — order-invariance of the tie groups is proven in the
    // prototype), then candidate indices.
    cands.sort_by_key(|c| {
        (
            (c.cost / COST_QUANTUM).round() as i64,
            c.i,
            c.j,
        )
    });
    let mut used = vec![false; n];
    let mut pairs: Vec<(usize, usize, Option<Vec2>)> = Vec::new();
    for c in &cands {
        if used[c.i] || used[c.j] {
            continue;
        }
        used[c.i] = true;
        used[c.j] = true;
        pairs.push((c.i, c.j, c.q));
    }
    let mut segs = Vec::new();
    for (i, j, q) in &pairs {
        match q {
            None => segs.push((rays[*i].start, rays[*j].start)),
            Some(q) => {
                for r in [&rays[*i], &rays[*j]] {
                    if (*q - r.start).length() > TOL {
                        segs.push((r.start, *q));
                    }
                }
            }
        }
    }
    let complete = used.iter().all(|u| *u);
    (segs, pairs.len(), complete)
}

/// Intersection parameters (t on c1, u on c2) of two segments, including
/// endpoint touches; collinear overlaps give the overlap endpoints.
fn seg_params(c1: &(Vec2, Vec2), c2: &(Vec2, Vec2)) -> Vec<(f64, f64)> {
    let (a, b) = *c1;
    let (c, d) = *c2;
    let ab = b - a;
    let cd = d - c;
    let denom = ab.cross(cd);
    let ac = c - a;
    if denom.abs() < TOL {
        if ab.cross(ac).abs() > TOL {
            return Vec::new(); // parallel, not collinear
        }
        let l2 = ab.dot(ab);
        if l2 < TOL * TOL {
            return Vec::new();
        }
        let t1 = ac.dot(ab) / l2;
        let t2 = (d - a).dot(ab) / l2;
        let (lo, hi) = if t1 <= t2 { (t1, t2) } else { (t2, t1) };
        let lo = lo.max(0.0);
        let hi = hi.min(1.0);
        if lo > hi + TOL {
            return Vec::new();
        }
        let mut out = Vec::new();
        for t in [lo, hi] {
            if (-TOL..=1.0 + TOL).contains(&t) {
                let p = a + ab * t;
                let u = if cd.dot(cd) > TOL * TOL {
                    (p - c).dot(cd) / cd.dot(cd)
                } else {
                    0.0
                };
                out.push((t.clamp(0.0, 1.0), u.clamp(0.0, 1.0)));
            }
        }
        return out;
    }
    let t = ac.cross(cd) / denom;
    let u = ac.cross(ab) / denom;
    if (-TOL..=1.0 + TOL).contains(&t) && (-TOL..=1.0 + TOL).contains(&u) {
        vec![(t.clamp(0.0, 1.0), u.clamp(0.0, 1.0))]
    } else {
        Vec::new()
    }
}

/// Planar arrangement of the paired motif segments against the tile
/// boundary: curves are split at every crossing (two-point motifs
/// interlace and cross inside the tile), then the bounded faces are
/// walked from the half-edge structure.
fn arrangement_faces(tile: &[Vec2], segs: &[(Vec2, Vec2)]) -> Vec<Vec<Vec2>> {
    let mut curves: Vec<(Vec2, Vec2)> = Vec::with_capacity(segs.len() + tile.len());
    curves.extend(segs.iter().copied());
    for i in 0..tile.len() {
        curves.push((tile[i], tile[(i + 1) % tile.len()]));
    }
    let n = curves.len();
    let mut params: Vec<Vec<f64>> = vec![Vec::new(); n];
    for i in 0..n {
        for j in (i + 1)..n {
            for (t, u) in seg_params(&curves[i], &curves[j]) {
                params[i].push(t);
                params[j].push(u);
            }
        }
    }

    let mut pts_all: Vec<Vec2> = Vec::new();
    let mut pieces: Vec<(usize, usize)> = Vec::new();
    for (i, &(a, b)) in curves.iter().enumerate() {
        let mut ts: Vec<f64> = params[i]
            .iter()
            .copied()
            .filter(|t| *t > TOL && *t < 1.0 - TOL)
            .collect();
        ts.sort_by(f64::total_cmp);
        ts.dedup_by(|x, y| (*x - *y).abs() < TOL);
        let mut chain = vec![a];
        for t in &ts {
            chain.push(a + (b - a) * *t);
        }
        chain.push(b);
        for w in chain.windows(2) {
            let (p, q) = (w[0], w[1]);
            if (q - p).length() < TOL {
                continue;
            }
            let ai = add_pt(p, &mut pts_all);
            let bi = add_pt(q, &mut pts_all);
            pieces.push((ai, bi));
        }
    }
    let mut seen = std::collections::HashSet::new();
    let mut edges: Vec<(usize, usize)> = Vec::new();
    for (ai, bi) in pieces {
        if ai == bi {
            continue;
        }
        let key = (ai.min(bi), ai.max(bi));
        if seen.insert(key) {
            edges.push(key);
        }
    }
    // half-edge structure
    let mut hedges: Vec<(usize, usize)> = Vec::with_capacity(2 * edges.len());
    let mut out_h: Vec<Vec<(f64, usize)>> = vec![Vec::new(); pts_all.len()];
    for &(ai, bi) in &edges {
        let hid = hedges.len();
        hedges.push((ai, bi));
        hedges.push((bi, ai));
        let a = pts_all[ai];
        let b = pts_all[bi];
        out_h[ai].push((f64::atan2(b.y - a.y, b.x - a.x), hid));
        out_h[bi].push((f64::atan2(a.y - b.y, a.x - b.x), hid + 1));
    }
    for (v, lst) in out_h.iter_mut().enumerate() {
        lst.sort_by(|x, y| x.0.total_cmp(&y.0));
        for w in lst.windows(2) {
            if w[1].0 - w[0].0 < TOL {
                panic!("hankin: overlapping half-edges at {:?}", pts_all[v]);
            }
        }
    }
    // next half-edge: clockwise-most from the reverse direction (bounds
    // the faces to the left of their half-edges)
    let mut nxt = vec![0usize; hedges.len()];
    for (hid, &(from, to)) in hedges.iter().enumerate() {
        let rev_dir = f64::atan2(
            pts_all[from].y - pts_all[to].y,
            pts_all[from].x - pts_all[to].x,
        );
        let mut best: Option<(f64, usize)> = None;
        for &(ang, oid) in &out_h[to] {
            let mut d = rev_dir - ang;
            if d <= TOL {
                d += 2.0 * std::f64::consts::PI;
            }
            if best.is_none_or(|(bd, _)| d < bd) {
                best = Some((d, oid));
            }
        }
        nxt[hid] = best.expect("vertex with no outgoing half-edge").1;
    }
    let mut vis = vec![false; hedges.len()];
    let mut faces = Vec::new();
    for h0 in 0..hedges.len() {
        if vis[h0] {
            continue;
        }
        let mut cycle = Vec::new();
        let mut h = h0;
        while !vis[h] {
            vis[h] = true;
            cycle.push(h);
            h = nxt[h];
        }
        let pts_f: Vec<Vec2> = cycle.iter().map(|&hh| pts_all[hedges[hh].0]).collect();
        if shoelace(&pts_f) > TOL {
            faces.push(pts_f);
        }
    }
    faces
}

fn add_pt(p: Vec2, pts_all: &mut Vec<Vec2>) -> usize {
    for (k, q) in pts_all.iter().enumerate() {
        if (q.x - p.x).abs() < TOL && (q.y - p.y).abs() < TOL {
            return k;
        }
    }
    pts_all.push(p);
    pts_all.len() - 1
}

/// Motif inference + arrangement for one substrate tile: canonical faces
/// (snapped, CCW, min-vertex first, sorted) plus the engine report.
fn tile_faces(tile: &[Vec2], theta_e_deg: f64, delta: f64) -> (Vec<Vec<Vec2>>, TileReport) {
    let (segs, pairs, complete) = infer_segments(tile, theta_e_deg, delta);
    let faces_raw = arrangement_faces(tile, &segs);
    let mut faces: Vec<Vec<Vec2>> = faces_raw
        .into_iter()
        .map(canon_face)
        .collect();
    faces.sort_by(|a, b| face_cmp(a, b));
    let area_sum: f64 = faces.iter().map(|f| shoelace(f).abs()).sum();
    let n_faces = faces.len();
    (
        faces,
        TileReport {
            rays: 2 * tile.len(),
            pairs,
            complete,
            faces: n_faces,
            area_sum,
            tile_area: shoelace(tile).abs(),
        },
    )
}

/// Shared tail of [`build_cells`] and [`build_cells_with_segments`]:
/// assign pattern seeds by congruence clustering (vertex count + area
/// within 1e-9, first-appearance order) and assert the exact partition
/// of every tile by its faces.
fn assemble_cells(
    all_faces: Vec<Vec<Vec<Vec2>>>,
    per_tile: Vec<TileReport>,
) -> (Vec<Cell>, HankinReport) {
    let mut seen: Vec<(usize, f64)> = Vec::new();
    let mut cells = Vec::new();
    for faces in &all_faces {
        for f in faces {
            let area = shoelace(f).abs();
            let seed = seen
                .iter()
                .position(|(l, ar)| *l == f.len() && (ar - area).abs() < 1e-9)
                .unwrap_or_else(|| {
                    seen.push((f.len(), area));
                    seen.len() - 1
                });
            cells.push(Cell {
                seed: seed as u32,
                poly: Polygon::from_slice(f),
            });
        }
    }
    let complete = per_tile.iter().all(|r| r.complete);
    for r in &per_tile {
        assert!(
            (r.area_sum - r.tile_area).abs() < 1e-9,
            "hankin: face partition inexact (Σ {} vs tile {})",
            r.area_sum,
            r.tile_area
        );
    }
    (
        cells,
        HankinReport {
            complete,
            per_tile,
        },
    )
}

/// Build the lattice cells of one fundamental domain: run the engine on
/// every substrate tile (in-cell positions), canonicalize and sort the
/// faces, and assign pattern seeds by congruence clustering (vertex
/// count + area within 1e-9, first-appearance order).
pub fn build_cells(
    tiles: &[&[Vec2]],
    theta_e_deg: f64,
    delta: f64,
) -> (Vec<Cell>, HankinReport) {
    let mut all_faces: Vec<Vec<Vec<Vec2>>> = Vec::with_capacity(tiles.len());
    let mut per_tile = Vec::with_capacity(tiles.len());
    for tile in tiles {
        let (faces, rep) = tile_faces(tile, theta_e_deg, delta);
        all_faces.push(faces);
        per_tile.push(rep);
    }
    assemble_cells(all_faces, per_tile)
}

/// Faces of one tile under designer-supplied motif segments: the same
/// arrangement, snapping and canonicalization as [`tile_faces`], minus
/// the ray inference.
fn tile_faces_with_segments(
    tile: &[Vec2],
    segs: &[(Vec2, Vec2)],
) -> (Vec<Vec<Vec2>>, TileReport) {
    let faces_raw = arrangement_faces(tile, segs);
    let mut faces: Vec<Vec<Vec2>> = faces_raw
        .into_iter()
        .map(canon_face)
        .collect();
    faces.sort_by(|a, b| face_cmp(a, b));
    let area_sum: f64 = faces.iter().map(|f| shoelace(f).abs()).sum();
    let n_faces = faces.len();
    (
        faces,
        TileReport {
            rays: segs.len(),
            pairs: 0,
            complete: true,
            faces: n_faces,
            area_sum,
            tile_area: shoelace(tile).abs(),
        },
    )
}

/// [`build_cells`] with the motif segments supplied per tile instead of
/// inferred from `(theta_e_deg, delta)` — the entry point for the design
/// elements of [`super::hankin_motifs`] (Kaplan diss. §3.5). Arrangement,
/// canonicalization, clustering and the exact-partition assertion are the
/// shared, unchanged engine pipeline. In the report `rays` counts the
/// supplied segments, `pairs` is 0 and `complete` is trivially true.
///
/// A tile with an empty segment list contributes one face: the tile
/// itself (substrate tiles carrying no motif).
pub fn build_cells_with_segments(
    tiles: &[&[Vec2]],
    segs_per_tile: &[Vec<(Vec2, Vec2)>],
) -> (Vec<Cell>, HankinReport) {
    assert_eq!(
        tiles.len(),
        segs_per_tile.len(),
        "hankin: one segment list per substrate tile"
    );
    let mut all_faces: Vec<Vec<Vec<Vec2>>> = Vec::with_capacity(tiles.len());
    let mut per_tile = Vec::with_capacity(tiles.len());
    for (tile, segs) in tiles.iter().zip(segs_per_tile) {
        let (faces, rep) = tile_faces_with_segments(tile, segs);
        all_faces.push(faces);
        per_tile.push(rep);
    }
    assemble_cells(all_faces, per_tile)
}
