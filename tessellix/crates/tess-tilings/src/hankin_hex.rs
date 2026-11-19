//! Hankin's method (Kaplan dissertation §3.4) — Hankin two-point hex weave (6³).
//!
//! Constructed at runtime by the ported inference engine in
//! [`crate::hankin`]: rays from every substrate-edge midpoint at Kaplan
//! contact angle 45° to the edge, two-point truncation δ = 0.5 edge lengths, greedy min-cost
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
//!   - torus (one-cell fold): valence {2:12}, all strands close (4 strands of 6+6+6+6 segments),
//!   - symmetry probes: p6m confirmed exactly on the torus segment
//!     multiset.
//!
//! ## Configuration
//!
//! Substrate: regular hexagonal (6³), hexagon edge 1 (vertices on the
//! unit circle at k·60°), lattice ⟨(3/2,√3/2),(0,√3)⟩. Kaplan's TWO-POINT
//! variant (dissertation Figure 3.5): each contact position is split
//! along the edge by ±δ/2 with δ = 0.5 edge lengths, rays leaving the
//! split points so that same-contact rays cross (Figure 3.5), and the
//! greedy re-pairs the truncated rays. The result is the bracket-and-
//! diamond weave of Kaplan's Figure 3.6 (first row, third column): the
//! engine's rendering matches that figure structurally (cross-correlation
//! F1 plateau over θK ∈ [45°, 47.5°], δ ∈ [0.5, 0.58] at figure scale).
//!
//! Every motif leg crosses legs of neighbouring pairs inside the tile,
//! so the arrangement must split at true curve-curve crossings — the
//! two-point family exercises the full planar-arrangement path of the
//! engine.
//!
//! Wallpaper group p6m (rot60 + both mirror classes verified exactly on
//! the torus segment multiset).

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

/// Substrate tile class 0 ('hex'), CCW, in-cell position.
pub(crate) static TILE_0: [Vec2; 6] = [Vec2::new(1.0, 0.0), Vec2::new(0.5, 0.8660254037844386), Vec2::new(-0.5, 0.8660254037844386), Vec2::new(-1.0, 0.0), Vec2::new(-0.5, -0.8660254037844386), Vec2::new(0.5, -0.8660254037844386)];

/// Template tiles in canonical class order.
pub(crate) fn tiles() -> Vec<&'static [Vec2]> {
    vec![&TILE_0]
}

/// Expected canonical faces for tile class 0 (machine-transcribed).
#[cfg(test)]
pub(crate) static EXPECTED_0_VERTS: &[f64] = &[
    -0.875, -0.21650635094610965, -0.625, -0.649519052838329, -0.5334936490538903, -0.3080127018922193,
    -0.875, 0.21650635094610965, -0.5334936490538903, 0.3080127018922193, -0.625, 0.649519052838329,
    -0.25, -0.8660254037844386, 0.25, -0.8660254037844386, 0.0, -0.6160254037844386,
    -0.25, 0.8660254037844386, 0.0, 0.6160254037844386, 0.25, 0.8660254037844386,
    0.5334936490538903, -0.3080127018922193, 0.625, -0.649519052838329, 0.875, -0.21650635094610965,
    0.5334936490538903, 0.3080127018922193, 0.875, 0.21650635094610965, 0.625, 0.649519052838329,
    -1.0, 0.0, -0.875, -0.21650635094610965, -0.5334936490538903, -0.3080127018922193,
    -0.45096189432334205, 0.0, -0.5334936490538903, 0.3080127018922193, -0.875, 0.21650635094610965,
    -0.625, -0.649519052838329, -0.5, -0.8660254037844386, -0.25, -0.8660254037844386,
    0.0, -0.6160254037844386, -0.22548094716167102, -0.39054445662276766, -0.5334936490538903, -0.3080127018922193,
    -0.625, 0.649519052838329, -0.5334936490538903, 0.3080127018922193, -0.22548094716167102, 0.39054445662276766,
    0.0, 0.6160254037844386, -0.25, 0.8660254037844386, -0.5, 0.8660254037844386,
    0.0, -0.6160254037844386, 0.25, -0.8660254037844386, 0.5, -0.8660254037844386,
    0.625, -0.649519052838329, 0.5334936490538903, -0.3080127018922193, 0.22548094716167102, -0.39054445662276766,
    0.0, 0.6160254037844386, 0.22548094716167102, 0.39054445662276766, 0.5334936490538903, 0.3080127018922193,
    0.625, 0.649519052838329, 0.5, 0.8660254037844386, 0.25, 0.8660254037844386,
    0.45096189432334205, 0.0, 0.5334936490538903, -0.3080127018922193, 0.875, -0.21650635094610965,
    1.0, 0.0, 0.875, 0.21650635094610965, 0.5334936490538903, 0.3080127018922193,
    -0.5334936490538903, -0.3080127018922193, -0.22548094716167102, -0.39054445662276766, 0.0, -0.6160254037844386,
    0.22548094716167102, -0.39054445662276766, 0.5334936490538903, -0.3080127018922193, 0.45096189432334205, 0.0,
    0.5334936490538903, 0.3080127018922193, 0.22548094716167102, 0.39054445662276766, 0.0, 0.6160254037844386,
    -0.22548094716167102, 0.39054445662276766, -0.5334936490538903, 0.3080127018922193, -0.45096189432334205, 0.0,
];
#[cfg(test)]
pub(crate) static EXPECTED_0_LENS: &[usize] = &[3, 3, 3, 3, 3, 3, 6, 6, 6, 6, 6, 6, 12];
/// Expected per-face seeds across all classes, canonical order.
#[cfg(test)]
pub(crate) static EXPECTED_SEEDS: &[u32] = &[0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2];

/// Hankin two-point hex weave (6³).
pub fn hankin_hex_two_point() -> LatticeTiling {
    let (cells, report) = hankin::build_cells(&tiles(), THETA_E_DEG, DELTA);
    debug_assert!(report.complete, "pairing incomplete");
    debug_assert!(
        report.per_tile.iter().all(|t| (t.area_sum - t.tile_area).abs() < 1e-9),
        "face partition inexact"
    );
    LatticeTiling {
        id: "hankin-hex-two-point",
        name: "Hankin two-point hex weave (6³)",
        description: "Kaplan two-point Hankin motif on the regular hexagonal (6³) substrate at Kaplan contact angle 45° with two-point truncation δ = 1/2 edge lengths: bracket-and-diamond interlace with true curve-curve crossings inside each tile, matching dissertation Figure 3.6 (row 1, two-point column), wallpaper group p6m.",
        lattice: [Vec2::new(1.5, 0.8660254037844386), Vec2::new(0.0, 1.7320508075688772)],
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
