//! Girih rhombus tiling — the classical girih rhombus prototile tiling
//! the plane alone, as a [`LatticeTiling`] data table.
//!
//! * `72.72.108.108` — Girih Rhombus (cmm)
//!
//! The girih *rhombus* is the thin equilateral rhombus among the five
//! classical girih tiles (Wikipedia "Girih tile"): all four edges are
//! unit-length and the interior angles are 72°, 108°, 72°, 108°. It is
//! centrally symmetric and convex, and it tessellates the plane by pure
//! translation.
//!
//! The lattice basis is the pair of edge directions `U(0°)` and
//! `U(72°)` of the 36° girih frame — adjacent edges of the rhombus
//! themselves: `|det(U0, U2)| = sin 72°` equals the rhombus area
//! `(1)(1)sin 72°` exactly over Q(√5), one tile per cell.
//!
//! Both diagonals of the rhombus are mirror axes of the tile (dihedral
//! group D2), and they map to the diagonal directions `U0 + U2` and
//! `U0 − U2` of the lattice, which are exactly orthogonal
//! `((U0+U2)·(U0−U2) = |U0|² − |U2|² = 0)`. In the rectangular diagonal
//! basis the lattice is centered rectangular (the rhombus cell is half
//! the rectangle), the two rectangle axes are mirrors, and the 2-fold
//! centers at the edge midpoints are off the mirrors: wallpaper group
//! cmm. Every interior vertex figure is 72°/72°/108°/108°.
//!
//! # Seeds
//!
//! One tile per cell — a single orbit, so there is no orbit-distinct
//! coloring to ship. The shipped seed is decorative:
//! `SeedPattern::Checker` colors lattice sites by the parity of `(i + j)`
//! on the centered-rectangular basis.

use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

// Radical constants (exact double values; f64::sqrt is not const-stable).
const SQ5: f64 = 2.236_067_977_499_79; // √5
const C72: f64 = (SQ5 - 1.0) / 4.0; // cos 72° = (√5−1)/4
const S72: f64 = 0.951_056_516_295_153_5; // sin 72° = √(10+2√5)/4

/// Girih rhombus (72°/108°) lattice tiling, wallpaper group cmm.
///
/// Single-tile centered-rectangular lattice tiling. The rhombus (unit
/// edges, angles 72°,108°,72°,108°) tiles by translation on
/// `span{U(0°), U(72°)}`; `det = sin 72° = area(tile)` exactly. Vertex
/// figure 72.72.108.108 at every interior vertex.
pub fn girih_rhombus() -> LatticeTiling {
    // Rhombus, CCW: edges U(0°), U(72°), U(180°), U(252°). Interior
    // angle 72° at v0 and v2, 108° at v1 and v3.
    let rhomb = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0 + C72, S72),
        Vec2::new(C72, S72),
    ]);

    LatticeTiling {
        id: "girih-rhombus",
        name: "Girih Rhombus (72.72.108.108)",
        description: "Periodic girih tiling by the rhombus prototile alone — the thin equilateral girih rhombus (72°,108°,72°,108°, unit edges). Single-tile centered rectangular lattice (cmm) spanned by two adjacent edge directions; vertex figure 72.72.108.108. One of the classical girih prototile tessellations.",
        lattice: [
            Vec2::new(1.0, 0.0), // U(0°)
            Vec2::new(C72, S72), // U(72°)
        ],
        cells: vec![Cell { poly: rhomb, seed: 0 }],
        offset: Vec2::ZERO,
        group: WallpaperGroup::Cmm,
        lattice_kind: LatticeKind::CenteredRectangular,
        category: TilingCategory::Islamic,
        secondary: &[],
        seed_pattern: SeedPattern::Checker,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::geometry::BBox;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn girih_rhombus_covers_region() {
        check_covers(
            &girih_rhombus(),
            "girih-rhombus",
            BBox::new(v2(-8.0, -6.0), v2(16.0, 14.0)),
        );
    }

    #[test]
    fn girih_rhombus_rotated_scaled() {
        check_rotated(
            &girih_rhombus(),
            "girih-rhombus(rot30,scale0.7)",
            BBox::new(v2(-12.0, -12.0), v2(12.0, 12.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn girih_rhombus_deterministic() {
        check_det(
            &girih_rhombus(),
            "girih-rhombus",
            BBox::new(v2(-10.0, -10.0), v2(10.0, 10.0)),
        );
    }

    /// Battery: shoelace area == det to 1e-15; edge parity on a 7x7
    /// lattice patch (pinned to the Python exact-field census: 84
    /// interior + 28 boundary of 4*49 = 196 edge occurrences); vertex
    /// wedges either close to exactly 360 degrees or miss at least one
    /// tile angle; irrational-offset point sampling of the half-open
    /// fundamental parallelogram hits exactly one tile per sample.
    #[test]
    fn girih_rhombus_battery() {
        use std::collections::BTreeMap;

        let t = girih_rhombus();
        let [a1, a2] = t.lattice;
        let tile = &t.cells[0].poly;
        let det = a1.x * a2.y - a1.y * a2.x;

        assert!(
            (poly_area(tile) - det).abs() < 1e-15,
            "rhombus area {} vs det {}",
            poly_area(tile),
            det
        );

        let q = |v: f64| (v * 1e6).round() as i64;
        let key = |p: Vec2| (q(p.x), q(p.y));
        let mut ecnt: BTreeMap<((i64, i64), (i64, i64)), usize> = BTreeMap::new();
        let mut vang: BTreeMap<(i64, i64), f64> = BTreeMap::new();
        let patch = 3i32;
        for i in -patch..=patch {
            for j in -patch..=patch {
                let tr = v2(
                    a1.x * i as f64 + a2.x * j as f64,
                    a1.y * i as f64 + a2.y * j as f64,
                );
                let n = tile.pts.len();
                for k in 0..n {
                    let a = v2(tile.pts[k].x + tr.x, tile.pts[k].y + tr.y);
                    let b = v2(
                        tile.pts[(k + 1) % n].x + tr.x,
                        tile.pts[(k + 1) % n].y + tr.y,
                    );
                    let (ka, kb) = (key(a), key(b));
                    let ek = if ka <= kb { (ka, kb) } else { (kb, ka) };
                    *ecnt.entry(ek).or_insert(0) += 1;
                    *vang.entry(ka).or_insert(0.0) += interior_angle(tile, k);
                }
            }
        }
        assert!(
            ecnt.values().all(|&c| c <= 2),
            "rhombus: edge shared by more than 2 tiles"
        );
        assert_eq!(
            ecnt.values().filter(|&&c| c == 2).count(),
            84,
            "rhombus: interior edge count on the 7x7 patch"
        );
        let tau = std::f64::consts::TAU;
        for (vk, sum) in &vang {
            let closed = (*sum - tau).abs() < 1e-9;
            let boundary = *sum <= tau - 1.2; // at least one 72-degree wedge missing
            assert!(
                closed || boundary,
                "rhombus vertex {vk:?}: wedge sum {sum} neither closed nor boundary"
            );
        }

        let irr = (0.5_f64).sqrt() - 0.5; // irrational, deterministic
        let n = 40usize;
        for si in 0..n {
            for sj in 0..n {
                let al = (si as f64 + irr) / n as f64;
                let be = (sj as f64 + 1.0 - irr) / n as f64;
                let p = v2(a1.x * al + a2.x * be, a1.y * al + a2.y * be);
                let mut cnt = 0;
                for i in -1..=2 {
                    for j in -1..=2 {
                        let r = v2(
                            p.x - (a1.x * i as f64 + a2.x * j as f64),
                            p.y - (a1.y * i as f64 + a2.y * j as f64),
                        );
                        if pip(tile, r) {
                            cnt += 1;
                        }
                    }
                }
                assert_eq!(cnt, 1, "rhombus: cell sample covered {cnt} times");
            }
        }
    }

    fn poly_area(p: &Polygon) -> f64 {
        let mut s = 0.0;
        let n = p.pts.len();
        for i in 0..n {
            let (x0, y0) = (p.pts[i].x, p.pts[i].y);
            let (x1, y1) = (p.pts[(i + 1) % n].x, p.pts[(i + 1) % n].y);
            s += x0 * y1 - y0 * x1;
        }
        s / 2.0
    }

    /// Interior angle of a CCW polygon at vertex k.
    fn interior_angle(p: &Polygon, k: usize) -> f64 {
        let n = p.pts.len();
        let a = p.pts[(k + n - 1) % n];
        let b = p.pts[k];
        let c = p.pts[(k + 1) % n];
        let (ux, uy) = (b.x - a.x, b.y - a.y);
        let (vx, vy) = (c.x - b.x, c.y - b.y);
        let turn = (ux * vy - uy * vx).atan2(ux * vx + uy * vy);
        std::f64::consts::PI - turn
    }

    fn pip(poly: &Polygon, pt: Vec2) -> bool {
        let mut inside = false;
        let n = poly.pts.len();
        for i in 0..n {
            let (x1, y1) = (poly.pts[i].x, poly.pts[i].y);
            let (x2, y2) = (poly.pts[(i + 1) % n].x, poly.pts[(i + 1) % n].y);
            if (y1 > pt.y) != (y2 > pt.y) {
                let xint = x1 + (pt.y - y1) * (x2 - x1) / (y2 - y1);
                if xint > pt.x {
                    inside = !inside;
                }
            }
        }
        inside
    }
}
