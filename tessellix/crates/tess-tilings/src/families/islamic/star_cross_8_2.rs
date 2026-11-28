//! The classical `{8/2}` star-and-cross (A4 "implement both" — the
//! sibling of [`crate::zellige::StarCrossTiling`], which is the `{8/3}`
//! octagram reading).
//!
//! # The tiling
//!
//! Stars are the `{8/2}` compound of two equal squares rotated 45° —
//! the outline is the 16-gon union: eight tips at radius 1/2 on the
//! 45° grid (apex angle 90°, the vertex angle π(8 − 2·2)/8 of the
//! density-2 figure) and eight notches at radius √(2−√2)/2 = 0.382683…
//! between them (notch/tip ratio √(2−√2) = cos(π/4)/cos(π/8) =
//! 0.765367…). The stars sit on the unit square lattice and kiss
//! tip-to-tip at every cell-edge midpoint; each interstice is a
//! concave 16-gon cross with four 90°-pointed arms reaching the four
//! edge midpoints, reflex 270° corners at the four diagonal star tips
//! and convex 135° corners at the eight star notches. One star + one
//! cross per lattice cell; areas (2−√2) + (√2−1) = 1 = |det| exactly.
//!
//! # Exact geometry — and why there is no shape parameter
//!
//! Every vertex is in Q(√2) with denominator 4: with q = √2/4 the star
//! (centered at the lattice point) has tip (1/2, 0) and notch (q, 1/2
//! − q), and the cross (contained in cell [0,1]², centered at the cell
//! center) reaches (1/2, 0), (1, 1/2), (1/2, 1), (0, 1/2). All 32
//! prototile edges have the same length (√2−1)/2, so the partition is
//! edge-to-edge with every star edge glued to exactly one cross edge.
//! Unlike the `{8/3}` card — a measured-ornamental one-parameter
//! family whose decimal data stands under the M35 measurement
//! exception — this card is rigid: the kissing radius 1/2 is *forced*
//! by the edge-to-edge condition (the two squares' sides are the
//! skip-2 tip chords, and the notches are their crossings), so the
//! tiling carries no shape parameter, only the standard
//! scale/rotation similarity controls. Verified by a ±3×±3 lattice
//! patch battery over exact Q(√2) arithmetic (area sum = |det| to
//! exactly 0; interior-edge valence 2; vertex figures
//! 90.90.90.90 / 90.270 / 135.225; 24 336 shifted-irrational coverage
//! samples, 0 mismatches) — see docs/tilings/star-cross-8-2.md.
//!
//! The discriminator against the `{8/3}` sibling is pinned twice
//! below: `star_cross_8_2_star_is_the_two_square_compound` (radical
//! pins + skip-2-on/skip-3-off chord test + 90° apex) and
//! `star_cross_8_2_and_8_3_are_different_figures` (both tilings side
//! by side at the same tip radius).

use std::f64::consts::SQRT_2;

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

/// The `{8/2}` star-and-cross tiling (rigid, p4m, square lattice).
pub struct StarCross82Tiling;

/// √2/4 — the diagonal-tip coordinate and notch offset of the `{8/2}`
/// star at kissing radius 1/2 (exact expression, not a decimal).
const Q: f64 = SQRT_2 / 4.0;

/// 1/2 − √2/4 — the notch offset perpendicular to the tip axis.
const S: f64 = 0.5 - Q;

/// `{8/2}` octagram outline, CCW, centered at the cell origin, tips at
/// radius 1/2: tips at k·45°, notches at 22.5° + k·45° with radius
/// √(2−√2)/2. Vertex pairs per octant: tip (1/2, 0), notch (q, 1/2−q),
/// tip (q, q), notch (1/2−q, q); the rest by the D8 symmetries.
const STAR: [Vec2; 16] = [
    Vec2::new(0.5, 0.0),
    Vec2::new(Q, S),
    Vec2::new(Q, Q),
    Vec2::new(S, Q),
    Vec2::new(0.0, 0.5),
    Vec2::new(-S, Q),
    Vec2::new(-Q, Q),
    Vec2::new(-Q, S),
    Vec2::new(-0.5, 0.0),
    Vec2::new(-Q, -S),
    Vec2::new(-Q, -Q),
    Vec2::new(-S, -Q),
    Vec2::new(0.0, -0.5),
    Vec2::new(S, -Q),
    Vec2::new(Q, -Q),
    Vec2::new(Q, -S),
];

/// The interstice cross contained in cell [0,1]² (centered at the cell
/// center), CCW: four 90°-pointed arms reaching the edge midpoints
/// (1/2, 0), (1, 1/2), (1/2, 1), (0, 1/2); reflex 270° corners at the
/// four diagonal star tips (q, q), (1−q, q), (1−q, 1−q), (q, 1−q);
/// convex 135° corners at the eight star notches. Every edge has
/// length (√2−1)/2 — equal to every star edge, so the pair meets
/// edge-to-edge.
const CROSS: [Vec2; 16] = [
    Vec2::new(0.5, 0.0),
    Vec2::new(1.0 - Q, S),
    Vec2::new(1.0 - Q, Q),
    Vec2::new(0.5 + Q, Q),
    Vec2::new(1.0, 0.5),
    Vec2::new(0.5 + Q, 1.0 - Q),
    Vec2::new(1.0 - Q, 1.0 - Q),
    Vec2::new(1.0 - Q, 0.5 + Q),
    Vec2::new(0.5, 1.0),
    Vec2::new(Q, 0.5 + Q),
    Vec2::new(Q, 1.0 - Q),
    Vec2::new(S, 1.0 - Q),
    Vec2::new(0.0, 0.5),
    Vec2::new(S, Q),
    Vec2::new(Q, Q),
    Vec2::new(Q, S),
];

impl Tiling for StarCross82Tiling {
    fn id(&self) -> &'static str {
        "star-cross-8-2"
    }

    fn name(&self) -> &'static str {
        "Star and Cross ({8/2})"
    }

    fn description(&self) -> &'static str {
        "Classical Islamic star and cross built on the {8/2} octagram — two-square compound stars (90° points, notch/tip √(2−√2)) kissing tip-to-tip, with 90°-pointed concave crosses in every gap; rigid exact geometry, one star + one cross per square-lattice cell (p4m). Vertex figures 90.90.90.90 / 90.270 / 135.225."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Islamic
    }

    fn declared_symmetry(&self) -> Option<SymmetryMeta> {
        Some(SymmetryMeta::new(WallpaperGroup::P4m, LatticeKind::Square))
    }

    fn lattice_basis(&self) -> Option<[Vec2; 2]> {
        Some([Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)])
    }

    fn params(&self) -> &'static [ParamSpec] {
        // Rigid exact geometry: only the similarity controls (scale and
        // rotation). The kissing radius is forced by the edge-to-edge
        // condition — there is no shape knob to sweep.
        &SIMILARITY_PARAMS
    }

    fn default_color_mode(&self) -> ColorMode {
        ColorMode::Pattern
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let map = similarity_map(params);
        let l1 = map.transform(Vec2::new(1.0, 0.0));
        let l2 = map.transform(Vec2::new(0.0, 1.0));
        let meta = SymmetryMeta::new(WallpaperGroup::P4m, LatticeKind::Square)
            .with_notes("rigid exact {8/2} star-and-cross; kissing tips, no shape parameter");
        let fail = |message: &'static str| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: Some([l1, l2]),
            message: Some(message),
        };

        let inv = map.inverse();

        // Region in the tiling's own (cell) coordinates: the axis-aligned
        // box around the inverse image of the four world corners.
        let mut rmin = Vec2::new(f64::MAX, f64::MAX);
        let mut rmax = Vec2::new(f64::MIN, f64::MIN);
        for &corner in &query.region.corners() {
            let q = inv.transform(corner);
            rmin.x = rmin.x.min(q.x);
            rmin.y = rmin.y.min(q.y);
            rmax.x = rmax.x.max(q.x);
            rmax.y = rmax.y.max(q.y);
        }
        let region = BBox::new(rmin, rmax);
        if region.area() <= 1e-12 {
            return fail("empty view");
        }

        // Pre-generation budget (mirrors lattice.rs / zellige.rs): the
        // index range spans ~region_area cells (area 1 each) plus a
        // 1-cell pad, and each cell emits at most two tiles.
        let n_cells = (region.size().x + 3.0).ceil() * (region.size().y + 3.0).ceil();
        if n_cells * 2.0 > MAX_TILES as f64 {
            return fail("too many tiles for this view — zoom in");
        }
        let i0 = rmin.x.floor() as i64 - 1;
        let i1 = rmax.x.ceil() as i64 + 1;
        let j0 = rmin.y.floor() as i64 - 1;
        let j1 = rmax.y.ceil() as i64 + 1;

        let mut tiles = Vec::with_capacity(n_cells as usize * 2);
        for j in j0..=j1 {
            for i in i0..=i1 {
                let tr = Vec2::new(i as f64, j as f64);
                // Cross filling cell [i, i+1] × [j, j+1] (proto/seed 0).
                let cp: Vec<Vec2> = CROSS.iter().map(|p| *p + tr).collect();
                let poly = Polygon::from_slice(&cp).transform(&map);
                if poly.aabb().intersects(query.region) {
                    tiles.push(Tile { family: tess_core::tiling::FamilyTag::None, reflected: false, proto: 0, seed: 0, level: 0, poly });
                }
                // {8/2} star centered on lattice point (i, j) (proto/seed 1).
                let sp: Vec<Vec2> = STAR.iter().map(|p| *p + tr).collect();
                let poly = Polygon::from_slice(&sp).transform(&map);
                if poly.aabb().intersects(query.region) {
                    tiles.push(Tile { family: tess_core::tiling::FamilyTag::None, reflected: false, proto: 1, seed: 1, level: 0, poly });
                }
            }
        }

        if tiles.len() > MAX_TILES {
            return fail("too many tiles for this view — zoom in");
        }

        TilingResult {
            tiles,
            symmetry: meta,
            periodic: Some([l1, l2]),
            message: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{check_covers, check_det, check_rotated};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    fn params() -> ParamSet {
        ParamSet::default_for(StarCross82Tiling.params())
    }

    fn gen(a: Vec2, b: Vec2) -> tess_core::tiling::TilingResult {
        StarCross82Tiling.generate(&params(), &Query { region: BBox::new(a, b) })
    }

    /// Centroid of a polygon's vertices.
    fn centroid(pts: &[Vec2]) -> Vec2 {
        let s: Vec2 = pts.iter().fold(v2(0.0, 0.0), |a, p| a + *p);
        s * (1.0 / pts.len() as f64)
    }

    /// The emitted star centered near `c` and the emitted cross whose
    /// vertices match the prototile frame, both found in a window.
    fn tiles_near(c: Vec2, tol: f64) -> Vec<Vec<Vec2>> {
        let r = gen(v2(-2.0, -2.0), v2(4.0, 4.0));
        assert!(r.message.is_none());
        r.tiles
            .iter()
            .map(|tile| tile.poly.pts.clone())
            .filter(|pts| (centroid(pts) - c).length() < tol)
            .collect()
    }

    /// Interior angle (radians) of a CCW simple polygon at index `i`.
    fn interior_angle(pts: &[Vec2], i: usize) -> f64 {
        let n = pts.len();
        let a = pts[(i + n - 1) % n] - pts[i];
        let b = pts[(i + 1) % n] - pts[i];
        let cross = a.cross(b);
        let phi = (cross.abs().atan2(a.dot(b))).abs();
        // CCW polygon: convex <=> a x b < 0 (turn right from prev→v→next
        // means left turn of the boundary); reflex flips to 2π − phi.
        if cross < 0.0 {
            phi
        } else {
            std::f64::consts::TAU - phi
        }
    }

    /// Perpendicular distance from `q` to the line through `a`,`b`,
    /// reported as infinity when the projection falls off the segment
    /// (same rule as the `{8/3}` discriminator in `zellige_tests`).
    fn line_dist(a: Vec2, b: Vec2, q: Vec2) -> f64 {
        let e = b - a;
        let s = (q - a).dot(e) / e.dot(e);
        if s < -0.01 || s > 1.01 {
            return f64::INFINITY;
        }
        e.cross(q - a).abs() / e.length()
    }

    // ---------------------------------------------------------------
    // The three standard registry checks.
    // ---------------------------------------------------------------

    #[test]
    fn star_cross_8_2_covers_region() {
        check_covers(
            &StarCross82Tiling,
            "{8/2}",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn star_cross_8_2_rotated_scaled() {
        check_rotated(
            &StarCross82Tiling,
            "{8/2}(rot22.5,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            22.5,
        );
    }

    #[test]
    fn star_cross_8_2_deterministic() {
        check_det(
            &StarCross82Tiling,
            "{8/2}",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    // ---------------------------------------------------------------
    // Exact pins (Q(√2) radicals, matching the Python battery).
    // ---------------------------------------------------------------

    /// The emitted star, pinned in radicals: tip radius 1/2; tip
    /// (1/2, 0) and notch (√2/4, 1/2 − √2/4); notch radius² =
    /// (2−√2)/4; notch/tip ratio √(2−√2) = cos(π/4)/cos(π/8) =
    /// 0.7653668647301796; 90° apex at every tip; all 16 edges equal
    /// to (√2−1)/2; area 2−√2.
    #[test]
    fn star_cross_8_2_star_exact_pins() {
        let star = tiles_near(v2(0.0, 0.0), 0.1);
        assert_eq!(star.len(), 1, "exactly one star centred at the origin");
        let star = &star[0];
        assert_eq!(star.len(), 16);

        // Radical coordinate pins (bit-exact to the same f64 expressions).
        let near = |p: Vec2, x: f64, y: f64| (p.x - x).abs() <= 1e-12 && (p.y - y).abs() <= 1e-12;
        assert!(near(star[0], 0.5, 0.0), "tip T0 = (1/2, 0), got {:?}", star[0]);
        assert!(
            near(star[1], Q, 0.5 - Q),
            "notch N0 = (√2/4, 1/2−√2/4), got {:?}",
            star[1]
        );

        let rad = |q: Vec2| q.length();
        let rmax = star.iter().map(|q| rad(*q)).fold(f64::MIN, f64::max);
        let rmin = star.iter().map(|q| rad(*q)).fold(f64::MAX, f64::min);
        assert!(
            (rmax - 0.5).abs() <= 1e-12,
            "tip radius {rmax} != 1/2 (kissing)"
        );
        let rmin2 = rmin * rmin;
        assert!(
            (rmin2 - (2.0 - SQRT_2) / 4.0).abs() <= 1e-15,
            "notch radius² {rmin2} != (2−√2)/4"
        );
        let ratio = rmin / rmax;
        assert!(
            (ratio - (2.0 - SQRT_2).sqrt()).abs() <= 1e-12,
            "notch/tip ratio {ratio} != √(2−√2)"
        );
        // The constant exactly as the sibling card pins it:
        // cos(π/4)/cos(π/8) = 0.7653668647301796.
        assert!(
            (ratio - 0.7653668647301796).abs() <= 1e-12,
            "notch/tip ratio {ratio} != the documented {{8/2}} constant"
        );

        // 90° apex at every tip; every edge length (√2−1)/2.
        let edge = (SQRT_2 - 1.0) / 2.0;
        for i in 0..16 {
            let d = star[(i + 1) % 16] - star[i];
            assert!(
                (d.length() - edge).abs() <= 1e-12,
                "star edge {i} length {} != (√2−1)/2",
                d.length()
            );
        }
        let tips: Vec<usize> = (0..16).filter(|&i| rad(star[i]) > (rmax + rmin) / 2.0).collect();
        assert_eq!(tips.len(), 8, "star must have 8 tips");
        for &i in &tips {
            let a = interior_angle(star, i);
            assert!(
                (a - std::f64::consts::FRAC_PI_2).abs() <= 1e-9,
                "tip apex angle {a} != 90° (the {{8/3}} figure would give 45°)"
            );
        }

        // Area 2 − √2 (star), and one star + one cross per cell sums to
        // |det| = 1 exactly.
        let cross = tiles_near(v2(0.5, 0.5), 0.1);
        assert_eq!(cross.len(), 1, "exactly one cross centred at (1/2, 1/2)");
        let star_area = Polygon::from_slice(star).area();
        let cross_area = Polygon::from_slice(&cross[0]).area();
        assert!(
            (star_area - (2.0 - SQRT_2)).abs() <= 1e-12,
            "star area {star_area} != 2−√2"
        );
        assert!(
            (cross_area - (SQRT_2 - 1.0)).abs() <= 1e-12,
            "cross area {cross_area} != √2−1"
        );
        assert!(
            (star_area + cross_area - 1.0).abs() <= 1e-12,
            "cell content {star_area} + {cross_area} != |det| = 1"
        );
    }

    /// The emitted cross, pinned: concave 16-gon whose arms reach the
    /// four cell-edge midpoints with 90° apexes, reflex 270° corners at
    /// the four diagonal star tips, 135° corners at the eight star
    /// notches, all 16 edges (√2−1)/2.
    #[test]
    fn star_cross_8_2_cross_exact_pins() {
        let cross = tiles_near(v2(0.5, 0.5), 0.1);
        assert_eq!(cross.len(), 1);
        let cross = &cross[0];
        assert_eq!(cross.len(), 16);

        // Arm tips at the four cell-edge midpoints, 90° apex each.
        let arms = [v2(0.5, 0.0), v2(1.0, 0.5), v2(0.5, 1.0), v2(0.0, 0.5)];
        for a in arms {
            let i = cross
                .iter()
                .position(|p| (*p - a).length() <= 1e-12)
                .unwrap_or_else(|| panic!("no cross vertex at arm tip {a}"));
            let ang = interior_angle(cross, i);
            assert!(
                (ang - std::f64::consts::FRAC_PI_2).abs() <= 1e-9,
                "cross arm apex at {a} is {ang}, not 90°"
            );
        }
        // Reflex 270° corners at the four diagonal star tips.
        let diags = [v2(Q, Q), v2(1.0 - Q, Q), v2(1.0 - Q, 1.0 - Q), v2(Q, 1.0 - Q)];
        for d in diags {
            let i = cross
                .iter()
                .position(|p| (*p - d).length() <= 1e-12)
                .unwrap_or_else(|| panic!("no cross vertex at diagonal star tip {d}"));
            let ang = interior_angle(cross, i);
            assert!(
                (ang - 3.0 * std::f64::consts::FRAC_PI_2).abs() <= 1e-9,
                "cross corner at {d} is {ang}, not the reflex 270°"
            );
        }
        // Convex 135° corners at the eight star notches.
        let notches = [
            v2(Q, S),
            v2(S, Q),
            v2(1.0 - S, Q),
            v2(1.0 - Q, S),
            v2(1.0 - Q, 1.0 - S),
            v2(1.0 - S, 1.0 - Q),
            v2(S, 1.0 - Q),
            v2(Q, 1.0 - S),
        ];
        for n in notches {
            let i = cross
                .iter()
                .position(|p| (*p - n).length() <= 1e-12)
                .expect("star notch is a cross vertex (edge-to-edge pairing)");
            let ang = interior_angle(cross, i);
            assert!(
                (ang - 3.0 * std::f64::consts::PI / 4.0).abs() <= 1e-9,
                "cross notch corner at {n} is {ang}, not 135°"
            );
        }
        // Equal edges (√2−1)/2 — matching the star's 16.
        let edge = (SQRT_2 - 1.0) / 2.0;
        for i in 0..16 {
            let d = cross[(i + 1) % 16] - cross[i];
            assert!(
                (d.length() - edge).abs() <= 1e-12,
                "cross edge {i} length {} != (√2−1)/2",
                d.length()
            );
        }
    }

    /// The `{8/2}` reading, pinned structurally (mirrors the `{8/3}`
    /// discriminator in `zellige_tests`): the notches lie EXACTLY on
    /// the skip-2 tip chords — the sides of the two squares — and a
    /// macroscopic distance off every skip-3 chord; the notch/tip ratio
    /// is √(2−√2) = 0.7653668647301796, not the `{8/3}` crossing ratio
    /// √((2−√2)/2) = 0.5411961001461969; the apex is 90°, not the
    /// `{8/3}` vertex angle 45°.
    #[test]
    fn star_cross_8_2_star_is_the_two_square_compound() {
        use std::f64::consts::{FRAC_PI_4, PI};

        let r82: f64 = FRAC_PI_4.cos() / (PI / 8.0).cos(); // 0.765368…
        let r83: f64 = ((2.0 - 2.0f64.sqrt()) / 2.0).sqrt(); // 0.541196…
        assert!(
            (r82 - 0.7653668647301796).abs() <= 1e-12
                && (r83 - 0.5411961001461969).abs() <= 1e-12,
            "the two ratios must be the documented constants"
        );

        let star = tiles_near(v2(0.0, 0.0), 0.1).swap_remove(0);
        let rad = |q: Vec2| q.length();
        let rmax = star.iter().map(|q| rad(*q)).fold(f64::MIN, f64::max);
        let rmin = star.iter().map(|q| rad(*q)).fold(f64::MAX, f64::min);
        let tips: Vec<Vec2> = star
            .iter()
            .copied()
            .filter(|q| rad(*q) > (rmax + rmin) / 2.0)
            .collect();
        let notches: Vec<Vec2> = star
            .iter()
            .copied()
            .filter(|q| rad(*q) <= (rmax + rmin) / 2.0)
            .collect();
        assert_eq!(tips.len(), 8);
        assert_eq!(notches.len(), 8);

        let ratio = rmin / rmax;
        assert!(
            (ratio - r82).abs() <= 1e-12,
            "notch/tip ratio {ratio} is not the {{8/2}} two-square ratio {r82}"
        );
        assert!(
            (ratio - r83).abs() > 0.2,
            "notch/tip ratio {ratio} would be the {{8/3}} crossing ratio"
        );

        for (k, notch) in notches.iter().enumerate() {
            let d2 = (0..8)
                .map(|i| line_dist(tips[i], tips[(i + 2) % 8], *notch))
                .fold(f64::INFINITY, f64::min);
            let d3 = (0..8)
                .map(|i| line_dist(tips[i], tips[(i + 3) % 8], *notch))
                .fold(f64::INFINITY, f64::min);
            assert!(
                d2 <= 1e-12,
                "notch {k} is off every skip-2 chord by {d2} — not the two-square {{8/2}} compound"
            );
            // The binding clearance is chord T_k→T_(k+3): exactly
            // (1/4 − √2/8)/√(1/2 + √2/4) = 0.079256… at tip radius 1/2,
            // i.e. 0.1586·R — macroscopic, where an {8/3} notch would
            // sit ON it (distance ~1e-16). Threshold 0.15·R separates.
            assert!(
                d3 >= 0.15 * rmax,
                "notch {k} sits on a skip-3 chord (d3 = {d3}) — that is {{8/3}}"
            );
        }

        for i in 0..16 {
            if rad(star[i]) > (rmax + rmin) / 2.0 {
                let a = interior_angle(&star, i);
                assert!(
                    (a - PI / 2.0).abs() <= 1e-9 && (a - PI / 4.0).abs() > 0.7,
                    "apex {a} is not the 90° {{8/2}} point (45° would be {{8/3}})"
                );
            }
        }
    }

    /// Side-by-side discrimination: at the same tip radius 1/2, this
    /// card's star and the `{8/3}` card's star have different notch
    /// ratios, chord densities and apex angles — the two registry
    /// cards are genuinely different figures (the point of "implement
    /// both").
    #[test]
    fn star_cross_8_2_and_8_3_are_different_figures() {
        use std::f64::consts::FRAC_PI_2;

        use super::super::zellige;
        use tess_core::params::ParamValue;

        // {8/3} card at its kissing default.
        let z = zellige::StarCrossTiling;
        let mut p = ParamSet::default_for(z.params());
        p.set(zellige::STAR_RADIUS_ID, ParamValue::Float(0.5));
        let r = z.generate(&p, &Query { region: BBox::new(v2(-1.5, -1.5), v2(2.5, 2.5)) });
        let star83 = r
            .tiles
            .iter()
            .map(|t| &t.poly.pts)
            .filter(|pts| pts.len() == 16)
            .min_by(|a, b| centroid(a).length().total_cmp(&centroid(b).length()))
            .expect("{8/3} star near the origin")
            .clone();
        let ratio_of = |pts: &[Vec2]| -> f64 {
            let rmax = pts.iter().map(|q| q.length()).fold(f64::MIN, f64::max);
            let rmin = pts.iter().map(|q| q.length()).fold(f64::MAX, f64::min);
            rmin / rmax
        };
        let ratio_83 = ratio_of(&star83);
        let ratio_82 = ratio_of(&tiles_near(v2(0.0, 0.0), 0.1).swap_remove(0));

        assert!(
            (ratio_83 - 0.5411961001461969).abs() <= 1e-12,
            "sibling {{8/3}} ratio drifted: {ratio_83}"
        );
        assert!(
            (ratio_82 - 0.7653668647301796).abs() <= 1e-12,
            "{{8/2}} ratio wrong: {ratio_82}"
        );
        assert!(
            (ratio_82 - ratio_83).abs() > 0.2,
            "the two cards collapsed onto one figure"
        );

        // Apex: 90° here, 45° there.
        let star82 = tiles_near(v2(0.0, 0.0), 0.1).swap_remove(0);
        let apex_at_max = |pts: &[Vec2]| -> f64 {
            let rmax = pts.iter().map(|q| q.length()).fold(f64::MIN, f64::max);
            let rmin = pts.iter().map(|q| q.length()).fold(f64::MAX, f64::min);
            let i = (0..pts.len())
                .filter(|&i| pts[i].length() > (rmax + rmin) / 2.0)
                .max_by(|&i, &j| pts[i].x.total_cmp(&pts[j].x))
                .unwrap();
            interior_angle(pts, i)
        };
        let a82 = apex_at_max(&star82);
        let a83 = apex_at_max(&star83);
        assert!(
            (a82 - FRAC_PI_2).abs() <= 1e-9,
            "{{8/2}} apex {a82} != 90°"
        );
        assert!(
            (a83 - std::f64::consts::PI / 4.0).abs() <= 1e-9,
            "{{8/3}} apex {a83} != 45°"
        );
    }
}
