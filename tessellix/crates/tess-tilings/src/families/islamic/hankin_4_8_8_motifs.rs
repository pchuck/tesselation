//! Kaplan §3.5 design-element motifs on the 4.8.8 substrate.
//!
//! A [`crate::hankin_motifs`] showcase: the truncated-square (4.8.8)
//! substrate of [`super::hankin_4_8_8`] (reused verbatim via `tiles()`,
//! bit-identical tile statics) with a per-tile motif assignment instead
//! of greedy ray inference:
//!
//! * octagon — [`Motif::Rosette`] at the ideal angle θ = π/8 = 22.5°
//!   (Lee's rosette, `h = 1`, `phi = 0`), depth `s = 3`: 32 segments,
//!   9 faces — 8 hexagonal petals + one 32-gon core;
//! * diamond — [`Motif::ExtendedRosette`] at realized outer angle
//!   θ = 3π/8 = 67.5° (child angle θ′ = θ − π/n = 22.5°, fit factor
//!   `F = cos θ / cos(π/n − θ)`), depth `s = 2`: 8 segments, 5 faces —
//!   4 quads + one octagonal core.
//!
//! The extension is exact on the shared chords: each child arm line,
//! extended backward, passes exactly through a parent-edge midpoint
//! (pinned in `hankin_motifs` tests to ~1e-13), so rosette and
//! extended rosette meet in the classical Hankin starburst at every
//! octagon–diamond contact.
//!
//! 14 cells per ⟨(1+√2, 0), (0, 1+√2)⟩ cell, Σ areas = |det| exact.
//! Wallpaper group p4m: verified on the torus segment multiset of a
//! ±2-cell window — rot90 about the octagon centre, rot90 about the
//! diamond centre, and the mirrors y = 0 and y = x all hold exactly,
//! while rot45, half-lattice translations and rot60 fail (motif
//! battery kaplan-22a, `final_out.txt`).

use tess_core::TilingCategory;
use tess_core::geometry::Vec2;
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use super::hankin_motifs::{Motif, build_cells_with_motifs};
use crate::lattice::{similarity_map, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// Per-tile motif assignment over [`super::hankin_4_8_8::tiles()`]
/// (octagon, diamond).
pub(crate) fn motifs() -> Vec<Option<Motif>> {
    vec![
        Some(Motif::Rosette {
            n: 8,
            theta_deg: 22.5,
            s: 3,
            h: 1.0,
            phi_deg: 0.0,
        }),
        Some(Motif::ExtendedRosette {
            n: 4,
            theta_deg: 67.5,
            s: 2,
            h: 1.0,
            phi_deg: 0.0,
        }),
    ]
}

/// Expected per-cell seed layout, vertex counts and full cell-vertex
/// table of this tiling — machine-transcribed from a debug build of
/// this module (kaplan-22a); the Python battery agrees to 4.5e-16.
#[cfg(test)]
pub(crate) static EXPECTED_SEEDS: &[u32] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 2, 3,
];

#[cfg(test)]
pub(crate) static EXPECTED_CELL_LENS: &[usize] = &[
    6, 6, 6, 6, 6, 6, 6, 6, 32, 4, 4, 4, 4, 8,
];

#[cfg(test)]
pub(crate) static EXPECTED_VERTS: &[f64] = &[
    0.0, 0.7071067811865476, 0.3535533905932738, 0.3535533905932738, 0.22570294329727994,
    0.6622116744107289, 0.4619397662556435, 0.8984484973690925, 0.1278504472959939,
    0.8984484973690925, 0.0, 1.2071067811865475, 0.0, 1.2071067811865475, 0.12785044729599396,
    1.5157650650040027, 0.4619397662556436, 1.5157650650040027, 0.22570294329727994,
    1.7520018879623662, 0.3535533905932738, 2.060660171779821, 0.0, 1.7071067811865475,
    0.3535533905932738, 0.3535533905932738, 0.7071067811865476, 0.0, 1.2071067811865475, 0.0,
    0.8984484973690925, 0.12785044729599393, 0.8984484973690925, 0.46193976625564326,
    0.6622116744107289, 0.22570294329727988, 0.3535533905932738, 2.060660171779821,
    0.6622116744107289, 2.188510619075815, 0.8984484973690925, 1.9522737961174519,
    0.8984484973690925, 2.286363115077101, 1.2071067811865475, 2.414213562373095,
    0.7071067811865476, 2.414213562373095, 1.2071067811865475, 0.0, 1.7071067811865475, 0.0,
    2.060660171779821, 0.3535533905932738, 1.752001887962366, 0.22570294329727997,
    1.5157650650040027, 0.46193976625564337, 1.5157650650040027, 0.1278504472959939,
    1.2071067811865475, 2.414213562373095, 1.5157650650040027, 2.286363115077101,
    1.5157650650040027, 1.9522737961174517, 1.752001887962366, 2.188510619075815,
    2.060660171779821, 2.060660171779821, 1.7071067811865475, 2.414213562373095,
    1.9522737961174514, 0.8984484973690926, 2.188510619075815, 0.6622116744107289,
    2.060660171779821, 0.3535533905932738, 2.414213562373095, 0.7071067811865476,
    2.414213562373095, 1.2071067811865475, 2.286363115077101, 0.8984484973690925,
    1.9522737961174519, 1.5157650650040027, 2.286363115077101, 1.5157650650040027,
    2.414213562373095, 1.2071067811865475, 2.414213562373095, 1.7071067811865475,
    2.060660171779821, 2.060660171779821, 2.188510619075815, 1.7520018879623658, 0.0,
    1.2071067811865475, 0.1278504472959939, 0.8984484973690925, 0.4619397662556435,
    0.8984484973690925, 0.22570294329727994, 0.6622116744107289, 0.3535533905932738,
    0.3535533905932738, 0.6622116744107289, 0.22570294329727988, 0.8984484973690925,
    0.46193976625564326, 0.8984484973690925, 0.12785044729599393, 1.2071067811865475, 0.0,
    1.5157650650040027, 0.1278504472959939, 1.5157650650040027, 0.46193976625564337,
    1.752001887962366, 0.22570294329727997, 2.060660171779821, 0.3535533905932738,
    2.188510619075815, 0.6622116744107289, 1.9522737961174514, 0.8984484973690926,
    2.286363115077101, 0.8984484973690925, 2.414213562373095, 1.2071067811865475,
    2.286363115077101, 1.5157650650040027, 1.9522737961174519, 1.5157650650040027,
    2.188510619075815, 1.7520018879623658, 2.060660171779821, 2.060660171779821,
    1.752001887962366, 2.188510619075815, 1.5157650650040027, 1.9522737961174517,
    1.5157650650040027, 2.286363115077101, 1.2071067811865475, 2.414213562373095,
    0.8984484973690925, 2.286363115077101, 0.8984484973690925, 1.9522737961174519,
    0.6622116744107289, 2.188510619075815, 0.3535533905932738, 2.060660171779821,
    0.22570294329727994, 1.7520018879623662, 0.4619397662556436, 1.5157650650040027,
    0.12785044729599396, 1.5157650650040027, -0.7071067811865476, 0.0, -0.3535533905932738,
    -0.3535533905932738, -0.20710678118654757, 0.0, -0.3535533905932738, 0.3535533905932738,
    -0.3535533905932738, -0.3535533905932738, 0.0, -0.7071067811865476, 0.3535533905932738,
    -0.3535533905932738, 0.0, -0.20710678118654754, -0.3535533905932738, 0.3535533905932738,
    0.0, 0.20710678118654757, 0.3535533905932738, 0.3535533905932738, 0.0, 0.7071067811865476,
    0.20710678118654757, 0.0, 0.3535533905932738, -0.3535533905932738, 0.7071067811865476, 0.0,
    0.3535533905932738, 0.3535533905932738, -0.3535533905932738, -0.3535533905932738, 0.0,
    -0.20710678118654754, 0.3535533905932738, -0.3535533905932738, 0.20710678118654757, 0.0,
    0.3535533905932738, 0.3535533905932738, 0.0, 0.20710678118654757, -0.3535533905932738,
    0.3535533905932738, -0.20710678118654757, 0.0,
];

/// Hankin §3.5 motif khatam on the 4.8.8 substrate.
pub fn hankin_4_8_8_motifs() -> LatticeTiling {
    let tiles = super::hankin_4_8_8::tiles();
    let (cells, report) = build_cells_with_motifs(&tiles, &motifs());
    debug_assert!(report.complete, "motif figures incomplete");
    debug_assert!(
        report.per_tile.iter().all(|t| (t.area_sum - t.tile_area).abs() < 1e-9),
        "face partition inexact"
    );
    let a = 1.0 + core::f64::consts::SQRT_2;
    LatticeTiling {
        id: "hankin-4-8-8-motifs",
        name: "Hankin motif khatam (4.8.8)",
        description: "Kaplan §3.5 design-element motifs (diss. §3.5) on the 4.8.8 substrate: ideal rosette theta = 22.5 degrees depth s = 3 on the octagon (8 hexagonal petals + 32-gon core), extension operator at realized outer angle 67.5 degrees depth s = 2 on the diamond (4 quads + octagonal core); 14 cells per A*Z^2 cell (A = 1+sqrt(2), |det| = 3+2*sqrt(2)), wallpaper group p4m.",
        lattice: [Vec2::new(a, 0.0), Vec2::new(0.0, a)],
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
