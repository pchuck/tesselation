//! Hankin polygons-in-contact (PIC) strapwork **layer mode** (M57).
//!
//! The general engine behind the registered PIC partition tilings
//! (`pic-4-8-8`, `pic-3-6-3-6`, `pic-3-12-12` in `tess-tilings`): given
//! the tiles of an *arbitrary* host tiling and a contact angle δ, it
//! computes the wall network the PIC construction lays over that host —
//! an overlay for rendering, not a registered partition. All arithmetic
//! is f64 (hosts are arbitrary generators); exactness is asserted only
//! where the host is exact, and the three canonical substrates tie this
//! engine to the tiling-mode modules below.
//!
//! ## Construction
//!
//! For every edge midpoint M of every tile, two wall rays depart into
//! that tile at contact angle δ to the edge, symmetric about the inward
//! normal (the same convention as the PIC modules: δ measured from the
//! edge, so δ = 90° runs along the normal). Each ray travels to the tile
//! boundary; the adjacent tile across the edge fires the point-reflected
//! pair at the shared midpoint, so every wall is **continuous across the
//! edge, straight through at M**. Inside its tile a ray is split at
//! every crossing with another ray of the same tile, and each piece
//! between consecutive events (start, crossings, boundary exit) is
//! emitted as one [`Wall`].
//!
//! Crossing-splitting, not first-crossing termination, is what the
//! ground truth demands: the registered partitions cut strands at every
//! crossing — the rosette core of `pic-4-8-8` (octagon edges) and of
//! `pic-3-6-3-6` / `pic-3-12-12` (hexagon / dodecagon edges) is bounded
//! by the chord segments *between two crossings*, which a ray that stops
//! at its first crossing would never emit. The per-tile network is the
//! full strand from midpoint to boundary exit, segmented at crossings;
//! rays therefore terminate at the boundary (rule (b)), and a crossing
//! only ever subdivides (rule (a) refined — PIC modules are ground
//! truth per PROPOSED #14).
//!
//! ## Numerical guards
//!
//! * Tiles with < 3 vertices, or with twice-signed-area below
//!   `1e-12 · scale²` (collinear/degenerate), are skipped; zero-length
//!   edges fire no rays.
//! * Rays whose unit directions satisfy `|d₁ × d₂| < 1e-9` are treated
//!   as parallel and produce no crossing event. This also covers the
//!   collinear case that occurs everywhere: the two ends of one strand
//!   (`M → M'` and `M' → M`) overlap on the same line, and the identical
//!   pieces they emit are removed by the deduplication grid below rather
//!   than turned into spurious split points.
//! * A crossing counts only strictly inside both spans (margin
//!   `1e-9 · tile scale`), events closer than `1e-8 · tile scale` merge
//!   (multi-strand crossings), and pieces shorter than the start margin
//!   are dropped. A boundary hit at `s ≤ 1e-9 · scale` is the ray's own
//!   starting midpoint, never an exit.
//!
//! ## Determinism
//!
//! [`walls`] is a pure function of `(tiles, δ)` — no hidden state, no
//! RNG — and its output is ordered and deduplicated through a
//! [`BTreeMap`] keyed on the segment endpoints rounded to a 1e-6 grid
//! (endpoint-canonical, order- and direction-independent), so the
//! emitted vector is identical across calls on identical input.

use std::collections::BTreeMap;

use crate::geometry::Vec2;
use crate::tiling::Tile;

/// One strapwork wall piece: the segment from `a` to `b`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wall {
    pub a: Vec2,
    pub b: Vec2,
}

impl Wall {
    pub fn length(&self) -> f64 {
        (self.b - self.a).length()
    }
}

/// Edges shorter than this (world units) are degenerate; no midpoint ray.
const EPS_EDGE: f64 = 1e-12;
/// Near-parallel guard on unit ray directions (`|d₁ × d₂| = |sin θ|`).
const EPS_PAR: f64 = 1e-9;
/// Dedup grid: endpoints round to 1e-6 world units (same key convention
/// as the tess-tilings cross-checks).
const GRID: f64 = 1e6;

type SegKey = ((i64, i64), (i64, i64));

fn key_pt(p: Vec2) -> (i64, i64) {
    ((p.x * GRID).round() as i64, (p.y * GRID).round() as i64)
}

/// Endpoint-canonical key: the two grid points in fixed order.
fn key_seg(a: Vec2, b: Vec2) -> SegKey {
    let (ka, kb) = (key_pt(a), key_pt(b));
    if ka <= kb {
        (ka, kb)
    } else {
        (kb, ka)
    }
}

/// A ray inside one tile: start point, unit direction, boundary-exit
/// distance along the direction.
struct Ray {
    p: Vec2,
    d: Vec2,
    exit: f64,
}

/// The PIC strapwork wall network over `tiles` at contact angle
/// `delta_deg` **in degrees**.
///
/// Each wall piece lies fully inside the host tile that produced it;
/// pieces are deduplicated on the 1e-6 endpoint grid and returned in a
/// deterministic (sorted) order. See the module docs for the
/// construction and the numerical guards.
pub fn walls(tiles: &[Tile], delta_deg: f64) -> Vec<Wall> {
    let (sd, cd) = delta_deg.to_radians().sin_cos();
    let mut out: BTreeMap<SegKey, Wall> = BTreeMap::new();

    for tile in tiles {
        let pts = &tile.poly.pts;
        let n = pts.len();
        if n < 3 {
            continue; // degenerate tile
        }
        // Twice the signed area + a per-tile scale for the tolerances.
        let mut a2 = 0.0f64;
        let mut scale = 0.0f64;
        for i in 0..n {
            let (p, q) = (pts[i], pts[(i + 1) % n]);
            a2 += p.cross(q);
            scale = scale.max((q - p).length());
        }
        if a2.abs() < 1e-12 * scale.max(1.0) * scale.max(1.0) {
            continue; // collinear / zero-area tile
        }
        let sgn = if a2 > 0.0 { 1.0 } else { -1.0 };
        let eps = 1e-9 * scale.max(1.0);
        let eps_merge = 1e-8 * scale.max(1.0);

        // Fire the two tilted rays at every non-degenerate edge midpoint.
        let mut rays: Vec<Ray> = Vec::new();
        for i in 0..n {
            let (va, vb) = (pts[i], pts[(i + 1) % n]);
            let e = vb - va;
            let len = e.length();
            if len < EPS_EDGE {
                continue;
            }
            let t = e / len;
            // Inward normal: left of the edge for CCW loops, right for CW.
            let nin = Vec2::new(-t.y * sgn, t.x * sgn);
            let m = (va + vb) * 0.5;
            for lean in [-1.0, 1.0] {
                // Unit: t ⊥ nin, both unit, lean² + … = cd² + sd² = 1.
                let d = Vec2::new(lean * cd * t.x + sd * nin.x, lean * cd * t.y + sd * nin.y);
                // Boundary exit: first hit (s > eps) with any tile edge.
                let mut exit = f64::INFINITY;
                for j in 0..n {
                    let (qa, qb) = (pts[j], pts[(j + 1) % n]);
                    let ee = qb - qa;
                    let den = d.cross(ee);
                    if den.abs() < EPS_PAR * ee.length() {
                        continue; // ray parallel to this edge
                    }
                    let q = qa - m;
                    let s = q.cross(ee) / den;
                    let u = q.cross(d) / den;
                    if s > eps && u > -1e-9 && u < 1.0 + 1e-9 && s < exit {
                        exit = s;
                    }
                }
                if exit.is_finite() && exit > eps_merge {
                    rays.push(Ray { p: m, d, exit });
                }
            }
        }

        // Split each ray at its crossings with the other rays of this
        // tile and emit the pieces.
        for (ri, r) in rays.iter().enumerate() {
            let mut events: Vec<f64> = Vec::new();
            for (rj, o) in rays.iter().enumerate() {
                if ri == rj {
                    continue;
                }
                let den = r.d.cross(o.d);
                if den.abs() < EPS_PAR {
                    continue; // parallel, incl. the collinear strand ends
                }
                let q = o.p - r.p;
                let s1 = q.cross(o.d) / den;
                let s2 = q.cross(r.d) / den;
                if s1 > eps && s1 < r.exit - eps && s2 > eps && s2 < o.exit - eps {
                    events.push(s1);
                }
            }
            events.sort_by(f64::total_cmp);
            let mut prev = 0.0;
            let mut cuts: Vec<f64> = Vec::new();
            for &s in &events {
                if s - prev > eps_merge && r.exit - s > eps_merge {
                    cuts.push(s);
                    prev = s;
                }
            }
            cuts.push(r.exit);
            prev = 0.0;
            for s in cuts {
                let a = r.p + r.d * prev;
                let b = r.p + r.d * s;
                prev = s;
                if (b - a).length() <= eps {
                    continue;
                }
                out.insert(key_seg(a, b), Wall { a, b });
            }
        }
    }

    out.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Polygon;

    fn pt(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    fn tile(pts: &[(f64, f64)]) -> Tile {
        Tile {
            proto: 0,
            poly: Polygon::from_slice(&pts.iter().map(|&(x, y)| pt(x, y)).collect::<Vec<_>>()),
            seed: 0,
        }
    }

    /// `n × n` unit-square host.
    fn square_grid(n: i64) -> Vec<Tile> {
        let mut v = Vec::new();
        for i in 0..n {
            for j in 0..n {
                let (x, y) = (i as f64, j as f64);
                v.push(tile(&[
                    (x, y),
                    (x + 1.0, y),
                    (x + 1.0, y + 1.0),
                    (x, y + 1.0),
                ]));
            }
        }
        v
    }

    fn key(w: &Wall) -> SegKey {
        key_seg(w.a, w.b)
    }

    /// Distance from `p` to segment `ab`.
    fn seg_dist(p: Vec2, a: Vec2, b: Vec2) -> f64 {
        let e = b - a;
        let t = ((p - a).dot(e) / e.dot(e)).clamp(0.0, 1.0);
        (p - (a + e * t)).length()
    }

    #[test]
    fn square_grid_at_45_gives_median_chords() {
        // δ = 45° on the square grid: each square carries the four
        // midpoint-to-midpoint chords of its inscribed diamond, shared
        // midpoint pairs dedupe, so 2×2 tiles yield 16 pieces of length
        // √½ with endpoints exactly on edge midpoints.
        let ws = walls(&square_grid(2), 45.0);
        assert_eq!(ws.len(), 16, "walls: {ws:?}");
        for w in &ws {
            assert!(
                (w.length() - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-12,
                "bad wall {w:?} len {}",
                w.length()
            );
        }
    }

    #[test]
    fn walls_meet_host_edges_at_midpoints() {
        // Midpoint-crossing invariant: on the regular (square, δ = 45°)
        // host every wall endpoint is the midpoint of some host edge —
        // walls cross substrate edges exactly at midpoints by construction.
        let tiles = square_grid(2);
        for w in walls(&tiles, 45.0) {
            for p in [w.a, w.b] {
                let is_midpoint = tiles.iter().any(|t| {
                    let q = &t.poly.pts;
                    (0..q.len()).any(|i| (p - (q[i] + q[(i + 1) % q.len()]) * 0.5).length() < 1e-9)
                });
                assert!(is_midpoint, "endpoint {p} is not an edge midpoint: {w:?}");
            }
        }
    }

    #[test]
    fn walls_never_exit_their_tile() {
        // Termination behavior at δ = 30° on the square grid: no wall
        // leaves its host tile (both endpoints contained — squares are
        // convex), and every endpoint is either on a tile edge (boundary
        // exit or midpoint) or an interior crossing where strands meet
        // (endpoint multiplicity ≥ 4: each crossing splits two strands
        // into four piece-ends). A non-midpoint boundary exit occurs.
        let tiles = square_grid(2);
        let ws = walls(&tiles, 30.0);
        assert!(!ws.is_empty());
        let mut mult: std::collections::BTreeMap<(i64, i64), usize> = Default::default();
        for w in &ws {
            for p in [w.a, w.b] {
                *mult.entry(key_pt(p)).or_default() += 1;
            }
        }
        let mut saw_boundary_exit = false;
        for w in &ws {
            let host = tiles
                .iter()
                .find(|t| t.poly.contains(w.a) && t.poly.contains(w.b))
                .unwrap_or_else(|| panic!("wall {w:?} escapes every tile"));
            for p in [w.a, w.b] {
                let q = &host.poly.pts;
                let d = (0..q.len())
                    .map(|i| seg_dist(p, q[i], q[(i + 1) % q.len()]))
                    .fold(f64::INFINITY, f64::min);
                if d < 1e-9 {
                    let is_mid = (0..q.len())
                        .any(|i| (p - (q[i] + q[(i + 1) % q.len()]) * 0.5).length() < 1e-9);
                    let is_vertex = q.iter().any(|v| (*v - p).length() < 1e-9);
                    if !is_mid && !is_vertex {
                        saw_boundary_exit = true;
                    }
                } else {
                    assert!(
                        mult[&key_pt(p)] >= 4,
                        "interior endpoint {p} of {w:?} is not a strand crossing"
                    );
                }
            }
        }
        assert!(saw_boundary_exit, "no ray ended at a non-midpoint boundary point");
    }

    #[test]
    fn walls_are_deterministic() {
        let tiles = square_grid(2);
        let a = walls(&tiles, 50.0);
        let b = walls(&tiles, 50.0);
        assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(&b) {
            assert_eq!(x, y, "walls must be bit-identical across calls");
        }
    }

    #[test]
    fn degenerate_tiles_are_skipped() {
        // < 3 vertices, collinear "tiles" and zero-area loops emit nothing…
        let junk = vec![
            tile(&[(0.0, 0.0), (1.0, 0.0)]),
            tile(&[(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)]),
            tile(&[]),
        ];
        assert!(walls(&junk, 60.0).is_empty());
        // …and a zero-length edge (duplicated vertex) contributes no rays:
        // the split-edge triangle must yield exactly the plain triangle's walls.
        let tri = tile(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]);
        let dup = tile(&[(0.0, 0.0), (1.0, 0.0), (1.0, 0.0), (0.0, 1.0)]);
        let w_tri = walls(&[tri], 60.0);
        let w_dup = walls(&[dup], 60.0);
        assert_eq!(w_tri.len(), w_dup.len());
        let mut k1: Vec<_> = w_tri.iter().map(key).collect();
        let mut k2: Vec<_> = w_dup.iter().map(key).collect();
        k1.sort();
        k2.sort();
        assert_eq!(k1, k2);
    }
}
