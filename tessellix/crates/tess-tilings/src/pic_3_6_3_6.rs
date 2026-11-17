//! Hankin polygons-in-contact sixfold khatam strapwork (M54).
//!
//! Roger Hankin's polygons-in-contact (PIC) construction: from every edge
//! midpoint of a substrate tiling two wall rays depart at a fixed contact
//! angle δ to the edge, symmetric about the inward normal; where a ray meets
//! another substrate edge it is taken on at the next midpoint, and the wall
//! network partitions the plane into the khatam pieces. This module ships
//! the **sixfold engine** — δ = 60° — laid on the trihexagonal (3.6.3.6)
//! Archimedean substrate: the classic sixfold khatam, the hexagonal twin of
//! the fourfold `pic_4_8_8` (M53).
//!
//! ## Why δ = 60° is the closed choice
//!
//! Rays from the two ends `V_i`, `V_{i+1}` of a substrate edge meet on the
//! edge's midline at depth `(s/2)·tan(90° − δ)` (for the unit edge here,
//! `0.5/tan δ`). Walking the edge, the ray launched at the next midpoint
//! closes exactly onto that point only when the depth is `√3/4`, i.e. only at
//! δ = 60°: the chord then spans exactly two midpoint steps, so every wall
//! ends head-to-head on another midpoint and the network is face-to-face.
//! At δ = 30° the depth is `√3/2` and the hexagon chords span four steps —
//! the same lines run backwards, so the triangle walls never arrive; δ = 90°
//! sends the rays through the substrate vertices, producing T-junctions. So
//! the sixfold engine closes at δ = 60° and the wall network is
//!
//! * in each hexagon the `{6/2}` hexagram — the chords `M_i → M_{i+2}`
//!   joining every edge midpoint to the two midpoints two steps away;
//! * in each triangle the three midsegments joining its edge midpoints.
//!
//! ## Closure — the network is a half-scale 3.6.3.6
//!
//! Both families are exactly the wall network of a trihexagonal tiling of
//! edge `1/2`. The core of each hexagram is a regular hexagon of side `1/2`
//! and the hexagram arms and the midsegment cuts are equilateral triangles
//! of side `1/2`; around each hexagon–hexagon contact the two hexagram
//! chords and the two midsegments that arrive there alternate into a fourth
//! such hexagon. Every piece edge is therefore a half-chord or a
//! half-midsegment — the partition is face-to-face with only 60° and 120°
//! corners. Per lattice cell of `Λ = ⟨(2, 0), (1, √3)⟩` (the substrate's own
//! translation lattice, one hexagon per cell) the partition is
//!
//! | seed | piece                              | n | area        |
//! |------|------------------------------------|---|-------------|
//! | 0    | regular hexagon (hexagram core)    | 4 | `3√3/8`     |
//! | 1    | hexagram tip triangle              | 6 | `√3/16`     |
//! | 2    | midsegment (medial) triangle       | 2 | `√3/16`     |
//!
//! `Σ = 4·(3√3/8) + 8·(√3/16) = 3√3/2 + √3/2 = 2√3 = |det Λ|` exactly, so
//! the twelve faces tile the plane with no gap and no overlap. That the
//! twelve are genuinely distinct orbits is pinned by their centroids: the
//! four hexagons sit on the hexagon centre and on the three hexagon–hexagon
//! contacts, `O`, `(1, 0)`, `(1/2, √3/2)`, `(3/2, √3/2)`, the six tips ride
//! the six hexagram arms of the hexagon at `O`, and the two medials sit in
//! the two substrate triangles — twelve pairwise non-congruent positions
//! modulo `Λ`.
//!
//! ## Exact over Q(√3)
//!
//! Every vertex coordinate is a `(p + q√3)/4` surd with integer `p, q` (the
//! table stores numerator quadruples `[p, q, r, s]`, vertex =
//! `((p+q√3)/4, (r+s√3)/4)`); in fact all of them land on the quarter grid
//! `x ∈ Z/4`, `y ∈ (√3/4)·Z`. Nothing in the construction depends on decimal
//! approximation: the table was derived programmatically from an exact
//! Q(√3) Python spike (spike `VERDICT: ALL GOOD`) and re-verified there —
//! face-to-face wall network by exact segment splitting, coverage exactly 1×
//! at 32 300 probes, every interior edge shared by exactly 2 faces, corner
//! angles only 60°/120° summing to 360° at all 50 core vertices, and Σ areas
//! = 2√3 as an exact field identity. A separate exact script re-checked the
//! shipped table against the spike's face set in both directions.
//!
//! ## Symmetry — verified exactly p6m
//!
//! Positive probes (each mapping every table face onto a table face plus an
//! exact lattice translation): rot60 about the hexagon centre `O` and about
//! the contact `(1, 0)`, rot120 about a substrate triangle centre, rot180
//! about an edge midline point, and the mirrors `y = 0`, `x = 0`, `y = √3`,
//! `x = 2` and the 60° mirror through `O`. Negative probes (must fail, which
//! is what pins the group to *exactly* p6m rather than a supergroup): rot90
//! about `O`, rot60 about the triangle centre, rot180 about a quarter-point
//! and the mirror `y = √3/4`. The partition is un-interlaced (over/under
//! strand crossing is a rendering concern), which is what keeps the mirrors
//! and the full p6m holonomy exact.
//!
//! No image, PDF or other visual material is ingested; the construction is
//! derived from published mathematics (Hankin's sixfold PIC family and the
//! standard `{6/2}` hexagram construction) with prose fair-use reference
//! only.
use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// √3/2 — the same apothem constant the trihexagonal substrate module
/// (`archimedean`) uses, so the two tilings agree bit for bit.
const H: f64 = 0.866_025_403_784_438_6;
/// √3, exactly `2·H` (doubling is exact in binary floating point).
/// `std::f64::consts::SQRT_3` is unstable at this crate's MSRV.
const RT3: f64 = 2.0 * H;

/// Cell constructor: each vertex is `(p + q√3)/4` (integer numerators,
/// verified exact for the whole cell).
fn cell(seed: u32, g: &[[f64; 4]]) -> Cell {
    Cell {
        seed,
        poly: Polygon::from_slice(
            &g.iter()
                .map(|q| Vec2::new((q[0] + q[1] * RT3) / 4.0, (q[2] + q[3] * RT3) / 4.0))
                .collect::<Vec<_>>(),
        ),
    }
}

/// Hankin PIC sixfold khatam (3.6.3.6 substrate, δ = 60°).
pub fn pic_3_6_3_6() -> LatticeTiling {
    let cells = vec![
        // seed 0 — regular hexagon (hexagram core) centred on O, side 1/2,
        // area 3√3/8
        cell(0, &[
            [2.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 1.0],
            [-1.0, 0.0, 0.0, 1.0],
            [-2.0, 0.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0, -1.0],
            [1.0, 0.0, 0.0, -1.0],
        ]),
        // seed 0 — regular hexagon centred on the contact (1, 0)
        cell(0, &[
            [6.0, 0.0, 0.0, 0.0],
            [5.0, 0.0, 0.0, 1.0],
            [3.0, 0.0, 0.0, 1.0],
            [2.0, 0.0, 0.0, 0.0],
            [3.0, 0.0, 0.0, -1.0],
            [5.0, 0.0, 0.0, -1.0],
        ]),
        // seed 0 — regular hexagon centred on the contact (1/2, √3/2)
        cell(0, &[
            [4.0, 0.0, 0.0, 2.0],
            [3.0, 0.0, 0.0, 3.0],
            [1.0, 0.0, 0.0, 3.0],
            [0.0, 0.0, 0.0, 2.0],
            [1.0, 0.0, 0.0, 1.0],
            [3.0, 0.0, 0.0, 1.0],
        ]),
        // seed 0 — regular hexagon centred on the contact (3/2, √3/2)
        cell(0, &[
            [8.0, 0.0, 0.0, 2.0],
            [7.0, 0.0, 0.0, 3.0],
            [5.0, 0.0, 0.0, 3.0],
            [4.0, 0.0, 0.0, 2.0],
            [5.0, 0.0, 0.0, 1.0],
            [7.0, 0.0, 0.0, 1.0],
        ]),
        // seed 1 — hexagram tip, apex on the hexagon midpoint M0 = (3/4, √3/4)
        cell(1, &[[2.0, 0.0, 0.0, 0.0], [3.0, 0.0, 0.0, 1.0], [1.0, 0.0, 0.0, 1.0]]),
        // seed 1 — tip, apex M1 = (0, √3/2)
        cell(1, &[[1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 0.0, 2.0], [-1.0, 0.0, 0.0, 1.0]]),
        // seed 1 — tip, apex M2 = (-3/4, √3/4)
        cell(1, &[
            [-1.0, 0.0, 0.0, 1.0],
            [-3.0, 0.0, 0.0, 1.0],
            [-2.0, 0.0, 0.0, 0.0],
        ]),
        // seed 1 — tip, apex M3 = (-3/4, -√3/4)
        cell(1, &[
            [-2.0, 0.0, 0.0, 0.0],
            [-3.0, 0.0, 0.0, -1.0],
            [-1.0, 0.0, 0.0, -1.0],
        ]),
        // seed 1 — tip, apex M4 = (0, -√3/2)
        cell(1, &[
            [-1.0, 0.0, 0.0, -1.0],
            [0.0, 0.0, 0.0, -2.0],
            [1.0, 0.0, 0.0, -1.0],
        ]),
        // seed 1 — tip, apex M5 = (3/4, -√3/4)
        cell(1, &[[1.0, 0.0, 0.0, -1.0], [3.0, 0.0, 0.0, -1.0], [2.0, 0.0, 0.0, 0.0]]),
        // seed 2 — midsegment cut of the down-pointing substrate triangle
        // (1, 0), (1/2, √3/2), (3/2, √3/2)
        cell(2, &[[3.0, 0.0, 0.0, 1.0], [5.0, 0.0, 0.0, 1.0], [4.0, 0.0, 0.0, 2.0]]),
        // seed 2 — midsegment cut of the up-pointing substrate triangle
        // (1, 0), (1/2, -√3/2), (3/2, -√3/2)
        cell(2, &[
            [3.0, 0.0, 0.0, -1.0],
            [4.0, 0.0, 0.0, -2.0],
            [5.0, 0.0, 0.0, -1.0],
        ]),
    ];
    LatticeTiling {
        id: "pic-3-6-3-6",
        name: "Hankin PIC sixfold khatam",
        description: "Hankin polygons-in-contact strapwork on the trihexagonal (3.6.3.6) substrate at contact angle 60°: sixfold khatam partition into hexagram-core hexagon, hexagram tip and midsegment triangle, exact over Q(√3), 12 faces per lattice cell, wallpaper group p6m.",
        lattice: [Vec2::new(2.0, 0.0), Vec2::new(1.0, RT3)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Islamic,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
