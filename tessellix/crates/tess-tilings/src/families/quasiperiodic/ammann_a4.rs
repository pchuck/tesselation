//! Ammann A4 (quasiperiodic) — rectilinear chair set, λ = 1+√2.
//!
//! Grünbaum & Shephard's A4 set (Tilings and Patterns, 1987): an aperiodic
//! tiling by a **chair** (a square of side S_c = 2+√2 with two opposite
//! unit corner notches) and a **half-chair** (HC) — a λ × S_c rectangle
//! (tall or wide orientation) with a single √2 × 1 corner notch. All
//! vertices lie in the ring Z[√2]:
//! every coordinate is m + n·√2 with integer m, n.
//!
//! The substitution is the canonical A4 **rectilinear** scheme with
//! inflation λ = 1+√2 (silver mean; same as Ammann-Beenker). PF count
//! matrix M = [[3, 4], [2, 3]] (chair, HC rows): M·[A1, A2] =
//! λ²·[A1, A2] with A1 = 4+4√2 (chair area), A2 = 2+√2 (HC area); the
//! Perron eigenvalue is λ² = 3+2√2 and the Perron ratio 1 : √2 agrees
//! with the patch census (Bielefeld A4 patch: N_chair ≈ 2014, N_HC ≈ 2798,
//! ratio 1.3891 vs the Perron prediction √2 ≈ 1.4142 — 1.78 % deviation,
//! finite-size, not a rule error).
//!
//! ```text
//! chair      -> 3 chairs + 4 half-chairs   (area: λ²A1 = 3A1 + 4A2)
//! half-chair -> 2 chairs + 3 half-chairs   (area: λ²A2 = 2A1 + 3A2)
//! ```
//!
//! The rule set is the **10-state achiral** one pinned by the exact Z[√2]
//! certificate (see `ammann_a34/` scratch; re-pin approved by the
//! orchestrator, supersedes the original λ = 2 brief wording):
//!
//! * chair states 0 = TL+BR notches, 1 = TR+BL notches (mutual R90 image);
//! * tall half-chairs (box λ × S_c = (1+√2) × (2+√2), notch √2×1):
//!     2 = TL, 4 = BR, 6 = TR, 8 = BL;
//! * wide half-chairs (box S_c × λ = (2+√2) × (1+√2), notch 1×√2):
//!     3 = TR, 5 = BL, 7 = BR, 9 = TL.
//!
//! State permutations under the parent's 180°/mirror/diag symmetries:
//!
//! ```text
//! R90ST  = [1, 0, 3, 4, 5, 2, 7, 8, 9, 6]   (order 4, R90⁴ = id)
//! MXST   = [1, 0, 6, 9, 8, 7, 2, 5, 4, 3]   (order 2, Mx² = id)
//! DIAGST = [0, 1, 9, 8, 7, 6, 5, 4, 3, 2]   (order 2, diag² = id)
//! ```
//!
//! The rules are covariance-closed under R90 and Mx (rule_1 = R90(rule_0),
//! rule_3 = R90(rule_2), rule_4 = R90(rule_3), rule_5 = R90(rule_4),
//! rule_6 = Mx(rule_2), rule_7 = R90(rule_6), rule_8 = R90(rule_7),
//! rule_9 = R90(rule_8)). Additionally rule_0 is invariant under the
//! main-diagonal mirror as a multiset (chair rule achirality), and the
//! probe in the in-module tests verifies per-state diag covariance for
//! all 10 states.
//!
//! **T-junctions are intrinsic** to the substitution: the rule frames
//! themselves carry them (8 per chair rule, 5 per HC rule; 56 at level 2
//! of the chair seed). A deflated A4 tiling is a valid gap-free
//! partition (exact cell coverage) but is NOT edge-to-edge. The T-junction
//! count is informational; the partition proof is the cell-coverage
//! check in the in-module tests.
//!
//! **Model-set / diffraction 8-fold claim: cited, not proven.** The A4
//! set is the canonical 8-fold quasiperiodic chair tiling in the
//! Grünbaum–Shephard classification, but the shipped constructor is the
//! substitution itself; the finite seed patch is only Klein-four
//! symmetric (R180 + main-diag + anti-diag, verified exactly in the
//! in-module tests). Diffraction is cited from the literature.
//!
//! Generation mirrors `ammann_beenker.rs`: seed with one chair state-0
//! at scale `s0`, body centre at the origin (inradius about body centre =
//! exactly 1.0·s0 — the nearest reflex vertex (1,1) sits at distance
//! (S_c/2 − 1)·√2 = 1). Grow while `reach > scale` (no √2 factor,
//! unlike AB's √2/2·scale inradius), guard against STEP_CAP overflow;
//! repeatedly deflate (each step replaces a piece by its 5–7 children at
//! 1/λ, which tile the parent exactly; culling is sound because a child
//! never leaves its parent). Emit each leaf piece; no pairing step is
//! needed (A4 has no glue/half-tile pairing, unlike AB).

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, MAX_TILES, SCALE_ID, SIMILARITY_PARAMS};

const SQRT_2: f64 = std::f64::consts::SQRT_2;
/// Silver-mean inflation factor λ = 1+√2 (same as Ammann-Beenker).
const LAM: f64 = 1.0 + SQRT_2;
/// Chair body side S_c = 2+√2 (Z[√2]: (2, 1)).
const SC: f64 = 2.0 + SQRT_2;
/// Chair area in cell units — the largest tile area: S_c² − 2 = 4 + 4√2.
/// Pre-budget uses this as a valid lower bound on tile count: every
/// emitted tile's area is at most s²·AREA_CHAIR.
const AREA_CHAIR: f64 = 4.0 + 4.0 * SQRT_2;
/// Refinement cap. λ^18 ≈ 7.8e6 units of seed reach. No pairing grid to
/// round against (unlike AB's 1e-7 hypotenuse pairing); beyond ~18
/// levels the accumulated float error would approach tile dimensions, so
/// such extreme zoom-outs fail with a message instead.
const STEP_CAP: usize = 18;

/// A Z[√2] scalar: (a, b) ⇔ a + b·√2.
type Z = (i32, i32);
/// A Z[√2] 2-point: (x, y) with each coordinate in Z.
type ZPt = (Z, Z);

/// The 10 state outlines in Z[√2] units of the seed chair's scale,
/// transcribed verbatim from the exact certificate (the
/// "SHAPE_N" dumps in the arc's scratch). Chairs (8 vertices, box
/// (2,1)²), tall half-chairs (6 vertices, box (1,1) × (2,1) =
/// (1+√2) × (2+√2)), wide half-chairs (6 vertices, box (2,1) × (1,1)
/// = (2+√2) × (1+√2)).
const SHAPE_0: [ZPt; 8] = [
    ((1, 0), (0, 0)),
    ((2, 1), (0, 0)),
    ((2, 1), (1, 1)),
    ((1, 1), (1, 1)),
    ((1, 1), (2, 1)),
    ((0, 0), (2, 1)),
    ((0, 0), (1, 0)),
    ((1, 0), (1, 0)),
];
const SHAPE_1: [ZPt; 8] = [
    ((0, 0), (0, 0)),
    ((1, 1), (0, 0)),
    ((1, 1), (1, 0)),
    ((2, 1), (1, 0)),
    ((2, 1), (2, 1)),
    ((1, 0), (2, 1)),
    ((1, 0), (1, 1)),
    ((0, 0), (1, 1)),
];
const SHAPE_2: [ZPt; 6] = [
    ((0, 1), (0, 0)),
    ((1, 1), (0, 0)),
    ((1, 1), (2, 1)),
    ((0, 0), (2, 1)),
    ((0, 0), (1, 0)),
    ((0, 1), (1, 0)),
];
const SHAPE_3: [ZPt; 6] = [
    ((0, 0), (0, 0)),
    ((1, 1), (0, 0)),
    ((1, 1), (0, 1)),
    ((2, 1), (0, 1)),
    ((2, 1), (1, 1)),
    ((0, 0), (1, 1)),
];
const SHAPE_4: [ZPt; 6] = [
    ((0, 0), (0, 0)),
    ((1, 1), (0, 0)),
    ((1, 1), (1, 1)),
    ((1, 0), (1, 1)),
    ((1, 0), (2, 1)),
    ((0, 0), (2, 1)),
];
const SHAPE_5: [ZPt; 6] = [
    ((0, 0), (0, 0)),
    ((2, 1), (0, 0)),
    ((2, 1), (1, 1)),
    ((1, 0), (1, 1)),
    ((1, 0), (1, 0)),
    ((0, 0), (1, 0)),
];
const SHAPE_6: [ZPt; 6] = [
    ((0, 0), (0, 0)),
    ((1, 0), (0, 0)),
    ((1, 0), (1, 0)),
    ((1, 1), (1, 0)),
    ((1, 1), (2, 1)),
    ((0, 0), (2, 1)),
];
const SHAPE_7: [ZPt; 6] = [
    ((0, 0), (0, 0)),
    ((2, 1), (0, 0)),
    ((2, 1), (1, 0)),
    ((1, 1), (1, 0)),
    ((1, 1), (1, 1)),
    ((0, 0), (1, 1)),
];
const SHAPE_8: [ZPt; 6] = [
    ((0, 0), (0, 0)),
    ((1, 1), (0, 0)),
    ((1, 1), (2, 1)),
    ((0, 1), (2, 1)),
    ((0, 1), (1, 1)),
    ((0, 0), (1, 1)),
];
const SHAPE_9: [ZPt; 6] = [
    ((1, 0), (0, 0)),
    ((2, 1), (0, 0)),
    ((2, 1), (1, 1)),
    ((0, 0), (1, 1)),
    ((0, 0), (0, 1)),
    ((1, 0), (0, 1)),
];

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
        _ => &SHAPE_9,
    }
}

/// Embed a Z[√2] pair as a real number: (a, b) → a + b·√2.
#[inline]
fn q(a: i32, b: i32) -> f64 {
    a as f64 + b as f64 * SQRT_2
}

/// Divide a Z[√2] coordinate by λ = 1+√2 — exact on the ring.
/// (a, b)/λ = (2b − a, a − b)  (multiply by the conjugate 1 − √2, /−1).
#[inline]
fn zdiv_lam(x: (i32, i32)) -> (i32, i32) {
    let (a, b) = x;
    (2 * b - a, a - b)
}

/// One substitution child: `(state, anchor x, anchor y)` — the child
/// body's top-left corner in the parent body's Z[√2] units.
type Child = (u8, (i32, i32), (i32, i32));

/// rule_0: state 0 (chair TL+BR) -> 3 chairs + 4 HC. Anchors in the
/// λ-inflated parent frame (box (4,3)² in Z[√2]).
const RULE_0: [Child; 7] = [
    (0, (2, 2), (0, 0)),
    (1, (1, 1), (1, 1)),
    (0, (0, 0), (2, 2)),
    (7, (1, 1), (0, 0)),
    (4, (0, 0), (1, 1)),
    (2, (3, 2), (1, 1)),
    (9, (1, 1), (3, 2)),
];
/// rule_1: state 1 (chair TR+BL) = R90(rule_0) = Mx(rule_0).
const RULE_1: [Child; 7] = [
    (1, (2, 2), (2, 2)),
    (0, (1, 1), (1, 1)),
    (1, (0, 0), (0, 0)),
    (8, (3, 2), (1, 1)),
    (5, (1, 1), (0, 0)),
    (3, (1, 1), (3, 2)),
    (6, (0, 0), (1, 1)),
];
/// rule_2: state 2 (tall TL) -> 2 chairs + 3 HC.
const RULE_2: [Child; 5] = [
    (1, (1, 1), (1, 1)),
    (0, (0, 0), (2, 2)),
    (8, (2, 1), (0, 0)),
    (4, (0, 0), (1, 1)),
    (9, (1, 1), (3, 2)),
];
/// rule_3: state 3 (wide TR) = R90(rule_2).
const RULE_3: [Child; 5] = [
    (0, (1, 1), (1, 1)),
    (1, (0, 0), (0, 0)),
    (9, (2, 2), (2, 1)),
    (5, (1, 1), (0, 0)),
    (6, (0, 0), (1, 1)),
];
/// rule_4: state 4 (tall BR) = R90(rule_3).
const RULE_4: [Child; 5] = [
    (1, (0, 0), (1, 1)),
    (0, (1, 1), (0, 0)),
    (6, (0, 0), (2, 2)),
    (2, (2, 1), (1, 1)),
    (7, (0, 0), (0, 0)),
];
/// rule_5: state 5 (wide BL) = R90(rule_4).
const RULE_5: [Child; 5] = [
    (0, (1, 1), (0, 0)),
    (1, (2, 2), (1, 1)),
    (7, (0, 0), (0, 0)),
    (3, (1, 1), (2, 1)),
    (8, (3, 2), (0, 0)),
];
/// rule_6: state 6 (tall TR) = Mx(rule_2).
const RULE_6: [Child; 5] = [
    (0, (0, 0), (1, 1)),
    (1, (1, 1), (2, 2)),
    (4, (0, 0), (0, 0)),
    (8, (2, 1), (1, 1)),
    (3, (0, 0), (3, 2)),
];
/// rule_7: state 7 (wide BR) = R90(rule_6).
const RULE_7: [Child; 5] = [
    (1, (1, 1), (0, 0)),
    (0, (0, 0), (1, 1)),
    (5, (2, 2), (0, 0)),
    (9, (1, 1), (2, 1)),
    (4, (0, 0), (0, 0)),
];
/// rule_8: state 8 (tall BL) = R90(rule_7).
const RULE_8: [Child; 5] = [
    (0, (1, 1), (1, 1)),
    (1, (0, 0), (0, 0)),
    (2, (2, 1), (2, 2)),
    (6, (0, 0), (1, 1)),
    (5, (1, 1), (0, 0)),
];
/// rule_9: state 9 (wide TL) = R90(rule_8).
const RULE_9: [Child; 5] = [
    (1, (1, 1), (1, 1)),
    (0, (2, 2), (0, 0)),
    (3, (0, 0), (2, 1)),
    (7, (1, 1), (0, 0)),
    (2, (3, 2), (1, 1)),
];

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
        _ => &RULE_9,
    }
}

/// A piece: state, anchor position of the body top-left corner, and
/// edge scale (unit pieces have `s == 1`).
#[derive(Clone, Copy, Debug)]
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

    /// Substitution children at 1/λ of this piece's scale; they exactly
    /// tile this piece (deflation view). The child anchor lives in the
    /// λ-inflated parent frame as a Z[√2] point; dividing the frame down
    /// to the parent divides by λ = 1+√2 — exact on the ring.
    fn children(&self) -> Vec<Piece> {
        let s2 = self.s / LAM;
        rule(self.st)
            .iter()
            .map(|&(st, ax, ay)| {
                let (dx, dy) = zdiv_lam(ax);
                let (ex, ey) = zdiv_lam(ay);
                Piece {
                    st,
                    p: self.p + Vec2::new(q(dx, dy), q(ex, ey)) * self.s,
                    s: s2,
                }
            })
            .collect()
    }
}

/// The seed: one state-0 chair, body centre at the origin (body top-left
/// at −(S_c/2)·scale per axis). The inradius about the body centre is
/// exactly 1.0·scale — the nearest reflex vertex (1, 1) sits at
/// distance (S_c/2 − 1)·√2 = 1.
fn seed(scale: f64) -> Vec<Piece> {
    let half = 0.5 * SC * scale;
    vec![Piece {
        st: 0,
        p: Vec2::new(-half, -half),
        s: scale,
    }]
}

/// One deflation step: replace every piece by its substitution children
/// (scale 1/λ), which tile the parent exactly.
///
/// Culling is sound: a child never leaves its parent, so a final piece
/// intersecting `region` has an ancestor chain whose aabbs all intersect
/// `region` — nothing needed is ever dropped.
fn refine_step(pieces: &[Piece], region: &BBox) -> Vec<Piece> {
    let mut out = Vec::with_capacity(pieces.len() * 5);
    for t in pieces {
        for ch in t.children() {
            if ch.aabb().intersects(*region) {
                out.push(ch);
            }
        }
    }
    out
}

/// Ammann A4 tiling (aperiodic, rectilinear chair set, quasiperiodic
/// model set with 8-fold diffraction).
pub struct AmmannA4Tiling;

impl Tiling for AmmannA4Tiling {
    fn id(&self) -> &'static str {
        "ammann-a4"
    }

    fn name(&self) -> &'static str {
        "Ammann A4 (quasiperiodic)"
    }

    fn description(&self) -> &'static str {
        "Quasiperiodic rectilinear chair tiling by substitution; 8-fold symmetric model set."
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
            "aperiodic; 8-fold symmetric model set (rectilinear D4 tiles); \
             non-edge-to-edge — T-junctions intrinsic to the substitution rules",
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

        // Farthest region corner from the origin (the seed chair is
        // centred there).
        let mut reach: f64 = 0.0;
        for &corner in &region.corners() {
            reach = reach.max(corner.length());
        }
        if reach < 1e-9 {
            return fail("empty view");
        }

        // Pre-generation budget (mirrors lattice.rs): the returned tiles
        // must cover the query region and no A4 tile exceeds the chair's
        // area (cell units; tiles only get smaller than unit scale), so
        // this is a lower bound on the tile count — checked before any
        // refinement work.
        let s = params.float(SCALE_ID, 1.0);
        if query.region.area() / (s * s * AREA_CHAIR) > MAX_TILES as f64 {
            return fail("too many tiles for this view — zoom in");
        }

        // Seed scale: the chair's inradius about its body centre is
        // 1.0·scale, so pick scale = λ^n with n minimal subject to
        // reach ≤ scale; the refinement below then lands exactly on
        // unit pieces.
        let mut scale = 1.0_f64;
        let mut steps = 0_usize;
        while reach > scale && steps < STEP_CAP {
            scale *= LAM;
            steps += 1;
        }
        if reach > scale * (1.0 + 1e-9) {
            return fail("view too far from the seed — pan back or zoom in");
        }

        let mut pieces = seed(scale);
        for _ in 0..steps {
            pieces = refine_step(&pieces, &region);
        }
        if pieces.is_empty() {
            return fail("empty view");
        }

        // Emit every leaf piece: chairs (states 0/1) are one prototile
        // class, tall/wide half-chairs the other. No pairing step — A4
        // children meet edge-to-edge within each rule frame (T-junctions
        // are intrinsic to the rule frames, not a pairing requirement).
        let mut tiles = Vec::new();
        for t in &pieces {
            let world = t.poly().transform(&map);
            if world.aabb().intersects(query.region) {
                tiles.push(Tile {
                    proto: if t.st <= 1 { 0 } else { 1 },
                    seed: if t.st <= 1 { 0 } else { 1 },
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
// Standard trio (mirrors wis_12_fold.rs)
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
    fn ammann_a4_covers_region() {
        check_covers(
            &AmmannA4Tiling,
            "ammann-a4",
            BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
        );
    }

    #[test]
    fn ammann_a4_rotated_scaled() {
        check_rotated(
            &AmmannA4Tiling,
            "ammann-a4(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn ammann_a4_deterministic() {
        check_det(
            &AmmannA4Tiling,
            "ammann-a4",
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
            s: LAM,
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
            "rule_0 (chair TL+BR)",
            "rule_1 (chair TR+BL)",
            "rule_2 (tall TL)",
            "rule_3 (wide TR)",
            "rule_4 (tall BR)",
            "rule_5 (wide BL)",
            "rule_6 (tall TR)",
            "rule_7 (wide BR)",
            "rule_8 (tall BL)",
            "rule_9 (wide TL)",
        ];
        for (st, label) in (0..10u8).zip(labels.iter()) {
            check_substitution(st, label);
        }
    }
}

// ---------------------------------------------------------------------------
// Exact Z[√2] battery (ports of a4-rules-cert.py D1–D6 + the corrected
// level-2 patch symmetry probe from NOTES.md 2026-11-24)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod a4_exact {
    use super::*;

    // ---------- Z[√2] core ----------
    type ZP = (i32, i32);

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
            (a1 as i64 * a2 as i64 + 2 * b1 as i64 * b2 as i64) as i32,
            (a1 as i64 * b2 as i64 + a2 as i64 * b1 as i64) as i32,
        )
    }
    fn zeq(x: ZP, y: ZP) -> bool {
        x.0 == y.0 && x.1 == y.1
    }
    /// Sign of a Z[√2] value: 0 / +1 / −1.
    /// For same-sign (a, b): the sign is that sign.
    /// For opposite signs: a² vs 2b² (note the factor 2 on b² — the
    /// old a4-cert.py bug was to compare a² vs b²; the fixed form is
    /// a² vs 2b²).
    fn zsign(x: ZP) -> i32 {
        let (a, b) = x;
        if b == 0 {
            return (a > 0) as i32 - (a < 0) as i32;
        }
        if a == 0 {
            return (b > 0) as i32 - (b < 0) as i32;
        }
        let same = (a > 0) == (b > 0);
        if same {
            return if a > 0 { 1 } else { -1 };
        }
        let a2 = (a as i64) * (a as i64);
        let b2 = 2 * (b as i64) * (b as i64);
        if a2 == b2 {
            return 0;
        }
        if a2 > b2 {
            if a > 0 {
                1
            } else {
                -1
            }
        } else {
            if b > 0 {
                1
            } else {
                -1
            }
        }
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
    /// Exact total order on Z[√2] (by the sign of the difference); a valid
    /// comparator because √2 is irrational, so only (0,0) is zero.
    fn zord(a: ZP, b: ZP) -> std::cmp::Ordering {
        match zsign(zsub(a, b)) {
            0 => std::cmp::Ordering::Equal,
            1 => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Less,
        }
    }
    fn zval(x: ZP) -> f64 {
        x.0 as f64 + x.1 as f64 * SQRT_2
    }
    /// Total order on Z[√2] by real value (f64 is not `Ord`; the values
    /// used here are finite and well-separated, so `partial_cmp` is total).
    fn zcmp(a: &&ZP, b: &&ZP) -> std::cmp::Ordering {
        zval(**a)
            .partial_cmp(&zval(**b))
            .unwrap_or(std::cmp::Ordering::Equal)
    }
    /// Sort and dedupe Z[√2] values (the certificate built its grids from
    /// sets); assert all distinct and float-gap-separated (the float key is
    /// safe iff all values are > 1e-9 apart).
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
    /// Canonical multiset: group equal (state, anchor) pairs, then sort by
    /// state, then exact Z[√2] value of each anchor, so equality is
    /// insertion-order independent.
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

    // ---------- constants ----------
    const ZERO: ZP = (0, 0);
    const ONE: ZP = (1, 0);
    const S2: ZP = (0, 1); // √2
    const TWO: ZP = (2, 0);
    const LAMZ: ZP = (1, 1);
    // LAM2 = (3,2), LAM4 = (17,12), LAM6 = (99,70) — pinned.
    const LAM2: ZP = (3, 2);
    const LAM4: ZP = (17, 12);
    const LAM6: ZP = (99, 70);
    const INV_LAM: ZP = (-1, 1); // −1 + √2 = 1/λ
    const SC_Z: ZP = (2, 1); // 2 + √2
    const W_Z: ZP = (1, 1); // 1 + √2
    const A1: ZP = (4, 4); // chair area
    const A2: ZP = (4, 2); // HC area

    /// D(x) = x/λ in Z[√2]: (a, b) -> (2b − a, a − b).
    fn d(x: ZP) -> ZP {
        (2 * x.1 - x.0, x.0 - x.1)
    }

    // ---------- state defs ----------
    // kind: 0 = chair, 1 = tall, 2 = wide
    #[derive(Clone, Debug)]
    struct StateDef {
        kind: u8,
        box_w: ZP,
        box_h: ZP,
        // notch rectangles (x0, x1, y0, y1) in Z[√2], unit = seed chair
        // scale. Chairs carry two notches; tall/wide carry one.
        notches: Vec<[ZP; 4]>,
    }

    fn notch_chair(c: u8) -> [ZP; 4] {
        // c: 0=TL, 1=TR, 2=BL, 3=BR
        match c {
            0 => [ZERO, ONE, ZERO, ONE],
            1 => [zsub(SC_Z, ONE), SC_Z, ZERO, ONE],
            2 => [ZERO, ONE, zsub(SC_Z, ONE), SC_Z],
            _ => [zsub(SC_Z, ONE), SC_Z, zsub(SC_Z, ONE), SC_Z],
        }
    }
    fn notch_tall(c: u8) -> [ZP; 4] {
        match c {
            0 => [ZERO, S2, ZERO, ONE],
            1 => [zsub(W_Z, S2), W_Z, ZERO, ONE],
            2 => [ZERO, S2, zsub(SC_Z, ONE), SC_Z],
            _ => [zsub(W_Z, S2), W_Z, zsub(SC_Z, ONE), SC_Z],
        }
    }
    fn notch_wide(c: u8) -> [ZP; 4] {
        // wide: box (S_c, λ) = ((2,1),(1,1)); notch 1×√2. Top notches sit
        // at y in [0,√2]; bottom notches at y in [1,λ] (the box height is
        // λ, so the bottom √2-strip is [λ−√2, λ] = [1, λ]).
        match c {
            // TL: x in [0,1], y in [0,√2]
            0 => [ZERO, ONE, ZERO, S2],
            // TR: x in [S_c−1, S_c], y in [0,√2]
            1 => [zsub(SC_Z, ONE), SC_Z, ZERO, S2],
            // BL: x in [0,1], y in [1,λ]
            2 => [ZERO, ONE, ONE, LAMZ],
            // BR: x in [S_c−1, S_c], y in [1,λ]
            _ => [zsub(SC_Z, ONE), SC_Z, ONE, LAMZ],
        }
    }

    fn state_defs() -> [StateDef; 10] {
        let sc2 = SC_Z;
        let tall_box = (W_Z, SC_Z);
        let wide_box = (SC_Z, W_Z);
        [
            // state 0: chair, TL + BR notches (exp_corner LT/RB)
            StateDef {
                kind: 0,
                box_w: sc2,
                box_h: sc2,
                notches: vec![notch_chair(0), notch_chair(3)],
            },
            // state 1: chair, TR + BL notches (exp_corner RT/LB)
            StateDef {
                kind: 0,
                box_w: sc2,
                box_h: sc2,
                notches: vec![notch_chair(1), notch_chair(2)],
            },
            StateDef {
                kind: 1,
                box_w: tall_box.0,
                box_h: tall_box.1,
                notches: vec![notch_tall(0)],
            },
            StateDef {
                kind: 2,
                box_w: wide_box.0,
                box_h: wide_box.1,
                notches: vec![notch_wide(1)],
            },
            StateDef {
                kind: 1,
                box_w: tall_box.0,
                box_h: tall_box.1,
                notches: vec![notch_tall(3)],
            },
            StateDef {
                kind: 2,
                box_w: wide_box.0,
                box_h: wide_box.1,
                notches: vec![notch_wide(2)],
            },
            StateDef {
                kind: 1,
                box_w: tall_box.0,
                box_h: tall_box.1,
                notches: vec![notch_tall(1)],
            },
            StateDef {
                kind: 2,
                box_w: wide_box.0,
                box_h: wide_box.1,
                notches: vec![notch_wide(3)],
            },
            StateDef {
                kind: 1,
                box_w: tall_box.0,
                box_h: tall_box.1,
                notches: vec![notch_tall(2)],
            },
            StateDef {
                kind: 2,
                box_w: wide_box.0,
                box_h: wide_box.1,
                notches: vec![notch_wide(0)],
            },
        ]
    }

    fn child_dim(st: u8) -> (ZP, ZP) {
        let defs = state_defs();
        (defs[st as usize].box_w, defs[st as usize].box_h)
    }

    /// R90 rule image (y-down; anchor TL; child dim w, h; parent box by = height):
    /// (R90ST[st], (by − ay − h, ax)).
    const R90ST: [u8; 10] = [1, 0, 3, 4, 5, 2, 7, 8, 9, 6];
    const MXST: [u8; 10] = [1, 0, 6, 9, 8, 7, 2, 5, 4, 3];
    const DIAGST: [u8; 10] = [0, 1, 9, 8, 7, 6, 5, 4, 3, 2];

    fn r90_rule(children: &[(u8, ZP, ZP)], box_h: ZP) -> Vec<(u8, ZP, ZP)> {
        children
            .iter()
            .map(|&(st, ax, ay)| {
                let (_, h) = child_dim(st);
                (R90ST[st as usize], zsub(zsub(box_h, ay), h), ax)
            })
            .collect()
    }
    fn mx_rule(children: &[(u8, ZP, ZP)], box_w: ZP) -> Vec<(u8, ZP, ZP)> {
        children
            .iter()
            .map(|&(st, ax, ay)| {
                let (w, _) = child_dim(st);
                (MXST[st as usize], zsub(zsub(box_w, ax), w), ay)
            })
            .collect()
    }
    fn diag_rule(children: &[(u8, ZP, ZP)]) -> Vec<(u8, ZP, ZP)> {
        children
            .iter()
            .map(|&(st, ax, ay)| (DIAGST[st as usize], ay, ax))
            .collect()
    }

    /// Rule boxes (the λ-inflated parent frame):
    /// 0: (4,3)², 2: (3,2)×(4,3)  (base)
    /// 1: r90 of 0 = (4,3)²
    /// 3: r90 of 2 = (4,3)×(3,2)
    /// 4: r90 of 3 = (3,2)×(4,3)
    /// 5: r90 of 4 = (4,3)×(3,2)
    /// 6: mx of 2 = (3,2)×(4,3)
    /// 7: r90 of 6 = (4,3)×(3,2)
    /// 8: r90 of 7 = (3,2)×(4,3)
    /// 9: r90 of 8 = (4,3)×(3,2)
    fn rule_box(st: u8) -> (ZP, ZP) {
        match st {
            0 | 1 => ((4, 3), (4, 3)),
            2 | 4 | 6 | 8 => ((3, 2), (4, 3)),
            _ => ((4, 3), (3, 2)),
        }
    }

    // ---------- tests ----------

    #[test]
    fn z_helpers_roundtrip() {
        // D identity: λ·D(x) == x for the key values.
        for x in [ZERO, ONE, S2, TWO, SC_Z, W_Z, (3, 2), (4, 3), (2, 2)] {
            assert!(zeq(zmul(LAMZ, d(x)), x), "D fails for {x:?} (λ·D(x) != x)");
        }
        // λ² = (3,2), λ⁴ = (17,12), λ⁶ = (99,70) — pinned.
        assert_eq!(LAM2, zmul(LAMZ, LAMZ));
        assert_eq!(LAM4, zmul(LAM2, LAM2));
        assert_eq!(LAM6, zmul(LAM4, LAM2));
        // S_c² = (6,4)
        assert_eq!(zmul(SC_Z, SC_Z), (6, 4));
        // chair area = S_c² − 2 = (4,4) = A1
        assert_eq!(zsub((6, 4), TWO), A1);
        // HC area = S_c·λ − √2 = (4,3) − (0,1) = (4,2) = A2 (the HC
        // box is S_c × λ; the notch is √2 × 1).
        assert_eq!(zsub(zmul(SC_Z, LAMZ), S2), A2);
        // λ·INV_LAM == 1
        assert_eq!(zmul(LAMZ, INV_LAM), ONE);
        // zsign sanity: −1+√2 = (−1, 1) > 0 (the fixed zsign — the old
        // a4-cert.py bug returned −1 here).
        assert_eq!(zsign((-1, 1)), 1);
        assert_eq!(zsign((-1, -1)), -1);
        assert_eq!(zsign((0, 0)), 0);
        assert_eq!(zsign((1, 0)), 1);
        assert_eq!(zsign((0, -1)), -1);
        // strict gap on the grid actually used in D3/D5 (spot check).
        let xs = zsort(vec![ZERO, ONE, S2, SC_Z, W_Z, LAM2, (4, 3), (10, 7)]);
        assert_eq!(xs.len(), 8);
        for i in 0..xs.len().saturating_sub(1) {
            assert!(zval(xs[i + 1]) - zval(xs[i]) > 1e-9);
        }
    }

    /// Port of the D1 state-shape battery: axis-aligned, area, bbox,
    /// notch corner + dims, R90/Mx closure via poly_key.
    fn poly_key(poly: Vec<(ZP, ZP)>) -> Vec<(ZP, ZP)> {
        // normalize orientation: reflections reverse winding, rotations
        // don't. Shoelace2 < 0 ⇒ reverse to CCW. Then rotate so the
        // min-(x, y) vertex leads.
        let mut p = poly;
        let a2 = shoelace2(&p);
        if zsign(a2) < 0 {
            p.reverse();
        }
        let mut idx = 0;
        for i in 1..p.len() {
            if zlt2(p[i], p[idx]) {
                idx = i;
            }
        }
        let n = p.len();
        (0..n).map(|k| p[(idx + k) % n]).collect()
    }

    /// Lexicographic Z[√2] min (by (zval(x), zval(y)) — x then y, each
    /// by its float value; safe because the grid values are
    /// well-separated).
    fn zlt2(x: (ZP, ZP), y: (ZP, ZP)) -> bool {
        if zsign(zsub(x.0, y.0)) != 0 {
            return zlt(x.0, y.0);
        }
        zlt(x.1, y.1)
    }

    fn shoelace2(poly: &[(ZP, ZP)]) -> ZP {
        let mut s = ZERO;
        let n = poly.len();
        for i in 0..n {
            let (x1, y1) = poly[i];
            let (x2, y2) = poly[(i + 1) % n];
            s = zadd(s, zsub(zmul(x1, y2), zmul(x2, y1)));
        }
        s
    }

    fn state_poly(st: u8) -> Vec<(ZP, ZP)> {
        // Rebuild from the SHAPE_N consts (already transcribed verbatim
        // from the cert); convert ZPt to (ZP, ZP).
        let s = shape(st);
        s.iter().map(|&(x, y)| (x, y)).collect()
    }

    /// Corner label of a notch (x0, x1, y0, y1) inside a (bx, by) box.
    /// "LT" = left-top, "RT" = right-top, "LB" = left-bottom,
    /// "RB" = right-bottom (y-down: top = y0, bottom = y1).
    fn corner_of(n: &[ZP; 4], bx: ZP, by: ZP) -> &'static str {
        let (x0, x1, y0, y1) = (n[0], n[1], n[2], n[3]);
        let l = zeq(x0, ZERO);
        let r = zeq(x1, bx);
        let t = zeq(y0, ZERO);
        let b = zeq(y1, by);
        assert!(
            l != r && t != b,
            "notch must touch exactly one x-side and one y-side"
        );
        if l && t {
            "LT"
        } else if r && t {
            "RT"
        } else if l && b {
            "LB"
        } else {
            "RB"
        }
    }

    fn poly_r90(poly: &[(ZP, ZP)], box_h: ZP) -> Vec<(ZP, ZP)> {
        poly.iter().map(|&(x, y)| (zsub(box_h, y), x)).collect()
    }
    fn poly_mx(poly: &[(ZP, ZP)], box_w: ZP) -> Vec<(ZP, ZP)> {
        poly.iter().map(|&(x, y)| (zsub(box_w, x), y)).collect()
    }
    fn poly_diag(poly: &[(ZP, ZP)]) -> Vec<(ZP, ZP)> {
        poly.iter().map(|&(x, y)| (y, x)).collect()
    }
    fn poly_scale(poly: &[(ZP, ZP)], f: ZP) -> Vec<(ZP, ZP)> {
        poly.iter()
            .map(|&(x, y)| (zmul(f, x), zmul(f, y)))
            .collect()
    }

    /// Port of D1: every state shape is axis-aligned, has the exact
    /// doubled area 2A1 (chair) or 2A2 (HC), bbox (0,0)–(box_w, box_h),
    /// the expected notch corner set + dims, and R90/Mx closure.
    #[test]
    fn state_shapes() {
        let defs = state_defs();
        for st in 0..10u8 {
            let def = &defs[st as usize];
            let p = state_poly(st);
            let (bx, by) = (def.box_w, def.box_h);
            // axis-aligned edges
            for i in 0..p.len() {
                let (x1, y1) = p[i];
                let (x2, y2) = p[(i + 1) % p.len()];
                assert!(
                    zeq(x1, x2) || zeq(y1, y2),
                    "state {st}: edge {i} not axis-aligned: ({x1:?},{y1:?}) -> ({x2:?},{y2:?})"
                );
            }
            // doubled area
            let area2 = zabs(shoelace2(&p));
            let exp_a2 = if def.kind == 0 {
                zmul(TWO, A1)
            } else {
                zmul(TWO, A2)
            };
            assert!(
                zeq(area2, exp_a2),
                "state {st}: doubled area {area2:?} != {exp_a2:?}"
            );
            // bbox
            let xs: Vec<ZP> = p.iter().map(|v| v.0).collect();
            let ys: Vec<ZP> = p.iter().map(|v| v.1).collect();
            let xmin = *xs.iter().min_by(zcmp).unwrap();
            let xmax = *xs.iter().max_by(zcmp).unwrap();
            let ymin = *ys.iter().min_by(zcmp).unwrap();
            let ymax = *ys.iter().max_by(zcmp).unwrap();
            assert!(zeq(xmin, ZERO), "state {st}: xmin {xmin:?} != 0");
            assert!(zeq(xmax, bx), "state {st}: xmax {xmax:?} != {bx:?}");
            assert!(zeq(ymin, ZERO), "state {st}: ymin {ymin:?} != 0");
            assert!(zeq(ymax, by), "state {st}: ymax {ymax:?} != {by:?}");
            // notch corner + dims
            let exp_corner: &[&str] = match (def.kind, st) {
                (0, 0) => &["LT", "RB"],
                (0, 1) => &["RT", "LB"],
                (1, 2) => &["LT"],
                (1, 4) => &["RB"],
                (1, 6) => &["RT"],
                (1, 8) => &["LB"],
                (2, 3) => &["RT"],
                (2, 5) => &["LB"],
                (2, 7) => &["RB"],
                (2, 9) => &["LT"],
                _ => unreachable!("unmapped state {st}"),
            };
            let exp_dim = match def.kind {
                0 => (ONE, ONE),
                1 => (S2, ONE),
                _ => (ONE, S2),
            };
            let mut corners: Vec<&str> = def.notches.iter().map(|n| corner_of(n, bx, by)).collect();
            corners.sort_unstable();
            let mut exp_corners: Vec<&str> = exp_corner.to_vec();
            exp_corners.sort_unstable();
            assert_eq!(corners, exp_corners, "state {st}: notch corners");
            for n in &def.notches {
                assert!(
                    zeq(zsub(n[1], n[0]), exp_dim.0) && zeq(zsub(n[3], n[2]), exp_dim.1),
                    "state {st}: notch dims {:?} != {:?}",
                    n,
                    exp_dim
                );
            }
            // R90 / Mx closure via poly_key
            let p_r90 = poly_r90(&p, by);
            let p_mx = poly_mx(&p, bx);
            let p_r90_k = poly_key(p_r90);
            let p_mx_k = poly_key(p_mx);
            let p_r90_target = poly_key(state_poly(R90ST[st as usize]));
            let p_mx_target = poly_key(state_poly(MXST[st as usize]));
            assert!(
                p_r90_k == p_r90_target,
                "state {st}: R90 image != state {} (poly_key mismatch)",
                R90ST[st as usize]
            );
            assert!(
                p_mx_k == p_mx_target,
                "state {st}: Mx image != state {} (poly_key mismatch)",
                MXST[st as usize]
            );
        }
        // chair states 0 and 1 are distinct
        assert_ne!(poly_key(state_poly(0)), poly_key(state_poly(1)));
    }

    /// Port of D2: diag invariance of the chair rule + per-state diag
    /// covariance (all 10 states, verified by the corrected probe on
    /// 2026-11-24 in NOTES.md — this test pins it for Rust).
    #[test]
    fn chair_rule_diag_invariant() {
        let r0: Vec<(u8, ZP, ZP)> = RULE_0.iter().cloned().collect();
        let d0 = diag_rule(&r0);
        assert_eq!(zmultiset(&d0), zmultiset(&r0), "diag(rule_0) != rule_0");
        // chair shapes 0 and 1 are diag-self
        let s0k = poly_key(state_poly(0));
        let s1k = poly_key(state_poly(1));
        assert_eq!(poly_key(poly_diag(&state_poly(0))), s0k);
        assert_eq!(poly_key(poly_diag(&state_poly(1))), s1k);
        // per-state diag covariance (probe, all 10 states PASS)
        for st in 0..10u8 {
            let r: Vec<(u8, ZP, ZP)> = rule(st).iter().cloned().collect();
            let target: Vec<(u8, ZP, ZP)> = rule(DIAGST[st as usize]).iter().cloned().collect();
            assert_eq!(
                zmultiset(&diag_rule(&r)),
                zmultiset(&target),
                "diag covariance fails at state {st}: diag(rule_{st}) != rule_{}",
                DIAGST[st as usize]
            );
        }
    }

    /// Port of D3: per-rule exact cell coverage, containment, area,
    /// counts, and supertile outline for all 10 rules.
    #[test]
    fn per_rule_exact_coverage() {
        // rect containment: big (x0, x1, y0, y1) contains small (x0, x1, y0, y1)
        fn rect_in(big: (ZP, ZP, ZP, ZP), small: (ZP, ZP, ZP, ZP)) -> bool {
            zle(big.0, small.0) && zle(small.1, big.1) && zle(big.2, small.2) && zle(small.3, big.3)
        }
        let defs = state_defs();
        for st_p in 0..10u8 {
            let (bx, by) = rule_box(st_p);
            let children: Vec<(u8, ZP, ZP)> = rule(st_p).iter().cloned().collect();
            let def_p = &defs[st_p as usize];
            let pa = if def_p.kind == 0 { A1 } else { A2 };
            // λ× parent shape
            let parent_poly = state_poly(st_p);
            let sup_poly = poly_scale(&parent_poly, LAMZ);
            // λ× notches
            let sup_notches: Vec<[ZP; 4]> = def_p
                .notches
                .iter()
                .map(|n| {
                    [
                        zmul(LAMZ, n[0]),
                        zmul(LAMZ, n[1]),
                        zmul(LAMZ, n[2]),
                        zmul(LAMZ, n[3]),
                    ]
                })
                .collect();
            // grid
            let mut xs: Vec<ZP> = vec![ZERO, bx];
            let mut ys: Vec<ZP> = vec![ZERO, by];
            for &(st_c, ax, ay) in &children {
                let (w, h) = child_dim(st_c);
                xs.push(ax);
                xs.push(zadd(ax, w));
                ys.push(ay);
                ys.push(zadd(ay, h));
                for n in &defs[st_c as usize].notches {
                    xs.push(zadd(ax, n[0]));
                    xs.push(zadd(ax, n[1]));
                    ys.push(zadd(ay, n[2]));
                    ys.push(zadd(ay, n[3]));
                }
            }
            for n in &sup_notches {
                xs.push(n[0]);
                xs.push(n[1]);
                ys.push(n[2]);
                ys.push(n[3]);
            }
            let gx = zsort(xs);
            let gy = zsort(ys);
            let mut bad: Vec<(ZP, ZP, ZP, ZP, i32)> = Vec::new();
            let mut ncells = 0;
            for i in 0..gx.len().saturating_sub(1) {
                for j in 0..gy.len().saturating_sub(1) {
                    let cell = (gx[i], gx[i + 1], gy[j], gy[j + 1]);
                    let in_notch = sup_notches.iter().any(|n| rect_in((n[0], n[1], n[2], n[3]), cell));
                    let mut cnt: i32 = 0;
                    for &(st_c, ax, ay) in &children {
                        let (w, h) = child_dim(st_c);
                        let body = (ax, zadd(ax, w), ay, zadd(ay, h));
                        if rect_in(body, cell) {
                            let covered = defs[st_c as usize].notches.iter().all(|n| {
                                !rect_in(
                                    (
                                        zadd(ax, n[0]),
                                        zadd(ax, n[1]),
                                        zadd(ay, n[2]),
                                        zadd(ay, n[3]),
                                    ),
                                    cell,
                                )
                            });
                            if covered {
                                cnt += 1;
                            }
                        }
                    }
                    ncells += 1;
                    if cnt != if in_notch { 0 } else { 1 } {
                        bad.push((cell.0, cell.1, cell.2, cell.3, cnt));
                    }
                }
            }
            assert!(
                bad.is_empty(),
                "rule_{st_p}: coverage bad cells (first 5): {bad:?} (total {ncells} cells)"
            );
            // containment: 0 ≤ anchor, anchor + dim ≤ box
            for &(st_c, ax, ay) in &children {
                let (w, h) = child_dim(st_c);
                assert!(
                    zle(ZERO, ax) && zle(zadd(ax, w), bx) && zle(ZERO, ay) && zle(zadd(ay, h), by),
                    "rule_{st_p}: child {st_c} anchor ({ax:?},{ay:?}) + dim ({w:?},{h:?}) not in box ({bx:?},{by:?})"
                );
            }
            // area identity: Σ child areas == λ²·A_parent == box − Σ λ²-notch
            // areas; per-child area: chair (st ≤ 1) = A1, HC = A2
            let mut sum_a = ZERO;
            for &(st_c, _, _) in &children {
                sum_a = zadd(sum_a, if st_c <= 1 { A1 } else { A2 });
            }
            let mut notch_a = ZERO;
            for n in &sup_notches {
                notch_a = zadd(notch_a, zmul(zsub(n[1], n[0]), zsub(n[3], n[2])));
            }
            let supfill_a = zsub(zmul(bx, by), notch_a);
            assert!(
                zeq(sum_a, zmul(LAM2, pa)),
                "rule_{st_p}: Σ child areas {sum_a:?} != λ²·A_parent {:?} (={:?})",
                if def_p.kind == 0 { A1 } else { A2 },
                zmul(LAM2, pa)
            );
            assert!(
                zeq(sum_a, supfill_a),
                "rule_{st_p}: Σ child areas {sum_a:?} != box − Σ λ²-notch {supfill_a:?}"
            );
            // counts: chair parent -> (3,4), HC parent -> (2,3)
            let nch = children.iter().filter(|&&(st_c, _, _)| st_c <= 1).count();
            let nhc = children.len() - nch;
            let exp = if def_p.kind == 0 { (3, 4) } else { (2, 3) };
            assert_eq!(
                (nch, nhc),
                exp,
                "rule_{st_p}: counts ({nch},{nhc}) != {exp:?}"
            );
            // supertile outline == λ× parent shape (by poly_key)
            let sp_xs: Vec<ZP> = sup_poly.iter().map(|v| v.0).collect();
            let sp_ys: Vec<ZP> = sup_poly.iter().map(|v| v.1).collect();
            let sp_xmin = *sp_xs.iter().min_by(zcmp).unwrap();
            let sp_xmax = *sp_xs.iter().max_by(zcmp).unwrap();
            let sp_ymin = *sp_ys.iter().min_by(zcmp).unwrap();
            let sp_ymax = *sp_ys.iter().max_by(zcmp).unwrap();
            assert!(zeq(sp_xmin, ZERO) && zeq(sp_xmax, bx) && zeq(sp_ymin, ZERO) && zeq(sp_ymax, by),
                    "rule_{st_p}: λ× parent shape bbox not (0,0)-(bx,by): ({sp_xmin:?},{sp_ymin:?})-({sp_xmax:?},{sp_ymax:?})");
            // outline: build the supertile polygon (box − λ× notches) at the
            // supertile dimensions, compare to λ× parent shape by poly_key.
            // The supertile polygon is the box with the sup-notches cut out;
            // the parent shape is (box / λ) with the unit notches cut out.
            // Build directly at sup dims:
            let sup_poly2 = build_supertile(def_p.kind, bx, by, &sup_notches);
            assert!(
                poly_key(sup_poly2) == poly_key(sup_poly),
                "rule_{st_p}: supertile outline != λ× parent shape"
            );
        }
    }

    /// Build the supertile polygon (box with corner notches cut out) at
    /// the supertile dimensions, for outline comparison against
    /// λ× parent shape.
    fn build_supertile(kind: u8, bx: ZP, by: ZP, sup_notches: &[[ZP; 4]]) -> Vec<(ZP, ZP)> {
        // For the chair supertile: 8 vertices, same shape as SHAPE_0/1
        // scaled to sup dims. For HC: 6 vertices.
        // Use the notch corners to decide TL/BR vs TR/BL (chair).
        if kind == 0 {
            // Two notches; identify which corner each is.
            let (n1, n2) = (sup_notches[0], sup_notches[1]);
            let c1 = corner_of(&n1, bx, by);
            let c2 = corner_of(&n2, bx, by);
            let nw = zsub(n1[1], n1[0]); // notch width (x1 − x0)
            let nh = zsub(n1[3], n1[2]); // notch height (y1 − y0)
            assert!(zeq(zsub(n2[1], n2[0]), nw) && zeq(zsub(n2[3], n2[2]), nh));
            let corners = [c1, c2];
            let (lt, rb) = if corners == ["LT", "RB"] || corners == ["RB", "LT"] {
                (true, true)
            } else {
                (false, false)
            };
            if lt && rb {
                // SHAPE_0 style: notches at LT and RB (TL+BR in y-down)
                vec![
                    (nw, ZERO),
                    (bx, ZERO),
                    (bx, zsub(by, nh)),
                    (zsub(bx, nw), zsub(by, nh)),
                    (zsub(bx, nw), by),
                    (ZERO, by),
                    (ZERO, nh),
                    (nw, nh),
                ]
            } else {
                // SHAPE_1 style: notches at RT and LB
                vec![
                    (ZERO, ZERO),
                    (zsub(bx, nw), ZERO),
                    (zsub(bx, nw), nh),
                    (bx, nh),
                    (bx, by),
                    (nw, by),
                    (nw, zsub(by, nh)),
                    (ZERO, zsub(by, nh)),
                ]
            }
        } else {
            // HC supertile: single notch at one corner.
            let n = sup_notches[0];
            let c = corner_of(&n, bx, by);
            let nw = zsub(n[1], n[0]);
            let nh = zsub(n[3], n[2]);
            match c {
                "LT" => vec![
                    (nw, ZERO),
                    (bx, ZERO),
                    (bx, by),
                    (ZERO, by),
                    (ZERO, nh),
                    (nw, nh),
                ],
                "RT" => vec![
                    (ZERO, ZERO),
                    (zsub(bx, nw), ZERO),
                    (zsub(bx, nw), nh),
                    (bx, nh),
                    (bx, by),
                    (ZERO, by),
                ],
                "RB" => vec![
                    (ZERO, ZERO),
                    (bx, ZERO),
                    (bx, zsub(by, nh)),
                    (zsub(bx, nw), zsub(by, nh)),
                    (zsub(bx, nw), by),
                    (ZERO, by),
                ],
                "LB" => vec![
                    (ZERO, ZERO),
                    (bx, ZERO),
                    (bx, by),
                    (nw, by),
                    (nw, zsub(by, nh)),
                    (ZERO, zsub(by, nh)),
                ],
                _ => unreachable!(),
            }
        }
    }

    /// Port of D3b (achirality) + D4 (covariance).
    #[test]
    fn achirality_and_covariance() {
        let r0: Vec<(u8, ZP, ZP)> = RULE_0.iter().cloned().collect();
        let (bw0, bh0) = rule_box(0);
        let r0_r90 = r90_rule(&r0, bh0);
        let r0_mx = mx_rule(&r0, bw0);
        // R90(rule_0) == Mx(rule_0) == rule_1
        assert_eq!(
            zmultiset(&r0_r90),
            zmultiset(&r0_mx),
            "R90(rule_0) != Mx(rule_0) — chair rule not achiral"
        );
        let r1: Vec<(u8, ZP, ZP)> = RULE_1.iter().cloned().collect();
        assert_eq!(zmultiset(&r0_r90), zmultiset(&r1), "R90(rule_0) != rule_1");
        assert_eq!(zmultiset(&r0_mx), zmultiset(&r1), "Mx(rule_0) != rule_1");
        // covariance: all 10 states
        for st in 0..10u8 {
            let (bw, bh) = rule_box(st);
            let r: Vec<(u8, ZP, ZP)> = rule(st).iter().cloned().collect();
            let r90_target: Vec<(u8, ZP, ZP)> = rule(R90ST[st as usize]).iter().cloned().collect();
            let mx_target: Vec<(u8, ZP, ZP)> = rule(MXST[st as usize]).iter().cloned().collect();
            assert_eq!(
                zmultiset(&r90_rule(&r, bh)),
                zmultiset(&r90_target),
                "R90 covariance fails at state {st}: != rule_{}",
                R90ST[st as usize]
            );
            assert_eq!(
                zmultiset(&mx_rule(&r, bw)),
                zmultiset(&mx_target),
                "Mx covariance fails at state {st}: != rule_{}",
                MXST[st as usize]
            );
        }
    }

    /// Refine a level of pieces (count-only, no culling; the cert's
    /// `refine()` drops the per-level 1/λ factor on the anchor so its
    /// positions are exact only at level 1 — from L2 on they are
    /// count-only placeholders).
    fn refine(pieces: &[(u8, ZP, ZP)]) -> Vec<(u8, ZP, ZP)> {
        let mut out = Vec::new();
        for &(st, px, py) in pieces {
            for &(st_c, ax, ay) in rule(st) {
                let (dx, dy) = d(ax);
                let (ex, ey) = d(ay);
                out.push((st_c, zadd(px, (dx, dy)), zadd(py, (ex, ey))));
            }
        }
        out
    }

    fn census(pieces: &[(u8, ZP, ZP)]) -> [u32; 10] {
        let mut c = [0u32; 10];
        for &(st, _, _) in pieces {
            c[st as usize] += 1;
        }
        c
    }

    /// Port of D5 census + D6 census (per-state tables and area
    /// identities for levels 1, 2, 3).
    #[test]
    fn census_levels_123() {
        let l0 = vec![(0u8, ZERO, ZERO)];
        let l1 = refine(&l0);
        let l2 = refine(&l1);
        let l3 = refine(&l2);

        // level 1: 3 chairs + 4 HC, total 7; per-state [2,1,1,0,1,0,0,1,0,1]
        let c1 = census(&l1);
        assert_eq!(
            c1,
            [2, 1, 1, 0, 1, 0, 0, 1, 0, 1],
            "level-1 per-state census"
        );
        assert_eq!(c1.iter().sum::<u32>(), 7, "level-1 total");
        assert_eq!(c1[0] + c1[1], 3, "level-1 chairs");

        // level 2: 17 chairs + 24 HC, total 41; per-state [9,8,4,2,4,2,2,4,2,4]
        let c2 = census(&l2);
        assert_eq!(
            c2,
            [9, 8, 4, 2, 4, 2, 2, 4, 2, 4],
            "level-2 per-state census"
        );
        assert_eq!(c2.iter().sum::<u32>(), 41, "level-2 total");
        assert_eq!(c2[0] + c2[1], 17, "level-2 chairs");

        // level 3: 99 chairs + 140 HC, total 239; per-state [50,49,19,16,19,16,16,19,16,19]
        let c3 = census(&l3);
        assert_eq!(
            c3,
            [50, 49, 19, 16, 19, 16, 16, 19, 16, 19],
            "level-3 per-state census"
        );
        assert_eq!(c3.iter().sum::<u32>(), 239, "level-3 total");
        assert_eq!(c3[0] + c3[1], 99, "level-3 chairs");

        // exact 2A sums (doubled area): d1 (56,40), d2 (328,232), d3 (1912,1352)
        let sum1: ZP = c1
            .iter()
            .enumerate()
            .map(|(st, &n)| {
                let a2 = if st <= 1 {
                    zmul(TWO, A1)
                } else {
                    zmul(TWO, A2)
                };
                (n as i32 * a2.0, n as i32 * a2.1)
            })
            .fold(ZERO, |acc, t| zadd(acc, t));
        assert!(zeq(sum1, zmul(LAM2, zmul(TWO, A1))), "d1 sum 2A");
        assert!(zeq(sum1, (56, 40)), "d1 sum 2A pinned value");

        let sum2: ZP = c2
            .iter()
            .enumerate()
            .map(|(st, &n)| {
                let a2 = if st <= 1 {
                    zmul(TWO, A1)
                } else {
                    zmul(TWO, A2)
                };
                (n as i32 * a2.0, n as i32 * a2.1)
            })
            .fold(ZERO, |acc, t| zadd(acc, t));
        assert!(zeq(sum2, zmul(LAM4, zmul(TWO, A1))), "d2 sum 2A");
        assert!(zeq(sum2, (328, 232)), "d2 sum 2A pinned value");

        let sum3: ZP = c3
            .iter()
            .enumerate()
            .map(|(st, &n)| {
                let a2 = if st <= 1 {
                    zmul(TWO, A1)
                } else {
                    zmul(TWO, A2)
                };
                (n as i32 * a2.0, n as i32 * a2.1)
            })
            .fold(ZERO, |acc, t| zadd(acc, t));
        assert!(zeq(sum3, zmul(LAM6, zmul(TWO, A1))), "d3 sum 2A");
        assert!(zeq(sum3, (1912, 1352)), "d3 sum 2A pinned value");
    }

    /// Port of D5 (level-2 exact cell coverage of the seed shape, 361
    /// cells) + edge parity by grid interval (288 intervals, interior 2
    /// / boundary 1) + T-junction count (56, informational).
    #[test]
    fn level2_partition() {
        fn rect_in(big: (ZP, ZP, ZP, ZP), small: (ZP, ZP, ZP, ZP)) -> bool {
            zle(big.0, small.0) && zle(small.1, big.1) && zle(big.2, small.2) && zle(small.3, big.3)
        }
        let defs = state_defs();
        // level-2 tiling: for each (st1, a1) in RULE0, for each (st2, a2)
        // in RULE_{st1}: q = λ·a1 + a2 (in λ²-scaled space)
        let mut tiles2: Vec<(u8, ZP, ZP)> = Vec::new();
        for (st1, ax1, ay1) in RULE_0 {
            for &(st2, ax2, ay2) in rule(st1) {
                tiles2.push((st2, zadd(zmul(LAMZ, ax1), ax2), zadd(zmul(LAMZ, ay1), ay2)));
            }
        }
        assert_eq!(tiles2.len(), 41, "level-2 tile count");

        // region = λ² × seed shape (chair state 0): box (10,7)², TL + BR
        // notches (3,2)²
        let reg2 = zmul(LAM2, SC_Z); // (10, 7)
        assert_eq!(reg2, (10, 7), "λ²·S_c");
        let reg2_n = zmul(LAM2, ONE); // (3, 2) = λ²
        let reg2_notches = [
            [ZERO, reg2_n, ZERO, reg2_n],
            [zsub(reg2, reg2_n), reg2, zsub(reg2, reg2_n), reg2],
        ];

        // tile bodies + absolute notches
        let mut tiles2_body: Vec<(u8, (ZP, ZP, ZP, ZP), Vec<[ZP; 4]>)> = Vec::new();
        for &(st, qx, qy) in &tiles2 {
            let (w, h) = child_dim(st);
            let body = (qx, zadd(qx, w), qy, zadd(qy, h));
            let nabs: Vec<[ZP; 4]> = defs[st as usize]
                .notches
                .iter()
                .map(|n| {
                    [
                        zadd(qx, n[0]),
                        zadd(qx, n[1]),
                        zadd(qy, n[2]),
                        zadd(qy, n[3]),
                    ]
                })
                .collect();
            tiles2_body.push((st, body, nabs));
        }

        // grid
        let mut xs: Vec<ZP> = vec![ZERO, reg2];
        let mut ys: Vec<ZP> = vec![ZERO, reg2];
        for (_, body, nabs) in &tiles2_body {
            xs.push(body.0);
            xs.push(body.1);
            ys.push(body.2);
            ys.push(body.3);
            for n in nabs {
                xs.push(n[0]);
                xs.push(n[1]);
                ys.push(n[2]);
                ys.push(n[3]);
            }
        }
        for n in &reg2_notches {
            xs.push(n[0]);
            xs.push(n[1]);
            ys.push(n[2]);
            ys.push(n[3]);
        }
        let gx = zsort(xs);
        let gy = zsort(ys);

        // cell coverage: every cell (between grid lines) is either in a
        // notch (cnt == 0) or covered by exactly one tile body (cnt == 1)
        let mut bad: Vec<(ZP, ZP, ZP, ZP, i32)> = Vec::new();
        let mut ncells = 0;
        for i in 0..gx.len().saturating_sub(1) {
            for j in 0..gy.len().saturating_sub(1) {
                let cell = (gx[i], gx[i + 1], gy[j], gy[j + 1]);
                let in_notch = reg2_notches.iter().any(|n| rect_in((n[0], n[1], n[2], n[3]), cell));
                let cnt = tiles2_body
                    .iter()
                    .filter(|&(_, body, nabs)| {
                        rect_in(*body, cell)
                            && nabs.iter().all(|n| !rect_in((n[0], n[1], n[2], n[3]), cell))
                    })
                    .count() as i32;
                ncells += 1;
                if cnt != if in_notch { 0 } else { 1 } {
                    bad.push((cell.0, cell.1, cell.2, cell.3, cnt));
                }
            }
        }
        assert!(
            bad.is_empty(),
            "level-2 cell coverage bad (first 5): {bad:?} (total {ncells} cells; cert pinned 361)"
        );

        // edge parity by grid interval: split every tile edge at the
        // grid lines; each interior interval owned by exactly 2 tiles,
        // each boundary interval by exactly 1.
        let xidx: std::collections::HashMap<ZP, usize> =
            gx.iter().enumerate().map(|(i, v)| (*v, i)).collect();
        let yidx: std::collections::HashMap<ZP, usize> =
            gy.iter().enumerate().map(|(i, v)| (*v, i)).collect();
        let all_polys: Vec<Vec<(ZP, ZP)>> = tiles2
            .iter()
            .map(|&(st, qx, qy)| {
                state_poly(st)
                    .iter()
                    .map(|&(x, y)| (zadd(qx, x), zadd(qy, y)))
                    .collect()
            })
            .collect();

        // intvl: (kind, coord, idx) -> set of tile indices
        let mut intvl: std::collections::HashMap<
            (u8, ZP, usize),
            std::collections::HashSet<usize>,
        > = std::collections::HashMap::new();
        for (idx, poly) in all_polys.iter().enumerate() {
            let n = poly.len();
            for i in 0..n {
                let (x1, y1) = poly[i];
                let (x2, y2) = poly[(i + 1) % n];
                if zeq(x1, x2) {
                    // vertical edge
                    let (lo, hi) = (zmin(y1, y2), zmax(y1, y2));
                    let j0 = yidx[&lo];
                    let j1 = yidx[&hi];
                    for j in j0..j1 {
                        intvl.entry((0, x1, j)).or_default().insert(idx);
                    }
                } else {
                    let (lo, hi) = (zmin(x1, x2), zmax(x1, x2));
                    let i0 = xidx[&lo];
                    let i1 = xidx[&hi];
                    for ik in i0..i1 {
                        intvl.entry((1, y1, ik)).or_default().insert(idx);
                    }
                }
            }
        }

        // boundary: λ²× seed shape (chair state 0), per-axis ranges
        let reg_poly = poly_scale(&state_poly(0), LAM2);
        let mut bvert: std::collections::HashMap<ZP, Vec<(ZP, ZP)>> =
            std::collections::HashMap::new();
        let mut bhoriz: std::collections::HashMap<ZP, Vec<(ZP, ZP)>> =
            std::collections::HashMap::new();
        for i in 0..reg_poly.len() {
            let (x1, y1) = reg_poly[i];
            let (x2, y2) = reg_poly[(i + 1) % reg_poly.len()];
            if zeq(x1, x2) {
                bvert
                    .entry(x1)
                    .or_default()
                    .push((zmin(y1, y2), zmax(y1, y2)));
            } else {
                bhoriz
                    .entry(y1)
                    .or_default()
                    .push((zmin(x1, x2), zmax(x1, x2)));
            }
        }
        fn intvl_on_boundary(
            c: ZP,
            j: usize,
            grid: &[ZP],
            tab: &std::collections::HashMap<ZP, Vec<(ZP, ZP)>>,
        ) -> bool {
            let lo = grid[j];
            let hi = grid[j + 1];
            tab.get(&c)
                .map(|ranges| ranges.iter().any(|&(r0, r1)| zle(r0, lo) && zle(hi, r1)))
                .unwrap_or(false)
        }
        let mut parity_bad: Vec<(u8, ZP, usize, usize)> = Vec::new();
        for (&(kind, c, j), owners) in &intvl {
            let on_b = if kind == 0 {
                intvl_on_boundary(c, j, &gy, &bvert)
            } else {
                intvl_on_boundary(c, j, &gx, &bhoriz)
            };
            let exp = if on_b { 1 } else { 2 };
            if owners.len() != exp {
                parity_bad.push((kind, c, j, owners.len()));
            }
        }
        assert!(
            parity_bad.is_empty(),
            "level-2 edge parity bad (first 5): {parity_bad:?} (total {} intervals; cert pinned 288)",
            intvl.len()
        );

        // T-junction count (informational — intrinsic to the substitution
        // rules; 56 at level 2).
        let mut tj: u32 = 0;
        for idx in 0..all_polys.len() {
            let n = all_polys[idx].len();
            for i in 0..n {
                let (x1, y1) = all_polys[idx][i];
                let (x2, y2) = all_polys[idx][(i + 1) % n];
                for (j, poly) in all_polys.iter().enumerate() {
                    if j == idx {
                        continue;
                    }
                    for &(vx, vy) in poly {
                        if zeq(x1, x2) {
                            if zeq(vx, x1) && zlt(zmin(y1, y2), vy) && zlt(vy, zmax(y1, y2)) {
                                tj += 1;
                            }
                        } else {
                            if zeq(vy, y1) && zlt(zmin(x1, x2), vx) && zlt(vx, zmax(x1, x2)) {
                                tj += 1;
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(
            tj, 56,
            "level-2 T-junction count (informational, pinned by cert)"
        );
    }

    /// The corrected level-2 patch symmetry probe (NOTES.md 2026-11-24):
    /// R180, diag, anti-diag INVARIANT; R90/Mx map the state-0 patch to
    /// the state-1 patch (NOT invariant under the state-0 multiset).
    #[test]
    fn level2_patch_symmetry() {
        // level-2 tiling in λ²-scaled space (same construction as
        // level2_partition).
        let mut tiles2: Vec<(u8, ZP, ZP)> = Vec::new();
        for (st1, ax1, ay1) in RULE_0 {
            for &(st2, ax2, ay2) in rule(st1) {
                tiles2.push((st2, zadd(zmul(LAMZ, ax1), ax2), zadd(zmul(LAMZ, ay1), ay2)));
            }
        }
        assert_eq!(tiles2.len(), 41);

        let c_reg = zmul(LAM2, SC_Z); // (10, 7) — the region side (square)
        assert_eq!(c_reg, (10, 7));

        // TL maps (Z-space, region side c_reg per axis, square region):
        //   r90:  (c_reg − qy − dh, qx)  [size swap]
        //   mx:   (c_reg − qx − dw, qy)  [no swap]
        //   r180: (c_reg − qx − dw, c_reg − qy − dh)
        //   diag: (qy, qx)               [size swap]
        //   anti: (c_reg − qy − dh, c_reg − qx − dw) [size swap]
        fn img_r90(q: (ZP, ZP), d: (ZP, ZP), c: ZP) -> ((ZP, ZP), (ZP, ZP)) {
            ((zsub(zsub(c, q.1), d.1), q.0), (d.1, d.0))
        }
        fn img_mx(q: (ZP, ZP), d: (ZP, ZP), c: ZP) -> ((ZP, ZP), (ZP, ZP)) {
            ((zsub(zsub(c, q.0), d.0), q.1), d)
        }
        fn img_r180(q: (ZP, ZP), d: (ZP, ZP), c: ZP) -> ((ZP, ZP), (ZP, ZP)) {
            ((zsub(zsub(c, q.0), d.0), zsub(zsub(c, q.1), d.1)), d)
        }
        fn img_diag(q: (ZP, ZP), d: (ZP, ZP)) -> ((ZP, ZP), (ZP, ZP)) {
            ((q.1, q.0), (d.1, d.0))
        }
        fn img_anti(q: (ZP, ZP), d: (ZP, ZP), c: ZP) -> ((ZP, ZP), (ZP, ZP)) {
            (
                (zsub(zsub(c, q.1), d.1), zsub(zsub(c, q.0), d.0)),
                (d.1, d.0),
            )
        }

        // state-1 patch: rule_1 → refine; build the same multiset as
        // tiles2 but for the state-1 chair.
        let mut tiles2_s1: Vec<(u8, ZP, ZP)> = Vec::new();
        for (st1, ax1, ay1) in RULE_1 {
            for &(st2, ax2, ay2) in rule(st1) {
                tiles2_s1.push((st2, zadd(zmul(LAMZ, ax1), ax2), zadd(zmul(LAMZ, ay1), ay2)));
            }
        }

        let base: Vec<(u8, ZP, ZP, ZP)> = tiles2
            .iter()
            .map(|&(st, qx, qy)| {
                let (w, _) = child_dim(st);
                (st, qx, qy, w) // (st, qx, qy, w) — h determined by st
            })
            .collect();
        let base_s1: Vec<(u8, ZP, ZP, ZP)> = tiles2_s1
            .iter()
            .map(|&(st, qx, qy)| {
                let (w, _) = child_dim(st);
                (st, qx, qy, w)
            })
            .collect();

        // multiset helper: sort by (st, zval(qx), zval(qy), zval(w)) — the
        // key is a total order since (st, qx, qy) is unique per tile.
        fn mset(v: &[(u8, ZP, ZP, ZP)]) -> Vec<(u8, ZP, ZP, ZP)> {
            let mut out = v.to_vec();
            out.sort_by(|a, b| {
                a.0.cmp(&b.0)
                    .then_with(|| zval(a.1).partial_cmp(&zval(b.1)).unwrap())
                    .then_with(|| zval(a.2).partial_cmp(&zval(b.2)).unwrap())
                    .then_with(|| zval(a.3).partial_cmp(&zval(b.3)).unwrap())
            });
            out
        }

        // R90: expect base to map to base_s1 (not invariant)
        let img_r90 = tiles2
            .iter()
            .map(|&(st, qx, qy)| {
                let d = child_dim(st);
                let ((tx, ty), (tw, _)) = img_r90((qx, qy), d, c_reg);
                (R90ST[st as usize], tx, ty, tw)
            })
            .collect::<Vec<_>>();
        assert_eq!(
            mset(&img_r90),
            mset(&base_s1),
            "R90 of state-0 patch != state-1 patch (should be the state-1 patch)"
        );
        assert_ne!(
            mset(&img_r90),
            mset(&base),
            "R90 of state-0 patch should NOT equal state-0 patch"
        );

        // Mx: expect base to map to base_s1
        let img_mx = tiles2
            .iter()
            .map(|&(st, qx, qy)| {
                let d = child_dim(st);
                let ((tx, ty), (tw, _)) = img_mx((qx, qy), d, c_reg);
                (MXST[st as usize], tx, ty, tw)
            })
            .collect::<Vec<_>>();
        assert_eq!(
            mset(&img_mx),
            mset(&base_s1),
            "Mx of state-0 patch != state-1 patch (should be the state-1 patch)"
        );
        assert_ne!(mset(&img_mx), mset(&base));

        // R180: invariant
        let img_r180 = tiles2
            .iter()
            .map(|&(st, qx, qy)| {
                let d = child_dim(st);
                let ((tx, ty), (tw, _)) = img_r180((qx, qy), d, c_reg);
                (R90ST[R90ST[st as usize] as usize], tx, ty, tw)
            })
            .collect::<Vec<_>>();
        assert_eq!(
            mset(&img_r180),
            mset(&base),
            "R180 of state-0 patch != state-0 patch"
        );

        // diag: invariant
        let img_diag = tiles2
            .iter()
            .map(|&(st, qx, qy)| {
                let d = child_dim(st);
                let ((tx, ty), (tw, _)) = img_diag((qx, qy), d);
                (DIAGST[st as usize], tx, ty, tw)
            })
            .collect::<Vec<_>>();
        assert_eq!(
            mset(&img_diag),
            mset(&base),
            "diag of state-0 patch != state-0 patch"
        );

        // anti-diag: invariant (= R90 ∘ Mx; both state-0-preserving on the
        // chair pair)
        let img_anti = tiles2
            .iter()
            .map(|&(st, qx, qy)| {
                let d = child_dim(st);
                let ((tx, ty), (tw, _)) = img_anti((qx, qy), d, c_reg);
                (R90ST[MXST[st as usize] as usize], tx, ty, tw)
            })
            .collect::<Vec<_>>();
        assert_eq!(
            mset(&img_anti),
            mset(&base),
            "anti-diag of state-0 patch != state-0 patch"
        );
    }
}
