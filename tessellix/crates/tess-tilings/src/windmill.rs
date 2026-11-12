//! Periodic pinwheel arrangement ("windmill"), wallpaper group p4.
//!
//! The periodic companion to the aperiodic `pinwheel` tiling: Conway's
//! "spin" of the pinwheel motif — 1–2–√5 right triangles arranged
//! around a square, repeated on the 3×3 square lattice. Each 3×3 cell
//! holds four triangles (orientation classes 1–4, seeds 1–4) and one
//! square (seed 0 — the largest prototile, per the AGENTS.md seed
//! rule); areas 4·1 + 5 = 9 = |det|, all coordinates exact integers,
//! no surds.
//!
//! Not edge-to-edge: the triangles' length-2 legs meet the square's
//! sides at T-junctions (a 2-leg split against a 1-leg, as in
//! `chair.rs` / `pinwheel.rs`). The vertex figures have two orbits:
//! 90⁴ at the lattice points (3i, 3j) and
//! 180·90·arctan(1/2)·arctan(2) at the four T-junction residue
//! classes {(0,1),(0,2),(1,0),(2,0)} mod 3 (descriptive angles only;
//! the table is all integers).
//!
//! The arrangement is homochiral — all four block triangles share one
//! chirality sign (R90 about the motif centre permutes them
//! T1→T4→T3→T2→T1, never reflecting) — so the group is p4, not p4m or
//! p4g: 4-fold centres at both the motif centres (3i+1.5, 3j+1.5)
//! and the lattice points (3i, 3j), 2-folds at all four classes
//! {(0,0),(1.5,0),(0,1.5),(1.5,1.5)} mod 3, no mirrors, no glides.
//!
//! Naming waiver (AGENTS.md step 4, approved by the repo owner
//! 2026-11-12): no canonical published name exists for this
//! arrangement — Wikipedia has no "Windmill tiling", the pinwheel
//! literature (Conway, Radin) is aperiodic-only, and Grünbaum's 1977
//! "windmill blades" quilt design is a different, edge-to-edge polygon
//! tiling — so the descriptive name "Periodic pinwheel arrangement
//! (p4)" is used with repo-owner approval.

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

pub fn spec() -> LatticeTiling {
    // Cell [0,3]×[0,3]: four 1–2–√5 triangles (orientation classes
    // 1–4, seeds 1–4) pinwheeling around the central square
    // (seed 0, the largest prototile).
    let t1 = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(2.0, 0.0),
        Vec2::new(0.0, 1.0),
    ]);
    let t2 = Polygon::from_slice(&[
        Vec2::new(0.0, 1.0),
        Vec2::new(1.0, 3.0),
        Vec2::new(0.0, 3.0),
    ]);
    let t3 = Polygon::from_slice(&[
        Vec2::new(3.0, 3.0),
        Vec2::new(1.0, 3.0),
        Vec2::new(3.0, 2.0),
    ]);
    let t4 = Polygon::from_slice(&[
        Vec2::new(3.0, 2.0),
        Vec2::new(2.0, 0.0),
        Vec2::new(3.0, 0.0),
    ]);
    let square = Polygon::from_slice(&[
        Vec2::new(2.0, 0.0),
        Vec2::new(3.0, 2.0),
        Vec2::new(1.0, 3.0),
        Vec2::new(0.0, 1.0),
    ]);
    LatticeTiling {
        id: "windmill",
        name: "Periodic pinwheel arrangement (p4)",
        description: "Conway's periodic 'spin' of the pinwheel motif: 1–2–√5 \
                      right triangles around squares on the 3×3 square lattice; \
                      homochiral, so p4 (4-folds at motif centres and lattice \
                      points, no mirrors or glides); non-edge-to-edge \
                      (T-junctions).",
        lattice: [Vec2::new(3.0, 0.0), Vec2::new(0.0, 3.0)],
        cells: vec![
            Cell { poly: t1, seed: 1 },
            Cell { poly: t2, seed: 2 },
            Cell { poly: t3, seed: 3 },
            Cell { poly: t4, seed: 4 },
            Cell { poly: square, seed: 0 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P4,
        lattice_kind: LatticeKind::Square,
        category: TilingCategory::Wallpaper,
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
