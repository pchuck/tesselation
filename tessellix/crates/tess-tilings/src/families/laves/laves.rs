//! Laves (Catalan, dual-uniform) tilings, expressed as [`LatticeTiling`]
//! data tables.
//!
//! Each entry is the face-centroid dual of one of the exact Archimedean
//! tables in `archimedean.rs`: the centers of the faces meeting at every
//! primal vertex, joined in cyclic order (primal faces are regular, so the
//! vertex average is the face center). The dual shares the primal lattice;
//! the per-cell table holds one canonical copy per primal-vertex orbit, so
//! every dual face is emitted exactly once by exactly one cell.
//!
//! * `tetrakis-square` — V4.8.8 (dual of the truncated square tiling), p4m
//! * `rhombille` — V3.6.3.6 (dual of the trihexagonal tiling), p6m
//! * `triakis-triangular` — V3.12.12 (dual of the truncated hexagonal tiling), p6m
//! * `deltoidal-trihexagonal` — V3.4.6.4 (dual of the rhombitrihexagonal tiling), p6m
//! * `kisrhombille` — V4.6.12 (dual of the truncated trihexagonal tiling), p6m
//! * `prismatic-pentagonal` — V3.3.3.4.4 (dual of the elongated triangular tiling), cmm
//! * `floret-pentagonal` — V3.3.3.3.6 (dual of the snub trihexagonal tiling), p6
//!
//! Every table below was derived and verified in exact quadratic-field
//! arithmetic (Q(√2) / Q(√3), no rounded keys) before being transcribed:
//! cell areas sum to |det(a1, a2)| exactly, every interior edge is shared
//! by exactly two faces, every complete dual vertex shows n copies of
//! 360/n degrees, and a shifted fine grid plus random samples over a
//! fundamental parallelogram are covered exactly once. Pattern seeds are
//! a proper coloring of the dual adjacency graph (edge-adjacent tiles
//! never share a "pattern" color).

use std::f64::consts::SQRT_2;

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

const T: f64 = SQRT_2 / 2.0; // √2/2
const H: f64 = 0.8660254037844386; // √3/2

/// Laves tiling V4.8.8 (tetrakis square tiling), wallpaper group p4m.
///
/// The centroid dual of the truncated square tiling (`4.8.8` in
/// `archimedean.rs`): every primal vertex — where a square and two
/// octagons meet — becomes a right isosceles triangle joining the three
/// face centers. Same square lattice as the primal, a1 = (A, 0),
/// a2 = (0, A) with A = 1 + √2; each cell holds 4 congruent triangles
/// (right angle at a square center, hypotenuse A between adjacent octagon
/// centers, legs A/√2 = 1/2 + T), totalling A² = |det|. Eight triangles
/// close at 45° around every octagon center, four at 90° around every
/// square center (face configuration V4.8.8).
pub fn tetrakis_square() -> LatticeTiling {
    let a = 1.0 + SQRT_2;
    // Right triangle with its right angle at the origin square center:
    // apexes at the octagon centers (±A/2, A/2) above it.
    let up = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(0.5 + T, 0.5 + T),    // (A/2, A/2) octagon center
        Vec2::new(-0.5 - T, 0.5 + T),   // (-A/2, A/2) octagon center
    ]);
    // Same triangle across the top edge of the square center one cell up.
    let down = Polygon::from_slice(&[
        Vec2::new(-0.5 - T, 0.5 + T),
        Vec2::new(0.5 + T, 0.5 + T),
        Vec2::new(0.0, 1.0 + 2.0 * T),  // (0, A) square center
    ]);
    // Left-pointing wedge of the square center one cell right.
    let left = Polygon::from_slice(&[
        Vec2::new(0.5 + T, -0.5 - T),
        Vec2::new(1.0 + 2.0 * T, 0.0),  // (A, 0) square center
        Vec2::new(0.5 + T, 0.5 + T),
    ]);
    // Right-pointing wedge at the origin square center.
    let right = Polygon::from_slice(&[
        Vec2::new(0.5 + T, -0.5 - T),
        Vec2::new(0.5 + T, 0.5 + T),
        Vec2::new(0.0, 0.0),
    ]);
    LatticeTiling {
        id: "tetrakis-square",
        name: "Tetrakis Square (V4.8.8)",
        description: "Laves tiling V4.8.8 (tetrakis square tiling) — right isosceles triangles, dual of the truncated square tiling (p4m).",
        lattice: [Vec2::new(a, 0.0), Vec2::new(0.0, a)],
        cells: vec![
            Cell { poly: up, seed: 0 },
            Cell { poly: down, seed: 1 },
            Cell { poly: left, seed: 2 },
            Cell { poly: right, seed: 3 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P4m,
        lattice_kind: LatticeKind::Square,
        category: TilingCategory::Laves,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
/// Laves tiling V3.6.3.6 (rhombille tiling), wallpaper group p6m.
///
/// The centroid dual of the trihexagonal tiling (`3.6.3.6` in
/// `archimedean.rs`): every primal vertex — where two hexagons and two
/// triangles alternate — becomes a 60°/120° rhombus joining the four face
/// centers (a parallelogram, so the dual face is centered on the primal
/// vertex). Same lattice as the primal, a1 = (2, 0), a2 = (1, √3); each
/// cell holds 3 congruent rhombi of side 2/√3 (two upright, one flat),
/// area 3 · 2/√3 = 2√3 = |det|. Three rhombi close at 120° around every
/// triangle center, six at 60° around every hexagon center.
///
/// The three rhombus orientations get seeds 0/1/2, so "pattern" mode
/// 3-colors by orientation — the classic tumbling-blocks cube illusion.
pub fn rhombille() -> LatticeTiling {
    // Upright rhombus with its left edge on the y axis: (0,0) up the
    // 120-degree diagonal to (0, 4H/3), across to (1, 2H), down to
    // (1, 2H/3).
    let upright0 = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 2.0 / 3.0 * H),
        Vec2::new(1.0, 2.0 * H),
        Vec2::new(0.0, 4.0 / 3.0 * H),
    ]);
    // Upright rhombus one column right (the second orientation copy).
    let upright1 = Polygon::from_slice(&[
        Vec2::new(1.0, 2.0 / 3.0 * H),
        Vec2::new(2.0, 0.0),
        Vec2::new(2.0, 4.0 / 3.0 * H),
        Vec2::new(1.0, 2.0 * H),
    ]);
    // Flat rhombus with horizontal long diagonal (2, 0)-(0, 0).
    let flat = Polygon::from_slice(&[
        Vec2::new(1.0, -2.0 / 3.0 * H),
        Vec2::new(2.0, 0.0),
        Vec2::new(1.0, 2.0 / 3.0 * H),
        Vec2::new(0.0, 0.0),
    ]);
    LatticeTiling {
        id: "rhombille",
        name: "Rhombille (V3.6.3.6)",
        description: "Laves tiling V3.6.3.6 (rhombille tiling) — 60/120-degree rhombi in three orientations, dual of the trihexagonal tiling (p6m).",
        lattice: [Vec2::new(2.0, 0.0), Vec2::new(1.0, 2.0 * H)],
        cells: vec![
            Cell { poly: upright0, seed: 0 },
            Cell { poly: upright1, seed: 1 },
            Cell { poly: flat, seed: 2 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Laves,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// Laves tiling V3.12.12 (triakis triangular tiling), wallpaper group p6m.
///
/// The centroid dual of the truncated hexagonal tiling (`3.12.12` in
/// `archimedean.rs`): every primal vertex — where one triangle and two
/// dodecagons meet — becomes an isosceles triangle joining the three face
/// centers. Same lattice as the primal, a1 = (3/2 + √3, 1 + √3/2),
/// a2 = (0, 2 + √3); each cell holds 6 congruent triangles (base
/// 2 + √3 between adjacent dodecagon centers, legs (2 + √3)/√3, apex at a
/// triangle center), area 6 · (2 + √3)²/(4√3) = 6 + 7√3/2 = |det|.
/// Twelve triangles close at 30° around every dodecagon center, three at
/// 120° around every triangle center.
pub fn triakis_triangular() -> LatticeTiling {
    // The six triangles tile the fundamental parallelogram spanned by a1
    // and a2 (height b = 2 + √3, the dodecagon-center spacing); every
    // triangle has its base on a dodecagon-center line and its apex on a
    // triangle-center line.
    let b = 2.0 + 2.0 * H; // 2 + √3
    let t_up = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(1.5 + 2.0 * H, 1.0 + H),
        Vec2::new(0.5 + 2.0 / 3.0 * H, 1.0 + H),
    ]);
    let t_left = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(0.5 + 2.0 / 3.0 * H, 1.0 + H),
        Vec2::new(0.0, b),
    ]);
    let t_mid = Polygon::from_slice(&[
        Vec2::new(0.5 + 2.0 / 3.0 * H, 1.0 + H),
        Vec2::new(1.5 + 2.0 * H, 1.0 + H),
        Vec2::new(0.0, b),
    ]);
    let t_mid2 = Polygon::from_slice(&[
        Vec2::new(1.5 + 2.0 * H, 1.0 + H),
        Vec2::new(1.0 + 4.0 / 3.0 * H, b),
        Vec2::new(0.0, b),
    ]);
    let t_top = Polygon::from_slice(&[
        Vec2::new(0.0, b),
        Vec2::new(1.0 + 4.0 / 3.0 * H, b),
        Vec2::new(1.5 + 2.0 * H, 3.0 + 3.0 * H),
    ]);
    let t_right = Polygon::from_slice(&[
        Vec2::new(1.5 + 2.0 * H, 1.0 + H),
        Vec2::new(1.5 + 2.0 * H, 3.0 + 3.0 * H),
        Vec2::new(1.0 + 4.0 / 3.0 * H, b),
    ]);
    LatticeTiling {
        id: "triakis-triangular",
        name: "Triakis Triangular (V3.12.12)",
        description: "Laves tiling V3.12.12 (triakis triangular tiling) — isosceles triangles on a triangular grid, dual of the truncated hexagonal tiling (p6m).",
        lattice: [Vec2::new(1.5 + 2.0 * H, 1.0 + H), Vec2::new(0.0, b)],
        cells: vec![
            Cell { poly: t_up, seed: 1 },
            Cell { poly: t_left, seed: 0 },
            Cell { poly: t_mid, seed: 2 },
            Cell { poly: t_mid2, seed: 1 },
            Cell { poly: t_top, seed: 0 },
            Cell { poly: t_right, seed: 2 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Laves,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// Laves tiling V3.4.6.4 (deltoidal trihexagonal tiling), wallpaper group
/// p6m.
///
/// The centroid dual of the rhombitrihexagonal tiling (`3.4.6.4` in
/// `archimedean.rs`): every primal vertex — where a hexagon, two squares,
/// and a triangle meet — becomes a kite with angles 60° (at a hexagon
/// center), 90°, 120° (at a triangle center), 90°. Same lattice as the
/// primal, a1 = (3/2 + √3/2, √3/2 + 1/2), a2 = (0, 1 + √3); each cell
/// holds 6 congruent kites (long sides √(1 + √3/2), short sides
/// √((2 + √3)/6)), area 6 · (2 + √3)/(2√3) = 3 + 2√3 = |det|. Six kites
/// close at 60° around every hexagon center, four at 90° around every
/// square center, three at 120° around every triangle center.
pub fn deltoidal_trihexagonal() -> LatticeTiling {
    let k0 = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(0.75 + 0.5 * H, 0.25 + 0.5 * H),
        Vec2::new(0.5 + H / 3.0, 0.5 + H),
        Vec2::new(0.0, 0.5 + H),
    ]);
    let k1 = Polygon::from_slice(&[
        Vec2::new(0.75 + 0.5 * H, 0.25 + 0.5 * H),
        Vec2::new(1.5 + H, 0.5 + H),
        Vec2::new(0.75 + 0.5 * H, 0.75 + 1.5 * H),
        Vec2::new(0.5 + H / 3.0, 0.5 + H),
    ]);
    let k2 = Polygon::from_slice(&[
        Vec2::new(1.0 + 2.0 / 3.0 * H, 1.0 + 2.0 * H),
        Vec2::new(1.5 + H, 1.0 + 2.0 * H),
        Vec2::new(1.5 + H, 1.5 + 3.0 * H),
        Vec2::new(0.75 + 0.5 * H, 1.25 + 2.5 * H),
    ]);
    let k3 = Polygon::from_slice(&[
        Vec2::new(0.0, 0.5 + H),
        Vec2::new(0.5 + H / 3.0, 0.5 + H),
        Vec2::new(0.75 + 0.5 * H, 0.75 + 1.5 * H),
        Vec2::new(0.0, 1.0 + 2.0 * H),
    ]);
    let k4 = Polygon::from_slice(&[
        Vec2::new(0.75 + 0.5 * H, 0.75 + 1.5 * H),
        Vec2::new(1.0 + 2.0 / 3.0 * H, 1.0 + 2.0 * H),
        Vec2::new(0.75 + 0.5 * H, 1.25 + 2.5 * H),
        Vec2::new(0.0, 1.0 + 2.0 * H),
    ]);
    let k5 = Polygon::from_slice(&[
        Vec2::new(0.75 + 0.5 * H, 0.75 + 1.5 * H),
        Vec2::new(1.5 + H, 0.5 + H),
        Vec2::new(1.5 + H, 1.0 + 2.0 * H),
        Vec2::new(1.0 + 2.0 / 3.0 * H, 1.0 + 2.0 * H),
    ]);
    LatticeTiling {
        id: "deltoidal-trihexagonal",
        name: "Deltoidal Trihexagonal (V3.4.6.4)",
        description: "Laves tiling V3.4.6.4 (deltoidal trihexagonal tiling) — kites (60/90/120/90 degrees), dual of the rhombitrihexagonal tiling (p6m).",
        lattice: [Vec2::new(1.5 + H, 0.5 + H), Vec2::new(0.0, 1.0 + 2.0 * H)],
        cells: vec![
            Cell { poly: k0, seed: 0 },
            Cell { poly: k1, seed: 1 },
            Cell { poly: k2, seed: 0 },
            Cell { poly: k3, seed: 2 },
            Cell { poly: k4, seed: 1 },
            Cell { poly: k5, seed: 2 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Laves,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// Laves tiling V4.6.12 (kisrhombille tiling), wallpaper group p6m.
///
/// The centroid dual of the truncated trihexagonal tiling (`4.6.12` in
/// `archimedean.rs`): every primal vertex — where a square, a hexagon,
/// and a dodecagon meet — becomes a 30-60-90 right triangle joining the
/// three face centers. Same lattice as the primal,
/// a1 = (3/2 + 3√3/2, 3/2 + √3/2), a2 = (0, 3 + √3); each cell holds 12
/// congruent right triangles (hypotenuse 1 + √3 between a hexagon and a
/// dodecagon center, legs (1 + √3)/2 and (1 + √3)√3/2), area
/// 12 · (1 + √3)²√3/8 = 9 + 6√3 = |det|. Twelve 30° angles close at
/// every dodecagon center, six 60° at every hexagon center, four 90° at
/// every square center. The dual adjacency graph is bipartite, so two
/// pattern colors suffice.
pub fn kisrhombille() -> LatticeTiling {
    let t0 = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(0.75 + 1.5 * H, 0.75 + 0.5 * H),
        Vec2::new(0.5 + H, 1.5 + H),
    ]);
    let t1 = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(0.5 + H, 1.5 + H),
        Vec2::new(0.0, 1.5 + H),
    ]);
    let t2 = Polygon::from_slice(&[
        Vec2::new(0.75 + 1.5 * H, 2.25 + 1.5 * H),
        Vec2::new(1.0 + 2.0 * H, 3.0 + 2.0 * H),
        Vec2::new(0.0, 3.0 + 2.0 * H),
    ]);
    let t3 = Polygon::from_slice(&[
        Vec2::new(0.75 + 1.5 * H, 2.25 + 1.5 * H),
        Vec2::new(1.5 + 3.0 * H, 1.5 + H),
        Vec2::new(1.0 + 2.0 * H, 3.0 + 2.0 * H),
    ]);
    let t4 = Polygon::from_slice(&[
        Vec2::new(1.5 + 3.0 * H, 1.5 + H),
        Vec2::new(1.5 + 3.0 * H, 3.0 + 2.0 * H),
        Vec2::new(1.0 + 2.0 * H, 3.0 + 2.0 * H),
    ]);
    let t5 = Polygon::from_slice(&[
        Vec2::new(1.0 + 2.0 * H, 3.0 + 2.0 * H),
        Vec2::new(1.5 + 3.0 * H, 3.0 + 2.0 * H),
        Vec2::new(1.5 + 3.0 * H, 4.5 + 3.0 * H),
    ]);
    let t6 = Polygon::from_slice(&[
        Vec2::new(1.0 + 2.0 * H, 3.0 + 2.0 * H),
        Vec2::new(1.5 + 3.0 * H, 4.5 + 3.0 * H),
        Vec2::new(0.75 + 1.5 * H, 3.75 + 2.5 * H),
    ]);
    let t7 = Polygon::from_slice(&[
        Vec2::new(0.0, 3.0 + 2.0 * H),
        Vec2::new(1.0 + 2.0 * H, 3.0 + 2.0 * H),
        Vec2::new(0.75 + 1.5 * H, 3.75 + 2.5 * H),
    ]);
    let t8 = Polygon::from_slice(&[
        Vec2::new(0.75 + 1.5 * H, 0.75 + 0.5 * H),
        Vec2::new(1.5 + 3.0 * H, 1.5 + H),
        Vec2::new(0.5 + H, 1.5 + H),
    ]);
    let t9 = Polygon::from_slice(&[
        Vec2::new(0.5 + H, 1.5 + H),
        Vec2::new(1.5 + 3.0 * H, 1.5 + H),
        Vec2::new(0.75 + 1.5 * H, 2.25 + 1.5 * H),
    ]);
    let t10 = Polygon::from_slice(&[
        Vec2::new(0.5 + H, 1.5 + H),
        Vec2::new(0.75 + 1.5 * H, 2.25 + 1.5 * H),
        Vec2::new(0.0, 3.0 + 2.0 * H),
    ]);
    let t11 = Polygon::from_slice(&[
        Vec2::new(0.0, 1.5 + H),
        Vec2::new(0.5 + H, 1.5 + H),
        Vec2::new(0.0, 3.0 + 2.0 * H),
    ]);
    LatticeTiling {
        id: "kisrhombille",
        name: "Kisrhombille (V4.6.12)",
        description: "Laves tiling V4.6.12 (kisrhombille tiling) — 30-60-90 right triangles, dual of the truncated trihexagonal tiling (p6m).",
        lattice: [Vec2::new(1.5 + 3.0 * H, 1.5 + H), Vec2::new(0.0, 3.0 + 2.0 * H)],
        cells: vec![
            Cell { poly: t0, seed: 0 },
            Cell { poly: t1, seed: 1 },
            Cell { poly: t2, seed: 0 },
            Cell { poly: t3, seed: 1 },
            Cell { poly: t4, seed: 0 },
            Cell { poly: t5, seed: 1 },
            Cell { poly: t6, seed: 0 },
            Cell { poly: t7, seed: 1 },
            Cell { poly: t8, seed: 1 },
            Cell { poly: t9, seed: 0 },
            Cell { poly: t10, seed: 1 },
            Cell { poly: t11, seed: 0 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Laves,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// Laves tiling V3.3.3.4.4 (prismatic pentagonal tiling), wallpaper group
/// cmm.
///
/// The centroid dual of the elongated triangular tiling (`3.3.3.4.4` in
/// `archimedean.rs`): every primal vertex — where three triangles and two
/// squares meet — becomes a pentagon joining the five face centers. Same
/// lattice as the primal, a1 = (1, 0), a2 = (1/2, 1 + √3/2); each cell
/// holds 2 congruent pentagons (three 120° angles at the triangle
/// centers, two 90° at the square centers; sides √3/3 twice,
/// 1/2 + √3/6 = (3+√3)/6 twice — no radical needed, its square is
/// (2+√3)/6 — plus a unit edge), area 2 · (1/2 + √3/4)
/// = 1 + √3/2 = |det|.
///
/// Two pattern colors are needed and `SeedPattern::Checker` provides
/// them: the two pentagon entries each take base parity (i + j) plus
/// seeds 0 and 2, a proper 4-coloring of the adjacency graph.
///
/// Pentagon census: realizes type 1 (repo lettering shifts k=1 up /
/// k=3 down: C + D = 180°, A + B + E = 360°), verified by test
/// `prismatic_pentagonal_is_pentagon_type_1` (M101).
pub fn prismatic_pentagonal() -> LatticeTiling {
    let p_up = Polygon::from_slice(&[
        Vec2::new(0.0, 2.0 / 3.0 * H),
        Vec2::new(0.5, H / 3.0),
        Vec2::new(1.0, 2.0 / 3.0 * H),
        Vec2::new(1.0, 0.5 + H),
        Vec2::new(0.0, 0.5 + H),
    ]);
    let p_down = Polygon::from_slice(&[
        Vec2::new(-0.5, -0.5),
        Vec2::new(0.5, -0.5),
        Vec2::new(0.5, H / 3.0),
        Vec2::new(0.0, 2.0 / 3.0 * H),
        Vec2::new(-0.5, H / 3.0),
    ]);
    LatticeTiling {
        id: "prismatic-pentagonal",
        name: "Prismatic Pentagonal (V3.3.3.4.4)",
        description: "Laves tiling V3.3.3.4.4 (prismatic pentagonal tiling) — pentagons with three 120-degree and two 90-degree angles, dual of the elongated triangular tiling (cmm).",
        lattice: [Vec2::new(1.0, 0.0), Vec2::new(0.5, 1.0 + H)],
        cells: vec![
            Cell { poly: p_up, seed: 0 },
            Cell { poly: p_down, seed: 2 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::Cmm,
        lattice_kind: LatticeKind::CenteredRectangular,
        category: TilingCategory::Laves,
        secondary: &[],
        seed_pattern: SeedPattern::Checker,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// Laves tiling V3.3.3.3.6 (floret pentagonal tiling), wallpaper group p6.
///
/// The centroid dual of the snub trihexagonal tiling (`3.3.3.3.6` in
/// `archimedean.rs`): every primal vertex — where four triangles and one
/// hexagon meet — becomes a pentagon joining the five face centers. Same
/// lattice as the primal, a1 = (5/2, −√3/2), a2 = (2, √3) (spacing √7);
/// each cell holds 6 congruent pentagons (one 60° angle at a hexagon
/// center, four 120° at triangle centers; sides √3/3 three times and
/// 2√3/3 twice), area 6 · 7√3/12 = 7√3/2 = |det|. Six pentagons
/// close at 60° around every hexagon center, three at 120° around every
/// triangle center — the "floret" petal rosettes. The tiling is chiral
/// (p6 has no mirrors), like its snub primal.
///
/// Pentagon census: realizes type 5 (repo lettering shift k=3:
/// B = 60°, E = 120°, a = b, d = e), verified by test
/// `floret_pentagonal_is_pentagon_type_5` (M101).
pub fn floret_pentagonal() -> LatticeTiling {
    let f0 = Polygon::from_slice(&[
        Vec2::new(1.0, -2.0 / 3.0 * H),
        Vec2::new(1.5, -H / 3.0),
        Vec2::new(1.5, H / 3.0),
        Vec2::new(1.0, 2.0 / 3.0 * H),
        Vec2::new(0.0, 0.0),
    ]);
    let f1 = Polygon::from_slice(&[
        Vec2::new(2.5, -H),
        Vec2::new(3.5, -H / 3.0),
        Vec2::new(3.5, H / 3.0),
        Vec2::new(3.0, 2.0 / 3.0 * H),
        Vec2::new(2.5, H / 3.0),
    ]);
    let f2 = Polygon::from_slice(&[
        Vec2::new(3.0, 2.0 / 3.0 * H),
        Vec2::new(3.5, H / 3.0),
        Vec2::new(4.5, H),
        Vec2::new(3.5, 5.0 / 3.0 * H),
        Vec2::new(3.0, 4.0 / 3.0 * H),
    ]);
    let f3 = Polygon::from_slice(&[
        Vec2::new(1.0, 2.0 / 3.0 * H),
        Vec2::new(1.5, H / 3.0),
        Vec2::new(2.0, 2.0 / 3.0 * H),
        Vec2::new(2.0, 2.0 * H),
        Vec2::new(1.0, 4.0 / 3.0 * H),
    ]);
    let f4 = Polygon::from_slice(&[
        Vec2::new(1.5, -H / 3.0),
        Vec2::new(2.5, -H),
        Vec2::new(2.5, H / 3.0),
        Vec2::new(2.0, 2.0 / 3.0 * H),
        Vec2::new(1.5, H / 3.0),
    ]);
    let f5 = Polygon::from_slice(&[
        Vec2::new(2.0, 2.0 / 3.0 * H),
        Vec2::new(2.5, H / 3.0),
        Vec2::new(3.0, 2.0 / 3.0 * H),
        Vec2::new(3.0, 4.0 / 3.0 * H),
        Vec2::new(2.0, 2.0 * H),
    ]);
    LatticeTiling {
        id: "floret-pentagonal",
        name: "Type 5 · Floret (V3.3.3.3.6)",
        description: "Laves tiling V3.3.3.3.6 (floret pentagonal tiling) — chiral pentagons (four 120-degree and one 60-degree angle), dual of the snub trihexagonal tiling (p6).",
        lattice: [Vec2::new(2.5, -H), Vec2::new(2.0, 2.0 * H)],
        cells: vec![
            Cell { poly: f0, seed: 0 },
            Cell { poly: f1, seed: 1 },
            Cell { poly: f2, seed: 2 },
            Cell { poly: f3, seed: 3 },
            Cell { poly: f4, seed: 4 },
            Cell { poly: f5, seed: 5 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Laves,
        secondary: &[TilingCategory::PentagonTypes],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
