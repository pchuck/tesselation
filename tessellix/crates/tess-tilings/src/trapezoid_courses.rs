//! Trapezoid courses (M79) — the first **pm** wallpaper realizer in
//! the catalog, expressed as a [`LatticeTiling`] data table.
//!
//! # The pattern
//!
//! Horizontal courses of trapezoids. Each 2×1 cell of the rectangular
//! lattice `Λ = ⟨(2,0), (0,1)⟩` (det 2) holds two courses of two
//! trapezoids each, separated by two slants of slope `±1/5` (the left
//! slant is `y = 3/10 + (1/5)x`, the right slant its mirror in the
//! line `x = 1`). The four cell tiles:
//!
//! * `P   = (0,3/10), (1,1/2), (1,1), (0,1)` — upper course, left
//!   half (area 6/10),
//! * `mP  = (1,1/2), (2,3/10), (2,1), (1,1)` — upper course, right
//!   half, the mirror of `P` in `x = 1` (area 6/10),
//! * `P'  = (0,0), (1,0), (1,1/2), (0,3/10)` — lower course, left
//!   half (area 4/10),
//! * `mP' = (1,0), (2,0), (2,3/10), (1,1/2)` — lower course, right
//!   half, the mirror of `P'` in `x = 1` (area 4/10).
//!
//! The upper course (`P`, `mP`) has horizontal top edges at `y = 1`
//! and slants dipping to `y = 1/2` at the cell center; the lower
//! course (`P'`, `mP'`) is the complementary "peak". Areas
//! `6/10 + 6/10 + 4/10 + 4/10 = 2 = |det Λ|`, so the lattice
//! translates tile the plane with no gaps and no overlaps.
//!
//! The shape parameters are chosen to force the group: the slope
//! `m = 1/5 ∉ {0, ±1}` (no parallelogram or axis-aligned degeneracy)
//! and the intercept `c = 3/10 ≠ (1 − m)/2 = 2/5` — with
//! `c = (1 − m)/2` the two trapezoid classes would be congruent
//! (isosceles pair), half-turns would appear, and the group would be
//! pmm. With `c = 3/10` the classes are incongruent: `P` has parallel
//! sides `1 − c = 7/10` and `1 − c − m = 1/2` (area 6/10), `P'` has
//! `c = 3/10` and `c + m = 1/2` (area 4/10) — already distinguishable
//! by area.
//!
//! # Edge-to-edge note
//!
//! The partition is edge-to-edge: every edge is shared by exactly two
//! tiles and every vertex figure sums to 360°. Edge classes: the
//! slant edges (slope `±1/5`, length `√(1 + (1/5)²) = √65/5`), each
//! pairing one upper with one lower trapezoid; the horizontal unit
//! edges at `y = 0` and `y = 1` (paired between vertical cell
//! translates); and the vertical edges at `x ∈ {0, 1, 2}` of lengths
//! `3/10`, `1/2`, `7/10` (at `x = 1` inside the cell, at `x = 0, 2`
//! between cells). There are no T-junctions: every slant endpoint is
//! a vertex of all four incident tiles.
//!
//! # Vertex stars
//!
//! Two vertex types, both summing to 360°:
//!
//! * the slant endpoints (e.g. `(1,1/2)`- and `(0,3/10)`-type
//!   points): four tiles meet with angles
//!   `arctan(5) = 78.6901°`, `180° − arctan(5) = 101.3099°` — two of
//!   each;
//! * the horizontal-line vertices `(k, 0)` and `(k, 1)`, `k ∈ ℤ`:
//!   `90°` × 4.
//!
//! # Wallpaper group
//!
//! **pm** — translations plus one mirror family. The mirror lines are
//! exactly the vertical lines `x = k`, `k ∈ ℤ`: the mirror `x = 1`
//! exchanges `P ↔ mP` and `P' ↔ mP'` within one cell, and the mirror
//! `x = 0` exchanges each cell with the mirror image in the adjacent
//! cell (a lattice translate). Mirrors at `x = k + 1/2` are NOT
//! symmetries (they would land each trapezoid between two tiles).
//!
//! The group is pm, not pmm, because there is no second mirror family
//! and no half-turn:
//!
//! * a horizontal mirror `(x, y) ↦ (x, 2b − y)` sends the upper
//!   course (horizontal top edge `y = 1`, area 6/10) to a tile with a
//!   horizontal edge at `y = 2b − 1 ∈ {0, 1}` — at `b = 1/2` that is
//!   a lower-course tile (area 4/10), impossible by area; at
//!   `b = 1` the image is an upper-course tile with the slant
//!   reversed, which is not a tile;
//! * a half-turn `(x, y) ↦ (2a − x, 2b − y)` has the same dichotomy
//!   (center on `y = 1/2 + n`: area contradiction; center on
//!   `y = 1 + n`: the image slant is not a tile edge).
//!
//! The incongruence of the two prototile classes (areas 6/10 vs 4/10)
//! is the structural reason half-turns are absent: any half-turn or
//! horizontal mirror of the pattern must preserve the two courses'
//! area classes, and the exact probe below confirms no such isometry
//! survives.
//!
//! # Bravais kind
//!
//! **Rectangular** (primitive): the declared basis `(2,0), (0,1)` is
//! an orthogonal pair of unequal length, and the lattice-legality test
//! pins the declaration to the measured kind.
//!
//! # Seeds and protos
//!
//! `proto` = cell index (`0` = `P`, `1` = `mP`, `2` = `P'`, `3` =
//! `mP'`); `seed` = shape class — two classes (0 = the 6/10
//! trapezoid and its mirror, 1 = the 4/10 trapezoid and its mirror),
//! so the pattern coloring distinguishes the two courses.
//!
//! # Verification
//!
//! Verified exactly before porting (Python, stdlib,
//! `m79_verify.py`): the four cell polygons are convex CCW with
//! areas 6/10, 6/10, 4/10, 4/10 summing to |det| = 2; the cell
//! partition is boundary-only (pairwise interiors disjoint, union =
//! cell); edge-to-edge over windows W = 2/3/5 (every interior edge
//! paired exactly twice, no T-junctions, no crossings); every vertex
//! figure sums to 360° over a 121-interior-vertex window (star census
//! `{78.6901° × 2, 101.3099° × 2}` and `{90° × 4}`); `P` and `P'`
//! incongruent (area and squared-edge multisets); the full probe
//! battery — 15 positive candidates (the lattice translations
//! `(2i, 0j)` in a window and the vertical mirrors `x = k` with
//! lattice-translate composites) against 53 negative candidates
//! (off-lattice translations, half-integer vertical mirrors, 18
//! half-turn centers, horizontal mirrors, horizontal glides,
//! rot90/rot270, diagonal and anti-diagonal mirrors/glides, vertical
//! glides) — matches the pm expectation exactly. Rust: the standard
//! trio, the brute-force culling cross-check
//! (`lattice_culling_matches_brute_force`), the in-module pm group
//! probe (`trapezoid_courses_wallpaper_group_is_pm`), and the
//! lattice-legality test
//! (`declared_symmetry_is_legal_and_matches_the_measured_lattice`).

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

fn poly(pts: &[Vec2]) -> Polygon {
    Polygon::from_slice(pts)
}

/// The 6/10 trapezoid (upper course, left half) translated to `(x, y)`:
/// `(x, y+3/10), (x+1, y+1/2), (x+1, y+1), (x, y+1)`, CCW — slant
/// rising to the right.
fn trapezoid(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x, y + 0.3),
        Vec2::new(x + 1.0, y + 0.5),
        Vec2::new(x + 1.0, y + 1.0),
        Vec2::new(x, y + 1.0),
    ])
}

/// Mirror of [`trapezoid`] in the vertical line `x + 1` (upper course,
/// right half): `(x, y+1/2), (x+1, y+3/10), (x+1, y+1), (x, y+1)`,
/// CCW — slant falling to the right.
fn trapezoid_mirrored(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x, y + 0.5),
        Vec2::new(x + 1.0, y + 0.3),
        Vec2::new(x + 1.0, y + 1.0),
        Vec2::new(x, y + 1.0),
    ])
}

/// The 4/10 trapezoid (lower course, left half) translated to `(x, y)`:
/// `(x, y), (x+1, y), (x+1, y+1/2), (x, y+3/10)`, CCW.
fn trapezoid_complement(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x, y),
        Vec2::new(x + 1.0, y),
        Vec2::new(x + 1.0, y + 0.5),
        Vec2::new(x, y + 0.3),
    ])
}

/// Mirror of [`trapezoid_complement`] in the vertical line `x + 1`
/// (lower course, right half):
/// `(x, y), (x+1, y), (x+1, y+3/10), (x, y+1/2)`, CCW.
fn trapezoid_complement_mirrored(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x, y),
        Vec2::new(x + 1.0, y),
        Vec2::new(x + 1.0, y + 0.3),
        Vec2::new(x, y + 0.5),
    ])
}

/// Trapezoid courses — two courses of trapezoids per 2×1 cell on the
/// rectangular lattice ⟨(2,0),(0,1)⟩ (pm).
pub fn trapezoid_courses() -> LatticeTiling {
    // Two courses of two trapezoids each: the upper course P, mP
    // (areas 6/10, horizontal tops at y = 1) and the lower course P',
    // mP' (areas 4/10, horizontal bottoms at y = 0). The right-half
    // tiles are the mirrors of the left-half tiles in x = 1, so the
    // mirror line x = 1 (and every integer x) is a tiling symmetry.
    let cells = vec![
        Cell {
            poly: trapezoid(0.0, 0.0),
            seed: 0,
        },
        Cell {
            poly: trapezoid_mirrored(1.0, 0.0),
            seed: 0,
        },
        Cell {
            poly: trapezoid_complement(0.0, 0.0),
            seed: 1,
        },
        Cell {
            poly: trapezoid_complement_mirrored(1.0, 0.0),
            seed: 1,
        },
    ];

    LatticeTiling {
        id: "trapezoid-courses",
        name: "Trapezoid courses",
        description: "Two courses of trapezoids per 2x1 rectangular cell <(2,0),(0,1)> (det 2): four trapezoids, two incongruent shapes (areas 3/5 and 2/5) each with its mirror in x = 1. Edge-to-edge (slant slope 1/5), vertical mirror lines at x = integer, no horizontal mirrors or half-turns. Wallpaper group pm: one mirror family; the incongruent prototile classes kill pmm.",
        lattice: [Vec2::new(2.0, 0.0), Vec2::new(0.0, 1.0)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::Pm,
        lattice_kind: LatticeKind::Rectangular,
        category: TilingCategory::Wallpaper,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
