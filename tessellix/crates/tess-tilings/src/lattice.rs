//! Data-driven generator for periodic (wallpaper) tilings.
//!
//! A [`LatticeTiling`] is fully described by:
//! * a lattice basis (two vectors in "cell space"),
//! * a set of cell polygons (prototiles, in cell space — they may extend
//!   beyond the unit cell, only their lattice translates must tile the plane),
//! * a cell-to-world map (default: uniform scale + rotation, but any
//!   affine map works, which is how anisotropic families are expressed),
//! * symmetry metadata (wallpaper group, lattice kind).
//!
//! Adding a new uniform / Archimedean / rectangular tiling is therefore a
//! small data table, not new code.

use tess_core::category::TilingCategory;
use tess_core::geometry::{Affine, BBox, Polygon, Vec2};
use tess_core::params::{ParamId, ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;

/// Upper bound on tiles generated for a single view.
pub const MAX_TILES: usize = 250_000;

/// How the `seed` (pattern color) is derived from lattice coordinates.
#[derive(Debug, Clone, Copy, Default)]
pub enum SeedPattern {
    /// Seed comes only from the per-cell value.
    #[default]
    Uniform,
    /// Seed parity alternates with `(i + j)` — checkerboard.
    Checker,
    /// Seed cycles with `(i - j)` — 3-coloring for hex/triangle lattices.
    Triad,
}

impl SeedPattern {
    pub(crate) fn base(self, i: i64, j: i64) -> u32 {
        match self {
            SeedPattern::Uniform => 0,
            SeedPattern::Checker => ((((i + j) % 2) + 2) % 2) as u32,
            SeedPattern::Triad => ((((i - j) % 3) + 3) % 3) as u32,
        }
    }
}

/// One prototile in cell space.
#[derive(Debug, Clone)]
pub struct Cell {
    pub poly: Polygon,
    /// Added to the lattice-derived seed; e.g. distinguishes octagon vs.
    /// square in the 4.8.8 tiling.
    pub seed: u32,
}

pub const SCALE_ID: ParamId = ParamId::new("scale");
pub const ROTATION_ID: ParamId = ParamId::new("rotation");

/// Standard parameters shared by all lattice tilings.
pub const SIMILARITY_PARAMS: [ParamSpec; 2] = [
    // Default scale 1.0: unit tiles at unit scale (a 0.1 default forced
    // Hirschhorn growth to a ~10× larger patch and hung the UIs).
    ParamSpec::float(SCALE_ID, "scale", 0.1, 50.0, 0.05, 1.0),
    ParamSpec::angle(ROTATION_ID, "rotation (deg)", 0.0, 360.0, 5.0, 0.0),
];

/// Default cell-space-to-world map: uniform scale then rotation.
pub fn similarity_map(p: &ParamSet) -> Affine {
    let s = p.float(SCALE_ID, 1.0);
    let r = p.angle(ROTATION_ID, 0.0).to_radians();
    Affine::scale(s).then(Affine::rotation(r))
}

/// A periodic tiling specified as (lattice basis, cells, map, symmetry).
pub struct LatticeTiling {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    /// Lattice basis vectors, in cell space.
    pub lattice: [Vec2; 2],
    /// Prototiles, in cell space.
    pub cells: Vec<Cell>,
    /// Translation applied in cell space before the map.
    pub offset: Vec2,
    pub group: WallpaperGroup,
    pub lattice_kind: LatticeKind,
    /// Canonical display category (declared by each table — see
    /// [`TilingCategory`]).
    pub category: TilingCategory,
    pub seed_pattern: SeedPattern,
    pub params: &'static [ParamSpec],
    /// Builds the linear map from cell space to world space from params.
    pub make_map: fn(&ParamSet) -> Affine,
    /// Default color mode for UIs.
    pub color_mode: ColorMode,
}

impl Tiling for LatticeTiling {
    fn id(&self) -> &'static str {
        self.id
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn description(&self) -> &'static str {
        self.description
    }

    fn category(&self) -> TilingCategory {
        self.category
    }

    fn params(&self) -> &'static [ParamSpec] {
        self.params
    }

    fn default_color_mode(&self) -> ColorMode {
        self.color_mode
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let t = (self.make_map)(params);
        let l1 = t.transform(self.lattice[0]);
        let l2 = t.transform(self.lattice[1]);
        let origin = t.transform(self.offset);
        let det = l1.cross(l2);
        let meta = SymmetryMeta::new(self.group, self.lattice_kind);
        if det.abs() < 1e-12 {
            return TilingResult {
                tiles: Vec::new(),
                symmetry: meta,
                periodic: None,
                message: Some("degenerate parameter values"),
            };
        }

        // Inverse of the lattice map p -> origin + [l1 l2] * u.
        let inv = |p: Vec2| -> Vec2 {
            let q = p - origin;
            Vec2::new(
                (l2.y * q.x - l2.x * q.y) / det,
                (-l1.y * q.x + l1.x * q.y) / det,
            )
        };

        // Region corners in cell (lattice) coordinates.
        let mut u_min = inv(query.region.corners()[0]);
        let mut u_max = u_min;
        for &c in &query.region.corners()[1..] {
            let u = inv(c);
            u_min.x = u_min.x.min(u.x);
            u_min.y = u_min.y.min(u.y);
            u_max.x = u_max.x.max(u.x);
            u_max.y = u_max.y.max(u.y);
        }

        // Cells may extend outside the unit cell; pad the index ranges.
        //
        // The cell polygons live in cell space, but the index ranges live
        // in lattice coordinates (u space): the tile at index (i, j) is
        // the cell translated by `offset + i*a1 + j*a2` in cell space,
        // which in u space is exactly the cell shifted by (i, j). So the
        // per-cell extents must be measured in u space: map every cell
        // vertex through `u(p) = A⁻¹ p` (A = [a1 a2], the lattice basis as
        // columns) and take that bounding box. Using the cell-space AABB
        // here instead would only be safe when the basis happens to be
        // close to the standard basis — correctness must not depend on
        // that.
        let (a1, a2) = (self.lattice[0], self.lattice[1]);
        let det_a = a1.cross(a2);
        // det_a cannot be ~0 here: l1×l2 = det(map)·det_a was checked
        // above, and det(map) is finite and bounded away from zero only
        // via this same product — a singular basis trips that check.
        debug_assert!(det_a.abs() >= 1e-12, "degenerate lattice basis");
        let inv_det_a = 1.0 / det_a;
        let u_of = |p: Vec2| {
            Vec2::new(
                (a2.y * p.x - a2.x * p.y) * inv_det_a,
                (a1.x * p.y - a1.y * p.x) * inv_det_a,
            )
        };
        let mut cumin = Vec2::new(f64::MAX, f64::MAX);
        let mut cumax = Vec2::new(f64::MIN, f64::MIN);
        for cell in &self.cells {
            for p in &cell.poly.pts {
                let u = u_of(*p);
                cumin.x = cumin.x.min(u.x);
                cumin.y = cumin.y.min(u.y);
                cumax.x = cumax.x.max(u.x);
                cumax.y = cumax.y.max(u.y);
            }
        }

        // A tile at (i, j) has u-space extent `cell_u_aabb + (i, j)`, so
        // it can intersect the region (whose u bbox is [u_min, u_max])
        // only if
        //   i ∈ [u_min.x - cumax.x, u_max.x - cumin.x]  (componentwise in j).
        let i0 = (u_min.x - cumax.x).floor() as i64 - 1;
        let i1 = (u_max.x - cumin.x).ceil() as i64 + 1;
        let j0 = (u_min.y - cumax.y).floor() as i64 - 1;
        let j1 = (u_max.y - cumin.y).ceil() as i64 + 1;
        let n_i = (i1 - i0 + 1).max(0);
        let n_j = (j1 - j0 + 1).max(0);

        if n_i * n_j * self.cells.len() as i64 > MAX_TILES as i64 {
            return TilingResult {
                tiles: Vec::new(),
                symmetry: meta,
                periodic: Some([l1, l2]),
                message: Some("too many tiles for this view — zoom in"),
            };
        }

        // Transform cells to world space once (per cell), reuse for all (i, j).
        let world_cells: Vec<(Polygon, u32)> = self
            .cells
            .iter()
            .map(|c| (c.poly.transform(&t), c.seed))
            .collect();
        let cell_aabb: Vec<BBox> = world_cells.iter().map(|(p, _)| p.aabb()).collect();

        let mut tiles = Vec::new();
        for i in i0..=i1 {
            for j in j0..=j1 {
                let tr = origin + l1 * i as f64 + l2 * j as f64;
                let base = self.seed_pattern.base(i, j);
                for (ci, (poly, seed)) in world_cells.iter().enumerate() {
                    let b = cell_aabb[ci];
                    let bb = BBox::new(b.min + tr, b.max + tr);
                    if !bb.intersects(query.region) {
                        continue;
                    }
                    tiles.push(Tile {
                        proto: ci as u8,
                        poly: poly.translated(tr),
                        seed: base.wrapping_add(*seed),
                    });
                }
            }
        }

        TilingResult {
            tiles,
            symmetry: meta,
            periodic: Some([l1, l2]),
            message: None,
        }
    }
}
