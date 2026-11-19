//! Hankin's method (Kaplan dissertation §3.4) — Hankin rosette khatam (4.8.8).
//!
//! Constructed at runtime by the ported inference engine in
//! [`crate::hankin`]: rays from every substrate-edge midpoint at Kaplan
//! contact angle 67.5° to the edge, no truncation (single-point contacts, δ = 0), greedy min-cost
//! pairing (Kaplan §3.4: pairs sorted by total induced segment length),
//! then a planar arrangement of the paired segments against the tile
//! boundary gives the face partition. Template constants and the test
//! expectations in this file were machine-transcribed from the validated
//! 40-dps Python prototype (m94_emit.py); nothing hand-copied.
//!
//! Validation (Python battery, mpmath 40 dps, thresholds 1e-25):
//!   - pairing complete and tie-insensitive on every tile class,
//!   - face partition exact: |Σ faces − tile area| < 1e-25 per class,
//!   - planar patch: X-consistency at every valence-4 contact,
//!     400-point coverage probe exact,
//!   - torus (one-cell fold): valence {2:12, 4:6}, all strands close (4 strands of 12+12+12+12 segments),
//!   - symmetry probes: p4m confirmed exactly on the torus segment
//!     multiset.
//!
//! ## Configuration
//!
//! Substrate: truncated square (4.8.8), octagon edge 1, lattice A·Z² with
//! A = 1+√2 — octagon centred (A/2, A/2), substrate square replaced by the
//! diamond of the dual. At θK = 67.5° the octagon's greedy pairing is the
//! adjacent-edge rosette ring: each ray meets its neighbour's ray from the
//! adjacent edge, producing the 8-ray rosette (8 petals around a central
//! octagon, tips at 45°); the diamond gets a 4-pointed star. This is the
//! dense khatam-style column of Kaplan's Figure 3.3 (third column), which
//! the engine reproduces at exact figure scale.
//!
//! Distinct from the registered `pic-4-8-8` (M53): that is Hankin's
//! REFLECTION variant at θ = 45° producing {8/2} midpoint chords; this is
//! the greedy-inference rosette at θK = 67.5° — different construction
//! and different partition.
//!
//! Wallpaper group p4m (rot90 + mirrors verified exactly on the torus
//! segment multiset).

use tess_core::TilingCategory;
use tess_core::geometry::Vec2;
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::hankin;
use crate::lattice::{similarity_map, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// √2/2 (bit-exact as SQRT_2/2: division by 2 is exact in f64).
const RT2H: f64 = core::f64::consts::SQRT_2 / 2.0;

/// Kaplan contact angle from the edge (degrees). Test-only:
/// tests pin THETA_E_DEG == 90 − THETA_K_DEG.
#[cfg(test)]
pub(crate) const THETA_K_DEG: f64 = 67.5;
/// Engine ray angle from the edge normal's frame: 90° − θK.
pub(crate) const THETA_E_DEG: f64 = 22.5;
/// Two-point truncation as a fraction of the substrate edge length.
pub(crate) const DELTA: f64 = 0.0;

/// Substrate tile class 0 ('oct'), CCW, in-cell position.
pub(crate) static TILE_0: [Vec2; 8] = [Vec2::new(RT2H, 0.0), Vec2::new(1.7071067811865475, 0.0), Vec2::new(2.414213562373095, RT2H), Vec2::new(2.414213562373095, 1.7071067811865475), Vec2::new(1.7071067811865475, 2.414213562373095), Vec2::new(RT2H, 2.414213562373095), Vec2::new(0.0, 1.7071067811865475), Vec2::new(0.0, RT2H)];
/// Substrate tile class 1 ('sq'), CCW, in-cell position.
pub(crate) static TILE_1: [Vec2; 4] = [Vec2::new(RT2H, 0.0), Vec2::new(0.0, RT2H), Vec2::new(-RT2H, 0.0), Vec2::new(0.0, -RT2H)];

/// Template tiles in canonical class order.
pub(crate) fn tiles() -> Vec<&'static [Vec2]> {
    vec![&TILE_0, &TILE_1]
}

/// Expected canonical faces for tile class 0 (machine-transcribed).
#[cfg(test)]
pub(crate) static EXPECTED_0_VERTS: &[f64] = &[
    0.0, RT2H, 0.3535533905932738, 0.3535533905932738, 0.6035533905932737, 0.9571067811865476,
    0.0, 1.2071067811865475, 0.0, 1.2071067811865475, 0.6035533905932737, 1.4571067811865475,
    0.3535533905932738, 2.0606601717798214, 0.0, 1.7071067811865475, 0.3535533905932738, 0.3535533905932738,
    RT2H, 0.0, 1.2071067811865475, 0.0, 0.9571067811865476, 0.6035533905932737,
    0.3535533905932738, 2.0606601717798214, 0.9571067811865476, 1.8106601717798212, 1.2071067811865475, 2.414213562373095,
    RT2H, 2.414213562373095, 1.2071067811865475, 0.0, 1.7071067811865475, 0.0,
    2.0606601717798214, 0.3535533905932738, 1.4571067811865475, 0.6035533905932737, 1.2071067811865475, 2.414213562373095,
    1.4571067811865475, 1.8106601717798212, 2.0606601717798214, 2.0606601717798214, 1.7071067811865475, 2.414213562373095,
    1.8106601717798212, 0.9571067811865476, 2.0606601717798214, 0.3535533905932738, 2.414213562373095, RT2H,
    2.414213562373095, 1.2071067811865475, 1.8106601717798212, 1.4571067811865475, 2.414213562373095, 1.2071067811865475,
    2.414213562373095, 1.7071067811865475, 2.0606601717798214, 2.0606601717798214, 0.0, 1.2071067811865475,
    0.6035533905932737, 0.9571067811865476, 0.3535533905932738, 0.3535533905932738, 0.9571067811865476, 0.6035533905932737,
    1.2071067811865475, 0.0, 1.4571067811865475, 0.6035533905932737, 2.0606601717798214, 0.3535533905932738,
    1.8106601717798212, 0.9571067811865476, 2.414213562373095, 1.2071067811865475, 1.8106601717798212, 1.4571067811865475,
    2.0606601717798214, 2.0606601717798214, 1.4571067811865475, 1.8106601717798212, 1.2071067811865475, 2.414213562373095,
    0.9571067811865476, 1.8106601717798212, 0.3535533905932738, 2.0606601717798214, 0.6035533905932737, 1.4571067811865475,
];
#[cfg(test)]
pub(crate) static EXPECTED_0_LENS: &[usize] = &[4, 4, 4, 4, 4, 4, 4, 4, 16];
/// Expected canonical faces for tile class 1 (machine-transcribed).
#[cfg(test)]
pub(crate) static EXPECTED_1_VERTS: &[f64] = &[
    -RT2H, 0.0, -0.3535533905932738, -0.3535533905932738, -0.20710678118654752, 0.0,
    -0.3535533905932738, 0.3535533905932738, -0.3535533905932738, -0.3535533905932738, 0.0, -RT2H,
    0.3535533905932738, -0.3535533905932738, 0.0, -0.20710678118654752, -0.3535533905932738, 0.3535533905932738,
    0.0, 0.20710678118654752, 0.3535533905932738, 0.3535533905932738, 0.0, RT2H,
    0.20710678118654752, 0.0, 0.3535533905932738, -0.3535533905932738, RT2H, 0.0,
    0.3535533905932738, 0.3535533905932738, -0.3535533905932738, -0.3535533905932738, 0.0, -0.20710678118654752,
    0.3535533905932738, -0.3535533905932738, 0.20710678118654752, 0.0, 0.3535533905932738, 0.3535533905932738,
    0.0, 0.20710678118654752, -0.3535533905932738, 0.3535533905932738, -0.20710678118654752, 0.0,
];
#[cfg(test)]
pub(crate) static EXPECTED_1_LENS: &[usize] = &[4, 4, 4, 4, 8];
/// Expected per-face seeds across all classes, canonical order.
#[cfg(test)]
pub(crate) static EXPECTED_SEEDS: &[u32] = &[0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 2, 3];

/// Hankin rosette khatam (4.8.8).
pub fn hankin_4_8_8_rosette() -> LatticeTiling {
    let (cells, report) = hankin::build_cells(&tiles(), THETA_E_DEG, DELTA);
    debug_assert!(report.complete, "pairing incomplete");
    debug_assert!(
        report.per_tile.iter().all(|t| (t.area_sum - t.tile_area).abs() < 1e-9),
        "face partition inexact"
    );
    LatticeTiling {
        id: "hankin-4-8-8-rosette",
        name: "Hankin rosette khatam (4.8.8)",
        description: "Hankin greedy-inference motif on the truncated-square (4.8.8) substrate at Kaplan contact angle 67.5°, single-point contacts: octagon 8-ray rosette (45° tips) plus diamond 4-pointed stars, exact per-tile face partitions, wallpaper group p4m.",
        lattice: [Vec2::new(2.414213562373095, 0.0), Vec2::new(0.0, 2.414213562373095)],
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
