//! Wavy courses (M82) — the first **cm** wallpaper-group realizer in the
//! catalog, expressed as a [`LatticeTiling`] data table.
//!
//! # The pattern
//!
//! A course (horizontal strip) `k` is bounded by two phase-shifted
//! zigzag waves of period 2: the bottom wave `Z(x−k) + k` and the top
//! wave `4/3 − Z(x−k) + k`, where `Z(t) = |t|/3` on `[−1,1]` extended
//! with period 2 (valleys at the even integers, peaks of height `1/3`
//! at the odd integers, slopes `±1/3`). The identity
//! `Z(t−1) + 1 = 4/3 − Z(t)` makes course `k+1`'s bottom wave coincide
//! exactly with course `k`'s top wave, so the courses stack without
//! gaps or overlaps (vertical width `4/3 − 2Z(x−k)` ∈ `[2/3, 4/3]`).
//!
//! Each column-course quadrilateral (unit width, area 1) is cut by one
//! diagonal seam, and the seam ALTERNATES SLOPE with the
//! column-course parity:
//!
//! * `j−k` even: seam `(j,k) → (j+1,k+1)` (slope +1):
//!   `T_low = (j,k), (j+1,k+1/3), (j+1,k+1)` (area 1/3) and
//!   `T_high = (j,k), (j+1,k+1), (j,k+4/3)` (area 2/3);
//! * `j−k` odd: seam `(j,k+1) → (j+1,k)` (slope −1):
//!   `T_low' = (j,k+1/3), (j+1,k), (j,k+1)` (area 1/3) and
//!   `T_high' = (j,k+1), (j+1,k), (j+1,k+4/3)` (area 2/3).
//!
//! The two seam families are mirror images of each other in every
//! vertical line — that alternation is what makes the vertical mirrors
//! symmetries (a single-seam-slope construction has no mirror at all).
//!
//! The primitive cell of `Λ = ⟨(1,1), (2,0)⟩` (det 2) is the union of
//! column 0 and column 1 of course 0 — four triangles, areas
//! `1/3 + 2/3 + 1/3 + 2/3 = 2 = det Λ`. The four cell triangles fall
//! into two congruence classes: `C1 = {T_low, T_low'}` (squared edges
//! `{4/9, 10/9, 2}`, the 2/3 edge) and `C2 = {T_high, T_high'}`
//! (squared edges `{10/9, 16/9, 2}`, the 4/3 edge) — the classes are
//! INCONGRUENT (different areas), so no symmetry can mix them.
//!
//! # Edge-to-edge note
//!
//! The partition is edge-to-edge: every tile edge is a wave segment
//! (slope `±1/3`), a seam (slope `±1`), or a vertical boundary segment,
//! and every such edge is shared by exactly two tiles — wave edges
//! pair the two courses that share the wave, the vertical boundaries
//! at `x ∈ ℤ` pair the two column tiles that meet there (the seam
//! alternation keeps the vertical edge lengths matched on both sides),
//! and the seams split the quadrilateral internally. Every tile vertex
//! is a wave vertex (an integer or a third), so there are no
//! T-junctions.
//!
//! # Vertex stars
//!
//! Two vertex types, both summing to 360° (no 180° angles):
//!
//! * the integer-y wave vertices (valleys and top peaks): eight tiles
//!   meet with angles `45°` × 4, `arctan(1/2) ≈ 26.5651°` × 2,
//!   `arctan(2) ≈ 63.4349°` × 2;
//! * the third-y wave vertices (the bottom peaks, `y = k + 1/3`): four
//!   tiles meet with angles `90° ± arctan(1/3)` (≈ 108.4349° and
//!   ≈ 71.5651°), two of each.
//!
//! # Wallpaper group
//!
//! **cm** — translations, one mirror family, one glide family, and
//! nothing else:
//!
//! * **Mirrors** `x = j` for ALL integers `j`: the mirror `x = 0`
//!   maps `T_low(0,0)` exactly onto `T_low'(−1,0)` (equivalently the
//!   cell tile `T_low' + (−2,0)`) and `T_high(0,0)` onto
//!   `T_high'(−1,0)` (the even- and odd-column tiles of course 0
//!   swap), C1 → C1, C2 → C2;
//! * **Glides** `g_j(x,y) = (2j + 1 − x, y + 1)` — axes
//!   `x = j + 1/2`, glide vector `(0,1)` — with `g_j² = (0,2) ∈ Λ`;
//!   `g_j` maps `T_low → T_low' + (−1,1)` and
//!   `T_high → T_high' + (−1,1)` (and the inverse images back), so the
//!   glide family is a symmetry;
//! * mirrors and glides with the same linear part form ONE coset of
//!   Λ: `(3,3) ∈ Λ`, so every glide representative `(6j+3, 3)` differs
//!   from a mirror representative `(6j, 0)` by a lattice vector — the
//!   coset contains the mirrors `x = ℤ`, the half-integer-axis glides
//!   of glide (0,1), and their composites (even-glide mirrors on
//!   integer axes, odd-glide glides on half-integer axes).
//!
//! It is cm, and nothing larger, because the exact probe shows no
//! other isometry survives:
//!
//! * no half-turns (all 18 cosets of the 180° part are negative);
//! * no horizontal mirrors or glides (all 18 cosets of the horizontal
//!   mirror part are negative — the third-level vertex geometry breaks
//!   the y-parity that a horizontal symmetry would need);
//! * no 90°/270° rotations, diagonal or anti-diagonal mirrors/glides
//!   (all their cosets are negative — the seam directions ±1 and the
//!   wave directions ±1/3 are closed under `s ↦ −s` but not under
//!   `s ↦ 1/s`);
//! * the translation subgroup is exactly Λ (the identity coset is the
//!   only positive translation coset).
//!
//! It is cm rather than pm because the primitive cell (det 2) carries
//! TWO mirror lines (`x = 0` and `x = 1`), with the glides on the
//! midlines `x = j + 1/2` between them — the defining centered
//! structure of cm.
//!
//! # Bravais kind
//!
//! **Centered-rectangular**: `Λ = ⟨(1,1), (2,0)⟩ = {(x,y) ∈ ℤ² :
//! x + y even}` — the x+y-even sublattice of ℤ². Its rectangular cell
//! `⟨(2,0), (0,2)⟩` has cell center `(1,1) ∈ Λ` (the centering
//! translation). The same lattice is ALSO square — `⟨(1,1), (−1,1)⟩`
//! is an orthogonal equal-length basis — so the lattice-legality
//! measurement reports Square and the declared CenteredRectangular is
//! admitted by the degenerate-realization clause (a square-metric
//! translation lattice under a ≤2-fold point group is a valid
//! centered-rectangular Bravais realization — the pattern has no
//! 4-fold, so the square metric is not forced by the symmetry; the
//! running-bond / M78 / M81 precedent).
//!
//! # Seeds and protos
//!
//! `proto` = cell index (0 = `T_low`, 1 = `T_high`, 2 = `T_low'`,
//! 3 = `T_high'`); `seed` = shape class — two classes (0 = C1, area
//! 1/3, the 2/3 edge; 1 = C2, area 2/3, the 4/3 edge), so the Pattern
//! coloring distinguishes the two triangle shapes.
//!
//! # Verification
//!
//! Verified exactly before porting (Python, stdlib, `m82_verify.py`,
//! integer ×3 coordinates): the four cell polygons are convex CCW
//! with doubled areas 6, 12, 6, 12 (areas 1/3, 2/3, 1/3, 2/3)
//! summing to 36 = 9 · 2 · det Λ; two congruence classes (C1 squared
//! edges ×9 `{4,10,18}`, C2 `{10,16,18}` — incongruent); the window
//! `W = [−4,4]²` (×3) partition (200 window tiles, 124 interior edges
//! paired exactly twice, no T-junctions among 121 vertices, clipped
//! coverage 576 = 576, 0 overlaps, 0 gaps on a 48×48 fine grid); all
//! 53 interior vertex figures sum to 360° with the two-star census
//! above (25 eight-tile stars, 28 four-tile stars); and the full
//! probe battery — 25 lattice translations, 5 mirrors `x = k`,
//! `k = −2..2`, 3 glide samples, and the composites (0,2)/(0,−2)
//! positive — against the complete 144-coset enumeration of all eight
//! D4 linear parts that normalize the square-metric Λ (18 cosets
//! each): exactly the identity coset and the mirror/glide coset are
//! positive (2 pos / 142 neg), plus the named negatives
//! (off-lattice translations; vertical mirror `x = 1/2`; vertical
//! glides with slides (0,1)/(0,2); horizontal mirrors `y = 0, 1/2`;
//! the horizontal glide; half-turns about twelve thirds-grid centers;
//! rot90/270/diagonal/anti-diagonal spot checks) — all matching the
//! cm expectation. Rust: the standard trio, the brute-force culling
//! cross-check (`lattice_culling_matches_brute_force`), the in-module
//! cm group probe (`wavy_courses_wallpaper_group_is_cm`), and the
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

/// The 1/3-area triangle of an even-parity column course (j−k even),
/// translated to (x, y): `(x, y), (x+1, y+1/3), (x+1, y+1)`, CCW —
/// the seam rises (slope +1) from the valley (x, y) to the peak
/// (x+1, y+1).
fn course_low(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x, y),
        Vec2::new(x + 1.0, y + 1.0 / 3.0),
        Vec2::new(x + 1.0, y + 1.0),
    ])
}

/// The 2/3-area triangle of an even-parity column course (j−k even),
/// translated to (x, y): `(x, y), (x+1, y+1), (x, y+4/3)`, CCW —
/// shares the rising seam with [`course_low`].
fn course_high(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x, y),
        Vec2::new(x + 1.0, y + 1.0),
        Vec2::new(x, y + 4.0 / 3.0),
    ])
}

/// The 1/3-area triangle of an odd-parity column course (j−k odd),
/// translated to (x, y): `(x, y+1/3), (x+1, y), (x, y+1)`, CCW — the
/// seam falls (slope −1) from the peak (x, y+1) to the valley
/// (x+1, y): the mirror image of [`course_low`] in the vertical line
/// x + 1.
fn course_low_prime(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x, y + 1.0 / 3.0),
        Vec2::new(x + 1.0, y),
        Vec2::new(x, y + 1.0),
    ])
}

/// The 2/3-area triangle of an odd-parity column course (j−k odd),
/// translated to (x, y): `(x, y+1), (x+1, y), (x+1, y+4/3)`, CCW —
/// shares the falling seam with [`course_low_prime`]: the mirror image
/// of [`course_high`] in the vertical line x + 1.
fn course_high_prime(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x, y + 1.0),
        Vec2::new(x + 1.0, y),
        Vec2::new(x + 1.0, y + 4.0 / 3.0),
    ])
}

/// Wavy courses — zigzag courses of height 4/3, each column-course
/// quadrilateral split by a diagonal seam alternating slope +1/−1 with
/// column-course parity, per primitive cell ⟨(1,1),(2,0)⟩ of the
/// x+y-even lattice (cm).
pub fn wavy_courses() -> LatticeTiling {
    // The primitive cell of Λ = <(1,1),(2,0)> (det 2) is column 0
    // (even parity: T_low, T_high) plus column 1 (odd parity: T_low',
    // T_high') of course 0. The seam alternation (+1 in even columns,
    // −1 in odd columns) is what makes the mirrors x = integer
    // symmetries and leaves the group cm: the primitive cell carries
    // two mirror lines with the glides on the midlines between them.
    let cells = vec![
        Cell {
            poly: course_low(0.0, 0.0),
            seed: 0,
        },
        Cell {
            poly: course_high(0.0, 0.0),
            seed: 1,
        },
        Cell {
            poly: course_low_prime(1.0, 0.0),
            seed: 0,
        },
        Cell {
            poly: course_high_prime(1.0, 0.0),
            seed: 1,
        },
    ];

    LatticeTiling {
        id: "wavy-courses",
        name: "Wavy courses",
        description: "Zigzag courses of height 4/3 on the centered-rectangular lattice <(1,1),(2,0)> (the x+y-even lattice, det 2), each column-course quadrilateral split by a diagonal seam alternating slope +1/-1 with column parity: four triangles per cell, two incongruent shapes (areas 1/3 and 2/3). Edge-to-edge. The catalog's first cm realizer: mirror lines x = integer, glides (2j+1-x, y+1) on the half-integer axes with glide (0,1), no half-turns; the lattice's square metric is a degenerate centered-rectangular realization.",
        lattice: [Vec2::new(1.0, 1.0), Vec2::new(2.0, 0.0)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::Cm,
        lattice_kind: LatticeKind::CenteredRectangular,
        category: TilingCategory::Wallpaper,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
