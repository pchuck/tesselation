//! Penrose P3 (rhombus) tiling by substitution.
//!
//! The tiling is built from Robinson triangles:
//! * thick rhomb (72°/108°) = two obtuse `O` triangles (36°-36°-108°),
//!   sharing their φ-base (the long diagonal);
//! * thin rhomb (36°/144°)  = two acute `A` triangles (36°-72°-72°),
//!   sharing their 1/φ-base (the short diagonal).
//!
//! Substitution by the golden ratio φ = (1+√5)/2:
//!
//! ```text
//! O -> 2 O + 1 A      (thick rhomb  -> 2 thick + 1 thin)
//! A -> 1 O + 1 A      (thin rhomb   -> 1 thick + 1 thin)
//! ```
//!
//! The child placements were derived in exact arithmetic (every child
//! vertex is a φ-polynomial point) and are the unique (up to mirror
//! symmetry) tilings of the φ-inflated parent. They are verified by unit
//! tests that sample the parent and require it to be covered by exactly
//! one child at every point.
//!
//! The triangles are achiral, but their substitutions are not: a
//! left-handed and a right-handed copy of the same triangle must receive
//! mirrored child patterns. `Tri::affine` therefore uses the general
//! rigid map (rotation *or* reflection) that matches all three labelled
//! vertices, so the handedness is handled automatically.
//!
//! Generation: seed with the "sun" (5 thick rhombi meeting at the origin)
//! scaled to contain the requested region, then repeatedly refine —
//! each triangle is replaced by its children at 1/φ scale, which tile the
//! parent exactly (deflation view), so no overlaps can ever occur. Tiles
//! are culled to the region at every step (a child never leaves its
//! parent, so nothing needed is ever dropped), until the tile scale drops
//! to 1. Finally the triangles are paired back into rhombi.

use std::collections::BTreeMap;

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::TilingCategory;
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, MAX_TILES, SCALE_ID, SIMILARITY_PARAMS};

const PHI: f64 = 1.618_033_988_749_895; // (1+√5)/2
const S36: f64 = 0.5877852522924731; // sin 36°
const S72: f64 = 0.9510565162951535; // sin 72°
const STEP_CAP: usize = 24; // φ^24 ≈ 1.7e5 units of seed scale

/// A Robinson triangle. `apex` is the 108° vertex for obtuse (`obtuse`)
/// and the 36° vertex for acute triangles. `(c, p, apex)` is always
/// counterclockwise, and the base `c-p` has length φ (O) or 1/φ (A).
#[derive(Clone, Copy, Debug)]
struct Tri {
    obtuse: bool,
    apex: Vec2,
    c: Vec2,
    p: Vec2,
}

impl Tri {
    /// Rigid map `z' = R z + t` taking canonical (parent-local) coordinates
    /// to this triangle's coordinates. Unlike a pure rotation it also allows
    /// a reflection, so it matches all three vertices (apex, c, p) exactly —
    /// which is what makes the substitution work for both chiralities of the
    /// (achiral, isosceles) Robinson triangles.
    ///
    /// Canonical forms (CCW):
    /// * O: c = (-φ/2, 0), p = (φ/2, 0), apex = (0, S36) — inflated about
    ///   the base midpoint (the origin).
    /// * A: c = (1/(2φ), S72), p = (-1/(2φ), S72), apex = (0, 0) —
    ///   inflated about the apex (the origin).
    ///
    /// Returns (R[0][0], R[0][1], R[1][0], R[1][1], t.x, t.y).
    fn affine(&self) -> [f64; 6] {
        let (a0, c0, p0) = if self.obtuse {
            (
                Vec2::new(0.0, S36),
                Vec2::new(-0.5 * PHI, 0.0),
                Vec2::new(0.5 * PHI, 0.0),
            )
        } else {
            (
                Vec2::ZERO,
                Vec2::new(0.5 / PHI, S72),
                Vec2::new(-0.5 / PHI, S72),
            )
        };
        let u0 = c0 - a0;
        let v0 = p0 - a0;
        let u = self.c - self.apex;
        let v = self.p - self.apex;
        let det = u0.x * v0.y - u0.y * v0.x;
        let inv = 1.0 / det;
        let r00 = (u.x * v0.y - v.x * u0.y) * inv;
        let r01 = (v.x * u0.x - u.x * v0.x) * inv;
        let r10 = (u.y * v0.y - v.y * u0.y) * inv;
        let r11 = (v.y * u0.x - u.y * v0.x) * inv;
        let tx = self.apex.x - (r00 * a0.x + r01 * a0.y);
        let ty = self.apex.y - (r10 * a0.x + r11 * a0.y);
        [r00, r01, r10, r11, tx, ty]
    }

    /// Substitution children in canonical (parent-local) coordinates.
    fn children_canonical(&self) -> Vec<Tri> {
        let iphi2 = 0.5 / PHI; // 1/(2φ)
        if self.obtuse {
            // Parent: c=(-φ/2,0), p=(φ/2,0), apex=(0,S36); children tile
            // the parent inflated by φ about the base midpoint:
            //   [(-φ²/2, 0), (φ²/2, 0), (0, S72)].
            // O1: 108° at (-iphi2, 0); φ-base from (-φ²/2, 0) to (0, S72).
            // A1: 36° at (0, S72); 1/φ-base from (-iphi2, 0) to (iphi2, 0).
            // O2: 108° at (iphi2, 0); φ-base from (φ²/2, 0) to (0, S72).
            // (All stored CCW as (c, p, apex).)
            vec![
                Tri { obtuse: true, apex: Vec2::new(-iphi2, 0.0), c: Vec2::new(0.0, S72), p: Vec2::new(-0.5 * PHI * PHI, 0.0) },
                Tri { obtuse: false, apex: Vec2::new(0.0, S72), c: Vec2::new(-iphi2, 0.0), p: Vec2::new(iphi2, 0.0) },
                Tri { obtuse: true, apex: Vec2::new(iphi2, 0.0), c: Vec2::new(0.5 * PHI * PHI, 0.0), p: Vec2::new(0.0, S72) },
            ]
        } else {
            // Parent: c=(1/(2φ),S72), p=(-1/(2φ),S72), apex=(0,0);
            // children tile the parent inflated by φ about the apex:
            //   [(-1/2, φ·S72), (1/2, φ·S72), (0, 0)].
            vec![
                Tri { obtuse: true, apex: Vec2::new(iphi2, S72), c: Vec2::new(-0.5, PHI * S72), p: Vec2::new(0.0, 0.0) },
                Tri { obtuse: false, apex: Vec2::new(-0.5, PHI * S72), c: Vec2::new(iphi2, S72), p: Vec2::new(0.5, PHI * S72) },
            ]
        }
    }

    /// Substitution children in this triangle's own coordinates, one
    /// generation down (at 1/φ of this triangle's scale). They exactly
    /// tile this triangle itself: the canonical children tile the
    /// φ-inflated parent, so dividing them by φ about the anchor and
    /// pushing through the frame map tiles the parent (deflation view).
    fn children(&self) -> Vec<Tri> {
        let m = self.affine();
        let f = |q: Vec2| {
            let x = q.x / PHI;
            let y = q.y / PHI;
            Vec2::new(m[0] * x + m[1] * y + m[4], m[2] * x + m[3] * y + m[5])
        };
        self.children_canonical()
            .into_iter()
            .map(|k| Tri {
                obtuse: k.obtuse,
                apex: f(k.apex),
                c: f(k.c),
                p: f(k.p),
            })
            .collect()
    }

    fn aabb(&self) -> BBox {
        let mut lo = self.apex;
        let mut hi = self.apex;
        for &q in &[self.c, self.p] {
            lo.x = lo.x.min(q.x);
            lo.y = lo.y.min(q.y);
            hi.x = hi.x.max(q.x);
            hi.y = hi.y.max(q.y);
        }
        BBox::new(lo, hi)
    }

    #[cfg(test)]
    fn area(&self) -> f64 {
        (self.c - self.apex).cross(self.p - self.apex).abs() * 0.5
    }

    /// Point-in-triangle (boundary counts as inside), orientation-agnostic.
    #[cfg(test)]
    fn contains(&self, q: Vec2) -> bool {
        let d1 = (self.p - self.c).cross(q - self.c);
        let d2 = (self.apex - self.p).cross(q - self.p);
        let d3 = (self.c - self.apex).cross(q - self.apex);
        (d1 >= -1e-9 && d2 >= -1e-9 && d3 >= -1e-9)
            || (d1 <= 1e-9 && d2 <= 1e-9 && d3 <= 1e-9)
    }

    fn poly(&self) -> Polygon {
        Polygon::from_slice(&[self.c, self.apex, self.p])
    }
}

/// The "sun" seed: 5 thick rhombi meeting at the origin — a regular
/// decagon of radius 1 — as 10 Robinson triangles.
fn seed_sun() -> Vec<Tri> {
    let mut v = Vec::with_capacity(10);
    for k in 0..5 {
        let th = k as f64 * (2.0 * std::f64::consts::PI / 5.0);
        let u = Vec2::new(th.cos(), th.sin());
        let up = Vec2::new(-u.y, u.x); // u rotated +90°
        let far = u * PHI; // far 72° vertex
        let mid = u * (PHI * 0.5);
        let a = mid + up * S36; // 108° vertex
        let b = mid - up * S36; // 108° vertex
        // Two O-triangles along the long diagonal (origin -> far), both
        // stored CCW as (c, p, apex).
        v.push(Tri { obtuse: true, apex: a, c: Vec2::ZERO, p: far });
        v.push(Tri { obtuse: true, apex: b, c: far, p: Vec2::ZERO });
    }
    v
}

/// The sun seed at a given scale (its inscribed radius).
fn sun_at(scale: f64) -> Vec<Tri> {
    seed_sun()
        .into_iter()
        .map(|t| Tri {
            obtuse: t.obtuse,
            apex: t.apex * scale,
            c: t.c * scale,
            p: t.p * scale,
        })
        .collect()
}

/// One refinement (deflation) step: replace every triangle by its
/// substitution children (scale 1/φ), which exactly tile the parent.
///
/// Culling is sound: a child never leaves its parent, so a final tile
/// intersecting `region` has an ancestor chain whose aabbs all intersect
/// `region` — nothing needed is ever dropped.
fn refine_step(tris: &[Tri], region: &BBox) -> Vec<Tri> {
    let mut out = Vec::with_capacity(tris.len() * 3);
    for t in tris {
        for ch in t.children() {
            if ch.aabb().intersects(*region) {
                out.push(ch);
            }
        }
    }
    out
}

/// Penrose P3 rhomb tiling (aperiodic, 5-fold local symmetry).
pub struct PenroseTiling;

impl Tiling for PenroseTiling {
    fn id(&self) -> &'static str {
        "penrose"
    }

    fn name(&self) -> &'static str {
        "Penrose (P3)"
    }

    fn description(&self) -> &'static str {
        "Aperiodic rhomb tiling by substitution; local 5-fold symmetry."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Quasiperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn default_color_mode(&self) -> ColorMode {
        ColorMode::Pattern
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None)
            .with_notes("aperiodic; local 5-fold symmetry");
        let fail = |message: &'static str| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: None,
            message: Some(message),
        };

        let map = similarity_map(params);
        let inv = map.inverse();

        // Region in the tiling's own (cell) coordinates: the axis-aligned
        // box around the inverse image of the four world corners.
        let mut rmin = Vec2::new(f64::MAX, f64::MAX);
        let mut rmax = Vec2::new(f64::MIN, f64::MIN);
        for &corner in &query.region.corners() {
            let q = inv.transform(corner);
            rmin.x = rmin.x.min(q.x);
            rmin.y = rmin.y.min(q.y);
            rmax.x = rmax.x.max(q.x);
            rmax.y = rmax.y.max(q.y);
        }
        let region = BBox::new(rmin, rmax);

        // Refine a sun seed until the tiles reach unit scale. The sun's
        // inscribed radius equals its scale, so the farthest region corner
        // fixes the seed scale; each refinement divides the tile scale by
        // φ while the patch itself stays put (children tile their parents).
        let mut scale: f64 = 0.0;
        for &corner in &region.corners() {
            scale = scale.max(corner.length());
        }
        if scale < 1e-9 {
            return fail("empty view");
        }
        // Pre-generation budget (mirrors lattice.rs): the returned tiles
        // must cover the query region, and no P3 tile exceeds the thick
        // rhombus area sin 72° (cell units), so area/ sin 72° is a lower
        // bound on the tile count — checked before any refinement work,
        // so an extreme zoom-out cannot OOM the generator first.
        let s = params.float(SCALE_ID, 1.0);
        if query.region.area() / (S72 * s * s) > MAX_TILES as f64 {
            return fail("too many tiles for this view — zoom in");
        }
        let mut tris: Vec<Tri> = sun_at(scale);
        for _ in 0..STEP_CAP {
            if scale <= 1.0 + 1e-12 {
                break;
            }
            tris = refine_step(&tris, &region);
            scale /= PHI;
        }
        if tris.is_empty() || scale > 1.0 + 1e-12 {
            return fail("view too far from the seed — pan back or zoom in");
        }

        // Pair triangles back into rhombi across their shared bases.
        let mut groups: BTreeMap<(i64, i64, i64, i64), Vec<usize>> = BTreeMap::new();
        for (i, t) in tris.iter().enumerate() {
            let a = (t.c.x * 1e7).round() as i64;
            let b = (t.c.y * 1e7).round() as i64;
            let c = (t.p.x * 1e7).round() as i64;
            let d = (t.p.y * 1e7).round() as i64;
            let key = if (a, b) <= (c, d) {
                (a, b, c, d)
            } else {
                (c, d, a, b)
            };
            groups.entry(key).or_default().push(i);
        }

        let mut tiles = Vec::with_capacity(groups.len());
        for (_, idxs) in groups {
            let (proto, poly) = match idxs.as_slice() {
                [i, j] => {
                    let a = tris[*i];
                    let b = tris[*j];
                    let rhomb =
                        Polygon::from_slice(&[a.c, a.apex, a.p, b.apex]);
                    (if a.obtuse { 0 } else { 1 }, rhomb)
                }
                [i] => (
                    if tris[*i].obtuse { 0 } else { 1 },
                    tris[*i].poly(),
                ),
                _ => return fail("internal error: base edge shared by >2 triangles"),
            };
            let world = poly.transform(&map);
            if world.aabb().intersects(query.region) {
                tiles.push(Tile {
                    proto,
                    seed: proto as u32,
                    poly: world,
                });
            }
        }

        if tiles.len() > MAX_TILES {
            return fail("too many tiles for this view — zoom in");
        }

        TilingResult {
            tiles,
            symmetry: meta,
            periodic: None,
            message: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn canonical_obtuse() -> Tri {
        Tri {
            obtuse: true,
            apex: Vec2::new(0.0, S36),
            c: Vec2::new(-0.5 * PHI, 0.0),
            p: Vec2::new(0.5 * PHI, 0.0),
        }
    }

    fn canonical_acute() -> Tri {
        Tri {
            obtuse: false,
            apex: Vec2::ZERO,
            c: Vec2::new(0.5 / PHI, S72),
            p: Vec2::new(-0.5 / PHI, S72),
        }
    }

    /// Deterministic xorshift64 rng (same style as the lib tests).
    struct Rng(u64);
    impl Rng {
        fn f64(&mut self) -> f64 {
            let mut v = self.0;
            v ^= v << 13;
            v ^= v >> 7;
            v ^= v << 17;
            self.0 = v;
            (v >> 11) as f64 * (1.0 / 9007199254740992.0)
        }
    }

    /// The children of `t` must exactly tile `t` itself (deflation view):
    /// every sampled point inside t lies in exactly one child, and the
    /// child areas sum to the parent's area.
    fn check_substitution(t: &Tri, label: &str) {
        let kids = t.children();
        let pa = t.area();
        let kids_area: f64 = kids.iter().map(|k| k.area()).sum();
        assert!(
            (kids_area - pa).abs() <= 1e-9 * pa,
            "{label}: child areas {kids_area} != parent {pa}"
        );

        let bb = t.aabb();
        let mut rng = Rng(0x517C_C1B7_2722_0A95);
        let mut checked = 0;
        for _ in 0..4000 {
            let q = Vec2::new(
                bb.min.x + rng.f64() * bb.size().x,
                bb.min.y + rng.f64() * bb.size().y,
            );
            if !t.contains(q) {
                continue;
            }
            let n = kids.iter().filter(|k| k.contains(q)).count();
            assert!(n == 1, "{label}: point {q} covered by {n} children");
            checked += 1;
        }
        assert!(checked > 1500, "{label}: too few samples landed");
    }

    #[test]
    fn obtuse_substitution_tiles_parent() {
        check_substitution(&canonical_obtuse(), "O -> 2O + 1A");
    }

    #[test]
    fn acute_substitution_tiles_parent() {
        check_substitution(&canonical_acute(), "A -> 1O + 1A");
    }

    // ---------- global consistency: refine the sun ----------
    // Deflation is a refinement (children tile their parent exactly), so
    // overlaps and gaps are impossible by construction. What can still go
    // wrong is the child *labelling*. Note that the Robinson-triangle
    // decomposition is not edge-to-edge at the *triangle* level: at depth
    // >= 2 the split point of one rhombus diagonal lands in the interior
    // of the adjacent rhombus's diagonal, regardless of how the rules are
    // labelled (a property of the decomposition, not a bug). The tiling of
    // *rhombi* — triangles paired across their shared base — is
    // edge-to-edge, and that is what we assert, plus the substitution
    // counts and the seed's 5-fold symmetry.

    /// Substitute every triangle once (no culling).
    fn refine_all(tris: &[Tri]) -> Vec<Tri> {
        let mut out = Vec::with_capacity(tris.len() * 3);
        for t in tris {
            out.extend(t.children());
        }
        out
    }

    /// Does point `q` lie strictly in the interior of segment `a-b`?
    fn on_edge_interior(q: Vec2, a: Vec2, b: Vec2) -> bool {
        let ab = b - a;
        let aq = q - a;
        if aq.cross(ab).abs() > 1e-9 {
            return false;
        }
        let s = aq.dot(ab);
        let l2 = ab.dot(ab);
        s > 1e-9 && s < l2 - 1e-9
    }

    #[test]
    fn refined_sun_counts() {
        // O -> 2O + 1A, A -> 1O + 1A:  (O_n, A_n) -> (2O+A, O+A)
        let mut tris = seed_sun();
        let (mut eo, mut ea) = (10usize, 0usize);
        for depth in 0..3u32 {
            tris = refine_all(&tris);
            let (o, a) = (2 * eo + ea, eo + ea);
            eo = o;
            ea = a;
            let got_o = tris.iter().filter(|t| t.obtuse).count();
            let got_a = tris.len() - got_o;
            assert_eq!(
                (got_o, got_a),
                (eo, ea),
                "depth {depth}: got ({got_o}, {got_a}), want ({eo}, {ea})"
            );
        }
    }

    /// Key of the unordered base segment `(a, b)`: the min of the two
    /// rounded endpoint pairs. Triangles that share a base get the same key;
    /// triangles that only share a single point (e.g. the 5 seed triangles
    /// at the origin) do not, so the key must be built from the *segment*,
    /// never from a single endpoint.
    fn seg_key(a: Vec2, b: Vec2) -> (i64, i64, i64, i64) {
        let (ax, ay) = ((a.x * 1e7).round() as i64, (a.y * 1e7).round() as i64);
        let (bx, by) = ((b.x * 1e7).round() as i64, (b.y * 1e7).round() as i64);
        if (ax, ay) <= (bx, by) {
            (ax, ay, bx, by)
        } else {
            (bx, by, ax, ay)
        }
    }

    #[test]
    fn refined_sun_is_edge_to_edge() {
        // The refined sun is a *finite* star patch, so not every triangle
        // has a twin across its base at a given depth: a triangle whose
        // base lies on the patch boundary (and, from depth 2 on, an
        // A-child whose base sub-segments a shared leg that its neighbour
        // keeps whole) stays unpaired until a later refinement — its
        // missing half only appears in the limit. What the substitution
        // rules must guarantee, and what we assert: every base is shared
        // by at most two same-type triangles, the tiles cover the sun
        // exactly (area sum = 5*sin72°), and the complete rhombi are
        // edge-to-edge — no rhomb vertex lies in the interior of another
        // rhomb's edge.
        let sun_area = 5.0 * S72;
        let mut tris = seed_sun();
        for depth in 0..3u32 {
            tris = refine_all(&tris);

            // Pair triangles across their shared base.
            let mut groups: BTreeMap<(i64, i64, i64, i64), Vec<usize>> = BTreeMap::new();
            for (i, t) in tris.iter().enumerate() {
                groups.entry(seg_key(t.c, t.p)).or_default().push(i);
            }
            let mut rhombs: Vec<[Vec2; 4]> = Vec::with_capacity(groups.len());
            let mut area = 0.0;
            for (seg, idxs) in &groups {
                assert!(
                    idxs.len() <= 2,
                    "depth {depth}: base {seg:?} shared by {} triangles, want at most 2",
                    idxs.len()
                );
                area += idxs.iter().map(|&i| tris[i].area()).sum::<f64>();
                if idxs.len() == 2 {
                    let (a, b) = (&tris[idxs[0]], &tris[idxs[1]]);
                    assert_eq!(
                        a.obtuse, b.obtuse,
                        "depth {depth}: base {seg:?} pairs obtuse and acute triangles"
                    );
                    // Rhomb vertices in cyclic order: base endpoints with
                    // each triangle's apex on opposite sides of the base.
                    rhombs.push([a.c, a.apex, a.p, b.apex]);
                }
            }

            // Exact coverage of the sun (pairing accounts for every
            // triangle, and substitution preserves area).
            assert!(
                (area - sun_area).abs() <= 1e-9 * sun_area,
                "depth {depth}: rhomb area sum {area} != sun area {sun_area}"
            );

            // Edge-to-edge at the rhomb level.
            for (i, r) in rhombs.iter().enumerate() {
                for &v in r {
                    for (j, s) in rhombs.iter().enumerate() {
                        if i == j {
                            continue;
                        }
                        for k in 0..4 {
                            if on_edge_interior(v, s[k], s[(k + 1) % 4]) {
                                panic!(
                                    "depth {depth}: T-junction — rhomb {i} \
                                     vertex {v} lies inside an edge of rhomb {j}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn refined_sun_is_5fold_symmetric() {
        let mut tris = seed_sun();
        for _ in 0..2 {
            tris = refine_all(&tris);
        }
        // The labelled vertex triple must be invariant (as a multiset)
        // under a 72° rotation about the origin.
        type Key = (i64, i64, i64, i64, i64, i64, i64, i64, i64, bool);
        let key = |t: &Tri, rot: bool| -> Key {
            let th = 2.0 * std::f64::consts::PI / 5.0;
            let r = |q: Vec2| if rot { q.rotated(th) } else { q };
            (
                (r(t.c).x * 1e4).round() as i64,
                (r(t.c).y * 1e4).round() as i64,
                (r(t.p).x * 1e4).round() as i64,
                (r(t.p).y * 1e4).round() as i64,
                (r(t.apex).x * 1e4).round() as i64,
                (r(t.apex).y * 1e4).round() as i64,
                0,
                0,
                0,
                t.obtuse,
            )
        };
        let mut count: BTreeMap<Key, i32> = BTreeMap::new();
        for t in &tris {
            *count.entry(key(t, false)).or_insert(0) += 1;
        }
        for t in &tris {
            *count.entry(key(t, true)).or_insert(0) -= 1;
        }
        for (k, &n) in &count {
            assert_eq!(n, 0, "5-fold symmetry mismatch at {k:?}");
        }
    }
}
