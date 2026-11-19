//! Hankin's method (Kaplan dissertation §3.4) — Hankin two-point square weave (4.4.4.4).
//!
//! Constructed at runtime by the ported inference engine in
//! [`crate::hankin`]: rays from every substrate-edge midpoint at Kaplan
//! contact angle 45° to the edge, two-point truncation δ = 0.5 edge
//! lengths, greedy min-cost pairing (Kaplan §3.4: pairs sorted by total
//! induced segment length), then a planar arrangement of the paired
//! segments against the tile boundary gives the face partition.
//!
//! This substrate has no 40-dps Python prototype: the template tiles
//! and the expected-face constants below were machine-emitted from the
//! Rust engine itself, so the test battery is a self-consistency pin —
//! re-running `build_cells` on these tiles must reproduce these faces
//! exactly.
//!
//! Angle choice: all three candidate contact angles pair completely on
//! the square substrate (8 rays, 4 pairs per tile). θK = 30° is the
//! flat limiting case — the core (an 8-gon) closes at 90° tips with
//! quad petals; θK = 60° is spiky — 60° tips, nine faces per tile.
//! θK = 45° gives the classical decagonal star: a 10-gon core with
//! 90° tips plus two pentagonal petals and four corner triangles —
//! the mid-"shine" choice, symmetric θK = θE like the two-point hex
//! entry. Measured on the emitted cells: core minimum interior angle
//! 90.0° at θK = 45° (90.0° at θK = 30°, 60.0° at θK = 60°).
//!
//! Validation (engine battery, f64, thresholds 1e-9):
//!   - pairing complete: (8 rays, 4 pairs), face partition exact:
//!     |Σ faces − tile area| < 1e-9,
//!   - 7 faces per lattice cell, seeds {0: 4 corner triangles,
//!     1: two pentagonal petals, 2: one 10-gon star core},
//!   - symmetry probe: cmm element battery on radius-4 invariant
//!     disks — both diagonal mirror families (lines x − y = k and
//!     x + y = k, k ∈ ℤ) pass on two instances each; 2-fold points
//!     verified at (0,0), (0.5,0.5) (on mirrors, at the family
//!     intersections) and at (0.5,0), (0,0.5) (off-mirror edge
//!     midpoints); no 4-fold points at any candidate orbit
//!     (corner / edge midpoint / cell center all fail) and no
//!     half-integer or axis-parallel mirrors — all as cmm (2*22)
//!     requires, 0 unexpected outcomes, 1e-6 quantization.
//!
//! ## Configuration
//!
//! Substrate: square grid (4.4.4.4), unit square CCW, lattice
//! ⟨(1,0),(0,1)⟩, area 1 per cell. Deviation from the M94 trio and
//! the M99 3.12.12 entry: this substrate is defined by `square::spec`
//! (the Uniform family), not by the Archimedean substrate family in
//! `archimedean.rs`. Lattice-kind note: cmm is a centered-rectangular
//! crystallographic family (the "c" of 2*22) — the translation lattice
//! here is the square lattice ⟨(1,0),(0,1)⟩, which is the C-centered
//! 45°-rotated special case (primitive vectors (√2/2,±√2/2) with the
//! C-center (√2/2,0)), so the entry declares
//! `LatticeKind::CenteredRectangular` like every other cmm entry in
//! the registry.
//!
//! At θK = 45°, δ = 0.5 the square yields a decagonal star: four
//! corner triangles, two pentagonal petals, one 10-gon core —
//! 7 faces per lattice cell.
//!
//! Wallpaper group cmm (2*22): mirror families along the two
//! diagonal directions of the square lattice (x − y = k and
//! x + y = k, k ∈ ℤ); 2-fold points at every point of
//! (1/2)ℤ × (1/2)ℤ — on-mirror at lattice vertices and cell centers,
//! off-mirror at edge midpoints; no 4-fold points. (The p4m
//! hypothesis is falsified: the 4-fold probes at the corner, edge
//! midpoint, and cell-center orbits all fail, and the axis-parallel
//! mirrors do not hold.)

use tess_core::TilingCategory;
use tess_core::geometry::Vec2;
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::hankin;
use crate::lattice::{similarity_map, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// Kaplan contact angle from the edge (degrees). Test-only:
/// tests pin THETA_E_DEG == 90 − THETA_K_DEG.
#[cfg(test)]
pub(crate) const THETA_K_DEG: f64 = 45.0;
/// Engine ray angle from the edge normal's frame: 90° − θK.
pub(crate) const THETA_E_DEG: f64 = 45.0;
/// Two-point truncation as a fraction of the substrate edge length.
pub(crate) const DELTA: f64 = 0.5;

/// Substrate tile class 0 ('sq'), CCW, in-cell position.
pub(crate) static TILE_0: [Vec2; 4] =
    [Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0)];

/// Template tiles in canonical class order.
pub(crate) fn tiles() -> Vec<&'static [Vec2]> {
    vec![&TILE_0]
}

/// Expected canonical faces for tile class 0 (machine-emitted from the
/// Rust engine at θe = 45°).
#[cfg(test)]
pub(crate) static EXPECTED_0_VERTS: &[f64] = &[
    0.0, 0.25, 0.2500000000000001, 0.5, 0.0, 0.75,
    0.25, 0.0, 0.75, 0.0, 0.5, 0.25000000000000006,
    0.25, 1.0, 0.5, 0.75, 0.75, 1.0,
    0.7499999999999999, 0.5, 1.0, 0.25, 1.0, 0.75,
    0.0, 0.75, 0.2500000000000001, 0.5, 0.5, 0.75, 0.25, 1.0, 0.0, 1.0,
    0.5, 0.25000000000000006, 0.75, 0.0, 1.0, 0.0, 1.0, 0.25, 0.7499999999999999, 0.5,
    0.0, 0.0, 0.25, 0.0, 0.5, 0.25000000000000006, 0.7499999999999999, 0.5, 1.0, 0.75, 1.0, 1.0, 0.75, 1.0, 0.5, 0.75, 0.2500000000000001, 0.5, 0.0, 0.25,
];
#[cfg(test)]
pub(crate) static EXPECTED_0_LENS: &[usize] = &[3, 3, 3, 3, 5, 5, 10];
/// Expected per-face seeds across all classes, canonical order.
#[cfg(test)]
pub(crate) static EXPECTED_SEEDS: &[u32] = &[0, 0, 0, 0, 1, 1, 2];

/// Hankin two-point square weave (4.4.4.4).
pub fn hankin_square_two_point() -> LatticeTiling {
    let (cells, report) = hankin::build_cells(&tiles(), THETA_E_DEG, DELTA);
    debug_assert!(report.complete, "pairing incomplete");
    debug_assert!(
        report.per_tile.iter().all(|t| (t.area_sum - t.tile_area).abs() < 1e-9),
        "face partition inexact"
    );
    LatticeTiling {
        id: "hankin-square-two-point",
        name: "Hankin two-point square weave (4.4.4.4)",
        description: "Kaplan two-point Hankin motif on the square (4.4.4.4) substrate at Kaplan contact angle 45° with two-point truncation δ = 1/2 edge lengths: decagonal star per tile — four corner triangles, two pentagonal petals, one 10-gon core — 7 faces per lattice cell, wallpaper group cmm.",
        lattice: [Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::Cmm,
        lattice_kind: LatticeKind::CenteredRectangular,
        category: TilingCategory::Islamic,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
