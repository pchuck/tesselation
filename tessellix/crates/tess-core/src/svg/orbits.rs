//! Vertex figure orbits (C1): the "two vertex orbits" a 2-uniform card
//! displays — the equivalence classes of vertices distinguished by the
//! cyclic sequence of tile side counts around the vertex (the vertex
//! figure, e.g. 3⁶ and 3².6² on the p6m 2-uniform).
//!
//! # Derivation (pure geometry, no registry or symmetry data)
//!
//! 1. **Dedupe vertices** by a quantized 1e-6 key — the same key the
//!    k-uniform census uses (`check_vertex_figures`), so vertices shared
//!    by neighboring tiles collapse to one.
//! 2. **Wedge per tile per vertex.** Each tile contributes one interior
//!    wedge at each of its own vertices: the angle span between its two
//!    incident edges, measured on the tile's interior side (the sign of
//!    the signed area decides which side is interior), carrying the
//!    tile's side count.
//! 3. **Completeness.** A vertex is *complete* when its incident wedges
//!    tile a full turn — the spans sum to 2π within [`COMPLETENESS_TOL`]
//!    AND no wedge gap exceeds that tolerance (this excludes boundary
//!    vertices of the finite patch, T-junction midpoints and any
//!    self-overlap artifact).
//! 4. **Figure key.** A complete vertex's figure is the cyclic sequence
//!    of its wedge side counts, ordered around the vertex by bisector
//!    angle. The key is the lexicographically smallest rotation of that
//!    side-count sequence over both cyclic orientations (dihedral
//!    canonical form; the lex order is on the numeric sequences, BEFORE
//!    dot-joining), so `3.6.3.6` reads as `3.6.3.6` from either
//!    traversal and digit-run collisions like `12` vs `1,2` are
//!    impossible once joined.
//! 5. **Partition + order.** Vertices partition by figure key into a
//!    `BTreeMap`, so orbits come out in lexicographic figure-key order
//!    (deterministic); each orbit's points are sorted by (x, y) for
//!    byte-deterministic emission.
//!
//! On a 2-uniform tiling the result is exactly the two vertex classes
//! the card declares in its `vertex_config`.

use crate::geometry::Vec2;
use crate::tiling::Tile;

/// One vertex figure class: the vertices of a tiling sharing the same
/// cyclic side-count sequence.
#[derive(Debug, Clone, PartialEq)]
pub struct VertexOrbit {
    /// Dot-joined cyclic vertex figure in dihedral lex-smallest rotation,
    /// e.g. `"3.6.3.6"`, `"3.3.4.4"`, `"4.6.12"`.
    pub figure: String,
    /// One point per patch vertex in this class (deduped, sorted by
    /// (x, y) for deterministic emission order).
    pub points: Vec<Vec2>,
}

/// Closure tolerance (radians) for "the incident wedges make a full turn
/// around this vertex": an interior vertex closes to exactly 2π, a
/// boundary or T-junction vertex leaves a gap. 1e-3 rad ≈ 0.057°, far
/// below any genuine tile angle (smallest here is the 30° 12-gon corner).
const COMPLETENESS_TOL: f64 = 1e-3;

/// Quantization scale for vertex identity (the census 1e-6 precedent).
const VKEY_SCALE: f64 = 1e6;

/// The vertex figure orbits of `tiles`, in lexicographic figure-key
/// order (deterministic for a fixed tile list).
///
/// Complete vertices only (see the module docs): on a 2-uniform tiling
/// the result is exactly the two vertex classes the card declares in its
/// `vertex_config`, and orbit 0 is the lexicographically first figure.
pub fn vertex_orbits(tiles: &[Tile]) -> Vec<VertexOrbit> {
    type Wedge = (f64, f64, u32); // (start angle, span, side count)

    // Per deduped vertex: representative point + one wedge per incident
    // tile.
    let mut acc: std::collections::HashMap<(i64, i64), (Vec2, Vec<Wedge>)> =
        std::collections::HashMap::new();

    for tile in tiles {
        let pts = &tile.poly.pts;
        let n = pts.len();
        if n < 3 {
            continue; // degenerate: no interior wedge
        }
        // Signed area: positive = CCW, interior to the left of each edge.
        let mut twice_area = 0.0f64;
        for k in 0..n {
            let a = pts[k];
            let b = pts[(k + 1) % n];
            twice_area += a.x * b.y - a.y * b.x;
        }
        let ccw = twice_area > 0.0;
        for k in 0..n {
            let v = pts[k];
            let prev = pts[(k + n - 1) % n];
            let next = pts[(k + 1) % n];
            let ang = |p: Vec2| f64::atan2(p.y - v.y, p.x - v.x);
            // The interior spans from one incident-edge direction to the
            // other, going the way the interior lies. CCW: start at the
            // v->next direction and turn CCW (positive) to v->prev. CW:
            // start at v->prev and turn CCW to v->next (the interior side
            // of a CW polygon is the right of each edge).
            let (start, theta) = if ccw {
                let a = ang(next);
                (a, norm2pi(ang(prev) - a))
            } else {
                let a = ang(prev);
                (a, norm2pi(ang(next) - a))
            };
            if theta < 1e-9 {
                continue; // collinear vertex: zero wedge, no figure token
            }
            let key = vkey(v);
            let entry = acc.entry(key).or_insert((v, Vec::new()));
            entry.1.push((norm2pi(start), theta, n as u32));
        }
    }

    let mut by_key: std::collections::BTreeMap<String, Vec<Vec2>> =
        std::collections::BTreeMap::new();
    for (_key, (v, wedges)) in acc {
        let sum: f64 = wedges.iter().map(|w| w.1).sum();
        if (sum - std::f64::consts::TAU).abs() > COMPLETENESS_TOL {
            continue; // boundary / T-junction: does not close to 2π
        }
        let mut ws = wedges;
        ws.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        // Defense: the wedges must genuinely tile the turn — no gap
        // (measured from each wedge's end to the next wedge's start,
        // cyclic) may exceed the tolerance. Overlapping wedges read as a
        // near-full-turn gap and drop the vertex too.
        let mut ok = true;
        for i in 0..ws.len() {
            let end = ws[i].0 + ws[i].1;
            let next_start = ws[(i + 1) % ws.len()].0;
            let gap = (next_start - end).rem_euclid(std::f64::consts::TAU);
            if gap > COMPLETENESS_TOL {
                ok = false;
                break;
            }
        }
        if !ok {
            continue;
        }
        let seq: Vec<u32> = ws.iter().map(|w| w.2).collect();
        let figure = canonical_figure(&seq);
        by_key.entry(figure).or_default().push(v);
    }

    by_key
        .into_iter()
        .map(|(figure, mut points)| {
            points.sort_by(|a, b| {
                a.x.partial_cmp(&b.x)
                    .unwrap()
                    .then_with(|| a.y.partial_cmp(&b.y).unwrap())
            });
            VertexOrbit { figure, points }
        })
        .collect()
}

/// Normalize an angle into [0, 2π).
fn norm2pi(a: f64) -> f64 {
    let m = a % std::f64::consts::TAU;
    if m < 0.0 {
        m + std::f64::consts::TAU
    } else {
        m
    }
}

/// Quantized vertex identity (the census 1e-6 precedent).
fn vkey(p: Vec2) -> (i64, i64) {
    ((p.x * VKEY_SCALE).round() as i64, (p.y * VKEY_SCALE).round() as i64)
}

/// The lexicographically smallest dot-joined rotation of the cyclic
/// side-count sequence `seq`, considered up to cyclic rotation and
/// reversal (the dihedral group): two figures are the same class iff
/// they are dihedral-equivalent.
fn canonical_figure(seq: &[u32]) -> String {
    let n = seq.len();
    let mut best: Option<Vec<u32>> = None;
    for start in 0..n {
        for rev in [false, true] {
            let mut cand = Vec::with_capacity(n);
            for i in 0..n {
                let idx = if rev {
                    (start + n - 1 - i).rem_euclid(n)
                } else {
                    (start + i) % n
                };
                cand.push(seq[idx]);
            }
            match &best {
                None => best = Some(cand),
                Some(b) if cand < *b => best = Some(cand),
                Some(_) => {}
            }
        }
    }
    let b = best.expect("non-empty sequence");
    b.iter().map(u32::to_string).collect::<Vec<_>>().join(".")
}
