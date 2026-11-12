//! Khatam girih decomposition (M36) — the classical 8-pointed khatam
//! ({8/3} star-and-cross) decomposed into its prototile kitab, expressed
//! as a [`LatticeTiling`] data table.
//!
//! # The decomposition
//!
//! The kissing khatam — the `T = 1/2` classical case of the
//! [`crate::zellige::StarCrossTiling`] family (M15) — cuts into a small
//! girih-style kitab of three prototiles, all angles multiples of
//! 22.5°:
//!
//! * **rhombus** (45°/135°) — each {8/3} khatam star is cut by the
//!   segments joining its centre to its 8 notches, giving 8 congruent
//!   rhombi with vertices `centre, notch, tip, notch`; every side equals
//!   the notch radius `ρ = √(4−2√2)/4`;
//! * **square** (side `(2−√2)/2`) — each interstice cross (the concave
//!   16-gon between four kissing stars) contains the square through the
//!   four diagonal star-tip apexes;
//! * **pentagon** (angles 135°, 90°, 90°, 112.5°, 112.5°; four sides of
//!   `ρ` and one square side) — the rest of each cross falls into 4
//!   congruent pieces, each bounded by 4 star edges and 1 square edge.
//!
//! Per unit cell: 8 rhombi + 1 square + 4 pentagons = 13 tiles, with
//! areas `8·(√2−1)/8 + (3−2√2)/2 + 4·(1/8) = 1` exactly.
//!
//! The cut is edge-to-edge across the whole plane: every edge is a full
//! edge on both sides (no T-junctions), and the vertex figures are
//!
//! * `4⁸` at the star centres (eight rhombi at 45° each),
//! * `90.135.135` at the star notches (2 rhombi + 1 pentagon),
//! * `45.45.135.135` at the kissing points (2 rhombi + 2 pentagons),
//! * `45.90.112.5.112.5` at the diagonal tips (1 rhombus + square
//!   + 2 pentagons).
//!
//! The wallpaper group is **p4m** (square lattice, mirrors through the
//! star centres and the cross centres); the cut is invariant under all
//! of them.
//!
//! # Verification
//!
//! Verified exactly before porting (Python, stdlib) over a ±3×±3 lattice
//! patch (637 tiles): per-cell area sum = 1 to 1.1e-16; every emitted
//! tile has exactly the claimed side lengths and angles; all 448 interior
//! edges shared by exactly 2 tiles; all 257 interior vertex angle sums
//! = 360° falling into exactly the four orbits above; a 29 580-point
//! irrational-offset grid over [−2,2]² covered exactly once; the patch
//! set invariant under the p4m mirror and 90° rotation.

use std::f64::consts::SQRT_2;

use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

// Exact constants of the kissing khatam (lattice constant 1, star tip
// radius t = 1/2). Offsets as in `zellige.rs`: tn = t(√2−1)/2 (notch
// offset perpendicular to the tip axis), t2 = t/2 (notch offset along
// it), ts = t√2/2 (diagonal tip coordinate).
const T2: f64 = 0.25; // t/2 = 1/4
const TN: f64 = (SQRT_2 - 1.0) / 4.0; // t(√2−1)/2
const TS: f64 = SQRT_2 / 4.0; // t√2/2
const R45: f64 = SQRT_2 / 2.0; // cos 45° = sin 45°

/// CCW rotation by 45° (exact in Q(√2)).
fn rot45(p: Vec2) -> Vec2 {
    Vec2::new((p.x - p.y) * R45, (p.x + p.y) * R45)
}

/// CCW rotation by 90° about the cross centre (1/2, 1/2).
fn rot90c(p: Vec2) -> Vec2 {
    Vec2::new(1.0 - p.y, p.x)
}

fn poly(pts: &[Vec2]) -> Polygon {
    Polygon::from_slice(pts)
}

/// Khatam girih decomposition — the classical 8-pointed star-and-cross
/// cut into its rhombus / square / pentagon kitab (p4m, square lattice).
pub fn khatam_girih() -> LatticeTiling {
    // Star (0,0): 8 tips at radius 1/2 and 8 notches at radius
    // ρ = √(4−2√2)/4 on the 22.5° grid. The base notch is exactly
    // (t2, tn); the rest follow by 45° rotation. Each vertex is computed
    // once so that edge-sharing tiles reference bit-identical points.
    let mut tips = [Vec2::ZERO; 8];
    let mut notches = [Vec2::ZERO; 8];
    let (mut tip, mut notch) = (Vec2::new(0.5, 0.0), Vec2::new(T2, TN));
    for k in 0..8 {
        tips[k] = tip;
        notches[k] = notch;
        tip = rot45(tip);
        notch = rot45(notch);
    }

    let origin = Vec2::ZERO;

    // 8 rhombi of the star: centre, notch(k−1), tip(k), notch(k) — CCW.
    let mut cells: Vec<Cell> = Vec::with_capacity(13);
    for k in 0..8 {
        cells.push(Cell {
            poly: poly(&[origin, notches[(k + 7) % 8], tips[k], notches[k]]),
            seed: 2,
        });
    }

    // Cross (0,0) — central square through the four diagonal tip apexes.
    cells.push(Cell {
        poly: poly(&[
            Vec2::new(TS, TS),
            Vec2::new(1.0 - TS, TS),
            Vec2::new(1.0 - TS, 1.0 - TS),
            Vec2::new(TS, 1.0 - TS),
        ]),
        seed: 1,
    });

    // 4 pentagons: the East piece is bounded by the star edges around
    // the kissing point (1, 1/2) — tip 90° of star (1,0) = tip 270° of
    // star (1,1) — and the diagonal tips 135° / 225°; its fifth edge is
    // the square edge between those tips. Notch offsets adjacent to the
    // diagonal tips are exactly (∓tn, ±t2).
    let east = [
        Vec2::new(1.0, 0.5),           // kissing point A
        Vec2::new(1.0 - TN, 1.0 - T2), // notch 247.5° of star (1,1)
        Vec2::new(1.0 - TS, 1.0 - TS), // diagonal tip 225° of star (1,1)
        Vec2::new(1.0 - TS, TS),       // diagonal tip 135° of star (1,0)
        Vec2::new(1.0 - TN, T2),       // notch 112.5° of star (1,0)
    ];
    for k in 0..4 {
        let mut p = east;
        for v in &mut p {
            for _ in 0..k {
                *v = rot90c(*v);
            }
        }
        cells.push(Cell {
            poly: poly(&p),
            seed: 0,
        });
    }

    LatticeTiling {
        id: "khatam-girih",
        name: "Khatam Girih Decomposition (8-point)",
        description: "Classical kissing khatam ({8/3} star-and-cross, the T = 1/2 case of star-cross) decomposed girih-style: each 8-pointed star cut into 8 rhombi (45°/135°, side √(4−2√2)/4), each interstice cross into a central square and 4 pentagons (135/90/90/112.5/112.5). Edge-to-edge across the plane; vertex figures 4⁸, 90.135.135, 45.45.135.135, 45.90.112.5.112.5 (p4m).",
        lattice: [Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P4m,
        lattice_kind: LatticeKind::Square,
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
