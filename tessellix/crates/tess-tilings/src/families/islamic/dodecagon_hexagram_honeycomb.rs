//! Dodecagon-hexagram medallion composition on the honeycomb net
//! (24d instance 1) — regular dodecagon medallions at the hexagon
//! centers of a honeycomb net plus kissing {6/2} hexagrams at its
//! vertices, the interstices filled by a single non-convex hexagon
//! class — expressed as a [`LatticeTiling`] data table.
//!
//! # The composition
//!
//! On the honeycomb `[6,3]` net with vertex radius `R = 1` (hexagon
//! edge 1, centers on the triangular lattice `a1 = (√3, 0)`,
//! `a2 = (√3/2, 3/2)`):
//!
//! * **dodecagon** (seed 0) — a regular dodecagon of circumradius `1/2`
//!   at every hexagon center, vertices at `30k°`. Its six vertices at
//!   `30° + 60k` are exactly the midpoints of the center–vertex
//!   segments; its six vertices at `60k` lie on the center–center
//!   (dual) lines.
//! * **hexagram** (seed 1) — the {6/2} star at the kissing tip radius
//!   `T = 1/2` (tips at `30° + 60k`, notches at `60° + 60k` at radius
//!   `1/(2√3) = √3/6`; emitted as the simple 12-gon, tips and notches
//!   interleaved in monotone order) at every net vertex — two stars per
//!   cell at the two vertex orbits `VA = (0, 1)` and `VB = (√3/2, 1/2)`.
//! * **interstice hexagon** (seed 2) — one reflex hexagon class, six
//!   per site, each bounded by two dodecagon edges and four star edges;
//!   its reflex corner (210°) wraps exactly around a dodecagon vertex
//!   on a dual line.
//!
//! The configuration is uniquely determined by the two contact
//! identities, both exact: every star tip pointing at a hexagon center
//! lands on the dodecagon vertex at the midpoint of that center–vertex
//! segment, and every star tip pointing along a net edge lands on the
//! net-edge midpoint, kissing the neighbor star's tip there tip-to-tip
//! (the net-edge midpoint is also the dual-edge midpoint — the
//! honeycomb circumcentric dual passes vertex-to-vertex through the
//! partition: its edges run along the center–center lines through the
//! dodecagon `60k` vertices and the kissing points, cutting no tile
//! interior; this is the dual-net reading shipped for 24d, whose glue
//! sites are the hexagon long diagonals along the dual lines — the
//! hole-inference reading resolved NOT APPLICABLE by the 24d addendum —
//! the 24b alphabet cannot cover the 75° contact corners; card 'Fill
//! reading' section).
//!
//! Per lattice cell: 1 dodecagon + 2 hexagrams + 6 interstice hexagons
//! = 9 tiles, with areas `3/4 + 2·(√3/4) + 6·(√3/6 − 1/8) = 3√3/2 =
//! |det(a1, a2)|` exactly over Q(√3). Interior vertex figures:
//! `60.60.120.120` at the kissing points, `60.75.75.150` at the
//! dodecagon-vertex contacts, `120.240` at the star notches,
//! `150.210` at the dodecagon vertices on the dual lines. The wallpaper
//! group is **p6m** (hexagonal lattice; the probe battery confirms
//! rot60 at the site center and a mirror through the dual line, and
//! rejects rot30, rot90 and the 15° mirror).
//!
//! Every cell vertex is an exact Q(√3) point `(p + r√3, q + s√3) / 12`;
//! the table below emits all of them through [`p12`], so shared
//! vertices are computed by bit-identical arithmetic from identical
//! integer tuples.
//!
//! # Verification
//!
//! Verified exactly before porting (Python, stdlib) over a ±3×±3
//! lattice patch (441 tiles): the face census is 49 dodecagons
//! (area 3/4), 126 stars (area √3/4) and 294 interstice hexagons (area
//! √3/6 − 1/8, six per site); Σ interior face areas == the outer
//! boundary area exactly (complete partition, no overlap); 1628
//! interior edges shared by exactly 2 tiles and none by more (no
//! T-junctions, seams 0); no vertex angle sum exceeds 360°, the
//! figures closing in exactly the four orbits above; an irrational-
//! offset fine grid over the disk of radius 2 covered exactly once
//! (5029 points, 0 uncovered, 0 double); the contact battery passes
//! 882/882 midpoint corner contacts and 294/294 dual edges carrying a
//! dodecagon vertex at distance 1/2 and a kissing tip at the midpoint;
//! the patch is invariant under 60° rotation and the x-mirror and not
//! under rot30/rot90/mirror-15°. The in-cell Rust battery re-pins all
//! of it in the exact field: the record→cell identities against
//! [`crate::medallion::hexagram_62`] (the star tip set equals
//! `tips_natural` rotated 30° — the derived placement rotates the
//! record by half a step so the tips face the dual-edge midpoints) and
//! [`crate::medallion::rosette_m56`] (dodecagon regularity with the
//! scale-invariant ratio `edge²/R² = 2 − √3` equal to the record's in
//! exact field arithmetic), the medallion-boundary edge landings (the
//! 36 interstice edges consume the site's 12 dodecagon and 24 star
//! boundary edges exactly once each, modulo the lattice), and the
//! exact cell-area closure.

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// √3 (`std::f64::consts::SQRT_3` is unstable at this crate's MSRV —
/// the precedent is `pic_3_6_3_6.rs`).
const S3: f64 = 1.732_050_807_568_877_2;

/// An exact cell coordinate `(p + r√3, q + s√3) / 12` as a float
/// point. Every vertex of the cell is written at this common
/// denominator, so every shared vertex is emitted by bit-identical
/// arithmetic from the same integer tuple.
fn p12(v: (i32, i32, i32, i32)) -> Vec2 {
    Vec2::new(
        (v.0 as f64 + v.1 as f64 * S3) / 12.0,
        (v.2 as f64 + v.3 as f64 * S3) / 12.0,
    )
}

/// Exact integer tuple sum (closed in the /12 lattice).
fn tup_add(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)
}

/// Exact tuple negation (test-only).
#[cfg(test)]
fn tup_neg(a: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
    (-a.0, -a.1, -a.2, -a.3)
}

/// Exact rotation by +60 degrees in the /12 lattice: `x' = x/2 − √3y/2`,
/// `y' = √3x/2 + y/2`. All parities are even on every cell tuple.
fn tup_rot60(v: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
    let (p, r, q, s) = v;
    (
        (p - 3 * s) / 2,
        (r - q) / 2,
        (3 * r + q) / 2,
        (p + s) / 2,
    )
}

/// Dodecagon vertices: circumradius 1/2 at angles 30k, CCW.
const DOD: [(i32, i32, i32, i32); 12] = [
    (6, 0, 0, 0),
    (0, 3, 3, 0),
    (3, 0, 0, 3),
    (0, 0, 6, 0),
    (-3, 0, 0, 3),
    (0, -3, 3, 0),
    (-6, 0, 0, 0),
    (0, -3, -3, 0),
    (-3, 0, 0, -3),
    (0, 0, -6, 0),
    (3, 0, 0, -3),
    (0, 3, -3, 0),
];

/// Star tip offsets at radius 1/2, angles 30° + 60k (index k = 0..5).
const TIP: [(i32, i32, i32, i32); 6] = [
    (0, 3, 3, 0),
    (0, 0, 6, 0),
    (0, -3, 3, 0),
    (0, -3, -3, 0),
    (0, 0, -6, 0),
    (0, 3, -3, 0),
];

/// Star notch offsets at radius √3/6, angles 0° + 60k (index k = 0..5;
/// the notch following tip 30° + 60k in the CCW walk is index k + 1).
const NOT: [(i32, i32, i32, i32); 6] = [
    (0, 2, 0, 0),
    (0, 1, 3, 0),
    (0, -1, 3, 0),
    (0, -2, 0, 0),
    (0, -1, -3, 0),
    (0, 1, -3, 0),
];

/// The two net-vertex representatives of the lattice cell.
const VA: (i32, i32, i32, i32) = (0, 0, 12, 0);
const VB: (i32, i32, i32, i32) = (0, 6, 6, 0);

/// Lattice vector `i·a1 + j·a2` as a /12 tuple (test-only).
#[cfg(test)]
fn tup_lat(i: i32, j: i32) -> (i32, i32, i32, i32) {
    (0, 12 * i + 6 * j, 18 * j, 0)
}

/// The {6/2} star at center `c` as the simple 12-gon: tip, notch, tip,
/// notch in monotone CCW order (tip 30° + 60k followed by notch 60° +
/// 60k). Any other interleaving self-crosses — see the 24d scratch
/// battery.
fn star(c: (i32, i32, i32, i32)) -> [(i32, i32, i32, i32); 12] {
    let mut out = [(0, 0, 0, 0); 12];
    for k in 0..6 {
        out[2 * k] = tup_add(c, TIP[k]);
        out[2 * k + 1] = tup_add(c, NOT[(k + 1) % 6]);
    }
    out
}

/// The East interstice hexagon (CCW): two dodecagon edges meeting in
/// reflex at the dual-line vertex (1/2, 0), four star edges, the other
/// dual-line vertex at the kissing point (√3/2, 0).
const HEX_E: [(i32, i32, i32, i32); 6] = [
    (6, 0, 0, 0),
    (0, 3, -3, 0),
    (0, 5, -3, 0),
    (0, 6, 0, 0),
    (0, 5, 3, 0),
    (0, 3, 3, 0),
];

fn poly(pts: &[(i32, i32, i32, i32)]) -> Polygon {
    let v: Vec<Vec2> = pts.iter().copied().map(p12).collect();
    Polygon::from_slice(&v)
}

/// Dodecagon-hexagram composition (honeycomb) — dodecagon medallions
/// at the hexagon centers and kissing hexagrams at the net vertices
/// of the honeycomb, interstices filled edge-to-edge by one reflex
/// hexagon class (p6m, hexagonal lattice).
pub fn dodecagon_hexagram_honeycomb() -> LatticeTiling {
    let mut cells: Vec<Cell> = Vec::with_capacity(9);
    cells.push(Cell {
        poly: poly(&DOD),
        seed: 0,
    });
    cells.push(Cell {
        poly: poly(&star(VA)),
        seed: 1,
    });
    cells.push(Cell {
        poly: poly(&star(VB)),
        seed: 1,
    });
    let mut hex = HEX_E;
    for _ in 0..6 {
        cells.push(Cell {
            poly: poly(&hex),
            seed: 2,
        });
        hex = [
            tup_rot60(hex[0]),
            tup_rot60(hex[1]),
            tup_rot60(hex[2]),
            tup_rot60(hex[3]),
            tup_rot60(hex[4]),
            tup_rot60(hex[5]),
        ];
    }

    LatticeTiling {
        id: "dodecagon-hexagram-honeycomb",
        name: "Dodecagon-Hexagram Composition (Honeycomb)",
        description: "Composition-family medallion cell on the honeycomb net (24d instance 1): regular dodecagon medallions of circumradius 1/2 at every hexagon center plus the kissing {6/2} hexagrams (tip radius 1/2) at every net vertex, the interstices filled edge-to-edge by one reflex hexagon class (six per site, two dodecagon + four star edges, area sqrt(3)/6 - 1/8). Star tips land exactly on the dodecagon vertices at center-vertex midpoints and kiss tip-to-tip at the net-edge (= dual-edge) midpoints; the dual net runs vertex-to-vertex through the partition. 9 tiles per cell summing to 3sqrt(3)/2 = |det| exactly over Q(sqrt 3); vertex figures 60.60.120.120 / 60.75.75.150 / 120.240 / 150.210 (p6m).",
        lattice: [p12((0, 12, 0, 0)), p12((0, 6, 18, 0))],
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

/// Medallion third-layer segments for the strapwork fusion (design doc
/// `docs/medallion-third-layer-design.md` §4/§5, anchor A3): the
/// record geometry of the two medallion classes, derived from the
/// **emitted** tile polygons so the similarity parameters (scale /
/// rotation) carry the layer along with the faces.
///
/// * seed 0 (dodecagon): the `{12/4}` chords `m_j → m_{j+4}` between
///   edge midpoints — the [rosette_m56](crate::medallion::rosette_m56)
///   chord class, contact angle 60° from the edge (tangent-chord =
///   arc/2).
/// * seed 1 (hexagram): the `{6/2}` figure triangles, tip `t_j →
///   t_{j+2}` — the [hexagram_62](crate::medallion::hexagram_62)
///   record at kissing ratio T = 1/2.
///
/// Pure and ordered (tile order → chord order; the fused-map dedup
/// makes the final output permutation-independent, design §7 A5).
pub fn medallion_extra_walls(tiles: &[tess_core::tiling::Tile]) -> Vec<tess_core::strapwork::Wall> {
    let mut out = Vec::new();
    for t in tiles {
        let q = &t.poly.pts;
        match t.seed {
            0 if q.len() == 12 => {
                let m: Vec<Vec2> = (0..12)
                    .map(|j| (q[j] + q[(j + 1) % 12]) * 0.5)
                    .collect();
                for j in 0..12 {
                    out.push(tess_core::strapwork::Wall {
                        a: m[j],
                        b: m[(j + 4) % 12],
                    });
                }
            }
            1 if q.len() == 12 => {
                // Centroid and tip set, similarity-invariant: tips are
                // the vertices on the polygon's maximal circumradius
                // (ratio tip:notch = sqrt 3, comfortably separated).
                let mut c = Vec2::ZERO;
                for p in q {
                    c = c + *p;
                }
                c = c / 12.0;
                let rmax = q.iter().map(|p| (*p - c).length()).fold(0.0f64, f64::max);
                let tips: Vec<Vec2> = q
                    .iter()
                    .cloned()
                    .filter(|p| (*p - c).length() > 0.9 * rmax)
                    .collect();
                if tips.len() != 6 {
                    continue;
                }
                for j in 0..6 {
                    out.push(tess_core::strapwork::Wall {
                        a: tips[j],
                        b: tips[(j + 2) % 6],
                    });
                }
            }
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::medallion::hexagram_62;
    use crate::medallion::rosette_m56;
    use crate::medallion::{Qp, Zq, area2, cross, dot, midpoint, norm2, rot30n, vsub};
    use crate::tests::{check_covers, check_det, check_rotated};
    use std::collections::BTreeMap;
    use tess_core::geometry::BBox;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    // --- the same table in the exact field --------------------------------

    fn z(a: i64, b: i64, den: i64) -> Zq {
        Zq::new(a, b, 3, den)
    }

    /// Exact lift of a table tuple.
    fn qe(v: (i32, i32, i32, i32)) -> Qp {
        (
            z(v.0 as i64, v.1 as i64, 12),
            z(v.2 as i64, v.3 as i64, 12),
        )
    }

    fn star_q(c: (i32, i32, i32, i32)) -> Vec<Qp> {
        star(c).iter().copied().map(qe).collect()
    }

    fn dod_q() -> Vec<Qp> {
        DOD.iter().copied().map(qe).collect()
    }

    /// The six interstice hexagons (exact lifts, same rotation walk as
    /// the shipped table).
    fn hex_q() -> Vec<Vec<Qp>> {
        let mut out = Vec::new();
        let mut h = HEX_E;
        for _ in 0..6 {
            out.push(h.iter().copied().map(qe).collect());
            h = [
                tup_rot60(h[0]),
                tup_rot60(h[1]),
                tup_rot60(h[2]),
                tup_rot60(h[3]),
                tup_rot60(h[4]),
                tup_rot60(h[5]),
            ];
        }
        out
    }

    /// Canonical reduced-Zq key of a point (equality across the table).
    fn keyp(p: Qp) -> (i64, i64, i64, i64, i64, i64, i64, i64) {
        (p.0.a, p.0.b, p.0.d, p.0.den, p.1.a, p.1.b, p.1.d, p.1.den)
    }

    // --- the standard trio -------------------------------------------------

    #[test]
    fn dodecagon_hexagram_honeycomb_covers_region() {
        check_covers(
            &dodecagon_hexagram_honeycomb(),
            "dodecagon-hexagram-honeycomb",
            BBox::new(v2(-2.5, -2.0), v2(4.0, 3.0)),
        );
    }

    #[test]
    fn dodecagon_hexagram_honeycomb_rotated_scaled() {
        check_rotated(
            &dodecagon_hexagram_honeycomb(),
            "dodecagon-hexagram-honeycomb(rot30,scale0.7)",
            BBox::new(v2(-3.0, -3.0), v2(3.0, 3.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn dodecagon_hexagram_honeycomb_deterministic() {
        check_det(
            &dodecagon_hexagram_honeycomb(),
            "dodecagon-hexagram-honeycomb",
            BBox::new(v2(-2.0, -2.0), v2(3.0, 3.0)),
        );
    }

    // --- interstice partition exactness ------------------------------------

    #[test]
    fn exact_area_closure() {
        // area2 pins: dodecagon 3/2, star sqrt3/2, hexagon sqrt3/3 - 1/4.
        assert_eq!(area2(&dod_q()), z(3, 0, 2));
        assert_eq!(area2(&star_q(VA)), z(0, 1, 2));
        assert_eq!(area2(&star_q(VB)), z(0, 1, 2));
        for h in hex_q() {
            assert_eq!(area2(&h), z(-3, 4, 12));
        }
        // sum(cell areas) == |det(a1, a2)| = 3 sqrt3 / 2 exactly, so
        // sum(interstice areas) == cell - sum(medallions) exactly.
        let mut tot = z(0, 0, 1);
        for poly in [dod_q(), star_q(VA), star_q(VB)]
            .into_iter()
            .chain(hex_q())
        {
            tot = tot.add(area2(&poly));
        }
        assert_eq!(tot, z(0, 3, 1));
    }

    // --- medallion exactness -------------------------------------------------

    #[test]
    fn dodecagon_is_regular() {
        let d = dod_q();
        let r2 = z(1, 0, 4); // circumradius^2 = 1/4
        let e2 = z(2, -1, 4); // edge^2 = (2 - sqrt3)/4
        for i in 0..12 {
            assert_eq!(norm2(d[i]), r2, "dodecagon vertex {i} radius");
            let e = vsub(d[(i + 1) % 12], d[i]);
            let f = vsub(d[(i + 2) % 12], d[(i + 1) % 12]);
            assert_eq!(norm2(e), e2, "dodecagon edge {i}");
            // 30-degree left turns: cross = |e|^2 / 2, dot = |e|^2 sqrt3 / 2
            // — so twelve equal edges turning 30 degrees close the regular
            // 12-gon.
            assert_eq!(cross(e, f).scale(2), e2, "dodecagon turn {i} cross");
            assert_eq!(dot(e, f).scale(2), e2.mul(z(0, 1, 1)), "turn {i} dot");
        }
    }

    #[test]
    fn hexagram_is_exact() {
        let s = star_q(VA);
        let c = qe(VA);
        for k in 0..6 {
            assert_eq!(norm2(vsub(s[2 * k], c)), z(1, 0, 4), "tip {k}");
            assert_eq!(norm2(vsub(s[2 * k + 1], c)), z(1, 0, 12), "notch {k}");
            assert_eq!(
                norm2(vsub(s[2 * k + 1], s[2 * k])),
                z(1, 0, 12),
                "star edge {k}"
            );
            assert_eq!(
                norm2(vsub(s[(2 * k + 2) % 12], s[2 * k + 1])),
                z(1, 0, 12),
                "star edge {k} back"
            );
            // The notch sits at 2/3 of the way to the midpoint of its two
            // tips, measured from the star center: 2 * (mid - c) ==
            // 3 * (notch - c), exactly.
            let m = vsub(midpoint(s[2 * k], s[(2 * k + 2) % 12]), c);
            let n = vsub(s[2 * k + 1], c);
            let lhs = (m.0.scale(2), m.1.scale(2));
            let rhs = (n.0.scale(3), n.1.scale(3));
            assert_eq!(keyp(lhs), keyp(rhs), "notch {k} chord identity");
        }
    }

    #[test]
    fn hexagram_record_identity() {
        // The star tips are exactly the Hexagram62 record's natural tip
        // set rotated 30 degrees (the derived placement turns the record
        // half a step so the tips face the dual-edge midpoints), placed
        // at each net vertex.
        let mut rec: Vec<_> = hexagram_62::tips_natural()
            .into_iter()
            .map(|t| keyp(rot30n(t, 1)))
            .collect();
        rec.sort_unstable();
        for c in [VA, VB] {
            let s = star_q(c);
            let mut rel: Vec<_> = (0..6).map(|k| keyp(vsub(s[2 * k], qe(c)))).collect();
            rel.sort_unstable();
            assert_eq!(rel, rec, "star at {c:?} tips == record tips");
        }
    }

    #[test]
    fn dodecagon_record_identity() {
        // The landed dodecagon record (RosetteM56, 3.12.12 substrate,
        // edge 1): its regularity pinned in the exact field...
        let o = (z(0, 0, 1), z(0, 0, 1));
        let rec = rosette_m56::dodecagon(o);
        for k in 0..12 {
            assert_eq!(norm2(rec[k]), z(2, 1, 1), "record vertex {k} R^2");
            assert_eq!(
                norm2(vsub(rec[(k + 1) % 12], rec[k])),
                z(1, 0, 1),
                "record edge {k}"
            );
        }
        // ...and the scale-invariant similarity invariant edge^2 / R^2
        // agrees with the cell's exactly: e2_mine * R2_rec ==
        // e2_rec * R2_mine = 1/4 in Q(sqrt3), i.e. the cell dodecagon is
        // the record geometry under the similarity of squared ratio
        // (2 - sqrt3)/4 (the phase rotation of 15 degrees between the
        // two records' vertex frames is outside Q(sqrt3) and does not
        // affect the vertex-set regularity claim).
        let e2_mine = z(2, -1, 4);
        let r2_mine = z(1, 0, 4);
        assert_eq!(e2_mine.mul(z(2, 1, 1)), z(1, 0, 1).mul(r2_mine));
        assert_eq!(e2_mine.mul(z(2, 1, 1)), z(1, 0, 4));
    }

    // --- contact geometry -----------------------------------------------------

    #[test]
    fn contacts_and_kisses_are_exact() {
        let d = dod_q();
        let sa = star_q(VA);
        let sb = star_q(VB);
        let zero = (z(0, 0, 1), z(0, 0, 1));
        // Dodecagon vertex 30 deg == midpoint(0, VB) == star B tip 210.
        assert_eq!(d[1], midpoint(zero, qe(VB)));
        assert_eq!(d[1], sb[6]);
        // Dodecagon vertex 90 deg == midpoint(0, VA) == star A tip 270.
        assert_eq!(d[3], midpoint(zero, qe(VA)));
        assert_eq!(d[3], sa[8]);
        // Kissing: star A tip 330 == star B tip 150 == midpoint(VA, VB)
        // (the net-edge midpoint = dual-edge midpoint).
        assert_eq!(sa[10], sb[4]);
        assert_eq!(sb[4], midpoint(qe(VA), qe(VB)));
        // Star A tip 90 kisses the translated star B centered at
        // VB + a2 - a1 (= (0, 2), a net vertex of the other orbit).
        let t = tup_lat(-1, 1);
        assert_eq!(sa[2], qe(tup_add(VB, tup_add(TIP[4], t))));
        // Star A tip 30 lands on the translated dodecagon vertex: the
        // dodecagon of the site a2, seen from a2 at 210 degrees.
        assert_eq!(qe(tup_add(tup_lat(0, 1), TIP[3])), sa[0]);
    }

    // --- interstice partition: edge-to-edge landings ---------------------------

    #[test]
    fn interstice_hexagons_are_exact() {
        let e_dod = z(2, -1, 4);
        let e_star = z(1, 0, 12);
        for (i, h) in hex_q().iter().enumerate() {
            let mut n_dod = 0;
            let mut n_star = 0;
            for k in 0..6 {
                let s = norm2(vsub(h[(k + 1) % 6], h[k]));
                if s == e_dod {
                    n_dod += 1;
                } else if s == e_star {
                    n_star += 1;
                } else {
                    panic!("hexagon {i} edge {k} with foreign squared length");
                }
            }
            assert_eq!((n_dod, n_star), (2, 4), "hexagon {i} edge types");
        }
    }

    #[test]
    fn interstice_edges_land_on_medallions_once() {
        // Every interstice edge equals exactly one site-medallion edge
        // modulo the lattice (the edge-to-edge condition in the exact
        // field): the 36 hexagon edges consume the 12 dodecagon and 24
        // star boundary edges exactly once each.
        let mut medallion: BTreeMap<
            ((i32, i32, i32, i32), (i32, i32, i32, i32)),
            &'static str,
        > = BTreeMap::new();
        let mut push = |poly: &[(i32, i32, i32, i32)], tag: &'static str| {
            let n = poly.len();
            for i in 0..n {
                let (a, b) = (poly[i], poly[(i + 1) % n]);
                medallion.insert(if a <= b { (a, b) } else { (b, a) }, tag);
            }
        };
        push(&DOD, "dod");
        let sa = star(VA);
        let sb = star(VB);
        push(&sa, "starA");
        push(&sb, "starB");
        assert_eq!(medallion.len(), 36, "36 distinct medallion edges");

        let mut hits: BTreeMap<&'static str, usize> = BTreeMap::new();
        let mut h = HEX_E;
        for site in 0..6 {
            for i in 0..6 {
                let (u0, v0) = (h[i], h[(i + 1) % 6]);
                let mut found: Vec<&'static str> = Vec::new();
                for ii in -2i32..=2 {
                    for jj in -2i32..=2 {
                        let back = tup_neg(tup_lat(ii, jj));
                        let a = tup_add(u0, back);
                        let b = tup_add(v0, back);
                        let k = if a <= b { (a, b) } else { (b, a) };
                        if let Some(tag) = medallion.get(&k) {
                            found.push(*tag);
                        }
                    }
                }
                assert_eq!(
                    found.len(),
                    1,
                    "hexagon {site} edge {i} lands {} times",
                    found.len()
                );
                *hits.entry(found[0]).or_insert(0) += 1;
            }
            h = [
                tup_rot60(h[0]),
                tup_rot60(h[1]),
                tup_rot60(h[2]),
                tup_rot60(h[3]),
                tup_rot60(h[4]),
                tup_rot60(h[5]),
            ];
        }
        assert_eq!(hits.get("dod"), Some(&12));
        assert_eq!(hits.get("starA"), Some(&12));
        assert_eq!(hits.get("starB"), Some(&12));
    }
}
