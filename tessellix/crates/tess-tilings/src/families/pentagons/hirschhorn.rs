//! Hirschhorn (Hirschhorn–Hunt 1985) nonperiodic monohedral tiling.
//!
//! The prototile is the equilateral convex pentagon with interior angles
//! A = 140°, B = 60°, C = 160°, D = 80°, E = 100° (unit edges) — the
//! n = 6 member of the C11-T1A pentagon family (Sugimoto,
//! arXiv:2005.08470), originally constructed by Hirschhorn and Hunt
//! (JCTA 39, 1985). Pentagon census (M101): the template is a type-1
//! pentagon (T1, repo lettering ccw k=1); the §5 type-8 attribution
//! is falsified by the M101 audit.
//!
//! The tiling is not locally forced, so the generator builds one valid
//! realization by constrained 6-fold-symmetric growth:
//!
//! 1. seed with the "flower": six petals related by 60° rotations, all of
//!    their 60° (B) vertices at the origin;
//! 2. at each step, enumerate every placement of a unit pentagon across
//!    every directed boundary edge — both chiralities × 5 template edges ×
//!    2 directions — keeping the ones that pass all local rules:
//!    V1  the tile lies on the exterior side of the boundary edge;
//!    V2  no interior overlap with any existing tile (convex tests);
//!    V3  edge-to-edge: no T-junctions, no partial edge overlaps;
//!    V4  the angle sum at every vertex stays at most 360° and no residual gap of 20°/40° is left behind (no tile angle sums to one, so such a gap can never be closed);
//!    V5  the tile is not a duplicate (centroid test).
//!    A full-unit-edge overlap is legal only when the two tiles sit on
//!    opposite sides of it — that is what permits the two-edge "corner
//!    bridges" that must close the 300° (A + C) gaps at the flower's
//!    corners.
//! 3. group the surviving placements into complete 6-fold rotation orbits.
//!    The pool is restricted to vertex-closing placements (falling back to
//!    all valid placements only when none close a vertex — committing
//!    non-closing orbits eagerly stretches the boundary into long fjords
//!    that can never be closed), and the first orbit in boundary
//!    enumeration order that stays valid tile by tile, keeps the patch a
//!    topological disk, and strands no dead boundary edge is committed —
//!    an edge with no legal placement left can never be filled, since
//!    growth only adds constraints.
//!
//! Every grown patch is C6-symmetric about the seed origin. Both
//! chiralities occur (mirror tiles appear sparsely, as in the paper's
//! figures) and all tile frames land on multiples of 20°.
//!
//! Generation: grow until the patch boundary provably contains the
//! requested region (all region samples lie inside the boundary cycle and
//! no boundary edge crosses the region), then cull to the region. The
//! grown patch is cached and continued across calls, so repeated queries
//! (the UIs regenerate every frame) only grow what the new view
//! additionally needs.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::Mutex;

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::TilingCategory;
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

const TOL: f64 = 1e-9;
/// Snap used to identify points (computation paths of the same
/// mathematical point drift by ~1e-16; distinct tiling points are far more
/// than 1e-3 apart, so 1e-9 snapping is safe).
const KEY_SCALE: f64 = 1e9;
/// Uniform grid cell size for the spatial hash.
const GRID_CELL: f64 = 0.5;
const TAU: f64 = std::f64::consts::PI * 2.0;
const ANG60: f64 = std::f64::consts::PI / 3.0;
/// Interior angles of the pentagon, in units of 20°: 140, 60, 160, 80, 100.
const ANG20: [u8; 5] = [7, 3, 8, 4, 5];
const MAX_GROW_STEPS: usize = 100_000;

// ---------------------------------------------------------------------------
// Geometry helpers (convex, orientation-agnostic)
// ---------------------------------------------------------------------------

fn signed_area2(pts: &[Vec2; 5]) -> f64 {
    let mut s = 0.0;
    for i in 0..5 {
        let a = pts[i];
        let b = pts[(i + 1) % 5];
        s += a.x * b.y - b.x * a.y;
    }
    s
}

fn bbox_of(pts: &[Vec2; 5]) -> BBox {
    let mut lo = pts[0];
    let mut hi = pts[0];
    for &p in &pts[1..] {
        lo.x = lo.x.min(p.x);
        lo.y = lo.y.min(p.y);
        hi.x = hi.x.max(p.x);
        hi.y = hi.y.max(p.y);
    }
    BBox::new(lo, hi)
}

fn centroid_of(pts: &[Vec2; 5]) -> Vec2 {
    let mut c = Vec2::ZERO;
    for &p in pts {
        c += p;
    }
    c * 0.2
}

/// Strict interior test for a convex pentagon (boundary counts as outside).
fn point_in_poly_strict(p: Vec2, pts: &[Vec2; 5]) -> bool {
    let mut sign: i32 = 0;
    for i in 0..5 {
        let a = pts[i];
        let b = pts[(i + 1) % 5];
        let c = (b - a).cross(p - a);
        if c.abs() < 1e-12 {
            return false;
        }
        let s = if c > 0.0 { 1 } else { -1 };
        if sign == 0 {
            sign = s
        } else if s != sign {
            return false;
        }
    }
    true
}

/// Whether the open segments (a,b) and (c,d) intersect at an interior point.
fn segs_properly_cross(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> bool {
    let d1 = (d - c).cross(a - c);
    let d2 = (d - c).cross(b - c);
    let d3 = (b - a).cross(c - a);
    let d4 = (b - a).cross(d - a);
    ((d1 > 1e-12 && d2 < -1e-12) || (d1 < -1e-12 && d2 > 1e-12))
        && ((d3 > 1e-12 && d4 < -1e-12) || (d3 < -1e-12 && d4 > 1e-12))
}

/// Length of the collinear overlap of [a, b] and [c, d], else 0.
fn seg_overlap(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> f64 {
    let ab = b - a;
    let l2 = ab.dot(ab);
    if l2 < 1e-18 {
        return 0.0;
    }
    let l = l2.sqrt();
    if (c - a).cross(ab).abs() > TOL * l || (d - a).cross(ab).abs() > TOL * l {
        return 0.0;
    }
    let proj = |w: Vec2| (w - a).dot(ab) / l2;
    let t0 = proj(c);
    let t1 = proj(d);
    let lo = 0f64.max(t0.min(t1));
    let hi = 1.0f64.min(t0.max(t1));
    (hi - lo).max(0.0) * l
}

/// `p` strictly between `a` and `b`, collinear with them (T-junction test).
fn point_on_seg_open(p: Vec2, a: Vec2, b: Vec2) -> bool {
    let ab = b - a;
    let l = ab.length();
    if (p - a).cross(ab).abs() > TOL * l.max(1.0) {
        return false;
    }
    let t = (p - a).dot(ab) / ab.dot(ab);
    t > 1e-8 && t < 1.0 - 1e-8
}

// ---------------------------------------------------------------------------
// Point / edge identity
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PKey(i64, i64);

impl PKey {
    fn of(p: Vec2) -> Self {
        PKey((p.x * KEY_SCALE).round() as i64, (p.y * KEY_SCALE).round() as i64)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct EKey(PKey, PKey);

impl EKey {
    fn of(a: Vec2, b: Vec2) -> Self {
        let (x, y) = (PKey::of(a), PKey::of(b));
        if x <= y {
            EKey(x, y)
        } else {
            EKey(y, x)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GKey(i64, i64);

/// The vertex-set signature of a tile (for geometric dedup / orbit lookup).
type Sign = BTreeSet<PKey>;

fn sign_of(pts: &[Vec2; 5]) -> Sign {
    let mut s = BTreeSet::new();
    for &p in pts {
        s.insert(PKey::of(p));
    }
    s
}

// ---------------------------------------------------------------------------
// The pentagon template
// ---------------------------------------------------------------------------

fn dir(deg: f64) -> Vec2 {
    let r = deg.to_radians();
    Vec2::new(r.cos(), r.sin())
}

/// Canonical CCW pentagon, unit edges. Interior angles at V[0..5]:
/// 140 (A), 60 (B), 160 (C), 80 (D), 100 (E).
fn template_ccw() -> [Vec2; 5] {
    let mut v = [Vec2::ZERO; 5];
    v[1] = Vec2::new(1.0, 0.0);
    v[2] = Vec2::new(0.5, (3.0f64).sqrt() / 2.0);
    v[3] = v[2] + dir(140.0);
    v[4] = v[3] + dir(240.0);
    v
}

/// The 6-petal seed flower: petal j is the template rotated by
/// (60j − 180)° about its B vertex, so every petal's B vertex sits at the
/// origin.
fn seed_flower(tmpl: &[Vec2; 5]) -> [[Vec2; 5]; 6] {
    let mut out = [*tmpl; 6];
    for (j, petal) in out.iter_mut().enumerate() {
        let ang = (j as f64 * 60.0 - 180.0).to_radians();
        for p in petal.iter_mut() {
            *p = (*p - tmpl[1]).rotated(ang);
        }
    }
    out
}

// ---------------------------------------------------------------------------
// A single tile and the growing patch
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct Tile5 {
    pts: [Vec2; 5],
}

impl Tile5 {
    fn centroid(&self) -> Vec2 {
        centroid_of(&self.pts)
    }
    fn ccw(&self) -> bool {
        signed_area2(&self.pts) > 0.0
    }
}

/// A growing patch of tiles with the bookkeeping the local validity rules
/// need: a spatial hash (grid), per-vertex angle sums (in 20° units) and
/// per-edge ownership counts.
struct Patch {
    tiles: Vec<Tile5>,
    bbs: Vec<BBox>,
    grid: HashMap<GKey, Vec<usize>>,
    angle: HashMap<PKey, u8>,
    edge_count: HashMap<EKey, u8>,
    bb_all: BBox,
    tmpl_ccw: [Vec2; 5],
    tmpl_cw: [Vec2; 5],
}

impl Patch {
    fn new() -> Self {
        let ccw = template_ccw();
        let cw = ccw.map(|p| Vec2::new(p.x, -p.y));
        let mut patch = Self {
            tiles: Vec::with_capacity(4096),
            bbs: Vec::with_capacity(4096),
            grid: HashMap::new(),
            angle: HashMap::new(),
            edge_count: HashMap::new(),
            bb_all: BBox::new(Vec2::new(f64::MAX, f64::MAX), Vec2::new(f64::MIN, f64::MIN)),
            tmpl_ccw: ccw,
            tmpl_cw: cw,
        };
        for petal in seed_flower(&ccw) {
            patch.add_tile(petal);
        }
        patch
    }

    /// Commit a tile (seed petals are valid by construction; grown tiles
    /// are validated before this is called).
    fn add_tile(&mut self, pts: [Vec2; 5]) {
        let i = self.tiles.len();
        let bb = bbox_of(&pts);
        self.tiles.push(Tile5 { pts });
        self.bbs.push(bb);
        self.bb_all = self.bb_all.union(bb);
        let x0 = (bb.min.x / GRID_CELL).floor() as i64;
        let x1 = (bb.max.x / GRID_CELL).floor() as i64;
        let y0 = (bb.min.y / GRID_CELL).floor() as i64;
        let y1 = (bb.max.y / GRID_CELL).floor() as i64;
        for gx in x0..=x1 {
            for gy in y0..=y1 {
                self.grid.entry(GKey(gx, gy)).or_default().push(i);
            }
        }
        for (j, &p) in pts.iter().enumerate() {
            let e = self.angle.entry(PKey::of(p)).or_insert(0);
            *e += ANG20[j];
            debug_assert!(*e <= 18);
        }
        for e in 0..5 {
            *self.edge_count.entry(EKey::of(pts[e], pts[(e + 1) % 5])).or_insert(0) += 1;
        }
    }

    /// Indices of committed tiles whose bbox intersects `bb`.
    fn near(&self, bb: &BBox) -> Vec<usize> {
        let x0 = (bb.min.x / GRID_CELL).floor() as i64;
        let x1 = (bb.max.x / GRID_CELL).floor() as i64;
        let y0 = (bb.min.y / GRID_CELL).floor() as i64;
        let y1 = (bb.max.y / GRID_CELL).floor() as i64;
        let mut out = Vec::new();
        for gx in x0..=x1 {
            for gy in y0..=y1 {
                if let Some(v) = self.grid.get(&GKey(gx, gy)) {
                    out.extend_from_slice(v);
                }
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }

    /// Do an existing tile `t` and a candidate placement `cand` conflict
    /// (V2 overlap, V3 T-junction / partial overlap, V5 duplicate)?
    fn tile_conflicts(t: &Tile5, t_bb: &BBox, cand: &[Vec2; 5], cand_bb: &BBox, cen: Vec2) -> bool {
        if !cand_bb.intersects(*t_bb) {
            return false;
        }
        if point_in_poly_strict(cen, &t.pts) {
            return true;
        }
        for &v in cand {
            if point_in_poly_strict(v, &t.pts) {
                return true;
            }
        }
        for &v in &t.pts {
            if point_in_poly_strict(v, cand) {
                return true;
            }
        }
        for i in 0..5 {
            let na = cand[i];
            let nb = cand[(i + 1) % 5];
            for j in 0..5 {
                let oa = t.pts[j];
                let ob = t.pts[(j + 1) % 5];
                if segs_properly_cross(na, nb, oa, ob) {
                    return true;
                }
                let ov = seg_overlap(na, nb, oa, ob);
                if ov > 1e-7 {
                    if (ov - 1.0).abs() > 1e-7 {
                        return true;
                    }
                    // Full-unit-edge overlap: legitimate only on opposite
                    // sides (an attachment); same side is an overlap. This
                    // is what allows the two-edge "corner bridge" tiles.
                    let tc = t.centroid();
                    let s_new = (nb - na).cross(cen - na);
                    let s_old = (nb - na).cross(tc - na);
                    if s_new * s_old > 0.0 {
                        return true;
                    }
                }
            }
        }
        for i in 0..5 {
            let na = cand[i];
            let nb = cand[(i + 1) % 5];
            for &v in &t.pts {
                if point_on_seg_open(v, na, nb) {
                    return true;
                }
            }
            for j in 0..5 {
                let oa = t.pts[j];
                let ob = t.pts[(j + 1) % 5];
                for &v in cand {
                    if point_on_seg_open(v, oa, ob) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Angle sum (in 20° units) contributed by `extra` at point `k`.
    fn extra_angle(extra: &[Tile5], k: PKey) -> u32 {
        let mut s = 0;
        for t in extra {
            for (j, &w) in t.pts.iter().enumerate() {
                if PKey::of(w) == k {
                    s += ANG20[j] as u32;
                }
            }
        }
        s
    }

    /// Full local validity of `cand` placed across the directed boundary
    /// edge (a, b) — patch interior on the left of a→b — on top of the
    /// committed patch plus the not-yet-committed tiles `extra`.
    fn valid(&self, extra: &[Tile5], extra_bb: &[BBox], cand: &[Vec2; 5], a: Vec2, b: Vec2) -> bool {
        let cen = centroid_of(cand);
        // V1: exterior side of the primary boundary edge.
        if (b - a).cross(cen - a) > -TOL {
            return false;
        }
        let cand_bb = bbox_of(cand);
        for &ti in &self.near(&cand_bb) {
            if Self::tile_conflicts(&self.tiles[ti], &self.bbs[ti], cand, &cand_bb, cen) {
                return false;
            }
        }
        for (ei, t) in extra.iter().enumerate() {
            if Self::tile_conflicts(t, &extra_bb[ei], cand, &cand_bb, cen) {
                return false;
            }
        }
        // V4: at every vertex the angle sum stays at most 360° (= 18
        // units) and any residual gap must be fillable. Tile angles are
        // {3, 4, 5, 7, 8} units; no subset sums to 1 or 2, so leaving a
        // 20° or 40° gap at a vertex can never be closed and kills the
        // whole branch (a local dead end).
        for (j, &v) in cand.iter().enumerate() {
            let k = PKey::of(v);
            let s = self.angle.get(&k).copied().unwrap_or(0) as u32 + Self::extra_angle(extra, k);
            let sum = s + ANG20[j] as u32;
            if sum > 18 || (sum < 18 && 18 - sum <= 2) {
                return false;
            }
        }
        true
    }

    /// Does placing `cand` bring some vertex to exactly 360°?
    fn closes_vertex(&self, extra: &[Tile5], cand: &[Vec2; 5]) -> bool {
        for (j, &v) in cand.iter().enumerate() {
            let k = PKey::of(v);
            let s = self.angle.get(&k).copied().unwrap_or(0) as u32 + Self::extra_angle(extra, k);
            if s + ANG20[j] as u32 == 18 {
                return true;
            }
        }
        false
    }

    /// Directed boundary edges (a, b): owned by exactly one tile, oriented
    /// with the patch interior on the left. Deterministic order (tile
    /// insertion order × edge index).
    fn boundary(&self) -> Vec<(Vec2, Vec2)> {
        let mut out = Vec::new();
        for t in &self.tiles {
            let ccw = t.ccw();
            for e in 0..5 {
                let a = t.pts[e];
                let b = t.pts[(e + 1) % 5];
                if self.edge_count.get(&EKey::of(a, b)).copied() == Some(1) {
                    out.push(if ccw { (a, b) } else { (b, a) });
                }
            }
        }
        out
    }

    /// Euler characteristic χ = V − E + F of the current patch. A disk
    /// (simply connected, no holes) has χ = 1; each enclosed unfilled
    /// pocket lowers χ by one.
    #[cfg(test)]
    fn euler(&self) -> i64 {
        self.angle.len() as i64 - self.edge_count.len() as i64 + self.tiles.len() as i64
    }

    /// Δχ if `orbit_pts` were placed in order. Placing pentagons on the
    /// boundary of a disk keeps χ constant (no new pocket) ⟺ Δχ = 0.
    fn orbit_euler_delta(&self, orbit_pts: &[[Vec2; 5]]) -> i64 {
        let mut new_v = 0i64;
        let mut new_e = 0i64;
        let mut o_v: HashMap<PKey, ()> = HashMap::new();
        let mut o_e: HashMap<EKey, ()> = HashMap::new();
        for pts in orbit_pts {
            for &p in pts {
                let k = PKey::of(p);
                if !self.angle.contains_key(&k) && o_v.insert(k, ()).is_none() {
                    new_v += 1;
                }
            }
            for i in 0..5 {
                let k = EKey::of(pts[i], pts[(i + 1) % 5]);
                if !self.edge_count.contains_key(&k) && o_e.insert(k, ()).is_none() {
                    new_e += 1;
                }
            }
        }
        new_v - new_e + orbit_pts.len() as i64
    }

    /// True when at least one tile can be legally placed across the
    /// boundary edge (a, b) — interior on the left of a→b — on top of
    /// `self` plus the extra tiles. (a, b) is used as the fixed V1
    /// reference edge, exactly as in [`Patch::grow_step`].
    fn has_valid_placement(&self, a: Vec2, b: Vec2, extra: &[Tile5], extra_bb: &[BBox]) -> bool {
        let ka = PKey::of(a);
        let kb = PKey::of(b);
        let sa = self.angle.get(&ka).copied().unwrap_or(0) as i32
            + Self::extra_angle(extra, ka) as i32;
        let sb = self.angle.get(&kb).copied().unwrap_or(0) as i32
            + Self::extra_angle(extra, kb) as i32;
        for &(r1, r2) in &[(a, b), (b, a)] {
            let (s1, s2) = if PKey::of(r1) == ka { (sa, sb) } else { (sb, sa) };
            for tmpl in [&self.tmpl_ccw, &self.tmpl_cw] {
                for k in 0..5 {
                    // Angle pre-filter: template vertex k lands on r1 and
                    // vertex (k + 1) on r2. A residual gap of 1–2 units
                    // (20°/40°) is unfillable — no tile has such an angle —
                    // so such a placement can never close the vertex.
                    let g1 = 18 - s1 - ANG20[k] as i32;
                    let g2 = 18 - s2 - ANG20[(k + 1) % 5] as i32;
                    if g1 < 0 || g1 == 1 || g1 == 2 || g2 < 0 || g2 == 1 || g2 == 2 {
                        continue;
                    }
                    let pts = place(tmpl, k, r1, r2);
                    if self.valid(extra, extra_bb, &pts, a, b) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// True when adding the candidate tiles `extra` (already checked valid
    /// and non-conflicting on top of `self`) would create a dead boundary
    /// edge: an edge with no legal placement left. Placements only become
    /// more constrained as the patch grows, so a dead edge can never be
    /// filled and the patch can never become a complete tiling. Only edges
    /// near the new tiles can change state, so each tile is checked against
    /// its own local influence zone.
    fn kills_boundary(&self, extra: &[Tile5], extra_bb: &[BBox]) -> bool {
        let n = extra.len();
        // How many candidate tiles use each undirected edge, so interior
        // edges (shared within the batch) can be told from new boundary
        // edges (used by exactly one candidate, not shared with the patch).
        let mut extra_edge: HashMap<EKey, usize> = HashMap::new();
        for t in extra {
            for j in 0..5 {
                let k = EKey::of(t.pts[j], t.pts[(j + 1) % 5]);
                *extra_edge.entry(k).or_insert(0) += 1;
            }
        }
        let boundary = self.boundary();
        for i in 0..n {
            let t = &extra[i];
            let bb = extra_bb[i];
            let lo = Vec2::new(bb.min.x - 3.0, bb.min.y - 3.0);
            let hi = Vec2::new(bb.max.x + 3.0, bb.max.y + 3.0);
            let in_zone =
                |p: Vec2| p.x >= lo.x && p.x <= hi.x && p.y >= lo.y && p.y <= hi.y;
            // New boundary edges contributed by this tile. Orient each
            // reference edge with the patch interior on the left: for a
            // CCW tile the vertex order already does that; for a CW tile
            // the edge must be reversed.
            let interior_left = t.ccw();
            for j in 0..5 {
                let u = t.pts[j];
                let v = t.pts[(j + 1) % 5];
                let ek = EKey::of(u, v);
                if extra_edge[&ek] != 1 || self.edge_count.contains_key(&ek) {
                    continue; // interior within the batch or shared with the patch
                }
                let (ra, rb) = if interior_left { (u, v) } else { (v, u) };
                if !self.has_valid_placement(ra, rb, extra, extra_bb) {
                    return true;
                }
            }
            // Old boundary edges in this tile's zone that survive the batch.
            for &(ua, ub) in &boundary {
                if !in_zone(ua) && !in_zone(ub) {
                    continue;
                }
                if extra_edge.contains_key(&EKey::of(ua, ub)) {
                    continue; // becomes interior
                }
                if !self.has_valid_placement(ua, ub, extra, extra_bb) {
                    return true;
                }
            }
        }
        false
    }

    /// One growth step: place the first viable complete 6-fold orbit of
    /// valid placements (closing orbits preferred), in deterministic
    /// boundary order. Orbits that would enclose a pocket (Δχ ≠ 0) are
    /// rejected, keeping the patch a topological disk. False when no orbit
    /// can be placed.
    fn grow_step(&mut self) -> bool {
        let boundary = self.boundary();
        // Enumerate candidates in deterministic order.
        let mut pool: Vec<([Vec2; 5], Vec2, Vec2)> = Vec::new();
        let mut seen: HashSet<Sign> = HashSet::new();
        for &(a, b) in &boundary {
            for tmpl in [&self.tmpl_ccw, &self.tmpl_cw] {
                for k in 0..5 {
                    for &(r1, r2) in &[(a, b), (b, a)] {
                        let pts = place(tmpl, k, r1, r2);
                        if !self.valid(&[], &[], &pts, a, b) {
                            continue;
                        }
                        if !seen.insert(sign_of(&pts)) {
                            continue;
                        }
                        pool.push((pts, a, b));
                    }
                }
            }
        }
        if pool.is_empty() {
            return false;
        }
        // Which candidates form the pool: the closing placements (they
        // complete 360° vertices and drive the tiling forward); when none
        // exists, fall back to all valid placements. Committing
        // non-closing orbits too eagerly stretches the boundary into long
        // fjords that the tiling can never close, so they are a last
        // resort only.
        let closing: Vec<usize> = (0..pool.len())
            .filter(|&i| self.closes_vertex(&[], &pool[i].0))
            .collect();
        let pool_idx: Vec<usize> = if closing.is_empty() {
            (0..pool.len()).collect()
        } else {
            closing
        };
        // Group the pool into 6-fold rotation orbits (about the seed
        // origin).
        let mut by_sig: BTreeMap<Sign, usize> = BTreeMap::new();
        for (oi, &pi) in pool_idx.iter().enumerate() {
            by_sig.insert(sign_of(&pool[pi].0), oi);
        }
        let mut used = vec![false; pool_idx.len()];
        let mut orbits: Vec<Vec<usize>> = Vec::new();
        for i in 0..pool_idx.len() {
            if used[i] {
                continue;
            }
            let mut orbit = vec![i];
            used[i] = true;
            let mut pts = rot60_arr(&pool[pool_idx[i]].0);
            for _ in 1..6 {
                let s = sign_of(&pts);
                match by_sig.get(&s) {
                    Some(&j) if !used[j] => {
                        used[j] = true;
                        orbit.push(j);
                        pts = rot60_arr(&pool[pool_idx[j]].0);
                    }
                    _ => break,
                }
            }
            // A complete 6-cycle is the only committable orbit shape.
            // Under the C6 invariants the candidate pool is closed under
            // 60° rotation, so every orbit here is a full 6-cycle; a
            // partial orbit (some rotation missing or already consumed)
            // would commit tiles whose rotations are absent and break the
            // exact C6 symmetry the tests assert — skip it instead.
            if orbit.len() == 6 {
                orbits.push(orbit);
            }
        }
        // Order the orbits: closing orbits first (they complete 360°
        // vertices and drive the tiling forward), then boundary
        // enumeration order for determinism. (pool_idx already restricted
        // to closing candidates when any exist, so the flag only matters
        // in the all-non-closing fallback case, where it keeps the sort
        // stable.)
        let mut scored: Vec<(u8, usize)> = Vec::new();
        for (oi, orbit) in orbits.iter().enumerate() {
            let is_closing =
                orbit.iter().any(|&i| self.closes_vertex(&[], &pool[pool_idx[i]].0));
            scored.push((if is_closing { 0 } else { 1 }, oi));
        }
        scored.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        // Commit the first orbit that stays valid tile by tile, keeps the
        // patch a topological disk, and strands no unfillable boundary
        // edge (the dead-edge lookahead below).
        for (_c, oi) in &scored {
            let orbit = &orbits[*oi];
            let orbit_pts: Vec<[Vec2; 5]> =
                orbit.iter().map(|&i| pool[pool_idx[i]].0).collect();
            if self.orbit_euler_delta(&orbit_pts) != 0 {
                continue;
            }
            let mut extra: Vec<Tile5> = Vec::new();
            let mut extra_bb: Vec<BBox> = Vec::new();
            let mut ok = true;
            for &i in orbit {
                let (pts, a, b) = &pool[pool_idx[i]];
                if !self.valid(&extra, &extra_bb, pts, *a, *b) {
                    ok = false;
                    break;
                }
                extra_bb.push(bbox_of(pts));
                extra.push(Tile5 { pts: *pts });
            }
            if ok && self.kills_boundary(&extra, &extra_bb) {
                continue; // would strand an unfillable boundary edge
            }
            if ok {
                for t in &extra {
                    self.add_tile(t.pts);
                }
                return true;
            }
        }
        false
    }

    /// Trace the patch boundary as one cycle (vertices, no repeated
    /// endpoint). None when the boundary is not a single simple cycle (a
    /// pinch or pocket) — which the symmetric growth never produces.
    fn boundary_cycle(&self) -> Option<Vec<Vec2>> {
        let edges = self.boundary();
        let n = edges.len();
        if n == 0 {
            return Some(Vec::new());
        }
        let mut adj: HashMap<PKey, Vec<usize>> = HashMap::new();
        for (i, &(a, _)) in edges.iter().enumerate() {
            adj.entry(PKey::of(a)).or_default().push(i);
        }
        let mut visited = vec![false; n];
        let mut cur = 0usize;
        visited[0] = true;
        let mut verts = vec![edges[0].0];
        let mut steps = 0;
        loop {
            let (a, b) = edges[cur];
            verts.push(b);
            steps += 1;
            if steps >= n {
                break;
            }
            let bk = PKey::of(b);
            let cands: Vec<usize> = adj
                .get(&bk)
                .map(|v| v.iter().copied().filter(|&e| !visited[e]).collect())
                .unwrap_or_default();
            if cands.is_empty() {
                break;
            }
            // Continue along the edge immediately clockwise of the reverse
            // direction (the ray bounding the entry gap on its far side).
            let d_back = a - b;
            let next = cands
                .iter()
                .copied()
                .min_by(|&x, &y| {
                    cw_angle(d_back, edges[x].0 - b)
                        .total_cmp(&cw_angle(d_back, edges[y].0 - b))
                })
                .unwrap();
            visited[next] = true;
            cur = next;
        }
        let closed = visited.iter().all(|&v| v)
            && steps == n
            && PKey::of(*verts.last().unwrap()) == PKey::of(verts[0]);
        if !closed {
            return None;
        }
        let _ = verts.pop();
        Some(verts)
    }

    /// Does the patch provably contain `box_`? All box-boundary samples and
    /// an interior sample grid lie inside the boundary cycle, and no
    /// boundary edge properly crosses the box.
    fn covers_box(&self, box_: &BBox) -> bool {
        if !self.bb_all.contains_point(box_.min) || !self.bb_all.contains_point(box_.max) {
            return false;
        }
        let Some(cyc) = self.boundary_cycle() else {
            return false;
        };
        if cyc.len() < 3 {
            return false;
        }
        let poly = Polygon::from_slice(&cyc);
        let (lo, hi) = (box_.min, box_.max);
        let edges4 = [
            (lo, Vec2::new(hi.x, lo.y)),
            (Vec2::new(hi.x, lo.y), hi),
            (hi, Vec2::new(hi.x, lo.y)),
            (Vec2::new(hi.x, lo.y), lo),
        ];
        for &(c, d) in &edges4 {
            if !poly.contains(c) || !poly.contains(d) {
                return false;
            }
            for k in 0..16 {
                let t = (k as f64 + 0.5) / 16.0;
                let p = c + (d - c) * t;
                if !poly.contains(p) {
                    return false;
                }
            }
        }
        // Interior 5×5 grid (also catches a boundary edge running along
        // the box with the patch on the outside).
        let sz = box_.size();
        for gy in 0..5 {
            for gx in 0..5 {
                let p = lo + Vec2::new(sz.x * ((gx as f64 + 0.5) / 5.0), sz.y * ((gy as f64 + 0.5) / 5.0));
                if !poly.contains(p) {
                    return false;
                }
            }
        }
        for i in 0..cyc.len() {
            let a = cyc[i];
            let b = cyc[(i + 1) % cyc.len()];
            for &(c, d) in &edges4 {
                if segs_properly_cross(a, b, c, d) {
                    return false;
                }
            }
        }
        true
    }
}

// ---------------------------------------------------------------------------
// Placement / rotation helpers
// ---------------------------------------------------------------------------

/// Rigid map taking template edge k → k+1 onto the directed unit edge
/// (r1, r2), applied to the whole template.
fn place(tmpl: &[Vec2; 5], k: usize, r1: Vec2, r2: Vec2) -> [Vec2; 5] {
    let t1 = tmpl[k];
    let t2 = tmpl[(k + 1) % 5];
    let ang = (r2 - r1).y.atan2((r2 - r1).x) - (t2 - t1).y.atan2((t2 - t1).x);
    let mut out = [Vec2::ZERO; 5];
    for (i, &p) in tmpl.iter().enumerate() {
        out[i] = r1 + (p - t1).rotated(ang);
    }
    out
}

fn rot60_arr(pts: &[Vec2; 5]) -> [Vec2; 5] {
    pts.map(|p| p.rotated(ANG60))
}

/// Clockwise angle (0, 2π) to rotate `u` onto `v`.
fn cw_angle(u: Vec2, v: Vec2) -> f64 {
    let ccw = (v.y.atan2(v.x) - u.y.atan2(u.x)).rem_euclid(TAU);
    if ccw < 1e-12 {
        0.0
    } else {
        TAU - ccw
    }
}

// ---------------------------------------------------------------------------
// Tiling trait implementation
// ---------------------------------------------------------------------------

/// A grown patch plus a flag that growth has permanently stopped (tile
/// budget hit, or frontier deadlocked). From the same state, growth would
/// re-enumerate the whole boundary and fail identically again, so once
/// `maxed`, any box the patch does not cover is rejected without
/// re-attempting.
struct Grown {
    patch: Patch,
    maxed: bool,
}

/// Hirschhorn nonperiodic pentagonal tiling.
///
/// The grown patch is cached and continued in place across
/// [`Tiling::generate`] calls: growth is a deterministic function of the
/// algorithm alone (the parameters only select the queried box), so a
/// patch grown for one view is valid for every other. Without the cache
/// each UI frame regrew the whole C6 patch from the seed flower —
/// seconds to minutes for views far from the seed — the reported UI
/// hang.
pub struct HirschhornTiling {
    cache: Mutex<Option<Grown>>,
}

impl HirschhornTiling {
    pub fn new() -> Self {
        Self { cache: Mutex::new(None) }
    }
}

impl Default for HirschhornTiling {
    fn default() -> Self {
        Self::new()
    }
}

impl Tiling for HirschhornTiling {
    fn id(&self) -> &'static str {
        "hirschhorn"
    }

    fn name(&self) -> &'static str {
        "Hirschhorn pentagon"
    }

    fn description(&self) -> &'static str {
        "Nonperiodic monohedral tiling by the 140°/60°/160°/80°/100° pentagon (Hirschhorn–Hunt 1985), grown 6-fold symmetrically."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Nonperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None)
            .with_notes("nonperiodic monohedral; C6-symmetric growth from a seed flower");
        let fail = |message: &'static str| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: None,
            message: Some(message),
        };

        let map = similarity_map(params);
        let inv = map.inverse();

        // Region in the tiling's own (unit-edge) coordinates.
        let mut rmin = Vec2::new(f64::MAX, f64::MAX);
        let mut rmax = Vec2::new(f64::MIN, f64::MIN);
        for &corner in &query.region.corners() {
            let q = inv.transform(corner);
            rmin.x = rmin.x.min(q.x);
            rmin.y = rmin.y.min(q.y);
            rmax.x = rmax.x.max(q.x);
            rmax.y = rmax.y.max(q.y);
        }
        let cell_box = BBox::new(rmin, rmax);
        if cell_box.area() <= 1e-9 {
            return fail("empty view");
        }
        let pent_area = Polygon::from_slice(&template_ccw()).area();
        // The patch is a jagged disk about the origin, so covering the
        // box needs ~π·r²/pent_area tiles, r = farthest box corner.
        // Rejected up front, before touching the (expensive) growth —
        // this is what keeps far-from-origin or very wide views instant.
        // Subsumes the old box-area check (the box lies inside that disk).
        let mut r_need: f64 = 0.0;
        for &c in &cell_box.corners() {
            r_need = r_need.max(c.length());
        }
        r_need += 1.5; // tile-size margin for the jagged frontier
        if TAU / 2.0 * r_need * r_need / pent_area > MAX_TILES as f64 {
            return fail("too many tiles for this view — zoom in");
        }

        // Continue the cached C6-symmetric patch until it provably
        // contains the box (growth is parameter-independent, so the
        // cache is valid for any view/params).
        let mut guard = self.cache.lock().unwrap_or_else(|e| e.into_inner());
        let grown =
            guard.get_or_insert_with(|| Grown { patch: Patch::new(), maxed: false });
        if !grown.patch.covers_box(&cell_box) {
            if grown.maxed {
                drop(guard);
                return fail("too many tiles for this view — zoom in");
            }
            let mut steps = 0;
            while !grown.patch.covers_box(&cell_box) {
                if !grown.patch.grow_step() {
                    grown.maxed = true;
                    drop(guard);
                    return fail("hirschhorn: growth failed for this view");
                }
                steps += 1;
                if grown.patch.tiles.len() > MAX_TILES {
                    grown.maxed = true;
                    drop(guard);
                    return fail("too many tiles for this view — zoom in");
                }
                if steps > MAX_GROW_STEPS {
                    // Consistent with the other failure paths: growth has
                    // not finished within the budget, and retrying from
                    // the same state would repeat the same work every
                    // frame. Mark the patch maxed so later views fail
                    // fast. (With complete 6-orbits committed per step,
                    // the tile-budget check above always fires first —
                    // 100k steps would mean ≥ 600k tiles — so this path
                    // is unreachable belt-and-braces.)
                    grown.maxed = true;
                    drop(guard);
                    return fail("hirschhorn: growth did not converge for this view");
                }
            }
        }

        // Cull to the region and map to world coordinates. The pattern
        // seed is the tile's frame (direction of the A→B edge) in 18
        // sectors of 20° — all frames of the tiling are 20° multiples.
        let mut tiles = Vec::with_capacity(grown.patch.tiles.len());
        for t in &grown.patch.tiles {
            let poly = Polygon::from_slice(&t.pts).transform(&map);
            // Cull in world space: a tile whose tiling-space bbox overlaps
            // the box can rotate entirely outside the query region.
            if !poly.aabb().intersects(query.region) {
                continue;
            }
            let d = t.pts[1] - t.pts[0];
            let ang = d.y.atan2(d.x).rem_euclid(TAU);
            let seed = ((ang / (TAU / 18.0)).round() % 18.0) as u32;
            tiles.push(Tile { proto: 0, poly, seed });
        }

        TilingResult {
            tiles,
            symmetry: meta,
            periodic: None,
            message: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn pent() -> [Vec2; 5] {
        template_ccw()
    }

    /// Grow the patch by `n` orbits (6 tiles each) on top of the seed.
    fn grow_n_orbits(n: usize) -> Patch {
        let mut p = Patch::new();
        for _ in 0..n {
            assert!(p.grow_step(), "growth stalled at {} tiles", p.tiles.len());
        }
        p
    }

    /// All five edges unit length, the pentagon closes, and the interior
    /// angles are exactly 140°/60°/160°/80°/100°.
    #[test]
    fn pentagon_is_unit_equilateral_with_expected_angles() {
        let v = pent();
        for i in 0..5 {
            let l = (v[(i + 1) % 5] - v[i]).length();
            assert!((l - 1.0).abs() < 1e-12, "edge {i} length {l} != 1");
        }
        let want: [f64; 5] = [140.0, 60.0, 160.0, 80.0, 100.0];
        for i in 0..5 {
            let a = v[(i + 4) % 5] - v[i];
            let b = v[(i + 1) % 5] - v[i];
            let cos_a = a.dot(b) / (a.length() * b.length());
            let ang = cos_a.clamp(-1.0, 1.0).acos().to_degrees();
            assert!((ang - want[i]).abs() < 1e-9, "angle at V{i}: {ang} != {}", want[i]);
        }
        let area = Polygon::from_slice(&v).area();
        assert!((area - 1.4178).abs() < 1e-4, "area {area}");
    }

    /// M101 census audit: the template is a type-1 pentagon in the
    /// repo lettering, ccw shift k=1 (A = v1, B = v2, C = v3,
    /// D = v4, E = v0; repo letters shift the Wikipedia letters by
    /// one — W-angle X = our-(X + 1 mod 5), side a joins A–B):
    /// C + D = 180° and A + B + E = 360° on the template angle
    /// constants [140, 60, 160, 80, 100]. Pins the audit finding
    /// that the shape is type 1 — the old "type 8" attribution in
    /// PROPOSED.md §5 is falsified (0/10 letterings satisfy
    /// 2C + D = E + 2A = 360°; the sides are all equal, but the
    /// angle-sum pairs come out 260/260 and 280/280, never
    /// 360/360 — and type-8 tilings are periodic pgg, while this
    /// tiling is nonperiodic).
    #[test]
    fn template_is_pentagon_type_1() {
        let v = pent();
        let want: [f64; 5] = [140.0, 60.0, 160.0, 80.0, 100.0];
        let mut ang = [0.0; 5];
        for i in 0..5 {
            let a = v[(i + 4) % 5] - v[i];
            let b = v[(i + 1) % 5] - v[i];
            let cos_a = a.dot(b) / (a.length() * b.length());
            ang[i] = cos_a.clamp(-1.0, 1.0).acos().to_degrees();
        }
        for i in 0..5 {
            assert!((ang[i] - want[i]).abs() < 1e-9, "angle at V{i}: {} != {}", ang[i], want[i]);
        }
        // ccw k=1: A = v1, B = v2, C = v3, D = v4, E = v0.
        let (a, b, c, d, e) = (ang[1], ang[2], ang[3], ang[4], ang[0]);
        assert!((c + d - 180.0).abs() < 1e-12, "C + D = {c} + {d} != 180");
        assert!(
            (a + b + e - 360.0).abs() < 1e-12,
            "A + B + E = {a} + {b} + {e} != 360"
        );
    }

    /// The seed flower: 6 petals, 12 shared + 18 boundary edges, a single
    /// 18-edge boundary cycle, no conflicts between petals.
    #[test]
    fn seed_flower_structure() {
        let p = Patch::new();
        assert_eq!(p.tiles.len(), 6);
        let mut interior = 0;
        let mut boundary = 0;
        for &c in p.edge_count.values() {
            if c == 2 {
                interior += 1;
            } else if c == 1 {
                boundary += 1;
            } else {
                panic!("edge owned {c} times");
            }
        }
        assert_eq!(interior, 6, "want 6 shared edges");
        assert_eq!(boundary, 18, "want 18 boundary edges");
        // The 6 B vertices all sit at the origin: 6 × 60° = 360°.
        let origin = PKey::of(Vec2::ZERO);
        assert_eq!(p.angle.get(&origin).copied(), Some(18));
        let cyc = p.boundary_cycle().expect("flower boundary must be one cycle");
        assert_eq!(cyc.len(), 18);
        // No two petals conflict.
        for i in 0..6 {
            for j in (i + 1)..6 {
                let ci = p.tiles[i].centroid();
                let cj = p.tiles[j].centroid();
                assert!(
                    !Patch::tile_conflicts(&p.tiles[j], &p.bbs[j], &p.tiles[i].pts, &p.bbs[i], ci)
                        && !Patch::tile_conflicts(&p.tiles[i], &p.bbs[i], &p.tiles[j].pts, &p.bbs[j], cj),
                    "petals {i} and {j} conflict"
                );
            }
        }
    }

    /// Grow past the first left-handed tile (it appears at 186 tiles) and
    /// assert every invariant of a valid Hirschhorn patch.
    #[test]
    fn growth_invariants() {
        let p = grow_n_orbits(31); // 6 + 31×6 = 192 tiles
        let n = p.tiles.len();
        assert_eq!(n, 192);

        // (a) No overlapping pairs (O(n²) with bbox early-out).
        for i in 0..n {
            for j in (i + 1)..n {
                if !p.bbs[i].intersects(p.bbs[j]) {
                    continue;
                }
                let ci = p.tiles[i].centroid();
                let cj = p.tiles[j].centroid();
                assert!(
                    !Patch::tile_conflicts(&p.tiles[j], &p.bbs[j], &p.tiles[i].pts, &p.bbs[i], ci)
                        && !Patch::tile_conflicts(&p.tiles[i], &p.bbs[i], &p.tiles[j].pts, &p.bbs[j], cj),
                    "tiles {i} and {j} conflict"
                );
            }
        }

        // (b) Edge ownership ≤ 2, and every interior vertex sums to 360°.
        for &c in p.edge_count.values() {
            assert!(c <= 2, "edge owned {c} times");
        }
        let bverts: HashSet<PKey> =
            p.boundary().into_iter().flat_map(|(a, b)| [PKey::of(a), PKey::of(b)]).collect();
        for (k, &s) in &p.angle {
            assert!((s as u32) <= 18);
            if !bverts.contains(k) {
                assert_eq!(s, 18, "interior vertex {k:?} sums to {} (want 18)", s * 20);
            }
        }

        // (c) Euler: the patch is a topological disk (no holes/pockets).
        assert_eq!(p.euler(), 1, "Euler: χ={}", p.euler());

        // (d) Mixed chirality, right-handed dominated.
        let mut rh = 0;
        let mut lh = 0;
        for t in &p.tiles {
            if t.ccw() {
                rh += 1;
            } else {
                lh += 1;
            }
        }
        assert!(rh > 0 && lh > 0, "expected both chiralities: rh={rh} lh={lh}");
        assert!(rh > lh, "growth is RH-dominated: rh={rh} lh={lh}");

        // (e) Every frame is a multiple of 20° (finite orientation set).
        for t in &p.tiles {
            let d = t.pts[1] - t.pts[0];
            let deg = d.y.atan2(d.x).to_degrees().rem_euclid(360.0);
            let m = (deg / 20.0).round();
            assert!((deg - m * 20.0).abs() < 1e-6, "frame {deg} not a 20° multiple");
        }

        // (f) Exact C6 symmetry about the seed origin.
        let set: HashSet<Sign> = p.tiles.iter().map(|t| sign_of(&t.pts)).collect();
        for (i, t) in p.tiles.iter().enumerate() {
            let r = rot60_arr(&t.pts);
            assert!(set.contains(&sign_of(&r)), "tile {i} breaks C6 symmetry");
        }
    }

    /// The boundary stays one simple cycle (no pinches/pockets) through a
    /// long growth.
    #[test]
    fn boundary_stays_simple() {
        let p = grow_n_orbits(60); // 366 tiles
        let cyc = p.boundary_cycle().expect("boundary must stay one cycle");
        assert_eq!(cyc.len(), p.boundary().len());
        // Still a disk.
        assert_eq!(p.euler(), 1, "Euler: χ={}", p.euler());
    }
}
