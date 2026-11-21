//! Hankin's method (Kaplan dissertation §3.4) — Hankin motif on the
//! elongated-triangular (3.3.3.4.4) substrate.
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
//! Orientation: the archimedean module stores the down-triangle
//! instance clockwise (its in-cell position runs around the vertex
//! (0.5, H) clockwise). Under the engine's contact convention a CW
//! tile emits rays that point out of the tile interior and pairs
//! with nothing at every probed angle (1 face, 0 pairs), so the
//! all-CCW in-cell table is used: t_up as stored, t_down re-oriented
//! CCW, and the square CCW.
//!
//! Angle choice: probed θK ∈ {30°, 45°, 60°, 67.5°, 90°} (θe =
//! 90° − θK) on the all-CCW in-cell table (faithful f64 port of the
//! engine; `*` = complete pairing):
//!
//!```text
//!   tile      θK=30   θK=45   θK=60   θK=67.5  θK=90
//!   tri CCW   4f/3p*  4f/3p*  4f/3p*  4f/3p*   2f/2p (incomplete)
//!   sq  CCW   5f/4p*  5f/4p*  5f/4p*  5f/4p*   4f/4p*
//!   sq  CW    1f/0p   1f/0p   1f/0p   1f/0p    1f/0p
//!   tri CW    1f/0p   1f/0p   1f/0p   1f/0p    1f/0p
//!```
//!
//! Rejected candidates: θK = 30° (minimum interior angle 30° —
//! razor-thin slivers in both tile classes), θK = 67.5° (45° minimum,
//! sharper than 60°), θK = 45° (45° minimum, acceptable but sharper
//! than 60°), θK = 90° (the triangle pairings degenerate — the 3
//! spokes meet at the incenter into 2 pairs / 2 faces, incomplete;
//! the square star collapses to 4 faces), and both CW rows (no
//! pairing at any angle — the motif stays the raw tile). θK = 60°
//! (θe = 30°) is chosen: the largest minimum interior angle among
//! the complete candidates (60°) — the triangle tiles sit in the
//! classical 4-subdivision limit (each triangle splits into four
//! side-1/2 equilateral triangles — 4 triangular faces, 3 pairs) and
//! the square yields a 5-face motif (4 corner quadrilaterals + a
//! central octagon, 4 pairs) — 13 faces per lattice cell, every tile
//! paired completely.
//!
//! Cell census: 13 faces per lattice cell = 8 side-1/2 equilateral
//! triangles (area √3/16 each), 4 square corner quadrilaterals
//! (area (3 − √3)/8 each), 1 central octagon (area (√3 − 1)/2);
//! Σ = 1 + √3/2 = |det(a1, a2)|. All face vertices lie in
//! Z[1/24] + Z[1/48]·(√3/2) (i.e. a/24 + b√3/96, (a, b) ∈ ℤ²
//! unique), verified exactly on a ±9-lattice window of 4693 faces.
//!
//! Measured symmetry (exact Q(√3) face-maps — faces re-canonicalized
//! to CCW / minimum-vertex before keying — on the 551-face disk
//! about the square center C = (1, H + 1/2); two face centroids sit
//! at exactly distance 5, so the probe radius is the strict midpoint
//! of the nearest in/out squared distances, R² =
//! 24882743452934203698152347053/10²⁶): cmm. The 20 cmm elements —
//! translations ⟨a1 = (1, 0), a2 = (1/2, 1 + H)⟩; two-fold orbits
//! {C + L} ∪ {C + a1 + L} ∪ {(1/2, H + 1/2) + L} (on the mirror
//! lines) and {(±1/4, H/2) + L} ∪ {(1/4, H/2 + 1 + H)} (on the
//! glides); mirrors x = k/2 and y = −1/2 + k(1 + H); glides
//! y = H/2 + k(1 + H) of shift a1/2 — all verified 0-mismatch on the
//! face multiset. Negative controls fail: rot90 about C (exact),
//! rot30/rot60 about the triangle centroids, the off-orbit mirror
//! y = H/2, and the off-orbit glides.
//!
//! ## Configuration
//!
//! Substrate: elongated-triangular (3.3.3.4.4) — alternating rows of
//! unit equilateral triangles and unit squares, wallpaper cmm on the
//! lattice a1 = (1, 0), a2 = (1/2, 1 + √3/2), per-cell area
//! 1 + √3/2 = |det(a1, a2)| (two triangles + one square per cell).
//! At θK = 60° the two triangle tiles each 4-subdivide into four
//! side-1/2 equilateral triangles (4 faces each) and the square
//! yields the 5-face motif above — 13 faces per lattice cell.
//!
//! Wallpaper group cmm (battery as above; the substrate's own cmm
//! group survives the hankin partition).

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

/// Substrate tile class 0 ('t_up'), CCW, in-cell position.
pub(crate) static TILE_0: [Vec2; 3] =
    [Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.5, H)];
/// Substrate tile class 1 ('t_down'), CCW (the archimedean module
/// stores this instance CW; the all-CCW table is required — see the
/// module header), in-cell position.
pub(crate) static TILE_1: [Vec2; 3] =
    [Vec2::new(1.5, H), Vec2::new(0.5, H), Vec2::new(1.0, 0.0)];
/// Substrate tile class 2 ('square'), CCW, in-cell position.
pub(crate) static TILE_2: [Vec2; 4] = [
    Vec2::new(0.5, H),
    Vec2::new(1.5, H),
    Vec2::new(1.5, 1.0 + H),
    Vec2::new(0.5, 1.0 + H),
];

/// Template tiles in canonical instance order (t_up, t_down, square)
/// — the three in-cell positions of one lattice cell.
pub(crate) fn tiles() -> Vec<&'static [Vec2]> {
    vec![&TILE_0, &TILE_1, &TILE_2]
}

/// Expected canonical faces for tile class 0 (machine-emitted from the
/// Rust engine at θe = 30°: the side-1/2 4-subdivision of t_up).
#[cfg(test)]
pub(crate) static EXPECTED_0_VERTS: &[f64] = &[
    0.0, 0.0, 0.5, 0.0, 0.25, 0.4330127018922193,
    0.25, 0.4330127018922193, 0.5, 0.0, 0.75, 0.4330127018922193,
    0.25, 0.4330127018922193, 0.75, 0.4330127018922193, 0.5, 0.8660254037844386,
    0.5, 0.0, 1.0, 0.0, 0.75, 0.4330127018922193,
];
#[cfg(test)]
pub(crate) static EXPECTED_0_LENS: &[usize] = &[3, 3, 3, 3];
/// Expected canonical faces for tile class 1 (machine-emitted: the
/// side-1/2 4-subdivision of t_down).
#[cfg(test)]
pub(crate) static EXPECTED_1_VERTS: &[f64] = &[
    0.5, 0.8660254037844386, 0.75, 0.4330127018922193, 1.0, 0.8660254037844386,
    0.75, 0.4330127018922193, 1.0, 0.0, 1.25, 0.4330127018922193,
    0.75, 0.4330127018922193, 1.25, 0.4330127018922193, 1.0, 0.8660254037844386,
    1.0, 0.8660254037844386, 1.25, 0.4330127018922193, 1.5, 0.8660254037844386,
];
#[cfg(test)]
pub(crate) static EXPECTED_1_LENS: &[usize] = &[3, 3, 3, 3];
/// Expected canonical faces for tile class 2 (machine-emitted: the
/// square motif — four corner quadrilaterals, then the central
/// octagon).
#[cfg(test)]
pub(crate) static EXPECTED_2_VERTS: &[f64] = &[
    0.5, 0.8660254037844386, 1.0, 0.8660254037844386, 0.8169872981077807, 1.1830127018922192, 0.5, 1.3660254037844386,
    0.5, 1.3660254037844386, 0.8169872981077807, 1.549038105676658, 1.0, 1.8660254037844386, 0.5, 1.8660254037844386,
    1.0, 0.8660254037844386, 1.5, 0.8660254037844386, 1.5, 1.3660254037844386, 1.1830127018922192, 1.1830127018922192,
    1.0, 1.8660254037844386, 1.1830127018922194, 1.5490381056766578, 1.5, 1.3660254037844386, 1.5, 1.8660254037844386,
    0.5, 1.3660254037844386, 0.8169872981077807, 1.1830127018922192, 1.0, 0.8660254037844386, 1.1830127018922192, 1.1830127018922192, 1.5, 1.3660254037844386, 1.1830127018922194, 1.5490381056766578, 1.0, 1.8660254037844386, 0.8169872981077807, 1.549038105676658,
];
#[cfg(test)]
pub(crate) static EXPECTED_2_LENS: &[usize] = &[4, 4, 4, 4, 8];
/// Expected per-face seeds across all classes, canonical order
/// (both triangle subdivisions share the (3-gon, √3/16) seed
/// signature).
#[cfg(test)]
pub(crate) static EXPECTED_SEEDS: &[u32] = &[0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2];

/// Hankin motif on the elongated-triangular (3.3.3.4.4) substrate.
pub fn hankin_3_3_3_4_4() -> LatticeTiling {
    let (cells, report) = hankin::build_cells(&tiles(), THETA_E_DEG, DELTA);
    debug_assert!(
        report.complete,
        "all three tile instances must pair completely"
    );
    debug_assert_eq!(
        report.per_tile[0].faces, 4,
        "up-triangle 4-subdivision must have 4 faces"
    );
    debug_assert_eq!(
        report.per_tile[1].faces, 4,
        "down-triangle 4-subdivision must have 4 faces"
    );
    debug_assert_eq!(
        report.per_tile[2].faces, 5,
        "square motif must have 5 faces"
    );
    debug_assert!(
        report
            .per_tile
            .iter()
            .all(|t| (t.area_sum - t.tile_area).abs() < 1e-9),
        "face partition inexact"
    );
    LatticeTiling {
        id: "hankin-3-3-3-4-4",
        name: "Hankin elongated-triangular motif (3.3.3.4.4)",
        description: "Hankin greedy-inference motif on the elongated-triangular (3.3.3.4.4) substrate at Kaplan contact angle 60°, single-point contacts: both triangle tiles in the side-1/2 4-subdivision limit (four equilateral triangles each) and a square motif of four corner quadrilaterals around a central octagon — 13 faces per lattice cell, wallpaper group cmm.",
        lattice: [Vec2::new(1.0, 0.0), Vec2::new(0.5, 1.0 + H)],
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

