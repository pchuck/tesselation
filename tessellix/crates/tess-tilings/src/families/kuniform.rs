//! Kuniform family: the 2-uniform program: 18 k-uniform primal tables + 17 face-centroid
//! duals (the duals are Laves-tab primaries)
//!
//! Owned here: module declarations, `register` (the family's registry
//! slice, in within-tab order), `classification` arms, the family's
//! lattice-culling entries and the family census pin. The family doc is
//! `docs/families/kuniform.md`.

use tess_core::Registry;
use tess_core::classify::TilingClass;

use crate::classify::{two_uniform, kuniform_dual};

pub mod kuniform;
pub mod dual;

/// The family's registry slice, in within-tab order.
pub fn register(mut r: Registry) -> Registry {
        r = r.register(kuniform::kuniform_p6m());
        r = r.register(kuniform::kuniform_pmm());
        r = r.register(kuniform::kuniform_n18());
        r = r.register(kuniform::kuniform_n9());
        r = r.register(kuniform::kuniform_n8());
        r = r.register(kuniform::kuniform_n5());
        r = r.register(kuniform::kuniform_n1());
        r = r.register(kuniform::kuniform_n13());
        r = r.register(kuniform::kuniform_n20());
        r = r.register(kuniform::kuniform_n12());
        r = r.register(kuniform::kuniform_n19());
        r = r.register(kuniform::kuniform_n4());
        r = r.register(kuniform::kuniform_n14());
        r = r.register(kuniform::kuniform_n3());
        r = r.register(kuniform::kuniform_n15());
        r = r.register(kuniform::kuniform_n2());
        r = r.register(kuniform::kuniform_n16());
        r = r.register(kuniform::kuniform_n17());
        r = r.register(dual::kuniform_dual_p6m());
        r = r.register(dual::kuniform_dual_n18());
        r = r.register(dual::kuniform_dual_n9());
        r = r.register(dual::kuniform_dual_n8());
        r = r.register(dual::kuniform_dual_n5());
        r = r.register(dual::kuniform_dual_n1());
        r = r.register(dual::kuniform_dual_n13());
        r = r.register(dual::kuniform_dual_n20());
        r = r.register(dual::kuniform_dual_pmm());
        r = r.register(dual::kuniform_dual_n12());
        r = r.register(dual::kuniform_dual_n19());
        r = r.register(dual::kuniform_dual_n2());
        r = r.register(dual::kuniform_dual_n16());
        r = r.register(dual::kuniform_dual_n4());
        r = r.register(dual::kuniform_dual_n3());
        r = r.register(dual::kuniform_dual_n14());
        r = r.register(dual::kuniform_dual_n15());
    r
}

/// The family's classification arms (see `crate::classify` for the
/// composer and the shared constructor helpers).
pub(crate) fn classification(id: &str) -> Option<TilingClass> {
    // Arms are verbatim from the pre-split table (they yield
    // `TilingClass`); an unknown id returns None so the composer can
    // fall through to later families.
    Some(match id {
        "kuniform-p6m" => two_uniform(&["Two-Uniform"], "(3⁶; 3².6²)"),
        "kuniform-pmm" => two_uniform(&["Two-Uniform", "Wallpaper"], "(3.6.3.6; 3².6²)"),
        "kuniform-n18" => two_uniform(&["Two-Uniform"], "(3⁶; 3².4.3.4)"),
        "kuniform-n9" => two_uniform(&["Two-Uniform"], "(3.4.6.4; 3².4.3.4)"),
        "kuniform-n8" => two_uniform(&["Two-Uniform"], "(3.4.6.4; 3³.4²)"),
        "kuniform-n5" => two_uniform(&["Two-Uniform"], "(3.4.6.4; 3.4².6)"),
        "kuniform-n1" => two_uniform(&["Two-Uniform"], "(4.6.12; 3.4.6.4)"),
        "kuniform-n13" => two_uniform(&["Two-Uniform"], "(3⁶; 3².4.12)"),
        "kuniform-n20" => two_uniform(&["Two-Uniform"], "(3⁶; 3⁴.6)₁"),
        "kuniform-n12" => two_uniform(&["Two-Uniform"], "(3².6²; 3⁴.6)"),
        "kuniform-n19" => two_uniform(&["Two-Uniform"], "(3⁶; 3⁴.6)₂"),
        "kuniform-n4" => two_uniform(&["Two-Uniform"], "(4⁴; 3³.4²)₁"),
        "kuniform-n14" => two_uniform(&["Two-Uniform"], "(3⁶; 3³.4²)₁"),
        "kuniform-n3" => two_uniform(&["Two-Uniform"], "(4⁴; 3³.4²)₂"),
        "kuniform-n15" => two_uniform(&["Two-Uniform"], "(3⁶; 3³.4²)₂"),
        "kuniform-n2" => two_uniform(&["Two-Uniform"], "(3.12.12; 3.4.3.12)"),
        "kuniform-n16" => two_uniform(&["Two-Uniform"], "(3³.4²; 3².4.3.4)"),
        "kuniform-n17" => two_uniform(&["Two-Uniform"], "(3³.4²; 3².4.3.4)₂"),
        "kuniform-dual-p6m" => kuniform_dual("V3⁶; V3².6²", "(3⁶; 3².6²)"),
        "kuniform-dual-n18" => kuniform_dual("V3⁶; V3².4.3.4", "(3⁶; 3².4.3.4)"),
        "kuniform-dual-n9" => kuniform_dual("V3.4.6.4; V3².4.3.4", "(3.4.6.4; 3².4.3.4)"),
        "kuniform-dual-n8" => kuniform_dual("V3.4.6.4; V3³.4²", "(3.4.6.4; 3³.4²)"),
        "kuniform-dual-n5" => kuniform_dual("V3.4.6.4; V3.4².6", "(3.4.6.4; 3.4².6)"),
        "kuniform-dual-n1" => kuniform_dual("V4.6.12; V3.4.6.4", "(4.6.12; 3.4.6.4)"),
        "kuniform-dual-n13" => kuniform_dual("V3⁶; V3².4.12", "(3⁶; 3².4.12)"),
        "kuniform-dual-n20" => kuniform_dual("V3⁶; V3⁴.6", "(3⁶; 3⁴.6)₁"),
        "kuniform-dual-pmm" => kuniform_dual("V3.6.3.6; V3².6²", "(3.6.3.6; 3².6²)"),
        "kuniform-dual-n12" => kuniform_dual("V3².6²; V3⁴.6", "(3².6²; 3⁴.6)"),
        "kuniform-dual-n19" => kuniform_dual("V3⁶; V3⁴.6", "(3⁶; 3⁴.6)₂"),
        "kuniform-dual-n2" => kuniform_dual("V3.4.3.12; V3.12.12", "(3.12.12; 3.4.3.12)"),
        "kuniform-dual-n16" => kuniform_dual("V3².4.3.4; V3³.4²", "(3³.4²; 3².4.3.4)"),
        "kuniform-dual-n4" => kuniform_dual("V4⁴; V3³.4²", "(4⁴; 3³.4²)₁"),
        "kuniform-dual-n3" => kuniform_dual("V4⁴; V3³.4²", "(4⁴; 3³.4²)₂"),
        "kuniform-dual-n14" => kuniform_dual("V3⁶; V3³.4²", "(3⁶; 3³.4²)₁"),
        "kuniform-dual-n15" => kuniform_dual("V3⁶; V3³.4²", "(3⁶; 3³.4²)₂"),
        _ => return None,
    })
}

/// This family's entries for the global lattice-culling cross-check
/// (`lattice_culling_tests` in `lib.rs`).
#[cfg(test)]
pub(crate) fn lattice_tilings() -> Vec<(&'static str, crate::lattice::LatticeTiling)> {
    vec![
            ("kuniform-p6m", kuniform::kuniform_p6m()),
            ("kuniform-pmm", kuniform::kuniform_pmm()),
            ("kuniform-n18", kuniform::kuniform_n18()),
            ("kuniform-n9", kuniform::kuniform_n9()),
            ("kuniform-n8", kuniform::kuniform_n8()),
            ("kuniform-n5", kuniform::kuniform_n5()),
            ("kuniform-n1", kuniform::kuniform_n1()),
            ("kuniform-n13", kuniform::kuniform_n13()),
            ("kuniform-n2", kuniform::kuniform_n2()),
            ("kuniform-n16", kuniform::kuniform_n16()),
            ("kuniform-n17", kuniform::kuniform_n17()),
            ("kuniform-dual-p6m", dual::kuniform_dual_p6m()),
            ("kuniform-dual-n18", dual::kuniform_dual_n18()),
            ("kuniform-dual-n9", dual::kuniform_dual_n9()),
            ("kuniform-dual-n8", dual::kuniform_dual_n8()),
            ("kuniform-dual-n5", dual::kuniform_dual_n5()),
            ("kuniform-dual-n1", dual::kuniform_dual_n1()),
            ("kuniform-dual-n13", dual::kuniform_dual_n13()),
            ("kuniform-dual-n20", dual::kuniform_dual_n20()),
            ("kuniform-dual-pmm", dual::kuniform_dual_pmm()),
            ("kuniform-dual-n12", dual::kuniform_dual_n12()),
            ("kuniform-dual-n19", dual::kuniform_dual_n19()),
            ("kuniform-dual-n2", dual::kuniform_dual_n2()),
            ("kuniform-dual-n16", dual::kuniform_dual_n16()),
            ("kuniform-dual-n4", dual::kuniform_dual_n4()),
            ("kuniform-dual-n3", dual::kuniform_dual_n3()),
            ("kuniform-dual-n14", dual::kuniform_dual_n14()),
            ("kuniform-dual-n15", dual::kuniform_dual_n15()),
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
        "kuniform-p6m",
        "kuniform-pmm",
        "kuniform-n18",
        "kuniform-n9",
        "kuniform-n8",
        "kuniform-n5",
        "kuniform-n1",
        "kuniform-n13",
        "kuniform-n20",
        "kuniform-n12",
        "kuniform-n19",
        "kuniform-n4",
        "kuniform-n14",
        "kuniform-n3",
        "kuniform-n15",
        "kuniform-n2",
        "kuniform-n16",
        "kuniform-n17",
        "kuniform-dual-p6m",
        "kuniform-dual-n18",
        "kuniform-dual-n9",
        "kuniform-dual-n8",
        "kuniform-dual-n5",
        "kuniform-dual-n1",
        "kuniform-dual-n13",
        "kuniform-dual-n20",
        "kuniform-dual-pmm",
        "kuniform-dual-n12",
        "kuniform-dual-n19",
        "kuniform-dual-n2",
        "kuniform-dual-n16",
        "kuniform-dual-n4",
        "kuniform-dual-n3",
        "kuniform-dual-n14",
        "kuniform-dual-n15",
    ];

    #[test]
    fn family_census_matches() {
        let r = register(Registry::new());
        assert_eq!(
            r.len(),
            35,
            "kuniform family census — update when adding a tiling to this family"
        );
        let have: BTreeSet<&str> = r.iter().map(|t| t.id()).collect();
        let want: BTreeSet<&str> = IDS.iter().copied().collect();
        assert_eq!(
            have, want,
            "kuniform family membership drift: register() vs IDS"
        );
        for id in IDS {
            assert!(
                classification(id).is_some(),
                "{id} has no classification arm in this family"
            );
        }
    }
}


/// Tests for the 2-uniform (k-uniform) `LatticeTiling` tables.
#[cfg(test)]
mod kuniform_tests {
    use crate::lattice::LatticeTiling;
    use crate::tests::{check_covers, check_det, check_rotated};
    use crate::kuniform;
    use std::collections::{BTreeSet, HashMap};
    use tess_core::geometry::{BBox, Vec2};

    const H: f64 = 0.8660254037844386; // √3/2

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn kuniform_p6m_covers_region() {
        check_covers(
            &kuniform::kuniform_p6m(),
            "kuniform-p6m",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn kuniform_p6m_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_p6m(),
            "kuniform-p6m(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_p6m_deterministic() {
        check_det(
            &kuniform::kuniform_p6m(),
            "kuniform-p6m",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// One expected figure orbit in the ±3×±3 patch census.
    struct Figure<'a> {
        /// Canonical figure string (lexicographically smallest cyclic
        /// rotation, e.g. "3366", "3636", "333333").
        figure: &'a str,
        /// Interior vertices of that figure in the patch.
        count: usize,
        /// Orbit reps in [0,b1]×[0,b2], 3-decimal quantized.
        orbits: &'a [(i32, i32)],
    }

    /// Per-vertex sector record: (start angle deg, span deg, side count).
    type Sector = (f64, f64, u32);

    /// Emit the ±3×±3 patch straight from the table (the same
    /// emission the Python spike used), deduplicate vertices by
    /// 6-decimal key, and check the interior-vertex census: the
    /// angle-sum-360 vertices carry exactly the declared figures,
    /// with the expected counts and translation-orbit structure.
    fn check_vertex_figures(lt: &LatticeTiling, expect: &[Figure]) {
        let b1 = lt.lattice[0];
        let b2 = lt.lattice[1];
        let det = b1.cross(b2);
        // Inverse of the lattice map p -> [b1 b2] * u (the same
        // world->lattice inverse as the generator, in cell space).
        let inv = |p: Vec2| -> Vec2 {
            Vec2::new(
                (b2.y * p.x - b2.x * p.y) / det,
                (-b1.y * p.x + b1.x * p.y) / det,
            )
        };
        let key = |p: Vec2| -> (i64, i64) {
            ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64)
        };

        // Per vertex key: the accumulated interior angle (60° triangle
        // corner, 120° hexagon corner) and the CCW sectors
        // (start angle deg, span deg, side count).
        let mut angles: HashMap<(i64, i64), f64> = HashMap::new();
        let mut sectors: HashMap<(i64, i64), Vec<Sector>> = HashMap::new();
        for i in -3i64..=3 {
            for j in -3i64..=3 {
                let tr = b1 * i as f64 + b2 * j as f64;
                for c in &lt.cells {
                    let pts = c.poly.translated(tr).pts;
                    let n = pts.len();
                    let span = (n - 2) as f64 * 180.0 / n as f64;
                    for k in 0..n {
                        let v = pts[k];
                        let b = pts[(k + 1) % n];
                        let kk = key(v);
                        *angles.entry(kk).or_insert(0.0) += span;
                        // Sector of this CCW polygon at v: CCW from the
                        // v->next direction, spanning the interior angle.
                        let start = f64::atan2(b.y - v.y, b.x - v.x).to_degrees();
                        let start = if start < 0.0 { start + 360.0 } else { start };
                        sectors.entry(kk).or_default().push((start, span, n as u32));
                    }
                }
            }
        }

        // Canonical cyclic figure (lex-smallest rotation of the
        // side-count sequence) of one interior vertex.
        let figure_of = |secs: &[Sector]| -> String {
            let mut s: Vec<Sector> = secs.to_vec();
            s.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            // Sectors of an interior vertex tile the 360 degrees:
            // walk them CCW from the smallest start angle.
            let mut cur = s[0].0;
            for (s0, sp, _) in s.iter() {
                assert!(
                    (*s0 - cur).abs() < 1e-6,
                    "figure walk gap: sector at {s0}, expected {cur}"
                );
                cur += sp;
                if cur >= 360.0 - 1e-9 {
                    cur -= 360.0;
                }
            }
            assert!(
                (cur - s[0].0).abs() < 1e-6,
                "figure walk must close, ends at {cur}"
            );
            let seq: Vec<u32> = s.iter().map(|t| t.2).collect();
            let mut best = seq.clone();
            for r in 1..seq.len() {
                let mut rot = Vec::with_capacity(seq.len());
                rot.extend_from_slice(&seq[r..]);
                rot.extend_from_slice(&seq[..r]);
                if rot < best {
                    best = rot;
                }
            }
            best.iter().map(|n| n.to_string()).collect()
        };

        let mut figs: HashMap<String, Vec<(i64, i64)>> = HashMap::new();
        for (kk, sum) in &angles {
            if (sum - 360.0).abs() < 1e-6 {
                let secs = sectors
                    .get(kk)
                    .unwrap_or_else(|| panic!("interior vertex without sectors"));
                let f = figure_of(secs);
                figs.entry(f).or_default().push(*kk);
            }
        }

        for e in expect {
            let vs = figs.get(e.figure).cloned().unwrap_or_default();
            assert_eq!(
                vs.len(),
                e.count,
                "{}: {} interior vertices in the ±3 patch, expected {}",
                e.figure,
                vs.len(),
                e.count
            );
            // Orbit reps: v - A*floor(A^-1 v), quantized to 3
            // decimals (a 1e-5 guard keeps vertices exactly on a
            // lattice line in the correct fundamental cell).
            let mut orbits: BTreeSet<(i32, i32)> = BTreeSet::new();
            for &kk in &vs {
                let p = v2(kk.0 as f64 * 1e-6, kk.1 as f64 * 1e-6);
                let u = inv(p);
                let fx = (u.x + 1e-5).floor();
                let fy = (u.y + 1e-5).floor();
                let rep = p - b1 * fx - b2 * fy;
                orbits.insert(
                    ((rep.x * 1000.0).round() as i32, (rep.y * 1000.0).round() as i32),
                );
            }
            let want: BTreeSet<(i32, i32)> = e.orbits.iter().copied().collect();
            assert_eq!(&orbits, &want, "{}: orbit reps in [0,b1]×[0,b2]", e.figure);
        }
        let declared: BTreeSet<&str> = expect.iter().map(|e| e.figure).collect();
        for f in figs.keys() {
            assert!(
                declared.contains(f.as_str()),
                "unexpected vertex figure {f} (declared: {declared:?})"
            );
        }
    }

    /// Interior vertex figures over the ±3×±3 patch: one 3⁶ orbit
    /// (the fan apexes) and six 3².6² orbits — the honeycomb-vertex
    /// orbit splits into six translation orbits under the index-3
    /// translation lattice. Counts and reps match the Python spike.
    #[test]
    fn kuniform_p6m_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_p6m(),
            &[
                Figure {
                    figure: "333333",
                    count: 49,
                    orbits: &[(0, 0)],
                },
                Figure {
                    figure: "3366",
                    count: 241,
                    orbits: &[
                        (500, 866),
                        (1000, 0),
                        (1000, 1732),
                        (2000, 0),
                        (2000, 1732),
                        (2500, 866),
                    ],
                },
            ],
        );
    }

    /// Cell-set symmetry probe (the Python spike's exact test in
    /// floats): `g` must send every table cell to some table cell
    /// plus a lattice translation — Cramer solve for the shift,
    /// integrality within 1e-6, vertex geometry within 1e-9. With
    /// `symmetry = false` (a negative probe) at least one cell must
    /// fail to map — individual cells can match (a hexagon is
    /// invariant under 60° about its own centre) without the
    /// mapping extending to the whole tiling.
    fn probe_symmetry(lt: &LatticeTiling, g: impl Fn(Vec2) -> Vec2, symmetry: bool, label: &str) {
        let b1 = lt.lattice[0];
        let b2 = lt.lattice[1];
        let det = b1.cross(b2);
        let inv = |p: Vec2| -> Vec2 {
            Vec2::new(
                (b2.y * p.x - b2.x * p.y) / det,
                (-b1.y * p.x + b1.x * p.y) / det,
            )
        };
        let cells: Vec<Vec<Vec2>> = lt.cells.iter().map(|c| c.poly.pts.clone()).collect();
        let mut mismatched = 0;
        for (i, cp) in cells.iter().enumerate() {
            let gp: Vec<Vec2> = cp.iter().map(|p| g(*p)).collect();
            let mut matched = false;
            for cj in &cells {
                let mut hit = false;
                for w in cj {
                    let t = gp[0] - *w;
                    let u = inv(t);
                    // A symmetry shift must be a lattice translation.
                    if (u.x - u.x.round()).abs() > 1e-6 || (u.y - u.y.round()).abs() > 1e-6 {
                        continue;
                    }
                    let tr = b1 * u.x.round() + b2 * u.y.round();
                    let shifted: Vec<Vec2> = cj.iter().map(|w| *w + tr).collect();
                    hit = gp.iter().all(|p| {
                        shifted.iter().any(|s| (*p - *s).length() < 1e-9)
                    }) && shifted.iter().all(|s| gp.iter().any(|p| (*p - *s).length() < 1e-9));
                    if hit {
                        break;
                    }
                }
                if hit {
                    matched = true;
                    break;
                }
            }
            if symmetry {
                assert!(
                    matched,
                    "{label}: table cell {i} expected a match, none found"
                );
            } else if !matched {
                mismatched += 1;
            }
        }
        assert!(
            symmetry || mismatched > 0,
            "{label}: every table cell matched — expected a negative probe to fail"
        );
    }

    /// The declared p6m generators act on the cell set: 60°/120°
    /// rotations about the fan apex at the origin, the mirror y = 0,
    /// and the mirror along the 60° line.
    #[test]
    fn kuniform_p6m_symmetry() {
        let lt = kuniform::kuniform_p6m();
        probe_symmetry(
            &lt,
            |p| p.rotated(std::f64::consts::PI / 3.0),
            true,
            "R60 about (0,0)",
        );
        probe_symmetry(
            &lt,
            |p| p.rotated(2.0 * std::f64::consts::PI / 3.0),
            true,
            "R120 about (0,0)",
        );
        probe_symmetry(&lt, |p| v2(p.x, -p.y), true, "mirror y=0");
        probe_symmetry(
            &lt,
            |p| {
                let d = v2(0.5, H);
                d * (2.0 * p.dot(d)) - p
            },
            true,
            "mirror line at 60 deg",
        );
    }

    #[test]
    fn kuniform_pmm_covers_region() {
        check_covers(
            &kuniform::kuniform_pmm(),
            "kuniform-pmm",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn kuniform_pmm_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_pmm(),
            "kuniform-pmm(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_pmm_deterministic() {
        check_det(
            &kuniform::kuniform_pmm(),
            "kuniform-pmm",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: one 3.6.3.6
    /// orbit (the hexagon-corner vertices (2i+1, √3j)) and two 3².6²
    /// orbits (the triangle-base vertices (2i+1/2, H+√3j)). Counts
    /// and reps match the Python spike.
    #[test]
    fn kuniform_pmm_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_pmm(),
            &[
                Figure {
                    figure: "3636",
                    count: 36,
                    orbits: &[(1000, 0)],
                },
                Figure {
                    figure: "3366",
                    count: 78,
                    orbits: &[(500, 866), (1500, 866)],
                },
            ],
        );
    }

    fn rot_about(deg: f64, c: Vec2, p: Vec2) -> Vec2 {
        c + (p - c).rotated(deg.to_radians())
    }

    /// The declared pmm generators act on the cell set: mirrors
    /// x = 0, y = 0, y = H and the half-turns at (1, 0) and (1, H).
    /// Negative probes: no 6-, 9-, or 12-fold rotation about any of
    /// (0,0), (1,0), (1,H) — pmm has none (lattice argument: R60
    /// sends b1 = (2,0) to (1, √3) = b1/2 + b2, not a lattice
    /// vector).
    #[test]
    fn kuniform_pmm_symmetry() {
        let lt = kuniform::kuniform_pmm();
        probe_symmetry(&lt, |p| v2(-p.x, p.y), true, "mirror x=0");
        probe_symmetry(&lt, |p| v2(p.x, -p.y), true, "mirror y=0");
        probe_symmetry(&lt, |p| v2(p.x, 2.0 * H - p.y), true, "mirror y=H");
        probe_symmetry(&lt, |p| v2(2.0 - p.x, -p.y), true, "180 deg about (1,0)");
        probe_symmetry(&lt, |p| v2(2.0 - p.x, 2.0 * H - p.y), true, "180 deg about (1,H)");
        for c in [Vec2::ZERO, v2(1.0, 0.0), v2(1.0, H)] {
            for deg in [60.0, 90.0, 120.0] {
                probe_symmetry(
                    &lt,
                    move |p| rot_about(deg, c, p),
                    false,
                    &format!("{deg} deg about ({c})"),
                );
            }
        }
    }

    #[test]
    fn kuniform_n18_covers_region() {
        check_covers(
            &kuniform::kuniform_n18(),
            "kuniform-n18",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn kuniform_n18_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n18(),
            "kuniform-n18(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n18_deterministic() {
        check_det(
            &kuniform::kuniform_n18(),
            "kuniform-n18",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch for the p6m
    /// tiling (3⁶; 3².4.3.4): one 3⁶ orbit (the origin's six-triangle
    /// vertex class) and six 3².4.3.4 orbits. Counts and reps match
    /// the Python battery.
    #[test]
    fn kuniform_n18_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n18(),
            &[
                Figure {
                    figure: "333333",
                    count: 36,
                    orbits: &[(0, 0)],
                },
                Figure {
                    figure: "33434",
                    count: 260,
                    orbits: &[
                        (500, 866),
                        (500, 1866),
                        (1000, 2732),
                        (1366, 1366),
                        (1866, 2232),
                        (1866, 3232),
                    ],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n9_covers_region() {
        check_covers(
            &kuniform::kuniform_n9(),
            "kuniform-n9",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn kuniform_n9_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n9(),
            "kuniform-n9(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n9_deterministic() {
        check_det(
            &kuniform::kuniform_n9(),
            "kuniform-n9",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch for the p6m
    /// tiling (3.4.6.4; 3².4.3.4): six 3².4.3.4 orbits (the
    /// mirror-only, palindromic vertices) and six 3.4.6.4 orbits.
    /// Counts and reps match the Python battery.
    #[test]
    fn kuniform_n9_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n9(),
            &[
                Figure {
                    figure: "33434",
                    count: 234,
                    orbits: &[
                        (-2366, -1366),
                        (-1866, -500),
                        (-1366, -1366),
                        (-500, -1866),
                        (0, -2732),
                        (500, -1866),
                    ],
                },
                Figure {
                    figure: "3464",
                    count: 287,
                    orbits: &[
                        (-2732, 0),
                        (-1366, -2366),
                        (-1000, 0),
                        (-500, -866),
                        (500, -866),
                        (1366, -2366),
                    ],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n8_covers_region() {
        check_covers(
            &kuniform::kuniform_n8(),
            "kuniform-n8",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn kuniform_n8_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n8(),
            "kuniform-n8(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n8_deterministic() {
        check_det(
            &kuniform::kuniform_n8(),
            "kuniform-n8",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch for the p6m
    /// tiling (3.4.6.4; 3³.4²): six 3³.4² orbits and six 3.4.6.4
    /// orbits. Counts and reps match the Python battery.
    #[test]
    fn kuniform_n8_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n8(),
            &[
                Figure {
                    figure: "33344",
                    count: 260,
                    orbits: &[
                        (-2732, 0),
                        (-1866, -500),
                        (-1866, 500),
                        (-1366, 1366),
                        (-1366, 2366),
                        (-500, 1866),
                    ],
                },
                Figure {
                    figure: "3464",
                    count: 267,
                    orbits: &[
                        (-2732, -1000),
                        (-2732, 1000),
                        (-2232, 1866),
                        (-1000, 0),
                        (-500, 866),
                        (-500, 2866),
                    ],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n5_covers_region() {
        check_covers(
            &kuniform::kuniform_n5(),
            "kuniform-n5",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn kuniform_n5_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n5(),
            "kuniform-n5(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n5_deterministic() {
        check_det(
            &kuniform::kuniform_n5(),
            "kuniform-n5",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch for the p6m
    /// tiling (3.4.6.4; 3.4².6): the achiral 3.4.6.4 orbit ("3464")
    /// and the CHIRAL 3.4.4.6 orbit, whose figure "3446" differs from
    /// its mirror image "3644" (three figure strings, two orbits).
    /// Counts and reps match the Python battery.
    #[test]
    fn kuniform_n5_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n5(),
            &[
                Figure {
                    figure: "3446",
                    count: 266,
                    orbits: &[
                        (-1866, 3232),
                        (-500, 1866),
                        (-500, 3598),
                        (500, 866),
                        (1000, 2732),
                        (3732, 0),
                    ],
                },
                Figure {
                    figure: "3464",
                    count: 267,
                    orbits: &[
                        (1500, 866),
                        (1500, 1866),
                        (2366, 366),
                        (2366, 2366),
                        (3232, 866),
                        (3232, 1866),
                    ],
                },
                Figure {
                    figure: "3644",
                    count: 267,
                    orbits: &[
                        (-1000, 2732),
                        (-500, 866),
                        (500, 1866),
                        (500, 3598),
                        (1000, 0),
                        (1866, 3232),
                    ],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n1_covers_region() {
        check_covers(
            &kuniform::kuniform_n1(),
            "kuniform-n1",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn kuniform_n1_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n1(),
            "kuniform-n1(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n1_deterministic() {
        check_det(
            &kuniform::kuniform_n1(),
            "kuniform-n1",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch for the p6m
    /// 3-4-6-12 tiling (4.6.12; 3.4.6.4): the achiral 3.4.6.4 orbit
    /// ("3464") and the CHIRAL 4.6.12 orbit, whose figure "4612"
    /// differs from its mirror image "4126" (three figure strings, two
    /// orbits — the canonical strings are the Vec<u32> lex-min
    /// rotations; the string-concatenation ring labels are "1246"/
    /// "1264"). Counts and reps match the Python battery.
    #[test]
    fn kuniform_n1_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n1(),
            &[
                Figure {
                    figure: "3464",
                    count: 267,
                    orbits: &[
                        (866, 500),
                        (1366, 1366),
                        (1866, 500),
                        (6330, 4232),
                        (6830, 3366),
                        (7330, 4232),
                    ],
                },
                Figure {
                    figure: "4126",
                    count: 266,
                    orbits: &[
                        (2232, 1866),
                        (2732, 3732),
                        (3598, 500),
                        (4598, 4232),
                        (5464, 1000),
                        (5964, 2866),
                    ],
                },
                Figure {
                    figure: "4612",
                    count: 266,
                    orbits: &[
                        (2232, 2866),
                        (2732, 1000),
                        (3598, 4232),
                        (4598, 500),
                        (5464, 3732),
                        (5964, 1866),
                    ],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n13_covers_region() {
        check_covers(
            &kuniform::kuniform_n13(),
            "kuniform-n13",
            BBox::new(v2(-4.0, -3.0), v2(14.0, 12.0)),
        );
    }

    #[test]
    fn kuniform_n13_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n13(),
            "kuniform-n13(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n13_deterministic() {
        check_det(
            &kuniform::kuniform_n13(),
            "kuniform-n13",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch for the p6m
    /// tiling (3⁶; 3².4.12): the achiral 3⁶ orbit ("333333", the
    /// 6-fold vertices) and the CHIRAL 3².4.12 orbit, whose figure
    /// "33412" differs from its mirror image "33124" (three figure
    /// strings, two orbits — the canonical strings are the Vec<u32>
    /// lex-min rotations; the string-concatenation ring labels are
    /// "12334"/"12433"). Counts and reps match the Python battery.
    #[test]
    fn kuniform_n13_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n13(),
            &[
                Figure {
                    figure: "33124",
                    count: 240,
                    orbits: &[
                        (0, 3732),
                        (500, 1866),
                        (866, 500),
                        (1366, 3366),
                        (2232, 1866),
                        (3232, 2866),
                    ],
                },
                Figure {
                    figure: "333333",
                    count: 84,
                    orbits: &[(0, 0), (1366, 2366)],
                },
                Figure {
                    figure: "33412",
                    count: 241,
                    orbits: &[
                        (0, 1000),
                        (500, 2866),
                        (866, 4232),
                        (1366, 1366),
                        (2232, 2866),
                        (3232, 1866),
                    ],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n20_covers_region() {
        check_covers(
            &kuniform::kuniform_n20(),
            "kuniform-n20",
            BBox::new(v2(-4.0, -4.0), v2(12.0, 12.0)),
        );
    }

    #[test]
    fn kuniform_n20_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n20(),
            "kuniform-n20(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n20_deterministic() {
        check_det(
            &kuniform::kuniform_n20(),
            "kuniform-n20",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the achiral 3⁶
    /// orbit (6 translation classes per cell, 266 interior points) and
    /// the achiral 3⁴.6 orbit (6 classes, 240 interior points — the
    /// counts differ only through patch boundary truncation). Both
    /// counts and all 12 orbit reps match the Python torus battery
    /// (m96b_rust_expect.py).
    #[test]
    fn kuniform_n20_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n20(),
            &[
                Figure {
                    figure: "333333",
                    count: 266,
                    orbits: &[
                        (0, 1732),
                        (500, 2598),
                        (500, 4330),
                        (1000, 1732),
                        (1000, 3464),
                        (1500, 4330),
                    ],
                },
                Figure {
                    figure: "33336",
                    count: 240,
                    orbits: &[
                        (-500, 2598),
                        (0, 3464),
                        (500, 866),
                        (1000, 5196),
                        (1500, 2598),
                        (2000, 3464),
                    ],
                },
            ],
        );
    }

    #[test]
fn kuniform_n12_covers_region() {
        check_covers(
            &kuniform::kuniform_n12(),
            "kuniform-n12",
            BBox::new(v2(-4.0, -4.0), v2(8.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_n19_covers_region() {
        check_covers(
            &kuniform::kuniform_n19(),
            "kuniform-n19",
            BBox::new(v2(-4.0, -4.0), v2(9.0, 9.0)),
        );
    }

    #[test]
fn kuniform_n12_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n12(),
            "kuniform-n12(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(4.0, 4.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n19_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n19(),
            "kuniform-n19(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
fn kuniform_n12_deterministic() {
        check_det(
            &kuniform::kuniform_n12(),
            "kuniform-n12",
            BBox::new(v2(-3.0, -3.0), v2(6.0, 6.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the 3².6² orbit
    /// (2 translation classes per primitive cell — the hex–hex·tri–tri
    /// junctions along the shared hexagon edges, 78 interior points)
    /// and the 3⁴.6 orbit (2 classes, 84 interior points — four
    /// triangles plus a hexagon corner; the counts differ only through
    /// patch boundary truncation). Both counts and all 4 orbit reps
    /// match the Python battery (m97_rust_expect.py).
    #[test]
    fn kuniform_n12_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n12(),
            &[
                Figure {
                    figure: "3366",
                    count: 78,
                    orbits: &[(866, 500), (1732, 2000)],
                },
                Figure {
                    figure: "33336",
                    count: 84,
                    orbits: &[(866, 1500), (1732, 1000)],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n19_deterministic() {
        check_det(
            &kuniform::kuniform_n19(),
            "kuniform-n19",
            BBox::new(v2(-3.0, -3.0), v2(8.0, 8.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the achiral 3⁶
    /// orbit (2 translation classes per cell under L9 — the two petal-tip
    /// classes, 72 interior points) and the achiral 3⁴.6 orbit (6
    /// classes — the hexagon corners, 241 interior points). Both counts
    /// and all 8 orbit reps match the Python torus battery replica
    /// (m95x_rust_expect.py).
    #[test]
    fn kuniform_n19_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n19(),
            &[
                Figure {
                    figure: "333333",
                    count: 72,
                    orbits: &[(1500, 866), (3000, 1732)],
                },
                Figure {
                    figure: "33336",
                    count: 241,
                    orbits: &[
                        (500, 866),
                        (1000, 0),
                        (1000, 1732),
                        (2000, 0),
                        (2000, 1732),
                        (2500, 866),
                    ],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n4_covers_region() {
        check_covers(
            &kuniform::kuniform_n4(),
            "kuniform-n4",
            BBox::new(v2(-4.0, -4.0), v2(9.0, 9.0)),
        );
    }

    #[test]
    fn kuniform_n4_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n4(),
            "kuniform-n4(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(4.0, 4.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n4_deterministic() {
        check_det(
            &kuniform::kuniform_n4(),
            "kuniform-n4",
            BBox::new(v2(-3.0, -3.0), v2(6.0, 6.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the 3³.4² orbit
    /// (2 translation classes per cell — the square-band/triangle-band
    /// junctions, 78 interior points) and the 4⁴ orbit (1 class — the
    /// crossing of four squares inside a band, 42 interior points).
    /// Both counts and all 3 orbit reps match the Python torus battery
    /// (tku3_run.py rust_expect).
    #[test]
    fn kuniform_n4_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n4(),
            &[
                Figure {
                    figure: "33344",
                    count: 78,
                    orbits: &[(0, 0), (1000, 2000)],
                },
                Figure {
                    figure: "4444",
                    count: 42,
                    orbits: &[(1000, 1000)],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n14_covers_region() {
        check_covers(
            &kuniform::kuniform_n14(),
            "kuniform-n14",
            BBox::new(v2(-4.0, -4.0), v2(9.0, 9.0)),
        );
    }

    #[test]
    fn kuniform_n14_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n14(),
            "kuniform-n14(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(4.0, 4.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n14_deterministic() {
        check_det(
            &kuniform::kuniform_n14(),
            "kuniform-n14",
            BBox::new(v2(-3.0, -3.0), v2(6.0, 6.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the 3⁶ orbit
    /// (1 translation class per cell — the double-strip junction line,
    /// 42 interior points) and the 3³.4² orbit (2 classes — the
    /// square-row corners, 78 interior points). Both counts and all 3
    /// orbit reps match the Python torus battery (tku3_run.py
    /// rust_expect).
    #[test]
    fn kuniform_n14_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n14(),
            &[
                Figure {
                    figure: "333333",
                    count: 42,
                    orbits: &[(500, 866)],
                },
                Figure {
                    figure: "33344",
                    count: 78,
                    orbits: &[(0, 0), (0, 1732)],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n3_covers_region() {
        check_covers(
            &kuniform::kuniform_n3(),
            "kuniform-n3",
            BBox::new(v2(-4.0, -4.0), v2(9.0, 9.0)),
        );
    }

    #[test]
    fn kuniform_n3_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n3(),
            "kuniform-n3(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(4.0, 4.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n3_deterministic() {
        check_det(
            &kuniform::kuniform_n3(),
            "kuniform-n3",
            BBox::new(v2(-3.0, -3.0), v2(6.0, 6.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the 3³.4² orbit
    /// (2 translation classes per cell — the square-band/triangle-band
    /// junctions, 72 interior points) and the 4⁴ orbit (2 classes — the
    /// interior crossings of a three-row band, 84 interior points).
    /// Both counts and all 4 orbit reps match the Python exact battery
    /// (tku4_run.py rust_expect).
    #[test]
    fn kuniform_n3_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n3(),
            &[
                Figure {
                    figure: "33344",
                    count: 72,
                    orbits: &[(0, 0), (1000, 3000)],
                },
                Figure {
                    figure: "4444",
                    count: 84,
                    orbits: &[(1000, 1000), (1000, 2000)],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n15_covers_region() {
        check_covers(
            &kuniform::kuniform_n15(),
            "kuniform-n15",
            BBox::new(v2(-4.0, -4.0), v2(9.0, 9.0)),
        );
    }

    #[test]
    fn kuniform_n15_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n15(),
            "kuniform-n15(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(4.0, 4.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n15_deterministic() {
        check_det(
            &kuniform::kuniform_n15(),
            "kuniform-n15",
            BBox::new(v2(-3.0, -3.0), v2(6.0, 6.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the 3⁶ orbit
    /// (2 translation classes per cell — inside the triple strips,
    /// 78 interior points) and the 3³.4² orbit (2 classes — the
    /// square-row corners, 78 interior points). Both counts and all 4
    /// orbit reps match the Python exact battery (tku4_run.py
    /// rust_expect).
    #[test]
    fn kuniform_n15_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n15(),
            &[
                Figure {
                    figure: "333333",
                    count: 78,
                    orbits: &[(500, 2732), (1000, 1866)],
                },
                Figure {
                    figure: "33344",
                    count: 78,
                    orbits: &[(500, 0), (500, 1000)],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n2_covers_region() {
        check_covers(
            &kuniform::kuniform_n2(),
            "kuniform-n2",
            BBox::new(v2(-4.0, -4.0), v2(8.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_n2_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n2(),
            "kuniform-n2(rot30,scale0.7)",
            BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n2_deterministic() {
        check_det(
            &kuniform::kuniform_n2(),
            "kuniform-n2",
            BBox::new(v2(-3.0, -3.0), v2(6.0, 6.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the 3.12.12 orbit
    /// ("31212", the dodecagon–dodecagon kiss points, 4 translation
    /// classes, 156 interior points) and the 3.4.3.12 orbit ("34312",
    /// the square corners, 4 classes, 169 interior points). Counts and
    /// reps match the Python exact battery (tku5_n2j.py).
    #[test]
    fn kuniform_n2_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n2(),
            &[
                Figure {
                    figure: "31212",
                    count: 156,
                    orbits: &[(0, 1366), (0, 2366), (1366, 0), (2366, 0)],
                },
                Figure {
                    figure: "34312",
                    count: 169,
                    orbits: &[(500, 500), (500, 3232), (3232, 500), (3232, 3232)],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n16_covers_region() {
        check_covers(
            &kuniform::kuniform_n16(),
            "kuniform-n16",
            BBox::new(v2(-4.0, -4.0), v2(8.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_n16_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n16(),
            "kuniform-n16(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n16_deterministic() {
        check_det(
            &kuniform::kuniform_n16(),
            "kuniform-n16",
            BBox::new(v2(-3.0, -3.0), v2(6.0, 6.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the 3³.4² orbit
    /// ("33344", 4 translation classes, 182 interior points — the
    /// square-band junction vertices) and the 3².4.3.4 orbit
    /// ("33434", 8 classes, 352 interior points — snub-square-type
    /// vertices; the 4:8 class ratio matches the cell's 4 + 8
    /// vertices). Counts and reps match the Python exact battery
    /// (tku5_n16k.py), which uses the same right-handed basis
    /// b1 = (h,−h), b2 = (h,h) as the table.
    #[test]
    fn kuniform_n16_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n16(),
            &[
                Figure {
                    figure: "33344",
                    count: 182,
                    orbits: &[(866, 500), (1366, -1366), (1866, 500), (3732, 0)],
                },
                Figure {
                    figure: "33434",
                    count: 352,
                    orbits: &[
                        (0, 0),
                        (866, -500),
                        (1866, -500),
                        (1866, 1500),
                        (2366, -1366),
                        (2732, 0),
                        (2732, 1000),
                        (3232, -866),
                    ],
                },
            ],
        );
    }

    #[test]
    fn kuniform_n17_covers_region() {
        check_covers(
            &kuniform::kuniform_n17(),
            "kuniform-n17",
            BBox::new(v2(-4.0, -4.0), v2(8.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_n17_rotated_scaled() {
        check_rotated(
            &kuniform::kuniform_n17(),
            "kuniform-n17(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_n17_deterministic() {
        check_det(
            &kuniform::kuniform_n17(),
            "kuniform-n17",
            BBox::new(v2(-3.0, -3.0), v2(6.0, 6.0)),
        );
    }

    /// Interior vertex figures over the ±3×±3 patch: the 3³.4² orbit
    /// ("33344", 4 translation classes, 156 interior points) and the
    /// 3².4.3.4 orbit ("33434", 4 classes, 136 interior points).
    /// Counts and reps match the Python exact battery, which mirrors
    /// the same right-handed basis b1 = (1/2, 1+H), b2 = (−2−S3, 1)
    /// as the table.
    #[test]
    fn kuniform_n17_vertex_figures() {
        check_vertex_figures(
            &kuniform::kuniform_n17(),
            &[
                Figure {
                    figure: "33344",
                    count: 156,
                    orbits: &[(-2931, 1467), (-2931, 2467), (-1065, 967), (-1065, 1967)],
                },
                Figure {
                    figure: "33434",
                    count: 136,
                    orbits: &[(-2065, 967), (-2065, 1967), (-65, 967), (-65, 1967)],
                },
            ],
        );
    }
}

