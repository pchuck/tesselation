//! Hankin rosette-transform khatam (4.8.8) — the Najm ↔ Hankin bridge
//! (Kaplan dissertation §3.8) materialized as a registered tiling.
//!
//! Construction: the truncated-square (4.8.8) substrate is first passed
//! through the rosette transform (`crate::rosette_transform`), and
//! Hankin's motif inference (`crate::hankin`) is then run on the result —
//! Kaplan's Figure 3.33(d) construction: "first compute the rosette
//! transform of the tiling, and run Hankin's method on the result".
//!
//! ## The rosette transform on the 4.8.8 substrate
//!
//! * **Octagon** (regular 8-gon, edge 1, circumradius
//!   r = 1/(2·sin(π/8)) ≈ 1.306563): regular-polygon rule (Figure 3.30) —
//!   concentric inner octagon rotated
//!   by π/8 at radius r′ = r·cos(π/8)/(1+sin(π/8)) ≈ 0.873057, spokes of
//!   length exactly s′/2 (s′ = 2r′sin(π/8) ≈ 0.668740, spoke ≈ 0.334050)
//!   to the eight edge midpoints → 1 inner octagon + 8 pentagons.
//! * **Diamond** (regular 4-gon): the n ≥ 5 rule does not apply; the
//!   perpendicular-bisector rule (Figure 3.31) takes over. All four
//!   bisectors pass through the centre, so the six pairwise intersections
//!   form a single cluster that averages to the centre (the documented
//!   heuristic, exercised here) → 4 small squares of side 1/2.
//!
//! 13 cells per lattice cell ⟨(1+√2,0),(0,1+√2)⟩ (det 3+2√2); the
//! transformed table is edge-to-edge (Σ areas = |det|, every interior
//! edge shared by exactly two cells, no T-junctions — pinned in tests).
//!
//! ## Inference
//!
//! Hankin inference at Kaplan contact angle θK (δ = 0, single-point
//! contacts) is then run on the 13 transformed tiles. θK was selected
//! from {30°, 45°, 60°}:
//!
//! * **θK = 30°** is disqualified — the greedy pairing is *incomplete*
//!   (`build_cells` reports unpaired rays), so no valid tiling results.
//! * **θK = 45°** and **θK = 60°** are both complete, p4m-exact on the
//!   cell table (rot90 + x-mirror invariant, pinned in tests), with
//!   zero slivers and per-tile area closure ≤ 2.3e-16.
//!
//! **θK = 45° is chosen** (93 cells: 16×3-gon, 36×4-gon, 32×5-gon,
//! 8×6-gon, 1×16-gon). At 45° the inference preserves the pentagon
//! rosette rings of the transform (32 five-gons, one per transformed
//! octagon edge-pair), and the renders show the crisp eight-petal
//! rosettes around the inner-octagon cores with square clusters at the
//! diamond sites — the structure closest to Kaplan's Figure 3.33(d).
//! At 60° the pentagons largely collapse into squares (64×4-gon) plus
//! eight elongated 10-gons, reading as a square star-web rather than
//! a rosette. (Render comparison motivated the pick; the programmatic
//! gates that both candidates satisfy are the test pins below.)
//!
//! Distinct from `hankin-4-8-8-rosette` (M94, inference directly on the
//! 4.8.8 substrate at θK = 67.5°) and from `pic-4-8-8` (M53, the Hankin
//! PIC strapwork layer): this is the two-step Najm → transform →
//! inference pipeline.

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::hankin;
use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};
use crate::rosette_transform;

/// √2/2 (bit-exact as SQRT_2/2: division by 2 is exact in f64).
const RT2H: f64 = core::f64::consts::SQRT_2 / 2.0;

/// Kaplan contact angle from the edge (degrees). Test-only:
/// tests pin THETA_E_DEG == 90 − THETA_K_DEG.
#[cfg(test)]
pub(crate) const THETA_K_DEG: f64 = 45.0;
/// Engine ray angle from the edge normal's frame: 90° − θK.
pub(crate) const THETA_E_DEG: f64 = 45.0;
/// Selected-θ inference census (machine-transcribed, cell order):
/// 16 triangles, 36 quads, 32 pentagons, 8 hexagons, one 16-gon.
#[cfg(test)]
pub(crate) static EXPECTED_LENS: &[usize] = &[4, 4, 4, 5, 5, 5, 5, 6, 4, 4, 4, 5, 5, 5, 5, 6, 4, 4, 4, 5, 5, 5, 5, 6, 4, 4, 4, 5, 5, 5, 5, 6, 4, 4, 4, 5, 5, 5, 5, 6, 4, 4, 4, 5, 5, 5, 5, 6, 4, 4, 4, 5, 5, 5, 5, 6, 4, 4, 4, 5, 5, 5, 5, 6, 4, 4, 4, 4, 4, 4, 4, 4, 16, 3, 3, 3, 3, 4, 3, 3, 3, 3, 4, 3, 3, 3, 3, 4, 3, 3, 3, 3, 4];
/// Two-point truncation as a fraction of the substrate edge length.
pub(crate) const DELTA: f64 = 0.0;

/// Substrate tile class 0 ('oct'), CCW, in-cell position (identical to
/// `crate::hankin_4_8_8::TILE_0`: the same 4.8.8 substrate).
pub(crate) static TILE_0: [Vec2; 8] = [Vec2::new(RT2H, 0.0), Vec2::new(1.7071067811865475, 0.0), Vec2::new(2.414213562373095, RT2H), Vec2::new(2.414213562373095, 1.7071067811865475), Vec2::new(1.7071067811865475, 2.414213562373095), Vec2::new(RT2H, 2.414213562373095), Vec2::new(0.0, 1.7071067811865475), Vec2::new(0.0, RT2H)];
/// Substrate tile class 1 ('sq'), CCW, in-cell position (identical to
/// `crate::hankin_4_8_8::TILE_1`).
pub(crate) static TILE_1: [Vec2; 4] = [Vec2::new(RT2H, 0.0), Vec2::new(0.0, RT2H), Vec2::new(-RT2H, 0.0), Vec2::new(0.0, -RT2H)];

/// Lattice basis: A·Z².
pub(crate) const LATTICE: [Vec2; 2] = [Vec2::new(2.414213562373095, 0.0), Vec2::new(0.0, 2.414213562373095)];

/// The 4.8.8 substrate cell table (octagon + diamond, in-cell positions),
/// the input to the rosette transform.
pub(crate) fn substrate_cells() -> [Cell; 2] {
    [
        Cell {
            poly: Polygon::from_slice(&TILE_0),
            seed: 0,
        },
        Cell {
            poly: Polygon::from_slice(&TILE_1),
            seed: 1,
        },
    ]
}

/// The rosette-transformed 4.8.8 cell table and its per-tile report:
/// 13 cells per lattice cell — 1 inner octagon + 8 pentagons (from the
/// octagon, regular-polygon rule) + 4 small squares of side 1/2 (from
/// the diamond, bisector rule with the centre cluster). The table and
/// report are what the edge-to-edge / area-census / exactly-once tests
/// pin.
pub(crate) fn transformed_cells() -> (Vec<Cell>, rosette_transform::TransformReport) {
    rosette_transform::rosette_transform(&substrate_cells(), LATTICE)
}

/// Expected per-cell seeds of the inference output, in cell order
/// (machine-transcribed from the engine; pins the congruence clustering
/// and the tile order).
#[cfg(test)]
pub(crate) static EXPECTED_SEEDS: &[u32] = &[0, 1, 1, 2, 3, 2, 3, 4, 0, 1, 1, 2, 3, 2, 3, 4, 1, 0, 1, 2, 3, 3, 2, 4, 1, 0, 1, 2, 3, 3, 2, 4, 1, 0, 1, 2, 3, 3, 2, 4, 1, 0, 1, 3, 2, 3, 2, 4, 1, 1, 0, 3, 2, 3, 2, 4, 1, 1, 0, 3, 2, 3, 2, 4, 5, 5, 5, 5, 5, 5, 5, 5, 6, 7, 7, 7, 7, 8, 7, 7, 7, 7, 8, 7, 7, 7, 7, 8, 7, 7, 7, 7, 8];

/// Hankin rosette-transform khatam (4.8.8).
pub fn hankin_4_8_8_rosette_transform() -> LatticeTiling {
    let (transformed, trep) = transformed_cells();
    debug_assert_eq!(
        transformed.len(),
        13,
        "transformed table must have 13 cells (1 oct + 8 pent + 4 sq)"
    );
    debug_assert!(
        trep.per_tile.iter().all(|t| (t.area_sum - t.tile_area).abs() < 1e-9),
        "rosette transform face partition inexact"
    );
    debug_assert!(
        (trep.per_tile.iter().map(|t| t.area_sum).sum::<f64>() - trep.lattice_area).abs() < 1e-9,
        "transformed areas must sum to |det|"
    );
    let tiles: Vec<&[Vec2]> = transformed.iter().map(|c| c.poly.pts.as_slice()).collect();
    let (cells, report) = hankin::build_cells(&tiles, THETA_E_DEG, DELTA);
    debug_assert!(report.complete, "pairing incomplete");
    debug_assert!(
        report.per_tile.iter().all(|t| (t.area_sum - t.tile_area).abs() < 1e-9),
        "inference face partition inexact"
    );
    debug_assert_eq!(
        cells.len(),
        93,
        "inference table must have 93 cells (16×3 + 36×4 + 32×5 + 8×6 + 1×16)"
    );
    let det = LATTICE[0].cross(LATTICE[1]).abs();
    let area_sum: f64 = cells.iter().map(|c| c.poly.area().abs()).sum();
    debug_assert!(
        (area_sum - det).abs() < 1e-9,
        "inference areas {area_sum} must sum to |det| {det}"
    );
    LatticeTiling {
        id: "hankin-4-8-8-rosette-transform",
        name: "Hankin rosette-transform khatam (4.8.8)",
        description: "Rosette transform (Kaplan §3.8) on the 4.8.8 substrate — octagon to inner octagon + 8 pentagons, diamond to 4 small squares (centre-clustered bisectors) — then Hankin motif inference at contact angle 45°, 13 transformed tiles per cell, exact per-tile face partitions, wallpaper group p4m.",
        lattice: LATTICE,
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P4m,
        lattice_kind: LatticeKind::Square,
        category: TilingCategory::Islamic,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
