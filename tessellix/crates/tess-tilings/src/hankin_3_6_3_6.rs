//! Hankin's method (Kaplan dissertation §3.4) — Hankin sixfold hexagram
//! (3.6.3.6).
//!
//! Constructed at runtime by the ported inference engine in
//! [`crate::hankin`]: rays from every substrate-edge midpoint at Kaplan
//! contact angle 60° to the edge, no truncation (single-point contacts,
//! δ = 0), greedy min-cost pairing (Kaplan §3.4: pairs sorted by total
//! induced segment length), then a planar arrangement of the paired
//! segments against the tile boundary gives the face partition.
//!
//! Unlike the M94 trio, this substrate has no 40-dps Python prototype:
//! the template tiles and the expected-face constants below were
//! machine-emitted from the Rust engine itself, so the test battery is
//! a self-consistency pin — re-running `build_cells` on these tiles
//! must reproduce these faces exactly.
//!
//! Angle choice: the hexagon tile pairs completely at every probed
//! contact angle (12 rays, 6 pairs); the equilateral triangle tiles
//! pair with none of them (6 rays, 0 pairs), so their motif stays the
//! original triangle. Measured on the emitted cells (minimum interior
//! angle of the hexagon's central core face): θK = 30° closes the core
//! to a hexagon with 120° tips — no star; θK = 45° gives a
//! truncated-hexagon 12-gon with 90° tips — no star; θK = 60° gives the
//! classical regular hexagram: a 12-gon core with 60° tips and 240°
//! notches (vertices alternating on circles of radius 1/2 and √3/2);
//! θK = 67.5° gives a sharper star variant with 45° notches (probed,
//! not registered); θK = 90° is degenerate — the star core collapses
//! and the hexagon splits into 6 quadrilaterals plus a central
//! equilateral triangle. θK = 60° is chosen: the most classical
//! sixfold star on the trihexagonal substrate and the largest minimum
//! interior angle (60°) among the star candidates — no razor-thin
//! slivers. Per the PROPOSED.md §22a star heuristic (s = 2 when n > 6
//! and θ > 2π/n), n = 6 is not a star-trigger polygon in the formal
//! sense: the hexagon star first appears at the critical angle
//! 60° = 2π/6, and its regular {6/2} endpoint is exactly this
//! configuration.
//!
//! Validation (engine battery, f64, thresholds 1e-9):
//!   - pairing: hexagon tile complete (12 rays, 6 pairs); each triangle
//!     tile unpaired (6 rays, 0 pairs, single-face motif),
//!   - face partition exact: |Σ faces − tile area| < 1e-9 per class,
//!   - 9 faces per lattice cell, seeds {0: 6 lozenge rhombi,
//!     1: one 12-gon star core, 2: two triangle motifs},
//!   - symmetry probe: p6m window check on a ±5-lattice window — the
//!     517-face multiset (faces whose centroids fall in the radius-8
//!     disk about the origin) is invariant under all 12 D6 elements
//!     (rot k·60° + M_0 mirrors), 0 key mismatches at 1e-6
//!     quantization.
//!
//! ## Configuration
//!
//! Substrate: trihexagonal (3.6.3.6), unit flat-top hexagons on the
//! lattice a1 = (2, 0), a2 = (1, √3), per-cell area 2√3 =
//! |det(a1,a2)|. At θK = 60° each hexagon yields a 12-gon hexagram
//! core plus six 60°-lozenge rhombi (7 faces); the two triangle tiles
//! are unpaired and stay single equilateral-triangle faces — 9 faces
//! per lattice cell.
//!
//! Distinct from the registered `pic-3-6-3-6` (δ = 60° reflection rule
//! on the 3.6.3.6 substrate): same substrate, different construction
//! (greedy inference vs reflection chords) and different partition.
//!
//! Wallpaper group p6m (rot60 + mirror verified on the ±5-lattice
//! window face multiset, all 12 D6 elements, 0 mismatches).

use tess_core::TilingCategory;
use tess_core::geometry::Vec2;
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::hankin;
use crate::lattice::{similarity_map, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// Kaplan contact angle from the edge (degrees). Test-only:
/// tests pin THETA_E_DEG == 90 − THETA_K_DEG.
#[cfg(test)]
pub(crate) const THETA_K_DEG: f64 = 60.0;
/// Engine ray angle from the edge normal's frame: 90° − θK.
pub(crate) const THETA_E_DEG: f64 = 30.0;
/// Two-point truncation as a fraction of the substrate edge length.
pub(crate) const DELTA: f64 = 0.0;

/// Substrate tile class 0 ('hex'), CCW, in-cell position.
pub(crate) static TILE_0: [Vec2; 6] = [
    Vec2::new(1.0, 0.0),
    Vec2::new(0.5, 0.8660254037844386),
    Vec2::new(-0.5, 0.8660254037844386),
    Vec2::new(-1.0, 0.0),
    Vec2::new(-0.5, -0.8660254037844386),
    Vec2::new(0.5, -0.8660254037844386),
];
/// Substrate tile class 1 ('tri'), CW, in-cell position (the engine
/// canonicalizes emitted faces to CCW).
pub(crate) static TILE_1: [Vec2; 3] =
    [Vec2::new(1.0, 0.0), Vec2::new(0.5, 0.8660254037844386), Vec2::new(1.5, 0.8660254037844386)];
/// Substrate tile class 2 ('tri'), CW, in-cell position.
pub(crate) static TILE_2: [Vec2; 3] =
    [Vec2::new(0.5, 0.8660254037844386), Vec2::new(-0.5, 0.8660254037844386), Vec2::new(0.0, 1.7320508075688772)];

/// Template tiles in canonical class order.
pub(crate) fn tiles() -> Vec<&'static [Vec2]> {
    vec![&TILE_0, &TILE_1, &TILE_2]
}

/// Expected canonical faces for tile class 0 (machine-emitted from the
/// Rust engine at θe = 30°).
#[cfg(test)]
pub(crate) static EXPECTED_0_VERTS: &[f64] = &[
    -1.0, 0.0, -0.75, -0.4330127018922193, -0.5, 0.0, -0.75, 0.4330127018922193,
    -0.75, -0.4330127018922193, -0.5, -0.8660254037844386, 0.0, -0.8660254037844386, -0.25, -0.4330127018922192,
    -0.75, 0.4330127018922193, -0.24999999999999997, 0.43301270189221924, 0.0, 0.8660254037844386, -0.5, 0.8660254037844386,
    0.0, -0.8660254037844386, 0.5, -0.8660254037844386, 0.75, -0.4330127018922193, 0.24999999999999997, -0.43301270189221924,
    0.0, 0.8660254037844386, 0.25, 0.4330127018922192, 0.75, 0.4330127018922193, 0.5, 0.8660254037844386,
    0.5, 0.0, 0.75, -0.4330127018922193, 1.0, 0.0, 0.75, 0.4330127018922193,
    -0.75, -0.4330127018922193, -0.25, -0.4330127018922192, 0.0, -0.8660254037844386, 0.24999999999999997, -0.43301270189221924, 0.75, -0.4330127018922193, 0.5, 0.0, 0.75, 0.4330127018922193, 0.25, 0.4330127018922192, 0.0, 0.8660254037844386, -0.24999999999999997, 0.43301270189221924, -0.75, 0.4330127018922193, -0.5, 0.0,
];
#[cfg(test)]
pub(crate) static EXPECTED_0_LENS: &[usize] = &[4, 4, 4, 4, 4, 4, 12];
/// Expected canonical faces for tile class 1 (machine-emitted).
#[cfg(test)]
pub(crate) static EXPECTED_1_VERTS: &[f64] =
    &[0.5, 0.8660254037844386, 1.0, 0.0, 1.5, 0.8660254037844386];
#[cfg(test)]
pub(crate) static EXPECTED_1_LENS: &[usize] = &[3];
/// Expected canonical faces for tile class 2 (machine-emitted).
#[cfg(test)]
pub(crate) static EXPECTED_2_VERTS: &[f64] =
    &[-0.5, 0.8660254037844386, 0.5, 0.8660254037844386, 0.0, 1.7320508075688772];
#[cfg(test)]
pub(crate) static EXPECTED_2_LENS: &[usize] = &[3];
/// Expected per-face seeds across all classes, canonical order.
#[cfg(test)]
pub(crate) static EXPECTED_SEEDS: &[u32] = &[0, 0, 0, 0, 0, 0, 1, 2, 2];

/// Hankin sixfold hexagram (3.6.3.6).
pub fn hankin_3_6_3_6() -> LatticeTiling {
    let (cells, report) = hankin::build_cells(&tiles(), THETA_E_DEG, DELTA);
    debug_assert!(
        report.per_tile[0].complete,
        "hexagon pairing must be complete"
    );
    debug_assert_eq!(
        report.per_tile[1].faces, 1,
        "unpaired triangle motif must stay a single face"
    );
    debug_assert_eq!(
        report.per_tile[2].faces, 1,
        "unpaired triangle motif must stay a single face"
    );
    debug_assert!(
        report
            .per_tile
            .iter()
            .all(|t| (t.area_sum - t.tile_area).abs() < 1e-9),
        "face partition inexact"
    );
    LatticeTiling {
        id: "hankin-3-6-3-6",
        name: "Hankin sixfold hexagram (3.6.3.6)",
        description: "Hankin greedy-inference motif on the trihexagonal (3.6.3.6) substrate at Kaplan contact angle 60°, single-point contacts: a regular hexagram per hexagon (12-gon core plus six 60°-lozenge rhombi), unpaired equilateral triangle tiles — 9 faces per lattice cell, wallpaper group p6m.",
        lattice: [Vec2::new(2.0, 0.0), Vec2::new(1.0, 1.7320508075688772)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Islamic,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
