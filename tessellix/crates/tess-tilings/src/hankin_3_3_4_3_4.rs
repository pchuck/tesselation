//! Hankin's method (Kaplan dissertation §3.4) — Hankin motif on the
//! snub-square (3.3.4.3.4) substrate.
//!
//! Constructed at runtime by the ported inference engine in
//! [`crate::hankin`]: rays from every substrate-edge midpoint at
//! Kaplan contact angle 60° to the edge, no truncation (single-point
//! contacts, δ = 0), greedy min-cost pairing (Kaplan §3.4: pairs
//! sorted by total induced segment length), then a planar arrangement
//! of the paired segments against the tile boundary gives the face
//! partition.
//!
//! The archimedean module stores all twelve snub-square instances
//! CCW in their in-cell positions (s0–s3 squares, t0–t7 triangles),
//! so the as-stored table is used directly — the CW pitfall probed in
//! the M107/M111 survey (reversed instances emit rays that point out
//! of the tile interior and pair with nothing at every angle,
//! 1 face / 0 pairs) does not apply.
//!
//! The expected-face constants below were machine-emitted by the M111
//! spike (a faithful f64 port of the Rust engine) and cross-audited
//! exactly in the Q(√2, √3) coefficient ring, so the test battery is
//! a self-consistency pin — re-running `build_cells` on these tiles
//! must reproduce these faces within 1e-9.
//!
//! Angle choice: probed θK ∈ {30°, 45°, 60°, 67.5°, 90°} (θe =
//! 90° − θK) on the all-CCW in-cell table (faithful f64 port of the
//! engine; `*` = complete pairing):
//!
//!```text
//!   tile      θK=30   θK=45   θK=60   θK=67.5  θK=90
//!   sq ×4     5f/4p*  5f/4p*  5f/4p*  5f/4p*   4f/4p*
//!   tri ×4    4f/3p*  4f/3p*  4f/3p*  4f/3p*   2f/2p (incomplete)
//!   CW (12)   1f/0p   1f/0p   1f/0p   1f/0p    1f/0p
//!```
//!
//! Rejected candidates: θK = 30° (minimum interior angle 30° —
//! razor-thin slivers in both tile classes), θK = 67.5° and 45°
//! (45° minimum, sharper than 60°), θK = 90° (the triangle pairings
//! degenerate — the 3 spokes meet at the incenter into 2 pairs /
//! 2 faces, incomplete; the square star collapses to 4 faces), and
//! the CW rows (no pairing at any angle — the motif stays the raw
//! tile). θK = 60° (θe = 30°) is chosen: the largest minimum interior
//! angle among the complete candidates (60°) — each square yields the
//! 5-face motif (4 corner quadrilaterals + a central octagon, 4
//! pairs) and each triangle sits in the classical 4-subdivision limit
//! (four side-1/2 equilateral triangles, 3 pairs) — 52 faces per
//! lattice cell, every tile paired completely.
//!
//! Cell census: 52 faces per lattice cell = 16 square corner
//! quadrilaterals (area (3 − √3)/8 each), 4 central octagons (area
//! (√3 − 1)/2 each), 32 side-1/2 equilateral triangles (area √3/16
//! each); Σ = 4 + 2√3 = |det(a1, a2)| with L = 1 + √3. All face
//! vertices lie in (ℤ + ℤ√3)/4 (i.e. (a + b√3)/4, (a, b) ∈ ℤ² —
//! maximum coefficient denominator 4): verified exactly in the full
//! motif field Q(√2, √3), where the √2 and √6 coefficients are
//! identically zero — √2 enters only as the direction of the 45°
//! motif/glide rays, no face coordinate carries it.
//!
//! Measured symmetry (exact Q(√2, √3) face-maps — faces
//! re-canonicalized to CCW / minimum-vertex before keying — on the
//! 121-cell window of 6292 faces, all distinct; each candidate is
//! censused 0-mismatch on its interior square S(c, 2L)): p4g.
//! Translations (L, 0) / (0, L) with L = 1 + √3 and the centered
//! (L/2, ±L/2) (788 faces each, 0 mismatch; (L/2, 0) / (0, L/2)
//! fail 788/788) — the face-set translation lattice is the CENTERED
//! lattice Λ′ = ⟨(L/2, L/2), (L/2, −L/2)⟩, not L² (conventional
//! p4g cell side L/√2). Four-fold centers (769 faces each, two Λ′
//! orbits of two points mod L²): {(L/4, L/4 + 1/2), (L/4, 3L/4 + 1/2),
//! (3L/4, L/4 + 1/2), (3L/4, 3L/4 + 1/2)} ≈
//! {(0.683, 1.183), (0.683, 2.549), (2.049, 1.183), (2.049, 2.549)}.
//! Two-fold centers (800 faces each, two Λ′ orbits of
//! two points mod L²):
//! the four 4-fold points above ∪ {(0, 1/2), (0, (L + 1)/2),
//! (L/2, 1/2), (L/2, (L + 1)/2)} ≈
//! {(0, 0.5), (0, 1.866), (1.366, 0.5), (1.366, 1.866)}. Glides,
//! measured off the L/8 grid the window sweep cannot reach: 45°
//! lines y − x ≡ L/4 + 1/2, 3L/4 + 1/2 (mod L) of shift
//! ±(L/4, L/4), and 135° lines y + x ≡ L/4 + 1/2, 3L/4 + 1/2
//! (mod L) of shift ±(L/4, −L/4) (785 faces each, 0 mismatch). The
//! full-disk census about (5L/4, 5L/4) (2615 faces, adaptive exact
//! R² = 377/6 + 98√3/3 ≈ 119.413660 — the strict midpoint of the
//! nearest in/out squared centroid distances) confirms the orbit
//! structure. Negative controls fail: no mirror line in either
//! family (all 18 t = 0 controls, 772–800 faces tested), the
//! half-period shifts ±(L/2, ±L/2) on the off-grid lines (their
//! squares are the passing centered-lattice translations), rot45
//! about a 4-fold center, and the off-orbit L/8 mirrors and glides.
//! p4g (glides, no mirrors) — consistent with the substrate group.
//!
//! ## Configuration
//!
//! Substrate: snub-square (3.3.4.3.4) — the snub of the square
//! tiling: 4 unit squares (two orientation classes) + 8 unit
//! triangles per cell, wallpaper p4g on the lattice
//! a1 = (1 + √3, 0), a2 = (0, 1 + √3), per-cell area
//! 4 + 2√3 = (1 + √3)² = |det(a1, a2)| (the archimedean module's
//! `snub_square`). At θK = 60° the four square tiles each yield the
//! 5-face motif above (20 faces) and the eight triangle tiles each
//! 4-subdivide into four side-1/2 equilateral triangles (32 faces) —
//! 52 faces per lattice cell.
//!
//! Wallpaper group p4g (battery as above). Motif field Q(√2, √3):
//! the tiles and the lattice live in Q(√3) (L = 1 + √3), √2 enters
//! via the 45° motif/glide rays. Glides run off the L/8 grid: 45°
//! lines y − x ≡ L/4 + 1/2, 3L/4 + 1/2 (mod L) with shift
//! (L/4, L/4); 135° lines y + x ≡ L/4 + 1/2, 3L/4 + 1/2 (mod L)
//! with shift (L/4, −L/4). No mirror lines.
//!
//! PATTERNS.MD §66 (provisional).

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

/// Substrate tile class 0 ('s0'), CCW, in-cell position.
pub(crate) static TILE_0: [Vec2; 4] = [
    Vec2::new(0.0, 1.0 + 2.0 * H),
    Vec2::new(0.5, 1.0 + H),
    Vec2::new(0.5 + H, 1.5 + H),
    Vec2::new(H, 1.5 + 2.0 * H),
];
/// Substrate tile class 1 ('s1'), CCW, in-cell position.
pub(crate) static TILE_1: [Vec2; 4] = [
    Vec2::new(H, 0.5),
    Vec2::new(0.5 + H, 0.5 + H),
    Vec2::new(0.5, 1.0 + H),
    Vec2::new(0.0, 1.0),
];
/// Substrate tile class 2 ('s2'), CCW, in-cell position.
pub(crate) static TILE_2: [Vec2; 4] = [
    Vec2::new(1.0 + 2.0 * H, 1.0),
    Vec2::new(0.5 + 2.0 * H, 1.0 + H),
    Vec2::new(0.5 + H, 0.5 + H),
    Vec2::new(1.0 + H, 0.5),
];
/// Substrate tile class 3 ('s3'), CCW, in-cell position.
pub(crate) static TILE_3: [Vec2; 4] = [
    Vec2::new(1.0 + H, 1.5 + 2.0 * H),
    Vec2::new(0.5 + H, 1.5 + H),
    Vec2::new(0.5 + 2.0 * H, 1.0 + H),
    Vec2::new(1.0 + 2.0 * H, 1.0 + 2.0 * H),
];
/// Substrate tile class 4 ('t0'), CCW, in-cell position.
pub(crate) static TILE_4: [Vec2; 3] = [
    Vec2::new(0.0, 0.0),
    Vec2::new(H, 0.5),
    Vec2::new(0.0, 1.0),
];
/// Substrate tile class 5 ('t1'), CCW, in-cell position.
pub(crate) static TILE_5: [Vec2; 3] = [
    Vec2::new(1.0 + 2.0 * H, 0.0),
    Vec2::new(1.0 + 2.0 * H, 1.0),
    Vec2::new(1.0 + H, 0.5),
];
/// Substrate tile class 6 ('t2'), CCW, in-cell position.
pub(crate) static TILE_6: [Vec2; 3] = [
    Vec2::new(0.0, 1.0 + 2.0 * H),
    Vec2::new(-0.5, 1.0 + H),
    Vec2::new(0.5, 1.0 + H),
];
/// Substrate tile class 7 ('t3'), CCW, in-cell position.
pub(crate) static TILE_7: [Vec2; 3] = [
    Vec2::new(H, 0.5),
    Vec2::new(0.5 + H, 0.5 - H),
    Vec2::new(1.0 + H, 0.5),
];
/// Substrate tile class 8 ('t4'), CCW, in-cell position.
pub(crate) static TILE_8: [Vec2; 3] = [
    Vec2::new(H, 0.5),
    Vec2::new(1.0 + H, 0.5),
    Vec2::new(0.5 + H, 0.5 + H),
];
/// Substrate tile class 9 ('t5'), CCW, in-cell position.
pub(crate) static TILE_9: [Vec2; 3] = [
    Vec2::new(0.0, 1.0),
    Vec2::new(0.5, 1.0 + H),
    Vec2::new(-0.5, 1.0 + H),
];
/// Substrate tile class 10 ('t6'), CCW, in-cell position.
pub(crate) static TILE_10: [Vec2; 3] = [
    Vec2::new(0.5 + 2.0 * H, 1.0 + H),
    Vec2::new(0.5 + H, 1.5 + H),
    Vec2::new(0.5 + H, 0.5 + H),
];
/// Substrate tile class 11 ('t7'), CCW, in-cell position.
pub(crate) static TILE_11: [Vec2; 3] = [
    Vec2::new(0.5, 1.0 + H),
    Vec2::new(0.5 + H, 0.5 + H),
    Vec2::new(0.5 + H, 1.5 + H),
];

/// Template tiles in canonical instance order (s0–s3, t0–t7) — the
/// twelve in-cell positions of one lattice cell.
pub(crate) fn tiles() -> Vec<&'static [Vec2]> {
    vec![
        &TILE_0,
        &TILE_1,
        &TILE_2,
        &TILE_3,
        &TILE_4,
        &TILE_5,
        &TILE_6,
        &TILE_7,
        &TILE_8,
        &TILE_9,
        &TILE_10,
        &TILE_11,
    ]
}

/// Expected canonical face vertices across all twelve tiles (machine-
/// emitted; 52 faces in canonical order — four square motifs of
/// [4, 4, 4, 4, 8] then eight triangle 4-subdivisions of [3, 3, 3, 3]).
#[cfg(test)]
pub(crate) static EXPECTED_N_VERTS: &[f64] = &[
    0.0, 2.732050807568877, 0.25, 2.299038105676658, 0.43301270189221935, 2.616025403784439,
    0.4330127018922193, 2.982050807568877, 0.25, 2.299038105676658, 0.5, 1.8660254037844386,
    0.9330127018922193, 2.1160254037844384, 0.6160254037844382, 2.299038105676658, 0.4330127018922193, 2.982050807568877,
    0.7499999999999998, 2.799038105676658, 1.1160254037844386, 2.799038105676658, 0.8660254037844386, 3.232050807568877,
    0.9330127018922193, 2.1160254037844384, 1.3660254037844386, 2.3660254037844384, 1.1160254037844386, 2.799038105676658,
    0.9330127018922193, 2.4820508075688776, 0.25, 2.299038105676658, 0.6160254037844382, 2.299038105676658,
    0.9330127018922193, 2.1160254037844384, 0.9330127018922193, 2.4820508075688776, 1.1160254037844386, 2.799038105676658,
    0.7499999999999998, 2.799038105676658, 0.4330127018922193, 2.982050807568877, 0.43301270189221935, 2.616025403784439,
    0.0, 1.0, 0.4330127018922193, 0.75, 0.43301270189221935, 1.1160254037844384,
    0.25, 1.4330127018922192, 0.25, 1.4330127018922192, 0.6160254037844384, 1.4330127018922192,
    0.9330127018922193, 1.6160254037844386, 0.5, 1.8660254037844386, 0.4330127018922193, 0.75,
    0.8660254037844386, 0.5, 1.1160254037844386, 0.9330127018922193, 0.75, 0.9330127018922193,
    0.9330127018922192, 1.25, 1.1160254037844386, 0.9330127018922193, 1.3660254037844386, 1.3660254037844386,
    0.9330127018922193, 1.6160254037844386, 0.25, 1.4330127018922192, 0.43301270189221935, 1.1160254037844384,
    0.4330127018922193, 0.75, 0.75, 0.9330127018922193, 1.1160254037844386, 0.9330127018922193,
    0.9330127018922192, 1.25, 0.9330127018922193, 1.6160254037844386, 0.6160254037844384, 1.4330127018922192,
    1.3660254037844386, 1.3660254037844386, 1.6160254037844386, 0.9330127018922193, 1.799038105676658, 1.2500000000000002,
    1.799038105676658, 1.6160254037844386, 1.6160254037844386, 0.9330127018922193, 1.8660254037844386, 0.5,
    2.299038105676658, 0.75, 1.9820508075688774, 0.9330127018922193, 1.799038105676658, 1.6160254037844386,
    2.116025403784439, 1.4330127018922192, 2.482050807568877, 1.4330127018922192, 2.232050807568877, 1.8660254037844386,
    2.299038105676658, 0.75, 2.732050807568877, 1.0, 2.482050807568877, 1.4330127018922192,
    2.299038105676658, 1.1160254037844386, 1.6160254037844386, 0.9330127018922193, 1.9820508075688774, 0.9330127018922193,
    2.299038105676658, 0.75, 2.299038105676658, 1.1160254037844386, 2.482050807568877, 1.4330127018922192,
    2.116025403784439, 1.4330127018922192, 1.799038105676658, 1.6160254037844386, 1.799038105676658, 1.2500000000000002,
    1.3660254037844386, 2.3660254037844384, 1.799038105676658, 2.1160254037844384, 1.799038105676658, 2.482050807568877,
    1.6160254037844386, 2.799038105676658, 1.6160254037844386, 2.799038105676658, 1.9820508075688776, 2.799038105676658,
    2.299038105676658, 2.982050807568877, 1.8660254037844386, 3.232050807568877, 1.799038105676658, 2.1160254037844384,
    2.232050807568877, 1.8660254037844386, 2.482050807568877, 2.299038105676658, 2.1160254037844393, 2.299038105676658,
    2.299038105676658, 2.6160254037844384, 2.482050807568877, 2.299038105676658, 2.732050807568877, 2.732050807568877,
    2.299038105676658, 2.982050807568877, 1.6160254037844386, 2.799038105676658, 1.799038105676658, 2.482050807568877,
    1.799038105676658, 2.1160254037844384, 2.1160254037844393, 2.299038105676658, 2.482050807568877, 2.299038105676658,
    2.299038105676658, 2.6160254037844384, 2.299038105676658, 2.982050807568877, 1.9820508075688776, 2.799038105676658,
    0.0, 0.0, 0.4330127018922193, 0.25, 0.0, 0.5,
    0.0, 0.5, 0.4330127018922193, 0.25, 0.4330127018922193, 0.75,
    0.0, 0.5, 0.4330127018922193, 0.75, 0.0, 1.0,
    0.4330127018922193, 0.25, 0.8660254037844386, 0.5, 0.4330127018922193, 0.75,
    1.8660254037844386, 0.5, 2.299038105676658, 0.25, 2.299038105676658, 0.75,
    2.299038105676658, 0.25, 2.732050807568877, 0.0, 2.732050807568877, 0.5,
    2.299038105676658, 0.25, 2.732050807568877, 0.5, 2.299038105676658, 0.75,
    2.299038105676658, 0.75, 2.732050807568877, 0.5, 2.732050807568877, 1.0,
    -0.5, 1.8660254037844386, 0.0, 1.8660254037844386, -0.25, 2.299038105676658,
    -0.25, 2.299038105676658, 0.0, 1.8660254037844386, 0.25, 2.299038105676658,
    -0.25, 2.299038105676658, 0.25, 2.299038105676658, 0.0, 2.732050807568877,
    0.0, 1.8660254037844386, 0.5, 1.8660254037844386, 0.25, 2.299038105676658,
    0.8660254037844386, 0.5, 1.1160254037844386, 0.0669872981077807, 1.3660254037844386, 0.5,
    1.1160254037844386, 0.0669872981077807, 1.3660254037844386, -0.3660254037844386, 1.6160254037844386, 0.0669872981077807,
    1.1160254037844386, 0.0669872981077807, 1.6160254037844386, 0.0669872981077807, 1.3660254037844386, 0.5,
    1.3660254037844386, 0.5, 1.6160254037844386, 0.0669872981077807, 1.8660254037844386, 0.5,
    0.8660254037844386, 0.5, 1.3660254037844386, 0.5, 1.1160254037844386, 0.9330127018922193,
    1.1160254037844386, 0.9330127018922193, 1.3660254037844386, 0.5, 1.6160254037844386, 0.9330127018922193,
    1.1160254037844386, 0.9330127018922193, 1.6160254037844386, 0.9330127018922193, 1.3660254037844386, 1.3660254037844386,
    1.3660254037844386, 0.5, 1.8660254037844386, 0.5, 1.6160254037844386, 0.9330127018922193,
    -0.5, 1.8660254037844386, -0.25, 1.4330127018922192, 0.0, 1.8660254037844386,
    -0.25, 1.4330127018922192, 0.0, 1.0, 0.25, 1.4330127018922192,
    -0.25, 1.4330127018922192, 0.25, 1.4330127018922192, 0.0, 1.8660254037844386,
    0.0, 1.8660254037844386, 0.25, 1.4330127018922192, 0.5, 1.8660254037844386,
    1.3660254037844386, 1.3660254037844386, 1.799038105676658, 1.6160254037844386, 1.3660254037844386, 1.8660254037844384,
    1.3660254037844386, 1.8660254037844384, 1.799038105676658, 1.6160254037844386, 1.799038105676658, 2.1160254037844384,
    1.3660254037844386, 1.8660254037844384, 1.799038105676658, 2.1160254037844384, 1.3660254037844386, 2.3660254037844384,
    1.799038105676658, 1.6160254037844386, 2.232050807568877, 1.8660254037844386, 1.799038105676658, 2.1160254037844384,
    0.5, 1.8660254037844386, 0.9330127018922193, 1.6160254037844386, 0.9330127018922193, 2.1160254037844384,
    0.9330127018922193, 1.6160254037844386, 1.3660254037844386, 1.3660254037844386, 1.3660254037844386, 1.8660254037844384,
    0.9330127018922193, 1.6160254037844386, 1.3660254037844386, 1.8660254037844384, 0.9330127018922193, 2.1160254037844384,
    0.9330127018922193, 2.1160254037844384, 1.3660254037844386, 1.8660254037844384, 1.3660254037844386, 2.3660254037844384,
];
/// Expected canonical face vertex counts (52 entries).
#[cfg(test)]
pub(crate) static EXPECTED_LENS: &[usize] = &[
    4, 4, 4, 4, 8, 4, 4, 4, 4, 8,
    4, 4, 4, 4, 8, 4, 4, 4, 4, 8,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3,
];
/// Expected per-face seeds across all tiles, canonical order (seed 0
/// = corner-quadrilateral faces, 1 = octagon faces, 2 = triangle
/// subdivision faces).
#[cfg(test)]
pub(crate) static EXPECTED_SEEDS: &[u32] = &[
    0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0,
    0, 0, 1, 0, 0, 0, 0, 1, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2,
];

/// Hankin motif on the snub-square (3.3.4.3.4) substrate.
pub fn hankin_3_3_4_3_4() -> LatticeTiling {
    let (cells, report) = hankin::build_cells(&tiles(), THETA_E_DEG, DELTA);
    debug_assert!(
        report.complete,
        "all twelve tile instances must pair completely"
    );
    let expected: [usize; 12] = [5, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4];
    debug_assert!(
        report.per_tile.iter().zip(expected.iter()).all(|(t, &e)| t.faces == e),
        "face census per tile instance"
    );
    debug_assert!(
        report
            .per_tile
            .iter()
            .all(|t| (t.area_sum - t.tile_area).abs() < 1e-9),
        "face partition inexact"
    );
    LatticeTiling {
        id: "hankin-3-3-4-3-4",
        name: "Hankin snub-square motif (3.3.4.3.4)",
        description: "Hankin greedy-inference motif on the snub-square (3.3.4.3.4) substrate at Kaplan contact angle 60°, single-point contacts: each of the four square tiles in its 5-face motif (four corner quadrilaterals around a central octagon) and each of the eight triangle tiles in the side-1/2 4-subdivision limit (four equilateral triangles) — 52 faces per lattice cell, wallpaper group p4g with glides off the L/8 grid.",
        lattice: [Vec2::new(1.0 + 2.0 * H, 0.0), Vec2::new(0.0, 1.0 + 2.0 * H)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P4g,
        lattice_kind: LatticeKind::Square,
        category: TilingCategory::Islamic,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
