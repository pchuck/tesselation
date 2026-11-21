//! Archimedean (uniform) tilings, expressed as [`LatticeTiling`] data tables.
//!
//! * `4.8.8` — truncated square tiling (octagons + squares), p4m
//! * `3.6.3.6` — trihexagonal tiling (hexagons + triangles), p6m
//! * `3.4.6.4` — rhombitrihexagonal tiling (hexagons + squares + triangles), p6m
//! * `3.3.3.3.6` — snub trihexagonal tiling (hexagons + triangles), p6
//! * `3.3.3.4.4` — elongated triangular tiling (triangle rows + square rows), cmm
//! * `4.6.12` — truncated trihexagonal tiling (dodecagons + hexagons + squares), p6m
//! * `3.12.12` — truncated hexagonal tiling (dodecagons + triangles), p6m
//! * `3.3.4.3.4` — snub square tiling (squares + triangles), p4g

use std::f64::consts::SQRT_2;

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

const H: f64 = 0.8660254037844386; // √3/2
const A: f64 = 1.0 + SQRT_2; // 4.8.8 cell side (octagon edge 1, square diagonal √2)
const T: f64 = SQRT_2 / 2.0; // √2/2

/// Uniform tiling 4.8.8 (truncated square tiling), wallpaper group p4m.
///
/// The square grid is truncated: each original vertex becomes a rotated
/// square and each original square becomes a regular octagon.
pub fn truncated_square() -> LatticeTiling {
    // Octagon inside the unit cell [0, A]²; all eight edges have length 1.
    let octagon = Polygon::from_slice(&[
        Vec2::new(T, 0.0),
        Vec2::new(A - T, 0.0),
        Vec2::new(A, T),
        Vec2::new(A, A - T),
        Vec2::new(A - T, A),
        Vec2::new(T, A),
        Vec2::new(0.0, A - T),
        Vec2::new(0.0, T),
    ]);
    // Rotated square at the origin: the truncated corner shared by four
    // octagons. Its edges are the diagonals of the original unit grid.
    let square = Polygon::from_slice(&[
        Vec2::new(T, 0.0),
        Vec2::new(0.0, T),
        Vec2::new(-T, 0.0),
        Vec2::new(0.0, -T),
    ]);
    LatticeTiling {
        id: "truncated-square",
        name: "Truncated Square (4.8.8)",
        description: "Uniform tiling 4.8.8 — regular octagons and squares (p4m).",
        lattice: [Vec2::new(A, 0.0), Vec2::new(0.0, A)],
        cells: vec![
            Cell { poly: octagon, seed: 0 },
            Cell { poly: square, seed: 1 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P4m,
        lattice_kind: LatticeKind::Square,
        category: TilingCategory::Uniform,
        secondary: &[TilingCategory::Wallpaper],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// Uniform tiling 3.6.3.6 (trihexagonal tiling), wallpaper group p6m.
///
/// Unit flat-top hexagons sit on the lattice `[(2,0), (1,√3)]`; every
/// hexagon edge borders a unit equilateral triangle. Each triangle borders
/// three hexagons and is emitted by exactly one of them — a hexagon claims
/// the triangles across its edges e0 and e1 — so the per-cell emission
/// (1 hexagon + 2 triangles) is gap-free and overlap-free.
pub fn trihexagonal() -> LatticeTiling {
    // Flat-top unit hexagon centered at the origin.
    let hex = Polygon::from_slice(&[
        Vec2::new(1.0, 0.0),
        Vec2::new(0.5, H),
        Vec2::new(-0.5, H),
        Vec2::new(-1.0, 0.0),
        Vec2::new(-0.5, -H),
        Vec2::new(0.5, -H),
    ]);
    // Unit triangle across edge e0: (1,0) -> (0.5,H).
    let t0 = Polygon::from_slice(&[
        Vec2::new(1.0, 0.0),
        Vec2::new(0.5, H),
        Vec2::new(1.5, H),
    ]);
    // Unit triangle across edge e1: (0.5,H) -> (-0.5,H).
    let t1 = Polygon::from_slice(&[
        Vec2::new(0.5, H),
        Vec2::new(-0.5, H),
        Vec2::new(0.0, 2.0 * H),
    ]);
    LatticeTiling {
        id: "trihexagonal",
        name: "Trihexagonal (3.6.3.6)",
        description: "Uniform tiling 3.6.3.6 (trihexagonal tiling) — hexagons and triangles (p6m).",
        lattice: [Vec2::new(2.0, 0.0), Vec2::new(1.0, 2.0 * H)],
        cells: vec![
            Cell { poly: hex, seed: 0 },
            Cell { poly: t0, seed: 1 },
            Cell { poly: t1, seed: 2 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Uniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// Uniform tiling 3.3.3.3.6 (snub hexagonal tiling), wallpaper group p6.
///
/// The chiral snub of the hexagonal tiling: one regular hexagon and four
/// unit triangles at every vertex (six 60-degree angles total, exactly one
/// hexagon per vertex). Hexagon centers form the hexagonal lattice with
/// basis A1 = (5/2, -sqrt(3)/2) and A2 = (2, sqrt(3)) (spacing sqrt(7),
/// 60-degree angle); each unit cell contains one hexagon and eight unit
/// triangles, totalling the cell area 7*sqrt(3)/2. The cell polygons below
/// were extracted from a verified forced-growth construction (every vertex
/// angle closed to 360 degrees, no overlaps, exact A1/A2 periodicity).
pub fn snub_trihexagonal() -> LatticeTiling {
    // Unit hexagon centered at the origin, vertex at angle 0.
    let hex = Polygon::from_slice(&[
        Vec2::new(1.0, 0.0),
        Vec2::new(0.5, H),
        Vec2::new(-0.5, H),
        Vec2::new(-1.0, 0.0),
        Vec2::new(-0.5, -H),
        Vec2::new(0.5, -H),
    ]);
    // Eight unit triangles; all coordinates lie in Q(sqrt(3)).
    let t0 = Polygon::from_slice(&[
        Vec2::new(-1.0, 0.0),
        Vec2::new(-0.5, -H),
        Vec2::new(-1.5, -H),
    ]);
    let t1 = Polygon::from_slice(&[
        Vec2::new(-0.5, -H),
        Vec2::new(0.5, -H),
        Vec2::new(0.0, -2.0 * H),
    ]);
    let t2 = Polygon::from_slice(&[
        Vec2::new(0.5, -H),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.5, -H),
    ]);
    let t3 = Polygon::from_slice(&[
        Vec2::new(1.0, 0.0),
        Vec2::new(0.5, H),
        Vec2::new(1.5, H),
    ]);
    let t4 = Polygon::from_slice(&[
        Vec2::new(0.5, H),
        Vec2::new(-0.5, H),
        Vec2::new(0.0, 2.0 * H),
    ]);
    let t5 = Polygon::from_slice(&[
        Vec2::new(-1.5, -H),
        Vec2::new(-1.0, 0.0),
        Vec2::new(-2.0, 0.0),
    ]);
    let t6 = Polygon::from_slice(&[
        Vec2::new(-0.5, H),
        Vec2::new(-1.0, 0.0),
        Vec2::new(-1.5, H),
    ]);
    let t7 = Polygon::from_slice(&[
        Vec2::new(1.0, 0.0),
        Vec2::new(1.5, H),
        Vec2::new(2.0, 0.0),
    ]);
    LatticeTiling {
        id: "snub-trihexagonal",
        name: "Snub Trihexagonal (3.3.3.3.6)",
        description: "Uniform tiling 3.3.3.3.6 (snub hexagonal tiling) — hexagons and triangles (p6, chiral).",
        lattice: [Vec2::new(2.5, -H), Vec2::new(2.0, 2.0 * H)],
        cells: vec![
            Cell { poly: hex, seed: 0 },
            Cell { poly: t0, seed: 1 },
            Cell { poly: t1, seed: 1 },
            Cell { poly: t2, seed: 1 },
            Cell { poly: t3, seed: 1 },
            Cell { poly: t4, seed: 1 },
            Cell { poly: t5, seed: 1 },
            Cell { poly: t6, seed: 1 },
            Cell { poly: t7, seed: 1 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Uniform,
        secondary: &[TilingCategory::Wallpaper],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// Uniform tiling 3.3.3.4.4 (elongated triangular tiling), wallpaper group cmm.
///
/// Alternating rows of equilateral triangles and unit squares (Conway's
/// "isosnub quadrille"); every vertex sees three triangles and two squares.
/// Consecutive rows are offset by half a unit, giving the primitive lattice
/// a1 = (1, 0), a2 = (1/2, 1 + sqrt(3)/2) (area 1 + sqrt(3)/2 = one square
/// + two unit triangles, the per-cell emission).
pub fn elongated_triangular() -> LatticeTiling {
    // One period: triangle strip y in [0, H] plus square strip y in [H, 1+H].
    // Up-pointing triangle on the bottom strip.
    let t_up = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.5, H),
    ]);
    // Down-pointing triangle completing the strip.
    let t_down = Polygon::from_slice(&[
        Vec2::new(0.5, H),
        Vec2::new(1.5, H),
        Vec2::new(1.0, 0.0),
    ]);
    // Unit square above the strip; bottom edge (0.5,H)-(1.5,H).
    let square = Polygon::from_slice(&[
        Vec2::new(0.5, H),
        Vec2::new(1.5, H),
        Vec2::new(1.5, 1.0 + H),
        Vec2::new(0.5, 1.0 + H),
    ]);
    LatticeTiling {
        id: "elongated-triangular",
        name: "Elongated Triangular (3.3.3.4.4)",
        description: "Uniform tiling 3.3.3.4.4 (elongated triangular tiling) — triangle rows and square rows (cmm).",
        lattice: [Vec2::new(1.0, 0.0), Vec2::new(0.5, 1.0 + H)],
        cells: vec![
            Cell { poly: t_up, seed: 1 },
            Cell { poly: t_down, seed: 1 },
            Cell { poly: square, seed: 2 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::Cmm,
        lattice_kind: LatticeKind::CenteredRectangular,
        category: TilingCategory::Uniform,
        secondary: &[TilingCategory::Wallpaper],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// Uniform tiling 3.4.6.4 (rhombitrihexagonal tiling), wallpaper group p6m.
///
/// The "expanded hexagonal tiling": unit hexagons pulled apart so every
/// hexagon edge is bridged by a unit square and every triple junction of
/// three mutually adjacent hexagons is filled with a unit triangle.
/// Hexagon centers form the hexagonal lattice a1 = (3/2 + sqrt(3)/2,
/// sqrt(3)/2 + 1/2) (30 degrees), a2 = (0, 1 + sqrt(3)) (spacing
/// 1 + sqrt(3) = apothem + 1 + apothem); each cell holds 1 hexagon,
/// 3 squares, and 2 triangles (area 4*sqrt(3)/2 + 3 = |det(a1, a2)|).
pub fn rhombitrihexagonal() -> LatticeTiling {
    // Unit pointy-top hexagon centered at the origin.
    let hex = Polygon::from_slice(&[
        Vec2::new(1.0, 0.0),
        Vec2::new(0.5, H),
        Vec2::new(-0.5, H),
        Vec2::new(-1.0, 0.0),
        Vec2::new(-0.5, -H),
        Vec2::new(0.5, -H),
    ]);
    // Square on the 30-degree edge (1,0)-(0.5,H), extending outward.
    let s30 = Polygon::from_slice(&[
        Vec2::new(1.0, 0.0),
        Vec2::new(0.5, H),
        Vec2::new(0.5 + H, H + 0.5),
        Vec2::new(1.0 + H, 0.5),
    ]);
    // Square on the 150-degree edge (-0.5,H)-(-1,0).
    let s150 = Polygon::from_slice(&[
        Vec2::new(-0.5, H),
        Vec2::new(-1.0, 0.0),
        Vec2::new(-1.0 - H, 0.5),
        Vec2::new(-0.5 - H, H + 0.5),
    ]);
    // Square on the 270-degree edge (-0.5,-H)-(0.5,-H).
    let s270 = Polygon::from_slice(&[
        Vec2::new(-0.5, -H),
        Vec2::new(0.5, -H),
        Vec2::new(0.5, -H - 1.0),
        Vec2::new(-0.5, -H - 1.0),
    ]);
    // Triangle in the triple junction at (1,0) between the hexagons at
    // the origin and their two neighbors in the -30-degree direction.
    let t0 = Polygon::from_slice(&[
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0 + H, -0.5),
        Vec2::new(1.0 + H, 0.5),
    ]);
    // Triangle in the triple junction at (0.5,H) between the origin,
    // the +30-degree neighbor, and the +90-degree neighbor.
    let t1 = Polygon::from_slice(&[
        Vec2::new(0.5, H),
        Vec2::new(0.5 + H, H + 0.5),
        Vec2::new(0.5, H + 1.0),
    ]);
    LatticeTiling {
        id: "rhombitrihexagonal",
        name: "Rhombitrihexagonal (3.4.6.4)",
        description: "Uniform tiling 3.4.6.4 (rhombitrihexagonal tiling) — hexagons, squares, and triangles (p6m).",
        lattice: [
            Vec2::new(1.5 + H, H + 0.5),
            Vec2::new(0.0, 2.0 * H + 1.0),
        ],
        cells: vec![
            Cell { poly: hex, seed: 0 },
            Cell { poly: s30, seed: 2 },
            Cell { poly: s150, seed: 2 },
            Cell { poly: s270, seed: 2 },
            Cell { poly: t0, seed: 1 },
            Cell { poly: t1, seed: 1 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Uniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
/// Uniform tiling 4.6.12 (truncated trihexagonal tiling, a.k.a. great
/// rhombitrihexagonal), wallpaper group p6m.
///
/// One square, one hexagon, and one dodecagon meet at every vertex.
/// Dodecagon centers form the hexagonal lattice
/// a1 = (3/2 + 3*sqrt(3)/2, 3/2 + sqrt(3)/2) (30 degrees),
/// a2 = (0, 3 + sqrt(3)) (90 degrees), spacing 3 + sqrt(3); each cell
/// holds 1 dodecagon + 2 hexagons + 3 squares, totalling the cell area
/// 9 + 6*sqrt(3) = |det(a1, a2)|.
pub fn truncated_trihexagonal() -> LatticeTiling {
    // Unit dodecagon centered at the origin (vertices at 15 + 60k degrees).
    let dodec = Polygon::from_slice(&[
        Vec2::new(1.0 + H, 0.5),
        Vec2::new(H + 0.5, H + 0.5),
        Vec2::new(0.5, 1.0 + H),
        Vec2::new(-0.5, 1.0 + H),
        Vec2::new(-H - 0.5, H + 0.5),
        Vec2::new(-1.0 - H, 0.5),
        Vec2::new(-1.0 - H, -0.5),
        Vec2::new(-H - 0.5, -H - 0.5),
        Vec2::new(-0.5, -1.0 - H),
        Vec2::new(0.5, -1.0 - H),
        Vec2::new(H + 0.5, -H - 0.5),
        Vec2::new(1.0 + H, -0.5),
    ]);
    // Unit pointy-top hexagon centered at (1 + 2H, 3 + 2H).
    let hex_up = Polygon::from_slice(&[
        Vec2::new(1.0 + H, 2.5 + 2.0 * H),
        Vec2::new(1.0 + 2.0 * H, 2.0 + 2.0 * H),
        Vec2::new(1.0 + 3.0 * H, 2.5 + 2.0 * H),
        Vec2::new(1.0 + 3.0 * H, 3.5 + 2.0 * H),
        Vec2::new(1.0 + 2.0 * H, 4.0 + 2.0 * H),
        Vec2::new(1.0 + H, 3.5 + 2.0 * H),
    ]);
    // Unit square on the dodecagon's 15-45 degree edge.
    let s1 = Polygon::from_slice(&[
        Vec2::new(1.0 + H, 0.5),
        Vec2::new(1.0 + 2.0 * H, 1.0),
        Vec2::new(0.5 + 2.0 * H, 1.0 + H),
        Vec2::new(H + 0.5, H + 0.5),
    ]);
    // Unit pointy-top hexagon centered at (H + 1/2, 3/2 + H).
    let hex_mid = Polygon::from_slice(&[
        Vec2::new(H + 0.5, H + 0.5),
        Vec2::new(0.5 + 2.0 * H, 1.0 + H),
        Vec2::new(0.5 + 2.0 * H, 2.0 + H),
        Vec2::new(H + 0.5, 2.5 + H),
        Vec2::new(0.5, 2.0 + H),
        Vec2::new(0.5, 1.0 + H),
    ]);
    // Unit axis-aligned square on the dodecagon's 90-degree edge.
    let s2 = Polygon::from_slice(&[
        Vec2::new(0.5, 1.0 + H),
        Vec2::new(0.5, 2.0 + H),
        Vec2::new(-0.5, 2.0 + H),
        Vec2::new(-0.5, 1.0 + H),
    ]);
    // Unit square between the two hexagons, on the dodecagon's 45-75
    // degree edge.
    let s3 = Polygon::from_slice(&[
        Vec2::new(1.0 + 2.0 * H, 2.0 + 2.0 * H),
        Vec2::new(1.0 + H, 2.5 + 2.0 * H),
        Vec2::new(H + 0.5, 2.5 + H),
        Vec2::new(0.5 + 2.0 * H, 2.0 + H),
    ]);
    LatticeTiling {
        id: "truncated-trihexagonal",
        name: "Truncated Trihexagonal (4.6.12)",
        description: "Uniform tiling 4.6.12 (truncated trihexagonal tiling) — dodecagons, hexagons, and squares (p6m).",
        lattice: [
            Vec2::new(1.5 + 3.0 * H, 1.5 + H),
            Vec2::new(0.0, 3.0 + 2.0 * H),
        ],
        cells: vec![
            Cell { poly: dodec, seed: 0 },
            Cell { poly: hex_up, seed: 1 },
            Cell { poly: s1, seed: 2 },
            Cell { poly: hex_mid, seed: 1 },
            Cell { poly: s2, seed: 2 },
            Cell { poly: s3, seed: 2 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Uniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// Uniform tiling 3.12.12 (truncated hexagonal tiling), wallpaper group p6m.
///
/// One triangle and two dodecagons meet at every vertex. Dodecagon
/// centers form the hexagonal lattice
/// a1 = (3/2 + sqrt(3), 1 + sqrt(3)/2) (30 degrees),
/// a2 = (0, 2 + sqrt(3)) (90 degrees), spacing 2 + sqrt(3) (two
/// apothems), so adjacent dodecagons share an edge. Each cell holds
/// 1 dodecagon + 2 triangles (two triangle orbits); per-cell area
/// 3(2 + sqrt(3)) + sqrt(3)/2 = 6 + 7*sqrt(3)/2 = |det(a1, a2)|.
pub fn truncated_hexagonal() -> LatticeTiling {
    // Unit dodecagon centered at the origin (vertices at 15 + 30k degrees).
    let dodec = Polygon::from_slice(&[
        Vec2::new(1.0 + H, 0.5),
        Vec2::new(H + 0.5, H + 0.5),
        Vec2::new(0.5, 1.0 + H),
        Vec2::new(-0.5, 1.0 + H),
        Vec2::new(-H - 0.5, H + 0.5),
        Vec2::new(-1.0 - H, 0.5),
        Vec2::new(-1.0 - H, -0.5),
        Vec2::new(-H - 0.5, -H - 0.5),
        Vec2::new(-0.5, -1.0 - H),
        Vec2::new(0.5, -1.0 - H),
        Vec2::new(H + 0.5, -H - 0.5),
        Vec2::new(1.0 + H, -0.5),
    ]);
    // Unit triangle across the dodecagon's 0-degree edge (1,0)-side:
    // base (1+H, +/-0.5), apex (1+2H, 0).
    let t_e = Polygon::from_slice(&[
        Vec2::new(1.0 + H, -0.5),
        Vec2::new(1.0 + H, 0.5),
        Vec2::new(1.0 + 2.0 * H, 0.0),
    ]);
    // Unit triangle across the dodecagon's 60-degree edge:
    // base (H+0.5, H+0.5)-(0.5, 1+H), apex (H+0.5, 1.5+H).
    let t_ne = Polygon::from_slice(&[
        Vec2::new(H + 0.5, H + 0.5),
        Vec2::new(0.5, 1.0 + H),
        Vec2::new(H + 0.5, 1.5 + H),
    ]);
    LatticeTiling {
        id: "truncated-hexagonal",
        name: "Truncated Hexagonal (3.12.12)",
        description: "Uniform tiling 3.12.12 (truncated hexagonal tiling) — dodecagons and triangles (p6m).",
        lattice: [
            Vec2::new(1.5 + 2.0 * H, 1.0 + H),
            Vec2::new(0.0, 2.0 + 2.0 * H),
        ],
        cells: vec![
            Cell { poly: dodec, seed: 0 },
            Cell { poly: t_e, seed: 1 },
            Cell { poly: t_ne, seed: 2 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Uniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// Uniform tiling 3.3.4.3.4 (snub square tiling), wallpaper group p4g.
///
/// The snub of the square tiling (achiral: p4g contains mirrors, unlike
/// the chiral snub hexagonal tiling whose group p6 has none): three unit triangles and two
/// unit squares meet at every vertex (3.3.4.3.4); the squares occur in
/// two orientations. The fundamental lattice is
/// a1 = (1 + sqrt(3), 0), a2 = (0, 1 + sqrt(3)); each cell holds
/// 4 unit squares + 8 unit triangles, area 4 + 2*sqrt(3) =
/// (1 + sqrt(3))^2 = |det(a1, a2)|. The 12 cell polygons below (all
/// CCW, coordinates in Z + sqrt(3)*Q) were extracted from a verified
/// forced-growth construction: all interior edges shared by exactly two
/// faces, every vertex cyclic figure 3.3.4.3.4, a shifted fine grid
/// covered exactly once, (L, 0)/(0, L) periods while (L/2, 0) is not.
pub fn snub_square() -> LatticeTiling {
    // Four unit squares (two orientation classes), CCW.
    let s0 = Polygon::from_slice(&[
        Vec2::new(0.0, 1.0 + 2.0 * H),
        Vec2::new(0.5, 1.0 + H),
        Vec2::new(0.5 + H, 1.5 + H),
        Vec2::new(H, 1.5 + 2.0 * H),
    ]);
    let s1 = Polygon::from_slice(&[
        Vec2::new(H, 0.5),
        Vec2::new(0.5 + H, 0.5 + H),
        Vec2::new(0.5, 1.0 + H),
        Vec2::new(0.0, 1.0),
    ]);
    let s2 = Polygon::from_slice(&[
        Vec2::new(1.0 + 2.0 * H, 1.0),
        Vec2::new(0.5 + 2.0 * H, 1.0 + H),
        Vec2::new(0.5 + H, 0.5 + H),
        Vec2::new(1.0 + H, 0.5),
    ]);
    let s3 = Polygon::from_slice(&[
        Vec2::new(1.0 + H, 1.5 + 2.0 * H),
        Vec2::new(0.5 + H, 1.5 + H),
        Vec2::new(0.5 + 2.0 * H, 1.0 + H),
        Vec2::new(1.0 + 2.0 * H, 1.0 + 2.0 * H),
    ]);
    // Eight unit triangles, CCW.
    let t0 = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(H, 0.5),
        Vec2::new(0.0, 1.0),
    ]);
    let t1 = Polygon::from_slice(&[
        Vec2::new(1.0 + 2.0 * H, 0.0),
        Vec2::new(1.0 + 2.0 * H, 1.0),
        Vec2::new(1.0 + H, 0.5),
    ]);
    let t2 = Polygon::from_slice(&[
        Vec2::new(0.0, 1.0 + 2.0 * H),
        Vec2::new(-0.5, 1.0 + H),
        Vec2::new(0.5, 1.0 + H),
    ]);
    let t3 = Polygon::from_slice(&[
        Vec2::new(H, 0.5),
        Vec2::new(0.5 + H, 0.5 - H),
        Vec2::new(1.0 + H, 0.5),
    ]);
    let t4 = Polygon::from_slice(&[
        Vec2::new(H, 0.5),
        Vec2::new(1.0 + H, 0.5),
        Vec2::new(0.5 + H, 0.5 + H),
    ]);
    let t5 = Polygon::from_slice(&[
        Vec2::new(0.0, 1.0),
        Vec2::new(0.5, 1.0 + H),
        Vec2::new(-0.5, 1.0 + H),
    ]);
    let t6 = Polygon::from_slice(&[
        Vec2::new(0.5 + 2.0 * H, 1.0 + H),
        Vec2::new(0.5 + H, 1.5 + H),
        Vec2::new(0.5 + H, 0.5 + H),
    ]);
    let t7 = Polygon::from_slice(&[
        Vec2::new(0.5, 1.0 + H),
        Vec2::new(0.5 + H, 0.5 + H),
        Vec2::new(0.5 + H, 1.5 + H),
    ]);
    LatticeTiling {
        id: "snub-square",
        name: "Snub Square (3.3.4.3.4)",
        description: "Uniform tiling 3.3.4.3.4 (snub square tiling) — squares and triangles (p4g; achiral, p4g has mirrors).",
        lattice: [
            Vec2::new(1.0 + 2.0 * H, 0.0),
            Vec2::new(0.0, 1.0 + 2.0 * H),
        ],
        cells: vec![
            Cell { poly: s0, seed: 0 },
            Cell { poly: s1, seed: 0 },
            Cell { poly: s2, seed: 0 },
            Cell { poly: s3, seed: 0 },
            Cell { poly: t0, seed: 1 },
            Cell { poly: t1, seed: 1 },
            Cell { poly: t2, seed: 1 },
            Cell { poly: t3, seed: 1 },
            Cell { poly: t4, seed: 1 },
            Cell { poly: t5, seed: 1 },
            Cell { poly: t6, seed: 1 },
            Cell { poly: t7, seed: 1 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P4g,
        lattice_kind: LatticeKind::Square,
        category: TilingCategory::Uniform,
        secondary: &[TilingCategory::Wallpaper],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
