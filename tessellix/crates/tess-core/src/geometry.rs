//! 2D geometry primitives: vectors, affine maps, polygons, bounding boxes.

use std::fmt;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

/// A 2D vector (or point, see [`Point`]).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

/// A point in the plane.
pub type Point = Vec2;

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub const fn dot(self, o: Self) -> f64 {
        self.x * o.x + self.y * o.y
    }

    /// 2D cross product (z-component of the 3D cross product).
    pub const fn cross(self, o: Self) -> f64 {
        self.x * o.y - self.y * o.x
    }

    pub fn length(self) -> f64 {
        (self.dot(self)).sqrt()
    }

    pub fn rotated(self, theta: f64) -> Self {
        let (s, c) = theta.sin_cos();
        Self {
            x: c * self.x - s * self.y,
            y: s * self.x + c * self.y,
        }
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Self {
            x: self.x + o.x,
            y: self.y + o.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        Self {
            x: self.x - o.x,
            y: self.y - o.y,
        }
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, s: f64) -> Self {
        Self {
            x: self.x / s,
            y: self.y / s,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, o: Self) {
        *self = *self + o;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, o: Self) {
        *self = *self - o;
    }
}

impl std::ops::Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, s: f64) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.4}, {:.4})", self.x, self.y)
    }
}

/// An affine map `p -> m * p + t`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Affine {
    pub m: [[f64; 2]; 2],
    pub t: Vec2,
}

impl Affine {
    pub const fn new(m: [[f64; 2]; 2], t: Vec2) -> Self {
        Self { m, t }
    }

    pub const fn identity() -> Self {
        Self {
            m: [[1.0, 0.0], [0.0, 1.0]],
            t: Vec2::ZERO,
        }
    }

    pub fn translation(v: Vec2) -> Self {
        Self {
            m: [[1.0, 0.0], [0.0, 1.0]],
            t: v,
        }
    }

    pub fn rotation(theta: f64) -> Self {
        let (s, c) = theta.sin_cos();
        Self {
            m: [[c, -s], [s, c]],
            t: Vec2::ZERO,
        }
    }

    /// Uniform scale about the origin.
    pub fn scale(s: f64) -> Self {
        Self::scale_xy(s, s)
    }

    /// Anisotropic scale about the origin.
    pub fn scale_xy(sx: f64, sy: f64) -> Self {
        Self {
            m: [[sx, 0.0], [0.0, sy]],
            t: Vec2::ZERO,
        }
    }

    /// Composition: apply `self` first, then `other`.
    pub fn then(self, other: Self) -> Self {
        Self {
            m: [
                [
                    other.m[0][0] * self.m[0][0] + other.m[0][1] * self.m[1][0],
                    other.m[0][0] * self.m[0][1] + other.m[0][1] * self.m[1][1],
                ],
                [
                    other.m[1][0] * self.m[0][0] + other.m[1][1] * self.m[1][0],
                    other.m[1][0] * self.m[0][1] + other.m[1][1] * self.m[1][1],
                ],
            ],
            t: Vec2 {
                x: other.m[0][0] * self.t.x + other.m[0][1] * self.t.y + other.t.x,
                y: other.m[1][0] * self.t.x + other.m[1][1] * self.t.y + other.t.y,
            },
        }
    }

    pub fn inverse(&self) -> Affine {
        let det = self.m[0][0] * self.m[1][1] - self.m[0][1] * self.m[1][0];
        // A singular map has no inverse. Returning identity keeps release
        // builds total; debug builds fail loudly at the call site instead
        // of silently generating geometry in the wrong space.
        debug_assert!(det.abs() >= 1e-15, "singular affine map (det = {det})");
        if det.abs() < 1e-15 {
            return Affine::identity();
        }
        let inv = Self {
            m: [
                [self.m[1][1] / det, -self.m[0][1] / det],
                [-self.m[1][0] / det, self.m[0][0] / det],
            ],
            t: Vec2::ZERO,
        };
        Self {
            m: inv.m,
            t: -inv.transform(self.t),
        }
    }

    pub fn transform(&self, p: Vec2) -> Vec2 {
        Vec2 {
            x: self.m[0][0] * p.x + self.m[0][1] * p.y + self.t.x,
            y: self.m[1][0] * p.x + self.m[1][1] * p.y + self.t.y,
        }
    }
}

impl Default for Affine {
    fn default() -> Self {
        Self::identity()
    }
}

/// A simple axis-aligned bounding box.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BBox {
    pub min: Vec2,
    pub max: Vec2,
}

impl BBox {
    pub const fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn center(self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    pub fn size(self) -> Vec2 {
        self.max - self.min
    }

    pub fn area(self) -> f64 {
        (self.max.x - self.min.x) * (self.max.y - self.min.y)
    }

    pub fn contains_point(self, p: Vec2) -> bool {
        p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
    }

    /// Intersection test with a small tolerance for shared boundaries.
    pub fn intersects(self, o: BBox) -> bool {
        self.min.x <= o.max.x + 1e-9
            && o.min.x <= self.max.x + 1e-9
            && self.min.y <= o.max.y + 1e-9
            && o.min.y <= self.max.y + 1e-9
    }

    pub fn union(self, o: BBox) -> BBox {
        BBox {
            min: Vec2::new(self.min.x.min(o.min.x), self.min.y.min(o.min.y)),
            max: Vec2::new(self.max.x.max(o.max.x), self.max.y.max(o.max.y)),
        }
    }

    pub fn padding(self, e: f64) -> BBox {
        BBox {
            min: self.min - Vec2::new(e, e),
            max: self.max + Vec2::new(e, e),
        }
    }

    pub fn corners(self) -> [Vec2; 4] {
        [
            self.min,
            Vec2::new(self.max.x, self.min.y),
            self.max,
            Vec2::new(self.min.x, self.max.y),
        ]
    }
}

impl fmt::Display for BBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} .. {}]", self.min, self.max)
    }
}

/// A (possibly non-convex) polygon given by its vertex loop (not repeated).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Polygon {
    pub pts: Vec<Vec2>,
}

impl Polygon {
    pub fn new(pts: Vec<Vec2>) -> Self {
        Self { pts }
    }

    pub fn from_slice(pts: &[Vec2]) -> Self {
        Self { pts: pts.to_vec() }
    }

    pub fn len(&self) -> usize {
        self.pts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pts.len() < 3
    }

    pub fn aabb(&self) -> BBox {
        // Empty input yields a degenerate box (callers never pass empty
        // polygons; the fallback only prevents an index panic).
        let Some(&first) = self.pts.first() else {
            return BBox { min: Vec2::ZERO, max: Vec2::ZERO };
        };
        let mut min = first;
        let mut max = first;
        for &p in &self.pts[1..] {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
        }
        BBox { min, max }
    }

    /// Absolute area (shoelace formula).
    pub fn area(&self) -> f64 {
        let n = self.pts.len();
        if n < 3 {
            return 0.0;
        }
        let mut a = 0.0;
        for i in 0..n {
            let p = self.pts[i];
            let q = self.pts[(i + 1) % n];
            a += p.x * q.y - q.x * p.y;
        }
        a.abs() * 0.5
    }

    /// Even-odd point containment; boundary points count as contained.
    pub fn contains(&self, p: Vec2) -> bool {
        let n = self.pts.len();
        if n < 3 {
            return false;
        }
        let mut inside = false;
        for i in 0..n {
            let a = self.pts[i];
            let b = self.pts[(i + 1) % n];
            if on_segment(p, a, b) {
                return true;
            }
            if (a.y > p.y) != (b.y > p.y) {
                let x_at = a.x + (p.y - a.y) * (b.x - a.x) / (b.y - a.y);
                if x_at > p.x {
                    inside = !inside;
                }
            }
        }
        inside
    }

    pub fn transform(&self, a: &Affine) -> Polygon {
        Polygon {
            pts: self.pts.iter().map(|p| a.transform(*p)).collect(),
        }
    }

    pub fn translated(&self, d: Vec2) -> Polygon {
        Polygon {
            pts: self.pts.iter().map(|p| *p + d).collect(),
        }
    }

    /// Clip against an axis-aligned box (Sutherland–Hodgman).
    /// Returns `None` when the result is empty or degenerate.
    pub fn clipped_to(&self, b: &BBox) -> Option<Polygon> {
        let mut cur = self.pts.clone();
        cur = clip_halfplane(&cur, |p| p.x - b.min.x);
        if cur.len() < 3 {
            return None;
        }
        cur = clip_halfplane(&cur, |p| b.max.x - p.x);
        if cur.len() < 3 {
            return None;
        }
        cur = clip_halfplane(&cur, |p| p.y - b.min.y);
        if cur.len() < 3 {
            return None;
        }
        cur = clip_halfplane(&cur, |p| b.max.y - p.y);
        if cur.len() < 3 {
            return None;
        }
        let poly = Polygon { pts: cur };
        if poly.area() < 1e-12 {
            None
        } else {
            Some(poly)
        }
    }

    /// Triangulate this (simple) polygon by ear clipping.
    ///
    /// Returns index triples into `self.pts`; the triangles cover the
    /// polygon exactly, convex or concave. Renderers must use this for
    /// arbitrary tiles: a fan from a single vertex is only a valid
    /// triangulation for convex polygons.
    pub fn triangulate(&self) -> Vec<[u32; 3]> {
        let n = self.pts.len();
        assert!(n >= 3, "triangulate requires at least 3 vertices");
        let mut idx: Vec<usize> = (0..n).collect();
        if signed_area(&self.pts, &idx) < 0.0 {
            idx.reverse(); // normalize to counterclockwise
        }
        let mut tris: Vec<[u32; 3]> = Vec::with_capacity(n - 2);
        while idx.len() > 3 {
            let m = idx.len();
            let ear = (0..m).find(|&i| is_ear(&self.pts, &idx, i, true))
                .or_else(|| (0..m).find(|&i| is_ear(&self.pts, &idx, i, false)));
            match ear {
                Some(i) => {
                    let i0 = (i + m - 1) % m;
                    let i2 = (i + 1) % m;
                    tris.push([idx[i0] as u32, idx[i] as u32, idx[i2] as u32]);
                    idx.remove(i);
                }
                // No ear at all: input is not a simple polygon (the
                // generators never emit those). Fall back to a fan rather
                // than hang.
                None => {
                    for k in 1..m - 1 {
                        tris.push([idx[0] as u32, idx[k] as u32, idx[k + 1] as u32]);
                    }
                    return tris;
                }
            }
        }
        tris.push([idx[0] as u32, idx[1] as u32, idx[2] as u32]);
        tris
    }
}

fn signed_area(pts: &[Vec2], idx: &[usize]) -> f64 {
    let mut a = 0.0;
    for i in 0..idx.len() {
        let p = pts[idx[i]];
        let q = pts[idx[(i + 1) % idx.len()]];
        a += p.x * q.y - q.x * p.y;
    }
    a * 0.5
}

/// Is vertex `i` of the CCW polygon `idx` an ear: convex (strictly, when
/// `strict`; weakly otherwise, to admit collinear vertices), its triangle
/// holds no other polygon vertex, and no other vertex lies on the ear's
/// diagonal (prev–next) — clipping such an ear would leave a pinched,
/// weakly-simple remainder. The weak pass additionally allows collinear
/// vertices on the two edges adjacent to the ear vertex.
fn is_ear(pts: &[Vec2], idx: &[usize], i: usize, strict: bool) -> bool {
    let m = idx.len();
    let i0 = (i + m - 1) % m;
    let i2 = (i + 1) % m;
    let a = pts[idx[i0]];
    let b = pts[idx[i]];
    let c = pts[idx[i2]];
    let turn = (b - a).cross(c - a);
    if strict {
        if turn <= 1e-12 {
            return false;
        }
    } else if turn < -1e-12 {
        return false;
    }
    for j in 0..m {
        if j == i0 || j == i || j == i2 {
            continue;
        }
        let p = pts[idx[j]];
        if point_in_triangle_strict(p, a, b, c) || on_segment(p, a, c) {
            return false;
        }
        if strict && (on_segment(p, a, b) || on_segment(p, b, c)) {
            return false;
        }
    }
    true
}

/// Strict (interior-only) point-in-triangle test.
fn point_in_triangle_strict(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
    let d1 = (b - a).cross(p - a);
    let d2 = (c - b).cross(p - b);
    let d3 = (a - c).cross(p - c);
    (d1 > 1e-12 && d2 > 1e-12 && d3 > 1e-12) || (d1 < -1e-12 && d2 < -1e-12 && d3 < -1e-12)
}

fn on_segment(p: Vec2, a: Vec2, b: Vec2) -> bool {
    let ab = b - a;
    let ab_len = ab.length().max(1e-12);
    if ab.cross(p - a).abs() > 1e-9 * ab_len {
        return false;
    }
    (p - a).dot(p - b) <= 1e-9
}

fn clip_halfplane(poly: &[Vec2], f: impl Fn(Vec2) -> f64) -> Vec<Vec2> {
    let mut out = Vec::with_capacity(poly.len() + 2);
    let n = poly.len();
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        let fa = f(a);
        let fb = f(b);
        if fa >= 0.0 {
            out.push(a);
        }
        if (fa >= 0.0) != (fb >= 0.0) {
            let t = fa / (fa - fb);
            out.push(a + (b - a) * t);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn affine_inverse_roundtrip() {
        let a = Affine::rotation(0.7).then(Affine::scale(2.5)).then(Affine::translation(Vec2::new(3.0, -1.0)));
        let p = Vec2::new(1.3, -4.1);
        let q = a.inverse().transform(a.transform(p));
        assert!((q.x - p.x).abs() < 1e-12);
        assert!((q.y - p.y).abs() < 1e-12);
    }

    #[test]
    fn polygon_area_and_contains() {
        let sq = Polygon::from_slice(&[Vec2::ZERO, Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0)]);
        assert!((sq.area() - 1.0).abs() < 1e-12);
        assert!(sq.contains(Vec2::new(0.5, 0.5)));
        assert!(sq.contains(Vec2::new(1.0, 0.5))); // boundary
        assert!(!sq.contains(Vec2::new(1.5, 0.5)));
    }

    #[test]
    fn clip_rect_partition() {
        // Two unit squares clipped to a half-overlapping box partition its area.
        let box_ = BBox::new(Vec2::new(0.5, 0.0), Vec2::new(1.5, 1.0));
        let a = Polygon::from_slice(&[Vec2::ZERO, Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0)]);
        let b = a.translated(Vec2::new(1.0, 0.0));
        let ca = a.clipped_to(&box_).unwrap();
        let cb = b.clipped_to(&box_).unwrap();
        assert!((ca.area() + cb.area() - box_.area()).abs() < 1e-9);
    }

    fn tri_area(pts: &[Vec2], t: &[u32; 3]) -> f64 {
        let (a, b, c) = (pts[t[0] as usize], pts[t[1] as usize], pts[t[2] as usize]);
        (b - a).cross(c - a).abs() * 0.5
    }

    /// True if `p` is in the interior of, or on the boundary of, some triangle.
    fn in_any_tris(pts: &[Vec2], tris: &[[u32; 3]], p: Vec2) -> bool {
        tris.iter().any(|t| {
            let (a, b, c) = (pts[t[0] as usize], pts[t[1] as usize], pts[t[2] as usize]);
            point_in_triangle_strict(p, a, b, c)
                || on_segment(p, a, b)
                || on_segment(p, b, c)
                || on_segment(p, a, c)
        })
    }

    /// Strict interior count of triangles containing `p`.
    fn strict_count(pts: &[Vec2], tris: &[[u32; 3]], p: Vec2) -> usize {
        tris.iter()
            .filter(|t| {
                let (a, b, c) = (pts[t[0] as usize], pts[t[1] as usize], pts[t[2] as usize]);
                point_in_triangle_strict(p, a, b, c)
            })
            .count()
    }

    #[test]
    fn triangulate_convex_square() {
        let sq = Polygon::from_slice(&[
            Vec2::ZERO,
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ]);
        let tris = sq.triangulate();
        assert_eq!(tris.len(), 2);
        let sum: f64 = tris.iter().map(|t| tri_area(&sq.pts, t)).sum();
        assert!((sum - sq.area()).abs() < 1e-12);
        // The center may sit exactly on the diagonal: boundary counts as covered.
        assert!(in_any_tris(&sq.pts, &tris, Vec2::new(0.5, 0.5)));
    }

    #[test]
    fn triangulate_concave_l_shape() {
        // L-shape (hexagon, reflex at (1,1)): area 3; the notch square
        // [(1,1)..(2,2)] must not be covered.
        let l = Polygon::from_slice(&[
            Vec2::new(0.0, 0.0),
            Vec2::new(2.0, 0.0),
            Vec2::new(2.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 2.0),
            Vec2::new(0.0, 2.0),
        ]);
        let tris = l.triangulate();
        assert_eq!(tris.len(), 4);
        let sum: f64 = tris.iter().map(|t| tri_area(&l.pts, t)).sum();
        assert!((sum - l.area()).abs() < 1e-12);
        // Notch region: outside the polygon, must not be covered.
        assert!(!in_any_tris(&l.pts, &tris, Vec2::new(1.75, 1.75)));
        // Interior points: covered (boundary-inclusive; they may land on
        // a diagonal).
        for p in [Vec2::new(0.25, 0.25), Vec2::new(0.25, 1.75), Vec2::new(1.75, 0.25)] {
            assert!(in_any_tris(&l.pts, &tris, p), "{p}");
        }
        // (1.75, 0.25) is off every possible diagonal: covered exactly once.
        assert_eq!(strict_count(&l.pts, &tris, Vec2::new(1.75, 0.25)), 1);
    }

    #[test]
    fn triangulate_star_outline() {
        // 8-pointed star: tips at radius 2 (0°, 45°, …), notches at radius 1
        // (22.5°, 67.5°, …) — same combinatorics as the {8/3} star tile.
        let pts: Vec<Vec2> = (0..16)
            .map(|k| {
                let rad = if k % 2 == 0 { 2.0 } else { 1.0 };
                let ang = k as f64 * std::f64::consts::PI / 8.0;
                Vec2::new(rad * ang.cos(), rad * ang.sin())
            })
            .collect();
        let star = Polygon::from_slice(&pts);
        let tris = star.triangulate();
        assert_eq!(tris.len(), 14);
        let sum: f64 = tris.iter().map(|t| tri_area(&star.pts, t)).sum();
        assert!((sum - star.area()).abs() < 1e-9);
        // Fine-grid coverage check against the even-odd containment test.
        let bb = star.aabb();
        let step = 0.1;
        let mut x = bb.min.x;
        while x <= bb.max.x + 1e-9 {
            let mut y = bb.min.y;
            while y <= bb.max.y + 1e-9 {
                let p = Vec2::new(x, y);
                let on_edge = star
                    .pts
                    .iter()
                    .zip(star.pts.iter().cycle().skip(1))
                    .any(|(a, b)| on_segment(p, *a, *b))
                    || tris.iter().any(|t| {
                        let (a, b, c) = (
                            star.pts[t[0] as usize],
                            star.pts[t[1] as usize],
                            star.pts[t[2] as usize],
                        );
                        on_segment(p, a, b) || on_segment(p, b, c) || on_segment(p, a, c)
                    });
                if !on_edge {
                    let c = strict_count(&star.pts, &tris, p);
                    assert_eq!(c as i32, star.contains(p) as i32, "sample ({x}, {y})");
                }
                y += step;
            }
            x += step;
        }
    }

    #[test]
    fn triangulate_winding_independent() {
        let pts: [Vec2; 6] = [
            Vec2::new(0.0, 0.0),
            Vec2::new(2.0, 0.0),
            Vec2::new(2.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 2.0),
            Vec2::new(0.0, 2.0),
        ];
        let fwd = Polygon::from_slice(&pts).triangulate();
        let rev: Vec<Vec2> = pts.iter().rev().copied().collect();
        let bwd = Polygon::from_slice(&rev).triangulate();
        for (tris, poly) in [
            (&fwd, &Polygon::from_slice(&pts)),
            (&bwd, &Polygon::from_slice(&rev)),
        ] {
            assert_eq!(tris.len(), 4);
            let sum: f64 = tris.iter().map(|t| tri_area(&poly.pts, t)).sum();
            assert!((sum - poly.area()).abs() < 1e-12);
            assert!(!in_any_tris(&poly.pts, tris, Vec2::new(1.75, 1.75)));
        }
    }
}
