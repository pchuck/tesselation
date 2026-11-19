//! Rosette transform — the Najm ↔ Hankin bridge (Kaplan dissertation §3.8).
//!
//! The rosette transform is an operation on Euclidean tilings: for each
//! distinct tile shape a planar map is constructed, and the maps are
//! assembled into a new *template* tiling. When the template is then
//! finished by Hankin's inference method (§3.4), the result is the
//! "Hankin tiling" corresponding to the Najm tiling of the input — the
//! construction behind Kaplan's Figure 3.32 (simpler Najm tiling turned
//! into a more complex Hankin tiling) and Figure 3.33 (rosette
//! transform of 4.8.8, then inference).
//!
//! ## Per-tile rules
//!
//! 1. **Regular n-gon, n ≥ 5** (Figure 3.30): a smaller regular n-gon
//!    P′ is placed concentrically with the tile, rotated by π/n relative
//!    to it, with radius r′ chosen so that each spoke segment (a
//!    vertex of P′ to the midpoint of the corresponding outer edge) has
//!    length exactly half of P′'s side length. The trigonometry gives
//!    (dissertation form)
//!    ```text
//!    r′ = r (cos(π/n) − sin(π/n)·tan(π(n−2)/(4n)))
//!       ≡ r·cos(π/n) / (1 + sin(π/n))
//!    ```
//!    The map is P′ together with the spokes; its faces are the inner
//!    n-gon plus n corner pentagons.
//!
//! 2. **Every other polygon** (Figure 3.31): the perpendicular
//!    bisectors of the sides are extended as rays toward the tile's
//!    interior and truncated where they meet each other — in spirit the
//!    inference algorithm at contact angle 90°. Kaplan's heuristic is
//!    applied here: all forward pairwise ray intersections inside the
//!    tile are clustered (tolerance below), each cluster is averaged
//!    down to a single point, and every ray contributing to a cluster
//!    uses that point as its endpoint (a ray that contributes to more
//!    than one cluster uses the closest one). Rays that meet no other
//!    ray inside the tile are extended to the tile boundary, which keeps
//!    the map closed (the "same pitfalls" Kaplan warns about).
//!
//! **The regular square** (n = 4) falls to rule 2: the n ≥ 5 formula is
//! not expected to give a meaningful map for quadrilaterals, and
//! indeed all four perpendicular bisectors pass through the centre, so
//! the 6 pairwise intersections collapse into a single cluster whose
//! average *is* the centre. The map is the four small squares of side
//! s/2 (for the 4.8.8 substrate: side 1/2) — the bisector rule handles
//! the n < 5 case exactly.
//!
//! ## Coordinate conventions
//!
//! The transform is local to each tile: it reuses the substrate's cell
//! coordinates verbatim (inputs are CCW polygons in cell space, in-cell
//! positions — the same convention as [`crate::hankin`]'s input), and
//! the lattice basis is carried through unchanged. It enters the
//! [`rosette_transform`] API only so the report can carry |det| for the
//! area-census invariant (Σ face areas per lattice cell = |det|).
//!
//! ## Helper duplication
//!
//! The canonicalization and planar-arrangement helpers below
//! (`snap`, `shoelace`, `canon_face`, `cmp_tol`, `face_cmp`,
//! `seg_params`, `add_pt`, `arrangement_faces`) are copies of the
//! private helpers in [`crate::hankin`], which this module must not
//! modify (read-only constraint); the two copies must be kept in sync.

use tess_core::geometry::{Polygon, Vec2};

use crate::lattice::Cell;

/// Geometric tolerance, at the scale of [`crate::hankin`]'s TOL: every
/// registered lattice tiling's cell-space coordinates are O(1)–O(3).
const TOL: f64 = 1e-9;

/// Minimum forward ray parameter for a bisector to count as meeting
/// another ray (rejects zero-length segments at the ray starts).
const RAY_TOL: f64 = TOL;

/// Which rule produced a tile's map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    /// Inner regular n-gon + n corner pentagons (Figure 3.30), n ≥ 5.
    RegularPolygon,
    /// Truncated perpendicular bisectors (Figure 3.31).
    Bisectors,
}

/// Per-tile transform report.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TileReport {
    /// Which rule was applied.
    pub rule: Rule,
    /// Vertex count of the input tile.
    pub n: usize,
    /// Number of output faces.
    pub faces: usize,
    /// Σ face areas (closure against `tile_area`).
    pub area_sum: f64,
    /// Input tile area.
    pub tile_area: f64,
    /// [`Rule::Bisectors`]: forward pairwise ray intersections found
    /// inside the tile (before clustering).
    pub intersections: usize,
    /// [`Rule::Bisectors`]: number of distinct cluster endpoints.
    pub clusters: usize,
    /// [`Rule::Bisectors`]: whether Kaplan's averaging heuristic was
    /// exercised, i.e. some cluster merged more than one intersection
    /// point (the regular square merges all six into the centre).
    pub clustered: bool,
}

/// Whole-table transform report.
#[derive(Debug, Clone)]
pub struct TransformReport {
    pub per_tile: Vec<TileReport>,
    /// |det(lattice)| of the substrate lattice — the area-census target
    /// for the sum of all face areas over one lattice cell.
    pub lattice_area: f64,
}

/// Dissertated form: `r′ = r (cos(π/n) − sin(π/n) tan(π(n−2)/(4n)))`.
pub fn inner_radius(n: usize, r: f64) -> f64 {
    let pi_n = std::f64::consts::PI / n as f64;
    r * (pi_n.cos() - pi_n.sin() * (std::f64::consts::PI * (n - 2) as f64 / (4.0 * n as f64)).tan())
}

/// Equivalent closed form: `r′ = r·cos(π/n) / (1 + sin(π/n))`.
pub fn inner_radius_closed(n: usize, r: f64) -> f64 {
    let pi_n = std::f64::consts::PI / n as f64;
    r * pi_n.cos() / (1.0 + pi_n.sin())
}

/// Transform one substrate tile (CCW polygon, cell space) into the face
/// polygons of its rosette map: canonical (snapped, CCW, min-vertex
/// first) and sorted, so identical input shapes give identical output.
pub fn transform_tile(tile: &[Vec2]) -> (Vec<Vec<Vec2>>, TileReport) {
    let n = tile.len();
    debug_assert!(n >= 3, "degenerate substrate tile");
    match regular_geometry(tile) {
        Some((c, r)) if n >= 5 => regular_map(tile, n, c, r),
        _ => bisector_map(tile, n),
    }
}

/// Transform a full substrate cell table (in-cell positions, the same
/// convention as [`crate::hankin::build_cells`]' input). Returns the
/// transformed cell table — one output cell per map face, carrying its
/// parent cell's seed — and the report. `lattice` is the substrate's
/// basis; the transform itself is local per tile and reuses the
/// substrate's cell coordinates, so the basis only feeds
/// [`TransformReport::lattice_area`].
pub fn rosette_transform(cells: &[Cell], lattice: [Vec2; 2]) -> (Vec<Cell>, TransformReport) {
    let det = lattice[0].cross(lattice[1]);
    debug_assert!(det.abs() >= 1e-12, "degenerate substrate lattice");
    let mut out: Vec<Cell> = Vec::new();
    let mut per_tile: Vec<TileReport> = Vec::new();
    for cell in cells {
        let (faces, rep) = transform_tile(&cell.poly.pts);
        per_tile.push(rep);
        for f in faces {
            out.push(Cell {
                poly: Polygon::from_slice(&f),
                seed: cell.seed,
            });
        }
    }
    (
        out,
        TransformReport {
            per_tile,
            lattice_area: det.abs(),
        },
    )
}

// ---------------------------------------------------------------------
// Rule 1: regular n-gon, n ≥ 5 (Figure 3.30).
// ---------------------------------------------------------------------

/// Detect a regular polygon at `1e-9` relative tolerance: equal vertex
/// radii about the centroid, equal edge lengths, strictly convex, CCW.
/// Returns (centroid, circumradius). All substrate tiles in this repo
/// are convex, which the convex-interior tests below also assume.
fn regular_geometry(tile: &[Vec2]) -> Option<(Vec2, f64)> {
    let n = tile.len();
    if n < 3 {
        return None;
    }
    if shoelace(tile) <= TOL {
        return None; // must be CCW
    }
    let mut acc = Vec2::ZERO;
    for &v in tile {
        acc += v;
    }
    let c = acc / n as f64;
    let mut r = 0.0;
    for &v in tile {
        r += (v - c).length();
    }
    r /= n as f64;
    if r < TOL {
        return None;
    }
    for &v in tile {
        if ((v - c).length() - r).abs() > 1e-9 * r {
            return None;
        }
    }
    let mut e = 0.0;
    for i in 0..n {
        e += (tile[(i + 1) % n] - tile[i]).length();
    }
    e /= n as f64;
    if e < TOL {
        return None;
    }
    for i in 0..n {
        if ((tile[(i + 1) % n] - tile[i]).length() - e).abs() > 1e-9 * e {
            return None;
        }
    }
    for i in 0..n {
        let a = tile[(i + n - 1) % n];
        let b = tile[i];
        let d = tile[(i + 1) % n];
        if (b - a).cross(d - b) <= TOL {
            return None; // must be strictly convex
        }
    }
    Some((c, r))
}

/// Inner n-gon + n corner pentagons.
fn regular_map(tile: &[Vec2], n: usize, c: Vec2, r: f64) -> (Vec<Vec<Vec2>>, TileReport) {
    let rp = inner_radius(n, r);
    // w_i: inner vertex in the direction of edge i's midpoint m_i
    // (the π/n rotation of the figure), m_i: outer edge midpoint.
    let mut w: Vec<Vec2> = Vec::with_capacity(n);
    let mut m: Vec<Vec2> = Vec::with_capacity(n);
    for i in 0..n {
        let mid = (tile[i] + tile[(i + 1) % n]) / 2.0;
        m.push(mid);
        w.push(c + (mid - c) / (mid - c).length() * rp);
    }
    let mut faces: Vec<Vec<Vec2>> = Vec::with_capacity(n + 1);
    // Inner regular n-gon (CCW: edge midpoints are CCW around c).
    faces.push(w.clone());
    // Corner pentagon for outer edge i: the face owning vertex v_{i+1},
    // bounded by spokes w_i–m_i and w_{i+1}–m_{i+1}, the two half
    // outer edges to v_{i+1}, and the inner edge w_i–w_{i+1}.
    for i in 0..n {
        let j = (i + 1) % n;
        faces.push(vec![w[i], m[i], tile[j], m[j], w[j]]);
    }
    let mut out: Vec<Vec<Vec2>> = faces.into_iter().map(canon_face).collect();
    out.sort_by(|a, b| face_cmp(a, b));
    let area_sum = out.iter().map(|f| shoelace(f).abs()).sum();
    let n_faces = out.len();
    (
        out,
        TileReport {
            rule: Rule::RegularPolygon,
            n,
            faces: n_faces,
            area_sum,
            tile_area: shoelace(tile).abs(),
            intersections: 0,
            clusters: 0,
            clustered: false,
        },
    )
}

// ---------------------------------------------------------------------
// Rule 2: perpendicular bisectors (Figure 3.31) + Kaplan's clustering.
// ---------------------------------------------------------------------

fn bisector_map(tile: &[Vec2], n: usize) -> (Vec<Vec<Vec2>>, TileReport) {
    // CCW orientation so the left normal is the inward normal.
    let mut pts: Vec<Vec2> = tile.to_vec();
    if shoelace(&pts) < 0.0 {
        pts.reverse();
    }
    // Ray per edge: from the midpoint along the inward normal.
    let mut rays: Vec<(Vec2, Vec2)> = Vec::with_capacity(n);
    for i in 0..n {
        let a = pts[i];
        let b = pts[(i + 1) % n];
        let t = unit(b - a);
        rays.push(((a + b) / 2.0, Vec2::new(-t.y, t.x)));
    }
    // All forward pairwise intersections inside the tile.
    let diag = pts
        .iter()
        .map(|&p| (p - pts[0]).length())
        .fold(0.0, f64::max);
    let cluster_tol = 1e-9 * diag.max(1.0);
    let mut inter: Vec<(usize, usize, Vec2)> = Vec::new(); // (i, j, q)
    for i in 0..n {
        for j in (i + 1)..n {
            let (a, d1) = rays[i];
            let (b, d2) = rays[j];
            let det = d1.cross(d2);
            if det.abs() < TOL {
                continue; // parallel bisectors never meet
            }
            let ab = b - a;
            let s = ab.cross(d2) / det;
            let u = ab.cross(d1) / det;
            if s < RAY_TOL || u < RAY_TOL {
                continue;
            }
            let q = a + d1 * s;
            if !inside_convex(&pts, q) {
                continue;
            }
            inter.push((i, j, q));
        }
    }
    // Cluster the intersection points (Kaplan's averaging heuristic).
    let k = inter.len();
    let mut parent: Vec<usize> = (0..k).collect();
    for a in 0..k {
        for b in (a + 1)..k {
            if (inter[a].2 - inter[b].2).length() <= cluster_tol {
                union(&mut parent, a, b);
            }
        }
    }
    // (root, members) per cluster, first-appearance order.
    let mut clusters: Vec<(usize, Vec<usize>)> = Vec::new();
    for x in 0..k {
        let r = find(&parent, x);
        match clusters.iter_mut().find(|(root, _)| *root == r) {
            Some((_, members)) => members.push(x),
            None => clusters.push((r, vec![x])),
        }
    }
    // Cluster representative: the average of its member points.
    let mut rep: Vec<Vec2> = Vec::with_capacity(clusters.len());
    for (_, members) in &clusters {
        let mut acc = Vec2::ZERO;
        for &x in members {
            acc += inter[x].2;
        }
        rep.push(acc / members.len() as f64);
    }
    // Ray i's endpoint: the representative of the closest cluster it
    // contributes to (the first meeting point along the ray). Both
    // parameters are recomputed per pair.
    let mut endpoints: Vec<Option<usize>> = vec![None; n];
    let mut best_param: Vec<f64> = vec![f64::MAX; n];
    for (idx, (i, j, _)) in inter.iter().enumerate() {
        let (a, d1) = rays[*i];
        let (b, d2) = rays[*j];
        let det = d1.cross(d2);
        let ab = b - a;
        let s_i = ab.cross(d2) / det;
        let s_j = ab.cross(d1) / det;
        let r = find(&parent, idx);
        let ci = clusters.iter().position(|(root, _)| *root == r).unwrap();
        for (ray, s) in [(*i, s_i), (*j, s_j)] {
            if s < best_param[ray] {
                best_param[ray] = s;
                endpoints[ray] = Some(ci);
            }
        }
    }
    // Rays that met nobody inside: extend to the boundary (fallback).
    for i in 0..n {
        if endpoints[i].is_none() {
            let (a, d) = rays[i];
            let t = boundary_exit(&pts, a, d).unwrap_or_else(|| {
                panic!(
                    "rosette transform: bisector of edge {i} of an {n}-gon \
                     met no ray inside the tile and has no boundary exit"
                )
            });
            endpoints[i] = Some(rep.len()); // sentinel: boundary point
            best_param[i] = t;
            rep.push(a + d * t);
        }
    }
    let segs: Vec<(Vec2, Vec2)> = (0..n)
        .map(|i| (rays[i].0, rep[endpoints[i].unwrap()]))
        .collect();
    let mut faces: Vec<Vec<Vec2>> =
        arrangement_faces(&pts, &segs).into_iter().map(canon_face).collect();
    faces.sort_by(|a, b| face_cmp(a, b));
    let area_sum = faces.iter().map(|f| shoelace(f).abs()).sum();
    let clustered = clusters.iter().any(|(_, members)| members.len() > 1);
    let n_faces = faces.len();
    (
        faces,
        TileReport {
            rule: Rule::Bisectors,
            n,
            faces: n_faces,
            area_sum,
            tile_area: shoelace(&pts).abs(),
            intersections: k,
            clusters: clusters.len(),
            clustered,
        },
    )
}

/// Union-find root (no compression: `parent` is shared read-only here;
/// the trees are shallow — at most one union per pair).
fn find(parent: &[usize], mut x: usize) -> usize {
    while parent[x] != x {
        x = parent[x];
    }
    x
}

/// Union by attaching root(a) under root(b).
fn union(parent: &mut [usize], a: usize, b: usize) {
    let (ra, rb) = (find(parent, a), find(parent, b));
    if ra != rb {
        parent[ra] = rb;
    }
}

/// Whether `q` is inside the convex polygon (boundary counts as inside).
fn inside_convex(poly: &[Vec2], q: Vec2) -> bool {
    for i in 0..poly.len() {
        let pa = poly[i];
        let pb = poly[(i + 1) % poly.len()];
        if (pb - pa).cross(q - pa) < -TOL {
            return false;
        }
    }
    true
}

/// First exit parameter of ray `a + t·d` (t > 0) through the convex
/// polygon boundary.
fn boundary_exit(poly: &[Vec2], a: Vec2, d: Vec2) -> Option<f64> {
    let mut best = f64::MAX;
    for i in 0..poly.len() {
        let p = poly[i];
        let e = poly[(i + 1) % poly.len()] - p;
        let den = d.cross(e);
        if den.abs() < TOL {
            continue;
        }
        let t = (p - a).cross(e) / den;
        let s = (p - a).cross(d) / den;
        if t > RAY_TOL && (-TOL..=1.0 + TOL).contains(&s) && t < best {
            best = t;
        }
    }
    if best == f64::MAX {
        None
    } else {
        Some(best)
    }
}

// ---------------------------------------------------------------------
// Duplicated engine helpers (see the module doc).
// ---------------------------------------------------------------------

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
/// smallest vertex first.
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

/// Tolerant scalar comparison: |a − b| ≤ 1e-12 counts as equal.
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

/// Planar arrangement of the bisector segments against the tile
/// boundary, walked through the half-edge structure (copy of the
/// engine's arrangement).
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
                panic!("rosette transform: overlapping half-edges at {:?}", pts_all[v]);
            }
        }
    }
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
