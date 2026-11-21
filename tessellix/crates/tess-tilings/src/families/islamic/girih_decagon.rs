//! Girih decagon tiling — the regular decagon prototile with its forced
//! bowtie infill, as a [`LatticeTiling`] data table.
//!
//! * `10.6.10` — Girih Decagon (cmm)
//!
//! The regular girih decagon (ten unit edges, 144° angles — the fifth
//! classical girih tile) cannot tessellate alone: 144° admits no closed
//! vertex figure from decagons only (2 × 144 = 288 leaves 72°, 3 × 144
//! overshoots). Its primitive periodic edge-to-edge arrangement is the
//! two-tile cell of one decagon plus one bowtie — the decagon's 72°
//! residue is exactly the bowtie's acute corner, so at each decagon
//! vertex two decagons and one bowtie close (144 + 72 + 144 = 360, the
//! `10.6.10` figure).
//!
//! The lattice `span{ (0, −2(S72 + S36)), (1/2 + 3·C36, S72 + 2·S36) }`
//! has `|det| = 2·a³ − 5a/2` over K = Q(2cos 18°), which equals
//! decagon-area + bowtie-area exactly (7.694208843 + 1.314327780 =
//! 9.008536623): one cell per two tiles. Over the rectangle axes of the
//! centered lattice the tiling carries two perpendicular mirror
//! families (through the decagon's vertex–edge axes) with interleaved
//! glide lines, and 2-fold centers off the mirrors: wallpaper group
//! cmm. The probe (`wp3.py`, spike) confirms no other point-group
//! elements.
//!
//! # Seeds
//!
//! Decagon seed 0, bowtie seed 1; the shipped seed is decorative
//! (`SeedPattern::Checker` on lattice-site parity).

use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

// Radical constants (exact double values; f64::sqrt is not const-stable).
const SQ5: f64 = 2.236_067_977_499_79; // √5
const C36: f64 = (1.0 + SQ5) / 4.0; // cos 36° = (1+√5)/4
const S36: f64 = 0.587_785_252_292_473_1; // sin 36° = (√5−1)/2 · sin 72°
const S72: f64 = 0.951_056_516_295_153_5; // sin 72° = √(10+2√5)/4

/// Girih decagon + bowtie lattice tiling, wallpaper group cmm.
///
/// Two tiles per cell: the regular decagon (unit edges, 144° angles)
/// and one bowtie filling its 72° residue; `det =` both areas summed,
/// exact over Q(√5). Centered rectangular lattice, cmm.
pub fn girih_decagon() -> LatticeTiling {
    // Regular decagon, CCW, base edge from the origin along +x.
    let dec = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0 + C36, S36),
        Vec2::new(0.5 + 2.0 * C36, S72 + S36),
        Vec2::new(1.0 + C36, 2.0 * S72 + S36),
        Vec2::new(1.0, 2.0 * S72 + 2.0 * S36),
        Vec2::new(0.0, 2.0 * S72 + 2.0 * S36),
        Vec2::new(-C36, 2.0 * S72 + S36),
        Vec2::new(0.5 - 2.0 * C36, S72 + S36),
        Vec2::new(-C36, S36),
    ]);
    // Concave girih bowtie (216°,72°,72°,216°,72°,72°), CCW, sharing the
    // decagon edge (1,0) — (1 + C36, S36).
    let bow = Polygon::from_slice(&[
        Vec2::new(0.5 + 2.0 * C36, S36 - S72),
        Vec2::new(1.0 + C36, S36),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0 + C36, -S36),
        Vec2::new(0.5 + 2.0 * C36, -S72 - S36),
        Vec2::new(0.5 + 3.0 * C36, -S72),
    ]);

    LatticeTiling {
        id: "girih-decagon",
        name: "Girih Decagon (10.6.10)",
        description: "Periodic girih tiling by the regular decagon prototile with its forced bowtie infill — the decagon (unit edges, ten 144° angles) cannot tile alone; the primitive edge-to-edge cell is one decagon plus one bowtie (det = both areas exact over Q(√5), vertex figure 10.6.10). Centered rectangular lattice; mirror families along both rectangle axes with interleaved glides give cmm.",
        lattice: [
            Vec2::new(0.0, -2.0 * (S72 + S36)),
            Vec2::new(0.5 + 3.0 * C36, S72 + 2.0 * S36),
        ],
        cells: vec![
            Cell { poly: dec, seed: 0 },
            Cell {
                poly: bow,
                seed: 1,
            },
        ],
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
    fn girih_decagon_covers_region() {
        check_covers(
            &girih_decagon(),
            "girih-decagon",
            BBox::new(v2(-8.0, -8.0), v2(16.0, 14.0)),
        );
    }

    #[test]
    fn girih_decagon_rotated_scaled() {
        check_rotated(
            &girih_decagon(),
            "girih-decagon(rot30,scale0.7)",
            BBox::new(v2(-12.0, -12.0), v2(12.0, 12.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn girih_decagon_deterministic() {
        check_det(
            &girih_decagon(),
            "girih-decagon",
            BBox::new(v2(-10.0, -10.0), v2(10.0, 10.0)),
        );
    }

    /// Battery: shoelace area sum == det to 1e-15; edge parity on a 7x7
    /// lattice patch (pinned to the Python exact-field battery: 313
    /// interior + 158 boundary of (10 + 6) * 49 = 784 edge
    /// occurrences); vertex wedges closed-or-boundary; irrational-offset
    /// point sampling of the half-open fundamental parallelogram hits
    /// exactly one tile per sample.
    #[test]
    fn girih_decagon_battery() {
        use std::collections::BTreeMap;

        let t = girih_decagon();
        let [a1, a2] = t.lattice;
        let det = a1.x * a2.y - a1.y * a2.x;

        let area: f64 = t.cells.iter().map(|c| poly_area(&c.poly)).sum();
        assert!(
            (area - det).abs() < 1e-15,
            "decagon cell area {} vs det {}",
            area,
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
                for cell in &t.cells {
                    let tile = &cell.poly;
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
        }
        assert!(
            ecnt.values().all(|&c| c <= 2),
            "decagon: edge shared by more than 2 tiles"
        );
        assert_eq!(
            ecnt.values().filter(|&&c| c == 2).count(),
            313,
            "decagon: interior edge count on the 7x7 patch"
        );
        let tau = std::f64::consts::TAU;
        for (vk, sum) in &vang {
            let closed = (*sum - tau).abs() < 1e-9;
            let boundary = *sum <= tau - 1.2; // at least a 72-degree wedge missing
            assert!(
                closed || boundary,
                "decagon vertex {vk:?}: wedge sum {sum} neither closed nor boundary"
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
                for i in -2..=2 {
                    for j in -2..=2 {
                        let r = v2(
                            p.x - (a1.x * i as f64 + a2.x * j as f64),
                            p.y - (a1.y * i as f64 + a2.y * j as f64),
                        );
                        for cell in &t.cells {
                            if pip(&cell.poly, r) {
                                cnt += 1;
                            }
                        }
                    }
                }
                assert_eq!(cnt, 1, "decagon: cell sample covered {cnt} times");
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

    /// Interior angle of a CCW polygon at vertex k (reflex > pi).
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
