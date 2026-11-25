//! Shifted trapezoid courses (M81) — the first **pmg** wallpaper-group
//! realizer in the catalog, expressed as a [`LatticeTiling`] data table.
//!
//! # The pattern
//!
//! The M79 trapezoid-course row stacked in horizontal rows of height 1,
//! with ALTERNATE ROWS reflected in the horizontal line `y = 1` and
//! shifted by `(1,0)`. Each 2×2 cell of the rectangular lattice
//! `Λ = ⟨(2,0), (0,2)⟩` (det 4) holds two such rows, eight tiles:
//!
//! * row 0 (`y ∈ [0,1]`) — the M79 pm cell:
//!   `P = (0,3/10), (1,1/2), (1,1), (0,1)` (area 6/10, slant rising to
//!   the right), its mirror `mP` in `x = 1`, and the complementary
//!   lower course `P' = (0,0), (1,0), (1,1/2), (0,3/10)` (area 4/10)
//!   with its mirror `mP'`;
//! * row 1 (`y ∈ [1,2]`) — `F(row 0) + (1,0)` with `F(x,y) = (x, 2−y)`:
//!   `R0 = (1,1), (2,1), (2,15/10), (1,17/10)`,
//!   `R1 = (1,17/10), (2,15/10), (2,2), (1,2)`, and the right-half
//!   mirrors `R2`, `R3` in `x = 2` (areas 6/10, 4/10, 6/10, 4/10, each
//!   the mirror-in-a-vertical-line of its row-0 twin).
//!
//! The `(1,0)` shift is what makes the group pmg rather than pmm:
//! row 1 is the vertical flip of row 0 *displaced by half the x-period
//! of the tiling* (the tiling's x-period is 2, and `(1,0) ∉ Λ`…
//! precisely, `F(P) = R0 + (−1,0)` and `(1,0)` is not a translation of
//! the tiling). The flip therefore lands the row-1 slant between the
//! row-0 slant pairs, so no horizontal mirror can be a symmetry, while
//! the *glide* with glide 1 absorbs exactly that half-period displacement.
//!
//! The shape parameters are inherited from M79 and already force
//! incongruence: slope `m = 1/5 ∉ {0, ±1}` and intercept `c = 3/10 ≠
//! (1 − m)/2 = 2/5`, so the 6/10 and 4/10 trapezoid classes are
//! distinguishable by area.
//!
//! # Edge-to-edge note
//!
//! The partition is edge-to-edge: every edge is shared by exactly two
//! tiles and every vertex figure sums to 360°. The slants pair within
//! each row — row 0: `P ↔ P'` and `mP ↔ mP'` at the `3/10`/`1/2`
//! levels (as in M79); row 1: `R0 ↔ R1` and `R2 ↔ R3` at the
//! `15/10`/`17/10` levels — and the flat–flat seam at every integer
//! `y` pairs row-1 bottoms with row-0 tops ACROSS cell boundaries
//! (the `(1,0)` shift): the top edge of `P` at `y = 1`,
//! `(0,1)–(1,1)`, pairs with the bottom of `R2 + (−2,0)` — a row-1
//! tile in the cell to the left. Edge classes (exact lengths): the
//! slant edges (slope `±1/5`, length `√26/5`); the horizontal unit
//! edges at `y ∈ ℤ`; and the vertical edges at `x ∈ ℤ` of lengths
//! `3/10`, `1/2`, `7/10` (row-0 verticals at the `3/10`/`1/2`/`7/10`
//! levels; row-1 verticals at the `5/10`/`7/10` levels). There are no
//! T-junctions: every slant endpoint is a vertex of all four incident
//! tiles.
//!
//! # Vertex stars
//!
//! Two vertex types, both summing to 360° (the same two stars M79
//! carries, the row-1 flip preserving the slant angles):
//!
//! * the slant endpoints (the `3/10`/`1/2`-level points in row 0 and
//!   the `15/10`/`17/10`-level points in row 1): four tiles meet with
//!   angles `arctan(5) = 78.6901°`, `180° − arctan(5) = 101.3099°` —
//!   two of each;
//! * the flat–flat seam vertices `(k, n)` and `(k+1, n)`, `k, n ∈ ℤ`:
//!   `90°` × 4.
//!
//! # Wallpaper group
//!
//! **pmg** — translations, one mirror family, one perpendicular glide
//! family, and half-turns at the mirror × glide intersections:
//!
//! * **Mirrors** `x = k`, `k ∈ ℤ`: the mirror `x = 1` exchanges
//!   `P ↔ mP` and `P' ↔ mP'` within row 0 (and `R0 ↔ R2 + (−2,0)`,
//!   `R1 ↔ R3 + (−2,0)` within row 1); the mirror `x = 0` exchanges
//!   each cell with the mirror image in the adjacent cell (a lattice
//!   translate). All vertical mirrors are one orbit.
//! * **Glides** `g(x,y) = (x + 1, −y)` (axis `y = 0`, glide 1):
//!   `g(P) = R0 + (0,−2)`, `g(P') = R1 + (0,−2)`,
//!   `g(mP) = R2 + (0,−2)`, `g(mP') = R3 + (0,−2)`, and `g² = (2,0)
//!   ∈ Λ`. The family `g · Λ = {g ∘ (2i, 2j)}` is the single horizontal
//!   glide family: glide 1 on every axis `y = j`, `j ∈ ℤ` — e.g.
//!   `g ∘ (0,2) = (x + 1, 2 − y)` is the glide on `y = 1` and maps
//!   `P` to `R0` with no translate at all.
//! * **Half-turns** about `(m + 1/2, n)` for ALL `m, n ∈ ℤ` — four
//!   per 2×2 cell: `(1/2, 0)`, `(3/2, 0)`, `(1/2, 1)`, `(3/2, 1)`.
//!   The even-y centers (e.g. `(1/2, 0)`) map `P` to
//!   `R2 + (−2,−2)`; the odd-y centers (e.g. `(1/2, 1)`) map `P` to
//!   `R2 + (−2,0)` and `P'` to `R1`. The centers are the
//!   mirror × glide intersections and form two Λ-orbits (even-y and
//!   odd-y) that are exchanged by the mirror `x = 1`.
//!
//! It is pmg, not pmm, because there is no horizontal mirror:
//!
//! * the horizontal mirror `y = 1`, `(x,y) ↦ (x, 2−y)`, sends `P` to
//!   `F(P) = R0 + (−1,0)` — a row-1 tile shape at the WRONG x-position
//!   (`(−1,0) ∉ Λ`, half a tiling period off);
//! * the horizontal mirror `y = 0` likewise lands `P` between tiles;
//! * the horizontal *glide* with glide `1/2`, `(x + 1/2, −y)`, has the
//!   right x-displacement parity for some vertices but sends the slant
//!   endpoint `(0,3/10)` to `(1/2, −3/10)`, which is not a vertex.
//!
//! The `(1,0)` row shift is exactly what kills the horizontal mirrors:
//! without it (row 1 = `F(row 0)`), the pattern would have the
//! horizontal mirror `y = 1` and be pmm. The exact probe confirms no
//! other isometry survives: the true translation subgroup is exactly
//! `Λ` (the identity coset is the only positive translation coset),
//! and the 90°/270° rotations and diagonal/anti-diagonal mirrors and
//! glides (whose linear parts normalize this square-metric lattice)
//! are negative in every one of their 16 cosets — no p4/p4m/p4g/cmm
//! upgrade.
//!
//! # Bravais kind
//!
//! **Rectangular** (primitive): the declared basis `(2,0), (0,2)` is an
//! orthogonal pair, and its equal lengths make the lattice square-
//! METRIC. That is a legal rectangular Bravais realization under a
//! ≤2-fold point group (the pattern has no 4-fold, so the square
//! metric of the translation lattice is not forced by the symmetry) —
//! the lattice-legality test's degenerate-realization exception
//! (running-bond precedent) admits a declared Rectangular lattice whose
//! basis measures Square.
//!
//! # Seeds and protos
//!
//! `proto` = cell index (`0` = `P`, `1` = `mP`, `2` = `P'`, `3` =
//! `mP'`, `4` = `R0`, `5` = `R1`, `6` = `R2`, `7` = `R3`); `seed` =
//! shape class — two classes (0 = the 6/10 trapezoid
//! `P, mP, R0, R2`, 1 = the 4/10 trapezoid `P', mP', R1, R3`), so the
//! Pattern coloring distinguishes the two trapezoid shapes.
//!
//! # Verification
//!
//! Verified exactly before porting (Python, stdlib,
//! `m81_verify.py`): the eight cell polygons are convex CCW with
//! doubled areas 120, 120, 80, 80, 120, 80, 120, 80 summing to 800 =
//! 2 · 10² · det Λ (the doubled-area factor 2 times the ×10 scale
//! times det Λ = 4); exactly two congruence classes (the 6/10 class with
//! squared edges {25, 49, 100, 104} and the 4/10 class with
//! {9, 25, 100, 104}); the window `W = [−4,4]²` (×10, `[−40,40]²`)
//! partition (188 interior edges paired exactly twice, no T-junctions
//! among 448 vertices, clipped coverage 6400 = 6400, 0 overlaps among
//! 180 clipped tiles); every one of the 105 interior vertex figures
//! sums to 360° (the two-star census above); and the full probe
//! battery — 48 positive candidates (25 lattice translations, 5
//! vertical mirrors `x = k`, `k = −2..2`, 9 glide samples
//! `(x + 1 + 2i, −y + 2j)`, 9 half-turns about `(m + 1/2, n)`) against
//! the complete 128-coset enumeration of all eight D4 linear parts
//! that normalize Λ (16 cosets each: exactly the identity, the mirror
//! `x = k` coset, the glide coset `(1/2, 0)`, and the half-turn coset
//! `(1/2, 0)` are positive — 4 pos / 124 neg) plus the named negatives
//! ((1,0), (0,1), (1,1), (1/2,0) translations; horizontal mirrors at
//! `y = 0, 1/2`; the glide-1/2 `(x + 1/2, −y)`; vertical mirrors
//! `x = 1/2`; vertical glides; off-set half-turn centers about
//! `(0,0), (1/2,1/2), (0,1/2), (1,1/2)`) — all matching the pmg
//! expectation. Rust: the standard trio, the brute-force culling
//! cross-check (`lattice_culling_matches_brute_force`), the in-module
//! pmg group probe
//! (`shifted_trapezoid_courses_wallpaper_group_is_pmg`), and the
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

/// The 6/10 trapezoid (row 0, upper course, left half) translated to
/// `(x, y)`: `(x, y+3/10), (x+1, y+1/2), (x+1, y+1), (x, y+1)`, CCW —
/// slant rising to the right.
fn trapezoid(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x, y + 0.3),
        Vec2::new(x + 1.0, y + 0.5),
        Vec2::new(x + 1.0, y + 1.0),
        Vec2::new(x, y + 1.0),
    ])
}

/// Mirror of [`trapezoid`] in the vertical line `x + 1` (row 0, upper
/// course, right half): `(x, y+1/2), (x+1, y+3/10), (x+1, y+1),
/// (x, y+1)`, CCW — slant falling to the right.
fn trapezoid_mirrored(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x, y + 0.5),
        Vec2::new(x + 1.0, y + 0.3),
        Vec2::new(x + 1.0, y + 1.0),
        Vec2::new(x, y + 1.0),
    ])
}

/// The 4/10 trapezoid (row 0, lower course, left half) translated to
/// `(x, y)`: `(x, y), (x+1, y), (x+1, y+1/2), (x, y+3/10)`, CCW.
fn trapezoid_complement(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x, y),
        Vec2::new(x + 1.0, y),
        Vec2::new(x + 1.0, y + 0.5),
        Vec2::new(x, y + 0.3),
    ])
}

/// Mirror of [`trapezoid_complement`] in the vertical line `x + 1`
/// (row 0, lower course, right half): `(x, y), (x+1, y),
/// (x+1, y+3/10), (x, y+1/2)`, CCW.
fn trapezoid_complement_mirrored(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x, y),
        Vec2::new(x + 1.0, y),
        Vec2::new(x + 1.0, y + 0.3),
        Vec2::new(x, y + 0.5),
    ])
}

/// Row 1, the flipped-and-shifted image of [`trapezoid`]:
/// `F(trapezoid(x,y)) + (1,0)` = `(x+1, y+1), (x+2, y+1),
/// (x+2, y+15/10), (x+1, y+17/10)`, CCW — `R0` at `(0,0)`.
fn trapezoid_flip(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x + 1.0, y + 1.0),
        Vec2::new(x + 2.0, y + 1.0),
        Vec2::new(x + 2.0, y + 1.5),
        Vec2::new(x + 1.0, y + 1.7),
    ])
}

/// Row 1, the flipped-and-shifted image of [`trapezoid_complement`]:
/// `F(trapezoid_complement(x,y)) + (1,0)` = `(x+1, y+17/10),
/// (x+2, y+15/10), (x+2, y+2), (x+1, y+2)`, CCW — `R1` at `(0,0)`.
fn trapezoid_complement_flip(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x + 1.0, y + 1.7),
        Vec2::new(x + 2.0, y + 1.5),
        Vec2::new(x + 2.0, y + 2.0),
        Vec2::new(x + 1.0, y + 2.0),
    ])
}

/// Row 1, the flipped-and-shifted image of [`trapezoid_mirrored`]
/// (mirror in the vertical line `x + 2` of [`trapezoid_flip`]):
/// `(x+2, y+1), (x+3, y+1), (x+3, y+17/10), (x+2, y+15/10)`, CCW —
/// `R2` at `(0,0)`.
fn trapezoid_mirrored_flip(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x + 2.0, y + 1.0),
        Vec2::new(x + 3.0, y + 1.0),
        Vec2::new(x + 3.0, y + 1.7),
        Vec2::new(x + 2.0, y + 1.5),
    ])
}

/// Row 1, the flipped-and-shifted image of
/// [`trapezoid_complement_mirrored`] (mirror in the vertical line
/// `x + 2` of [`trapezoid_complement_flip`]): `(x+2, y+15/10),
/// (x+3, y+17/10), (x+3, y+2), (x+2, y+2)`, CCW — `R3` at `(0,0)`.
fn trapezoid_complement_mirrored_flip(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x + 2.0, y + 1.5),
        Vec2::new(x + 3.0, y + 1.7),
        Vec2::new(x + 3.0, y + 2.0),
        Vec2::new(x + 2.0, y + 2.0),
    ])
}

/// Shifted trapezoid courses — the trapezoid-course row stacked in
/// rows of height 1, alternate rows flipped in `y = 1` and shifted by
/// `(1,0)`, per 2×2 cell on the rectangular lattice ⟨(2,0),(0,2)⟩
/// (pmg).
pub fn shifted_trapezoid_courses() -> LatticeTiling {
    // Row 0 (y in [0,1]) is the M79 trapezoid-course cell: P, mP
    // (areas 6/10, horizontal tops at y = 1) and P', mP' (areas 4/10,
    // horizontal bottoms at y = 0). Row 1 (y in [1,2]) is the vertical
    // flip F(x, y) = (x, 2-y) of row 0 shifted by (1,0): R0, R1 under
    // P, P' and R2, R3 under mP, mP'. The (1,0) shift — half the
    // tiling's x-period — is what kills the horizontal mirrors and
    // leaves the group pmg (glide 1, axes y = integer) instead of pmm.
    // The right-half row-1 tiles (R2, R3) straddle the cell's right
    // edge and are the row-1 tiles of the cell at (i−1, j) under the
    // lattice.
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
        Cell {
            poly: trapezoid_flip(0.0, 0.0),
            seed: 0,
        },
        Cell {
            poly: trapezoid_complement_flip(0.0, 0.0),
            seed: 1,
        },
        Cell {
            poly: trapezoid_mirrored_flip(0.0, 0.0),
            seed: 0,
        },
        Cell {
            poly: trapezoid_complement_mirrored_flip(0.0, 0.0),
            seed: 1,
        },
    ];

    LatticeTiling {
        id: "shifted-trapezoid-courses",
        name: "Shifted trapezoid courses",
        description: "The M79 trapezoid-course row stacked in rows of height 1 per 2x2 rectangular cell <(2,0),(0,2)> (det 4), alternate rows reflected in y = 1 and shifted by (1,0): eight trapezoids, two incongruent shapes (areas 3/5 and 2/5) times mirror times row-flip. Edge-to-edge (slant slope 1/5). The catalog's first pmg realizer: vertical mirror lines x = integer, one horizontal glide family (glide 1, axes y = integer), half-turns at (m+1/2, n) for all integers m, n; the (1,0) row shift kills the horizontal mirrors that would make it pmm.",
        lattice: [Vec2::new(2.0, 0.0), Vec2::new(0.0, 2.0)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::Pmg,
        lattice_kind: LatticeKind::Rectangular,
        category: TilingCategory::Wallpaper,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
