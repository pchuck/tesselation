//! Fibonacci × Fibonacci — the Bielefeld product-substitution tiling.
//!
//! This is the Cartesian product of two 1-D Fibonacci word substitutions
//! (Bielefeld Tilings Encyclopedia, entry "fibonacci-times-fibonacci"):
//! each 1-D tiling has a long tile L (length φ) and a short tile S
//! (length 1) and the rule L → LS, S → L (linear inflation φ). The 2-D
//! tiling is the product: the plane is tiled by L×L squares (φ×φ),
//! L×S and S×L rectangles (φ×1 and 1×φ) and S×S squares (1×1) —
//! **three prototile shapes**, carried by **four orientation classes**
//! (LL, LS, SL, SS; the two rectangle orientations LS/SL share one
//! geometric shape but keep separate substitution classes).
//!
//! **The module rule is TWO product steps (linear inflation τ² = φ²).**
//! The encyclopedia figure shows the single-step product rule R1
//! (inflation φ, count matrix A4). This module implements the square of
//! that rule: R2 = R1 ∘ R1, whose 1-D words are L → LSL and S → LS, so
//! one module step inflates linearly by φ² = τ². The count matrix over
//! the 4 classes is B = A4²:
//!
//! ```text
//! A4 = [[1,1,1,1],     B = A4² = [[4,2,2,1],
//!      [1,0,1,0],                    [2,2,1,1],
//!      [1,1,0,0],                    [2,1,2,1],
//!      [1,0,0,0]]                    [1,1,1,1]]
//! ```
//!
//! B·v = φ⁴·v with v = (φ², φ, φ, 1) — the exact areas of the four
//! classes — so the Perron eigenvalue is τ⁴ = φ⁴ = 2 + 3φ (a unit of
//! Z[φ], conjugate 5 − 3φ = φ⁻⁴). Characteristic polynomial
//! (x−1)²(x²−7x+1) = x⁴−9x³+16x²−9x+1, with integer traces
//! tr(B^k) = 9, 49, 324, 2209 for k = 1..=4 — all pinned exactly in the
//! test battery below. The per-parent child-area sums are
//! (5,8), (3,5), (3,5), (2,3) = area(parent)·φ⁴ exact.
//!
//! The 3-shape view (merging LS/SL) is well-defined per step (both
//! rectangle orientations give the same merged child vector (2,3,1))
//! but NOT a closed count system (C·merge(v₁) = (33,42,13) ≠
//! merge(B·v₁) = (25,30,9)), so the module keeps 4 classes.
//!
//! **Exact arithmetic.** All geometry lives in F = Z[φ]: an element is
//! the integer pair (a,b) = a + b·φ with the reduction law φ² = φ + 1.
//! Every class shape, every rule anchor, every growth anchor and every
//! test assertion is an exact F equality; f64 is used only (i) for the
//! real embedding that *orders* F elements in the tests (safe — the gap
//! assert in `fcmp`) and (ii) at emission, after the exact patch is
//! computed. No decimal literal appears in the rule/growth path.
//!
//! **Refinement convention (inflation-growth, NOT in-place deflation).**
//! The children of a parent are SAME-NOMINAL-SIZE class tiles that tile
//! the φ²-INFLATED parent about its lower-left corner: a child's anchor
//! is φ²·(parent anchor) + (rule anchor). This is the exact dual of the
//! golden_bee convention (children strictly inside the parent): here
//! refinement GROWS the patch. From the LL seed, depth d is the square
//! [0, φ^(2d+1)]² (area φ^(4d+2), exactly F_{2d+2}² tiles) and the
//! depth-d tiling restricted to [0, φ^(2d−1)]² IS the depth-(d−1)
//! patch (1-D prefix property of the Fibonacci words, product form) —
//! so tile sets are stable under refinement and nested views on one
//! depth render bit-identical. The battery verifies no interior
//! overlap, no proper crossings and no T-junctions at every depth,
//! with per-parent exact dissections for BOTH R1 (figure, inflation φ)
//! and R2 (module, inflation φ²).
//!
//! **Finite-patch model and view-anchored depth.** The covered domain
//! is the depth-D_PRESENT patch, recentered on the origin by the fixed
//! float RECENTRE = −φ⁷/2 (1/2 ∉ Z[φ], so the centering is a fixed
//! float, not a lattice-exact element — the golden_bee precedent;
//! view-independent because constant). Depth is anchored at the patch:
//! d = the smallest depth ≥ D_PRESENT whose recentered patch covers the
//! view's positive extent, capped at MAX_DEPTH = 6
//! (F₁₄² = 142 129 ≤ MAX_TILES = 250 000 < F₁₆² = 974 169). The floor
//! exists because the UI preview window (probe → 9× mean tile side) is
//! anchor-self-canceling: it lands exactly on D_PRESENT and the whole
//! pane is tiled edge-to-edge (pinned in `default_view_fills_pane`).
//!
//! **Category — declared [`TilingCategory::Quasiperiodic`].** The exact
//! Z[φ] battery verifies the substitution mechanics (per-parent
//! dissections, area laws, count matrix, PF eigenvalue, growth
//! structure). The quasiperiodicity claim rests on the product
//! structure — the 2-D model set is the Cartesian product of two 1-D
//! Fibonacci model sets (Bielefeld entry: model set, square window,
//! own dual, Ammann bars) — cited, not re-proven here (golden-family
//! scope convention).
//!
//! **Pattern seed = orientation class** (0..3), so Pattern coloring
//! paints the four classes (LL, LS, SL, SS) in four hues.

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, ROTATION_ID, SCALE_ID};

// ---------------------------------------------------------------------------
// Exact arithmetic in F = Z[φ]
// ---------------------------------------------------------------------------

/// φ as f64 (real embedding at emission and in test ordering only).
const PHI_FLT: f64 = 1.618033988749895;
/// ln φ, for the depth anchor.
const LN_PHI: f64 = 0.48121182505960347;

/// An element of F = Z[φ]: (a, b) ⇔ a + b·φ, with φ² = φ + 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct F(i64, i64);

impl F {
    fn add(self, o: Self) -> Self {
        F(self.0 + o.0, self.1 + o.1)
    }
    /// Difference (test-only — growth uses only + and ×).
    #[cfg(test)]
    fn sub(self, o: Self) -> Self {
        F(self.0 - o.0, self.1 - o.1)
    }
    /// (a + bφ)(c + dφ) = (ac + bd) + (ad + bc + bd)·φ.
    fn mul(self, o: Self) -> Self {
        let (a, b) = (self.0, self.1);
        let (c, d) = (o.0, o.1);
        F(a * c + b * d, a * d + b * c + b * d)
    }
    /// Real embedding in the standard ordering F ⊂ ℝ.
    fn val(self) -> f64 {
        self.0 as f64 + self.1 as f64 * PHI_FLT
    }
}

const F0: F = F(0, 0);
const F1: F = F(1, 0);
const PHI: F = F(0, 1); // φ
const PHI2: F = F(1, 1); // φ² = φ + 1
/// φ⁴ = 2 + 3φ (test-only — the PF eigenvalue pin).
#[cfg(test)]
const PHI4: F = F(2, 3);

// ---------------------------------------------------------------------------
// The 4 orientation classes and the substitution rules
// ---------------------------------------------------------------------------

/// Class sizes (w, h) in F: 0 = LL (φ×φ), 1 = LS (φ×1), 2 = SL (1×φ),
/// 3 = SS (1×1) (test-only — emission reads the shapes from SHAPES).
#[cfg(test)]
const WH: [(F, F); 4] = [(PHI, PHI), (PHI, F1), (F1, PHI), (F1, F1)];

/// The four class shapes — CCW rectangles, bbox lower-left at the
/// origin, all vertices exact F.
const SHAPES: [[(F, F); 4]; 4] = [
    // LL: φ × φ square
    [(F0, F0), (PHI, F0), (PHI, PHI), (F0, PHI)],
    // LS: φ × 1 rectangle
    [(F0, F0), (PHI, F0), (PHI, F1), (F0, F1)],
    // SL: 1 × φ rectangle
    [(F0, F0), (F1, F0), (F1, PHI), (F0, PHI)],
    // SS: 1 × 1 square
    [(F0, F0), (F1, F0), (F1, F1), (F0, F1)],
];

/// Proto index per class — the 3 prototile SHAPES: 0 = φ×φ square (LL),
/// 1 = φ×1 rectangle (LS and SL), 2 = 1×1 square (SS).
const PROTO: [u8; 4] = [0, 1, 1, 2];

/// The single-step product rule R1 (the encyclopedia figure's rule),
/// linear inflation φ: (child class, anchor x, anchor y), exact F
/// (test-only: the battery pins it; the module rule is R2 below).
#[cfg(test)]
const RULES1: [&[(u8, F, F)]; 4] = [
    // LL -> LL@(0,0) LS@(0,φ) SL@(φ,0) SS@(φ,φ)
    &[(0, F0, F0), (1, F0, PHI), (2, PHI, F0), (3, PHI, PHI)],
    // LS -> LL@(0,0) SL@(φ,0)
    &[(0, F0, F0), (2, PHI, F0)],
    // SL -> LL@(0,0) LS@(0,φ)
    &[(0, F0, F0), (1, F0, PHI)],
    // SS -> LL@(0,0)
    &[(0, F0, F0)],
];

/// The module rule R2 = R1 ∘ R1, linear inflation φ² (the 1-D words are
/// L → LSL, S → LS). Children tile the φ²-inflated parent:
/// (child class, anchor x, anchor y), exact F, anchors the lower-left
/// corner of the child in the parent's unit frame.
const RULES: [&[(u8, F, F)]; 4] = [
    // LL -> 4 LL + 2 LS + 2 SL + 1 SS
    &[
        (0, F0, F0),
        (1, F0, PHI),
        (0, F0, PHI2),
        (2, PHI, F0),
        (3, PHI, PHI),
        (2, PHI, PHI2),
        (0, PHI2, F0),
        (1, PHI2, PHI),
        (0, PHI2, PHI2),
    ],
    // LS -> 2 LL + 2 LS + 1 SL + 1 SS
    &[
        (0, F0, F0),
        (1, F0, PHI),
        (2, PHI, F0),
        (3, PHI, PHI),
        (0, PHI2, F0),
        (1, PHI2, PHI),
    ],
    // SL -> 2 LL + 1 LS + 2 SL + 1 SS
    &[
        (0, F0, F0),
        (1, F0, PHI),
        (0, F0, PHI2),
        (2, PHI, F0),
        (3, PHI, PHI),
        (2, PHI, PHI2),
    ],
    // SS -> 1 of each
    &[(0, F0, F0), (1, F0, PHI), (2, PHI, F0), (3, PHI, PHI)],
];

/// One tile of the exact patch (frame B: the patch's lower-left corner
/// is the origin): orientation class `cls`, bbox lower-left `(ox, oy)`
/// in exact F. Every tile is at NOMINAL class size — refinement grows
/// the patch, not the tiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TileRec {
    cls: u8,
    ox: F,
    oy: F,
}

/// One substitution step (module rule, linear inflation φ²): each tile
/// is replaced by its rule children at anchors φ²·(parent anchor) +
/// (rule anchor) — the children tile the φ²-inflated parent about its
/// lower-left corner.
fn refine(tiles: &[TileRec]) -> Vec<TileRec> {
    let mut out = Vec::with_capacity(tiles.len() * 6);
    for t in tiles {
        for &(cls, ax, ay) in RULES[t.cls as usize] {
            out.push(TileRec {
                cls,
                ox: t.ox.mul(PHI2).add(ax),
                oy: t.oy.mul(PHI2).add(ay),
            });
        }
    }
    out
}

// ---------------------------------------------------------------------------
// View-anchored depth
// ---------------------------------------------------------------------------

/// Presentation depth floor: the depth of the covered patch.
const D_PRESENT: u32 = 3;
/// Max refinement depth: F₁₄² = 142 129 ≤ MAX_TILES = 250 000
/// < F₁₆² = 974 169.
const MAX_DEPTH: u32 = 6;

/// The depth-d patch side: the patch is the square [0, φ^(2d+1)]²,
/// so the side is φ·(φ²)^d = φ^(2d+1).
fn patch_side(d: u32) -> f64 {
    PHI_FLT * PHI_FLT.powi(2 * d as i32)
}

/// The fixed float recentering (frame B → presentation frame): the
/// depth-D_PRESENT patch [0, φ⁷]² is centered on the origin. 1/2 ∉
/// Z[φ], so this is a fixed float, not a lattice-exact element
/// (golden_bee precedent; constant, hence view-independent).
const RECENTRE: f64 = {
    // (const context: `while`, not `for`.)
    let mut v = 1.0f64;
    let mut n = 0;
    while n < 7 {
        v *= PHI_FLT;
        n += 1;
    }
    -0.5 * v // −φ⁷/2
};

/// Smallest depth ≥ D_PRESENT whose patch side reaches extent `u`,
/// capped at MAX_DEPTH.
fn depth_for_extent(u: f64) -> u32 {
    if u <= patch_side(D_PRESENT) {
        return D_PRESENT;
    }
    let d = (((u.ln() / LN_PHI - 1.0) / 2.0).ceil() as u32).max(D_PRESENT);
    d.min(MAX_DEPTH)
}

// ---------------------------------------------------------------------------
// The tiling
// ---------------------------------------------------------------------------

/// Fibonacci × Fibonacci — the Bielefeld product substitution, module
/// rule = two product steps (linear inflation τ² = φ², count matrix
/// B = A4², PF eigenvalue τ⁴ = 2 + 3φ). Finite covered patch: the
/// depth-3 corner patch, recentered on the origin; exact Z[φ].
const PARAMS: [ParamSpec; 2] = [
    ParamSpec::float(SCALE_ID, "scale", 0.1, 50.0, 0.05, 1.0),
    ParamSpec::angle(ROTATION_ID, "rotation (deg)", 0.0, 360.0, 5.0, 0.0),
];

pub struct FibonacciFibTiling;

impl Tiling for FibonacciFibTiling {
    fn id(&self) -> &'static str {
        "fibonacci-fibonacci"
    }

    fn name(&self) -> &'static str {
        "Fibonacci × Fibonacci (product substitution)"
    }

    fn description(&self) -> &'static str {
        "Bielefeld \"fibonacci-times-fibonacci\": Cartesian product of two \
         1-D Fibonacci substitutions (L → LS, S → L); 3 prototiles (φ×φ square, \
         φ×1 / 1×φ rectangle, 1×1 square) in 4 orientation classes; module rule \
         = two product steps, linear inflation τ² = φ², count matrix B = A4², \
         PF eigenvalue τ⁴ = 2 + 3φ; exact Z[φ], inflation-growing corner patch \
         [0, φ^(2d+1)]² (F_{2d+2}² tiles), depth 6 = 142129 tiles"
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Quasiperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &PARAMS
    }

    fn default_color_mode(&self) -> ColorMode {
        ColorMode::Pattern
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None).with_notes(
            "quasiperiodic (Bielefeld Tilings Encyclopedia: product of two \
             1-D Fibonacci substitutions, square model-set window); finite \
             covered patch — the depth-3 corner patch is the domain",
        );
        let map = similarity_map(params);
        let inv = map.inverse();

        // Query box in cell space: axis-aligned hull of the inverse
        // image of the four world corners.
        let mut rmin = Vec2::new(f64::MAX, f64::MAX);
        let mut rmax = Vec2::new(f64::MIN, f64::MIN);
        for &corner in query.region.corners().iter() {
            let q = inv.transform(corner);
            rmin.x = rmin.x.min(q.x);
            rmin.y = rmin.y.min(q.y);
            rmax.x = rmax.x.max(q.x);
            rmax.y = rmax.y.max(q.y);
        }
        let region = BBox::new(rmin, rmax);
        if region.area() <= 1e-12 {
            return TilingResult {
                tiles: Vec::new(),
                symmetry: meta,
                periodic: None,
                message: Some("empty view"),
            };
        }

        // The patch at depth d is [0, φ^(2d+1)]² in frame B, shown in
        // the recentered frame [RECENTRE, RECENTRE + φ^(2d+1)]². Choose
        // the smallest depth whose patch covers the view's positive
        // extent (floored at D_PRESENT — the UI preview window is
        // anchor-self-canceling, so an absolute rule would pin every
        // derived window at the floor either way).
        let u = rmax.x.max(rmax.y).max(0.0) - RECENTRE;
        let d = depth_for_extent(u);
        let side = patch_side(d);
        let patch = BBox::new(
            Vec2::new(RECENTRE, RECENTRE),
            Vec2::new(RECENTRE + side, RECENTRE + side),
        );
        if !patch.intersects(region) {
            // Finite patch: outside the covered domain is background.
            return TilingResult {
                tiles: Vec::new(),
                symmetry: meta,
                periodic: None,
                message: None,
            };
        }

        let mut recs: Vec<TileRec> = vec![TileRec {
            cls: 0,
            ox: F0,
            oy: F0,
        }];
        for _ in 0..d {
            recs = refine(&recs);
        }

        // Emission: exact F → f64 embedding + the fixed recentering,
        // culled to the query region.
        let mut tiles = Vec::new();
        for t in &recs {
            let ax = t.ox.val() + RECENTRE;
            let ay = t.oy.val() + RECENTRE;
            let pts: Vec<Vec2> = SHAPES[t.cls as usize]
                .iter()
                .map(|&(vx, vy)| Vec2::new(ax + vx.val(), ay + vy.val()))
                .collect();
            let poly = Polygon::new(pts).transform(&map);
            if poly.aabb().intersects(query.region) {
                tiles.push(Tile { family: tess_core::tiling::FamilyTag::None, reflected: false,
                    proto: PROTO[t.cls as usize],
                    seed: t.cls as u32, level: d as u8,
                    poly,
                });
            }
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
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{check_covers, check_det, check_rotated};

    // --- exact-F ordering helpers (real embedding, gap-asserted) ---------
    // F has no intrinsic order; distinct elements at the coefficient
    // sizes used here separate by > 1e-9 in ℝ, so the float decision is
    // safe once equality is handled exactly.

    fn fcmp(a: F, b: F) -> i32 {
        let d = a.val() - b.val();
        if d.abs() < 1e-9 {
            assert_eq!(a, b, "ambiguous F order: {a:?} vs {b:?}");
            return 0;
        }
        if d < 0.0 {
            -1
        } else {
            1
        }
    }
    fn flt(a: F, b: F) -> bool {
        fcmp(a, b) < 0
    }
    fn fle(a: F, b: F) -> bool {
        fcmp(a, b) <= 0
    }

    /// Exact-F rectangle (x0, x1, y0, y1), y up.
    fn rect(cls: u8, x0: F, y0: F) -> (F, F, F, F) {
        let (w, h) = WH[cls as usize];
        (x0, x0.add(w), y0, y0.add(h))
    }

    fn iv_open(a0: F, a1: F, b0: F, b1: F) -> bool {
        flt(b0, a1) && flt(a0, b1)
    }

    fn interior_overlap(r: &(F, F, F, F), s: &(F, F, F, F)) -> bool {
        iv_open(r.0, r.1, s.0, s.1) && iv_open(r.2, r.3, s.2, s.3)
    }

    /// Interior of a vertical edge of one == interior of a horizontal
    /// edge of the other (a proper "+" crossing).
    fn proper_crossing(r: &(F, F, F, F), s: &(F, F, F, F)) -> bool {
        for vv in [r, s] {
            for hh in [r, s] {
                if vv == hh {
                    continue;
                }
                for xa in [vv.0, vv.1] {
                    for yb in [hh.2, hh.3] {
                        if flt(vv.2, yb) && flt(yb, vv.3) && flt(hh.0, xa) && flt(xa, hh.1) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn corners(a: &(F, F, F, F)) -> [(F, F); 4] {
        [(a.0, a.2), (a.1, a.2), (a.0, a.3), (a.1, a.3)]
    }

    /// A corner of one in the RELATIVE INTERIOR of an edge of the other.
    fn t_junction(r: &(F, F, F, F), s: &(F, F, F, F)) -> bool {
        for (a, b) in [(r, s), (s, r)] {
            for (px, py) in corners(a) {
                if (px == b.0 || px == b.1) && flt(b.2, py) && flt(py, b.3) {
                    return true;
                }
                if (py == b.2 || py == b.3) && flt(b.0, px) && flt(px, b.1) {
                    return true;
                }
            }
        }
        false
    }

    /// Every positive-length shared boundary segment is the FULL side of
    /// both tiles (no partial-edge / T contacts).
    fn edge_to_edge(ch: &[(F, F, F, F)]) {
        for i in 0..ch.len() {
            for j in (i + 1)..ch.len() {
                let (r, s) = (&ch[i], &ch[j]);
                for x in [r.0, r.1, s.0, s.1] {
                    if (r.1 == x && s.0 == x) || (r.0 == x && s.1 == x) {
                        let lo = if fle(s.2, r.2) { r.2 } else { s.2 }; // max of lows
                        let hi = if fle(r.3, s.3) { r.3 } else { s.3 }; // min of highs
                        if flt(lo, hi) {
                            let h1 = r.3.sub(r.2);
                            let h2 = s.3.sub(s.2);
                            assert_eq!(
                                hi.sub(lo),
                                h1,
                                "partial vertical contact at x={x:?}: {i},{j}"
                            );
                            assert_eq!(
                                hi.sub(lo),
                                h2,
                                "partial vertical contact at x={x:?}: {i},{j}"
                            );
                        }
                    }
                }
                for y in [r.2, r.3, s.2, s.3] {
                    if (r.3 == y && s.2 == y) || (r.2 == y && s.3 == y) {
                        let lo = if fle(s.0, r.0) { r.0 } else { s.0 }; // max of lows
                        let hi = if fle(r.1, s.1) { r.1 } else { s.1 }; // min of highs
                        if flt(lo, hi) {
                            let w1 = r.1.sub(r.0);
                            let w2 = s.1.sub(s.0);
                            assert_eq!(
                                hi.sub(lo),
                                w1,
                                "partial horizontal contact at y={y:?}: {i},{j}"
                            );
                            assert_eq!(
                                hi.sub(lo),
                                w2,
                                "partial horizontal contact at y={y:?}: {i},{j}"
                            );
                        }
                    }
                }
            }
        }
    }

    /// Children of parent `p` under rule `rule` tile the `lfact`-inflated
    /// parent box exactly: containment, pairwise interior-disjoint, no
    /// proper crossings, edge-to-edge, and exact total area.
    fn check_parent(p: u8, rule: &[(u8, F, F)], lfact: F) {
        let (w, h) = WH[p as usize];
        let box_ = (F0, w.mul(lfact), F0, h.mul(lfact));
        let ch: Vec<(F, F, F, F)> = rule.iter().map(|&(c, ax, ay)| rect(c, ax, ay)).collect();
        for r in &ch {
            assert!(
                fle(box_.0, r.0) && fle(r.1, box_.1) && fle(box_.2, r.2) && fle(r.3, box_.3),
                "parent {p}: child {r:?} escapes the inflated box {box_:?}"
            );
        }
        for i in 0..ch.len() {
            for j in (i + 1)..ch.len() {
                assert!(!interior_overlap(&ch[i], &ch[j]), "parent {p}: interior overlap {i},{j}");
                assert!(!proper_crossing(&ch[i], &ch[j]), "parent {p}: crossing {i},{j}");
            }
        }
        edge_to_edge(&ch);
        let mut acc = F0;
        for r in &ch {
            acc = acc.add(r.1.sub(r.0).mul(r.3.sub(r.2)));
        }
        assert_eq!(
            acc,
            box_.1.sub(box_.0).mul(box_.3.sub(box_.2)),
            "parent {p}: child area sum != inflated parent area"
        );
    }

    /// B[i]·v = Σ_j B[i][j]·v[j], summed exactly in F.
    fn b_row_v(m: &[[i64; 4]; 4], i: usize, v: &[F; 4]) -> F {
        let mut acc = F0;
        for j in 0..4 {
            acc = acc.add(F(m[i][j], 0).mul(v[j]));
        }
        acc
    }

    fn counts_of(rule: &[&[(u8, F, F)]; 4]) -> [[i64; 4]; 4] {
        let mut c = [[0i64; 4]; 4];
        for p in 0..4 {
            for &(cls, _, _) in rule[p] {
                c[p][cls as usize] += 1;
            }
        }
        c
    }

    const A4_PINNED: [[i64; 4]; 4] = [
        [1, 1, 1, 1],
        [1, 0, 1, 0],
        [1, 1, 0, 0],
        [1, 0, 0, 0],
    ];
    const B_PINNED: [[i64; 4]; 4] = [
        [4, 2, 2, 1],
        [2, 2, 1, 1],
        [2, 1, 2, 1],
        [1, 1, 1, 1],
    ];

    #[test]
    fn field_identities() {
        assert_eq!(PHI.mul(PHI), PHI2, "φ² = φ + 1");
        assert_eq!(PHI2.mul(PHI), F(1, 2), "φ³");
        assert_eq!(PHI.mul(PHI).mul(PHI).mul(PHI), PHI4, "φ⁴ = 2 + 3φ");
        // conjugation (a, b) -> (a + b, -b); φ⁻⁴ = 5 − 3φ
        let conj = F(PHI4.0 + PHI4.1, -PHI4.1);
        assert_eq!(conj, F(5, -3), "conjugate of φ⁴");
        assert_eq!(PHI4.mul(conj), F1, "φ⁴ · conjugate = 1");
        // embedding cross-check
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        assert!((PHI_FLT - phi).abs() < 1e-15);
        assert!((PHI4.val() - phi.powi(4)).abs() < 1e-12);
        assert!((patch_side(3) - phi.powi(7)).abs() < 1e-12, "patch side d3 = φ⁷");
        assert!((patch_side(6) - phi.powi(13)).abs() < 1e-12, "patch side d6 = φ¹³");
        // The fixed recentering is exactly −φ⁷/2 (the pinned f64 value).
        assert!((RECENTRE + 14.517220926874318).abs() < 1e-12, "RECENTRE = −φ⁷/2");
    }

    #[test]
    fn rule_counts_matrix_squared() {
        // Figure rule counts == the pinned A4.
        assert_eq!(
            counts_of(&RULES1),
            A4_PINNED,
            "R1 counts must equal the figure-pinned A4"
        );
        // Module rule counts == A4² (matrix-squared invariant) == pinned B.
        let a = counts_of(&RULES1);
        let sq = [[
            a[0][0] * a[0][0] + a[0][1] * a[1][0] + a[0][2] * a[2][0] + a[0][3] * a[3][0],
            a[0][0] * a[0][1] + a[0][1] * a[1][1] + a[0][2] * a[2][1] + a[0][3] * a[3][1],
            a[0][0] * a[0][2] + a[0][1] * a[1][2] + a[0][2] * a[2][2] + a[0][3] * a[3][2],
            a[0][0] * a[0][3] + a[0][1] * a[1][3] + a[0][2] * a[2][3] + a[0][3] * a[3][3],
        ], [
            a[1][0] * a[0][0] + a[1][1] * a[1][0] + a[1][2] * a[2][0] + a[1][3] * a[3][0],
            a[1][0] * a[0][1] + a[1][1] * a[1][1] + a[1][2] * a[2][1] + a[1][3] * a[3][1],
            a[1][0] * a[0][2] + a[1][1] * a[1][2] + a[1][2] * a[2][2] + a[1][3] * a[3][2],
            a[1][0] * a[0][3] + a[1][1] * a[1][3] + a[1][2] * a[2][3] + a[1][3] * a[3][3],
        ], [
            a[2][0] * a[0][0] + a[2][1] * a[1][0] + a[2][2] * a[2][0] + a[2][3] * a[3][0],
            a[2][0] * a[0][1] + a[2][1] * a[1][1] + a[2][2] * a[2][1] + a[2][3] * a[3][1],
            a[2][0] * a[0][2] + a[2][1] * a[1][2] + a[2][2] * a[2][2] + a[2][3] * a[3][2],
            a[2][0] * a[0][3] + a[2][1] * a[1][3] + a[2][2] * a[2][3] + a[2][3] * a[3][3],
        ], [
            a[3][0] * a[0][0] + a[3][1] * a[1][0] + a[3][2] * a[2][0] + a[3][3] * a[3][0],
            a[3][0] * a[0][1] + a[3][1] * a[1][1] + a[3][2] * a[2][1] + a[3][3] * a[3][1],
            a[3][0] * a[0][2] + a[3][1] * a[1][2] + a[3][2] * a[2][2] + a[3][3] * a[3][2],
            a[3][0] * a[0][3] + a[3][1] * a[1][3] + a[3][2] * a[2][3] + a[3][3] * a[3][3],
        ]];
        let b = counts_of(&RULES);
        assert_eq!(b, sq, "module rule counts must equal A4²");
        assert_eq!(b, B_PINNED, "module rule counts must equal the pinned B");
    }

    #[test]
    fn pf_eigenvalue_and_charpoly() {
        let v = [PHI2, PHI, PHI, F1]; // areas of LL, LS, SL, SS
        // B·v = φ⁴·v exactly.
        for i in 0..4 {
            assert_eq!(
                b_row_v(&B_PINNED, i, &v),
                PHI4.mul(v[i]),
                "B·v = φ⁴·v failed at row {i}"
            );
        }
        // φ⁴ is the exact root of x² − 7x + 1: (2+3φ)² = 7(2+3φ) − 1.
        assert_eq!(PHI4.mul(PHI4), F(2, 3).mul(F(7, 0)).sub(F1));
        // Characteristic polynomial by Newton identities (exact in Z).
        // p walks B, B², B³, B⁴ (multiply by B each step — NOT square).
        let mut p = B_PINNED;
        let mut traces = [0i64; 4];
        for k in 0..4 {
            let mut s = 0i64;
            for i in 0..4 {
                s += p[i][i];
            }
            traces[k] = s;
            let mut np = [[0i64; 4]; 4];
            for i in 0..4 {
                for j in 0..4 {
                    for kk in 0..4 {
                        np[i][j] += p[i][kk] * B_PINNED[kk][j];
                    }
                }
            }
            p = np;
        }
        assert_eq!(traces, [9, 49, 324, 2209], "tr(B^k) for k = 1..=4");
        let e1 = traces[0];
        let e2 = (e1 * traces[0] - traces[1]) / 2;
        assert_eq!((e1 * traces[0] - traces[1]) % 2, 0);
        let e3 = (e2 * traces[0] - e1 * traces[1] + traces[2]) / 3;
        assert_eq!((e2 * traces[0] - e1 * traces[1] + traces[2]) % 3, 0);
        let e4 = (e3 * traces[0] - e2 * traces[1] + e1 * traces[2] - traces[3]) / 4;
        assert_eq!((e3 * traces[0] - e2 * traces[1] + e1 * traces[2] - traces[3]) % 4, 0);
        // x⁴ − e1 x³ + e2 x² − e3 x + e4
        assert_eq!((e1, e2, e3, e4), (9, 16, 9, 1), "char poly coefficients");
    }

    #[test]
    fn per_parent_area_law() {
        // Σ_j B[i][j]·area(j) = area(i)·φ⁴, exact in F.
        let area = [PHI2, PHI, PHI, F1];
        let want = [F(5, 8), F(3, 5), F(3, 5), F(2, 3)];
        for i in 0..4 {
            assert_eq!(
                b_row_v(&B_PINNED, i, &area),
                area[i].mul(PHI4),
                "per-parent area law failed at parent {i}"
            );
            assert_eq!(b_row_v(&B_PINNED, i, &area), want[i]);
        }
    }

    #[test]
    fn per_parent_exact_tiling_both_rules() {
        // Figure rule R1: children tile the φ-inflated parent.
        for p in 0..4u8 {
            check_parent(p, RULES1[p as usize], PHI);
        }
        // Module rule R2: children tile the φ²-inflated parent.
        for p in 0..4u8 {
            check_parent(p, RULES[p as usize], PHI2);
        }
    }

    #[test]
    fn growth_counts_area_geometry() {
        // Pinned from the battery spike (Python, exact Z[φ]): per-depth
        // class count vectors, total tile counts F_{2d+2}² and exact
        // total areas φ^{4d+2}.
        let v_pinned: [[i64; 4]; 5] = [
            [1, 0, 0, 0],
            [4, 2, 2, 1],
            [25, 15, 15, 9],
            [169, 104, 104, 64],
            [1156, 714, 714, 441],
        ];
        let area_pinned: [F; 5] = [F(1, 1), F(5, 8), F(34, 55), F(233, 377), F(1597, 2584)];
        let fib = |n: usize| {
            let (mut a, mut b) = (0i64, 1);
            for _ in 0..n {
                (a, b) = (b, a + b);
            }
            a
        };
        let mut recs: Vec<TileRec> = vec![TileRec {
            cls: 0,
            ox: F0,
            oy: F0,
        }];
        for d in 0..5u32 {
            let mut cnt = [0i64; 4];
            for t in &recs {
                cnt[t.cls as usize] += 1;
            }
            assert_eq!(cnt, v_pinned[d as usize], "depth {d}: class counts");
            assert_eq!(
                recs.len() as i64,
                fib(2 * d as usize + 2).pow(2),
                "depth {d}: tile count must be F_{}²",
                2 * d + 2
            );
            let mut area = F0;
            for t in &recs {
                let (w, h) = WH[t.cls as usize];
                area = area.add(w.mul(h));
            }
            assert_eq!(area, area_pinned[d as usize], "depth {d}: exact total area");
            // The whole patch is the square [0, φ^(2d+1)]²: exact bbox.
            let mut mxx = F0;
            let mut mxy = F0;
            for t in &recs {
                let (w, h) = WH[t.cls as usize];
                mxx = if fle(mxx, t.ox.add(w)) {
                    t.ox.add(w)
                } else {
                    mxx
                };
                mxy = if fle(mxy, t.oy.add(h)) {
                    t.oy.add(h)
                } else {
                    mxy
                };
            }
            assert_eq!(mxx, patch_side_exact(d), "depth {d}: patch side x");
            assert_eq!(mxy, patch_side_exact(d), "depth {d}: patch side y");
            // Global structure: no interior overlap, no proper
            // crossings, no T-junctions anywhere in the patch.
            let ch: Vec<(F, F, F, F)> =
                recs.iter().map(|t| rect(t.cls, t.ox, t.oy)).collect();
            for i in 0..ch.len() {
                for j in (i + 1)..ch.len() {
                    assert!(
                        !interior_overlap(&ch[i], &ch[j]),
                        "depth {d}: interior overlap {i},{j}"
                    );
                    assert!(!proper_crossing(&ch[i], &ch[j]), "depth {d}: crossing {i},{j}");
                    assert!(!t_junction(&ch[i], &ch[j]), "depth {d}: T-junction {i},{j}");
                }
            }
            recs = refine(&recs);
        }
        assert_eq!(recs.len(), 20736, "depth 5: F₁₂² tiles");
    }

    /// φ^(2d+1) exact in F.
    fn patch_side_exact(d: u32) -> F {
        let mut s = PHI;
        for _ in 0..d {
            s = s.mul(PHI2);
        }
        s
    }

    #[test]
    fn merged_view_well_defined_but_not_closed() {
        // MERGE: LL, LS, SL, SS -> LL, R, SS. Both R-orientation
        // parents give the SAME merged child vector — the 3-shape view
        // is well-defined per step — but the merged matrix is NOT a
        // closed count system.
        let merge = [0usize, 1, 1, 2];
        let mut c = [[0i64; 3]; 3];
        for i in 0..4 {
            for j in 0..4 {
                c[merge[i]][merge[j]] += B_PINNED[i][j];
            }
        }
        assert_eq!(
            c,
            [[4, 4, 1], [4, 6, 2], [1, 2, 1]],
            "merged 3-shape matrix"
        );
        let mut m1 = [0i64; 3];
        let mut m2 = [0i64; 3];
        for j in 0..4 {
            m1[merge[j]] += B_PINNED[1][j];
            m2[merge[j]] += B_PINNED[2][j];
        }
        assert_eq!(m1, [2, 3, 1], "LS merged child vector");
        assert_eq!(m2, [2, 3, 1], "SL merged child vector (same as LS)");
        assert_eq!(c[1], [m1[0] + m2[0], m1[1] + m2[1], m1[2] + m2[2]]);
        // Not closed: C·merge(v₁) != merge(B·v₁), v₁ = (4,2,2,1).
        let v1 = [4i64, 2, 2, 1];
        let mv1 = [4i64, 4, 1];
        let cv1 = [
            c[0][0] * mv1[0] + c[0][1] * mv1[1] + c[0][2] * mv1[2],
            c[1][0] * mv1[0] + c[1][1] * mv1[1] + c[1][2] * mv1[2],
            c[2][0] * mv1[0] + c[2][1] * mv1[1] + c[2][2] * mv1[2],
        ];
        let bv1 = [
            b_row_v_i(v1, 0),
            b_row_v_i(v1, 1),
            b_row_v_i(v1, 2),
            b_row_v_i(v1, 3),
        ];
        let mut mbv1 = [0i64; 3];
        for i in 0..4 {
            mbv1[merge[i]] += bv1[i];
        }
        assert_eq!(cv1, [33, 42, 13], "C·merge(v₁)");
        assert_eq!(mbv1, [25, 30, 9], "merge(B·v₁)");
        assert_ne!(cv1, mbv1, "merged view must NOT be closed");
    }

    fn b_row_v_i(v: [i64; 4], i: usize) -> i64 {
        let mut s = 0i64;
        for j in 0..4 {
            s += B_PINNED[i][j] * v[j];
        }
        s
    }

    // --- view / emission tests ------------------------------------------

    fn gen_keys(region: BBox) -> Vec<(u8, u32, Vec<u64>)> {
        let t = FibonacciFibTiling;
        // Mechanics pinned at scale 1.0 (the default — see
        // default_view_fills_pane); at s = 1.0 the windows below are in
        // cell units directly.
        let mut p = ParamSet::default_for(t.params());
        p.set(SCALE_ID, tess_core::params::ParamValue::Float(1.0));
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "unexpected message");
        assert!(!r.tiles.is_empty(), "empty view");
        r.tiles
            .iter()
            .map(|tile| {
                let mut bits = Vec::with_capacity(tile.poly.pts.len() * 2);
                for pt in &tile.poly.pts {
                    bits.push(pt.x.to_bits());
                    bits.push(pt.y.to_bits());
                }
                (tile.proto, tile.seed, bits)
            })
            .collect()
    }

    #[test]
    fn patch_nested_view_bit_identical() {
        // All three windows are cell-space squares of side 2, 2.8 and
        // 1.2 (cell == world at the scale-1.0 pin), so all pick the
        // D_PRESENT floor (u = 1.0/1.4/0.6 + φ⁷/2 < φ⁷) and refine the
        // same recentered patch: every tile of a smaller view must
        // reappear bit-identically in the larger one containing it.
        let base = gen_keys(BBox::new(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 1.0)));
        let grown = gen_keys(BBox::new(Vec2::new(-1.4, -1.4), Vec2::new(1.4, 1.4)));
        let zoomed = gen_keys(BBox::new(Vec2::new(-0.6, -0.6), Vec2::new(0.6, 0.6)));
        for k in base
            .iter()
            .chain(grown.iter())
            .chain(zoomed.iter())
        {
            assert!((0..3).contains(&k.0), "proto out of range: {k:?}");
        }
        for k in &base {
            assert!(grown.contains(k), "base tile not bit-identical in grown view");
        }
        for k in &zoomed {
            assert!(base.contains(k), "zoomed tile not bit-identical in base view");
        }
    }

    #[test]
    fn default_view_fills_pane() {
        // Pins the default presentation against the "single lonely tile"
        // failure mode of an inflating finite patch:
        //
        // (a) The UI/preview convention (TUI render pane, GUI canvas,
        //     card thumbnails): tess_core::app derives the window from
        //     the tiling itself — a bounded ±10 probe, then
        //     preview_window(9 × mean tile side) — an anchor-self-
        //     canceling loop that lands on the D_PRESENT floor. The
        //     derived window is a fully-tiled central CROP of the
        //     recentered depth-3 patch (window ⊆ patch: no background
        //     in the pane, all 4 orientation classes visible), pinned
        //     below with its exact tile census.
        // (b) The CLI convention: a ~20-unit-tall origin-centered 4:3
        //     pane — at the default scale it sits strictly inside the
        //     patch, so it is covered edge-to-edge (full area/census
        //     check).
        let t = FibonacciFibTiling;
        let p = ParamSet::default_for(t.params());
        assert!(
            (p.float(SCALE_ID, 1.0) - 1.0).abs() < 1e-12,
            "scale default must stay 1.0 (patch-fitted: the CLI pane \
             ±13.333×±10 sits inside the recentered depth-3 patch)"
        );
        let s = 1.0;
        let ps = patch_side(D_PRESENT) * s; // patch side, world units

        // --- (a) App/preview convention --------------------------------
        let probe = t.generate(&p, &Query {
            region: BBox::new(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0)),
        });
        assert!(probe.message.is_none());
        assert_eq!(probe.tiles.len(), 225, "probe tile census (depth-3 crop)");
        let mw = tess_core::app::mean_tile_side(&probe.tiles);
        assert!(
            (mw - 1.370820393249937).abs() < 1e-9,
            "probe mean tile side drift: {mw}"
        );
        let win = tess_core::app::preview_window(probe.periodic, mw, 4.0 / 3.0);
        assert!(
            (win.max.y - win.min.y - 12.337383539249434).abs() < 1e-9
                && (win.max.x - win.min.x - 16.449844718999244).abs() < 1e-9,
            "preview window drift: {win:?}"
        );
        let r = t.generate(&p, &Query { region: win });
        assert!(r.message.is_none());
        assert_eq!(r.tiles.len(), 143, "preview tile census (depth-3 crop)");
        // The floor renders a pane-sized crop of the recentered
        // depth-3 patch: the window must sit strictly inside the patch
        // (edge-to-edge tiling, no background in the pane).
        assert!(
            win.min.x >= -ps / 2.0 - 1e-9 && win.max.x <= ps / 2.0 + 1e-9
                && win.min.y >= -ps / 2.0 - 1e-9 && win.max.y <= ps / 2.0 + 1e-9,
            "preview window must fit the recentered depth-3 patch: {win:?} vs side {ps}"
        );
        // All 3 prototile shapes and all 4 orientation classes present.
        let mut protos = [false; 3];
        let mut classes = [false; 4];
        for tl in &r.tiles {
            protos[tl.proto as usize] = true;
            classes[tl.seed as usize] = true;
        }
        assert!(protos.iter().all(|x| *x), "preview must show all 3 shapes: {protos:?}");
        assert!(classes.iter().all(|x| *x), "preview must show all 4 classes: {classes:?}");
        // Self-consistency: regenerating over a window re-derived from
        // THIS result keeps the same tile census (no depth drift).
        let mw2 = tess_core::app::mean_tile_side(&r.tiles);
        let win2 = tess_core::app::preview_window(r.periodic, mw2, 4.0 / 3.0);
        let r2 = t.generate(&p, &Query { region: win2 });
        assert_eq!(
            r2.tiles.len(),
            r.tiles.len(),
            "second probe round must be a fixed point"
        );

        // --- (b) CLI convention ----------------------------------------
        let region = BBox::new(
            Vec2::new(-20.0 / s * 2.0 / 3.0, -10.0 / s),
            Vec2::new(20.0 / s * 2.0 / 3.0, 10.0 / s),
        );
        assert!(
            region.min.x >= -ps / 2.0 - 1e-9 && region.max.x <= ps / 2.0 + 1e-9
                && region.min.y >= -ps / 2.0 - 1e-9 && region.max.y <= ps / 2.0 + 1e-9,
            "CLI pane must sit inside the recentered depth-3 patch: {region:?}"
        );
        let rc = t.generate(&p, &Query { region });
        assert_eq!(rc.tiles.len(), 315, "CLI default pane census (depth-3 crop)");
        crate::tests::check_coverage(&t, "fibonacci-fibonacci(cli-pane)", &p, region);
    }

    // --- standard trio ----------------------------------------------------

    const WIN: [f64; 4] = [-1.0, -1.0, 1.0, 1.0];
    const WIN_ROT: [f64; 4] = [-0.6, -0.6, 0.6, 0.6];

    #[test]
    fn fibonacci_fib_covers_region() {
        check_covers(
            &FibonacciFibTiling,
            "fibonacci-fibonacci",
            BBox::new(Vec2::new(WIN[0], WIN[1]), Vec2::new(WIN[2], WIN[3])),
        );
    }

    #[test]
    fn fibonacci_fib_rotated_scaled() {
        // Inverse cell hull of the rotated 1.2×1.2 world square at
        // scale 0.6: hull half-extent 0.6·(cos25°+sin25°)/0.6 ≈ 1.3289
        // cell < φ⁷/2 = 14.517 (the depth-3 patch; margin ample).
        check_rotated(
            &FibonacciFibTiling,
            "fibonacci-fibonacci(rot25,scale0.6)",
            BBox::new(Vec2::new(WIN_ROT[0], WIN_ROT[1]), Vec2::new(WIN_ROT[2], WIN_ROT[3])),
            0.6,
            25.0,
        );
    }

    #[test]
    fn fibonacci_fib_deterministic() {
        check_det(
            &FibonacciFibTiling,
            "fibonacci-fibonacci",
            BBox::new(Vec2::new(WIN[0], WIN[1]), Vec2::new(WIN[2], WIN[3])),
        );
    }
}
