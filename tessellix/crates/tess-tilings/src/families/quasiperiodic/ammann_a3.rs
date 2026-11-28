//! Ammann A3 (quasiperiodic) — the three-tile golden set, inflation φ = (1+√5)/2.
//!
//! Ammann A3 (the canonical Bielefeld-encyclopedia name) is a rectilinear
//! aperiodic substitution tiling by three "golden" tiles P1, P2, P3, each an
//! axis-aligned box with corner notches cut from it:
//!
//! ```text
//! P1: box W1 × H1, one TL notch a × b.
//! P2: box W2 × H2, TL notch c × d and TR notch e × f.
//! P3: box W3 × H3, TL notch g × h, TR notch i × j, BL notch k × l.
//! ```
//!
//! Every vertex lies in the ring Z[φ] = Q(√5): each coordinate is m + n·φ
//! with integer m, n (φ² = φ + 1). The dimensions (Z[φ] pairs, y-down, top-
//! left corner at the origin of each tile's own bbox):
//!
//! ```text
//! W1 = (15,23)   H1 = W1/φ = (8,15)
//! a  = W1/φ² = (7,8)   b = W1/φ³ = (1,7)   k = W1/φ⁴ = (6,1)
//! W2 = 2W1/φ = (16,30) H2 = W1 = (15,23)
//! c = φ·a = (8,15)  d = φ·b = a = (7,8)  e = b = (1,7)  f = H1 = (8,15)
//! W3 = H3 = W1 = (15,23)   g = h = a   i = b   j = H1   l = a
//! ```
//!
//! Areas (Z[φ]): A1 = (402,641), A2 = (641,1043) = φ·A1, A3 = (478,804).
//! The substitution (inflation φ) is the canonical three-row scheme; the
//! children tile the φ-inflated parent exactly (deflation view):
//!
//! ```text
//! S1 = φ·P1:  P2 r0          P1 r90ccw            (φ²A1 = A2 + A1)
//! S2 = φ·P2:  P1 r90cw  P3 r180  P1 r90ccw  P1 r0   (φ²A2 = 3A1 + A3)
//! S3 = φ·P3:  P1 r90ccw  P3 r180  P1 r0            (φ²A3 = 2A1 + A3)
//! ```
//!
//! Count matrix N (rows = parents P1,P2,P3; columns = children P1,P2,P3):
//! N = [[1,1,0],[3,0,1],[2,0,1]], with N·A = φ²·A exact in Z[φ].
//!
//! The shipped rule set is the **12-state** one (3 classes × 4 rotations)
//! pinned by the exact Z[φ] certificate (scratch in
//! `.scratch/ammann-a3/`; the `derive_a3.py` / `dump_a3_tables.py` outputs
//! are the constants transcribed below): states 0..3 = P2 r0/r90cw/r180/
//! r90ccw (P2 is the largest class = seed 0), 4..7 = P1, 8..11 = P3.
//!
//! **T-junctions are intrinsic** to this substitution (non-edge-to-edge,
//! the A4/M173 precedent for this family): level-1 rule frames S2 and S3
//! each carry 5 full-edge defects and 3 distinct T-junctions (S1 is clean).
//! At level 2 (deflating each row's r0 parent) the full-edge / T-junction
//! census is **S1 9/4, S2 13/5, S3 9/4** (defects / distinct junction
//! locations per level), pinned exactly in the in-module tests. The
//! partition is exact (no gaps or overlaps) at every level; only the
//! edge-to-edge property fails.
//!
//! Substitution-limit densities (left Perron–Frobenius eigenvector of N ∝
//! (φ², 1, 1/φ)): tile-count fractions ≈ 61.8 / 23.6 / 14.6 and area
//! fractions ≈ 52.4 / 32.4 / 15.3 for P1 / P2 / P3.
//!
//! Generation mirrors `ammann_a4.rs`: seed one P2 state-0 piece at scale
//! `s0`, body centre at the origin (the inradius about the body centre is
//! exactly (b/2)·s0 — the nearest boundary is the TL-notch inner corner).
//! Grow by φ while `reach > inradius·scale`, guarded by STEP_CAP; then
//! repeatedly deflate (each step replaces a piece by its 2–4 children at
//! 1/φ, which tile the parent exactly; culling is sound because a child
//! never leaves its parent). Emit each leaf piece (proto = class: P2=0,
//! P1=1, P3=2); no pairing step is needed.

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, MAX_TILES, SCALE_ID, SIMILARITY_PARAMS};

/// Golden ratio φ = (1+√5)/2. A decimal literal: the ring math below is
/// exact in Z[φ] and φ only ever appears as the *scale* at float emission
/// time (A4's √2 is a `std::f64::consts` literal, but the golden ratio has
/// no such constant).
const PHI: f64 = 1.618033988749895;
/// P2 area in cell units — the largest tile area, 641 + 1043·φ. The
/// pre-budget uses this as a valid lower bound on the tile count: every
/// emitted tile's area is at most s²·AREA_A2.
const AREA_A2: f64 = 641.0 + 1043.0 * PHI;
/// Refinement cap. φ^18 ≈ 5.9e6 units of seed reach; beyond ~18 levels the
/// accumulated float error would approach tile dimensions, so such extreme
/// zoom-outs fail with a message instead.
const STEP_CAP: usize = 18;

/// A Z[φ] scalar: (a, b) ⇔ a + b·φ (φ² = φ + 1).
type Z = (i32, i32);
/// A Z[φ] 2-point: (x, y) with each coordinate in Z.
type ZPt = (Z, Z);
/// A substitution child: (child state, anchor x, anchor y); the anchor is
/// the child's top-left corner in the φ-inflated parent frame.
type Child = (u8, Z, Z);

/// Class of a state: 0 = P2, 1 = P1, 2 = P3.
#[inline]
fn class_of(st: u8) -> u8 {
    st / 4
}

// ---------------------------------------------------------------------------
// The 12 state outlines in Z[φ] units (transcribed verbatim from the exact
// certificate, `dump_a3_tables.py`). Coordinates are (x=(a,b), y=(c,d));
// each cycle is in the tile's own local frame with the bbox top-left at
// (0,0). P2 states 0..3 (8 vertices, box (16,30)×(15,23)), P1 states 4..7
// (6 vertices, box (15,23)×(8,15)), P3 states 8..11 (10 vertices, box
// (15,23)×(15,23)).
// ---------------------------------------------------------------------------

const SHAPE_0: [ZPt; 8] = [
    ((0, 0), (7, 8)),
    ((8, 15), (7, 8)),
    ((8, 15), (0, 0)),
    ((15, 23), (0, 0)),
    ((15, 23), (8, 15)),
    ((16, 30), (8, 15)),
    ((16, 30), (15, 23)),
    ((0, 0), (15, 23)),
];

const SHAPE_1: [ZPt; 8] = [
    ((8, 15), (0, 0)),
    ((8, 15), (8, 15)),
    ((15, 23), (8, 15)),
    ((15, 23), (15, 23)),
    ((7, 8), (15, 23)),
    ((7, 8), (16, 30)),
    ((0, 0), (16, 30)),
    ((0, 0), (0, 0)),
];

const SHAPE_2: [ZPt; 8] = [
    ((16, 30), (8, 15)),
    ((8, 15), (8, 15)),
    ((8, 15), (15, 23)),
    ((1, 7), (15, 23)),
    ((1, 7), (7, 8)),
    ((0, 0), (7, 8)),
    ((0, 0), (0, 0)),
    ((16, 30), (0, 0)),
];

const SHAPE_3: [ZPt; 8] = [
    ((7, 8), (16, 30)),
    ((7, 8), (8, 15)),
    ((0, 0), (8, 15)),
    ((0, 0), (1, 7)),
    ((8, 15), (1, 7)),
    ((8, 15), (0, 0)),
    ((15, 23), (0, 0)),
    ((15, 23), (16, 30)),
];

const SHAPE_4: [ZPt; 6] = [
    ((0, 0), (8, 15)),
    ((15, 23), (8, 15)),
    ((15, 23), (0, 0)),
    ((7, 8), (0, 0)),
    ((7, 8), (1, 7)),
    ((0, 0), (1, 7)),
];

const SHAPE_5: [ZPt; 6] = [
    ((0, 0), (0, 0)),
    ((0, 0), (15, 23)),
    ((8, 15), (15, 23)),
    ((8, 15), (7, 8)),
    ((7, 8), (7, 8)),
    ((7, 8), (0, 0)),
];

const SHAPE_6: [ZPt; 6] = [
    ((15, 23), (0, 0)),
    ((0, 0), (0, 0)),
    ((0, 0), (8, 15)),
    ((8, 15), (8, 15)),
    ((8, 15), (7, 8)),
    ((15, 23), (7, 8)),
];

const SHAPE_7: [ZPt; 6] = [
    ((8, 15), (15, 23)),
    ((8, 15), (0, 0)),
    ((0, 0), (0, 0)),
    ((0, 0), (8, 15)),
    ((1, 7), (8, 15)),
    ((1, 7), (15, 23)),
];

const SHAPE_8: [ZPt; 10] = [
    ((0, 0), (7, 8)),
    ((7, 8), (7, 8)),
    ((7, 8), (0, 0)),
    ((14, 16), (0, 0)),
    ((14, 16), (8, 15)),
    ((15, 23), (8, 15)),
    ((15, 23), (15, 23)),
    ((6, 1), (15, 23)),
    ((6, 1), (8, 15)),
    ((0, 0), (8, 15)),
];

const SHAPE_9: [ZPt; 10] = [
    ((8, 15), (0, 0)),
    ((8, 15), (7, 8)),
    ((15, 23), (7, 8)),
    ((15, 23), (14, 16)),
    ((7, 8), (14, 16)),
    ((7, 8), (15, 23)),
    ((0, 0), (15, 23)),
    ((0, 0), (6, 1)),
    ((7, 8), (6, 1)),
    ((7, 8), (0, 0)),
];

const SHAPE_10: [ZPt; 10] = [
    ((15, 23), (8, 15)),
    ((8, 15), (8, 15)),
    ((8, 15), (15, 23)),
    ((1, 7), (15, 23)),
    ((1, 7), (7, 8)),
    ((0, 0), (7, 8)),
    ((0, 0), (0, 0)),
    ((9, 22), (0, 0)),
    ((9, 22), (7, 8)),
    ((15, 23), (7, 8)),
];

const SHAPE_11: [ZPt; 10] = [
    ((7, 8), (15, 23)),
    ((7, 8), (8, 15)),
    ((0, 0), (8, 15)),
    ((0, 0), (1, 7)),
    ((8, 15), (1, 7)),
    ((8, 15), (0, 0)),
    ((15, 23), (0, 0)),
    ((15, 23), (9, 22)),
    ((8, 15), (9, 22)),
    ((8, 15), (15, 23)),
];

/// The 12 state outlines.
fn shape(st: u8) -> &'static [ZPt] {
    match st {
        0 => &SHAPE_0,
        1 => &SHAPE_1,
        2 => &SHAPE_2,
        3 => &SHAPE_3,
        4 => &SHAPE_4,
        5 => &SHAPE_5,
        6 => &SHAPE_6,
        7 => &SHAPE_7,
        8 => &SHAPE_8,
        9 => &SHAPE_9,
        10 => &SHAPE_10,
        _ => &SHAPE_11,
    }
}

/// Evaluate a Z[φ] scalar at float: a + b·φ.
#[inline]
fn q(a: i32, b: i32) -> f64 {
    a as f64 + b as f64 * PHI
}

/// Division by φ in Z[φ]: (a, b) → (b − a, a). Exact on the ring since
/// 1/φ = φ − 1 = (−1, 1).
#[inline]
fn zdiv_phi(a: i32, b: i32) -> Z {
    (b - a, a)
}

// ---------------------------------------------------------------------------
// The 12 substitution rules (transcribed verbatim from the exact
// certificate, `dump_a3_tables.py`). Each rule lists (child state, anchor
// x, anchor y); the anchor is the child's top-left corner in the
// φ-inflated parent frame. RULE_4 = S1 (φ·P1), RULE_0 = S2 (φ·P2),
// RULE_8 = S3 (φ·P3) verbatim; the other nine are their R90 rotations
// (verified by the per-rule covariance test below).
// ---------------------------------------------------------------------------

const RULE_0: [Child; 4] = [
    (5, (0, 0), (8, 15)),
    (10, (7, 8), (8, 15)),
    (7, (15, 23), (0, 0)),
    (4, (15, 23), (15, 23)),
];

const RULE_1: [Child; 4] = [
    (6, (0, 0), (0, 0)),
    (11, (0, 0), (7, 8)),
    (4, (8, 15), (15, 23)),
    (5, (0, 0), (15, 23)),
];

const RULE_2: [Child; 4] = [
    (7, (22, 31), (0, 0)),
    (8, (8, 15), (0, 0)),
    (5, (7, 8), (8, 15)),
    (6, (0, 0), (0, 0)),
];

const RULE_3: [Child; 4] = [
    (4, (8, 15), (22, 31)),
    (9, (8, 15), (8, 15)),
    (6, (0, 0), (7, 8)),
    (7, (15, 23), (0, 0)),
];

const RULE_4: [Child; 2] = [(0, (0, 0), (0, 0)), (7, (15, 23), (0, 0))];

const RULE_5: [Child; 2] = [(1, (0, 0), (0, 0)), (4, (0, 0), (15, 23))];

const RULE_6: [Child; 2] = [(2, (7, 8), (0, 0)), (5, (0, 0), (0, 0))];

const RULE_7: [Child; 2] = [(3, (0, 0), (7, 8)), (6, (0, 0), (0, 0))];

const RULE_8: [Child; 3] = [
    (7, (8, 15), (0, 0)),
    (10, (0, 0), (8, 15)),
    (4, (8, 15), (15, 23)),
];

const RULE_9: [Child; 3] = [
    (4, (8, 15), (8, 15)),
    (11, (0, 0), (0, 0)),
    (5, (0, 0), (8, 15)),
];

const RULE_10: [Child; 3] = [
    (5, (7, 8), (8, 15)),
    (8, (8, 15), (0, 0)),
    (6, (0, 0), (0, 0)),
];

const RULE_11: [Child; 3] = [
    (6, (0, 0), (7, 8)),
    (9, (8, 15), (8, 15)),
    (7, (15, 23), (0, 0)),
];

/// The substitution rule for parent state `st`: the children that tile the
/// φ-inflated parent exactly.
fn rule(st: u8) -> &'static [Child] {
    match st {
        0 => &RULE_0,
        1 => &RULE_1,
        2 => &RULE_2,
        3 => &RULE_3,
        4 => &RULE_4,
        5 => &RULE_5,
        6 => &RULE_6,
        7 => &RULE_7,
        8 => &RULE_8,
        9 => &RULE_9,
        10 => &RULE_10,
        _ => &RULE_11,
    }
}

// ---------------------------------------------------------------------------
// Runtime pieces (floats only at emission, mirroring ammann_a4.rs)
// ---------------------------------------------------------------------------

struct Piece {
    st: u8,
    p: Vec2,
    s: f64,
}

impl Piece {
    fn verts(&self) -> Vec<Vec2> {
        shape(self.st)
            .iter()
            .map(|&(x, y)| self.p + Vec2::new(q(x.0, x.1), q(y.0, y.1)) * self.s)
            .collect()
    }

    fn poly(&self) -> Polygon {
        Polygon::from_slice(&self.verts())
    }

    fn aabb(&self) -> BBox {
        let mut lo = Vec2::new(f64::MAX, f64::MAX);
        let mut hi = Vec2::new(f64::MIN, f64::MIN);
        for &(x, y) in shape(self.st) {
            let v = self.p + Vec2::new(q(x.0, x.1), q(y.0, y.1)) * self.s;
            lo.x = lo.x.min(v.x);
            lo.y = lo.y.min(v.y);
            hi.x = hi.x.max(v.x);
            hi.y = hi.y.max(v.y);
        }
        BBox::new(lo, hi)
    }

    /// Substitution children at 1/φ of this piece's scale; they exactly
    /// tile this piece (deflation view). The child anchor lives in the
    /// φ-inflated parent frame as a Z[φ] point; dividing the frame down to
    /// the parent divides by φ = (1, 1) — exact on the ring.
    fn children(&self) -> Vec<Piece> {
        let s2 = self.s / PHI;
        rule(self.st)
            .iter()
            .map(|&(st, ax, ay)| {
                let (dx, dy) = zdiv_phi(ax.0, ax.1);
                let (ex, ey) = zdiv_phi(ay.0, ay.1);
                Piece {
                    st,
                    p: self.p + Vec2::new(q(dx, dy), q(ex, ey)) * self.s,
                    s: s2,
                }
            })
            .collect()
    }
}

/// The seed: one state-0 P2 piece, body centre at the origin (body
/// top-left at −(W2/2)·scale per axis). The inradius about the body centre
/// is exactly (b/2)·scale = (q(1,7)/2)·scale — the nearest boundary is the
/// TL-notch inner corner (c, d), at distance H2/2 − d = b/2 (exact: d = a,
/// H2/2 − a = b/2).
fn seed(scale: f64) -> Vec<Piece> {
    let half_x = 0.5 * q(16, 30) * scale; // W2/2
    let half_y = 0.5 * q(15, 23) * scale; // H2/2
    vec![Piece {
        st: 0,
        p: Vec2::new(-half_x, -half_y),
        s: scale,
    }]
}

/// One deflation step: replace every piece by its substitution children
/// (scale 1/φ), which tile the parent exactly.
///
/// Culling is sound: a child never leaves its parent, so a final piece
/// intersecting `region` has an ancestor chain whose aabbs all intersect
/// `region` — nothing needed is ever dropped.
fn refine_step(pieces: &[Piece], region: &BBox) -> Vec<Piece> {
    let mut out = Vec::with_capacity(pieces.len() * 3);
    for t in pieces {
        for ch in t.children() {
            if ch.aabb().intersects(*region) {
                out.push(ch);
            }
        }
    }
    out
}

/// Ammann A3 tiling (aperiodic, three-tile golden rectilinear set,
/// quasiperiodic substitution tiling with 5-fold / φ inflation).
pub struct AmmannA3Tiling;

impl Tiling for AmmannA3Tiling {
    fn id(&self) -> &'static str {
        "ammann-a3"
    }

    fn name(&self) -> &'static str {
        "Ammann A3 (quasiperiodic)"
    }

    fn description(&self) -> &'static str {
        "Quasiperiodic rectilinear golden tiling by substitution; non-edge-to-edge (T-junctions intrinsic)."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Quasiperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn default_color_mode(&self) -> ColorMode {
        ColorMode::Pattern
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None).with_notes(
            "aperiodic; three-tile golden set (φ = (1+√5)/2); non-edge-to-edge — T-junctions intrinsic to the substitution rules",
        );
        let fail = |message: &'static str| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: None,
            message: Some(message),
        };

        let map = similarity_map(params);
        let inv = map.inverse();

        // Region in the tiling's own (cell) coordinates: the axis-aligned
        // box around the inverse image of the four world corners.
        let mut rmin = Vec2::new(f64::MAX, f64::MAX);
        let mut rmax = Vec2::new(f64::MIN, f64::MIN);
        for &corner in &query.region.corners() {
            let pt = inv.transform(corner);
            rmin.x = rmin.x.min(pt.x);
            rmin.y = rmin.y.min(pt.y);
            rmax.x = rmax.x.max(pt.x);
            rmax.y = rmax.y.max(pt.y);
        }
        let region = BBox::new(rmin, rmax);

        // Farthest region corner from the origin (the seed P2 is centred
        // there).
        let mut reach: f64 = 0.0;
        for &corner in &region.corners() {
            reach = reach.max(corner.length());
        }
        if reach < 1e-9 {
            return fail("empty view");
        }

        // Pre-generation budget (mirrors lattice.rs): the returned tiles
        // must cover the query region and no A3 tile exceeds the P2's area
        // (cell units; tiles only get smaller than unit scale), so this is
        // a lower bound on the tile count — checked before any refinement
        // work.
        let s = params.float(SCALE_ID, 1.0);
        if query.region.area() / (s * s * AREA_A2) > MAX_TILES as f64 {
            return fail("too many tiles for this view — zoom in");
        }

        // Seed scale: the P2's inradius about its body centre is (b/2)·
        // scale, so pick scale = φ^n with n minimal subject to
        // reach ≤ inradius·scale; the refinement below then lands exactly
        // on unit pieces.
        let inrad = 0.5 * q(1, 7); // b/2
        let mut scale = 1.0_f64;
        let mut steps = 0_usize;
        while reach > inrad * scale && steps < STEP_CAP {
            scale *= PHI;
            steps += 1;
        }
        if reach > inrad * scale * (1.0 + 1e-9) {
            return fail("view too far from the seed — pan back or zoom in");
        }

        let mut pieces = seed(scale);
        for _ in 0..steps {
            pieces = refine_step(&pieces, &region);
        }
        if pieces.is_empty() {
            return fail("empty view");
        }

        // Emit every leaf piece: proto = seed = class (P2 = 0, P1 = 1,
        // P3 = 2). No pairing step — A3 children are not forced
        // edge-to-edge (T-junctions are intrinsic to the rule frames).
        let mut tiles = Vec::new();
        for t in &pieces {
            let world = t.poly().transform(&map);
            if world.aabb().intersects(query.region) {
                tiles.push(Tile { family: tess_core::tiling::FamilyTag::None, reflected: false,
                    proto: class_of(t.st),
                    seed: class_of(t.st) as u32, level: steps as u8,
                    poly: world,
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
// Standard trio + float substitution check (mirrors ammann_a4.rs)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::geometry::BBox;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn ammann_a3_covers_region() {
        check_covers(
            &AmmannA3Tiling,
            "ammann-a3",
            BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
        );
    }

    #[test]
    fn ammann_a3_rotated_scaled() {
        check_rotated(
            &AmmannA3Tiling,
            "ammann-a3(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn ammann_a3_deterministic() {
        check_det(
            &AmmannA3Tiling,
            "ammann-a3",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }

    /// Deterministic xorshift64 rng (same style as lib.rs).
    struct Rng(u64);
    impl Rng {
        fn f64(&mut self) -> f64 {
            let mut v = self.0;
            v ^= v << 13;
            v ^= v >> 7;
            v ^= v << 17;
            self.0 = v;
            (v >> 11) as f64 * (1.0 / 9007199254740992.0)
        }
    }

    /// Point in polygon (even-odd, boundary-inclusive) via Polygon::contains.
    fn point_in_piece(t: &Piece, q: Vec2) -> bool {
        t.poly().contains(q)
    }

    /// The children of `t` must exactly tile `t` itself (deflation view):
    /// every sampled point inside t lies in exactly one child, and the
    /// child areas sum to the parent's area.
    fn check_substitution(st: u8, label: &str) {
        let parent = Piece {
            st,
            p: Vec2::ZERO,
            s: PHI,
        };
        let kids = parent.children();
        let pa = parent.poly().area();
        let kids_area: f64 = kids.iter().map(|k| k.poly().area()).sum();
        assert!(
            (kids_area - pa).abs() <= 1e-9 * pa,
            "{label}: child areas {kids_area} != parent {pa}"
        );

        let bb = parent.aabb();
        let mut rng = Rng(0x517C_C1B7_2722_0A95);
        let mut checked = 0;
        for _ in 0..4000 {
            let q = Vec2::new(
                bb.min.x + rng.f64() * bb.size().x,
                bb.min.y + rng.f64() * bb.size().y,
            );
            if !point_in_piece(&parent, q) {
                continue;
            }
            let n = kids.iter().filter(|k| point_in_piece(k, q)).count();
            assert!(
                n == 1,
                "{label}: point ({}, {}) covered by {} children",
                q.x,
                q.y,
                n
            );
            checked += 1;
        }
        assert!(
            checked > 1500,
            "{label}: too few samples landed ({checked})"
        );
    }

    #[test]
    fn each_state_float_substitution_tiles_parent() {
        let labels = [
            "rule_0 (P2 r0)",
            "rule_1 (P2 r90cw)",
            "rule_2 (P2 r180)",
            "rule_3 (P2 r90ccw)",
            "rule_4 (P1 r0)",
            "rule_5 (P1 r90cw)",
            "rule_6 (P1 r180)",
            "rule_7 (P1 r90ccw)",
            "rule_8 (P3 r0)",
            "rule_9 (P3 r90cw)",
            "rule_10 (P3 r180)",
            "rule_11 (P3 r90ccw)",
        ];
        for (st, label) in (0..12u8).zip(labels.iter()) {
            check_substitution(st, label);
        }
    }
}

// ---------------------------------------------------------------------------
// Exact Z[φ] battery (ports of derive_a3.py + ppm_exact.py; the ring sign
// port mirrors qphi.py sign_x5). All arithmetic exact in Z[φ]; floats only
// for display / ordering sanity.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod a3_exact {
    use super::*;

    // ---------- Z[φ] ring ----------
    type ZP = (i32, i32);
    /// A rect (x0, y0, x1, y1), Z[φ], non-degenerate, y-down.
    type Rect = (ZP, ZP, ZP, ZP);

    fn zadd(x: ZP, y: ZP) -> ZP {
        (x.0 + y.0, x.1 + y.1)
    }
    fn zsub(x: ZP, y: ZP) -> ZP {
        (x.0 - y.0, x.1 - y.1)
    }
    fn zmul(x: ZP, y: ZP) -> ZP {
        let (a1, b1) = x;
        let (a2, b2) = y;
        (
            (a1 as i64 * a2 as i64 + b1 as i64 * b2 as i64) as i32,
            (a1 as i64 * b2 as i64 + a2 as i64 * b1 as i64 + b1 as i64 * b2 as i64) as i32,
        )
    }
    fn zeq(x: ZP, y: ZP) -> bool {
        x.0 == y.0 && x.1 == y.1
    }
    fn zmul_phi(x: ZP) -> ZP {
        let (a, b) = x;
        (b, a + b)
    }
    fn zdiv_phi2(x: ZP) -> ZP {
        let (a, b) = x;
        (b - a, a)
    }
    fn zmuli(x: ZP, n: i32) -> ZP {
        (x.0 * n, x.1 * n)
    }
    /// Sign of x + y·√5: 0 / +1 / −1 (port of qphi.py sign_x5).
    fn sign_x5(x: i64, y: i64) -> i32 {
        if y == 0 {
            return (x > 0) as i32 - (x < 0) as i32;
        }
        if x == 0 {
            return (y > 0) as i32 - (y < 0) as i32;
        }
        if y > 0 {
            if x >= 0 {
                1
            } else if x * x > 5 * y * y {
                -1
            } else {
                1
            }
        } else {
            if x <= 0 {
                -1
            } else if x * x < 5 * y * y {
                -1
            } else {
                1
            }
        }
    }
    /// Sign of a Z[φ] value: zsign((a, b)) = sign of (2a + b) + b·√5.
    fn zsign(x: ZP) -> i32 {
        let (a, b) = x;
        sign_x5(2 * a as i64 + b as i64, b as i64)
    }
    fn zlt(x: ZP, y: ZP) -> bool {
        zsign(zsub(x, y)) < 0
    }
    fn zle(x: ZP, y: ZP) -> bool {
        zsign(zsub(x, y)) <= 0
    }
    fn zmin(x: ZP, y: ZP) -> ZP {
        if zle(x, y) {
            x
        } else {
            y
        }
    }
    fn zmax(x: ZP, y: ZP) -> ZP {
        if zle(y, x) {
            x
        } else {
            y
        }
    }
    fn zabs(x: ZP) -> ZP {
        if zsign(x) >= 0 {
            x
        } else {
            (-x.0, -x.1)
        }
    }
    fn zval(x: ZP) -> f64 {
        x.0 as f64 + x.1 as f64 * PHI
    }
    fn zord(a: ZP, b: ZP) -> std::cmp::Ordering {
        match zsign(zsub(a, b)) {
            0 => std::cmp::Ordering::Equal,
            1 => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Less,
        }
    }
    /// Sort and dedupe Z[φ] values; assert all distinct and float-gap-
    /// separated (the float key is safe iff the values are > 1e-9 apart).
    fn zsort(xs: Vec<ZP>) -> Vec<ZP> {
        let mut out = xs;
        out.sort_by(|a, b| zval(*a).partial_cmp(&zval(*b)).unwrap());
        out.dedup();
        for i in 0..out.len().saturating_sub(1) {
            let (a, b) = (out[i], out[i + 1]);
            assert!(
                zsign(zsub(b, a)) > 0 && (zval(b) - zval(a)) > 1e-9,
                "zsort collision at {a:?} / {b:?}"
            );
        }
        out
    }
    /// Canonical multiset of (state, x, y) children, insertion-order
    /// independent.
    fn zmultiset(pairs: &[(u8, ZP, ZP)]) -> Vec<(u8, ZP, ZP, u32)> {
        let mut out: Vec<(u8, ZP, ZP, u32)> = Vec::new();
        for &p in pairs {
            match out
                .iter_mut()
                .find(|q| q.0 == p.0 && q.1 == p.1 && q.2 == p.2)
            {
                Some(slot) => slot.3 += 1,
                None => out.push((p.0, p.1, p.2, 1)),
            }
        }
        out.sort_by(|q, r| {
            q.0.cmp(&r.0)
                .then_with(|| zord(q.1, r.1))
                .then_with(|| zord(q.2, r.2))
        });
        out
    }

    // ---------- constants (certified, Z[φ]) ----------
    const ZERO: ZP = (0, 0);
    const ONE: ZP = (1, 0);
    const PHI2: ZP = (1, 1); // φ² = φ + 1
    const W1: ZP = (15, 23);
    const H1: ZP = (8, 15);
    const W2: ZP = (16, 30);
    const H2: ZP = (15, 23);
    const W3: ZP = (15, 23);
    const H3: ZP = (15, 23);
    const A1: ZP = (402, 641);
    const A2: ZP = (641, 1043);
    const A3: ZP = (478, 804);
    const AA: ZP = (7, 8); // a
    const BB: ZP = (1, 7); // b
    const KK: ZP = (6, 1); // k
    const CC: ZP = (8, 15); // c = φa
    const DD: ZP = (7, 8); // d = a
    const EE: ZP = (1, 7); // e = b
    const FF: ZP = (8, 15); // f = H1
    const GG: ZP = (7, 8); // g = a
    const HH: ZP = (7, 8); // h = a
    const II: ZP = (1, 7); // i = b
    const JJ: ZP = (8, 15); // j = H1
    const LL: ZP = (7, 8); // l = a

    // ---------- state / class geometry ----------
    fn class_size(cls: u8) -> (ZP, ZP) {
        match cls {
            0 => (W2, H2),
            1 => (W1, H1),
            _ => (W3, H3),
        }
    }
    fn class_area(cls: u8) -> ZP {
        match cls {
            0 => A2,
            1 => A1,
            _ => A3,
        }
    }
    fn rot_bbox((w, h): (ZP, ZP), rot: u8) -> (ZP, ZP) {
        if rot == 0 || rot == 2 {
            (w, h)
        } else {
            (h, w)
        }
    }
    /// Rotate a point (x, y) within its (W × H) bbox, y-down.
    fn pt_rot(x: ZP, y: ZP, w: ZP, h: ZP, rot: u8) -> (ZP, ZP) {
        match rot {
            0 => (x, y),
            1 => (zsub(h, y), x),
            2 => (zsub(w, x), zsub(h, y)),
            _ => (y, zsub(w, x)),
        }
    }
    /// The base (r0) outline of a class = the shipped SHAPE for state cls*4.
    fn base_cycle(cls: u8) -> &'static [ZPt] {
        shape(cls * 4)
    }
    /// Rotation of a state: 0 = r0, 1 = r90cw, 2 = r180, 3 = r90ccw.
    #[inline]
    fn rot_of(st: u8) -> u8 {
        st % 4
    }
    /// rot90 state: (st/4)*4 + (st%4+1)%4.
    fn rot90_state(st: u8) -> u8 {
        (st / 4) * 4 + (st % 4 + 1) % 4
    }
    /// The φ-inflated frame of a parent state: (φW, φH).
    fn frame_of(st: u8) -> (ZP, ZP) {
        let (w, h) = class_size(class_of(st));
        (zmul_phi(w), zmul_phi(h))
    }
    /// The φ-scaled r0 outline of a class (the level-1 frame boundary).
    fn phi_cycle(cls: u8) -> Vec<(ZP, ZP)> {
        base_cycle(cls)
            .iter()
            .map(|&(x, y)| (zmul_phi(x), zmul_phi(y)))
            .collect()
    }

    // ---------- rect machinery ----------
    fn box_of(w: ZP, h: ZP) -> Rect {
        (ZERO, ZERO, w, h)
    }
    fn notch_tl(_w: ZP, _h: ZP, nw: ZP, d: ZP) -> Rect {
        (ZERO, ZERO, nw, d)
    }
    fn notch_tr(w: ZP, _h: ZP, nw: ZP, d: ZP) -> Rect {
        (zsub(w, nw), ZERO, w, d)
    }
    fn notch_bl(_w: ZP, h: ZP, nw: ZP, d: ZP) -> Rect {
        (ZERO, zsub(h, d), nw, h)
    }
    /// r minus a corner notch n, as up to 4 non-degenerate rects
    /// (provably tiles r \ n exactly).
    fn rect_minus(r: Rect, n: Rect) -> Vec<Rect> {
        let (x0, y0, x1, y1) = r;
        let (nx0, ny0, nx1, ny1) = n;
        let mut out = Vec::new();
        let lx1 = zmin(nx0, x1);
        if zlt(x0, lx1) {
            out.push((x0, y0, lx1, y1));
        }
        let rx0 = zmax(nx1, x0);
        if zlt(rx0, x1) {
            out.push((rx0, y0, x1, y1));
        }
        let tx0 = zmax(nx0, x0);
        let tx1 = zmin(nx1, x1);
        let ty1 = zmin(ny0, y1);
        if zlt(tx0, tx1) && zlt(y0, ty1) {
            out.push((tx0, y0, tx1, ty1));
        }
        let by0 = zmax(ny1, y0);
        if zlt(tx0, tx1) && zlt(by0, y1) {
            out.push((tx0, by0, tx1, y1));
        }
        out
    }
    /// The base (r0) rect list for a class = box minus its corner notches.
    fn base_rects(cls: u8) -> Vec<Rect> {
        let (w, h) = class_size(cls);
        let notches: Vec<Rect> = match cls {
            0 => vec![notch_tl(w, h, CC, DD), notch_tr(w, h, EE, FF)], // P2
            1 => vec![notch_tl(w, h, AA, BB)],                         // P1
            _ => vec![
                notch_tl(w, h, GG, HH),
                notch_tr(w, h, II, JJ),
                notch_bl(w, h, KK, LL),
            ], // P3
        };
        let mut out = vec![box_of(w, h)];
        for n in notches {
            let mut next = Vec::new();
            for r in out {
                next.extend(rect_minus(r, n));
            }
            out = next;
        }
        out
    }
    /// Rotate a rect within its (W × H) bbox, y-down (port of rot_rect).
    fn rot_rect(r: Rect, w: ZP, h: ZP, rot: u8) -> Rect {
        let (x0, y0, x1, y1) = r;
        match rot {
            0 => (x0, y0, x1, y1),
            1 => (zsub(h, y1), x0, zsub(h, y0), x1),
            2 => (zsub(w, x1), zsub(h, y1), zsub(w, x0), zsub(h, y0)),
            _ => (y0, zsub(w, x1), y1, zsub(w, x0)),
        }
    }
    /// The φ-inflated parent's rect list, rotated to the parent's rotation.
    fn region_rects(st: u8) -> Vec<Rect> {
        let c = class_of(st);
        let r = rot_of(st);
        let (pw, ph) = frame_of(st);
        base_rects(c)
            .iter()
            .map(|&(x0, y0, x1, y1)| {
                let sr = (zmul_phi(x0), zmul_phi(y0), zmul_phi(x1), zmul_phi(y1));
                rot_rect(sr, pw, ph, r)
            })
            .collect()
    }
    /// A child's rect list (unit scale) placed at its anchor (the two
    /// Z[φ] anchor coordinates, as in `Child = (state, ax, ay)`).
    fn child_rects(cstate: u8, ax: ZP, ay: ZP) -> Vec<Rect> {
        let c = class_of(cstate);
        let rot = rot_of(cstate);
        let (w, h) = class_size(c);
        base_rects(c)
            .iter()
            .map(|&(x0, y0, x1, y1)| {
                let rr = rot_rect((x0, y0, x1, y1), w, h, rot);
                (
                    zadd(rr.0, ax),
                    zadd(rr.1, ay),
                    zadd(rr.2, ax),
                    zadd(rr.3, ay),
                )
            })
            .collect()
    }

    // ---------- arrangement (grid) coverage audit ----------
    fn covers(r: &Rect, c2x: ZP, c2y: ZP) -> bool {
        zle(zadd(r.0, r.0), c2x)
            && zlt(c2x, zadd(r.2, r.2))
            && zle(zadd(r.1, r.1), c2y)
            && zlt(c2y, zadd(r.3, r.3))
    }
    /// For every arrangement cell of (region ∪ children) corners, the count
    /// of children covering the cell centre must equal the count of region
    /// rects covering it. Returns (bad cells, ncells).
    fn arrangement_check(
        region: &[Rect],
        children: &[Rect],
    ) -> (Vec<(ZP, ZP, ZP, ZP, usize, usize)>, usize) {
        let mut xs: Vec<ZP> = Vec::new();
        let mut ys: Vec<ZP> = Vec::new();
        for r in region {
            xs.push(r.0);
            xs.push(r.2);
            ys.push(r.1);
            ys.push(r.3);
        }
        for ch in children {
            xs.push(ch.0);
            xs.push(ch.2);
            ys.push(ch.1);
            ys.push(ch.3);
        }
        let xs = zsort(xs);
        let ys = zsort(ys);
        for i in 0..xs.len().saturating_sub(1) {
            assert!(zlt(xs[i], xs[i + 1]), "degenerate x-slab");
        }
        for j in 0..ys.len().saturating_sub(1) {
            assert!(zlt(ys[j], ys[j + 1]), "degenerate y-slab");
        }
        let mut bad = Vec::new();
        let mut ncells = 0;
        for i in 0..xs.len().saturating_sub(1) {
            for j in 0..ys.len().saturating_sub(1) {
                let c2x = zadd(xs[i], xs[i + 1]);
                let c2y = zadd(ys[j], ys[j + 1]);
                ncells += 1;
                let npar = region.iter().filter(|r| covers(r, c2x, c2y)).count();
                let nch = children.iter().filter(|r| covers(r, c2x, c2y)).count();
                if nch != npar {
                    bad.push((xs[i], xs[i + 1], ys[j], ys[j + 1], nch, npar));
                }
            }
        }
        (bad, ncells)
    }

    // ---------- census machinery (port of derive_a3.py 191–285) ----------
    /// Axial edges of a cycle (asserts every edge is axis-parallel).
    fn cycle_edges(cyc: &[(ZP, ZP)]) -> Vec<((ZP, ZP), (ZP, ZP))> {
        let n = cyc.len();
        let mut out = Vec::new();
        for m in 0..n {
            let p0 = cyc[m];
            let p1 = cyc[(m + 1) % n];
            assert!(zeq(p0.0, p1.0) || zeq(p0.1, p1.1), "non-axial edge");
            out.push((p0, p1));
        }
        out
    }
    /// Edge key: (kind, c, lo, hi) with sorted ends; kind 0 = vertical
    /// (c = x), 1 = horizontal (c = y).
    fn edge_key(e: &((ZP, ZP), (ZP, ZP))) -> (u8, ZP, ZP, ZP) {
        let ((x0, y0), (x1, y1)) = *e;
        if zeq(x0, x1) {
            let (lo, hi) = if zle(y0, y1) { (y0, y1) } else { (y1, y0) };
            (0, x0, lo, hi)
        } else {
            let (lo, hi) = if zle(x0, x1) { (x0, x1) } else { (x1, x0) };
            (1, y0, lo, hi)
        }
    }
    fn edge_on_parent(key: (u8, ZP, ZP, ZP), pedges: &[((ZP, ZP), (ZP, ZP))]) -> bool {
        let (kind, c, lo, hi) = key;
        for pe in pedges {
            let k2 = edge_key(pe);
            if k2.0 == kind && zeq(k2.1, c) && zle(k2.2, lo) && zle(hi, k2.3) {
                return true;
            }
        }
        false
    }
    /// Distinct tile corners strictly interior to another tile's edge
    /// (the T-junction census), deduped and sorted by (val x, val y).
    fn tjunctions(tiles: &[Vec<(ZP, ZP)>]) -> Vec<(ZP, ZP)> {
        let edges: Vec<_> = tiles.iter().map(|cyc| cycle_edges(cyc)).collect();
        let mut out = Vec::new();
        for b in 0..tiles.len() {
            for &(cx, cy) in &tiles[b] {
                for a in 0..tiles.len() {
                    if a == b {
                        continue;
                    }
                    for &((x0, y0), (x1, y1)) in &edges[a] {
                        if zeq(x0, x1) {
                            let (lo, hi) = if zle(y0, y1) { (y0, y1) } else { (y1, y0) };
                            if zeq(cx, x0) && zlt(lo, cy) && zlt(cy, hi) {
                                out.push((cx, cy));
                            }
                        } else {
                            let (lo, hi) = if zle(x0, x1) { (x0, x1) } else { (x1, x0) };
                            if zeq(cy, y0) && zlt(lo, cx) && zlt(cx, hi) {
                                out.push((cx, cy));
                            }
                        }
                    }
                }
            }
        }
        let mut out = out;
        out.sort_by(|p, q| {
            zval(p.0)
                .partial_cmp(&zval(q.0))
                .unwrap()
                .then_with(|| zval(p.1).partial_cmp(&zval(q.1)).unwrap())
        });
        out.dedup();
        out
    }
    /// Split defects: per maximal sub-segment on each line, the owner count
    /// must be 1 (on the parent boundary) or 2 (interior). 0 = valid tiling.
    fn split_audit(tiles: &[Vec<(ZP, ZP)>], parent_cyc: &[(ZP, ZP)]) -> usize {
        let mut edges: Vec<((u8, ZP, ZP, ZP), usize)> = Vec::new();
        for (ti, cyc) in tiles.iter().enumerate() {
            for e in cycle_edges(cyc) {
                edges.push((edge_key(&e), ti));
            }
        }
        let parent_edges: Vec<(u8, ZP, ZP, ZP)> =
            cycle_edges(parent_cyc).iter().map(edge_key).collect();
        let mut lines: Vec<((u8, ZP), Vec<(ZP, ZP, usize)>)> = Vec::new();
        for &(seg, ti) in &edges {
            let key = (seg.0, seg.1);
            match lines.iter_mut().find(|(k, _)| *k == key) {
                Some((_, segs)) => segs.push((seg.2, seg.3, ti)),
                None => lines.push((key, vec![(seg.2, seg.3, ti)])),
            }
        }
        let mut defects = 0;
        for ((kind, c), segs) in &lines {
            let mut pts: Vec<ZP> = Vec::new();
            for &(lo, hi, _ti) in segs {
                pts.push(lo);
                pts.push(hi);
            }
            for &(seg2, _ti) in &edges {
                if seg2.0 != *kind && zle(seg2.2, *c) && zle(*c, seg2.3) {
                    pts.push(seg2.1);
                }
            }
            let sp = zsort(pts);
            for i in 0..sp.len().saturating_sub(1) {
                let s0 = sp[i];
                let s1 = sp[i + 1];
                let mut owners: Vec<usize> = segs
                    .iter()
                    .filter(|s| zle(s.0, s0) && zle(s1, s.1))
                    .map(|s| s.2)
                    .collect();
                owners.sort();
                owners.dedup();
                if owners.is_empty() {
                    continue;
                }
                let on_parent = parent_edges.iter().any(|(k2, c2, lo2, hi2)| {
                    *k2 == *kind && zeq(*c2, *c) && zle(*lo2, s0) && zle(s1, *hi2)
                });
                let expected = if on_parent { 1 } else { 2 };
                if owners.len() != expected {
                    defects += 1;
                }
            }
        }
        defects
    }
    /// Full-edge defects: an interior edge key owned by 1 tile is a
    /// T-junction measure (non-edge-to-edge); boundary keys expect 1.
    fn full_edge_audit(tiles: &[Vec<(ZP, ZP)>], parent_cyc: &[(ZP, ZP)]) -> usize {
        let pedges = cycle_edges(parent_cyc);
        let mut by_key: Vec<((u8, ZP, ZP, ZP), Vec<usize>)> = Vec::new();
        for (k, cyc) in tiles.iter().enumerate() {
            for e in cycle_edges(cyc) {
                let key = edge_key(&e);
                match by_key.iter_mut().find(|(key2, _)| *key2 == key) {
                    Some((_, owners)) => owners.push(k),
                    None => by_key.push((key, vec![k])),
                }
            }
        }
        let mut problems = 0;
        for (key, owners) in &by_key {
            let mut uniq = owners.clone();
            uniq.sort();
            uniq.dedup();
            if edge_on_parent(*key, &pedges) {
                if uniq.len() != 1 {
                    problems += 1;
                }
            } else if uniq.len() != 2 {
                problems += 1;
            }
        }
        problems
    }

    /// Level-2 tiles of a row: the grandchildren of the r0 parent, in the
    /// level-1 (φ·parent) frame. Grandchild vertex = div_phi(gx + rx) + ax
    /// (per coordinate), where (rx, ry) = pt_rot(base vertex, Wd, Hd,
    /// rot_of(gstate)). Returns (grandchild state, boundary cycle).
    fn l2_tiles(pst: u8) -> Vec<(u8, Vec<(ZP, ZP)>)> {
        let mut l2 = Vec::new();
        for &(cstate, ax, ay) in rule(pst) {
            for &(gstate, gx, gy) in rule(cstate) {
                let gc = class_of(gstate);
                let (wd, hd) = class_size(gc);
                let cyc = base_cycle(gc)
                    .iter()
                    .map(|&(x, y)| {
                        let (rx, ry) = pt_rot(x, y, wd, hd, rot_of(gstate));
                        (
                            zadd(zdiv_phi2(zadd(gx, rx)), ax),
                            zadd(zdiv_phi2(zadd(gy, ry)), ay),
                        )
                    })
                    .collect();
                l2.push((gstate, cyc));
            }
        }
        l2
    }

    /// Level-2 T-location pins (exact Z[φ], sorted by (val x, val y)).
    const TJ_S1: &[(ZP, ZP)] = &[
        ((8, 15), (7, 8)),
        ((9, 22), (8, 15)),
        ((15, 23), (7, 8)),
        ((15, 23), (8, 15)),
    ];
    const TJ_S2: &[(ZP, ZP)] = &[
        ((8, 15), (15, 23)),
        ((8, 15), (16, 30)),
        ((14, 16), (15, 23)),
        ((15, 23), (16, 30)),
        ((22, 31), (15, 23)),
    ];
    const TJ_S3: &[(ZP, ZP)] = &[
        ((1, 7), (15, 23)),
        ((7, 8), (15, 23)),
        ((8, 15), (16, 30)),
        ((15, 23), (15, 23)),
    ];

    fn tj_sorted(tj: &[(ZP, ZP)]) -> Vec<(ZP, ZP)> {
        let mut v: Vec<(ZP, ZP)> = tj.to_vec();
        v.sort_by(|p, q| {
            zval(p.0)
                .partial_cmp(&zval(q.0))
                .unwrap()
                .then_with(|| zval(p.1).partial_cmp(&zval(q.1)).unwrap())
        });
        v
    }

    // ---------- tests ----------
    const PHI4: ZP = (2, 3); // φ⁴ = 3φ + 2

    /// Per-class child count vector of a rule, reported in [P1, P2, P3]
    /// order (class 1, 0, 2).
    fn count_rule(st: u8) -> [usize; 3] {
        let mut c = [0usize; 3];
        for &(cs, _, _) in rule(st) {
            c[class_of(cs) as usize] += 1;
        }
        [c[1], c[0], c[2]]
    }
    /// Point order: x first, then y (both by exact Z[φ] value).
    fn pt_lt(a: (ZP, ZP), b: (ZP, ZP)) -> bool {
        match zord(a.0, b.0) {
            std::cmp::Ordering::Less => true,
            std::cmp::Ordering::Greater => false,
            std::cmp::Ordering::Equal => zord(a.1, b.1) == std::cmp::Ordering::Less,
        }
    }
    /// Rotation-normalized cycle key: the cycle re-anchored at its
    /// (min x, min y) vertex (unique for every state shape).
    fn poly_key(cyc: &[(ZP, ZP)]) -> Vec<(ZP, ZP)> {
        let n = cyc.len();
        let mut best = 0usize;
        for i in 1..n {
            if pt_lt(cyc[i], cyc[best]) {
                best = i;
            }
        }
        (0..n).map(|k| cyc[(best + k) % n]).collect()
    }

    #[test]
    fn ring_pins() {
        // φ identities
        assert_eq!(zmul(PHI2, PHI2), PHI4, "φ⁴ = 3φ + 2");
        assert_eq!(zmul_phi(W1), zadd(W1, H1), "φW1 = W1 + H1");
        assert_eq!(zdiv_phi2(zmul_phi(ONE)), ONE, "φ·(x/φ) = x");
        // certified dimensions
        assert_eq!(zdiv_phi2(W1), H1, "H1 = W1/φ");
        assert_eq!(zdiv_phi2(H1), AA, "a = W1/φ²");
        assert_eq!(zdiv_phi2(AA), BB, "b = W1/φ³");
        assert_eq!(zdiv_phi2(BB), KK, "k = W1/φ⁴");
        assert_eq!(zmuli(zdiv_phi2(W1), 2), W2, "W2 = 2W1/φ");
        assert_eq!(CC, zmul_phi(AA), "c = φa");
        assert_eq!(DD, AA);
        assert_eq!(EE, BB);
        assert_eq!(FF, H1);
        assert_eq!(GG, AA);
        assert_eq!(HH, AA);
        assert_eq!(II, BB);
        assert_eq!(JJ, H1);
        assert_eq!(LL, AA);
        assert_eq!(A2, zmul_phi(A1), "A2 = φA1");
        // zsign sanity (port of qphi.py sign_x5)
        assert_eq!(zsign((2, -1)), 1, "2 − φ > 0");
        assert_eq!(zsign((-2, 1)), -1, "−2 + φ < 0");
        assert_eq!(zsign((1, -1)), -1, "1 − φ < 0");
        assert_eq!(zsign((-1, 1)), 1, "−1 + φ > 0");
        assert_eq!(zsign((5, -6)), -1, "5 − 6φ < 0");
        assert_eq!(zsign((7, -3)), 1, "7 − 3φ > 0");
        assert_eq!(zsign((-7, 3)), -1, "−7 + 3φ < 0");
        assert_eq!(zsign(ZERO), 0);
        assert_eq!(zsign(ONE), 1);
        assert_eq!(zsign((-1, 0)), -1);
        assert_eq!(zsign((0, 1)), 1, "φ > 0");
        assert_eq!(zsign((0, -1)), -1);
        // rational norm via the ring conjugate: conj(a + bφ) = (a+b) − bφ
        // (φ' = 1 − φ), so N(x) = x · conj(x) ∈ Z ⊂ Z[φ]:
        // N(a+bφ) = a² + ab − b².
        let x = (3, -2); // 3 − 2φ
        let norm = zmul(x, (x.0 + x.1, -x.1));
        assert_eq!(norm.1, 0, "norm is rational");
        assert_eq!(norm.0, -1, "N(3 − 2φ) = 9 − 6 − 4 = −1");
    }

    #[test]
    fn state_shapes() {
        for st in 0..12u8 {
            let c = class_of(st);
            let cyc: Vec<(ZP, ZP)> = shape(st).iter().map(|&(x, y)| (x, y)).collect();
            // every edge axis-aligned (the axial assertion is inside)
            cycle_edges(&cyc);
            // exact area: |shoelace2| = 2 · class area
            let n = cyc.len();
            let mut s2 = ZERO;
            for i in 0..n {
                let (x0, y0) = cyc[i];
                let (x1, y1) = cyc[(i + 1) % n];
                s2 = zsub(zadd(s2, zmul(x0, y1)), zmul(x1, y0));
            }
            assert_eq!(
                zabs(s2),
                zadd(class_area(c), class_area(c)),
                "state {st}: |shoelace2| = 2 · A_{c}"
            );
            // bbox: (0,0) to the rotated class box
            let (bw, bh) = rot_bbox(class_size(c), rot_of(st));
            let (mut xmin, mut ymin) = (cyc[0].0, cyc[0].1);
            let (mut xmax, mut ymax) = (cyc[0].0, cyc[0].1);
            for &(x, y) in &cyc {
                xmin = zmin(xmin, x);
                xmax = zmax(xmax, x);
                ymin = zmin(ymin, y);
                ymax = zmax(ymax, y);
            }
            assert!(
                zeq(xmin, ZERO) && zeq(ymin, ZERO),
                "state {st}: bbox origin"
            );
            assert!(
                zeq(xmax, bw) && zeq(ymax, bh),
                "state {st}: bbox size (want {bw:?} x {bh:?})"
            );
            // base (r0) notch inner corners lie on the cycle
            if rot_of(st) == 0 {
                let want: Vec<(ZP, ZP)> = match c {
                    1 => vec![(AA, BB)],                     // P1: (a, b)
                    0 => vec![(CC, DD), (zsub(W2, EE), FF)], // P2: (c, d), (W2 − e, f)
                    _ => vec![
                        (AA, AA),           // P3: (a, a)
                        (zsub(W3, II), JJ), // (W3 − i, j)
                        (KK, zsub(H3, LL)), // (k, H3 − l)
                    ],
                };
                for w in want {
                    assert!(cyc.contains(&w), "state {st}: notch corner {w:?} missing");
                }
            }
        }
        // R90 closure: rotating state st's cycle 90° (cw) within its bbox
        // reproduces state rot90(st)'s vertex cycle up to start vertex.
        for st in 0..12u8 {
            let (bw, bh) = rot_bbox(class_size(class_of(st)), rot_of(st));
            let src: Vec<(ZP, ZP)> = shape(st).iter().map(|&(x, y)| (x, y)).collect();
            let rot: Vec<(ZP, ZP)> = src.iter().map(|&(x, y)| pt_rot(x, y, bw, bh, 1)).collect();
            let dst: Vec<(ZP, ZP)> = shape(rot90_state(st))
                .iter()
                .map(|&(x, y)| (x, y))
                .collect();
            assert_eq!(poly_key(&rot), poly_key(&dst), "state {st}: R90 closure");
        }
    }

    #[test]
    fn area_laws_and_matrix() {
        // Count matrix N (rows = parents P1, P2, P3; columns = children
        // P1, P2, P3) computed from the shipped base rules (S1 = rule 4,
        // S2 = rule 0, S3 = rule 8), pinned.
        let n: [[usize; 3]; 3] = [count_rule(4), count_rule(0), count_rule(8)];
        assert_eq!(n, [[1, 1, 0], [3, 0, 1], [2, 0, 1]], "count matrix N");
        // Area laws: N · A = φ² · A, exact in Z[φ].
        let areas = [A1, A2, A3];
        for (i, row) in n.iter().enumerate() {
            let sum = row.iter().enumerate().fold(ZERO, |acc, (j, &cnt)| {
                zadd(acc, zmuli(areas[j], cnt as i32))
            });
            assert_eq!(sum, zmul(PHI2, areas[i]), "area law row {i} (N·A = φ²A)");
        }
        assert_eq!(A2, zmul_phi(A1), "A2 = φA1");
    }

    #[test]
    fn per_rule_child_counts() {
        // Rotation preserves per-class child counts: every state's rule
        // carries the same counts as its class's r0 rule.
        for st in 0..12u8 {
            let c = class_of(st);
            assert_eq!(
                count_rule(st),
                count_rule(c * 4),
                "state {st}: class counts drift under rotation"
            );
        }
    }

    #[test]
    fn per_rule_r90_covariance() {
        // R90 covariance within each class orbit: rotating rule(st) 90°
        // (cw) in its current frame reproduces rule(rot90(st)). The frame
        // is (φW × φH) for even parent rotations and (φH × φW) for odd
        // ones, so the CURRENT frame height is φW exactly when the parent
        // rotation is odd. Each child's bbox height is taken at the
        // child's CURRENT rotation.
        for st in 0..12u8 {
            let (fw, fh) = frame_of(st);
            let ph = if rot_of(st) % 2 == 1 { fw } else { fh };
            let got: Vec<(u8, ZP, ZP)> = rule(st)
                .iter()
                .map(|&(cs, ax, ay)| {
                    let (_, hcp) = rot_bbox(class_size(class_of(cs)), rot_of(cs));
                    (rot90_state(cs), zsub(zsub(ph, ay), hcp), ax)
                })
                .collect();
            let expected = rule(rot90_state(st))
                .iter()
                .map(|&(cs, ax, ay)| (cs, ax, ay))
                .collect::<Vec<_>>();
            assert_eq!(
                zmultiset(&got),
                zmultiset(&expected),
                "state {st}: R90 covariance of rule(st) vs rule(rot90(st))"
            );
        }
    }

    #[test]
    fn per_rule_exact_coverage() {
        for st in 0..12u8 {
            let c = class_of(st);
            let r = rot_of(st);
            let (pw, ph) = frame_of(st);
            let region = region_rects(st);
            let mut kids: Vec<Rect> = Vec::new();
            let mut kid_cycs: Vec<Vec<(ZP, ZP)>> = Vec::new();
            let mut kid_area = ZERO;
            for &(cs, ax, ay) in rule(st) {
                kids.extend(child_rects(cs, ax, ay));
                kid_area = zadd(kid_area, class_area(class_of(cs)));
                let (w, h) = class_size(class_of(cs));
                let cyc = base_cycle(class_of(cs))
                    .iter()
                    .map(|&(x, y)| {
                        let (rx, ry) = pt_rot(x, y, w, h, rot_of(cs));
                        (zadd(rx, ax), zadd(ry, ay))
                    })
                    .collect();
                kid_cycs.push(cyc);
            }
            // (a) arrangement: the children tile the φ-inflated parent
            // exactly — every cell of the (region ∪ children) corner grid
            // has equal child and region coverage counts.
            let (bad, ncells) = arrangement_check(&region, &kids);
            assert!(ncells > 0, "state {st}: empty arrangement grid");
            assert!(
                bad.is_empty(),
                "state {st}: {} bad cells of {ncells} ({bad:?})",
                bad.len()
            );
            // (b) area law: Σ child areas = φ² · A_parent (exact)
            assert_eq!(
                kid_area,
                zmul(PHI2, class_area(c)),
                "state {st}: Σ child areas = φ² · A_parent"
            );
            // (c) split audit: the child partition is valid (0 defects)
            let parent_poly: Vec<(ZP, ZP)> = phi_cycle(c)
                .iter()
                .map(|&(x, y)| pt_rot(x, y, pw, ph, r))
                .collect();
            assert_eq!(
                split_audit(&kid_cycs, &parent_poly),
                0,
                "state {st}: split defects at level 1"
            );
        }
    }

    #[test]
    fn level2_tjunction_census_pin() {
        // (r0 parent state, tile-count pin, full-edge pin, T-location pin)
        let rows = [
            (4, 6, 9, TJ_S1),  // S1 = φ·P1
            (0, 9, 13, TJ_S2), // S2 = φ·P2
            (8, 7, 9, TJ_S3),  // S3 = φ·P3
        ];
        for (pst, n_pin, fe_pin, tj_pin) in rows {
            let l2 = l2_tiles(pst);
            assert_eq!(l2.len(), n_pin, "parent state {pst}: level-2 tile count");
            // φ⁴ · A_parent area law over the grandchildren
            let area: ZP = l2
                .iter()
                .fold(ZERO, |acc, (gs, _)| zadd(acc, class_area(class_of(*gs))));
            assert_eq!(
                area,
                zmul(PHI4, class_area(class_of(pst))),
                "parent state {pst}: Σ grandchild areas = φ⁴ · A_parent"
            );
            let cycs: Vec<Vec<(ZP, ZP)>> = l2.iter().map(|(_, cyc)| cyc.clone()).collect();
            let parent_poly = phi_cycle(class_of(pst));
            // the level-2 partition is valid (no gaps or overlaps)
            assert_eq!(
                split_audit(&cycs, &parent_poly),
                0,
                "parent state {pst}: level-2 split defects"
            );
            // the full-edge census (non-edge-to-edge measure)
            assert_eq!(
                full_edge_audit(&cycs, &parent_poly),
                fe_pin,
                "parent state {pst}: level-2 full-edge census"
            );
            // the distinct T-junction locations, exactly
            assert_eq!(
                tjunctions(&cycs),
                tj_sorted(tj_pin),
                "parent state {pst}: level-2 T-junction locations"
            );
        }
    }
}
