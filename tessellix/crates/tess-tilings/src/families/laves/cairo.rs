//! Cairo pentagonal tiling (the Catalan dual of the snub square tiling),
//! expressed as a [`LatticeTiling`] data table.
//!
//! * `3.3.3 / 4.4.4.4` — Cairo pentagonal tiling (p4g)
//!
//! The Cairo pentagonal tiling is the dual of the snub square tiling
//! (`3.3.4.3.4`, in `archimedean.rs`): each snub-square vertex becomes a
//! pentagon, and the tiling's wallpaper group (p4g) is inherited. It is the
//! only one of the eleven Catalan (dual-uniform) tilings that is not itself
//! uniform. All eight pentagons per cell are congruent; the pattern seed
//! below is a proper 4-coloring of the class-adjacency graph, so edge-
//! adjacent pentagons always differ in color.

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

const H: f64 = 0.8660254037844386; // √3/2
const L: f64 = 1.0 + 2.0 * H; // = 1 + √3, lattice period

/// Cairo pentagonal tiling, wallpaper group p4g.
///
/// Lattice: a1 = (1+√3, 0), a2 = (0, 1+√3). Each cell holds eight
/// congruent pentagons (the dual of the snub-square cell's two squares and
/// eight triangles). Edge lengths are √3/3 ≈ 0.577 and √3/6 + 1/2 ≈ 0.789
/// (ratio √3 − 1); interior angles are 90° (×2) and 120° (×3). Vertex
/// figures: 3.3.3 at the former snub-square triangle centers and 4.4.4.4 at
/// the former square centers. The pentagons are tangential (each has an
/// incircle), and the tiling is the minimum-perimeter pentagonal tiling.
///
/// Pentagon census: realizes type 4 (repo lettering shift k=4:
/// C = E = 90°, b = c, d = e), verified by test
/// `cairo_pentagonal_is_pentagon_type_4` (M101).
pub fn cairo_pentagonal() -> LatticeTiling {
    // Eight congruent pentagons, CCW. Coordinates are a + b·(√3/2); the
    // set tiles the plane exactly with period L in both axes. Seeds are a
    // proper 4-coloring of the class-adjacency graph.
    let p0 = Polygon::from_slice(&[
        Vec2::new(0.0, 1.0 + 2.0 / 3.0 * H),
        Vec2::new(0.25 + 0.5 * H, 0.75 + 0.5 * H),
        Vec2::new(0.5 + 2.0 / 3.0 * H, 1.0 + H),
        Vec2::new(0.25 + 0.5 * H, 1.25 + 1.5 * H),
        Vec2::new(0.0, 1.0 + 4.0 / 3.0 * H),
    ]);
    let p1 = Polygon::from_slice(&[
        Vec2::new(0.25 + 0.5 * H, 0.25 - 0.5 * H),
        Vec2::new(0.5 + H, 0.5 - 1.0 / 3.0 * H),
        Vec2::new(0.5 + H, 0.5 + 1.0 / 3.0 * H),
        Vec2::new(0.25 + 0.5 * H, 0.75 + 0.5 * H),
        Vec2::new(1.0 / 3.0 * H, 0.5),
    ]);
    let p2 = Polygon::from_slice(&[
        Vec2::new(0.25 + 0.5 * H, 0.75 + 0.5 * H),
        Vec2::new(0.5 + H, 0.5 + 1.0 / 3.0 * H),
        Vec2::new(0.75 + 1.5 * H, 0.75 + 0.5 * H),
        Vec2::new(0.5 + 4.0 / 3.0 * H, 1.0 + H),
        Vec2::new(0.5 + 2.0 / 3.0 * H, 1.0 + H),
    ]);
    let p3 = Polygon::from_slice(&[
        Vec2::new(0.5 + 2.0 / 3.0 * H, 1.0 + H),
        Vec2::new(0.5 + 4.0 / 3.0 * H, 1.0 + H),
        Vec2::new(0.75 + 1.5 * H, 1.25 + 1.5 * H),
        Vec2::new(0.5 + H, 1.5 + 5.0 / 3.0 * H),
        Vec2::new(0.25 + 0.5 * H, 1.25 + 1.5 * H),
    ]);
    let p4 = Polygon::from_slice(&[
        Vec2::new(0.5 + H, 0.5 - 1.0 / 3.0 * H),
        Vec2::new(0.75 + 1.5 * H, 0.25 - 0.5 * H),
        Vec2::new(1.0 + 5.0 / 3.0 * H, 0.5),
        Vec2::new(0.75 + 1.5 * H, 0.75 + 0.5 * H),
        Vec2::new(0.5 + H, 0.5 + 1.0 / 3.0 * H),
    ]);
    let p5 = Polygon::from_slice(&[
        Vec2::new(0.5 + 4.0 / 3.0 * H, 1.0 + H),
        Vec2::new(0.75 + 1.5 * H, 0.75 + 0.5 * H),
        Vec2::new(L, 1.0 + 2.0 / 3.0 * H),
        Vec2::new(L, 1.0 + 4.0 / 3.0 * H),
        Vec2::new(0.75 + 1.5 * H, 1.25 + 1.5 * H),
    ]);
    let p6 = Polygon::from_slice(&[
        Vec2::new(1.0 + 5.0 / 3.0 * H, 0.5),
        Vec2::new(1.0 + 7.0 / 3.0 * H, 0.5),
        Vec2::new(1.25 + 2.5 * H, 0.75 + 0.5 * H),
        Vec2::new(L, 1.0 + 2.0 / 3.0 * H),
        Vec2::new(0.75 + 1.5 * H, 0.75 + 0.5 * H),
    ]);
    let p7 = Polygon::from_slice(&[
        Vec2::new(0.75 + 1.5 * H, 1.25 + 1.5 * H),
        Vec2::new(L, 1.0 + 4.0 / 3.0 * H),
        Vec2::new(1.25 + 2.5 * H, 1.25 + 1.5 * H),
        Vec2::new(1.0 + 7.0 / 3.0 * H, 1.5 + 2.0 * H),
        Vec2::new(1.0 + 5.0 / 3.0 * H, 1.5 + 2.0 * H),
    ]);
    LatticeTiling {
        id: "cairo-pentagonal",
        name: "Type 4 · Cairo",
        description: "Cairo pentagonal tiling — Catalan dual of the snub square tiling (p4g). Tangential pentagons; the minimum-perimeter pentagonal tiling. Vertex figures 3.3.3 / 4.4.4.4. An integer-coordinate 'collinear' variant of the same pattern exists: two perpendicular flattened hexagonal tilings give pentagons with angles 143.13°/108.43°/108.43°/90°/90°, edges 4×√10+4, integral vertices, and a square 72-area cell.",
        lattice: [
            Vec2::new(L, 0.0),
            Vec2::new(0.0, L),
        ],
        cells: vec![
            Cell { poly: p0, seed: 0 },
            Cell { poly: p1, seed: 0 },
            Cell { poly: p2, seed: 1 },
            Cell { poly: p3, seed: 2 },
            Cell { poly: p4, seed: 3 },
            Cell { poly: p5, seed: 3 },
            Cell { poly: p6, seed: 1 },
            Cell { poly: p7, seed: 2 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P4g,
        lattice_kind: LatticeKind::Square,
        category: TilingCategory::Laves,
        secondary: &[TilingCategory::PentagonTypes],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

