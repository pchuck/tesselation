//! Girih bowtie tiling — the classical girih bowtie (papillon) prototile
//! tiling the plane alone, as a [`LatticeTiling`] data table.
//!
//! * `72.72.216` — Girih Bowtie (cmm)
//!
//! The girih *bowtie* is the concave equilateral hexagon among the five
//! classical girih tiles (Wikipedia "Girih tile"): all six edges are
//! unit-length and the interior angles are 216°, 72°, 72°, 216°, 72°, 72°
//! at v0..v5. It is the only non-convex member of the family, and unlike
//! the regular decagon it tessellates the plane on its own.
//!
//! The tile is an affine image of the regular centrally symmetric hexagon
//! `a, b, c, −a, −b, −c` (edge steps U(0°), U(108°), U(216°) on the 36°
//! girih frame) and it is also mirror-symmetric about both of its long
//! diagonals (dihedral group D2). It therefore tiles by pure translation
//! on the centered rectangular lattice `span{t1, t2}` where `t1 = −(b + c)`
//! (glues the two horizontal edges) and `t2 = a − c` (glues the two
//! U(108°)-edges): `det(t1, t2) = area(tile)` exactly, one tile per cell.
//!
//! Every interior vertex figure is 72°/72°/216° — two convex corners of
//! one bowtie paired with the reflex notch of a third. The rectangular
//! basis `{t2, 2t1 − t2}` is exactly orthogonal (`t2 · (2t1 − t2) = 0`),
//! and both rectangle axes are mirror directions inherited from the tile,
//! with 2-fold centers off the mirrors (edge midpoints): the wallpaper
//! group is cmm.
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
const C36: f64 = (1.0 + SQ5) / 4.0; // cos 36° = (1+√5)/4
const C72: f64 = (SQ5 - 1.0) / 4.0; // cos 72° = (√5−1)/4
const S36: f64 = 0.587_785_252_292_473_1; // sin 36° = √(10−2√5)/4
const S72: f64 = 0.951_056_516_295_153_5; // sin 72° = √(10+2√5)/4

/// Girih bowtie (concave hexagon) lattice tiling, wallpaper group cmm.
///
/// Single-tile centered-rectangular lattice tiling. The bowtie (all edges
/// unit-length, angles 216°,72°,72°,216°,72°,72°) tiles by translation on
/// `span{t1, t2}` with `t1 = (cos36°+cos72°, sin36°−sin72°)` and
/// `t2 = (1+cos36°, sin36°)`; `det(t1, t2) = area(tile) ≈ 1.314328`
/// (exact in radicals). Vertex figure 72.72.216 at every interior vertex.
pub fn girih_bowtie() -> LatticeTiling {
    // Bowtie, CCW, unit edges stepping U(0°), U(108°), U(216°),
    // U(180°), U(288°), U(36°) on the 36° girih frame. Reflex (216°)
    // at v0 and v3.
    let bow = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0 - C72, S72),
        Vec2::new(1.0 - C72 - C36, S72 - S36),
        Vec2::new(-(C72 + C36), S72 - S36),
        Vec2::new(-C36, -S36),
    ]);

    LatticeTiling {
        id: "girih-bowtie",
        name: "Girih Bowtie (72.72.216)",
        description: "Periodic girih tiling by the bowtie (papillon) prototile alone — the concave equilateral girih hexagon (216°,72°,72°,216°,72°,72°, unit edges). Single-tile centered rectangular lattice (cmm); vertex figure 72.72.216. The one non-convex classical girih tile that tessellates by itself.",
        lattice: [
            Vec2::new(C36 + C72, S36 - S72), // t1 = −(U(108°) + U(216°))
            Vec2::new(1.0 + C36, S36),       // t2 = U(0°) − U(216°)
        ],
        cells: vec![Cell { poly: bow, seed: 0 }],
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
    fn girih_bowtie_covers_region() {
        check_covers(
            &girih_bowtie(),
            "girih-bowtie",
            BBox::new(v2(-8.0, -6.0), v2(16.0, 14.0)),
        );
    }

    #[test]
    fn girih_bowtie_rotated_scaled() {
        check_rotated(
            &girih_bowtie(),
            "girih-bowtie(rot30,scale0.7)",
            BBox::new(v2(-12.0, -12.0), v2(12.0, 12.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn girih_bowtie_deterministic() {
        check_det(
            &girih_bowtie(),
            "girih-bowtie",
            BBox::new(v2(-10.0, -10.0), v2(10.0, 10.0)),
        );
    }

    /// Battery: shoelace area == det to 1e-15; edge parity on a 7×7
    /// lattice patch (every interior edge shared exactly twice — pinned
    /// to the Python exact-field census: 120 interior + 54 boundary out
    /// of 6·49 edge occurrences); vertex wedges either close to exactly
    /// 360° or miss at least one tile angle; irrational-offset point
    /// sampling of the half-open fundamental parallelogram hits exactly
    /// one tile per sample.
    #[test]
    fn girih_bowtie_battery() {
        use std::collections::BTreeMap;

        let t = girih_bowtie();
        let [a1, a2] = t.lattice;
        let tile = &t.cells[0].poly;
        let det = a1.x * a2.y - a1.y * a2.x;

        assert!(
            (poly_area(tile) - det).abs() < 1e-15,
            "bowtie area {} vs det {}",
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
            "bowtie: edge shared by more than 2 tiles"
        );
        assert_eq!(
            ecnt.values().filter(|&&c| c == 2).count(),
            120,
            "bowtie: interior edge count on the 7x7 patch"
        );
        let tau = std::f64::consts::TAU;
        for (vk, sum) in &vang {
            let closed = (*sum - tau).abs() < 1e-9;
            let boundary = *sum <= tau - 1.2; // at least one 72-degree wedge missing
            assert!(
                closed || boundary,
                "bowtie vertex {vk:?}: wedge sum {sum} neither closed nor boundary"
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
                assert_eq!(cnt, 1, "bowtie: cell sample covered {cnt} times");
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

    /// Interior angle of a CCW polygon at vertex k (reflex works).
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
