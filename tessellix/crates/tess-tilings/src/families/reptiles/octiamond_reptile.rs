//! Octiamond reptile — the trapezoidal 8-triomino rep-tile substitution (λ = 3).
//!
//! The prototile is the *octiamond* `Q`: a trapezoidal 8-triomino of the
//! triangular lattice (eight unit triangles; lattice corners (0,0), (3,0),
//! (1,2), (0,2); area 2√3). It is a rep-9 rep-tile: a `Q` of triangle side
//! `s` is dissected into nine `Q`s of side `s/3`. The rule was pinned by an
//! exhaustive exact-integer enumeration: partitioning the 3×-inflated
//! 72-cell trapezoid into nine side-1 octiamonds admits exactly ONE
//! dissection over all 12 lattice orientations (backtracking over the nine
//! pinned anchors with exact cell-set masks), cross-confirmed per-component
//! against the Bielefeld rule image (pixel scores 0.911–0.991, runners-up
//! ≤ 0.892; the blue-marked child is the identity child I+(2,2) =
//! `CHILDREN[5]`).
//!
//! Geometry is exact on the triangular lattice. A lattice point is a
//! pair `(q, r)`; world position is `q·e1 + r·e2` with `e1 = (1, 0)`,
//! `e2 = (1/2, √3/2)`. The lattice has 12 symmetries (rotations by
//! 60°·k and their reflections), tabulated in [`SYMS`] as integer
//! matrices. `Q` is asymmetric, and its unique dissection mixes
//! chiralities (one mirrored child among the nine), so tiles come in
//! all **12 orientation classes**; the parent's symmetry conjugates the
//! canonical rule (child class = parent∘child, child anchor = parent
//! applied to the λ-scaled canonical anchor) — verified for all 12
//! parent classes by exact cell census.
//!
//! The tiling is **non-edge-to-edge**, and the census below is MEASURED
//! with the same machinery that reproduces the sphinx battery's pinned
//! values (0, 2, 16, 78, 344) exactly: the rule's own dissection of the
//! scale-3 supertile carries **8** (edge, foreign corner) incidences
//! among its nine children (each a vertex where two children meet with
//! their shared edge ending strictly inside a third child's edge — e.g.
//! the vertex (4,2), a corner of R180+(4,2) and I+(4,0), lies strictly
//! inside the identity child's bottom edge (5,2)→(2,2)). The seed-patch
//! census over the generations is 18, 226, 2202, 20286, 183974. (The
//! research brief pinned gen-0 = 0 from a recon measurement whose
//! unit-step boundary walk cannot see multi-unit edges at all — under
//! that walk even the sphinx's pinned gen-1 count of 2 reads 0; the
//! measured values above supersede it.) What the construction
//! guarantees — and the tests assert, exactly, on the integer grid — is
//! that every generation is a *perfect* tiling: every unit triangle of
//! the patch is covered by exactly one octiamond, counts follow N·9^g,
//! class counts follow the substitution matrix (rows and columns both
//! sum to 9), the total area never changes, and every refined edge
//! segment (edges split at all other tiles' vertices) is shared by
//! exactly two octiamonds or lies on the patch boundary.
//!
//! Emission follows the repo's finite-depth approximant convention (see
//! `sphinx.rs` / `chair.rs`): the plane is seeded with parallelogram
//! blocks — each a 12×6 lattice parallelogram holding 18 side-`3^g`
//! octiamonds (the one exact tiling of that block: the nine pinned
//! children of the canonical scale-3 supertile plus their 180°-about-
//! (6,3) images) — on a block lattice anchored through the origin
//! (block (i, j) at lattice `(12i, 6j)·3^g`), and every octiamond is
//! deflated exactly `g` times, so the final octiamonds land on unit
//! scale (area 2√3) regardless of the view. The seed scale is quantized
//! to powers of 3 measured from the region *extent*, not its position,
//! so panning and zooming within one tri-section only move the culling
//! window over one fixed global tiling (bit-identical tiles); crossing
//! a tri-section selects a finer global tiling with a tripled period —
//! the finite-depth shadow of the substitution's hierarchical structure.
//!
//! Pattern coloring: the tiling is monohedral, so `proto` is always 0;
//! `seed` is the orientation class + 1 (1..=12), which makes the
//! classes visible in "pattern" mode.
//!
//! Verification battery (exact, stdlib-only Python pre-check plus this
//! module's tests): triangular-lattice group machinery (12 symmetries,
//! each verified to permute the six unit directions); uniqueness of the
//! pinned dissection (exhaustive backtracking); conjugation check for
//! all 12 parent orientations; L2 nesting audit under the λ-scaled
//! framing (0 overlaps, 648/648, 81 distinct grandchildren — the
//! framing cross-validated on the sphinx λ = 2 rule, where the unscaled
//! variant is measurably defective); seed block (144 cells, 0 missing /
//! 0 dups over a 48×36 window); a 5-generation growth battery asserting
//! the perfect unit-cell census of the invariant 972×486-cell
//! parallelogram, the count/orientation/area laws, and final unit
//! scale; and the measured T-junction census above.

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::TilingCategory;
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

/// √3/2 — the height of the unit triangle (world y of lattice row 1).
const H: f64 = 0.8660254037844386;

/// The 12 symmetries of the triangular lattice as integer matrices,
/// row-major `[a, b, c, d]` acting on lattice points `(q, r)` as
/// `(a·q + b·r, c·q + d·r)`. Closed under composition (asserted by a
/// test); index 9 is the identity; determinants ±1 (the det −1 entries
/// are the reflections). Every entry maps the six unit edge directions
/// to themselves.
const SYMS: [[i64; 4]; 12] = [
    [-1, -1, 0, 1],
    [-1, -1, 1, 0],
    [-1, 0, 0, -1],
    [-1, 0, 1, 1],
    [0, -1, -1, 0],
    [0, -1, 1, 1],
    [0, 1, -1, -1],
    [0, 1, 1, 0],
    [1, 0, -1, -1],
    [1, 0, 0, 1],
    [1, 1, -1, 0],
    [1, 1, 0, -1],
];

/// Test-helper shapes: a unit cell keyed by its sorted vertex triple,
/// and a unit edge keyed by its sorted endpoint pair.
#[cfg(test)]
type UnitCell = ((i64, i64), (i64, i64), (i64, i64));
#[cfg(test)]
type UnitEdge = ((i64, i64), (i64, i64));

/// The canonical octiamond `Q`: its eight unit cells (sorted vertex
/// triples) in the canonical frame (translate-minimized). 2× world area
/// 8·H, i.e. area 2√3.
#[cfg(test)]
const CANONICAL_CELLS: [[[i64; 2]; 3]; 8] = [
    [[0, 0], [0, 1], [1, 0]],
    [[1, 0], [1, 1], [2, 0]],
    [[2, 0], [2, 1], [3, 0]],
    [[0, 1], [1, 0], [1, 1]],
    [[1, 1], [2, 0], [2, 1]],
    [[0, 1], [0, 2], [1, 1]],
    [[1, 1], [1, 2], [2, 1]],
    [[0, 2], [1, 1], [1, 2]],
];

/// The canonical octiamond corner outline (orientation class 9 =
/// identity), CCW in world orientation, four corners. 2× shoelace area
/// 8·H; the boundary sides have lattice lengths 3, 2, 1, 2. Every
/// orientation class is emitted as the image of this outline under its
/// lattice symmetry — reversed when that symmetry mirrors, so the
/// emitted polygon is always CCW — see `Octiamond::corners`.
const CANONICAL_OUTLINE: [[i64; 2]; 4] = [[0, 0], [3, 0], [1, 2], [0, 2]];

/// The pinned dissection: the nine children of the canonical side-3
/// octiamond as (orientation class, anchor q, anchor r) at scale 1 —
/// the UNIQUE exact cover of the 72-cell 3×-inflated trapezoid by nine
/// side-1 octiamonds (exhaustive backtracking over all 12 orientations
/// at the nine anchors; see module docs). Children of a class-`c`
/// parent: class `c∘k`, anchor `parent_pos + (s/3)·c(anchor)` — see
/// `Octiamond::children`. `CHILDREN[5]` = I+(2,2) is the blue-marked
/// child of the Bielefeld rule image (pixel score 0.991).
const CHILDREN: [(u8, i64, i64); 9] = [
    (1, 9, 0),
    (2, 4, 2),
    (4, 2, 6),
    (6, 0, 5),
    (9, 0, 0),
    (9, 2, 2),
    (9, 2, 4),
    (9, 4, 0),
    (10, 3, 4),
];

/// The seed block: 18 side-u octiamonds tiling the 12×6 lattice
/// parallelogram (the one exact tiling of that block): the nine pinned
/// children of the canonical scale-3 supertile, plus each one's image
/// under the 180° rotation about the block center (6, 3) — class
/// `2∘cls`, anchor `(12−q, 6−r)`. The block tiles the plane under the
/// lattice ⟨(12, 0), (0, 6)⟩.
const SEED_TILES: [(u8, i64, i64); 18] = [
    (1, 9, 0),
    (2, 4, 2),
    (4, 2, 6),
    (6, 0, 5),
    (9, 0, 0),
    (9, 2, 2),
    (9, 2, 4),
    (9, 4, 0),
    (10, 3, 4),
    (10, 3, 6),
    (9, 8, 4),
    (7, 10, 0),
    (5, 12, 1),
    (2, 12, 6),
    (2, 10, 4),
    (2, 10, 2),
    (2, 8, 6),
    (1, 9, 2),
];

/// Block lattice: the seed parallelogram is spanned by `12·e1` and
/// `6·e2` in lattice units, so block `(i, j)` has lattice origin
/// `(i·12u, j·6u)` at seed scale `u`.
const BLOCK_Q: i64 = 12;
const BLOCK_R: i64 = 6;

/// View extent (world units) at which the seed tri-section increments.
/// The seed block is 12 lattice units wide and 6·2 = 12 lattice rows
/// tall (world height 6·√3 ≈ 10.39); the unit puts the repo's standard
/// ±8×±6 test view (extent exactly 20) at ratio 4 → depth 2, two
/// refinement levels visible — the sphinx presentation convention.
const EXTENT_UNIT: f64 = 5.0;

fn mat_vec(m: [i64; 4], p: (i64, i64)) -> (i64, i64) {
    (m[0] * p.0 + m[1] * p.1, m[2] * p.0 + m[3] * p.1)
}

fn mat_det(m: [i64; 4]) -> i64 {
    m[0] * m[3] - m[1] * m[2]
}

/// Group composition: the class index of `SYMS[h] · SYMS[k]` (apply `k`
/// first, then `h`) — the product of two tabulated symmetries is
/// tabulated (asserted by a test).
fn compose(h: u8, k: u8) -> u8 {
    let a = SYMS[h as usize];
    let b = SYMS[k as usize];
    let m = [
        a[0] * b[0] + a[1] * b[2],
        a[0] * b[1] + a[1] * b[3],
        a[2] * b[0] + a[3] * b[2],
        a[2] * b[1] + a[3] * b[3],
    ];
    SYMS.iter()
        .position(|&x| x == m)
        .expect("triangular-lattice symmetry group is closed") as u8
}

/// An octiamond on the triangular lattice: orientation class `cls` (one
/// of the 12 lattice symmetries applied to the canonical shape), lattice
/// anchor `(q, r)`, triangle side `s` (a power of three ≥ 1). Every
/// coordinate in the construction is an integer, so refinement and
/// culling are exact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Octiamond {
    cls: u8,
    q: i64,
    r: i64,
    s: i64,
}

impl Octiamond {
    /// The tile's four corners in world coordinates (unit-scale tiling
    /// space): the canonical corner outline transformed by the class
    /// symmetry and the tile's position. Mirrored classes are reversed
    /// so every emitted polygon is CCW.
    fn corners(&self) -> [Vec2; 4] {
        let g = SYMS[self.cls as usize];
        let mut pts = [Vec2::new(0.0, 0.0); 4];
        for (i, p) in CANONICAL_OUTLINE.iter().enumerate() {
            let (lq, lr) = mat_vec(g, (p[0] * self.s, p[1] * self.s));
            let (q, r) = (self.q + lq, self.r + lr);
            pts[i] = Vec2::new(q as f64 + 0.5 * r as f64, H * r as f64);
        }
        if mat_det(g) < 0 {
            pts.reverse();
        }
        pts
    }

    /// Polygon outline (CCW, four corners).
    fn poly(&self) -> Polygon {
        Polygon::from_slice(&self.corners())
    }

    /// The tile's world-space bounding box.
    fn aabb(&self) -> BBox {
        let pts = self.corners();
        let mut min = pts[0];
        let mut max = pts[0];
        for p in &pts[1..] {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
        }
        BBox::new(min, max)
    }

    /// The nine substitution children (triangle side s/3; requires s
    /// divisible by 3). Rule: conjugate the canonical dissection by the
    /// parent's symmetry — child class = parent∘canonical, child anchor
    /// = parent anchor + (s/3)·parent-symmetry(canonical anchor). They
    /// tile the parent exactly (verified by an exact unit-cell census
    /// in the tests, for all twelve classes at two scales).
    fn children(&self) -> [Octiamond; 9] {
        let s = self.s;
        debug_assert!(s % 3 == 0, "children need a side divisible by 3");
        let h = SYMS[self.cls as usize];
        let third = s / 3;
        CHILDREN.map(|(k, aq, ar)| {
            let (tq, tr) = mat_vec(h, (aq * third, ar * third));
            Octiamond {
                cls: compose(self.cls, k),
                q: self.q + tq,
                r: self.r + tr,
                s: third,
            }
        })
    }

    /// The tile's unit cells (test helper): each of the eight side-`s`
    /// triangles of the tile, subdivided into its `s²` unit triangles.
    /// A unit cell is keyed by its sorted vertex triple.
    #[cfg(test)]
    fn cells(&self) -> std::collections::BTreeSet<UnitCell> {
        let g = SYMS[self.cls as usize];
        let s = self.s;
        let mut out = std::collections::BTreeSet::new();
        for c in CANONICAL_CELLS {
            let verts: Vec<(i64, i64)> = c
                .iter()
                .map(|p| {
                    let (lq, lr) = mat_vec(g, (p[0] * s, p[1] * s));
                    (self.q + lq, self.r + lr)
                })
                .collect();
            let (a, b, c2) = (verts[0], verts[1], verts[2]);
            let u = ((b.0 - a.0) / s, (b.1 - a.1) / s);
            let v = ((c2.0 - a.0) / s, (c2.1 - a.1) / s);
            debug_assert_eq!(smul_sq(u), 1, "edge of the side-s triangle");
            debug_assert_eq!(smul_sq(v), 1, "edge of the side-s triangle");
            let p = |i: i64, j: i64| (a.0 + i * u.0 + j * v.0, a.1 + i * u.1 + j * v.1);
            for i in 0..s {
                for j in 0..(s - i) {
                    let mut up = [p(i, j), p(i + 1, j), p(i, j + 1)];
                    up.sort();
                    out.insert((up[0], up[1], up[2]));
                    if i + j <= s - 2 {
                        let mut dn = [p(i + 1, j), p(i, j + 1), p(i + 1, j + 1)];
                        dn.sort();
                        out.insert((dn[0], dn[1], dn[2]));
                    }
                }
            }
        }
        out
    }
}

/// Squared lattice length of `(a, b)`: `a² + ab + b²` (the unit
/// directions are exactly the vectors of squared length 1).
#[cfg(test)]
fn smul_sq(d: (i64, i64)) -> i64 {
    d.0 * d.0 + d.0 * d.1 + d.1 * d.1
}

/// Seed octiamonds: the 12×6 parallelogram blocks (each holding the 18
/// `SEED_TILES` octiamonds at scale `3^g`) on the block lattice anchored
/// through the origin, covering `region`. Block indices are found by
/// projecting the region corners onto the block-lattice basis (with a
/// one-block margin), which is sound: a point in block `(i, j)` has
/// basis coordinates `i + α, j + β` with `α, β ∈ [0, 1]`, so the
/// containing block's indices lie within ±1 of any corner's projection.
fn seed_qtiles(region: &BBox, g: u32) -> Vec<Octiamond> {
    let u = 3i64.pow(g);
    // Block lattice vectors in world coordinates: U = 12u·e1,
    // V = 6u·e2 = (3u, 6u·H).
    let (ux, uy) = (12.0 * u as f64, 0.0);
    let (vx, vy) = (3.0 * u as f64, 6.0 * H * u as f64);
    let det = ux * vy - uy * vx;
    let mut imin = f64::MAX;
    let mut imax = f64::MIN;
    let mut jmin = f64::MAX;
    let mut jmax = f64::MIN;
    for corner in &region.corners() {
        let i = (corner.x * vy - corner.y * vx) / det;
        let j = (ux * corner.y - uy * corner.x) / det;
        imin = imin.min(i);
        imax = imax.max(i);
        jmin = jmin.min(j);
        jmax = jmax.max(j);
    }
    let mut v = Vec::new();
    for j in (jmin.floor() as i64 - 1)..=(jmax.ceil() as i64 + 1) {
        for i in (imin.floor() as i64 - 1)..=(imax.ceil() as i64 + 1) {
            let (bq, br) = (i * BLOCK_Q * u, j * BLOCK_R * u);
            for (cls, aq, ar) in SEED_TILES {
                v.push(Octiamond {
                    cls,
                    q: bq + aq * u,
                    r: br + ar * u,
                    s: u,
                });
            }
        }
    }
    v
}

/// One deflation step: replace every octiamond by its nine children
/// (scale 1/3), keeping only those whose bounding box intersects
/// `region`. Sound: children never leave their parent, so a final tile
/// intersecting the region has its full ancestor chain present —
/// nothing needed is ever dropped.
fn deflate_step(octiamonds: &[Octiamond], region: &BBox) -> Vec<Octiamond> {
    let mut out = Vec::with_capacity(octiamonds.len() * 9);
    for t in octiamonds {
        for kid in t.children() {
            if kid.aabb().intersects(*region) {
                out.push(kid);
            }
        }
    }
    out
}

/// Octiamond reptile tiling (aperiodic rep-9 substitution, λ = 3).
pub struct OctiamondReptileTiling;

impl Tiling for OctiamondReptileTiling {
    fn id(&self) -> &'static str {
        "octiamond-reptile"
    }

    fn name(&self) -> &'static str {
        "Octiamond reptile (trapezoidal 8-triomino)"
    }

    fn description(&self) -> &'static str {
        "Aperiodic rep-9 octiamond reptile (trapezoidal 8-triomino) substitution tiling; λ = 3; tiles in 12 orientations."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Nonperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None)
            .with_notes("aperiodic; rep-9 substitution; tiles in 12 orientations; non-edge-to-edge");
        let fail = |message: &'static str| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: None,
            message: Some(message),
        };

        let map = similarity_map(params);
        let inv = map.inverse();

        // Region in the tiling's own (unit-scale world) coordinates: the
        // axis-aligned box around the inverse image of the four world
        // corners.
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
        // Final octiamonds have area exactly 2√3 ≈ 3.464 in tiling
        // space (eight unit triangles; the tri-section-anchored
        // deflation always lands on unit scale), and the cell-space
        // AABB of a rotated world box is at most ~2× the world area, so
        // this bounds the tile count before any refinement work.
        if region.area() > MAX_TILES as f64 * 2.0 {
            return fail("too many tiles for this view — zoom in");
        }

        // View-independent patch: quantize the seed scale to powers of
        // 3 (one tri-section per deflation depth) measured from the
        // region *extent* (not its position — panning must not change
        // the tri-section), and anchor the parallelogram seed blocks to
        // the block lattice through the origin. The emitted tiles are
        // the restriction of one global refined tiling to the query
        // region: panning and resizing only move the culling window;
        // the pattern changes only when the extent crosses a
        // power-of-three tri-section (the depth increments and a finer
        // global tiling is selected). Deflating exactly `depth` times
        // lands the final octiamonds on unit scale. The cap is pure
        // i64 paranoia (12·3^37 still fits `i64`; 3^38·12 does not) —
        // geometrically the area guard above binds first.
        let extent = (rmax - rmin).length();
        if !extent.is_finite() || extent <= 1e-9 {
            return fail("empty view");
        }
        let depth = ((extent / EXTENT_UNIT).log(3.0).ceil().max(0.0) as u32).min(37);

        let mut octiamonds = seed_qtiles(&region, depth);
        for _ in 0..depth {
            octiamonds = deflate_step(&octiamonds, &region);
        }
        if octiamonds.is_empty() {
            return fail("empty view");
        }

        // Emit the octiamonds as tiles. The tiling is monohedral (one
        // prototile), so `proto` is always 0; `seed` is the orientation
        // class + 1 (1..=12) — see the module docs — which makes the
        // classes visible in "pattern" mode.
        let mut tiles = Vec::with_capacity(octiamonds.len());
        for t in &octiamonds {
            let poly = t.poly().transform(&map);
            if poly.aabb().intersects(query.region) {
                tiles.push(Tile {
                    proto: 0,
                    seed: t.cls as u32 + 1,
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
    use std::collections::{BTreeMap, BTreeSet};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Signed 2× shoelace area of a polygon.
    fn signed_area2(pts: &[Vec2]) -> f64 {
        let n = pts.len();
        let mut s = 0.0;
        for i in 0..n {
            let a = pts[i];
            let b = pts[(i + 1) % n];
            s += a.x * b.y - b.x * a.y;
        }
        s
    }

    /// The tabulated symmetries are exactly the 12-element triangular
    /// lattice group: distinct entries, identity at index 9, each of
    /// determinant ±1, each mapping the six unit edge directions to
    /// themselves, and closed under `compose`.
    #[test]
    fn symmetry_table_is_the_triangular_group() {
        let unit: Vec<(i64, i64)> = vec![(1, 0), (0, 1), (1, -1), (-1, 0), (0, -1), (-1, 1)];
        let unit_set: BTreeSet<(i64, i64)> = unit.iter().copied().collect();
        assert_eq!(SYMS.iter().collect::<BTreeSet<_>>().len(), 12, "distinct");
        assert_eq!(SYMS[9], [1, 0, 0, 1], "identity at index 9");
        for g in SYMS {
            assert_eq!(mat_det(g).abs(), 1, "det {g:?}");
            let imgs: BTreeSet<(i64, i64)> = unit.iter().map(|&p| mat_vec(g, p)).collect();
            assert_eq!(imgs, unit_set, "{g:?} preserves unit directions");
        }
        for h in 0..12u8 {
            for k in 0..12u8 {
                let prod = {
                    let a = SYMS[h as usize];
                    let b = SYMS[k as usize];
                    [
                        a[0] * b[0] + a[1] * b[2],
                        a[0] * b[1] + a[1] * b[3],
                        a[2] * b[0] + a[3] * b[2],
                        a[2] * b[1] + a[3] * b[3],
                    ]
                };
                assert!(
                    SYMS.contains(&prod),
                    "product of {h} and {k} ({prod:?}) not in table"
                );
                assert_eq!(
                    compose(h, k) as usize,
                    SYMS.iter().position(|&x| x == prod).unwrap()
                );
            }
        }
    }

    /// The emitted outline of every orientation class is a CCW octiamond
    /// trapezoid of the exact area (2× area 8·H·s²), and its corner set
    /// matches an INDEPENDENT derivation: the boundary walk of the
    /// tile's own unit cells (corners only — collinear steps merged).
    #[test]
    fn emitted_outlines_are_octiamonds() {
        for cls in 0..12u8 {
            let t = Octiamond {
                cls,
                q: 3,
                r: -2,
                s: 5,
            };
            let pts = t.poly().pts;
            assert_eq!(pts.len(), 4, "cls {cls}: vertex count");
            let a2 = signed_area2(&pts);
            let want = 8.0 * H * 25.0;
            assert!(
                (a2 - want).abs() < 1e-9 * want,
                "cls {cls}: signed 2×area {a2} != {want} (want CCW octiamond)"
            );
            // recover lattice coordinates of the emitted corners
            let got: BTreeSet<(i64, i64)> = pts
                .iter()
                .map(|p| {
                    let r = (p.y / H).round() as i64;
                    let q = (p.x - 0.5 * r as f64).round() as i64;
                    (q, r)
                })
                .collect();
            assert_eq!(
                got,
                boundary_outline(cls, 3, -2, 5),
                "cls {cls}: corner set"
            );
        }
    }

    /// Independent derivation of a tile's corner outline: unit cells of
    /// the tile → boundary unit edges → CCW walk → collinear merge.
    #[cfg(test)]
    fn boundary_outline(cls: u8, q: i64, r: i64, s: i64) -> BTreeSet<(i64, i64)> {
        let t = Octiamond { cls, q, r, s };
        let cells: Vec<Vec<(i64, i64)>> =
            t.cells().into_iter().map(|c| vec![c.0, c.1, c.2]).collect();
        let mut edges: BTreeMap<UnitEdge, u32> = BTreeMap::new();
        for cell in &cells {
            for k in 0..3 {
                let (a, b) = (cell[k], cell[(k + 1) % 3]);
                let key = if a <= b { (a, b) } else { (b, a) };
                *edges.entry(key).or_insert(0) += 1;
            }
        }
        let bedges: Vec<UnitEdge> = edges
            .into_iter()
            .filter(|(_, n)| *n == 1)
            .map(|(e, _)| e)
            .collect();
        assert_eq!(
            bedges.len(),
            (8 * t.s) as usize,
            "octiamond boundary: 8s unit edges"
        );
        let mut adj: BTreeMap<(i64, i64), Vec<(i64, i64)>> = BTreeMap::new();
        for (a, b) in &bedges {
            adj.entry(*a).or_default().push(*b);
            adj.entry(*b).or_default().push(*a);
        }
        for (v, ws) in &adj {
            assert_eq!(ws.len(), 2, "boundary vertex {v:?} pinched");
        }
        let start = *adj.keys().next().unwrap();
        let mut cyc = vec![start, adj[&start][0]];
        while cyc.last().copied() != Some(start) {
            let (prev, cur) = (cyc[cyc.len() - 2], cyc[cyc.len() - 1]);
            let mut nexts: Vec<(i64, i64)> =
                adj[&cur].iter().copied().filter(|&w| w != prev).collect();
            nexts.sort();
            assert_eq!(nexts.len(), 1, "ambiguous walk at {cur:?}");
            cyc.push(nexts[0]);
        }
        cyc.pop();
        // CCW in world orientation
        let area2 = |c: &[(i64, i64)]| -> f64 {
            let w = |p: (i64, i64)| Vec2::new(p.0 as f64 + 0.5 * p.1 as f64, H * p.1 as f64);
            let n = c.len();
            let mut s = 0.0;
            for i in 0..n {
                let a = w(c[i]);
                let b = w(c[(i + 1) % n]);
                s += a.x * b.y - b.x * a.y;
            }
            s
        };
        if area2(&cyc) < 0.0 {
            cyc.reverse();
        }
        // merge collinear runs into corners
        let n = cyc.len();
        let dirs: Vec<(i64, i64)> = (0..n)
            .map(|i| (cyc[(i + 1) % n].0 - cyc[i].0, cyc[(i + 1) % n].1 - cyc[i].1))
            .collect();
        let corners: BTreeSet<(i64, i64)> = (0..n)
            .filter(|&i| dirs[i] != dirs[(i + n - 1) % n])
            .map(|i| cyc[i])
            .collect();
        corners
    }

    /// The nine children of an octiamond exactly tile the octiamond
    /// (deflation view), for every orientation class and two scales
    /// (λ = 3: sides 3 and 9): exact integer unit-cell census — every
    /// parent cell covered exactly once, no cell outside.
    #[test]
    fn substitution_tiles_parent_exactly() {
        for cls in 0..12u8 {
            for s in [3i64, 9] {
                let parent = Octiamond {
                    cls,
                    q: 3,
                    r: -2,
                    s,
                };
                let want = parent.cells();
                assert_eq!(
                    want.len(),
                    (8 * s * s) as usize,
                    "cls {cls} s{s}: parent cells"
                );
                let mut cov = BTreeMap::new();
                for kid in parent.children() {
                    assert_eq!(kid.s, s / 3, "child scale");
                    for cell in kid.cells() {
                        *cov.entry(cell).or_insert(0) += 1;
                    }
                }
                assert_eq!(cov.len(), want.len(), "cls {cls} s{s}: coverage");
                for (cell, n) in &cov {
                    assert_eq!(*n, 1, "cls {cls} s{s}: cell {cell:?} covered {n} times");
                    assert!(want.contains(cell), "cls {cls} s{s}: child cell outside");
                }
            }
        }
    }

    /// L2 nesting audit under the λ-scaled framing (research notes
    /// s28): the L1 panels of the 9×-inflated canonical supertile are
    /// `M_i·TRAP3Q + 3·t_i` — the λ = 3 scale on the child translations
    /// is mandatory (the unscaled `1·t_i` variant is measurably
    /// defective: 54 ≠ 81 distinct grandchildren, and on the sphinx
    /// λ = 2 control the same defect shows as 45/63/96 vs the clean
    /// 0/96/0/0). Asserted: 0 overlaps, union = the 648-cell supertile
    /// (0 holes, 0 spills), and the 81 grandchildren
    /// `(M_i∘M_j, M_i·t_j + 3·t_i)` all distinct with a perfect census.
    #[test]
    fn l2_nesting_audit() {
        let parent9 = Octiamond {
            cls: 9,
            q: 0,
            r: 0,
            s: 9,
        };
        let want = parent9.cells();
        assert_eq!(want.len(), 648, "9×-inflated supertile cells");

        // L1 panels: the nine scale-3 children, expressed in the pinned
        // framing (class M_i, translation 3·t_i).
        let mut cov = BTreeMap::new();
        for (mi, tq, tr) in CHILDREN {
            let panel = Octiamond {
                cls: mi,
                q: 3 * tq,
                r: 3 * tr,
                s: 3,
            };
            assert_eq!(panel.cells().len(), 72, "panel cell count");
            for cell in panel.cells() {
                *cov.entry(cell).or_insert(0) += 1;
            }
        }
        assert_eq!(cov.len(), 648, "L1 union size (0 holes, 0 spills)");
        assert!(cov.values().all(|&n| n == 1), "L1 overlaps");
        assert_eq!(cov.keys().copied().collect::<BTreeSet<_>>(), want);

        // Grandchildren: (M_i∘M_j, M_i·t_j + 3·t_i) — 81 entries, all
        // distinct, tiling the supertile exactly.
        let mut gc: Vec<Octiamond> = Vec::with_capacity(81);
        let mut keys: BTreeSet<(u8, i64, i64)> = BTreeSet::new();
        for (mi, t1q, t1r) in CHILDREN {
            for (mj, t2q, t2r) in CHILDREN {
                let (dq, dr) = mat_vec(SYMS[mi as usize], (t2q, t2r));
                let g = Octiamond {
                    cls: compose(mi, mj),
                    q: 3 * t1q + dq,
                    r: 3 * t1r + dr,
                    s: 1,
                };
                assert!(
                    keys.insert((g.cls, g.q, g.r)),
                    "duplicate grandchild ({mi}, {mj})"
                );
                gc.push(g);
            }
        }
        assert_eq!(gc.len(), 81, "grandchild count");
        assert_eq!(keys.len(), 81, "grandchildren distinct");
        let mut gcov = BTreeMap::new();
        for g in &gc {
            for cell in g.cells() {
                *gcov.entry(cell).or_insert(0) += 1;
            }
        }
        assert_eq!(gcov.len(), 648, "grandchild union size");
        assert!(gcov.values().all(|&n| n == 1), "grandchild overlaps");
        assert_eq!(gcov.keys().copied().collect::<BTreeSet<_>>(), want);
    }

    /// All unit triangles with all three vertices inside the
    /// axis-aligned lattice parallelogram spanned by (mq, 0) and
    /// (0, mr) — the expected census set for the seed block and the
    /// growth battery (the block sides are lattice-axis-aligned, so
    /// vertex membership is exact).
    #[cfg(test)]
    fn parallelogram_cells(mq: i64, mr: i64) -> BTreeSet<UnitCell> {
        let inside = |p: (i64, i64)| p.0 >= 0 && p.0 <= mq && p.1 >= 0 && p.1 <= mr;
        let mut want = BTreeSet::new();
        for q in 0..mq {
            for r in 0..mr {
                for base in [
                    [(q, r), (q + 1, r), (q, r + 1)],
                    [(q + 1, r), (q, r + 1), (q + 1, r + 1)],
                ] {
                    let mut tri = base;
                    tri.sort();
                    assert!(tri.iter().all(|&p| inside(p)));
                    want.insert((tri[0], tri[1], tri[2]));
                }
            }
        }
        assert_eq!(want.len(), (2 * mq * mr) as usize, "parallelogram cell count");
        want
    }

    /// The seed block: 18 octiamonds tiling the 12×6 lattice
    /// parallelogram exactly (independently constructed expected cell
    /// set), the class multiset as measured, and the block tiles the
    /// plane exactly once under ⟨(12, 0), (0, 6)⟩ (0 missing / 0 dups
    /// over a 48×36 lattice window).
    ///
    /// Measured seed class multiset (op-index: count) = {1: 2, 2: 5,
    /// 4: 1, 5: 1, 6: 1, 7: 1, 9: 5, 10: 2}. (The research brief's
    /// transcription listed R180 twice instead of five times — its own
    /// list sums to 15 ≠ 18; the multiset here is the one derived from
    /// the pinned rule + the block mirror and verified by the census.)
    #[test]
    fn seed_block_tiles_parallelogram() {
        let seed: Vec<Octiamond> = SEED_TILES
            .iter()
            .map(|(cls, aq, ar)| Octiamond {
                cls: *cls,
                q: *aq,
                r: *ar,
                s: 1,
            })
            .collect();
        let mut classes = [0usize; 12];
        for t in &seed {
            classes[t.cls as usize] += 1;
        }
        assert_eq!(
            classes,
            [0, 2, 5, 0, 1, 1, 1, 1, 0, 5, 2, 0],
            "seed class multiset (index = orientation class)"
        );
        let mut cov = BTreeMap::new();
        for t in &seed {
            for cell in t.cells() {
                *cov.entry(cell).or_insert(0) += 1;
            }
        }
        let want = parallelogram_cells(BLOCK_Q, BLOCK_R);
        assert_eq!(want.len(), 144, "block cell count");
        assert_eq!(cov.len(), want.len(), "seed coverage");
        assert!(cov.values().all(|&n| n == 1), "seed overlap/gap");
        assert_eq!(cov.keys().copied().collect::<BTreeSet<_>>(), want);

        // plane tiling: blocks (i, j), i in 0..=4, j in 0..=6 cover the
        // 48×36 window [0, 48] × [0, 36]; every unit triangle in the
        // window is covered exactly once, and no cell anywhere is
        // covered twice.
        let mut plane = BTreeMap::new();
        for j in 0..=6i64 {
            for i in 0..=4i64 {
                let (bq, br) = (i * BLOCK_Q, j * BLOCK_R);
                for (cls, aq, ar) in SEED_TILES {
                    let t = Octiamond {
                        cls,
                        q: bq + aq,
                        r: br + ar,
                        s: 1,
                    };
                    for cell in t.cells() {
                        *plane.entry(cell).or_insert(0) += 1;
                    }
                }
            }
        }
        assert!(
            plane.values().all(|&n| n == 1),
            "block lattice: a cell is covered twice"
        );
        let window = parallelogram_cells(4 * BLOCK_Q, 6 * BLOCK_R);
        assert_eq!(window.len(), 3456, "window cell count");
        for cell in &window {
            assert!(plane.contains_key(cell), "window cell {cell:?} missing");
        }
    }

    /// Deflating the seed block: counts follow 18·9^g, class counts
    /// follow the substitution matrix (M^g applied to the seed
    /// distribution), the total 2× area never changes, every generation
    /// is a perfect unit-cell tiling of the SAME 12·3^4 × 6·3^4
    /// parallelogram, and the final tiles land exactly on unit scale.
    #[test]
    fn growth_battery() {
        let g = 4u32;
        let u = 3i64.pow(g);
        let mut tiles: Vec<Octiamond> = SEED_TILES
            .iter()
            .map(|(cls, aq, ar)| Octiamond {
                cls: *cls,
                q: aq * u,
                r: ar * u,
                s: u,
            })
            .collect();
        // substitution matrix from the rule
        let mut m = [[0usize; 12]; 12];
        for h in 0..12u8 {
            for (k, _, _) in CHILDREN {
                m[h as usize][compose(h, k) as usize] += 1;
            }
        }
        for row in &m {
            assert_eq!(row.iter().sum::<usize>(), 9, "row sum");
        }
        for (j, _) in m.iter().enumerate() {
            assert_eq!(m.iter().map(|row| row[j]).sum::<usize>(), 9, "col sum");
        }
        let mut dist = [0usize; 12];
        for (cls, _, _) in SEED_TILES {
            dist[cls as usize] += 1;
        }
        let area2: i64 = tiles.iter().map(|t| 8 * t.s * t.s).sum();
        // the patch is the same every generation: build the expected
        // census once
        let want = parallelogram_cells(BLOCK_Q * u, BLOCK_R * u);
        for gen in 0..=g {
            if gen > 0 {
                let mut next = Vec::with_capacity(tiles.len() * 9);
                for t in &tiles {
                    next.extend(t.children());
                }
                tiles = next;
            }
            assert_eq!(
                tiles.len(),
                18 * 9usize.pow(gen),
                "gen {gen}: count law 18·9^g"
            );
            let mut obs = [0usize; 12];
            for t in &tiles {
                obs[t.cls as usize] += 1;
            }
            assert_eq!(obs, dist, "gen {gen}: orientation law");
            dist = (0..12)
                .map(|j| (0..12).map(|h| m[h][j] * dist[h]).sum())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let a2: i64 = tiles.iter().map(|t| 8 * t.s * t.s).sum();
            assert_eq!(a2, area2, "gen {gen}: total 2×area changed");
            let mut cov = BTreeMap::new();
            for t in &tiles {
                for cell in t.cells() {
                    *cov.entry(cell).or_insert(0) += 1;
                }
            }
            assert_eq!(
                cov.keys().copied().collect::<BTreeSet<_>>(),
                want,
                "gen {gen}: census"
            );
            assert!(cov.values().all(|&n| n == 1), "gen {gen}: overlap/gap");
        }
        assert!(tiles.iter().all(|t| t.s == 1), "final scale must be 1");
    }

    /// (Edge, foreign corner) incidences: for each tile, interior
    /// lattice points of its outline edges that are corners of some
    /// other tile (the sphinx battery's T-junction definition, which
    /// reproduces that battery's pinned 0/2/16/78/344 exactly).
    #[cfg(test)]
    fn tjunc_count(tiles: &[Octiamond]) -> usize {
        let corners_of = |t: &Octiamond| -> Vec<(i64, i64)> {
            let g = SYMS[t.cls as usize];
            CANONICAL_OUTLINE
                .iter()
                .map(|p| {
                    let (lq, lr) = mat_vec(g, (p[0] * t.s, p[1] * t.s));
                    (t.q + lq, t.r + lr)
                })
                .collect()
        };
        let all: BTreeSet<(i64, i64)> =
            tiles.iter().flat_map(|t| corners_of(t)).collect();
        let mut n = 0usize;
        for t in tiles {
            let pts = corners_of(t);
            let m = pts.len();
            for i in 0..m {
                let (a, b) = (pts[i], pts[(i + 1) % m]);
                let d = (b.0 - a.0, b.1 - a.1);
                let len = d.0.abs().max(d.1.abs());
                assert!(
                    len > 0
                        && d.0 % len == 0
                        && d.1 % len == 0
                        && smul_sq((d.0 / len, d.1 / len)) == 1,
                    "edge is a multiple of a unit direction"
                );
                for k in 1..len {
                    let mid = (a.0 + k * d.0 / len, a.1 + k * d.1 / len);
                    if all.contains(&mid) {
                        n += 1;
                    }
                }
            }
        }
        n
    }

    /// T-junctions are inherent to the octiamond substitution (which is
    /// the unique rep-9 dissection): the rule's own dissection of the
    /// scale-3 supertile carries 8 (edge, foreign corner) incidences,
    /// and the seed-patch census per generation is 18, 226, 2202,
    /// 20286, 183974 — child corners land strictly inside sibling
    /// edges at every scale (the chair/pinwheel/sphinx situation:
    /// documented and asserted, not fixed).
    ///
    /// MEASURED VALUES (erratum): the research brief pinned gen-0 = 0
    /// from a recon measurement whose unit-step boundary walk sees no
    /// multi-unit edges (under that walk the sphinx's own pinned gen-1
    /// count of 2 also reads 0); the values here are measured with the
    /// same definition as `sphinx.rs::t_junctions_are_inherent` and
    /// cross-validated against that battery's pinned ladder.
    #[test]
    fn t_junctions_census() {
        // the rule's own dissection: nine children of the canonical
        // scale-3 supertile
        let parent = Octiamond {
            cls: 9,
            q: 0,
            r: 0,
            s: 3,
        };
        let kids = parent.children();
        assert_eq!(kids.len(), 9);
        assert_eq!(tjunc_count(&kids), 8, "rule dissection incidences");

        // the seed-patch ladder, generations 0–4 (seed at scale 3^4)
        let u = 3i64.pow(4);
        let mut tiles: Vec<Octiamond> = SEED_TILES
            .iter()
            .map(|(cls, aq, ar)| Octiamond {
                cls: *cls,
                q: aq * u,
                r: ar * u,
                s: u,
            })
            .collect();
        let want = [18usize, 226, 2202, 20286, 183974];
        for (gen, w) in want.iter().enumerate() {
            if gen > 0 {
                let mut next = Vec::with_capacity(tiles.len() * 9);
                for t in &tiles {
                    next.extend(t.children());
                }
                tiles = next;
            }
            assert_eq!(tiles.len(), 18 * 9usize.pow(gen as u32));
            assert_eq!(tjunc_count(&tiles), *w, "gen {gen} T-junction census");
        }
    }

    /// The orientation classes reachable from the seed under the rule
    /// (class c → c∘k for the nine canonical children k) = all 12: the
    /// dissection mixes chirality, and the seed closes onto the whole
    /// orientation group.
    #[test]
    fn orientation_closure() {
        let mut seen: BTreeSet<u8> = SEED_TILES.iter().map(|(c, _, _)| *c).collect();
        loop {
            let mut grew = false;
            for c in 0..12u8 {
                if !seen.contains(&c) {
                    continue;
                }
                for (k, _, _) in CHILDREN {
                    grew |= seen.insert(compose(c, k));
                }
            }
            if !grew {
                break;
            }
        }
        assert_eq!(seen.len(), 12, "orientation closure from the seed");
        assert_eq!(
            seen,
            (0..12).collect::<BTreeSet<u8>>(),
            "all 12 classes reachable"
        );
    }

    /// Bit-exact tile identity: same seed and identical vertex bits.
    fn tile_key(tile: &Tile) -> (u32, Vec<u64>) {
        let mut bits = Vec::with_capacity(tile.poly.pts.len() * 2);
        for p in &tile.poly.pts {
            bits.push(p.x.to_bits());
            bits.push(p.y.to_bits());
        }
        (tile.seed, bits)
    }

    /// The patch is view-independent within a power-of-three
    /// tri-section: growing, panning, or zooming the view only moves
    /// the culling window over one fixed, origin-anchored refined
    /// tiling. (All four extents lie in (15, 45] — the tri-section
    /// [EXTENT_UNIT·3, EXTENT_UNIT·9); the zoomed extent 20/1.3 ≈
    /// 15.385 sits 2.6% above the floor, far outside float noise.)
    #[test]
    fn patch_is_stable_under_view_changes_within_a_trisection() {
        let t = OctiamondReptileTiling;
        let p = ParamSet::default_for(t.params());
        let base = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
            },
        );
        assert!(base.message.is_none(), "{:?}", base.message);
        let grown = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-8.4, -6.3), v2(8.4, 6.3)),
            },
        );
        let panned = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-7.5, -5.5), v2(8.5, 6.5)),
            },
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
        let grown_keys: std::collections::HashSet<(u32, Vec<u64>)> =
            grown.tiles.iter().map(tile_key).collect();
        for k in &base_keys {
            assert!(
                grown_keys.contains(k),
                "grow: tile changed when the view grew"
            );
        }
        for k in &zoomed_keys {
            assert!(
                base_keys.contains(k),
                "zoom: tile changed when the view changed"
            );
        }
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
            assert!(
                base_keys.contains(&k),
                "pan: overlap tile missing/changed in base view"
            );
            assert!(
                panned_keys.contains(&k),
                "pan: overlap tile missing/changed in panned view"
            );
        }
        assert!(
            !overlap.tiles.is_empty(),
            "pan: overlap view produced no tiles"
        );
    }

    /// The tri-section-anchored deflation always lands exactly on unit
    /// scale: every emitted octiamond has area 2√3 at default params,
    /// and the patch shows several orientation classes (the identity
    /// class among them).
    #[test]
    fn emitted_tiles_are_unit_scale() {
        let t = OctiamondReptileTiling;
        let p = ParamSet::default_for(t.params());
        let r = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
            },
        );
        assert!(r.message.is_none(), "{:?}", r.message);
        assert!(!r.tiles.is_empty());
        // unit octiamond: eight unit triangles of area √3/4 each → 2√3 = 4·H
        let want_area = 4.0 * H;
        let mut classes = BTreeSet::new();
        for tile in &r.tiles {
            let a = tile.poly.area();
            assert!(
                (a - want_area).abs() < 1e-9,
                "tile area {a} != {want_area} (unit octiamond at default params)"
            );
            classes.insert(tile.seed);
        }
        assert!(
            classes.contains(&10),
            "identity class (seed 10) must appear"
        );
        assert!(classes.len() >= 2, "several orientation classes visible");
    }

    /// The area guard clamps oversized views before any refinement
    /// work: a 708×708 view (area 501264 > MAX_TILES·2) fails fast
    /// with the zoom-in message; a 700×700 view (area 490000) — the
    /// largest square view under the guard — succeeds, and its final
    /// tiles are region-bound (≈ area/2√3 ≈ 141.4k < MAX_TILES): the
    /// effective clamp is on view area, not depth (per-step culling
    /// keeps every depth's emission region-bound).
    #[test]
    fn view_guard_clamps_oversized_views() {
        let t = OctiamondReptileTiling;
        let p = ParamSet::default_for(t.params());
        let big = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-354.0, -354.0), v2(354.0, 354.0)),
            },
        );
        assert_eq!(big.tiles.len(), 0, "oversized view emits no tiles");
        assert_eq!(
            big.message,
            Some("too many tiles for this view — zoom in"),
            "pre-guard message"
        );
        let ok = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-350.0, -350.0), v2(350.0, 350.0)),
            },
        );
        assert!(ok.message.is_none(), "{:?}", ok.message);
        assert!(!ok.tiles.is_empty());
        assert!(
            ok.tiles.len() <= MAX_TILES,
            "region-bound emission under the cap ({} tiles)",
            ok.tiles.len()
        );
    }
}