//! Split-triangle grid (M84) — the first **p31m** (3*3) wallpaper-group
//! realizer in the catalog, expressed as a [`LatticeTiling`] data table.
//!
//! # The pattern
//!
//! Start with the side-2 triangular grid on the translation lattice
//! `Λ = ⟨(2,0), (1,√3)⟩` (det `2√3`). One primitive cell holds two
//! side-2 equilateral triangles of area `√3`: the up-triangle
//! `U = (0,0), (2,0), (1,√3)` and the down-triangle
//! `D = (1,√3), (3,√3), (2,0)`. Each small triangle is split by its
//! **orbit triangle**: a generic `Q(√3)` point
//! `P0 = (3/5, −1/40 + 2√3/15)` near a vertex of `U`, rotated 120°
//! about the centroid of `U` to `P1, P2`. The triangle `(P0, P1, P2)`
//! cuts `U` into an inner equilateral triangle plus three corner
//! quads, one per edge of `U`; `D` receives the mirror image of the
//! same construction (reflected in `y = 0`, shifted by `a2`). The
//! generic `Q(√3)` seed `P0` (on no 30° line) is what breaks the
//! remaining 6-fold and 2-fold structure of the plain grid down to
//! exactly p31m.
//!
//! The cell has eight tiles: two inner equilateral triangles (one
//! congruence class, achiral) and six corner quads (one congruence
//! class, CHIRAL: the mirror image is not directly congruent).
//!
//! # Edge-to-edge note
//!
//! The partition is edge-to-edge: every tile edge is shared by exactly
//! two tiles, with no T-junctions. The battery window check (interior
//! edges all paired exactly twice) confirms 750 interior edges,
//! 0 singles, 0 bad.
//!
//! # Vertex stars
//!
//! Two star types, all summing to 360°:
//!
//! * the grid vertices (lattice points of the triangular grid): TWELVE
//!   corner quads meet (two per each of the six adjacent small
//!   triangles) with angles `≈18.9442° × 6` + `≈41.0558° × 6`;
//! * the inner-triangle vertices `P0, P1, P2` (and their mirror
//!   images): three tiles meet (inner triangle + two corner quads of
//!   the same small triangle) with angles `60°, ≈126.0665°,
//!   ≈173.9335°`.
//!
//! # Wallpaper group
//!
//! **p31m** (3*3) — translations, 120°/240° rotations, and the three
//! mirror families on the grid lines (directions 0°, 60°, 120°), and
//! nothing else: no 60°/180°/300° rotations (so not p6 or p6m), no
//! 30°/90°/150° mirror families, no glides. The rot120 centres fall
//! into three orbits under the lattice: the grid vertices, the
//! up-triangle centroids, and the down-triangle centroids.
//!
//! # p31m vs p3m1 — the incidence discriminator
//!
//! The two hexagonal mirror groups differ only in which rot120 centres
//! lie on mirrors. Here exactly ONE of the three centre orbits — the
//! grid-vertex orbit — lies on a mirror line; both centroid orbits sit
//! off every mirror. In p3m1 all three orbits lie on mirrors. This is
//! verified exactly by the Rust probe
//! (`split_triangle_grid_wallpaper_group_is_p31m`): the on-mirror
//! incidence of the three orbit representatives is
//! `[true, false, false]`, alongside the p3m1-excluding negatives
//! (rot60/rot180/rot300, refl30/90/150 — all zero positive cosets).
//!
//! # Bravais kind
//!
//! **Hexagonal** (primitive): `Λ = ⟨(2,0), (1,√3)⟩`,
//! `det Λ = 2√3` — the side-2 triangular grid's own lattice, so the
//! cell is one up- plus one down-triangle (eight tiles).
//!
//! # Seeds and protos
//!
//! Two congruence classes per cell: the inner equilateral triangles
//! carry `seed = 0` (achiral), the corner quads carry `seed = 1`
//! (chiral) — Pattern coloring distinguishes the two classes.
//!
//! # Verification
//!
//! Coords are machine-emitted (`m84_emit.py` → `m84_emit_f64.inc` /
//! `m84_emit_rust.inc`, parsed back to the HX-STUDY cell JSON exactly,
//! no hand-typed coordinates). The full HX-STUDY battery was
//! reproduced before porting (`m84_verify.py`): the exact-Fraction
//! battery matches the JSON on every number (area sum `2√3 = det`,
//! e2e 750/0/0, stars 186/0, coverage 4096/0/0, positives only at the
//! zero coset of rot120/rot240/refl0/refl60/refl120, translations
//! 25/25, five off-lattice negatives 0/5, three rot120 centre orbits
//! with on-mirror incidence 1), AND the exact-integer ×240 probe that
//! the Rust test below mirrors (lattice `a1×240 = (480,0,0,0)`,
//! `a2×240 = (240,0,0,240)`; the 11 integer `2L` matrices shared with
//! the p3 probe; floor coset reduction). Rust: the standard trio, the
//! exact-integer p31m group probe
//! (`split_triangle_grid_wallpaper_group_is_p31m`), and the brute-force
//! culling cross-check (`lattice_culling_matches_brute_force`).

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// √3 — the y-coordinate scale of the lattice vector `a2 = (1, √3)`.
const SURD3: f64 = 1.7320508075688773;

fn poly(pts: &[Vec2]) -> Polygon {
    Polygon::from_slice(pts)
}

/// Tile 0 of the cell: the inner equilateral triangle of the
/// up-triangle `U`, `[P0, P1, P2]`, CCW (achiral, seed 0).
fn piece0() -> Polygon {
    poly(&[
        Vec2::new(3.0 / 5.0, -1.0 / 40.0 + 2.0 / 15.0 * SURD3),
        Vec2::new(3.0 / 2.0 + 1.0 / 80.0 * SURD3, 1.0 / 80.0 + 7.0 / 30.0 * SURD3),
        Vec2::new(9.0 / 10.0 - 1.0 / 80.0 * SURD3, 1.0 / 80.0 + 19.0 / 30.0 * SURD3),
    ])
}

/// Tile 1: the corner quad of `U` at edge `V0-V1`, `[V0, V1, P1, P0]`,
/// CCW (chiral class, seed 1).
fn piece1() -> Polygon {
    poly(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(2.0, 0.0),
        Vec2::new(3.0 / 2.0 + 1.0 / 80.0 * SURD3, 1.0 / 80.0 + 7.0 / 30.0 * SURD3),
        Vec2::new(3.0 / 5.0, -1.0 / 40.0 + 2.0 / 15.0 * SURD3),
    ])
}

/// Tile 2: the corner quad of `U` at edge `V1-V2`, `[V1, V2, P2, P1]`,
/// CCW.
fn piece2() -> Polygon {
    poly(&[
        Vec2::new(2.0, 0.0),
        Vec2::new(1.0, SURD3),
        Vec2::new(9.0 / 10.0 - 1.0 / 80.0 * SURD3, 1.0 / 80.0 + 19.0 / 30.0 * SURD3),
        Vec2::new(3.0 / 2.0 + 1.0 / 80.0 * SURD3, 1.0 / 80.0 + 7.0 / 30.0 * SURD3),
    ])
}

/// Tile 3: the corner quad of `U` at edge `V2-V0`, `[V2, V0, P0, P2]`,
/// CCW.
fn piece3() -> Polygon {
    poly(&[
        Vec2::new(1.0, SURD3),
        Vec2::new(0.0, 0.0),
        Vec2::new(3.0 / 5.0, -1.0 / 40.0 + 2.0 / 15.0 * SURD3),
        Vec2::new(9.0 / 10.0 - 1.0 / 80.0 * SURD3, 1.0 / 80.0 + 19.0 / 30.0 * SURD3),
    ])
}

/// Tile 4: the inner equilateral triangle of the down-triangle `D`,
/// the mirror image of tile 0 (reflected in `y = 0`, shifted by
/// `a2`), CCW (achiral, seed 0).
fn piece4() -> Polygon {
    poly(&[
        Vec2::new(19.0 / 10.0 - 1.0 / 80.0 * SURD3, -1.0 / 80.0 + 11.0 / 30.0 * SURD3),
        Vec2::new(5.0 / 2.0 + 1.0 / 80.0 * SURD3, -1.0 / 80.0 + 23.0 / 30.0 * SURD3),
        Vec2::new(8.0 / 5.0, 1.0 / 40.0 + 13.0 / 15.0 * SURD3),
    ])
}

/// Tile 5: the mirror image of tile 1 (the corner quad of `D` at the
/// mirrored edge `V0-V1`), CCW.
fn piece5() -> Polygon {
    poly(&[
        Vec2::new(8.0 / 5.0, 1.0 / 40.0 + 13.0 / 15.0 * SURD3),
        Vec2::new(5.0 / 2.0 + 1.0 / 80.0 * SURD3, -1.0 / 80.0 + 23.0 / 30.0 * SURD3),
        Vec2::new(3.0, SURD3),
        Vec2::new(1.0, SURD3),
    ])
}

/// Tile 6: the mirror image of tile 2 (the corner quad of `D` at the
/// mirrored edge `V1-V2`), CCW.
fn piece6() -> Polygon {
    poly(&[
        Vec2::new(5.0 / 2.0 + 1.0 / 80.0 * SURD3, -1.0 / 80.0 + 23.0 / 30.0 * SURD3),
        Vec2::new(19.0 / 10.0 - 1.0 / 80.0 * SURD3, -1.0 / 80.0 + 11.0 / 30.0 * SURD3),
        Vec2::new(2.0, 0.0),
        Vec2::new(3.0, SURD3),
    ])
}

/// Tile 7: the mirror image of tile 3 (the corner quad of `D` at the
/// mirrored edge `V2-V0`), CCW.
fn piece7() -> Polygon {
    poly(&[
        Vec2::new(19.0 / 10.0 - 1.0 / 80.0 * SURD3, -1.0 / 80.0 + 11.0 / 30.0 * SURD3),
        Vec2::new(8.0 / 5.0, 1.0 / 40.0 + 13.0 / 15.0 * SURD3),
        Vec2::new(1.0, SURD3),
        Vec2::new(2.0, 0.0),
    ])
}

/// Split-triangle grid — the side-2 triangular grid with every small
/// equilateral triangle split by its orbit triangle into an inner
/// equilateral triangle + three corner quads, per hexagonal cell
/// `⟨(2,0),(1,√3)⟩` (det `2√3`); eight tiles, two congruence classes
/// (the quad class chiral), group p31m.
pub fn split_triangle_grid() -> LatticeTiling {
    // Two congruence classes: the inner equilateral triangles (tiles
    // 0, 4; achiral, seed 0) and the corner quads (tiles 1,2,3,5,6,7;
    // chiral, seed 1).
    let cells = vec![
        Cell {
            poly: piece0(),
            seed: 0,
        },
        Cell {
            poly: piece1(),
            seed: 1,
        },
        Cell {
            poly: piece2(),
            seed: 1,
        },
        Cell {
            poly: piece3(),
            seed: 1,
        },
        Cell {
            poly: piece4(),
            seed: 0,
        },
        Cell {
            poly: piece5(),
            seed: 1,
        },
        Cell {
            poly: piece6(),
            seed: 1,
        },
        Cell {
            poly: piece7(),
            seed: 1,
        },
    ];

    LatticeTiling {
        id: "split-triangle-grid",
        name: "Split-triangle grid",
        description: "Side-2 triangular grid in which each small equilateral triangle is split by its orbit triangle — a generic Q(√3) point near a vertex, rotated 120° about the triangle's centroid — into an inner equilateral triangle + 3 corner quads, per hexagonal cell <(2,0),(1,√3)> (det 2√3): 8 tiles, two congruence classes (inner triangles achiral, corner quads chiral); edge-to-edge (750 interior edges, 0 T-junctions); the catalog's first p31m realizer — mirrors exactly on the grid lines (0°/60°/120° families), rot120 centres in three orbits (grid vertices, up/down-triangle centroids) with the incidence that separates p31m from p3m1 (only the grid-vertex orbit lies on mirrors), no 60°/180°/300° rotations, no glides (M84, HX-STUDY handoff).",
        lattice: [Vec2::new(2.0, 0.0), Vec2::new(1.0, SURD3)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P31m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Wallpaper,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
