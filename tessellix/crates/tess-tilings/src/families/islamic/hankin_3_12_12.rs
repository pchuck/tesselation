//! Hankin's method (Kaplan dissertation §3.4) — Hankin twelvefold khatam (3.12.12).
//!
//! Constructed at runtime by the ported inference engine in
//! [`crate::hankin`]: rays from every substrate-edge midpoint at Kaplan
//! contact angle 45° to the edge, no truncation (single-point contacts,
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
//! Angle choice: all three candidate contact angles pair completely on
//! this substrate. θK = 30° is the flat limiting case — the star core
//! (a 24-gon) closes at 120° tips and the dodecagon shows no points;
//! θK = 60° is spiky — 60° tips with chaotic triangle petals.
//! θK = 45° gives the classical shining twelve-pointed star: 90° tips
//! and 150° notches on the 24-gon core — the mid-"shine" range of the
//! star heuristic (PROPOSED.md §22a: s = 2 for n > 6 with θ > 2π/n,
//! i.e. θK > 30° here) — and is symmetric θK = θE like the two-point
//! entry. Measured on the emitted cells: seed-1 core minimum interior
//! angle 90.0° at θK = 45° (120.0° at θK = 30°, 60.0° at θK = 60°).
//!
//! Validation (engine battery, f64, thresholds 1e-9):
//!   - pairing complete on every tile class: (24 rays, 12 pairs),
//!     (6, 3), (6, 3),
//!   - face partition exact: |Σ faces − tile area| < 1e-9 per class,
//!   - 21 faces per lattice cell, seeds {0: 12 dodecagon petals,
//!     1: one 24-gon star core, 2: 6 triangle petals, 3: two
//!     triangle tips},
//!   - symmetry probe: p6m window check on a ±4-lattice window — the
//!     1701-face multiset (403 interior faces of the radius-8 disk)
//!     is invariant under all 12 D6 elements (rot k·60° + M_0
//!     mirrors), 0 key mismatches at 1e-6 quantization.
//!
//! ## Configuration
//!
//! Substrate: truncated hexagonal (3.12.12), dodecagon edge 1, lattice
//! a1 = (3/2+√3, 1+√3/2), a2 = (0, 2+√3), per-cell area
//! 3(2+√3) + √3/2 = 6 + 7√3/2 = |det(a1,a2)|. At θK = 45° the
//! dodecagon yields a twelve-pointed star (a 24-gon core plus 12
//! petals, 13 faces) and each triangle a 4-pointed star with a
//! hexagon tip (4 faces each) — 21 faces per lattice cell.
//!
//! Distinct from the registered `pic-3-12-12` (δ = 60° reflection rule
//! on the 3.12.12 substrate): same substrate, different construction
//! (greedy inference vs reflection chords) and different partition.
//!
//! Wallpaper group p6m (rot60 + mirror verified on the ±4-lattice
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
pub(crate) const THETA_K_DEG: f64 = 45.0;
/// Engine ray angle from the edge normal's frame: 90° − θK.
pub(crate) const THETA_E_DEG: f64 = 45.0;
/// Two-point truncation as a fraction of the substrate edge length.
pub(crate) const DELTA: f64 = 0.0;

/// Substrate tile class 0 ('dod'), CCW, in-cell position.
pub(crate) static TILE_0: [Vec2; 12] = [Vec2::new(1.8660254037844386, 0.5), Vec2::new(1.3660254037844386, 1.3660254037844386), Vec2::new(0.5, 1.8660254037844386), Vec2::new(-0.5, 1.8660254037844386), Vec2::new(-1.3660254037844386, 1.3660254037844386), Vec2::new(-1.8660254037844386, 0.5), Vec2::new(-1.8660254037844386, -0.5), Vec2::new(-1.3660254037844386, -1.3660254037844386), Vec2::new(-0.5, -1.8660254037844386), Vec2::new(0.5, -1.8660254037844386), Vec2::new(1.3660254037844386, -1.3660254037844386), Vec2::new(1.8660254037844386, -0.5)];
/// Substrate tile class 1 ('tri'), CCW, in-cell position.
pub(crate) static TILE_1: [Vec2; 3] = [Vec2::new(1.8660254037844386, -0.5), Vec2::new(2.732050807568877, 0.0), Vec2::new(1.8660254037844386, 0.5)];
/// Substrate tile class 2 ('tri'), CCW, in-cell position.
pub(crate) static TILE_2: [Vec2; 3] = [Vec2::new(0.5, 1.8660254037844386), Vec2::new(1.3660254037844386, 1.3660254037844386), Vec2::new(1.3660254037844386, 2.366025403784439)];

/// Template tiles in canonical class order.
pub(crate) fn tiles() -> Vec<&'static [Vec2]> {
    vec![&TILE_0, &TILE_1, &TILE_2]
}

/// Expected canonical faces for tile class 0 (machine-emitted from the
/// Rust engine at θe = 45°).
#[cfg(test)]
pub(crate) static EXPECTED_0_VERTS: &[f64] = &[
    -1.8660254037844386, -0.5, -1.6160254037844386, -0.9330127018922193, -1.471687836487032, -0.39433756729740643,
    -1.8660254037844386, 0.0, -1.8660254037844386, 0.0, -1.471687836487032, 0.39433756729740643,
    -1.6160254037844386, 0.9330127018922193, -1.8660254037844386, 0.5, -1.6160254037844386, -0.9330127018922193,
    -1.3660254037844386, -1.3660254037844386, -0.9330127018922193, -1.6160254037844386, -1.0773502691896257, -1.0773502691896257,
    -1.6160254037844386, 0.9330127018922193, -1.0773502691896257, 1.0773502691896257, -0.9330127018922193, 1.6160254037844386,
    -1.3660254037844386, 1.3660254037844386, -0.9330127018922193, -1.6160254037844386, -0.5, -1.8660254037844386,
    0.0, -1.8660254037844386, -0.39433756729740643, -1.471687836487032, -0.9330127018922193, 1.6160254037844386,
    -0.39433756729740643, 1.471687836487032, 0.0, 1.8660254037844386, -0.5, 1.8660254037844386,
    0.0, -1.8660254037844386, 0.5, -1.8660254037844386, 0.9330127018922193, -1.6160254037844386,
    0.39433756729740643, -1.471687836487032, 0.0, 1.8660254037844386, 0.39433756729740643, 1.471687836487032,
    0.9330127018922193, 1.6160254037844386, 0.5, 1.8660254037844386, 0.9330127018922193, -1.6160254037844386,
    1.3660254037844386, -1.3660254037844386, 1.6160254037844386, -0.9330127018922193, 1.0773502691896257, -1.0773502691896257,
    0.9330127018922193, 1.6160254037844386, 1.0773502691896257, 1.0773502691896257, 1.6160254037844386, 0.9330127018922193,
    1.3660254037844386, 1.3660254037844386, 1.471687836487032, -0.39433756729740643, 1.6160254037844386, -0.9330127018922193,
    1.8660254037844386, -0.5, 1.8660254037844386, 0.0, 1.471687836487032, 0.39433756729740643,
    1.8660254037844386, 0.0, 1.8660254037844386, 0.5, 1.6160254037844386, 0.9330127018922193,
    -1.8660254037844386, 0.0, -1.471687836487032, -0.39433756729740643, -1.6160254037844386, -0.9330127018922193,
    -1.0773502691896257, -1.0773502691896257, -0.9330127018922193, -1.6160254037844386, -0.39433756729740643, -1.471687836487032,
    0.0, -1.8660254037844386, 0.39433756729740643, -1.471687836487032, 0.9330127018922193, -1.6160254037844386,
    1.0773502691896257, -1.0773502691896257, 1.6160254037844386, -0.9330127018922193, 1.471687836487032, -0.39433756729740643,
    1.8660254037844386, 0.0, 1.471687836487032, 0.39433756729740643, 1.6160254037844386, 0.9330127018922193,
    1.0773502691896257, 1.0773502691896257, 0.9330127018922193, 1.6160254037844386, 0.39433756729740643, 1.471687836487032,
    0.0, 1.8660254037844386, -0.39433756729740643, 1.471687836487032, -0.9330127018922193, 1.6160254037844386,
    -1.0773502691896257, 1.0773502691896257, -1.6160254037844386, 0.9330127018922193, -1.471687836487032, 0.39433756729740643,
];
#[cfg(test)]
pub(crate) static EXPECTED_0_LENS: &[usize] = &[4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 24];
/// Expected canonical faces for tile class 1 (machine-emitted).
#[cfg(test)]
pub(crate) static EXPECTED_1_VERTS: &[f64] = &[
    1.8660254037844386, -0.5, 2.299038105676658, -0.25, 2.049038105676658, -0.1830127018922193,
    1.8660254037844386, 0.0, 1.8660254037844386, 0.0, 2.049038105676658, 0.1830127018922193,
    2.299038105676658, 0.25, 1.8660254037844386, 0.5, 2.299038105676658, -0.25,
    2.732050807568877, 0.0, 2.299038105676658, 0.25, 2.366025403784439, 0.0,
    1.8660254037844386, 0.0, 2.049038105676658, -0.1830127018922193, 2.299038105676658, -0.25,
    2.366025403784439, 0.0, 2.299038105676658, 0.25, 2.049038105676658, 0.1830127018922193,
];
#[cfg(test)]
pub(crate) static EXPECTED_1_LENS: &[usize] = &[4, 4, 4, 6];
/// Expected canonical faces for tile class 2 (machine-emitted).
#[cfg(test)]
pub(crate) static EXPECTED_2_VERTS: &[f64] = &[
    0.5, 1.8660254037844386, 0.9330127018922193, 1.6160254037844386, 0.8660254037844386, 1.8660254037844386,
    0.9330127018922193, 2.116025403784439, 0.9330127018922193, 1.6160254037844386, 1.3660254037844386, 1.3660254037844386,
    1.3660254037844386, 1.8660254037844388, 1.183012701892219, 1.6830127018922192, 0.9330127018922193, 2.116025403784439,
    1.1830127018922192, 2.049038105676658, 1.3660254037844386, 1.8660254037844388, 1.3660254037844386, 2.366025403784439,
    0.8660254037844386, 1.8660254037844386, 0.9330127018922193, 1.6160254037844386, 1.183012701892219, 1.6830127018922192,
    1.3660254037844386, 1.8660254037844388, 1.1830127018922192, 2.049038105676658, 0.9330127018922193, 2.116025403784439,
];
#[cfg(test)]
pub(crate) static EXPECTED_2_LENS: &[usize] = &[4, 4, 4, 6];
/// Expected per-face seeds across all classes, canonical order.
#[cfg(test)]
pub(crate) static EXPECTED_SEEDS: &[u32] = &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 3, 2, 2, 2, 3];

/// Hankin twelvefold khatam (3.12.12).
pub fn hankin_3_12_12() -> LatticeTiling {
    let (cells, report) = hankin::build_cells(&tiles(), THETA_E_DEG, DELTA);
    debug_assert!(report.complete, "pairing incomplete");
    debug_assert!(
        report.per_tile.iter().all(|t| (t.area_sum - t.tile_area).abs() < 1e-9),
        "face partition inexact"
    );
    LatticeTiling {
        id: "hankin-3-12-12",
        name: "Hankin twelvefold khatam (3.12.12)",
        description: "Hankin greedy-inference motif on the truncated hexagonal (3.12.12) substrate at Kaplan contact angle 45°, single-point contacts: dodecagon twelve-pointed star (24-gon core + 12 petals), triangle 4-pointed stars with hexagon tips, 21 faces per lattice cell, wallpaper group p6m.",
        lattice: [Vec2::new(3.232050807568877, 1.8660254037844386), Vec2::new(0.0, 3.732050807568877)],
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
