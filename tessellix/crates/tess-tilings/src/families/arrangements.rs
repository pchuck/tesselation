//! Arrangements family: classic irrational arrangement layouts: pinwheel, pythagorean, windmill
//!
//! Owned here: module declarations, `register` (the family's registry
//! slice, in within-tab order), `classification` arms, the family's
//! lattice-culling entries and the family census pin. The family doc is
//! `docs/families/arrangements.md`.

use tess_core::Registry;
use tess_core::classify::TilingClass;

pub mod pinwheel;
pub mod pythagorean;
pub mod windmill;

/// The family's registry slice, in within-tab order.
pub fn register(mut r: Registry) -> Registry {
        r = r.register(pinwheel::PinwheelTiling);
        r = r.register(pythagorean::PythagoreanTiling);
        r = r.register(windmill::spec());
    r
}

/// The family's classification arms (see `crate::classify` for the
/// composer and the shared constructor helpers).
pub(crate) fn classification(id: &str) -> Option<TilingClass> {
    // Arms are verbatim from the pre-split table (they yield
    // `TilingClass`); an unknown id returns None so the composer can
    // fall through to later families.
    Some(match id {
        "windmill" => TilingClass {
            families: &["Arrangement", "Wallpaper"],
            tile_set: Some("1–2–√5 right triangles"),
            chirality: Some("homochiral"),
            ..TilingClass::EMPTY
        },
        "pythagorean" => TilingClass {
            families: &["Arrangement"],
            method: Some("Pythagorean dissection"),
            tile_set: Some("two squares per cell"),
            ..TilingClass::EMPTY
        },
        "pinwheel" => TilingClass {
            families: &["Aperiodic"],
            method: Some("substitution"),
            inflation: Some("√5"),
            source: Some("Conway & Radin"),
            year: Some("1994"),
            aperiodic_status: Some("dense tile orientations"),
            tile_set: Some("1–2–√5 right triangle"),
            translational: Some("non-periodic"),
            ..TilingClass::EMPTY
        },
        _ => return None,
    })
}

/// This family's entries for the global lattice-culling cross-check
/// (`lattice_culling_tests` in `lib.rs`).
#[cfg(test)]
pub(crate) fn lattice_tilings() -> Vec<(&'static str, crate::lattice::LatticeTiling)> {
    vec![
            ("windmill", windmill::spec()),
    ]
}

#[cfg(test)]
mod census {
    use super::*;
    use std::collections::BTreeSet;

    /// The family's registry ids — by construction also its
    /// classification arm ids. Update alongside any membership change
    /// (this test fails on drift in either direction: a tiling
    /// registered but not classified, or classified but not
    /// registered).
    const IDS: &[&str] = &[
        "windmill",
        "pythagorean",
        "pinwheel",
    ];

    #[test]
    fn family_census_matches() {
        let r = register(Registry::new());
        assert_eq!(
            r.len(),
            3,
            "arrangements family census — update when adding a tiling to this family"
        );
        let have: BTreeSet<&str> = r.iter().map(|t| t.id()).collect();
        let want: BTreeSet<&str> = IDS.iter().copied().collect();
        assert_eq!(
            have, want,
            "arrangements family membership drift: register() vs IDS"
        );
        for id in IDS {
            assert!(
                classification(id).is_some(),
                "{id} has no classification arm in this family"
            );
        }
    }
}

#[cfg(test)]
mod pinwheel_tests {
    use crate::tests::{check_coverage, check_det, check_rotated};
    use crate::pinwheel;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::Tiling;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn pinwheel_covers_region() {
        let t = pinwheel::PinwheelTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
        check_coverage(&t, "pinwheel", &p, BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)));
    }

    #[test]
    fn pinwheel_rotated_scaled() {
        check_rotated(
            &pinwheel::PinwheelTiling,
            "pinwheel(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn pinwheel_deterministic() {
        check_det(
            &pinwheel::PinwheelTiling,
            "pinwheel",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}

#[cfg(test)]
mod pythagorean_tests {
    use crate::tests::{check_coverage, check_det, check_rotated};
    use crate::lattice::similarity_map;
    use crate::pythagorean::{PythagoreanTiling, RATIO_ID};
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::tiling::{Query, Tile, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn pythagorean_covers_region() {
        let t = PythagoreanTiling;
        let p = ParamSet::default_for(t.params());
        check_coverage(
            &t,
            "pythagorean",
            &p,
            BBox::new(v2(-4.0, -3.0), v2(12.0, 12.0)),
        );
    }

    #[test]
    fn pythagorean_rotated_scaled() {
        check_rotated(
            &PythagoreanTiling,
            "pythagorean(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn pythagorean_deterministic() {
        check_det(
            &PythagoreanTiling,
            "pythagorean",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// The lattice depends on the ratio parameter, so coverage must hold
    /// across the whole parameter range, not just at the default —
    /// including both extremes, the r = 1 edge-to-edge case, and the
    /// default φ.
    #[test]
    fn pythagorean_ratio_sweep_covers() {
        for r in [0.2, 0.5, 1.0, 1.618_033_988_749_895, 2.0, 3.7, 10.0] {
            let t = PythagoreanTiling;
            let mut p = ParamSet::default_for(t.params());
            p.set(RATIO_ID, ParamValue::Float(r));
            check_coverage(
                &t,
                &format!("pythagorean(r={r})"),
                &p,
                BBox::new(v2(-3.0, -2.0), v2(9.0, 8.0)),
            );
        }
        // Rotated + scaled views must survive too: the culling inverts
        // the world lattice map, which only matches the cell-space map
        // when the rotation is 0.
        for r in [0.5, 1.1] {
            let t = PythagoreanTiling;
            let mut p = ParamSet::default_for(t.params());
            p.set(RATIO_ID, ParamValue::Float(r));
            p.set(crate::lattice::ROTATION_ID, ParamValue::Angle(30.0));
            p.set(crate::lattice::SCALE_ID, ParamValue::Float(0.7));
            check_coverage(
                &t,
                &format!("pythagorean(r={r},rot30,scale0.7)"),
                &p,
                BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            );
        }
    }

    /// The family is not a `LatticeTiling`, so it is not covered by
    /// `lattice_culling_matches_brute_force`; this is its own version:
    /// re-emit every tile over a fixed index window straight from the
    /// construction (no range math at all) and require the exact same
    /// tile set — geometry bit-for-bit, proto, and seed — as the
    /// generator's u-space range culling. Runs at a steep lattice
    /// rotation (r = 1.1 tilts the basis by 42°, where a cell-space bbox
    /// mistaken for the u-space one would cull wrongly) and at a mild
    /// one (r = 2.5).
    #[test]
    fn pythagorean_culling_matches_brute_force() {
        let t = PythagoreanTiling;
        for r in [1.1, 2.5] {
            let mut p = ParamSet::default_for(t.params());
            p.set(RATIO_ID, ParamValue::Float(r));

            // Brute force: every (i, j) in a fixed window, every cell. Mirrors the
            // generator's construction exactly (same reads, same op order) so
            // the comparison is bit-for-bit.
            let r = p.float(RATIO_ID, 1.618_033_988_749_895).clamp(0.2, 10.0);
            let (s1, s2) = if r >= 1.0 { (r, 1.0) } else { (1.0, r) };
            let map = similarity_map(&p);
            let a1 = map.transform(Vec2::new(s1, s2));
            let a2 = map.transform(Vec2::new(-s2, s1));
            let big = Polygon::from_slice(&[
                Vec2::new(0.0, 0.0),
                Vec2::new(s1, 0.0),
                Vec2::new(s1, s1),
                Vec2::new(0.0, s1),
            ])
            .transform(&map);
            let small = Polygon::from_slice(&[
                Vec2::new(s1, 0.0),
                Vec2::new(s1 + s2, 0.0),
                Vec2::new(s1 + s2, s2),
                Vec2::new(s1, s2),
            ])
            .transform(&map);
            let aabbs = [big.aabb(), small.aabb()];
            let region = BBox::new(v2(-3.0, -2.5), v2(5.0, 4.5));
            let window = 8i64;
            let mut brute: Vec<Tile> = Vec::new();
            for i in -window..=window {
                for j in -window..=window {
                    let tr = a1 * i as f64 + a2 * j as f64;
                    for (ci, poly) in [&big, &small].into_iter().enumerate() {
                        let bb = aabbs[ci];
                        let bb = BBox::new(bb.min + tr, bb.max + tr);
                        if bb.intersects(region) {
                            brute.push(Tile {
                                proto: ci as u8,
                                poly: poly.translated(tr),
                                seed: ci as u32,
                            });
                        }
                    }
                }
            }

            let got = t.generate(&p, &Query { region });
            assert!(got.message.is_none(), "r={r}: {:?}", got.message);

            fn key(tile: &Tile) -> (u8, u32, Vec<u64>) {
                let mut bits = Vec::with_capacity(tile.poly.pts.len() * 2);
                for pt in &tile.poly.pts {
                    bits.push(pt.x.to_bits());
                    bits.push(pt.y.to_bits());
                }
                (tile.proto, tile.seed, bits)
            }
            let mut a = brute;
            let mut b = got.tiles;
            a.sort_by_key(key);
            b.sort_by_key(key);
            assert_eq!(
                a.len(),
                b.len(),
                "r={r}: tile count {} vs {}",
                a.len(),
                b.len()
            );
            for (ta, tb) in a.iter().zip(b.iter()) {
                assert_eq!(key(ta), key(tb), "r={r}: tile mismatch");
            }
        }
    }
}

#[cfg(test)]
mod windmill_tests {
    use crate::tests::{check_covers, check_det, check_rotated};
    use crate::windmill;
    use std::collections::{BTreeSet, HashSet};
    use tess_core::geometry::{BBox, Vec2};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn windmill_covers_region() {
        check_covers(
            &windmill::spec(),
            "windmill",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
        );
    }

    #[test]
    fn windmill_rotated_scaled() {
        check_rotated(
            &windmill::spec(),
            "windmill(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn windmill_deterministic() {
        check_det(
            &windmill::spec(),
            "windmill",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
        );
    }

    /// The vertex figures, pinned exactly. Emit the 3×3-block patch
    /// [−6, 6]² straight from the table data (blocks i, j ∈ [−2, 1]² —
    /// no culling) and census every integer vertex of [−3, 3]²:
    /// * 9 lattice vertices (residue (0,0) mod 3) with figure 90⁴;
    /// * 24 T-junctions — 6 of each residue class
    ///   {(0,1), (0,2), (1,0), (2,0)} mod 3 — with figure
    ///   180·90·arctan(1/2)·arctan(2): the 180° is the straight angle
    ///   of the length-2 leg the junction splits (the non-edge-to-edge
    ///   mid-side T-junction, as in `chair.rs`);
    /// * no vertex at the other four residue classes (they lie in tile
    ///   interiors); every occupied figure sums to 360°.
    #[test]
    fn windmill_vertex_figures() {
        const PI: f64 = std::f64::consts::PI;
        const TAU: f64 = std::f64::consts::TAU;
        let spec = windmill::spec();
        // Blocks [−2, 1]²: the patch [−6, 6]², so every vertex of
        // [−3, 3]² is fully surrounded by emitted tiles.
        let mut tiles: Vec<Vec<[i64; 2]>> = Vec::new();
        for i in -2..=1 {
            for j in -2..=1 {
                for c in &spec.cells {
                    tiles.push(
                        c.poly
                            .pts
                            .iter()
                            .map(|p| [p.x.round() as i64 + 3 * i, p.y.round() as i64 + 3 * j])
                            .collect(),
                    );
                }
            }
        }
        assert_eq!(tiles.len(), 80, "5 tiles × 16 blocks");

        /// Interior angle (radians) at the corner `k` of the CCW
        /// polygon `t`, where `p` is the vertex.
        fn corner_angle(t: &[[i64; 2]], k: usize, p: [i64; 2]) -> f64 {
            let n = t.len();
            let a = t[(k + n - 1) % n];
            let b = t[(k + 1) % n];
            let (ux, uy) = (a[0] - p[0], a[1] - p[1]);
            let (vx, vy) = (b[0] - p[0], b[1] - p[1]);
            f64::atan2(
                (ux * vy - uy * vx).unsigned_abs() as f64,
                (ux * vx + uy * vy) as f64,
            )
        }

        /// `p` in the relative interior of the open segment (a, b)?
        /// Collinear + inside the closed box + not an endpoint (the box
        /// test must be non-strict: a vertical/horizontal segment has
        /// its constant coordinate equal at `p`).
        fn on_open_segment(p: [i64; 2], a: [i64; 2], b: [i64; 2]) -> bool {
            if p == a || p == b {
                return false;
            }
            let cross = (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]);
            if cross != 0 {
                return false;
            }
            p[0] >= a[0].min(b[0])
                && p[0] <= a[0].max(b[0])
                && p[1] >= a[1].min(b[1])
                && p[1] <= a[1].max(b[1])
        }

        // The T-junction figure: the straight angle of the split leg,
        // the square's 90° corner, and the two 1–2–√5 acute angles.
        let t_expect = [f64::atan(0.5), f64::atan(2.0), PI / 2.0, PI];
        let assert_figure = |got: &[f64], want: &[f64], p: [i64; 2], label: &str| {
            assert!(
                got.len() == want.len()
                    && got.iter().zip(want).all(|(g, e)| (g - e).abs() < 1e-9),
                "{label} at {p:?}: {got:?} != {want:?}"
            );
        };

        let mut a_count = 0usize;
        let mut t_counts = [0usize; 4];
        for x in -3..=3 {
            for y in -3..=3 {
                let p = [x, y];
                let mut angles: Vec<f64> = Vec::new();
                for t in &tiles {
                    let n = t.len();
                    let mut corner = false;
                    for k in 0..n {
                        if t[k] == p {
                            angles.push(corner_angle(t, k, p));
                            corner = true;
                            break;
                        }
                    }
                    if !corner {
                        // A T-junction sits on the open midpoint of a
                        // length-2 leg: that tile contributes a
                        // straight angle.
                        for k in 0..n {
                            if on_open_segment(p, t[k], t[(k + 1) % n]) {
                                angles.push(PI);
                                break;
                            }
                        }
                    }
                }
                angles.sort_by(|u, v| u.partial_cmp(v).expect("angles are finite"));
                let rem = |v: i64| v.rem_euclid(3);
                match (rem(x), rem(y)) {
                    (0, 0) => {
                        a_count += 1;
                        assert_figure(&angles, &[PI / 2.0; 4], p, "lattice-vertex figure (90⁴)");
                    }
                    (0, 1) | (0, 2) | (1, 0) | (2, 0) => {
                        let idx = match (rem(x), rem(y)) {
                            (0, 1) => 0,
                            (0, 2) => 1,
                            (1, 0) => 2,
                            _ => 3,
                        };
                        t_counts[idx] += 1;
                        assert_figure(
                            &angles,
                            &t_expect,
                            p,
                            "T-junction figure (180·90·arctan(1/2)·arctan(2))",
                        );
                    }
                    _ => {
                        assert!(
                            angles.is_empty(),
                            "vertex figure at the tile-interior point {p:?}: {angles:?}"
                        );
                    }
                }
                if !angles.is_empty() {
                    let sum: f64 = angles.iter().sum();
                    assert!(
                        (sum - TAU).abs() < 1e-9,
                        "vertex figure at {p:?} sums to {sum} rad, not 2π"
                    );
                }
            }
        }
        assert_eq!(a_count, 9, "the 90⁴ orbit is 3×3 in [−3, 3]²");
        let class_labels = ["(0,1)", "(0,2)", "(1,0)", "(2,0)"];
        for (i, n) in t_counts.iter().enumerate() {
            assert_eq!(
                *n,
                6,
                "T-junction orbit {i} (residue class {} mod 3) has 6 vertices in [−3, 3]²",
                class_labels[i]
            );
        }
    }

    /// The wallpaper group is p4, pinned in both directions. The
    /// positive side: 4-fold centres at the motif centres (3i+1.5,
    /// 3j+1.5) *and* the lattice points (3i, 3j), 2-folds at all four
    /// centre classes mod 3, translations 3Z²; the negative side (the
    /// distinctive assertions): no mirrors, no glides, no short
    /// translations — p4m/p4g would add mirrors or glides, p2 would
    /// lack the 4-folds. Homochirality (every triangle the same
    /// chirality sign) is what rules out p4g at the source.
    #[test]
    fn windmill_symmetry_p4() {
        // Window: all tiles whose vertices lie in [−9, 9]² (blocks
        // i, j ∈ [−3, 2]²), emitted straight from the table data.
        let spec = windmill::spec();
        let mut tiles: Vec<Vec<[i64; 2]>> = Vec::new();
        for i in -3..=2 {
            for j in -3..=2 {
                for c in &spec.cells {
                    tiles.push(
                        c.poly
                            .pts
                            .iter()
                            .map(|p| [p.x.round() as i64 + 3 * i, p.y.round() as i64 + 3 * j])
                            .collect(),
                    );
                }
            }
        }
        let key = |t: &[[i64; 2]]| {
            let mut k = t.to_vec();
            k.sort_unstable();
            k
        };
        let set: HashSet<Vec<[i64; 2]>> = tiles.iter().map(|t| key(t)).collect();
        assert_eq!(tiles.len(), 180, "5 tiles × 36 blocks");
        assert_eq!(set.len(), 180, "no two tiles share a vertex set");

        /// Restricted probe: apply `g` and `g⁻¹` to every window tile.
        /// Images whose vertices all stay in the [−9, 9]² window must
        /// land on window tiles; images that leave the integer vertex
        /// lattice (half-integer offsets) prove non-symmetry, and
        /// images that leave the window are skipped (they are checked
        /// by other tiles' inverses, or are the candidate's business
        /// outside the window).
        fn probe<G: Fn([i64; 2]) -> [f64; 2], I: Fn([i64; 2]) -> [f64; 2]>(
            tiles: &[Vec<[i64; 2]>],
            set: &HashSet<Vec<[i64; 2]>>,
            g: G,
            gi: I,
        ) -> bool {
            let check_dir = |dir: &dyn Fn([i64; 2]) -> [f64; 2]| -> bool {
                for t in tiles {
                    let mut img: Vec<[i64; 2]> = Vec::with_capacity(t.len());
                    for &v in t {
                        let w = dir(v);
                        if (w[0].round() - w[0]).abs() > 1e-6 || (w[1].round() - w[1]).abs() > 1e-6 {
                            // Leaves Z²: the image can be no tile.
                            return false;
                        }
                        img.push([w[0].round() as i64, w[1].round() as i64]);
                    }
                    if !img.iter().all(|v| v[0].abs() <= 9 && v[1].abs() <= 9) {
                        continue; // image left the window
                    }
                    let mut k = img;
                    k.sort_unstable();
                    if !set.contains(&k) {
                        return false;
                    }
                }
                true
            };
            check_dir(&|p| g(p)) && check_dir(&|p| gi(p))
        }

        // --- positive: the p4 point group at every centre class ---
        for (cx, cy) in [
            (0.0, 0.0),
            (3.0, 0.0),
            (0.0, 3.0),
            (3.0, 3.0),
            (1.5, 1.5),
            (4.5, 1.5),
            (1.5, 4.5),
        ] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [cx - (p[1] as f64 - cy), cy + (p[0] as f64 - cx)]
            };
            let gi = |p: [i64; 2]| -> [f64; 2] {
                [cx + (p[1] as f64 - cy), cy - (p[0] as f64 - cx)]
            };
            assert!(
                probe(&tiles, &set, g, gi),
                "R90 about ({cx}, {cy}) must be a 4-fold symmetry"
            );
        }
        for (cx, cy) in [
            (0.0, 0.0),
            (1.5, 0.0),
            (0.0, 1.5),
            (1.5, 1.5),
            (3.0, 0.0),
            (0.0, 3.0),
        ] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [2.0 * cx - p[0] as f64, 2.0 * cy - p[1] as f64]
            };
            assert!(
                probe(&tiles, &set, g, g),
                "R180 about ({cx}, {cy}) must be a 2-fold symmetry"
            );
        }
        for (dx, dy) in [(3, 0), (0, 3)] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [p[0] as f64 + dx as f64, p[1] as f64 + dy as f64]
            };
            let gi = |p: [i64; 2]| -> [f64; 2] {
                [p[0] as f64 - dx as f64, p[1] as f64 - dy as f64]
            };
            assert!(
                probe(&tiles, &set, g, gi),
                "lattice translation ({dx}, {dy}) must be a symmetry"
            );
        }

        // --- negative: no 4-/2-fold centres off the four classes ---
        for (cx, cy) in [(1.5, 0.0), (0.5, 0.5)] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [cx - (p[1] as f64 - cy), cy + (p[0] as f64 - cx)]
            };
            let gi = |p: [i64; 2]| -> [f64; 2] {
                [cx + (p[1] as f64 - cy), cy - (p[0] as f64 - cx)]
            };
            assert!(
                !probe(&tiles, &set, g, gi),
                "R90 about ({cx}, {cy}) must NOT be a symmetry (no extra 4-fold centres)"
            );
        }
        {
            let (cx, cy) = (0.5, 0.5);
            let g = |p: [i64; 2]| -> [f64; 2] {
                [2.0 * cx - p[0] as f64, 2.0 * cy - p[1] as f64]
            };
            assert!(
                !probe(&tiles, &set, g, g),
                "R180 about (0.5, 0.5) must NOT be a symmetry (no off-class 2-folds)"
            );
        }

        // --- negative: no mirrors (the complete axis set is pinned by
        // the 4-fold centre set {0, 1.5} mod 3) ---
        for c in [0.0, 1.5] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [2.0 * c - p[0] as f64, p[1] as f64]
            };
            assert!(
                !probe(&tiles, &set, g, g),
                "the mirror line x = {c} must NOT be a symmetry (p4 has no mirrors)"
            );
        }
        for c in [0.0, 1.5] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [p[0] as f64, 2.0 * c - p[1] as f64]
            };
            assert!(
                !probe(&tiles, &set, g, g),
                "the mirror line y = {c} must NOT be a symmetry"
            );
        }
        {
            let g = |p: [i64; 2]| -> [f64; 2] { [p[1] as f64, p[0] as f64] };
            assert!(
                !probe(&tiles, &set, g, g),
                "the mirror line y = x must NOT be a symmetry"
            );
        }
        {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [p[1] as f64 - 1.5, p[0] as f64 + 1.5]
            };
            assert!(
                !probe(&tiles, &set, g, g),
                "the mirror line y = x + 1.5 must NOT be a symmetry"
            );
        }
        for c in [3.0, 4.5] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [c - p[1] as f64, c - p[0] as f64]
            };
            assert!(
                !probe(&tiles, &set, g, g),
                "the mirror line y = -x + {c} must NOT be a symmetry"
            );
        }

        // --- negative: no glides (each mirror axis with the half-
        // period along it; half-lattice vectors 3/2) ---
        for c in [0.0, 1.5] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [2.0 * c - p[0] as f64, p[1] as f64 + 1.5]
            };
            let gi = |p: [i64; 2]| -> [f64; 2] {
                [2.0 * c - p[0] as f64, p[1] as f64 - 1.5]
            };
            assert!(
                !probe(&tiles, &set, g, gi),
                "the glide on x = {c} (half-period (0, 1.5)) must NOT be a symmetry"
            );
        }
        for c in [0.0, 1.5] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [p[0] as f64 + 1.5, 2.0 * c - p[1] as f64]
            };
            let gi = |p: [i64; 2]| -> [f64; 2] {
                [p[0] as f64 - 1.5, 2.0 * c - p[1] as f64]
            };
            assert!(
                !probe(&tiles, &set, g, gi),
                "the glide on y = {c} (half-period (1.5, 0)) must NOT be a symmetry"
            );
        }
        {
            // y = x, half-period (1.5, 1.5)
            let g = |p: [i64; 2]| -> [f64; 2] {
                [p[1] as f64 + 1.5, p[0] as f64 + 1.5]
            };
            let gi = |p: [i64; 2]| -> [f64; 2] {
                [p[1] as f64 - 1.5, p[0] as f64 - 1.5]
            };
            assert!(
                !probe(&tiles, &set, g, gi),
                "the glide on y = x (half-period (1.5, 1.5)) must NOT be a symmetry"
            );
        }
        {
            // y = x + 1.5, half-period (1.5, 1.5): g(x, y) = (y, x + 3)
            let g = |p: [i64; 2]| -> [f64; 2] { [p[1] as f64, p[0] as f64 + 3.0] };
            let gi = |p: [i64; 2]| -> [f64; 2] { [p[1] as f64 - 3.0, p[0] as f64] };
            assert!(
                !probe(&tiles, &set, g, gi),
                "the glide on y = x + 1.5 (half-period (1.5, 1.5)) must NOT be a symmetry"
            );
        }
        for c in [3.0, 4.5] {
            // y = -x + c, half-period (1.5, -1.5)
            let g = |p: [i64; 2]| -> [f64; 2] {
                [c - p[1] as f64 + 1.5, c - p[0] as f64 - 1.5]
            };
            let gi = |p: [i64; 2]| -> [f64; 2] {
                [c - p[1] as f64 - 1.5, c - p[0] as f64 + 1.5]
            };
            assert!(
                !probe(&tiles, &set, g, gi),
                "the glide on y = -x + {c} (half-period (1.5, -1.5)) must NOT be a symmetry"
            );
        }

        // --- negative: no translations shorter than 3Z² ---
        for (dx, dy) in [(1, 0), (0, 1), (2, 0), (0, 2)] {
            let g = |p: [i64; 2]| -> [f64; 2] {
                [p[0] as f64 + dx as f64, p[1] as f64 + dy as f64]
            };
            let gi = |p: [i64; 2]| -> [f64; 2] {
                [p[0] as f64 - dx as f64, p[1] as f64 - dy as f64]
            };
            assert!(
                !probe(&tiles, &set, g, gi),
                "translation ({dx}, {dy}) must NOT be a symmetry (minimal translations are 3Z²)"
            );
        }

        // --- homochirality: every triangle the same chirality sign ---
        let signs: BTreeSet<i32> = tiles
            .iter()
            .filter(|t| t.len() == 3)
            .map(|t| {
                // The right-angle vertex: the two edge vectors dot to 0
                // (exact — integer coordinates).
                for k in 0..t.len() {
                    let a = t[(k + t.len() - 1) % t.len()];
                    let b = t[(k + 1) % t.len()];
                    let (ux, uy) = (a[0] - t[k][0], a[1] - t[k][1]);
                    let (vx, vy) = (b[0] - t[k][0], b[1] - t[k][1]);
                    if ux * vx + uy * vy == 0 {
                        // Cross of the legs in the CCW vertex order —
                        // one sign across the tiling (p4, not p4g).
                        return (ux * vy - uy * vx).signum() as i32;
                    }
                }
                panic!("no right angle in triangle {t:?}");
            })
            .collect();
        assert_eq!(
            signs.len(),
            1,
            "all 100 window triangles are homochiral (p4, not p4g): signs = {signs:?}"
        );
    }
}

