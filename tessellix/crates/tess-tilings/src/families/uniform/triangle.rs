//! Triangular tiling 3.3.3.3.3.3, wallpaper group p6m.
//!
//! Lattice `[(1,0), (0.5, H)]` (H = √3/2); each parallelogram cell holds
//! one up-triangle and one down-triangle of side 1.

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

const H: f64 = 0.8660254037844386; // √3/2

pub fn spec() -> LatticeTiling {
    let up = Polygon::from_slice(&[
        Vec2::ZERO,
        Vec2::new(1.0, 0.0),
        Vec2::new(0.5, H),
    ]);
    let down = Polygon::from_slice(&[
        Vec2::new(0.5, H),
        Vec2::new(1.5, H),
        Vec2::new(1.0, 0.0),
    ]);
    LatticeTiling {
        id: "triangle",
        name: "Triangular (3⁶)",
        description: "Uniform tiling 3⁶ — equilateral triangles (p6m).",
        lattice: [Vec2::new(1.0, 0.0), Vec2::new(0.5, H)],
        cells: vec![
            Cell { poly: up, seed: 0 },
            Cell { poly: down, seed: 1 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Uniform,
        secondary: &[TilingCategory::Laves],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
