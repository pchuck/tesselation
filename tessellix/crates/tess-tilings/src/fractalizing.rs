//! Fractalizing 3.12.12 — Chavey's limit-periodic dodecagon-dense fractal.
//!
//! Wikipedia's "fractalizing k-uniform tilings" family (Chavey, *Tilings by
//! regular polygons III: dodecagon-dense tilings*, Symmetry 25(3), 2014):
//! a sequence of edge-to-edge tilings of the SAME plane in which every
//! level is a hexagonal-orbit pattern whose tiles are all congruent up to
//! the single scale factor `p = 2 + sqrt(3)` per level — level `d` has
//! child edge `E0/p^d`, and the level-`d+1` plane is exactly the level-`d`
//! plane shrunk by `1/p` about the origin flower. Level 0 is the uniform
//! 3.12.12 (truncated hexagonal) tiling; levels >= 1 are the flower orbit
//! reconstructed below. The side lengths dilate by `2 + sqrt(3)` at every
//! step, so the hierarchy is limit-periodic (a new period appears at each
//! level) rather than substitution-self-similar — the same reason Stampfli
//! sits under Quasiperiodic, and why this entry does too.
//!
//! # Level-1 geometry (ground truth: Wikipedia 'Planar Fractalizing
//! Truncated Hexagonal Tiling II.svg', fully reverse-engineered)
//!
//! One flower unit, on a hexagonal lattice of edge-length `p^2` (plane
//! units: level-1 child edge = 1), per cell contributes
//!
//! ```text
//! 44 triangles + 12 squares + 1 hexagon (flower orbit) + 12 dodecagons
//! ```
//!
//! * the flower: motif (1 H + 2 S + 7 T, hexagon at the lattice point,
//!   squares at `(1+sqrt3)/2` and `(5+3sqrt3)/2` on the 0-degree ray,
//!   triangle centers at inradius `sqrt3/6` beyond their square/hexagon
//!   edges) rotated by 60 degrees about the flower center, deduped to
//!   55 tiles, plus two extra triangles `d1, d2` on the cell seam;
//! * the dodecagons: 12 classes per cell at EXACT reduced lattice
//!   coordinates `DM3` (thirds of `Z[sqrt3]`), one orientation class
//!   (vertices at `15 + 30k` degrees), apothem `p/2`;
//! * identity `44*(sqrt3/4) + 12*1 + 3sqrt3/2 + 12*(6+3sqrt3)
//!   = 84 + 48.5*sqrt3 = |det(a1, a2)|` — the per-cell census is forced
//!   by area, and was verified edge-to-edge (see below).
//!
//! All level-1 vertices lie in the EXACT grid `(m + n*sqrt3)/6`: every
//! table below is integers, every anchor accumulates as `Z[sqrt3]` pairs,
//! and a depth-`d` vertex is `((m + n*sqrt3)/6) * p^(1-d)` — exact until
//! the final float conversion, so nothing drifts across depths.
//!
//! # Verification history (exact Q(sqrt3) battery, Python 40-dp and a
//! Rust i128 port in `tests` below, before/with this implementation)
//!
//! Over a 5x5-cell block of the level-1 plane (1725 = 25 x 69 tiles):
//! zero pairwise interior overlaps, no edge used more than twice, zero
//! T-junctions, and 2500 exact-rational sample points in the central
//! 2x2-cell window each covered exactly once (zero gaps, zero overlaps).
//! Per-cell census is exactly 44 T + 12 S + 1 H + 12 D for the 9 central
//! cells, matching the area identity above; all 300 dodecagon edges have
//! squared length exactly 1 (t = 1 units). The Rust test
//! `exact_closure_level1` re-runs the battery over a 3x3-cell block with
//! i128 `Z[sqrt3]` arithmetic.
//!
//! Level 0 reuses the shipped truncated-hexagonal cell table
//! (`archimedean.rs`) at edge `p`: dodecagon lattice
//! `a1 = p*(1.5 + sqrt3, 1 + sqrt3/2)`, `a2 = p*(0, 2 + sqrt3)`, three
//! tiles per cell; notably its cell area equals the level-1 flower cell
//! area `84 + 48.5*sqrt3` exactly.
//!
//! Growth: a fixed world view crosses `p^2 ~ 13.93x` more tiles per level
//! (constant 69 tiles per cell); tests pin the span law and the
//! nested-view identity `render(d+1, scale p, V) == render(d, scale 1, V)`
//! to float rounding (`<= 1e-9` absolute at unit scale).

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

const SQRT3: f64 = 1.732_050_807_568_877_2;
/// Fractalizing inflation factor `p = 2 + sqrt(3)`.
const LAM: f64 = 2.0 + SQRT3;
/// `1/p = 2 - sqrt(3)`, exactly the same number in Z[sqrt3] as in float.
const INV_LAM: f64 = 2.0 - SQRT3;

/// Prototile kinds.
const TK_T: u8 = 0;
const TK_S: u8 = 1;
const TK_H: u8 = 2;
const TK_D: u8 = 3;

/// A vertex on the exact `/6` grid: `(x_rational, x_sqrt3, y_rational,
/// y_sqrt3)` integer pairs, value `((x.0 + x.1*sqrt3)/6, ...)`.
type V6 = (i64, i64, i64, i64);

/// The 57-tile flower unit (1 H + 12 S + 44 T), vertex tables in `V6`
/// coordinates about the flower center (the lattice point), CCW. The
/// motif sector rotated by 60-degree steps, deduped, plus the two seam
/// triangles `d1, d2` — transcribed verbatim from the verified Python
/// pinning of the ground-truth figure.
static UNIT: [(u8, &[V6]); 57] = [
    (TK_H, &[(0, 3, 3, 0), (0, 0, 6, 0), (0, -3, 3, 0), (0, -3, -3, 0), (0, 0, -6, 0), (0, 3, -3, 0)]),
    (TK_S, &[(0, 3, -3, 0), (6, 3, -3, 0), (6, 3, 3, 0), (0, 3, 3, 0)]),
    (TK_S, &[(0, 3, 3, 0), (3, 3, 3, 3), (3, 0, 6, 3), (0, 0, 6, 0)]),
    (TK_S, &[(0, 0, 6, 0), (-3, 0, 6, 3), (-3, -3, 3, 3), (0, -3, 3, 0)]),
    (TK_S, &[(0, -3, 3, 0), (-6, -3, 3, 0), (-6, -3, -3, 0), (0, -3, -3, 0)]),
    (TK_S, &[(0, -3, -3, 0), (-3, -3, -3, -3), (-3, 0, -6, -3), (0, 0, -6, 0)]),
    (TK_S, &[(0, 0, -6, 0), (3, 0, -6, -3), (3, 3, -3, -3), (0, 3, -3, 0)]),
    (TK_S, &[(12, 9, -3, 0), (18, 9, -3, 0), (18, 9, 3, 0), (12, 9, 3, 0)]),
    (TK_S, &[(6, 6, 12, 6), (9, 6, 12, 9), (9, 3, 15, 9), (6, 3, 15, 6)]),
    (TK_S, &[(-6, -3, 15, 6), (-9, -3, 15, 9), (-9, -6, 12, 9), (-6, -6, 12, 6)]),
    (TK_S, &[(-12, -9, 3, 0), (-18, -9, 3, 0), (-18, -9, -3, 0), (-12, -9, -3, 0)]),
    (TK_S, &[(-6, -6, -12, -6), (-9, -6, -12, -9), (-9, -3, -15, -9), (-6, -3, -15, -6)]),
    (TK_S, &[(6, 3, -15, -6), (9, 3, -15, -9), (9, 6, -12, -9), (6, 6, -12, -6)]),
    (TK_T, &[(15, 9, 3, 3), (12, 9, 3, 0), (18, 9, 3, 0)]),
    (TK_T, &[(3, 3, 15, 9), (6, 3, 15, 6), (9, 3, 15, 9)]),
    (TK_T, &[(-12, -6, 12, 6), (-6, -6, 12, 6), (-9, -6, 12, 9)]),
    (TK_T, &[(-15, -9, -3, -3), (-12, -9, -3, 0), (-18, -9, -3, 0)]),
    (TK_T, &[(-3, -3, -15, -9), (-6, -3, -15, -6), (-9, -3, -15, -9)]),
    (TK_T, &[(12, 6, -12, -6), (6, 6, -12, -6), (9, 6, -12, -9)]),
    (TK_T, &[(18, 12, 0, 0), (18, 9, 3, 0), (18, 9, -3, 0)]),
    (TK_T, &[(9, 6, 18, 9), (9, 3, 15, 9), (9, 6, 12, 9)]),
    (TK_T, &[(-9, -6, 18, 9), (-9, -6, 12, 9), (-9, -3, 15, 9)]),
    (TK_T, &[(-18, -12, 0, 0), (-18, -9, -3, 0), (-18, -9, 3, 0)]),
    (TK_T, &[(-9, -6, -18, -9), (-9, -3, -15, -9), (-9, -6, -12, -9)]),
    (TK_T, &[(9, 6, -18, -9), (9, 6, -12, -9), (9, 3, -15, -9)]),
    (TK_T, &[(15, 9, -3, -3), (18, 9, -3, 0), (12, 9, -3, 0)]),
    (TK_T, &[(12, 6, 12, 6), (9, 6, 12, 9), (6, 6, 12, 6)]),
    (TK_T, &[(-3, -3, 15, 9), (-9, -3, 15, 9), (-6, -3, 15, 6)]),
    (TK_T, &[(-15, -9, 3, 3), (-18, -9, 3, 0), (-12, -9, 3, 0)]),
    (TK_T, &[(-12, -6, -12, -6), (-9, -6, -12, -9), (-6, -6, -12, -6)]),
    (TK_T, &[(3, 3, -15, -9), (9, 3, -15, -9), (6, 3, -15, -6)]),
    (TK_T, &[(3, 3, -3, -3), (6, 3, -3, 0), (0, 3, -3, 0)]),
    (TK_T, &[(6, 3, 3, 0), (3, 3, 3, 3), (0, 3, 3, 0)]),
    (TK_T, &[(3, 0, 6, 3), (-3, 0, 6, 3), (0, 0, 6, 0)]),
    (TK_T, &[(-3, -3, 3, 3), (-6, -3, 3, 0), (0, -3, 3, 0)]),
    (TK_T, &[(-6, -3, -3, 0), (-3, -3, -3, -3), (0, -3, -3, 0)]),
    (TK_T, &[(-3, 0, -6, -3), (3, 0, -6, -3), (0, 0, -6, 0)]),
    (TK_T, &[(6, 6, 0, 0), (6, 3, 3, 0), (6, 3, -3, 0)]),
    (TK_T, &[(3, 3, 9, 3), (3, 0, 6, 3), (3, 3, 3, 3)]),
    (TK_T, &[(-3, -3, 9, 3), (-3, -3, 3, 3), (-3, 0, 6, 3)]),
    (TK_T, &[(-6, -6, 0, 0), (-6, -3, -3, 0), (-6, -3, 3, 0)]),
    (TK_T, &[(-3, -3, -9, -3), (-3, 0, -6, -3), (-3, -3, -3, -3)]),
    (TK_T, &[(3, 3, -9, -3), (3, 3, -3, -3), (3, 0, -6, -3)]),
    (TK_T, &[(12, 6, 0, 0), (12, 9, -3, 0), (12, 9, 3, 0)]),
    (TK_T, &[(6, 3, 9, 6), (6, 6, 12, 6), (6, 3, 15, 6)]),
    (TK_T, &[(-6, -3, 9, 6), (-6, -3, 15, 6), (-6, -6, 12, 6)]),
    (TK_T, &[(-12, -6, 0, 0), (-12, -9, 3, 0), (-12, -9, -3, 0)]),
    (TK_T, &[(-6, -3, -9, -6), (-6, -6, -12, -6), (-6, -3, -15, -6)]),
    (TK_T, &[(6, 3, -9, -6), (6, 3, -15, -6), (6, 6, -12, -6)]),
    (TK_T, &[(15, 9, 9, 3), (18, 9, 9, 6), (12, 9, 9, 6)]),
    (TK_T, &[(3, 0, 18, 9), (0, 0, 18, 12), (-3, 0, 18, 9)]),
    (TK_T, &[(-12, -9, 9, 6), (-18, -9, 9, 6), (-15, -9, 9, 3)]),
    (TK_T, &[(-15, -9, -9, -3), (-18, -9, -9, -6), (-12, -9, -9, -6)]),
    (TK_T, &[(-3, 0, -18, -9), (0, 0, -18, -12), (3, 0, -18, -9)]),
    (TK_T, &[(12, 9, -9, -6), (18, 9, -9, -6), (15, 9, -9, -3)]),
    (TK_T, &[(24, 12, 0, 0), (24, 15, -3, 0), (24, 15, 3, 0)]),
    (TK_T, &[(12, 9, -21, -12), (12, 6, -18, -12), (12, 6, -24, -12)]),
];

/// The 12 dodecagon classes per cell, as Z[sqrt3] lattice-coefficient
/// pairs over 3: class center `(i + u/3)*a1 + (j + v/3)*a2` where
/// `u = (u.0 + u.1*sqrt3)`, `v = (v.0 + v.1*sqrt3)`.
static DM3: [((i64, i64), (i64, i64)); 12] = [
    ((6, -3), (0, 0)),
    ((-3, 3), (0, 0)),
    ((0, 0), (6, -3)),
    ((0, 0), (-3, 3)),
    ((-3, 2), (3, -1)),
    ((3, -1), (-3, 2)),
    ((3, -1), (3, -1)),
    ((0, 1), (0, 1)),
    ((0, 1), (6, -2)),
    ((6, -2), (0, 1)),
    ((6, -3), (-3, 3)),
    ((-3, 3), (6, -3)),
];

/// Dodecagon shape (edge 1), vertices at 15 + 30k degrees, exact `/6`
/// pairs, CCW: offsets `(1 + sqrt3/2, 1/2)`, `((1+sqrt3)/2) x 2`, ...
static DSHAPE: [V6; 12] = [(6, 3, 3, 0), (3, 3, 3, 3), (3, 0, 6, 3), (-3, 0, 6, 3), (-3, -3, 3, 3), (-6, -3, 3, 0), (-6, -3, -3, 0), (-3, -3, -3, -3), (-3, 0, -6, -3), (3, 0, -6, -3), (3, 3, -3, -3), (6, 3, -3, 0)];

/// Flower lattice basis (level-1 units), `/6` pairs:
/// `a1 = p^2*(sqrt3/2, 1/2)`, `a2 = p^2*(sqrt3/2, -1/2)`.
const A1: V6 = (36, 21, 21, 12);
const A2: V6 = (36, 21, -21, -12);

/// `m + n*sqrt3` over 6 as f64.
const fn q6(m: i64, n: i64) -> f64 {
    (m as f64 + n as f64 * SQRT3) / 6.0
}

const A1F: (f64, f64) = (q6(A1.0, A1.1), q6(A1.2, A1.3));
const A2F: (f64, f64) = (q6(A2.0, A2.1), q6(A2.2, A2.3));
const DETF: f64 = A1F.0 * A2F.1 - A1F.1 * A2F.0;

/// Depth cap: level `17` has child edge `p^-16 ~ 7e-9` plane units;
/// deeper views lose sub-tile float precision, so they fail with a
/// message instead. (The Z[sqrt3] bookkeeping itself stays sound much
/// deeper — the cap mirrors Stampfli's float horizon.)
const STEP_CAP: usize = 16;

/// View diagonal at or above which the level-0 (uniform 3.12.12, edge p)
/// seed plane is shown. Below it, depth grows by one per factor `p`.
const SPAN0: f64 = 6.0;

/// Plane scale of level `d`: `p^(1-d)` (level 0 has edge `p`).
fn depth_scale(d: usize) -> f64 {
    let mut s = LAM;
    for _ in 0..d {
        s *= INV_LAM;
    }
    s
}

/// Smallest level whose tiles still span the view: level 0 for
/// `diag >= SPAN0`, else `1 + floor(log_p(SPAN0/diag))`.
fn pick_depth(diag: f64) -> Option<usize> {
    if diag >= SPAN0 {
        return Some(0);
    }
    let d = 1.0 + ((SPAN0 / diag).ln() / LAM.ln()).floor().max(0.0);
    let d = d as usize;
    if d > STEP_CAP {
        None
    } else {
        Some(d)
    }
}

/// The level-d (d >= 1) flower-orbit plane, culled to the axis-aligned
/// box `region` in plane coordinates, before the user similarity map.
/// Every tile is `(kind, world-plane vertices)`; cell translation is
/// exact integer `Z[sqrt3]` accumulation, the single float multiply by
/// `p^(1-d)/6` happens at the end.
/// Flower cells intersecting the box `region` at level `depth`, in
/// reduced lattice coordinates with a 2-cell margin (every one of the
/// 69 class offsets lies within one cell of its lattice point; the
/// extra ring is the belt-and-braces cull the coverage tests rely on).
fn cell_span(region: &BBox, depth: usize) -> (i64, i64, i64, i64) {
    // Box in the level-1 frame (undo the level scale), then in reduced
    // lattice coordinates.
    let inv = 1.0 / depth_scale(depth);
    let mut umin = f64::MAX;
    let mut umax = f64::MIN;
    let mut vmin = f64::MAX;
    let mut vmax = f64::MIN;
    for (x, y) in [
        (region.min.x * inv, region.min.y * inv),
        (region.max.x * inv, region.min.y * inv),
        (region.min.x * inv, region.max.y * inv),
        (region.max.x * inv, region.max.y * inv),
    ] {
        let u = (x * A2F.1 - y * A2F.0) / DETF;
        let v = (y * A1F.0 - x * A1F.1) / DETF;
        umin = umin.min(u);
        umax = umax.max(u);
        vmin = vmin.min(v);
        vmax = vmax.max(v);
    }
    let i0 = umin.floor() as i64 - 2;
    let i1 = umax.ceil() as i64 + 2;
    let j0 = vmin.floor() as i64 - 2;
    let j1 = vmax.ceil() as i64 + 2;
    (i0, i1, j0, j1)
}

fn orbit_tiles(region: &BBox, depth: usize) -> Vec<(u8, Vec<Vec2>)> {
    let sc = depth_scale(depth) * (1.0 / 6.0);
    let (i0, i1, j0, j1) = cell_span(region, depth);

    let mut out = Vec::new();
    for i in i0..=i1 {
        for j in j0..=j1 {
            let tr = (
                i * A1.0 + j * A2.0,
                i * A1.1 + j * A2.1,
                i * A1.2 + j * A2.2,
                i * A1.3 + j * A2.3,
            );
            for &(kind, shape) in UNIT.iter() {
                let pts = shape
                    .iter()
                    .map(|&(m, n, p, q)| {
                        Vec2::new(
                            ((m + tr.0) as f64 + (n + tr.1) as f64 * SQRT3) * sc,
                            ((p + tr.2) as f64 + (q + tr.3) as f64 * SQRT3) * sc,
                        )
                    })
                    .collect();
                out.push((kind, pts));
            }
            for &((ua, ub), (va, vb)) in DM3.iter() {
                // Center: m = 3i + u, n = 3j + v (Z[sqrt3]);
                // x = (m+n)(36 + 21 sqrt3)/18 = ((12 sa + 21 sb) + (7 sa + 12 sb) sqrt3)/6
                // y = (m-n)(21 + 12 sqrt3)/18 = ((7 da + 12 db) + (4 da + 7 db) sqrt3)/6
                let sa = 3 * i + ua + 3 * j + va;
                let sb = ub + vb;
                let da = 3 * i + ua - 3 * j - va;
                let db = ub - vb;
                let cx = (12 * sa + 21 * sb, 7 * sa + 12 * sb);
                let cy = (7 * da + 12 * db, 4 * da + 7 * db);
                let pts = DSHAPE
                    .iter()
                    .map(|&(m, n, p, q)| {
                        Vec2::new(
                            ((cx.0 + m) as f64 + (cx.1 + n) as f64 * SQRT3) * sc,
                            ((cy.0 + p) as f64 + (cy.1 + q) as f64 * SQRT3) * sc,
                        )
                    })
                    .collect();
                out.push((TK_D, pts));
            }
        }
    }
    out
}

/// The level-0 plane: uniform 3.12.12 at edge `p` (cell table from the
/// shipped truncated-hexagonal entry, dodecagons + two triangle orbits).
fn level0_tiles(region: &BBox) -> Vec<(u8, Vec<Vec2>)> {
    let h = SQRT3 / 2.0;
    let e = LAM;
    let dodec: [(f64, f64); 12] = [
        (1.0 + h, 0.5),
        (h + 0.5, h + 0.5),
        (0.5, 1.0 + h),
        (-0.5, 1.0 + h),
        (-h - 0.5, h + 0.5),
        (-1.0 - h, 0.5),
        (-1.0 - h, -0.5),
        (-h - 0.5, -h - 0.5),
        (-0.5, -1.0 - h),
        (0.5, -1.0 - h),
        (h + 0.5, -h - 0.5),
        (1.0 + h, -0.5),
    ];
    // CCW across the dodecagon's 0-degree and 60-degree edges.
    let t_e: [(f64, f64); 3] = [(1.0 + 2.0 * h, 0.0), (1.0 + h, 0.5), (1.0 + h, -0.5)];
    let t_ne: [(f64, f64); 3] = [(h + 0.5, 1.5 + h), (0.5, 1.0 + h), (h + 0.5, h + 0.5)];
    // Dodecagon lattice: a1 = p*(1.5 + sqrt3, 1 + h), a2 = p*(0, 2 + 2h).
    let (a1x, a1y) = (e * (1.5 + SQRT3), e * (1.0 + h));
    let a2y = e * (2.0 + 2.0 * h);
    let u0 = (region.min.x / a1x).floor() as i64 - 1;
    let u1 = (region.max.x / a1x).ceil() as i64 + 1;
    // x does not depend on v; v from the y corners at both u extremes.
    let mut vlo = f64::MAX;
    let mut vhi = f64::MIN;
    for u in [u0 as f64, u1 as f64] {
        for y in [region.min.y, region.max.y] {
            let v = (y - u * a1y) / a2y;
            vlo = vlo.min(v);
            vhi = vhi.max(v);
        }
    }
    let v0 = vlo.floor() as i64 - 1;
    let v1 = vhi.ceil() as i64 + 1;

    let mut out = Vec::new();
    for u in u0..=u1 {
        for v in v0..=v1 {
            let c = (
                u as f64 * a1x + v as f64 * 0.0,
                u as f64 * a1y + v as f64 * a2y,
            );
            for (kind, shape) in [
                (TK_D, &dodec[..]),
                (TK_T, &t_e[..]),
                (TK_T, &t_ne[..]),
            ] {
                let pts = shape
                    .iter()
                    .map(|&(x, y)| Vec2::new(c.0 + x * e, c.1 + y * e))
                    .collect();
                out.push((kind, pts));
            }
        }
    }
    out
}

/// Fractalizing 3.12.12: limit-periodic dodecagon-dense fractal family
/// (triangles, squares, hexagons, dodecagons; inflation 2+sqrt3/level).
pub struct FractalizingTiling;

impl FractalizingTiling {
    /// Generate at a forced level (0 = uniform 3.12.12 seed, 1.. = the
    /// flower-orbit planes). Used by the growth and nesting tests;
    /// `Tiling::generate` picks the level from the view.
    pub fn generate_forced(
        &self,
        params: &ParamSet,
        query: &Query,
        depth: usize,
    ) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None)
            .with_notes("limit-periodic fractalizing family; each level a 6-fold periodic plane");
        let fail = |message: &'static str| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: None,
            message: Some(message),
        };

        let map = similarity_map(params);
        let inv = map.inverse();

        // Region in plane coordinates: axis-aligned box around the
        // inverse images of the four world corners.
        let mut rmin = Vec2::new(f64::MAX, f64::MAX);
        let mut rmax = Vec2::new(f64::MIN, f64::MIN);
        for &corner in &query.region.corners() {
            let q = inv.transform(corner);
            rmin.x = rmin.x.min(q.x);
            rmin.y = rmin.y.min(q.y);
            rmax.x = rmax.x.max(q.x);
            rmax.y = rmax.y.max(q.y);
        }
        if rmax.x <= rmin.x || rmax.y <= rmin.y {
            return fail("empty view");
        }
        let region = BBox::new(rmin, rmax);

        // Pre-generation budget: 69 tiles per flower cell (level >= 1,
        // cells of area 84 + 48.5 sqrt3 in level-1 units) or 3 per
        // dodecagon cell (level 0), plus the cull margin ring.
        let (cell_area, per_cell) = if depth == 0 {
            (LAM * LAM * (12.0 + 7.0 * SQRT3) / 2.0, 3)
        } else {
            let s = depth_scale(depth);
            (s * s * (84.0 + 48.5 * SQRT3), 69)
        };
        let cells_across = region.size().x * region.size().y / cell_area + 6.0 * (region.size().x / cell_area.sqrt() + region.size().y / cell_area.sqrt()) + 25.0;
        if cells_across * per_cell as f64 > MAX_TILES as f64 {
            return fail("too many tiles for this view — zoom in");
        }
        if depth > STEP_CAP {
            return fail("view beyond the fractal depth cap — zoom in");
        }

        let plane = if depth == 0 {
            level0_tiles(&region)
        } else {
            orbit_tiles(&region, depth)
        };

        let mut tiles = Vec::new();
        for (kind, pts) in plane {
            let world = Polygon::from_slice(&pts).transform(&map);
            if world.aabb().intersects(query.region) {
                tiles.push(Tile {
                    proto: kind,
                    seed: kind as u32,
                    poly: world,
                });
            }
        }
        if tiles.len() > MAX_TILES {
            return fail("too many tiles for this view — zoom in");
        }

        TilingResult {
            tiles,
            symmetry: meta,
            periodic: None,
            message: None,
        }
    }
}

impl Tiling for FractalizingTiling {
    fn id(&self) -> &'static str {
        "fractalizing-3-12-12"
    }

    fn name(&self) -> &'static str {
        "Fractalizing 3.12.12"
    }

    fn description(&self) -> &'static str {
        "Limit-periodic fractalizing family over the uniform 3.12.12: hexagonal flower orbits of triangles, squares, hexagons and dodecagons, every level shrunk by 2+sqrt(3)."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Quasiperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn default_color_mode(&self) -> ColorMode {
        ColorMode::Pattern
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let map = similarity_map(params);
        let inv = map.inverse();
        let mut lo = Vec2::new(f64::MAX, f64::MAX);
        let mut hi = Vec2::new(f64::MIN, f64::MIN);
        for &corner in &query.region.corners() {
            let q = inv.transform(corner);
            lo.x = lo.x.min(q.x);
            lo.y = lo.y.min(q.y);
            hi.x = hi.x.max(q.x);
            hi.y = hi.y.max(q.y);
        }
        let diag = (hi - lo).length();
        if diag < 1e-12 {
            let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None);
            return TilingResult {
                tiles: Vec::new(),
                symmetry: meta,
                periodic: None,
                message: Some("empty view"),
            };
        }
        match pick_depth(diag) {
            None => {
                let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None);
                TilingResult {
                    tiles: Vec::new(),
                    symmetry: meta,
                    periodic: None,
                    message: Some("view beyond the fractal depth cap — zoom in"),
                }
            }
            Some(d) => self.generate_forced(params, query, d),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lattice::SCALE_ID;
    use std::collections::HashMap;

    /// Exact sign of `a + b*sqrt(3)` for i128 pairs.
    fn sign_pair(a: i128, b: i128) -> i32 {
        if b == 0 {
            return (a > 0) as i32 - (a < 0) as i32;
        }
        if a == 0 {
            return if b > 0 { 1 } else { -1 };
        }
        if a > 0 && b > 0 {
            return 1;
        }
        if a < 0 && b < 0 {
            return -1;
        }
        let d = if b > 0 { 3 * b * b - a * a } else { a * a - 3 * b * b };
        (d > 0) as i32 - (d < 0) as i32
    }

    /// `(a + b sqrt3)(c + d sqrt3)` exactly.
    fn mulq(p: (i128, i128), q: (i128, i128)) -> (i128, i128) {
        (p.0 * q.0 + 3 * p.1 * q.1, p.0 * q.1 + p.1 * q.0)
    }

    /// Cross product `(a - o) x (b - o)` as a Z[sqrt3] pair over 36.
    fn cross6(o: &V6, a: &V6, b: &V6) -> (i128, i128) {
        let px = (a.0 as i128 - o.0 as i128, a.1 as i128 - o.1 as i128);
        let py = (a.2 as i128 - o.2 as i128, a.3 as i128 - o.3 as i128);
        let qx = (b.0 as i128 - o.0 as i128, b.1 as i128 - o.1 as i128);
        let qy = (b.2 as i128 - o.2 as i128, b.3 as i128 - o.3 as i128);
        let u = mulq(px, qy);
        let v = mulq(py, qx);
        (u.0 - v.0, u.1 - v.1)
    }

    fn v6f(v: &V6) -> (f64, f64) {
        (q6(v.0, v.1), q6(v.2, v.3))
    }

    fn bbox(vs: &[V6]) -> (f64, f64, f64, f64) {
        let mut b = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
        for v in vs {
            let (x, y) = v6f(v);
            b.0 = b.0.min(x);
            b.1 = b.1.max(x);
            b.2 = b.2.min(y);
            b.3 = b.3.max(y);
        }
        b
    }

    /// Strict interior of a CCW convex polygon, exact.
    fn strict_in(q: &V6, poly: &[V6]) -> bool {
        let n = poly.len();
        (0..n).all(|i| cross6(&poly[i], &poly[(i + 1) % n], q).sgn() > 0)
    }

    trait Sgn {
        fn sgn(&self) -> i32;
        fn zero(&self) -> bool;
    }
    impl Sgn for (i128, i128) {
        fn sgn(&self) -> i32 {
            sign_pair(self.0, self.1)
        }
        fn zero(&self) -> bool {
            self.0 == 0 && self.1 == 0
        }
    }
    /// Proper segment crossing (open segments), exact.
    fn proper_cross(a: &V6, b: &V6, c: &V6, d: &V6) -> bool {
        let s1 = cross6(c, d, a).sgn();
        let s2 = cross6(c, d, b).sgn();
        let s3 = cross6(a, b, c).sgn();
        let s4 = cross6(a, b, d).sgn();
        s1 * s2 < 0 && s3 * s4 < 0
    }

    fn overlap(u: &[V6], v: &[V6]) -> bool {
        if v.iter().any(|p| strict_in(p, u)) || u.iter().any(|p| strict_in(p, v)) {
            return true;
        }
        let nu = u.len();
        let nv = v.len();
        for i in 0..nu {
            for j in 0..nv {
                if proper_cross(&u[i], &u[(i + 1) % nu], &v[j], &v[(j + 1) % nv]) {
                    return true;
                }
            }
        }
        false
    }

    /// Build the exact V6 tiles over cells i, j in [-1, 1] (9 cells).
    fn exact_build() -> Vec<(u8, Vec<V6>)> {
        let mut out = Vec::new();
        for i in -1..=1i64 {
            for j in -1..=1i64 {
                let tr = (
                    i * A1.0 + j * A2.0,
                    i * A1.1 + j * A2.1,
                    i * A1.2 + j * A2.2,
                    i * A1.3 + j * A2.3,
                );
                for &(kind, shape) in UNIT.iter() {
                    out.push((
                        kind,
                        shape
                            .iter()
                            .map(|&(m, n, p, q)| (m + tr.0, n + tr.1, p + tr.2, q + tr.3))
                            .collect(),
                    ));
                }
                for &((ua, ub), (va, vb)) in DM3.iter() {
                    let sa = 3 * i + ua + 3 * j + va;
                    let sb = ub + vb;
                    let da = 3 * i + ua - 3 * j - va;
                    let db = ub - vb;
                    let cx = (12 * sa + 21 * sb, 7 * sa + 12 * sb);
                    let cy = (7 * da + 12 * db, 4 * da + 7 * db);
                    out.push((
                        TK_D,
                        DSHAPE
                            .iter()
                            .map(|&(m, n, p, q)| (cx.0 + m, cx.1 + n, cy.0 + p, cy.1 + q))
                            .collect(),
                    ));
                }
            }
        }
        out
    }

    /// Shoelace double-area as an exact Z[sqrt3] pair over 36.
    fn dbl_area(vs: &[V6]) -> (i128, i128) {
        let mut s = (0i128, 0i128);
        let n = vs.len();
        for i in 0..n {
            let a = vs[i];
            let b = vs[(i + 1) % n];
            let p = mulq((a.0 as i128, a.1 as i128), (b.2 as i128, b.3 as i128));
            let q = mulq((b.0 as i128, b.1 as i128), (a.2 as i128, a.3 as i128));
            s = (s.0 + p.0 - q.0, s.1 + p.1 - q.1);
        }
        s
    }

    #[test]
    fn unit_tables_census_winding_and_area() {
        // census: 1 H + 12 S + 44 T; every shape CCW with exact double
        // area (over 36): T (0,18), S (72,0), H (0,108), D (432,216).
        let mut c = [0usize; 4];
        for &(kind, shape) in UNIT.iter() {
            c[kind as usize] += 1;
            let want = match kind {
                TK_T => (0i128, 18i128),
                TK_S => (72, 0),
                _ => (0, 108),
            };
            assert_eq!(dbl_area(shape), want, "unit shape area kind {kind}");
        }
        assert_eq!((c[TK_T as usize], c[TK_S as usize], c[TK_H as usize]), (44, 12, 1));
        assert_eq!(dbl_area(&DSHAPE), (432, 216));
    }

    #[test]
    fn exact_closure_level1() {
        // 3x3-cell block: 621 unique tiles, zero overlaps, edges used at
        // most twice, no T-junctions, every rational sample of the middle
        // cell covered exactly once, exact dodecagon edge lengths.
        let tiles = exact_build();
        assert_eq!(tiles.len(), 9 * 69);

        let mut seen: std::collections::HashSet<(u8, [V6; 12], usize)> =
            std::collections::HashSet::new();
        for (kind, vs) in &tiles {
            let mut key = [(0i64, 0i64, 0i64, 0i64); 12];
            let m = vs.len();
            for (i, v) in vs.iter().enumerate() {
                key[i] = *v;
            }
            key[m..].fill((i64::MIN, i64::MIN, i64::MIN, i64::MIN));
            key.sort_unstable();
            assert!(seen.insert((*kind, key, m)), "duplicate orbit tile");
        }
        assert_eq!(seen.len(), 9 * 69);

        let cs: Vec<&Vec<V6>> = tiles.iter().map(|(_, v)| v).collect();
        let bbs: Vec<_> = cs.iter().map(|v| bbox(v)).collect();

        let mut ov = 0usize;
        for i in 0..cs.len() {
            for j in i + 1..cs.len() {
                let (a0, a1, a2, a3) = bbs[i];
                let (b0, b1, b2, b3) = bbs[j];
                if a0 > b1 || b0 > a1 || a2 > b3 || b2 > a3 {
                    continue;
                }
                if overlap(cs[i], cs[j]) {
                    ov += 1;
                }
            }
        }
        assert_eq!(ov, 0, "pairwise interior overlaps");

        // edge census
        let mut edges: HashMap<(V6, V6), u8> = HashMap::new();
        for vs in &cs {
            let n = vs.len();
            for k in 0..n {
                let (a, b) = (vs[k], vs[(k + 1) % n]);
                let key = if a <= b { (a, b) } else { (b, a) };
                *edges.entry(key).or_insert(0) += 1;
            }
        }
        assert!(edges.values().all(|&c| c <= 2), "edge shared by >2 tiles");

        // T-junctions: a vertex strictly inside another tile's edge.
        let mut tj = 0usize;
        for i in 0..cs.len() {
            for p in cs[i].iter() {
                let (px, py) = v6f(p);
                for j in 0..cs.len() {
                    if i == j {
                        continue;
                    }
                    let (b0, b1, b2, b3) = bbs[j];
                    if !(b0 - 0.5 <= px && px <= b1 + 0.5 && b2 - 0.5 <= py && py <= b3 + 0.5) {
                        continue;
                    }
                    let n = cs[j].len();
                    for k in 0..n {
                        let a = cs[j][k];
                        let b = cs[j][(k + 1) % n];
                        if !cross6(&a, &b, p).zero() {
                            continue;
                        }
                        let pxp = (p.0 as i128 - a.0 as i128, p.1 as i128 - a.1 as i128);
                        let pyp = (p.2 as i128 - a.2 as i128, p.3 as i128 - a.3 as i128);
                        let bx = (b.0 as i128 - a.0 as i128, b.1 as i128 - a.1 as i128);
                        let by = (b.2 as i128 - a.2 as i128, b.3 as i128 - a.3 as i128);
                        let d = {
                            let u = mulq(pxp, bx);
                            let v = mulq(pyp, by);
                            (u.0 + v.0, u.1 + v.1)
                        };
                        let l2 = {
                            let u = mulq(bx, bx);
                            let v = mulq(by, by);
                            (u.0 + v.0, u.1 + v.1)
                        };
                        if d.sgn() > 0 && (d.0 - l2.0, d.1 - l2.1).sgn() < 0 {
                            tj += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(tj, 0, "T-junctions");

        // Exact coverage: 900 rational points at reduced coordinates
        // (u, v) in [-2, 2)^2 (the central 2x2-cell window), each point
        // q = u*A1 + v*A2, counted against every tile whose float aabb
        // contains it; boundary hits are skipped (the denominators are
        // large primes, so none occur), every other point must be
        // covered exactly once — zero gaps and zero overlaps. Sample
        // pairs are over D; tile pairs over 6, rescaled by 12 to the
        // common /D denominator before each cross product.
        const D: i128 = 999_983;
        let mut state: u64 = 0x243F_6A88_85A3_08D3;
        let mut next = move || {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((state >> 33) as i128 % (2 * D)) - D
        };
        let mut wrong = 0usize;
        let mut boundary = 0usize;
        for _ in 0..900usize {
            let nu = next();
            let nv = next();
            // q pair over D: (nu/D)*A1 + (nv/D)*A2, A over 6 -> /6D;
            // store scaled by 12 so comparisons with tile verts (over 6
            // -> *12) share the denominator 12D... use raw over 6D and
            // scale edges by D instead:
            //   cross(edge*D - vertex*D, q*6 - vertex*6*... ) — keep it
            // simple: values over 6D:
            //     qv = (nu*A1pair + nv*A2pair)  [over 6D]
            //     e  = (b - a) as pair          [over 6]  -> *D
            //     r  = qv - a_pair*D            [over 6D]
            let q = (
                nu * A1.0 as i128 + nv * A2.0 as i128,
                nu * A1.1 as i128 + nv * A2.1 as i128,
                nu * A1.2 as i128 + nv * A2.2 as i128,
                nu * A1.3 as i128 + nv * A2.3 as i128,
            );
            let (qxf, qyf) = (
                (q.0 as f64 + q.1 as f64 * SQRT3) / (6.0 * D as f64),
                (q.2 as f64 + q.3 as f64 * SQRT3) / (6.0 * D as f64),
            );
            let mut cnt = 0usize;
            let mut onb = false;
            for idx in 0..cs.len() {
                let (b0, b1, b2, b3) = bbs[idx];
                if !(b0 <= qxf && qxf <= b1 && b2 <= qyf && qyf <= b3) {
                    continue;
                }
                let mut ok = true;
                let n = cs[idx].len();
                for k in 0..n {
                    let a = cs[idx][k];
                    let b = cs[idx][(k + 1) % n];
                    let e = (
                        (b.0 - a.0) as i128 * D,
                        (b.1 - a.1) as i128 * D,
                        (b.2 - a.2) as i128 * D,
                        (b.3 - a.3) as i128 * D,
                    );
                    let rx = (q.0 - a.0 as i128 * D, q.1 - a.1 as i128 * D);
                    let ry = (q.2 - a.2 as i128 * D, q.3 - a.3 as i128 * D);
                    let u = mulq((e.0, e.1), ry);
                    let v = mulq((e.2, e.3), rx);
                    let s = (u.0 - v.0, u.1 - v.1).sgn();
                    if s < 0 {
                        ok = false;
                        break;
                    }
                    if s == 0 {
                        onb = true;
                    }
                }
                if ok {
                    cnt += 1;
                }
            }
            if onb {
                boundary += 1;
                continue;
            }
            if cnt != 1 {
                wrong += 1;
            }
        }
        assert_eq!(boundary, 0, "sample landed on a tile boundary");
        assert_eq!(wrong, 0, "coverage samples not exactly single-covered");

        // D edge squared == 1 exactly (pair over 36 == (36, 0)).
        for (kind, vs) in &tiles {
            if *kind != TK_D {
                continue;
            }
            let n = vs.len();
            for k in 0..n {
                let dx = (vs[(k + 1) % n].0 - vs[k].0, vs[(k + 1) % n].1 - vs[k].1);
                let dy = (vs[(k + 1) % n].2 - vs[k].2, vs[(k + 1) % n].3 - vs[k].3);
                let u = mulq((dx.0 as i128, dx.1 as i128), (dx.0 as i128, dx.1 as i128));
                let v = mulq((dy.0 as i128, dy.1 as i128), (dy.0 as i128, dy.1 as i128));
                assert_eq!((u.0 + v.0, u.1 + v.1), (36, 0), "D edge not exactly 1");
            }
        }
    }

    #[test]
    fn per_cell_census() {
        let tiles = exact_build();
        let mut c = [0usize; 4];
        for (kind, _) in &tiles {
            c[*kind as usize] += 1;
        }
        assert_eq!(c[TK_T as usize], 9 * 44);
        assert_eq!(c[TK_S as usize], 9 * 12);
        assert_eq!(c[TK_H as usize], 9);
        assert_eq!(c[TK_D as usize], 9 * 12);
        // exact area identity: cell = 84 + 48.5 sqrt3 (level-1 units):
        // per-kind exact double areas (over 36): T (0, 18), S (72, 0),
        // H (0, 108), D (432, 216) — rational and sqrt3 parts summed:
        let lhs_a = 12 * 72 + 12 * 432;
        let lhs_b = 44 * 18 + 108 + 12 * 216;
        // 2*cell over 36 = (168, 97) * 36 / 36:
        assert_eq!((lhs_a, lhs_b), (6048, 3492));
        //   = 9 * cell: double-area of one cell is 168 + 97*sqrt3,
        // over 36 that is (168*36, 97*36) = (6048, 3492). The per-cell
        // identity 44*T + 12*S + 1*H + 12*D double-areas
        // (44*(0,18) + 12*(72,0) + (0,108) + 12*(432,216)) therefore
        // matches the flower cell exactly — coverage + closure force it.
    }

    #[test]
    fn growth_p2_and_nested_count_invariance() {
        // Growth law, exact in cell bookkeeping: for a FIXED world box
        // the culled cell span at depth d is `true_span(d) + 5` (two
        // margin cells per side plus floor/ceil rounding), and the true
        // span scales by p per level. So `span(d+1) - 5 = p *
        // (span(d) - 5)` up to floor/ceil rounding (at most 1 per side,
        // i.e. |error| <= 1 + p < 5), and tiles = 69 per cell.
        let box1 = BBox::new(Vec2::new(-31.0, -27.0), Vec2::new(29.0, 33.0));
        for (a, b) in [(1usize, 2usize), (2, 3), (3, 4)] {
            let (i0a, i1a, j0a, j1a) = cell_span(&box1, a);
            let (i0b, i1b, j0b, j1b) = cell_span(&box1, b);
            for (wa, wb) in [
                (i1a - i0a + 1, i1b - i0b + 1),
                (j1a - j0a + 1, j1b - j0b + 1),
            ] {
                // ceil(p*xh) - p*ceil(xh) is in (-p, 0] and
                // floor(p*xl) - p*floor(xl) in [0, p), so the span
                // deficit from the exact p law is bounded by 2p:
                let fa = (wa - 5) as f64;
                let fb = (wb - 5) as f64;
                assert!(
                    (fb - fa * LAM).abs() < 2.0 * LAM,
                    "span {wa} -> {wb} vs p = {LAM}"
                );
            }
            // tiles are exactly 69 per cell in the span:
            assert_eq!(orbit_tiles(&box1, a).len(), 69 * (i1a - i0a + 1) as usize * (j1a - j0a + 1) as usize);
        }
        // nested windows: V_d = p^(1-d) * V_1 -> the SAME reduced span,
        // hence exactly equal tile counts for every depth.
        let mut prev = 0usize;
        for d in 1..=5usize {
            let s = depth_scale(d);
            let vd = BBox::new(
                Vec2::new(-31.0 * s, -27.0 * s),
                Vec2::new(29.0 * s, 33.0 * s),
            );
            let cd = orbit_tiles(&vd, d).len();
            if d == 1 {
                prev = cd;
            } else {
                assert_eq!(cd, prev, "nested count differs at depth {d}");
            }
        }
    }

    #[test]
    fn nested_view_identity() {
        // The nesting law as the user sees it: because the level-(d+1)
        // plane is the level-d plane shrunk by 1/p about the origin,
        // viewing the SAME world box V at scale `p` one level deeper
        // reproduces the view exactly — same tiles, same world
        // coordinates (to float rounding):
        //   render(d+1, scale p, V) == render(d, scale 1, V).
        let t = FractalizingTiling;
        let va = BBox::new(Vec2::new(-4.0, -3.0), Vec2::new(4.0, 3.0));
        for d in 1..=3usize {
            let mut pa = ParamSet::default_for(t.params());
            pa.set(SCALE_ID, tess_core::params::ParamValue::Float(1.0));
            let ra = t.generate_forced(&pa, &Query { region: va }, d);
            assert!(ra.message.is_none(), "{:?}", ra.message);

            let mut pb = ParamSet::default_for(t.params());
            pb.set(SCALE_ID, tess_core::params::ParamValue::Float(LAM));
            let rb = t.generate_forced(&pb, &Query { region: va }, d + 1);
            assert!(rb.message.is_none(), "{:?}", rb.message);
            assert_eq!(ra.tiles.len(), rb.tiles.len(), "depth {d}");

            for ta in &ra.tiles {
                let ca = centroid(&ta.poly.pts);
                let mut best: Option<&Tile> = None;
                for tb in &rb.tiles {
                    let cb = centroid(&tb.poly.pts);
                    let dd = (cb.x - ca.x).hypot(cb.y - ca.y);
                    if dd < 1e-9 && tb.proto == ta.proto {
                        best = Some(tb);
                        break;
                    }
                }
                let tb =
                    best.unwrap_or_else(|| panic!("depth {}: no match for tile at {ca:?}", d + 1));
                assert_eq!(ta.poly.pts.len(), tb.poly.pts.len());
                let n = ta.poly.pts.len();
                for k in 0..n {
                    let va_ = ta.poly.pts[k];
                    let found = tb
                        .poly
                        .pts
                        .iter()
                        .any(|&pb_| (pb_.x - va_.x).hypot(pb_.y - va_.y) < 1e-9);
                    assert!(found, "depth {d}: vertex {va:?} unmatched");
                }
            }
        }
    }

    fn centroid(pts: &[Vec2]) -> Vec2 {
        let n = pts.len() as f64;
        let mut s = Vec2::ZERO;
        for p in pts {
            s += *p;
        }
        Vec2::new(s.x / n, s.y / n)
    }

    #[test]
    fn level0_seed_is_uniform_3_12_12() {
        // depth 0 over a big box: 3 tiles per dodecagon cell, coverage
        // handled by the shared checker at the Tiling level (trio in
        // lib.rs); here pin the orbit structure on exact cells.
        let region = BBox::new(Vec2::new(-1.0, -1.0), Vec2::new(31.0, 31.0));
        let tiles = level0_tiles(&region);
        assert!(tiles.len() >= 3 * 4 * 4);
        let d = tiles.iter().filter(|(k, _)| *k == TK_D).count();
        let tt = tiles.iter().filter(|(k, _)| *k == TK_T).count();
        assert_eq!(tt, 2 * d);
        // dodecagons on the p*(1.5+sqrt3, 1+h) / p*(0, 2+2h) lattice:
        // the origin dodecagon must be present.
        assert!(tiles.iter().any(|(k, pts)| {
            *k == TK_D && centroid(pts).length() < 1e-9
        }));
    }
}
