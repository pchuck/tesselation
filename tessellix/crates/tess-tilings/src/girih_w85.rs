//! Girih 12-pointed star interlace — Wichmann panel `data18/W85` (the
//! oldest dated star pattern: Alhambra, Mexuar patio; after the SPA 1914
//! drawing of Wade; cf. also Lee's Kharraqan 1067–8 attribution and Bonner's
//! Uljaytu panel, the Topkapı scroll — reproduced from the vector source as
//! a [`LatticeTiling`] table.
//!
//! # Provenance
//!
//! O. Glezer / R. Wichmann, *Islamic Tiling Patterns* database, panel
//! `data18/W85` (`tilingsearch.mit.edu/HTML/data18/W85.html`). Wichmann
//! classifies the **motif** (ground faces + unbroken strap bands) as
//! *632 = p6m — correct for the un-interlaced pattern. The panel is drawn
//! interlaced (strap families cross with over/under breaks: one finite and
//! one infinite strap class), and the over/under assignment is chiral, so
//! the symmetry group of the drawn face-to-face tiling (the 42 polygons
//! emitted here) is **p6**, not p6m. Both facts are verified exactly
//! (below); p6 is what is shipped.
//!
//! # Construction (exact, over Q(√3))
//!
//! * Lattice `a1 = (L, 0)`, `a2 = (L/2, L√3/2)` with `L = 283.92` and strap
//!   width `W = 9.0` (as drawn by Wichmann; world units = source PDF units
//!   / 10; a least-squares refit of the vector source gives L = 283.922,
//!   W = 8.999 at lattice angle 15.000° — the drawing quantises exactly to
//!   these values).
//! * Strap centrelines: six band families of direction 30°·k; for
//!   k ∈ {1, 3, 5} at signed offsets L/4 + m·L/2 across the band, for
//!   k ∈ {0, 2, 4} at ±L/4 + m·L√3/2. Band edges at ±W/2 about each
//!   centreline; all crossings at 30°/60°/90°.
//! * A 12-pointed star with regular vertices sits at every lattice point;
//!   its edge lines are the band edges at perpendicular distance
//!   `d = L/4 − W/2 = 66.48`: tip radius d√2 ≈ 94.0169, notch radius
//!   2d/√3 ≈ 76.7645, edge d(1 − 1/√3) ≈ 28.0977. All tile angles are
//!   multiples of 30° and every joint is edge-to-edge.
//! * Consequently every vertex of the pattern lies on the exact grid
//!   `(p + q√3)/50`, integers — the table below is the exact model, not a
//!   tracing of the (0.01-unit quantised) drawing.
//!
//! # Unit cell (42 tiles, seeds = tile classes)
//!
//! star ×1 (24-gon, seed 0) · kite ×12 (60/90/120/90, seed 1) · pentagon ×6
//! (90/120/90/120/120, seed 2) · triangle ×2 (seed 3) · rectangle ×3
//! (W × 94.9, seed 4) · sheared quad ×6 (seed 5) · reflex ink hexagon ×12
//! (strap pieces with 240° joints, seed 6).
//!
//! # Verification (Python spike before porting)
//!
//! * Σ of the 42 exact cell areas == |det(a1, a2)| = 40305.2832·√3 with
//!   **zero** error in Fraction-Q(√3) arithmetic;
//! * all 344 usable drawn polygons of the vector source snap to this grid
//!   within 0.08 world units (= drawing quantisation); unit-cell coverage
//!   is exactly-once on a 150×150 grid and every joint passes the full
//!   side-by-side edge test;
//! * symmetry tested exactly over Q(√3) on the infinite lattice union:
//!   rot60 at every lattice point (6-fold star centres), rot180 at the
//!   midpoints of all three lattice edge classes, rot120 at (a1+a2)/3 and
//!   2(a1+a2)/3 — all hold; 30° rotations and every mirror axis fail (the
//!   ground faces alone are p6m: 167/167 mirror matches, but the drawn ink
//!   pieces never mirror-match: 0/177). Wallpaper group **p6**.
use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// √3 — every coordinate in this module is a Q(√3) surd built from it
/// (computed at runtime; no decimal vertex approximations are shipped).
pub(crate) fn s3() -> f64 {
    3.0f64.sqrt()
}

/// Lattice period L (as drawn by Wichmann, world units = PDF units/10).
pub const L: f64 = 283.92;
/// Strap width W.
pub const STRAP_W: f64 = 9.0;

/// Cell constructor: each vertex is `(p + q√3)/50` (integer grid values
/// verified exact for the whole panel).
fn cell(seed: u32, g: &[[f64; 4]]) -> Cell {
    let s = s3();
    Cell {
        seed,
        poly: Polygon::from_slice(
            &g.iter()
                .map(|q| Vec2::new((q[0] + q[1] * s) / 50.0, (q[2] + q[3] * s) / 50.0))
                .collect::<Vec<_>>(),
        ),
    }
}

/// Girih 12-pointed star interlace (Wichmann data18/W85, Alhambra
/// Mexuar): 42 tiles per hexagonal cell, wallpaper group p6 (the drawn
/// over/under interlace is chiral; Wichmann's *632 is the motif alone).
pub fn girih_w85() -> LatticeTiling {
    let s3 = s3();
    let cells = vec![
        // star
        cell(0, &[
            [-1662.0, -1662.0, -1662.0, 1662.0],
            [0.0, -2216.0, 0.0, 0.0],
            [-1662.0, -1662.0, 1662.0, -1662.0],
            [-3324.0, 0.0, 0.0, -1108.0],
            [-3324.0, 0.0, -3324.0, 0.0],
            [0.0, -1108.0, -3324.0, 0.0],
            [1662.0, -1662.0, -1662.0, -1662.0],
            [0.0, 0.0, 0.0, -2216.0],
            [-1662.0, 1662.0, -1662.0, -1662.0],
            [0.0, 1108.0, -3324.0, 0.0],
            [3324.0, 0.0, -3324.0, 0.0],
            [3324.0, 0.0, 0.0, -1108.0],
            [1662.0, 1662.0, 1662.0, -1662.0],
            [0.0, 2216.0, 0.0, 0.0],
            [1662.0, 1662.0, -1662.0, 1662.0],
            [3324.0, 0.0, 0.0, 1108.0],
            [3324.0, 0.0, 3324.0, 0.0],
            [0.0, 1108.0, 3324.0, 0.0],
            [-1662.0, 1662.0, 1662.0, 1662.0],
            [0.0, 0.0, 0.0, 2216.0],
            [1662.0, -1662.0, 1662.0, 1662.0],
            [0.0, -1108.0, 3324.0, 0.0],
            [-3324.0, 0.0, 3324.0, 0.0],
            [-3324.0, 0.0, 0.0, 1108.0],
        ]),
        // kite
        cell(1, &[
            [-1662.0, -1887.0, 1887.0, -1662.0],
            [0.0, -2516.0, 0.0, 0.0],
            [-1662.0, -1887.0, -1887.0, 1662.0],
            [-6648.0, 0.0, 0.0, 0.0],
        ]),
        // kite
        cell(1, &[
            [0.0, -3324.0, 3324.0, 0.0],
            [-1887.0, -1662.0, -1662.0, 1887.0],
            [-3774.0, 0.0, 0.0, 1258.0],
            [-3774.0, 0.0, 3324.0, 0.0],
        ]),
        // kite
        cell(1, &[
            [-3324.0, 0.0, -3774.0, 0.0],
            [-3324.0, 0.0, 0.0, -3324.0],
            [1662.0, -1887.0, -1887.0, -1662.0],
            [0.0, -1258.0, -3774.0, 0.0],
        ]),
        // kite
        cell(1, &[
            [0.0, 0.0, 0.0, 2516.0],
            [-1887.0, 1662.0, 1662.0, 1887.0],
            [0.0, 0.0, 6648.0, 0.0],
            [1887.0, -1662.0, 1662.0, 1887.0],
        ]),
        // kite
        cell(1, &[
            [3774.0, 0.0, 0.0, -1258.0],
            [3774.0, 0.0, -3324.0, 0.0],
            [0.0, 3324.0, -3324.0, 0.0],
            [1887.0, 1662.0, 1662.0, -1887.0],
        ]),
        // kite
        cell(1, &[
            [-3774.0, 0.0, -3324.0, 0.0],
            [-3774.0, 0.0, 0.0, -1258.0],
            [-1887.0, -1662.0, 1662.0, -1887.0],
            [0.0, -3324.0, -3324.0, 0.0],
        ]),
        // kite
        cell(1, &[
            [-3324.0, 0.0, 0.0, 3324.0],
            [-3324.0, 0.0, 3774.0, 0.0],
            [0.0, -1258.0, 3774.0, 0.0],
            [1662.0, -1887.0, 1887.0, 1662.0],
        ]),
        // kite
        cell(1, &[
            [3324.0, 0.0, 0.0, -3324.0],
            [3324.0, 0.0, -3774.0, 0.0],
            [0.0, 1258.0, -3774.0, 0.0],
            [-1662.0, 1887.0, -1887.0, -1662.0],
        ]),
        // kite
        cell(1, &[
            [3774.0, 0.0, 3324.0, 0.0],
            [3774.0, 0.0, 0.0, 1258.0],
            [1887.0, 1662.0, -1662.0, 1887.0],
            [0.0, 3324.0, 3324.0, 0.0],
        ]),
        // kite
        cell(1, &[
            [0.0, 0.0, -6648.0, 0.0],
            [-1887.0, 1662.0, -1662.0, -1887.0],
            [0.0, 0.0, 0.0, -2516.0],
            [1887.0, -1662.0, -1662.0, -1887.0],
        ]),
        // kite
        cell(1, &[
            [-1662.0, 1887.0, 1887.0, 1662.0],
            [0.0, 1258.0, 3774.0, 0.0],
            [3324.0, 0.0, 3774.0, 0.0],
            [3324.0, 0.0, 0.0, 3324.0],
        ]),
        // kite
        cell(1, &[
            [1662.0, 1887.0, 1887.0, -1662.0],
            [6648.0, 0.0, 0.0, 0.0],
            [1662.0, 1887.0, -1887.0, 1662.0],
            [0.0, 2516.0, 0.0, 0.0],
        ]),
        // pent
        cell(2, &[
            [-12309.0, 1887.0, 1887.0, -1887.0],
            [-14196.0, 3624.0, -3324.0, 0.0],
            [0.0, -3624.0, -3324.0, 0.0],
            [-1887.0, -1887.0, 1887.0, -1887.0],
            [-7098.0, 0.0, 0.0, -150.0],
        ]),
        // pent
        cell(2, &[
            [0.0, -3474.0, 3774.0, 0.0],
            [-3774.0, 0.0, 3774.0, 0.0],
            [-3774.0, 0.0, 0.0, 3474.0],
            [-8985.0, 1887.0, -1887.0, 5211.0],
            [-7098.0, 150.0, -7098.0, 7098.0],
        ]),
        // pent
        cell(2, &[
            [-3324.0, 0.0, -3774.0, 7098.0],
            [-3324.0, 0.0, 0.0, 3624.0],
            [1887.0, -1887.0, 1887.0, 1887.0],
            [0.0, -150.0, 7098.0, 0.0],
            [-7098.0, 3474.0, -3774.0, 7098.0],
        ]),
        // pent
        cell(2, &[
            [-3324.0, 0.0, 0.0, -3624.0],
            [-3324.0, 0.0, 3774.0, -7098.0],
            [-7098.0, 3474.0, 3774.0, -7098.0],
            [0.0, -150.0, -7098.0, 0.0],
            [1887.0, -1887.0, -1887.0, -1887.0],
        ]),
        // pent
        cell(2, &[
            [-8985.0, 1887.0, 1887.0, -5211.0],
            [-3774.0, 0.0, 0.0, -3474.0],
            [-3774.0, 0.0, -3774.0, 0.0],
            [0.0, -3474.0, -3774.0, 0.0],
            [-7098.0, 150.0, 7098.0, -7098.0],
        ]),
        // pent
        cell(2, &[
            [-7098.0, 0.0, 0.0, 150.0],
            [-1887.0, -1887.0, -1887.0, 1887.0],
            [0.0, -3624.0, 3324.0, 0.0],
            [-14196.0, 3624.0, 3324.0, 0.0],
            [-12309.0, 1887.0, -1887.0, 1887.0],
        ]),
        // tri
        cell(3, &[
            [-7098.0, 3774.0, -3774.0, 7098.0],
            [0.0, 0.0, 7548.0, 0.0],
            [7098.0, -3774.0, -3774.0, 7098.0],
        ]),
        // tri
        cell(3, &[
            [7098.0, -3774.0, 3774.0, -7098.0],
            [0.0, 0.0, -7548.0, 0.0],
            [-7098.0, 3774.0, 3774.0, -7098.0],
        ]),
        // rect
        cell(4, &[
            [12309.0, -1887.0, 1887.0, -1887.0],
            [12534.0, -1887.0, 1887.0, -1662.0],
            [1887.0, 1887.0, -1887.0, 1887.0],
            [1662.0, 1887.0, -1887.0, 1662.0],
        ]),
        // rect
        cell(4, &[
            [-8985.0, 1887.0, 1887.0, -5211.0],
            [-8760.0, 1887.0, 1887.0, -5436.0],
            [1887.0, -1887.0, -1887.0, -1887.0],
            [1662.0, -1887.0, -1887.0, -1662.0],
        ]),
        // rect
        cell(4, &[
            [3324.0, 0.0, 3774.0, -7098.0],
            [3774.0, 0.0, 3774.0, -7098.0],
            [3774.0, 0.0, -3774.0, 0.0],
            [3324.0, 0.0, -3774.0, 0.0],
        ]),
        // quad
        cell(5, &[
            [-14196.0, 3624.0, -3324.0, 0.0],
            [-14196.0, 3774.0, -3774.0, 0.0],
            [-3774.0, 0.0, -3774.0, 0.0],
            [-3774.0, 0.0, -3324.0, 0.0],
        ]),
        // quad
        cell(5, &[
            [1887.0, -1887.0, 1887.0, 1887.0],
            [1887.0, -1662.0, 1662.0, 1887.0],
            [7098.0, -3474.0, -3774.0, 7098.0],
            [7098.0, -3774.0, -3774.0, 7098.0],
        ]),
        // quad
        cell(5, &[
            [7098.0, -150.0, 7098.0, -7098.0],
            [7098.0, 0.0, 7548.0, -7098.0],
            [1887.0, 1887.0, 1887.0, -1887.0],
            [1887.0, 1662.0, 1662.0, -1887.0],
        ]),
        // quad
        cell(5, &[
            [3774.0, 0.0, 3774.0, 0.0],
            [3774.0, 0.0, 3324.0, 0.0],
            [14196.0, -3624.0, 3324.0, 0.0],
            [14196.0, -3774.0, 3774.0, 0.0],
        ]),
        // quad
        cell(5, &[
            [-7098.0, 3774.0, 3774.0, -7098.0],
            [-1887.0, 1887.0, -1887.0, -1887.0],
            [-1887.0, 1662.0, -1662.0, -1887.0],
            [-7098.0, 3474.0, 3774.0, -7098.0],
        ]),
        // quad
        cell(5, &[
            [-1887.0, -1887.0, -1887.0, 1887.0],
            [-1887.0, -1662.0, -1662.0, 1887.0],
            [-7098.0, 150.0, -7098.0, 7098.0],
            [-7098.0, 0.0, -7548.0, 7098.0],
        ]),
        // hexink
        cell(6, &[
            [-3324.0, 0.0, 0.0, 3624.0],
            [-3324.0, 0.0, 0.0, 3324.0],
            [0.0, 0.0, 0.0, 2216.0],
            [-1662.0, 1662.0, 1662.0, 1662.0],
            [-1887.0, 1662.0, 1662.0, 1887.0],
            [0.0, 0.0, 0.0, 2516.0],
        ]),
        // hexink
        cell(6, &[
            [-1662.0, 1887.0, -1887.0, -1662.0],
            [0.0, 1258.0, -3774.0, 0.0],
            [0.0, 3474.0, -3774.0, 0.0],
            [0.0, 3324.0, -3324.0, 0.0],
            [0.0, 1108.0, -3324.0, 0.0],
            [-1662.0, 1662.0, -1662.0, -1662.0],
        ]),
        // hexink
        cell(6, &[
            [-7098.0, 0.0, 0.0, 150.0],
            [-6648.0, 0.0, 0.0, 0.0],
            [-3324.0, 0.0, 0.0, 1108.0],
            [-3324.0, 0.0, 3324.0, 0.0],
            [-3774.0, 0.0, 3324.0, 0.0],
            [-3774.0, 0.0, 0.0, 1258.0],
        ]),
        // hexink
        cell(6, &[
            [0.0, -3474.0, 3774.0, 0.0],
            [0.0, -3324.0, 3324.0, 0.0],
            [0.0, -1108.0, 3324.0, 0.0],
            [1662.0, -1662.0, 1662.0, 1662.0],
            [1662.0, -1887.0, 1887.0, 1662.0],
            [0.0, -1258.0, 3774.0, 0.0],
        ]),
        // hexink
        cell(6, &[
            [3324.0, 0.0, 0.0, -1108.0],
            [3324.0, 0.0, -3324.0, 0.0],
            [3774.0, 0.0, -3324.0, 0.0],
            [3774.0, 0.0, 0.0, -1258.0],
            [7098.0, 0.0, 0.0, -150.0],
            [6648.0, 0.0, 0.0, 0.0],
        ]),
        // hexink
        cell(6, &[
            [-3324.0, 0.0, -3324.0, 0.0],
            [-3324.0, 0.0, -3774.0, 0.0],
            [0.0, -1258.0, -3774.0, 0.0],
            [0.0, -150.0, -7098.0, 0.0],
            [0.0, 0.0, -6648.0, 0.0],
            [0.0, -1108.0, -3324.0, 0.0],
        ]),
        // hexink
        cell(6, &[
            [3324.0, 0.0, 0.0, 1108.0],
            [1662.0, 1662.0, -1662.0, 1662.0],
            [1887.0, 1662.0, -1662.0, 1887.0],
            [3774.0, 0.0, 0.0, 1258.0],
            [3774.0, 0.0, 0.0, 3474.0],
            [3324.0, 0.0, 0.0, 3324.0],
        ]),
        // hexink
        cell(6, &[
            [0.0, 1108.0, 3324.0, 0.0],
            [3324.0, 0.0, 3324.0, 0.0],
            [3324.0, 0.0, 3774.0, 0.0],
            [0.0, 1258.0, 3774.0, 0.0],
            [0.0, 150.0, 7098.0, 0.0],
            [0.0, 0.0, 6648.0, 0.0],
        ]),
        // hexink
        cell(6, &[
            [0.0, -3624.0, -3324.0, 0.0],
            [0.0, -3324.0, -3324.0, 0.0],
            [0.0, -2216.0, 0.0, 0.0],
            [-1662.0, -1662.0, -1662.0, 1662.0],
            [-1662.0, -1887.0, -1887.0, 1662.0],
            [0.0, -2516.0, 0.0, 0.0],
        ]),
        // hexink
        cell(6, &[
            [-3774.0, 0.0, 0.0, -1258.0],
            [-3774.0, 0.0, 0.0, -3474.0],
            [-3324.0, 0.0, 0.0, -3324.0],
            [-3324.0, 0.0, 0.0, -1108.0],
            [-1662.0, -1662.0, 1662.0, -1662.0],
            [-1887.0, -1662.0, 1662.0, -1887.0],
        ]),
        // hexink
        cell(6, &[
            [1662.0, -1662.0, -1662.0, -1662.0],
            [1887.0, -1662.0, -1662.0, -1887.0],
            [0.0, 0.0, 0.0, -2516.0],
            [3324.0, 0.0, 0.0, -3624.0],
            [3324.0, 0.0, 0.0, -3324.0],
            [0.0, 0.0, 0.0, -2216.0],
        ]),
        // hexink
        cell(6, &[
            [1662.0, 1662.0, 1662.0, -1662.0],
            [1662.0, 1887.0, 1887.0, -1662.0],
            [0.0, 2516.0, 0.0, 0.0],
            [0.0, 3624.0, 3324.0, 0.0],
            [0.0, 3324.0, 3324.0, 0.0],
            [0.0, 2216.0, 0.0, 0.0],
        ]),
    ];
    LatticeTiling {
        id: "girih-w85",
        name: "Girih 12-pointed star interlace (W85)",
        description: "Wichmann panel data18/W85 (Alhambra, Mexuar patio): 12-pointed girih star interlace on the triangular lattice L = 283.92, strap width 9, exact over Q(√3); 42 tiles per cell; wallpaper group p6 (the drawn over/under interlace is chiral; the motif alone is *632).",
        lattice: [Vec2::new(L, 0.0), Vec2::new(L / 2.0, (L / 2.0) * s3)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Wallpaper,
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
