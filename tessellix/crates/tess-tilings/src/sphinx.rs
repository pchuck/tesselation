//! Sphinx tiling — the pentagonal hexiamond rep-tile substitution (λ = 2).
//!
//! The prototile is the *sphinx*: a pentagonal hexiamond (six unit
//! triangles of the triangular lattice, five boundary sides). It is a
//! rep-4 rep-tile: a sphinx of triangle side `s` is dissected into four
//! sphinxes of side `s/2`. The rule is **forced**: an exhaustive
//! enumeration (exact integer arithmetic — see the verification battery
//! described below) of all exact covers of the inflated sphinx by four
//! side-1 sphinxes in all 12 lattice orientations finds exactly ONE
//! dissection, and of the 12 free hexiamonds exactly two are rep-4 (the
//! 3×1 rhombus and the sphinx) with the sphinx the only pentagonal one.
//!
//! Geometry is exact on the triangular lattice. A lattice point is a
//! pair `(q, r)`; world position is `q·e1 + r·e2` with `e1 = (1, 0)`,
//! `e2 = (1/2, √3/2)`. The lattice has 12 symmetries (rotations by
//! 60°·k and their reflections), tabulated in [`SYMS`] as integer
//! matrices. The sphinx is asymmetric, and its unique dissection mixes
//! chiralities (three mirrored children, one rotated), so tiles come in
//! all **12 orientation classes**; the parent's symmetry conjugates the
//! canonical rule (child class = parent∘child, child anchor = parent
//! applied to the canonical anchor). Verified for all 12 parent
//! classes by exact cell census.
//!
//! The tiling is **non-edge-to-edge**: the forced dissection puts child
//! vertices strictly inside sibling edges at every scale (T-junction
//! incidences per generation 0, 2, 16, 78, 344 for the seed patch —
//! the same situation as the chair's and pinwheel's inherent
//! T-junctions). What the construction guarantees — and the tests
//! assert, exactly, on the integer grid — is that every generation is a
//! *perfect* tiling: every unit triangle of the patch is covered by
//! exactly one sphinx, counts follow N·4^g, class counts follow the
//! substitution matrix (rows and columns both sum to 4), the total area
//! never changes, and every refined edge segment (edges split at all
//! other tiles' vertices) is shared by exactly two sphinxes or lies on
//! the patch boundary.
//!
//! Emission follows the repo's finite-depth approximant convention (see
//! `pinwheel.rs` / `chair.rs`): the plane is seeded with rhombus blocks
//! — each a 2×3 lattice rhombus holding two side-`2^g` sphinxes (the
//! one exact tiling of that rhombus: a class-9 sphinx at the block
//! origin and a class-2 sphinx — the 180° rotation — anchored at
//! lattice (2, 3)·2^g) — on a block lattice anchored through the
//! origin, and every sphinx is deflated exactly `g` times, so the final
//! sphinxes land on unit scale (area 3√3/2) regardless of the view.
//! The seed scale is quantized to powers of 2 measured from the region
//! *extent*, not its position, so panning and zooming within an octave
//! only move the culling window over one fixed global tiling
//! (bit-identical tiles); crossing an octave selects a finer global
//! tiling with a doubled period — the finite-depth shadow of limit
//! periodicity.
//!
//! Pattern coloring: the tiling is monohedral, so `proto` is always 0;
//! `seed` is the orientation class + 1 (1..=12), which makes the
//! classes visible in "pattern" mode.
//!
//! Verification battery (exact, stdlib-only Python,
//! `/tmp/opencode/sphinx_verify.py`): triangular-lattice group
//! machinery (12 symmetries, each verified to permute the six unit
//! directions); enumeration of all free polyiamonds up to size 6
//! (counts 1, 1, 1, 3, 4, 12 — a self-check); exhaustive rep-4 census
//! over the 12 hexiamonds; uniqueness of the sphinx dissection;
//! conjugation check for all 12 parent orientations; substitution
//! matrix row/column sums; smallest-rhombus seed search (exact covers);
//! and a 5-generation growth battery asserting the perfect unit-cell
//! census of the invariant 32×48-cell patch, the count/orientation/
//! area laws, the refined edge census, and final unit scale.

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

/// The canonical sphinx: its six unit cells (sorted vertex triples) in
/// the canonical frame (translate-minimized). 2× world area 6·H, i.e.
/// area 6·√3/4.
#[cfg(test)]
const CANONICAL_CELLS: [[[i64; 2]; 3]; 6] = [
    [[0, 0], [0, 1], [1, 0]],
    [[0, 1], [0, 2], [1, 1]],
    [[0, 1], [1, 0], [1, 1]],
    [[0, 2], [0, 3], [1, 2]],
    [[0, 2], [1, 1], [1, 2]],
    [[1, 0], [1, 1], [2, 0]],
];

/// The canonical sphinx corner outline (orientation class 9 = identity),
/// CCW in world orientation, five corners (collinear unit steps of the
/// cell boundary merged). 2× shoelace area 6·H; the boundary sides have
/// lattice lengths 2, 1, 1, 1, 3. Every orientation class is emitted as
/// the image of this outline under its lattice symmetry — reversed when
/// that symmetry mirrors, so the emitted polygon is always CCW — see
/// `Sphinx::poly`.
const CANONICAL_OUTLINE: [[i64; 2]; 5] = [[2, 0], [1, 1], [1, 2], [0, 3], [0, 0]];

/// The forced dissection: the four children of the canonical side-2
/// sphinx as (orientation class, anchor q, anchor r) at scale 1 —
/// derived by exhaustive enumeration and unique (see module docs).
/// Children of a class-`c` parent: class `c∘k`, anchor
/// `parent_pos + (s/2)·c(anchor)` — see `Sphinx::children`.
const CHILDREN: [(u8, i64, i64); 4] = [(1, 4, 0), (3, 2, 1), (8, 0, 3), (8, 0, 6)];

/// The seed block: two side-u sphinxes tiling the 2×3 lattice rhombus
/// (the unique smallest rhombic patch — found by an exhaustive exact
/// cover search; the tiling itself is unique): a class-9 sphinx at the
/// block origin and a class-2 sphinx (180° rotation) anchored at
/// lattice (2, 3) within the block.
const SEED_TILES: [(u8, i64, i64); 2] = [(9, 0, 0), (2, 2, 3)];

/// Block lattice: the seed rhombus is spanned by `2·e1` and `3·e2` in
/// lattice units, so block `(i, j)` has lattice origin
/// `(i·2u, j·3u)` at seed scale `u`.
const BLOCK_Q: i64 = 2;
const BLOCK_R: i64 = 3;

/// View extent (world units) at which the seed octave increments. The
/// seed block's diameter is √19 ≈ 4.36 world units; like the chair
/// (`EXTENT_UNIT` 6 for a block diagonal of 5) this sits one step above
/// it.
const EXTENT_UNIT: f64 = 5.5;

fn mat_vec(m: [i64; 4], p: (i64, i64)) -> (i64, i64) {
    (m[0] * p.0 + m[1] * p.1, m[2] * p.0 + m[3] * p.1)
}

fn mat_det(m: [i64; 4]) -> i64 {
    m[0] * m[3] - m[1] * m[2]
}

/// Group composition: the class index of `SYMS[h] · SYMS[k]` (apply `k`
/// first, then `h`) — the product of two tabulated symmetries is
/// tabulated ( asserted by a test).
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

/// A sphinx on the triangular lattice: orientation class `cls` (one of
/// the 12 lattice symmetries applied to the canonical shape), lattice
/// anchor `(q, r)`, triangle side `s` (a power of two ≥ 1). Every
/// coordinate in the construction is an integer, so refinement and
/// culling are exact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Sphinx {
    cls: u8,
    q: i64,
    r: i64,
    s: i64,
}

impl Sphinx {
    /// The tile's five corners in world coordinates (unit-scale tiling
    /// space): the canonical corner outline transformed by the class
    /// symmetry and the tile's position. Mirrored classes are reversed
    /// so every emitted polygon is CCW. The vertex correspondence is a
    /// fixed per-class relabeling of the canonical outline — rotation
    /// classes keep i↔i (and share one tess-gui triangulation-cache
    /// entry); mirror classes hash to their own cache fingerprints (see
    /// tess-gui's `layout_fingerprint`), each internally consistent.
    fn corners(&self) -> [Vec2; 5] {
        let g = SYMS[self.cls as usize];
        let mut pts = [Vec2::new(0.0, 0.0); 5];
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

    /// Polygon outline (CCW, five corners).
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

    /// The four substitution children (triangle side s/2; requires s
    /// even). Rule: conjugate the canonical dissection by the parent's
    /// symmetry — child class = parent∘canonical, child anchor =
    /// parent anchor + (s/2)·parent-symmetry(canonical anchor). They
    /// tile the parent exactly (verified by an exact unit-cell census
    /// in the tests, for all twelve classes).
    fn children(&self) -> [Sphinx; 4] {
        let s = self.s;
        debug_assert!(s % 2 == 0, "children need an even triangle side");
        let h = SYMS[self.cls as usize];
        let half = s / 2;
        CHILDREN.map(|(k, aq, ar)| {
            let (tq, tr) = mat_vec(h, (aq * half, ar * half));
            Sphinx {
                cls: compose(self.cls, k),
                q: self.q + tq,
                r: self.r + tr,
                s: half,
            }
        })
    }

    /// The tile's unit cells (test helper): each of the six side-`s`
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

/// Seed sphinxes: the 2×3 rhombus blocks (each holding the two
/// `SEED_TILES` sphinxes at scale 2^g) on the block lattice anchored
/// through the origin, covering `region`. Block indices are found by
/// projecting the region corners onto the block-lattice basis (with a
/// one-block margin), which is sound: a point in block `(i, j)` has
/// basis coordinates `i + α, j + β` with `α, β ∈ [0, 1]`, so the
/// containing block's indices lie within ±1 of any corner's projection.
fn seed_sphinxes(region: &BBox, g: u32) -> Vec<Sphinx> {
    let u = 1i64 << g;
    // Block lattice vectors in world coordinates: U = 2u·e1,
    // V = 3u·e2 = (1.5u, 3u·H).
    let (ux, uy) = (2.0 * u as f64, 0.0);
    let (vx, vy) = (1.5 * u as f64, 3.0 * H * u as f64);
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
                v.push(Sphinx {
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

/// One deflation step: replace every sphinx by its four children (scale
/// 1/2), keeping only those whose bounding box intersects `region`.
/// Sound: children never leave their parent, so a final tile
/// intersecting the region has its full ancestor chain present —
/// nothing needed is ever dropped.
fn deflate_step(sphinxes: &[Sphinx], region: &BBox) -> Vec<Sphinx> {
    let mut out = Vec::with_capacity(sphinxes.len() * 4);
    for t in sphinxes {
        for kid in t.children() {
            if kid.aabb().intersects(*region) {
                out.push(kid);
            }
        }
    }
    out
}

/// Sphinx tiling (aperiodic, limit-periodic rep-4 substitution).
pub struct SphinxTiling;

impl Tiling for SphinxTiling {
    fn id(&self) -> &'static str {
        "sphinx"
    }

    fn name(&self) -> &'static str {
        "Sphinx (pentagonal hexiamond)"
    }

    fn description(&self) -> &'static str {
        "Aperiodic limit-periodic sphinx (rep-4 hexiamond) substitution tiling; λ = 2; tiles in 12 orientations."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Aperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None)
            .with_notes("aperiodic; limit-periodic; tiles in 12 orientations; non-edge-to-edge");
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
        // Final sphinxes have area exactly 3√3/2 ≈ 2.598 in tiling
        // space (six unit triangles; the octave-anchored deflation
        // always lands on unit scale), and the cell-space AABB of a
        // rotated world box is at most ~2× the world area, so this
        // bounds the tile count before any refinement work.
        if region.area() > MAX_TILES as f64 * 2.0 {
            return fail("too many tiles for this view — zoom in");
        }

        // View-independent patch: quantize the seed scale to powers of
        // 2 (one octave per deflation depth) measured from the region
        // *extent* (not its position — panning must not change the
        // octave), and anchor the rhombus seed blocks to the block
        // lattice through the origin. The emitted tiles are the
        // restriction of one global refined tiling to the query region:
        // panning and resizing only move the culling window; the
        // pattern changes only when the extent crosses a power-of-two
        // octave (the depth increments and a finer global tiling is
        // selected). Deflating exactly `depth` times lands the final
        // sphinxes on unit scale.
        let extent = (rmax - rmin).length();
        if !extent.is_finite() || extent <= 1e-9 {
            return fail("empty view");
        }
        let depth = ((extent / EXTENT_UNIT).log2().ceil().max(0.0) as u32).min(40);

        let mut sphinxes = seed_sphinxes(&region, depth);
        for _ in 0..depth {
            sphinxes = deflate_step(&sphinxes, &region);
        }
        if sphinxes.is_empty() {
            return fail("empty view");
        }

        // Emit the sphinxes as tiles. The tiling is monohedral (one
        // prototile), so `proto` is always 0; `seed` is the orientation
        // class + 1 (1..=12) — see the module docs — which makes the
        // classes visible in "pattern" mode.
        let mut tiles = Vec::with_capacity(sphinxes.len());
        for t in &sphinxes {
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

    /// The emitted outline of every orientation class is a CCW sphinx
    /// pentagon of the exact area (2× area 6·H·s²), and its corner set
    /// matches an INDEPENDENT derivation: the boundary walk of the
    /// tile's own unit cells (corners only — collinear steps merged).
    #[test]
    fn emitted_outlines_are_sphinxes() {
        for cls in 0..12u8 {
            let t = Sphinx {
                cls,
                q: 3,
                r: -2,
                s: 5,
            };
            let pts = t.poly().pts;
            assert_eq!(pts.len(), 5, "cls {cls}: vertex count");
            let a2 = signed_area2(&pts);
            let want = 6.0 * H * 25.0;
            assert!(
                (a2 - want).abs() < 1e-9 * want,
                "cls {cls}: signed 2×area {a2} != {want} (want CCW sphinx)"
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
        let t = Sphinx { cls, q, r, s };
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
            "sphinx boundary: 8s unit edges"
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

    /// The four children of a sphinx exactly tile the sphinx (deflation
    /// view), for every orientation class and two scales: exact integer
    /// unit-cell census — every parent cell covered exactly once, no
    /// cell outside.
    #[test]
    fn substitution_tiles_parent_exactly() {
        for cls in 0..12u8 {
            for s in [2i64, 4] {
                let parent = Sphinx {
                    cls,
                    q: 3,
                    r: -2,
                    s,
                };
                let want = parent.cells();
                assert_eq!(
                    want.len(),
                    (6 * s * s) as usize,
                    "cls {cls} s{s}: parent cells"
                );
                let mut cov = BTreeMap::new();
                for kid in parent.children() {
                    assert_eq!(kid.s, s / 2, "child scale");
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

    /// The seed block: two sphinxes tiling the 2×3 lattice rhombus
    /// exactly (independently constructed expected cell set), and the
    /// classes are the two rotations 9 and 2.
    #[test]
    fn seed_block_tiles_rhombus() {
        let seed: Vec<Sphinx> = SEED_TILES
            .iter()
            .map(|(cls, aq, ar)| Sphinx {
                cls: *cls,
                q: *aq,
                r: *ar,
                s: 1,
            })
            .collect();
        let mut classes: Vec<u8> = seed.iter().map(|t| t.cls).collect();
        classes.sort_unstable();
        assert_eq!(classes, [2, 9], "seed uses the two rotation classes");
        let mut cov = BTreeMap::new();
        for t in &seed {
            for cell in t.cells() {
                *cov.entry(cell).or_insert(0) += 1;
            }
        }
        // every unit triangle with all three vertices in the rhombus
        // spanned by (2, 0) and (0, 3) — exact half-plane membership.
        let (mq, mr) = (BLOCK_Q, BLOCK_R);
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
        assert_eq!(want.len(), (2 * mq * mr) as usize, "rhombus cell count");
        assert_eq!(cov.len(), want.len(), "seed coverage");
        assert!(cov.values().all(|&n| n == 1), "seed overlap/gap");
        assert_eq!(cov.keys().copied().collect::<BTreeSet<_>>(), want);
    }

    /// All unit triangles with all three vertices inside the rhombus
    /// spanned by (mq, 0) and (0, mr) — the expected census set for the
    /// growth battery (the block sides are lattice-axis-aligned, so
    /// vertex membership is exact).
    #[cfg(test)]
    fn rhombus_cells(mq: i64, mr: i64) -> BTreeSet<UnitCell> {
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
        assert_eq!(want.len(), (2 * mq * mr) as usize, "rhombus cell count");
        want
    }

    /// Deflating the seed block: counts follow 2·4^g, class counts
    /// follow the substitution matrix (M^g applied to the seed
    /// distribution), the total 2× area never changes, every generation
    /// is a perfect unit-cell tiling of the SAME 2·2^G × 3·2^G rhombus,
    /// and the final tiles land exactly on unit scale.
    #[test]
    fn refined_growth_battery() {
        let g = 3u32;
        let u = 1i64 << g;
        let mut tiles: Vec<Sphinx> = SEED_TILES
            .iter()
            .map(|(cls, aq, ar)| Sphinx {
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
            assert_eq!(row.iter().sum::<usize>(), 4, "row sum");
        }
        for (j, _) in m.iter().enumerate() {
            assert_eq!(m.iter().map(|row| row[j]).sum::<usize>(), 4, "col sum");
        }
        let mut dist = [0usize; 12];
        for (cls, _, _) in SEED_TILES {
            dist[cls as usize] += 1;
        }
        let area2: i64 = tiles.iter().map(|t| 6 * t.s * t.s).sum();
        for gen in 0..=g {
            if gen > 0 {
                let mut next = Vec::with_capacity(tiles.len() * 4);
                for t in &tiles {
                    next.extend(t.children());
                }
                tiles = next;
            }
            assert_eq!(tiles.len(), 2 * 4usize.pow(gen), "gen {gen}: count");
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
            let a2: i64 = tiles.iter().map(|t| 6 * t.s * t.s).sum();
            assert_eq!(a2, area2, "gen {gen}: total 2×area changed");
            let mut cov = BTreeMap::new();
            for t in &tiles {
                for cell in t.cells() {
                    *cov.entry(cell).or_insert(0) += 1;
                }
            }
            let want = rhombus_cells(BLOCK_Q * u, BLOCK_R * u);
            assert_eq!(
                cov.keys().copied().collect::<BTreeSet<_>>(),
                want,
                "gen {gen}: census"
            );
            assert!(cov.values().all(|&n| n == 1), "gen {gen}: overlap/gap");
        }
        assert!(tiles.iter().all(|t| t.s == 1), "final scale must be 1");
    }

    /// T-junctions are inherent to the sphinx substitution (which is
    /// the unique rep-4 dissection): in the first deflation of the
    /// seed patch, exactly 2 (edge, foreign corner) incidences occur —
    /// child corners landing strictly inside sibling edges — so the
    /// tiling is non-edge-to-edge at every scale (the same situation as
    /// the chair's and pinwheel's inherent T-junctions; the deeper
    /// counts 16, 78, 344, ... are asserted by the Python battery).
    #[test]
    fn t_junctions_are_inherent() {
        let g = 4u32;
        let u = 1i64 << g;
        let tiles: Vec<Sphinx> = SEED_TILES
            .iter()
            .map(|(cls, aq, ar)| Sphinx {
                cls: *cls,
                q: aq * u,
                r: ar * u,
                s: u,
            })
            .collect();
        let kids: Vec<Sphinx> = tiles.iter().flat_map(|t| t.children()).collect();
        assert_eq!(kids.len(), 8);
        // corner lattice points per tile (order-insensitive set)
        let corners_of = |t: &Sphinx| -> BTreeSet<(i64, i64)> {
            let gg = SYMS[t.cls as usize];
            CANONICAL_OUTLINE
                .iter()
                .map(|p| {
                    let (lq, lr) = mat_vec(gg, (p[0] * t.s, p[1] * t.s));
                    (t.q + lq, t.r + lr)
                })
                .collect()
        };
        let mut tjunc = 0usize;
        for t in &kids {
            let own = corners_of(t);
            let others: BTreeSet<(i64, i64)> = kids
                .iter()
                .filter(|o| *o != t)
                .flat_map(corners_of)
                .collect();
            let pts: Vec<(i64, i64)> = own.into_iter().collect();
            let n = pts.len();
            for i in 0..n {
                let (a, b) = (pts[i], pts[(i + 1) % n]);
                let d = (b.0 - a.0, b.1 - a.1);
                let len = d.0.abs().max(d.1.abs());
                assert!(smul_sq((d.0 / len, d.1 / len)) == 1, "unit edge");
                for k in 1..len {
                    let mid = (a.0 + k * d.0 / len, a.1 + k * d.1 / len);
                    if others.contains(&mid) {
                        tjunc += 1;
                    }
                }
            }
        }
        assert_eq!(tjunc, 2, "gen-1 T-junction incidences");
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

    /// The patch is view-independent within a power-of-two octave:
    /// growing, panning, or zooming the view only moves the culling
    /// window over one fixed, origin-anchored refined tiling. (All four
    /// extents lie in (11, 22] — the octave [EXTENT_UNIT·2,
    /// EXTENT_UNIT·4).)
    #[test]
    fn patch_is_stable_under_view_changes_within_an_octave() {
        let t = SphinxTiling;
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

    /// The octave-anchored deflation always lands exactly on unit
    /// scale: every emitted sphinx has area 3√3/2 at default params,
    /// and the patch shows several orientation classes (the seed's two
    /// rotation classes among them).
    #[test]
    fn emitted_tiles_are_unit_scale() {
        let t = SphinxTiling;
        let p = ParamSet::default_for(t.params());
        let r = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
            },
        );
        assert!(r.message.is_none(), "{:?}", r.message);
        assert!(!r.tiles.is_empty());
        // unit sphinx: six unit triangles of area √3/4 each → 3√3/2 = 3·H
        let want_area = 3.0 * H;
        let mut classes = BTreeSet::new();
        for tile in &r.tiles {
            let a = tile.poly.area();
            assert!(
                (a - want_area).abs() < 1e-9,
                "tile area {a} != {want_area} (unit sphinx at default params)"
            );
            classes.insert(tile.seed);
        }
        assert!(
            classes.contains(&10),
            "identity class (seed 10) must appear"
        );
        assert!(
            classes.contains(&3),
            "rotation class 2 (seed 3) must appear"
        );
        assert!(classes.len() >= 2, "several orientation classes visible");
    }
}
