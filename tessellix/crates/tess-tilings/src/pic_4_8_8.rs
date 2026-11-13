//! Hankin polygons-in-contact fourfold khatam strapwork (M53).
//!
//! Roger Hankin's polygons-in-contact (PIC) construction: from every edge
//! midpoint of a substrate tiling two wall rays depart at a fixed contact
//! angle δ to the edge, symmetric about the inward normal; where the rays
//! meet a substrate edge they reflect at the next midpoint, and the wall
//! network partitions the plane into the khatam pieces. This module ships
//! the **fourfold engine** — the regular square/hexagon/octagon family
//! with δ = 45° — laid on the truncated-square (4.8.8) Archimedean
//! substrate: the classic fourfold khatam ("median" variant, as in Eric
//! Broug's *Handbook of Islamic Pattern* octagram construction).
//!
//! ## Design decision (P2): tiling mode, not layer mode
//!
//! The strapwork *region partition* (octagon, small square, house
//! pentagon, star-tip triangle) is itself a face-to-face periodic tiling
//! by exact polygons, so it registers directly as a registry tiling
//! rather than as a rendering layer on a host tiling — the M35 `girih-sc`
//! precedent of shipping a derived pattern as its own periodic tiling.
//! The PIC **layer mode** (drawing the wall network over an arbitrary
//! host substrate without registering a partition) is recorded as future
//! work in PROPOSED.md #14.
//!
//! ## Construction — exact over Q(√2)
//!
//! Substrate: 4.8.8 with unit edges on the square lattice `A·Z²`,
//! `A = 1+√2` (octagon centres at `(A/2, A/2) + A·Z²`, substrate-square
//! centres at `A·Z²`). At δ = 45° every ray from an edge midpoint exits
//! the tile exactly at another edge midpoint, so the wall network is the
//! union of the chords joining opposite edge midpoints: inside an octagon
//! the `{8/2}` chords (the two overlaid squares of the classic octagram,
//! Rub el Hizb), inside each substrate square the four midpoint chords.
//! Neighbouring tiles join head-to-head at shared midpoints, so the
//! network is face-to-face: the angle census of the partition is exactly
//! {45°, 90°, 135°}.
//!
//! One lattice cell holds 14 faces in four seed classes:
//!
//! | seed | piece                         | n  | area       |
//! |------|-------------------------------|----|------------|
//! | 0    | regular octagon (octagram core) | 1 | `1+√2`    |
//! | 1    | small square `[±√2/4]²`       | 1  | `1/2`      |
//! | 2    | house pentagon                | 4  | `1/8+√2/4` |
//! | 3    | star-tip triangle             | 8  | `1/8`      |
//!
//! `Σ = (1+√2) + 1/2 + 4(1/8+√2/4) + 8(1/8) = 3+2√2 = A² = |det|`
//! exactly over Q(√2). Every vertex coordinate is a `(p+q√2)/4` surd
//! with integer p, q (the table stores numerator quadruples
//! `[p, q, r, s]`, vertex = `((p+q√2)/4, (r+s√2)/4)`), so nothing in the
//! construction depends on decimal approximation: the table was derived
//! programmatically from the exact Q(√2) Python spike and re-verified
//! there (coverage exactly 1× at 8100 probes, every interior edge shared
//! by exactly 2 faces, full angle census, all symmetry probes).
//!
//! ## Symmetry — verified exactly p4m
//!
//! Positive probes (each mapping every orbit face onto an orbit face
//! plus an exact lattice translation): rot90 about the substrate-square
//! centre and about the octagon centre, rot180 about the two edge-contact
//! classes, mirrors y = 0, x = 0, y = A/2, y = x and y = −x. Negative
//! probes (must fail, pinning the group to *exactly* p4m): rot90 about
//! (A/4, A/4), rot180 about (A/4, 0), and the glide (x, y) ↦
//! (x + A/2, A/2 − y). The partition is un-interlaced (over/under strand
//! crossing is a rendering concern), which is what makes the mirrors and
//! the full p4m holonomy exact.
//!
//! No image, PDF or other visual material is ingested; the construction
//! is derived from published mathematics (Broug's fourfold-star PIC
//! description) with prose fair-use reference only.
use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

const RT2: f64 = core::f64::consts::SQRT_2;
/// Truncated-square lattice constant for unit substrate edges.
const A: f64 = 1.0 + RT2;

/// Cell constructor: each vertex is `(p + q√2)/4` (integer numerators,
/// verified exact for the whole cell).
fn cell(seed: u32, g: &[[f64; 4]]) -> Cell {
    Cell {
        seed,
        poly: Polygon::from_slice(
            &g.iter()
                .map(|q| Vec2::new((q[0] + q[1] * RT2) / 4.0, (q[2] + q[3] * RT2) / 4.0))
                .collect::<Vec<_>>(),
        ),
    }
}

/// Hankin PIC fourfold khatam (4.8.8 substrate, δ = 45°).
pub fn pic_4_8_8() -> LatticeTiling {
    let cells = vec![
        // seed 0 — regular octagon (octagram core) centred on (A/2, A/2), area 1+√2
        cell(0, &[
            [4.0,3.0,2.0,3.0],
            [2.0,3.0,4.0,3.0],
            [2.0,1.0,4.0,3.0],
            [0.0,1.0,2.0,3.0],
            [0.0,1.0,2.0,1.0],
            [2.0,1.0,0.0,1.0],
            [2.0,3.0,0.0,1.0],
            [4.0,3.0,2.0,1.0],
        ]),
        // seed 1 — small square [±√2/4]² (khatam centre), area 1/2
        cell(1, &[
            [0.0,-1.0,0.0,1.0],
            [0.0,-1.0,0.0,-1.0],
            [0.0,1.0,0.0,-1.0],
            [0.0,1.0,0.0,1.0],
        ]),
        // seed 2 — house pentagon, area 1/8 + √2/4
        cell(2, &[
            [0.0,-1.0,0.0,1.0],
            [0.0,1.0,0.0,1.0],
            [0.0,1.0,2.0,1.0],
            [0.0,0.0,2.0,2.0],
            [0.0,-1.0,2.0,1.0],
        ]),
        // seed 2 — house pentagon, area 1/8 + √2/4 (2)
        cell(2, &[
            [0.0,0.0,2.0,2.0],
            [0.0,1.0,2.0,3.0],
            [0.0,1.0,4.0,3.0],
            [0.0,-1.0,4.0,3.0],
            [0.0,-1.0,2.0,3.0],
        ]),
        // seed 2 — house pentagon, area 1/8 + √2/4 (3)
        cell(2, &[
            [2.0,2.0,0.0,0.0],
            [2.0,1.0,0.0,1.0],
            [0.0,1.0,0.0,1.0],
            [0.0,1.0,0.0,-1.0],
            [2.0,1.0,0.0,-1.0],
        ]),
        // seed 2 — house pentagon, area 1/8 + √2/4 (4)
        cell(2, &[
            [2.0,3.0,0.0,-1.0],
            [4.0,3.0,0.0,-1.0],
            [4.0,3.0,0.0,1.0],
            [2.0,3.0,0.0,1.0],
            [2.0,2.0,0.0,0.0],
        ]),
        // seed 3 — star-tip triangle, area 1/8
        cell(3, &[
            [0.0,1.0,2.0,1.0],
            [0.0,1.0,2.0,3.0],
            [0.0,0.0,2.0,2.0],
        ]),
        // seed 3 — star-tip triangle, area 1/8 (2)
        cell(3, &[
            [2.0,1.0,0.0,1.0],
            [0.0,1.0,2.0,1.0],
            [0.0,1.0,0.0,1.0],
        ]),
        // seed 3 — star-tip triangle, area 1/8 (3)
        cell(3, &[
            [0.0,1.0,2.0,3.0],
            [2.0,1.0,4.0,3.0],
            [0.0,1.0,4.0,3.0],
        ]),
        // seed 3 — star-tip triangle, area 1/8 (4)
        cell(3, &[
            [2.0,3.0,0.0,1.0],
            [2.0,1.0,0.0,1.0],
            [2.0,2.0,0.0,0.0],
        ]),
        // seed 3 — star-tip triangle, area 1/8 (5)
        cell(3, &[
            [2.0,1.0,4.0,3.0],
            [2.0,3.0,4.0,3.0],
            [2.0,2.0,4.0,4.0],
        ]),
        // seed 3 — star-tip triangle, area 1/8 (6)
        cell(3, &[
            [4.0,3.0,0.0,1.0],
            [4.0,3.0,2.0,1.0],
            [2.0,3.0,0.0,1.0],
        ]),
        // seed 3 — star-tip triangle, area 1/8 (7)
        cell(3, &[
            [2.0,3.0,4.0,3.0],
            [4.0,3.0,2.0,3.0],
            [4.0,3.0,4.0,3.0],
        ]),
        // seed 3 — star-tip triangle, area 1/8 (8)
        cell(3, &[
            [4.0,4.0,2.0,2.0],
            [4.0,3.0,2.0,3.0],
            [4.0,3.0,2.0,1.0],
        ]),
    ];
    LatticeTiling {
        id: "pic-4-8-8",
        name: "Hankin PIC fourfold khatam",
        description: "Hankin polygons-in-contact strapwork on the truncated-square (4.8.8) substrate at contact angle 45°: fourfold khatam partition into octagon, small square, house pentagon and star-tip triangle, exact over Q(√2), 14 faces per lattice cell, wallpaper group p4m.",
        lattice: [Vec2::new(A, 0.0), Vec2::new(0.0, A)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P4m,
        lattice_kind: LatticeKind::Square,
        category: TilingCategory::Wallpaper,
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
