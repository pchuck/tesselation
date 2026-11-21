//! Pinwheel (Pythagorean) tiling by substitution.
//!
//! The tiling is built from the 1-2-√5 right triangle. Conway's
//! observation: such a triangle can be divided into five isometric copies
//! of itself at 1/√5 scale; repeatedly inflating by √5 (each inflation
//! also twists by θ = arctan(1/2)) yields an aperiodic tiling (Radin,
//! 1991) in which the tiles appear in infinitely many orientations, since
//! θ is not a rational fraction of a turn.
//!
//! The triangles are labelled `(p, q, r)`: `p` is the long-leg end
//! (|p−r| = 2s), `q` the short-leg end (|q−r| = s), `r` the right-angle
//! vertex. Both chiralities occur in the tiling (the rule contains
//! reflected children), so `affine` uses the general rigid map (rotation
//! *or* reflection) that matches all three labelled vertices — the same
//! approach as the Robinson triangles in `penrose.rs`.
//!
//! Like the Robinson-triangle decomposition, the pinwheel subdivision is
//! not edge-to-edge: four children meet at the midpoint of the parent's
//! long leg, and along a pair of triangles sharing a hypotenuse the split
//! points land at the 1/5, 2/5, 3/5, 4/5 points. These T-junctions are
//! inherent to the substitution, not a labelling bug. What the rule does
//! guarantee — and what the tests assert — is the pinwheel *intersection
//! rule*: any two tiles meet only along a segment that is a whole edge of,
//! or half a long leg of, one of them, and every T-junction lies on a long
//! leg.
//!
//! Generation: seed a grid of 2s×s rectangles (each cut along its
//! diagonal into two triangles — a valid pinwheel fragment), then
//! repeatedly refine — each triangle is replaced by its children at 1/√5
//! scale, which tile the parent exactly (deflation view), so no overlaps
//! can ever occur. Tiles are culled to the region at every step (a child
//! never leaves its parent, so nothing needed is ever dropped).
//!
//! The patch is **view-independent**: the seed scale is quantized to
//! powers of √5 (one octave per refinement depth) and measured from the
//! region *extent*, not its position, and the seed grid is aligned to
//! multiples of the cell size through the origin. The emitted tiles are
//! therefore the restriction of one global refined tiling to the query
//! region — panning and resizing only move the culling window, and the
//! pattern changes only when the extent crosses a √5 octave (the seed
//! depth increments and a different global tiling is selected). The
//! refinement always lands exactly on unit scale, so every emitted tile
//! has area = params_scale² regardless of the view.
//!
//! Pattern coloring: the five substitution children play distinct roles
//! (two sit on the parent's hypotenuse, two fill the middle, one caps
//! it), so each tile is seeded with its child slot + 1 (seeds 1..=5);
//! that makes the five-fold substitution motif visible in "pattern"
//! mode. Seed-grid tiles (visible only at extreme zoom-in, depth 0) are
//! seeded 0.

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::TilingCategory;
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

const SQRT5: f64 = 2.236_067_977_499_79; // √5

/// A 1-2-√5 right triangle, labelled `(p, q, r)`: `p` = long-leg end,
/// `q` = short-leg end, `r` = right-angle vertex.
#[derive(Clone, Copy, Debug)]
struct Tri {
    p: Vec2,
    q: Vec2,
    r: Vec2,
}

impl Tri {
    /// Rigid map `z' = M z + t` taking the canonical triangle to this one.
    /// Unlike a pure rotation it also allows a reflection, so it matches
    /// all three labelled vertices (p, q, r) exactly — which is what makes
    /// the substitution work for both chiralities of the triangle.
    ///
    /// Canonical form: p = (0, 0), q = (2, 1), r = (2, 0).
    /// Returns (m00, m01, m10, m11, tx, ty).
    fn affine(&self) -> [f64; 6] {
        let (p0, q0, r0) = (
            Vec2::ZERO,
            Vec2::new(2.0, 1.0),
            Vec2::new(2.0, 0.0),
        );
        let u0 = q0 - p0; // (2, 1)
        let v0 = r0 - p0; // (2, 0)
        let u = self.q - self.p;
        let v = self.r - self.p;
        let det = u0.x * v0.y - u0.y * v0.x;
        let inv = 1.0 / det;
        let r00 = (u.x * v0.y - v.x * u0.y) * inv;
        let r01 = (v.x * u0.x - u.x * v0.x) * inv;
        let r10 = (u.y * v0.y - v.y * u0.y) * inv;
        let r11 = (v.y * u0.x - u.y * v0.x) * inv;
        let tx = self.p.x - (r00 * p0.x + r01 * p0.y);
        let ty = self.p.y - (r10 * p0.x + r11 * p0.y);
        [r00, r01, r10, r11, tx, ty]
    }

    /// The five substitution children in the canonical parent's
    /// coordinates, at 1/√5 scale. They tile the canonical parent
    /// triangle exactly (the deflation view of Conway's cut, derived in
    /// exact arithmetic from Radin's level-1 decomposition of the
    /// √5-inflated parent). A and B are long legs on the parent's
    /// hypotenuse, E's short leg caps it; C and D share the long leg on
    /// the parent's ... ; verified by unit tests.
    fn children_canonical() -> [Tri; 5] {
        [
            Tri {
                p: Vec2::new(0.0, 0.0),
                q: Vec2::new(1.0, 0.0),
                r: Vec2::new(4.0 / 5.0, 2.0 / 5.0),
            },
            Tri {
                p: Vec2::new(8.0 / 5.0, 4.0 / 5.0),
                q: Vec2::new(1.0, 0.0),
                r: Vec2::new(4.0 / 5.0, 2.0 / 5.0),
            },
            Tri {
                p: Vec2::new(1.0, 0.0),
                q: Vec2::new(8.0 / 5.0, 4.0 / 5.0),
                r: Vec2::new(9.0 / 5.0, 2.0 / 5.0),
            },
            Tri {
                p: Vec2::new(1.0, 0.0),
                q: Vec2::new(2.0, 0.0),
                r: Vec2::new(9.0 / 5.0, 2.0 / 5.0),
            },
            Tri {
                p: Vec2::new(2.0, 0.0),
                q: Vec2::new(2.0, 1.0),
                r: Vec2::new(8.0 / 5.0, 4.0 / 5.0),
            },
        ]
    }

    /// Substitution children in this triangle's own coordinates, one
    /// generation down (at 1/√5 of this triangle's scale). They tile this
    /// triangle exactly: the canonical children tile the canonical
    /// parent, and the affine frame map is an isometry.
    fn children(&self) -> [Tri; 5] {
        let m = self.affine();
        let f = |w: Vec2| {
            Vec2::new(
                m[0] * w.x + m[1] * w.y + m[4],
                m[2] * w.x + m[3] * w.y + m[5],
            )
        };
        let mut kids = Self::children_canonical();
        for k in kids.iter_mut() {
            k.p = f(k.p);
            k.q = f(k.q);
            k.r = f(k.r);
        }
        kids
    }

    fn aabb(&self) -> BBox {
        let mut lo = self.p;
        let mut hi = self.p;
        for &w in &[self.q, self.r] {
            lo.x = lo.x.min(w.x);
            lo.y = lo.y.min(w.y);
            hi.x = hi.x.max(w.x);
            hi.y = hi.y.max(w.y);
        }
        BBox::new(lo, hi)
    }

    /// The three edges in fixed order: (0) long leg `p-r`, (1) short leg
    /// `r-q`, (2) hypotenuse `p-q`.
    #[cfg(test)]
    fn edges(&self) -> [(Vec2, Vec2); 3] {
        [(self.p, self.r), (self.r, self.q), (self.p, self.q)]
    }

    #[cfg(test)]
    fn area(&self) -> f64 {
        (self.q - self.p).cross(self.r - self.p).abs() * 0.5
    }

    /// Point-in-triangle (boundary counts as inside), orientation-agnostic.
    #[cfg(test)]
    fn contains(&self, w: Vec2) -> bool {
        let d1 = (self.q - self.p).cross(w - self.p);
        let d2 = (self.r - self.q).cross(w - self.q);
        let d3 = (self.p - self.r).cross(w - self.r);
        (d1 >= -1e-9 && d2 >= -1e-9 && d3 >= -1e-9)
            || (d1 <= 1e-9 && d2 <= 1e-9 && d3 <= 1e-9)
    }

    fn poly(&self) -> Polygon {
        Polygon::from_slice(&[self.p, self.q, self.r])
    }
}

/// One seed rectangle 2s × s with lower-left corner (x, y), cut along its
/// (x, y) → (x+2s, y+s) diagonal into two triangles. The pair is a valid
/// pinwheel fragment: they share a hypotenuse, and the substitution keeps
/// every boundary of the patch consistent at all depths.
fn rect_pair(x: f64, y: f64, s: f64) -> [Tri; 2] {
    [
        Tri {
            p: Vec2::new(x, y),
            q: Vec2::new(x + 2.0 * s, y + s),
            r: Vec2::new(x + 2.0 * s, y),
        },
        Tri {
            p: Vec2::new(x + 2.0 * s, y + s),
            q: Vec2::new(x, y),
            r: Vec2::new(x, y + s),
        },
    ]
}

/// A grid of seed rectangles covering `box`, each of size 2s × s.
fn seed_grid(box_: &BBox, s: f64) -> Vec<Tri> {
    let x0 = (box_.min.x / (2.0 * s)).floor() * (2.0 * s);
    let x1 = (box_.max.x / (2.0 * s)).ceil() * (2.0 * s);
    let y0 = (box_.min.y / s).floor() * s;
    let y1 = (box_.max.y / s).ceil() * s;
    let nx = ((x1 - x0) / (2.0 * s)).round() as i64;
    let ny = ((y1 - y0) / s).round() as i64;
    let mut v = Vec::with_capacity(2 * nx as usize * ny as usize);
    for j in 0..ny {
        for i in 0..nx {
            let pair = rect_pair(x0 + 2.0 * i as f64 * s, y0 + j as f64 * s, s);
            v.push(pair[0]);
            v.push(pair[1]);
        }
    }
    v
}

/// One refinement (deflation) step: replace every triangle by its
/// substitution children (scale 1/√5), which exactly tile the parent.
/// Each output carries the child's slot index + 1 (1..=5) as its pattern
/// seed — the five canonical children have distinct roles, and the seed
/// makes that structure visible. The input seeds are ignored: refinement
/// always re-derives the seed from the slot, so the coloring reflects the
/// last substitution step only.
///
/// Culling is sound: a child never leaves its parent, so a final tile
/// intersecting `region` has an ancestor chain whose aabbs all intersect
/// `region` — nothing needed is ever dropped.
fn refine_step(tris: &[(Tri, u8)], region: &BBox) -> Vec<(Tri, u8)> {
    let mut out = Vec::with_capacity(tris.len() * 5);
    for (t, _) in tris {
        for (slot, ch) in t.children().into_iter().enumerate() {
            if ch.aabb().intersects(*region) {
                out.push((ch, slot as u8 + 1));
            }
        }
    }
    out
}

/// Pinwheel tiling (aperiodic, infinitely many tile orientations).
pub struct PinwheelTiling;

impl Tiling for PinwheelTiling {
    fn id(&self) -> &'static str {
        "pinwheel"
    }

    fn name(&self) -> &'static str {
        "Pinwheel (1-2-√5)"
    }

    fn description(&self) -> &'static str {
        "Aperiodic Pythagorean-triangle tiling (Conway–Radin pinwheel); tiles in infinitely many orientations."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Aperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None)
            .with_notes("aperiodic; tiles in infinitely many orientations");
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
        if region.area() <= 1e-12 {
            return fail("empty view");
        }
        // Final tiles have area exactly 1 in cell space (the octave-
        // anchored refinement always lands on unit scale; the cell-space
        // AABB of a rotated world box is at most ~2× the world area), so
        // this bounds the tile count before spending time on refinement.
        if region.area() > MAX_TILES as f64 / 5.0 {
            return fail("too many tiles for this view — zoom in");
        }

        // View-independent patch: quantize the seed scale to √5 octaves
        // measured from the region *extent* (not its position — panning
        // must not change the octave), and align the seed grid to
        // multiples of the cell size through the origin (`seed_grid`
        // floors/ceils the region onto that global grid). The emitted
        // tiles are then the restriction of one global refined tiling to
        // the query region: panning and resizing only move the culling
        // window over a fixed pattern; the pattern itself changes only
        // when the extent crosses a √5 octave. Refining exactly `depth`
        // times lands the final tiles on unit scale (√5^depth / √5^depth).
        let extent = (rmax - rmin).length();
        if !extent.is_finite() || extent <= 1e-9 {
            return fail("empty view");
        }
        let depth = (extent.log(SQRT5)).ceil().max(0.0) as usize;
        let seed_scale = SQRT5.powi(depth as i32);

        let mut tris: Vec<(Tri, u8)> =
            seed_grid(&region, seed_scale).into_iter().map(|t| (t, 0)).collect();
        for _ in 0..depth {
            tris = refine_step(&tris, &region);
        }
        if tris.is_empty() {
            return fail("empty view");
        }

        // Emit the triangles as tiles. The tiling is monohedral (one
        // prototile), so `proto` is always 0; `seed` is the tile's
        // substitution slot + 1 (1..=5) — see the module docs — which
        // makes the five-fold substitution structure visible in "pattern"
        // mode. (Seed-grid tiles, visible only at extreme zoom-in, carry
        // seed 0.)
        let mut tiles = Vec::with_capacity(tris.len());
        for (t, slot) in &tris {
            let poly = t.poly().transform(&map);
            if poly.aabb().intersects(query.region) {
                tiles.push(Tile {
                    proto: 0,
                    seed: *slot as u32,
                    poly,
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
    use tess_core::geometry::Affine;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    fn canonical() -> Tri {
        Tri {
            p: Vec2::ZERO,
            q: Vec2::new(2.0, 1.0),
            r: Vec2::new(2.0, 0.0),
        }
    }

    /// The 2-triangle seed: a unit-scale 2×1 rectangle, centered.
    fn seed() -> Vec<Tri> {
        rect_pair(-1.0, -0.5, 1.0).to_vec()
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

    /// Substitute every triangle once (no culling).
    fn refine_all(tris: &[Tri]) -> Vec<Tri> {
        let mut out = Vec::with_capacity(tris.len() * 5);
        for t in tris {
            out.extend(t.children());
        }
        out
    }

    /// The children of `t` must exactly tile `t` itself (deflation view):
    /// every sampled point inside t lies in exactly one child, and the
    /// child areas sum to the parent's area.
    fn check_substitution(t: &Tri, label: &str) {
        let kids = t.children();
        assert_eq!(kids.len(), 5, "{label}: want 5 children");
        let pa = t.area();
        let ka: f64 = kids.iter().map(|k| k.area()).sum();
        assert!(
            (ka - pa).abs() <= 1e-9 * pa,
            "{label}: child areas {ka} != parent {pa}"
        );

        let bb = t.aabb();
        let mut rng = Rng(0x517C_C1B7_2722_0A95);
        let mut checked = 0;
        for _ in 0..6000 {
            let w = Vec2::new(
                bb.min.x + rng.f64() * bb.size().x,
                bb.min.y + rng.f64() * bb.size().y,
            );
            if !t.contains(w) {
                continue;
            }
            let n = kids.iter().filter(|k| k.contains(w)).count();
            assert!(n == 1, "{label}: point {w} covered by {n} children");
            checked += 1;
        }
        // A rotated 1-2-√5 triangle covers at least ~31% of its aabb, so
        // 6000 samples must land well inside; 20% is a wide margin.
        assert!(checked > 1200, "{label}: too few samples landed ({checked})");
    }

    #[test]
    fn substitution_tiles_parent() {
        check_substitution(&canonical(), "canonical");

        // A rotated instance and a reflected instance: the affine frame
        // map must handle both chiralities of the (achiral) triangle.
        let c = canonical();
        let m = Affine::rotation(1.23).then(Affine::translation(Vec2::new(-1.7, 0.9)));
        let rotated = Tri {
            p: m.transform(c.p),
            q: m.transform(c.q),
            r: m.transform(c.r),
        };
        check_substitution(&rotated, "rotated");

        // Mirror the canonical triangle about the x-axis (labels stay
        // attached to their geometric roles), then rotate.
        let flip = |w: Vec2| Vec2::new(w.x, -w.y);
        let mirrored = Tri {
            p: m.transform(flip(c.p)),
            q: m.transform(flip(c.q)),
            r: m.transform(flip(c.r)),
        };
        check_substitution(&mirrored, "mirrored");
    }

    #[test]
    fn refined_seed_counts_and_area() {
        let mut tris = seed();
        for depth in 1..=4u32 {
            tris = refine_all(&tris);
            assert_eq!(
                tris.len(),
                2 * 5usize.pow(depth),
                "depth {depth}: want 2*5^depth tiles"
            );
            let total: f64 = tris.iter().map(|t| t.area()).sum();
            assert!(
                (total - 2.0).abs() < 1e-9,
                "depth {depth}: total area {total} != 2"
            );
        }
    }

    #[test]
    fn refined_seed_edge_lengths() {
        let mut tris = seed();
        for depth in 1..=4u32 {
            tris = refine_all(&tris);
            let s = 5f64.powf(-(depth as f64) / 2.0);
            let allowed = [1.0 * s, 2.0 * s, SQRT5 * s];
            for t in &tris {
                for &(a, b) in &t.edges() {
                    let l = (b - a).length();
                    assert!(
                        allowed.iter().any(|&x| (l - x).abs() < 1e-9),
                        "depth {depth}: edge length {l} not in {allowed:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn refined_seed_coverage() {
        // Every point of the seed rectangle lies in exactly one tile —
        // catches both gaps and overlaps at every depth.
        let mut tris = seed();
        let mut rng = Rng(0xC0FF_EE00_1111_2222);
        for depth in 1..=3u32 {
            tris = refine_all(&tris);
            let bad = (0..20000)
                .filter(|_| {
                    let w = Vec2::new(rng.f64() * 2.0 - 1.0, rng.f64() - 0.5);
                    tris.iter().filter(|t| t.contains(w)).count() != 1
                })
                .count();
            assert_eq!(bad, 0, "depth {depth}: {bad} mis-covered samples");
        }
    }

    /// The pinwheel's signature property: tile orientations never close
    /// up. The number of distinct long-leg directions (mod π) must keep
    /// growing as the substitution deepens (in the limit it is infinite).
    #[test]
    fn orientations_grow_without_bound() {
        let mut tris = seed();
        let mut prev = 0usize;
        for depth in 1..=5u32 {
            tris = refine_all(&tris);
            let dirs: std::collections::BTreeSet<i64> = tris
                .iter()
                .map(|t| {
                    let d = t.r - t.p;
                    let a = d.y.atan2(d.x).rem_euclid(std::f64::consts::PI);
                    (a * 1e6).round() as i64
                })
                .collect();
            assert!(
                dirs.len() > prev,
                "depth {depth}: orientation count stopped growing"
            );
            prev = dirs.len();
        }
        assert!(prev >= 8, "expected >= 8 orientations by depth 5, got {prev}");
    }

    /// Length of the collinear overlap of segments [a, b] and [c, d], or
    /// None when they are not collinear or touch only at a point.
    fn overlap_len(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> Option<f64> {
        let ab = b - a;
        let l2 = ab.dot(ab);
        if l2 < 1e-18 {
            return None;
        }
        let tol = 1e-9 * l2.sqrt();
        if ((c - a).cross(ab)).abs() > tol || ((d - a).cross(ab)).abs() > tol {
            return None;
        }
        let proj = |w: Vec2| (w - a).dot(ab) / l2;
        let lo = 0.0f64.max(proj(c).min(proj(d)));
        let hi = 1.0f64.min(proj(c).max(proj(d)));
        if hi - lo <= 1e-9 {
            return None;
        }
        Some((hi - lo) * l2.sqrt())
    }

    /// The pinwheel intersection rule: any two tiles meet only along a
    /// segment that is a whole edge of — or half a long leg of — one of
    /// them. This is the tiling's "half-edge-to-edge" property; in
    /// particular it rules out arbitrary T-junctions, which are the
    /// failure mode a wrong child placement would produce.
    #[test]
    fn refined_seed_obey_intersection_rule() {
        let mut tris = seed();
        for depth in 1..=3u32 {
            tris = refine_all(&tris);
            let long_leg = 2.0 * 5f64.powf(-(depth as f64) / 2.0);
            for i in 0..tris.len() {
                for j in (i + 1)..tris.len() {
                    for &(a, b) in &tris[i].edges() {
                        let la = (b - a).length();
                        for &(c, d) in &tris[j].edges() {
                            let Some(ov) = overlap_len(a, b, c, d) else {
                                continue;
                            };
                            let lb = (d - c).length();
                            let whole_i = (ov - la).abs() < 1e-9;
                            let whole_j = (ov - lb).abs() < 1e-9;
                            let half_i = (ov - la / 2.0).abs() < 1e-9
                                && (la - long_leg).abs() < 1e-9;
                            let half_j = (ov - lb / 2.0).abs() < 1e-9
                                && (lb - long_leg).abs() < 1e-9;
                            assert!(
                                whole_i || whole_j || half_i || half_j,
                                "depth {depth}: tiles {i} and {j} share a segment of \
                                 length {ov} that is neither a whole edge nor a \
                                 half long leg"
                            );
                        }
                    }
                }
            }
        }
    }

    /// T-junctions (a vertex of one tile strictly inside an edge of
    /// another) are inherent to the pinwheel subdivision — four children
    /// meet at the midpoint of the parent's long leg, and hypotenuse
    /// pairs split at 1/5..4/5. Assert they occur only on long legs, so
    /// the junction structure is exactly the known one.
    #[test]
    fn t_junctions_only_on_long_legs() {
        let mut tris = seed();
        for depth in 1..=3u32 {
            tris = refine_all(&tris);
            for (i, t) in tris.iter().enumerate() {
                for v in [t.p, t.q, t.r] {
                    for (j, u) in tris.iter().enumerate() {
                        if i == j {
                            continue;
                        }
                        for (k, &(a, b)) in u.edges().iter().enumerate() {
                            let ab = b - a;
                            let l2 = ab.dot(ab);
                            if ((v - a).cross(ab)).abs() > 1e-9 * l2.sqrt() {
                                continue;
                            }
                            let s = (v - a).dot(ab) / l2;
                            if s > 1e-9 && s < 1.0 - 1e-9 {
                                assert_eq!(
                                    k, 0,
                                    "depth {depth}: T-junction on edge {k} (want only \
                                     long legs): vertex {v} of tile {i} lies inside an \
                                     edge of tile {j}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    /// Bit-exact tile identity: same seed and identical vertex bits. The
    /// view-stability tests below rely on this — the emitted patch must
    /// be the *same global tiling* restricted to the query region, so a
    /// tile visible in two views must be computed through the identical
    /// refinement chain and come out bit-for-bit equal.
    fn tile_key(tile: &Tile) -> (u32, Vec<u64>) {
        let mut bits = Vec::with_capacity(tile.poly.pts.len() * 2);
        for p in &tile.poly.pts {
            bits.push(p.x.to_bits());
            bits.push(p.y.to_bits());
        }
        (tile.seed, bits)
    }

    /// The patch is view-independent within a √5 octave: growing, panning,
    /// or zooming the view only moves the culling window over one fixed,
    /// origin-anchored refined tiling. Every tile visible in the smaller
    /// view must reappear bit-identically in the other views. (The
    /// region-anchored seeding this replaced re-derived the whole patch
    /// from the query box, so the GUI re-tiled the world on every
    /// resize/zoom frame — the pattern jumped around, reading as flicker,
    /// with boundary tiles popping in and out.)
    #[test]
    fn patch_is_stable_under_view_changes_within_an_octave() {
        let t = PinwheelTiling;
        let p = ParamSet::default_for(t.params());
        // All four regions have extents inside [√5³, √5⁴] ≈ [11.2, 25],
        // so they select the same octave (the same global tiling).
        let base = t.generate(
            &p,
            &Query { region: BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)) },
        );
        assert!(base.message.is_none(), "{:?}", base.message);
        let grown = t.generate(
            &p,
            &Query { region: BBox::new(v2(-8.4, -6.3), v2(8.4, 6.3)) },
        );
        let panned = t.generate(
            &p,
            &Query { region: BBox::new(v2(-7.5, -5.5), v2(8.5, 6.5)) },
        );
        let zoomed = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-8.0 / 1.3, -6.0 / 1.3), v2(8.0 / 1.3, 6.0 / 1.3)),
            },
        );

        let base_keys: std::collections::HashSet<(u32, Vec<u64>)> =
            base.tiles.iter().map(tile_key).collect();
        let zoomed_keys: std::collections::HashSet<(u32, Vec<u64>)> =
            zoomed.tiles.iter().map(tile_key).collect();

        // grow: the base view is a subset of the grown view.
        let grown_keys: std::collections::HashSet<(u32, Vec<u64>)> =
            grown.tiles.iter().map(tile_key).collect();
        for k in &base_keys {
            assert!(grown_keys.contains(k), "grow: tile changed when the view grew");
        }
        // zoom in: the zoomed view is a subset of the base view.
        for k in &zoomed_keys {
            assert!(base_keys.contains(k), "zoom: tile changed when the view changed");
        }
        // pan: the window shifted by (0.5, 0.5); the two views overlap but
        // neither contains the other. Generate the overlap as its own view
        // (same extent → same octave) and require its tiles to appear
        // bit-identically in both the base and the panned view.
        let overlap = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-7.45, -5.45), v2(7.95, 5.95)),
            },
        );
        let panned_keys: std::collections::HashSet<(u32, Vec<u64>)> =
            panned.tiles.iter().map(tile_key).collect();
        for tile in &overlap.tiles {
            let k = tile_key(tile);
            assert!(base_keys.contains(&k), "pan: overlap tile missing/changed in base view");
            assert!(panned_keys.contains(&k), "pan: overlap tile missing/changed in panned view");
        }
        assert!(!overlap.tiles.is_empty(), "pan: overlap view produced no tiles");
    }

    /// The octave-anchored refinement always lands exactly on unit scale:
    /// every emitted tile has area = params_scale² (1 at default params).
    /// The previous loop stopped at whatever scale it reached (1/√5..1],
    /// so tile sizes — and the visible density — drifted with the view.
    #[test]
    fn emitted_tiles_are_unit_scale() {
        let t = PinwheelTiling;
        let p = ParamSet::default_for(t.params());
        let r = t.generate(
            &p,
            &Query { region: BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)) },
        );
        assert!(r.message.is_none(), "{:?}", r.message);
        assert!(!r.tiles.is_empty());
        for tile in &r.tiles {
            let a = tile.poly.area();
            assert!(
                (a - 1.0).abs() < 1e-9,
                "tile area {a} != 1 (unit 1-2-√5 triangle at default params)"
            );
        }
    }
}
