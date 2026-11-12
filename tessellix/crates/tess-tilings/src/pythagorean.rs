//! Pythagorean (two-square) tiling — a parameterized family.
//!
//! Squares of two sizes, sides `s1` (large) and `s2` (small), on the
//! rotated square lattice
//!
//! ```text
//! a1 = (s1, s2),  a2 = (−s2, s1)
//! ```
//!
//! — the two leg vectors of the right triangle with legs s1, s2, so
//! |a1| = |a2| = √(s1² + s2²) and the fundamental cell has area
//! |a1×a2| = s1² + s2². Each cell holds one s1² square (corner at the
//! lattice point) and one s2² square tucked into its notch (corner at
//! (s1, 0)): the square on the hypotenuse dissected into the squares on
//! the legs — the classic visual proof of the Pythagorean theorem, and
//! the source of the name. The single parameter is the square ratio
//! r = b/a; the construction canonicalizes (s1, s2) = (max(a,b), min(a,b))
//! because the notch only fits the smaller square (for r < 1 this is the
//! r' = 1/r family, congruent up to scale). Big and small squares share
//! edge segments; the shared edge of two big squares is interrupted by
//! the small squares (T-junctions), so the tiling is not edge-to-edge
//! except at r = 1, where both prototiles are congruent unit squares of
//! the ordinary square grid.
//!
//! The wallpaper group is p4 for every ratio: quarter turns about the
//! square centers preserve both tile families (verified numerically over
//! generated patches and analytically — a reflection symmetry would need
//! the lattice orientation arctan(a/b) to be a multiple of 45°, i.e. a =
//! b), and at r = 1 the tiling is the plain square grid, p4m.
//!
//! Unlike the periodic tables, the lattice depends on a parameter (the
//! square ratio r = b/a), so this family implements [`Tiling`] directly
//! with the same index-range culling the [`crate::lattice::LatticeTiling`]
//! tables use, computed per call.

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamId, ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

/// Side ratio r = b/a of the two squares. 1 is the equal-square (plain
/// square grid, p4m) case; the default is the golden ratio.
pub const RATIO_ID: ParamId = ParamId::new("ratio");

/// Golden ratio — the default square ratio.
const PHI: f64 = 1.6180339887498949;

/// Parameters: the standard scale/rotation, plus the square ratio.
pub const PYTHAGOREAN_PARAMS: [ParamSpec; 3] = [
    SIMILARITY_PARAMS[0],
    SIMILARITY_PARAMS[1],
    ParamSpec::float(RATIO_ID, "square ratio b/a", 0.2, 10.0, 0.05, PHI),
];

/// Pythagorean two-square tiling — one-parameter family (p4, square
/// lattice of spacing √(a² + b²)).
pub struct PythagoreanTiling;

impl Tiling for PythagoreanTiling {
    fn id(&self) -> &'static str {
        "pythagorean"
    }

    fn name(&self) -> &'static str {
        "Pythagorean (two squares)"
    }

    fn description(&self) -> &'static str {
        "Two-square Pythagorean tiling — a b² square and an a² square per cell on the rotated lattice (b,a),(−a,b), so each cell is the square on the hypotenuse dissected into the leg squares. Ratio r = b/a is a parameter (default φ); p4, degenerating to the p4m square grid at r = 1."
    }

    fn params(&self) -> &'static [ParamSpec] {
        &PYTHAGOREAN_PARAMS
    }

    fn default_color_mode(&self) -> ColorMode {
        ColorMode::Pattern
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let r = params.float(RATIO_ID, PHI).clamp(0.2, 10.0);
        // The lattice must lead with the LARGER side: the small square is
        // tucked into the notch of the big squares, which only fits when
        // its side does not exceed the big side. For r < 1 this is the
        // r' = 1/r construction (congruent up to uniform scale).
        let (s1, s2) = if r >= 1.0 { (r, 1.0) } else { (1.0, r) };

        let map = similarity_map(params);
        let a1 = Vec2::new(s1, s2);
        let a2 = Vec2::new(-s2, s1);
        let l1 = map.transform(a1);
        let l2 = map.transform(a2);
        let det = l1.cross(l2);
        let meta = SymmetryMeta::new(WallpaperGroup::P4, LatticeKind::Square)
            .with_notes("rotations only (p4); mirrors appear at ratio 1, the p4m square-grid case");
        let fail = |message: &'static str| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: Some([l1, l2]),
            message: Some(message),
        };
        // det = det(map) · (a² + b²): the ratio is clamped away from 0, so
        // only a degenerate view transform can trip this.
        if det.abs() < 1e-12 {
            return fail("degenerate parameter values");
        }

        // Region corners in lattice (u) coordinates: invert the world lattice
// map p -> [l1 l2]·u (no offset — the cell origin is the world origin).
        // This must be the lattice inverse, not the cell-space map
        // inverse: the index ranges are in u space, and for a rotated
        // basis the cell-space bbox of the region is a different box (the
        // pads cannot be relied on to absorb the difference).
        let inv = |p: Vec2| {
            Vec2::new(
                (l2.y * p.x - l2.x * p.y) / det,
                (l1.x * p.y - l1.y * p.x) / det,
            )
        };
        let mut rmin = inv(query.region.corners()[0]);
        let mut rmax = rmin;
        for &corner in &query.region.corners()[1..] {
            let q = inv(corner);
            rmin.x = rmin.x.min(q.x);
            rmin.y = rmin.y.min(q.y);
            rmax.x = rmax.x.max(q.x);
            rmax.y = rmax.y.max(q.y);
        }
        if (rmax.x - rmin.x) * (rmax.y - rmin.y) <= 1e-12 {
            return fail("empty view");
        }

        // Index ranges in lattice coordinates: map every cell vertex
        // through u(p) = A⁻¹p (A = [a1 a2]) and pad — the same u-space
        // bounding box the lattice tables use (correct for any basis
        // orientation, not just near-axis-aligned ones).
        let det_a = a1.cross(a2);
        let inv_det_a = 1.0 / det_a;
        let u_of = |p: Vec2| {
            Vec2::new(
                (a2.y * p.x - a2.x * p.y) * inv_det_a,
                (a1.x * p.y - a1.y * p.x) * inv_det_a,
            )
        };
        let big = [
            Vec2::new(0.0, 0.0),
            Vec2::new(s1, 0.0),
            Vec2::new(s1, s1),
            Vec2::new(0.0, s1),
        ];
        let small = [
            Vec2::new(s1, 0.0),
            Vec2::new(s1 + s2, 0.0),
            Vec2::new(s1 + s2, s2),
            Vec2::new(s1, s2),
        ];
        let mut cumin = Vec2::new(f64::MAX, f64::MAX);
        let mut cumax = Vec2::new(f64::MIN, f64::MIN);
        for cell in [&big, &small] {
            for p in cell {
                let u = u_of(*p);
                cumin.x = cumin.x.min(u.x);
                cumin.y = cumin.y.min(u.y);
                cumax.x = cumax.x.max(u.x);
                cumax.y = cumax.y.max(u.y);
            }
        }
        let i0 = (rmin.x - cumax.x).floor() as i64 - 1;
        let i1 = (rmax.x - cumin.x).ceil() as i64 + 1;
        let j0 = (rmin.y - cumax.y).floor() as i64 - 1;
        let j1 = (rmax.y - cumin.y).ceil() as i64 + 1;
        let n_i = (i1 - i0 + 1).max(0);
        let n_j = (j1 - j0 + 1).max(0);
        if n_i * n_j * 2 > MAX_TILES as i64 {
            return fail("too many tiles for this view — zoom in");
        }

        // World-space cells, transformed once and translated per index.
        let world_cells: [(Polygon, u8, u32); 2] = [
            (Polygon::from_slice(&big).transform(&map), 0, 0),
            (Polygon::from_slice(&small).transform(&map), 1, 1),
        ];
        let cell_aabb: [BBox; 2] = [world_cells[0].0.aabb(), world_cells[1].0.aabb()];

        let mut tiles = Vec::new();
        for i in i0..=i1 {
            for j in j0..=j1 {
                let tr = l1 * i as f64 + l2 * j as f64;
                for (ci, (poly, proto, seed)) in world_cells.iter().enumerate() {
                    let b = cell_aabb[ci];
                    let bb = BBox::new(b.min + tr, b.max + tr);
                    if !bb.intersects(query.region) {
                        continue;
                    }
                    tiles.push(Tile {
                        proto: *proto,
                        poly: poly.translated(tr),
                        seed: *seed,
                    });
                }
            }
        }
        if tiles.len() > MAX_TILES {
            return fail("too many tiles for this view — zoom in");
        }

        TilingResult {
            tiles,
            symmetry: meta,
            periodic: Some([l1, l2]),
            message: None,
        }
    }
}