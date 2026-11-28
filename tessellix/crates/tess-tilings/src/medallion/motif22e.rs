//! §22e motif emitters — the n = 16 / n = 10 monster figures as
//! third-layer wall segments on the two quasiperiodic substrates
//! (diss. §3.10.1 placement; §22e Stage-1 recon §6).
//!
//! # Placement (diss. §3.10.1, applied identically to both substrates)
//!
//! A regular 2n-gon, inflated to apothem = substrate edge / 2, sits at
//! **every tiling vertex**, its
//! edge midpoints on the tile edges through that vertex — the medallion
//! edge through a tile-edge midpoint is perpendicular to that edge,
//! and neighbouring medallions meet exactly at the midpoint. The
//! absolute contact-normal grid is `22.5°·k` on the AB patch (all
//! Ammann-Beenker edge directions are multiples of 45°) and `36°·k` on
//! the Penrose patch (all P3 edge directions are multiples of 36°), so
//! the record frame rotates once, globally, contact 0 onto `+x`.
//!
//! # Records (Stage-2 STOP decisions)
//!
//! * **AB** — [`Monster168`] (`{16/8}`, `(16, 3)` rung): the figure is
//!   the eight diameters joining opposite edge midpoints; the monster
//!   pair is the 16-gon pair at the two 135° corners of a 45° rhomb
//!   (separation² = `2 − √2`, the certified `c = 3` lens, recon §6.1).
//!   Only the figure outline is emitted — the interior motif stays
//!   deferred (M211 deferral: "monster interior motif stays
//!   figure-only").
//! * **Penrose** — [`Monster102`] (`{10/2}`, `(10, 3)` rung): the
//!   substrate monster is the decagon pair at the two 144° corners of
//!   a thin rhomb, separation `1/φ` on the **mid-edge axis** — the
//!   `(10, 3)` rung pins `ρ = (√5 − 1)/4` mid-edge, while `(10, 2)`
//!   (`Monster104`, `{10/4}`) pins the vertex axis, so `Monster102` is
//!   the axis-matched pick. The figure is the two pentagons `M_i →
//!   M_{i+2}` (closed 5-cycles: every contact has figure degree 2).
//!
//! # Emission
//!
//! The emitters are the pure `fn(&[Tile]) -> Vec<Wall>` third-layer
//! hooks of the [`weave`] table: every chord is emitted **once per
//! incident tile vertex**, in tile order, and the engine's
//! endpoint-canonical [`SegKey`] dedup (design §4.2) collapses the
//! duplicates — each medallion is re-emitted once per incident tile,
//! and `{16/8}` lists every diameter twice (`star_figure(16, 8)` pairs
//! `(i, i + 8)` with `(i + 8, i)`). The chords are raw `Wall`
//! geometry; tess-core names no medallion concept (design F1).

use tess_core::geometry::Vec2;
use tess_core::strapwork::Wall;
use tess_core::tiling::Tile;

use super::monster::dec10::{self, Monster102};
use super::monster::n16::{self, Monster168};
use super::Medallion;

/// The substrate edge length: both carriers emit uniform-edge tiles,
/// and the edge fixes the medallion scale (diss. §3.10.1 inflates the
/// polygons until they meet at the tile-edge midpoints — apothem =
/// edge / 2). The Penrose generator's refinement stops by a global
/// scale rule, so the patch edge is region-dependent (e.g. 0.6305 on
/// the 16×16 window) and must be **measured, not assumed**; the AB
/// patch is unit-edged at default scale and scales with `--param
/// scale`. `max` over the 4-gon carriers' first edges is exactly
/// permutation-invariant, and every 4-gon (rhomb or reassembled
/// square) has the substrate edge as its first perimeter edge.
fn substrate_edge(tiles: &[Tile]) -> f64 {
    let mut e = 0.0f64;
    for t in tiles {
        if t.poly.len() == 4 {
            let l = (t.poly.pts[1] - t.poly.pts[0]).length();
            if l > e {
                e = l;
            }
        }
    }
    if e > 0.0 {
        e
    } else {
        1.0
    }
}

/// The record-frame contacts (edge midpoints) of a unit-edge regular
/// 2n-gon centered at the origin, rotated so contact 0 points along
/// `+x` (whence every contact direction is exactly `k · 2π/n`) and
/// scaled to apothem `apothem` (diss. §3.10.1: medallions meet at the
/// tile-edge midpoints — [`substrate_edge`] / 2 on the ship path).
fn placed_contacts(hull: &[[f64; 2]], apothem: f64) -> Vec<Vec2> {
    let n = hull.len();
    let mut sum = Vec2::ZERO;
    for p in hull {
        sum = sum + Vec2::new(p[0], p[1]);
    }
    let center = sum / n as f64;
    let mid = |i: usize| {
        let a = Vec2::new(hull[i][0], hull[i][1]);
        let b = Vec2::new(hull[(i + 1) % n][0], hull[(i + 1) % n][1]);
        (a + b) * 0.5
    };
    let r0 = mid(0) - center;
    let rho = -r0.y.atan2(r0.x);
    let s = apothem / r0.length();
    (0..n)
        .map(|i| (mid(i) - center).rotated(rho) * s)
        .collect()
}

/// Emit the record figure at every vertex of every tile (the vertex-
/// centered placement of diss. §3.10.1), medallions inflated to
/// apothem = substrate edge / 2. Order: tile order, then
/// polygon-vertex order, then figure-pair order — deterministic and
/// pure; duplicate chords collapse in the engine's [`SegKey`] dedup.
fn figure_walls(hull: &[[f64; 2]], fig: &[(usize, usize)], tiles: &[Tile]) -> Vec<Wall> {
    let contacts = placed_contacts(hull, substrate_edge(tiles) * 0.5);
    let cap: usize = tiles.iter().map(|t| t.poly.len()).sum();
    let mut out = Vec::with_capacity(cap * fig.len());
    for t in tiles {
        for &v in &t.poly.pts {
            for &(i, j) in fig {
                out.push(Wall {
                    a: v + contacts[i],
                    b: v + contacts[j],
                });
            }
        }
    }
    out
}

/// The Ammann-Beenker motif layer: `{16/8}` hexadecagon monster
/// diameters at every AB vertex (16 emitted pairs per vertex — the
/// eight diameters listed twice, `star_figure(16, 8)`).
pub fn ab_extra_walls(tiles: &[Tile]) -> Vec<Wall> {
    figure_walls(&Monster168::hull_f64(), &n16::figure(), tiles)
}

/// The Penrose motif layer: `{10/2}` decagon monster pentagons at
/// every P3 vertex (10 chords per vertex, two closed 5-cycles).
pub fn penrose_extra_walls(tiles: &[Tile]) -> Vec<Wall> {
    figure_walls(&Monster102::hull_f64(), &dec10::figure2(), tiles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tess_core::geometry::Polygon;

    /// sin 36° = √(10 − 2√5)/4.
    const SIN36: f64 = 0.587_785_252_292_473_1;
    /// cos 72° = (√5 − 1)/4.
    const COS72: f64 = 0.309_016_994_374_947_45;

    fn square() -> Tile {
        Tile {
            proto: 0,
            seed: 0,
            level: 0,
            poly: Polygon::from_slice(&[
                Vec2::new(0.0, 0.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(0.0, 1.0),
            ]),
        }
    }

    /// The AB frame: 16 contacts on the apothem-½ circle, directions
    /// exactly `22.5°·k`, opposite contacts antipodal (mod-16 symmetry
    /// of the record frame, f64 rounding pinned at 1e-12).
    #[test]
    fn ab_contacts_land_on_the_normal_grid() {
        let c = placed_contacts(&Monster168::hull_f64(), 0.5);
        assert_eq!(c.len(), 16);
        for (i, p) in c.iter().enumerate() {
            assert!((p.length() - 0.5).abs() < 1e-12, "contact {i} radius: {}", p.length());
            let want = i as f64 * std::f64::consts::FRAC_PI_8;
            let mut d = p.y.atan2(p.x) - want;
            while d > std::f64::consts::PI {
                d -= std::f64::consts::TAU;
            }
            while d < -std::f64::consts::PI {
                d += std::f64::consts::TAU;
            }
            assert!(d.abs() < 1e-12, "contact {i} angle drift {d:e}");
            let q = c[(i + 8) % 16];
            assert!((p.x + q.x).abs() < 1e-12 && (p.y + q.y).abs() < 1e-12, "contact {i} antipode");
        }
    }

    /// The `{16/8}` figure on the placed frame: 16 pairs, 8 distinct
    /// chords, each a unit diameter centered on the medallion vertex.
    #[test]
    fn ab_figure_is_eight_unit_diameters() {
        let c = placed_contacts(&Monster168::hull_f64(), 0.5);
        let fig = n16::figure();
        assert_eq!(fig.len(), 16);
        let mut distinct = std::collections::BTreeSet::new();
        for &(i, j) in &fig {
            distinct.insert((i.min(j), i.max(j)));
            let chord = c[j] - c[i];
            assert!((chord.length() - 1.0).abs() < 1e-12, "diameter {i}-{j} length {}", chord.length());
            assert!((c[i] + c[j]).length() < 1e-12, "diameter {i}-{j} off-centre");
        }
        assert_eq!(distinct.len(), 8, "{{16/8}} lists each diameter twice");
    }

    /// The Penrose frame: 10 contacts on the apothem-½ circle,
    /// directions exactly `36°·k`.
    #[test]
    fn penrose_contacts_land_on_the_normal_grid() {
        let c = placed_contacts(&Monster102::hull_f64(), 0.5);
        assert_eq!(c.len(), 10);
        for (i, p) in c.iter().enumerate() {
            assert!((p.length() - 0.5).abs() < 1e-12, "contact {i} radius");
            let want = i as f64 * std::f64::consts::TAU / 10.0;
            let mut d = p.y.atan2(p.x) - want;
            while d > std::f64::consts::PI {
                d -= std::f64::consts::TAU;
            }
            while d < -std::f64::consts::PI {
                d += std::f64::consts::TAU;
            }
            assert!(d.abs() < 1e-12, "contact {i} angle drift {d:e}");
        }
    }

    /// The `{10/2}` figure: 10 distinct chords, each `M_i → M_{i+2}`
    /// has length sin 36° and subtends exactly 72° at the medallion
    /// vertex — the (10, 3) rung's chord class.
    #[test]
    fn penrose_figure_chords_subtend_72_degrees() {
        let c = placed_contacts(&Monster102::hull_f64(), 0.5);
        let fig = dec10::figure2();
        assert_eq!(fig.len(), 10);
        let mut distinct = std::collections::BTreeSet::new();
        for &(i, j) in &fig {
            assert_eq!((j + 10 - i) % 10, 2, "figure step");
            distinct.insert((i.min(j), i.max(j)));
            let (u, v) = (c[i], c[j]);
            let len = (v - u).length();
            assert!((len - SIN36).abs() < 1e-12, "chord {i}-{j} length {len}");
            let cos = u.dot(v) / (u.length() * v.length());
            assert!((cos - COS72).abs() < 1e-12, "chord {i}-{j} subtends {cos}");
        }
        assert_eq!(distinct.len(), 10, "{{10/2}} is 10 distinct chords (two pentagons)");
    }

    /// Emitter census on one unit square: 16 raw AB chords per vertex
    /// and 10 raw Penrose chords per vertex; every raw chord is
    /// centered on one of the four vertices.
    #[test]
    fn emitter_census_and_chord_centers() {
        let ts = [square()];
        let ab = ab_extra_walls(&ts);
        assert_eq!(ab.len(), 4 * 16);
        let pen = penrose_extra_walls(&ts);
        assert_eq!(pen.len(), 4 * 10);
        let verts = [
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ];
        for w in &ab {
            let m = (w.a + w.b) * 0.5;
            assert!(
                verts.iter().any(|v| (m - *v).length() < 1e-9),
                "AB diameter not centered on a tile vertex: {:?} {:?}",
                w.a,
                w.b
            );
        }
        for w in &pen {
            // {10/2} chords end on the apothem-½ contact circle about
            // their medallion vertex (the midpoint is off-vertex).
            for p in [w.a, w.b] {
                assert!(
                    verts
                        .iter()
                        .any(|v| ((p - *v).length() - 0.5).abs() < 1e-9),
                    "Penrose chord endpoint off the contact circle: {p:?}"
                );
            }
        }
    }

    /// Purity: the emitters are functions of the tile list alone —
    /// repeated calls are bit-identical.
    #[test]
    fn emitters_are_pure() {
        let ts = [square()];
        let a = ab_extra_walls(&ts);
        let b = ab_extra_walls(&ts);
        assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(&b) {
            assert!(x.a.x == y.a.x && x.a.y == y.a.y && x.b.x == y.b.x && x.b.y == y.b.y);
        }
    }

    /// Scale covariance: the medallions are inflated to apothem =
    /// substrate edge / 2, so a patch at edge e has chords of exactly
    /// `e` (AB diameters) and `e · sin 36°` (Penrose {10/2}), and the
    /// Penrose monster pair separation `e / φ` is the similar figure
    /// of the certified unit-edge placement.
    #[test]
    fn emitter_scales_with_the_substrate_edge() {
        let e = 0.630_5;
        let scaled = [Tile {
            proto: 0,
            seed: 0,
            level: 0,
            poly: Polygon::from_slice(&[
                Vec2::new(0.0, 0.0),
                Vec2::new(e, 0.0),
                Vec2::new(e, e),
                Vec2::new(0.0, e),
            ]),
        }];
        for w in ab_extra_walls(&scaled) {
            assert!(((w.b - w.a).length() - e).abs() < 1e-12);
        }
        for w in penrose_extra_walls(&scaled) {
            assert!(((w.b - w.a).length() - e * SIN36).abs() < 1e-12);
        }
    }
}
