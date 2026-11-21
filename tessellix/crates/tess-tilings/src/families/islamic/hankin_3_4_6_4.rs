//! Hankin's method (Kaplan dissertation §3.4) — Hankin sixfold motif
//! on the rhombitrihexagonal (3.4.6.4) substrate.
//!
//! Constructed at runtime by the ported inference engine in
//! [`crate::hankin`]: rays from every substrate-edge midpoint at
//! Kaplan contact angle 60° to the edge, no truncation (single-point
//! contacts, δ = 0), greedy min-cost pairing (Kaplan §3.4: pairs
//! sorted by total induced segment length), then a planar arrangement
//! of the paired segments against the tile boundary gives the face
//! partition.
//!
//! Unlike the M94/M100 entries, this substrate has no 40-dps Python
//! prototype: the template tiles and the expected-face constants
//! below were machine-emitted from the Rust engine itself, so the
//! test battery is a self-consistency pin — re-running
//! `build_cells` on these tiles must reproduce these faces exactly.
//!
//! Angle choice: probed θK ∈ {30°, 45°, 60°, 67.5°, 90°} (θe =
//! 90° − θK). The hexagon tile pairs completely at every probed
//! angle (12 rays, 6 pairs). The three square tiles pair with
//! nothing for θK ∈ {30°, 45°, 60°, 67.5°} (8 rays, 0 pairs — at
//! 45° the adjacent-edge rays are exactly parallel or meet exactly
//! at an edge midpoint, and the engine rejects zero-length
//! crossings; at 90° they pair head-on into 2 pairs / 2 faces),
//! so their motif stays the original square. The two equilateral
//! triangle tiles: at θK = 30°/45°/67.5° they carry a proper
//! three-pointed star (6 rays, 3 pairs, 4 quadrilateral faces),
//! but the two instances break congruence — the greedy tie-break
//! gives the two orientations different motifs (one a 6-gon, the
//! other 4-gons), which is not D6-compatible. At θK = 60° all
//! three pairs sit exactly on the edge midpoints: the star
//! collapses into the classical 4-subdivision (the triangle splits
//! into four side-1/2 equilateral triangles — 4 triangular faces,
//! orientation-invariant, both instances congruent). At θK = 90°
//! the triangle pairs head-on into 2 pairs / 2 faces (the 3
//! spokes meet at the incenter) and the hexagon's star core
//! collapses (6 faces instead of 7).
//!
//! Measured on the emitted cells (minimum interior angle over all
//! 18 cell faces): θK = 30° → 30.0° (the hexagon splits into six
//! thin sliver triangles — razor-thin); θK = 45° → 45.0°;
//! **θK = 60° → 60.0° (no slivers — the regular hexagram tips and
//! the side-1/2 equilateral triangles)**; θK = 67.5° → 45.0°
//! (sharper hexagram, 45° tips); θK = 90° → degenerate (no hexagon
//! star; 13 faces). θK = 60° is chosen: the most classical
//! sixfold star on this substrate — a regular {6/2} hexagram per
//! hexagon (12-gon core with 60° tips and 240° notches, vertices
//! alternating on circles of radius 1/2 and √3/2, numerically
//! verified) — and the largest minimum interior angle among the
//! star candidates (60° at 60° vs 45° at 67.5°). No 40-dps
//! Python prototype for this substrate: the angle choice was made
//! on the Rust engine's own emitted cells (same self-consistency
//! status as the EXPECTED_* constants, documented in the module
//! header).
//!
//! Validation (engine battery, f64, thresholds 1e-9):
//!   - pairing: hexagon tile complete (12 rays, 6 pairs); each
//!     square tile unpaired (8 rays, 0 pairs, single-face motif);
//!     each triangle tile complete in the 4-subdivision limit
//!     (6 rays, 3 pairs, 4 side-1/2 equilateral faces),
//!   - face partition exact: |Σ faces − tile area| < 1e-9 per
//!     class,
//!   - 18 faces per lattice cell, seeds {0: 6 lozenge rhombi,
//!     1: one 12-gon star core, 2: three squares, 3: eight
//!     side-1/2 equilateral triangles},
//!   - symmetry probe: p6m window check on a ±5-lattice window —
//!     the 553-face multiset (faces whose centroids fall in the
//!     radius-8 disk about the origin, a hexagon center) is
//!     invariant under all 12 D6 elements (rot k·60° + M_0
//!     mirrors, the 90° + 60k mirror families of this tiling),
//!     0 key mismatches at 1e-6 quantization.
//!
//! ## Configuration
//!
//! Substrate: rhombitrihexagonal (3.4.6.4), unit pointy-top
//! hexagons on the lattice a1 = (1.5 + √3/2, √3/2 + 1/2),
//! a2 = (0, 1 + √3), per-cell area 4√3/2 + 3 =
//! |det(a1,a2)|; the cell holds 1 hexagon, 3 unit squares, and 2
//! unit equilateral triangles. At θK = 60° the hexagon yields a
//! 12-gon hexagram core plus six 60°-lozenge rhombi (7 faces);
//! the squares stay single faces; each triangle tile 4-subdivides
//! into side-1/2 equilateral triangles (4 faces each) — 18 faces
//! per lattice cell.
//!
//! Distinct from the registered `rhombitrihexagonal` (the plain
//! Uniform-family substrate tiling with the same 3.4.6.4 vertex
//! figure): same substrate, different construction (greedy
//! inference vs. the Archimedean tiling) and a different
//! partition.
//!
//! Wallpaper group p6m (rot60 + mirror verified on the ±5-lattice
//! window face multiset, all 12 D6 elements, 0 mismatches).

use tess_core::TilingCategory;
use tess_core::geometry::Vec2;
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::hankin;
use crate::lattice::{similarity_map, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// √3/2 as a literal (the archimedean module's H).
const H: f64 = 0.8660254037844386;
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
    Vec2::new(0.5, H),
    Vec2::new(-0.5, H),
    Vec2::new(-1.0, 0.0),
    Vec2::new(-0.5, -H),
    Vec2::new(0.5, -H),
];
/// Substrate tile class 1 ('s30'), CW, in-cell position.
pub(crate) static TILE_1: [Vec2; 4] = [
    Vec2::new(1.0, 0.0),
    Vec2::new(0.5, H),
    Vec2::new(0.5 + H, H + 0.5),
    Vec2::new(1.0 + H, 0.5),
];
/// Substrate tile class 2 ('s150'), CW, in-cell position.
pub(crate) static TILE_2: [Vec2; 4] = [
    Vec2::new(-0.5, H),
    Vec2::new(-1.0, 0.0),
    Vec2::new(-1.0 - H, 0.5),
    Vec2::new(-0.5 - H, H + 0.5),
];
/// Substrate tile class 3 ('s270'), CW, in-cell position.
pub(crate) static TILE_3: [Vec2; 4] = [
    Vec2::new(-0.5, -H),
    Vec2::new(0.5, -H),
    Vec2::new(0.5, -H - 1.0),
    Vec2::new(-0.5, -H - 1.0),
];
/// Substrate tile class 4 ('t0'), CCW, in-cell position (the engine
/// canonicalizes emitted faces to CCW).
pub(crate) static TILE_4: [Vec2; 3] =
    [Vec2::new(1.0, 0.0), Vec2::new(1.0 + H, -0.5), Vec2::new(1.0 + H, 0.5)];
/// Substrate tile class 5 ('t1'), CCW, in-cell position.
pub(crate) static TILE_5: [Vec2; 3] =
    [Vec2::new(0.5, H), Vec2::new(0.5 + H, H + 0.5), Vec2::new(0.5, H + 1.0)];

/// Template tiles in canonical instance order (hex, s30, s150, s270,
/// t0, t1) — the six in-cell positions of one lattice cell.
pub(crate) fn tiles() -> Vec<&'static [Vec2]> {
    vec![&TILE_0, &TILE_1, &TILE_2, &TILE_3, &TILE_4, &TILE_5]
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
pub(crate) static EXPECTED_1_VERTS: &[f64] = &[
    0.5, 0.8660254037844386, 1.0, 0.0, 1.8660254037844386, 0.5, 1.3660254037844386, 1.3660254037844386,
];
#[cfg(test)]
pub(crate) static EXPECTED_1_LENS: &[usize] = &[4];
/// Expected canonical faces for tile class 2 (machine-emitted).
#[cfg(test)]
pub(crate) static EXPECTED_2_VERTS: &[f64] = &[
    -1.8660254037844386, 0.5, -1.0, 0.0, -0.5, 0.8660254037844386, -1.3660254037844386, 1.3660254037844386,
];
#[cfg(test)]
pub(crate) static EXPECTED_2_LENS: &[usize] = &[4];
/// Expected canonical faces for tile class 3 (machine-emitted).
#[cfg(test)]
pub(crate) static EXPECTED_3_VERTS: &[f64] = &[
    -0.5, -1.8660254037844386, 0.5, -1.8660254037844386, 0.5, -0.8660254037844386, -0.5, -0.8660254037844386,
];
#[cfg(test)]
pub(crate) static EXPECTED_3_LENS: &[usize] = &[4];
/// Expected canonical faces for tile class 4 (machine-emitted: the
/// side-1/2 4-subdivision of t0).
#[cfg(test)]
pub(crate) static EXPECTED_4_VERTS: &[f64] = &[
    1.0, 0.0, 1.4330127018922192, -0.25, 1.4330127018922192, 0.25,
    1.4330127018922192, -0.25, 1.8660254037844386, -0.5, 1.8660254037844386, 0.0,
    1.4330127018922192, -0.25, 1.8660254037844386, 0.0, 1.4330127018922192, 0.25,
    1.4330127018922192, 0.25, 1.8660254037844386, 0.0, 1.8660254037844386, 0.5,
];
#[cfg(test)]
pub(crate) static EXPECTED_4_LENS: &[usize] = &[3, 3, 3, 3];
/// Expected canonical faces for tile class 5 (machine-emitted: the
/// side-1/2 4-subdivision of t1).
#[cfg(test)]
pub(crate) static EXPECTED_5_VERTS: &[f64] = &[
    0.5, 0.8660254037844386, 0.9330127018922193, 1.1160254037844386, 0.5, 1.3660254037844386,
    0.5, 1.3660254037844386, 0.9330127018922193, 1.1160254037844386, 0.9330127018922193, 1.6160254037844386,
    0.5, 1.3660254037844386, 0.9330127018922193, 1.6160254037844386, 0.5, 1.8660254037844386,
    0.9330127018922193, 1.1160254037844386, 1.3660254037844386, 1.3660254037844386, 0.9330127018922193, 1.6160254037844386,
];
#[cfg(test)]
pub(crate) static EXPECTED_5_LENS: &[usize] = &[3, 3, 3, 3];
/// Expected per-face seeds across all classes, canonical order.
#[cfg(test)]
pub(crate) static EXPECTED_SEEDS: &[u32] = &[0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3];

/// Hankin sixfold motif on the rhombitrihexagonal (3.4.6.4) substrate.
pub fn hankin_3_4_6_4() -> LatticeTiling {
    let (cells, report) = hankin::build_cells(&tiles(), THETA_E_DEG, DELTA);
    debug_assert!(
        report.per_tile[0].complete,
        "hexagon pairing must be complete"
    );
    debug_assert_eq!(
        report.per_tile[1].faces, 1,
        "unpaired square motif must stay a single face"
    );
    debug_assert_eq!(
        report.per_tile[2].faces, 1,
        "unpaired square motif must stay a single face"
    );
    debug_assert_eq!(
        report.per_tile[3].faces, 1,
        "unpaired square motif must stay a single face"
    );
    debug_assert_eq!(
        report.per_tile[4].faces, 4,
        "triangle 4-subdivision must have 4 faces"
    );
    debug_assert_eq!(
        report.per_tile[5].faces, 4,
        "triangle 4-subdivision must have 4 faces"
    );
    debug_assert!(
        report
            .per_tile
            .iter()
            .all(|t| (t.area_sum - t.tile_area).abs() < 1e-9),
        "face partition inexact"
    );
    LatticeTiling {
        id: "hankin-3-4-6-4",
        name: "Hankin sixfold rhombitrihex motif (3.4.6.4)",
        description: "Hankin greedy-inference motif on the rhombitrihexagonal (3.4.6.4) substrate at Kaplan contact angle 60°, single-point contacts: a regular hexagram per hexagon (12-gon core plus six 60°-lozenge rhombi), unpaired single-face squares, equilateral triangle tiles in the side-1/2 4-subdivision limit — 18 faces per lattice cell, wallpaper group p6m.",
        lattice: [Vec2::new(1.5 + H, H + 0.5), Vec2::new(0.0, 2.0 * H + 1.0)],
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
