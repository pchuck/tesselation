//! Gähler shield tiling (dodecagonal, 12-fold quasiperiodic substitution).
//!
//! Prototiles: a "shield" hexagon, a unit square, and two unit
//! equilateral triangles (the "black" and "white" labels). Every shape
//! occurs in two labelled orientations (R/L) that share geometry but
//! carry different substitution rules — Gähler's (1988) matching rules.
//! Output protos collapse L/R: 0 = shield, 1 = square, 2 = black
//! triangle, 3 = white triangle.
//!
//! Inflation factor `θ = 2cos(15°) = (√6+√2)/2`, so `θ² = 2+√3` — the
//! Perron eigenvalue of the 8-kind count matrix. The children of a
//! parent tile (scale `s/θ`) do not tile the plain θ-parent but a
//! *kinked* supertile: the θ-parent with triangular kinks in its
//! boundary (shield: 6 in, square: 4 in, black triangle: 3 out, white
//! triangle: 2 out + 1 in; each kink is ±1/4 of a unit square). All
//! eight rules were verified in exact `Q(√2,√3)` arithmetic before
//! porting: children pairwise interior-disjoint, union boundary a
//! closed unit-edge cycle in 15° directions, exact pinned areas
//! (shield 3+5√3/2, square 1+√3, black (3+√3)/2, white 1+√3/2, per
//! unit-edge kinked supertile).
//!
//! Geometry lives in `Q(√2,√3)`: child anchors are dodecagonal
//! 6-vectors mapped by the same dodec→orth transform the published
//! recipe uses, all orientations are multiples of 15°, and every
//! vertex is a finite `Q(√2,√3)` combination (in floats, to ~1e-13).
//!
//! The 12-fold character is Bragg/MLD, not point symmetry: no local
//! 30° rotational symmetry point exists in the tiling (every
//! prototile angle is ≥ 60° > 30°, so no vertex can be a 30° center;
//! a depth-5 search over 5 632 tiles found at best 4.3% partial local
//! matches, no exact one). What the rule guarantees is the
//! dodecagonal *frame*: every edge lies on one of the 12 directions
//! `15°·k` (asserted by tests), and the matching window is a regular
//! dodecagon.
//!
//! Generation mirrors the other substitution modules: seed with the
//! 13-tile kinked-shield supertile (the RSH parent's children) at
//! scale `θ^n`, its area centroid on the origin — the patch covers a
//! disk of radius `R_IN·θ^n` with `R_IN = (1+√3)/2` exact — then
//! deflate `n` times with per-step culling to unit tiles. Culling is
//! sound because every descendant of a piece of scale `s` stays
//! within `aabb(piece) ⊕ B·s` with `B = sin15°/(θ−1) ≈ 0.2777`
//! (the only shapes whose kinks point outward are the triangles, and
//! their kink depth is exactly sin15°; the geometric series over the
//! deflation levels sums to sin15°/(θ−1)).

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::TilingCategory;
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, MAX_TILES, SCALE_ID, SIMILARITY_PARAMS};

const SQRT_2: f64 = std::f64::consts::SQRT_2;
const SQRT_3: f64 = 1.732_050_807_568_877_2;
const SQRT_6: f64 = 2.449_489_742_783_178;
/// cos 15° = (√6+√2)/4.
const C15: f64 = (SQRT_6 + SQRT_2) / 4.0;
/// sin 15° = (√6−√2)/4.
const S15: f64 = (SQRT_6 - SQRT_2) / 4.0;
/// √2/2.
const S2: f64 = SQRT_2 / 2.0;
/// Inflation factor θ = 2cos 15° = (√6+√2)/2 (θ² = 2+√3).
const THETA: f64 = 2.0 * C15;
/// 1/θ = (√6−√2)/2.
const INV_THETA: f64 = (SQRT_6 - SQRT_2) / 2.0;
/// Area centroid of the unit-edge kinked-shield seed region,
/// √2/2 + √6/3 (exact in Q(√2,√3)).
const SEED_CENTER: f64 = S2 + SQRT_6 / 3.0;
/// Inradius of the seed region about that centroid, (1+√3)/2 (exact:
/// the six inward-kink edges are tangent to the inscribed disk).
const R_IN: f64 = (1.0 + SQRT_3) / 2.0;
/// Culling margin in child-scale units: every descendant of a piece
/// of scale `s` stays within `aabb(piece) ⊕ CULL_B·s`, with the exact
/// bound `sin15°/(θ−1) ≈ 0.27775`; a hair of slack on top.
const CULL_B: f64 = S15 / (THETA - 1.0) + 1e-9;
/// Largest prototile area (the shield hexagon), for the pre-budget.
const MAX_TILE_AREA: f64 = (3.0 + SQRT_3) / 2.0;
/// Refinement cap: θ^20 ≈ 5.1e6 units of seed reach.
const STEP_CAP: usize = 20;

/// The eight labelled pieces: 4 shapes × R/L. L/R are rule labels.
const RSH: u8 = 0;
const LSH: u8 = 1;
const RSQ: u8 = 2;
const LSQ: u8 = 3;
const RBT: u8 = 4;
const LBT: u8 = 5;
const RWT: u8 = 6;
const LWT: u8 = 7;

static SHIELD_SHAPE: [Vec2; 6] = [
    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(1.0 + SQRT_3 / 2.0, 0.5),
    Vec2::new((1.0 + SQRT_3) / 2.0, (1.0 + SQRT_3) / 2.0),
    Vec2::new(0.5, (2.0 + SQRT_3) / 2.0),
    Vec2::new(0.0, 1.0),
];
static SQUARE_SHAPE: [Vec2; 4] = [
    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(1.0, 1.0),
    Vec2::new(0.0, 1.0),
];
static TRI_SHAPE: [Vec2; 3] = [
    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(0.5, SQRT_3 / 2.0),
];

/// Output proto index of a piece kind (L/R collapse).
fn proto_of(kind: u8) -> u8 {
    match kind {
        RSH | LSH => 0,
        RSQ | LSQ => 1,
        RBT | LBT => 2,
        _ => 3,
    }
}

/// Canonical shape of a piece kind (anchor v0 at the origin, first
/// edge along +x, unit edges).
fn shape(kind: u8) -> &'static [Vec2] {
    match proto_of(kind) {
        0 => &SHIELD_SHAPE,
        1 => &SQUARE_SHAPE,
        _ => &TRI_SHAPE,
    }
}

/// One substitution child: `(child kind, orientation delta in 15°
/// steps, dodecagonal offset (u,v,w,x,y,z))`. The child anchor in the
/// parent frame is `dodec2orth(offset) · (parent scale / θ)`, rotated
/// by the parent's orientation.
type Child = (u8, i32, [i32; 6]);

/// RightShield → 2 squares + 3 black + 4 black-left… (13 children):
/// 2 RSQ, 1 LSQ, 3 RBT, 4 LBT, 2 RWT, 1 LWT.
const RSH_RULE: [Child; 13] = [
    (RBT, 1, [0, 0, 0, 0, 0, 0]),
    (RBT, 7, [1, 0, 0, 0, 0, -1]),
    (RSQ, 9, [1, 0, 0, 1, 0, -1]),
    (LBT, 11, [0, 1, 1, 0, 0, 0]),
    (LBT, 5, [0, 1, 1, 0, -1, 0]),
    (LSQ, 1, [0, 1, 1, 0, 0, 0]),
    (RWT, -1, [0, 1, 1, 0, 0, 1]),
    (LBT, 9, [1, 1, 1, 1, 0, 0]),
    (LBT, 3, [1, 1, 1, 0, 0, 0]),
    (RSQ, -7, [1, 1, 1, 0, 0, 0]),
    (RWT, -9, [1, 2, 1, 0, 0, 0]),
    (LWT, -5, [1, 1, 0, 0, 0, 0]),
    (RBT, 9, [2, 1, 0, 0, 0, -1]),
];

/// LeftShield → 13 children: 1 RSQ, 2 LSQ, 4 RBT, 3 LBT, 1 RWT, 2 LWT.
const LSH_RULE: [Child; 13] = [
    (LBT, -7, [0, 0, 1, 0, 0, 0]),
    (RBT, 7, [1, 0, 0, 0, 0, -1]),
    (LSQ, 9, [1, 0, 0, 1, 0, -1]),
    (LBT, 11, [0, 1, 1, 0, 0, 0]),
    (RBT, 13, [1, 1, 1, 0, 0, 0]),
    (LSQ, 1, [0, 1, 1, 0, 0, 0]),
    (RWT, -1, [0, 1, 1, 0, 0, 1]),
    (LBT, 9, [1, 1, 1, 1, 0, 0]),
    (LWT, 3, [1, 1, 1, 0, 0, 0]),
    (RSQ, -7, [1, 1, 1, 0, 0, 0]),
    (RBT, -9, [1, 2, 1, 0, 0, 0]),
    (LWT, -5, [1, 1, 0, 0, 0, 0]),
    (RBT, 9, [2, 1, 0, 0, 0, -1]),
];

/// RightSquare → 5 children: 1 LSQ, 2 RBT, 2 LBT.
const RSQ_RULE: [Child; 5] = [
    (LBT, -7, [0, 0, 1, 0, 0, 0]),
    (RBT, 7, [1, 0, 0, 0, 0, -1]),
    (LSQ, 9, [1, 0, 0, 1, 0, -1]),
    (LBT, 11, [0, 1, 1, 0, 0, 0]),
    (RBT, 13, [1, 1, 1, 0, 0, 0]),
];

/// LeftSquare → 5 children: 1 RSQ, 2 RBT, 2 LBT.
const LSQ_RULE: [Child; 5] = [
    (LBT, -7, [0, 0, 1, 0, 0, 0]),
    (RBT, 7, [1, 0, 0, 0, 0, -1]),
    (RSQ, -9, [0, 1, 1, 0, 0, 0]),
    (LBT, 11, [0, 1, 1, 0, 0, 0]),
    (RBT, 13, [1, 1, 1, 0, 0, 0]),
];

/// RightBlackTriangle → 1 child: the inflated right shield.
const RBT_RULE: [Child; 1] = [(RSH, 15, [0, 1, 1, 0, 0, 0])];

/// LeftBlackTriangle → 1 child: the inflated left shield.
const LBT_RULE: [Child; 1] = [(LSH, 15, [0, 1, 1, 0, 0, 0])];

/// RightWhiteTriangle → 3 children: 1 LSQ, 1 RWT, 1 LWT.
const RWT_RULE: [Child; 3] = [
    (LWT, -7, [0, 0, 1, 0, 0, 0]),
    (RWT, 7, [1, 0, 0, 0, 0, -1]),
    (LSQ, 9, [1, 0, 0, 1, 0, -1]),
];

/// LeftWhiteTriangle → 3 children: 1 RSQ, 1 RWT, 1 LWT.
const LWT_RULE: [Child; 3] = [
    (LWT, -7, [0, 0, 1, 0, 0, 0]),
    (RWT, 7, [1, 0, 0, 0, 0, -1]),
    (RSQ, -9, [0, 1, 1, 0, 0, 0]),
];

fn rule(kind: u8) -> &'static [Child] {
    match kind {
        LSH => &LSH_RULE,
        RSQ => &RSQ_RULE,
        LSQ => &LSQ_RULE,
        RBT => &RBT_RULE,
        LBT => &LBT_RULE,
        RWT => &RWT_RULE,
        LWT => &LWT_RULE,
        _ => &RSH_RULE,
    }
}

/// Dodecagonal 6-vector → orthogonal coordinates (child-edge units),
/// the published recipe's `dod2orth`.
fn dodec2orth(d: [i32; 6]) -> Vec2 {
    let (u, v, w, x, y, z) = (
        d[0] as f64,
        d[1] as f64,
        d[2] as f64,
        d[3] as f64,
        d[4] as f64,
        d[5] as f64,
    );
    Vec2::new(
        (u - z) * C15 + (v - y) * S2 + (w - x) * S15,
        (u + z) * S15 + (v + y) * S2 + (w + x) * C15,
    )
}

/// A piece: prototile `kind`, orientation (`k` 15° CCW steps from the
/// canonical frame), anchor position (of v0), and edge scale (unit
/// pieces have `s == 1`).
#[derive(Clone, Copy, Debug)]
struct Piece {
    kind: u8,
    k: i32,
    p: Vec2,
    s: f64,
}

fn rot15(v: Vec2) -> Vec2 {
    Vec2::new(C15 * v.x - S15 * v.y, S15 * v.x + C15 * v.y)
}

/// Rotate by `k` 15° CCW steps.
fn rot(v: Vec2, k: i32) -> Vec2 {
    let mut v = v;
    for _ in 0..k.rem_euclid(24) {
        v = rot15(v);
    }
    v
}

impl Piece {
    fn verts(&self) -> Vec<Vec2> {
        shape(self.kind)
            .iter()
            .map(|&c| self.p + rot(c, self.k) * self.s)
            .collect()
    }

    fn poly(&self) -> Polygon {
        Polygon::from_slice(&self.verts())
    }

    fn aabb(&self) -> BBox {
        let mut lo = Vec2::new(f64::MAX, f64::MAX);
        let mut hi = Vec2::new(f64::MIN, f64::MIN);
        for &c in shape(self.kind) {
            let q = self.p + rot(c, self.k) * self.s;
            lo.x = lo.x.min(q.x);
            lo.y = lo.y.min(q.y);
            hi.x = hi.x.max(q.x);
            hi.y = hi.y.max(q.y);
        }
        BBox::new(lo, hi)
    }

    /// Substitution children at 1/θ of this piece's scale. They tile
    /// the piece's *kinked* supertile (the θ-parent with its boundary
    /// kinks), which for shield/square parents sits inside the plain
    /// parent and for triangle parents sticks out by at most
    /// `sin15°` per child edge — see the culling bound in the module
    /// docs.
    fn children(&self) -> Vec<Piece> {
        let s2 = self.s * INV_THETA;
        rule(self.kind)
            .iter()
            .map(|&(kind, dk, d)| {
                let off = dodec2orth(d);
                Piece {
                    kind,
                    k: self.k + dk,
                    p: self.p + rot(off, self.k) * s2,
                    s: s2,
                }
            })
            .collect()
    }
}

/// The seed: the 13-tile kinked-shield supertile (a right shield's
/// children) at `scale`, its area centroid on the origin. The patch
/// covers a disk of radius `R_IN·scale`.
fn seed(scale: f64) -> Vec<Piece> {
    let parent = Piece {
        kind: RSH,
        k: 0,
        p: Vec2::new(-SEED_CENTER * scale, -SEED_CENTER * scale),
        s: scale * THETA,
    };
    parent.children()
}

/// Substitute every piece once, culling children that cannot reach
/// `region` (see the module docs for the soundness bound).
fn refine_step(pieces: &[Piece], region: &BBox) -> Vec<Piece> {
    let mut out = Vec::with_capacity(pieces.len() * 4);
    for t in pieces {
        for ch in t.children() {
            let m = ch.aabb().padding(CULL_B * ch.s);
            if m.intersects(*region) {
                out.push(ch);
            }
        }
    }
    out
}

/// Gähler shield tiling (aperiodic, dodecagonal 12-fold quasiperiodic).
pub struct ShieldTiling;

impl Tiling for ShieldTiling {
    fn id(&self) -> &'static str {
        "shield"
    }

    fn name(&self) -> &'static str {
        "Gähler shield"
    }

    fn description(&self) -> &'static str {
        "Quasiperiodic shield-hexagon + square + triangle substitution tiling; dodecagonal (12-fold), inflation √(2+√3)."
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
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None)
            .with_notes("aperiodic; dodecagonal (12-fold) quasiperiodic; Gähler 1988");
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
            let q = inv.transform(corner);
            rmin.x = rmin.x.min(q.x);
            rmin.y = rmin.y.min(q.y);
            rmax.x = rmax.x.max(q.x);
            rmax.y = rmax.y.max(q.y);
        }
        let region = BBox::new(rmin, rmax);

        // Farthest *region* corner from the origin — the seed patch
        // (a kinked region, not a disk) covers a true disk of radius
        // R_IN·scale, so the region's own bbox corners are the exact
        // reach requirement.
        let mut reach: f64 = 0.0;
        for &corner in &region.corners() {
            reach = reach.max(corner.length());
        }
        if reach < 1e-9 {
            return fail("empty view");
        }

        // Pre-generation budget: no unit-scale tile exceeds the shield
        // hexagon's area (cell units), so this is a lower bound on the
        // tile count — checked before any refinement work.
        let s = params.float(SCALE_ID, 1.0);
        if query.region.area() / (MAX_TILE_AREA * s * s) > MAX_TILES as f64 {
            return fail("too many tiles for this view — zoom in");
        }

        // Seed scale: θ^n with n minimal subject to R_IN·θ^n ≥ reach
        // (n = 0 keeps the patch at the 13-tile level).
        let mut scale = 1.0_f64;
        let mut steps = 0_usize;
        while R_IN * scale < reach && steps < STEP_CAP {
            scale *= THETA;
            steps += 1;
        }
        if R_IN * scale < reach * (1.0 - 1e-9) {
            return fail("view too far from the seed — pan back or zoom in");
        }

        let mut pieces = seed(scale);
        for _ in 0..steps {
            pieces = refine_step(&pieces, &region);
        }
        if pieces.is_empty() {
            return fail("empty view");
        }

        let mut tiles = Vec::new();
        for t in &pieces {
            let world = t.poly().transform(&map);
            if world.aabb().intersects(query.region) {
                let proto = proto_of(t.kind);
                tiles.push(Tile {
                    proto,
                    seed: proto as u32, level: steps as u8,
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

#[cfg(test)]
mod tests {
    use super::*;

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

    /// Point in (non-convex) polygon by ray casting; the boundary
    /// counts as inside.
    fn in_poly(vs: &[Vec2], q: Vec2) -> bool {
        let n = vs.len();
        let mut inside = false;
        for i in 0..n {
            let a = vs[i];
            let b = vs[(i + 1) % n];
            if (a.y > q.y) != (b.y > q.y) {
                let xin = (b.x - a.x) * (q.y - a.y) / (b.y - a.y) + a.x;
                if q.x < xin {
                    inside = !inside;
                }
            }
        }
        inside
    }

    fn polygon_area(vs: &[Vec2]) -> f64 {
        let n = vs.len();
        let mut a = 0.0;
        for i in 0..n {
            let (p, q) = (vs[i], vs[(i + 1) % n]);
            a += p.x * q.y - q.x * p.y;
        }
        a.abs() * 0.5
    }

    /// Pinned unit-edge kinked supertile boundaries (parent anchor at
    /// the origin, 15°-step directions), exactly as certified in
    /// Q(√2,√3) before porting.
    fn kinked(kind: u8) -> Vec<Vec2> {
        let (start, dirs): (Vec2, &[i32]) = match kind {
            RSH => (
                Vec2::new(0.0, 0.0),
                &[1, 23, 3, 1, 9, 7, 11, 9, 17, 15, 19, 17],
            ),
            LSH => (
                Vec2::new(S15, C15),
                &[17, 1, 23, 3, 1, 9, 7, 11, 9, 17, 15, 19],
            ),
            RSQ | LSQ => (Vec2::new(S15, C15), &[17, 1, 23, 7, 5, 13, 11, 19]),
            RBT | LBT => (
                Vec2::new(C15, C15 + S2),
                &[15, 17, 23, 1, 7, 9],
            ),
            _ => (Vec2::new(S15, C15), &[17, 1, 23, 7, 9, 15]),
        };
        let mut pts = vec![start];
        for &d in dirs {
            let u = rot(Vec2::new(1.0, 0.0), d);
            pts.push(*pts.last().unwrap() + u);
        }
        let (last, first) = (pts.pop().unwrap(), pts[0]);
        assert!(
            (last - first).length() < 1e-12,
            "kinked boundary for kind {kind} does not close"
        );
        pts
    }

    fn kinked_area(kind: u8) -> f64 {
        match kind {
            RSH | LSH => 3.0 + 5.0 * SQRT_3 / 2.0,
            RSQ | LSQ => 1.0 + SQRT_3,
            RBT | LBT => (3.0 + SQRT_3) / 2.0,
            _ => 1.0 + SQRT_3 / 2.0,
        }
    }

    fn canonical_parent(kind: u8) -> Piece {
        Piece {
            kind,
            k: 0,
            p: Vec2::ZERO,
            s: THETA,
        }
    }

    /// The children of a θ-scaled parent (unit children) must exactly
    /// tile the pinned kinked supertile: pinned area, and every sample
    /// point of the kinked polygon covered by exactly one child.
    fn check_substitution(kind: u8) {
        let parent = canonical_parent(kind);
        let kids = parent.children();
        assert!(
            kids.iter().all(|k| (k.s - 1.0).abs() < 1e-9),
            "children at unit scale"
        );

        let ka = polygon_area(&kinked(kind));
        assert!(
            (ka - kinked_area(kind)).abs() <= 1e-12 * ka,
            "kind {kind}: pinned kinked polygon area {ka} != {0}",
            kinked_area(kind)
        );
        let kids_area: f64 = kids.iter().map(|k| polygon_area(&k.verts())).sum();
        assert!(
            (kids_area - ka).abs() <= 1e-9 * ka,
            "kind {kind}: child areas {kids_area} != kinked area {ka}"
        );

        let kpol = kinked(kind);
        let bb_min = kpol.iter().fold(Vec2::new(f64::MAX, f64::MAX), |m, p| {
            Vec2::new(m.x.min(p.x), m.y.min(p.y))
        });
        let bb_max = kpol.iter().fold(Vec2::new(f64::MIN, f64::MIN), |m, p| {
            Vec2::new(m.x.max(p.x), m.y.max(p.y))
        });
        let mut rng = Rng(0x517C_C1B7_2722_0A95);
        let mut checked = 0;
        for _ in 0..4000 {
            let q = Vec2::new(
                bb_min.x + rng.f64() * (bb_max.x - bb_min.x),
                bb_min.y + rng.f64() * (bb_max.y - bb_min.y),
            );
            if !in_poly(&kpol, q) {
                continue;
            }
            let n = kids
                .iter()
                .filter(|k| in_poly(&k.verts(), q))
                .count();
            assert_eq!(
                n, 1,
                "kind {kind}: point {q} covered by {n} children"
            );
            checked += 1;
        }
        assert!(checked > 400, "kind {kind}: too few samples landed");
    }

    #[test]
    fn rsh_substitution_tiles_kinked_supertile() {
        check_substitution(RSH);
    }
    #[test]
    fn lsh_substitution_tiles_kinked_supertile() {
        check_substitution(LSH);
    }
    #[test]
    fn rsq_substitution_tiles_kinked_supertile() {
        check_substitution(RSQ);
    }
    #[test]
    fn lsq_substitution_tiles_kinked_supertile() {
        check_substitution(LSQ);
    }
    #[test]
    fn rbt_substitution_tiles_kinked_supertile() {
        check_substitution(RBT);
    }
    #[test]
    fn lbt_substitution_tiles_kinked_supertile() {
        check_substitution(LBT);
    }
    #[test]
    fn rwt_substitution_tiles_kinked_supertile() {
        check_substitution(RWT);
    }
    #[test]
    fn lwt_substitution_tiles_kinked_supertile() {
        check_substitution(LWT);
    }

    // ---------- growth / Perron ----------

    /// The 8-kind count matrix (rows = parent, columns = child; order
    /// RSH LSH RSQ LSQ RBT LBT RWT LWT).
    const M: [[i64; 8]; 8] = [
        [0, 0, 2, 1, 3, 4, 2, 1],
        [0, 0, 1, 2, 4, 3, 1, 2],
        [0, 0, 0, 1, 2, 2, 0, 0],
        [0, 0, 1, 0, 2, 2, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 1, 1],
        [0, 0, 1, 0, 0, 0, 1, 1],
    ];

    fn m_trans_v(v: &[i64; 8]) -> [i64; 8] {
        let mut out = [0i64; 8];
        for i in 0..8 {
            for j in 0..8 {
                out[i] += M[j][i] * v[j];
            }
        }
        out
    }

    /// Substitute every piece once, no culling.
    fn refine_all(pieces: &[Piece]) -> Vec<Piece> {
        let mut out = Vec::with_capacity(pieces.len() * 4);
        for t in pieces {
            out.extend(t.children());
        }
        out
    }

    fn count_kinds(pieces: &[Piece]) -> [usize; 8] {
        let mut c = [0usize; 8];
        for p in pieces {
            c[p.kind as usize] += 1;
        }
        c
    }

    #[test]
    fn growth_counts_follow_the_count_matrix() {
        // The 13-tile seed (RSH children), refined by the real
        // `children()`: totals 13 → 31 → 151 → 493 (matrix orbit).
        let mut v = [0i64, 0, 2, 1, 3, 4, 2, 1];
        let mut pieces = seed(1.0);
        assert_eq!(pieces.len(), 13);
        for (depth, total) in [(0usize, 13i64), (1, 31), (2, 151), (3, 493)] {
            let got = count_kinds(&pieces);
            let want: [i64; 8] = v.map(|x| x);
            assert_eq!(
                got.map(|g| g as i64),
                want,
                "depth {depth}: got {got:?}, want {want:?}"
            );
            assert_eq!(pieces.len() as i64, total, "depth {depth} total");
            pieces = refine_all(&pieces);
            v = m_trans_v(&v);
        }
    }

    #[test]
    fn perron_eigenvalue_is_2_plus_sqrt3() {
        // Power iteration on the count matrix converges to the Perron
        // eigenvalue 2+√3 = θ² (the area growth per deflation).
        let mut x = [1.0_f64; 8];
        let mut lambda = 0.0_f64;
        for _ in 0..200 {
            let mut y = [0.0_f64; 8];
            for i in 0..8 {
                for j in 0..8 {
                    y[i] += M[j][i] as f64 * x[j];
                }
            }
            let m = *y.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            lambda = m;
            for i in 0..8 {
                x[i] = y[i] / m;
            }
        }
        let want = 2.0 + SQRT_3;
        assert!(
            (lambda - want).abs() < 1e-9,
            "Perron {lambda} != 2+√3 = {want}"
        );
        // θ² must agree too: inflation is exact in this frame.
        assert!((THETA * THETA - want).abs() < 1e-12);
    }

    #[test]
    fn seed_is_dodecagonal_frame() {
        // The substitution's 12-fold guarantee: every edge of every
        // tile lies on one of the 12 directions 15°·k (the dodecagonal
        // frame; the matching window is a regular dodecagon). Python
        // audit (exact Q(√2,√3)): the 151-tile patch has 522 edges
        // covering exactly 6 axis classes mod 180° (odd 15° steps = 12
        // oriented directions), so the window is a 12-sided dodecagon.
        // Note the honest negative: no local 30° rotational symmetry
        // point exists in this tiling — every prototile angle is ≥ 60° >
        // 30°, so no vertex can be a 30° symmetry center (a depth-5
        // search over 5 632 tiles found at best 242/5 632 = 4.3%
        // partial local matches and no exact one).
        let pieces = refine_all(&refine_all(&seed(1.0)));
        assert_eq!(pieces.len(), 151);
        for t in &pieces {
            let vs = t.verts();
            let n = vs.len();
            for i in 0..n {
                let e = vs[(i + 1) % n] - vs[i];
                let mut on_frame = false;
                for k in 0..24 {
                    let d = rot(Vec2::new(1.0, 0.0), k);
                    if e.cross(d).abs() <= 1e-9 * e.length() {
                        on_frame = true;
                        break;
                    }
                }
                assert!(on_frame, "edge {e} off the 15° dodecagonal frame");
            }
        }
    }

    #[test]
    fn culling_matches_unculled_for_full_patch() {
        // A region containing the whole patch must cull nothing: the
        // cull gate (aabb ⊕ CULL_B·s vs region) drops exactly the
        // pieces whose descendants stay outside.
        let pieces0 = seed(THETA * THETA);
        let region = BBox::new(
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, 50.0),
        );
        let mut c = pieces0.clone();
        let mut u = pieces0.clone();
        for _ in 0..2 {
            c = refine_step(&c, &region);
            u = refine_all(&u);
        }
        assert_eq!(c.len(), u.len(), "culling dropped or duplicated pieces");
        let key = |p: &Piece| {
            (
                p.kind,
                p.k.rem_euclid(24),
                (p.p.x * 1e6).round() as i64,
                (p.p.y * 1e6).round() as i64,
            )
        };
        let mut a: Vec<_> = c.iter().map(key).collect();
        let mut b: Vec<_> = u.iter().map(key).collect();
        a.sort_unstable();
        b.sort_unstable();
        assert_eq!(a, b, "culled and unculled patches differ");
    }

    /// Through four refinements the 13-tile seed stays uniformly scaled
    /// (all pieces at `theta^-d`, within float rounding) and follows the
    /// pinned growth totals 13, 31, 151, 493, 1981 (matrix orbit;
    /// verified in exact integer Python against the published count
    /// sequence).
    #[test]
    fn refinement_uniform_scale_growth() {
        let totals = [13usize, 31, 151, 493, 1981];
        let mut pieces = seed(1.0);
        for (d, want) in totals.iter().enumerate() {
            assert_eq!(&pieces.len(), want, "depth {d}");
            let s_want = THETA.powi(-(d as i32));
            for p in &pieces {
                assert!(
                    (p.s - s_want).abs() <= 1e-9 * s_want,
                    "depth {d}: non-uniform scale {} vs {s_want}",
                    p.s
                );
            }
            pieces = refine_all(&pieces);
        }
    }

    /// Growth reference from the published 10-tile seed (3 left squares,
    /// 4 black + 3 white triangles): 10, 28, 112, 400, 1528, 5632,
    /// 21160, 78688, 294232 tiles at depths 0-8, ratios -> 2+sqrt3.
    /// Computed exactly from the count matrix `M` (cross-checked in
    /// exact Python against the published sequence).
    #[test]
    fn published_seed_growth_reference() {
        let mut v = [0i64, 0, 0, 3, 0, 4, 0, 3]; // LSQ 3, LBT 4, LWT 3
        let want = [10i64, 28, 112, 400, 1528, 5632, 21160, 78688, 294232];
        for (d, w) in want.iter().enumerate() {
            assert_eq!(v.iter().sum::<i64>(), *w, "published growth depth {d}");
            v = m_trans_v(&v);
        }
    }

    /// The seed patch covers a disk of radius `R_IN` about the origin
    /// (module docs): its boundary (the kinked RSH supertile, translated
    /// so the area centroid sits at the origin) stays at least `R_IN`
    /// away, and six inward-kink edges touch exactly.
    #[test]
    fn seed_covers_disk_of_radius_r_in() {
        let pts = {
            let mut v = kinked(RSH);
            for q in &mut v {
                q.x -= SEED_CENTER;
                q.y -= SEED_CENTER;
            }
            v
        };
        let mut touches = 0;
        let n = pts.len();
        for i in 0..n {
            let a = pts[i];
            let b = pts[(i + 1) % n];
            let e = b - a;
            let t = -(a.dot(e)) / e.dot(e);
            let d = if (0.0..=1.0).contains(&t) {
                a.cross(e).abs() / e.length()
            } else {
                a.length().min(b.length())
            };
            if d < R_IN - 1e-9 {
                panic!("seed boundary edge {a} -> {b} at distance {d} < R_IN {R_IN}");
            }
            if (d - R_IN).abs() < 1e-9 {
                touches += 1;
            }
        }
        assert_eq!(touches, 6, "inward-kink tangencies");
    }

    /// Gap/overlap check at patch scale: every sampled point strictly
    /// inside the kinked-seed region stays covered by exactly one piece
    /// through three refinements (the pieces may stick out past the
    /// region boundary at outward triangle kinks, but never open a gap
    /// or double-cover inside). Verified in exact Python to depth 4.
    /// The 10-tile published-seed growth reference (10, 28, 112, 400,
    /// 1528, 5632, 21160, 78688, 294232 at depths 0-8) is asserted in
    /// `published_seed_growth_reference`.
    #[test]
    fn seed_refinement_covers_seed_region_once() {
        let mut region = kinked(RSH);
        for q in &mut region {
            q.x -= SEED_CENTER;
            q.y -= SEED_CENTER;
        }
        let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
        let mut bb_min = Vec2::new(f64::MAX, f64::MAX);
        let mut bb_max = Vec2::new(f64::MIN, f64::MIN);
        for q in &region {
            bb_min.x = bb_min.x.min(q.x);
            bb_min.y = bb_min.y.min(q.y);
            bb_max.x = bb_max.x.max(q.x);
            bb_max.y = bb_max.y.max(q.y);
        }
        let mut pts = Vec::new();
        while pts.len() < 1500 {
            let q = Vec2::new(
                bb_min.x + rng.f64() * (bb_max.x - bb_min.x),
                bb_min.y + rng.f64() * (bb_max.y - bb_min.y),
            );
            if in_poly(&region, q) {
                pts.push(q);
            }
        }
        let mut pieces = seed(1.0);
        for depth in 0..4usize {
            for &q in &pts {
                let n = pieces.iter().filter(|k| in_poly(&k.verts(), q)).count();
                assert_eq!(n, 1, "depth {depth}: seed point {q} covered {n} times");
            }
            if depth < 3 {
                pieces = refine_all(&pieces);
            }
        }
    }
}
