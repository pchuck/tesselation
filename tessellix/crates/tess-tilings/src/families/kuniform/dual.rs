//! Face-centroid duals of periodic (lattice) tilings.
//!
//! [`dual`] constructs the centroid (Laves-style) dual of a
//! [`LatticeTiling`]: one dual vertex at each primal face center, one
//! dual edge across each primal edge, one dual face at each primal
//! vertex — the polygon of the surrounding face centers in cyclic
//! (counterclockwise) order. Primal faces are regular, so the vertex
//! average is the face center. The dual shares the primal lattice basis
//! (same `LatticeKind`, offset 0); the per-cell table holds one canonical
//! dual face per primal-vertex orbit, so every dual face is emitted
//! exactly once by exactly one cell.
//!
//! The construction is exact in the input coordinates and needs no
//! window scan: the primal is periodic in cell space under the lattice
//! Λ = ⟨a1, a2⟩, so every primal vertex falls into a translation orbit
//! whose key is the fractional part of A⁻¹p rounded to a 1e6 grid
//! (A = [a1 a2]). A cell is incident to that orbit exactly when one of
//! its vertices carries that key, and the orbit's representative
//! v_rep = offset + A·k (k the key, in [0, 1)²) is a vertex of one
//! translate of the cell whose center is c + (v_rep − p) — that point is
//! one vertex of the dual face. Sorting the contributions
//! counterclockwise around v_rep gives the dual face.
//!
//! This module is the machine replacement for the hand-transcribed Laves
//! tables in `laves.rs`: [`dual`] on each Archimedean primal reproduces
//! its Laves twin exactly (oracle tests in `dual_tests` below), and the
//! eleven constructors dualize the three k-uniform batches — batch 1's
//! p6m 2-uniform family `kuniform-p6m`, `kuniform-n18`, `kuniform-n9`,
//! `kuniform-n8`, batch 2's `kuniform-n5`, `kuniform-n1`,
//! `kuniform-n13` and `kuniform-n20` (the chiral p6 form), and batch 3's
//! `kuniform-dual-pmm`, `kuniform-dual-n12` and `kuniform-dual-n19`
//! (rectangular, centered-rectangular and hexagonal primals).
//!
//! Every `LatticeTiling` here is built at call time (the orbit scan is
//! O(cells × vertices) — trivial), so the registry re-constructs the
//! eleven duals on each `registry()` call. All id/name/description/seed
//! fields are `'static` literals; only `cells` is computed, so the
//! returned tables need no lazy globals and register like any other.
//!
//! Pattern seeds are a congruence-aware proper coloring of the
//! dual-face adjacency graph (edge-sharing): congruent dual faces share
//! a base seed (their congruence class index, first-appearance order),
//! and a deterministic greedy pass bumps a face's seed past the colors
//! of its already-colored edge-neighbors, so edge-adjacent faces always
//! differ in `seed % 8`.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use tess_core::category::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::view::ColorMode;

use crate::kuniform;
use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// Fractional lattice coordinates are rounded to this grid to group
/// primal vertices into translation orbits (far below the separation of
/// any two vertex orbits in the tables dualized here).
const ORBIT_GRID: f64 = 1.0e6;

/// Inverse of the lattice matrix A = [a1 a2] (basis as columns).
struct LatticeInv {
    m: [[f64; 2]; 2],
}

impl LatticeInv {
    fn new(a1: Vec2, a2: Vec2) -> Option<Self> {
        let det = a1.cross(a2);
        if det.abs() < 1e-12 {
            return None;
        }
        let d = 1.0 / det;
        Some(Self {
            m: [
                [a2.y * d, -a2.x * d],
                [-a1.y * d, a1.x * d],
            ],
        })
    }

    /// Lattice (u-space) coordinates of a cell-space point.
    fn apply(&self, p: Vec2) -> Vec2 {
        Vec2::new(
            self.m[0][0] * p.x + self.m[0][1] * p.y,
            self.m[1][0] * p.x + self.m[1][1] * p.y,
        )
    }
}

/// Translation-orbit key of a cell-space point: its lattice coordinate's
/// fractional part on the [`ORBIT_GRID`] grid (a fractional part that
/// rounds to 1 wraps to 0, so keys land in [0, 1e6)²).
fn orbit_key(inv: &LatticeInv, p: Vec2) -> (i64, i64) {
    let u = inv.apply(p);
    let comp = |x: f64| -> i64 {
        let f = x - x.floor();
        let r = (f * ORBIT_GRID).round() as i64;
        if r >= ORBIT_GRID as i64 {
            0
        } else {
            r
        }
    };
    (comp(u.x), comp(u.y))
}

/// One dual face per primal-vertex orbit, in orbit-key order.
///
/// Each polygon is the loop of incident face centers around the orbit
/// representative (the orbit's point inside the fundamental
/// parallelogram), sorted counterclockwise around it.
///
/// `None` on a degenerate lattice or on a vertex orbit with fewer than
/// three incident face centers (no proper 2D tiling vertex).
fn dual_cells(primal: &LatticeTiling) -> Option<Vec<((i64, i64), Polygon)>> {
    let (a1, a2) = (primal.lattice[0], primal.lattice[1]);
    let inv = LatticeInv::new(a1, a2)?;

    // Per orbit: the "face-center minus shared vertex" contributions of
    // the incident cells, plus one *stored* table vertex as the orbit's
    // representative (using a real table vertex keeps the construction at
    // float-1-ulp precision; the 1e6 grid is used for grouping only).
    // Primal cells are simple polygons, so each cell contributes to a
    // given orbit at most once.
    let mut orbits: BTreeMap<(i64, i64), (Vec<Vec2>, Vec2)> = BTreeMap::new();
    for cell in &primal.cells {
        let pts = &cell.poly.pts;
        let n = pts.len();
        if n < 3 {
            continue;
        }
        // Face center = vertex average (primal faces are regular).
        let mut c = Vec2::ZERO;
        for &p in pts {
            c += p;
        }
        c = c / n as f64;
        for &p in pts {
            let k = orbit_key(&inv, p + primal.offset);
            let e = orbits.entry(k).or_insert_with(|| (Vec::new(), p));
            e.0.push(c - p);
        }
    }

    let mut out = Vec::with_capacity(orbits.len());
    for (k, (offs, vrep)) in &orbits {
        if offs.len() < 3 {
            return None;
        }
        // vrep is a stored primal vertex, the orbit's representative.
        let mut pts: Vec<Vec2> = offs.iter().map(|o| *vrep + *o).collect();
        // Counterclockwise (y-up) around the representative.
        pts.sort_by(|u, v| {
            let ang = |p: &Vec2| {
                let d = *p - *vrep;
                d.y.atan2(d.x)
            };
            ang(u).total_cmp(&ang(v))
        });
        out.push((*k, Polygon::from_slice(&pts)));
    }
    Some(out)
}

/// The centroid dual of `primal`, with placeholder identity (`"dual"`) —
/// for oracle and test use; the named constructors below carry the
/// catalog metadata.
pub fn dual(primal: &LatticeTiling) -> Option<LatticeTiling> {
    let cells = dual_cells(primal)?;
    Some(LatticeTiling {
        id: "dual",
        name: "Dual (placeholder)",
        description: "Face-centroid dual of a LatticeTiling (placeholder metadata).",
        lattice: primal.lattice,
        cells: cells
            .into_iter()
            .enumerate()
            .map(|(i, (_, poly))| Cell {
                poly,
                seed: i as u32,
            })
            .collect(),
        offset: Vec2::ZERO,
        group: primal.group,
        lattice_kind: primal.lattice_kind,
        category: TilingCategory::Laves,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    })
}

/// Core construction for a named catalog entry: the [`dual`] geometry
/// with the dual's own id/name/description, the Laves category (primary
/// only), and the congruence-aware proper-coloring seeds.
fn build(
    primal: &LatticeTiling,
    id: &'static str,
    name: &'static str,
    description: &'static str,
) -> Option<LatticeTiling> {
    let cells = dual_cells(primal)?;
    let seeds = proper_seeds(primal, &cells);
    Some(LatticeTiling {
        id,
        name,
        description,
        lattice: primal.lattice,
        cells: cells
            .into_iter()
            .zip(seeds)
            .map(|((_, poly), seed)| Cell { poly, seed })
            .collect(),
        offset: Vec2::ZERO,
        group: primal.group,
        lattice_kind: primal.lattice_kind,
        category: TilingCategory::Laves,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    })
}

/// Congruence signature of a CCW polygon: sorted side lengths and
/// sorted interior angles, each on a 1e-9 grid — discriminates the face
/// classes of the tables dualized here.
fn signature(poly: &Polygon) -> (Vec<i64>, Vec<i64>) {
    let pts = &poly.pts;
    let n = pts.len();
    let mut sides = Vec::with_capacity(n);
    let mut angles = Vec::with_capacity(n);
    for i in 0..n {
        let prev = pts[(i + n - 1) % n];
        let cur = pts[i];
        let next = pts[(i + 1) % n];
        sides.push(((next - cur).length() * 1e9).round() as i64);
        let e_in = cur - prev;
        let e_out = next - cur;
        // Interior angle of a CCW polygon: π minus the signed turn.
        let turn = e_in.cross(e_out).atan2(e_in.dot(e_out));
        angles.push(((std::f64::consts::PI - turn) * 1e9).round() as i64);
    }
    sides.sort_unstable();
    angles.sort_unstable();
    (sides, angles)
}

/// Congruence-aware proper-coloring seeds for the dual faces
/// (same order as [`dual_cells`]).
///
/// Base seed = congruence-class index (first-appearance order); the
/// greedy pass then bumps a face's seed only past the colors of its
/// already-colored (lower-index) edge-neighbors, modulo 8.
fn proper_seeds(primal: &LatticeTiling, cells: &[((i64, i64), Polygon)]) -> Vec<u32> {
    let n = cells.len();
    let (a1, a2) = (primal.lattice[0], primal.lattice[1]);
    let inv = LatticeInv::new(a1, a2).expect("non-degenerate lattice");

    // Congruence classes, first appearance in table order.
    let mut class_of: BTreeMap<(Vec<i64>, Vec<i64>), u32> = BTreeMap::new();
    let mut base = Vec::with_capacity(n);
    for (_, poly) in cells {
        let sig = signature(poly);
        let c = match class_of.get(&sig) {
            Some(&c) => c,
            None => {
                let c = class_of.len() as u32;
                class_of.insert(sig, c);
                c
            }
        };
        base.push(c);
    }

    // Edge adjacency between vertex orbits: each primal cell edge joins
    // the two orbits of its endpoints.
    let mut orbit_idx: HashMap<(i64, i64), usize> = HashMap::with_capacity(n);
    for (i, (k, _)) in cells.iter().enumerate() {
        orbit_idx.insert(*k, i);
    }
    let mut adj: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n];
    for cell in &primal.cells {
        let pts = &cell.poly.pts;
        let m = pts.len();
        for e in 0..m {
            let k0 = orbit_key(&inv, pts[e] + primal.offset);
            let k1 = orbit_key(&inv, pts[(e + 1) % m] + primal.offset);
            if let (Some(&i), Some(&j)) = (orbit_idx.get(&k0), orbit_idx.get(&k1)) {
                if i != j {
                    adj[i].insert(j);
                    adj[j].insert(i);
                }
            }
        }
    }

    let mut seeds = vec![0u32; n];
    for i in 0..n {
        let mut s = base[i];
        while adj[i].iter().any(|&j| j < i && seeds[j] % 8 == s % 8) {
            s += 1;
        }
        seeds[i] = s;
    }
    seeds
}

/// Face-centroid dual of `kuniform-p6m` (2-uniform 3⁶; 3².6², p6m).
///
/// Same lattice as the primal, a1 = (3, 0), a2 = (3/2, 3√3/2), |det| =
/// 9√3/2. The cell holds the 7 dual faces of the 7 vertex orbits: one
/// regular hexagon (the 3⁶ fan apex; side 1/√3, area √3/2) and six
/// congruent quadrilaterals (the 3².6² vertices; sides 1/√3, 2√3/3,
/// 2√3/3, √3 in cyclic order; angles 60°/60°/120°/120°; area 2√3/3).
/// The seven areas sum to 9√3/2 = |det|. Pattern seeds: the hexagon
/// base 0, the six quads base 1 (a W₆ wheel), greedy-bumped to {1, 2}
/// mod 8.
pub fn kuniform_dual_p6m() -> LatticeTiling {
    build(
        &kuniform::kuniform_p6m(),
        "kuniform-dual-p6m",
        "Dual of 2-uniform (V3⁶; V3².6²)",
        "Face-centroid dual of the 2-uniform (3⁶; 3².6²) tiling (p6m) — one regular hexagon plus six congruent quadrilaterals per cell.",
    )
    .expect("kuniform-p6m dual: non-degenerate lattice")
}

/// Face-centroid dual of `kuniform-n18` (2-uniform 3⁶; 3².4.3.4, p6m).
///
/// Same lattice as the primal, a1 = (0, 1+√3), a2 = ((3+√3)/2,
/// (1+√3)/2), |det| = 3 + 2√3. The cell holds the 7 dual faces of the 7
/// vertex orbits: one regular hexagon (the 3⁶ vertex; side 1/√3, area
/// √3/2) and six congruent pentagons (the 3².4.3.4 vertices; one side
/// 1/√3 — between two triangle centers — and four sides (3+√3)/6
/// — triangle-to-square center distances; angles 90°/90°/120°/120°/
/// 120°; area (2+√3)/4). The seven areas sum to 3 + 2√3 = |det|.
pub fn kuniform_dual_n18() -> LatticeTiling {
    build(
        &kuniform::kuniform_n18(),
        "kuniform-dual-n18",
        "Dual of 2-uniform (V3⁶; V3².4.3.4)",
        "Face-centroid dual of the 2-uniform (3⁶; 3².4.3.4) tiling (p6m) — one regular hexagon plus six congruent pentagons per cell.",
    )
    .expect("kuniform-n18 dual: non-degenerate lattice")
}

/// Face-centroid dual of `kuniform-n9` (2-uniform 3.4.6.4; 3².4.3.4,
/// p6m).
///
/// Same (left-handed) lattice as the primal, |det| = 6 + 7√3/2. The
/// cell holds the 12 dual faces of the 12 vertex orbits: six congruent
/// quadrilaterals (the 3.4.6.4 vertices; sides (3+√3)/6 ×2 —
/// triangle-to-square — and (1+√3)/2 ×2 — square-to-hexagon — center
/// distances; angles 60°/90°/90°/120°; area (3+2√3)/6) and six
/// congruent pentagons (the 3².4.3.4 vertices; same shape as the
/// `kuniform-dual-n18` pentagons: side 1/√3 and (3+√3)/6 ×4, angles
/// 90°/90°/120°/120°/120°, area (2+√3)/4). The twelve areas sum to
/// 6 + 7√3/2 = |det|.
pub fn kuniform_dual_n9() -> LatticeTiling {
    build(
        &kuniform::kuniform_n9(),
        "kuniform-dual-n9",
        "Dual of 2-uniform (V3.4.6.4; V3².4.3.4)",
        "Face-centroid dual of the 2-uniform (3.4.6.4; 3².4.3.4) tiling (p6m) — six quadrilaterals and six pentagons per cell.",
    )
    .expect("kuniform-n9 dual: non-degenerate lattice")
}

/// Face-centroid dual of `kuniform-n8` (2-uniform 3.4.6.4; 3³.4², p6m).
///
/// Same (left-handed) lattice as the primal, |det| = 6 + 7√3/2. The
/// cell holds the 12 dual faces of the 12 vertex orbits: six congruent
/// quadrilaterals (the 3.4.6.4 vertices; same shape as the
/// `kuniform-dual-n9` quadrilaterals) and six congruent pentagons (the
/// 3³.4² vertices; sides 1/√3 ×2, (3+√3)/6 ×2, and 1; angles
/// 90°/90°/120°/120°/120°; area (2+√3)/4 — equal to the n9 pentagon
/// area, different shape). The twelve areas sum to 6 + 7√3/2 = |det|.
pub fn kuniform_dual_n8() -> LatticeTiling {
    build(
        &kuniform::kuniform_n8(),
        "kuniform-dual-n8",
        "Dual of 2-uniform (V3.4.6.4; V3³.4²)",
        "Face-centroid dual of the 2-uniform (3.4.6.4; 3³.4²) tiling (p6m) — six quadrilaterals and six pentagons per cell.",
    )
    .expect("kuniform-n8 dual: non-degenerate lattice")
}

/// Face-centroid dual of `kuniform-n5` (2-uniform 3.4.6.4; 3.4².6, p6m).
///
/// Same lattice as the primal, a1 = (3+√3, 0), a2 = (−(3+√3)/2,
/// √3(3+√3)/2), |det| = 9+6√3. The cell holds the 18 dual faces of the
/// 18 vertex orbits: six congruent quadrilaterals (the 3.4.6.4 orbit;
/// sides (3+√3)/6, (3+√3)/2, (3+√3)/2, (3+√3)/6 in cyclic order; angles
/// 120°/90°/60°/90°; area (3+2√3)/6) and twelve congruent quadrilaterals
/// (the 3.4².6 orbit; sides 2/√3, (3+√3)/2, 1, (3+√3)/6; angles
/// 120°/60°/90°/90°; area (3+2√3)/6 — same area, different shape). The
/// eighteen areas sum to 9+6√3 = |det|. Measured symmetry p6m, with the
/// 6-fold center at (3+√3, 2+2√3) (the origin is a 3-fold center of
/// this dual).
pub fn kuniform_dual_n5() -> LatticeTiling {
    build(
        &kuniform::kuniform_n5(),
        "kuniform-dual-n5",
        "Dual of 2-uniform (V3.4.6.4; V3.4².6)",
        "Face-centroid dual of the 2-uniform (3.4.6.4; 3.4².6) tiling (p6m) — six quadrilaterals and twelve congruent quadrilaterals per cell.",
    )
    .expect("kuniform-n5 dual: non-degenerate lattice")
}

/// Face-centroid dual of `kuniform-n1` (2-uniform 4.6.12; 3.4.6.4,
/// p6m).
///
/// Same lattice as the primal, a1 = (2+2√3, 0), a2 = (1+√3, 3+√3),
/// |det| = 12+8√3. The cell holds the 18 dual faces of the 18 vertex
/// orbits: twelve congruent 30°/60°/90° triangles (the 4.6.12 orbit;
/// sides 1+√3, (3+√3)/2, (1+√3)/2 in cyclic order; angles 60°/30°/90°;
/// area (3+2√3)/4) and six congruent quadrilaterals (the 3.4.6.4 orbit;
/// sides (3+√3)/2, (3+√3)/6, (3+√3)/6, (3+√3)/2; angles 60°/90°/120°/
/// 90°; area (3+2√3)/6). The eighteen areas sum to 12+8√3 = |det|.
/// Measured symmetry p6m, with the 6-fold center at (3(1+√3)/2,
/// (3+√3)/2).
pub fn kuniform_dual_n1() -> LatticeTiling {
    build(
        &kuniform::kuniform_n1(),
        "kuniform-dual-n1",
        "Dual of 2-uniform (V4.6.12; V3.4.6.4)",
        "Face-centroid dual of the 2-uniform (4.6.12; 3.4.6.4) tiling (p6m) — twelve 30°/60°/90° triangles plus six congruent quadrilaterals per cell.",
    )
    .expect("kuniform-n1 dual: non-degenerate lattice")
}

/// Face-centroid dual of `kuniform-n13` (2-uniform 3⁶; 3².4.12, p6m).
///
/// Same lattice as the primal, a1 = (3(1+√3)/2, (3+√3)/2), a2 = (0,
/// 3+√3), |det| = 9+6√3. The cell holds the 14 dual faces of the 14
/// vertex orbits: two regular hexagons (the 3⁶ orbit; side 1/√3, area
/// √3/2) and twelve congruent quadrilaterals (the 3².4.12 orbit; sides
/// 1/√3, (3+√3)/6, (3+√3)/2, 1+2/√3 in cyclic order; angles
/// 120°/120°/90°/30°; area (9+5√3)/12). The fourteen areas sum to
/// 9+6√3 = |det|. Measured symmetry p6m, with the 6-fold center at
/// (5(1+√3)/2, 5(3+√3)/2).
pub fn kuniform_dual_n13() -> LatticeTiling {
    build(
        &kuniform::kuniform_n13(),
        "kuniform-dual-n13",
        "Dual of 2-uniform (V3⁶; V3².4.12)",
        "Face-centroid dual of the 2-uniform (3⁶; 3².4.12) tiling (p6m) — two regular hexagons plus twelve congruent quadrilaterals per cell.",
    )
    .expect("kuniform-n13 dual: non-degenerate lattice")
}

/// Face-centroid dual of `kuniform-n20` (2-uniform 3⁶; 3⁴.6, p6).
///
/// Same lattice as the primal, a1 = (5/2, 3√3/2), a2 = (−1, 2√3),
/// |det| = 13√3/2. The cell holds the 12 dual faces of the 12 vertex
/// orbits: six regular hexagons (the 3⁶ orbit; side 1/√3, area √3/2)
/// and six congruent pentagons (the 3⁴.6 orbit; sides 1/√3, 1/√3,
/// 1/√3, 2/√3, 2/√3 in cyclic order; angles 120°/120°/120°/120°/60°;
/// area 7√3/12). The twelve areas sum to 13√3/2 = |det|. Measured
/// symmetry p6 (chiral — no mirrors, matching the primal's group), with
/// the 6-fold center at the origin.
pub fn kuniform_dual_n20() -> LatticeTiling {
    build(
        &kuniform::kuniform_n20(),
        "kuniform-dual-n20",
        "Dual of 2-uniform (V3⁶; V3⁴.6)₁",
        "Face-centroid dual of the 2-uniform (3⁶; 3⁴.6)₁ tiling (p6) — six regular hexagons plus six congruent pentagons per cell.",
    )
    .expect("kuniform-n20 dual: non-degenerate lattice")
}

/// Face-centroid dual of `kuniform-pmm` (2-uniform 3.6.3.6; 3².6², pmm).
///
/// Same lattice as the primal, a1 = (2, 0), a2 = (0, √3), |det| = 2√3
/// (rectangular). The cell holds the 3 dual faces of the 3 vertex
/// orbits: one rhombus (the 3.6.3.6 orbit — the shared vertex of a
/// hexagon pair; sides 2/√3 ×4, angles 60°/120°/120°/60° in cyclic
/// order, area 2√3/3, center (1, 0)) and two congruent quadrilaterals
/// (the 3².6² orbit — the hexagon-row vertices (±1/2, √3/2), two
/// translation classes under the rectangular lattice; sides 1/√3,
/// 2/√3, 2/√3, √3 in cyclic order; angles 60°/60°/120°/120°; area
/// 2√3/3). The three areas sum to 2√3 = |det|. Measured symmetry pmm
/// (the dual keeps the primal's group): 2-fold centers at (0, 0),
/// (1, 0), (0, √3/2), (1, √3/2) — one per mirror intersection — and
/// mirror lines horizontal and vertical through all of them; R60, R90
/// and every off-axis mirror fail.
pub fn kuniform_dual_pmm() -> LatticeTiling {
    build(
        &kuniform::kuniform_pmm(),
        "kuniform-dual-pmm",
        "Dual of 2-uniform (V3.6.3.6; V3².6²)",
        "Face-centroid dual of the 2-uniform (3.6.3.6; 3².6²) tiling (pmm) — one rhombus plus two congruent quadrilaterals per cell.",
    )
    .expect("kuniform-pmm dual: non-degenerate lattice")
}

/// Face-centroid dual of `kuniform-n12` (2-uniform 3².6²; 3⁴.6, cmm).
///
/// Same lattice as the primal, a1 = (√3, 0), a2 = (√3/2, 5/2),
/// |det| = 5√3/2 (centered rectangular). The cell holds the 4 dual
/// faces of the 4 vertex orbits: two congruent quadrilaterals (the
/// 3².6² orbit — the hexagon side corners (±√3/2, ±1/2), two
/// translation classes under the primitive cell, class
/// representatives (−√3/2, ±1/2); sides 1/√3, 2/√3, 2/√3, √3 in
/// cyclic order; angles 60°/60°/120°/120°; area 2√3/3) and two
/// congruent pentagons (the 3⁴.6 orbit — the hexagon top/bottom
/// vertices (0, ±1); sides 1/√3 ×3 and 2/√3 ×2; angles
/// 60°/120°/120°/120°/120°; area 7√3/12 — the same 3⁴.6 dual
/// pentagon as `kuniform-dual-n20`). The four areas sum to 5√3/2 = |det|.
/// Measured symmetry cmm (the dual keeps the primal's group): 2-fold
/// centers at (0, 0) and (√3/2, 0) on the mirror intersections, plus
/// the two off-mirror (glide-only) centers (√3/4, 5/4) and (3√3/4,
/// 5/4); mirror lines horizontal and vertical through the on-mirror
/// centers only; R60, R90 and every off-axis mirror fail.
pub fn kuniform_dual_n12() -> LatticeTiling {
    build(
        &kuniform::kuniform_n12(),
        "kuniform-dual-n12",
        "Dual of 2-uniform (V3².6²; V3⁴.6)",
        "Face-centroid dual of the 2-uniform (3².6²; 3⁴.6) tiling (cmm) — two quadrilaterals and two congruent pentagons per cell.",
    )
    .expect("kuniform-n12 dual: non-degenerate lattice")
}

/// Face-centroid dual of `kuniform-n19` (2-uniform 3⁶; 3⁴.6, p6m).
///
/// Same lattice as the primal, a1 = (3, 0), a2 = (3/2, 3√3/2),
/// |det| = 9√3/2 (hexagonal — the flower lattice L9). The cell holds
/// the 8 dual faces of the 8 vertex orbits: six congruent pentagons
/// (the 3⁴.6 orbit — the six corners of the origin flower, one per
/// translation class; sides 1/√3 ×3 and 2/√3 ×2; angles
/// 60°/120°/120°/120°/120°; area 7√3/12) and two regular hexagons
/// (the 3⁶ orbit — the petal tips, at (3/2, √3/2) and (0, √3); side
/// 1/√3, area √3/2). The eight areas sum to 9√3/2 = |det|. Measured
/// symmetry p6m, with the 6-fold center at the origin (the primal's
/// flower center — in the dual, the point where the six 3⁴.6 pentagons
/// meet); 2-fold centers additionally at (3/2, 0), (3/4, 3√3/4) and
/// (9/4, 3√3/4).
pub fn kuniform_dual_n19() -> LatticeTiling {
    build(
        &kuniform::kuniform_n19(),
        "kuniform-dual-n19",
        "Dual of 2-uniform (V3⁶; V3⁴.6)₂",
        "Face-centroid dual of the 2-uniform (3⁶; 3⁴.6)₂ tiling (p6m) — six congruent pentagons plus two regular hexagons per cell.",
    )
    .expect("kuniform-n19 dual: non-degenerate lattice")
}

/// Face-centroid dual of `kuniform-n2` (2-uniform 3.12.12; 3.4.3.12, p4m).
///
/// Same lattice as the primal, a1 = (2+√3, 0), a2 = (0, 2+√3),
/// |det| = 7+4√3 (square). The cell holds the 8 dual faces of the 8
/// vertex orbits: four congruent kites (the 3.4.3.12 orbit; sides
/// (3+√3)/6, (2+√3)/√3, (2+√3)/√3, (3+√3)/6 in cyclic order; angles
/// 90°/120°/30°/120°; area 3/4+5√3/12) and four congruent isosceles
/// triangles (the 3.12.12 orbit; sides (2+√3)/√3, 2+√3, (2+√3)/√3;
/// angles 120°/30°/30° — the base is one full lattice period; area
/// 1+7√3/12). The eight areas sum to 7+4√3 = |det|. Measured symmetry
/// p4m (the dual keeps the primal's group): every coset of the
/// measured group mod the lattice is represented by an operation
/// fixing the origin (the primal square center) — R90/R180/R270 about
/// it and the four mirror classes (horizontal, vertical, and both
/// diagonals) through it, all pure reflections with no glide coset
/// (the p4g discriminator fails: mirrors pass the 4-fold center).
pub fn kuniform_dual_n2() -> LatticeTiling {
    build(
        &kuniform::kuniform_n2(),
        "kuniform-dual-n2",
        "Dual of 2-uniform (3.12.12; 3.4.3.12)",
        "Face-centroid dual of the 2-uniform (3.12.12; 3.4.3.12) tiling (p4m) — four kites plus four isosceles triangles per cell.",
    )
    .expect("kuniform-n2 dual: non-degenerate lattice")
}

/// Face-centroid dual of `kuniform-n16` (2-uniform 3³.4²; 3².4.3.4, p4g).
///
/// Same lattice as the primal, a1 = (h, −h), a2 = (h, h) with
/// h = (3+√3)/2, |det| = 6+3√3 (square). The cell holds the 12 dual
/// faces of the 12 vertex orbits: eight congruent pentagons (the
/// 3².4.3.4 orbit; sides (3+√3)/6, (3+√3)/6, 1/√3, (3+√3)/6,
/// (3+√3)/6 in cyclic order; angles 120°/90°/120°/120°/90°; area
/// 1/2+√3/4) and four congruent pentagons (the 3³.4² orbit; sides
/// (3+√3)/6, 1, (3+√3)/6, 1/√3, 1/√3 in cyclic order — the unit side
/// crosses the shared edge of two unit squares; angles
/// 120°/90°/90°/120°/120°; area 1/2+√3/4, equal to the other class).
/// The twelve areas sum to 6+3√3 = |det|. Measured symmetry p4g (the
/// dual keeps the primal's group): order-8 cosets with 4-fold centers
/// at (√3−1)/4, (5+√3)/4 and its lattice partners, pure axis mirrors
/// only along the lines x = −1 and y = 1/2 (meeting at the corner-2
/// point (−1, 1/2)); both diagonal reflection classes are TRUE glides
/// (measured glide g(x, y) = (y + 3/2 + √3, x − √3/2, half-period
/// axial shift), so no mirror passes a 4-fold center — the p4g
/// discriminator against p4m.
pub fn kuniform_dual_n16() -> LatticeTiling {
    build(
        &kuniform::kuniform_n16(),
        "kuniform-dual-n16",
        "Dual of 2-uniform (3³.4²; 3².4.3.4)",
        "Face-centroid dual of the 2-uniform (3³.4²; 3².4.3.4) tiling (p4g) — eight plus four congruent pentagons per cell (the key-lock tiling).",
    )
    .expect("kuniform-n16 dual: non-degenerate lattice")
}

/// Override a dual's pattern coloring with the checker (`(i + j) mod 2`)
/// seed pattern and an explicit per-face seed vector, replacing the
/// congruence-class `Uniform` seeds from [`proper_seeds`].
///
/// Required when a dual face is edge-adjacent to one of its own lattice
/// translates: a `Uniform` color (one seed per face, constant across all
/// translates) then paints two edge-adjacent copies identically. The
/// checker base offsets such self-adjacent copies by one color, and
/// `seeds` must be a proper coloring of the edge-adjacency graph on top
/// of that base. The seed values are exhaustive-searched over
/// (face seed) × (shared-edge lattice step) and pinned by
/// `dual_pattern_mode_is_proper_coloring`. `seeds` follows the
/// [`dual_cells`] cell order (ascending orbit key).
fn with_checker_seeds(mut tiling: LatticeTiling, seeds: &[u32]) -> LatticeTiling {
    assert_eq!(
        tiling.cells.len(),
        seeds.len(),
        "checker seed override length must match the dual's cell count"
    );
    tiling.seed_pattern = SeedPattern::Checker;
    for (c, s) in tiling.cells.iter_mut().zip(seeds) {
        c.seed = *s;
    }
    tiling
}

/// Face-centroid dual of `kuniform-n4` (2-uniform (4⁴; 3³.4²)₁, cmm).
///
/// Same lattice as the primal, a1 = (1, 0), a2 = (1/2, 2+√3/2),
/// |det| = 2+√3/2 (centered-rectangular). The cell holds the 3 dual
/// faces of the primal's 3 vertex orbits: two congruent pentagons
/// (over the two 3³.4² vertex orbits; sides 1/√3 ×2, (3+√3)/6 ×2, 1
/// in cyclic order; area 1/2+√3/4) and one square (over the 4⁴
/// vertex orbit; side 1, area 1). The three areas sum to 2+√3/2 =
/// |det|. Measured symmetry cmm (the dual keeps the primal's group):
/// 2-fold centers at (0, 1) and (1/2, 1) on the mirror
/// intersections plus the glide-only off-mirror centers at
/// (1/4, 2+√3/4) and (3/4, 2+√3/4) — the cmm-vs-pmm discriminator;
/// horizontal mirrors y = 1 + m(2+√3/2), vertical mirrors x = k/2,
/// horizontal glides y = 2+√3/4 + m(2+√3/2) (vector (±1/2, 0)) and
/// vertical glides x = 1/4, 3/4 (vector (0, ±(2+√3/2))).
///
/// Every dual face is edge-adjacent to one of its own lattice
/// translates — each primal vertex orbit runs along a horizontal edge
/// string (the 4⁴ orbit on the square-band interior lines, the two
/// 3³.4² orbits on the square/triangle junction lines), so a `Uniform`
/// pattern color (one seed per face) would paint two edge-adjacent
/// copies identically. `SeedPattern::Checker` offsets the seed by the
/// (i + j) lattice parity, and the per-face seeds [0, 2, 0] — in the
/// `dual_cells` orbit-key cell order [pentagon (0,0), pentagon
/// (651085,697831), square (825542,348915)] — are a proper 4-coloring
/// on top of that base: self-adjacent (1, 0) copies differ by one
/// color and every cross-face adjacency lands on a distinct color
/// (colors {0, 1, 2, 3}; same scheme as `prismatic-pentagonal` in the
/// Laves family; seed values exhaustive-searched and pinned by
/// `dual_pattern_mode_is_proper_coloring`).
pub fn kuniform_dual_n4() -> LatticeTiling {
    with_checker_seeds(
        build(
            &kuniform::kuniform_n4(),
            "kuniform-dual-n4",
            "Dual of 2-uniform (4⁴; 3³.4²)₁",
            "Face-centroid dual of the 2-uniform (4⁴; 3³.4²)₁ tiling (cmm) — two pentagons plus one square per cell.",
        )
        .expect("kuniform-n4 dual: non-degenerate lattice"),
        &[0, 2, 0],
    )
}

/// Face-centroid dual of `kuniform-n3` (2-uniform (4⁴; 3³.4²)₂, cmm).
///
/// Same lattice as the primal, a1 = (1, 0), a2 = (1/2, 3+√3/2),
/// |det| = 3+√3/2 (centered-rectangular). The cell holds the 4 dual
/// faces of the primal's 4 vertex orbits: two pentagons (over the two
/// 3³.4² vertex orbits; sides 1/√3 ×2, (3+√3)/6 ×2, 1 in cyclic
/// order; area 1/2+√3/4) and two squares (over the two 4⁴ vertex
/// orbits; side 1, area 1). The four areas sum to 3+√3/2 = |det|.
/// Measured symmetry cmm (the dual keeps the primal's group):
/// 2-fold centers at (0, 3/2) and (1/2, 3/2) on the mirror
/// intersections plus the glide-only off-mirror centers at
/// (1/4, 3+√3/4) and (3/4, 3+√3/4) — the cmm-vs-pmm discriminator;
/// horizontal mirrors y = 3/2 + m(3+√3/2), vertical mirrors x = k/2,
/// horizontal glides y = 3+√3/4 + m(3+√3/2) (vector (±1/2, 0)) and
/// vertical glides x = 1/4, 3/4 (vector (0, ±(3+√3/2))).
///
/// Same pattern-coloring scheme as [`kuniform_dual_n4`]: every dual
/// face is edge-adjacent to one of its own lattice translates, so the
/// `Uniform` seeds are infeasible; the checker base with per-face
/// seeds [0, 2, 0, 2] — in the `dual_cells` orbit-key cell order
/// [pentagon (0,0), pentagon (612005,775991), square (741336,517327),
/// square (870668,258664)] — is a proper 4-coloring on top of that
/// base (colors {0, 1, 2, 3}; same scheme as `prismatic-pentagonal`
/// in the Laves family; seed values exhaustive-searched and pinned by
/// `dual_pattern_mode_is_proper_coloring`).
pub fn kuniform_dual_n3() -> LatticeTiling {
    with_checker_seeds(
        build(
            &kuniform::kuniform_n3(),
            "kuniform-dual-n3",
            "Dual of 2-uniform (4⁴; 3³.4²)₂",
            "Face-centroid dual of the 2-uniform (4⁴; 3³.4²)₂ tiling (cmm) — two pentagons plus two squares per cell.",
        )
        .expect("kuniform-n3 dual: non-degenerate lattice"),
        &[0, 2, 0, 2],
    )
}

#[cfg(test)]
mod dual_tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet, HashMap};
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::ParamSet;
    use tess_core::symmetry::WallpaperGroup;
    use tess_core::tiling::{Query, Tiling};

    use crate::kuniform;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::symmetry::LatticeKind;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// One dual under test, with the machine-derived face-class census.
    /// `classes` lists (vertex count, count in cell, exact per-face area)
    /// for each congruence class, machine-derived during M98/M102/M106.
    /// `group` is the wallpaper group measured by the symmetry probes
    /// below (the centroid dual is equivariant and involutive, so every
    /// dual keeps its primal's group: p6m duals stay p6m, the p6 dual
    /// stays chiral, the pmm/cmm duals stay pmm/cmm). `center` is the
    /// group's special center in cell coordinates: the 6-fold center
    /// for the P6m/P6 cases (the origin for the M98 batch and
    /// `kuniform-dual-n20`/`kuniform-dual-n19`; off-origin for the M102
    /// duals `n5`/`n1`/`n13`) and a mirror-intersection 2-fold center
    /// for the Pmm/Cmm cases (M106). `lattice_kind` is the shared
    /// lattice's kind (the dual keeps the primal's lattice).
    /// `mirror_dirs` are the measured mirror-line directions (unit
    /// vectors) through `center` for the Pmm/Cmm cases — horizontal and
    /// vertical for both; the P6m/P6 cases probe the six k·30° axes
    /// through their 6-fold center instead, so the field is unused
    /// there. The P4m case (`kuniform-dual-n2`) has its 4-fold center
    /// at the origin with all four mirror classes through it; the P4g
    /// case (`kuniform-dual-n16`) probes rotations about its 4-fold
    /// center and mirrors through a separate mirror-intersection 2-fold
    /// center (a p4g mirror never meets its 4-fold center), with
    /// `mirror_dirs` giving the two pure (axis) directions there.
    /// `axis_pt` is that mirror-intersection point for the P4g case
    /// (where the measured mirrors avoid the 4-fold `center`); unused
    /// (zero) elsewhere.
    struct Case {
        id: &'static str,
        make: fn() -> LatticeTiling,
        primal: fn() -> LatticeTiling,
        classes: [(usize, usize, f64); 2],
        group: WallpaperGroup,
        center: Vec2,
        lattice_kind: LatticeKind,
        mirror_dirs: [Vec2; 2],
        axis_pt: Vec2,
    }

    const S3: f64 = 1.7320508075688773; // √3
    const H: f64 = 0.8660254037844386; // √3/2

    fn cases() -> Vec<Case> {
        // The P6m/P6 cases leave `mirror_dirs` unset for the probes
        // (they use the six k·30° axes about the 6-fold center); the
        // value below is what the Pmm/Cmm cases measure, and is shared
        // for readability.
        let axis_mirrors = [v2(1.0, 0.0), v2(0.0, 1.0)];
        vec![
            Case {
                id: "kuniform-dual-p6m",
                make: kuniform_dual_p6m,
                primal: kuniform::kuniform_p6m,
                classes: [
                    (6, 1, H),              // regular hexagon, side 1/√3
                    (4, 6, 2.0 * S3 / 3.0), // kite: 1/√3, 2/√3, 2/√3, √3
                ],
                group: WallpaperGroup::P6m,
                center: Vec2::ZERO,
                lattice_kind: LatticeKind::Hexagonal,
                mirror_dirs: axis_mirrors,
                axis_pt: Vec2::ZERO,
            },
            Case {
                id: "kuniform-dual-n18",
                make: kuniform_dual_n18,
                primal: kuniform::kuniform_n18,
                classes: [
                    (6, 1, H),               // regular hexagon, side 1/√3
                    (5, 6, (2.0 + S3) / 4.0), // 3².4.3.4 dual pentagon
                ],
                group: WallpaperGroup::P6m,
                center: Vec2::ZERO,
                lattice_kind: LatticeKind::Hexagonal,
                mirror_dirs: axis_mirrors,
                axis_pt: Vec2::ZERO,
            },
            Case {
                id: "kuniform-dual-n9",
                make: kuniform_dual_n9,
                primal: kuniform::kuniform_n9,
                classes: [
                    (4, 6, (3.0 + 2.0 * S3) / 6.0), // 3.4.6.4 dual quad
                    (5, 6, (2.0 + S3) / 4.0),       // 3².4.3.4 dual pentagon
                ],
                group: WallpaperGroup::P6m,
                center: Vec2::ZERO,
                lattice_kind: LatticeKind::Hexagonal,
                mirror_dirs: axis_mirrors,
                axis_pt: Vec2::ZERO,
            },
            Case {
                id: "kuniform-dual-n8",
                make: kuniform_dual_n8,
                primal: kuniform::kuniform_n8,
                classes: [
                    (4, 6, (3.0 + 2.0 * S3) / 6.0), // 3.4.6.4 dual quad
                    (5, 6, (2.0 + S3) / 4.0),       // 3³.4² dual pentagon
                ],
                group: WallpaperGroup::P6m,
                center: Vec2::ZERO,
                lattice_kind: LatticeKind::Hexagonal,
                mirror_dirs: axis_mirrors,
                axis_pt: Vec2::ZERO,
            },
            Case {
                id: "kuniform-dual-n5",
                make: kuniform_dual_n5,
                primal: kuniform::kuniform_n5,
                classes: [
                    (4, 6, (3.0 + 2.0 * S3) / 6.0),  // 3.4.6.4-ordered quad
                    (4, 12, (3.0 + 2.0 * S3) / 6.0), // 3.4².6-ordered quad
                ],
                group: WallpaperGroup::P6m,
                center: v2(3.0 + S3, 2.0 + 2.0 * S3),
                lattice_kind: LatticeKind::Hexagonal,
                mirror_dirs: axis_mirrors,
                axis_pt: Vec2::ZERO,
            },
            Case {
                id: "kuniform-dual-n1",
                make: kuniform_dual_n1,
                primal: kuniform::kuniform_n1,
                classes: [
                    (3, 12, (3.0 + 2.0 * S3) / 4.0), // 4.6.12 dual triangle (30-60-90)
                    (4, 6, (3.0 + 2.0 * S3) / 6.0),  // 3.4.6.4 dual quad
                ],
                group: WallpaperGroup::P6m,
                center: v2(3.0 * (1.0 + S3) / 2.0, (3.0 + S3) / 2.0),
                lattice_kind: LatticeKind::Hexagonal,
                mirror_dirs: axis_mirrors,
                axis_pt: Vec2::ZERO,
            },
            Case {
                id: "kuniform-dual-n13",
                make: kuniform_dual_n13,
                primal: kuniform::kuniform_n13,
                classes: [
                    (4, 12, (9.0 + 5.0 * S3) / 12.0), // 3².4.12 dual quad
                    (6, 2, H),                         // regular hexagon, side 1/√3
                ],
                group: WallpaperGroup::P6m,
                center: v2(5.0 * (1.0 + S3) / 2.0, 5.0 * (3.0 + S3) / 2.0),
                lattice_kind: LatticeKind::Hexagonal,
                mirror_dirs: axis_mirrors,
                axis_pt: Vec2::ZERO,
            },
            Case {
                id: "kuniform-dual-n20",
                make: kuniform_dual_n20,
                primal: kuniform::kuniform_n20,
                classes: [
                    (5, 6, 7.0 * S3 / 12.0), // 3⁴.6 dual pentagon
                    (6, 6, H),               // regular hexagon, side 1/√3
                ],
                group: WallpaperGroup::P6,
                center: Vec2::ZERO,
                lattice_kind: LatticeKind::Hexagonal,
                mirror_dirs: axis_mirrors,
                axis_pt: Vec2::ZERO,
            },
            Case {
                id: "kuniform-dual-pmm",
                make: kuniform_dual_pmm,
                primal: kuniform::kuniform_pmm,
                classes: [
                    (4, 2, 2.0 * S3 / 3.0), // 3².6²-ordered kite: 1/√3, 2/√3, 2/√3, √3
                    (4, 1, 2.0 * S3 / 3.0), // 3.6.3.6-ordered rhombus, side 2/√3
                ],
                group: WallpaperGroup::Pmm,
                center: v2(1.0, 0.0), // rhombus center (a 2-fold center)
                lattice_kind: LatticeKind::Rectangular,
                mirror_dirs: axis_mirrors,
                axis_pt: Vec2::ZERO,
            },
            Case {
                id: "kuniform-dual-n12",
                make: kuniform_dual_n12,
                primal: kuniform::kuniform_n12,
                classes: [
                    (4, 2, 2.0 * S3 / 3.0),   // 3².6²-ordered kite: 1/√3, 2/√3, 2/√3, √3
                    (5, 2, 7.0 * S3 / 12.0),  // 3⁴.6-ordered pentagon
                ],
                group: WallpaperGroup::Cmm,
                center: Vec2::ZERO, // dual vertex (a 2-fold center on the mirrors)
                lattice_kind: LatticeKind::CenteredRectangular,
                mirror_dirs: axis_mirrors,
                axis_pt: Vec2::ZERO,
            },
            Case {
                id: "kuniform-dual-n19",
                make: kuniform_dual_n19,
                primal: kuniform::kuniform_n19,
                classes: [
                    (5, 6, 7.0 * S3 / 12.0), // 3⁴.6-ordered pentagon
                    (6, 2, H),               // regular hexagon, side 1/√3
                ],
                group: WallpaperGroup::P6m,
                center: Vec2::ZERO, // 6-fold center (the primal's flower center)
                lattice_kind: LatticeKind::Hexagonal,
                mirror_dirs: axis_mirrors,
                axis_pt: Vec2::ZERO,
            },
            Case {
                id: "kuniform-dual-n2",
                make: kuniform_dual_n2,
                primal: kuniform::kuniform_n2,
                classes: [
                    // Kite over the 3.4.3.12 vertices.
                    (4, 4, 3.0 / 4.0 + 5.0 * S3 / 12.0),
                    // Isosceles triangle over the 3.12.12 vertices.
                    (3, 4, 1.0 + 7.0 * S3 / 12.0),
                ],
                group: WallpaperGroup::P4m,
                center: Vec2::ZERO, // 4-fold center (the primal square center)
                lattice_kind: LatticeKind::Square,
                mirror_dirs: axis_mirrors,
                axis_pt: Vec2::ZERO,
            },
            Case {
                id: "kuniform-dual-n16",
                make: kuniform_dual_n16,
                primal: kuniform::kuniform_n16,
                classes: [
                    // Pentagon over the 3².4.3.4 vertices.
                    (5, 8, 0.5 + S3 / 4.0),
                    // Pentagon over the 3³.4² vertices (same area).
                    (5, 4, 0.5 + S3 / 4.0),
                ],
                group: WallpaperGroup::P4g,
                // Measured 4-fold center; the mirrors avoid it.
                center: v2((S3 - 1.0) / 4.0, (5.0 + S3) / 4.0),
                lattice_kind: LatticeKind::Square,
                mirror_dirs: axis_mirrors,
                // Corner-2 point where the two pure axis mirrors meet.
                axis_pt: v2(-1.0, 0.5),
            },
            Case {
                id: "kuniform-dual-n4",
                make: kuniform_dual_n4,
                primal: kuniform::kuniform_n4,
                classes: [
                    // Pentagons over the two 3³.4² vertex orbits.
                    (5, 2, 0.5 + H / 2.0),
                    // Square over the 4⁴ vertex orbit.
                    (4, 1, 1.0),
                ],
                group: WallpaperGroup::Cmm,
                // Measured 2-fold on the horizontal mirror (cmm).
                center: v2(0.5, 1.0),
                lattice_kind: LatticeKind::CenteredRectangular,
                mirror_dirs: axis_mirrors,
                axis_pt: Vec2::ZERO,
            },
            Case {
                id: "kuniform-dual-n3",
                make: kuniform_dual_n3,
                primal: kuniform::kuniform_n3,
                classes: [
                    // Pentagons over the two 3³.4² vertex orbits.
                    (5, 2, 0.5 + H / 2.0),
                    // Squares over the two 4⁴ vertex orbits.
                    (4, 2, 1.0),
                ],
                group: WallpaperGroup::Cmm,
                // Measured 2-fold on the horizontal mirror (cmm).
                center: v2(0.5, 1.5),
                lattice_kind: LatticeKind::CenteredRectangular,
                mirror_dirs: axis_mirrors,
                axis_pt: Vec2::ZERO,
            },
        ]
    }

    fn signed_area(pts: &[Vec2]) -> f64 {
        let mut s = 0.0;
        for i in 0..pts.len() {
            let (a, b) = (pts[i], pts[(i + 1) % pts.len()]);
            s += a.x * b.y - b.x * a.y;
        }
        s / 2.0
    }

    /// Undirected edge key on the 1e-6 grid (AGENTS.md edge-parity
    /// convention; shared edges between lattice translates agree to ~1 ulp).
    fn edge_key(a: Vec2, b: Vec2) -> [i64; 4] {
        let mut k = [
            (a.x * 1e6).round() as i64,
            (a.y * 1e6).round() as i64,
            (b.x * 1e6).round() as i64,
            (b.y * 1e6).round() as i64,
        ];
        if (k[0], k[1]) > (k[2], k[3]) {
            k.swap(0, 2);
            k.swap(1, 3);
        }
        k
    }

    /// The partner table edge for every table edge of `t`: for each edge
    /// of cell `ci`, the unique `(cell, lattice offset)` whose drawn edge
    /// is a lattice translate of it. Panics if any edge has a different
    /// number of partners than one (a table that is not edge-to-edge at
    /// the period level). Result is indexed per cell: `partners[ci][e]`.
    fn edge_partners(t: &LatticeTiling) -> Vec<Vec<(usize, i64, i64)>> {
        let (a1, a2) = (t.lattice[0], t.lattice[1]);
        let inv = LatticeInv::new(a1, a2).expect("non-degenerate lattice");
        let n = t.cells.len();
        let mut partners = Vec::new();
        for ci in 0..n {
            let pts = &t.cells[ci].poly.pts;
            let m = pts.len();
            let mut row = Vec::new();
            for e in 0..m {
                let p = pts[e];
                let q = pts[(e + 1) % m];
                let mut found: Vec<(usize, i64, i64)> = Vec::new();
                for cj in 0..n {
                    let p2 = &t.cells[cj].poly.pts;
                    let m2 = p2.len();
                    for f in 0..m2 {
                        if cj == ci && f == e {
                            continue;
                        }
                        for (pa, pb) in [
                            (p2[f], p2[(f + 1) % m2]),
                            (p2[(f + 1) % m2], p2[f]),
                        ] {
                            for (ta, tb) in [(p, q), (q, p)] {
                                let s = pa - ta;
                                let u = inv.apply(s);
                                if (u.x - u.x.round()).abs() > 1e-6
                                    || (u.y - u.y.round()).abs() > 1e-6
                                {
                                    continue;
                                }
                                if (pb - tb - s).length() > 1e-9 {
                                    continue;
                                }
                                found.push((cj, u.x.round() as i64, u.y.round() as i64));
                                break;
                            }
                        }
                    }
                }
                // Dedup: the same partner edge is one solution, but a
                // non-edge-to-edge table could carry several.
                found.sort();
                found.dedup();
                assert_eq!(
                    found.len(),
                    1,
                    "table edge {} of cell {} has {} partners",
                    e,
                    ci,
                    found.len()
                );
                row.push(found[0]);
            }
            partners.push(row);
        }
        partners
    }

    // ------------------------------------------------------------------
    // Per-dual coverage trio (house pattern).
    // ------------------------------------------------------------------

    #[test]
    fn kuniform_dual_p6m_covers_region() {
        check_covers(
            &kuniform_dual_p6m(),
            "kuniform-dual-p6m",
            BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_dual_p6m_rotated_scaled() {
        check_rotated(
            &kuniform_dual_p6m(),
            "kuniform-dual-p6m(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_dual_p6m_deterministic() {
        check_det(
            &kuniform_dual_p6m(),
            "kuniform-dual-p6m",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn kuniform_dual_n18_covers_region() {
        check_covers(
            &kuniform_dual_n18(),
            "kuniform-dual-n18",
            BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_dual_n18_rotated_scaled() {
        check_rotated(
            &kuniform_dual_n18(),
            "kuniform-dual-n18(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_dual_n18_deterministic() {
        check_det(
            &kuniform_dual_n18(),
            "kuniform-dual-n18",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn kuniform_dual_n9_covers_region() {
        check_covers(
            &kuniform_dual_n9(),
            "kuniform-dual-n9",
            BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_dual_n9_rotated_scaled() {
        check_rotated(
            &kuniform_dual_n9(),
            "kuniform-dual-n9(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_dual_n9_deterministic() {
        check_det(
            &kuniform_dual_n9(),
            "kuniform-dual-n9",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn kuniform_dual_n8_covers_region() {
        check_covers(
            &kuniform_dual_n8(),
            "kuniform-dual-n8",
            BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_dual_n8_rotated_scaled() {
        check_rotated(
            &kuniform_dual_n8(),
            "kuniform-dual-n8(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_dual_n8_deterministic() {
        check_det(
            &kuniform_dual_n8(),
            "kuniform-dual-n8",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn kuniform_dual_n5_covers_region() {
        check_covers(
            &kuniform_dual_n5(),
            "kuniform-dual-n5",
            BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_dual_n5_rotated_scaled() {
        check_rotated(
            &kuniform_dual_n5(),
            "kuniform-dual-n5(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_dual_n5_deterministic() {
        check_det(
            &kuniform_dual_n5(),
            "kuniform-dual-n5",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn kuniform_dual_n1_covers_region() {
        check_covers(
            &kuniform_dual_n1(),
            "kuniform-dual-n1",
            BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_dual_n1_rotated_scaled() {
        check_rotated(
            &kuniform_dual_n1(),
            "kuniform-dual-n1(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_dual_n1_deterministic() {
        check_det(
            &kuniform_dual_n1(),
            "kuniform-dual-n1",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn kuniform_dual_n13_covers_region() {
        check_covers(
            &kuniform_dual_n13(),
            "kuniform-dual-n13",
            BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_dual_n13_rotated_scaled() {
        check_rotated(
            &kuniform_dual_n13(),
            "kuniform-dual-n13(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_dual_n13_deterministic() {
        check_det(
            &kuniform_dual_n13(),
            "kuniform-dual-n13",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn kuniform_dual_n20_covers_region() {
        check_covers(
            &kuniform_dual_n20(),
            "kuniform-dual-n20",
            BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_dual_n20_rotated_scaled() {
        check_rotated(
            &kuniform_dual_n20(),
            "kuniform-dual-n20(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_dual_n20_deterministic() {
        check_det(
            &kuniform_dual_n20(),
            "kuniform-dual-n20",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn kuniform_dual_pmm_covers_region() {
        check_covers(
            &kuniform_dual_pmm(),
            "kuniform-dual-pmm",
            BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_dual_pmm_rotated_scaled() {
        check_rotated(
            &kuniform_dual_pmm(),
            "kuniform-dual-pmm(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_dual_pmm_deterministic() {
        check_det(
            &kuniform_dual_pmm(),
            "kuniform-dual-pmm",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn kuniform_dual_n12_covers_region() {
        check_covers(
            &kuniform_dual_n12(),
            "kuniform-dual-n12",
            BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_dual_n12_rotated_scaled() {
        check_rotated(
            &kuniform_dual_n12(),
            "kuniform-dual-n12(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_dual_n12_deterministic() {
        check_det(
            &kuniform_dual_n12(),
            "kuniform-dual-n12",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn kuniform_dual_n19_covers_region() {
        check_covers(
            &kuniform_dual_n19(),
            "kuniform-dual-n19",
            BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_dual_n19_rotated_scaled() {
        check_rotated(
            &kuniform_dual_n19(),
            "kuniform-dual-n19(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_dual_n19_deterministic() {
        check_det(
            &kuniform_dual_n19(),
            "kuniform-dual-n19",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn kuniform_dual_n2_covers_region() {
        check_covers(
            &kuniform_dual_n2(),
            "kuniform-dual-n2",
            BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_dual_n2_rotated_scaled() {
        check_rotated(
            &kuniform_dual_n2(),
            "kuniform-dual-n2(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_dual_n2_deterministic() {
        check_det(
            &kuniform_dual_n2(),
            "kuniform-dual-n2",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn kuniform_dual_n16_covers_region() {
        check_covers(
            &kuniform_dual_n16(),
            "kuniform-dual-n16",
            BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_dual_n16_rotated_scaled() {
        check_rotated(
            &kuniform_dual_n16(),
            "kuniform-dual-n16(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_dual_n16_deterministic() {
        check_det(
            &kuniform_dual_n16(),
            "kuniform-dual-n16",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn kuniform_dual_n4_covers_region() {
        check_covers(
            &kuniform_dual_n4(),
            "kuniform-dual-n4",
            BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_dual_n4_rotated_scaled() {
        check_rotated(
            &kuniform_dual_n4(),
            "kuniform-dual-n4(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_dual_n4_deterministic() {
        check_det(
            &kuniform_dual_n4(),
            "kuniform-dual-n4",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    #[test]
    fn kuniform_dual_n3_covers_region() {
        check_covers(
            &kuniform_dual_n3(),
            "kuniform-dual-n3",
            BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
        );
    }

    #[test]
    fn kuniform_dual_n3_rotated_scaled() {
        check_rotated(
            &kuniform_dual_n3(),
            "kuniform-dual-n3(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn kuniform_dual_n3_deterministic() {
        check_det(
            &kuniform_dual_n3(),
            "kuniform-dual-n3",
            BBox::new(v2(-3.0, -3.0), v2(10.0, 10.0)),
        );
    }

    // ------------------------------------------------------------------
    // Geometry census: convexity, orientation, exact areas, face classes.
    // ------------------------------------------------------------------

    #[test]
    fn dual_faces_convex_ccw_and_areas_close() {
        for c in cases() {
            let dual = (c.make)();
            let det = dual.lattice[0].cross(dual.lattice[1]).abs();
            let mut total = 0.0;
            for (i, cell) in dual.cells.iter().enumerate() {
                let pts = &cell.poly.pts;
                let sa = signed_area(pts);
                assert!(
                    sa > 0.0,
                    "{}: cell {} is not CCW (signed area {})",
                    c.id,
                    i,
                    sa
                );
                for k in 0..pts.len() {
                    let e_in = pts[k] - pts[(k + pts.len() - 1) % pts.len()];
                    let e_out = pts[(k + 1) % pts.len()] - pts[k];
                    assert!(
                        e_in.cross(e_out) > -1e-12,
                        "{}: cell {} not convex at vertex {}",
                        c.id,
                        i,
                        k
                    );
                }
                total += sa;
            }
            assert!(
                (total - det).abs() <= 1e-9 * det,
                "{}: sum of face areas {} != |det| {} (rel {})",
                c.id,
                total,
                det,
                (total - det) / det
            );
            // Congruence classes: (vertex count, sorted 1e-9 side lengths).
            let mut classes: BTreeMap<(usize, Vec<i64>), (usize, f64)> = BTreeMap::new();
            for cell in &dual.cells {
                let pts = &cell.poly.pts;
                let n = pts.len();
                let mut sides: Vec<i64> = (0..n)
                    .map(|k| ((pts[(k + 1) % n] - pts[k]).length() * 1e9).round() as i64)
                    .collect();
                sides.sort_unstable();
                let e = classes.entry((n, sides)).or_insert((0, 0.0));
                e.0 += 1;
                e.1 += signed_area(pts);
            }
            assert_eq!(
                classes.len(),
                2,
                "{}: expected 2 congruence classes, got {}",
                c.id,
                classes.len()
            );
            for (nv, count, exact_area) in c.classes {
                // Match by (vertex count, count in cell): a dual may carry
                // two classes of the same arity (kuniform-dual-n5: two
                // quadrilateral classes of 6 and 12 faces).
                let (cnt, area) = classes
                    .iter()
                    .find(|((n, _), (cc, _))| *n == nv && *cc == count)
                    .map(|(_, v)| *v)
                    .unwrap_or_else(|| panic!("{}: missing class of {nv}-gons x{count}", c.id));
                assert_eq!(cnt, count, "{}: {}-gon class count", c.id, nv);
                // `area` is the class sum; the per-face area is area/cnt.
                let per = area / cnt as f64;
                assert!(
                    (per - exact_area).abs() <= 1e-9 * exact_area,
                    "{}: {}-gon per-face area {} vs exact {} (rel {})",
                    c.id,
                    nv,
                    per,
                    exact_area,
                    (per - exact_area) / exact_area
                );
            }
            // Seeds stay in the 8-color pattern palette.
            assert!(
                dual.cells.iter().all(|cell| cell.seed < 8),
                "{}: a pattern seed reached 8",
                c.id
            );
        }
    }

    // ------------------------------------------------------------------
    // Edge-to-edge on the *tiled* dual (window i, j in [-W, W]²).
    // ------------------------------------------------------------------

    #[test]
    fn dual_tilings_are_edge_to_edge() {
        let w = 4i64;
        for c in cases() {
            let dual = (c.make)();
            // Period-level check: every table edge has exactly one
            // partner table edge (a lattice translate of it).
            let partners = edge_partners(&dual);
            let (a1, a2) = (dual.lattice[0], dual.lattice[1]);
            // Tile the window and count each edge by rounded endpoints.
            let mut counts: HashMap<[i64; 4], (usize, bool)> = HashMap::new();
            for i in -w..=w {
                for j in -w..=w {
                    let tr = a1 * i as f64 + a2 * j as f64;
                    for (ci, cell) in dual.cells.iter().enumerate() {
                        let pts = &cell.poly.pts;
                        for e in 0..pts.len() {
                            let a = pts[e] + tr;
                            let b = pts[(e + 1) % pts.len()] + tr;
                            let (_pc, poi, pojj) = partners[ci][e];
                            let pi = i - poi;
                            let pj = j - pojj;
                            let inwin = (-w..=w).contains(&pi) && (-w..=w).contains(&pj);
                            let entry = counts.entry(edge_key(a, b)).or_insert((0, false));
                            entry.0 += 1;
                            entry.1 |= inwin;
                        }
                    }
                }
            }
            for (k, (n, inwin)) in &counts {
                assert!(
                    *n <= 2,
                    "{}: edge {k:?} shared by {} tiles (T-junction?)",
                    c.id,
                    n
                );
                if *inwin {
                    assert_eq!(*n, 2, "{}: unpaired interior edge {k:?}", c.id);
                } else {
                    assert_eq!(
                        *n, 1,
                        "{}: edge {k:?} appears {} times though its partner lies outside the window",
                        c.id, n
                    );
                }
            }
        }
    }

    // ------------------------------------------------------------------
    // Exactly-once coverage on a fine grid + clipped-area closure.
    // ------------------------------------------------------------------

    #[test]
    fn duals_cover_fine_grid_exactly_once() {
        let region = BBox::new(v2(-4.5, -4.5), v2(4.5, 4.5));
        let step = 0.25;
        let nsteps = (region.size().x / step) as usize;
        for c in cases() {
            let dual = (c.make)();
            let p = ParamSet::default_for(dual.params());
            let r = dual.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{}: {:?}", c.id, r.message);
            let tiles = &r.tiles;
            for gy in 0..nsteps {
                for gx in 0..nsteps {
                    let pt = region.min
                        + v2(step * (gx as f64 + 0.5), step * (gy as f64 + 0.5));
                    let cnt = tiles.iter().filter(|t| t.poly.contains(pt)).count();
                    assert!(cnt >= 1, "{}: gap at {pt}", c.id);
                    if cnt == 2 {
                        // Must be a point on the shared edge of the two tiles.
                        let idx: Vec<usize> = tiles
                            .iter()
                            .enumerate()
                            .filter(|(_, t)| t.poly.contains(pt))
                            .map(|(i, _)| i)
                            .collect();
                        let a = &tiles[idx[0]].poly;
                        let b = &tiles[idx[1]].poly;
                        let mut on_shared = false;
                        for e0 in 0..a.pts.len() {
                            let p0 = a.pts[e0];
                            let p1 = a.pts[(e0 + 1) % a.pts.len()];
                            let k = edge_key(p0, p1);
                            for e1 in 0..b.pts.len() {
                                if edge_key(b.pts[e1], b.pts[(e1 + 1) % b.pts.len()]) == k {
                                    let ab = p1 - p0;
                                    let t = ((pt - p0).dot(ab) / ab.dot(ab)).clamp(0.0, 1.0);
                                    on_shared = (p0 + ab * t - pt).length() <= 1e-6;
                                    break;
                                }
                            }
                            if on_shared {
                                break;
                            }
                        }
                        assert!(
                            on_shared,
                            "{}: double-covered point {pt} not on a shared edge",
                            c.id
                        );
                    } else if cnt > 2 {
                        let near = tiles
                            .iter()
                            .filter(|t| t.poly.pts.iter().any(|&v| (v - pt).length() <= 1e-6))
                            .count();
                        assert_eq!(
                            near, cnt,
                            "{}: vertex fan count {} != coverage {} at {pt}",
                            c.id,
                            near,
                            cnt
                        );
                    }
                }
            }
            // Clipped area closes against the region area.
            let mut area = 0.0;
            for t in tiles {
                if let Some(cl) = t.poly.clipped_to(&region) {
                    area += cl.area();
                }
            }
            let ra = region.area();
            assert!(
                (area - ra).abs() <= 1e-6 * ra,
                "{}: clipped area {area} vs region {ra} (rel {})",
                c.id,
                (area - ra) / ra
            );
        }
    }

    // ------------------------------------------------------------------
    // Pattern mode is a proper coloring of the dual-face adjacency.
    // ------------------------------------------------------------------

    #[test]
    fn dual_pattern_mode_is_proper_coloring() {
        let region = BBox::new(v2(-5.0, -4.0), v2(7.0, 6.0));
        for c in cases() {
            let dual = (c.make)();
            let p = ParamSet::default_for(dual.params());
            let r = dual.generate(&p, &Query { region });
            assert!(r.message.is_none(), "{}: {:?}", c.id, r.message);
            let mut edges: HashMap<[i64; 4], Vec<u32>> = HashMap::new();
            for t in &r.tiles {
                let n = t.poly.pts.len();
                for w in 0..n {
                    let a = t.poly.pts[w];
                    let b = t.poly.pts[(w + 1) % n];
                    edges.entry(edge_key(a, b)).or_default().push(t.seed % 8);
                }
            }
            for (k, seeds) in &edges {
                assert!(
                    seeds.len() <= 2,
                    "{}: edge {k:?} shared by {} tiles (not edge-to-edge?)",
                    c.id,
                    seeds.len()
                );
                if seeds.len() == 2 {
                    assert!(
                        seeds[0] != seeds[1],
                        "{}: adjacent tiles share pattern color {} (edge {k:?})",
                        c.id,
                        seeds[0]
                    );
                }
            }
        }
    }

    // ------------------------------------------------------------------
    // Symmetry: each dual matches its measured wallpaper group about
    // its special center. The M98 batch, kuniform-dual-n20 and
    // kuniform-dual-n19 carry their 6-fold center at the origin; the
    // M102 duals n5/n1/n13 have their 6-fold centers off the origin
    // (the origin is only a 3-fold / 2-fold center of those tables).
    // P6m cases: C6 + six mirrors through the 6-fold center positive.
    // The P6 case: C6 positive, every mirror negative (chiral).
    // R30 is never a 6-fold-center symmetry. Pmm/Cmm cases (M106,
    // rectangular / centered-rectangular lattices): R180 about the
    // measured mirror-intersection 2-fold center positive, no higher
    // rotation (R30/R60/R90/R120 negative), the two measured
    // axis-parallel mirrors through the center positive, the 45°
    // bisector mirror negative, R180 about a non-center negative.
    // ------------------------------------------------------------------

    /// Cell-set symmetry probe (same method as `kuniform_tests::
    /// probe_symmetry`): `g` must send every table cell to some table
    /// cell plus a lattice translation (Cramer solve, integrality
    /// 1e-6, vertices 1e-9). With `symmetry = false` at least one cell
    /// must fail to map. `g` may be centered anywhere (a rotation about
    /// a 6-fold center off the origin, a mirror through a line not
    /// through the origin): the lattice check applies to `g`'s linear
    /// part, `g(x) - g(0)`, which is a lattice automorphism whenever the
    /// map is a true symmetry of the lattice tiling.
    fn probe_dual_symmetry(
        dual: &LatticeTiling,
        g: impl Fn(Vec2) -> Vec2,
        symmetry: bool,
        label: &str,
    ) {
        let (b1, b2) = (dual.lattice[0], dual.lattice[1]);
        let det = b1.cross(b2);
        let inv = |p: Vec2| -> Vec2 {
            Vec2::new(
                (b2.y * p.x - b2.x * p.y) / det,
                (-b1.y * p.x + b1.x * p.y) / det,
            )
        };
        // A symmetry must preserve the lattice. Check the linear part
        // (the map may carry a translation: probes are about a center,
        // not necessarily the origin).
        let t = g(Vec2::ZERO);
        for (bi, bv) in dual.lattice.iter().enumerate() {
            let u = inv(g(*bv) - t);
            let ok = (u.x - u.x.round()).abs() <= 1e-6 && (u.y - u.y.round()).abs() <= 1e-6;
            if symmetry {
                assert!(ok, "{label}: lattice vector {bi} not preserved");
            } else if !ok {
                return;
            }
        }
        let cells: Vec<Vec<Vec2>> = dual.cells.iter().map(|c| c.poly.pts.clone()).collect();
        let mut mismatched = 0;
        for cp in cells.iter() {
            let gp: Vec<Vec2> = cp.iter().map(|p| g(*p)).collect();
            let mut matched = false;
            for cj in &cells {
                let mut hit = false;
                for w in cj {
                    let t = gp[0] - *w;
                    let u = inv(t);
                    if (u.x - u.x.round()).abs() > 1e-6 || (u.y - u.y.round()).abs() > 1e-6 {
                        continue;
                    }
                    let tr = b1 * u.x.round() + b2 * u.y.round();
                    let shifted: Vec<Vec2> = cj.iter().map(|w| *w + tr).collect();
                    hit = gp
                        .iter()
                        .all(|p| shifted.iter().any(|s| (*p - *s).length() < 1e-9))
                        && shifted
                            .iter()
                            .all(|s| gp.iter().any(|p| (*p - *s).length() < 1e-9));
                    if hit {
                        break;
                    }
                }
                if hit {
                    matched = true;
                    break;
                }
            }
            if !matched {
                mismatched += 1;
            }
        }
        if symmetry {
            assert!(
                mismatched == 0,
                "{label}: {mismatched} of {} cells failed to map",
                cells.len()
            );
        } else {
            assert!(
                mismatched > 0,
                "{label}: expected a negative probe to fail"
            );
        }
    }

    #[test]
    fn duals_match_measured_groups() {
        for c in cases() {
            let dual = (c.make)();
            assert_eq!(dual.group, c.group, "{}: declared group", c.id);
            assert_eq!(
                dual.lattice_kind, c.lattice_kind, "{}: lattice kind", c.id
            );
            let deg = std::f64::consts::PI / 180.0;
            let ctr = c.center;
            match c.group {
                WallpaperGroup::P6m | WallpaperGroup::P6 => {
                    // C6 about the measured 6-fold center: positive for
                    // every case (both the p6m and the chiral p6 duals).
                    for k in 1..=5 {
                        let ang = (k as f64) * 60.0 * deg;
                        probe_dual_symmetry(
                            &dual,
                            |p| ctr + (p - ctr).rotated(ang),
                            true,
                            &format!("{} R{}", c.id, k * 60),
                        );
                    }
                    // Mirrors through the 6-fold center: positive exactly
                    // for the p6m cases; every one of them a negative
                    // probe for the p6 case (chirality is part of the
                    // measured group).
                    for k in 0..6 {
                        let phi = (k as f64) * 30.0 * deg;
                        let d = v2(phi.cos(), phi.sin());
                        let is_p6m = c.group == WallpaperGroup::P6m;
                        probe_dual_symmetry(
                            &dual,
                            move |p| ctr + d * (2.0 * (p - ctr).dot(d)) - (p - ctr),
                            is_p6m,
                            &format!("{} M{}", c.id, k * 30),
                        );
                    }
                    // R30 about the 6-fold center: never a symmetry (the
                    // rotation subgroup is C6, not C12).
                    probe_dual_symmetry(
                        &dual,
                        |p| ctr + (p - ctr).rotated(30.0 * deg),
                        false,
                        &format!("{} R30 (negative probe)", c.id),
                    );
                }
                WallpaperGroup::Pmm | WallpaperGroup::Cmm => {
                    // R180 about the measured 2-fold center (a mirror
                    // intersection): positive.
                    probe_dual_symmetry(
                        &dual,
                        |p| ctr + (ctr - p),
                        true,
                        &format!("{} R180 (2-fold center)", c.id),
                    );
                    // No higher-order rotation about that center: the
                    // rotation subgroup is C2 (R30/R60/R90/R120 all
                    // fail; on the rectangular lattices they also fail
                    // the lattice-automorphism check).
                    for k in [30.0, 60.0, 90.0, 120.0] {
                        probe_dual_symmetry(
                            &dual,
                            |p| ctr + (p - ctr).rotated(k * deg),
                            false,
                            &format!("{} R{} (negative probe)", c.id, k as i32),
                        );
                    }
                    // The two measured mirror directions through the
                    // center (horizontal + vertical for both duals):
                    // positive.
                    for (mi, d) in c.mirror_dirs.iter().enumerate() {
                        let d = *d;
                        probe_dual_symmetry(
                            &dual,
                            move |p| ctr + d * (2.0 * (p - ctr).dot(d)) - (p - ctr),
                            true,
                            &format!("{} mirror {} (through 2-fold center)", c.id, mi),
                        );
                    }
                    // The 45° bisector is not a mirror (the point group
                    // is 2mm with axis-parallel mirrors only).
                    let d = v2((45.0 * deg).cos(), (45.0 * deg).sin());
                    probe_dual_symmetry(
                        &dual,
                        move |p| ctr + d * (2.0 * (p - ctr).dot(d)) - (p - ctr),
                        false,
                        &format!("{} mirror 45 (negative probe)", c.id),
                    );
                    // R180 about a generic (non-center) point fails.
                    probe_dual_symmetry(
                        &dual,
                        |p| ctr + v2(0.13, 0.17) + (ctr + v2(0.13, 0.17) - p),
                        false,
                        &format!("{} R180 off-center (negative probe)", c.id),
                    );
                    // Glide closure: in pmm and cmm the 2-fold centers
                    // form the corner ∪ cell-center arrangement, so R180
                    // about the cell-center 2-fold composed with a
                    // mirror through ctr is a glide reflection in the
                    // measured group — positive for every pmm/cmm dual.
                    let q = ctr + (dual.lattice[0] + dual.lattice[1]) * 0.5;
                    for (mi, d) in c.mirror_dirs.iter().enumerate() {
                        let d = *d;
                        probe_dual_symmetry(
                            &dual,
                            move |p| {
                                let w = p - ctr;
                                let m = ctr + d * (2.0 * w.dot(d)) - w;
                                q * 2.0 - m
                            },
                            true,
                            &format!(
                                "{} glide {} (R180(q) o mirror)",
                                c.id, mi
                            ),
                        );
                    }
                }
                WallpaperGroup::P4g => {
                    // C4 about the measured 4-fold center: positive
                    // (p4g has 4-fold rotation centers — the pgg
                    // discriminator).
                    for k in 1..=3 {
                        let ang = (k as f64) * 90.0 * deg;
                        probe_dual_symmetry(
                            &dual,
                            |p| ctr + (p - ctr).rotated(ang),
                            true,
                            &format!("{} R{}", c.id, k * 90),
                        );
                    }
                    // Pure mirrors only in the two axis directions,
                    // through the measured corner-2 point (in p4g the
                    // mirrors never pass a 4-fold center): positive.
                    let q = c.axis_pt;
                    for (mi, d) in c.mirror_dirs.iter().enumerate() {
                        let d = *d;
                        probe_dual_symmetry(
                            &dual,
                            move |p| q + d * (2.0 * (p - q).dot(d)) - (p - q),
                            true,
                            &format!("{} mirror {} (through corner-2 point)", c.id, mi),
                        );
                    }
                    // R180 about the corner-2 point: positive (the
                    // product of the two mirrors above).
                    probe_dual_symmetry(
                        &dual,
                        |p| q + (q - p),
                        true,
                        &format!("{} R180 (corner-2 point)", c.id),
                    );
                    // The measured diagonal glide
                    // g(x, y) = (y + 3/2 + √3, x − √3/2): positive —
                    // the diagonal classes are true glides, not
                    // mirrors (this is what separates p4g from p4m
                    // positively, complementing the negative probes).
                    probe_dual_symmetry(
                        &dual,
                        |p| v2(p.y + 1.5 + S3, p.x - S3 / 2.0),
                        true,
                        &format!("{} diagonal glide", c.id),
                    );
                    // Every mirror margin through the 4-fold center
                    // fails (a p4m signature absent here).
                    for k in 0..4 {
                        let phi = (k as f64) * 45.0 * deg;
                        let d = v2(phi.cos(), phi.sin());
                        probe_dual_symmetry(
                            &dual,
                            move |p| ctr + d * (2.0 * (p - ctr).dot(d)) - (p - ctr),
                            false,
                            &format!(
                                "{} M{} through 4-fold center (negative probe)",
                                c.id,
                                k * 45
                            ),
                        );
                    }
                    // R90 about the corner-2 point fails (q is 2-fold only).
                    probe_dual_symmetry(
                        &dual,
                        |p| q + (p - q).rotated(90.0 * deg),
                        false,
                        &format!("{} R90 at corner-2 point (negative probe)", c.id),
                    );
                }
                WallpaperGroup::P4m => {
                    // C4 about the measured 4-fold center (the primal
                    // square center for `kuniform-dual-n2`): positive.
                    for k in 1..=3 {
                        let ang = (k as f64) * 90.0 * deg;
                        probe_dual_symmetry(
                            &dual,
                            |p| ctr + (p - ctr).rotated(ang),
                            true,
                            &format!("{} R{}", c.id, k * 90),
                        );
                    }
                    // All four mirror classes through the 4-fold
                    // center (axis, axis+45°, and their perpendiculars
                    // — the full D4 margin set): positive. This is
                    // exactly what p4g forbids (its mirrors avoid the
                    // 4-fold centers), so it separates the two.
                    for k in 0..4 {
                        let phi = (k as f64) * 45.0 * deg;
                        let d = v2(phi.cos(), phi.sin());
                        probe_dual_symmetry(
                            &dual,
                            move |p| ctr + d * (2.0 * (p - ctr).dot(d)) - (p - ctr),
                            true,
                            &format!("{} M{}", c.id, k * 45),
                        );
                    }
                    // R45 about the 4-fold center: never a symmetry
                    // (rotation subgroup C4, and on the square lattice
                    // the lattice check rejects it outright).
                    probe_dual_symmetry(
                        &dual,
                        |p| ctr + (p - ctr).rotated(45.0 * deg),
                        false,
                        &format!("{} R45 (negative probe)", c.id),
                    );
                    // A horizontal mirror translated off the measured
                    // family fails (no mirror at y = 0.17).
                    let d = v2(1.0, 0.0);
                    probe_dual_symmetry(
                        &dual,
                        move |p| {
                            let c2 = ctr + v2(0.0, 0.17);
                            c2 + d * (2.0 * (p - c2).dot(d)) - (p - c2)
                        },
                        false,
                        &format!("{} mirror 0 off-family (negative probe)", c.id),
                    );
                    // R180 about a generic (non-center) point fails.
                    probe_dual_symmetry(
                        &dual,
                        |p| {
                            let c2 = ctr + v2(0.13, 0.17);
                            c2 + (c2 - p)
                        },
                        false,
                        &format!("{} R180 off-center (negative probe)", c.id),
                    );
                }
                other => panic!("{}: unhandled group {:?} in symmetry probes", c.id, other),
            }
        }
    }

    // ------------------------------------------------------------------
    // Correspondence: dual cells == distinct primal vertex orbits, and
    // each dual face's vertex count equals the number of primal faces
    // incident to its orbit (recomputed independently via explicit
    // lattice translates).
    // ------------------------------------------------------------------

    #[test]
    fn dual_cells_are_primal_vertex_orbits() {
        for c in cases() {
            let primal = (c.primal)();
            let dual = (c.make)();
            let (a1, a2) = (primal.lattice[0], primal.lattice[1]);
            let inv = LatticeInv::new(a1, a2).unwrap();
            let mut orbits: BTreeSet<(i64, i64)> = BTreeSet::new();
            for cell in &primal.cells {
                for &p in &cell.poly.pts {
                    orbits.insert(orbit_key(&inv, p + primal.offset));
                }
            }
            assert_eq!(
                dual.cells.len(),
                orbits.len(),
                "{}: {} dual cells vs {} primal vertex orbits",
                c.id,
                dual.cells.len(),
                orbits.len()
            );
            // Independent recount: for each orbit, the incident tiles are
            // (cell ci, translation vrep - q) for every table vertex q of
            // orbit key k. Distinct (ci, translation) pairs are distinct
            // tiles, so the count must equal the dual face's degree.
            // `dual_cells` yields the (key, polygon) pairs in the same
            // BTreeMap order that `build` used, so the keys align with
            // `dual.cells` by index.
            let dcells = dual_cells(&primal).expect("dual of a valid primal");
            assert_eq!(dcells.len(), dual.cells.len());
            for (di, ((k, _), cell)) in dcells.iter().zip(dual.cells.iter()).enumerate() {
                let vrep = primal.offset
                    + a1 * (k.0 as f64 / ORBIT_GRID)
                    + a2 * (k.1 as f64 / ORBIT_GRID);
                let mut incident: BTreeSet<(usize, i64, i64)> = BTreeSet::new();
                for (ci, pc) in primal.cells.iter().enumerate() {
                    for &q in &pc.poly.pts {
                        if orbit_key(&inv, q + primal.offset) != *k {
                            continue;
                        }
                        let t = vrep - q;
                        let u = inv.apply(t);
                        assert!(
                            (u.x - u.x.round()).abs() <= 1e-6
                                && (u.y - u.y.round()).abs() <= 1e-6,
                            "{}: orbit representative not a lattice translate",
                            c.id
                        );
                        incident.insert((ci, u.x.round() as i64, u.y.round() as i64));
                    }
                }
                assert_eq!(
                    incident.len(),
                    cell.poly.pts.len(),
                    "{}: dual cell {} has {} vertices, orbit {:?} incident to {} tiles",
                    c.id,
                    di,
                    cell.poly.pts.len(),
                    k,
                    incident.len()
                );
            }
        }
    }


    // ------------------------------------------------------------------
    // Oracle: dual() on each Archimedean primal must reproduce the
    // hand-built Laves table exactly (same lattice, same cell count,
    // cells matching up to lattice translation + vertex rotation/reversal).
    // ------------------------------------------------------------------

    #[cfg(all(feature = "uniform", feature = "laves"))]
    mod oracle {
        use super::*;
        use crate::archimedean;
        use crate::laves;

        /// `d` matches `q` iff they are the same polygon up to a lattice
        /// translation and a cyclic vertex rotation or reversal.
        fn cell_matches(d: &Polygon, q: &Polygon, a1: Vec2, a2: Vec2, inv: &LatticeInv) -> bool {
            if d.pts.len() != q.pts.len() {
                return false;
            }
            let n = d.pts.len();
            for o in 0..2 {
                for s in 0..n {
                    let qv = |k: usize| -> Vec2 {
                        if o == 0 {
                            q.pts[(s + k) % n]
                        } else {
                            q.pts[(s + n - k) % n]
                        }
                    };
                    let l0 = d.pts[0] - qv(0);
                    let u = inv.apply(l0);
                    if (u.x - u.x.round()).abs() > 1e-6 || (u.y - u.y.round()).abs() > 1e-6 {
                        continue;
                    }
                    let tr = a1 * u.x.round() + a2 * u.y.round();
                    if (0..n)
                        .all(|k| (d.pts[k] - (qv(k) + tr)).length() < 1e-9)
                    {
                        return true;
                    }
                }
            }
            false
        }

        fn oracle(primal: &LatticeTiling, laves: &LatticeTiling, label: &str) {
            let d = dual(primal).unwrap_or_else(|| panic!("{label}: dual() returned None"));
            // 1. Same lattice basis, same offset.
            for i in 0..2 {
                let dl = (d.lattice[i] - laves.lattice[i]).length();
                assert!(dl < 1e-9, "{label}: lattice vector {i} differs by {dl}");
            }
            let doff = (d.offset - laves.offset).length();
            assert!(doff < 1e-9, "{label}: offset differs by {doff}");
            // 2. Same number of faces.
            assert_eq!(
                d.cells.len(),
                laves.cells.len(),
                "{label}: {} dual cells vs {} Laves cells",
                d.cells.len(),
                laves.cells.len()
            );
            // 3. Perfect bijection under translation + rotation/reversal.
            let n = d.cells.len();
            let (a1, a2) = (d.lattice[0], d.lattice[1]);
            let inv = LatticeInv::new(a1, a2).unwrap();
            let matches = |di: usize, lj: usize| {
                cell_matches(&d.cells[di].poly, &laves.cells[lj].poly, a1, a2, &inv)
            };
            assert!(
                dfs_match(0, n, &mut Vec::new(), &mut vec![false; n], &matches),
                "{label}: no perfect matching between dual cells and Laves cells"
            );
        }

        fn dfs_match(
            di: usize,
            n: usize,
            depth: &mut Vec<usize>,
            used: &mut Vec<bool>,
            matches: &dyn Fn(usize, usize) -> bool,
        ) -> bool {
            if di == n {
                return true;
            }
            debug_assert_eq!(depth.len(), di);
            for lj in 0..n {
                if used[lj] || !matches(di, lj) {
                    continue;
                }
                used[lj] = true;
                depth.push(lj);
                if dfs_match(di + 1, n, depth, used, matches) {
                    return true;
                }
                depth.pop();
                used[lj] = false;
            }
            false
        }

        #[test]
        fn dual_reproduces_laves_tables() {
            oracle(
                &archimedean::truncated_square(),
                &laves::tetrakis_square(),
                "4.8.8 -> tetrakis-square",
            );
            oracle(
                &archimedean::trihexagonal(),
                &laves::rhombille(),
                "3.6.3.6 -> rhombille",
            );
            oracle(
                &archimedean::truncated_hexagonal(),
                &laves::triakis_triangular(),
                "3.12.12 -> triakis-triangular",
            );
            oracle(
                &archimedean::rhombitrihexagonal(),
                &laves::deltoidal_trihexagonal(),
                "3.4.6.4 -> deltoidal-trihexagonal",
            );
            oracle(
                &archimedean::truncated_trihexagonal(),
                &laves::kisrhombille(),
                "4.6.12 -> kisrhombille",
            );
            oracle(
                &archimedean::elongated_triangular(),
                &laves::prismatic_pentagonal(),
                "3.3.3.4.4 -> prismatic-pentagonal",
            );
            oracle(
                &archimedean::snub_trihexagonal(),
                &laves::floret_pentagonal(),
                "3.3.3.3.6 -> floret-pentagonal (chiral, reversal allowed)",
            );
        }
    }
}
