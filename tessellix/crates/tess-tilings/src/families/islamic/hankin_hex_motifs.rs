//! Kaplan §3.5 design-element motifs on the regular hexagonal (6³)
//! substrate.
//!
//! Two flat-top unit hexagons (vertices at 30° + 60k, edge 1) in a
//! rectangular 2-hex cell ⟨(√3, 0), (0, 3)⟩ (det 3√3):
//!
//! * hexagon A at the origin — [`Motif::Rosette`] at the ideal angle
//!   θ = π/6 = 30° (`h = 1`, `phi = 0`), depth `s = 2`: 24 segments,
//!   7 faces — 6 hexagonal petals + one 24-gon core;
//! * hexagon B at (√3/2, 3/2) — [`Motif::ExtendedStar`] at realized
//!   outer angle θ = π/3 = 60° (child angle θ′ = 30°, fit factor
//!   `F = cos 60° / cos(30° − 60°)`), depth `s = 1`: 12 segments,
//!   7 faces — 6 quads + one 12-gon core.
//!
//! The motif assignment deliberately breaks the substrate's 6-fold
//! symmetry: a 60° rotation about A maps hexagon A onto a class-B
//! centre, swapping rosettes and stars, so it is NOT a symmetry of the
//! motif pattern. The preserved group is pmm, verified exactly on the
//! torus segment multiset of a ±2-cell window: mirrors x = 0, y = 0,
//! x = √3/2 and y = 3/2 plus rot180 about A and about the shared-edge
//! midpoint, while rot60/rot120 about both centres, the 30° mirror,
//! rot90 and the half-diagonal translation all fail as expected (motif
//! battery kaplan-22a, `final_out.txt`).
//!
//! # Colour is not what makes it pmm (review A7)
//!
//! Symmetry labels in this catalog are measured from tile GEOMETRY and
//! never from the colouring (`tess_core::symmetry` carries the group and
//! lattice the tiling declares; the `Pattern` colour mode is a display
//! decoration on top of it — the same group label is shown in every
//! colour mode). So the pmm above does not depend on "the coloured rows
//! counting": what the probe actually checks is that the A/B assignment
//! is preserved, and A and B are different MOTIF CLASSES — a 24-segment
//! rosette versus a 12-segment extended star — i.e. a geometric
//! distinction visible in the emitted segment multiset, not a paint
//! distinction. The single-class tilings are separate registry entries
//! with the higher group: the plain regular hexagonal 6³ tiling
//! (`hexagon`, p6m) and the single-construction Hankin hex weave
//! (`hankin-hex-two-point`, p6m), where every hexagon carries the same
//! motif.

use tess_core::TilingCategory;
use tess_core::geometry::Vec2;
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use super::hankin_motifs::{Motif, build_cells_with_motifs};
use crate::lattice::{similarity_map, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// √3/2 (nearest double; `SQRT_3` in `core::f64::consts` is unstable
/// in this toolchain — local const, same convention as the other
/// hexagonal tables).
const H3: f64 = 0.8660254037844386;

/// Substrate tile class 0: flat-top unit hexagon at the origin
/// (vertices at 30° + 60k, CCW).
static TILE_0: [Vec2; 6] = [
    Vec2::new(H3, 0.5),
    Vec2::new(0.0, 1.0),
    Vec2::new(-H3, 0.5),
    Vec2::new(-H3, -0.5),
    Vec2::new(0.0, -1.0),
    Vec2::new(H3, -0.5),
];

/// Substrate tile class 1: the same hexagon translated to (√3/2, 3/2)
/// (the second hexagon of the rectangular 2-hex cell).
static TILE_1: [Vec2; 6] = [
    Vec2::new(H3 + H3, 2.0),
    Vec2::new(H3, 2.5),
    Vec2::new(0.0, 2.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(H3, 0.5),
    Vec2::new(H3 + H3, 1.0),
];

/// Template tiles in canonical class order.
pub(crate) fn tiles() -> Vec<&'static [Vec2]> {
    vec![&TILE_0, &TILE_1]
}

/// Per-tile motif assignment over [`tiles()`].
pub(crate) fn motifs() -> Vec<Option<Motif>> {
    vec![
        Some(Motif::Rosette {
            n: 6,
            theta_deg: 30.0,
            s: 2,
            h: 1.0,
            phi_deg: 0.0,
        }),
        Some(Motif::ExtendedStar {
            n: 6,
            theta_deg: 60.0,
            s: 1,
        }),
    ]
}

/// Expected per-cell seed layout, vertex counts and full cell-vertex
/// table of this tiling — machine-transcribed from a debug build of
/// this module (kaplan-22a); the Python battery agrees to 4.5e-16.
#[cfg(test)]
pub(crate) static EXPECTED_SEEDS: &[u32] = &[
    0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 3,
];

#[cfg(test)]
pub(crate) static EXPECTED_CELL_LENS: &[usize] = &[
    6, 6, 6, 6, 6, 6, 24, 4, 4, 4, 4, 4, 4, 12,
];

#[cfg(test)]
pub(crate) static EXPECTED_VERTS: &[f64] = &[
    -0.8660254037844386, -0.5, -0.4330127018922193, -0.75, -0.5773502691896257,
    -0.5000000000000001, -0.4330127018922192, -0.25, -0.7216878364870322, -0.25,
    -0.8660254037844386, 0.0, -0.8660254037844386, 0.0, -0.7216878364870322, 0.25,
    -0.43301270189221924, 0.25, -0.5773502691896257, 0.5000000000000001, -0.4330127018922193,
    0.75, -0.8660254037844386, 0.5, -0.4330127018922193, -0.75, 0.0, -1.0, 0.4330127018922193,
    -0.75, 0.14433756729740654, -0.75, 0.0, -0.49999999999999994, -0.14433756729740654, -0.75,
    -0.4330127018922193, 0.75, -0.14433756729740654, 0.75, 0.0, 0.49999999999999994,
    0.14433756729740654, 0.75, 0.4330127018922193, 0.75, 0.0, 1.0, 0.4330127018922193, -0.75,
    0.8660254037844386, -0.5, 0.8660254037844386, 0.0, 0.7216878364870322, -0.25,
    0.43301270189221924, -0.25, 0.5773502691896257, -0.5000000000000001, 0.43301270189221924,
    0.25, 0.7216878364870322, 0.25, 0.8660254037844386, 0.0, 0.8660254037844386, 0.5,
    0.4330127018922193, 0.75, 0.5773502691896257, 0.5000000000000001, -0.8660254037844386, 0.0,
    -0.7216878364870322, -0.25, -0.4330127018922192, -0.25, -0.5773502691896257,
    -0.5000000000000001, -0.4330127018922193, -0.75, -0.14433756729740654, -0.75, 0.0,
    -0.49999999999999994, 0.14433756729740654, -0.75, 0.4330127018922193, -0.75,
    0.5773502691896257, -0.5000000000000001, 0.43301270189221924, -0.25, 0.7216878364870322,
    -0.25, 0.8660254037844386, 0.0, 0.7216878364870322, 0.25, 0.43301270189221924, 0.25,
    0.5773502691896257, 0.5000000000000001, 0.4330127018922193, 0.75, 0.14433756729740654,
    0.75, 0.0, 0.49999999999999994, -0.14433756729740654, 0.75, -0.4330127018922193, 0.75,
    -0.5773502691896257, 0.5000000000000001, -0.43301270189221924, 0.25, -0.7216878364870322,
    0.25, 0.0, 1.0, 0.4330127018922193, 0.75, 0.43301270189221924, 1.25, 0.0, 1.5, 0.0, 1.5,
    0.4330127018922193, 1.75, 0.4330127018922193, 2.25, 0.0, 2.0, 0.4330127018922193, 0.75,
    0.8660254037844386, 0.5, 1.299038105676658, 0.75, 0.8660254037844386, 1.0,
    0.4330127018922193, 2.25, 0.8660254037844386, 2.0, 1.299038105676658, 2.25,
    0.8660254037844386, 2.5, 1.299038105676658, 0.75, 1.7320508075688772, 1.0,
    1.7320508075688772, 1.5, 1.299038105676658, 1.25, 1.299038105676658, 1.75,
    1.7320508075688772, 1.5, 1.7320508075688772, 2.0, 1.299038105676658, 2.25, 0.0, 1.5,
    0.43301270189221924, 1.25, 0.4330127018922193, 0.75, 0.8660254037844386, 1.0,
    1.299038105676658, 0.75, 1.299038105676658, 1.25, 1.7320508075688772, 1.5,
    1.299038105676658, 1.75, 1.299038105676658, 2.25, 0.8660254037844386, 2.0,
    0.4330127018922193, 2.25, 0.4330127018922193, 1.75,
];

/// Hankin §3.5 motif hex weave (rosette + extended star).
pub fn hankin_hex_motifs() -> LatticeTiling {
    let (cells, report) = build_cells_with_motifs(&tiles(), &motifs());
    debug_assert!(report.complete, "motif figures incomplete");
    debug_assert!(
        report.per_tile.iter().all(|t| (t.area_sum - t.tile_area).abs() < 1e-9),
        "face partition inexact"
    );
    LatticeTiling {
        id: "hankin-hex-motifs",
        name: "Hankin motif hex weave (6³)",
        description: "Kaplan §3.5 design-element motifs on the regular hexagonal (6^3) substrate, rectangular 2-hex cell: ideal rosette theta = 30 degrees depth s = 2 on the origin hexagon (6 hexagonal petals + 24-gon core), extension operator at realized outer angle 60 degrees depth s = 1 on the offset hexagon (6 quads + 12-gon core); the class assignment breaks the substrate's 6-fold symmetry, 14 cells per <(sqrt(3),0),(0,3)> cell (|det| = 3*sqrt(3)); wallpaper group pmm.",
        lattice: [Vec2::new(H3 + H3, 0.0), Vec2::new(0.0, 3.0)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::Pmm,
        lattice_kind: LatticeKind::Rectangular,
        category: TilingCategory::Islamic,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
