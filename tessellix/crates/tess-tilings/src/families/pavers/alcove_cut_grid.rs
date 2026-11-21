//! Alcove-cut grid (M85) — the **p3m1** (*333) wallpaper-group
//! realizer in the catalog, expressed as a [`LatticeTiling`] data
//! table. M85 closes the last gap of the 17-group wallpaper catalog
//! (M78–M84 landed p1, pm, pg, pmg, cm, p3, p31m; p3m1 was the one
//! remaining group).
//!
//! # The pattern
//!
//! Start with the side-2 triangular grid on the index-3 sublattice
//! `Λ = ⟨(3,−√3), (3,√3)⟩` (det `6√3`, three times the grid's own
//! lattice `⟨(2,0), (1,√3)⟩` of det `2√3`). One primitive cell holds
//! six side-2 equilateral triangles of area `√3`: three up-pointing
//! `U0 = (0,0), (2,0), (1,√3)`, `U1 = (1,−√3), (3,−√3), (2,0)`,
//! `U2 = (2,0), (4,0), (3,√3)` and three down-pointing `D0 =
//! (0,0), (2,0), (1,−√3)`, `D1 = (2,0), (4,0), (3,−√3)`, `D2 =
//! (1,√3), (3,√3), (2,0)`.
//!
//! Each grid triangle is an **alcove**: under the mirror arrangement
//! below, every grid triangle has trivial stabilizer, so a decoration
//! of one alcove extends equivariantly to a p3m1-invariant tiling.
//! The decoration: the reflection group of the up-triangle `U0` —
//! mirrors in its three edge lines, whose closure is the full
//! 3-family triangular mirror arrangement (directions 0°/60°/120°) —
//! acts on the interior junction `P0 = vmean(U0) + (1/7, √3/9) =
//! (8/7, 4√3/9)`, producing the six alcove points `J1 … J6` (one per
//! grid triangle of the cell). Each alcove triangle is cut from its
//! alcove point to its three vertices: three tiles per grid triangle,
//! eighteen per cell.
//!
//! # Edge-to-edge note
//!
//! The partition is edge-to-edge: every tile edge is shared by
//! exactly two tiles, with no T-junctions. The battery window check
//! (interior edges all paired exactly twice) confirms 1350 interior
//! edges, 0 singles, 0 bad.
//!
//! # Vertex stars
//!
//! Four star types, all summing to 360°:
//!
//! * the alcove junctions `J1 … J6`: three tiles meet (the three
//!   cuts of one alcove triangle) with angles `≈104.110°,
//!   ≈115.519°, ≈140.372°`;
//! * the grid vertices in the `(0,0)` lattice orbit: twelve tiles
//!   meet with angles `≈26.037° × 6` + `≈33.963° × 6`;
//! * the grid vertices in the `(2,0)` orbit: twelve tiles meet with
//!   angles `≈18.073° × 6` + `≈41.927° × 6`;
//! * the grid vertices in the `(4,0)` orbit: twelve tiles meet with
//!   angles `≈21.555° × 6` + `≈38.444° × 6`.
//!
//! # Wallpaper group
//!
//! **p3m1** (*333) — translations, 120°/240° rotations, and the
//! three mirror families at directions 0°, 60°, 120°, and nothing
//! else: no 60°/180°/300° rotations (so not p6 or p6m), no
//! 30°/90°/150° mirror families, no glides. The rot120 centres fall
//! into three orbits under the lattice — the grid-vertex cosets of
//! `(0,0)`, `(2,0)`, `(4,0)` — and **all three orbits lie on mirror
//! lines**: the *333 fingerprint.
//!
//! # p3m1 vs p31m — the incidence discriminator
//!
//! The two hexagonal mirror groups differ only in which rot120
//! centres lie on mirrors. Here all three centre orbits lie on
//! mirrors — on-mirror incidence `[true, true, true]`. In contrast,
//! M84's split-triangle grid (p31m) puts exactly one of its three
//! orbits (the grid-vertex orbit) on a mirror: `[true, false,
//! false]`. This is verified exactly by the Rust probe
//! (`alcove_cut_grid_wallpaper_group_is_p3m1`), alongside the
//! p6/p6m-excluding negatives (rot60/rot180/rot300,
//! refl30/90/150 — all zero positive cosets).
//!
//! # Bravais kind
//!
//! **Hexagonal** (primitive): `Λ = ⟨(3,−√3), (3,√3)⟩`,
//! `det Λ = 6√3` — the index-3 sublattice of the side-2 triangular
//! grid's lattice `⟨(2,0), (1,√3)⟩`, so the cell holds six grid
//! triangles (eighteen tiles). (The basis is a rotated orientation,
//! not the axis-aligned `(2,0), (1,√3)` form; the floret entry and
//! M83's `(2√3,0), (√3,3)` bases are precedents for non-canonical
//! hexagonal bases.)
//!
//! # Seeds and protos
//!
//! Three congruence classes per cell (six tiles each — the three
//! piece shapes, each realized in up- and down-triangle
//! orientations), areas `4√3/9`, `13√3/63`, `22√3/63`:
//! `seed = 0` for tiles {0,4,7,9,12,16}, `seed = 1` for
//! {1,3,6,10,13,15}, `seed = 2` for {2,5,8,11,14,17}. Every tile is
//! scalene (chiral individually), but each class is mirror-closed
//! under `y = 0` (three mirror pairs), so the classes are achiral as
//! sets — Pattern coloring distinguishes the three shapes.
//!
//! # Verification
//!
//! Coords are machine-emitted (`m85_emit.py` → `m85_emit_f64.inc` /
//! `m85_emit_x126.inc`, parsed back to the HX-STUDY cell JSON
//! exactly, no hand-typed coordinates). The full HX-STUDY battery
//! was reproduced before porting (`m85_verify.out`): the
//! exact-Fraction battery matches the JSON on every number (area sum
//! `6√3 = det`, e2e 1350/0/0, stars 245/0, coverage 4096/0/0,
//! positives only at the zero coset of rot120/rot240/refl0/refl60/
//! refl120, translations 25/25, five off-lattice negatives 0/5,
//! three rot120 centre orbits all on mirrors), AND the exact-integer
//! ×126 probe that the Rust test below mirrors (lattice
//! `a1×126 = (378,0,0,−126)`, `a2×126 = (378,0,0,126)`; the 11
//! integer `2L` matrices shared with the p3 probes; coupled-floor
//! coset reduction). Rust: the standard trio, the exact-integer p3m1
//! group probe (`alcove_cut_grid_wallpaper_group_is_p3m1`), and the
//! brute-force culling cross-check
//! (`lattice_culling_matches_brute_force`).

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// √3 — the y-coordinate scale of the lattice vectors
/// `a1 = (3, −√3)`, `a2 = (3, √3)`.
const SURD3: f64 = 1.7320508075688773;

fn poly(pts: &[Vec2]) -> Polygon {
    Polygon::from_slice(pts)
}

/// Tile 0: the cut of `U0` at edge `(0,0)–(2,0)`,
/// `[J1, (0,0), (2,0)]`, CCW (class 0, seed 0).
fn piece0() -> Polygon {
    poly(&[
        Vec2::new(8.0 / 7.0, 4.0 / 9.0 * SURD3),
        Vec2::new(0.0, 0.0),
        Vec2::new(2.0, 0.0),
    ])
}

/// Tile 1: the cut of `U0` at edge `(2,0)–(1,√3)`,
/// `[J1, (2,0), (1,√3)]`, CCW (class 1, seed 1).
fn piece1() -> Polygon {
    poly(&[
        Vec2::new(8.0 / 7.0, 4.0 / 9.0 * SURD3),
        Vec2::new(2.0, 0.0),
        Vec2::new(1.0, SURD3),
    ])
}

/// Tile 2: the cut of `U0` at edge `(1,√3)–(0,0)`,
/// `[J1, (1,√3), (0,0)]`, CCW (class 2, seed 2).
fn piece2() -> Polygon {
    poly(&[
        Vec2::new(8.0 / 7.0, 4.0 / 9.0 * SURD3),
        Vec2::new(1.0, SURD3),
        Vec2::new(0.0, 0.0),
    ])
}

/// Tile 3: the cut of `D0` at edge `(1,−√3)–(2,0)`,
/// `[J2, (1,−√3), (2,0)]`, CCW (class 1, seed 1).
fn piece3() -> Polygon {
    poly(&[
        Vec2::new(8.0 / 7.0, -4.0 / 9.0 * SURD3),
        Vec2::new(1.0, -SURD3),
        Vec2::new(2.0, 0.0),
    ])
}

/// Tile 4: the cut of `D0` at edge `(2,0)–(0,0)`,
/// `[J2, (2,0), (0,0)]`, CCW (class 0, seed 0).
fn piece4() -> Polygon {
    poly(&[
        Vec2::new(8.0 / 7.0, -4.0 / 9.0 * SURD3),
        Vec2::new(2.0, 0.0),
        Vec2::new(0.0, 0.0),
    ])
}

/// Tile 5: the cut of `D0` at edge `(0,0)–(1,−√3)`,
/// `[J2, (0,0), (1,−√3)]`, CCW (class 2, seed 2).
fn piece5() -> Polygon {
    poly(&[
        Vec2::new(8.0 / 7.0, -4.0 / 9.0 * SURD3),
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, -SURD3),
    ])
}

/// Tile 6: the cut of `D2` at edge `(1,√3)–(2,0)`,
/// `[J3, (1,√3), (2,0)]`, CCW (class 1, seed 1).
fn piece6() -> Polygon {
    poly(&[
        Vec2::new(37.0 / 21.0, 41.0 / 63.0 * SURD3),
        Vec2::new(1.0, SURD3),
        Vec2::new(2.0, 0.0),
    ])
}

/// Tile 7: the cut of `D2` at edge `(2,0)–(3,√3)`,
/// `[J3, (2,0), (3,√3)]`, CCW (class 0, seed 0).
fn piece7() -> Polygon {
    poly(&[
        Vec2::new(37.0 / 21.0, 41.0 / 63.0 * SURD3),
        Vec2::new(2.0, 0.0),
        Vec2::new(3.0, SURD3),
    ])
}

/// Tile 8: the cut of `D2` at edge `(3,√3)–(1,√3)`,
/// `[J3, (3,√3), (1,√3)]`, CCW (class 2, seed 2).
fn piece8() -> Polygon {
    poly(&[
        Vec2::new(37.0 / 21.0, 41.0 / 63.0 * SURD3),
        Vec2::new(3.0, SURD3),
        Vec2::new(1.0, SURD3),
    ])
}

/// Tile 9: the cut of `U1` at edge `(3,−√3)–(2,0)`,
/// `[J4, (3,−√3), (2,0)]`, CCW (class 0, seed 0).
fn piece9() -> Polygon {
    poly(&[
        Vec2::new(37.0 / 21.0, -41.0 / 63.0 * SURD3),
        Vec2::new(3.0, -SURD3),
        Vec2::new(2.0, 0.0),
    ])
}

/// Tile 10: the cut of `U1` at edge `(2,0)–(1,−√3)`,
/// `[J4, (2,0), (1,−√3)]`, CCW (class 1, seed 1).
fn piece10() -> Polygon {
    poly(&[
        Vec2::new(37.0 / 21.0, -41.0 / 63.0 * SURD3),
        Vec2::new(2.0, 0.0),
        Vec2::new(1.0, -SURD3),
    ])
}

/// Tile 11: the cut of `U1` at edge `(1,−√3)–(3,−√3)`,
/// `[J4, (1,−√3), (3,−√3)]`, CCW (class 2, seed 2).
fn piece11() -> Polygon {
    poly(&[
        Vec2::new(37.0 / 21.0, -41.0 / 63.0 * SURD3),
        Vec2::new(1.0, -SURD3),
        Vec2::new(3.0, -SURD3),
    ])
}

/// Tile 12: the cut of `U2` at edge `(3,√3)–(2,0)`,
/// `[J5, (3,√3), (2,0)]`, CCW (class 0, seed 0).
fn piece12() -> Polygon {
    poly(&[
        Vec2::new(65.0 / 21.0, 13.0 / 63.0 * SURD3),
        Vec2::new(3.0, SURD3),
        Vec2::new(2.0, 0.0),
    ])
}

/// Tile 13: the cut of `U2` at edge `(2,0)–(4,0)`,
/// `[J5, (2,0), (4,0)]`, CCW (class 1, seed 1).
fn piece13() -> Polygon {
    poly(&[
        Vec2::new(65.0 / 21.0, 13.0 / 63.0 * SURD3),
        Vec2::new(2.0, 0.0),
        Vec2::new(4.0, 0.0),
    ])
}

/// Tile 14: the cut of `U2` at edge `(4,0)–(3,√3)`,
/// `[J5, (4,0), (3,√3)]`, CCW (class 2, seed 2).
fn piece14() -> Polygon {
    poly(&[
        Vec2::new(65.0 / 21.0, 13.0 / 63.0 * SURD3),
        Vec2::new(4.0, 0.0),
        Vec2::new(3.0, SURD3),
    ])
}

/// Tile 15: the cut of `D1` at edge `(4,0)–(2,0)`,
/// `[J6, (4,0), (2,0)]`, CCW (class 1, seed 1).
fn piece15() -> Polygon {
    poly(&[
        Vec2::new(65.0 / 21.0, -13.0 / 63.0 * SURD3),
        Vec2::new(4.0, 0.0),
        Vec2::new(2.0, 0.0),
    ])
}

/// Tile 16: the cut of `D1` at edge `(2,0)–(3,−√3)`,
/// `[J6, (2,0), (3,−√3)]`, CCW (class 0, seed 0).
fn piece16() -> Polygon {
    poly(&[
        Vec2::new(65.0 / 21.0, -13.0 / 63.0 * SURD3),
        Vec2::new(2.0, 0.0),
        Vec2::new(3.0, -SURD3),
    ])
}

/// Tile 17: the cut of `D1` at edge `(3,−√3)–(4,0)`,
/// `[J6, (3,−√3), (4,0)]`, CCW (class 2, seed 2).
fn piece17() -> Polygon {
    poly(&[
        Vec2::new(65.0 / 21.0, -13.0 / 63.0 * SURD3),
        Vec2::new(3.0, -SURD3),
        Vec2::new(4.0, 0.0),
    ])
}

/// Alcove-cut grid — the side-2 triangular grid on the index-3
/// sublattice, every grid triangle cut from its alcove point (the
/// orbit of one generic `Q(√3)` junction under the edge-line mirrors
/// of the up-triangle) to its three vertices, per hexagonal cell
/// `⟨(3,−√3), (3,√3)⟩` (det `6√3`): 18 tiles, three congruence
/// classes (each mirror-closed — achiral as a set); edge-to-edge
/// (1350 interior edges, 0 T-junctions); the catalog's p3m1
/// realizer — the *333 fingerprint (all three rot120 centre orbits
/// on mirrors, the contrast to M84's p31m one-orbit-on-mirrors),
/// mirrors in the 0°/60°/120° families, no 60°/180°/300° rotations,
/// no glides; closes the last gap of the 17-group wallpaper catalog
/// (M85, HX-STUDY handoff).
pub fn alcove_cut_grid() -> LatticeTiling {
    // Three congruence classes (six tiles each, mirror-closed under
    // y = 0): class 0 = {0,4,7,9,12,16} (area 4√3/9), class 1 =
    // {1,3,6,10,13,15} (area 13√3/63), class 2 = {2,5,8,11,14,17}
    // (area 22√3/63).
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
            seed: 2,
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
            seed: 2,
        },
        Cell {
            poly: piece6(),
            seed: 1,
        },
        Cell {
            poly: piece7(),
            seed: 0,
        },
        Cell {
            poly: piece8(),
            seed: 2,
        },
        Cell {
            poly: piece9(),
            seed: 0,
        },
        Cell {
            poly: piece10(),
            seed: 1,
        },
        Cell {
            poly: piece11(),
            seed: 2,
        },
        Cell {
            poly: piece12(),
            seed: 0,
        },
        Cell {
            poly: piece13(),
            seed: 1,
        },
        Cell {
            poly: piece14(),
            seed: 2,
        },
        Cell {
            poly: piece15(),
            seed: 1,
        },
        Cell {
            poly: piece16(),
            seed: 0,
        },
        Cell {
            poly: piece17(),
            seed: 2,
        },
    ];

    LatticeTiling {
        id: "alcove-cut-grid",
        name: "Alcove-cut grid",
        description: "Side-2 triangular grid on the index-3 sublattice in which every grid triangle is cut from its alcove point — the orbit of one generic Q(√3) junction under the edge-line mirrors of the up-triangle — to its three vertices, per hexagonal cell <(3,−√3),(3,√3)> (det 6√3): 18 tiles, three congruence classes (each mirror-closed, achiral as a set); edge-to-edge (1350 interior edges, 0 T-junctions); the catalog's p3m1 realizer — the *333 fingerprint (all three rot120 centre orbits on mirrors, the contrast to M84's p31m one-orbit-on-mirrors), mirrors in the 0°/60°/120° families, no 60°/180°/300° rotations, no glides; closes the last gap of the 17-group wallpaper catalog (M85, HX-STUDY handoff).",
        lattice: [Vec2::new(3.0, -SURD3), Vec2::new(3.0, SURD3)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P3m1,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Wallpaper,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
