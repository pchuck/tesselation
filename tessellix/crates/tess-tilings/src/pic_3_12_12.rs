//! Hankin polygons-in-contact twelvefold khatam strapwork (M56).
//!
//! Roger Hankin's polygons-in-contact (PIC) construction: from every edge
//! midpoint of a substrate tiling two wall rays depart at a fixed contact
//! angle δ to the edge, symmetric about the inward normal, and the wall
//! network partitions the plane into khatam pieces. This module ships the
//! **twelvefold engine** — δ = 60° — on the truncated-hexagonal (3.12.12)
//! Archimedean substrate: the twelve-pointed rosette (khatam-e shamsa),
//! third of the PIC family after the fourfold `pic_4_8_8` (M53) and the
//! sixfold `pic_3_6_3_6` (M54).
//!
//! ## Why δ = 60° is the only closing choice
//!
//! The midpoints of a regular n-gon lie on its incircle, `360°/n` apart. A
//! ray that leaves a midpoint at angle δ to the edge leaves it at angle δ to
//! the tangent there, so by the tangent-chord (alternate segment) theorem it
//! cuts the incircle again after a central angle of exactly `2δ`. It
//! therefore arrives head-on at another midpoint iff `2δ = 360°·k/n`, i.e.
//! iff `δ = 180°·k/n`:
//!
//! * dodecagon, `n = 12`: `δ ∈ {15°, 30°, 45°, 60°, 75°, 90°}`;
//! * triangle, `n = 3`: only `δ = 60°` in `(0°, 90°]` (δ = 120° is the same
//!   ray backwards).
//!
//! The two lists meet at δ = 60° alone, so that is the one contact angle at
//! which every wall lands on a midpoint instead of dangling on an edge.
//! δ = 90°, the other angle the dodecagon would accept, runs the triangle
//! rays along their normals straight through the substrate vertices and
//! leaves T-junctions behind — the M54 lesson, repeated here. At δ = 60° the
//! wall network is
//!
//! * in every dodecagon the 12 chords `M_i → M_{i+4}` — the `{12/4}` star,
//!   four overlaid equilateral triangles, i.e. the twelvefold rosette;
//! * in every triangle its three midsegments `M_i → M_{i+1}`.
//!
//! Every wall that reaches a substrate midpoint has a collinear partner
//! leaving the far side — at the dodecagon–triangle midpoint `M_0` the
//! incident walls pair up into the 30° and 150° lines, at the
//! dodecagon–dodecagon midpoint `M_1` into the 0° and 60° lines — so the
//! walls run straight through the shared midpoints and the network is
//! face-to-face: exact segment splitting of the 18 walls per
//! cell over 49 cells (882 whole walls) gives V = 2266, E = 4410, F = 2146
//! (Euler characteristic 2), every interior edge shared by exactly 2 faces,
//! and no T-junction anywhere.
//!
//! ## The partition: 45 faces in six orbits
//!
//! Over the substrate's own translation lattice
//! `Λ = ⟨(3/2+√3, 1+√3/2), (0, 2+√3)⟩` — hexagonal, both vectors of length
//! `2+√3` and 60° apart, `|det Λ| = 6 + 7√3/2`, one dodecagon and two
//! triangles per cell — the partition has 45 faces in six congruence orbits:
//!
//! | seed | piece                                    | corners        | sides   | area            | n  |
//! |------|------------------------------------------|----------------|---------|-----------------|----|
//! | 0    | rosette core: regular dodecagon at `O`   | 150°           | 1/2     | `3/2 + 3√3/4`   | 1  |
//! | 1    | vertex pentagon (see below)              | 90,90,120×3    | 1/2, ℓ  | `3/8 + 3√3/16`  | 6  |
//! | 2    | rosette band quadrilateral               | 60,90,90,120   | ℓ, m    | `1/8 + √3/12`   | 12 |
//! | 3    | medial quarter of a substrate triangle   | 60°            | 1/2     | `√3/16`         | 2  |
//! | 4    | inner quadrilateral                      | 60,60,90,150   | √3/6, m | `1/16 + √3/48`  | 12 |
//! | 5    | star notch triangle                      | 30,120,30      | √3/6, ½ | `√3/48`         | 12 |
//!
//! with `ℓ = √(1/4 + √3/8)` and `m = √(1/12 + √3/24)`. The areas add up
//! exactly:
//!
//! ```text
//! 1·(3/2 + 3√3/4) + 6·(3/8 + 3√3/16) + 12·(1/8 + √3/12)
//!   + 2·(√3/16) + 12·(1/16 + √3/48) + 12·(√3/48) = 6 + 7√3/2 = |det Λ|
//! ```
//!
//! so the 45 pieces cover the plane exactly once — no gap, no overlap.
//!
//! The resulting picture is the classic twelvefold one. The midsegments cut
//! every substrate triangle into four equal quarters of area `√3/16`: the
//! medial quarter is a face of its own (seed 3) and the three corner quarters
//! are tips of vertex pentagons. Each of the six vertex pentagons wraps one
//! 3.12.12 vertex (one triangle + two dodecagons), its short side being the
//! midsegment it crosses, and it is exactly one triangle quarter plus two
//! dodecagon corner pieces of area `(3+√3)/16`. And the 12 chords cut every
//! dodecagon into the rosette core, twelve copies each of seeds 2, 4 and 5,
//! and twelve half-pentagon corner pieces — `6 + 3√3`, the dodecagon's own
//! area, exactly.
//!
//! ## Exact over Q(√3)
//!
//! Every vertex of the table is a `(p + q√3)/24` surd (rows store numerator
//! quadruples `[p, q, r, s]`, vertex = `((p+q√3)/24, (r+s√3)/24)`); 24 is the
//! exact common denominator the spike's denominator survey returns. Nothing
//! here depends on decimal approximation: the table was derived
//! programmatically from an exact Q(√3) Python spike (`VERDICT: ALL GOOD`)
//! and re-verified there — Σ areas = `6 + 7√3/2` as a field identity, every
//! interior edge shared exactly twice, 2500 fine-grid probes each landing in
//! exactly one face, and a corner census of only 30°, 60°, 90°, 120° and
//! 150° (1176/2058/2232/1878/1176 corners) that sums to 360° at every
//! interior vertex.
//!
//! ## Symmetry — verified exactly p6m
//!
//! Positive probes (each maps every table face onto a table face plus an
//! exact lattice translation): rot60, rot120 and rot180 about the dodecagon
//! centre `O`, rot120 about the substrate triangle centre `(1 + 2√3/3, 0)`,
//! rot180 about `M_1`, the midpoint of a dodecagon–dodecagon edge, and the
//! mirrors `y = 0`, `x = 0` and the 30° line through `O`. Negative probes
//! (each must fail — that is what pins the group to exactly p6m and not to a
//! supergroup): rot30 and rot90 about `O`, rot60 about the triangle centre,
//! rot180 about `M_0` (a dodecagon–triangle midpoint), rot180 about a
//! substrate vertex, and the mirrors `y = √3/2` and `x = 1`. The partition is
//! un-interlaced (over/under strand crossing is a rendering concern), which
//! is what keeps the mirrors and the full p6m holonomy exact.
//!
//! No image, PDF or other visual material is ingested; the construction is
//! derived from published mathematics (Hankin's PIC family and the standard
//! `{12/4}` twelvefold rosette construction) with prose fair-use reference
//! only.
use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// √3/2 — the same apothem constant the truncated-hexagonal substrate module
/// (`archimedean`) uses, so the two tilings agree bit for bit.
const H: f64 = 0.866_025_403_784_438_6;
/// √3, exactly `2·H` (doubling is exact in binary floating point).
/// `std::f64::consts::SQRT_3` is unstable at this crate's MSRV.
const RT3: f64 = 2.0 * H;

/// Cell constructor: each vertex is `(p + q√3)/24` (integer numerators,
/// verified exact for the whole 45-face cell).
fn cell(seed: u32, g: &[[f64; 4]]) -> Cell {
    Cell {
        seed,
        poly: Polygon::from_slice(
            &g.iter()
                .map(|q| Vec2::new((q[0] + q[1] * RT3) / 24.0, (q[2] + q[3] * RT3) / 24.0))
                .collect::<Vec<_>>(),
        ),
    }
}

/// Hankin PIC twelvefold khatam (3.12.12 substrate, δ = 60°).
pub fn pic_3_12_12() -> LatticeTiling {
    let cells = vec![
        // seed 0 — twelvefold rosette core, regular dodecagon, centroid (0, 0)
        cell(0, &[
            [-6.0, 0.0, 12.0, 6.0],
            [-6.0, -6.0, 6.0, 6.0],
            [-12.0, -6.0, 6.0, 0.0],
            [-12.0, -6.0, -6.0, 0.0],
            [-6.0, -6.0, -6.0, -6.0],
            [-6.0, 0.0, -12.0, -6.0],
            [6.0, 0.0, -12.0, -6.0],
            [6.0, 6.0, -6.0, -6.0],
            [12.0, 6.0, -6.0, 0.0],
            [12.0, 6.0, 6.0, 0.0],
            [6.0, 6.0, 6.0, 6.0],
            [6.0, 0.0, 12.0, 6.0],
        ]),
        // seed 1 — vertex pentagon (wraps a 3.12.12 vertex), centroid ((-39+10√3)/36, (22-5√3)/6)
        cell(1, &[
            [-12.0, -6.0, 30.0, 12.0],
            [-12.0, -6.0, 18.0, 12.0],
            [-3.0, -3.0, 15.0, 9.0],
            [0.0, 0.0, 24.0, 12.0],
            [-3.0, -3.0, 33.0, 15.0],
        ]),
        // seed 1 — vertex pentagon (wraps a 3.12.12 vertex), centroid ((261-77√3)/72, (67-16√3)/24)
        cell(1, &[
            [12.0, 12.0, 24.0, 12.0],
            [12.0, 6.0, 18.0, 12.0],
            [12.0, 6.0, 12.0, 6.0],
            [18.0, 12.0, 12.0, 6.0],
            [21.0, 15.0, 21.0, 9.0],
        ]),
        // seed 1 — vertex pentagon (wraps a 3.12.12 vertex), centroid ((261-77√3)/72, (-67+16√3)/24)
        cell(1, &[
            [18.0, 12.0, -12.0, -6.0],
            [12.0, 6.0, -12.0, -6.0],
            [12.0, 6.0, -18.0, -12.0],
            [12.0, 12.0, -24.0, -12.0],
            [21.0, 15.0, -21.0, -9.0],
        ]),
        // seed 1 — vertex pentagon (wraps a 3.12.12 vertex), centroid ((39-10√3)/36, (-22+5√3)/6)
        cell(1, &[
            [3.0, 3.0, -15.0, -9.0],
            [0.0, 0.0, -24.0, -12.0],
            [3.0, 3.0, -33.0, -15.0],
            [12.0, 6.0, -30.0, -12.0],
            [12.0, 6.0, -18.0, -12.0],
        ]),
        // seed 1 — vertex pentagon (wraps a 3.12.12 vertex), centroid ((-261+77√3)/72, (-67+16√3)/24)
        cell(1, &[
            [-12.0, -6.0, -12.0, -6.0],
            [-18.0, -12.0, -12.0, -6.0],
            [-21.0, -15.0, -21.0, -9.0],
            [-12.0, -12.0, -24.0, -12.0],
            [-12.0, -6.0, -18.0, -12.0],
        ]),
        // seed 1 — vertex pentagon (wraps a 3.12.12 vertex), centroid ((-261+77√3)/72, (67-16√3)/24)
        cell(1, &[
            [-12.0, -6.0, 18.0, 12.0],
            [-12.0, -12.0, 24.0, 12.0],
            [-21.0, -15.0, 21.0, 9.0],
            [-18.0, -12.0, 12.0, 6.0],
            [-12.0, -6.0, 12.0, 6.0],
        ]),
        // seed 2 — rosette band quadrilateral, centroid ((-79+89√3)/48, (-105+133√3)/144)
        cell(2, &[
            [15.0, 9.0, 3.0, 3.0],
            [18.0, 12.0, 12.0, 6.0],
            [12.0, 6.0, 12.0, 6.0],
            [12.0, 6.0, 6.0, 4.0],
        ]),
        // seed 2 — rosette band quadrilateral, centroid (0, (105-133√3)/72)
        cell(2, &[
            [-3.0, -3.0, -15.0, -9.0],
            [0.0, 0.0, -24.0, -12.0],
            [3.0, 3.0, -15.0, -9.0],
            [0.0, 0.0, -12.0, -8.0],
        ]),
        // seed 2 — rosette band quadrilateral, centroid ((105-133√3)/72, 0)
        cell(2, &[
            [-15.0, -9.0, -3.0, -3.0],
            [-12.0, -8.0, 0.0, 0.0],
            [-15.0, -9.0, 3.0, 3.0],
            [-24.0, -12.0, 0.0, 0.0],
        ]),
        // seed 2 — rosette band quadrilateral, centroid ((-105+133√3)/144, (-79+89√3)/48)
        cell(2, &[
            [12.0, 6.0, 18.0, 12.0],
            [3.0, 3.0, 15.0, 9.0],
            [6.0, 4.0, 12.0, 6.0],
            [12.0, 6.0, 12.0, 6.0],
        ]),
        // seed 2 — rosette band quadrilateral, centroid (0, (-105+133√3)/72)
        cell(2, &[
            [3.0, 3.0, 15.0, 9.0],
            [0.0, 0.0, 24.0, 12.0],
            [-3.0, -3.0, 15.0, 9.0],
            [0.0, 0.0, 12.0, 8.0],
        ]),
        // seed 2 — rosette band quadrilateral, centroid ((-105+133√3)/72, 0)
        cell(2, &[
            [24.0, 12.0, 0.0, 0.0],
            [15.0, 9.0, 3.0, 3.0],
            [12.0, 8.0, 0.0, 0.0],
            [15.0, 9.0, -3.0, -3.0],
        ]),
        // seed 2 — rosette band quadrilateral, centroid ((-79+89√3)/48, (105-133√3)/144)
        cell(2, &[
            [12.0, 6.0, -6.0, -4.0],
            [12.0, 6.0, -12.0, -6.0],
            [18.0, 12.0, -12.0, -6.0],
            [15.0, 9.0, -3.0, -3.0],
        ]),
        // seed 2 — rosette band quadrilateral, centroid ((79-89√3)/48, (105-133√3)/144)
        cell(2, &[
            [-12.0, -6.0, -6.0, -4.0],
            [-15.0, -9.0, -3.0, -3.0],
            [-18.0, -12.0, -12.0, -6.0],
            [-12.0, -6.0, -12.0, -6.0],
        ]),
        // seed 2 — rosette band quadrilateral, centroid ((79-89√3)/48, (-105+133√3)/144)
        cell(2, &[
            [-12.0, -6.0, 12.0, 6.0],
            [-18.0, -12.0, 12.0, 6.0],
            [-15.0, -9.0, 3.0, 3.0],
            [-12.0, -6.0, 6.0, 4.0],
        ]),
        // seed 2 — rosette band quadrilateral, centroid ((105-133√3)/144, (-79+89√3)/48)
        cell(2, &[
            [-12.0, -6.0, 12.0, 6.0],
            [-6.0, -4.0, 12.0, 6.0],
            [-3.0, -3.0, 15.0, 9.0],
            [-12.0, -6.0, 18.0, 12.0],
        ]),
        // seed 2 — rosette band quadrilateral, centroid ((105-133√3)/144, (79-89√3)/48)
        cell(2, &[
            [-6.0, -4.0, -12.0, -6.0],
            [-12.0, -6.0, -12.0, -6.0],
            [-12.0, -6.0, -18.0, -12.0],
            [-3.0, -3.0, -15.0, -9.0],
        ]),
        // seed 2 — rosette band quadrilateral, centroid ((-105+133√3)/144, (79-89√3)/48)
        cell(2, &[
            [12.0, 6.0, -18.0, -12.0],
            [12.0, 6.0, -12.0, -6.0],
            [6.0, 4.0, -12.0, -6.0],
            [3.0, 3.0, -15.0, -9.0],
        ]),
        // seed 3 — midsegment medial triangle, centroid ((-3-4√3)/3, 0)
        cell(3, &[[-24.0, -18.0, 6.0, 0.0], [-24.0, -18.0, -6.0, 0.0], [-24.0, -12.0, 0.0, 0.0]]),
        // seed 3 — midsegment medial triangle, centroid ((3+4√3)/3, 0)
        cell(3, &[[24.0, 18.0, -6.0, 0.0], [24.0, 18.0, 6.0, 0.0], [24.0, 12.0, 0.0, 0.0]]),
        // seed 4 — inner quadrilateral, centroid ((-87-23√3)/144, (87+23√3)/144)
        cell(4, &[
            [-12.0, -6.0, 6.0, 4.0],
            [-6.0, -6.0, 6.0, 6.0],
            [-6.0, -4.0, 12.0, 6.0],
            [-12.0, -6.0, 12.0, 6.0],
        ]),
        // seed 4 — inner quadrilateral, centroid ((-87-23√3)/144, (-87-23√3)/144)
        cell(4, &[
            [-12.0, -6.0, -6.0, -4.0],
            [-12.0, -6.0, -12.0, -6.0],
            [-6.0, -4.0, -12.0, -6.0],
            [-6.0, -6.0, -6.0, -6.0],
        ]),
        // seed 4 — inner quadrilateral, centroid ((75-7√3)/144, (81+8√3)/72)
        cell(4, &[
            [0.0, 0.0, 12.0, 8.0],
            [6.0, 0.0, 12.0, 6.0],
            [6.0, 4.0, 12.0, 6.0],
            [3.0, 3.0, 15.0, 9.0],
        ]),
        // seed 4 — inner quadrilateral, centroid ((-75+7√3)/144, (81+8√3)/72)
        cell(4, &[
            [0.0, 0.0, 12.0, 8.0],
            [-3.0, -3.0, 15.0, 9.0],
            [-6.0, -4.0, 12.0, 6.0],
            [-6.0, 0.0, 12.0, 6.0],
        ]),
        // seed 4 — inner quadrilateral, centroid ((75-7√3)/144, (-81-8√3)/72)
        cell(4, &[
            [6.0, 4.0, -12.0, -6.0],
            [6.0, 0.0, -12.0, -6.0],
            [0.0, 0.0, -12.0, -8.0],
            [3.0, 3.0, -15.0, -9.0],
        ]),
        // seed 4 — inner quadrilateral, centroid ((87+23√3)/144, (-87-23√3)/144)
        cell(4, &[
            [12.0, 6.0, -12.0, -6.0],
            [12.0, 6.0, -6.0, -4.0],
            [6.0, 6.0, -6.0, -6.0],
            [6.0, 4.0, -12.0, -6.0],
        ]),
        // seed 4 — inner quadrilateral, centroid ((-75+7√3)/144, (-81-8√3)/72)
        cell(4, &[
            [-6.0, -4.0, -12.0, -6.0],
            [-3.0, -3.0, -15.0, -9.0],
            [0.0, 0.0, -12.0, -8.0],
            [-6.0, 0.0, -12.0, -6.0],
        ]),
        // seed 4 — inner quadrilateral, centroid ((-81-8√3)/72, (75-7√3)/144)
        cell(4, &[
            [-12.0, -6.0, 6.0, 0.0],
            [-12.0, -6.0, 6.0, 4.0],
            [-15.0, -9.0, 3.0, 3.0],
            [-12.0, -8.0, 0.0, 0.0],
        ]),
        // seed 4 — inner quadrilateral, centroid ((-81-8√3)/72, (-75+7√3)/144)
        cell(4, &[
            [-12.0, -6.0, -6.0, -4.0],
            [-12.0, -6.0, -6.0, 0.0],
            [-12.0, -8.0, 0.0, 0.0],
            [-15.0, -9.0, -3.0, -3.0],
        ]),
        // seed 4 — inner quadrilateral, centroid ((81+8√3)/72, (-75+7√3)/144)
        cell(4, &[
            [15.0, 9.0, -3.0, -3.0],
            [12.0, 8.0, 0.0, 0.0],
            [12.0, 6.0, -6.0, 0.0],
            [12.0, 6.0, -6.0, -4.0],
        ]),
        // seed 4 — inner quadrilateral, centroid ((87+23√3)/144, (87+23√3)/144)
        cell(4, &[
            [6.0, 6.0, 6.0, 6.0],
            [12.0, 6.0, 6.0, 4.0],
            [12.0, 6.0, 12.0, 6.0],
            [6.0, 4.0, 12.0, 6.0],
        ]),
        // seed 4 — inner quadrilateral, centroid ((81+8√3)/72, (75-7√3)/144)
        cell(4, &[
            [12.0, 8.0, 0.0, 0.0],
            [15.0, 9.0, 3.0, 3.0],
            [12.0, 6.0, 6.0, 4.0],
            [12.0, 6.0, 6.0, 0.0],
        ]),
        // seed 5 — star notch triangle, centroid (0, (-9-10√3)/18)
        cell(5, &[[-6.0, 0.0, -12.0, -6.0], [0.0, 0.0, -12.0, -8.0], [6.0, 0.0, -12.0, -6.0]]),
        // seed 5 — star notch triangle, centroid ((-5-5√3)/12, (9+10√3)/36)
        cell(5, &[[-12.0, -6.0, 6.0, 4.0], [-12.0, -6.0, 6.0, 0.0], [-6.0, -6.0, 6.0, 6.0]]),
        // seed 5 — star notch triangle, centroid (0, (9+10√3)/18)
        cell(5, &[[0.0, 0.0, 12.0, 8.0], [-6.0, 0.0, 12.0, 6.0], [6.0, 0.0, 12.0, 6.0]]),
        // seed 5 — star notch triangle, centroid ((9+10√3)/36, (5+5√3)/12)
        cell(5, &[[6.0, 0.0, 12.0, 6.0], [6.0, 6.0, 6.0, 6.0], [6.0, 4.0, 12.0, 6.0]]),
        // seed 5 — star notch triangle, centroid ((-5-5√3)/12, (-9-10√3)/36)
        cell(5, &[[-12.0, -6.0, -6.0, -4.0], [-6.0, -6.0, -6.0, -6.0], [-12.0, -6.0, -6.0, 0.0]]),
        // seed 5 — star notch triangle, centroid ((-9-10√3)/18, 0)
        cell(5, &[[-12.0, -8.0, 0.0, 0.0], [-12.0, -6.0, -6.0, 0.0], [-12.0, -6.0, 6.0, 0.0]]),
        // seed 5 — star notch triangle, centroid ((9+10√3)/36, (-5-5√3)/12)
        cell(5, &[[6.0, 0.0, -12.0, -6.0], [6.0, 4.0, -12.0, -6.0], [6.0, 6.0, -6.0, -6.0]]),
        // seed 5 — star notch triangle, centroid ((-9-10√3)/36, (5+5√3)/12)
        cell(5, &[[-6.0, -4.0, 12.0, 6.0], [-6.0, -6.0, 6.0, 6.0], [-6.0, 0.0, 12.0, 6.0]]),
        // seed 5 — star notch triangle, centroid ((-9-10√3)/36, (-5-5√3)/12)
        cell(5, &[[-6.0, -4.0, -12.0, -6.0], [-6.0, 0.0, -12.0, -6.0], [-6.0, -6.0, -6.0, -6.0]]),
        // seed 5 — star notch triangle, centroid ((5+5√3)/12, (9+10√3)/36)
        cell(5, &[[12.0, 6.0, 6.0, 4.0], [6.0, 6.0, 6.0, 6.0], [12.0, 6.0, 6.0, 0.0]]),
        // seed 5 — star notch triangle, centroid ((5+5√3)/12, (-9-10√3)/36)
        cell(5, &[[12.0, 6.0, -6.0, -4.0], [12.0, 6.0, -6.0, 0.0], [6.0, 6.0, -6.0, -6.0]]),
        // seed 5 — star notch triangle, centroid ((9+10√3)/18, 0)
        cell(5, &[[12.0, 6.0, 6.0, 0.0], [12.0, 6.0, -6.0, 0.0], [12.0, 8.0, 0.0, 0.0]]),
    ];
    LatticeTiling {
        id: "pic-3-12-12",
        name: "Hankin PIC twelvefold khatam",
        description: "Hankin polygons-in-contact strapwork on the truncated-hexagonal (3.12.12) substrate at contact angle 60°: twelvefold rosette partition into rosette-core dodecagon, vertex pentagon, two quadrilaterals, star notch and medial triangle, exact over Q(√3), 45 faces per lattice cell, wallpaper group p6m.",
        lattice: [Vec2::new(1.5 + RT3, 1.0 + H), Vec2::new(0.0, 2.0 + RT3)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Wallpaper,
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
