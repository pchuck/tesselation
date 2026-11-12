//! Hexagonal tiling 6.6.6, wallpaper group p6m.

use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

const H: f64 = 0.8660254037844386; // √3/2, apothem of a unit hexagon

pub fn spec() -> LatticeTiling {
    // Flat-top unit hexagon centered at the origin.
    let hex = Polygon::from_slice(&[
        Vec2::new(1.0, 0.0),
        Vec2::new(0.5, H),
        Vec2::new(-0.5, H),
        Vec2::new(-1.0, 0.0),
        Vec2::new(-0.5, -H),
        Vec2::new(0.5, -H),
    ]);
    LatticeTiling {
        id: "hexagon",
        name: "Hexagonal",
        description: "Uniform tiling 6.6.6 — regular hexagons (p6m).",
        lattice: [Vec2::new(1.5, H), Vec2::new(1.5, -H)],
        cells: vec![Cell { poly: hex, seed: 0 }],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        seed_pattern: SeedPattern::Triad,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
