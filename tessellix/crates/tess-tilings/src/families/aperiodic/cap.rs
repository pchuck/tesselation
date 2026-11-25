//! CAP tiling — the cut-and-project tiling of Baake, Gähler and Sadun
//! (arXiv:2305.05639), realized as the edge-consistent deformation of the
//! hat metatile construction.
//!
//! # The object
//!
//! The CAP tiling's prototiles are the four hat metatiles T, H, P, F with
//! every boundary edge replaced by its deformed length. The paper fixes
//! the shape vector (its §3, lines 1007–1030):
//!
//! ```text
//! (A, B, X, F, L) = (3φ, 3φ, 1, φ+ξ, 2φ) = μ₁ + (φ+ξ)μ₂ + 2φμ₃
//! ```
//!
//! a left-eigenvector of σ̃₁ with eigenvalue φ² — "this constitutes the
//! LIDS property". Each prototile occurs in six rotations `r^k` and **not**
//! in a reflected version (Fig 6: "Each tile occurs in six different
//! orientations, but not in a reflected version"). Tile areas are exactly
//! `v = (√3/4)·(15+27φ, 9+9φ, 14+22φ, 15+23φ)` for (H, T, P, F), and the
//! paper's frequency vector `f = (1/3, 5/3−φ, 2φ−3, 2−φ)` gives `v·f =
//! 5√3φ²` (lines 1338–1375) — both re-derived exactly by the tests below.
//!
//! # Exact geometry
//!
//! Hat-space positions are Eisenstein pairs `(a, b)` = `a + b·ω` exactly
//! as in [`super::hat`] (this module duplicates hat.rs's substitution
//! tables verbatim — same precedent as turtle.rs — so the lineage blobs
//! are the *same objects* the batteries verified against hat.rs).
//!
//! CAP-space positions live in `ℤ[ξ,φ]`, stored as integer 4-vectors
//! `(a, b, c, d)` meaning `a + bφ + cξ + dφξ` with `φ² = φ+1`, `ξ² = ξ−1`,
//! `ξ³ = −1`. Multiplication by `ξ^k` is the integer map
//! `(a,b,c,d) → (−c,−d,a+c,b+d)`; the world position is
//! `x = a + bφ + (c+dφ)/2`, `y = (c+dφ)·√3/2`. All construction arithmetic
//! is integer; floating point appears only when a vertex is emitted.
//!
//! # The deformation D (the construction actually generated)
//!
//! Each metatile type has a boundary walk on the unit Eisenstein lattice:
//! consecutive walk marks are `m·ξ^{kd}` apart, and every walk segment
//! carries one of the matching-rule labels (tables [`WALK_H`]..[`WALK_F`];
//! marks in [`MARKS_H`]..[`MARKS_F`]; label codes as in hat.rs's
//! `BOUNDARY`, which the tests cross-check). The deformation D replaces
//! every placed walk segment `m·ξ^{kd+k}` of a tile `(ty, k, t)` by the
//! straight CAP edge
//!
//! ```text
//! λ_label · ξ^{kd+k},   λ = (3φ, 3φ, 1, φ+ξ, 2φ) per label class
//! ```
//!
//! ([`LAMBDA`]) — exactly the paper's Fig 6 tiles. D is extended to all
//! walk-mark vertices of a patch by propagation from one anchor vertex;
//! it is well-defined because the matching rules force the same
//! replacement on both sides of every shared edge ([BGS23] Lemma 7:
//! "consistent pseudo-inflation with a complete 'belt' of positive
//! thickness" — the belt makes legal patches rigidly interconnected).
//! The tests assert the propagation is conflict-free (every vertex one
//! unique value) and that every tile's deformed ring equals the shared
//! prototile shape translated by `D(tile origin)` — the shape law that
//! makes the four prototiles well-defined.
//!
//! # Generation for a view
//!
//! The plumbing mirrors `hat.rs`: similarity-map region handling, the
//! 96-grid anchor + wedge candidate seeds, depth escalation 3..=6 over
//! the H lineage ([`build_levels`]/[`collect_metas`], the paper's own
//! Fig 2 assignment of tiles to supertiles), a displacement-padded prune
//! window (the deformation moves tiles by up to ~0.25·|t| against the
//! anchor, so the window pad grows with the view radius), and exact
//! clipped-area acceptance `covered ≥ want·(1−1e-9)` which catches both
//! gaps and overlaps at once. One step differs: there is **no demote** —
//! the metatiles themselves are the tiles. After window culling, one BFS
//! deformation map is built over the culled patch's walk-mark mesh,
//! anchored (gauge `D(t₀) = t₀`) at the culled tile nearest the view
//! center with a deterministic tie-break; tiles outside the BFS component
//! are skipped, and if any of them met the view the acceptance would fail
//! and the sweep would escalate — rejection, never corruption.
//!
//! Termination: the blob reach grows ≈ φ² per level (measured 148, 410,
//! 1096, 2892 Eisenstein units at depths 3..6 — the LIDS scaling), and
//! the depth-(d+1) blob contains the depth-d one at fixed positions, so
//! acceptance is monotone in depth. Depth 6 suffices for every view
//! within the tile budget (the budget caps square views at side ≈ 1597;
//! depth 6 reaches ≈ 2080), so larger views fail the budget precheck with
//! the standard zoom-in message.
//!
//! Determinism: same params + query ⇒ same region, anchor, candidate
//! list, culled order, BFS anchor (total order on (distance², type,
//! rotation, position)), same D-values (conflict-free ⇒ unique), same
//! emission order ⇒ bit-identical output (asserted by the standard
//! `*_deterministic` test).
//!
//! Panning across a 96-grid anchor cell (or a radius bucket) re-seeds the
//! patch, and the per-generation gauge translates the emitted piece of
//! the plane tiling by a metatile-scale vector — a valid translate of the
//! same tiling; coverage, exactness and determinism are unaffected.
//!
//! # Prototile indexing and seeds
//!
//! `proto` 0..3 = the metatile types (H, T, P, F) — hat.rs's type order.
//! `seed` ranks the prototiles by descending area, largest first per the
//! registry convention: H (25.41) → 0, F (22.61) → 1, P (21.48) → 2,
//! T (10.20) → 3 ([`SEED_OF`]).
//!
//! # Verification
//!
//! Exact stdlib-only Python battery (`.scratch/cap2/`: `cap_spike.py`,
//! `probe9.py`, `probe10.py`, `probe11.py`; results in `results*.txt`;
//! design doc `docs/cap-cut-project-design.md`): BFS conflicts 0 at
//! depths 1–3 (135/797/5000 vertices), cross-level D agreement, patch
//! agreement with the hat lineage at levels 1–3, orientation census,
//! displacement law, and a full emission battery (40×40 grids + 400
//! seeded random points per region, clipped-area equality rel = 0.0,
//! culling exactness, bit-exact determinism, far offset, scale+rotation).
//! The Rust tests below re-assert the walk tables, prototile geometry,
//! conflict-freeness, the shape law, blob nesting/counts, pairwise
//! non-overlap and edge matching, and the orientation census.

use std::collections::HashMap;

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::TilingCategory;
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

/// φ — the golden ratio.
const PHI: f64 = 1.618033988749895;

/// √3/2 — the y scale of the Eisenstein-to-world map (as in `hat.rs`).
const H: f64 = 0.8660254037844386;

/// Metatile type indices (hat.rs's order).
const TYPE_H: u8 = 0;
const TYPE_T: u8 = 1;
const TYPE_P: u8 = 2;
const TYPE_F: u8 = 3;

/// Depth cap for the seeded coverage search (hat.rs's value). The blob
/// reach grows ≈ φ² per level (measured 148/410/1096/2892 at depths
/// 3..6) and the blob is nested, so depth 6 covers every view the tile
/// budget admits; the precheck below usually fires first.
const MAX_DEPTH: u32 = 6;

/// Smallest CAP tile area (T): `(√3/4)(9+9φ) = 2.25φ²√3 ≈ 10.203` —
/// the tile-budget unit (hat.rs uses the hat's 16·H).
const A_MIN: f64 = (9.0 + 9.0 * PHI) * H / 2.0;

/// Exact CAP tile areas in type order (H, T, P, F):
/// `v = (√3/4)·(15+27φ, 9+9φ, 14+22φ, 15+23φ)` (paper lines 1370–1375).
const AREA: [f64; 4] = [
    (15.0 + 27.0 * PHI) * H / 2.0,
    (9.0 + 9.0 * PHI) * H / 2.0,
    (14.0 + 22.0 * PHI) * H / 2.0,
    (15.0 + 23.0 * PHI) * H / 2.0,
];

/// Pattern seed per type: prototiles ranked by descending area, largest
/// first (H → 0, F → 1, P → 2, T → 3); indexed by type.
const SEED_OF: [u32; 4] = [0, 3, 2, 1];

/// Extra window pad around the region for the mark rings: metatile
/// hex-radius 12 plus one (the rings stay within their tile's hat bbox).
const META_PAD: f64 = 13.0;

// ---------------------------------------------------------------------------
// Hat-lineage machinery — duplicated verbatim from `hat.rs` (same precedent
// as turtle.rs duplicating HAT) so that this module is self-contained and
// the blobs are exactly the objects the Python batteries verified.
// ---------------------------------------------------------------------------

/// The hat's 13 vertices in Eisenstein coordinates `(a, b)` = `a + b·ω`.
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

/// Cluster contents per metatile type: its hats as
/// `(rotation k, reflected, translation (a, b))` in the metatile's frame.
const CLUSTER_HATS: [[(u8, bool, i64, i64); 4]; 4] = [
    [(4, true, 4, 4), (0, false, 0, 6), (0, false, 6, 0), (4, false, 2, 2)],
    [(0, false, 0, 2), (0, false, 0, 0), (0, false, 0, 0), (0, false, 0, 0)],
    [(5, false, 2, 0), (0, false, 6, -2), (0, false, 0, 0), (0, false, 0, 0)],
    [(5, false, 2, 0), (0, false, 6, -2), (0, false, 0, 0), (0, false, 0, 0)],
];

/// Number of hats per metatile type.
const CLUSTER_HAT_COUNT: [usize; 4] = [4, 1, 2, 2];

/// Level-0 metatile outlines (corners in the order [`RULES`] refers to
/// them), padded with a repeat of corner 0 to 6 entries.
const OUTLINES0: [[[i64; 2]; 6]; 4] = [
    [[10, 0], [2, 8], [0, 8], [0, 0], [2, -2], [10, -2]],
    [[0, 0], [6, 0], [0, 6], [0, 0], [0, 0], [0, 0]],
    [[4, -4], [12, -4], [8, 0], [0, 0], [0, 0], [0, 0]],
    [[4, -4], [10, -4], [10, -2], [8, 0], [0, 0], [0, 0]],
];

/// Corner counts of the level-0 outlines.
const OUTLINE_LEN0: [usize; 4] = [6, 3, 4, 5];

/// One substitution rule (Figure 2 of [BGS23], "the original one from
/// [17]").
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

/// The patch children kept as the contents of each next-level supertile
/// (indices into the 29-tile patch), padded with zeros.
const KEPT: [[usize; 10]; 4] = [
    [0, 9, 16, 27, 26, 6, 1, 8, 10, 15],
    [11, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [7, 2, 3, 4, 28, 0, 0, 0, 0, 0],
    [21, 20, 22, 23, 24, 25, 0, 0, 0, 0],
];

/// Kept-children count per supertile type.
const KEPT_LEN: [usize; 4] = [10, 1, 5, 6];

/// Rotate an Eisenstein point by k·60° anticlockwise (`(a, b) · ω^k`).
fn rot(p: [i64; 2], k: u8) -> [i64; 2] {
    let (mut a, mut b) = (p[0], p[1]);
    for _ in 0..(k % 6) {
        let (na, nb) = (-b, a + b);
        a = na;
        b = nb;
    }
    [a, b]
}

/// Reflect an Eisenstein point in the x-axis (conjugate).
fn refl(p: [i64; 2]) -> [i64; 2] {
    [p[0] + p[1], -p[1]]
}

/// Apply a hat placement `(k, reflected, t)` to a point.
fn place_pt(p: [i64; 2], k: u8, reflected: bool, t: [i64; 2]) -> [i64; 2] {
    let q = if reflected { refl(p) } else { p };
    let r = rot(q, k);
    [r[0] + t[0], r[1] + t[1]]
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

/// Line intersection in floating point (as in the reference construction
/// of the new supertile outlines).
fn intersect_f(p1: (f64, f64), q1: (f64, f64), p2: (f64, f64), q2: (f64, f64)) -> (f64, f64) {
    let den = (p1.0 - q1.0) * (p2.1 - q2.1) - (p1.1 - q1.1) * (p2.0 - q2.0);
    let u = ((p1.0 - p2.0) * (p2.1 - q2.1) - (p1.1 - p2.1) * (p2.0 - q2.0)) / den;
    (p1.0 + u * (q1.0 - p1.0), p1.1 + u * (q1.1 - p1.1))
}

/// Find the rotation k (0..6) and translation putting the outline's edge
/// `f` (direction-reversed) onto the segment p->q.
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
/// (in debug builds) the construction's exactness.
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
/// and the kept children as `(child type, rotation, translation)`
/// referencing the PREVIOUS level's supertiles, with exact integer
/// translations. `blob` is the exact `(a, b)` bounding box
/// `(a0, b0, a1, b1)` of the supertile's hat content — which contains
/// every walk-mark ring of its level-0 metatiles (the marks lie on the
/// metatile boundaries, and the hats tile each metatile exactly).
struct Level {
    outlines: [Vec<(f64, f64)>; 4],
    children: [Vec<(u8, u8, [i64; 2])>; 4],
    blob: [(i64, i64, i64, i64); 4],
}

/// Transform a blob box `(a0, b0, a1, b1)` by the placement `(k, t)`.
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

/// Compose a supertile placement `(k, t)` with a child placement
/// `(k2, t2)` given in the supertile's frame (rotations only — metatile
/// placements never reflect).
fn compose_meta(k: u8, t: [i64; 2], k2: u8, t2: [i64; 2]) -> (u8, [i64; 2]) {
    let rt = rot(t2, k);
    ((k + k2) % 6, [t[0] + rt[0], t[1] + rt[1]])
}

/// Build the supertile hierarchy for levels 0..=depth (hat.rs verbatim).
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

        // fit the 29-tile patch: placements are snapped to exact integers
        let mut patch_k: Vec<u8> = Vec::with_capacity(29);
        let mut patch_t: Vec<[i64; 2]> = Vec::with_capacity(29);
        let mut patch_ty: Vec<u8> = Vec::with_capacity(29);
        for rule in &RULES {
            let (k, t) = match *rule {
                Rule::Seed(_) => (0u8, [0i64, 0]),
                Rule::Edge(i, e, ty, f) => {
                    let outline = &prev.outlines[patch_ty[i] as usize];
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

        // new outlines: key patch points + 120° hexagon corners (hat.rs
        // verbatim — the outlines only drive rule alignment; CAP never
        // emits them)
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
/// `(type, rotation, translation)` with exact integer placements. When
/// `prune` is given, subtrees whose blob box misses the
/// `(a0, b0, a1, b1)` window are skipped (sound: a supertile's hat
/// content — hence all its walk-mark rings — lies inside its blob box).
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

// ---------------------------------------------------------------------------
// CAP deformation tables
// ---------------------------------------------------------------------------

/// CAP edge displacement per matching-rule label, as `(a, b, c, d)` =
/// `a + bφ + cξ + dφξ`. Label classes (hat.rs's codes 0 A+, 1 A−, 2 B+,
/// 3 B−, 4 X+, 5 X−, 6 F+, 7 F−, 8 L): A/B edges get `3φ`, X edges `1`,
/// F edges `φ+ξ`, L edges `2φ` — the paper's vector
/// `(A, B, X, F, L) = (3φ, 3φ, 1, φ+ξ, 2φ)` (§3, lines 1007–1010).
const LAMBDA: [[i64; 4]; 9] = [
    [0, 3, 0, 0], // A+ : 3φ
    [0, 3, 0, 0], // A− : 3φ
    [0, 3, 0, 0], // B+ : 3φ
    [0, 3, 0, 0], // B− : 3φ
    [1, 0, 0, 0], // X+ : 1
    [1, 0, 0, 0], // X− : 1
    [0, 1, 1, 0], // F+ : φ+ξ
    [0, 1, 1, 0], // F− : φ+ξ
    [0, 2, 0, 0], // L  : 2φ
];

/// Boundary walk marks per type, in walk order (the marked points of
/// hat.rs's `BOUNDARY`, regrouped by type). Walk mark 0 is the tile
/// origin `(0, 0)` for every type.
const MARKS_H: [[i64; 2]; 9] = [
    [0, 0],
    [2, -2],
    [8, -2],
    [10, -2],
    [10, 0],
    [4, 6],
    [2, 8],
    [0, 8],
    [0, 2],
];
const MARKS_T: [[i64; 2]; 3] = [[0, 0], [6, 0], [0, 6]];
const MARKS_P: [[i64; 2]; 8] = [
    [0, 0],
    [2, -2],
    [4, -4],
    [6, -4],
    [12, -4],
    [10, -2],
    [8, 0],
    [6, 0],
];
const MARKS_F: [[i64; 2]; 9] = [
    [0, 0],
    [2, -2],
    [4, -4],
    [6, -4],
    [8, -4],
    [10, -4],
    [10, -2],
    [8, 0],
    [6, 0],
];

/// Boundary walk segments per type as `(direction class kd, label, step
/// count m)`: the segment leaving mark j runs `m·ξ^{kd}` to mark j+1
/// (indices mod ring length). The tests verify these against hat.rs's
/// `BOUNDARY` table (marks, labels and lengths).
const WALK_H: [[u8; 3]; 9] = [
    [5, 4, 2],
    [0, 3, 6],
    [0, 5, 2],
    [1, 4, 2],
    [2, 3, 6],
    [2, 5, 2],
    [3, 4, 2],
    [4, 0, 6],
    [4, 5, 2],
];
const WALK_T: [[u8; 3]; 3] = [[0, 1, 6], [2, 1, 6], [4, 2, 6]];
const WALK_P: [[u8; 3]; 8] = [
    [5, 8, 2],
    [5, 5, 2],
    [0, 4, 2],
    [0, 1, 6],
    [2, 8, 2],
    [2, 5, 2],
    [3, 4, 2],
    [3, 2, 6],
];
const WALK_F: [[u8; 3]; 9] = [
    [5, 8, 2],
    [5, 5, 2],
    [0, 4, 2],
    [0, 8, 2],
    [0, 5, 2],
    [1, 6, 2],
    [2, 7, 2],
    [3, 4, 2],
    [3, 2, 6],
];

fn marks_of(ty: u8) -> &'static [[i64; 2]] {
    match ty {
        TYPE_H => &MARKS_H,
        TYPE_T => &MARKS_T,
        TYPE_P => &MARKS_P,
        _ => &MARKS_F,
    }
}

fn walk_of(ty: u8) -> &'static [[u8; 3]] {
    match ty {
        TYPE_H => &WALK_H,
        TYPE_T => &WALK_T,
        TYPE_P => &WALK_P,
        _ => &WALK_F,
    }
}

// ---------------------------------------------------------------------------
// ℤ[ξ,φ] (abcd) arithmetic
// ---------------------------------------------------------------------------

/// A CAP-space position: `(a, b, c, d)` meaning `a + bφ + cξ + dφξ`.
type Abcd = [i64; 4];

/// Multiply by `ξ^k` (`(a,b,c,d) → (−c,−d,a+c,b+d)` per step).
fn rot4(z: Abcd, k: u8) -> Abcd {
    let (mut a, mut b, mut c, mut d) = (z[0], z[1], z[2], z[3]);
    for _ in 0..(k % 6) {
        let (na, nb, nc, nd) = (-c, -d, a + c, b + d);
        a = na;
        b = nb;
        c = nc;
        d = nd;
    }
    [a, b, c, d]
}

fn neg4(z: Abcd) -> Abcd {
    [-z[0], -z[1], -z[2], -z[3]]
}

fn add4(u: Abcd, v: Abcd) -> Abcd {
    [u[0] + v[0], u[1] + v[1], u[2] + v[2], u[3] + v[3]]
}

/// World position of a CAP-space point.
fn world4(z: Abcd) -> Vec2 {
    let cf = z[2] as f64 + z[3] as f64 * PHI; // c + dφ
    let ab = z[0] as f64 + z[1] as f64 * PHI;
    Vec2::new(ab + cf / 2.0, cf * H)
}

// ---------------------------------------------------------------------------
// The deformation map
// ---------------------------------------------------------------------------

/// Build the deformation map over the culled patch's walk-mark mesh by
/// propagation from the anchor tile's origin (gauge `D(t₀) = t₀`): every
/// shared walk edge forces the same displacement on both sides, so each
/// reached mark gets one unique value; `conflicts` counts arrivals that
/// disagree with an existing value (must be 0 — the tests assert it at
/// every depth, and the belt structure of [BGS23] Lemma 7 is why).
/// Returns the map (hat-space mark → CAP-space abcd) and the conflict
/// count.
fn bfs_deform(culled: &[(u8, u8, [i64; 2])], anchor: [i64; 2]) -> (HashMap<[i64; 2], Abcd>, usize) {
    let mut adj: HashMap<[i64; 2], Vec<([i64; 2], Abcd)>> = HashMap::new();
    for &(ty, k, t) in culled {
        let marks = marks_of(ty);
        let segs = walk_of(ty);
        let n = marks.len();
        let mut w = Vec::with_capacity(n);
        for mk in marks.iter() {
            let r = rot(*mk, k);
            w.push([t[0] + r[0], t[1] + r[1]]);
        }
        for j in 0..n {
            let &[kd, lab, _] = &segs[j];
            let d = rot4(LAMBDA[lab as usize], kd.wrapping_add(k) % 6);
            adj.entry(w[j]).or_default().push((w[(j + 1) % n], d));
            adj.entry(w[(j + 1) % n]).or_default().push((w[j], neg4(d)));
        }
    }
    let mut dmap: HashMap<[i64; 2], Abcd> = HashMap::new();
    dmap.insert(anchor, [anchor[0], 0, anchor[1], 0]);
    let mut conflicts = 0usize;
    let mut stack = vec![anchor];
    while let Some(v) = stack.pop() {
        let dv = dmap[&v];
        if let Some(nbrs) = adj.get(&v) {
            for &(w, d) in nbrs {
                let nw = add4(dv, d);
                match dmap.get(&w) {
                    Some(&old) => {
                        if old != nw {
                            conflicts += 1;
                        }
                    }
                    None => {
                        dmap.insert(w, nw);
                        stack.push(w);
                    }
                }
            }
        }
    }
    (dmap, conflicts)
}

/// Bounding box of a placed tile's deformed mark ring (world f64).
fn ring_aabb(pts: &[Vec2]) -> (f64, f64, f64, f64) {
    let mut x0 = f64::MAX;
    let mut y0 = f64::MAX;
    let mut x1 = f64::MIN;
    let mut y1 = f64::MIN;
    for p in pts {
        x0 = x0.min(p.x);
        y0 = y0.min(p.y);
        x1 = x1.max(p.x);
        y1 = y1.max(p.y);
    }
    (x0, y0, x1, y1)
}

/// AABB hit with the battery's 1e-9 pad (culling must be exact: a tile
/// whose ring merely touches the region still counts).
fn aabb_hit(bb: (f64, f64, f64, f64), x0: f64, y0: f64, x1: f64, y1: f64) -> bool {
    const PAD: f64 = 1e-9;
    bb.0 <= x1 + PAD && bb.2 >= x0 - PAD && bb.1 <= y1 + PAD && bb.3 >= y0 - PAD
}

/// Area of the tile polygon `pts` clipped to `region` (the tiles are
/// convex, so one Sutherland–Hodgman clip against the box is exact;
/// interior tiles short-circuit to their exact area).
fn clipped_area(pts: &[Vec2], ty: u8, region: &BBox) -> f64 {
    let bb = ring_aabb(pts);
    if !aabb_hit(bb, region.min.x, region.min.y, region.max.x, region.max.y) {
        return 0.0;
    }
    if region.min.x <= bb.0 && region.min.y <= bb.1 && region.max.x >= bb.2 && region.max.y >= bb.3
    {
        return AREA[ty as usize];
    }
    let poly = Polygon::from_slice(pts);
    poly.clipped_to(region).map_or(0.0, |c| c.area())
}

// ---------------------------------------------------------------------------
// The tiling
// ---------------------------------------------------------------------------

/// CAP tiling (aperiodic; cut-and-project model set of control points).
pub struct CapTiling;

impl Tiling for CapTiling {
    fn id(&self) -> &'static str {
        "cap"
    }

    fn name(&self) -> &'static str {
        "CAP (cut-and-project tiling)"
    }

    fn description(&self) -> &'static str {
        "Baake–Gähler–Sadun's CAP tiling: the four hat metatiles T/H/P/F with edge displacements (3φ, 3φ, 1, φ+ξ, 2φ); aperiodic, homochiral (6 orientations, no reflections), inflation φ², pure point spectrum."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Aperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None).with_notes(
            "aperiodic; cut-and-project; 4 prototiles; inflation φ²; homochiral; MLD to its φ²-rescaling",
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
        // The smallest CAP tile (T) has area exactly A_MIN, so this
        // bounds the tile count before any substitution work.
        if region.area() > MAX_TILES as f64 * A_MIN {
            return fail("too many tiles for this view — zoom in");
        }

        // (a, b) geometry of the region (b = y/H, a = x − b/2). The
        // padded window drives blob pruning (mark rings span ≤ 12 in a
        // and b); the center and hexagonal radius drive the seeded
        // placement and the displacement pad.
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
        let (ca, cb) = ((a0 + a1) / 2.0, (b0 + b1) / 2.0);
        let mut radius = 0.0f64;
        for &(a, b) in &corners {
            let (da, db) = (a - ca, b - cb);
            radius = radius.max(da.abs()).max(db.abs()).max((da + db).abs());
        }

        // Seed the lineage blob into the view (hat.rs's scheme): the
        // view center is quantized to a 96-unit grid anchor and the blob
        // is translated by the anchor plus a wedge offset growing with
        // the hexagonal view radius.
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

        // Mark-ring bounding boxes per (type, rotation) for the
        // translated-tile window cull.
        let mut ring_box = [[0i64; 4]; 24];
        for ty in 0..4u8 {
            for k in 0..6u8 {
                let (mut amin, mut bmin, mut amax, mut bmax) =
                    (i64::MAX, i64::MAX, i64::MIN, i64::MIN);
                for mk in marks_of(ty).iter() {
                    let r = rot(*mk, k);
                    amin = amin.min(r[0]);
                    bmin = bmin.min(r[1]);
                    amax = amax.max(r[0]);
                    bmax = bmax.max(r[1]);
                }
                ring_box[(ty * 6 + k) as usize] = [amin, bmin, amax, bmax];
            }
        }

        let want = region.area();
        let mut found: Option<Vec<(u8, Vec<Vec2>)>> = None;
        'sweep: for &disp_coef in &[0.35f64, 1.0] {
            // The deformation moves tiles by up to ~0.25·|t| against the
            // BFS anchor, so the window pad grows with the view radius
            // (first sweep 0.35·radius, retry 1.0·radius; the exact
            // acceptance makes any shortfall merely a rejection).
            let disp_pad = disp_coef * radius + 2.0;
            let win = (
                a0 - META_PAD - disp_pad,
                b0 - META_PAD - disp_pad,
                a1 + META_PAD + disp_pad,
                b1 + META_PAD + disp_pad,
            );
            for &(da, db) in &candidates {
                let seed = [anchor[0] + da, anchor[1] + db];
                // integer prune window for collect_metas, in the blob's
                // own (seed-relative) frame, rounded outward
                let prune = (
                    (win.0.floor() as i64) - seed[0],
                    (win.1.floor() as i64) - seed[1],
                    (win.2.ceil() as i64) - seed[0],
                    (win.3.ceil() as i64) - seed[1],
                );
                for depth in 3..=MAX_DEPTH {
                    let levels = build_levels(depth);
                    let mut metas = Vec::new();
                    collect_metas(
                        &levels,
                        depth as usize,
                        TYPE_H as usize,
                        Some(prune),
                        &mut metas,
                    );
                    // translate into view frame + window cull on the
                    // translated mark-ring bbox
                    let mut culled: Vec<(u8, u8, [i64; 2])> = Vec::new();
                    for (ty, k, t) in metas {
                        let ta = t[0] + seed[0];
                        let tb = t[1] + seed[1];
                        let bb = ring_box[(ty * 6 + k) as usize];
                        let hi_a = (ta + bb[2]) as f64;
                        let lo_a = (ta + bb[0]) as f64;
                        let hi_b = (tb + bb[3]) as f64;
                        let lo_b = (tb + bb[1]) as f64;
                        if hi_a < win.0 || lo_a > win.2 || hi_b < win.1 || lo_b > win.3 {
                            continue;
                        }
                        culled.push((ty, k, [ta, tb]));
                    }
                    if culled.is_empty() {
                        continue;
                    }
                    // BFS anchor: the culled tile nearest the view
                    // center (deterministic tie-break on (type, k, t)).
                    let cx = ca.round() as f64 + (cb.round() as f64) / 2.0;
                    let cy = (cb.round() as f64) * H;
                    let mut best: Option<(f64, u8, u8, i64, i64)> = None;
                    for &(ty, k, t) in &culled {
                        let wx = t[0] as f64 + t[1] as f64 / 2.0;
                        let wy = t[1] as f64 * H;
                        let d2 = (wx - cx) * (wx - cx) + (wy - cy) * (wy - cy);
                        let better = match best {
                            None => true,
                            Some((bd2, bty, bk, bta, btb)) => {
                                d2 < bd2
                                    || (d2 == bd2
                                        && (ty, k, t[0], t[1]) < (bty, bk, bta, btb))
                            }
                        };
                        if better {
                            best = Some((d2, ty, k, t[0], t[1]));
                        }
                    }
                    let anchor_tile = [best.unwrap().3, best.unwrap().4];
                    let (dmap, conflicts) = bfs_deform(&culled, anchor_tile);
                    if conflicts != 0 {
                        continue;
                    }
                    // emit: deformed mark rings, culled by their own
                    // world aabb against the region (tiles whose ring is
                    // not in the BFS component are skipped — if any of
                    // them met the region, acceptance below fails and
                    // the sweep escalates)
                    let mut tiles_ts: Vec<(u8, Vec<Vec2>)> = Vec::new();
                    for &(ty, k, t) in &culled {
                        let marks = marks_of(ty);
                        let mut pts: Vec<Vec2> = Vec::with_capacity(marks.len());
                        let mut ok = true;
                        for mk in marks.iter() {
                            let r = rot(*mk, k);
                            let key = [t[0] + r[0], t[1] + r[1]];
                            match dmap.get(&key) {
                                Some(&z) => pts.push(world4(z)),
                                None => {
                                    ok = false;
                                    break;
                                }
                            }
                        }
                        if !ok {
                            continue;
                        }
                        if aabb_hit(
                            ring_aabb(&pts),
                            rmin.x,
                            rmin.y,
                            rmax.x,
                            rmax.y,
                        ) {
                            tiles_ts.push((ty, pts));
                        }
                    }
                    // exact acceptance: clipped-area accounting catches
                    // both gaps and overlaps
                    let mut covered = 0.0;
                    for (ty, pts) in &tiles_ts {
                        covered += clipped_area(pts, *ty, &region);
                    }
                    if covered >= want * (1.0 - 1e-9) {
                        found = Some(tiles_ts);
                        break 'sweep;
                    }
                }
            }
        }
        let tiles_ts = match found {
            Some(t) => t,
            None => return fail("too many tiles for this view — zoom in"),
        };

        // Emit. proto = metatile type; seed = descending-area rank.
        let mut tiles = Vec::with_capacity(tiles_ts.len());
        for &(ty, ref pts) in &tiles_ts {
            let poly = Polygon::new(pts.clone()).transform(&map);
            if aabb_hit(
                ring_aabb(&poly.pts),
                query.region.min.x,
                query.region.min.y,
                query.region.max.x,
                query.region.max.y,
            ) {
                tiles.push(Tile {
                    proto: ty,
                    seed: SEED_OF[ty as usize],
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

// ---------------------------------------------------------------------------
// Tests — the Rust mirror of the Python battery
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    /// hat.rs's `BOUNDARY` table (marked point, label code, segment
    /// lattice length), for cross-checking the walk tables.
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
    const BOUNDARY_RANGE: [(usize, usize); 4] = [(3, 9), (0, 3), (12, 8), (20, 9)];

    /// The walk tables close (walks return to their start) and match
    /// hat.rs's BOUNDARY table exactly: same marks, same labels, same
    /// segment lengths, same order.
    #[test]
    fn cap_walk_tables_match_hat_boundary() {
        let table: [(&'static [[i64; 2]], &'static [[u8; 3]]); 4] = [
            (&MARKS_H, &WALK_H),
            (&MARKS_T, &WALK_T),
            (&MARKS_P, &WALK_P),
            (&MARKS_F, &WALK_F),
        ];
        for (ty, (marks, segs)) in table.iter().enumerate() {
            assert_eq!(marks.len(), segs.len());
            // accumulate the walk: marks derive from the segments …
            let mut cur = [0i64, 0];
            for (j, &[kd, _lab, m]) in segs.iter().enumerate() {
                assert_eq!([cur[0], cur[1]], marks[j], "mark {j} of type {ty}");
                let d = rot([m as i64, 0], kd);
                cur = [cur[0] + d[0], cur[1] + d[1]];
            }
            assert_eq!([cur[0], cur[1]], [0, 0], "walk of type {ty} must close");
            // … and both agree with hat.rs's BOUNDARY rows.
            let (start, count) = BOUNDARY_RANGE[ty];
            for j in 0..count {
                let (bm, bl, blen) = BOUNDARY[start + j];
                assert_eq!(marks[j], bm, "BOUNDARY mark {j} of type {ty}");
                assert_eq!(segs[j][1], bl, "BOUNDARY label {j} of type {ty}");
                assert_eq!(segs[j][2] as i64, blen, "BOUNDARY length {j} of type {ty}");
            }
        }
    }

    /// Simplified corner count per type (collinear walk marks sit
    /// mid-edge): H 9→6, T 3→3, P 8→4, F 9→5 — the paper's Fig 6 tiles.
    fn simplify(pts: &[Vec2]) -> Vec<Vec2> {
        let mut pts = pts.to_vec();
        loop {
            let n = pts.len();
            let mut dropped = false;
            let mut out = Vec::with_capacity(n);
            for j in 0..n {
                let p = pts[(j + n - 1) % n];
                let q = pts[j];
                let r = pts[(j + 1) % n];
                let cross = (q.x - p.x) * (r.y - q.y) - (q.y - p.y) * (r.x - q.x);
                if cross.abs() < 1e-6 {
                    dropped = true;
                } else {
                    out.push(q);
                }
            }
            pts = out;
            if !dropped || pts.len() < 3 {
                return pts;
            }
        }
    }

    /// The four CAP prototiles: for every rotation, the deformed ring of
    /// a lone tile is CCW with exact area `v` (the paper's area vector)
    /// and simplifies to the right corner count.
    #[test]
    fn cap_outline_geometry() {
        for ty in 0..4u8 {
            let marks = marks_of(ty);
            let segs = walk_of(ty);
            let n = marks.len();
            for k in 0..6u8 {
                // base ring of a lone tile at the origin, identity gauge
                let mut z = [0i64; 4];
                let mut pts = Vec::with_capacity(n);
                pts.push(world4(z));
                for &[kd, lab, _] in segs.iter() {
                    z = add4(z, rot4(LAMBDA[lab as usize], (kd + k) % 6));
                    pts.push(world4(z));
                }
                assert_eq!(pts.len(), n + 1);
                pts.pop(); // the closing repeat of vertex 0
                // CCW with exact area
                let mut a2 = 0.0;
                for j in 0..n {
                    let p = pts[j];
                    let q = pts[(j + 1) % n];
                    a2 += p.x * q.y - q.x * p.y;
                }
                let area = a2 / 2.0;
                let exact = AREA[ty as usize];
                assert!(area > 0.0, "type {ty} k {k} not CCW: {area}");
                assert!(
                    (area - exact).abs() <= 1e-9 * exact,
                    "type {ty} k {k}: area {area} vs exact {exact}"
                );
                // corner count
                let corners = simplify(&pts);
                assert_eq!(
                    corners.len(),
                    OUTLINE_LEN0[ty as usize],
                    "type {ty} k {k}: {} corners",
                    corners.len()
                );
            }
        }
    }

    /// The deformation is consistent on the depth-3 blob (442 metatiles,
    /// census (147, 22, 105, 168)): BFS conflicts == 0, and every tile's
    /// deformed ring equals the shared base ring of its (type, rotation)
    /// translated by D(tile origin) — the shape law (exact in ℤ[ξ,φ]).
    #[test]
    fn cap_deformation_consistent_depth3() {
        let levels = build_levels(3);
        let mut metas = Vec::new();
        collect_metas(&levels, 3, TYPE_H as usize, None, &mut metas);
        assert_eq!(metas.len(), 442);
        let mut cen = [0usize; 4];
        for &(ty, _, _) in &metas {
            cen[ty as usize] += 1;
        }
        assert_eq!(cen, [147, 22, 105, 168]);

        // the origin H tile is present at every level (nested frames)
        let origin = (TYPE_H, 0u8, [0i64, 0]);
        assert!(metas.contains(&origin));
        let (dmap, conflicts) = bfs_deform(&metas, [0, 0]);
        assert_eq!(conflicts, 0, "BFS conflicts on the depth-3 blob");

        // base rings per (type, rotation), identity gauge at the origin
        let mut base = [[[0i64; 4]; 10]; 24];
        for ty in 0..4u8 {
            let segs = walk_of(ty);
            for k in 0..6u8 {
                let mut z = [0i64; 4];
                for (j, &[kd, lab, _]) in segs.iter().enumerate() {
                    base[(ty * 6 + k) as usize][j] = z;
                    z = add4(z, rot4(LAMBDA[lab as usize], (kd + k) % 6));
                }
            }
        }
        // shape law for every tile
        for &(ty, k, t) in &metas {
            let marks = marks_of(ty);
            let d0 = dmap[&t];
            for (j, mk) in marks.iter().enumerate() {
                let r = rot(*mk, k);
                let key = [t[0] + r[0], t[1] + r[1]];
                let zv = dmap[&key];
                let expect = add4(base[(ty * 6 + k) as usize][j], d0);
                assert_eq!(zv, expect, "shape law: ({ty}, {k}, {:?}) mark {j}", t);
            }
        }
    }

    /// Sutherland–Hodgman clip of convex `subject` against convex
    /// `clip` (both CCW) — test-only pairwise overlap accounting.
    fn clip_convex_convex(subject: &[Vec2], clip: &[Vec2]) -> Vec<Vec2> {
        let mut out = subject.to_vec();
        let n = clip.len();
        for i in 0..n {
            if out.is_empty() {
                break;
            }
            let a = clip[i];
            let b = clip[(i + 1) % n];
            let ex = b.x - a.x;
            let ey = b.y - a.y;
            let input = std::mem::take(&mut out);
            for j in 0..input.len() {
                let p = input[j];
                let q = input[(j + 1) % input.len()];
                let dp = ex * (p.y - a.y) - ey * (p.x - a.x);
                let dq = ex * (q.y - a.y) - ey * (q.x - a.x);
                if dp >= 0.0 {
                    out.push(p);
                }
                if (dp > 0.0 && dq < 0.0) || (dp < 0.0 && dq > 0.0) {
                    let t = dp / (dp - dq);
                    out.push(Vec2::new(p.x + t * (q.x - p.x), p.y + t * (q.y - p.y)));
                }
            }
        }
        out
    }

    fn shoelace(pts: &[Vec2]) -> f64 {
        let mut a = 0.0;
        let n = pts.len();
        for j in 0..n {
            let p = pts[j];
            let q = pts[(j + 1) % n];
            a += p.x * q.y - q.x * p.y;
        }
        a / 2.0
    }

    /// The depth-2 blob (64 metatiles) deforms to a legal CAP patch:
    /// every ring resolves, areas sum to the exact census total, no
    /// pairwise overlap (convex clip census), and every shared boundary
    /// carries complementary matching-rule labels.
    #[test]
    fn cap_blob_patch_is_legal_depth2() {
        let levels = build_levels(2);
        let mut metas = Vec::new();
        collect_metas(&levels, 2, TYPE_H as usize, None, &mut metas);
        assert_eq!(metas.len(), 64);
        let (dmap, conflicts) = bfs_deform(&metas, [0, 0]);
        assert_eq!(conflicts, 0);

        // rings + exact area bookkeeping
        let mut rings: Vec<(u8, Vec<Vec2>)> = Vec::new();
        let mut total = 0.0;
        let mut exact_total = 0.0;
        let mut cen = [0usize; 4];
        for &(ty, k, t) in &metas {
            cen[ty as usize] += 1;
            let marks = marks_of(ty);
            let mut pts = Vec::with_capacity(marks.len());
            for mk in marks.iter() {
                let r = rot(*mk, k);
                let key = [t[0] + r[0], t[1] + r[1]];
                let &z = dmap.get(&key).expect("depth-2 blob is one component");
                pts.push(world4(z));
            }
            let mut a2 = 0.0;
            let n = pts.len();
            for j in 0..n {
                let p = pts[j];
                let q = pts[(j + 1) % n];
                a2 += p.x * q.y - q.x * p.y;
            }
            assert!(a2 > 0.0, "ring of ({ty}, {k}) not CCW");
            total += a2 / 2.0;
            exact_total += AREA[ty as usize];
            rings.push((ty, pts));
        }
        assert!((total - exact_total).abs() <= 1e-6, "{total} vs {exact_total}");

        // pairwise overlap census (AABB prefilter, then convex clip)
        let boxes: Vec<(f64, f64, f64, f64)> =
            rings.iter().map(|(_, p)| ring_aabb(p)).collect();
        for i in 0..rings.len() {
            for j in (i + 1)..rings.len() {
                let (x0a, y0a, x1a, y1a) = boxes[i];
                let (x0b, y0b, x1b, y1b) = boxes[j];
                if x0a > x1b || x0b > x1a || y0a > y1b || y0b > y1a {
                    continue;
                }
                let inter = clip_convex_convex(&rings[i].1, &rings[j].1);
                if inter.len() >= 3 {
                    let area = shoelace(&inter).abs();
                    assert!(
                        area < 1e-9,
                        "tiles {i} and {j} overlap by {area}"
                    );
                }
            }
        }

        // shared walk segments carry complementary labels (the
        // metatile matching rules survive the deformation)
        let comp = |l: u8| match l {
            0 => 1,
            1 => 0,
            2 => 3,
            3 => 2,
            4 => 5,
            5 => 4,
            6 => 7,
            7 => 6,
            _ => 8,
        };
        let mut segs: BTreeSet<([i64; 2], [i64; 2], u8)> = BTreeSet::new();
        let mut pairs = 0usize;
        for &(ty, k, t) in &metas {
            let marks = marks_of(ty);
            let walk = walk_of(ty);
            let n = marks.len();
            let mut w = Vec::with_capacity(n);
            for mk in marks.iter() {
                let r = rot(*mk, k);
                w.push([t[0] + r[0], t[1] + r[1]]);
            }
            for j in 0..n {
                let lab = walk[j][1];
                if segs.contains(&(w[(j + 1) % n], w[j], comp(lab))) {
                    pairs += 1;
                }
                assert!(
                    segs.insert((w[j], w[(j + 1) % n], lab)),
                    "duplicate directed segment at tile ({ty}, {k}, {t:?})"
                );
            }
        }
        assert_eq!(pairs, 210, "shared walk segments");
    }

    /// One common frame: the depth-d blob is a literal subset of the
    /// depth-(d+1) blob (the level-n H supertile sits at the identity in
    /// every level), so the deformation maps agree on their common
    /// domain and deeper acceptance only adds tiles.
    #[test]
    fn cap_blob_nested_across_depths() {
        let mut prev: BTreeSet<(u8, u8, i64, i64)> = BTreeSet::new();
        for depth in 1..=3u32 {
            let levels = build_levels(depth);
            let mut metas = Vec::new();
            collect_metas(&levels, depth as usize, TYPE_H as usize, None, &mut metas);
            let cur: BTreeSet<(u8, u8, i64, i64)> = metas
                .iter()
                .map(|&(ty, k, t)| (ty, k, t[0], t[1]))
                .collect();
            assert_eq!(cur.len(), metas.len(), "duplicate placements at {depth}");
            for p in &prev {
                assert!(cur.contains(p), "depth {} lost {:?}", depth - 1, p);
            }
            prev = cur;
        }
    }

    /// Metatile census of the H-lineage blobs at depths 1..=6 (the
    /// termination table: reach ≈ φ² per level, blob nested), and the
    /// hat counts they imply via the fixed cluster dissections
    /// (matching hat.rs's 25/169/1156/7921/54289 table).
    #[test]
    fn cap_blob_count_table() {
        const COUNTS: [usize; 6] = [10, 64, 442, 3025, 20737, 142129];
        const HAT_COUNTS: [usize; 5] = [25, 169, 1156, 7921, 54289];
        for depth in 1..=6u32 {
            let levels = build_levels(depth);
            let mut metas = Vec::new();
            collect_metas(&levels, depth as usize, TYPE_H as usize, None, &mut metas);
            assert_eq!(metas.len(), COUNTS[depth as usize - 1], "depth {depth}");
            if depth <= 5 {
                let mut cen = [0usize; 4];
                for &(ty, _, _) in &metas {
                    cen[ty as usize] += 1;
                }
                let hats: usize = (0..4)
                    .map(|ty| cen[ty] * CLUSTER_HAT_COUNT[ty])
                    .sum();
                assert_eq!(hats, HAT_COUNTS[depth as usize - 1], "depth {depth}");
            }
        }
    }

    /// Orientation census on a real generated view: every (type,
    /// rotation) pair occurs (six orientations per prototile, no
    /// reflections — each emitted tile matches exactly one of the 24
    /// base-ring shapes up to translation).
    #[test]
    fn cap_orientation_census() {
        let t = CapTiling;
        let p = ParamSet::default_for(t.params());
        let result = t.generate(
            &p,
            &Query {
                region: BBox::new(Vec2::new(-60.0, -45.0), Vec2::new(60.0, 45.0)),
            },
        );
        assert!(result.message.is_none(), "{:?}", result.message);
        assert!(!result.tiles.is_empty());

        // the 24 base-ring shapes (lone tile, identity gauge)
        let mut shapes: Vec<(u8, u8, Vec<Vec2>)> = Vec::new();
        for ty in 0..4u8 {
            let marks = marks_of(ty);
            let segs = walk_of(ty);
            for k in 0..6u8 {
                let mut z = [0i64; 4];
                let mut pts = vec![world4(z)];
                for &[kd, lab, _] in segs.iter() {
                    z = add4(z, rot4(LAMBDA[lab as usize], (kd + k) % 6));
                    pts.push(world4(z));
                }
                pts.pop();
                shapes.push((ty, k, pts));
            }
        }
        let mut seen: BTreeSet<(u8, u8)> = BTreeSet::new();
        for tile in &result.tiles {
            assert_eq!(tile.seed, SEED_OF[tile.proto as usize], "seed of proto");
            let mut matched = None;
            for &(ty, k, ref exp) in &shapes {
                if exp.len() != tile.poly.pts.len() {
                    continue;
                }
                let o = Vec2::new(
                    tile.poly.pts[0].x - exp[0].x,
                    tile.poly.pts[0].y - exp[0].y,
                );
                let ok = exp.iter().zip(tile.poly.pts.iter()).all(|(e, p)| {
                    (p.x - (e.x + o.x)).abs() < 1e-6 && (p.y - (e.y + o.y)).abs() < 1e-6
                });
                if ok {
                    matched = Some((ty, k));
                    break;
                }
            }
            assert!(
                matched.is_some(),
                "tile does not match any base shape: proto {}",
                tile.proto
            );
            seen.insert(matched.unwrap());
        }
        let all: BTreeSet<(u8, u8)> = (0..4u8).flat_map(|ty| (0..6u8).map(move |k| (ty, k))).collect();
        assert_eq!(seen, all, "all 24 orientations must occur");
    }
}