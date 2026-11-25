//! The 3.12.12 host: bind the M158 WIS star to a unit-dodecagon hole
//! and absorb all 24 exit rays exactly (design doc §7 — the pipeline
//! §24b consumes first).
//!
//! The host patch is the exact local 3.12.12 neighbourhood of one
//! dodecagonal hole: hole vertices `w_k` at angles 15 + 30k (unit edge,
//! circumradius^2 `2 + sqrt3`); six dodecagons across the odd hole
//! edges (centres `(2 + sqrt3) * unit(30(k+1))`, the same vertex
//! placement translated); six unit triangles on the even hole edges
//! whose apex is the vertex shared by the two neighbouring dodecagons.
//!
//! The binding is fixed by the contact data alone (doc §2): the medallion
//! records corner contacts on a unit-edge hull and the hole supplies the
//! corner frame, so the similarity is `S(p) = rot30^4(p) + w_0` — no
//! scale factor is ever computed in the field. Under `S` the twelve
//! hull vertices land exactly on the twelve hole vertices `w_i` and the
//! centre lands at the hole centre (origin).
//!
//! Absorption (path (c), spike-certified): every bound exit ray,
//! marched with [`first_hit`] over the twelve declared tiles, lands at
//! parameter exactly `1 + sqrt3` (12 rays) or `2 + sqrt3` (12 rays) —
//! always on a dodecagon VERTEX (4 per neighbour, 0 on the triangles),
//! so the ray terminates on the PIC contact lattice of the host: the
//! star's rays continue as the host network's own lines.

use super::*;
use std::collections::BTreeMap;

/// Hole vertices `w_k` as raw `(x2, y2)` numerator pairs over 2 in
/// `Q(sqrt3)`: `w_k` at angle 15 + 30k (the certified table, spike
/// `host_patch()`; `|w_k|^2 = 2 + sqrt3`).
pub const HOLE_TBL: [(i64, i64, i64, i64); 12] = [
    (2, 1, 1, 0),
    (1, 1, 1, 1),
    (1, 0, 2, 1),
    (-1, 0, 2, 1),
    (-1, -1, 1, 1),
    (-2, -1, 1, 0),
    (-2, -1, -1, 0),
    (-1, -1, -1, -1),
    (-1, 0, -2, -1),
    (1, 0, -2, -1),
    (1, 1, -1, -1),
    (2, 1, -1, 0),
];

/// Labels of the twelve neighbour slots by direction `30m`: odd `m`
/// triangles, even `m` dodecagons.
pub const LABELS: [&'static str; 12] = [
    "dod@0", "tri@30", "dod@60", "tri@90", "dod@120", "tri@150", "dod@180", "tri@210", "dod@240",
    "tri@270", "dod@300", "tri@330",
];

/// `w_0 = (1 + sqrt3/2, 1/2)`, exact.
pub fn w0() -> Qp {
    (Zq::new(2, 1, 3, 2), Zq::new(1, 0, 3, 2))
}

/// The twelve hole vertices, exact: `w_k` at angle 15 + 30k.
pub fn hole() -> Vec<Qp> {
    (0..12).map(|k| rot30n(w0(), k)).collect()
}

/// One declared host tile: label (absorption census key) + polygon.
pub type HostTile = (&'static str, Vec<Qp>);

/// Neighbour dodecagon across ODD hole edge `k`: the hole's own vertex
/// placement translated to the centre `(2 + sqrt3) * unit(30(k+1))`.
fn dodecagon_across(k: usize) -> Vec<Qp> {
    let c = vmul(
        rot30n((Zq::whole(3, 1), Zq::zero(3)), (k + 1) % 12),
        z(2, 1),
    );
    (0..12).map(|j| vadd(c, rot30n(w0(), j))).collect()
}

/// Slot of the neighbour dodecagon across odd edge `k` in
/// [`neighbours`] (the six dodecagons come first).
fn dod_slot(k: usize) -> usize {
    ((k + 1) % 12) / 2
}

/// Slot of the neighbour triangle on even edge `k` in [`neighbours`]
/// (test helper; the push order in [`neighbours`] is the definition).
#[cfg(test)]
fn tri_slot(k: usize) -> usize {
    6 + k / 2
}

/// The twelve declared neighbours of the hole: six dodecagons across
/// the odd edges (slots 0..6, ordered by edge `k = 11, 1, 3, 5, 7, 9`),
/// then six unit triangles on the even edges (slots 6..12, `k = 0, 2,
/// …, 10`). The `tile` index of [`first_hit`] refers to this list.
pub fn neighbours() -> Vec<HostTile> {
    let w = hole();
    let mut out: Vec<HostTile> = Vec::with_capacity(12);
    let dods: Vec<(usize, Vec<Qp>)> = [11usize, 1, 3, 5, 7, 9]
        .iter()
        .map(|&k| (k, dodecagon_across(k)))
        .collect();
    for (k, poly) in &dods {
        assert!(
            poly.iter().any(|p| pt_eq(*p, w[*k]))
                && poly.iter().any(|p| pt_eq(*p, w[(k + 1) % 12])),
            "dodecagon across edge {k} covers the edge vertices"
        );
        out.push((LABELS[(k + 1) % 12], poly.clone()));
    }
    for k in [0usize, 2, 4, 6, 8, 10] {
        // apex: the unique vertex shared by the two neighbouring
        // dodecagons at unit distance from w_k (spike host_patch).
        let v1 = &dods[dod_slot((k + 11) % 12)].1;
        let v2 = &dods[dod_slot((k + 1) % 12)].1;
        let apex: Vec<Qp> = v1
            .iter()
            .filter(|p| {
                v2.iter().any(|q| pt_eq(**p, *q)) && norm2(vsub(**p, w[k])).eq_x(Zq::whole(3, 1))
            })
            .copied()
            .collect();
        assert_eq!(apex.len(), 1, "unique triangle apex at hole edge {k}");
        assert!(
            norm2(vsub(apex[0], w[(k + 1) % 12])).eq_x(Zq::whole(3, 1)),
            "apex unit from both edge vertices"
        );
        out.push((LABELS[(k + 1) % 12], vec![w[k], w[(k + 1) % 12], apex[0]]));
    }
    out
}

/// Field element `a + b sqrt3` at denominator 1.
fn z(a: i64, b: i64) -> Zq {
    Zq::new(a, b, 3, 1)
}

/// The binding similarity `S(p) = rot30^4(p) + w_0` (record edge 0 maps
/// to hole edge 0, direction 120 degrees; spike `m = 4`).
pub fn bind(p: Qp) -> Qp {
    vadd(rot30n(p, 4), w0())
}

/// Bind-and-absorb certificate over the 24 recorded exit rays of the
/// M158 star: `(1+sqrt3 count, 2+sqrt3 count, vertex hits, misses,
/// per-tile census)`. Every hit must be on a dodecagon vertex and the
/// census must be four rays per dodecagon, zero on the triangles.
pub fn bind_and_absorb() -> (usize, usize, usize, usize, BTreeMap<&'static str, usize>) {
    let w = hole();
    let tiles = neighbours();
    let polys: Vec<Vec<Qp>> = tiles.iter().map(|(_, p)| p.clone()).collect();
    let (mut t1, mut t2, mut vtx, mut miss) = (0, 0, 0, 0);
    let mut census: BTreeMap<&'static str, usize> = BTreeMap::new();
    for (i, d) in crate::medallion::wis_m158::exit_rays() {
        let p = w[i];
        let u = rot30n(d, 4);
        match first_hit(p, u, &polys) {
            None => miss += 1,
            Some(h) => {
                if t_is(&h, z(1, 1)) {
                    t1 += 1;
                } else if t_is(&h, z(2, 1)) {
                    t2 += 1;
                } else {
                    panic!("unexpected absorption parameter");
                }
                let (label, poly) = &tiles[h.tile];
                *census.entry(label).or_insert(0) += 1;
                // the hit point P + t u is exactly a vertex of the tile
                // (cross-multiplied componentwise, no division)
                let on_vertex = poly.iter().any(|v| {
                    let lhs = vmul(u, h.t_num);
                    let s = vsub(*v, p);
                    let rhs = (s.0.mul(h.t_den), s.1.mul(h.t_den));
                    pt_eq(lhs, rhs)
                });
                if on_vertex {
                    vtx += 1;
                }
            }
        }
    }
    (t1, t2, vtx, miss, census)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::medallion::wis_m158;
    use std::cmp::Ordering;

    #[test]
    fn hole_patch_is_exact_3_12_12_neighbourhood() {
        let w = hole();
        for k in 0..12 {
            let (a, b, c, d) = HOLE_TBL[k];
            assert!(w[k].0.eq_x(Zq::new(a, b, 3, 2)), "hole x {k}");
            assert!(w[k].1.eq_x(Zq::new(c, d, 3, 2)), "hole y {k}");
            assert!(norm2(w[k]).eq_x(z(2, 1)), "|w_k|^2 = 2 + sqrt3");
            assert!(
                norm2(vsub(w[(k + 1) % 12], w[k])).eq_x(Zq::whole(3, 1)),
                "unit hole edge {k}"
            );
        }
        let tiles = neighbours();
        assert_eq!(tiles.len(), 12);
        // dodecagons across odd edges: unit edges, own circumradius
        // 2 + sqrt3, and they cover the hole edge they sit across
        for k in [11usize, 1, 3, 5, 7, 9] {
            let (label, poly) = &tiles[dod_slot(k)];
            assert_eq!(poly.len(), 12);
            assert!(label.starts_with("dod@"));
            let s = vertex_sum(poly);
            let c = (div_i(s.0, 12), div_i(s.1, 12));
            for j in 0..12 {
                assert!(
                    norm2(vsub(poly[(j + 1) % 12], poly[j])).eq_x(Zq::whole(3, 1)),
                    "neighbour unit edge"
                );
                assert!(norm2(vsub(poly[j], c)).eq_x(z(2, 1)), "neighbour radius");
            }
        }
        // triangles on even edges are unit equilateral
        for k in [0usize, 2, 4, 6, 8, 10] {
            let (label, tri) = &tiles[tri_slot(k)];
            assert_eq!(tri.len(), 3);
            assert!(label.starts_with("tri@"));
            for j in 0..3 {
                assert!(
                    norm2(vsub(tri[(j + 1) % 3], tri[j])).eq_x(Zq::whole(3, 1)),
                    "unit triangle edge"
                );
            }
        }
        // neighbouring dodecagons (across edges 11 and 1) share an edge
        let n11 = &tiles[dod_slot(11)].1;
        let n1 = &tiles[dod_slot(1)].1;
        let shared = n11
            .iter()
            .filter(|p| n1.iter().any(|q| pt_eq(**p, *q)))
            .count();
        assert_eq!(shared, 2, "neighbouring dodecagons share an edge");
    }

    #[test]
    fn binding_from_contact_data_is_exact() {
        // S maps the record hull onto the hole vertex table, all 12
        // contacts exact, and the centre to the origin (doc §2).
        let h = wis_m158::hull();
        let w = hole();
        for i in 0..12 {
            assert!(pt_eq(bind(h[i]), w[i]), "S(V_{i}) == w_{i}");
        }
        let c = bind(wis_m158::centre());
        assert_eq!(c.0.sgn(), Ordering::Equal, "centre x -> 0");
        assert_eq!(c.1.sgn(), Ordering::Equal, "centre y -> 0");
    }

    #[test]
    fn all_24_exit_rays_are_absorbed_on_dodecagon_vertices() {
        let (t1, t2, vtx, miss, census) = bind_and_absorb();
        assert_eq!(miss, 0, "no ray escapes the patch");
        assert_eq!(t1, 12, "t = 1 + sqrt3");
        assert_eq!(t2, 12, "t = 2 + sqrt3");
        assert_eq!(vtx, 24, "every hit is exactly a tile vertex");
        // four rays per dodecagon, none on the triangles
        let mut want: BTreeMap<&'static str, usize> = BTreeMap::new();
        for m in [0usize, 2, 4, 6, 8, 10] {
            want.insert(LABELS[m], 4);
        }
        assert_eq!(census, want, "absorption census");
    }

    #[test]
    fn bound_ray_directions_are_on_the_thirty_degree_grid() {
        // The bound rays leave hole vertices along the lattice line
        // directions (multiples of 30 degrees) — the host network's own
        // directions (spike `bind_and_absorb` direction set).
        let grid: Vec<Qp> = (0..12)
            .map(|j| rot30n((Zq::whole(3, 1), Zq::zero(3)), j))
            .collect();
        for (_, d) in wis_m158::exit_rays() {
            let u = rot30n(d, 4);
            let hits = grid
                .iter()
                .filter(|w| cross(u, **w).sgn() == Ordering::Equal)
                .count();
            assert_eq!(hits, 2, "bound direction on the 30-degree grid");
        }
    }
}
