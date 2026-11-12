//! Square grid — the uniform tiling 4.4.4.4, wallpaper group p4m.

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

pub fn spec() -> LatticeTiling {
    let unit = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
    ]);
    LatticeTiling {
        id: "square",
        name: "Square grid",
        description: "Uniform tiling 4.4.4.4 — the classic square grid (p4m).",
        lattice: [Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)],
        cells: vec![Cell { poly: unit, seed: 0 }],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P4m,
        lattice_kind: LatticeKind::Square,
        category: TilingCategory::Uniform,
        seed_pattern: SeedPattern::Checker,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
