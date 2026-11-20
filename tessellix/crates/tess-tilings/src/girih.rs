//! Girih shesh-band tiling — the periodic band limit of the classical five
//! girih tile shapes, expressed as a [`LatticeTiling`] data table.
//!
//! * `6.6.6` — Girih Shesh-Band (p2)
//!
//! The *shesh band* (Persian: شش‌باند, "six-band") is the elongated irregular
//! convex hexagon among the five classical girih tiles (Wikipedia "Girih tile"
//! article): all edges unit-length, interior angles 2 × [72°, 144°, 144°].
//!
//! The tile is centrally symmetric (180° about its centroid maps v0↔v3,
//! v1↔v4, v2↔v5), so it tiles by pure translation on the centered
//! rectangular lattice `span{t1, t4}` where
//! `t1 = (−(cos36°+cos72°), −(sin36°+sin72°))` and
//! `t4 = (1−cos72°, −sin72°)`.
//! `det(t1, t4) = area(tile)` exactly, giving one tile per cell.
//!
//! Every interior vertex figure is 6.6.6 (three hexagons, angles 72°/144°/144°).
//! The wallpaper group is p2 (180° rotations about each tile centroid; no
//! mirror axes). The girih straps form a periodic band pattern — the
//! band-only limit of girih strapwork with no stars or crosses.
//!
//! This is the *periodic stand-in* for the genuinely quasiperiodic girih
//! star-and-cross tiling (see the separate `girih-quasiperiodic` entry, M18).
//!
//! # Seeds
//!
//! One tile per cell — a single orbit, so there is no orbit-distinct
//! coloring to ship.  The shipped seed is therefore *decorative*:
//! `SeedPattern::Checker` colors lattice sites by the parity of
//! `(i + j)` on the centered-rectangular basis — a lattice-parity
//! coloring, not an orbit class.

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

// Radical constants (exact double values; f64::sqrt is not const-stable).
const SQ5: f64 = 2.236_067_977_499_79; // √5
const C36: f64 = (1.0 + SQ5) / 4.0; // cos 36° = (1+√5)/4
const C72: f64 = (SQ5 - 1.0) / 4.0; // cos 72° = (√5−1)/4
const S36: f64 = 0.5877852522924731; // sin 36° = √(10−2√5)/4
const S72: f64 = 0.9510565162951535; // sin 72° = √(10+2√5)/4

/// Girih shesh-band (elongated hexagon) lattice tiling, wallpaper group p2.
///
/// Single-tile centered-rectangular lattice tiling. The shesh-band hexagon (all edges
/// unit-length, angles 2×[72°,144°,144°]) tiles by translation on
/// `span{t1, t4}` with `det(t1,t4) = tile area ≈ 2.1266` (exact in radicals).
/// One tile per cell. Vertex figure 6.6.6 at every interior vertex
/// (72°/144°/144°). The pattern has 180° rotational symmetry about each
/// tile centroid (p2); no reflections.
pub fn girih_shesh_band() -> LatticeTiling {
    // Shesh-band hexagon, CCW, unit edge length.
    // Angles: 72°, 144°, 144°, 72°, 144°, 144° (at v0..v5).
    // Centrally symmetric: 180° about centroid maps v0↔v3, v1↔v4, v2↔v5.
    let shb = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0 + C36, S36),
        Vec2::new(1.0 + C36 + C72, S36 + S72),
        Vec2::new(C36 + C72, S36 + S72),
        Vec2::new(C72, S72),
    ]);

    LatticeTiling {
        id: "girih-shesh-band",
        name: "Girih Shesh-Band (6.6.6)",
        description: "Periodic girih tiling by the shesh-band (elongated hexagon) tile — the six-sided girih prototile (2×[72°,144°,144°], unit edges). Single-tile centered rectangular lattice (p2); vertex figure 6.6.6. The periodic band-only limit of girih strapwork: parallel straight straps, no stars.",
        lattice: [
            Vec2::new(-(C36 + C72), -(S36 + S72)), // t1
            Vec2::new(1.0 - C72, -S72),            // t4
        ],
        cells: vec![
            Cell { poly: shb, seed: 0 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P2,
        lattice_kind: LatticeKind::CenteredRectangular,
        category: TilingCategory::Islamic,
        secondary: &[],
        seed_pattern: SeedPattern::Checker,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
