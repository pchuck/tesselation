//! Hat tiling — the aperiodic monotile ("einstein") of Smith, Myers,
//! Kaplan and Goodman-Strauss (arXiv:2303.10798).
//!
//! The prototile is the *hat*: a 13-sided polykite assembled from eight
//! kites of the [3.4.6.4] Laves tiling. The hat tiles the plane, but
//! only aperiodically, and only together with its mirror image.
//!
//! # Exact geometry
//!
//! Everything lives in Eisenstein integers: a point `(a, b)` stands for
//! `a + b·ω` with `ω = e^{iπ/3}`; the world position is
//! `(a + b/2, (√3/2)·b)` — the same triangular-lattice map used by
//! `sphinx.rs` (and by Kaplan's reference renderer). The hat's 13
//! vertices are the constant [`HAT`]; in world units its sides are six
//! of length 1, six of length √3, and one of length 2 (two collinear
//! short kite edges), and its area is exactly 8√3. Orientations are the
//! 12 lattice symmetries (rotation by 60°·k, plus an optional
//! reflection): rotations keep the outline CCW, reflected placements
//! reverse the walk so every emitted polygon is CCW.
//!
//! # Metatile substitution (the construction actually generated)
//!
//! Hats cluster (Theorem 4.1 of the paper) into four metatile shapes:
//! an irregular hexagon **H** (4 hats), an equilateral triangle **T**
//! (1 hat), a parallelogram **P** (2 hats) and a pentagonal triskelion
//! leg **F** (2 hats). The cluster contents are the constant
//! [`CLUSTER_HATS`] (H1 is the only reflected hat); P and F carry the
//! same two hats and differ only in their boundaries. The metatiles
//! carry matching rules on their edges (labels A±, B±, X±, F±, L —
//! table [`BOUNDARY`], checked by a test), fit together edge to edge
//! without reflections, and obey a substitution system with inflation
//! factor **φ² ≈ 2.618** (areas by φ⁴).
//!
//! One substitution step builds a 29-tile patch from the four current
//! supertiles by edge alignment ([`RULES`], identical to Figure 2.6 of
//! the paper and to Kaplan's hatviz), then keeps 22 of them as the
//! contents of the next level's four supertiles ([`KEPT`]); the other
//! seven only position the kept ones. The new outlines follow from a
//! few key patch points and 120° hexagon corners; per the paper, none
//! of the iterated supertiles is similar to its metatile — the
//! outlines drift slightly in every generation (rational corners with
//! growing denominators), by design. The patch places the H supertile
//! at the identity, so the level-`n` H supertile's hats are a literal
//! SUBSET of the level-`n+1` hats: all levels live in one global frame
//! and the patches are nested — a sequence of finite patches growing
//! in radius without bound which, by the paper's Theorem 2.1 (via the
//! Extension Theorem), determines aperiodic tilings of the plane
//! (nesting asserted exactly by a test, matching the exact-arithmetic
//! Python battery).
//!
//! # Generation for a view
//!
//! Supertile outlines are built in floating point (their corners come
//! from line intersections with irregular rational coordinates — the
//! exact values need denominators like 3529), but every *tile
//! placement* the construction produces is provably an exact Eisenstein
//! integer pair; the builder snaps each placement to integers and
//! asserts (in debug builds) that the residual is < 0.25, so the
//! emitted hat positions are exact at every depth. Each level stores
//! only its constant-size kept-child table (10/1/5/6 placements of the
//! previous level's supertiles) plus the exact integer bounding box of
//! its hat blob; generation descends the H supertile depth-first with
//! blob-box pruning, so the work stays proportional to the metatiles
//! near the view, not to the whole supertile.
//!
//! One lineage alone cannot cover arbitrary views: the exact rim
//! analysis shows the level-`d` blob's *solid* interior stabilizes
//! while permanent interstitial fjords near the seed remain — in the
//! full plane tiling they are filled by *neighboring* supertile
//! hierarchies (the Extension Theorem guarantees they exist), which a
//! single lineage never materializes. The generator therefore seeds
//! the blob per view: the view center is quantized to a 96-unit grid
//! anchor in Eisenstein coordinates, and the whole blob is translated
//! by the anchor plus a fixed offset into the blob's solid interior —
//! up the open (1, −1) wedge at distance ~0.7·m, where m grows with
//! the view's hexagonal radius W (fallbacks along (1, 0) and further
//! out). Acceptance is exact: the hat area clipped to the view box
//! (triangulated Sutherland–Hodgman clipping) must equal the view
//! area, which catches both gaps and overlaps; the depth escalates
//! 3, 4, 5, 6 per candidate until some (seed, depth) covers the view.
//!
//! Views sharing the same anchor cell and radius bucket see one fixed
//! translated blob, and because the patches are nested across depths,
//! growing or zooming the view keeps every visible tile bit-identical
//! (a deeper acceptance only adds tiles). Crossing an anchor cell
//! (every ~96 world units of panning) or a radius bucket re-seeds the
//! patch — a different, equally legal finite piece of the plane
//! tiling.
//!
//! The tiling is monohedral: `proto` is always 0. `seed` is the hat's
//! cluster role + 1 (1..=9): H1..H4 (the four hats of an H metatile,
//! H1 being the reflected one), T1, P1..P2, F1..F2 — visible in
//! "pattern" mode.
//!
//! # Verification
//!
//! Exact stdlib-only Python battery (`/tmp/opencode/hat_arc/hat_verify.py`,
//! transcribed from the paper's ancillary `validate/` code and Kaplan's
//! hatviz, cross-checked against the einstein-tiling tables): cluster
//! internal census; hat counts 4, 25, 169, 1156, 7921, 54289 for the H
//! supertile; 0 matching-rule violations at every level; no hat
//! overlaps; rim = closed boundary walks; exact nesting of every level
//! in the next; per-view seeded coverage sweeps over 100 view boxes
//! (view half-widths up to 200 world units, nine centers). The patches
//! are NOT edge-to-edge: 565 directed hat sides of lattice length 2
//! carry a neighboring hat's vertex at their midpoint at depth 3. The
//! Rust tests re-assert the count tables, nesting, the no-overlap and
//! T-junction census, the matching rules on the integer grid, and the
//! exact per-view clipped-area acceptance.

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

/// √3/2 — the y scale of the Eisenstein-to-world map (world y of
/// lattice row 1). A unit hat has world area exactly 8√3 = 16·H.
const H: f64 = 0.8660254037844386;

/// Metatile type indices.
const TYPE_H: u8 = 0;
const TYPE_T: u8 = 1;
const TYPE_P: u8 = 2;
const TYPE_F: u8 = 3;

/// Depth cap for the seeded coverage search: the blob's solid interior
/// along its open wedge reaches ~φ^(2d)·8 Eisenstein units, so depth 6
/// covers view hexagonal radii up to ~850 world units; beyond that (or
/// if every candidate seed fails) the generator returns the zoom-in
/// message — the tile-budget precheck usually fires first.
const MAX_DEPTH: u32 = 6;

/// The hat's 13 vertices in Eisenstein coordinates `(a, b)` = `a + b·ω`,
/// anticlockwise, in units of the short kite edge.
const HAT: [[i64; 2]; 13] = [
    [0, 0],
    [-1, -1],
    [0, -2],
    [2, -2],
    [2, -1],
    [4, -2],
    [5, -1],
    [4, 0],
    [3, 0],
    [2, 2],
    [0, 3],
    [0, 2],
    [-1, 2],
];

/// Cluster contents: for each metatile type, its hats as
/// `(rotation k, reflected, translation (a, b))` in the metatile's
/// frame. H1 (the first hat of H) is the only reflected one; metatiles
/// themselves are never reflected. P and F carry the same two hats.
/// Unused trailing slots are zero-filled.
const CLUSTER_HATS: [[(u8, bool, i64, i64); 4]; 4] = [
    // H: H1 (reflected), H2, H3, H4
    [(4, true, 4, 4), (0, false, 0, 6), (0, false, 6, 0), (4, false, 2, 2)],
    // T: T1
    [(0, false, 0, 2), (0, false, 0, 0), (0, false, 0, 0), (0, false, 0, 0)],
    // P: P1, P2
    [(5, false, 2, 0), (0, false, 6, -2), (0, false, 0, 0), (0, false, 0, 0)],
    // F: F1, F2
    [(5, false, 2, 0), (0, false, 6, -2), (0, false, 0, 0), (0, false, 0, 0)],
];

/// Number of hats per metatile type.
const CLUSTER_HAT_COUNT: [usize; 4] = [4, 1, 2, 2];

/// Level-0 metatile outlines (corners in the order [`RULES`] refers to
/// them), padded with a repeat of corner 0 to 6 entries.
const OUTLINES0: [[[i64; 2]; 6]; 4] = [
    // H: irregular hexagon, 6 corners
    [[10, 0], [2, 8], [0, 8], [0, 0], [2, -2], [10, -2]],
    // T: equilateral triangle, 3 corners
    [[0, 0], [6, 0], [0, 6], [0, 0], [0, 0], [0, 0]],
    // P: parallelogram, 4 corners
    [[4, -4], [12, -4], [8, 0], [0, 0], [0, 0], [0, 0]],
    // F: pentagon, 5 corners
    [[4, -4], [10, -4], [10, -2], [8, 0], [0, 0], [0, 0]],
];

/// Corner counts of the level-0 outlines.
const OUTLINE_LEN0: [usize; 4] = [6, 3, 4, 5];

/// One substitution rule (Figure 2.6 of the paper).
#[derive(Clone, Copy)]
enum Rule {
    /// Start the patch with that supertile at the identity.
    Seed(u8),
    /// Place `type` with its outline edge `f` matched (isometrically,
    /// direction-reversed) against edge `e` of patch child `i`.
    Edge(usize, usize, u8, usize),
    /// Match edge `f` against the segment from corner `a` of child `i`
    /// to corner `b` of child `j` (a gap between two placed tiles).
    Gap(usize, usize, usize, usize, u8, usize),
}

/// The 29 substitution rules.
const RULES: [Rule; 29] = [
    Rule::Seed(TYPE_H),
    Rule::Edge(0, 0, TYPE_P, 2),
    Rule::Edge(1, 0, TYPE_H, 2),
    Rule::Edge(2, 0, TYPE_P, 2),
    Rule::Edge(3, 0, TYPE_H, 2),
    Rule::Edge(4, 4, TYPE_P, 2),
    Rule::Edge(0, 4, TYPE_F, 3),
    Rule::Edge(2, 4, TYPE_F, 3),
    Rule::Gap(4, 1, 3, 2, TYPE_F, 0),
    Rule::Edge(8, 3, TYPE_H, 0),
    Rule::Edge(9, 2, TYPE_P, 0),
    Rule::Edge(10, 2, TYPE_H, 0),
    Rule::Edge(11, 4, TYPE_P, 2),
    Rule::Edge(12, 0, TYPE_H, 2),
    Rule::Edge(13, 0, TYPE_F, 3),
    Rule::Edge(14, 2, TYPE_F, 1),
    Rule::Edge(15, 3, TYPE_H, 4),
    Rule::Edge(8, 2, TYPE_F, 1),
    Rule::Edge(17, 3, TYPE_H, 0),
    Rule::Edge(18, 2, TYPE_P, 0),
    Rule::Edge(19, 2, TYPE_H, 2),
    Rule::Edge(20, 4, TYPE_F, 3),
    Rule::Edge(20, 0, TYPE_P, 2),
    Rule::Edge(22, 0, TYPE_H, 2),
    Rule::Edge(23, 4, TYPE_F, 3),
    Rule::Edge(23, 0, TYPE_F, 3),
    Rule::Edge(16, 0, TYPE_P, 2),
    Rule::Gap(9, 4, 0, 2, TYPE_T, 2),
    Rule::Edge(4, 0, TYPE_F, 3),
];

/// The patch children kept as the contents of each next-level
/// supertile (indices into the 29-tile patch), padded with zeros.
const KEPT: [[usize; 10]; 4] = [
    // H: 10 children
    [0, 9, 16, 27, 26, 6, 1, 8, 10, 15],
    // T: 1 child
    [11, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    // P: 5 children
    [7, 2, 3, 4, 28, 0, 0, 0, 0, 0],
    // F: 6 children
    [21, 20, 22, 23, 24, 25, 0, 0, 0, 0],
];

/// Kept-children count per supertile type.
const KEPT_LEN: [usize; 4] = [10, 1, 5, 6];

/// Matching-rule boundary labels per metatile type: each entry is a
/// `(marked point, label code, segment lattice length)` triple; the
/// labelled segment leaves the marked point anticlockwise (towards the
/// next entry's marked point). Label codes: 0 A+, 1 A−, 2 B+, 3 B−,
/// 4 X+, 5 X−, 6 F+, 7 F−, 8 L. Test-only (see the matching-rule test).
#[cfg(test)]
const BOUNDARY: [([i64; 2], u8, i64); 29] = [
    // T (3 segments)
    ([0, 0], 1, 6),
    ([6, 0], 1, 6),
    ([0, 6], 2, 6),
    // H (9 segments)
    ([0, 0], 4, 2),
    ([2, -2], 3, 6),
    ([8, -2], 5, 2),
    ([10, -2], 4, 2),
    ([10, 0], 3, 6),
    ([4, 6], 5, 2),
    ([2, 8], 4, 2),
    ([0, 8], 0, 6),
    ([0, 2], 5, 2),
    // P (8 segments)
    ([0, 0], 8, 2),
    ([2, -2], 5, 2),
    ([4, -4], 4, 2),
    ([6, -4], 1, 6),
    ([12, -4], 8, 2),
    ([10, -2], 5, 2),
    ([8, 0], 4, 2),
    ([6, 0], 2, 6),
    // F (9 segments)
    ([0, 0], 8, 2),
    ([2, -2], 5, 2),
    ([4, -4], 4, 2),
    ([6, -4], 8, 2),
    ([8, -4], 5, 2),
    ([10, -4], 6, 2),
    ([10, -2], 7, 2),
    ([8, 0], 4, 2),
    ([6, 0], 2, 6),
];

/// Boundary segment ranges per type into [`BOUNDARY`] (start, count),
/// indexed by metatile type.
#[cfg(test)]
const BOUNDARY_RANGE: [(usize, usize); 4] = [(3, 9), (0, 3), (12, 8), (20, 9)];

/// Complement label of each label code (the matching rule).
#[cfg(test)]
fn complement_label(label: u8) -> u8 {
    match label {
        0 => 1,
        1 => 0,
        2 => 3,
        3 => 2,
        4 => 5,
        5 => 4,
        6 => 7,
        7 => 6,
        _ => 8,
    }
}

/// Rotate an Eisenstein point by k·60° anticlockwise (multiply by ω^k:
/// `(a, b) -> (−b, a+b)`).
fn rot(p: [i64; 2], k: u8) -> [i64; 2] {
    let (mut a, mut b) = (p[0], p[1]);
    for _ in 0..(k % 6) {
        let (na, nb) = (-b, a + b);
        a = na;
        b = nb;
    }
    [a, b]
}

/// Reflect an Eisenstein point in the x-axis (conjugate:
/// `(a, b) -> (a+b, −b)`).
fn refl(p: [i64; 2]) -> [i64; 2] {
    [p[0] + p[1], -p[1]]
}

/// Apply a hat placement `(k, reflected, t)` to a point.
fn place_pt(p: [i64; 2], k: u8, reflected: bool, t: [i64; 2]) -> [i64; 2] {
    let q = if reflected { refl(p) } else { p };
    let r = rot(q, k);
    [r[0] + t[0], r[1] + t[1]]
}

/// World position of an Eisenstein point.
fn world(p: [i64; 2]) -> Vec2 {
    Vec2::new(p[0] as f64 + 0.5 * p[1] as f64, H * p[1] as f64)
}

/// The hat's 13 vertices under a placement, anticlockwise (a reflected
/// placement reverses the walk, keeping vertex 0 first, so every
/// emitted polygon is CCW).
fn hat_vertices(k: u8, reflected: bool, t: [i64; 2]) -> [[i64; 2]; 13] {
    let mut pts = [[0i64; 2]; 13];
    for (i, v) in HAT.iter().enumerate() {
        pts[i] = place_pt(*v, k, reflected, t);
    }
    if reflected {
        let first = pts[0];
        for i in 0..12 {
            pts[1 + i] = place_pt(HAT[12 - i], k, reflected, t);
        }
        pts[0] = first;
    }
    pts
}

/// Compose a supertile placement `(k, t)` with a child placement
/// `(k2, t2)` given in the supertile's frame: metatile placements are
/// never reflected, so only rotations compose.
fn compose_meta(k: u8, t: [i64; 2], k2: u8, t2: [i64; 2]) -> (u8, [i64; 2]) {
    let rt = rot(t2, k);
    ((k + k2) % 6, [t[0] + rt[0], t[1] + rt[1]])
}

/// Rotate an `(a, b)` pair by k·60° in floating point.
fn rot_f(p: (f64, f64), k: u8) -> (f64, f64) {
    let (mut a, mut b) = p;
    for _ in 0..(k % 6) {
        let (na, nb) = (-b, a + b);
        a = na;
        b = nb;
    }
    (a, b)
}

fn corner_f(outline: &[(f64, f64)], c: usize) -> (f64, f64) {
    outline[c % outline.len()]
}

/// Line intersection in floating point (as in the reference
/// construction of the new supertile outlines).
fn intersect_f(p1: (f64, f64), q1: (f64, f64), p2: (f64, f64), q2: (f64, f64)) -> (f64, f64) {
    let den = (p1.0 - q1.0) * (p2.1 - q2.1) - (p1.1 - q1.1) * (p2.0 - q2.0);
    let u = ((p1.0 - p2.0) * (p2.1 - q2.1) - (p1.1 - p2.1) * (p2.0 - q2.0)) / den;
    (p1.0 + u * (q1.0 - p1.0), p1.1 + u * (q1.1 - p1.1))
}

/// Find the rotation k (0..6) and translation putting the outline's
/// edge `f` (direction-reversed) onto the segment p->q, as the
/// substitution rules require. Returns None if no rotation matches
/// (must not happen for the verified rule set).
fn align_edge(
    outline: &[(f64, f64)],
    f: usize,
    p: (f64, f64),
    q: (f64, f64),
) -> Option<(u8, (f64, f64))> {
    let n = outline.len();
    let start = outline[f];
    let end = outline[(f + 1) % n];
    let target = (q.0 - p.0, q.1 - p.1);
    for k in 0..6u8 {
        let e = rot_f((end.0 - start.0, end.1 - start.1), k);
        if (e.0 - target.0).abs() < 1e-3 && (e.1 - target.1).abs() < 1e-3 {
            let s = rot_f(start, k);
            return Some((k, (p.0 - s.0, p.1 - s.1)));
        }
    }
    None
}

/// Snap a floating-point placement translation to integers, asserting
/// (in debug builds) the construction's exactness: the true translation
/// is provably an integer pair at every depth.
fn snap_t(t: (f64, f64)) -> [i64; 2] {
    let a = t.0.round();
    let b = t.1.round();
    debug_assert!(
        (t.0 - a).abs() < 0.25 && (t.1 - b).abs() < 0.25,
        "hat substitution placement is not integral: ({}, {})",
        t.0,
        t.1
    );
    [a as i64, b as i64]
}

/// One level of the supertile hierarchy: per type, the outline corners
/// (floating point — see the module docs) and the kept children as
/// `(child type, rotation, translation)` referencing the PREVIOUS
/// level's supertiles, with EXACT integer translations. `blob` is the
/// exact (a, b) bounding box `(a0, b0, a1, b1)` of the supertile's hat
/// vertices.
struct Level {
    outlines: [Vec<(f64, f64)>; 4],
    children: [Vec<(u8, u8, [i64; 2])>; 4],
    blob: [(i64, i64, i64, i64); 4],
}

/// Transform a blob box `(a0, b0, a1, b1)` by the placement `(k, t)`
/// (metatile placements never reflect, so transforming the four box
/// corners is exact).
fn blob_box(b: (i64, i64, i64, i64), k: u8, t: [i64; 2]) -> (i64, i64, i64, i64) {
    let mut a0 = i64::MAX;
    let mut b0 = i64::MAX;
    let mut a1 = i64::MIN;
    let mut b1 = i64::MIN;
    for &c in &[[b.0, b.1], [b.2, b.1], [b.0, b.3], [b.2, b.3]] {
        let p = place_pt(c, k, false, t);
        a0 = a0.min(p[0]);
        b0 = b0.min(p[1]);
        a1 = a1.max(p[0]);
        b1 = b1.max(p[1]);
    }
    (a0, b0, a1, b1)
}

/// Build the supertile hierarchy for levels 0..=depth.
fn build_levels(depth: u32) -> Vec<Level> {
    let mut levels = Vec::with_capacity(depth as usize + 1);

    // level 0: the four metatiles themselves
    {
        let mut outlines: [Vec<(f64, f64)>; 4] = Default::default();
        let mut children: [Vec<(u8, u8, [i64; 2])>; 4] = Default::default();
        let mut blob = [(0i64, 0i64, 0i64, 0i64); 4];
        for ty in 0..4usize {
            outlines[ty] = OUTLINES0[ty]
                .iter()
                .take(OUTLINE_LEN0[ty])
                .map(|&p| (p[0] as f64, p[1] as f64))
                .collect();
            children[ty] = vec![(ty as u8, 0u8, [0i64, 0])];
            let (mut a0, mut b0, mut a1, mut b1) = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);
            for &(k, r, ha, hb) in CLUSTER_HATS[ty].iter().take(CLUSTER_HAT_COUNT[ty]) {
                for v in HAT.iter() {
                    let p = place_pt(*v, k, r, [ha, hb]);
                    a0 = a0.min(p[0]);
                    b0 = b0.min(p[1]);
                    a1 = a1.max(p[0]);
                    b1 = b1.max(p[1]);
                }
            }
            blob[ty] = (a0, b0, a1, b1);
        }
        levels.push(Level {
            outlines,
            children,
            blob,
        });
    }

    for _ in 0..depth {
        let prev = levels.last().unwrap();

        // fit the 29-tile patch: placements are snapped to exact
        // integers (see the module docs)
        let mut patch_k: Vec<u8> = Vec::with_capacity(29);
        let mut patch_t: Vec<[i64; 2]> = Vec::with_capacity(29);
        let mut patch_ty: Vec<u8> = Vec::with_capacity(29);
        for rule in &RULES {
            let (k, t) = match *rule {
                Rule::Seed(_) => (0u8, [0i64, 0]),
                Rule::Edge(i, e, ty, f) => {
                    let outline = &prev.outlines[patch_ty[i] as usize];
                    // the new tile's edge f is matched direction-reversed
                    // against edge e of child i: p = corner e+1, q = corner e
                    let ck = patch_k[i];
                    let ct = patch_t[i];
                    let cp = rot_f(corner_f(outline, e + 1), ck);
                    let cq = rot_f(corner_f(outline, e), ck);
                    let p = (cp.0 + ct[0] as f64, cp.1 + ct[1] as f64);
                    let q = (cq.0 + ct[0] as f64, cq.1 + ct[1] as f64);
                    let (k2, t2) = align_edge(&prev.outlines[ty as usize], f, p, q)
                        .expect("substitution edge rule must align");
                    (k2, snap_t(t2))
                }
                Rule::Gap(i, a, j, b, ty, f) => {
                    let o1 = &prev.outlines[patch_ty[i] as usize];
                    let o2 = &prev.outlines[patch_ty[j] as usize];
                    let ca = rot_f(corner_f(o1, a), patch_k[i]);
                    let cb = rot_f(corner_f(o2, b), patch_k[j]);
                    // p = corner b of child j, q = corner a of child i
                    let p = (cb.0 + patch_t[j][0] as f64, cb.1 + patch_t[j][1] as f64);
                    let q = (ca.0 + patch_t[i][0] as f64, ca.1 + patch_t[i][1] as f64);
                    let (k2, t2) = align_edge(&prev.outlines[ty as usize], f, p, q)
                        .expect("substitution gap rule must align");
                    (k2, snap_t(t2))
                }
            };
            let ty = match *rule {
                Rule::Seed(ty) | Rule::Edge(_, _, ty, _) | Rule::Gap(_, _, _, _, ty, _) => ty,
            };
            patch_k.push(k);
            patch_t.push(t);
            patch_ty.push(ty);
        }

        // kept children (one level of references) + blob boxes
        let mut children: [Vec<(u8, u8, [i64; 2])>; 4] = Default::default();
        let mut blob = [(0i64, 0i64, 0i64, 0i64); 4];
        for ty in 0..4usize {
            let mut v = Vec::with_capacity(KEPT_LEN[ty]);
            let (mut a0, mut b0, mut a1, mut b1) = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);
            for &idx in KEPT[ty].iter().take(KEPT_LEN[ty]) {
                let entry = (patch_ty[idx], patch_k[idx], patch_t[idx]);
                let bb = blob_box(prev.blob[entry.0 as usize], entry.1, entry.2);
                a0 = a0.min(bb.0);
                b0 = b0.min(bb.1);
                a1 = a1.max(bb.2);
                b1 = b1.max(bb.3);
                v.push(entry);
            }
            children[ty] = v;
            blob[ty] = (a0, b0, a1, b1);
        }

        // new outlines: key patch points + 120° hexagon corners
        let cp = |idx: usize, c: usize| -> (f64, f64) {
            let w = rot_f(
                corner_f(&prev.outlines[patch_ty[idx] as usize], c),
                patch_k[idx],
            );
            (w.0 + patch_t[idx][0] as f64, w.1 + patch_t[idx][1] as f64)
        };
        let mut outlines: [Vec<(f64, f64)>; 4] = Default::default();
        let b1p = cp(8, 2);
        let b2p = cp(21, 2);
        let p7 = cp(7, 2);
        let p25 = cp(25, 2);
        let f6 = cp(6, 2);
        let d4 = rot_f((b2p.0 - b1p.0, b2p.1 - b1p.1), 4);
        let rb2 = (b1p.0 + d4.0, b1p.1 + d4.1);
        let low = intersect_f(b1p, rb2, f6, p7);
        let w = rot_f((f6.0 - low.0, f6.1 - low.1), 5);
        let h3 = cp(14, 2);
        let v = rot_f(w, 5);
        outlines[TYPE_H as usize] = vec![
            low,
            b1p,
            (b1p.0 + w.0, b1p.1 + w.1),
            h3,
            (h3.0 - v.0, h3.1 - v.1),
            f6,
        ];
        outlines[TYPE_P as usize] = vec![
            p7,
            (p7.0 + (b1p.0 - low.0), p7.1 + (b1p.1 - low.1)),
            b1p,
            low,
        ];
        let f24 = cp(24, 2);
        let f25_0 = cp(25, 0);
        outlines[TYPE_F as usize] = vec![
            b2p,
            f24,
            f25_0,
            p25,
            (p25.0 + (low.0 - b1p.0), p25.1 + (low.1 - b1p.1)),
        ];
        // T: t_start = h[1] + (h[4] - h[5]); the apex is h[2], the third
        // corner completes the equilateral triangle
        let h = &outlines[TYPE_H as usize];
        let t_start = (h[1].0 + (h[4].0 - h[5].0), h[1].1 + (h[4].1 - h[5].1));
        let td = rot_f((h[2].0 - t_start.0, h[2].1 - t_start.1), 5);
        outlines[TYPE_T as usize] = vec![t_start, (t_start.0 + td.0, t_start.1 + td.1), h[2]];

        levels.push(Level {
            outlines,
            children,
            blob,
        });
    }
    levels
}

/// Descend the depth-`depth` supertile `root` (indices 0..=depth into
/// `levels`), collecting the placed level-0 metatiles as
/// `(type, rotation, translation)` with exact integer placements.
/// When `prune` is given, subtrees whose blob box misses the
/// `(a0, b0, a1, b1)` region are skipped (sound: a supertile's hats all
/// lie inside its blob box).
fn collect_metas(
    levels: &[Level],
    depth: usize,
    root: usize,
    prune: Option<(i64, i64, i64, i64)>,
    out: &mut Vec<(u8, u8, [i64; 2])>,
) {
    let mut stack = vec![(depth, root, 0u8, [0i64, 0])];
    while let Some((li, ty, k, t)) = stack.pop() {
        let bb = blob_box(levels[li].blob[ty], k, t);
        if let Some(region) = prune {
            if bb.2 < region.0 || bb.0 > region.2 || bb.3 < region.1 || bb.1 > region.3 {
                continue;
            }
        }
        if li == 0 {
            out.push((ty as u8, k, t));
            continue;
        }
        for &(cty, ck, ct) in &levels[li].children[ty] {
            let (k3, t3) = compose_meta(k, t, ck, ct);
            stack.push((li - 1, cty as usize, k3, t3));
        }
    }
}

/// A placed hat: rotation `k`, reflection flag, exact integer
/// translation, and the pattern seed (cluster role + 1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Hat {
    k: u8,
    refl: bool,
    t: [i64; 2],
    role: u32,
}

/// Pattern seed of hat `hat_idx` (0-based) of metatile type `ty`: the
/// roles H1..H4, T1, P1..P2, F1..F2 map to seeds 1..=9.
fn hat_seed(ty: u8, hat_idx: usize) -> u32 {
    let base = [0u32, 4, 5, 7][ty as usize];
    base + hat_idx as u32 + 1
}

/// Demote placed level-0 metatiles to their hats.
fn demote(metas: &[(u8, u8, [i64; 2])]) -> Vec<Hat> {
    let mut out = Vec::new();
    for &(ty, k, t) in metas {
        for (hi, &(hk, hr, ha, hb)) in CLUSTER_HATS[ty as usize]
            .iter()
            .take(CLUSTER_HAT_COUNT[ty as usize])
            .enumerate()
        {
            let (k2, t2) = compose_meta(k, t, hk, [ha, hb]);
            out.push(Hat {
                k: k2,
                refl: hr,
                t: t2,
                role: hat_seed(ty, hi),
            });
        }
    }
    out
}

impl Hat {
    fn vertices(&self) -> [[i64; 2]; 13] {
        hat_vertices(self.k, self.refl, self.t)
    }

    fn world_pts(&self) -> [Vec2; 13] {
        let mut pts = [Vec2::ZERO; 13];
        for (i, p) in self.vertices().iter().enumerate() {
            pts[i] = world(*p);
        }
        pts
    }

    fn aabb(&self) -> BBox {
        let pts = self.world_pts();
        let mut min = pts[0];
        let mut max = pts[0];
        for p in &pts[1..] {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
        }
        BBox::new(min, max)
    }

    /// Area of the hat's intersection with `region` (tiling space).
    /// The hat is triangulated and each (convex) triangle is clipped
    /// with Sutherland–Hodgman — exact for convex subjects, and the
    /// triangles partition the hat, so the sum is the true clipped
    /// area. Interior hats short-circuit to the exact area 8√3.
    fn clipped_area(&self, region: &BBox) -> f64 {
        let bb = self.aabb();
        if !bb.intersects(*region) {
            return 0.0;
        }
        if region.min.x <= bb.min.x
            && region.min.y <= bb.min.y
            && region.max.x >= bb.max.x
            && region.max.y >= bb.max.y
        {
            return 16.0 * H;
        }
        let pts = self.world_pts();
        let poly = Polygon::from_slice(pts.as_ref());
        let mut total = 0.0;
        for tri in poly.triangulate() {
            let tri_poly = Polygon::from_slice(&[
                pts[tri[0] as usize],
                pts[tri[1] as usize],
                pts[tri[2] as usize],
            ]);
            if let Some(c) = tri_poly.clipped_to(region) {
                total += c.area();
            }
        }
        total
    }
}

/// Hat tiling (aperiodic monotile; metatile-substitution generator).
pub struct HatTiling;

impl Tiling for HatTiling {
    fn id(&self) -> &'static str {
        "hat"
    }

    fn name(&self) -> &'static str {
        "Hat (aperiodic monotile)"
    }

    fn description(&self) -> &'static str {
        "Aperiodic monotile (einstein) by H/T/P/F metatile substitution; inflation φ²; tiles with its mirror image in 12 orientations."
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None).with_notes(
            "aperiodic; einstein; monotile; tiles with its mirror image; non-edge-to-edge",
        );
        let fail = |message: &'static str| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: None,
            message: Some(message),
        };

        let map = similarity_map(params);
        let inv = map.inverse();

        // Region in the tiling's own (unit-scale world) coordinates: the
        // axis-aligned box around the inverse image of the four world
        // corners.
        let mut rmin = Vec2::new(f64::MAX, f64::MAX);
        let mut rmax = Vec2::new(f64::MIN, f64::MIN);
        for &corner in &query.region.corners() {
            let q = inv.transform(corner);
            rmin.x = rmin.x.min(q.x);
            rmin.y = rmin.y.min(q.y);
            rmax.x = rmax.x.max(q.x);
            rmax.y = rmax.y.max(q.y);
        }
        let region = BBox::new(rmin, rmax);
        if region.area() <= 1e-12 {
            return fail("empty view");
        }
        // Final hats have area exactly 8√3 = 16H in tiling space (the
        // construction always places unit hats), so this bounds the tile
        // count before any substitution work.
        if region.area() > MAX_TILES as f64 * 16.0 * H {
            return fail("too many tiles for this view — zoom in");
        }

        // (a, b) geometry of the region (the world->Eisenstein map is
        // linear: b = y/H, a = x − b/2). The padded box drives blob
        // pruning (hat vertices span 6 in a and 5 in b); the box center
        // and hexagonal radius drive the seeded placement below.
        let mut corners = [(0.0f64, 0.0f64); 4];
        let (mut a0, mut b0, mut a1, mut b1) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
        for (i, corner) in region.corners().iter().enumerate() {
            let b = corner.y / H;
            let a = corner.x - 0.5 * b;
            corners[i] = (a, b);
            a0 = a0.min(a);
            b0 = b0.min(b);
            a1 = a1.max(a);
            b1 = b1.max(b);
        }
        let region_ab = (
            (a0 - 6.0).floor() as i64,
            (b0 - 6.0).floor() as i64,
            (a1 + 6.0).ceil() as i64,
            (b1 + 6.0).ceil() as i64,
        );
        let (ca, cb) = ((a0 + a1) / 2.0, (b0 + b1) / 2.0);
        let mut radius = 0.0f64;
        for &(a, b) in &corners {
            let (da, db) = (a - ca, b - cb);
            radius = radius.max(da.abs()).max(db.abs()).max((da + db).abs());
        }

        // Seed the lineage blob into the view (see the module docs): the
        // view center is quantized to a 96-unit grid anchor, and the blob
        // is translated by the anchor plus a fixed offset up its open
        // (1, −1) wedge, distance growing with the hexagonal view radius;
        // fallback offsets run along (1, 0) and further out. Per seed the
        // depth escalates 3..=MAX_DEPTH; acceptance is exact clipped-area
        // accounting (it catches both gaps and overlaps).
        let anchor = [
            ((ca / 96.0).round() * 96.0) as i64,
            ((cb / 96.0).round() * 96.0) as i64,
        ];
        let bucket = (radius / 32.0).ceil() as i64;
        let m = (bucket * 32 + 96) / 10 * 10;
        let candidates = [
            (7 * m / 10, -7 * m / 10),
            (m, 0),
            (7 * m / 5, -7 * m / 5),
            (2 * m, 0),
            (14 * m / 5, -14 * m / 5),
        ];

        let want = region.area();
        let mut found: Option<Vec<Hat>> = None;
        'search: for &(da, db) in &candidates {
            let seed = [anchor[0] + da, anchor[1] + db];
            // prune window in the blob's own frame
            let prune = (
                region_ab.0 - seed[0],
                region_ab.1 - seed[1],
                region_ab.2 - seed[0],
                region_ab.3 - seed[1],
            );
            for depth in 3..=MAX_DEPTH {
                let levels = build_levels(depth);
                let mut metas = Vec::new();
                collect_metas(&levels, depth as usize, TYPE_H as usize, Some(prune), &mut metas);
                let mut hats = demote(&metas);
                for h in &mut hats {
                    h.t[0] += seed[0];
                    h.t[1] += seed[1];
                }
                hats.retain(|h| h.aabb().intersects(region));
                let mut covered = 0.0;
                for h in &hats {
                    covered += h.clipped_area(&region);
                }
                if covered >= want * (1.0 - 1e-9) {
                    found = Some(hats);
                    break 'search;
                }
            }
        }
        let hats = match found {
            Some(h) => h,
            None => return fail("too many tiles for this view — zoom in"),
        };

        // Emit. Monohedral: `proto` is always 0; `seed` is the cluster
        // role + 1 (1..=9) — see the module docs.
        let mut tiles = Vec::with_capacity(hats.len());
        for h in &hats {
            let poly = Polygon::from_slice(h.world_pts().as_ref()).transform(&map);
            if poly.aabb().intersects(query.region) {
                tiles.push(Tile {
                    proto: 0,
                    seed: h.role,
                    poly,
                });
            }
        }

        if tiles.len() > MAX_TILES {
            return fail("too many tiles for this view — zoom in");
        }

        TilingResult {
            tiles,
            symmetry: meta,
            periodic: None,
            message: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// The hat outline: 13 sides with Eisenstein lattice lengths
    /// 2, 1, 2, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2 (world lengths √3, 1, 2,
    /// 1, √3, √3, 1, 1, √3, √3, 1, 1, √3 — the length-2 lattice side is
    /// two collinear short kite edges), 20 unit lattice steps around,
    /// CCW with exact area 8√3 = 16H, for every orientation.
    #[test]
    fn hat_outline_geometry() {
        for k in 0..6u8 {
            for &reflected in &[false, true] {
                let pts = hat_vertices(k, reflected, [3, -2]);
                let world_pts: Vec<Vec2> = pts.iter().map(|&p| world(p)).collect();
                let mut a2 = 0.0;
                for i in 0..13 {
                    let p = world_pts[i];
                    let q = world_pts[(i + 1) % 13];
                    a2 += p.x * q.y - q.x * p.y;
                }
                assert!(a2 > 0.0, "k={k} refl={reflected}: not CCW");
                // the raw shoelace sum is the doubled area: 2 × 8√3 = 32H
                assert!(
                    (a2 - 32.0 * H).abs() < 1e-9,
                    "k={k} refl={reflected}: 2×area {a2} != {}",
                    32.0 * H
                );
                let lens: Vec<i64> = (0..13)
                    .map(|i| {
                        let d = (
                            pts[(i + 1) % 13][0] - pts[i][0],
                            pts[(i + 1) % 13][1] - pts[i][1],
                        );
                        d.0.abs().max(d.1.abs()).max((d.0 + d.1).abs())
                    })
                    .collect();
                // a reflected placement reverses the vertex walk, so its
                // side-length sequence is the reversed CCW one
                let base = [2i64, 1, 2, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2];
                let want: Vec<i64> = if reflected {
                    base.iter().rev().copied().collect()
                } else {
                    base.to_vec()
                };
                assert_eq!(
                    lens, want,
                    "k={k} refl={reflected}: side lattice lengths"
                );
                assert_eq!(lens.iter().sum::<i64>(), 20, "unit steps around");
            }
        }
    }

    /// Cluster sanity: the hats inside each metatile do not overlap
    /// (no directed side appears twice) — the exact level-0 data.
    #[test]
    fn clusters_have_no_internal_overlap() {
        for ty in 0..4usize {
            let mut edges = BTreeSet::new();
            for &(k, r, ha, hb) in CLUSTER_HATS[ty].iter().take(CLUSTER_HAT_COUNT[ty]) {
                let pts = hat_vertices(k, r, [ha, hb]);
                for i in 0..13 {
                    let (p, q) = (pts[i], pts[(i + 1) % 13]);
                    assert!(
                        edges.insert((p, q)),
                        "cluster {ty}: duplicate directed side ({p:?}, {q:?})"
                    );
                }
            }
        }
    }

    /// Flattened level-0 metatile and hat counts per supertile type and
    /// level, matching the exact-arithmetic Python battery (and the
    /// sequences documented by hat-amp), with all placements distinct.
    #[test]
    fn substitution_counts_match_reference() {
        const META_COUNTS: [[usize; 4]; 5] = [
            [1, 1, 1, 1],
            [10, 1, 5, 6],
            [64, 10, 37, 43],
            [442, 64, 251, 294],
            [3025, 442, 1723, 2017],
        ];
        const HAT_COUNTS: [[usize; 4]; 5] = [
            [4, 1, 2, 2],
            [25, 4, 14, 16],
            [169, 25, 96, 112],
            [1156, 169, 658, 770],
            [7921, 1156, 4510, 5280],
        ];
        let levels = build_levels(4);
        for d in 0..5usize {
            for ty in 0..4usize {
                let mut metas = Vec::new();
                collect_metas(&levels, d, ty, None, &mut metas);
                assert_eq!(
                    metas.len(),
                    META_COUNTS[d][ty],
                    "level {d} type {ty}: metatile count"
                );
                let hats = demote(&metas);
                assert_eq!(
                    hats.len(),
                    HAT_COUNTS[d][ty],
                    "level {d} type {ty}: hat count"
                );
                let set: BTreeSet<_> = hats.iter().map(|h| (h.k, h.refl, h.t)).collect();
                assert_eq!(set.len(), hats.len(), "level {d} type {ty}: duplicate hats");
            }
        }
    }

    /// Exact nesting: the hats of the depth-d H supertile are a subset
    /// of the depth-(d+1) hats — all levels live in one global frame,
    /// so deeper views only ever add tiles.
    #[test]
    fn hats_are_nested_across_depths() {
        let levels = build_levels(3);
        let hat_set = |d: usize| {
            let mut metas = Vec::new();
            collect_metas(&levels, d, TYPE_H as usize, None, &mut metas);
            demote(&metas)
                .into_iter()
                .map(|h| (h.k, h.refl, h.t))
                .collect::<BTreeSet<_>>()
        };
        let s1 = hat_set(1);
        let s2 = hat_set(2);
        let s3 = hat_set(3);
        assert_eq!(s1.len(), 25);
        assert_eq!(s2.len(), 169);
        assert_eq!(s3.len(), 1156);
        assert!(s1.is_subset(&s2), "depth-1 hats missing at depth 2");
        assert!(s2.is_subset(&s3), "depth-2 hats missing at depth 3");
    }

    /// The depth-3 H patch (1156 hats) is a perfect tiling of its blob:
    /// no two hats share a directed side (no overlap). It is NOT
    /// edge-to-edge: hat sides of lattice length 2 are two collinear
    /// unit kite edges, and their midpoints carry hat vertices of
    /// adjacent hats — exactly 565 directed-side midpoint incidences at
    /// depth 3, matching the exact-arithmetic Python census.
    #[test]
    fn hat_patch_has_no_overlaps_and_midpoint_tjuncs() {
        let levels = build_levels(3);
        let mut metas = Vec::new();
        collect_metas(&levels, 3, TYPE_H as usize, None, &mut metas);
        assert_eq!(metas.len(), 442);
        let hats = demote(&metas);
        assert_eq!(hats.len(), 1156);
        let mut edges = BTreeSet::new();
        let mut vset = BTreeSet::new();
        for h in &hats {
            let pts = h.vertices();
            for v in pts.iter() {
                vset.insert(*v);
            }
            for i in 0..13 {
                let (p, q) = (pts[i], pts[(i + 1) % 13]);
                assert!(
                    edges.insert((p, q)),
                    "duplicate directed side ({p:?}, {q:?})"
                );
            }
        }
        let mut tjunc = 0usize;
        for &(p, q) in &edges {
            let d = (q[0] - p[0], q[1] - p[1]);
            if d.0 % 2 == 0 && d.1 % 2 == 0 {
                let mid = [p[0] + d.0 / 2, p[1] + d.1 / 2];
                if vset.contains(&mid) {
                    tjunc += 1;
                }
            }
        }
        assert_eq!(
            tjunc, 565,
            "midpoint T-junction incidences in the depth-3 patch"
        );
    }

    /// The matching rules on metatile edges (labels A±, B±, X±, F±, L)
    /// are honored across the depth-2 flattened H patch: no edge
    /// overlap, and every shared sub-edge carries complementary labels
    /// with reversed sub-index.
    #[test]
    fn metatile_matching_rules_honored() {
        use std::collections::BTreeMap;
        let levels = build_levels(2);
        let mut metas = Vec::new();
        collect_metas(&levels, 2, TYPE_H as usize, None, &mut metas);
        assert_eq!(metas.len(), 64);
        type EdgeKey = ([i64; 2], [i64; 2]);
        let mut edges: BTreeMap<EdgeKey, (u8, usize, usize)> = BTreeMap::new();
        for &(name, k, t) in &metas {
            let range = BOUNDARY_RANGE[name as usize];
            let segs = &BOUNDARY[range.0..range.0 + range.1];
            for i in 0..segs.len() {
                let (start, label, len) = segs[i];
                let next = segs[(i + 1) % segs.len()].0;
                let sp = place_pt(start, k, false, t);
                let ep = place_pt(next, k, false, t);
                let d = (ep[0] - sp[0], ep[1] - sp[1]);
                let total = d.0.abs().max(d.1.abs()).max((d.0 + d.1).abs());
                assert_eq!(total % len, 0, "labelled segment length {total} vs {len}");
                let n = (total / len) as usize;
                for s in 0..n {
                    let a = [
                        sp[0] + d.0 * s as i64 / n as i64,
                        sp[1] + d.1 * s as i64 / n as i64,
                    ];
                    let b = [
                        sp[0] + d.0 * (s as i64 + 1) / n as i64,
                        sp[1] + d.1 * (s as i64 + 1) / n as i64,
                    ];
                    assert!(
                        edges.insert((a, b), (label, s, n)).is_none(),
                        "metatile edge overlap at ({a:?}, {b:?})"
                    );
                }
            }
        }
        for ((a, b), &(label, s, n)) in &edges {
            if let Some(&(label2, s2, n2)) = edges.get(&(*b, *a)) {
                assert_eq!(n2, n, "sub-edge split mismatch at ({a:?}, {b:?})");
                assert_eq!(
                    label2,
                    complement_label(label),
                    "label mismatch at ({a:?}, {b:?})"
                );
                assert_eq!(s2, n - 1 - s, "sub-index mismatch at ({a:?}, {b:?})");
            }
        }
    }

    /// H1 — the only reflected hat — really occurs (the hat tiles only
    /// together with its mirror image), and all nine cluster roles are
    /// visible in a deep patch.
    #[test]
    fn reflected_hats_and_roles_occur() {
        let levels = build_levels(4);
        let mut metas = Vec::new();
        collect_metas(&levels, 4, TYPE_H as usize, None, &mut metas);
        let hats = demote(&metas);
        assert!(hats.iter().any(|h| h.refl), "no reflected hat");
        let roles: BTreeSet<u32> = hats.iter().map(|h| h.role).collect();
        assert_eq!(roles, (1..=9).collect::<BTreeSet<u32>>());
    }

    /// The substitution inflates geometrically: the H outline's radius
    /// grows every level, with per-step factors 1.78, 2.64, 2.66, 2.64
    /// (as built) — within ~2% of φ² ≈ 2.618 after the first step. The
    /// iterated outlines are deliberately NOT exactly φ²-self-similar
    /// (the paper builds each generation's supertiles with slightly
    /// different outlines, which drift around the limit inflation), so
    /// only the growth band is asserted.
    #[test]
    fn substitution_inflates_geometrically() {
        const PHI2: f64 = 2.618033988749895;
        let levels = build_levels(4);
        let radius = |d: usize| -> f64 {
            levels[d].outlines[TYPE_H as usize]
                .iter()
                .map(|&(a, b)| (a * a + a * b + b * b).sqrt())
                .fold(0.0, f64::max)
        };
        let mut prev = radius(0);
        assert!(prev > 0.0);
        for d in 1..=4usize {
            let ratio = radius(d) / prev;
            if d == 1 {
                assert!(ratio > 1.5, "level 1: outline radius ratio {ratio}");
            } else {
                assert!(
                    (ratio - PHI2).abs() < 0.065,
                    "level {d}: outline radius ratio {ratio} not within 2.5% of φ²"
                );
            }
            prev = radius(d);
        }
    }

    /// Bit-exact tile identity: same seed and identical vertex bits.
    fn tile_key(tile: &Tile) -> (u32, Vec<u64>) {
        let mut bits = Vec::with_capacity(tile.poly.pts.len() * 2);
        for p in &tile.poly.pts {
            bits.push(p.x.to_bits());
            bits.push(p.y.to_bits());
        }
        (tile.seed, bits)
    }

    /// Views sharing a 96-grid anchor cell and a view-radius bucket see
    /// one fixed translated lineage blob: growing, panning within the
    /// cell, or zooming keeps every visible tile bit-identical (nested
    /// patches mean a deeper acceptance only adds tiles). The four views
    /// below all quantize to anchor (0, 0) with seed offset (84, −84).
    #[test]
    fn patch_is_stable_under_view_changes() {
        let t = HatTiling;
        let p = ParamSet::default_for(t.params());
        let base = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-9.0, -7.0), v2(9.0, 7.0)),
            },
        );
        assert!(base.message.is_none(), "{:?}", base.message);
        let grown = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-9.4, -7.3), v2(9.4, 7.3)),
            },
        );
        let panned = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-8.5, -6.5), v2(9.5, 7.5)),
            },
        );
        let zoomed = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-9.0 / 1.3, -7.0 / 1.3), v2(9.0 / 1.3, 7.0 / 1.3)),
            },
        );

        let base_keys: std::collections::HashSet<(u32, Vec<u64>)> =
            base.tiles.iter().map(tile_key).collect();
        let zoomed_keys: std::collections::HashSet<(u32, Vec<u64>)> =
            zoomed.tiles.iter().map(tile_key).collect();
        let grown_keys: std::collections::HashSet<(u32, Vec<u64>)> =
            grown.tiles.iter().map(tile_key).collect();
        for k in &base_keys {
            assert!(
                grown_keys.contains(k),
                "grow: tile changed when the view grew"
            );
        }
        for k in &zoomed_keys {
            assert!(
                base_keys.contains(k),
                "zoom: tile changed when the view changed"
            );
        }
        let overlap = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-8.45, -6.45), v2(8.95, 6.95)),
            },
        );
        let panned_keys: std::collections::HashSet<(u32, Vec<u64>)> =
            panned.tiles.iter().map(tile_key).collect();
        for tile in &overlap.tiles {
            let k = tile_key(tile);
            assert!(
                base_keys.contains(&k),
                "pan: overlap tile missing/changed in base view"
            );
            assert!(
                panned_keys.contains(&k),
                "pan: overlap tile missing/changed in panned view"
            );
        }
        assert!(
            !overlap.tiles.is_empty(),
            "pan: overlap view produced no tiles"
        );
    }

    /// Every emitted tile is a unit hat: 13 vertices, area exactly 8√3
    /// at default parameters, seeds within 1..=9, several roles visible.
    #[test]
    fn emitted_tiles_are_unit_hats() {
        let t = HatTiling;
        let p = ParamSet::default_for(t.params());
        let r = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-9.0, -7.0), v2(9.0, 7.0)),
            },
        );
        assert!(r.message.is_none(), "{:?}", r.message);
        assert!(!r.tiles.is_empty());
        let want = 16.0 * H;
        let mut seeds = BTreeSet::new();
        for tile in &r.tiles {
            assert_eq!(tile.poly.pts.len(), 13, "hat vertex count");
            assert_eq!(tile.proto, 0, "monohedral");
            assert!((1..=9).contains(&tile.seed), "seed {}", tile.seed);
            assert!(
                (tile.poly.area() - want).abs() < 1e-9,
                "tile area {} != {want}",
                tile.poly.area()
            );
            seeds.insert(tile.seed);
        }
        assert!(seeds.len() >= 2, "several roles visible");
    }
}
